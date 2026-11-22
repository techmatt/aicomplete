//! Placing and removing adversary pieces: agents from the supply, troop landings, and the players' removals.

use super::*;

/// Places `n` agents from the supply on hex `h`. The supply
/// is a cap (027): a placement it can't make doesn't happen.
pub fn place(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> Res {
    place_as(g, cx, h, n, false, None)
}

/// 285: [`place`] with [`place_line`]'s note (a factory's own agent, a creation line).
pub fn place_noted(g: &mut GameState, cx: &mut Cx, h: usize, n: i64, what: &dyn Fn() -> String) -> Res {
    place_as(g, cx, h, n, false, Some(what))
}

/// r133 (280), 285: [`place`] onto a site an Infiltrate or Escalate line names: its arrival check is made as the
/// agents go on, in the same read of the site (the rule is the same; only the table's count of checks differs),
/// with a note naming the line (`what()`, "Infiltrate (cities)", built only for a reader of
/// events) and what landed, written after
/// the arrivals and before the arrival check's own note, so the turn's log says where each line's agents went.
pub fn place_line(g: &mut GameState, cx: &mut Cx, h: usize, n: i64, what: &dyn Fn() -> String) -> Res {
    place_as(g, cx, h, n, true, Some(what))
}

/// "1 agent" / "2 agents".
fn agents(n: i64) -> String {
    format!("{n} agent{}", if n == 1 { "" } else { "s" })
}

fn place_as(g: &mut GameState, cx: &mut Cx, h: usize, n: i64, named: bool, what: Option<&dyn Fn() -> String>) -> Res {
    let w = cx.world;
    // 158: a firewall stops every arrival on its hex; they stay in the supply.
    if firewalled(g, h) {
        cx.tally(g, "adversary.firewalled", n.max(0));
        if let Some(what) = what.filter(|_| n > 0) {
            cx.note(Some(h), || format!("{}: a firewall keeps {} off {}", what(), agents(n), hex_name(&w, h)));
        }
        return Ok(());
    }
    // 285: a cordon (178) stops every agent landing in its country, whatever line sends it (a factory's push, Night
    // Raid, a fallback); they stay in the supply. The lines that read a whole country skip it before this.
    if super::plan::cordoned(&w, g, h) {
        cx.tally(g, "adversary.cordoned", n.max(0));
        if let Some(what) = what.filter(|_| n > 0) {
            cx.note(Some(h), || format!("{}: the cordon keeps {} off {}", what(), agents(n), hex_name(&w, h)));
        }
        return Ok(());
    }
    let mut landed = false;
    let (mut down, mut canceled) = (0, 0);
    for k in 0..n {
        if g.var(SUPPLY) <= 0 {
            cx.tally(g, "adversary.supply_short", n - k);
            break;
        }
        cx.add_key(g, SUPPLY, -1);
        cx.per_round(g, OPS, 1);
        cx.tally(g, "adversary.arrivals", 1);
        let one = arrive(g, cx, h)?;
        landed |= one;
        if one {
            down += 1;
        } else {
            canceled += 1;
        }
    }
    if let Some(what) = what.filter(|_| n > 0) {
        cx.note(Some(h), || {
            let at = hex_name(&w, h);
            let short = n - down - canceled;
            let mut parts = Vec::new();
            if down > 0 {
                parts.push(format!("{} {} on {at}", agents(down), if down == 1 { "lands" } else { "land" }));
            }
            if canceled > 0 {
                parts.push(format!("{} sent to {at} canceled by as many player ambassadors", agents(canceled)));
            }
            if short > 0 {
                parts.push(format!("the supply is short {} for {at}", agents(short)));
            }
            format!("{}: {}", what(), parts.join("; "))
        });
    }
    // 275: arrival corruption: the hex an agent landed on is checked, and nothing else (all canceled or none sent:
    // nothing arrived, nothing is checked).
    if landed {
        arrive_check_as(g, cx, h, named)?;
    }
    Ok(())
}

/// One agent arrives on hex `h` from the supply: it cancels a
/// player ambassador there (both are removed, no Unity; r75: the players choose
/// whose, [`board::agent_cancels`]). 138: with the spread gone, no botnet forms
/// by arriving (botnets come only from printed lines). Whether it landed (275: a canceled one didn't).
fn arrive(g: &mut GameState, cx: &mut Cx, h: usize) -> Res<bool> {
    if board::agent_cancels(g, cx, h)?.is_some() {
        cx.add_key(g, SUPPLY, 1);
        cx.tally(g, "adversary.canceled", 1);
        return Ok(false);
    }
    cx.add_tokens(g, h, ADV_AMBASSADOR, 1);
    Ok(true)
}

/// Removes the botnet on hex `h` (a card's effect, or r75's Take down a botnet standard action).
pub fn remove_botnet(g: &mut GameState, cx: &mut Cx, h: usize) {
    if g.tokens(h, BOTNET) > 0 {
        cx.add_tokens(g, h, BOTNET, -1);
        // A botnet formed here later is a new one: the hack cut this one (028).
        cx.set_key(g, &cut_key(h), 0);
        cx.tally(g, "adversary.botnets_removed", 1);
        cx.per_round(g, "map.botnets_removed", 1);
        alignment::for_botnet_removed(g, cx);
    }
}

/// Botnets on `hexes`, or on the whole board for `None`.
pub fn botnet_count(g: &GameState, hexes: Option<&[usize]>) -> i64 {
    match hexes {
        Some(hexes) => hexes.iter().map(|&h| g.tokens(h, BOTNET)).sum(),
        None => (0..g.hexes.len()).map(|h| g.tokens(h, BOTNET)).sum(),
    }
}

/// Removes up to `n` agents from hex `h` to the supply.
pub fn remove(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) -> i64 {
    let k = n.min(g.tokens(h, ADV_AMBASSADOR)).max(0);
    if k > 0 {
        cx.add_tokens(g, h, ADV_AMBASSADOR, -k);
        cx.add_key(g, SUPPLY, k);
    }
    k
}


/// An adversary troop lands on hex `h` (the caller counts it). `source` names what made it for 201's by-round
/// measure (`map.troops_placed.<source>`: event, research, project, threat, discord), a stat only. r98 (Matt,
/// 2026-11-13, 224 #10): Wall: no troop is ever placed on the Protector's hex: it stays in the supply. Returns
/// whether it was placed.
pub fn place_troop(g: &mut GameState, cx: &mut Cx, h: usize, source: &str) -> bool {
    place_troop_as(g, cx, h, source, true)
}

/// 285: [`place_troop`] with no note, for a caller that writes its own (a factory's troop, a project's botnet or
/// factory troops).
pub fn place_troop_quiet(g: &mut GameState, cx: &mut Cx, h: usize, source: &str) -> bool {
    place_troop_as(g, cx, h, source, false)
}

fn place_troop_as(g: &mut GameState, cx: &mut Cx, h: usize, source: &str, noted: bool) -> bool {
    if board::walled(&cx.world, g, h) {
        cx.tally(g, "adversary.troops_walled", 1);
        cx.tally(g, &format!("adversary.troops_walled.{source}"), 1);
        return false;
    }
    cx.add_tokens(g, h, TROOP, 1);
    cx.per_round(g, &format!("map.troops_placed.{source}"), 1);
    // 285: the turn panel showed "+1 adversary troop" with no hex or cause (a threat's, Weapons Program's).
    let w = cx.world;
    if !noted {
        return true;
    }
    cx.note(Some(h), || {
        let by = match source {
            "event" => "the event card",
            "research" => "the research track",
            "discord" => "a Discord card",
            "threat" => "a threat",
            "project" => "a project",
            other => other,
        };
        format!("An adversary troop garrisons {} ({by})", hex_name(&w, h))
    });
    true
}

/// `count` adversary troops garrison among `hexes` (a country's, or a threat's scope): each on the lowest-numbered
/// corrupted factory there, else its lowest-numbered corrupted city, else its lowest-numbered city
/// (`board::garrison_spot`: r141, no choice to make). Wall keeps one off the Protector's node hex (it stays in
/// the supply). With `destroy`, each destroys the node on the hex it lands on (027), named for its tally. Returns
/// how many were placed.
pub fn land_troops(
    g: &mut GameState,
    cx: &mut Cx,
    hexes: &[usize],
    count: i64,
    tally: &str,
    destroy: Option<&'static str>,
    source: &str,
) -> Res<i64> {
    let w = cx.world;
    let (mut walled, mut placed) = (0, 0);
    let Some(h) = board::garrison_spot(&w, g, hexes) else {
        return Ok(0);
    };
    for _ in 0..count {
        if !place_troop(g, cx, h, source) {
            cx.tally(g, "adversary.walled_landing", 1);
            walled += 1;
            continue;
        }
        placed += 1;
        cx.per_round(g, OPS, 1);
        cx.tally(g, tally, 1);
        if let Some(source) = destroy {
            for s in 0..g.seats.len() {
                if g.tokens(h, node(s)) > 0 {
                    board::destroy_piece(g, cx, h, node(s), board::Cause::Effect(source))?;
                }
            }
        }
    }
    if walled > 0 {
        let troops = if walled == 1 { "a troop".to_string() } else { format!("{walled} troops") };
        cx.note(Some(h), || format!("The Protector's Wall keeps {troops} off {}", hex_name(&w, h)));
    }
    Ok(placed)
}

/// Whether a removal of `ambassadors` or `troops` has a piece to take on hex
/// `h` (F64: a card that names no troop can't touch one).
pub fn can_damage(g: &GameState, h: usize, ambassadors: i64, troops: i64) -> bool {
    (ambassadors > 0 && g.tokens(h, ADV_AMBASSADOR) > 0)
        || (troops > 0 && g.tokens(h, TROOP) > 0)
}

/// Whether a "remove everything on the hex" has a piece to take on `h`.
pub fn can_destroy_all(g: &GameState, h: usize) -> bool {
    can_damage(g, h, 1, 1)
}

/// In hex `h`, removes up to `ambassadors` agents or up to
/// `troops` troops: one kind, seat `seat`'s choice where the hex holds both.
pub fn damage(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize, ambassadors: i64, troops: i64) -> Res {
    let w = cx.world;
    let kinds: Vec<(&str, i64)> = [(ADV_AMBASSADOR, ambassadors), (TROOP, troops)]
        .into_iter()
        .filter(|(k, n)| *n > 0 && g.tokens(h, k) > 0)
        .collect();
    let chosen = match kinds.as_slice() {
        [] => return Ok(()),
        [only] => *only,
        _ => {
            let opts = kinds
                .iter()
                .map(|(k, n)| {
                    // 235: "remove 3 agent" read wrong: the shown count sets the plural.
                    let m = (*n).min(g.tokens(h, k));
                    opt(format!("remove {m} {}{} at {}", piece_name(k), if m == 1 { "" } else { "s" }, hex_name(&w, h)))
                })
                .collect();
            let prompt = || format!("Seat {seat}: remove which pieces?");
            let Some(i) = cx.choose(seat, prompt, opts)? else {
                return Ok(());
            };
            kinds[i]
        }
    };
    for _ in 0..chosen.1.min(g.tokens(h, chosen.0)) {
        remove_piece(g, cx, Some(seat), h, chosen.0);
    }
    Ok(())
}

/// Removes one adversary piece of `kind` from hex `h`. A removed troop
/// sways toward Unity.
pub fn remove_piece(g: &mut GameState, cx: &mut Cx, seat: Option<usize>, h: usize, kind: &str) {
    if kind == TROOP {
        cx.add_tokens(g, h, TROOP, -1);
        cx.tally(g, "adversary.troops_removed", 1);
        if seat.is_some() {
            cx.per_round(g, "map.troops_removed", 1);
        }
        alignment::for_troop_removed(g, cx, seat);
    } else {
        remove(g, cx, h, 1);
        cx.tally(g, "adversary.ambassadors_removed", 1);
    }
}

/// Removes every agent and troop in hex `h`.
pub fn destroy_all(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let n = g.tokens(h, ADV_AMBASSADOR);
    for _ in 0..n {
        remove_piece(g, cx, Some(seat), h, ADV_AMBASSADOR);
    }
    for _ in 0..g.tokens(h, TROOP) {
        remove_piece(g, cx, Some(seat), h, TROOP);
    }
}
