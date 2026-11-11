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
    /// 147: as the engine does (`board::destroy_piece`): a hardening token or the
    /// node owner's own ambassador (r75: never the site owner's); each destroy spends one unit
    /// while any lasts, then takes a node. 148: one pool a hex across the
    /// country's threats (175: a seat's last node too, as the engine takes it).
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
            // r75 (Matt's ruling 5): only the node owner's own ambassador guards it.
            let own = |s: usize| g.tokens(h, common::ambassador(s)) > 0;
            common::node_owner(g, h).is_some_and(own)
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

/// Reads the country's armor pool and clears its fallen-node counts, once a
/// call (148).
fn fill(g: &GameState, hexes: &[usize], pool: &mut Vec<HexArmor>, taken: &mut Vec<i64>) {
    if pool.is_empty() {
        pool.extend(hexes.iter().map(|&h| HexArmor::read(g, h)));
        taken.resize(hexes.len(), 0);
    }
}

/// 148: the armor on one hex, as `board::destroy_piece` spends it: hardening
/// tokens, then the node owner's own ambassadors (r75: never the site owner's). One pool a
/// hex for a whole forecast, so a unit one threat spends is gone for the next.
#[derive(Clone, Copy)]
struct HexArmor {
    harden: i64,
    amb: [i64; 4],
}

impl HexArmor {
    fn read(g: &GameState, h: usize) -> HexArmor {
        let mut amb = [0; 4];
        for (s, a) in amb.iter_mut().enumerate().take(g.seats.len()) {
            *a = g.tokens(h, common::ambassador(s));
        }
        HexArmor { harden: g.tokens(h, HARDEN), amb }
    }

    /// Spends one unit against a destroy of seat `s`'s node here, if any lasts.
    fn spend(&mut self, s: usize) -> bool {
        if self.harden > 0 {
            self.harden -= 1;
            return true;
        }
        if self.amb[s] > 0 {
            self.amb[s] -= 1;
            return true;
        }
        false
    }
}

/// The share of threat `t`'s counter still unpaid.
fn unpaid(w: &World, g: &GameState, t: &str) -> f64 {
    // 145: by the best resource option, in its own currencies (135 summed units across them).
    1.0 - threats::paid_share(w, g, t).min(1.0)
}

