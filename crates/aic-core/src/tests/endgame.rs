//! 027: the two decks as clocks, the node loss, arrivals, the hold, the
//! piece ladder and what destroys nodes.

use super::*;
use crate::content::Op;
use crate::modules::common::{hex_opt, is_ocean, MARKER};
use crate::modules::{adversary, alignment, board};
use crate::state::Pick;

/// Liberate's city numbers (the lab pins them off).
const HOLD: [&str; 1] = ["board.liberate_at=5"];

fn empty_deck(g: &mut GameState, side: &str) {
    g.piles.insert(format!("alignment.deck.{side}").into(), Vec::new().into());
}

#[test]
fn a_discord_tip_with_an_empty_deck_loses_and_nothing_reshuffles() {
    let (rules, mut g) = action_only(&[]);
    empty_deck(&mut g, "discord");
    set_pile(&mut g, "alignment.resolved.discord", &["blight"]);
    fill_bucket(&rules, &mut g, Side::Discord);
    any_action(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "discord_deck"));
    assert_eq!(g.pile("alignment.resolved.discord"), ["blight"]);
}

#[test]
fn drawing_the_last_unity_card_wins_nothing_and_an_empty_unity_deck_reshuffles() {
    // 156: no Unity-deck win. The last card resolves; the next tip shuffles the
    // resolved pile back in and draws from it.
    let (rules, mut g) = action_only(&[]);
    empty_deck(&mut g, "unity");
    set_pile(&mut g, "alignment.deck.unity", &["volunteer_corps"]);
    set_pile(&mut g, "alignment.resolved.unity", &[]);
    fill_bucket(&rules, &mut g, Side::Unity);
    any_action(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert!(g.outcome.as_ref().is_none_or(|o| o.reason != "unity_deck"), "no Unity-deck win");
    assert_eq!(g.pile("alignment.resolved.unity"), ["volunteer_corps"]);
    assert!(g.pile("alignment.deck.unity").is_empty());
    set_pile(&mut g, "alignment.resolved.unity", &["volunteer_corps", "think_tank"]);
    fill_bucket(&rules, &mut g, Side::Unity);
    any_action(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.pile("alignment.resolved.unity").len() + g.pile("alignment.deck.unity").len(), 2, "reshuffled, one drawn");
    assert_eq!(g.pile("alignment.resolved.unity").len(), 1);
}

#[test]
fn a_player_with_no_node_loses_the_game_for_everyone() {
    let (rules, mut g) = action_only(&[]);
    for h in 0..g.hexes.len() {
        put(&mut g, h, node(1), 0);
    }
    any_action(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "node"));
}

#[test]
fn the_supply_is_a_cap_not_a_loss() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let station = rules.map.numbered(0, piece(0).stations[0]);
    put(&mut g, station, STATION, 1);
    g.vars.insert(SUPPLY.into(), 1);
    for c in 0..rules.map.countries.len() {
        g.vars.insert(crate::modules::adversary::pool_key(c).into(), 0);
    }
    g.vars.insert(crate::modules::adversary::pool_key(0).into(), 2);
    set_pile(&mut g, NEXT, &["mil_s1_long_patrols"]);
    run_answered(&rules, &mut g);
    // 151: Long Patrols rallies board 0's pool of 2 to its one station; one lands.
    assert_eq!(total(&g, ADV_AMBASSADOR), 1);
    assert_eq!(g.tokens(station, ADV_AMBASSADOR), 1);
    assert_eq!(g.var(SUPPLY), 0);
    assert!(!g.is_over());
}

#[test]
fn every_corrupted_city_is_no_loss() {
    let (rules, mut g) = lab(&[]);
    corrupt_cities_except(&rules.map, &mut g, &[]);
    any_action(&rules, &mut g);
    assert!(!g.is_over());
}

