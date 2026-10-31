//! The heuristic value of a state, from the team's point of view. It reads
//! only what the table shows: pieces on the board, tracks, resources, open
//! hands and buildings, active threats and what is paid on them, deck
//! sizes and the face-up adversary card. Units are roughly "credits": a
//! credit in hand is worth `credits` (0.6) because credits buy cards and
//! standard actions that do more.
//!
//! The board (014): cities are clean, aligned (a player cube) or corrupted
//! (the adversary's cube; every city corrupted loses). Adversary ambassadors
//! gather on clean cities until `adversary.corrupt_at` corrupts them, divert
//! the production of the pieces they sit on (a factory builds a troop),
//! and come out of a supply that caps them.
//!
//! The weights are agent parameters, not rules: they live in [`Weights`]
//! and an experiment can override any of them (`greedy:w.corrupted=40`).
//!
//! **Horizon mode** (`w.horizon` > 0; the `engine` agent's default): income
//! and buildings are valued by what they will pay over the rounds the game
//! is expected to last ([`Eval::rounds_left`]: the adversary deck's clock,
//! cut short by how soon the team is on course to win and by the danger it
//! is in), not by a flat weight; a card in hand is valued by whether it can
//! be paid for before the game ends; and resources lose their value as the
//! end nears (nothing is left to buy).

use aic_core::content::{CardDef, Op, Resource, Side, RESOURCES};
use aic_core::modules::common::{
    self, ADV_AMBASSADOR, ADV_CUBE, FACTORY, HARDEN, STATION, TROOP,
};
use aic_core::modules::{adversary, ai, alignment, board, cards, economy, threats};
use aic_core::{GameState, Rules};
use serde::{Deserialize, Serialize};

