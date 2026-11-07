use super::*;
use crate::content::Op;
use crate::modules::{adversary, alignment};

#[test]
fn the_build_step_grows_corrupted_cities_and_the_factories_next_to_them() {
    // The baseline's factory build is 0 (018); pinned here so it shows.
    let (rules, mut g0) = lab_with(
        &["adversary.build"],
        &["adversary.city_build=2", "adversary.factory_build=1"],
    );
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    put(&mut g0, city, ADV_CUBE, 1);
    let next_to_city = |h: usize, c: usize| m.hexes[h].neighbors.contains(&c);
    let clean_city_next = |h: usize| {
        m.hexes[h]
            .neighbors
            .iter()
            .any(|&x| kind(m, x) == HexKind::City && x != city)
    };
    let open = |h: usize| kind(m, h) == HexKind::Open;
    // A factory next to the corrupted city, and one next only to a clean city.
    let near = hexes_where(m, |h| open(h) && next_to_city(h, city))[0];
    let far = hexes_where(m, |h| {
        open(h) && !next_to_city(h, city) && clean_city_next(h)
    })[0];
    put(&mut g0, near, FACTORY, 1);
    put(&mut g0, far, FACTORY, 1);
    g0.piles.remove(CURRENT);
    let supply = g0.var(SUPPLY);

    let mut g = g0.clone();
    set_pile(&mut g, NEXT, &["mil_s1_probing_strikes"]);
    run_answered(&rules, &mut g);
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(near, ADV_AMBASSADOR),
            g.tokens(far, ADV_AMBASSADOR)
        ),
        (2, 1, 0)
    );
    assert_eq!(g.var(SUPPLY), supply - 3);
    assert_eq!(g.pile(CURRENT), ["mil_s1_probing_strikes"]);
    assert!(g.pile(NEXT).is_empty());
    assert_eq!(g.var(STAGE), 1);

    // Factory Drive: factories next to clean cities build too.
    let mut g = g0.clone();
    set_pile(&mut g, NEXT, &["mil_s1_factory_drive"]);
    run_answered(&rules, &mut g);
    assert_eq!(
        g.tokens(near, ADV_AMBASSADOR),
        1 + clean_city_next(near) as i64
    );
    assert_eq!(g.tokens(far, ADV_AMBASSADOR), 1);

    // Mobilization (stage II): corrupted cities build 1 more, and (137) the
    // escalation puts an ambassador on each country's lowest-numbered factory,
    // never on a base.
    let mut g = g0.clone();
    let esc = crate::modules::plan::escalation_hexes(&rules.world(), &g);
    assert!(!esc.is_empty());
    let lands = |h: usize| esc.iter().any(|&(x, _)| x == h) as i64;
    set_pile(&mut g, NEXT, &["mil_s2_mobilization"]);
    run_answered(&rules, &mut g);
    assert_eq!(g.tokens(city, ADV_AMBASSADOR), 3);
    assert_eq!(g.tokens(near, ADV_AMBASSADOR), 1 + lands(near));
    assert_eq!(g.tokens(far, ADV_AMBASSADOR), lands(far));
    for f in &m.fronts {
        for &b in &f.bases {
            assert_eq!(g.tokens(b, ADV_AMBASSADOR), 0);
        }
    }
    assert_eq!(g.var(STAGE), 2);
}

/// 137: escalation lands on each country's lowest-numbered factory (it was
/// each country's first base); a country with no factory gains none.
#[test]
fn escalation_lands_on_each_countrys_lowest_numbered_factory() {
    let (rules, mut g) = lab_with(&["adversary.build"], &["countries=3"]);
    let w = rules.world();
    let m = &rules.map;
    g.piles.remove(CURRENT);
    let open_in = |c: u8| {
        let mut v = hexes_where(m, |h| kind(m, h) == HexKind::Open && m.hexes[h].country == c);
        v.sort_by_key(|&h| m.hexes[h].number);
        v
    };
    let (zero, one) = (open_in(0), open_in(1));
    let (low, high, other) = (zero[0], zero[zero.len() - 1], one[0]);
    for h in [low, high, other] {
        put(&mut g, h, FACTORY, 1);
    }
    let n = w.int("adversary.escalation_ambassadors");
    let mut want = vec![(low, n), (other, n)];
    want.sort_unstable();
    assert_eq!(crate::modules::plan::escalation_hexes(&w, &g), want);
    set_pile(&mut g, NEXT, &["mil_s2_border_war"]);
    run_answered(&rules, &mut g);
    assert_eq!(
        (g.tokens(low, ADV_AMBASSADOR), g.tokens(high, ADV_AMBASSADOR), g.tokens(other, ADV_AMBASSADOR)),
        (n, 0, n)
    );
    let bases = hexes_where(m, |h| kind(m, h) == HexKind::Base);
    assert_eq!(bases.iter().map(|&b| g.tokens(b, ADV_AMBASSADOR)).sum::<i64>(), 0);
}

