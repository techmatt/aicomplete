use super::*;
use crate::modules::board;

#[test]
fn the_clock_sets_production_and_war_steps_and_the_war_twist() {
    // 137: the baseline's clock is 3, production and war every round.
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    g.piles.remove(CURRENT);
    for r in 1..=12 {
        g.round = r;
        assert!(board::war_round(&w, &g) && board::production_round(&w, &g), "round {r}");
    }
    // The dial is kept to restore a printed track.
    // 135: no event card prints a war twist now; one is overlaid to read the rule.
    let rules = Rules::new(&baseline_with(&[
        "board.clock=1,2,1,2,1,3",
        r#"adversary.mil_s2_border_war.twists=[{"op":"war"},{"op":"corrupt","site":"factory"}]"#,
    ]))
    .unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    g.piles.remove(CURRENT);
    let war: Vec<u32> = (1..=12)
        .filter(|&r| {
            g.round = r;
            board::war_round(&w, &g)
        })
        .collect();
    assert_eq!(war, [2, 4, 6, 7, 8, 9, 10, 11, 12]);
    let production: Vec<u32> = (1..=8)
        .filter(|&r| {
            g.round = r;
            board::production_round(&w, &g)
        })
        .collect();
    assert_eq!(production, [1, 3, 5, 6, 7, 8]);
    g.round = 1;
    set_pile(&mut g, CURRENT, &["mil_s2_border_war"]);
    assert!(board::war_round(&w, &g));
    let v = rules.view(&g);
    assert_eq!(
        v.globals
            .iter()
            .find(|x| x.key == "board.war")
            .unwrap()
            .value,
        1
    );
}

#[test]
fn on_a_war_round_units_cancel_and_the_surplus_destroys_rival_buildings() {
    let (rules, mut g0) = lab(&["board.war"]);
    let m = &rules.map;
    let [b0, b1] = m.fronts[0].bases;
    let [f1, f2] = [0, 1].map(|i| m.numbered(1, piece(1).factories[i]));
    let d1 = m.numbered(1, piece(1).node);
    put(&mut g0, d1, node(1), 1);
    // 137: every round is a war round (round 1 was peace).
    g0.round = 1;
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

    // At peace (a production-only dial) nothing fights.
    let (rules, mut g) = lab_with(&["board.war"], &["board.clock=1"]);
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    run(&rules, &mut g);
    assert_eq!((units(&g), g.tokens(f1, FACTORY)), ((3, 0), 1));
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

