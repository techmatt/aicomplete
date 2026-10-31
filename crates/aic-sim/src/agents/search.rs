//! The searching agents: greedy, beam and rollout. See `mod.rs` for what
//! each does; everything here simulates on determinized copies of the
//! state, never on the state the agent was handed.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use aic_core::modules::adversary;
use aic_core::{Action, Agent, GameState, Rules};
use rand_chacha::ChaCha8Rng;

use super::determinize::{determinize, SHUFFLED};
use super::eval::Eval;
use crate::prof::{self, Part};

/// Hidden piles, compared by size only when checking a plan against the
/// table: every pile [`determinize`] guesses (the shuffled decks and the
/// adversary deck).
fn hidden(key: &str) -> bool {
    key == adversary::DECK || key == adversary::PROJECT_DECK || SHUFFLED.contains(&key)
}

/// `g` after `a`, or `None` if the rules refuse it (a guess of the hidden
/// cards can, rarely, make a replayed choice impossible).
fn step(rules: &Rules, g: &GameState, a: &Action) -> Option<GameState> {
    #[cfg(feature = "profile")]
    drop(prof::time(Part::Clone, || g.clone()));
    prof::time(Part::Apply, || rules.apply_to(g, a, None)).ok()
}

fn value(rules: &Rules, ev: &Eval, g: &GameState) -> f64 {
    prof::time(Part::Eval, || ev.value(rules, g))
}

fn legal_into(rules: &Rules, g: &GameState, out: &mut Vec<Action>) {
    prof::time(Part::Legal, || rules.legal_actions_into(g, out))
}

/// The value of `g`, with any choice it is waiting on answered best-first
/// up to `choices` levels deep: an action's own choices are part of it.
fn settled(
    rules: &Rules,
    ev: &Eval,
    g: &GameState,
    choices: usize,
    legal: &mut Vec<Action>,
) -> f64 {
    if g.is_over() || g.pending.is_none() || choices == 0 {
        return value(rules, ev, g);
    }
    legal_into(rules, g, legal);
    let opts = std::mem::take(legal);
    let best = opts
        .iter()
        .filter_map(|a| step(rules, g, a))
        .map(|c| settled(rules, ev, &c, choices - 1, legal))
        .fold(f64::NEG_INFINITY, f64::max);
    if best.is_finite() {
        best
    } else {
        value(rules, ev, g)
    }
}

/// Best value reachable from `g` in `depth` more actions (0 = `g` itself).
fn lookahead(rules: &Rules, ev: &Eval, g: &GameState, depth: usize, choices: usize) -> f64 {
    let mut legal = Vec::new();
    if depth == 0 || g.is_over() {
        return settled(rules, ev, g, choices, &mut legal);
    }
    legal_into(rules, g, &mut legal);
    let best = legal
        .iter()
        .filter_map(|a| step(rules, g, a))
        .map(|c| lookahead(rules, ev, &c, depth - 1, choices))
        .fold(f64::NEG_INFINITY, f64::max);
    if best.is_finite() {
        best
    } else {
        settled(rules, ev, g, choices, &mut legal)
    }
}

/// How far [`greedy_scores`] looks.
#[derive(Clone, Copy)]
pub struct Scoring {
    /// Guesses averaged (0 = use the state as it is: it is already a guess).
    pub samples: usize,
    /// Actions looked ahead.
    pub depth: usize,
    /// Levels of prompts an action's own choices resolve before it is scored:
    /// prompts, not effects, so one placement (a track, a hex) takes two, and
    /// a card that places more is judged part of the way.
    pub choices: usize,
    /// `choices` for a Growth action.
    pub growth_choices: usize,
}

impl Scoring {
    /// At the top level of a decision: growth at [`full_growth`].
    pub fn decision(samples: usize, depth: usize, choices: usize) -> Self {
        Scoring { samples, depth, choices, growth_choices: full_growth(choices) }
    }

