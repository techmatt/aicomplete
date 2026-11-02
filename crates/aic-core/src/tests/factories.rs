//! Factories, troops and chips.

use super::*;

#[test]
fn cities_grow_factories_that_take_their_owner() {
    let (rules, g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0: cities 6, 7, 14; zones 2 (by 6), 8 and 11 (by 7), 11 and 13 (by 14).
    // Cities build in board order: 6, 7, 14.
    assert_eq!(
        (piece(0).cities.clone(), piece(0).industry.clone()),
        (vec![7, 6, 14], vec![2, 8, 11, 13])
    );
    let (c6, c7) = (at(6), at(7));
    let zones =
        |g: &GameState| [2, 8, 11, 13].map(|n| (g.tokens(at(n), FACTORY), cube_on(g, at(n))));

    // Clean 6 builds at 2 (its only zone), neutral (no presence near);
    // corrupted 7 and 14 at the lowest-numbered free zones, 8 then 11,
    // adversary-owned.
    let mut g = g0.clone();
    put(&mut g, c7, ADV_CUBE, 1);
    put(&mut g, at(14), ADV_CUBE, 1);
    let mut gg = g.clone();
    run(&rules, &mut gg);
    answer_all(&rules, &mut gg);
    let adv = Some("cube:adversary".to_string());
    assert_eq!(
        zones(&gg),
        [(1, None), (1, adv.clone()), (1, adv.clone()), (0, None)]
    );

    // An aligned city's factory is its owner's.
    let mut gg = g0.clone();
    put(&mut gg, c6, &cube(1), 1);
    run(&rules, &mut gg);
    answer_all(&rules, &mut gg);
    assert_eq!(zones(&gg)[0], (1, Some("cube:1".into())));

    // A clean city with two free zones: the players choose (seat 0).
    let mut gg = g0.clone();
    run(&rules, &mut gg);
    let p = gg.pending.as_ref().expect("a factory choice");
    assert!(
        p.ask.prompt.starts_with(&format!("{} #7 builds a factory", piece(0).name)),
        "{}",
        p.ask.prompt
    );
    let options: BTreeSet<usize> = p
        .ask
        .options
        .iter()
        .map(|o| m.find(o.target.unwrap()).unwrap())
        .collect();
    assert_eq!(options, [at(8), at(11)].into());
    answer_hex(&rules, &mut gg, at(11));
    // 14 is then left only 13.
    answer_all(&rules, &mut gg);
    assert_eq!(zones(&gg).map(|z| z.0), [1, 0, 1, 1]);
}

fn cube_on(g: &GameState, h: usize) -> Option<String> {
    g.hexes[h]
        .tokens
        .keys()
        .find(|k| k.starts_with("cube:"))
        .map(|k| k.to_string())
}

#[test]
fn a_corrupted_city_builds_next_to_an_adversary_ambassador_else_at_its_lowest_free_zone() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0 (see above): city 6 builds first; zone 2 is taken, so its
    // own neighborhood has no free zone.
    put(&mut g0, at(2), STATION, 1);
    put(&mut g0, at(6), ADV_CUBE, 1);
    let adv = Some("cube:adversary".to_string());
    let zone = |g: &GameState, n: u8| (g.tokens(at(n), FACTORY), cube_on(g, at(n)));

    // No adversary ambassador: the lowest-numbered free zone, 8, even though
    // it is not next to the city; then 7 builds at 11 and 14 at 13, neutral.
    let mut g = g0.clone();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert!(!m.adjacent(at(6), at(8)));
    assert_eq!(zone(&g, 8), (1, adv.clone()));
    assert_eq!((zone(&g, 11), zone(&g, 13)), ((1, None), (1, None)));

    // An adversary ambassador next to 13 (on 9) draws it there, past 8 and 11.
    let mut g = g0.clone();
    put(&mut g, at(9), ADV_AMBASSADOR, 1);
    assert!(m.adjacent(at(9), at(13)) && !m.adjacent(at(9), at(8)) && !m.adjacent(at(9), at(11)));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 13), (1, adv.clone()));
    assert_ne!(zone(&g, 8).1, adv);
    assert_ne!(zone(&g, 11).1, adv);

    // An ambassador on a zone counts too; among two drawn zones (13 on one,
    // 11 next to the one on 14; 8 next to neither), the lower.
    let mut g = g0.clone();
    put(&mut g, at(13), ADV_AMBASSADOR, 1);
    put(&mut g, at(14), ADV_AMBASSADOR, 1);
    assert!(!m.adjacent(at(8), at(14)) && !m.adjacent(at(8), at(13)));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 11), (1, adv.clone()));
    assert_ne!(zone(&g, 13).1, adv);
    // A player ambassador no longer repels it.
    let mut g = g0.clone();
    put(&mut g, at(8), &ambassador(0), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 8), (1, adv));
}

