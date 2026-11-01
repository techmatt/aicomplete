use super::*;
use crate::modules::{adversary, board};

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
            g.tokens(city, &ambassador(0))
        ),
        (0, 0)
    );
    assert_eq!(g.var(SUPPLY), supply + 1);
    assert_eq!(sway(&g), sway(&g0));

    // An adversary ambassador arrives on a player one.
    let mut g = g0.clone();
    put(&mut g, city, &ambassador(1), 1);
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, city, 1));
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(city, &ambassador(1))
        ),
        (0, 0)
    );
    assert_eq!(g.var(SUPPLY), supply);
    assert_eq!(sway(&g), sway(&g0));
}

#[test]
fn a_player_ambassador_takes_a_neutral_station_and_waits_on_a_corrupted_one() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let neutral = m.numbered(0, piece(0).stations[0]);
    let corrupted = m.numbered(1, piece(1).stations[0]);
    put(&mut g, neutral, STATION, 1);
    put(&mut g, corrupted, STATION, 1);
    put(&mut g, corrupted, ADV_CUBE, 1);
    put(&mut g, neutral, &presence(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, neutral));
    assert_eq!(
        (
            g.tokens(neutral, &cube(0)),
            g.tokens(neutral, &ambassador(0))
        ),
        (1, 0)
    );
    // On a corrupted station it just waits.
    with_cx(&rules, |cx| {
        board::place_ambassador(&mut g, cx, 0, corrupted)
    });
    assert_eq!(
        (
            g.tokens(corrupted, &cube(0)),
            g.tokens(corrupted, &ambassador(0))
        ),
        (0, 1)
    );

}

#[test]
fn ambassadors_divert_bases_and_factories() {
    let (rules, mut g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let [base, other] = m.fronts[0].bases;
    assert_eq!(m.fronts[0].countries, [0, 1]);
    let factory = m.numbered(0, piece(0).factories[0]);
    put(&mut g0, factory, FACTORY, 1);
    let unity = |g: &GameState| g.var("alignment.unity");
    let discord = |g: &GameState| g.var("alignment.discord");

    // Untouched: each base builds a unit, each factory one for its nearest base.
    let mut g = g0.clone();
    run(&rules, &mut g);
    assert_eq!((g.tokens(base, UNIT), g.tokens(other, UNIT)), (2, 1));

    // An adversary ambassador on a base: no unit, 3 Discord.
    let mut g = g0.clone();
    put(&mut g, base, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!(
        (g.tokens(base, UNIT), g.tokens(base, ADV_AMBASSADOR)),
        (1, 0)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (3, 0));

    // A player ambassador on a base: no unit, 2 Unity.
    let mut g = g0.clone();
    put(&mut g, base, &ambassador(0), 1);
    run(&rules, &mut g);
    assert_eq!(
        (g.tokens(base, UNIT), g.tokens(base, &ambassador(0))),
        (1, 0)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 2));

    // An adversary ambassador on a factory: it builds an adversary troop.
    let mut g = g0.clone();
    put(&mut g, factory, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(factory, TROOP),
            g.tokens(factory, ADV_AMBASSADOR),
            g.tokens(base, UNIT)
        ),
        (1, 0, 1)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 0));

    // A player ambassador on a factory: no unit, 1 Unity.
    let mut g = g0.clone();
    put(&mut g, factory, &ambassador(1), 1);
    run(&rules, &mut g);
    assert_eq!(
        (g.tokens(factory, &ambassador(1)), g.tokens(base, UNIT)),
        (0, 1)
    );
    assert_eq!((discord(&g) - discord(&g0), unity(&g) - unity(&g0)), (0, 1));
}

#[test]
fn ambassadors_divert_cities_and_stations_and_leak_presence_compute() {
    let (rules, g0) = lab(&["board.produce"]);
    let m = &rules.map;
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let res = |g: &GameState, s: usize, r: Resource| g.resource(s, r) - g0.resource(s, r);
    let (city, lonely) = city_with_lonely_neighbors(m);
    let open = lonely[0];

    // A corrupted city's energy goes to the adversary: 1 Discord a unit.
    let mut g = g0.clone();
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, open, &presence(0), 1);
    run(&rules, &mut g);
    assert_eq!((discord(&g), res(&g, 0, Resource::Energy)), (1, 0));

    // So does a clean city's with an adversary ambassador on it (which goes).
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 1);
    put(&mut g, open, &presence(0), 1);
    run(&rules, &mut g);
    assert_eq!(
        (
            discord(&g),
            res(&g, 0, Resource::Energy),
            g.tokens(city, ADV_AMBASSADOR)
        ),
        (1, 0, 0)
    );

    // 021: an adversary ambassador on a presence costs its owner 1 insight
    // next round and adds 1 compute to the cheapest project; it is used up.
    let mut g = g0.clone();
    let d = m.numbered(0, piece(0).presence);
    put(&mut g, d, &presence(0), 1);
    put(&mut g, d, ADV_AMBASSADOR, 1);
    let made = g.var("adversary.compute");
    run(&rules, &mut g);
    assert_eq!(
        (
            g.var("economy.insight_debt.0"),
            g.var("adversary.compute") - made,
            g.tokens(d, ADV_AMBASSADOR)
        ),
        (1, 1, 0)
    );

    // 020: only a lone presence leaks. On a corrupted city the ambassador is
    // the city's (it stays to spread); on a neutral station it waits.
    let mut g = g0.clone();
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, city, ADV_AMBASSADOR, 1);
    put(&mut g, city, &presence(0), 1);
    let station = m.numbered(0, piece(0).stations[0]);
    put(&mut g, station, STATION, 1);
    put(&mut g, station, ADV_AMBASSADOR, 1);
    put(&mut g, station, &presence(1), 1);
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

    // An owned station's cube goes to the adversary; a corrupted station's
    // always does; a neutral one makes none.
    let mut g = g0.clone();
    let [s0, s1] = [0, 1].map(|c| m.numbered(c, piece(c).stations[0]));
    let neutral = m.numbered(0, piece(0).factories[0]);
    for s in [s0, s1, neutral] {
        put(&mut g, s, STATION, 1);
    }
    put(&mut g, s0, &cube(0), 1);
    put(&mut g, s0, ADV_AMBASSADOR, 1);
    put(&mut g, s1, ADV_CUBE, 1);
    run(&rules, &mut g);
    // (Round 1 of the new game already put cubes on the projects.)
    let key = |g: &GameState, c: usize, s: Option<usize>| {
        let k = board::project_key(c, s);
        g.var(&k) - g0.var(&k)
    };
    assert_eq!(
        (key(&g, 0, None), key(&g, 0, Some(0)), key(&g, 1, None)),
        (1, 0, 1)
    );
    assert_eq!(g.tokens(s0, ADV_AMBASSADOR), 0);
    let mut g = g0.clone();
    put(&mut g, s0, STATION, 1);
    put(&mut g, s0, &cube(0), 1);
    run(&rules, &mut g);
    assert_eq!((key(&g, 0, None), key(&g, 0, Some(0))), (0, 1));
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
    put(&mut g, at, &cube(0), 1);
    put(&mut g, at, TROOP, 1);
    put(&mut g, at, &ambassador(0), 1);
    let discord = g.var("alignment.discord");
    // The ambassador absorbs the strike; the troop that struck is used up.
    run(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(at, FACTORY),
            g.tokens(at, &ambassador(0)),
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
