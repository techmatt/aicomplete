//! Where a threat lands (135): the draw rule and its location tokens.

use super::*;
use crate::content::{PullTag, ThreatTag};
use crate::modules::threats;

fn pull(rules: &Rules, g: &mut GameState, tags: &[PullTag], c: usize) {
    with_cx(rules, |cx| threats::deal_tier(g, cx, 1, tags, c)).unwrap();
}

#[test]
fn a_pull_skips_a_misfit_and_a_card_whose_location_the_country_lacks() {
    let (rules, mut g) = threat_lab();
    for &h in &rules.map.countries[0].hexes {
        put(&mut g, h, STATION, 0);
    }
    set_pile(&mut g, "threats.deck.1", &["sabotage", "embargo", "blackout", "backdoor"]);
    set_pile(&mut g, "threats.discard.1", &[]);
    pull(&rules, &mut g, &[PullTag::Is(ThreatTag::Cyber)], 0);
    assert_eq!(g.pile("threats.on.0"), ["blackout"], "sabotage needs a station, embargo isn't cyber");
    let mut gone = g.pile("threats.discard.1").to_vec();
    gone.sort();
    assert_eq!(gone, ["embargo", "sabotage"]);
    assert_eq!(g.pile("threats.deck.1"), ["backdoor"]);
    let at = threats::tokens(&g, &Key::new("blackout"));
    assert!(at.len() == 1 && rules.map.hexes[at[0]].country == 0 && cities(&rules.map).contains(&at[0]));
}

#[test]
fn a_token_lands_on_the_most_threatened_uncorrupted_site() {
    let (rules, mut g) = threat_lab();
    let m = &rules.map;
    let mine: Vec<usize> = cities(m).into_iter().filter(|&h| m.hexes[h].country == 0).collect();
    // Two cities of country 0 not next to each other: the lower, and one it doesn't touch.
    let (a, b) = mine
        .iter()
        .flat_map(|&a| mine.iter().map(move |&b| (a, b)))
        .find(|&(a, b)| a < b && !m.hexes[a].neighbors.contains(&b))
        .expect("two cities apart");
    for &h in &mine {
        put(&mut g, h, ADV_CUBE, i64::from(h != a && h != b));
    }
    for h in 0..g.hexes.len() {
        for kind in [ADV_AMBASSADOR, TROOP, BOTNET] {
            put(&mut g, h, kind, 0);
        }
    }
    set_pile(&mut g, "threats.deck.1", &["eviction", "defection"]);
    // No adversary pieces: the lower city.
    pull(&rules, &mut g, &[PullTag::Any], 0);
    assert_eq!(threats::tokens(&g, &Key::new("eviction")), [a]);
    // An adversary ambassador on b: b, though a is lower; corrupted cities never.
    put(&mut g, b, ADV_AMBASSADOR, 1);
    pull(&rules, &mut g, &[PullTag::Any], 0);
    assert_eq!(threats::tokens(&g, &Key::new("defection")), [b]);
}
