use super::*;
use crate::modules::{board, threats};

#[test]
fn the_view_shows_the_board_the_fronts_the_projects_and_the_next_card() {
    // 078: at `city_build` 2 every opening stops at a spread choice; at 1 some
    // stop at the draft, which is what this reads.
    let rules = Rules::new(&baseline_with(&["adversary.city_build=1", "adversary.wall_troops=0"])).unwrap();
    let m = &rules.map;
    // A seed whose opening stops at the draft, not at a spread choice
    // before next round's card is revealed.
    let g = (0..64)
        .map(|s| rules.new_game(s))
        .find(|g| g.pending.is_none())
        .unwrap();
    let v = rules.view(&g);
    assert_eq!(v.hexes.len(), g.hexes.len());
    let count = |k: &str| v.hexes.iter().filter(|h| h.kind == k).count();
    assert_eq!(count("city"), 6);
    assert_eq!(count("base"), 2 * m.fronts.len());
    assert_eq!(count("ocean"), piece(0).ocean.len() + piece(1).ocean.len());
    assert_eq!(
        count("city") + count("base") + count("open") + count("ocean"),
        32
    );
    for h in &v.hexes {
        assert!((1..=16).contains(&h.number));
        assert_eq!(h.facing.is_some(), h.kind == "base");
        // Tokens as in the state, non-zero and sorted.
        let want: Vec<(String, i64)> = g.hexes[h.index]
            .tokens
            .iter()
            .map(|(k, &n)| (k.to_string(), n))
            .collect();
        let have: Vec<(String, i64)> = h.tokens.iter().map(|t| (t.kind.clone(), t.count)).collect();
        assert_eq!(have, want);
    }
    let corrupted = m.numbered(0, piece(0).corrupted[0]);
    assert!(v.hexes[corrupted].tokens.iter().any(|t| t.kind == ADV_CUBE));
    let d = m.numbered(1, piece(1).node);
    assert!(v.hexes[d].tokens.iter().any(|t| t.kind == node(1)));

    let global = |k: &str| {
        v.globals
            .iter()
            .find(|x| x.key == k)
            .unwrap_or_else(|| panic!("{k} missing"))
    };
    assert_eq!(global("board.fronts").value, m.fronts.len() as i64);
    let clock = global("board.clock");
    assert_eq!(clock.value, 1);
    assert!(
        clock.doc.contains(
            "1: production, 2: war, 3: production, 4: war, 5: production, 6: production + war"
        ),
        "{}",
        clock.doc
    );
    assert!(global("adversary.compute").value > 0);
    assert_eq!(global("board.war").value, 0);
    let supply = global("adversary.supply");
    assert_eq!((supply.value, supply.max), (g.var(SUPPLY), Some(48)));
    assert_eq!(
        global("adversary.deck").value,
        (g.pile(DECK).len() + 1) as i64
    );
    let next = v.piles.iter().find(|p| p.key == "adversary.next").unwrap();
    assert_eq!(next.count, 1);
    assert_eq!(next.visible[0].id, g.pile(NEXT)[0]);
    assert_eq!(next.visible[0].status, "face up: next round");
    let deck = v.piles.iter().find(|p| p.key == "adversary.deck").unwrap();
    assert!(deck.visible.is_empty() && deck.count == g.pile(DECK).len());

    for (c, cv) in v.countries.iter().enumerate() {
        let value = |k: &str| {
            cv.values
                .iter()
                .find(|x| x.key == k)
                .unwrap_or_else(|| panic!("{k} missing"))
        };
        let p = value("board.project");
        assert_eq!(p.max, Some(piece(c).megaproject.cubes));
        assert!(p.label.contains(&piece(c).megaproject.name));
        assert_eq!(
            value("board.project.adversary").value,
            g.var(&board::project_key(c, None))
        );
        assert_eq!(value("tokens.factory").value, 2);
    }
    for s in &v.seats {
        let value = |k: &str| {
            s.values
                .iter()
                .find(|x| x.key == k)
                .unwrap_or_else(|| panic!("{k} missing"))
        };
        assert_eq!(value("board.node").value, 1);
        assert_eq!(value("board.node_track").value, 6);
        assert_eq!(
            value("resource.ambassadors").value,
            g.resource(s.index as usize, Resource::Ambassadors)
        );
    }
}

