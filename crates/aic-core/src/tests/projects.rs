//! 017: the adversary project deck, compute and its counterplay, and (019)
//! tier-2 botnet removal.

use super::*;
use crate::modules::adversary::{self, PROJECTS_DONE, PROJECT_DECK};

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
            + g.pile(PROJECTS_DONE).len();
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

/// r80 (ruling #49): compute fills the leftmost project, whatever it costs; when it resolves the rest slide one
/// slot left (keeping their compute) and the right slot refills.
#[test]
fn compute_fills_the_leftmost_project_and_the_rest_slide_left() {
    // Costs and Disinformation's sway pinned as printed at r65.
    let (rules, mut g) = lab_with(
        &[],
        &[
            "projects.botnet_swarm.cost=6",
            "projects.disinformation.cost=5",
            "projects.backdoor_research.cost=5",
            r#"projects.disinformation.effects=[{"op":"sway","side":"discord","amount":5}]"#,
        ],
    );
    // At 2 players: Disinformation 10, Botnet Swarm 12, Backdoor Research 10.
    row(&mut g, &["disinformation", "botnet_swarm", "backdoor_research"]);
    set_var(&mut g, adversary::project_key("backdoor_research"), 2);
    g.piles.insert(PROJECT_DECK.into(), ["station_coup", "crackdown"].map(Key::new).to_vec().into());
    set_pile(&mut g, adversary::PROJECT_DISCARD, &[]);
    with_cx(&rules, |cx| gain_and_pour(&mut g, cx, 9)).unwrap();
    assert_eq!(g.var(&adversary::project_key("disinformation")), 9, "all on the leftmost");
    assert_eq!(g.var(&adversary::project_key("botnet_swarm")), 0);
    with_cx(&rules, |cx| gain_and_pour(&mut g, cx, 2)).unwrap();
    assert_eq!(g.pile(PROJECTS_DONE), ["disinformation"]);
    assert_eq!(g.var("alignment.discord"), 5);
    assert_eq!(g.pile(PROJECTS), ["botnet_swarm", "backdoor_research", "station_coup"], "slid left, refilled at the right");
    assert_eq!(g.var(&adversary::project_key("botnet_swarm")), 1, "the 11th point onto the new leftmost");
    assert_eq!(g.var(&adversary::project_key("backdoor_research")), 2, "a slid project keeps its compute");
    assert_eq!(g.var(adversary::COMPUTE), 11);
    assert_eq!(g.stats.counters["adversary.project.disinformation"], 1);
}

#[test]
fn a_new_row_is_dealt_into_the_slots_when_all_have_resolved() {
    // Costs pinned as printed at r65.
    let (rules, mut g) = lab_with(
        &[],
        &[
            "projects.disinformation.cost=5",
            r#"projects.disinformation.effects=[{"op":"sway","side":"discord","amount":5}]"#,
            "projects.station_coup.cost=9",
            // 259: its research line is a third consequence, refused below cost 18.
            r#"projects.station_coup.effects=[{"op":"take_site","site":"station"},{"op":"fallout","at":"corrupted_station"}]"#,
            "projects.recruitment_drive.cost=11",
            "projects.war_plants.cost=10",
            "projects.lab_purge.cost=9",
        ],
    );
    row(&mut g, &["disinformation"]);
    g.piles.insert(
        PROJECT_DECK.into(),
        [
            "station_coup",
            "recruitment_drive",
            "war_plants",
            "crackdown",
        ]
        .map(Key::new)
        .to_vec()
        .into(),
    );
    // Setup's misses: cleared, so the deck never reshuffles here.
    set_pile(&mut g, adversary::PROJECT_DISCARD, &[]);
    with_cx(&rules, |cx| gain_and_pour(&mut g, cx,11)).unwrap();
    // Disinformation (10) resolved: Sway 5 toward Discord.
    assert_eq!(g.pile(PROJECTS_DONE), ["disinformation"]);
    assert_eq!(g.var("alignment.discord"), 5);
    // r83 (#57): every slot is open: the row is the deck's order.
    assert_eq!(
        g.pile(PROJECTS),
        ["station_coup", "recruitment_drive", "war_plants"]
    );
    assert_eq!(g.pile(PROJECT_DECK), ["crackdown"]);
    // The 11th point went onto the leftmost of the new row (r80).
    assert_eq!(g.var(&adversary::project_key("station_coup")), 1);
    // A project resolves once (231: none is ongoing): Lab Purge (18 at 2 countries).
    row(&mut g, &["lab_purge"]);
    set_var(&mut g, adversary::project_key("lab_purge"), 17);
    with_cx(&rules, |cx| gain_and_pour(&mut g, cx,1)).unwrap();
    assert_eq!(g.pile(PROJECTS_DONE).last().map(|k| k.as_str()), Some("lab_purge"));
    // The deck holds one card: it fills the first slot, the rest stay empty.
    assert_eq!(g.pile(PROJECTS), ["crackdown"]);
}

