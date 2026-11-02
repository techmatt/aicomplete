use super::freeing::pick;
use super::*;
use crate::modules::adversary::{self, PROJECTS};
use crate::modules::board;
use crate::modules::common::NODE;

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

/// The baseline at its action phase after setup. Every seat has room to
/// grow at setup: free open hexes next to its starting presence.
fn actions_with_room() -> (Rules, GameState) {
    let (rules, g) = lab_actions();
    let w = rules.world();
    for s in 0..2 {
        assert!(
            modules::board::presence_hexes(&w, &g, s).len() >= 2,
            "seat {s} has no room to grow at setup"
        );
    }
    (rules, g)
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
    // 1 credit: moving an ambassador.
    set_res(&mut g, 0, Resource::Credits, 1);
    assert_eq!(standard_offered(&rules, &g, 0), ["move".to_string()].into());
    // No ambassadors: no moving.
    set_res(&mut g, 0, Resource::Ambassadors, 0);
    assert!(standard_offered(&rules, &g, 0).is_empty());
    set_res(&mut g, 0, Resource::Ambassadors, 1);

    for s in 0..2 {
        set_res(&mut g, s, Resource::Energy, 20);
        set_res(&mut g, s, Resource::Credits, 50);
        set_res(&mut g, s, Resource::Chips, 5);
    }
    let o0 = standard_offered(&rules, &g, 0);
    for a in ["station", "move", "ambassador", "chips"] {
        assert!(o0.contains(a), "{a}: {o0:?}");
    }
    assert!(
        !o0.contains("remove") && !o0.contains("harden"),
        "{o0:?}"
    );
    assert!(!o0.contains("factory"), "cities build factories now");
    let d = presence_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    assert!(standard_offered(&rules, &g, 0).contains("remove"));
    // Removing a node is no standard action (019: only tier-2 effects remove nodes).
    put(&mut g, d, NODE, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("node"));
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
        (0, 0, 2)
    );
}

#[test]
fn standard_actions_cost_what_the_config_says() {
    let (rules, mut g0) = actions_with_room();
    for s in 0..2 {
        set_res(&mut g0, s, Resource::Energy, 20);
        set_res(&mut g0, s, Resource::Credits, 50);
        set_res(&mut g0, s, Resource::Chips, 5);
    }
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
        ((0, 0, 2), 1)
    );
    assert_eq!(total(&g, &cube(0)), 2, "the bound city's and the station's");

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "chips"));
    // 021: sell 1 chip for 1 credit.
    assert_eq!(spent(&g, 0), (0, -1, 1));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "ambassador"));
    let price = rules.world().int("board.ambassador_credits");
    assert_eq!((spent(&g, 0), g.resource(0, Resource::Ambassadors)), ((0, price, 0), 2));

    let mut g = g0.clone();
    act(&rules, &mut g, std_act(0, "move"));
    assert_eq!((spent(&g, 0), g.resource(0, Resource::Ambassadors)), ((0, 1, 0), 0));

    let mut g = g0.clone();
    let d = presence_of(&g, 0)[0];
    put(&mut g, d, ADV_AMBASSADOR, 1);
    let supply = g.var(SUPPLY);
    act(&rules, &mut g, std_act(0, "remove"));
    // 066: an ambassador pays for it, and nothing else does.
    assert_eq!(
        (spent(&g, 0), g.tokens(d, ADV_AMBASSADOR), g.var(SUPPLY), g.resource(0, Resource::Ambassadors)),
        ((0, 0, 0), 0, supply + 1, 0)
    );

}

#[test]
fn a_card_s_harden_action_costs_chips() {
    let (rules, mut g) = actions_with_room();
    let p = presence_of(&g, 0)[0];
    put(&mut g, p, STATION, 1);
    put(&mut g, p, &cube(0), 1);
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Chips, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("harden_chips"));
    g.piles.insert(crate::modules::cards::built(0).into(), Arc::new(vec![crate::key::Key::new("hardened_bunker_complex")]));
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
    let p = presence_of(&g, 0)[0];
    put(&mut g, p, STATION, 1);
    put(&mut g, p, &cube(0), 1);
    rules.apply_mut(&mut g, &std_act(0, "harden_buy"), None).unwrap();
    pick(&rules, &mut g, p);
    assert_eq!(g.tokens(p, HARDEN), 1);

    // A drain needs compute on a project.
    g.piles.insert(PROJECTS.into(), vec!["market_crash".into()].into());
    assert!(rules.apply_mut(&mut g.clone(), &std_act(0, "drain"), None).is_err());
    g.vars.insert(adversary::project_key("market_crash").into(), 5);
    rules.apply_mut(&mut g, &std_act(0, "drain"), None).unwrap();
    answer_all(&rules, &mut g);
    assert_eq!(g.var(&adversary::project_key("market_crash")), 5 - w.int("board.drain_amount"));
}

