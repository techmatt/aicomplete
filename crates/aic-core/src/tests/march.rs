//! The march (019).

use super::*;
use crate::modules::board;

/// A land hex `d` from `p` with a land path toward it, for march tests.
fn land_at(m: &Map, p: usize, d: i32) -> usize {
    (0..m.hexes.len())
        .find(|&h| {
            m.distance(p, h) == d
                && kind(m, h) != HexKind::Ocean
                && m.hexes[h]
                    .neighbors
                    .iter()
                    .any(|&n| kind(m, n) != HexKind::Ocean && m.distance(p, n) == d - 1)
        })
        .unwrap()
}

#[test]
fn a_troop_marches_two_steps_toward_presence_then_strikes() {
    let (rules, g0) = lab(&["board.march"]);
    let m = &rules.map;
    assert_eq!(rules.world().int("board.troop_steps"), 2);
    let p = m.numbered(0, piece(0).presence);

    // Three away: two moves, ending next to the presence, which survives the round.
    let mut g = g0.clone();
    put(&mut g, p, &presence(0), 1);
    let t = land_at(m, p, 3);
    put(&mut g, t, TROOP, 1);
    // The view shows where it is heading, two adversary phases out.
    let v = rules.view(&g);
    assert_eq!(v.arrows.len(), 1);
    assert_eq!((v.arrows[0].from, v.arrows[0].to, v.arrows[0].rounds), (t, p, 2));
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    let at = (0..m.hexes.len()).find(|&h| g.tokens(h, TROOP) > 0).unwrap();
    assert_eq!(m.distance(at, p), 1);
    assert_eq!(g.tokens(p, &presence(0)), 1);
    assert_eq!(g.stats.counters["board.troop_moves"], 2);
    let v = rules.view(&g);
    assert_eq!(v.arrows[0].rounds, 1);
    // Next round it strikes, sways no Discord, and is used up.
    let discord = g.var("alignment.discord");
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(p, &presence(0)), total(&g, TROOP)), (0, 0));
    assert_eq!(g.stats.counters["board.troop_destroyed.presence"], 1);
    assert_eq!(g.var("alignment.discord"), discord);
    assert!(rules.view(&g).arrows.is_empty());

    // Two away: one move, then it destroys in the same round.
    let mut g = g0.clone();
    put(&mut g, p, &presence(0), 1);
    put(&mut g, land_at(m, p, 2), TROOP, 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(p, &presence(0)), total(&g, TROOP)), (0, 0));
    assert_eq!(g.stats.counters["board.troop_steps"], 2);
}

