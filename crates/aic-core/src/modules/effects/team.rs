//! Threats and the team: the wordings differ, the rule does not.

use super::*;
use crate::modules::plan;
use crate::op::{CorruptSite, Per, Removable, Where};
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
        // 185 (D77): a firewalled or prohibited station is skipped.
        spots.retain(|&h| !adversary::guard_holds(g, cx, h, "flip"));
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

/// 185 (Matt's ruling #3): `count` adversary research cubes onto each open megaproject.
pub(super) fn adversary_cubes(g: &mut GameState, cx: &mut Cx, count: i64) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        if g.var(&board::project_done(c)) == 0 {
            cx.add_key(g, &board::project_key(c, None), count);
            cx.tally(g, "board.adversary_cubes_op", count);
        }
    }
}

/// 185 (Matt's ruling #3): up to `count` botnets in countries with an open megaproject (players choose which) each
/// come off and become an adversary cube on that megaproject. Not a removal by the players: no Unity for it.
pub(super) fn botnet_cubes(g: &mut GameState, cx: &mut Cx, count: i64) -> Res {
    let w = cx.world;
    for _ in 0..count {
        let open = |h: usize| g.var(&board::project_done(w.map.hexes[h].country as usize)) == 0;
        let botnets: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, BOTNET) > 0 && open(h)).collect();
        let opts = botnets.iter().map(|&h| hex_opt(&w, h)).collect();
        let Some(i) = cx.choose(0, "Which botnet becomes an adversary research cube?", opts)? else {
            break;
        };
        let h = botnets[i];
        cx.add_tokens(g, h, BOTNET, -1);
        cx.set_key(g, &adversary::cut_key(h), 0);
        cx.add_key(g, &board::project_key(w.map.hexes[h].country as usize, None), 1);
        cx.tally(g, "board.botnet_cubes", 1);
    }
    Ok(())
}

/// 195: the hexes of country `c`, lowest-numbered first (a project's tie-break).
fn by_number(w: &World, c: usize) -> Vec<usize> {
    let mut hexes = w.map.countries[c].hexes.clone();
    hexes.sort_by_key(|&h| w.map.hexes[h].number);
    hexes
}

/// 195: agents on hex `h` and the hexes next to it (a project's "most agents on or next to it").
fn agents_around(w: &World, g: &GameState, h: usize) -> i64 {
    within(w, h, 1).into_iter().map(|x| g.tokens(x, ADV_AMBASSADOR)).sum()
}

/// 195: the hex of `hexes` with the most agents on or next to it; ties: the first (lowest-numbered, as
/// [`by_number`] orders them).
fn most_agents_around(w: &World, g: &GameState, hexes: &[usize]) -> Option<usize> {
    let best = hexes.iter().map(|&h| agents_around(w, g, h)).max()?;
    hexes.iter().copied().find(|&h| agents_around(w, g, h) == best)
}

/// 195 (`BotnetTroops`): in each country, up to `count` botnets, lowest-numbered first, become `troops` troops each.
/// They are made on the botnet's own hex, as a factory's output is: they enter no hex, so Wall doesn't stop them.
pub(super) fn botnet_troops(g: &mut GameState, cx: &mut Cx, count: i64, troops: i64) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        let botnets: Vec<usize> = by_number(&w, c).into_iter().filter(|&h| g.tokens(h, BOTNET) > 0).collect();
        for &h in botnets.iter().take(count.max(0) as usize) {
            cx.add_tokens(g, h, BOTNET, -1);
            cx.set_key(g, &adversary::cut_key(h), 0);
            for _ in 0..troops {
                adversary::place_troop(g, cx, h, "project");
            }
            cx.tally(g, "adversary.botnet_troops", troops);
            cx.tally(g, "adversary.project_troops", troops);
            cx.note(Some(h), || format!("The botnet at {} becomes {troops} adversary troops", hex_name(&w, h)));
        }
    }
}

