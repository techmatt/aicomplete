//! The adversary (014, 016, 017): its deck (the clock and the stager), its
//! ambassadors (build, spread, corrupt), nodes, adversary compute and the
//! project deck it fills, the ambassador supply and the difficulty levels.
//! Also the shared rules for placing and removing adversary ambassadors and
//! troops, which cards, threats and Alignment use, and the board conditions
//! for winning and losing.

use crate::content::{AdversaryCard, Op, ProjectCard, RoundFlag};
use crate::key::{Key, KeyBuf};
use crate::module::{Build, Cx, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{alignment, board, cards, effects, threats};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
    ParamSpec::list("stage_cards", 0, 20, "adversary cards dealt from stage I, II and III (before difficulty)"),
    ParamSpec::int("supply_per_country", 0, 40, "adversary ambassadors in the supply per country"),
    ParamSpec::int("base_sites_per_country", 0, 1, "1: arrivals and escalation reach only each country's first military base, its lowest-numbered (0: every base)"),
    ParamSpec::int("city_build", 0, 10, "build: ambassadors each corrupted city gains"),
    ParamSpec::int("factory_build", 0, 10, "build: ambassadors each factory next to a corrupted city gains"),
    ParamSpec::int("spread_range", 1, 6, "spread: how far (hexes) a spreading ambassador may travel"),
    ParamSpec::int("corrupt_at", 1, 20, "corrupt: a city that is not corrupted, holding this many adversary ambassadors, is corrupted"),
    ParamSpec::int("escalation_ambassadors", 0, 5, "stage-II escalation: ambassadors each base it reaches gains when the card builds"),
    ParamSpec::int("counter_bonus", 0, 10, "difficulty 1+: added to every threat's first counter requirement"),
    ParamSpec::int("counter_bonus_all", 0, 10, "difficulty 1+: added to every counter requirement of every threat"),
    ParamSpec::int("extra_threats", 0, 5, "difficulty 1+: extra threats dealt each round in all, one per country in turn"),
    ParamSpec::int("stage1_removed", 0, 3, "difficulty 2+: stage-I cards removed"),
    ParamSpec::int("stage3_added", 0, 3, "difficulty 2+: stage-III cards added (the cap holds, its end gets harder)"),
    ParamSpec::int("stage2_removed", 0, 3, "difficulty 3+: stage-II cards removed (a shorter round cap)"),
    ParamSpec::int("extra_corrupted", 0, 3, "difficulty 4: more cities per country start corrupted"),
    ParamSpec::int("build_bonus", 0, 5, "difficulty 3+: ambassadors each corrupted city gains on top of city_build"),
    ParamSpec::int("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
    ParamSpec::int("discord_removed", 0, 3, "difficulty 4+: Discord cards removed at setup (from tier 1)"),
    ParamSpec::int("arrivals", 0, 5, "build: ambassadors each site the adversary card names gains, whatever the board holds"),
    ParamSpec::int("wall_troops", 0, 5, "build: adversary troops landing in each country where a seat with Wall has presence (pressure for Wall to stop)"),
    ParamSpec::int("node_compute", 0, 5, "adversary compute each node makes a round (none with a player ambassador on it)"),
    ParamSpec::list("project_cards", 0, 20, "adversary projects dealt from stage 1 (cheap) and stage 2 (big), cheap on top"),
    ParamSpec::int("project_row", 1, 6, "adversary projects visible at once; a new row is dealt when all have resolved"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto("build", "the next card becomes this round's; corrupted cities and the factories next to them gain ambassadors; ongoing projects act; nodes make adversary compute onto the projects"),
    PhaseSpec::auto("spread", "every hex with several adversary ambassadors keeps one and sends the rest to pieces (a second on a piece makes a node)"),
    PhaseSpec::auto("corrupt", "cities that are not corrupted and hold enough adversary ambassadors are corrupted"),
    PhaseSpec::auto("reveal", "reveal the next adversary card face up, a round ahead"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "adversary",
    summary: "The adversary deck (the clock), ambassadors (build, spread, corrupt), nodes, the supply, difficulty",
    requires: &["board", "alignment"],
    variants: &[
        VariantDef {
            name: "militarist",
            summary: "The Militarist: ambassadors spread from corrupted cities; stage II escalates at the bases",
            params: PARAMS,
            phases: PHASES,
            rng: &["adversary"],
            uses_cards: false,
            build,
        },
    ],
};

pub const DECK: &str = "adversary.deck";
pub const DISCARD: &str = "adversary.discard";
/// This round's card (its twists apply).
pub const CURRENT: &str = "adversary.current";
/// Next round's card, face up a round ahead.
pub const NEXT: &str = "adversary.next";
/// Adversary ambassadors in the supply.
pub const SUPPLY: &str = "adversary.supply";
/// Stage of this round's card.
pub const STAGE: &str = "adversary.stage";
/// How many more deck cards beyond the next are shown (Recon Sweep).
pub const PEEK: &str = "adversary.peek";
/// Adversary compute made so far (all of it goes onto projects).
pub const COMPUTE: &str = "adversary.compute";
/// The project deck (face down), the visible row, ongoing projects in
/// play, and resolved or discarded ones.
pub const PROJECT_DECK: &str = "adversary.project_deck";
pub const PROJECTS: &str = "adversary.projects";
pub const PROJECTS_ONGOING: &str = "adversary.projects_ongoing";
pub const PROJECTS_DONE: &str = "adversary.projects_done";
/// The per-round counter of adversary operations (table time).
pub const OPS: &str = "adversary.ops";
/// The round the first project resolved (sim observation).
const FIRST_PROJECT: &str = "adversary.first_project";
/// The round of the first corruption after setup (sim observation).
const FIRST_CORRUPTION: &str = "adversary.first_corruption";

/// Set on a node's hex by a hack (021): it makes no compute in the next build.
pub fn cut_key(h: usize) -> String {
    format!("adversary.cut.{h}")
}

/// Whether the configured difficulty is at least `n` (setup rules).
pub fn level(w: &World, n: i64) -> bool {
    w.int("adversary.difficulty") >= n
}

/// Compute on visible project `id`.
pub fn project_key(id: &str) -> KeyBuf {
    KeyBuf::new("adversary.project.").text(id)
}

/// The visible projects, in row order.
pub fn projects(g: &GameState) -> Vec<Key> {
    g.pile(PROJECTS).to_vec()
}

/// Visible projects holding compute: what a drain can act on.
pub fn projects_with_compute(g: &GameState) -> Vec<Key> {
    let mut ids = projects(g);
    ids.retain(|p| g.var(&project_key(p)) > 0);
    ids
}

/// Seat `seat` picks a project holding compute, and `amount` drains from it.
pub fn choose_drain(g: &mut GameState, cx: &mut Cx, seat: usize, amount: i64) -> Res {
    let ids = projects_with_compute(g);
    let opts = ids.iter().map(|p| opt(*p)).collect();
    let prompt = format!("Seat {seat}: remove compute from which adversary project?");
    if let Some(i) = cx.choose(seat, &prompt, opts)? {
        drain_project(g, cx, &ids[i], amount);
    }
    Ok(())
}

fn project<'w>(w: &World<'w>, id: &str) -> &'w ProjectCard {
    w.content.project(id)
}

/// Compute that fills project `id`: its printed cost per player.
pub fn project_cost(w: &World, g: &GameState, id: &str) -> i64 {
    project(w, id).cost * g.seats.len().max(1) as i64
}

/// The ambassador supply at the start, for the map's countries.
pub fn start_supply(w: &World) -> i64 {
    (w.int("adversary.supply_per_country") * w.map.countries.len() as i64).max(0)
}

/// Which tier of threats is dealt now: tier 2 once this round's card is
/// stage II or later (at difficulty 4, `tier2_early` cards sooner).
pub fn threat_tier(w: &World, g: &GameState) -> u8 {
    if g.var(STAGE) >= 2 {
        return 2;
    }
    if level(w, 4) {
        let early = w.int("adversary.tier2_early") as usize;
        let soon = g
            .pile(NEXT)
            .iter()
            .chain(g.pile(DECK))
            .take(early)
            .any(|id| card(w, id).stage >= 2);
        if soon {
            return 2;
        }
    }
    1
}

pub(super) fn card<'w>(w: &World<'w>, id: &str) -> &'w AdversaryCard {
    w.content.adversary_card(id)
}

