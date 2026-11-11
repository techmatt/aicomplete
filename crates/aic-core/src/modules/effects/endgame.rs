//! 158: the tier-2 end-game shapes (Matt, 2026-11-09): convert and the firewall (gather is `moves.rs`, 166).

use super::*;

/// The cities in seat `seat`'s reach with an agent on or next to them (what a conversion takes from).
pub fn convert_cities(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| is_city(w, h) && in_reach(w, g, seat, h) && board::guarded(w, g, h))
        .collect()
}

/// 158 (amendment 6949): up to `max` agents on or next to one city in reach become the seat's. 178 (Matt's
/// ruling #32): each becomes the seat's ambassador where it stood (it was on the node whose reach the card
/// used); another agent still there meets it as any arrival does, 1 for 1 (`board::place_ambassador`).
pub(super) fn convert(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64) -> Res {
    let w = cx.world;
    let cities = convert_cities(&w, g, seat);
    let opts = cities.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: convert agents around which city?");
    let Some(i) = cx.choose(seat, prompt, opts)? else {
        return Ok(());
    };
    let city = cities[i];
    let mut hexes = w.map.hexes[city].neighbors.to_vec();
    hexes.sort_unstable();
    hexes.insert(0, city);
    let mut left = max;
    // The agents go first, all of them, then the ambassadors take their places: a swap, so two converted on one
    // hex both stay; only an agent the card left there meets them.
    let mut taken: Vec<usize> = Vec::new();
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
            let prompt = || format!("Seat {seat}: convert an agent from which hex?");
            match cx.choose(seat, prompt, opts)? {
                Some(k) => held[k],
                None => break,
            }
        };
        adversary::remove(g, cx, at, 1);
        taken.push(at);
        cx.tally(g, "effects.converted", 1);
        left -= 1;
    }
    for at in taken {
        board::place_ambassador(g, cx, seat, at);
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
