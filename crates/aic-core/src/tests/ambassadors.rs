use super::*;
use crate::content::Op;
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
fn one_opposing_ambassador_blocks_a_factory() {
    // 137: one opposing ambassador blocks a factory's output while it stands. r75: a
    // factory's output is in the income step (`board.produce`).
    let (rules, mut g0) = lab(&["board.produce"]);
    let w = rules.world();
    let m = &rules.map;
    let factory = m.numbered(0, piece(0).factories[0]);
    put(&mut g0, factory, FACTORY, 1);
    let chips = w.int("board.factory_chips");
    assert!(chips > 0);
    let sway = |g: &GameState| (g.var("alignment.unity") - g0.var("alignment.unity"), g.var("alignment.discord") - g0.var("alignment.discord"));

    // A neutral factory makes nothing in production.
    let mut g = g0.clone();
    run(&rules, &mut g);
    assert_eq!(g.tokens(factory, TROOP), 0);
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
        (g.resource(0, Resource::Chips) - before, g.tokens(factory, ADV_AMBASSADOR), g.tokens(factory, TROOP)),
        (0, 1, 0)
    );
    assert_eq!(g.stats.counters["board.blocked.factory_player"], 1);

    // 293: the adversary's factory makes 1 agent on its hex, in the adversary's Income beat (no troop, and
    // nothing in production); a player ambassador on it blocks it and stays.
    let mut g = g0.clone();
    put(&mut g, factory, ADV_CUBE, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(factory, TROOP), g.tokens(factory, ADV_AMBASSADOR)), (0, 0));
    with_cx(&rules, |cx| adversary::factories_output(&mut g, cx)).unwrap();
    assert_eq!((g.tokens(factory, TROOP), g.tokens(factory, ADV_AMBASSADOR)), (0, 1));
    put(&mut g, factory, ADV_AMBASSADOR, 0);
    put(&mut g, factory, ambassador(1), 1);
    with_cx(&rules, |cx| adversary::factories_output(&mut g, cx)).unwrap();
    assert_eq!((g.tokens(factory, ADV_AMBASSADOR), g.tokens(factory, ambassador(1))), (0, 1));
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

    // 293: a corrupted city makes 1 Discord, its one output, in the adversary's Income beat (none in production),
    // and pays no player.
    let mut g = g0.clone();
    for h in 0..g.hexes.len() {
        put(&mut g, h, ADV_CUBE, 0);
    }
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, open, node(0), 1);
    run(&rules, &mut g);
    assert_eq!((discord(&g), res(&g, 0, Resource::Energy)), (0, 0));
    with_cx(&rules, |cx| board::corrupted_cities_discord(&mut g, cx));
    assert_eq!(discord(&g), 1);

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
    assert_eq!((g.var("adversary.compute") - made, g.tokens(d, ADV_AMBASSADOR)), (0, 1));

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
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(station, ADV_AMBASSADOR)), (1, 1));

    // 137: an agent on an owned station blocks its cube and
    // stays; a corrupted station puts nothing on a megaproject (293: its cube is the research track's);
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
    let key = |g: &GameState, c: usize, s: usize| {
        let k = board::project_key(c, s);
        g.var(&k) - g0.var(&k)
    };
    let neutral_key = |g: &GameState| {
        let k = board::project_neutral(0);
        g.var(&k) - g0.var(&k)
    };
    // 293: the corrupted station puts no cube on a megaproject (its cube goes onto the research track).
    assert_eq!((key(&g, 0, 0), key(&g, 1, 0), key(&g, 1, 1), neutral_key(&g)), (0, 0, 0, 1));
    // 175: the corrupted station's compute comes in the adversary's compute step, not in production.
    assert_eq!(g.var(adversary::COMPUTE) - compute, 0);
    assert_eq!(g.tokens(s0, ADV_AMBASSADOR), 1);
    let mut g = g0.clone();
    put(&mut g, s0, STATION, 1);
    put(&mut g, s0, cube(0), 1);
    run(&rules, &mut g);
    // 142: a player-owned station makes `owned_station_cubes`.
    assert_eq!(key(&g, 0, 0), rules.world().int("board.owned_station_cubes"));
}

