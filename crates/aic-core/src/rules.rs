//! [`Rules`]: a config compiled against the module registry and content.
//! Build once per config, then run any number of games with it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use serde::Serialize;

use crate::action::Action;
use crate::config::{Config, ModuleConfig, ParamValue};
use crate::content::{overlaid, Content};
use crate::event::Event;
use crate::hash::fnv1a64;
use crate::key::{FastMap, Key, Table};
use crate::map::Map;
use crate::module::{
    end_game, Build, Choices, Cx, Halt, Module, ModuleDef, ParamKind, ParamSpec, PhaseKind, Res,
    VariantDef, World,
};
use crate::modules;
use crate::rng::Stream;
use crate::state::{GameState, HexState, Pending, Pick, Seat, Stats, Trigger};
use crate::view::{self, View, ViewOut};

/// Hard stop for configs whose round never ends the game.
pub const MAX_ROUNDS: u32 = 1000;

pub struct Rules {
    /// The resolved config: validated, lists normalized, `round` filled in.
    /// Each game takes its own copy; states of one game share that copy.
    pub config: Arc<Config>,
    pub config_hash: String,
    /// Of the content these rules play: the embedded files, overlay applied.
    pub content_hash: &'static str,
    pub map: Map,
    pub round: Vec<PhaseRef>,
    content: &'static Content,
    modules: Vec<Instance>,
    /// Every module's resolved parameters, `"<module>.<param>"`.
    params: FastMap<ParamValue>,
    /// Each configured module's variant, by module id.
    variants: FastMap<String>,
    /// `ai.boards`, read once (see [`World::ai_boards`]).
    ai_boards: Vec<i64>,
}

/// The `ai.boards` parameter, or nothing without the `ai` module.
fn ai_boards(params: &FastMap<ParamValue>) -> Vec<i64> {
    match params.get("ai.boards") {
        Some(ParamValue::List(v)) => v.clone(),
        _ => Vec::new(),
    }
}

/// One phase of the round: one module's phase, or several modules'
/// decision phases played as one simultaneous phase (`"a.play+b.build"`).
#[derive(Debug, Clone)]
pub struct PhaseRef {
    pub parts: Vec<PhasePart>,
    /// The round entry, e.g. `"board.act"`.
    pub name: String,
    pub kind: PhaseKind,
}

#[derive(Debug, Clone, Copy)]
pub struct PhasePart {
    /// Index into the config's modules.
    pub module: usize,
    /// Index into that variant's phases.
    pub phase: usize,
}

/// A tunable of a configured module, with its current value.
#[derive(Debug, Clone, Serialize)]
pub struct Tunable {
    /// `"<module>.<param>"`, the key `Config::apply_override` takes.
    pub key: String,
    pub kind: &'static str,
    pub min: i64,
    pub max: i64,
    pub doc: &'static str,
    pub value: ParamValue,
}

struct Instance {
    id: &'static str,
    variant: &'static VariantDef,
    module: Box<dyn Module>,
}

/// A module to build: its definition, variant and resolved cards.
type ToBuild = (&'static ModuleDef, &'static VariantDef, Vec<usize>);

