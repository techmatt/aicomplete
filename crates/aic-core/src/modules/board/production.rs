//! Production: units, factories, chips, city energy, research cubes and megaprojects.

use super::*;

/// The resources a seat is measured on at the end of each action phase (068).
pub const LEFT: [Resource; 5] =
    [Resource::Credits, Resource::Chips, Resource::Energy, Resource::Antimatter, Resource::Ambassadors];

fn add_unit(g: &mut GameState, cx: &mut Cx, base: usize) {
    let w = cx.world;
    if g.tokens(base, UNIT) < w.int("board.unit_cap") {
        cx.add_tokens(g, base, UNIT, 1);
        cx.tally(g, "board.units_built", 1);
    }
}

/// Production step 1, on a production step of the clock: bases, then
/// factories, make their corp's units (a factory 1 at each of its corp's
/// bases); an adversary ambassador on the piece diverts it and is removed, a
/// player's diverts it and stays (068: a position); an adversary-owned factory
/// makes an adversary troop. Player-owned factories
/// make chips. Then the cities grow factories.
pub(super) fn produce_units(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    // 068: what each seat left unspent, the measure's: not a rule.
    for s in 0..g.seats.len() {
        for r in LEFT {
            cx.per_round(g, &format!("unspent.{s}.{r}"), g.resource(s, r));
        }
        ai::tally_dear_powers(g, cx, s);
        // 078: the seat's most-held tag on its cards in play, the measure's (tags by round).
        let top = crate::modules::cards::tag_counts(&w, g, s).into_iter().max().unwrap_or(0);
        cx.per_round(g, &format!("ai.tag_top.{s}"), top);
        // 074: converters a seat holding cubes left unused, and why: the measure's.
        if g.resource(s, Resource::ResearchCubes) > 0 {
            for c in 0..w.map.countries.len() {
                match converter_blocked(&w, g, s, c) {
                    None => cx.tally(g, "board.converter_idle", 1),
                    Some("no presence here") => cx.tally(g, "board.converter_no_city", 1),
                    Some("can't pay it") => cx.tally(g, "board.converter_unpaid", 1),
                    Some(_) => {}
                }
            }
        }
    }
    if !production_round(&w, g) {
        return Ok(());
    }
    base_units(g, cx);
    factory_output(g, cx);
    grow_factories(g, cx)
}

/// Bases make their corp's units, or an ambassador diverts them: the adversary's
/// is used up, a player's stays (068).
fn base_units(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        if !is_base(&w, h) {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            cx.tally(g, "board.bases_diverted", 1);
            alignment::discord_from(g, cx, "base", w.int("board.base_discord"));
        } else if player_ambassador(g, h).is_some() {
            cx.tally(g, "board.bases_united", 1);
            alignment::sway(g, cx, Side::Unity, w.int("board.base_unity"));
        } else {
            add_unit(g, cx, h);
        }
    }
}

/// Factories make chips for their owners, then units or an adversary troop.
fn factory_output(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let per = w.int("board.factory_units");
    let chips = w.int("board.factory_chips");
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 {
            continue;
        }
        if let Some(s) = cube_owner(g, h) {
            cx.add_resource(g, s, Resource::Chips, chips);
            cx.tally(g, "board.chips_made", chips);
        }
        if per == 0 {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 || corrupted(g, h) {
            // 028: an adversary-owned factory uses its ambassador up too.
            if g.tokens(h, ADV_AMBASSADOR) > 0 {
                adversary::remove(g, cx, h, 1);
            } else if player_ambassador(g, h).is_some() {
                // 022 (D1): a player ambassador on an adversary-owned factory
                // sways Unity; the troop is still made (locked). 068: it stays.
                alignment::sway(g, cx, Side::Unity, w.int("board.factory_unity"));
            }
            for _ in 0..per {
                adversary::place_troop(g, cx, h);
            }
            cx.tally(g, "board.troops_built", per);
            cx.note(Some(h), || {
                format!(
                    "The factory at {} makes an adversary troop",
                    hex_name(&w, h)
                )
            });
        } else if player_ambassador(g, h).is_some() {
            alignment::sway(g, cx, Side::Unity, w.int("board.factory_unity"));
        } else {
            let c = w.map.hexes[h].country;
            let bases: Vec<usize> = (0..g.hexes.len())
                .filter(|&b| is_base(&w, b) && w.map.hexes[b].country == c)
                .collect();
            for b in bases {
                for _ in 0..per {
                    add_unit(g, cx, b);
                }
            }
        }
    }
}

