//! The planner (028): plays like the engine, but tries its shortlisted
//! alternatives by playing each forward a few rounds with the engine as the
//! rollout policy, and takes whichever line ends best on the heuristic.
//!
//! **Full information** (Matt, 2026-10-29, `docs/principles.md`): a rollout
//! plays on the real state, decks and dice included, so a line is exactly the
//! game that would follow. It is not a model of a table; it is a stronger
//! judge of the rules than a one-step agent. The one exception is D20: which
//! Alignment cards were dealt, and in what order, is never read. Each decision inside a line is
//! still the engine's own, scored on a guess: scored on the open decks the
//! one-step heuristic won 19% against the guessing engine's 59% (c2 d2).
//!
//! **Sampled deals (054).** A line's value is averaged over `deals` guesses of
//! the deal, the same guesses for every line. Up to its first tip a line plays
//! the same on all of them, since nothing else reads the cards, so it is
//! played once and forks at that tip; one way it goes on the first guess is
//! the line followed, and it is dropped at the next real tip: the table has
//! seen a card and the guesses another. An alternative must lead the line to
//! beat by over `lead`, on the first guess and then on the mean: a guessed
//! line is a noisy read, and the best of several noisy reads is mostly luck.
//! Only one past the first test plays the other guesses, so a line
//! unchallenged plays one. The default is four guesses and no `lead` (064):
//! it led one guess by 12.5 ±5.4 on 96 fresh paired seeds for 1.57 × (c2) and
//! 1.54 × (c3) an action. `lead` lost points, sampling gained them (060), and
//! `deals=1,lead=0`, 061's default, is a spec away.
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

use aic_core::content::Side;
use aic_core::modules::alignment;
use aic_core::{Action, Agent, GameState, Rules};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::determinize::hide_alignment_deal;
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
    /// Guesses of the Alignment deal a line's value is averaged over.
    pub deals: usize,
    /// What an alternative must lead the line to beat by.
    pub lead: f64,
}

/// The table before a line's first tip, and the move that makes the tip.
type Fork = (GameState, Action);

/// A line of play: the actions from `cursor` on, and where they end.
struct Line {
    /// As played on the first deal: the same on all of them up to the first tip.
    actions: Vec<Action>,
    next: usize,
    /// The table the next action is played on.
    cursor: GameState,
    /// Where each deal ends: the first deal's alone until the line forks.
    ends: Vec<GameState>,
    /// Each end's value.
    values: Vec<f64>,
    /// The line's first tip, while the other deals are to play it.
    fork: Option<Fork>,
    /// Every end is the first decision of this round, or the end of the game.
    stop: u32,
    /// Tips made when the line was built (D20): its later cards are a guess.
    tips: i64,
}

