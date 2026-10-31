//! The planner (028): plays like the engine, but tries its shortlisted
//! alternatives by playing each forward a few rounds with the engine as the
//! rollout policy, and takes whichever line ends best on the heuristic.
//!
//! **Full information** (Matt, 2026-10-29, `docs/principles.md`): a rollout
//! plays on the real state, decks and dice included, so a line is exactly the
//! game that would follow. It is not a model of a table; it is a stronger
//! judge of the rules than a one-step agent. Each decision inside a line is
//! still the engine's own, scored on a guess: scored on the open decks the
//! one-step heuristic won 19% against the guessing engine's 59% (c2 d2).
//!
//! **Speed.** Because the game is deterministic from a state, the line chosen
//! at one decision is still valid at the next: the engine's own continuation
//! is known, with its value, without playing it again. So each decision pays
//! only for its alternatives, and a new round only extends the line by a
//! round. Alternatives are capped by `alts` a decision and `budget` a round,
//! spent only in the action phase's decisions (the draft and keeps follow the
//! engine: a rollout over a hand of unknown use says little), and only on
//! actions the engine's own score keeps within `margin` of its best.
//!
//! A dominance check is a [`Policy`] around the planner, with the same
//! [`Forced`] in its rollouts, so the check's rule holds for the whole line.

use aic_core::{Action, Agent, GameState, Rules};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::eval::{Eval, Weights};
use super::policy::{Forced, Policy};
use super::search::{greedy_scores, pick, Greedy, PhaseHistory, Scoring};
use crate::prof::{self, Part};

/// The phase the planner searches.
const ACTION_PHASE: &str = "cards.play+";

/// Whether `state`'s phase is one the planner searches, a pending choice
/// inside it included. In every other phase it plays the next move of its
/// line, whatever the top-level score says; a real growth pick moves the line
/// first (see `choose`).
fn searched_phase(state: &GameState) -> bool {
    state.config.round.get(state.phase).is_some_and(|p| p.starts_with(ACTION_PHASE))
}

pub struct PlannerCfg {
    /// Rounds a line runs past the current one's start.
    pub horizon: u32,
    /// Alternatives tried per decision.
    pub alts: usize,
    /// Alternatives tried per round.
    pub budget: usize,
    /// Only actions within this much of the engine's best score are tried.
    pub margin: f64,
    /// Levels of an action's own choices resolved when scoring it, in the
    /// shortlist and in rollouts.
    pub choices: usize,
    /// Guesses of the hidden cards averaged per decision, as the engine's.
    pub samples: usize,
    /// Actions each seat keeps through a phase of a rollout ([`Greedy`]).
    pub prune: usize,
}

/// A line of play: the actions from `cursor` on, and where they end.
struct Line {
    actions: Vec<Action>,
    next: usize,
    /// The table the next action is played on.
    cursor: GameState,
    end: GameState,
    value: f64,
    /// `end` is the first decision of this round, or the end of the game.
    stop: u32,
}

pub struct Planner {
    forced: Forced,
    ev: Eval,
    rng: ChaCha8Rng,
    cfg: PlannerCfg,
    line: Option<Line>,
    /// (round, alternatives tried in it).
    spent: (u32, usize),
    history: PhaseHistory,
}

/// Actions that are the same idea: the shortlist tries one of each. A threat
/// paid in pieces is one idea; insight gifts and withdrawals are not tried.
fn idea(a: &Action) -> Option<String> {
    match a {
        Action::Give { .. } | Action::Withdraw { .. } => None,
        Action::Pay { seat, threat, .. } => Some(format!("pay {seat} {threat}")),
        _ => Some(format!("{a:?}")),
    }
}

impl Planner {
    pub fn new(rules: &Rules, weights: Weights, forced: Forced, rng: ChaCha8Rng, cfg: PlannerCfg) -> Self {
        Planner {
            ev: Eval::new(rules, weights),
            forced,
            rng,
            cfg,
            line: None,
            spent: (0, 0),
            history: PhaseHistory::default(),
        }
    }

    /// A fresh rollout policy: the engine with this planner's forced
    /// behaviours, guessing from a seed of the state's own actions count, so
    /// a line repeats.
    fn policy(&self, g: &GameState) -> Box<dyn Agent + Send> {
        let rng = ChaCha8Rng::seed_from_u64(g.seed ^ g.stats.actions as u64);
        let inner = Box::new(Greedy::new(self.ev.clone(), rng, self.cfg.samples, 1, self.cfg.choices).pruned(self.cfg.prune).in_rollout());
        Box::new(Policy::new(inner, self.forced.clone()))
    }

