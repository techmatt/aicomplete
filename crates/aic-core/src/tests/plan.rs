use super::*;
use crate::modules::plan;

/// 151: `plan::next_rally` must foresee what the build's Rally places, hex by hex (a share where players
/// break a tie): for every event card face up, on set-up boards, with each board's pool at 0-5. Player
/// ambassadors are taken off first (an arriving agent cancels one rather than landing), and
/// the supply is made roomy (the plan doesn't check it).
#[test]
fn the_rally_forecast_matches_the_rally_step() {
    let cards: Vec<String> = content().adversary.iter().map(|c| c.id.clone()).collect();
    let mut c = lab_config(&["adversary.build", ACTIONS]);
    // No compute: a project resolving in the build would place ambassadors too (024).
    c.apply_override("adversary.botnet_compute=0").unwrap();
    let rules = Rules::new(&c).unwrap();
    for seed in 0..3 {
        let mut g0 = rules.new_game(seed);
        answer_all(&rules, &mut g0);
        let players: Vec<Key> = (0..g0.seats.len()).map(|s| Key::new(ambassador(s))).collect();
        for h in Arc::make_mut(&mut g0.hexes).iter_mut() {
            Arc::make_mut(h).tokens.retain(|k, _| !players.contains(k));
        }
        g0.vars.insert(SUPPLY.into(), 1000);
        g0.piles.remove(CURRENT);
        for (k, id) in cards.iter().enumerate() {
            let mut g = g0.clone();
            for b in 0..rules.map.countries.len() {
                g.vars.insert(crate::modules::adversary::pool_key(b).into(), ((k + b) % 6) as i64);
            }
            g.piles.insert(NEXT.into(), vec![id.into()].into());
            let want = plan::next_rally(&rules.world(), &g);
            run(&rules, &mut g);
            answer_all(&rules, &mut g);
            let got: Vec<(usize, i64)> = (0..g.hexes.len())
                .map(|h| (h, g.tokens(h, ADV_AMBASSADOR) - g0.tokens(h, ADV_AMBASSADOR)))
                .filter(|&(_, n)| n != 0)
                .collect();
            let fore: f64 = want.iter().map(|&(_, n)| n).sum();
            assert_eq!(got.iter().map(|&(_, n)| n).sum::<i64>() as f64, fore.round(), "card {id}, seed {seed}");
            for (h, n) in got {
                let share = want.iter().find(|&&(x, _)| x == h).map(|&(_, s)| s);
                let share = share.unwrap_or_else(|| panic!("card {id}, seed {seed}: {h} not foreseen"));
                assert!(n as f64 >= share.floor() && n as f64 <= share.ceil(), "card {id}, seed {seed}, hex {h}");
            }
        }
    }
}
