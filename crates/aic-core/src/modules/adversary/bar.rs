//! The win's count (what still stands between the players and the win) and the board loss.

use super::*;

/// The corrupted cities on the board: the players win when there is none (r141, Matt 2026-11-19: the purge's
/// steps are gone; botnets, troops and agents never count).
pub fn purge_count(w: &World, g: &GameState) -> i64 {
    (0..g.hexes.len()).filter(|&h| corrupted_city(w, g, h)).count() as i64
}

/// Whether no city is corrupted, stopping at the first: every action asks.
pub fn under_bar(w: &World, g: &GameState) -> bool {
    !(0..g.hexes.len()).any(|h| corrupted_city(w, g, h))
}

/// A board loss, if any: a player with no nodes on the board (locked,
/// 027). (The deck running out ends the game in the build step, and the
/// Discord deck running out in the tip.)
pub fn loss(g: &GameState) -> Option<(&'static str, String)> {
    let gone = (0..g.seats.len()).find(|&s| !has_node(g, s))?;
    Some(("node", format!("seat {gone} has no nodes left on the board")))
}
