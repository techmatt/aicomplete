//! Team agents for simulation: the skill ladder. One agent drives every seat,
//! with hands and drafts open (as at a table where the team talks it
//! through), and makes the "players choose" decisions too. Agents play
//! through the public API only (`Rules::legal_actions_into`, `apply_mut`,
//! reading the state) and, but for the planner, see only what the printed
//! game shows: before any search they hide the decks' order and the dice
//! behind a guess of their own ([`determinize`]).
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
//!
//! - `planner` (028): the engine, trying a shortlist of alternatives in the
//!   action phase by rolling each out `horizon` rounds with the engine as
//!   the policy, on full information ([`planner`]).
//!
//! `greedy`, `engine` and `planner` keep by plan: [`policy::Policy`] wraps them, decides
//! their keeps, and applies the dominance checks' forced behaviours.
//!
//! An agent is named by a spec, `kind[:key=value,...]`: `greedy`,
//! `beam:width=4`, `rollout:rollouts=8,horizon=3`, and `w.<weight>=V` to
//! change a heuristic weight (see [`eval::Weights`]).

pub mod determinize;
pub mod eval;
pub mod planner;
pub mod policy;
pub mod search;

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
        + d.energy
        + d.chips
        + INSIGHT_COST * d.insight
        + LOWERED_COST * d.lowers.iter().map(|(_, n)| n).sum::<i64>()
}

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
}

/// A budget parameter: name, default, what it controls.
pub type Param = (&'static str, f64, &'static str);

/// A tier's own parameters, then the ones `greedy`, `engine` and `planner` share (022):
/// keeping by plan, and the dominance checks' forced behaviours.
macro_rules! with_policy_params {
    ($($own:expr),* $(,)?) => {
        &[
            $($own,)*
            ("plan_keep", 1.0, "keep by a sketch of the next rounds (0: the heuristic keeps)"),
            ("plan_rounds", 4.0, "rounds the keep plan sketches, this one included"),
            ("plan_reserve", 0.0, "insight the keep plan holds back in each later round for that round's keeps"),
            ("plan_hand", 0.0, "keep nothing while the hand holds this many cards (0: off)"),
            ("plan_t2", 1.0, "share of a tier-2 card's value the keep plan counts"),
            ("plan_credit", 3.0, "what a credit paid for a keep costs the keep plan"),
            ("plan_niche", 3.0, "credits the keep plan adds per marker a card would clear now, threat it would delay and presence it would shield"),
            ("keep_all", 0.0, "dominance check: keep every drafted card"),
            ("keep_k", 0.0, "dominance check: keep only the k cheapest drafted cards (0: off)"),
            ("keep_none", 0.0, "sanity check (028): keep no drafted card"),
            ("keep_random", 0.0, "sanity check (028): keep each drafted card on a coin toss"),
            ("tier1_only", 0.0, "dominance check: never draft from tier 2"),
            ("tier2_rush", 0.0, "dominance check: draft from tier 2 whenever the keep is affordable"),
            ("no_powers", 0.0, "dominance check: never use an AI-board power"),
            ("no_counters", 0.0, "dominance check: never counter a threat"),
            ("no_presence", 0.0, "dominance check: place no presence beyond setup"),
            ("only_tag", 0.0, "dominance check (023): keep and play only this tag (1-6: science, industry, cyber, power, diplomacy, security)"),
            ("never_tag", 0.0, "dominance check (023): never keep or play this tag (1-6 as only_tag)"),
            ("events_only", 0.0, "dominance check (023): keep and play only events"),
            ("ignore_threats", 0.0, "exploit check (027): no counters, no card that counters a threat"),
            ("turtle", 0.0, "exploit check (027): no presence beyond setup; only remove, clear, chips, ambassador and hardening actions"),
            ("free_only", 0.0, "exploit check (027): only Diplomacy cards and the ambassador, move, remove, clear and chips actions"),
            ("unity_farm", 0.0, "exploit check (028): before anything else, buy ambassadors and land them on bases and factories"),
            ("rich_spend", 0.0, "exploit check (034): every spare credit on the rich action"),
            ("science_rush", 0.0, "exploit check (034): draft, keep and play Science first, for its discounts"),
            ("always_expand", 0.0, "fixed policy (041): the growth option that places the most presence"),
            ("always_consolidate", 0.0, "fixed policy (041): the growth option that places none"),
            ("never_remove", 0.0, "fixed policy (041): never clear a corruption marker"),
            ("science_board", 0.0, "fixed policy (041): the Science rush, board cards first"),
        ]
    };
}

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
            ("prune", 6.0, "actions each seat keeps through a phase of a rollout (0: all)"),
        ],
    ),
];