/// Whether the team can still counter threat `t` from what the seats who may
/// counter it hold: then a player counters it rather than shelter (092: crediting
/// a shelter for threats the team went on to counter cost 7.6 ±4.0 points).
fn counterable(w: &World, g: &GameState, t: &str) -> bool {
    let seats = threats::near_seats(w, g, t);
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
    // 148, by owner: one armor pool a hex and one node count a seat for the
    // whole call, drawn down in resolution order, each read when first needed.
    // `taken[i]`: nodes on `hexes[i]` the forecast has taken already.
    let mut pool: Vec<HexArmor> = Vec::new();
    let mut taken: Vec<i64> = Vec::new();
    let mut left: [Option<i64>; 4] = [None; 4];
    // 175: no last-node floor: every source takes a seat's last node.
    for (t, scope) in threats::active(g) {
        if scope != country || threats::waits(g, &t) {
            continue;
        }
        let share = unpaid(w, g, &t);
        if share <= 0.0 || (counter_first && counterable(w, g, &t)) {
            continue;
        }
        // 145: each seat's nodes this threat takes, capped once at its open
        // nodes here. 146: a threat hits one seat, the nearest by node (a tie
        // read as the first tied seat); no unprinted node harm.
        let mut lost = [0i64; 4];
        let hit = threats::nearest_seats(w, g, &t).first().copied();
        for op in &w.content.threat(&t).effects {
            match op {
                Op::DestroyNode { count } if armor == Armor::AnyAmbassador => {
                    if let Some(s) = hit.filter(|&s| s < seats) {
                        let near: Vec<usize> =
                            threats::nearest_nodes(w, g, s, &t).into_iter().filter(|h| hexes.contains(h)).collect();
                        lost[s] += (near.iter().filter(|&&h| !absorbed(h)).count() as i64).min(*count);
                    }
                }
                Op::DestroyNode { count } => {
                    // D71: the hit player spends armor first, one unit a
                    // destroy (148: from the call's pool), then loses a node.
                    let Some(s) = hit.filter(|&s| s < seats) else { continue };
                    let near: Vec<usize> = threats::nearest_nodes(w, g, s, &t)
                        .into_iter()
                        .filter_map(|h| hexes.iter().position(|&x| x == h))
                        .collect();
                    if near.is_empty() {
                        continue;
                    }
                    fill(g, hexes, &mut pool, &mut taken);
                    let k = common::node(s);
                    for _ in 0..*count {
                        let have = *left[s].get_or_insert_with(|| common::node_count(g, s));
                        if have <= 0 {
                            break;
                        }
                        if near.iter().any(|&i| pool[i].spend(s)) {
                            continue;
                        }
                        let Some(&i) = near.iter().find(|&&i| taken[i] < g.tokens(hexes[i], k)) else { break };
                        taken[i] += 1;
                        left[s] = Some(have - 1);
                        lost[s] += 1;
                    }
                }
                Op::Troops { count, destroy: true, .. } if armor == Armor::ByOwner => {
                    // As below, the armor and the seat's nodes drawn from the
                    // call's pool and counts.
                    fill(g, hexes, &mut pool, &mut taken);
                    let mut hit = 0;
                    let factories = hexes.iter().any(|&h| g.tokens(h, FACTORY) > 0);
                    for (i, &h) in hexes.iter().enumerate() {
                        let site = if factories { g.tokens(h, FACTORY) > 0 } else { common::is_city(w, h) };
                        if !site || hit >= *count || board::walled(w, g, h) {
                            continue;
                        }
                        let Some(s) = common::node_owner(g, h).filter(|&s| s < seats) else { continue };
                        if pool[i].spend(s) {
                            continue;
                        }
                        let have = *left[s].get_or_insert_with(|| common::node_count(g, s));
                        if taken[i] >= g.tokens(h, common::node(s)) {
                            continue;
                        }
                        taken[i] += 1;
                        left[s] = Some(have - 1);
                        lost[s] += 1;
                        hit += 1;
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
                            if s < seats {
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
            // armor runs out falls) and the nodes counted out of what stands
            // (148); before, an armored node never fell.
            let open = match armor {
                Armor::AnyAmbassador => nodes_here(s).into_iter().filter(|&h| !absorbed(h)).count() as i64,
                Armor::ByOwner => n,
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

#[cfg(test)]
mod tests {
    use super::*;
    use aic_core::Key;
    use std::sync::Arc;

    /// A board with no nodes, troops, armor or threats on it but `threats`
    /// (token-less, near the whole country) on country 0, and seat 0's nodes on
    /// the first `nodes` hexes of country 0 with a hardening token on the
    /// first `harden` of them.
    fn lab(threats: &[&str], nodes: usize, harden: usize) -> (aic_core::Rules, GameState) {
        let r = crate::agents::tests::rules(0);
        let mut g = r.new_game(1);
        let seats = g.seats.len();
        for hex in Arc::make_mut(&mut g.hexes).iter_mut() {
            Arc::make_mut(hex).tokens.retain(|k, _| {
                let k = k.as_str();
                k != UNIT && k != HARDEN && (0..seats).all(|s| k != common::node(s) && k != common::ambassador(s))
            });
        }
        g.piles.retain(|k, _| !k.as_str().starts_with("threats.on."));
        g.piles.insert("threats.on.0".into(), Arc::new(threats.iter().map(|&t| Key::from(t)).collect()));
        let hexes = r.world().map.countries[0].hexes.clone();
        for (i, &h) in hexes.iter().take(nodes).enumerate() {
            let t = &mut Arc::make_mut(&mut Arc::make_mut(&mut g.hexes)[h]).tokens;
            t.insert(common::node(0).into(), 1);
            if i < harden {
                t.insert(HARDEN.into(), 1);
            }
        }
        (r, g)
    }

    /// 148: two "destroy a node" threats on one country share its one
    /// hardening token: one destroy is absorbed, the other takes a node (each
    /// read apart, both were absorbed).
    #[test]
    fn armor_is_one_pool_across_a_countrys_threats() {
        let (r, g) = lab(&["dark_cities", "purge_protocols"], 3, 1);
        let w = r.world();
        let saves = shelter_saves(&w, &g, 0, false, Armor::ByOwner);
        assert_eq!(saves.nodes[0], 1.0, "one unit absorbs one destroy, not two");
        assert_eq!(saves.factories, 0.0);
        let (r, g) = lab(&["dark_cities", "purge_protocols"], 3, 2);
        assert_eq!(shelter_saves(&r.world(), &g, 0, false, Armor::ByOwner).nodes[0], 0.0, "two units, two destroys");
    }

    /// 175: the forecast takes a seat's last node as the engine does (148's floor is gone): three destroys
    /// take both of two nodes, and one destroy a lone node.
    #[test]
    fn the_forecast_takes_the_last_node() {
        let (r, g) = lab(&["overload", "dark_cities"], 2, 0);
        assert_eq!(shelter_saves(&r.world(), &g, 0, false, Armor::ByOwner).nodes[0], 2.0, "three destroys, two nodes");
        let (r, g) = lab(&["overload"], 1, 0);
        let saves = shelter_saves(&r.world(), &g, 0, false, Armor::ByOwner);
        assert_eq!(saves.nodes[0], 1.0);
        assert!(saves.any());
    }
}
