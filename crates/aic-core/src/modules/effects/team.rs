//! Threats and the team: the wordings differ, the rule does not.

use super::*;

pub(super) fn troops(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64, each_country: bool, destroy: bool) -> Res {
    let w = cx.world;
        let prompt = ctx.says(
            "Threat: an adversary troop lands on which hex?",
            "An adversary troop lands on which hex?",
        );
        let tally = ctx.says("threats.troops", "adversary.project_troops");
        for hexes in ctx.regions(&w, each_country) {
            let kills = destroy.then_some(ctx.label);
            adversary::land_troops(g, cx, &hexes, count, prompt, tally, kills)?;
        }
    Ok(())
}

pub(super) fn flip_station(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, each_country: bool) -> Res {
    let w = cx.world;
        let prompt = ctx.says(
            "Threat: which research station flips to the adversary?",
            "Which research station flips to the adversary?",
        );
        for hexes in ctx.regions(&w, each_country) {
            let spots: Vec<usize> = hexes
                .into_iter()
                .filter(|&h| g.tokens(h, STATION) > 0 && !corrupted(g, h))
                .collect();
            let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
            if let Some(i) = cx.choose(0, prompt, opts)? {
                adversary::corrupt_station(g, cx, spots[i]);
            }
        }
    Ok(())
}

pub(super) fn destroy_presence(g: &mut GameState, cx: &mut Cx, ctx: &Ctx) {
    let w = cx.world;
        let threat = |g: &GameState, h: usize| g.tokens(h, ADV_AMBASSADOR) + g.tokens(h, TROOP);
        // `max_by_key` takes the last of equals: walk from the top.
        let top = scope_hexes(&w, ctx.scope)
            .into_iter()
            .rev()
            .filter(|&h| presence_owner(g, h).is_some())
            .max_by_key(|&h| threat(g, h));
        if let Some(h) = top {
            for s in 0..g.seats.len() {
                if g.tokens(h, &presence(s)) > 0 {
                    board::destroy_piece(g, cx, h, &presence(s), board::Cause::Effect(ctx.label));
                }
            }
        }
}

pub(super) fn each_destroy_presence(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64) -> Res {
    let w = cx.world;
        let hexes = scope_hexes(&w, ctx.scope);
        for s in ctx.seats(&w, g) {
            for _ in 0..count {
                board::destroy_own_presence(g, cx, s, &hexes, board::Cause::Effect(ctx.label))?;
            }
        }
    Ok(())
}

pub(super) fn node_ambassadors(g: &mut GameState, cx: &mut Cx, amount: i64) {
        let nodes: Vec<(usize, i64)> = (0..g.hexes.len())
            .map(|h| (h, g.tokens(h, NODE)))
            .filter(|&(_, n)| n > 0)
            .collect();
        for (h, n) in nodes {
            adversary::place(g, cx, h, n * amount);
        }
}

pub(super) fn clean_city_ambassadors(g: &mut GameState, cx: &mut Cx, amount: i64) {
    let w = cx.world;
        let cities: Vec<usize> = (0..g.hexes.len())
            .filter(|&h| clean_city(&w, g, h))
            .collect();
        for h in cities {
            adversary::place(g, cx, h, amount);
        }
}

pub(super) fn remove_nodes(g: &mut GameState, cx: &mut Cx, count: i64) -> Res {
    let w = cx.world;
        for _ in 0..count {
            let nodes: Vec<usize> = (0..g.hexes.len())
                .filter(|&h| g.tokens(h, NODE) > 0 && !threats::shielded(&w, g, h, ThreatSource::Cyber))
                .collect();
            let opts = nodes.iter().map(|&h| hex_opt(&w, h)).collect();
            let Some(i) = cx.choose(0, "Remove the node in which hex?", opts)? else {
                break;
            };
            adversary::remove_node(g, cx, nodes[i]);
        }
    Ok(())
}