use super::policy::list_cost;
use super::INSIGHT_COST;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    /// A finished game: won or lost.
    pub win: f64,
    /// Added per adversary card left (deck and face-up) on a win (a wider margin).
    pub win_margin: f64,
    /// Each corrupted city (it builds ambassadors, diverts its energy to
    /// Discord, and every city corrupted loses).
    pub corrupted: f64,
    /// A seat down to its last presence (027: a player with none loses);
    /// half that with two.
    pub presence_danger: f64,
    /// Each aligned city (a player cube: extra energy for its owner).
    pub aligned: f64,
    /// Each adversary ambassador on the board (the bar-0 win needs none).
    pub adv_ambassador: f64,
    /// Ambassadors on a city not yet corrupted: this share of `corrupted`
    /// times (ambassadors / the next corrupt threshold)², capped at 1.
    pub pressure: f64,
    /// Each adversary ambassador on a player piece, base or factory (it
    /// diverts the piece's production).
    pub on_piece: f64,
    /// Extra per factory holding any adversary ambassador, however many (it
    /// makes one troop a production step).
    pub on_factory: f64,
    /// Each adversary troop (it destroys a building every round).
    pub troop: f64,
    /// Per Discord card short of 3 left in the deck (027: the tip that finds
    /// it empty loses).
    pub discord_danger: f64,
    /// Each adversary piece that keeps the current clear-board bar from
    /// being met (the win condition).
    pub blocker: f64,
    /// Per Unity card short of 3 left in the deck (028: drawing the last wins).
    pub unity_deck: f64,
    /// Each Unity tip made; Sway in the Unity bucket counts as its share of
    /// the next tip (so a tip about to happen is worth nearly a tip).
    pub unity_tip: f64,
    /// Each Discord tip made (a cost); Sway in the Discord bucket costs its
    /// share of the next one.
    pub discord_tip: f64,
    /// Each level the clear-board bar has dropped (Unity tips).
    pub level: f64,
    /// Each visible adversary project's fill, as a share of its cost (a
    /// cost: it will resolve; 017).
    pub project: f64,
    /// Each player presence on the board (reach, site claims, a share of
    /// city energy; its track income is counted as income).
    /// Was `data_centre` (still accepted, see [`RENAMED`]).
    pub presence: f64,
    /// Each node on the board (a cost: adversary compute, contests presence,
    /// blocks the purge).
    pub node: f64,
    /// Each corruption marker (041: a hex lost to placement).
    pub marker: f64,
    /// A hardening token on a presence hex, on top of `harden` (041: it
    /// keeps a token that can't be replaced).
    pub shield: f64,
    /// A threat that waits this round counts this share (041): it comes back,
    /// but with a counter phase first.
    pub threat_waits: f64,
    /// Each player-owned factory (chips each production step); an
    /// adversary-owned one costs as much.
    pub owned_factory: f64,
    /// Each chip a seat holds.
    pub chips: f64,
    /// Each city, base or station hex inside some seat's reach.
    pub reach: f64,
    /// Each research station a player owns; a corrupted one costs as much.
    pub station: f64,
    /// Each player cube on a megaproject; an adversary cube costs half.
    pub project_cube: f64,
    pub factory: f64,
    /// Each hardening token.
    pub harden: f64,
    /// Each player ambassador in a seat's supply.
    pub ambassador: f64,
    /// Each player ambassador on the board (shields a building, cancels an
    /// arriving adversary ambassador, unites a base).
    pub ambassador_board: f64,
    /// Player ambassadors toward a corrupted city's hold (027), on it or in
    /// the supply of a seat with it in reach: this share of `corrupted`.
    pub freeing: f64,
    /// Each adversary ambassador next to a corrupted city (024: it blocks
    /// the freeing), on top of `adv_ambassador`.
    pub guard: f64,
    /// Each point of per-round income (AI tracks).
    pub income: f64,
    pub credits: f64,
    /// 024: credits a seat holds beyond `credit_cap` are worth `credit_idle`
    /// each: a stockpile buys nothing this round, so spending wins.
    pub credit_cap: f64,
    pub credit_idle: f64,
    /// A program in a seat's play area (021: its tags pay AI-board powers).
    pub program: f64,
    /// A token on one of a seat's cards (021).
    pub token: f64,
    /// Cards in hand (and drafted) that count, best first (021); 0: all.
    /// At 8 the 021 screen fell to 38% at d0 (agents hoarded their best
    /// cards and stopped playing), so it is off by default.
    pub hand_cap: f64,
    pub energy: f64,
    pub antimatter: f64,
    pub insight: f64,
    /// A card in hand: `hand + hand_cost * cost`, cost in credits (see
    /// [`super::credit_equivalent`]).
    pub hand: f64,
    pub hand_cost: f64,
    /// A drafted card not yet kept, as a share of its hand value.
    pub drafted: f64,
    /// 023: a card dealt into the draft packet counts as drafted, less its
    /// keep price (insight, at `INSIGHT_COST` credits) times this; so the
    /// tier pick is a choice (before 023 a tier pick scored nothing, and
    /// tier 2 was drafted only once the tier-1 deck ran dry).
    pub packet_keep: f64,
    /// 046: above 0, drafted and packet cards count only as far as this
    /// round's insight keeps them, best first. Priced one at a time, every
    /// tier-2 pick looked worth a keep and agents drafted tier 2 three picks
    /// in four, more than a seat could keep, and tier-1 Science went unseen.
    pub packet_budget: f64,
    /// A building in play: `built + built_cost * cost`.
    pub built: f64,
    pub built_cost: f64,
    /// Each active threat, scaled by the share still unpaid.
    pub threat: f64,
    /// Horizon mode, when above 0: each unit of income (AI tracks,
    /// buildings, rewards) is worth `horizon` × the resource's own weight ×
    /// the rounds left, replacing `income`. 0 = the flat weights above.
    pub horizon: f64,
    /// Horizon mode: Sway toward Unity the team is assumed to gain per round
    /// before it has a record of its own (round 1), for the win estimate.
    /// Was `opp_rate` (still accepted, see [`RENAMED`]).
    pub unity_rate_start: f64,
    /// Horizon mode: rounds a win needs after the Unity target is reached.
    pub win_lag: f64,
    /// Horizon mode: chance of losing per round per unit of danger (an
    /// unpaid threat per country, a last presence, the Discord deck at its edge);
    /// the rounds left are the expected survival over the clock.
    pub risk: f64,
    /// Horizon mode: per round left, a building that pays no income (an
    /// ability, a discount, protection), on top of `built`.
    pub ability: f64,
    /// Horizon mode: resources keep their full value only while at least
    /// this many rounds are left, then fall toward 0 (nothing left to buy).
    pub spend_horizon: f64,
    /// Horizon mode: board production (energy of aligned cities not
    /// diverted) counts this share of income over the rounds left, on top
    /// of the flat piece weights.
    pub production: f64,
    /// Horizon mode, when above 0: cards in hand wait their turn for
    /// insight (each needs the insight of the cards before it too).
    pub insight_wait: f64,
    /// The deck clock (011), any mode: per round the team's projected win
    /// (Unity still needed at its rate so far, plus `win_lag`) comes
    /// after the adversary deck runs out. 0 = off.
    pub clock: f64,
}

/// Weights renamed since experiments named them: (former name, name).
const RENAMED: [(&str, &str); 3] = [
    ("opp_rate", "unity_rate_start"),
    ("hardware", "chips"),
    ("data_centre", "presence"),
];

/// A drafted card's keep price in credits (its tier's insight at
/// `INSIGHT_COST`), for valuing the draft packet (023).
fn keep_insight(world: &aic_core::module::World, tier: u8) -> f64 {
    let key = if tier == 2 { "cards.keep_tier2" } else { "cards.keep_tier1" };
    match world.params.get(key) {
        Some(aic_core::config::ParamValue::Int(v)) => (INSIGHT_COST * v) as f64,
        _ => 0.0,
    }
}
/// Flat mode: a point of track income is worth this many of the resource's
/// own points for antimatter and insight (1 for the rest), times `income`.
const ANTIMATTER_INCOME_UNIT: f64 = 3.0;
const INSIGHT_INCOME_UNIT: f64 = 1.5;

