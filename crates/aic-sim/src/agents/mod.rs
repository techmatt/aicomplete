//! Team agents for simulation: the skill ladder. One agent drives every seat,
//! with hands and drafts open (as at a table where the team talks it
//! through), and makes the "players choose" decisions too. Agents play
//! through the public API only (`Rules::legal_actions_into`, `apply_mut`,
//! reading the state) and, but for the planner and the oracle, see only what the
//! printed game shows: before any search they hide the decks' order and the dice
//! behind a guess of their own ([`determinize`]). The one exception (151): in the
//! middle of a resolution (a pending choice) the search scores the true table, since
//! a guess can change what the recorded answers replay through.
//!
//! Each agent has its own RNG, seeded from the game seed on a stream no
//! engine purpose uses, so a game still replays from seed + actions and
//! results do not depend on thread count or game order.
//!
//! The ladder, weakest first:
//!
//! - `random`: uniform over legal actions. A floor, not a model of a weak team.
//! - `greedy`: one action at a time, each scored by the heuristic after a
//!   short look ahead (`depth` actions, the choices an action asks for
//!   resolved best-first up to `choices` deep), averaged over `samples`
//!   guesses of the hidden cards.
//! - `beam`: beam search over the rest of the round's action sequences
//!   (`width` kept per step, at most `depth` steps), each scored by the
//!   heuristic at the start of the next round: the end-of-round preview.
//!   It plans once and follows the plan while the table matches it.
//! - `rollout`: the beam's `candidates` best end-of-round positions, each
//!   continued by `rollouts` greedy playouts for `horizon` more rounds (on
//!   fresh guesses of the hidden cards); the best mean wins. It looks across
//!   rounds where the beam stops at the round's end.
//! - `engine`: the heuristic's horizon mode ([`eval::Weights::engine`]):
//!   income and buildings are worth what they pay over the rounds the deck
//!   clock and the threat state leave, so an investment is scored by its
//!   payback to the expected end of the game, and a card in hand by whether
//!   it can be paid for in time. Played one action at a time (`width` 0,
//!   as greedy), or as a beam over the round (`width` > 0) with optional
//!   rollouts across rounds (`candidates` > 0); in the 008 sweep the beam
//!   and rollouts did worse. Built (008) to find out whether a building
//!   engine can beat greedy's no-building play.
//! - `strategist` (082): an alias, the engine with strategies (`strategy=1`),
//!   one action at a time with no search, and the deck clock off (157).
//!
//! - `planner` (028): the engine, trying a shortlist of alternatives in the
//!   action phase by rolling each out `horizon` rounds with the engine as
//!   the policy, on full information but for the Alignment deal, whose guesses
//!   it averages ([`planner`]).
//! - `oracle` (155): the planner's search on full information, the Alignment
//!   deal included, each line played to the game's end: a line that wins
//!   proves the seed winnable and is followed ([`oracle`]). A measuring
//!   stick for a skilled player, opt in.
//!
//! `greedy`, `engine` and `planner` keep by plan: [`policy::Policy`] wraps them, decides
//! their keeps, and applies the dominance checks' forced behaviors.
//!
//! An agent is named by a spec, `kind[:key=value,...]`: `greedy`,
//! `beam:width=4`, `rollout:rollouts=8,horizon=3`, and `w.<weight>=V` to
//! change a heuristic weight (see [`eval::Weights`]). `sweep` is a planner
//! profile, not a rung.

pub mod determinize;
pub mod eval;
pub mod guide;
pub mod moves;
pub mod oracle;
pub mod planner;
pub mod policy;
pub mod search;
pub mod strategy;
pub mod value;

use std::collections::BTreeMap;

use aic_core::content::{CardDef, TAGS};
use aic_core::{fnv1a64, Action, Agent, GameState, Rules};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub use search::{Beam, Greedy};

/// What a card's other costs count in credits: each antimatter this many...
const ANTIMATTER_COST: i64 = 3;
/// ...each insight this many...
pub(crate) const INSIGHT_COST: i64 = 2;
/// ...and each point of income lowered this many (energy and chips, 1).
const LOWERED_COST: i64 = 3;

/// A card's cost in credits, with `credits` as its credit price (printed,
/// or after a seat's discount).
pub(crate) fn credit_equivalent(d: &CardDef, credits: i64) -> i64 {
    credits
        + ANTIMATTER_COST * d.antimatter
        + 2 * d.research_cubes
        + d.energy
        + d.chips
        + INSIGHT_COST * d.insight
        + LOWERED_COST * d.lowers.iter().map(|(_, n)| n).sum::<i64>()
}

/// A card's printed price in credits, its other costs and lowered income at the
/// fixed rates above: the plan's measure of what it does. Not the core's
/// `cards::list_cost`, the printed credit price alone.
pub(crate) fn credit_cost(d: &CardDef) -> i64 {
    credit_equivalent(d, d.cost)
}

