//! Keeping by plan, and the forced behaviors of the dominance checks (022).
//!
//! [`Policy`] wraps a greedy, engine or planner agent. At keep time it sketches the
//! seat's next rounds ([`plan_keeps`]) and keeps a drafted card only if the
//! card lands in that plan and the plan is worth more with it than without
//! it (the keep's insight would otherwise pay for plays this round). The
//! dominance checks ([`Forced`]) are fixed policies, mostly the engine with
//! one behavior forced: they must lose to thoughtful play
//! (`docs/principles.md`, *Dominance checks*).
//!
//! The plan is simple and deterministic, and reads only the public state
//! and the seat's own hand: projected credit, insight,
//! chips, energy and antimatter income; the cards it would play (hand plus the
//! candidates) in value-per-cost order, filled into each round's budget.

use std::sync::Arc;

use aic_core::content::{CardDef, CardKind, Op, Resource, Tag, TAGS};
use aic_core::modules::{adversary, ai, board, cards, common, economy, effects, threats};
use aic_core::module::World;
use aic_core::state::AskKind;
use aic_core::{fnv1a64, Action, Agent, GameState, Key, Rules};

use super::{credit_cost, credit_equivalent};
use super::strategy::{self, Choice, Mults};
use super::value::{self, CardValue, Ctx};
use super::Strategic;

/// A play's value falls by this share for each round it waits.
const WAIT: f64 = 0.15;

/// One value of a forced behavior, read from its agent parameter.
trait ForcedValue {
    fn from_param(x: f64) -> Self;
    /// Whether this value forces anything.
    fn is_on(&self) -> bool;
}

impl ForcedValue for bool {
    fn from_param(x: f64) -> bool {
        x > 0.0
    }
    fn is_on(&self) -> bool {
        *self
    }
}

impl ForcedValue for usize {
    fn from_param(x: f64) -> usize {
        x as usize
    }
    fn is_on(&self) -> bool {
        *self > 0
    }
}

impl ForcedValue for i64 {
    fn from_param(x: f64) -> i64 {
        x as i64
    }
    fn is_on(&self) -> bool {
        *self > 0
    }
}

impl ForcedValue for f64 {
    fn from_param(x: f64) -> f64 {
        x
    }
    fn is_on(&self) -> bool {
        *self > 0.0
    }
}

impl ForcedValue for Option<Tag> {
    fn from_param(x: f64) -> Option<Tag> {
        super::TAGS.get((x as usize).wrapping_sub(1)).copied()
    }
    fn is_on(&self) -> bool {
        self.is_some()
    }
}

