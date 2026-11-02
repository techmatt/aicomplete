use super::presence_powers::{at_step, run_ops};
use super::*;
use crate::content::RoundFlag;
use crate::modules::{board, threats};

pub(super) fn threat_lab() -> (Rules, GameState) {
    let rules = Rules::new(&lab_config(&[ACTIONS, "threats.execute"])).unwrap();
    let g = rules.new_game(3);
    (rules, g)
}

/// Puts `threat` on country `c` (out of its deck).
pub(super) fn deal(g: &mut GameState, threat: &str, c: usize) {
    for t in [1, 2] {
        if let Some(p) = g.piles.get_mut(format!("threats.deck.{t}").as_str()) {
            Arc::make_mut(p).retain(|x| x != threat);
        }
    }
    Arc::make_mut(g.piles.entry(format!("threats.on.{c}")).or_default()).push(threat.into());
}

#[test]
fn threats_are_countered_only_when_fully_paid_and_partial_payments_come_back() {
    let (rules, mut g) = threat_lab();
    deal(&mut g, "sabotage", 0);
    deal(&mut g, "blackout", 1);
    for s in 0..2 {
        set_res(&mut g, s, Resource::Credits, 6);
    }
    let legal = rules.legal_actions(&g);
    // Only players with a presence in the threat's country may pay.
    let pays = |seat: u8, t: &str| {
        legal
            .iter()
            .any(|a| matches!(a, Action::Pay { seat: s, threat, .. } if *s == seat && threat == t))
    };
    assert!(pays(0, "sabotage") && !pays(0, "blackout"));
    assert!(pays(1, "blackout") && !pays(1, "sabotage"));
    let pay = |seat, threat: &str, resource: Resource, amount| Action::Pay {
        seat,
        threat: threat.into(),
        resource,
        amount,
    };
    // Blackout: 4 credits (024). Pay 1, withdraw, then pay in full.
    rules
        .apply_mut(&mut g, &pay(1, "blackout", Resource::Credits, 1), None)
        .unwrap();
    assert_eq!(g.resource(1, Resource::Credits), 5);
    let withdraw = Action::Withdraw {
        seat: 1,
        threat: "blackout".into(),
        resource: Resource::Credits,
    };
    rules.apply_mut(&mut g, &withdraw, None).unwrap();
    assert_eq!(g.resource(1, Resource::Credits), 6);
    assert!(rules
        .apply_mut(&mut g, &pay(1, "blackout", Resource::Credits, 5), None)
        .is_err());
    assert!(rules
        .apply_mut(&mut g, &pay(0, "blackout", Resource::Credits, 1), None)
        .is_err());
    rules
        .apply_mut(&mut g, &pay(1, "blackout", Resource::Credits, 4), None)
        .unwrap();
    assert!(g.pile("threats.on.1").is_empty());
    assert_eq!(g.pile("threats.discard.1")[0], "blackout");
    assert_eq!(g.resource(1, Resource::Credits), 2);
    // Sabotage: 5 credits. Pay only part; it comes back.
    rules
        .apply_mut(&mut g, &pay(0, "sabotage", Resource::Credits, 2), None)
        .unwrap();
    assert_eq!(g.pile("threats.on.0"), ["sabotage"]);
    let discord = g.var("alignment.discord");
    for s in 0..2 {
        rules
            .apply_mut(&mut g, &Action::Pass { seat: s }, None)
            .unwrap();
    }
    assert!(g.pending.is_none());
    assert_eq!(g.resource(0, Resource::Credits), 6);
    assert!(g
        .pile("threats.discard.1")
        .contains(&Key::new("sabotage")));
    assert!(board::flagged(&g, RoundFlag::Sabotage, 0));
    assert_eq!(g.var("alignment.discord"), discord + 1);
}

