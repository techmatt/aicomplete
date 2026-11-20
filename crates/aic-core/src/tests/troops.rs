//! Adversary troops (r141): they garrison where placed, guard the corrupted sites on or next to them, destroy only
//! on a Strike icon and move only on a Redeploy icon. The win with troops still on the board is in `alignment.rs`.

use super::*;
use crate::content::Op;
use crate::modules::adversary::{self, RESEARCH, RESEARCH_SLOT};
use crate::modules::common::prohibit_token;
use crate::modules::{board, effects};
use crate::op::{Per, Piece, Removable, Step, Where};

fn land(m: &Map, h: usize) -> bool {
    kind(m, h) != HexKind::Ocean
}

/// A hex's place in the troops' order: country, then hex number.
fn order(m: &Map, h: usize) -> (u32, u32) {
    (m.hexes[h].country as u32, m.hexes[h].number as u32)
}

fn redeploy(rules: &Rules, g: &mut GameState) {
    team_ops(rules, g, &[Op::Redeploy], |_| 0);
}

fn counter(g: &GameState, k: &str) -> i64 {
    g.stats.counters.get(k).copied().unwrap_or(0)
}

/// The walk over land from `from` to every hex (`None`: no land path), as Redeploy counts it.
fn land_walk(m: &Map, from: usize) -> Vec<Option<u32>> {
    let mut dist = vec![None; m.hexes.len()];
    dist[from] = Some(0);
    let mut queue = std::collections::VecDeque::from([from]);
    while let Some(h) = queue.pop_front() {
        for &n in &m.hexes[h].neighbors {
            if dist[n].is_none() && land(m, n) {
                dist[n] = Some(dist[h].unwrap() + 1);
                queue.push_back(n);
            }
        }
    }
    dist
}

/// A country's cities, lowest-numbered first.
fn cities_of(m: &Map, c: usize) -> Vec<usize> {
    let mut out: Vec<usize> = cities(m).into_iter().filter(|&h| m.hexes[h].country as usize == c).collect();
    out.sort_by_key(|&h| m.hexes[h].number);
    out
}

/// The view's arrows of `kind`, as (from, to, count, rounds).
fn arrows(rules: &Rules, g: &GameState, kind: &str) -> Vec<(usize, usize, i64, i64)> {
    rules.view(g).arrows.iter().filter(|a| a.kind == kind).map(|a| (a.from, a.to, a.count, a.rounds)).collect()
}

fn global(rules: &Rules, g: &GameState, key: &str) -> Option<i64> {
    rules.view(g).globals.iter().find(|v| v.key == key).map(|v| v.value)
}

// ---------- garrison ----------

/// A new troop garrisons its country's lowest-numbered corrupted factory, else its lowest-numbered corrupted
/// city, else its lowest-numbered city; no choice is asked.
#[test]
fn a_new_troop_garrisons_a_corrupted_factory_else_a_corrupted_city_else_a_city() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let hexes = m.countries[0].hexes.clone();
    let cs = cities_of(m, 0);
    assert!(cs.len() >= 3);
    let mut zones: Vec<usize> = hexes.iter().copied().filter(|&h| m.hexes[h].industrial).collect();
    zones.sort_by_key(|&h| m.hexes[h].number);
    assert!(zones.len() >= 2, "two industrial zones in country 0");
    let lands_on = |g: &GameState, n: i64| {
        let mut g = g.clone();
        let spot = board::garrison_spot(&w, &g, &hexes).expect("a garrison");
        let placed = with_cx(&rules, |cx| adversary::land_troops(&mut g, cx, &hexes, n, "test.landed", None, "test")).unwrap();
        assert_eq!(placed, n);
        assert_eq!((g.tokens(spot, TROOP), total(&g, TROOP)), (n, n), "every troop on the one garrison");
        assert_eq!(counter(&g, "test.landed"), n);
        spot
    };
    // No corrupted site: the lowest-numbered city.
    let mut g = g0.clone();
    assert_eq!(lands_on(&g, 1), cs[0]);
    // Corrupted cities: the lowest-numbered of them.
    put(&mut g, cs[1], ADV_CUBE, 1);
    put(&mut g, cs[2], ADV_CUBE, 1);
    assert_eq!(lands_on(&g, 2), cs[1]);
    // A neutral factory and a player's are no garrison.
    put(&mut g, zones[0], FACTORY, 1);
    put(&mut g, zones[1], FACTORY, 1);
    put(&mut g, zones[0], cube(0), 1);
    assert_eq!(lands_on(&g, 1), cs[1]);
    // A corrupted factory comes before any city; of two, the lower-numbered.
    put(&mut g, zones[1], ADV_CUBE, 1);
    assert_eq!(lands_on(&g, 1), zones[1]);
    put(&mut g, zones[0], cube(0), 0);
    put(&mut g, zones[0], ADV_CUBE, 1);
    assert_eq!(lands_on(&g, 2), zones[0]);
    // Another country's corrupted factory is not this country's garrison.
    let other = &m.countries[1].hexes;
    assert_eq!(board::garrison_spot(&w, &g, other), Some(cities_of(m, 1)[0]));
}

