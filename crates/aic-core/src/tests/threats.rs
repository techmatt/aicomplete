//! Threats (135): who is near one, pooled and whole counters, execution.

use super::*;
use crate::content::Tag;
use crate::modules::{cards, threats};

/// Clears every seat's nodes off the board.
fn no_nodes(g: &mut GameState) {
    for h in 0..g.hexes.len() {
        for s in 0..g.seats.len() {
            put(g, h, node(s), 0);
        }
    }
}

fn legal_has(rules: &Rules, g: &GameState, f: impl Fn(&Action) -> bool) -> bool {
    rules.legal_actions(g).iter().any(f)
}

fn pays(rules: &Rules, g: &GameState, seat: u8, t: &str) -> bool {
    legal_has(rules, g, |a| matches!(a, Action::Pay { seat: s, threat, .. } if *s == seat && threat == t))
}

fn counted(g: &GameState, key: &str) -> i64 {
    g.stats.counters.get(key).copied().unwrap_or(0)
}

/// A land hex in another country than `c`, not within 1 of `h`.
fn far_from(m: &Map, c: usize, h: usize) -> usize {
    (0..m.hexes.len())
        .find(|&x| m.hexes[x].country as usize != c && kind(m, x) != HexKind::Ocean && !m.hexes[h].neighbors.contains(&x))
        .unwrap()
}

#[test]
fn only_seats_near_a_threat_may_pay_and_pooled_credits_counter_it() {
    // 142: Eviction pinned to one 6-credit counter, so the pooled sums below
    // read no reprice and no cheaper option.
    let (rules, mut g) = threat_lab_with(&[r#"threats.eviction.counters=[[{"resource":"credits","amount":6}]]"#]);
    let m = &rules.map;
    let city = cities(m).into_iter().find(|&h| m.hexes[h].country == 0).unwrap();
    let next = m.hexes[city].neighbors[0];
    no_nodes(&mut g);
    put(&mut g, next, node(0), 1);
    put(&mut g, far_from(m, 0, city), node(1), 1);
    deal_at(&mut g,"eviction", 0, &[city]);
    for s in 0..2 {
        set_res(&mut g, s, Resource::Credits, 10);
    }
    assert!(pays(&rules, &g, 0, "eviction"), "a node next to its token");
    assert!(!pays(&rules, &g, 1, "eviction"), "far away");
    let pay = |seat, amount| Action::Pay { seat, threat: "eviction".into(), resource: Resource::Credits, amount };
    assert!(rules.apply_mut(&mut g, &pay(1, 1), None).is_err());
    put(&mut g, city, ambassador(1), 1);
    assert!(pays(&rules, &g, 1, "eviction"), "an ambassador on its token");
    assert!(rules.apply_mut(&mut g, &pay(0, 7), None).is_err(), "more than its 6 credits");
    act(&rules, &mut g, pay(0, 4));
    assert_eq!(threats::cheapest_resource_need(&rules.world(), &g, "eviction"), 2);
    act(&rules, &mut g, pay(1, 2));
    assert!(threats::active(&g).is_empty());
    assert!(threats::tokens(&g, &Key::new("eviction")).is_empty(), "its token is off");
    assert_eq!((g.resource(0, Resource::Credits), g.resource(1, Resource::Credits)), (6, 8));
    assert_eq!(counted(&g, "threats.countered_by.credits"), 1);
    assert_eq!(counted(&g, "threats.countered_by.t1.credits"), 1);
    assert_eq!(counted(&g, "threats.countered.t1"), 1);
}

/// 145: a [city, coastal city] threat whose city token would take the only
/// clean coastal city places its coastal token there and the city token inland.
#[test]
fn a_city_token_leaves_the_only_coastal_city_to_the_coastal_token() {
    let (rules, mut g) = threat_lab();
    let m = &rules.map;
    let c = (0..m.countries.len())
        .find(|&c| {
            let cs: Vec<usize> = cities(m).into_iter().filter(|&h| m.hexes[h].country as usize == c).collect();
            cs.iter().any(|&h| m.hexes[h].coastal) && cs.iter().any(|&h| !m.hexes[h].coastal)
        })
        .expect("a country with a coastal and an inland city");
    let here: Vec<usize> = cities(m).into_iter().filter(|&h| m.hexes[h].country as usize == c).collect();
    let coastal = *here.iter().find(|&&h| m.hexes[h].coastal).unwrap();
    let inland = *here.iter().find(|&&h| !m.hexes[h].coastal).unwrap();
    // Every other city corrupted; the coastal one under more pressure.
    for &h in &here {
        if h != coastal && h != inland {
            put(&mut g, h, ADV_CUBE, 1);
        }
    }
    put(&mut g, coastal, ADV_AMBASSADOR, 2);
    set_pile(&mut g, "threats.deck.2", &["propaganda_ministry"]);
    with_cx(&rules, |cx| threats::deal_tier(&mut g, cx, 2, &[crate::content::PullTag::Any], c)).unwrap();
    let mut got = threats::tokens(&g, &Key::new("propaganda_ministry"));
    got.sort();
    let mut want = vec![coastal, inland];
    want.sort();
    assert_eq!(got, want, "both tokens on the board");
}

/// 145: the share paid reads each option in its own currencies: 1 credit
/// toward 3 energy or 15 credits is 1/15 paid, not a quarter.
#[test]
fn the_share_paid_reads_each_option_in_its_own_currency() {
    let (rules, mut g) = threat_lab_with(&[
        r#"threats.eviction.counters=[[{"resource":"energy","amount":3}],[{"resource":"credits","amount":15}]]"#,
        "adversary.counter_credit_pct=100",
    ]);
    let m = &rules.map;
    let city = cities(m).into_iter().find(|&h| m.hexes[h].country == 0).unwrap();
    no_nodes(&mut g);
    put(&mut g, city, node(0), 1);
    deal_at(&mut g, "eviction", 0, &[city]);
    let w = rules.world();
    let need = |g: &GameState| threats::requirement(&w, g, "eviction");
    let (energy, credits) = (need(&g)[0][0], need(&g)[1][0]);
    let amount = |i: &crate::content::CounterItem| match i {
        crate::content::CounterItem::Pay { amount, .. } => *amount,
        _ => unreachable!(),
    };
    assert_eq!(threats::paid_share(&w, &g, "eviction"), 0.0);
    set_res(&mut g, 0, Resource::Credits, 10);
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "eviction".into(), resource: Resource::Credits, amount: 1 });
    let share = threats::paid_share(&w, &g, "eviction");
    assert!((share - 1.0 / amount(&credits) as f64).abs() < 1e-9, "{share}, energy {}", amount(&energy));
}