/// Each city builds one factory: a corrupted city at a free zone in its
/// country next to an adversary ambassador if there is one, else the
/// lowest-numbered free zone (lowest number among equals); any other city on
/// a free zone next to it, where the players choose (an aligned city's
/// owner, else seat 0). A corrupted city's factory is the adversary's, an
/// aligned city's its owner's, and a clean city's goes to the player with
/// the most presence next to the zone (players choose on a tie), neutral if
/// none.
fn grow_factories(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for city in 0..g.hexes.len() {
        if !is_city(&w, city) {
            continue;
        }
        let zones = free_zones(&w, g);
        let at = if corrupted(g, city) {
            let c = w.map.hexes[city].country;
            zones
                .into_iter()
                .filter(|&z| w.map.hexes[z].country == c)
                .min_by_key(|&z| {
                    (
                        !next_to(&w, z, |x| g.tokens(x, ADV_AMBASSADOR) > 0),
                        w.map.hexes[z].number,
                    )
                })
        } else {
            let near: Vec<usize> = zones
                .into_iter()
                .filter(|&z| w.map.adjacent(city, z))
                .collect();
            if near.is_empty() {
                None
            } else {
                let who = cube_owner(g, city).unwrap_or(0);
                let opts = near.iter().map(|&z| hex_opt(&w, z)).collect();
                let prompt = format!("{} builds a factory on which zone?", hex_name(&w, city));
                cx.choose(who, &prompt, opts)?.map(|i| near[i])
            }
        };
        let Some(z) = at else {
            continue;
        };
        cx.add_tokens(g, z, FACTORY, 1);
        if corrupted(g, city) {
            cx.add_tokens(g, z, ADV_CUBE, 1);
            // 059: corruption replaces any marker here, as on a city.
            adversary::clear_markers_on(g, cx, z);
            cx.tally(g, "board.factories_built.adversary", 1);
        } else if let Some(s) = match cube_owner(g, city) {
            Some(s) => Some(s),
            None => zone_owner(g, cx, z)?,
        } {
            cx.add_tokens(g, z, cube(s), 1);
            cx.tally(g, "board.factories_built.player", 1);
        } else {
            cx.tally(g, "board.factories_built.neutral", 1);
        }
        cx.note(Some(z), || {
            format!(
                "{} builds a factory at {}",
                hex_name(&w, city),
                hex_name(&w, z)
            )
        });
    }
    Ok(())
}

/// Who takes a clean city's new factory on zone `z`: the player with the
/// most presence next to it (players choose on a tie), none if nobody has any.
fn zone_owner(g: &mut GameState, cx: &mut Cx, z: usize) -> Res<Option<usize>> {
    let w = cx.world;
    let hexes = within(&w, z, 1);
    let near: Vec<usize> = (0..g.seats.len())
        .map(|s| hexes.iter().filter(|&&x| g.tokens(x, presence(s)) > 0).count())
        .collect();
    let most = near.iter().copied().max().unwrap_or(0);
    if most == 0 {
        return Ok(None);
    }
    let tied: Vec<usize> = (0..g.seats.len()).filter(|&s| near[s] == most).collect();
    if tied.len() == 1 {
        return Ok(Some(tied[0]));
    }
    let opts = tied.iter().map(|&s| seat_opt(s)).collect();
    let prompt = format!("The factory at {} goes to which player?", hex_name(&w, z));
    Ok(cx.choose(0, &prompt, opts)?.map(|i| tied[i]))
}

