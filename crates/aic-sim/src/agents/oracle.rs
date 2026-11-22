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
//! scored on a guess, as the planner's are (`docs/principles.md`), unless
//! `truth` (243) scores them on the true table.
//!
//! 243: `tries` restarts the whole search from the first decision on other
//! policy seeds when a game is lost ([`Tries`]): the cheapest way found to win
//! more seeds, since one oracle's result on a seed swings with its lines' seeds.
//! `prove` is a card proof: only a line that plays the card counts as a win.
//!
//! 249, **the real oracle** (`worlds` > 0): the same search on worlds sampled from what players can see
//! ([`super::determinize`]), never the true table, so it makes very strong plays but can't see the future: the
//! player the game is designed against. See [`Oracle::choose_real`].

use aic_core::{Action, Agent, GameState, Rules};
use rand_chacha::ChaCha8Rng;

use super::eval::Eval;
use super::planner::{idea, rollout_policy, searched_phase};
use super::policy::{Behaviors, Forced};
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
    /// 174: levels a discard prompt is resolved at, at most (99: no cap).
    pub discards: usize,
    /// Lines played side by side (scoped threads); the choice is the same at any count.
    pub threads: usize,
    /// 243: lines whose policy scores on the true table, not a guess (0: none, 1: all but the reseeds,
    /// whose guesses are their variety; 2: all).
    pub truth: usize,
    /// 243, a card proof: a line counts as won only if this card is played in the game, and lines take
    /// the card at every chance (drafted, kept, played) until it is played.
    pub prove: Option<String>,
    /// 243: moves every line's policy seed (a restart's try: [`Tries`]); 0: today's lines.
    pub salt: u64,
    /// 249, the real oracle: worlds sampled from what the players can see, each candidate played to the end on
    /// every one ([`Oracle::choose_real`]); 0: the full oracle, on the true table.
    pub worlds: usize,
    /// 314, the real oracle: 1 searches every phase's decisions (the draft too), 0 the action phase's alone.
    pub phases: usize,
    /// 314, the real oracle: 1 plays a decision's lone candidate (within `margin`) unsearched, so the round's
    /// lines go to decisions with a choice; 0 plays it out on every world as 249 did.
    pub lone: usize,
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
    behaviors: Behaviors,
    ev: Eval,
    rng: ChaCha8Rng,
    cfg: OracleCfg,
    line: Option<Line>,
    /// (round, alternatives tried in it, reseeded in it).
    spent: (u32, usize, bool),
    history: PhaseHistory,
    /// Lines played, for the cost read.
    pub lines: usize,
    /// The card to prove has been played on the real table.
    proved: bool,
    /// 249, the real oracle: the chosen candidate's line on each sampled world still agreeing with the table.
    followed: Vec<Line>,
}

impl Oracle {
    pub fn new(ev: Eval, forced: Forced, behaviors: Behaviors, rng: ChaCha8Rng, cfg: OracleCfg) -> Self {
        Oracle {
            ev,
            forced,
            behaviors,
            rng,
            cfg,
            line: None,
            spent: (0, 0, false),
            history: PhaseHistory::default(),
            lines: 0,
            proved: false,
            followed: Vec::new(),
        }
    }

    fn kit(&self) -> Kit<'_> {
        Kit { ev: &self.ev, forced: &self.forced, behaviors: &self.behaviors, cfg: &self.cfg, proved: self.proved }
    }

    fn line_from(&mut self, rules: &Rules, state: &GameState, first: Option<&Action>, salt: u64) -> Option<Line> {
        self.lines += 1;
        self.kit().line(rules, state, first, salt)
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
        let kit = Kit { ev: &self.ev, forced: &self.forced, behaviors: &self.behaviors, cfg: &self.cfg, proved: self.proved };
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
    behaviors: &'a Behaviors,
    cfg: &'a OracleCfg,
    /// The card to prove has been played on the real table.
    proved: bool,
}

/// 243: a move that takes the card to prove a step toward play: drafting, keeping or playing it.
fn takes(prove: &str, a: &Action) -> bool {
    matches!(a, Action::Draft { card, .. } | Action::Keep { card, .. } | Action::Play { card, .. } if card.as_str() == prove)
}