/// Wall: a troop whose garrison is the Protector's node hex is not placed there or anywhere else; the Builder's
/// node is no wall.
#[test]
fn wall_keeps_a_garrison_off_the_protectors_node_hex() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let hexes = m.countries[0].hexes.clone();
    let spot = board::garrison_spot(&w, &g0, &hexes).unwrap();
    let mut g = g0.clone();
    put(&mut g, spot, node(1), 1);
    assert!(board::walled(&w, &g, spot));
    let placed = with_cx(&rules, |cx| adversary::land_troops(&mut g, cx, &hexes, 2, "test.landed", None, "test")).unwrap();
    assert_eq!((placed, total(&g, TROOP)), (0, 0), "the troops stay in the supply");
    assert_eq!((counter(&g, "adversary.troops_walled"), counter(&g, "test.landed")), (2, 0));
    let mut g = g0.clone();
    put(&mut g, spot, node(0), 1);
    assert!(!board::walled(&w, &g, spot));
    with_cx(&rules, |cx| adversary::land_troops(&mut g, cx, &hexes, 1, "test.landed", None, "test")).unwrap();
    assert_eq!(g.tokens(spot, TROOP), 1);
}

/// The research row's Garrison icon does what it prints: its troops garrison in each country. Botnet Conscripts'
/// troops garrison too, not on the botnet's hex.
#[test]
fn the_garrison_icon_and_a_botnets_troops_garrison_in_each_country() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let b = &w.content.adversary_board;
    let row = b.research.as_ref().expect("a research row");
    let id = row.slots.iter().find(|s| s.as_str() == "garrison").expect("the row holds a Garrison slot");
    let icon = b.icons.iter().find(|i| &i.id == id).unwrap();
    let count = icon.effects.iter().find_map(|e| match e {
        Op::Troops { count, each_country: true, .. } => Some(*count),
        _ => None,
    });
    let count = count.expect("Garrison: troops in each country") + adversary::troop_bonus(&w);
    let mut g = g0.clone();
    // Country 0 holds a corrupted city that is not its lowest-numbered; country 1 none.
    let c0 = cities_of(m, 0)[1];
    put(&mut g, c0, ADV_CUBE, 1);
    with_answers(&rules, &mut g, |_| 0, |g, cx| effects::run(g, cx, &icon.effects, &mut effects::Ctx::team().named("research")));
    assert_eq!((g.tokens(c0, TROOP), g.tokens(cities_of(m, 1)[0], TROOP)), (count, count));
    assert_eq!(total(&g, TROOP), count * m.countries.len() as i64);

    let mut g = g0.clone();
    put(&mut g, c0, ADV_CUBE, 1);
    let net = m.countries[0].hexes.iter().copied().find(|&h| land(m, h) && !cities(m).contains(&h)).unwrap();
    put(&mut g, net, BOTNET, 1);
    team_ops(&rules, &mut g, &[Op::BotnetTroops { count: 1, troops: 2 }], |_| 0);
    assert_eq!((g.tokens(net, BOTNET), g.tokens(net, TROOP), g.tokens(c0, TROOP)), (0, 0, 2));
}

// ---------- guarding ----------

