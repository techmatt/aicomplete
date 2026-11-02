//! What a card costs a seat: tag discounts and floors.

use super::*;

/// The rule for keeping one more card of `tier` when `seat` has already kept
/// `kept` this round, before any conversion: (insight needed, credits). The
/// opening (round 1): free for the first `opening_free`, then credits. Later
/// rounds: insight by tier. Paying a shortfall of insight in credits is the
/// caller's: it depends on the insight the seat has left at that point.
///
/// Variant `off` has no draft and no keep parameters: keeping is free there.
pub fn keep_need(w: &World, g: &GameState, seat: usize, tier: u8, kept: i64) -> (i64, i64) {
    if w.variant("cards") == Some("off") {
        return (0, 0);
    }
    if g.round <= 1 {
        if kept < w.int("cards.opening_free") {
            return (0, 0);
        }
        return (0, w.int("cards.opening_keep_credits"));
    }
    let need = match tier {
        2 if ai::keep2(w, g, seat) => science_off(w, g, seat, w.int("cards.keep_tier2_upgraded")),
        2 => science_off(w, g, seat, w.int("cards.keep_tier2")),
        _ => w.int("cards.keep_tier1"),
    };
    (need, 0)
}

/// A card's printed credit cost.
pub fn list_cost(w: &World, id: &str) -> i64 {
    def(w, id).cost
}

pub(super) const SCIENCE_EVERY: &str = "cards.science_every";

pub(super) const INDUSTRY_EVERY: &str = "cards.industry_every";

/// One off per `every` tags on a seat's cards, of which it has `count` (0: off).
pub(super) fn per_tags(w: &World, count: i64, every: &str) -> i64 {
    if w.variant("cards") != Some("draft") {
        return 0;
    }
    match w.int(every) {
        0 => 0,
        n => count / n,
    }
}

/// An insight cost `need` after seat `s`'s Science tags (034): never below
/// `cards.discount_floor`, and a cost under the floor stays as printed.
pub(super) fn science_off(w: &World, g: &GameState, seat: usize, need: i64) -> i64 {
    floored(need, per_tags(w, tags(w, g, seat, Tag::Science), SCIENCE_EVERY), discount_floor(w))
}

/// The lowest a discount takes a cost that is at least this: only the draft
/// variant has a floor.
pub(super) fn discount_floor(w: &World) -> i64 {
    if w.variant("cards") == Some("draft") {
        w.int("cards.discount_floor")
    } else {
        0
    }
}

/// `cost` less `off`, never below `floor`: a cost under the floor stays as printed.
pub(super) fn floored(cost: i64, off: i64, floor: i64) -> i64 {
    (cost - off).max(cost.min(floor))
}

/// The insight seat `s` pays to play `d`.
pub fn play_insight(w: &World, g: &GameState, seat: usize, d: &CardDef) -> i64 {
    science_off(w, g, seat, d.insight)
}

/// What seat `s` takes off card costs now, worked out once: a caller pricing
/// a whole hand gets the same numbers as [`play_cost`] and [`play_insight`]
/// without counting the seat's tags again for each card (042: that was most of
/// what the heuristic cost). Good until the seat's cards or vars change.
#[derive(Clone, Copy)]
pub struct Prices {
    insight_off: i64,
    building_off: i64,
    floor: i64,
}

pub fn prices(w: &World, g: &GameState, seat: usize) -> Prices {
    prices_with(w, g, seat, &tag_counts(w, g, seat))
}

/// [`prices`], with the seat's [`tag_counts`] already counted.
pub fn prices_with(w: &World, g: &GameState, seat: usize, counts: &[i64; TAGS.len()]) -> Prices {
    Prices {
        insight_off: per_tags(w, counts[Tag::Science as usize], SCIENCE_EVERY),
        building_off: passive_discount(w, g, seat)
            + per_tags(w, counts[Tag::Industry as usize], INDUSTRY_EVERY),
        floor: discount_floor(w),
    }
}

impl Prices {
    /// [`play_insight`].
    pub fn insight(&self, d: &CardDef) -> i64 {
        floored(d.insight, self.insight_off, self.floor)
    }

    /// [`play_cost`].
    pub fn credits(&self, d: &CardDef) -> i64 {
        if d.is_building() {
            floored(d.cost, self.building_off, self.floor)
        } else {
            d.cost
        }
    }
}

/// A card's costs beyond credits, in the order its text prints them, with
/// its insight cost for the seat already known.
pub(super) fn extra_costs_at(d: &CardDef, insight: i64) -> [(Resource, i64); 4] {
    [
        (Resource::Insight, insight),
        (Resource::Energy, d.energy),
        (Resource::Chips, d.chips),
        (Resource::Antimatter, d.antimatter),
    ]
}

/// A card's costs beyond credits, for seat `s`.
pub(super) fn extra_costs(w: &World, g: &GameState, seat: usize, d: &CardDef) -> [(Resource, i64); 4] {
    extra_costs_at(d, play_insight(w, g, seat, d))
}

/// Credits to play a card: buildings get the seat's discounts.
pub fn play_cost(w: &World, g: &GameState, seat: usize, d: &CardDef) -> i64 {
    if !d.is_building() {
        return d.cost;
    }
    prices(w, g, seat).credits(d)
}

/// The credits seat `s`'s buildings cost less now before tags: its passives
/// and a one-off discount.
pub(super) fn passive_discount(w: &World, g: &GameState, seat: usize) -> i64 {
    let mut off = 0;
    each_passive(w, g, seat, |p| {
        if let Op::BuildingDiscount { amount } = p {
            off += *amount;
        }
    });
    off
}
