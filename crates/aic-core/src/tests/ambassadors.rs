use super::*;
use crate::modules::common::{is_city, is_ocean};
use crate::modules::{adversary, board, cards, economy};

#[test]
fn opposing_ambassadors_on_one_piece_cancel_without_unity() {
    let (rules, g0) = lab(&[]);
    let city = rules.map.numbered(0, piece(0).cities[0]);
    let sway = |g: &GameState| (g.var("alignment.unity"), g.var("alignment.discord"));

    // A player ambassador arrives on an adversary one.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 1);
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, city));
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(city, ambassador(0))
        ),
        (0, 0)
    );
    assert_eq!(g.var(SUPPLY), supply + 1);
    assert_eq!(sway(&g), sway(&g0));

    // An agent arrives on a player one.
    let mut g = g0.clone();
    put(&mut g, city, ambassador(1), 1);
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, city, 1)).unwrap();
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(city, ambassador(1))
        ),
        (0, 0)
    );
    assert_eq!(g.var(SUPPLY), supply);
    assert_eq!(sway(&g), sway(&g0));
}

#[test]
fn an_arriving_player_ambassador_claims_nothing_and_stands() {
    // 137: claiming is the Claim action; an arriving ambassador just stands,
    // on a neutral station the seat could claim too.
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let neutral = m.numbered(0, piece(0).stations[0]);
    let corrupted = m.numbered(1, piece(1).stations[0]);
    put(&mut g, neutral, STATION, 1);
    put(&mut g, corrupted, STATION, 1);
    put(&mut g, corrupted, ADV_CUBE, 1);
    put(&mut g, neutral, node(0), 1);
    assert!(board::can_claim(&rules.world(), &g, 0, neutral));
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, neutral));
    assert_eq!(
        (
            g.tokens(neutral, cube(0)),
            g.tokens(neutral, ambassador(0))
        ),
        (0, 1)
    );
    // On a corrupted station it stands too.
    with_cx(&rules, |cx| {
        board::place_ambassador(&mut g, cx, 0, corrupted)
    });
    assert_eq!(
        (
            g.tokens(corrupted, cube(0)),
            g.tokens(corrupted, ambassador(0))
        ),
        (0, 1)
    );

}

#[test]
fn one_opposing_ambassador_blocks_a_factory_and_bases_make_no_troops() {
    // 137: production makes no corp troop (they come at Discord tips); one
    // opposing ambassador blocks a factory's output while it stands. r75: a
    // factory's output is in the income step (`board.produce`), growth
    // (`board.bases`) after it.
    let (rules, mut g0) = lab(&["board.produce", "board.bases"]);
    let w = rules.world();
    let m = &rules.map;
    let [base, other] = m.fronts[0].bases;
    assert_eq!(m.fronts[0].countries, [0, 1]);
    let factory = m.numbered(0, piece(0).factories[0]);
    put(&mut g0, factory, FACTORY, 1);
    let chips = w.int("board.factory_chips");
    assert!(chips > 0);
    let sway = |g: &GameState| (g.var("alignment.unity") - g0.var("alignment.unity"), g.var("alignment.discord") - g0.var("alignment.discord"));
    let units = |g: &GameState| (g.tokens(base, UNIT), g.tokens(other, UNIT));

    // A neutral factory and the bases make nothing in production.
    let mut g = g0.clone();
    run(&rules, &mut g);
    assert_eq!((units(&g), g.tokens(factory, TROOP)), ((0, 0), 0));

    // Ambassadors on a base stand there and make nothing happen.
    let mut g = g0.clone();
    put(&mut g, base, ADV_AMBASSADOR, 1);
    put(&mut g, other, ambassador(0), 1);
    run(&rules, &mut g);
    assert_eq!((units(&g), g.tokens(base, ADV_AMBASSADOR), g.tokens(other, ambassador(0))), ((0, 0), 1, 1));
    assert_eq!(sway(&g), (0, 0));

    // A player's factory makes its owner chips; an agent on it
    // blocks them and stays (it no longer becomes a troop).
    let mut g = g0.clone();
    put(&mut g, factory, cube(0), 1);
    let before = g.resource(0, Resource::Chips);
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Chips) - before, chips);
    put(&mut g, factory, ADV_AMBASSADOR, 1);
    let before = g.resource(0, Resource::Chips);
    run(&rules, &mut g);
    assert_eq!(
        (g.resource(0, Resource::Chips) - before, g.tokens(factory, ADV_AMBASSADOR), g.tokens(factory, TROOP), units(&g)),
        (0, 1, 0, (0, 0))
    );
    assert_eq!(g.stats.counters["board.blocked.factory_player"], 1);

    // The adversary's factory makes 1 adversary troop on its hex, no corp troop;
    // a player ambassador on it blocks it and stays.
    let mut g = g0.clone();
    put(&mut g, factory, ADV_CUBE, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(factory, TROOP), units(&g)), (1, (0, 0)));
    put(&mut g, factory, TROOP, 0);
    put(&mut g, factory, ambassador(1), 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(factory, TROOP), g.tokens(factory, ambassador(1))), (0, 1));
    assert_eq!(g.stats.counters["board.blocked.factory_adversary"], 1);
    assert_eq!(sway(&g), (0, 0));
}

