//! The purge bar (what still counts against the players' win) and the board loss.

use super::*;

/// How much the purge still asks (136): each step of `alignment.purge_at`
/// reached drops one kind of adversary piece. Corrupted cities always count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PurgeLevel {
    /// Every adversary piece: corrupted cities, botnets, troops, ambassadors.
    All,
    /// Ambassadors no longer count.
    NoAmbassadors,
    /// Ambassadors and troops no longer count.
    NoTroops,
    /// Only corrupted cities count.
    CitiesOnly,
}

impl PurgeLevel {
    /// The level after `steps` thresholds reached (three or more: cities only).
    pub fn from_steps(steps: i64) -> PurgeLevel {
        match steps {
            ..=0 => PurgeLevel::All,
            1 => PurgeLevel::NoAmbassadors,
            2 => PurgeLevel::NoTroops,
            _ => PurgeLevel::CitiesOnly,
        }
    }

    /// The steps reached, 0-3 (the view's bar, the agents' eval term).
    pub fn steps(self) -> i64 {
        self as i64
    }

    pub fn counts_ambassadors(self) -> bool {
        self < PurgeLevel::NoAmbassadors
    }

    pub fn counts_troops(self) -> bool {
        self < PurgeLevel::NoTroops
    }

    pub fn counts_botnets(self) -> bool {
        self < PurgeLevel::CitiesOnly
    }

    /// The win line for this level.
    pub fn win_text(self) -> &'static str {
        match self {
            PurgeLevel::All => "purge: no corrupted city, botnet, adversary troop or agent on the board",
            PurgeLevel::NoAmbassadors => {
                "purge: no corrupted city, botnet or adversary troop (agents no longer count)"
            }
            PurgeLevel::NoTroops => "purge: no corrupted city or botnet (agents and troops no longer count)",
            PurgeLevel::CitiesOnly => "purge: no corrupted city (agents, troops and botnets no longer count)",
        }
    }
}

/// What still counts against the purge at `level` (019: botnets until the
/// last step).
pub fn purge_count(w: &World, g: &GameState, level: PurgeLevel) -> i64 {
    (0..g.hexes.len()).map(|h| counts_against(w, g, h, level)).sum()
}

/// What hex `h` adds to [`purge_count`].
fn counts_against(w: &World, g: &GameState, h: usize, level: PurgeLevel) -> i64 {
    against_bar(corrupted_city(w, g, h), g.tokens(h, BOTNET), g.tokens(h, ADV_AMBASSADOR), g.tokens(h, TROOP), level)
}

/// What a hex with these pieces adds to [`purge_count`] at `level`.
pub fn against_bar(corrupted_city: bool, botnets: i64, ambassadors: i64, troops: i64, level: PurgeLevel) -> i64 {
    let mut n = corrupted_city as i64;
    if level.counts_botnets() {
        n += botnets;
    }
    if level.counts_ambassadors() {
        n += ambassadors;
    }
    if level.counts_troops() {
        n += troops;
    }
    n
}

/// Whether [`purge_count`] is 0, stopping at the first piece: every action asks.
pub fn under_bar(w: &World, g: &GameState, level: PurgeLevel) -> bool {
    !(0..g.hexes.len()).any(|h| counts_against(w, g, h, level) > 0)
}

/// A board loss, if any: a player with no nodes on the board (locked,
/// 027). (The deck running out ends the game in the build step, and the
/// Discord deck running out in the tip.)
pub fn loss(g: &GameState) -> Option<(&'static str, String)> {
    let gone = (0..g.seats.len()).find(|&s| !has_node(g, s))?;
    Some(("node", format!("seat {gone} has no nodes left on the board")))
}