#[test]
fn troops_ignore_neutral_buildings_and_cities_and_stay_with_no_target() {
    let (rules, mut g) = lab(&["board.march"]);
    let m = &rules.map;
    let f = m.numbered(0, piece(0).factories[0]);
    let city = m.numbered(0, piece(0).cities[0]);
    put(&mut g, f, FACTORY, 1);
    let t = land_at(m, f, 1);
    put(&mut g, t, TROOP, 1);
    put(&mut g, city, &cube(0), 1);
    run(&rules, &mut g);
    assert_eq!((g.tokens(t, TROOP), g.tokens(f, FACTORY)), (1, 1));
    assert!(rules.view(&g).arrows.is_empty());
    // A player-owned factory is a target.
    put(&mut g, f, &cube(0), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!((total(&g, TROOP), g.tokens(f, FACTORY)), (0, 0));
    assert_eq!(g.stats.counters["board.troop_destroyed.factory"], 1);
}

#[test]
fn equally_near_targets_are_the_players_choice() {
    let (rules, g0) = lab(&["board.march"]);
    let m = &rules.map;
    let w = rules.world();
    // A troop with two presence tokens equally far on foot, each alone
    // reachable by exactly one best step, the two steps different: with both
    // on the board the players choose between them.
    let land: Vec<usize> = (0..m.hexes.len()).filter(|&h| kind(m, h) != HexKind::Ocean).collect();
    let steps = |t: usize, targets: &[usize]| {
        let mut g = g0.clone();
        for (i, &x) in targets.iter().enumerate() {
            put(&mut g, x, &presence(i), 1);
        }
        board::troop_steps_from(&w, &g, t)
    };
    let (t, a, b) = land
        .iter()
        .flat_map(|&t| land.iter().map(move |&a| (t, a)))
        .filter(|&(t, a)| m.distance(t, a) == 3)
        .flat_map(|(t, a)| land.iter().map(move |&b| (t, a, b)))
        .find(|&(t, a, b)| {
            a < b && m.distance(t, b) == 3 && {
                let (sa, sb) = (steps(t, &[a]), steps(t, &[b]));
                sa.len() == 1 && sb.len() == 1 && sa != sb && steps(t, &[a, b]).len() == 2
            }
        })
        .expect("a troop between two equally near targets");
    let mut g = g0.clone();
    put(&mut g, t, TROOP, 1);
    put(&mut g, a, &presence(0), 1);
    put(&mut g, b, &presence(1), 1);
    run(&rules, &mut g);
    let p = g.pending.as_ref().expect("a step choice");
    assert!(p.ask.prompt.contains("moves to which hex"), "{}", p.ask.prompt);
    assert_eq!(p.ask.options.len(), 2);
}

#[test]
fn a_troop_walking_onto_an_unmarched_troop_marches_once_and_so_does_it() {
    let (rules, g0) = lab(&["board.march"]);
    let m = &rules.map;
    let w = rules.world();
    let land = |h: usize| kind(m, h) != HexKind::Ocean;
    // Troop A (lower hex number, so it marches first) whose first step is
    // onto troop B's hex; the target 4 hexes from A on foot.
    let p = m.numbered(0, piece(0).presence);
    let mut g = g0.clone();
    put(&mut g, p, &presence(0), 1);
    let (a, b) = (0..m.hexes.len())
        .filter(|&a| land(a) && m.distance(a, p) == 4)
        .find_map(|a| {
            let first = *board::troop_steps_from(&w, &g, a).first()?;
            (a < first && m.distance(first, p) == 3).then_some((a, first))
        })
        .expect("a troop whose first step lands on a later hex");
    put(&mut g, a, TROOP, 1);
    put(&mut g, b, TROOP, 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    // Each marched exactly 2 steps, no strike: both still on the board, the
    // presence untouched, one 2 hexes away and one next to it.
    assert_eq!(g.stats.counters["board.troop_moves"], 4);
    assert_eq!((total(&g, TROOP), g.tokens(p, &presence(0))), (2, 1));
    let mut gaps: Vec<i32> = (0..m.hexes.len())
        .flat_map(|h| std::iter::repeat_n(m.distance(h, p), g.tokens(h, TROOP) as usize))
        .collect();
    gaps.sort();
    assert_eq!(gaps, vec![1, 2]);
}

#[test]
fn with_troop_spent_off_a_troop_strikes_on_both_steps() {
    let (rules, mut g) = lab_with(&["board.march"], &["board.troop_spent=0"]);
    let m = &rules.map;
    let p = m.numbered(0, piece(0).presence);
    let q = m.hexes[p]
        .neighbors
        .iter()
        .copied()
        .find(|&h| kind(m, h) != HexKind::Ocean)
        .unwrap();
    put(&mut g, p, TROOP, 1);
    put(&mut g, p, &presence(0), 1);
    put(&mut g, q, &presence(1), 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.stats.counters["board.troop_strikes"], 2);
    assert_eq!((presence_of(&g, 0).len(), presence_of(&g, 1).len(), total(&g, TROOP)), (0, 0, 1));
}

#[test]
fn troops_walk_around_ocean_rather_than_stall() {
    // A troop whose every straight-line step toward the target is ocean
    // still walks (019: distance is the walk over land).
    let (rules, g0) = lab(&["board.march"]);
    let m = &rules.map;
    let land = |h: usize| kind(m, h) != HexKind::Ocean;
    let (t, p) = (0..m.hexes.len())
        .filter(|&t| land(t))
        .flat_map(|t| (0..m.hexes.len()).map(move |p| (t, p)))
        .find(|&(t, p)| {
            land(p)
                && m.distance(t, p) >= 3
                && m.hexes[t].neighbors.iter().any(|&n| land(n))
                && !m.hexes[t]
                    .neighbors
                    .iter()
                    .any(|&n| land(n) && m.distance(n, p) < m.distance(t, p))
        })
        .expect("a bay between a land hex and a target");
    let mut g = g0.clone();
    put(&mut g, p, &presence(0), 1);
    put(&mut g, t, TROOP, 1);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(t, TROOP), 0, "the troop left {t}");
    assert!(g.stats.counters["board.troop_moves"] >= 1);
}