impl Kit<'_> {
    /// The rollout policy: the planner's, guessing from the state's own
    /// actions count, moved by `salt` (0: the planner's own seed).
    fn policy(&self, g: &GameState, salt: u64) -> Box<dyn Agent + Send> {
        let c = self.cfg;
        let seed = (g.seed ^ g.stats.actions as u64)
            .wrapping_add(salt.wrapping_mul(0x9E37_79B9_7F4A_7C15))
            .wrapping_add(c.salt.wrapping_mul(0xD1B5_4A32_D192_ED03));
        let truth = c.truth > 1 || (c.truth == 1 && salt == 0);
        rollout_policy(self.ev, self.forced, self.behaviors, seed, c.samples, c.choices, c.prune, c.powers, c.answers, c.discards, truth)
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
            let prove = self.cfg.prove.as_deref();
            let played = |a: &Action| prove.is_some_and(|p| matches!(a, Action::Play { card, .. } if card.as_str() == p));
            let mut proved = self.proved || first.is_some_and(played);
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
                // A proof's line takes the card at every step it is offered (until played once).
                let rush = if proved { None } else { prove.and_then(|p| legal.iter().position(|a| takes(p, a))) };
                let i = match rush {
                    Some(i) => i,
                    None if legal.len() == 1 => 0,
                    None => agent.choose(rules, &g, &legal),
                };
                if prof::time(Part::ApplyMut, || rules.apply_mut(&mut g, &legal[i], None)).is_err() {
                    break;
                }
                proved |= played(&legal[i]);
                actions.push(legal[i].clone());
            }
            marks.push((u32::MAX, self.ev.value(rules, &g)));
            let won = g.outcome.as_ref().is_some_and(|o| o.won) && (prove.is_none() || proved);
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
        if self.cfg.worlds > 0 {
            return self.choose_real(rules, state, legal);
        }
        self.sync(rules, state);
        if legal.len() == 1 {
            self.note_play(&legal[0]);
            return 0;
        }
        self.history.enter(state);
        if self.spent.0 != state.round {
            self.spent = (state.round, 0, false);
        }
        let how = Scoring::decision(self.cfg.samples, 1, self.cfg.choices).discards(self.cfg.discards);
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
        self.note_play(&legal[i]);
        i
    }
}

/// 243, a card proof on the real table: the keep plan around the oracle keeps by its own sketch, so this
/// takes the card's draft, keep and play first, as a proof's lines do, until the card is played once.
pub struct Rush {
    inner: Box<dyn Agent + Send>,
    prove: String,
    played: bool,
}

impl Rush {
    pub fn new(inner: Box<dyn Agent + Send>, prove: String) -> Self {
        Rush { inner, prove, played: false }
    }
}

impl Agent for Rush {
    fn choose(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize {
        let rush = if self.played { None } else { legal.iter().position(|a| takes(&self.prove, a)) };
        let i = rush.unwrap_or_else(|| self.inner.choose(rules, state, legal));
        self.played |= matches!(&legal[i], Action::Play { card, .. } if card.as_str() == self.prove);
        i
    }
}

/// 243, restarts (`tries`): at its first decision the oracle plays whole games on copies of the table,
/// each a fresh oracle, and plays the first that wins (else the first). The first try is the oracle as
/// specified, so the game it plays is the one a single oracle plays wherever that one wins; the rest
/// move every line's policy seed (`salt`) and switch `truth`. Its wins are a superset of one try's.
pub struct Tries {
    tries: Vec<super::AgentSpec>,
    seed: u64,
    plan: Option<Vec<Action>>,
    next: usize,
}

impl Tries {
    pub fn new(tries: Vec<super::AgentSpec>, seed: u64) -> Self {
        Tries { tries, seed, plan: None, next: 0 }
    }

