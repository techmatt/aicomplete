//! The node tracks, the node powers (066) and the one payment path (041).

use super::*;
use crate::content::{Op, Tag};
use crate::modules::common::node_count;
use crate::modules::{ai, board, cards, effects};

/// A tier-1 program carrying `tag`.
fn program(w: &module::World, tag: Tag) -> Key {
    let found = programs(w, tag).find(|c| c.tier == 1);
    Key::new(&found.expect("a program").id)
}

/// r81 (friction #16): every placement pays the space; no card places a node free.
#[test]
fn a_card_pays_the_step() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let step = step_cost(&w, 5);
    assert!(step > 0);
    let paying = [Op::PlaceNode { count: 1 }];
    set_res(&mut g, 0, Resource::Energy, step - 1);
    assert!(!effects::feasible(&w, &g, 0, &paying), "the step can't be paid");
    set_res(&mut g, 0, Resource::Energy, step);
    assert!(effects::feasible(&w, &g, 0, &paying));
    // The op has no "free" any more: content printing one is refused (`deny_unknown_fields`).
    assert!(serde_json::from_str::<Op>(r#"{"op": "place_node", "count": 1, "free": true}"#).is_err());
}

#[test]
fn a_placements_prompt_names_what_it_charges() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let step = ai::board(&w, 0).economy_track[5].cost;
    assert!(step > 0);
    set_res(&mut g, 0, Resource::Energy, 20);
    set_res(&mut g, 0, Resource::Credits, 50);
    let prompt_for = |ops: &[Op]| {
        let mut gg = g.clone();
        let mut choices = module::Choices::new(Vec::new());
        let mut cx = test_cx(&rules, &mut choices);
        let _ = effects::run(&mut gg, &mut cx, ops, &mut effects::Ctx::seat(0));
        let ask = choices.asked.take().expect("two tracks to choose from");
        ask.options.iter().map(|o| o.option.clone()).collect::<Vec<_>>().join(" | ")
    };
    let paid = prompt_for(&[Op::PlaceNode { count: 1 }]);
    assert!(paid.contains(&format!("costs {step} energy")), "{paid}");
}

/// 206 (Matt, 2026-11-12): Expand left every board; the standard Place a node costs `node_credits` and the
/// space, `node_per_round` times a round.
#[test]
fn place_a_node_costs_its_credits_and_the_space_a_limited_number_of_times_a_round() {
    for seat in [0u8, 1] {
        let (rules, mut g) = at_step(&[], 1);
        let w = rules.world();
        assert!(ai::board(&w, seat as usize).powers.iter().all(|p| p.name != "Expand"));
        let start = rules.map.numbered(seat as usize, piece(seat as usize).node);
        put(&mut g, start, node(seat as usize), 1);
        let deploy = std_act(seat, "node");
        let price = w.int("board.node_credits");
        set_res(&mut g, seat as usize, Resource::Credits, price - 1);
        assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "its credits");
        set_res(&mut g, seat as usize, Resource::Credits, 10 * price);
        set_res(&mut g, seat as usize, Resource::Energy, 10);
        for _ in 0..w.int("board.node_per_round") {
            let before = node_count(&g, seat as usize);
            rules.apply_mut(&mut g, &deploy, None).unwrap();
            answer_all(&rules, &mut g);
            assert_eq!(node_count(&g, seat as usize), before + 1);
        }
        assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "its limit a round");
    }
}

/// 206: a placement pays the step's energy beside Place a node's credits.
#[test]
fn a_node_power_pays_its_energy_and_the_step() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let step = step_cost(&w, 5);
    let deploy = std_act(0, "node");
    set_res(&mut g, 0, Resource::Credits, w.int("board.node_credits"));
    set_res(&mut g, 0, Resource::Energy, step - 1);
    assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "the step's energy");
    set_res(&mut g, 0, Resource::Energy, step);
    rules.apply_mut(&mut g, &deploy, None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Energy), 0);
}

#[test]
fn the_builder_s_second_node_power_takes_its_credits() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let buy = power(&rules, 0, "Buy In");
    let price = ai::board(&w, 0).powers[power_index(&rules, 0, "Buy In")].cost.iter().find(|(r, _)| *r == Resource::Credits).unwrap().1;
    set_res(&mut g, 0, Resource::Energy, 1);
    set_res(&mut g, 0, Resource::Credits, price - 1);
    assert!(rules.apply_mut(&mut g.clone(), &buy, None).is_err());
    set_res(&mut g, 0, Resource::Credits, price);
    rules.apply_mut(&mut g, &buy, None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!((g.resource(0, Resource::Energy), g.resource(0, Resource::Credits)), (1, 0), "Buy In pays credits only");
}

#[test]
fn the_protector_s_second_node_power_sacrifices_diplomacy() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let start = rules.map.numbered(1, piece(1).node);
    put(&mut g, start, node(1), 1);
    let accord = power(&rules, 1, "Accord");
    set_res(&mut g, 1, Resource::Energy, 1);
    let program = program(&w, Tag::Diplomacy);
    assert!(rules.apply_mut(&mut g.clone(), &accord, None).is_err(), "no tag");
    g.piles.insert(cards::played(1).into(), Arc::new(vec![program]));
    rules.apply_mut(&mut g, &accord, None).unwrap();
    answer_all(&rules, &mut g);
    assert!(g.pile(&cards::played(1)).is_empty(), "sacrificed");
    assert_eq!(g.resource(1, Resource::Energy), 1, "Accord pays its tag only");
}

