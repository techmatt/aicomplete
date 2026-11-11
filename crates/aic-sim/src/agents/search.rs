//! The searching agents: greedy, beam and rollout. See `mod.rs` for what
//! each does; everything here simulates on determinized copies of the
//! state, never on the state the agent was handed.

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::sync::Arc;

use aic_core::key::Fnv;
use aic_core::module::World;
use aic_core::modules::{adversary, ai, cards};
use aic_core::state::{AskKind, Pick};
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

/// Positions already valued in one scoring, among the answers to a loss or a
/// discard. A strike's losses come in a chain of prompts over the same pieces,
/// and destroying one and then another leaves the table the other order leaves:
/// 069 found these prompts, a few thousand a batch of games, were 46% of the
/// time scored, each option replaying the phase. Found by hash, confirmed by
/// [`same_position`], so a collision can't change a score. Only losses and
/// discards: on every prompt it cost more than it saved (planner games a minute
/// at c2: 32 to 26).
///
/// 162: a chain of discards ("discard which card (1 of 3)?") was 64-88% of
/// greedy's time: every order of the same cards scored again. Two orders leave
/// the same table but for the discard pile's order, which only a reshuffle
/// reads (`cards::deck`), and a reshuffle draws on the `cards` stream. So a
/// position matching but for that order takes the stored value only when no
/// state under the stored one moved that stream (`dirty` false): the same score
/// it would have computed.
#[derive(Default)]
struct Seen {
    table: HashMap<u64, Vec<Entry>, BuildHasherDefault<Fnv>>,
    /// Steps so far, in this scoring, that moved the `cards` stream (or reached
    /// a stored value whose own subtree did).
    moved: u64,
    /// Values by a discard chain's set of cards, found before any replay.
    chains: HashMap<u64, Vec<Chain>, BuildHasherDefault<Fnv>>,
    /// 174: levels a discard prompt is resolved at, at most (`discards`; `None`: as `choices`).
    discards: Option<usize>,
}

struct Entry {
    g: GameState,
    choices: usize,
    value: f64,
    /// A state under this one moved the `cards` stream.
    dirty: bool,
}

/// Whether `a`'s prompt is one the search remembers positions under.
fn remembered(a: &aic_core::state::Pending) -> bool {
    a.ask.kind == AskKind::Loss || a.ask.prompt.contains(cards::DISCARD_PROMPT)
}

fn discard_pile(key: &aic_core::Key) -> bool {
    key.as_str().starts_with(cards::DISCARD_PILES)
}

/// The `cards` stream (a reshuffle's) moved from `a` to `b`.
fn cards_moved(a: &GameState, b: &GameState) -> bool {
    match (a.rng.get("cards"), b.rng.get("cards")) {
        (Some(x), Some(y)) => !(Arc::ptr_eq(x, y) || x == y),
        (x, y) => x.is_some() != y.is_some(),
    }
}

/// A hash of what [`same_position`] compares, or enough of it: a discard
/// pile's cards in any order.
fn position_hash(g: &GameState, choices: usize) -> u64 {
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut h = choices as u64 ^ ((g.round as u64) << 8) ^ ((g.phase as u64) << 40);
    let mut mix = |x: u64| h = (h ^ x).wrapping_mul(PRIME).rotate_left(23);
    for hex in g.hexes.iter() {
        for (k, n) in hex.tokens.iter() {
            mix(k.as_str().as_ptr() as u64);
            mix(*n as u64);
        }
        mix(1);
    }
    for seat in &g.seats {
        for (_, n) in seat.resources.iter() {
            mix(*n as u64);
        }
    }
    for (k, n) in g.vars.iter() {
        mix(k.as_str().as_ptr() as u64);
        mix(*n as u64);
    }
    // 148: the piles too, which `same_position` compares: positions differing
    // only in piles all fell in one bucket, and a scan of deep compares went
    // quadratic (one Shadow Factories scoring, 6.2M compares: r62's strategist
    // 1.30x a decision, 1.14x with this; the memo is confirmed, so same play).
    for (k, v) in g.piles.iter() {
        mix(k.as_str().as_ptr() as u64);
        mix(v.len() as u64);
        if discard_pile(k) {
            let sum = v.iter().fold(0u64, |s, id| s.wrapping_add((id.as_str().as_ptr() as u64).wrapping_mul(PRIME)));
            mix(sum);
        } else {
            for id in v.iter() {
                mix(id.as_str().as_ptr() as u64);
            }
        }
    }
    h
}