/// Production step 3: cities make energy, stations research cubes; an
/// adversary ambassador on the piece diverts its output and is removed, and
/// diverted energy sways toward Discord. Then an adversary ambassador on a
/// lone presence drains 1 of its owner's next insight into adversary compute,
/// and last full megaprojects complete.
pub(super) fn produce(g: &mut GameState, cx: &mut Cx) -> Res {
    city_energy(g, cx)?;
    station_cubes(g, cx)?;
    presence_leak(g, cx)?;
    complete_projects(g, cx)
}

/// Cities make energy: diverted, free to an aligned city's owner, or for sale.
pub(super) fn city_energy(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let divert = w.int("board.divert_discord");
    let price = w.int("board.city_energy_credits");
    for h in 0..g.hexes.len() {
        if !is_city(&w, h) {
            continue;
        }
        let c = w.map.hexes[h].country as usize;
        let energy = {
            let l = w.list("board.corp_energy");
            l[c % l.len()]
        };
        if flagged(g, RoundFlag::Blackout, c) {
            continue;
        }
        if corrupted(g, h) {
            alignment::discord_from(g, cx, "city", energy * divert);
            cx.tally(g, "board.energy_diverted", energy);
        } else if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            alignment::sway(g, cx, Side::Discord, energy * w.int("board.ambassador_discord"));
            cx.tally(g, "board.energy_diverted", energy);
        } else if let Some(s) = cube_owner(g, h) {
            // An aligned city's cube goes free to the player who aligned it.
            cx.add_resource(g, s, Resource::Energy, energy + w.int("board.aligned_bonus"));
        } else {
            // A neutral city's cube: a player with presence next to it may buy
            // it (players choose who), otherwise it is lost (021).
            let buyers: Vec<usize> = (0..g.seats.len())
                .filter(|&s| g.resource(s, Resource::Credits) >= price && in_reach(&w, g, s, h))
                .collect();
            if buyers.is_empty() {
                continue;
            }
            let mut opts: Vec<_> = buyers.iter().map(|&s| opt(format!("seat {s} buys"))).collect();
            opts.push(opt("nobody".to_string()));
            let prompt = format!(
                "The energy of {} ({energy}) can be bought for {price} credits: by whom?",
                hex_name(&w, h)
            );
            if let Some(i) = cx.choose(0, &prompt, opts)? {
                if let Some(&s) = buyers.get(i) {
                    cx.add_resource(g, s, Resource::Credits, -price);
                    cx.add_resource(g, s, Resource::Energy, energy);
                    cx.tally(g, "board.city_energy_bought", energy);
                }
            }
        }
    }
    Ok(())
}

/// Whose cube a station makes this round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Seat(usize),
    Adversary,
    Neutral,
}

/// 074: stations make research cubes in their owner's color: onto their
/// country's megaproject until it completes, then onto their owner's board, or
/// the adversary's convert at once. An adversary ambassador takes the round's
/// cube and is used up, on a neutral station too.
pub(super) fn station_cubes(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let cubes = w.int("board.station_cubes");
    for h in 0..g.hexes.len() {
        if g.tokens(h, STATION) == 0 {
            continue;
        }
        let c = w.map.hexes[h].country as usize;
        let color = if g.tokens(h, ADV_AMBASSADOR) > 0 {
            adversary::remove(g, cx, h, 1);
            Color::Adversary
        } else if corrupted(g, h) {
            // 068: a player ambassador on a corrupted station diverts its cube
            // to that player, and stays.
            player_ambassador(g, h).map_or(Color::Adversary, Color::Seat)
        } else {
            cube_owner(g, h).map_or(Color::Neutral, Color::Seat)
        };
        if g.var(&project_done(c)) == 0 {
            let key = match color {
                Color::Seat(s) => project_key(c, Some(s)),
                Color::Adversary => project_key(c, None),
                Color::Neutral => project_neutral(c),
            };
            cx.add_key(g, &key, cubes);
            continue;
        }
        match color {
            Color::Seat(s) => {
                cx.add_resource(g, s, Resource::ResearchCubes, cubes);
                cx.tally(g, "board.cubes_stored", cubes);
            }
            Color::Adversary => {
                let after = &w.content.map.countries[c].megaproject.after;
                cx.tally(g, "board.cubes_after", cubes);
                cx.tally(g, &format!("board.cubes_after.{c}"), cubes);
                for _ in 0..cubes {
                    effects::run(g, cx, after, &mut effects::Ctx::team().named("megaproject"))?;
                }
                if g.is_over() {
                    return Ok(());
                }
            }
            Color::Neutral => {}
        }
    }
    Ok(())
}

