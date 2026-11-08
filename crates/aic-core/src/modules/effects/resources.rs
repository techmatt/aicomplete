//! Trades, tokens and what a player's effects build or harden.

use super::*;

pub(super) fn transfer(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, seat: usize, resource: Resource, max: i64) -> Res {
    if let Some(o) = ctx.other {
        let most = max.min(g.resource(seat, resource)).max(0);
        let opts = (0..=most)
            .map(|k| opt(format!("give {}", resource.amount(k))))
            .collect();
        let prompt = || format!("Seat {seat}: give how much {resource} to seat {o}?");
        let k = cx.choose(seat, prompt, opts)?.unwrap_or(0) as i64;
        if k > 0 {
            cx.add_resource(g, seat, resource, -k);
            cx.add_resource(g, o, resource, k);
            if resource == Resource::Insight && o != seat {
                cx.add_key(g, &economy::insight_received(o), k);
            }
        }
    }
    Ok(())
}

/// The one card a token count reads (076, Matt's ruling 12): with `here`, the
/// building whose Action this is; else one of seat `seat`'s `kind` cards
/// holding at least `least`, the player's pick (no pick when only one).
pub(super) fn token_card(
    g: &mut GameState,
    cx: &mut Cx,
    ctx: &Ctx,
    seat: usize,
    kind: TokenKind,
    here: bool,
    least: i64,
) -> Res<Option<Key>> {
    if here {
        return Ok(ctx.card);
    }
    let w = cx.world;
    let ids: Vec<Key> = cards::token_cards(&w, g, seat, Some(kind))
        .into_iter()
        .filter(|c| g.var(&cards::tokens_key(seat, c)) >= least.max(1))
        .collect();
    let opts = ids.iter().map(|c| opt(*c)).collect();
    let prompt = || format!("Seat {seat}: count the {kind} on which card?");
    Ok(cx.choose(seat, prompt, opts)?.map(|i| ids[i]))
}

pub(super) fn build_factory(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let hexes = factory_sites(&w, g, seat);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: build a factory on which zone?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        cx.add_tokens(g, hexes[i], FACTORY, 1);
        cx.add_tokens(g, hexes[i], cube(seat), 1);
        cx.tally(g, "board.factories_built.player", 1);
    }
    Ok(())
}

/// Builds a research station of `seat`'s in reach, paying no chips (075).
pub(super) fn build_station(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let hexes = board::station_sites(&w, g, seat);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: build a research station on which hex?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        cx.add_tokens(g, hexes[i], STATION, 1);
        cx.add_tokens(g, hexes[i], cube(seat), 1);
        // 102 (D40): was `board.stations_free`, which read as `stations_freed`.
        cx.tally(g, "board.stations_built_free", 1);
    }
    Ok(())
}

/// Claim a neutral research station in reach with a card (142).
pub(super) fn claim_station(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let hexes = board::card_claim_stations(&w, g, seat);
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("Seat {seat}: claim the research station on which hex?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        // 148 (D70): as the Claim action, one ambassador more for each adversary ambassador there (they cancel).
        let extra = board::card_claim_extra(&w, g, hexes[i]).unwrap_or(0);
        if extra > 0 {
            cx.add_resource(g, seat, Resource::Ambassadors, -extra);
        }
        board::claim_site(g, cx, seat, hexes[i], "card");
    }
    Ok(())
}

/// `count` research cubes in `seat`'s color onto an incomplete megaproject (075).
pub(super) fn project_cube(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64, station: bool) -> Res {
    let w = cx.world;
    let cs = cube_projects(&w, g, seat, station);
    let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
    let prompt = || format!("Seat {seat}: add {} to which country's megaproject?", Resource::ResearchCubes.amount(count));
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        cx.add_key(g, &board::project_key(cs[i], Some(seat)), count);
        cx.tally(g, "board.cubes_bought", count);
        cx.per_round(g, "board.cubes_bought", count);
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
        let prompt = || format!("Seat {seat}: hardening {} of {count} on which facility?", k + 1);
        let Some(j) = cx.choose(seat, prompt, opts)? else {
            break;
        };
        cx.add_tokens(g, hexes.remove(j), HARDEN, 1);
    }
    Ok(())
}
