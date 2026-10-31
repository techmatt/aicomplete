//! Keeping by plan, and the forced behaviours of the dominance checks (022).
//!
//! [`Policy`] wraps a greedy, engine or planner agent. At keep time it sketches the
//! seat's next rounds ([`plan_keeps`]) and keeps a drafted card only if the
//! card lands in that plan and the plan is worth more with it than without
//! it (the keep's insight would otherwise pay for plays this round). The
//! dominance checks ([`Forced`]) are fixed policies, mostly the engine with
//! one behaviour forced: they must lose to thoughtful play
//! (`docs/principles.md`, *Dominance checks*).
//!
//! The plan is simple and deterministic, and reads only the public state
//! and the seat's own hand: projected credit, insight,
//! chips and energy income; the cards it would play (hand plus the
//! candidates) in value-per-cost order, filled into each round's budget.

use aic_core::content::{CardDef, Op, Resource, Tag};
use aic_core::modules::{ai, cards, common, economy, effects, threats};
use aic_core::module::World;
use aic_core::{fnv1a64, Action, Agent, GameState, Key, Rules};

use super::credit_equivalent;

/// A play's value falls by this share for each round it waits.
const WAIT: f64 = 0.15;

/// The behaviour a dominance check forces (none for thoughtful play).
#[derive(Debug, Clone)]
pub struct Forced {
    /// Keep every drafted card it can afford.
    pub keep_all: bool,
    /// Exploit check (034): spend every spare credit on the rich action.
    pub rich_spend: bool,
    /// Exploit check (034): draft, keep and play Science first, for its discounts.
    pub science_rush: bool,
    /// Fixed policy (041): the growth option that places the most presence.
    pub always_expand: bool,
    /// Fixed policy (041): the growth option that places none.
    pub always_consolidate: bool,
    /// Fixed policy (041): never clear a corruption marker.
    pub never_remove: bool,
    /// Fixed policy (041): the Science rush, board cards first.
    pub science_board: bool,
    /// Keep only its `keep_k` cheapest drafted cards (0: off).
    pub keep_k: usize,
    /// Sanity check (028): keep no drafted card.
    pub keep_none: bool,
    /// Sanity check (028): keep each drafted card on a coin toss.
    pub keep_random: bool,
    /// Never draft from tier 2.
    pub tier1_only: bool,
    /// Draft from tier 2 whenever it can afford the keep.
    pub tier2_rush: bool,
    /// Never use an AI-board power.
    pub no_powers: bool,
    /// Never pay toward a threat's counter.
    pub no_counters: bool,
    /// Place no presence beyond setup (no standard action, card or growth
    /// that places one).
    pub no_presence: bool,
    /// Mono-tag (023): keep and play only cards carrying this tag.
    pub only_tag: Option<Tag>,
    /// Ignore-X (023): never keep or play a card carrying this tag.
    pub never_tag: Option<Tag>,
    /// Event-hoarder (023): keep and play only events.
    pub events_only: bool,
    /// Exploit check (027): no counter paid and no card that counters a threat.
    pub ignore_threats: bool,
    /// Exploit check (027): stay on the setup hex and defend it: no presence,
    /// only the remove, clear, ambassador, hardening and chips standard actions.
    pub turtle: bool,
    /// Exploit check (027): only free cities: Diplomacy cards, the ambassador,
    /// move, remove, clear and chips standard actions, no counters, no presence,
    /// no powers.
    pub free_only: bool,
    /// Exploit check (028): farm the Unity deck. Every seat buys ambassadors
    /// and lands them on bases and factories, each worth Unity when it produces,
    /// before anything else; the rest of its play is the agent's.
    pub unity_farm: bool,
    /// Keep by plan (off: the wrapped agent's heuristic decides keeps).
    pub plan_keep: bool,
    /// Rounds the keep plan looks at: this round's action phase and the
    /// ones after it.
    pub plan_rounds: usize,
    /// Insight the keep plan holds back in each later round for that round's
    /// own keeps (024: the plan spent all of it on plays, so it thought it
    /// could play twice what it could and kept double what it played).
    pub plan_reserve: i64,
    /// The keep plan keeps nothing while the hand holds this many cards
    /// (024; 0: off). Hands of 8-10 held cards never played: at a table a
    /// player stops paying for cards they can't use.
    pub plan_hand: usize,
    /// Share of a tier-2 card's value the keep plan counts (024): its printed
    /// cost overstates what the one-step play logic gets from it, and a keep
    /// costs 2-3 insight.
    pub plan_t2: f64,
    /// What a credit paid for a keep costs the plan (024).
    pub plan_credit: f64,
    /// What the keep plan adds, in credits, for each marker a card would clear
    /// threat it would delay and presence it would shield on the table as it
    /// stands (046: priced at its printed cost, a 2-credit Science card lost
    /// every keep to a tier-2).
    pub plan_niche: f64,
}