/// 137: the Liberate action frees an adversary city for `liberate_at`
/// ambassadors plus 1 for each agent on it (those cancel); agents next to it
/// don't count. r75 (Matt's ruling 2): the seat's own ambassadors on the city pay
/// first, its supply the rest, never another seat's; the city becomes the
/// seat's (its cube goes on) and no agent lands on it.
#[test]
fn liberating_a_city_pays_from_your_own_ambassadors_and_makes_it_yours() {
    let (rules, mut g) = lab_with(&[], &HOLD);
    let w = rules.world();
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let n = rules.map.hexes[h].neighbors[0];
    put(&mut g, h, ADV_CUBE, 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    put(&mut g, n, ADV_AMBASSADOR, 2);
    put(&mut g, n, node(0), 1);
    assert_eq!(board::liberate_cost(&w, &g, h), Some(6));
    // Another seat's ambassadors on the city pay nothing of seat 0's bill.
    put(&mut g, h, ambassador(1), 2);
    assert_eq!(board::site_supply_cost(&w, &g, 0, h, true), Some(6));
    set_res(&mut g, 0, Resource::Ambassadors, 3);
    assert!(!rules.legal_actions(&g).contains(&std_act(0, "liberate")), "3 of 6");
    // Seat 0's own 2 on it pay first: 4 from the supply.
    put(&mut g, h, ambassador(0), 2);
    assert_eq!(board::site_supply_cost(&w, &g, 0, h, true), Some(4));
    set_res(&mut g, 0, Resource::Ambassadors, 4);
    let unity = g.var("alignment.unity") + alignment::unity_pending(&g);
    act(&rules, &mut g, std_act(0, "liberate"));
    assert_eq!((g.tokens(h, ADV_CUBE), g.tokens(h, cube(0)), g.resource(0, Resource::Ambassadors)), (0, 1, 0));
    assert_eq!((g.tokens(h, ambassador(0)), g.tokens(h, ambassador(1))), (0, 2));
    // No agent lands on it; the ones next to it stay.
    assert_eq!((g.tokens(h, ADV_AMBASSADOR), g.tokens(n, ADV_AMBASSADOR)), (0, 2));
    assert_eq!(g.var("alignment.unity") + alignment::unity_pending(&g) - unity, w.int("alignment.free_city"));
    assert_eq!(g.stats.counters["board.liberated.city"], 1);
}

/// 137: Liberate needs the city in reach (a node on or next to it), and
/// player ambassadors standing on it liberate nothing (the fallback is gone).
#[test]
fn liberating_needs_a_node_on_or_next_to_the_city() {
    let (rules, mut g) = lab_with(&[], &HOLD);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, ADV_CUBE, 1);
    put(&mut g, h, ambassador(0), 5);
    set_res(&mut g, 0, Resource::Ambassadors, 5);
    assert!(!rules.legal_actions(&g).contains(&std_act(0, "liberate")), "no nodes near it");
    put(&mut g, rules.map.hexes[h].neighbors[0], node(0), 1);
    any_action(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 1, "standing ambassadors don't liberate it");
    act(&rules, &mut g, std_act(0, "liberate"));
    assert_eq!(g.tokens(h, ADV_CUBE), 0);
}

/// 175: no stage protects a last node: corruption takes it in stage I.
#[test]
fn stage_one_takes_a_last_node_too() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, node(0), 1);
    g.vars.insert(STAGE.into(), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h)).unwrap();
    assert_eq!(g.tokens(h, node(0)), 0, "stage I: it goes");
    assert_eq!(g.stats.counters["board.last_node_lost"], 1);
}

