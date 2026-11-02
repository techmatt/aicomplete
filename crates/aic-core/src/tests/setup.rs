use super::*;

#[test]
fn setup_places_the_printed_pieces() {
    // Difficulty 4's second corrupted city is a tunable, 0 in the baseline.
    let cases: [(i64, usize, usize, i64, &str); 4] = [
        (0, 1, 10, 48, ""),
        (3, 1, 9, 48, ""),
        (4, 1, 9, 48, ""),
        (4, 2, 9, 48, "adversary.extra_corrupted=1"),
    ];
    for (d, corrupt, deck, supply, set) in cases {
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
                nums(&|h| g.tokens(h, crate::modules::common::NODE) > 0),
                p.stations.iter().copied().collect()
            );
        }
        for s in 0..2 {
            let want: Vec<usize> = vec![m.numbered(s, piece(s).presence)];
            let mut have = presence_of(&g, s);
            have.sort();
            let mut want = want;
            want.sort();
            assert_eq!(have, want, "seat {s}");
            assert_eq!(g.resource(s, Resource::Ambassadors), 1);
            assert_eq!(g.resource(s, Resource::Credits), rules.world().int("ai.start_credits"));
        }
        assert_eq!(
            total(&g, ADV_AMBASSADOR) + total(&g, TROOP) + total(&g, UNIT),
            0
        );
        assert_eq!(g.var(SUPPLY), supply, "d{d}");
        // The next adversary card is face up, from stage I.
        let next = g.pile(NEXT);
        assert_eq!(next.len(), 1);
        assert!(next[0].starts_with("adv_s1_"), "{next:?}");
        assert_eq!(g.pile(DECK).len(), deck, "d{d}");
        assert!(g.pile(CURRENT).is_empty());
    }
}
