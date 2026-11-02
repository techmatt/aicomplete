use super::*;

#[test]
fn setup_places_the_printed_pieces() {
    // Difficulty 4's second corrupted city is a tunable, 0 in the baseline.
    let cases: [(i64, usize, usize, &str); 4] = [
        (0, 1, 10, ""),
        (3, 1, 9, ""),
        (4, 1, 9, ""),
        (4, 2, 9, "adversary.extra_corrupted=1"),
    ];
    for (d, corrupt, deck, set) in cases {
        let level = format!("adversary.difficulty={d}");
        let mut sets = vec![level.as_str()];
        if !set.is_empty() {
            sets.push(set);
        }
        let mut c = baseline_with(&sets);
        c.round = vec![ACTIONS.into()];
        let rules = Rules::new(&c).unwrap();
        let g = rules.new_game(1);
        let m = &rules.map;
        for c in 0..2 {
            let p = piece(c);
            let nums = |f: &dyn Fn(usize) -> bool| -> BTreeSet<u8> {
                m.countries[c]
                    .hexes
                    .iter()
                    .filter(|&&h| f(h))
                    .map(|&h| m.hexes[h].number)
                    .collect()
            };
            let corrupted = nums(&|h| kind(m, h) == HexKind::City && g.tokens(h, ADV_CUBE) > 0);
            assert_eq!(
                corrupted,
                p.corrupted[..corrupt].iter().copied().collect(),
                "d{d} country {c}"
            );
            let stations = nums(&|h| g.tokens(h, STATION) > 0);
            assert_eq!(stations, p.stations.iter().copied().collect());
            assert!(
                nums(&|h| g.tokens(h, STATION) > 0 && g.tokens(h, ADV_CUBE) == 0).is_empty(),
                "stations start corrupted"
            );
            assert_eq!(
                nums(&|h| g.tokens(h, FACTORY) > 0),
                p.factories.iter().copied().collect()
            );
            // Setup factories are neutral; each country's station holds a node.
            assert!(nums(&|h| g.tokens(h, FACTORY) > 0
                && g.hexes[h].tokens.keys().any(|k| k.starts_with("cube:")))
            .is_empty());
            assert_eq!(
                nums(&|h| g.tokens(h, NODE) > 0),
                p.stations.iter().copied().collect()
            );
        }
        for s in 0..2 {
            assert_eq!(presence_of(&g, s), [m.numbered(s, piece(s).presence)], "seat {s}");
            assert_eq!(g.resource(s, Resource::Ambassadors), 1);
            assert_eq!(g.resource(s, Resource::Credits), crate::modules::ai::start_credits(&rules.world(), s));
        }
        assert_eq!(
            total(&g, ADV_AMBASSADOR) + total(&g, TROOP) + total(&g, UNIT),
            0
        );
        assert_eq!(g.var(SUPPLY), 48, "d{d}");
        // The next adversary card is face up, from stage I.
        let next = g.pile(NEXT);
        assert_eq!(next.len(), 1);
        assert!(next[0].starts_with("adv_s1_"), "{next:?}");
        assert_eq!(g.pile(DECK).len(), deck, "d{d}");
        assert!(g.pile(CURRENT).is_empty());
    }
}

#[test]
fn each_seat_starts_with_a_bound_clean_city_at_every_size_and_difficulty() {
    use crate::modules::common::{cube, cube_owner, corrupted, is_city};
    for countries in 1..=4 {
        for sets in [vec![], vec!["adversary.difficulty=4", "adversary.extra_corrupted=1"]] {
            let mut c = baseline();
            c.apply_override(&format!("countries={countries}")).unwrap();
            for s in &sets {
                c.apply_override(s).unwrap();
            }
            let rules = Rules::new(&c).unwrap();
            let g = rules.new_game(1);
            let w = rules.world();
            for s in 0..g.seats.len() {
                let bound: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, cube(s)) > 0).collect();
                assert_eq!(bound.len(), 1, "seat {s}, {countries} countries");
                let h = bound[0];
                assert!(is_city(&w, h) && !corrupted(&g, h) && cube_owner(&g, h) == Some(s));
                let c = crate::modules::common::home_country(&w, s);
                let start = w.map.numbered(c, w.content.map.countries[c].presence);
                let nearest = w.content.map.countries[c].cities.iter().map(|&n| w.map.numbered(c, n));
                let clean_or_bound = nearest.filter(|&x| !corrupted(&g, x));
                let best = clean_or_bound.map(|x| w.map.distance(start, x)).min().unwrap();
                assert_eq!(w.map.distance(start, h), best, "the nearest");
            }
        }
    }
}

#[test]
fn start_energy_lets_a_seat_deploy_in_round_1() {
    let rules = Rules::new(&baseline()).unwrap();
    let g = rules.new_game(1);
    assert_eq!(g.resource(0, Resource::Energy), rules.world().int("ai.start_energy"));
    assert!(rules.world().int("ai.start_energy") >= 1);
}

#[test]
fn starting_credits_lean_to_the_protector() {
    let rules = Rules::new(&baseline()).unwrap();
    let g = rules.new_game(1);
    assert!(g.resource(0, Resource::Credits) < g.resource(1, Resource::Credits));
}