#[test]
fn ambassadors_divert_cities_and_stations_and_leak_node_compute() {
    let (rules, g0) = lab(&["board.produce"]);
    let m = &rules.map;
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let res = |g: &GameState, s: usize, r: Resource| g.resource(s, r) - g0.resource(s, r);
    let (city, lonely) = city_with_lonely_neighbors(m);
    let open = lonely[0];

    // A corrupted city's energy goes to the adversary: 1 Discord a unit.
    let mut g = g0.clone();
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, open, node(0), 1);
    run(&rules, &mut g);
    assert_eq!((discord(&g), res(&g, 0, Resource::Energy)), (1, 0));

    // r38: on an aligned city the ambassador stays too and the city is dark: no
    // energy to its owner, and `ambassador_discord` (1 here) a unit, every round.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 1);
    put(&mut g, city, cube(0), 1);
    put(&mut g, open, node(0), 1);
    for round in 1..=2 {
        run(&rules, &mut g);
        assert_eq!(
            (discord(&g), res(&g, 0, Resource::Energy), g.tokens(city, ADV_AMBASSADOR)),
            (round, 0, 1)
        );
    }
    assert_eq!(g.stats.counters["board.city_dark_aligned"], 2);
    assert!(!g.stats.counters.contains_key("board.city_dark"));

    // r37: on a neutral city the ambassador stays and the city is dark: no sale, no
    // Discord, every round it stands there.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 1);
    put(&mut g, open, node(0), 1);
    with_cx(&rules, |cx| cx.add_resource(&mut g, 0, Resource::Credits, 50));
    let credits = g.resource(0, Resource::Credits);
    for _ in 0..2 {
        run(&rules, &mut g);
        assert_eq!(
            (discord(&g), res(&g, 0, Resource::Energy), g.resource(0, Resource::Credits), g.tokens(city, ADV_AMBASSADOR)),
            (0, 0, credits, 1)
        );
    }
    assert_eq!(g.stats.counters["board.city_dark"], 2);

    // 175: an agent on a node no longer drains insight or makes compute (a node hex corrupts instead).
    let mut g = g0.clone();
    let d = m.numbered(0, piece(0).node);
    put(&mut g, d, node(0), 1);
    put(&mut g, d, ADV_AMBASSADOR, 1);
    let made = g.var("adversary.compute");
    run(&rules, &mut g);
    assert_eq!(
        (
            g.var("economy.insight_debt.0"),
            g.var("adversary.compute") - made,
            g.tokens(d, ADV_AMBASSADOR)
        ),
        (0, 0, 1)
    );

    // On a corrupted city the ambassador is the city's (it stays to spread); on a station it blocks the cube
    // and stays (137: it took the cube and was used up).
    let mut g = g0.clone();
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, city, ADV_AMBASSADOR, 1);
    put(&mut g, city, node(0), 1);
    let station = m.numbered(0, piece(0).stations[0]);
    put(&mut g, station, STATION, 1);
    put(&mut g, station, ADV_AMBASSADOR, 1);
    put(&mut g, station, node(1), 1);
    run(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(station, ADV_AMBASSADOR),
            g.var("economy.insight_debt.0"),
            g.var("economy.insight_debt.1")
        ),
        (1, 1, 0, 0)
    );

    // 137: an agent on an owned station blocks its cube and
    // stays; a corrupted station puts an adversary cube on its open megaproject (185);
    // a neutral one makes a neutral cube.
    let mut g = g0.clone();
    let [s0, s1] = [0, 1].map(|c| m.numbered(c, piece(c).stations[0]));
    let neutral = m.numbered(0, piece(0).factories[0]);
    for s in [s0, s1, neutral] {
        put(&mut g, s, STATION, 1);
    }
    put(&mut g, s0, cube(0), 1);
    put(&mut g, s0, ADV_AMBASSADOR, 1);
    put(&mut g, s1, ADV_CUBE, 1);
    let compute = g.var(adversary::COMPUTE);
    run(&rules, &mut g);
    // (Round 1 of the new game already put cubes on the projects.)
    let key = |g: &GameState, c: usize, s: Option<usize>| {
        let k = board::project_key(c, s);
        g.var(&k) - g0.var(&k)
    };
    let neutral_key = |g: &GameState| {
        let k = board::project_neutral(0);
        g.var(&k) - g0.var(&k)
    };
    // 185: the corrupted station puts an adversary cube on its open megaproject.
    let cubes = rules.world().int("board.station_cubes");
    assert_eq!(
        (key(&g, 0, None), key(&g, 0, Some(0)), key(&g, 1, None), neutral_key(&g)),
        (0, 0, cubes, 1)
    );
    // 175: the corrupted station's compute comes in the adversary's compute step, not in production.
    assert_eq!(g.var(adversary::COMPUTE) - compute, 0);
    assert_eq!(g.tokens(s0, ADV_AMBASSADOR), 1);
    let mut g = g0.clone();
    put(&mut g, s0, STATION, 1);
    put(&mut g, s0, cube(0), 1);
    run(&rules, &mut g);
    // 142: a player-owned station makes `owned_station_cubes`.
    assert_eq!((key(&g, 0, None), key(&g, 0, Some(0))), (0, rules.world().int("board.owned_station_cubes")));
}

