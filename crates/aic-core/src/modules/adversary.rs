//! The adversary (014, 016, 017): its deck (the clock and the stager), its
//! ambassadors (creation lines, Corrupt X, natural corruption; 138), botnets, adversary compute and the
//! project deck it fills, the ambassador supply and the difficulty levels.
//! Also the shared rules for placing and removing agents and
//! troops, which cards, threats and Alignment use, and the board conditions
//! for winning and losing.

use crate::content::{AdversaryCard, Op, ProjectCard, PullTag, ThreatTag};
use crate::op::CorruptSite;
use crate::key::{Key, KeyBuf};
use crate::module::{Build, Cx, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{alignment, board, cards, effects};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
    ParamSpec::list("stage_cards", 0, 20, "adversary cards dealt from stage I, II and III (before difficulty)"),
    ParamSpec::int("supply_per_country", 0, 40, "agents in the supply per country"),
    ParamSpec::int("corrupt_at", 1, 20, "Corrupt city: agents a city gathers (on it and next to it) to corrupt"),
    ParamSpec::int("corrupt_station_at", 1, 20, "Corrupt station: agents a research station gathers to corrupt"),
    ParamSpec::int("corrupt_factory_at", 1, 20, "Corrupt factory: agents a factory gathers to corrupt"),
    ParamSpec::int("corrupt_land_at", 1, 20, "Corrupt land: agents an empty land hex gathers to take land corruption"),
    ParamSpec::list("corrupt_cut", 0, 5, "difficulty corrupt_cut_at+: how much lower each corrupt number is (city, station, factory, land)"),
    ParamSpec::int("corrupt_cut_at", 0, 5, "the difficulty level from which corrupt_cut applies"),
    ParamSpec::int("blight_discord", 0, 5, "Discord each land-corruption token sways each round"),
    ParamSpec::list("counter_pct", 50, 300, "every resource counter item of a tier 1, 2 threat costs this percent of its printed amount (rounded up)"),
    ParamSpec::int("counter_credit_pct", 50, 300, "every credits counter item costs this percent of its amount after counter_pct (rounded up; 100 = unchanged; 142's dial)"),
    ParamSpec::int("counter_bonus", 0, 10, "difficulty counter_bonus_at+: added to every resource item of each threat's first counter option (142; its credits before)"),
    ParamSpec::int("counter_bonus_all", 0, 10, "difficulty counter_bonus_at+: added to every resource counter item of every threat"),
    ParamSpec::int("counter_bonus_credits", 0, 20, "difficulty counter_bonus_at+: added to every credits counter item, in place of counter_bonus_all (142)"),
    ParamSpec::int("counter_more_all", 0, 10, "difficulty counter_more_at+: added again to every resource counter item but credits (142's second step)"),
    ParamSpec::int("counter_more_credits", 0, 20, "difficulty counter_more_at+: added again to every credits counter item (142)"),
    ParamSpec::int("counter_more_at", 0, 5, "the difficulty level from which counter_more_all and counter_more_credits apply (142)"),
    ParamSpec::int("counter_bonus_at", 0, 5, "the difficulty level from which counter_bonus and counter_bonus_all apply (142; 1 before)"),
    ParamSpec::int("extra_threats", 0, 5, "difficulty extra_threats_at+: extra threats dealt each round in all, one per country in turn"),
    ParamSpec::int("extra_threats_at", 0, 5, "the difficulty level from which extra_threats are dealt (139)"),
    ParamSpec::int("stage1_removed", 0, 3, "difficulty stage_swap_at+: stage-I cards removed"),
    ParamSpec::int("stage3_added", 0, 3, "difficulty stage_swap_at+: stage-III cards added (the cap holds, its end gets harder)"),
    ParamSpec::int("stage_swap_at", 0, 5, "the difficulty level from which stage1_removed and stage3_added apply (142; 2 before)"),
    ParamSpec::int("stage2_removed", 0, 3, "difficulty 3+: stage-II cards removed (a shorter round cap)"),
    ParamSpec::int("stage1_added", 0, 3, "151: difficulty 3+: stage-I cards added back (with stage2_removed, a smaller d3 step)"),
    ParamSpec::int("extra_corrupted", 0, 3, "difficulty 4: more cities per country start corrupted"),
    ParamSpec::list("compute_income", 0, 30, "151: the adversary's base compute income a round, at difficulty 0-4 (fixed, not per board)"),
    ParamSpec::list("ambassador_income", 0, 10, "151: the agent income a board a round (into each board's pool), at difficulty 0-4"),
    ParamSpec::int("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
    ParamSpec::int("discord_removed", 0, 3, "difficulty 4+: Discord cards removed at setup (from tier 1)"),
    ParamSpec::int("wall_troops", 0, 5, "build: adversary troops landing in each country where a seat with Wall has a node (pressure for Wall to stop)"),
    ParamSpec::int("station_flip_at", 0, 5, "agents on a player's research station before an effect can flip it (short of that, one lands there instead)"),
    ParamSpec::int("botnet_compute", 0, 5, "adversary compute each botnet makes a round (none with a player ambassador on it)"),
    ParamSpec::int("board_difficulty_at", 0, 5, "the difficulty level from which the adversary board's own difficulty line applies (135)"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto("build", "the event card after its threats: Rally X (each board's pool lands), Deploy (its troops), its other lines; the board's rules; ongoing projects act; botnets make adversary compute onto the projects"),
    PhaseSpec::auto("corrupt", "the event card's Corrupt X lines: every X that can gather its number of agents (on it and next to it) pulls them in and corrupts"),
    PhaseSpec::auto("natural", "natural corruption: every city, research station or factory already holding its number corrupts, with no move"),
    PhaseSpec::auto("reveal", "reveal the next event card face up, a round ahead; then income: base compute onto the projects, ambassador income into each board's pool; then every firewall comes off"),
    PhaseSpec::auto("event", "the face-up event card becomes this round's (none left: the adversary wins)"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "adversary",
    summary: "The adversary deck (the clock), ambassadors (creation lines, Corrupt X), botnets, the supply, difficulty",
    requires: &["board", "alignment"],
    variants: &[
        VariantDef {
            name: "militarist",
            summary: "The Militarist: bounded creation lines and Corrupt X; troops where the Protector stands",
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
/// Agents in the supply.
pub const SUPPLY: &str = "adversary.supply";
/// Stage of this round's card.
pub const STAGE: &str = "adversary.stage";
/// How many more deck cards beyond the next are shown (Recon Sweep).
pub const PEEK: &str = "adversary.peek";
/// Adversary compute made so far (all of it goes onto projects).
pub const COMPUTE: &str = "adversary.compute";
/// The project deck (face down), the visible row (in slot order), ongoing
/// projects in play, and resolved or discarded ones. The draw rule's misses
/// go to the project discard, which reshuffles into the deck (135).
pub const PROJECT_DECK: &str = "adversary.project_deck";
pub const PROJECT_DISCARD: &str = "adversary.project_discard";
pub const PROJECTS: &str = "adversary.projects";
pub const PROJECTS_ONGOING: &str = "adversary.projects_ongoing";
pub const PROJECTS_DONE: &str = "adversary.projects_done";
/// 151: the adversary's compute income a round (base, raised by "increase income" lines).
pub const INCOME_COMPUTE: &str = "adversary.income_compute";
/// 151: its ambassador income a board a round.
pub const INCOME_AMBASSADORS: &str = "adversary.income_ambassadors";

/// 151: board `c`'s ambassador pool, waiting for the next Rally.
pub fn pool_key(c: usize) -> String {
    format!("adversary.pool.{c}")
}

/// 151: the income-list entry for this difficulty (the last entry past the list's end).
fn income_at(w: &World, param: &str) -> i64 {
    let l = w.list(param);
    let d = w.int("adversary.difficulty").max(0) as usize;
    l.get(d).or(l.last()).copied().unwrap_or(0)
}

/// The per-round counter of adversary operations (table time).
pub const OPS: &str = "adversary.ops";
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
    let prompt = || format!("Seat {seat}: remove compute from which adversary project?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
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

/// The tier an extra threat comes from (an old-form `DealThreats`): this
/// round's card's stage, at most 3 (at difficulty 4, tier 2 `tier2_early`
/// cards sooner). Event cards' pulls name their own tiers (135).
pub fn threat_tier(w: &World, g: &GameState) -> u8 {
    // 146: no tier 3; stages II and III deal tier 2.
    if g.var(STAGE) >= 2 {
        return 2;
    }
    if level(w, 4) && tier2_soon(w, g, 0) {
        return 2;
    }
    1
}

/// Difficulty 4's look-ahead: a stage-II card within `tier2_early` cards of
/// the face-up pile and the deck, past its first `skip` (145: in the Sway
/// phase this round's card is still face up, so the deal's window starts one later).
pub fn tier2_soon(w: &World, g: &GameState, skip: usize) -> bool {
    let early = w.int("adversary.tier2_early") as usize;
    g.pile(NEXT).iter().chain(g.pile(DECK)).skip(skip).take(early).any(|id| card(w, id).stage >= 2)
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

/// 148 (D59): the twists of next round's card, face up in `NEXT` once revealed
/// (none before the reveal).
pub fn next_twists<'w>(w: &World<'w>, g: &GameState) -> Vec<&'w Op> {
    g.pile(NEXT).iter().take(1).flat_map(|id| card(w, id).twists.iter()).collect()
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
    let dealt = [
        (stage_cards[0] + if difficulty >= 3 { b.int("stage1_added")? } else { 0 } - if swap { b.int("stage1_removed")? } else { 0 }).max(0),
        (stage_cards[1] - if difficulty >= 3 { b.int("stage2_removed")? } else { 0 }).max(0),
        stage_cards[2] + if swap { b.int("stage3_added")? } else { 0 },
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
    Ok(Box::new(Adversary { dealt }))
}

/// The face-up event card becomes this round's (135: the `event` phase; in a
/// round without one, the build does it). None left: the adversary wins.
fn flip(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for id in g.pile(CURRENT).to_vec() {
        cx.move_card(g, &id, CURRENT, DISCARD, true)
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

/// The board's difficulty line's extra standing troops (135), from
/// `adversary.board_difficulty_at`.
fn standing_troops_bonus(w: &World) -> i64 {
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

/// The event card's standing lines (135): compute, ambassadors and troops,
/// resolved as the team's; a line landing troops gets the difficulty bonus.
fn standing(g: &mut GameState, cx: &mut Cx, c: &AdversaryCard) -> Res {
    let w = cx.world;
    let bonus = standing_troops_bonus(&w);
    for e in &c.standing {
        let op = match e {
            Op::Troops { count, each_country: true, destroy, at, stand, location, if_ } if bonus > 0 => Op::Troops {
                count: count + bonus,
                each_country: true,
                destroy: *destroy,
                at: *at,
                stand: *stand,
                location: *location,
                if_: *if_,
            },
            other => other.clone(),
        };
        effects::run(g, cx, std::slice::from_ref(&op), &mut effects::Ctx::team().named("standing").why(&c.name))?;
        if g.is_over() {
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
        // 151: the income, from the config's level and the board's own lines; round 1's pools.
        let (mut compute, mut ambassadors) =
            (income_at(&w, "adversary.compute_income"), income_at(&w, "adversary.ambassador_income"));
        for e in w.content.adversary_board.rules.iter().flat_map(|r| r.effects.iter()) {
            if let Op::IncreaseIncome { compute: c, ambassadors: a } = e {
                compute += c;
                ambassadors += a;
            }
        }
        cx.set_key(g, INCOME_COMPUTE, compute);
        cx.set_key(g, INCOME_AMBASSADORS, ambassadors);
        fill_pools(g, cx);
        // The project deck (135): one shuffled deck, dealt into the board's slots.
        let mut ids: Vec<Key> = w.content.projects.iter().map(|c| Key::new(&c.id)).collect();
        shuffle(&mut ids, cx.rng(g, "adversary"));
        cx.set_pile(g, PROJECT_DECK, ids);
        deal_projects(g, cx);
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        match phase {
            4 => flip(g, cx),
            0 => {
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
                standing(g, cx, c)?;
                if g.is_over() {
                    return Ok(());
                }
                for t in &c.twists {
                    match t {
                        // These resolve as they do anywhere (034, 044).
                        Op::Sway { .. } | Op::EachDestroyNode { .. } | Op::PullUnity { .. } => {
                            effects::run(g, cx, std::slice::from_ref(t), &mut effects::Ctx::team().named("twist").why(&c.name))?
                        }
                        // Read where they act: the corrupt step (138) and the march.
                        Op::Corrupt { .. } | Op::TroopSteps { .. } => {}
                        // The load check lets no other op in: one added to `Slot::Twist`
                        // needs a reader (044: `_ => {}` swallowed the Unity pull).
                        other => unreachable!("twist with no reader: {other:?}"),
                    }
                }
                wall_pressure(g, cx)?;
                // Ongoing projects act, in the order they came into play.
                for id in ongoing_acting(g) {
                    let p = project(&w, &id);
                    let acts: Vec<Op> = p.effects.iter().filter(|e| !standing_project_op(e)).cloned().collect();
                    effects::run(g, cx, &acts, &mut effects::Ctx::team().named("project").why(&p.name))?;
                    if g.is_over() {
                        return Ok(());
                    }
                }
                // Botnets make compute, except under a player ambassador.
                let per = w.int("adversary.botnet_compute");
                let (mut made, mut blocked) = (0, 0);
                for h in 0..g.hexes.len() {
                    let n = g.tokens(h, BOTNET) * per;
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
                cx.tally(g, "adversary.compute_botnets", made);
                gain_compute(g, cx, made)?;
            }
            1 => {
                // 138: the card's Corrupt X lines, in card order (they replace the spread).
                for t in twists(&w, g) {
                    if let Op::Corrupt { site, cut } = t {
                        corrupt_sites(g, cx, *site, *cut, None);
                        if g.is_over() {
                            return Ok(());
                        }
                    }
                }
            }
            2 => {
                natural_corruption(g, cx);
                // Botnets at their peak, per board (country).
                for c in 0..w.map.countries.len() {
                    let n = botnet_count(g, Some(&w.map.countries[c].hexes));
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
            3 => {
                reveal(g, cx);
                income(g, cx)?;
                // 158: firewalls come off at the end of the adversary turn after they went on.
                for h in 0..g.hexes.len() {
                    let n = g.tokens(h, FIREWALL);
                    if n > 0 {
                        cx.add_tokens(g, h, FIREWALL, -n);
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
                "adversary compute made so far: each botnet makes {} a round (none under a player ambassador); it fills the cheapest visible project",
                w.int("adversary.botnet_compute")
            ),
        );
        out.global(
            "income_compute",
            "adv. compute income",
            g.var(INCOME_COMPUTE),
            None,
            "adversary compute income a round, at the end of its turn, onto the projects",
        );
        out.global(
            "income_ambassadors",
            "adv. ambassador income",
            g.var(INCOME_AMBASSADORS),
            None,
            "agents a board a round, into each board's pool; the next Rally lands the pool",
        );
        let pool: i64 = (0..w.map.countries.len()).map(|c| g.var(&pool_key(c))).sum();
        out.global(
            "pool",
            "adv. ambassador pool",
            pool,
            None,
            "agents waiting to land at the next Rally, all boards",
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
            let kind: Vec<&str> = p.tags.iter().map(|t| t.name()).collect();
            let kind = kind.join(" + ");
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
        out.bookkeeping(&[
            PEEK,
            FIRST_CORRUPTION,
            FIRST_PROJECT,
            "adversary.project.",
            "adversary.ongoing_from.",
            "adversary.cut.",
            "adversary.pool.",
        ]);
    }
}

/// Shows the next `n` deck cards beyond the face-up one (Recon Sweep); a
/// longer peek already running is kept.
pub fn peek(g: &mut GameState, cx: &mut Cx, n: i64) {
    let cur = g.var(PEEK);
    cx.set_key(g, PEEK, cur.max(n));
}

/// Draws the top of the project deck; an empty deck reshuffles the discard
/// first (counted in `reshuffles`). None: both are empty.
fn draw_project(g: &mut GameState, cx: &mut Cx, reshuffles: &mut u32) -> Option<Key> {
    if g.pile(PROJECT_DECK).is_empty() {
        let mut ids = g.pile(PROJECT_DISCARD).to_vec();
        if ids.is_empty() {
            return None;
        }
        cx.set_pile(g, PROJECT_DISCARD, Vec::new());
        shuffle(&mut ids, cx.adversary_rng(g));
        cx.set_pile(g, PROJECT_DECK, ids);
        *reshuffles += 1;
        cx.tally(g, "adversary.project_reshuffles", 1);
    }
    g.pile(PROJECT_DECK).first().cloned()
}

/// Fills the board's project slots by the draw rule (135): draw; the card goes
/// into the first empty slot it fits (a slot's tag, or any; Universal fits
/// every slot), else to the discard; the discard reshuffles when the deck runs
/// out; a slot still unfilled after one reshuffle takes the next card anyway
/// (tallied: a content bug). The row is the slots in order.
fn deal_projects(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let slots = &w.content.adversary_board.project_slots;
    let mut filled: Vec<Option<Key>> = vec![None; slots.len()];
    let mut reshuffles = 0u32;
    while filled.iter().any(Option::is_none) {
        let Some(id) = draw_project(g, cx, &mut reshuffles) else {
            break;
        };
        // 151 (amendment 6618): what share of the deck a game turns up.
        cx.tally(g, "adversary.projects_revealed", 1);
        cx.tally(g, &KeyBuf::new("adversary.revealed.").text(&id), 1);
        let tags = &project(&w, &id).tags;
        let fits = |k: usize| {
            filled[k].is_none()
                && slots[k].tag.is_none_or(|t| ThreatTag::fits(tags, &[PullTag::Is(t)]))
        };
        let at = (0..slots.len()).find(|&k| fits(k));
        let at = match at {
            Some(k) => Some(k),
            None if reshuffles >= 2 => {
                cx.tally(g, "adversary.project_pull_forced", 1);
                filled.iter().position(Option::is_none)
            }
            None => None,
        };
        let to = if at.is_some() { PROJECTS } else { PROJECT_DISCARD };
        cx.move_card(g, &id, PROJECT_DECK, to, false)
            .expect("top of the project deck");
        if let Some(k) = at {
            filled[k] = Some(id);
        }
    }
    // The row in slot order.
    let row: Vec<Key> = filled.into_iter().flatten().collect();
    cx.set_pile(g, PROJECTS, row);
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

fn ongoing_from_key(id: &str) -> KeyBuf {
    KeyBuf::new("adversary.ongoing_from.").text(id)
}

/// The ongoing projects acting this round, in the order they came into play:
/// one that resolved this round acts from the next (145).
pub fn ongoing_acting(g: &GameState) -> Vec<Key> {
    g.pile(PROJECTS_ONGOING).iter().filter(|id| g.var(&ongoing_from_key(id.as_str())) <= g.round as i64).copied().collect()
}

/// A full project resolves: once (to the resolved pile), or as an ongoing
/// project from next round's build on. An empty row deals a new one.
fn resolve_project(g: &mut GameState, cx: &mut Cx, id: &str) -> Res {
    let w = cx.world;
    let p = project(&w, id);
    cx.set_key(g, &project_key(id), 0);
    let to = if p.ongoing {
        // 145: it acts from next round on (it acted in the build that filled it).
        cx.set_key(g, &ongoing_from_key(id), g.round as i64 + 1);
        PROJECTS_ONGOING
    } else {
        PROJECTS_DONE
    };
    cx.move_card(g, id, PROJECTS, to, false)
        .expect("visible project");
    cx.tally(g, "adversary.projects_resolved", 1);
    cx.tally(g, &KeyBuf::new("adversary.project.").text(id), 1);
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
    let once = if p.ongoing { &p.resolve } else { &p.effects };
    effects::run(g, cx, once, &mut effects::Ctx::team().named("project").why(&p.name))?;
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

/// An ongoing project's op that is read where it acts, not resolved each
/// round (135): a front fighting twice, corp troops each production.
fn standing_project_op(e: &Op) -> bool {
    matches!(
        e,
        Op::ExtraWar { ongoing_use: true, .. }
            | Op::CorpTroops { when: crate::op::CorpWhen::EachProduction, .. }
    )
}

/// 151: each board's pool gains its ambassador income (the supply caps it at rally).
fn fill_pools(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let n = g.var(INCOME_AMBASSADORS).max(0);
    for c in 0..w.map.countries.len() {
        cx.add_key(g, &pool_key(c), n);
    }
}

/// 151, income, at the end of the adversary's turn: its compute income goes onto the projects, and each board's
/// pool gains its ambassador income, to land at the next Rally.
fn income(g: &mut GameState, cx: &mut Cx) -> Res {
    if g.is_over() {
        return Ok(());
    }
    fill_pools(g, cx);
    let n = g.var(INCOME_COMPUTE);
    cx.tally(g, "adversary.compute_income", n.max(0));
    gain_compute(g, cx, n)
}

/// 151: "increase income": for the rest of the game.
pub fn increase_income(g: &mut GameState, cx: &mut Cx, compute: i64, ambassadors: i64) {
    cx.add_key(g, INCOME_COMPUTE, compute);
    cx.add_key(g, INCOME_AMBASSADORS, ambassadors);
    cx.tally(g, "adversary.income_raised_compute", compute);
    cx.tally(g, "adversary.income_raised_ambassadors", ambassadors);
}

/// The site type the card ahead rallies to (its Rally line).
pub fn rally_site(c: &AdversaryCard) -> Option<crate::op::RallyAt> {
    c.standing.iter().find_map(|e| if let Op::Rally { site, count: 0 } = e { Some(*site) } else { None })
}

/// 151, Rally X in board `scope` or every board: each board places its pool (or `count`, a project's rally)
/// by [`super::plan::rally_plan`]: every eligible site gets the even share, and players choose which get one
/// more (the remainder, one each). The supply is a cap. Tallied as `adversary.rallied` and by source.
pub fn rally(g: &mut GameState, cx: &mut Cx, site: crate::op::RallyAt, count: i64, scope: Option<usize>, source: &str) -> Res {
    let w = cx.world;
    let boards: Vec<usize> = match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    };
    for c in boards {
        let n = if count > 0 {
            count
        } else {
            let p = g.var(&pool_key(c));
            cx.set_key(g, &pool_key(c), 0);
            p
        };
        if n <= 0 || board::flagged(g, crate::content::RoundFlag::Cordon, c) {
            continue;
        }
        let Some(plan) = super::plan::rally_plan(&w, g, site, c, n) else {
            cx.tally(g, "adversary.rally_nowhere", n);
            continue;
        };
        let before = g.var(SUPPLY);
        for &h in &plan.sites {
            place(g, cx, h, plan.each);
        }
        let mut among = plan.sites.clone();
        for _ in 0..plan.extra {
            let opts = among.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = || format!("Rally in {}: which site gains one more agent?", country_name(&w, c));
            let Some(i) = cx.choose(0, prompt, opts)? else {
                break;
            };
            place(g, cx, among.remove(i), 1);
        }
        let made = before - g.var(SUPPLY);
        cx.tally(g, "adversary.rallied", made);
        cx.tally(g, &format!("adversary.rallied.{source}"), made);
    }
    Ok(())
}

/// "The adversary builds now" (an out-of-turn build: Grand Offensive, the Offensive Discord card), in board
/// `scope` or every board. 151: each board in scope rallies its ambassador income now, to this round's card's
/// Rally site (none: nothing).
pub fn build_all(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    let Some(id) = g.pile(round_pile(g)).first().cloned() else {
        return Ok(());
    };
    let Some(site) = rally_site(card(&w, &id)) else {
        return Ok(());
    };
    let n = g.var(INCOME_AMBASSADORS);
    if n <= 0 {
        return Ok(());
    }
    rally(g, cx, site, n, scope, "build")
}

/// 138, a creation line: `count` agents on each of the
/// `sites` lowest-numbered matching sites in each country in scope (one
/// source with the agents' forecast, [`super::plan::create_plan`]). A
/// cordoned country gains none. Tallied by `source` (event, board, threat, project,
/// discord, unity).
pub fn create(g: &mut GameState, cx: &mut Cx, line: super::plan::CreateLine, scope: Option<usize>, near: &[usize], location: Option<usize>, source: &str) {
    let w = cx.world;
    let mut made = 0;
    for (h, n) in super::plan::create_plan(&w, g, line, scope, near, location) {
        let before = g.var(SUPPLY);
        place(g, cx, h, n);
        made += before - g.var(SUPPLY);
    }
    cx.tally(g, "adversary.created", made);
    cx.tally(g, &format!("adversary.created.{source}"), made);
}

/// Troop pressure where the Protector stands (080, candidate): in the round's
/// build, `wall_troops` adversary troops land in each country where a seat with
/// Wall has a node, as any landing does (on a factory, else a city, never on
/// a walled hex), so Wall has strikes to stop.
fn wall_pressure(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let n = w.int("adversary.wall_troops");
    if n == 0 {
        return Ok(());
    }
    let walls: Vec<usize> = (0..g.seats.len()).filter(|&s| board::has_wall(&w, s)).collect();
    for c in 0..w.map.countries.len() {
        if !walls.iter().any(|&s| has_node_in(&w, g, s, c)) {
            continue;
        }
        let hexes = w.map.countries[c].hexes.clone();
        land_troops(g, cx, &hexes, n, "Troops land where the Protector stands: on which hex?", "adversary.wall_troops", None, true)?;
    }
    Ok(())
}

/// Places `n` agents from the supply on hex `h`. The supply
/// is a cap (027): a placement it can't make doesn't happen.
pub fn place(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) {
    // 158: a firewall stops every arrival on its hex; they stay in the supply.
    if firewalled(g, h) {
        cx.tally(g, "adversary.firewalled", n.max(0));
        return;
    }
    for k in 0..n {
        if g.var(SUPPLY) <= 0 {
            cx.tally(g, "adversary.supply_short", n - k);
            return;
        }
        cx.add_key(g, SUPPLY, -1);
        cx.per_round(g, OPS, 1);
        arrive(g, cx, h);
    }
}

/// One agent arrives on hex `h` from the supply: it cancels a
/// player ambassador there (both are removed, no Unity). 138: with the spread
/// gone, no botnet forms by arriving (botnets come only from printed lines).
fn arrive(g: &mut GameState, cx: &mut Cx, h: usize) {
    if let Some(s) = player_ambassador(g, h) {
        cx.add_tokens(g, h, ambassador(s), -1);
        cx.add_key(g, SUPPLY, 1);
        cx.tally(g, "adversary.canceled", 1);
        return;
    }
    cx.add_tokens(g, h, ADV_AMBASSADOR, 1);
}

/// Removes the botnet on hex `h` (a card's effect; no standard action does, 019).
pub fn remove_botnet(g: &mut GameState, cx: &mut Cx, h: usize) {
    if g.tokens(h, BOTNET) > 0 {
        cx.add_tokens(g, h, BOTNET, -1);
        // A botnet formed here later is a new one: the hack cut this one (028).
        cx.set_key(g, &cut_key(h), 0);
        cx.tally(g, "adversary.botnets_removed", 1);
        alignment::for_botnet_removed(g, cx);
    }
}

/// Botnets on `hexes`, or on the whole board for `None`.
pub fn botnet_count(g: &GameState, hexes: Option<&[usize]>) -> i64 {
    match hexes {
        Some(hexes) => hexes.iter().map(|&h| g.tokens(h, BOTNET)).sum(),
        None => (0..g.hexes.len()).map(|h| g.tokens(h, BOTNET)).sum(),
    }
}

/// Removes up to `n` agents from hex `h` to the supply.
pub fn remove(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> i64 {
    let k = n.min(g.tokens(h, ADV_AMBASSADOR)).max(0);
    if k > 0 {
        cx.add_tokens(g, h, ADV_AMBASSADOR, -k);
        cx.add_key(g, SUPPLY, k);
    }
    k
}

/// Removes every agent in country `c` to the supply (one
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
/// comes off), its agents return to the supply, and a node on
/// it is destroyed (138: only on the hex itself; an allied ambassador there is
/// armor).
pub fn corrupt_city(g: &mut GameState, cx: &mut Cx, h: usize) {
    corrupt_hex(g, cx, h, CorruptSite::City);
}

/// 138: hex `h` corrupts as a `site`: a city, research station or factory
/// takes the adversary's cube (a player's comes off); an empty land hex takes
/// land corruption. Its agents return to the supply; a node on
/// the hex is destroyed (its owner's ambassador there is armor, used up
/// instead; the Sanctuary capstone keeps it); player ambassadors still there
/// are lost (024: they would free it in the same step); a marker comes off.
pub fn corrupt_hex(g: &mut GameState, cx: &mut Cx, h: usize, site: CorruptSite) {
    let w = cx.world;
    if site == CorruptSite::Land {
        cx.add_tokens(g, h, LAND_CORRUPTION, 1);
    } else {
        take_for_adversary(g, cx, h);
    }
    let n = g.tokens(h, ADV_AMBASSADOR);
    remove(g, cx, h, n);
    if let Some(s) = node_owner(g, h) {
        if !cards::has_passive(&w, g, s, |p| matches!(p, Op::CorruptionShield)) {
            board::destroy_piece(g, cx, h, node(s), board::Cause::Effect("corruption"));
        }
    }
    for s in 0..g.seats.len() {
        let k = ambassador(s);
        let mine = g.tokens(h, k);
        cx.add_tokens(g, h, k, -mine);
    }
    clear_markers_on(g, cx, h);
    cx.tally(g, "adversary.corruptions", 1);
    cx.tally(g, &format!("adversary.corruptions.{}", site.name()), 1);
    if g.round > 0 && g.var(FIRST_CORRUPTION) == 0 {
        cx.set_key(g, FIRST_CORRUPTION, g.round as i64);
        cx.tally(g, "adversary.first_corruption_round", g.round as i64);
    }
    cx.note(Some(h), || match site {
        CorruptSite::Land => format!("{} takes land corruption", hex_name(&w, h)),
        _ => format!("{} is corrupted", hex_name(&w, h)),
    });
}

/// 138: the corrupt number of `site` this game: its config number, less the
/// difficulty's `corrupt_cut` from `corrupt_cut_at`, less a line's `cut`; at
/// least 1.
pub fn corrupt_n(w: &World, site: CorruptSite, cut: i64) -> i64 {
    let base = match site {
        CorruptSite::City => w.int("adversary.corrupt_at"),
        CorruptSite::Station => w.int("adversary.corrupt_station_at"),
        CorruptSite::Factory => w.int("adversary.corrupt_factory_at"),
        CorruptSite::Land => w.int("adversary.corrupt_land_at"),
    };
    let ladder = if level(w, w.int("adversary.corrupt_cut_at")) {
        let i = CorruptSite::ALL.iter().position(|&s| s == site).unwrap_or(0);
        w.list("adversary.corrupt_cut").get(i).copied().unwrap_or(0)
    } else {
        0
    };
    (base - ladder - cut).max(1)
}

/// 138, a Corrupt X line in country `scope` or everywhere: what
/// [`super::plan::corrupt_plan`] says, pulled in and corrupted.
pub fn corrupt_sites(g: &mut GameState, cx: &mut Cx, site: CorruptSite, cut: i64, scope: Option<usize>) {
    let w = cx.world;
    let n = corrupt_n(&w, site, cut);
    for (t, pulls) in super::plan::corrupt_plan(&w, g, site, n, scope) {
        for (h, k) in pulls {
            remove(g, cx, h, k);
            cx.tally(g, "adversary.pulled", k);
        }
        corrupt_hex(g, cx, t, site);
        if g.is_over() {
            return;
        }
    }
}

/// 138, natural corruption at the end of the event card: every city,
/// research station or factory already holding its corrupt number (a city's
/// markers count, 034) corrupts, with no move, in hex order.
fn natural_corruption(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for site in [CorruptSite::City, CorruptSite::Station, CorruptSite::Factory] {
        let n = corrupt_n(&w, site, 0);
        for h in super::plan::natural_targets(&w, g, site, n) {
            corrupt_hex(g, cx, h, site);
            cx.tally(g, "adversary.corruptions_natural", 1);
            if g.is_over() {
                return;
            }
        }
    }
}

/// `count` adversary troops, each onto a factory among `hexes` (players
/// choose; 151, `lowest`: the lowest-numbered, deterministic targeting for events, projects and the board), or
/// with no factory onto a city. With `destroy`, each destroys the node on the hex it lands on (027), named for
/// its tally.
#[allow(clippy::too_many_arguments)]
pub fn land_troops(
    g: &mut GameState,
    cx: &mut Cx,
    hexes: &[usize],
    count: i64,
    prompt: &str,
    tally: &str,
    destroy: Option<&'static str>,
    lowest: bool,
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
        let i = if lowest {
            let Some(i) = (0..spots.len()).min_by_key(|&i| w.map.hexes[spots[i]].number) else {
                break;
            };
            i
        } else {
            let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
            let Some(i) = cx.choose(0, prompt, opts)? else {
                break;
            };
            i
        };
        place_troop(g, cx, spots[i]);
        cx.per_round(g, OPS, 1);
        cx.tally(g, tally, 1);
        if let Some(source) = destroy {
            for s in 0..g.seats.len() {
                if g.tokens(spots[i], node(s)) > 0 {
                    board::destroy_piece(g, cx, spots[i], node(s), board::Cause::Effect(source));
                }
            }
        }
    }
    Ok(())
}

/// Research station `h`, not yet corrupted, flips to the adversary. r35: a
/// player's station is defended: it flips only with `station_flip_at`
/// agents on it and no player ambassador, and those are used
/// up; short of that one agent lands on it instead (a player
/// ambassador standing there cancels it).
/// Returns whether it flipped (099: a player's station that holds is no new source).
pub fn corrupt_station(g: &mut GameState, cx: &mut Cx, h: usize) -> bool {
    let w = cx.world;
    if cube_owner(g, h).is_some() {
        let need = w.int("adversary.station_flip_at");
        if g.tokens(h, ADV_AMBASSADOR) < need || player_ambassador(g, h).is_some() {
            // 099: say what happened: a player's ambassador there cancels the one
            // arriving, and an empty supply sends none (both read "lands on it").
            let (adv, supply) = (g.tokens(h, ADV_AMBASSADOR), g.var(SUPPLY));
            place(g, cx, h, 1);
            cx.tally(g, "adversary.station_flip_held", 1);
            let what = if supply <= 0 {
                "no agent is left to send"
            } else if g.tokens(h, ADV_AMBASSADOR) > adv {
                "an agent lands on it"
            } else {
                "the agent sent and a player ambassador there cancel each other"
            };
            cx.note(Some(h), || format!("The station at {} holds: {what}", hex_name(&w, h)));
            return false;
        }
        remove(g, cx, h, need);
    }
    // 146 (D60): a flip is corruption: its cleanup as everywhere (the node on it or its armor, the adversary
    // ambassadors back to the supply, player ambassadors there lost, markers off).
    corrupt_hex(g, cx, h, CorruptSite::Station);
    cx.tally(g, "adversary.stations_flipped", 1);
    cx.note(Some(h), || {
        format!("The station at {} flips to the adversary", hex_name(&w, h))
    });
    true
}

/// Factory `h` becomes the adversary's (070: a threat's target): it corrupts,
/// with corruption's cleanup (146, D60: the node on it or its armor, the
/// ambassadors; any player cube comes off, its cube goes on, markers off).
pub fn corrupt_factory(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    corrupt_hex(g, cx, h, CorruptSite::Factory);
    cx.tally(g, "adversary.factories_seized", 1);
    cx.note(Some(h), || format!("The factory at {} goes to the adversary", hex_name(&w, h)));
}

/// Whether a removal of `ambassadors` or `troops` has a piece to take on hex
/// `h` (F64: a card that names no troop can't touch one).
pub fn can_damage(g: &GameState, h: usize, ambassadors: i64, troops: i64) -> bool {
    (ambassadors > 0 && g.tokens(h, ADV_AMBASSADOR) > 0)
        || (troops > 0 && g.tokens(h, TROOP) > 0)
}

/// Whether a "remove everything on the hex" has a piece to take on `h`.
pub fn can_destroy_all(g: &GameState, h: usize) -> bool {
    can_damage(g, h, 1, 1)
}

/// In hex `h`, removes up to `ambassadors` agents or up to
/// `troops` troops: one kind, seat `seat`'s choice where the hex holds both.
pub fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
    let kinds: Vec<(&str, i64)> = [(ADV_AMBASSADOR, ambassadors), (TROOP, troops)]
        .into_iter()
        .filter(|(k, n)| *n > 0 && g.tokens(h, k) > 0)
        .collect();
    let chosen = match kinds.as_slice() {
        [] => return Ok(()),
        [only] => *only,
        _ => {
            let opts = kinds
                .iter()
                .map(|(k, n)| opt(format!("remove {} {} at {}", (*n).min(g.tokens(h, k)), piece_name(k), hex_name(&w, h))))
                .collect();
            let prompt = || format!("Seat {seat}: remove which pieces?");
            let Some(i) = cx.choose(seat, prompt, opts)? else {
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

/// Removes every agent and troop in hex `h`.
pub fn destroy_all(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let n = g.tokens(h, ADV_AMBASSADOR);
    for _ in 0..n {
        remove_piece(g, cx, Some(seat), h, ADV_AMBASSADOR);
    }
    for _ in 0..g.tokens(h, TROOP) {
        remove_piece(g, cx, Some(seat), h, TROOP);
    }
}

/// Removes one agent in country `c` (players choose the hex).
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
        &format!("Unity: remove an agent in {} from which hex?", country_name(&w, c)),
        opts,
    )? {
        remove_piece(g, cx, None, hexes[i], ADV_AMBASSADOR);
    }
    Ok(())
}

/// How much the purge still asks (136): each step of `alignment.purge_at`
/// reached drops one kind of adversary piece. Corrupted cities always count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PurgeLevel {
    /// Every adversary piece: corrupted cities, botnets, troops, ambassadors.
    All,
    /// Ambassadors no longer count.
    NoAmbassadors,
    /// Ambassadors and troops no longer count.
    NoTroops,
    /// Only corrupted cities count.
    CitiesOnly,
}

impl PurgeLevel {
    /// The level after `steps` thresholds reached (three or more: cities only).
    pub fn from_steps(steps: i64) -> PurgeLevel {
        match steps {
            ..=0 => PurgeLevel::All,
            1 => PurgeLevel::NoAmbassadors,
            2 => PurgeLevel::NoTroops,
            _ => PurgeLevel::CitiesOnly,
        }
    }

    /// The steps reached, 0-3 (the view's bar, the agents' eval term).
    pub fn steps(self) -> i64 {
        self as i64
    }

    pub fn counts_ambassadors(self) -> bool {
        self < PurgeLevel::NoAmbassadors
    }

    pub fn counts_troops(self) -> bool {
        self < PurgeLevel::NoTroops
    }

    pub fn counts_botnets(self) -> bool {
        self < PurgeLevel::CitiesOnly
    }

    /// The win line for this level.
    pub fn win_text(self) -> &'static str {
        match self {
            PurgeLevel::All => "purge: no corrupted city, botnet, adversary troop or agent on the board",
            PurgeLevel::NoAmbassadors => {
                "purge: no corrupted city, botnet or adversary troop (ambassadors no longer count)"
            }
            PurgeLevel::NoTroops => "purge: no corrupted city or botnet (ambassadors and troops no longer count)",
            PurgeLevel::CitiesOnly => "purge: no corrupted city (ambassadors, troops and botnets no longer count)",
        }
    }
}

/// What still counts against the purge at `level` (019: botnets until the
/// last step).
pub fn purge_count(w: &World, g: &GameState, level: PurgeLevel) -> i64 {
    (0..g.hexes.len()).map(|h| counts_against(w, g, h, level)).sum()
}

/// What hex `h` adds to [`purge_count`].
fn counts_against(w: &World, g: &GameState, h: usize, level: PurgeLevel) -> i64 {
    against_bar(corrupted_city(w, g, h), g.tokens(h, BOTNET), g.tokens(h, ADV_AMBASSADOR), g.tokens(h, TROOP), level)
}

/// What a hex with these pieces adds to [`purge_count`] at `level`.
pub fn against_bar(corrupted_city: bool, botnets: i64, ambassadors: i64, troops: i64, level: PurgeLevel) -> i64 {
    let mut n = corrupted_city as i64;
    if level.counts_botnets() {
        n += botnets;
    }
    if level.counts_ambassadors() {
        n += ambassadors;
    }
    if level.counts_troops() {
        n += troops;
    }
    n
}

/// Whether [`purge_count`] is 0, stopping at the first piece: every action asks.
pub fn under_bar(w: &World, g: &GameState, level: PurgeLevel) -> bool {
    !(0..g.hexes.len()).any(|h| counts_against(w, g, h, level) > 0)
}

/// A board loss, if any: a player with no nodes on the board (locked,
/// 027). (The deck running out ends the game in the build step, and the
/// Discord deck running out in the tip.)
pub fn loss(g: &GameState) -> Option<(&'static str, String)> {
    let gone = (0..g.seats.len()).find(|&s| !has_node(g, s))?;
    Some(("node", format!("seat {gone} has no nodes left on the board")))
}
