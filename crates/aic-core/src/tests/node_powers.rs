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

/// The board's free Growth option (277: the easy one; Buy In on the Builder, Levy on the Protector), the old base route.
fn free_growth(seat: usize) -> &'static str {
    ["Buy In", "Levy"][seat]
}

/// 252 (Matt's ruling 1): every board has a free node placement paying only the track step's energy, once a round;
/// r142 (Matt, 2026-11-19): it is a node power, and the board's others may be used in the same round.
#[test]
fn the_base_route_costs_only_the_step_once_a_round() {
    for seat in [0u8, 1] {
        let (rules, mut g) = at_step(&[], 1);
        let w = rules.world();
        let start = rules.map.numbered(seat as usize, piece(seat as usize).node);
        put(&mut g, start, node(seat as usize), 1);
        let deploy = power(&rules, seat as usize, free_growth(seat as usize));
        let other = power(&rules, seat as usize, ["Annex", "Accord"][seat as usize]);
        set_res(&mut g, seat as usize, Resource::Credits, 0);
        set_res(&mut g, seat as usize, Resource::Energy, step_cost(&w, 1));
        let before = node_count(&g, seat as usize);
        rules.apply_mut(&mut g, &deploy, None).unwrap();
        answer_all(&rules, &mut g);
        assert_eq!(node_count(&g, seat as usize), before + 1);
        assert_eq!(g.resource(seat as usize, Resource::Energy), 0, "the step's energy alone");
        set_res(&mut g, seat as usize, Resource::Energy, 10);
        set_res(&mut g, seat as usize, Resource::Chips, 1);
        assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "once a round");
        assert!(rules.apply_mut(&mut g.clone(), &other, None).is_ok(), "another node power, the same round");
    }
}

/// 289 (D129): an unaffordable Growth option's status names its numbers (it read "unaffordable: the next track
/// space's energy, after the power's own" with none), and no power reads "ready" once the game is over.
#[test]
fn an_unaffordable_power_says_what_is_short() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let step = step_cost(&w, 5);
    assert!(step > 0);
    set_res(&mut g, 0, Resource::Energy, step - 1);
    set_res(&mut g, 0, Resource::Credits, 50);
    let status = |g: &GameState, name: &str| {
        let b = rules.view(g).seats[0].board.clone().expect("a board");
        b.cards.into_iter().find(|c| c.name == name).expect("the power").status
    };
    let st = status(&g, free_growth(0));
    assert_eq!(st, format!("unaffordable: needs {} (for the next track space); you have {}", Resource::Energy.amount(step), step - 1));
    g.outcome = Some(crate::state::Outcome { won: false, round: g.round, reason: "test".into(), detail: String::new() });
    assert_eq!(status(&g, free_growth(0)), "game over");
}

/// 261: a node route is used for its node: with no free hex in its range of the seat's nodes it isn't offered (Outpost, the
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
    // 297: Outpost places within the range it prints (2), so its route is shut only when every hex that near is taken.
    let printed = ai::board(&w, seat).powers[power_index(&rules, seat, "Outpost")].effects.iter().find_map(|e| match e {
        Op::PlaceNode { range, .. } => Some(*range),
        _ => None,
    });
    let range = printed.expect("Outpost places a node");
    assert_eq!(range, 2);
    assert!(rules.apply_mut(&mut g.clone(), &outpost, None).is_ok(), "offered with a free hex");
    // The hexes next to its node taken by seat 0's nodes: still offered, a hex 2 away is free.
    for h in board::node_sites_within(&w, &g, seat, 1) {
        put(&mut g, h, node(0), 1);
    }
    assert!(board::node_sites_within(&w, &g, seat, 1).is_empty() && !board::node_sites_within(&w, &g, seat, range).is_empty());
    assert!(rules.apply_mut(&mut g.clone(), &outpost, None).is_ok(), "a free hex 2 away: offered");
    // Every hex in its range taken: no free hex for seat 1's.
    for h in board::node_sites_within(&w, &g, seat, range) {
        put(&mut g, h, node(0), 1);
    }
    assert!(board::node_sites_within(&w, &g, seat, range).is_empty());
    assert!(rules.apply_mut(&mut g.clone(), &outpost, None).is_err(), "no free hex: not offered");
    let mut gg = g.clone();
    let mut choices = module::Choices::new(Vec::new());
    let mut cx = test_cx(&rules, &mut choices);
    let placed = board::place_node_within(&mut gg, &mut cx, seat, range).unwrap();
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

/// 206: a placement pays the step's energy (252: the free route, nothing more; 276: Buy In).
#[test]
fn a_node_power_pays_its_energy_and_the_step() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let step = step_cost(&w, 5);
    let deploy = power(&rules, 0, "Buy In");
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