    /// Inside a rollout: growth at plain `choices` (046).
    pub fn in_rollout(self) -> Self {
        Scoring { growth_choices: self.choices, ..self }
    }
}

/// Scores every legal action of `state` by greedy look-ahead, averaged over
/// the guesses `how` asks for.
pub fn greedy_scores(
    rules: &Rules,
    ev: &Eval,
    rng: &mut ChaCha8Rng,
    state: &GameState,
    legal: &[Action],
    how: Scoring,
) -> Vec<f64> {
    let Scoring { samples, depth, choices, growth_choices } = how;
    let mut scores = vec![0.0; legal.len()];
    let n = samples.max(1);
    for _ in 0..n {
        let det;
        let base = if samples == 0 {
            state
        } else {
            let mut d = state.clone();
            determinize(rules, &mut d, rng);
            det = d;
            &det
        };
        // A pass is scored as the table it leaves, not what follows it: the
        // last pass hands the round on to threats and the adversary, and
        // scoring that would make stalling (pay, withdraw, pay...) look
        // better than stopping.
        let stay = settled(rules, ev, base, choices, &mut Vec::new());
        for (i, a) in legal.iter().enumerate() {
            scores[i] += match a {
                Action::Pass { .. } => stay,
                _ => match step(rules, base, a) {
                    Some(c) if matches!(a, Action::Growth { .. }) => lookahead(rules, ev, &c, depth - 1, growth_choices),
                    Some(c) => lookahead(rules, ev, &c, depth - 1, choices),
                    None => f64::NEG_INFINITY,
                },
            };
        }
    }
    scores.iter_mut().for_each(|s| *s /= n as f64);
    scores
}

/// Levels a Growth action's choices are scored at, top level (041): an option
/// places up to 2 presence, each a track and a hex: scored at 2 levels, Expand
/// looked like its first token. 046: not inside rollouts, at about 3.7
/// evaluations a decision, so a planner decides growth on its top-level score.
fn full_growth(choices: usize) -> usize {
    choices * 2
}

/// The best-scoring action; ties go to a pass (doing nothing is never worse
/// than a move that changes nothing), then to the first.
pub fn pick(legal: &[Action], scores: &[f64]) -> usize {
    let best = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let close = |i: usize| scores[i] >= best - 1e-9;
    (0..legal.len())
        .find(|&i| close(i) && matches!(legal[i], Action::Pass { .. }))
        .or_else(|| (0..legal.len()).find(|&i| close(i)))
        .unwrap_or(0)
}

/// A team's own moves this phase: a move that undoes one of them is not
/// offered, so a look-ahead cannot cycle.
#[derive(Default)]
pub(crate) struct PhaseHistory {
    round: u32,
    phase: usize,
    actions: Vec<Action>,
}

impl PhaseHistory {
    /// Starts over if `state` has left the phase; true if it did.
    pub(crate) fn enter(&mut self, state: &GameState) -> bool {
        let new = (self.round, self.phase) != (state.round, state.phase);
        if new {
            *self = PhaseHistory { round: state.round, phase: state.phase, actions: Vec::new() };
        }
        new
    }

    /// Rules out the actions of `legal` that undo a move of this phase.
    pub(crate) fn mask(&self, legal: &[Action], scores: &mut [f64]) {
        for (i, a) in legal.iter().enumerate() {
            if undoes(&self.actions, a) {
                scores[i] = f64::NEG_INFINITY;
            }
        }
    }

    pub(crate) fn push(&mut self, a: &Action) {
        self.actions.push(a.clone());
    }
}

pub struct Greedy {
    ev: Eval,
    rng: ChaCha8Rng,
    how: Scoring,
    history: PhaseHistory,
    /// When above 0: after a decision phase's first full scoring, each seat
    /// keeps only this many actions for the rest of the phase (the planner's
    /// rollouts, 028: a wide menu's tail rarely comes back).
    prune: usize,
    /// The actions dropped this phase, once it has been scored.
    dropped: Option<HashSet<Action>>,
}

