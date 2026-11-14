//! Corruption: Corrupt X lines with their convergence, natural corruption, and station and factory flips.

use super::*;

/// Any player cube on hex `h` comes off and the adversary's goes on.
fn take_for_adversary(g: &mut GameState, cx: &mut Cx, h: usize) {
    if let Some(s) = cube_owner(g, h) {
        cx.add_tokens(g, h, cube(s), -1);
    }
    cx.add_tokens(g, h, ADV_CUBE, 1);
}

/// City `h` is corrupted, as [`corrupt_hex`] with [`CorruptSite::City`]. It checks no guard: a caller that a
/// firewall or Corrupt prohibition can stop asks [`guard_holds`] first (185).
pub fn corrupt_city(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    corrupt_hex(g, cx, h, CorruptSite::City)
}

/// 138: hex `h` corrupts as a `site`: a city, research station or factory
/// takes the adversary's cube (a player's comes off); an empty land hex takes
/// Fallout. Its agents return to the supply; a node on
/// the hex is destroyed (its owner's ambassador there is armor, used up
/// instead; the Sanctuary capstone keeps it); player ambassadors still there
/// are lost (024: they would free it in the same step).
pub fn corrupt_hex(g: &mut GameState, cx: &mut Cx, h: usize, site: CorruptSite) -> Res {
    let w = cx.world;
    if site == CorruptSite::Node {
        return corrupt_node(g, cx, h);
    }
    if site == CorruptSite::Land {
        cx.add_tokens(g, h, FALLOUT, 1);
        cx.tally(g, "fallout.placed", 1);
        cx.tally(g, "fallout.placed.event", 1);
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
    cx.tally(g, "adversary.corruptions", 1);
    cx.tally(g, &format!("adversary.corruptions.{}", site.name()), 1);
    if g.round > 0 && g.var(FIRST_CORRUPTION) == 0 {
        cx.set_key(g, FIRST_CORRUPTION, g.round as i64);
        cx.tally(g, "adversary.first_corruption_round", g.round as i64);
    }
    cx.note(Some(h), || match site {
        CorruptSite::Land => format!("{} takes Fallout", hex_name(&w, h)),
        _ => format!("{} is corrupted", hex_name(&w, h)),
    });
    Ok(())
}

/// 175, 2 agents on a player's node hex destroy the node: the agents go to the supply and the node is destroyed
/// (the protections apply; the Sanctuary capstone keeps it). r98 (Matt, 2026-11-13, 224 #2): a node rule, not
/// corruption, and it leaves no mark (r83-r97 laid face-down Fallout): losing it is punishment enough.
fn corrupt_node(g: &mut GameState, cx: &mut Cx, h: usize) -> Res {
    let w = cx.world;
    let n = g.tokens(h, ADV_AMBASSADOR);
    remove(g, cx, h, n);
    if let Some(s) = node_owner(g, h) {
        if !cards::has_passive(&w, g, s, |p| matches!(p, Op::CorruptionShield)) {
            board::destroy_piece(g, cx, h, node(s), board::Cause::Effect("corruption"))?;
        }
    }
    cx.tally(g, "adversary.corruptions", 1);
    cx.tally(g, "adversary.corruptions.node", 1);
    cx.note(Some(h), || format!("Agents destroy the node at {}", hex_name(&w, h)));
    Ok(())
}

/// A Fallout token comes off hex `h`. 227: tallied `fallout.lifted`.
pub fn lift_fallout(g: &mut GameState, cx: &mut Cx, h: usize) {
    cx.add_tokens(g, h, FALLOUT, -1);
    cx.tally(g, "fallout.lifted", 1);
}

/// 227: a hex Fallout may land on: open land with no building, node or Fallout (agents and ambassadors may stand).
pub fn fallout_free(w: &World, g: &GameState, h: usize) -> bool {
    buildable(w, g, h) && node_owner(g, h).is_none()
}

/// 227: a Fallout token goes on hex `h`, tallied `fallout.placed` and by `source` (a threat tier, `project`,
/// `discord`, `event`, `burn`).
pub fn place_fallout(g: &mut GameState, cx: &mut Cx, h: usize, source: &str) {
    let w = cx.world;
    cx.add_tokens(g, h, FALLOUT, 1);
    cx.tally(g, "fallout.placed", 1);
    cx.tally(g, &format!("fallout.placed.{source}"), 1);
    cx.note(Some(h), || format!("Fallout lands on {}", hex_name(&w, h)));
}

/// Discord the Fallout on the board sways a round: `blight_discord` a token (r98: no token lies face down).
pub fn blight_drip(w: &World, g: &GameState) -> i64 {
    let all: i64 = (0..g.hexes.len()).map(|h| g.tokens(h, FALLOUT)).sum();
    all * w.int("adversary.blight_discord")
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
/// moves onto it (r98: one next to several stays where it is); then every X holding its number corrupts. A
/// moving agent arriving on a player ambassador cancels it, as any arrival does.
pub fn corrupt_sites(g: &mut GameState, cx: &mut Cx, site: CorruptSite, cut: i64, scope: Option<usize>) -> Res {
    let w = cx.world;
    let n = corrupt_n(&w, site, cut);
    let cv = super::plan::converge(&w, g, site, scope);
    // r98 (Matt, 2026-11-13, 224 #6): an agent next to two or more targets stays where it is.
    for &(h, t) in &cv.sources {
        for _ in 0..g.tokens(h, ADV_AMBASSADOR) {
            converge_one(g, cx, h, t)?;
        }
    }
    for &h in &cv.ties {
        let k = g.tokens(h, ADV_AMBASSADOR);
        cx.tally(g, "adversary.converge_ties", k);
        cx.tally(g, "adversary.converge_tie_hexes", 1);
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
    cx.tally(g, "adversary.factories_corrupted", 1);
    cx.note(Some(h), || format!("The factory at {} goes to the adversary", hex_name(&w, h)));
    Ok(())
}