impl AgentSpec {
    pub fn parse(spec: &str) -> Result<AgentSpec, String> {
        let (kind, rest) = spec.split_once(':').unwrap_or((spec, ""));
        let Some((_, known)) = LADDER.iter().find(|(k, _)| *k == kind) else {
            let names: Vec<&str> = LADDER.iter().map(|(k, _)| *k).collect();
            return Err(format!(
                "unknown agent {kind:?}; known: {}",
                names.join(", ")
            ));
        };
        let mut params: BTreeMap<String, f64> =
            known.iter().map(|(k, v, _)| (k.to_string(), *v)).collect();
        let mut weights = BTreeMap::new();
        for kv in rest.split(',').filter(|s| !s.is_empty()) {
            let (k, v) = kv
                .split_once('=')
                .ok_or_else(|| format!("agent {spec:?}: {kv:?} is not key=value"))?;
            let v: f64 = v
                .parse()
                .map_err(|_| format!("agent {spec:?}: {k}: not a number: {v:?}"))?;
            if let Some(w) = k.strip_prefix("w.") {
                if kind == "random" || !eval::Weights::default().set(w, v) {
                    return Err(format!("agent {spec:?}: no heuristic weight {w:?}"));
                }
                weights.insert(w.to_string(), v);
            } else if params.contains_key(k) {
                if v < 0.0 || !v.is_finite() {
                    return Err(format!("agent {spec:?}: {k} must be >= 0"));
                }
                // Played as at least 1: say so rather than label a cell "0" (020).
                // (`engine`'s width and candidates 0 mean "no beam" and "no rollouts",
                // so those two only for beam and rollout.)
                let at_least_one = (k == "horizon" && kind == "planner")
                    || ["samples", "depth", "rollouts"].contains(&k)
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
        if kind == "engine" {
            // What the chosen mode never reads: set, it would only be a label.
            let on = |k: &str| params[k].round() > 0.0;
            let mut unread = vec!["samples"];
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
        })
    }

    /// The canonical spec: the kind, then every parameter that differs from
    /// its default, then weights.
    pub fn label(&self) -> String {
        let known = LADDER
            .iter()
            .find(|(k, _)| *k == self.kind)
            .map_or(&[][..], |l| l.1);
        let mut parts: Vec<String> = known
            .iter()
            .filter(|(k, d, _)| self.params[*k] != *d)
            .map(|(k, _, _)| format!("{k}={}", self.params[*k]))
            .collect();
        parts.extend(self.weights.iter().map(|(k, v)| format!("w.{k}={v}")));
        if parts.is_empty() {
            self.kind.clone()
        } else {
            format!("{}:{}", self.kind, parts.join(","))
        }
    }

    /// Position on the ladder (0 = random).
    pub fn tier(&self) -> usize {
        LADDER
            .iter()
            .position(|(k, _)| *k == self.kind)
            .unwrap_or(0)
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
            _ => false,
        }
    }

    fn p(&self, k: &str) -> usize {
        self.params[k].round() as usize
    }

    fn weights(&self) -> eval::Weights {
        let mut w = if self.kind == "engine" || self.kind == "planner" {
            eval::Weights::engine()
        } else {
            eval::Weights::default()
        };
        for (k, v) in &self.weights {
            w.set(k, *v);
        }
        w
    }

