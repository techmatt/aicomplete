//! Playing a card, taking and keeping one, and using an ability.

use crate::key::KeyBuf;
use super::*;

impl Cards {
    /// The insight keeping `card` costs `seat` now: [`keep_need`]. r82: no credits buy a shortfall.
    pub(super) fn keep_price(&self, w: &World, g: &GameState, seat: usize, card: &str) -> i64 {
        keep_need(w, g, seat, def(w, card).tier)
    }
}

/// 189: why each card left in seat `s`'s hand when the action phase ends stayed there (stats only, for the
/// resource-use read): `limit.play.short.<resource>` per resource it lacked, `.needs_tags`, else `.held`.
pub fn tally_short(w: &World, g: &mut GameState, cx: &mut Cx, seat: usize) {
    let p = prices(w, g, seat);
    let mut keys: Vec<String> = Vec::new();
    for id in g.pile(&hand(seat)).iter() {
        let d = def(w, id);
        if !requirements_met(w, g, seat, d) {
            keys.push("limit.play.needs_tags".into());
            continue;
        }
        let mut short = false;
        let mut other_short = false;
        let mut insight_short = false;
        if g.resource(seat, Resource::Credits) < p.credits(d) {
            keys.push("limit.play.short.credits".into());
            short = true;
            other_short = true;
        }
        for (r, n) in extra_costs_at(d, p.insight(d)) {
            if n > 0 && g.resource(seat, r) < n {
                keys.push(format!("limit.play.short.{}", r.name()));
                short = true;
                if r == Resource::Insight {
                    insight_short = true;
                } else {
                    other_short = true;
                }
            }
        }
        // 196: a seat-round a card was held for want of insight: `insight_short` for any insight shortfall,
        // `insight_bound` short of insight alone (any count > 0 is one seat-round).
        if insight_short {
            keys.push(format!("@cards.insight_short.{seat}"));
            if !other_short {
                keys.push(format!("@cards.insight_bound.{seat}"));
            }
        }
        if !short {
            keys.push("limit.play.held".into());
            // 199: a kept card the seat could have played this round: a dead keep later is "held back".
            if g.stats.counters.get(keep_at(seat, id).as_str()).is_some_and(|&r0| r0 > 0) {
                keys.push(format!("={}", keep_ok(seat, id)));
            }
        }
    }
    for k in keys {
        if let Some(flag) = k.strip_prefix('=') {
            cx.level(g, flag, 1);
            continue;
        }
        match k.strip_prefix('@') {
            Some(round_key) => cx.per_round(g, round_key, 1),
            None => cx.tally(g, &k, 1),
        }
    }
}

/// 199 (stats only): kept `card` left seat `s`'s hand unplayed: a dead keep, counted now
/// (`cards.keep_plan.left_never` / `.left_held`, by whether it was ever playable), its keys cleared.
pub(super) fn keep_left(g: &mut GameState, cx: &mut Cx, s: usize, card: &str) {
    let ka = keep_at(s, card);
    if !g.stats.counters.get(ka.as_str()).is_some_and(|&r0| r0 > 0) {
        return;
    }
    let ok = keep_ok(s, card);
    let held = g.stats.counters.get(ok.as_str()).is_some_and(|&v| v > 0);
    cx.tally(g, if held { "cards.keep_plan.left_held" } else { "cards.keep_plan.left_never" }, 1);
    cx.level(g, &ka, 0);
    if held {
        cx.level(g, &ok, 0);
    }
}

/// 199 (stats only): marks each kept card in seat `seat`'s hand it could play now (its tags met, every cost in hand)
/// as `keep_ok`: called as the play phase begins, before the round's spending, beside `tally_short`'s end-of-phase mark.
pub fn mark_playable_keeps(w: &World, g: &mut GameState, cx: &mut Cx, seat: usize) {
    let p = prices(w, g, seat);
    let mut ok = Vec::new();
    for id in g.pile(&hand(seat)).iter() {
        if !g.stats.counters.get(keep_at(seat, id).as_str()).is_some_and(|&r0| r0 > 0) {
            continue;
        }
        let d = def(w, id);
        if requirements_met(w, g, seat, d)
            && g.resource(seat, Resource::Credits) >= p.credits(d)
            && extra_costs_at(d, p.insight(d)).iter().all(|&(r, n)| n <= 0 || g.resource(seat, r) >= n)
        {
            ok.push(keep_ok(seat, id));
        }
    }
    for k in ok {
        cx.level(g, &k, 1);
    }
}