/// How two positions compare in [`same_position`].
#[derive(PartialEq)]
enum Same {
    No,
    /// The same but for the order of a discard pile.
    ButDiscards,
    Exactly,
}

/// The piles of two positions: equal, or equal but for a discard pile's order.
fn same_piles(a: &GameState, b: &GameState) -> Same {
    if a.piles == b.piles {
        return Same::Exactly;
    }
    if a.piles.len() != b.piles.len() {
        return Same::No;
    }
    let sorted = |v: &[aic_core::Key]| {
        let mut ids: Vec<usize> = v.iter().map(|id| id.as_str().as_ptr() as usize).collect();
        ids.sort_unstable();
        ids
    };
    for ((ka, va), (kb, vb)) in a.piles.iter().zip(b.piles.iter()) {
        if ka != kb || va.len() != vb.len() {
            return Same::No;
        }
        if Arc::ptr_eq(va, vb) || va == vb {
            continue;
        }
        if !discard_pile(ka) || sorted(va) != sorted(vb) {
            return Same::No;
        }
    }
    Same::ButDiscards
}

/// Whether two tables reached by answering one prompt's options are the same
/// position: all that is scored and all a further answer can read, with the
/// same answers given in either order. The answers must match as a set, not
/// only the tables: a loop's progress lives in them alone (a hit Shelter absorbs
/// moves nothing, and a troop's march keeps its place in a local).
fn same_position(a: &GameState, b: &GameState) -> Same {
    // Cheapest compares first; the pending snapshot (a whole state) last (148).
    let same_answers = || match (&a.pending, &b.pending) {
        (Some(p), Some(q)) => {
            let count = |made: &[Pick], x: &Pick| made.iter().filter(|y| *y == x).count();
            p.ask == q.ask
                && p.trigger == q.trigger
                && p.made.len() == q.made.len()
                && p.made.iter().all(|x| count(&p.made, x) == count(&q.made, x))
                && (Arc::ptr_eq(&p.snapshot, &q.snapshot) || p.snapshot == q.snapshot)
        }
        (None, None) => true,
        _ => false,
    };
    let rest = a.round == b.round
        && a.phase == b.phase
        && a.outcome == b.outcome
        && a.rng == b.rng
        && a.vars == b.vars
        && a.seats == b.seats;
    if !rest {
        return Same::No;
    }
    let piles = same_piles(a, b);
    if piles == Same::No || a.hexes != b.hexes || a.stats.counters != b.stats.counters || !same_answers() {
        return Same::No;
    }
    piles
}

/// 162: where the chain of discards `p` asks for began, in its answers: its
/// own prompt reads "(k of n)" (a lone "discard which card?" is a chain of one),
/// and each earlier prompt of the chain had more cards to choose from, so each
/// recorded its answer. `None` if it isn't a discard.
fn discard_chain(p: &aic_core::state::Pending) -> Option<usize> {
    let at = p.ask.prompt.find(cards::DISCARD_PROMPT)?;
    let rest = &p.ask.prompt[at + cards::DISCARD_PROMPT.len()..];
    let k = match rest.strip_prefix(" (") {
        Some(r) => r.split(" of ").next()?.parse::<usize>().ok()?,
        None => 1,
    };
    p.made.len().checked_sub(k.checked_sub(1)?)
}

