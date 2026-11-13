//! Shields, delays and marker clearing on the board (Science's niche until 212; any tag's now).

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

/// 212 (Matt, 2026-11-13): no tag owns an effect, so a card of any tag may print the three (046's Science
/// niche is gone); a board power still prints no shield or delay.
#[test]
fn shields_delays_and_marker_clearing_go_on_any_tag_but_shields_and_delays_on_no_power() {
    for op in ["shield_node", "delay_threat", "clear_markers"] {
        let set = format!("cards.payroll_automation.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        assert!(Rules::new(&baseline_with(&[&set])).is_ok(), "{op} on an Industry card");
    }
    for op in ["shield_node", "delay_threat"] {
        let set = format!("ais.builder.powers.0.effects=[{{\"op\":\"{op}\",\"count\":1}}]");
        let err = Rules::new(&baseline_with(&[&set])).err().expect("refused");
        assert!(err.contains("the cards'"), "{err}");
    }
}
