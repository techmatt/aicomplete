//! Threats and the team: the wordings differ, the rule does not.

use super::*;
use crate::modules::plan;
use crate::op::{Per, Removable, Where};
use crate::vocab::Location;

/// The hexes an op acts in (135): near a resolving threat's tokens, else
/// [`Ctx::regions`].
pub(super) fn regions(w: &World, g: &GameState, ctx: &Ctx, each_country: bool) -> Vec<Vec<usize>> {
    match &ctx.threat {
        Some(_) if !each_country => vec![ctx.hexes(w, g)],
        _ => ctx.regions(w, each_country),
    }
}

pub(super) fn troops(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64, each_country: bool, destroy: bool) -> Res {
    let w = cx.world;
    let base = ctx.says(
        "Threat: an adversary troop lands on which hex?",
        "An adversary troop lands on which hex?",
    );
    // 118: named for what lands it (a threat's, a Discord card's).
    let named = ctx.why.as_deref().map(|n| format!("{n}: an adversary troop lands on which hex?"));
    let prompt = named.as_deref().unwrap_or(base);
    let tally = ctx.says("threats.troops", "adversary.project_troops");
    // 151: an event's or a project's troops land deterministically (the lowest-numbered; the board's Protector
    // landing does too, in `adversary::wall_pressure`).
    // 175: each of those is a Muster: a Muster prohibition stops it and the level surcharge adds to it.
    let muster = ctx.threat.is_none() && matches!(ctx.label, "standing" | "project");
    let count = if muster { count + adversary::muster_bonus(&w) } else { count };
    for hexes in regions(&w, g, ctx, each_country) {
        let kills = destroy.then_some(ctx.label);
        adversary::land_troops(g, cx, &hexes, count, prompt, tally, kills, muster)?;
    }
    Ok(())
}

pub(super) fn flip_station(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, each_country: bool) -> Res {
    let w = cx.world;
    let base = ctx.says(
        "Threat: which research station flips to the adversary?",
        "Which research station flips to the adversary?",
    );
    let named = ctx.why.as_deref().map(|n| format!("{n}: which research station flips to the adversary?"));
    let prompt = named.as_deref().unwrap_or(base);
    for hexes in regions(&w, g, ctx, each_country) {
        let mut spots: Vec<usize> = hexes
            .into_iter()
            .filter(|&h| g.tokens(h, STATION) > 0 && !corrupted(g, h))
            .collect();
        // 135: "the station here": the threat's station token; 146 (D63): nothing once it is corrupted.
        if let Some(t) = ctx.threat.as_ref() {
            let here = threats::token_of(&w, g, t, Location::Station);
            spots.retain(|h| Some(*h) == here);
        }
        // 151: a project's or an event's flip takes the lowest-numbered station.
        if ctx.threat.is_none() && matches!(ctx.label, "standing" | "project") {
            if let Some(&h) = spots.iter().min_by_key(|&&h| w.map.hexes[h].number) {
                adversary::corrupt_station(g, cx, h)?;
            }
            continue;
        }
        let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
        if let Some(i) = cx.choose(0, prompt, opts)? {
            adversary::corrupt_station(g, cx, spots[i])?;
        }
    }
    Ok(())
}

/// 139 (Offensive): a `tier` threat dealt with its first token on the most threatened node hex in scope.
pub(super) fn threat_at_node(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, tier: u8) -> Res {
    let w = cx.world;
    let threat = |g: &GameState, h: usize| g.tokens(h, ADV_AMBASSADOR) + g.tokens(h, TROOP);
    // `max_by_key` takes the last of equals: walk from the top.
    let top = ctx.hexes(&w, g)
        .into_iter()
        .rev()
        .filter(|&h| node_owner(g, h).is_some())
        .max_by_key(|&h| threat(g, h));
    if let Some(h) = top {
        threats::deal_at(g, cx, tier, h)?;
    }
    Ok(())
}

pub(super) fn each_destroy_node(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64, bases: bool) -> Res {
    let w = cx.world;
    let mut hexes = ctx.hexes(&w, g);
    // 139 (Border War): only nodes on or next to a front's base.
    if bases {
        hexes.retain(|&h| within(&w, h, 1).into_iter().any(|x| is_base(&w, x)));
    }
    for s in ctx.seats(&w, g) {
        // 175: any source takes a last node (135's threat floor is gone).
        for _ in 0..count {
            board::destroy_own_node(g, cx, s, &hexes, board::Cause::Effect(ctx.label), ctx.why.as_deref())?;
        }
    }
    Ok(())
}

/// 146 (`DestroyNode`): the player a threat hits destroys `count` of their
/// nodes closest to it, wherever they are (175: their last node too).
pub(super) fn destroy_nearest_node(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64) -> Res {
    let w = cx.world;
    let Some(t) = ctx.threat else {
        return Ok(());
    };
    for s in ctx.seats(&w, g) {
        for _ in 0..count {
            let hexes = threats::nearest_nodes(&w, g, s, &t);
            if hexes.is_empty() {
                break;
            }
            board::destroy_own_node(g, cx, s, &hexes, board::Cause::Effect(ctx.label), ctx.why.as_deref())?;
        }
    }
    Ok(())
}

