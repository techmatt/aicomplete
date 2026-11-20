//! 167: the Unity and Discord decks in fear-card shape (Matt, 2026-11-09): board conditions, never reach;
//! prohibitions (158's firewall generalized).

use super::*;
use crate::content::Op;
use crate::modules::{adversary, plan};
use crate::modules::common::prohibit_token;
use crate::op::{CorruptSite, Per, SiteType, Removable, Step, Where};

/// r80 (ruling #53): a Corrupt prohibition puts its token on each hex its condition holds on when the card
/// resolves; the token, not the condition, keeps a covered city from corrupting (an ambassador arriving later
/// covers nothing, one leaving uncovers nothing), and the tokens come off at the end of the next adversary turn.
/// 275: the city corrupts on an arrival.
#[test]
fn a_corrupt_prohibition_stops_a_covered_city() {
    // 263: the lab plays d0 (a city's 5 from r124); here a city's 4.
    let (rules, mut g) = lab_with(&["adversary.compute"], &["adversary.corrupt_at=4"]);
    let w = rules.world();
    let m = &rules.map;
    assert_eq!(adversary::corrupt_n(&w, CorruptSite::City, 0), 4);
    // No botnet seeds an agent in the Income beat below.
    for h in 0..g.hexes.len() {
        put(&mut g, h, BOTNET, 0);
    }
    let (c, lonely) = city_with_lonely_neighbors(m);
    put(&mut g, c, ADV_AMBASSADOR, 3);
    run_ops(&rules, &mut g, &[Op::Prohibit { step: Step::Corrupt, where_: Where::ByAmbassador }]);
    assert_eq!(g.tokens(c, prohibit_token(Step::Corrupt)), 0, "no ambassador when it resolved");
    put(&mut g, lonely[1], ambassador(0), 1);
    let mut later = g.clone();
    with_cx(&rules, |cx| adversary::place(&mut later, cx, c, 1)).unwrap();
    assert_eq!(later.tokens(c, ADV_CUBE), 1, "a later ambassador covers nothing");
    run_ops(&rules, &mut g, &[Op::Prohibit { step: Step::Corrupt, where_: Where::ByAmbassador }]);
    assert_eq!(g.tokens(c, prohibit_token(Step::Corrupt)), 1);
    put(&mut g, lonely[1], ambassador(0), 0);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1)).unwrap();
    assert_eq!((g.tokens(c, ADV_CUBE), g.tokens(c, ADV_AMBASSADOR)), (0, 4), "the token holds");
    // The end of the adversary turn (the Income beat) takes the tokens off, with the firewalls.
    run(&rules, &mut g);
    assert_eq!(total(&g, prohibit_token(Step::Corrupt)), 0);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1)).unwrap();
    assert_eq!(g.tokens(c, ADV_CUBE), 1);
}

/// r82 (Matt, 2026-11-11, as at r79), 275: an Infiltrate prohibition's token blocks like a firewall: nothing lands
/// on the covered site and its agent stays in the supply; every other site takes its own 1.
#[test]
fn an_infiltrate_prohibition_keeps_the_agent_in_the_supply() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let country = 0;
    let sites = plan::infiltrate_sites(&w, &g, SiteType::City, country);
    assert!(sites.len() >= 2);
    let k = sites.len() as i64;
    put(&mut g, sites[0], ambassador(0), 1);
    run_ops(&rules, &mut g, &[Op::Prohibit { step: Step::Infiltrate, where_: Where::Ambassador }]);
    assert_eq!(g.tokens(sites[0], prohibit_token(Step::Infiltrate)), 1);
    let before: Vec<i64> = sites.iter().map(|&h| g.tokens(h, ADV_AMBASSADOR)).collect();
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| adversary::infiltrate(&mut g, cx, SiteType::City, Some(country), "test")).unwrap();
    assert_eq!(g.tokens(sites[0], ADV_AMBASSADOR), before[0], "nothing lands on the covered site");
    assert_eq!(g.tokens(sites[0], ambassador(0)), 1, "and its ambassador stands");
    for (i, &h) in sites.iter().enumerate().skip(1) {
        assert_eq!(g.tokens(h, ADV_AMBASSADOR), before[i] + 1, "each other site takes its own 1");
    }
    assert_eq!(g.var(SUPPLY), supply - (k - 1), "the covered site's agent stays in the supply");
    assert_eq!(g.stats.counters["adversary.infiltrate_blocked"], 1);
}

