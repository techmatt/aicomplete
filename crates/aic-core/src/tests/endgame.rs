//! 027: the two decks as clocks, the presence loss, arrivals, the hold, the
//! piece ladder and what destroys presence.

use super::*;
use crate::content::Op;
use crate::modules::common::{hex_opt, is_ocean, MARKER};
use crate::modules::{adversary, alignment, board};
use crate::state::Pick;

/// The fallback's numbers (the lab pins them off).
const HOLD: [&str; 2] = ["board.liberate_at=5", "board.free_leaves=1"];

fn empty_deck(g: &mut GameState, side: &str) {
    for tier in 1..=2 {
        g.piles.insert(format!("alignment.deck.{side}.{tier}").into(), Vec::new().into());
    }
}

#[test]
fn a_discord_tip_with_an_empty_deck_loses_and_nothing_reshuffles() {
    let (rules, mut g) = action_only(&[]);
    empty_deck(&mut g, "discord");
    set_pile(&mut g, "alignment.resolved.discord", &["blight"]);
    fill_bucket(&rules, &mut g, Side::Discord);
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "discord_deck"));
    assert_eq!(g.pile("alignment.resolved.discord"), ["blight"]);
}

#[test]
fn drawing_the_last_unity_card_wins_outright() {
    let (rules, mut g) = action_only(&[]);
    empty_deck(&mut g, "unity");
    set_pile(&mut g, "alignment.deck.unity.1", &["civil_defense"]);
    fill_bucket(&rules, &mut g, Side::Unity);
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a win");
    assert_eq!((o.won, o.reason.as_str()), (true, "unity_deck"));
}

#[test]
fn a_player_with_no_presence_loses_the_game_for_everyone() {
    let (rules, mut g) = action_only(&[]);
    for h in 0..g.hexes.len() {
        put(&mut g, h, presence(1), 0);
    }
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "presence"));
}

#[test]
fn the_supply_is_a_cap_not_a_loss() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let city = rules.map.numbered(0, piece(0).corrupted[0]);
    put(&mut g, city, ADV_CUBE, 1);
    g.vars.insert(SUPPLY.into(), 1);
    set_pile(&mut g, NEXT, &["adv_s1_long_patrols"]);
    run(&rules, &mut g);
    assert_eq!(g.tokens(city, ADV_AMBASSADOR), 1);
    assert!(!g.is_over());
}

#[test]
fn every_corrupted_city_is_no_loss() {
    let (rules, mut g) = lab(&[]);
    corrupt_cities_except(&rules.map, &mut g, &[]);
    give(&rules, &mut g);
    assert!(!g.is_over());
}

#[test]
fn a_card_brings_arrivals_to_every_site_it_names_a_round_ahead() {
    let (rules, mut g) = lab_with(&["adversary.build"], &["adversary.arrivals=1"]);
    g.vars.insert(SUPPLY.into(), 40);
    set_pile(&mut g, NEXT, &["adv_s1_probing_strikes"]);
    let w = rules.world();
    let plan = crate::modules::plan::next_build(&w, &g);
    let coast = hexes_where(&rules.map, |h| rules.map.hexes[h].coastal);
    assert_eq!(plan.iter().map(|&(h, _)| h).collect::<Vec<_>>(), coast);
    assert_eq!(coast.len(), 4, "2 a country");
    run(&rules, &mut g);
    for h in coast {
        assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1);
    }
}

#[test]
fn the_fallback_is_a_flat_count_whatever_the_adversary_holds_nearby() {
    let (rules, mut g) = lab_with(&[], &HOLD);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let n = rules.map.hexes[h].neighbors[0];
    put(&mut g, h, ADV_CUBE, 1);
    put(&mut g, n, ADV_AMBASSADOR, 2);
    put(&mut g, n, presence(0), 1);
    put(&mut g, n, presence(1), 1);
    put(&mut g, h, ambassador(0), 2);
    put(&mut g, h, ambassador(1), 2);
    give(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 1, "4 of 5");
    put(&mut g, h, ambassador(1), 3);
    // The next end check liberates it: any players, adversary ambassadors next to it don't count.
    give(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 0);
    assert_eq!(g.tokens(h, ambassador(0)) + g.tokens(h, ambassador(1)), 0);
    // Down the ladder, not off the board.
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1);
    assert_eq!(g.stats.counters["board.liberated_fallback"], 1);
}

