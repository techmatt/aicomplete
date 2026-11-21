//! What the board shows: countries, flags, projects, arrows, actions and seats.

use super::*;

fn flag_doc(f: RoundFlag) -> &'static str {
    match f {
        // 261: an adversary card makes these (a project, an event, the Militarist's Naval Blockade; no threat does),
        // and it says how long; nothing in the content makes an Embargo.
        // 285, 293: a blacked-out corrupted city makes no Discord either (production.rs `corrupted_cities_discord`).
        RoundFlag::Blackout => "Blackout: while this is up, cities here make no energy in production, and corrupted ones no Discord in the adversary's Income beat (until the end of the next round, as its card says)",
        RoundFlag::Sabotage => "Sabotage: while this is up, players with a node here make no energy in upkeep (until the end of the next round, as its card says)",
        RoundFlag::Embargo => "Embargo: while this is up, no site here can be claimed (until the end of the next round)",
        // 285, 293: every agent arrival, whatever line or push sends it.
        RoundFlag::Cordon => "Cordon: until the end of the next adversary turn no agent lands there (any line, an Escalate or a factory's push included), and its own factories neither push nor make agents (troops still land)",
    }
}

pub(super) fn board_view(g: &GameState, w: &World, out: &mut ViewOut) {
    let flags: Vec<String> = FLAGS
        .iter()
        .map(|f| format!("board.{}.", f.name()))
        .collect();
    let mut keys: Vec<&str> = flags.iter().map(String::as_str).collect();
    keys.extend(KEYED);
    // 135: hex flags, standing troops, occupations and registrations.
    keys.extend(MARKS);
    out.bookkeeping(&keys);
    for s in 0..g.seats.len() {
        // r142: the seat's ambassadors on the board that still have their action this round.
        let on = ambassadors_on_board(g, s);
        let ready: i64 = (0..g.hexes.len()).map(|h| ready_ambassadors(g, s, h)).sum();
        let doc = "ready ambassadors: yours on the board that have not moved or acted this round (a tipped one has); each moves or acts once a round";
        out.seat(s, "ambassadors_ready", "ready ambassadors", ready, Some(on), doc);
        // 235: a card's reach Shelter (r106) lives in hex flags the view hides as bookkeeping: say it on the seat.
        // It covers the seat's own pieces in its reach as it stands now (238).
        let mine = |h: usize| {
            g.tokens(h, node(s)) > 0
                || (g.tokens(h, cube(s)) > 0 && (g.tokens(h, STATION) > 0 || g.tokens(h, FACTORY) > 0))
        };
        let sheltered: Vec<String> = (0..g.hexes.len())
            .filter(|&h| reach_sheltered(w, g, s, h) && mine(h))
            .map(|h| hex_name(w, h))
            .collect();
        if !sheltered.is_empty() {
            let doc = format!(
                "Sheltered this round: your nodes and facilities on {} can't be destroyed",
                sheltered.join(", ")
            );
            out.seat(s, "sheltered", "sheltered hexes", sheltered.len() as i64, None, &doc);
        }
    }
    for c in 0..w.map.countries.len() {
        for f in FLAGS {
            if flagged(g, f, c) {
                let doc = flag_doc(f);
                // 148: the label is the doc's name ("Sheltered"), not the key.
                let label = doc.split(':').next().unwrap_or(doc);
                out.country(c, f.name(), label, 1, None, doc);
            }
        }
        // 102 (D29): dark cities, named, so a dark city reads as one off the hover.
        let dark: Vec<String> = w.map.countries[c]
            .hexes
            .iter()
            .filter(|&&h| is_city(w, h) && !corrupted(g, h) && g.tokens(h, ADV_AMBASSADOR) > 0)
            .map(|&h| hex_name(w, h))
            .collect();
        if !dark.is_empty() {
            out.country(
                c,
                "dark",
                "dark cities",
                dark.len() as i64,
                None,
                &format!(
                    "dark: an agent stands on {}: no energy for anyone there while it stays",
                    dark.join(", ")
                ),
            );
        }
        let p = &w.content.map.countries[c].megaproject;
        let done = g.var(&project_done(c)) > 0;
        let total: i64 = g.var(&project_neutral(c))
            + (0..g.seats.len())
                .map(|s| g.var(&project_key(c, s)))
                .sum::<i64>();
        out.country(
            c,
            "project",
            &format!("megaproject: {}", p.name),
            if done { p.cubes } else { total },
            Some(p.cubes),
            &format!("{}: {} ({} cubes)", p.name, p.text, p.cubes),
        );
        if !done {
            out.country(
                c,
                "project.neutral",
                "neutral cubes",
                g.var(&project_neutral(c)),
                None,
                "neutral cubes on the megaproject: they fill it and pay nothing",
            );
            for s in 0..g.seats.len() {
                out.country(
                    c,
                    &format!("project.{s}"),
                    &format!("seat {s} cubes"),
                    g.var(&project_key(c, s)),
                    None,
                    "this player's cubes on the megaproject",
                );
            }
        } else {
            out.country(c, "project.done", "completed", 1, None, &p.text);
            let paid = |color: &str| g.var(&project_paid(c, color));
            let (mine, neutral) = (paid("player"), paid("neutral"));
            out.country(
                c,
                "project.paid",
                &format!("{mine} player, {neutral} neutral"),
                mine + neutral,
                None,
                "the cubes on the card when it completed: every one paid, overflow included",
            );
        }
    }
    // r141: the Strike and Redeploy icons shown a round ahead, and each garrison: what it guards and what it
    // would strike (019: players see each hit coming).
    let strike = adversary::line_ahead(w, g, &Op::Strike) || adversary::strike_slot_ahead(w, g);
    let redeploy = adversary::line_ahead(w, g, &Op::Redeploy);
    let troops: i64 = (0..g.hexes.len()).map(|h| g.tokens(h, TROOP)).sum();
    if troops > 0 || strike || redeploy {
        out.global(
            "strike_ahead",
            "Strike ahead",
            strike as i64,
            Some(1),
            "1: a Strike comes in the next adversary turn: the next card shows a Strike icon (in its Card beat), or the research track reaches its Strike slot at the next check (in the Income beat); every troop destroys one player node, research station or factory on or next to it",
        );
        out.global(
            "redeploy_ahead",
            "Redeploy ahead",
            redeploy as i64,
            Some(1),
            "1: the next adversary Card beat shows a Redeploy icon: each troop moves onto its country's corrupted city nearest a player node or ambassador (one already there stays)",
        );
    }
    // The guarding rule, said to the seat it stops: the corrupted sites in its reach that only a troop keeps it
    // from liberating (no agent on them).
    for s in 0..g.seats.len() {
        let refused = (0..g.hexes.len())
            .filter(|&h| {
                corrupted(g, h)
                    && site_kind(w, g, h).is_some()
                    && g.tokens(h, ADV_AMBASSADOR) == 0
                    && troop_guarded(w, g, h)
                    && (in_reach(w, g, s, h) || g.tokens(h, ambassador(s)) > 0)
            })
            .count() as i64;
        if refused > 0 {
            out.seat(
                s,
                "liberate_refused",
                "guarded sites Liberate can't take",
                refused,
                None,
                "corrupted sites in your reach, or holding your ambassador, that can't be liberated: a troop is on or next to each (remove the troop or push it away first)",
            );
        }
    }
    for h in 0..g.hexes.len() {
        if g.tokens(h, TROOP) <= 0 {
            continue;
        }
        let all = g.tokens(h, TROOP);
        let s = if all > 1 { "s" } else { "" };
        for x in guarded_sites(w, g, h) {
            let doc = format!(
                "adversary troop{s} at {} guard{} the corrupted {} at {}: it can't be liberated while a troop is on or next to it",
                hex_name(w, h),
                if all > 1 { "" } else { "s" },
                site_kind(w, g, x).map_or("site", |k| k.name()),
                hex_name(w, x)
            );
            out.arrow("guard", h, x, all, 0, &doc);
        }
        // 145: standing troops never strike (troops.md): no strike arrow for them.
        let n = all - standing(g, h);
        if n <= 0 {
            continue;
        }
        for t in strike_targets(w, g, h) {
            let what = troop_targets_on(w, g, t).iter().map(|k| piece_name(k)).collect::<Vec<_>>().join(", ");
            let when = if strike { "a Strike is shown: it strikes in the next adversary turn" } else { "no Strike is shown yet" };
            let doc = format!(
                "adversary troop{s} at {} can strike {what} at {}: {when} (one target a troop, the players choose)",
                hex_name(w, h),
                hex_name(w, t)
            );
            out.arrow("troop", h, t, n, if strike { 1 } else { 2 }, &doc);
        }
    }
    // 293: each industrial zone's printed arrow to its city (kind `zone`: `count` the agents a corrupted factory
    // there holds to push, `rounds` 1 when the push lands in the next Income beat, else 0: the printed arrow alone).
    for h in 0..g.hexes.len() {
        if !w.map.hexes[h].industrial {
            continue;
        }
        let Some(t) = super::super::plan::factory_arrow(w, h) else {
            continue;
        };
        let live = g.tokens(h, FACTORY) > 0 && corrupted(g, h);
        let n = if live { g.tokens(h, ADV_AMBASSADOR) } else { 0 };
        // As `adversary::factories_output` reads it: an occupying troop, a player ambassador or a cordon stops it.
        let soon = n > 0
            && super::super::plan::push_lands(w, g, t)
            && player_ambassador(g, h).is_none()
            && !occupied(g, h)
            && !super::super::plan::cordoned(w, g, h);
        let doc = if soon {
            format!(
                "the corrupted factory at {} pushes its agent along its arrow onto {} in the adversary's next Income beat, then makes another",
                hex_name(w, h),
                hex_name(w, t)
            )
        } else if live {
            format!(
                "the corrupted factory at {} makes 1 agent a round and pushes it along its arrow onto {} (not while that city is the adversary's)",
                hex_name(w, h),
                hex_name(w, t)
            )
        } else {
            format!(
                "industrial zone {}: its printed arrow points to {}: a corrupted factory here pushes its agents there",
                hex_name(w, h),
                hex_name(w, t)
            )
        };
        out.arrow("zone", h, t, n, i64::from(soon), &doc);
    }
    for act in STANDARD {
        // 206: a cut action (its price param 0) is not published.
        if !std_offered(w, act) {
            continue;
        }
        let price = printed_price(w, act);
        // 105: a cube cost (the general converter) reaches the client in its words.
        let Price { energy: e, credits: cr, chips, ambassadors, cubes: _ } = price;
        let cost = price.label();
        // 285: Liberate's price depends on its target (`target_price`): its text gives it; "costs nothing" contradicted it.
        let doc = if matches!(act, StandardAction::Liberate) {
            std_text(w, act).to_string()
        } else {
            format!("{}; costs {}", std_text(w, act), if cost.is_empty() { "nothing".into() } else { cost.join(" + ") })
        };
        out.standard(act.name(), e, cr, chips, ambassadors, &doc);
    }
    for s in 0..g.seats.len() {
        out.seat(
            s,
            "ambassadors_board",
            "ambassadors on the board",
            ambassadors_on_board(g, s),
            None,
            "your player ambassadors on pieces",
        );
        out.seat(
            s,
            "node",
            "nodes on the board",
            node_count(g, s),
            None,
            "your nodes: you act within 1 of it, and claim sites on or next to it",
        );
        // 074: every completed card's converter, usable or not, says why.
        for c in 0..w.map.countries.len() {
            if g.var(&project_done(c)) == 0 {
                continue;
            }
            let m = &w.content.map.countries[c].megaproject;
            // 199: the chip converter beside the cube one, where the card prints it.
            for chips in [false, true] {
                let Some(conv) = m.converter_of(chips) else { continue };
                let blocked = converter_blocked_of(w, g, s, c, chips);
                let name = if chips { "chip converter" } else { "converter" };
                let label = match blocked {
                    Some(why) => format!("{} {name}: {why}", country_name(w, c)),
                    None => format!("{} {name}", country_name(w, c)),
                };
                out.seat(
                    s,
                    &format!("{}converter.{c}", if chips { "chip_" } else { "" }),
                    &label,
                    blocked.is_none() as i64,
                    None,
                    // 118: `words` says "(once a round)" where the converter is (r75: every one).
                    &format!("{}, with a city of {} in your reach", conv.words(), country_name(w, c)),
                );
            }
        }
        let b = ai::board(w, s);
        let left = b.economy_track.len().saturating_sub(track_taken(g, s));
        out.seat(
            s,
            "node_track",
            "nodes left on the economy track",
            left as i64,
            None,
            "nodes still on your AI board's economy track (the card-play track shows on the board)",
        );
    }
}