/// 139 (Offensive): a Discord card deals a threat onto the board's most
/// threatened node hex instead of destroying the nodes there.
#[test]
fn a_discord_card_deals_a_threat_onto_the_most_threatened_node_hex() {
    use crate::modules::{effects, threats};
    let (rules, mut g) = lab(&[]);
    let (a, b) = (rules.map.numbered(0, piece(0).node), rules.map.numbered(1, piece(1).node));
    put(&mut g, a, node(0), 1);
    put(&mut g, b, node(1), 1);
    put(&mut g, b, ADV_AMBASSADOR, 2);
    let before = threats::active(&g).len();
    with_cx(&rules, |cx| {
        effects::run(&mut g, cx, &[Op::ThreatAtNode { tier: 2 }], &mut effects::Ctx::team().named("discord"))
    })
    .unwrap();
    assert_eq!((g.tokens(a, node(0)), g.tokens(b, node(1))), (1, 1), "no node destroyed");
    let active = threats::active(&g);
    assert_eq!(active.len(), before + 1);
    let (t, country) = *active.last().unwrap();
    assert_eq!(country, 1);
    assert_eq!(threats::tokens(&g, &t).first(), Some(&b));
    assert_eq!(rules.world().content.threat(&t).tier, 2);
    assert_eq!(g.stats.counters["threats.dealt_at_node"], 1);
}

/// 175: a choice threat may take a last node: unable to pay, the player loses it.
#[test]
fn a_choice_threat_takes_a_last_node_too() {
    use crate::modules::threats;
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).node);
    put(&mut g, h, node(0), 1);
    set_res(&mut g, 0, Resource::Credits, 2);
    with_cx(&rules, |cx| {
        let hexes = crate::modules::common::scope_hexes(&rules.world(), Some(0));
        threats::lose_node_or(&mut g, cx, &[0], &hexes, Resource::Credits, 6, "threat1", None)
    })
    .unwrap();
    assert_eq!(g.tokens(h, node(0)), 0);
}

// 028: rules revision 17.

/// Lands `s`'s node on `hexes` and counts them on the track.
fn node_on_track(g: &mut GameState, s: usize, hexes: &[usize]) {
    for &h in hexes {
        put(g, h, node(s), 1);
    }
    g.vars.insert(board::track_key(s).into(), hexes.len() as i64);
}

#[test]
fn each_player_destroys_a_node_of_their_choice_last_included() {
    use crate::modules::common::hex_name;
    let (rules, mut g) = lab(&[]);
    let (a, b) = (rules.map.numbered(0, piece(0).node), rules.map.numbered(0, piece(0).cities[0]));
    let last = rules.map.numbered(1, piece(1).node);
    put(&mut g, a, node(0), 1);
    put(&mut g, b, node(0), 1);
    put(&mut g, last, node(1), 1);
    g.vars.insert(STAGE.into(), 1);
    let hit = |g: &mut GameState, cx: &mut module::Cx, s: usize, h: usize| {
        let hexes = [h, b];
        board::destroy_own_node(g, cx, s, &hexes, board::Cause::Effect("discord"), None).unwrap();
    };
    // The player picks which: here the second of seat 0's two.
    let pick = crate::state::Pick { option: hex_name(&rules.world(), b), target: Some(rules.map.hexes[b].hex) };
    with_cx_picks(&rules, vec![pick], |cx| hit(&mut g, cx, 0, a));
    assert_eq!((g.tokens(a, node(0)), g.tokens(b, node(0))), (1, 0));
    // 175: a last node goes in stage I too.
    with_cx(&rules, |cx| hit(&mut g, cx, 1, last));
    assert_eq!(g.tokens(last, node(1)), 0, "stage I: it goes");
}

/// 136: every printed Unity card is dealt (156: into one deck).
#[test]
fn a_game_deals_every_printed_unity_card() {
    let (_, g) = action_only(&[]);
    let printed = crate::content().alignment.iter().filter(|c| c.side == Side::Unity).count();
    assert_eq!(g.pile("alignment.deck.unity").len(), printed);
    assert_eq!(crate::modules::alignment::cards_left(&g, Side::Unity), printed);
}

#[test]
fn removing_a_botnet_clears_a_hack_on_its_hex() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, BOTNET, 1);
    g.vars.insert(adversary::cut_key(h).into(), 1);
    with_cx(&rules, |cx| adversary::remove_botnet(&mut g, cx, h));
    assert_eq!(g.var(&adversary::cut_key(h)), 0);
}