/// The one list of keep-plan numbers and forced behaviors: a new check is a row
/// here and its arm in [`Policy`]. A row is `(name: type, active, default, what it
/// does)`, `active` saying whether a value on its own makes the agent a [`Policy`]
/// (a plan's numbers do nothing alone). It makes the [`Forced`] field, the agent
/// parameter of the same name and the reading of one from the other. The order is
/// the parameters', which [`super::AgentSpec::label`] prints in: don't reorder rows,
/// or spec labels move. Hands the rows to `$then!`, after `$args`.
macro_rules! forced_table {
    ($then:ident ($($args:tt)*)) => {
        $then! {
            ($($args)*)
            /// Keep by plan (off: the wrapped agent's heuristic decides keeps).
            (plan_keep: bool, true, 1.0, "keep by a sketch of the next rounds (0: the heuristic keeps)")
            /// Rounds the keep plan looks at: this round's action phase and the
            /// ones after it.
            (plan_rounds: usize, false, 4.0, "rounds the keep plan sketches, this one included")
            /// The keep plan keeps nothing while the hand holds this many cards
            /// (024; 0: off). A probe guard (082): hands of 8-10 held cards never
            /// played, and at a table a player stops paying for cards they can't use.
            (plan_hand: usize, false, 0.0, "probe guard: keep nothing while the hand holds this many cards (0: off)")
            /// What a credit paid for a keep costs the plan (024).
            (plan_credit: f64, false, 3.0, "what a credit paid for a keep costs the keep plan")
            /// What the keep plan adds, in credits, for each marker a card would clear
            /// threat it would delay and nodes it would shield on the table as it
            /// stands (046: priced at its printed cost, a 2-credit Science card lost
            /// every keep to a tier-2).
            (plan_niche: f64, false, 3.0, "credits the keep plan adds per marker a card would clear now, threat it would delay and nodes it would shield")
            /// Keep every drafted card it can afford.
            (keep_all: bool, true, 0.0, "dominance check: keep every drafted card")
            /// Keep only its `keep_k` cheapest drafted cards (0: off).
            (keep_k: usize, true, 0.0, "dominance check: keep only the k cheapest drafted cards (0: off)")
            /// Sanity check (028): keep no drafted card.
            (keep_none: bool, true, 0.0, "sanity check (028): keep no drafted card")
            /// Sanity check (028): keep each drafted card on a coin toss.
            (keep_random: bool, true, 0.0, "sanity check (028): keep each drafted card on a coin toss")
            /// Never draft from tier 2.
            (tier1_only: bool, true, 0.0, "dominance check: never draft from tier 2")
            /// Draft from tier 2 whenever it can afford the keep.
            (tier2_rush: bool, true, 0.0, "dominance check: draft from tier 2 whenever the keep is affordable")
            /// Never use an AI-board power, the node powers aside (066: they
            /// are how anyone places, which `no_node` checks).
            (no_powers: bool, true, 0.0, "dominance check: never use an AI-board power but the node powers")
            /// Never pay toward a threat's counter.
            (no_counters: bool, true, 0.0, "dominance check: never counter a threat")
            /// 135: counter only with credits, energy, insight or chips: no tag
            /// sacrifice, no ambassadors, no research cubes, no nodes spent (the
            /// off arm of the new counter kinds' read).
            (old_counters: bool, true, 0.0, "135 read: counter only with credits, energy, insight or chips (no tags, ambassadors, research cubes or nodes)")
            /// Place no nodes beyond setup (no power or card that places one).
            (no_node: bool, true, 0.0, "dominance check: place no nodes beyond setup")
            /// Mono-tag (023): keep and play only cards carrying this tag.
            (only_tag: Option<Tag>, true, 0.0, "dominance check (023): keep and play only this tag (1-6: science, industry, cyber, power, diplomacy, security)")
            /// Ignore-X (023): never keep or play a card carrying this tag.
            (never_tag: Option<Tag>, true, 0.0, "dominance check (023): never keep or play this tag (1-6 as only_tag)")
            /// Event-hoarder (023): keep and play only events.
            (events_only: bool, true, 0.0, "dominance check (023): keep and play only events")
            /// Exploit check (027): no counter paid and no card that counters a threat.
            (ignore_threats: bool, true, 0.0, "exploit check (027): no counters, no card that counters a threat")
            /// Exploit check (027): stay on the setup hex and defend it: no nodes,
            /// only the remove, clear, ambassador, hardening and chips standard actions.
            (turtle: bool, true, 0.0, "exploit check (027): no nodes beyond setup; only remove, clear, chips, ambassador and hardening actions")
            /// Exploit check (027): only free cities: Diplomacy cards, the ambassador,
            /// move, remove, clear and chips standard actions, no counters, no nodes,
            /// no powers.
            (free_only: bool, true, 0.0, "exploit check (027): only Diplomacy cards and the ambassador, move, remove, clear and chips actions")
            /// Exploit check (028): farm the Unity deck. Every seat buys ambassadors
            /// and lands them on bases and factories, each worth Unity when it produces,
            /// before anything else; the rest of its play is the agent's.
            (unity_farm: bool, true, 0.0, "exploit check (028): before anything else, buy ambassadors and land them on bases and factories")
            /// Exploit check (053): give up the last country: never counter a threat
            /// on it and never pick a hex in it while another pick is open, losses
            /// aside (those may land there), so the team concentrates on the rest. Pressure counts countries at setup, so
            /// this should lose ground.
            (abandon: bool, true, 0.0, "exploit check (053): give up the last country: no counter on it, no hex in it while another pick is open, losses aside")
            /// Exploit check (034): draft, keep and play Science first, for its discounts.
            (science_rush: bool, true, 0.0, "exploit check (034): draft, keep and play Science first, for its discounts")
            /// Fixed policy (041): never clear a corruption marker.
            (never_clear: bool, true, 0.0, "fixed policy (041): never clear a corruption marker")
            /// Fixed policy (041): the Science rush, board cards first.
            (science_board: bool, true, 0.0, "fixed policy (041): the Science rush, board cards first")
            /// Fixed policy (066): both node powers whenever affordable.
            (always_max_node: bool, true, 0.0, "fixed policy (066): use both node powers whenever affordable")
            /// Fixed policy (066): never the board's second node power.
            (never_second_node: bool, true, 0.0, "fixed policy (066): never use the board's second node power")
            /// Fixed policy (068): no power that sacrifices.
            (never_sacrifice: bool, true, 0.0, "fixed policy (068): never use a power that sacrifices")
            /// Fixed policy (080): keep every liberate card, play one the first round it is legal.
            (always_liberate: bool, true, 0.0, "fixed policy (080): draft and keep every liberate card, and play one the first round it is legal")
            /// Fixed policy (068): buy every ambassador it can.
            (ambassador_mass: bool, true, 0.0, "fixed policy (068): before anything else, buy every ambassador it can afford")
            /// Fixed policy (068): the Builder's expensive power at every chance.
            (free_build_rush: bool, true, 0.0, "fixed policy (068): the Builder's free-building power at every chance")
            /// Fixed policy (068): a seat with Wall places nodes only to block troops.
            (wall_turtle: bool, true, 0.0, "fixed policy (068): a seat with Wall places nodes only with a troop on the board, on the hex nearest one")
            /// Fixed policy (070): pays only threats in its own home country.
            (home_only: bool, true, 0.0, "fixed policy (070): counter only threats in your home country")
            /// Fixed policy (070): tier-2 capstones first.
            (capstone_rush: bool, true, 0.0, "fixed policy (070): tier 2 whenever the keep is affordable, and tier-2 capstones before anything else")
            /// Fixed policy (075): a power that buys a cube at every chance.
            (always_buy_cubes: bool, true, 0.0, "fixed policy (075): the power that buys a research cube at every chance")
            /// Fixed policy (075): a seat with Patience plays and keeps only in
            /// every n-th round, the rest of the time holding its insight for a burst.
            (hoard_insight: usize, true, 0.0, "fixed policy (075): a seat with Patience plays and keeps cards only every n-th round (0: off)")
            /// Fixed policy (075): no converter, and no conversion on a power menu.
            (never_convert: bool, true, 0.0, "fixed policy (075): no converter and no power that only converts a resource")
            /// 082: the keep plan values cards by the value table (0: printed cost
            /// plus 1, the niche jobs and capstone jobs, as before 082).
            (plan_table: bool, false, 1.0, "keep plan values cards by the value table (0: by printed cost, as before 082)")
            /// 082: the keep plan plans a card whose `needs` are short now only if it
            /// is short this many tags at most (a player doesn't keep a card four tags
            /// away on the hope of drawing them); 99: no limit.
            (plan_short: usize, false, 1.0, "082: keep plan: plan a card only if its needs are at most this many tags short now (99: no limit)")
            /// 082: the keep plan counts a building that must go over one of the
            /// seat's at what it adds over the one it covers (its tags stay).
            (plan_cover: bool, false, 1.0, "082: keep plan: a building that must go over one of yours adds only what it beats the one it covers by")
            /// 092: the keep plan charges a building played with every slot full
            /// the buried building's remaining value (its standing effects and
            /// Action over the rounds left); the cheapest top is the one buried.
            (plan_bury: bool, false, 1.0, "092: keep plan: a building played with every slot full loses what the one it buries would still earn")
            /// 092 (089's call): the keep plan's budget carries research cubes, so two
            /// cube cards are not planned on the same cubes.
            (plan_cubes: bool, false, 1.0, "092: keep plan: research cubes are a budget, spent by each card planned and earned by income")
            /// 092: the opening's strategy choice reads the board's powers too.
            (strat_powers: bool, false, 1.0, "092: the strategy choice reads the board's powers as well as the opening 10")
            /// 092: the uses of each power the strategy choice counts.
            (strat_power_uses: f64, false, 1.0, "092: uses of each board power the strategy choice counts (with strat_powers)")
            /// 082, a player's rule, kept as a probe: with a card in hand that needs
            /// antimatter the seat lacks, and a board power that trades spent cards for
            /// antimatter, use it. It never fired in 288 games at d2 (seeds 31000): the
            /// Researcher, whose Particle Run it is, rarely holds two Power cards.
            (antimatter_rule: bool, true, 0.0, "082, a probe: use a power that trades spent cards for antimatter when a card in hand needs the antimatter (never fired at d2: off)")
            /// 082: each seat commits to a strategy at its round-1 keep.
            (strategy: bool, true, 0.0, "082: each seat commits to a strategy at its round-1 keep, from its board and opening 10")
            /// 082: seat 0's strategy, fixed (1-based in its board's menu; 0: chosen).
            (strat0: usize, true, 0.0, "082: seat 0's strategy, fixed: 1-based in its board's menu (configs/agents/strategies.json; 0: chosen)")
            /// 082: seat 1's strategy, fixed.
            (strat1: usize, true, 0.0, "082: seat 1's strategy, fixed (as strat0)")
            /// 082: seat 2's strategy, fixed.
            (strat2: usize, true, 0.0, "082: seat 2's strategy, fixed (as strat0)")
            /// 107: a chain card's `needs` are short counted against the tags in
            /// play and those of the cards in hand and kept ("keep the top of a
            /// chain when the cards in your hand get you there"), not the tags in
            /// play alone. The plan still plays it only once its feeders are planned.
            (plan_reach: bool, false, 1.0, "107: keep plan: a card's needs may be short now by the tags of the cards in hand and kept (plan_short counts the rest)")
            /// 107: a building that must go over one of the seat's may go over a
            /// building the plan plays before it ("build the under-card first").
            (plan_under: bool, false, 1.0, "107: keep plan: a built-over card may go over a building the plan plays first")
            /// 107: a chain card in hand or kept that the plan can't play yet, but
            /// whose `needs` the plan's cards bring within `plan_short` tags, counts
            /// its value at the plan's last round times the square of the share of
            /// its tags reached ("build toward the chain card you hold"): a feeder
            /// adds progress, and the chain card is kept for it. Off (107: -2.8
            /// ±2.2 for the strategist on 96 dev seeds; it kept chain cards it never played).
            (plan_chain: bool, false, 0.0, "107, off: keep plan: a chain card the plan's tags bring within plan_short counts its later value times its share reached, squared")
            /// 107: a card that copies a building's income is planned only with one
            /// built or planned before it to copy.
            (plan_copy: bool, false, 1.0, "107: keep plan: plan a copy-income card only with an income building built or planned before it")
            /// 107, a probe: draft and keep every engine card (a trigger, a
            /// threshold, a per-tag income or discount, a copy) and play one the
            /// moment it is legal: what the engines win when they are played.
            (engine_rush: usize, true, 0.0, "107, a probe: 1 drafts and keeps every engine card and plays one as soon as it is legal; 2 only plays a kept one at once")
            /// 124: insight beyond the heuristic's `insight_cap` buys the picks the
            /// plan can't fit, each worth its price at the table, while it pays the
            /// whole keep ("with insight to burn, keep every card worth having").
            (keep_spare: bool, false, 1.0, "124: keep plan: a pick worth its price is kept when insight beyond the heuristic's insight_cap pays its whole keep")
            /// 124 (D47): the keep plan reads a teammate trigger at the teammates' tag share.
            /// The play check's twin is `w.team_share`: screen the two together.
            (plan_team: bool, false, 0.0, "124 (D47): keep plan: a trigger on a teammate's plays fires at the teammates' share of the tag")
            /// 128: the keep plan counts an Action that places project cubes only
            /// while a megaproject is open to the seat (Particle Accelerator).
            /// The play check's twin is `w.cube_target`: screen the two together.
            (plan_cube_target: bool, false, 1.0, "128: keep plan: a project-cube Action counts no uses once no megaproject is open")
        }
    };
}
pub(crate) use forced_table;

