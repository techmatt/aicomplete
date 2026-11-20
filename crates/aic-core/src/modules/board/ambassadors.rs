//! Player ambassadors, holding and claiming sites, and freeing cities.

use crate::module::Hot;
use super::*;

/// Whether seat `s` can claim the site on hex `h` (align a city, take a
/// station or factory): it is in reach and no embargo is up on it (its
/// country's, or 135's on the hexes near a threat). r75 (Matt's ruling 2): a
/// botnet next to it no longer blocks a claim.
pub fn can_claim(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    in_reach(w, g, seat, h) && !hex_flagged(w, g, RoundFlag::Embargo, h)
}

pub fn ambassadors_on_board(g: &GameState, seat: usize) -> i64 {
    let k = ambassador(seat);
    (0..g.hexes.len()).map(|h| g.tokens(h, k)).sum()
}

/// Seat `s`'s player ambassador arrives on hex `h`: it cancels an
/// agent there (both removed, no Unity); otherwise it stands on
/// the piece (137: claiming and liberating are standard actions, so an
/// arriving ambassador claims nothing).
pub fn place_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    if g.tokens(h, ADV_AMBASSADOR) > 0 {
        adversary::remove(g, cx, h, 1);
        cx.tally(g, "board.canceled", 1);
        return;
    }
    cx.add_tokens(g, h, ambassador(seat), 1);
}

/// r75 (Matt's ruling 1): an agent arriving on hex `h` cancels one player ambassador standing there, 1 for 1;
/// with several players' ambassadors there the players choose whose (seat 0 asks for the team; it was the
/// lowest seat's). Returns the seat whose ambassador went, if one stood there; the caller handles the agent.
pub fn agent_cancels(g: &mut GameState, cx: &mut Cx, h: usize) -> Res<Option<usize>> {
    let w = cx.world;
    let seats: Vec<usize> = (0..g.seats.len()).filter(|&s| g.tokens(h, ambassador(s)) > 0).collect();
    let i = match seats.len() {
        0 => return Ok(None),
        1 => 0,
        _ => {
            let opts = seats.iter().map(|&s| seat_opt(s)).collect();
            let prompt = || format!("An agent arrives at {}: whose ambassador does it cancel?", hex_name(&w, h));
            cx.choose_loss(0, prompt, opts)?.unwrap_or(0)
        }
    };
    let s = seats[i];
    cx.add_tokens(g, h, ambassador(s), -1);
    Ok(Some(s))
}

/// A site's type (137: cities, research stations and factories follow one
/// set of rules and differ only in output).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiteKind {
    City,
    Station,
    Factory,
}

impl SiteKind {
    pub fn name(self) -> &'static str {
        match self {
            SiteKind::City => "city",
            SiteKind::Station => "station",
            SiteKind::Factory => "factory",
        }
    }
}

/// The site on hex `h`, if any.
pub fn site_kind(w: &World, g: &GameState, h: usize) -> Option<SiteKind> {
    if is_city(w, h) {
        Some(SiteKind::City)
    } else if g.tokens(h, STATION) > 0 {
        Some(SiteKind::Station)
    } else if g.tokens(h, FACTORY) > 0 {
        Some(SiteKind::Factory)
    } else {
        None
    }
}

/// 137: the site price of a neutral station or factory: `claim_station`, `claim_factory`. 262 cut the Claim
/// action that paid it in ambassadors: it is Liberate's price for a station or factory now (Matt kept the
/// names), and the base a card's claim adds its extra to ([`card_claim_extra`]). A city has none (254: no city
/// is claimed; Liberate's city price is `liberate_at`).
pub fn claim_ambassadors(w: &World, kind: SiteKind) -> i64 {
    match kind {
        SiteKind::City => liberate_at(w),
        _ => w.int(&format!("board.claim_{}", kind.name())).max(1),
    }
}

/// 137: the ambassadors that liberate an adversary site of `kind`.
pub fn liberate_ambassadors(w: &World, kind: SiteKind) -> i64 {
    // 206 (amendment 9208): one table (its city price is `liberate_at`, the hot knob).
    claim_ambassadors(w, kind)
}

/// What claiming neutral station or factory `h` would cost in ambassadors, if a card may claim it: the site's
/// price ([`claim_ambassadors`]); a site with an agent on it, or a city (254, 262), can't be claimed.
pub fn claim_cost(w: &World, g: &GameState, h: usize) -> Option<i64> {
    let kind = site_kind(w, g, h)?;
    // 206 (amendment 9208): no "+1 per agent": a site with an agent on it can't be claimed (clear them first).
    if corrupted(g, h) || cube_owner(g, h).is_some() || g.tokens(h, ADV_AMBASSADOR) > 0 || kind == SiteKind::City {
        return None;
    }
    Some(claim_ambassadors(w, kind))
}

