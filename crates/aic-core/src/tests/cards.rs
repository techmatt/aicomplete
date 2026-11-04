use super::*;
use crate::content::{CardDef, Op, Tag, TAGS};
use crate::modules::{adversary, cards, effects, threats};

#[test]
fn draft_picks_cannot_be_passed() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(2);
    // 021: round 1 is the opening (no draft); pass through its action phase.
    advance_to(&rules, &mut g, "cards.draft");
    let legal = rules.legal_actions(&g);
    assert!(
        legal.iter().all(|a| matches!(a, Action::Draft { .. })),
        "{legal:?}"
    );
    assert_eq!(legal.iter().filter(|a| a.seat() == 0).count(), 4);
    assert!(rules
        .apply_mut(&mut g, &Action::Pass { seat: 0 }, None)
        .is_err());
}

#[test]
fn every_card_is_unique_in_every_game() {
    let rules = Rules::new(&baseline()).unwrap();
    for seed in 0..4 {
        let (_, actions) = random_game(&rules, seed);
        let mut g = rules.new_game(seed);
        for a in &actions {
            rules.apply_mut(&mut g, a, None).unwrap();
            let mut seen = BTreeSet::new();
            for (k, pile) in &g.piles {
                for id in pile.iter() {
                    assert!(seen.insert(*id), "{id} twice (in {k})");
                }
            }
        }
    }
}

#[test]
fn off_variants_remove_threats_alignment_and_the_draft() {
    let rules = Rules::new(&baseline_with(&["threats.variant=off"])).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    assert!(threats::active(&g).is_empty());

    let rules = Rules::new(&baseline_with(&["alignment.variant=off"])).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    assert_eq!(
        (g.var("alignment.unity"), g.var("alignment.discord")),
        (0, 0)
    );
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Antimatter { .. })));
    g.vars.insert("alignment.unity".into(), 4);
    give(&rules, &mut g);
    assert_eq!(g.var("alignment.unity_tips"), 0);

    let rules = Rules::new(&baseline_with(&["cards.variant=off"])).unwrap();
    let mut g = rules.new_game(4);
    advance_to(&rules, &mut g, ACTIONS);
    // Nothing dealt or drafted: no player cards at all.
    assert!(g.pile("cards.hand.0").is_empty());
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Tier { .. } | Action::Draft { .. })));
}

#[test]
fn playing_a_card_pays_its_insight_and_lowers_its_income() {
    // 022: insight to play, shared with keeps; `lowers` cuts an income for good.
    let (rules, mut g) = lab_actions_with(&["cards.darknet_exchange.cost=2", "cards.darknet_exchange.insight=1"]);
    for c in ["darknet_exchange", "recursive_self_improvement"] {
        out_of_decks(&mut g, c);
    }
    set_pile(&mut g, "cards.hand.1", &["darknet_exchange"]);
    set_res(&mut g, 1, Resource::Credits, 20);
    let playable = |g: &GameState| {
        rules
            .legal_actions(g)
            .iter()
            .any(|a| matches!(a, Action::Play { seat: 1, card } if card == "darknet_exchange"))
    };
    set_res(&mut g, 1, Resource::Insight, 0);
    assert!(!playable(&g), "Darknet Exchange costs 1 insight");
    set_res(&mut g, 1, Resource::Insight, 2);
    assert!(playable(&g));
    rules
        .apply_mut(&mut g, &Action::Play { seat: 1, card: "darknet_exchange".into() }, None)
        .unwrap();
    assert_eq!(g.resource(1, Resource::Insight), 1);
    // Endowed Research Chair lowers credit income by 1, through the production key.
    let w = rules.world();
    let d = cards::def(&w, "endowed_research_chair");
    assert_eq!(d.lowers, vec![(Resource::Credits, 1)]);
    let before = crate::modules::economy::income(&w, &g, 1)[Resource::Credits];
    let mut c = cards::def(&w, "endowed_research_chair").clone();
    c.lowers = vec![(Resource::Credits, before + 1)];
    assert!(!cards::lowers_ok(&w, &g, 1, &c), "can't lower an income below 0");
    c.lowers = vec![(Resource::Credits, 1)];
    assert!(cards::lowers_ok(&w, &g, 1, &c));
}

