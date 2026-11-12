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
mod rally;

pub use bar::*;
pub use corrupt::*;
pub use pieces::*;
pub use projects::*;
pub use rally::*;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("difficulty", 0, 4, "difficulty level 0-4 (each level includes the ones below)"),
    ParamSpec::list("stage_cards", 0, 20, "adversary cards dealt from stage I, II and III (before difficulty)"),
    ParamSpec::int("supply_per_country", 0, 40, "agents in the supply per country"),
    ParamSpec::int("corrupt_at", 1, 20, "a city's corrupt number: the agents on it that corrupt it (coastal cities too)"),
    ParamSpec::int("corrupt_station_at", 1, 20, "a research station's corrupt number"),
    ParamSpec::int("corrupt_factory_at", 1, 20, "a factory's corrupt number"),
    ParamSpec::int("corrupt_land_at", 1, 20, "open land's corrupt number (only a Corrupt open land line; it takes land corruption)"),
    ParamSpec::int("corrupt_node_at", 1, 20, "175: a node's corrupt number: a hex with a player's node and this many agents loses the node"),
    ParamSpec::list("corrupt_cut", 0, 5, "difficulty corrupt_cut_at+: how much lower each corrupt number is (city, station, factory, land)"),
    ParamSpec::int("corrupt_cut_at", 0, 5, "the difficulty level from which corrupt_cut applies"),
    ParamSpec::int("blight_discord", 0, 5, "Discord each land-corruption token sways each round"),
    ParamSpec::int("blight_node_discord", 0, 5, "192: Discord each land-corruption token a corrupted node hex took (placed face down) sways each round, in place of blight_discord"),
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
    ParamSpec::list("ambassador_income", 0, 10, "151: the agent income a country a round (into each country's pool), at difficulty 0-4"),
    ParamSpec::int("tier2_early", 0, 3, "difficulty 4+: tier-2 threats start this many rounds sooner"),
    ParamSpec::int("discord_removed", 0, 3, "difficulty 4+: Discord cards removed at setup (from tier 1)"),
    ParamSpec::int("wall_troops", 0, 5, "build: adversary troops landing in each country where a seat with Wall has a node (pressure for Wall to stop)"),
    ParamSpec::int("wall_landing", 0, 1, "192: 1: the Protector's Muster skips players' factories (the lowest-numbered neutral or adversary factory, else city); 0: anyone's"),
    ParamSpec::int("war_node", 0, 1, "build (r83, the Militarist's War Takes Ground): 1: a drone strike that destroys a factory destroys a node on its hex too"),
    ParamSpec::int("station_flip_at", 0, 5, "agents on a player's research station before an effect can flip it (short of that, one lands there instead)"),
    ParamSpec::int("botnet_compute", 0, 5, "adversary compute each botnet makes a round (none with a player ambassador on it)"),
    ParamSpec::int("board_difficulty_at", 0, 5, "the difficulty level from which the adversary board's own difficulty line applies (135)"),
];

/// r80 (ruling #41): the adversary turn's four beats, printed as a strip on the adversary board, with the round
/// phases each runs (working names: a rename is this one edit). Threat resolution stays its own round step.
pub const BEATS: [(&str, &[&str]); 4] = [
    ("Card", &["adversary.event", "threats.deal", "adversary.build"]),
    ("Corrupt", &["adversary.corrupt", "adversary.natural"]),
    ("March", &["board.march"]),
    ("Income", &["adversary.reveal", "adversary.compute"]),
];

/// The beat a round phase belongs to (none: not part of the adversary turn).
pub fn beat_of(phase: &str) -> Option<&'static str> {
    BEATS.iter().find(|(_, ps)| ps.contains(&phase)).map(|(b, _)| *b)
}

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto("build", "the Card beat after the threats: Rally X (each country's pool lands), Muster (its troops), the card's other lines, the Protector's Muster"),
    PhaseSpec::auto("corrupt", "the event card's Corrupt X lines: every agent next to an X moves onto it, then every X holding its number corrupts"),
    PhaseSpec::auto("natural", "natural corruption: every city, research station or factory holding its number corrupts, then every node hex holding a node's"),
    PhaseSpec::auto("reveal", "reveal the next event card face up, a round ahead; then agent income into each country's pool"),
    PhaseSpec::auto("event", "the face-up event card becomes this round's (none left: the adversary wins)"),
    // 178 (amendment 8112): its own phase, so a project's prompt doesn't follow the reveal's hidden draw in one
    // phase (a determinized replay of it could resolve another project first). No play change.
    PhaseSpec::auto("compute", "the Income beat's compute (income, botnets, corrupted stations and the compute banked since the last turn onto the leftmost project, which resolves as it fills); ongoing projects act; then every firewall, cordon and prohibition token comes off: the adversary turn ends"),
];

