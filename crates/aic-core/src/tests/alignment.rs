use super::*;
use crate::modules::adversary::PurgeLevel;
use crate::modules::common::{cube_owner, is_city};
use crate::modules::{adversary, alignment};

/// This file's purge steps come at 1, 1 and 2 Unity cards drawn.
const EASE: &str = "alignment.purge_at=1,1,2";

/// No corrupted city and no botnet left (ambassadors and troops stay).
fn uncorrupt_cities(rules: &Rules, g: &mut GameState) {
    for h in cities(&rules.map) {
        put(g, h, ADV_CUBE, 0);
    }
    for h in 0..rules.map.hexes.len() {
        put(g, h, BOTNET, 0);
    }
}

#[test]
fn a_full_unity_bucket_tips_a_card_and_lowers_the_bar() {
    let (rules, mut g) = action_only(&[EASE]);
    let m = &rules.map;
    // One Unity tip made, the bucket full (2 a player, 2 players); no city
    // corrupted but agents about. The next card is Civil
    // Defense (167: tier I removes next to a corrupted city: none here).
    uncorrupt_cities(&rules, &mut g);
    for c in 0..2 {
        put(&mut g, m.numbered(c, piece(c).cities[1]), ADV_AMBASSADOR, 2);
    }
    set_var(&mut g, STAGE,1);
    unity_drawn(&mut g, 1);
    fill_bucket(&rules, &mut g, Side::Unity);
    next_alignment_card(&mut g, Side::Unity, "civil_defense");
    any_action(&rules, &mut g);
    // The tip: no corrupted city, so no agent comes off; at this bar agents don't count.
    assert_eq!(total(&g, ADV_AMBASSADOR), 4);
    assert_eq!(g.var("alignment.unity_tips"), 2);
    assert_eq!(g.var("alignment.unity"), 0);
    assert!(g.pile("alignment.resolved.unity").iter().any(|k| k.as_str() == "civil_defense"));
    // Two Unity cards drawn: bar 3 (019), where only corrupted cities count.
    assert_eq!(alignment::bar_level(&rules.world(), &g), PurgeLevel::CitiesOnly);
    let o = g.outcome.as_ref().expect("a win");
    assert!(o.won && o.reason == "purge", "{o:?}");
}

/// 157 (r68): the tip's own count picks the tier: Grassroots Funding (167: 1 a round for each site a player
/// owns) counts at most 2 at tier I (tips 1-2: 0-1 earlier), at most 3 at tier II (tips 3-5: 2-4 earlier).
#[test]
fn the_tip_count_picks_a_unity_cards_tier() {
    for (tips, most, tier) in [(0, 2, 1), (1, 2, 1), (2, 3, 2), (4, 3, 2)] {
        let (rules, mut g) = action_only(&[]);
        let w = rules.world();
        // Four owned cities, so the tier's cap is what counts.
        let cities: Vec<usize> = (0..g.hexes.len()).filter(|&h| is_city(&w, h) && cube_owner(&g, h).is_none()).take(4).collect();
        for h in cities {
            put(&mut g, h, ADV_CUBE, 0);
            put(&mut g, h, cube(0), 1);
        }
        let owned = crate::modules::plan::sites_where(&w, &g, crate::op::Where::Owned).len() as i64;
        assert!(owned >= 3);
        set_var(&mut g, "alignment.unity_tips",tips);
        assert_eq!(alignment::next_tier(&w, &g, Side::Unity), tier, "{tips} tips");
        fill_bucket(&rules, &mut g, Side::Unity);
        next_alignment_card(&mut g, Side::Unity, "grassroots_funding");
        any_action(&rules, &mut g);
        assert_eq!(alignment::income_bonus(&g), most, "{tips} tips");
    }
}

#[test]
fn bar_levels_follow_the_unity_cards_drawn() {
    use PurgeLevel::*;
    let rules = Rules::new(&baseline_with(&[EASE])).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    // 019: `purge_at` 1, 1, 2: ambassadors and troops at 1 card, botnets at 2.
    for (tips, level) in [(0, All), (1, NoTroops), (2, CitiesOnly), (9, CitiesOnly)] {
        unity_drawn(&mut g, tips);
        assert_eq!(alignment::bar_level(&w, &g), level, "{tips} cards");
    }
    // 136: `purge_at` 3, 7, 7 (215's baseline, pinned here): 0-2 cards everything counts, 3-6 all but
    // adversary ambassadors, 7+ only the corrupted cities (troops and botnets drop together).
    let rules = Rules::new(&baseline_with(&["alignment.purge_at=3,7,7"])).unwrap();
    let w = rules.world();
    for (n, level) in [(0, All), (2, All), (3, NoAmbassadors), (6, NoAmbassadors), (7, CitiesOnly), (8, CitiesOnly)] {
        unity_drawn(&mut g, n);
        assert_eq!(alignment::bar_level(&w, &g), level, "{n} cards");
    }
    assert!(All.win_text().contains("botnet, adversary troop or agent"));
    assert!(NoAmbassadors.win_text().contains("agents no longer count"));
    assert!(NoTroops.win_text().contains("no corrupted city or botnet"));
    assert!(CitiesOnly.win_text().contains("botnets no longer count"));
}