#[test]
fn the_cheap_powers_sacrifice_one_tag_for_chips_or_credits() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    g.piles.insert(cards::played(0).into(), Arc::new(vec![program(&w, Tag::Industry)]));
    g.piles.insert(cards::played(1).into(), Arc::new(vec![program(&w, Tag::Security)]));
    set_res(&mut g, 0, Resource::Chips, 0);
    set_res(&mut g, 1, Resource::Credits, 0);
    rules.apply_mut(&mut g, &power(&rules, 0, "Scrap"), None).unwrap();
    answer_all(&rules, &mut g);
    rules.apply_mut(&mut g, &power(&rules, 1, "Stand Down"), None).unwrap();
    answer_all(&rules, &mut g);
    // 190: Scrap pays 1 chip (a chip is worth 4 since r83).
    assert_eq!((g.resource(0, Resource::Chips), g.resource(1, Resource::Credits)), (1, 3));
    assert!(g.pile(&cards::played(0)).is_empty() && g.pile(&cards::played(1)).is_empty());
}

#[test]
fn tokens_of_a_placement_still_uncover() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    set_res(&mut g, 0, Resource::Energy, 1);
    set_res(&mut g, 0, Resource::Credits, w.int("board.node_credits"));
    let income = ai::track_income(&w, &g, 0).len();
    rules.apply_mut(&mut g, &std_act(0, "node"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(board::track_taken(&g, 0) + board::track2_taken(&g, 0), 3);
    assert_eq!(ai::track_income(&w, &g, 0).len(), income + 1);
}

#[test]
fn placing_node_takes_the_next_track_token_and_uncovers_its_income() {
    let (rules, mut g) = actions_with_room();
    let w = rules.world();
    let credit_income = |g: &GameState| {
        ai::track_income(&w, g, 0)
            .iter()
            .filter(|(r, _)| *r == Resource::Credits)
            .map(|(_, n)| n)
            .sum::<i64>()
    };
    assert_eq!(
        (board::track_taken(&g, 0), credit_income(&g), ai::keep2(&w, &g, 0)),
        (1, 0, false)
    );
    set_res(&mut g, 0, Resource::Energy, 5);
    let start = node_of(&g, 0)[0];
    // A seat 1 node next to it blocks that hex (one node a hex).
    let blocked = rules.map.hexes[start]
        .neighbors
        .iter()
        .copied()
        .find(|&h| kind(&rules.map, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, blocked, node(1), 1);
    let hexes = board::node_sites(&w, &g, 0);
    assert!(!hexes.contains(&blocked));
    assert!(hexes
        .iter()
        .all(|&h| rules.map.adjacent(h, start) && g.tokens(h, node(0)) == 0));
    // Place a node, which refreshes here: a test of the track, not of its limit.
    let place = |rules: &Rules, g: &mut GameState| {
        g.vars.insert(board::std_once_key(board::StandardAction::Node, 0).into(), 0);
        set_res(g, 0, Resource::Credits, 50);
        act(rules, g, std_act(0, "node"));
    };
    place(&rules, &mut g);
    // The 2nd space (the Builder's): +3 credits a round (078).
    assert_eq!(
        (board::track_taken(&g, 0), credit_income(&g), ai::keep2(&w, &g, 0)),
        (2, 3, false)
    );
    place(&rules, &mut g);
    // The keep upgrade is the card-play track's second space (068).
    assert_eq!((board::track_taken(&g, 0), ai::keep2(&w, &g, 0)), (3, false));
    g.vars.insert(board::track2_key(0).into(), 2);
    assert!(ai::keep2(&w, &g, 0));
    assert_eq!(node_of(&g, 0).len(), 3);
    // The tracks run out after their last spaces (021: both of them).
    g.vars
        .insert(board::track_key(0).into(), ai::board(&w, 0).economy_track.len() as i64);
    assert!(ai::can_place(&w, &g, 0));
    g.vars
        .insert(board::track2_key(0).into(), ai::board(&w, 0).play_track.len() as i64);
    assert!(!ai::can_place(&w, &g, 0));
}

#[test]
fn a_track_s_last_steps_cost_more() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let b = ai::board(&w, 0);
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, node(0), 1);
    // Both tracks down to their last steps.
    g.vars.insert(board::track_key(0).into(), b.economy_track.len() as i64 - 1);
    g.vars.insert(board::track2_key(0).into(), b.play_track.len() as i64 - 1);
    let extra = b.economy_track.last().unwrap().cost.min(b.play_track.last().unwrap().cost);
    assert!(extra > 0);
    let deploy = std_act(0, "node");
    set_res(&mut g, 0, Resource::Credits, w.int("board.node_credits"));
    set_res(&mut g, 0, Resource::Energy, extra - 1);
    assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "the step can't be paid");
    set_res(&mut g, 0, Resource::Energy, extra);
    act(&rules, &mut g, deploy);
    assert_eq!(g.resource(0, Resource::Energy), 0);
}
