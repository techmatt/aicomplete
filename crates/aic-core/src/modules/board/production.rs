//! Production: factories, chips, city energy, research cubes and megaprojects.

use super::*;

/// The resources a seat is measured on at the end of each action phase (068).
pub const LEFT: [Resource; 5] =
    [Resource::Credits, Resource::Chips, Resource::Energy, Resource::Antimatter, Resource::Ambassadors];

/// The measure's tallies taken as production starts, not a rule: what each
/// seat left unspent, its dear powers, its top tag, its idle converters.
fn measure_round(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    // 190: facilities on the map by owner, and the chips waiting on cities.
    for h in 0..g.hexes.len() {
        for (token, name) in [(FACTORY, "factories"), (STATION, "stations")] {
            if g.tokens(h, token) > 0 {
                let owner = if corrupted(g, h) {
                    "adversary"
                } else if cube_owner(g, h).is_some() {
                    "player"
                } else {
                    "neutral"
                };
                cx.per_round(g, &format!("map.{name}.{owner}"), 1);
            }
        }
        let t = g.tokens(h, TROOP);
        cx.per_round(g, "map.troops", t);
        // 201: botnets, agents and player ambassadors on the map.
        let b = g.tokens(h, BOTNET);
        cx.per_round(g, "map.botnets", b);
        let a = g.tokens(h, ADV_AMBASSADOR);
        cx.per_round(g, "map.agents", a);
        let p: i64 = (0..g.seats.len()).map(|s| g.tokens(h, ambassador(s))).sum();
        cx.per_round(g, "map.ambassadors", p);
    }
    // 201: the players' ambassadors in hand (275: no agent pools).
    let held: i64 = (0..g.seats.len()).map(|s| g.resource(s, Resource::Ambassadors)).sum();
    cx.per_round(g, "map.ambassadors_held", held);
    // 201: countries where agents on the map outnumber the players' ambassadors there.
    for country in &w.map.countries {
        let hexes = &country.hexes;
        let a: i64 = hexes.iter().map(|&h| g.tokens(h, ADV_AMBASSADOR)).sum();
        let p: i64 = hexes.iter().map(|&h| (0..g.seats.len()).map(|s| g.tokens(h, ambassador(s))).sum::<i64>()).sum();
        if a > p {
            cx.per_round(g, "map.agents_over", 1);
        }
    }
    // 068: what each seat left unspent.
    for s in 0..g.seats.len() {
        for r in LEFT {
            cx.per_round(g, &format!("unspent.{s}.{r}"), g.resource(s, r));
        }
        // 148: research cubes too (the leftover read; LEFT is the margin's order).
        cx.per_round(g, &format!("unspent.{s}.research_cubes"), g.resource(s, Resource::ResearchCubes));
        ai::tally_dear_powers(g, cx, s);
        // 078: the seat's most-held tag on its cards in play (tags by round).
        let top = crate::modules::cards::tag_counts(&w, g, s).into_iter().max().unwrap_or(0);
        cx.per_round(g, &format!("ai.tag_top.{s}"), top);
        // 074: converters a seat holding cubes left unused, and why.
        if g.resource(s, Resource::ResearchCubes) > 0 {
            for c in 0..w.map.countries.len() {
                match converter_blocked(&w, g, s, c) {
                    None => cx.tally(g, "board.converter_idle", 1),
                    Some(NO_CITY_NODE) => cx.tally(g, "board.converter_no_city", 1),
                    Some(CANT_PAY) => cx.tally(g, "board.converter_unpaid", 1),
                    Some(_) => {}
                }
            }
        }
        // 199: chip converters a seat holding chips left unused (stats only).
        if g.resource(s, Resource::Chips) > 0 {
            for c in 0..w.map.countries.len() {
                if converter_blocked_of(&w, g, s, c, true).is_none() {
                    cx.tally(g, "board.chip_converter_idle", 1);
                }
            }
        }
    }
}

/// 137: factories make their owner's output: a player's chips, the adversary's
/// 1 troop on its hex, a neutral one nothing. One opposing ambassador blocks it while it stands: an adversary
/// ambassador on a player's factory, a player ambassador on the adversary's.
fn factory_output(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let chips = w.int("board.factory_chips");
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 {
            continue;
        }
        // 135 (Factory Occupation): a factory a troop occupies makes nothing (an adversary's: its Income beat).
        if occupied(g, h) {
            if !corrupted(g, h) {
                cx.tally(g, "board.factories_occupied", 1);
            }
            continue;
        }
        if let Some(s) = cube_owner(g, h) {
            if g.tokens(h, ADV_AMBASSADOR) > 0 {
                cx.tally(g, "board.blocked.factory_player", 1);
            } else {
                cx.add_resource(g, s, Resource::Chips, chips);
                cx.tally(g, "board.chips_made", chips);
            }
        }
        // 293 (Matt, 2026-11-19): an adversary factory's one output is an agent, made in the adversary's Income
        // beat (`adversary::factories_output`), no longer a troop here.
    }
}