/// 199 (stats only): the round seat `s` kept `card`, 0 once it is played; a key left above 0 at the end is a dead keep.
pub(super) fn keep_at(s: usize, card: &str) -> String {
    format!("cards.keep_at.{s}.{card}")
}

/// 199 (stats only): 1 once seat `s` could have played its kept `card` at the end of an action phase.
pub(super) fn keep_ok(s: usize, card: &str) -> String {
    format!("cards.keep_ok.{s}.{card}")
}

/// 199 (stats only): a drafted card seat `s` could keep but couldn't play by the next round: its tags unmet, or its
/// credits over what it holds and one round's credit income.
pub(super) fn keep_unaffordable(w: &World, g: &GameState, seat: usize, card: &str) -> bool {
    let d = def(w, card);
    if !requirements_met(w, g, seat, d) {
        return true;
    }
    let reach = g.resource(seat, Resource::Credits) + economy::income(w, g, seat)[Resource::Credits];
    prices(w, g, seat).credits(d) > reach
}

/// Whether seat `s` meets what `d` requires.
/// The seat that seat `s`'s draft packet passes to among `n` (178: always the next seat; it alternated by round before
/// r77). The agents' packet pass reads it too (`agents/eval.rs`).
pub fn pass_to(s: usize, n: usize) -> usize {
    (s + 1) % n
}