/// 195 (`SpreadBotnets`): in each country, `count` new botnets, each on the lowest-numbered site (city, research
/// station or factory) next to one of its botnets that holds none; with no botnet in the country, on its
/// lowest-numbered research station without one.
pub(super) fn spread_botnets(g: &mut GameState, cx: &mut Cx, count: i64) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        let hexes = by_number(&w, c);
        for _ in 0..count {
            let site = |h: usize| is_city(&w, h) || g.tokens(h, STATION) > 0 || g.tokens(h, FACTORY) > 0;
            let has = hexes.iter().any(|&h| g.tokens(h, BOTNET) > 0);
            let spot = if has {
                hexes.iter().copied().find(|&h| {
                    site(h)
                        && g.tokens(h, BOTNET) == 0
                        && w.map.hexes[h].neighbors.iter().any(|&n| w.map.hexes[n].country as usize == c && g.tokens(n, BOTNET) > 0)
                })
            } else {
                hexes.iter().copied().find(|&h| g.tokens(h, STATION) > 0 && g.tokens(h, BOTNET) == 0)
            };
            let Some(h) = spot else {
                break;
            };
            cx.add_tokens(g, h, BOTNET, 1);
            cx.tally(g, "adversary.botnets_made", 1);
            cx.tally(g, "adversary.botnets_spread", 1);
            cx.peak(g, "adversary.botnets_peak", adversary::botnet_count(g, None));
            cx.note(Some(h), || format!("A botnet spreads to {}", hex_name(&w, h)));
        }
    }
}

/// 195 (`TakeSite`): in each country, the neutral `site` with the most agents on or next to it corrupts; with none
/// neutral, the player-owned one picked the same way turns neutral. A guarded site (firewall, no-Corrupt token) is skipped,
/// and a guarded neutral one still counts as neutral (D88).
pub(super) fn take_site(g: &mut GameState, cx: &mut Cx, site: Facility) -> Res {
    let w = cx.world;
    let kind = site.token();
    for c in 0..w.map.countries.len() {
        let hexes: Vec<usize> = by_number(&w, c).into_iter().filter(|&h| g.tokens(h, kind) > 0 && !corrupted(g, h)).collect();
        let guarded: Vec<usize> = hexes.iter().copied().filter(|&h| adversary::guard_holds(g, cx, h, "take_site")).collect();
        // 199 (D88, Matt 2026-11-12): a guarded neutral site still counts as neutral, so with any neutral site in
        // the country no owned one turns neutral; with every neutral one guarded, nothing happens there.
        let any_neutral = hexes.iter().any(|&h| cube_owner(g, h).is_none());
        let open: Vec<usize> = hexes.into_iter().filter(|h| !guarded.contains(h)).collect();
        let neutral: Vec<usize> = open.iter().copied().filter(|&h| cube_owner(g, h).is_none()).collect();
        if let Some(h) = most_agents_around(&w, g, &neutral) {
            adversary::corrupt_hex(g, cx, h, if site == Facility::Station { CorruptSite::Station } else { CorruptSite::Factory })?;
            cx.tally(g, "adversary.sites_taken", 1);
            cx.note(Some(h), || format!("The neutral {} at {} goes to the adversary", piece_name(kind), hex_name(&w, h)));
            continue;
        }
        if any_neutral {
            cx.tally(g, "adversary.sites_guarded", 1);
            continue;
        }
        let owned: Vec<usize> = open.into_iter().filter(|&h| cube_owner(g, h).is_some()).collect();
        if let Some(h) = most_agents_around(&w, g, &owned) {
            if let Some(s) = cube_owner(g, h) {
                cx.add_tokens(g, h, cube(s), -1);
            }
            cx.tally(g, "adversary.sites_neutralized", 1);
            cx.note(Some(h), || format!("The {} at {} turns neutral", piece_name(kind), hex_name(&w, h)));
        }
    }
    Ok(())
}

/// 195 (`FactoryTroops`): every adversary factory makes `count` troops on its hex; a player ambassador on it stops
/// it (as its own output).
pub(super) fn factory_troops(g: &mut GameState, cx: &mut Cx, count: i64) {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 || !corrupted(g, h) {
            continue;
        }
        if player_ambassador(g, h).is_some() {
            cx.tally(g, "adversary.factory_troops_blocked", 1);
            continue;
        }
        for _ in 0..count {
            adversary::place_troop(g, cx, h, "project");
        }
        cx.tally(g, "adversary.factory_troops", count);
        cx.tally(g, "adversary.project_troops", count);
        // 197: "makes", not "musters": no Muster rule (surcharge, Wall, no-Muster token) applies here.
        cx.note(Some(h), || format!("The adversary factory at {} makes {count} troop", hex_name(&w, h)));
    }
}

