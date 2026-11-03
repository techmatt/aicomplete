use super::*;

fn face_of(id: &str) -> crate::face::Face {
    crate::face::card_face(content().cards.iter().find(|c| c.id == id).expect("a card"))
}

#[test]
fn a_face_opens_with_its_boxes_word() {
    let f = face_of("synthetic_data_forge");
    assert_eq!(f.top_text, "Action: add 1 Dataset here, OR get 1 insight per Dataset here.");
    assert_eq!(f.text, "");
    let f = face_of("orbital_embassy");
    assert_eq!(f.top_text, "Each round, 2 ambassadors and 3 credits.");
    assert_eq!(f.text, "Pay 1 antimatter. Gain 2 ambassadors.");
}

#[test]
fn a_bare_arrow_is_a_free_action() {
    let f = face_of("honeypot_relay");
    assert_eq!(f.top_text, "Action: remove 2 adversary ambassadors from a hex in reach.");
    // 076: an OR Action prints each half; the second line opens with OR.
    let f = face_of("agency_field_station");
    assert_eq!(f.top_text, "Action: add 1 Agent here, OR get 1 credit per Agent here.");
    let json = serde_json::to_string(&f.top).unwrap();
    assert_eq!(json, r#"[["→","+1","agents"],["OR","→","credits","/","agents"]]"#);
}

#[test]
fn income_below_the_art_raises_or_lowers_the_track() {
    let f = face_of("endowed_research_chair");
    assert_eq!(f.top_text, "");
    assert_eq!(f.text, "Lower your credit income by 1. Raise your insight income by 2.");
}

#[test]
fn only_buildings_print_above_the_art() {
    for c in &content().cards {
        let f = crate::face::card_face(c);
        assert!(c.is_building() || f.top.is_empty(), "{} prints above the art", c.id);
        assert!(!c.text.is_empty() && c.text == f.full_text(), "{}", c.id);
    }
}

/// 059: every symbol means what it always means, on every card.
#[test]
fn no_face_prints_an_exception() {
    for c in &content().cards {
        let f = crate::face::card_face(c);
        let all = serde_json::to_string(&f).unwrap();
        assert!(!all.contains('*'), "{} prints a *: {all}", c.id);
        assert!(!all.contains(" worth"), "{} prints worth: {all}", c.id);
    }
}

/// 059: a removal names its pieces, and a choice of pieces is one kind from one hex.
#[test]
fn a_removal_says_which_pieces() {
    let monitors = face_of("ceasefire_monitors").text;
    assert!(monitors.starts_with("Remove 3 adversary ambassadors or 1 troop from a hex in reach."), "{monitors}");
    assert_eq!(face_of("bounty_board").text, "Remove 1 troop in reach. Gain 1 credit per Security tag on your cards.");
    // 076 (ruling 6): a printed number in one country, never "per hex".
    assert_eq!(face_of("eviction_worm").text, "Remove 6 adversary ambassadors in one country.");
    assert_eq!(face_of("kinetic_interdiction").text, "Pay 1 antimatter. Remove 6 troops in one country.");
    for c in &content().cards {
        assert!(!c.text.contains("per hex"), "{}: {}", c.id, c.text);
    }
}

/// 063: a facility is a station or factory; a building is always a card.
#[test]
fn harden_says_facility_and_a_shield_says_shield() {
    assert_eq!(face_of("hardened_conduits").text, "Harden 3 facilities in reach.");
    assert_eq!(face_of("benchmark_suite").text, "Shield 3 of your nodes.");
}

/// 063: one mark, one reading.
#[test]
fn each_mark_has_one_reading() {
    let json = |id: &str| serde_json::to_string(&face_of(id)).unwrap();
    assert!(json("sentinel_mesh").contains(r#""−2","jam""#));
    assert!(json("beamed_power_constellation").contains(r#""3","free_node""#));
    // Dashed is another player; the solid box is one of your cards (076: never a count across them).
    assert!(json("vendor_financing").contains(r#"{"other":["3","credits"]}"#));
    assert!(json("recruit_operatives").contains(r#"{"yours":["3","agents"]}"#));
    assert!(json("commons_grid").contains(r#"["OR","→","energy","/","followers"]"#));
    assert!(json("aegis_dome_array").contains(r#"["harden","country"]"#));
    for c in &content().cards {
        let all = serde_json::to_string(&crate::face::card_face(c)).unwrap();
        for token in ["datasets", "capital", "followers", "agents"] {
            assert!(!all.contains(&format!("{{\"other\":[\"{token}\"")), "{}: tokens in the other's box", c.id);
        }
    }
}
