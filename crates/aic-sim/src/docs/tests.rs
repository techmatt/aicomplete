use aic_core::content::{content, Resource, Uncover};
use aic_core::{Config, Rules};

use super::blocks::*;
use super::ladder::{difficulty_block, ladder};
use super::run::{
    block, budget_warnings, card_note_failures, card_quote_failures, op_docs, op_uses, revision_line_failures, rulebook_revision_failure, rules_file_failures, unknown_params, BLOCKS,
    DOC_BUDGET, DOC_WARN, PALETTE_OPS,
};

const BASELINE_JSON: &str = include_str!("../../../../configs/baseline.json");

fn baseline(overrides: &[&str]) -> Rules {
    let mut c = Config::from_json(BASELINE_JSON).unwrap();
    for o in overrides {
        c.apply_override(o).unwrap();
    }
    Rules::new(&c).unwrap()
}

/// 286 (D130): no tier-1 threat takes its own site (its creation and the anchor agent stay under the number at every
/// level); the tier-2 ones that do are counted, and a creation one larger at tier 1 is refused.
#[test]
fn only_tier_two_threats_take_their_own_site() {
    use super::harms::{captures, harm_failures};
    let r = baseline(&[]);
    let taken = captures(&r);
    assert!(taken.iter().all(|c| c.tier == 2), "{:?}", taken.iter().map(|c| &c.name).collect::<Vec<_>>());
    for name in ["Insurrection", "Trade Blockade", "Closed City", "Rootkit"] {
        assert!(taken.iter().any(|c| c.name == name), "{name}");
    }
    assert!(harm_failures(&r).is_empty(), "{:?}", harm_failures(&r));
    // The anchor agent at 2 would make Agitators' 1 reach a city's 3 (from level 1).
    let two = baseline(&["adversary.threat_agents=2"]);
    assert!(harm_failures(&two).iter().any(|f| f.contains("Agitators")), "{:?}", harm_failures(&two));
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
    // 229: the deal's tunables are pinned here, so a dial item moving the baseline doesn't break this test
    // (`a_config_change_changes_its_block` checks the block moves with them). With them: 5, 5, 3 cards;
    // difficulty 3 removes two stage-II cards; difficulty 4 swaps a stage-I card for a stage-III.
    // 200: difficulty 0 deals as 1 (196's `stage_extra` is gone).
    // 267: `stage2_added` pinned off here; the baseline's 3 / 4 / 4 at d4 is the core's setup test's.
    const DEAL: [&str; 7] = [
        "adversary.stage_cards=5,5,3",
        "adversary.stage1_added=0",
        "adversary.stage2_removed=2",
        "adversary.stage_swap_at=4",
        "adversary.stage1_removed=1",
        "adversary.stage2_added=0",
        "adversary.stage3_added=1",
    ];
    let cap = |more: &[&str]| {
        let mut sets: Vec<&str> = DEAL.to_vec();
        sets.extend_from_slice(more);
        round_cap_block(&baseline(&sets))
    };
    let base = cap(&[]);
    assert_eq!(row(&base, "0"), ["0", "5", "5", "3", "6", "13"]);
    assert_eq!(row(&base, "1"), ["1", "5", "5", "3", "6", "13"]);
    assert_eq!(row(&base, "2"), ["2", "5", "5", "3", "6", "13"]);
    assert_eq!(row(&base, "3"), ["3", "5", "3", "3", "6", "11"]);
    assert_eq!(row(&base, "4"), ["4", "4", "3", "4", "5", "11"]);
    let shorter = cap(&["adversary.stage_cards=4,4,2"]);
    assert_eq!(row(&shorter, "1")[5], "10");
    let harsher = cap(&["adversary.stage1_removed=2"]);
    assert_eq!(row(&base, "3")[5], row(&harsher, "3")[5], "difficulty 3 removes no stage-I card");
    assert_eq!(row(&harsher, "4")[5], "10");
}

/// 099: every standard action has its row (r35's Free a station had none),
/// plus selling a card, which is no standard action.
#[test]
fn every_standard_action_has_a_row() {
    let r = baseline(&[]);
    let block = standard_actions_block(&r);
    let rows = block.lines().filter(|l| l.starts_with("| ") && !l.starts_with("| Action |")).count();
    let w = r.world();
    let offered = aic_core::modules::board::STANDARD.iter().filter(|&&a| aic_core::modules::board::std_offered(&w, a)).count();
    assert_eq!(rows, offered + 1, "{block}");
}

