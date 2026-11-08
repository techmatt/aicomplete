//! 017: the adversary project deck, compute and its counterplay, and (019)
//! tier-2 botnet removal.

use super::*;
use crate::modules::adversary::{self, PROJECTS_DONE, PROJECTS_ONGOING, PROJECT_DECK};

#[test]
fn setup_deals_one_project_into_each_board_slot() {
    // 135: one shuffled deck, dealt into the adversary board's slots by the
    // draw rule (a tagged slot takes a card of its tag or Universal).
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let content = crate::content();
    let slots = &content.adversary_board.project_slots;
    for seed in 0..8 {
        let g = rules.new_game(seed);
        let row = g.pile(PROJECTS);
        assert_eq!(row.len(), slots.len(), "seed {seed}: {row:?}");
        let all = row.len()
            + g.pile(PROJECT_DECK).len()
            + g.pile(adversary::PROJECT_DISCARD).len()
            + g.pile(PROJECTS_DONE).len()
            + g.pile(PROJECTS_ONGOING).len();
        assert_eq!(all, content.projects.len());
        for (slot, id) in slots.iter().zip(row) {
            let tags = &content.project(id).tags;
            if let Some(t) = slot.tag {
                assert!(
                    tags.contains(&t) || tags.contains(&crate::vocab::ThreatTag::Universal),
                    "seed {seed}: {id} in a {t:?} slot"
                );
            }
            // Cost is per player.
            let c = adversary::project_cost(&w, &g, id);
            assert_eq!(c, content.project(id).cost * g.seats.len() as i64);
        }
        assert_eq!(g.stats.counters.get("adversary.project_pull_forced"), None);
    }
}

#[test]
fn compute_fills_the_cheapest_project_which_resolves_when_full() {
    // Costs and Disinformation's sway pinned as printed at r65.
    let (rules, mut g) = lab_with(
        &[],
        &[
            "projects.botnet_swarm.cost=6",
            "projects.disinformation.cost=5",
            "projects.market_crash.cost=5",
            r#"projects.disinformation.effects=[{"op":"sway","side":"discord","amount":5}]"#,
        ],
    );
    // At 2 players: Botnet Swarm 12, Disinformation 10, Market Crash 10.
    row(&mut g, &["botnet_swarm", "disinformation", "market_crash"]);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 11)).unwrap();
    // Disinformation (the first of the cheapest) filled at 10 and resolved:
    // 5 (142; 4 before) toward Discord. The 11th point went onto Market Crash.
    assert_eq!(g.pile(PROJECTS_DONE), ["disinformation"]);
    assert_eq!(g.var("alignment.discord"), 5);
    assert_eq!(g.var(&adversary::project_key("market_crash")), 1);
    assert_eq!(g.var(&adversary::project_key("botnet_swarm")), 0);
    assert_eq!(g.var(adversary::COMPUTE), 11);
    assert_eq!(g.stats.counters["adversary.project.disinformation"], 1);
    assert_eq!(g.stats.per_round["adversary.compute"][1], 11);
}

#[test]
fn a_new_row_is_dealt_into_the_slots_when_all_have_resolved() {
    // Costs pinned as printed at r65.
    let (rules, mut g) = lab_with(
        &[],
        &[
            "projects.disinformation.cost=5",
            "projects.station_coup.cost=9",
            "projects.recruitment_drive.cost=11",
            "projects.armored_columns.cost=10",
            "projects.grid_capture.cost=9",
        ],
    );
    row(&mut g, &["disinformation"]);
    g.piles.insert(
        PROJECT_DECK.into(),
        [
            "station_coup",
            "recruitment_drive",
            "armored_columns",
            "war_footing",
        ]
        .map(Key::new)
        .to_vec()
        .into(),
    );
    // Setup's misses: cleared, so the deck never reshuffles here.
    set_pile(&mut g, adversary::PROJECT_DISCARD, &[]);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 11)).unwrap();
    // Disinformation (10) resolved: Sway 5 toward Discord.
    assert_eq!(g.pile(PROJECTS_DONE), ["disinformation"]);
    assert_eq!(g.var("alignment.discord"), 5);
    // 135: the slots are Military, any, any: Station Coup and Recruitment Drive
    // take the open slots, Armored Columns the Military one; the row is slot order.
    assert_eq!(
        g.pile(PROJECTS),
        ["armored_columns", "station_coup", "recruitment_drive"]
    );
    assert_eq!(g.pile(PROJECT_DECK), ["war_footing"]);
    // The 11th point went onto the cheapest of the new row (Station Coup, 18).
    assert_eq!(g.var(&adversary::project_key("station_coup")), 1);
    // An ongoing project (Grid Capture, 18) goes into play and acts every round's build.
    row(&mut g, &["grid_capture"]);
    g.vars.insert(adversary::project_key("grid_capture").into(), 17);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 1)).unwrap();
    assert_eq!(g.pile(PROJECTS_ONGOING), ["grid_capture"]);
    // The deck holds one card: it fills the Military slot, the rest stay empty.
    assert_eq!(g.pile(PROJECTS), ["war_footing"]);
}

