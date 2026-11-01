//! The module framework. A game is a set of modules, each in one variant;
//! each variant declares its tunable parameters, the phases it contributes
//! to the round, and the RNG purposes it draws from, and builds a [`Module`]
//! (its typed, resolved parameters plus behavior) from config.
//!
//! See `docs/architecture.md` for how to add a module or a variant.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::action::Action;
use crate::config::ParamValue;
use crate::content::Content;
use crate::event::Event;
use crate::key::{FastMap, Key};
use crate::map::Map;
use crate::rng::Stream;
use crate::vocab::Resource;
use crate::state::{Ask, GameState, Opt, Outcome, Pick};
use crate::view::ViewOut;

/// A module: an id, a one-line summary and its variants. Registered in
/// `modules::REGISTRY`.
pub struct ModuleDef {
    pub id: &'static str,
    pub summary: &'static str,
    /// Modules whose parameters this one reads (through [`World::int`]);
    /// a config without them is rejected.
    pub requires: &'static [&'static str],
    pub variants: &'static [VariantDef],
}

pub struct VariantDef {
    pub name: &'static str,
    pub summary: &'static str,
    pub params: &'static [ParamSpec],
    pub phases: &'static [PhaseSpec],
    /// RNG purposes this variant draws from. Each gets its own stream.
    pub rng: &'static [&'static str],
    /// Whether the variant plays content cards (config `cards`).
    pub uses_cards: bool,
    pub build: fn(&Build) -> Result<Box<dyn Module>, String>,
}

#[derive(Debug, Clone, Copy)]
pub struct ParamSpec {
    pub name: &'static str,
    pub kind: ParamKind,
    pub min: i64,
    pub max: i64,
    pub doc: &'static str,
}

impl ParamSpec {
    /// An integer parameter in `min..=max`.
    pub const fn int(name: &'static str, min: i64, max: i64, doc: &'static str) -> ParamSpec {
        ParamSpec {
            name,
            kind: ParamKind::Int,
            min,
            max,
            doc,
        }
    }

