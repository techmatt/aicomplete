//! 105: the engine wording (triggers, discounts, rebate, copy, reuse,
//! threshold) and the insight economy (the cube converter, the corps'
//! converters, the boards' insight powers, Vigil, insight counters).

use super::*;
use crate::content::{Op, Tag};
use crate::modules::{board, cards, economy};

/// The lab at its action phase with these cards out of the decks, seat 0 (the
/// Builder) holding `hand` and `built`, credits and insight to spare.
fn table(hand: &[&str], built: &[&str]) -> (Rules, GameState) {
    let (rules, mut g) = lab_actions();
    for c in hand.iter().chain(built) {
        out_of_decks(&mut g, c);
    }
    set_pile(&mut g, "cards.hand.0", hand);
    set_pile(&mut g, "cards.built.0", built);
    for s in 0..2 {
        set_res(&mut g, s, Resource::Credits, 60);
        set_res(&mut g, s, Resource::Insight, 10);
    }
    (rules, g)
}

fn play(rules: &Rules, g: &mut GameState, seat: u8, card: &str) {
    act(rules, g, Action::Play { seat, card: card.into() });
}

/// What `card`'s first trigger gains, as printed.
fn trigger_gain(rules: &Rules, card: &str) -> (Resource, i64) {
    let w = rules.world();
    for p in &cards::def(&w, card).passives {
        if let Op::When { then, .. } = p {
            if let Some(Op::Gain { resource, amount }) = then.first() {
                return (*resource, *amount);
            }
        }
    }
    panic!("{card} prints no gain trigger");
}

#[test]
fn a_play_trigger_fires_for_its_tag_and_counts_itself() {
    // Citation Engine: a Science card played pays; a Cyber one doesn't.
    let (rules, mut g) = table(&["literature_crawler", "darknet_exchange"], &["citation_engine"]);
    let (r, n) = trigger_gain(&rules, "citation_engine");
    let before = g.resource(0, r);
    let w = rules.world();
    let cost = cards::play_cost(&w, &g, 0, cards::def(&w, "literature_crawler"));
    play(&rules, &mut g, 0, "literature_crawler");
    assert_eq!(g.resource(0, r), before - cost + n);
    let before = g.resource(0, r);
    let cost = cards::play_cost(&w, &g, 0, cards::def(&w, "darknet_exchange"));
    play(&rules, &mut g, 0, "darknet_exchange");
    assert_eq!(g.resource(0, r), before - cost, "no Science tag, no trigger");
    // Telemetry Archive is a Science card: playing it fires its own trigger.
    let (rules, mut g) = table(&["telemetry_archive"], &[]);
    play(&rules, &mut g, 0, "telemetry_archive");
    assert_eq!(g.var(&cards::tokens_key(0, "telemetry_archive")), 1);
}

#[test]
fn the_per_tag_form_counts_the_played_cards_tags() {
    let (rules, mut g) = table(&["simulation_cluster"], &["quantum_decryptor"]);
    let w = rules.world();
    let d = cards::def(&w, "simulation_cluster");
    let science = d.tags.iter().filter(|t| **t == Tag::Science).count() as i64;
    let (r, n) = trigger_gain(&rules, "quantum_decryptor");
    assert_eq!(r, Resource::Insight);
    let before = g.resource(0, Resource::Insight);
    let paid = cards::play_insight(&w, &g, 0, d);
    play(&rules, &mut g, 0, "simulation_cluster");
    assert_eq!(g.resource(0, Resource::Insight), before - paid + n * science);
}

#[test]
fn a_teammate_trigger_pays_on_another_players_play() {
    // Grid-Tied Smelter on seat 1: seat 0 plays a Power card, seat 1 gains.
    let (rules, mut g) = table(&["baseload_contract"], &[]);
    out_of_decks(&mut g, "grid_tied_smelter");
    set_pile(&mut g, "cards.built.1", &["grid_tied_smelter"]);
    let (r, n) = trigger_gain(&rules, "grid_tied_smelter");
    let before = g.resource(1, r);
    play(&rules, &mut g, 0, "baseload_contract");
    assert_eq!(g.resource(1, r), before + n);
    // Consulate Tower: both gain.
    let (rules, mut g) = table(&["consular_pouch"], &[]);
    out_of_decks(&mut g, "consulate_tower");
    set_pile(&mut g, "cards.built.1", &["consulate_tower"]);
    let (r, n) = trigger_gain(&rules, "consulate_tower");
    let w = rules.world();
    let d = cards::def(&w, "consular_pouch");
    assert!(d.tags.contains(&Tag::Diplomacy));
    let (mine, theirs) = (g.resource(0, r), g.resource(1, r));
    let cost = if r == Resource::Credits { cards::play_cost(&w, &g, 0, d) } else { 0 };
    play(&rules, &mut g, 0, "consular_pouch");
    assert_eq!(g.resource(1, r), theirs + n);
    assert_eq!(g.resource(0, r), mine - cost + n);
}

