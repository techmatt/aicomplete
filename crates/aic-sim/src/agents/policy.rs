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
//! The plan itself is [`super::keep_plan`]; the card tests the checks and the
//! strategies share are [`super::cards`].

use std::sync::Arc;

use aic_core::content::{CardDef, Op, Resource, Tag};
use aic_core::modules::{ai, board, cards, common, economy, threats};
use aic_core::module::World;
use aic_core::state::AskKind;
use aic_core::{fnv1a64, Action, Agent, GameState, Key, Rules};

use super::credit_cost;
use super::keep_plan::{plan_keeps, PlanCtx};
use super::strategy::{self, Choice, Mults};
use super::value::{self, CardValue, Ctx};
use super::Strategic;

pub use super::cards::cards_csv;
pub(crate) use super::cards::{is_capstone, is_engine, is_liberate};

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
/// (a plan's numbers do nothing alone; the rules every agent follows are [`behavior_table`]'s, which
/// always wrap it). It makes the [`Forced`] field, the agent
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
            /// What the keep plan adds, in credits, for each marker a card would clear,
            /// threat it would delay and node it would shield on the table as it
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
            /// (The play check's twin, `w.team_share`, was retired at 165.)
            (plan_team: bool, false, 0.0, "124 (D47): keep plan: a trigger on a teammate's plays fires at the teammates' share of the tag")
            /// 128: the keep plan counts an Action that places project cubes only
            /// while a megaproject is open to the seat (Particle Accelerator).
            /// The play check's twin is `w.cube_target`: screen the two together.
            (plan_cube_target: bool, false, 1.0, "128: keep plan: a project-cube Action counts no uses once no megaproject is open")
            /// 150: under Patience with insight beyond the heuristic's `insight_cap`,
            /// the keep plan prices an insight income at the heuristic's idle rate,
            /// as the play eval does (its horizon term): keep and play agree
            /// (Exascale Training Cluster, +3 insight a round, was kept in 62-76% of
            /// games and played in 1-3%; with the Researcher, 86% and 0%).
            (plan_spare: bool, false, 1.0, "150: keep plan: under Patience with spare insight, an insight income is worth the heuristic's idle rate, as in its play eval")
        }
    };
}
pub(crate) use forced_table;

/// The agent behaviors (185, from 182's F31): rules a player follows whatever the keep plan or a dominance check
/// says, so any value of theirs on, at their defaults, wraps the agent in a [`Policy`] (they sat in
/// [`forced_table`] as rows that wrapped nothing alone, so `plan_keep=0` turned them off unseen). A row is
/// `(name: type, default, what it does)`; it makes the [`Behaviors`] field and the agent parameter of the same
/// name, listed after [`forced_table`]'s. Hands the rows to `$then!`, after `$args`.
macro_rules! behavior_table {
    ($then:ident ($($args:tt)*)) => {
        $then! {
            ($($args)*)
            /// 166: the move heuristics answer Gather and Push prompts (`agents::moves`: push agents off the
            /// next Corrupt site, gather agents onto ambassadors, gather ambassadors toward a Liberate).
            (move_rules: bool, 1.0, "166: move heuristics for Gather and Push (push agents off the next Corrupt site, gather agents onto ambassadors, gather ambassadors to a Liberate)")
            /// 175: a Corrupt X line's agent next to several sites goes where the players lose least
            /// (`plan::converge_pick`: onto a site corrupting anyway, else the one holding fewest).
            (converge_rule: bool, 1.0, "175: a Corrupt line's converging agent goes onto a site corrupting anyway, else the one holding fewest")
            /// 176: Buy energy (r75's action-phase purchase) when the credits it costs would sit idle: the seat's
            /// credits and this round's credit income, less the price, still reach the eval's credit cap. A pass is
            /// scored on the table it leaves, before income, so the search alone never saw idle credits here
            /// (r74's production prompt looked past the income).
            (energy_rule: bool, 1.0, "176: buy a neutral city's energy when its credits would sit idle (credits plus this round's income, less the price, at or over the eval's credit cap)")
            /// 178: a strike's batches (`agents::moves::strike`): the largest of troops, then of agents, never a
            /// stop; its hex stays the search's. Its own lever since 185 (it sat under `move_rules`).
            (strike_rule: bool, 1.0, "178: a strike's batches go largest troops first, then agents, never a stop (its hex is the search's)")
        }
    };
}
pub(crate) use behavior_table;