#[test]
fn an_ongoing_project_acts_in_the_build() {
    // Propaganda Network's ongoing line pinned as printed at r65: Sway 2 toward Discord.
    let (rules, mut g) = lab_with(
        &["adversary.build"],
        &[r#"projects.propaganda_network.effects=[{"op":"sway","side":"discord","amount":2}]"#],
    );
    set_pile(&mut g, PROJECTS_ONGOING, &["propaganda_network"]);
    set_pile(&mut g, NEXT, &["mil_s1_probing_strikes"]);
    let before = g.var("alignment.discord");
    run(&rules, &mut g);
    // 151: the card's troops land on the lowest-numbered factory (else city), asking nothing.
    assert!(g.pending.is_none());
    assert_eq!(g.var("alignment.discord"), before + 2);
}

/// 145: an ongoing project filled this round (here by the build's botnet
/// compute) acts from next round's build, not in the build that filled it.
#[test]
fn an_ongoing_project_filled_in_the_build_waits_a_round() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let w = rules.world();
    row(&mut g, &["grid_capture"]);
    let cost = adversary::project_cost(&w, &g, "grid_capture");
    g.vars.insert(adversary::project_key("grid_capture").into(), cost - 1);
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, 1)).unwrap();
    assert_eq!(g.pile(PROJECTS_ONGOING), ["grid_capture"]);
    assert!(adversary::ongoing_acting(&g).is_empty(), "not this round");
    g.round += 1;
    assert_eq!(adversary::ongoing_acting(&g).len(), 1, "from the next");
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
    set_pile(&mut g, NEXT, &["mil_s1_probing_strikes"]);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    // The free botnet's 3 (lab); 151: the card prints no compute.
    assert_eq!(g.var(adversary::COMPUTE), before + 3);
    assert_eq!(g.tokens(a, ambassador(0)), 1);
    assert_eq!(g.stats.counters["adversary.compute_blocked"], 3);
}

#[test]
fn a_fifth_to_two_thirds_of_the_threats_carry_compute() {
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
    // 151: compute comes mostly from the adversary's income and botnets (142: most
    // tier-1 threats carry 3 again, 36 of 64).
    assert!(t * 5 >= n && t * 3 <= n * 2, "{t} of {n}");
}

#[test]
fn an_uncountered_threat_puts_its_compute_on_a_project() {
    let (rules, mut g) = threat_lab_with(&[r#"threats.embargo.effects=[{"op":"compute","amount":3}]"#]);
    row(&mut g, &["false_flag"]);
    // Embargo, pinned to 3 compute.
    deal(&mut g, "embargo", 0);
    pass_round(&rules, &mut g);
    assert_eq!(g.stats.counters["threats.executed"], 1);
    assert_eq!(g.var(&adversary::project_key("false_flag")), 3);
}

#[test]
fn security_cards_drain_or_discard_a_project() {
    // 119: both cards pinned to r46's print, under what `fund` pays.
    let (rules, mut g) = lab_actions_with(&[
        "cards.zero_knowledge_exploit.cost=6",
        r#"cards.zero_knowledge_exploit.effects=[{"op":"drain_project","amount":5},{"op":"draw","count":1}]"#,
        "cards.killswitch_directive.cost=14",
        r#"cards.killswitch_directive.effects=[{"op":"discard_project"},{"op":"remove_ambassadors_country","unity_per":0,"max":4,"reach":true}]"#,
    ]);
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
    // 119: Singularity Exploit pinned to r46's price and need of 3 Cyber.
    let (rules, mut g) = lab_actions_with(&[
        "cards.singularity_exploit.cost=4",
        "cards.singularity_exploit.antimatter=1",
        r#"cards.singularity_exploit.needs=[["cyber",3]]"#,
    ]);
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
    // Only a botnet within reach. r48: nothing to act on never blocks a card, so with
    // a project to discard it plays with no botnet in reach; with neither it doesn't.
    assert!(rules.legal_actions(&g).contains(&play));
    let mut none = g.clone();
    row(&mut none, &[]);
    assert!(!rules.legal_actions(&none).contains(&play));
    put(&mut g, near, BOTNET, 1);
    assert!(rules.legal_actions(&g).contains(&play));
    act(&rules, &mut g, play);
    assert_eq!((g.tokens(near, BOTNET), g.tokens(far, BOTNET)), (0, 1));
    assert_eq!(g.stats.counters["adversary.botnets_removed"], 1);
}

/// 139: Purge the Unaligned takes an ambassador from each seat, never a node.
#[test]
fn purge_the_unaligned_takes_an_ambassador_never_a_node() {
    let (rules, mut g) = lab(&[]);
    let homes = [0, 1].map(|c| rules.map.numbered(c, piece(c).node));
    for (s, &h) in homes.iter().enumerate() {
        put(&mut g, h, node(s), 2);
        set_res(&mut g, s, Resource::Ambassadors, 1);
    }
    // One factory a country, so its troop asks nothing (024).
    for c in 0..2 {
        put(&mut g, rules.map.numbered(c, piece(c).factories[0]), FACTORY, 1);
    }
    row(&mut g, &["purge_the_unaligned"]);
    let cost = adversary::project_cost(&rules.world(), &g, "purge_the_unaligned");
    with_cx(&rules, |cx| adversary::gain_compute(&mut g, cx, cost)).unwrap();
    assert_eq!(g.pile(PROJECTS_DONE), ["purge_the_unaligned"]);
    assert_eq!((total(&g, node(0)), total(&g, node(1))), (2, 2));
    assert_eq!((g.resource(0, Resource::Ambassadors), g.resource(1, Resource::Ambassadors)), (0, 0));
}