/// r80 (ruling #41): the four beats name every adversary phase of the baseline round, in round order, in one
/// constant (a rename is one edit).
#[test]
fn the_four_beats_cover_the_adversary_turn_in_order() {
    let cfg = baseline();
    let mut seen: Vec<&str> = Vec::new();
    for p in &cfg.round {
        if let Some(b) = adversary::beat_of(p) {
            if seen.last() != Some(&b) {
                seen.push(b);
            }
        }
    }
    let names: Vec<&str> = adversary::BEATS.iter().map(|(b, _)| *b).collect();
    assert_eq!(seen, names);
    for (_, phases) in adversary::BEATS {
        for p in phases {
            assert!(cfg.round.iter().any(|r| r == p), "{p} is a baseline phase");
        }
    }
}

#[test]
fn a_player_ambassador_on_a_botnet_stops_its_compute_and_stays() {
    // 175: botnets make compute in the compute step, after the reveal.
    let (rules, mut g) = lab_with(&["adversary.reveal", "adversary.compute"], &["adversary.compute_income=0,0,0,0,0"]);
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
    // Embargo, pinned to 3 compute: 175, it waits for the adversary's compute step.
    deal(&mut g, "embargo", 0);
    pass_round(&rules, &mut g);
    assert_eq!(g.stats.counters["threats.executed"], 1);
    assert_eq!(g.var(adversary::WAITING), 3);
    with_cx(&rules, |cx| adversary::pour_waiting(&mut g, cx)).unwrap();
    assert_eq!(g.var(&adversary::project_key("false_flag")), 3);
}

