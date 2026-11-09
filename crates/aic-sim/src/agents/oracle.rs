//! The oracle (155): a full-information search for a winning line, the
//! measuring stick for a very skilled player (not a model of a table).
//!
//! It sees the seeded game exactly: the decks' order, the dice, the adversary's
//! future cards and which Alignment cards were dealt (the planner's one
//! exception, D20, is dropped). The game is deterministic from a state and the
//! choices, and one agent plays every seat, so a line played on the real state
//! is exactly the game that follows: a line that ends in a win proves the seed
//! winnable, and the oracle then follows it to the end without searching
//! again. Its win rate is a floor on what a perfect player can do.
//!
//! The search is the planner's (`planner.rs`), run to the end of the game: at a
//! decision of the action phase it plays the followed line and up to `alts`
//! alternatives (the engine's best-scored, within `margin`; `budget` a round)
//! to the game's end with the engine as the rollout policy. A line that wins
//! beats one that does not; between two that do not, the heuristic at the
//! first decision of round `horizon` rounds on (the planner's judgment) picks.
//! `reseeds` adds, at a round's first searched decision while no win is in
//! hand, that many lines of the policy guessing on other seeds (the engine's
//! decisions inside a line are scored on a guess, so another guess is another
//! line). The budget dials (`alts`, `budget`, `reseeds`) are the anytime knob:
//! more lines, more seeds found winnable. Decisions inside a line are still
//! scored on a guess, as the planner's are (`docs/principles.md`).

use aic_core::{Action, Agent, GameState, Rules};
use rand_chacha::ChaCha8Rng;

use super::eval::Eval;
use super::planner::{idea, rollout_policy, searched_phase};
use super::policy::Forced;
use super::search::{draw_guesses, pick, score_guesses, PhaseHistory, Scoring};
use crate::prof::{self, Part};

/// A line is played at most this many actions past the table (the experiment's default cap).
const MAX_ACTIONS: usize = 5000;

pub struct OracleCfg {
    /// Rounds on at which two lines that do not win are compared.
    pub horizon: u32,
    /// Alternatives tried per decision.
    pub alts: usize,
    /// Alternatives tried per round.
    pub budget: usize,
    /// Only actions within this much of the engine's best score are tried.
    pub margin: f64,
    /// Lines on other policy seeds tried at a round's first searched decision.
    pub reseeds: usize,
    pub choices: usize,
    pub samples: usize,
    pub prune: usize,
    pub powers: usize,
    pub answers: usize,
    /// Lines played side by side (scoped threads); the choice is the same at any count.
    pub threads: usize,
}

/// A line of play to the end of the game.
struct Line {
    actions: Vec<Action>,
    next: usize,
    /// The table the next action is played on.
    cursor: GameState,
    /// The heuristic at each round's first decision along the line, then
    /// (`u32::MAX`) at its end.
    marks: Vec<(u32, f64)>,
    won: bool,
}

impl Line {
    /// The line's standing at round `at`: whether it wins, then the heuristic.
    fn key(&self, at: u32) -> (bool, f64) {
        let v = self.marks.iter().find(|(r, _)| *r >= at).map_or(f64::NEG_INFINITY, |m| m.1);
        (self.won, v)
    }
}

pub struct Oracle {
    forced: Forced,
    ev: Eval,
    rng: ChaCha8Rng,
    cfg: OracleCfg,
    line: Option<Line>,
    /// (round, alternatives tried in it, reseeded in it).
    spent: (u32, usize, bool),
    history: PhaseHistory,
    /// Lines played, for the cost read.
    pub lines: usize,
}

impl Oracle {
    pub fn new(ev: Eval, forced: Forced, rng: ChaCha8Rng, cfg: OracleCfg) -> Self {
        Oracle { ev, forced, rng, cfg, line: None, spent: (0, 0, false), history: PhaseHistory::default(), lines: 0 }
    }