#[test]
fn selling_a_card_discards_it_for_a_credit() {
    let (rules, mut g) = lab_actions_with(&["cards.darknet_exchange.cost=2", "cards.darknet_exchange.insight=1"]);
    let tier2 = "positron_storage_ring";
    for p in ["cards.deck.1", "cards.discard.1", "cards.deck.2", "cards.discard.2"] {
        if let Some(pile) = g.piles.get_mut(p) {
            Arc::make_mut(pile).retain(|c| c != "darknet_exchange" && c != tier2);
        }
    }
    for s in 0..g.seats.len() {
        if let Some(pile) = g.piles.get_mut(format!("cards.hand.{s}").as_str()) {
            Arc::make_mut(pile).retain(|c| c != tier2);
        }
    }
    set_pile(&mut g, "cards.hand.0", &[tier2, "darknet_exchange"]);
    let sells: Vec<Action> = rules
        .legal_actions(&g)
        .into_iter()
        .filter(|a| matches!(a, Action::Sell { seat: 0, .. }))
        .collect();
    assert_eq!(sells.len(), 2);
    let credits = g.resource(0, Resource::Credits);
    act(
        &rules,
        &mut g,
        Action::Sell {
            seat: 0,
            card: "darknet_exchange".into(),
        },
    );
    act(
        &rules,
        &mut g,
        Action::Sell {
            seat: 0,
            card: tier2.into(),
        },
    );
    assert_eq!(g.resource(0, Resource::Credits), credits + 2);
    assert!(g.pile("cards.hand.0").is_empty());
    // Each card to its own tier's discard pile.
    assert_eq!(g.pile("cards.discard.1")[0], "darknet_exchange");
    assert_eq!(g.pile("cards.discard.2")[0], tier2);
    assert!(rules
        .apply_mut(
            &mut g,
            &Action::Sell {
                seat: 0,
                card: "darknet_exchange".into()
            },
            None
        )
        .is_err());
}

/// 023's primitives: draw, drain every project, count tokens as power, cash
/// tokens out at a plain rate (059).
#[test]
fn the_023_primitives_draw_drain_all_count_and_cash_out() {
    let (rules, mut g) = lab_actions();
    let hand = g.pile("cards.hand.0").len();
    let deck = g.pile("cards.deck.1").len();
    run_ops(&rules, &mut g, &[Op::Draw { count: 2 }]);
    assert_eq!(g.pile("cards.hand.0").len(), hand + 2);
    assert_eq!(g.pile("cards.deck.1").len(), deck - 2);

    row(&mut g, &["false_flag", "disinformation"]);
    g.vars.insert(adversary::project_key("false_flag").into(), 6);
    g.vars.insert(adversary::project_key("disinformation").into(), 3);
    run_ops(&rules, &mut g, &[Op::DrainAll { amount: 4 }]);
    assert_eq!(g.var(&adversary::project_key("false_flag")), 2);
    assert_eq!(g.var(&adversary::project_key("disinformation")), 0);

    // Retained Earnings holds Capital.
    set_pile(&mut g, "cards.built.0", &["retained_earnings"]);
    g.vars.insert(cards::tokens_key(0, "retained_earnings").into(), 5);
    let credits = g.resource(0, Resource::Credits);
    run_ops(&rules, &mut g, &[Op::GainPerTokens { resource: Resource::Credits, kind: TokenKind::Capital, per: 2, here: false }]);
    assert_eq!(g.resource(0, Resource::Credits), credits + 2, "5 Capital count as 2; they stay");
    assert_eq!(g.var(&cards::tokens_key(0, "retained_earnings")), 5);
    run_ops(&rules, &mut g, &[Op::CashOut { kind: TokenKind::Capital, resource: Resource::Credits, rate: 3, here: false }]);
    assert_eq!(g.resource(0, Resource::Credits), credits + 2 + 15, "5 cashed out at 3");
    assert_eq!(g.var(&cards::tokens_key(0, "retained_earnings")), 0);
}

