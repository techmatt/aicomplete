//! 017: the adversary project deck, compute and its counterplay, and (019)
//! tier-2 botnet removal.

use super::*;
use crate::modules::adversary::{self, PROJECTS_DONE, PROJECTS_ONGOING, PROJECT_DECK};

#[test]
fn setup_deals_three_cheap_projects_from_a_staged_deck() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let g = rules.new_game(3);
    let all: Vec<Key> = g
        .pile(PROJECTS)
        .iter()
        .chain(g.pile(PROJECT_DECK))
        .chain(g.pile(PROJECTS_DONE))
        .chain(g.pile(PROJECTS_ONGOING))
        .cloned()
        .collect();
    assert_eq!(all.len(), 12);
    let stage =
        |id: &str| crate::content().projects[crate::content().project_index(id).unwrap()].stage;
    // Cheap (stage 1) on top: the first six are cheap, the rest big.
    let stages: Vec<u8> = all.iter().map(|id| stage(id)).collect();
    assert!(stages[..6].iter().all(|&s| s == 1), "{stages:?}");
    assert!(stages[6..].iter().all(|&s| s == 2), "{stages:?}");
    // Cost is per player: 6–7 a player for a cheap one (12–14 at 2).
    for id in g.pile(PROJECTS) {
        let c = adversary::project_cost(&w, &g, id);
        assert!((12..=14).contains(&c), "{id} costs {c}");
    }
}

#[test]
fn compute_fills_the_cheapest_project_which_resolves_when_full() {
    let (rules, mut g) = lab(&[]);
    // At 2 players: False Flag 12, Disinformation 10, Botnet Swarm 10.
    row(&mut g, &["false_flag", "disinformation", "botnet_swarm"]);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 11)).unwrap();
    // Disinformation (the first of the cheapest) filled at 10 and resolved:
    // 4 toward Discord. The 11th point went onto Botnet Swarm.
    assert_eq!(g.pile(PROJECTS_DONE), ["disinformation"]);
    assert_eq!(g.var("alignment.discord"), 4);
    assert_eq!(g.var(&adversary::project_key("botnet_swarm")), 1);
    assert_eq!(g.var(&adversary::project_key("false_flag")), 0);
    assert_eq!(g.var(adversary::COMPUTE), 11);
    assert_eq!(g.stats.counters["adversary.projects_resolved.cheap"], 1);
    assert_eq!(g.stats.per_round["adversary.compute"][1], 11);
}

#[test]
fn a_new_row_is_dealt_when_all_three_have_resolved() {
    let (rules, mut g) = lab(&[]);
    row(&mut g, &["market_crash"]);
    g.piles.insert(
        PROJECT_DECK.into(),
        [
            "station_coup",
            "recruitment_drive",
            "total_offensive",
            "armored_columns",
        ]
        .map(Key::new)
        .to_vec()
        .into(),
    );
    set_res(&mut g, 0, Resource::Credits, 10);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 11)).unwrap();
    // Market Crash (10) resolved: 8 credits each (as much as they have).
    assert_eq!(g.resource(0, Resource::Credits), 2);
    assert_eq!(
        g.pile(PROJECTS),
        ["station_coup", "recruitment_drive", "total_offensive"]
    );
    assert_eq!(g.pile(PROJECT_DECK), ["armored_columns"]);
    // The 13th point went onto the cheapest of the new row (Recruitment
    // Drive, 20).
    assert_eq!(g.var(&adversary::project_key("recruitment_drive")), 1);
    // An ongoing project goes into play and acts every round's build.
    g.vars
        .insert(adversary::project_key("recruitment_drive").into(), 19);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 1)).unwrap();
    assert_eq!(g.pile(PROJECTS_ONGOING), ["recruitment_drive"]);
}

#[test]
fn an_ongoing_project_acts_in_the_build() {
    let (rules, mut g) = lab(&["adversary.build"]);
    set_pile(&mut g, PROJECTS_ONGOING, &["recruitment_drive"]);
    set_pile(&mut g, NEXT, &["adv_s1_probing_strikes"]);
    run(&rules, &mut g);
    // Every clean city (lab: all of them) gains an adversary ambassador.
    assert_eq!(total(&g, ADV_AMBASSADOR), cities(&rules.map).len() as i64);
}

#[test]
fn a_player_ambassador_on_a_botnet_stops_its_compute_and_stays() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let m = &rules.map;
    let (a, b) = (
        m.numbered(0, piece(0).stations[0]),
        m.numbered(1, piece(1).stations[0]),
    );
    put(&mut g, a, BOTNET, 1);
    put(&mut g, b, BOTNET, 1);
    put(&mut g, a, ambassador(0), 1);
    let before = g.var(adversary::COMPUTE);
    set_pile(&mut g, NEXT, &["adv_s1_probing_strikes"]);
    run(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 3);
    assert_eq!(g.tokens(a, ambassador(0)), 1);
    assert_eq!(g.stats.counters["adversary.compute_blocked"], 3);
}

