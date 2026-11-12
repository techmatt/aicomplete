//! Corruption: Corrupt X lines with their convergence, natural corruption, and station and factory flips.

use super::*;

/// 175: a Corrupt X line's converging agent asks "Corrupt cities (at 4): the agent at .. moves onto which?". The
/// agents read it back ([`read_converge_prompt`]; 182: one home for the words, so a rewording can't silently turn
/// `converge_rule` off).
pub fn converge_prompt(site: CorruptSite, n: i64, at: &str) -> String {
    format!("Corrupt {} (at {n}): the agent at {at} moves onto which?", site.plural())
}

/// The site and number a [`converge_prompt`] prints, if `prompt` is one.
pub fn read_converge_prompt(prompt: &str) -> Option<(CorruptSite, i64)> {
    let rest = prompt.strip_prefix("Corrupt ")?;
    if !rest.contains("moves onto which?") {
        return None;
    }
    let site = CorruptSite::ALL.into_iter().find(|s| rest.starts_with(&format!("{} (at ", s.plural())))?;
    let n = rest[site.plural().len() + " (at ".len()..].split(')').next()?.parse().ok()?;
    Some((site, n))
}

/// Any player cube on hex `h` comes off and the adversary's goes on.
fn take_for_adversary(g: &mut GameState, cx: &mut Cx, h: usize) {
    if let Some(s) = cube_owner(g, h) {
        cx.add_tokens(g, h, cube(s), -1);
    }
    cx.add_tokens(g, h, ADV_CUBE, 1);
}

/// Every corruption marker on hex `h` comes off: corruption replaces them.
pub fn clear_markers_on(g: &mut GameState, cx: &mut Cx, h: usize) {
    let marked = g.tokens(h, MARKER);
    cx.add_tokens(g, h, MARKER, -marked);
}

/// City `h` is corrupted, as [`corrupt_hex`] with [`CorruptSite::City`]. It checks no guard: a caller that a
/// firewall or Corrupt prohibition can stop asks [`guard_holds`] first (185).
pub fn corrupt_city(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    corrupt_hex(g, cx, h, CorruptSite::City)
}

/// 138: hex `h` corrupts as a `site`: a city, research station or factory
/// takes the adversary's cube (a player's comes off); an empty land hex takes
/// land corruption. Its agents return to the supply; a node on
/// the hex is destroyed (its owner's ambassador there is armor, used up
/// instead; the Sanctuary capstone keeps it); player ambassadors still there
/// are lost (024: they would free it in the same step); a marker comes off.
pub fn corrupt_hex(g: &mut GameState, cx: &mut Cx, h: usize, site: CorruptSite) -> Res {
    let w = cx.world;
    if site == CorruptSite::Node {
        return corrupt_node(g, cx, h);
    }
    if site == CorruptSite::Land {
        cx.add_tokens(g, h, LAND_CORRUPTION, 1);
    } else {
        take_for_adversary(g, cx, h);
    }
    let n = g.tokens(h, ADV_AMBASSADOR);
    remove(g, cx, h, n);
    if let Some(s) = node_owner(g, h) {
        if !cards::has_passive(&w, g, s, |p| matches!(p, Op::CorruptionShield)) {
            board::destroy_piece(g, cx, h, node(s), board::Cause::Effect("corruption"))?;
        }
    }
    for s in 0..g.seats.len() {
        let k = ambassador(s);
        let mine = g.tokens(h, k);
        cx.add_tokens(g, h, k, -mine);
    }
    clear_markers_on(g, cx, h);
    cx.tally(g, "adversary.corruptions", 1);
    cx.tally(g, &format!("adversary.corruptions.{}", site.name()), 1);
    if g.round > 0 && g.var(FIRST_CORRUPTION) == 0 {
        cx.set_key(g, FIRST_CORRUPTION, g.round as i64);
        cx.tally(g, "adversary.first_corruption_round", g.round as i64);
    }
    cx.note(Some(h), || match site {
        CorruptSite::Land => format!("{} takes land corruption", hex_name(&w, h)),
        _ => format!("{} is corrupted", hex_name(&w, h)),
    });
    Ok(())
}

/// 175, a node hex corrupts: its agents go to the supply and the node is destroyed (the protections apply;
/// the Sanctuary capstone keeps it). r83 (Matt, 2026-11-11, #56): a node hex corrupts like open land: it always
/// takes land corruption, the one adversary mark, whether or not a protection saves the node (r80 marked it only
/// when the node was lost).
fn corrupt_node(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    let n = g.tokens(h, ADV_AMBASSADOR);
    remove(g, cx, h, n);
    if let Some(s) = node_owner(g, h) {
        if !cards::has_passive(&w, g, s, |p| matches!(p, Op::CorruptionShield)) {
            board::destroy_piece(g, cx, h, node(s), board::Cause::Effect("corruption"))?;
        }
    }
    clear_markers_on(g, cx, h);
    if g.tokens(h, LAND_CORRUPTION) == 0 {
        cx.add_tokens(g, h, LAND_CORRUPTION, 1);
        // 192: placed face down, so its drip is `blight_node_discord`'s, and counted for the agents.
        cx.set_key(g, &node_blight_hex(h), 1);
        let n = g.var(NODE_BLIGHT);
        cx.set_key(g, NODE_BLIGHT, n + 1);
    }
    cx.tally(g, "adversary.corruptions", 1);
    cx.tally(g, "adversary.corruptions.node", 1);
    cx.note(Some(h), || format!("The node at {} is corrupted", hex_name(&w, h)));
    Ok(())
}

