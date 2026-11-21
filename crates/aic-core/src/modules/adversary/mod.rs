//! The adversary (014, 016, 017): its deck (the clock and the stager), its
//! ambassadors (creation lines, Corrupt X, natural corruption; 138), botnets, adversary compute and the
//! project deck it fills, the ambassador supply and the difficulty levels.
//! Also the shared rules for placing and removing agents and
//! troops, which cards, threats and Alignment use, and the board conditions
//! for winning and losing.

use crate::content::{AdversaryCard, Op, ProjectCard, PullTag, ThreatTag};
use crate::op::{CorruptSite, Step, Where};
use crate::key::{Key, KeyBuf};
use crate::module::{Build, Cx, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{alignment, board, cards, effects, plan};

mod bar;
mod corrupt;
mod pieces;
mod projects;
mod lines;
mod research;

pub use research::*;
pub use bar::*;
pub use corrupt::*;
pub use pieces::*;
pub use projects::*;
pub use lines::*;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
    ParamSpec::list("stage_cards", 0, 20, "adversary cards dealt from stage I, II and III (before difficulty)"),
    ParamSpec::int("supply_per_country", 0, 40, "agents in the supply per country"),
    ParamSpec::int("corrupt_at", 1, 20, "a city's corrupt number: the agents on it that corrupt it (coastal cities too)"),
    ParamSpec::int("corrupt_station_at", 1, 20, "a research station's corrupt number"),
    ParamSpec::int("corrupt_factory_at", 1, 20, "a factory's corrupt number"),
    ParamSpec::int("corrupt_node_at", 1, 20, "a node's corrupt number: a hex with a player's node and this many agents loses the node"),
    ParamSpec::list("corrupt_cut", 0, 5, "difficulty corrupt_cut_at+: how much lower each corrupt number is (city, station, factory)"),
    ParamSpec::int("corrupt_cut_at", 0, 5, "the difficulty level from which corrupt_cut applies"),
    ParamSpec::int("city_cut_at", 0, 5, "263: the difficulty level from which a city (coastal too) corrupts with one agent fewer again, after corrupt_cut (5: never)"),
    ParamSpec::int("blight_discord", 0, 5, "Discord each Fallout token sways each round"),
    ParamSpec::list("counter_pct", 50, 300, "every resource counter item of a tier 1, 2 threat costs this percent of its printed amount (rounded up)"),
    ParamSpec::int("counter_credit_pct", 50, 300, "every credits counter item costs this percent of its amount after counter_pct (rounded up; 100 = unchanged)"),
    ParamSpec::int("jam_floor", 0, 1, "274: 1, a jam never takes a threat's resource item below 1 (a jam alone never counters); 0, below to 0"),
    ParamSpec::int("stage1_removed", 0, 3, "difficulty stage_swap_at+: stage-I cards removed"),
    ParamSpec::int("stage3_added", 0, 3, "difficulty stage_swap_at+: stage-III cards added (the cap holds, its end gets harder)"),
    ParamSpec::int("stage2_added", 0, 3, "267: difficulty stage_swap_at+: stage-II cards added back (tier-2 threats sooner, built into the stack)"),
    ParamSpec::int("stage_swap_at", 0, 5, "the difficulty level from which stage1_removed, stage2_added and stage3_added apply"),
    ParamSpec::int("stage2_removed", 0, 3, "difficulty 3+: stage-II cards removed (a shorter round cap)"),
    ParamSpec::int("stage1_added", 0, 3, "difficulty 3+: stage-I cards added back (with stage2_removed, a smaller d3 step)"),
    ParamSpec::int("extra_corrupted", 0, 3, "difficulty 4: more cities per country start corrupted (none past the cities the map prints corrupted, 2 a country: nothing while `board.start_corrupted` is 2)"),
    ParamSpec::list("compute_income", 0, 30, "the adversary's base compute income a country a round, at difficulty 0-4 (times the countries: per board)"),
    ParamSpec::list("setup_city_agents", 0, 5,"301: agents on each city that starts free (not corrupted), at setup, at difficulty 0-4 (toward its corrupt number; the city is blocked)"),
    ParamSpec::list("infiltrate_agents", 0, 5,"275: agents an Infiltrate line lands on each city it names, at difficulty 0-4"),
    ParamSpec::int("infiltrate_facility_cap", 0, 5, "282: at most this many of an Infiltrate line's agents land on each research station, factory or botnet hex it names (cities take every one)"),
    ParamSpec::int("threat_agents", 0, 5, "275: agents an uncountered threat lands on its anchor (its first location token) as it executes"),
    ParamSpec::int("discord_removed", 0, 3, "difficulty 4+: Discord cards removed at setup (from tier 1)"),
    ParamSpec::int("station_flip_at", 0, 5, "agents on a player's research station before an effect can flip it (short of that, one lands there instead)"),
    ParamSpec::int("botnet_compute", 0, 5, "a botnet's one output: adversary compute each makes a round (none with a player ambassador on it)"),
    ParamSpec::int("botnet_spread_at", 0, 5, "263: the difficulty level from which a 'Botnets spread' line (event cards, projects) spreads botnets; below it the line does nothing"),
    ParamSpec::int("board_difficulty_at", 0, 5, "the difficulty level from which the adversary board's `difficulty` line applies (267: the Militarist's War Footing, level 2's named rule)"),
    ParamSpec::list("research_at", 0, 30, "293: K, the research cubes a country that advance the adversary's research track one slot, at difficulty 0-4 (times the countries; 0: no track)"),
    ParamSpec::int("research_start", 0, 30, "293: research cubes a country on the adversary's track at setup"),
    ParamSpec::int("arms_race_at", 0, 5, "r144: the difficulty level from which a research slot's troop line lands arms_race_troops more troops a country (the Militarist's Arms Race, level 3's named rule)"),
    ParamSpec::int("arms_race_troops", 0, 3, "r144: the troops a country a research slot's troop line adds from arms_race_at (Arms Race)"),
];

