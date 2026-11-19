//! Factories, troops and chips; the adversary's Grow (275: it replaced city growth).

use super::*;
use crate::modules::adversary;
use crate::op::SiteType;

/// 275 (Matt, 2026-11-18): city growth and its chips are gone: a production round builds nothing on its own.
#[test]
fn production_grows_no_city() {
    let (rules, mut g) = lab(&["board.produce"]);
    let before = (total(&g, FACTORY), total(&g, STATION));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((total(&g, FACTORY), total(&g, STATION)), before);
}

/// 275: Grow: X builds one neutral facility a country, by the X holding the most agents, the type it has fewer of
/// next to it (ties: a factory) on its lowest-numbered free hex; a site holding agents builds it with 1 agent on it.
#[test]
fn grow_builds_one_facility_by_the_site_with_most_agents() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0: cities 6, 7, 14; zones 2 (by 6), 8 and 11 (by 7), 11 and 13 (by 14).
    assert_eq!(
        (piece(0).cities.clone(), piece(0).industry.clone()),
        (vec![7, 6, 14], vec![2, 8, 11, 13])
    );
    let built = |g: &GameState| total(g, FACTORY) + total(g, STATION) - total(&g0, FACTORY) - total(&g0, STATION);

    // 14 holds the most: a factory on 11, an agent on it.
    let mut g = g0.clone();
    set_var(&mut g, SUPPLY, 40);
    put(&mut g, at(14), ADV_AMBASSADOR, 2);
    put(&mut g, at(7), ADV_AMBASSADOR, 1);
    with_cx(&rules, |cx| adversary::grow(&mut g, cx, SiteType::City, Some(0))).unwrap();
    assert_eq!(built(&g), 1, "one facility in the country");
    assert_eq!((g.tokens(at(11), FACTORY), g.tokens(at(11), ADV_AMBASSADOR), cube_on(&g, at(11))), (1, 1, None));
    assert_eq!(g.stats.counters["adversary.grew_infested"], 1);

    // No agents anywhere: the lowest-numbered city (6) grows, with no agent.
    let mut g = g0.clone();
    with_cx(&rules, |cx| adversary::grow(&mut g, cx, SiteType::City, Some(0))).unwrap();
    assert_eq!(built(&g), 1);
    assert_eq!((g.tokens(at(2), FACTORY), g.tokens(at(2), ADV_AMBASSADOR)), (1, 0));
}

/// 275: a Grow site with a factory next to it and no station builds a station; with no free zone it builds the
/// other type rather than nothing.
#[test]
fn grow_builds_the_type_its_site_has_fewer_of() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let at = |n: u8| m.numbered(0, n);
    let near = |g: &GameState, c: u8, kind: &str| m.hexes[at(c)].neighbors.iter().filter(|&&h| g.tokens(h, kind) > 0).count();
    let mut g = g0.clone();
    put(&mut g, at(2), FACTORY, 1);
    put(&mut g, at(2), cube(1), 1);
    let (s0, f0) = (near(&g, 6, STATION), near(&g, 6, FACTORY));
    assert!(f0 > s0, "6: more factories than stations next to it");
    let (z, factory) = crate::modules::plan::grow_at(&w, &g, at(6)).expect("6 can grow");
    assert!(!factory && m.hexes[at(6)].neighbors.contains(&z));
    put(&mut g, at(6), ADV_AMBASSADOR, 1);
    with_cx(&rules, |cx| adversary::grow(&mut g, cx, SiteType::City, Some(0))).unwrap();
    assert_eq!((near(&g, 6, STATION), near(&g, 6, FACTORY)), (s0 + 1, f0));
    assert_eq!(cube_on(&g, z), None, "neutral");
    // A station on 6's only zone (2): it wants a factory, has no free zone for one, and builds a station.
    let mut g = g0.clone();
    put(&mut g, at(2), STATION, 1);
    assert!(near(&g, 6, FACTORY) < near(&g, 6, STATION));
    let (_, factory) = crate::modules::plan::grow_at(&w, &g, at(6)).expect("6 can grow");
    assert!(!factory);
}


fn cube_on(g: &GameState, h: usize) -> Option<String> {
    g.hexes[h]
        .tokens
        .keys()
        .find(|k| k.starts_with("cube:"))
        .map(|k| k.to_string())
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

#[test]
fn an_adversary_factory_makes_adversary_troops() {
    let (rules, mut g) = lab(&["board.produce"]);
    let m = &rules.map;
    for c in 0..2 {
        for &z in &piece(c).industry {
            put(&mut g, m.numbered(c, z), STATION, 1);
        }
    }
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f, STATION, 0);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    run(&rules, &mut g);
    assert_eq!(g.tokens(f, TROOP), 1);
}

/// 135 (Matt's ruling 3: it frees as a corrupted station does): a player
/// ambassador on an adversary-owned factory takes the factory's output, so no
/// troop is made. 068: it stays. 136: it sways no Unity (it swayed `factory_unity`).
#[test]
fn a_player_ambassador_on_an_adversary_factory_stops_its_troop_and_stays() {
    // r75: the troop is made in the income step, `board.produce`.
    let (rules, mut g) = lab(&["board.produce"]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    put(&mut g, f, ambassador(0), 1);
    let unity = g.var("alignment.unity");
    run(&rules, &mut g);
    assert_eq!((g.tokens(f, TROOP), g.tokens(f, ambassador(0))), (0, 1));
    assert_eq!(g.var("alignment.unity") - unity, 0);
}

/// 137: an adversary factory makes its troop and its own ambassadors stay
/// (028: one was used up making it). r75: in the income step, `board.produce`.
#[test]
fn ambassadors_on_an_adversary_factory_stay_as_it_makes_its_troop() {
    let (rules, mut g) = lab(&["board.produce"]);
    let f = rules.map.numbered(0, piece(0).factories[0]);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    put(&mut g, f, ADV_AMBASSADOR, 2);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f, TROOP), g.tokens(f, ADV_AMBASSADOR)), (1, 2));
}