/// What liberating adversary site `h` costs in ambassadors, if it is an adversary site that can be liberated:
/// no agent on it (206), and no troop on or next to it (r141, the guarding rule).
pub fn liberate_cost(w: &World, g: &GameState, h: usize) -> Option<i64> {
    let kind = site_kind(w, g, h)?;
    if !corrupted(g, h) || g.tokens(h, ADV_AMBASSADOR) > 0 || troop_guarded(w, g, h) {
        return None;
    }
    Some(liberate_ambassadors(w, kind))
}

/// r75 (Matt's ruling 2, 2026-11-10): the ambassadors standing on site `h` that a Liberate of `seat` there
/// spends before its supply: the seat's own, never another player's (166 spent anyone's).
pub fn site_standing(g: &GameState, seat: usize, h: usize) -> i64 {
    g.tokens(h, ambassador(seat))
}

/// What seat `seat`'s supply pays to liberate adversary site `h`: the bill less its own ambassadors standing
/// there, if there is such a site (262 cut the Claim action, which paid the same way).
pub fn site_supply_cost(w: &World, g: &GameState, seat: usize, h: usize) -> Option<i64> {
    liberate_cost(w, g, h).map(|n| (n - site_standing(g, seat, h)).max(0))
}

/// r75's one payment rule, Liberate's: `seat` pays the bill of site `h` with its ambassadors on it first, then
/// its supply.
pub fn pay_site(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    let mut left = liberate_cost(&w, g, h).unwrap_or(0);
    let k = ambassador(seat);
    let n = g.tokens(h, k).min(left);
    if n > 0 {
        cx.add_tokens(g, h, k, -n);
        cx.tally(g, "board.site_standing_spent", n);
        left -= n;
    }
    cx.add_resource(g, seat, Resource::Ambassadors, -left);
}

/// The neutral research stations and factories seat `seat` may claim with a card (142, `claim_site`; 256:
/// factories too): in its reach, a site claim on it (`can_claim`), and any extra ([`card_claim_extra`], 0 since
/// 206 refused a site with an agent on it), paid from the seat's ambassadors (178: [`pay_card_claim`]). A city is
/// never claimed (254).
pub fn card_claim_sites(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    let held = g.resource(seat, Resource::Ambassadors);
    (0..g.hexes.len())
        .filter(|&h| {
            matches!(site_kind(w, g, h), Some(SiteKind::Station | SiteKind::Factory))
                && card_claim_extra(w, g, h).is_some_and(|n| n <= held + site_standing(g, seat, h))
                && can_claim(w, g, seat, h)
        })
        .collect()
}

/// 256: a neutral station or factory a liberate card's rider may claim next to the city it frees: a claim on
/// it and no embargo; the card's target, not the seat's reach, puts it in range.
pub fn card_claimable_near(w: &World, g: &GameState, h: usize) -> bool {
    matches!(site_kind(w, g, h), Some(SiteKind::Station | SiteKind::Factory))
        && card_claim_extra(w, g, h) == Some(0)
        && !hex_flagged(w, g, RoundFlag::Embargo, h)
}

/// 148 (D70): what a card's claim of neutral site `h` costs beyond the card's own price: the old Claim action's
/// "1 more for each agent on it", in the seat's ambassadors (0 since 206: a site with an agent can't be claimed).
pub fn card_claim_extra(w: &World, g: &GameState, h: usize) -> Option<i64> {
    let kind = site_kind(w, g, h)?;
    claim_cost(w, g, h).map(|n| n - claim_ambassadors(w, kind))
}

/// 178 (amendment 8112): a card's claim of station `h` pays its extra ([`card_claim_extra`]) as [`pay_site`]
/// pays Liberate's bill: the seat's own ambassadors on the station first, then its supply.
pub fn pay_card_claim(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    let mut left = card_claim_extra(&w, g, h).unwrap_or(0);
    let k = ambassador(seat);
    let n = g.tokens(h, k).min(left);
    if n > 0 {
        cx.add_tokens(g, h, k, -n);
        cx.tally(g, "board.site_standing_spent", n);
        left -= n;
    }
    if left > 0 {
        cx.add_resource(g, seat, Resource::Ambassadors, -left);
    }
}

