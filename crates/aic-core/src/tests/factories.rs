//! Factories, troops and chips (293: the adversary's Grow line is gone; a corrupted factory's agent: adversary.rs).

use super::*;

/// 275 (Matt, 2026-11-18): city growth and its chips are gone: a production round builds nothing on its own.
#[test]
fn production_grows_no_city() {
    let (rules, mut g) = lab(&["board.produce"]);
    let before = (total(&g, FACTORY), total(&g, STATION));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((total(&g, FACTORY), total(&g, STATION)), before);
}


#[test]
fn player_factories_make_chips_every_round() {
    // 139: production runs every round (no clock). r75: a factory's output is in
    // the income step, `board.produce`.
    let (rules, mut g0) = lab(&["board.produce"]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g0, f, FACTORY, 1);
    put(&mut g0, f, cube(0), 1);
    // Every zone taken, so nothing grows.
    for c in 0..2 {
        for &z in &piece(c).industry {
            put(&mut g0, m.numbered(c, z), FACTORY, 1);
        }
    }
    for round in [1, 2] {
        let mut g = g0.clone();
        g.round = round;
        run(&rules, &mut g);
        let chips = rules.world().int("board.factory_chips");
        assert_eq!(chips, 1, "190: a player's factory makes 1 chip");
        assert_eq!(g.resource(0, Resource::Chips) - g0.resource(0, Resource::Chips), chips, "round {round}");
    }
}

/// 293 (Matt, 2026-11-19): an adversary factory makes no troop: production leaves it alone, and its one output
/// (an agent) comes in the adversary's Income beat (tests/adversary.rs).
#[test]
fn an_adversary_factory_makes_no_troop() {
    let (rules, mut g) = lab(&["board.produce"]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f, TROOP), g.tokens(f, ADV_AMBASSADOR)), (0, 0));
    assert!(!g.stats.counters.contains_key("board.troops_built"));
}

/// 135, 293: a player ambassador on an adversary-owned factory blocks its output (no agent is made) and stays;
/// it sways no Unity (136).
#[test]
fn a_player_ambassador_on_an_adversary_factory_stops_its_agent_and_stays() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    put(&mut g, f, ambassador(0), 1);
    let unity = g.var("alignment.unity");
    with_cx(&rules, |cx| crate::modules::adversary::factories_output(&mut g, cx)).unwrap();
    assert_eq!((g.tokens(f, ADV_AMBASSADOR), g.tokens(f, TROOP), g.tokens(f, ambassador(0))), (0, 0, 1));
    assert_eq!(g.var("alignment.unity") - unity, 0);
    assert_eq!(g.stats.counters["board.blocked.factory_adversary"], 1);
}