impl Greedy {
    pub fn new(ev: Eval, rng: ChaCha8Rng, samples: usize, depth: usize, choices: usize) -> Self {
        Greedy {
            ev,
            rng,
            how: Scoring::decision(samples, depth, choices),
            history: PhaseHistory::default(),
            prune: 0,
            dropped: None,
        }
    }

    pub fn pruned(mut self, prune: usize) -> Self {
        self.prune = prune;
        self
    }

    /// As a rollout's policy: growth at plain `choices` (046).
    pub fn in_rollout(mut self) -> Self {
        self.how = self.how.in_rollout();
        self
    }

    /// Scores `legal`, skipping what this phase has dropped, and at the
    /// phase's first full scoring drops all but each seat's best `prune`.
    fn scores(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> Vec<f64> {
        if self.prune == 0 || state.pending.is_some() {
            return greedy_scores(rules, &self.ev, &mut self.rng, state, legal, self.how);
        }
        let live: Vec<usize> = (0..legal.len())
            .filter(|&i| self.dropped.as_ref().is_none_or(|d| !d.contains(&legal[i])))
            .collect();
        let menu: Vec<Action> = live.iter().map(|&i| legal[i].clone()).collect();
        let found = greedy_scores(rules, &self.ev, &mut self.rng, state, &menu, self.how);
        let mut out = vec![f64::NEG_INFINITY; legal.len()];
        for (k, &i) in live.iter().enumerate() {
            out[i] = found[k];
        }
        if self.dropped.is_none() {
            let mut ranked: Vec<usize> = (0..legal.len()).collect();
            ranked.sort_by(|&x, &y| out[y].total_cmp(&out[x]));
            let mut taken = [0usize; 256];
            let mut dropped = HashSet::new();
            for i in ranked {
                let a = &legal[i];
                let n = &mut taken[a.seat() as usize];
                *n += 1;
                if *n > self.prune && !matches!(a, Action::Pass { .. }) {
                    dropped.insert(a.clone());
                }
            }
            self.dropped = Some(dropped);
        }
        out
    }
}

impl Agent for Greedy {
    fn choose(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize {
        if legal.len() == 1 {
            return 0;
        }
        if self.history.enter(state) {
            self.dropped = None;
        }
        let mut s = self.scores(rules, state, legal);
        self.history.mask(legal, &mut s);
        let i = pick(legal, &s);
        self.history.push(&legal[i]);
        i
    }
}

pub struct RolloutCfg {
    pub candidates: usize,
    pub rollouts: usize,
    pub horizon: usize,
}

/// One planned action and the table it expects to be played on.
struct Planned {
    before: GameState,
    action: Action,
}

pub struct Beam {
    ev: Eval,
    rng: ChaCha8Rng,
    width: usize,
    depth: usize,
    branch: usize,
    /// Levels of an action's own choices resolved best-first when scoring it.
    choices: usize,
    rollout: Option<RolloutCfg>,
    /// The rest of the current plan, next action last.
    plan: Vec<Planned>,
}

/// A sequence in the beam.
struct Node {
    g: GameState,
    seq: Vec<Action>,
    /// Order-free hash of `seq`: sequences that are reorderings of each
    /// other (seats acting in a different order) almost always reach the
    /// same table, so only the best of them is kept.
    key: u64,
}

fn action_hash(a: &Action) -> u64 {
    let mut h = DefaultHasher::new();
    a.hash(&mut h);
    h.finish()
}

/// A hundredth of a point per move (passes and answers aside): of two plans
/// reaching equal tables, the one with fewer moves wins, so a move that
/// changes nothing (an insight gift for nothing) is not made.
fn moves_cost(seq: &[Action]) -> f64 {
    0.01 * seq
        .iter()
        .filter(|a| !matches!(a, Action::Pass { .. } | Action::Choose { .. }))
        .count() as f64
}

/// Moves that only undo an earlier move of the same sequence.
pub(crate) fn undoes(seq: &[Action], a: &Action) -> bool {
    match a {
        Action::Give { seat, to } => seq.iter().any(|b| matches!(b, Action::Give { seat: s, to: t } if s == to && t == seat)),
        Action::Withdraw { seat, threat, resource } => seq.iter().any(|b| {
            matches!(b, Action::Pay { seat: s, threat: t, resource: r, .. } if s == seat && t == threat && r == resource)
        }),
        Action::Pay { seat, threat, resource, .. } => seq.iter().any(|b| {
            matches!(b, Action::Withdraw { seat: s, threat: t, resource: r } if s == seat && t == threat && r == resource)
        }),
        _ => false,
    }
}

/// The table as players see it: everything but the hidden piles' contents
/// and order (their sizes count), the dice and the simulator's stats.
fn same_table(a: &GameState, b: &GameState) -> bool {
    if a.round != b.round
        || a.phase != b.phase
        || a.hexes != b.hexes
        || a.vars != b.vars
        || a.seats != b.seats
    {
        return false;
    }
    if a.pending.as_ref().map(|p| &p.ask) != b.pending.as_ref().map(|p| &p.ask)
        || a.outcome != b.outcome
    {
        return false;
    }
    let keys: std::collections::BTreeSet<&aic_core::Key> = a.piles.keys().chain(b.piles.keys()).collect();
    keys.into_iter().all(|k| {
        if hidden(k) {
            a.pile(k).len() == b.pile(k).len()
        } else {
            a.pile(k) == b.pile(k)
        }
    })
}

impl Beam {
    pub fn new(
        ev: Eval,
        rng: ChaCha8Rng,
        width: usize,
        depth: usize,
        branch: usize,
        choices: usize,
        rollout: Option<RolloutCfg>,
    ) -> Self {
        Beam {
            ev,
            rng,
            width,
            depth,
            branch,
            choices,
            rollout,
            plan: Vec::new(),
        }
    }