#[test]
fn a_card_that_only_hardens_or_moves_needs_a_target_but_one_that_also_gains_does_not() {
    let (rules, g) = lab(&[]);
    let w = rules.world();
    let harden = Op::Harden { count: 2 };
    let moves = Op::MoveOne { range: 2 };
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&harden)), "no building, no nodes");
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&moves)), "no ambassador");
    assert!(!effects::feasible(&w, &g, 0, &[harden.clone(), moves.clone()]));
    // Either one acting is enough: Peacekeeping Detail with an ambassador to move.
    let mut with_one = g.clone();
    let c = rules.map.numbered(0, piece(0).node);
    let far = rules.map.numbered(0, piece(0).corrupted[0]);
    put(&mut with_one, c, ambassador(0), 1);
    put(&mut with_one, far, node(1), 1);
    assert!(effects::feasible(&w, &with_one, 0, &[harden.clone(), Op::MoveOne { range: 20 }]));
    let gain = Op::Gain { resource: Resource::Credits, amount: 1 };
    assert!(effects::feasible(&w, &g, 0, &[gain, moves]), "a card that also gains is worth playing");
}

#[test]
fn a_small_removal_has_no_target_on_a_hex_holding_only_a_troop() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, node(0), 1);
    put(&mut g, a, TROOP, 1);
    let one = [Op::Damage { ambassadors: 1, troops: 0 }];
    assert!(!effects::feasible(&w, &g, 0, &one), "a card that names no troop can't take one");
    let two = [Op::Damage { ambassadors: 2, troops: 1 }];
    assert!(effects::feasible(&w, &g, 0, &two));
    put(&mut g, a, ADV_AMBASSADOR, 1);
    assert!(effects::feasible(&w, &g, 0, &one));
}

#[test]
fn a_cash_in_counts_the_tokens_the_same_card_adds_first() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let kind = TokenKind::Capital;
    let holder = w.content.cards.iter().find(|c| c.tokens.as_ref().is_some_and(|t| t.kind == kind)).unwrap();
    g.piles.insert(cards::built(0).into(), vec![holder.id.as_str().into()].into());
    let cash = Op::CashOut { kind, resource: Resource::Credits, rate: 1, here: false };
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&cash)), "nothing to cash in");
    assert!(effects::feasible(&w, &g, 0, &[Op::AddTokens { kind, amount: 2, here: false }, cash]));
}

#[test]
fn energy_a_card_gains_first_can_pay_its_step_and_a_second_delay_is_not_offered() {
    let (rules, mut g) = at_step(&[], 5);
    let w = rules.world();
    let step = step_cost(&w, 5);
    set_res(&mut g, 0, Resource::Energy, 0);
    let gains = [
        Op::Gain { resource: Resource::Energy, amount: step },
        Op::PlaceNode { count: 1, free: false },
    ];
    assert!(effects::feasible(&w, &g, 0, &gains));
    let delay = [Op::DelayThreat { count: 1 }];
    deal(&mut g, "smear_campaign", 0);
    assert!(effects::feasible(&w, &g, 0, &delay));
    run_ops(&rules, &mut g, &delay);
    assert!(!effects::feasible(&w, &g, 0, &delay), "every threat already waits");
}

#[test]
fn hand_written_card_text_is_refused() {
    let mut c = content().clone();
    for card in &mut c.cards {
        card.text.clear();
    }
    c.cards[0].text = "Gain a thousand credits.".to_string();
    assert!(c.check().unwrap_err().contains("generated"));
}

#[test]
fn a_science_niche_card_needing_science_is_refused() {
    let mut c = content().clone();
    for card in &mut c.cards {
        card.text.clear();
    }
    let niche = |d: &CardDef| d.effects.iter().any(|o| matches!(o, Op::ClearMarkers { .. }));
    let card = c.cards.iter_mut().find(|d| niche(d)).unwrap();
    card.needs = vec![(Tag::Science, 1)];
    assert!(c.check().unwrap_err().contains("needs no Science"));
}