impl Default for Weights {
    fn default() -> Self {
        Weights {
            win: 1000.0,
            win_margin: 5.0,
            // 014: chosen by hand for the new board, not yet swept.
            corrupted: 30.0,
            presence_danger: 60.0,
            aligned: 8.0,
            adv_ambassador: 3.0,
            pressure: 0.7,
            on_piece: 2.0,
            on_factory: 6.0,
            troop: 12.0,
            discord_danger: 40.0,
            unity_deck: 40.0,
            blocker: 2.0,
            // 013: a Unity tip replaces about 4 opposition points (24) and
            // brings a card; a Discord tip brings one of the adversary's.
            unity_tip: 24.0,
            discord_tip: 16.0,
            level: 10.0,
            // 017: chosen by hand, not swept.
            project: 30.0,
            presence: 4.0,
            // 022 (020's D9): a node is permanent since 019; at 6 agents made
            // nodes answering the spread when an equal free option existed.
            node: 25.0,
            marker: 1.0,
            shield: 3.0,
            threat_waits: 0.5,
            owned_factory: 4.0,
            chips: 2.0,
            reach: 0.5,
            station: 5.0,
            project_cube: 2.0,
            factory: 0.5,
            harden: 1.0,
            // 018: was 3, below the 6 credits (3.6) an ambassador costs, so no
            // agent ever bought one and corrupted cities were never freed.
            ambassador: 5.0,
            ambassador_board: 2.5,
            freeing: 2.0,
            guard: 3.0,
            income: 3.0,
            credits: 0.6,
            credit_cap: 12.0,
            credit_idle: 0.15,
            program: 1.0,
            token: 0.5,
            hand_cap: 0.0,
            energy: 0.8,
            antimatter: 2.0,
            insight: 0.5,
            hand: 3.0,
            hand_cost: 0.4,
            drafted: 0.3,
            packet_keep: 1.0,
            packet_budget: 1.0,
            built: 4.0,
            built_cost: 0.5,
            threat: 6.0,
            horizon: 0.0,
            unity_rate_start: 2.0,
            win_lag: 1.0,
            risk: 0.1,
            ability: 0.5,
            spend_horizon: 2.0,
            // Horizon mode only; the engine turns it on (see `engine`).
            production: 0.0,
            // 008: on, it cut the engine to 4% wins at d4 (96 games): spending
            // insight on one card lowered every other card's chance, so
            // playing anything looked costly. Off until it is modelled per round.
            insight_wait: 0.0,
            clock: 0.0,
        }
    }
}

impl Weights {
    /// The `engine` agent's defaults: horizon mode, with hand and building
    /// weights that no longer stand in for the future (008 sweep).
    pub fn engine() -> Weights {
        Weights {
            horizon: 1.0,
            hand: 1.0,
            hand_cost: 0.2,
            built: 1.0,
            built_cost: 0.0,
            // 011: see `clock`; chosen on 009's cheap-tier-2 patch, where
            // without it the engine ran out the deck.
            clock: 30.0,
            // 014 (d0, 2 countries, 48 seeds, the checkpoint's map): 1 won
            // 64.6%, 0 won ~37% (32 seeds); 2 won 66.7%. On the map as since
            // edited, level within noise at d1-d3 (32 seeds). Energy and
            // compute now come from the board.
            production: 1.0,
            ..Weights::default()
        }
    }

    /// Sets one weight by name (or a former name, see [`RENAMED`]); false
    /// if there is no such weight.
    pub fn set(&mut self, name: &str, v: f64) -> bool {
        let name = RENAMED
            .iter()
            .find(|(old, _)| *old == name)
            .map_or(name, |(_, new)| *new);
        let mut map = match serde_json::to_value(&*self) {
            Ok(serde_json::Value::Object(m)) => m,
            _ => return false,
        };
        if !map.contains_key(name) {
            return false;
        }
        map.insert(name.to_string(), serde_json::json!(v));
        match serde_json::from_value(serde_json::Value::Object(map)) {
            Ok(w) => *self = w,
            Err(_) => return false,
        }
        true
    }
}

/// A [`Weights`] bound to one game's rules, with the parameters it reads
/// cached. Cheap to clone (030): each of a planner's rollouts has its own.
#[derive(Clone)]
pub struct Eval {
    pub w: Weights,
    seats: usize,
    /// Unity tip count the team aims for: the deck's last card.
    target_tips: i64,
    corrupt_at: i64,
    city_energy: i64,
    aligned_bonus: i64,
    tip_per_player: i64,
}

