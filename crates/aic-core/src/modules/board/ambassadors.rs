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

/// 137: the ambassadors that claim a neutral site of `kind` (the adversary's
/// Corrupt X for that type: `claim_city` is today's `corrupt_at`).
pub fn claim_ambassadors(w: &World, kind: SiteKind) -> i64 {
    w.int(&format!("board.claim_{}", kind.name())).max(1)
}

/// 137: the ambassadors that liberate an adversary site of `kind`.
pub fn liberate_ambassadors(w: &World, kind: SiteKind) -> i64 {
    match kind {
        SiteKind::City => liberate_at(w),
        _ => w.int("board.liberate_facility_at").max(1),
    }
}

/// What claiming neutral site `h` costs in ambassadors (one more for each
/// agent on it, which cancel), if it is a neutral site.
pub fn claim_cost(w: &World, g: &GameState, h: usize) -> Option<i64> {
    let kind = site_kind(w, g, h)?;
    if corrupted(g, h) || cube_owner(g, h).is_some() {
        return None;
    }
    Some(claim_ambassadors(w, kind) + g.tokens(h, ADV_AMBASSADOR))
}

/// What liberating adversary site `h` costs in ambassadors (one more for each
/// agent on it), if it is an adversary site.
pub fn liberate_cost(w: &World, g: &GameState, h: usize) -> Option<i64> {
    let kind = site_kind(w, g, h)?;
    if !corrupted(g, h) {
        return None;
    }
    Some(liberate_ambassadors(w, kind) + g.tokens(h, ADV_AMBASSADOR))
}

/// r75 (Matt's ruling 2, 2026-11-10): the ambassadors standing on site `h` that a Claim or Liberate of
/// `seat` there spends before its supply: the seat's own, never another player's (166 spent anyone's).
pub fn site_standing(g: &GameState, seat: usize, h: usize) -> i64 {
    g.tokens(h, ambassador(seat))
}

/// What seat `seat`'s supply pays to claim neutral site `h` (`liberate`: false) or liberate adversary site
/// `h` (true): the bill less its own ambassadors standing there, if there is such a site.
pub fn site_supply_cost(w: &World, g: &GameState, seat: usize, h: usize, liberate: bool) -> Option<i64> {
    let bill = if liberate { liberate_cost(w, g, h) } else { claim_cost(w, g, h) };
    bill.map(|n| (n - site_standing(g, seat, h)).max(0))
}

/// r75's one payment rule for Claim and Liberate: `seat` pays the bill of site `h` with its ambassadors on
/// it first, then its supply (each agent there cancels one of them: the bill's "1 more for each agent").
pub fn pay_site(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, liberate: bool) {
    let w = cx.world;
    let mut left = if liberate { liberate_cost(&w, g, h) } else { claim_cost(&w, g, h) }.unwrap_or(0);
    let k = ambassador(seat);
    let n = g.tokens(h, k).min(left);
    if n > 0 {
        cx.add_tokens(g, h, k, -n);
        cx.tally(g, "board.site_standing_spent", n);
        left -= n;
    }
    cx.add_resource(g, seat, Resource::Ambassadors, -left);
}

/// The neutral research stations seat `seat` may claim with a card (142, `claim_station`): in its reach, a
/// site claim on it (`can_claim`), and, as the Claim action asks (148, D70), one of the seat's ambassadors
/// for each agent on it ([`card_claim_extra`]).
pub fn card_claim_stations(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    let held = g.resource(seat, Resource::Ambassadors);
    (0..g.hexes.len())
        .filter(|&h| {
            site_kind(w, g, h) == Some(SiteKind::Station)
                && card_claim_extra(w, g, h).is_some_and(|n| n <= held)
                && can_claim(w, g, seat, h)
        })
        .collect()
}

/// 148 (D70): what a card's claim of neutral station `h` costs beyond the card's own price: the Claim
/// action's "1 more for each agent on it", in the seat's ambassadors (they cancel).
pub fn card_claim_extra(w: &World, g: &GameState, h: usize) -> Option<i64> {
    claim_cost(w, g, h).map(|n| n - claim_ambassadors(w, SiteKind::Station))
}

/// Seat `seat` claims neutral site `h` (the Claim action, paid, or a card: `route`, 142's tally): the
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
    cx.note(Some(h), || format!("Seat {seat} claims the {} at {}", kind.name(), hex_name(&w, h)));
}

/// Adversary site `h` is liberated by seat `seat`'s massed ambassadors (the
/// Liberate action, paid): the agents on it cancel, the adversary's cube comes
/// off and r75 (Matt's ruling 2) puts the seat's on: it is the seat's, as a
/// claim makes it (it turned neutral). A city sways `alignment.free_city`
/// toward Unity, a station or factory `alignment.free_facility` (a botnet on it
/// stays). No agents land on it (`free_leaves` is a liberate card's, [`liberate`]).
pub fn liberate_site(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
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
    cx.add_tokens(g, h, cube(seat), 1);
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

/// City `h` is liberated: the adversary's cube comes off, so it is clean,
/// Unity sways, and `board.free_leaves` agents land on it
/// (027; none at 0). A liberate card
/// with `align` (080) puts the seat's cube on it afterward
/// (`effects::removal::liberate_city`).
pub fn liberate(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    cx.add_tokens(g, h, ADV_CUBE, -1);
    cx.tally(g, "board.cities_freed", 1);
    alignment::for_city_freed(g, cx);
    adversary::place(g, cx, h, w.int("board.free_leaves"))?;
    cx.note(Some(h), || format!("{} is liberated", hex_name(&w, h)));
    Ok(())
}

/// Moves one of `seat`'s ambassadors (from its supply or a hex, the seat
/// chooses) onto hex `to`.
pub fn move_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, to: usize) -> Res {
    let w = cx.world;
    let k = ambassador(seat);
    let mut from = ambassador_hexes(g, seat);
    from.retain(|&h| h != to);
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