impl Forced {
    /// Whether anything is forced: keeping by plan or a dominance check. The
    /// plan's numbers do nothing on their own. Destructured whole so that a
    /// new field has to be placed here.
    pub fn is_active(&self) -> bool {
        let Forced {
            keep_all,
            keep_k,
            rich_spend,
            science_rush,
            always_expand,
            always_consolidate,
            never_remove,
            science_board,
            keep_none,
            keep_random,
            tier1_only,
            tier2_rush,
            no_powers,
            no_counters,
            no_presence,
            only_tag,
            never_tag,
            events_only,
            ignore_threats,
            turtle,
            free_only,
            unity_farm,
            plan_keep,
            plan_rounds: _,
            plan_reserve: _,
            plan_hand: _,
            plan_t2: _,
            plan_credit: _,
            plan_niche: _,
        } = self;
        *keep_all
            || *keep_k > 0
            || *rich_spend
            || *science_rush
            || *always_expand
            || *always_consolidate
            || *never_remove
            || *science_board
            || *keep_none
            || *keep_random
            || *tier1_only
            || *tier2_rush
            || *no_powers
            || *no_counters
            || *no_presence
            || only_tag.is_some()
            || never_tag.is_some()
            || *events_only
            || *ignore_threats
            || *turtle
            || *free_only
            || *unity_farm
            || *plan_keep
    }

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
        } else if self.forced.science_rush || self.forced.science_board {
            let world = rules.world();
            let mut set = plan_keeps(&world, g, s, &self.forced);
            for id in g.pile(&cards::drafted(s)) {
                if cards::def(&world, id).tags.contains(&Tag::Science) && !set.contains(id) {
                    set.push(id.clone());
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

    /// Whether `a` is an action the forced behaviour rules out.
    fn ruled_out(&self, world: &World, a: &Action) -> bool {
        let f = &self.forced;
        match a {
            Action::Tier { tier: 2, .. } => f.tier1_only,
            Action::Power { .. } => f.no_powers || f.free_only,
            Action::Pay { .. } => f.no_counters || f.ignore_threats || f.free_only,
            Action::Standard { act, .. } => {
                let presence = act == "presence" || act == "presence_buy";
                (f.no_presence && presence)
                    || (f.never_remove && act == "clear")
                    || (f.turtle && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "harden" | "harden_buy" | "chips"))
                    || (f.free_only && !matches!(act.as_str(), "remove" | "clear" | "ambassador" | "move" | "chips"))
            }
            Action::Play { card, .. } => {
                let d = cards::def(world, card);
                (f.stays_put() && places_presence(&d.effects)) || !f.allows(d)
            }
            Action::Keep { card, .. } => !f.allows(cards::def(world, card)),
            Action::Growth { seat, option } => {
                let options = &ai::board(world, *seat as usize).growth;
                let placed = presence_placed(&options[*option as usize].effects);
                (f.stays_put() && placed > 0)
                    || (f.always_consolidate && placed > 0)
                    || (f.always_expand && placed < options.iter().map(|o| presence_placed(&o.effects)).max().unwrap_or(0))
            }
            _ => false,
        }
    }
}

fn places_presence(effects: &[Op]) -> bool {
    presence_placed(effects) > 0
}

/// The presence `effects` place.
fn presence_placed(effects: &[Op]) -> i64 {
    effects
        .iter()
        .map(|e| if let Op::PlacePresence { count, .. } = e { *count } else { 0 })
        .sum()
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
        if self.forced.rich_spend && !pending {
            if let Some(i) = legal.iter().position(|a| matches!(a, Action::Standard { act, .. } if act == "presence_buy")) {
                return i;
            }
        }
        if self.forced.science_rush && !pending {
            let science = |id: &Key| cards::def(&world, id).tags.contains(&Tag::Science);
            if let Some(i) = legal.iter().position(|a| {
                matches!(a, Action::Draft { card, .. } | Action::Keep { card, .. } | Action::Play { card, .. } if science(card))
            }) {
                return i;
            }
        }
        if self.forced.science_board && !pending {
            let science = |id: &Key, board: bool| {
                let d = cards::def(&world, id);
                d.tags.contains(&Tag::Science) && (!board || acts_on_board(d))
            };
            for board in [true, false] {
                if let Some(i) = legal.iter().position(|a| {
                    matches!(a, Action::Draft { card, .. } | Action::Keep { card, .. } | Action::Play { card, .. } if science(card, board))
                }) {
                    return i;
                }
            }
        }
        // Tier-2 rush: tier 2 whenever the keep is affordable.
        if self.forced.tier2_rush && !pending {
            if let Some(i) = legal.iter().position(|a| {
                matches!(a, Action::Tier { seat, tier: 2 } if keep_affordable(&world, g, *seat as usize, 2))
            }) {
                return i;
            }
        }
        let keep: Vec<usize> = (0..legal.len())
            .filter(|&i| !self.ruled_out(&world, &legal[i]))
            .collect();
        if keep.is_empty() || keep.len() == legal.len() {
            return self.inner.choose(rules, g, legal);
        }
        let sub: Vec<Action> = keep.iter().map(|&i| legal[i].clone()).collect();
        keep[self.inner.choose(rules, g, &sub)]
    }
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
        let hex = |t: &aic_core::hex::Hex| (0..g.hexes.len()).find(|&h| w.map.hexes[h].hex == *t);
        if p.ask.prompt.contains("move which ambassador") {
            return legal.iter().position(|a| matches!(a, Action::Choose { option, .. } if option == "from your supply"));
        }
        return p.ask.prompt.contains(": move on which hex").then(|| {
            legal.iter().position(|a| {
                matches!(a, Action::Choose { target: Some(t), .. }
                    if hex(t).is_some_and(|h| farm_site(w, g, h)))
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
    let order = |ids: &mut Vec<Key>| {
        ids.sort_by(|a, b| {
            ratio(w, g, s, b, f)
                .total_cmp(&ratio(w, g, s, a, f))
                .then_with(|| a.cmp(b))
        })
    };
    order(&mut drafted);
    let mut kept: Vec<Key> = Vec::new();
    let mut best = plan_value(w, g, s, &kept, f);
    for c in drafted {
        let mut with = kept.clone();
        with.push(c.clone());
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
    let mut count = g.var(&cards::kept_key(s));
    let mut paid = 0;
    for id in kept {
        let (ins, cr) = cards::keep_need(w, g, s, cards::def(w, id).tier, count);
        count += 1;
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
    pool.sort_by(|a, b| {
        ratio(w, g, s, b, f)
            .total_cmp(&ratio(w, g, s, a, f))
            .then_with(|| a.cmp(b))
    });
    let mut planned: Vec<Key> = Vec::new();
    let mut total = 0.0;
    let mut b = now;
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
                && d.needs.iter().all(|(t, n)| cards::tags(w, g, s, *t) >= *n);
            if fits {
                b.credits -= credits;
                b.insight -= cards::play_insight(w, g, s, d);
                b.chips -= d.chips;
                b.energy -= d.energy;
                b.antimatter -= d.antimatter;
                let share = if d.tier == 2 { f.plan_t2 } else { 1.0 };
                total += share * value(w, g, s, d, f) * (1.0 - WAIT * round as f64).max(0.1);
                planned.push(id.clone());
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