impl Eval {
    pub fn new(rules: &Rules, w: Weights) -> Eval {
        let world = rules.world();
        let seats = rules.config.countries as usize;
        // 028: the Unity deck's last card wins, and it comes before the last bar drop.
        let target_tips = world.list("alignment.unity_cards").iter().sum();
        Eval {
            w,
            seats,
            target_tips,
            corrupt_at: world.int("adversary.corrupt_at"),
            city_energy: world.list("board.corp_energy").first().copied().unwrap_or(1),
            aligned_bonus: world.int("board.aligned_bonus"),
            tip_per_player: world.int("alignment.tip_per_player"),
        }
    }

    fn cost(d: &CardDef) -> f64 {
        list_cost(d).max(0) as f64
    }

    /// A card in hand, before any chance of paying for it. The packet's
    /// formula stays apart: its keep price sits inside the bracket, and moving
    /// it changes the last bits of a score and so the order of ties.
    fn card_value(w: &Weights, d: &CardDef) -> f64 {
        w.hand + w.hand_cost * Self::cost(d)
    }

    /// Adversary cards still to come: the face-up one and the deck.
    pub(crate) fn cards_left(g: &GameState) -> usize {
        g.pile(adversary::DECK).len() + g.pile(adversary::NEXT).len()
    }

    /// What a seat's credits are worth: `credits` each up to `credit_cap`,
    /// `credit_idle` each beyond it (024).
    fn credit_value(&self, n: f64) -> f64 {
        let w = &self.w;
        let cap = if w.credit_cap > 0.0 { w.credit_cap } else { f64::MAX };
        w.credits * n.min(cap) + w.credit_idle * (n - cap).max(0.0)
    }

    /// The next corrupt step's threshold, read off the face-up card (public).
    fn corrupt_threshold(&self, rules: &Rules, g: &GameState) -> i64 {
        let world = rules.world();
        let content = world.content;
        // Between this round's build and reveal the face-up slot is empty: until
        // this round's corrupt step has run, the corrupt step ahead is this
        // round's card's (020: it read nothing, so a Fifth Column round's spread
        // choices were judged at the normal threshold).
        let corrupt_ahead = g
            .config
            .round
            .iter()
            .position(|p| p == "adversary.corrupt")
            .is_some_and(|i| g.phase <= i);
        let card = match g.pile(adversary::NEXT).first() {
            None if corrupt_ahead => g.pile(adversary::CURRENT).first(),
            next => next,
        };
        let sooner: i64 = card
            .and_then(|id| content.adversary_index(id))
            .map_or(0, |i| {
                content.adversary[i]
                    .twists
                    .iter()
                    .filter_map(|t| match t {
                        Op::CorruptSooner { amount } => Some(*amount),
                        _ => None,
                    })
                    .sum()
            });
        (self.corrupt_at - sooner).max(1)
    }

    /// Each seat's reach as a bit per hex (a board has at most 4 × 16 hexes),
    /// worked out once a position: `in_reach` per hex was a tenth of the
    /// search's time (028).
    fn reach_masks(&self, rules: &Rules, g: &GameState) -> [u64; 4] {
        let world = rules.world();
        let mut near = [0u64; 4];
        for (h, hex) in world.map.hexes.iter().enumerate() {
            for (s, mask) in near.iter_mut().enumerate().take(g.seats.len()) {
                if g.tokens(h, common::presence(s)) > 0 {
                    *mask |= 1 << h;
                    for &n in &hex.neighbours {
                        *mask |= 1 << n;
                    }
                }
            }
        }
        near
    }