fn card_text(c: &AdversaryCard) -> String {
    format!(
        "stage {}: {}",
        c.stage,
        if c.text.is_empty() {
            "no twist"
        } else {
            &c.text
        }
    )
}

/// This round's twists. Before this round's build step (a Discord tip in
/// the Sway phase) this round's card is still the face-up one: read it (022:
/// that read last round's card, and none in round 1).
pub fn twists<'w>(w: &World<'w>, g: &GameState) -> Vec<&'w Op> {
    let before_build = g.phase_index("adversary.build").is_some_and(|i| g.phase < i);
    g.pile(if before_build { NEXT } else { CURRENT })
        .iter()
        .flat_map(|id| card(w, id).twists.iter())
        .collect()
}

fn twist_sum(w: &World, g: &GameState, f: impl Fn(&Op) -> Option<i64>) -> i64 {
    twists(w, g).into_iter().filter_map(f).sum()
}

/// Whether this round's card makes it a war round.
pub fn war_twist(w: &World, g: &GameState) -> bool {
    twists(w, g).iter().any(|t| matches!(t, Op::War))
}

#[derive(Debug)]
struct Adversary {
    /// What this difficulty deals from stage I, II and III.
    dealt: [i64; 3],
    project_cards: Vec<i64>,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let stage_cards = b.list("stage_cards")?;
    if stage_cards.len() != 3 {
        return Err("stage_cards: give 3 counts (stage I, II, III)".into());
    }
    // 044: what this difficulty deals, for setup and the check (which used to
    // add stage3_added at every level and take off no removal).
    let difficulty = b.int("difficulty")?;
    let dealt = [
        (stage_cards[0] - if difficulty >= 2 { b.int("stage1_removed")? } else { 0 }).max(0),
        (stage_cards[1] - if difficulty >= 3 { b.int("stage2_removed")? } else { 0 }).max(0),
        stage_cards[2] + if difficulty >= 2 { b.int("stage3_added")? } else { 0 },
    ];
    // An empty deck loses before round 1's first decision (020: it was accepted).
    if dealt.iter().sum::<i64>() < 1 {
        return Err("stage_cards: deal at least one card (the deck is the game's clock)".into());
    }
    for (s, &n) in dealt.iter().enumerate() {
        let have = b
            .world
            .content
            .adversary
            .iter()
            .filter(|c| c.stage as usize == s + 1)
            .count() as i64;
        if n > have {
            return Err(format!(
                "stage_cards: stage {} has only {have} cards",
                s + 1
            ));
        }
    }
    let project_cards = b.list("project_cards")?;
    if project_cards.len() != 2 {
        return Err("project_cards: give 2 counts (stage 1, stage 2)".into());
    }
    for (s, &n) in project_cards.iter().enumerate() {
        let have = b
            .world
            .content
            .projects
            .iter()
            .filter(|c| c.stage as usize == s + 1)
            .count() as i64;
        if n > have {
            return Err(format!(
                "project_cards: stage {} has only {have} projects",
                s + 1
            ));
        }
    }
    Ok(Box::new(Adversary { dealt, project_cards }))
}

