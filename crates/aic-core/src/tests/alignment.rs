use super::*;
use crate::modules::adversary::PurgeLevel;
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
    // corrupted but adversary ambassadors about. The next card is Civil
    // Defense (pushback).
    uncorrupt_cities(&rules, &mut g);
    for c in 0..2 {
        put(&mut g, m.numbered(c, piece(c).cities[1]), ADV_AMBASSADOR, 2);
    }
    g.vars.insert(STAGE.into(), 1);
    unity_drawn(&mut g, 1);
    fill_bucket(&rules, &mut g, Side::Unity);
    next_alignment_card(&mut g, Side::Unity, "civil_defense");
    any_action(&rules, &mut g);
    // The tip: each country removes an ambassador.
    assert_eq!(total(&g, ADV_AMBASSADOR), 2);
    assert_eq!(g.var("alignment.unity_tips"), 2);
    assert_eq!(g.var("alignment.unity"), 0);
    assert!(g.pile("alignment.resolved.unity").iter().any(|k| k.as_str() == "civil_defense"));
    // Two Unity cards drawn: bar 3 (019), where only corrupted cities count.
    assert_eq!(alignment::bar_level(&rules.world(), &g), PurgeLevel::CitiesOnly);
    let o = g.outcome.as_ref().expect("a win");
    assert!(o.won && o.reason == "purge", "{o:?}");
}

/// 157 (r68): the tip's own count picks the tier: Grassroots Funding pays 2 a
/// round at tier I (tips 1-2: 0-1 earlier), 3 at tier II (tips 3-5: 2-4 earlier).
#[test]
fn the_tip_count_picks_a_unity_cards_tier() {
    for (tips, income, tier) in [(0, 2, 1), (1, 2, 1), (2, 3, 2), (4, 3, 2)] {
        let (rules, mut g) = action_only(&[]);
        g.vars.insert("alignment.unity_tips".into(), tips);
        assert_eq!(alignment::next_tier(&rules.world(), &g, Side::Unity), tier, "{tips} tips");
        fill_bucket(&rules, &mut g, Side::Unity);
        next_alignment_card(&mut g, Side::Unity, "grassroots_funding");
        any_action(&rules, &mut g);
        assert_eq!(alignment::income_bonus(&g), income, "{tips} tips");
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
    // 136, the baseline: 0-2 cards everything counts, 3-5 all but adversary
    // ambassadors, 6+ only the corrupted cities.
    let rules = Rules::new(&baseline_with(&[])).unwrap();
    let w = rules.world();
    for (n, level) in [(0, All), (2, All), (3, NoAmbassadors), (5, NoAmbassadors), (6, CitiesOnly), (8, CitiesOnly)] {
        unity_drawn(&mut g, n);
        assert_eq!(alignment::bar_level(&w, &g), level, "{n} cards");
    }
    assert!(All.win_text().contains("botnet, adversary troop or adversary ambassador"));
    assert!(NoAmbassadors.win_text().contains("ambassadors no longer count"));
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
        g.vars.insert(STAGE.into(), stage);
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

/// The antimatter action sways toward Discord, onto its own track: the
/// Unity track is untouched (136: no tug of war).
#[test]
fn the_antimatter_action_fills_only_the_discord_track() {
    let (rules, mut g) = action_only(&[EASE]);
    g.vars.insert("alignment.unity".into(), 2);
    g.vars.insert("alignment.discord".into(), 0);
    let am = g.resource(0, Resource::Antimatter);
    let a = Action::Antimatter { seat: 0 };
    assert!(rules.legal_actions(&g).contains(&a));
    rules.apply_mut(&mut g, &a, None).unwrap();
    assert_eq!(g.resource(0, Resource::Antimatter), am + 1);
    assert_eq!((g.var("alignment.unity"), g.var("alignment.discord")), (2, 1));
    // Once a round per player.
    assert!(!rules.legal_actions(&g).contains(&a));
    assert!(rules.apply_mut(&mut g, &a, None).is_err());
    assert!(rules
        .legal_actions(&g)
        .contains(&Action::Antimatter { seat: 1 }));
    let (rules, g) = action_only(&["alignment.antimatter_discord=0"]);
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Antimatter { .. })));
}

