//! 293 (Matt, 2026-11-19): the adversary's research track: its sources, K by country, the row of icon slots (each
//! icon once, as the marker arrives), the loss past the last slot, and the players' removal.

use super::*;
use crate::content::Op;
use crate::modules::adversary::{self, RESEARCH, RESEARCH_SLOT};
use crate::modules::board;

/// The lab (no phases), K read back.
fn research_lab(sets: &[&str]) -> (Rules, GameState, i64) {
    let (rules, g) = lab_with(&[], sets);
    let at = adversary::research_threshold(&rules.world());
    (rules, g, at)
}

#[test]
fn k_is_a_country_and_zero_is_the_off_arm() {
    let (rules, g, at) = research_lab(&["adversary.difficulty=0"]);
    let w = rules.world();
    let n = w.map.countries.len() as i64;
    assert_eq!(at, w.list("adversary.research_at")[0] * n);
    assert_eq!(g.var(RESEARCH), w.int("adversary.research_start") * n, "the starting cubes, a country");
    // The off arm: all 0, never.
    let (rules, _, off) = research_lab(&["adversary.research_at=0,0,0,0,0"]);
    assert_eq!(off, 0);
    assert!(!adversary::research_on(&rules.world()));
    // Starting cubes are a setup number.
    let (rules, g, _) = research_lab(&["adversary.research_start=2"]);
    assert_eq!(g.var(RESEARCH), 2 * rules.world().map.countries.len() as i64);
}

/// Every board's row is content: six slots, each naming an icon of the board's vocabulary. (What its Garrison and
/// Strike icons do: `troops.rs`.)
#[test]
fn the_row_is_six_slots_of_the_boards_icons() {
    let b = &crate::content().adversary_board;
    let row = b.research.as_ref().expect("a research row");
    assert_eq!(row.slots.len(), crate::content::RESEARCH_SLOTS);
    for id in &row.slots {
        let icon = b.icons.iter().find(|i| &i.id == id).expect("a slot names an icon of the vocabulary");
        assert!(!icon.effects.is_empty(), "slot {id}: an icon does something");
    }
    assert!(b.rules.len() <= 1, "a board prints one rule");
}

#[test]
fn the_track_advances_a_slot_every_k_cubes_and_each_icon_happens_once() {
    let (rules, mut g, at) = research_lab(&["adversary.difficulty=0"]);
    let w = rules.world();
    set_var(&mut g, SUPPLY, 40);
    // Short of K: nothing.
    set_var(&mut g, RESEARCH, at - 1);
    with_cx(&rules, |cx| adversary::research_step(&mut g, cx)).unwrap();
    assert_eq!((g.var(RESEARCH), g.var(RESEARCH_SLOT)), (at - 1, 0));
    // K and 1: one slot, the rest kept. Slot 1 (New factory) opens a neutral factory in each country.
    let factories = total(&g, FACTORY);
    set_var(&mut g, RESEARCH, at + 1);
    with_cx(&rules, |cx| adversary::research_step(&mut g, cx)).unwrap();
    assert_eq!((g.var(RESEARCH), g.var(RESEARCH_SLOT)), (1, 1));
    assert_eq!(adversary::slot_icon(&w, 1).unwrap().id, "factory");
    assert!(total(&g, FACTORY) > factories, "the slot's icon happened");
    assert_eq!(g.stats.counters["adversary.research_fired"], 1);
    // Twice K: two slots in one check (slot 2: Infiltrate factories; slot 3: +1 compute a country).
    let income = g.var(adversary::INCOME_COMPUTE);
    set_var(&mut g, RESEARCH, 2 * at);
    with_answers(&rules, &mut g, |_| 0, adversary::research_step);
    assert_eq!((g.var(RESEARCH), g.var(RESEARCH_SLOT)), (0, 3));
    assert_eq!(g.var(adversary::INCOME_COMPUTE) - income, w.map.countries.len() as i64);
    assert!(g.outcome.is_none());
}

#[test]
fn slot_seven_loses_the_game() {
    let (rules, mut g, at) = research_lab(&[]);
    let len = adversary::row_len(&rules.world());
    assert_eq!(len, 6);
    set_var(&mut g, RESEARCH_SLOT, len);
    set_var(&mut g, RESEARCH, at);
    with_cx(&rules, |cx| adversary::research_step(&mut g, cx)).unwrap();
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "research"));
}

#[test]
fn a_removal_takes_cubes_off_and_never_below_zero() {
    let (rules, mut g, _) = research_lab(&[]);
    set_var(&mut g, RESEARCH, 3);
    set_var(&mut g, RESEARCH_SLOT, 2);
    let op = Op::RemoveResearch { amount: 4 };
    with_cx(&rules, |cx| crate::modules::effects::run(&mut g, cx, std::slice::from_ref(&op), &mut crate::modules::effects::Ctx::seat(0)))
        .unwrap();
    assert_eq!((g.var(RESEARCH), g.var(RESEARCH_SLOT)), (0, 2), "cubes come off, never a slot");
    assert_eq!(g.stats.counters["adversary.research_removed"], 3);
}

#[test]
fn a_corrupted_station_feeds_the_track_megaproject_open_or_done() {
    let (rules, mut g, _) = research_lab(&["adversary.research_at=9,9,9,9,9"]);
    let w = rules.world();
    let st = (0..g.hexes.len()).find(|&h| w.map.hexes[h].country == 0 && kind(w.map, h) != HexKind::City).unwrap();
    put(&mut g, st, STATION, 1);
    put(&mut g, st, ADV_CUBE, 1);
    let stations = (0..g.hexes.len()).filter(|&h| g.tokens(h, STATION) > 0 && g.tokens(h, ADV_CUBE) > 0).count() as i64;
    let per = stations * w.int("board.station_cubes");
    let before = g.var(RESEARCH);
    with_answers(&rules, &mut g, |_| 0, adversary::compute_step);
    assert_eq!(g.var(RESEARCH) - before, per, "megaproject open: from round 1");
    set_var(&mut g, board::project_done(0), 1);
    with_answers(&rules, &mut g, |_| 0, adversary::compute_step);
    assert_eq!(g.var(RESEARCH) - before, 2 * per);
}
