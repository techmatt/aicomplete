use aic_core::content::{content, Resource, Uncover};
use aic_core::{Config, Rules};

use super::blocks::*;
use super::run::{op_docs, unknown_params};

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
        ("ai.slots=4,3", |r| boards_block(r, content())),
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
    c.ais[0].presence[1].income[0].1 += 1;
    c.ais[1].presence2[2].cost += 1;
    c.ais[1].powers.iter_mut().find(|p| p.name == "Stand Down").unwrap().tags[0].1 += 1;
    c.ais[0].innate.name = "Something else".to_string();
    c.map.countries[0].megaproject.cubes += 1;
    assert_ne!(presence_track_block(base), presence_track_block(&c));
    assert_ne!(presence_track2_block(base), presence_track2_block(&c));
    assert_ne!(powers_block(base), powers_block(&c));
    assert_ne!(board_details_block(base), board_details_block(&c));
    assert_ne!(corps_block(base), corps_block(&c));
}

#[test]
fn check_flags_a_bare_name_that_is_no_tunable() {
    let r = baseline(&[]);
    assert!(unknown_params("`hold_base`, `harden_buy`", &r, &[]).is_empty());
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
        income: vec![(Resource::Credits, 2), (Resource::Chips, 1)],
        keep2: true,
        cost: 3,
        tags: vec![],
        capstone: None,
    };
    assert_eq!(space(&u), "+2 credits, +1 chip, **keep upgrade** (cost 3)");
}
