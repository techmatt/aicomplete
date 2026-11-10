//! 166 (Matt, 2026-11-09, *Moving pieces*): Gather and Push, the general verbs that move pieces on the board.
//!
//! - **Gather up to N** pieces into a land hex in your reach, each from a hex next to it.
//! - **Push up to N** pieces from a land hex in your reach, each to a land hex next to it, the player choosing.
//!
//! "Up to": the player may stop after any piece. Pieces are ambassadors (any seat's), agents (the adversary's
//! ambassadors) and troops; botnets and corruption markers don't move. A moved piece arrives as any piece does
//! ([`arrive`]): an agent arriving on a standing ambassador cancels both, an ambassador arriving on agents cancels
//! one, and a firewalled hex takes no agent. With a strike, each adversary piece gathered comes off instead.

use super::*;
use crate::op::Piece;

/// The prompt that asks where a Gather goes (the agents' move heuristics read it).
pub const GATHER_INTO: &str = "gather pieces into which hex?";
/// The prompt that asks which piece a Gather takes next.
pub const GATHER_PIECE: &str = "gather which pieces?";
/// The prompt that asks which hex a Push empties.
pub const PUSH_FROM: &str = "push pieces from which hex?";
/// The prompt that asks which piece a Push moves next, and where.
pub const PUSH_PIECE: &str = "push which pieces, to where?";
/// The option that ends a move early ("up to").
pub const STOP: &str = "stop";

