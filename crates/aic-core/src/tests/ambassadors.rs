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
    with_cx(&rules, |cx| adversary::place(&mut g, cx, city, 1));
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
    // opposing ambassador blocks a factory's output while it stands.
    let (rules, mut g0) = lab(&["board.bases"]);
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

    // 021: an agent on a node costs its owner 1 insight
    // next round and adds 1 compute to the cheapest project; it stays (068).
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
        (1, 1, 1)
    );

    // 020: only a lone node leaks. On a corrupted city the ambassador is
    // the city's (it stays to spread); on a station it blocks the cube and
    // stays (137: it took the cube and was used up).
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
    // stays; a corrupted station makes adversary compute, no megaproject cube;
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
    assert_eq!(
        (key(&g, 0, None), key(&g, 0, Some(0)), key(&g, 1, None), neutral_key(&g)),
        (0, 0, 0, 1)
    );
    assert_eq!(g.var(adversary::COMPUTE) - compute, rules.world().int("board.station_cubes"));
    assert_eq!(g.tokens(s0, ADV_AMBASSADOR), 1);
    let mut g = g0.clone();
    put(&mut g, s0, STATION, 1);
    put(&mut g, s0, cube(0), 1);
    run(&rules, &mut g);
    // 142: a player-owned station makes `owned_station_cubes`.
    assert_eq!((key(&g, 0, None), key(&g, 0, Some(0))), (0, rules.world().int("board.owned_station_cubes")));
}

#[test]
fn a_player_ambassador_absorbs_a_destruction() {
    let (rules, mut g) = lab(&["board.march"]);
    let m = &rules.map;
    let at = m.numbered(0, piece(0).factories[0]);
    for h in std::iter::once(at).chain(m.hexes[at].neighbors.iter().copied()) {
        assert!(g.hexes[h].tokens.is_empty());
    }
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, cube(0), 1);
    put(&mut g, at, TROOP, 1);
    put(&mut g, at, ambassador(0), 1);
    let discord = g.var("alignment.discord");
    // The ambassador absorbs the strike; the troop that struck is used up.
    run(&rules, &mut g);
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

/// 137: only the piece owner's ambassador is armor: another seat's on an owned
/// facility, or any on a neutral one, absorbs nothing.
#[test]
fn only_the_owners_ambassador_absorbs_a_destruction() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let at = m.numbered(0, piece(0).factories[0]);
    let destroy = |g: &mut GameState, kind: &str| {
        with_cx(&rules, |cx| board::destroy_piece(g, cx, at, kind, board::Cause::Effect("test")))
    };
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

#[test]
fn an_evicting_ambassador_stays() {
    let (rules, mut g) = lab_actions();
    let d = node_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 2);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    act(&rules, &mut g, std_act(0, "remove"));
    assert_eq!((g.tokens(d, ADV_AMBASSADOR), g.tokens(d, ambassador(0))), (0, 1));
    assert_eq!(g.resource(0, Resource::Ambassadors), 0);
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