/// The income step, the board's part (r75, Matt's ruling 6: one step in a
/// printed order; economy's upkeep, the board's and cards' income, follows it at
/// once): players' factories make chips, cities energy, players' and neutral stations
/// research cubes, then full megaprojects complete (293: the adversary's sites make their
/// outputs in its Income beat, `adversary::compute_step`). One opposing piece blocks a
/// site; an agent on a city stays and the city is dark (r37 neutral, r38
/// aligned too).
pub(super) fn produce(g: &mut GameState, cx: &mut Cx) -> Res {
    measure_round(g, cx);
    factory_output(g, cx);
    city_energy(g, cx)?;
    station_cubes(g, cx)?;
    complete_projects(g, cx)
}

/// The energy city `h` makes when it is producing (its country's `corp_energy`).
pub fn city_output(w: &World, h: usize) -> i64 {
    let c = w.map.hexes[h].country as usize;
    let l = w.list("board.corp_energy");
    l[c % l.len()]
}

/// Cities make energy: diverted by the adversary's, free to an aligned city's
/// owner; a neutral city's to every player with a node on or next to it (292).
pub(super) fn city_energy(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        if !is_city(&w, h) {
            continue;
        }
        let energy = city_output(&w, h);
        // A blackout on its country or (135) near a threat.
        if hex_flagged(&w, g, RoundFlag::Blackout, h) {
            continue;
        }
        if corrupted(g, h) {
            // 293: a corrupted city's one output is its Discord, made in the adversary's Income beat
            // ([`corrupted_cities_discord`]).
        } else if g.tokens(h, ADV_AMBASSADOR) > 0 && cube_owner(g, h).is_none() {
            // r37 (Matt): on a neutral city the ambassador stays and the city is dark:
            // no sale, and the unbought energy is lost as usual.
            cx.tally(g, "board.city_dark", 1);
        } else if g.tokens(h, ADV_AMBASSADOR) > 0 {
            // r38 (Matt): on an aligned city too the ambassador stays and the city is
            // dark: its owner gets no energy until a player removes it.
            let n = energy * w.int("board.ambassador_discord");
            alignment::sway(g, cx, Side::Discord, n);
            cx.tally(g, "board.city_dark_aligned", 1);
            if n > 0 && !w.hot.alignment_off {
                cx.note(Some(h), || format!("{} is dark (an agent on it): {n} Discord", hex_name(&w, h)));
            }
        } else if let Some(s) = cube_owner(g, h) {
            // An aligned city's cube goes free to the player who aligned it.
            let n = energy + w.int("board.aligned_bonus");
            cx.add_resource(g, s, Resource::Energy, n);
            cx.tally(g, "board.city_energy_aligned", n);
        } else {
            // 292: a city neither corrupted nor blocked pays every player with a node on or next to it.
            let mut paid = false;
            for s in 0..g.seats.len() {
                if node_by(&w, g, s, h) {
                    cx.add_resource(g, s, Resource::Energy, energy);
                    cx.tally(g, "board.city_energy", energy);
                    cx.tally(g, &format!("board.city_energy.s{s}"), energy);
                    paid = true;
                }
            }
            if paid {
                cx.per_round(g, "map.cities_paying", 1);
            }
        }
    }
    Ok(())
}

/// 293 (Matt, 2026-11-19: one output a corrupted site), a corrupted city's: `divert_discord` (1) Discord, in the
/// adversary's Income beat with the other outputs (it was the city's energy as Discord, in production); a player
/// ambassador on it blocks it, and so does a blackout on it.
pub fn corrupted_cities_discord(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let n = w.int("board.divert_discord");
    for h in 0..g.hexes.len() {
        if !is_city(&w, h) || !corrupted(g, h) || hex_flagged(&w, g, RoundFlag::Blackout, h) {
            continue;
        }
        if player_ambassador(g, h).is_some() {
            // 137: a player ambassador on an adversary city blocks its output.
            cx.tally(g, "board.blocked.city_adversary", 1);
            continue;
        }
        alignment::discord_from(g, cx, "city", n);
        cx.tally(g, "board.energy_diverted", n);
        if n > 0 && !w.hot.alignment_off {
            cx.note(Some(h), || format!("{} (corrupted) makes {n} Discord", hex_name(&w, h)));
        }
    }
}

/// Whose cube a station makes this round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Seat(usize),
    Neutral,
}

/// 074: stations make research cubes in their owner's color: onto their
/// country's megaproject until it completes, then onto their owner's board (a
/// neutral one's is lost). 137: an adversary station makes compute instead,
/// and one opposing ambassador blocks a station while it stands.
/// 286 (D124): what the next production pays seat `s`, (chips, research cubes), as its income tracks print it
/// (r133): its factories' chips (none from an occupied factory or one an agent blocks) and, once their country's
/// megaproject is complete, its stations' cubes (none from a blocked one). [`factory_output`] and [`station_cubes`]
/// pay it; the seat strip adds it to upkeep's income. Cities' energy: [`city_income`].
pub fn production_income(w: &World, g: &GameState, s: usize) -> (i64, i64) {
    let (mut chips, mut cubes) = (0, 0);
    for h in 0..g.hexes.len() {
        if cube_owner(g, h) != Some(s) || g.tokens(h, ADV_AMBASSADOR) > 0 {
            continue;
        }
        if g.tokens(h, FACTORY) > 0 && !occupied(g, h) {
            chips += w.int("board.factory_chips");
        }
        if g.tokens(h, STATION) > 0 && !corrupted(g, h) && g.var(&project_done(w.map.hexes[h].country as usize)) != 0 {
            cubes += w.int("board.owned_station_cubes");
        }
    }
    (chips, cubes)
}