#[test]
fn the_fallback_counts_only_seats_with_presence_on_or_next_to_the_city() {
    let (rules, mut g) = lab_with(&[], &HOLD);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, ADV_CUBE, 1);
    put(&mut g, h, ambassador(0), 5);
    give(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 1, "no presence near it");
    put(&mut g, rules.map.hexes[h].neighbors[0], presence(0), 1);
    give(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 0);
}

#[test]
fn stage_one_never_takes_a_last_presence() {
    let (rules, mut g) = lab_with(&[], &["board.last_safe_stage=2"]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, presence(0), 1);
    g.vars.insert(STAGE.into(), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    assert_eq!(g.tokens(h, presence(0)), 1, "stage I: it stays");
    let gone = with_cx(&rules, |cx| {
        board::destroy_piece(&mut g, cx, h, presence(0), board::Cause::Troop)
    });
    assert!(!gone);
    g.vars.insert(STAGE.into(), 2);
    let gone = with_cx(&rules, |cx| {
        board::destroy_piece(&mut g, cx, h, presence(0), board::Cause::Troop)
    });
    assert!(gone, "stage II: it goes");
}

#[test]
fn a_discord_card_takes_the_boards_most_threatened_hex() {
    use crate::modules::effects;
    let (rules, mut g) = lab(&[]);
    let (a, b) = (rules.map.numbered(0, piece(0).presence), rules.map.numbered(1, piece(1).presence));
    put(&mut g, a, presence(0), 1);
    put(&mut g, b, presence(1), 1);
    put(&mut g, b, ADV_AMBASSADOR, 2);
    with_cx(&rules, |cx| {
        effects::run(&mut g, cx, &[Op::DestroyPresence], &mut effects::Ctx::team().named("discord"))
    })
    .unwrap();
    assert_eq!((g.tokens(a, presence(0)), g.tokens(b, presence(1))), (1, 0));
    assert_eq!(g.stats.counters["board.presence_lost.discord"], 1);
}

#[test]
fn a_choice_threat_never_takes_a_last_presence() {
    use crate::modules::threats;
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).presence);
    put(&mut g, h, presence(0), 1);
    set_res(&mut g, 0, Resource::Credits, 2);
    // No option is left: the player pays what they have and keeps the presence.
    with_cx(&rules, |cx| {
        threats::lose_presence_or(&mut g, cx, Some(0), Resource::Credits, 6, "threat1")
    })
    .unwrap();
    assert_eq!(g.tokens(h, presence(0)), 1);
    assert_eq!(g.resource(0, Resource::Credits), 0);
}

// 028: rules revision 17.

/// Lands `s`'s presence on `hexes` and counts them on the track.
fn presence_on_track(g: &mut GameState, s: usize, hexes: &[usize]) {
    for &h in hexes {
        put(g, h, presence(s), 1);
    }
    g.vars.insert(board::track_key(s).into(), hexes.len() as i64);
}

#[test]
fn each_player_destroys_a_presence_of_their_choice_but_not_a_stage_one_last() {
    use crate::modules::common::hex_name;
    let (rules, mut g) = lab_with(&[], &["board.last_safe_stage=2"]);
    let (a, b) = (rules.map.numbered(0, piece(0).presence), rules.map.numbered(0, piece(0).cities[0]));
    let last = rules.map.numbered(1, piece(1).presence);
    put(&mut g, a, presence(0), 1);
    put(&mut g, b, presence(0), 1);
    put(&mut g, last, presence(1), 1);
    g.vars.insert(STAGE.into(), 1);
    let hit = |g: &mut GameState, cx: &mut module::Cx, s: usize, h: usize| {
        let hexes = [h, b];
        board::destroy_own_presence(g, cx, s, &hexes, board::Cause::Effect("discord")).unwrap();
    };
    with_cx(&rules, |cx| hit(&mut g, cx, 1, last));
    assert_eq!(g.tokens(last, presence(1)), 1, "stage I: it stays");
    // The player picks which: here the second of seat 0's two.
    let pick = crate::state::Pick { option: hex_name(&rules.world(), b), target: Some(rules.map.hexes[b].hex) };
    with_cx_picks(&rules, vec![pick], |cx| hit(&mut g, cx, 0, a));
    assert_eq!((g.tokens(a, presence(0)), g.tokens(b, presence(0))), (1, 0));
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| hit(&mut g, cx, 1, last));
    assert_eq!(g.tokens(last, presence(1)), 0, "stage II: it goes");
}

