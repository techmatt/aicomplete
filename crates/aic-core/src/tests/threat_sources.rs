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

/// Clears the whole board of every seat's node, and puts `seat`'s on `at`.
fn only_node(g: &mut GameState, seat: usize, at: usize) {
    for h in 0..g.hexes.len() {
        for s in 0..g.seats.len() {
            put(g, h, node(s), 0);
        }
    }
    put(g, at, node(seat), 1);
}

#[test]
fn a_source_in_reach_or_an_ambassador_on_one_lets_a_seat_pay() {
    let (rules, mut g) = threat_lab();
    let m = &rules.map;
    let troop = m.numbered(1, piece(1).node);
    let far = m.numbered(0, piece(0).node);
    only_node(&mut g, 0, far);
    put(&mut g, troop, TROOP, 1);
    deal(&mut g, "manhunt", 1);
    set_res(&mut g, 0, Resource::Credits, 20);
    set_res(&mut g, 1, Resource::Credits, 20);
    assert!(!pays(&rules, &g, 0, "manhunt"), "no source in reach, no ambassador");
    assert!(!pays(&rules, &g, 1, "manhunt"), "no nodes at all");

    put(&mut g, troop, ambassador(0), 1);
    assert!(pays(&rules, &g, 0, "manhunt"), "an ambassador on a source reaches it");
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "manhunt".into(), resource: Resource::Credits, amount: 8 });
    assert_eq!(g.tokens(troop, ambassador(0)), 1, "countering doesn't consume it");
    assert!(threats::active(&g).is_empty());

    deal(&mut g, "siege", 1);
    put(&mut g, troop, ambassador(0), 0);
    assert!(!pays(&rules, &g, 0, "siege"));
    put(&mut g, troop, node(1), 1);
    assert!(pays(&rules, &g, 1, "siege"), "node on or next to a source");
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
    only_node(&mut g, 0, setup);
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
    only_node(&mut g, 0, city);
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
    deal(&mut g, "defection", 0);
    put(&mut g, city, ambassador(0), hold);
    assert!(corrupted(&g, city));
    set_res(&mut g, 0, Resource::Credits, 20);
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "defection".into(), resource: Resource::Credits, amount: 1 });
    assert!(g.tokens(city, ADV_CUBE) > 0, "held while it is face up");
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "defection".into(), resource: Resource::Credits, amount: 3 });
    assert_eq!(g.tokens(city, ADV_CUBE), 0, "countered, the city frees");
}

#[test]
fn a_typed_jam_reaches_only_its_type_and_only_what_the_seat_may_counter() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    let troop = rules.map.numbered(0, piece(0).node);
    only_node(&mut g, 0, troop);
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
fn a_cyber_threat_reads_botnets_and_corrupted_stations() {
    let (rules, mut g) = threat_lab();
    let w = rules.world();
    let spot = rules.map.numbered(1, piece(1).node);
    put(&mut g, spot, BOTNET, 1);
    assert!(threats::sources(&w, &g, 1, ThreatSource::Cyber).contains(&spot));
    let station = rules.map.numbered(0, piece(0).stations[0]);
    put(&mut g, station, BOTNET, 0);
    put(&mut g, station, ADV_CUBE, 0);
    put(&mut g, station, STATION, 1);
    assert!(!threats::sources(&w, &g, 0, ThreatSource::Cyber).contains(&station));
    put(&mut g, station, ADV_CUBE, 1);
    assert!(threats::sources(&w, &g, 0, ThreatSource::Cyber).contains(&station));
}

/// 119: Autonomous Defense Grid's standing counter pinned to r46's 5 (cyber).
const GRID: &str = r#"cards.autonomous_defense_grid.passives=[{"op":"auto_counter","source":"cyber","amount":5}]"#;

/// r48 (D54): "Cyber threats you can pay toward need 5 less": every cyber threat its
/// holder may counter, in any country (not only its home) and on the shared row.
#[test]
fn an_auto_counter_cuts_the_threats_its_holder_can_pay_toward() {
    let (rules, mut g) = threat_lab_with(&[GRID]);
    let w = rules.world();
    // Two cyber threats printing 7 credits, one by each country.
    deal(&mut g, "station_seizure", 0);
    deal(&mut g, "fifth_column", 1);
    Arc::make_mut(g.piles.entry(cards::built(0)).or_default()).push("autonomous_defense_grid".into());
    for (t, c) in [("station_seizure", 0), ("fifth_column", 1)] {
        let may = threats::may_counter(&w, &g, 0, t, Some(c));
        let need = threats::requirement(&w, &g, t)[0].1;
        assert_eq!(need, if may { 2 } else { 7 }, "{t}: may pay toward it {may}");
    }
    // A cyber source in seat 0's reach in country 1 opens that threat to it, and the cut.
    let home = rules.map.numbered(0, piece(0).node);
    only_node(&mut g, 0, home);
    let far = rules.map.numbered(1, piece(1).node);
    put(&mut g, far, node(0), 1);
    put(&mut g, far, BOTNET, 1);
    assert!(threats::may_counter(&w, &g, 0, "fifth_column", Some(1)));
    assert_eq!(threats::requirement(&w, &g, "fifth_column")[0].1, 2, "abroad too");
    // Read, not stored: a delayed threat keeps the same cut.
    with_cx(&rules, |cx| threats::delay(&mut g, cx, "fifth_column"));
    assert_eq!(threats::requirement(&w, &g, "fifth_column")[0].1, 2);
    // The shared row: every seat may pay toward a row threat, so the cut applies.
    let (rules, mut g) = threat_lab_with(&[GRID, "threats.variant=shared_row"]);
    let w = rules.world();
    set_pile(&mut g, "threats.row", &["station_seizure"]);
    Arc::make_mut(g.piles.entry(cards::built(0)).or_default()).push("autonomous_defense_grid".into());
    assert_eq!(threats::requirement(&w, &g, "station_seizure")[0].1, 2, "on the row");
}

/// 118: a threat a standing counter pays in full is countered, not left face up at 0
/// holding its sources until execute (r48: Blackout's counter is 1 energy).
#[test]
fn a_threat_an_auto_counter_pays_in_full_is_countered() {
    let (rules, mut g) = threat_lab_with(&[GRID, "threats.variant=shared_row"]);
    let w = rules.world();
    set_pile(&mut g, "threats.row", &["blackout"]);
    Arc::make_mut(g.piles.entry(cards::built(0)).or_default()).push("autonomous_defense_grid".into());
    assert!(threats::requirement(&w, &g, "blackout").iter().all(|(_, n)| *n == 0), "{:?}", threats::requirement(&w, &g, "blackout"));
    with_cx(&rules, |cx| threats::complete_paid(&mut g, cx));
    assert!(!threats::active(&g).iter().any(|(t, _)| t == "blackout"), "countered at once");
}

#[test]
fn project_drain_takes_compute_off_each_project_each_upkeep() {
    // No card prints a standing project drain since r42: a building is given it here.
    let mut c = lab_config(&[ACTIONS, "threats.execute"]);
    c.apply_override(r#"cards.darknet_exchange.passives=[{"op":"project_drain","amount":3}]"#).unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(3);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 4)).unwrap();
    Arc::make_mut(g.piles.entry(cards::built(0)).or_default()).push("darknet_exchange".into());
    with_cx(&rules, |cx| cards::upkeep(&mut g, cx, 0));
    assert!(g.stats.counters["adversary.compute_drained"] > 0);
}
