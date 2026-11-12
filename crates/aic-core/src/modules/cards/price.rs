//! What a card costs a seat: tag discounts and floors.

use super::*;

/// The insight keeping one more drafted card of `tier` costs `seat`: by tier in every round, the opening's
/// draft included (r82: the opening hand is dealt free, and no credits buy a shortfall), less the keep upgrade
/// and the Science discount.
///
/// Variant `off` has no draft and no keep parameters: keeping is free there.
pub fn keep_need(w: &World, g: &GameState, seat: usize, tier: u8) -> i64 {
    if w.variant("cards") == Some("off") {
        return 0;
    }
    match tier {
        2 if ai::keep2(w, g, seat) => science_off(w, g, seat, w.int("cards.keep_tier2_upgraded")),
        2 => science_off(w, g, seat, w.int("cards.keep_tier2")),
        _ => w.int("cards.keep_tier1"),
    }
}

/// A card's printed credit cost.
pub fn list_cost(w: &World, id: &str) -> i64 {
    def(w, id).cost
}

const SCIENCE_EVERY: &str = "cards.science_every";

const INDUSTRY_EVERY: &str = "cards.industry_every";

/// One off per `every` tags on a seat's cards, of which it has `count` (0: off).
fn per_tags(w: &World, count: i64, every: &str) -> i64 {
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
fn science_off(w: &World, g: &GameState, seat: usize, need: i64) -> i64 {
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
fn floored(cost: i64, off: i64, floor: i64) -> i64 {
    (cost - off).max(cost.min(floor))
}

/// The insight seat `s` pays to play `d`.
pub fn play_insight(w: &World, g: &GameState, seat: usize, d: &CardDef) -> i64 {
    if !has_card_discount(w, g, seat) {
        return science_off(w, g, seat, d.insight);
    }
    prices(w, g, seat).insight(d)
}

/// What seat `s` takes off card costs now, worked out once: a caller pricing
/// a whole hand gets the same numbers as [`play_cost`] and [`play_insight`]
/// without counting the seat's tags again for each card (042: that was most of
/// what the heuristic cost). Good until the seat's cards or vars change.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Prices {
    insight_off: i64,
    building_off: i64,
    floor: i64,
    /// 105: card discounts by tag (`Tag as usize`), the last for every card:
    /// credits, then insight.
    tag_credits: [i64; TAGS.len() + 1],
    tag_insight: [i64; TAGS.len() + 1],
}

/// What a card's tags take off: each kind once, however many it carries, and
/// the every-card discount.
fn tag_off(off: &[i64; TAGS.len() + 1], d: &CardDef) -> i64 {
    let mut seen = [false; TAGS.len()];
    let mut n = off[TAGS.len()];
    for t in &d.tags {
        let i = *t as usize;
        if !seen[i] {
            seen[i] = true;
            n += off[i];
        }
    }
    n
}

pub fn prices(w: &World, g: &GameState, seat: usize) -> Prices {
    prices_with(w, g, seat, &tag_counts(w, g, seat))
}

/// [`prices`], with the seat's [`tag_counts`] already counted.
pub fn prices_with(w: &World, g: &GameState, seat: usize, counts: &[i64; TAGS.len()]) -> Prices {
    // One walk of the passives for both kinds of discount (105: the agents price
    // every hand with this).
    let (mut tag_credits, mut tag_insight) = ([0; TAGS.len() + 1], [0; TAGS.len() + 1]);
    let mut building = 0;
    each_passive(w, g, seat, |p| match p {
        Op::BuildingDiscount { amount } => building += *amount,
        Op::CardDiscount { tag, resource, amount } => {
            let i = tag.map_or(TAGS.len(), |t| t as usize);
            match resource {
                Resource::Insight => tag_insight[i] += *amount,
                _ => tag_credits[i] += *amount,
            }
        }
        _ => {}
    });
    Prices {
        insight_off: per_tags(w, counts[Tag::Science as usize], SCIENCE_EVERY),
        building_off: building + per_tags(w, counts[Tag::Industry as usize], INDUSTRY_EVERY),
        floor: discount_floor(w),
        tag_credits,
        tag_insight,
    }
}

impl Prices {
    /// [`play_insight`].
    pub fn insight(&self, d: &CardDef) -> i64 {
        floored(d.insight, self.insight_off + tag_off(&self.tag_insight, d), self.floor)
    }

    /// What the card discounts by tag (105) take off `d` after the floor:
    /// (credits, insight), for the measure.
    pub fn tag_savings(&self, d: &CardDef) -> (i64, i64) {
        let building = if d.is_building() { self.building_off } else { 0 };
        let credits = floored(d.cost, building, self.floor) - self.credits(d);
        let insight = floored(d.insight, self.insight_off, self.floor) - self.insight(d);
        (credits, insight)
    }

    /// [`play_cost`]: buildings get the building discounts, every card its
    /// tags' (105).
    pub fn credits(&self, d: &CardDef) -> i64 {
        let building = if d.is_building() { self.building_off } else { 0 };
        floored(d.cost, building + tag_off(&self.tag_credits, d), self.floor)
    }
}

/// A card's costs beyond credits, in the order the view lists them, with its
/// insight cost for the seat already known.
pub(super) fn extra_costs_at(d: &CardDef, insight: i64) -> [(Resource, i64); 5] {
    [
        (Resource::Insight, insight),
        (Resource::Energy, d.energy),
        (Resource::Chips, d.chips),
        (Resource::Antimatter, d.antimatter),
        (Resource::ResearchCubes, d.research_cubes),
    ]
}

/// A card's costs beyond credits, for seat `s`.
pub(super) fn extra_costs(w: &World, g: &GameState, seat: usize, d: &CardDef) -> [(Resource, i64); 5] {
    extra_costs_at(d, play_insight(w, g, seat, d))
}

/// Credits to play a card: the seat's discounts (buildings', and its tags').
pub fn play_cost(w: &World, g: &GameState, seat: usize, d: &CardDef) -> i64 {
    if !d.is_building() && !has_card_discount(w, g, seat) {
        return d.cost;
    }
    prices(w, g, seat).credits(d)
}

/// Whether seat `s` holds a card discount by tag (105): without one, a program's
/// price is its printed cost and an insight cost needs only the Science count.
fn has_card_discount(w: &World, g: &GameState, seat: usize) -> bool {
    holds(w, g, seat, |p| matches!(p, Op::CardDiscount { .. }))
}
