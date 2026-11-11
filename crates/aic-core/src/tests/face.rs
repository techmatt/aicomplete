use super::*;

fn face_of(id: &str) -> crate::face::Face {
    crate::face::card_face(content().cards.iter().find(|c| c.id == id).expect("a card"))
}

/// 119: a card as the test writes it (copied from content at r47), so a reprice or a
/// card pass leaves the face-grammar tests alone (108's note).
fn pinned(json: &str) -> crate::face::Face {
    let c: crate::content::CardDef = serde_json::from_str(json).expect("a card");
    crate::face::card_face(&c)
}

const FORGE: &str = r#"{"id": "synthetic_data_forge", "name": "Synthetic Data Forge", "tier": 1, "kind": "building",
    "cost": 8, "insight": 1, "tags": ["science"], "tokens": {"kind": "datasets"},
    "ability": {"effects": [{"op": "add_tokens", "kind": "datasets", "amount": 1, "here": true}],
        "or": {"effects": [{"op": "gain_per_tokens", "resource": "insight", "kind": "datasets", "per": 1, "here": true}]}},
    "effects": [{"op": "add_tokens", "kind": "datasets", "amount": 1, "here": true}]}"#;

#[test]
fn a_face_opens_with_its_boxes_word() {
    let f = pinned(FORGE);
    assert_eq!(f.top_text, "Action: add 1 Dataset here, OR get 1 insight per Dataset here.");
    assert_eq!(f.text, "Add 1 Dataset here.");
    let f = pinned(
        r#"{"id": "orbital_mass_driver", "name": "Orbital Mass Driver", "tier": 2, "kind": "building", "cost": 28,
        "chips": 2, "insight": 2, "tags": ["industry", "power"],
        "passives": [{"op": "income", "resource": "credits", "amount": 16}, {"op": "income", "resource": "chips", "amount": 3}],
        "effects": [{"op": "draw", "count": 3}]}"#,
    );
    assert_eq!(f.top_text, "Each round, 16 credits and 3 chips.");
    assert_eq!(f.text, "Pay 2 chips. Draw 3 cards.");
}

#[test]
fn a_bare_arrow_is_a_free_action() {
    let f = pinned(
        r#"{"id": "sentinel_mesh", "name": "Sentinel Mesh", "tier": 1, "kind": "building", "cost": 7, "insight": 0,
        "tags": ["cyber", "security"], "ability": {"effects": [{"op": "jam", "amount": 6, "source": "cyber"}]}}"#,
    );
    assert_eq!(f.top_text, "Action: counts as 6 toward a cyber threat.");
    // 076: an OR Action prints each half; the second line opens with OR.
    let f = pinned(FORGE);
    let json = serde_json::to_string(&f.top).unwrap();
    assert_eq!(json, r#"[["→","+1","datasets"],["OR","→","insight","/","datasets"]]"#);
}

#[test]
fn income_below_the_art_raises_or_lowers_the_track() {
    let f = pinned(
        r#"{"id": "endowed_research_chair", "name": "Endowed Research Chair", "tier": 1, "kind": "program", "cost": 8,
        "insight": 1, "lowers": [["credits", 1]], "tags": ["science"], "effects": [{"op": "production", "resource": "insight", "amount": 2}]}"#,
    );
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
    // 178 (amendment 8044): piece names print capitalized on cards.
    assert!(monitors.starts_with("Remove 3 Agents or 1 Troop from a hex in reach."), "{monitors}");
    assert_eq!(
        face_of("bounty_board").text,
        "Remove 1 Troop in reach. Gain 1 credit per Security tag on your cards (including this)."
    );
    // 076 (ruling 6): a printed number in one country, never "per hex"; 111 (Matt's ruling 3): most in reach.
    assert_eq!(
        face_of("eviction_worm").text,
        "Remove 3 Agents in reach. Remove 4 compute from an adversary project."
    );
    assert_eq!(
        face_of("kinetic_interdiction").text,
        "Pay 1 antimatter. Remove 2 Troops in one country. Remove 6 Agents in reach."
    );
    for c in &content().cards {
        assert!(!c.text.contains("per hex"), "{}: {}", c.id, c.text);
    }
}

/// 063: a facility is a station or factory; a building is always a card.
#[test]
fn harden_says_facility_and_a_shield_says_shield() {
    assert_eq!(face_of("faraday_yard").text, "Raise your energy income by 1. Harden 3 facilities in reach.");
    assert_eq!(face_of("benchmark_suite").text, "Shield 3 of your Nodes.");
}

