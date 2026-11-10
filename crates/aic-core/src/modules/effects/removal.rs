//! Taking adversary pieces off the board.

use super::*;

pub(super) fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
    let hexes = damage_hexes(&w, g, seat, ambassadors, troops);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: remove adversary pieces in which hex?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        adversary::damage(g, cx, seat, hexes[i], ambassadors, troops)?;
    }
    Ok(())
}

pub(super) fn damage_country(g: &mut GameState, cx: &mut Cx, seat: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
    let cs = countries_where(&w, |h| adversary::can_damage(g, h, ambassadors, troops));
    let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
    let prompt = || format!("Seat {seat}: remove pieces in each adversary hex of which country?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        let hexes: Vec<usize> = w.map.countries[cs[i]]
            .hexes
            .iter()
            .copied()
            .filter(|&h| adversary::can_damage(g, h, ambassadors, troops))
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
        .filter(|&h| adversary::can_destroy_all(g, h))
        .collect();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt =
        format!("Seat {seat}: remove every agent and troop in which hex?");
    if let Some(i) = cx.choose(seat, &prompt, opts)? {
        adversary::destroy_all(g, cx, seat, hexes[i]);
    }
    Ok(())
}

pub(super) fn remove_botnet(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let hexes: Vec<usize> = (0..g.hexes.len())
        .filter(|&h| botnet_in_reach(&w, g, seat, h))
        .collect();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: remove the botnet in which hex?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        adversary::remove_botnet(g, cx, hexes[i]);
    }
    Ok(())
}

pub(super) fn remove_ambassadors_country(
    g: &mut GameState,
    cx: &mut Cx,
    seat: usize,
    unity_per: i64,
    max: i64,
    reach: bool,
) -> Res {
    let w = cx.world;
    let removed = if reach {
        // 111 (Matt's ruling 3): in reach, across countries; no country to name.
        let area = reach_hexes(&w, g, seat);
        up_to_in(g, cx, seat, &area, max, ADV_AMBASSADOR)?
    } else {
        let cs = countries_where(&w, |h| g.tokens(h, ADV_AMBASSADOR) > 0);
        let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
        let prompt = || format!("Seat {seat}: remove agents in which country?");
        let Some(i) = cx.choose(seat, prompt, opts)? else {
            return Ok(());
        };
        if max > 0 {
            up_to_in(g, cx, seat, &w.map.countries[cs[i]].hexes, max, ADV_AMBASSADOR)?
        } else {
            adversary::remove_all_in_country(g, cx, cs[i])
        }
    };
    if unity_per > 0 {
        alignment::unity_from(g, cx, "pushback", removed / unity_per);
    }
    Ok(())
}

/// 076 (ruling 6): up to `count` adversary troops in one country, any country;
/// with `reach` (111, ruling 3), in reach instead.
pub(super) fn remove_troops(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64, reach: bool) -> Res {
    let w = cx.world;
    let removed = if reach {
        let area = reach_hexes(&w, g, seat);
        up_to_in(g, cx, seat, &area, count, TROOP)?
    } else {
        let cs = countries_where(&w, |h| adversary::can_damage(g, h, 0, 1));
        let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
        let prompt = || format!("Seat {seat}: remove adversary troops in which country?");
        let Some(i) = cx.choose(seat, prompt, opts)? else {
            return Ok(());
        };
        up_to_in(g, cx, seat, &w.map.countries[cs[i]].hexes, count, TROOP)?
    };
    // 093: what "up to N" removes in play, for the effect ledger.
    cx.tally(g, &format!("cards.remove_troops.{count}"), 1);
    cx.tally(g, &format!("cards.troops_removed.{count}"), removed);
    Ok(())
}

/// The hexes in seat `seat`'s reach (111).
fn reach_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len()).filter(|&h| in_reach(w, g, seat, h)).collect()
}