#[test]
fn the_board_clear_win_and_the_bar_easing() {
    // (Unity tips, clear_win_stage, this round's stage, a piece left, wins)
    let cases: &[(i64, i64, i64, Option<&str>, bool)] = &[
        (0, 1, 1, None, true),
        (0, 1, 0, None, false),
        (0, 2, 1, None, false),
        (0, 2, 2, None, true),
        (0, 1, 1, Some(ADV_AMBASSADOR), false),
        (1, 1, 1, Some(ADV_AMBASSADOR), true),
        (0, 1, 1, Some(TROOP), false),
        (1, 1, 1, Some(TROOP), true),
        (5, 1, 1, Some(ADV_CUBE), false),
        (0, 1, 1, Some(BOTNET), false),
        (1, 1, 1, Some(BOTNET), false),
        (2, 1, 1, Some(BOTNET), true),
    ];
    for &(tips, rule, stage, piece_left, wins) in cases {
        let (rules, mut g) = action_only(&[EASE, &format!("alignment.clear_win_stage={rule}")]);
        uncorrupt_cities(&rules, &mut g);
        set_var(&mut g, STAGE,stage);
        unity_drawn(&mut g, tips as usize);
        if let Some(p) = piece_left {
            put(&mut g, cities(&rules.map)[0], p, 1);
        }
        any_action(&rules, &mut g);
        let won = g
            .outcome
            .as_ref()
            .is_some_and(|o| o.won && o.reason == "purge");
        assert_eq!(
            won, wins,
            "{tips} tips, rule {rule}, stage {stage}, {piece_left:?}"
        );
        if !wins {
            // The view says when the clear-board win starts to count.
            let bar = rules
                .view(&g)
                .globals
                .into_iter()
                .find(|v| v.key == "alignment.bar")
                .unwrap();
            assert_eq!(
                bar.doc.contains("from adversary stage"),
                tips < 2 && stage < rule,
                "{}",
                bar.doc
            );
        }
    }
}

/// 204: the desperate source, `board.antimatter_cubes` research cubes for 1 antimatter, sways nothing; 0 turns it
/// off.
#[test]
fn the_antimatter_action_turns_cubes_into_antimatter_and_sways_nothing() {
    let (rules, mut g) = action_only(&[EASE, "board.antimatter_cubes=3"]);
    set_var(&mut g, "alignment.unity",2);
    set_var(&mut g, "alignment.discord",0);
    g.seats[0].resources.insert(Resource::ResearchCubes, 3);
    let am = g.resource(0, Resource::Antimatter);
    let a = Action::Standard { seat: 0, act: "antimatter".into() };
    assert!(rules.legal_actions(&g).contains(&a));
    rules.apply_mut(&mut g, &a, None).unwrap();
    assert_eq!(g.resource(0, Resource::Antimatter), am + 1);
    assert_eq!(g.resource(0, Resource::ResearchCubes), 0);
    assert_eq!((g.var("alignment.unity"), g.var("alignment.discord")), (2, 0));
    assert!(!rules.legal_actions(&g).contains(&a));
    let (rules, mut g) = action_only(&["board.antimatter_cubes=0"]);
    g.seats[0].resources.insert(Resource::ResearchCubes, 5);
    assert!(!rules.legal_actions(&g).contains(&Action::Standard { seat: 0, act: "antimatter".into() }));
}

/// 145: a player with no node loses before a full Unity track tips, with no
/// Discord tip first (028's check ran only after a tip).
#[test]
fn a_player_with_no_node_loses_before_a_unity_tip() {
    let (rules, mut g) = action_only(&[EASE]);
    for h in 0..rules.map.hexes.len() {
        put(&mut g, h, node(1), 0);
    }
    fill_bucket(&rules, &mut g, Side::Unity);
    let tips = g.var("alignment.unity_tips");
    any_action(&rules, &mut g);
    assert_eq!(g.var("alignment.unity_tips"), tips, "no Unity card drawn");
    let o = g.outcome.as_ref().expect("a loss");
    assert!(!o.won && o.reason == "node", "{o:?}");
}