/// r80 (ruling #41): the adversary turn's three beats (275; r80 had four), printed as a strip on the adversary board, with the round
/// phases each runs (working names: a rename is this one edit). Threat resolution stays its own round step. 247
/// (241's pick): read only by a test that holds the round's phases to the strip, so built only for tests.
#[cfg(test)]
pub const BEATS: [(&str, &[&str]); 2] = [
    ("Card", &["adversary.event", "threats.deal", "adversary.build"]),
    ("Income", &["adversary.reveal", "adversary.compute"]),
];

/// The beat a round phase belongs to (none: not part of the adversary turn).
#[cfg(test)]
pub fn beat_of(phase: &str) -> Option<&'static str> {
    BEATS.iter().find(|(_, ps)| ps.contains(&phase)).map(|(b, _)| *b)
}

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto("build", "the Card beat after the threats: Escalate (each site of last round's type that holds an agent takes 1 more), the board's one rule, Infiltrate X (agents on each named site), Strike (every troop destroys a player piece on or next to it), Redeploy (each troop moves onto its country's corrupted city nearest a player node or ambassador) and the card's other lines; each arrival checks the site it lands on"),
    PhaseSpec::auto("reveal", "reveal the next event card face up, a round ahead (its Infiltrate sites shown: they escalate the round after)"),
    PhaseSpec::auto("event", "the face-up event card becomes this round's (none left: the adversary wins)"),
    // 178 (amendment 8112): its own phase, so a project's prompt doesn't follow the reveal's hidden draw in one
    // phase (a determinized replay of it could resolve another project first). No play change.
    PhaseSpec::auto("compute", "the Income beat: each corrupted site makes its one output (a city 1 Discord, a station 1 research cube, a factory 1 agent after pushing its last along its arrow, a botnet its compute); then income and the compute banked since the last turn, all onto the leftmost project, which resolves as it fills; then the research track advances a slot for every K cubes; then every firewall, cordon and prohibition token comes off: the adversary turn ends"),
];

// `run`'s phase indices: each is its phase's position in `PHASES`.
const BUILD: usize = 0;
const REVEAL: usize = 1;
const EVENT: usize = 2;
const COMPUTE_STEP: usize = 3;