/// 020: moving the seat's only ambassador onto the hex it already holds was
/// offered, charged the energy and moved nothing.
#[test]
fn moving_an_ambassador_never_targets_the_hex_it_already_holds() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    for p in hexes_where(m, |h| kind(m, h) != HexKind::Ocean) {
        let mut g = g0.clone();
        put(&mut g, p, &presence(0), 1);
        put(&mut g, p, &ambassador(0), 1);
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
            None => assert_eq!(g.tokens(p, &ambassador(0)), 0, "hex {p}: paid and moved nothing"),
        }
    }
}

#[test]
fn aligning_puts_a_cube_on_a_clean_city_in_reach() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let (city, next) = city_with_lonely_neighbors(m);
    put(&mut g, next[0], &presence(0), 1);
    set_res(&mut g, 0, Resource::Energy, 0);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    act(&rules, &mut g, std_act(0, "align"));
    assert_eq!(g.tokens(city, &cube(0)), 1);
    assert_eq!(g.resource(0, Resource::Ambassadors), 0);
    // A corrupted or aligned city can't be aligned.
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("align"));
}

/// 022 (D10): hardening goes only on facilities a token can protect (a
/// player-owned one, not the adversary's), and "harden 2" is two
/// different buildings (fewer if fewer exist).
#[test]
fn hardening_goes_on_different_buildings_a_token_can_protect() {
    use crate::content::Op;
    use crate::modules::effects;
    let (rules, mut g0) = lab(&[]);
    let m = &rules.map;
    let me = m.numbered(0, piece(0).presence);
    put(&mut g0, me, &presence(0), 1);
    let near: Vec<usize> = m.hexes[me]
        .neighbors
        .iter()
        .copied()
        .filter(|&h| kind(m, h) != HexKind::Ocean && m.hexes[h].country == 0)
        .collect();
    let (a, b, adv) = (near[0], near[1], near[2]);
    put(&mut g0, a, STATION, 1);
    put(&mut g0, a, &cube(0), 1);
    put(&mut g0, adv, FACTORY, 1);
    put(&mut g0, adv, ADV_CUBE, 1);
    let harden = [Op::Harden {
        count: 2,
    }];
    let run = |g: &mut GameState, picks: Vec<crate::state::Pick>| {
        with_cx_picks(&rules, picks, |cx| {
            effects::run(g, cx, &harden, &mut effects::Ctx::seat(0))
        })
        .unwrap();
    };
    // One building it can protect: one token, not two on it, none on the
    // adversary's factory.
    let mut g = g0.clone();
    run(&mut g, Vec::new());
    assert_eq!((g.tokens(a, HARDEN), g.tokens(adv, HARDEN)), (1, 0));
    // Two: one token each.
    let mut g = g0.clone();
    put(&mut g, b, FACTORY, 1);
    put(&mut g, b, &cube(0), 1);
    let o = modules::common::hex_opt(&rules.world(), b);
    run(&mut g, vec![crate::state::Pick { option: o.option, target: o.target }]);
    assert_eq!((g.tokens(a, HARDEN), g.tokens(b, HARDEN)), (1, 1));
}

// ---------- presence, reach and site claims ----------

