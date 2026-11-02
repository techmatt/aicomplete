use super::*;

#[test]
fn same_seed_same_game() {
    let rules = Rules::new(&baseline()).unwrap();
    assert_eq!(rules.new_game(7), rules.new_game(7));
    assert_ne!(rules.new_game(7).piles, rules.new_game(8).piles);
}

#[test]
fn state_round_trips_through_json_with_string_seeds() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(u64::MAX - 3);
    let a = rules.legal_actions(&g)[0].clone();
    rules.apply_mut(&mut g, &a, None).unwrap();
    let json = g.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["seed"], serde_json::json!((u64::MAX - 3).to_string()));
    assert!(v["rng"]["cards"]["word_pos"].is_string());
    let back: GameState = serde_json::from_str(&json).unwrap();
    assert_eq!(g, back);
    for a in rules.legal_actions(&g) {
        let back: Action = serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
        assert_eq!(a, back);
    }
}

#[test]
fn every_country_count_plays_to_an_end_and_replays() {
    for n in 1..=4u8 {
        let mut c = baseline();
        c.countries = n;
        let rules = Rules::new(&c).unwrap();
        for seed in 0..4 {
            let (g, actions) = random_game(&rules, seed);
            let o = g.outcome.as_ref().expect("the game ends");
            assert_ne!(o.reason, "round_limit", "{n} countries, seed {seed}: {o:?}");
            let again = replay(&rules, seed, &actions).unwrap();
            assert_eq!(again.fingerprint(), g.fingerprint());
        }
    }
}

/// Fast lane: one seed per variant, the JSON path on every fourth config;
/// the slow lane plays more seeds, all through the JSON path.
#[test]
fn random_games_replay_exactly_in_every_variant() {
    random_games_replay_in_every_variant(0..1, 4);
}

#[test]
#[ignore = "slow lane: more seeds of the variant replay check, all through the JSON path"]
fn random_games_replay_exactly_in_every_variant_more_seeds() {
    random_games_replay_in_every_variant(1..6, 1);
}

fn random_games_replay_in_every_variant(seeds: std::ops::Range<u64>, json_every: usize) {
    for (i, config) in variant_matrix().into_iter().enumerate() {
        let rules = Rules::new(&config).unwrap();
        for seed in seeds.clone() {
            let (g, actions) = random_game(&rules, seed);
            assert!(g.is_over(), "{} seed {seed} did not finish", config.hash());
            assert_eq!(replay(&rules, seed, &actions).unwrap(), g);
            if i % json_every != 0 {
                continue;
            }
            // The slow, JSON-shaped path agrees with the hot path, and a
            // state survives serialization at any point (mid-choice too).
            let mut s = Rules::new(&config).unwrap().new_game(seed);
            for (k, a) in actions.iter().enumerate() {
                if k % 5 == 0 {
                    s = serde_json::from_str(&s.to_json()).unwrap();
                }
                s = apply(&s, a).unwrap().0;
            }
            assert_eq!(s, g);
        }
    }
}

#[test]
fn rng_streams_do_not_depend_on_module_order() {
    let a = baseline();
    let mut b = baseline();
    b.modules.reverse();
    let (ra, rb) = (Rules::new(&a).unwrap(), Rules::new(&b).unwrap());
    assert_ne!(ra.config_hash, rb.config_hash);
    let (ga, actions) = random_game(&ra, 3);
    let gb = replay(&rb, 3, &actions).unwrap();
    assert_eq!(ga.hexes, gb.hexes);
    assert_eq!(ga.seats, gb.seats);
}

const GOLDEN_DRAWS: [usize; 8] = [10, 13, 7, 2, 12, 9, 1, 7];

#[test]
fn rng_draws_are_fixed_width_and_pinned() {
    // Pinned so a native/wasm or rand-version difference shows up here.
    let mut s = rng::Stream::new(42, "spread");
    let draws: Vec<usize> = (0..8).map(|_| s.below(16)).collect();
    assert_eq!(draws, GOLDEN_DRAWS);
}

/// A state waiting on a spread choice (players choose among equals).
fn spread_choice() -> (Rules, GameState) {
    let (rules, mut g) = lab(&["adversary.spread"]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).corrupted[0]);
    let far = cities(m)
        .into_iter()
        .find(|&h| m.distance(h, city) > 2)
        .unwrap();
    corrupt_cities_except(m, &mut g, &[far]);
    put(&mut g, city, ADV_AMBASSADOR, 2);
    // Two presence next to it: equals, so the players choose.
    let next: Vec<usize> = m.hexes[city]
        .neighbors
        .iter()
        .copied()
        .filter(|&h| kind(m, h) == HexKind::Open)
        .take(2)
        .collect();
    for h in next {
        put(&mut g, h, presence(0), 1);
    }
    run(&rules, &mut g);
    assert!(g.pending.is_some());
    (rules, g)
}