macro_rules! forced_struct {
    (() $($(#[$doc:meta])* ($name:ident: $ty:ty, $active:literal, $default:expr, $desc:expr))*) => {
        /// Keep-plan numbers, strategy settings, and the behavior a dominance
        /// check forces (none for thoughtful play).
        #[derive(Debug, Clone)]
        pub struct Forced {
            $($(#[$doc])* pub $name: $ty,)*
        }

        impl Forced {
            /// Each field from its agent parameter.
            pub fn read(num: impl Fn(&str) -> f64) -> Forced {
                Forced { $($name: ForcedValue::from_param(num(stringify!($name))),)* }
            }

            /// Whether anything is forced: keeping by plan or a dominance check.
            pub fn is_active(&self) -> bool {
                $(($active && self.$name.is_on()))||*
            }
        }
    };
}

forced_table!(forced_struct());

impl Forced {
    /// Whether this seat places no nodes beyond setup.
    fn stays_put(&self) -> bool {
        self.no_node || self.turtle || self.free_only
    }

    /// Whether seat `s` is holding its insight this round (075): no play, and no
    /// keep but the opening's, which are free.
    fn hoarding(&self, w: &World, g: &GameState, s: usize, keep: bool) -> bool {
        self.hoard_insight > 0
            && !(keep && g.round == 1)
            && !(g.round as usize).is_multiple_of(self.hoard_insight)
            && economy::patient(w, s)
    }

    /// Whether the card restrictions (mono-tag, ignore-X, event-hoarder)
    /// let this seat keep or play `d`.
    pub fn allows(&self, d: &CardDef) -> bool {
        let has = |t: &Option<Tag>| t.as_ref().is_some_and(|t| d.tags.contains(t));
        (self.only_tag.is_none() || has(&self.only_tag))
            && !has(&self.never_tag)
            && (!self.events_only || d.is_event())
            && (!self.free_only || d.tags.contains(&Tag::Diplomacy))
            && (!self.ignore_threats || !counters_threats(d))
            && (!self.never_clear || !d.effects.iter().any(|e| matches!(e, Op::ClearMarkers { .. })))
    }
}

/// Whether `d` counters a threat or makes one cheaper, in any of its lists: an
/// automatic-counter capstone too (075, 073's finding 13).
fn counters_threats(d: &CardDef) -> bool {
    let ability = d.ability.iter().flat_map(|a| a.branches()).flat_map(|b| &b.effects);
    d.effects.iter().chain(ability).chain(&d.passives).any(|o| {
        matches!(o, Op::CounterThreat | Op::Jam { .. } | Op::DelayThreat { .. } | Op::AutoCounter { .. })
    })
}

pub struct Policy {
    inner: Box<dyn Strategic + Send>,
    forced: Forced,
    /// This round's keep set for a seat: (round, seat, cards).
    keeps: Vec<(u32, usize, Vec<Key>)>,
    /// 082: each seat's strategy, and whether it has been chosen.
    strat: Choice,
    chosen: bool,
    mults: Option<Arc<Mults>>,
    /// 102 (`plan_rates`): the value table with energy and antimatter at the
    /// heuristic's rates, made at the first keep.
    rates: Option<Arc<value::Table>>,
}

impl Policy {
    pub fn new(inner: Box<dyn Strategic + Send>, forced: Forced) -> Policy {
        Policy {
            inner,
            forced,
            keeps: Vec::new(),
            strat: [None; 4],
            chosen: false,
            mults: None,
            rates: None,
        }
    }

    /// 082: a rollout's policy, handed the strategies its planner chose (none yet:
    /// it chooses them itself at the round-1 keep, as the planner's would).
    pub fn with_strategies(mut self, c: Choice) -> Policy {
        if c.iter().any(Option::is_some) {
            self.set_strategies(c);
        }
        self
    }

    /// Whether this policy plays strategies (082).
    fn strategic(&self) -> bool {
        self.forced.strategy || self.forced.strat0 > 0 || self.forced.strat1 > 0 || self.forced.strat2 > 0
    }

    /// 082: each seat's strategy, chosen at the round-1 keep from its board and
    /// opening deal (a pure function of the table), and handed to the agent inside.
    fn choose_strategies(&mut self, w: &World, g: &GameState) {
        let contents = self.inner.eval().is_some_and(|e| e.w.strat_contents > 0.0);
        let t = self.inner.eval().map_or_else(value::table, |e| e.tbl);
        let c = opening_choice(t, w, g, &self.forced, contents);
        self.set_strategies(c);
    }

    /// The cards seat `s` keeps this round, worked out at its first keep
    /// decision of the round.
    fn keep_set(&mut self, rules: &Rules, g: &GameState, s: usize) -> Vec<Key> {
        let round = g.round;
        if let Some((_, _, set)) = self.keeps.iter().find(|(r, x, _)| *r == round && *x == s) {
            return set.clone();
        }
        let world = rules.world();
        let drafted = g.pile(&cards::drafted(s)).to_vec();
        let full = self.forced.plan_hand > 0 && g.pile(&cards::hand(s)).len() >= self.forced.plan_hand;
        let strat = self.strat.get(s).copied().flatten();
        let ev = self.inner.eval();
        if strat.is_some() && self.mults.is_none() {
            // The heuristic's own multipliers (116 `strat_scale`, 124 `strat_contents`).
            self.mults = Some(ev.map_or_else(|| Arc::new(Mults::new(&world, 1.0, false)), |e| e.mults()));
        }
        let super::eval::PlanRates { energy, antimatter, project, cap, idle, insight_cap, energy_cap, energy_share } =
            ev.map_or_else(super::eval::PlanRates::table, |e| e.plan_rates());
        // 128 (`energy_cap`): a seat whose energy piles up keeps what spends it.
        let held = g.resource(s, Resource::Energy) as f64;
        let energy_rate = if energy_cap > 0.0 && held > energy_cap { energy_share } else { 1.0 };
        // 102 (D23): energy and antimatter at the heuristic's own rates, as the play check prices them.
        let own = ev.map_or_else(value::table, |e| e.tbl);
        if self.rates.is_none() {
            self.rates = Some(Arc::new(own.repriced(energy, antimatter)));
        }
        let t = self.rates.as_deref().unwrap_or(own);
        let idle = (cap, idle);
        let spare_at = if self.forced.keep_spare { insight_cap } else { 0.0 };
        let pc = PlanCtx { table: self.forced.plan_table, strat, mults: self.mults.as_deref(), t, project, idle, spare_at, energy_rate };
        let set = if self.forced.keep_none {
            Vec::new()
        } else if self.forced.keep_random {
            let coin = |id: &Key| fnv1a64(format!("{}:{round}:{id}", g.seed).as_bytes()) & 1 == 0;
            drafted.into_iter().filter(coin).collect()
        } else if self.forced.keep_all {
            drafted
        } else if self.forced.science_rush
            || self.forced.science_board
            || self.forced.capstone_rush
            || self.forced.always_liberate
            || self.forced.engine_rush == 1
        {
            let mut set = plan_keeps(&world, g, s, &self.forced, &pc);
            for id in &drafted {
                let d = cards::def(&world, id);
                let rush = if self.forced.engine_rush == 1 {
                    is_engine(d)
                } else if self.forced.always_liberate {
                    is_liberate(d)
                } else if self.forced.capstone_rush {
                    is_capstone(d)
                } else {
                    d.tags.contains(&Tag::Science)
                };
                if rush && !set.contains(id) {
                    set.push(*id);
                }
            }
            set
        } else if self.forced.keep_k > 0 {
            let mut d = drafted;
            d.sort_by(|a, b| {
                credit_cost(cards::def(&world, a))
                    .cmp(&credit_cost(cards::def(&world, b)))
                    .then_with(|| a.cmp(b))
            });
            d.truncate(self.forced.keep_k);
            d
        } else if full {
            Vec::new()
        } else {
            plan_keeps(&world, g, s, &self.forced, &pc)
        };
        let hoards = self.forced.hoarding(&world, g, s, true);
        let set: Vec<Key> = set
            .into_iter()
            .filter(|id| !hoards && self.forced.allows(cards::def(&world, id)))
            .collect();
        self.keeps.retain(|(r, _, _)| *r == round);
        self.keeps.push((round, s, set.clone()));
        set
    }

    /// Whether `a` is an action the forced behavior rules out.
    fn ruled_out(&self, world: &World, g: &GameState, a: &Action) -> bool {
        let f = &self.forced;
        match a {
            Action::Pay { threat, .. } | Action::CounterWith { threat, .. } if f.abandon && on_given_up(world, g, threat) => {
                true
            }
            Action::Pay { seat, threat, .. } | Action::CounterWith { seat, threat, .. }
                if f.home_only && abroad(world, g, *seat as usize, threat) =>
            {
                true
            }
            Action::Pay { resource: Resource::Ambassadors | Resource::ResearchCubes, .. } if f.old_counters => true,
            Action::CounterWith { .. } => f.no_counters || f.ignore_threats || f.free_only || f.old_counters,
            Action::Choose { seat, option, .. } if f.home_only && abroad(world, g, *seat as usize, option) => true,
            Action::Choose { option, target, .. } if f.abandon => {
                on_given_up(world, g, option)
                    || (!losing_pick(g) && target.and_then(|t| world.map.find(t)).is_some_and(|h| given_up(world, h)))
            }
            Action::Tier { tier: 2, .. } => f.tier1_only,
            Action::Convert { .. } => f.never_convert,
            Action::Power { seat, power } => {
                let (s, k) = (*seat as usize, *power as usize);
                let sacrifices = !ai::board(world, s).powers[k].tags.is_empty();
                (f.never_sacrifice && sacrifices)
                    || (f.never_convert && ai::power_converts(world, s, k))
                    || if ai::power_places(world, s, k) {
                        f.stays_put()
                            || (f.never_second_node && second_node(world, s, k))
                            || (f.wall_turtle && has_wall(world, g, s) && !troops_up(g))
                    } else {
                        f.no_powers || f.free_only
                    }
            }
            Action::Pay { .. } => f.no_counters || f.ignore_threats || f.free_only,
            Action::Standard { act, .. } => {
                (f.never_clear && act == "clear")
                    || (f.turtle && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "harden_chips" | "harden_buy" | "chips"))
                    || (f.free_only && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "move" | "chips"))
            }
            Action::Play { card, seat } => {
                let d = cards::def(world, card);
                let walls = f.wall_turtle && has_wall(world, g, *seat as usize) && !troops_up(g);
                ((f.stays_put() || walls) && places_node(&d.effects)) || !f.allows(d) || f.hoarding(world, g, *seat as usize, false)
            }
            Action::Keep { card, seat } => !f.allows(cards::def(world, card)) || f.hoarding(world, g, *seat as usize, true),
            _ => false,
        }
    }
}

/// Whether seat `s` has the Wall rule.
fn has_wall(w: &World, g: &GameState, s: usize) -> bool {
    cards::has_passive(w, g, s, |p| matches!(p, Op::Wall))
}

/// Whether an adversary troop stands on the board.
fn troops_up(g: &GameState) -> bool {
    (0..g.hexes.len()).any(|h| g.tokens(h, common::TROOP) > 0)
}

/// The wall turtle's hex (068): of a node prompt's hexes, the one nearest a
/// troop but not on it, where a wall blocks nothing.
fn wall_hex(w: &World, g: &GameState, legal: &[Action]) -> Option<usize> {
    let p = g.pending.as_ref()?;
    // 118: the prompt reads "place a node (...)" (it said "place nodes"; the rename
    // turned this off unseen: one const now writes and reads it).
    if !p.ask.prompt.contains(board::NODE_PROMPT) {
        return None;
    }
    let troops: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, common::TROOP) > 0).collect();
    let gap = |t: &aic_core::hex::Hex| {
        let h = w.map.find(*t)?;
        troops.iter().map(|&x| w.map.distance(h, x)).filter(|&d| d > 0).min()
    };
    (0..legal.len())
        .filter_map(|i| match &legal[i] {
            Action::Choose { seat, target: Some(t), .. } if has_wall(w, g, *seat as usize) => gap(t).map(|d| (d, i)),
            _ => None,
        })
        .min()
        .map(|(_, i)| i)
}

