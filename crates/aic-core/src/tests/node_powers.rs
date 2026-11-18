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
    let paying = [Op::PlaceNode { count: 1, range: 1 }];
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
    let paid = prompt_for(&[Op::PlaceNode { count: 1, range: 1 }]);
    assert!(paid.contains(&format!("costs {step} energy")), "{paid}");
}

/// 252 (Matt's ruling 1): every board's base route, Expand, costs only the track step's energy, once a round (255
/// deleted the cut Place a node standard action).
#[test]
fn the_base_route_costs_only_the_step_once_a_round() {
    for seat in [0u8, 1] {
        let (rules, mut g) = at_step(&[], 1);
        let w = rules.world();
        let start = rules.map.numbered(seat as usize, piece(seat as usize).node);
        put(&mut g, start, node(seat as usize), 1);
        let deploy = power(&rules, seat as usize, "Expand");
        set_res(&mut g, seat as usize, Resource::Credits, 0);
        set_res(&mut g, seat as usize, Resource::Energy, step_cost(&w, 1));
        let before = node_count(&g, seat as usize);
        rules.apply_mut(&mut g, &deploy, None).unwrap();
        answer_all(&rules, &mut g);
        assert_eq!(node_count(&g, seat as usize), before + 1);
        assert_eq!(g.resource(seat as usize, Resource::Energy), 0, "the step's energy alone");
        set_res(&mut g, seat as usize, Resource::Energy, 10);
        assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "once a round");
    }
}

/// 261: a node route is used for its node: with no free hex next to the seat's nodes it isn't offered (Outpost, the
/// Protector's, was paid in full for its ambassador alone), and a placement there asks no track question.
#[test]
fn a_node_route_with_no_free_hex_is_not_offered() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let seat = 1usize;
    let start = rules.map.numbered(seat, piece(seat).node);
    put(&mut g, start, node(seat), 1);
    set_res(&mut g, seat, Resource::Energy, 20);
    let outpost = power(&rules, seat, "Outpost");
    assert!(rules.apply_mut(&mut g.clone(), &outpost, None).is_ok(), "offered with a free hex");
    // Every hex in reach taken by seat 0's nodes: no free hex for seat 1's.
    for h in board::node_sites_within(&w, &g, seat, 1) {
        put(&mut g, h, node(0), 1);
    }
    assert!(board::node_sites_within(&w, &g, seat, 1).is_empty());
    assert!(rules.apply_mut(&mut g.clone(), &outpost, None).is_err(), "no free hex: not offered");
    let mut gg = g.clone();
    let mut choices = module::Choices::new(Vec::new());
    let mut cx = test_cx(&rules, &mut choices);
    let placed = board::place_node_within(&mut gg, &mut cx, seat, 1).unwrap();
    assert_eq!(placed, None);
    assert!(choices.asked.is_none(), "no track question for a node that can't go down");
}

/// 252 (Matt's ruling 1: "occasionally range 2"): Annex, the Builder's top route, places within 2 of a node.
#[test]
fn the_builder_s_top_route_reaches_two_hexes() {
    let (rules, g) = at_step(&[], 1);
    let w = rules.world();
    let near = board::node_sites(&w, &g, 0);
    let far = board::node_sites_within(&w, &g, 0, 2);
    assert!(far.len() > near.len() && near.iter().all(|h| far.contains(h)));
}

/// 206: a placement pays the step's energy (252: the base route, Expand, nothing more).
#[test]
fn a_node_power_pays_its_energy_and_the_step() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let step = step_cost(&w, 5);
    let deploy = power(&rules, 0, "Expand");
    set_res(&mut g, 0, Resource::Energy, step - 1);
    assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "the step's energy");
    set_res(&mut g, 0, Resource::Energy, step);
    rules.apply_mut(&mut g, &deploy, None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Energy), 0);
}

/// The energy a board's power prints in its cost (247: the node powers run on energy).
fn power_energy(rules: &Rules, seat: usize, name: &str) -> i64 {
    let w = rules.world();
    ai::board(&w, seat).powers[power_index(rules, seat, name)].cost.iter().find(|(r, _)| *r == Resource::Energy).unwrap().1
}

