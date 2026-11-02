//! Playing a card, taking and keeping one, and using an ability.

use super::*;

impl Cards {
    /// What keeping `card` costs `seat` now: (insight, credits). [`keep_need`],
    /// with any shortfall of insight at `insight_credits` credits an insight
    /// (that conversion is only for keeps).
    pub(super) fn keep_price(&self, w: &World, g: &GameState, seat: usize, card: &str) -> (i64, i64) {
        let kept = g.var(&kept_key(seat));
        let (need, credits) = keep_need(w, g, seat, def(w, card).tier, kept);
        let have = g.resource(seat, Resource::Insight).max(0);
        let ins = need.min(have);
        (ins, credits + (need - ins) * self.insight_credits)
    }
}

/// Whether seat `s` meets what `d` requires.
fn requirements_met(w: &World, g: &GameState, seat: usize, d: &CardDef) -> bool {
    if d.needs.is_empty() {
        return true;
    }
    let have = required_tags(w, g, seat);
    d.needs.iter().all(|(t, n)| have[*t as usize] >= *n)
}

/// Why a card can't be played now, or `None` if it can.
pub(super) fn blocked(w: &World, g: &GameState, seat: usize, id: &str) -> Option<&'static str> {
    let d = def(w, id);
    if !requirements_met(w, g, seat, d) {
        return Some("needs tags");
    }
    let p = prices(w, g, seat);
    let short = g.resource(seat, Resource::Credits) < p.credits(d);
    if let Some(why) = blocked_beyond_credits(w, g, seat, d, &p, short) {
        return Some(why);
    }
    if !d.is_building() && !effects::feasible(w, g, seat, &d.effects) {
        return Some("no target");
    }
    None
}

/// Why `d`, its requirements met, can't be paid for and placed: `short` says
/// the seat lacks the credits, the rest is every cost but credits.
fn blocked_beyond_credits(w: &World, g: &GameState, seat: usize, d: &CardDef, p: &Prices, short: bool) -> Option<&'static str> {
    if short
        || extra_costs_at(d, p.insight(d)).iter().any(|&(r, n)| g.resource(seat, r) < n)
        || !lowers_ok(w, g, seat, d)
    {
        return Some("unaffordable");
    }
    if d.replaces.is_some() && stack_targets(w, g, seat, d).is_empty() {
        return Some("nothing to replace");
    }
    None
}

/// Buildings in seat `s`'s hand it could play paying no credits (068): every
/// other cost and requirement stands.
fn free_buildings(w: &World, g: &GameState, seat: usize) -> Vec<Key> {
    let p = prices(w, g, seat);
    g.pile(&hand(seat))
        .iter()
        .filter(|id| {
            let d = def(w, id);
            d.is_building()
                && requirements_met(w, g, seat, d)
                && blocked_beyond_credits(w, g, seat, d, &p, false).is_none()
        })
        .cloned()
        .collect()
}

pub fn free_building_exists(w: &World, g: &GameState, seat: usize) -> bool {
    !free_buildings(w, g, seat).is_empty()
}

/// Seat `s` plays a building from its hand, paying no credits (068).
pub fn play_free_building(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let ids = free_buildings(&w, g, seat);
    let opts = ids.iter().map(|id| opt(id.to_string())).collect();
    let prompt = format!("Seat {seat}: play which building, paying no credits?");
    let Some(i) = cx.choose(seat, &prompt, opts)? else {
        return Ok(());
    };
    let id = ids[i];
    let d = def(&w, &id);
    pay_card_extras(g, cx, seat, d);
    cx.tally(g, "cards.free_built", 1);
    enter_play(g, cx, seat, &id, d)
}

/// Whether seat `s` has the income card `d` lowers (022): each `lowers`
/// resource's income at least the amount.
pub fn lowers_ok(w: &World, g: &GameState, seat: usize, d: &CardDef) -> bool {
    if d.lowers.is_empty() {
        return true;
    }
    let income = economy::income(w, g, seat);
    d.lowers
        .iter()
        .all(|(r, n)| income[*r] >= *n)
}