/// 137: the owner's ambassador on the hex is armor. r75 (Matt's ruling 5): the
/// owner is asked whether to spend it, and may let the piece go instead.
#[test]
fn a_player_ambassador_absorbs_a_destruction_when_its_owner_spends_it() {
    let (rules, mut g) = lab(&[]);
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
    // The owner is asked which protection to spend.
    let asked = |a: &crate::state::Ask, take: &str| {
        assert_eq!(a.seat, 0);
        assert_eq!(a.options.iter().map(|o| o.option.as_str()).collect::<Vec<_>>(), ["spend your ambassador there", board::LET_IT_GO]);
        a.options.iter().position(|o| o.option == take).unwrap()
    };
    team_ops(&rules, &mut declined, &[Op::Strike], |a| asked(a, board::LET_IT_GO));
    assert_eq!((declined.tokens(at, FACTORY), declined.tokens(at, ambassador(0))), (0, 1));
    // Spent, the ambassador absorbs the strike; the troop that struck stays (r141).
    team_ops(&rules, &mut g, &[Op::Strike], |a| asked(a, "spend your ambassador there"));
    assert_eq!(g.tokens(beside, TROOP), 1);
    assert_eq!(
        (
            g.tokens(at, FACTORY),
            g.tokens(at, ambassador(0)),
            g.tokens(at, TROOP)
        ),
        (1, 0, 0)
    );
    assert_eq!(g.var("alignment.discord"), discord);
    // The next Strike destroys it (the same troop: it is not used up); a troop strike sways no Discord.
    strike(&rules, &mut g);
    assert_eq!((g.tokens(at, FACTORY), g.tokens(beside, TROOP)), (0, 1));
    assert_eq!(g.var("alignment.discord"), discord);
}

/// r83 (Matt, 2026-11-11, #44): troops ignore ambassadors: a troop on a hex with another seat's ambassadors
/// neither stops for nor removes them, and strikes the facility (only the owner's own ambassador guards it).
#[test]
fn a_troop_ignores_ambassadors_and_strikes_the_facility() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let at = m.numbered(0, piece(0).factories[0]);
    put(&mut g, at, FACTORY, 1);
    put(&mut g, at, cube(0), 1);
    put(&mut g, at, TROOP, 1);
    put(&mut g, at, ambassador(1), 2);
    strike(&rules, &mut g);
    assert_eq!((g.tokens(at, ambassador(1)), g.tokens(at, FACTORY)), (2, 0));
    assert!(!g.stats.counters.contains_key("board.troop_removed_ambassador"));
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
    set_var(&mut g0, board::track2_key(1), 7);
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

    // Seat 1's node: r82, the Aegis shelters it with nothing asked (Shelter on your own pieces once a
    // round), and a second destruction that round finds it spent.
    let mut g = g0.clone();
    let kept = !with_answers(&rules, &mut g, |x| -> usize { panic!("asked {x:?}") }, |g, cx| board::destroy_piece(g, cx, b, node(1), cause));
    assert!(kept);
    assert_eq!(g.tokens(b, node(1)), 1);
    assert_eq!(g.stats.counters["board.aegis_used"], 1);
    with_answers(&rules, &mut g, |x| -> usize { panic!("asked {x:?}") }, |g, cx| board::destroy_piece(g, cx, c, node(1), cause));
    assert_eq!(g.tokens(c, node(1)), 0);
}

/// r75 (Matt's ruling 1): Remove agents spends its ambassadors from the supply, used up, and places none (it
/// stood on the hex it cleared). Its price is `board.remove_ambassadors`, read from the world.
#[test]
fn remove_agents_uses_its_ambassador_up_and_places_none() {
    let (rules, mut g) = lab_actions();
    let price = rules.world().int("board.remove_ambassadors");
    let d = node_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 2);
    set_res(&mut g, 0, Resource::Ambassadors, price);
    act(&rules, &mut g, std_act(0, "remove"));
    assert_eq!((g.tokens(d, ADV_AMBASSADOR), g.tokens(d, ambassador(0))), (0, 0));
    assert_eq!((g.resource(0, Resource::Ambassadors), board::ambassadors_on_board(&g, 0)), (0, 0));
}

