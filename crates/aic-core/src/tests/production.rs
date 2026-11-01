use super::*;
use crate::modules::board;

#[test]
fn cities_power_the_player_with_the_most_presence_next_to_them() {
    let (rules, g0) = lab(&["board.produce"]);
    let (city, n) = city_with_lonely_neighbors(&rules.map);
    let res = |g: &GameState, s: usize, r: Resource| g.resource(s, r) - g0.resource(s, r);

    // 021: a neutral city's cube is lost unless a player with presence next
    // to it buys it (`city_energy_credits`, 2; players choose who).
    let mut g = g0.clone();
    put(&mut g, n[0], &presence(0), 1);
    put(&mut g, n[1], &presence(1), 1);
    run(&rules, &mut g);
    assert!(g.pending.is_none(), "nobody can pay");
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 0));
    let mut g = g0.clone();
    put(&mut g, n[0], &presence(0), 1);
    put(&mut g, n[1], &presence(1), 1);
    set_res(&mut g, 0, Resource::Credits, 5);
    set_res(&mut g, 1, Resource::Credits, 5);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("an energy choice");
    assert!(p.ask.prompt.starts_with("The energy of"), "{}", p.ask.prompt);
    answer_with(&rules, &mut g, "seat 1 buys");
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 1));
    assert_eq!(g.resource(1, Resource::Credits), 3);
    let mut g = g0.clone();
    put(&mut g, n[0], &presence(0), 1);
    set_res(&mut g, 0, Resource::Credits, 5);
    run(&rules, &mut g);
    answer_with(&rules, &mut g, "nobody");
    assert_eq!((res(&g, 0, Resource::Energy), g.resource(0, Resource::Credits)), (0, 5));

    // An aligned city's cube goes free to its owner.
    let mut g = g0.clone();
    put(&mut g, city, &cube(1), 1);
    put(&mut g, n[0], &presence(0), 1);
    run(&rules, &mut g);
    assert_eq!((res(&g, 0, Resource::Energy), res(&g, 1, Resource::Energy)), (0, 1));
}

#[test]
fn stations_fill_the_megaproject_and_its_completion_pays_out() {
    let (rules, mut g) = lab(&["board.produce"]);
    let m = &rules.map;
    let station = m.numbered(0, piece(0).stations[0]);
    put(&mut g, station, STATION, 1);
    put(&mut g, station, &cube(0), 1);
    let rival = m.numbered(1, piece(1).stations[0]);
    put(&mut g, rival, STATION, 1);
    put(&mut g, rival, ADV_CUBE, 1);
    assert_eq!(piece(0).megaproject.cubes, 7);
    g.vars.insert(board::project_key(0, Some(0)).into(), 3);
    g.vars.insert(board::project_key(0, Some(1)).into(), 1);
    g.vars.insert(board::project_key(0, None).into(), 2);
    g.vars.remove(board::project_key(1, None).as_str());
    let (c0, c1, discord) = (
        g.resource(0, Resource::Credits),
        g.resource(1, Resource::Credits),
        g.var("alignment.discord"),
    );
    run(&rules, &mut g);
    // The 7th cube completes it: 10 credits a player cube, 1 Discord an adversary cube.
    assert_eq!(
        (g.resource(0, Resource::Credits) - c0, g.resource(1, Resource::Credits) - c1),
        (40, 10)
    );
    assert_eq!(g.var("alignment.discord") - discord, 2);
    assert_eq!(g.var(&board::project_done(0)), 1);
    for s in [Some(0), Some(1), None] {
        assert_eq!(g.var(&board::project_key(0, s)), 0);
    }
    assert_eq!(g.var(&board::project_key(1, None)), 1);
    // Afterward the owned station makes the project's benefit.
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Credits) - c0, 42);
    assert_eq!(g.var(&board::project_key(0, Some(0))), 0);
    assert_eq!(g.var(&board::project_key(1, None)), 2);
}