/// Seat `seat` claims neutral site `h` (a card, or a liberate card's rider: `route`, 142's tally): the
/// agents on it cancel, and its cube goes on.
pub fn claim_site(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, route: &str) {
    let w = cx.world;
    let Some(kind) = site_kind(&w, g, h) else { return };
    let adv = g.tokens(h, ADV_AMBASSADOR);
    if adv > 0 {
        adversary::remove(g, cx, h, adv);
        cx.tally(g, "board.canceled", adv);
    }
    cx.add_tokens(g, h, cube(seat), 1);
    cx.tally(g, &format!("board.claimed.{}", kind.name()), 1);
    cx.tally(g, &format!("board.claimed_by.{route}.{}", kind.name()), 1);
    // 190: neutral claims by round (stats only).
    cx.per_round(g, &format!("map.claims.{}", kind.name()), 1);
    cx.note(Some(h), || format!("Seat {seat} claims the {} at {}", kind.name(), hex_name(&w, h)));
}

/// Adversary site `h` is liberated by seat `seat`'s massed ambassadors (the
/// Liberate action, paid): the agents on it cancel and the adversary's cube comes
/// off; 252 (Matt's ruling 4, free then claim; reverses r75) leaves it neutral, for a card to claim
/// (262: only cards claim), unless `own` (a rare card power) puts the seat's cube on. A city sways `alignment.free_city`
/// toward Unity, a station or factory `alignment.free_facility` (a botnet on it
/// stays). No agents land on it. A liberate card liberates through here too, unpaid (178, Matt's ruling #29).
pub fn liberate_site(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, own: bool) {
    let w = cx.world;
    let Some(kind) = site_kind(&w, g, h) else { return };
    let adv = g.tokens(h, ADV_AMBASSADOR);
    if adv > 0 {
        adversary::remove(g, cx, h, adv);
        cx.tally(g, "board.canceled", adv);
    }
    cx.tally(g, &format!("board.liberated.{}", kind.name()), 1);
    cx.add_tokens(g, h, ADV_CUBE, -1);
    if kind == SiteKind::City {
        cx.tally(g, "board.cities_freed", 1);
        alignment::for_city_freed(g, cx);
    } else {
        alignment::unity_from(g, cx, kind.name(), w.int("alignment.free_facility"));
    }
    // 253 (amendment 11525): `liberate_neutral` 0 is r112's rule (the site becomes the liberator's), a screen's
    // off arm for ruling 4.
    // 254 (`cities_unowned`): a liberated city is its country's whatever the route (Hostile Buyout's `own` included).
    let city = kind == SiteKind::City;
    if !(city && cities_unowned(&w)) && (own || w.int("board.liberate_neutral") == 0) {
        cx.add_tokens(g, h, cube(seat), 1);
    }
    cx.note(Some(h), || format!("Seat {seat} liberates the {} at {}", kind.name(), hex_name(&w, h)));
}

/// Whether an agent stands on hex `h` or an adjacent hex
/// (what Clear City clears).
pub fn guarded(w: &World, g: &GameState, h: usize) -> bool {
    next_to(w, h, |x| g.tokens(x, ADV_AMBASSADOR) > 0)
}

/// The ambassadors that liberate a corrupted city (137: the Liberate action;
/// it was the fallback's count of standing ambassadors).
pub fn liberate_at(w: &World) -> i64 {
    w.hot(Hot::LiberateAt).max(1)
}

/// Moves one of `seat`'s ambassadors (from its supply or a hex, the seat
/// chooses) onto hex `to`.
pub fn move_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, to: usize) -> Res {
    move_ambassador_from(g, cx, seat, to, None)
}

/// [`move_ambassador`], the board ambassadors limited to hexes in `sources` when given (236: a move card takes
/// them from on or next to its target; the supply stays a source).
pub fn move_ambassador_from(g: &mut GameState, cx: &mut Cx, seat: usize, to: usize, sources: Option<&[usize]>) -> Res {
    let w = cx.world;
    let k = ambassador(seat);
    let mut from = ambassador_hexes(g, seat);
    from.retain(|&h| h != to && sources.is_none_or(|s| s.contains(&h)));
    let mut opts = Vec::new();
    if g.resource(seat, Resource::Ambassadors) > 0 {
        opts.push(opt("from your supply"));
    }
    let off = opts.len();
    opts.extend(from.iter().map(|&h| hex_opt(&w, h)));
    let prompt = || format!(
        "Seat {seat}: move which ambassador to {}?",
        hex_name(&w, to)
    );
    let Some(i) = cx.choose(seat, prompt, opts)? else {
        return Ok(());
    };
    if i < off {
        cx.add_resource(g, seat, Resource::Ambassadors, -1);
    } else {
        cx.add_tokens(g, from[i - off], k, -1);
    }
    place_ambassador(g, cx, seat, to);
    // 149: ambassador moves a seat makes in a game (the movement read).
    cx.tally(g, &KeyBuf::new("board.amb_moves.").num(seat), 1);
    Ok(())
}