pub static DEF: ModuleDef = ModuleDef {
    id: "adversary",
    summary: "The adversary deck (the clock), agents (creation lines, Corrupt X), botnets, the supply, difficulty",
    requires: &["board", "alignment"],
    variants: &[
        VariantDef {
            name: "militarist",
            summary: "The Militarist: Infiltrate and Escalate, a research row, troops that garrison and Strike",
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
/// 293: last round's card, kept face up in the board's "last round" space: its Infiltrate types escalate.
pub const LAST: &str = "adversary.last";
/// Agents in the supply.
pub const SUPPLY: &str = "adversary.supply";
/// Stage of this round's card.
pub const STAGE: &str = "adversary.stage";
/// How many more deck cards beyond the next are shown (Recon Sweep).
pub const PEEK: &str = "adversary.peek";
/// Adversary compute made so far (all of it goes onto projects).
pub const COMPUTE: &str = "adversary.compute";
/// The project deck (face down), the visible row (in slot order), and resolved
/// or discarded ones (231: none is ongoing). The draw rule's misses go to the
/// project discard, which reshuffles into the deck (135).
pub const PROJECT_DECK: &str = "adversary.project_deck";
pub const PROJECT_DISCARD: &str = "adversary.project_discard";
pub const PROJECTS: &str = "adversary.projects";
pub const PROJECTS_DONE: &str = "adversary.projects_done";
/// 151: the adversary's compute income a round (base, raised by "increase income" lines).
pub const INCOME_COMPUTE: &str = "adversary.income_compute";

/// The per-round counter of adversary operations (table time).
pub const OPS: &str = "adversary.ops";
/// 275: the per-round counter of per-site checks the adversary turn makes (each site a line names, each
/// arrival, each botnet): table time, exported by round (`map.`).
pub const CHECKS: &str = "map.adversary_checks";
/// The round the first project resolved (sim observation).
const FIRST_PROJECT: &str = "adversary.first_project";
/// The round of the first corruption after setup (sim observation).
const FIRST_CORRUPTION: &str = "adversary.first_corruption";

/// Set on a botnet's hex by a hack (021): it makes no compute in the next build.
pub fn cut_key(h: usize) -> String {
    format!("adversary.cut.{h}")
}

/// Whether the configured difficulty is at least `n` (setup rules).
pub fn level(w: &World, n: i64) -> bool {
    w.int("adversary.difficulty") >= n
}

/// 151: a per-difficulty list param's entry at this game's difficulty (the last entry past the list's end): every
/// ladder list (`compute_income`, `infiltrate_agents`, `research_at`, `setup_city_agents`) reads through it.
pub(super) fn at_level(w: &World, param: &str) -> i64 {
    let l = w.list(param);
    let d = w.int("adversary.difficulty").max(0) as usize;
    l.get(d).or(l.last()).copied().unwrap_or(0)
}

/// The ambassador supply at the start, for the map's countries.
pub fn start_supply(w: &World) -> i64 {
    (w.int("adversary.supply_per_country") * w.map.countries.len() as i64).max(0)
}

/// The tier an extra threat comes from (an old-form `DealThreats`): this
/// round's card's stage (267: no look-ahead at any level; the ladder builds
/// the deck instead). Event cards' pulls name their own tiers (135).
pub fn threat_tier(g: &GameState) -> u8 {
    // 146: no tier 3; stages II and III deal tier 2.
    if g.var(STAGE) >= 2 { 2 } else { 1 }
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
    g.pile(round_pile(g)).iter().flat_map(|id| card(w, id).twists.iter()).collect()
}

/// r141: a seat has no node left and that loses (`alignment.node_loss`): the game ends as the phase does, so what
/// would follow a Strike in the same phase stops here.
pub fn node_lost(w: &World, g: &GameState) -> bool {
    w.hot(crate::module::Hot::NodeLoss) > 0 && loss(g).is_some()
}

/// r141: whether the card of the next Card beat prints `line` (a Strike or a Redeploy icon, shown a round ahead):
/// this round's card until its Card beat has run, then next round's, face up in `NEXT` from the reveal (inside the
/// beat, paused on a question, its own lines: the viewer says "this Card beat").
pub fn line_ahead(w: &World, g: &GameState, line: &Op) -> bool {
    let built = g.phase_index("adversary.build").is_some_and(|i| g.phase > i);
    let pile = if built { NEXT } else { round_pile(g) };
    g.pile(pile).iter().take(1).any(|id| card(w, id).standing.iter().any(|e| e == line))
}

/// r141: whether the research track reaches a Strike slot at the next research check, on the cubes it will hold
/// then: the cubes on it now and one from each corrupted station no player ambassador blocks
/// (the row and both counts are public: the other way a Strike is seen coming; a threat's or project's cubes
/// can still bring it sooner).
pub fn strike_slot_ahead(w: &World, g: &GameState) -> bool {
    if !research_on(w) {
        return false;
    }
    let stations = (0..g.hexes.len())
        .filter(|&h| g.tokens(h, STATION) > 0 && corrupted(g, h) && player_ambassador(g, h).is_none())
        .count() as i64;
    // Every slot those cubes reach at the next check (one check can advance several).
    let slots = (g.var(RESEARCH) + stations * w.int("board.station_cubes")) / research_threshold(w);
    let from = g.var(RESEARCH_SLOT);
    (from + 1..=from + slots).any(|k| slot_icon(w, k).is_some_and(|i| i.effects.iter().any(|e| matches!(e, Op::Strike))))
}

/// The pile holding this round's adversary card: the face-up one before this
/// round's flip (a Discord tip in the Sway phase), else the current one.
pub fn round_pile(g: &GameState) -> &'static str {
    let flip = if g.phase_index("adversary.event").is_some() { "adversary.event" } else { "adversary.build" };
    let before_build = g.phase_index(flip).is_some_and(|i| g.phase < i);
    if before_build { NEXT } else { CURRENT }
}

#[derive(Debug)]
struct Adversary {
    /// What this difficulty deals from stage I, II and III.
    dealt: [i64; 3],
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let stage_cards = b.list("stage_cards")?;
    if stage_cards.len() != 3 {
        return Err("stage_cards: give 3 counts (stage I, II, III)".into());
    }
    // 044: what this difficulty deals, for setup and the check (which used to
    // add stage3_added at every level and take off no removal).
    let difficulty = b.int("difficulty")?;
    // 142: the stage-I-for-stage-III swap adds threats (a stage-III card pulls twice), so it sits at
    // `stage_swap_at` (4) beside the extra threat a round (Matt, message 5982).
    let swap = difficulty >= b.int("stage_swap_at")?;
    let have = |s: usize| b.world.content.adversary.iter().filter(|c| c.stage as usize == s + 1).count() as i64;
    // 200 (r88): 196's `stage_extra` (two more event cards at d0) is gone: d0's band comes from the players' side.
    let dealt = [
        (stage_cards[0] + if difficulty >= 3 { b.int("stage1_added")? } else { 0 } - if swap { b.int("stage1_removed")? } else { 0 }).max(0),
        // 267: d4's "tier-2 threats sooner" is the stack's build: a stage-II card in a stage-I card's place.
        (stage_cards[1] - if difficulty >= 3 { b.int("stage2_removed")? } else { 0 } + if swap { b.int("stage2_added")? } else { 0 })
            .max(0),
        stage_cards[2] + if swap { b.int("stage3_added")? } else { 0 },
    ];
    // An empty deck loses before round 1's first decision (020: it was accepted).
    if dealt.iter().sum::<i64>() < 1 {
        return Err("stage_cards: deal at least one card (the deck is the game's clock)".into());
    }
    for (s, &n) in dealt.iter().enumerate() {
        let have = have(s);
        if n > have {
            return Err(format!(
                "stage_cards: stage {} has only {have} cards",
                s + 1
            ));
        }
    }
    Ok(Box::new(Adversary { dealt }))
}

/// The face-up event card becomes this round's (135: the `event` phase; in a
/// round without one, the build does it). None left: the adversary wins.
fn flip(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    // 293: this round's card moves to the "last round" space (its types escalate next round), the one there to
    // the discard.
    for id in g.pile(LAST).to_vec() {
        cx.move_card(g, &id, LAST, DISCARD, true).expect("last round's card");
    }
    for id in g.pile(CURRENT).to_vec() {
        cx.move_card(g, &id, CURRENT, LAST, true)
            .expect("current card");
    }
    let Some(id) = g.pile(NEXT).first().cloned() else {
        cx.end(g, false, "deck_empty", "the adversary deck ran out".into());
        return;
    };
    cx.move_card(g, &id, NEXT, CURRENT, true)
        .expect("next card");
    let c = card(&w, &id);
    cx.set_key(g, STAGE, c.stage as i64);
    cx.note(None, || {
        format!("Adversary card: {} ({})", c.name, card_text(c))
    });
}

/// The board's difficulty line's extra troops on every garrison line (135; an event's, a project's, a research
/// slot's; no board prints one since 267), from `adversary.board_difficulty_at`.
pub fn troop_bonus(w: &World) -> i64 {
    if !level(w, w.int("adversary.board_difficulty_at")) {
        return 0;
    }
    w.content
        .adversary_board
        .difficulty
        .effects
        .iter()
        .map(|e| match e {
            Op::StandingTroopsBonus { amount } => *amount,
            _ => 0,
        })
        .sum()
}

/// 146: the compute the adversary board prints for each threat that executes
/// uncountered: its rules' `ThreatCompute` lines, and its difficulty line's from
/// `adversary.board_difficulty_at`.
pub fn threat_compute(w: &World) -> i64 {
    let b = &w.content.adversary_board;
    let read = |ops: &[Op]| -> i64 { ops.iter().map(|e| if let Op::ThreatCompute { amount } = e { *amount } else { 0 }).sum() };
    let base: i64 = b.rules.iter().map(|r| read(&r.effects)).sum();
    let hard = if level(w, w.int("adversary.board_difficulty_at")) { read(&b.difficulty.effects) } else { 0 };
    base + hard
}

/// 293: the adversary board's one rule, where it prints a Card-beat line (the Militarist's Commandeered Plants: an
/// Escalate of its own every round): run after the card's escalate, before its Infiltrate. A rule read elsewhere
/// (an income or threat-compute line) is no line here.
fn board_rule(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for r in &w.content.adversary_board.rules {
        for e in r.effects.iter().filter(|e| matches!(e, Op::Escalate { .. })) {
            effects::run(g, cx, std::slice::from_ref(e), &mut effects::Ctx::team().named("board").why(&r.name))?;
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// The event card's standing lines (135): its Infiltrate, Strike, Redeploy and other lines, resolved as the
/// team's.
fn standing(g: &mut GameState, cx: &mut Cx, c: &AdversaryCard) -> Res {
    for e in &c.standing {
        effects::run(g, cx, std::slice::from_ref(e), &mut effects::Ctx::team().named("standing").why(&c.name))?;
        // r141: a Strike that took a seat's last node is the loss: no later line of the card runs.
        if g.is_over() || node_lost(&cx.world, g) {
            break;
        }
    }
    Ok(())
}

impl Module for Adversary {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        cx.set_key(g, SUPPLY, start_supply(&w));
        // Each country starts with a botnet on its (corrupted) research station.
        for c in 0..w.map.countries.len() {
            for &n in &w.content.map.countries[c].stations {
                cx.add_tokens(g, w.map.numbered(c, n), BOTNET, 1);
            }
        }
        // 301 (level 0's setup, a ladder number): `setup_city_agents` agents from the supply on each city that starts
        // free (the board's printed corruption order picks the others): the adversary is an arrival or two from
        // taking a new city, so the players can't liberate the starting ones before it spreads.
        let seeds = at_level(&w, "adversary.setup_city_agents");
        if seeds > 0 {
            let corrupted = board::start_corrupted_cities(&w);
            for h in 0..g.hexes.len() {
                if !is_city(&w, h) || corrupted.contains(&h) {
                    continue;
                }
                let n = seeds.min(g.var(SUPPLY));
                cx.add_tokens(g, h, ADV_AMBASSADOR, n);
                cx.add_key(g, SUPPLY, -n);
                cx.tally(g, "adversary.setup_agents", n);
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
        // 293: the research track's starting cubes.
        let start = w.int("adversary.research_start").max(0) * w.map.countries.len() as i64;
        if start > 0 && research_on(&w) {
            cx.set_key(g, RESEARCH, start);
        }
        // 151: the compute income, from the config's level and the board's own lines (275: no agent income).
        let mut compute = at_level(&w, "adversary.compute_income");
        for e in w.content.adversary_board.rules.iter().flat_map(|r| r.effects.iter()) {
            if let Op::IncreaseIncome { compute: c } = e {
                compute += c;
            }
        }
        // 231 (Matt): compute is per board: the income and every printed compute line, times the countries.
        cx.set_key(g, INCOME_COMPUTE, compute * w.map.countries.len() as i64);
        // The project deck (135): one shuffled deck, dealt into the board's slots.
        let mut ids: Vec<Key> = w.content.projects.iter().map(|c| Key::new(&c.id)).collect();
        shuffle(&mut ids, cx.rng(g, "adversary"));
        cx.set_pile(g, PROJECT_DECK, ids);
        deal_projects(g, cx);
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match phase {
            EVENT => flip(g, cx),
            BUILD => {
                // A round with no event phase flips the card here (lab rounds).
                if g.phase_index("adversary.event").is_none() {
                    flip(g, cx);
                    if g.is_over() {
                        return Ok(());
                    }
                }
                let Some(id) = g.pile(CURRENT).first().cloned() else {
                    return Ok(());
                };
                let c = card(&w, &id);
                // 293: escalate first (last round's type), then the board's one rule, then this card's lines.
                escalate_last(g, cx)?;
                if g.is_over() {
                    return Ok(());
                }
                board_rule(g, cx)?;
                if g.is_over() {
                    return Ok(());
                }
                standing(g, cx, c)?;
                if g.is_over() || node_lost(&w, g) {
                    return Ok(());
                }
                for t in &c.twists {
                    match t {
                        // These resolve as they do anywhere (034, 044).
                        Op::Sway { .. } | Op::EachDestroyNode { .. } | Op::PullUnity { .. } | Op::Fallout { .. } => {
                            effects::run(g, cx, std::slice::from_ref(t), &mut effects::Ctx::team().named("twist").why(&c.name))?
                        }
                        // The load check lets no other op in: one added to `Slot::Twist`
                        // needs a reader (044: `_ => {}` swallowed the Unity pull).
                        other => unreachable!("twist with no reader: {other:?}"),
                    }
                    if g.is_over() {
                        return Ok(());
                    }
                }
                // Botnets at their peak, per board (country).
                for (c, country) in w.map.countries.iter().enumerate() {
                    let n = botnet_count(g, Some(&country.hexes));
                    cx.peak(g, &format!("adversary.botnets_peak.c{c}"), n);
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
            REVEAL => reveal(g, cx),
            COMPUTE_STEP => {
                // 293: the outputs (each corrupted site's one), then the compute onto the projects.
                if !g.is_over() {
                    compute_step(g, cx)?;
                }
                // 293: then the research track: a slot for every K cubes.
                if !g.is_over() {
                    research_step(g, cx)?;
                }
                // 158: firewalls come off at the end of the adversary turn after they went on; r80 (#53) so do
                // prohibition tokens.
                for h in 0..g.hexes.len() {
                    for kind in [FIREWALL, prohibit_token(Step::Infiltrate), prohibit_token(Step::Strike), prohibit_token(Step::Corrupt)] {
                        let n = g.tokens(h, kind);
                        if n > 0 {
                            cx.add_tokens(g, h, kind, -n);
                            // 285: said (the turn panel showed only "-1 firewall").
                            let w = cx.world;
                            cx.note(Some(h), || {
                                let what = if kind == FIREWALL { "The firewall" } else { "The prohibition token" };
                                format!("{what} at {} comes off", hex_name(&w, h))
                            });
                        }
                    }
                }
                // 178 (Matt's ruling #30): so does a cordon (it lasted to the end of the next round).
                for c in 0..cx.world.map.countries.len() {
                    let k = board::flag(crate::content::RoundFlag::Cordon, c);
                    if g.var(&k) > 0 {
                        cx.set_key(g, &k, 0);
                    }
                }
                // 167: so does a prohibition whose adversary turn this was.
                for step in Step::ALL {
                    let (rk, wk) = super::plan::prohibit_keys(step);
                    let r = g.var(rk);
                    if r != 0 && r <= g.round as i64 {
                        cx.set_key(g, rk, 0);
                        cx.set_key(g, wk, 0);
                    }
                }
            }
            _ => unreachable!("adversary phase {phase}"),
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
        let last: Vec<CardView> = g.pile(LAST).iter().map(|id| show(id, "last round: its sites escalate")).collect();
        out.pile("last", "Last round's card (its sites escalate)", None, last.len(), last);
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
            "agent supply",
            g.var(SUPPLY),
            Some(start_supply(w)),
            "agents left: a cap, a placement it can't make doesn't happen",
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
                "adversary compute made so far: each botnet makes {} a round (none under a player ambassador); it fills the leftmost visible project",
                w.int("adversary.botnet_compute")
            ),
        );
        out.global(
            "income_compute",
            "adv. compute income",
            g.var(INCOME_COMPUTE),
            None,
            "adversary compute income a round, added in its compute step with every other source",
        );
        out.global(
            "compute_waiting",
            "adv. compute waiting",
            g.var(WAITING) + g.var(WAITING_EVERY) * g.pile(PROJECTS).len() as i64,
            None,
            "compute made since the adversary's last turn: it goes onto the projects in its next compute step",
        );
        research_view(g, w, out);
        // 293: the map shows which threat is growing: each site the next Card beat's Escalate lands on (a site of
        // the escalating type, not yet the adversary's, holding an agent), as an arrow onto itself, kind `escalate`.
        let mut marked: Vec<usize> = Vec::new();
        for site in next_escalating(w, g) {
            for c in 0..w.map.countries.len() {
                for h in super::plan::infiltrate_sites(w, g, site, c) {
                    // As the rule: a firewalled or no-Infiltrate site and a cordoned country take none.
                    let lands = !firewalled(g, h) && !super::plan::prohibited_now(g, Step::Infiltrate, h) && !super::plan::cordoned(w, g, h);
                    if g.tokens(h, ADV_AMBASSADOR) > 0 && lands && !marked.contains(&h) {
                        marked.push(h);
                        out.arrow(
                            "escalate",
                            h,
                            h,
                            1,
                            1,
                            &format!("{} takes 1 more agent in the adversary's next Card beat (Escalate: {}) unless its agents are cleared first", hex_name(w, h), site.plural()),
                        );
                    }
                }
            }
        }
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
            let kind: Vec<&str> = p.tags.iter().map(|t| t.name()).collect();
            let kind = kind.join(" + ");
            CardView {
                id: id.to_string(),
                name: p.name.clone(),
                // 235: the card prints one cost a board; say the sum it shows ("0/52" for a printed 13).
                text: {
                    let n = w.map.countries.len().max(1);
                    let c = if n == 1 { "country" } else { "countries" };
                    format!("{kind}: {} Cost: {} a board × {n} {c}.", p.text, p.cost)
                },
                status: status.to_string(),
                cost: Some(project_cost(w, g, id)),
            }
        };
        // 238 (D100): a project the next compute step fills, as the board stands, says so.
        let next = projects::completes_next_step(w, g);
        let fill = |id: &str| {
            let soon = if next.iter().any(|k| k.as_str() == id) { ", completes next compute step" } else { "" };
            format!("{}/{} compute{soon}", g.var(&project_key(id)), project_cost(w, g, id))
        };
        let row = g.pile(PROJECTS);
        out.pile(
            "projects",
            "Adversary projects (compute fills the leftmost)",
            None,
            row.len(),
            row.iter().map(|id| pshow(id, &fill(id))).collect(),
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
        let missed = g.pile(PROJECT_DISCARD);
        out.pile(
            "project_discard",
            "Adversary project discard (the draw rule's misses; reshuffles)",
            None,
            missed.len(),
            missed.iter().map(|id| pshow(id, "discarded")).collect(),
        );
        out.global(
            "deck",
            "adversary cards left",
            deck.len() as i64 + g.pile(NEXT).len() as i64,
            None,
            "rounds left on the clock after this one; players lose when the next round has no card",
        );
        // 169: a pending prohibition, as the printed game's marker would show it: the step that is off, where, and
        // the round whose adversary turn it acts in (its raw vars were an unlabeled bit mask).
        for step in Step::ALL {
            let (rk, wk) = super::plan::prohibit_keys(step);
            let round = g.var(rk);
            if round <= 0 {
                continue;
            }
            let mask = g.var(wk);
            let wheres: Vec<&str> =
                Where::ALL.iter().enumerate().filter(|(i, _)| mask & (1 << i) != 0).map(|(_, wh)| wh.words()).collect();
            let name = match step {
                Step::Infiltrate => "Infiltrate",
                Step::Strike => "Strike",
                Step::Corrupt => "Corrupt",
            };
            // 285: no Corrupt step since r130: a no-Corrupt marker lets agents land and stops the site corrupting.
            let what = match step {
                Step::Corrupt => {
                    format!("agents may land, but nothing corrupts or flips and no node is lost to agents on each {}", wheres.join("; each "))
                }
                _ => format!("{name} skips each {}", wheres.join("; each ")),
            };
            out.global(
                &format!("prohibit_{}", step.name()),
                &format!("no {name} in round"),
                round,
                None,
                &format!("a Unity prohibition: in round {round}'s adversary turn, {what}"),
            );
        }
        out.bookkeeping(&[
            "prohibit.",
            PEEK,
            FIRST_CORRUPTION,
            FIRST_PROJECT,
            "adversary.project.",
            "adversary.cut.",
            "adversary.slot.",
        ]);
    }
}

/// Shows the next `n` deck cards beyond the face-up one (Recon Sweep); a
/// longer peek already running is kept.
pub fn peek(g: &mut GameState, cx: &mut Cx, n: i64) {
    let cur = g.var(PEEK);
    cx.set_key(g, PEEK, cur.max(n));
}

/// Reveals the top deck card face up as next round's card (nothing if the deck is empty).
fn reveal(g: &mut GameState, cx: &mut Cx) {
    if let Some(id) = g.pile(DECK).first().cloned() {
        cx.move_card(g, &id, DECK, NEXT, true)
            .expect("top of the deck");
        // 285: the turn panel read this step as "cards changed": name the card turned up.
        let w = cx.world;
        cx.note(None, || format!("Next round's adversary card is turned up: {}", card(&w, &id).name));
        let peek = g.var(PEEK);
        if peek > 0 {
            cx.set_key(g, PEEK, peek - 1);
        }
    }
}

/// 167, a prohibition resolves: `step` won't act where `wh` holds in the next adversary turn (this round's
/// if its build is still ahead, else next round's). Another prohibition of the same step adds its condition.
pub fn prohibit(g: &mut GameState, cx: &mut Cx, step: Step, wh: Where) {
    let build_ahead = g.phase_index("adversary.build").is_some_and(|i| g.phase <= i);
    let round = g.round as i64 + if build_ahead { 0 } else { 1 };
    let (rk, wk) = super::plan::prohibit_keys(step);
    let bit = 1 << Where::ALL.iter().position(|&x| x == wh).unwrap_or(0);
    let mask = if g.var(rk) == round { g.var(wk) | bit } else { bit };
    cx.set_key(g, rk, round);
    cx.set_key(g, wk, mask);
    // r80 (ruling #53): a token on each hex the condition holds on now; the step reads the tokens, not the
    // condition, and they come off with the firewalls.
    let w = cx.world;
    for h in super::plan::sites_where(&w, g, wh) {
        if g.tokens(h, prohibit_token(step)) == 0 {
            cx.add_tokens(g, h, prohibit_token(step), 1);
        }
    }
    cx.tally(g, &format!("prohibit.{}.{}.made", step.name(), wh.name()), 1);
}

/// 167: a prohibition covered a site the step would act on (`fired`), keeping `n` arrivals or pulls off it.
pub(crate) fn prohibit_tally(g: &mut GameState, cx: &mut Cx, step: Step, wh: Where, n: i64) {
    cx.tally(g, &format!("prohibit.{}.{}.fired", step.name(), wh.name()), 1);
    cx.tally(g, &format!("prohibit.{}.{}.blocked", step.name(), wh.name()), n.max(0));
}

#[cfg(test)]
mod phase_tests {
    use super::*;

    /// `run`'s named indices point at the phases they name.
    #[test]
    fn phase_indices_match_phases() {
        let named = [
            (BUILD, "build"),
            (REVEAL, "reveal"),
            (EVENT, "event"),
            (COMPUTE_STEP, "compute"),
        ];
        assert_eq!(named.len(), PHASES.len());
        for (i, name) in named {
            assert_eq!(PHASES[i].name, name);
        }
    }
}