/// Finds each configured module and variant, and resolves its cards and
/// parameters into `resolved`.
fn resolve_modules(resolved: &mut Config, content: &'static Content) -> Result<Vec<ToBuild>, String> {
    let mut to_build = Vec::new();
    let mut purposes = BTreeSet::new();
    let mut card_owner: BTreeMap<usize, &'static str> = BTreeMap::new();
    for mc in resolved.modules.iter_mut() {
        let def = modules::find(&mc.module).ok_or_else(|| {
            let known: Vec<&str> = modules::REGISTRY.iter().map(|m| m.id).collect();
            format!(
                "unknown module {:?} (known: {})",
                mc.module,
                known.join(", ")
            )
        })?;
        if to_build
            .iter()
            .any(|(d, ..): &(&ModuleDef, _, _)| d.id == def.id)
        {
            return Err(format!("module {:?} appears twice", def.id));
        }
        let variant = def
            .variants
            .iter()
            .find(|v| v.name == mc.variant)
            .ok_or_else(|| {
                let known: Vec<&str> = def.variants.iter().map(|v| v.name).collect();
                format!(
                    "module {}: unknown variant {:?} (known: {})",
                    def.id,
                    mc.variant,
                    known.join(", ")
                )
            })?;
        for p in variant.rng {
            if !purposes.insert(*p) {
                return Err(format!(
                    "RNG purpose {p:?} is declared by two modules; each purpose needs its own stream"
                ));
            }
        }
        let cards = resolve_cards(def.id, variant, mc, content)?;
        for &c in &cards {
            if let Some(other) = card_owner.insert(c, def.id) {
                return Err(format!(
                    "card {:?} is used by both {other} and {}; a card belongs to one module",
                    content.cards[c].id, def.id
                ));
            }
        }
        resolve_params(def.id, variant.params, mc)?;
        to_build.push((def, variant, cards));
    }
    Ok(to_build)
}

/// The round's entries as phase references, each checked against the
/// modules in the config.
fn compile_round(names: &[String], instances: &[Instance]) -> Result<Vec<PhaseRef>, String> {
    let mut round = Vec::new();
    for name in names {
        let mut parts = Vec::new();
        let mut kinds = Vec::new();
        for part in name.split('+') {
            let (m, p) = part
                .split_once('.')
                .ok_or_else(|| format!("round entry {name:?}: expected <module>.<phase>"))?;
            let module = instances.iter().position(|i| i.id == m).ok_or_else(|| {
                format!("round entry {name:?}: module {m:?} is not in the config")
            })?;
            let phases = instances[module].variant.phases;
            let phase = phases.iter().position(|s| s.name == p).ok_or_else(|| {
                let known: Vec<&str> = phases.iter().map(|s| s.name).collect();
                format!(
                    "round entry {name:?}: {m} has no phase {p:?} (has: {})",
                    known.join(", ")
                )
            })?;
            if parts.iter().any(|q: &PhasePart| q.module == module) {
                return Err(format!("round entry {name:?}: {m} appears twice"));
            }
            parts.push(PhasePart { module, phase });
            kinds.push(phases[phase].kind);
        }
        if parts.len() > 1 && kinds.iter().any(|&k| k != PhaseKind::Decision) {
            return Err(format!(
                "round entry {name:?}: only decision phases can be combined with '+'"
            ));
        }
        round.push(PhaseRef {
            parts,
            name: name.clone(),
            kind: kinds[0],
        });
    }
    if round.is_empty() {
        return Err("the round has no phases".into());
    }
    Ok(round)
}

impl Rules {
    pub fn new(config: &Config) -> Result<Rules, String> {
        let (content, content_hash) = overlaid(&config.content)?;
        let map = Map::build(content, config.countries)?;

        let mut resolved = config.clone();
        let mut instances = Vec::new();
        let to_build = resolve_modules(&mut resolved, content)?;
        let mut params = FastMap::default();
        for mc in &resolved.modules {
            for (k, v) in &mc.params {
                params.insert(format!("{}.{k}", mc.module), v.clone());
            }
        }
        let variants: FastMap<String> = resolved
            .modules
            .iter()
            .map(|mc| (mc.module.clone(), mc.variant.clone()))
            .collect();
        let ids: Vec<&str> = to_build.iter().map(|(d, ..)| d.id).collect();
        for (def, ..) in &to_build {
            for r in def.requires {
                if !ids.contains(r) {
                    return Err(format!(
                        "module {} needs module {r}, which is not in the config",
                        def.id
                    ));
                }
            }
        }
        let ai_boards = ai_boards(&params);
        let world = World {
            map: &map,
            content,
            params: &params,
            variants: &variants,
            ai_boards: &ai_boards,
        };
        for ((def, variant, cards), mc) in to_build.into_iter().zip(&resolved.modules) {
            let b = Build {
                module: def.id,
                params: &mc.params,
                cards,
                world,
            };
            let module = (variant.build)(&b).map_err(|e| format!("module {}: {e}", def.id))?;
            instances.push(Instance {
                id: def.id,
                variant,
                module,
            });
        }

        if resolved.round.is_empty() {
            for i in &instances {
                for p in i.variant.phases {
                    resolved.round.push(format!("{}.{}", i.id, p.name));
                }
            }
        }
        let round = compile_round(&resolved.round, &instances)?;

        let config_hash = resolved.hash();
        Ok(Rules {
            config: Arc::new(resolved),
            config_hash,
            content_hash,
            map,
            round,
            content,
            modules: instances,
            params,
            variants,
            ai_boards,
        })
    }

