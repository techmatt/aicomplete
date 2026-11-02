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
//! chips and energy income; the cards it would play (hand plus the
//! candidates) in value-per-cost order, filled into each round's budget.

use aic_core::content::{CardDef, CardKind, Op, Resource, Tag};
use aic_core::modules::{ai, cards, common, economy, effects, threats};
use aic_core::module::World;
use aic_core::state::AskKind;
use aic_core::{fnv1a64, Action, Agent, GameState, Key, Rules};

use super::credit_equivalent;

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
            /// Insight the keep plan holds back in each later round for that round's
            /// own keeps (024: the plan spent all of it on plays, so it thought it
            /// could play twice what it could and kept double what it played).
            (plan_reserve: i64, false, 0.0, "insight the keep plan holds back in each later round for that round's keeps")
            /// The keep plan keeps nothing while the hand holds this many cards
            /// (024; 0: off). Hands of 8-10 held cards never played: at a table a
            /// player stops paying for cards they can't use.
            (plan_hand: usize, false, 0.0, "keep nothing while the hand holds this many cards (0: off)")
            /// Share of a tier-2 card's value the keep plan counts (024): its printed
            /// cost overstates what the one-step play logic gets from it, and a keep
            /// costs 2-3 insight.
            (plan_t2: f64, false, 1.0, "share of a tier-2 card's value the keep plan counts")
            /// What a credit paid for a keep costs the plan (024).
            (plan_credit: f64, false, 3.0, "what a credit paid for a keep costs the keep plan")
            /// What the keep plan adds, in credits, for each marker a card would clear
            /// threat it would delay and presence it would shield on the table as it
            /// stands (046: priced at its printed cost, a 2-credit Science card lost
            /// every keep to a tier-2).
            (plan_niche: f64, false, 3.0, "credits the keep plan adds per marker a card would clear now, threat it would delay and presence it would shield")
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
            /// Never use an AI-board power, the presence powers aside (066: they
            /// are how anyone places, which `no_presence` checks).
            (no_powers: bool, true, 0.0, "dominance check: never use an AI-board power but the presence powers")
            /// Never pay toward a threat's counter.
            (no_counters: bool, true, 0.0, "dominance check: never counter a threat")
            /// Place no presence beyond setup (no power or card that places one).
            (no_presence: bool, true, 0.0, "dominance check: place no presence beyond setup")
            /// Mono-tag (023): keep and play only cards carrying this tag.
            (only_tag: Option<Tag>, true, 0.0, "dominance check (023): keep and play only this tag (1-6: science, industry, cyber, power, diplomacy, security)")
            /// Ignore-X (023): never keep or play a card carrying this tag.
            (never_tag: Option<Tag>, true, 0.0, "dominance check (023): never keep or play this tag (1-6 as only_tag)")
            /// Event-hoarder (023): keep and play only events.
            (events_only: bool, true, 0.0, "dominance check (023): keep and play only events")
            /// Exploit check (027): no counter paid and no card that counters a threat.
            (ignore_threats: bool, true, 0.0, "exploit check (027): no counters, no card that counters a threat")
            /// Exploit check (027): stay on the setup hex and defend it: no presence,
            /// only the remove, clear, ambassador, hardening and chips standard actions.
            (turtle: bool, true, 0.0, "exploit check (027): no presence beyond setup; only remove, clear, chips, ambassador and hardening actions")
            /// Exploit check (027): only free cities: Diplomacy cards, the ambassador,
            /// move, remove, clear and chips standard actions, no counters, no presence,
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
            (never_remove: bool, true, 0.0, "fixed policy (041): never clear a corruption marker")
            /// Fixed policy (041): the Science rush, board cards first.
            (science_board: bool, true, 0.0, "fixed policy (041): the Science rush, board cards first")
            /// Fixed policy (066): both presence powers whenever affordable.
            (always_max_presence: bool, true, 0.0, "fixed policy (066): use both presence powers whenever affordable")
            /// Fixed policy (066): never the board's second presence power.
            (never_second_presence: bool, true, 0.0, "fixed policy (066): never use the board's second presence power")
            /// Fixed policy (068): no power that sacrifices.
            (never_sacrifice: bool, true, 0.0, "fixed policy (068): never use a power that sacrifices")
            /// Fixed policy (068): buy every ambassador it can.
            (ambassador_mass: bool, true, 0.0, "fixed policy (068): before anything else, buy every ambassador it can afford")
            /// Fixed policy (068): the Builder's expensive power at every chance.
            (free_build_rush: bool, true, 0.0, "fixed policy (068): the Builder's free-building power at every chance")
            /// Fixed policy (068): a seat with Wall places presence only to block troops.
            (wall_turtle: bool, true, 0.0, "fixed policy (068): a seat with Wall places presence only with a troop on the board, on the hex nearest one")
            /// Fixed policy (070): pays only threats in its own home country.
            (home_only: bool, true, 0.0, "fixed policy (070): counter only threats in your home country")
            /// Fixed policy (070): tier-2 capstones first.
            (capstone_rush: bool, true, 0.0, "fixed policy (070): tier 2 whenever the keep is affordable, and tier-2 capstones before anything else")
        }
    };
}
pub(crate) use forced_table;