    /// The value of `g`. Finished games score `±win` plus a margin bonus.
    pub fn value(&self, rules: &Rules, g: &GameState) -> f64 {
        let w = &self.w;
        if let Some(o) = &g.outcome {
            return if o.won {
                w.win + w.win_margin * Self::cards_left(g) as f64
            } else {
                -w.win
            };
        }
        let world = rules.world();
        let corrupt_at = self.corrupt_threshold(rules, g);
        let mut v = 0.0;
        let mut corrupted = 0i64;
        let mut reach = 0i64;
        let near = self.reach_masks(rules, g);
        for h in 0..g.hexes.len() {
            let city = common::is_city(&world, h);
            let adv = g.tokens(h, ADV_AMBASSADOR);
            let bad = common::corrupted(g, h);
            if city {
                if bad {
                    corrupted += 1;
                    let on = board::ambassadors_on(g, h) as f64;
                    let held: i64 = (0..g.seats.len())
                        .filter(|&s| near[s] >> h & 1 != 0)
                        .map(|s| g.resource(s, Resource::Ambassadors))
                        .sum();
                    let need = board::hold(&world, g, h).max(1) as f64;
                    // Gathering counts a little; landing, once enough are at
                    // hand to meet the hold, counts more each step, so a push
                    // climbs to the free one action at a time (027: agents
                    // bought ambassadors and never moved them).
                    let at_hand = on + held as f64;
                    let mut share = 0.25 * (at_hand / need).min(1.0);
                    if at_hand >= need {
                        share += 0.75 * (on / need).powi(2);
                    }
                    v += w.corrupted * w.freeing * share;
                    v += (w.ambassador - w.ambassador_board) * on;
                    let around = world.map.hexes[h].neighbours.iter();
                    let guards: i64 = around.map(|&n| g.tokens(n, ADV_AMBASSADOR)).sum();
                    v -= w.guard * guards as f64;
                } else {
                    if common::cube_owner(g, h).is_some() {
                        v += w.aligned;
                    }
                    // 034: a corruption marker counts toward corrupting.
                    let toward = adv + g.tokens(h, common::MARKER);
                    let p = (toward as f64 / corrupt_at as f64).min(1.0);
                    v -= w.corrupted * w.pressure * p * p;
                }
            }
            v -= w.adv_ambassador * adv as f64;
            if adv > 0 {
                let piece = common::player_owned(g, h)
                    || common::is_base(&world, h)
                    || g.tokens(h, FACTORY) > 0
                    || g.tokens(h, STATION) > 0;
                if piece {
                    v -= w.on_piece * adv as f64;
                }
                if g.tokens(h, FACTORY) > 0 {
                    v -= w.on_factory;
                }
            }
            v -= w.troop * g.tokens(h, TROOP) as f64;
            if g.tokens(h, STATION) > 0 {
                if bad {
                    v -= w.station;
                } else if common::cube_owner(g, h).is_some() {
                    v += w.station;
                }
            }
            v += w.factory * g.tokens(h, FACTORY) as f64 + w.harden * g.tokens(h, HARDEN) as f64;
            if g.tokens(h, FACTORY) > 0 {
                if bad {
                    v -= w.owned_factory;
                } else if common::cube_owner(g, h).is_some() {
                    v += w.owned_factory;
                }
            }
            v -= w.node * g.tokens(h, common::NODE) as f64;
            v -= w.marker * g.tokens(h, common::MARKER) as f64;
            let held = common::presence_owner(g, h).is_some();
            if held && g.tokens(h, HARDEN) > 0 {
                v += w.shield;
            }
            if held {
                v += w.presence;
            }
            for s in 0..self.seats {
                v += w.ambassador_board * g.tokens(h, &common::ambassador(s)) as f64;
            }
            let counts = city || common::is_base(&world, h) || g.tokens(h, STATION) > 0;
            if counts && (0..self.seats).any(|s| near[s] >> h & 1 != 0) {
                reach += 1;
            }
        }
        v -= w.corrupted * corrupted as f64;
        let mut thin = i64::MAX;
        for s in 0..self.seats {
            let count = common::presence_count(g, s);
            thin = thin.min(count);
            match count {
                ..=1 => v -= w.presence_danger,
                2 => v -= 0.5 * w.presence_danger,
                // 034: a token lost stays lost and a hit takes 2, so the danger
                // starts sooner.
                3 => v -= 0.25 * w.presence_danger,
                4 => v -= 0.1 * w.presence_danger,
                _ => {}
            }
        }
        if self.seats == 0 {
            thin = 0;
        }
        v += w.reach * reach as f64;
        // The Discord deck: the tip that finds it empty loses.
        let discord_left = alignment::cards_left(g, Side::Discord) as i64;
        v -= w.discord_danger * (3 - discord_left).max(0) as f64;
        v += w.unity_deck * (3 - alignment::cards_left(g, Side::Unity) as i64).max(0) as f64;
        // Megaprojects: each player cube pays when the project completes.
        for c in 0..world.map.countries.len() {
            if g.var(&board::project_done(c)) > 0 {
                continue;
            }
            let mine: i64 = (0..self.seats)
                .map(|s| g.var(&board::project_key(c, Some(s))))
                .sum();
            let theirs = g.var(&board::project_key(c, None));
            v += w.project_cube * (mine as f64 - 0.5 * theirs as f64);
        }
        for s in 0..self.seats {
            v += w.ambassador * g.resource(s, Resource::Ambassadors) as f64;
            v += w.chips * g.resource(s, Resource::Chips) as f64;
        }
        // The win bar: what still stands between the team and a clear board.
        let level = alignment::bar_level(&world, g);
        v -= w.blocker * adversary::purge_count(&world, g, level) as f64;
        // Alignment: tips made, and each bucket's share of its next tip.
        for (side, weight) in [(Side::Unity, w.unity_tip), (Side::Discord, -w.discord_tip)] {
            let cap = alignment::capacity(&world, g, side).max(1) as f64;
            let fill = (g.var(alignment::bucket(side)) as f64 / cap).min(1.0);
            v += weight * (g.var(alignment::tips_key(side)) as f64 + fill);
        }
        v += w.level * level as f64;
        // A resolved (or ongoing) project counts in full, so letting one
        // fill never scores better than holding it back.
        for id in adversary::projects(g) {
            let cost = adversary::project_cost(&world, g, &id).max(1) as f64;
            v -= w.project * g.var(&adversary::project_key(&id)) as f64 / cost;
        }
        let resolved = g.stats.counters.get("adversary.projects_resolved");
        v -= w.project * resolved.copied().unwrap_or(0) as f64;
        // Added in place, term by term, so the flat agents' values (and so
        // their games) do not depend on horizon mode.
        let unpaid = self.unpaid_threats(rules, g);
        if w.horizon > 0.0 {
            self.horizon_value(rules, g, &unpaid, thin, discord_left, &mut v);
        } else {
            self.flat_value(rules, g, &mut v);
        }
        for share in unpaid {
            v -= w.threat * share;
        }
        v -= self.clock_penalty(g);
        v
    }

