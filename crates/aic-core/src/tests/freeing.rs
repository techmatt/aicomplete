//! 024: freeing a corrupted city.

use super::*;
use crate::content::Op;
use crate::modules::adversary;

/// A corrupted city with seat 0's node on it, and an adjacent land hex.
fn city(rules: &Rules, g: &mut GameState) -> (usize, usize) {
    let h = rules.map.numbered(0, piece(0).cities[0]);
    put(g, h, ADV_CUBE, 1);
    put(g, h, node(0), 1);
    let n = rules.map.hexes[h].neighbors[0];
    (h, n)
}

/// Plays `card` for `seat` out of its deck, funded to afford it.
fn play_card(rules: &Rules, g: &mut GameState, seat: u8, card: &str) {
    out_of_decks(g, card);
    Arc::make_mut(g.piles.entry(crate::modules::cards::hand(seat as usize)).or_default()).push(card.into());
    let s = seat as usize;
    fund(g, s);
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
    // A tier-1 cleanse is not offered for a city, and does nothing to one. The test pins the
    // ultimate's price under what `fund` gives (r44 prints 32).
    // 119: and its need of 4 Diplomacy, r46's print.
    let (rules, mut g) = lab_actions_with(&["cards.great_liberation.cost=21", r#"cards.great_liberation.needs=[["diplomacy",4]]"#]);
    let (h, _) = city(&rules, &mut g);
    let others: Vec<usize> = (0..g.hexes.len()).filter(|&x| x != h).collect();
    for x in others {
        put(&mut g, x, ADV_CUBE, 0);
    }
    Arc::make_mut(g.piles.entry(crate::modules::cards::hand(0)).or_default()).push("back_channel_talks".into());
    set_res(&mut g, 0, Resource::Credits, 30);
    set_res(&mut g, 0, Resource::Insight, 5);
    // r48: nothing to act on never blocks a card, so the factory cleanse plays for its
    // Sway, and leaves the city corrupted.
    let plays = rules.legal_actions(&g);
    assert!(plays.iter().any(|a| matches!(a, Action::Play { card, .. } if card == "back_channel_talks")));
    let mut t = g.clone();
    play_card(&rules, &mut t, 0, "back_channel_talks");
    answer_all(&rules, &mut t);
    assert_eq!(t.tokens(h, ADV_CUBE), 1, "a tier-1 cleanse does nothing to a city");
    // The ultimate frees it.
    let mut k = g.clone();
    // It needs 4 Diplomacy tags in play (076: a chain).
    for c in ["consular_pouch", "peace_dividend", "mutual_aid_pact", "summit_caucus"] {
        Arc::make_mut(k.piles.entry(crate::modules::cards::played(0)).or_default()).push(c.into());
    }
    play_card(&rules, &mut k, 0, "great_liberation");
    pick(&rules, &mut k, h);
    answer_all(&rules, &mut k);
    assert_eq!(k.tokens(h, ADV_CUBE), 0);
}

#[test]
fn clearing_a_city_takes_the_ones_the_player_picks() {
    // 119: both cards pinned to r46's print, under what `fund` pays.
    let (rules, mut g) = lab_actions_with(&[
        "cards.amnesty_decree.cost=11",
        r#"cards.amnesty_decree.effects=[{"op":"clear_city","max":4},{"op":"gain_ambassadors","count":2}]"#,
        "cards.liberation_accords.cost=15",
        r#"cards.liberation_accords.needs=[["diplomacy",1]]"#,
        r#"cards.liberation_accords.effects=[{"op":"remove_ambassadors_country","unity_per":0,"max":6,"reach":true},{"op":"gain_ambassadors","count":4}]"#,
    ]);
    let (h, n) = city(&rules, &mut g);
    put(&mut g, h, ADV_AMBASSADOR, 4);
    put(&mut g, n, ADV_AMBASSADOR, 3);
    play_card(&rules, &mut g, 0, "amnesty_decree");
    pick(&rules, &mut g, h);
    answer_all(&rules, &mut g);
    let left = g.tokens(n, ADV_AMBASSADOR) + g.tokens(h, ADV_AMBASSADOR);
    assert_eq!(left, 3, "the card takes 4 of the 7, on or next to the city");
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
    put(&mut g, h, ambassador(0), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    set_res(&mut g, 0, Resource::Credits, 30);
    rules.apply_mut(&mut g, &std_act(0, "ambassador"), None).unwrap();
    assert_eq!(g.tokens(h, ADV_CUBE), 1);
    assert_eq!(g.tokens(h, ambassador(0)), 0);
}

#[test]
fn freeing_a_city_and_removing_a_botnet_sway_unity() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let city = m.numbered(0, piece(0).cities[0]);
    put(&mut g, city, ADV_CUBE, 1);
    put(&mut g, city, node(0), 1);
    // 137: the Liberate action (the lab's `liberate_at` is 1).
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    rules.apply_mut(&mut g, &std_act(0, "liberate"), None).unwrap();
    pick(&rules, &mut g, city);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(city, ADV_CUBE), 0);
    assert_eq!(g.var("alignment.unity"), 5);
    // No standard action removes a botnet (019); the Decommission Unity card does.
    put(&mut g, city, BOTNET, 1);
    assert!(rules.apply_mut(&mut g.clone(), &std_act(0, "botnet"), None).is_err());
    with_cx(&rules, |cx| {
        crate::modules::effects::run(
            &mut g,
            cx,
            &[content::Op::RemoveBotnets { count: 1 }],
            &mut crate::modules::effects::Ctx::team(),
        )
    })
    .unwrap();
    assert_eq!(g.tokens(city, BOTNET), 0);
    // Freeing a city sways 5 (034; 024: 4), removing a botnet 1 (136: 2); Unity
    // swayed outside an action waits to be placed (136: it may hold Discord off).
    assert_eq!(g.var("alignment.unity") + crate::modules::alignment::unity_pending(&g), 6);
}
