use super::*;
use crate::content::{content, content_hash, overlaid};

const CARD: &str = "telemetry_archive";

/// Takes the middle legal action: never only passes, so costs matter.
struct Middle;

impl crate::Agent for Middle {
    fn choose(&mut self, _: &Rules, _: &GameState, legal: &[Action]) -> usize {
        legal.len() / 2
    }
}

fn card_cost(r: &Rules) -> i64 {
    let c = r.world().content;
    c.cards[c.card_index(CARD).unwrap()].cost
}

#[test]
fn an_overlay_changes_only_its_own_rules_and_names_its_content() {
    let plain = Rules::new(&baseline()).unwrap();
    let cheap = Rules::new(&baseline_with(&["cards.telemetry_archive.cost=0"])).unwrap();
    let dear = Rules::new(&baseline_with(&["cards.telemetry_archive.cost=9"])).unwrap();
    assert_eq!(card_cost(&cheap), 0);
    assert_eq!(card_cost(&dear), 9);
    assert_eq!(card_cost(&plain), content().cards[content().card_index(CARD).unwrap()].cost);
    assert_eq!(plain.content_hash, content_hash());
    assert_ne!(cheap.content_hash, plain.content_hash);
    assert_ne!(cheap.content_hash, dear.content_hash);
    assert_ne!(cheap.config_hash, plain.config_hash);
}

/// 229: a last step `+` appends to a list, and two appends to one list both stand.
#[test]
fn a_plus_step_appends_to_a_list() {
    let passives = |r: &Rules| {
        let c = r.world().content;
        c.cards[c.card_index(CARD).unwrap()].passives.len()
    };
    let plain = Rules::new(&baseline()).unwrap();
    let one = r#"cards.telemetry_archive.passives.+={"op":"slots","amount":1}"#;
    let two = Rules::new(&baseline_with(&[one, one])).unwrap();
    assert_eq!(passives(&two), passives(&plain) + 2);
    assert!(Rules::new(&baseline_with(&["cards.telemetry_archive.cost.+=1"])).is_err(), "+ on a number");
}

#[test]
fn an_empty_overlay_leaves_the_config_hash_alone() {
    let mut c = baseline();
    c.apply_override("cards.telemetry_archive.cost=9").unwrap();
    c.apply_override("cards.telemetry_archive.cost=").unwrap();
    assert!(c.content.is_empty());
    assert_eq!(c.hash(), baseline().hash());
    assert!(!c.to_json_pretty().contains("\"content\""));
}

#[test]
fn the_same_key_replaces_its_entry() {
    let c = baseline_with(&["cards.a_b.cost=1", "cards.a_b.cost=2"]);
    assert_eq!(c.content, ["cards.a_b.cost=2"]);
}

#[test]
fn a_parameter_is_still_a_parameter() {
    let c = baseline_with(&["cards.keep_tier2=3"]);
    assert!(c.content.is_empty());
}

/// A record names the content it ran, and replays with no warning.
#[test]
fn a_record_of_an_overlaid_game_replays_from_itself() {
    let r = Rules::new(&baseline_with(&["cards.*.cost=1", "ais.*.economy_track.4.cost=2"])).unwrap();
    let mut agent = Middle;
    let (g, rec) = crate::play(&r, 7, &mut agent, 300);
    let saved = crate::SavedGame::new(&r, 7, rec.actions.clone(), &g, "overlay");
    assert_eq!(saved.content_hash, r.content_hash);
    let back = crate::SavedGame::from_json(&saved.to_text()).unwrap();
    let (again, warnings) = back.replay().unwrap();
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(again.fingerprint(), g.fingerprint());
}

#[test]
fn overlays_change_how_a_game_plays() {
    let play = |sets: &[&str]| {
        let r = Rules::new(&baseline_with(sets)).unwrap();
        let mut agent = Middle;
        crate::play(&r, 7, &mut agent, 300).1.options_total
    };
    assert_ne!(play(&[]), play(&["cards.*.cost=0"]));
}

