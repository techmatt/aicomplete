//! Placing and removing adversary pieces: agents from the supply, troop landings, and the players' removals.

use super::*;

/// Troop pressure where the Protector stands (080, candidate): in the round's
/// build, `wall_troops` adversary troops land in each country where a seat with
/// Wall has a node, on the lowest-numbered adversary or neutral factory (178;
/// a Wall there keeps it in the supply), so Wall has strikes to stop.
pub(super) fn wall_pressure(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let n = w.int("adversary.wall_troops");
    if n == 0 {
        return Ok(());
    }
    let walls: Vec<usize> = (0..g.seats.len()).filter(|&s| board::has_wall(&w, s)).collect();
    for c in 0..w.map.countries.len() {
        if !walls.iter().any(|&s| has_node_in(&w, g, s, c)) {
            continue;
        }
        // 178 (Matt's ruling #18, printed on the Protector's board): only on an adversary or neutral factory,
        // never a player's; with none in the country, nothing lands (it took a city, or a player's factory).
        let hexes: Vec<usize> = w.map.countries[c]
            .hexes
            .iter()
            .copied()
            .filter(|&h| g.tokens(h, FACTORY) > 0 && crate::modules::common::cube_owner(g, h).is_none())
            .collect();
        if hexes.is_empty() {
            continue;
        }
        // 175: a Muster like any other: the level surcharge, a Muster prohibition, Wall keeping it in the supply.
        let n = n + muster_bonus(&w);
        land_troops(g, cx, &hexes, n, "Troops land where the Protector stands: on which hex?", "adversary.wall_troops", None, true)?;
    }
    Ok(())
}