/// 247 (Matt, 2026-11-15: energy powers things); r142: the energy-rich boards' costly node powers (the Builder's
/// Sprawl, the Protector's Outpost) cost energy, not credits or a sacrifice, beside the step's, and pay 2 credits.
#[test]
fn the_costly_node_power_runs_on_energy() {
    for (seat, name) in [(0usize, "Sprawl"), (1, "Outpost")] {
        let (rules, mut g) = at_step(&[], 1);
        let w = rules.world();
        let start = rules.map.numbered(seat, piece(seat).node);
        put(&mut g, start, node(seat), 1);
        let grow = power(&rules, seat, name);
        let price = power_energy(&rules, seat, name);
        assert!(price > 0);
        g.piles.insert(cards::played(seat).into(), Arc::new(vec![program(&w, Tag::Diplomacy)]));
        set_res(&mut g, seat, Resource::Credits, 0);
        set_res(&mut g, seat, Resource::Energy, price - 1);
        assert!(rules.apply_mut(&mut g.clone(), &grow, None).is_err(), "{name}: its energy");
        set_res(&mut g, seat, Resource::Energy, price + 20);
        let before = node_count(&g, seat);
        rules.apply_mut(&mut g, &grow, None).unwrap();
        answer_all(&rules, &mut g);
        assert_eq!(node_count(&g, seat), before + 1, "{name} places 1 node");
        assert_eq!(g.pile(&cards::played(seat)).len(), 1, "no sacrifice");
        assert_eq!(g.resource(seat, Resource::Credits), 2, "its benefit");
        assert!(g.resource(seat, Resource::Energy) <= 20, "{name} pays its energy");
    }
}

/// 212: a troop power is force (a Security card for troops in reach); r98: Stand Down merged into Interdict.
#[test]
fn the_cheap_powers_sacrifice_one_tag_for_a_chip_or_a_troop() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    g.piles.insert(cards::played(0).into(), Arc::new(vec![program(&w, Tag::Industry)]));
    g.piles.insert(cards::played(1).into(), Arc::new(vec![program(&w, Tag::Security)]));
    set_res(&mut g, 0, Resource::Chips, 0);
    assert!(!rules.legal_actions(&g).contains(&power(&rules, 1, "Interdict I")), "no troop in reach");
    let h = rules.map.numbered(0, piece(0).node);
    put(&mut g, h, node(1), 1);
    put(&mut g, h, TROOP, 1);
    // 276: one card can't pay a tier II's 2 tags.
    assert!(!rules.legal_actions(&g).contains(&power(&rules, 0, "Scrap II")), "2 Industry");
    rules.apply_mut(&mut g, &power(&rules, 0, "Scrap I"), None).unwrap();
    answer_all(&rules, &mut g);
    rules.apply_mut(&mut g, &power(&rules, 1, "Interdict I"), None).unwrap();
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
    rules.apply_mut(&mut g, &power(&rules, 0, "Buy In"), None).unwrap();
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
    // The free Growth option, Buy In, which refreshes here: a test of the track, not of its limit.
    let expand = power_index(&rules, 0, "Buy In");
    let place = |rules: &Rules, g: &mut GameState| {
        set_var(g, ai::power_used(0, expand), 0);
        act(rules, g, power(rules, 0, "Buy In"));
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
    let deploy = power(&rules, 0, "Buy In");
    set_res(&mut g, 0, Resource::Energy, extra - 1);
    assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "the step can't be paid");
    set_res(&mut g, 0, Resource::Energy, extra);
    act(&rules, &mut g, deploy);
    assert_eq!(g.resource(0, Resource::Energy), 0);
}

