//! Strategy-guide heuristics (092): rules a player could say aloud, for what a
//! search one action deep never sees. Each has an on/off switch and its line in
//! docs/agents.md (*Strategy-guide heuristics*).
//!
//! **Shelter when a destruction is showing.** A shelter's whole worth is the
//! destruction it prevents, which resolves after the action phase: this round's
//! uncountered threats and its war step. [`shelter_saves`] reads them from the
//! open table; the eval credits a sheltered country with what it saves.

use aic_core::content::{Op, RoundFlag};
use aic_core::module::World;
use aic_core::modules::common::{self, FACTORY, HARDEN, UNIT};
use aic_core::modules::{board, threats};
use aic_core::GameState;

/// Pieces a destruction showing on the table would take in `country` this
/// round, by owner: `nodes[s]` of seat `s`'s nodes, `factories` player-owned
/// factories. What a hardening token or a player ambassador on the hex would
/// absorb anyway is left out, and an unpaid threat counts by its unpaid share.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Saves {
    pub nodes: [f64; 4],
    pub factories: f64,
}

impl Saves {
    pub fn any(&self) -> bool {
        self.factories > 0.0 || self.nodes.iter().any(|&n| n > 0.0)
    }
}

/// Whether `h` would absorb a destruction anyway (a hardening token or a
/// player ambassador on it).
fn absorbed(g: &GameState, h: usize) -> bool {
    g.tokens(h, HARDEN) > 0 || common::player_ambassador(g, h).is_some()
}

/// The share of threat `t`'s counter still unpaid.
fn unpaid(w: &World, g: &GameState, t: &str) -> f64 {
    let req: i64 = threats::requirement(w, g, t).iter().map(|(_, n)| n).sum();
    if req <= 0 {
        return 1.0;
    }
    let prefix = threats::paid_prefix(t);
    let paid: i64 = g
        .vars
        .from_key(prefix.as_str())
        .take_while(|(k, _)| k.starts_with(prefix.as_str()))
        .map(|(_, &n)| n)
        .sum();
    1.0 - (paid as f64 / req as f64).min(1.0)
}

/// Whether the team can still counter threat `t` from what the seats who may
/// counter it hold: then a player counters it rather than shelter (092: crediting
/// a shelter for threats the team went on to counter cost 7.6 ±4.0 points).
fn counterable(w: &World, g: &GameState, t: &str, scope: Option<usize>) -> bool {
    let seats: Vec<usize> = (0..g.seats.len()).filter(|&s| threats::may_counter(w, g, s, t, scope)).collect();
    if seats.is_empty() {
        return false;
    }
    threats::requirement(w, g, t).iter().all(|&(r, n)| {
        let paid: i64 = (0..g.seats.len()).map(|s| g.var(&threats::paid_key(t, s, r))).sum();
        let held: i64 = seats.iter().map(|&s| g.resource(s, r).max(0)).sum();
        n - paid <= held
    })
}

/// What sheltering `country` would save this round, were it not sheltered:
/// the threats on it that destroy (each seat's nodes there, troops landing on
/// its factories) and the war step's crossings into it.
pub fn shelter_saves(w: &World, g: &GameState, country: usize, counter_first: bool) -> Saves {
    let mut out = Saves::default();
    let seats = g.seats.len().min(4);
    let hexes = &w.map.countries[country].hexes;
    let nodes_here = |s: usize| -> Vec<usize> {
        let k = common::node(s);
        hexes.iter().copied().filter(|&h| g.tokens(h, k) > 0).collect()
    };
    for (t, scope) in threats::active(g) {
        if scope != Some(country) || threats::waits(g, &t) {
            continue;
        }
        let share = unpaid(w, g, &t);
        if share <= 0.0 || (counter_first && counterable(w, g, &t, scope)) {
            continue;
        }
        for op in &w.content.threat(&t).effects {
            match op {
                Op::EachDestroyNode { count } => {
                    for s in 0..seats {
                        if board::last_node_safe(w, g, s) {
                            continue;
                        }
                        let open = nodes_here(s).into_iter().filter(|&h| !absorbed(g, h)).count() as i64;
                        out.nodes[s] += share * open.min(*count) as f64;
                    }
                }
                Op::Troops { count, destroy: true, .. } => {
                    // Troops land on factories, or on cities with none; a
                    // landing destroys the nodes on its hex.
                    let factories: Vec<usize> = hexes.iter().copied().filter(|&h| g.tokens(h, FACTORY) > 0).collect();
                    let sites: Vec<usize> = if factories.is_empty() {
                        hexes.iter().copied().filter(|&h| common::is_city(w, h)).collect()
                    } else {
                        factories
                    };
                    let mut hit = 0;
                    for h in sites {
                        if hit >= *count || absorbed(g, h) || board::walled(w, g, h) {
                            continue;
                        }
                        if let Some(s) = common::node_owner(g, h) {
                            if s < seats && !board::last_node_safe(w, g, s) {
                                out.nodes[s] += share;
                                hit += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if board::war_round(w, g) {
        let produce = board::production_round(w, g);
        let cap = w.int("board.unit_cap");
        let units = |b: usize| -> i64 {
            let n = g.tokens(b, UNIT);
            if produce && common::player_ambassador(g, b).is_none() {
                (n + 1).min(cap.max(n))
            } else {
                n
            }
        };
        for f in &w.map.fronts {
            let [ba, bb] = f.bases;
            let (ua, ub) = (units(ba), units(bb));
            let (from, rival, n) = if ua > ub { (ba, f.countries[1], ua - ub) } else { (bb, f.countries[0], ub - ua) };
            if rival as usize != country || n <= 0 {
                continue;
            }
            let mut targets: Vec<usize> = hexes
                .iter()
                .copied()
                .filter(|&h| g.tokens(h, FACTORY) > 0 && !common::corrupted(g, h))
                .collect();
            targets.sort_by_key(|&h| (w.map.distance(from, h), w.map.hexes[h].country, w.map.hexes[h].number));
            // The unit forecast is a guess (factories add units too): count it
            // at a share.
            for &h in targets.iter().take(n as usize) {
                if absorbed(g, h) {
                    continue;
                }
                if common::cube_owner(g, h).is_some() {
                    out.factories += WAR_SHARE;
                }
                for x in common::within(w, h, w.int("board.war_reach")) {
                    if let Some(s) = common::node_owner(g, x) {
                        if s < seats {
                            out.nodes[s] += WAR_SHARE;
                        }
                    }
                }
            }
        }
    }
    out
}

/// How sure a war forecast is: this round's production adds units after the
/// actions.
const WAR_SHARE: f64 = 0.7;

/// The countries sheltered now, each with what its shelter saves.
pub fn sheltered_saves(w: &World, g: &GameState, counter_first: bool) -> Vec<Saves> {
    (0..w.map.countries.len())
        .filter(|&c| board::flagged(g, RoundFlag::Shelter, c))
        .map(|c| shelter_saves(w, g, c, counter_first))
        .filter(Saves::any)
        .collect()
}