/// 192: face-down land corruption (a corrupted node hex's) on the board, and the hex flag that marks one.
pub const NODE_BLIGHT: &str = "adversary.node_blight";

pub fn node_blight_hex(h: usize) -> KeyBuf {
    KeyBuf::new("adversary.node_blight.").num(h)
}

/// 192: a land-corruption token comes off hex `h`: a face-down one leaves the count.
pub fn lift_land_corruption(g: &mut GameState, cx: &mut Cx, h: usize) {
    cx.add_tokens(g, h, LAND_CORRUPTION, -1);
    if g.var(&node_blight_hex(h)) > 0 {
        cx.set_key(g, &node_blight_hex(h), 0);
        let n = g.var(NODE_BLIGHT);
        cx.set_key(g, NODE_BLIGHT, n - 1);
    }
}

/// 192: Discord the land corruption on the board sways a round: face-up tokens at `blight_discord`, face-down
/// (a corrupted node hex's) at `blight_node_discord`.
pub fn blight_drip(w: &World, g: &GameState) -> i64 {
    let all: i64 = (0..g.hexes.len()).map(|h| g.tokens(h, LAND_CORRUPTION)).sum();
    let down = g.var(NODE_BLIGHT).min(all);
    (all - down) * w.int("adversary.blight_discord") + down * w.int("adversary.blight_node_discord")
}

/// 138: the corrupt number of `site` this game: its config number, less the
/// difficulty's `corrupt_cut` from `corrupt_cut_at`, less a line's `cut`; at
/// least 1. 175: a coastal city's is a city's; a node's takes no difficulty cut.
pub fn corrupt_n(w: &World, site: CorruptSite, cut: i64) -> i64 {
    let site = if site == CorruptSite::CoastalCity { CorruptSite::City } else { site };
    let base = match site {
        CorruptSite::City | CorruptSite::CoastalCity => w.int("adversary.corrupt_at"),
        CorruptSite::Station => w.int("adversary.corrupt_station_at"),
        CorruptSite::Factory => w.int("adversary.corrupt_factory_at"),
        CorruptSite::Land => w.int("adversary.corrupt_land_at"),
        CorruptSite::Node => w.int("adversary.corrupt_node_at"),
    };
    let ladder = if level(w, w.int("adversary.corrupt_cut_at")) {
        let i = CorruptSite::ALL.iter().position(|&s| s == site).unwrap_or(0);
        w.list("adversary.corrupt_cut").get(i).copied().unwrap_or(0)
    } else {
        0
    };
    (base - ladder - cut).max(1)
}

/// 175, a Corrupt X line in country `scope` or everywhere: every agent next to an X ([`super::plan::converge`])
/// moves onto it, players choosing for one next to several; then every X holding its number corrupts. A
/// moving agent arriving on a player ambassador cancels it, as any arrival does.
pub fn corrupt_sites(g: &mut GameState, cx: &mut Cx, site: CorruptSite, cut: i64, scope: Option<usize>) -> Res {
    let w = cx.world;
    let n = corrupt_n(&w, site, cut);
    let cv = super::plan::converge(&w, g, site, scope);
    // Single-target agents first, so a choice sees where the forced ones went.
    let mut flexible = Vec::new();
    for (h, to) in &cv.sources {
        if to.len() == 1 {
            let k = g.tokens(*h, ADV_AMBASSADOR);
            for _ in 0..k {
                converge_one(g, cx, *h, to[0])?;
            }
        } else {
            flexible.push((*h, to.clone()));
        }
    }
    for (h, to) in flexible {
        while g.tokens(h, ADV_AMBASSADOR) > 0 {
            let opts = to.iter().map(|&t| hex_opt(&w, t)).collect();
            let prompt = || converge_prompt(site, n, &hex_name(&w, h));
            let Some(i) = cx.choose(0, prompt, opts)? else {
                break;
            };
            converge_one(g, cx, h, to[i])?;
        }
    }
    for t in cv.targets {
        // r80 (ruling #42): agents only; a marker no longer counts.
        let held = g.tokens(t, ADV_AMBASSADOR);
        if held < n {
            continue;
        }
        // A Corrupt prohibition's token or (178, Matt's ruling #30) a firewall holds it (185's audit: one guard).
        if guard_holds(g, cx, t, "line") {
            continue;
        }
        // An earlier corruption may have taken this target's node (a node line) or flipped it already.
        if !super::plan::corrupt_target(&w, g, t, site) {
            continue;
        }
        corrupt_hex(g, cx, t, site)?;
        if g.is_over() {
            return Ok(());
        }
    }
    Ok(())
}

