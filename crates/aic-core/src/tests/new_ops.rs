//! 135: the new adversary model's ops and their readers.

use super::*;
use crate::modules::common::{is_city, is_ocean};
use crate::modules::{board, effects};
use crate::op::TheftWho;

/// Runs `ops` as threat `t` executing in country `c`, hitting the nearest seat (146; a tie, the first).
fn run_threat(rules: &Rules, g: &mut GameState, t: &str, c: usize, ops: &[content::Op]) {
    let hit = crate::modules::threats::nearest_seats(&rules.world(), g, t).first().copied();
    let mut ctx = effects::Ctx::threat(Some(c)).with_threat(crate::key::Key::new(t)).why(t).hitting(hit);
    with_cx(rules, |cx| effects::run(g, cx, ops, &mut ctx)).unwrap();
}

/// Sets var `k` to `v` on a crafted state.
fn setv(g: &mut GameState, k: &str, v: i64) {
    g.vars.insert(crate::key::Key::new(k), v);
}

/// Puts threat `t`'s first location token on hex `h`.
fn token_on(g: &mut GameState, t: &str, h: usize) {
    setv(g, &format!("threats.loc.{t}.0"), h as i64 + 1);
}

/// 184 (r78): "2 adversary Troops land on the base here" with no base token (one country has no base) lands
/// nothing; it fell back to the threat's other token and put the troops on the city.
#[test]
fn a_troop_line_for_a_missing_token_type_lands_nothing() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let def = w.content.threats.iter().find(|t| t.id == "garrison_state").expect("Garrison State");
    assert_eq!(def.locations, [content::Location::City, content::Location::Base], "its printed types, city first");
    let troops: Vec<content::Op> = def.effects.iter().filter(|o| matches!(o, content::Op::Troops { .. })).cloned().collect();
    assert!(!troops.is_empty());
    let city = (0..g.hexes.len()).find(|&h| is_city(&w, h) && w.map.hexes[h].country == 0).expect("a city");
    token_on(&mut g, "garrison_state", city);
    let before = g.tokens(city, TROOP);
    run_threat(&rules, &mut g, "garrison_state", 0, &troops);
    assert_eq!(g.tokens(city, TROOP), before, "no troop on the city token");
}

#[test]
fn extra_corp_troops_go_past_the_cap_but_one_a_base_a_production_step() {
    let (rules, mut g) = lab(&["board.bases"]);
    let w = rules.world();
    let cap = w.int("board.unit_cap");
    let [b0, b1] = rules.map.fronts[0].bases;
    put(&mut g, b0, UNIT, cap);
    put(&mut g, b1, UNIT, cap);
    // Two registrations on b0 (two Conscriptions): still 1 extra (ruling 4).
    setv(&mut g, &board::corp_extra_key(b0, 3), 1);
    setv(&mut g, &board::corp_extra_key(b0, 5), 1);
    run(&rules, &mut g);
    assert_eq!(g.tokens(b0, UNIT), cap + 1);
    assert_eq!(g.tokens(b1, UNIT), cap);
}

#[test]
fn warmongering_needs_an_adversary_troop_by_the_base() {
    let (rules, mut g) = lab(&["board.bases"]);
    let w = rules.world();
    let cap = w.int("board.unit_cap");
    let [b0, b1] = rules.map.fronts[0].bases;
    put(&mut g, b0, UNIT, cap);
    put(&mut g, b1, UNIT, cap);
    put(&mut g, b0, TROOP, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(b0, UNIT), g.tokens(b1, UNIT)), (cap + 1, cap));
}

#[test]
fn war_brought_forward_fights_a_front_twice() {
    // 139: war every round, so it fights twice: the surplus strikes twice.
    let (rules, mut g0) = lab(&["board.war"]);
    let m = &rules.map;
    let [b0, b1] = m.fronts[0].bases;
    g0.piles.remove(CURRENT);
    for n in &piece(1).factories {
        put(&mut g0, m.numbered(1, *n), FACTORY, 1);
    }
    put(&mut g0, b0, UNIT, 2);
    put(&mut g0, b1, UNIT, 1);
    let fs = piece(1).factories.iter().map(|&n| m.numbered(1, n)).collect::<Vec<_>>();
    let left = |g: &GameState| fs.iter().map(|&f| g.tokens(f, FACTORY)).sum::<i64>();
    let mut g = g0.clone();
    // The key holds the round of the second fight: this one.
    setv(&mut g, &board::extra_war_key(0), 1);
    run(&rules, &mut g);
    assert_eq!(left(&g), fs.len() as i64 - 2.min(fs.len() as i64));
}

