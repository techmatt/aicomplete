//! Player ambassadors, holding and claiming sites, and freeing cities.

use crate::module::Hot;
use super::*;

/// Whether seat `s` can claim the site on hex `h` (align a city, take a
/// station or factory): its node is on or next to `h`, no botnet is, and no
/// embargo is up on it (its country's, or 135's on the hexes near a threat).
pub fn can_claim(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    in_reach(w, g, seat, h)
        && !next_to(w, h, |x| g.tokens(x, BOTNET) > 0)
        && !hex_flagged(w, g, RoundFlag::Embargo, h)
}

pub fn ambassadors_on_board(g: &GameState, seat: usize) -> i64 {
    let k = ambassador(seat);
    (0..g.hexes.len()).map(|h| g.tokens(h, k)).sum()
}

/// Seat `s`'s player ambassador arrives on hex `h`: it cancels an
/// adversary ambassador there (both removed, no Unity); on a neutral
/// station or factory the seat can claim it makes the site the seat's; on a
/// corrupted city, enough of them free it. Otherwise it waits on the piece.
pub fn place_ambassador(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    if g.tokens(h, ADV_AMBASSADOR) > 0 {
        adversary::remove(g, cx, h, 1);
        cx.tally(g, "board.canceled", 1);
        return;
    }
    if has_building(g, h) && !corrupted(g, h) && cube_owner(g, h).is_none() && can_claim(&w, g, seat, h) {
        cx.add_tokens(g, h, cube(seat), 1);
        let (what, tally) = if g.tokens(h, STATION) > 0 {
            ("station", "board.stations_owned")
        } else {
            ("factory", "board.factories_owned")
        };
        cx.tally(g, tally, 1);
        cx.note(Some(h), || {
            format!("Seat {seat} takes the {what} at {}", hex_name(&w, h))
        });
        return;
    }
    cx.add_tokens(g, h, ambassador(seat), 1);
}

/// Whether an adversary ambassador stands on hex `h` or an adjacent hex
/// (what Clear City clears).
pub fn guarded(w: &World, g: &GameState, h: usize) -> bool {
    next_to(w, h, |x| g.tokens(x, ADV_AMBASSADOR) > 0)
}

/// The fallback (080, r34): player ambassadors on a corrupted city that
/// liberate it, a flat `liberate_at` (027-r33's hold, `hold_base` plus the
/// adversary ambassadors on or next to it, is gone).
pub fn liberate_at(w: &World) -> i64 {
    w.hot(Hot::LiberateAt).max(1)
}

/// Player ambassadors on hex `h` that count toward liberating it (080): from
/// seats whose node is on or next to it.
pub fn liberating_ambassadors(w: &World, g: &GameState, h: usize) -> i64 {
    (0..g.seats.len())
        .filter(|&s| in_reach(w, g, s, h))
        .map(|s| g.tokens(h, ambassador(s)))
        .sum()
}

/// City `h` is liberated: the adversary's cube comes off, so it is clean,
/// Unity sways, and it drops down the piece ladder rather than off the board
/// (027: `free_leaves` adversary ambassadors stay on it). A liberate card
/// with `align` (080) puts the seat's cube on it afterward
/// (`effects::removal::liberate_city`).
pub fn liberate(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    cx.add_tokens(g, h, ADV_CUBE, -1);
    cx.tally(g, "board.cities_freed", 1);
    alignment::for_city_freed(g, cx);
    adversary::place(g, cx, h, w.int("board.free_leaves"));
    cx.note(Some(h), || format!("{} is liberated", hex_name(&w, h)));
}

/// The fallback (080): a corrupted city is liberated the moment
/// `liberate_at` player ambassadors stand on it from seats with a node on or
/// next to it; that many are used up, lower seats first. Checked after every
/// action and automatic phase.
pub(super) fn free_cities(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        if !corrupted_city(&w, g, h) {
            continue;
        }
        let need = liberate_at(&w);
        if liberating_ambassadors(&w, g, h) < need {
            continue;
        }
        let mut left = need;
        for s in 0..g.seats.len() {
            if !in_reach(&w, g, s, h) {
                continue;
            }
            let k = g.tokens(h, ambassador(s)).min(left);
            cx.add_tokens(g, h, ambassador(s), -k);
            left -= k;
        }
        cx.tally(g, "board.liberated_fallback", 1);
        liberate(g, cx, h);
    }
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
    Ok(())
}