fn places_node(effects: &[Op]) -> bool {
    effects.iter().any(|e| matches!(e, Op::PlaceNode { .. }))
}

/// Whether power `k` is seat `s`'s second node power: one that places,
/// after the board's first that does.
fn second_node(w: &World, s: usize, k: usize) -> bool {
    (0..k).any(|j| ai::power_places(w, s, j))
}

/// The first draft, keep or play of a card `pick` wants.
fn first_card(legal: &[Action], pick: impl Fn(&Key) -> bool) -> Option<usize> {
    legal.iter().position(|a| {
        matches!(a, Action::Draft { card, .. } | Action::Keep { card, .. } | Action::Play { card, .. } if pick(card))
    })
}

/// The first power `pick` wants, by seat and power.
fn first_power(legal: &[Action], pick: impl Fn(usize, usize) -> bool) -> Option<usize> {
    legal
        .iter()
        .position(|a| matches!(a, Action::Power { seat, power } if pick(*seat as usize, *power as usize)))
}

/// Whether a card acts on the board the way 041's Science does.
fn acts_on_board(d: &CardDef) -> bool {
    d.effects.iter().any(|e| {
        matches!(
            e,
            Op::ClearMarkers { .. } | Op::ShieldNode { .. } | Op::DelayThreat { .. } | Op::MoveOne { .. }
        )
    })
}

impl Strategic for Policy {
    fn set_strategies(&mut self, c: Choice) {
        self.strat = c;
        self.chosen = true;
        self.inner.set_strategies(c);
    }

    fn eval(&self) -> Option<&super::eval::Eval> {
        self.inner.eval()
    }
}

impl Agent for Policy {
    fn choose(&mut self, rules: &Rules, g: &GameState, legal: &[Action]) -> usize {
        let world = rules.world();
        let phase = g.config.round.get(g.phase).map(String::as_str).unwrap_or("");
        let pending = legal.iter().any(|a| matches!(a, Action::Choose { .. }));
        if !self.chosen && self.strategic() && phase == "cards.keep" && g.round == 1 {
            self.choose_strategies(&world, g);
        }
        // Keeping: by plan, or by the forced rule.
        let keeps = phase == "cards.keep"
            && !pending
            && (self.forced.plan_keep
                || self.forced.keep_all
                || self.forced.keep_k > 0
                || self.forced.keep_none
                || self.forced.keep_random);
        if keeps {
            if let Some(s) = legal.first().map(|a| a.seat() as usize) {
                let set = crate::prof::time(crate::prof::Part::Policy, || self.keep_set(rules, g, s));
                // 041: with insight short, the board cards go first.
                let board = |a: &Action| matches!(a, Action::Keep { seat, card } if *seat as usize == s
                    && set.contains(card) && acts_on_board(cards::def(&world, card)));
                if self.forced.science_board {
                    if let Some(i) = legal.iter().position(board) {
                        return i;
                    }
                }
                if let Some(i) = legal.iter().position(
                    |a| matches!(a, Action::Keep { seat, card } if *seat as usize == s && set.contains(card)),
                ) {
                    return i;
                }
                if let Some(i) = legal
                    .iter()
                    .position(|a| matches!(a, Action::Pass { seat } if *seat as usize == s))
                {
                    return i;
                }
            }
        }
        if self.forced.unity_farm {
            if let Some(i) = farm(&world, g, legal) {
                return i;
            }
        }
        if self.forced.antimatter_rule && !pending {
            if let Some(i) = first_power(legal, |s, k| antimatter_wanted(&world, g, s, k)) {
                if !self.ruled_out(&world, g, &legal[i]) {
                    return i;
                }
            }
        }
        if self.forced.ambassador_mass && !pending {
            if let Some(i) = legal.iter().position(|a| matches!(a, Action::Standard { act, .. } if act == "ambassador")) {
                return i;
            }
        }
        if self.forced.free_build_rush && !pending {
            if let Some(i) = first_power(legal, |s, k| ai::power_builds(&world, s, k)) {
                return i;
            }
        }
        if self.forced.always_buy_cubes && !pending {
            if let Some(i) = first_power(legal, |s, k| ai::power_buys_cube(&world, s, k)) {
                return i;
            }
        }
        if self.forced.wall_turtle {
            if let Some(i) = wall_hex(&world, g, legal) {
                return i;
            }
        }
        if self.forced.always_max_node && !pending {
            if let Some(i) = first_power(legal, |s, k| ai::power_places(&world, s, k)) {
                return i;
            }
        }
        if self.forced.science_rush && !pending {
            if let Some(i) = first_card(legal, |id| cards::def(&world, id).tags.contains(&Tag::Science)) {
                return i;
            }
        }
        if self.forced.science_board && !pending {
            let science = |id: &Key, board: bool| {
                let d = cards::def(&world, id);
                d.tags.contains(&Tag::Science) && (!board || acts_on_board(d))
            };
            for board in [true, false] {
                if let Some(i) = first_card(legal, |id| science(id, board)) {
                    return i;
                }
            }
        }
        if self.forced.always_liberate && !pending {
            if let Some(i) = first_card(legal, |id| is_liberate(cards::def(&world, id))) {
                return i;
            }
        }
        if self.forced.engine_rush > 0 && !pending {
            let engine = |id: &Key| is_engine(cards::def(&world, id));
            let at = if self.forced.engine_rush == 1 {
                first_card(legal, engine)
            } else {
                legal.iter().position(|a| matches!(a, Action::Play { card, .. } if engine(card)))
            };
            if let Some(i) = at {
                return i;
            }
        }
        if self.forced.capstone_rush && !pending {
            if let Some(i) = first_card(legal, |id| is_capstone(cards::def(&world, id))) {
                return i;
            }
        }
        // Tier-2 rush: tier 2 whenever the keep is affordable.
        if (self.forced.tier2_rush || self.forced.capstone_rush) && !pending {
            if let Some(i) = legal.iter().position(|a| {
                matches!(a, Action::Tier { seat, tier: 2 } if keep_affordable(&world, g, *seat as usize, 2))
            }) {
                return i;
            }
        }
        let keep: Vec<usize> = (0..legal.len())
            .filter(|&i| !self.ruled_out(&world, g, &legal[i]))
            .collect();
        if keep.is_empty() || keep.len() == legal.len() {
            return self.inner.choose(rules, g, legal);
        }
        let sub: Vec<Action> = keep.iter().map(|&i| legal[i].clone()).collect();
        keep[self.inner.choose(rules, g, &sub)]
    }
}

/// 082: each seat's strategy on the table at its round-1 keep, from its board and
/// opening deal, with `f`'s fixed ones (`strat0`..`strat2`) standing.
pub fn opening_choice(t: &value::Table, w: &World, g: &GameState, f: &Forced, contents: bool) -> Choice {
    let worth = |d: &CardDef| {
        let typical = Ctx::typical_for(t, d.tier);
        CardValue::of(t, d).total(t, &typical) - t.cost(d, d.cost, d.insight, typical.horizon)
    };
    // 092: one use of a power, less its resources. Over the opening's horizon
    // (4 uses) it outweighed the opening 10 and set every seat's choice alone
    // (every Builder Power grid, every Researcher Patience burst, seeds 31100).
    let power = |p: &aic_core::content::Power| {
        let typical = Ctx::typical_for(t, 1);
        (value::power_worth(t, p).instant(t, &typical), f.strat_power_uses)
    };
    let power: strategy::PowerWorth = if f.strat_powers { Some(&power) } else { None };
    let fixed = [f.strat0, f.strat1, f.strat2, 0];
    let mut c: Choice = [None; 4];
    for (s, slot) in c.iter_mut().enumerate().take(g.seats.len().min(4)) {
        *slot = strategy::choose(w, g, s, fixed[s], &worth, power, contents);
    }
    c
}

/// `aic-sim strategies` (082): each seed's strategies as the agent's seats choose
/// them at the round-1 keep, as CSV lines `seed,seat,board,strategy`. The agent
/// plays up to that keep, so the table is the one its game reaches.
pub fn print_choices(rules: &Rules, spec: &super::AgentSpec, first: u64, games: u64) {
    let w = rules.world();
    println!("seed,seat,board,strategy");
    for seed in first..first.saturating_add(games) {
        let mut g = rules.new_game(seed);
        let mut agent = spec.build(rules, seed);
        let mut legal = Vec::new();
        while !g.is_over() {
            let phase = g.config.round.get(g.phase).map(String::as_str).unwrap_or("");
            if phase == "cards.keep" && g.round == 1 {
                break;
            }
            rules.legal_actions_into(&g, &mut legal);
            if legal.is_empty() {
                break;
            }
            let i = if legal.len() == 1 { 0 } else { agent.choose(rules, &g, &legal) };
            if rules.apply_mut(&mut g, &legal[i], None).is_err() {
                break;
            }
        }
        let c = opening_choice(spec.table(), &w, &g, &spec.forced(), spec.weights().strat_contents > 0.0);
        for (s, k) in c.iter().enumerate().take(g.seats.len()) {
            let name = k.map_or("none", |k| strategy::strategies().strategies[k as usize].id.as_str());
            println!("{seed},{s},{},{name}", ai::board(&w, s).id);
        }
    }
}

