use super::*;
use crate::modules::adversary;
use crate::modules::board;

fn standard_offered(rules: &Rules, g: &GameState, seat: u8) -> BTreeSet<String> {
    rules
        .legal_actions(g)
        .into_iter()
        .filter_map(|a| match a {
            Action::Standard { seat: s, act } if s == seat => Some(act),
            _ => None,
        })
        .collect()
}

/// Every seat can afford every standard action.
fn rich(g: &mut GameState) {
    for s in 0..2 {
        set_res(g, s, Resource::Energy, 20);
        set_res(g, s, Resource::Credits, 50);
        set_res(g, s, Resource::Chips, 5);
    }
}

#[test]
fn standard_actions_are_offered_only_when_affordable_and_targeted() {
    let (rules, mut g) = actions_with_room();
    for s in 0..2 {
        set_res(&mut g, s, Resource::Energy, 0);
        set_res(&mut g, s, Resource::Credits, 0);
        set_res(&mut g, s, Resource::Chips, 0);
        // 137: ambassadors pay for Claim and Liberate (the lab's `liberate_at` is 1).
        set_res(&mut g, s, Resource::Ambassadors, 0);
        assert!(standard_offered(&rules, &g, s as u8).is_empty(), "seat {s}");
    }
    // 149: an ambassador and the board's Movement, nothing else, to move one.
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    assert!(board::moves_left(&rules.world(), &g, 0) > 0);
    assert_eq!(standard_offered(&rules, &g, 0), ["move".to_string()].into());
    // No ambassadors: no moving.
    set_res(&mut g, 0, Resource::Ambassadors, 0);
    assert!(standard_offered(&rules, &g, 0).is_empty());
    set_res(&mut g, 0, Resource::Ambassadors, 1);

    rich(&mut g);
    let o0 = standard_offered(&rules, &g, 0);
    for a in ["station", "move", "ambassador"] {
        assert!(o0.contains(a), "{a}: {o0:?}");
    }
    // 149: chips can't be sold.
    assert!(
        !o0.contains("remove") && !o0.contains("harden") && !o0.contains("chips"),
        "{o0:?}"
    );
    assert!(!o0.contains("factory"), "cities build factories now");
    let d = node_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    assert!(standard_offered(&rules, &g, 0).contains("remove"));
    // r75 (Matt's ruling 3): taking a botnet down needs the seat's ambassador on its hex.
    put(&mut g, d, BOTNET, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("botnet"));
    assert!(rules
        .apply_mut(&mut g.clone(), &std_act(0, "factory"), None)
        .is_err());
    assert!(rules
        .apply_mut(&mut g.clone(), &std_act(0, "nope"), None)
        .is_err());
    // The view publishes every standard action with its cost.
    let v = rules.view(&g);
    let acts: Vec<&str> = v.standard.iter().map(|x| x.act.as_str()).collect();
    assert_eq!(acts, board::STANDARD.map(board::StandardAction::name));
    let station = v.standard.iter().find(|x| x.act == "station").unwrap();
    assert_eq!(
        (station.energy, station.credits, station.chips),
        (0, 0, rules.world().int("board.station_chips"))
    );
}

#[test]
fn standard_actions_cost_what_the_config_says() {
    let (rules, mut g0) = actions_with_room();
    let w = rules.world();
    rich(&mut g0);
    let spent = |g: &GameState, s: usize| {
        (
            g0.resource(s, Resource::Energy) - g.resource(s, Resource::Energy),
            g0.resource(s, Resource::Credits) - g.resource(s, Resource::Credits),
            g0.resource(s, Resource::Chips) - g.resource(s, Resource::Chips),
        )
    };

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "station"));
    assert_eq!(
        (spent(&g, 0), total(&g, STATION) - total(&g0, STATION)),
        ((0, 0, w.int("board.station_chips")), 1)
    );
    assert_eq!(total(&g, cube(0)), 2, "the bound city's and the station's");

    // 149: no chip sale.
    assert!(rules.apply_mut(&mut g0.clone(), &std_act(0, "chips"), None).is_err());

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "ambassador"));
    let price = w.int("board.ambassador_credits");
    assert_eq!((spent(&g, 0), g.resource(0, Resource::Ambassadors)), ((0, price, 0), 2));

    // 149: a move is free and uses 1 of the round's Movement; with none left, none is offered.
    let mut g = g0.clone();
    let movement = board::moves_left(&w, &g, 0);
    act(&rules, &mut g, std_act(0, "move"));
    assert_eq!((spent(&g, 0), g.resource(0, Resource::Ambassadors)), ((0, 0, 0), 0));
    assert_eq!(board::moves_left(&w, &g, 0), movement - 1);
    while board::moves_left(&w, &g, 0) > 0 {
        set_res(&mut g, 0, Resource::Ambassadors, 1);
        act(&rules, &mut g, std_act(0, "move"));
    }
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("move"));

    let mut g = g0.clone();
    let d = node_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    let supply = g.var(SUPPLY);
    act(&rules, &mut g, std_act(0, "remove"));
    // 066: an ambassador pays for it, and nothing else does.
    assert_eq!(
        (spent(&g, 0), g.tokens(d, ADV_AMBASSADOR), g.var(SUPPLY), g.resource(0, Resource::Ambassadors)),
        ((0, 0, 0), 0, supply + 1, 0)
    );

}