/// A corrupted site with a troop on or next to it can't be liberated, by the action or by a card; it can again
/// once the troop is removed or pushed away.
#[test]
fn a_troop_on_or_next_to_a_corrupted_site_guards_it_from_liberation() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let (c, near, beside, _) = by_a_city(&mut g, m);
    put(&mut g, c, ADV_CUBE, 1);
    // r142: the Liberate is the act of the seat's ambassador on the city.
    put(&mut g, c, ambassador(0), 1);
    set_res(&mut g, 0, Resource::Ambassadors, 5);
    set_res(&mut g, 0, Resource::Credits, 30);
    let liberate = std_act(0, "liberate");
    let card = Op::Liberate { own: false, ambassadors: 0, troops: 0, botnet: false };
    let buyout = Op::Liberate { own: true, ambassadors: 0, troops: 0, botnet: false };
    // (the action's cost is there, the action is offered, Rescue has a target, Hostile Buyout has a target)
    let open = |g: &GameState| {
        (
            board::liberate_cost(&w, g, c).is_some(),
            rules.legal_actions(g).contains(&liberate),
            effects::feasible(&w, g, 0, std::slice::from_ref(&card)),
            effects::feasible(&w, g, 0, std::slice::from_ref(&buyout)),
        )
    };
    let free = (true, true, true, true);
    let shut = (false, false, false, false);
    assert_eq!(open(&g), free);
    // The seat's view counts the sites in its reach a troop keeps it from liberating (none: no line).
    let refused = |g: &GameState, s: usize| rules.view(g).seats[s].values.iter().find(|v| v.key == "board.liberate_refused").map(|v| v.value);
    assert_eq!(refused(&g, 0), None);
    // On it, next to it, and standing or not: guarded.
    for t in [c, beside] {
        let mut x = g.clone();
        put(&mut x, t, TROOP, 1);
        assert!(board::troop_guarded(&w, &x, c));
        assert_eq!(open(&x), shut, "a troop at {t}");
        assert!(effects::liberable_cities(&w, &x, 0).is_empty() && effects::liberable_sites(&w, &x, 0).is_empty());
        assert_eq!(arrows(&rules, &x, "guard"), vec![(t, c, 1, 0)]);
        assert_eq!((refused(&x, 0), refused(&x, 1)), (Some(1), None), "seat 1 has no node near it");
        // An agent on the city shuts it already: the troop refuses nothing more.
        let mut y = x.clone();
        put(&mut y, c, ADV_AMBASSADOR, 1);
        assert_eq!(refused(&y, 0), None);
        set_var(&mut x, board::stand_key(t), 1);
        assert_eq!(open(&x), shut, "a standing troop at {t}");
    }
    // Two away guards nothing.
    let far = (0..m.hexes.len()).find(|&h| land(m, h) && m.distance(h, c) == 2).unwrap();
    let mut x = g.clone();
    put(&mut x, far, TROOP, 1);
    assert_eq!(open(&x), free);
    assert!(arrows(&rules, &x, "guard").is_empty());
    // Removed (the standard action Remove a troop): the city can be liberated again.
    let mut x = g.clone();
    put(&mut x, beside, TROOP, 1);
    assert_eq!(m.distance(near, beside), 1, "the troop is in seat 0's reach");
    act(&rules, &mut x, std_act(0, "troop"));
    assert_eq!(total(&x, TROOP), 0);
    assert!(board::liberate_cost(&w, &x, c).is_some() && effects::feasible(&w, &x, 0, std::slice::from_ref(&card)));
    // Pushed away (a Push card): the same.
    let mut x = g.clone();
    put(&mut x, beside, TROOP, 1);
    let away = |a: &crate::state::Ask| {
        let clear = |o: &crate::state::Opt| {
            (0..m.hexes.len()).any(|h| Some(m.hexes[h].hex) == o.target && h != beside && m.distance(h, c) > 1)
        };
        a.options.iter().position(clear).or_else(|| a.options.iter().position(|o| o.option == effects::STOP)).unwrap_or(0)
    };
    with_answers(&rules, &mut x, away, |g, cx| {
        effects::run(g, cx, &[Op::Push { max: 1, pieces: vec![Piece::Troops] }], &mut effects::Ctx::seat(0))
    });
    assert_eq!((x.tokens(beside, TROOP), total(&x, TROOP)), (0, 1), "the troop was pushed");
    assert!(!board::troop_guarded(&w, &x, c));
    assert_eq!(open(&x), free);
    act(&rules, &mut x, liberate.clone());
    assert_eq!(x.tokens(c, ADV_CUBE), 0);
}

/// The guarding rule covers a corrupted research station and factory as it does a city.
#[test]
fn a_troop_guards_a_corrupted_station_and_factory() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let z = (0..m.hexes.len()).find(|&h| m.hexes[h].industrial).unwrap();
    let t = m.hexes[z].neighbors.iter().copied().find(|&h| land(m, h)).unwrap();
    for site in [FACTORY, STATION] {
        put(&mut g, z, site, 1);
        put(&mut g, z, ADV_CUBE, 1);
        assert!(board::liberate_cost(&w, &g, z).is_some(), "{site}");
        put(&mut g, t, TROOP, 1);
        assert_eq!(board::liberate_cost(&w, &g, z), None, "{site}");
        assert_eq!(board::guarded_sites(&w, &g, t), vec![z]);
        put(&mut g, t, TROOP, 0);
        put(&mut g, z, site, 0);
    }
}

/// Unity's Reclamation (`remove_where` cubes) takes the adversary's cube off a corrupted station or factory, but
/// not off one a troop guards.
#[test]
fn a_guarded_site_keeps_its_cube_from_reclamation() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let z = (0..m.hexes.len()).find(|&h| m.hexes[h].industrial).unwrap();
    let t = m.hexes[z].neighbors.iter().copied().find(|&h| land(m, h)).unwrap();
    put(&mut g, z, STATION, 1);
    put(&mut g, z, ADV_CUBE, 1);
    put(&mut g, z, ambassador(0), 1);
    let reclaim = [Op::RemoveWhere { pieces: Removable::Cubes, count: 1, per: Per::Hex, where_: Where::Ambassador }];
    for at in [t, z] {
        let mut guarded = g.clone();
        put(&mut guarded, at, TROOP, 1);
        team_ops(&rules, &mut guarded, &reclaim, |_| 0);
        assert_eq!(guarded.tokens(z, ADV_CUBE), 1, "a troop at {at} guards it");
    }
    team_ops(&rules, &mut g, &reclaim, |_| 0);
    assert_eq!((g.tokens(z, ADV_CUBE), g.tokens(z, STATION)), (0, 1));
}

// ---------- Strike ----------

