//! Taking adversary pieces off the board.

use super::*;

pub(super) fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
        let hexes = damage_hexes(&w, g, seat, ambassadors, troops);
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {seat}: remove adversary pieces in which hex?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            adversary::damage(g, cx, seat, hexes[i], ambassadors, troops)?;
        }
    Ok(())
}

pub(super) fn damage_country(g: &mut GameState, cx: &mut Cx, seat: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
        let cs = countries_where(&w, |h| adversary::can_damage(&w, g, h, ambassadors, troops));
        let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
        let prompt = format!("Seat {seat}: remove pieces in each adversary hex of which country?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            let hexes: Vec<usize> = w.map.countries[cs[i]]
                .hexes
                .iter()
                .copied()
                .filter(|&h| adversary::can_damage(&w, g, h, ambassadors, troops))
                .collect();
            for h in hexes {
                adversary::damage(g, cx, seat, h, ambassadors, troops)?;
            }
        }
    Ok(())
}

pub(super) fn destroy_all(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
        let hexes: Vec<usize> = (0..g.hexes.len())
            .filter(|&h| adversary::can_destroy_all(&w, g, h))
            .collect();
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt =
            format!("Seat {seat}: remove every adversary ambassador and troop in which hex?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            adversary::destroy_all(g, cx, seat, hexes[i]);
        }
    Ok(())
}

pub(super) fn remove_node(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
        let hexes: Vec<usize> = (0..g.hexes.len())
            .filter(|&h| node_in_reach(&w, g, seat, h))
            .collect();
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {seat}: remove the node in which hex?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            adversary::remove_node(g, cx, hexes[i]);
        }
    Ok(())
}

pub(super) fn remove_ambassadors_country(g: &mut GameState, cx: &mut Cx, seat: usize, unity_per: i64) -> Res {
    let w = cx.world;
        let cs = countries_where(&w, |h| g.tokens(h, ADV_AMBASSADOR) > 0);
        let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
        let prompt =
            format!("Seat {seat}: remove every adversary ambassador in which country?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            let removed = adversary::remove_all_in_country(g, cx, cs[i]);
            if unity_per > 0 {
                alignment::sway(g, cx, Side::Unity, removed / unity_per);
            }
        }
    Ok(())
}

pub(super) fn cleanse(g: &mut GameState, cx: &mut Cx, seat: usize, amount: i64, one_country: bool, cities: bool) -> Res {
    let w = cx.world;
        let mut country: Option<u8> = None;
        for k in 0..amount {
            let hexes: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| cleansable(&w, g, h, cities) && in_reach(&w, g, seat, h))
                .filter(|&h| country.is_none_or(|c| w.map.hexes[h].country == c))
                .collect();
            if hexes.is_empty() {
                break;
            }
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: take the adversary cube off which hex?");
            let Some(i) = pick_or_stop(cx, seat, &prompt, opts, k == 0)? else {
                break;
            };
            cx.tally(g, "effects.cleansed", 1);
            if is_city(&w, hexes[i]) {
                board::liberate(g, cx, hexes[i]);
            } else {
                cx.add_tokens(g, hexes[i], ADV_CUBE, -1);
            }
            if one_country {
                country = Some(w.map.hexes[hexes[i]].country);
            }
        }
    Ok(())
}

pub(super) fn clear_city(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64) -> Res {
    let w = cx.world;
        let cities = guarded_cities(&w, g, seat);
        let opts = cities.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {seat}: clear adversary ambassadors around which corrupted city?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            let city = cities[i];
            let mut hexes = w.map.hexes[city].neighbors.to_vec();
            hexes.sort_unstable();
            hexes.push(city);
            // 059: the card says "on or next to": every one goes at max 0, else the
            // player picks each while there are more than the card can take.
            let mut left = if max > 0 { max } else { i64::MAX };
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
                    let prompt = format!("Seat {seat}: remove an adversary ambassador from which hex?");
                    match cx.choose(seat, &prompt, opts)? {
                        Some(k) => held[k],
                        None => break,
                    }
                };
                adversary::remove_piece(g, cx, Some(seat), at, ADV_AMBASSADOR);
                left -= 1;
            }
        }
    Ok(())
}

pub(super) fn cut_node(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
        let hexes = cuttable_nodes(&w, g, seat);
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {seat}: cut off the node in which hex?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            cx.set_key(g, &adversary::cut_key(hexes[i]), 1);
            cx.tally(g, "effects.nodes_cut", 1);
        }
    Ok(())
}