#[test]
fn about_half_the_threats_carry_compute() {
    let t = crate::content()
        .threats
        .iter()
        .filter(|t| {
            t.effects
                .iter()
                .any(|e| matches!(e, crate::content::Op::Compute { .. }))
        })
        .count();
    let n = crate::content().threats.len();
    assert!(t * 3 >= n && t * 3 <= 2 * n, "{t} of {n}");
}

#[test]
fn an_uncountered_threat_puts_its_compute_on_a_project() {
    let (rules, mut g) = threat_lab();
    row(&mut g, &["false_flag"]);
    deal(&mut g, "blackout", 0);
    pass_round(&rules, &mut g);
    assert_eq!(g.stats.counters["threats.executed"], 1);
    assert_eq!(g.var(&adversary::project_key("false_flag")), 3);
}

#[test]
fn security_cards_drain_or_discard_a_project() {
    let (rules, mut g) = lab_actions();
    row(&mut g, &["false_flag", "disinformation"]);
    g.vars.insert(adversary::project_key("false_flag").into(), 8);
    set_pile(&mut g, "cards.hand.0", &["zero_knowledge_exploit", "killswitch_directive"]);
    fund(&mut g, 0);
    act(
        &rules,
        &mut g,
        Action::Play {
            seat: 0,
            card: "zero_knowledge_exploit".into(),
        },
    );
    // The only project holding compute: 8 - 5 (Zero-Knowledge Exploit).
    assert_eq!(g.var(&adversary::project_key("false_flag")), 3);
    // 111: Killswitch also removes adversary ambassadors in reach, so one stands there.
    let here = node_of(&g, 0)[0];
    put(&mut g, here, ADV_AMBASSADOR, 1);
    act(
        &rules,
        &mut g,
        Action::Play {
            seat: 0,
            card: "killswitch_directive".into(),
        },
    );
    assert_eq!(g.pile(PROJECTS).len(), 1);
    assert_eq!(g.var(&adversary::project_key("false_flag")), 0);
    assert_eq!(g.stats.counters["adversary.projects_discarded"], 1);
}

#[test]
fn botnet_takedown_removes_a_botnet_in_reach() {
    let (rules, mut g) = lab_actions();
    let m = &rules.map;
    let p = node_of(&g, 0)[0];
    let far = (0..m.hexes.len())
        .find(|&h| m.distance(p, h) >= 3 && g.hexes[h].tokens.is_empty())
        .unwrap();
    let near = m.hexes[p].neighbors[0];
    put(&mut g, far, BOTNET, 1);
    set_pile(&mut g, "cards.hand.0", &["singularity_exploit"]);
    fund(&mut g, 0);
    // It needs 3 Cyber tags in play, and (076) a project for its discard.
    for c in ["eviction_worm", "zero_click_breach", "adaptive_malware"] {
        Arc::make_mut(g.piles.entry(crate::modules::cards::played(0)).or_default()).push(c.into());
    }
    row(&mut g, &["false_flag"]);
    g.vars.insert(adversary::project_key("false_flag").into(), 6);
    let play = Action::Play {
        seat: 0,
        card: "singularity_exploit".into(),
    };
    // Only a botnet within reach.
    assert!(!rules.legal_actions(&g).contains(&play));
    put(&mut g, near, BOTNET, 1);
    assert!(rules.legal_actions(&g).contains(&play));
    act(&rules, &mut g, play);
    assert_eq!((g.tokens(near, BOTNET), g.tokens(far, BOTNET)), (0, 1));
    assert_eq!(g.stats.counters["adversary.botnets_removed"], 1);
}

/// A project that sacrifices node takes each seat's ambassador instead of
/// its last node (020, 027).
#[test]
fn purge_the_unaligned_never_takes_a_last_node() {
    let (rules, mut g) = lab(&[]);
    let homes = [0, 1].map(|c| rules.map.numbered(c, piece(c).node));
    for (s, &h) in homes.iter().enumerate() {
        put(&mut g, h, node(s), 1);
        set_res(&mut g, s, Resource::Ambassadors, 1);
    }
    // One factory a country, so its troop asks nothing (024).
    for c in 0..2 {
        put(&mut g, rules.map.numbered(c, piece(c).factories[0]), FACTORY, 1);
    }
    row(&mut g, &["purge_the_unaligned"]);
    let cost = adversary::project_cost(&rules.world(), &g, "purge_the_unaligned");
    // A last node is never offered (027): each seat's ambassador goes.
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, cost)).unwrap();
    assert_eq!(g.pile(PROJECTS_DONE), ["purge_the_unaligned"]);
    assert_eq!((total(&g, node(0)), total(&g, node(1))), (1, 1));
    assert_eq!(g.stats.counters["alignment.sacrificed_ambassadors"], 2);
}
