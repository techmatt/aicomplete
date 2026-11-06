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
        assert!(standard_offered(&rules, &g, s as u8).is_empty(), "seat {s}");
    }
    // Just the credits to move an ambassador.
    set_res(&mut g, 0, Resource::Credits, rules.world().int("board.move_credits"));
    assert_eq!(standard_offered(&rules, &g, 0), ["move".to_string()].into());
    // No ambassadors: no moving.
    set_res(&mut g, 0, Resource::Ambassadors, 0);
    assert!(standard_offered(&rules, &g, 0).is_empty());
    set_res(&mut g, 0, Resource::Ambassadors, 1);

    rich(&mut g);
    let o0 = standard_offered(&rules, &g, 0);
    for a in ["station", "move", "ambassador", "chips"] {
        assert!(o0.contains(a), "{a}: {o0:?}");
    }
    assert!(
        !o0.contains("remove") && !o0.contains("harden"),
        "{o0:?}"
    );
    assert!(!o0.contains("factory"), "cities build factories now");
    let d = node_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    assert!(standard_offered(&rules, &g, 0).contains("remove"));
    // Removing a botnet is no standard action (019: only tier-2 effects remove botnets).
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

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "chips"));
    // 021: sell 1 chip for 1 credit.
    assert_eq!(spent(&g, 0), (0, -1, 1));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "ambassador"));
    let price = w.int("board.ambassador_credits");
    assert_eq!((spent(&g, 0), g.resource(0, Resource::Ambassadors)), ((0, price, 0), 2));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "move"));
    assert_eq!((spent(&g, 0), g.resource(0, Resource::Ambassadors)), ((0, w.int("board.move_credits"), 0), 0));

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
/// project and both sales stay offered, and card raises of credit
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
    assert!(offered.contains("ambassador") && offered.contains("chips") && sells(&g));
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

#[test]
fn aligning_puts_a_cube_on_a_clean_city_in_reach() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (city, next) = city_with_lonely_neighbors(m);
    put(&mut g, next[0], node(0), 1);
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    act(&rules, &mut g, std_act(0, "align"));
    assert_eq!(g.tokens(city, cube(0)), 1);
    assert_eq!(g.resource(0, Resource::Ambassadors), 0);
    // A corrupted or aligned city can't be aligned.
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("align"));
}

/// r40 (D38): aligning a dark city (an adversary ambassador on it) cancels one
/// adversary ambassador, as a site claim does, and aligns nothing; once the city
/// is clean, a second Align aligns it.
#[test]
fn aligning_a_dark_city_cancels_an_adversary_ambassador() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (city, next) = city_with_lonely_neighbors(m);
    put(&mut g, next[0], node(0), 1);
    put(&mut g, city, ADV_AMBASSADOR, 1);
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Ambassadors, 2);
    act(&rules, &mut g, std_act(0, "align"));
    assert_eq!((g.tokens(city, ADV_AMBASSADOR), g.tokens(city, cube(0))), (0, 0));
    assert_eq!(g.resource(0, Resource::Ambassadors), 1);
    act(&rules, &mut g, std_act(0, "align"));
    assert_eq!(g.tokens(city, cube(0)), 1);
    assert_eq!(g.resource(0, Resource::Ambassadors), 0);
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

#[test]
fn claiming_a_site_needs_node_and_no_botnet_on_or_next_to_it() {
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

    // Node two hexes away: the ambassador just waits.
    let mut g = g0.clone();
    let away = hexes_where(m, |h| {
        m.distance(h, station) == 2 && kind(m, h) == HexKind::Open
    })[0];
    put(&mut g, away, node(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(
        (
            g.tokens(station, cube(0)),
            g.tokens(station, ambassador(0))
        ),
        (0, 1)
    );

    // Node next to it: claimed.
    let mut g = g0.clone();
    put(&mut g, next, node(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(g.tokens(station, cube(0)), 1);

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
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(g.tokens(station, cube(0)), 1);

    // A botnet next to the site blocks it, wherever the node is.
    let mut g = g0.clone();
    put(&mut g, next, node(0), 1);
    let botnet_near = m.hexes[station]
        .neighbors
        .iter()
        .copied()
        .find(|&h| h != next && kind(m, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, botnet_near, BOTNET, 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(
        (
            g.tokens(station, cube(0)),
            g.tokens(station, ambassador(0))
        ),
        (0, 1)
    );
    // So does a botnet on the site itself.
    let mut g = g0.clone();
    put(&mut g, next, node(0), 1);
    put(&mut g, station, BOTNET, 1);
    assert!(!board::can_claim(&rules.world(), &g, 0, station));

    // Aligning a city: likewise.
    let (city, lonely) = city_with_lonely_neighbors(m);
    let mut g = g0.clone();
    put(&mut g, lonely[0], node(0), 1);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    assert!(standard_offered(&rules, &g, 0).contains("align"));
    // A botnet next to the node, two from the city: still offered.
    let far = m.hexes[lonely[0]]
        .neighbors
        .iter()
        .copied()
        .find(|&h| m.distance(h, city) == 2)
        .unwrap();
    put(&mut g, far, BOTNET, 1);
    assert!(standard_offered(&rules, &g, 0).contains("align"));
    // A botnet next to the city: not.
    put(&mut g, far, BOTNET, 0);
    put(&mut g, lonely[1], BOTNET, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("align"));
}

