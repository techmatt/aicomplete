use super::*;
use crate::modules::{adversary, board};

/// The energy city `h` makes when producing: its country's `corp_energy`.
fn city_energy(rules: &Rules, h: usize) -> i64 {
    let w = rules.world();
    let l = w.list("board.corp_energy");
    l[rules.map.hexes[h].country as usize % l.len()]
}

/// 286 (D124): the seat strip's income adds production's: a seat's factory's chips (none while an agent blocks it)
/// and, once its country's megaproject is complete, its station's cubes.
#[test]
fn production_income_reads_the_seats_factories_and_stations() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let f = m.numbered(0, piece(0).node);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, cube(0), 1);
    let chips = w.int("board.factory_chips");
    assert_eq!(board::production_income(&w, &g, 0), (chips, 0));
    assert_eq!(board::production_income(&w, &g, 1), (0, 0));
    put(&mut g, f, ADV_AMBASSADOR, 1);
    assert_eq!(board::production_income(&w, &g, 0), (0, 0), "an agent blocks it");
    let s = m.numbered(1, piece(1).node);
    put(&mut g, s, STATION, 1);
    put(&mut g, s, cube(0), 1);
    assert_eq!(board::production_income(&w, &g, 0), (0, 0), "before its megaproject completes, onto it");
    set_var(&mut g, board::project_done(m.hexes[s].country as usize), 1);
    assert_eq!(board::production_income(&w, &g, 0), (0, w.int("board.owned_station_cubes")));
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
        (board::project_key(0, 0), 3),
        (board::project_key(0, 1), 1),
        (board::project_neutral(0), 4),
    ] {
        set_var(&mut g, k, n);
    }
    let (e0, e1, discord) = (g.resource(0, Resource::Energy), g.resource(1, Resource::Energy), g.var("alignment.discord"));
    // 142: the owned station makes `owned_station_cubes` (2 at r59).
    let owned = rules.world().int("board.owned_station_cubes");
    run(&rules, &mut g);
    // 8 + owned cubes on an 8-cube card: all pay. Horizon (075): 3 energy and 3
    // credits a player cube, nothing for a neutral cube (293: it takes no adversary cube). 127: no holding limit, so seat 0 keeps all of it.
    assert_eq!((g.resource(0, Resource::Energy) - e0, g.resource(1, Resource::Energy) - e1), ((3 + owned) * 3, 3));
    assert_eq!(g.var("alignment.discord") - discord, 0, "no adversary payout, and no city Discord in production");
    assert_eq!(g.var(&board::project_done(0)), 1);
    for k in [board::project_key(0, 0), board::project_key(0, 1), board::project_neutral(0)] {
        assert_eq!(g.var(&k), 0);
    }
    assert_eq!(g.stats.counters["board.cubes_paid.player"], 4 + owned);
    assert!(!g.stats.counters.contains_key("board.cubes_paid.adversary"));
    assert_eq!(g.stats.counters["board.cubes_paid.neutral"], 4);
    // Afterward the owned station's cubes go to its owner's board, and stay
    // between rounds.
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::ResearchCubes), owned);
    assert_eq!(g.var(&board::project_key(0, 0)), 0);
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::ResearchCubes), 2 * owned);
}

#[test]
fn a_project_short_of_its_cubes_does_not_complete() {
    let (rules, mut g) = lab(&["board.produce"]);
    owned_station(&rules, &mut g, 0, 0);
    // Round 1 already put a neutral station's cube on it (293: the corrupted one's goes to the research track).
    set_var(&mut g, board::project_key(0, 0), 1);
    run(&rules, &mut g);
    assert_eq!(g.var(&board::project_done(0)), 0);
    let owned = rules.world().int("board.owned_station_cubes");
    assert_eq!(g.var(&board::project_key(0, 0)), 1 + owned);
}

#[test]
fn a_neutral_station_makes_a_neutral_cube_unless_an_adversary_ambassador_blocks_it() {
    // 137: the ambassador blocks the cube while it stands (it took the cube
    // for the adversary and was used up).
    let (rules, mut g) = lab(&["board.produce"]);
    let h = rules.map.numbered(0, piece(0).neutral_station);
    put(&mut g, h, STATION, 1);
    let neutral = g.var(&board::project_neutral(0));
    run(&rules, &mut g);
    assert_eq!(g.var(&board::project_neutral(0)), neutral + 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    for _ in 0..2 {
        run(&rules, &mut g);
    }
    assert_eq!(g.var(&board::project_neutral(0)), neutral + 1);
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1);
    assert_eq!(g.stats.counters["board.blocked.station_neutral"], 2);
}

