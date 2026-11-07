use super::*;
use crate::modules::board;

#[test]
fn at_war_units_cancel_and_the_surplus_destroys_rival_buildings() {
    let (rules, mut g0) = lab(&["board.war"]);
    let m = &rules.map;
    let [b0, b1] = m.fronts[0].bases;
    let [f1, f2] = [0, 1].map(|i| m.numbered(1, piece(1).factories[i]));
    let d1 = m.numbered(1, piece(1).node);
    put(&mut g0, d1, node(1), 1);
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let units = |g: &GameState| (g.tokens(b0, UNIT), g.tokens(b1, UNIT));

    // 3 against 1: two cross, and take both factories (not the node).
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, f2, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    put(&mut g, b1, UNIT, 1);
    run(&rules, &mut g);
    assert_eq!(units(&g), (0, 0));
    assert_eq!(
        (
            g.tokens(f1, FACTORY),
            g.tokens(f2, FACTORY),
            g.tokens(d1, node(1))
        ),
        (0, 0, 1)
    );
    assert_eq!(discord(&g), 2);
    assert_eq!(g.stats.counters["alignment.discord_from.war"], 2);

    // A destroyed factory takes the node on its hex (027); a node
    // elsewhere stays, and a surplus with nothing to hit is spent.
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, f1, node(1), 1);
    put(&mut g, b0, UNIT, 3);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f1, FACTORY), g.tokens(f1, node(1)), g.tokens(d1, node(1))), (0, 0, 1));
    assert_eq!((units(&g), discord(&g)), ((0, 0), 1));

    // Hardening absorbs one.
    let mut g = g0.clone();
    for f in [f1, f2] {
        put(&mut g, f, FACTORY, 1);
        put(&mut g, f, HARDEN, 1);
    }
    put(&mut g, b0, UNIT, 2);
    put(&mut g, b1, UNIT, 1);
    run(&rules, &mut g);
    assert_eq!(
        (total(&g, FACTORY), total(&g, HARDEN), discord(&g)),
        (2, 1, 0)
    );

    // Equal forces cancel; the other way round hits the other country.
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b0, UNIT, 2);
    put(&mut g, b1, UNIT, 2);
    run(&rules, &mut g);
    assert_eq!((units(&g), g.tokens(f1, FACTORY)), ((0, 0), 1));
    let mut g = g0.clone();
    let f0 = m.numbered(0, piece(0).factories[0]);
    put(&mut g, f0, FACTORY, 1);
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b1, UNIT, 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f0, FACTORY), g.tokens(f1, FACTORY)), (0, 1));
}

/// 137: war runs after the threats execute, last before cleanup.
#[test]
fn war_comes_after_the_threats_execute() {
    let rules = Rules::new(&baseline()).unwrap();
    let war = rules.config.round.iter().position(|p| p == "board.war").unwrap();
    let threats = rules.config.round.iter().position(|p| p == "threats.execute").unwrap();
    assert!(threats < war, "{:?}", rules.config.round);
}

/// 137: a Discord tip's corp troops are held to `unit_cap` a base.
#[test]
fn bases_hold_at_most_three_units() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    for &f in &piece(0).factories {
        put(&mut g, m.numbered(0, f), FACTORY, 1);
    }
    let made = piece(0).factories.len() as i64;
    let [b0, b1] = m.fronts[0].bases;
    let cap = w.int("board.unit_cap");
    assert_eq!(cap, 3);
    with_cx(&rules, |cx| board::tip_corp_troops(&mut g, cx));
    assert_eq!((g.tokens(b0, UNIT), g.tokens(b1, UNIT)), (made.min(cap), 0));
    for _ in 0..3 {
        with_cx(&rules, |cx| board::tip_corp_troops(&mut g, cx));
    }
    assert_eq!((g.tokens(b0, UNIT), g.tokens(b1, UNIT)), (cap, 0));
}

