//! What an op can act on: the one home of each target filter, for `feasible` and `one`.

use super::*;

/// Hexes a removal effect of seat `s` may target: every card's map target
/// is within the seat's reach (016).
pub fn damage_hexes(w: &World, g: &GameState, seat: usize, ambassadors: i64, troops: i64) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| adversary::can_damage(w, g, h, ambassadors, troops) && in_reach(w, g, seat, h))
        .collect()
}

/// A corrupted research station or factory, or with `cities` (024: tier-2
/// ultimates only) a corrupted city: a cleanse target.
pub(super) fn cleansable(w: &World, g: &GameState, h: usize, cities: bool) -> bool {
    corrupted(g, h)
        && ((cities && w.map.hexes[h].kind == HexKind::City && !threats::shielded(w, g, h, ThreatSource::Influence))
            || (g.tokens(h, STATION) > 0 && !threats::shielded(w, g, h, ThreatSource::Cyber))
            || (g.tokens(h, FACTORY) > 0 && !threats::shielded(w, g, h, ThreatSource::Economic)))
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

/// Moves one of seat `s`'s ambassadors from hex `from` onto hex `to` (021:
/// movement keywords); it arrives as in rules.md §2.
pub(super) fn walk(g: &mut GameState, cx: &mut Cx, seat: usize, from: usize, to: usize) {
    cx.add_tokens(g, from, &ambassador(seat), -1);
    board::place_ambassador(g, cx, seat, to);
    cx.tally(g, "effects.ambassador_moves", 1);
}

/// A node in seat `seat`'s reach.
pub(super) fn node_in_reach(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    g.tokens(h, NODE) > 0 && in_reach(w, g, seat, h) && !threats::shielded(w, g, h, ThreatSource::Cyber)
}

/// A corruption marker in seat `seat`'s reach.
pub(super) fn marker_in_reach(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    g.tokens(h, MARKER) > 0 && in_reach(w, g, seat, h)
}

/// Seat `seat`'s presence with no hardening token on it.
pub fn shieldable(g: &GameState, seat: usize, h: usize) -> bool {
    g.tokens(h, &presence(seat)) > 0 && g.tokens(h, HARDEN) == 0
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

/// Active threats not already delayed this round.
pub(super) fn undelayed_threats(g: &GameState) -> Vec<(crate::key::Key, Option<usize>)> {
    let mut active = threats::active(g);
    active.retain(|(t, _)| !threats::waits(g, t));
    active
}

/// Asks `seat` to pick among `opts`, "stop" added unless this is the `first`
/// pick of an "up to N". `None`: no pick, or stop.
pub(super) fn pick_or_stop(cx: &mut Cx, seat: usize, prompt: &str, mut opts: Vec<Opt>, first: bool) -> Res<Option<usize>> {
    let n = opts.len();
    if !first {
        opts.push(opt("stop"));
    }
    Ok(cx.choose(seat, prompt, opts)?.filter(|&i| i < n))
}

/// Nodes in seat `seat`'s reach not already cut off (044: a second cut did nothing).
pub(super) fn cuttable_nodes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| g.tokens(h, NODE) > 0 && g.var(&adversary::cut_key(h)) == 0 && in_reach(w, g, seat, h))
        .collect()
}

/// Countries with no Shelter up (044: a second one did nothing).
pub(super) fn unsheltered(w: &World, g: &GameState) -> Vec<usize> {
    (0..w.map.countries.len())
        .filter(|&c| !board::flagged(g, crate::op::RoundFlag::Shelter, c))
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