#[test]
fn spread_keeps_one_and_sends_the_rest_by_priority() {
    let (rules, mut g0) = lab(&["adversary.spread"]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    put(&mut g0, city, ADV_CUBE, 1);
    let land = |h: usize| kind(m, h) != HexKind::Ocean;

    // A player-owned piece first, even further than a clean city or empty land.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 2);
    let owned = hexes_where(m, |h| {
        m.distance(h, city) == 2 && kind(m, h) == HexKind::Open
    })[0];
    put(&mut g, owned, node(0), 1);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(owned, ADV_AMBASSADOR)
        ),
        (1, 1)
    );

    // Then a clean city, which collects them.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 3);
    let target = cities(m)
        .into_iter()
        .find(|&h| h != city && m.distance(h, city) <= 2)
        .expect("a city near");
    corrupt_cities_except(m, &mut g, &[target]);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(target, ADV_AMBASSADOR)
        ),
        (1, 2)
    );
    assert_eq!(total(&g, ADV_AMBASSADOR), 3);

    // Among equals the players choose: here two node next to it.
    let mut g = g0.clone();
    put(&mut g, city, ADV_AMBASSADOR, 2);
    let next: Vec<usize> = m.hexes[city]
        .neighbors
        .iter()
        .copied()
        .filter(|&h| land(h) && kind(m, h) == HexKind::Open)
        .take(2)
        .collect();
    assert_eq!(next.len(), 2);
    for &h in &next {
        put(&mut g, h, node(0), 1);
    }
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a spread choice");
    assert!(
        p.ask.prompt.starts_with("Adversary spread"),
        "{}",
        p.ask.prompt
    );
    let options: BTreeSet<usize> = p
        .ask
        .options
        .iter()
        .map(|o| m.find(o.target.unwrap()).unwrap())
        .collect();
    assert_eq!(options, next.iter().copied().collect());
    answer_hex(&rules, &mut g, next[1]);
    assert!(g.pending.is_none());
    assert_eq!(
        (
            g.tokens(city, ADV_AMBASSADOR),
            g.tokens(next[1], ADV_AMBASSADOR)
        ),
        (1, 1)
    );

    // A city that is not corrupted holds its ambassadors.
    let mut g = g0.clone();
    let clean = cities(m).into_iter().find(|&h| h != city).unwrap();
    put(&mut g, clean, ADV_AMBASSADOR, 4);
    run(&rules, &mut g);
    assert_eq!(g.tokens(clean, ADV_AMBASSADOR), 4);
    assert_eq!(total(&g, ADV_AMBASSADOR), 4);
}

/// D39: a station flip's face prints `station_flip_at`, so the two stay in step.
#[test]
fn a_station_flip_face_prints_station_flip_at() {
    let rules = Rules::new(&baseline()).unwrap();
    let n = rules.world().int("adversary.station_flip_at");
    let want = format!("unless {n} adversary");
    // A forced flip (135: "even a player's") reads no tunable.
    let flips = |effects: &[Op]| effects.iter().any(|e| matches!(e, Op::FlipStation { force: false, .. }));
    let c = crate::content();
    let texts: Vec<(&str, &str)> = c
        .threats
        .iter()
        .filter(|t| flips(&t.effects))
        .map(|t| (t.id.as_str(), t.text.as_str()))
        .chain(c.projects.iter().filter(|p| flips(&p.effects)).map(|p| (p.id.as_str(), p.text.as_str())))
        .collect();
    assert!(texts.len() >= 2, "{texts:?}");
    for (id, text) in texts {
        assert!(text.contains(&want), "{id} should say \"{want}\": {text}");
    }
}