#[test]
fn a_rebate_reads_the_printed_cost() {
    let (rules, mut g) = table(&["baseload_contract", "literature_crawler"], &["lights_out_assembly"]);
    let w = rules.world();
    let (r, n) = trigger_gain(&rules, "lights_out_assembly");
    let at = cards::def(&w, "lights_out_assembly").passives.iter().find_map(|p| match p {
        Op::When { on: crate::content::Event::PlayCost { at }, .. } => Some(*at),
        _ => None,
    });
    let at = at.unwrap();
    for (card, pays) in [("baseload_contract", true), ("literature_crawler", false)] {
        let d = cards::def(&w, card);
        assert_eq!(d.cost >= at, pays, "{card} costs {}", d.cost);
        let before = g.resource(0, r);
        let cost = cards::play_cost(&w, &g, 0, d);
        play(&rules, &mut g, 0, card);
        assert_eq!(g.resource(0, r), before - cost + if pays { n } else { 0 }, "{card}");
    }
}

#[test]
fn discounts_by_tag_take_credits_or_insight_off_down_to_the_floor() {
    let (rules, g) = table(&[], &["darknet_exchange"]);
    let w = rules.world();
    let floor = w.int("cards.discount_floor");
    let cyber = cards::def(&w, "signal_hijack");
    let science = cards::def(&w, "literature_crawler");
    assert_eq!(cards::play_cost(&w, &g, 0, cyber), (cyber.cost - 2).max(cyber.cost.min(floor)));
    assert_eq!(cards::play_cost(&w, &g, 0, science), science.cost);
    let (rules, mut g) = table(&["planetary_referendum"], &["hyperscale_botnet"]);
    let w = rules.world();
    let d = cards::def(&w, "simulation_cluster");
    assert_eq!(cards::play_insight(&w, &g, 0, d), (d.insight - 1).max(d.insight.min(floor)));
    // Played: a 2-insight card pays 1, and the measure counts the insight saved.
    let d = cards::def(&w, "planetary_referendum");
    assert_eq!(d.insight, 2);
    let before = g.resource(0, Resource::Insight);
    play(&rules, &mut g, 0, "planetary_referendum");
    assert_eq!(g.resource(0, Resource::Insight), before - 1);
    assert_eq!(g.stats.counters["cards.discount_insight"], 1);
}

#[test]
fn a_threshold_cashes_the_tokens_its_trigger_leaves() {
    let (rules, mut g) = table(&["literature_crawler"], &["telemetry_archive"]);
    g.vars.insert(cards::tokens_key(0, "telemetry_archive").as_str().into(), 2);
    let w = rules.world();
    let reward = cards::def(&w, "telemetry_archive").passives.iter().find_map(|p| match p {
        Op::Threshold { then, .. } => match then.first() {
            Some(Op::Gain { amount, .. }) => Some(*amount),
            _ => None,
        },
        _ => None,
    });
    let before = g.resource(0, Resource::Insight);
    let paid = cards::play_insight(&w, &g, 0, cards::def(&w, "literature_crawler"));
    play(&rules, &mut g, 0, "literature_crawler");
    assert_eq!(g.var(&cards::tokens_key(0, "telemetry_archive")), 0, "3 reached: removed");
    assert_eq!(g.resource(0, Resource::Insight), before - paid + reward.unwrap());
}