/// r75 (Matt's ruling 1): Remove agents takes up to `remove_count` (3) agents: the target hex first, then one at
/// a time from a hex of the player's choice, who may stop short. 236: on or next to the target only (it took
/// them from any hexes in reach): an agent 2 from the target stays, though it is in reach.
#[test]
fn remove_agents_spreads_on_or_next_to_its_target() {
    let (rules, mut g0) = lab(&[]);
    let m = &rules.map;
    assert_eq!(rules.world().int("board.remove_count"), 3);
    let d = m.numbered(0, piece(0).node);
    let x = m.hexes[d].neighbors.iter().copied().find(|&x| kind(m, x) != HexKind::Ocean).unwrap();
    let y = m.hexes[x]
        .neighbors
        .iter()
        .copied()
        .find(|&y| y != d && kind(m, y) != HexKind::Ocean && !m.hexes[d].neighbors.contains(&y))
        .unwrap();
    put(&mut g0, d, node(0), 1);
    put(&mut g0, x, node(0), 1);
    put(&mut g0, y, ADV_AMBASSADOR, 1);
    put(&mut g0, d, ADV_AMBASSADOR, 1);
    put(&mut g0, x, ADV_AMBASSADOR, 3);
    set_res(&mut g0, 0, Resource::Ambassadors, rules.world().int("board.remove_ambassadors"));

    let mut g = g0.clone();
    rules.apply_mut(&mut g, &std_act(0, "remove"), None).unwrap();
    answer_hex(&rules, &mut g, d);
    // d is empty now: only x and "stop removing" are offered (y, in reach, is 2 from the target).
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
    let (rules, mut g) = lab(&["board.produce"]);
    let w = rules.world();
    let z = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial).unwrap();
    put(&mut g, z, FACTORY, 1);
    put(&mut g, z, ambassador(0), 1);
    let st = (0..g.hexes.len()).find(|&h| h != z && !is_city(&w, h) && !is_ocean(&w, h)).unwrap();
    put(&mut g, st, STATION, 1);
    put(&mut g, st, ADV_CUBE, 1);
    put(&mut g, st, ambassador(1), 1);
    let c = w.map.hexes[st].country as usize;
    let seat_cubes = g.var(&board::project_key(c, 1));
    let compute = g.var(adversary::COMPUTE);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(z, ambassador(0)), g.tokens(st, ambassador(1))), (1, 1));
    assert_eq!(g.var(&board::project_key(c, 1)), seat_cubes);
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

/// r142 (Matt, 2026-11-19): each ambassador has one action a round. One from the supply arrives on a node of
/// yours, ready; its move to a hex next to it tips it over, and a tipped one neither moves nor acts.
#[test]
fn an_ambassador_moves_once_a_round_and_arrives_ready() {
    use crate::modules::common::{ready_ambassadors, tipped};
    let (rules, mut g) = lab(&[]);
    let (c, n, far, _) = by_a_city(&mut g, &rules.map);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    let w = rules.world();
    // From the supply it comes onto a node of yours, never straight onto the city next to it.
    assert_eq!(board::move_targets(&w, &g, 0), vec![n]);
    act(&rules, &mut g, std_act(0, "move"));
    assert_eq!((g.resource(0, Resource::Ambassadors), g.tokens(n, ambassador(0)), ready_ambassadors(&g, 0, n)), (0, 1, 1));
    // It arrived ready: it moves on, to a hex next to it, and has then used its action.
    assert!(board::move_targets(&w, &g, 0).contains(&c) && !board::move_targets(&w, &g, 0).contains(&n));
    rules.apply_mut(&mut g, &std_act(0, "move"), None).unwrap();
    answer_hex(&rules, &mut g, c);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(n, ambassador(0)), g.tokens(c, ambassador(0)), g.tokens(c, tipped(0))), (0, 1, 1));
    assert_eq!(ready_ambassadors(&g, 0, c), 0);
    assert!(board::move_targets(&w, &g, 0).is_empty(), "a tipped ambassador does not move");
    assert!(!rules.legal_actions(&g).contains(&std_act(0, "move")));
    // A ready one elsewhere may come to any of your nodes, however far.
    put(&mut g, far, ambassador(0), 1);
    let far2 = rules.map.numbered(1, piece(1).cities[0]);
    put(&mut g, far2, ambassador(0), 1);
    assert!(board::move_targets(&w, &g, 0).contains(&n));
}