    /// Searches the rest of the round from a guess of `state`. Returns the
    /// guess and the finished sequences (reaching the next round or the
    /// end of the game), best first, with their end states and values.
    fn search(&mut self, rules: &Rules, state: &GameState) -> (GameState, Vec<(f64, Node)>) {
        let mut start = state.clone();
        determinize(rules, &mut start, &mut self.rng);
        let round = start.round;
        let mut beam = vec![Node {
            g: start.clone(),
            seq: Vec::new(),
            key: 0,
        }];
        let mut finished: Vec<(f64, Node)> = Vec::new();
        let mut legal = Vec::new();
        let mut scratch = Vec::new();
        for _ in 0..self.depth {
            let mut children: Vec<(f64, Node)> = Vec::new();
            for node in &beam {
                legal_into(rules, &node.g, &mut legal);
                let mut kids: Vec<(f64, Node)> = Vec::new();
                for a in &legal {
                    if undoes(&node.seq, a) {
                        continue;
                    }
                    let Some(c) = step(rules, &node.g, a) else {
                        continue;
                    };
                    // The choices an action asks for are scored best-first,
                    // so an action is not judged by its first open question.
                    let mut seq = node.seq.clone();
                    seq.push(a.clone());
                    let v =
                        settled(rules, &self.ev, &c, self.choices, &mut scratch) - moves_cost(&seq);
                    let key = node.key.wrapping_add(action_hash(a));
                    kids.push((v, Node { g: c, seq, key }));
                }
                if self.branch > 0 && kids.len() > self.branch {
                    kids.sort_by(|x, y| y.0.total_cmp(&x.0));
                    kids.truncate(self.branch);
                }
                for (v, kid) in kids {
                    if kid.g.is_over() || kid.g.round > round {
                        let v = value(rules, &self.ev, &kid.g) - moves_cost(&kid.seq);
                        finished.push((v, kid));
                    } else {
                        children.push((v, kid));
                    }
                }
            }
            if children.is_empty() {
                break;
            }
            // Stable sort: ties keep legal-action order, so play is deterministic.
            children.sort_by(|x, y| y.0.total_cmp(&x.0));
            let mut seen = HashSet::new();
            beam = children
                .into_iter()
                .filter(|(_, n)| seen.insert(n.key))
                .take(self.width)
                .map(|(_, n)| n)
                .collect();
        }
        if finished.is_empty() {
            // The round did not end within `depth` steps: plan what was searched.
            finished = beam
                .into_iter()
                .map(|n| (value(rules, &self.ev, &n.g) - moves_cost(&n.seq), n))
                .collect();
        }
        finished.sort_by(|x, y| y.0.total_cmp(&x.0));
        let mut seen = HashSet::new();
        finished.retain(|(_, n)| seen.insert(n.key));
        (start, finished)
    }