#[test]
fn copy_raises_income_by_a_buildings_output() {
    let (rules, mut g) = table(&["payroll_automation"], &["robotic_foundry"]);
    let w = rules.world();
    let before = economy::income(&w, &g, 0)[Resource::Credits];
    let foundry: i64 = cards::def(&w, "robotic_foundry")
        .passives
        .iter()
        .map(|p| match p {
            Op::Income { resource: Resource::Credits, amount } => *amount,
            _ => 0,
        })
        .sum();
    play(&rules, &mut g, 0, "payroll_automation");
    assert_eq!(economy::income(&w, &g, 0)[Resource::Credits], before + foundry);
    // With no building paying an income, there is nothing to copy.
    let (rules, g) = table(&["payroll_automation"], &[]);
    assert!(!rules.legal_actions(&g).iter().any(|a| matches!(a, Action::Play { seat: 0, card } if card == "payroll_automation")));
}

#[test]
fn reuse_uses_another_action_again_but_never_a_reuse() {
    let (rules, mut g) = table(&[], &["simulation_cluster", "sentry_drone_wing"]);
    let uses = |g: &GameState, card: &str| rules.legal_actions(g).iter().any(|a| matches!(a, Action::Use { seat: 0, card: c } if c == card));
    assert!(!uses(&g, "simulation_cluster"), "nothing used yet: nothing to reuse");
    act(&rules, &mut g, Action::Use { seat: 0, card: "sentry_drone_wing".into() });
    assert_eq!(g.var(&cards::tokens_key(0, "sentry_drone_wing")), 1);
    assert!(uses(&g, "simulation_cluster"));
    act(&rules, &mut g, Action::Use { seat: 0, card: "simulation_cluster".into() });
    assert_eq!(g.var(&cards::tokens_key(0, "sentry_drone_wing")), 2, "the Action ran again");
    assert!(!uses(&g, "sentry_drone_wing") && !uses(&g, "simulation_cluster"));
    let w = rules.world();
    assert!(!cards::reusable(&w, &g, 0, None).iter().any(|c| c == "simulation_cluster"), "a reuse building is never reused");
}

#[test]
fn fight_triggers_fire_once_an_action_or_once_a_troop() {
    let (rules, mut g) = table(&["signal_hijack"], &["security_contractor_hq", "honeypot_relay", "security_ops_floor"]);
    let m = &rules.map;
    let home = m.numbered(0, piece(0).node);
    let near = m.hexes[home].neighbors.iter().copied().find(|&h| kind(m, h) != HexKind::Ocean).unwrap();
    put(&mut g, near, TROOP, 2);
    let (r, n) = trigger_gain(&rules, "security_contractor_hq");
    let w = rules.world();
    let before = g.resource(0, r);
    let cost = cards::play_cost(&w, &g, 0, cards::def(&w, "signal_hijack"));
    play(&rules, &mut g, 0, "signal_hijack");
    assert_eq!(g.tokens(near, TROOP), 0);
    assert_eq!(g.resource(0, r), before - cost + 2 * n, "one firing a troop");
    // Ambassadors: once for the action, however many.
    let (r, n) = trigger_gain(&rules, "honeypot_relay");
    put(&mut g, near, ADV_AMBASSADOR, 3);
    set_res(&mut g, 0, Resource::Ambassadors, 1);
    let before = g.resource(0, r);
    act(&rules, &mut g, std_act(0, "remove"));
    answer_all(&rules, &mut g);
    assert!(g.tokens(near, ADV_AMBASSADOR) < 3);
    assert_eq!(g.resource(0, r), before + n);
    // Compute: the drain project.
    let (r, n) = trigger_gain(&rules, "security_ops_floor");
    let p = crate::modules::adversary::projects(&g)[0];
    g.vars.insert(crate::modules::adversary::project_key(&p).as_str().into(), 4);
    let before = g.resource(0, r);
    act(&rules, &mut g, std_act(0, "drain"));
    assert_eq!(g.resource(0, r), before + n);
}

#[test]
fn placing_a_dataset_pays_data_licensing_deal() {
    let (rules, mut g) = table(&[], &["data_licensing_deal", "grad_student_pool"]);
    let (r, n) = trigger_gain(&rules, "data_licensing_deal");
    let before = g.resource(0, r);
    rules.apply_mut(&mut g, &Action::Use { seat: 0, card: "grad_student_pool".into() }, None).unwrap();
    if g.pending.is_some() {
        answer_with(&rules, &mut g, "add tokens here");
    }
    assert_eq!(g.var(&cards::tokens_key(0, "grad_student_pool")), 1);
    assert_eq!(g.resource(0, r), before + n);
}

