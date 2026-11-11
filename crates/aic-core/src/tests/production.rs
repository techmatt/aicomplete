use super::*;
use crate::modules::{adversary, board};

/// The energy city `h` makes when producing: its country's `corp_energy`.
fn city_energy(rules: &Rules, h: usize) -> i64 {
    let w = rules.world();
    let l = w.list("board.corp_energy");
    l[rules.map.hexes[h].country as usize % l.len()]
}

/// r75 (Matt's ruling 6): production sells a neutral city's energy to nobody
/// (021 sold it to a player next to it); an aligned city's goes free to its owner.
#[test]
fn production_sells_no_neutral_energy_and_an_aligned_city_powers_its_owner() {
    let (rules, g0) = lab(&["board.produce"]);
    let (city, n) = city_with_lonely_neighbors(&rules.map);
    let res = |g: &GameState, s: usize, r: Resource| g.resource(s, r) - g0.resource(s, r);

    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    put(&mut g, n[1], node(1), 1);
    set_res(&mut g, 0, Resource::Credits, 5);
    set_res(&mut g, 1, Resource::Credits, 8);
    run(&rules, &mut g);
    assert!(g.pending.is_none(), "no sale is offered");
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 0));
    assert_eq!((g.resource(0, Resource::Credits), g.resource(1, Resource::Credits)), (5, 8));

    // An aligned city's cube goes free to its owner.
    let mut g = g0.clone();
    put(&mut g, city, cube(1), 1);
    put(&mut g, n[0], node(0), 1);
    run(&rules, &mut g);
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 1));
}

/// r75 (Matt's ruling 6): the Buy energy standard action, `city_energy_credits`
/// (4) for a neutral city's energy: the city in reach and producing (no
/// blackout, no agent on it), once per city a round.
#[test]
fn buy_energy_takes_a_neutral_producing_citys_energy_once_a_round() {
    let (rules, g0) = lab(&["board.produce"]);
    let w = rules.world();
    let (city, n) = city_with_lonely_neighbors(&rules.map);
    let price = w.int("board.city_energy_credits");
    assert_eq!(price, 4);
    let made = city_energy(&rules, city);
    assert!(made > 0);
    let buy = std_act(0, "buy_energy");
    let offered = |g: &GameState| rules.legal_actions(g).contains(&buy);

    let mut g = g0.clone();
    set_res(&mut g, 0, Resource::Credits, price);
    assert!(!offered(&g), "no city in reach");
    put(&mut g, n[0], node(0), 1);
    assert!(board::energy_for_sale(&w, &g, 0, city));
    assert!(offered(&g));
    // A dark city (an agent on it) is not for sale; nor is an aligned one.
    let mut dark = g.clone();
    put(&mut dark, city, ADV_AMBASSADOR, 1);
    assert!(!offered(&dark));
    let mut aligned = g.clone();
    put(&mut aligned, city, cube(1), 1);
    assert!(!board::energy_for_sale(&w, &aligned, 0, city));
    // Short of the price: not offered.
    let mut poor = g.clone();
    set_res(&mut poor, 0, Resource::Credits, price - 1);
    assert!(!offered(&poor));

    let energy = g.resource(0, Resource::Energy);
    act(&rules, &mut g, buy);
    assert_eq!((g.resource(0, Resource::Energy) - energy, g.resource(0, Resource::Credits)), (made, 0));
    // Not twice in a round, for anyone.
    set_res(&mut g, 0, Resource::Credits, price);
    put(&mut g, n[1], node(1), 1);
    assert!(!board::energy_for_sale(&w, &g, 0, city));
    assert!(!board::energy_for_sale(&w, &g, 1, city));
}

/// A station at country `c`'s printed site, owned by `seat`.
fn owned_station(rules: &Rules, g: &mut GameState, c: usize, seat: usize) -> usize {
    let h = rules.map.numbered(c, piece(c).stations[0]);
    put(g, h, STATION, 1);
    put(g, h, cube(seat), 1);
    h
}

