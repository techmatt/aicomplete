//! The experiment harness: configs × variants × difficulties × agents over a
//! seed range, played on every core, with the measures of
//! `docs/principles.md` gathered per game. `report/` aggregates and writes
//! them out. See docs/experiments.md for the spec format.
//!
//! A game's measures are read from the public state, the actions taken and
//! the engine's stats (`g.stats`: counters and per-round tallies): card offers
//! are read off the draft packets, picks, keeps, plays, payments and gifts off
//! the actions, margins off the final table. Probes that simulate (decision
//! closeness, win probability) use their own RNG and determinize first, like
//! the agents, and never touch the game being played.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, Once};
use std::time::Instant;

use aic_core::content::{Resource, Side};
use aic_core::modules::{adversary, alignment, board, cards, common};
use aic_core::{replay, Action, Config, GameState, Outcome, Rules, Stats};
use serde::{Deserialize, Serialize};

use crate::agents::determinize::determinize;
use crate::agents::eval::{Eval, Weights};
use crate::agents::{agent_rng, search, AgentSpec};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub name: String,
    /// What the experiment is for; printed at the top of the report.
    #[serde(default)]
    pub note: String,
    /// Config files, relative to the working directory.
    pub configs: Vec<String>,
    /// Named override sets (`--set` syntax), each applied to every config.
    #[serde(default = "default_variants")]
    pub variants: Vec<Variant>,
    /// `adversary.difficulty` levels.
    #[serde(default = "default_difficulties")]
    pub difficulties: Vec<i64>,
    /// Agent specs, weakest first (see `agents/mod.rs`).
    pub agents: Vec<String>,
    #[serde(default, with = "seed_string")]
    pub first_seed: u64,
    /// Seeds per cell: `first_seed .. first_seed + games`.
    pub games: u64,
    /// The seeds that picked a winner: the report confirms on the rest alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_seeds: Option<SelectionSeeds>,
    #[serde(default = "default_max_actions")]
    pub max_actions: usize,
    #[serde(default)]
    pub winprob: WinProb,
    #[serde(default)]
    pub closeness: Closeness,
    /// Records saved per category (close losses, lock-ins, surprises).
    #[serde(default = "default_records")]
    pub records: usize,
    /// Dominance-gate early stopping (`crate::gate`); `games` is then each
    /// check's cap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gate: Option<crate::gate::Gate>,
    /// Panics in one game: how a test proves a failed game doesn't stop a run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inject_panic: Option<InjectPanic>,
}

/// Cell `cell` (grid order) on seed `seed`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InjectPanic {
    pub cell: usize,
    #[serde(with = "seed_string")]
    pub seed: u64,
}

impl Spec {
    /// Games the grid of `cells` has.
    pub fn total_games(&self, cells: &[Cell]) -> usize {
        cells.len() * self.games as usize
    }
}

mod seed_string {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(v: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(v)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum N {
            S(String),
            N(u64),
        }
        match N::deserialize(d)? {
            N::N(n) => Ok(n),
            N::S(s) => s.parse().map_err(serde::de::Error::custom),
        }
    }
}

/// `first .. first + games`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionSeeds {
    #[serde(with = "seed_string")]
    pub first: u64,
    pub games: u64,
}

