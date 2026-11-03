//! Threat sources, typed counters and the tier-2 capstones' effects (070).

use super::*;
use crate::content::{Op, ThreatSource};
use crate::modules::adversary;
use crate::modules::common::corrupted;
use crate::modules::{cards, threats};

fn pays(rules: &Rules, g: &GameState, seat: u8, t: &str) -> bool {
    rules
        .legal_actions(g)
        .iter()
        .any(|a| matches!(a, Action::Pay { seat: s, threat, .. } if *s == seat && threat == t))
}

/// Clears the whole board of every seat's presence, and puts `seat`'s on `at`.
fn only_presence(g: &mut GameState, seat: usize, at: usize) {
    for h in 0..g.hexes.len() {
        for s in 0..g.seats.len() {
            put(g, h, presence(s), 0);
        }
    }
    put(g, at, presence(seat), 1);
}

#[test]
fn a_source_in_reach_or_an_ambassador_on_one_lets_a_seat_pay() {
    let (rules, mut g) = threat_lab();
    let m = &rules.map;
    let troop = m.numbered(1, piece(1).presence);
    let far = m.numbered(0, piece(0).presence);
    only_presence(&mut g, 0, far);
    put(&mut g, troop, TROOP, 1);
    deal(&mut g, "manhunt", 1);
    set_res(&mut g, 0, Resource::Credits, 20);
    set_res(&mut g, 1, Resource::Credits, 20);
    assert!(!pays(&rules, &g, 0, "manhunt"), "no source in reach, no ambassador");
    assert!(!pays(&rules, &g, 1, "manhunt"), "no presence at all");

    put(&mut g, troop, ambassador(0), 1);
    assert!(pays(&rules, &g, 0, "manhunt"), "an ambassador on a source reaches it");
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "manhunt".into(), resource: Resource::Credits, amount: 8 });
    assert_eq!(g.tokens(troop, ambassador(0)), 1, "countering doesn't consume it");
    assert!(threats::active(&g).is_empty());

    deal(&mut g, "siege", 1);
    put(&mut g, troop, ambassador(0), 0);
    assert!(!pays(&rules, &g, 0, "siege"));
    put(&mut g, troop, presence(1), 1);
    assert!(pays(&rules, &g, 1, "siege"), "presence on or next to a source");
}

#[test]
fn a_threat_with_no_source_names_a_target_that_is_made_only_if_uncountered() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    assert!(threats::sources(&w, &g, 0, ThreatSource::Economic).is_empty());
    let setup = rules.map.numbered(0, 2);
    assert!(g.tokens(setup, FACTORY) > 0, "the lowest setup factory");
    set_pile(&mut g, "threats.deck.1", &["supply_raid", "commandeer"]);
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    assert_eq!(g.pile("threats.on.0").len(), 1, "nothing is set aside for lack of a source");
    assert_eq!(threats::target(&g, "supply_raid"), Some(setup));
    assert!(!corrupted(&g, setup), "nothing is made at the deal");

    // In reach of the target, a seat may counter; countered, nothing is made.
    only_presence(&mut g, 0, setup);
    set_res(&mut g, 0, Resource::Credits, 20);
    assert!(pays(&rules, &g, 0, "supply_raid"));
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "supply_raid".into(), resource: Resource::Credits, amount: 5 });
    assert!(threats::active(&g).is_empty());
    assert!(!corrupted(&g, setup), "countered: nothing is created");
    assert_eq!(threats::target(&g, "supply_raid"), None);

    // Uncountered: made as it executes.
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    assert_eq!(threats::target(&g, "commandeer"), Some(setup));
    pass_round(&rules, &mut g);
    assert!(corrupted(&g, setup), "uncountered: the target goes over");
    assert_eq!(g.stats.counters["threats.sources_made"], 1);
}

#[test]
fn a_troop_threat_names_the_base_and_lands_there_uncountered() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    set_pile(&mut g, "threats.deck.1", &["eviction"]);
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    let base = threats::target(&g, "eviction").expect("a base");
    assert!(crate::modules::common::is_base(&w, base));
    pass_round(&rules, &mut g);
    assert_eq!(threats::sources(&w, &g, 0, ThreatSource::Military), vec![base]);
}

#[test]
fn a_threat_with_a_source_deals() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    assert!(!threats::sources(&w, &g, 0, ThreatSource::Influence).is_empty(), "cities start corrupted");
    set_pile(&mut g, "threats.deck.1", &["smear_campaign"]);
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    assert_eq!(g.pile("threats.on.0").len(), 1);
    assert_eq!(threats::target(&g, "smear_campaign"), None, "a threat with a source names nothing");
}

