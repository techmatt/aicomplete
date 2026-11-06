//! What an op can act on: the one home of each target filter, for `feasible` and `one`.

use crate::key::Key;

use super::*;

/// Hexes a removal effect of seat `s` may target: every card's map target
/// is within the seat's reach (016).
pub fn damage_hexes(w: &World, g: &GameState, seat: usize, ambassadors: i64, troops: i64) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| adversary::can_damage(g, h, ambassadors, troops) && in_reach(w, g, seat, h))
        .collect()
}

/// A corrupted research station or factory, or with `cities` (024: tier-2
/// ultimates only) a corrupted city: a cleanse target. `only` (r35) limits it
/// to one facility kind.
pub(super) fn cleansable(w: &World, g: &GameState, h: usize, cities: bool, only: Option<Facility>) -> bool {
    let stations = only != Some(Facility::Factory);
    let factories = only != Some(Facility::Station);
    corrupted(g, h)
        && ((cities && is_city(w, h)) || (stations && g.tokens(h, STATION) > 0) || (factories && g.tokens(h, FACTORY) > 0))
}

/// Corrupted cities in seat `s`'s reach that a liberate card can take (080).
pub fn liberable_cities(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| corrupted_city(w, g, h) && in_reach(w, g, seat, h))
        .collect()
}

/// Corrupted cities in seat `s`'s reach with an adversary ambassador on or
/// next to them (a `clear_city` target, 024).
pub(super) fn guarded_cities(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| {
            corrupted_city(w, g, h) && in_reach(w, g, seat, h) && board::guarded(w, g, h)
        })
        .collect()
}

/// Countries whose incomplete megaproject seat `s` can add a cube to (075):
/// with `station`, only where it owns a station.
pub fn cube_projects(w: &World, g: &GameState, seat: usize, station: bool) -> Vec<usize> {
    let mine = |c: usize| {
        w.map.countries[c].hexes.iter().any(|&h| g.tokens(h, STATION) > 0 && !corrupted(g, h) && g.tokens(h, cube(seat)) > 0)
    };
    (0..w.map.countries.len())
        .filter(|&c| g.var(&board::project_done(c)) == 0 && (!station || mine(c)))
        .collect()
}

/// Free industrial zones in seat `s`'s reach (a card's factory site).
pub(super) fn factory_sites(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    free_zones(w, g)
        .into_iter()
        .filter(|&h| in_reach(w, g, seat, h))
        .collect()
}

pub(super) fn countries_where(w: &World, f: impl Fn(usize) -> bool) -> Vec<usize> {
    (0..w.map.countries.len())
        .filter(|&c| w.map.countries[c].hexes.iter().any(|&h| f(h)))
        .collect()
}

pub(super) fn any_hex(g: &GameState, f: impl Fn(usize) -> bool) -> bool {
    (0..g.hexes.len()).any(f)
}

/// Ambassadors seat `s` could move: in its supply or on the board.
pub(super) fn movable_ambassadors(g: &GameState, seat: usize) -> i64 {
    g.resource(seat, Resource::Ambassadors) + board::ambassadors_on_board(g, seat)
}

/// A botnet in seat `seat`'s reach.
pub(super) fn botnet_in_reach(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    g.tokens(h, BOTNET) > 0 && in_reach(w, g, seat, h)
}

/// A corruption marker in seat `seat`'s reach.
pub(super) fn marker_in_reach(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    g.tokens(h, MARKER) > 0 && in_reach(w, g, seat, h)
}

/// Seat `seat`'s node with no hardening token on it.
pub fn shieldable(g: &GameState, seat: usize, h: usize) -> bool {
    g.tokens(h, node(seat)) > 0 && g.tokens(h, HARDEN) == 0
}

/// A piece an ambassador on hex `a` can move to with `range`.
pub(super) fn move_one_dest(w: &World, g: &GameState, a: usize, range: i64, h: usize) -> bool {
    h != a && has_piece(w, g, h) && w.map.distance(a, h) as i64 <= range
}

/// Pieces in seat `seat`'s reach: where its ambassadors can be moved.
pub(super) fn pieces_in_reach(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| in_reach(w, g, seat, h) && has_piece(w, g, h))
        .collect()
}

/// Active threats seat `s` may counter, not already delayed this round (118:
/// "a threat" on a card is one you may counter, the house term; a delay reached
/// every face-up threat).
pub(super) fn undelayed_threats(w: &World, g: &GameState, s: usize) -> Vec<(Key, Option<usize>)> {
    let mut active = threats::active(g);
    active.retain(|(t, scope)| !threats::waits(g, t) && threats::may_counter(w, g, s, t, *scope));
    active
}

/// Botnets in seat `seat`'s reach not already cut off (044: a second cut did nothing).
pub(super) fn cuttable_botnets(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| g.tokens(h, BOTNET) > 0 && g.var(&adversary::cut_key(h)) == 0 && in_reach(w, g, seat, h))
        .collect()
}

/// Countries with no Shelter up (044: a second one did nothing).
pub(super) fn unsheltered(w: &World, g: &GameState) -> Vec<usize> {
    (0..w.map.countries.len())
        .filter(|&c| !board::flagged(g, RoundFlag::Shelter, c))
        .collect()
}

/// Buildings in reach of seat `seat` a Harden's token can protect.
pub(super) fn harden_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len()).filter(|&h| board::hardenable(g, h) && in_reach(w, g, seat, h)).collect()
}

/// Seat `seat`'s ambassadors that have a piece within `range` to move to.
pub(super) fn movable_one(w: &World, g: &GameState, seat: usize, range: i64) -> Vec<usize> {
    ambassador_hexes(g, seat)
        .into_iter()
        .filter(|&a| any_hex(g, |h| move_one_dest(w, g, a, range, h)))
        .collect()
}