#[test]
fn deep_paths_reach_ops_and_tracks() {
    let (c, _) = overlaid(&[
        "cards.orbital_embassy.passives.0.then.0.count=3".to_string(),
        "ais.*.economy_track.0.income=[[\"credits\",7]]".to_string(),
        "cards.orbital_embassy.effects=[]".to_string(),
    ])
    .unwrap();
    let card = &c.cards[c.card_index("orbital_embassy").unwrap()];
    assert!(card.effects.is_empty());
    assert!(serde_json::to_string(&card.passives).unwrap().contains("\"count\":3"));
    assert!(c.ais.iter().all(|a| a.economy_track[0].income == [(Resource::Credits, 7)]));
}

/// The text follows the numbers: a face is generated from the overlaid ops.
#[test]
fn card_text_follows_the_overlay() {
    let (c, _) = overlaid(&["cards.orbital_embassy.passives.0.then.0.count=9".to_string()]).unwrap();
    let new = &c.cards[c.card_index("orbital_embassy").unwrap()].text;
    let old = &content().cards[content().card_index("orbital_embassy").unwrap()].text;
    assert_ne!(new, old);
}

#[test]
fn bad_overlays_are_errors_that_name_the_entry() {
    for (entry, why) in [
        ("cards.no_such_card.cost=1", "no id"),
        ("cards.telemetry_archive.costs=1", "unknown field"),
        ("cards.telemetry_archive.id=x", "id can't"),
        ("cards.orbital_embassy.effects.99.amount=1", "no \"99\""),
        ("ais.*.economy_track.99=[]", "no list entry"),
        ("cards.telemetry_archive.cost=lots", "invalid type"),
        ("threats.*.tier=9", "tier 1-2"),
        // 146: no tier 3.
        ("threats.eviction.tier=3", "tier 1-2"),
        ("threats.*.counters=[]", "a counter menu"),
        // 146: Supply Raid's harm requisitions chips, so a chips counter shares it.
        (r#"threats.supply_raid.counters=[[{"resource":"chips","amount":3}],[{"resource":"credits","amount":12}]]"#, "share"),
        // 146: Dark Cities destroys the nearest player's node, so a nodes counter shares it.
        (r#"threats.dark_cities.counters=[[{"nodes_near":1}],[{"resource":"credits","amount":12}]]"#, "share"),
    ] {
        let e = match Rules::new(&baseline_with(&[entry])) {
            Err(e) => e,
            Ok(_) => panic!("{entry} was accepted"),
        };
        assert!(e.contains(why), "{entry}: {e}");
    }
}

/// 146 (Matt's threat card standard): two counter options and two consequences
/// at most, no special pieces, and no node harm on each player; each refusal
/// names the threat.
#[test]
fn a_threat_off_the_card_standard_is_refused_by_name() {
    for (entry, id, why) in [
        (
            r#"threats.eviction.counters=[[{"resource":"chips","amount":6}],[{"resource":"energy","amount":3}],[{"resource":"credits","amount":15}]]"#,
            "eviction",
            "3 counter options",
        ),
        (
            r#"threats.eviction.effects=[{"op":"compute","amount":1},{"op":"compute","amount":2},{"op":"compute","amount":3}]"#,
            "eviction",
            "3 consequences",
        ),
        (
            r#"threats.orbital_strike.effects=[{"op":"troops","count":2,"at":"location","stand":true}]"#,
            "orbital_strike",
            "a troop that stands",
        ),
        (r#"threats.dark_cities.effects=[{"op":"each_destroy_node","count":1}]"#, "dark_cities", "each player destroys"),
        // 184: a troop line names a type the threat prints, and which of two.
        (
            r#"threats.iron_curtain.effects=[{"op":"troops","count":2,"at":"location"},{"op":"pull_unity","amount":2}]"#,
            "iron_curtain",
            "names which token",
        ),
        (
            r#"threats.supply_raid.effects=[{"op":"troops","count":1,"at":"location","location":"base"}]"#,
            "supply_raid",
            "doesn't print",
        ),
    ] {
        let e = match Rules::new(&baseline_with(&[entry])) {
            Err(e) => e,
            Ok(_) => panic!("{entry} was accepted"),
        };
        assert!(e.contains(&format!("threat {id:?}")) && e.contains(why), "{entry}: {e}");
    }
}
