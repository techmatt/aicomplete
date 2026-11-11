//! 166 (Matt, 2026-11-09, *Moving pieces*): Gather, Push, and Liberate paid by the ambassadors standing on the site.

use super::*;
use crate::content::Op;
use crate::modules::common::{hex_name, hex_opt, FIREWALL};
use crate::modules::{board, effects};
use crate::op::Piece;
use crate::state::Pick;


/// The hex pick, only when the prompt has more than one option (a single option is taken without asking).
fn hex_pick(w: &crate::module::World, options: &[usize], h: usize) -> Vec<Pick> {
    assert!(options.contains(&h), "{h} not among {options:?}");
    if options.len() == 1 {
        return Vec::new();
    }
    let o = hex_opt(w, h);
    vec![o.into()]
}

fn piece_pick(w: &crate::module::World, label: &str, h: usize) -> Pick {
    Pick { option: format!("{label} {}", hex_name(w, h)), target: Some(w.map.hexes[h].hex) }
}

fn stop() -> Pick {
    Pick { option: effects::STOP.to_string(), target: None }
}

/// Gathered agents arriving on a standing ambassador cancel it one for one; the next one stands; "up to" stops.
#[test]
fn gathered_agents_cancel_a_standing_ambassador() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let (c, _, a, _) = by_a_city(&mut g, m);
    put(&mut g, c, ambassador(0), 1);
    put(&mut g, a, ADV_AMBASSADOR, 3);
    let pieces = vec![Piece::Agents];
    let mut picks = hex_pick(&w, &effects::gather_hexes(&w, &g, 0, &pieces, false), c);
    picks.extend([piece_pick(&w, "1 agent from", a), piece_pick(&w, "1 agent from", a), stop()]);
    run_ops_with(&rules, &mut g, picks, &[Op::Gather { max: 3, pieces, strike: false }]);
    assert_eq!((g.tokens(c, ambassador(0)), g.tokens(c, ADV_AMBASSADOR), g.tokens(a, ADV_AMBASSADOR)), (0, 1, 1));
}

/// 178 (Matt's ruling #27, reversing D76): a strike removes up to N on or next to the hex, the player's choice
/// batch by batch, and may stop; nothing moves, so a neighbor's agents never raise the hex's Liberate bill.
#[test]
fn a_strike_removes_on_or_next_to_the_hex_your_choice() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let (c, _, a, b) = by_a_city(&mut g, m);
    put(&mut g, c, ADV_CUBE, 1);
    put(&mut g, c, ADV_AMBASSADOR, 1);
    put(&mut g, a, ADV_AMBASSADOR, 2);
    put(&mut g, b, TROOP, 1);
    let pieces = vec![Piece::Agents, Piece::Troops];
    let mut picks = hex_pick(&w, &effects::gather_hexes(&w, &g, 0, &pieces, true), c);
    picks.extend([piece_pick(&w, "1 agent on", c), piece_pick(&w, "2 agents on", a), stop()]);
    run_ops_with(&rules, &mut g, picks, &[Op::Gather { max: 6, pieces, strike: true }]);
    assert_eq!((g.tokens(c, ADV_AMBASSADOR), g.tokens(a, ADV_AMBASSADOR), g.tokens(b, TROOP)), (0, 0, 1), "stopped before the troop");
    assert_eq!(g.stats.counters["effects.struck"], 3);
}

/// A push sends each piece to a neighbor the player picks; a firewalled hex takes no agent; troops just move.
#[test]
fn a_push_moves_pieces_next_door_but_not_agents_onto_a_firewall() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let (c, _, a, b) = by_a_city(&mut g, m);
    put(&mut g, c, ADV_AMBASSADOR, 2);
    put(&mut g, c, TROOP, 1);
    put(&mut g, a, FIREWALL, 1);
    let pieces = vec![Piece::Agents, Piece::Troops];
    let mut picks = hex_pick(&w, &effects::push_hexes(&w, &g, 0, &pieces), c);
    picks.extend([piece_pick(&w, "1 agent to", b), piece_pick(&w, "1 troop to", a), stop()]);
    // The firewall refuses the agent: an "agent to a" pick would be illegal.
    run_ops_with(&rules, &mut g, picks, &[Op::Push { max: 3, pieces }]);
    assert_eq!((g.tokens(c, ADV_AMBASSADOR), g.tokens(b, ADV_AMBASSADOR), g.tokens(a, TROOP)), (1, 1, 1));
    assert_eq!(g.tokens(a, ADV_AMBASSADOR), 0);
}

/// 169: Wall holds against a push: no troop arrives on the Protector's node (seat 1 in the lab), however it moves.
#[test]
fn a_push_moves_no_troop_onto_a_protector_node() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    let (c, _, a, b) = by_a_city(&mut g, m);
    put(&mut g, c, TROOP, 1);
    put(&mut g, a, node(1), 1);
    assert!(board::walled(&w, &g, a));
    let pieces = vec![Piece::Troops];
    let mut picks = hex_pick(&w, &effects::push_hexes(&w, &g, 0, &pieces), c);
    picks.push(piece_pick(&w, "1 troop to", a));
    // Not an option: the push refuses the pick and moves nothing.
    let _ = with_cx_picks(&rules, picks, |cx| effects::run(&mut g, cx, &[Op::Push { max: 1, pieces: pieces.clone() }], &mut effects::Ctx::seat(0)));
    assert_eq!((g.tokens(a, TROOP), g.tokens(c, TROOP)), (0, 1));
    let mut picks = hex_pick(&w, &effects::push_hexes(&w, &g, 0, &pieces), c);
    picks.push(piece_pick(&w, "1 troop to", b));
    run_ops_with(&rules, &mut g, picks, &[Op::Push { max: 1, pieces }]);
    assert_eq!((g.tokens(b, TROOP), g.tokens(c, TROOP)), (1, 0));
}

/// r75 (Matt's ruling 2): Liberate spends the acting seat's own ambassadors standing on the site first, never
/// another player's; its supply pays the rest, and its cube goes on.
#[test]
fn liberate_spends_your_own_ambassadors_on_the_site() {
    let (rules, mut g) = lab_with(&[], &["board.liberate_at=5"]);
    let w = rules.world();
    let m = &rules.map;
    let (c, _, _, _) = by_a_city(&mut g, m);
    put(&mut g, c, ADV_CUBE, 1);
    put(&mut g, c, ambassador(0), 2);
    put(&mut g, c, ambassador(1), 1);
    let n = board::liberate_cost(&w, &g, c).unwrap();
    assert_eq!(board::site_supply_cost(&w, &g, 0, c, true), Some(n - 2));
    assert_eq!(board::site_supply_cost(&w, &g, 1, c, true), Some(n - 1), "only your own pay");
    set_res(&mut g, 0, Resource::Ambassadors, n - 3);
    assert!(!board::liberable(&w, &g, 0, c));
    set_res(&mut g, 0, Resource::Ambassadors, n - 2);
    assert!(board::liberable(&w, &g, 0, c));
    act(&rules, &mut g, std_act(0, "liberate"));
    pick(&rules, &mut g, c);
    let left = (g.tokens(c, ADV_CUBE), g.tokens(c, ambassador(0)), g.tokens(c, ambassador(1)), g.resource(0, Resource::Ambassadors));
    assert_eq!(left, (0, 0, 1, 0));
    assert_eq!(g.tokens(c, cube(0)), 1, "liberated: yours");
}