#[test]
fn counter_pct_scales_every_resource_item_by_tier_rounding_up() {
    // 142: Eviction (tier 1) pinned to 3 energy or 15 credits; at 150% it asks
    // 5 energy (4.5 up) and 23 credits (22.5 up), at 125% 4 (3.75 up) and 19 (18.75 up).
    const EVICTION: &str = r#"threats.eviction.counters=[[{"resource":"energy","amount":3}],[{"resource":"credits","amount":15}]]"#;
    for (pct, want) in [("150,100,100", (5, 23)), ("125,100,100", (4, 19))] {
        let (rules, g) =
            threat_lab_with(&[EVICTION, "adversary.counter_credit_pct=100", &format!("adversary.counter_pct={pct}")]);
        let need = threats::requirement(&rules.world(), &g, "eviction");
        let of = |r: Resource| {
            need.iter().flatten().find_map(|i| match i {
                crate::content::CounterItem::Pay { resource, amount } if *resource == r => Some(*amount),
                _ => None,
            })
        };
        assert_eq!((of(Resource::Energy), of(Resource::Credits)), (Some(want.0), Some(want.1)), "{pct}");
    }
}

#[test]
fn counter_with_tags_sacrifices_cards_in_play() {
    let (rules, mut g) =
        threat_lab_with(&[r#"threats.supply_raid.counters=[[{"resource":"chips","amount":4}],[{"sacrifice":"industry","amount":2}]]"#]);
    let w = rules.world();
    let factory = (0..rules.map.hexes.len()).find(|&h| g.tokens(h, FACTORY) > 0).unwrap();
    let c = rules.map.hexes[factory].country as usize;
    no_nodes(&mut g);
    put(&mut g, factory, node(0), 1);
    deal_at(&mut g,"supply_raid", c, &[factory]);
    // 142: Supply Raid's second option (pinned) sacrifices 2 industry tags.
    let industry: Vec<&str> = w
        .content
        .cards
        .iter()
        .filter(|d| d.tags.contains(&Tag::Industry))
        .map(|d| d.id.as_str())
        .take(2)
        .collect();
    let with = |option| Action::CounterWith { seat: 0, threat: "supply_raid".into(), option };
    set_pile(&mut g, &cards::played(0), &industry[..1]);
    assert!(!legal_has(&rules, &g, |a| *a == with(1)), "1 industry tag in play");
    set_pile(&mut g, &cards::played(0), &industry);
    assert!(legal_has(&rules, &g, |a| *a == with(1)), "2 industry tags in play");
    act(&rules, &mut g, with(1));
    assert!(threats::active(&g).is_empty());
    assert!(g.pile(&cards::played(0)).is_empty(), "both sacrificed");
    assert_eq!(counted(&g, "threats.countered_by.tags"), 1);
}

#[test]
fn counter_with_nodes_removes_nodes_near_it_never_the_last() {
    // 142: no printed threat has a nodes counter (refused while `threat_nodes`
    // takes nodes; threat_lab sets it 0), so the test gives Orbital Strike one.
    let (rules, mut g) =
        threat_lab_with(&[r#"threats.orbital_strike.counters=[[{"resource":"energy","amount":3}],[{"nodes_near":1}]]"#]);
    let m = &rules.map;
    let base = (0..m.hexes.len()).find(|&h| kind(m, h) == HexKind::Base).unwrap();
    let c = m.hexes[base].country as usize;
    no_nodes(&mut g);
    put(&mut g, base, node(0), 1);
    deal_at(&mut g,"orbital_strike", c, &[base]);
    let with = Action::CounterWith { seat: 0, threat: "orbital_strike".into(), option: 1 };
    assert!(!legal_has(&rules, &g, |a| *a == with), "its last node");
    put(&mut g, far_from(m, c, base), node(0), 1);
    assert!(legal_has(&rules, &g, |a| *a == with));
    act(&rules, &mut g, with);
    assert!(threats::active(&g).is_empty());
    assert_eq!(g.tokens(base, node(0)), 0);
    assert_eq!(counted(&g, "board.node_lost.counter"), 1);
    assert_eq!(counted(&g, "threats.countered_by.nodes"), 1);
}

#[test]
fn an_uncountered_threat_hits_the_seats_near_it_and_sways_discord() {
    let (rules, mut g) =
        threat_lab_with(&[r#"threats.supply_raid.effects=[{"op":"players_lose","resource":"energy","amount":2}]"#]);
    let factory = (0..rules.map.hexes.len()).find(|&h| g.tokens(h, FACTORY) > 0).unwrap();
    let c = rules.map.hexes[factory].country as usize;
    no_nodes(&mut g);
    put(&mut g, factory, node(0), 1);
    // In the country, but not near the token.
    let far = rules.map.countries[c]
        .hexes
        .iter()
        .copied()
        .find(|&h| h != factory && !rules.map.hexes[factory].neighbors.contains(&h))
        .unwrap();
    put(&mut g, far, node(1), 1);
    deal_at(&mut g,"supply_raid", c, &[factory]);
    for s in 0..2 {
        set_res(&mut g, s, Resource::Energy, 10);
    }
    let discord = g.var("alignment.discord");
    for s in 0..g.seats.len() as u8 {
        rules.apply_mut(&mut g, &Action::Pass { seat: s }, None).unwrap();
    }
    assert!(g.pending.is_none());
    // Supply Raid, pinned to take 2 energy from each player near it.
    assert_eq!((g.resource(0, Resource::Energy), g.resource(1, Resource::Energy)), (8, 10));
    assert_eq!(g.var("alignment.discord"), discord + 1);
    assert_eq!(counted(&g, "threats.uncountered"), 1);
    assert!(g.pile("threats.discard.1").contains(&Key::new("supply_raid")));
}

#[test]
fn an_uncountered_threat_takes_its_tiers_nodes_from_each_player_near_it() {
    // 138: `threat_nodes` 1 for tier 1: the near seat loses its node near the
    // token (it holds another, so it is not its last); the far seat keeps its.
    let (rules, mut g) = threat_lab_with(&["adversary.threat_nodes=1,1,2"]);
    let factory = (0..rules.map.hexes.len()).find(|&h| g.tokens(h, FACTORY) > 0).unwrap();
    let c = rules.map.hexes[factory].country as usize;
    no_nodes(&mut g);
    let far: Vec<usize> = rules.map.countries[c]
        .hexes
        .iter()
        .copied()
        .filter(|&h| h != factory && !rules.map.hexes[factory].neighbors.contains(&h))
        .take(2)
        .collect();
    put(&mut g, factory, node(0), 1);
    put(&mut g, far[0], node(0), 1);
    put(&mut g, far[1], node(1), 1);
    deal_at(&mut g,"supply_raid", c, &[factory]);
    for s in 0..g.seats.len() as u8 {
        rules.apply_mut(&mut g, &Action::Pass { seat: s }, None).unwrap();
    }
    assert!(g.pending.is_none());
    assert_eq!(g.tokens(factory, node(0)), 0, "the near node goes");
    assert_eq!((g.tokens(far[0], node(0)), g.tokens(far[1], node(1))), (1, 1));
}
