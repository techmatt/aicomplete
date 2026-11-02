use super::*;

fn face_of(id: &str) -> crate::face::Face {
    crate::face::card_face(content().cards.iter().find(|c| c.id == id).expect("a card"))
}

#[test]
fn a_face_opens_with_its_boxes_word() {
    let f = face_of("synthetic_data_forge");
    assert_eq!(f.top_text, "Each round, 2 Datasets here. Action: pay 4 Datasets, get 2 insight.");
    assert_eq!(f.text, "");
    let f = face_of("orbital_embassy");
    assert_eq!(f.top_text, "Each round, 1 ambassador, 2 credits and 1 insight.");
    assert_eq!(f.text, "Pay 1 antimatter. Gain 2 ambassadors.");
}

#[test]
fn a_bare_arrow_is_a_free_action() {
    let f = face_of("agency_field_station");
    assert!(f.top_text.contains("Action: get 1 credit per 2 Agents you have."), "{}", f.top_text);
}

#[test]
fn income_below_the_art_raises_or_lowers_the_track() {
    let f = face_of("antiproton_offtake_deal");
    assert_eq!(f.top_text, "");
    assert_eq!(f.text, "Lower your credit income by 1. Raise your antimatter income by 1.");
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
    assert_eq!(face_of("eviction_worm").text, "Remove 2 adversary ambassadors or 1 troop from a hex in reach.");
    assert_eq!(face_of("spoofed_credentials").text, "Remove 1 adversary ambassador in reach.");
    assert!(face_of("agency_directorate").top_text.contains("remove 1 troop per hex of a country"));
}

/// 063: a facility is a station or factory; a building is always a card.
#[test]
fn harden_says_facility_and_a_shield_says_shield() {
    assert_eq!(face_of("hardened_conduits").text, "Pay 1 energy. Harden 3 facilities in reach.");
    assert_eq!(face_of("blackstart_protocol").text, "Gain 2 energy. Harden 1 facility in reach.");
    assert_eq!(face_of("benchmark_suite").text, "Gain 1 insight. Shield 2 of your presence.");
}

/// 063: one mark, one reading.
#[test]
fn each_mark_has_one_reading() {
    let json = |id: &str| serde_json::to_string(&face_of(id)).unwrap();
    assert!(json("sentinel_mesh").contains(r#""−2","jam""#));
    assert!(json("beamed_power_constellation").contains(r#""3","free_presence""#));
    assert!(json("vendor_financing").contains(r#""−≤4","credits",{"other":["≤4","credits"]}"#));
    // Dashed is another player; the solid box counts across your cards.
    assert!(json("commons_grid").contains(r#"{"yours":["2","followers"]}"#));
    assert!(json("aegis_dome_array").contains(r#"["harden","country"]"#));
    for c in &content().cards {
        let all = serde_json::to_string(&crate::face::card_face(c)).unwrap();
        for token in ["datasets", "capital", "followers", "agents"] {
            assert!(!all.contains(&format!("{{\"other\":[\"{token}\"")), "{}: tokens in the other's box", c.id);
        }
    }
}
