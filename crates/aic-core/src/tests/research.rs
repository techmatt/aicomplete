//! 259 (Matt, 2026-11-16): the adversary's research track: its sources, the threshold by difficulty, the firing
//! (once a round, the overflow kept), the stronger effect from `research_strong_at`, and the players' removal.

use super::*;
use crate::content::Op;
use crate::modules::adversary::{self, RESEARCH};
use crate::modules::board;

fn troops(g: &GameState) -> i64 {
    total(g, TROOP)
}

/// The lab (no phases) at `difficulty`, the threshold read back.
fn research_lab(sets: &[&str]) -> (Rules, GameState, i64) {
    let (rules, g) = lab_with(&[], sets);
    let at = adversary::research_threshold(&rules.world());
    (rules, g, at)
}

#[test]
fn the_threshold_is_a_country_and_falls_with_difficulty() {
    let (rules, _, at) = research_lab(&["adversary.difficulty=0"]);
    let w = rules.world();
    let n = w.map.countries.len() as i64;
    assert_eq!(at, w.list("adversary.research_at")[0] * n);
    let (_, _, hard) = research_lab(&["adversary.difficulty=4"]);
    assert!(hard < at, "d4 {hard} against d0 {at}");
    // The off arm: all 0, never.
    let (rules, _, off) = research_lab(&["adversary.research_at=0,0,0,0,0"]);
    assert_eq!(off, 0);
    assert!(!adversary::research_on(&rules.world()));
}

#[test]
fn a_full_track_fires_once_keeps_the_overflow_and_musters() {
    let (rules, mut g, at) = research_lab(&["adversary.difficulty=0"]);
    let w = rules.world();
    // A factory a country for the Muster to land on.
    for c in 0..w.map.countries.len() {
        let z = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial && w.map.hexes[h].country as usize == c).unwrap();
        put(&mut g, z, FACTORY, 1);
    }
    // Short of the threshold: nothing.
    set_var(&mut g, RESEARCH, at - 1);
    with_cx(&rules, |cx| adversary::research_step(&mut g, cx)).unwrap();
    assert_eq!((g.var(RESEARCH), troops(&g)), (at - 1, 0));
    // Twice the threshold and 1: it fires once and keeps the rest.
    set_var(&mut g, RESEARCH, 2 * at + 1);
    with_cx(&rules, |cx| adversary::research_step(&mut g, cx)).unwrap();
    assert_eq!(g.var(RESEARCH), at + 1);
    assert_eq!(g.stats.counters["adversary.research_fired"], 1);
    // The Militarist's Weapons Program: Muster 2 in each country (d0: no board difficulty line).
    assert_eq!(troops(&g), 2 * w.map.countries.len() as i64);
}

#[test]
fn the_stronger_effect_from_research_strong_at() {
    let (rules, mut g, at) = research_lab(&["adversary.difficulty=3"]);
    let w = rules.world();
    for c in 0..w.map.countries.len() {
        let z = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial && w.map.hexes[h].country as usize == c).unwrap();
        put(&mut g, z, FACTORY, 1);
    }
    set_var(&mut g, RESEARCH, at);
    with_cx(&rules, |cx| adversary::research_step(&mut g, cx)).unwrap();
    // Muster 3, and the board's difficulty line (from board_difficulty_at 2) adds its troop: a Muster.
    let per = 3 + adversary::muster_bonus(&w);
    assert_eq!((g.var(RESEARCH), troops(&g)), (0, per * w.map.countries.len() as i64));
}

#[test]
fn a_removal_takes_cubes_off_and_never_below_zero() {
    let (rules, mut g, _) = research_lab(&[]);
    set_var(&mut g, RESEARCH, 3);
    let op = Op::RemoveResearch { amount: 4 };
    with_cx(&rules, |cx| crate::modules::effects::run(&mut g, cx, std::slice::from_ref(&op), &mut crate::modules::effects::Ctx::seat(0)))
        .unwrap();
    assert_eq!(g.var(RESEARCH), 0);
    assert_eq!(g.stats.counters["adversary.research_removed"], 3);
}

#[test]
fn a_corrupted_station_feeds_the_track_once_its_megaproject_is_done() {
    let (rules, mut g, _) = research_lab(&[]);
    let w = rules.world();
    let st = (0..g.hexes.len()).find(|&h| w.map.hexes[h].country == 0 && kind(w.map, h) != HexKind::City).unwrap();
    put(&mut g, st, STATION, 1);
    put(&mut g, st, ADV_CUBE, 1);
    let compute = g.var(adversary::COMPUTE);
    // Open megaproject: the step makes neither (its cube went on the card in production).
    with_cx(&rules, |cx| adversary::compute_step(&mut g, cx)).unwrap();
    assert_eq!(g.var(RESEARCH), 0);
    set_var(&mut g, board::project_done(0), 1);
    let before = g.var(adversary::COMPUTE) - compute;
    with_cx(&rules, |cx| adversary::compute_step(&mut g, cx)).unwrap();
    assert_eq!(g.var(RESEARCH), w.int("board.station_cubes"));
    // No compute from it: only the income came (the lab's botnets were cleared).
    let income = g.var(adversary::INCOME_COMPUTE).max(0);
    assert_eq!(g.var(adversary::COMPUTE) - compute - before, income);
}