/// On a Strike each troop destroys one player node or player-owned facility on or next to it (the players choose
/// which), and stays: it strikes again on the next icon.
#[test]
fn a_troop_strikes_one_target_stays_and_strikes_again() {
    let (rules, mut g0) = lab(&[]);
    let m = &rules.map;
    let (city, n) = city_with_lonely_neighbors(m);
    let at = n[0];
    let next: Vec<usize> = m.hexes[at].neighbors.iter().copied().filter(|&x| x != city && land(m, x)).collect();
    put(&mut g0, next[0], FACTORY, 1);
    put(&mut g0, next[0], cube(0), 1);
    put(&mut g0, at, node(0), 1);
    let discord = g0.var("alignment.discord");

    let mut g = g0.clone();
    put(&mut g, at, TROOP, 1);
    assert_eq!(board::strike_targets(&rules.world(), &g, at), {
        let mut both = vec![at, next[0]];
        both.sort();
        both
    });
    // Two in reach: the players choose; here the node on the troop's own hex.
    let pick_node = |a: &crate::state::Ask| {
        assert!(a.prompt.starts_with("Strike: the adversary troop at"), "{}", a.prompt);
        assert_eq!(a.options.len(), 2);
        a.options.iter().position(|o| o.option.to_lowercase().contains("node")).expect("the node is an option")
    };
    team_ops(&rules, &mut g, &[Op::Strike], pick_node);
    assert_eq!((g.tokens(at, node(0)), g.tokens(next[0], FACTORY), g.tokens(at, TROOP)), (0, 1, 1), "one target, and the troop stays");
    assert_eq!((counter(&g, "board.troop_strikes"), counter(&g, "board.troop_destroyed.node")), (1, 1));
    // The next icon: the same troop takes the factory. A strike sways no Discord.
    strike(&rules, &mut g);
    assert_eq!((g.tokens(next[0], FACTORY), g.tokens(at, TROOP)), (0, 1));
    assert_eq!(counter(&g, "board.troop_destroyed.factory"), 1);
    assert_eq!(g.var("alignment.discord"), discord);
    // Nothing left in reach: it does nothing, and stays.
    strike(&rules, &mut g);
    assert_eq!((counter(&g, "board.strikes"), counter(&g, "board.troop_strikes"), g.tokens(at, TROOP)), (3, 2, 1));
    // Nor did it move in between.
    assert_eq!(total(&g, TROOP), 1);

    // Two troops on the hex take both in one Strike; a third would find nothing.
    let mut g = g0.clone();
    put(&mut g, at, TROOP, 3);
    strike(&rules, &mut g);
    assert_eq!((g.tokens(at, node(0)), g.tokens(next[0], FACTORY), g.tokens(at, TROOP)), (0, 0, 3));
    assert_eq!(counter(&g, "board.troop_strikes"), 2);
}

/// A city, a neutral facility and an adversary facility are never struck; a standing troop never strikes.
#[test]
fn a_strike_spares_cities_neutral_and_adversary_facilities() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let (city, n) = city_with_lonely_neighbors(m);
    let at = n[0];
    let next: Vec<usize> = m.hexes[at].neighbors.iter().copied().filter(|&x| x != city && land(m, x)).collect();
    assert!(next.len() >= 2);
    // A player's city, a neutral factory, a corrupted research station.
    put(&mut g, city, cube(0), 1);
    put(&mut g, next[0], FACTORY, 1);
    put(&mut g, next[1], STATION, 1);
    put(&mut g, next[1], ADV_CUBE, 1);
    put(&mut g, at, TROOP, 2);
    let before = g.hexes.clone();
    assert!(board::strike_targets(&w, &g, at).is_empty());
    assert!(arrows(&rules, &g, "troop").is_empty());
    strike(&rules, &mut g);
    assert_eq!(g.hexes, before, "nothing struck, nothing moved");
    assert_eq!(counter(&g, "board.troop_strikes"), 0);
    // The factory, once a player's, is a target; a standing troop still leaves it.
    put(&mut g, next[0], cube(0), 1);
    set_var(&mut g, board::stand_key(at), 2);
    strike(&rules, &mut g);
    assert_eq!(g.tokens(next[0], FACTORY), 1, "standing troops never strike");
    assert!(arrows(&rules, &g, "troop").is_empty());
    set_var(&mut g, board::stand_key(at), 0);
    strike(&rules, &mut g);
    assert_eq!((g.tokens(next[0], FACTORY), g.tokens(at, TROOP)), (0, 2));
}