    /// Mean value of greedy playouts from `g` for `horizon` rounds past its own.
    fn playouts(&mut self, rules: &Rules, g: &GameState, cfg: &RolloutCfg) -> f64 {
        let mut total = 0.0;
        let mut legal = Vec::new();
        for _ in 0..cfg.rollouts {
            let mut s = g.clone();
            determinize(rules, &mut s, &mut self.rng);
            let stop = s.round + cfg.horizon as u32;
            let mut guard = 0;
            while !s.is_over() && s.round < stop && guard < 2000 {
                legal_into(rules, &s, &mut legal);
                if legal.is_empty() {
                    break;
                }
                let i = if legal.len() == 1 {
                    0
                } else {
                    let how = Scoring::decision(0, 1, 1).in_rollout();
                    let sc = greedy_scores(rules, &self.ev, &mut self.rng, &s, &legal, how);
                    pick(&legal, &sc)
                };
                if rules.apply_mut(&mut s, &legal[i], None).is_err() {
                    break;
                }
                guard += 1;
            }
            total += value(rules, &self.ev, &s);
        }
        total / cfg.rollouts as f64
    }

    fn replan(&mut self, rules: &Rules, state: &GameState) {
        let (start, mut finished) = self.search(rules, state);
        let mut best = 0;
        if let Some(cfg) = self.rollout.take() {
            let mut top = f64::NEG_INFINITY;
            for (i, (value, node)) in finished.iter().enumerate().take(cfg.candidates) {
                let v = if node.g.is_over() {
                    *value
                } else {
                    self.playouts(rules, &node.g, &cfg)
                };
                if v > top {
                    top = v;
                    best = i;
                }
            }
            self.rollout = Some(cfg);
        }
        let seq = match finished.get_mut(best) {
            Some((_, n)) => std::mem::take(&mut n.seq),
            None => Vec::new(),
        };
        // Replay the sequence on the guess to record the table each action
        // expects; a different table later means the guess was wrong.
        let mut g = start;
        let mut plan = Vec::with_capacity(seq.len());
        for a in seq {
            let before = g.clone();
            if rules.apply_mut(&mut g, &a, None).is_err() {
                break;
            }
            plan.push(Planned { before, action: a });
        }
        plan.reverse();
        self.plan = plan;
    }
}

impl Agent for Beam {
    fn choose(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize {
        for attempt in 0..2 {
            if let Some(p) = self.plan.last() {
                if same_table(&p.before, state) {
                    if let Some(i) = legal.iter().position(|a| *a == p.action) {
                        self.plan.pop();
                        return i;
                    }
                }
            }
            if attempt == 0 {
                if legal.len() == 1 {
                    self.plan.clear();
                    return 0;
                }
                self.replan(rules, state);
            }
        }
        // The fresh plan should start here; if it cannot, fall back to greedy.
        self.plan.clear();
        let s = greedy_scores(rules, &self.ev, &mut self.rng, state, legal, Scoring::decision(1, 1, 1));
        pick(legal, &s)
    }
}