/// 127 (Matt, 2026-11-05: no holding or production limit): a big holding
/// keeps every gain, a gain is always something to act on, the ambassador
/// project and the card sale stay offered, and card raises of credit
/// production stack without a cap.
#[test]
fn no_resource_has_a_holding_limit() {
    use crate::content::Op;
    use crate::modules::{economy, effects};
    let (rules, mut g) = actions_with_room();
    rich(&mut g);
    let w = rules.world();
    for r in [Resource::Credits, Resource::Energy, Resource::Chips, Resource::Ambassadors] {
        let mut h = g.clone();
        set_res(&mut h, 0, r, 500);
        run_ops(&rules, &mut h, &[Op::Gain { resource: r, amount: 3 }]);
        assert_eq!(h.resource(0, r), 503, "{r:?}: every gain is kept");
        let gain = Op::Gain { resource: r, amount: 1 };
        assert!(effects::feasible(&w, &h, 0, std::slice::from_ref(&gain)), "{r:?}: a gain always acts");
    }
    set_res(&mut g, 0, Resource::Ambassadors, 500);
    set_res(&mut g, 0, Resource::Credits, 5000);
    let sells = |g: &GameState| rules.legal_actions(g).iter().any(|a| matches!(a, Action::Sell { seat: 0, .. }));
    set_pile(&mut g, &crate::modules::cards::hand(0), &["open_assembly"]);
    let offered = standard_offered(&rules, &g, 0);
    // 149: the card sale stays; chips can't be sold.
    assert!(offered.contains("ambassador") && !offered.contains("chips") && sells(&g));
    let key = economy::production_key(0, Resource::Credits);
    let before = g.var(key.as_str());
    run_ops(&rules, &mut g, &[Op::Production { resource: Resource::Credits, amount: 200 }]);
    assert_eq!(g.var(key.as_str()) - before, 200, "no cap on credit production");
}

