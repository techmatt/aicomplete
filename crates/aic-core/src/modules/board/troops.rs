//! Adversary troops (r141, Matt 2026-11-19): they garrison where they are placed and guard the corrupted sites on
//! or next to them ([`troop_guarded`]: no liberation there); they destroy only on a Strike icon ([`strike`]), and
//! move only on a Redeploy icon ([`redeploy`]) or a player's Push or Gather. There is no march.

use crate::module::Hot;
use crate::modules::plan;
use crate::op::Step;
use super::*;

/// Whether a troop stands on or next to hex `h`: a corrupted site there can't be liberated, by the action or by
/// a card (the guarding rule).
pub fn troop_guarded(w: &World, g: &GameState, h: usize) -> bool {
    next_to(w, h, |x| g.tokens(x, TROOP) > 0)
}

/// Where a country's new troop garrisons: its lowest-numbered corrupted factory, else its lowest-numbered
/// corrupted city, else its lowest-numbered city (`hexes`: the country's, or a threat's scope).
pub fn garrison_spot(w: &World, g: &GameState, hexes: &[usize]) -> Option<usize> {
    let lowest = |f: &dyn Fn(usize) -> bool| hexes.iter().copied().filter(|&h| f(h)).min_by_key(|&h| (w.map.hexes[h].country, w.map.hexes[h].number));
    lowest(&|h| g.tokens(h, FACTORY) > 0 && corrupted(g, h))
        .or_else(|| lowest(&|h| corrupted_city(w, g, h)))
        .or_else(|| lowest(&|h| is_city(w, h)))
}

/// The hexes holding troops that act (standing ones never do), in country then hex-number order, with how many.
fn stacks(w: &World, g: &GameState) -> Vec<(usize, i64)> {
    let mut out: Vec<(usize, i64)> =
        (0..g.hexes.len()).map(|h| (h, g.tokens(h, TROOP) - standing(g, h))).filter(|&(_, n)| n > 0).collect();
    out.sort_by_key(|&(h, _)| (w.map.hexes[h].country, w.map.hexes[h].number));
    out
}

/// Strike (an icon on an event card, shown a round ahead, or on a research slot): every troop destroys one
/// player node, or player-owned research station or factory, on or next to its hex (players choose which; a
/// protection absorbs a hit as it does any destruction). The troop stays: a garrison strikes again on the next
/// icon. Troops strike hex by hex, country by country; a strike that takes a seat's last node ends it.
/// Then Vigil: each seat holding it is paid once for each troop on or next to its nodes, at most
/// `board.vigil_cap` a round.
pub fn strike(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let lost = |g: &GameState| w.hot(Hot::NodeLoss) > 0 && adversary::loss(g).is_some();
    cx.tally(g, "board.strikes", 1);
    cx.per_round(g, "map.strikes", 1);
    let all = stacks(&w, g);
    for &(h, n) in &all {
        for _ in 0..n.min(g.tokens(h, TROOP) - standing(g, h)) {
            if lost(g) || g.is_over() {
                return Ok(());
            }
            let hit = strike_from(g, cx, h)?;
            if lost(g) || g.is_over() {
                return Ok(());
            }
            if !hit {
                break;
            }
        }
    }
    if lost(g) || g.is_over() {
        return Ok(());
    }
    let vigil: Vec<usize> = (0..g.seats.len()).filter(|&s| cards::has_vigil(&w, g, s)).collect();
    for s in vigil {
        let near: i64 = all
            .iter()
            .filter(|&&(h, _)| next_to(&w, h, |x| g.tokens(x, node(s)) > 0))
            .map(|&(h, n)| n.min(g.tokens(h, TROOP)).max(0))
            .sum();
        let (round, paid) = (vigil_round(s), vigil_paid(s));
        if g.var(&round) != g.round as i64 {
            cx.set_key(g, &round, g.round as i64);
            cx.set_key(g, &paid, 0);
        }
        for _ in 0..near {
            if g.var(&paid) >= w.int("board.vigil_cap") {
                cx.tally(g, "board.vigil_capped", 1);
                continue;
            }
            cx.add_key(g, &paid, 1);
            cx.tally(g, "board.vigil", 1);
            cards::on_strike_near(g, cx, s)?;
        }
    }
    Ok(())
}