#[test]
fn only_a_building_keeps_an_action() {
    let mut c = content().clone();
    for card in &mut c.cards {
        card.text.clear();
    }
    assert_eq!(c.check(), Ok(()));
    let bld = c.cards.iter_mut().find(|d| d.id == "sentry_drone_wing").expect("a card");
    bld.kind = crate::content::CardKind::Program;
    assert!(c.check().unwrap_err().contains("only a building"));
}

#[test]
fn science_tags_discount_insight_to_a_floor_of_one() {
    let (rules, mut g) = lab_with(&[], &["cards.science_every=2"]);
    let w = rules.world();
    let two = w.content.cards.iter().find(|c| c.insight == 2).unwrap();
    let one = w.content.cards.iter().find(|c| c.insight == 1).unwrap();
    assert_eq!(cards::play_insight(&w, &g, 0, two), 2);
    let science: Vec<_> = w.content.cards.iter().filter(|c| c.tags.iter().filter(|t| **t == Tag::Science).count() == 1 && c.tier == 1).take(4).map(|c| Key::new(&c.id)).collect();
    g.piles.insert(cards::played(0).into(), Arc::new(science));
    assert_eq!(cards::play_insight(&w, &g, 0, two), 1, "4 tags: 2 off, stopped at the floor");
    assert_eq!(cards::play_insight(&w, &g, 0, one), 1, "1 stays 1");
}

#[test]
fn industry_tags_discount_buildings_to_a_floor_of_one() {
    let (rules, mut g) = lab_with(&[], &["cards.industry_every=2"]);
    let w = rules.world();
    let d = w.content.cards.iter().find(|c| c.is_building() && c.cost >= 3 && c.tier == 1).unwrap();
    let before = cards::play_cost(&w, &g, 1, d);
    let industry: Vec<_> = w.content.cards.iter().filter(|c| c.tags.iter().filter(|t| **t == Tag::Industry).count() == 1 && c.tier == 1 && !c.is_building()).take(2).map(|c| Key::new(&c.id)).collect();
    g.piles.insert(cards::played(1).into(), Arc::new(industry));
    assert_eq!(cards::play_cost(&w, &g, 1, d), before - 1);
    let many: Vec<_> = programs(&w, Tag::Industry).map(|c| Key::new(&c.id)).collect();
    g.piles.insert(cards::played(1).into(), Arc::new(many));
    assert_eq!(cards::play_cost(&w, &g, 1, d), w.int("cards.discount_floor"), "the floor");
}

/// 042: what the heuristic reads must be what the rules charge.
#[test]
fn prices_worked_out_once_match_each_cards_own() {
    let (rules, mut g) = lab_with(&[], &["cards.science_every=2", "cards.industry_every=2"]);
    let w = rules.world();
    let tagged: Vec<_> = w.content.cards.iter().filter(|c| !c.tags.is_empty()).take(12).map(|c| Key::new(&c.id)).collect();
    for held in [0, 4, tagged.len()] {
        g.piles.insert(cards::played(0).into(), Arc::new(tagged[..held].to_vec()));
        set_res(&mut g, 0, Resource::Energy, 3);
        let p = cards::prices(&w, &g, 0);
        for d in &w.content.cards {
            assert_eq!(p.insight(d), cards::play_insight(&w, &g, 0, d), "{}", d.id);
            assert_eq!(p.credits(d), cards::play_cost(&w, &g, 0, d), "{}", d.id);
        }
        let counts = cards::tag_counts(&w, &g, 0);
        for t in TAGS {
            let by_hand: usize = cards::in_play(&g, 0).iter().map(|id| cards::def(&w, id).tags.iter().filter(|x| **x == t).count()).sum();
            assert_eq!(counts[t as usize], by_hand as i64);
        }
    }
}

