//! 068: the Builder and Protector boards, and Cordon.

use super::*;
use crate::content::{Op, RoundFlag, Tag};
use crate::modules::{board, cards, plan};
use crate::op::Site;

/// Three Industry programs in the Builder's play area.
fn builder_with_tags(g: &mut GameState) {
    set_pile(g, &cards::played(0), &["payroll_automation", "materials_hedge_desk", "bulk_silicon_procurement"]);
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
    let (rules, mut g) = lab_actions();
    builder_with_tags(&mut g);
    set_pile(&mut g, &cards::hand(0), &["liaison_corps"]);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    act(&rules, &mut g, power(&rules, 0, "Rush Build"));
    assert!(g.pile(&cards::built(0)).contains(&Key::new("liaison_corps")));
    assert_eq!(g.resource(0, Resource::Credits), 0);
    assert!(g.resource(0, Resource::Insight) < 3, "its insight is still paid");
    assert!(g.pile(&cards::played(0)).is_empty(), "three tags went");
}

#[test]
fn a_cordon_stops_arrivals_for_the_next_build() {
    let (rules, mut g) = lab_actions_with(&["adversary.arrivals=1"]);
    let w = rules.world();
    let site = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial).unwrap();
    let c = w.map.hexes[site].country as usize;
    let op = Op::Arrivals { site: Site::Industrial };
    let lands = |g: &GameState| plan::build_plan(&w, g, &[&op], Some(c)).iter().any(|(h, _)| *h == site);
    assert!(lands(&g));
    g.vars.insert(board::flag(RoundFlag::Cordon, c).into(), 2);
    assert!(!lands(&g));
}

