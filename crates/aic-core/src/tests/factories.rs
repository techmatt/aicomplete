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

/// r75 (Matt's ruling 4): a corrupted city builds only on a free zone next to it
/// (it searched its whole country): the one next to an agent if any, else the
/// lowest-numbered; with none free it builds nowhere.
#[test]
fn a_corrupted_city_builds_next_to_itself_by_an_agent_else_at_its_lowest_free_zone() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0 (see above): zone 2 is taken, so city 6 (first to build) has no
    // free zone next to it; 8 and 11 are 7's, 11 and 13 are 14's.
    put(&mut g0, at(2), STATION, 1);
    let adv = Some("cube:adversary".to_string());
    let zone = |g: &GameState, n: u8| (g.tokens(at(n), FACTORY), cube_on(g, at(n)));
    let adversary_zones = |g: &GameState| [8, 11, 13].into_iter().filter(|&n| zone(g, n).1 == adv).count();

    // Corrupted 6 builds nowhere, though 8 is free in its country.
    let mut g = g0.clone();
    put(&mut g, at(6), ADV_CUBE, 1);
    assert!(!m.adjacent(at(6), at(8)));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(adversary_zones(&g), 0);

    // Corrupted 7 with no agent: its lowest-numbered free zone, 8.
    let mut g = g0.clone();
    put(&mut g, at(7), ADV_CUBE, 1);
    assert!(m.adjacent(at(7), at(8)) && m.adjacent(at(7), at(11)));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 8), (1, adv.clone()));
    assert_eq!(adversary_zones(&g), 1);

    // An agent next to 11 (on 14), not 8, draws it to 11.
    let mut g = g0.clone();
    put(&mut g, at(7), ADV_CUBE, 1);
    put(&mut g, at(14), ADV_AMBASSADOR, 1);
    assert!(m.adjacent(at(14), at(11)) && !m.adjacent(at(14), at(8)));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 11), (1, adv.clone()));
    assert_ne!(zone(&g, 8).1, adv);

    // A player ambassador doesn't repel it.
    let mut g = g0.clone();
    put(&mut g, at(7), ADV_CUBE, 1);
    put(&mut g, at(8), ambassador(0), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(zone(&g, 8), (1, adv));
}

/// r75 (Matt's ruling 4): a neutral city's factory is neutral (claimable),
/// whatever nodes are near (the player with the most nodes by its zone took it).
#[test]
fn a_neutral_citys_factory_is_neutral_whatever_nodes_are_near() {
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

    // Seat 1 next to 2 and on 11; seat 0 on 9 and 10, both next to 13: still
    // neutral, and nobody is asked who takes it: the only ask is 14's zone
    // (13, or a zone of the next country beside it), seat 0's.
    let mut g = g0.clone();
    put(&mut g, at(1), node(1), 1);
    put(&mut g, at(11), node(1), 1);
    put(&mut g, at(9), node(0), 1);
    put(&mut g, at(10), node(0), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("14's zone");
    assert!(p.ask.seat == 0 && p.ask.prompt.starts_with(&format!("{} #14 builds a factory", piece(0).name)), "{:?}", p.ask);
    answer_hex(&rules, &mut g, at(13));
    // The other country's cities ask only for zones too.
    while let Some(p) = g.pending.as_ref() {
        assert!(p.ask.seat == 0 && p.ask.prompt.contains("builds a factory on which zone?"), "{:?}", p.ask);
        answer(&rules, &mut g, 0);
    }
    assert_eq!(owners(&g), [None, None, None]);
    let built = |g: &GameState| g.stats.counters.get("board.factories_built.player").copied().unwrap_or(0);
    assert_eq!(built(&g) - built(&g0), 0);
    let c = |s: usize| Some(cube(s).to_string());

    // An aligned city's factory stays its owner's, whatever node is near.
    let mut g = g0.clone();
    put(&mut g, at(6), cube(0), 1);
    put(&mut g, at(1), node(1), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(owners(&g)[0], c(0));
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
    let tip = |g: &mut GameState| with_cx(&rules, |cx| board::tip_corp_troops(g, cx)).unwrap();
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

    // 145: a full base makes no troop, so its ambassador stays.
    let mut g = g0.clone();
    let cap = w.int("board.unit_cap");
    put(&mut g, bases[0], UNIT, cap);
    put(&mut g, bases[0], ambassador(1), 1);
    tip(&mut g);
    assert_eq!((g.tokens(bases[0], UNIT), g.tokens(bases[0], ambassador(1))), (cap, 1));
    // ... and with every base of its corp full, the factory's ambassador stays.
    let mut g = g0.clone();
    for &b in &bases {
        put(&mut g, b, UNIT, cap);
    }
    put(&mut g, f, ambassador(0), 1);
    tip(&mut g);
    assert_eq!((units(&g), g.tokens(f, ambassador(0))), (cap * n, 1));
}

/// r75: the adversary factory's troop comes in the income step (`board.produce`);
/// growth (`board.bases`) after it makes no corp troop either.
/// r75 (Matt's ruling 8): at a Discord tip each player ambassador on a neutral
/// factory stops 1 of its troops (one for each of its corp's bases with room)
/// and is used up; seat 0 chooses which base's troop when they can't stop all.
/// A full base takes none and spends no ambassador.
#[test]
fn each_ambassador_on_a_factory_stops_one_tip_troop() {
    let (rules, mut g0) = lab_with(&[], &["countries=3"]);
    let w = rules.world();
    let f = rules.map.numbered(0, piece(0).factories[0]);
    put(&mut g0, f, FACTORY, 1);
    let bases = board::corp_bases(&w, 0);
    assert_eq!(bases.len(), 2);
    let units = |g: &GameState| bases.iter().map(|&b| g.tokens(b, UNIT)).collect::<Vec<_>>();
    let tip = |g: &mut GameState, take: usize| {
        let asked = std::cell::RefCell::new(Vec::new());
        with_answers(&rules, g, |a| {
            asked.borrow_mut().push((a.seat, a.options.len()));
            let at = rules.map.hexes[bases[take]].hex;
            a.options.iter().position(|o| o.target == Some(at)).expect("a base option")
        }, board::tip_corp_troops);
        asked.into_inner()
    };

    // One ambassador: it stops the troop of the base seat 0 picks.
    let mut g = g0.clone();
    put(&mut g, f, ambassador(0), 1);
    assert_eq!(tip(&mut g, 1), [(0, 2)]);
    assert_eq!((units(&g), g.tokens(f, ambassador(0))), (vec![1, 0], 0));

    // Two (any seats'): both troops stopped, both used up. (The rule asks only when they can't stop all; the
    // engine still asks for the first, a choice with one outcome, so the asks aren't checked here.)
    let mut g = g0.clone();
    put(&mut g, f, ambassador(0), 1);
    put(&mut g, f, ambassador(1), 1);
    tip(&mut g, 0);
    assert_eq!((units(&g), g.tokens(f, ambassador(0)), g.tokens(f, ambassador(1))), (vec![0, 0], 0, 0));

    // A full base takes none: one ambassador stops the other base's troop, a
    // second stays.
    let cap = w.int("board.unit_cap");
    let mut g = g0.clone();
    put(&mut g, bases[0], UNIT, cap);
    put(&mut g, f, ambassador(0), 2);
    assert_eq!(tip(&mut g, 1), []);
    assert_eq!((units(&g), g.tokens(f, ambassador(0))), (vec![cap, 0], 1));
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