/// An agent that can be handed each seat's strategy (082): the search agents a
/// [`policy::Policy`] wraps.
pub trait Strategic: Agent {
    fn set_strategies(&mut self, _c: strategy::Choice) {}
    /// The agent's heuristic, which the keep plan reads for its rates
    /// (`Eval::plan_rates`, 102), its strategy multipliers (`Eval::mults`, 116)
    /// and `strat_contents` (124). `None` (random): the value table's rates and
    /// plain multipliers.
    fn eval(&self) -> Option<&eval::Eval> {
        None
    }
}

impl Strategic for RandomAgent {}

/// Uniform over legal actions. Seeded per game, so results do not depend on
/// thread count or game order. A floor, not a model of a weak player.
pub struct RandomAgent(ChaCha8Rng);

impl RandomAgent {
    pub fn new(game_seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(game_seed);
        // A stream no engine purpose uses, so the agent never mirrors the game's dice.
        rng.set_stream(0xa6e7_a6e7);
        RandomAgent(rng)
    }
}

impl Agent for RandomAgent {
    fn choose(&mut self, _rules: &Rules, _state: &GameState, legal: &[Action]) -> usize {
        (self.0.next_u64() % legal.len() as u64) as usize
    }
}

/// The agent's own RNG for one game: the game seed on a stream named after
/// the agent kind, apart from every engine purpose's stream.
pub fn agent_rng(game_seed: u64, kind: &str) -> ChaCha8Rng {
    let mut rng = ChaCha8Rng::seed_from_u64(game_seed);
    rng.set_stream(fnv1a64(format!("agent:{kind}").as_bytes()));
    rng
}

/// A parsed agent spec: `kind[:key=value,...]`.
#[derive(Debug, Clone, PartialEq)]
pub struct AgentSpec {
    pub kind: String,
    pub params: BTreeMap<String, f64>,
    pub weights: BTreeMap<String, f64>,
    /// 131: value-table numbers set for this agent (`t.<key>=v`, see [`value::Table::set`]).
    pub table: BTreeMap<String, f64>,
}

/// A budget parameter: name, default, what it controls.
pub type Param = (&'static str, f64, &'static str);

/// A tier's own parameters, then the ones `greedy`, `engine` and `planner` share (022):
/// keeping by plan, and the dominance checks' forced behaviors.
macro_rules! with_policy_params {
    ($($own:expr),* $(,)?) => {
        policy::forced_table!(policy_param_rows ($($own),*))
    };
}