impl SelectionSeeds {
    pub fn contains(&self, seed: u64) -> bool {
        seed >= self.first && seed - self.first < self.games
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    #[serde(default)]
    pub set: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct WinProb {
    /// Games per cell (the first seeds) that get win-probability checkpoints;
    /// 0 turns them off.
    pub games: u64,
    /// Playouts per checkpoint (the start of each round).
    pub rollouts: usize,
    /// The agent that plays the playouts.
    pub agent: String,
}

impl Default for WinProb {
    fn default() -> Self {
        WinProb {
            games: 0,
            rollouts: 8,
            agent: "greedy".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Closeness {
    /// Probe every n-th decision that offers a real choice; 0 = off.
    pub every: usize,
    /// A decision is close when the best and second-best options score
    /// within this many heuristic points (about credits).
    pub gap: f64,
}

/// Off by default (011): a probe costs about a greedy decision per probed
/// decision. Expensive measures are opt-in, per spec or `--closeness`.
impl Default for Closeness {
    fn default() -> Self {
        Closeness { every: 0, gap: 2.0 }
    }
}

fn default_variants() -> Vec<Variant> {
    vec![Variant {
        name: "base".into(),
        set: Vec::new(),
    }]
}
fn default_difficulties() -> Vec<i64> {
    vec![0]
}
fn default_max_actions() -> usize {
    5000
}
fn default_records() -> usize {
    4
}

/// One cell of the grid: a config under a variant at a difficulty, played
/// by one agent.
pub struct Cell {
    pub config_path: String,
    pub variant: String,
    pub difficulty: i64,
    pub agent: AgentSpec,
    pub rules: Rules,
    /// Overrides applied, in order (variant, then difficulty).
    pub overrides: Vec<String>,
}

impl Cell {
    /// The cell without its agent: what a skill gap or luck share compares across.
    pub fn group(&self) -> (String, String, i64) {
        (
            self.config_path.clone(),
            self.variant.clone(),
            self.difficulty,
        )
    }
}

/// Keeps only the named variants (the two-stage pattern's second stage:
/// probe the finalists). The first variant is the base the others are
/// compared with, so it is always kept, first.
pub fn keep_variants(spec: &mut Spec, keep: &[String]) -> Result<(), String> {
    for k in keep {
        if !spec.variants.iter().any(|v| &v.name == k) {
            let names: Vec<&str> = spec.variants.iter().map(|v| v.name.as_str()).collect();
            return Err(format!(
                "no variant {k:?} in the spec ({})",
                names.join(", ")
            ));
        }
    }
    let mut i = 0;
    spec.variants.retain(|v| {
        i += 1;
        i == 1 || keep.contains(&v.name)
    });
    Ok(())
}

/// Builds every cell, checking every config, override and agent first.
pub fn cells(spec: &Spec) -> Result<Vec<Cell>, String> {
    if spec.configs.is_empty()
        || spec.agents.is_empty()
        || spec.variants.is_empty()
        || spec.difficulties.is_empty()
    {
        return Err(
            "configs, variants, difficulties and agents must each have at least one entry".into(),
        );
    }
    if spec.games == 0 {
        return Err("games must be at least 1".into());
    }
    if spec.first_seed.checked_add(spec.games - 1).is_none() {
        return Err("first_seed + games overflows u64".into());
    }
    let agents: Vec<AgentSpec> = spec
        .agents
        .iter()
        .map(|a| AgentSpec::parse(a))
        .collect::<Result<_, _>>()?;
    // The report keys everything by agent label and variant name: a second
    // cell of the same name would show the first one's numbers.
    let labels: Vec<String> = agents.iter().map(AgentSpec::label).collect();
    if let Some(l) = first_repeat(labels.iter().map(String::as_str)) {
        return Err(format!("two agents play as {l:?}"));
    }
    if let Some(n) = first_repeat(spec.variants.iter().map(|v| v.name.as_str())) {
        return Err(format!("two variants are named {n:?}"));
    }
    if spec.winprob.games > 0 {
        AgentSpec::parse(&spec.winprob.agent).map_err(|e| format!("winprob.agent: {e}"))?;
        if spec.winprob.rollouts == 0 {
            return Err("winprob.rollouts must be at least 1 (or winprob.games 0 for none)".into());
        }
    }
    if !spec.closeness.gap.is_finite() || spec.closeness.gap < 0.0 {
        return Err("closeness.gap must be a number >= 0".into());
    }
    if spec.gate.as_ref().is_some_and(|g| !g.z.is_finite() || g.z <= 0.0) {
        return Err("gate.z must be above 0".into());
    }
    let mut out = Vec::new();
    for path in &spec.configs {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let base = Config::from_json(&text).map_err(|e| format!("{path}: {e}"))?;
        for v in &spec.variants {
            for &d in &spec.difficulties {
                let mut overrides = v.set.clone();
                overrides.push(format!("adversary.difficulty={d}"));
                let mut c = base.clone();
                for o in &overrides {
                    c.apply_override(o)
                        .map_err(|e| format!("{path} ({}): {e}", v.name))?;
                }
                for a in &agents {
                    let rules = Rules::new(&c)
                        .map_err(|e| format!("{path} ({}, difficulty {d}): {e}", v.name))?;
                    out.push(Cell {
                        config_path: path.clone(),
                        variant: v.name.clone(),
                        difficulty: d,
                        agent: a.clone(),
                        rules,
                        overrides: overrides.clone(),
                    });
                }
            }
        }
    }
    if spec.gate.is_some() {
        crate::gate::check_spec(&out)?;
    }
    Ok(out)
}

fn first_repeat<'a>(mut names: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    let mut seen = std::collections::BTreeSet::new();
    names.find(|n| !seen.insert(*n))
}

/// Everything measured in one game; one line of `games.jsonl`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameResult {
    pub cell: usize,
    #[serde(with = "seed_string")]
    pub seed: u64,
    pub won: bool,
    /// End reason, or `timeout`.
    pub reason: String,
    pub rounds: u32,
    pub decisions: u32,
    pub options_total: u64,
    pub forced: u32,
    /// Decisions probed for closeness, how many were close, and the sum of
    /// best-minus-second gaps (capped at 50 so one blunder-or-win option
    /// does not swamp the mean).
    pub probed: u32,
    pub close: u32,
    pub gap_sum: f64,
    pub margin: Margin,
    pub adversary_ops: Vec<u32>,
    pub setup_ops: u32,
    pub counters: BTreeMap<String, i64>,
    /// Per card: how it was offered, drafted, kept, played and drawn.
    pub cards: BTreeMap<String, CardCounts>,
    pub pays: u32,
    pub withdraws: u32,
    /// Each seat's AI and buildings at the end, `builder:a+b|protector:c`.
    pub build: String,
    /// 121: each seat's highest holding after any action, in [`PEAKS`] order
    /// (the runaway check, `scripts/cards/runaway_read.py`).
    #[serde(default)]
    pub peaks: Vec<Vec<i64>>,
    /// 121: what each seat gained over the game, in [`PEAKS`] order (rises
    /// between actions summed); over `rounds`, a round's income.
    #[serde(default)]
    pub gained: Vec<Vec<i64>>,
    /// 124: each seat's holdings when play starts (setup's), in [`PEAKS`] order.
    #[serde(default)]
    pub start: Vec<Vec<i64>>,
    /// 124: per seat, `gained` split by where each rise came from (see
    /// [`track_holdings`]: `own:`/`other:` an action, or `phase:` an automatic
    /// phase), in [`PEAKS`] order; a label whose row is all zero is left out.
    #[serde(default)]
    pub sources: Vec<BTreeMap<String, Vec<i64>>>,
    /// 125: what each seat spent or lost over the game, in [`PEAKS`] order
    /// (falls between actions summed): `start + gained - spent` is the end.
    #[serde(default)]
    pub spent: Vec<Vec<i64>>,
    /// 125: per seat, `spent` split by where each fall went, labeled as
    /// `sources` labels a rise.
    #[serde(default)]
    pub uses: Vec<BTreeMap<String, Vec<i64>>>,
    /// 124: per seat, its draft and keep decisions after the opening.
    #[serde(default)]
    pub keeps: Vec<KeepCounts>,
    #[serde(skip)]
    pub last_held: Vec<Vec<i64>>,
    /// Win probability at the start of each round (round 1 first), from playouts.
    pub winprob: Vec<f64>,
    pub ms: f64,
    /// Of `ms`, the time spent in the agent's own decisions and in the
    /// closeness and win-probability probes.
    pub agent_ms: f64,
    pub probe_ms: f64,
    /// The rest of `ms`: building the game and the agent, generating the
    /// legal actions, applying the chosen ones (with the measures' own
    /// bookkeeping), and the replay check.
    pub setup_ms: f64,
    pub legal_ms: f64,
    pub apply_ms: f64,
    pub replay_ms: f64,
    pub replay_ok: bool,
    pub actions: Vec<Action>,
    /// Rebuilt from the actions when read back.
    #[serde(skip)]
    pub final_state: Option<Box<GameState>>,
}

/// A game that panicked (046: one killed a gate 25 minutes in): one line of
/// `failed.jsonl`, with all it takes to play the game again. It has no result,
/// so no measure counts it; `--resume` plays it again.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedGame {
    pub cell: usize,
    #[serde(with = "seed_string")]
    pub seed: u64,
    pub config: String,
    pub variant: String,
    pub difficulty: i64,
    pub agent: String,
    pub overrides: Vec<String>,
    /// The spec's cap, for [`FailedGame::command`]; 0 in a list written before it.
    #[serde(default)]
    pub max_actions: usize,
    /// The panic: where, then its message.
    pub message: String,
}

impl FailedGame {
    fn new(spec: &Spec, cells: &[Cell], cell: usize, seed: u64, message: String) -> FailedGame {
        let c = &cells[cell];
        FailedGame {
            cell,
            seed,
            config: c.config_path.clone(),
            variant: c.variant.clone(),
            difficulty: c.difficulty,
            agent: c.agent.label(),
            overrides: c.overrides.clone(),
            max_actions: spec.max_actions,
            message,
        }
    }

    /// The `aic-sim` line that plays it again.
    pub fn command(&self) -> String {
        let sets: String = self.overrides.iter().map(|o| format!(" --set {o}")).collect();
        let cap = match self.max_actions {
            0 => String::new(),
            n => format!(" --max-actions {n}"),
        };
        format!(
            "aic-sim run {} --seed {} --games 1 --agent {}{cap}{sets}",
            self.config, self.seed, self.agent
        )
    }
}

/// One card's counts, in one game or summed over games. Every dealt card is
/// drafted by someone (packets are passed until empty), so how early it goes
/// is the draft's signal.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardCounts {
    /// Times offered in a draft packet.
    pub offered: u32,
    pub drafted: u32,
    /// Every keep, the opening deal's included: not a share of `offered`.
    pub kept: u32,
    pub played: u32,
    /// The sum of its pick positions (1 = a seat's first pick of the round).
    pub pick_sum: u32,
    /// Times drawn into a hand by an effect.
    pub drawn: u32,
}

impl std::ops::AddAssign for CardCounts {
    fn add_assign(&mut self, o: Self) {
        self.offered += o.offered;
        self.drafted += o.drafted;
        self.kept += o.kept;
        self.played += o.played;
        self.pick_sum += o.pick_sum;
        self.drawn += o.drawn;
    }
}

/// How a game ended, measured on the final table.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Margin {
    /// Adversary cards still to come (the face-up one and the deck): rounds
    /// to spare on a win.
    pub deck_left: u32,
    /// Adversary ambassadors left in the supply, and at the start.
    pub supply_left: i64,
    pub supply_start: i64,
    /// Cities with the adversary's cube at the end (every city = a loss),
    /// and cities with a player cube.
    pub cities_corrupted_end: i64,
    pub cities_aligned_end: i64,
    /// Adversary pieces keeping the current level's bar from being met
    /// (corrupted cities, and ambassadors or troops the bar still counts;
    /// 0 on a win): how far a loss was from winning.
    pub blockers: i64,
    pub level: i64,
    /// Unity tips.
    pub unity_tips: i64,
    /// Botnets and adversary troops on the board at the end (018).
    pub botnets_end: i64,
    pub troops_end: i64,
    /// What each seat left unspent, in [`UNSPENT`] order (068): [`left_unspent`].
    #[serde(default)]
    pub unspent: Vec<Vec<i64>>,
    /// 121: what each seat held when its last completed action phase ended, in
    /// [`UNSPENT`] order (0 if none completed). Unlike `unspent`, never "now".
    #[serde(default)]
    pub held_after_actions: Vec<Vec<i64>>,
}

