//! The presence powers (066), the one payment path (041), the marker's reward
//! and Science on the board.

use super::*;
use crate::content::Op;
use crate::modules::common::{hex_opt, is_ocean, presence, HARDEN, MARKER};
use crate::modules::{ai, alignment, board, effects};
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
fn clearing_a_marker_costs_its_credits_and_pays_its_reward() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).presence);
    let marked = w.map.hexes[a].neighbors.iter().copied().find(|&h| !is_ocean(&w, h)).unwrap();
    put(&mut g, marked, MARKER, 1);
    assert!(!board::presence_hexes(&w, &g, 0).contains(&marked), "no presence on a marker");
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Credits, 5);
    let credits = 5 - w.int("board.clear_cost");
    let unity = g.var(alignment::bucket(content::Side::Unity));
    rules.apply_mut(&mut g, &std_act(0, "clear"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(marked, MARKER), 0);
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
    super::threats::deal(&mut g, "smear_campaign", 0);
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
fn shields_delays_and_marker_clearing_are_refused_off_science() {
    for op in ["shield_presence", "delay_threat", "clear_markers"] {
        let set = format!("cards.payroll_automation.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        let err = Rules::new(&baseline_with(&[&set])).err().expect("refused");
        assert!(err.contains("Science"), "{err}");
        let on_science = format!("cards.literature_crawler.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        assert!(Rules::new(&baseline_with(&[&on_science])).is_ok());
    }
}

/// The board's power called `name`, as an action of `seat`.
fn power(w: &module::World, seat: u8, name: &str) -> Action {
    let k = ai::board(w, seat as usize).powers.iter().position(|p| p.name == name).expect("a power");
    Action::Power { seat, power: k as u8 }
}

#[test]
fn every_board_places_for_1_energy_once_a_round() {
    for seat in [0u8, 1] {
        let (rules, mut g) = at_step(&[], 1);
        let w = rules.world();
        let start = rules.map.numbered(seat as usize, piece(seat as usize).presence);
        put(&mut g, start, presence(seat as usize), 1);
        let deploy = power(&w, seat, "Deploy");
        set_res(&mut g, seat as usize, Resource::Energy, 1);
        let before = crate::modules::common::presence_count(&g, seat as usize);
        rules.apply_mut(&mut g, &deploy, None).unwrap();
        answer_all(&rules, &mut g);
        assert_eq!(crate::modules::common::presence_count(&g, seat as usize), before + 1);
        assert_eq!(g.resource(seat as usize, Resource::Energy), 0);
        set_res(&mut g, seat as usize, Resource::Energy, 1);
        assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "once a round");
    }
}

#[test]
fn a_presence_power_pays_its_energy_and_then_the_step() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let b = ai::board(&w, 0);
    let step = b.presence[5].cost.min(b.presence2[5].cost);
    let deploy = power(&w, 0, "Deploy");
    set_res(&mut g, 0, Resource::Energy, step);
    assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "the step is on top");
    set_res(&mut g, 0, Resource::Energy, step + 1);
    rules.apply_mut(&mut g, &deploy, None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Energy), 0);
}