#[test]
fn a_face_up_threat_shields_its_sources_until_it_is_countered() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    let city = threats::sources(&w, &g, 0, ThreatSource::Influence)[0];
    only_presence(&mut g, 0, city);
    let hold = crate::modules::board::liberate_at(&w);
    put(&mut g, city, TROOP, 1);
    let troops = Op::Damage { ambassadors: 0, troops: 1 };

    // Military: the troop can't be taken.
    deal(&mut g, "manhunt", 0);
    assert!(!adversary::can_damage(&w, &g, city, 0, 1));
    assert!(!adversary::can_destroy_all(&w, &g, city));
    set_res(&mut g, 0, Resource::Credits, 20);
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "manhunt".into(), resource: Resource::Credits, amount: 8 });
    assert!(adversary::can_damage(&w, &g, city, 0, 1), "countered, it can go");
    run_ops(&rules, &mut g, &[troops]);
    assert_eq!(g.tokens(city, TROOP), 0);

    // Influence: ambassadors at the hold wait, and free it once it is countered.
    deal(&mut g, "smear_campaign", 0);
    put(&mut g, city, ambassador(0), hold);
    assert!(corrupted(&g, city));
    set_res(&mut g, 0, Resource::Credits, 20);
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "smear_campaign".into(), resource: Resource::Credits, amount: 1 });
    assert!(g.tokens(city, ADV_CUBE) > 0, "held while it is face up");
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "smear_campaign".into(), resource: Resource::Credits, amount: 3 });
    assert_eq!(g.tokens(city, ADV_CUBE), 0, "countered, the city frees");
}

#[test]
fn a_typed_jam_reaches_only_its_type_and_only_what_the_seat_may_counter() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    let troop = rules.map.numbered(0, piece(0).presence);
    only_presence(&mut g, 0, troop);
    put(&mut g, troop, TROOP, 1);
    deal(&mut g, "manhunt", 0);
    deal(&mut g, "supply_raid", 0);
    deal(&mut g, "siege", 1);
    assert_eq!(threats::counterable(&w, &g, 0, Some(ThreatSource::Military)), vec![Key::new("manhunt")]);
    assert!(threats::counterable(&w, &g, 1, Some(ThreatSource::Military)).is_empty());
    run_ops(&rules, &mut g, &[Op::Jam { amount: 4, source: Some(ThreatSource::Military) }]);
    let need = threats::requirement(&w, &g, "manhunt")[0].1;
    assert_eq!(need, 4);
    assert_eq!(threats::requirement(&w, &g, "supply_raid")[0].1, 5);
}

#[test]
fn a_cyber_threat_reads_nodes_and_corrupted_stations() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    let spot = rules.map.numbered(1, piece(1).presence);
    put(&mut g, spot, NODE, 1);
    assert!(threats::sources(&w, &g, 1, ThreatSource::Cyber).contains(&spot));
    let station = rules.map.numbered(0, piece(0).stations[0]);
    put(&mut g, station, NODE, 0);
    put(&mut g, station, ADV_CUBE, 0);
    put(&mut g, station, STATION, 1);
    assert!(!threats::sources(&w, &g, 0, ThreatSource::Cyber).contains(&station));
    put(&mut g, station, ADV_CUBE, 1);
    assert!(threats::sources(&w, &g, 0, ThreatSource::Cyber).contains(&station));
}

#[test]
fn an_auto_counter_works_its_home_country() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    deal(&mut g, "manhunt", 0);
    deal(&mut g, "siege", 1);
    Arc::make_mut(g.piles.entry(cards::built(0)).or_default()).push("sentinel_command".into());
    let n = 8 - threats::requirement(&w, &g, "manhunt")[0].1;
    assert!(n > 0 && n < 8, "a standing counter stays below a typical counter");
    assert_eq!(threats::requirement(&w, &g, "siege")[0].1, 8, "only its home country");
    // Read, not stored: a delayed threat keeps the same cut.
    with_cx(&rules, |cx| threats::delay(&mut g, cx, "manhunt"));
    assert_eq!(threats::requirement(&w, &g, "manhunt")[0].1, 8 - n);
}

#[test]
fn project_drain_takes_compute_off_each_project_each_upkeep() {
    let (rules, mut g) = threat_lab();
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 4)).unwrap();
    Arc::make_mut(g.piles.entry(cards::built(0)).or_default()).push("black_ice_citadel".into());
    with_cx(&rules, |cx| cards::upkeep(&mut g, cx, 0));
    assert!(g.stats.counters["adversary.compute_drained"] > 0);
}
