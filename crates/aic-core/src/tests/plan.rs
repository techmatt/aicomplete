use super::*;
use crate::modules::{adversary, plan};

/// 275: `plan::next_infiltrate` must foresee exactly what the card's Infiltrate lines place, hex by hex: for every
/// event card face up, on set-up boards with a firewall and an Infiltrate prohibition's token on a city each.
/// Agents and player ambassadors are taken off first (1 agent then corrupts nothing at d0, and none is canceled),
/// and the supply is made roomy (the plan doesn't check it).
#[test]
fn the_infiltrate_forecast_matches_the_infiltrate_lines() {
    use crate::modules::common::{prohibit_token, FIREWALL};
    use crate::op::Step;
    let rules = Rules::new(&lab_config(&["adversary.build", ACTIONS])).unwrap();
    let w = rules.world();
    for seed in 0..3 {
        let mut g0 = rules.new_game(seed);
        answer_all(&rules, &mut g0);
        let gone: Vec<Key> = (0..g0.seats.len()).map(|s| Key::new(ambassador(s))).chain([Key::new(ADV_AMBASSADOR)]).collect();
        for h in Arc::make_mut(&mut g0.hexes).iter_mut() {
            Arc::make_mut(h).tokens.retain(|k, _| !gone.contains(k));
        }
        let open: Vec<usize> = cities(&rules.map).into_iter().filter(|&h| g0.tokens(h, ADV_CUBE) == 0).collect();
        put(&mut g0, open[0], FIREWALL, 1);
        put(&mut g0, open[1], prohibit_token(Step::Infiltrate), 1);
        set_var(&mut g0, SUPPLY, 1000);
        g0.piles.remove(CURRENT);
        for card in &content().adversary {
            let mut g = g0.clone();
            g.piles.insert(NEXT.into(), vec![card.id.as_str().into()].into());
            let mut want: Vec<(usize, i64)> = Vec::new();
            for (h, n) in plan::next_infiltrate(&w, &g) {
                assert_eq!(n.fract(), 0.0, "an exact forecast");
                match want.iter_mut().find(|x| x.0 == h) {
                    Some(x) => x.1 += n as i64,
                    None => want.push((h, n as i64)),
                }
            }
            want.sort_unstable();
            assert!(!want.iter().any(|&(h, _)| h == open[0] || h == open[1]), "{}: a guarded site", card.id);
            for site in adversary::infiltrate_sites(card) {
                with_cx(&rules, |cx| adversary::infiltrate(&mut g, cx, site, None, "test")).unwrap();
            }
            let got: Vec<(usize, i64)> = (0..g.hexes.len())
                .map(|h| (h, g.tokens(h, ADV_AMBASSADOR)))
                .filter(|&(_, n)| n != 0)
                .collect();
            assert_eq!(got, want, "card {}, seed {seed}", card.id);
        }
    }
}