/// Why an ability can't be used now, or `None` if it can.
pub(super) fn ability_blocked(w: &World, g: &GameState, seat: usize, id: &str) -> Option<&'static str> {
    let d = def(w, id);
    let ab = d.ability.as_ref()?;
    if g.seats[seat].exhausted.iter().any(|c| c == id) {
        return Some("used");
    }
    if g.var(&tokens_key(seat, id)) < ab.tokens || ab.cost.iter().any(|(r, n)| g.resource(seat, *r) < *n) {
        return Some("unaffordable");
    }
    if !effects::feasible(w, g, seat, &ab.effects) {
        return Some("no target");
    }
    None
}

/// Seat `s` plays `id` from its hand at its price.
pub(super) fn play_card(g: &mut GameState, cx: &mut Cx, seat: usize, id: &str) -> Res {
    let w = cx.world;
    // An answer replays this from a table that passed it.
    if !cx.replaying() {
        if !g.pile(&hand(seat)).iter().any(|c| c == id) {
            return Err(Halt::Illegal(format!("{id} is not in seat {seat}'s hand")));
        }
        if let Some(why) = blocked(&w, g, seat, id) {
            return Err(Halt::Illegal(format!("can't play {id}: {why}")));
        }
    }
    let d = def(&w, id);
    cx.add_resource(g, seat, Resource::Credits, -play_cost(&w, g, seat, d));
    pay_card_extras(g, cx, seat, d);
    enter_play(g, cx, seat, id, d)
}

impl Cards {
    /// Card by card: the chosen tier's top card joins the hand.
    pub(super) fn take_tier(&self, g: &mut GameState, cx: &mut Cx, s: usize, tier: u8) -> Res {
        if !(1..=2).contains(&tier) || !tier_open(g, tier) {
            return Err("no such tier to draft from".into());
        }
        if !deal_top(g, cx, tier, &packet(s))? {
            return Err("no card to deal".into());
        }
        cx.tally(g, &format!("cards.dealt_tier{tier}"), 1);
        let full = g.pile(&packet(s)).len() as i64 >= self.deal;
        if full {
            g.seats[s].passed = true;
        }
        if !(tier_open(g, 1) || tier_open(g, 2)) {
            // The decks ran dry: every seat's hand is what it has.
            pass_all(g);
        }
        Ok(())
    }

    pub(super) fn keep_card(&self, g: &mut GameState, cx: &mut Cx, s: usize, card: &Key) -> Res {
        let w = cx.world;
        if !g.pile(&drafted(s)).contains(card) {
            return Err(Halt::Illegal(format!("{card} was not drafted by seat {s}")));
        }
        // A Unity card's free keep first, then the price.
        let (ins, cr) = self.keep_price(&w, g, s, card);
        if (ins > 0 || cr > 0) && !alignment::take_free_purchase(g, cx) {
            if g.resource(s, Resource::Credits) < cr {
                return Err("can't afford to keep it".into());
            }
            cx.add_resource(g, s, Resource::Insight, -ins);
            cx.add_resource(g, s, Resource::Credits, -cr);
            if ins > 0 {
                cx.tally(g, "cards.insight_kept", ins);
                cx.per_round(g, "cards.insight_kept", ins);
            }
            if g.round > 1 && cr > 0 {
                cx.tally(g, "cards.insight_bought", cr / self.insight_credits);
            }
        }
        cx.add_key(g, &kept_key(s), 1);
        cx.move_card(g, card, &drafted(s), &hand(s), false)?;
        cx.count(g, &format!("kept.{card}"), 1);
        cx.per_round(g, "cards.kept", 1);
        if g.pile(&drafted(s)).is_empty() {
            g.seats[s].passed = true;
        }
        Ok(())
    }
}

/// A paid card leaves the hand and resolves.
fn enter_play(g: &mut GameState, cx: &mut Cx, seat: usize, id: &str, d: &CardDef) -> Res {
    cx.count(g, &format!("played.{id}"), 1);
    cx.per_round(g, "cards.played", 1);
    if d.is_building() {
        place_building(g, cx, seat, id, d)?;
        cx.move_card(g, id, &hand(seat), &built(seat), false)?;
    } else if d.is_event() {
        discard(g, cx, id, &hand(seat));
    } else {
        cx.move_card(g, id, &hand(seat), &played(seat), false)?;
    }
    effects::run(g, cx, &d.effects, &mut effects::Ctx::seat(seat))
}