    fn search(&self, rules: &Rules, state: &GameState) -> Vec<Action> {
        let mut first = None;
        let mut legal = Vec::new();
        for spec in &self.tries {
            let mut g = state.clone();
            let mut agent = spec.build(rules, self.seed);
            let mut actions = Vec::new();
            // As the experiment's game loop: every decision goes to the agent.
            while actions.len() < 100_000 {
                rules.legal_actions_into(&g, &mut legal);
                if legal.is_empty() {
                    break;
                }
                let i = agent.choose(rules, &g, &legal);
                if rules.apply_mut(&mut g, &legal[i], None).is_err() {
                    break;
                }
                actions.push(legal[i].clone());
            }
            if g.outcome.as_ref().is_some_and(|o| o.won) {
                return actions;
            }
            first.get_or_insert(actions);
        }
        first.unwrap_or_default()
    }
}

impl Agent for Tries {
    fn choose(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize {
        if self.plan.is_none() {
            self.plan = Some(self.search(rules, state));
        }
        let a = self.plan.as_ref().and_then(|p| p.get(self.next));
        self.next += 1;
        a.and_then(|a| legal.iter().position(|b| b == a)).expect("a try replays on the real table")
    }
}

/// 249: a candidate's standing over the worlds: worlds won, then the mean heuristic at round `at`.
fn standing(lines: &[Line], at: u32) -> (usize, f64) {
    let won = lines.iter().filter(|l| l.won).count();
    let v = lines.iter().map(|l| l.key(at).1).sum::<f64>() / lines.len().max(1) as f64;
    (won, v)
}

impl Oracle {
    /// 249: moves each followed line on to `state` by its own moves, dropping one whose world no longer agrees
    /// with the table (the table showed another card than its guess, or another move was taken).
    fn sync_worlds(&mut self, rules: &Rules, state: &GameState) {
        prof::time(Part::Sync, || {
            self.followed.retain_mut(|l| {
                while l.cursor.stats.actions < state.stats.actions {
                    let Some(a) = l.actions.get(l.next) else { return false };
                    if rules.apply_mut(&mut l.cursor, a, None).is_err() {
                        return false;
                    }
                    l.next += 1;
                }
                l.cursor.stats.actions == state.stats.actions && super::determinize::agrees(&l.cursor, state)
            });
        })
    }

    /// 249: the followed lines' next move here, the one most of them play (the first such line's on a tie).
    fn vote(&self, legal: &[Action]) -> Option<usize> {
        let mut counts = vec![0usize; legal.len()];
        let mut first = vec![usize::MAX; legal.len()];
        for (k, l) in self.followed.iter().enumerate() {
            if let Some(i) = l.actions.get(l.next).and_then(|a| legal.iter().position(|b| b == a)) {
                counts[i] += 1;
                first[i] = first[i].min(k);
            }
        }
        (0..legal.len()).filter(|&i| counts[i] > 0).max_by(|&x, &y| counts[x].cmp(&counts[y]).then(first[y].cmp(&first[x])))
    }