/// 137: the owner's ambassador on the hex is armor. r75 (Matt's ruling 5): the
/// owner is asked whether to spend it, and may let the piece go instead.
#[test]
fn a_player_ambassador_absorbs_a_destruction_when_its_owner_spends_it() {
    let (rules, mut g) = lab(&["board.march"]);
    let m = &rules.map;
    let at = m.numbered(0, piece(0).factories[0]);
    for h in std::iter::once(at).chain(m.hexes[at].neighbors.iter().copied()) {
        assert!(g.hexes[h].tokens.is_empty());
    }
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, cube(0), 1);
    // 175: a troop next door (not on the ambassador's hex) strikes; the ambassador absorbs it.
    let beside = m.hexes[at].neighbors.iter().copied().find(|&x| kind(m, x) != HexKind::Ocean).unwrap();
    put(&mut g, beside, TROOP, 1);
    put(&mut g, at, ambassador(0), 1);
    let discord = g.var("alignment.discord");
    // Declined, the strike destroys the factory and the ambassador stays.
    let mut declined = g.clone();
    run(&rules, &mut declined);
    let p = declined.pending.as_ref().expect("the owner is asked which protection to spend");
    assert_eq!(p.ask.seat, 0);
    assert_eq!(p.ask.options.iter().map(|o| o.option.as_str()).collect::<Vec<_>>(), ["spend your ambassador there", board::LET_IT_GO]);
    answer_with(&rules, &mut declined, board::LET_IT_GO);
    answer_all(&rules, &mut declined);
    assert_eq!((declined.tokens(at, FACTORY), declined.tokens(at, ambassador(0))), (0, 1));
    // Spent, the ambassador absorbs the strike; the troop that struck is used up.
    run(&rules, &mut g);
    answer_with(&rules, &mut g, "spend your ambassador there");
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(beside, TROOP), 0);
    assert_eq!(
        (
            g.tokens(at, FACTORY),
            g.tokens(at, ambassador(0)),
            g.tokens(at, TROOP)
        ),
        (1, 0, 0)
    );
    assert_eq!(g.var("alignment.discord"), discord);
    // A second troop destroys it; a troop strike sways no Discord.
    put(&mut g, at, TROOP, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(at, FACTORY), g.tokens(at, TROOP)), (0, 0));
    assert_eq!(g.var("alignment.discord"), discord);
}