    fn line_from(&mut self, rules: &Rules, state: &GameState, first: Option<&Action>, salt: u64) -> Option<Line> {
        self.lines += 1;
        Kit { ev: &self.ev, forced: &self.forced, cfg: &self.cfg }.line(rules, state, first, salt)
    }

    /// Plays the lines of `tasks` (an opening move or the policy's, a policy
    /// seed) in order, `threads` at a time, handing each to `take` in order
    /// until it says stop: the same lines reach `take` at any thread count.
    fn lines_until(
        &mut self,
        rules: &Rules,
        state: &GameState,
        tasks: &[(Option<Action>, u64)],
        mut take: impl FnMut(Option<Line>) -> bool,
    ) {
        let kit = Kit { ev: &self.ev, forced: &self.forced, cfg: &self.cfg };
        let t = self.cfg.threads.max(1);
        for batch in tasks.chunks(t) {
            self.lines += batch.len();
            let got: Vec<Option<Line>> = if batch.len() == 1 {
                vec![kit.line(rules, state, batch[0].0.as_ref(), batch[0].1)]
            } else {
                std::thread::scope(|sc| {
                    let hs: Vec<_> = batch
                        .iter()
                        .map(|(a, salt)| sc.spawn(move || kit.line(rules, state, a.as_ref(), *salt)))
                        .collect();
                    hs.into_iter().map(|h| h.join().expect("a line panicked")).collect()
                })
            };
            for l in got {
                if take(l) {
                    return;
                }
            }
        }
    }
}

/// What a line reads, shared by the threads that play lines.
#[derive(Clone, Copy)]
struct Kit<'a> {
    ev: &'a Eval,
    forced: &'a Forced,
    cfg: &'a OracleCfg,
}

impl Kit<'_> {
    /// The rollout policy: the planner's, guessing from the state's own
    /// actions count, moved by `salt` (0: the planner's own seed).
    fn policy(&self, g: &GameState, salt: u64) -> Box<dyn Agent + Send> {
        let c = self.cfg;
        let seed = (g.seed ^ g.stats.actions as u64).wrapping_add(salt.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        rollout_policy(self.ev, self.forced, seed, c.samples, c.choices, c.prune, c.powers, c.answers)
    }

    /// The line from `state` opening with `first` (or the policy's own move),
    /// played to the end of the game.
    fn line(&self, rules: &Rules, state: &GameState, first: Option<&Action>, salt: u64) -> Option<Line> {
        prof::time(Part::Line, || {
            let mut g = state.clone();
            let mut actions = Vec::new();
            let mut marks = Vec::new();
            let mut last = state.round;
            if let Some(a) = first {
                rules.apply_mut(&mut g, a, None).ok()?;
                actions.push(a.clone());
            }
            let mut agent = self.policy(&g, salt);
            let mut legal = Vec::new();
            let cap = state.stats.actions as usize + MAX_ACTIONS;
            while !g.is_over() && (g.stats.actions as usize) < cap {
                if g.round > last {
                    marks.push((g.round, self.ev.value(rules, &g)));
                    last = g.round;
                }
                rules.legal_actions_into(&g, &mut legal);
                if legal.is_empty() {
                    break;
                }
                let i = if legal.len() == 1 { 0 } else { agent.choose(rules, &g, &legal) };
                if prof::time(Part::ApplyMut, || rules.apply_mut(&mut g, &legal[i], None)).is_err() {
                    break;
                }
                actions.push(legal[i].clone());
            }
            marks.push((u32::MAX, self.ev.value(rules, &g)));
            let won = g.outcome.as_ref().is_some_and(|o| o.won);
            Some(Line { actions, next: 0, cursor: state.clone(), marks, won })
        })
    }
}

impl Oracle {