/// 082's antimatter rule: whether power `k` of seat `s` sacrifices cards for
/// antimatter while a card in the seat's hand needs antimatter it lacks and is
/// otherwise within reach (its credits held).
fn antimatter_wanted(w: &World, g: &GameState, s: usize, k: usize) -> bool {
    let p = &ai::board(w, s).powers[k];
    let gains = p.effects.iter().any(|e| matches!(e, Op::Gain { resource: Resource::Antimatter, .. }));
    if !gains || p.tags.is_empty() {
        return false;
    }
    let have = g.resource(s, Resource::Antimatter);
    g.pile(&cards::hand(s)).iter().any(|id| {
        let d = cards::def(w, id);
        d.antimatter > have && d.antimatter <= have + 1 && g.resource(s, Resource::Credits) >= cards::play_cost(w, g, s, d)
    })
}

/// The country the abandon check gives up.
fn last_country(w: &World) -> usize {
    w.map.countries.len() - 1
}

/// Whether hex `h` is in the country the abandon check gives up.
fn given_up(w: &World, h: usize) -> bool {
    w.map.hexes[h].country as usize == last_country(w)
}

/// Whether the open choice picks what the team loses: there the check may
/// spend the country it gave up.
fn losing_pick(g: &GameState) -> bool {
    g.pending.as_ref().is_some_and(|p| p.ask.kind == AskKind::Loss)
}

/// Whether `threat` is an active threat outside seat `s`'s home country.
fn abroad(w: &World, g: &GameState, s: usize, threat: &str) -> bool {
    threats::active(g).iter().any(|(t, c)| t == threat && c.is_some_and(|c| c != common::home_country(w, s)))
}

/// Whether `threat` sits on the country the abandon check gives up.
fn on_given_up(w: &World, g: &GameState, threat: &str) -> bool {
    threats::active(g).iter().any(|(t, c)| t == threat && *c == Some(last_country(w)))
}

/// Whether hex `h` pays Unity for a player ambassador when it produces and
/// holds none yet (028).
fn farm_site(w: &World, g: &GameState, h: usize) -> bool {
    (common::is_base(w, h) || g.tokens(h, common::FACTORY) > 0)
        && common::player_ambassador(g, h).is_none()
        && g.tokens(h, common::ADV_AMBASSADOR) == 0
}

/// The Unity farmer's move (028), if it has one: send an ambassador to a free
/// farm site in reach, or buy one to send.
fn farm(w: &World, g: &GameState, legal: &[Action]) -> Option<usize> {
    if let Some(p) = &g.pending {
        if p.ask.prompt.contains("move which ambassador") {
            return legal.iter().position(|a| matches!(a, Action::Choose { option, .. } if option == "from your supply"));
        }
        return p.ask.prompt.contains(": move on which hex").then(|| {
            legal.iter().position(|a| {
                matches!(a, Action::Choose { target: Some(t), .. }
                    if w.map.find(*t).is_some_and(|h| farm_site(w, g, h)))
            })
        })?;
    }
    let sites = |s: usize| {
        (0..g.hexes.len())
            .filter(|&h| farm_site(w, g, h) && common::in_reach(w, g, s, h))
            .count() as i64
    };
    let act = |want: &str| {
        legal.iter().position(|a| {
            matches!(a, Action::Standard { seat, act } if act == want && {
                let s = *seat as usize;
                let held = g.resource(s, Resource::Ambassadors);
                sites(s) > if want == "move" { 0 } else { held } && (want != "move" || held > 0)
            })
        })
    };
    act("move").or_else(|| act("ambassador"))
}

/// Whether seat `s` could pay to keep a card of `tier` now (insight, with
/// any shortfall in credits).
fn keep_affordable(w: &World, g: &GameState, s: usize, tier: u8) -> bool {
    let (ins, cr) = cards::keep_need(w, g, s, tier, 0);
    let short = (ins - g.resource(s, Resource::Insight).max(0)).max(0);
    g.resource(s, Resource::Credits) >= cr + short * w.int("cards.insight_credits")
}

/// One round's budget in the plan.
#[derive(Clone, Copy)]
struct Budget {
    credits: i64,
    insight: i64,
    chips: i64,
    energy: i64,
    antimatter: i64,
    /// 092 (`plan_cubes`): research cubes.
    research_cubes: i64,
}

/// What the keep plan values cards by (082): the value table or the printed
/// cost, and the seat's strategy.
#[derive(Clone, Copy)]
pub struct PlanCtx<'a> {
    pub table: bool,
    pub strat: Option<u16>,
    pub mults: Option<&'a Mults>,
    /// The value table the plan prices by (102, D23: repriced at the heuristic's rates).
    pub t: &'a value::Table,
    /// 102 (D34): what the heuristic charges for an adversary project
    /// wholly filled, in table credits (0: no situational project value).
    pub project: f64,
    /// 102: the credits the heuristic counts in full and an idle
    /// credit's share of one beyond them; a net price pays from the idle ones
    /// first (`(f64::MAX, 1.0)`: every credit in full).
    pub idle: (f64, f64),
    /// 124 (`keep_spare`): insight beyond this is spare (0: off).
    pub spare_at: f64,
    /// 128: the seat's `Ctx::energy_rate` at the keep.
    pub energy_rate: f64,
}

/// What a plan's values read once (082): rounds left by the adversary deck, the
/// seat's nodes, whether a city is corrupted; 102: the visible projects' shares
/// filled, fullest first.
struct PlanTable {
    left: f64,
    node: f64,
    corrupted: bool,
    projects: Vec<(f64, i64)>,
    /// 107 (`rates.own_mix`): the share of the seat's cards carrying each tag.
    share: [f64; TAGS.len()],
    /// 124 (`plan_team`): the teammates' share (`None`: the seat's own).
    team: Option<[f64; TAGS.len()]>,
    /// 128: `Ctx::cubes_open` for the seat.
    cubes_open: bool,
}

impl PlanTable {
    fn read(t: &value::Table, w: &World, g: &GameState, s: usize, f: &Forced) -> PlanTable {
        PlanTable {
            team: (f.plan_team && g.seats.len() > 1).then(|| {
                let others = (0..g.seats.len())
                    .filter(|&o| o != s)
                    .map(|o| super::eval::own_mix_counts(w, g, o, &cards::tag_counts(w, g, o)));
                super::eval::team_share(t, others)
            }),
            left: super::eval::Eval::cards_left(g) as f64,
            node: common::node_count(g, s) as f64,
            corrupted: (0..g.hexes.len()).any(|h| common::corrupted_city(w, g, h)),
            cubes_open: !f.plan_cube_target || !aic_core::modules::effects::cube_projects(w, g, s, false).is_empty(),
            share: {
                let tags = cards::tag_counts(w, g, s);
                super::eval::own_mix(t, w, g, s, &tags, Ctx::at(t, 0.0, 0.0, tags, false)).share
            },
            projects: {
                // (share filled, compute on it), fullest first.
                let mut p: Vec<(f64, i64)> = adversary::projects(g)
                    .iter()
                    .map(|id| {
                        let on = g.var(&adversary::project_key(id));
                        (on as f64 / adversary::project_cost(w, g, id).max(1) as f64, on)
                    })
                    .collect();
                p.sort_by(|a, b| b.0.total_cmp(&a.0));
                p
            },
        }
    }
}

/// 102 (D34): what a card's project ops add over their table rows on this table:
/// a discard takes the fullest project, a drain the compute it can reach, each
/// at what the heuristic charges for it (`PlanCtx::project`). A one-use worth,
/// once (the project is the table as it stands).
fn situational(d: &CardDef, pc: &PlanCtx, pt: &PlanTable) -> f64 {
    if pc.project <= 0.0 {
        return 0.0;
    }
    let t = pc.t;
    let action = d.ability.iter().flat_map(|a| a.branches()).flat_map(|b| &b.effects);
    d.effects
        .iter()
        .chain(action)
        .map(|e| match e {
            Op::DiscardProject => match pt.projects.first() {
                Some((share, _)) => (pc.project * share - t.op("discard_project")).max(0.0),
                None => 0.0,
            },
            Op::DrainProject { amount } => match pt.projects.first() {
                Some((share, on)) if *on > 0 => {
                    let cost = *on as f64 / share.max(1e-9);
                    let drained = (*amount).min(*on) as f64;
                    (pc.project * drained / cost - t.op("drain_project") * *amount as f64).max(0.0)
                }
                _ => 0.0,
            },
            _ => 0.0,
        })
        .fold(0.0, f64::max)
}

