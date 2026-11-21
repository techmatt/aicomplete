//! 166 (Matt, 2026-11-09, *Moving pieces*): Gather and Push, the general verbs that move pieces on the board.
//!
//! - **Gather up to N** pieces into a land hex in your reach, each from a hex next to it.
//! - **Push up to N** pieces from a land hex in your reach, each to a land hex next to it, the player choosing.
//!
//! "Up to": the player may stop after any piece. Pieces are ambassadors (any seat's), agents (the adversary's
//! ambassadors) and troops; botnets don't move. A moved piece arrives as any piece does
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
/// The prompt that asks where a strike lands (178: "remove up to N on or next to a hex in reach").
pub const STRIKE_AT: &str = "remove pieces on or next to which hex?";
/// The prompt that asks which adversary pieces a strike removes next.
pub const STRIKE_PIECE: &str = "remove which pieces?";
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

/// The hexes a Gather may target: land in reach, with a piece it may take on a hex next to it; a strike's (178):
/// land in reach with an adversary piece on or next to it.
/// r142 (308): a strike also takes a hex outside node reach holding a troop on or next to your ambassador (Kettle
/// Operation's "Target: Agents or Troops"); around it, agents come off only where your nodes reach ([`strike`]).
pub fn gather_hexes(w: &World, g: &GameState, seat: usize, pieces: &[Piece], strike: bool) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| !is_ocean(w, h) && (in_reach(w, g, seat, h) || (strike && troop_by_ambassador(w, g, seat, h, pieces))))
        .filter(|&h| {
            if strike {
                return next_to(w, h, |x| tokens_of(g, x, pieces).iter().any(|(k, _)| adversary_piece(k)));
            }
            gathers_into(w, g, h, pieces)
        })
        .collect()
}

/// 238: a Gather of `pieces` into hex `h` moves one: a piece next to it may arrive there (the Gather's test, and
/// the `ambassadors_next` target's).
pub fn gathers_into(w: &World, g: &GameState, h: usize, pieces: &[Piece]) -> bool {
    !is_ocean(w, h)
        && w.map.hexes[h].neighbors.iter().any(|&x| tokens_of(g, x, pieces).iter().any(|(k, _)| may_arrive(w, g, k, h)))
}

/// The pieces seat `seat` may push from hex `h`: all of `pieces` there in its reach; r142 (308): a troop on or next to
/// one of its ambassadors is in its reach too (troops.md, *Guarding*; [`super::target_hexes`]'s rule), so a Push takes
/// only the troops from such a hex outside its node reach.
fn pushable(w: &World, g: &GameState, seat: usize, h: usize, pieces: &[Piece]) -> Vec<(&'static str, String)> {
    if is_ocean(w, h) {
        return Vec::new();
    }
    let mut here = tokens_of(g, h, pieces);
    if !in_reach(w, g, seat, h) {
        let by_ambassador = troop_by_ambassador(w, g, seat, h, pieces);
        here.retain(|(k, _)| by_ambassador && *k == TROOP);
    }
    here
}

/// r142: hex `h` holds a troop `pieces` may take, on or next to one of seat `seat`'s ambassadors (in its reach for
/// anything that targets troops, wherever its nodes are).
fn troop_by_ambassador(w: &World, g: &GameState, seat: usize, h: usize, pieces: &[Piece]) -> bool {
    pieces.contains(&Piece::Troops) && g.tokens(h, TROOP) > 0 && next_to(w, h, |x| g.tokens(x, ambassador(seat)) > 0)
}

/// The hexes a Push may empty: land holding a piece it may push ([`pushable`]) with a land hex next door to go to.
pub fn push_hexes(w: &World, g: &GameState, seat: usize, pieces: &[Piece]) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| {
            let here = pushable(w, g, seat, h, pieces);
            w.map.hexes[h].neighbors.iter().any(|&x| here.iter().any(|(k, _)| may_arrive(w, g, k, x)))
        })
        .collect()
}

fn adversary_piece(kind: &str) -> bool {
    kind == ADV_AMBASSADOR || kind == TROOP
}

/// A piece of token `kind` arrives on hex `to` (it has already left its hex). An agent onto a standing
/// ambassador cancels both; an ambassador onto agents cancels one; otherwise it stands there.
///
/// r142: a player ambassador keeps the state it left with (`used`, from [`leave`]): a card's move is not its own
/// action, so a tipped one arrives tipped.
pub fn arrive(g: &mut GameState, cx: &mut Cx, kind: &'static str, to: usize, used: bool) -> Res {
    if kind == ADV_AMBASSADOR {
        // r75: the players choose whose ambassador it cancels.
        if board::agent_cancels(g, cx, to)?.is_some() {
            cx.add_key(g, adversary::SUPPLY, 1);
            cx.tally(g, "effects.moved_canceled", 1);
            return Ok(());
        }
    } else if kind != TROOP && g.tokens(to, ADV_AMBASSADOR) > 0 {
        adversary::remove(g, cx, to, 1);
        cx.tally(g, "effects.moved_canceled", 1);
        return Ok(());
    }
    match (0..g.seats.len()).find(|&s| kind == ambassador(s)) {
        Some(s) => {
            let used = used || !board::arrive_ready(&cx.world);
            board::place_ambassador_as(g, cx, s, to, used);
        }
        None => {
            cx.add_tokens(g, to, kind, 1);
        }
    }
    Ok(())
}

