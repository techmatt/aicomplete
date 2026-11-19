//! 135: the new adversary model's ops and their readers.

use super::*;
use crate::modules::common::{is_city, is_ocean};
use crate::modules::{board, effects};
use crate::op::TheftWho;

/// Runs `ops` as threat `t` executing in country `c`, hitting its player (292).
fn run_threat(rules: &Rules, g: &mut GameState, t: &str, c: usize, ops: &[content::Op]) {
    let hit = crate::modules::threats::hit_seat_of(&rules.world(), g, c);
    let mut ctx = effects::Ctx::threat(Some(c)).with_threat(crate::key::Key::new(t)).why(t).hitting(hit);
    with_cx(rules, |cx| effects::run(g, cx, ops, &mut ctx)).unwrap();
}

/// Puts threat `t`'s first location token on hex `h`.
fn token_on(g: &mut GameState, t: &str, h: usize) {
    set_var(g, format!("threats.loc.{t}.0"), h as i64 + 1);
}

/// 184 (r78): "1 adversary Troop lands on the factory here" with no factory token lands nothing; it fell back to
/// the threat's other token and put the troops on the city.
#[test]
fn a_troop_line_for_a_missing_token_type_lands_nothing() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let def = w.content.threats.iter().find(|t| t.id == "garrison_state").expect("Garrison State");
    assert_eq!(def.locations, [content::Location::City, content::Location::Factory], "its printed types, city first");
    let troops: Vec<content::Op> = def.effects.iter().filter(|o| matches!(o, content::Op::Troops { .. })).cloned().collect();
    assert!(!troops.is_empty());
    let city = (0..g.hexes.len()).find(|&h| is_city(&w, h) && w.map.hexes[h].country == 0).expect("a city");
    token_on(&mut g, "garrison_state", city);
    let before = g.tokens(city, TROOP);
    run_threat(&rules, &mut g, "garrison_state", 0, &troops);
    assert_eq!(g.tokens(city, TROOP), before, "no troop on the city token");
}

#[test]
fn theft_hits_the_countrys_player_not_the_richest() {
    // 146: theft hits one player like every harm (292: its country's), however rich another is.
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
    set_var(&mut g, board::hex_flag(content::RoundFlag::Blackout, a), 1);
    run(&rules, &mut g);
    assert_eq!(g.resource(0, Resource::Energy), one);
}

#[test]
fn an_adversary_factory_liberates_as_an_adversary_station_does() {
    // 137: Liberate takes an adversary station or factory at one price.
    let (rules, mut g) = lab(&[]);
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
    let (rules, mut g) = lab_with(&[], &["board.liberate_at=5"]);
    let w = rules.world();
    let c = (0..g.hexes.len()).find(|&h| is_city(&w, h)).unwrap();
    put(&mut g, c, ADV_CUBE, 1);
    for stage in 1..=3 {
        set_var(&mut g, crate::modules::adversary::STAGE, stage);
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
    set_var(&mut g, board::stand_key(next), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(z, FACTORY), g.tokens(next, TROOP)), (1, 1));
}

/// 227: a threat's Fallout lands next to its token (a city is no open land), the lowest-numbered free hex first,
/// never on a node; two tokens take two hexes.
#[test]
fn a_threats_fallout_lands_next_to_its_token_never_on_a_node() {
    use crate::modules::adversary::fallout_free;
    use crate::modules::common::FALLOUT;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let city = rules.map.countries[0].hexes.iter().copied().find(|&h| is_city(&w, h)).unwrap();
    let mut free: Vec<usize> = w.map.hexes[city].neighbors.iter().copied().filter(|&n| fallout_free(&w, &g, n)).collect();
    free.sort_by_key(|&n| (w.map.hexes[n].country, w.map.hexes[n].number));
    assert!(free.len() >= 3, "a city with three free open neighbors");
    put(&mut g, free[0], node(0), 1);
    token_on(&mut g, "agitators", city);
    run_threat(&rules, &mut g, "agitators", 0, &[content::Op::Fallout { count: 2, at: crate::op::FalloutAt::Location }]);
    let on = |h: usize| g.tokens(h, FALLOUT);
    assert_eq!((on(city), on(free[0]), on(free[1]), on(free[2])), (0, 0, 1, 1));
    assert_eq!(g.stats.counters["fallout.placed"], 2);
}

/// 238 (D103): across a border the rulebook's §2 tie-break holds: the lower country first, then the lower hex
/// number (hex number alone put Onyx #14's Fallout on Cintamani #1).
#[test]
fn fallout_breaks_a_tie_by_country_then_number() {
    use crate::modules::adversary::fallout_free;
    use crate::modules::common::FALLOUT;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let hx = |h: usize| (w.map.hexes[h].country, w.map.hexes[h].number);
    // A hex whose free neighbors sort differently by number alone than by (country, number).
    let found = (0..w.map.hexes.len()).find_map(|h| {
        let free: Vec<usize> = w.map.hexes[h].neighbors.iter().copied().filter(|&n| fallout_free(&w, &g, n)).collect();
        let by_rule = *free.iter().min_by_key(|&&n| hx(n))?;
        let by_number = *free.iter().min_by_key(|&&n| w.map.hexes[n].number)?;
        (by_rule != by_number).then_some((h, by_rule))
    });
    let (h, want) = found.expect("a hex with free neighbors across a border");
    put(&mut g, h, node(0), 1);
    token_on(&mut g, "agitators", h);
    run_threat(&rules, &mut g, "agitators", 0, &[content::Op::Fallout { count: 1, at: crate::op::FalloutAt::Location }]);
    assert_eq!(g.tokens(want, FALLOUT), 1, "{:?} first", hx(want));
}

/// 227: a site-anchored line puts one token next to each of a country's lowest-numbered `count` corrupted sites of
/// its kind, and nothing in a country with none.
#[test]
fn a_site_anchored_fallout_needs_a_corrupted_site() {
    use crate::modules::common::FALLOUT;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let total = |g: &GameState| (0..g.hexes.len()).map(|h| g.tokens(h, FALLOUT)).sum::<i64>();
    let ops = [content::Op::Fallout { count: 1, at: crate::op::FalloutAt::CorruptedCity }];
    run_ops(&rules, &mut g, &ops);
    assert_eq!(total(&g), 0, "no corrupted city, no Fallout");
    let city = rules.map.countries[0].hexes.iter().copied().find(|&h| is_city(&w, h)).unwrap();
    put(&mut g, city, ADV_CUBE, 1);
    run_ops(&rules, &mut g, &ops);
    assert_eq!(total(&g), 1);
    assert!(w.map.hexes[city].neighbors.iter().any(|&n| g.tokens(n, FALLOUT) == 1));
}