/// The drafted cards seat `s` keeps: each, best value per cost first, if
/// the plan with it is worth more than the plan without it.
pub fn plan_keeps(w: &World, g: &GameState, s: usize, f: &Forced, pc: &PlanCtx) -> Vec<Key> {
    let pt = PlanTable::read(pc.t, w, g, s, f);
    let mut drafted = g.pile(&cards::drafted(s)).to_vec();
    let tags = cards::tag_counts(w, g, s);
    by_ratio(w, g, s, f, pc, &pt, &tags, &mut drafted);
    let mut kept: Vec<Key> = Vec::new();
    let mut best = plan_value(w, g, s, &kept, f, pc, &pt);
    for c in drafted {
        let mut with = kept.clone();
        with.push(c);
        let Some(v) = plan_value_checked(w, g, s, &with, &c, f, pc, &pt) else {
            continue;
        };
        if v > best + 1e-9 {
            best = v;
            kept = with;
        }
    }
    // 124 (`keep_spare`): insight beyond the heuristic's cap buys the picks the
    // plan can't fit yet: each worth its price at the table, best first, while
    // the spare insight pays its whole keep ("with insight to burn, keep every
    // card worth having").
    if pc.spare_at > 0.0 {
        let mut count = g.var(&cards::kept_key(s));
        let mut spare = g.resource(s, Resource::Insight) as f64 - pc.spare_at;
        for id in &kept {
            let tier = cards::def(w, id).tier;
            spare -= cards::keep_need(w, g, s, tier, count).0 as f64;
            count += cards::counts_as_kept(g, tier) as i64;
        }
        let mut rest: Vec<Key> = g.pile(&cards::drafted(s)).iter().filter(|c| !kept.contains(c)).cloned().collect();
        by_ratio(w, g, s, f, pc, &pt, &tags, &mut rest);
        for c in rest {
            let d = cards::def(w, &c);
            let (ins, cr) = cards::keep_need(w, g, s, d.tier, count);
            if cr > 0 || ins as f64 > spare {
                continue;
            }
            let worth = if pc.table {
                let typical = Ctx::typical_for(pc.t, d.tier);
                CardValue::of(pc.t, d).total(pc.t, &typical) - pc.t.cost(d, d.cost, d.insight, typical.horizon)
            } else {
                1.0
            };
            if worth <= 0.0 {
                continue;
            }
            spare -= ins as f64;
            count += cards::counts_as_kept(g, d.tier) as i64;
            kept.push(c);
        }
    }
    kept
}

/// [`plan_value`] with `kept`, or `None` if the keeps can't be paid or the
/// new card `c` is not played in the plan.
#[allow(clippy::too_many_arguments)]
fn plan_value_checked(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[Key],
    c: &str,
    f: &Forced,
    pc: &PlanCtx,
    pt: &PlanTable,
) -> Option<f64> {
    let (v, planned) = plan(w, g, s, kept, f, pc, pt)?;
    planned.iter().any(|p| p == c).then_some(v)
}

fn plan_value(w: &World, g: &GameState, s: usize, kept: &[Key], f: &Forced, pc: &PlanCtx, pt: &PlanTable) -> f64 {
    plan(w, g, s, kept, f, pc, pt).map_or(f64::NEG_INFINITY, |(v, _)| v)
}

/// The jobs `d` would do for seat `s` on this table: markers in reach to
/// clear, threats to delay, nodes with no token to shield.
fn niche_jobs(w: &World, g: &GameState, s: usize, d: &CardDef) -> i64 {
    d.effects
        .iter()
        .map(|e| match e {
            Op::ClearMarkers { count } => {
                let open = |h: &usize| g.tokens(*h, common::MARKER) > 0 && common::in_reach(w, g, s, *h);
                (*count).min((0..g.hexes.len()).filter(open).count() as i64)
            }
            // 118: only threats the seat may counter, as the engine delays (F136).
            Op::DelayThreat { count } => {
                (*count).min(threats::counterable(w, g, s, None).iter().filter(|t| !threats::waits(g, t)).count() as i64)
            }
            Op::ShieldNode { count } => {
                (*count).min((0..g.hexes.len()).filter(|&h| effects::shieldable(g, s, h)).count() as i64)
            }
            // 070: a typed counter with a threat up it can reach (threats are dealt before keeps).
            Op::Jam { amount, source: Some(t) } => {
                if threats::counterable(w, g, s, Some(*t)).is_empty() { 0 } else { (*amount + 1) / 2 }
            }
            _ => 0,
        })
        .sum::<i64>()
}

/// A tier-2 capstone (070): a tier-2 building whose standing effect ends an
/// engine: income per tag, an automatic counter or project drain.
pub(crate) fn is_capstone(d: &CardDef) -> bool {
    d.tier == 2
        && d.kind == CardKind::Building
        && d.passives
            .iter()
            .any(|p| matches!(p, Op::IncomePerTag { .. } | Op::AutoCounter { .. } | Op::ProjectDrain { .. }))
}

/// 070: what a capstone's standing effect is worth to the keep plan, in
/// `plan_niche` jobs: its printed price undersells a card that pays every round.
/// 082: only on printed costs; the table values a standing effect by its rounds.
fn capstone_jobs(d: &CardDef) -> i64 {
    if !is_capstone(d) {
        return 0;
    }
    d.passives
        .iter()
        .map(|p| match p {
            Op::AutoCounter { amount, .. } => *amount,
            Op::ProjectDrain { amount } => 3 * *amount,
            Op::IncomePerTag { .. } => 3,
            _ => 0,
        })
        .sum()
}

/// An engine card (107's probe): one whose standing effects fire on plays or
/// other events, fill a threshold, pay by tags or cut play costs, or whose play
/// raises an income by tags or copies one.
pub(crate) fn is_engine(d: &CardDef) -> bool {
    d.passives.iter().any(|p| {
        matches!(p, Op::When { .. } | Op::Threshold { .. } | Op::IncomePerTag { .. } | Op::CardDiscount { .. })
    }) || d.effects.iter().any(|e| matches!(e, Op::ProductionPerTag { .. } | Op::CopyIncome))
}

/// 119: `aic-sim cards`: every content card with the agents' card tests
/// ([`is_engine`], [`is_liberate`]), so a script reads them here instead of
/// copying them (108 copied `is_engine` into Python twice).
pub fn cards_csv() -> String {
    let mut out = String::from("id,kind,tier,engine,liberate\n");
    for d in &aic_core::content::content().cards {
        let kind = serde_json::to_value(d.kind).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
        out.push_str(&format!("{},{kind},{},{},{}\n", d.id, d.tier, is_engine(d) as u8, is_liberate(d) as u8));
    }
    out
}

/// A liberate card (080): one that liberates a corrupted city when played or
/// by its Action (Great Liberation's cleanse of cities too).
pub(crate) fn is_liberate(d: &CardDef) -> bool {
    let lib = |e: &Op| matches!(e, Op::Liberate { .. } | Op::Cleanse { cities: true, .. });
    d.effects.iter().any(lib)
        || d.ability.as_ref().is_some_and(|a| {
            a.effects.iter().any(lib) || a.or.as_ref().is_some_and(|b| b.effects.iter().any(lib))
        })
}

/// The card's value to the plan, played `round` rounds from now with `tags` on
/// the seat's cards. On printed costs (before 082): its printed cost plus 1, plus
/// the jobs it would do (046) and a capstone's (070). By the table (082): its
/// ops' value at the rounds then left, times the seat's strategy multiplier, plus
/// the jobs it would do now.
#[allow(clippy::too_many_arguments)]
fn value(
    w: &World,
    g: &GameState,
    s: usize,
    d: &CardDef,
    f: &Forced,
    pc: &PlanCtx,
    pt: &PlanTable,
    round: usize,
    tags: &[i64; TAGS.len()],
    planned: &[Key],
) -> f64 {
    let jobs = f.plan_niche * niche_jobs(w, g, s, d) as f64;
    if !pc.table {
        return 1.0 + credit_cost(d) as f64 + jobs + f.plan_niche * capstone_jobs(d) as f64;
    }
    let t = pc.t;
    let ctx = Ctx {
        share: pt.share,
        team: pt.team,
        energy_rate: pc.energy_rate,
        cubes_open: pt.cubes_open,
        ..Ctx::at(t, (pt.left - round as f64).max(0.0), pt.node, *tags, pt.corrupted)
    };
    let m = match (pc.strat, pc.mults) {
        (Some(k), Some(mults)) => {
            let i = w.content.card_index(&d.id).unwrap_or(0);
            mults.get(k, i, strategy::phase(g.round + round as u32))
        }
        _ => 1.0,
    };
    // A building that must go over one of the seat's adds only what it beats the
    // one it covers by (082: kept and never played, it was worth less than that).
    let covered = match (f.plan_cover, &d.replaces) {
        (true, Some(tag)) => {
            let built = || g.pile(&cards::built(s)).iter().map(|id| cards::def(w, id)).filter(|b| b.tags.contains(tag));
            // 107 (`plan_under`): with none built, the planned building it goes over.
            let under: Vec<&CardDef> = if built().next().is_none() && f.plan_under {
                planned.iter().map(|id| cards::def(w, id)).filter(|b| b.is_building() && b.tags.contains(tag)).collect()
            } else {
                built().collect()
            };
            under
                .into_iter()
                // 102: what it would still earn; what playing it did is spent.
                .map(|b| buried_value(t, b, &ctx))
                .fold(f64::INFINITY, f64::min)
        }
        _ => f64::INFINITY,
    };
    let covered = if covered.is_finite() { covered.max(0.0) } else { 0.0 };
    m * CardValue::of(t, d).total(t, &ctx) - covered + jobs + situational(d, pc, pt)
}