#[test]
fn a_game_deals_the_unity_deck_it_is_told_to() {
    let (_, g) = action_only(&["alignment.unity_cards=3,2"]);
    assert_eq!(g.pile("alignment.deck.unity.1").len(), 3);
    assert_eq!(g.pile("alignment.deck.unity.2").len(), 2);
}

#[test]
fn removing_a_node_clears_a_hack_on_its_hex() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, NODE, 1);
    g.vars.insert(adversary::cut_key(h).into(), 1);
    with_cx(&rules, |cx| adversary::remove_node(&mut g, cx, h));
    assert_eq!(g.var(&adversary::cut_key(h)), 0);
}

/// A Discord tip that leaves a seat with no presence loses before a Unity tip
/// in the same check can draw the last card.
#[test]
fn a_lost_last_presence_comes_before_a_unity_tip_in_the_same_check() {
    let (rules, mut g) = action_only(&[]);
    for h in 0..g.hexes.len() {
        put(&mut g, h, presence(1), 0);
    }
    empty_deck(&mut g, "unity");
    set_pile(&mut g, "alignment.deck.unity.1", &["civil_defense"]);
    fill_bucket(&rules, &mut g, Side::Discord);
    fill_bucket(&rules, &mut g, Side::Unity);
    give(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "presence"));
}

// ---------- 034: presence leaves the game, the marker ----------

#[test]
fn a_city_that_corrupts_destroys_presence_on_it_and_leaves_no_marker() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let far = rules.map.numbered(1, piece(1).cities[0]);
    put(&mut g, h, presence(0), 1);
    put(&mut g, far, presence(0), 1);
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    assert_eq!((g.tokens(h, presence(0)), g.tokens(h, MARKER)), (0, 0), "it has corruption already");
}

#[test]
fn corruption_reach_takes_presence_around_the_city() {
    let (rules, mut g) = lab_with(&[], &["board.corruption_reach=1"]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let n = rules.map.hexes[h].neighbors[0];
    let far = rules.map.numbered(1, piece(1).cities[0]);
    put(&mut g, n, presence(0), 1);
    put(&mut g, far, presence(0), 1);
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    assert_eq!((g.tokens(n, presence(0)), g.tokens(n, MARKER)), (0, 1));
}

#[test]
fn a_lost_presence_leaves_the_game_and_its_income_stays() {
    use crate::modules::ai;
    let (rules, mut g) = lab(&[]);
    let (a, b) = (rules.map.numbered(0, piece(0).presence), rules.map.numbered(0, piece(0).cities[0]));
    let c = rules.map.hexes[a].neighbors[0];
    presence_on_track(&mut g, 0, &[a, b, c]);
    let w = rules.world();
    let income = ai::track_income(&w, &g, 0);
    with_cx(&rules, |cx| board::lose_presence(&mut g, cx, c, 0, "test"));
    assert_eq!(board::track_taken(&g, 0), 3, "it does not go back on the track");
    assert_eq!(ai::track_income(&w, &g, 0), income);
    assert_eq!(g.tokens(c, MARKER), 1);
}

#[test]
fn a_marker_adds_nothing_where_there_is_corruption_and_blocks_the_hex() {
    let (rules, mut g) = lab(&[]);
    let a = rules.map.numbered(0, piece(0).presence);
    let n = rules.map.hexes[a].neighbors[0];
    put(&mut g, a, presence(0), 1);
    put(&mut g, n, presence(1), 1);
    put(&mut g, n, NODE, 1);
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| board::lose_presence(&mut g, cx, n, 1, "test"));
    assert_eq!(g.tokens(n, MARKER), 0, "a node is corruption");
    put(&mut g, n, MARKER, 1);
    let w = rules.world();
    assert!(!board::presence_hexes(&w, &g, 0).contains(&n));
    with_cx(&rules, |cx| board::lose_presence(&mut g, cx, a, 0, "test"));
    assert_eq!(g.tokens(a, MARKER), 1);
}