    /// Moves the line on to `state`, dropping it if the table is not the one
    /// it expected (a forced behavior outside the oracle took another move).
    fn sync(&mut self, rules: &Rules, state: &GameState) {
        prof::time(Part::Sync, || {
            let Some(l) = &mut self.line else { return };
            while l.cursor.stats.actions < state.stats.actions {
                let done = match l.actions.get(l.next) {
                    Some(a) => rules.apply_mut(&mut l.cursor, a, None).is_ok(),
                    None => false,
                };
                if !done {
                    self.line = None;
                    return;
                }
                l.next += 1;
            }
            if l.cursor != *state {
                self.line = None;
            }
        })
    }
}

impl super::Strategic for Oracle {
    fn set_strategies(&mut self, c: super::strategy::Choice) {
        self.ev.set_strategies(c);
    }

    fn eval(&self) -> Option<&Eval> {
        Some(&self.ev)
    }
}

impl Agent for Oracle {
    fn choose(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize {
        self.sync(rules, state);
        if legal.len() == 1 {
            return 0;
        }
        self.history.enter(state);
        if self.spent.0 != state.round {
            self.spent = (state.round, 0, false);
        }
        let how = Scoring::decision(self.cfg.samples, 1, self.cfg.choices);
        let guesses = draw_guesses(rules, &mut self.rng, state, self.cfg.samples);
        let at = state.round + self.cfg.horizon;
        let followed = self.line.take().filter(|l| l.actions.get(l.next).is_some_and(|a| legal.contains(a)));
        let mut best = match followed {
            Some(l) => l,
            None => self.line_from(rules, state, None, 0).expect("only an opening move can be refused"),
        };
        let mut fallback = None;
        let searching = !best.won && searched_phase(state);
        if searching && self.cfg.reseeds > 0 && !self.spent.2 {
            self.spent.2 = true;
            let tasks: Vec<(Option<Action>, u64)> = (1..=self.cfg.reseeds as u64).map(|salt| (None, salt)).collect();
            self.lines_until(rules, state, &tasks, |l| {
                let l = l.expect("no opening move");
                if l.key(at) > best.key(at) {
                    best = l;
                }
                best.won
            });
        }
        if !best.won && searching && self.cfg.alts > 0 && self.spent.1 < self.cfg.budget {
            let mut scores = prof::time(Part::Top, || score_guesses(rules, &self.ev, state, &guesses, legal, how));
            self.history.mask(legal, &mut scores);
            let engine = pick(legal, &scores);
            fallback = Some(engine);
            let mut order: Vec<usize> = (0..legal.len()).filter(|&i| scores[i].is_finite()).collect();
            order.sort_by(|&x, &y| scores[y].total_cmp(&scores[x]));
            let top = scores[engine];
            let mut seen = vec![best.actions.get(best.next).and_then(idea)];
            // The shortlist, at most what the decision and the round have left.
            let room = self.cfg.alts.min(self.cfg.budget - self.spent.1);
            let mut tasks: Vec<(Option<Action>, u64)> = Vec::new();
            for i in order {
                if tasks.len() >= room || scores[i] < top - self.cfg.margin {
                    break;
                }
                let key = idea(&legal[i]);
                if key.is_none() || seen.contains(&key) {
                    continue;
                }
                seen.push(key);
                tasks.push((Some(legal[i].clone()), 0));
            }
            let mut spent = self.spent.1;
            self.lines_until(rules, state, &tasks, |l| {
                spent += 1;
                if let Some(l) = l.filter(|l| l.key(at) > best.key(at)) {
                    best = l;
                }
                best.won
            });
            self.spent.1 = spent;
        }
        let i = best.actions.get(best.next).and_then(|a| legal.iter().position(|b| b == a));
        self.line = Some(best);
        let i = i.or(fallback).unwrap_or_else(|| {
            let mut scores = score_guesses(rules, &self.ev, state, &guesses, legal, how);
            self.history.mask(legal, &mut scores);
            pick(legal, &scores)
        });
        self.history.push(&legal[i]);
        i
    }
}