#[test]
fn the_builder_s_second_presence_power_takes_its_credits() {
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let buy = power(&w, 0, "Buy In");
    let Action::Power { power: k, .. } = buy else { unreachable!() };
    let price = ai::board(&w, 0).powers[k as usize].cost.iter().find(|(r, _)| *r == Resource::Credits).unwrap().1;
    set_res(&mut g, 0, Resource::Energy, 1);
    set_res(&mut g, 0, Resource::Credits, price - 1);
    assert!(rules.apply_mut(&mut g.clone(), &buy, None).is_err());
    set_res(&mut g, 0, Resource::Credits, price);
    rules.apply_mut(&mut g, &buy, None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!((g.resource(0, Resource::Energy), g.resource(0, Resource::Credits)), (0, 0));
}

#[test]
fn the_protector_s_second_presence_power_sacrifices_diplomacy() {
    use crate::content::Tag;
    use crate::key::Key;
    use crate::modules::cards;
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    let start = rules.map.numbered(1, piece(1).presence);
    put(&mut g, start, presence(1), 1);
    let accord = power(&w, 1, "Accord");
    set_res(&mut g, 1, Resource::Energy, 1);
    let program = w
        .content
        .cards
        .iter()
        .find(|c| c.tier == 1 && c.tags.contains(&Tag::Diplomacy) && !c.is_building() && !c.is_event())
        .map(|c| Key::new(&c.id))
        .unwrap();
    assert!(rules.apply_mut(&mut g.clone(), &accord, None).is_err(), "no tag");
    g.piles.insert(cards::played(1).into(), std::sync::Arc::new(vec![program]));
    rules.apply_mut(&mut g, &accord, None).unwrap();
    answer_all(&rules, &mut g);
    assert!(g.pile(&cards::played(1)).is_empty(), "sacrificed");
    assert_eq!(g.resource(1, Resource::Energy), 0);
}

#[test]
fn the_cheap_powers_sacrifice_one_tag_for_chips_or_credits() {
    use crate::content::Tag;
    use crate::key::Key;
    use crate::modules::cards;
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let program = |t: Tag| {
        let c = w.content.cards.iter().find(|c| c.tags.contains(&t) && !c.is_building() && !c.is_event() && c.tier == 1);
        Key::new(&c.expect("a program").id)
    };
    g.piles.insert(cards::played(0).into(), std::sync::Arc::new(vec![program(Tag::Industry)]));
    g.piles.insert(cards::played(1).into(), std::sync::Arc::new(vec![program(Tag::Security)]));
    set_res(&mut g, 0, Resource::Chips, 0);
    set_res(&mut g, 1, Resource::Credits, 0);
    rules.apply_mut(&mut g, &power(&w, 0, "Scrap"), None).unwrap();
    answer_all(&rules, &mut g);
    rules.apply_mut(&mut g, &power(&w, 1, "Stand Down"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!((g.resource(0, Resource::Chips), g.resource(1, Resource::Credits)), (2, 3));
    assert!(g.pile(&cards::played(0)).is_empty() && g.pile(&cards::played(1)).is_empty());
}

#[test]
fn each_seat_starts_with_a_bound_clean_city_at_every_size_and_difficulty() {
    use crate::modules::common::{cube, cube_owner, corrupted, is_city};
    for countries in 1..=4 {
        for sets in [vec![], vec!["adversary.difficulty=4", "adversary.extra_corrupted=1"]] {
            let mut c = baseline();
            c.apply_override(&format!("countries={countries}")).unwrap();
            for s in &sets {
                c.apply_override(s).unwrap();
            }
            let rules = Rules::new(&c).unwrap();
            let g = rules.new_game(1);
            let w = rules.world();
            for s in 0..g.seats.len() {
                let bound: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, &cube(s)) > 0).collect();
                assert_eq!(bound.len(), 1, "seat {s}, {countries} countries");
                let h = bound[0];
                assert!(is_city(&w, h) && !corrupted(&g, h) && cube_owner(&g, h) == Some(s));
                let c = crate::modules::common::home_country(&w, s);
                let start = w.map.numbered(c, w.content.map.countries[c].presence);
                let nearest = w.content.map.countries[c].cities.iter().map(|&n| w.map.numbered(c, n));
                let clean_or_bound = nearest.filter(|&x| !corrupted(&g, x));
                let best = clean_or_bound.map(|x| w.map.distance(start, x)).min().unwrap();
                assert_eq!(w.map.distance(start, h), best, "the nearest");
            }
        }
    }
}

#[test]
fn start_energy_lets_a_seat_deploy_in_round_1() {
    let rules = Rules::new(&baseline()).unwrap();
    let g = rules.new_game(1);
    assert_eq!(g.resource(0, Resource::Energy), rules.world().int("ai.start_energy"));
    assert!(rules.world().int("ai.start_energy") >= 1);
}

#[test]
fn no_config_has_a_growth_phase_and_tokens_of_a_placement_still_uncover() {
    let c = baseline();
    assert!(!c.round.iter().any(|p| p.contains("growth")));
    let (rules, mut g) = at_step(&[], 1);
    let w = rules.world();
    set_res(&mut g, 0, Resource::Energy, 1);
    let income = ai::track_income(&w, &g, 0).len();
    rules.apply_mut(&mut g, &power(&w, 0, "Deploy"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(board::track_taken(&g, 0) + board::track2_taken(&g, 0), 3);
    assert_eq!(ai::track_income(&w, &g, 0).len(), income + 1);
}
