//! 075: the Researcher board, Patience, bought cubes, free stations, Wall's strikes.

use super::*;
use crate::content::Op;
use crate::content::Tag;
use crate::modules::{ai, board, cards, economy};

/// The lab at three countries: Builder, Protector, Researcher (seat 2).
fn c3() -> (Rules, GameState) {
    lab_actions_with(&["countries=3"])
}

fn researcher_power(r: &Rules, name: &str) -> Action {
    power(r, 2, name)
}

#[test]
fn c2_seats_builder_and_protector_and_c3_adds_the_researcher() {
    let (rules, _) = lab_actions();
    let w = rules.world();
    assert_eq!((ai::board(&w, 0).id.as_str(), ai::board(&w, 1).id.as_str()), ("builder", "protector"));
    let (rules, _) = c3();
    let w = rules.world();
    assert_eq!(ai::board(&w, 2).id, "researcher");
    // Its slots are the baseline's `ai.slots` entry for the Researcher board, whichever seat plays it.
    let base = Rules::new(&baseline()).unwrap();
    let bw = base.world();
    let slots = bw.list("ai.slots");
    let i = bw.content.ais.iter().position(|b| b.id == "researcher").unwrap();
    assert_eq!(ai::slots(&w, 2), slots[i % slots.len()]);
}

#[test]
fn patience_carries_insight_with_no_cap() {
    let (rules, mut g) = c3();
    let w = rules.world();
    let inc = economy::income(&w, &g, 2)[Resource::Insight];
    for have in [3, 30] {
        set_res(&mut g, 2, Resource::Insight, have);
        set_res(&mut g, 0, Resource::Insight, have);
        next_round(&rules, &mut g);
        assert_eq!(g.resource(2, Resource::Insight), inc + have, "the Researcher keeps what it left");
        assert_eq!(g.var(&economy::insight_carried(2)), have);
        assert_eq!(g.resource(0, Resource::Insight), economy::income(&w, &g, 0)[Resource::Insight], "anyone else's expires");
    }
}

#[test]
fn a_bought_cube_goes_on_an_incomplete_project_and_pays_at_completion_only() {
    let (rules, mut g) = c3();
    let w = rules.world();
    set_res(&mut g, 2, Resource::Credits, 10);
    act(&rules, &mut g, researcher_power(&rules, "Grant"));
    assert_eq!(g.resource(2, Resource::Credits), 7);
    let on: i64 = (0..w.map.countries.len()).map(|c| g.var(&board::project_key(c, Some(2)))).sum();
    assert_eq!(on, 1);
    for c in 0..w.map.countries.len() {
        g.vars.insert(board::project_done(c).into(), 1);
    }
    assert!(!rules.legal_actions(&g).contains(&researcher_power(&rules, "Grant")), "no project left to take it");
}

#[test]
fn the_second_node_pays_cubes_and_no_energy_but_the_step() {
    let (rules, mut g) = c3();
    set_res(&mut g, 2, Resource::Energy, 0);
    // 212: Peer Review costs 1 research cube (3 before).
    set_res(&mut g, 2, Resource::ResearchCubes, 0);
    assert!(!rules.legal_actions(&g).contains(&researcher_power(&rules, "Peer Review")));
    set_res(&mut g, 2, Resource::ResearchCubes, 1);
    assert!(rules.legal_actions(&g).contains(&researcher_power(&rules, "Peer Review")));
}

#[test]
fn the_industry_power_builds_a_station_for_no_chips() {
    let (rules, mut g) = c3();
    set_pile(&mut g, &cards::played(2), &["payroll_automation"]);
    set_res(&mut g, 2, Resource::Chips, 0);
    let w = rules.world();
    let sites = board::station_sites(&w, &g, 2);
    assert!(!sites.is_empty());
    let before = sites[0];
    act(&rules, &mut g, researcher_power(&rules, "Field Station"));
    assert!(g.pile(&cards::played(2)).is_empty(), "the Industry card went");
    assert_eq!(g.tokens(before, cube(2)), 1);
    assert_eq!(g.tokens(before, STATION), 1);
}