pub(super) fn remove_botnets(g: &mut GameState, cx: &mut Cx, count: i64) -> Res {
    let w = cx.world;
    for _ in 0..count {
        let botnets: Vec<usize> = (0..g.hexes.len())
            .filter(|&h| g.tokens(h, BOTNET) > 0)
            .collect();
        let opts = botnets.iter().map(|&h| hex_opt(&w, h)).collect();
        let Some(i) = cx.choose(0, "Remove the botnet in which hex?", opts)? else {
            break;
        };
        adversary::remove_botnet(g, cx, botnets[i]);
    }
    Ok(())
}

/// 167: whether hex `h` holds a piece of `pieces` a board-conditioned removal can take.
fn removable(w: &World, g: &GameState, h: usize, pieces: Removable) -> bool {
    match pieces {
        Removable::Agents => g.tokens(h, ADV_AMBASSADOR) > 0,
        Removable::Troops => g.tokens(h, TROOP) > 0,
        Removable::Botnets => g.tokens(h, BOTNET) > 0,
        Removable::Markers => g.tokens(h, MARKER) > 0,
        Removable::Cubes => corrupted(g, h) && !is_city(w, h),
    }
}

/// 167: up to `n` of `pieces` come off hex `h`; returns how many. A marker is cleared for its reward to `seat`.
fn take(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, pieces: Removable, n: i64) -> i64 {
    let w = cx.world;
    let mut k = 0;
    while k < n && removable(&w, g, h, pieces) {
        match pieces {
            Removable::Agents => adversary::remove_piece(g, cx, None, h, ADV_AMBASSADOR),
            Removable::Troops => adversary::remove_piece(g, cx, None, h, TROOP),
            Removable::Botnets => adversary::remove_botnet(g, cx, h),
            Removable::Markers => board::clear_marker(g, cx, seat, h),
            Removable::Cubes => {
                cx.tally(g, "effects.cleansed", 1);
                cx.add_tokens(g, h, ADV_CUBE, -1);
            }
        }
        k += 1;
    }
    k
}

/// 167, a board-conditioned removal (fear-card shape): on each hex where `wh` holds, in each country piece from
/// such hexes (players choose, one piece at a time), or each player from one such hex (their choice).
pub(super) fn remove_where(g: &mut GameState, cx: &mut Cx, pieces: Removable, count: i64, per: Per, wh: Where) -> Res {
    let w = cx.world;
    let fits = |g: &GameState, h: usize| plan::holds(&w, g, h, wh) && removable(&w, g, h, pieces);
    let mut removed = 0;
    match per {
        Per::Hex => {
            let hexes: Vec<usize> = (0..g.hexes.len()).filter(|&h| fits(g, h)).collect();
            for h in hexes {
                let seat = player_ambassador(g, h).unwrap_or(0);
                removed += take(g, cx, seat, h, pieces, count);
            }
        }
        Per::Country => {
            for c in 0..w.map.countries.len() {
                for _ in 0..count {
                    let hexes: Vec<usize> = w.map.countries[c].hexes.iter().copied().filter(|&h| fits(g, h)).collect();
                    let i = match hexes.len() {
                        0 => break,
                        1 => 0,
                        _ => {
                            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                            let what = pieces.words();
                            let prompt = || format!("In {}: remove {what} from which hex?", country_name(&w, c));
                            match cx.choose(0, prompt, opts)? {
                                Some(i) => i,
                                None => break,
                            }
                        }
                    };
                    let seat = player_ambassador(g, hexes[i]).unwrap_or(0);
                    removed += take(g, cx, seat, hexes[i], pieces, 1);
                }
            }
        }
        Per::Player => {
            for s in 0..g.seats.len() {
                let hexes: Vec<usize> = (0..g.hexes.len()).filter(|&h| fits(g, h)).collect();
                let i = match hexes.len() {
                    0 => continue,
                    1 => 0,
                    _ => {
                        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                        let what = pieces.words();
                        let prompt = || format!("Seat {s}: remove {what} from which hex?");
                        match cx.choose(s, prompt, opts)? {
                            Some(i) => i,
                            None => continue,
                        }
                    }
                };
                removed += take(g, cx, s, hexes[i], pieces, count);
            }
        }
    }
    cx.tally(g, &format!("effects.remove_where.{}", pieces.words()), removed);
    Ok(())
}

/// 167: "for each `wh` (at most `max`)": how many times `then` resolves.
pub(super) fn per_site_count(w: &World, g: &GameState, wh: Where, max: i64) -> i64 {
    (plan::sites_where(w, g, wh).len() as i64).min(max)
}
