//! 286 (284's audit pick): the ops no content prints, each run once on a lab state, so a rules change that breaks
//! one shows here and not first on the card that prints it. These ops stay until the card set freezes, and are
//! deleted then unless a card prints them (Matt's pick); a printed one leaves this list for its card's tests.
//! `every_unprinted_op_has_a_lab_run` (aic-sim's docs tests) checks every op `docs/ops.md` counts 0 times is here.

use super::*;
use crate::content::Op;
use crate::modules::{adversary, alignment, cards, effects, threats};

/// Who resolves a sample: a seat's card, a threat hitting seat 0 in country 0, or the team.
#[derive(Clone, Copy, Debug)]
enum By {
    Seat,
    Threat,
    Team,
}

/// Each unprinted op the effects resolve, as a list of ops it runs in (a rider after the op it reads), by whom,
/// and whether the lab state below gives it something to change. Its name is the first `"op"` naming it.
const RESOLVED: &[(&str, By, bool)] = &[
    (r#"[{"op":"choose_player"},{"op":"transfer","resource":"credits","max":2}]"#, By::Seat, true),
    (r#"[{"op":"damage_country","ambassadors":1,"troops":1}]"#, By::Seat, true),
    (r#"[{"op":"peek","count":2}]"#, By::Seat, true),
    (r#"[{"op":"cut_botnet"}]"#, By::Seat, false),
    (r#"[{"op":"move_one","range":2}]"#, By::Seat, false),
    (r#"[{"op":"spread_ambassadors"}]"#, By::Seat, false),
    (r#"[{"op":"if_tags","tag":"science","count":0,"then":[{"op":"gain","resource":"credits","amount":1}]}]"#, By::Seat, true),
    (r#"[{"op":"place_node","count":1},{"op":"if_node_next","spots":["city","station","factory"],"then":[{"op":"gain","resource":"credits","amount":1}]}]"#, By::Seat, true),
    (r#"[{"op":"place_node","count":1},{"op":"ambassador_to_spot"}]"#, By::Seat, true),
    (r#"[{"op":"copy_income"}]"#, By::Seat, false),
    (r#"[{"op":"discard_hand","resource":"credits","amount":2}]"#, By::Seat, true),
    (r#"[{"op":"lose_node_or","resource":"credits","amount":2}]"#, By::Threat, true),
    (r#"[{"op":"discard_most_expensive"}]"#, By::Threat, true),
    (r#"[{"op":"compute_every","amount":2}]"#, By::Team, true),
    (r#"[{"op":"compute_per_missing","resource":"antimatter","amount":1}]"#, By::Threat, true),
    (r#"[{"op":"lose_ambassadors_or","count":1,"resource":"credits","amount":2}]"#, By::Threat, true),
    (r#"[{"op":"lose_income","resource":"credits","amount":1}]"#, By::Threat, true),
    (r#"[{"op":"lose_or","options":[{"op":"lose_income","resource":"credits","amount":1},{"op":"discard_most_expensive"}]}]"#, By::Threat, true),
    (r#"[{"op":"occupy_factory","else_compute":2}]"#, By::Threat, true),
    (r#"[{"op":"counter_all"}]"#, By::Team, true),
    (r#"[{"op":"pushback","ambassadors":1}]"#, By::Team, false),
    (r#"[{"op":"remove_botnets","count":1}]"#, By::Team, true),
    (r#"[{"op":"one_player","then":[{"op":"gain","resource":"credits","amount":1}]}]"#, By::Team, true),
];

/// The lab after setup, the board as set up: seat 0 with 10 of each resource, a card in hand and a threat on
/// country 0 it may counter, so most samples have something to act on.
fn stocked() -> (Rules, GameState) {
    let (rules, mut g) = lab_actions();
    for r in [Resource::Credits, Resource::Energy, Resource::Insight, Resource::Chips, Resource::Ambassadors] {
        set_res(&mut g, 0, r, 10);
    }
    let card = rules.world().content.cards[0].id.clone();
    set_pile(&mut g, &cards::hand(0), &[card.as_str()]);
    counterable_manhunt(&rules, &mut g);
    (rules, g)
}

fn ctx(by: By) -> effects::Ctx {
    match by {
        By::Seat => effects::Ctx::seat(0),
        By::Threat => effects::Ctx::threat(Some(0)).hitting(Some(0)).why("test"),
        By::Team => effects::Ctx::team().named("test"),
    }
}

fn name(sample: &str) -> &str {
    let ops: Vec<Op> = serde_json::from_str(sample).unwrap();
    let i = if matches!(ops[0], Op::ChoosePlayer | Op::PlaceNode { .. }) { 1 } else { 0 };
    let rest = &sample[sample.match_indices(r#""op":""#).nth(i).unwrap().0 + 6..];
    &rest[..rest.find('"').unwrap()]
}

/// Every resolving unprinted op runs once, its asks answered by the last option (a transfer's most), without a panic or an error;
/// those the lab gives something to act on change the state.
#[test]
fn every_unprinted_op_resolves_once_on_a_lab_state() {
    let (rules, base) = stocked();
    let mut idle = Vec::new();
    for &(sample, by, changes) in RESOLVED {
        let ops: Vec<Op> = serde_json::from_str(sample).unwrap_or_else(|e| panic!("{sample}: {e}"));
        let mut g = base.clone();
        with_answers(&rules, &mut g, |a| a.options.len() - 1, |g, cx| effects::run(g, cx, &ops, &mut ctx(by)));
        if changes == (g == base) {
            idle.push((name(sample), changes));
        }
    }
    assert!(idle.is_empty(), "(op, expected a change) the lab didn't match: {idle:?}");
    // A few effects worth reading, cheaply.
    let run = |sample: &str, by| {
        let ops: Vec<Op> = serde_json::from_str(sample).unwrap();
        let mut g = base.clone();
        with_answers(&rules, &mut g, |_| 0, |g, cx| effects::run(g, cx, &ops, &mut ctx(by)));
        g
    };
    let g = run(r#"[{"op":"lose_income","resource":"credits","amount":1}]"#, By::Threat);
    assert_eq!(g.var(&crate::modules::economy::income_debt(0, Resource::Credits)), 1);
    let g = run(r#"[{"op":"counter_all"}]"#, By::Team);
    assert!(threats::active(&g).is_empty(), "Manhunt countered");
    let g = run(r#"[{"op":"discard_hand","resource":"credits","amount":2}]"#, By::Seat);
    assert_eq!(g.resource(0, Resource::Credits), 12);
    assert!(g.pile(&cards::hand(0)).is_empty());
}

/// `"cards.orbital_embassy.passives=[...]"`: a building of seat 0's holding `passives`.
fn holding(passives: &str) -> String {
    format!("cards.orbital_embassy.passives={passives}")
}

/// The read ops (standing lines a reader looks up, never resolved), each read once by its reader.
#[test]
fn every_unprinted_read_op_is_read_once_on_a_lab_state() {
    let built = |g: &mut GameState| set_pile(g, &cards::built(0), &["orbital_embassy"]);
    // slots: building capacity.
    let (rules, mut g) = lab_actions_with(&[&holding(r#"[{"op":"slots","amount":2}]"#)]);
    let w = rules.world();
    let before = cards::capacity(&w, &g, 0);
    built(&mut g);
    assert_eq!(cards::capacity(&w, &g, 0), before + 2);
    // project_drain and convert_upkeep: the economy's upkeep.
    let (rules, mut g) = lab_actions_with(&[&holding(
        r#"[{"op":"project_drain","amount":1},{"op":"convert_upkeep","from":"energy","amount":1,"to":"credits","gain":2}]"#,
    )]);
    built(&mut g);
    set_res(&mut g, 0, Resource::Energy, 3);
    set_res(&mut g, 0, Resource::Credits, 0);
    with_cx(&rules, |cx| cards::upkeep(&mut g, cx, 0));
    assert_eq!((g.resource(0, Resource::Energy), g.resource(0, Resource::Credits)), (2, 2));
    // unity_bonus: more Sway toward Unity for a troop seat 0 removes.
    let (rules, mut g) = lab_actions_with(&[&holding(r#"[{"op":"unity_bonus","amount":2}]"#)]);
    let mut plain = g.clone();
    built(&mut g);
    with_cx(&rules, |cx| alignment::for_troop_removed(&mut plain, cx, Some(0)));
    with_cx(&rules, |cx| alignment::for_troop_removed(&mut g, cx, Some(0)));
    plain.piles = g.piles.clone();
    assert_ne!(g, plain, "the bonus Sway");
    // tags_from_hand: a card in hand becomes a sacrifice source.
    let (rules, mut g) = lab_actions_with(&[&holding(r#"[{"op":"tags_from_hand","tag":"any"}]"#)]);
    let w = rules.world();
    let card = w.content.cards.iter().find(|c| !c.tags.is_empty()).unwrap().id.clone();
    set_pile(&mut g, &cards::hand(0), &[card.as_str()]);
    let before = cards::sources(&w, &g, 0).len();
    built(&mut g);
    assert!(cards::sources(&w, &g, 0).len() > before, "the hand's card is a source");
    // logistics: a payer of a countered threat gains it.
    let (rules, mut g) = threat_lab_with(&[
        r#"threats.eviction.counters=[[{"resource":"credits","amount":2}]]"#,
        &holding(r#"[{"op":"logistics","resource":"credits","amount":3}]"#),
    ]);
    let m = &rules.map;
    let city = cities(m).into_iter().find(|&h| m.hexes[h].country == 0).unwrap();
    put(&mut g, city, crate::modules::common::node(0), 1);
    deal_at(&mut g, "eviction", 0, &[city]);
    built(&mut g);
    set_res(&mut g, 0, Resource::Credits, 10);
    act(&rules, &mut g, Action::Pay { seat: 0, threat: "eviction".into(), resource: Resource::Credits, amount: 2 });
    assert!(threats::active(&g).iter().all(|(t, _)| &**t != "eviction"), "countered");
    assert_eq!(g.resource(0, Resource::Credits), 11, "10, less 2 paid, plus 3");
    // standing_troops_bonus: the adversary board's difficulty line (no overlay reaches the board: a copy of it).
    let (rules, _) = lab_actions_with(&["adversary.difficulty=4"]);
    let mut content = rules.world().content.clone();
    content.adversary_board.difficulty.effects =
        vec![serde_json::from_str(r#"{"op":"standing_troops_bonus","amount":2}"#).unwrap()];
    let w = module::World { content: &content, ..rules.world() };
    assert_eq!(adversary::troop_bonus(&w), 2 * i64::from(adversary::level(&w, w.int("adversary.board_difficulty_at"))));
    assert_eq!(adversary::troop_bonus(&w), 2, "level 4 reads the difficulty line");
}
