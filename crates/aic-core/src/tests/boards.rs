//! 068: the Builder and Protector boards, and Cordon.

use super::*;
use crate::content::{Op, RoundFlag, Tag};
use crate::modules::{board, cards, plan};
use crate::op::{CreateAt, CreateIf};

/// Three Industry programs in the Builder's play area.
fn builder_with_tags(g: &mut GameState) {
    set_pile(g, &cards::played(0), &["payroll_automation", "supply_chain_audit", "vendor_financing"]);
}

#[test]
fn printed_and_revealed_tags_meet_a_requirement() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let extra = |g: &GameState| cards::required_tags(&w, g, 0)[Tag::Industry as usize] - cards::tag_counts(&w, g, 0)[Tag::Industry as usize];
    assert_eq!(extra(&g), 1, "printed on the Builder");
    g.vars.insert(board::track2_key(0).into(), 2);
    assert_eq!(extra(&g), 2, "revealed on its card-play track");
}

#[test]
fn a_capstone_counts_once_uncovered() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let discount = |g: &GameState| cards::passives(&w, g, 0).iter().any(|p| matches!(p, Op::BuildingDiscount { amount: 2 }));
    assert!(!discount(&g));
    g.vars.insert(board::track2_key(0).into(), 7);
    assert!(discount(&g));
}

#[test]
fn rush_build_pays_tags_and_no_credits() {
    // 119: Open Assembly pinned to r46's no energy, 1 insight and no needs.
    let (rules, mut g) = lab_actions_with(&["cards.open_assembly.energy=0", "cards.open_assembly.insight=1", "cards.open_assembly.needs=[]"]);
    builder_with_tags(&mut g);
    set_pile(&mut g, &cards::hand(0), &["open_assembly"]);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    set_res(&mut g, 0, Resource::Energy, 1);
    act(&rules, &mut g, power(&rules, 0, "Rush Build"));
    assert!(g.pile(&cards::built(0)).contains(&Key::new("open_assembly")));
    assert_eq!(g.resource(0, Resource::Credits), 0);
    assert_eq!(g.resource(0, Resource::Energy), 0, "r35: an Industry tag and an energy");
    assert!(g.resource(0, Resource::Insight) < 3, "its insight is still paid");
    assert_eq!(g.pile(&cards::played(0)).len(), 2, "one tag went (r35), two cards stay");
}

/// 099: Rush Build isn't offered when its own sacrifice or energy leaves no
/// building to play (it took both and built nothing).
#[test]
fn rush_build_needs_a_building_left_once_paid() {
    // Open Assembly pinned to 4 energy: no building prints energy and no needs since r42.
    // Vertical Monopoly pinned to needs Industry 4 (r43 printed 3).
    // 119: Market Stabilization Board pinned to r46's needs Industry 2 and no energy.
    let (rules, mut g) = lab_actions_with(&[
        "cards.open_assembly.energy=4",
        r#"cards.vertical_monopoly.needs=[["industry",4]]"#,
        r#"cards.market_stabilization_board.needs=[["industry",2]]"#,
        "cards.market_stabilization_board.energy=0",
    ]);
    let rush = power(&rules, 0, "Rush Build");
    let offered = |g: &GameState| rules.legal_actions(g).contains(&rush);
    builder_with_tags(&mut g);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    set_res(&mut g, 0, Resource::Energy, 1);
    // Printed Industry + 3 programs = 4: Vertical Monopoly needs all 4, so the
    // sacrifice of any one program leaves it short.
    set_pile(&mut g, &cards::hand(0), &["vertical_monopoly"]);
    assert!(!offered(&g), "every sacrifice leaves Industry 3 of the 4 it needs");
    set_pile(&mut g, &cards::hand(0), &["market_stabilization_board"]);
    assert!(offered(&g), "Industry 3 is left after the sacrifice, of the 2 it needs");
    // Open Assembly's 4 energy, after Rush Build's 1.
    set_pile(&mut g, &cards::hand(0), &["open_assembly"]);
    set_res(&mut g, 0, Resource::Energy, 4);
    assert!(!offered(&g), "4 energy pays Rush Build or the card, not both");
    set_res(&mut g, 0, Resource::Energy, 5);
    assert!(offered(&g));
}

/// 099: Rush Build's sacrifice offers only what leaves a building to play: the
/// stack Capital Projects Office would go over is not offered while a program can go.
#[test]
fn rush_build_offers_no_sacrifice_that_strands_it() {
    // Capital Projects Office pinned to built over Industry (r43 dropped it).
    // 119: and to r46's 1 insight, no energy and no needs.
    let (rules, mut g) = lab_actions_with(&[
        r#"cards.capital_projects_office.replaces="industry""#,
        "cards.capital_projects_office.insight=1",
        "cards.capital_projects_office.energy=0",
        "cards.capital_projects_office.needs=[]",
    ]);
    set_pile(&mut g, &cards::played(0), &["payroll_automation"]);
    set_pile(&mut g, &cards::built(0), &["retained_earnings"]);
    set_pile(&mut g, &cards::hand(0), &["capital_projects_office"]);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    set_res(&mut g, 0, Resource::Chips, 2);
    set_res(&mut g, 0, Resource::Energy, 1);
    // Not `act`: it answers every prompt itself. With the stack filtered out one
    // source is left, so the sacrifice asks nothing.
    rules.apply_mut(&mut g, &power(&rules, 0, "Rush Build"), None).unwrap();
    for _ in 0..4 {
        let Some(p) = g.pending.as_ref() else { break };
        assert!(
            !p.ask.prompt.contains("sacrifice") || p.ask.options.iter().all(|o| !o.option.contains("retained_earnings")),
            "the stack is not offered: {:?}",
            p.ask.options
        );
        answer(&rules, &mut g, 0);
    }
    assert!(g.pile(&cards::built(0)).contains(&Key::new("capital_projects_office")), "{:?}", g.pile(&cards::built(0)));
}

#[test]
fn a_cordon_stops_creation_for_the_next_build() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let c = 0;
    let line = plan::CreateLine { count: 1, at: CreateAt::City, if_: CreateIf::Any, sites: 1 };
    let lands = |g: &GameState| !plan::create_plan(&w, g, line, Some(c), &[], None).is_empty();
    assert!(lands(&g));
    g.vars.insert(board::flag(RoundFlag::Cordon, c).into(), 2);
    assert!(!lands(&g));
}