    /// Each active threat's share still unpaid.
    fn unpaid_threats(&self, rules: &Rules, g: &GameState) -> Vec<f64> {
        let world = rules.world();
        let w = &self.w;
        let mut out = Vec::new();
        for (t, _) in threats::active(g) {
            let req: i64 = threats::requirement(&world, g, &t)
                .iter()
                .map(|(_, n)| n)
                .sum();
            if req <= 0 {
                continue;
            }
            let prefix = threats::paid_prefix(&t);
            let paid: i64 = g
                .vars
                .from_key(prefix.as_str())
                .take_while(|(k, _)| k.starts_with(prefix.as_str()))
                .map(|(_, &n)| n)
                .sum();
            let waits = if threats::waits(g, &t) { w.threat_waits } else { 1.0 };
            out.push(waits * (1.0 - (paid as f64 / req as f64).min(1.0)));
        }
        out
    }

    /// The rounds of income still to come, as the table reads: the adversary
    /// deck's clock (each card left is one more round), cut short by when
    /// the team is on course to win (Unity still needed at its rate so far,
    /// plus `win_lag`), then the expected rounds survived at a per-round
    /// loss chance of `risk` per unit of danger (unpaid threats per
    /// country, a seat's last presence, the Discord deck at its edge).
    fn rounds_left(
        &self,
        rules: &Rules,
        g: &GameState,
        unpaid: &[f64],
        thin: i64,
        discord_left: i64,
    ) -> f64 {
        let w = &self.w;
        let clock = Self::cards_left(g) as f64;
        let (points, bar) = self.unity_progress(g);
        let to_win = ((bar - points).max(0.0) / self.unity_rate(g)).ceil() + w.win_lag;
        let rounds = clock.min(to_win).max(0.0);
        let world = rules.world();
        let countries = world.map.countries.len().max(1) as f64;
        let mut danger = unpaid.iter().sum::<f64>() / countries;
        if discord_left <= 1 {
            danger += 0.5;
        }
        if thin <= 1 {
            danger += 0.5;
        }
        let p = (1.0 - w.risk * danger).clamp(0.0, 1.0);
        // Sum of p^k for k = 1..=rounds, with a fractional last round.
        let mut total = 0.0;
        let mut surv = 1.0;
        let mut k = 0.0;
        while k < rounds {
            surv *= p;
            total += surv * (rounds - k).min(1.0);
            k += 1.0;
        }
        total
    }

    /// Unity so far, in Sway (each tip counts a full bucket at the base
    /// size), and the Sway the target (the outright Unity win) needs.
    fn unity_progress(&self, g: &GameState) -> (f64, f64) {
        let per = (self.tip_per_player * g.seats.len() as i64) as f64;
        let tips = g.var(alignment::tips_key(Side::Unity)) as f64;
        let points = tips * per + g.var(alignment::bucket(Side::Unity)) as f64;
        (points, self.target_tips as f64 * per)
    }

    /// Unity gained per round so far, in Sway (`unity_rate_start` before the
    /// team has a record of its own). Sway held may include this round's, so
    /// this round counts as played.
    fn unity_rate(&self, g: &GameState) -> f64 {
        let w = &self.w;
        let points = self.unity_progress(g).0;
        if g.round > 1 {
            (points / g.round as f64).max(w.unity_rate_start * 0.25)
        } else {
            w.unity_rate_start
        }
        .max(0.1)
    }

    /// The deck-clock penalty: `clock` per round the team, at its Unity
    /// rate so far, would still be short of its Unity target when the deck
    /// runs out. With `k` adversary cards left (the face-up one included),
    /// `k` action phases come after this one (the build step after the last
    /// loses); this round's points count as they are scored, not as a round
    /// still to come, so in the last round every point short costs
    /// `clock` / rate.
    fn clock_penalty(&self, g: &GameState) -> f64 {
        if self.w.clock == 0.0 {
            return 0.0;
        }
        let clock = Self::cards_left(g) as f64;
        let (points, top) = self.unity_progress(g);
        let need = (top - points).max(0.0) / self.unity_rate(g);
        self.w.clock * (need - clock).max(0.0)
    }