    /// The forced behaviours (022): keeping by plan and the dominance checks.
    pub fn forced(&self) -> policy::Forced {
        // An agent without these parameters is never wrapped, so its numbers
        // are read by nothing.
        let num = |k: &str| self.params.get(k).copied().unwrap_or(0.0);
        let on = |k: &str| num(k) > 0.0;
        let tag = |k: &str| TAGS.get((num(k) as usize).wrapping_sub(1)).copied();
        policy::Forced {
            keep_all: on("keep_all"),
            keep_k: num("keep_k") as usize,
            keep_none: on("keep_none"),
            keep_random: on("keep_random"),
            tier1_only: on("tier1_only"),
            tier2_rush: on("tier2_rush"),
            no_powers: on("no_powers"),
            no_counters: on("no_counters"),
            no_presence: on("no_presence"),
            only_tag: tag("only_tag"),
            never_tag: tag("never_tag"),
            events_only: on("events_only"),
            ignore_threats: on("ignore_threats"),
            turtle: on("turtle"),
            free_only: on("free_only"),
            unity_farm: on("unity_farm"),
            rich_spend: on("rich_spend"),
            science_rush: on("science_rush"),
            always_expand: on("always_expand"),
            always_consolidate: on("always_consolidate"),
            never_remove: on("never_remove"),
            science_board: on("science_board"),
            plan_keep: on("plan_keep"),
            plan_rounds: num("plan_rounds") as usize,
            plan_reserve: num("plan_reserve") as i64,
            plan_hand: num("plan_hand") as usize,
            plan_t2: num("plan_t2"),
            plan_credit: num("plan_credit"),
            plan_niche: num("plan_niche"),
        }
    }

    /// A fresh agent for one game: the search agent, wrapped in its keep
    /// policy and forced behaviours when it has any (022).
    pub fn build(&self, rules: &Rules, game_seed: u64) -> Box<dyn Agent + Send> {
        let inner = self.build_search(rules, game_seed);
        let forced = self.forced();
        if !forced.is_active() {
            return inner;
        }
        Box::new(policy::Policy::new(inner, forced))
    }

