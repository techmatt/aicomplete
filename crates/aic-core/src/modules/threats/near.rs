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
pub fn tokens(g: &GameState, t: &Key) -> Vec<usize> {
    tokens_of(g, t)
}

pub(super) fn tokens_of(g: &GameState, t: &str) -> Vec<usize> {
    (0..MAX_TOKENS)
        .filter_map(|i| {
            let v = g.var(&loc_key(t, i));
            (v > 0).then(|| v as usize - 1)
        })
        .collect()
}

/// The hex of `t`'s token of location type `loc`, if it has one.
pub fn token_of(w: &World, g: &GameState, t: &Key, loc: Location) -> Option<usize> {
    def(w, t).locations.iter().enumerate().filter(|(_, l)| **l == loc).find_map(|(i, _)| {
        let v = g.var(&loc_key(t, i));
        (v > 0).then(|| v as usize - 1)
    })
}

/// Whether hex `h` in country `c` is a site for a token of type `loc`, with
/// the factory fallback: a free industrial zone counts only when `c` has no
/// factory that isn't the adversary's.
fn sites(w: &World, g: &GameState, c: usize, loc: Location) -> Vec<usize> {
    let hexes = &w.map.countries[c].hexes;
    let pick = |f: &dyn Fn(usize) -> bool| -> Vec<usize> { hexes.iter().copied().filter(|&h| f(h)).collect() };
    match loc {
        Location::City => pick(&|h| is_city(w, h) && !corrupted(g, h)),
        Location::CoastalCity => pick(&|h| is_city(w, h) && w.map.hexes[h].coastal && !corrupted(g, h)),
        Location::Station => pick(&|h| g.tokens(h, STATION) > 0 && !corrupted(g, h)),
        Location::Factory => {
            let factories = pick(&|h| g.tokens(h, FACTORY) > 0 && !corrupted(g, h));
            if factories.is_empty() {
                pick(&|h| w.map.hexes[h].industrial && !has_building(g, h))
            } else {
                factories
            }
        }
        Location::Base => pick(&|h| is_base(w, h)),
    }
}

/// Adversary pieces (ambassadors, troops, botnets) on or next to hex `h`.
fn pressure(w: &World, g: &GameState, h: usize) -> i64 {
    within(w, h, 1)
        .into_iter()
        .map(|x| g.tokens(x, ADV_AMBASSADOR) + g.tokens(x, TROOP) + g.tokens(x, BOTNET))
        .sum()
}

/// Whether country `c` has a site for at least one of `t`'s location types.
pub(super) fn lands_in(w: &World, g: &GameState, t: &ThreatDef, c: usize) -> bool {
    t.locations.iter().any(|&l| !sites(w, g, c, l).is_empty())
}

/// Puts `threat`'s tokens in country `c`: each on the site of its type with
/// the most adversary pieces on or next to it, ties the lower hex; two tokens
/// never share a hex.
pub(super) fn place_tokens(g: &mut GameState, cx: &mut Cx, threat: &Key, c: usize) {
    let w = cx.world;
    let mut used: Vec<usize> = Vec::new();
    for (i, &loc) in def(&w, threat).locations.iter().enumerate().take(MAX_TOKENS) {
        let best = sites(&w, g, c, loc)
            .into_iter()
            .filter(|h| !used.contains(h))
            .max_by_key(|&h| (pressure(&w, g, h), std::cmp::Reverse(h)));
        if let Some(h) = best {
            cx.set_key(g, &loc_key(threat, i), h as i64 + 1);
            used.push(h);
        }
    }
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

/// The hexes `t` reaches: within 1 of any token. A threat with no token (a
/// forced pull, a content bug) reaches its whole country.
fn reach(w: &World, g: &GameState, t: &str) -> Vec<usize> {
    let tk = tokens_of(g, t);
    if tk.is_empty() {
        return scope_of(g, t).map(|c| w.map.countries[c].hexes.clone()).unwrap_or_default();
    }
    let mut hexes: Vec<usize> = tk.iter().flat_map(|&h| within(w, h, 1)).collect();
    hexes.sort_unstable();
    hexes.dedup();
    hexes
}

/// Whether seat `s` is near `t`: a node or a player ambassador of its within
/// 1 hex of any of `t`'s tokens.
pub fn near(w: &World, g: &GameState, s: usize, t: &Key) -> bool {
    is_near(w, g, s, t)
}

pub(super) fn is_near(w: &World, g: &GameState, s: usize, t: &str) -> bool {
    let (n, a) = (node(s), ambassador(s));
    reach(w, g, t).into_iter().any(|h| g.tokens(h, n) > 0 || g.tokens(h, a) > 0)
}

/// The seats near `t`, in seat order: who may pay, and who its effects hit.
pub fn near_seats(w: &World, g: &GameState, t: &Key) -> Vec<usize> {
    seats_near(w, g, t)
}

pub(super) fn seats_near(w: &World, g: &GameState, t: &str) -> Vec<usize> {
    (0..g.seats.len()).filter(|&s| is_near(w, g, s, t)).collect()
}

/// The land hexes within 1 of any of `t`'s tokens, sorted: its effects' "near it".
pub fn near_hexes(w: &World, g: &GameState, t: &Key) -> Vec<usize> {
    reach(w, g, t).into_iter().filter(|&h| !is_ocean(w, h)).collect()
}

/// Seat `s`'s node hexes on or next to one of `t`'s tokens (a `nodes_near` counter).
pub(super) fn nodes_near(w: &World, g: &GameState, s: usize, t: &str) -> Vec<usize> {
    let n = node(s);
    reach(w, g, t).into_iter().filter(|&h| g.tokens(h, n) > 0).collect()
}

/// Whether seat `s` may counter `threat` (135: whether it is near it; `scope`
/// is no longer read).
pub fn may_counter(w: &World, g: &GameState, s: usize, threat: &str, _scope: Option<usize>) -> bool {
    is_near(w, g, s, threat)
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
