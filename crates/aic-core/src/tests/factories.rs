//! Factories, troops and chips.

use super::*;
use crate::modules::board;

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

    // Clean 6 builds at 2 (its only zone), neutral (no nodes near);
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
    put(&mut gg, c6, cube(1), 1);
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
    put(&mut g, at(8), ambassador(0), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 8), (1, adv));
}

#[test]
fn a_clean_citys_factory_goes_to_the_most_node_by_its_zone() {
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

    // Nobody's node near: neutral.
    let mut g = g0.clone();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(owners(&g), [None, None, None]);

    // Seat 1 next to 2; seat 1 on 11 against seat 0 on 10 (next to it), a
    // tie the players settle; seat 0 on 9 and 10, both next to 13.
    let mut g = g0.clone();
    put(&mut g, at(1), node(1), 1);
    put(&mut g, at(11), node(1), 1);
    put(&mut g, at(9), node(0), 1);
    put(&mut g, at(10), node(0), 1);
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

    // An aligned city's factory stays its owner's, whatever node is near.
    let mut g = g0.clone();
    put(&mut g, at(6), cube(0), 1);
    put(&mut g, at(1), node(1), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(owners(&g)[0], c(0));
}

#[test]
fn player_factories_make_chips_on_production_steps_only() {
    // 137: the clock is 3 (both) every round; a war-only dial makes none.
    let setup = |sets: &[&str]| {
        let (rules, mut g0) = lab_with(&["board.bases"], sets);
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
        (rules, g0)
    };
    let (rules, g0) = setup(&[]);
    for round in [1, 2] {
        let mut g = g0.clone();
        g.round = round;
        run(&rules, &mut g);
        assert_eq!(g.resource(0, Resource::Chips) - g0.resource(0, Resource::Chips), 1, "round {round}");
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

/// 137: at a tip, one opposing ambassador blocks a neutral factory's troop
/// and stays; a player ambassador on it is used up instead; an occupied,
/// player or adversary factory makes none; a player ambassador on a base is
/// used up instead of the troop there.
#[test]
fn what_stops_a_tip_troop() {
    let (rules, mut g0) = lab(&[]);
    let w = rules.world();
    let f = rules.map.numbered(0, piece(0).factories[0]);
    put(&mut g0, f, FACTORY, 1);
    let bases = board::corp_bases(&w, 0);
    assert!(!bases.is_empty());
    let tip = |g: &mut GameState| with_cx(&rules, |cx| board::tip_corp_troops(g, cx));
    let units = |g: &GameState| bases.iter().map(|&b| g.tokens(b, UNIT)).sum::<i64>();
    let n = bases.len() as i64;

    let mut g = g0.clone();
    tip(&mut g);
    assert_eq!(units(&g), n);

    let mut g = g0.clone();
    put(&mut g, f, ADV_AMBASSADOR, 1);
    tip(&mut g);
    assert_eq!((units(&g), g.tokens(f, ADV_AMBASSADOR)), (0, 1));

    let mut g = g0.clone();
    put(&mut g, f, ambassador(0), 1);
    tip(&mut g);
    assert_eq!((units(&g), g.tokens(f, ambassador(0))), (0, 0));
    assert_eq!(g.stats.counters["board.factory_ambassador_used"], 1);

    for (tok, label) in [(TROOP, "occupied"), (cube(0), "a player's"), (ADV_CUBE, "the adversary's")] {
        let mut g = g0.clone();
        put(&mut g, f, tok, 1);
        if tok == TROOP {
            g.vars.insert(crate::key::Key::new(&board::occupied_key(f)), 1);
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
    // 137: and no corp troop at its corp's base.
    assert_eq!((g.tokens(f, TROOP), g.tokens(b0, UNIT)), (1, 0));
}

/// 135 (Matt's ruling 3: it frees as a corrupted station does): a player
/// ambassador on an adversary-owned factory takes the factory's output, so no
/// troop is made. 068: it stays. 136: it sways no Unity (it swayed `factory_unity`).
#[test]
fn a_player_ambassador_on_an_adversary_factory_stops_its_troop_and_stays() {
    let (rules, mut g) = lab(&["board.bases"]);
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
/// (028: one was used up making it).
#[test]
fn ambassadors_on_an_adversary_factory_stay_as_it_makes_its_troop() {
    let (rules, mut g) = lab(&["board.bases"]);
    let f = rules.map.numbered(0, piece(0).factories[0]);
    put(&mut g, f, FACTORY, 1);
    put(&mut g, f, ADV_CUBE, 1);
    put(&mut g, f, ADV_AMBASSADOR, 2);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f, TROOP), g.tokens(f, ADV_AMBASSADOR)), (1, 2));
}
