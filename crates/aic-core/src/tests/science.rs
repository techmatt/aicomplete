//! Science on the board: shields, and the effects only a Science card may print.

use super::*;
use crate::content::Op;
use crate::modules::board;

#[test]
fn a_shield_absorbs_a_destruction() {
    let (rules, mut g) = at_step(&[], 1);
    let a = rules.map.numbered(0, piece(0).node);
    run_ops(&rules, &mut g, &[Op::ShieldNode { count: 1 }]);
    assert_eq!(g.tokens(a, HARDEN), 1);
    let destroyed = destroy_with(&rules, &mut g, a, node(0), board::Cause::Troop, true);
    assert!(!destroyed, "absorbed");
    assert_eq!(g.tokens(a, node(0)), 1);
}

#[test]
fn a_shield_goes_with_a_token_lost_another_way() {
    let (rules, mut g) = at_step(&[], 1);
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, HARDEN, 1);
    g.vars.insert(STAGE.into(), 2);
    with_cx(&rules, |cx| board::lose_node(&mut g, cx, a, 0, "test"));
    assert_eq!(g.tokens(a, HARDEN), 0);
}

#[test]
fn shields_delays_and_marker_clearing_are_refused_off_science() {
    for op in ["shield_node", "delay_threat", "clear_markers"] {
        let set = format!("cards.payroll_automation.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        let err = Rules::new(&baseline_with(&[&set])).err().expect("refused");
        assert!(err.contains("Science"), "{err}");
        let on_science = format!("cards.shared_compute_grant.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        assert!(Rules::new(&baseline_with(&[&on_science])).is_ok());
    }
}