/// 292: the energy the next production pays seat `s` from cities, as its energy income track shows it: each city
/// neither corrupted nor blocked with its node on or next to it ([`city_energy`] pays it). The marker moves only
/// when a node is placed or lost or a city changes state.
pub fn city_income(w: &World, g: &GameState, s: usize) -> i64 {
    (0..g.hexes.len())
        .filter(|&h| is_city(w, h))
        .map(|h| match cube_owner(g, h) {
            // r114's ownership (`cities_unowned` 0): an aligned city pays its owner alone, with its bonus, unless
            // an agent or a blackout darkens it, as [`city_energy`] pays it.
            Some(o) if o == s && !corrupted(g, h) && g.tokens(h, ADV_AMBASSADOR) == 0 && !hex_flagged(w, g, RoundFlag::Blackout, h) => {
                city_output(w, h) + w.int("board.aligned_bonus")
            }
            Some(_) => 0,
            None if city_pays(w, g, h) && node_by(w, g, s, h) => city_output(w, h),
            None => 0,
        })
        .sum()
}

pub(super) fn station_cubes(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let cubes = w.int("board.station_cubes");
    for h in 0..g.hexes.len() {
        if g.tokens(h, STATION) == 0 {
            continue;
        }
        let c = w.map.hexes[h].country as usize;
        // 137: one opposing ambassador blocks the output while it stands: an
        // agent on a neutral or player station, a player
        // ambassador on the adversary's. An adversary station makes compute.
        // 175: an adversary station's compute is added in the adversary's compute step. 185 (Matt's ruling): until
        // its country's megaproject completes it puts `station_cubes` adversary cubes onto it instead; one player
        // ambassador on it blocks that as it blocks the compute.
        // 293 (Matt, 2026-11-19): an adversary station's cube goes only onto the adversary's research track, in
        // its Income beat (`adversary::compute_step`); megaprojects take neutral and player cubes only.
        if corrupted(g, h) {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 {
            let side = if cube_owner(g, h).is_some() { "player" } else { "neutral" };
            cx.tally(g, &format!("board.blocked.station_{side}"), 1);
            continue;
        }
        let color = cube_owner(g, h).map_or(Color::Neutral, Color::Seat);
        // 142: a player's station makes `owned_station_cubes`, a neutral one `station_cubes`.
        let made = if matches!(color, Color::Seat(_)) { w.int("board.owned_station_cubes") } else { cubes };
        if g.var(&project_done(c)) == 0 {
            let key = match color {
                Color::Seat(s) => project_key(c, s),
                Color::Neutral => project_neutral(c),
            };
            cx.add_key(g, &key, made);
            continue;
        }
        if let Color::Seat(s) = color {
            cx.add_resource(g, s, Resource::ResearchCubes, made);
            cx.tally(g, "board.cubes_stored", made);
        }
    }
    Ok(())
}


/// 074: checked once, at the end of production: a megaproject holding its
/// cubes or more completes. Every cube on it pays, overflow included, and is
/// spent: a player cube pays its owner, an adversary cube the adversary, a
/// neutral cube nothing.
fn complete_projects(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    // The map's countries (a game may seat fewer than content prints), each with its content row.
    for (c, country) in w.content.map.countries.iter().enumerate().take(w.map.countries.len()) {
        if g.var(&project_done(c)) > 0 {
            continue;
        }
        let p = &country.megaproject;
        let neutral = g.var(&project_neutral(c));
        let mine: Vec<i64> = (0..g.seats.len()).map(|s| g.var(&project_key(c, s))).collect();
        let paid: i64 = mine.iter().sum();
        if neutral + paid < p.cubes {
            continue;
        }
        for s in 0..mine.len() {
            cx.set_key(g, &project_key(c, s), 0);
        }
        cx.set_key(g, &project_neutral(c), 0);
        cx.set_key(g, &project_done(c), 1);
        cx.tally(g, "board.projects_completed", 1);
        cx.tally(g, &format!("board.completed_round.{}", g.round), 1);
        cx.tally(g, &format!("board.completed.{c}"), 1);
        for (color, n) in [("player", paid), ("neutral", neutral)] {
            cx.tally(g, &format!("board.cubes_paid.{color}"), n);
            cx.set_key(g, &project_paid(c, color), n);
        }
        cx.note(None, || {
            format!("{} completes: {} cubes pay ({paid} player, {neutral} neutral)", p.name, paid + neutral)
        });
        for (s, &n) in mine.iter().enumerate() {
            for _ in 0..n {
                effects::run(g, cx, &p.player, &mut effects::Ctx::seat(s))?;
            }
        }
        if g.is_over() {
            return Ok(());
        }
    }
    Ok(())
}