#[test]
fn a_clean_city_holding_corrupt_at_ambassadors_is_corrupted() {
    let (rules, mut g) = lab(&["adversary.corrupt"]);
    let m = &rules.map;
    let n = rules.world().int("adversary.corrupt_at");
    let [a, b, aligned] = [0, 1, 2].map(|i| m.numbered(0, piece(0).cities[i]));
    put(&mut g, a, ADV_AMBASSADOR, n);
    put(&mut g, b, ADV_AMBASSADOR, n - 1);
    put(&mut g, aligned, ADV_AMBASSADOR, n);
    put(&mut g, aligned, cube(0), 1);
    let supply = g.var(SUPPLY);
    run(&rules, &mut g);
    // Their ambassadors go back to the supply; an aligned city loses its cube.
    assert_eq!((g.tokens(a, ADV_CUBE), g.tokens(a, ADV_AMBASSADOR)), (1, 0));
    assert_eq!(
        (g.tokens(b, ADV_CUBE), g.tokens(b, ADV_AMBASSADOR)),
        (0, n - 1)
    );
    assert_eq!(
        (g.tokens(aligned, ADV_CUBE), g.tokens(aligned, cube(0))),
        (1, 0)
    );
    assert_eq!(g.var(SUPPLY), supply + 2 * n);
    assert_eq!(g.stats.counters["adversary.corruptions"], 2);
}

#[test]
fn spread_never_targets_an_empty_hex_and_returns_the_surplus() {
    let (rules, mut g) = lab(&["adversary.spread"]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    // Every city corrupted and holding an ambassador with a botnet (not a
    // destination); no building, node or base in range: nowhere to go.
    for h in cities(m) {
        put(&mut g, h, ADV_CUBE, 1);
        if h != city {
            put(&mut g, h, ADV_AMBASSADOR, 1);
            put(&mut g, h, BOTNET, 1);
        }
    }
    let bases_near = hexes_where(m, |h| {
        kind(m, h) == HexKind::Base && m.distance(h, city) <= 2
    });
    for &b in &bases_near {
        put(&mut g, b, ADV_AMBASSADOR, 1);
        put(&mut g, b, BOTNET, 1);
    }
    put(&mut g, city, ADV_AMBASSADOR, 3);
    let supply = g.var(SUPPLY);
    let before = total(&g, ADV_AMBASSADOR);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!(g.tokens(city, ADV_AMBASSADOR), 1);
    assert_eq!(total(&g, ADV_AMBASSADOR), before - 2);
    assert_eq!(g.var(SUPPLY), supply + 2);
}

/// A spread lab whose only way out is the neighboring city it returns: a
/// corrupted city holding one ambassador, a piece, not a clean city. The other
/// pieces in range hold an ambassador and a botnet each, out of the way.
fn spread_to_neighbor() -> (Rules, GameState, usize) {
    let (rules, mut g) = lab(&["adversary.spread"]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    let target = cities(m).into_iter().find(|&h| m.adjacent(h, city)).expect("a city next to it");
    let far = cities(m).into_iter().find(|&h| m.distance(h, city) > 2).unwrap();
    corrupt_cities_except(m, &mut g, &[far]);
    put(&mut g, city, ADV_AMBASSADOR, 2);
    put(&mut g, target, ADV_AMBASSADOR, 1);
    for h in 0..m.hexes.len() {
        if h != city && h != target && m.distance(h, city) <= 2 && (kind(m, h) == HexKind::City || kind(m, h) == HexKind::Base) {
            put(&mut g, h, ADV_AMBASSADOR, 1);
            put(&mut g, h, BOTNET, 1);
        }
    }
    (rules, g, target)
}

#[test]
fn a_second_ambassador_spreading_onto_a_piece_makes_a_botnet() {
    let (rules, mut g, target) = spread_to_neighbor();
    let supply = g.var(SUPPLY);
    run(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!(
        (g.tokens(target, BOTNET), g.tokens(target, ADV_AMBASSADOR)),
        (1, 0)
    );
    assert_eq!(
        g.var(SUPPLY),
        supply + 2,
        "both ambassadors go back to the supply"
    );
    assert_eq!(g.stats.counters["adversary.botnets_made"], 1);
}

#[test]
fn botnets_start_on_the_stations_and_make_adversary_compute() {
    let rules = Rules::new(&lab_config(&["adversary.build", ACTIONS])).unwrap();
    let mut g = rules.new_game(1);
    answer_all(&rules, &mut g);
    let m = &rules.map;
    for c in 0..2 {
        assert_eq!(g.tokens(m.numbered(c, piece(c).stations[0]), BOTNET), 1);
    }
    assert_eq!(total(&g, BOTNET), 2);
    // Round 1's build has run: 3 compute a botnet (lab: botnet_compute 3), and
    // (135) the event card's standing compute.
    assert_eq!(g.stats.counters["adversary.compute_botnets"], 6);
    // Probing Strikes and War Drums each print 2 standing compute.
    let before = g.var(adversary::COMPUTE);
    set_pile(&mut g, NEXT, &["mil_s1_probing_strikes"]);
    run_answered(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 6 + 2);
    put(&mut g, m.numbered(0, piece(0).stations[0]), BOTNET, 0);
    set_pile(&mut g, NEXT, &["mil_s1_war_drums"]);
    run_answered(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 8 + 3 + 2);
}

#[test]
fn the_players_lose_when_the_next_round_has_no_adversary_card() {
    let rules = Rules::new(&lab_config(&[
        "adversary.build",
        "adversary.reveal",
        ACTIONS,
    ]))
    .unwrap();
    let mut g = rules.new_game(1);
    // The last card is face up: its round plays, the one after is lost.
    g.piles.remove(DECK);
    set_pile(&mut g, NEXT, &["mil_s3_occupation"]);
    run(&rules, &mut g);
    assert!(!g.is_over());
    assert_eq!(g.pile(CURRENT), ["mil_s3_occupation"]);
    assert!(g.pile(NEXT).is_empty());
    let deck = rules
        .view(&g)
        .globals
        .into_iter()
        .find(|v| v.key == "adversary.deck")
        .unwrap();
    assert_eq!(deck.value, 0);
    run(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "deck_empty"));
}

/// 022 (D13): before this round's build step (a Discord tip in the Sway
/// phase resolving Buildup or Offensive) the round's twists are the face-up
/// card's, the one about to become this round's; after it, the current card's.
#[test]
fn twists_before_the_build_step_are_the_face_up_cards() {
    let (rules, mut g) = lab(&["alignment.sway", "adversary.build"]);
    let w = rules.world();
    set_pile(&mut g, CURRENT, &["mil_s1_probing_strikes"]);
    set_pile(&mut g, NEXT, &["mil_s1_long_patrols"]);
    let reach = |g: &GameState| {
        adversary::twists(&w, g)
            .iter()
            .any(|t| matches!(t, Op::SpreadReach { .. }))
    };
    g.phase = 0;
    assert!(reach(&g), "the Sway phase reads the face-up card");
    g.phase = 2;
    assert!(!reach(&g), "after the build step, this round's card");
}

#[test]
fn a_stage_iii_card_pulls_sway_out_of_the_unity_bucket() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let w = rules.world();
    let card = w.content.adversary.iter().find(|c| c.stage == 3).unwrap();
    let pull = card.twists.iter().find_map(|t| match t {
        Op::PullUnity { amount } => Some(*amount),
        _ => None,
    });
    let pull = pull.expect("a stage-III card prints the pull");
    g.vars.insert(alignment::track(Side::Unity).into(), pull + 1);
    g.piles.insert(NEXT.into(), vec![card.id.as_str().into()].into());
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.stats.counters.get("alignment.unity_pulled").copied(), Some(pull));
    assert!(g.var(alignment::track(Side::Unity)) <= 1, "the bucket loses the pull");
}