    /// A list parameter, each entry in `min..=max`.
    pub const fn list(name: &'static str, min: i64, max: i64, doc: &'static str) -> ParamSpec {
        ParamSpec {
            name,
            kind: ParamKind::List,
            min,
            max,
            doc,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    Int,
    /// A non-empty list; a single integer in config is read as a one-item list.
    List,
}

impl ParamKind {
    /// `"int"` or `"list"` (the registry description, tunables).
    pub fn name(self) -> &'static str {
        match self {
            ParamKind::Int => "int",
            ParamKind::List => "list",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhaseSpec {
    pub name: &'static str,
    pub kind: PhaseKind,
    pub doc: &'static str,
}

impl PhaseSpec {
    pub const fn auto(name: &'static str, doc: &'static str) -> PhaseSpec {
        PhaseSpec {
            name,
            kind: PhaseKind::Auto,
            doc,
        }
    }

    pub const fn decision(name: &'static str, doc: &'static str) -> PhaseSpec {
        PhaseSpec {
            name,
            kind: PhaseKind::Decision,
            doc,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseKind {
    /// Resolved by the module as soon as it is reached; no player input
    /// except the choices it asks ([`Cx::choose`]).
    Auto,
    /// Simultaneous player phase: every seat may act, in any order, until
    /// every seat has passed.
    Decision,
}

impl PhaseKind {
    /// `"auto"` or `"decision"` (the registry description, the view).
    pub fn name(self) -> &'static str {
        match self {
            PhaseKind::Auto => "auto",
            PhaseKind::Decision => "decision",
        }
    }
}

/// Why resolution stopped early.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Halt {
    /// The action is not legal. Before any choice is answered, the engine
    /// restores the state from before it; after one, it panics
    /// ([`Module::act`]).
    Illegal(String),
    /// A choice is needed ([`Cx::choose`] recorded the question).
    Ask,
}

impl From<String> for Halt {
    fn from(e: String) -> Self {
        Halt::Illegal(e)
    }
}

impl From<&str> for Halt {
    fn from(e: &str) -> Self {
        Halt::Illegal(e.to_string())
    }
}

pub type Res<T = ()> = Result<T, Halt>;

/// Behavior of a built module. Phase arguments are indices into the
/// variant's `phases`. Every method is deterministic: randomness only via
/// [`Cx::rng`], and only from purposes the variant declared, or
/// [`Cx::deck_rng`] for a card deck's reshuffle.
///
/// `run`, `act` and `after` may stop for a choice (`Cx::choose` returning
/// `Err(Halt::Ask)`, passed on with `?`). They are then run again from the
/// start, from the state before them, with the answers so far, so they must
/// depend only on the state and the answers.
pub trait Module: Send + Sync + Debug {
    /// Once, when the game is created, before round 1.
    fn setup(&self, _g: &mut GameState, _cx: &mut Cx) {}
    /// At the start of every round, before its first phase.
    fn round_start(&self, _g: &mut GameState, _cx: &mut Cx) {}
    /// Resolve an `Auto` phase.
    fn run(&self, _phase: usize, _g: &mut GameState, _cx: &mut Cx) -> Res {
        Ok(())
    }
    /// A `Decision` phase begins (every seat is un-passed). The module may
    /// prepare it, and pass seats that have nothing to decide.
    fn begin(&self, _phase: usize, _g: &mut GameState, _cx: &mut Cx) {}
    /// Append `seat`'s legal actions (other than `Pass`, which the engine
    /// adds if `can_pass` allows) in a `Decision` phase.
    fn legal(&self, _phase: usize, _g: &GameState, _w: &World, _seat: u8, _out: &mut Vec<Action>) {}
    /// Whether `seat` may pass now (default: always).
    fn can_pass(&self, _phase: usize, _g: &GameState, _seat: u8) -> bool {
        true
    }
    /// Whether a non-pass action in this decision phase is this module's to
    /// handle. Several modules can share a decision phase (`"a.x+b.y"` in the
    /// round); the engine routes each action to the module that owns it.
    fn owns(&self, _phase: usize, _action: &Action) -> bool {
        false
    }
    /// Validate and apply a non-pass action this module owns. On
    /// `Halt::Illegal` the engine restores the state from before it, but only
    /// if no choice has been answered: after the first answer it is a bug
    /// and panics, so do every legality check before the first `choose`.
    fn act(&self, _phase: usize, _g: &mut GameState, _cx: &mut Cx, action: &Action) -> Res {
        Err(Halt::Illegal(format!(
            "this phase takes no action like {action:?}"
        )))
    }
    /// After every action and every automatic phase: standing checks
    /// (thresholds, the end of the game).
    fn after(&self, _g: &mut GameState, _cx: &mut Cx) -> Res {
        Ok(())
    }
    /// Add derived values, card groups or piles to the generic view (see
    /// `view.rs`). Tokens, resources, `vars` and exhausted cards already
    /// show without this. Display only: the rules never read the view.
    fn view(&self, _g: &GameState, _w: &World, _out: &mut ViewOut) {}
}

/// Read-only game data shared by every game of a config.
#[derive(Clone, Copy)]
pub struct World<'a> {
    pub map: &'a Map,
    pub content: &'a Content,
    /// Every module's resolved parameters, keyed `"<module>.<param>"`.
    pub params: &'a FastMap<ParamValue>,
    /// Each configured module's variant, by module id.
    pub variants: &'a FastMap<String>,
}

impl World<'_> {
    /// The variant a module is configured in, if the module is configured.
    /// For rules that depend on another module's variant (threats acting
    /// on whichever adversary pieces the adversary variant uses).
    pub fn variant(&self, module: &str) -> Option<&str> {
        self.variants.get(module).map(|v| v.as_str())
    }

    /// An integer parameter of any configured module, `"<module>.<param>"`.
    /// Panics if absent: a module may read only parameters of modules it
    /// `requires`, which the config is checked for.
    pub fn int(&self, key: &str) -> i64 {
        match self.params.get(key) {
            Some(ParamValue::Int(v)) => *v,
            Some(ParamValue::List(_)) => panic!("parameter {key} is not an int"),
            None => panic!("parameter {key} is not configured"),
        }
    }

    pub fn list(&self, key: &str) -> &[i64] {
        match self.params.get(key) {
            Some(ParamValue::List(v)) => v,
            Some(ParamValue::Int(_)) => panic!("parameter {key} is not a list"),
            None => panic!("parameter {key} is not configured"),
        }
    }

}

/// The answers given so far while resolving one phase or action, and the
/// question it stopped on.
#[derive(Debug, Default)]
pub struct Choices {
    pub(crate) made: Vec<Pick>,
    pub(crate) used: usize,
    pub(crate) asked: Option<Ask>,
}

impl Choices {
    pub(crate) fn new(made: Vec<Pick>) -> Self {
        Choices {
            made,
            used: 0,
            asked: None,
        }
    }

    /// Replaying answers already given: their events were logged then.
    fn replaying(&self) -> bool {
        self.used < self.made.len()
    }
}

/// What a module gets while it resolves: the world, its own id (for
/// [`Cx::count`] and the RNG panic message), its RNG purposes, the event log
/// and the choices of the current resolution.
pub struct Cx<'a> {
    pub world: World<'a>,
    pub module: &'static str,
    pub(crate) purposes: &'static [&'static str],
    pub(crate) events: Option<&'a mut Vec<Event>>,
    pub(crate) choices: &'a mut Choices,
}

impl<'a> Cx<'a> {
    pub fn emit(&mut self, e: impl FnOnce() -> Event) {
        if self.choices.replaying() {
            return;
        }
        if let Some(log) = self.events.as_deref_mut() {
            log.push(e());
        }
    }