/// 247 (Matt, 2026-11-15: energy powers things): Buy In costs energy, not credits, beside the step's.
#[test]
fn the_builder_s_second_node_power_runs_on_energy() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let buy = power(&rules, 0, "Buy In");
    let need = power_energy(&rules, 0, "Buy In") + step_cost(&w, 1);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Energy, need - 1);
    assert!(rules.apply_mut(&mut g.clone(), &buy, None).is_err(), "its energy and the step's");
    set_res(&mut g, 0, Resource::Energy, need);
    rules.apply_mut(&mut g, &buy, None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!((g.resource(0, Resource::Energy), g.resource(0, Resource::Credits)), (0, 0), "Buy In pays energy only");
}

/// 247: Accord costs energy and sacrifices nothing (it took a Diplomacy card).
#[test]
fn the_protector_s_second_node_power_runs_on_energy() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let start = rules.map.numbered(1, piece(1).node);
    put(&mut g, start, node(1), 1);
    let accord = power(&rules, 1, "Accord");
    let price = power_energy(&rules, 1, "Accord");
    g.piles.insert(cards::played(1).into(), Arc::new(vec![program(&w, Tag::Diplomacy)]));
    set_res(&mut g, 1, Resource::Energy, price - 1);
    assert!(rules.apply_mut(&mut g.clone(), &accord, None).is_err(), "its energy");
    set_res(&mut g, 1, Resource::Energy, price);
    rules.apply_mut(&mut g, &accord, None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.pile(&cards::played(1)).len(), 1, "no sacrifice");
    assert_eq!(g.resource(1, Resource::Energy), 0, "Accord pays its energy");
}

/// 212: a troop power is force (a Security card for troops in reach); r98: Stand Down merged into Interdict.
#[test]
fn the_cheap_powers_sacrifice_one_tag_for_a_chip_or_a_troop() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    g.piles.insert(cards::played(0).into(), Arc::new(vec![program(&w, Tag::Industry)]));
    g.piles.insert(cards::played(1).into(), Arc::new(vec![program(&w, Tag::Security)]));
    set_res(&mut g, 0, Resource::Chips, 0);
    assert!(!rules.legal_actions(&g).contains(&power(&rules, 1, "Interdict")), "no troop in reach");
    let h = rules.map.numbered(0, piece(0).node);
    put(&mut g, h, node(1), 1);
    put(&mut g, h, TROOP, 1);
    rules.apply_mut(&mut g, &power(&rules, 0, "Scrap"), None).unwrap();
    answer_all(&rules, &mut g);
    rules.apply_mut(&mut g, &power(&rules, 1, "Interdict"), None).unwrap();
    answer_all(&rules, &mut g);
    // 190: Scrap pays 1 chip (a chip is worth 4 since r83).
    assert_eq!((g.resource(0, Resource::Chips), g.tokens(h, TROOP)), (1, 0));
    assert!(g.pile(&cards::played(0)).is_empty() && g.pile(&cards::played(1)).is_empty());
}

#[test]
fn tokens_of_a_placement_still_uncover() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    set_res(&mut g, 0, Resource::Energy, 1);
    let income = ai::track_income(&w, &g, 0).len();
    rules.apply_mut(&mut g, &power(&rules, 0, "Expand"), None).unwrap();
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
    // The base route, Expand, which refreshes here: a test of the track, not of its limit.
    let expand = power_index(&rules, 0, "Expand");
    let place = |rules: &Rules, g: &mut GameState| {
        set_var(g, ai::power_used(0, expand), 0);
        act(rules, g, power(rules, 0, "Expand"));
    };
    place(&rules, &mut g);
    // The 2nd space (the Builder's): +2 credits a round (078; 273: 3 ÷ 1.5).
    assert_eq!(
        (board::track_taken(&g, 0), credit_income(&g), ai::keep2(&w, &g, 0)),
        (2, 2, false)
    );
    place(&rules, &mut g);
    // The keep upgrade is the card-play track's second space (068).
    assert_eq!((board::track_taken(&g, 0), ai::keep2(&w, &g, 0)), (3, false));
    set_var(&mut g, board::track2_key(0), 2);
    assert!(ai::keep2(&w, &g, 0));
    assert_eq!(node_of(&g, 0).len(), 3);
    // The tracks run out after their last spaces (021: both of them).
    set_var(&mut g, board::track_key(0), ai::board(&w, 0).economy_track.len() as i64);
    assert!(ai::can_place(&w, &g, 0));
    set_var(&mut g, board::track2_key(0), ai::board(&w, 0).play_track.len() as i64);
    assert!(!ai::can_place(&w, &g, 0));
}