/// 028: an aligned city collects spreading ambassadors as a clean one does,
/// and no botnet forms on it.
#[test]
fn an_aligned_city_collects_ambassadors_and_never_forms_a_botnet() {
    let (rules, mut g, target) = spread_to_neighbor();
    put(&mut g, target, ADV_CUBE, 0);
    put(&mut g, target, cube(0), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(target, BOTNET), g.tokens(target, ADV_AMBASSADOR)), (0, 2));
}

#[test]
fn each_stage_spreads_the_site_types() {
    use crate::op::{Op, Site};
    for stage in 1..=3u8 {
        let mut seen: Vec<String> = Vec::new();
        for a in content().adversary.iter().filter(|a| a.stage == stage) {
            let sites: Vec<&Site> = a
                .twists
                .iter()
                .filter_map(|t| match t {
                    Op::Arrivals { site } => Some(site),
                    _ => None,
                })
                .collect();
            assert_eq!(sites.len(), 1, "{} names one site", a.id);
            seen.push(format!("{:?}", sites[0]));
        }
        seen.sort();
        seen.dedup();
        // 137: coastal, industrial, stations and each country's lowest-numbered
        // factory (it took the first base's cards).
        assert_eq!(seen.len(), 4, "stage {stage}: {seen:?}");
    }
}