#[test]
fn the_track_space_builds_a_free_station() {
    let (rules, _) = c3();
    let w = rules.world();
    let b = ai::board(&w, 2);
    let spaces = b.economy_track.iter().filter(|u| u.effects == [Op::BuildStation]).count();
    assert_eq!(spaces, 1);
}

#[test]
fn power_costs_never_mix_tags() {
    let (rules, _) = lab_actions();
    let w = rules.world();
    for b in &w.content.ais {
        for p in &b.powers {
            assert!(p.tags.len() <= 1, "{}", p.name);
        }
    }
}

#[test]
fn no_researcher_power_sacrifices_science() {
    let (rules, _) = c3();
    let w = rules.world();
    let b = ai::board(&w, 2);
    assert!(b.powers.iter().all(|p| p.tags.iter().all(|(t, _)| !t.accepts(Tag::Science))));
}

#[test]
fn the_giving_capstone_draws_from_the_supply() {
    let (rules, mut g) = lab_with(&["economy.upkeep"], &["countries=3"]);
    let w = rules.world();
    g.vars.insert(board::track2_key(2).into(), 7);
    set_res(&mut g, 2, Resource::Insight, 5);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.var(&economy::insight_gift(0)), 2, "two picks of the first option");
    assert_eq!(g.resource(2, Resource::Insight), 5, "the Researcher's bank pays nothing");
    let before = economy::income(&w, &g, 0)[Resource::Insight];
    let mut h = g.clone();
    next_round(&rules, &mut h);
    assert!(h.resource(0, Resource::Insight) >= before, "it lands in the next round's insight");
}

#[test]
fn the_new_cards_can_be_played() {
    // 119: both prices and the repository's need pinned to r46's print, under what is given.
    let (rules, mut g) = lab_actions_with(&[
        "countries=3",
        "cards.sample_repository.cost=8",
        "cards.sample_repository.insight=0",
        r#"cards.sample_repository.needs=[["science",2]]"#,
        "cards.forensic_audit.cost=27",
        "cards.forensic_audit.research_cubes=4",
        "cards.forensic_audit.insight=1",
    ]);
    // The cube cards (r42): one raises cube income, one pays cubes.
    set_pile(&mut g, &cards::played(0), &["shared_compute_grant", "dataset_scraper"]);
    // Forensic Audit liberates: a corrupted city in reach.
    corrupt_cities_except(&rules.map, &mut g, &[]);
    for id in ["sample_repository", "forensic_audit"] {
        set_pile(&mut g, &cards::hand(0), &[id]);
        set_res(&mut g, 0, Resource::Credits, 30);
        set_res(&mut g, 0, Resource::Insight, 5);
        set_res(&mut g, 0, Resource::ResearchCubes, 4);
        let legal = rules.legal_actions(&g);
        assert!(legal.iter().any(|a| matches!(a, Action::Play { card, .. } if card.as_str() == id)), "{id}: {legal:?}");
    }
}

/// r82 (Matt, 2026-11-11): Patience keeps all the Researcher's unspent insight, what another player's card
/// gave it this round included (r48 to r81 it expired).
#[test]
fn patience_banks_insight_another_player_gave() {
    let (rules, mut g) = c3();
    let w = rules.world();
    let inc = economy::income(&w, &g, 2)[Resource::Insight];
    set_res(&mut g, 0, Resource::Credits, 30);
    set_res(&mut g, 2, Resource::Insight, 3);
    // Seat 0's help card gives the Researcher 4 insight.
    let gift = [Op::ChoosePlayer, Op::OtherGain { resource: Resource::Insight, amount: 4 }];
    with_cx_picks(&rules, vec![crate::state::Pick { option: "seat 2".into(), target: None }], |cx| {
        crate::modules::effects::run(&mut g, cx, &gift, &mut crate::modules::effects::Ctx::seat(0))
    })
    .unwrap();
    assert_eq!(g.resource(2, Resource::Insight), 7);
    next_round(&rules, &mut g);
    assert_eq!(g.resource(2, Resource::Insight), inc + 7, "all 7 carried, the gift too");
    assert_eq!(g.var(&economy::insight_carried(2)), 7);
}