/// 255 (247's note): every standard action has its name in the viewer (`STANDARD_NAMES`, web/src/format.ts) and its
/// pricing row and usage key in scripts/design_check.json, and nothing there names one that is gone.
#[test]
fn every_standard_action_is_named_everywhere() {
    const FORMAT_TS: &str = include_str!("../../../../web/src/format.ts");
    const DESIGN_CHECK: &str = include_str!("../../../../scripts/design_check.json");
    let names_block = FORMAT_TS.split("const STANDARD_NAMES").nth(1).and_then(|s| s.split("};").next()).expect("STANDARD_NAMES");
    let check: serde_json::Value = serde_json::from_str(DESIGN_CHECK).unwrap();
    let srcs: Vec<&str> =
        check["standard_actions"]["rows"].as_array().unwrap().iter().filter_map(|r| r["src"].as_str()).collect();
    let usage: Vec<&str> = check["usage"]["rows"].as_array().unwrap().iter().filter_map(|r| r["key"].as_str()).collect();
    let standard = aic_core::modules::board::STANDARD;
    for act in standard {
        let key = format!("standard:{}", act.name());
        assert!(names_block.contains(&format!("\n  {}: ", act.name())), "{} has no STANDARD_NAMES entry", act.name());
        // Move spends no resource (the ambassador's action), so it has no give / get row.
        let priced = act !=aic_core::modules::board::StandardAction::Move;
        assert!(!priced || srcs.contains(&key.as_str()), "{key} has no design_check.json standard_actions row");
        assert!(usage.contains(&key.as_str()), "{key} has no design_check.json usage row");
    }
    let known = |k: &str| k == "standard:sell" || standard.iter().any(|a| k == format!("standard:{}", a.name()));
    for k in srcs.iter().chain(&usage).filter(|k| k.starts_with("standard:")) {
        assert!(known(k), "design_check.json names {k}, no standard action");
    }
    let entries = names_block.lines().filter(|l| l.starts_with("  ") && l.contains(": ")).count();
    assert_eq!(entries, standard.len(), "STANDARD_NAMES has an entry for a standard action that is gone");
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
    let blocks: [(&str, Block); 7] = [
        (&stage2_removed, difficulty_block),
        ("board.remove_count=2", standard_actions_block),
        ("alignment.botnet=2", sway_sources_block),
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
    c.ais[1].powers.iter_mut().find(|p| p.name == "Interdict I").unwrap().tags[0].1 += 1;
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
    assert!(unknown_params("`legal_actions`, `play_tag`", &r, &[]).is_empty());
    assert_eq!(unknown_params("`hold_bass`", &r, &[]), ["hold_bass"]);
}

/// 267: the ladder is every level, each adding at most one named rule, the count cumulative; level 0 prints
/// the whole setup and no rule (the base adversary).
#[test]
fn the_ladder_adds_a_rule_at_most_a_level() {
    let l = ladder(&baseline(&[]));
    assert_eq!(l.iter().map(|v| v.level).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
    assert!(l[0].rule.is_empty() && l[0].setup.len() >= 4, "{:?}", l[0]);
    let mut active = 0;
    for v in &l {
        assert!(v.new_rules <= 1, "{v:?}");
        active += v.new_rules;
        assert_eq!(v.active_rules, active, "{v:?}");
        // A named rule stands for at least one mid-game change; one may merge two (Sleeper Cells did until 269 printed the spread on its cards).
        assert!(v.new_changes >= v.new_rules, "{v:?}");
        assert!(v.level == 0 || !v.setup.is_empty() || !v.rule.is_empty(), "level {} adds nothing", v.level);
    }
}

/// 229 (Matt's ruling): rulebook §21's difficulty table is generated, in players' words (no tunable names).
#[test]
fn the_rulebook_difficulty_table_is_in_players_words() {
    let book = include_str!("../../../../docs/rulebook.md");
    let open = "<!-- generated: rulebook_difficulty -->
";
    assert_eq!(book.matches(open).count(), 1);
    let body = book.split_once(open).unwrap().1.split_once("<!-- /generated -->").expect("closed").0;
    assert!(body.starts_with("| Level | Name | Setup (set once"), "{body}");
    assert!(!body.contains('`'), "a tunable name in the rulebook: {body}");
}

/// 229: every palette op the ops doc names is an op.
#[test]
fn the_palette_ops_are_ops() {
    let ops: Vec<String> = op_docs(include_str!("../../../aic-core/src/op.rs")).into_iter().map(|o| o.name).collect();
    for p in PALETTE_OPS {
        assert!(ops.iter().any(|o| o == p), "{p} is no op");
    }
}

/// 286: every op no content prints is run once by aic-core's `tests/unprinted_ops.rs` (a sample naming it,
/// `"op":"<name>"`), so a new unprinted op gets its lab run, and one that loses its last card too.
#[test]
fn every_unprinted_op_has_a_lab_run() {
    let lab = include_str!("../../../aic-core/src/tests/unprinted_ops.rs");
    let uses = op_uses();
    let missing: Vec<String> = op_docs(include_str!("../../../aic-core/src/op.rs"))
        .into_iter()
        .map(|o| o.name)
        .filter(|n| !uses.contains_key(n) && !lab.contains(&format!("\"op\":\"{n}\"")))
        .collect();
    assert!(missing.is_empty(), "ops no content prints with no run in aic-core's tests/unprinted_ops.rs: {missing:?}");
}

#[test]
fn check_takes_an_op_name() {
    let r = baseline(&[]);
    let ops: Vec<String> = op_docs(include_str!("../../../aic-core/src/op.rs")).into_iter().map(|o| o.name).collect();
    assert!(ops.iter().any(|o| o == "lift_fallout"));
    assert!(unknown_params("`lift_fallout`, `move_one`", &r, &ops).is_empty());
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

/// 284: `experiments/history.md` indexes every item's write-up (five went unindexed, three of them rules bumps):
/// a results folder with a `README.md` is named in a row (`results/<folder>/`).
#[test]
fn every_write_up_has_a_history_row() {
    let exp = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../experiments");
    let history = std::fs::read_to_string(exp.join("history.md")).unwrap();
    let mut missing = Vec::new();
    for e in std::fs::read_dir(exp.join("results")).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if p.join("README.md").is_file() && !history.contains(&format!("results/{name}/")) {
            missing.push(name);
        }
    }
    missing.sort();
    assert!(missing.is_empty(), "experiments/history.md has no row for {missing:?}: add `| NNN summary | Rev | [folder](results/folder/README.md) |`");
    // 286: a row's summary is a title clause (rows had grown to 450 characters; the findings are in the write-up).
    let long: Vec<&str> = history
        .lines()
        .filter_map(|l| l.strip_prefix("| ").and_then(|r| r.split(" | ").next()))
        .filter(|s| s.chars().count() > 100)
        .collect();
    assert!(long.is_empty(), "experiments/history.md summaries over 100 characters (cut to a title clause): {long:?}");
}