/// 293 (Matt, 2026-11-19): an adversary station's one output is a research cube onto the adversary's track, in its
/// Income beat, from round 1: megaproject open or complete, it puts no cube on the megaproject and makes no compute.
/// A player ambassador on it blocks it.
#[test]
fn an_adversary_station_puts_its_cube_on_the_research_track_from_round_one() {
    let (rules, mut g) = lab_with(&["board.produce", "adversary.reveal", "adversary.compute"], &["adversary.compute_income=0,0,0,0,0", "adversary.research_at=9,9,9,9,9"]);
    let cubes = rules.world().int("board.station_cubes");
    for c in 0..2 {
        let h = rules.map.numbered(c, piece(c).stations[0]);
        put(&mut g, h, STATION, 1);
        put(&mut g, h, ADV_CUBE, 1);
    }
    let on_projects = |g: &GameState| (0..2).map(|c| g.var(&board::project_neutral(c)) + g.var(&board::project_key(c, 0)) + g.var(&board::project_key(c, 1))).collect::<Vec<_>>();
    let corrupted_stations = (0..g.hexes.len()).filter(|&h| g.tokens(h, STATION) > 0 && g.tokens(h, ADV_CUBE) > 0).count() as i64;
    let neutral_stations: Vec<i64> = (0..2u8).map(|c| (0..g.hexes.len()).filter(|&h| rules.map.hexes[h].country == c && g.tokens(h, STATION) > 0 && g.tokens(h, ADV_CUBE) == 0 && g.tokens(h, ADV_AMBASSADOR) == 0).count() as i64).collect();
    let (made, before, research) = (g.var(adversary::COMPUTE), on_projects(&g), g.var(adversary::RESEARCH));
    let botnets = g.stats.counters.get("adversary.compute_botnets").copied().unwrap_or(0);
    run(&rules, &mut g);
    let from_botnets = g.stats.counters.get("adversary.compute_botnets").copied().unwrap_or(0) - botnets;
    assert_eq!(g.var(adversary::COMPUTE) - made, from_botnets, "a station makes no compute");
    assert_eq!(g.var(adversary::RESEARCH) - research, corrupted_stations * cubes, "a cube each, megaprojects open");
    assert_eq!(on_projects(&g), before.iter().zip(&neutral_stations).map(|(b, n)| b + n * cubes).collect::<Vec<_>>(), "only neutral cubes on the megaprojects");
    // A player ambassador on country 0's station blocks its cube.
    let h0 = rules.map.numbered(0, piece(0).stations[0]);
    put(&mut g, h0, ambassador(0), 1);
    let research = g.var(adversary::RESEARCH);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::RESEARCH) - research, (corrupted_stations - 1) * cubes);
    put(&mut g, h0, ambassador(0), 0);
    set_var(&mut g, board::project_done(0), 1);
    set_var(&mut g, board::project_done(1), 1);
    let research = g.var(adversary::RESEARCH);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::RESEARCH) - research, corrupted_stations * cubes, "the same after completion");
}