impl Module for Adversary {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        cx.set_key(g, SUPPLY, start_supply(&w));
        // Each country starts with a node on its (corrupted) research station.
        for c in 0..w.map.countries.len() {
            for &n in &w.content.map.countries[c].stations {
                cx.add_tokens(g, w.map.numbered(c, n), NODE, 1);
            }
        }
        let mut deck = Vec::new();
        for stage in 1..=3u8 {
            let mut ids: Vec<Key> = w
                .content
                .adversary
                .iter()
                .filter(|c| c.stage == stage)
                .map(|c| Key::new(&c.id))
                .collect();
            shuffle(&mut ids, cx.rng(g, "adversary"));
            deck.extend(ids.into_iter().take(self.dealt[stage as usize - 1] as usize));
        }
        cx.set_pile(g, DECK, deck);
        reveal(g, cx);
        // The project deck: each stage shuffled on its own, cheap on top.
        let mut deck = Vec::new();
        for stage in 1..=2u8 {
            let mut ids: Vec<Key> = w
                .content
                .projects
                .iter()
                .filter(|c| c.stage == stage)
                .map(|c| Key::new(&c.id))
                .collect();
            shuffle(&mut ids, cx.rng(g, "adversary"));
            let n = self.project_cards[stage as usize - 1];
            deck.extend(ids.into_iter().take(n as usize));
        }
        cx.set_pile(g, PROJECT_DECK, deck);
        deal_projects(g, cx);
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match phase {
            0 => {
                // The face-up card becomes this round's.
                for id in g.pile(CURRENT).to_vec() {
                    cx.move_card(g, &id, CURRENT, DISCARD, true)
                        .expect("current card");
                }
                let Some(id) = g.pile(NEXT).first().cloned() else {
                    cx.end(g, false, "deck_empty", "the adversary deck ran out".into());
                    return Ok(());
                };
                cx.move_card(g, &id, NEXT, CURRENT, true)
                    .expect("next card");
                let c = card(&w, &id);
                cx.set_key(g, STAGE, c.stage as i64);
                cx.note(None, || {
                    format!("Adversary card: {} ({})", c.name, card_text(c))
                });
                for t in &c.twists {
                    match t {
                        // These resolve as they do anywhere (034, 044).
                        Op::Sway { .. } | Op::EachDestroyPresence { .. } | Op::PullUnity { .. } => {
                            effects::run(g, cx, std::slice::from_ref(t), &mut effects::Ctx::team().named("twist"))?
                        }
                        // Read where they act: the build, spread, corrupt and arrivals
                        // steps, and the war step.
                        Op::CityBuild { .. }
                        | Op::CleanFactories { .. }
                        | Op::StationBuild { .. }
                        | Op::SpreadReach { .. }
                        | Op::CorruptSooner { .. }
                        | Op::Arrivals { .. }
                        | Op::War => {}
                        // The load check lets no other op in: one added to `Slot::Twist`
                        // needs a reader (044: `_ => {}` swallowed the Unity pull).
                        other => unreachable!("twist with no reader: {other:?}"),
                    }
                }
                build_all(g, cx, None);
                wall_pressure(g, cx)?;
                if c.escalation {
                    for (h, n) in super::plan::escalation_hexes(&w, g) {
                        place(g, cx, h, n);
                    }
                }
                // Ongoing projects act, in the order they came into play.
                for id in g.pile(PROJECTS_ONGOING).to_vec() {
                    effects::run(g, cx, &project(&w, &id).effects, &mut effects::Ctx::team().named("project"))?;
                    if g.is_over() {
                        return Ok(());
                    }
                }
                // 068: a cordon stopped this build's arrivals, and is done.
                for c in 0..w.map.countries.len() {
                    cx.set_key(g, &board::flag(RoundFlag::Cordon, c), 0);
                }
                // Nodes make compute, except under a player ambassador.
                let per = w.int("adversary.node_compute");
                let (mut made, mut blocked) = (0, 0);
                for h in 0..g.hexes.len() {
                    let n = g.tokens(h, NODE) * per;
                    if n == 0 {
                        continue;
                    }
                    let cut = cut_key(h);
                    if g.var(&cut) > 0 {
                        // A hack cut it off (021): no compute this build.
                        cx.set_key(g, &cut, 0);
                        blocked += n;
                    } else if player_ambassador(g, h).is_some() {
                        blocked += n;
                    } else {
                        made += n;
                    }
                }
                if blocked > 0 {
                    cx.tally(g, "adversary.compute_blocked", blocked);
                    cx.per_round(g, "adversary.compute_blocked", blocked);
                }
                cx.tally(g, "adversary.compute_nodes", made);
                gain_compute(g, cx, made)?;
            }
            1 => spread(g, cx, None)?,
            2 => {
                let at = (w.int("adversary.corrupt_at")
                    - twist_sum(&w, g, |t| match t {
                        Op::CorruptSooner { amount } => Some(*amount),
                        _ => None,
                    }))
                .max(1);
                let cities: Vec<usize> = (0..g.hexes.len())
                    .filter(|&h| {
                        // 034: a corruption marker counts toward corrupting.
                        is_city(&w, h)
                            && !corrupted(g, h)
                            && g.tokens(h, ADV_AMBASSADOR) + g.tokens(h, MARKER) >= at
                    })
                    .collect();
                for h in cities {
                    corrupt_city(g, cx, h);
                }
                // Nodes at their peak, per board (country).
                for c in 0..w.map.countries.len() {
                    let n = node_count(g, Some(&w.map.countries[c].hexes));
                    cx.peak(g, &format!("adversary.nodes_peak.c{c}"), n);
                }
                let on_board: i64 = (0..g.hexes.len())
                    .map(|h| g.tokens(h, ADV_AMBASSADOR))
                    .sum();
                cx.tally(
                    g,
                    &format!("adversary.ambassadors_r{:02}", g.round),
                    on_board,
                );
            }
            _ => reveal(g, cx),
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let show = |id: &str, status: &str| {
            let c = card(w, id);
            CardView {
                id: id.to_string(),
                name: c.name.clone(),
                text: card_text(c),
                status: status.to_string(),
                cost: None,
            }
        };
        let deck = g.pile(DECK);
        let peek = g.var(PEEK).max(0) as usize;
        out.pile(
            "deck",
            "Adversary deck (the clock)",
            None,
            deck.len(),
            deck.iter().take(peek).map(|id| show(id, "next")).collect(),
        );
        let next: Vec<CardView> = g
            .pile(NEXT)
            .iter()
            .map(|id| show(id, "face up: next round"))
            .collect();
        out.pile("next", "Next round's card", None, next.len(), next);
        let cur: Vec<CardView> = g
            .pile(CURRENT)
            .iter()
            .map(|id| show(id, "this round"))
            .collect();
        out.pile("current", "This round's card", None, cur.len(), cur);
        let disc = g.pile(DISCARD);
        out.pile(
            "discard",
            "Adversary discard",
            None,
            disc.len(),
            disc.iter().map(|id| show(id, "done")).collect(),
        );
        out.global(
            "supply",
            "adversary ambassador supply",
            g.var(SUPPLY),
            Some(start_supply(w)),
            "adversary ambassadors left: a cap, a placement it can't make doesn't happen",
        );
        out.global(
            "stage",
            "adversary stage",
            g.var(STAGE),
            Some(3),
            "stage of this round's adversary card",
        );
        out.global(
            "compute",
            "adv. compute made",
            g.var(COMPUTE),
            None,
            &format!(
                "adversary compute made so far: each node makes {} a round (none under a player ambassador, or in the build after a hack cuts it); it fills the cheapest visible project",
                w.int("adversary.node_compute")
            ),
        );
        out.global(
            "difficulty",
            "difficulty",
            w.int("adversary.difficulty"),
            Some(4),
            "difficulty level",
        );
        // A visible project's status is its fill, "n/cost compute".
        let pshow = |id: &str, status: &str| {
            let p = project(w, id);
            let kind = if p.stage == 1 { "cheap" } else { "big" };
            CardView {
                id: id.to_string(),
                name: p.name.clone(),
                text: format!("{kind}: {}", p.text),
                status: status.to_string(),
                cost: Some(project_cost(w, g, id)),
            }
        };
        let fill = |id: &str| {
            format!("{}/{} compute", g.var(&project_key(id)), project_cost(w, g, id))
        };
        let row = g.pile(PROJECTS);
        out.pile(
            "projects",
            "Adversary projects (compute fills the cheapest)",
            None,
            row.len(),
            row.iter().map(|id| pshow(id, &fill(id))).collect(),
        );
        let on = g.pile(PROJECTS_ONGOING);
        out.pile(
            "projects_ongoing",
            "Ongoing adversary projects",
            None,
            on.len(),
            on.iter().map(|id| pshow(id, "ongoing")).collect(),
        );
        let done = g.pile(PROJECTS_DONE);
        out.pile(
            "projects_done",
            // A discarded project (Zero-Day Wipe) goes here too (020: labeled "resolved").
            "Adversary projects resolved or discarded",
            None,
            done.len(),
            done.iter().map(|id| pshow(id, "done")).collect(),
        );
        out.pile(
            "project_deck",
            "Adversary project deck",
            None,
            g.pile(PROJECT_DECK).len(),
            Vec::new(),
        );
        out.global(
            "deck",
            "adversary cards left",
            deck.len() as i64 + g.pile(NEXT).len() as i64,
            None,
            "rounds left on the clock after this one; players lose when the next round has no card",
        );
        out.bookkeeping(&[
            PEEK,
            FIRST_CORRUPTION,
            FIRST_PROJECT,
            "adversary.project.",
            "adversary.cut.",
        ]);
    }
}

