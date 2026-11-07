//! The generated docs are current: `aic-sim docs --check`, and docs/cards.md
//! against content (its generator is a Python script, so it is checked here
//! by what it must hold).

mod common;

use std::fs;
use std::path::Path;

use aic_core::content::content;

#[test]
fn generated_docs_are_current() {
    let out = common::sim().args(["docs", "--check"]).output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// 145: a scene's `card` is the only link from a card to its art: one naming
/// an id that left content (135's `adv_s1_war_drums`) loses the art quietly.
#[test]
fn every_art_scene_names_a_card_in_content() {
    fn ids(v: &serde_json::Value, out: &mut std::collections::BTreeSet<String>) {
        match v {
            serde_json::Value::Object(o) => {
                if let Some(serde_json::Value::String(id)) = o.get("id") {
                    out.insert(id.clone());
                }
                o.values().for_each(|x| ids(x, out));
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| ids(x, out)),
            _ => {}
        }
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |p: &str| serde_json::from_str::<serde_json::Value>(&fs::read_to_string(root.join(p)).unwrap()).unwrap();
    let mut known = std::collections::BTreeSet::new();
    for f in ["cards", "adversary", "threats", "projects", "alignment", "ais"] {
        ids(&read(&format!("content/{f}.json")), &mut known);
    }
    let scenes = read("tools/card-art/scenes.json");
    let mut missing = Vec::new();
    for group in scenes["scenes"].as_object().into_iter().flat_map(|o| o.values()) {
        for (scene, s) in group.as_object().into_iter().flatten() {
            if let Some(card) = s.get("card").and_then(|c| c.as_str()) {
                if !known.contains(card) {
                    missing.push(format!("{scene} -> {card}"));
                }
            }
        }
    }
    assert!(missing.is_empty(), "tools/card-art/scenes.json names cards not in content: {missing:?}");
}

#[test]
fn the_card_list_names_every_card_and_this_revision() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/cards.md");
    let doc = fs::read_to_string(path).unwrap();
    assert!(
        doc.contains(&format!("Rules revision {}", aic_core::RULES_REVISION)),
        "docs/cards.md is stale: run python scripts/cards/cards_doc.py"
    );
    for c in &content().cards {
        assert!(doc.contains(&c.name), "docs/cards.md lacks {}", c.name);
    }
}
