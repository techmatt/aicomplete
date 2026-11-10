//! Shared vocabulary of the ruleset: piece names, board queries and choice
//! options. Rules live in the modules; this only answers questions about
//! the board.
//!
//! Terms used throughout (see `docs/rules/README.md`):
//! - **adjacent**: a neighboring hex on the board (across country borders too);
//! - **next to X**: holding X, or adjacent to a hex holding X;
//! - **reach**: a seat's reach is every hex within 1 of one of its nodes.
//! - **node**: a player's tokens on the board (one per hex), placed off
//!   their AI board's node track.

use crate::content::HexKind;
use crate::module::World;
use crate::state::{GameState, Opt};
pub use crate::vocab::{Resource, ThreatSource};

pub const FACTORY: &str = "factory";
pub const STATION: &str = "station";
/// The adversary's owner cube (on a city, research station or factory): corrupted.
pub const ADV_CUBE: &str = "cube:adversary";
pub const ADV_AMBASSADOR: &str = "ambassador:adversary";
/// An adversary troop: marches in the adversary phase (019). A strike uses it
/// up by `troop_spent`.
pub const TROOP: &str = "troop";
/// Corp units waiting at a military base (the base's country's corp).
pub const UNIT: &str = "unit";
/// A hardening token: absorbs one destruction of a facility or node on its hex.
pub const HARDEN: &str = "harden";
/// A botnet: persistent, makes adversary compute, blocks claims next to it.
pub const BOTNET: &str = "botnet";
/// A corruption marker (034): what a destroyed node leaves on its hex.
pub const MARKER: &str = "corruption";
/// Land corruption (138): a rare hex token a Corrupt land line leaves on an
/// empty land hex. No node may be placed there, and it drips Discord.
pub const LAND_CORRUPTION: &str = "land_corruption";
/// A firewall (158): no agent arrives on its hex; it comes off at
/// the end of the next adversary turn.
pub const FIREWALL: &str = "firewall";

/// Whether hex `h` holds a firewall (158).
pub fn firewalled(g: &GameState, h: usize) -> bool {
    g.tokens(h, FIREWALL) > 0
}

/// Seat keys are tables, not `format!`: the search asks for them on every hex
/// of every position it scores (028: a tenth of the engine's time). A game has
/// at most 4 countries, so at most 4 seats.
const NODE: [&str; 4] = ["node:0", "node:1", "node:2", "node:3"];
const CUBE: [&str; 4] = ["cube:0", "cube:1", "cube:2", "cube:3"];
const AMBASSADOR: [&str; 4] = ["ambassador:0", "ambassador:1", "ambassador:2", "ambassador:3"];

/// Seat `s`'s node on a hex.
pub fn node(seat: usize) -> &'static str {
    NODE[seat]
}

/// Seat `s`'s owner cube (an aligned city or an owned station or factory).
pub fn cube(seat: usize) -> &'static str {
    CUBE[seat]
}

/// Seat `s`'s player ambassador on a piece.
pub fn ambassador(seat: usize) -> &'static str {
    AMBASSADOR[seat]
}

pub fn is_city(w: &World, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::City
}

pub fn is_ocean(w: &World, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Ocean
}

pub fn is_base(w: &World, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Base
}

pub fn corrupted(g: &GameState, h: usize) -> bool {
    g.tokens(h, ADV_CUBE) > 0
}

/// The seat whose cube is on hex `h`, if any.
pub fn cube_owner(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, cube(s)) > 0)
}

/// A city with the adversary's cube.
pub fn corrupted_city(w: &World, g: &GameState, h: usize) -> bool {
    is_city(w, h) && corrupted(g, h)
}

/// A city with no cube.
pub fn clean_city(w: &World, g: &GameState, h: usize) -> bool {
    is_city(w, h) && !corrupted(g, h) && cube_owner(g, h).is_none()
}

/// The seat whose node is on hex `h`, if any.
pub fn node_owner(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, node(s)) > 0)
}

/// Whether hex `h` holds a building (factory, research station).
pub fn has_building(g: &GameState, h: usize) -> bool {
    g.tokens(h, FACTORY) > 0 || g.tokens(h, STATION) > 0
}

/// An open hex with no building: where buildings go.
pub fn buildable(w: &World, g: &GameState, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Open && !has_building(g, h)
}

/// Whether hex `h` holds a piece an ambassador can sit on (node included).
pub fn has_piece(w: &World, g: &GameState, h: usize) -> bool {
    is_city(w, h) || is_base(w, h) || has_building(g, h) || node_owner(g, h).is_some()
}

/// A player ambassador's seat on hex `h`, if any (seat order).
pub fn player_ambassador(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, ambassador(s)) > 0)
}

/// Hex `h`, and with `range` 1 its neighbors.
pub fn within(w: &World, h: usize, range: i64) -> Vec<usize> {
    let mut hexes = vec![h];
    if range > 0 {
        hexes.extend(w.map.hexes[h].neighbors.iter().copied());
    }
    hexes
}

/// Whether hex `h` holds, or is adjacent to a hex holding, something `pred` accepts.
pub fn next_to(w: &World, h: usize, pred: impl Fn(usize) -> bool) -> bool {
    pred(h) || w.map.hexes[h].neighbors.iter().any(|&n| pred(n))
}