/// 162: a value by the cards a chain of discards has taken, as a set: the
/// cards of one chain, in any order, leave the same table but for the discard
/// pile's order, so a set already valued is not replayed at all. Kept only when
/// neither the pick's step nor its subtree moved the `cards` stream (a
/// reshuffle, the one reader of that order); nothing between a chain's picks
/// draws (`cards::DISCARD_PROMPT` says so to its sites).
struct Chain {
    snapshot: Arc<GameState>,
    trigger: aic_core::state::Trigger,
    /// The answers before the chain, in order.
    before: Vec<Pick>,
    /// The chain's cards with the new pick, sorted.
    set: Vec<String>,
    choices: usize,
    value: f64,
}

fn chain_hash(p: &aic_core::state::Pending, start: usize, set: &[&str], choices: usize) -> u64 {
    let mut h = Fnv::default();
    (Arc::as_ptr(&p.snapshot) as usize).hash(&mut h);
    start.hash(&mut h);
    choices.hash(&mut h);
    for x in &p.made[..start] {
        x.option.hash(&mut h);
    }
    for s in set {
        s.hash(&mut h);
    }
    h.finish()
}

/// The value of `g`, with any choice it is waiting on answered best-first
/// up to `choices` levels deep: an action's own choices are part of it.
fn settled(
    rules: &Rules,
    ev: &Eval,
    g: &GameState,
    choices: usize,
    legal: &mut Vec<Action>,
    seen: &mut Seen,
) -> f64 {
    let Some(pending) = g.pending.as_ref().filter(|_| !g.is_over() && choices > 0) else {
        return value(rules, ev, g);
    };
    // 174: a chain of discards scored every set of `choices` + 1 cards of the hand
    // (H³/6 positions a step at 2): `discards` caps the levels a discard prompt takes.
    let choices = match seen.discards {
        Some(cap) if cap < choices && pending.ask.prompt.contains(cards::DISCARD_PROMPT) => cap,
        _ => choices,
    };
    if choices == 0 {
        return value(rules, ev, g);
    }
    let memo = remembered(pending);
    let chain = if memo { discard_chain(pending) } else { None };
    legal_into(rules, g, legal);
    let opts = std::mem::take(legal);
    let mut best = f64::NEG_INFINITY;
    for a in &opts {
        let mut by_set = None;
        if let (Some(start), Action::Choose { option, target: None, .. }) = (chain, a) {
            let mut set: Vec<&str> = pending.made[start..].iter().map(|x| x.option.as_str()).collect();
            set.push(option.as_str());
            set.sort_unstable();
            let key = chain_hash(pending, start, &set, choices);
            let hit = seen.chains.get(&key).and_then(|bucket| {
                bucket.iter().find(|e| {
                    e.choices == choices
                        && Arc::ptr_eq(&e.snapshot, &pending.snapshot)
                        && e.trigger == pending.trigger
                        && e.before[..] == pending.made[..start]
                        && e.set.iter().map(String::as_str).eq(set.iter().copied())
                })
            });
            if let Some(e) = hit {
                best = best.max(e.value);
                continue;
            }
            by_set = Some((key, start, set.iter().map(|s| s.to_string()).collect::<Vec<_>>()));
        }
        let Some(c) = step(rules, g, a) else { continue };
        let moved = cards_moved(g, &c);
        if moved {
            seen.moved += 1;
        }
        let (v, dirty) = if memo {
            let key = position_hash(&c, choices);
            let found = seen.table.get(&key).and_then(|bucket| {
                bucket.iter().find_map(|e| {
                    if e.choices != choices {
                        return None;
                    }
                    match same_position(&e.g, &c) {
                        Same::Exactly => Some((e.value, e.dirty)),
                        Same::ButDiscards if !e.dirty => Some((e.value, false)),
                        _ => None,
                    }
                })
            });
            match found {
                Some((v, dirty)) => {
                    seen.moved += dirty as u64;
                    (v, dirty)
                }
                None => {
                    let before = seen.moved;
                    let v = settled(rules, ev, &c, choices - 1, legal, seen);
                    let dirty = seen.moved != before;
                    seen.table.entry(key).or_default().push(Entry { g: c, choices, value: v, dirty });
                    (v, dirty)
                }
            }
        } else {
            (settled(rules, ev, &c, choices - 1, legal, seen), false)
        };
        if let Some((key, start, set)) = by_set {
            if !(dirty || moved) {
                seen.chains.entry(key).or_default().push(Chain {
                    snapshot: pending.snapshot.clone(),
                    trigger: pending.trigger.clone(),
                    before: pending.made[..start].to_vec(),
                    set,
                    choices,
                    value: v,
                });
            }
        }
        best = best.max(v);
    }
    if best.is_finite() {
        best
    } else {
        value(rules, ev, g)
    }
}