/// A Discord tip that leaves a seat with no nodes loses before a Unity tip
/// in the same check can draw the last card.
#[test]
fn a_lost_last_node_comes_before_a_unity_tip_in_the_same_check() {
    let (rules, mut g) = action_only(&[]);
    for h in 0..g.hexes.len() {
        put(&mut g, h, node(1), 0);
    }
    empty_deck(&mut g, "unity");
    set_pile(&mut g, "alignment.deck.unity.1", &["civil_defense"]);
    fill_bucket(&rules, &mut g, Side::Discord);
    fill_bucket(&rules, &mut g, Side::Unity);
    any_action(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "node"));
}

// ---------- 034: node leaves the game, the marker ----------

#[test]
fn a_city_that_corrupts_destroys_node_on_it_and_leaves_no_marker() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    let far = rules.map.numbered(1, piece(1).cities[0]);
    put(&mut g, h, node(0), 1);
    put(&mut g, far, node(0), 1);
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h)).unwrap();
    assert_eq!((g.tokens(h, node(0)), g.tokens(h, MARKER)), (0, 0), "it has corruption already");
}

#[test]
fn a_lost_node_leaves_the_game_and_its_income_stays() {
    use crate::modules::ai;
    let (rules, mut g) = lab(&[]);
    let (a, b) = (rules.map.numbered(0, piece(0).node), rules.map.numbered(0, piece(0).cities[0]));
    let c = rules.map.hexes[a].neighbors[0];
    node_on_track(&mut g, 0, &[a, b, c]);
    let w = rules.world();
    let income = ai::track_income(&w, &g, 0);
    with_cx(&rules, |cx| board::lose_node(&mut g, cx, c, 0, "test"));
    assert_eq!(board::track_taken(&g, 0), 3, "it does not go back on the track");
    assert_eq!(ai::track_income(&w, &g, 0), income);
    assert_eq!(g.tokens(c, MARKER), 1);
}

#[test]
fn a_marker_adds_nothing_where_there_is_corruption_and_blocks_the_hex() {
    let (rules, mut g) = lab(&[]);
    let a = rules.map.numbered(0, piece(0).node);
    let n = rules.map.hexes[a].neighbors[0];
    put(&mut g, a, node(0), 1);
    put(&mut g, n, node(1), 1);
    put(&mut g, n, BOTNET, 1);
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| board::lose_node(&mut g, cx, n, 1, "test"));
    assert_eq!(g.tokens(n, MARKER), 0, "a botnet is corruption");
    put(&mut g, n, MARKER, 1);
    let w = rules.world();
    assert!(!board::node_sites(&w, &g, 0).contains(&n));
    with_cx(&rules, |cx| board::lose_node(&mut g, cx, a, 0, "test"));
    assert_eq!(g.tokens(a, MARKER), 1);
}