/// The rows of [`policy::forced_table`] as parameters, after a tier's own.
macro_rules! policy_param_rows {
    (($($own:expr),*) $($(#[$doc:meta])* ($name:ident: $ty:ty, $active:literal, $default:expr, $desc:expr))*) => {
        &[$($own,)* $((stringify!($name), $default, $desc),)*]
    };
}

/// 174: the `discards` parameter's description (greedy, engine, planner, oracle).
/// 174 (r73): 0, each card of a discard chain valued by the table its own discard leaves. At 2 (as `choices`) a
/// chain step scored every set of three cards of the hand: 50-95% of a game, 5.8 x a decision; 0 holds every tier's
/// win % within about one SE on fresh seeds (`experiments/results/174_speed/`). 99: no cap.
const DISCARDS: &str = "levels a discard prompt (\"discard which card\") is resolved at, at most choices (174; 99: no cap)";

/// The skill tiers, weakest first, with each one's budget parameters and
/// their defaults.
pub const LADDER: &[(&str, &[Param])] = &[
    ("random", &[]),
    (
        "greedy",
        with_policy_params![
            (
                "samples",
                1.0,
                "guesses of the hidden cards averaged per decision",
            ),
            ("depth", 1.0, "actions looked ahead (1 = the action itself)"),
            (
                "choices",
                2.0,
                "levels of an action's own choices resolved best-first before scoring",
            ),
            ("discards", 0.0, DISCARDS),
        ],
    ),
    (
        "beam",
        &[
            ("width", 6.0, "sequences kept per step"),
            (
                "depth",
                120.0,
                "most steps searched (a round is about 25-45 decisions)",
            ),
            (
                "branch",
                0.0,
                "children kept per sequence, best first (0 = all)",
            ),
            (
                "choices",
                2.0,
                "levels of an action's own choices resolved best-first before scoring",
            ),
        ],
    ),
    (
        "rollout",
        &[
            ("width", 4.0, "beam width for the round's candidates"),
            ("depth", 120.0, "most beam steps"),
            ("branch", 0.0, "children kept per sequence (0 = all)"),
            ("choices", 2.0, "levels of an action's own choices resolved before scoring"),
            ("candidates", 3.0, "end-of-round positions rolled out"),
            ("rollouts", 4.0, "greedy playouts per candidate"),
            (
                "horizon",
                2.0,
                "rounds each playout looks past the current one",
            ),
        ],
    ),
    (
        "engine",
        with_policy_params![
            (
                "width",
                0.0,
                "beam width over the round (0 = no beam: one action at a time, as greedy; the 008 sweep's best)",
            ),
            ("depth", 120.0, "most beam steps"),
            ("branch", 0.0, "children kept per sequence (0 = all)"),
            ("choices", 2.0, "levels of an action's own choices resolved before scoring"),
            ("answers", 99.0, "width 0: prompts an answer to an open prompt looks on, at most choices (103)"),
            ("discards", 0.0, DISCARDS),
            ("samples", 1.0, "width 0: guesses of the hidden cards averaged per decision"),
            (
                "candidates",
                0.0,
                "end-of-round positions rolled out (0 = plain beam)",
            ),
            ("rollouts", 2.0, "greedy playouts per candidate"),
            (
                "horizon",
                1.0,
                "rounds each playout looks past the current one",
            ),
        ],
    ),
    (
        "planner",
        with_policy_params![
            ("horizon", 2.0, "rounds a rollout runs past the current one's start"),
            ("alts", 2.0, "alternatives rolled out per decision"),
            ("budget", 8.0, "alternatives rolled out per round"),
            ("margin", 40.0, "only actions within this of the engine's best score are tried"),
            ("choices", 2.0, "levels of an action's own choices resolved before scoring"),
            ("samples", 1.0, "guesses of the hidden cards averaged per decision"),
            ("prune", 12.0, "actions each seat keeps through a phase of a rollout (0: all)"),
            ("powers", 6.0, "with prune, powers each seat keeps through a phase of a rollout (069)"),
            ("answers", 99.0, "prompts an answer looks on inside a rollout, at most choices (069)"),
            ("discards", 0.0, DISCARDS),
            ("deals", 4.0, "guesses of the Alignment deal a line's value is averaged over"),
            ("lead", 0.0, "what an alternative must lead the line to beat by, on the first deal and on the mean"),
        ],
    ),
    (
        "oracle",
        with_policy_params![
            ("horizon", 2.0, "rounds on at which two lines that do not win are compared"),
            ("alts", 2.0, "alternatives played to the end per decision"),
            ("budget", 8.0, "alternatives played to the end per round"),
            ("margin", 40.0, "only actions within this of the engine's best score are tried"),
            ("reseeds", 0.0, "lines on other policy seeds at a round's first searched decision, while no win is found"),
            ("choices", 2.0, "levels of an action's own choices resolved before scoring"),
            ("samples", 1.0, "guesses of the hidden cards averaged per decision"),
            ("prune", 12.0, "actions each seat keeps through a phase of a rollout (0: all)"),
            ("powers", 6.0, "with prune, powers each seat keeps through a phase of a rollout"),
            ("answers", 99.0, "prompts an answer looks on inside a rollout, at most choices"),
            ("discards", 0.0, DISCARDS),
            ("threads", 1.0, "lines played side by side in one decision (the same play at any count)"),
        ],
    ),
];

/// `sweep`: the planner on one guess, for price sweeps, not the default (069):
/// 1.5 x faster, and card rankings hold (`experiments/results/screen-069-speed`).
const SWEEP: &str = "deals=1,lead=0";

/// `strategist` (082): the engine with strategies, one action at a time and no
/// search: the greedy-with-strategy tier. 157 (r68): the deck clock off
/// (`w.clock` 0; the engine and planner keep it): it charged each round the team's
/// Unity rate left it short of the purge's last step, which bought Unity tips over
/// liberations. Dev seeds 36800-36831, 96 games a level: d1-d3 70.8 / 62.5 / 42.7
/// on, 74.0 / 69.8 / 49.0 off.
/// 162 (r69): greedy's flat weights with strategy, and the purge term (`w.purge` 3.6):
/// horizon mode valued an ambassador at 2.4 against 3.6 for the credits that buy one,
/// so the old judge bought 0.7-0.9 a game and freed 2.6-3.0 cities to greedy's 4.3-5.1.
/// Dev seeds 36800-36831 (screen in `experiments/results/162_judge_purge`); the old
/// judge is `engine:strategy=1,w.clock=0`.
const STRATEGIST: &str = "strategy=1,w.built=4,w.built_cost=0.5,w.clock=0,w.energy_cap=0,w.hand=3,w.hand_cost=0.4,\
w.horizon=0,w.insight_income=0,w.movement=0,w.packet_keep=1,w.production=0,w.strat_contents=0,w.table=0,\
w.tag_hand=0,w.purge=3.6";

/// A kind's place on the [`LADDER`].
fn rung(kind: &str) -> Option<usize> {
    LADDER.iter().position(|(k, _)| *k == kind)
}

impl AgentSpec {
    pub fn parse(spec: &str) -> Result<AgentSpec, String> {
        let (kind, rest) = spec.split_once(':').unwrap_or((spec, ""));
        let swept;
        let (kind, rest) = if kind == "sweep" {
            swept = format!("{SWEEP},{rest}");
            ("planner", swept.as_str())
        } else if kind == "strategist" {
            swept = format!("{STRATEGIST},{rest}");
            ("engine", swept.as_str())
        } else {
            (kind, rest)
        };
        let Some(rung) = rung(kind) else {
            let names: Vec<&str> = LADDER.iter().map(|(k, _)| *k).collect();
            return Err(format!(
                "unknown agent {kind:?}; known: {}, or the aliases strategist, sweep",
                names.join(", ")
            ));
        };
        let known = LADDER[rung].1;
        let mut params: BTreeMap<String, f64> =
            known.iter().map(|(k, v, _)| (k.to_string(), *v)).collect();
        let mut weights = BTreeMap::new();
        let mut table = BTreeMap::new();
        for kv in rest.split(',').filter(|s| !s.is_empty()) {
            let (k, v) = kv
                .split_once('=')
                .ok_or_else(|| format!("agent {spec:?}: {kv:?} is not key=value"))?;
            let v: f64 = v
                .parse()
                .map_err(|_| format!("agent {spec:?}: {k}: not a number: {v:?}"))?;
            if let Some(w) = k.strip_prefix("w.") {
                if let Some((_, why)) = eval::RETIRED.iter().find(|(name, _)| *name == w) {
                    return Err(format!("agent {spec:?}: weight {w:?} is retired (165): {why}"));
                }
                if kind == "random" || !eval::Weights::default().set(w, v) {
                    return Err(format!("agent {spec:?}: no heuristic weight {w:?}"));
                }
                weights.insert(w.to_string(), v);
            } else if let Some(t) = k.strip_prefix("t.") {
                if kind == "random" {
                    return Err(format!("agent {spec:?}: random reads no value table"));
                }
                table.insert(t.to_string(), v);
            } else if params.contains_key(k) {
                if v < 0.0 || !v.is_finite() {
                    return Err(format!("agent {spec:?}: {k} must be >= 0"));
                }
                // Must be at least 1, so `build_search` plays the number as given: say
                // so rather than label a cell "0" (020).
                // (`engine`'s width and candidates 0 mean "no beam" and "no rollouts",
                // so those two only for beam and rollout.)
                let at_least_one = (k == "horizon" && (kind == "planner" || kind == "oracle"))
                    || ["samples", "depth", "rollouts", "deals", "threads"].contains(&k)
                    || (k == "width" && (kind == "beam" || kind == "rollout"))
                    || (k == "candidates" && kind == "rollout");
                if v < 1.0 && at_least_one {
                    return Err(format!("agent {spec:?}: {k} must be >= 1"));
                }
                // 0 is off; anything else falling off the list would be played
                // as the plain agent under a dominance check's label.
                let tag = k == "only_tag" || k == "never_tag";
                if tag && (v.fract() != 0.0 || v > TAGS.len() as f64) {
                    return Err(format!(
                        "agent {spec:?}: {k} must be a whole number from 0 to {}",
                        TAGS.len()
                    ));
                }
                params.insert(k.to_string(), v);
            } else {
                let names: Vec<&str> = known.iter().map(|(k, _, _)| *k).collect();
                return Err(format!(
                    "agent {spec:?}: {kind} has no parameter {k:?}; it has: {}",
                    if names.is_empty() {
                        "none".to_string()
                    } else {
                        names.join(", ")
                    }
                ));
            }
        }
        eval::check_names(weights.keys().map(String::as_str)).map_err(|e| format!("agent {spec:?}: {e}"))?;
        value::table_with(&table).map_err(|e| format!("agent {spec:?}: {e}"))?;
        if kind == "engine" {
            // What the chosen mode never reads: set, it would only be a label.
            let on = |k: &str| params[k].round() > 0.0;
            let mut unread = vec!["samples", "answers", "discards"];
            if !on("width") {
                unread = vec!["depth", "branch", "candidates", "rollouts", "horizon"];
            } else if !on("candidates") {
                unread.extend(["rollouts", "horizon"]);
            }
            let set = known
                .iter()
                .find(|(k, d, _)| unread.contains(k) && params[*k] != *d);
            if let Some((k, _, _)) = set {
                return Err(format!("agent {spec:?}: this engine mode ignores {k}"));
            }
        }
        Ok(AgentSpec {
            kind: kind.to_string(),
            params,
            weights,
            table,
        })
    }

    /// 131: the value table this agent reads: the table with its `t.` numbers set.
    pub fn table(&self) -> &'static value::Table {
        value::table_with(&self.table).expect("checked in parse")
    }

    /// The canonical spec: the kind, then every parameter that differs from
    /// its default, then weights.
    pub fn label(&self) -> String {
        let known = LADDER[self.tier()].1;
        let mut parts: Vec<String> = known
            .iter()
            .filter(|(k, d, _)| self.params[*k] != *d)
            .map(|(k, _, _)| format!("{k}={}", self.params[*k]))
            .collect();
        parts.extend(self.weights.iter().map(|(k, v)| format!("w.{k}={v}")));
        parts.extend(self.table.iter().map(|(k, v)| format!("t.{k}={v}")));
        if parts.is_empty() {
            self.kind.clone()
        } else {
            format!("{}:{}", self.kind, parts.join(","))
        }
    }

    /// Position on the ladder (0 = random).
    pub fn tier(&self) -> usize {
        rung(&self.kind).expect("agent kind is checked in parse")
    }

    /// A shallow agent: one action at a time on one guess (random, greedy
    /// and engine at their defaults), not a beam, rollouts, more guesses or
    /// a deeper look-ahead; and the planner, whatever its budget. The scoping
    /// runs' agents (`--quick`): deep search waits until the rules are close
    /// to final.
    pub fn shallow(&self) -> bool {
        match self.kind.as_str() {
            "random" => true,
            "greedy" => self.p("samples") <= 1 && self.p("depth") <= 1,
            "engine" => self.p("width") == 0 && self.p("samples") <= 1,
            // 028: built to be run on screens, whatever the rules' stage.
            "planner" => true,
            // 155: a measuring stick, run deliberately.
            "oracle" => false,
            _ => false,
        }
    }

    fn p(&self, k: &str) -> usize {
        self.params[k].round() as usize
    }

    fn weights(&self) -> eval::Weights {
        let mut w = if ["engine", "planner", "oracle"].contains(&self.kind.as_str()) {
            eval::Weights::engine()
        } else {
            eval::Weights::default()
        };
        for (k, v) in &self.weights {
            w.set(k, *v);
        }
        w
    }

    /// The forced behaviors (022): keeping by plan and the dominance checks.
    pub fn forced(&self) -> policy::Forced {
        // An agent without these parameters is never wrapped, so its numbers
        // are read by nothing.
        policy::Forced::read(|k| self.params.get(k).copied().unwrap_or(0.0))
    }

    /// A fresh agent for one game: the search agent, wrapped in its keep
    /// policy and forced behaviors when it has any (022).
    pub fn build(&self, rules: &Rules, game_seed: u64) -> Box<dyn Agent + Send> {
        let inner = self.build_search(rules, game_seed);
        let forced = self.forced();
        if !forced.is_active() {
            return inner as Box<dyn Agent + Send>;
        }
        Box::new(policy::Policy::new(inner, forced))
    }

    fn build_search(&self, rules: &Rules, game_seed: u64) -> Box<dyn Strategic + Send> {
        if self.kind == "random" {
            return Box::new(RandomAgent::new(game_seed));
        }
        let rng = agent_rng(game_seed, &self.kind);
        let ev = eval::Eval::with_table(rules, self.weights(), self.table());
        match self.kind.as_str() {
            "greedy" => Box::new(
                Greedy::new(ev, rng, self.p("samples"), self.p("depth"), self.p("choices")).discards(self.p("discards")),
            ),
            "beam" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width"),
                self.p("depth"),
                self.p("branch"),
                self.p("choices"),
                None,
            )),
            "rollout" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width"),
                self.p("depth"),
                self.p("branch"),
                self.p("choices"),
                Some(search::RolloutCfg {
                    candidates: self.p("candidates"),
                    rollouts: self.p("rollouts"),
                    horizon: self.p("horizon"),
                }),
            )),
            "engine" if self.p("width") == 0 => Box::new(
                Greedy::new(ev, rng, self.p("samples"), 1, self.p("choices"))
                    .answers(self.p("answers"))
                    .discards(self.p("discards")),
            ),
            "engine" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width"),
                self.p("depth"),
                self.p("branch"),
                self.p("choices"),
                (self.p("candidates") > 0).then(|| search::RolloutCfg {
                    candidates: self.p("candidates"),
                    rollouts: self.p("rollouts"),
                    horizon: self.p("horizon"),
                }),
            )),
            "planner" => Box::new(planner::Planner::new(
                ev,
                self.forced(),
                rng,
                planner::PlannerCfg {
                    horizon: self.p("horizon") as u32,
                    alts: self.p("alts"),
                    budget: self.p("budget"),
                    margin: self.params["margin"],
                    choices: self.p("choices"),
                    samples: self.p("samples"),
                    prune: self.p("prune"),
                    powers: self.p("powers"),
                    answers: self.p("answers"),
                    discards: self.p("discards"),
                    deals: self.p("deals"),
                    lead: self.params["lead"],
                },
            )),
            "oracle" => Box::new(oracle::Oracle::new(
                ev,
                self.forced(),
                rng,
                oracle::OracleCfg {
                    horizon: self.p("horizon") as u32,
                    alts: self.p("alts"),
                    budget: self.p("budget"),
                    margin: self.params["margin"],
                    reseeds: self.p("reseeds"),
                    choices: self.p("choices"),
                    samples: self.p("samples"),
                    prune: self.p("prune"),
                    powers: self.p("powers"),
                    answers: self.p("answers"),
                    discards: self.p("discards"),
                    threads: self.p("threads"),
                },
            )),
            k => unreachable!("agent kind {k} is checked in parse"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aic_core::{play, replay, Config};

    /// `sweep` is the planner at its profile, and takes what the planner takes.
    #[test]
    fn sweep_is_a_planner_profile() {
        let named = |spec: &str| AgentSpec::parse(spec).unwrap().params;
        assert_eq!(named("strategist"), named("engine:strategy=1"));
        assert_eq!(named("sweep"), named("planner:deals=1,lead=0"));
        assert_eq!(named("sweep:budget=4"), named("planner:deals=1,lead=0,budget=4"));
        assert_eq!(named("sweep:deals=2"), named("planner:deals=2,lead=0"));
    }

    /// 131: `t.<key>=v` sets one value-table number for one agent: none is the
    /// table itself, a name in two sections or none is refused, the label keeps it.
    #[test]
    fn table_numbers_set_per_agent() {
        let spec = |s: &str| AgentSpec::parse(s);
        let base = value::table();
        assert!(std::ptr::eq(spec("strategist").unwrap().table(), base));
        let a = spec("engine:strategy=1,w.clock=0,t.chips=2.25,t.auto_counter=1.2").unwrap();
        let t = a.table();
        assert_eq!(t.res(aic_core::content::Resource::Chips), 2.25);
        assert_eq!(t.op("auto_counter"), 1.2);
        assert_eq!(base.res(aic_core::content::Resource::Chips), value::table().res(aic_core::content::Resource::Chips));
        assert!(std::ptr::eq(spec("greedy:t.auto_counter=1.2,t.chips=2.25").unwrap().table(), t), "one table a set");
        assert_eq!(a.label(), "engine:strategy=1,w.clock=0,t.auto_counter=1.2,t.chips=2.25");
        assert_eq!(AgentSpec::parse(&a.label()).unwrap(), a);
        assert_eq!(spec("greedy:t.rates.tag_count=3").unwrap().table().rate("tag_count"), 3.0);
        assert!(spec("greedy:t.no_such_rate=1").is_err());
        assert!(spec("random:t.chips=2").is_err());
    }

    /// An agent that takes the first legal action.
    pub(crate) struct First;

    impl aic_core::Agent for First {
        fn choose(&mut self, _: &Rules, _: &aic_core::GameState, _: &[aic_core::Action]) -> usize {
            0
        }
    }

    impl Strategic for First {}

    pub(crate) fn rules(difficulty: i64) -> Rules {
        config_rules("baseline", difficulty)
    }

    /// Rules for `configs/<name>.json` at `difficulty`.
    pub(crate) fn config_rules(name: &str, difficulty: i64) -> Rules {
        config_rules_with(name, difficulty, &[])
    }

    /// [`config_rules`] with `overrides` on top.
    pub(crate) fn config_rules_with(name: &str, difficulty: i64, overrides: &[&str]) -> Rules {
        let text = std::fs::read_to_string(format!(
            "{}/../../configs/{name}.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let mut c = Config::from_json(&text).unwrap();
        c.apply_override(&format!("adversary.difficulty={difficulty}"))
            .unwrap();
        for o in overrides {
            c.apply_override(o).unwrap();
        }
        Rules::new(&c).unwrap()
    }

    const GREEDY: &str = "greedy";
    const BEAM: &str = "beam:width=2";
    const ROLLOUT: &str = "rollout:width=2,candidates=2,rollouts=1,horizon=1";
    // The engine's beam is beam's code with another heuristic.
    const ENGINE: &str = "engine";
    const PLANNER: &str = "planner:budget=2";
    const ORACLE: &str = "oracle:budget=2,reseeds=1";

    /// The agent plays seed 3 at difficulty 0 for up to `max_actions`; the
    /// game replays from seed + actions, and the same agent on the same seed
    /// plays the same actions: its RNG is its own and seeded per game.
    /// (014: not difficulty 4, where the adversary's supply runs out in the
    /// first build step, before any decision.)
    fn replay_and_repeat(spec: &str, max_actions: usize, to_the_end: bool) {
        let r = rules(0);
        let spec = AgentSpec::parse(spec).unwrap();
        let seed = 3;
        let (g, rec) = play(&r, seed, &mut *spec.build(&r, seed), max_actions);
        if to_the_end {
            assert!(g.is_over(), "{}", spec.label());
        } else {
            assert_eq!(rec.actions.len(), max_actions, "{}", spec.label());
        }
        assert_eq!(replay(&r, seed, &rec.actions).unwrap(), g);
        let (_, again) = play(&r, seed, &mut *spec.build(&r, seed), max_actions);
        assert_eq!(rec.actions, again.actions, "{}", spec.label());
    }

    /// Fast lane: the first 30 actions, which take every agent into round 2
    /// (whole games are 200–300 actions). One test an agent, so they run side by side.
    #[test]
    fn greedy_replays_and_repeats_opening() {
        replay_and_repeat(GREEDY, 30, false);
    }

    #[test]
    fn beam_replays_and_repeats_opening() {
        replay_and_repeat(BEAM, 30, false);
    }

    #[test]
    fn rollout_replays_and_repeats_opening() {
        replay_and_repeat(ROLLOUT, 30, false);
    }

    #[test]
    fn engine_replays_and_repeats_opening() {
        replay_and_repeat(ENGINE, 30, false);
    }

    #[test]
    fn planner_replays_and_repeats_opening() {
        replay_and_repeat(PLANNER, 30, false);
    }

    #[test]
    fn oracle_replays_and_repeats_opening() {
        replay_and_repeat(ORACLE, 30, false);
    }

    /// 022: the dominance checks parse, label and replay (into round 3,
    /// past two keeps). Split in five so the parts run side by side (the two
    /// planner specs, the slowest, fall in different parts).
    fn dominance_checks(part: usize) {
        for spec in DOMINANCE.iter().skip(part).step_by(5) {
            assert_eq!(AgentSpec::parse(spec).unwrap().label(), *spec);
            replay_and_repeat(spec, 60, false);
        }
    }

    #[test]
    fn dominance_checks_replay_and_repeat_0() {
        dominance_checks(0);
        assert!(AgentSpec::parse("random:keep_all=1").is_err());
    }

    #[test]
    fn dominance_checks_replay_and_repeat_1() {
        dominance_checks(1);
    }

    #[test]
    fn dominance_checks_replay_and_repeat_2() {
        dominance_checks(2);
    }

    #[test]
    fn dominance_checks_replay_and_repeat_3() {
        dominance_checks(3);
    }

    #[test]
    fn dominance_checks_replay_and_repeat_4() {
        dominance_checks(4);
    }

    const DOMINANCE: [&str; 20] = [
            "engine:keep_all=1",
            "engine:keep_k=2",
            "engine:tier2_rush=1",
            "greedy:no_node=1",
            "engine:only_tag=3",
            "engine:never_tag=6",
            "engine:events_only=1",
            "engine:ignore_threats=1",
            "engine:home_only=1",
            "engine:capstone_rush=1",
            "engine:turtle=1",
            "engine:free_only=1",
            "engine:abandon=1",
            "engine:keep_none=1",
            "engine:keep_random=1",
            "planner:budget=2,never_tag=2",
            // 082: strategies, chosen and fixed, and the pre-082 terms.
            "engine:strategy=1",
            "engine:strat0=2,strat1=3",
            "planner:budget=2,strategy=1",
            "engine:plan_table=0,w.table=0",
    ];

    // Slow lane: whole games, one test per agent so they run side by side
    // (014 board, 015 timings, each run alone: beam ~18 s, engine ~3 s, greedy
    // ~1 s; the beam's cost is in the action phase's wide menus. `rollout` does
    // a full beam search per decision: ~92 s a game at c2, ~373 s at c3, r17, 030.)
    #[test]
    #[ignore = "slow lane: a whole search-agent game"]
    fn greedy_replays_and_repeats() {
        replay_and_repeat(GREEDY, 3000, true);
    }

    #[test]
    #[ignore = "slow lane: a whole search-agent game"]
    fn beam_replays_and_repeats() {
        replay_and_repeat(BEAM, 3000, true);
    }

    #[test]
    #[ignore = "slow lane: a whole search-agent game"]
    fn rollout_replays_and_repeats() {
        replay_and_repeat(ROLLOUT, 3000, true);
    }

    #[test]
    #[ignore = "slow lane: a whole search-agent game"]
    fn engine_replays_and_repeats() {
        replay_and_repeat(ENGINE, 3000, true);
    }

    #[test]
    #[ignore = "slow lane: a whole search-agent game"]
    fn planner_replays_and_repeats() {
        replay_and_repeat(PLANNER, 3000, true);
    }

    #[test]
    #[ignore = "slow lane: a whole search-agent game"]
    fn oracle_replays_and_repeats() {
        replay_and_repeat(ORACLE, 3000, true);
    }

    /// An agent's decision is the same on two tables that differ only in
    /// what the printed game hides (deck order and the dice).
    #[test]
    fn decisions_ignore_hidden_information() {
        let r = rules(0);
        let mut legal = Vec::new();
        for spec in ["greedy", "beam:width=2", "engine", "engine:width=2", "strategist"] {
            let spec = AgentSpec::parse(spec).unwrap();
            for seed in 0..3 {
                // Some way into the game, so piles have moved.
                let (mut a, _) = play(&r, seed, &mut RandomAgent::new(seed), 12);
                if a.is_over() {
                    a = r.new_game(seed);
                }
                let mut b = a.clone();
                for (k, p) in b.piles.iter_mut() {
                    if k.starts_with("cards.deck.") || k.starts_with("threats.deck.") {
                        std::sync::Arc::make_mut(p).reverse();
                    }
                }
                for (purpose, s) in b.rng.iter_mut() {
                    *s = std::sync::Arc::new(aic_core::rng::Stream::new(seed + 77, purpose));
                }
                r.legal_actions_into(&a, &mut legal);
                let ia = spec.build(&r, 9).choose(&r, &a, &legal);
                let ib = spec.build(&r, 9).choose(&r, &b, &legal);
                assert_eq!(ia, ib, "{} seed {seed}", spec.label());
            }
        }
    }

    /// D20 with sampled deals (054): two tables that differ only in which
    /// Unity cards were dealt give the same decision, deep in a line's forks.
    #[test]
    fn sampled_deals_ignore_the_true_deal() {
        let r = rules(2);
        let key = "alignment.deck.unity";
        let spec = AgentSpec::parse("planner:budget=2,horizon=1,deals=3").unwrap();
        let mut legal = Vec::new();
        for seed in 0..2 {
            let a = r.new_game(seed);
            let printed: Vec<aic_core::Key> = r
                .world()
                .content
                .alignment
                .iter()
                .filter(|c| c.side == aic_core::content::Side::Unity)
                .map(|c| aic_core::Key::new(&c.id))
                .collect();
            let other: Vec<aic_core::Key> =
                printed.iter().filter(|id| !a.pile(key).contains(id)).cloned().collect();
            let mut b = a.clone();
            b.piles.insert(key.into(), std::sync::Arc::new(other));
            r.legal_actions_into(&a, &mut legal);
            let ia = spec.build(&r, 9).choose(&r, &a, &legal);
            let ib = spec.build(&r, 9).choose(&r, &b, &legal);
            assert_eq!(ia, ib, "seed {seed}");
        }
    }

    #[test]
    fn specs() {
        let s = AgentSpec::parse("beam:width=4,w.corrupted=20").unwrap();
        assert_eq!(s.label(), "beam:width=4,w.corrupted=20");
        assert!(AgentSpec::parse("greedy:w.pool=3").is_err());
        assert_eq!(AgentSpec::parse("greedy").unwrap().label(), "greedy");
        assert!(AgentSpec::parse("greedy:width=3").is_err());
        assert!(AgentSpec::parse("greedy:w.nope=3").is_err());
        // 165: a retired probe is refused with its reason.
        for (name, _) in eval::RETIRED {
            let e = AgentSpec::parse(&format!("engine:w.{name}=1")).unwrap_err();
            assert!(e.contains("retired"), "{e}");
            assert!(!eval::Weights::default().set(name, 1.0), "{name} is still a weight");
        }
        assert!(AgentSpec::parse("mcts").is_err());
        assert!(AgentSpec::parse("random:samples=2").is_err());
        // 020: a 0 that would be played as 1 is refused; engine's width 0 (no beam) is not.
        assert!(AgentSpec::parse("greedy:samples=0").is_err());
        assert!(AgentSpec::parse("beam:width=0").is_err());
        assert!(AgentSpec::parse("engine:width=0").is_ok());
        assert!(AgentSpec::parse("rollout:rollouts=0").is_err());
        assert!(AgentSpec::parse("rollout:candidates=0").is_err());
        assert!(AgentSpec::parse("engine:rollouts=0").is_err());
        // A tag off the list would be the plain agent under a check's label.
        assert!(AgentSpec::parse("engine:only_tag=7").is_err());
        assert!(AgentSpec::parse("engine:never_tag=1.5").is_err());
        assert!(AgentSpec::parse("engine:only_tag=6").is_ok());
        // What the engine's mode never reads.
        assert!(AgentSpec::parse("engine:depth=5").is_err());
        assert!(AgentSpec::parse("engine:width=4,samples=2").is_err());
        assert!(AgentSpec::parse("engine:width=4,rollouts=3").is_err());
        assert!(AgentSpec::parse("engine:width=4,candidates=2,rollouts=3").is_ok());
    }

    /// The deck clock (011; 013 on Unity; 156 on the purge's last step):
    /// `w.clock` per round the team, at its Unity rate so far, would still
    /// be short of its Unity target when the deck runs out; nothing while it
    /// is on course.
    #[test]
    fn deck_clock_penalizes_a_team_behind_the_deck() {
        // 080: the Wall pressure's landing may not ask where. 137: round 1's
        // card may ask where its troops land: a seed whose opening asks nothing.
        let r = config_rules_with("baseline", 0, &["adversary.wall_troops=0"]);
        let mut g = (1..64).map(|s| r.new_game(s)).find(|g| g.pending.is_none()).unwrap();
        let value = |clock: f64, g: &aic_core::GameState| {
            let w = eval::Weights { clock, ..Default::default() };
            eval::Eval::new(&r, w).value(&r, g)
        };
        // Round 4, one Unity tip (6 Sway: 1.5 a round), 6 tips (36 Sway, 156:
        // the purge's last step, `purge_at`) for the target: 20 rounds
        // needed. Two cards left (the face-up one and one in the deck): 2
        // action phases after this one. 18 short.
        g.round = 4;
        g.vars.insert("alignment.unity_tips".into(), 1);
        assert_eq!(g.pile("adversary.next").len(), 1);
        let deck = g.pile("adversary.deck").to_vec();
        g.piles.insert("adversary.deck".into(), deck[..1].to_vec().into());
        assert!((value(0.0, &g) - value(2.0, &g) - 2.0 * 18.0).abs() < 1e-9);
        // On course (7 tips by round 4): no cost.
        g.vars.insert("alignment.unity_tips".into(), 7);
        assert_eq!(value(0.0, &g), value(2.0, &g));
        // The engine watches the deck by default; greedy doesn't (its
        // games, the difficulty calibration, are unchanged).
        assert!(eval::Weights::engine().clock > 0.0);
        assert_eq!(eval::Weights::default().clock, 0.0);
    }
}