#[test]
fn jamming_a_threat_down_to_what_is_paid_counters_it() {
    let (rules, mut g) = threat_lab();
    // Requisition needs 4 credits (024); with 2 paid, Threat Triage (023; was
    // Jam Signals) takes it to 2: countered at once.
    deal(&mut g, "requisition", 0);
    set_res(&mut g, 0, Resource::Credits, 12);
    rules
        .apply_mut(
            &mut g,
            &Action::Pay {
                seat: 0,
                threat: "requisition".into(),
                resource: Resource::Credits,
                amount: 2,
            },
            None,
        )
        .unwrap();
    Arc::make_mut(g.piles.get_mut("cards.deck.1").unwrap()).retain(|c| c != "threat_triage");
    g.piles
        .insert("cards.hand.0".into(), vec!["threat_triage".into()].into());
    set_res(&mut g, 0, Resource::Insight, 1);
    act(
        &rules,
        &mut g,
        Action::Play {
            seat: 0,
            card: "threat_triage".into(),
        },
    );
    assert!(g.pile("threats.on.0").is_empty());
    assert!(g
        .pile("threats.discard.1")
        .contains(&Key::new("requisition")));
    assert_eq!(g.resource(0, Resource::Credits), 8);
}

#[test]
fn difficulty_1_raises_the_counters_and_deals_more_threats() {
    let sets = [
        "adversary.counter_bonus=1",
        "adversary.counter_bonus_all=1",
        "adversary.extra_threats=1",
    ];
    let at = |d: i64| {
        let mut c = lab_config(&["threats.deal", ACTIONS]);
        for s in sets
            .iter()
            .map(|s| s.to_string())
            .chain([format!("adversary.difficulty={d}")])
        {
            c.apply_override(&s).unwrap();
        }
        Rules::new(&c).unwrap()
    };
    let (r0, r1) = (at(0), at(1));
    let (g0, mut g1) = (r0.new_game(1), r1.new_game(1));
    let req = |r: &Rules, g: &GameState| threats::requirement(&r.world(), g, "orbital_strike");
    let credits = |c| vec![(Resource::Credits, c)];
    assert_eq!(req(&r0, &g0), credits(6));
    assert_eq!(req(&r1, &g1), credits(8));
    let on = |g: &GameState, c: usize| {
        threats::active(g)
            .iter()
            .filter(|(_, at)| *at == Some(c))
            .count()
    };
    assert_eq!((on(&g0, 0), on(&g0, 1)), (1, 1));
    // One extra a round, one country after the other.
    assert_eq!((on(&g1, 0), on(&g1, 1)), (2, 1));
    for s in 0..2 {
        r1.apply_mut(&mut g1, &Action::Pass { seat: s }, None)
            .unwrap();
    }
    assert_eq!(g1.round, 2);
    assert_eq!((on(&g1, 0), on(&g1, 1)), (3, 3));
}

/// 020: threats execute after production and upkeep, so a Blackout or
/// Sabotage flag lasts through the next round's production and upkeep
/// (cleared "this round", it never landed).
#[test]
fn blackout_and_sabotage_last_into_the_next_rounds_production() {
    let round = [ACTIONS, "board.produce", "economy.upkeep", "threats.execute", "board.cleanup"];
    let mut c = lab_config(&round);
    // 021: no neutral city's energy offered for sale (that would ask).
    c.apply_override("board.city_energy_credits=20").unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(3);
    deal(&mut g, "blackout", 0);
    deal(&mut g, "sabotage", 1);
    // A twin with no threats: what round 2 would have paid.
    let mut twin = rules.new_game(3);
    pass_round(&rules, &mut g);
    pass_round(&rules, &mut twin);
    assert_eq!(g.round, 2);
    assert!(board::flagged(&g, RoundFlag::Blackout, 0) && board::flagged(&g, RoundFlag::Sabotage, 1));
    // A city of country 0 aligned to seat 0: its energy comes free in production.
    let city = cities(&rules.map)
        .into_iter()
        .find(|&h| rules.map.hexes[h].country == 0 && g.tokens(h, ADV_CUBE) == 0)
        .unwrap();
    put(&mut g, city, &cube(0), 1);
    put(&mut twin, city, &cube(0), 1);
    let before = |g: &GameState| (0..2).map(|s| g.resource(s, Resource::Energy)).collect::<Vec<_>>();
    let (b, bt) = (before(&g), before(&twin));
    pass_round(&rules, &mut g);
    pass_round(&rules, &mut twin);
    assert_eq!(g.round, 3);
    assert!(!board::flagged(&g, RoundFlag::Blackout, 0) && !board::flagged(&g, RoundFlag::Sabotage, 1));
    let gain = |g: &GameState, b: &[i64], s: usize| g.resource(s, Resource::Energy) - b[s];
    // Round 2's production paid seat 0 nothing for the blacked-out city.
    let w = rules.world();
    let city_energy = w.list("board.corp_energy")[0] + w.int("board.aligned_bonus");
    assert!(city_energy > 0);
    assert_eq!(gain(&g, &b, 0), gain(&twin, &bt, 0) - city_energy);
    // 021: round 2's upkeep paid seat 1 (presence in sabotaged country 1) no
    // energy; its twin got its energy income.
    let track = crate::modules::economy::income(&w, &twin, 1)[Resource::Energy];
    assert!(track > 0);
    assert_eq!(gain(&g, &b, 1), gain(&twin, &bt, 1) - track);
}

