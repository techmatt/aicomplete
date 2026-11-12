//! Factories, troops and chips.

use super::*;
use crate::modules::board;
use crate::modules::common::CITY_CHIP;

/// 190 (Matt, 2026-11-11, amendment 8642): a city gathers 1 chip a round from each unclaimed neutral factory next
/// to it, none of its own; a claimed or corrupted factory adds nothing; an agent on the city blocks nothing; it holds
/// at most `board.city_chip_cap`.
#[test]
fn a_city_gathers_a_chip_from_each_unclaimed_neutral_factory_next_to_it() {
    let (rules, g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // Country 0: cities 6, 7, 14; zones 2 (by 6), 8 and 11 (by 7), 11 and 13 (by 14).
    assert_eq!(
        (piece(0).cities.clone(), piece(0).industry.clone()),
        (vec![7, 6, 14], vec![2, 8, 11, 13])
    );
    let chips = |g: &GameState| [6, 7, 14].map(|n| g.tokens(at(n), CITY_CHIP));

    // Nothing next to a city: no chip, and nothing is built (r75's factory a round is gone).
    let mut g = g0.clone();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(chips(&g), [0, 0, 0]);
    assert!((0..g.hexes.len()).all(|h| g.tokens(h, FACTORY) == 0 && g.tokens(h, STATION) == 0));

    // Neutral factories on 8 and 11; 2 claimed by seat 0; an agent on 7.
    let mut g = g0.clone();
    put(&mut g, at(8), FACTORY, 1);
    put(&mut g, at(11), FACTORY, 1);
    put(&mut g, at(2), FACTORY, 1);
    put(&mut g, at(2), cube(0), 1);
    put(&mut g, at(7), ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(chips(&g), [0, 2, 1]);

    // A corrupted factory adds nothing either.
    let mut g = g0.clone();
    put(&mut g, at(2), FACTORY, 1);
    put(&mut g, at(2), ADV_CUBE, 1);
    run(&rules, &mut g);
    assert_eq!(chips(&g)[0], 0);

    // The cap: 2 waiting and two factories next to it reach 3, not 4, and it builds.
    let mut g = g0.clone();
    put(&mut g, at(11), FACTORY, 1);
    put(&mut g, at(13), FACTORY, 1);
    put(&mut g, at(14), CITY_CHIP, 2);
    put(&mut g, at(10), STATION, 1);
    put(&mut g, at(15), STATION, 1);
    let before = g.stats.counters.get("board.city_chips").copied().unwrap_or(0);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.stats.counters.get("board.city_chips").copied().unwrap_or(0) - before, 1 + 1, "14 gets 1, 7 gets 1");
    assert_eq!(rules.world().int("board.city_chip_cap"), 3);
}

/// 190: at 3 chips a city spends them on a neutral facility, whichever of factories and research stations it has
/// fewer of next to it (ties: a factory), any owner's counted.
#[test]
fn at_three_chips_a_city_builds_a_neutral_facility_of_the_kind_it_has_fewer_of() {
    let (rules, g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    assert_eq!(rules.world().int("board.city_build_chips"), 3);
    let near = |g: &GameState, c: u8, kind: &str| m.hexes[at(c)].neighbors.iter().filter(|&&h| g.tokens(h, kind) > 0).count();

    // 6 has a factory next to it (a player's: claimed, so it feeds nothing) and no station: a station, neutral.
    let mut g = g0.clone();
    put(&mut g, at(2), FACTORY, 1);
    put(&mut g, at(2), cube(1), 1);
    put(&mut g, at(6), CITY_CHIP, 3);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(at(6), CITY_CHIP), 0);
    assert_eq!((near(&g, 6, STATION), near(&g, 6, FACTORY)), (1, 1));
    let s = m.hexes[at(6)].neighbors.iter().copied().find(|&h| g.tokens(h, STATION) > 0).unwrap();
    assert_eq!(cube_on(&g, s), None, "neutral");

    // 14 has nothing next to it: a tie, so a factory, neutral, on its lowest free zone (11).
    let mut g = g0.clone();
    put(&mut g, at(14), CITY_CHIP, 3);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(at(11), FACTORY), cube_on(&g, at(11))), (1, None));
    assert_eq!(near(&g, 14, STATION), 0);
}

/// 190: an aligned city's owner picks the hex; a corrupted city builds neutral too, next to an agent if it can,
/// else on its lowest-numbered site; a neutral city on its lowest-numbered site.
#[test]
fn the_owner_picks_the_hex_and_a_corrupted_citys_facility_is_neutral() {
    let (rules, g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);

    // Aligned 7 (seat 1), a tie: a factory, its owner choosing between 8 and 11.
    let mut g = g0.clone();
    put(&mut g, at(7), cube(1), 1);
    put(&mut g, at(7), CITY_CHIP, 3);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("the owner's choice");
    assert_eq!(p.ask.seat, 1);
    assert!(p.ask.prompt.starts_with(&format!("{} #7 builds a neutral factory", piece(0).name)), "{}", p.ask.prompt);
    answer_hex(&rules, &mut g, at(11));
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(at(11), FACTORY), cube_on(&g, at(11))), (1, None));
    assert_eq!(g.tokens(at(8), FACTORY), 0);

    // Corrupted 7: nobody is asked; 8 with no agent near, 11 with an agent on 14 beside it; neutral either way.
    for (agent, want) in [(false, 8), (true, 11)] {
        let mut g = g0.clone();
        put(&mut g, at(7), ADV_CUBE, 1);
        put(&mut g, at(7), CITY_CHIP, 3);
        if agent {
            put(&mut g, at(14), ADV_AMBASSADOR, 1);
        }
        run(&rules, &mut g);
        assert!(g.pending.is_none());
        assert_eq!((g.tokens(at(want), FACTORY), cube_on(&g, at(want))), (1, None), "agent {agent}");
    }
}

/// 190: with no free hex of its kind next to it, a city builds within 2 in its own country; with none there, nothing
/// is built and its chips wait (no switch to the other kind, amendment 8642).
#[test]
fn with_no_site_next_to_it_a_city_builds_within_two_in_its_country_else_its_chips_wait() {
    let (rules, g0) = lab(&["board.bases"]);
    let m = &rules.map;
    let at = |n: u8| m.numbered(0, n);
    // 6: a station on 2 (its only zone), so it wants a factory and has no zone next to it.
    let mut g1 = g0.clone();
    put(&mut g1, at(2), STATION, 1);
    put(&mut g1, at(6), CITY_CHIP, 3);
    let mut g = g1.clone();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    let built: Vec<u8> = [8, 11, 13].into_iter().filter(|&n| g.tokens(at(n), FACTORY) > 0).collect();
    assert_eq!(built, vec![8], "the lowest zone within 2");
    assert_eq!(m.distance(at(6), at(8)), 2);

    // Every zone within 2 taken: no factory, no station instead, and the chips wait.
    let mut g = g1.clone();
    for n in [8, 11, 13] {
        put(&mut g, at(n), STATION, 1);
    }
    let stations = (0..g.hexes.len()).filter(|&h| g.tokens(h, STATION) > 0).count();
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(at(6), CITY_CHIP), 3);
    assert_eq!((0..g.hexes.len()).filter(|&h| g.tokens(h, STATION) > 0).count(), stations);
    assert!((0..g.hexes.len()).all(|h| g.tokens(h, FACTORY) == 0));
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