#[test]
fn a_clean_citys_factory_goes_to_the_most_presence_by_its_zone() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0 (see above), 8 taken: 6 builds at 2, 7 at 11, 14 at 13.
    put(&mut g0, at(8), STATION, 1);
    let owners = |g: &GameState| {
        [2, 11, 13].map(|n| {
            assert_eq!(g.tokens(at(n), FACTORY), 1, "zone {n}");
            cube_on(g, at(n))
        })
    };

    // Nobody's presence near: neutral.
    let mut g = g0.clone();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(owners(&g), [None, None, None]);

    // Seat 1 next to 2; seat 1 on 11 against seat 0 on 10 (next to it), a
    // tie the players settle; seat 0 on 9 and 10, both next to 13.
    let mut g = g0.clone();
    put(&mut g, at(1), &presence(1), 1);
    put(&mut g, at(11), &presence(1), 1);
    put(&mut g, at(9), &presence(0), 1);
    put(&mut g, at(10), &presence(0), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a tie to settle");
    assert!(
        p.ask
            .prompt
            .starts_with(&format!("The factory at {} #11 goes to which player?", piece(0).name)),
        "{}",
        p.ask.prompt
    );
    assert_eq!(p.ask.options.len(), 2);
    answer_with(&rules, &mut g, "seat 1");
    answer_all(&rules, &mut g);
    let c = |s: usize| Some(cube(s).to_string());
    assert_eq!(owners(&g), [c(1), c(1), c(0)]);
    let built = |g: &GameState| {
        g.stats
            .counters
            .get("board.factories_built.player")
            .copied()
    };
    assert_eq!(built(&g).unwrap_or(0) - built(&g0).unwrap_or(0), 3);

    // An aligned city's factory stays its owner's, whatever presence is near.
    let mut g = g0.clone();
    put(&mut g, at(6), &cube(0), 1);
    put(&mut g, at(1), &presence(1), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(owners(&g)[0], c(0));
}

#[test]
fn player_factories_make_chips_on_production_steps_only() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g0, f, FACTORY, 1);
    put(&mut g0, f, &cube(0), 1);
    // Every zone taken, so nothing grows.
    for c in 0..2 {
        for &z in &piece(c).industry {
            put(&mut g0, m.numbered(c, z), FACTORY, 1);
        }
    }
    let mut g = g0.clone();
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Chips) - g0.resource(0, Resource::Chips), 1);
    // Round 2 is a war step only: no chips, troops or growth.
    let mut g = g0.clone();
    g.round = 2;
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Chips), g0.resource(0, Resource::Chips));
    assert_eq!(total(&g, UNIT), 0);
}

#[test]
fn a_factory_makes_a_troop_at_each_of_its_corps_bases() {
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
    run(&rules, &mut g);
    for b in bases {
        assert_eq!(g.tokens(b, UNIT), 2, "1 from the base, 1 from the factory");
    }
    let other: Vec<usize> = hexes_where(m, |h| {
        kind(m, h) == HexKind::Base && m.hexes[h].country != 0
    });
    assert!(other.iter().all(|&b| g.tokens(b, UNIT) == 1));
}

#[test]
fn an_adversary_factory_makes_adversary_troops() {
    let (rules, mut g) = lab(&["board.bases"]);
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
    assert_eq!((g.tokens(f, TROOP), g.tokens(b0, UNIT)), (1, 1));
}

/// 022 (D1): a player ambassador on an adversary-owned factory sways
/// `factory_unity` Unity, and the factory still makes its troop. 068: it stays.
#[test]
fn a_player_ambassador_on_an_adversary_factory_sways_unity_and_stays() {
    let (rules, mut g) = lab(&["board.bases"]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    put(&mut g, f, &ambassador(0), 1);
    let unity = g.var("alignment.unity");
    run(&rules, &mut g);
    assert_eq!((g.tokens(f, TROOP), g.tokens(f, &ambassador(0))), (1, 1));
    assert_eq!(g.var("alignment.unity") - unity, 1);
}

#[test]
fn a_troop_at_a_base_absorbs_a_strike_next_to_it() {
    let (rules, mut g) = lab(&["board.march"]);
    let m = &rules.map;
    let [b0, _] = m.fronts[0].bases;
    let next = m.hexes[b0]
        .neighbors
        .iter()
        .copied()
        .find(|&h| kind(m, h) == HexKind::Open)
        .unwrap();
    put(&mut g, next, FACTORY, 1);
    put(&mut g, next, &cube(0), 1);
    put(&mut g, next, TROOP, 1);
    put(&mut g, b0, UNIT, 1);
    let discord = g.var("alignment.discord");
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(next, FACTORY),
            g.tokens(b0, UNIT),
            g.var("alignment.discord")
        ),
        (1, 0, discord)
    );
    assert_eq!(g.stats.counters["board.base_absorbed"], 1);
}

/// 028: an adversary-owned factory's ambassador is used up making its troop.
#[test]
fn an_ambassador_on_an_adversary_factory_is_used_up_making_its_troop() {
    let (rules, mut g) = lab(&["board.bases"]);
    let f = rules.map.numbered(0, piece(0).factories[0]);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    put(&mut g, f, ADV_AMBASSADOR, 2);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f, TROOP), g.tokens(f, ADV_AMBASSADOR)), (1, 1));
}