/// What seat `s` held when its last completed action phase ended (121).
fn held_after_actions(g: &GameState, s: usize) -> Vec<i64> {
    UNSPENT
        .iter()
        .map(|r| {
            g.stats
                .per_round
                .get(format!("unspent.{s}.{r}").as_str())
                .and_then(|v| v.last().copied())
                .unwrap_or(0)
        })
        .collect()
}

/// What seat `s` left unspent, in [`UNSPENT`] order: what it held when its last
/// action phase ended (production and upkeep pay after that, and a seat can't
/// have spent what it was just paid), or now if the game ended in one.
fn left_unspent(g: &GameState, s: usize) -> Vec<i64> {
    let in_actions = g.config.round.get(g.phase).is_some_and(|p| p.contains("cards.play"));
    UNSPENT
        .iter()
        .map(|r| {
            let at_end = g.stats.per_round.get(format!("unspent.{s}.{r}").as_str()).and_then(|v| v.last().copied());
            match at_end {
                Some(n) if !in_actions => n,
                _ => g.resource(s, *r),
            }
        })
        .collect()
}

/// The resources the unspent measure reads (068). Insight is not one: what a
/// round leaves of it is lost.
pub const UNSPENT: [Resource; 5] = board::LEFT;

/// A seat's mean left at game end above this flags the resource as ending high
/// (068, in [`UNSPENT`] order): about a round's income of each, so a seat
/// ending there left a round's worth of play on the table. `docs/principles.md`.
pub const UNSPENT_HIGH: [f64; 5] = [12.0, 4.0, 4.0, 2.0, 3.0];

fn margin(rules: &Rules, g: &GameState) -> Margin {
    let w = rules.world();
    let level = alignment::bar_level(&w, g);
    let blockers = if g.outcome.as_ref().is_some_and(|o| o.won) {
        0
    } else {
        adversary::purge_count(&w, g, level)
    };
    let cities: Vec<usize> = (0..g.hexes.len())
        .filter(|&h| common::is_city(&w, h))
        .collect();
    Margin {
        deck_left: Eval::cards_left(g) as u32,
        supply_left: g.var(adversary::SUPPLY),
        supply_start: adversary::start_supply(&w),
        cities_corrupted_end: cities.iter().filter(|&&h| common::corrupted(g, h)).count() as i64,
        cities_aligned_end: cities
            .iter()
            .filter(|&&h| common::cube_owner(g, h).is_some())
            .count() as i64,
        blockers,
        level: level.steps(),
        unity_tips: g.var(alignment::tips_key(Side::Unity)),
        botnets_end: (0..g.hexes.len()).map(|h| g.tokens(h, common::BOTNET)).sum(),
        troops_end: (0..g.hexes.len()).map(|h| g.tokens(h, common::TROOP)).sum(),
        unspent: (0..g.seats.len()).map(|s| left_unspent(g, s)).collect(),
        held_after_actions: (0..g.seats.len()).map(|s| held_after_actions(g, s)).collect(),
    }
}

/// How a game ended: its reason, or `timeout` when the action cap hit first.
pub fn end_reason(outcome: Option<&Outcome>) -> String {
    outcome.map_or("timeout".into(), |o| o.reason.clone())
}

/// The generic per-round counter (017), split: adversary operations during
/// setup (entry 0), and in each round since (entry r, round r).
pub fn adversary_ops(stats: &Stats) -> (i64, &[i64]) {
    match stats.per_round.get("adversary.ops").map(|v| v.as_slice()) {
        Some([setup, rounds @ ..]) => (*setup, rounds),
        _ => (0, &[]),
    }
}

fn build_signature(rules: &Rules, g: &GameState) -> String {
    let w = rules.world();
    (0..g.seats.len())
        .map(|s| {
            let ai = &aic_core::modules::ai::board(&w, s).id;
            let mut b: Vec<&str> = g
                .pile(&cards::built(s))
                .iter()
                .map(|x| x.as_str())
                .collect();
            b.sort_unstable();
            format!("{ai}:{}", b.join("+"))
        })
        .collect::<Vec<_>>()
        .join("|")
}