/// 063: one mark, one reading.
#[test]
fn each_mark_has_one_reading() {
    // A program whose effects the test names (119: pinned, not read from content).
    let json = |effects: &str| {
        let card = format!(r#"{{"id": "t", "name": "T", "tier": 1, "kind": "program", "cost": 8, "insight": 1, "tags": ["science"], "effects": {effects}}}"#);
        serde_json::to_string(&pinned(&card)).unwrap()
    };
    assert!(json(r#"[{"op": "jam", "amount": 6}, {"op": "draw", "count": 2}]"#).contains(r#""−6","jam""#));
    assert!(json(r#"[{"op": "place_node", "count": 3, "free": true}, {"op": "gain_per_node", "resource": "energy"}]"#).contains(r#""3","free_node""#));
    // Dashed is another player; the solid box is one of your cards (076: never a count across them).
    let other = r#"[{"op": "choose_player"}, {"op": "other_gain", "resource": "chips", "amount": 2}, {"op": "production", "resource": "credits", "amount": 4}]"#;
    assert!(json(other).contains(r#"{"other":["2","chips"]}"#));
    assert!(json(r#"[{"op": "draw", "count": 2}, {"op": "add_tokens", "kind": "agents", "amount": 1}]"#).contains(r#"{"yours":["agents"]}"#));
    assert!(serde_json::to_string(&pinned(FORGE)).unwrap().contains(r#"["OR","→","insight","/","datasets"]"#));
    // No card prints Civic Shelter since r42: a building is given it here.
    let mut shelter = content().cards.iter().find(|c| c.is_building()).expect("a building").clone();
    shelter.passives = vec![crate::content::Op::CivicShelter];
    // r75: Aegis guards your own pieces, not a country: shield + node.
    assert!(serde_json::to_string(&crate::face::card_face(&shelter)).unwrap().contains(r#"["harden","node"]"#));
    for c in &content().cards {
        let all = serde_json::to_string(&crate::face::card_face(c)).unwrap();
        for token in ["datasets", "capital", "followers", "agents"] {
            assert!(!all.contains(&format!("{{\"other\":[\"{token}\"")), "{}: tokens in the other's box", c.id);
        }
    }
}

/// 100: every word of every icon line is one `web/card/card.js` `part()` draws: an
/// icon or tag of `web/card/icons.js`, a connector, or a number. 090 met a stray word
/// only when `npm run cards:fit` threw "no icon or number".
#[test]
fn icon_lines_print_only_what_the_viewer_draws() {
    use crate::face::Part;
    let js = include_str!("../../../../web/card/icons.js");
    let mut known: BTreeSet<String> = ["→", "/", ":", "OR", "ALL", "ACT"].iter().map(|s| s.to_string()).collect();
    for (table, prefix) in [("RESOURCES", ""), ("TOKENS", ""), ("PIECES", ""), ("TAGS", "#")] {
        let start = js.find(&format!("const {table} = {{")).unwrap_or_else(|| panic!("icons.js: no {table}"));
        let body = &js[start..];
        let body = &body[..body.find("\n};").expect("the table closes")];
        for line in body.lines().skip(1) {
            if let Some(key) = line.strip_prefix("  ").and_then(|l| l.split(':').next()) {
                if !key.is_empty() && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    known.insert(format!("{prefix}{key}"));
                }
            }
        }
    }
    assert!(known.contains("credits") && known.contains("troop") && known.contains("#cyber"), "{known:?}");
    let number = |w: &str| {
        // card.js: /^[−+]?[≤≥]?[\dn]+$/
        let w = w.strip_prefix(['−', '+']).unwrap_or(w);
        let w = w.strip_prefix(['≤', '≥']).unwrap_or(w);
        !w.is_empty() && w.chars().all(|c| c.is_ascii_digit() || c == 'n')
    };
    fn words<'a>(parts: &'a [Part], out: &mut Vec<&'a str>) {
        for p in parts {
            match p {
                Part::Word(w) => out.push(w),
                Part::Income { income: v } | Part::Adversary { adversary: v } | Part::Other { other: v } | Part::Yours { yours: v } => words(v, out),
            }
        }
    }
    let mut bad = Vec::new();
    for c in &content().cards {
        let f = crate::face::card_face(c);
        let mut all = Vec::new();
        for line in f.top.iter().chain(f.effect.iter()) {
            words(line, &mut all);
        }
        bad.extend(all.into_iter().filter(|w| !known.contains(*w) && !number(w)).map(|w| format!("{}: {w:?}", c.id)));
    }
    assert!(bad.is_empty(), "icon-line words the viewer can't draw:\n{}", bad.join("\n"));
}

/// r48 (D48): icons say what the words say: Great Liberation's city on its cube,
/// Field Grant's station limit. 178 (#29): every liberation puts your cube on, so no card prints the align mark
/// (Hostile Buyout's was the one).
#[test]
fn icons_show_the_city_and_the_station() {
    let line = |id: &str| serde_json::to_string(&face_of(id).effect).unwrap();
    assert!(line("great_liberation").contains(r#""cube","city""#), "{}", line("great_liberation"));
    assert!(line("field_grant").contains(r#""megaproject","/","station""#), "{}", line("field_grant"));
    assert!(!line("hostile_buyout").contains(r#""→","cube""#), "{}", line("hostile_buyout"));
}