/// A card's costs beyond credits, and the income it lowers.
fn pay_card_extras(g: &mut GameState, cx: &mut Cx, seat: usize, d: &CardDef) {
    let w = cx.world;
    let [insight, energy, chips, antimatter] = extra_costs(&w, g, seat, d);
    for (r, n) in [antimatter, energy, chips, insight] {
        cx.add_resource(g, seat, r, -n);
    }
    if insight.1 > 0 {
        cx.tally(g, "cards.insight_played", insight.1);
        cx.per_round(g, "cards.insight_played", insight.1);
    }
    for (r, n) in &d.lowers {
        cx.add_key(g, &economy::production_key(seat, *r), -n);
    }
    if d.chips > 0 {
        cx.tally(g, "board.chips_spent", d.chips);
    }
}

/// Where a building goes: a new slot, or over one of yours.
fn place_building(g: &mut GameState, cx: &mut Cx, seat: usize, id: &str, d: &CardDef) -> Res {
    let w = cx.world;
    // A new slot while one is free (not for a replacement), or
    // over one of your buildings: it splays down.
    let targets = stack_targets(&w, g, seat, d);
    let free = d.replaces.is_none()
        && (slotted(g, seat).len() as i64) < capacity(&w, g, seat);
    let mut opts = Vec::new();
    if free {
        opts.push(opt("a new slot".to_string()));
    }
    opts.extend(targets.iter().map(|t| opt(format!("over {t}"))));
    let prompt = format!("Seat {seat}: build {id} where?");
    let k = cx.choose(seat, &prompt, opts)?;
    let over = match k {
        Some(0) if free => None,
        Some(k) => targets.get(k - free as usize).cloned(),
        None => None,
    };
    if let Some(old) = over {
        // The old stack goes under the new card, the old top last.
        for c in g.pile(&under(seat, &old)).to_vec() {
            cx.move_card(g, &c, &under(seat, &old), &under(seat, id), false)?;
        }
        cx.move_card(g, &old, &built(seat), &under(seat, id), false)?;
        cx.tally(g, "cards.replaced", 1);
        cx.per_round(g, "cards.replaced", 1);
    }
    Ok(())
}

pub(super) fn draft_pick(g: &mut GameState, cx: &mut Cx, s: usize, card: &Key) -> Res {
    if g.var(&picked_key(s)) > 0 {
        return Err("already took a card from this hand".into());
    }
    cx.move_card(g, card, &packet(s), &drafted(s), false)?;
    cx.set_key(g, &picked_key(s), 1);
    if all_picked(g) {
        let n = g.seats.len();
        let packets: Vec<Vec<Key>> =
            (0..n).map(|p| g.pile(&packet(p)).to_vec()).collect();
        // As in Terraforming Mars, the direction alternates by round.
        let step = if g.round % 2 == 1 { 1 } else { n - 1 };
        for (p, cards) in packets.into_iter().enumerate() {
            cx.set_pile(g, &packet((p + step) % n), cards);
        }
        for p in 0..n {
            cx.set_key(g, &picked_key(p), 0);
        }
        if (0..n).all(|p| g.pile(&packet(p)).is_empty()) {
            pass_all(g);
        }
    }
    Ok(())
}

pub(super) fn use_ability(g: &mut GameState, cx: &mut Cx, s: usize, card: &Key) -> Res {
    let w = cx.world;
    if !cx.replaying() {
        if !g.pile(&built(s)).contains(card) {
            return Err(Halt::Illegal(format!("seat {s} has no {card} in play")));
        }
        if let Some(why) = ability_blocked(&w, g, s, card) {
            return Err(Halt::Illegal(format!("can't use {card}: {why}")));
        }
    }
    let ab = def(&w, card).ability.as_ref().ok_or("no ability")?;
    for (r, n) in &ab.cost {
        cx.add_resource(g, s, *r, -n);
    }
    add_tokens(g, cx, s, card, -ab.tokens);
    cx.exhaust(g, s, card);
    cx.count(g, &format!("used.{card}"), 1);
    effects::run(g, cx, &ab.effects, &mut effects::Ctx::seat(s))?;
    Ok(())
}