/// 175: one agent moves from `from` onto `to` (Corrupt X's convergence): on a player ambassador the two cancel.
fn converge_one(g: &mut GameState, cx: &mut Cx, from: usize, to: usize) -> Res {
    cx.add_tokens(g, from, ADV_AMBASSADOR, -1);
    cx.tally(g, "adversary.converged", 1);
    // r75: the players choose whose ambassador it cancels.
    if board::agent_cancels(g, cx, to)?.is_some() {
        cx.add_key(g, SUPPLY, 1);
        cx.tally(g, "adversary.canceled", 1);
    } else {
        cx.add_tokens(g, to, ADV_AMBASSADOR, 1);
    }
    Ok(())
}

/// 138, natural corruption at the end of the event card: every city,
/// research station or factory already holding its corrupt number (r80: agents only, no markers) corrupts, with no move, in hex order; 175: then every
/// hex with a player's node holding a node's number loses the node.
pub(super) fn natural_corruption(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for site in [CorruptSite::City, CorruptSite::Station, CorruptSite::Factory, CorruptSite::Node] {
        let n = corrupt_n(&w, site, 0);
        for h in super::plan::natural_targets(&w, g, site, n) {
            // 167: a Corrupt prohibition covers natural corruption too; 178 (Matt's ruling #30): so does a firewall.
            if guard_holds(g, cx, h, "natural") {
                continue;
            }
            corrupt_hex(g, cx, h, site)?;
            cx.tally(g, "adversary.corruptions_natural", 1);
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// 185 (Matt's ruling on 184's D77, candidate): a firewall or a Corrupt prohibition guards every flip and
/// corruption, not only Corrupt lines and natural corruption: whether site `h` is guarded, tallied as held
/// (`adversary.guard_held.<path>`, and the firewall's or prohibition's own counter) when it is.
pub fn guard_holds(g: &mut GameState, cx: &mut Cx, h: usize, path: &str) -> bool {
    if let Some(wh) = super::plan::prohibition_on(g, Step::Corrupt, h) {
        prohibit_tally(g, cx, Step::Corrupt, wh, g.tokens(h, ADV_AMBASSADOR));
    } else if firewalled(g, h) {
        cx.tally(g, "adversary.firewall_held", 1);
    } else {
        return false;
    }
    cx.tally(g, &format!("adversary.guard_held.{path}"), 1);
    true
}

/// Research station `h`, not yet corrupted, flips to the adversary. r35: a
/// player's station is defended: it flips only with `station_flip_at`
/// agents on it and no player ambassador, and those are used
/// up; short of that one agent lands on it instead (a player
/// ambassador standing there cancels it).
/// Returns whether it flipped (099: a player's station that holds is no new source).
pub fn corrupt_station(g: &mut GameState, cx: &mut Cx, h: usize) -> Res<bool> {
    let w = cx.world;
    // 185 (D77): a firewalled or prohibited station doesn't flip (the flip paths leave it out of their choice).
    if guard_holds(g, cx, h, "flip") {
        return Ok(false);
    }
    if cube_owner(g, h).is_some() {
        let need = w.int("adversary.station_flip_at");
        if g.tokens(h, ADV_AMBASSADOR) < need || player_ambassador(g, h).is_some() {
            // 099: say what happened: a player's ambassador there cancels the one
            // arriving, and an empty supply sends none (both read "lands on it").
            let (adv, supply) = (g.tokens(h, ADV_AMBASSADOR), g.var(SUPPLY));
            place(g, cx, h, 1)?;
            cx.tally(g, "adversary.station_flip_held", 1);
            let what = if supply <= 0 {
                "no agent is left to send"
            } else if g.tokens(h, ADV_AMBASSADOR) > adv {
                "an agent lands on it"
            } else {
                "the agent sent and a player ambassador there cancel each other"
            };
            cx.note(Some(h), || format!("The station at {} holds: {what}", hex_name(&w, h)));
            return Ok(false);
        }
        remove(g, cx, h, need);
    }
    // 146 (D60): a flip is corruption: its cleanup as everywhere (the node on it or its armor, the adversary
    // ambassadors back to the supply, player ambassadors there lost, markers off).
    corrupt_hex(g, cx, h, CorruptSite::Station)?;
    cx.tally(g, "adversary.stations_flipped", 1);
    cx.note(Some(h), || {
        format!("The station at {} flips to the adversary", hex_name(&w, h))
    });
    Ok(true)
}

/// Factory `h` becomes the adversary's (070: a threat's target): it corrupts,
/// with corruption's cleanup (146, D60: the node on it or its armor, the
/// ambassadors; any player cube comes off, its cube goes on, markers off).
pub fn corrupt_factory(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    corrupt_hex(g, cx, h, CorruptSite::Factory)?;
    cx.tally(g, "adversary.factories_seized", 1);
    cx.note(Some(h), || format!("The factory at {} goes to the adversary", hex_name(&w, h)));
    Ok(())
}