/// "Remove 1 agent from each hex on or next to an ambassador": every such hex, no other.
#[test]
fn remove_where_acts_on_each_covered_hex() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (c, lonely) = city_with_lonely_neighbors(m);
    put(&mut g, c, ambassador(0), 1);
    put(&mut g, lonely[0], ADV_AMBASSADOR, 2);
    let far = (0..g.hexes.len())
        .find(|&h| h != c && !m.hexes[c].neighbors.contains(&h) && !m.hexes[h].neighbors.contains(&c))
        .unwrap();
    put(&mut g, far, ADV_AMBASSADOR, 2);
    run_ops(&rules, &mut g, &[Op::RemoveWhere { pieces: Removable::Agents, count: 1, per: Per::Hex, where_: Where::ByAmbassador }]);
    assert_eq!(g.tokens(lonely[0], ADV_AMBASSADOR), 1);
    assert_eq!(g.tokens(far, ADV_AMBASSADOR), 2, "a hex away from every ambassador keeps its agents");
}

/// "For each corrupted city (at most N)": once per corrupted city, capped.
#[test]
fn per_site_counts_the_board() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let corrupted = plan::sites_where(&w, &g, Where::CorruptedCity).len() as i64;
    let before = g.resource(0, Resource::Credits);
    let gain = vec![Op::Gain { resource: Resource::Credits, amount: 1 }];
    run_ops(&rules, &mut g, &[Op::PerSite { where_: Where::CorruptedCity, max: 99, then: gain.clone() }]);
    assert_eq!(g.resource(0, Resource::Credits), before + corrupted);
    run_ops(&rules, &mut g, &[Op::PerSite { where_: Where::CorruptedCity, max: 0, then: gain }]);
    assert_eq!(g.resource(0, Resource::Credits), before + corrupted, "max 0 is a blank");
}

/// The shape (Matt, 2026-11-09): no Unity or Discord card prints reach or "one player", and each tier acts by a
/// board condition: its ops are the board-conditioned ones.
#[test]
fn the_decks_name_board_conditions() {
    for c in &crate::content().alignment {
        for t in &c.tiers {
            let text = t.text.to_lowercase();
            assert!(!text.contains("reach") && !text.contains("one player"), "{}: {}", c.id, t.text);
            for e in &t.effects {
                assert!(
                    matches!(
                        e,
                        Op::PerSite { .. }
                            | Op::RemoveWhere { .. }
                            | Op::Prohibit { .. }
                            | Op::Create { .. }
                            | Op::CreateSite { .. }
                            | Op::Escalate { .. }
                            | Op::ThreatAtNode { .. }
                            | Op::AdversaryBuild
                    ) || matches!(e, Op::Fallout { at, .. } if *at != crate::op::FalloutAt::Location),
                    "{}: {e:?} names no board condition",
                    c.id
                );
            }
        }
    }
}

/// 169: a pending prohibition shows as the printed marker would: a labeled global naming the step, the round of the
/// adversary turn it acts in, and every condition (its raw vars were an unlabeled bit mask).
#[test]
fn a_pending_prohibition_is_labeled_in_the_view() {
    let (rules, mut g) = lab(&[]);
    let none = rules.view(&g);
    assert!(!none.globals.iter().any(|v| v.key.starts_with("adversary.prohibit")));
    run_ops(&rules, &mut g, &[Op::Prohibit { step: Step::Infiltrate, where_: Where::ByAmbassador }]);
    run_ops(&rules, &mut g, &[Op::Prohibit { step: Step::Infiltrate, where_: Where::CorruptedCity }]);
    let v = rules.view(&g);
    let p = v.globals.iter().find(|v| v.key == "adversary.prohibit_infiltrate").expect("the Infiltrate prohibition");
    assert_eq!(p.value, g.var(plan::prohibit_keys(Step::Infiltrate).0));
    assert!(p.label.contains("Infiltrate"), "{}", p.label);
    for wh in [Where::ByAmbassador, Where::CorruptedCity] {
        assert!(p.doc.contains(wh.words()), "{}", p.doc);
    }
    assert!(!v.globals.iter().any(|v| v.key.contains("prohibit.")), "raw vars are bookkeeping");
}

/// 169: Incursion's extra threats, one for each corrupted city, go one per country in turn across the whole batch
/// (each city's own deal began again at country 0, so every one landed in country 0).
#[test]
fn extra_threats_for_each_site_go_one_per_country() {
    use crate::modules::{effects, threats};
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    assert_eq!(m.countries.len(), 2);
    for h in cities(m).into_iter().take(2) {
        put(&mut g, h, ADV_CUBE, 1);
    }
    let per_country = |g: &GameState| [0, 1].map(|c| threats::active(g).iter().filter(|(_, k)| *k == c).count());
    let before = per_country(&g);
    let incursion = Op::PerSite {
        where_: Where::CorruptedCity,
        max: 4,
        then: vec![Op::DealThreats { count: 1, tier: None, scope: None, each_country: false }],
    };
    with_cx(&rules, |cx| effects::run(&mut g, cx, &[incursion], &mut effects::Ctx::team().named("discord"))).unwrap();
    let after = per_country(&g);
    assert_eq!([after[0] - before[0], after[1] - before[1]], [1, 1]);
}