#[test]
fn a_card_s_harden_action_costs_chips() {
    // No card prints a chip Harden since r42: a building is given it here.
    let (rules, mut g) = lab_actions_with(&[r#"cards.darknet_exchange.passives=[{"op":"harden_action","chips":1}]"#]);
    let p = node_of(&g, 0)[0];
    put(&mut g, p, STATION, 1);
    put(&mut g, p, cube(0), 1);
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Chips, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("harden_chips"));
    set_pile(&mut g, &crate::modules::cards::built(0), &["darknet_exchange"]);
    assert!(standard_offered(&rules, &g, 0).contains("harden_chips"));
    act(&rules, &mut g, std_act(0, "harden_chips"));
    assert_eq!((g.resource(0, Resource::Chips), total(&g, HARDEN)), (0, 1));
}

#[test]
fn standard_projects_turn_credits_into_hardening_and_drains() {
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    set_res(&mut g, 0, Resource::Credits, 40);

    // Any player may harden with credits (the Builder has no Harden passive).
    let p = node_of(&g, 0)[0];
    put(&mut g, p, STATION, 1);
    put(&mut g, p, cube(0), 1);
    rules.apply_mut(&mut g, &std_act(0, "harden_buy"), None).unwrap();
    pick(&rules, &mut g, p);
    assert_eq!(g.tokens(p, HARDEN), 1);

    // A drain needs compute on a project.
    set_pile(&mut g, PROJECTS, &["market_crash"]);
    assert!(rules.apply_mut(&mut g.clone(), &std_act(0, "drain"), None).is_err());
    g.vars.insert(adversary::project_key("market_crash").into(), 5);
    rules.apply_mut(&mut g, &std_act(0, "drain"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.var(&adversary::project_key("market_crash")), 5 - w.int("board.drain_amount"));
}

/// 020: moving the seat's only ambassador onto the hex it already holds was
/// offered, charged for and moved nothing.
#[test]
fn moving_an_ambassador_never_targets_the_hex_it_already_holds() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    for p in hexes_where(m, |h| kind(m, h) != HexKind::Ocean) {
        let mut g = g0.clone();
        put(&mut g, p, node(0), 1);
        put(&mut g, p, ambassador(0), 1);
        set_res(&mut g, 0, Resource::Ambassadors, 0);
        set_res(&mut g, 0, Resource::Credits, 3);
        if !rules.legal_actions(&g).contains(&std_act(0, "move")) {
            continue;
        }
        rules.apply_mut(&mut g, &std_act(0, "move"), None).unwrap();
        match &g.pending {
            Some(pc) => assert!(
                pc.ask.options.iter().all(|o| o.target != Some(m.hexes[p].hex)),
                "hex {p}: {:?}",
                pc.ask.options
            ),
            None => assert_eq!(g.tokens(p, ambassador(0)), 0, "hex {p}: paid and moved nothing"),
        }
    }
}

/// 137: Claim puts the seat's cube on a neutral city in reach with a site
/// claim, for `claim_city` ambassadors from the supply (Align took 1).
#[test]
fn claiming_puts_a_cube_on_a_neutral_city_in_reach() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let n = rules.world().int("board.claim_city");
    let (city, next) = city_with_lonely_neighbors(m);
    put(&mut g, next[0], node(0), 1);
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Ambassadors, n - 1);
    assert!(!standard_offered(&rules, &g, 0).contains("claim"), "{} of {n}", n - 1);
    set_res(&mut g, 0, Resource::Ambassadors, n);
    act(&rules, &mut g, std_act(0, "claim"));
    assert_eq!(g.tokens(city, cube(0)), 1);
    assert_eq!(g.resource(0, Resource::Ambassadors), 0);
    assert_eq!(g.stats.counters["board.claimed.city"], 1);
    // An owned city can't be claimed.
    set_res(&mut g, 0, Resource::Ambassadors, n);
    assert!(!standard_offered(&rules, &g, 0).contains("claim"));
}

/// 137: claiming a dark city (agents on it) costs 1 more for
/// each; they cancel back to the supply and the cube goes on at once (r40:
/// Align canceled one and aligned nothing).
#[test]
fn claiming_a_dark_city_pays_one_more_for_each_adversary_ambassador() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let n = w.int("board.claim_city");
    let (city, next) = city_with_lonely_neighbors(m);
    put(&mut g, next[0], node(0), 1);
    put(&mut g, city, ADV_AMBASSADOR, 2);
    assert_eq!(board::claim_cost(&w, &g, city), Some(n + 2));
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Ambassadors, n + 1);
    assert!(!standard_offered(&rules, &g, 0).contains("claim"));
    set_res(&mut g, 0, Resource::Ambassadors, n + 2);
    let supply = g.var(SUPPLY);
    act(&rules, &mut g, std_act(0, "claim"));
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(city, cube(0))), (0, 1));
    assert_eq!((g.resource(0, Resource::Ambassadors), g.var(SUPPLY)), (0, supply + 2));
}

/// 137: Claim costs by site type: `claim_city`, `claim_station`,
/// `claim_factory`; an adversary or owned site has no claim cost.
#[test]
fn claiming_costs_by_site_type() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let me = m.numbered(0, piece(0).node);
    put(&mut g, me, node(0), 1);
    let near: Vec<usize> = m.hexes[me]
        .neighbors
        .iter()
        .copied()
        .filter(|&h| kind(m, h) == HexKind::Open)
        .collect();
    let city = m.numbered(0, piece(0).cities[0]);
    let (st, fa) = (near[0], near[1]);
    put(&mut g, st, STATION, 1);
    put(&mut g, fa, FACTORY, 1);
    assert_eq!(board::claim_cost(&w, &g, city), Some(w.int("board.claim_city")));
    assert_eq!(board::claim_cost(&w, &g, st), Some(w.int("board.claim_station")));
    assert_eq!(board::claim_cost(&w, &g, fa), Some(w.int("board.claim_factory")));
    assert_eq!(board::claim_cost(&w, &g, me), None, "no site");
    let mut t = g.clone();
    put(&mut t, st, ADV_CUBE, 1);
    put(&mut t, fa, cube(1), 1);
    assert_eq!((board::claim_cost(&w, &t, st), board::claim_cost(&w, &t, fa)), (None, None));
    // Claiming the factory: the seat's cube on it, its ambassadors used up.
    set_res(&mut g, 0, Resource::Ambassadors, w.int("board.claim_factory"));
    rules.apply_mut(&mut g, &std_act(0, "claim"), None).unwrap();
    pick(&rules, &mut g, fa);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(fa, cube(0)), g.resource(0, Resource::Ambassadors)), (1, 0));
    assert_eq!(g.stats.counters["board.claimed.factory"], 1);
}