/// 175: ambassadors are non-combatants: a troop starting a step on an ambassador's hex removes one instead
/// of moving or striking, so with 2 steps it removes two.
#[test]
fn a_troop_on_an_ambassadors_hex_removes_one_a_step() {
    let (rules, mut g) = lab(&["board.march"]);
    let m = &rules.map;
    let at = m.numbered(0, piece(0).factories[0]);
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, cube(0), 1);
    put(&mut g, at, TROOP, 1);
    put(&mut g, at, ambassador(0), 2);
    run_answered(&rules, &mut g);
    assert_eq!((g.tokens(at, ambassador(0)), g.tokens(at, FACTORY), g.tokens(at, TROOP)), (0, 1, 1));
    assert_eq!(g.stats.counters["board.troop_removed_ambassador"], 2);
}

/// 137: only the piece owner's ambassador is armor: another seat's on an owned
/// facility, or any on a neutral one, absorbs nothing.
#[test]
fn only_the_owners_ambassador_absorbs_a_destruction() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let at = m.numbered(0, piece(0).factories[0]);
    // r75: the owner spends the first protection offered.
    let destroy = |g: &mut GameState, kind: &str| destroy_with(&rules, g, at, kind, board::Cause::Effect("test"), true);
    // Seat 0's factory: seat 1's ambassador is no armor, seat 0's is.
    let mut g = g0.clone();
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, cube(0), 1);
    put(&mut g, at, ambassador(1), 1);
    assert!(destroy(&mut g, FACTORY));
    assert_eq!((g.tokens(at, FACTORY), g.tokens(at, ambassador(1))), (0, 1));
    let mut g = g0.clone();
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, cube(0), 1);
    put(&mut g, at, ambassador(0), 1);
    assert!(!destroy(&mut g, FACTORY));
    assert_eq!((g.tokens(at, FACTORY), g.tokens(at, ambassador(0))), (1, 0));
    // A neutral factory has no armor.
    let mut g = g0.clone();
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, ambassador(0), 1);
    assert!(destroy(&mut g, FACTORY));
    assert_eq!(g.tokens(at, ambassador(0)), 1);
    // A node: its owner's ambassador, not another seat's.
    let mut g = g0.clone();
    put(&mut g, at, node(1), 1);
    put(&mut g, at, ambassador(0), 1);
    assert!(destroy(&mut g, node(1)));
    let mut g = g0.clone();
    put(&mut g, at, node(1), 1);
    put(&mut g, at, ambassador(1), 1);
    assert!(!destroy(&mut g, node(1)));
    assert_eq!(g.tokens(at, ambassador(1)), 0);
}