    /// 249, the real oracle: the oracle's search on worlds sampled from what the players can see, never the
    /// true table. At a decision of the action phase, while the round's `budget` (lines, a world each) lasts and
    /// the followed lines do not win on every world, it samples `worlds` worlds ([`super::determinize`]), plays
    /// each candidate (the followed lines' move, then the engine's shortlist: `alts` more within `margin`) to
    /// the game's end on every world, and takes the one that wins on most, then the best mean heuristic
    /// `horizon` rounds on; a candidate that wins on every world ends the search. Between searches it plays the
    /// move most followed lines play next; a line is dropped once the table shows what its world guessed
    /// otherwise. With none left, the engine's pick. No restarts in a fair player: a restart reads the true game's
    /// loss (314 takes `tries` here as a measuring arm only). 314: `phases` searches every phase, the draft too;
    /// `lone` plays a lone candidate unsearched.
    /// On a pending choice its worlds follow `determinize`'s pending rule: a step that drew a pile out and
    /// reshuffled keeps the table's streams in its snapshot (rare), within the agents' standing exception (151),
    /// whose shortlist scores the true table there.
    fn choose_real(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize {
        self.sync_worlds(rules, state);
        if legal.len() == 1 {
            return 0;
        }
        self.history.enter(state);
        if self.spent.0 != state.round {
            self.spent = (state.round, 0, false);
        }
        let k = self.cfg.worlds;
        let how = Scoring::decision(self.cfg.samples, 1, self.cfg.choices).discards(self.cfg.discards);
        let guesses = draw_guesses(rules, &mut self.rng, state, self.cfg.samples);
        let at = state.round + self.cfg.horizon;
        let vote = self.vote(legal);
        let secure = vote.is_some() && self.followed.len() == k && self.followed.iter().all(|l| l.won);
        let mut chosen = None;
        // 314 (`phases`): outside the action phase only a decision with no pending prompt is searched, so a world
        // never keeps the table's streams for a step that drew out a pile (`determinize`'s pending rule).
        let phase = searched_phase(state) || (self.cfg.phases > 0 && state.pending.is_none());
        if !secure && phase && self.spent.1 + k <= self.cfg.budget {
            let worlds: Vec<GameState> = (0..k)
                .map(|_| {
                    let mut w = state.clone();
                    super::determinize::determinize(rules, &mut w, &mut self.rng);
                    w
                })
                .collect();
            let mut scores = prof::time(Part::Top, || score_guesses(rules, &self.ev, state, &guesses, legal, how));
            self.history.mask(legal, &mut scores);
            let engine = pick(legal, &scores);
            let mut order: Vec<usize> = (0..legal.len()).filter(|&i| scores[i].is_finite()).collect();
            order.sort_by(|&x, &y| scores[y].total_cmp(&scores[x]));
            let top = scores[engine];
            let mut cands: Vec<usize> = vote.into_iter().collect();
            let mut seen: Vec<Option<String>> = cands.iter().map(|&i| idea(&legal[i])).collect();
            for i in order {
                if cands.len() > self.cfg.alts || scores[i] < top - self.cfg.margin {
                    break;
                }
                let key = idea(&legal[i]);
                if key.is_none() || seen.contains(&key) {
                    continue;
                }
                seen.push(key);
                cands.push(i);
            }
            // 314 (`lone`): a lone candidate is no choice: play it, and keep the round's lines for a real one.
            let lone = self.cfg.lone > 0 && cands.len() == 1;
            if lone {
                chosen = Some(cands[0]);
                cands.clear();
            }
            let mut best: Option<(usize, (usize, f64), Vec<Line>)> = None;
            for i in cands {
                if self.spent.1 + k > self.cfg.budget {
                    break;
                }
                self.spent.1 += k;
                let lines = self.lines_on(rules, &worlds, &legal[i]);
                if lines.is_empty() {
                    continue;
                }
                let s = standing(&lines, at);
                if best.as_ref().is_none_or(|b| s.0 > b.1 .0 || (s.0 == b.1 .0 && s.1 > b.1 .1)) {
                    best = Some((i, s, lines));
                }
                if s.0 == k {
                    break;
                }
            }
            if let Some((i, _, lines)) = best {
                chosen = Some(i);
                self.followed = lines;
            } else if !lone {
                chosen = Some(engine);
            }
        }
        let i = chosen.or(vote).unwrap_or_else(|| {
            let mut scores = score_guesses(rules, &self.ev, state, &guesses, legal, how);
            self.history.mask(legal, &mut scores);
            pick(legal, &scores)
        });
        self.history.push(&legal[i]);
        i
    }

    /// 249: the lines opening with `first` on each world, `threads` at a time (the same lines at any count);
    /// a world that refuses the move (none should: the move is the table's) gives none.
    fn lines_on(&mut self, rules: &Rules, worlds: &[GameState], first: &Action) -> Vec<Line> {
        let kit = Kit { ev: &self.ev, forced: &self.forced, behaviors: &self.behaviors, cfg: &self.cfg, proved: self.proved };
        self.lines += worlds.len();
        let t = self.cfg.threads.max(1);
        let mut out = Vec::new();
        for batch in worlds.chunks(t) {
            let got: Vec<Option<Line>> = if batch.len() == 1 {
                vec![kit.line(rules, &batch[0], Some(first), 0)]
            } else {
                std::thread::scope(|sc| {
                    let hs: Vec<_> = batch.iter().map(|w| sc.spawn(move || kit.line(rules, w, Some(first), 0))).collect();
                    hs.into_iter().map(|h| h.join().expect("a line panicked")).collect()
                })
            };
            out.extend(got.into_iter().flatten());
        }
        out
    }

    /// A card proof's card played on the real table: every line from here counts it.
    fn note_play(&mut self, a: &Action) {
        if let (Some(p), Action::Play { card, .. }) = (&self.cfg.prove, a) {
            self.proved |= card.as_str() == p;
        }
    }
}