/// 148 (D70): a card's station claim (`claim_station`) claims as the Claim action does: a station holding
/// agents is open to it for one of the seat's ambassadors each, which cancel.
#[test]
fn card_claims_a_station_as_claim_does() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let me = m.numbered(0, piece(0).node);
    put(&mut g, me, node(0), 1);
    let st = m.hexes[me].neighbors.iter().copied().find(|&h| kind(m, h) == HexKind::Open).unwrap();
    put(&mut g, st, STATION, 1);
    assert_eq!(board::card_claim_stations(&w, &g, 0), vec![st]);
    put(&mut g, st, ADV_AMBASSADOR, 2);
    assert_eq!(board::card_claim_extra(&w, &g, st), Some(2));
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    assert!(board::card_claim_stations(&w, &g, 0).is_empty(), "short of the extra ambassadors");
    set_res(&mut g, 0, Resource::Ambassadors, 3);
    assert_eq!(board::card_claim_stations(&w, &g, 0), vec![st]);
    run_ops(&rules, &mut g, &[crate::content::Op::ClaimStation]);
    assert_eq!((g.tokens(st, ADV_AMBASSADOR), g.tokens(st, cube(0))), (0, 1));
    assert_eq!(g.resource(0, Resource::Ambassadors), 1);
}

/// 137: Liberate turns an adversary station or factory in reach neutral for
/// `liberate_facility_at` ambassadors (plus 1 for each agent
/// on it, which cancel) and sways `free_facility` Unity; a botnet on it stays
/// and doesn't block it.
#[test]
fn liberating_a_station_or_factory_turns_it_neutral() {
    for (tok, what) in [(STATION, "station"), (FACTORY, "factory")] {
        let (rules, mut g) = lab(&[]);
        let w = rules.world();
        let m = &rules.map;
        let me = m.numbered(0, piece(0).node);
        put(&mut g, me, node(0), 1);
        let h = m.hexes[me].neighbors.iter().copied().find(|&h| kind(m, h) == HexKind::Open).unwrap();
        put(&mut g, h, tok, 1);
        put(&mut g, h, ADV_CUBE, 1);
        put(&mut g, h, BOTNET, 1);
        put(&mut g, h, ADV_AMBASSADOR, 1);
        let n = w.int("board.liberate_facility_at") + 1;
        assert_eq!(board::liberate_cost(&w, &g, h), Some(n), "{what}");
        set_res(&mut g, 0, Resource::Ambassadors, n - 1);
        assert!(!standard_offered(&rules, &g, 0).contains("liberate"), "{what}");
        set_res(&mut g, 0, Resource::Ambassadors, n);
        let unity = g.var("alignment.unity") + crate::modules::alignment::unity_pending(&g);
        act(&rules, &mut g, std_act(0, "liberate"));
        assert_eq!(
            (g.tokens(h, ADV_CUBE), g.tokens(h, ADV_AMBASSADOR), g.tokens(h, BOTNET), g.tokens(h, tok)),
            (0, 0, 1, 1),
            "{what}"
        );
        assert_eq!(g.resource(0, Resource::Ambassadors), 0, "{what}");
        assert_eq!(
            g.var("alignment.unity") + crate::modules::alignment::unity_pending(&g) - unity,
            w.int("alignment.free_facility"),
            "{what}"
        );
        assert_eq!(g.stats.counters[format!("board.liberated.{what}").as_str()], 1);
        // Out of reach, it isn't offered.
        let mut t = g.clone();
        put(&mut t, h, ADV_CUBE, 1);
        put(&mut t, me, node(0), 0);
        let far = hexes_where(m, |x| m.distance(x, h) > 2 && kind(m, x) == HexKind::Open)[0];
        put(&mut t, far, node(0), 1);
        set_res(&mut t, 0, Resource::Ambassadors, 10);
        assert!(!board::liberable(&w, &t, 0, h), "{what}");
    }
}