/// A Discord tip no longer escalates the difficulty (017): it resolves one
/// card, one-shot; Momentum shrinks a bucket by 1 per 2 of its side's tips,
/// down to 1 per player.
#[test]
fn discord_tips_do_not_escalate_and_momentum_shrinks_buckets() {
    let (rules, mut g) = action_only(&[EASE]);
    set_var(&mut g, "alignment.discord_tips",1);
    fill_bucket(&rules, &mut g, Side::Discord);
    any_action(&rules, &mut g);
    assert_eq!(g.var("alignment.discord_tips"), 2);
    let w = rules.world();
    assert!(!adversary::level(&w, 1));
    assert_eq!(g.pile("alignment.resolved.discord").len(), 1);
    let d = rules
        .view(&g)
        .globals
        .into_iter()
        .find(|v| v.key == "adversary.difficulty")
        .unwrap();
    assert_eq!(d.value, 0);

    let rules = Rules::new(&baseline_with(&["alignment.momentum_every=2", "alignment.tip_per_player=2"])).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(4);
    for (tips, cap) in [(0, 4), (1, 4), (2, 3), (4, 2), (9, 2)] {
        set_var(&mut g, "alignment.unity_tips",tips);
        assert_eq!(alignment::capacity(&w, &g, Side::Unity), cap, "{tips} tips");
        assert_eq!(alignment::capacity(&w, &g, Side::Discord), w.int("alignment.discord_tip_per_player") * 2);
    }
}

/// r48: Sway past a tip stays on the track toward the next tip; twice a tip's worth
/// tips twice. `alignment.carry` 1 (the baseline, Matt): Unity's only, a Discord tip
/// empties its track as at r47; 2 both sides, 0 neither.
#[test]
fn sway_past_a_tip_carries_over_to_the_next_tip() {
    for (carry, side, carries) in [
        (1, Side::Unity, true),
        (1, Side::Discord, false),
        (2, Side::Discord, true),
        (0, Side::Unity, false),
    ] {
        let set = format!("alignment.carry={carry}");
        let (rules, mut g) = action_only(&[EASE, &set]);
        uncorrupt_cities(&rules, &mut g);
        set_var(&mut g, STAGE,1);
        let cap = alignment::capacity(&rules.world(), &g, side);
        let key = format!("alignment.{}", side.name());
        let tips = alignment::tips_key(side);
        let what = format!("carry {carry}, {side:?}");
        // One tip and 3 over: one tip, and 3 left in the bucket if it carries.
        set_var(&mut g, &key, cap + 3);
        any_action(&rules, &mut g);
        answer_all(&rules, &mut g);
        assert_eq!(g.var(tips), 1, "{what}");
        let left = if carries { 3 } else { 0 };
        assert_eq!(g.var(&key), left, "{what}");
        // Two tips' worth: two tips (and the bucket empty) only if it carries.
        let (rules, mut g) = action_only(&[EASE, &set]);
        uncorrupt_cities(&rules, &mut g);
        set_var(&mut g, STAGE,1);
        set_var(&mut g, &key, 2 * cap);
        any_action(&rules, &mut g);
        answer_all(&rules, &mut g);
        if !g.is_over() {
            assert_eq!(g.var(tips), if carries { 2 } else { 1 }, "{what}");
            assert_eq!(g.var(&key), 0, "{what}");
        }
    }
}

#[test]
fn a_discord_tip_no_longer_moves_troops() {
    // 019: the march replaced 017's Discord-tip advance.
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let p = m.numbered(0, piece(0).node);
    let t = (0..m.hexes.len())
        .find(|&h| m.distance(p, h) == 3 && kind(m, h) != content::HexKind::Ocean)
        .unwrap();
    put(&mut g, p, node(0), 1);
    put(&mut g, t, TROOP, 1);
    next_alignment_card(&mut g, Side::Discord, "propaganda");
    fill_bucket(&rules, &mut g, Side::Discord);
    any_action(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.var("alignment.discord_tips"), 1);
    assert_eq!((g.tokens(t, TROOP), g.tokens(p, node(0))), (1, 1));
}

