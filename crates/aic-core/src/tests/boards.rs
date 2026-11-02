//! 068: the Builder and Protector boards, ambassadors that stay, and Cordon.

use std::sync::Arc;

use super::*;
use crate::content::{Op, RoundFlag, Tag};
use crate::key::Key;
use crate::modules::common::{is_base, is_city, is_ocean};
use crate::modules::{adversary, ai, board, cards, economy, plan};
use crate::op::Site;

/// A power of seat `s`'s board by name.
fn power(rules: &Rules, s: usize, name: &str) -> Action {
    let w = rules.world();
    let k = ai::board(&w, s).powers.iter().position(|p| p.name == name).unwrap();
    Action::Power { seat: s as u8, power: k as u8 }
}

fn piles(g: &mut GameState, pile: String, cards: &[&str]) {
    g.piles.insert(pile.into(), Arc::new(cards.iter().map(|c| Key::new(c)).collect()));
}

/// Three Industry programs in the Builder's play area.
fn builder_with_tags(g: &mut GameState) {
    piles(g, cards::played(0).to_string(), &["payroll_automation", "chip_spot_market", "bulk_silicon_procurement"]);
}

#[test]
fn a_protector_presence_stops_a_marching_troop() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let p = m.numbered(0, piece(0).presence);
    put(&mut g, p, &presence(1), 1);
    assert!(board::walled(&w, &g, p));
    let t = m.hexes[p].neighbors.iter().copied().find(|&h| kind(m, h) != HexKind::Ocean).unwrap();
    put(&mut g, t, TROOP, 1);
    // A target behind the wall: the walled hex is never a step toward it.
    let beyond = m.hexes[p].neighbors.iter().copied().find(|&h| kind(m, h) != HexKind::Ocean && m.distance(h, t) == 2);
    if let Some(b) = beyond {
        put(&mut g, b, STATION, 1);
        put(&mut g, b, &cube(1), 1);
        assert!(!board::troop_steps_from(&w, &g, t).contains(&p));
    }
    // The Builder's presence is no wall.
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    put(&mut g, p, &presence(0), 1);
    assert!(!board::walled(&w, &g, p));
}

#[test]
fn a_troop_strikes_a_target_on_a_walled_hex_from_next_to_it() {
    let (rules, mut g) = lab(&["board.march"]);
    let m = &rules.map;
    let p = m.numbered(0, piece(0).presence);
    put(&mut g, p, &presence(1), 1);
    put(&mut g, p, STATION, 1);
    put(&mut g, p, &cube(1), 1);
    let t = m.hexes[p].neighbors.iter().copied().find(|&h| kind(m, h) != HexKind::Ocean).unwrap();
    put(&mut g, t, TROOP, 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(p, STATION), 0);
}

#[test]
fn a_landing_troop_skips_a_walled_hex() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let c = m.numbered(0, piece(0).presence);
    let hexes: Vec<usize> = m.countries[m.hexes[c].country as usize].hexes.clone();
    let city = cities(m).into_iter().find(|h| hexes.contains(h)).unwrap();
    put(&mut g, city, &presence(1), 1);
    with_cx(&rules, |cx| adversary::land_troops(&mut g, cx, &hexes, 1, "land", "test.landed", None)).ok();
    assert_eq!(g.tokens(city, TROOP), 0);
}

#[test]
fn printed_and_revealed_tags_meet_a_requirement() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let extra = |g: &GameState| cards::required_tags(&w, g, 0)[Tag::Industry as usize] - cards::tag_counts(&w, g, 0)[Tag::Industry as usize];
    assert_eq!(extra(&g), 1, "printed on the Builder");
    g.vars.insert((&*board::track2_key(0)).into(), 2);
    assert_eq!(extra(&g), 2, "revealed on its card-play track");
}

#[test]
fn a_capstone_counts_once_uncovered() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let discount = |g: &GameState| cards::passives(&w, g, 0).iter().any(|p| matches!(p, Op::BuildingDiscount { amount: 2 }));
    assert!(!discount(&g));
    g.vars.insert((&*board::track2_key(0)).into(), 7);
    assert!(discount(&g));
}

#[test]
fn rush_build_pays_tags_and_no_credits() {
    let (rules, mut g) = lab_actions();
    builder_with_tags(&mut g);
    piles(&mut g, cards::hand(0).to_string(), &["liaison_corps"]);
    set_res(&mut g, 0, Resource::Credits, 0);
    set_res(&mut g, 0, Resource::Insight, 3);
    act(&rules, &mut g, power(&rules, 0, "Rush Build"));
    assert!(g.pile(&cards::built(0)).contains(&Key::new("liaison_corps")));
    assert_eq!(g.resource(0, Resource::Credits), 0);
    assert!(g.resource(0, Resource::Insight) < 3, "its insight is still paid");
    assert!(g.pile(&cards::played(0)).is_empty(), "three tags went");
}

#[test]
fn an_evicting_ambassador_stays() {
    let (rules, mut g) = lab_actions();
    let d = presence_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 2);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    act(&rules, &mut g, std_act(0, "remove"));
    assert_eq!((g.tokens(d, ADV_AMBASSADOR), g.tokens(d, ambassador(0))), (0, 1));
    assert_eq!(g.resource(0, Resource::Ambassadors), 0);
}

#[test]
fn an_ambassador_on_a_base_diverts_it_and_stays() {
    let (rules, mut g) = lab(&["board.bases"]);
    let w = rules.world();
    let base = (0..g.hexes.len()).find(|&h| is_base(&w, h)).unwrap();
    put(&mut g, base, ambassador(0), 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(base, ambassador(0)), g.tokens(base, UNIT)), (1, 0));
}

#[test]
fn an_ambassador_on_a_factory_stays_and_one_on_a_corrupted_station_takes_its_cube() {
    let (rules, mut g) = lab(&["board.bases", "board.produce"]);
    let w = rules.world();
    let z = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial).unwrap();
    put(&mut g, z, FACTORY, 1);
    put(&mut g, z, ambassador(0), 1);
    let st = (0..g.hexes.len()).find(|&h| h != z && !is_city(&w, h) && !is_ocean(&w, h)).unwrap();
    put(&mut g, st, STATION, 1);
    put(&mut g, st, ADV_CUBE, 1);
    put(&mut g, st, ambassador(1), 1);
    let c = w.map.hexes[st].country as usize;
    let adversary_cubes = g.var(&board::project_key(c, None));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(z, ambassador(0)), g.tokens(st, ambassador(1))), (1, 1));
    assert_eq!(g.var(&board::project_key(c, Some(1))), w.int("board.station_cubes"));
    assert_eq!(g.var(&board::project_key(c, None)), adversary_cubes);
}

#[test]
fn an_adversary_ambassador_stays_on_a_lone_presence() {
    let (rules, mut g) = lab(&["board.produce"]);
    let w = rules.world();
    let h = (0..g.hexes.len()).find(|&h| !is_city(&w, h) && !is_base(&w, h) && !is_ocean(&w, h)).unwrap();
    put(&mut g, h, presence(0), 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 1);
    assert_eq!(g.var(&economy::insight_debt(0)), 1);
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

#[test]
fn income_can_print_ambassadors() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    piles(&mut g, cards::built(0).to_string(), &["orbital_embassy"]);
    assert_eq!(economy::ambassador_income(&w, &g, 0), 1);
}

#[test]
fn starting_credits_lean_to_the_protector() {
    let rules = Rules::new(&baseline()).unwrap();
    let g = rules.new_game(1);
    assert!(g.resource(0, Resource::Credits) < g.resource(1, Resource::Credits));
}