/// Whether hex `h` holds a player piece: any seat's node or ambassador.
fn player_piece(g: &GameState, h: usize) -> bool {
    (0..g.seats.len()).any(|s| g.tokens(h, node(s)) > 0 || g.tokens(h, ambassador(s)) > 0)
}

/// Where a Redeploy sends country `c`'s troops (r144, Matt 2026-11-20): its corrupted city nearest a player piece
/// (a node or an ambassador; distance in hexes; ties: the lower hex number). A city under the Protector's node is no
/// destination (Wall). None: no corrupted city, or no player piece on the map.
pub fn redeploy_spot(w: &World, g: &GameState, c: u8) -> Option<usize> {
    let pieces: Vec<usize> = (0..g.hexes.len()).filter(|&x| player_piece(g, x)).collect();
    (0..g.hexes.len())
        .filter(|&x| w.map.hexes[x].country == c && corrupted_city(w, g, x) && !walled(w, g, x))
        .filter_map(|x| pieces.iter().map(|&p| w.map.distance(x, p)).min().map(|d| (d, w.map.hexes[x].number, x)))
        .min()
        .map(|(_, _, x)| x)
}

/// Redeploy (an icon on an event card, rare; r144, Matt 2026-11-20): each troop moves onto its country's corrupted
/// city nearest a player piece ([`redeploy_spot`]); one already there stays, and with no such city it stays. So a
/// Redeploy brings the garrisons to where the players are working: they guard the city being liberated and the next
/// Strike finds pieces beside them. (r141-r143 moved only troops not on or next to a corrupted city, onto the
/// nearest one: about 0 troops a game, as a garrison always starts on a corrupted site.)
pub fn redeploy(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    cx.tally(g, "board.redeploys", 1);
    cx.per_round(g, "map.redeploys", 1);
    for (h, n) in stacks(&w, g) {
        let to = redeploy_spot(&w, g, w.map.hexes[h].country);
        let Some(to) = to.filter(|&to| to != h) else {
            cx.tally(g, "board.redeploy_stayed", n);
            continue;
        };
        cx.add_tokens(g, h, TROOP, -n);
        cx.add_tokens(g, to, TROOP, n);
        cx.note(Some(to), || {
            format!("{n} adversary troop{} redeploy{} from {} to {}", if n == 1 { "" } else { "s" }, if n == 1 { "s" } else { "" }, hex_name(&w, h), hex_name(&w, to))
        });
        cx.per_round(g, adversary::OPS, n);
        cx.tally(g, "board.troops_redeployed", n);
        cx.per_round(g, "map.troops_redeployed", n);
    }
    Ok(())
}

/// What an adversary troop may destroy on hex `h`, as token keys: a
/// node, and a player-owned research station or factory (019: not
/// neutral or corrupted ones, and never cities).
pub(super) fn troop_targets_on(w: &World, g: &GameState, h: usize) -> Vec<String> {
    let mut out = Vec::new();
    // 075: a Wall stops the strike; a no-Strike token (a Unity card's) does too.
    if walled(w, g, h) || plan::prohibited_now(g, Step::Strike, h) {
        return out;
    }
    if cube_owner(g, h).is_some() && !corrupted(g, h) {
        if g.tokens(h, FACTORY) > 0 {
            out.push(FACTORY.to_string());
        }
        if g.tokens(h, STATION) > 0 {
            out.push(STATION.to_string());
        }
    }
    if g.var(adversary::STAGE) >= w.hot(Hot::TroopNode) {
        if let Some(s) = node_owner(g, h) {
            out.push(node(s).to_string());
        }
    }
    out
}

/// Whether seat `s` has Wall: `Content::check` keeps it to innates, so no
/// card's passives need reading.
pub fn has_wall(w: &World, seat: usize) -> bool {
    ai::board(w, seat).innate.passives.iter().any(|p| matches!(p, Op::Wall))
}

