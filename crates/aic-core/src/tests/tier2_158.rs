//! 158: the tier-2 end-game ops (Matt, 2026-11-09): convert, the firewall, "with N X tags, also".

use super::*;
use crate::content::{Op, Tag};
use crate::modules::common::FIREWALL;
use crate::modules::{adversary, effects, plan};
use crate::op::CorruptSite;

/// A city, the seat's node on a lonely neighbor (its reach takes in only this city), and two more lonely
/// neighbors.
fn by_a_city(g: &mut GameState, m: &Map) -> (usize, usize, usize, usize) {
    let (c, lonely) = city_with_lonely_neighbors(m);
    put(g, lonely[0], node(0), 1);
    (c, lonely[0], lonely[1], lonely[2])
}

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