fn requirements_met(w: &World, g: &GameState, seat: usize, d: &CardDef) -> bool {
    if d.needs.is_empty() {
        return true;
    }
    let have = tag_counts(w, g, seat);
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
    if d.is_building() {
        return None;
    }
    let counted = effects::own_counted(d);
    if !effects::feasible(w, g, seat, &counted) {
        // 118: a gain per tag with too few tags read "no target" (Capacitor Discharge).
        // Only on a card that does nothing else: beside another op, that op vetoed it.
        let few = counted.iter().all(|e| matches!(e, Op::GainPerTag { tag, per, .. } if tags(w, g, seat, *tag) < *per));
        return Some(if few { "too few tags" } else { "no target" });
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
    let mut free = FreeTest::new(w, g, seat);
    g.pile(&hand(seat)).iter().filter(|id| free.test(id)).cloned().collect()
}

/// Whether one card of `free_buildings` exists: what the Rush Build power's
/// sacrifice asks of every source it could give up (099), so it stops at the
/// first.
pub fn free_building_exists(w: &World, g: &GameState, seat: usize) -> bool {
    let mut free = FreeTest::new(w, g, seat);
    g.pile(&hand(seat)).iter().any(|id| free.test(id))
}

/// [`free_buildings`]' test of one card, with the seat's tags and prices read
/// once a hand, not once a card (141: a Rush Build sacrifice read them for
/// every card of every source of every option, 70% of a long game's time).
struct FreeTest<'a, 'w> {
    w: &'a World<'w>,
    g: &'a GameState,
    seat: usize,
    tags: Option<[i64; TAGS.len()]>,
    prices: Option<Prices>,
}

impl<'a, 'w> FreeTest<'a, 'w> {
    fn new(w: &'a World<'w>, g: &'a GameState, seat: usize) -> Self {
        FreeTest { w, g, seat, tags: None, prices: None }
    }

    fn test(&mut self, id: &str) -> bool {
        let (w, g, seat) = (self.w, self.g, self.seat);
        let d = def(w, id);
        if !d.is_building() {
            return false;
        }
        if !d.needs.is_empty() {
            let have = self.tags.get_or_insert_with(|| tag_counts(w, g, seat));
            if !d.needs.iter().all(|(t, n)| have[*t as usize] >= *n) {
                return false;
            }
        }
        let p = self.prices.get_or_insert_with(|| prices(w, g, seat));
        blocked_beyond_credits(w, g, seat, d, p, false).is_none()
    }
}

/// Seat `s` plays a building from its hand, paying no credits (068).
pub fn play_free_building(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let ids = free_buildings(&w, g, seat);
    let opts = ids.iter().map(|id| opt(id.to_string())).collect();
    let prompt = || format!("Seat {seat}: play which building, paying no credits?");
    let Some(i) = cx.choose(seat, prompt, opts)? else {
        return Ok(());
    };
    let id = ids[i];
    let d = def(&w, &id);
    pay_card_extras(g, cx, seat, d);
    cx.tally(g, "cards.free_built", 1);
    // 169: a building with its own fight trigger fires it for its own play here too (Rush Build on Honeypot Relay);
    // only when the seat held none, else the power's own watch (`fighting`) already counts these removals.
    if fight_watch(&w, g, seat).is_none() && fighting_play_watches(&w, &id) {
        return fighting_play(g, cx, seat, &id, |g, cx| enter_play(g, cx, seat, &id, d));
    }
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
    // 076: an OR Action is usable while either branch is.
    let mut first = None;
    for b in ab.branches() {
        let why = branch_blocked(w, g, seat, id, b)?;
        first.get_or_insert(why);
    }
    first
}

/// Why one branch of an Action can't be used now.
pub(super) fn branch_blocked(w: &World, g: &GameState, seat: usize, id: &str, ab: &Ability) -> Option<&'static str> {
    // 135: a face-down building has no Action.
    if disabled(g, seat, id) {
        return Some("face down");
    }
    if g.var(&tokens_key(seat, id)) < ab.tokens || ab.cost.iter().any(|(r, n)| g.resource(seat, *r) < *n) {
        return Some("unaffordable");
    }
    if !effects::feasible_on(w, g, seat, &ab.effects, 0, Some(id)) {
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
    // 105: what the card discounts by tag saved, the measure's.
    let (credits, insight) = if holds(&w, g, seat, |p| matches!(p, Op::CardDiscount { .. })) {
        prices(&w, g, seat).tag_savings(d)
    } else {
        (0, 0)
    };
    if credits > 0 {
        cx.tally(g, "cards.discount_credits", credits);
    }
    if insight > 0 {
        cx.tally(g, "cards.discount_insight", insight);
    }
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
        let ins = self.keep_price(&w, g, s, card);
        if ins > 0 && !alignment::take_free_purchase(g, cx) {
            if g.resource(s, Resource::Insight) < ins {
                return Err("can't afford to keep it".into());
            }
            cx.add_resource(g, s, Resource::Insight, -ins);
            cx.tally(g, "cards.insight_kept", ins);
            cx.per_round(g, "cards.insight_kept", ins);
            cx.per_round(g, &format!("cards.seat_insight_kept.{s}"), ins);
        }
        cx.move_card(g, card, &drafted(s), &hand(s), false)?;
        cx.count(g, &format!("kept.{card}"), 1);
        cx.per_round(g, "cards.kept", 1);
        // 199: the keep plan (stats only): kept by tier and round, and the round it was kept, read when it plays
        // (`enter_play`) and, for a dead keep, at the game's end.
        let tier = def(&w, card).tier;
        cx.per_round(g, &format!("cards.kept_t{tier}"), 1);
        cx.level(g, &keep_at(s, card), g.round as i64);
        let ok = keep_ok(s, card);
        if g.stats.counters.get(ok.as_str()).is_some_and(|&v| v > 0) {
            cx.level(g, &ok, 0);
        }
        if g.pile(&drafted(s)).is_empty() {
            g.seats[s].passed = true;
        }
        Ok(())
    }
}

