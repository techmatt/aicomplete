//! 158: the tier-2 end-game ops (Matt, 2026-11-09): convert, the firewall, "with N X tags, also".

use super::*;
use crate::content::{Op, Tag};
use crate::modules::common::{corrupted, FIREWALL};
use crate::modules::{adversary, effects, plan};
use crate::op::CorruptSite;


/// Amendment 6949: a converted agent leaves the stack (lowering the city's Liberate bill) and the adversary's
/// supply takes it back; 178 (#32): it becomes the seat's ambassador where it stood.
#[test]
fn convert_makes_each_agent_your_ambassador_where_it_stood() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let (c, me, other, _) = by_a_city(&mut g, m);
    put(&mut g, c, ADV_AMBASSADOR, 2);
    put(&mut g, other, ADV_AMBASSADOR, 1);
    assert_eq!(effects::convert_cities(&w, &g, 0), vec![c]);
    let supply = g.var(SUPPLY);
    run_ops(&rules, &mut g, &[Op::Convert { max: 3 }]);
    assert_eq!((g.tokens(c, ADV_AMBASSADOR), g.tokens(other, ADV_AMBASSADOR)), (0, 0));
    assert_eq!((g.tokens(me, ambassador(0)), g.tokens(c, ambassador(0)), g.tokens(other, ambassador(0))), (0, 2, 1));
    assert_eq!(g.var(SUPPLY), supply + 3);
    assert!(effects::convert_cities(&w, &g, 0).is_empty(), "nothing left to convert");
}

/// 178: an agent the conversion left on a hex meets the new ambassador there, 1 for 1.
#[test]
fn an_agent_left_behind_meets_the_converted_one() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (c, _, _, _) = by_a_city(&mut g, m);
    put(&mut g, c, ADV_AMBASSADOR, 3);
    run_ops(&rules, &mut g, &[Op::Convert { max: 1 }]);
    assert_eq!((g.tokens(c, ADV_AMBASSADOR), g.tokens(c, ambassador(0))), (1, 0));
}

/// 158: a firewall stops placements on its hex, keeps it off the Rally's sites, and a Corrupt line can't pull
/// into it from next door.
#[test]
fn a_firewall_stops_arrivals_and_pulls() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let (c, _, a, _) = by_a_city(&mut g, m);
    put(&mut g, c, ADV_AMBASSADOR, 1);
    put(&mut g, a, ADV_AMBASSADOR, 3);
    let n = adversary::corrupt_n(&w, CorruptSite::City, 0);
    assert!(plan::corrupt_plan(&w, &g, CorruptSite::City, n, None).iter().any(|(t, _)| *t == c));
    put(&mut g, c, FIREWALL, 1);
    assert!(!plan::corrupt_plan(&w, &g, CorruptSite::City, n, None).iter().any(|(t, _)| *t == c));
    let country = m.hexes[c].country as usize;
    // 175: a firewalled site is still a Rally site; its share stays in the supply.
    assert!(plan::rally_sites(&w, &g, crate::op::RallyAt::City, country).contains(&c));
    assert!(!plan::next_rally(&w, &g).iter().any(|&(h, _)| h == c));
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 2)).unwrap();
    assert_eq!(g.tokens(c, ADV_AMBASSADOR), 1, "no arrival on a firewalled hex");
}

/// 158: "with N X tags, also": nothing below the count, and it isn't offered alone.
#[test]
fn if_tags_needs_its_count() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let ops = [Op::IfTags { tag: Tag::Diplomacy, count: 3, then: vec![Op::GainAmbassadors { count: 2 }] }];
    assert!(!effects::feasible(&w, &g, 0, &ops));
    let before = g.resource(0, Resource::Ambassadors);
    run_ops(&rules, &mut g, &ops);
    assert_eq!(g.resource(0, Resource::Ambassadors), before);
}

/// Each country's neutral station on the board (the lab sets up only the corrupted ones), in country order.
fn neutral_stations(rules: &Rules, g: &mut GameState) -> Vec<usize> {
    let hexes: Vec<usize> = (0..2).map(|c| rules.map.numbered(c, piece(c).neutral_station)).collect();
    for &h in &hexes {
        put(g, h, STATION, 1);
    }
    hexes
}

/// 185 (Matt's ruling on 184's D77, candidate): a firewall guards Station Coup's flip (a project's, the
/// lowest-numbered open station): a firewalled station is skipped, and the next one flips.
#[test]
fn a_firewall_guards_station_coups_flip() {
    let (rules, mut g) = lab(&[]);
    let open = neutral_stations(&rules, &mut g);
    for &h in &open {
        put(&mut g, h, FIREWALL, 1);
    }
    let flip = [Op::FlipStation { each_country: true, force: false }];
    let mut ctx = effects::Ctx::team().named("project");
    with_cx(&rules, |cx| effects::run(&mut g, cx, &flip, &mut ctx)).unwrap();
    assert!(open.iter().all(|&h| !corrupted(&g, h)), "nothing firewalled flips");
    assert!(g.stats.counters["adversary.guard_held.flip"] >= open.len() as i64);
    put(&mut g, open[1], FIREWALL, 0);
    let mut ctx = effects::Ctx::team().named("project");
    with_cx(&rules, |cx| effects::run(&mut g, cx, &flip, &mut ctx)).unwrap();
    assert!(corrupted(&g, open[1]), "the open one flips");
}

/// 185 (D77): a Corrupt prohibition or a firewall guards a forced flip (Lab Takeover, Station Seizure).
#[test]
fn a_firewall_guards_a_forced_flip() {
    let (rules, mut g) = lab(&[]);
    let open = neutral_stations(&rules, &mut g);
    for &h in &open {
        put(&mut g, h, FIREWALL, 1);
    }
    let flip = [Op::FlipStation { each_country: true, force: true }];
    with_cx(&rules, |cx| effects::run(&mut g, cx, &flip, &mut effects::Ctx::team())).unwrap();
    assert!(open.iter().all(|&h| !corrupted(&g, h)));
    assert!(g.stats.counters["adversary.guard_held.force_flip"] >= open.len() as i64);
    put(&mut g, open[0], FIREWALL, 0);
    with_cx(&rules, |cx| effects::run(&mut g, cx, &flip, &mut effects::Ctx::team())).unwrap();
    assert!(corrupted(&g, open[0]), "the one choice left flips");
}

/// 185 (D77): a firewall guards Overrun's city ("the city here is corrupted").
#[test]
fn a_firewall_guards_overrun() {
    let (rules, mut g) = threat_lab();
    let m = &rules.map;
    let city = cities(m).into_iter().find(|&h| m.hexes[h].country == 0 && !corrupted(&g, h)).unwrap();
    deal_at(&mut g, "overrun", 0, &[city]);
    put(&mut g, city, FIREWALL, 1);
    let ops = [Op::CorruptCity { at: Some(crate::op::Spot::City) }];
    let overrun = |g: &mut GameState| {
        let mut ctx = effects::Ctx::threat(Some(0));
        ctx.threat = Some(Key::new("overrun"));
        with_cx(&rules, |cx| effects::run(g, cx, &ops, &mut ctx)).unwrap();
    };
    overrun(&mut g);
    assert!(!corrupted(&g, city));
    assert_eq!(g.stats.counters["adversary.guard_held.corrupt_city"], 1);
    put(&mut g, city, FIREWALL, 0);
    overrun(&mut g);
    assert!(corrupted(&g, city));
}
