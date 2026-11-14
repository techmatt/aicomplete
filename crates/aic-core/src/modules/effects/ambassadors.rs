//! Moving a player's ambassadors.

use super::*;

/// Moves one of seat `s`'s ambassadors from hex `from` onto hex `to` (021:
/// movement keywords); it arrives as in docs/rules/ambassadors.md.
fn walk(g: &mut GameState, cx: &mut Cx, seat: usize, from: usize, to: usize) {
    cx.add_tokens(g, from, ambassador(seat), -1);
    board::place_ambassador(g, cx, seat, to);
    cx.tally(g, "effects.ambassador_moves", 1);
    cx.tally(g, &crate::key::KeyBuf::new("board.amb_moves.").num(seat), 1);
}

/// 236 (Matt, 2026-11-14): with a target, the moved ambassadors come from your supply or from hexes in `sources`
/// (on or next to the target, as Summit Caucus reads), never from anywhere.
pub(super) fn move_ambassadors(
    g: &mut GameState,
    cx: &mut Cx,
    seat: usize,
    max: i64,
    zone: &[usize],
    sources: Option<&[usize]>,
) -> Res {
    let w = cx.world;
    for k in 0..max {
        let mut mine = ambassador_hexes(g, seat);
        mine.retain(|h| sources.is_none_or(|s| s.contains(h)));
        let supply = g.resource(seat, Resource::Ambassadors) > 0;
        if !supply && mine.is_empty() {
            break;
        }
        // 118 (D8): a piece is a destination only if an ambassador can come to it: from the
        // supply, or from another hex (the hex holding the seat's only one moved nothing).
        let dest: Vec<usize> = zone
            .iter()
            .copied()
            .filter(|&d| has_piece(&w, g, d) && (supply || mine.iter().any(|&a| a != d)))
            .collect();
        if dest.is_empty() {
            break;
        }
        let mut opts: Vec<_> = dest.iter().map(|&h| hex_opt(&w, h)).collect();
        opts.push(opt("stop"));
        let prompt = || format!(
            "Seat {seat}: move ambassador {} of up to {max} to which piece?",
            k + 1
        );
        let Some(i) = cx.choose(seat, prompt, opts)? else {
            break;
        };
        if i >= dest.len() {
            break;
        }
        board::move_ambassador_from(g, cx, seat, dest[i], sources)?;
    }
    Ok(())
}

/// 234 (Matt: Place means onto the map): `count` new ambassadors of seat `seat`'s onto pieces in `zone` (the
/// target, or on or next to it), one at a time, each arriving as any ambassador does (`board::place_ambassador`:
/// onto agents it cancels one).
pub(super) fn place_ambassadors(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64, zone: &[usize]) -> Res {
    let w = cx.world;
    let dest: Vec<usize> = zone.iter().copied().filter(|&h| has_piece(&w, g, h)).collect();
    for k in 0..count {
        let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("Seat {seat}: place ambassador {} of {count} on which piece?", k + 1);
        let Some(i) = cx.choose(seat, prompt, opts)? else {
            break;
        };
        board::place_ambassador(g, cx, seat, dest[i]);
        cx.tally(g, "effects.ambassadors_placed", 1);
    }
    Ok(())
}

pub(super) fn move_one(g: &mut GameState, cx: &mut Cx, seat: usize, range: i64) -> Res {
    let w = cx.world;
    let from = movable_one(&w, g, seat, range);
    let mut opts: Vec<_> = from.iter().map(|&h| hex_opt(&w, h)).collect();
    opts.push(opt("stop"));
    let prompt = || format!("Seat {seat}: move which of your ambassadors (up to {range} hexes)?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        if let Some(&a) = from.get(i) {
            let dest: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| move_one_dest(&w, g, a, range, h))
                .collect();
            let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = || format!("Seat {seat}: move it to which piece?");
            if let Some(j) = cx.choose(seat, prompt, opts)? {
                walk(g, cx, seat, a, dest[j]);
            }
        }
    }
    Ok(())
}

pub(super) fn spread_ambassadors(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    // Each ambassador on the board when it starts, in hex order.
    let mut each: Vec<usize> = Vec::new();
    for h in ambassador_hexes(g, seat) {
        for _ in 0..g.tokens(h, ambassador(seat)) {
            each.push(h);
        }
    }
    for a in each {
        let dest: Vec<usize> = w.map.hexes[a]
            .neighbors
            .iter()
            .copied()
            .filter(|&h| has_piece(&w, g, h))
            .collect();
        let mut opts = vec![opt("stay")];
        opts.extend(dest.iter().map(|&h| hex_opt(&w, h)));
        let prompt = || format!("Seat {seat}: spread your ambassador at {}?", hex_name(&w, a));
        if let Some(j) = cx.choose(seat, prompt, opts)? {
            if j > 0 && g.tokens(a, ambassador(seat)) > 0 {
                walk(g, cx, seat, a, dest[j - 1]);
            }
        }
    }
    Ok(())
}

/// 234: with a target, the ambassadors on or next to it come onto it (`target` `Some(None)`: no target found,
/// nothing moves); with none, as before: any in reach onto one piece in reach.
pub(super) fn consolidate(g: &mut GameState, cx: &mut Cx, seat: usize, max: i64, target: Option<Option<usize>>) -> Res {
    let w = cx.world;
    let from_ok = |g: &GameState, h: usize| match target {
        Some(Some(t)) => in_area(&w, t, Area::Near, h),
        Some(None) => false,
        None => in_reach(&w, g, seat, h),
    };
    // 118: with no ambassador of yours in reach there is nothing to bring (Summit
    // Caucus still gains its ambassadors): no piece to pick.
    let mine: Vec<usize> = ambassador_hexes(g, seat).into_iter().filter(|&h| from_ok(g, h)).collect();
    // A piece is a destination only if some ambassador in reach stands elsewhere: the hex
    // holding all of them moved nothing (D8's pattern).
    let dest: Vec<usize> = match target {
        Some(t) => t.into_iter().filter(|&d| has_piece(&w, g, d) && mine.iter().any(|&a| a != d)).collect(),
        None => pieces_in_reach(&w, g, seat).into_iter().filter(|d| mine.iter().any(|a| a != d)).collect(),
    };
    if dest.is_empty() {
        return Ok(());
    }
    let opts = dest.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: consolidate up to {max} ambassadors onto which piece?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        let to = dest[i];
        for _ in 0..max {
            let from: Vec<usize> = ambassador_hexes(g, seat)
                .into_iter()
                .filter(|&h| h != to && from_ok(g, h))
                .collect();
            if from.is_empty() {
                break;
            }
            let mut opts: Vec<_> = from.iter().map(|&h| hex_opt(&w, h)).collect();
            opts.push(opt("stop"));
            let prompt = || format!("Seat {seat}: bring which ambassador to {}?", hex_name(&w, to));
            let Some(j) = cx.choose(seat, prompt, opts)? else {
                break;
            };
            let Some(&a) = from.get(j) else {
                break;
            };
            walk(g, cx, seat, a, to);
        }
    }
    Ok(())
}