macro_rules! forced_struct {
    (() $($(#[$doc:meta])* ($name:ident: $ty:ty, $active:literal, $default:expr, $desc:expr))*) => {
        /// The behavior a dominance check forces (none for thoughtful play).
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
    /// Whether this seat places no presence beyond setup.
    fn stays_put(&self) -> bool {
        self.no_presence || self.turtle || self.free_only
    }

    /// Whether the card restrictions (mono-tag, ignore-X, event-hoarder)
    /// let this seat keep or play `d`. AI-unique cards (tier 0) are exempt.
    pub fn allows(&self, d: &CardDef) -> bool {
        if d.tier == 0 {
            return true;
        }
        let has = |t: &Option<Tag>| t.as_ref().is_some_and(|t| d.tags.contains(t));
        (self.only_tag.is_none() || has(&self.only_tag))
            && !has(&self.never_tag)
            && (!self.events_only || d.is_event())
            && (!self.free_only || d.tags.contains(&Tag::Diplomacy))
            && (!self.ignore_threats || !counters_threats(&d.effects))
            && (!self.never_remove || !d.effects.iter().any(|e| matches!(e, Op::ClearMarkers { .. })))
    }
}

/// Whether `ops` counter a threat or make one cheaper.
fn counters_threats(ops: &[Op]) -> bool {
    ops.iter().any(|o| matches!(o, Op::CounterThreat | Op::Jam { .. } | Op::DelayThreat { .. }))
}

pub struct Policy {
    inner: Box<dyn Agent + Send>,
    forced: Forced,
    /// This round's keep set for a seat: (round, seat, cards).
    keeps: Vec<(u32, usize, Vec<Key>)>,
}

impl Policy {
    pub fn new(inner: Box<dyn Agent + Send>, forced: Forced) -> Policy {
        Policy {
            inner,
            forced,
            keeps: Vec::new(),
        }
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
        let set = if self.forced.keep_none {
            Vec::new()
        } else if self.forced.keep_random {
            let coin = |id: &Key| fnv1a64(format!("{}:{round}:{id}", g.seed).as_bytes()) & 1 == 0;
            drafted.into_iter().filter(coin).collect()
        } else if self.forced.keep_all {
            drafted
        } else if self.forced.science_rush || self.forced.science_board || self.forced.capstone_rush {
            let mut set = plan_keeps(&world, g, s, &self.forced);
            for id in &drafted {
                let d = cards::def(&world, id);
                let rush = if self.forced.capstone_rush { is_capstone(d) } else { d.tags.contains(&Tag::Science) };
                if rush && !set.contains(id) {
                    set.push(*id);
                }
            }
            set
        } else if self.forced.keep_k > 0 {
            let mut d = drafted;
            d.sort_by(|a, b| {
                list_cost(cards::def(&world, a))
                    .cmp(&list_cost(cards::def(&world, b)))
                    .then_with(|| a.cmp(b))
            });
            d.truncate(self.forced.keep_k);
            d
        } else if full {
            Vec::new()
        } else {
            plan_keeps(&world, g, s, &self.forced)
        };
        let set: Vec<Key> = set
            .into_iter()
            .filter(|id| self.forced.allows(cards::def(&world, id)))
            .collect();
        self.keeps.retain(|(r, _, _)| *r == round);
        self.keeps.push((round, s, set.clone()));
        set
    }

    /// Whether `a` is an action the forced behavior rules out.
    fn ruled_out(&self, world: &World, g: &GameState, a: &Action) -> bool {
        let f = &self.forced;
        match a {
            Action::Pay { threat, .. } if f.abandon && on_given_up(world, g, threat) => true,
            Action::Pay { seat, threat, .. } if f.home_only && abroad(world, g, *seat as usize, threat) => true,
            Action::Choose { seat, option, .. } if f.home_only && abroad(world, g, *seat as usize, option) => true,
            Action::Choose { option, target, .. } if f.abandon => {
                on_given_up(world, g, option)
                    || (!losing_pick(g) && target.and_then(|t| world.map.find(t)).is_some_and(|h| given_up(world, h)))
            }
            Action::Tier { tier: 2, .. } => f.tier1_only,
            Action::Power { seat, power } => {
                let (s, k) = (*seat as usize, *power as usize);
                let sacrifices = !ai::board(world, s).powers[k].tags.is_empty();
                (f.never_sacrifice && sacrifices)
                    || if ai::power_places(world, s, k) {
                        f.stays_put()
                            || (f.never_second_presence && second_presence(world, s, k))
                            || (f.wall_turtle && has_wall(world, g, s) && !troops_up(g))
                    } else {
                        f.no_powers || f.free_only
                    }
            }
            Action::Pay { .. } => f.no_counters || f.ignore_threats || f.free_only,
            Action::Standard { act, .. } => {
                (f.never_remove && act == "clear")
                    || (f.turtle && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "harden_chips" | "harden_buy" | "chips"))
                    || (f.free_only && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "move" | "chips"))
            }
            Action::Play { card, seat } => {
                let d = cards::def(world, card);
                let walls = f.wall_turtle && has_wall(world, g, *seat as usize) && !troops_up(g);
                ((f.stays_put() || walls) && places_presence(&d.effects)) || !f.allows(d)
            }
            Action::Keep { card, .. } => !f.allows(cards::def(world, card)),
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

/// The wall turtle's hex (068): of a presence prompt's hexes, the one nearest a
/// troop but not on it, where a wall blocks nothing.
fn wall_hex(w: &World, g: &GameState, legal: &[Action]) -> Option<usize> {
    let p = g.pending.as_ref()?;
    if !p.ask.prompt.contains("place presence") {
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

fn places_presence(effects: &[Op]) -> bool {
    effects.iter().any(|e| matches!(e, Op::PlacePresence { .. }))
}

/// Whether power `k` is seat `s`'s second presence power: one that places,
/// after the board's first that does.
fn second_presence(w: &World, s: usize, k: usize) -> bool {
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
            Op::ClearMarkers { .. } | Op::ShieldPresence { .. } | Op::DelayThreat { .. } | Op::MoveOne { .. }
        )
    })
}

impl Agent for Policy {
    fn choose(&mut self, rules: &Rules, g: &GameState, legal: &[Action]) -> usize {
        let world = rules.world();
        let phase = g.config.round.get(g.phase).map(String::as_str).unwrap_or("");
        let pending = legal.iter().any(|a| matches!(a, Action::Choose { .. }));
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
        if self.forced.wall_turtle {
            if let Some(i) = wall_hex(&world, g, legal) {
                return i;
            }
        }
        if self.forced.always_max_presence && !pending {
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

/// A card's printed cost in credits (antimatter, insight, energy, chips
/// and lowered income at fixed rates): the plan's measure of what it does.
pub fn list_cost(d: &CardDef) -> i64 {
    credit_equivalent(d, d.cost)
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
}

/// The drafted cards seat `s` keeps: each, best value per cost first, if
/// the plan with it is worth more than the plan without it.
pub fn plan_keeps(w: &World, g: &GameState, s: usize, f: &Forced) -> Vec<Key> {
    let mut drafted = g.pile(&cards::drafted(s)).to_vec();
    by_ratio(w, g, s, f, &mut drafted);
    let mut kept: Vec<Key> = Vec::new();
    let mut best = plan_value(w, g, s, &kept, f);
    for c in drafted {
        let mut with = kept.clone();
        with.push(c);
        let Some(v) = plan_value_checked(w, g, s, &with, &c, f) else {
            continue;
        };
        if v > best + 1e-9 {
            best = v;
            kept = with;
        }
    }
    kept
}

/// [`plan_value`] with `kept`, or `None` if the keeps can't be paid or the
/// new card `c` is not played in the plan.
fn plan_value_checked(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[Key],
    c: &str,
    f: &Forced,
) -> Option<f64> {
    let (v, planned) = plan(w, g, s, kept, f)?;
    planned.iter().any(|p| p == c).then_some(v)
}

fn plan_value(w: &World, g: &GameState, s: usize, kept: &[Key], f: &Forced) -> f64 {
    plan(w, g, s, kept, f).map_or(f64::NEG_INFINITY, |(v, _)| v)
}

/// The jobs `d` would do for seat `s` on this table: markers in reach to
/// clear, threats to delay, presence with no token to shield.
fn niche_jobs(w: &World, g: &GameState, s: usize, d: &CardDef) -> i64 {
    d.effects
        .iter()
        .map(|e| match e {
            Op::ClearMarkers { count } => {
                let open = |h: &usize| g.tokens(*h, common::MARKER) > 0 && common::in_reach(w, g, s, *h);
                (*count).min((0..g.hexes.len()).filter(open).count() as i64)
            }
            Op::DelayThreat { count } => {
                (*count).min(threats::active(g).iter().filter(|(t, _)| !threats::waits(g, t)).count() as i64)
            }
            Op::ShieldPresence { count } => {
                (*count).min((0..g.hexes.len()).filter(|&h| effects::shieldable(g, s, h)).count() as i64)
            }
            // 070: a typed counter with a threat up it can reach (threats are dealt before keeps).
            Op::Jam { amount, source: Some(t) } => {
                if threats::counterable(w, g, s, Some(*t)).is_empty() { 0 } else { (*amount + 1) / 2 }
            }
            _ => 0,
        })
        .sum::<i64>()
        + capstone_jobs(d)
}

/// A tier-2 capstone (070): a tier-2 building whose standing effect ends an
/// engine: income per tag, an automatic counter or project drain.
fn is_capstone(d: &CardDef) -> bool {
    d.tier == 2
        && d.kind == CardKind::Building
        && d.passives
            .iter()
            .any(|p| matches!(p, Op::IncomePerTag { .. } | Op::AutoCounter { .. } | Op::ProjectDrain { .. }))
}

/// 070: what a capstone's standing effect is worth to the keep plan, in
/// `plan_niche` jobs: its printed price undersells a card that pays every round.
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

/// The card's value: its printed cost plus 1 (a play is worth something),
/// plus the jobs it would do (046).
fn value(w: &World, g: &GameState, s: usize, d: &CardDef, f: &Forced) -> f64 {
    1.0 + list_cost(d) as f64 + f.plan_niche * niche_jobs(w, g, s, d) as f64
}

/// Whether a building that must go over one of the seat's can (024: the
/// plan kept them with nothing to go over, and they were never played).
fn replaceable(w: &World, g: &GameState, s: usize, d: &CardDef) -> bool {
    d.replaces.as_ref().is_none_or(|t| {
        g.pile(&cards::built(s)).iter().any(|id| cards::def(w, id).tags.contains(t))
    })
}

/// What seat `s` pays to play `d`: credits after its building discount.
fn spend(w: &World, g: &GameState, s: usize, d: &CardDef) -> i64 {
    credit_equivalent(d, cards::play_cost(w, g, s, d))
}

fn ratio(w: &World, g: &GameState, s: usize, id: &str, f: &Forced) -> f64 {
    let d = cards::def(w, id);
    value(w, g, s, d, f) / (1 + spend(w, g, s, d)) as f64
}

/// `ids`, best value per cost first.
fn by_ratio(w: &World, g: &GameState, s: usize, f: &Forced, ids: &mut [Key]) {
    ids.sort_by(|a, b| {
        ratio(w, g, s, b, f)
            .total_cmp(&ratio(w, g, s, a, f))
            .then_with(|| a.cmp(b))
    })
}

/// The plan with `kept` bought now: its value, and the cards it plays.
/// `None` if the keeps can't be paid.
fn plan(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[Key],
    f: &Forced,
) -> Option<(f64, Vec<Key>)> {
    let (rounds, reserve) = (f.plan_rounds, f.plan_reserve);
    let res = |r: Resource| g.resource(s, r).max(0);
    let income = economy::income(w, g, s);
    let inc = |r: Resource| income[r];
    let mut now = Budget {
        credits: res(Resource::Credits),
        insight: res(Resource::Insight),
        chips: res(Resource::Chips),
        energy: res(Resource::Energy),
        antimatter: res(Resource::Antimatter),
    };
    // Pay the keeps: insight first, the shortfall in credits.
    let rate = w.int("cards.insight_credits");
    let mut paid = 0;
    for (count, id) in (g.var(&cards::kept_key(s))..).zip(kept) {
        let (ins, cr) = cards::keep_need(w, g, s, cards::def(w, id).tier, count);
        let from_insight = ins.min(now.insight);
        now.insight -= from_insight;
        paid += cr + (ins - from_insight) * rate;
        now.credits -= cr + (ins - from_insight) * rate;
        if now.credits < 0 {
            return None;
        }
    }
    let mut pool: Vec<Key> = g.pile(&cards::hand(s)).to_vec();
    pool.extend(kept.iter().cloned());
    by_ratio(w, g, s, f, &mut pool);
    let mut planned: Vec<Key> = Vec::new();
    let mut total = 0.0;
    let mut b = now;
    let needs = cards::required_tags(w, g, s);
    for round in 0..rounds.max(1) {
        if round > 0 {
            // Credits, chips, energy and antimatter carry over; insight resets.
            b = Budget {
                credits: b.credits + inc(Resource::Credits),
                insight: (inc(Resource::Insight) - reserve).max(0),
                chips: b.chips + inc(Resource::Chips),
                energy: b.energy + inc(Resource::Energy),
                antimatter: b.antimatter + inc(Resource::Antimatter),
            };
        }
        for id in &pool {
            if planned.contains(id) {
                continue;
            }
            let d = cards::def(w, id);
            let credits = cards::play_cost(w, g, s, d);
            let fits = b.credits >= credits
                && b.insight >= cards::play_insight(w, g, s, d)
                && b.chips >= d.chips
                && b.energy >= d.energy
                && b.antimatter >= d.antimatter
                && cards::lowers_ok(w, g, s, d)
                && replaceable(w, g, s, d)
                && (d.is_building() || effects::feasible(w, g, s, &d.effects))
                && d.needs.iter().all(|(t, n)| needs[*t as usize] >= *n);
            if fits {
                b.credits -= credits;
                b.insight -= cards::play_insight(w, g, s, d);
                b.chips -= d.chips;
                b.energy -= d.energy;
                b.antimatter -= d.antimatter;
                let share = if d.tier == 2 { f.plan_t2 } else { 1.0 };
                total += share * value(w, g, s, d, f) * (1.0 - WAIT * round as f64).max(0.1);
                planned.push(*id);
            }
        }
    }
    // 024: credits paid to keep are credits not spent on the board (the
    // opening's 5-credit keeps took all 20 starting credits).
    Some((total - f.plan_credit * paid as f64, planned))
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

    #[test]
    fn the_presence_checks_rule_out_the_powers_they_name() {
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
        let no_presence = forced("no_presence");
        assert!(no_presence.ruled_out(&w, &g, &deploy) && no_presence.ruled_out(&w, &g, &buy));
        assert!(!no_presence.ruled_out(&w, &g, &scrap));
        let never_second = forced("never_second_presence");
        assert!(never_second.ruled_out(&w, &g, &buy) && !never_second.ruled_out(&w, &g, &deploy));
    }

    #[test]
    fn a_shield_counts_only_the_presence_it_can_cover() {
        let r = crate::agents::tests::rules(0);
        let mut g = r.new_game(1);
        let w = r.world();
        let shield = cards::def(&w, "benchmark_suite");
        let open = (0..g.hexes.len()).filter(|&h| effects::shieldable(&g, 0, h)).count() as i64;
        assert!(open > 0);
        assert_eq!(niche_jobs(&w, &g, 0, shield), open.min(2));
        for h in g.hexes.iter_mut() {
            Arc::make_mut(h).tokens.clear();
        }
        assert_eq!(niche_jobs(&w, &g, 0, shield), 0);
    }
}