/// Best value reachable from `g` in `depth` more actions (0 = `g` itself).
fn lookahead(rules: &Rules, ev: &Eval, g: &GameState, depth: usize, choices: usize, seen: &mut Seen) -> f64 {
    let mut legal = Vec::new();
    if depth == 0 || g.is_over() {
        return settled(rules, ev, g, choices, &mut legal, seen);
    }
    legal_into(rules, g, &mut legal);
    let best = legal
        .iter()
        .filter_map(|a| step(rules, g, a))
        .map(|c| lookahead(rules, ev, &c, depth - 1, choices, seen))
        .fold(f64::NEG_INFINITY, f64::max);
    if best.is_finite() {
        best
    } else {
        settled(rules, ev, g, choices, &mut legal, seen)
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
    /// `choices` for a power that places nodes.
    pub placing_choices: usize,
    /// `choices` for a power with several sacrifices before its effect (068).
    pub sacrificing_choices: usize,
    /// `choices` for an answer to an open prompt.
    pub answer_choices: usize,
    /// 174: levels a discard prompt ("discard which card") is resolved at, at most
    /// (`usize::MAX`: as any other prompt).
    pub discard_choices: usize,
}

impl Scoring {
    /// At the top level of a decision: a node power at twice `choices`
    /// (041, 066): a track and a hex, after up to two sacrifices. 046: not inside
    /// rollouts, at about 3.7 evaluations a decision.
    pub fn decision(samples: usize, depth: usize, choices: usize) -> Self {
        let placing_choices = choices * 2;
        Scoring {
            samples,
            depth,
            choices,
            placing_choices,
            sacrificing_choices: placing_choices + SACRIFICE_PROMPTS,
            answer_choices: choices,
            discard_choices: usize::MAX,
        }
    }

    /// 174: a discard prompt resolved at most `levels` deep (99: no cap).
    pub fn discards(self, levels: usize) -> Self {
        Scoring { discard_choices: if levels >= 99 { usize::MAX } else { levels }, ..self }
    }

    /// Inside a rollout: a node power at no more levels than it asks (046,
    /// 066: a sacrifice, a track and a hex; short of a level, the action reads
    /// as paying and placing nothing, so a rollout never takes it).
    pub fn in_rollout(self) -> Self {
        // 068: not the expensive powers' full depth: three sacrifices, a card and a
        // slot, scored for every action of a rollout, doubled the planner's cost.
        let placing_choices = self.choices.max(PLACING_PROMPTS);
        Scoring { placing_choices, sacrificing_choices: placing_choices, ..self }
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
    let guesses = draw_guesses(rules, rng, state, how.samples);
    score_guesses(rules, ev, state, &guesses, legal, how)
}

/// 151: a guess of the hidden information, except in the middle of a resolution (a pending choice): there the
/// replay of the answers runs through hidden piles and hands (a project's "each player pays N or discards M
/// cards", answered card by card, for every seat), and a guess that changes a pile or a hand the resolution
/// reads replays a different line, so the recorded answers no longer fit (screen 3 and F2 each lost a game to a
/// replay panic). The leak is the true table for one pending choice.
fn guess_unless_pending(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    if g.pending.is_none() {
        determinize(rules, g, rng);
    }
}

/// The guesses a scoring of `state` averages, drawn from `rng`: none with
/// `samples` 0, which uses the state as it is. Apart from scoring so a caller
/// that may not need the scores (the planner past its budget, 069) still takes
/// the same draws, and so plays the same.
pub fn draw_guesses(rules: &Rules, rng: &mut ChaCha8Rng, state: &GameState, samples: usize) -> Vec<GameState> {
    (0..samples)
        .map(|_| {
            let mut d = state.clone();
            guess_unless_pending(rules, &mut d, rng);
            d
        })
        .collect()
}

/// [`greedy_scores`] on guesses already drawn; profiled, tallied by the
/// prompt it answers (`prof::prompt`).
pub fn score_guesses(
    rules: &Rules,
    ev: &Eval,
    state: &GameState,
    guesses: &[GameState],
    legal: &[Action],
    how: Scoring,
) -> Vec<f64> {
    let prompt = state.pending.as_ref().map(|p| p.ask.prompt.as_str());
    prof::prompt(prompt, || score_all(rules, ev, state, guesses, legal, how))
}

fn score_all(
    rules: &Rules,
    ev: &Eval,
    state: &GameState,
    guesses: &[GameState],
    legal: &[Action],
    how: Scoring,
) -> Vec<f64> {
    let Scoring { samples, depth, choices, placing_choices, sacrificing_choices, answer_choices, discard_choices } = how;
    super::eval::new_decision();
    let world = rules.world();
    let mut scores = vec![0.0; legal.len()];
    let n = samples.max(1);
    let bases: &[GameState] = if samples == 0 { std::slice::from_ref(state) } else { guesses };
    let has_pass = legal.iter().any(|a| matches!(a, Action::Pass { .. }));
    for base in bases {
        let mut seen = Seen { discards: (discard_choices != usize::MAX).then_some(discard_choices), ..Seen::default() };
        // A pass is scored as the table it leaves, not what follows it: the
        // last pass hands the round on to threats and the adversary, and
        // scoring that would make stalling (pay, withdraw, pay...) look
        // better than stopping. Only worked out for a menu with a pass: a
        // prompt's has none, and settling one here scored all its options
        // twice (069).
        let stay = if has_pass { settled(rules, ev, base, choices, &mut Vec::new(), &mut seen) } else { 0.0 };
        for (i, a) in legal.iter().enumerate() {
            let part = match a {
                Action::Power { .. } => Part::ScorePower,
                Action::Play { .. } => Part::ScorePlay,
                Action::Standard { .. } => Part::ScoreStandard,
                Action::Choose { .. } => Part::ScoreChoose,
                _ => Part::ScoreOther,
            };
            scores[i] += prof::time(part, || match a {
                Action::Pass { .. } => stay,
                _ => match step(rules, base, a) {
                    Some(c) if sacrifices_many(&world, a) => lookahead(rules, ev, &c, depth - 1, sacrificing_choices, &mut seen),
                    Some(c) if places(&world, a) => lookahead(rules, ev, &c, depth - 1, placing_choices, &mut seen),
                    Some(c) if matches!(a, Action::Choose { .. }) => lookahead(rules, ev, &c, depth - 1, answer_choices, &mut seen),
                    Some(c) => lookahead(rules, ev, &c, depth - 1, choices, &mut seen),
                    None => f64::NEG_INFINITY,
                },
            }) + ev.bonus(&world, a);
        }
    }
    scores.iter_mut().for_each(|s| *s /= n as f64);
    scores
}

/// The prompts of a power that places: a sacrifice's, a track, a hex.
const PLACING_PROMPTS: usize = 3;

/// Whether `a` is a power that places nodes.
fn places(world: &World, a: &Action) -> bool {
    matches!(a, Action::Power { seat, power } if ai::power_places(world, *seat as usize, *power as usize))
}

/// The prompts an expensive power adds to a placement's (068): up to two more
/// sacrifices, with what it does (a building and its slot, or a country) the
/// placement's two.
const SACRIFICE_PROMPTS: usize = 2;

/// Whether `a` is a power with several sacrifice prompts before its effect:
/// short of them it reads as losing the cards for nothing.
fn sacrifices_many(world: &World, a: &Action) -> bool {
    matches!(a, Action::Power { seat, power } if ai::power_sacrifices_many(world, *seat as usize, *power as usize))
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
    /// rollouts, 028). A dropped action stays out for the whole phase, even
    /// after the seat's resources change: a card it could pay for but ranked
    /// below the cut at that scoring stays out even if it becomes the best play
    /// (one it could not yet pay for was not on the menu, so is not dropped). So
    /// this is strength, not only speed: at `prune` 6 the line alone won 29% at c3
    /// against 81% at 12 (054).
    prune: usize,
    /// With `prune`: powers each seat keeps through a phase of a rollout.
    powers: usize,
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
            powers: usize::MAX,
            dropped: None,
        }
    }

    pub fn pruned(mut self, prune: usize) -> Self {
        self.prune = prune;
        self
    }

    /// With `pruned`, also at most `powers` power actions a seat for the phase.
    pub fn powers(mut self, powers: usize) -> Self {
        self.powers = powers;
        self
    }

    /// An answer to a prompt looks at most `levels` prompts on (069).
    pub fn answers(mut self, levels: usize) -> Self {
        self.how.answer_choices = self.how.answer_choices.min(levels);
        self
    }

    /// 174: a discard prompt resolved at most `levels` deep (99: no cap).
    pub fn discards(mut self, levels: usize) -> Self {
        self.how = self.how.discards(levels);
        self
    }

    /// As a rollout's policy: see [`Scoring::in_rollout`].
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
            let mut powers = [0usize; 256];
            let mut dropped = HashSet::new();
            for i in ranked {
                let a = &legal[i];
                let n = &mut taken[a.seat() as usize];
                *n += 1;
                let mut over = *n > self.prune;
                if matches!(a, Action::Power { .. }) {
                    let p = &mut powers[a.seat() as usize];
                    *p += 1;
                    over |= *p > self.powers;
                }
                if over && !matches!(a, Action::Pass { .. }) {
                    dropped.insert(a.clone());
                }
            }
            self.dropped = Some(dropped);
        }
        out
    }
}

