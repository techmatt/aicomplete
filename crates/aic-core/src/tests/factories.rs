//! Factories, troops and chips; the adversary's Grow (275: it replaced city growth).

use super::*;
use crate::modules::board;
use crate::modules::adversary;
use crate::op::RallyAt;

/// 275 (Matt, 2026-11-18): city growth and its chips are gone: a production round builds nothing on its own.
#[test]
fn production_grows_no_city() {
    let (rules, mut g) = lab(&["board.bases"]);
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
    with_cx(&rules, |cx| adversary::grow(&mut g, cx, RallyAt::City, Some(0))).unwrap();
    assert_eq!(built(&g), 1, "one facility in the country");
    assert_eq!((g.tokens(at(11), FACTORY), g.tokens(at(11), ADV_AMBASSADOR), cube_on(&g, at(11))), (1, 1, None));
    assert_eq!(g.stats.counters["adversary.grew_infested"], 1);

    // No agents anywhere: the lowest-numbered city (6) grows, with no agent.
    let mut g = g0.clone();
    with_cx(&rules, |cx| adversary::grow(&mut g, cx, RallyAt::City, Some(0))).unwrap();
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
    with_cx(&rules, |cx| adversary::grow(&mut g, cx, RallyAt::City, Some(0))).unwrap();
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

/// 137: corp troops come from Discord tips: each neutral factory makes 1 on
/// each of its corp's bases; production makes none.
#[test]
fn a_discord_tip_makes_a_troop_at_each_of_a_factorys_corp_bases() {
    let (rules, mut g) = lab_with(&["board.bases"], &["countries=3"]);
    let m = &rules.map;
    for c in 0..3 {
        for &z in &piece(c).industry {
            put(&mut g, m.numbered(c, z), FACTORY, 1);
        }
        // Only country 0's first zone keeps its factory.
        for &z in &piece(c).industry {
            if !(c == 0 && z == piece(0).factories[0]) {
                put(&mut g, m.numbered(c, z), FACTORY, 0);
                put(&mut g, m.numbered(c, z), STATION, 1);
            }
        }
    }
    let bases: Vec<usize> = hexes_where(m, |h| {
        kind(m, h) == HexKind::Base && m.hexes[h].country == 0
    });
    assert_eq!(bases.len(), 2);
    assert_eq!(bases, board::corp_bases(&rules.world(), 0));
    run(&rules, &mut g);
    assert_eq!(total(&g, UNIT), 0, "production makes no corp troop");
    next_alignment_card(&mut g, Side::Discord, "propaganda");
    fill_bucket(&rules, &mut g, Side::Discord);
    any_action(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.var("alignment.discord_tips"), 1);
    for b in bases {
        assert_eq!(g.tokens(b, UNIT), 1, "1 from the factory");
    }
    let other: Vec<usize> = hexes_where(m, |h| {
        kind(m, h) == HexKind::Base && m.hexes[h].country != 0
    });
    assert!(other.iter().all(|&b| g.tokens(b, UNIT) == 0));
}

/// r80 (ruling #50): at a Discord tip each base of a country with an unblocked neutral factory gains 1 drone, once
/// a country however many such factories it has. An agent or a player ambassador on the factory blocks it and
/// stays; an occupied, player or adversary factory isn't neutral; a player ambassador on a base stops its drone and
/// is used up; a full base takes none and keeps its ambassador.
#[test]
fn what_stops_a_tip_troop() {
    let (rules, mut g0) = lab(&[]);
    let w = rules.world();
    let f = rules.map.numbered(0, piece(0).factories[0]);
    put(&mut g0, f, FACTORY, 1);
    let bases = board::corp_bases(&w, 0);
    assert!(!bases.is_empty());
    let tip = |g: &mut GameState| with_cx(&rules, |cx| board::tip_corp_troops(g, cx)).unwrap();
    let units = |g: &GameState| bases.iter().map(|&b| g.tokens(b, UNIT)).sum::<i64>();
    let n = bases.len() as i64;

    let mut g = g0.clone();
    tip(&mut g);
    assert_eq!(units(&g), n);

    // A second neutral factory in the country adds nothing: one drone a base.
    let mut g = g0.clone();
    let f2 = piece(0).industry.iter().map(|&z| rules.map.numbered(0, z)).find(|&h| h != f && g.tokens(h, FACTORY) == 0);
    if let Some(f2) = f2 {
        put(&mut g, f2, FACTORY, 1);
        tip(&mut g);
        assert_eq!(units(&g), n, "once a country");
    }

    let mut g = g0.clone();
    put(&mut g, f, ADV_AMBASSADOR, 1);
    tip(&mut g);
    assert_eq!((units(&g), g.tokens(f, ADV_AMBASSADOR)), (0, 1));

    let mut g = g0.clone();
    put(&mut g, f, ambassador(0), 1);
    tip(&mut g);
    assert_eq!((units(&g), g.tokens(f, ambassador(0))), (0, 1), "a player ambassador blocks it and stays");

    for (tok, label) in [(TROOP, "occupied"), (cube(0), "a player's"), (ADV_CUBE, "the adversary's")] {
        let mut g = g0.clone();
        put(&mut g, f, tok, 1);
        if tok == TROOP {
            set_var(&mut g, board::occupied_key(f), 1);
        }
        tip(&mut g);
        assert_eq!(units(&g), 0, "{label}");
    }

    let mut g = g0.clone();
    put(&mut g, bases[0], ambassador(1), 1);
    tip(&mut g);
    assert_eq!((g.tokens(bases[0], UNIT), g.tokens(bases[0], ambassador(1))), (0, 0));
    assert_eq!(units(&g), n - 1);
    assert_eq!(g.stats.counters["board.base_ambassador_used"], 1);

    // 145: a full base makes no troop, so its ambassador stays.
    let mut g = g0.clone();
    let cap = w.int("board.unit_cap");
    put(&mut g, bases[0], UNIT, cap);
    put(&mut g, bases[0], ambassador(1), 1);
    tip(&mut g);
    assert_eq!((g.tokens(bases[0], UNIT), g.tokens(bases[0], ambassador(1))), (cap, 1));
}

#[test]
fn an_adversary_factory_makes_adversary_troops() {
    let (rules, mut g) = lab(&["board.produce", "board.bases"]);
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
    let [b0, _] = m.fronts[0].bases;
    run(&rules, &mut g);
    // 137: and no corp troop at its corp's base.
    assert_eq!((g.tokens(f, TROOP), g.tokens(b0, UNIT)), (1, 0));
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