/// Plays `rollouts` games from a guess of `g` with `agent`; the share won.
fn win_probability(
    rules: &Rules,
    g: &GameState,
    agent: &AgentSpec,
    rollouts: usize,
    salt: u64,
    max_actions: usize,
) -> f64 {
    let mut rng = agent_rng(g.seed ^ salt, "winprob");
    let mut wins = 0;
    let mut legal = Vec::new();
    for k in 0..rollouts {
        let mut s = g.clone();
        determinize(rules, &mut s, &mut rng);
        let mut a = agent.build(
            rules,
            g.seed
                .wrapping_add(salt)
                .wrapping_add(k as u64 * 0x9e37_79b9),
        );
        let mut n = 0;
        while n < max_actions {
            rules.legal_actions_into(&s, &mut legal);
            if legal.is_empty() {
                break;
            }
            let i = a.choose(rules, &s, &legal);
            if rules.apply_mut(&mut s, &legal[i], None).is_err() {
                break;
            }
            n += 1;
        }
        wins += s.outcome.as_ref().is_some_and(|o| o.won) as usize;
    }
    wins as f64 / rollouts as f64
}

/// Plays one game of one cell and measures it.
pub fn play_one(spec: &Spec, cells: &[Cell], ci: usize, seed: u64) -> GameResult {
    if spec.inject_panic.as_ref().is_some_and(|p| p.cell == ci && p.seed == seed) {
        panic!("injected panic (the spec's inject_panic)");
    }
    let cell = &cells[ci];
    let rules = &cell.rules;
    let start = Instant::now();
    let mut agent = cell.agent.build(rules, seed);
    let probe_eval = Eval::new(rules, Weights::default());
    let mut probe_rng = agent_rng(seed, "probe");
    let wp_agent = (spec.winprob.games > 0 && seed - spec.first_seed < spec.winprob.games)
        .then(|| AgentSpec::parse(&spec.winprob.agent).expect("checked in cells()"));
    let mut g = rules.new_game(seed);
    let mut legal = Vec::new();
    let mut r = GameResult {
        cell: ci,
        seed,
        ..Default::default()
    };
    let (mut offered_round, mut checked_round, mut real_choices) = (0u32, 0u32, 0usize);
    let mut picks = vec![0u32; g.seats.len()];
    let mut fill_round = 0u32;
    let mut slots: BTreeMap<String, i64> = BTreeMap::new();
    r.setup_ms = ms_since(start);
    track_holdings(&mut r, &g, None);
    // 124: each action's events, for `sources` (they change no state).
    let mut events = Vec::new();
    while r.actions.len() < spec.max_actions {
        let t = Instant::now();
        rules.legal_actions_into(&g, &mut legal);
        r.legal_ms += ms_since(t);
        if legal.is_empty() {
            break;
        }
        if let Some(a) = &wp_agent {
            if g.round != checked_round && g.round >= 1 {
                checked_round = g.round;
                let t = Instant::now();
                let p = win_probability(
                    rules,
                    &g,
                    a,
                    spec.winprob.rollouts,
                    0x5eed_0000 + g.round as u64,
                    spec.max_actions,
                );
                r.winprob.push(p);
                r.probe_ms += ms_since(t);
            }
        }
        // Draft packets are dealt face down to each seat and passed: the
        // first draft decision of a round sees every card dealt.
        if g.round != offered_round && legal.iter().any(|a| matches!(a, Action::Draft { .. })) {
            offered_round = g.round;
            picks.iter_mut().for_each(|p| *p = 0);
            count_offers(&mut r, &g);
        }
        let n = legal.len();
        r.decisions += 1;
        r.options_total += n as u64;
        if n == 1 {
            r.forced += 1;
        } else if spec.closeness.every > 0 {
            real_choices += 1;
            if real_choices % spec.closeness.every == 0 {
                let t = Instant::now();
                let how = search::Scoring::decision(1, 1, 1);
                let mut s = search::greedy_scores(rules, &probe_eval, &mut probe_rng, &g, &legal, how);
                record_closeness(&mut r, &mut s, spec.closeness.gap);
                r.probe_ms += ms_since(t);
            }
        }
        let t = Instant::now();
        let i = agent.choose(rules, &g, &legal);
        r.agent_ms += ms_since(t);
        let t = Instant::now();
        let a = legal[i].clone();
        // 092: building over a standing building, and how full each seat's
        // slots are as each round starts.
        if g.round != fill_round {
            fill_round = g.round;
            for s in 0..g.seats.len() {
                let used = g.pile(&cards::built(s)).len() as i64;
                slots.insert(format!("slots.used.{s}.r{:02}", g.round), used);
                slots.insert(format!("slots.cap.{s}"), cards::capacity(&rules.world(), &g, s));
            }
        }
        if let Action::Choose { seat, option, .. } = &a {
            if option.starts_with("over ") {
                *slots.entry(format!("slots.over.{seat}")).or_default() += 1;
                *slots.entry(format!("slots.over.{seat}.r{:02}", g.round)).or_default() += 1;
            }
        }
        // 125: as a seat ends its action phase, each card in its hand that
        // costs energy, and those it can't play for want of energy.
        if let Action::Pass { seat } = &a {
            if rules.phase(&g).is_some_and(|p| p.name.starts_with("cards.play")) {
                let s = *seat as usize;
                let held = g.seats[s].resources.get(&Resource::Energy).copied().unwrap_or(0);
                let w = rules.world();
                // 128: each standing building with an Action as its seat passes:
                // its Action offered and not taken (`skipped`), or not offered
                // (`blocked`: used this round, unaffordable or no target), and
                // the energy the seat held.
                for c in g.pile(&cards::built(s)).iter() {
                    if w.content.card(c).ability.is_some() {
                        let offered = legal.iter().any(|x| matches!(x, Action::Use { card, .. } if card == c));
                        let how = if offered { "skipped" } else { "blocked" };
                        *slots.entry(format!("actions.{how}.{s}.{c}")).or_default() += 1;
                        *slots.entry(format!("actions.{how}_energy.{s}.{c}")).or_default() += held;
                    }
                }
                for c in g.pile(&cards::hand(s)).iter() {
                    let need = w.content.card(c).energy;
                    if need > 0 {
                        *slots.entry(format!("energy_cards.held.{s}")).or_default() += 1;
                        if need > held {
                            *slots.entry(format!("energy_cards.short.{s}")).or_default() += 1;
                            *slots.entry(format!("energy_cards.short.{s}.{c}")).or_default() += 1;
                        }
                    }
                }
            }
        }
        tally_action(&mut r, &a, &mut picks);
        let src = source_of(rules, &g, &a);
        tally_keeps(&mut r, rules, &g, &a);
        events.clear();
        apply_counting_draws(&mut r, rules, &mut g, &a, Some(&mut events));
        track_holdings(&mut r, &g, Some((&src, &events)));
        r.actions.push(a);
        r.apply_ms += ms_since(t);
    }
    finish(&mut r, rules, g);
    r.counters.extend(slots);
    r.ms = ms_since(start);
    r
}

