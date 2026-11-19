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
/// A botnet: persistent, makes adversary compute, an ambassador on its hex may take it down (r75's standard action).
pub const BOTNET: &str = "botnet";
/// Fallout (138): a rare hex token a Corrupt land line leaves on an
/// empty land hex. No node may be placed there, and it drips Discord.
pub const FALLOUT: &str = "fallout";
/// A firewall (158): no agent arrives on its hex; it comes off at
/// the end of the next adversary turn.
pub const FIREWALL: &str = "firewall";
/// r80 (ruling #53): a prohibition's token, one per step, on each hex its condition held on when the card
/// resolved; it lasts like a firewall (off at the end of the next adversary turn).
pub fn prohibit_token(step: crate::op::Step) -> &'static str {
    match step {
        crate::op::Step::Infiltrate => "no_infiltrate",
        crate::op::Step::Muster => "no_muster",
        crate::op::Step::Corrupt => "no_corrupt",
    }
}

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

/// 254 (Matt's candidate, `board.cities_unowned`): players never own a city.
pub fn cities_unowned(w: &World) -> bool {
    w.int("board.cities_unowned") != 0
}

/// 254 (`board.host_city`): the seat hosting neutral city `h` this round ([`host_assignment`]): its energy goes to
/// that seat in the income step.
pub fn city_host(w: &World, g: &GameState, h: usize) -> Option<usize> {
    host_assignment(w, g).into_iter().find(|&(c, _)| c == h).map(|(_, s)| s)
}

/// 254: the neutral cities seat `seat` hosts this round ([`host_assignment`]).
pub fn hosted_cities(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    host_assignment(w, g).into_iter().filter(|&(_, s)| s == seat).map(|(c, _)| c).collect()
}

/// 254: who hosts which neutral city in this round's income step, as (city, seat). A seat may host a neutral city
/// producing now (no cube, no agent on it, no blackout; 262: a purchase no longer unhosts it, as a hosted city
/// can't be bought, Matt's D107) with its node on it, or
/// with `board.host_reach` 1 on or next to it; each seat hosts `board.host_city` at most and each city has one host.
/// The printed rule: the players choose (a team game; every city makes the same energy), so the engine hosts as many
/// as can be ([`match_hosts`]).
pub fn host_assignment(w: &World, g: &GameState) -> Vec<(usize, usize)> {
    let caps = host_caps(w, g);
    if caps.iter().all(|&c| c == 0) {
        return Vec::new();
    }
    let reach = w.int("board.host_reach") > 0;
    let open: Vec<usize> = (0..g.hexes.len())
        .filter(|&h| {
            clean_city(w, g, h)
                && g.tokens(h, ADV_AMBASSADOR) == 0
                && !super::board::hex_flagged(w, g, crate::content::RoundFlag::Blackout, h)
        })
        .collect();
    let hosts = |s: usize, h: usize| {
        g.tokens(h, node(s)) > 0 || (reach && w.map.hexes[h].neighbors.iter().any(|&x| g.tokens(x, node(s)) > 0))
    };
    match_hosts(&caps, &open, hosts)
}

/// 258: the var holding seat `seat`'s extra host slots (`host_more`, a card's lasting effect).
pub fn host_more_key(seat: usize) -> String {
    format!("board.host_more.{seat}")
}

/// 258: how many neutral cities each seat may host: `board.host_city`, plus the `host_more` its cards raised. With
/// `board.host_city` 0 hosting is off and no card turns it on.
pub fn host_caps(w: &World, g: &GameState) -> Vec<usize> {
    let cap = w.int("board.host_city").max(0) as usize;
    (0..g.seats.len()).map(|s| if cap == 0 { 0 } else { cap + g.var(&host_more_key(s)).max(0) as usize }).collect()
}

