//! What the board shows: countries, flags, projects, arrows, actions and seats.

use super::*;

pub(super) fn flag_doc(f: RoundFlag) -> &'static str {
    match f {
        RoundFlag::Blackout => "Blackout: while this is up, cities here make no energy in production (an uncountered threat's lasts to the end of the next round)",
        RoundFlag::Sabotage => "Sabotage: while this is up, players with presence here make no energy in upkeep (an uncountered threat's lasts to the end of the next round)",
        RoundFlag::Shelter => "Sheltered: buildings and presence here can't be destroyed this round",
        RoundFlag::Embargo => "Embargo: while this is up, no site here can be claimed (an uncountered threat's lasts to the end of the next round)",
    }
}

pub(super) fn board_view(g: &GameState, w: &World, out: &mut ViewOut) {
    let flags: Vec<String> = FLAGS
        .iter()
        .map(|f| format!("board.{}.", f.name()))
        .collect();
    let mut keys: Vec<&str> = flags.iter().map(String::as_str).collect();
    keys.extend(["board.project.", "board.project_done.", "board.track.", "board.track2.", "board.rich.", "board.placed."]);
    out.bookkeeping(&keys);
    for c in 0..w.map.countries.len() {
        for f in FLAGS {
            if flagged(g, f, c) {
                out.country(c, f.name(), f.name(), 1, None, flag_doc(f));
            }
        }
        let p = &w.content.map.countries[c].megaproject;
        let done = g.var(&project_done(c)) > 0;
        let total: i64 = g.var(&project_key(c, None))
            + (0..g.seats.len())
                .map(|s| g.var(&project_key(c, Some(s))))
                .sum::<i64>();
        let doc = format!(
            "{}: {} ({} cubes; each player cube pays {} credits, every {} adversary cubes 1 Discord)",
            p.name,
            p.text,
            p.cubes,
            w.int("board.project_credits"),
            w.int("board.project_discord_per")
        );
        out.country(
            c,
            "project",
            &format!("megaproject: {}", p.name),
            if done { p.cubes } else { total },
            Some(p.cubes),
            &doc,
        );
        if !done {
            out.country(
                c,
                "project.adversary",
                "adversary cubes",
                g.var(&project_key(c, None)),
                None,
                "adversary cubes on the megaproject",
            );
            for s in 0..g.seats.len() {
                out.country(
                    c,
                    &format!("project.{s}"),
                    &format!("seat {s} cubes"),
                    g.var(&project_key(c, Some(s))),
                    None,
                    "this player's cubes on the megaproject",
                );
            }
        } else {
            out.country(
                c,
                "project.done",
                "completed",
                1,
                None,
                "owned stations here produce the project's benefit",
            );
        }
    }
    let step = clock_step(w, g.round);
    let name = |k: i64| match k & 3 {
        1 => "production",
        2 => "war",
        3 => "production + war",
        _ => "—",
    };
    let track: Vec<String> = (1..=8u32)
        .map(|r| format!("{r}: {}", name(clock_step(w, r))))
        .collect();
    out.global(
        "clock",
        name(step),
        step,
        None,
        &format!("the production/war clock (1 production, 2 war, 3 both); this round's step; track {}…", track.join(", ")),
    );
    let war = war_round(w, g) as i64;
    out.global(
        "war",
        "war step",
        war,
        None,
        "1 on a war step (the clock or a War card): in production the fronts fight",
    );
    out.global(
        "fronts",
        "fronts",
        w.map.fronts.len() as i64,
        None,
        "printed borders between countries, each with a base on either side",
    );
    // Each adversary troop's next target (019): players see each hit coming.
    for h in 0..g.hexes.len() {
        let n = g.tokens(h, TROOP);
        if n <= 0 {
            continue;
        }
        if let Some((t, rounds)) = troop_plan(w, g, h) {
            let what = troop_targets_on(w, g, t)
                .iter()
                .map(|k| piece_name(k))
                .collect::<Vec<_>>()
                .join(", ");
            let when = if rounds <= 1 {
                "strikes it in the next adversary phase".to_string()
            } else {
                format!("strikes it in {rounds} adversary phases unless stopped")
            };
            let doc = format!(
                "adversary troop{} at {} heading for {what} at {}: {when} (players choose among equally near targets)",
                if n > 1 { "s" } else { "" },
                hex_name(w, h),
                hex_name(w, t)
            );
            out.arrow("troop", h, t, n, rounds, &doc);
        }
    }
    for act in STANDARD {
        let price = printed_price(w, act);
        let Price { energy: e, credits: cr, chips } = price;
        let cost = price.label();
        let doc = format!(
            "{}; costs {}",
            std_text(w, act),
            if cost.is_empty() {
                "nothing".into()
            } else {
                cost.join(" + ")
            }
        );
        out.standard(act.name(), e, cr, chips, &doc);
    }
    for s in 0..g.seats.len() {
        // 034: what a presence costs this seat now, rise and step cost in.
        out.seat(
            s,
            "presence_energy_now",
            "next presence (energy)",
            seat_price(w, g, s, StandardAction::Presence).energy,
            None,
            "the standard action at your cheapest track: printed price plus the step's cost",
        );
        out.seat(
            s,
            "presence_credits_now",
            "next rich action (credits)",
            seat_price(w, g, s, StandardAction::PresenceBuy).credits,
            None,
            "printed price, plus its rise this round and the step's cost",
        );
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
            "presence",
            "presence on the board",
            presence_count(g, s),
            None,
            "your presence: you act within 1 of it, and claim sites on or next to it",
        );
        let b = ai::board(w, s);
        let left = b.presence.len().saturating_sub(track_taken(g, s));
        out.seat(
            s,
            "presence_track",
            "presence left on track 1",
            left as i64,
            None,
            "presence tokens still on your AI board's first track (the second shows on the board)",
        );
    }
}
