use aic_core::content::{content, Resource, Uncover};
use aic_core::{Config, Rules};

use super::blocks::*;
use super::run::{block, card_note_failures, op_docs, rules_file_failures, unknown_params, BLOCKS, RULES_BUDGET};

const BASELINE_JSON: &str = include_str!("../../../../configs/baseline.json");

fn baseline(overrides: &[&str]) -> Rules {
    let mut c = Config::from_json(BASELINE_JSON).unwrap();
    for o in overrides {
        c.apply_override(o).unwrap();
    }
    Rules::new(&c).unwrap()
}

/// A row of `block` whose first cell is `first`, as its cells.
fn row(block: &str, first: &str) -> Vec<String> {
    let line = block
        .lines()
        .find(|l| l.starts_with(&format!("| {first} |")))
        .unwrap_or_else(|| panic!("no row {first} in {block}"));
    line.trim_matches('|').split(" | ").map(|c| c.trim().to_string()).collect()
}

#[test]
fn the_round_cap_comes_from_the_engines_deal() {
    let base = round_cap_block(&baseline(&[]));
    // At the baseline: 4, 5, 2 cards; difficulty 2 swaps a stage-I card for a stage-III.
    assert_eq!(row(&base, "0"), ["0", "4", "5", "2", "5", "11"]);
    assert_eq!(row(&base, "2"), ["2", "3", "5", "3", "4", "11"]);
    let shorter = round_cap_block(&baseline(&["adversary.stage_cards=4,4,2"]));
    assert_eq!(row(&shorter, "0")[5], "10");
    let harsher = round_cap_block(&baseline(&["adversary.stage1_removed=2"]));
    assert_eq!(row(&base, "1")[5], row(&harsher, "1")[5], "difficulty 1 removes nothing");
    assert_eq!(row(&harsher, "2")[5], "10");
}

/// 099: every standard action has its row (r35's Free a station had none),
/// plus selling a card, which is no standard action.
#[test]
fn every_standard_action_has_a_row() {
    let block = standard_actions_block(&baseline(&[]));
    let rows = block.lines().filter(|l| l.starts_with("| ") && !l.starts_with("| Action |")).count();
    assert_eq!(rows, aic_core::modules::board::STANDARD.len() + 1, "{block}");
}

/// Every block that reads a config moves with it.
#[test]
fn a_config_change_changes_its_block() {
    let base = baseline(&[]);
    type Block = fn(&Rules) -> String;
    let blocks: [(&str, Block); 8] = [
        ("adversary.build_bonus=2", difficulty_block),
        ("board.remove_count=2", standard_actions_block),
        ("board.base_unity=3", sway_sources_block),
        ("alignment.unity_ease=1,3,5", purge_ease_block),
        ("cards.keep_tier2=5", new_players_block),
        ("adversary.corrupt_at=7", game_end_block),
        ("adversary.stage_cards=3,5,2", round_cap_block),
        ("ai.slots=4,3,2", |r| boards_block(r, content())),
    ];
    for (o, block) in blocks {
        assert_ne!(block(&base), block(&baseline(&[o])), "{o}");
    }
}

/// Every block that reads content moves with it.
#[test]
fn a_content_change_changes_its_block() {
    let base = content();
    let mut c = base.clone();
    c.ais[0].economy_track[1].income[0].1 += 1;
    c.ais[1].play_track[2].cost += 1;
    c.ais[1].powers.iter_mut().find(|p| p.name == "Stand Down").unwrap().tags[0].1 += 1;
    c.ais[0].innate.name = "Something else".to_string();
    c.map.countries[0].megaproject.cubes += 1;
    assert_ne!(node_track_block(base), node_track_block(&c));
    assert_ne!(node_track2_block(base), node_track2_block(&c));
    assert_ne!(powers_block(base), powers_block(&c));
    assert_ne!(board_details_block(base), board_details_block(&c));
    assert_ne!(corps_block(base), corps_block(&c));
}

#[test]
fn every_listed_block_has_a_body() {
    let r = baseline(&[]);
    for name in BLOCKS {
        assert!(block(name, &r).is_some(), "{name}");
    }
    assert!(block("no_such_block", &r).is_none());
}

/// 101: the rules are split over docs/rules/; a block that went missing in the move, or one
/// pasted twice, or a file past the budget, fails the check.
#[test]
fn check_wants_each_block_once_and_files_in_budget() {
    let all: String = BLOCKS.iter().map(|b| format!("<!-- generated: {b} -->\n<!-- /generated -->\n")).collect();
    let ok = vec![("docs/rules/a.md".to_string(), all.clone())];
    assert!(rules_file_failures(&ok).is_empty());
    let missing = vec![("docs/rules/a.md".to_string(), all.replace("generated: tunables -->", "x -->"))];
    let f = rules_file_failures(&missing);
    assert_eq!(f.len(), 1);
    assert!(f[0].contains("\"tunables\" stands 0 times"), "{f:?}");
    let twice = vec![
        ("docs/rules/a.md".to_string(), all.clone()),
        ("docs/rules/b.md".to_string(), "<!-- generated: corps -->".to_string()),
    ];
    assert_eq!(rules_file_failures(&twice).len(), 1);
    let big = vec![("docs/rules/a.md".to_string(), all + &"x".repeat(RULES_BUDGET))];
    assert_eq!(rules_file_failures(&big).len(), 1);
}