/// Every card in the round's draft packets counts as offered.
fn count_offers(r: &mut GameResult, g: &GameState) {
    for (k, pile) in g.piles.from_key(cards::PACKETS) {
        if !k.starts_with(cards::PACKETS) {
            break;
        }
        for c in pile.iter() {
            r.cards.entry(c.to_string()).or_default().offered += 1;
        }
    }
}

/// One closeness probe's result, from the options' `scores`.
fn record_closeness(r: &mut GameResult, scores: &mut [f64], close_gap: f64) {
    scores.sort_by(|a, b| b.total_cmp(a));
    // An option the rules refused on a guess scores -inf: two of
    // them would make the gap NaN; count that as wide open.
    let gap = (scores[0] - scores[1]).clamp(0.0, 50.0);
    let gap = if gap.is_nan() { 50.0 } else { gap };
    r.probed += 1;
    r.gap_sum += gap;
    r.close += (gap < close_gap) as u32;
}

/// What the action `a` itself counts for; `picks` is each seat's picks this round.
fn tally_action(r: &mut GameResult, a: &Action, picks: &mut [u32]) {
    match a {
        Action::Draft { seat, card } => {
            let s = *seat as usize;
            picks[s] += 1;
            let e = r.cards.entry(card.to_string()).or_default();
            e.drafted += 1;
            e.pick_sum += picks[s];
        }
        Action::Keep { card, .. } => r.cards.entry(card.to_string()).or_default().kept += 1,
        Action::Play { card, .. } => r.cards.entry(card.to_string()).or_default().played += 1,
        Action::Pay { .. } | Action::CounterWith { .. } => r.pays += 1,
        Action::Withdraw { .. } => r.withdraws += 1,
        _ => {}
    }
}

/// Applies `a`, counting the cards it drew. A draw's card is only known once
/// drawn, and any effect of any step may draw, for any seat: whatever is new
/// in a hand, bar a keep's card.
fn apply_counting_draws(
    r: &mut GameResult,
    rules: &Rules,
    g: &mut GameState,
    a: &Action,
    events: Option<&mut Vec<aic_core::Event>>,
) {
    let mut hands: Vec<Vec<aic_core::Key>> = (0..g.seats.len())
        .map(|s| g.pile(&cards::hand(s)).to_vec())
        .collect();
    if let Action::Keep { seat, card } = a {
        hands[*seat as usize].push(*card);
    }
    rules.apply_mut(g, a, events).expect("a legal action applies");
    for (s, mut before) in hands.into_iter().enumerate() {
        for c in g.pile(&cards::hand(s)) {
            match before.iter().position(|b| b == c) {
                Some(k) => {
                    before.swap_remove(k);
                }
                None => r.cards.entry(c.to_string()).or_default().drawn += 1,
            }
        }
    }
}

/// What [`GameResult::peaks`] and [`GameResult::gained`] measure, in order
/// (121, the runaway check): each resource a seat holds, its tokens on cards
/// (summed over its cards) and its ambassadors in supply.
pub const PEAKS: [&str; 8] = [
    "credits",
    "insight",
    "energy",
    "chips",
    "antimatter",
    "research_cubes",
    "card_tokens",
    "ambassadors",
];

/// Seat `s`'s holdings in [`PEAKS`] order.
fn holdings(g: &GameState, s: usize) -> [i64; 8] {
    let prefix = format!("cards.tokens.{s}.");
    let tokens: i64 = g
        .vars
        .from_key(prefix.as_str())
        .take_while(|(k, _)| k.starts_with(prefix.as_str()))
        .map(|(_, v)| (*v).max(0))
        .sum();
    [
        g.resource(s, Resource::Credits),
        g.resource(s, Resource::Insight),
        g.resource(s, Resource::Energy),
        g.resource(s, Resource::Chips),
        g.resource(s, Resource::Antimatter),
        g.resource(s, Resource::ResearchCubes),
        tokens,
        g.resource(s, Resource::Ambassadors),
    ]
}

/// 124: one seat's draft and keep decisions after the opening (the opening
/// deals a hand to keep from and has no draft), from its actions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeepCounts {
    /// Draft picks it made: each is a card it may then keep.
    pub offered: u32,
    /// Keeps it made once it had made a draft pick.
    pub kept: u32,
    /// Tier choices for its draft hand, and how many of them were tier 2.
    pub tiers: u32,
    pub tier2: u32,
    /// Its insight as it made its first decision of each keep phase after the
    /// opening (a keep or a pass; a seat with nothing drafted has none).
    pub insight: Vec<i64>,
    #[serde(skip)]
    last_round: u32,
}

/// Counts `a` into its seat's [`KeepCounts`] (before it is applied).
fn tally_keeps(r: &mut GameResult, rules: &Rules, g: &GameState, a: &Action) {
    if r.keeps.len() != g.seats.len() {
        r.keeps = vec![KeepCounts::default(); g.seats.len()];
    }
    let s = a.seat() as usize;
    let Some(k) = r.keeps.get_mut(s) else { return };
    match a {
        Action::Draft { .. } => k.offered += 1,
        Action::Keep { .. } if k.offered > 0 => k.kept += 1,
        Action::Tier { tier, .. } => {
            k.tiers += 1;
            k.tier2 += u32::from(*tier == 2);
        }
        _ => {}
    }
    let at_keep = rules.phase(g).is_some_and(|p| p.name == "cards.keep");
    if at_keep && k.offered > 0 && k.last_round != g.round {
        k.last_round = g.round;
        k.insight.push(g.resource(s, Resource::Insight));
    }
}

/// Who acted (none for an automatic phase's choice) and the action's label.
type Source = (Option<u8>, String);

/// 124: what [`GameResult::sources`] labels a rise by, read before `a` is
/// applied: the seat whose action it is (none for a choice an automatic phase
/// asked) and the action, `<type>[:<card, power name, threat or act>]`. A
/// choice is labeled by the action that opened it, so a card's choice counts
/// as its play.
fn source_of(rules: &Rules, g: &GameState, a: &Action) -> Source {
    let a = match (a, g.pending.as_deref()) {
        (Action::Choose { .. }, Some(p)) => match &p.trigger {
            aic_core::state::Trigger::Action { action } => action,
            aic_core::state::Trigger::Phase => {
                let phase = rules.phase(g).map_or("", |p| p.name.as_str());
                return (None, format!("phase:{phase}"));
            }
        },
        _ => a,
    };
    let label = match a {
        Action::Pass { .. } => "pass".to_string(),
        Action::Choose { .. } => "choose".to_string(),
        Action::Tier { .. } => "tier".to_string(),
        Action::Draft { .. } => "draft".to_string(),
        Action::Keep { .. } => "keep".to_string(),
        Action::Play { card, .. } => format!("play:{card}"),
        Action::Use { card, .. } => format!("use:{card}"),
        Action::Power { seat, power } => {
            let w = rules.world();
            let b = aic_core::modules::ai::board(&w, *seat as usize);
            match b.powers.get(*power as usize) {
                Some(p) => format!("power:{}", p.name),
                None => format!("power:{power}"),
            }
        }
        Action::Pay { threat, .. } => format!("pay:{threat}"),
        Action::CounterWith { threat, .. } => format!("counter:{threat}"),
        Action::Withdraw { threat, .. } => format!("withdraw:{threat}"),
        Action::Antimatter { .. } => "antimatter".to_string(),
        Action::Standard { act, .. } => format!("standard:{act}"),
        Action::Sell { .. } => "sell".to_string(),
        Action::Convert { country, .. } => format!("convert:{country}"),
    };
    (Some(a.seat()), label)
}