/// r142, the screen's off arm (`board.arrive_ready` 0): an ambassador arrives with its action used, from the
/// supply or by a card.
#[test]
fn arriving_spent_tips_the_ambassador() {
    use crate::modules::common::ready_ambassadors;
    let (rules, mut g) = lab_with(&[], &["board.arrive_ready=0"]);
    let (c, n, _, _) = by_a_city(&mut g, &rules.map);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    act(&rules, &mut g, std_act(0, "move"));
    assert_eq!((g.tokens(n, ambassador(0)), ready_ambassadors(&g, 0, n)), (1, 0));
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, c));
    assert_eq!((g.tokens(c, ambassador(0)), ready_ambassadors(&g, 0, c)), (1, 0));
}

/// r142 (Matt, 2026-11-19): liberation is move in, then act: the Liberate is the act of the seat's ready
/// ambassador standing on the site, whatever its reach; its bill is paid by the seat's ambassadors on the site
/// first, then its supply.
#[test]
fn liberate_is_the_act_of_a_ready_ambassador_on_the_site() {
    use crate::modules::common::{ready_ambassadors, tipped};
    let (rules, mut g) = lab_with(&[], &["board.liberate_at=3"]);
    let (c, _n, _, _) = by_a_city(&mut g, &rules.map);
    let w = rules.world();
    put(&mut g, c, ADV_CUBE, 1);
    set_res(&mut g, 0, Resource::Ambassadors, 5);
    assert!(!board::liberable(&w, &g, 0, c), "in reach and paid for, but no ambassador on it");
    // It moved in this round: tipped, it can't act until the next.
    put(&mut g, c, ambassador(0), 1);
    put(&mut g, c, tipped(0), 1);
    assert!(!board::liberable(&w, &g, 0, c) && !rules.legal_actions(&g).contains(&std_act(0, "liberate")));
    put(&mut g, c, tipped(0), 0);
    assert!(board::liberable(&w, &g, 0, c));
    // Another seat's ambassador there is no act of yours, and pays nothing.
    assert!(!board::liberable(&w, &g, 1, c));
    // No reach is needed: the piece is there.
    let mut far = g.clone();
    for h in node_of(&far, 0) {
        put(&mut far, h, node(0), 0);
    }
    assert!(board::liberable(&w, &far, 0, c));
    act(&rules, &mut g, std_act(0, "liberate"));
    assert_eq!((g.tokens(c, ADV_CUBE), g.tokens(c, ambassador(0)), g.resource(0, Resource::Ambassadors)), (0, 0, 3));
    assert_eq!(ready_ambassadors(&g, 0, c), 0);
}

/// r142: the round's end stands every ambassador up again.
#[test]
fn ambassadors_stand_up_at_the_round_end() {
    use crate::modules::common::{ready_ambassadors, tipped};
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(3);
    answer_all(&rules, &mut g);
    let n = node_of(&g, 0)[0];
    put(&mut g, n, ambassador(0), 2);
    put(&mut g, n, tipped(0), 2);
    assert_eq!(ready_ambassadors(&g, 0, n), 0);
    let round = g.round;
    while g.round == round && !g.is_over() {
        if g.pending.is_some() {
            answer_all(&rules, &mut g);
            continue;
        }
        let legal = rules.legal_actions(&g);
        let a = legal.iter().find(|a| matches!(a, Action::Pass { .. })).unwrap_or(&legal[0]).clone();
        rules.apply_mut(&mut g, &a, None).unwrap();
    }
    assert_eq!(g.tokens(n, tipped(0)), 0);
}