#[test]
fn a_full_megaproject_completes_at_the_end_of_production_and_every_cube_pays() {
    let (rules, mut g) = lab(&["board.produce"]);
    owned_station(&rules, &mut g, 0, 0);
    assert_eq!(piece(0).megaproject.cubes, 8);
    for (k, n) in [
        (board::project_key(0, Some(0)), 3),
        (board::project_key(0, Some(1)), 1),
        (board::project_key(0, None), 2),
        (board::project_neutral(0), 2),
    ] {
        g.vars.insert(k.into(), n);
    }
    let (e0, e1, discord) = (g.resource(0, Resource::Energy), g.resource(1, Resource::Energy), g.var("alignment.discord"));
    // 142: the owned station makes `owned_station_cubes` (2 at r59).
    let owned = rules.world().int("board.owned_station_cubes");
    run(&rules, &mut g);
    // 8 + owned cubes on an 8-cube card: all pay. Horizon (075): 3 energy and 3
    // credits a player cube, 1 Discord an adversary cube, nothing for a neutral
    // cube. 127: no holding limit, so seat 0 keeps all of it.
    assert_eq!((g.resource(0, Resource::Energy) - e0, g.resource(1, Resource::Energy) - e1), ((3 + owned) * 3, 3));
    assert_eq!(g.var("alignment.discord") - discord, 2);
    assert_eq!(g.var(&board::project_done(0)), 1);
    for k in [board::project_key(0, Some(0)), board::project_key(0, Some(1)), board::project_key(0, None), board::project_neutral(0)] {
        assert_eq!(g.var(&k), 0);
    }
    assert_eq!(g.stats.counters["board.cubes_paid.player"], 4 + owned);
    assert_eq!(g.stats.counters["board.cubes_paid.adversary"], 2);
    assert_eq!(g.stats.counters["board.cubes_paid.neutral"], 2);
    // Afterward the owned station's cubes go to its owner's board, and stay
    // between rounds.
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::ResearchCubes), owned);
    assert_eq!(g.var(&board::project_key(0, Some(0))), 0);
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::ResearchCubes), 2 * owned);
}

#[test]
fn a_project_short_of_its_cubes_does_not_complete() {
    let (rules, mut g) = lab(&["board.produce"]);
    owned_station(&rules, &mut g, 0, 0);
    // Round 1 already put a corrupted and a neutral station's cubes on it.
    g.vars.insert(board::project_key(0, Some(0)).into(), 4);
    run(&rules, &mut g);
    assert_eq!(g.var(&board::project_done(0)), 0);
    let owned = rules.world().int("board.owned_station_cubes");
    assert_eq!(g.var(&board::project_key(0, Some(0))), 4 + owned);
}

#[test]
fn a_neutral_station_makes_a_neutral_cube_unless_an_adversary_ambassador_blocks_it() {
    // 137: the ambassador blocks the cube while it stands (it took the cube
    // for the adversary and was used up).
    let (rules, mut g) = lab(&["board.produce"]);
    let h = rules.map.numbered(0, piece(0).neutral_station);
    put(&mut g, h, STATION, 1);
    let k = board::project_key(0, None);
    let (neutral, adversary) = (g.var(&board::project_neutral(0)), g.var(&k));
    run(&rules, &mut g);
    assert_eq!(g.var(&board::project_neutral(0)), neutral + 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    for _ in 0..2 {
        run(&rules, &mut g);
    }
    assert_eq!(g.var(&board::project_neutral(0)), neutral + 1);
    assert_eq!(g.var(&k), adversary);
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1);
    assert_eq!(g.stats.counters["board.blocked.station_neutral"], 2);
}

