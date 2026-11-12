use aic_core::content::{content, Resource, Uncover};
use aic_core::{Config, Rules};

use super::blocks::*;
use super::run::{
    block, budget_warnings, card_note_failures, card_quote_failures, op_docs, revision_line_failures, rulebook_revision_failure, rules_file_failures, unknown_params, BLOCKS,
    DOC_BUDGET, DOC_WARN,
};

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
    // At the baseline (138): 4, 4, 3 cards; difficulty 3 removes two stage-II cards
    // (150: `stage2_removed` 2); difficulty 4 (142: stage_swap_at, 2 before) swaps a
    // stage-I card for a stage-III.
    assert_eq!(row(&base, "0"), ["0", "4", "4", "3", "5", "11"]);
    assert_eq!(row(&base, "2"), ["2", "4", "4", "3", "5", "11"]);
    assert_eq!(row(&base, "3"), ["3", "4", "2", "3", "5", "9"]);
    assert_eq!(row(&base, "4"), ["4", "3", "2", "4", "4", "9"]);
    let shorter = round_cap_block(&baseline(&["adversary.stage_cards=4,4,2"]));
    assert_eq!(row(&shorter, "0")[5], "10");
    let harsher = round_cap_block(&baseline(&["adversary.stage1_removed=2"]));
    assert_eq!(row(&base, "3")[5], row(&harsher, "3")[5], "difficulty 3 removes no stage-I card");
    assert_eq!(row(&harsher, "4")[5], "8");
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
    // One off the baseline, so a dial item landing on any value can't make it a no-op
    // (149); down from the range's top, which an override can't pass (150).
    let s2 = base.world().int("adversary.stage2_removed");
    let top = base.tunables().into_iter().find(|t| t.key == "adversary.stage2_removed").expect("a tunable").max;
    let stage2_removed = format!("adversary.stage2_removed={}", if s2 < top { s2 + 1 } else { s2 - 1 });
    type Block = fn(&Rules) -> String;
    let blocks: [(&str, Block); 8] = [
        (&stage2_removed, difficulty_block),
        ("board.remove_count=2", standard_actions_block),
        ("board.clear_unity=3", sway_sources_block),
        ("alignment.purge_at=1,3,5", purge_ease_block),
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
    for (name, _) in BLOCKS {
        assert!(block(name, &r).is_some(), "{name}");
    }
    assert!(block("no_such_block", &r).is_none());
}

/// 101: the rules are split over docs/rules/; a block that went missing in the move, or one
/// pasted twice, or a file past the budget, fails the check.
#[test]
fn check_wants_each_block_once_and_files_in_budget() {
    let all: String = BLOCKS.iter().map(|(b, _)| format!("<!-- generated: {b} -->\n<!-- /generated -->\n")).collect();
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
    let big = vec![("docs/rules/a.md".to_string(), all + &"x".repeat(DOC_BUDGET))];
    assert_eq!(rules_file_failures(&big).len(), 1);
}

/// 165: a doc within 1 KB of the budget is warned of; one under it, or over the budget (a failure), is not.
#[test]
fn docs_near_the_budget_are_warned_of() {
    let sizes = |n: usize| vec![("docs/a.md".to_string(), n)];
    assert!(budget_warnings(&sizes(DOC_WARN)).is_empty());
    let w = budget_warnings(&sizes(DOC_WARN + 1));
    assert_eq!(w.len(), 1);
    assert!(w[0].starts_with("warning: docs/a.md"), "{w:?}");
    assert_eq!(budget_warnings(&sizes(DOC_BUDGET)).len(), 1);
    assert!(budget_warnings(&sizes(DOC_BUDGET + 1)).is_empty());
}

/// 118: a rules file quoting a card's words its face no longer prints fails the check.
#[test]
fn check_flags_a_card_quote_its_face_no_longer_prints() {
    let faces = vec![("Citation Engine".to_string(), "Effect: when you play a Science card, gain 7 credits.".to_string())];
    let ok = vec![("docs/rules/a.md".to_string(), r#"Say "When you play a Science card, gain 7 credits" (Citation Engine)."#.to_string())];
    assert!(card_quote_failures(&ok, &faces).is_empty());
    let stale = vec![("docs/rules/a.md".to_string(), r#"Say "When you play a Science card, gain 6 credits" (Citation Engine)."#.to_string())];
    assert_eq!(card_quote_failures(&stale, &faces).len(), 1);
    let other = vec![("docs/rules/a.md".to_string(), r#"A "house term" (not a card) and "x" (Citation Engine)."#.to_string())];
    assert!(card_quote_failures(&other, &faces).is_empty(), "no card named, or too short to be a quote");
    let pattern = vec![("docs/rules/a.md".to_string(), r#""Gain up to N credits" (Citation Engine)"#.to_string())];
    assert!(card_quote_failures(&pattern, &faces).is_empty(), "a pattern with N is no quote");
    let wrapped = vec![("docs/rules/a.md".to_string(), "\"When you play a Science card, gain 6 credits\" (Citation\n  Engine)".to_string())];
    assert_eq!(card_quote_failures(&wrapped, &faces).len(), 1, "a name wrapped across a line");
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
        moves: 0,
        capstone: None,
        insight: Some(9),
    };
    // 179 (Matt, friction #16): the cost prints as a cost, first.
    assert_eq!(space(&u), "pay 3 energy: **insight 9**, +2 credits, +1 chip, **keep upgrade**");
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


/// 117: the rules README names the revision the code implements, and a stale one fails.
#[test]
fn the_rules_readme_names_the_rules_revision() {
    let readme = include_str!("../../../../docs/rules/README.md");
    let files = vec![("docs/rules/README.md".to_string(), readme.to_string())];
    let ok = revision_line_failures(&files, aic_core::RULES_REVISION);
    assert!(ok.is_empty(), "{ok:?}");
    let stale = vec![("docs/rules/README.md".to_string(), "at rules revision 43 (x)".to_string())];
    assert_eq!(revision_line_failures(&stale, 45).len(), 1);
    let none = vec![("docs/rules/README.md".to_string(), "no line".to_string())];
    assert_eq!(revision_line_failures(&none, 45).len(), 1);
}

/// 184: the rulebook names the revision too (it said 77 at 78).
#[test]
fn the_rulebook_names_the_rules_revision() {
    let book = include_str!("../../../../docs/rulebook.md");
    assert_eq!(rulebook_revision_failure(book, aic_core::RULES_REVISION), None);
    assert!(rulebook_revision_failure("*Rules revision 43. x", 45).is_some());
    assert!(rulebook_revision_failure("no line", 45).is_some());
}

/// 106: every card `tools/card-art/samples.json` names is in content (one named a card cut
/// at r41, unnoticed until the art tool was run).
#[test]
fn the_art_samples_name_cards_in_content() {
    let v: serde_json::Value = serde_json::from_str(include_str!("../../../../tools/card-art/samples.json")).unwrap();
    let ids: Vec<&str> = content().cards.iter().map(|c| c.id.as_str()).collect();
    let named: Vec<&str> = v["variants"]
        .as_object()
        .expect("samples.json has variants")
        .values()
        .chain(v["splay"].as_array().expect("samples.json has a splay"))
        .map(|x| x.as_str().expect("a card id"))
        .collect();
    assert!(named.len() > 3, "{named:?}");
    let gone: Vec<&&str> = named.iter().filter(|n| !ids.contains(n)).collect();
    assert!(gone.is_empty(), "samples.json names cards not in content: {gone:?}");
}