/// Hexes of country `c`, or of the whole board for `None`, in board order.
pub fn scope_hexes(w: &World, scope: Option<usize>) -> Vec<usize> {
    match scope {
        Some(c) => w.map.countries[c].hexes.clone(),
        None => (0..w.map.hexes.len()).collect(),
    }
}

pub fn scope_countries(w: &World, scope: Option<usize>) -> Vec<usize> {
    match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    }
}

/// Free industrial zones: printed zones holding no building, where a factory
/// may be built.
pub fn free_zones(w: &World, g: &GameState) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| w.map.hexes[h].industrial && !has_building(g, h))
        .collect()
}

/// The country seat `s` starts in (one seat per country, wrapping).
pub fn home_country(w: &World, seat: usize) -> usize {
    seat % w.map.countries.len()
}

/// Hexes holding seat `s`'s node, in board order (the tests' reading).
#[cfg(test)]
pub fn node_of(g: &GameState, seat: usize) -> Vec<usize> {
    let k = node(seat);
    (0..g.hexes.len())
        .filter(|&h| g.tokens(h, k) > 0)
        .collect()
}

/// Hexes holding seat `s`'s ambassadors on the board, in hex order.
pub fn ambassador_hexes(g: &GameState, seat: usize) -> Vec<usize> {
    let k = ambassador(seat);
    (0..g.hexes.len()).filter(|&h| g.tokens(h, k) > 0).collect()
}

pub fn node_count(g: &GameState, seat: usize) -> i64 {
    let k = node(seat);
    (0..g.hexes.len()).filter(|&h| g.tokens(h, k) > 0).count() as i64
}

/// Whether seat `s` has a node on the board: [`node_count`] above 0,
/// stopping at the first.
pub fn has_node(g: &GameState, seat: usize) -> bool {
    let k = node(seat);
    (0..g.hexes.len()).any(|h| g.tokens(h, k) > 0)
}

/// Whether hex `h` is within seat `s`'s reach (1 of one of its nodes).
pub fn in_reach(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    let k = node(seat);
    next_to(w, h, |x| g.tokens(x, k) > 0)
}

pub fn has_node_in(w: &World, g: &GameState, seat: usize, country: usize) -> bool {
    let k = node(seat);
    w.map.countries[country]
        .hexes
        .iter()
        .any(|&h| g.tokens(h, k) > 0)
}

/// Whether `seat` has a node on a city hex of `country` (any city, owned or not): r37's
/// converter gate.
pub fn has_city_node_in(w: &World, g: &GameState, seat: usize, country: usize) -> bool {
    let k = node(seat);
    w.map.countries[country]
        .hexes
        .iter()
        .any(|&h| is_city(w, h) && g.tokens(h, k) > 0)
}

/// Seats a threat in `scope` concerns: with a node in the country,
/// or every seat for the whole board.
pub fn seats_in_scope(w: &World, g: &GameState, scope: Option<usize>) -> Vec<usize> {
    match scope {
        Some(c) => (0..g.seats.len()).filter(|&s| has_node_in(w, g, s, c)).collect(),
        None => (0..g.seats.len()).collect(),
    }
}

/// A piece's token key in words, for prompts, options and notes: `node:2`
/// is "seat 2's node", `station` "research station".
pub fn piece_name(kind: &str) -> String {
    match kind.split_once(':') {
        Some(("node", s)) => format!("seat {s}'s node"),
        // 166 (Matt, 2026-11-09): the agents print as agents.
        Some(("ambassador", "adversary")) => "agent".into(),
        Some((kind, "adversary")) => format!("adversary {kind}"),
        _ if kind == STATION => "research station".into(),
        _ => kind.replace('_', " "),
    }
}

/// "Cintamani #5": the printed piece's name and hex number (020: "C1 #5",
/// a country index the board never shows).
pub fn hex_name(w: &World, h: usize) -> String {
    let c = &w.map.hexes[h];
    // One allocation, no formatter: every hex option's label is one of these,
    // rebuilt on each replay of a prompt (103).
    match w.content.map.countries.get(c.country as usize) {
        Some(p) => {
            let mut s = String::with_capacity(p.name.len() + 4);
            s.push_str(&p.name);
            s.push_str(" #");
            s.push_str(crate::key::KeyBuf::new("").num(c.number as usize).as_str());
            s
        }
        None => format!("{} #{}", country_name(w, c.country as usize), c.number),
    }
}

/// The printed name of country `c`'s piece ("country 1" if the map has none).
pub fn country_name(w: &World, c: usize) -> String {
    w.content
        .map
        .countries
        .get(c)
        .map_or_else(|| format!("country {c}"), |p| p.name.clone())
}

/// An option naming a hex (the viewer offers it as a hex pick).
pub fn hex_opt(w: &World, h: usize) -> Opt {
    Opt {
        option: hex_name(w, h),
        target: Some(w.map.hexes[h].hex),
    }
}

/// [`hex_opt`] with its label led by `what`: "node at Cintamani #5".
pub fn labeled_hex_opt(w: &World, what: &str, h: usize) -> Opt {
    let mut o = hex_opt(w, h);
    o.option = format!("{what} at {}", o.option);
    o
}

pub fn opt(label: impl Into<String>) -> Opt {
    Opt {
        option: label.into(),
        target: None,
    }
}

/// Seat `s` as an option label.
pub fn seat_opt(s: usize) -> Opt {
    opt(format!("seat {s}"))
}

pub fn country_opt(w: &World, c: usize) -> Opt {
    opt(country_name(w, c))
}