#[test]
fn malformed_and_mismatched_states_are_errors_not_panics() {
    let (rules, g) = spread_choice();
    assert!(rules.check_state(&g).is_ok());
    // Mid-choice states survive JSON, and only answers are legal.
    let back: GameState = serde_json::from_str(&g.to_json()).unwrap();
    assert_eq!(back, g);
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| matches!(a, Action::Choose { .. })));
    assert!(rules
        .apply_mut(&mut g.clone(), &Action::Pass { seat: 0 }, None)
        .is_err());
    let mut bad = g.clone();
    let p = std::sync::Arc::make_mut(bad.pending.as_mut().unwrap());
    std::sync::Arc::make_mut(&mut p.snapshot)
        .seats
        .pop();
    assert!(rules.check_state(&bad).is_err());
    let mut bad = g.clone();
    std::sync::Arc::make_mut(bad.pending.as_mut().unwrap()).ask.seat = 9;
    assert!(rules.check_state(&bad).is_err());
    let mut bad = g.clone();
    std::sync::Arc::make_mut(bad.pending.as_mut().unwrap()).ask.options.truncate(1);
    assert!(rules.check_state(&bad).is_err());

    let config = baseline();
    let mut g = Rules::new(&config).unwrap().new_game(1);
    g.rng.clear();
    assert!(legal_actions(&g).err().unwrap().contains("RNG streams"));
    let mut g = Rules::new(&config).unwrap().new_game(1);
    g.seats.pop();
    assert!(apply(&g, &Action::Pass { seat: 0 }).is_err());
    let mut g = Rules::new(&config).unwrap().new_game(1);
    g.hexes.pop();
    assert!(view(&g).is_err());
    let other = Rules::new(&baseline_with(&["countries=3"])).unwrap();
    assert!(other.check_state(&Rules::new(&config).unwrap().new_game(1)).is_err());
}

#[test]
fn saved_games_round_trip_and_replay_to_the_saved_state() {
    let config = baseline_with(&["threats.variant=shared_row"]);
    let rules = Rules::new(&config).unwrap();
    let (g, actions) = random_game(&rules, 21);
    let rec = SavedGame::new(&rules, 21, actions.clone(), &g, "a loss");
    let text = rec.to_text();
    assert!(text.starts_with("{\n  \"format\": \"aic-game-record/1\",\n  \"note\": \"a loss\""));
    let back = SavedGame::from_json(&text).unwrap();
    assert_eq!(back, rec);
    let (h, warnings) = back.replay().unwrap();
    assert_eq!((h, warnings), (g.clone(), vec![]));

    // The minimal hand-written form loads too.
    for seed in [serde_json::json!("21"), serde_json::json!(21)] {
        let minimal = serde_json::json!({
            "config": config, "seed": seed, "actions": actions,
        });
        let (h, warnings) = SavedGame::from_json(&minimal.to_string())
            .unwrap()
            .replay()
            .unwrap();
        assert_eq!((h, warnings), (g.clone(), vec![]));
    }

    let mut bad = rec.clone();
    bad.final_fingerprint = "0000000000000000".into();
    let w = bad.replay().unwrap().1;
    assert!(w[0].contains("fingerprint"), "{w:?}");
    let mut bad = rec.clone();
    bad.actions.insert(0, Action::Pass { seat: 9 });
    assert!(bad.replay().err().unwrap().contains("action 0"));
    let other = text.replace("aic-game-record/1", "aic-game-record/9");
    assert!(SavedGame::from_json(&other)
        .err()
        .unwrap()
        .contains("format"));
    let empty = SavedGame::new(&rules, 1, vec![], &rules.new_game(1), "");
    assert_eq!(SavedGame::from_json(&empty.to_text()).unwrap(), empty);
}

/// 020: a hand-written record's seed may be a number; a bad one names the seed.
#[test]
fn a_hand_written_record_s_seed_may_be_a_number() {
    let rec = |seed: &str| format!(r#"{{"config": {BASELINE}, "seed": {seed}, "actions": []}}"#);
    assert!(SavedGame::from_json(&rec("5")).is_ok());
    assert!(SavedGame::from_json(&rec("\"5\"")).is_ok());
    for bad in ["-1", "5.5"] {
        let e = SavedGame::from_json(&rec(bad)).unwrap_err();
        assert!(e.contains("seed"), "{bad}: {e}");
    }
}

