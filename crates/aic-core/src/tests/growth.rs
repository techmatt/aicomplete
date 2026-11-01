//! 041: growth as a choice, the one payment path, the marker's reward, Science
//! on the board, and the round cap's shape by difficulty.

use super::*;
use crate::content::Op;
use crate::modules::common::{hex_opt, is_ocean, presence, HARDEN, MARKER};
use crate::modules::{ai, alignment, board, effects, threats};
use crate::state::Pick;

/// A lab with seat 0's presence on the board and both tracks at `taken` steps.
pub(super) fn at_step(sets: &[&str], taken: i64) -> (Rules, GameState) {
    let (rules, mut g) = lab_with(&[], sets);
    let a = rules.map.numbered(0, piece(0).presence);
    put(&mut g, a, presence(0), 1);
    g.vars.insert(board::track_key(0).into(), taken);
    g.vars.insert(board::track2_key(0).into(), taken);
    (rules, g)
}

pub(super) fn run_ops(rules: &Rules, g: &mut GameState, ops: &[content::Op]) {
    let mut seed = g.clone();
    with_cx(rules, |cx| {
        effects::run(&mut seed, cx, ops, &mut effects::Ctx::seat(0)).unwrap();
    });
    *g = seed;
}

#[test]
fn a_card_pays_the_step_and_a_free_one_does_not() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let b = ai::board(&w, 0);
    let step = b.presence[5].cost.min(b.presence2[5].cost);
    assert!(step > 0);
    let paying = [content::Op::PlacePresence { count: 1, free: false }];
    set_res(&mut g, 0, Resource::Energy, step - 1);
    assert!(!effects::feasible(&w, &g, 0, &paying), "the step can't be paid");
    let free = [content::Op::PlacePresence { count: 1, free: true }];
    assert!(effects::feasible(&w, &g, 0, &free));
    assert_eq!(
        board::placing_price(&w, &g, 0, board::Placing::Action, step).energy,
        step + w.int("board.presence_energy")
    );
    assert_eq!(
        board::placing_price(&w, &g, 0, board::Placing::Rich, step).credits,
        w.int("board.presence_credits") + step * w.int("board.energy_credits")
    );
}

#[test]
fn up_to_two_markers_can_stop_after_one() {
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let c = rules.map.numbered(0, piece(0).presence);
    let n: Vec<usize> = w.map.hexes[c].neighbors.iter().copied().filter(|&h| !crate::modules::common::is_ocean(&w, h)).take(2).collect();
    put(&mut g, c, presence(0), 1);
    for &h in &n {
        put(&mut g, h, MARKER, 1);
    }
    let first = hex_opt(&w, n[0]);
    let picks = vec![Pick { option: first.option, target: first.target }, Pick { option: "stop".into(), target: None }];
    let mut after = g.clone();
    with_cx_picks(&rules, picks, |cx| {
        effects::run(&mut after, cx, &[Op::ClearMarkers { count: 2 }], &mut effects::Ctx::seat(0)).unwrap();
    });
    assert_eq!(n.iter().map(|&h| after.tokens(h, MARKER)).sum::<i64>(), 1, "the second pick was stop");
}

#[test]
fn a_placements_prompt_names_what_it_charges() {
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).presence);
    put(&mut g, a, presence(0), 1);
    g.vars.insert(board::track_key(0).into(), 5);
    g.vars.insert(board::track2_key(0).into(), 5);
    let step = crate::modules::ai::board(&w, 0).presence[5].cost;
    assert!(step > 0);
    set_res(&mut g, 0, Resource::Energy, 20);
    set_res(&mut g, 0, Resource::Credits, 50);
    let prompt_for = |ops: &[Op]| {
        let mut gg = g.clone();
        let mut choices = module::Choices::new(Vec::new());
        let mut cx = module::Cx { world: rules.world(), module: "test", purposes: &[], events: None, choices: &mut choices };
        let _ = effects::run(&mut gg, &mut cx, ops, &mut effects::Ctx::seat(0));
        let ask = choices.asked.take().expect("two tracks to choose from");
        ask.options.iter().map(|o| o.option.clone()).collect::<Vec<_>>().join(" | ")
    };
    let free = prompt_for(&[Op::PlacePresence { count: 1, free: true }]);
    assert!(!free.contains("costs"), "{free}");
    let paid = prompt_for(&[Op::PlacePresence { count: 1, free: false }]);
    assert!(paid.contains(&format!("costs {step} energy")), "{paid}");
}