impl super::Strategic for Greedy {
    fn set_strategies(&mut self, c: super::strategy::Choice) {
        self.ev.set_strategies(c);
    }

    fn eval(&self) -> Option<&super::eval::Eval> {
        Some(&self.ev)
    }
}

impl super::Strategic for Beam {
    fn set_strategies(&mut self, c: super::strategy::Choice) {
        self.ev.set_strategies(c);
    }

    fn eval(&self) -> Option<&super::eval::Eval> {
        Some(&self.ev)
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
struct Seq {
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
    fn search(&mut self, rules: &Rules, state: &GameState) -> (GameState, Vec<(f64, Seq)>) {
        let mut start = state.clone();
        guess_unless_pending(rules, &mut start, &mut self.rng);
        let round = start.round;
        let mut beam = vec![Seq {
            g: start.clone(),
            seq: Vec::new(),
            key: 0,
        }];
        let mut finished: Vec<(f64, Seq)> = Vec::new();
        let mut legal = Vec::new();
        let mut scratch = Vec::new();
        for _ in 0..self.depth {
            let mut children: Vec<(f64, Seq)> = Vec::new();
            for item in &beam {
                legal_into(rules, &item.g, &mut legal);
                let mut kids: Vec<(f64, Seq)> = Vec::new();
                for a in &legal {
                    if undoes(&item.seq, a) {
                        continue;
                    }
                    let Some(c) = step(rules, &item.g, a) else {
                        continue;
                    };
                    // The choices an action asks for are scored best-first,
                    // so an action is not judged by its first open question.
                    let mut seq = item.seq.clone();
                    seq.push(a.clone());
                    let v =
                        settled(rules, &self.ev, &c, self.choices, &mut scratch, &mut Seen::default()) - moves_cost(&seq);
                    let key = item.key.wrapping_add(action_hash(a));
                    kids.push((v, Seq { g: c, seq, key }));
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
            guess_unless_pending(rules, &mut s, &mut self.rng);
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
            for (i, (value, item)) in finished.iter().enumerate().take(cfg.candidates) {
                let v = if item.g.is_over() {
                    *value
                } else {
                    self.playouts(rules, &item.g, &cfg)
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