    /// A log line, optionally about a hex.
    pub fn note(&mut self, hex: Option<usize>, text: impl FnOnce() -> String) {
        let at = hex.map(|h| self.world.map.hexes[h].hex);
        self.emit(|| Event::Note {
            text: text(),
            hex: at,
        });
    }

    /// Asks `seat` to choose one of `options` (players choose). `None` if
    /// there are no options; a single option is taken without asking.
    /// Otherwise returns the answer if it was given, else stops resolution
    /// with `Halt::Ask` (pass it on with `?`).
    pub fn choose(&mut self, seat: usize, prompt: &str, options: Vec<Opt>) -> Res<Option<usize>> {
        match options.len() {
            0 => return Ok(None),
            1 => return Ok(Some(0)),
            _ => {}
        }
        let ch = &mut *self.choices;
        if ch.used < ch.made.len() {
            let p = &ch.made[ch.used];
            let i = options
                .iter()
                .position(|o| o.option == p.option && o.target == p.target)
                .ok_or_else(|| {
                    Halt::Illegal(format!("{:?} is not an option of {prompt:?}", p.option))
                })?;
            ch.used += 1;
            return Ok(Some(i));
        }
        let ask = Ask {
            seat: seat as u8,
            prompt: prompt.to_string(),
            options,
        };
        if let Some(log) = self.events.as_deref_mut() {
            log.push(Event::Asked {
                seat: seat as u8,
                prompt: prompt.to_string(),
            });
        }
        ch.asked = Some(ask);
        Err(Halt::Ask)
    }

    /// Adds `delta` tokens (negative removes, never below zero). Returns the
    /// change actually made.
    pub fn add_tokens(&mut self, g: &mut GameState, hex: usize, kind: &str, delta: i64) -> i64 {
        let cur = g.tokens(hex, kind);
        let next = (cur + delta).max(0);
        if next == cur {
            return 0;
        }
        let tokens = &mut Arc::make_mut(&mut g.hexes[hex]).tokens;
        if next == 0 {
            tokens.remove(kind);
        } else {
            tokens.insert(kind.into(), next);
        }
        let at = self.world.map.hexes[hex].hex;
        self.emit(|| Event::TokensChanged {
            hex: at,
            token: kind.to_string(),
            delta: next - cur,
            total: next,
        });
        next - cur
    }

