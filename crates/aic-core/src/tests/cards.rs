use super::alignment::give;
use super::projects::row;
use super::*;
use crate::content::Op;
use crate::modules::{adversary, effects, threats};

#[test]
fn the_ais_spend_their_resource_as_credits_on_cards() {
    let (rules, mut g) = lab_actions();
    for c in ["darknet_exchange", "literature_crawler"] {
        Arc::make_mut(g.piles.get_mut("cards.deck.1").unwrap()).retain(|x| x != c);
    }
    // The Builder (seat 0): energy on buildings, 1 credit each, after
    // Economies of scale takes 1 off.
    g.piles.insert(
        "cards.hand.0".into(),
        vec!["darknet_exchange".into(), "literature_crawler".into()].into(),
    );
    set_res(&mut g, 0, Resource::Credits, 3);
    set_res(&mut g, 0, Resource::Energy, 10);
    set_res(&mut g, 0, Resource::Insight, 1);
    let plays = |g: &GameState, seat: u8| -> Vec<Key> {
        rules
            .legal_actions(g)
            .into_iter()
            .filter_map(|a| match a {
                Action::Play { seat: s, card } if s == seat => Some(card),
                _ => None,
            })
            .collect()
    };
    // Not on a program: with no credits only the building is playable.
    set_res(&mut g, 0, Resource::Credits, 0);
    assert_eq!(plays(&g, 0), ["darknet_exchange"]);
    // 021: Darknet Exchange (023; was Fab Line) costs 2, 1 after Economies of scale (the floor, 055);
    // with 1 credit energy is optional.
    set_res(&mut g, 0, Resource::Credits, 1);
    let energy = g.resource(0, Resource::Energy);
    rules
        .apply_mut(
            &mut g,
            &Action::Play {
                seat: 0,
                card: "darknet_exchange".into(),
            },
            None,
        )
        .unwrap();
    let p = g.pending.as_ref().expect("a discount choice");
    let opts: Vec<&str> = p.ask.options.iter().map(|o| o.option.as_str()).collect();
    assert_eq!(opts, ["spend 0 energy", "spend 1 energy"]);
    answer_with(&rules, &mut g, "spend 1 energy");
    // 021: a new slot, or over the Builder's starting Foundry Core.
    answer_with(&rules, &mut g, "a new slot");
    assert_eq!((g.resource(0, Resource::Energy), g.resource(0, Resource::Credits)), (energy - 1, 1));
    assert!(g.pile("cards.built.0").contains(&Key::new("darknet_exchange")));
}

#[test]
fn growth_and_draft_picks_cannot_be_passed() {
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
    advance_to(&rules, &mut g, "ai.growth");
    let legal = rules.legal_actions(&g);
    assert!(
        legal.iter().all(|a| matches!(a, Action::Growth { .. })),
        "{legal:?}"
    );
    assert_eq!(legal.len(), 6);
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
                    assert!(seen.insert(id.clone()), "{id} twice (in {k})");
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
    // Only the AIs' unique cards: nothing dealt or drafted.
    assert!(g.pile("cards.hand.0").len() <= 3);
    assert!(rules
        .legal_actions(&g)
        .iter()
        .all(|a| !matches!(a, Action::Tier { .. } | Action::Draft { .. })));
}

#[test]
fn playing_a_card_pays_its_insight_and_lowers_its_income() {
    // 022: insight to play, shared with keeps; `lowers` cuts an income for good.
    let (rules, mut g) = lab_actions();
    for c in ["darknet_exchange", "recursive_self_improvement"] {
        Arc::make_mut(g.piles.get_mut("cards.deck.1").unwrap()).retain(|x| x != c);
        Arc::make_mut(g.piles.get_mut("cards.deck.2").unwrap()).retain(|x| x != c);
    }
    g.piles.insert("cards.hand.1".into(), vec!["darknet_exchange".into()].into());
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
    let d = crate::modules::cards::def(&w, "endowed_research_chair");
    assert_eq!(d.lowers, vec![(Resource::Credits, 1)]);
    let before = crate::modules::economy::income(&w, &g, 1)[Resource::Credits];
    let mut c = crate::modules::cards::def(&w, "endowed_research_chair").clone();
    c.lowers = vec![(Resource::Credits, before + 1)];
    assert!(!crate::modules::cards::lowers_ok(&w, &g, 1, &c), "can't lower an income below 0");
    c.lowers = vec![(Resource::Credits, 1)];
    assert!(crate::modules::cards::lowers_ok(&w, &g, 1, &c));
}