/// Wall: nothing on the Protector's node hex is struck. A no-Strike token (Demilitarized Zone) keeps its hex
/// from every troop too.
#[test]
fn a_walled_hex_and_a_no_strike_hex_are_not_struck() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let p = m.numbered(0, piece(0).node);
    let t = m.hexes[p].neighbors.iter().copied().find(|&h| land(m, h)).unwrap();
    // The Protector's node and its station under it; a troop next door and one on the hex.
    let mut g = g0.clone();
    put(&mut g, p, node(1), 1);
    put(&mut g, p, STATION, 1);
    put(&mut g, p, cube(1), 1);
    put(&mut g, t, TROOP, 1);
    put(&mut g, p, TROOP, 1);
    assert!(board::strike_targets(&w, &g, t).is_empty());
    strike(&rules, &mut g);
    assert_eq!((g.tokens(p, STATION), g.tokens(p, node(1)), total(&g, TROOP)), (1, 1, 2));
    assert_eq!(counter(&g, "board.troop_strikes"), 0);
    assert_eq!(counter(&g, "board.wall_strikes"), 2, "the Wall's measure: once for each troop it turned");
    // The Builder's node is no wall: the same board with seat 0's pieces is struck.
    let mut g = g0.clone();
    put(&mut g, p, node(0), 1);
    put(&mut g, t, TROOP, 1);
    let mut held = g.clone();
    strike(&rules, &mut g);
    assert_eq!(g.tokens(p, node(0)), 0);
    // With a no-Strike token on the node's hex it is not; the block is counted.
    put(&mut held, p, prohibit_token(Step::Strike), 1);
    assert!(board::strike_targets(&w, &held, t).is_empty());
    strike(&rules, &mut held);
    assert_eq!((held.tokens(p, node(0)), held.tokens(t, TROOP)), (1, 1));
    let wh = crate::modules::plan::prohibition_on(&held, Step::Strike, p).expect("the token's condition").name();
    assert_eq!(counter(&held, &format!("prohibit.strike.{wh}.fired")), 1);
    assert_eq!(counter(&held, &format!("prohibit.strike.{wh}.blocked")), 1);
}

/// A strike that takes a seat's last node ends the game there: the troops after it don't strike. Played through
/// the Card beat of an event card that prints the icon.
#[test]
fn a_strike_taking_a_seats_last_node_ends_the_game() {
    let (rules, mut g) = lab_with(&["adversary.build"], &["alignment.node_loss=1"]);
    let m = &rules.map;
    let w = rules.world();
    let card = w.content.adversary.iter().find(|c| c.id == "mil_s2_fifth_column").unwrap();
    let at = card.standing.iter().position(|e| e == &Op::Strike).expect("Fifth Column prints a Strike");
    assert!(card.standing[at + 1..].iter().any(|e| matches!(e, Op::SpreadBotnets { .. })), "and a botnet line after it");
    // Seat 0's only node in country 0 with a troop next to it; in country 1 (struck later) seat 1's factory with
    // a troop on it.
    let p = m.numbered(0, piece(0).node);
    let t1 = m.hexes[p].neighbors.iter().copied().find(|&h| land(m, h) && m.hexes[h].country == 0).unwrap();
    let f = m.numbered(1, piece(1).factories[0]);
    let home = m.countries[1].hexes.iter().copied().find(|&h| land(m, h) && m.distance(h, f) > 1).unwrap();
    put(&mut g, p, node(0), 1);
    for s in 1..g.seats.len() {
        put(&mut g, home, node(s), 1);
    }
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, cube(1), 1);
    put(&mut g, t1, TROOP, 1);
    put(&mut g, f, TROOP, 1);
    // A research station for the card's botnet line to land on, were it to run.
    put(&mut g, m.numbered(1, piece(1).neutral_station), STATION, 1);
    assert!(order(m, t1) < order(m, f));
    // The build takes the face-up card as this round's.
    set_pile(&mut g, NEXT, &["mil_s2_fifth_column"]);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(p, node(0)), 0, "the last node fell");
    assert_eq!((g.tokens(f, FACTORY), counter(&g, "board.troop_strikes")), (1, 1), "the later troop never struck");
    // Nor do the card's later lines run: Fifth Column's botnets, after its Strike, never spread.
    assert_eq!(total(&g, BOTNET), 0);
    let o = g.outcome.as_ref().expect("the game ended");
    assert!(!o.won && o.reason == "node", "{o:?}");
}

/// A protection absorbs a troop's hit as it does any destruction: hardening is spent, the piece and the troop
/// both stay. (The owner's ambassador: `ambassadors.rs`.)
#[test]
fn hardening_absorbs_a_strike() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let p = m.numbered(0, piece(0).node);
    let t = m.hexes[p].neighbors.iter().copied().find(|&h| land(m, h)).unwrap();
    put(&mut g, p, node(0), 1);
    put(&mut g, p, HARDEN, 1);
    put(&mut g, t, TROOP, 1);
    strike(&rules, &mut g);
    assert_eq!((g.tokens(p, node(0)), g.tokens(p, HARDEN), g.tokens(t, TROOP)), (1, 0, 1));
    assert_eq!((counter(&g, "board.troop_absorbed"), counter(&g, "board.troop_destroyed")), (1, 0));
    strike(&rules, &mut g);
    assert_eq!(g.tokens(p, node(0)), 0, "the next Strike finds it unprotected");
}

// ---------- Redeploy ----------