/// 092: what building `d`, buried now, would still have earned: its standing
/// effects and a share of its Action, each round left. Its tags stay, and what
/// playing it did (production included) is spent.
fn buried_value(t: &value::Table, d: &CardDef, ctx: &Ctx) -> f64 {
    let standing = value::passives_worth(t, &d.passives);
    // An Action at the value table's own rates, as `CardValue::of` reads it.
    let action = d.ability.as_ref().map(|a| value::ability_worth(t.own(), a)).unwrap_or_default();
    let use_ = (action.instant(t, ctx) + action.per_round(ctx)).max(0.0);
    // 099: a one-shot Action (Popular Uprising) buried unused loses its one use,
    // not a share of it every round left.
    if d.ability.as_ref().is_some_and(|a| a.once && a.or.is_none()) {
        return (ctx.horizon * standing.per_round(ctx)).max(0.0) + use_;
    }
    let uses = if d.ability.is_some() { t.rate("uses_share") } else { 0.0 };
    let round = standing.per_round(ctx) + uses * use_;
    (ctx.horizon * round).max(0.0)
}

/// Whether a building that must go over one of the seat's can (024: the
/// plan kept them with nothing to go over, and they were never played).
fn replaceable(w: &World, g: &GameState, s: usize, d: &CardDef) -> bool {
    d.replaces.as_ref().is_none_or(|t| {
        g.pile(&cards::built(s)).iter().any(|id| cards::def(w, id).tags.contains(t))
    })
}

/// 107 (`plan_under`): whether a building that must go over one of the seat's
/// tag may go over a building the plan plays before it.
fn under_planned(w: &World, d: &CardDef, planned: &[Key]) -> bool {
    d.replaces.as_ref().is_some_and(|t| {
        planned.iter().map(|id| cards::def(w, id)).any(|b| b.is_building() && b.replaces.is_none() && b.tags.contains(t))
    })
}

/// What seat `s` pays to play `d`: credits after its building discount, its
/// other costs at fixed rates (before 082) or the table's (082).
fn spend(w: &World, g: &GameState, s: usize, d: &CardDef, pc: &PlanCtx, pt: &PlanTable) -> f64 {
    if pc.table {
        pc.t.cost(d, cards::play_cost(w, g, s, d), cards::play_insight(w, g, s, d), pt.left)
    } else {
        credit_equivalent(d, cards::play_cost(w, g, s, d)) as f64
    }
}

#[allow(clippy::too_many_arguments)]
fn ratio(w: &World, g: &GameState, s: usize, id: &str, f: &Forced, pc: &PlanCtx, pt: &PlanTable, tags: &[i64; TAGS.len()]) -> f64 {
    let d = cards::def(w, id);
    value(w, g, s, d, f, pc, pt, 0, tags, &[]) / (1.0 + spend(w, g, s, d, pc, pt))
}

/// `ids`, best value per cost first.
#[allow(clippy::too_many_arguments)]
fn by_ratio(w: &World, g: &GameState, s: usize, f: &Forced, pc: &PlanCtx, pt: &PlanTable, tags: &[i64; TAGS.len()], ids: &mut [Key]) {
    let mut keyed: Vec<(f64, Key)> = ids.iter().map(|id| (ratio(w, g, s, id, f, pc, pt, tags), *id)).collect();
    keyed.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    for (slot, (_, id)) in ids.iter_mut().zip(keyed) {
        *slot = id;
    }
}

/// The plan with `kept` bought now: its value, and the cards it plays.
/// `None` if the keeps can't be paid. By the table (082), a card's `needs` may be
/// met by the tags of the cards the plan plays before it, and a card with no
/// target now may be played in a later round, at the table's target share.
fn plan(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[Key],
    f: &Forced,
    pc: &PlanCtx,
    pt: &PlanTable,
) -> Option<(f64, Vec<Key>)> {
    let rounds = f.plan_rounds;
    let res = |r: Resource| g.resource(s, r).max(0);
    let income = economy::income(w, g, s);
    let inc = |r: Resource| income[r];
    let mut now = Budget {
        credits: res(Resource::Credits),
        insight: res(Resource::Insight),
        chips: res(Resource::Chips),
        energy: res(Resource::Energy),
        antimatter: res(Resource::Antimatter),
        research_cubes: res(Resource::ResearchCubes),
    };
    // Pay the keeps: insight first, the shortfall in credits.
    let rate = w.int("cards.insight_credits");
    let mut paid = 0;
    let mut count = g.var(&cards::kept_key(s));
    for id in kept {
        let tier = cards::def(w, id).tier;
        let (ins, cr) = cards::keep_need(w, g, s, tier, count);
        // 108: an opening tier-2 keep takes none of the free tier-1 keeps.
        count += cards::counts_as_kept(g, tier) as i64;
        let from_insight = ins.min(now.insight);
        now.insight -= from_insight;
        paid += cr + (ins - from_insight) * rate;
        now.credits -= cr + (ins - from_insight) * rate;
        if now.credits < 0 {
            return None;
        }
    }
    let mut tags = cards::tag_counts(w, g, s);
    let mut pool: Vec<Key> = g.pile(&cards::hand(s)).to_vec();
    pool.extend(kept.iter().cloned());
    by_ratio(w, g, s, f, pc, pt, &tags, &mut pool);
    let mut planned: Vec<Key> = Vec::new();
    let mut total = 0.0;
    let mut b = now;
    let mut needs = cards::required_tags(w, g, s);
    let needs_now = needs;
    // 107 (`plan_reach`): the tags the hand and the keeps would add, a card's own aside.
    let mut reach = needs_now;
    if f.plan_reach && pc.table {
        for id in &pool {
            let d = cards::def(w, id);
            if !d.is_event() {
                for t in &d.tags {
                    reach[*t as usize] += 1;
                }
            }
        }
    }
    // 107: a seat builds toward a chain card only on its strategy's tags ("commit
    // to the tags your strategy plays"); greedy, with no strategy, never does.
    let committed = |d: &CardDef| {
        pc.strat.is_some_and(|k| {
            let spec = &strategy::strategies().strategies[k as usize];
            d.needs.iter().all(|(t, _)| spec.tag_weight(*t) > 0.0)
        })
    };
    let near = |d: &CardDef| {
        let by_hand = f.plan_reach && committed(d);
        let own = |t: usize| if by_hand && !d.is_event() { d.tags.iter().filter(|x| **x as usize == t).count() as i64 } else { 0 };
        let have = |t: usize| if by_hand { reach[t] - own(t) } else { needs_now[t] };
        !pc.table || d.needs.iter().map(|(t, n)| (n - have(*t as usize)).max(0)).sum::<i64>() <= f.plan_short as i64
    };
    let patient = economy::patient(w, s);
    let share = pc.t.rate("target_share");
    // 092 (`plan_bury`): the slots still open, and the tops a building may bury.
    let mut tops: Vec<&CardDef> = g.pile(&cards::built(s)).iter().map(|id| cards::def(w, id)).collect();
    let mut open = cards::capacity(w, g, s) - tops.len() as i64;
    for round in 0..rounds.max(1) {
        if round > 0 {
            // Credits, chips, energy and antimatter carry over; insight resets,
            // but under Patience (075).
            b = Budget {
                credits: b.credits + inc(Resource::Credits),
                insight: inc(Resource::Insight).max(0) + if patient { b.insight } else { 0 },
                chips: b.chips + inc(Resource::Chips),
                energy: b.energy + inc(Resource::Energy),
                antimatter: b.antimatter + inc(Resource::Antimatter),
                research_cubes: b.research_cubes + inc(Resource::ResearchCubes),
            };
        }
        for id in &pool {
            if planned.contains(id) {
                continue;
            }
            let d = cards::def(w, id);
            let credits = cards::play_cost(w, g, s, d);
            // 118: as the engine judges a play (a program counts its own tags: F183).
            let target = d.is_building() || effects::feasible(w, g, s, &effects::own_counted(d));
            let later = if pc.table {
                round > 0 && pile_ok(w, g, s, d, &planned) && (!f.plan_copy || copy_ok(w, g, s, d, &planned))
            } else {
                liberate_later(w, g, d)
            };
            let cubes = if f.plan_cubes { b.research_cubes } else { g.resource(s, Resource::ResearchCubes) };
            let fits = b.credits >= credits
                && b.insight >= cards::play_insight(w, g, s, d)
                && b.chips >= d.chips
                && b.energy >= d.energy
                && b.antimatter >= d.antimatter
                && cubes >= d.research_cubes
                && cards::lowers_ok(w, g, s, d)
                && (replaceable(w, g, s, d) || (f.plan_under && pc.table && under_planned(w, d, &planned)))
                && (target || later)
                && near(d)
                && d.needs.iter().all(|(t, n)| needs[*t as usize] >= *n);
            // 102 (D34): at the table a card adds its value less its price, and one
            // worth no more than it costs is no plan: the play check nets the price,
            // so spare credits kept cards it never played (099).
            let (v, price) = if fits {
                let v = value(w, g, s, d, f, pc, pt, round, &tags, &planned);
                let price = if pc.table {
                    // 102: credits over the heuristic's cap are worth its
                    // idle rate to it, so a price paid from them costs that much.
                    let (cap, share) = pc.idle;
                    let idle_paid = (credits as f64).min((b.credits as f64 - cap).max(0.0));
                    // 128: its energy at the seat's rate (spare energy is cheap).
                    let spare = d.energy as f64 * pc.t.res(Resource::Energy) * (1.0 - pc.energy_rate);
                    spend(w, g, s, d, pc, pt) - idle_paid * (1.0 - share) - spare
                } else {
                    0.0
                };
                (v, price)
            } else {
                (0.0, 0.0)
            };
            if fits && pc.table && v <= price {
                continue;
            }
            if fits {
                b.credits -= credits;
                b.insight -= cards::play_insight(w, g, s, d);
                b.chips -= d.chips;
                b.energy -= d.energy;
                b.antimatter -= d.antimatter;
                if f.plan_cubes {
                    b.research_cubes -= d.research_cubes;
                    b.research_cubes += d
                        .effects
                        .iter()
                        .map(|e| match e {
                            Op::Gain { resource: Resource::ResearchCubes, amount } => *amount,
                            _ => 0,
                        })
                        .sum::<i64>();
                }
                let found = if target || !pc.table { 1.0 } else { share };
                // 092: with every slot full, a plain building buries the top that
                // would earn least over the rounds left.
                let buried = if f.plan_bury && pc.table && d.is_building() && d.replaces.is_none() {
                    if open > 0 {
                        open -= 1;
                        0.0
                    } else {
                        let ctx = Ctx { share: pt.share, ..Ctx::at(pc.t, (pt.left - round as f64).max(0.0), pt.node, tags, pt.corrupted) };
                        let worst = (0..tops.len()).min_by(|&i, &j| {
                            buried_value(pc.t, tops[i], &ctx).total_cmp(&buried_value(pc.t, tops[j], &ctx))
                        });
                        match worst {
                            Some(i) => buried_value(pc.t, tops.swap_remove(i), &ctx),
                            None => 0.0,
                        }
                    }
                } else {
                    0.0
                };
                // 104 (Matt, 2026-11-04): a card worth no more than its price is
                // dropped above, and one that is adds its gross value (099's form).
                total += found * (v - buried) * (1.0 - WAIT * round as f64).max(0.1);
                planned.push(*id);
                if pc.table && !d.is_event() {
                    // 082: what it leaves in play counts toward later cards' needs.
                    for t in &d.tags {
                        needs[*t as usize] += 1;
                        tags[*t as usize] += 1;
                    }
                }
            }
        }
    }
    // 107 (`plan_chain`): the chain cards the plan builds toward but can't play
    // within its rounds count their value after it, times their share reached.
    if f.plan_chain && pc.table {
        let last = rounds.max(1);
        for id in &pool {
            let d = cards::def(w, id);
            if planned.contains(id) || d.needs.is_empty() || !committed(d) {
                continue;
            }
            let short: i64 = d.needs.iter().map(|(t, n)| (n - needs[*t as usize]).max(0)).sum();
            if short == 0 || short > f.plan_short as i64 {
                continue;
            }
            let want: i64 = d.needs.iter().map(|(_, n)| *n).sum();
            let share = (want - short) as f64 / want.max(1) as f64;
            let v = value(w, g, s, d, f, pc, pt, last, &tags, &planned) - spend(w, g, s, d, pc, pt);
            if v > 0.0 {
                total += share * share * v * (1.0 - WAIT * last as f64).max(0.1);
                planned.push(*id);
            }
        }
    }
    // 024: credits paid to keep are credits not spent on the board (the
    // opening's 5-credit keeps took all 20 starting credits).
    Some((total - f.plan_credit * paid as f64, planned))
}