/// 076 (ruling 12): a progressive token card adds a token here, or counts only
/// its own tokens; with nothing here only the add is offered, so nothing asks.
#[test]
fn an_or_action_adds_here_then_offers_both_halves_and_counts_only_its_own_tokens() {
    let (rules, mut g) = lab_actions();
    let id = "agency_field_station";
    set_pile(&mut g, "cards.built.0", &[id, "kill_team_roster"]);
    g.vars.insert(cards::tokens_key(0, "kill_team_roster").into(), 5);
    act(&rules, &mut g, Action::Use { seat: 0, card: id.into() });
    assert_eq!(g.var(&cards::tokens_key(0, id)), 1, "the add, asked nothing");
    g.seats[0].exhausted.clear();
    let credits = g.resource(0, Resource::Credits);
    rules.apply_mut(&mut g, &Action::Use { seat: 0, card: id.into() }, None).unwrap();
    assert_eq!(g.pending.as_ref().expect("which half").ask.options.len(), 2);
    answer_with(&rules, &mut g, "GainPerTokens");
    assert_eq!(g.resource(0, Resource::Credits), credits + 1, "1 Agent here; the other card's 5 don't count");
    assert_eq!(g.var(&cards::tokens_key(0, id)), 1, "they stay");
}

/// 076 (ruling 6): a printed troop number in one country, no "per hex".
#[test]
fn remove_troops_takes_up_to_its_number_in_one_country() {
    let (rules, mut g) = lab(&[]);
    let hexes: Vec<usize> = (1..=3).map(|n| rules.map.numbered(0, n)).collect();
    for &h in &hexes {
        put(&mut g, h, TROOP, 1);
    }
    run_ops(&rules, &mut g, &[Op::RemoveTroops { count: 6 }]);
    assert_eq!(total(&g, TROOP), 0, "3 troops, all of them: nothing to ask");
}

/// 076 (ruling 11): a one-off per tag on your cards.
#[test]
fn gain_per_tag_counts_the_tag_on_your_cards() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let per_tag = [Op::GainPerTag { resource: Resource::Energy, tag: Tag::Science, per: 1 }];
    assert!(!effects::feasible(&w, &g, 0, &per_tag), "no Science on your cards: it would gain nothing");
    let science: Vec<_> = programs(&w, Tag::Science).filter(|c| c.tags.len() == 1).take(3).map(|c| Key::new(&c.id)).collect();
    assert_eq!(science.len(), 3);
    g.piles.insert(cards::played(0).into(), Arc::new(science));
    assert!(effects::feasible(&w, &g, 0, &per_tag));
    let energy = g.resource(0, Resource::Energy);
    run_ops(&rules, &mut g, &per_tag);
    assert_eq!(g.resource(0, Resource::Energy), energy + 3);
}

/// 099: the energy gift Actions need another player (at one country they took
/// the energy and gave nothing), and a draw needs a tier-1 card to draw.
#[test]
fn a_gift_needs_another_player_and_a_draw_a_card() {
    let gift = [Op::ChoosePlayer, Op::OtherGain { resource: Resource::Chips, amount: 5 }];
    let (rules, g) = lab_actions();
    assert!(effects::feasible(&rules.world(), &g, 0, &gift));
    let (rules, g) = lab_actions_with(&["countries=1"]);
    assert!(!effects::feasible(&rules.world(), &g, 0, &gift), "no one to give to");
    let (rules, mut g) = lab_actions();
    let w = rules.world();
    let draw = [Op::Draw { count: 2 }];
    assert!(effects::feasible(&w, &g, 0, &draw));
    set_pile(&mut g, "cards.deck.1", &[]);
    set_pile(&mut g, "cards.discard.1", &[]);
    assert!(!effects::feasible(&w, &g, 0, &draw), "both tier-1 piles empty");
    let draw_and_more = [Op::Draw { count: 2 }, Op::Gain { resource: Resource::Credits, amount: 1 }];
    assert!(effects::feasible(&w, &g, 0, &draw_and_more), "a card that also gains still plays");
}