    /// Plays `g` on with the rollout policy to the first decision of round
    /// `stop` or the end of the game, adding its moves to `actions`.
    fn play_on(&self, rules: &Rules, g: &mut GameState, stop: u32, actions: &mut Vec<Action>) {
        let mut agent = self.policy(g);
        let mut legal = Vec::new();
        while !g.is_over() && g.round < stop {
            rules.legal_actions_into(g, &mut legal);
            if legal.is_empty() {
                break;
            }
            let i = if legal.len() == 1 { 0 } else { agent.choose(rules, g, &legal) };
            if prof::time(Part::ApplyMut, || rules.apply_mut(g, &legal[i], None)).is_err() {
                break;
            }
            actions.push(legal[i].clone());
        }
    }

    /// The line from `state` opening with `first`, or the engine's own.
    fn line_from(&self, rules: &Rules, state: &GameState, first: Option<&Action>, stop: u32) -> Option<Line> {
        prof::time(Part::Line, || {
            let mut end = state.clone();
            let mut actions = Vec::new();
            if let Some(a) = first {
                rules.apply_mut(&mut end, a, None).ok()?;
                actions.push(a.clone());
            }
            self.play_on(rules, &mut end, stop, &mut actions);
            Some(Line {
                actions,
                next: 0,
                cursor: state.clone(),
                value: self.ev.value(rules, &end),
                end,
                stop,
            })
        })
    }

    /// Moves the line on to `state`, dropping it if the table is not the one
    /// it expected (a forced behaviour outside the planner took another move).
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

    /// The line to beat at `state`: the one being followed, run on to `stop`
    /// if the round has moved, or else the engine's.
    fn incumbent(&mut self, rules: &Rules, state: &GameState, legal: &[Action], stop: u32) -> Line {
        let line = self.line.take().filter(|l| {
            l.actions.get(l.next).is_some_and(|a| legal.contains(a))
        });
        let Some(mut l) = line else {
            return self.line_from(rules, state, None, stop).expect("only an opening move can be refused");
        };
        if l.stop < stop {
            let mut end = std::mem::replace(&mut l.end, state.clone());
            self.play_on(rules, &mut end, stop, &mut l.actions);
            l.value = self.ev.value(rules, &end);
            l.end = end;
            l.stop = stop;
        }
        l
    }
}

impl Agent for Planner {
    fn choose(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize {
        self.sync(rules, state);
        if legal.len() == 1 {
            return 0;
        }
        self.history.enter(state);
        if self.spent.0 != state.round {
            self.spent = (state.round, 0);
        }
        let how = Scoring::decision(self.cfg.samples, 1, self.cfg.choices);
        let mut scores = greedy_scores(rules, &self.ev, &mut self.rng, state, legal, how);
        self.history.mask(legal, &mut scores);
        let engine = pick(legal, &scores);
        let stop = state.round + self.cfg.horizon;
        let mut best = self.incumbent(rules, state, legal, stop);
        // 046: a line's growth was scored shallow, so a real one is the top
        // level's score, and the line follows it.
        if matches!(legal[engine], Action::Growth { .. }) && best.actions.get(best.next) != Some(&legal[engine]) {
            if let Some(l) = self.line_from(rules, state, Some(&legal[engine]), stop) {
                best = l;
            }
        }
        if searched_phase(state) {
            let mut order: Vec<usize> = (0..legal.len()).filter(|&i| scores[i].is_finite()).collect();
            order.sort_by(|&x, &y| scores[y].total_cmp(&scores[x]));
            let top = scores[engine];
            let mut seen = vec![best.actions.get(best.next).and_then(idea)];
            let mut tried = 0;
            for i in order {
                if tried >= self.cfg.alts || self.spent.1 >= self.cfg.budget || scores[i] < top - self.cfg.margin {
                    break;
                }
                let key = idea(&legal[i]);
                if key.is_none() || seen.contains(&key) {
                    continue;
                }
                seen.push(key);
                tried += 1;
                self.spent.1 += 1;
                if let Some(l) = self.line_from(rules, state, Some(&legal[i]), stop) {
                    if l.value > best.value + 1e-9 {
                        best = l;
                    }
                }
            }
        }
        let i = best.actions.get(best.next).and_then(|a| legal.iter().position(|b| b == a));
        self.line = Some(best);
        let i = i.unwrap_or(engine);
        self.history.push(&legal[i]);
        i
    }
}
