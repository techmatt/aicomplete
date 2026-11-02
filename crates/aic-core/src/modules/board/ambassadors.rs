//! Player ambassadors, holding and claiming sites, and freeing cities.

use super::*;

/// Whether seat `s` can claim the site on hex `h` (align a city, take a
/// station or factory): its presence is on or next to `h`, and no node is.
pub fn can_claim(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    in_reach(w, g, seat, h)
        && !next_to(w, h, |x| g.tokens(x, NODE) > 0)
        && !flagged(g, RoundFlag::Embargo, w.map.hexes[h].country as usize)
}

pub fn ambassadors_on_board(g: &GameState, seat: usize) -> i64 {
    let k = ambassador(seat);
    (0..g.hexes.len()).map(|h| g.tokens(h, &k)).sum()
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
        cx.add_tokens(g, h, &cube(seat), 1);
        let what = if g.tokens(h, STATION) > 0 {
            "station"
        } else {
            "factory"
        };
        cx.tally(
            g,
            &format!(
                "board.{}_owned",
                if what == "station" {
                    "stations"
                } else {
                    "factories"
                }
            ),
            1,
        );
        cx.note(Some(h), || {
            format!("Seat {seat} takes the {what} at {}", hex_name(&w, h))
        });
        return;
    }
    cx.add_tokens(g, h, &ambassador(seat), 1);
}

/// Whether an adversary ambassador stands on hex `h` or an adjacent hex
/// (each adds 1 to a corrupted city's [`hold`]).
pub fn guarded(w: &World, g: &GameState, h: usize) -> bool {
    next_to(w, h, |x| g.tokens(x, ADV_AMBASSADOR) > 0)
}

/// A corrupted city's hold (027): the player ambassadors, from any players,
/// that free it. `hold_base`, plus 1 for each adversary ambassador on or
/// next to it, never below `hold_min`.
pub fn hold(w: &World, g: &GameState, h: usize) -> i64 {
    let near: i64 = g.tokens(h, ADV_AMBASSADOR)
        + w.map.hexes[h]
            .neighbors
            .iter()
            .map(|&x| g.tokens(x, ADV_AMBASSADOR))
            .sum::<i64>();
    (w.int("board.hold_base") + near).max(w.int("board.hold_min"))
}

/// Player ambassadors on hex `h`, from all seats.
pub fn ambassadors_on(g: &GameState, h: usize) -> i64 {
    (0..g.seats.len()).map(|s| g.tokens(h, &ambassador(s))).sum()
}

/// City `h` frees: the adversary's cube comes off, Unity sways, and it drops
/// down the piece ladder rather than off the board (027: `free_leaves`
/// adversary ambassadors stay on it).
pub fn liberate(g: &mut GameState, cx: &mut Cx, h: usize) {
    let w = cx.world;
    cx.add_tokens(g, h, ADV_CUBE, -1);
    cx.tally(g, "board.cities_freed", 1);
    alignment::for_city_freed(g, cx);
    adversary::place(g, cx, h, w.int("board.free_leaves"));
    cx.note(Some(h), || format!("{} is freed", hex_name(&w, h)));
}

/// 027: a corrupted city frees the moment player ambassadors on it, from any
/// players, reach its [`hold`]; that many are used up, lower seats first.
/// Checked after every action and automatic phase.
pub(super) fn free_cities(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        if !corrupted_city(&w, g, h) {
            continue;
        }
        let need = hold(&w, g, h);
        // 070: a face-up influence threat here holds the city until it is countered.
        if ambassadors_on(g, h) < need || threats::shielded(&w, g, h, ThreatSource::Influence) {
            continue;
        }
        let mut left = need;
        for s in 0..g.seats.len() {
            let k = g.tokens(h, &ambassador(s)).min(left);
            cx.add_tokens(g, h, &ambassador(s), -k);
            left -= k;
        }
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
    let prompt = format!(
        "Seat {seat}: move which ambassador to {}?",
        hex_name(&w, to)
    );
    let Some(i) = cx.choose(seat, &prompt, opts)? else {
        return Ok(());
    };
    if i < off {
        cx.add_resource(g, seat, Resource::Ambassadors, -1);
    } else {
        cx.add_tokens(g, from[i - off], &k, -1);
    }
    place_ambassador(g, cx, seat, to);
    Ok(())
}
