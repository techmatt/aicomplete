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
use crate::key::{FastMap, Key, KeyBuf};
use crate::map::Map;
use crate::rng::Stream;
use crate::state::{Ask, AskKind, GameState, Opt, Outcome, Pick};
use crate::view::ViewOut;
use crate::vocab::Resource;

/// A module: an id, a one-line summary and its variants. Registered in
/// `modules::REGISTRY`.
pub struct ModuleDef {
    pub id: &'static str,
    pub summary: &'static str,
    /// Modules this one reaches into (parameters through [`World::int`],
    /// tracks, piles); a config without them is rejected.
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
/// [`Cx::deck_rng`], [`Cx::threat_rng`] and [`Cx::adversary_rng`] (the project deck) for a deck's reshuffle.
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
    /// `ai.boards` (empty without the `ai` module), read once with the rules:
    /// each seat's board is asked for by every passive, track and capstone
    /// read, and the parameter's lookup by name was 2% of a search (103).
    pub ai_boards: &'a [i64],
    /// The int parameters read after every action and in every position, read
    /// once with the rules ([`Hot`], 104): by name they were 2% of a search.
    pub hot: &'a HotParams,
}

/// An int parameter read often enough to keep in a slot (104): read it with
/// [`World::hot`], which reads as [`World::int`] by its name does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hot {
    NodeLoss,
    ClearWinStage,
    TipPerPlayer,
    DiscordTipPerPlayer,
    MomentumEvery,
    TroopNode,
    LiberateAt,
}

impl Hot {
    const ALL: [Hot; 7] = [
        Hot::NodeLoss,
        Hot::ClearWinStage,
        Hot::TipPerPlayer,
        Hot::DiscordTipPerPlayer,
        Hot::MomentumEvery,
        Hot::TroopNode,
        Hot::LiberateAt,
    ];

    /// The parameter's name.
    pub fn key(self) -> &'static str {
        match self {
            Hot::NodeLoss => "alignment.node_loss",
            Hot::ClearWinStage => "alignment.clear_win_stage",
            Hot::TipPerPlayer => "alignment.tip_per_player",
            Hot::DiscordTipPerPlayer => "alignment.discord_tip_per_player",
            Hot::MomentumEvery => "alignment.momentum_every",
            Hot::TroopNode => "board.troop_node",
            Hot::LiberateAt => "board.liberate_at",
        }
    }
}

/// The [`Hot`] parameters' values, each `None` where its module isn't
/// configured; and whether the alignment module is in its `off` variant.
#[derive(Debug, Clone, Default)]
pub struct HotParams {
    ints: [Option<i64>; Hot::ALL.len()],
    pub alignment_off: bool,
}

impl HotParams {
    pub fn read(params: &FastMap<ParamValue>, variants: &FastMap<String>) -> HotParams {
        let mut ints = [None; Hot::ALL.len()];
        for (slot, h) in ints.iter_mut().zip(Hot::ALL) {
            *slot = match params.get(h.key()) {
                Some(ParamValue::Int(v)) => Some(*v),
                _ => None,
            };
        }
        HotParams {
            ints,
            alignment_off: variants.get("alignment").map(|v| v.as_str()) == Some("off"),
        }
    }
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

