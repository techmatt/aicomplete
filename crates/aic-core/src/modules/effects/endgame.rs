//! 158: the tier-2 end-game shapes (Matt, 2026-11-09): convert, gather and the firewall.

use super::*;

/// The cities in seat `seat`'s reach with an adversary ambassador on or next to them (what a conversion takes from).
pub fn convert_cities(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| is_city(w, h) && in_reach(w, g, seat, h) && board::guarded(w, g, h))
        .collect()
}

/// Seat `seat`'s nodes whose reach takes in city `city` (on it or next to it).
fn casting_nodes(w: &World, g: &GameState, seat: usize, city: usize) -> Vec<usize> {
    let k = node(seat);
    let mut out: Vec<usize> = w.map.hexes[city].neighbors.iter().copied().filter(|&x| g.tokens(x, k) > 0).collect();
    if g.tokens(city, k) > 0 {
        out.push(city);
    }
    out.sort_unstable();
    out
}

/// 158 (amendment 6949): up to `max` adversary ambassadors on or next to one city in reach become the seat's,
/// each onto the node whose reach the card used (arriving as a player ambassador does: onto an adversary
/// ambassador there, the two cancel).
pub(super) fn convert(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64) -> Res {
    let w = cx.world;
    let cities = convert_cities(&w, g, seat);
    let opts = cities.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: convert adversary ambassadors around which city?");
    let Some(i) = cx.choose(seat, prompt, opts)? else {
        return Ok(());
    };
    let city = cities[i];
    let nodes = casting_nodes(&w, g, seat, city);
    let to = match nodes.as_slice() {
        [] => return Ok(()),
        [only] => *only,
        _ => {
            let opts = nodes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = || format!("Seat {seat}: which node casts the conversion?");
            match cx.choose(seat, prompt, opts)? {
                Some(k) => nodes[k],
                None => return Ok(()),
            }
        }
    };
    let mut hexes = w.map.hexes[city].neighbors.to_vec();
    hexes.sort_unstable();
    hexes.insert(0, city);
    let mut left = max;
    while left > 0 {
        let held: Vec<usize> = hexes.iter().copied().filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 0).collect();
        let count: i64 = held.iter().map(|&h| g.tokens(h, ADV_AMBASSADOR)).sum();
        if held.is_empty() {
            break;
        }
        let at = if count <= left || held.len() == 1 {
            held[0]
        } else {
            let opts = held.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = || format!("Seat {seat}: convert an adversary ambassador from which hex?");
            match cx.choose(seat, prompt, opts)? {
                Some(k) => held[k],
                None => break,
            }
        };
        adversary::remove(g, cx, at, 1);
        board::place_ambassador(g, cx, seat, to);
        cx.tally(g, "effects.converted", 1);
        left -= 1;
    }
    Ok(())
}

/// The hexes a gather may target: a corrupted city in reach (158: what is gathered can't corrupt a clean site),
/// no firewall, with an adversary piece it moves on a hex next to it or a piece there to strike.
pub fn gather_hexes(w: &World, g: &GameState, seat: usize, troops: bool, strike: i64) -> Vec<usize> {
    let has = |h: usize| g.tokens(h, ADV_AMBASSADOR) > 0 || (troops && g.tokens(h, TROOP) > 0);
    (0..g.hexes.len())
        .filter(|&h| is_city(w, h) && corrupted(g, h) && in_reach(w, g, seat, h) && !firewalled(g, h))
        .filter(|&h| w.map.hexes[h].neighbors.iter().any(|&x| has(x)) || (strike > 0 && has(h)))
        .collect()
}

/// 158: every adversary ambassador (with `troops`, troop) next to one hex in reach moves onto it; then up to
/// `strike` adversary pieces there come off, troops first.
pub(super) fn gather(g: &mut GameState, cx: &mut Cx, seat: usize, troops: bool, strike: i64) -> Res {
    let w = cx.world;
    let hexes = gather_hexes(&w, g, seat, troops, strike);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: gather adversary pieces onto which hex?");
    let Some(i) = cx.choose(seat, prompt, opts)? else {
        return Ok(());
    };
    let to = hexes[i];
    let mut from = w.map.hexes[to].neighbors.to_vec();
    from.sort_unstable();
    for x in from {
        let a = g.tokens(x, ADV_AMBASSADOR);
        if a > 0 {
            cx.add_tokens(g, x, ADV_AMBASSADOR, -a);
            cx.add_tokens(g, to, ADV_AMBASSADOR, a);
            cx.tally(g, "effects.gathered", a);
        }
        if troops {
            let t = g.tokens(x, TROOP);
            if t > 0 {
                cx.add_tokens(g, x, TROOP, -t);
                cx.add_tokens(g, to, TROOP, t);
                cx.tally(g, "effects.gathered_troops", t);
            }
        }
    }
    let mut left = strike;
    for kind in [TROOP, ADV_AMBASSADOR] {
        while left > 0 && g.tokens(to, kind) > 0 {
            adversary::remove_piece(g, cx, Some(seat), to, kind);
            cx.tally(g, "effects.struck", 1);
            left -= 1;
        }
    }
    Ok(())
}

/// The hexes a firewall may go on: land in reach, none there yet.
pub fn firewall_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len()).filter(|&h| !is_ocean(w, h) && in_reach(w, g, seat, h) && !firewalled(g, h)).collect()
}

/// 158: a firewall on one hex in reach (it comes off at the end of the next adversary turn).
pub(super) fn firewall(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let hexes = firewall_hexes(&w, g, seat);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: put a firewall on which hex?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        cx.add_tokens(g, hexes[i], FIREWALL, 1);
        cx.tally(g, "effects.firewalls", 1);
        cx.note(Some(hexes[i]), || format!("Seat {seat} firewalls {}", hex_name(&w, hexes[i])));
    }
    Ok(())
}