    fn build_search(&self, rules: &Rules, game_seed: u64) -> Box<dyn Agent + Send> {
        let rng = agent_rng(game_seed, &self.kind);
        let ev = eval::Eval::new(rules, self.weights());
        match self.kind.as_str() {
            "random" => Box::new(RandomAgent::new(game_seed)),
            "greedy" => Box::new(Greedy::new(
                ev,
                rng,
                self.p("samples").max(1),
                self.p("depth").max(1),
                self.p("choices"),
            )),
            "beam" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width").max(1),
                self.p("depth").max(1),
                self.p("branch"),
                self.p("choices"),
                None,
            )),
            "rollout" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width").max(1),
                self.p("depth").max(1),
                self.p("branch"),
                self.p("choices"),
                Some(search::RolloutCfg {
                    candidates: self.p("candidates").max(1),
                    rollouts: self.p("rollouts").max(1),
                    horizon: self.p("horizon"),
                }),
            )),
            "engine" if self.p("width") == 0 => Box::new(Greedy::new(
                ev,
                rng,
                self.p("samples").max(1),
                1,
                self.p("choices"),
            )),
            "engine" => Box::new(Beam::new(
                ev,
                rng,
                self.p("width").max(1),
                self.p("depth").max(1),
                self.p("branch"),
                self.p("choices"),
                (self.p("candidates") > 0).then(|| search::RolloutCfg {
                    candidates: self.p("candidates"),
                    rollouts: self.p("rollouts").max(1),
                    horizon: self.p("horizon"),
                }),
            )),
            "planner" => Box::new(planner::Planner::new(
                rules,
                self.weights(),
                self.forced(),
                rng,
                planner::PlannerCfg {
                    horizon: self.p("horizon").max(1) as u32,
                    alts: self.p("alts"),
                    budget: self.p("budget"),
                    margin: self.params["margin"],
                    choices: self.p("choices"),
                    samples: self.p("samples").max(1),
                    prune: self.p("prune"),
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

    pub(crate) fn rules(difficulty: i64) -> Rules {
        config_rules("baseline", difficulty)
    }

    /// Rules for `configs/<name>.json` at `difficulty`.
    pub(crate) fn config_rules(name: &str, difficulty: i64) -> Rules {
        let text = std::fs::read_to_string(format!(
            "{}/../../configs/{name}.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let mut c = Config::from_json(&text).unwrap();
        c.apply_override(&format!("adversary.difficulty={difficulty}"))
            .unwrap();
        Rules::new(&c).unwrap()
    }

    const GREEDY: &str = "greedy";
    const BEAM: &str = "beam:width=2";
    const ROLLOUT: &str = "rollout:width=2,candidates=2,rollouts=1,horizon=1";
    // The engine's beam is beam's code with another heuristic.
    const ENGINE: &str = "engine";
    const PLANNER: &str = "planner:budget=2";

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
    /// (about 1 s; whole games are 200–300 actions).
    #[test]
    fn agents_replay_and_repeat_opening() {
        for spec in [GREEDY, BEAM, ROLLOUT, ENGINE, PLANNER] {
            replay_and_repeat(spec, 30, false);
        }
    }

    /// 022: the dominance checks parse, label and replay (into round 3,
    /// past two keeps).
    #[test]
    fn dominance_checks_replay_and_repeat() {
        for spec in [
            "engine:keep_all=1",
            "engine:keep_k=2",
            "engine:tier2_rush=1",
            "greedy:no_presence=1",
            "engine:only_tag=3",
            "engine:never_tag=6",
            "engine:events_only=1",
            "engine:ignore_threats=1",
            "engine:turtle=1",
            "engine:free_only=1",
            "engine:unity_farm=1",
            "engine:keep_none=1",
            "engine:keep_random=1",
            "planner:budget=2,never_tag=2",
        ] {
            assert_eq!(AgentSpec::parse(spec).unwrap().label(), spec);
            replay_and_repeat(spec, 60, false);
        }
        assert!(AgentSpec::parse("random:keep_all=1").is_err());
    }

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

    /// An agent's decision is the same on two tables that differ only in
    /// what the printed game hides (deck order and the dice).
    #[test]
    fn decisions_ignore_hidden_information() {
        let r = rules(0);
        let mut legal = Vec::new();
        for spec in ["greedy", "beam:width=2", "engine", "engine:width=2"] {
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

    #[test]
    fn specs() {
        let s = AgentSpec::parse("beam:width=4,w.corrupted=20").unwrap();
        assert_eq!(s.label(), "beam:width=4,w.corrupted=20");
        assert!(AgentSpec::parse("greedy:w.pool=3").is_err());
        assert_eq!(AgentSpec::parse("greedy").unwrap().label(), "greedy");
        assert!(AgentSpec::parse("greedy:width=3").is_err());
        assert!(AgentSpec::parse("greedy:w.nope=3").is_err());
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

    /// The deck clock (011; 013 on Unity; 028 on the Unity deck's last card):
    /// `w.clock` per round the team, at its Unity rate so far, would still
    /// be short of its Unity target when the deck runs out; nothing while it
    /// is on course.
    #[test]
    fn deck_clock_penalises_a_team_behind_the_deck() {
        let r = rules(0);
        let mut g = r.new_game(1);
        let value = |clock: f64, g: &aic_core::GameState| {
            let mut w = eval::Weights::default();
            w.clock = clock;
            eval::Eval::new(&r, w).value(&r, g)
        };
        // Round 4, one Unity tip (4 Sway: 1 a round), 6 tips (24 Sway) for the
        // Unity deck's last card: 20 rounds needed. Two cards left (the
        // face-up one and one in the deck): 2 action phases after this one.
        // 18 short.
        g.round = 4;
        g.vars.insert("alignment.unity_tips".into(), 1);
        assert_eq!(g.pile("adversary.next").len(), 1);
        let deck = g.pile("adversary.deck").to_vec();
        g.piles.insert("adversary.deck".into(), deck[..1].to_vec().into());
        assert!((value(0.0, &g) - value(2.0, &g) - 2.0 * 18.0).abs() < 1e-9);
        // On course (4 tips by round 4): no cost.
        g.vars.insert("alignment.unity_tips".into(), 4);
        assert_eq!(value(0.0, &g), value(2.0, &g));
        // The engine watches the deck by default; greedy doesn't (its
        // games, the difficulty calibration, are unchanged).
        assert!(eval::Weights::engine().clock > 0.0);
        assert_eq!(eval::Weights::default().clock, 0.0);
    }
}