/// 136: Unity swayed may hold Discord off instead, 1 for 1, as the players
/// choose once the action resolves; with `hold_off` 0 it all goes on the
/// Unity track and nothing is asked.
#[test]
fn unity_swayed_may_hold_discord_off() {
    for (hold, pick, want) in [(1, 0, (3, 2)), (1, 2, (1, 0)), (0, 0, (3, 2))] {
        let (rules, mut g) = action_only(&[&format!("alignment.hold_off={hold}")]);
        g.vars.insert("alignment.discord".into(), 2);
        // Unity swayed and not yet placed: the next action's check places it.
        g.vars.insert("alignment.unity_new".into(), 3);
        set_res(&mut g, 0, Resource::ResearchCubes, 1);
        rules.apply_mut(&mut g, &std_act(0, "insight"), None).unwrap();
        if hold > 0 {
            let p = g.pending.as_ref().expect("asked how much holds Discord off");
            assert_eq!(p.ask.options.len(), 3, "0, 1 or 2 off the Discord track");
            answer(&rules, &mut g, pick);
        }
        answer_all(&rules, &mut g);
        assert_eq!(alignment::unity_pending(&g), 0);
        assert_eq!((g.var("alignment.unity"), g.var("alignment.discord")), want, "hold_off {hold}, pick {pick}");
    }
}

/// 145: a hold-off that leaves the Discord track full is not offered (the
/// track tips anyway and empties); with every amount too small, nothing is asked.
#[test]
fn only_a_hold_off_that_stops_the_tip_is_offered() {
    let (rules, g0) = action_only(&["alignment.hold_off=1"]);
    let cap = alignment::capacity(&rules.world(), &g0, crate::content::Side::Discord);
    // (Discord over the cap, Unity swayed, options offered): 1 over takes 0, 2
    // or 3 off; at the cap, 0 to 3; 3 over with 2 Unity, nothing.
    for (over, n, offered) in [(1, 3, 3), (0, 3, 4), (3, 2, 0)] {
        let mut g = g0.clone();
        g.vars.insert("alignment.discord".into(), cap + over);
        g.vars.insert("alignment.unity_new".into(), n);
        set_res(&mut g, 0, Resource::ResearchCubes, 1);
        rules.apply_mut(&mut g, &std_act(0, "insight"), None).unwrap();
        let asked = g.pending.as_ref().map_or(0, |p| p.ask.options.len());
        assert_eq!(asked, offered, "Discord {} over, {n} Unity", over);
    }
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

/// 136: Envoy spends an ambassador from the supply to move 1 Discord off the
/// Discord track; with none on the track it isn't offered.
#[test]
fn envoy_moves_one_discord_off() {
    let (rules, mut g) = action_only(&[]);
    g.seats[0].resources.insert(Resource::Ambassadors, 2);
    let envoy = |g: &GameState| {
        rules.legal_actions(g).into_iter().find(|a| matches!(a, Action::Standard { seat: 0, act } if act == "envoy"))
    };
    g.vars.insert("alignment.discord".into(), 0);
    assert!(envoy(&g).is_none());
    g.vars.insert("alignment.discord".into(), 3);
    let a = envoy(&g).expect("Envoy offered");
    act(&rules, &mut g, a);
    assert_eq!(g.var("alignment.discord"), 2);
    assert_eq!(g.resource(0, Resource::Ambassadors), 1);
    let (rules, g) = action_only(&["board.envoy_ambassadors=0"]);
    assert!(!rules.legal_actions(&g).iter().any(|a| matches!(a, Action::Standard { act, .. } if act == "envoy")));
}

/// A Discord tip no longer escalates the difficulty (017): it resolves one
/// card, one-shot; Momentum shrinks a bucket by 1 per 2 of its side's tips,
/// down to 1 per player.
#[test]
fn discord_tips_do_not_escalate_and_momentum_shrinks_buckets() {
    let (rules, mut g) = action_only(&[EASE]);
    g.vars.insert("alignment.discord_tips".into(), 1);
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
        g.vars.insert("alignment.unity_tips".into(), tips);
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
        g.vars.insert(STAGE.into(), 1);
        let cap = alignment::capacity(&rules.world(), &g, side);
        let key = format!("alignment.{}", side.name());
        let tips = alignment::tips_key(side);
        let what = format!("carry {carry}, {side:?}");
        // One tip and 3 over: one tip, and 3 left in the bucket if it carries.
        g.vars.insert(key.as_str().into(), cap + 3);
        any_action(&rules, &mut g);
        answer_all(&rules, &mut g);
        assert_eq!(g.var(tips), 1, "{what}");
        let left = if carries { 3 } else { 0 };
        assert_eq!(g.var(&key), left, "{what}");
        // Two tips' worth: two tips (and the bucket empty) only if it carries.
        let (rules, mut g) = action_only(&[EASE, &set]);
        uncorrupt_cities(&rules, &mut g);
        g.vars.insert(STAGE.into(), 1);
        g.vars.insert(key.as_str().into(), 2 * cap);
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
        let (rules, mut g) = lab(&[]);
        // 138: Blight creates on each country's lowest-numbered city holding one.
        let seeded: Vec<usize> = (0..2).map(|c| rules.map.numbered(c, piece(c).cities[0])).collect();
        for &h in &seeded {
            put(&mut g, h, ADV_AMBASSADOR, 1);
        }
        g.vars.insert("alignment.discord_tips".into(), tips);
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