impl Line {
    /// The line's value: the mean over the deals it has played. Before its
    /// first tip one deal stands for all.
    fn value(&self) -> f64 {
        self.values.iter().sum::<f64>() / self.values.len() as f64
    }
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

/// Tips made so far, both sides.
fn tips(g: &GameState) -> i64 {
    g.var(alignment::tips_key(Side::Unity)) + g.var(alignment::tips_key(Side::Discord))
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
    /// behaviors, guessing from a seed of the state's own actions count, so
    /// a line repeats.
    fn policy(&self, g: &GameState) -> Box<dyn Agent + Send> {
        let rng = ChaCha8Rng::seed_from_u64(g.seed ^ g.stats.actions as u64);
        // `pruned` changes what the policy can play, not only how fast: see `Greedy::prune`.
        let inner = Box::new(Greedy::new(self.ev.clone(), rng, self.cfg.samples, 1, self.cfg.choices).pruned(self.cfg.prune).in_rollout());
        Box::new(Policy::new(inner, self.forced.clone()))
    }

    /// Plays `g` on with the rollout policy to the first decision of round
    /// `stop` or the end of the game, adding its moves to `actions`. With
    /// `watch` tips made so far, gives back the next tip.
    fn play_on(
        &self,
        rules: &Rules,
        g: &mut GameState,
        stop: u32,
        actions: &mut Vec<Action>,
        mut watch: Option<i64>,
    ) -> Option<Fork> {
        let mut agent = self.policy(g);
        let mut fork = None;
        let mut legal = Vec::new();
        while !g.is_over() && g.round < stop {
            rules.legal_actions_into(g, &mut legal);
            if legal.is_empty() {
                break;
            }
            let i = if legal.len() == 1 { 0 } else { agent.choose(rules, g, &legal) };
            let before = watch.map(|_| g.clone());
            if prof::time(Part::ApplyMut, || rules.apply_mut(g, &legal[i], None)).is_err() {
                break;
            }
            actions.push(legal[i].clone());
            if watch.is_some_and(|t| t != tips(g)) {
                fork = before.map(|b| (b, legal[i].clone()));
                watch = None;
            }
        }
        fork
    }

    /// The tip count `play_on` should watch on a line that has not forked.
    fn watch(&self, line_tips: i64) -> Option<i64> {
        (self.cfg.deals > 1).then_some(line_tips)
    }

    /// The first deal of the line from `state` opening with `first`, or the
    /// engine's own.
    fn line_from(&self, rules: &Rules, state: &GameState, first: Option<&Action>, stop: u32) -> Option<Line> {
        prof::time(Part::Line, || {
            let mut end = state.clone();
            hide_alignment_deal(rules, &mut end, state.seed, 0);
            let mut actions = Vec::new();
            let made = tips(state);
            let mut fork = None;
            if let Some(a) = first {
                let before = self.watch(made).map(|_| end.clone());
                rules.apply_mut(&mut end, a, None).ok()?;
                actions.push(a.clone());
                if tips(&end) != made {
                    fork = before.map(|b| (b, a.clone()));
                }
            }
            let watch = if fork.is_none() { self.watch(made) } else { None };
            fork = fork.or(self.play_on(rules, &mut end, stop, &mut actions, watch));
            Some(Line {
                actions,
                next: 0,
                cursor: state.clone(),
                values: vec![self.ev.value(rules, &end)],
                ends: vec![end],
                fork,
                stop,
                tips: made,
            })
        })
    }

    /// Plays the line's other deals from its fork, if it has reached one.
    fn other_deals(&self, rules: &Rules, line: &mut Line) {
        let Some((table, tipping)) = line.fork.take() else { return };
        prof::time(Part::Line, || {
            for sample in 1..self.cfg.deals {
                let mut end = table.clone();
                hide_alignment_deal(rules, &mut end, table.seed, sample);
                if rules.apply_mut(&mut end, &tipping, None).is_ok() {
                    self.play_on(rules, &mut end, line.stop, &mut Vec::new(), None);
                }
                line.values.push(self.ev.value(rules, &end));
                line.ends.push(end);
            }
        })
    }

    /// Moves the line on to `state`, dropping it if the table is not the one
    /// it expected (a forced behavior outside the planner took another move).
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
            if l.cursor != *state || l.tips != tips(state) {
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
            // Only one that has not forked can fork now.
            let watch = if l.ends.len() == 1 && l.fork.is_none() { self.watch(l.tips) } else { None };
            let mut unused = Vec::new();
            for (k, end) in l.ends.iter_mut().enumerate() {
                let actions = if k == 0 { &mut l.actions } else { &mut unused };
                if let Some(f) = self.play_on(rules, end, stop, actions, watch) {
                    l.fork = Some(f);
                }
                l.values[k] = self.ev.value(rules, end);
            }
            l.stop = stop;
        }
        l
    }

    /// `alt` if it leads `best` by over `lead` on the first deal and then on
    /// the mean. Neither plays its other deals before the first test.
    fn better(&self, rules: &Rules, mut alt: Line, mut best: Line) -> Line {
        let bar = self.cfg.lead + 1e-9;
        if alt.values[0] <= best.values[0] + bar {
            return best;
        }
        self.other_deals(rules, &mut alt);
        self.other_deals(rules, &mut best);
        if alt.value() > best.value() + bar {
            alt
        } else {
            best
        }
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
                    best = self.better(rules, l, best);
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