/// 212: a rider's spot is on or next to the new node; a fresh one has none of the seat's other nodes on or next to it.
#[test]
fn a_rider_finds_its_spot_and_a_fresh_one_no_other_node_of_yours() {
    use crate::op::NodeSpot;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    // The lab's board starts empty: a factory of our own on a land hex with two land neighbors.
    let f = rules.map.numbered(0, piece(0).node);
    put(&mut g, f, crate::modules::common::FACTORY, 1);
    let ns = w.map.hexes[f].neighbors.clone();
    let (h, other) = (ns[0], ns[1]);
    // No node of seat 0 within 2 of the factory, so only the ones put here count.
    for x in 0..g.hexes.len() {
        if w.map.distance(x, f) <= 2 {
            put(&mut g, x, node(0), 0);
        }
    }
    put(&mut g, h, node(0), 1);
    assert!(board::spots_near(&w, &g, 0, h, &[NodeSpot::Factory], false).contains(&f));
    assert!(board::spots_near(&w, &g, 0, h, &[NodeSpot::Factory], true).contains(&f), "no other node of yours there");
    put(&mut g, other, node(0), 1);
    assert!(!board::spots_near(&w, &g, 0, h, &[NodeSpot::Factory], true).contains(&f), "another node of yours is next to it");
    assert!(board::spots_near(&w, &g, 0, h, &[NodeSpot::Factory], false).contains(&f));
}

/// 261: Peer Review's rider pays for a station the node brings into reach, not one the placement's own uncovered
/// space built there (the Researcher's free station): it reads the stations standing before the placement. This
/// checks the rider's filter on `Ctx::node_spots_before`; the PlaceNode arm fills it before `place_node_within`.
#[test]
fn a_rider_ignores_a_station_its_own_track_space_built() {
    use crate::op::NodeSpot;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let s = rules.map.numbered(0, piece(0).node);
    let h = w.map.hexes[s].neighbors[0];
    for x in 0..g.hexes.len() {
        put(&mut g, x, node(0), 0);
    }
    put(&mut g, h, node(0), 1);
    put(&mut g, s, crate::modules::common::STATION, 1);
    let rider = [Op::IfNodeNext {
        spots: vec![NodeSpot::Station],
        then: vec![Op::Gain { resource: Resource::Insight, amount: 1 }],
        fresh: true,
    }];
    let insight = |before: Vec<usize>| {
        let mut gg = g.clone();
        set_res(&mut gg, 0, Resource::Insight, 0);
        let mut choices = module::Choices::new(Vec::new());
        let mut cx = test_cx(&rules, &mut choices);
        let mut ctx = effects::Ctx::seat(0);
        ctx.node = Some(h);
        ctx.node_spots_before = before;
        effects::run(&mut gg, &mut cx, &rider, &mut ctx).unwrap();
        gg.resource(0, Resource::Insight)
    };
    assert_eq!(insight(vec![s]), 1, "a station that stood there pays");
    assert_eq!(insight(Vec::new()), 0, "one the placement built doesn't");
}

/// 212: a green card's side bonus lifts a Fallout token in reach.
#[test]
fn lift_fallout_takes_a_token_in_reach() {
    let (rules, mut g) = at_step(&[], 1);
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, crate::modules::common::FALLOUT, 1);
    run_ops(&rules, &mut g, &[Op::LiftFallout { count: 1, at: Default::default() }]);
    assert_eq!(g.tokens(a, crate::modules::common::FALLOUT), 0);
}

#[test]
fn a_track_s_last_steps_cost_more() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let b = ai::board(&w, 0);
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, node(0), 1);
    // Both tracks down to their last steps.
    set_var(&mut g, board::track_key(0), b.economy_track.len() as i64 - 1);
    set_var(&mut g, board::track2_key(0), b.play_track.len() as i64 - 1);
    let extra = b.economy_track.last().unwrap().cost.min(b.play_track.last().unwrap().cost);
    assert!(extra > 0);
    let deploy = power(&rules, 0, "Expand");
    set_res(&mut g, 0, Resource::Energy, extra - 1);
    assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "the step can't be paid");
    set_res(&mut g, 0, Resource::Energy, extra);
    act(&rules, &mut g, deploy);
    assert_eq!(g.resource(0, Resource::Energy), 0);
}
