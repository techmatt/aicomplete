//! 024: freeing a corrupted city.

use super::*;
use crate::content::Op;
use crate::modules::adversary;
use crate::modules::common::{ambassador, presence, ADV_AMBASSADOR};

/// A corrupted city with seat 0's presence on it, and an adjacent land hex.
fn city(rules: &Rules, g: &mut GameState) -> (usize, usize) {
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(g, h, ADV_CUBE, 1);
    put(g, h, &presence(0), 1);
    let n = rules.map.hexes[h].neighbours[0];
    (h, n)
}

/// Answers a hex choice, if it was asked (a single option is taken for you).
pub(super) fn pick(rules: &Rules, g: &mut GameState, h: usize) {
    if g.pending.is_some() {
        answer_hex(rules, g, h);
    }
}

/// Plays `card` for `seat` out of its deck, funded to afford it.
fn play_card(rules: &Rules, g: &mut GameState, seat: u8, card: &str) {
    for tier in [1, 2] {
        if let Some(p) = g.piles.get_mut(format!("cards.deck.{tier}").as_str()) {
            Arc::make_mut(p).retain(|c| c != card);
        }
    }
    Arc::make_mut(g.piles.get_mut(format!("cards.hand.{seat}").as_str()).unwrap()).push(card.into());
    let s = seat as usize;
    set_res(g, s, Resource::Credits, 30);
    set_res(g, s, Resource::Insight, 5);
    set_res(g, s, Resource::Antimatter, 2);
    rules
        .apply_mut(g, &Action::Play { seat, card: card.into() }, None)
        .unwrap();
}

#[test]
fn only_a_tier_2_ultimate_frees_a_city_outright() {
    for c in &crate::content().cards {
        let cities = c.effects.iter().any(|e| matches!(e, Op::Cleanse { cities: true, .. }));
        assert_eq!(cities, c.id == "great_liberation", "{}", c.id);
    }
    // A tier-1 cleanse is not offered for a city, and does nothing to one.
    let (rules, mut g) = lab_actions();
    let (h, _) = city(&rules, &mut g);
    let others: Vec<usize> = (0..g.hexes.len()).filter(|&x| x != h).collect();
    for x in others {
        put(&mut g, x, ADV_CUBE, 0);
    }
    Arc::make_mut(g.piles.get_mut("cards.hand.0").unwrap()).push("back_channel_talks".into());
    set_res(&mut g, 0, Resource::Credits, 30);
    set_res(&mut g, 0, Resource::Insight, 5);
    let plays = rules.legal_actions(&g);
    assert!(!plays.iter().any(|a| matches!(a, Action::Play { card, .. } if card == "back_channel_talks")));
    // The ultimate frees it.
    let mut k = g.clone();
    // It needs a Diplomacy tag in play.
    Arc::make_mut(k.piles.entry(crate::modules::cards::played(0)).or_default()).push("consular_pouch".into());
    play_card(&rules, &mut k, 0, "great_liberation");
    pick(&rules, &mut k, h);
    answer_all(&rules, &mut k);
    assert_eq!(k.tokens(h, ADV_CUBE), 0);
}

#[test]
fn clearing_a_city_takes_the_ones_the_player_picks() {
    let (rules, mut g) = lab_actions();
    let (h, n) = city(&rules, &mut g);
    put(&mut g, h, ADV_AMBASSADOR, 2);
    put(&mut g, n, ADV_AMBASSADOR, 2);
    play_card(&rules, &mut g, 0, "amnesty_decree");
    pick(&rules, &mut g, h);
    answer_all(&rules, &mut g);
    let left = g.tokens(n, ADV_AMBASSADOR) + g.tokens(h, ADV_AMBASSADOR);
    assert_eq!(left, 2, "the card takes 2 of the 4, on or next to the city");
    let mut k = g.clone();
    put(&mut k, n, ADV_AMBASSADOR, 3);
    // It needs a Diplomacy tag in play.
    Arc::make_mut(k.piles.entry(crate::modules::cards::played(0)).or_default()).push("consular_pouch".into());
    play_card(&rules, &mut k, 0, "liberation_accords");
    pick(&rules, &mut k, h);
    answer_all(&rules, &mut k);
    assert_eq!((k.tokens(n, ADV_AMBASSADOR), k.tokens(h, ADV_AMBASSADOR)), (0, 0));
}

/// A corruption doesn't free the city at once: a player ambassador waiting
/// there is lost with it (review of 024).
#[test]
fn a_corruption_takes_a_waiting_ambassador_with_it() {
    let (rules, mut g) = lab(&[]);
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(&mut g, h, &ambassador(0), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    set_res(&mut g, 0, Resource::Credits, 10);
    rules.apply_mut(&mut g, &std_act(0, "energy"), None).unwrap();
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
    assert_eq!(g.tokens(h, &ambassador(0)), 0);
}