/// 022 (D11): a threat dealt after the counter phase (a Discord tip in
/// production) waits beside the row: it executes next round, after a
/// counter phase in which it could be paid.
#[test]
fn a_threat_dealt_after_the_counter_phase_executes_next_round() {
    let (rules, mut g) = threat_lab();
    deal(&mut g, "supply_raid", 1);
    // Deal one extra as if in the execute phase (after the counter phase).
    g.phase = 1;
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    g.phase = 0;
    let late = g.pile("threats.on.0")[0].clone();
    assert_eq!(g.var(&format!("threats.late.{late}")), 1);
    pass_round(&rules, &mut g);
    assert_eq!(g.round, 2);
    assert_eq!(g.stats.counters["threats.executed"], 1);
    assert_eq!(g.pile("threats.on.0"), [late.clone()]);
    pass_round(&rules, &mut g);
    assert_eq!(g.stats.counters["threats.executed"], 2);
    assert!(g.pile("threats.on.0").is_empty());
    assert_eq!(g.var(&format!("threats.late.{late}")), 0);
}

/// 022 (D12): the tier-2 deck covers the largest round's deal (4 countries,
/// difficulty 1+), and a deal the deck can't supply is counted.
#[test]
fn the_tier_2_deck_covers_four_countries_and_a_short_deal_is_counted() {
    let base = Rules::new(&baseline()).unwrap();
    let w = base.world();
    let tier2 = crate::content().threats.iter().filter(|t| t.tier == 2).count() as i64;
    let most = 4 * w.int("threats.per_country") + w.int("adversary.extra_threats");
    assert!(tier2 > most, "{tier2} tier-2 threats for a deal of {most}");
    let (rules, mut g) = threat_lab();
    g.piles.insert("threats.deck.1".into(), vec!["blackout".into()].into());
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 3));
    assert_eq!(g.stats.counters["threats.short"], 2);
}

#[test]
fn a_seat_that_lost_its_presence_there_can_take_back_a_payment() {
    let (rules, mut g) = lab_with(&[], &[]);
    let t = rules.world().content.threats[0].id.clone();
    let on = format!("threats.on.{}", 0);
    g.piles.insert(on.as_str().into(), vec![t.as_str().into()].into());
    g.vars.insert(format!("threats.paid.{t}.0.credits").as_str().into(), 1);
    let legal = rules.legal_actions(&g);
    assert!(legal.iter().any(|a| matches!(a, Action::Withdraw { seat: 0, .. })), "{legal:?}");
    assert!(!legal.iter().any(|a| matches!(a, Action::Pay { seat: 0, .. })), "no presence, no paying");
}

#[test]
fn a_delayed_threat_skips_this_round_and_a_shield_absorbs_a_destruction() {
    let (rules, mut g) = at_step(&[], 1);
    let a = rules.map.numbered(0, piece(0).presence);
    with_cx(&rules, |cx| threats::deal_extra(&mut g, cx, 1));
    let t = threats::active(&g)[0].0.clone();
    assert!(!threats::waits(&g, &t));
    run_ops(&rules, &mut g, &[content::Op::DelayThreat { count: 1 }]);
    assert!(threats::waits(&g, &t));
    g.round += 1;
    assert!(!threats::waits(&g, &t), "it comes back next round");
    run_ops(&rules, &mut g, &[content::Op::ShieldPresence { count: 1 }]);
    assert_eq!(g.tokens(a, HARDEN), 1);
    let kept = with_cx(&rules, |cx| board::destroy_piece(&mut g, cx, a, &presence(0), board::Cause::Troop));
    assert!(!kept, "absorbed");
    assert_eq!(g.tokens(a, &presence(0)), 1);
}