/// 082: whether a card that works on a token pile of the seat's own (a feeder,
/// a cash-out, a count) has one to work on: a token card of its kind standing or
/// planned before it. A pile can't arrive from the board, as a target can.
fn pile_ok(w: &World, g: &GameState, s: usize, d: &CardDef, planned: &[Key]) -> bool {
    let kinds: Vec<aic_core::content::TokenKind> = d
        .effects
        .iter()
        .filter_map(|e| match e {
            Op::AddTokens { kind, here: false, .. } | Op::CashOut { kind, here: false, .. } => Some(*kind),
            Op::GainPerTokens { kind, here: false, .. } => Some(*kind),
            _ => None,
        })
        .collect();
    kinds.iter().all(|k| {
        cards::has_token_card(w, g, s, Some(*k))
            || planned.iter().any(|id| cards::def(w, id).tokens.as_ref().is_some_and(|t| t.kind == *k))
    })
}


/// 107 (`plan_copy`): whether a card that copies a building's income has one to
/// copy, built or planned before it.
fn copy_ok(w: &World, g: &GameState, s: usize, d: &CardDef, planned: &[Key]) -> bool {
    if !d.effects.iter().any(|e| matches!(e, Op::CopyIncome)) {
        return true;
    }
    let earns = |b: &CardDef| {
        b.is_building() && b.passives.iter().any(|p| matches!(p, Op::Income { .. } | Op::IncomePerTag { .. }))
    };
    g.pile(&cards::built(s)).iter().chain(planned).any(|id| earns(cards::def(w, id)))
}

/// 080: a liberate card with no corrupted city in reach now still plans to
/// play while a city is corrupted: a node can get there.
fn liberate_later(w: &World, g: &GameState, d: &CardDef) -> bool {
    is_liberate(d) && (0..g.hexes.len()).any(|h| common::corrupted_city(w, g, h))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// A fixed policy with one parameter on and all else 0.
    fn forced(name: &str) -> Policy {
        let f = Forced::read(|k| f64::from(k == name));
        Policy::new(Box::new(crate::agents::tests::First), f)
    }

    /// 119: `aic-sim cards` lists every content card once, with the agents' own tests.
    #[test]
    fn the_cards_listing_carries_the_engine_test() {
        let csv = cards_csv();
        let rows: Vec<&str> = csv.lines().skip(1).collect();
        let all = &aic_core::content::content().cards;
        assert_eq!(rows.len(), all.len());
        let engines = rows.iter().filter(|r| r.split(',').nth(3) == Some("1")).count();
        assert_eq!(engines, all.iter().filter(|d| is_engine(d)).count());
        assert!(engines > 0 && rows.iter().all(|r| r.split(',').count() == 5), "{csv}");
    }

    /// 082: the strategies are chosen at the round-1 keep from each seat's board's
    /// menu, the same on the same table, a fixed one wins, and a rollout's policy
    /// handed them doesn't choose again.
    #[test]
    fn strategies_are_chosen_at_the_opening_keep() {
        let r = crate::agents::tests::rules(2);
        let w = r.world();
        let mut g = r.new_game(5);
        while g.config.round[g.phase] != "cards.keep" {
            let a = r.legal_actions(&g)[0].clone();
            g = r.apply_to(&g, &a, None).unwrap();
        }
        assert_eq!(g.round, 1);
        let legal = r.legal_actions(&g);
        let chosen = |name: &str, v: f64| {
            let mut p = Policy::new(Box::new(crate::agents::tests::First), Forced::read(|k| if k == name { v } else { 0.0 }));
            p.choose(&r, &g, &legal);
            p.strat
        };
        let a = chosen("strategy", 1.0);
        assert_eq!(a, chosen("strategy", 1.0));
        for (s, k) in a.iter().enumerate().take(g.seats.len()) {
            assert!(strategy::menu(&w, s).contains(&k.unwrap()), "seat {s}");
        }
        assert_eq!(chosen("strat0", 2.0)[0], Some(strategy::menu(&w, 0)[1]));
        let handed = Policy::new(Box::new(crate::agents::tests::First), Forced::read(|_| 0.0)).with_strategies(a);
        assert!(handed.chosen && handed.strat == a);
    }

    #[test]
    fn the_node_checks_rule_out_the_powers_they_name() {
        let r = crate::agents::tests::rules(0);
        let g = r.new_game(1);
        let w = r.world();
        let by_name = |name: &str| {
            let k = ai::board(&w, 0).powers.iter().position(|p| p.name == name).unwrap();
            Action::Power { seat: 0, power: k as u8 }
        };
        let (deploy, buy, scrap) = (by_name("Deploy"), by_name("Buy In"), by_name("Scrap"));
        let no_powers = forced("no_powers");
        assert!(no_powers.ruled_out(&w, &g, &scrap) && !no_powers.ruled_out(&w, &g, &deploy));
        let no_node = forced("no_node");
        assert!(no_node.ruled_out(&w, &g, &deploy) && no_node.ruled_out(&w, &g, &buy));
        assert!(!no_node.ruled_out(&w, &g, &scrap));
        let never_second = forced("never_second_node");
        assert!(never_second.ruled_out(&w, &g, &buy) && !never_second.ruled_out(&w, &g, &deploy));
    }

    #[test]
    fn a_shield_counts_only_the_node_it_can_cover() {
        let r = crate::agents::tests::rules(0);
        let mut g = r.new_game(1);
        let w = r.world();
        let shield = cards::def(&w, "benchmark_suite");
        let open = (0..g.hexes.len()).filter(|&h| effects::shieldable(&g, 0, h)).count() as i64;
        assert!(open > 0);
        assert_eq!(niche_jobs(&w, &g, 0, shield), open.min(2));
        for h in Arc::make_mut(&mut g.hexes).iter_mut() {
            Arc::make_mut(h).tokens.clear();
        }
        assert_eq!(niche_jobs(&w, &g, 0, shield), 0);
    }
}