    /// A [`Hot`] parameter: `self.int(h.key())`, from its slot.
    #[inline]
    pub fn hot(&self, h: Hot) -> i64 {
        match self.hot.ints[h as usize] {
            Some(v) => v,
            None => self.int(h.key()),
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

/// A prompt's words for [`Cx::choose`]: a `&str`, or a closure building them
/// (104). Most prompts a search meets are replayed or have one option, and
/// their text is never read: `format!` there was 3% of a search (103).
pub trait PromptText {
    fn text(self) -> String;
}

impl PromptText for &str {
    fn text(self) -> String {
        self.to_string()
    }
}

impl PromptText for &String {
    fn text(self) -> String {
        self.clone()
    }
}

impl<F: FnOnce() -> String> PromptText for F {
    fn text(self) -> String {
        self()
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
    /// What another seat's trigger removed inside the action resolving (118: a
    /// teammate's Joint Task Force): ambassadors, troops, compute. Its fight
    /// triggers fired for it, so the acting seat's don't (`cards::fight_fire`).
    pub(crate) fought: [i64; 3],
}

impl<'a> Cx<'a> {
    /// Replaying answers already given: at the start of an action, that the
    /// table passed its checks when it was first played.
    pub fn replaying(&self) -> bool {
        self.choices.replaying()
    }

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
    /// The prompt is a [`PromptText`]: text, or a closure that builds it, run only
    /// when the question is really asked (a replay matches answers by label).
    pub fn choose(&mut self, seat: usize, prompt: impl PromptText, options: Vec<Opt>) -> Res<Option<usize>> {
        self.ask(seat, prompt, options, AskKind::Plain)
    }

    /// [`Cx::choose`] for a loss: which of your things goes, or where a loss lands.
    pub fn choose_loss(&mut self, seat: usize, prompt: impl PromptText, options: Vec<Opt>) -> Res<Option<usize>> {
        self.ask(seat, prompt, options, AskKind::Loss)
    }

    /// [`Cx::choose`] among `keys`, each an option labeled by its name (a card of a
    /// hand): the same answers, but a replay matches its answer without building the
    /// labels (174: a discard chain's replays built a hand of them for every answer).
    pub fn choose_key(&mut self, seat: usize, prompt: impl PromptText, keys: &[Key]) -> Res<Option<usize>> {
        match keys.len() {
            0 => return Ok(None),
            1 => return Ok(Some(0)),
            _ => {}
        }
        let ch = &mut *self.choices;
        if ch.replaying() {
            let p = &ch.made[ch.used];
            let i = keys
                .iter()
                .position(|k| p.target.is_none() && k.as_str() == p.option)
                .ok_or_else(|| Halt::Illegal(format!("{:?} is not an option of {:?}", p.option, prompt.text())))?;
            ch.used += 1;
            return Ok(Some(i));
        }
        let options = keys.iter().map(|k| Opt { option: k.as_str().to_string(), target: None }).collect();
        self.ask(seat, prompt, options, AskKind::Plain)
    }

    fn ask(&mut self, seat: usize, prompt: impl PromptText, options: Vec<Opt>, kind: AskKind) -> Res<Option<usize>> {
        match options.len() {
            0 => return Ok(None),
            1 => return Ok(Some(0)),
            _ => {}
        }
        let ch = &mut *self.choices;
        if ch.replaying() {
            let p = &ch.made[ch.used];
            let i = options
                .iter()
                .position(|o| o.option == p.option && o.target == p.target)
                .ok_or_else(|| {
                    Halt::Illegal(format!("{:?} is not an option of {:?}", p.option, prompt.text()))
                })?;
            ch.used += 1;
            return Ok(Some(i));
        }
        let prompt = prompt.text();
        if let Some(log) = self.events.as_deref_mut() {
            log.push(Event::Asked {
                seat: seat as u8,
                prompt: prompt.clone(),
            });
        }
        let ask = Ask {
            seat: seat as u8,
            prompt,
            options,
            kind,
        };
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
        let tokens = &mut Arc::make_mut(&mut Arc::make_mut(&mut g.hexes)[hex]).tokens;
        if next == 0 {
            tokens.remove(kind);
        } else {
            tokens.insert_str(kind, next);
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

    /// Adds `delta` of `r` to `seat`. No resource has a holding limit (127,
    /// Matt: a runaway pile is fixed at the card that feeds it, never by a ceiling).
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
        let v = Arc::make_mut(g.stats.per_round.entry_str(key).or_default());
        if v.len() <= r {
            v.resize(r + 1, 0);
        }
        v[r] += n;
    }

    /// Adds to the counter `"<module>.<name>"`.
    pub fn count(&mut self, g: &mut GameState, name: &str, n: i64) {
        self.tally(g, KeyBuf::new(self.module).text(".").text(name).as_str(), n);
    }

    /// Adds to a counter by its full key (shared rules code counting for
    /// the module that owns the rule).
    pub fn tally(&mut self, g: &mut GameState, key: &str, n: i64) {
        *g.stats.counters.entry_str(key).or_insert(0) += n;
    }

    /// Raises a counter by its full key to `n` if it is lower (a peak).
    pub fn peak(&mut self, g: &mut GameState, key: &str, n: i64) {
        let e = g.stats.counters.entry_str(key).or_insert(0);
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
            g.vars.insert_str(key, v).unwrap_or(0)
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
            let pile = Arc::make_mut(g.piles.entry_str(to).or_default());
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
    /// whichever module's phase played it (046: a draw outside `cards.*`
    /// emptied the tier-1 deck and panicked in [`Cx::rng`]).
    pub fn deck_rng<'g>(&self, g: &'g mut GameState) -> &'g mut Stream {
        stream(g, "cards")
    }

    /// The threats module's stream, for a threat deck reshuffled in another
    /// module's step (070: Discord cards and projects deal threats).
    pub fn threat_rng<'g>(&self, g: &'g mut GameState) -> &'g mut Stream {
        stream(g, "threats")
    }

    /// The adversary module's stream, for its project deck reshuffled in another
    /// module's step (146: compute an uncountered threat gives fills a project, and
    /// the refill reshuffles in `threats.execute`).
    pub fn adversary_rng<'g>(&self, g: &'g mut GameState) -> &'g mut Stream {
        stream(g, "adversary")
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
        stream(g, purpose)
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

/// The stream of a declared RNG purpose.
fn stream<'g>(g: &'g mut GameState, purpose: &str) -> &'g mut Stream {
    Arc::make_mut(g.rng.get_mut(purpose).expect("declared purposes have streams"))
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