macro_rules! behavior_struct {
    (() $($(#[$doc:meta])* ($name:ident: $ty:ty, $default:expr, $desc:expr))*) => {
        /// The agent behaviors ([`behavior_table`]): on for every agent that has them unless a spec turns one off.
        #[derive(Debug, Clone)]
        pub struct Behaviors {
            $($(#[$doc])* pub $name: $ty,)*
        }

        impl Behaviors {
            /// Each field from its agent parameter.
            pub fn read(num: impl Fn(&str) -> f64) -> Behaviors {
                Behaviors { $($name: ForcedValue::from_param(num(stringify!($name))),)* }
            }

            /// Whether any behavior is on (the agent is then wrapped in a [`Policy`]).
            pub fn is_active(&self) -> bool {
                $(self.$name.is_on())||*
            }
        }
    };
}

behavior_table!(behavior_struct());

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

/// The credits a seat holds before more sit idle: the heuristic's `credit_cap` (its default with no heuristic),
/// beyond which a credit is worth `credit_idle`; 0 or less is no cap, as the eval reads it.
fn idle_credits(ev: Option<&super::eval::Eval>) -> f64 {
    let cap = ev.map_or_else(|| super::eval::Weights::default().credit_cap, |e| e.w.credit_cap);
    if cap > 0.0 { cap } else { f64::MAX }
}

/// 176 (`energy_rule`): a Buy energy action whose credits would sit idle: the seat has no card left it can play
/// this phase, and its credits plus this round's credit income, less the price, still reach `idle`
/// ([`idle_credits`]).
fn buy_energy(w: &World, g: &GameState, legal: &[Action], idle: f64) -> Option<usize> {
    legal.iter().position(|a| match a {
        Action::Standard { seat, act } if act == board::StandardAction::BuyEnergy.name() => {
            let s = *seat as usize;
            if legal.iter().any(|b| matches!(b, Action::Play { seat, .. } if *seat as usize == s)) {
                return false;
            }
            let price = board::seat_price(w, g, s, board::StandardAction::BuyEnergy).credits;
            let after = g.resource(s, Resource::Credits) + economy::income(w, g, s)[Resource::Credits] - price;
            after as f64 >= idle
        }
        _ => false,
    })
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
    /// Whether `forced` holds anything (no: nothing is ruled out, so `choose` skips the filter).
    forcing: bool,
    behaviors: Behaviors,
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
    pub fn new(inner: Box<dyn Strategic + Send>, forced: Forced, behaviors: Behaviors) -> Policy {
        Policy {
            inner,
            forcing: forced.is_active(),
            forced,
            behaviors,
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
        let super::eval::PlanRates { energy, antimatter, project, cap, idle, insight_cap, insight_idle, energy_cap, energy_share } =
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
        // 150 (`plan_spare`): a Patience seat's insight beyond the cap is spare, so
        // more insight income is worth the idle rate, as the play eval reads it.
        let spare = economy::patient(&world, s) && insight_cap > 0.0 && g.resource(s, Resource::Insight) as f64 > insight_cap;
        let insight_idle = (self.forced.plan_spare && spare).then_some(insight_idle);
        let pc = PlanCtx { table: self.forced.plan_table, strat, mults: self.mults.as_deref(), t, project, idle, spare_at, energy_rate, insight_idle };
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
                    || (f.turtle && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "harden_chips" | "harden_buy"))
                    || (f.free_only && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "move"))
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
        // The agent behaviors (185): each pick stands only where no forced behavior rules it out (182's F32).
        let b = &self.behaviors;
        if b.converge_rule && pending {
            let last_node = self.eval().is_some_and(|e| e.w.last_node > 0.0);
            if let Some(i) = super::moves::converge(&world, g, legal, last_node) {
                if !self.ruled_out(&world, g, &legal[i]) {
                    return i;
                }
            }
        }
        if b.energy_rule && !pending {
            if let Some(i) = buy_energy(&world, g, legal, idle_credits(self.eval())) {
                if !self.ruled_out(&world, g, &legal[i]) {
                    return i;
                }
            }
        }
        if b.strike_rule && pending {
            if let Some((i, which)) = super::moves::strike(g, legal) {
                if !self.ruled_out(&world, g, &legal[i]) {
                    super::moves::fired(which);
                    return i;
                }
            }
        }
        if b.move_rules && pending {
            let last_node = self.eval().is_some_and(|e| e.w.last_node > 0.0);
            if let Some((i, which)) = super::moves::pick(&world, g, legal, last_node) {
                if !self.ruled_out(&world, g, &legal[i]) {
                    super::moves::fired(which);
                    return i;
                }
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
        if !self.forcing {
            return self.inner.choose(rules, g, legal);
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
    threats::active(g).iter().any(|(t, c)| t == threat && *c != common::home_country(w, s))
}

/// Whether `threat` sits on the country the abandon check gives up.
fn on_given_up(w: &World, g: &GameState, threat: &str) -> bool {
    threats::active(g).iter().any(|(t, c)| t == threat && *c == last_country(w))
}

/// Whether seat `s` could pay to keep a card of `tier` now (insight, with
/// any shortfall in credits).
fn keep_affordable(w: &World, g: &GameState, s: usize, tier: u8) -> bool {
    let (ins, cr) = cards::keep_need(w, g, s, tier, 0);
    let short = (ins - g.resource(s, Resource::Insight).max(0)).max(0);
    g.resource(s, Resource::Credits) >= cr + short * w.int("cards.insight_credits")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fixed policy with one parameter on and all else 0.
    fn forced(name: &str) -> Policy {
        let f = Forced::read(|k| f64::from(k == name));
        Policy::new(Box::new(crate::agents::tests::First), f, Behaviors::read(|_| 0.0))
    }

    /// 185 (182's F31): the agent behaviors stay on when the keep plan is off, so such an agent is still
    /// wrapped; a spec turns one off by its old name, and the label prints it as before.
    #[test]
    fn behaviors_outlive_the_keep_plan() {
        let spec = crate::agents::AgentSpec::parse("greedy:plan_keep=0").unwrap();
        assert!(!spec.forced().is_active());
        let b = spec.behaviors();
        assert!(b.is_active() && b.move_rules && b.converge_rule && b.energy_rule && b.strike_rule);
        let off = crate::agents::AgentSpec::parse("greedy:move_rules=0").unwrap();
        assert!(!off.behaviors().move_rules && off.behaviors().strike_rule);
        assert_eq!(off.label(), "greedy:move_rules=0");
        let strike = crate::agents::AgentSpec::parse("engine:strike_rule=0").unwrap();
        assert!(!strike.behaviors().strike_rule && strike.behaviors().move_rules);
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
            let mut p = Policy::new(Box::new(crate::agents::tests::First), Forced::read(|k| if k == name { v } else { 0.0 }), Behaviors::read(|_| 0.0));
            p.choose(&r, &g, &legal);
            p.strat
        };
        let a = chosen("strategy", 1.0);
        assert_eq!(a, chosen("strategy", 1.0));
        for (s, k) in a.iter().enumerate().take(g.seats.len()) {
            assert!(strategy::menu(&w, s).contains(&k.unwrap()), "seat {s}");
        }
        assert_eq!(chosen("strat0", 2.0)[0], Some(strategy::menu(&w, 0)[1]));
        let handed = Policy::new(Box::new(crate::agents::tests::First), Forced::read(|_| 0.0), Behaviors::read(|_| 0.0)).with_strategies(a);
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
        let (deploy, buy, scrap) = (by_name("Expand"), by_name("Buy In"), by_name("Scrap"));
        let no_powers = forced("no_powers");
        assert!(no_powers.ruled_out(&w, &g, &scrap) && !no_powers.ruled_out(&w, &g, &deploy));
        let no_node = forced("no_node");
        assert!(no_node.ruled_out(&w, &g, &deploy) && no_node.ruled_out(&w, &g, &buy));
        assert!(!no_node.ruled_out(&w, &g, &scrap));
        let never_second = forced("never_second_node");
        assert!(never_second.ruled_out(&w, &g, &buy) && !never_second.ruled_out(&w, &g, &deploy));
    }
}
