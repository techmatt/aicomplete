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

/// r80 (ruling #46): each token, in printed order, goes on the site of its type (not the adversary's) with the
/// most agents on its own hex, ties the lowest number; agents next door don't count, and two tokens never share a
/// hex: a [city, coastal city] threat whose city token takes the only clean coastal city leaves its coastal token
/// off (145's reorder is gone).
#[test]
fn a_location_token_goes_where_most_agents_stand_on_the_hex() {
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
    for &h in &here {
        if h != coastal && h != inland {
            put(&mut g, h, ADV_CUBE, 1);
        }
        put(&mut g, h, ADV_AMBASSADOR, 0);
    }
    put(&mut g, coastal, ADV_AMBASSADOR, 1);
    // Three agents beside the inland city, none on it: they don't count.
    let beside = m.hexes[inland].neighbors.iter().copied().find(|&x| kind(m, x) != HexKind::Ocean && kind(m, x) != HexKind::City);
    if let Some(x) = beside {
        put(&mut g, x, ADV_AMBASSADOR, 3);
    }
    set_pile(&mut g, "threats.deck.2", &["propaganda_ministry"]);
    with_cx(&rules, |cx| threats::deal_tier(&mut g, cx, 2, &[crate::content::PullTag::Any], c)).unwrap();
    let got = threats::tokens(&g, &Key::new("propaganda_ministry"));
    assert_eq!(got, vec![coastal], "the city token takes the coastal city; the coastal token has no site left");
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
    for (pct, want) in [("150,100", (5, 23)), ("125,100", (4, 19))] {
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
    // 146: credits first so the sacrifice is option 1 (a chips counter would share
    // Supply Raid's chips requisition).
    let (rules, mut g) =
        threat_lab_with(&[r#"threats.supply_raid.counters=[[{"resource":"credits","amount":15}],[{"sacrifice":"industry","amount":2}]]"#]);
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
    // 142: no printed threat has a nodes counter, so the test gives Orbital
    // Strike one (its troops destroy no node, so no shared currency).
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

/// 146: a land hex exactly `d` from `from`, not in `avoid`.
fn at_distance(m: &Map, from: usize, d: i32, avoid: &[usize]) -> usize {
    (0..m.hexes.len())
        .find(|&h| m.distance(from, h) == d && kind(m, h) != HexKind::Ocean && !avoid.contains(&h))
        .unwrap_or_else(|| panic!("no land hex {d} from {from}"))
}

/// 146: a lab with Supply Raid (one factory token) pinned to `effects` and no
/// node on the board: (rules, game, its factory hex, its country).
fn supply_raid_lab(effects: &str, sets: &[&str]) -> (Rules, GameState, usize, usize) {
    let pin = format!("threats.supply_raid.effects={effects}");
    let mut all = vec![pin.as_str()];
    all.extend_from_slice(sets);
    let (rules, mut g) = threat_lab_with(&all);
    let factory = (0..rules.map.hexes.len()).find(|&h| g.tokens(h, FACTORY) > 0).unwrap();
    let c = rules.map.hexes[factory].country as usize;
    no_nodes(&mut g);
    (rules, g, factory, c)
}

const LOSE_2_ENERGY: &str = r#"[{"op":"players_lose","resource":"energy","amount":2}]"#;

fn energies(g: &GameState) -> Vec<i64> {
    (0..g.seats.len()).map(|s| g.resource(s, Resource::Energy)).collect()
}

fn pass_all(rules: &Rules, g: &mut GameState) {
    for s in 0..g.seats.len() as u8 {
        rules.apply_mut(g, &Action::Pass { seat: s }, None).unwrap();
    }
}

/// 146: an uncountered threat's harm hits one player, the nearest by node
/// (hex distance to a token), whether or not that seat is near; a seat near it
/// but farther is not hit, and no Discord sways beyond what it prints.
#[test]
fn an_uncountered_threat_hits_only_the_nearest_seat() {
    // Seat 1 on the token, seat 0 next to it (near: it may pay).
    let (rules, mut g, factory, c) = supply_raid_lab(LOSE_2_ENERGY, &[]);
    let m = &rules.map;
    let next = at_distance(m, factory, 1, &[]);
    put(&mut g, factory, node(1), 1);
    put(&mut g, next, node(0), 1);
    deal_at(&mut g, "supply_raid", c, &[factory]);
    for s in 0..g.seats.len() {
        set_res(&mut g, s, Resource::Energy, 10);
    }
    assert!(threats::near(&rules.world(), &g, 0, &Key::new("supply_raid")), "seat 0 may pay");
    let discord = g.var("alignment.discord");
    pass_all(&rules, &mut g);
    assert!(g.pending.is_none(), "one nearest seat: no choice");
    assert_eq!(energies(&g), [10, 8], "only the seat on its token");
    assert_eq!(g.var("alignment.discord"), discord, "no unprinted Discord");
    assert_eq!(counted(&g, "threats.uncountered"), 1);
    assert_eq!(counted(&g, "threats.hit.s1"), 1);
    assert!(g.pile("threats.discard.1").contains(&Key::new("supply_raid")));

    // Nobody near it: the nearest node, 2 away, still takes the harm.
    let (rules, mut g, factory, c) = supply_raid_lab(LOSE_2_ENERGY, &[]);
    let m = &rules.map;
    let two = at_distance(m, factory, 2, &[]);
    put(&mut g, two, node(0), 1);
    put(&mut g, at_distance(m, factory, 4, &[]), node(1), 1);
    deal_at(&mut g, "supply_raid", c, &[factory]);
    for s in 0..g.seats.len() {
        set_res(&mut g, s, Resource::Energy, 10);
    }
    assert!(threats::near_seats(&rules.world(), &g, &Key::new("supply_raid")).is_empty());
    pass_all(&rules, &mut g);
    assert!(g.pending.is_none());
    assert_eq!(energies(&g), [8, 10], "the nearest, not near");
}

/// 146: two seats equally near: the lowest tied seat picks which takes the
/// harm (a loss choice among the tied seats); its first option is itself.
#[test]
fn a_tie_for_nearest_is_the_lowest_tied_seats_choice() {
    let (rules, mut g, factory, c) = supply_raid_lab(LOSE_2_ENERGY, &[]);
    let m = &rules.map;
    let a = at_distance(m, factory, 1, &[]);
    let b = at_distance(m, factory, 1, &[a]);
    put(&mut g, a, node(0), 1);
    put(&mut g, b, node(1), 1);
    deal_at(&mut g, "supply_raid", c, &[factory]);
    for s in 0..g.seats.len() {
        set_res(&mut g, s, Resource::Energy, 10);
    }
    pass_all(&rules, &mut g);
    let p = g.pending.as_ref().expect("a tie is a choice");
    assert_eq!(p.ask.seat, 0, "the lowest tied seat answers");
    let opts: Vec<&str> = p.ask.options.iter().map(|o| o.option.as_str()).collect();
    assert_eq!(opts, ["seat 0", "seat 1"]);
    answer(&rules, &mut g, 0);
    answer_all(&rules, &mut g);
    assert_eq!(energies(&g), [8, 10]);
    assert_eq!(counted(&g, "threats.hit_ties"), 1);

    // The other answer hands it to seat 1.
    let (rules, mut g, factory, c) = supply_raid_lab(LOSE_2_ENERGY, &[]);
    put(&mut g, a, node(0), 1);
    put(&mut g, b, node(1), 1);
    deal_at(&mut g, "supply_raid", c, &[factory]);
    for s in 0..g.seats.len() {
        set_res(&mut g, s, Resource::Energy, 10);
    }
    pass_all(&rules, &mut g);
    answer_with(&rules, &mut g, "seat 1");
    answer_all(&rules, &mut g);
    assert_eq!(energies(&g), [10, 8]);
}

/// 146: a printed `destroy_node` takes the nearest player's node closest to
/// the threat, wherever it is, never their last; nobody else loses one.
#[test]
fn an_uncountered_threat_destroys_the_nearest_players_closest_node_never_the_last() {
    const DESTROY: &str = r#"[{"op":"destroy_node","count":1}]"#;
    let (rules, mut g, factory, c) = supply_raid_lab(DESTROY, &[]);
    let m = &rules.map;
    // Seat 0: one node 2 away (not near), one 4 away; seat 1: 3 away and far.
    let s0_close = at_distance(m, factory, 2, &[]);
    let s0_far = at_distance(m, factory, 4, &[]);
    let s1_a = at_distance(m, factory, 3, &[]);
    let s1_b = at_distance(m, factory, 6, &[]);
    put(&mut g, s0_close, node(0), 1);
    put(&mut g, s0_far, node(0), 1);
    put(&mut g, s1_a, node(1), 1);
    put(&mut g, s1_b, node(1), 1);
    deal_at(&mut g, "supply_raid", c, &[factory]);
    pass_all(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(s0_close, node(0)), 0, "the nearest player's closest node goes");
    assert_eq!(g.tokens(s0_far, node(0)), 1);
    assert_eq!((g.tokens(s1_a, node(1)), g.tokens(s1_b, node(1))), (1, 1), "nobody else's");

    // 175: the nearest player's last node goes too.
    let (rules, mut g, factory, c) = supply_raid_lab(DESTROY, &[]);
    let m = &rules.map;
    let next = at_distance(m, factory, 1, &[]);
    put(&mut g, factory, node(0), 1);
    put(&mut g, next, node(1), 1);
    put(&mut g, at_distance(m, factory, 6, &[]), node(1), 1);
    deal_at(&mut g, "supply_raid", c, &[factory]);
    pass_all(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(total(&g, node(0)), 0, "its last node");
    assert_eq!(total(&g, node(1)), 2);
}

/// 146: the Militarist's difficulty line (`threat_compute 4`, from
/// `board_difficulty_at`) gives the adversary 4 compute per uncountered threat,
/// none below that difficulty.
#[test]
fn the_militarists_difficulty_line_adds_compute_per_uncountered_threat() {
    let made = |difficulty: i64| {
        let at = format!("adversary.difficulty={difficulty}");
        let (rules, mut g, factory, c) = supply_raid_lab(LOSE_2_ENERGY, &[&at]);
        put(&mut g, factory, node(0), 1);
        deal_at(&mut g, "supply_raid", c, &[factory]);
        let before = g.var(crate::modules::adversary::COMPUTE);
        pass_all(&rules, &mut g);
        answer_all(&rules, &mut g);
        assert_eq!(counted(&g, "threats.uncountered"), 1, "difficulty {difficulty}");
        (crate::modules::adversary::threat_compute(&rules.world()), g.var(crate::modules::adversary::COMPUTE) - before)
    };
    let at = threat_lab().0.world().int("adversary.board_difficulty_at");
    assert_eq!(made(at - 1), (0, 0));
    assert_eq!(made(at), (4, 4));
}

/// 148: `near_mask` (the bit-mask reach the eval asks of every threat) reads the
/// same hexes as the Vec it replaced, at every step of random games, 2-4 countries.
#[test]
fn near_mask_matches_the_reach() {
    for n in 2..=4u8 {
        let mut c = baseline();
        c.countries = n;
        let rules = Rules::new(&c).unwrap();
        let w = rules.world();
        let (_, actions) = random_game(&rules, 7);
        let mut g = rules.new_game(7);
        let mut checked = 0;
        for a in &actions {
            for (t, _) in threats::active(&g) {
                assert!(threats::mask_matches_reach(&w, &g, &t), "{n} countries: {t}");
                checked += 1;
            }
            rules.apply_mut(&mut g, a, None).unwrap();
        }
        assert!(checked > 0, "{n} countries: no threat was ever active");
    }
}

/// 184's D86 (a), stated in r80's rulebook and threats.md: if a reshuffled deck has no card that fits, the pull
/// takes the next card anyway.
#[test]
fn a_reshuffled_deck_with_no_fitting_card_deals_the_next_anyway() {
    let (rules, mut g) = threat_lab();
    set_pile(&mut g, "threats.deck.2", &["propaganda_ministry"]);
    set_pile(&mut g, "threats.discard.2", &[]);
    let military = [crate::content::PullTag::Is(crate::vocab::ThreatTag::Military)];
    with_cx(&rules, |cx| threats::deal_tier(&mut g, cx, 2, &military, 0)).unwrap();
    assert!(threats::active(&g).iter().any(|(t, _)| t.as_str() == "propaganda_ministry"), "an Influence threat, dealt anyway");
    assert_eq!(g.stats.counters.get("threats.pull_forced").copied(), Some(1));
}