#[test]
fn security_cards_drain_or_discard_a_project() {
    // 119: both cards pinned to r46's print, under what `fund` pays.
    let (rules, mut g) = lab_actions_with(&[
        "cards.zero_knowledge_exploit.cost=6",
        r#"cards.zero_knowledge_exploit.effects=[{"op":"drain_project","amount":5},{"op":"draw","count":1}]"#,
        "cards.killswitch_directive.cost=14",
        r#"cards.killswitch_directive.effects=[{"op":"discard_project"},{"op":"target","on":"agents"},{"op":"remove_ambassadors_country","max":4,"at":"near"}]"#,
    ]);
    row(&mut g, &["false_flag", "disinformation"]);
    // 195: setup's deck may still hold the row's cards; the refill draws one that isn't.
    set_pile(&mut g, PROJECT_DECK, &["crackdown"]);
    set_var(&mut g, adversary::project_key("false_flag"), 8);
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
    // 111: Killswitch also removes agents in reach, so one stands there.
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
    // 175: its slot refills at once.
    assert!(!g.pile(PROJECTS).iter().any(|p| p == "false_flag"));
    assert!(g.pile(PROJECTS).iter().any(|p| p == "disinformation"));
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
    set_var(&mut g, adversary::project_key("false_flag"), 6);
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

/// 195: the project deck's board verbs, each read off the board when it resolves (deterministic targets).
#[test]
fn the_board_verbs_take_the_targets_the_board_shows() {
    use crate::content::Op;
    use crate::modules::common::corrupted;
    use crate::op::Facility;
    // 263: botnets spread from `botnet_spread_at` (1) on; the lab plays d0.
    let (rules, mut g) = lab_with(&[], &["adversary.botnet_spread_at=0"]);
    let m = &rules.map;
    let f0 = m.numbered(0, piece(0).industry[0]);
    // A zone not next to f0, so only f1 counts the agents on it.
    let f1 = piece(0).industry.iter().map(|&n| m.numbered(0, n)).find(|&h| h != f0 && !m.adjacent(f0, h)).unwrap();
    // Take a factory: country 0's neutral factory with the most agents on or beside it corrupts.
    put(&mut g, f0, FACTORY, 1);
    put(&mut g, f1, FACTORY, 1);
    put(&mut g, f1, ADV_AMBASSADOR, 2);
    // 199 (D88): a firewalled neutral factory still counts as neutral: the player's one stays theirs.
    {
        use crate::modules::common::FIREWALL;
        let mut d88 = g.clone();
        put(&mut d88, f1, FIREWALL, 1);
        put(&mut d88, f0, cube(0), 1);
        run_ops(&rules, &mut d88, &[Op::TakeSite { site: Facility::Factory }]);
        assert_eq!(d88.tokens(f0, cube(0)), 1, "the guarded neutral one counts: nothing happens");
        assert!(!corrupted(&d88, f1));
    }
    run_ops(&rules, &mut g, &[Op::TakeSite { site: Facility::Factory }]);
    assert!(corrupted(&g, f1) && !corrupted(&g, f0), "the one with agents beside it");
    // A player's factory turns neutral once no neutral one is left.
    put(&mut g, f0, cube(0), 1);
    run_ops(&rules, &mut g, &[Op::TakeSite { site: Facility::Factory }]);
    assert_eq!(g.tokens(f0, cube(0)), 0, "turned neutral");
    assert!(!corrupted(&g, f0));
    // Every adversary factory musters a troop; a player ambassador on one stops it.
    put(&mut g, f0, ADV_CUBE, 1);
    put(&mut g, f0, ambassador(0), 1);
    let troops = total(&g, TROOP);
    run_ops(&rules, &mut g, &[Op::FactoryTroops { count: 1 }]);
    assert_eq!((g.tokens(f1, TROOP), total(&g, TROOP)), (1, troops + 1));
    // 231, Night Raid: an agent onto each of `count` node hexes, the one holding the most agents first. 275: an
    // arrival that brings a node hex to 2 destroys the node (its agents go to the supply).
    let (a, b) = (m.numbered(1, piece(1).node), m.numbered(1, piece(1).industry[0]));
    put(&mut g, a, node(1), 1);
    put(&mut g, b, node(1), 1);
    put(&mut g, b, ADV_AMBASSADOR, 1);
    let on_a = g.tokens(a, ADV_AMBASSADOR);
    assert_eq!(on_a, 0);
    run_ops(&rules, &mut g, &[Op::AgentsOnNodes { count: 1 }]);
    assert_eq!((g.tokens(a, ADV_AMBASSADOR), g.tokens(b, ADV_AMBASSADOR), g.tokens(b, node(1))), (0, 0, 0), "most agents first: the 2nd takes the node");
    run_ops(&rules, &mut g, &[Op::AgentsOnNodes { count: 2 }]);
    assert_eq!((g.tokens(a, ADV_AMBASSADOR), g.tokens(a, node(1))), (1, 1), "the one node hex left in country 1");
    // 231: the hex holding the most player ambassadors in each country sends them back to their supplies.
    put(&mut g, a, ambassador(0), 2);
    put(&mut g, a, ambassador(1), 1);
    let (s0, s1) = (g.resource(0, Resource::Ambassadors), g.resource(1, Resource::Ambassadors));
    run_ops(&rules, &mut g, &[Op::RecallAmbassadors]);
    assert_eq!((g.tokens(a, ambassador(0)), g.tokens(a, ambassador(1))), (0, 0));
    assert!(g.resource(0, Resource::Ambassadors) >= s0 + 2 && g.resource(1, Resource::Ambassadors) > s1);
    // 231: an open megaproject loses player cubes, the most-held player's first.
    set_var(&mut g, crate::modules::board::project_key(1, Some(0)), 3);
    set_var(&mut g, crate::modules::board::project_key(1, Some(1)), 1);
    run_ops(&rules, &mut g, &[Op::StripCubes { count: 2 }]);
    { use crate::modules::board::project_key as pk; assert_eq!((g.var(&pk(1, Some(0))), g.var(&pk(1, Some(1)))), (1, 1)); }
    // A botnet becomes troops; a botnet spreads to a site next to one.
    let st = m.numbered(1, piece(1).neutral_station);
    put(&mut g, st, BOTNET, 1);
    run_ops(&rules, &mut g, &[Op::BotnetTroops { count: 1, troops: 2 }]);
    assert_eq!((g.tokens(st, BOTNET), g.tokens(st, TROOP)), (0, 2));
    put(&mut g, st, STATION, 1);
    let botnets = total(&g, BOTNET);
    run_ops(&rules, &mut g, &[Op::SpreadBotnets { count: 1 }]);
    // Country 1 has no botnet left: its lowest-numbered station takes one; country 0 has no station: none.
    assert_eq!((g.tokens(st, BOTNET), total(&g, BOTNET)), (1, botnets + 1));
    // A player's hardened facility loses its hardening; a node shield on a hex with no facility is no target.
    let pf = m.numbered(1, piece(1).industry[1]);
    put(&mut g, pf, FACTORY, 1);
    put(&mut g, pf, cube(1), 1);
    put(&mut g, pf, HARDEN, 2);
    put(&mut g, a, HARDEN, 1);
    put(&mut g, a, ADV_AMBASSADOR, 3);
    run_ops(&rules, &mut g, &[Op::StripHarden]);
    assert_eq!((g.tokens(pf, HARDEN), g.tokens(a, HARDEN)), (0, 1));
    // The clock discards the top event card.
    let deck = g.pile(adversary::DECK).len();
    let next = g.pile(adversary::DECK)[..2].to_vec();
    run_ops(&rules, &mut g, &[Op::Clock { count: 1 }]);
    assert_eq!(g.pile(adversary::DECK).len(), deck.saturating_sub(1));
    // 197: each discard goes on the discard's top, as a flip's does (the viewer's "last:" reads the top).
    run_ops(&rules, &mut g, &[Op::Clock { count: 1 }]);
    assert_eq!(g.pile(adversary::DISCARD).first(), next.get(1));
}

/// 233: what `after` changed from `before`: per country (its hexes' tokens, its seat's resources, and the state
/// keys and piles whose first number is its own: `board.project.1.adversary`) and for the table (every other key
/// and pile).
fn footprint(w: &module::World, before: &GameState, after: &GameState) -> (Vec<i64>, i64) {
    let n = w.map.countries.len();
    let mut by = vec![0; n];
    let mut table = 0;
    for (h, cell) in w.map.hexes.iter().enumerate() {
        let (a, b) = (&before.hexes[h].tokens, &after.hexes[h].tokens);
        let d: i64 = a.keys().chain(b.keys()).map(|k| (a.get(k).copied().unwrap_or(0) - b.get(k).copied().unwrap_or(0)).abs()).sum();
        by[cell.country as usize] += d;
    }
    for (s, (a, b)) in before.seats.iter().zip(&after.seats).enumerate() {
        by[s] += a.resources.keys().chain(b.resources.keys())
            .map(|r| (a.resources.get(r).copied().unwrap_or(0) - b.resources.get(r).copied().unwrap_or(0)).abs())
            .sum::<i64>();
    }
    let owner = |k: &str| k.split('.').find_map(|t| t.parse::<usize>().ok()).filter(|&c| c < n);
    let (a, b) = (&before.vars, &after.vars);
    for k in a.keys().chain(b.keys()) {
        let d = (a.get(k).copied().unwrap_or(0) - b.get(k).copied().unwrap_or(0)).abs();
        match owner(k.as_str()) {
            Some(c) => by[c] += d,
            None => table += d,
        }
    }
    let (a, b) = (&before.piles, &after.piles);
    for k in a.keys().chain(b.keys()) {
        if a.get(k) != b.get(k) {
            match owner(k.as_str()) {
                Some(c) => by[c] += 1,
                None => table += 1,
            }
        }
    }
    (by, table)
}

/// 233: every op the project deck prints reaches as [`Op::team_reach`] says (231, Matt: a project's effect lands in
/// each country), measured on the engine: resolved once as a project resolves it, on a 2-country board where each
/// op has a target in each country, a per-country op changes both countries, a table-wide op neither. The worth's
/// twin is aic-sim's `team_ops_price_per_country` (231's IncreaseIncome miss).
#[test]
fn project_ops_reach_as_declared() {
    use crate::modules::common::corrupted;
    use crate::op::{FactoryWhere, Op, SiteType, TeamReach};
    use crate::modules::effects;
    let team = |rules: &Rules, g: &mut GameState, ops: &[Op]| {
        with_answers(rules, g, |_| 0, |g, cx| effects::run(g, cx, ops, &mut effects::Ctx::team()));
    };
    // Boards 1 and 2: both print a research station (board 0 prints none, so a station op has nothing there).
    // 263: botnets spread from `botnet_spread_at` (1) on; the baseline plays d0.
    let sets = ["ai.boards=1,2", "adversary.botnet_spread_at=0"];
    let rules = rules_of(&baseline_with(&sets), "baseline_with", &sets);
    let w = rules.world();
    let m = &w.map;
    assert_eq!(m.countries.len(), 2);
    let mut rich = rules.new_game(1);
    answer_all(&rules, &mut rich);
    // A target for every op in each country: agent stacks by every site, a corrupted city, an adversary factory,
    // then fresh stacks (something left to corrupt), and a seat's hardened factory with its ambassadors on it and
    // its cubes on the country's megaproject.
    // 275: an Infiltrate's 1 a site corrupts nothing at d0. Each country's highest-numbered neutral city corrupts
    // (an inland one first), so a city, coastal if the country has one, stays neutral for the Surge lines.
    let infiltrate = [SiteType::City, SiteType::Station, SiteType::Factory].map(|site| Op::Infiltrate { site });
    team(&rules, &mut rich, &infiltrate);
    for c in 0..m.countries.len() {
        let neutral = cities(m).into_iter().filter(|&h| m.hexes[h].country as usize == c && !corrupted(&rich, h));
        let h = neutral.max_by_key(|&h| (!m.hexes[h].coastal, m.hexes[h].number)).expect("a neutral city");
        with_cx(&rules, |cx| crate::modules::adversary::corrupt_city(&mut rich, cx, h)).unwrap();
    }
    team(&rules, &mut rich, &[Op::AdversaryFactory { count: 1, where_: Some(FactoryWhere::EachCountry), fallback_ambassadors: 0 }]);
    team(&rules, &mut rich, &infiltrate[..1]);
    for c in 0..m.countries.len() {
        let empty = |h: usize| rich.hexes[h].tokens.is_empty();
        let open = hexes_where(m, |h| m.hexes[h].country as usize == c && kind(m, h) == HexKind::Open && empty(h));
        let h = open[open.len() - 1];
        put(&mut rich, h, FACTORY, 1);
        put(&mut rich, h, cube(c), 1);
        put(&mut rich, h, HARDEN, 1);
        put(&mut rich, h, ambassador(c), 2);
        set_var(&mut rich, crate::modules::board::project_key(c, Some(c)), 3);
    }
    // A Fallout line lands beside each country's lowest-numbered corrupted cities: clear a hex beside the first one
    // where buildings ring it, so the line has land to take.
    for c in 0..m.countries.len() {
        let mut bad: Vec<usize> = cities(m).into_iter().filter(|&h| m.hexes[h].country as usize == c && corrupted(&rich, h)).collect();
        bad.sort_by_key(|&h| m.hexes[h].number);
        let first = bad[0];
        let free = |g: &GameState, x: usize| crate::modules::adversary::fallout_free(&w, g, x);
        if !m.hexes[first].neighbors.iter().any(|&x| free(&rich, x)) {
            let no_node = |x: usize| crate::modules::common::node_owner(&rich, x).is_none();
            let x = m.hexes[first].neighbors.iter().copied().find(|&x| kind(m, x) == HexKind::Open && no_node(x)).expect("open land beside it");
            Arc::make_mut(&mut Arc::make_mut(&mut rich.hexes)[x]).tokens.clear();
            assert!(free(&rich, x));
        }
    }
    // An income rise is one number: compute is kept times the countries (`rally::increase_income`).
    {
        use crate::modules::adversary::INCOME_COMPUTE;
        let mut g = rich.clone();
        team(&rules, &mut g, &[Op::IncreaseIncome { compute: 1 }]);
        assert_eq!(g.var(INCOME_COMPUTE) - rich.var(INCOME_COMPUTE), m.countries.len() as i64);
    }
    // 259: the research track is one number too: a project's research line puts its amount a country on it.
    {
        use crate::modules::adversary::RESEARCH;
        let mut g = rich.clone();
        team(&rules, &mut g, &[Op::AdversaryResearch { amount: 2, each_country: true }]);
        assert_eq!(g.var(RESEARCH) - rich.var(RESEARCH), 2 * m.countries.len() as i64);
    }
    // Not staged here: a Unity pull needs a Unity tip on the track, a token loss cards in play holding tokens; Fallout
    // by a corrupted station, because board 1's lowest-numbered one (hex 3) has no free land beside it (the
    // Fallout arm's city sites are measured). Each reads its seats, the track or its countries as the rest do.
    let staged_elsewhere = |e: &Op| {
        matches!(e, Op::PullUnity { .. } | Op::LoseTokens { .. } | Op::IncreaseIncome { .. } | Op::AdversaryResearch { .. })
            || matches!(e, Op::Fallout { at: crate::op::FalloutAt::CorruptedStation, .. })
    };
    let mut seen = Vec::new();
    let mut wrong = Vec::new();
    // 237: an alignment card's tiered deal (Incursion, Two Fronts: `{count 1, tier 2}`) reaches every country too.
    let mut lists: Vec<(&str, Vec<Op>)> = crate::content().projects.iter().map(|p| (p.id.as_str(), p.effects.clone())).collect();
    lists.push(("alignment", vec![Op::DealThreats { count: 1, tier: Some(2), scope: None, each_country: false }]));
    for (id, effects) in &lists {
        for (i, e) in effects.iter().enumerate() {
            let Some(reach) = e.team_reach() else { continue };
            if seen.contains(e) || staged_elsewhere(e) {
                continue;
            }
            seen.push(e.clone());
            // After the card's earlier effects, as the project resolves (Station Coup's Fallout lands by the station
            // it just corrupted).
            let mut before = rich.clone();
            team(&rules, &mut before, &effects[..i]);
            let mut g = before.clone();
            team(&rules, &mut g, std::slice::from_ref(e));
            let (by, table) = footprint(&w, &before, &g);
            let measured = if by.iter().all(|&d| d > 0) {
                Some(TeamReach::EachCountry)
            } else if by.iter().all(|&d| d == 0) && table > 0 {
                Some(TeamReach::Once)
            } else {
                None
            };
            if measured != Some(reach) {
                wrong.push(format!("{id} {e:?}: declared {reach:?}; changed {by:?} by country, {table} for the table"));
            }
        }
    }
    assert!(wrong.is_empty(), "project ops off their declared reach:\n{}", wrong.join("\n"));
}