/// Places `n` agents from the supply on hex `h`. The supply
/// is a cap (027): a placement it can't make doesn't happen.
pub fn place(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> Res {
    // 158: a firewall stops every arrival on its hex; they stay in the supply.
    if firewalled(g, h) {
        cx.tally(g, "adversary.firewalled", n.max(0));
        return Ok(());
    }
    for k in 0..n {
        if g.var(SUPPLY) <= 0 {
            cx.tally(g, "adversary.supply_short", n - k);
            return Ok(());
        }
        cx.add_key(g, SUPPLY, -1);
        cx.per_round(g, OPS, 1);
        arrive(g, cx, h)?;
    }
    Ok(())
}

/// One agent arrives on hex `h` from the supply: it cancels a
/// player ambassador there (both are removed, no Unity; r75: the players choose
/// whose, [`board::agent_cancels`]). 138: with the spread gone, no botnet forms
/// by arriving (botnets come only from printed lines).
fn arrive(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    if board::agent_cancels(g, cx, h)?.is_some() {
        cx.add_key(g, SUPPLY, 1);
        cx.tally(g, "adversary.canceled", 1);
        return Ok(());
    }
    cx.add_tokens(g, h, ADV_AMBASSADOR, 1);
    Ok(())
}

/// Removes the botnet on hex `h` (a card's effect, or r75's Take down a botnet standard action).
pub fn remove_botnet(g: &mut GameState, cx: &mut Cx, h: usize) {
    if g.tokens(h, BOTNET) > 0 {
        cx.add_tokens(g, h, BOTNET, -1);
        // A botnet formed here later is a new one: the hack cut this one (028).
        cx.set_key(g, &cut_key(h), 0);
        cx.tally(g, "adversary.botnets_removed", 1);
        alignment::for_botnet_removed(g, cx);
    }
}

/// Botnets on `hexes`, or on the whole board for `None`.
pub fn botnet_count(g: &GameState, hexes: Option<&[usize]>) -> i64 {
    match hexes {
        Some(hexes) => hexes.iter().map(|&h| g.tokens(h, BOTNET)).sum(),
        None => (0..g.hexes.len()).map(|h| g.tokens(h, BOTNET)).sum(),
    }
}

/// Removes up to `n` agents from hex `h` to the supply.
pub fn remove(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> i64 {
    let k = n.min(g.tokens(h, ADV_AMBASSADOR)).max(0);
    if k > 0 {
        cx.add_tokens(g, h, ADV_AMBASSADOR, -k);
        cx.add_key(g, SUPPLY, k);
    }
    k
}

/// Removes every agent in country `c` to the supply (one
/// tally for them all); returns how many.
pub fn remove_all_in_country(g: &mut GameState, cx: &mut Cx, c: usize) -> i64 {
    let w = cx.world;
    let mut removed = 0;
    for &h in &w.map.countries[c].hexes {
        let n = g.tokens(h, ADV_AMBASSADOR);
        removed += remove(g, cx, h, n);
    }
    cx.tally(g, "adversary.ambassadors_removed", removed);
    removed
}

/// An adversary troop lands on hex `h` (the caller counts it).
pub fn place_troop(g: &mut GameState, cx: &mut Cx, h: usize) {
    cx.add_tokens(g, h, TROOP, 1);
}

/// `count` adversary troops, each onto a factory among `hexes` (players
/// choose; 151, `muster`: the lowest-numbered, deterministic targeting for events, projects and the board), or
/// with no factory onto a city. With `destroy`, each destroys the node on the hex it lands on (027), named for
/// its tally. 175: a `muster` landing is also stopped by a Muster prohibition (the caller adds [`muster_bonus`]).
#[allow(clippy::too_many_arguments)]
pub fn land_troops(
    g: &mut GameState,
    cx: &mut Cx,
    hexes: &[usize],
    count: i64,
    prompt: &str,
    tally: &str,
    destroy: Option<&'static str>,
    muster: bool,
) -> Res {
    let w = cx.world;
    for _ in 0..count {
        // 068: a landing enters its hex, so a Wall stops it too. 175: a deterministic landing (a Muster) aims at
        // the lowest-numbered factory whatever stands there, and a Wall there keeps it in the supply; a chosen
        // one (a threat's) is offered only open hexes.
        let open = |h: &usize| muster || !board::walled(&w, g, *h);
        let mut spots: Vec<usize> = hexes
            .iter()
            .copied()
            .filter(open)
            .filter(|&h| g.tokens(h, FACTORY) > 0)
            .collect();
        if spots.is_empty() {
            spots = hexes.iter().copied().filter(open).filter(|&h| is_city(&w, h)).collect();
        }
        let i = if muster {
            let Some(i) = (0..spots.len()).min_by_key(|&i| w.map.hexes[spots[i]].number) else {
                break;
            };
            i
        } else {
            let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
            let Some(i) = cx.choose(0, prompt, opts)? else {
                break;
            };
            i
        };
        // 167: a Muster prohibition keeps the troop in the supply where its condition holds.
        if muster {
            if let Some(wh) = super::plan::prohibition_on(&w, g, Step::Muster, spots[i], g.round as i64) {
                prohibit_tally(g, cx, Step::Muster, wh, 1);
                continue;
            }
        }
        if board::walled(&w, g, spots[i]) {
            cx.tally(g, "adversary.walled_landing", 1);
            continue;
        }
        place_troop(g, cx, spots[i]);
        cx.per_round(g, OPS, 1);
        cx.tally(g, tally, 1);
        if let Some(source) = destroy {
            for s in 0..g.seats.len() {
                if g.tokens(spots[i], node(s)) > 0 {
                    board::destroy_piece(g, cx, spots[i], node(s), board::Cause::Effect(source))?;
                }
            }
        }
    }
    Ok(())
}

/// Whether a removal of `ambassadors` or `troops` has a piece to take on hex
/// `h` (F64: a card that names no troop can't touch one).
pub fn can_damage(g: &GameState, h: usize, ambassadors: i64, troops: i64) -> bool {
    (ambassadors > 0 && g.tokens(h, ADV_AMBASSADOR) > 0)
        || (troops > 0 && g.tokens(h, TROOP) > 0)
}

/// Whether a "remove everything on the hex" has a piece to take on `h`.
pub fn can_destroy_all(g: &GameState, h: usize) -> bool {
    can_damage(g, h, 1, 1)
}

/// In hex `h`, removes up to `ambassadors` agents or up to
/// `troops` troops: one kind, seat `seat`'s choice where the hex holds both.
pub fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
    let kinds: Vec<(&str, i64)> = [(ADV_AMBASSADOR, ambassadors), (TROOP, troops)]
        .into_iter()
        .filter(|(k, n)| *n > 0 && g.tokens(h, k) > 0)
        .collect();
    let chosen = match kinds.as_slice() {
        [] => return Ok(()),
        [only] => *only,
        _ => {
            let opts = kinds
                .iter()
                .map(|(k, n)| opt(format!("remove {} {} at {}", (*n).min(g.tokens(h, k)), piece_name(k), hex_name(&w, h))))
                .collect();
            let prompt = || format!("Seat {seat}: remove which pieces?");
            let Some(i) = cx.choose(seat, prompt, opts)? else {
                return Ok(());
            };
            kinds[i]
        }
    };
    for _ in 0..chosen.1.min(g.tokens(h, chosen.0)) {
        remove_piece(g, cx, Some(seat), h, chosen.0);
    }
    Ok(())
}

/// Removes one adversary piece of `kind` from hex `h`. A removed troop
/// sways toward Unity.
pub fn remove_piece(g: &mut GameState, cx: &mut Cx, seat: Option<usize>, h: usize, kind: &str) {
    if kind == TROOP {
        cx.add_tokens(g, h, TROOP, -1);
        cx.tally(g, "adversary.troops_removed", 1);
        alignment::for_troop_removed(g, cx, seat);
    } else {
        remove(g, cx, h, 1);
        cx.tally(g, "adversary.ambassadors_removed", 1);
    }
}

/// Removes every agent and troop in hex `h`.
pub fn destroy_all(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let n = g.tokens(h, ADV_AMBASSADOR);
    for _ in 0..n {
        remove_piece(g, cx, Some(seat), h, ADV_AMBASSADOR);
    }
    for _ in 0..g.tokens(h, TROOP) {
        remove_piece(g, cx, Some(seat), h, TROOP);
    }
}

/// Removes one agent in country `c` (players choose the hex).
pub fn pushback(g: &mut GameState, cx: &mut Cx, c: usize) -> Res {
    let w = cx.world;
    let hexes: Vec<usize> = w.map.countries[c]
        .hexes
        .iter()
        .copied()
        .filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 0)
        .collect();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    if let Some(i) = cx.choose(
        0,
        &format!("Unity: remove an agent in {} from which hex?", country_name(&w, c)),
        opts,
    )? {
        remove_piece(g, cx, None, hexes[i], ADV_AMBASSADOR);
    }
    Ok(())
}