/// 254 (diff review: a greedy pick could host fewer than the players would): a largest matching of `cities` to
/// the seats, seat s `caps[s]` at most (258: per seat), `can(seat, city)` the edges (augmenting paths; a few seats
/// and cities). Deterministic: seats in order, each city list in hex order. The agents' eval calls it on its own
/// token view.
pub fn match_hosts(caps: &[usize], cities: &[usize], can: impl Fn(usize, usize) -> bool) -> Vec<(usize, usize)> {
    // Seat slots, seat by seat: slot_seat[slot] is the slot's seat.
    let slot_seat: Vec<usize> = caps.iter().enumerate().flat_map(|(s, &c)| std::iter::repeat_n(s, c)).collect();
    let slots = slot_seat.len();
    let edges: Vec<Vec<usize>> = (0..slots).map(|slot| (0..cities.len()).filter(|&c| can(slot_seat[slot], cities[c])).collect()).collect();
    let mut city_slot: Vec<Option<usize>> = vec![None; cities.len()];
    fn augment(slot: usize, edges: &[Vec<usize>], seen: &mut [bool], city_slot: &mut [Option<usize>]) -> bool {
        for &c in &edges[slot] {
            if seen[c] {
                continue;
            }
            seen[c] = true;
            if city_slot[c].is_none_or(|other| augment(other, edges, seen, city_slot)) {
                city_slot[c] = Some(slot);
                return true;
            }
        }
        false
    }
    for slot in 0..slots {
        let mut seen = vec![false; cities.len()];
        augment(slot, &edges, &mut seen, &mut city_slot);
    }
    city_slot.iter().enumerate().filter_map(|(c, s)| s.map(|s| (cities[c], slot_seat[s]))).collect()
}

/// The seat whose node is on hex `h`, if any.
pub fn node_owner(g: &GameState, h: usize) -> Option<usize> {
    (0..g.seats.len()).find(|&s| g.tokens(h, node(s)) > 0)
}

/// Whether hex `h` holds a building (factory, research station).
pub fn has_building(g: &GameState, h: usize) -> bool {
    g.tokens(h, FACTORY) > 0 || g.tokens(h, STATION) > 0
}

/// An open hex with no building and no Fallout: where buildings go (178, Matt's ruling #28: corrupted
/// land forbids building).
pub fn buildable(w: &World, g: &GameState, h: usize) -> bool {
    w.map.hexes[h].kind == HexKind::Open && !has_building(g, h) && g.tokens(h, FALLOUT) == 0
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

/// 234: the hexes `at` names around target `t`, ascending: the target, it and the hexes next to it, or its
/// country.
pub fn area_of(w: &World, t: usize, at: crate::op::Area) -> Vec<usize> {
    use crate::op::Area;
    let mut out: Vec<usize> = match at {
        Area::On | Area::Reach => vec![t],
        Area::Near => std::iter::once(t).chain(w.map.hexes[t].neighbors.iter().copied()).collect(),
        Area::Country => w.map.countries[w.map.hexes[t].country as usize].hexes.to_vec(),
    };
    out.sort_unstable();
    out.dedup();
    out
}

/// 234: whether hex `h` is in [`area_of`] `t`, `at`.
pub fn in_area(w: &World, t: usize, at: crate::op::Area, h: usize) -> bool {
    use crate::op::Area;
    match at {
        Area::On | Area::Reach => h == t,
        Area::Near => h == t || w.map.hexes[t].neighbors.contains(&h),
        Area::Country => w.map.hexes[h].country == w.map.hexes[t].country,
    }
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

/// Free industrial zones: printed zones holding no building and no Fallout (178), where a factory
/// may be built.
/// 137, 253: whether hex `h` is an empty printed station site, where a neutral research station may open: no
/// building, node or Fallout on it.
pub fn empty_station_site(w: &World, g: &GameState, h: usize) -> bool {
    w.map.hexes[h].station_site && buildable(w, g, h) && node_owner(g, h).is_none()
}

pub fn free_zones(w: &World, g: &GameState) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| w.map.hexes[h].industrial && !has_building(g, h) && g.tokens(h, FALLOUT) == 0)
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

/// Whether a city of `country` (any city, owned or not) is in `seat`'s reach: r75's converter gate (Matt's
/// ruling 9, 2026-11-10; r37's was a node on the city).
pub fn city_in_reach_in(w: &World, g: &GameState, seat: usize, country: usize) -> bool {
    w.map.countries[country]
        .hexes
        .iter()
        .any(|&h| is_city(w, h) && in_reach(w, g, seat, h))
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

/// 289 (D133): [`piece_name`] for a note, a node "Seat 2's node" as "Seat 2" beside it. Options keep
/// [`piece_name`]'s words: a record's answer is matched by its option's label, so a changed label strands it.
pub fn piece_words(kind: &str) -> String {
    match kind.split_once(':') {
        Some(("node", s)) => format!("Seat {s}'s node"),
        _ => piece_name(kind),
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