/// 022 (D10): hardening goes only on facilities a token can protect (a
/// player-owned one, not the adversary's), and "harden 2" is two
/// different buildings (fewer if fewer exist).
#[test]
fn hardening_goes_on_different_buildings_a_token_can_protect() {
    use crate::content::Op;
    let (rules, mut g0) = lab(&[]);
    let m = &rules.map;
    let me = m.numbered(0, piece(0).node);
    put(&mut g0, me, node(0), 1);
    let near: Vec<usize> = m.hexes[me]
        .neighbors
        .iter()
        .copied()
        .filter(|&h| kind(m, h) != HexKind::Ocean && m.hexes[h].country == 0)
        .collect();
    let (a, b, adv) = (near[0], near[1], near[2]);
    put(&mut g0, a, STATION, 1);
    put(&mut g0, a, cube(0), 1);
    put(&mut g0, adv, FACTORY, 1);
    put(&mut g0, adv, ADV_CUBE, 1);
    let harden = [Op::Harden {
        count: 2,
    }];
    // One building it can protect: one token, not two on it, none on the
    // adversary's factory.
    let mut g = g0.clone();
    run_ops(&rules, &mut g, &harden);
    assert_eq!((g.tokens(a, HARDEN), g.tokens(adv, HARDEN)), (1, 0));
    // Two: one token each.
    let mut g = g0.clone();
    put(&mut g, b, FACTORY, 1);
    put(&mut g, b, cube(0), 1);
    let o = modules::common::hex_opt(&rules.world(), b);
    run_ops_with(&rules, &mut g, vec![crate::state::Pick { option: o.option, target: o.target }], &harden);
    assert_eq!((g.tokens(a, HARDEN), g.tokens(b, HARDEN)), (1, 1));
}

// ---------- reach and site claims ----------

#[test]
fn card_targets_on_the_map_must_be_in_reach() {
    use crate::modules::effects;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let me = m.numbered(0, piece(0).node);
    put(&mut g, me, node(0), 1);
    let near = m.hexes[me].neighbors[0];
    let far = hexes_where(m, |h| m.distance(h, me) > 2 && kind(m, h) != HexKind::Ocean)[0];
    put(&mut g, near, ADV_AMBASSADOR, 1);
    put(&mut g, far, ADV_AMBASSADOR, 1);
    assert_eq!(effects::damage_hexes(&w, &g, 0, 1, 0), vec![near]);
}

/// r75 (Matt's ruling 2): Claim needs the site in reach (a node on or next to it)
/// and no embargo; a botnet on or next to it no longer blocks it.
#[test]
fn claiming_a_site_needs_it_in_reach_and_a_botnet_does_not_block_it() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let station = m.numbered(0, piece(0).stations[0]);
    let next = m.hexes[station]
        .neighbors
        .iter()
        .copied()
        .find(|&h| kind(m, h) == HexKind::Open)
        .unwrap();
    let mut g0 = g0;
    put(&mut g0, station, STATION, 1);

    let w = rules.world();
    // 137: the Claim action reads the site claim (an arriving ambassador
    // claims nothing now); the seat holds what a claim costs.
    let claim = |g: &GameState| board::claimable(&w, g, 0, station);
    set_res(&mut g0, 0, Resource::Ambassadors, 10);

    // Node two hexes away: no claim.
    let mut g = g0.clone();
    let away = hexes_where(m, |h| {
        m.distance(h, station) == 2 && kind(m, h) == HexKind::Open
    })[0];
    put(&mut g, away, node(0), 1);
    assert!(!claim(&g));

    // Node next to it: a claim.
    let mut g = g0.clone();
    put(&mut g, next, node(0), 1);
    assert!(claim(&g));

    // A botnet next to the node but not the site does not block it.
    let mut g = g0.clone();
    put(&mut g, next, node(0), 1);
    let botnet_far = m.hexes[next]
        .neighbors
        .iter()
        .copied()
        .find(|&h| m.distance(h, station) == 2)
        .unwrap();
    put(&mut g, botnet_far, BOTNET, 1);
    assert!(claim(&g));

    // r75: a botnet next to the site no longer blocks it.
    let mut g = g0.clone();
    put(&mut g, next, node(0), 1);
    let botnet_near = m.hexes[station]
        .neighbors
        .iter()
        .copied()
        .find(|&h| h != next && kind(m, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, botnet_near, BOTNET, 1);
    assert!(claim(&g));
    // Nor does a botnet on the site itself (`can_claim` reads reach and embargo only).
    let mut g = g0.clone();
    put(&mut g, next, node(0), 1);
    put(&mut g, station, BOTNET, 1);
    assert!(board::can_claim(&w, &g, 0, station));

    // Claiming a city: likewise.
    let (city, lonely) = city_with_lonely_neighbors(m);
    let mut g = g0.clone();
    put(&mut g, station, STATION, 0);
    put(&mut g, lonely[0], node(0), 1);
    set_res(&mut g, 0, Resource::Ambassadors, w.int("board.claim_city"));
    assert!(standard_offered(&rules, &g, 0).contains("claim"));
    // A botnet next to the city: still offered.
    put(&mut g, lonely[1], BOTNET, 1);
    assert!(m.adjacent(lonely[1], city));
    assert!(standard_offered(&rules, &g, 0).contains("claim"));
}