/// 124: the phases an action's events ran, and what each seat gained in each:
/// `(seat, PEAKS index, phase, sum of rises)`, `None` for the action itself,
/// before any phase it set running began (the last pass of a phase earns
/// nothing itself; the production it runs does).
fn gains_by_phase(events: &[aic_core::Event], sign: i64) -> Vec<(usize, usize, Option<&str>, i64)> {
    use aic_core::Event;
    type Gain<'e> = (usize, usize, Option<&'e str>, i64);
    fn add<'e>(out: &mut Vec<Gain<'e>>, s: usize, i: usize, ph: Option<&'e str>, d: i64) {
        match out.iter_mut().find(|(s2, i2, p2, _)| *s2 == s && *i2 == i && *p2 == ph) {
            Some(e) => e.3 += d,
            None => out.push((s, i, ph, d)),
        }
    }
    let mut phase: Option<&str> = None;
    let mut out: Vec<Gain> = Vec::new();
    for e in events {
        match e {
            Event::PhaseStarted { phase: p } => phase = Some(p.as_str()),
            Event::ResourceChanged { seat, resource, delta, .. } if *delta * sign > 0 => {
                let i = match resource {
                    Resource::Credits => 0,
                    Resource::Insight => 1,
                    Resource::Energy => 2,
                    Resource::Chips => 3,
                    Resource::Antimatter => 4,
                    Resource::ResearchCubes => 5,
                    Resource::Ambassadors => 7,
                };
                add(&mut out, *seat as usize, i, phase, *delta * sign);
            }
            Event::VarChanged { key, delta, .. } if *delta * sign > 0 => {
                let seat = key.strip_prefix("cards.tokens.").and_then(|k| k.split('.').next()?.parse().ok());
                if let Some(s) = seat {
                    add(&mut out, s, 6, phase, *delta * sign);
                }
            }
            _ => {}
        }
    }
    out
}

/// After each action: each seat's peak holding and what it has gained (the
/// sum of rises between actions), sim-side so no rule or state moves; with
/// `src` (who acted and the label, from [`source_of`], and the action's
/// events), each rise also goes to its source: `phase:<name>` when an
/// automatic phase the action set running gave the most of it (production,
/// upkeep, the next round's opening), else `own:` or `other:` the label (the
/// label alone for a choice an automatic phase asked). 125: falls the same
/// way, into `spent` and `uses`.
fn track_holdings(r: &mut GameResult, g: &GameState, src: Option<(&Source, &[aic_core::Event])>) {
    let by_phase = src.map(|(_, ev)| gains_by_phase(ev, 1)).unwrap_or_default();
    let falls_by_phase = src.map(|(_, ev)| gains_by_phase(ev, -1)).unwrap_or_default();
    let seats = g.seats.len();
    if r.peaks.len() != seats {
        r.peaks = (0..seats).map(|s| holdings(g, s).to_vec()).collect();
        r.gained = vec![vec![0; PEAKS.len()]; seats];
        r.spent = vec![vec![0; PEAKS.len()]; seats];
        r.start = r.peaks.clone();
        r.sources = vec![BTreeMap::new(); seats];
        r.uses = vec![BTreeMap::new(); seats];
        r.last_held = r.peaks.clone();
        return;
    }
    for s in 0..seats {
        let now = holdings(g, s);
        for (i, &n) in now.iter().enumerate() {
            let rise = (n - r.last_held[s][i]).max(0);
            let fall = (r.last_held[s][i] - n).max(0);
            r.peaks[s][i] = r.peaks[s][i].max(n);
            r.gained[s][i] += rise;
            r.spent[s][i] += fall;
            r.last_held[s][i] = n;
            let Some(((actor, label), _)) = src else { continue };
            for (amount, parts, out) in [(rise, &by_phase, &mut r.sources), (fall, &falls_by_phase, &mut r.uses)] {
                if amount == 0 {
                    continue;
                }
                // The part of the action that moved it most (the first on a tie).
                let mut best: Option<(Option<&str>, i64)> = None;
                for &(s2, i2, ph, d) in parts {
                    if s2 == s && i2 == i && best.is_none_or(|(_, b)| d > b) {
                        best = Some((ph, d));
                    }
                }
                let key = match (best, actor) {
                    (Some((Some(ph), _)), _) => format!("phase:{ph}"),
                    (_, None) => label.clone(),
                    (_, Some(a)) if *a as usize == s => format!("own:{label}"),
                    (_, Some(_)) => format!("other:{label}"),
                };
                out[s].entry(key).or_insert_with(|| vec![0; PEAKS.len()])[i] += amount;
            }
        }
    }
}

/// The replay check and what the final table shows.
fn finish(r: &mut GameResult, rules: &Rules, g: GameState) {
    let t = Instant::now();
    r.replay_ok = replay(rules, r.seed, &r.actions).is_ok_and(|h| h == g);
    r.replay_ms = ms_since(t);
    r.won = g.outcome.as_ref().is_some_and(|o| o.won);
    r.reason = end_reason(g.outcome.as_ref());
    r.rounds = g.round;
    r.margin = margin(rules, &g);
    let (setup_ops, round_ops) = adversary_ops(&g.stats);
    r.setup_ops = setup_ops as u32;
    r.adversary_ops = (0..g.round as usize)
        .map(|i| round_ops.get(i).copied().unwrap_or(0) as u32)
        .collect();
    r.counters = g.stats.counters.iter().map(|(k, v)| (k.to_string(), *v)).collect();
    // 099: a card played by an effect (Rush Build's free building) is no Play
    // action: the engine's count of plays is the whole of it.
    for (k, v) in &g.stats.counters {
        if let Some(id) = k.strip_prefix("cards.played.") {
            let e = r.cards.entry(id.to_string()).or_default();
            e.played = e.played.max(*v as u32);
        }
    }
    // 021: the per-round tables named in PER_ROUND_EXPORT, as `<key>.rNN`.
    // 100: every round the counter's table reaches is written, zeros
    // included, so an absent `.rNN` means no record by round NN (for a counter
    // recorded every round, such as `unspent.*`, the round was not reached).
    for (k, v) in &g.stats.per_round {
        if PER_ROUND_EXPORT.iter().any(|(pre, suf)| k.starts_with(pre) && k.ends_with(suf)) {
            for (i, n) in v.iter().enumerate().skip(1) {
                r.counters.insert(format!("{k}.r{i:02}"), *n);
            }
        }
    }
    r.build = build_signature(rules, &g);
    r.final_state = Some(Box::new(g));
}

