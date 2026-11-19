//! 068: the Builder and Protector boards, and Cordon.

use super::*;
use crate::content::{Op, RoundFlag, Tag};
use crate::modules::{board, cards, plan};
use crate::op::{CreateAt, CreateIf};

/// Three Industry programs in the Builder's play area.
fn builder_with_tags(g: &mut GameState) {
    set_pile(g, &cards::played(0), &["payroll_automation", "supply_chain_audit", "vendor_financing"]);
}

/// 150: one tag rule. A printed tag (150) and a revealed one (149) count like
/// any tag on your cards, but neither can be sacrificed: a sacrifice discards a card.
#[test]
fn printed_and_revealed_tags_count_like_card_tags() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    for p in [cards::played(0), cards::built(0)] {
        set_pile(&mut g, &p, &[]);
    }
    let industry = |g: &GameState| cards::tag_counts(&w, g, 0)[Tag::Industry as usize];
    assert_eq!(industry(&g), 1, "printed on the Builder, counted with no card in play");
    assert!(cards::sources(&w, &g, 0).is_empty(), "a board tag is nothing to sacrifice");
    let cyber = crate::modules::ai::board(&w, 0).play_track.iter().position(|u| u.tags.contains(&Tag::Cyber)).unwrap() + 1;
    let count = |g: &GameState| cards::tag_counts(&w, g, 0)[Tag::Cyber as usize];
    set_var(&mut g, board::track2_key(0), cyber as i64 - 1);
    let before = count(&g);
    set_var(&mut g, board::track2_key(0), cyber as i64);
    assert_eq!(count(&g), before + 1, "revealed on its card-play track, counted as a card's");
    assert!(cards::sources(&w, &g, 0).is_empty(), "nor is a revealed one");
}

/// 149: Movement is the board's printed moves plus its uncovered Movement
/// spaces; each free move uses one this round, and cleanup gives them back.
#[test]
fn movement_is_printed_plus_uncovered_spaces() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let b = crate::modules::ai::board(&w, 0);
    assert_eq!(crate::modules::ai::movement(&w, &g, 0), b.moves + b.economy_track[0].moves);
    let k = b.economy_track.iter().position(|u| u.moves > 0).expect("a Movement space");
    set_var(&mut g, board::track_key(0), k as i64 + 1);
    assert_eq!(crate::modules::ai::movement(&w, &g, 0), b.moves + b.economy_track[..=k].iter().map(|u| u.moves).sum::<i64>());
    set_var(&mut g, board::moved_key(0), 1);
    assert_eq!(board::moves_left(&w, &g, 0), crate::modules::ai::movement(&w, &g, 0) - 1);
}

#[test]
fn a_capstone_counts_once_uncovered() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let discount = |g: &GameState| cards::passives(&w, g, 0).iter().any(|p| matches!(p, Op::BuildingDiscount { amount: 1 }));
    assert!(!discount(&g));
    set_var(&mut g, board::track2_key(0), 7);
    assert!(discount(&g));
}

/// 276: Scrap II (Rush Build's job) sacrifices 2 Industry for a chip and a building played paying no credits.
#[test]
fn rush_build_pays_tags_and_no_credits() {
    // 119: Open Assembly pinned to r46's no energy, 1 insight and no needs.
    let (rules, mut g) = lab_actions_with(&["cards.open_assembly.energy=0", "cards.open_assembly.insight=1", "cards.open_assembly.needs=[]"]);
    builder_with_tags(&mut g);
    set_pile(&mut g, &cards::hand(0), &["open_assembly"]);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Chips, 0);
    act(&rules, &mut g, power(&rules, 0, "Scrap II"));
    assert!(g.pile(&cards::built(0)).contains(&Key::new("open_assembly")));
    assert_eq!(g.resource(0, Resource::Credits), 0);
    assert_eq!(g.resource(0, Resource::Chips), 1, "and a chip");
    assert!(g.resource(0, Resource::Insight) < 3, "its insight is still paid");
    assert_eq!(g.pile(&cards::played(0)).len(), 1, "two Industry tags went, one card stays");
}

/// 099: Scrap II (Rush Build before 276) isn't offered when its own sacrifice or energy leaves no
/// building to play (it took both and built nothing); 276: its 2-tag sacrifice is tried whole.
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
    let rush = power(&rules, 0, "Scrap II");
    let offered = |g: &GameState| rules.legal_actions(g).contains(&rush);
    builder_with_tags(&mut g);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    set_res(&mut g, 0, Resource::Energy, 4);
    // Printed Industry + 3 programs = 4: Vertical Monopoly needs all 4, so the
    // sacrifice of any two programs leaves it short.
    set_pile(&mut g, &cards::hand(0), &["vertical_monopoly"]);
    assert!(!offered(&g), "every sacrifice leaves Industry 2 of the 4 it needs");
    set_pile(&mut g, &cards::hand(0), &["market_stabilization_board"]);
    assert!(offered(&g), "Industry 2 is left after the sacrifice, of the 2 it needs");
    // Open Assembly's 4 energy: Scrap II costs none.
    set_pile(&mut g, &cards::hand(0), &["open_assembly"]);
    set_res(&mut g, 0, Resource::Energy, 3);
    assert!(!offered(&g), "3 energy doesn't pay the card");
    set_res(&mut g, 0, Resource::Energy, 4);
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
    // 276: Scrap II sacrifices 2 Industry: two programs and the stack, of which only the programs pay it safely.
    set_pile(&mut g, &cards::played(0), &["payroll_automation", "supply_chain_audit"]);
    set_pile(&mut g, &cards::built(0), &["retained_earnings"]);
    set_pile(&mut g, &cards::hand(0), &["capital_projects_office"]);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    set_res(&mut g, 0, Resource::Chips, 2);
    set_res(&mut g, 0, Resource::Energy, 1);
    // Not `act`: it answers every prompt itself. With the stack filtered out the
    // programs are left.
    rules.apply_mut(&mut g, &power(&rules, 0, "Scrap II"), None).unwrap();
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
    set_var(&mut g, board::flag(RoundFlag::Cordon, c), 2);
    assert!(!lands(&g));
}