/// 259: the off arm (`research_at` all 0): an adversary station makes compute.
#[test]
fn with_the_research_rule_off_an_adversary_station_makes_compute_after_completion() {
    let (rules, mut g) = lab_with(
        &["board.produce", "adversary.reveal", "adversary.compute"],
        &["adversary.compute_income=0,0,0,0,0", "adversary.research_at=0,0,0,0,0"],
    );
    let cubes = rules.world().int("board.station_cubes");
    for c in 0..2 {
        let h = rules.map.numbered(c, piece(c).stations[0]);
        put(&mut g, h, STATION, 1);
        put(&mut g, h, ADV_CUBE, 1);
        set_var(&mut g, board::project_done(c), 1);
    }
    let made = g.var(adversary::COMPUTE);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE) - made, 2 * cubes);
    assert_eq!(g.var(adversary::RESEARCH), 0);
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
    let convert = Action::Convert { seat: 0, country: 0, chips: false };
    let offered = |g: &GameState| rules.legal_actions(g).contains(&convert);
    with_cx(&rules, |cx| {
        cx.add_resource(&mut g, 0, Resource::ResearchCubes, 3);
        cx.add_resource(&mut g, 0, Resource::Energy, 10);
    });
    // Not complete, then no nodes there (075: an aligned city was 074's gate).
    assert!(!offered(&g));
    set_var(&mut g, board::project_done(0), 1);
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
    // Horizon (282): 2 cubes for 4 energy (262: 3 for 5; r101-r122: 3 cubes + 1 energy for 1 antimatter; r94: 2
    // cubes; r88: 1 cube for 2).
    assert_eq!(
        (g.resource(0, Resource::ResearchCubes), g.resource(0, Resource::Energy), g.resource(0, Resource::Antimatter)),
        (1, e + 4, a)
    );
    // r75 (content): Horizon's converter is once a round too (105 had no limit on it).
    assert_eq!(board::converter_blocked(&w, &g, 0, 0), Some("used this round"));
    assert!(!offered(&g));
}

/// 292: each city that is neither corrupted nor blocked pays its energy to every player with a node on or next to
/// it, with no limit a player; an agent on it or a blackout blocks it, and a corrupted city pays no player. The
/// energy income track (`city_income`) shows it before the income step.
#[test]
fn a_city_pays_every_player_with_a_node_on_or_next_to_it() {
    let (rules, g0) = lab(&["board.produce"]);
    let w = rules.world();
    let (city, n) = city_with_lonely_neighbors(&rules.map);
    let made = city_energy(&rules, city);
    assert!(made > 0);
    let base = [board::city_income(&w, &g0, 0), board::city_income(&w, &g0, 1)];
    let res = |g: &GameState, s: usize| g.resource(s, Resource::Energy) - g0.resource(s, Resource::Energy) - base[s];

    // Two players beside one city: it pays both.
    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    put(&mut g, n[1], node(1), 1);
    assert_eq!((board::city_income(&w, &g, 0) - base[0], board::city_income(&w, &g, 1) - base[1]), (made, made));
    run(&rules, &mut g);
    assert_eq!((res(&g, 0), res(&g, 1)), (made, made));

    // An agent on it blocks it for everyone.
    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    put(&mut g, n[1], node(1), 1);
    put(&mut g, city, ADV_AMBASSADOR, 1);
    assert_eq!(board::city_income(&w, &g, 0), base[0]);
    run(&rules, &mut g);
    assert_eq!((res(&g, 0), res(&g, 1)), (0, 0));

    // A blackout blocks it too.
    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    set_var(&mut g, board::hex_flag(content::RoundFlag::Blackout, city), 1);
    assert_eq!(board::city_income(&w, &g, 0), base[0]);
    run(&rules, &mut g);
    assert_eq!(res(&g, 0), 0);

    // A corrupted city pays no player.
    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    put(&mut g, city, ADV_CUBE, 1);
    assert_eq!(board::city_income(&w, &g, 0), base[0]);
    run(&rules, &mut g);
    assert_eq!(res(&g, 0), 0);

    // r114's ownership (`cities_unowned` 0): an owned city pays its owner alone, and the track agrees with the
    // payout there too, an agent on it darkening it.
    let (rules, g0) = lab_with(&["board.produce"], &["board.cities_unowned=0"]);
    let w = rules.world();
    for agent in [0, 1] {
        let mut g = g0.clone();
        put(&mut g, city, cube(1), 1);
        put(&mut g, n[0], node(0), 1);
        put(&mut g, city, ADV_AMBASSADOR, agent);
        let track = [board::city_income(&w, &g, 0), board::city_income(&w, &g, 1)];
        let before = [g.resource(0, Resource::Energy), g.resource(1, Resource::Energy)];
        run(&rules, &mut g);
        let paid = [g.resource(0, Resource::Energy) - before[0], g.resource(1, Resource::Energy) - before[1]];
        assert_eq!(paid, track, "agent {agent}");
        assert_eq!(paid[1] > 0, agent == 0, "the owner, unless an agent darkens it");
    }
}
