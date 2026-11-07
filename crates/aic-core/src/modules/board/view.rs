//! What the board shows: countries, flags, projects, arrows, actions and seats.

use super::*;

fn flag_doc(f: RoundFlag) -> &'static str {
    match f {
        RoundFlag::Blackout => "Blackout: while this is up, cities here make no energy in production (an uncountered threat's lasts to the end of the next round)",
        RoundFlag::Sabotage => "Sabotage: while this is up, players with a node here make no energy in upkeep (an uncountered threat's lasts to the end of the next round)",
        RoundFlag::Shelter => "Sheltered: facilities and nodes here can't be destroyed this round",
        RoundFlag::Embargo => "Embargo: while this is up, no site here can be claimed (an uncountered threat's lasts to the end of the next round)",
        RoundFlag::Cordon => "Cordon: no arrivals land here this round",
        RoundFlag::PowerDown => "Power down: players near it can't use Actions that cost energy",
        RoundFlag::StationOutage => "Station outage: players near it make no research cubes",
        RoundFlag::DoubleBuild => "Blockade: the next adversary build on or next to it happens twice",
        RoundFlag::StationDark => "Stations dark: research stations on or next to it make no research cubes",
        RoundFlag::GridDown => "Grid down: cities and stations on or next to it produce nothing",
    }
}

pub(super) fn board_view(g: &GameState, w: &World, out: &mut ViewOut) {
    let flags: Vec<String> = FLAGS
        .iter()
        .map(|f| format!("board.{}.", f.name()))
        .collect();
    let mut keys: Vec<&str> = flags.iter().map(String::as_str).collect();
    keys.extend(["board.project.", "board.project_done.", "board.project_paid.", "board.converter_round.", "board.track.", "board.track2."]);
    // 135: hex and seat flags, standing troops, occupations and registrations.
    keys.extend([
        HEX_FLAGS,
        SEAT_FLAGS,
        "board.stand.",
        "board.occupied.",
        "board.corp_extra.",
        "board.extra_war.",
        "board.new_factory.",
        "board.destroyed_at.",
        "board.landed_at.",
        "board.landed_n.",
    ]);
    out.bookkeeping(&keys);
    for c in 0..w.map.countries.len() {
        for f in FLAGS {
            if flagged(g, f, c) {
                // 118: a Cordon at 2 (played this round, or renewed) lasts through the next.
                let doc = if f == RoundFlag::Cordon && g.var(&flag(f, c)) >= 2 {
                    "Cordon: no arrivals land here this round or next"
                } else {
                    flag_doc(f)
                };
                out.country(c, f.name(), f.name(), 1, None, doc);
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
                    "dark: an adversary ambassador stands on {}: no energy for anyone there (sold or to its owner) while it stays",
                    dark.join(", ")
                ),
            );
        }
        let p = &w.content.map.countries[c].megaproject;
        let done = g.var(&project_done(c)) > 0;
        let total: i64 = g.var(&project_key(c, None))
            + g.var(&project_neutral(c))
            + (0..g.seats.len())
                .map(|s| g.var(&project_key(c, Some(s))))
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
                "project.adversary",
                "adversary cubes",
                g.var(&project_key(c, None)),
                None,
                "adversary cubes on the megaproject",
            );
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
                    g.var(&project_key(c, Some(s))),
                    None,
                    "this player's cubes on the megaproject",
                );
            }
        } else {
            out.country(c, "project.done", "completed", 1, None, &p.text);
            let paid = |color: &str| g.var(&project_paid(c, color));
            let (mine, adv, neutral) = (paid("player"), paid("adversary"), paid("neutral"));
            out.country(
                c,
                "project.paid",
                &format!("{mine} player, {adv} adversary, {neutral} neutral"),
                mine + adv + neutral,
                None,
                "the cubes on the card when it completed: every one paid, overflow included",
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
        // 105: a cube cost (the general converter) reaches the client in its words.
        let Price { energy: e, credits: cr, chips, ambassadors, cubes: _ } = price;
        let cost = price.label();
        // 099: the Harden action a card grants costs that card's chips, not nothing.
        let card_priced = act == StandardAction::HardenChips;
        let doc = format!(
            "{}; costs {}",
            std_text(w, act),
            if card_priced {
                "the chips the card prints".into()
            } else if cost.is_empty() {
                "nothing".into()
            } else {
                cost.join(" + ")
            }
        );
        out.standard(act.name(), e, cr, chips, ambassadors, &doc);
        if card_priced {
            out.standard_seat_chips((0..g.seats.len()).map(|s| seat_price(w, g, s, act).chips).collect());
        }
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
            "node on the board",
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
            let blocked = converter_blocked(w, g, s, c);
            let label = match blocked {
                Some(why) => format!("{} converter: {why}", country_name(w, c)),
                None => format!("{} converter", country_name(w, c)),
            };
            out.seat(
                s,
                &format!("converter.{c}"),
                &label,
                blocked.is_none() as i64,
                None,
                // 118: `words` says "(once a round)" where the converter is (Onyx's); the rest have no limit.
                &format!("{}, with a node in a city of {}", m.converter.words(), country_name(w, c)),
            );
        }
        let b = ai::board(w, s);
        let left = b.economy_track.len().saturating_sub(track_taken(g, s));
        out.seat(
            s,
            "node_track",
            "node left on the economy track",
            left as i64,
            None,
            "nodes still on your AI board's economy track (the card-play track shows on the board)",
        );
    }
}