    pub fn world(&self) -> World<'_> {
        World {
            map: &self.map,
            content: self.content,
            params: &self.params,
            variants: &self.variants,
            ai_boards: &self.ai_boards,
        }
    }

    fn cx<'a>(
        &'a self,
        i: &Instance,
        events: Option<&'a mut Vec<Event>>,
        choices: &'a mut Choices,
    ) -> Cx<'a> {
        Cx {
            world: self.world(),
            module: i.id,
            purposes: i.variant.rng,
            events,
            choices,
        }
    }

    /// Every tunable of this config, in module then declaration order.
    /// What a sweep can vary with `--set`.
    pub fn tunables(&self) -> Vec<Tunable> {
        let mut out = Vec::new();
        for (i, mc) in self.modules.iter().zip(&self.config.modules) {
            for s in i.variant.params {
                out.push(Tunable {
                    key: format!("{}.{}", i.id, s.name),
                    kind: s.kind.name(),
                    min: s.min,
                    max: s.max,
                    doc: s.doc,
                    value: mc.params[s.name].clone(),
                });
            }
        }
        out
    }

    /// A fresh game, advanced to the first decision.
    pub fn new_game(&self, seed: u64) -> GameState {
        self.new_game_logged(seed, None)
    }

    pub fn new_game_logged(&self, seed: u64, mut events: Option<&mut Vec<Event>>) -> GameState {
        let mut rng = Table::default();
        for i in &self.modules {
            for p in i.variant.rng {
                rng.insert((*p).into(), Arc::new(Stream::new(seed, p)));
            }
        }
        let n = self.config.countries as usize;
        let seat = Seat::default();
        let mut g = GameState {
            seed,
            // A game's own copy: every state clone counts a reference to it, and
            // one count shared by all workers is a cache line they fight over (030).
            config: Arc::new((*self.config).clone()),
            round: 0,
            phase: self.round.len(),
            seats: vec![seat; n],
            // One empty hex shared until written: copy on write, as `state.rs` says.
            #[allow(clippy::rc_clone_in_vec_init)]
            hexes: vec![Arc::new(HexState::default()); self.map.hexes.len()],
            vars: Table::default(),
            piles: Table::default(),
            pending: None,
            rng,
            stats: Stats::default(),
            outcome: None,
        };
        let mut ch = Choices::default();
        for i in &self.modules {
            i.module
                .setup(&mut g, &mut self.cx(i, events.as_deref_mut(), &mut ch));
        }
        self.enter(&mut g, events);
        g
    }

    /// Checks that a state (say, one deserialized from a client) fits these
    /// rules, so a mismatch is an error rather than a panic.
    pub fn check_state(&self, g: &GameState) -> Result<(), String> {
        if g.config != self.config {
            return Err("state was built from a different config".into());
        }
        if g.seats.len() != self.config.countries as usize || g.hexes.len() != self.map.hexes.len()
        {
            return Err("state does not fit the board (seat or hex count)".into());
        }
        let want: BTreeSet<&str> = self
            .modules
            .iter()
            .flat_map(|i| i.variant.rng.iter().copied())
            .collect();
        let have: BTreeSet<&str> = g.rng.keys().map(Key::as_str).collect();
        if want != have {
            return Err(format!(
                "state RNG streams {have:?} do not match the config's {want:?}"
            ));
        }
        for (p, s) in &g.rng {
            if s.stream_id() != fnv1a64(p.as_bytes()) {
                return Err(format!("RNG stream {p:?} has the wrong stream id"));
            }
        }
        if let Some(p) = &g.pending {
            let snap = &p.snapshot;
            let kind = self.round.get(g.phase).map(|r| r.kind);
            let trigger_fits = match p.trigger {
                Trigger::Phase => kind == Some(PhaseKind::Auto),
                Trigger::Action { .. } => kind == Some(PhaseKind::Decision),
            };
            if snap.pending.is_some()
                || snap.config != self.config
                || snap.seats.len() != g.seats.len()
                || snap.hexes.len() != g.hexes.len()
                || snap.phase != g.phase
                || snap.rng.keys().ne(g.rng.keys())
                || p.ask.seat as usize >= g.seats.len()
                || p.ask.options.len() < 2
                || !trigger_fits
            {
                return Err("state has a malformed pending choice".into());
            }
        }
        if !g.is_over()
            && g.pending.is_none()
            && self
                .round
                .get(g.phase)
                .is_none_or(|p| p.kind != PhaseKind::Decision)
        {
            return Err(format!("state is not at a decision (phase {})", g.phase));
        }
        Ok(())
    }

    /// The current phase, or `None` once the game is over.
    pub fn phase(&self, g: &GameState) -> Option<&PhaseRef> {
        if g.is_over() {
            None
        } else {
            self.round.get(g.phase)
        }
    }

    /// The generic view of a state, for clients (see `view.rs`).
    pub fn view(&self, g: &GameState) -> View {
        let mut v = view::generic(self, g);
        let w = self.world();
        for i in &self.modules {
            i.module.view(
                g,
                &w,
                &mut ViewOut {
                    module: i.id,
                    view: &mut v,
                },
            );
        }
        view::finish(&mut v, g, self.content);
        v
    }

    pub fn legal_actions(&self, g: &GameState) -> Vec<Action> {
        let mut out = Vec::new();
        self.legal_actions_into(g, &mut out);
        out
    }

    /// Clears `out` and fills it with every legal action, seat by seat
    /// (each seat's actions from each module of the phase, then its pass).
    /// Empty once the game is over.
    pub fn legal_actions_into(&self, g: &GameState, out: &mut Vec<Action>) {
        out.clear();
        let Some(p) = self.phase(g) else { return };
        if let Some(pend) = &g.pending {
            for o in &pend.ask.options {
                out.push(Action::Choose {
                    seat: pend.ask.seat,
                    option: o.option.clone(),
                    target: o.target,
                });
            }
            return;
        }
        debug_assert_eq!(p.kind, PhaseKind::Decision);
        let w = self.world();
        for (s, seat) in g.seats.iter().enumerate() {
            if seat.passed {
                continue;
            }
            for part in &p.parts {
                self.modules[part.module]
                    .module
                    .legal(part.phase, g, &w, s as u8, out);
            }
            if self.can_pass(p, g, s as u8) {
                out.push(Action::Pass { seat: s as u8 });
            }
        }
    }

    fn can_pass(&self, p: &PhaseRef, g: &GameState, seat: u8) -> bool {
        p.parts.iter().all(|part| {
            self.modules[part.module]
                .module
                .can_pass(part.phase, g, seat)
        })
    }

    /// Applies an action in place. An action refused before any choice was
    /// answered is an error and leaves the state unchanged; `Halt::Illegal`
    /// after one is a bug and panics (see [`Module::act`]). Pass `None` for
    /// `events` in hot loops to skip building the event log.
    pub fn apply_mut(
        &self,
        g: &mut GameState,
        a: &Action,
        events: Option<&mut Vec<Event>>,
    ) -> Result<(), String> {
        match self.check(g, a)? {
            Step::Answer(pick) => {
                let Pending {
                    trigger,
                    mut made,
                    snapshot,
                    ..
                } = Arc::unwrap_or_clone(g.pending.take().unwrap());
                made.push(pick);
                *g = (*snapshot).clone();
                self.resolve(g, Snap::Shared(snapshot), trigger, made, events)
            }
            Step::Pass => {
                self.pass(g, a.seat() as usize, events);
                Ok(())
            }
            Step::Act => {
                let snapshot = Arc::new(g.clone());
                let trigger = Trigger::Action { action: a.clone() };
                let r = self.resolve(
                    g,
                    Snap::Shared(snapshot.clone()),
                    trigger,
                    Vec::new(),
                    events,
                );
                if r.is_err() {
                    *g = Arc::unwrap_or_clone(snapshot);
                }
                r
            }
        }
    }

    /// `g` after `a`, as a new state; `g` is untouched. The same as cloning
    /// and [`Rules::apply_mut`], with one state copy where that makes two
    /// (the copy, and the snapshot an action keeps in case it asks a
    /// choice): what a search should call.
    pub fn apply_to(
        &self,
        g: &GameState,
        a: &Action,
        events: Option<&mut Vec<Event>>,
    ) -> Result<GameState, String> {
        match self.check(g, a)? {
            Step::Answer(pick) => {
                let pend = g.pending.as_ref().unwrap();
                let mut made = pend.made.clone();
                made.push(pick);
                let mut next = (*pend.snapshot).clone();
                let snap = Snap::Shared(pend.snapshot.clone());
                self.resolve(&mut next, snap, pend.trigger.clone(), made, events)?;
                Ok(next)
            }
            Step::Pass => {
                let mut next = g.clone();
                self.pass(&mut next, a.seat() as usize, events);
                Ok(next)
            }
            Step::Act => {
                let mut next = g.clone();
                let trigger = Trigger::Action { action: a.clone() };
                self.resolve(&mut next, Snap::Borrowed(g), trigger, Vec::new(), events)?;
                Ok(next)
            }
        }
    }

    /// What `a` does on `g`, or why it can't be played.
    fn check(&self, g: &GameState, a: &Action) -> Result<Step, String> {
        let p = self.phase(g).ok_or("the game is over")?;
        let seat = a.seat() as usize;
        if seat >= g.seats.len() {
            return Err(format!("no seat {seat}"));
        }
        if let Some(pend) = &g.pending {
            let Action::Choose { option, target, .. } = a else {
                return Err(format!("waiting for a choice: {}", pend.ask.prompt));
            };
            if seat != pend.ask.seat as usize
                || !pend
                    .ask
                    .options
                    .iter()
                    .any(|o| &o.option == option && &o.target == target)
            {
                return Err(format!("not an option of {:?}: {a:?}", pend.ask.prompt));
            }
            return Ok(Step::Answer(Pick {
                option: option.clone(),
                target: *target,
            }));
        }
        if matches!(a, Action::Choose { .. }) {
            return Err("there is no choice to make".into());
        }
        if g.seats[seat].passed {
            return Err(format!("seat {seat} has passed this phase"));
        }
        if let Action::Pass { .. } = a {
            if !self.can_pass(p, g, seat as u8) {
                return Err(format!("seat {seat} can't pass yet"));
            }
            return Ok(Step::Pass);
        }
        Ok(Step::Act)
    }

    fn pass(&self, g: &mut GameState, seat: usize, mut events: Option<&mut Vec<Event>>) {
        g.seats[seat].passed = true;
        if let Some(log) = events.as_deref_mut() {
            log.push(Event::Passed { seat: seat as u8 });
        }
        g.stats.actions += 1;
        if g.seats.iter().all(|s| s.passed) {
            g.phase += 1;
            self.enter(g, events);
        }
    }

    /// Resolves a phase or an action (then every module's `after`) on `g`,
    /// which equals `snapshot`, replaying the answers in `made`. Stops at
    /// the next unanswered choice, or goes on to the next decision. On an
    /// illegal action `g` is left part-resolved: the caller restores it.
    fn resolve(
        &self,
        g: &mut GameState,
        snapshot: Snap,
        trigger: Trigger,
        made: Vec<Pick>,
        mut events: Option<&mut Vec<Event>>,
    ) -> Result<(), String> {
        let mut ch = Choices::new(made);
        let r = self.run_trigger(g, &trigger, &mut ch, events.as_deref_mut());
        let answered = ch.made.len() as u32;
        let is_action = matches!(trigger, Trigger::Action { .. });
        match r {
            Ok(()) => {
                g.stats.actions += answered + is_action as u32;
                if !is_action || g.seats.iter().all(|s| s.passed) {
                    g.phase += 1;
                    self.enter(g, events);
                }
                Ok(())
            }
            Err(Halt::Ask) => {
                g.stats.actions += answered + is_action as u32;
                park(g, trigger, ch, snapshot.share());
                Ok(())
            }
            Err(Halt::Illegal(e)) => {
                if !is_action || answered > 0 {
                    panic!("resolution failed on replay: {e}");
                }
                Err(e)
            }
        }
    }

    /// One phase or action, then every module's `after`.
    fn run_trigger(
        &self,
        g: &mut GameState,
        trigger: &Trigger,
        ch: &mut Choices,
        mut events: Option<&mut Vec<Event>>,
    ) -> Res {
        let p = &self.round[g.phase];
        match trigger {
            Trigger::Phase => {
                let part = p.parts[0];
                let inst = &self.modules[part.module];
                inst.module
                    .run(part.phase, g, &mut self.cx(inst, events.as_deref_mut(), ch))?;
            }
            Trigger::Action { action } => {
                let part = p
                    .parts
                    .iter()
                    .find(|q| self.modules[q.module].module.owns(q.phase, action))
                    .ok_or_else(|| format!("no module in phase {} takes {action:?}", p.name))?;
                let inst = &self.modules[part.module];
                inst.module.act(
                    part.phase,
                    g,
                    &mut self.cx(inst, events.as_deref_mut(), ch),
                    action,
                )?;
            }
        }
        for i in &self.modules {
            if g.is_over() {
                break;
            }
            i.module
                .after(g, &mut self.cx(i, events.as_deref_mut(), ch))?;
        }
        Ok(())
    }

    /// Applies an action to a copy; returns the new state and its events.
    pub fn apply(&self, g: &GameState, a: &Action) -> Result<(GameState, Vec<Event>), String> {
        let mut events = Vec::new();
        let next = self.apply_to(g, a, Some(&mut events))?;
        Ok((next, events))
    }

    /// Runs phases from `g.phase` until a decision phase is entered or the
    /// game ends, starting new rounds as needed.
    pub(crate) fn enter(&self, g: &mut GameState, mut events: Option<&mut Vec<Event>>) {
        loop {
            if g.is_over() {
                return;
            }
            if g.phase >= self.round.len() {
                if g.round >= MAX_ROUNDS {
                    let detail = format!("no module ended the game within {MAX_ROUNDS} rounds");
                    end_game(g, false, "round_limit", detail, events.as_deref_mut());
                    return;
                }
                g.round += 1;
                g.phase = 0;
                let round = g.round;
                if let Some(log) = events.as_deref_mut() {
                    log.push(Event::RoundStarted { round });
                }
                let mut ch = Choices::default();
                for i in &self.modules {
                    i.module
                        .round_start(g, &mut self.cx(i, events.as_deref_mut(), &mut ch));
                }
                if g.is_over() {
                    return;
                }
            }
            let p = &self.round[g.phase];
            if let Some(log) = events.as_deref_mut() {
                log.push(Event::PhaseStarted {
                    phase: p.name.clone(),
                });
            }
            match p.kind {
                PhaseKind::Auto => {
                    let mut ch = Choices::default();
                    let snapshot = g.clone();
                    match self.run_trigger(g, &Trigger::Phase, &mut ch, events.as_deref_mut()) {
                        Ok(()) => g.phase += 1,
                        Err(Halt::Ask) => {
                            park(g, Trigger::Phase, ch, Arc::new(snapshot));
                            return;
                        }
                        Err(Halt::Illegal(e)) => panic!("phase {} failed: {e}", p.name),
                    }
                }
                PhaseKind::Decision => {
                    for s in g.seats.iter_mut() {
                        s.passed = false;
                    }
                    let mut ch = Choices::default();
                    for part in &p.parts {
                        let inst = &self.modules[part.module];
                        inst.module.begin(
                            part.phase,
                            g,
                            &mut self.cx(inst, events.as_deref_mut(), &mut ch),
                        );
                    }
                    if g.is_over() {
                        return;
                    }
                    if g.seats.iter().all(|s| s.passed) {
                        g.phase += 1;
                        continue;
                    }
                    return;
                }
            }
        }
    }
}