#[test]
fn every_growth_option_can_be_taken_and_expand_places_what_it_can_pay() {
    let (rules, g) = lab_with(&[], &[]);
    let w = rules.world();
    for seat in 0..g.seats.len() {
        let places: Vec<i64> = ai::board(&w, seat)
            .growth
            .iter()
            .map(|o| {
                o.effects
                    .iter()
                    .map(|e| if let content::Op::PlacePresence { count, .. } = e { *count } else { 0 })
                    .sum()
            })
            .collect();
        assert_eq!(places, [2, 1, 0], "Expand, Develop, Consolidate");
    }
}

#[test]
fn clearing_a_marker_costs_its_energy_and_pays_its_reward() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).presence);
    let marked = w.map.hexes[a].neighbors.iter().copied().find(|&h| !is_ocean(&w, h)).unwrap();
    put(&mut g, marked, MARKER, 1);
    assert!(!board::presence_hexes(&w, &g, 0).contains(&marked), "no presence on a marker");
    set_res(&mut g, 0, Resource::Energy, w.int("board.clear_energy"));
    let credits = g.resource(0, Resource::Credits);
    let unity = g.var(alignment::bucket(content::Side::Unity));
    rules.apply_mut(&mut g, &std_act(0, "clear"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(marked, MARKER), 0);
    assert_eq!(g.resource(0, Resource::Energy), 0);
    assert_eq!(g.resource(0, Resource::Credits), credits + w.int("board.clear_credits"));
    assert_eq!(g.var(alignment::bucket(content::Side::Unity)), unity + w.int("board.clear_unity"));
}

#[test]
fn energy_a_card_gains_first_can_pay_its_step_and_a_second_delay_is_not_offered() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let b = ai::board(&w, 0);
    let step = b.presence[5].cost.min(b.presence2[5].cost);
    set_res(&mut g, 0, Resource::Energy, 0);
    let gains = [
        content::Op::Gain { resource: Resource::Energy, amount: step },
        content::Op::PlacePresence { count: 1, free: false },
    ];
    assert!(effects::feasible(&w, &g, 0, &gains));
    let delay = [content::Op::DelayThreat { count: 1 }];
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    assert!(effects::feasible(&w, &g, 0, &delay));
    run_ops(&rules, &mut g, &delay);
    assert!(!effects::feasible(&w, &g, 0, &delay), "every threat already waits");
}

#[test]
fn a_shield_goes_with_a_token_lost_another_way() {
    let (rules, mut g) = at_step(&[], 1);
    let a = rules.map.numbered(0, piece(0).presence);
    put(&mut g, a, HARDEN, 1);
    g.vars.insert(crate::modules::adversary::STAGE.into(), 2);
    with_cx(&rules, |cx| board::lose_presence(&mut g, cx, a, 0, "test"));
    assert_eq!(g.tokens(a, HARDEN), 0);
}

#[test]
fn the_standard_placement_is_once_a_round_and_the_cap_can_be_off() {
    for (sets, second) in [(&[][..], false), (&["board.presence_action_cap=0"][..], true)] {
        let (rules, mut g) = at_step(sets, 1);
        set_res(&mut g, 0, Resource::Energy, 9);
        rules.apply_mut(&mut g, &std_act(0, "presence"), None).unwrap();
        answer_all(&rules, &mut g);
        let again = rules.apply_mut(&mut g, &std_act(0, "presence"), None);
        assert_eq!(again.is_ok(), second, "{sets:?}");
    }
}

#[test]
fn shields_delays_and_marker_clearing_are_refused_off_science() {
    for op in ["shield_presence", "delay_threat", "clear_markers"] {
        let set = format!("cards.payroll_automation.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        let err = Rules::new(&baseline_with(&[&set])).err().expect("refused");
        assert!(err.contains("Science"), "{err}");
        let on_science = format!("cards.literature_crawler.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        assert!(Rules::new(&baseline_with(&[&on_science])).is_ok());
    }
}

#[test]
fn develop_can_reshuffle_the_tier_1_deck() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(1);
    let mut legal = Vec::new();
    loop {
        rules.legal_actions_into(&g, &mut legal);
        if legal.iter().any(|a| matches!(a, Action::Growth { .. })) {
            break;
        }
        rules.apply_mut(&mut g, &legal[0], None).unwrap();
    }
    let seat = legal[0].seat();
    let hand = crate::modules::cards::hand(seat as usize);
    let held = g.pile(&hand).len();
    let deck = g.pile("cards.deck.1").to_vec();
    g.piles.insert("cards.deck.1".into(), Vec::new().into());
    g.piles.insert("cards.discard.1".into(), deck.clone().into());
    act(&rules, &mut g, Action::Growth { seat, option: 1 });
    assert_eq!(g.pile(&hand).len(), held + 1);
    assert_eq!(g.pile("cards.deck.1").len(), deck.len() - 1);
}