/// Removes up to `max` pieces of `kind` in the hexes of `area`, hex by hex: all
/// of them with no question when they are `max` or fewer, else the player picks
/// each hex, which gives up as many as it holds and the card has left.
fn up_to_in(g: &mut GameState, cx: &mut Cx, seat: usize, area: &[usize], max: i64, kind: &'static str) -> Res<i64> {
    let w = cx.world;
    let mut left = max;
    let mut removed = 0;
    while left > 0 {
        let held: Vec<usize> = area
            .iter()
            .copied()
            .filter(|&h| if kind == TROOP { adversary::can_damage(g, h, 0, 1) } else { g.tokens(h, kind) > 0 })
            .collect();
        let total: i64 = held.iter().map(|&h| g.tokens(h, kind)).sum();
        if held.is_empty() {
            break;
        }
        let at = if total <= left || held.len() == 1 {
            held[0]
        } else {
            let opts = held.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = || format!("Seat {seat}: remove {} from which hex?", piece_name(kind));
            match cx.choose(seat, prompt, opts)? {
                Some(k) => held[k],
                None => break,
            }
        };
        let n = g.tokens(at, kind).min(left);
        for _ in 0..n {
            adversary::remove_piece(g, cx, Some(seat), at, kind);
        }
        left -= n;
        removed += n;
    }
    Ok(removed)
}

pub(super) fn cleanse(
    g: &mut GameState,
    cx: &mut Cx,
    seat: usize,
    amount: i64,
    one_country: bool,
    cities: bool,
    only: Option<Facility>,
) -> Res {
    let w = cx.world;
    let mut country: Option<u8> = None;
    for k in 0..amount {
        let hexes: Vec<usize> = (0..g.hexes.len())
            .filter(|&h| cleansable(&w, g, h, cities, only) && in_reach(&w, g, seat, h))
            .filter(|&h| country.is_none_or(|c| w.map.hexes[h].country == c))
            .collect();
        if hexes.is_empty() {
            break;
        }
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("Seat {seat}: take the adversary cube off which hex?");
        let Some(i) = pick_or_stop(cx, seat, prompt, opts, k == 0)? else {
            break;
        };
        cx.tally(g, "effects.cleansed", 1);
        if is_city(&w, hexes[i]) {
            cx.tally(g, "board.liberated_card", 1);
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

/// A liberate card (080): one corrupted city in reach is liberated; with
/// `align` the seat's cube goes on it. Each engine's twist (r35): `ambassadors`
/// gained, up to `troops` adversary troops off in the city's country, and with
/// `botnet` one botnet on or next to the city off.
#[allow(clippy::too_many_arguments)]
pub(super) fn liberate_city(
    g: &mut GameState,
    cx: &mut Cx,
    seat: usize,
    align: bool,
    ambassadors: i64,
    troops: i64,
    botnet: bool,
) -> Res {
    let w = cx.world;
    let cities = liberable_cities(&w, g, seat);
    let opts = cities.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: liberate which corrupted city?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        let h = cities[i];
        cx.tally(g, "board.liberated_card", 1);
        board::liberate(g, cx, h);
        if align {
            cx.add_tokens(g, h, cube(seat), 1);
            cx.tally(g, "board.liberated_aligned", 1);
        }
        if ambassadors > 0 {
            cx.add_resource(g, seat, Resource::Ambassadors, ambassadors);
            cx.tally(g, "board.liberated_ambassadors", ambassadors);
        }
        if troops > 0 {
            let c = w.map.hexes[h].country as usize;
            let n = up_to_in(g, cx, seat, &w.map.countries[c].hexes, troops, TROOP)?;
            cx.tally(g, "board.liberated_troops", n);
        }
        if botnet {
            let mut near = w.map.hexes[h].neighbors.to_vec();
            near.sort_unstable();
            near.insert(0, h);
            let hexes: Vec<usize> = near
                .into_iter()
                .filter(|&x| g.tokens(x, BOTNET) > 0)
                .collect();
            let opts = hexes.iter().map(|&x| hex_opt(&w, x)).collect();
            let prompt = || format!("Seat {seat}: remove the botnet in which hex?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                adversary::remove_botnet(g, cx, hexes[i]);
                cx.tally(g, "board.liberated_botnet", 1);
            }
        }
    }
    Ok(())
}

pub(super) fn clear_city(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64) -> Res {
    let w = cx.world;
    let cities = guarded_cities(&w, g, seat);
    let opts = cities.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: clear agents around which corrupted city?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
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
                let prompt = || format!("Seat {seat}: remove an agent from which hex?");
                match cx.choose(seat, prompt, opts)? {
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

pub(super) fn cut_botnet(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let hexes = cuttable_botnets(&w, g, seat);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: cut off the botnet in which hex?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        cx.set_key(g, &adversary::cut_key(hexes[i]), 1);
        cx.tally(g, "effects.botnets_cut", 1);
    }
    Ok(())
}