/// Stops a resolution at the question it asked, to go on from `snapshot`
/// once it is answered.
fn park(g: &mut GameState, trigger: Trigger, ch: Choices, snapshot: Arc<GameState>) {
    g.pending = Some(Arc::new(Pending {
        trigger,
        made: ch.made,
        ask: ch.asked.expect("a halted choice records its question"),
        snapshot,
    }));
}

/// What a checked action does.
enum Step {
    /// Answers the pending choice.
    Answer(Pick),
    Pass,
    /// Resolves through the phase's module.
    Act,
}

/// The state a resolution started from, kept if it stops at a choice.
enum Snap<'a> {
    Shared(Arc<GameState>),
    /// Copied only if a choice is asked.
    Borrowed(&'a GameState),
}

impl Snap<'_> {
    fn share(self) -> Arc<GameState> {
        match self {
            Snap::Shared(s) => s,
            Snap::Borrowed(g) => Arc::new(g.clone()),
        }
    }
}

fn resolve_cards(
    module: &str,
    variant: &VariantDef,
    mc: &ModuleConfig,
    content: &Content,
) -> Result<Vec<usize>, String> {
    if !variant.uses_cards && !mc.cards.is_empty() {
        return Err(format!(
            "module {module}: variant {} takes no cards",
            variant.name
        ));
    }
    let mut out = Vec::new();
    for id in &mc.cards {
        let i = content
            .card_index(id)
            .ok_or_else(|| format!("module {module}: unknown card {id:?}"))?;
        if out.contains(&i) {
            return Err(format!("module {module}: card {id:?} listed twice"));
        }
        out.push(i);
    }
    Ok(out)
}