/// r142: one piece of `kind` leaves hex `from` for a Gather or Push. Returns whether it is a player ambassador
/// that had used its action (a ready one leaves first: `board::take_ambassadors`).
fn leave(g: &mut GameState, cx: &mut Cx, kind: &'static str, from: usize) -> bool {
    match (0..g.seats.len()).find(|&s| kind == ambassador(s)) {
        Some(s) => {
            let used = ready_ambassadors(g, s, from) == 0;
            board::take_ambassadors(g, cx, s, from, 1);
            used
        }
        None => {
            cx.add_tokens(g, from, kind, -1);
            false
        }
    }
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
/// X", as often as they like until `max` (one answer may take a whole stack, so a search sees a gather whole).
/// A strike is no gather any more (178, Matt's ruling #27, reversing 169's D76): it removes up to `max` adversary
/// pieces on or next to the hex, the player choosing each batch ([`strike`]).
pub(super) fn gather(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64, pieces: &[Piece], strike: bool, zone: &[usize]) -> Res {
    let w = cx.world;
    let mut hexes = gather_hexes(&w, g, seat, pieces, strike);
    hexes.retain(|h| zone.contains(h));
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = if strike { STRIKE_AT } else { GATHER_INTO };
    let Some(i) = cx.choose(seat, || format!("Seat {seat}: {prompt}"), opts)? else {
        return Ok(());
    };
    let to = hexes[i];
    if strike {
        return self::strike(g, cx, seat, max, pieces, to);
    }
    let mut from = w.map.hexes[to].neighbors.to_vec();
    from.sort_unstable();
    let mut moved = 0;
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
            let used = leave(g, cx, k, x);
            cx.tally(g, "effects.gathered", 1);
            arrive(g, cx, k, to, used)?;
        }
        moved += n;
    }
    cx.note(Some(to), || format!("Seat {seat} gathers {} into {}", batch(moved, "piece"), hex_name(&w, to)));
    Ok(())
}

/// The strike (178, Matt's ruling #27): up to `max` adversary `pieces` on or next to hex `at` come off, the player
/// choosing batches ("2 troops on X", as gathers ask) until `max` or a stop.
fn strike(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64, pieces: &[Piece], at: usize) -> Res {
    let w = cx.world;
    let mut around = w.map.hexes[at].neighbors.to_vec();
    around.sort_unstable();
    around.insert(0, at);
    // 308: around a hex reached only through your ambassador (a troop beside it), agents come off only where your
    // nodes reach; troops anywhere around it.
    let troops_only = !in_reach(&w, g, seat, at);
    let mut removed = 0;
    while removed < max {
        let mut units = Vec::new();
        for &x in &around {
            for (k, name) in tokens_of(g, x, pieces) {
                if adversary_piece(k) && (!troops_only || k == TROOP || in_reach(&w, g, seat, x)) {
                    for n in sizes(g.tokens(x, k), max - removed) {
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
            .map(|(_, x, _, what)| Opt { option: format!("{what} on {}", hex_name(&w, *x)), target: Some(w.map.hexes[*x].hex) })
            .collect();
        opts.push(opt(STOP));
        let Some(j) = cx.choose(seat, || format!("Seat {seat}: {STRIKE_PIECE} (on or next to {})", hex_name(&w, at)), opts)? else { break };
        let Some(&(k, x, n, _)) = units.get(j) else { break };
        for _ in 0..n {
            adversary::remove_piece(g, cx, Some(seat), x, k);
            cx.tally(g, "effects.struck", 1);
        }
        removed += n;
    }
    cx.note(Some(at), || format!("Seat {seat} removes {} on or next to {}", batch(removed, "piece"), hex_name(&w, at)));
    Ok(())
}

/// Push up to `max` `pieces` from one hex in reach, each to a hex next to it (166): the player picks the hex, then
/// batches, "2 agents to Y", as often as they like until `max`, so pieces may go to different hexes.
pub(super) fn push(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64, pieces: &[Piece], zone: &[usize]) -> Res {
    let w = cx.world;
    let mut hexes = push_hexes(&w, g, seat, pieces);
    hexes.retain(|h| zone.contains(h));
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
        for (k, name) in pushable(&w, g, seat, from, pieces) {
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
            let used = leave(g, cx, k, from);
            cx.tally(g, "effects.pushed", 1);
            arrive(g, cx, k, x, used)?;
        }
        moved += n;
    }
    cx.note(Some(from), || format!("Seat {seat} pushes {} from {}", batch(moved, "piece"), hex_name(&w, from)));
    Ok(())
}
