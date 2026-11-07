use super::*;
use crate::modules::plan;

/// `plan::next_build` must foresee exactly what the build phase places,
/// hex by hex: for every adversary card face up, on set-up boards, at
/// difficulties below, at and above the build bonus. Player ambassadors
/// are taken off first (an arriving adversary ambassador cancels one rather
/// than landing), and the supply is made roomy (the plan doesn't check it).
#[test]
fn the_build_forecast_matches_the_build_step() {
    let cards: Vec<String> = content().adversary.iter().map(|c| c.id.clone()).collect();
    for difficulty in [0, 3, 4] {
        let mut c = lab_config(&["adversary.build", ACTIONS]);
        c.apply_override(&format!("adversary.difficulty={difficulty}"))
            .unwrap();
        // No compute: a project resolving in the build would place ambassadors too (024).
        c.apply_override("adversary.botnet_compute=0").unwrap();
        let rules = Rules::new(&c).unwrap();
        for seed in 0..3 {
            let mut g0 = rules.new_game(seed);
            // 137: round 1's card may land troops on a factory of the players' choice.
            answer_all(&rules, &mut g0);
            let players: Vec<Key> = (0..g0.seats.len()).map(|s| Key::new(ambassador(s))).collect();
            for h in Arc::make_mut(&mut g0.hexes).iter_mut() {
                Arc::make_mut(h).tokens.retain(|k, _| !players.contains(k));
            }
            g0.vars.insert(SUPPLY.into(), 1000);
            g0.piles.remove(CURRENT);
            for id in &cards {
                let mut g = g0.clone();
                g.piles.insert(NEXT.into(), vec![id.into()].into());
                let want = plan::next_build(&rules.world(), &g);
                run(&rules, &mut g);
                // 135: the card's standing troops ask where they land first.
                answer_all(&rules, &mut g);
                let got: Vec<(usize, i64)> = (0..g.hexes.len())
                    .map(|h| {
                        (
                            h,
                            g.tokens(h, ADV_AMBASSADOR) - g0.tokens(h, ADV_AMBASSADOR),
                        )
                    })
                    .filter(|&(_, n)| n != 0)
                    .collect();
                assert_eq!(got, want, "card {id}, difficulty {difficulty}, seed {seed}");
            }
        }
    }
}