#[test]
fn theft_hits_the_nearest_player_not_the_richest() {
    // 146: theft hits the nearest player like every harm, however rich another is.
    let (rules, mut g) = lab(&["board.produce"]);
    let w = rules.world();
    let city = rules.map.countries[0].hexes.iter().copied().find(|&h| is_city(&w, h)).unwrap();
    let far = (0..g.hexes.len()).find(|&h| !is_ocean(&w, h) && rules.map.distance(city, h) >= 3).unwrap();
    token_on(&mut g, "shakedown", city);
    put(&mut g, city, node(0), 1);
    put(&mut g, far, node(1), 1);
    set_res(&mut g, 0, Resource::Credits, 10);
    set_res(&mut g, 1, Resource::Credits, 40);
    let ops = [content::Op::Theft { who: TheftWho::Nearest, resource: Resource::Credits, amount: 8, compute: 0, cards: 2 }];
    run_threat(&rules, &mut g, "shakedown", 0, &ops);
    assert_eq!((g.resource(0, Resource::Credits), g.resource(1, Resource::Credits)), (2, 40));
}

#[test]
fn a_hex_blackout_darkens_only_the_cities_near_it() {
    let (rules, mut g) = lab(&["board.produce"]);
    let w = rules.world();
    let cities: Vec<usize> = rules.map.countries[0].hexes.iter().copied().filter(|&h| is_city(&w, h)).collect();
    let (a, b) = (cities[0], *cities.iter().find(|&&h| rules.map.distance(cities[0], h) > 1).unwrap());
    put(&mut g, a, cube(0), 1);
    put(&mut g, b, cube(0), 1);
    let one = {
        let mut g = g.clone();
        put(&mut g, b, cube(0), 0);
        run(&rules, &mut g);
        g.resource(0, Resource::Energy)
    };
    assert!(one > 0);
    setv(&mut g, &board::hex_flag(content::RoundFlag::Blackout, a), 1);
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Energy), one);
}

#[test]
fn an_adversary_factory_liberates_as_an_adversary_station_does() {
    // 137: Liberate takes an adversary station or factory at one price.
    let (rules, mut g) = lab(&["board.bases"]);
    let w = rules.world();
    let z = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial).unwrap();
    put(&mut g, z, FACTORY, 1);
    assert_eq!(board::liberate_cost(&w, &g, z), None);
    put(&mut g, z, ADV_CUBE, 1);
    // 206 (amendment 9208): Claim's table, and no agent on it.
    let at = w.int("board.claim_factory");
    assert_eq!(board::liberate_cost(&w, &g, z), Some(at));
    put(&mut g, z, ADV_AMBASSADOR, 1);
    assert_eq!(board::liberate_cost(&w, &g, z), None);
}

#[test]
fn a_city_liberates_at_one_price_all_game() {
    // 156: the late liberation (154) is gone: a city costs `liberate_at` at every stage, plus one an
    // agent on it. The lab prices Liberate at 1, so the baseline's number goes back on.
    let (rules, mut g) = lab_with(&["board.bases"], &["board.liberate_at=5"]);
    let w = rules.world();
    let c = (0..g.hexes.len()).find(|&h| is_city(&w, h)).unwrap();
    put(&mut g, c, ADV_CUBE, 1);
    for stage in 1..=3 {
        setv(&mut g, crate::modules::adversary::STAGE, stage);
        assert_eq!(board::liberate_cost(&w, &g, c), Some(w.int("board.liberate_at")));
    }
}

#[test]
fn a_standing_troop_never_marches_or_strikes() {
    let (rules, mut g) = lab(&["board.march"]);
    let w = rules.world();
    let z = (0..g.hexes.len()).find(|&h| w.map.hexes[h].industrial).unwrap();
    put(&mut g, z, FACTORY, 1);
    put(&mut g, z, cube(0), 1);
    let next = w.map.hexes[z].neighbors.iter().copied().find(|&h| !is_ocean(&w, h)).unwrap();
    put(&mut g, next, TROOP, 1);
    setv(&mut g, &board::stand_key(next), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(z, FACTORY), g.tokens(next, TROOP)), (1, 1));
}