    pub fn add_resource(&mut self, g: &mut GameState, seat: usize, r: Resource, delta: i64) {
        if delta == 0 {
            return;
        }
        let v = g.seats[seat].resources.entry(r).or_insert(0);
        *v += delta;
        let total = *v;
        self.emit(|| Event::ResourceChanged {
            seat: seat as u8,
            resource: r,
            delta,
            total,
        });
    }

    /// Adds `n` to the per-round counter `key` (full key) for the current
    /// round: entry `r` is round `r`, entry 0 setup.
    pub fn per_round(&mut self, g: &mut GameState, key: &str, n: i64) {
        let r = g.round as usize;
        let v = Arc::make_mut(g.stats.per_round.entry(key).or_default());
        if v.len() <= r {
            v.resize(r + 1, 0);
        }
        v[r] += n;
    }

    /// Adds to the counter `"<module>.<name>"`.
    pub fn count(&mut self, g: &mut GameState, name: &str, n: i64) {
        *g.stats
            .counters
            .entry(format!("{}.{name}", self.module))
            .or_insert(0) += n;
    }

    /// Adds to a counter by its full key (shared rules code counting for
    /// the module that owns the rule).
    pub fn tally(&mut self, g: &mut GameState, key: &str, n: i64) {
        *g.stats.counters.entry(key).or_insert(0) += n;
    }

    /// Raises a counter by its full key to `n` if it is lower (a peak).
    pub fn peak(&mut self, g: &mut GameState, key: &str, n: i64) {
        let e = g.stats.counters.entry(key).or_insert(0);
        *e = (*e).max(n);
    }

    /// Sets a counter by its full key to `n` (a level; the last one set stays).
    pub fn level(&mut self, g: &mut GameState, key: &str, n: i64) {
        g.stats.counters.insert(key.into(), n);
    }

    /// Sets any module's track by its full key (shared rules code acting
    /// for another module, such as a card setting a country's
    /// `board.shelter` flag).
    pub fn set_key(&mut self, g: &mut GameState, key: &str, v: i64) {
        let old = if v == 0 {
            g.vars.remove(key).unwrap_or(0)
        } else {
            g.vars.insert(key.into(), v).unwrap_or(0)
        };
        if old == v {
            return;
        }
        self.emit(|| Event::VarChanged {
            key: key.to_string(),
            delta: v - old,
            total: v,
        });
    }

    pub fn add_key(&mut self, g: &mut GameState, key: &str, delta: i64) {
        let v = g.var(key) + delta;
        self.set_key(g, key, v);
    }

    /// Moves card `card` from the pile `from` to the top (`top`) or bottom
    /// of `to`. `""` is the box (outside the game). Errors if it isn't there.
    pub fn move_card(
        &mut self,
        g: &mut GameState,
        card: &str,
        from: &str,
        to: &str,
        top: bool,
    ) -> Result<(), String> {
        if !from.is_empty() {
            let pile = g
                .piles
                .get_mut(from)
                .ok_or_else(|| format!("{card} is not in {from}"))?;
            let i = pile
                .iter()
                .position(|c| c == card)
                .ok_or_else(|| format!("{card} is not in {from}"))?;
            Arc::make_mut(pile).remove(i);
            if pile.is_empty() {
                g.piles.remove(from);
            }
        }
        if !to.is_empty() {
            let pile = Arc::make_mut(g.piles.entry(to).or_default());
            if top {
                pile.insert(0, card.into());
            } else {
                pile.push(card.into());
            }
        }
        self.emit(|| Event::CardMoved {
            card: card.to_string(),
            from: from.to_string(),
            to: to.to_string(),
        });
        Ok(())
    }

    /// Replaces a pile's order (a shuffle). No event: the order is hidden.
    pub fn set_pile(&mut self, g: &mut GameState, key: &str, cards: Vec<Key>) {
        if cards.is_empty() {
            g.piles.remove(key);
        } else {
            g.piles.insert(key.into(), Arc::new(cards));
        }
    }