/// 137: an adversary station makes adversary compute, before its country's
/// megaproject completes and after (it put an adversary cube on the project).
/// 175: in the adversary's compute step (the reveal step's income), not production.
#[test]
fn an_adversary_station_makes_compute_before_and_after_completion() {
    let (rules, mut g) = lab_with(&["board.produce", "adversary.reveal", "adversary.compute"], &["adversary.compute_income=0,0,0,0,0"]);
    let cubes = rules.world().int("board.station_cubes");
    for c in 0..2 {
        let h = rules.map.numbered(c, piece(c).stations[0]);
        put(&mut g, h, STATION, 1);
        put(&mut g, h, ADV_CUBE, 1);
    }
    let on_projects = |g: &GameState| (0..2).map(|c| g.var(&board::project_key(c, None))).collect::<Vec<_>>();
    let (made, before) = (g.var(adversary::COMPUTE), on_projects(&g));
    // (Round 1 of the new game already ran the setup's adversary stations.)
    let tallied = g.stats.counters.get("board.station_compute").copied().unwrap_or(0);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE) - made, 2 * cubes);
    assert_eq!(on_projects(&g), before);
    g.vars.insert(board::project_done(0).into(), 1);
    g.vars.insert(board::project_done(1).into(), 1);
    let made = g.var(adversary::COMPUTE);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE) - made, 2 * cubes);
    assert_eq!(g.stats.counters["board.station_compute"] - tallied, 4 * cubes);
}

/// 137, kept at 139 (no dial): an agent blocking a station
/// stands while it blocks.
#[test]
fn a_blocker_stands_while_it_blocks() {
    let (rules, mut g) = lab(&["board.produce"]);
    let h = rules.map.numbered(0, piece(0).neutral_station);
    put(&mut g, h, STATION, 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    let neutral = g.var(&board::project_neutral(0));
    run(&rules, &mut g);
    assert_eq!((g.var(&board::project_neutral(0)), g.tokens(h, ADV_AMBASSADOR)), (neutral, 1));
}

/// r75 (Matt's ruling 9): a completed megaproject's converter needs a city of its
/// country in the seat's reach, any city (r37: a node on one); every converter is
/// once a round.
#[test]
fn a_converter_needs_a_city_of_its_country_in_reach() {
    let (rules, mut g) = lab(&["board.produce"]);
    let w = rules.world();
    let convert = Action::Convert { seat: 0, country: 0 };
    let offered = |g: &GameState| rules.legal_actions(g).contains(&convert);
    with_cx(&rules, |cx| {
        cx.add_resource(&mut g, 0, Resource::ResearchCubes, 3);
        cx.add_resource(&mut g, 0, Resource::Energy, 10);
    });
    // Not complete, then no nodes there (075: an aligned city was 074's gate).
    assert!(!offered(&g));
    g.vars.insert(board::project_done(0).into(), 1);
    assert_eq!(board::converter_blocked(&w, &g, 0, 0), Some(board::NO_CITY_NODE));
    assert!(rules.apply_mut(&mut g, &convert, None).is_err());
    // A node in the country with no city of it in reach is not enough.
    let m = &rules.map;
    let is_city = |h: usize| crate::modules::common::is_city(&w, h);
    let city_of_0 = |h: usize| is_city(h) && m.hexes[h].country == 0;
    let far = hexes_where(m, |h| {
        m.hexes[h].country == 0 && kind(m, h) == HexKind::Open && !m.hexes[h].neighbors.iter().any(|&x| city_of_0(x))
    })[0];
    put(&mut g, far, node(0), 1);
    assert_eq!(board::converter_blocked(&w, &g, 0, 0), Some(board::NO_CITY_NODE));
    // An aligned city out of reach is not the gate either (075).
    let city = m.numbered(0, piece(0).cities[0]);
    put(&mut g, city, cube(0), 1);
    assert!(!offered(&g), "an aligned city is no longer the gate");
    // A node next to the city, not on it: the city is in reach.
    let next = m.hexes[city].neighbors.iter().copied().find(|&x| kind(m, x) == HexKind::Open).unwrap();
    put(&mut g, next, node(0), 1);
    assert!(offered(&g));
    let (e, a) = (g.resource(0, Resource::Energy), g.resource(0, Resource::Antimatter));
    rules.apply_mut(&mut g, &convert, None).unwrap();
    // Horizon: 1 cube + 2 energy, 1 antimatter.
    assert_eq!(
        (g.resource(0, Resource::ResearchCubes), g.resource(0, Resource::Energy), g.resource(0, Resource::Antimatter)),
        (2, e - 2, a + 1)
    );
    // r75 (content): Horizon's converter is once a round too (105 had no limit on it).
    assert_eq!(board::converter_blocked(&w, &g, 0, 0), Some("used this round"));
    assert!(!offered(&g));
}