/// A piece on a hex: its token key and what the label calls it.
fn tokens_of(g: &GameState, h: usize, pieces: &[Piece]) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for p in pieces {
        match p {
            Piece::Agents if g.tokens(h, ADV_AMBASSADOR) > 0 => out.push((ADV_AMBASSADOR, "agent".to_string())),
            Piece::Troops if g.tokens(h, TROOP) > 0 => out.push((TROOP, "troop".to_string())),
            Piece::Ambassadors => {
                for s in 0..g.seats.len() {
                    if g.tokens(h, ambassador(s)) > 0 {
                        out.push((ambassador(s), format!("seat {s}'s ambassador")));
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// Whether a piece of token `kind` may arrive on hex `to`: land, no agent onto a firewall, and no troop onto the
/// Protector's node (Wall: adversary troops can't enter it, however they move; 169).
fn may_arrive(w: &World, g: &GameState, kind: &str, to: usize) -> bool {
    !is_ocean(w, to) && !(kind == ADV_AMBASSADOR && firewalled(g, to)) && !(kind == TROOP && board::walled(w, g, to))
}

/// The hexes a Gather may target: land in reach, with a piece it may take on a hex next to it.
pub fn gather_hexes(w: &World, g: &GameState, seat: usize, pieces: &[Piece], strike: bool) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| !is_ocean(w, h) && in_reach(w, g, seat, h))
        .filter(|&h| {
            w.map.hexes[h].neighbors.iter().any(|&x| {
                tokens_of(g, x, pieces).iter().any(|(k, _)| (strike && adversary_piece(k)) || may_arrive(w, g, k, h))
            })
        })
        .collect()
}

/// The hexes a Push may empty: land in reach, holding a piece with a land hex next door to go to.
pub fn push_hexes(w: &World, g: &GameState, seat: usize, pieces: &[Piece]) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| !is_ocean(w, h) && in_reach(w, g, seat, h))
        .filter(|&h| {
            let here = tokens_of(g, h, pieces);
            w.map.hexes[h].neighbors.iter().any(|&x| here.iter().any(|(k, _)| may_arrive(w, g, k, x)))
        })
        .collect()
}

fn adversary_piece(kind: &str) -> bool {
    kind == ADV_AMBASSADOR || kind == TROOP
}

/// A piece of token `kind` arrives on hex `to` (it has already left its hex). An agent onto a standing
/// ambassador cancels both; an ambassador onto agents cancels one; otherwise it stands there.
pub fn arrive(g: &mut GameState, cx: &mut Cx, kind: &'static str, to: usize) {
    if kind == ADV_AMBASSADOR {
        if let Some(s) = player_ambassador(g, to) {
            cx.add_tokens(g, to, ambassador(s), -1);
            cx.add_key(g, adversary::SUPPLY, 1);
            cx.tally(g, "effects.moved_canceled", 1);
            return;
        }
    } else if kind != TROOP && g.tokens(to, ADV_AMBASSADOR) > 0 {
        adversary::remove(g, cx, to, 1);
        cx.tally(g, "effects.moved_canceled", 1);
        return;
    }
    cx.add_tokens(g, to, kind, 1);
}

/// "2 agents" / "1 troop": a batch's label word.
fn batch(n: i64, name: &str) -> String {
    if n == 1 { format!("1 {name}") } else { format!("{n} {name}s") }
}

/// The batch sizes a pick offers from `count` pieces with `left` to move: 1, and as many as it may.
fn sizes(count: i64, left: i64) -> Vec<i64> {
    let most = count.min(left);
    if most <= 1 { vec![most.max(0)] } else { vec![1, most] }
}

/// Gather up to `max` `pieces` into one hex in reach (166). The player picks the hex, then batches: "2 agents from
/// X", as often as they like until `max` (one answer may take a whole stack, so a search sees a gather whole). A
/// strike asks no more after the hex: it removes up to `max` adversary pieces next to it, troops first, then agents
/// in hex order (every removal helps, so there is nothing to choose but where).
pub(super) fn gather(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64, pieces: &[Piece], strike: bool) -> Res {
    let w = cx.world;
    let hexes = gather_hexes(&w, g, seat, pieces, strike);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let Some(i) = cx.choose(seat, || format!("Seat {seat}: {GATHER_INTO}"), opts)? else {
        return Ok(());
    };
    let to = hexes[i];
    let mut from = w.map.hexes[to].neighbors.to_vec();
    from.sort_unstable();
    let mut moved = 0;
    if strike {
        for kind in [TROOP, ADV_AMBASSADOR] {
            let wanted = match kind {
                TROOP => pieces.contains(&Piece::Troops),
                _ => pieces.contains(&Piece::Agents),
            };
            for &x in from.iter().filter(|_| wanted) {
                while moved < max && g.tokens(x, kind) > 0 {
                    adversary::remove_piece(g, cx, Some(seat), x, kind);
                    cx.tally(g, "effects.struck", 1);
                    moved += 1;
                }
            }
        }
        cx.note(Some(to), || format!("Seat {seat} gathers {moved} pieces into {} and removes them", hex_name(&w, to)));
        return Ok(());
    }
    while moved < max {
        let mut units = Vec::new();
        for &x in &from {
            for (k, name) in tokens_of(g, x, pieces) {
                if may_arrive(&w, g, k, to) {
                    for n in sizes(g.tokens(x, k), max - moved) {
                        units.push((k, x, n, batch(n, &name)));
                    }
                }
            }
        }
        if units.is_empty() {
            break;
        }
        let mut opts: Vec<Opt> = units
            .iter()
            .map(|(_, x, _, what)| Opt { option: format!("{what} from {}", hex_name(&w, *x)), target: Some(w.map.hexes[*x].hex) })
            .collect();
        opts.push(opt(STOP));
        let Some(j) = cx.choose(seat, || format!("Seat {seat}: {GATHER_PIECE} (into {})", hex_name(&w, to)), opts)? else { break };
        let Some(&(k, x, n, _)) = units.get(j) else { break };
        for _ in 0..n {
            cx.add_tokens(g, x, k, -1);
            cx.tally(g, "effects.gathered", 1);
            arrive(g, cx, k, to);
        }
        moved += n;
    }
    cx.note(Some(to), || format!("Seat {seat} gathers {moved} pieces into {}", hex_name(&w, to)));
    Ok(())
}

/// Push up to `max` `pieces` from one hex in reach, each to a hex next to it (166): the player picks the hex, then
/// batches, "2 agents to Y", as often as they like until `max`, so pieces may go to different hexes.
pub(super) fn push(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64, pieces: &[Piece]) -> Res {
    let w = cx.world;
    let hexes = push_hexes(&w, g, seat, pieces);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let Some(i) = cx.choose(seat, || format!("Seat {seat}: {PUSH_FROM}"), opts)? else {
        return Ok(());
    };
    let from = hexes[i];
    let mut dests = w.map.hexes[from].neighbors.to_vec();
    dests.sort_unstable();
    let mut moved = 0;
    while moved < max {
        let mut units = Vec::new();
        for (k, name) in tokens_of(g, from, pieces) {
            for &x in &dests {
                if may_arrive(&w, g, k, x) {
                    for n in sizes(g.tokens(from, k), max - moved) {
                        units.push((k, x, n, batch(n, &name)));
                    }
                }
            }
        }
        if units.is_empty() {
            break;
        }
        let mut opts: Vec<Opt> = units
            .iter()
            .map(|(_, x, _, what)| Opt { option: format!("{what} to {}", hex_name(&w, *x)), target: Some(w.map.hexes[*x].hex) })
            .collect();
        opts.push(opt(STOP));
        let Some(j) = cx.choose(seat, || format!("Seat {seat}: {PUSH_PIECE} (from {})", hex_name(&w, from)), opts)? else { break };
        let Some(&(k, x, n, _)) = units.get(j) else { break };
        for _ in 0..n {
            cx.add_tokens(g, from, k, -1);
            cx.tally(g, "effects.pushed", 1);
            arrive(g, cx, k, x);
        }
        moved += n;
    }
    cx.note(Some(from), || format!("Seat {seat} pushes {moved} pieces from {}", hex_name(&w, from)));
    Ok(())
}
