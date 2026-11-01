//! Trades, tokens and what a player's effects build or harden.

use super::*;

pub(super) fn transfer(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, seat: usize, resource: Resource, max: i64) -> Res {
        if let Some(o) = ctx.other {
            let most = max.min(g.resource(seat, resource)).max(0);
            let opts = (0..=most)
                .map(|k| opt(format!("give {}", resource.amount(k))))
                .collect();
            let prompt = format!("Seat {seat}: give how much {resource} to seat {o}?");
            let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
            if k > 0 {
                cx.add_resource(g, seat, resource, -k);
                cx.add_resource(g, o, resource, k);
            }
        }
    Ok(())
}

pub(super) fn gain_per_tokens(g: &mut GameState, cx: &mut Cx, seat: usize, resource: Resource, kind: TokenKind, per: i64) {
    let w = cx.world;
        let held: i64 = cards::token_cards(&w, g, seat, Some(kind))
            .iter()
            .map(|c| g.var(&cards::tokens_key(seat, c)))
            .sum();
        cx.add_resource(g, seat, resource, held / per.max(1));
}

pub(super) fn cash_out(g: &mut GameState, cx: &mut Cx, seat: usize, kind: TokenKind, resource: Resource, rate: i64) {
    let w = cx.world;
        let mut n = 0;
        for c in cards::token_cards(&w, g, seat, Some(kind)) {
            let k = g.var(&cards::tokens_key(seat, &c));
            cards::add_tokens(g, cx, seat, &c, -k);
            n += k;
        }
        cx.add_resource(g, seat, resource, n * rate);
}

pub(super) fn build_factory(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
        let hexes = factory_sites(&w, g, seat);
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {seat}: build a factory on which zone?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            cx.add_tokens(g, hexes[i], FACTORY, 1);
            cx.add_tokens(g, hexes[i], &cube(seat), 1);
            cx.tally(g, "board.factories_built.player", 1);
        }
    Ok(())
}

pub(super) fn harden(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64) -> Res {
    let w = cx.world;
        // 022: only facilities a token can protect, each at most once ("harden 2
        // facilities" is two facilities; fewer if fewer exist).
        let mut hexes = harden_hexes(&w, g, seat);
        for k in 0..count {
            let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = format!("Seat {seat}: hardening {} of {count} on which facility?", k + 1);
            let Some(j) = cx.choose(seat, &prompt, opts)? else {
                break;
            };
            cx.add_tokens(g, hexes.remove(j), HARDEN, 1);
        }
    Ok(())
}
