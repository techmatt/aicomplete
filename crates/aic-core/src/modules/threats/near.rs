//! Where a threat sits (135): its location tokens, where they land when it is
//! dealt, and who is near it.

use super::*;

/// The most location tokens a threat may carry (content prints 1 or 2).
const MAX_TOKENS: usize = 4;

/// The var holding the hex (plus 1) of `threat`'s token for its `i`th location.
pub(super) fn loc_key(threat: &str, i: usize) -> KeyBuf {
    KeyBuf::new("threats.loc.").text(threat).text(".").num(i)
}

/// The hexes of `t`'s location tokens, in its locations' order (a location
/// with no site in its country has none).
pub fn tokens(g: &GameState, t: &str) -> Vec<usize> {
    token_hexes(g, t).collect()
}

/// [`tokens`] with no allocation.
fn token_hexes<'a>(g: &'a GameState, t: &'a str) -> impl Iterator<Item = usize> + 'a {
    (0..MAX_TOKENS).filter_map(move |i| {
        let v = g.var(&loc_key(t, i));
        (v > 0).then(|| v as usize - 1)
    })
}

/// The hexes of `t`'s country: where a threat with no token (a forced pull, a
/// content bug) is measured from and reaches.
fn scope_hexes<'a>(w: &'a World, g: &GameState, t: &str) -> &'a [usize] {
    scope_of(g, t).map(|c| w.map.countries[c].hexes.as_slice()).unwrap_or(&[])
}

/// The hex of `t`'s token of location type `loc`, if it has one.
pub fn token_of(w: &World, g: &GameState, t: &Key, loc: Location) -> Option<usize> {
    def(w, t).locations.iter().enumerate().filter(|(_, l)| **l == loc).find_map(|(i, _)| {
        let v = g.var(&loc_key(t, i));
        (v > 0).then(|| v as usize - 1)
    })
}

/// The sites in country `c` for a token of type `loc`. 178 (Matt's ruling #8): one condition for every type, a
/// site of the type that is not the adversary's (a base never is); the factory type's free-zone fallback is gone.
fn sites(w: &World, g: &GameState, c: usize, loc: Location) -> Vec<usize> {
    let of_type = |h: usize| match loc {
        Location::City => is_city(w, h),
        Location::CoastalCity => is_city(w, h) && w.map.hexes[h].coastal,
        Location::Station => g.tokens(h, STATION) > 0,
        Location::Factory => g.tokens(h, FACTORY) > 0,
        Location::Base => is_base(w, h),
    };
    w.map.countries[c].hexes.iter().copied().filter(|&h| of_type(h) && !corrupted(g, h)).collect()
}

/// Whether country `c` has a site for at least one of `t`'s location types.
pub(super) fn lands_in(w: &World, g: &GameState, t: &ThreatDef, c: usize) -> bool {
    t.locations.iter().any(|&l| !sites(w, g, c, l).is_empty())
}

/// Puts `threat`'s tokens in country `c`, in printed order: r80 (ruling #46) each on the site of its type (not the
/// adversary's) with the most agents on its own hex, ties the lowest number; two tokens never share a hex (a token
/// with no site left stays off). It counted adversary pieces on and next to each site, with a reorder for a token
/// left off (145).
pub(super) fn place_tokens(g: &mut GameState, cx: &mut Cx, threat: &Key, c: usize) {
    let w = cx.world;
    let locs: Vec<(usize, _)> = def(&w, threat).locations.iter().copied().enumerate().take(MAX_TOKENS).collect();
    let mut used: Vec<usize> = Vec::new();
    for (i, loc) in locs {
        let best = sites(&w, g, c, loc)
            .into_iter()
            .filter(|h| !used.contains(h))
            .max_by_key(|&h| (g.tokens(h, ADV_AMBASSADOR), std::cmp::Reverse(w.map.hexes[h].number)));
        if let Some(h) = best {
            used.push(h);
            cx.set_key(g, &loc_key(threat, i), h as i64 + 1);
        }
    }
}

/// 139: moves `threat`'s first location token to hex `h`; a second token already there moves off the board.
pub(super) fn place_first_at(g: &mut GameState, cx: &mut Cx, threat: &Key, h: usize) {
    for i in 1..MAX_TOKENS {
        let k = loc_key(threat, i);
        if g.var(&k) == h as i64 + 1 {
            cx.set_key(g, &k, 0);
        }
    }
    cx.set_key(g, &loc_key(threat, 0), h as i64 + 1);
}

/// Takes `threat`'s tokens off the board.
pub(super) fn clear_tokens(g: &mut GameState, cx: &mut Cx, threat: &Key) {
    for i in 0..MAX_TOKENS {
        let k = loc_key(threat, i);
        if g.var(&k) != 0 {
            cx.set_key(g, &k, 0);
        }
    }
}

/// 148: the hexes `t` reaches as a bit mask, bit `h` for hex `h`, built with no
/// allocation: within 1 of any token, or its whole country with none (a forced
/// pull, a content bug). `Map::build` holds a board to `map::MAX_HEXES` (128).
/// The eval asks "is seat s near t" of every threat for every seat.
fn near_mask(w: &World, g: &GameState, t: &str) -> u128 {
    let mut mask = 0u128;
    for h in token_hexes(g, t) {
        mask |= 1u128 << h;
        for &n in &w.map.hexes[h].neighbors {
            mask |= 1u128 << n;
        }
    }
    if mask == 0 {
        for &h in scope_hexes(w, g, t) {
            mask |= 1u128 << h;
        }
    }
    mask
}