    /// Marks a card used (face down) for `seat`. Refreshing is the owning
    /// module's job (usually in `round_start`).
    pub fn exhaust(&mut self, g: &mut GameState, seat: usize, card: &str) {
        let ex = &mut g.seats[seat].exhausted;
        if let Err(at) = ex.binary_search_by(|c| c.as_str().cmp(card)) {
            ex.insert(at, card.into());
        }
    }

    pub fn refresh(&mut self, g: &mut GameState, seat: usize, card: &str) {
        g.seats[seat].exhausted.retain(|c| c.as_str() != card);
    }

    /// The card decks' stream, for a reshuffle: a draw is an op, so it runs in
    /// whichever module's phase played it (046: Develop's draw in `ai.growth`
    /// emptied the tier-1 deck and panicked in [`Cx::rng`]).
    pub fn deck_rng<'g>(&self, g: &'g mut GameState) -> &'g mut Stream {
        Arc::make_mut(g.rng.get_mut("cards").expect("the cards module has a stream"))
    }

    /// The stream for one of this variant's declared purposes. Drawing from
    /// any other purpose, even one another module declared, panics.
    pub fn rng<'g>(&self, g: &'g mut GameState, purpose: &str) -> &'g mut Stream {
        if !self.purposes.contains(&purpose) {
            panic!(
                "module {} drew from RNG purpose {purpose:?}, which it did not declare",
                self.module
            );
        }
        Arc::make_mut(g.rng.get_mut(purpose).expect("declared purposes have streams"))
    }

    /// Ends the game (first call wins).
    pub fn end(&mut self, g: &mut GameState, won: bool, reason: &str, detail: String) {
        if g.outcome.is_some() {
            return;
        }
        let log = if self.choices.replaying() {
            None
        } else {
            self.events.as_deref_mut()
        };
        end_game(g, won, reason, detail, log);
    }
}

/// Sets the game's outcome and logs its `GameEnded` event to `log`, if any
/// (shared by [`Cx::end`] and the engine's round limit).
pub(crate) fn end_game(
    g: &mut GameState,
    won: bool,
    reason: &str,
    detail: String,
    log: Option<&mut Vec<Event>>,
) {
    let round = g.round;
    if let Some(log) = log {
        log.push(Event::GameEnded {
            won,
            round,
            reason: reason.to_string(),
            detail: detail.clone(),
        });
    }
    g.outcome = Some(Outcome {
        won,
        round,
        reason: reason.to_string(),
        detail,
    });
}

/// Input to a variant's `build`: its validated parameters and content.
pub struct Build<'a> {
    pub module: &'static str,
    pub params: &'a BTreeMap<String, ParamValue>,
    /// Content indices of the cards selected in config, in config order.
    pub cards: Vec<usize>,
    pub world: World<'a>,
}

impl Build<'_> {
    pub fn int(&self, name: &str) -> Result<i64, String> {
        match self.params.get(name) {
            Some(ParamValue::Int(v)) => Ok(*v),
            Some(ParamValue::List(_)) => Err(format!(
                "{}.{name}: expected an integer, got a list",
                self.module
            )),
            None => Err(format!("{}.{name}: missing", self.module)),
        }
    }

    pub fn list(&self, name: &str) -> Result<Vec<i64>, String> {
        match self.params.get(name) {
            Some(ParamValue::List(v)) => Ok(v.clone()),
            Some(ParamValue::Int(_)) => Err(format!(
                "{}.{name}: expected a list, got an integer",
                self.module
            )),
            None => Err(format!("{}.{name}: missing", self.module)),
        }
    }
}

/// Entry `round - 1` of a per-round schedule; the last entry repeats.
pub fn scheduled(schedule: &[i64], round: u32) -> i64 {
    let i = (round.max(1) as usize - 1).min(schedule.len() - 1);
    schedule[i]
}