#[test]
fn placing_presence_takes_the_next_track_token_and_uncovers_its_income() {
    use crate::modules::ai;
    let (rules, mut g) = actions_with_room();
    let w = rules.world();
    let credit_income = |g: &GameState| {
        ai::track_income(&w, g, 0)
            .iter()
            .filter(|(r, _)| *r == Resource::Credits)
            .map(|(_, n)| n)
            .sum::<i64>()
    };
    assert_eq!(
        (board::track_taken(&g, 0), credit_income(&g), ai::keep2(&w, &g, 0)),
        (1, 0, false)
    );
    set_res(&mut g, 0, Resource::Energy, 5);
    let start = presence_of(&g, 0)[0];
    // A seat 1 presence next to it blocks that hex (one presence a hex).
    let blocked = rules.map.hexes[start]
        .neighbors
        .iter()
        .copied()
        .find(|&h| kind(&rules.map, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, blocked, &presence(1), 1);
    let hexes = board::presence_hexes(&w, &g, 0);
    assert!(!hexes.contains(&blocked));
    assert!(hexes
        .iter()
        .all(|&h| rules.map.adjacent(h, start) && g.tokens(h, &presence(0)) == 0));
    // Deploy, which refreshes here: a test of the track, not of its limit.
    let deploy = Action::Power { seat: 0, power: 1 };
    let place = |rules: &Rules, g: &mut GameState| {
        g.vars.insert("ai.power_used.0.1".into(), 0);
        act(rules, g, deploy.clone());
    };
    place(&rules, &mut g);
    // The 2nd space (the Builder's): +2 credits a round.
    assert_eq!(
        (board::track_taken(&g, 0), credit_income(&g), ai::keep2(&w, &g, 0)),
        (2, 2, false)
    );
    place(&rules, &mut g);
    // The keep upgrade is the card-play track's second space (068).
    assert_eq!((board::track_taken(&g, 0), ai::keep2(&w, &g, 0)), (3, false));
    g.vars.insert(board::track2_key(0).into(), 2);
    assert!(ai::keep2(&w, &g, 0));
    assert_eq!(presence_of(&g, 0).len(), 3);
    // The tracks run out after their last spaces (021: both of them).
    g.vars
        .insert(board::track_key(0).into(), ai::board(&w, 0).presence.len() as i64);
    assert!(ai::can_place(&w, &g, 0));
    g.vars
        .insert(board::track2_key(0).into(), ai::board(&w, 0).presence2.len() as i64);
    assert!(!ai::can_place(&w, &g, 0));
}

#[test]
fn card_targets_on_the_map_must_be_in_reach() {
    use crate::modules::effects;
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let me = m.numbered(0, piece(0).presence);
    put(&mut g, me, &presence(0), 1);
    let near = m.hexes[me].neighbors[0];
    let far = hexes_where(m, |h| m.distance(h, me) > 2 && kind(m, h) != HexKind::Ocean)[0];
    put(&mut g, near, ADV_AMBASSADOR, 1);
    put(&mut g, far, ADV_AMBASSADOR, 1);
    assert_eq!(effects::damage_hexes(&w, &g, 0, 1, 0), vec![near]);
}

#[test]
fn claiming_a_site_needs_presence_and_no_node_on_or_next_to_it() {
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

    // Presence two hexes away: the ambassador just waits.
    let mut g = g0.clone();
    let away = hexes_where(m, |h| {
        m.distance(h, station) == 2 && kind(m, h) == HexKind::Open
    })[0];
    put(&mut g, away, &presence(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(
        (
            g.tokens(station, &cube(0)),
            g.tokens(station, &ambassador(0))
        ),
        (0, 1)
    );

    // Presence next to it: claimed.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(g.tokens(station, &cube(0)), 1);

    // A node next to the presence but not the site does not block it.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    let node_far = m.hexes[next]
        .neighbors
        .iter()
        .copied()
        .find(|&h| m.distance(h, station) == 2)
        .unwrap();
    put(&mut g, node_far, NODE, 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(g.tokens(station, &cube(0)), 1);

    // A node next to the site blocks it, wherever the presence is.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    let node_near = m.hexes[station]
        .neighbors
        .iter()
        .copied()
        .find(|&h| h != next && kind(m, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, node_near, NODE, 1);
    with_cx(&rules, |cx| board::place_ambassador(&mut g, cx, 0, station));
    assert_eq!(
        (
            g.tokens(station, &cube(0)),
            g.tokens(station, &ambassador(0))
        ),
        (0, 1)
    );
    // So does a node on the site itself.
    let mut g = g0.clone();
    put(&mut g, next, &presence(0), 1);
    put(&mut g, station, NODE, 1);
    assert!(!board::can_claim(&rules.world(), &g, 0, station));

    // Aligning a city: likewise.
    let (city, lonely) = city_with_lonely_neighbors(m);
    let mut g = g0.clone();
    put(&mut g, lonely[0], &presence(0), 1);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    assert!(standard_offered(&rules, &g, 0).contains("align"));
    // A node next to the presence, two from the city: still offered.
    let far = m.hexes[lonely[0]]
        .neighbors
        .iter()
        .copied()
        .find(|&h| m.distance(h, city) == 2)
        .unwrap();
    put(&mut g, far, NODE, 1);
    assert!(standard_offered(&rules, &g, 0).contains("align"));
    // A node next to the city: not.
    put(&mut g, far, NODE, 0);
    put(&mut g, lonely[1], NODE, 1);
    assert!(!standard_offered(&rules, &g, 0).contains("align"));
}

#[test]
fn a_track_s_last_steps_cost_more() {
    use crate::modules::ai;
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let b = ai::board(&w, 0);
    let a = rules.map.numbered(0, piece(0).presence);
    put(&mut g, a, presence(0), 1);
    // Both tracks down to their last steps.
    g.vars.insert(board::track_key(0).into(), b.presence.len() as i64 - 1);
    g.vars.insert(board::track2_key(0).into(), b.presence2.len() as i64 - 1);
    let extra = b.presence.last().unwrap().cost.min(b.presence2.last().unwrap().cost);
    assert!(extra > 0);
    let deploy = Action::Power { seat: 0, power: 1 };
    set_res(&mut g, 0, Resource::Energy, extra);
    assert!(rules.apply_mut(&mut g.clone(), &deploy, None).is_err(), "the power's 1 energy too");
    set_res(&mut g, 0, Resource::Energy, extra + 1);
    act(&rules, &mut g, deploy);
    assert_eq!(g.resource(0, Resource::Energy), 0);
}