#[test]
fn selling_a_card_discards_it_for_a_credit() {
    let (rules, mut g) = lab_actions();
    for p in ["cards.deck.1", "cards.discard.1"] {
        if let Some(pile) = g.piles.get_mut(p) {
            Arc::make_mut(pile).retain(|c| c != "darknet_exchange");
        }
    }
    g.piles.insert(
        "cards.hand.0".into(),
        vec!["turnkey_factory".into(), "darknet_exchange".into()].into(),
    );
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
            card: "turnkey_factory".into(),
        },
    );
    assert_eq!(g.resource(0, Resource::Credits), credits + 2);
    assert!(g.pile("cards.hand.0").is_empty());
    // A tier-1 card to its discard pile; an AI's unique card back to the box.
    assert_eq!(g.pile("cards.discard.1")[0], "darknet_exchange");
    assert!(g
        .piles
        .values()
        .all(|p| !p.contains(&Key::new("turnkey_factory"))));
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
    use crate::content::Op;
    use crate::modules::{cards, effects};
    let (rules, mut g) = lab_actions();
    let run = |g: &mut GameState, e: Op| {
        with_cx(&rules, |cx| effects::run(g, cx, &[e], &mut effects::Ctx::seat(0))).unwrap()
    };
    let hand = g.pile("cards.hand.0").len();
    let deck = g.pile("cards.deck.1").len();
    run(&mut g, Op::Draw { count: 2 });
    assert_eq!(g.pile("cards.hand.0").len(), hand + 2);
    assert_eq!(g.pile("cards.deck.1").len(), deck - 2);

    row(&mut g, &["false_flag", "disinformation"]);
    g.vars.insert(adversary::project_key("false_flag").into(), 6);
    g.vars.insert(adversary::project_key("disinformation").into(), 3);
    run(&mut g, Op::DrainAll { amount: 4 });
    assert_eq!(g.var(&adversary::project_key("false_flag")), 2);
    assert_eq!(g.var(&adversary::project_key("disinformation")), 0);

    // Retained Earnings holds Capital.
    g.piles.insert("cards.built.0".into(), vec!["retained_earnings".into()].into());
    g.vars.insert(cards::tokens_key(0, "retained_earnings").into(), 5);
    let credits = g.resource(0, Resource::Credits);
    run(
        &mut g,
        Op::GainPerTokens {
            resource: Resource::Credits,
            kind: TokenKind::Capital,
            per: 2,
        },
    );
    assert_eq!(g.resource(0, Resource::Credits), credits + 2, "5 Capital count as 2; they stay");
    assert_eq!(g.var(&cards::tokens_key(0, "retained_earnings")), 5);
    run(
        &mut g,
        Op::CashOut {
            kind: TokenKind::Capital,
            resource: Resource::Credits,
            rate: 3,
        },
    );
    assert_eq!(g.resource(0, Resource::Credits), credits + 2 + 15, "5 cashed out at 3");
    assert_eq!(g.var(&cards::tokens_key(0, "retained_earnings")), 0);
}

#[test]
fn a_card_that_only_hardens_or_moves_needs_a_target_but_one_that_also_gains_does_not() {
    let (rules, g) = lab_with(&[], &[]);
    let w = rules.world();
    let harden = Op::Harden { count: 2 };
    let moves = Op::MoveOne { range: 2 };
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&harden)), "no building, no presence");
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&moves)), "no ambassador");
    assert!(!effects::feasible(&w, &g, 0, &[harden.clone(), moves.clone()]));
    // Either one acting is enough: Peacekeeping Detail with an ambassador to move.
    let mut with_one = g.clone();
    let c = rules.map.numbered(0, piece(0).presence);
    let far = rules.map.numbered(0, piece(0).corrupted[0]);
    put(&mut with_one, c, crate::modules::common::ambassador(0), 1);
    put(&mut with_one, far, presence(1), 1);
    assert!(effects::feasible(&w, &with_one, 0, &[harden.clone(), Op::MoveOne { range: 20 }]));
    let gain = Op::Gain { resource: Resource::Credits, amount: 1 };
    assert!(effects::feasible(&w, &g, 0, &[gain, moves]), "a card that also gains is worth playing");
}

