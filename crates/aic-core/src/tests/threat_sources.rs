//! Where a threat lands (135): the next card and its location tokens.

use super::*;
use crate::content::ThreatTag;
use crate::vocab::ThreatSource;
use crate::modules::threats;

fn pull(rules: &Rules, g: &mut GameState, c: usize) {
    with_cx(rules, |cx| threats::deal_tier(g, cx, 1, c)).unwrap();
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
    pull(&rules, &mut g, 0);
    assert_eq!(threats::tokens(&g, &Key::new("eviction")), [a]);
    // An agent on b: b, though a is lower; corrupted cities never.
    put(&mut g, b, ADV_AMBASSADOR, 1);
    pull(&rules, &mut g, 0);
    assert_eq!(threats::tokens(&g, &Key::new("defection")), [b]);
}

/// r81 (friction #47): one tag vocabulary. Every tag a threat, a project or a project slot prints is a
/// player card's tag (Universal aside), so a threat's tag is the tag its cards sacrifice, with no table between.
#[test]
fn every_threat_tag_is_a_card_tag() {
    let content = crate::content();
    let card_tag = |t: ThreatTag| t == ThreatTag::Universal || crate::vocab::TAGS.iter().any(|c| c.name() == t.name());
    let mut seen = 0;
    for t in content.threats.iter().flat_map(|t| t.tags.iter()).chain(content.projects.iter().flat_map(|p| p.tags.iter())) {
        assert!(card_tag(*t), "{}", t.name());
        seen += 1;
    }
    for t in content.adversary_board.project_slots.iter().filter_map(|s| s.tag) {
        assert!(card_tag(t), "{}", t.name());
    }
    assert!(seen > 0);
    for s in [ThreatSource::Diplomacy, ThreatSource::Cyber, ThreatSource::Industry, ThreatSource::Security] {
        assert!(crate::vocab::TAGS.iter().any(|c| c.name() == s.name()), "{}", s.name());
    }
}

/// 292: threat dealing never rejects a card. The top card is dealt, whatever its tags; with no site of its types
/// left in the country its token goes on the country's lowest-numbered city not the adversary's, off its kind, so
/// its located lines do nothing.
#[test]
fn the_next_card_is_dealt_and_a_siteless_one_falls_back_to_a_city() {
    let (rules, mut g) = threat_lab();
    let m = &rules.map;
    for &h in &m.countries[0].hexes {
        put(&mut g, h, STATION, 0);
    }
    set_pile(&mut g, "threats.deck.1", &["sabotage", "embargo"]);
    set_pile(&mut g, "threats.discard.1", &[]);
    pull(&rules, &mut g, 0);
    assert_eq!(g.pile("threats.on.0"), ["sabotage"], "a station threat, dealt though the country has no station");
    assert_eq!(g.pile("threats.deck.1"), ["embargo"]);
    assert!(g.pile("threats.discard.1").is_empty(), "nothing is turned and discarded");
    let city = cities(m)
        .into_iter()
        .filter(|&h| m.hexes[h].country == 0 && g.tokens(h, ADV_CUBE) == 0)
        .min_by_key(|&h| m.hexes[h].number)
        .expect("a city not the adversary's");
    assert_eq!(threats::tokens(&g, &Key::new("sabotage")), [city]);
    assert!(threats::here(&g, "sabotage").is_none(), "off its kind: no \"here\"");
    assert_eq!(g.stats.counters.get("threats.dealt_fallback").copied(), Some(1));
    // Every card turned up is dealt (a read's "rejected a round" is turned less dealt).
    assert_eq!(g.stats.counters.get("threats.turned"), g.stats.counters.get("threats.dealt"));
    // A city threat falls back only when every city is the adversary's: there too its located lines do nothing.
    for h in cities(m).into_iter().filter(|&h| m.hexes[h].country == 0) {
        put(&mut g, h, ADV_CUBE, 1);
    }
    set_pile(&mut g, "threats.deck.1", &["eviction"]);
    pull(&rules, &mut g, 0);
    assert_eq!(threats::tokens(&g, &Key::new("eviction")).len(), 1, "its token is on a city all the same");
    assert!(threats::here(&g, "eviction").is_none(), "the fallback city is no \"here\" for a city threat either");
    assert_eq!(g.stats.counters.get("threats.dealt_fallback").copied(), Some(2));
}

/// 292 (Matt, 2026-11-19): the threat decks are built once, at setup, by the adversary board's lean: every threat
/// of the tier carrying its tag (Universal counts) and as many of the others; nothing is filtered during a round.
#[test]
fn setup_builds_each_threat_deck_by_the_boards_lean() {
    let rules = Rules::new(&baseline_with(&[])).unwrap();
    let content = crate::content();
    let tag = content.adversary_board.threat_deck.as_ref().expect("the Militarist's lean").tag;
    assert_eq!(tag, ThreatTag::Security);
    let mut decks = std::collections::BTreeSet::new();
    for seed in 0..4 {
        let g = rules.new_game(seed);
        for tier in [1u8, 2] {
            let mut in_game: Vec<String> = g.pile(&format!("threats.deck.{tier}")).iter().map(|k| k.to_string()).collect();
            in_game.extend(g.pile(&format!("threats.discard.{tier}")).iter().map(|k| k.to_string()));
            in_game.extend(threats::active(&g).iter().map(|(t, _)| t.to_string()).filter(|t| content.threat(t).tier == tier));
            let leans = |id: &str| content.threat(id).tags.iter().any(|&x| x == tag || x == ThreatTag::Universal);
            let lean: Vec<&str> = content.threats.iter().filter(|t| t.tier == tier && leans(&t.id)).map(|t| t.id.as_str()).collect();
            assert!(lean.iter().all(|id| in_game.iter().any(|x| x == id)), "tier {tier}: every lean card is in");
            assert_eq!(in_game.len(), 2 * lean.len(), "tier {tier}: as many others as lean cards");
            in_game.sort();
            decks.insert((tier, in_game));
        }
    }
    assert!(decks.len() > 2, "the others differ from game to game");
}