/// 148: whether [`near_mask`] reads the same hexes as a plain computation from
/// [`origins`] (the test's check).
#[cfg(test)]
pub fn mask_matches_reach(w: &World, g: &GameState, t: &str) -> bool {
    let mut plain: Vec<usize> = if tokens(g, t).is_empty() {
        origins(w, g, t)
    } else {
        origins(w, g, t).into_iter().flat_map(|h| within(w, h, 1)).collect()
    };
    plain.sort_unstable();
    plain.dedup();
    mask_hexes(near_mask(w, g, t)).collect::<Vec<_>>() == plain
}

/// The hexes of `mask`, ascending.
fn mask_hexes(mut mask: u128) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        if mask == 0 {
            return None;
        }
        let h = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        Some(h)
    })
}

/// Whether seat `s` is near `t`: a node or a player ambassador of its within
/// 1 hex of any of `t`'s tokens.
pub fn near(w: &World, g: &GameState, s: usize, t: &str) -> bool {
    seat_in_mask(g, s, near_mask(w, g, t))
}

/// Whether a node or a player ambassador of seat `s` sits on a hex of `mask`.
fn seat_in_mask(g: &GameState, s: usize, mask: u128) -> bool {
    let (n, a) = (node(s), ambassador(s));
    mask_hexes(mask).any(|h| g.tokens(h, n) > 0 || g.tokens(h, a) > 0)
}

/// The seats near `t`, in seat order: who may pay, and who its effects hit.
pub fn near_seats(w: &World, g: &GameState, t: &str) -> Vec<usize> {
    // 147: the reach read once for every seat (it was once a seat). 148: as a mask.
    let m = near_mask(w, g, t);
    (0..g.seats.len()).filter(|&s| seat_in_mask(g, s, m)).collect()
}

/// 146: the hexes an uncountered `t`'s harm is measured from: its tokens, or
/// its country with none (a forced pull, a content bug).
fn origins(w: &World, g: &GameState, t: &str) -> Vec<usize> {
    let tk = tokens(g, t);
    if tk.is_empty() {
        return scope_hexes(w, g, t).to_vec();
    }
    tk
}

/// 146: seat `s`'s hex distance from `t`: its node closest to any of `t`'s
/// tokens. `None` with no node on the board.
pub fn node_distance(w: &World, g: &GameState, s: usize, t: &str) -> Option<i32> {
    let from = origins(w, g, t);
    let n = node(s);
    (0..g.hexes.len())
        .filter(|&h| g.tokens(h, n) > 0)
        .filter_map(|h| from.iter().map(|&o| w.map.distance(o, h)).min())
        .min()
}

/// 146: the seats an uncountered `t`'s harm may hit: the ones whose node is
/// closest to one of its tokens (hex distance), in seat order. Several on a
/// tie (the players pick one); none if no seat has a node.
pub fn nearest_seats(w: &World, g: &GameState, t: &str) -> Vec<usize> {
    let d: Vec<Option<i32>> = (0..g.seats.len()).map(|s| node_distance(w, g, s, t)).collect();
    let Some(best) = d.iter().flatten().min().copied() else {
        return Vec::new();
    };
    (0..g.seats.len()).filter(|&s| d[s] == Some(best)).collect()
}

/// 146: seat `s`'s node hexes closest to `t`'s tokens: where "destroy a node"
/// lands, wherever that node is.
pub fn nearest_nodes(w: &World, g: &GameState, s: usize, t: &str) -> Vec<usize> {
    let Some(best) = node_distance(w, g, s, t) else {
        return Vec::new();
    };
    let from = origins(w, g, t);
    let n = node(s);
    (0..g.hexes.len())
        .filter(|&h| g.tokens(h, n) > 0 && from.iter().any(|&o| w.map.distance(o, h) == best))
        .collect()
}

/// The land hexes within 1 of any of `t`'s tokens, sorted: its effects' "near it".
pub fn near_hexes(w: &World, g: &GameState, t: &str) -> Vec<usize> {
    mask_hexes(near_mask(w, g, t)).filter(|&h| !is_ocean(w, h)).collect()
}

/// Seat `s`'s node hexes on or next to one of `t`'s tokens (a `nodes_near` counter).
pub(super) fn nodes_near(w: &World, g: &GameState, s: usize, t: &str) -> Vec<usize> {
    let n = node(s);
    mask_hexes(near_mask(w, g, t)).filter(|&h| g.tokens(h, n) > 0).collect()
}

/// Whether `threat` carries `source`'s tag, or Universal (a typed Jam or Auto Counter).
pub(super) fn of_type(w: &World, threat: &str, source: ThreatSource) -> bool {
    let tag = ThreatTag::from_source(source);
    def(w, threat).tags.iter().any(|&t| t == tag || t == ThreatTag::Universal)
}

/// The active threats seat `s` is near, carrying `source`'s tag (or
/// Universal) if given.
pub fn counterable(w: &World, g: &GameState, s: usize, source: Option<ThreatSource>) -> Vec<Key> {
    active(g)
        .into_iter()
        .filter(|(t, _)| source.is_none_or(|x| of_type(w, t, x)) && near(w, g, s, t))
        .map(|(t, _)| t)
        .collect()
}