    /// Per-seat resources, income, hands and buildings, by flat weights.
    fn flat_value(&self, rules: &Rules, g: &GameState, v: &mut f64) {
        let w = &self.w;
        let world = rules.world();
        for s in 0..self.seats {
            let res = |k: Resource| g.resource(s, k) as f64;
            *v += self.credit_value(res(Resource::Credits))
                + w.energy * res(Resource::Energy)
                + w.antimatter * res(Resource::Antimatter)
                + w.insight * res(Resource::Insight);
            for (r, n) in ai::track_income(&world, g, s) {
                let unit = match r {
                    Resource::Antimatter => ANTIMATTER_INCOME_UNIT,
                    Resource::Insight => INSIGHT_INCOME_UNIT,
                    _ => 1.0,
                };
                *v += w.income * unit * n as f64;
            }
            let mut cards: Vec<f64> = g
                .pile(&cards::hand(s))
                .iter()
                .map(|id| Self::card_value(w, cards::def(&world, id)))
                .collect();
            let drafted = g.pile(&cards::drafted(s)).iter().map(|id| {
                let d = cards::def(&world, id);
                (w.drafted * Self::card_value(w, d), d.tier)
            });
            let packet = g.pile(&cards::packet(s)).iter().map(|id| {
                let d = cards::def(&world, id);
                let keep = keep_insight(&world, d.tier);
                let v = w.hand + w.hand_cost * (Self::cost(d) - w.packet_keep * keep);
                (w.drafted * v.max(0.0), d.tier)
            });
            let picks: Vec<(f64, u8)> = drafted.chain(packet).collect();
            self.keepable(&world, g, s, picks, &mut cards);
            *v += Self::capped(&mut cards, w.hand_cap);
            for id in g.pile(&cards::built(s)) {
                *v += w.built + w.built_cost * Self::cost(cards::def(&world, id));
            }
            self.in_play_value(rules, g, s, v);
        }
    }

    /// Adds to `cards` the value of each of seat `s`'s picks (value, tier) that
    /// its insight this round keeps, best first (046; `packet_budget` 0: all).
    fn keepable(
        &self,
        world: &aic_core::module::World,
        g: &GameState,
        s: usize,
        mut picks: Vec<(f64, u8)>,
        cards: &mut Vec<f64>,
    ) {
        if self.w.packet_budget <= 0.0 {
            cards.extend(picks.iter().map(|p| p.0));
            return;
        }
        picks.sort_by(|a, b| b.0.total_cmp(&a.0));
        let mut left = g.resource(s, Resource::Insight);
        let mut kept = 0;
        for (v, tier) in picks {
            let (need, _) = cards::keep_need(world, g, s, tier, kept);
            if need <= left {
                left -= need;
                kept += 1;
                cards.push(v);
            }
        }
    }

    /// The best `cap` of a seat's card values (021: hands outgrew what a
    /// seat can play, so cards beyond a playable hand are worth nothing and
    /// agents stop keeping every card they draft). `cap` 0: no cap.
    fn capped(cards: &mut [f64], cap: f64) -> f64 {
        if cap <= 0.0 {
            return cards.iter().sum();
        }
        cards.sort_by(|a, b| b.total_cmp(a));
        cards.iter().take(cap as usize).sum()
    }

    /// 021: programs in the play area (their tags pay powers) and tokens on
    /// cards, at flat weights.
    fn in_play_value(&self, rules: &Rules, g: &GameState, s: usize, v: &mut f64) {
        let world = rules.world();
        *v += self.w.program * g.pile(&cards::played(s)).len() as f64;
        for id in cards::token_cards(&world, g, s, None) {
            *v += self.w.token * g.var(&cards::tokens_key(s, &id)) as f64;
        }
    }

    /// What the board makes each seat a round if production came now: the
    /// energy of its aligned cities, less what adversary ambassadors divert.
    fn board_production(&self, rules: &Rules, g: &GameState) -> Vec<i64> {
        let world = rules.world();
        let mut out = vec![0; self.seats];
        for h in 0..g.hexes.len() {
            if g.tokens(h, ADV_AMBASSADOR) > 0 || g.tokens(h, ADV_CUBE) > 0 {
                continue;
            }
            if common::is_city(&world, h) {
                if let Some(s) = common::cube_owner(g, h).filter(|&s| s < self.seats) {
                    out[s] += self.city_energy + self.aligned_bonus;
                }
            }
        }
        out
    }