/// r75 (Matt's ruling 1): an arriving agent cancels one player ambassador on its
/// hex, 1 for 1; with several seats' ambassadors there seat 0 chooses whose (it
/// was the lowest seat's).
#[test]
fn an_arriving_agent_cancels_one_ambassador_and_the_players_choose_whose() {
    let (rules, g0) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let supply = g0.var(adversary::SUPPLY);
    assert!(supply > 0);

    // One seat's ambassadors: one cancels, nothing is asked.
    let mut g = g0.clone();
    put(&mut g, h, ambassador(0), 2);
    with_answers(&rules, &mut g, |a| -> usize { panic!("asked {a:?}") }, |g, cx| adversary::place(g, cx, h, 1));
    assert_eq!((g.tokens(h, ambassador(0)), g.tokens(h, ADV_AMBASSADOR), g.var(adversary::SUPPLY)), (1, 0, supply));

    // Two seats': seat 0 is asked whose; it picks seat 1's.
    let mut g = g0.clone();
    put(&mut g, h, ambassador(0), 1);
    put(&mut g, h, ambassador(1), 1);
    let asked = std::cell::RefCell::new(Vec::new());
    with_answers(&rules, &mut g, |a| {
        asked.borrow_mut().push((a.seat, a.prompt.clone(), a.options.len()));
        1
    }, |g, cx| adversary::place(g, cx, h, 1));
    let asked = asked.into_inner();
    assert_eq!(asked.len(), 1);
    assert_eq!((asked[0].0, asked[0].2), (0, 2));
    assert!(asked[0].1.contains("whose ambassador does it cancel?"), "{}", asked[0].1);
    assert_eq!((g.tokens(h, ambassador(0)), g.tokens(h, ambassador(1)), g.tokens(h, ADV_AMBASSADOR)), (1, 0, 0));
}

/// r75 (Matt's ruling 5): the Aegis Doctrine (`CivicShelter`, the Protector's
/// play-track capstone) guards only its holder's own pieces, when the holder
/// chooses, once a round (it was spent on anyone's piece in a country it was in).
#[test]
fn the_aegis_doctrine_guards_only_its_holders_own_pieces() {
    let (rules, mut g0) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    g0.vars.insert(board::track2_key(1).into(), 7);
    assert!(cards::has_passive(&w, &g0, 1, |p| matches!(p, crate::content::Op::CivicShelter)), "seat 1 plays the Protector");
    let [a, b, c] = [piece(0).node, piece(0).cities[0], piece(0).cities[1]].map(|n| m.numbered(0, n));
    put(&mut g0, a, node(0), 1);
    put(&mut g0, b, node(1), 1);
    put(&mut g0, c, node(1), 1);
    let cause = board::Cause::Effect("test");

    // Seat 0's node: no protection offered, it goes.
    let mut g = g0.clone();
    with_answers(&rules, &mut g, |x| -> usize { panic!("asked {x:?}") }, |g, cx| board::destroy_piece(g, cx, a, node(0), cause));
    assert_eq!(g.tokens(a, node(0)), 0);

    // Seat 1's node: seat 1 is offered the Aegis; used, it keeps the node, and a
    // second destruction that round finds it spent.
    let mut g = g0.clone();
    let asked = std::cell::RefCell::new(Vec::new());
    let kept = !with_answers(&rules, &mut g, |x| {
        asked.borrow_mut().push((x.seat, x.options.iter().map(|o| o.option.clone()).collect::<Vec<_>>()));
        0
    }, |g, cx| board::destroy_piece(g, cx, b, node(1), cause));
    assert!(kept);
    assert_eq!(asked.into_inner(), [(1, vec!["use the Aegis Doctrine".to_string(), board::LET_IT_GO.to_string()])]);
    assert_eq!(g.tokens(b, node(1)), 1);
    with_answers(&rules, &mut g, |x| -> usize { panic!("asked {x:?}") }, |g, cx| board::destroy_piece(g, cx, c, node(1), cause));
    assert_eq!(g.tokens(c, node(1)), 0);
}

/// r75 (Matt's ruling 1): Remove agents spends its ambassador from the supply, used up, and places none (it
/// stood on the hex it cleared).
#[test]
fn remove_agents_uses_its_ambassador_up_and_places_none() {
    let (rules, mut g) = lab_actions();
    let d = node_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 2);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    act(&rules, &mut g, std_act(0, "remove"));
    assert_eq!((g.tokens(d, ADV_AMBASSADOR), g.tokens(d, ambassador(0))), (0, 0));
    assert_eq!((g.resource(0, Resource::Ambassadors), board::ambassadors_on_board(&g, 0)), (0, 0));
}

