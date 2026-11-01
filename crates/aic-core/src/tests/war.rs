use super::*;
use crate::modules::board;

#[test]
fn the_clock_sets_production_and_war_steps_and_the_war_twist() {
    let rules = Rules::new(&baseline()).unwrap();
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
    g.piles
        .insert(CURRENT.into(), vec!["adv_s2_border_war".into()].into());
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
    let d1 = m.numbered(1, piece(1).presence);
    put(&mut g0, d1, &presence(1), 1);
    g0.round = 2;
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");
    let units = |g: &GameState| (g.tokens(b0, UNIT), g.tokens(b1, UNIT));

    // 3 against 1: two cross, and take both factories (not the presence).
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
            g.tokens(d1, &presence(1))
        ),
        (0, 0, 1)
    );
    assert_eq!(discord(&g), 2);

    // A destroyed factory takes the presence on its hex (027); a presence
    // elsewhere stays, and a surplus with nothing to hit is spent.
    let mut g = g0.clone();
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, f1, &presence(1), 1);
    put(&mut g, b0, UNIT, 3);
    run(&rules, &mut g);
    assert_eq!((g.tokens(f1, FACTORY), g.tokens(f1, &presence(1)), g.tokens(d1, &presence(1))), (0, 0, 1));
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

    // At peace nothing fights.
    let mut g = g0.clone();
    g.round = 1;
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, b0, UNIT, 3);
    run(&rules, &mut g);
    assert_eq!((units(&g), g.tokens(f1, FACTORY)), ((3, 0), 1));
}

#[test]
fn bases_hold_at_most_three_units() {
    let (rules, mut g) = lab(&["board.bases"]);
    let m = &rules.map;
    for &f in &piece(0).factories {
        put(&mut g, m.numbered(0, f), FACTORY, 1);
    }
    let [b0, b1] = m.fronts[0].bases;
    run(&rules, &mut g);
    assert_eq!((g.tokens(b0, UNIT), g.tokens(b1, UNIT)), (3, 1));
    for _ in 0..3 {
        run(&rules, &mut g);
    }
    assert_eq!((g.tokens(b0, UNIT), g.tokens(b1, UNIT)), (3, 3));
}

#[test]
fn adversary_troops_strike_buildings_next_to_them() {
    let (rules, mut g0) = lab(&["board.march"]);
    let m = &rules.map;
    let (city, n) = city_with_lonely_neighbors(m);
    let at = n[0];
    let next: Vec<usize> = m.hexes[at]
        .neighbors
        .iter()
        .copied()
        .filter(|&x| x != city && kind(m, x) != HexKind::Ocean)
        .collect();
    put(&mut g0, at, TROOP, 1);
    let discord = |g: &GameState| g.var("alignment.discord") - g0.var("alignment.discord");

    let mut g = g0.clone();
    put(&mut g, next[0], FACTORY, 1);
    put(&mut g, next[0], &cube(0), 1);
    run(&rules, &mut g);
    // A troop strike sways no Discord (only war does), and uses the troop up.
    assert_eq!(
        (g.tokens(next[0], FACTORY), discord(&g), g.tokens(at, TROOP)),
        (0, 0, 0)
    );

    // Two in reach: the players choose.
    let mut g = g0.clone();
    put(&mut g, next[0], FACTORY, 1);
    put(&mut g, next[0], &cube(0), 1);
    put(&mut g, next[1], &presence(0), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a strike choice");
    assert!(
        p.ask.prompt.starts_with("Adversary troop"),
        "{}",
        p.ask.prompt
    );
    assert_eq!(p.ask.options.len(), 2);
    let label = p
        .ask
        .options
        .iter()
        .find(|o| o.option.starts_with("seat 0's presence"))
        .unwrap()
        .option
        .clone();
    answer_with(&rules, &mut g, &label);
    assert_eq!(
        (
            g.tokens(next[0], FACTORY),
            g.tokens(next[1], &presence(0)),
            discord(&g),
            g.tokens(at, TROOP)
        ),
        (1, 0, 0, 0)
    );
}

#[test]
fn a_troop_that_strikes_is_used_up_and_one_with_no_target_stays() {
    let (rules, mut g0) = lab(&["board.march"]);
    let m = &rules.map;
    assert_eq!(rules.world().int("board.troop_spent"), 1);
    let (city, n) = city_with_lonely_neighbors(m);
    let at = n[0];
    let target = m.hexes[at]
        .neighbors
        .iter()
        .copied()
        .find(|&x| x != city && kind(m, x) != HexKind::Ocean)
        .unwrap();
    put(&mut g0, at, TROOP, 2);

    // Nothing in reach: both stay, round after round.
    let mut g = g0.clone();
    run(&rules, &mut g);
    run(&rules, &mut g);
    assert_eq!(g.tokens(at, TROOP), 2);
    assert!(!g.stats.counters.contains_key("board.troops_spent"));

    // One building in reach: the first troop destroys it and is used up;
    // the second finds nothing left and stays.
    let mut g = g0.clone();
    put(&mut g, target, FACTORY, 1);
    put(&mut g, target, &cube(0), 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(target, FACTORY), g.tokens(at, TROOP)), (0, 1));
    assert_eq!(g.stats.counters["board.troops_spent"], 1);

    // With `troop_spent` 0 a troop strikes and stays.
    let (rules, mut g) = lab_with(&["board.march"], &["board.troop_spent=0"]);
    put(&mut g, at, TROOP, 1);
    put(&mut g, target, FACTORY, 1);
    put(&mut g, target, &cube(0), 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(target, FACTORY), g.tokens(at, TROOP)), (0, 1));
}

