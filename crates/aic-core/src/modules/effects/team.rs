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
    // 259: the research track's troops are tallied apart from the projects'.
    let tally = if ctx.label == "research" { "adversary.research_troops" } else { ctx.says("threats.troops", "adversary.project_troops") };
    // r141: what placed it, for the by-round measure (`map.troops_placed.<source>`).
    let source = match (ctx.threat.is_some(), ctx.label) {
        (true, _) => "threat",
        (_, "standing") => "event",
        (_, "research") => "research",
        (_, "project") => "project",
        _ => "discord",
    };
    let count = if ctx.threat.is_none() { count + adversary::troop_bonus(&w) } else { count };
    for hexes in regions(&w, g, ctx, each_country) {
        let kills = destroy.then_some(ctx.label);
        adversary::land_troops(g, cx, &hexes, count, tally, kills, source)?;
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

/// 139 (Offensive): a `tier` threat dealt with its first token on the lowest-numbered node hex in scope holding an
/// agent or a troop (r144, Matt 2026-11-20, as Night Raid reads: the last "most" pick went; it was the node hex with
/// the most agents and troops). With none, no threat.
pub(super) fn threat_at_node(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, tier: u8) -> Res {
    let w = cx.world;
    let held = |g: &GameState, h: usize| g.tokens(h, ADV_AMBASSADOR) + g.tokens(h, TROOP) > 0;
    let top = ctx.hexes(&w, g)
        .into_iter()
        .filter(|&h| node_owner(g, h).is_some() && held(g, h))
        .min_by_key(|&h| (w.map.hexes[h].country, w.map.hexes[h].number));
    match top {
        Some(h) => threats::deal_at(g, cx, tier, h)?,
        None => cx.tally(g, "discord.offensive_idle", 1),
    }
    Ok(())
}

pub(super) fn each_destroy_node(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64, factories: bool) -> Res {
    let w = cx.world;
    let mut hexes = ctx.hexes(&w, g);
    // 292 (Border War; 139: a front's base): only nodes on or next to a factory.
    if factories {
        hexes.retain(|&h| within(&w, h, 1).into_iter().any(|x| g.tokens(x, FACTORY) > 0));
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


/// 195: the hexes of country `c`, lowest-numbered first (a project's tie-break).
fn by_number(w: &World, c: usize) -> Vec<usize> {
    let mut hexes = w.map.countries[c].hexes.clone();
    hexes.sort_by_key(|&h| w.map.hexes[h].number);
    hexes
}

/// 297 (a project's or a Discord card's troop line that names its place), in each country, `count` troops:
/// `TroopAt::CorruptedFactory`: on its lowest-numbered corrupted factory, and a country with none takes no troop;
/// `TroopAt::BesiegedCity`: on its lowest-numbered corrupted city with a player's node or ambassador on or next to
/// it, and a country with none garrisons as any troop does. Wall keeps one off the Protector's hex.
pub(super) fn troops_at(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, count: i64, at: crate::op::TroopAt) -> Res {
    use crate::op::TroopAt;
    let w = cx.world;
    let (tally, source) = if ctx.label == "project" { ("adversary.project_troops", "project") } else { ("adversary.discord_troops", "discord") };
    // As every troop line off a threat: the board's bonus, where one prints it (none does since 267).
    let count = count + adversary::troop_bonus(&w);
    for c in 0..w.map.countries.len() {
        let hexes = by_number(&w, c);
        let spot = match at {
            TroopAt::CorruptedFactory => hexes.iter().copied().find(|&h| g.tokens(h, FACTORY) > 0 && corrupted(g, h)),
            _ => {
                let by_player = |h: usize| next_to(&w, h, |x| player_ambassador(g, x).is_some() || node_owner(g, x).is_some());
                let besieged = hexes.iter().copied().find(|&h| corrupted_city(&w, g, h) && by_player(h));
                if besieged.is_some() {
                    cx.tally(g, "adversary.troops_besiege", 1);
                }
                besieged.or_else(|| board::garrison_spot(&w, g, &hexes))
            }
        };
        let Some(h) = spot else {
            cx.tally(g, "adversary.troops_no_factory", 1);
            continue;
        };
        for _ in 0..count {
            if adversary::place_troop(g, cx, h, source) {
                cx.per_round(g, adversary::OPS, 1);
                cx.tally(g, tally, 1);
            }
        }
    }
    Ok(())
}

/// 195 (`BotnetTroops`): in each country, up to `count` botnets, lowest-numbered first, become `troops` troops each.
/// 301 (Matt, 297/1): the botnet comes off and its troops garrison by the rule (the country's lowest-numbered
/// corrupted factory, else corrupted city, else city), never on the botnet's hex. Only in a country holding `least`
/// botnets or more. Wall (r98) keeps any off the Protector's hex.
pub(super) fn botnet_troops(g: &mut GameState, cx: &mut Cx, count: i64, troops: i64, least: i64) -> Res {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        let botnets: Vec<usize> = by_number(&w, c).into_iter().filter(|&h| g.tokens(h, BOTNET) > 0).collect();
        // 297: "each country holding N or more botnets": a country under it keeps its botnets.
        if (botnets.len() as i64) < least {
            cx.tally(g, "adversary.botnet_troops_short", 1);
            continue;
        }
        for &h in botnets.iter().take(count.max(0) as usize) {
            cx.add_tokens(g, h, BOTNET, -1);
            cx.set_key(g, &adversary::cut_key(h), 0);
            let Some(spot) = board::garrison_spot(&w, g, &by_number(&w, c)) else {
                continue;
            };
            let placed = (0..troops).filter(|_| adversary::place_troop_quiet(g, cx, spot, "project")).count() as i64;
            cx.per_round(g, adversary::OPS, placed);
            cx.tally(g, "adversary.project_troops", placed);
            // 235: what landed (Wall's are tallied as `troops_walled`).
            cx.tally(g, "adversary.botnet_troops", placed);
            cx.note(Some(spot), || {
                format!("The botnet at {} becomes {}, garrisoned at {}", hex_name(&w, h), troop_words(placed as usize, troops), hex_name(&w, spot))
            });
        }
    }
    Ok(())
}

/// 195 (`SpreadBotnets`): in each country, `count` new botnets, each on the lowest-numbered site (city, research
/// station or factory) next to one of its botnets that holds none; with no botnet in the country, on its
/// lowest-numbered research station without one.
pub(super) fn spread_botnets(g: &mut GameState, cx: &mut Cx, count: i64) {
    let w = cx.world;
    // 263: a difficulty step: below `botnet_spread_at` a "Botnets spread" line does nothing.
    if !adversary::level(&w, w.int("adversary.botnet_spread_at")) {
        cx.tally(g, "adversary.botnets_spread_skipped", 1);
        cx.note(None, || "Botnets don't spread at this difficulty".to_string());
        return;
    }
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

/// 195 (`TakeSite`): in each country, the lowest-numbered neutral `site` corrupts (297: the "most agents" pick
/// went, as Spill's and Surge's did at r140); with none neutral, the lowest-numbered player-owned one turns
/// neutral. A guarded site (firewall, no-Corrupt token) is skipped, and a guarded neutral one still counts as
/// neutral (D88).
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
        if let Some(&h) = neutral.first() {
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
        if let Some(&h) = owned.first() {
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
        let placed = (0..count).filter(|_| adversary::place_troop_quiet(g, cx, h, "project")).count();
        cx.tally(g, "adversary.factory_troops", placed as i64);
        cx.tally(g, "adversary.project_troops", placed as i64);
        // Wall (r98) keeps a troop off the Protector's node hex.
        cx.note(Some(h), || format!("The adversary factory at {} makes {}", hex_name(&w, h), troop_words(placed, count)));
    }
}

/// 235: "1 adversary troop", "2 adversary troops", or what Wall kept off the hex.
fn troop_words(placed: usize, asked: i64) -> String {
    let n = |k: usize| format!("{k} adversary troop{}", if k == 1 { "" } else { "s" });
    if placed as i64 == asked {
        n(placed)
    } else {
        format!("{} (Wall keeps {} off)", n(placed), asked - placed as i64)
    }
}

/// 231 (`AgentsOnNodes`, Night Raid): in each country, 1 agent onto each of its `count` lowest-numbered hexes that
/// hold a player's node and an agent (297: no "most agents" pick, and no agent onto a clean node hex: the table
/// sees which nodes are at risk, the ones an agent already stands on); a firewalled hex is passed over.
/// 275: each agent is an arrival: a node hex it brings to 2 agents loses its node at once.
pub(super) fn agents_on_nodes(g: &mut GameState, cx: &mut Cx, count: i64) -> Res {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        // 285: a cordoned country takes no agent (178).
        if board::flagged(g, crate::content::RoundFlag::Cordon, c) {
            continue;
        }
        let hexes: Vec<usize> = by_number(&w, c)
            .into_iter()
            .filter(|&h| node_owner(g, h).is_some() && g.tokens(h, ADV_AMBASSADOR) > 0 && !firewalled(g, h))
            .collect();
        for &h in hexes.iter().take(count.max(0) as usize) {
            cx.tally(g, "adversary.night_raid_agents", 1);
            // 235: the note follows the placement: an empty supply lands nothing, an ambassador there cancels it.
            // 285: written by the placement from what it did (landed, canceled, short), before the arrival's own note
            // (it read the agents left after an arrival that destroyed the node, and called that a cancel).
            adversary::place_noted(g, cx, h, 1, &|| "Night Raid".to_string())?;
        }
    }
    Ok(())
}

/// 231 (`RecallAmbassadors`): in each country, the hex holding the most player ambassadors (at least one; ties:
/// the lowest-numbered) sends them all back to their owners' supplies.
pub(super) fn recall_ambassadors(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let seats = g.seats.len();
    for c in 0..w.map.countries.len() {
        let on = |g: &GameState, h: usize| (0..seats).map(|s| g.tokens(h, ambassador(s))).sum::<i64>();
        let hexes = by_number(&w, c);
        let best = hexes.iter().map(|&h| on(g, h)).max().unwrap_or(0);
        if best < 1 {
            continue;
        }
        let Some(h) = hexes.iter().copied().find(|&h| on(g, h) == best) else {
            continue;
        };
        let owners = (0..seats).filter(|&s| g.tokens(h, ambassador(s)) > 0).count();
        for s in 0..seats {
            let n = g.tokens(h, ambassador(s));
            if n > 0 {
                crate::modules::board::take_ambassadors(g, cx, s, h, n);
                cx.add_resource(g, s, Resource::Ambassadors, n);
                cx.tally(g, "adversary.ambassadors_recalled", n);
            }
        }
        // 285: one ambassador read "ambassadors ... their supplies".
        cx.note(Some(h), || {
            let (one, at) = (best == 1, hex_name(&w, h));
            let to = if owners > 1 { "their owners' supplies" } else if one { "its owner's supply" } else { "their owner's supply" };
            format!("The player ambassador{} at {at} go{} back to {to}", if one { "" } else { "s" }, if one { "es" } else { "" })
        });
    }
}

/// 231 (`StripCubes`): each open megaproject loses up to `count` player research cubes, one at a time from the
/// player with the most on it (ties: the first seat).
pub(super) fn strip_cubes(g: &mut GameState, cx: &mut Cx, count: i64) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        if g.var(&board::project_done(c)) != 0 {
            continue;
        }
        for _ in 0..count.max(0) {
            let most = (0..g.seats.len()).rev().max_by_key(|&s| g.var(&board::project_key(c, s)));
            let Some(s) = most.filter(|&s| g.var(&board::project_key(c, s)) > 0) else {
                break;
            };
            cx.add_key(g, &board::project_key(c, s), -1);
            cx.tally(g, "adversary.cubes_stripped", 1);
        }
    }
}