/// r75 (Matt's ruling 1): Remove agents takes up to `remove_count` (3) agents from any hexes in reach: the
/// target hex first, then one at a time from a hex of the player's choice, who may stop short.
#[test]
fn remove_agents_spreads_over_hexes_in_reach() {
    let (rules, mut g0) = lab(&[]);
    let m = &rules.map;
    assert_eq!(rules.world().int("board.remove_count"), 3);
    let d = m.numbered(0, piece(0).node);
    let x = m.hexes[d].neighbors.iter().copied().find(|&x| kind(m, x) != HexKind::Ocean).unwrap();
    put(&mut g0, d, node(0), 1);
    put(&mut g0, d, ADV_AMBASSADOR, 1);
    put(&mut g0, x, ADV_AMBASSADOR, 3);
    set_res(&mut g0, 0, Resource::Ambassadors, 1);

    let mut g = g0.clone();
    rules.apply_mut(&mut g, &std_act(0, "remove"), None).unwrap();
    answer_hex(&rules, &mut g, d);
    // d is empty now: only x and "stop removing" are offered.
    let p = g.pending.as_ref().expect("asked for another agent");
    assert_eq!(p.ask.options.len(), 2);
    assert_eq!(p.ask.options[1].option, board::STOP_REMOVING);
    answer_hex(&rules, &mut g, x);
    answer_hex(&rules, &mut g, x);
    assert!(g.pending.is_none(), "3 removed: no more is asked");
    assert_eq!((g.tokens(d, ADV_AMBASSADOR), g.tokens(x, ADV_AMBASSADOR)), (0, 1));
    assert_eq!((g.resource(0, Resource::Ambassadors), board::ambassadors_on_board(&g, 0)), (0, 0));

    // The player may stop after the first.
    let mut g = g0.clone();
    rules.apply_mut(&mut g, &std_act(0, "remove"), None).unwrap();
    answer_hex(&rules, &mut g, x);
    answer_with(&rules, &mut g, board::STOP_REMOVING);
    assert!(g.pending.is_none());
    assert_eq!((g.tokens(d, ADV_AMBASSADOR), g.tokens(x, ADV_AMBASSADOR), board::ambassadors_on_board(&g, 0)), (1, 2, 0));
}

#[test]
fn an_ambassador_on_a_factory_stays_and_one_on_a_corrupted_station_blocks_it() {
    // 137: a player ambassador on an adversary station blocks its output (it
    // took the cube for its seat) and stays.
    let (rules, mut g) = lab(&["board.bases", "board.produce"]);
    let w = rules.world();
    let z = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial).unwrap();
    put(&mut g, z, FACTORY, 1);
    put(&mut g, z, ambassador(0), 1);
    let st = (0..g.hexes.len()).find(|&h| h != z && !is_city(&w, h) && !is_ocean(&w, h)).unwrap();
    put(&mut g, st, STATION, 1);
    put(&mut g, st, ADV_CUBE, 1);
    put(&mut g, st, ambassador(1), 1);
    let c = w.map.hexes[st].country as usize;
    let adversary_cubes = g.var(&board::project_key(c, None));
    let seat_cubes = g.var(&board::project_key(c, Some(1)));
    let compute = g.var(adversary::COMPUTE);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(z, ambassador(0)), g.tokens(st, ambassador(1))), (1, 1));
    assert_eq!(g.var(&board::project_key(c, Some(1))), seat_cubes);
    assert_eq!(g.var(&board::project_key(c, None)), adversary_cubes);
    assert_eq!(g.var(adversary::COMPUTE), compute);
    // 175: the block is read in the adversary's compute step.
    with_cx(&rules, |cx| adversary::compute_step(&mut g, cx)).unwrap();
    assert_eq!(g.stats.counters["board.blocked.station_adversary"], 1);
}

#[test]
fn income_can_print_ambassadors() {
    // No card prints ambassador income since r42: the card is built here.
    let (rules, mut g) = lab_actions_with(&[
        r#"cards.orbital_embassy.passives=[{"op":"income","resource":"ambassadors","amount":2}]"#,
    ]);
    let w = rules.world();
    set_pile(&mut g, &cards::built(0), &["orbital_embassy"]);
    assert_eq!(economy::ambassador_income(&w, &g, 0), 2);
}
