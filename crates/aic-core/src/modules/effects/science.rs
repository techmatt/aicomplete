//! Science's niche: marker clearing, shields and delays ("up to N").

use super::*;

pub(super) fn clear_markers(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64) -> Res {
    let w = cx.world;
    for k in 0..count {
        let hexes: Vec<usize> = (0..g.hexes.len())
            .filter(|&h| marker_in_reach(&w, g, seat, h))
            .collect();
        if hexes.is_empty() {
            break;
        }
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("Seat {seat}: clear the corruption marker on which hex?");
        let Some(i) = pick_or_stop(cx, seat, prompt, opts, k == 0)? else {
            break;
        };
        board::clear_marker(g, cx, seat, hexes[i]);
    }
    Ok(())
}

pub(super) fn shield_node(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64) -> Res {
    let w = cx.world;
    for k in 0..count {
        let hexes: Vec<usize> = (0..g.hexes.len()).filter(|&h| shieldable(g, seat, h)).collect();
        if hexes.is_empty() {
            break;
        }
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("Seat {seat}: shield your nodes on which hex?");
        let Some(i) = pick_or_stop(cx, seat, prompt, opts, k == 0)? else {
            break;
        };
        cx.add_tokens(g, hexes[i], HARDEN, 1);
    }
    Ok(())
}

pub(super) fn delay_threat(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64) -> Res {
    for k in 0..count {
        let active = undelayed_threats(g);
        if active.is_empty() {
            break;
        }
        let opts = active.iter().map(|(t, _)| opt(*t)).collect();
        let prompt = || format!("Seat {seat}: delay which threat?");
        let Some(i) = pick_or_stop(cx, seat, prompt, opts, k == 0)? else {
            break;
        };
        threats::delay(g, cx, &active[i].0);
    }
    Ok(())
}
