//! Strategy-guide heuristics (092): rules a player could say aloud, for what a
//! search one action deep never sees. Each has an on/off switch and its line in
//! docs/agents.md (*Strategy-guide heuristics*).
//!
//! **Shelter when a destruction is showing.** A shelter's whole worth is the
//! destruction it prevents, which resolves after the action phase: this round's
//! uncountered threats and its war step. [`shelter_saves`] reads them from the
//! open table; the eval credits a sheltered country with what it saves.

use aic_core::content::{CounterItem, Op, RoundFlag};
use aic_core::module::World;
use aic_core::modules::common::{self, FACTORY, HARDEN, UNIT};
use aic_core::modules::{board, threats};
use aic_core::GameState;

/// Pieces a destruction showing on the table would take in `country` this
/// round, by owner: `nodes[s]` of seat `s`'s nodes, `factories` player-owned
/// factories. What armor on the hex would absorb anyway is left out (see
/// [`Armor`]), and an unpaid threat counts by its unpaid share.
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

/// How the forecast reads armor (D71).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Armor {
    /// Before 147: any player's ambassador on the hex, and each "destroy N of
    /// your nodes" line read apart (an armored node never fell).
    AnyAmbassador,
    /// 147: as the engine does (`board::destroy_piece`): a hardening token, the
    /// node owner's ambassador or the site owner's; each destroy spends one unit
    /// while any lasts, then takes a node.
    ByOwner,
}

/// Whether `h` would absorb a destruction anyway (a hardening token or an
/// ambassador the engine takes as armor there).
fn absorbed(g: &GameState, h: usize, armor: Armor) -> bool {
    if g.tokens(h, HARDEN) > 0 {
        return true;
    }
    match armor {
        Armor::AnyAmbassador => common::player_ambassador(g, h).is_some(),
        Armor::ByOwner => {
            let own = |s: usize| g.tokens(h, common::ambassador(s)) > 0;
            common::node_owner(g, h).is_some_and(own) || common::cube_owner(g, h).is_some_and(own)
        }
    }
}

/// Whether a strike on the factory on `h` would be absorbed: by owner, only a
/// hardening token or the factory owner's ambassador (the piece is no node).
fn factory_absorbed(g: &GameState, h: usize, armor: Armor) -> bool {
    match armor {
        Armor::AnyAmbassador => absorbed(g, h, armor),
        Armor::ByOwner => {
            g.tokens(h, HARDEN) > 0 || common::cube_owner(g, h).is_some_and(|s| g.tokens(h, common::ambassador(s)) > 0)
        }
    }
}

/// D71: the armor units on hex `h` against destroys aimed at seat `s`'s node
/// there: hardening tokens, `s`'s ambassadors, and the site owner's.
fn armor_units(g: &GameState, s: usize, h: usize) -> i64 {
    let site = common::cube_owner(g, h).filter(|&o| o != s).map_or(0, |o| g.tokens(h, common::ambassador(o)));
    g.tokens(h, HARDEN) + g.tokens(h, common::ambassador(s)) + site
}

/// The share of threat `t`'s counter still unpaid.
fn unpaid(w: &World, g: &GameState, t: &str) -> f64 {
    // 145: by the best resource option, in its own currencies (135 summed units across them).
    1.0 - threats::paid_share(w, g, t).min(1.0)
}

/// Whether the team can still counter threat `t` from what the seats who may
/// counter it hold: then a player counters it rather than shelter (092: crediting
/// a shelter for threats the team went on to counter cost 7.6 ±4.0 points).
fn counterable(w: &World, g: &GameState, t: &str, scope: Option<usize>) -> bool {
    let seats: Vec<usize> = (0..g.seats.len()).filter(|&s| threats::may_counter(w, g, s, t, scope)).collect();
    if seats.is_empty() {
        return false;
    }
    // 135: some resource-only option of the menu the near seats can cover.
    threats::requirement(w, g, t).iter().any(|option| {
        option.iter().all(|item| match *item {
            CounterItem::Pay { resource: r, amount: n } => {
                let paid: i64 = (0..g.seats.len()).map(|s| g.var(&threats::paid_key(t, s, r))).sum();
                let held: i64 = seats.iter().map(|&s| g.resource(s, r).max(0)).sum();
                n - paid <= held
            }
            CounterItem::Sacrifice { .. } | CounterItem::NodesNear { .. } => false,
        })
    })
}