/// Checks the config's parameters against `specs` and normalizes them
/// (a single integer for a list parameter becomes a one-item list).
fn resolve_params(module: &str, specs: &[ParamSpec], mc: &mut ModuleConfig) -> Result<(), String> {
    for name in mc.params.keys() {
        if name == "variant" || !specs.iter().any(|s| s.name == *name) {
            let known: Vec<&str> = specs.iter().map(|s| s.name).collect();
            return Err(format!(
                "{module}.{name}: not a parameter of variant {} (parameters: {})",
                mc.variant,
                if known.is_empty() {
                    "none".to_string()
                } else {
                    known.join(", ")
                }
            ));
        }
    }
    for s in specs {
        let v = mc.params.get_mut(s.name).ok_or_else(|| {
            format!(
                "{module}.{}: missing (every parameter must be set in config: {})",
                s.name, s.doc
            )
        })?;
        let check = |x: i64| -> Result<(), String> {
            if x < s.min || x > s.max {
                Err(format!(
                    "{module}.{}: {x} is outside {}..={}",
                    s.name, s.min, s.max
                ))
            } else {
                Ok(())
            }
        };
        match (s.kind, &*v) {
            (ParamKind::Int, ParamValue::Int(x)) => check(*x)?,
            (ParamKind::Int, ParamValue::List(_)) => {
                return Err(format!(
                    "{module}.{}: expected an integer, got a list",
                    s.name
                ))
            }
            (ParamKind::List, ParamValue::Int(x)) => {
                check(*x)?;
                *v = ParamValue::List(vec![*x]);
            }
            (ParamKind::List, ParamValue::List(xs)) => {
                if xs.is_empty() {
                    return Err(format!("{module}.{}: list is empty", s.name));
                }
                xs.iter().try_for_each(|&x| check(x))?;
            }
        }
    }
    Ok(())
}