/// A paid card leaves the hand and resolves.
fn enter_play(g: &mut GameState, cx: &mut Cx, seat: usize, id: &str, d: &CardDef) -> Res {
    // 099: by its full key: Rush Build plays from the ai module, and `count`
    // filed its buildings under `ai.played.`, out of every read of plays.
    cx.tally(g, &KeyBuf::new("cards.played.").text(id), 1);
    cx.per_round(g, "cards.played", 1);
    // 199: the keep plan (stats only): plays by tier and round; a kept card's wait from keep to play, 3 rounds or
    // more a late keep (`cards.late_play.<id>`, valued by the reader). Its `keep_at` goes to 0: played.
    cx.per_round(g, &format!("cards.played_t{}", d.tier), 1);
    let ka = keep_at(seat, id);
    if let Some(&r0) = g.stats.counters.get(ka.as_str()).filter(|&&r0| r0 > 0) {
        let wait = g.round as i64 - r0;
        cx.tally(g, "cards.keep_plan.played", 1);
        cx.tally(g, "cards.keep_plan.wait", wait);
        if wait >= 3 {
            cx.tally(g, "cards.keep_plan.late", 1);
            cx.tally(g, &KeyBuf::new("cards.late_play.").text(id), 1);
        }
        cx.level(g, &ka, 0);
        let ok = keep_ok(seat, id);
        if g.stats.counters.get(ok.as_str()).is_some() {
            cx.level(g, &ok, 0);
        }
    }
    if d.is_building() {
        place_building(g, cx, seat, id, d)?;
        cx.move_card(g, id, &hand(seat), &built(seat), false)?;
    } else if d.is_event() {
        // 111: an event sits aside while it resolves, then goes to its
        // player's played events (r48), never the discard: a draw that
        // reshuffles can't bring it back (Breakthrough Leak drew itself, a
        // 2061-play loop, when events were discarded first).
        cx.move_card(g, id, &hand(seat), &resolving(seat), false)?;
    } else {
        cx.move_card(g, id, &hand(seat), &played(seat), false)?;
    }
    // 076: a building's play effects may load it ("add 2 Agents here").
    let mut ctx = if d.is_building() { effects::Ctx::ability(seat, Key::new(id)) } else { effects::Ctx::seat(seat) };
    effects::run(g, cx, &d.effects, &mut ctx)?;
    // 105: then every seat's play triggers (a building counts itself).
    if !g.is_over() {
        on_play(g, cx, seat, d)?;
    }
    // 118: a standing counter (Autonomous Defense Grid) counters at once what it
    // now pays in full.
    if d.passives.iter().any(|p| matches!(p, Op::AutoCounter { .. })) {
        threats::complete_paid(g, cx);
    }
    if d.is_event() {
        cx.move_card(g, id, &resolving(seat), &events(seat), false)?;
    }
    Ok(())
}

/// A card's costs beyond credits, and the income it lowers.
fn pay_card_extras(g: &mut GameState, cx: &mut Cx, seat: usize, d: &CardDef) {
    let w = cx.world;
    let [insight, energy, chips, antimatter, cubes] = extra_costs(&w, g, seat, d);
    for (r, n) in [antimatter, energy, chips, cubes, insight] {
        cx.add_resource(g, seat, r, -n);
    }
    if insight.1 > 0 {
        cx.tally(g, "cards.insight_played", insight.1);
        cx.per_round(g, "cards.insight_played", insight.1);
        cx.per_round(g, &format!("cards.seat_insight_played.{seat}"), insight.1);
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
    let prompt = || format!("Seat {seat}: build {} where?", def(&w, id).name);
    let k = cx.choose(seat, prompt, opts)?;
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
        for (p, cards) in packets.into_iter().enumerate() {
            cx.set_pile(g, &packet(pass_to(p, n)), cards);
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
    let root = def(&w, card).ability.as_ref().ok_or("no ability")?;
    // 076: an OR Action asks which branch, among those it can use now.
    let usable: Vec<&Ability> = root
        .branches()
        .into_iter()
        .filter(|b| branch_blocked(&w, g, s, card, b).is_none())
        .collect();
    let ab = match usable.as_slice() {
        [] => return Err(Halt::Illegal(format!("can't use {card}: no branch"))),
        [one] => *one,
        _ => {
            let opts = usable.iter().map(|b| opt(branch_label(b))).collect();
            let prompt = || format!("Seat {s}: {}, which half of the Action?", def(&w, card).name);
            usable[cx.choose(s, prompt, opts)?.unwrap_or(0)]
        }
    };
    for (r, n) in &ab.cost {
        cx.add_resource(g, s, *r, -n);
    }
    add_tokens(g, cx, s, card, -ab.tokens);
    cx.exhaust(g, s, card);
    cx.count(g, &format!("used.{card}"), 1);
    effects::run(g, cx, &ab.effects, &mut effects::Ctx::ability(s, *card))?;
    // r35: a one-shot branch takes its card out of play, with the stack under it.
    if ab.once {
        let mut stack: Vec<(String, Key)> =
            g.pile(&under(s, card)).iter().map(|c| (under(s, card).to_string(), *c)).collect();
        stack.push((built(s).to_string(), *card));
        for (pile, c) in &stack {
            cx.set_key(g, &tokens_key(s, c), 0);
            discard(g, cx, c, pile);
        }
        cx.tally(g, "cards.once_discarded", 1);
    }
    Ok(())
}

/// A branch's option label: its first effect, for records and the viewer.
fn branch_label(b: &Ability) -> String {
    match b.effects.first() {
        Some(Op::AddTokens { .. }) => "add tokens here".to_string(),
        Some(op) => format!("{op:?}").split_whitespace().next().unwrap_or("act").to_string(),
        None => "act".to_string(),
    }
}