    /// Per-seat resources, income, hands and buildings in horizon mode.
    fn horizon_value(
        &self,
        rules: &Rules,
        g: &GameState,
        unpaid: &[f64],
        thin: i64,
        discord_left: i64,
        out: &mut f64,
    ) {
        let w = &self.w;
        let world = rules.world();
        let left = self.rounds_left(rules, g, unpaid, thin, discord_left);
        let prod = if w.production != 0.0 {
            self.board_production(rules, g)
        } else {
            vec![0; self.seats]
        };
        // Resources fade once fewer than `spend_horizon` rounds are left.
        let fade = if w.spend_horizon > 0.0 {
            ((left + 1.0) / w.spend_horizon).min(1.0)
        } else {
            1.0
        };
        // 021: chips sell 1:1 for credits (what a seat holds counts in `chips`).
        let unit = [w.credits, w.credits, w.energy, w.antimatter, w.insight];
        let mut v = 0.0;
        for s in 0..self.seats {
            let res = |k: Resource| g.resource(s, k);
            // Insight is reset each round, so what is held now only counts this round.
            for (i, r) in RESOURCES.iter().enumerate() {
                if *r == Resource::Credits {
                    v += fade * self.credit_value(res(*r) as f64);
                } else if *r != Resource::Chips {
                    v += fade * unit[i] * res(*r) as f64;
                }
            }
            self.in_play_value(rules, g, s, &mut v);
            // Income (tracks, buildings with income passives, rewards) over
            // the rounds left; board production counts `production` of its own.
            let income = economy::income(&world, g, s);
            for (i, (_, n)) in income.iter().enumerate() {
                v += w.horizon * left * unit[i] * n as f64;
            }
            v += w.horizon * w.production * left * w.energy * prod[s] as f64;
            for id in g.pile(&cards::built(s)) {
                let d = cards::def(&world, id);
                v += w.built + w.built_cost * Self::cost(d);
                if !d.pays_income() {
                    v += w.ability * left;
                }
            }
            // A card in hand is worth its value times the chance of paying
            // for it before the game ends, from what the seat has and earns.
            // Insight is spent one card at a time: each card waits for the
            // insight the cards before it in hand take.
            let earn_credits = income[Resource::Credits];
            let mut insight_queue = 0i64;
            // The same for every card in hand: worked out once.
            let prices = cards::prices(&world, g, s);
            let mut play = |id: &str| -> f64 {
                let d = cards::def(&world, id);
                let mut p = 1.0;
                // 022: playing costs insight again (shared with keeps).
                insight_queue += prices.insight(d);
                let insight_need = if w.insight_wait > 0.0 {
                    insight_queue
                } else {
                    0
                };
                for (have, need, earn) in [
                    // The AI's discount: energy or compute spent as credits.
                    (
                        res(Resource::Credits)
                            + cards::discount_room_at(&world, g, s, d, &prices)
                                .map_or(0, |(_, rate, k)| rate * k),
                        prices.credits(d),
                        earn_credits,
                    ),
                    (
                        res(Resource::Antimatter),
                        d.antimatter,
                        income[Resource::Antimatter],
                    ),
                    (res(Resource::Insight), insight_need, income[Resource::Insight]),
                ] {
                    let short = need - have;
                    if short > 0 {
                        if earn <= 0 {
                            return 0.0;
                        }
                        let rounds = (short as f64 / earn as f64).ceil();
                        p = f64::min(p, (1.0 - rounds / (left + 1.0)).max(0.0));
                    }
                }
                p * Self::card_value(w, d)
            };
            let mut cards: Vec<f64> = g.pile(&cards::hand(s)).iter().map(|id| play(id)).collect();
            let mut picks = Vec::new();
            for id in g.pile(&cards::drafted(s)) {
                picks.push((w.drafted * play(id), cards::def(&world, id).tier));
            }
            for id in g.pile(&cards::packet(s)) {
                let tier = cards::def(&world, id).tier;
                let keep = keep_insight(&world, tier);
                picks.push((w.drafted * (play(id) - w.hand_cost * w.packet_keep * keep).max(0.0), tier));
            }
            self.keepable(&world, g, s, picks, &mut cards);
            v += Self::capped(&mut cards, w.hand_cap);
        }
        *out += v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_count_only_as_far_as_insight_keeps_them() {
        let r = crate::agents::tests::rules(0);
        let w = r.world();
        let mut g = r.new_game(1);
        g.round = 3;
        let keep = |tier| cards::keep_need(&w, &g, 0, tier, 0).0;
        let (one, two) = (keep(1), keep(2));
        g.seats[0].resources.insert(Resource::Insight, two + one);
        let picks = vec![(1.0, 2), (3.0, 2), (2.0, 1)];
        let run = |budget: f64| {
            let ev = Eval::new(&r, Weights { packet_budget: budget, ..Weights::default() });
            let mut cards = Vec::new();
            ev.keepable(&w, &g, 0, picks.clone(), &mut cards);
            cards
        };
        assert_eq!(run(1.0), [3.0, 2.0], "the second tier-2 card is past the budget");
        assert_eq!(run(0.0), [1.0, 3.0, 2.0]);
    }

    #[test]
    fn renamed_weight_keeps_its_old_name() {
        let mut w = Weights::default();
        assert!(w.set("opp_rate", 1.5));
        assert_eq!(w.unity_rate_start, 1.5);
        assert!(w.set("unity_rate_start", 0.5));
        assert_eq!(w.unity_rate_start, 0.5);
        assert!(w.set("data_centre", 2.5));
        assert_eq!(w.presence, 2.5);
    }
}