/// r142: a troop on or next to one of your ambassadors is in your reach for "Target: Troops": the guard beside a
/// city your ambassador moved onto can be removed though it stands two hexes from your node (294's gap).
#[test]
fn a_troop_next_to_your_ambassador_is_in_reach() {
    use crate::modules::effects;
    use crate::op::TargetSpec;
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (c, n, _, _) = by_a_city(&mut g, m);
    let w = rules.world();
    // A hex next to the city, two from the node.
    let guard = m.hexes[c].neighbors.iter().copied().find(|&x| kind(m, x) != HexKind::Ocean && m.distance(x, n) == 2);
    let Some(guard) = guard else { panic!("no hex next to the city two from the node") };
    put(&mut g, guard, TROOP, 1);
    assert!(effects::target_hexes(&w, &g, 0, TargetSpec::Troops).is_empty(), "out of the node's reach");
    put(&mut g, c, ambassador(0), 1);
    assert_eq!(effects::target_hexes(&w, &g, 0, TargetSpec::Troops), vec![guard]);
    assert!(effects::target_hexes(&w, &g, 1, TargetSpec::Troops).is_empty(), "only your own ambassador");
    assert_eq!(effects::target_hexes(&w, &g, 0, TargetSpec::Adversary), vec![guard], "Agents or Troops, where a troop stands");
    // The op that follows its target acts there (Interdict I: Target: Troops, remove up to 2 on it).
    let mut x = g.clone();
    with_answers(&rules, &mut x, |_| 0, |g, cx| {
        let ops = [Op::Target { on: TargetSpec::Troops }, Op::Damage { ambassadors: 0, troops: 2 }];
        effects::run(g, cx, &ops, &mut effects::Ctx::seat(0))
    });
    assert_eq!(x.tokens(guard, TROOP), 0);
    put(&mut g, guard, TROOP, 0);
    put(&mut g, guard, ADV_AMBASSADOR, 1);
    assert!(effects::target_hexes(&w, &g, 0, TargetSpec::Agents).is_empty(), "troops only");
    assert!(effects::target_hexes(&w, &g, 0, TargetSpec::Adversary).is_empty(), "an agent alone is out of reach");
}

/// r142: an ambassador a card moves keeps its state (the move is not its own action): a tipped one arrives
/// tipped, a ready one ready, and a ready one leaves first. An ambassador that leaves takes its marker with it.
#[test]
fn a_card_moves_an_ambassador_as_it_stands() {
    use crate::modules::common::{ready_ambassadors, tipped};
    use crate::op::Piece;
    let (rules, mut g) = lab(&[]);
    let (c, n, _, _) = by_a_city(&mut g, &rules.map);
    let gather = [Op::Gather { max: 1, pieces: vec![Piece::Ambassadors], strike: false }];
    let onto_city = |a: &crate::state::Ask| {
        let at = rules.map.hexes[c].hex;
        a.options.iter().position(|o| o.target == Some(at)).unwrap_or(0)
    };
    // One ready and one tipped on the node: the gather takes the ready one, which arrives ready.
    put(&mut g, n, ambassador(0), 2);
    put(&mut g, n, tipped(0), 1);
    let mut x = g.clone();
    with_answers(&rules, &mut x, onto_city, |g, cx| crate::modules::effects::run(g, cx, &gather, &mut crate::modules::effects::Ctx::seat(0)));
    assert_eq!((x.tokens(c, ambassador(0)), ready_ambassadors(&x, 0, c)), (1, 1));
    assert_eq!((x.tokens(n, ambassador(0)), x.tokens(n, tipped(0))), (1, 1));
    // Only a tipped one there: it arrives tipped, and its marker goes with it.
    put(&mut g, n, ambassador(0), 1);
    with_answers(&rules, &mut g, onto_city, |g, cx| crate::modules::effects::run(g, cx, &gather, &mut crate::modules::effects::Ctx::seat(0)));
    assert_eq!((g.tokens(c, ambassador(0)), g.tokens(c, tipped(0)), ready_ambassadors(&g, 0, c)), (1, 1, 0));
    assert_eq!((g.tokens(n, ambassador(0)), g.tokens(n, tipped(0))), (0, 0));
    // An agent cancels it: no marker stays behind.
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1)).unwrap();
    assert_eq!((g.tokens(c, ambassador(0)), g.tokens(c, tipped(0))), (0, 0));
}