/// On a Redeploy a troop not on or next to a corrupted city is put on the nearest corrupted city, counted as a walk over land;
/// ties go to the lower country, then the lower hex number.
#[test]
fn a_troop_guarding_nothing_redeploys_onto_the_nearest_corrupted_city() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let all = cities(m);
    let spots: Vec<usize> = (0..m.hexes.len()).filter(|&h| land(m, h) && all.iter().all(|&c| m.distance(h, c) > 1)).collect();
    let pairs = |t: usize| {
        let d = land_walk(m, t);
        let all = all.clone();
        all.clone().into_iter().flat_map(move |a| all.clone().into_iter().map(move |b| (a, b))).filter_map(move |(a, b)| Some((t, a, b, d[a]?, d[b]?)))
    };
    // The nearer city wins though the farther one comes first in country and number order.
    let (t, a, b, ..) = spots
        .iter()
        .flat_map(|&t| pairs(t))
        .find(|&(_, a, b, da, db)| da < db && order(m, a) > order(m, b))
        .expect("a troop nearer the higher-numbered of two cities");
    let mut g = g0.clone();
    put(&mut g, a, ADV_CUBE, 1);
    put(&mut g, b, ADV_CUBE, 1);
    put(&mut g, t, TROOP, 2);
    redeploy(&rules, &mut g);
    assert_eq!((g.tokens(t, TROOP), g.tokens(a, TROOP), g.tokens(b, TROOP)), (0, 2, 0), "the whole stack, to the nearer city");
    assert_eq!((counter(&g, "board.redeploys"), counter(&g, "board.troops_redeployed")), (1, 2));
    // There it guards: a second Redeploy leaves it.
    redeploy(&rules, &mut g);
    assert_eq!(g.tokens(a, TROOP), 2);

    // Equally near: the lower country, then the lower hex number (a pair in two countries where the map has one).
    let ties: Vec<(usize, usize, usize)> = spots
        .iter()
        .flat_map(|&t| pairs(t))
        .filter(|&(_, a, b, da, db)| da == db && order(m, a) < order(m, b))
        .map(|(t, a, b, ..)| (t, a, b))
        .collect();
    let split = ties.iter().find(|&&(_, a, b)| m.hexes[a].country != m.hexes[b].country && m.hexes[a].number > m.hexes[b].number);
    let &(t, a, b) = split.or(ties.first()).expect("a troop equally near two cities");
    let mut g = g0.clone();
    put(&mut g, a, ADV_CUBE, 1);
    put(&mut g, b, ADV_CUBE, 1);
    put(&mut g, t, TROOP, 1);
    redeploy(&rules, &mut g);
    assert_eq!((g.tokens(a, TROOP), g.tokens(b, TROOP), total(&g, TROOP)), (1, 0, 1));
}

/// A troop on or next to a corrupted city stays on a Redeploy (one on a corrupted factory or station away from any
/// leaves it); so does every troop when no corrupted city can be reached, a standing troop, and a troop whose only
/// destination is under the Protector's node (Wall).
#[test]
fn a_redeploy_leaves_a_guarding_troop_and_one_with_nowhere_to_go() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let all = cities(m);
    let t = (0..m.hexes.len()).find(|&h| land(m, h) && all.iter().all(|&c| m.distance(h, c) > 1)).unwrap();
    let d = land_walk(m, t);
    let mut near: Vec<usize> = all.iter().copied().filter(|&c| d[c].is_some()).collect();
    near.sort_by_key(|&c| (d[c], order(m, c)));
    assert!(near.len() >= 2);
    // No corrupted city: it stays.
    let mut g = g0.clone();
    put(&mut g, t, TROOP, 2);
    let before = g.hexes.clone();
    redeploy(&rules, &mut g);
    assert_eq!(g.hexes, before);
    assert_eq!((counter(&g, "board.redeploy_stayed"), counter(&g, "board.troops_redeployed")), (2, 0));
    // A troop on or next to a corrupted city stays, though another corrupted city stands elsewhere.
    for at in std::iter::once(near[0]).chain(m.hexes[near[0]].neighbors.iter().copied().filter(|&h| land(m, h) && !all.contains(&h)).take(1)) {
        let mut g = g0.clone();
        put(&mut g, near[0], ADV_CUBE, 1);
        put(&mut g, near[1], ADV_CUBE, 1);
        put(&mut g, at, TROOP, 1);
        let before = g.hexes.clone();
        redeploy(&rules, &mut g);
        assert_eq!(g.hexes, before, "a troop at {at}");
    }
    // A corrupted factory or research station is not a city: a troop on one leaves it for the corrupted city.
    for site in [FACTORY, STATION] {
        let mut g = g0.clone();
        put(&mut g, near[0], ADV_CUBE, 1);
        put(&mut g, t, site, 1);
        put(&mut g, t, ADV_CUBE, 1);
        put(&mut g, t, TROOP, 1);
        redeploy(&rules, &mut g);
        assert_eq!((g.tokens(t, TROOP), g.tokens(near[0], TROOP)), (0, 1), "{site}");
    }
    // A standing troop never Redeploys.
    let mut g = g0.clone();
    put(&mut g, near[0], ADV_CUBE, 1);
    put(&mut g, t, TROOP, 1);
    set_var(&mut g, board::stand_key(t), 1);
    redeploy(&rules, &mut g);
    assert_eq!(g.tokens(t, TROOP), 1);
    // Wall: the nearest corrupted city under the Protector's node is no destination; the next one is.
    let mut g = g0.clone();
    put(&mut g, near[0], ADV_CUBE, 1);
    put(&mut g, near[0], node(1), 1);
    put(&mut g, t, TROOP, 1);
    redeploy(&rules, &mut g);
    assert_eq!((g.tokens(t, TROOP), g.tokens(near[0], TROOP)), (1, 0), "only a walled city: it stays");
    put(&mut g, near[1], ADV_CUBE, 1);
    redeploy(&rules, &mut g);
    assert_eq!((g.tokens(t, TROOP), g.tokens(near[0], TROOP), g.tokens(near[1], TROOP)), (0, 0, 1));
}