#[test]
fn check_flags_a_bare_name_that_is_no_tunable() {
    let r = baseline(&[]);
    assert!(unknown_params("`legal_actions`, `harden_buy`", &r, &[]).is_empty());
    assert_eq!(unknown_params("`hold_bass`", &r, &[]), ["hold_bass"]);
}

#[test]
fn check_takes_an_op_name() {
    let r = baseline(&[]);
    let ops: Vec<String> = op_docs(include_str!("../../../aic-core/src/op.rs")).into_iter().map(|o| o.name).collect();
    assert!(ops.iter().any(|o| o == "clear_markers"));
    assert!(unknown_params("`clear_markers`, `move_one`", &r, &ops).is_empty());
}

#[test]
fn a_track_space_prints_income_upgrade_and_cost() {
    let u = Uncover {
        effects: Vec::new(),
        income: vec![(Resource::Credits, 2), (Resource::Chips, 1)],
        keep2: true,
        cost: 3,
        tags: vec![],
        capstone: None,
    };
    assert_eq!(space(&u), "+2 credits, +1 chip, **keep upgrade** (cost 3)");
}

#[test]
fn card_notes_name_cards_one_line_each() {
    let ids = ["interceptor_battery", "requisition"];
    assert!(card_note_failures(r#"{"interceptor_battery": "a line"}"#, &ids).is_empty());
    assert_eq!(card_note_failures(r#"{"no_such_card": "a line"}"#, &ids).len(), 1);
    assert_eq!(card_note_failures(r#"{"requisition": "two\nlines"}"#, &ids).len(), 1);
    assert_eq!(card_note_failures("[]", &ids).len(), 1);
}

#[test]
fn the_card_notes_on_disk_name_cards() {
    let text = include_str!("../../../../docs/card-pricing-notes.json");
    let ids: Vec<&str> = content().cards.iter().map(|c| c.id.as_str()).collect();
    assert!(card_note_failures(text, &ids).is_empty());
}

/// The keys of `const NAME = {` in `web/card/icons.js`: a two-space key at the block's top.
fn icon_keys(js: &str, name: &str) -> Vec<String> {
    let start = js.find(&format!("const {name} = {{")).unwrap_or_else(|| panic!("icons.js has no {name}"));
    let body = &js[start..];
    let body = &body[..body.find("\n};").expect("the block closes")];
    body.lines()
        .skip(1)
        .filter_map(|l| l.strip_prefix("  "))
        .filter(|l| !l.starts_with(' ') && !l.starts_with('/'))
        .filter_map(|l| l.split_once(':').map(|(k, _)| k.trim().to_string()))
        .filter(|k| k.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
        .collect()
}

/// 096 (P4): every word on a face's icon line is one `web/card/card.js` draws (an icon,
/// a `#tag`, a connector or a number), so a face the card-fit run would crash on fails here.
#[test]
fn every_face_word_is_a_known_icon() {
    use aic_core::face::{card_face, Part};
    let js = include_str!("../../../../web/card/icons.js");
    let icons: Vec<String> = ["RESOURCES", "TOKENS", "PIECES"].iter().flat_map(|n| icon_keys(js, n)).collect();
    let tags = icon_keys(js, "TAGS");
    assert!(icons.contains(&"followers".to_string()) && tags.contains(&"science".to_string()), "{icons:?} {tags:?}");
    // card.js `TEXT` and its number pattern /^[−+]?≤?[\dn]+$/.
    let known = |w: &str| {
        ["\u{2192}", "/", ":", "OR", "ALL", "ACT"].contains(&w)
            || icons.iter().any(|i| i == w)
            || w.strip_prefix('#').is_some_and(|t| tags.iter().any(|x| x == t))
            || {
                let n = w.trim_start_matches(['\u{2212}', '+']).trim_start_matches(['\u{2264}', '\u{2265}']);
                !n.is_empty() && n.chars().all(|c| c.is_ascii_digit() || c == 'n')
            }
    };
    fn words<'a>(p: &'a Part, out: &mut Vec<&'a str>) {
        match p {
            Part::Word(w) => out.push(w),
            Part::Income { income: v } | Part::Adversary { adversary: v } | Part::Other { other: v } | Part::Yours { yours: v } => {
                v.iter().for_each(|x| words(x, out))
            }
        }
    }
    let mut bad = vec![];
    for c in &content().cards {
        let f = card_face(c);
        let mut ws = vec![];
        f.top.iter().chain(&f.effect).flatten().for_each(|p| words(p, &mut ws));
        bad.extend(ws.into_iter().filter(|w| !known(w)).map(|w| format!("{}: {w}", c.id)));
    }
    assert!(bad.is_empty(), "icon-line words card.js can't draw: {bad:?}");
}