#[test]
fn a_marker_counts_toward_corrupting_a_city() {
    let (rules, mut g) = lab_with(&["adversary.corrupt"], &["adversary.corrupt_at=2"]);
    let h = rules.map.numbered(0, piece(0).cities[1]);
    put(&mut g, h, MARKER, 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
    assert_eq!(g.tokens(h, MARKER), 0, "the city's corruption replaces it");
}

#[test]
fn up_to_two_markers_can_stop_after_one() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let c = rules.map.numbered(0, piece(0).presence);
    let n: Vec<usize> = w.map.hexes[c].neighbors.iter().copied().filter(|&h| !is_ocean(&w, h)).take(2).collect();
    put(&mut g, c, presence(0), 1);
    for &h in &n {
        put(&mut g, h, MARKER, 1);
    }
    let first = hex_opt(&w, n[0]);
    let picks = vec![Pick { option: first.option, target: first.target }, Pick { option: "stop".into(), target: None }];
    run_ops_with(&rules, &mut g, picks, &[Op::ClearMarkers { count: 2 }]);
    assert_eq!(n.iter().map(|&h| g.tokens(h, MARKER)).sum::<i64>(), 1, "the second pick was stop");
}

#[test]
fn clearing_a_marker_costs_its_credits_and_pays_its_reward() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).presence);
    let marked = w.map.hexes[a].neighbors.iter().copied().find(|&h| !is_ocean(&w, h)).unwrap();
    put(&mut g, marked, MARKER, 1);
    assert!(!board::presence_hexes(&w, &g, 0).contains(&marked), "no presence on a marker");
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Credits, 5);
    let credits = 5 - w.int("board.clear_cost");
    let unity = g.var(alignment::bucket(Side::Unity));
    rules.apply_mut(&mut g, &std_act(0, "clear"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(marked, MARKER), 0);
    assert_eq!(g.resource(0, Resource::Credits), credits + w.int("board.clear_credits"));
    assert_eq!(g.var(alignment::bucket(Side::Unity)), unity + w.int("board.clear_unity"));
}

/// 034's d4 fix: every printed step of the Discord deck acts before it ends,
/// at each difficulty, with the deck's tips counted as the game counts them.
#[test]
fn every_discord_step_is_reached_at_every_difficulty() {
    for d in 0..=4 {
        let arg = format!("adversary.difficulty={d}");
        let (rules, g) = lab_with(&[], &[&arg]);
        let w = rules.world();
        let start = g.var("alignment.discord_tips").max(0);
        let cards = w.content.alignment.iter().filter(|c| c.side == crate::content::Side::Discord).count() as i64;
        let cut = if d >= 4 { w.int("adversary.discord_removed") } else { 0 };
        let last = start + cards - cut - 1;
        for c in w.content.alignment.iter().filter(|c| c.side == crate::content::Side::Discord) {
            for st in &c.steps {
                assert!(st.at <= last, "{} step at {} never acts at d{d} (last tip {last})", c.id, st.at);
            }
        }
    }
}

#[test]
fn the_round_cap_is_the_deck_and_higher_difficulty_shortens_it() {
    let cap = |d: i64| {
        let arg = format!("adversary.difficulty={d}");
        let (_, g) = lab_with(&[], &[&arg]);
        g.pile(DECK).len() + g.pile(NEXT).len()
    };
    assert_eq!((cap(0), cap(1), cap(2), cap(3), cap(4)), (11, 11, 11, 10, 10));
}

#[test]
fn the_fallback_is_the_same_at_every_difficulty() {
    let at = |d: i64| {
        let arg = format!("adversary.difficulty={d}");
        let (rules, _) = lab_with(&[], &["board.liberate_at=5", &arg]);
        board::liberate_at(&rules.world())
    };
    for d in 1..=4 {
        assert_eq!(at(d), at(0));
    }
}