/// What sheltering `country` would save this round, were it not sheltered:
/// the threats on it that destroy (each seat's nodes there, troops landing on
/// its factories) and the war step's crossings into it.
pub fn shelter_saves(w: &World, g: &GameState, country: usize, counter_first: bool, armor: Armor) -> Saves {
    let absorbed = |h: usize| absorbed(g, h, armor);
    let mut out = Saves::default();
    let seats = g.seats.len().min(4);
    let hexes = &w.map.countries[country].hexes;
    let nodes_here = |s: usize| -> Vec<usize> {
        let k = common::node(s);
        hexes.iter().copied().filter(|&h| g.tokens(h, k) > 0).collect()
    };
    for (t, scope) in threats::active(g) {
        if scope != country || threats::waits(g, &t) {
            continue;
        }
        let share = unpaid(w, g, &t);
        if share <= 0.0 || (counter_first && counterable(w, g, &t, Some(scope))) {
            continue;
        }
        // 145: each seat's nodes this threat takes, capped once at its open
        // nodes here. 146: a threat hits one seat, the nearest by node (a tie
        // read as the first tied seat); no unprinted node harm.
        let mut lost = [0i64; 4];
        let hit = threats::nearest_seats(w, g, &t).first().copied();
        for op in &w.content.threat(&t).effects {
            match op {
                Op::DestroyNode { count } => {
                    if let Some(s) = hit.filter(|&s| s < seats && !board::last_node_safe(w, g, s)) {
                        let near: Vec<usize> =
                            threats::nearest_nodes(w, g, s, &t).into_iter().filter(|h| hexes.contains(h)).collect();
                        lost[s] += match armor {
                            Armor::AnyAmbassador => (near.iter().filter(|&&h| !absorbed(h)).count() as i64).min(*count),
                            // D71: the hit player spends armor first, one unit a destroy.
                            Armor::ByOwner => {
                                let units: i64 = near.iter().map(|&h| armor_units(g, s, h)).sum();
                                (*count - units).max(0).min(near.len() as i64)
                            }
                        };
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
                        if hit >= *count || absorbed(h) || board::walled(w, g, h) {
                            continue;
                        }
                        if let Some(s) = common::node_owner(g, h) {
                            if s < seats && !board::last_node_safe(w, g, s) {
                                lost[s] += 1;
                                hit += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        for (s, &n) in lost.iter().enumerate().take(seats) {
            if n <= 0 {
                continue;
            }
            // D71: by owner, the armor is already spent above (a node whose
            // armor runs out falls); before, an armored node never fell.
            let open = match armor {
                Armor::AnyAmbassador => nodes_here(s).into_iter().filter(|&h| !absorbed(h)).count() as i64,
                Armor::ByOwner => nodes_here(s).len() as i64,
            };
            out.nodes[s] += share * n.min(open) as f64;
        }
    }
    {
        // 145: bases make no troop of their own (137); the war forecast reads the
        // troops standing (it added one a base every round).
        let units = |b: usize| -> i64 { g.tokens(b, UNIT) };
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
                if factory_absorbed(g, h, armor) {
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
pub fn sheltered_saves(w: &World, g: &GameState, counter_first: bool, armor: Armor) -> Vec<Saves> {
    (0..w.map.countries.len())
        .filter(|&c| board::flagged(g, RoundFlag::Shelter, c))
        .map(|c| shelter_saves(w, g, c, counter_first, armor))
        .filter(Saves::any)
        .collect()
}