// `run`'s phase indices: each is its phase's position in `PHASES`.
const BUILD: usize = 0;
const CORRUPT: usize = 1;
const NATURAL: usize = 2;
const REVEAL: usize = 3;
const EVENT: usize = 4;
const COMPUTE_STEP: usize = 5;

pub static DEF: ModuleDef = ModuleDef {
    id: "adversary",
    summary: "The adversary deck (the clock), agents (creation lines, Corrupt X), botnets, the supply, difficulty",
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
    let have = |s: usize| b.world.content.adversary.iter().filter(|c| c.stage as usize == s + 1).count() as i64;
    // 200 (r88): 196's `stage_extra` (two more event cards at d0) is gone: d0's band comes from the players' side.
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

/// The board's difficulty line's extra troops on every Muster (135; 175: an event's, a project's and the
/// Protector landing alike), from `adversary.board_difficulty_at`.
pub fn muster_bonus(w: &World) -> i64 {
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
/// resolved as the team's; a Muster gets the difficulty bonus where it lands (`team::troops`, 175).
fn standing(g: &mut GameState, cx: &mut Cx, c: &AdversaryCard) -> Res {
    for e in &c.standing {
        effects::run(g, cx, std::slice::from_ref(e), &mut effects::Ctx::team().named("standing").why(&c.name))?;
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
            }
            CORRUPT => {
                // 138: the card's Corrupt X lines, in card order (they replace the spread).
                for t in twists(&w, g) {
                    if let Op::Corrupt { site, cut } = t {
                        corrupt_sites(g, cx, *site, *cut, None)?;
                        if g.is_over() {
                            return Ok(());
                        }
                    }
                }
            }
            NATURAL => {
                natural_corruption(g, cx)?;
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
            REVEAL => {
                reveal(g, cx);
                if !g.is_over() {
                    fill_pools(g, cx);
                }
            }
            COMPUTE_STEP => {
                if !g.is_over() {
                    compute_step(g, cx)?;
                }
                // r80 (ruling #41): ongoing projects act in the Income beat, after compute, in the order they came
                // into play (they acted in the build, between Muster and Corrupt).
                if !g.is_over() {
                    ongoing_projects_act(g, cx)?;
                }
                // 158: firewalls come off at the end of the adversary turn after they went on; r80 (#53) so do
                // prohibition tokens.
                for h in 0..g.hexes.len() {
                    for kind in [FIREWALL, prohibit_token(Step::Rally), prohibit_token(Step::Muster), prohibit_token(Step::Corrupt)] {
                        let n = g.tokens(h, kind);
                        if n > 0 {
                            cx.add_tokens(g, h, kind, -n);
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
        out.global(
            "income_ambassadors",
            "agent income",
            g.var(INCOME_AMBASSADORS),
            None,
            "agents a country a round, into each country's pool; the next Rally lands the pool",
        );
        let pool: i64 = (0..w.map.countries.len()).map(|c| g.var(&pool_key(c))).sum();
        out.global(
            "pool",
            "agent pool",
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
            "Adversary projects (compute fills the leftmost)",
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
                Step::Rally => "Rally",
                Step::Muster => "Muster",
                Step::Corrupt => "Corrupt",
            };
            out.global(
                &format!("prohibit_{}", step.name()),
                &format!("no {name} (round)"),
                round,
                None,
                &format!("a Unity prohibition: in round {round}'s adversary turn, {name} skips each {}", wheres.join("; each ")),
            );
        }
        out.bookkeeping(&[
            "prohibit.",
            // 192: which land corruption is face down (the view doesn't draw the face yet).
            NODE_BLIGHT,
            "adversary.node_blight.",
            PEEK,
            FIRST_CORRUPTION,
            FIRST_PROJECT,
            "adversary.project.",
            "adversary.ongoing_from.",
            "adversary.cut.",
            "adversary.pool.",
            "adversary.slot.",
        ]);
    }
}

/// Ongoing projects act, in the order they came into play (r80: in the Income beat).
fn ongoing_projects_act(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for id in ongoing_acting(g) {
        let p = project(&w, &id);
        let acts: Vec<Op> = p.effects.iter().filter(|e| !standing_project_op(e)).cloned().collect();
        let before = g.hexes.clone();
        effects::run(g, cx, &acts, &mut effects::Ctx::team().named("project").why(&p.name))?;
        // 195: an ongoing project's rounds, and its moves (measures).
        cx.tally(g, &KeyBuf::new("adversary.project_acts.").text(&id), 1);
        projects::tally_moves(g, cx, &id, &before);
        if g.is_over() {
            break;
        }
    }
    Ok(())
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
fn prohibit_tally(g: &mut GameState, cx: &mut Cx, step: Step, wh: Where, n: i64) {
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
            (CORRUPT, "corrupt"),
            (NATURAL, "natural"),
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
