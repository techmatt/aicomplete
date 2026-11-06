use super::*;
use crate::modules::{adversary, board};

#[test]
fn cities_power_the_player_with_the_most_node_next_to_them() {
    let (rules, g0) = lab(&["board.produce"]);
    let (city, n) = city_with_lonely_neighbors(&rules.map);
    let res = |g: &GameState, s: usize, r: Resource| g.resource(s, r) - g0.resource(s, r);

    // 021: a neutral city's cube is lost unless a player with a node next
    // to it buys it (`city_energy_credits`, 5 at r36; players choose who).
    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    put(&mut g, n[1], node(1), 1);
    run(&rules, &mut g);
    assert!(g.pending.is_none(), "nobody can pay");
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 0));
    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    put(&mut g, n[1], node(1), 1);
    set_res(&mut g, 0, Resource::Credits, 5);
    set_res(&mut g, 1, Resource::Credits, 8);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("an energy choice");
    assert!(p.ask.prompt.starts_with("The energy of"), "{}", p.ask.prompt);
    answer_with(&rules, &mut g, "seat 1 buys");
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 1));
    assert_eq!(g.resource(1, Resource::Credits), 3, "8 less the price, 5");
    let mut g = g0.clone();
    put(&mut g, n[0], node(0), 1);
    set_res(&mut g, 0, Resource::Credits, 5);
    run(&rules, &mut g);
    answer_with(&rules, &mut g, "nobody");
    assert_eq!((res(&g, 0, Resource::Energy), g.resource(0, Resource::Credits)), (0, 5));

    // An aligned city's cube goes free to its owner.
    let mut g = g0.clone();
    put(&mut g, city, cube(1), 1);
    put(&mut g, n[0], node(0), 1);
    run(&rules, &mut g);
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 1));
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
    run(&rules, &mut g);
    // 9 cubes on an 8-cube card: all pay. Horizon (075): 3 energy and 3 credits
    // a player cube, 1 Discord an adversary cube, nothing for a neutral cube.
    // 127: no holding limit, so seat 0 keeps all 12.
    assert_eq!((g.resource(0, Resource::Energy) - e0, g.resource(1, Resource::Energy) - e1), (12, 3));
    assert_eq!(g.var("alignment.discord") - discord, 2);
    assert_eq!(g.var(&board::project_done(0)), 1);
    for k in [board::project_key(0, Some(0)), board::project_key(0, Some(1)), board::project_key(0, None), board::project_neutral(0)] {
        assert_eq!(g.var(&k), 0);
    }
    assert_eq!(g.stats.counters["board.cubes_paid.player"], 5);
    assert_eq!(g.stats.counters["board.cubes_paid.adversary"], 2);
    assert_eq!(g.stats.counters["board.cubes_paid.neutral"], 2);
    // Afterward the owned station's cube goes to its owner's board, and stays
    // between rounds.
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::ResearchCubes), 1);
    assert_eq!(g.var(&board::project_key(0, Some(0))), 0);
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::ResearchCubes), 2);
}

#[test]
fn a_project_short_of_its_cubes_does_not_complete() {
    let (rules, mut g) = lab(&["board.produce"]);
    owned_station(&rules, &mut g, 0, 0);
    // Round 1 already put a corrupted and a neutral station's cubes on it.
    g.vars.insert(board::project_key(0, Some(0)).into(), 4);
    run(&rules, &mut g);
    assert_eq!(g.var(&board::project_done(0)), 0);
    assert_eq!(g.var(&board::project_key(0, Some(0))), 5);
}

#[test]
fn a_neutral_station_makes_a_neutral_cube_unless_an_adversary_ambassador_takes_it() {
    let (rules, mut g) = lab(&["board.produce"]);
    let h = rules.map.numbered(0, piece(0).neutral_station);
    put(&mut g, h, STATION, 1);
    let k = board::project_key(0, None);
    let (neutral, adversary) = (g.var(&board::project_neutral(0)), g.var(&k));
    run(&rules, &mut g);
    assert_eq!(g.var(&board::project_neutral(0)), neutral + 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!(g.var(&board::project_neutral(0)), neutral + 1);
    assert_eq!(g.var(&k), adversary + 1);
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 0);
}

/// 075 (Matt): no corp prints an after line, so an adversary cube made after
/// completion is lost, as a neutral one is.
#[test]
fn adversary_cubes_after_completion_are_lost() {
    let (rules, mut g) = lab(&["board.produce"]);
    for c in 0..2 {
        g.vars.insert(board::project_done(c).into(), 1);
        let h = rules.map.numbered(c, piece(c).stations[0]);
        put(&mut g, h, STATION, 1);
        put(&mut g, h, ADV_CUBE, 1);
    }
    let (made, discord) = (g.var(adversary::COMPUTE), g.var("alignment.discord"));
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), made);
    assert_eq!(g.var("alignment.discord"), discord);
    assert_eq!(g.stats.counters["board.cubes_after"], 2);
}

/// The rule still reads an after line content prints (074's, by overlay).
#[test]
fn adversary_cubes_convert_at_once_after_completion_as_the_card_prints() {
    let (rules, mut g) = lab_with(
        &["board.produce"],
        &[r#"corps.horizon.project.after=[{"op":"compute","amount":2}]"#, r#"corps.cintamani.project.after=[{"op":"compute_every","amount":1}]"#],
    );
    // Onyx is not in a 2-country game: Horizon's 2, Cintamani's every project.
    for c in 0..2 {
        g.vars.insert(board::project_done(c).into(), 1);
        let h = rules.map.numbered(c, piece(c).stations[0]);
        put(&mut g, h, STATION, 1);
        put(&mut g, h, ADV_CUBE, 1);
    }
    let visible = adversary::projects(&g).len() as i64;
    assert!(visible >= 2);
    let made = g.var(adversary::COMPUTE);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE) - made, 2 + visible);
    assert_eq!(g.stats.counters["adversary.compute_corp"], 2 + visible);
}

#[test]
fn a_converter_needs_node_in_a_city_of_its_country() {
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
    // r37: a node off the cities, here the starting node's hex, is not enough.
    let open = rules.map.numbered(0, piece(0).node);
    assert!(!crate::modules::common::is_city(&w, open));
    put(&mut g, open, node(0), 1);
    assert_eq!(board::converter_blocked(&w, &g, 0, 0), Some(board::NO_CITY_NODE));
    let city = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, city, cube(0), 1);
    assert!(!offered(&g), "an aligned city is no longer the gate");
    put(&mut g, city, node(0), 1);
    assert!(offered(&g));
    let (e, a) = (g.resource(0, Resource::Energy), g.resource(0, Resource::Antimatter));
    rules.apply_mut(&mut g, &convert, None).unwrap();
    // Horizon: 1 cube + 2 energy, 1 antimatter.
    assert_eq!(
        (g.resource(0, Resource::ResearchCubes), g.resource(0, Resource::Energy), g.resource(0, Resource::Antimatter)),
        (2, e - 2, a + 1)
    );
    // 105: Horizon's converter has no limit a round (only Onyx's keeps one,
    // tests/engine.rs): it is offered again while the seat can pay.
    assert_eq!(board::converter_blocked(&w, &g, 0, 0), None);
    assert!(offered(&g));
}
