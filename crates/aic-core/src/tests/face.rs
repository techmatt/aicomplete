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
    assert_eq!(f.top_text, "Each round, 4 credits and 1 insight.");
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