/// 276 (diff review): a two-node power is offered only when both its nodes' steps can be paid; it was paid in
/// full for one. r142: no board prints one (the test makes Sprawl one again).
#[test]
fn a_two_node_power_needs_both_steps() {
    let two = [r#"ais.builder.powers.0.effects=[{"op":"place_node","count":2}]"#];
    let (rules, mut g) = at_step(&two, 4);
    let w = rules.world();
    let sprawl = power(&rules, 0, "Sprawl");
    let price = power_energy(&rules, 0, "Sprawl");
    let one = step_cost(&w, 4);
    assert!(one > 0);
    set_res(&mut g, 0, Resource::Energy, price + one);
    assert!(!rules.legal_actions(&g).contains(&sprawl), "one step's energy, not two");
    set_res(&mut g, 0, Resource::Energy, 30);
    assert!(rules.legal_actions(&g).contains(&sprawl));
}

/// 286 (D121): a two-node option needs a hex for its second node once the first is placed: with one free hex in
/// range, the hexes next to it decide.
#[test]
fn a_two_node_growth_needs_a_hex_for_its_second_node() {
    use crate::modules::common::FALLOUT;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let a = m.numbered(0, piece(0).node);
    put(&mut g, a, node(0), 1);
    let h = *m.hexes[a].neighbors.iter().find(|&&x| board::node_sites_within(&w, &g, 0, 1).contains(&x)).unwrap();
    // Fallout on every free hex within 2 of `a` but `h`: `h` is the one site in range 1, and nothing is next to it.
    let near2: Vec<usize> = (0..g.hexes.len())
        .filter(|&x| x != h && x != a && (m.hexes[a].neighbors.contains(&x) || m.hexes[h].neighbors.contains(&x)))
        .collect();
    for &x in &near2 {
        put(&mut g, x, FALLOUT, 1);
    }
    assert_eq!(board::node_sites_within(&w, &g, 0, 1), vec![h]);
    assert!(!board::second_node_site(&w, &g, 0, 1), "one free hex: the second node has none");
    let x = *near2.iter().find(|&&x| m.hexes[h].neighbors.contains(&x) && !m.hexes[a].neighbors.contains(&x)).unwrap();
    put(&mut g, x, FALLOUT, 0);
    assert!(board::second_node_site(&w, &g, 0, 1), "the first node's reach finds the second a hex");
}

/// 286 (D128): Patrol II's extra energy buys its hardening alone: with no facility to harden on or next to a target
/// it isn't offered (Patrol I is); with one, it is.
#[test]
fn patrol_ii_needs_a_facility_to_harden() {
    let (rules, mut g) = at_step(&[], 1);
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, node(1), 1);
    put(&mut g, a, crate::modules::common::ADV_AMBASSADOR, 1);
    set_res(&mut g, 1, Resource::Energy, 10);
    let (one, two) = (power(&rules, 1, "Patrol I"), power(&rules, 1, "Patrol II"));
    let tried = |g: &GameState, a: &Action| rules.apply_mut(&mut g.clone(), a, None).map(|_| ()).map_err(|e| format!("{e:?}"));
    assert!(tried(&g, &one).is_ok(), "Patrol I pushes");
    let refused = tried(&g, &two).expect_err("nothing to harden");
    assert!(refused.contains("no facility to harden"), "{refused}");
    put(&mut g, a, crate::modules::common::FACTORY, 1);
    put(&mut g, a, crate::modules::common::cube(1), 1);
    assert!(tried(&g, &two).is_ok(), "a factory of its own to harden");
}

/// 276 (diff review): Interdict II removes both kinds on its target, troops and agents (one damage op asked which).
#[test]
fn interdict_ii_removes_troops_and_agents() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let sec: Vec<Key> = programs(&w, Tag::Security).filter(|c| c.tier == 1).take(2).map(|c| Key::new(&c.id)).collect();
    assert_eq!(sec.len(), 2);
    g.piles.insert(cards::played(1).into(), Arc::new(sec));
    let h = rules.map.numbered(0, piece(0).node);
    put(&mut g, h, node(1), 1);
    put(&mut g, h, TROOP, 2);
    put(&mut g, h, crate::modules::common::ADV_AMBASSADOR, 2);
    rules.apply_mut(&mut g, &power(&rules, 1, "Interdict II"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(h, TROOP), g.tokens(h, crate::modules::common::ADV_AMBASSADOR)), (0, 0));
}

/// 276 (diff review): a hardening after a target counts only where a facility it can protect stands in its area
/// (Patrol II was offered, and paid for, on a hex where neither its push nor its hardening could act).
#[test]
fn a_targeted_hardening_needs_a_facility_to_protect() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let ops: Vec<Op> = serde_json::from_str(r#"[{"op": "target", "on": "adversary"}, {"op": "harden", "count": 1, "at": "near"}]"#).unwrap();
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, crate::modules::common::ADV_AMBASSADOR, 1);
    assert!(!effects::feasible(&w, &g, 0, &ops), "no facility on or next to it");
    put(&mut g, a, crate::modules::common::FACTORY, 1);
    put(&mut g, a, crate::modules::common::cube(0), 1);
    assert!(effects::feasible(&w, &g, 0, &ops));
}

/// r142 (Matt, 2026-11-19): Growth is gone: each board prints ordinary node powers, one cheap (free) and two costly
/// ones that carry a benefit, each its own use, so all are usable in the same round; each places one node.
#[test]
fn each_board_prints_one_cheap_node_power_and_two_costly_ones() {
    let (rules, _) = at_step(&[], 1);
    let w = rules.world();
    for b in &w.content.ais {
        assert!(b.powers.iter().all(|p| p.group.as_deref() != Some("Growth")), "{}: no Growth group", b.id);
        let nodes = |p: &crate::content::Power| -> i64 {
            p.effects.iter().map(|e| if let Op::PlaceNode { count, .. } = e { *count } else { 0 }).sum()
        };
        let node_powers: Vec<_> = b.powers.iter().filter(|p| nodes(p) > 0).collect();
        let cheap: Vec<_> = node_powers.iter().filter(|p| p.cost.is_empty() && p.tags.is_empty()).collect();
        assert_eq!((node_powers.len(), cheap.len()), (3, 1), "{}: three node powers, one free", b.id);
        for p in &node_powers {
            assert!(p.group.is_none(), "{} {}: its own use", b.id, p.name);
            assert_eq!(nodes(p), 1, "{} {}: one node", b.id, p.name);
            assert!(p.effects.len() >= 2, "{} {}: a node and a benefit", b.id, p.name);
        }
    }
}