/// r75 (Matt's ruling 2): Claim's bill is paid by the seat's own ambassadors on the site first, then its
/// supply; another seat's ambassadors there pay nothing and stay. `claim_city` is 5.
#[test]
fn claiming_pays_from_your_own_ambassadors_on_the_site_first() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let n = w.int("board.claim_city");
    assert_eq!(n, 5);
    let (city, next) = city_with_lonely_neighbors(m);
    put(&mut g, next[0], node(0), 1);
    put(&mut g, city, ambassador(1), 2);
    assert_eq!(board::site_supply_cost(&w, &g, 0, city, false), Some(n));
    put(&mut g, city, ambassador(0), 2);
    assert_eq!(board::site_supply_cost(&w, &g, 0, city, false), Some(n - 2));
    set_res(&mut g, 0, Resource::Ambassadors, n - 3);
    assert!(!standard_offered(&rules, &g, 0).contains("claim"));
    set_res(&mut g, 0, Resource::Ambassadors, n - 2);
    act(&rules, &mut g, std_act(0, "claim"));
    assert_eq!(g.tokens(city, cube(0)), 1);
    assert_eq!((g.resource(0, Resource::Ambassadors), g.tokens(city, ambassador(0)), g.tokens(city, ambassador(1))), (0, 0, 2));
}

/// r75 (Matt's ruling 3): the Take down a botnet action, `botnet_credits` (5): the seat's ambassador on the
/// botnet's hex (it stays); the botnet comes off and sways Unity (`alignment.botnet`).
#[test]
fn taking_a_botnet_down_needs_your_ambassador_on_its_hex() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let price = w.int("board.botnet_credits");
    assert_eq!(price, 5);
    let me = m.numbered(0, piece(0).node);
    put(&mut g, me, node(0), 1);
    let h = m.hexes[me].neighbors.iter().copied().find(|&x| kind(m, x) != HexKind::Ocean).unwrap();
    put(&mut g, h, BOTNET, 1);
    set_res(&mut g, 0, Resource::Credits, price);
    assert!(!standard_offered(&rules, &g, 0).contains("botnet"), "no ambassador on it");
    put(&mut g, h, ambassador(1), 1);
    assert!(!standard_offered(&rules, &g, 0).contains("botnet"), "another seat's ambassador");
    put(&mut g, h, ambassador(0), 1);
    assert_eq!(board::botnet_targets(&g, 0), vec![h]);
    set_res(&mut g, 0, Resource::Credits, price - 1);
    assert!(!standard_offered(&rules, &g, 0).contains("botnet"), "short of the price");
    set_res(&mut g, 0, Resource::Credits, price);
    let unity = g.var("alignment.unity") + crate::modules::alignment::unity_pending(&g);
    act(&rules, &mut g, std_act(0, "botnet"));
    assert_eq!((g.tokens(h, BOTNET), g.tokens(h, ambassador(0)), g.resource(0, Resource::Credits)), (0, 1, 0));
    assert_eq!(g.var("alignment.unity") + crate::modules::alignment::unity_pending(&g) - unity, w.int("alignment.botnet"));
    assert_eq!(g.stats.counters["board.botnets_taken_down"], 1);
}