#[test]
fn the_view_follows_every_variant_and_keeps_bookkeeping_out_of_globals() {
    // Flags shown elsewhere: game-wide keys, but still never globals.
    const FLAGS: &[&str] = &[
        "adversary.peek",
        "adversary.first_project",
        "adversary.first_corruption",
    ];
    let names: Vec<&str> = crate::content()
        .map
        .countries
        .iter()
        .map(|c| c.name.as_str())
        .collect();
    let (mut saw_track_var, mut saw_project, mut saw_choice) = (false, false, false);
    for config in variant_matrix() {
        let rules = Rules::new(&config).unwrap();
        let shared = config.module("threats").unwrap().variant == "shared_row";
        let mut g = rules.new_game(5);
        for step in 0..300 {
            if g.is_over() {
                break;
            }
            let v = rules.view(&g);
            assert_eq!(v.hexes.len(), g.hexes.len());
            assert_eq!(v.choice.is_some(), g.pending.is_some());
            saw_choice |= v.choice.is_some();
            for (c, cv) in v.countries.iter().enumerate() {
                assert_eq!(cv.name, names[c]);
                assert!(cv.values.iter().any(|x| x.key == "board.project"));
            }
            for s in &v.seats {
                let b = s.board.as_ref().expect("every seat shows its AI board");
                // 021: two node tracks; the second has nothing uncovered at setup.
                assert_eq!((b.key.as_str(), b.tracks.len()), ("ai.board", 2));
                for (i, t) in b.tracks.iter().enumerate() {
                    let least = if i == 0 { 1 } else { 0 };
                    assert!((least..=t.steps.len()).contains(&t.uncovered));
                    assert_eq!(t.current, t.uncovered.saturating_sub(1));
                }
                assert!(s.cards.iter().any(|c| c.key == "cards.hand"));
            }
            let threats_shown: usize = if shared {
                v.piles
                    .iter()
                    .find(|p| p.key == "threats.row")
                    .map_or(0, |p| p.count)
            } else {
                v.countries
                    .iter()
                    .flat_map(|c| &c.cards)
                    .map(|c| c.cards.len())
                    .sum()
            };
            assert_eq!(threats_shown, threats::active(&g).len());
            for x in &v.globals {
                // Per-seat, per-country and per-card `vars` carry a third
                // key segment: `module.name` is the shape of a game-wide one.
                let game_wide = x.key.matches('.').count() == 1 && !FLAGS.contains(&x.key.as_str());
                assert!(
                    game_wide || x.source != "engine",
                    "bookkeeping {} in globals",
                    x.key
                );
            }
            for k in [
                "alignment.undecided",
                "alignment.unity_tips",
                "alignment.bar",
                "adversary.supply",
                "adversary.deck",
                "adversary.compute",
                "board.clock",
                "board.war",
                "board.fronts",
            ] {
                assert!(v.globals.iter().any(|x| x.key == k), "{k}");
            }
            assert!(v.piles.iter().any(|p| p.key == "adversary.next"));
            saw_track_var |= g.vars.keys().any(|k| k.starts_with("board.track."));
            saw_project |= g.vars.keys().any(|k| k.starts_with("board.project."));
            let legal = rules.legal_actions(&g);
            let a = legal[step * 7919 % legal.len()].clone();
            rules.apply_mut(&mut g, &a, None).unwrap();
        }
    }
    // The games really did hold the bookkeeping the view hides.
    assert!(saw_track_var && saw_project && saw_choice);
}
