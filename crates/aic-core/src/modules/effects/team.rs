//! Threats and the team: the wordings differ, the rule does not.

use super::*;
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
    // 151: an event's, a project's or the board's troops land deterministically (the lowest-numbered).
    let lowest = ctx.threat.is_none() && matches!(ctx.label, "standing" | "project" | "board");
    for hexes in regions(&w, g, ctx, each_country) {
        let kills = destroy.then_some(ctx.label);
        adversary::land_troops(g, cx, &hexes, count, prompt, tally, kills, lowest)?;
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
        if ctx.threat.is_none() && matches!(ctx.label, "standing" | "project" | "board") {
            if let Some(&h) = spots.iter().min_by_key(|&&h| w.map.hexes[h].number) {
                adversary::corrupt_station(g, cx, h);
            }
            continue;
        }
        let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
        if let Some(i) = cx.choose(0, prompt, opts)? {
            adversary::corrupt_station(g, cx, spots[i]);
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
        for _ in 0..count {
            // 135: a threat never takes a last node.
            if ctx.threat.is_some() && node_count(g, s) <= 1 {
                break;
            }
            board::destroy_own_node(g, cx, s, &hexes, board::Cause::Effect(ctx.label), ctx.why.as_deref())?;
        }
    }
    Ok(())
}

/// 146 (`DestroyNode`): the player a threat hits destroys `count` of their
/// nodes closest to it, wherever they are; never their last node.
pub(super) fn destroy_nearest_node(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64) -> Res {
    let w = cx.world;
    let Some(t) = ctx.threat else {
        return Ok(());
    };
    for s in ctx.seats(&w, g) {
        for _ in 0..count {
            if node_count(g, s) <= 1 {
                break;
            }
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