#[test]
fn discord_cards_step_up_with_the_discord_tip_count() {
    // 157: tier I on tips 1-2, tier III from the 6th (5 earlier tips).
    for (tips, each) in [(1, 1), (5, 3)] {
        // 263: the lab plays d0, whose city number is 5 from r124: this test's 1 + 3 are r123's 4.
        let (rules, mut g) = lab_with(&[], &["adversary.corrupt_at=4"]);
        // 138: Blight creates on each country's lowest-numbered city holding one.
        let seeded: Vec<usize> = (0..2).map(|c| rules.map.numbered(c, piece(c).cities[0])).collect();
        for &h in &seeded {
            put(&mut g, h, ADV_AMBASSADOR, 1);
        }
        set_var(&mut g, "alignment.discord_tips",tips);
        next_alignment_card(&mut g, Side::Discord, "blight");
        fill_bucket(&rules, &mut g, Side::Discord);
        let corrupt = |g: &GameState| (0..g.hexes.len()).filter(|&h| g.tokens(h, ADV_CUBE) > 0).count();
        let before = corrupt(&g);
        any_action(&rules, &mut g);
        answer_all(&rules, &mut g);
        if tips == 1 {
            assert_eq!(total(&g, ADV_AMBASSADOR), 2 + 2 * each, "{tips} tips");
            for h in seeded {
                assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1 + each, "{tips} tips");
            }
        } else {
            // 139: tier III creates 3, then Corrupt cities: the 1 + 3 there are a city's 4,
            // so a city in each country gathers them and corrupts; they return.
            assert_eq!(corrupt(&g), before + 2, "{tips} tips");
            assert_eq!(seeded.iter().map(|&h| g.tokens(h, ADV_AMBASSADOR)).sum::<i64>(), 0, "{tips} tips");
        }
    }
}

#[test]
fn alignment_cards_are_one_shot() {
    assert!(crate::content()
        .alignment
        .iter()
        .all(|c| c.tiers.len() == 3 && c.tiers.iter().all(|t| !t.effects.is_empty())));
    // The ongoing field is gone from the format: content with it is rejected.
    let bad = r#"[{"id":"x","name":"X","side":"unity","ongoing":true,"tiers":[]}]"#;
    assert!(serde_json::from_str::<Vec<content::AlignmentCard>>(bad).is_err());
}

/// 254 (`cities_unowned` 1): no city is owned, so a free city with a player's node on or next to it (292) counts,
/// an agent or a blackout on it or not, and a corrupted one doesn't, for the Unity cards that pay per site a player owns ("owned"); with r114's ownership (`cities_unowned` 0)
/// a node beside a city adds nothing.
#[test]
fn a_free_city_by_a_player_counts_as_owned_for_the_unity_cards() {
    for (sets, counted) in [(vec![], 1), (vec!["board.cities_unowned=0"], 0)] {
        let (rules, mut g) = lab_with(&[], &sets);
        let w = rules.world();
        let (city, n) = city_with_lonely_neighbors(&rules.map);
        put(&mut g, n[0], node(0), 1);
        let count = |g: &GameState| {
            crate::modules::plan::sites_where(&w, g, crate::op::Where::Owned).iter().filter(|&&h| h == city).count()
        };
        assert!(crate::modules::common::free_city_by_a_player(&w, &g, city));
        assert_eq!(count(&g), counted, "{sets:?}");
        // Blocked (an agent on it, a blackout): it pays no one and counts all the same, as the cards print.
        put(&mut g, city, ADV_AMBASSADOR, 1);
        set_var(&mut g, crate::modules::board::hex_flag(content::RoundFlag::Blackout, city), 1);
        assert!(!crate::modules::common::city_pays(&w, &g, city));
        assert_eq!(count(&g), counted, "{sets:?}, blocked");
        // Corrupted: no longer a free city.
        put(&mut g, city, ADV_CUBE, 1);
        assert_eq!(count(&g), 0, "{sets:?}, corrupted");
    }
}

/// 292: Unity never cancels Discord. Unity swayed always goes onto the Unity track once the action resolves, the
/// Discord track is untouched and nobody is asked (136-r138: it could move Discord off instead, 1 for 1); no
/// standard action moves Discord off either (Envoy is gone).
#[test]
fn unity_swayed_always_goes_onto_the_unity_track() {
    let (rules, mut g) = action_only(&[]);
    g.seats[0].resources.insert(Resource::Ambassadors, 2);
    set_var(&mut g, "alignment.discord", 2);
    // Unity swayed and not yet placed: the next action's check places it.
    set_var(&mut g, "alignment.unity_new", 3);
    assert!(!rules.legal_actions(&g).iter().any(|a| matches!(a, Action::Standard { act, .. } if act == "envoy")));
    rules.apply_mut(&mut g, &noop(1), None).unwrap();
    assert!(g.pending.is_none(), "nothing to choose");
    assert_eq!(alignment::unity_pending(&g), 0);
    assert_eq!((g.var("alignment.unity"), g.var("alignment.discord")), (3, 2));
}