/// The per-round counters games.jsonl carries, as (prefix, suffix) pairs: a
/// key matching both is written round by round (`<key>.rNN`). Others
/// (`adversary.*`) stay out, to keep games.jsonl small; a new by-round read
/// adds its pair here. `unspent.<seat>.<resource>` is what a seat holds as the
/// action phase ends, before production and upkeep pay (090: energy; 148: every
/// resource, research cubes included, for the leftover read).
pub const PER_ROUND_EXPORT: &[(&str, &str)] = &[
    ("cards.", ""),
    ("ai.", ""),
    ("economy.", ""),
    ("board.node", ""),
    ("unspent.", ""),
    // 135: threats countered and uncountered, round by round.
    ("threats.", ""),
];

fn ms_since(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

/// Where [`run`] puts what it plays.
pub struct Sinks<'a> {
    pub results: &'a Mutex<Vec<GameResult>>,
    pub failed: &'a Mutex<Vec<FailedGame>>,
    /// Called with each game, the games finished and the total, under
    /// `results`' lock.
    pub on_game: &'a (dyn Fn(&GameResult, usize, usize) + Sync),
    /// Called with each game that panicked, under `failed`'s lock.
    pub on_failed: &'a (dyn Fn(&FailedGame) + Sync),
}

/// Plays every (cell, seed) not already in `results` on `threads` workers,
/// adding each game to `results` as it finishes and handing it to
/// `on_game`, so a caller can stream games to disk and report from `results`
/// while the run goes on. A game that panics goes to `failed` instead and the
/// worker takes the next. Work is handed out one game at a time (agents differ
/// a hundredfold in cost), seed by seed across every cell, the highest ladder
/// tier first within a seed: a run stopped early has every cell's first
/// seeds, not some cells whole and others missing. Sort the results by cell
/// then seed ([`sort`]) and nothing depends on the thread count or order.
/// With a `gate`, the same but seed block by seed block ([`crate::gate`]).
pub fn run(spec: &Spec, cells: &[Cell], threads: usize, sinks: &Sinks) {
    capture_panics();
    let results = sinks.results;
    let total = spec.total_games(cells);
    let have = |results: &Mutex<Vec<GameResult>>| -> std::collections::BTreeSet<(usize, u64)> {
        let out = results.lock().expect("results lock");
        out.iter().map(|r| (r.cell, r.seed)).collect()
    };
    if let Some(gate) = &spec.gate {
        for (from, to) in gate.blocks(spec.games) {
            let (live, order) = {
                let out = results.lock().expect("results lock");
                let failed = sinks.failed.lock().expect("failed lock");
                let state = crate::gate::evaluate(gate, spec, cells, &out, &failed);
                (state.live_at(from), state.order(cells, &out))
            };
            if live.is_empty() {
                break;
            }
            let done = have(results);
            let mut jobs: Vec<(usize, u64)> = (from..to)
                .flat_map(|k| live.iter().map(move |&c| (c, spec.first_seed + k)))
                .filter(|j| !done.contains(j))
                .collect();
            jobs.sort_by_key(|&(c, seed)| (seed, order[c]));
            play_jobs(spec, cells, &jobs, threads, total, sinks);
        }
        return;
    }
    let done = have(results);
    let mut jobs: Vec<(usize, u64)> = (0..spec.games)
        .flat_map(|k| (0..cells.len()).map(move |c| (c, spec.first_seed + k)))
        .filter(|j| !done.contains(j))
        .collect();
    jobs.sort_by_key(|&(c, seed)| (seed, std::cmp::Reverse(cells[c].agent.tier())));
    play_jobs(spec, cells, &jobs, threads, total, sinks);
}

thread_local! {
    /// This thread's last panic, as the hook saw it: `catch_unwind` hands back
    /// only the message, not where.
    static LAST_PANIC: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Keeps each panic's text for [`FailedGame`], on top of the usual hook.
fn capture_panics() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let usual = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            LAST_PANIC.with(|p| *p.borrow_mut() = info.to_string().replace('\n', " "));
            usual(info);
        }));
    });
}

fn play_jobs(
    spec: &Spec,
    cells: &[Cell],
    jobs: &[(usize, u64)],
    threads: usize,
    total: usize,
    sinks: &Sinks,
) {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let k = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&(c, seed)) = jobs.get(k) else { break };
                    match catch_unwind(AssertUnwindSafe(|| play_one(spec, cells, c, seed))) {
                        Ok(res) => {
                            let mut out = sinks.results.lock().expect("results lock");
                            (sinks.on_game)(&res, out.len() + 1, total);
                            out.push(res);
                        }
                        Err(_) => {
                            let message = LAST_PANIC.with(|p| p.take());
                            let f = FailedGame::new(spec, cells, c, seed, message);
                            let mut out = sinks.failed.lock().expect("failed lock");
                            (sinks.on_failed)(&f);
                            out.push(f);
                        }
                    }
                }
                crate::prof::flush();
            });
        }
    });
}

/// Results in report order: by cell, then seed.
pub fn sort(games: &mut [GameResult]) {
    games.sort_by_key(|r| (r.cell, r.seed));
}

/// The first line of `games.jsonl`: what the games that follow were played
/// under, so a resumed run only adds games of the same experiment.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StreamHeader {
    pub format: String,
    pub spec: Spec,
    pub engine_version: String,
    pub rules_revision: u32,
    pub content_hash: String,
    /// Each cell's resolved config hash: the spec names config files, and
    /// an edited file is another experiment.
    pub config_hashes: Vec<String>,
}

impl StreamHeader {
    pub fn new(spec: &Spec, cells: &[Cell]) -> StreamHeader {
        StreamHeader {
            format: STREAM_FORMAT.into(),
            spec: spec.clone(),
            engine_version: aic_core::VERSION.into(),
            rules_revision: aic_core::RULES_REVISION,
            content_hash: aic_core::content_hash(),
            config_hashes: cells.iter().map(|c| c.rules.config_hash.clone()).collect(),
        }
    }
}

