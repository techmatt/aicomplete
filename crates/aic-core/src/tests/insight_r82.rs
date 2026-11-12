//! r82 (189; Matt, 2026-11-11): insight is this round's attention, readable off the board; keeps cost insight
//! by tier in every round with no credit fallback; the opening hand is dealt free and round 1 drafts; no
//! adversary harm takes insight. (Patience: `researcher.rs`; Expand: `node_powers.rs`; Aegis: `ambassadors.rs`;
//! the blocked Rally share: `fear_cards.rs`.)

use super::*;
use crate::content::{Currency, Op};
use crate::modules::{ai, board, cards, economy};

/// The board prints 3 insight (196, r86; 6 before), as the card-play track's start; the setup node's track space adds
/// none, so a new game's insight is the printed 3.
#[test]
fn the_board_prints_3_insight_and_the_setup_space_adds_none() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let g = rules.new_game(1);
    for s in 0..g.seats.len() {
        let b = ai::board(&w, s);
        assert_eq!(b.insight, 3, "{}", b.id);
        assert!(b.economy_track.iter().all(|u| u.insight.is_none() && u.income.iter().all(|(r, _)| *r != Resource::Insight)));
        assert_eq!(economy::income(&w, &g, s)[Resource::Insight], 3, "{}", b.id);
    }
}

/// Amendment 8615 (Spirit Island style): the card-play track prints insight totals; your insight is the number on
/// the rightmost space you've uncovered that prints one (a space printing none leaves it).
#[test]
fn the_card_play_track_prints_insight_totals_and_the_rightmost_counts() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(1);
    let b = ai::board(&w, 0);
    let mut shown = b.insight;
    for i in 0..b.play_track.len() {
        g.vars.insert(board::track2_key(0).into(), i as i64 + 1);
        shown = b.play_track[i].insight.unwrap_or(shown);
        assert_eq!(ai::track_insight(&w, &g, 0), shown, "space {}", i + 1);
        assert_eq!(economy::income(&w, &g, 0)[Resource::Insight], shown);
    }
    assert_eq!(shown, 13, "the Builder's last total");
    let mut bad = w.content.ais[0].clone();
    bad.play_track[2].insight = Some(5);
    w.content.check_ai_board(&w.content.ais[0]).unwrap();
    let e = w.content.check_ai_board(&bad).unwrap_err();
    assert!(e.contains("insight totals rise"), "{e}");
}

/// The opening: 2 tier-1 and 2 tier-2 cards dealt into each hand, kept free; then round 1 builds a hand like
/// every round.
#[test]
fn the_opening_deals_2_and_2_free_then_round_1_drafts() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(3);
    advance_to(&rules, &mut g, "cards.tier");
    assert_eq!(g.round, 1);
    for s in 0..g.seats.len() {
        let tiers: Vec<u8> = g.pile(&cards::hand(s)).iter().map(|id| cards::def(&w, id).tier).collect();
        assert_eq!(tiers.iter().filter(|&&t| t == 1).count(), 2, "seat {s}: {tiers:?}");
        assert_eq!(tiers.iter().filter(|&&t| t == 2).count(), 2, "seat {s}: {tiers:?}");
        assert!(g.pile(&cards::drafted(s)).is_empty());
    }
    assert!(rules.legal_actions(&g).iter().any(|a| matches!(a, Action::Tier { .. })), "round 1 builds a draft hand");
    // The opening hand cost nothing (the Protector's Vigil may have added 1 in the march).
    assert!((0..g.seats.len()).all(|s| g.resource(s, Resource::Insight) >= 3));
}

/// Keeps cost 1 insight a tier-1 card and 2 a tier-2, in round 1 too, and no credits buy a shortfall.
#[test]
fn keeps_cost_insight_by_tier_in_round_1_and_credits_never_buy_them() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let mut g = rules.new_game(5);
    advance_to(&rules, &mut g, "cards.keep");
    assert_eq!(g.round, 1);
    let one = w.content.cards.iter().find(|c| c.tier == 1).unwrap().id.clone();
    let two = w.content.cards.iter().find(|c| c.tier == 2).unwrap().id.clone();
    for id in [&one, &two] {
        out_of_decks(&mut g, id);
        for s in 0..g.seats.len() {
            for pile in [cards::drafted(s), cards::hand(s)] {
                let rest: Vec<String> = g.pile(&pile).iter().filter(|c| c.as_str() != id).map(|c| c.to_string()).collect();
                set_pile(&mut g, &pile, &rest.iter().map(String::as_str).collect::<Vec<_>>());
            }
        }
    }
    set_pile(&mut g, &cards::drafted(0), &[one.as_str(), two.as_str()]);
    g.seats[0].passed = false;
    assert_eq!((cards::keep_need(&w, &g, 0, 1), cards::keep_need(&w, &g, 0, 2)), (1, 2));
    let keeps = |g: &GameState| -> Vec<String> {
        rules.legal_actions(g).iter().filter_map(|a| match a {
            Action::Keep { seat: 0, card } => Some(card.to_string()),
            _ => None,
        }).collect()
    };
    set_res(&mut g, 0, Resource::Credits, 100);
    set_res(&mut g, 0, Resource::Insight, 0);
    assert!(keeps(&g).is_empty(), "credits never pay a keep");
    set_res(&mut g, 0, Resource::Insight, 1);
    assert_eq!(keeps(&g), vec![one.clone()], "1 insight keeps a tier-1 card only");
    rules.apply_mut(&mut g, &Action::Keep { seat: 0, card: one.as_str().into() }, None).unwrap();
    assert_eq!((g.resource(0, Resource::Insight), g.resource(0, Resource::Credits)), (0, 100));
}

/// Insight is spent this turn: no threat, project, adversary card or Discord card takes it, and content that
/// tried would be refused.
#[test]
fn insight_is_never_an_adversary_harm() {
    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let c = &w.content;
    let sources = c.threats.iter().map(|t| (&t.id, &t.effects)).chain(c.projects.iter().map(|p| (&p.id, &p.effects)));
    for (id, effects) in sources {
        assert!(!content::harm_currencies(effects).contains(&Currency::Resource(Resource::Insight)), "{id}");
    }
    let lose = [Op::PlayersLose { resource: Resource::Insight, amount: 2, all: false }];
    assert!(content::check_ops(&lose, crate::op::Slot::Threat, "test").unwrap_err().contains("insight is never a harm"));
}