/// Redeployment's icon, through the Card beat: the card's Redeploy line moves the troop.
#[test]
fn an_event_cards_redeploy_line_moves_the_troops() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let m = &rules.map;
    let w = rules.world();
    let card = w.content.adversary.iter().find(|c| c.standing.contains(&Op::Redeploy)).expect("a card prints Redeploy");
    let all = cities(m);
    let t = (0..m.hexes.len()).find(|&h| land(m, h) && all.iter().all(|&c| m.distance(h, c) > 1)).unwrap();
    put(&mut g, t, TROOP, 1);
    for &c in &all {
        put(&mut g, c, ADV_CUBE, 1);
    }
    set_pile(&mut g, NEXT, &[card.id.as_str()]);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(t, TROOP), 0);
    assert_eq!(all.iter().map(|&c| g.tokens(c, TROOP)).sum::<i64>(), 1, "the troop stands on a corrupted city");
    assert_eq!(counter(&g, "board.redeploys"), 1);
}

// ---------- Vigil ----------

/// Vigil: at a Strike the Protector gains its insight once for each troop on or next to its nodes, at most
/// `board.vigil_cap` troops a round; a troop elsewhere pays nothing.
#[test]
fn vigil_pays_for_each_troop_by_the_protectors_node_at_a_strike_up_to_its_cap() {
    let (rules, g0) = lab(&[]);
    let cap = rules.world().int("board.vigil_cap");
    let m = &rules.map;
    let p = m.numbered(0, piece(0).node);
    let t = m.hexes[p].neighbors.iter().copied().find(|&h| land(m, h)).unwrap();
    let far = (0..m.hexes.len()).find(|&h| land(m, h) && m.distance(h, p) >= 3).unwrap();
    let mut base = g0.clone();
    put(&mut base, p, node(1), 1);
    set_res(&mut base, 1, Resource::Insight, 0);
    // A troop far off: nothing.
    let mut g = base.clone();
    put(&mut g, far, TROOP, 1);
    strike(&rules, &mut g);
    assert_eq!((g.resource(1, Resource::Insight), counter(&g, "board.vigil")), (0, 0));
    // One next to the node: 1 insight, with no strike made (the Wall holds).
    let mut g = base.clone();
    put(&mut g, t, TROOP, 1);
    strike(&rules, &mut g);
    assert_eq!((g.tokens(p, node(1)), g.resource(1, Resource::Insight), counter(&g, "board.vigil")), (1, 1, 1));
    // No Strike, no Vigil: a Redeploy pays nothing.
    redeploy(&rules, &mut g);
    assert_eq!(g.resource(1, Resource::Insight), 1);
    // More troops than the cap: the cap, then again in a new round. The cap is a round's, not a Strike's.
    let mut g = base.clone();
    put(&mut g, t, TROOP, cap + 2);
    strike(&rules, &mut g);
    assert_eq!((g.resource(1, Resource::Insight), counter(&g, "board.vigil_capped")), (cap, 2));
    strike(&rules, &mut g);
    assert_eq!(g.resource(1, Resource::Insight), cap, "a second Strike in the round pays nothing more");
    g.round += 1;
    strike(&rules, &mut g);
    assert_eq!(g.resource(1, Resource::Insight), 2 * cap, "a new round, a new cap");
    // The Builder has no Vigil.
    assert_eq!(g.resource(0, Resource::Insight), base.resource(0, Resource::Insight));
}

// ---------- the view ----------