/// 195 (`RaidNode`): in each country, the node hex holding the most agents (at least one; ties: the lowest-numbered)
/// loses its node; its protections apply.
pub(super) fn raid_node(g: &mut GameState, cx: &mut Cx, ctx: &Ctx) -> Res {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        let hexes: Vec<usize> = by_number(&w, c).into_iter().filter(|&h| node_owner(g, h).is_some()).collect();
        let best = hexes.iter().map(|&h| g.tokens(h, ADV_AMBASSADOR)).max().unwrap_or(0);
        if best < 1 {
            continue;
        }
        let Some(h) = hexes.iter().copied().find(|&h| g.tokens(h, ADV_AMBASSADOR) == best) else {
            continue;
        };
        let Some(s) = node_owner(g, h) else {
            continue;
        };
        cx.tally(g, "adversary.node_raids", 1);
        board::destroy_piece(g, cx, h, node(s), board::Cause::Effect(ctx.label))?;
        if g.is_over() {
            break;
        }
    }
    Ok(())
}

/// 195 (`StripHarden`): in each country, the hardened facility with the most agents on or next to it loses its
/// hardening tokens.
pub(super) fn strip_harden(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        // A facility's hardening: a Science card's node shield on a hex with no facility is no target (195 review).
        let hexes: Vec<usize> =
            by_number(&w, c).into_iter().filter(|&h| g.tokens(h, HARDEN) > 0 && board::hardenable(g, h)).collect();
        if let Some(h) = most_agents_around(&w, g, &hexes) {
            let n = g.tokens(h, HARDEN);
            cx.add_tokens(g, h, HARDEN, -n);
            cx.tally(g, "adversary.hardening_stripped", n);
            cx.note(Some(h), || format!("The hardening at {} is stripped", hex_name(&w, h)));
        }
    }
}

/// 195 (`Clock`): the top `count` cards of the face-down event deck are discarded (face up, into the event discard).
pub(super) fn clock(g: &mut GameState, cx: &mut Cx, count: i64) {
    for _ in 0..count {
        let Some(id) = g.pile(adversary::DECK).first().cloned() else {
            break;
        };
        // 197: onto the discard's top, as every flip discards (the viewer's "last:" reads the top).
        cx.move_card(g, &id, adversary::DECK, adversary::DISCARD, true).expect("top of the event deck");
        cx.tally(g, "adversary.clock_cards", 1);
        cx.note(None, || "The adversary's clock runs: the top event card is discarded".to_string());
    }
}

/// 167: whether hex `h` holds a piece of `pieces` a board-conditioned removal can take.
fn removable(w: &World, g: &GameState, h: usize, pieces: Removable) -> bool {
    match pieces {
        Removable::Agents => g.tokens(h, ADV_AMBASSADOR) > 0,
        Removable::Troops => g.tokens(h, TROOP) > 0,
        Removable::Botnets => g.tokens(h, BOTNET) > 0,
        Removable::Fallout => g.tokens(h, FALLOUT) > 0,
        Removable::Cubes => corrupted(g, h) && !is_city(w, h),
    }
}

/// 167: up to `n` of `pieces` come off hex `h`; returns how many.
fn take(g: &mut GameState, cx: &mut Cx, h: usize, pieces: Removable, n: i64) -> i64 {
    let w = cx.world;
    let mut k = 0;
    while k < n && removable(&w, g, h, pieces) {
        match pieces {
            Removable::Agents => adversary::remove_piece(g, cx, None, h, ADV_AMBASSADOR),
            Removable::Troops => adversary::remove_piece(g, cx, None, h, TROOP),
            Removable::Botnets => adversary::remove_botnet(g, cx, h),
            Removable::Fallout => {
                adversary::lift_fallout(g, cx, h);
                cx.tally(g, "effects.land_lifted", 1);
            }
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
                removed += take(g, cx, h, pieces, count);
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
                    removed += take(g, cx, hexes[i], pieces, 1);
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
                removed += take(g, cx, hexes[i], pieces, count);
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