/// r80 (ruling #42): a marker no longer counts as an agent toward a city's corruption (034-r79 counted it); a
/// city that does corrupt takes the cube, its one mark, and the marker comes off.
#[test]
fn a_marker_no_longer_counts_toward_corrupting_a_city() {
    let (rules, mut g0) = lab_with(&["adversary.natural"], &["adversary.corrupt_at=2"]);
    let h = rules.map.numbered(0, piece(0).cities[1]);
    put(&mut g0, h, MARKER, 1);
    put(&mut g0, h, ADV_AMBASSADOR, 1);
    let mut g = g0.clone();
    run(&rules, &mut g);
    assert_eq!((g.tokens(h, ADV_CUBE), g.tokens(h, MARKER)), (0, 1), "one agent and a marker: short of 2");
    let mut g = g0.clone();
    put(&mut g, h, ADV_AMBASSADOR, 2);
    run(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
    assert_eq!(g.tokens(h, MARKER), 0, "the city's corruption replaces it");
}

#[test]
fn up_to_two_markers_can_stop_after_one() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let c = rules.map.numbered(0, piece(0).node);
    let n: Vec<usize> = w.map.hexes[c].neighbors.iter().copied().filter(|&h| !is_ocean(&w, h)).take(2).collect();
    put(&mut g, c, node(0), 1);
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
    let a = rules.map.numbered(0, piece(0).node);
    let marked = w.map.hexes[a].neighbors.iter().copied().find(|&h| !is_ocean(&w, h)).unwrap();
    put(&mut g, marked, MARKER, 1);
    assert!(!board::node_sites(&w, &g, 0).contains(&marked), "no nodes on a marker");
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Credits, 5);
    let credits = 5 - w.int("board.clear_cost");
    let unity = g.var(alignment::track(Side::Unity));
    rules.apply_mut(&mut g, &std_act(0, "clear"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(marked, MARKER), 0);
    assert_eq!(g.resource(0, Resource::Credits), credits + w.int("board.clear_credits"));
    assert_eq!(g.var(alignment::track(Side::Unity)), unity + w.int("board.clear_unity"));
}

/// 034's d4 fix: every printed step of the Discord deck acts before it ends,
/// at each difficulty, with the deck's tips counted as the game counts them.
#[test]
fn discord_tier_two_is_reached_at_every_difficulty() {
    // 156: tier II is reached before the Discord deck ends at every level; 157 (r68):
    // tiers count the tip itself, so with 6 cards the 6th tip resolves tier III too.
    for d in 0..=4 {
        let arg = format!("adversary.difficulty={d}");
        let (rules, g) = lab_with(&[], &[&arg]);
        let w = rules.world();
        let start = g.var("alignment.discord_tips").max(0);
        let cards = w.content.alignment.iter().filter(|c| c.side == crate::content::Side::Discord).count() as i64;
        let cut = if d >= 4 { w.int("adversary.discord_removed") } else { 0 };
        // The number of the last tip that finds a card.
        let last = start + cards - cut;
        let tiers = w.list("alignment.tier_at");
        assert!(tiers[0] <= last, "tier II at {} never acts at d{d} (last tip {last})", tiers[0]);
        assert!(tiers[1] <= last, "tier III at {} never acts at d{d} (last tip {last})", tiers[1]);
    }
}

#[test]
fn the_round_cap_is_the_deck_and_higher_difficulty_shortens_it() {
    let cap = |d: i64| {
        let arg = format!("adversary.difficulty={d}");
        let (_, g) = lab_with(&[], &[&arg]);
        g.pile(DECK).len() + g.pile(NEXT).len()
    };
    // 150: d3 removes two stage-II cards (`stage2_removed` 2). 196: d0 adds two (`stage_extra` 2 at d0), stage I
    // first: 5 stage-I cards, then 5 stage-II.
    assert_eq!((cap(0), cap(1), cap(2), cap(3), cap(4)), (13, 11, 11, 9, 9));
    let (_, g) = lab_with(&[], &["adversary.difficulty=0"]);
    let stage = |id: &str| if id.starts_with("mil_s1_") { 1 } else if id.starts_with("mil_s2_") { 2 } else { 3 };
    let dealt: Vec<u8> = g.pile(NEXT).iter().chain(g.pile(DECK).iter()).map(|id| stage(id)).collect();
    assert_eq!(dealt.iter().filter(|&&s| s == 1).count(), 5, "{dealt:?}");
    assert_eq!(dealt.iter().filter(|&&s| s == 2).count(), 5, "{dealt:?}");
}

#[test]
fn liberating_a_city_costs_the_same_at_every_difficulty() {
    let at = |d: i64| {
        let arg = format!("adversary.difficulty={d}");
        let (rules, _) = lab_with(&[], &["board.liberate_at=5", &arg]);
        board::liberate_at(&rules.world())
    };
    for d in 1..=4 {
        assert_eq!(at(d), at(0));
    }
}