/// 195 (`StripHarden`): in each country, the lowest-numbered hardened facility loses its hardening tokens (297:
/// no "most agents" pick).
pub(super) fn strip_harden(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        // A facility's hardening: a Science card's node shield on a hex with no facility is no target (195 review).
        let hexes: Vec<usize> =
            by_number(&w, c).into_iter().filter(|&h| g.tokens(h, HARDEN) > 0 && board::hardenable(g, h)).collect();
        if let Some(&h) = hexes.first() {
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
        // r141, the guarding rule: no cube comes off a site with a troop on or next to it.
        Removable::Cubes => corrupted(g, h) && !is_city(w, h) && !board::troop_guarded(w, g, h),
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
            // 312 (D136): a cube off a station or factory frees it, as Liberate does (its agents off, its Unity).
            Removable::Cubes => {
                cx.tally(g, "effects.cleansed", 1);
                // Counted as a card's freeing, not the Liberate action's (`energy_read.py` subtracts these).
                cx.tally(g, "board.liberated_card", 1);
                match board::free_site(g, cx, h) {
                    Some(k) => cx.tally(g, &format!("board.liberated_card.{}", k.name()), 1),
                    None => {
                        cx.add_tokens(g, h, ADV_CUBE, -1);
                    }
                }
            }
        }
        k += 1;
    }
    k
}

/// 167, a board-conditioned removal (fear-card shape): on each hex where `wh` holds, in each country piece from
/// such hexes (players choose, one piece at a time), or each player from one such hex (their choice).
pub(super) fn remove_where(g: &mut GameState, cx: &mut Cx, pieces: Removable, count: i64, per: Per, wh: Where) -> Res<i64> {
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
    Ok(removed)
}

/// 167: "for each `wh` (at most `max`)": how many times `then` resolves.
pub(super) fn per_site_count(w: &World, g: &GameState, wh: Where, max: i64) -> i64 {
    (plan::sites_where(w, g, wh).len() as i64).min(max)
}