/// Whether hex `h` holds node of a seat with Wall (068): no troop is placed on it or Redeploys onto it, and no
/// Strike hits it.
pub fn walled(w: &World, g: &GameState, h: usize) -> bool {
    node_owner(g, h).is_some_and(|s| has_wall(w, s))
}

/// One adversary troop at `h` strikes a target on or next to its hex
/// (players choose): it is destroyed or the hit absorbed. Returns false when it had no target.
fn strike_from(g: &mut GameState, cx: &mut Cx, h: usize) -> Res<bool> {
    let w = cx.world;
    let mut targets: Vec<(usize, String)> = Vec::new();
    for x in within(&w, h, 1) {
        // 261: a blocked strike is counted where a Unity prohibition holds (`prohibit.strike.<where>.fired` and
        // `.blocked`, as Infiltrate's and Corrupt's are).
        if let Some(wh) = plan::prohibition_on(g, Step::Strike, x).filter(|_| node_owner(g, x).is_some() || cube_owner(g, x).is_some()) {
            adversary::prohibit_tally(g, cx, Step::Strike, wh, 1);
        }
        // 075: the Wall's measure: a player piece this troop would have struck but for the Protector's node there.
        if walled(&w, g, x) {
            cx.tally(g, "board.wall_strikes", 1);
        }
        for k in troop_targets_on(&w, g, x) {
            targets.push((x, k));
        }
    }
    // 308: a struck station or factory names its owner, as a node does (seat 0 answers for the table).
    let owned = |x: usize, k: &str, name: String| match cube_owner(g, x).filter(|_| k == STATION || k == FACTORY) {
        Some(s) => format!("seat {s}'s {name}"),
        None => name,
    };
    let opts = targets
        .iter()
        .map(|(x, k)| opt(format!("{} at {}", owned(*x, k, piece_name(k)), hex_name(&w, *x))))
        .collect();
    let prompt = || format!(
        "Strike: the adversary troop at {} destroys which target?",
        hex_name(&w, h)
    );
    let Some(i) = cx.choose_loss(0, prompt, opts)? else {
        return Ok(false);
    };
    let (x, k) = targets[i].clone();
    let owner = cube_owner(g, x).filter(|_| k == STATION || k == FACTORY);
    // 289 (D133): written before any protection choice, so a strike answered in the owner's own frame says what
    // struck.
    cx.note(Some(x), || {
        let piece = owner.map_or_else(|| piece_words(&k), |s| format!("Seat {s}'s {}", piece_name(&k)));
        let the = if piece.starts_with("Seat ") { "" } else { "the " };
        format!("The adversary troop at {} strikes {the}{piece} at {}", hex_name(&w, h), hex_name(&w, x))
    });
    cx.tally(g, "board.troop_strikes", 1);
    cx.per_round(g, "map.troop_strikes", 1);
    if destroy_piece(g, cx, x, &k, Cause::Troop)? {
        let kind = if k == FACTORY || k == STATION { k.as_str() } else { "node" };
        cx.tally(g, &format!("board.troop_destroyed.{kind}"), 1);
        cx.per_round(g, &format!("map.troop_destroyed.{kind}"), 1);
        cx.tally(g, "board.troop_destroyed", 1);
    } else {
        cx.tally(g, "board.troop_absorbed", 1);
    }
    Ok(true)
}

/// For the view: the hexes the troops on `h` would strike on the next Strike icon (every target on or next to
/// it; the players choose one a troop).
pub fn strike_targets(w: &World, g: &GameState, h: usize) -> Vec<usize> {
    within(w, h, 1).into_iter().filter(|&x| !troop_targets_on(w, g, x).is_empty()).collect()
}

/// For the view: the corrupted sites the troops on `h` guard (on or next to it).
pub fn guarded_sites(w: &World, g: &GameState, h: usize) -> Vec<usize> {
    within(w, h, 1).into_iter().filter(|&x| corrupted(g, x) && site_kind(w, g, x).is_some()).collect()
}