/// Shows the next `n` deck cards beyond the face-up one (Recon Sweep); a
/// longer peek already running is kept.
pub fn peek(g: &mut GameState, cx: &mut Cx, n: i64) {
    let cur = g.var(PEEK);
    cx.set_key(g, PEEK, cur.max(n));
}

/// Deals a new row of projects from the top of the project deck.
fn deal_projects(g: &mut GameState, cx: &mut Cx) {
    let n = cx.world.int("adversary.project_row") as usize;
    let ids: Vec<Key> = g.pile(PROJECT_DECK).iter().take(n).cloned().collect();
    for id in ids {
        cx.move_card(g, &id, PROJECT_DECK, PROJECTS, false)
            .expect("top of the project deck");
    }
}

/// The adversary gains `n` compute: each point goes onto the cheapest
/// visible project (ties: the first in the row); a project that fills
/// resolves at once. With no project left, compute is lost.
pub fn gain_compute(g: &mut GameState, cx: &mut Cx, n: i64) -> Res {
    if n <= 0 {
        return Ok(());
    }
    let w = cx.world;
    cx.add_key(g, COMPUTE, n);
    cx.tally(g, "adversary.compute_made", n);
    cx.per_round(g, "adversary.compute", n);
    for _ in 0..n {
        let Some(id) = g
            .pile(PROJECTS)
            .iter()
            .min_by_key(|id| project_cost(&w, g, id))
            .cloned()
        else {
            cx.tally(g, "adversary.compute_lost", 1);
            continue;
        };
        cx.add_key(g, &project_key(&id), 1);
        cx.per_round(g, OPS, 1);
        if g.var(&project_key(&id)) >= project_cost(&w, g, &id) {
            resolve_project(g, cx, &id)?;
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// The adversary gains `n` compute on every visible project (074). A project
/// that fills resolves at once, and one dealt by that gains nothing this time.
pub fn compute_every(g: &mut GameState, cx: &mut Cx, n: i64) -> Res {
    let w = cx.world;
    for id in projects(g) {
        cx.add_key(g, COMPUTE, n);
        cx.tally(g, "adversary.compute_made", n);
        cx.tally(g, "adversary.compute_corp", n);
        cx.tally(g, "adversary.compute_every", n);
        cx.per_round(g, "adversary.compute", n);
        cx.per_round(g, OPS, n);
        cx.add_key(g, &project_key(&id), n);
        if g.var(&project_key(&id)) >= project_cost(&w, g, &id) {
            resolve_project(g, cx, &id)?;
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// A full project resolves: once (to the resolved pile), or as an ongoing
/// project from next round's build on. An empty row deals a new one.
fn resolve_project(g: &mut GameState, cx: &mut Cx, id: &str) -> Res {
    let w = cx.world;
    let p = project(&w, id);
    cx.set_key(g, &project_key(id), 0);
    let to = if p.ongoing {
        PROJECTS_ONGOING
    } else {
        PROJECTS_DONE
    };
    cx.move_card(g, id, PROJECTS, to, false)
        .expect("visible project");
    cx.tally(g, "adversary.projects_resolved", 1);
    cx.tally(
        g,
        if p.stage == 1 {
            "adversary.projects_resolved.cheap"
        } else {
            "adversary.projects_resolved.big"
        },
        1,
    );
    cx.tally(g, &format!("adversary.project.{id}"), 1);
    if g.var(FIRST_PROJECT) == 0 {
        cx.set_key(g, FIRST_PROJECT, g.round as i64);
        cx.tally(g, "adversary.first_project_round", g.round as i64);
    }
    cx.note(None, || {
        format!("Adversary project {} resolves: {}", p.name, p.text)
    });
    if g.pile(PROJECTS).is_empty() {
        deal_projects(g, cx);
    }
    if !p.ongoing {
        effects::run(g, cx, &p.effects, &mut effects::Ctx::team().named("project"))?;
    }
    Ok(())
}

/// Removes up to `n` compute from visible project `id` (Security cards).
pub fn drain_project(g: &mut GameState, cx: &mut Cx, id: &str, n: i64) {
    let k = n.min(g.var(&project_key(id))).max(0);
    if k > 0 {
        cx.add_key(g, &project_key(id), -k);
        cx.tally(g, "adversary.compute_drained", k);
    }
}

/// Discards visible project `id` with its compute; an empty row deals a
/// new one.
pub fn discard_project(g: &mut GameState, cx: &mut Cx, id: &str) {
    let w = cx.world;
    cx.set_key(g, &project_key(id), 0);
    cx.move_card(g, id, PROJECTS, PROJECTS_DONE, false)
        .expect("visible project");
    cx.tally(g, "adversary.projects_discarded", 1);
    cx.note(None, || {
        format!("Adversary project {} is discarded", project(&w, id).name)
    });
    if g.pile(PROJECTS).is_empty() {
        deal_projects(g, cx);
    }
}

/// Reveals the top deck card face up as next round's card (nothing if the deck is empty).
fn reveal(g: &mut GameState, cx: &mut Cx) {
    if let Some(id) = g.pile(DECK).first().cloned() {
        cx.move_card(g, &id, DECK, NEXT, true)
            .expect("top of the deck");
        let peek = g.var(PEEK);
        if peek > 0 {
            cx.set_key(g, PEEK, peek - 1);
        }
    }
}

/// The build step, in country `scope` or everywhere: each corrupted city
/// gains `city_build` (+ difficulty 3, + twists), each factory next to a
/// corrupted city `factory_build`; twists add clean-city factories and
/// corrupted stations.
pub fn build_all(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) {
    let w = cx.world;
    // One source for the rule: the plan is the build.
    let on = twists(&w, g);
    cx.tally(g, "adversary.arrivals", super::plan::arrival_count(&w, g, &on, scope));
    for (h, n) in super::plan::build_plan(&w, g, &on, scope) {
        place(g, cx, h, n);
    }
}

/// Troop pressure where the Protector stands (080, candidate): in the round's
/// build, `wall_troops` adversary troops land in each country where a seat with
/// Wall has presence, as any landing does (on a factory, else a city, never on
/// a walled hex), so Wall has strikes to stop.
fn wall_pressure(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let n = w.int("adversary.wall_troops");
    if n == 0 {
        return Ok(());
    }
    let walls: Vec<usize> = (0..g.seats.len()).filter(|&s| board::has_wall(&w, s)).collect();
    for c in 0..w.map.countries.len() {
        if !walls.iter().any(|&s| has_presence_in(&w, g, s, c)) {
            continue;
        }
        let hexes = w.map.countries[c].hexes.clone();
        land_troops(g, cx, &hexes, n, "Troops land where the Protector stands: on which hex?", "adversary.wall_troops", None)?;
    }
    Ok(())
}

/// Places `n` adversary ambassadors from the supply on hex `h`. The supply
/// is a cap (027): a placement it can't make doesn't happen.
pub fn place(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) {
    for k in 0..n {
        if g.var(SUPPLY) <= 0 {
            cx.tally(g, "adversary.supply_short", n - k);
            return;
        }
        cx.add_key(g, SUPPLY, -1);
        cx.per_round(g, OPS, 1);
        arrive(g, cx, h, false);
    }
}

/// One adversary ambassador arrives on hex `h` (from the supply or by
/// spreading): it cancels a player ambassador there (both are removed, no
/// Unity); spreading onto a piece that already holds one (not a city that
/// isn't corrupted, clean or aligned, which collects them, and not a node)
/// makes a node there, and both ambassadors return to the supply.
fn arrive(g: &mut GameState, cx: &mut Cx, h: usize, spreading: bool) {
    let w = cx.world;
    if let Some(s) = player_ambassador(g, h) {
        cx.add_tokens(g, h, ambassador(s), -1);
        cx.add_key(g, SUPPLY, 1);
        cx.tally(g, "adversary.canceled", 1);
        return;
    }
    if spreading
        && g.tokens(h, ADV_AMBASSADOR) > 0
        && !collecting_city(&w, g, h)
        && g.tokens(h, NODE) == 0
    {
        remove(g, cx, h, 1);
        cx.add_key(g, SUPPLY, 1);
        cx.add_tokens(g, h, NODE, 1);
        cx.tally(g, "adversary.nodes_made", 1);
        cx.peak(g, "adversary.nodes_peak", node_count(g, None));
        cx.note(Some(h), || format!("A node forms at {}", hex_name(&w, h)));
        return;
    }
    cx.add_tokens(g, h, ADV_AMBASSADOR, 1);
}

/// Removes the node on hex `h` (a card's effect; no standard action does, 019).
pub fn remove_node(g: &mut GameState, cx: &mut Cx, h: usize) {
    if g.tokens(h, NODE) > 0 {
        cx.add_tokens(g, h, NODE, -1);
        // A node formed here later is a new one: the hack cut this one (028).
        cx.set_key(g, &cut_key(h), 0);
        cx.tally(g, "adversary.nodes_removed", 1);
        alignment::for_node_removed(g, cx);
    }
}

/// Nodes on `hexes`, or on the whole board for `None`.
pub fn node_count(g: &GameState, hexes: Option<&[usize]>) -> i64 {
    match hexes {
        Some(hexes) => hexes.iter().map(|&h| g.tokens(h, NODE)).sum(),
        None => (0..g.hexes.len()).map(|h| g.tokens(h, NODE)).sum(),
    }
}

/// Removes up to `n` adversary ambassadors from hex `h` to the supply.
pub fn remove(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> i64 {
    let k = n.min(g.tokens(h, ADV_AMBASSADOR)).max(0);
    if k > 0 {
        cx.add_tokens(g, h, ADV_AMBASSADOR, -k);
        cx.add_key(g, SUPPLY, k);
    }
    k
}

/// Removes every adversary ambassador in country `c` to the supply (one
/// tally for them all); returns how many.
pub fn remove_all_in_country(g: &mut GameState, cx: &mut Cx, c: usize) -> i64 {
    let w = cx.world;
    let mut removed = 0;
    for &h in &w.map.countries[c].hexes {
        let n = g.tokens(h, ADV_AMBASSADOR);
        removed += remove(g, cx, h, n);
    }
    cx.tally(g, "adversary.ambassadors_removed", removed);
    removed
}

/// An adversary troop lands on hex `h` (the caller counts it).
pub fn place_troop(g: &mut GameState, cx: &mut Cx, h: usize) {
    cx.add_tokens(g, h, TROOP, 1);
}

/// Any player cube on hex `h` comes off and the adversary's goes on.
fn take_for_adversary(g: &mut GameState, cx: &mut Cx, h: usize) {
    if let Some(s) = cube_owner(g, h) {
        cx.add_tokens(g, h, cube(s), -1);
    }
    cx.add_tokens(g, h, ADV_CUBE, 1);
}

/// Every corruption marker on hex `h` comes off: corruption replaces them.
pub fn clear_markers_on(g: &mut GameState, cx: &mut Cx, h: usize) {
    let marked = g.tokens(h, MARKER);
    cx.add_tokens(g, h, MARKER, -marked);
}

/// City `h` is corrupted: the adversary's cube goes on (a player cube
/// comes off), its adversary ambassadors return to the supply, and any
/// presence on it is destroyed (027).
pub fn corrupt_city(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    take_for_adversary(g, cx, h);
    let n = g.tokens(h, ADV_AMBASSADOR);
    remove(g, cx, h, n);
    // 024: player ambassadors waiting here are lost, or they would free the
    // city in the same step (and pay Unity for an adversary attack).
    for s in 0..g.seats.len() {
        let k = ambassador(s);
        let mine = g.tokens(h, k);
        cx.add_tokens(g, h, k, -mine);
    }
    // The city's corruption replaces any marker on it.
    clear_markers_on(g, cx, h);
    // 034: by `corruption_reach`, presence around the city goes too.
    for x in within(&w, h, w.int("board.corruption_reach")) {
        if let Some(s) = presence_owner(g, x) {
            board::lose_presence(g, cx, x, s, "corruption");
        }
    }
    cx.tally(g, "adversary.corruptions", 1);
    if g.round > 0 && g.var(FIRST_CORRUPTION) == 0 {
        cx.set_key(g, FIRST_CORRUPTION, g.round as i64);
        cx.tally(g, "adversary.first_corruption_round", g.round as i64);
    }
    cx.note(Some(h), || format!("{} is corrupted", hex_name(&w, h)));
}

/// `count` adversary troops, each onto a factory among `hexes` (players
/// choose), or with no factory onto a city. With `destroy`, each destroys
/// the presence on the hex it lands on (027), named for its tally.
pub fn land_troops(
    g: &mut GameState,
    cx: &mut Cx,
    hexes: &[usize],
    count: i64,
    prompt: &str,
    tally: &str,
    destroy: Option<&'static str>,
) -> Res {
    let w = cx.world;
    for _ in 0..count {
        // 068: a landing enters its hex, so a Wall stops it too.
        let open = |h: &usize| !board::walled(&w, g, *h);
        let mut spots: Vec<usize> = hexes
            .iter()
            .copied()
            .filter(open)
            .filter(|&h| g.tokens(h, FACTORY) > 0)
            .collect();
        if spots.is_empty() {
            spots = hexes.iter().copied().filter(open).filter(|&h| is_city(&w, h)).collect();
        }
        let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
        let Some(i) = cx.choose(0, prompt, opts)? else {
            break;
        };
        place_troop(g, cx, spots[i]);
        cx.per_round(g, OPS, 1);
        cx.tally(g, tally, 1);
        if let Some(source) = destroy {
            for s in 0..g.seats.len() {
                if g.tokens(spots[i], presence(s)) > 0 {
                    board::destroy_piece(g, cx, spots[i], presence(s), board::Cause::Effect(source));
                }
            }
        }
    }
    Ok(())
}

/// Research station `h`, not yet corrupted, flips to the adversary.
pub fn corrupt_station(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    take_for_adversary(g, cx, h);
    // 059: corruption replaces any marker here, as on a city.
    clear_markers_on(g, cx, h);
    cx.tally(g, "adversary.stations_flipped", 1);
    cx.note(Some(h), || {
        format!("The station at {} flips to the adversary", hex_name(&w, h))
    });
}

/// Factory `h` becomes the adversary's (070: a threat's target): any player
/// cube comes off, its cube goes on, and it replaces any marker.
pub fn corrupt_factory(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    take_for_adversary(g, cx, h);
    clear_markers_on(g, cx, h);
    cx.tally(g, "adversary.factories_seized", 1);
    cx.note(Some(h), || format!("The factory at {} goes to the adversary", hex_name(&w, h)));
}

/// Where an ambassador spreading from `from` may go: hexes within `range`
/// holding a piece (never an empty hex), not shielded, and
/// without an adversary ambassador unless a clean or aligned city (cities
/// collect them) or a piece with no node (a second ambassador makes a node).
/// Returns the best by priority (player owned, then clean cities, then the
/// rest), then distance.
fn destinations(w: &World, g: &GameState, from: usize, range: i32) -> Vec<usize> {
    let shields: Vec<usize> = (0..g.seats.len())
        .filter(|&s| {
            cards::passives(w, g, s)
                .iter()
                .any(|p| matches!(p, Op::SpreadShield))
        })
        .collect();
    let mut best: Option<(u8, i32)> = None;
    let mut out = Vec::new();
    for h in 0..g.hexes.len() {
        let d = w.map.distance(from, h);
        if h == from || d > range || is_ocean(w, h) {
            continue;
        }
        if !has_piece(w, g, h) {
            continue;
        }
        if shields.iter().any(|&s| g.tokens(h, presence(s)) > 0) {
            continue;
        }
        let city = collecting_city(w, g, h);
        if g.tokens(h, ADV_AMBASSADOR) > 0 && !city && g.tokens(h, NODE) > 0 {
            continue;
        }
        let tier = if player_owned(g, h) {
            0
        } else if city {
            1
        } else {
            2
        };
        let key = (tier, d);
        match best {
            Some(b) if key > b => {}
            Some(b) if key == b => out.push(h),
            _ => {
                best = Some(key);
                out = vec![h];
            }
        }
    }
    out
}

/// The spread step, in country `scope` or everywhere: each hex (fixed when
/// the step starts) holding several adversary ambassadors, except a city
/// that is not corrupted, keeps one and sends each other one to the best
/// destination in range (players choose among equals).
pub fn spread(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    let range = w.int("adversary.spread_range")
        + twist_sum(&w, g, |t| match t {
            Op::SpreadReach { amount } => Some(*amount),
            _ => None,
        });
    let sources: Vec<usize> = scope_hexes(&w, scope)
        .into_iter()
        .filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 1 && !collecting_city(&w, g, h))
        .collect();
    for from in sources {
        let n = g.tokens(from, ADV_AMBASSADOR) - 1;
        for _ in 0..n {
            let dest = destinations(&w, g, from, range as i32);
            if dest.is_empty() {
                // Nowhere to go: the surplus returns to the supply.
                let left = g.tokens(from, ADV_AMBASSADOR) - 1;
                remove(g, cx, from, left);
                cx.tally(g, "adversary.spread_returned", left);
                break;
            }
            let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!(
                "Adversary spread: an ambassador from {} goes to which hex?",
                hex_name(&w, from)
            );
            let Some(i) = cx.choose(0, &prompt, opts)? else {
                break;
            };
            cx.add_tokens(g, from, ADV_AMBASSADOR, -1);
            cx.per_round(g, OPS, 1);
            arrive(g, cx, dest[i], true);
        }
    }
    Ok(())
}

/// Whether a removal of `ambassadors` or `troops` has a piece to take on hex
/// `h` (F64: a card that names no troop can't touch one).
pub fn can_damage(w: &World, g: &GameState, h: usize, ambassadors: i64, troops: i64) -> bool {
    (ambassadors > 0 && g.tokens(h, ADV_AMBASSADOR) > 0)
        || (troops > 0 && g.tokens(h, TROOP) > 0 && !troops_shielded(w, g, h))
}

/// A face-up military threat in `h`'s country holds the troops on it (070).
fn troops_shielded(w: &World, g: &GameState, h: usize) -> bool {
    threats::shielded(w, g, h, ThreatSource::Military)
}

/// Whether a "remove everything on the hex" has a piece to take on `h`.
pub fn can_destroy_all(w: &World, g: &GameState, h: usize) -> bool {
    can_damage(w, g, h, 1, 1)
}

/// In hex `h`, removes up to `ambassadors` adversary ambassadors or up to
/// `troops` troops: one kind, seat `seat`'s choice where the hex holds both.
pub fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
    let shielded = troops_shielded(&w, g, h);
    let kinds: Vec<(&str, i64)> = [(ADV_AMBASSADOR, ambassadors), (TROOP, troops)]
        .into_iter()
        .filter(|(k, n)| *n > 0 && g.tokens(h, k) > 0 && !(shielded && *k == TROOP))
        .collect();
    let chosen = match kinds.as_slice() {
        [] => return Ok(()),
        [only] => *only,
        _ => {
            let opts = kinds
                .iter()
                .map(|(k, n)| opt(format!("remove {} {} at {}", (*n).min(g.tokens(h, k)), piece_name(k), hex_name(&w, h))))
                .collect();
            let prompt = format!("Seat {seat}: remove which pieces?");
            let Some(i) = cx.choose(seat, &prompt, opts)? else {
                return Ok(());
            };
            kinds[i]
        }
    };
    for _ in 0..chosen.1.min(g.tokens(h, chosen.0)) {
        remove_piece(g, cx, Some(seat), h, chosen.0);
    }
    Ok(())
}

/// Removes one adversary piece of `kind` from hex `h`. A removed troop
/// sways toward Unity.
pub fn remove_piece(g: &mut GameState, cx: &mut Cx, seat: Option<usize>, h: usize, kind: &str) {
    if kind == TROOP {
        cx.add_tokens(g, h, TROOP, -1);
        cx.tally(g, "adversary.troops_removed", 1);
        alignment::for_troop_removed(g, cx, seat);
    } else {
        remove(g, cx, h, 1);
        cx.tally(g, "adversary.ambassadors_removed", 1);
    }
}

/// Removes every adversary ambassador and troop in hex `h`.
pub fn destroy_all(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    let n = g.tokens(h, ADV_AMBASSADOR);
    for _ in 0..n {
        remove_piece(g, cx, Some(seat), h, ADV_AMBASSADOR);
    }
    if troops_shielded(&w, g, h) {
        return;
    }
    for _ in 0..g.tokens(h, TROOP) {
        remove_piece(g, cx, Some(seat), h, TROOP);
    }
}

/// Removes one adversary ambassador in country `c` (players choose the hex).
pub fn pushback(g: &mut GameState, cx: &mut Cx, c: usize) -> Res {
    let w = cx.world;
    let hexes: Vec<usize> = w.map.countries[c]
        .hexes
        .iter()
        .copied()
        .filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 0)
        .collect();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    if let Some(i) = cx.choose(
        0,
        &format!("Unity: remove an adversary ambassador in {} from which hex?", country_name(&w, c)),
        opts,
    )? {
        remove_piece(g, cx, None, hexes[i], ADV_AMBASSADOR);
    }
    Ok(())
}

/// In each country, the clean city with the most adversary ambassadors
/// (ties: lower hex number) gains `n` more (Discord cards, Infiltration Drive).
pub fn infiltrate(g: &mut GameState, cx: &mut Cx, n: i64) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        if let Some(h) = most_infiltrated_city(&w, g, c, |h| clean_city(&w, g, h)) {
            // No source in the note: a project calls this too (020: it said "Discord").
            cx.note(Some(h), || {
                format!("{n} adversary ambassador{} gather{} at {}", if n == 1 { "" } else { "s" }, if n == 1 { "s" } else { "" }, hex_name(&w, h))
            });
            place(g, cx, h, n);
        }
    }
}

/// In country `c`, the hex passing `pred` with the most adversary
/// ambassadors (ties: the first in the country's hex order), if any.
pub fn most_infiltrated_city(
    w: &World,
    g: &GameState,
    c: usize,
    pred: impl Fn(usize) -> bool,
) -> Option<usize> {
    let mut best: Option<(usize, i64)> = None;
    for &h in &w.map.countries[c].hexes {
        if pred(h) {
            let k = g.tokens(h, ADV_AMBASSADOR);
            if best.is_none_or(|(_, b)| k > b) {
                best = Some((h, k));
            }
        }
    }
    best.map(|(h, _)| h)
}

/// What still counts against the purge at `level` (the Unity thresholds
/// reached): corrupted cities always; adversary ambassadors below level 1;
/// adversary troops below level 2; nodes below level 3 (019; with two
/// thresholds, always).
pub fn purge_count(w: &World, g: &GameState, level: i64) -> i64 {
    (0..g.hexes.len()).map(|h| counts_against(w, g, h, level)).sum()
}

/// What hex `h` adds to [`purge_count`].
fn counts_against(w: &World, g: &GameState, h: usize, level: i64) -> i64 {
    against_bar(corrupted_city(w, g, h), g.tokens(h, NODE), g.tokens(h, ADV_AMBASSADOR), g.tokens(h, TROOP), level)
}

/// What a hex with these pieces adds to [`purge_count`] at bar `level`.
pub fn against_bar(corrupted_city: bool, nodes: i64, ambassadors: i64, troops: i64, level: i64) -> i64 {
    let mut n = corrupted_city as i64;
    if level < 3 {
        n += nodes;
    }
    if level < 1 {
        n += ambassadors;
    }
    if level < 2 {
        n += troops;
    }
    n
}

/// Whether [`purge_count`] is 0, stopping at the first piece: every action asks.
pub fn under_bar(w: &World, g: &GameState, level: i64) -> bool {
    !(0..g.hexes.len()).any(|h| counts_against(w, g, h, level) > 0)
}

pub fn win_text(level: i64) -> String {
    match level {
        0 => "purge: no corrupted city, node, adversary troop or adversary ambassador on the board"
            .into(),
        1 => {
            "purge: no corrupted city, node or adversary troop (ambassadors no longer count)".into()
        }
        2 => "purge: no corrupted city or node (ambassadors and troops no longer count)".into(),
        _ => "purge: no corrupted city (ambassadors, troops and nodes no longer count)".into(),
    }
}

/// A board loss, if any: a player with no presence on the board (locked,
/// 027). (The deck running out ends the game in the build step, and the
/// Discord deck running out in the tip.)
pub fn loss(g: &GameState) -> Option<(&'static str, String)> {
    let gone = (0..g.seats.len()).find(|&s| !has_presence(g, s))?;
    Some(("presence", format!("seat {gone} has no presence left on the board")))
}