/// 2 (016): `margin.points` became `margin.unity_tips`, and each card's
/// counts an object (`CardCounts`) instead of a 6-number array.
pub const STREAM_FORMAT: &str = "aic-games/2";

/// Reads a `games.jsonl` written for `spec` and `cells`: the games in it,
/// each replayed from its seed and actions to rebuild its final table (and
/// checked to end as it recorded). A last line cut short (the run was
/// killed mid-write) is dropped.
pub fn read_stream(path: &str, spec: &Spec, cells: &[Cell]) -> Result<Vec<GameResult>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut lines = text.lines();
    let head: StreamHeader = lines
        .next()
        .ok_or_else(|| format!("{path}: empty"))
        .and_then(|l| serde_json::from_str(l).map_err(|e| format!("{path}: header: {e}")))?;
    let want = StreamHeader::new(spec, cells);
    if head.format != want.format {
        return Err(format!(
            "{path}: format {:?}, expected {:?}",
            head.format, want.format
        ));
    }
    if head != want {
        // Name what differs (020: it printed the current revision and content,
        // which pointed at the engine when the cause was --quick left off).
        let mut differs = Vec::new();
        if head.spec != want.spec {
            differs.push(
                "the spec (as run: --quick, --games, --seed, --variants and the probes rewrite it; \
                 give the same flags as the run you resume)"
                    .to_string(),
            );
        }
        if head.engine_version != want.engine_version {
            differs.push(format!("the engine ({} then, {} now)", head.engine_version, want.engine_version));
        }
        if head.rules_revision != want.rules_revision {
            differs.push(format!("the rules revision (r{} then, r{} now)", head.rules_revision, want.rules_revision));
        }
        if head.content_hash != want.content_hash {
            differs.push(format!("the content ({} then, {} now)", head.content_hash, want.content_hash));
        }
        // A changed spec changes the cell grid too: only the same spec over other
        // config hashes means an edited config file.
        if head.spec == want.spec && head.config_hashes != want.config_hashes {
            differs.push("a config file (edited since)".to_string());
        }
        return Err(format!(
            "{path} was written by another run: {} differs; start afresh without --resume",
            differs.join(", ")
        ));
    }
    let lines: Vec<&str> = lines.filter(|l| !l.trim().is_empty()).collect();
    let mut games = Vec::with_capacity(lines.len());
    for (i, l) in lines.iter().enumerate() {
        let mut g: GameResult = match serde_json::from_str(l) {
            Ok(g) => g,
            Err(_) if i + 1 == lines.len() => break,
            Err(e) => return Err(format!("{path}: line {}: {e}", i + 2)),
        };
        let cell = cells
            .get(g.cell)
            .ok_or_else(|| format!("{path}: line {}: no cell {}", i + 2, g.cell))?;
        match replay(&cell.rules, g.seed, &g.actions) {
            Ok(fin) => {
                let won = fin.outcome.as_ref().is_some_and(|o| o.won);
                if g.replay_ok && (won != g.won || fin.round != g.rounds) {
                    return Err(format!(
                        "{path}: line {}: the game replays to another end than it recorded",
                        i + 2
                    ));
                }
                g.final_state = Some(Box::new(fin));
            }
            // It failed its replay check when played, too: kept, and counted
            // as the mismatch it is, without a final table.
            Err(_) if !g.replay_ok => {}
            Err(e) => return Err(format!("{path}: line {}: replay: {e}", i + 2)),
        }
        games.push(g);
    }
    let mut seen = std::collections::BTreeSet::new();
    games.retain(|g| seen.insert((g.cell, g.seed)));
    Ok(games)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn json_in(dir: &Path) -> Vec<PathBuf> {
        let files = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path());
        files.filter(|p| p.extension().is_some_and(|e| e == "json")).collect()
    }

    /// 066: a spec names parameters and weights only `AgentSpec::parse` checks,
    /// so a rename strands a spec that nothing plays until someone runs it.
    #[test]
    fn every_live_spec_loads_and_its_agents_parse() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../experiments");
        let mut files = json_in(&root);
        for e in std::fs::read_dir(&root).unwrap().map(|e| e.unwrap().path()) {
            if e.is_dir() && e.file_name().is_some_and(|n| n.to_string_lossy().starts_with("gate-close")) {
                files.extend(json_in(&e));
            }
        }
        assert!(files.len() > 10, "{root:?}: {} specs", files.len());
        for f in files {
            let text = std::fs::read_to_string(&f).unwrap();
            let spec: Spec = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{f:?}: {e}"));
            for a in &spec.agents {
                AgentSpec::parse(a).unwrap_or_else(|e| panic!("{f:?}: {e}"));
            }
        }
    }

    /// 124: `sources` splits `gained` with nothing lost or made up, and a
    /// game's rounds after the opening show in `keeps`.
    #[test]
    fn sources_sum_to_gained_and_keeps_count_the_draft() {
        let config = format!("{}/../../configs/baseline.json", env!("CARGO_MANIFEST_DIR"));
        let spec: Spec = serde_json::from_value(serde_json::json!({
            "name": "t", "configs": [config], "agents": ["random"], "games": 1,
        }))
        .unwrap();
        let cs = cells(&spec).unwrap();
        let r = play_one(&spec, &cs, 0, 3);
        assert_eq!(r.start.len(), r.gained.len());
        assert_eq!(r.keeps.len(), r.gained.len());
        for (s, src) in r.sources.iter().enumerate() {
            let sum: Vec<i64> = (0..PEAKS.len()).map(|i| src.values().map(|v| v[i]).sum()).collect();
            assert_eq!(sum, r.gained[s], "seat {s}: {src:?}");
            assert!(src.values().all(|v| v.iter().any(|&x| x > 0)), "seat {s}: a zero row");
        }
        // 125: `uses` splits `spent` the same way, and start + gained - spent is the end.
        let end = r.final_state.as_ref().unwrap();
        for (s, u) in r.uses.iter().enumerate() {
            let sum: Vec<i64> = (0..PEAKS.len()).map(|i| u.values().map(|v| v[i]).sum()).collect();
            assert_eq!(sum, r.spent[s], "seat {s}: {u:?}");
            let held = holdings(end, s);
            for i in 0..PEAKS.len() {
                assert_eq!(r.start[s][i] + r.gained[s][i] - r.spent[s][i], held[i], "seat {s}, {}", PEAKS[i]);
            }
        }
        let drafted = r.actions.iter().filter(|a| matches!(a, Action::Draft { .. })).count() as u32;
        assert_eq!(r.keeps.iter().map(|k| k.offered).sum::<u32>(), drafted);
        if r.rounds > 1 {
            assert!(drafted > 0 && r.keeps.iter().all(|k| k.tiers >= k.tier2 && k.tiers > 0));
        }
    }
}