#[test]
fn the_cube_converter_has_no_limit_a_round() {
    let (rules, mut g) = table(&[], &[]);
    set_res(&mut g, 0, Resource::ResearchCubes, 2);
    let w = rules.world();
    let each = w.int("board.cube_insight");
    let before = g.resource(0, Resource::Insight);
    act(&rules, &mut g, std_act(0, "insight"));
    act(&rules, &mut g, std_act(0, "insight"));
    assert_eq!(g.resource(0, Resource::Insight), before + 2 * each);
    assert_eq!(g.resource(0, Resource::ResearchCubes), 0);
    assert!(!rules.legal_actions(&g).contains(&std_act(0, "insight")), "no cube, no action");
}

#[test]
fn only_onyxs_converter_keeps_once_a_round() {
    let (rules, mut g) = table(&[], &[]);
    let w = rules.world();
    let m = &rules.map;
    for (c, piece) in w.content.map.countries.iter().enumerate().take(w.map.countries.len()) {
        g.vars.insert(board::project_done(c).as_str().into(), 1);
        // A node of seat 0 in a city of the country.
        let city = m.numbered(c, piece.cities[0]);
        put(&mut g, city, node(0), 1);
        assert_eq!(piece.megaproject.converter.once, piece.name == "Onyx", "{}", piece.name);
    }
    set_res(&mut g, 0, Resource::ResearchCubes, 10);
    set_res(&mut g, 0, Resource::Energy, 10);
    for (c, piece) in w.content.map.countries.iter().enumerate().take(w.map.countries.len()) {
        assert!(board::converter_blocked(&w, &g, 0, c).is_none(), "{}", piece.name);
        act(&rules, &mut g, Action::Convert { seat: 0, country: c as u8 });
        let again = board::converter_blocked(&w, &g, 0, c).is_none();
        assert_eq!(again, !piece.megaproject.converter.once, "{}", piece.name);
    }
}

#[test]
fn the_boards_insight_powers() {
    // Builder: Focus, credits for insight.
    let (rules, mut g) = table(&[], &[]);
    let before = (g.resource(0, Resource::Credits), g.resource(0, Resource::Insight));
    act(&rules, &mut g, power(&rules, 0, "Focus"));
    assert_eq!((g.resource(0, Resource::Credits), g.resource(0, Resource::Insight)), (before.0 - 5, before.1 + 1));
    // Researcher: Synthesis, cards from hand for insight.
    let mut c = lab_config(&[ACTIONS]);
    c.apply_override("ai.boards=2,1").unwrap();
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(1);
    for card in ["signal_hijack", "baseload_contract"] {
        out_of_decks(&mut g, card);
    }
    set_pile(&mut g, "cards.hand.0", &["signal_hijack", "baseload_contract"]);
    set_res(&mut g, 0, Resource::Insight, 0);
    rules.apply_mut(&mut g, &power(&rules, 0, "Synthesis"), None).unwrap();
    answer(&rules, &mut g, 0);
    assert_eq!(g.pile("cards.hand.0").len(), 1, "one card a use");
    assert_eq!(g.resource(0, Resource::Insight), 2);
}

#[test]
fn vigil_pays_when_the_wall_stops_a_troop() {
    let (rules, mut g) = lab_with(&["board.march"], &["board.troop_node=0"]);
    let m = &rules.map;
    let p = m.numbered(0, piece(0).node);
    put(&mut g, p, node(1), 1);
    let t = m.hexes[p].neighbors.iter().copied().find(|&h| kind(m, h) != HexKind::Ocean).unwrap();
    put(&mut g, t, TROOP, 1);
    set_res(&mut g, 1, Resource::Insight, 0);
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(p, node(1)), 1, "the Wall held");
    assert_eq!(g.resource(1, Resource::Insight), 1, "Vigil: 1 insight for the troop stopped");
    assert_eq!(g.stats.counters["board.wall_stops"], 1);
}

#[test]
fn some_threats_take_insight() {
    let ins: Vec<&str> = crate::content()
        .threats
        .iter()
        .filter(|t| t.counter.iter().any(|(r, _)| *r == Resource::Insight))
        .map(|t| t.id.as_str())
        .collect();
    assert!((3..=4).contains(&ins.len()), "{ins:?}");
}