/// The view draws a `troop` arrow from a garrison to each hex it can strike (rounds 1 once a Strike is shown
/// ahead, else 2) and a `guard` arrow to each corrupted site it guards, and says whether a Strike or a Redeploy
/// icon is ahead.
#[test]
fn the_view_shows_what_a_garrison_guards_and_strikes_and_the_icons_ahead() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let (city, n) = city_with_lonely_neighbors(m);
    let at = n[0];
    let target = m.hexes[at].neighbors.iter().copied().find(|&x| x != city && land(m, x)).unwrap();
    // No troop, no icon: neither global is shown.
    assert_eq!((global(&rules, &g, "board.strike_ahead"), global(&rules, &g, "board.redeploy_ahead")), (None, None));
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, target, node(0), 1);
    put(&mut g, at, TROOP, 2);
    assert_eq!(arrows(&rules, &g, "guard"), vec![(at, city, 2, 0)]);
    assert_eq!(arrows(&rules, &g, "troop"), vec![(at, target, 2, 2)], "no Strike shown yet");
    assert_eq!((global(&rules, &g, "board.strike_ahead"), global(&rules, &g, "board.redeploy_ahead")), (Some(0), Some(0)));
    // The lab's round is its action phase alone: the card of the next Card beat is the face-up one.
    let pile = adversary::round_pile(&g);
    for card in &w.content.adversary {
        set_pile(&mut g, pile, &[card.id.as_str()]);
        let (s, r) = (card.standing.contains(&Op::Strike), card.standing.contains(&Op::Redeploy));
        assert_eq!(global(&rules, &g, "board.strike_ahead"), Some(s as i64), "{}", card.id);
        assert_eq!(global(&rules, &g, "board.redeploy_ahead"), Some(r as i64), "{}", card.id);
        assert_eq!(arrows(&rules, &g, "troop"), vec![(at, target, 2, if s { 1 } else { 2 })], "{}", card.id);
        assert_eq!((adversary::line_ahead(&w, &g, &Op::Strike), adversary::line_ahead(&w, &g, &Op::Redeploy)), (s, r));
    }
    assert!(w.content.adversary.iter().any(|c| c.standing.contains(&Op::Strike)));
    // A standing troop guards but strikes nothing: no `troop` arrow for it.
    set_pile(&mut g, pile, &[]);
    set_var(&mut g, board::stand_key(at), 2);
    assert!(arrows(&rules, &g, "troop").is_empty());
    assert_eq!(arrows(&rules, &g, "guard").len(), 1);
}

/// After this round's Card beat the icon ahead is next round's card's (face up from the reveal), not the spent
/// one's.
#[test]
fn the_icon_ahead_is_next_rounds_card_once_this_rounds_has_run() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let w = rules.world();
    let strike = w.content.adversary.iter().find(|c| c.standing.contains(&Op::Strike)).unwrap().id.clone();
    let quiet = w.content.adversary.iter().find(|c| !c.standing.contains(&Op::Strike)).unwrap().id.clone();
    // The lab stands at its action phase, past the build.
    set_pile(&mut g, CURRENT, &[strike.as_str()]);
    set_pile(&mut g, NEXT, &[quiet.as_str()]);
    assert!(!adversary::line_ahead(&w, &g, &Op::Strike), "this round's card is spent");
    set_pile(&mut g, NEXT, &[strike.as_str()]);
    assert!(adversary::line_ahead(&w, &g, &Op::Strike));
    assert_eq!(global(&rules, &g, "board.strike_ahead"), Some(1));
}

/// The research row is public too: a Strike slot the track already holds the cubes to reach is a Strike ahead,
/// and reaching it strikes.
#[test]
fn the_research_rows_strike_slot_is_shown_ahead_and_strikes() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    assert!(adversary::research_on(&w));
    let row = w.content.adversary_board.research.as_ref().expect("a research row");
    let slot = row.slots.iter().position(|s| s == "strike").expect("the row holds a Strike slot") as i64 + 1;
    let at = adversary::research_threshold(&w);
    let p = m.numbered(0, piece(0).node);
    let t = m.hexes[p].neighbors.iter().copied().find(|&h| land(m, h)).unwrap();
    put(&mut g, p, node(0), 1);
    put(&mut g, t, TROOP, 1);
    set_var(&mut g, RESEARCH_SLOT, slot - 1);
    set_var(&mut g, RESEARCH, at - 1);
    assert!(!adversary::strike_slot_ahead(&w, &g), "short of the slot's cubes");
    assert_eq!(arrows(&rules, &g, "troop"), vec![(t, p, 1, 2)]);
    // The forecast counts the cube each corrupted station will make, unless a player ambassador blocks it.
    if w.int("board.station_cubes") >= 1 {
        let mut soon = g.clone();
        let st = (0..m.hexes.len()).find(|&h| land(m, h) && m.distance(h, p) >= 3 && m.distance(h, t) >= 2).unwrap();
        put(&mut soon, st, STATION, 1);
        assert!(!adversary::strike_slot_ahead(&w, &soon), "a neutral station feeds no adversary research");
        put(&mut soon, st, ADV_CUBE, 1);
        assert!(adversary::strike_slot_ahead(&w, &soon));
        put(&mut soon, st, ambassador(0), 1);
        assert!(!adversary::strike_slot_ahead(&w, &soon));
    }
    set_var(&mut g, RESEARCH, at);
    assert!(adversary::strike_slot_ahead(&w, &g));
    assert_eq!(global(&rules, &g, "board.strike_ahead"), Some(1));
    assert_eq!(arrows(&rules, &g, "troop"), vec![(t, p, 1, 1)]);
    // Another slot next: no Strike ahead.
    let mut other = g.clone();
    let quiet = (1..=row.slots.len() as i64).find(|&s| row.slots[s as usize - 1] != "strike").unwrap();
    set_var(&mut other, RESEARCH_SLOT, quiet - 1);
    assert!(!adversary::strike_slot_ahead(&w, &other));
    // The track reaches the slot: the troop strikes, and stays.
    with_answers(&rules, &mut g, |_| 0, adversary::research_step);
    assert_eq!((g.var(RESEARCH_SLOT), g.tokens(p, node(0)), g.tokens(t, TROOP)), (slot, 0, 1));
}