/// An adversary ambassador on a presence (with no other piece to divert)
/// costs its owner 1 insight next round and adds 1 compute to the cheapest
/// adversary project; it stays until removed (068; 021: it was used up).
fn presence_leak(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        let Some(s) = presence_owner(g, h) else {
            continue;
        };
        // (020: a Sabotage flag used to skip the leak here, a data-center-era
        // leftover the rules don't have; once F1 let the flag reach production
        // it would have helped the players.)
        if g.tokens(h, ADV_AMBASSADOR) == 0 {
            continue;
        }
        // Only a lone presence: on a city, base, station or factory the
        // ambassador belongs to that piece (020: the leak drained a corrupted
        // city's ambassadors so it never spread, and ate those a clean city or
        // a base was gathering).
        if is_city(&w, h) || is_base(&w, h) || has_building(g, h) {
            continue;
        }
        cx.add_key(g, &economy::insight_debt(s), 1);
        cx.tally(g, "board.presence_drained", 1);
        adversary::gain_compute(g, cx, 1)?;
    }
    Ok(())
}

/// 074: checked once, at the end of production: a megaproject holding its
/// cubes or more completes. Every cube on it pays, overflow included, and is
/// spent: a player cube pays its owner, an adversary cube the adversary, a
/// neutral cube nothing.
fn complete_projects(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        if g.var(&project_done(c)) > 0 {
            continue;
        }
        let p = &w.content.map.countries[c].megaproject;
        let adv = g.var(&project_key(c, None));
        let neutral = g.var(&project_neutral(c));
        let mine: Vec<i64> = (0..g.seats.len()).map(|s| g.var(&project_key(c, Some(s)))).collect();
        let paid: i64 = mine.iter().sum();
        if adv + neutral + paid < p.cubes {
            continue;
        }
        for s in 0..mine.len() {
            cx.set_key(g, &project_key(c, Some(s)), 0);
        }
        cx.set_key(g, &project_key(c, None), 0);
        cx.set_key(g, &project_neutral(c), 0);
        cx.set_key(g, &project_done(c), 1);
        cx.tally(g, "board.projects_completed", 1);
        cx.tally(g, &format!("board.completed_round.{}", g.round), 1);
        cx.tally(g, &format!("board.completed.{c}"), 1);
        for (color, n) in [("player", paid), ("adversary", adv), ("neutral", neutral)] {
            cx.tally(g, &format!("board.cubes_paid.{color}"), n);
            cx.set_key(g, &project_paid(c, color), n);
        }
        cx.note(None, || {
            format!("{} completes: {} cubes pay ({paid} player, {adv} adversary, {neutral} neutral)", p.name, paid + adv + neutral)
        });
        for (s, &n) in mine.iter().enumerate() {
            for _ in 0..n {
                effects::run(g, cx, &p.player, &mut effects::Ctx::seat(s))?;
            }
        }
        for _ in 0..adv {
            effects::run(g, cx, &p.adversary, &mut effects::Ctx::team().named("megaproject"))?;
        }
        if g.is_over() {
            return Ok(());
        }
    }
    Ok(())
}