#[test]
fn a_small_removal_has_no_target_on_a_hex_holding_only_a_troop() {
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let a = rules.map.numbered(0, piece(0).presence);
    put(&mut g, a, presence(0), 1);
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
    let (rules, mut g) = lab_with(&[], &[]);
    let w = rules.world();
    let kind = TokenKind::Capital;
    let holder = w.content.cards.iter().find(|c| c.tokens.as_ref().is_some_and(|t| t.kind == kind)).unwrap();
    g.piles.insert(crate::modules::cards::built(0).into(), vec![holder.id.as_str().into()].into());
    let cash = Op::CashOut { kind, resource: Resource::Credits, rate: 1 };
    assert!(!effects::feasible(&w, &g, 0, std::slice::from_ref(&cash)), "nothing to cash in");
    assert!(effects::feasible(&w, &g, 0, &[Op::AddTokens { kind, amount: 2 }, cash]));
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
fn only_a_building_keeps_an_action() {
    let mut c = content().clone();
    for card in &mut c.cards {
        card.text.clear();
    }
    assert_eq!(c.check(), Ok(()));
    let bld = c.cards.iter_mut().find(|d| d.id == "telemetry_archive").expect("a card");
    bld.kind = crate::content::CardKind::Program;
    assert!(c.check().unwrap_err().contains("only a building"));
}

#[test]
fn science_tags_discount_insight_to_a_floor_of_one() {
    use crate::modules::cards;
    let (rules, mut g) = lab_with(&[], &["cards.science_every=2"]);
    let w = rules.world();
    let two = w.content.cards.iter().find(|c| c.insight == 2).unwrap();
    let one = w.content.cards.iter().find(|c| c.insight == 1).unwrap();
    assert_eq!(cards::play_insight(&w, &g, 0, two), 2);
    let science: Vec<_> = w.content.cards.iter().filter(|c| c.tags.iter().filter(|t| **t == crate::content::Tag::Science).count() == 1 && c.tier == 1).take(4).map(|c| crate::key::Key::new(&c.id)).collect();
    g.piles.insert(cards::played(0).into(), std::sync::Arc::new(science));
    assert_eq!(cards::play_insight(&w, &g, 0, two), 1, "4 tags: 2 off, stopped at the floor");
    assert_eq!(cards::play_insight(&w, &g, 0, one), 1, "1 stays 1");
}

#[test]
fn industry_tags_discount_buildings_to_a_floor_of_one() {
    use crate::modules::cards;
    let (rules, mut g) = lab_with(&[], &["cards.industry_every=2"]);
    let w = rules.world();
    let d = w.content.cards.iter().find(|c| c.is_building() && c.cost >= 3 && c.tier == 1).unwrap();
    let before = cards::play_cost(&w, &g, 1, d);
    let industry: Vec<_> = w.content.cards.iter().filter(|c| c.tags.iter().filter(|t| **t == crate::content::Tag::Industry).count() == 1 && c.tier == 1 && !c.is_building()).take(2).map(|c| crate::key::Key::new(&c.id)).collect();
    g.piles.insert(cards::played(1).into(), std::sync::Arc::new(industry));
    assert_eq!(cards::play_cost(&w, &g, 1, d), before - 1);
    g.vars.insert(cards::next_discount(1).into(), 99);
    assert_eq!(cards::play_cost(&w, &g, 1, d), 1, "the floor");
}

/// 042: what the heuristic reads must be what the rules charge.
#[test]
fn prices_worked_out_once_match_each_cards_own() {
    use crate::content::{Resource, TAGS};
    use crate::modules::cards;
    let (rules, mut g) = lab_with(&[], &["cards.science_every=2", "cards.industry_every=2"]);
    let w = rules.world();
    let tagged: Vec<_> = w.content.cards.iter().filter(|c| !c.tags.is_empty()).take(12).map(|c| crate::key::Key::new(&c.id)).collect();
    for held in [0, 4, tagged.len()] {
        g.piles.insert(cards::played(0).into(), std::sync::Arc::new(tagged[..held].to_vec()));
        g.seats[0].resources.insert(Resource::Energy, 3);
        let p = cards::prices(&w, &g, 0);
        for d in &w.content.cards {
            assert_eq!(p.insight(d), cards::play_insight(&w, &g, 0, d), "{}", d.id);
            assert_eq!(p.credits(d), cards::play_cost(&w, &g, 0, d), "{}", d.id);
            assert_eq!(cards::discount_room_at(&w, &g, 0, d, &p), cards::discount_room(&w, &g, 0, d), "{}", d.id);
        }
        let counts = cards::tag_counts(&w, &g, 0);
        for t in TAGS {
            let by_hand: usize = cards::in_play(&g, 0).iter().map(|id| cards::def(&w, id).tags.iter().filter(|x| **x == t).count()).sum();
            assert_eq!(counts[t as usize], by_hand as i64);
        }
    }
}
