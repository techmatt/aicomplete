//! Production: units, factories, chips, city energy, research cubes and megaprojects.

use super::*;
use crate::op::{CorpAt, CorpNear, CorpWhen};

/// The resources a seat is measured on at the end of each action phase (068).
pub const LEFT: [Resource; 5] =
    [Resource::Credits, Resource::Chips, Resource::Energy, Resource::Antimatter, Resource::Ambassadors];

/// 137: `n` corp troops are made at base `base` (held to `unit_cap` unless
/// `past_cap`). A player ambassador standing on the base is used up instead of
/// each troop, while one stands (the troop rule: no decision at tip time).
/// Returns the troops made.
pub fn corp_troops_at(g: &mut GameState, cx: &mut Cx, base: usize, n: i64, past_cap: bool) -> i64 {
    let w = cx.world;
    let mut made = 0;
    for _ in 0..n {
        // 145: a full base makes no troop, so no ambassador is used up in its place.
        if !past_cap && g.tokens(base, UNIT) >= w.int("board.unit_cap") {
            break;
        }
        if let Some(s) = player_ambassador(g, base) {
            cx.add_tokens(g, base, ambassador(s), -1);
            cx.tally(g, "board.base_ambassador_used", 1);
            continue;
        }
        cx.add_tokens(g, base, UNIT, 1);
        made += 1;
    }
    made
}

fn add_unit(g: &mut GameState, cx: &mut Cx, base: usize) {
    let made = corp_troops_at(g, cx, base, 1, false);
    cx.tally(g, "board.units_built", made);
}

/// The bases of country `c`'s corp (137: one on each of its active fronts;
/// every printed front is active).
pub fn corp_bases(w: &World, c: u8) -> Vec<usize> {
    (0..w.map.hexes.len()).filter(|&b| is_base(w, b) && w.map.hexes[b].country == c).collect()
}

/// A neutral factory that isn't blocked: no cube, not corrupted or occupied, and no agent or player ambassador
/// on it (one opposing piece blocks a site's output).
fn open_neutral_factory(g: &GameState, h: usize) -> bool {
    g.tokens(h, FACTORY) > 0
        && !corrupted(g, h)
        && cube_owner(g, h).is_none()
        && !occupied(g, h)
        && g.tokens(h, ADV_AMBASSADOR) == 0
        && player_ambassador(g, h).is_none()
}

/// A Discord tip, r80 (ruling #50): each base of a country with an unblocked neutral factory gains 1 drone; a
/// player ambassador on the base stops it (used up, the base rule of [`corp_troops_at`]), and a full base takes
/// none. (137-r79: 1 drone per neutral factory per base, an ambassador on the factory stopping one, players
/// choosing which base.)
pub fn tip_corp_troops(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for c in 0..w.map.countries.len() {
        let hexes = &w.map.countries[c].hexes;
        let blocked = hexes
            .iter()
            .filter(|&&h| g.tokens(h, FACTORY) > 0 && !corrupted(g, h) && cube_owner(g, h).is_none() && !occupied(g, h))
            .filter(|&&h| !open_neutral_factory(g, h))
            .count() as i64;
        cx.tally(g, "board.blocked.factory_neutral", blocked);
        if !hexes.iter().any(|&h| open_neutral_factory(g, h)) {
            continue;
        }
        cx.tally(g, "board.tip_factory_troops", 1);
        for b in corp_bases(&w, c as u8) {
            add_unit(g, cx, b);
        }
    }
    Ok(())
}

/// Growth, after the income step (r75: it was production's first part): bases
/// make their extra corp troops (corp troops otherwise come from Discord tips at
/// neutral factories, [`tip_corp_troops`]), then the cities grow (190: [`grow_cities`]).
pub(super) fn produce_units(g: &mut GameState, cx: &mut Cx) -> Res {
    extra_units(g, cx);
    grow_cities(g, cx)
}

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
        if is_city(&w, h) {
            let n = g.tokens(h, CITY_CHIP);
            cx.per_round(g, "map.city_chips", n);
        }
        let t = g.tokens(h, TROOP);
        cx.per_round(g, "map.troops", t);
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

/// The extra corp troops a base asks for this production step (135): (past
/// its cap, held to it), from the threats registered on it (Conscription),
/// the ongoing projects (none since r85; Arms Race was one) and the adversary board's rules
/// (Warmongering).
fn extra_asked(w: &World, g: &GameState, base: usize, registered: &[(usize, u32, i64)]) -> (i64, i64) {
    let (mut past, mut capped) = (0, 0);
    for &(b, until, n) in registered {
        if b == base && until >= g.round {
            past += n;
        }
    }
    let acting = adversary::ongoing_acting(g);
    let ongoing = acting.iter().flat_map(|id| w.content.project(id).effects.iter());
    let rules = w.content.adversary_board.rules.iter().flat_map(|r| r.effects.iter());
    for op in ongoing.chain(rules) {
        if let Op::CorpTroops { count, at: CorpAt::EveryBase, near, when: CorpWhen::EachProduction, past_cap, .. } = op {
            if matches!(near, Some(CorpNear::AdversaryTroop)) && !next_to(w, base, |x| g.tokens(x, TROOP) > 0) {
                continue;
            }
            if *past_cap {
                past += count;
            } else {
                capped += count;
            }
        }
    }
    (past, capped)
}

/// 135: bases make their extra corp troops. Matt's ruling 4: at most 1 extra a
/// base each production step, whatever makes it; past the cap when any source
/// goes past it.
fn extra_units(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let registered = corp_extras(g);
    for h in 0..g.hexes.len() {
        if !is_base(&w, h) {
            continue;
        }
        let (past, capped) = extra_asked(&w, g, h, &registered);
        if past > 0 {
            let made = corp_troops_at(g, cx, h, 1, true);
            cx.tally(g, "board.units_extra", made);
            if past > 1 {
                cx.tally(g, "board.units_extra_capped", past - 1);
            }
        } else if capped > 0 {
            add_unit(g, cx, h);
        }
    }
}

/// 137: factories make their owner's output: a player's chips, the adversary's
/// 1 troop on its hex, a neutral one nothing (its troop comes at a Discord
/// tip). One opposing ambassador blocks it while it stands: an adversary
/// ambassador on a player's factory, a player ambassador on the adversary's.
fn factory_output(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let chips = w.int("board.factory_chips");
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 {
            continue;
        }
        // 135 (Factory Occupation): a factory a troop occupies makes nothing.
        if occupied(g, h) {
            cx.tally(g, "board.factories_occupied", 1);
            continue;
        }
        if let Some(s) = cube_owner(g, h) {
            if g.tokens(h, ADV_AMBASSADOR) > 0 {
                cx.tally(g, "board.blocked.factory_player", 1);
            } else {
                cx.add_resource(g, s, Resource::Chips, chips);
                cx.tally(g, "board.chips_made", chips);
            }
        } else if corrupted(g, h) {
            if player_ambassador(g, h).is_some() {
                // 137: a player ambassador blocks it while it stands.
                cx.tally(g, "board.blocked.factory_adversary", 1);
                continue;
            }
            adversary::place_troop(g, cx, h);
            cx.tally(g, "board.troops_built", 1);
            cx.note(Some(h), || format!("The factory at {} makes an adversary troop", hex_name(&w, h)));
        }
    }
}

/// Whether hex `h` holds an unclaimed neutral factory (no owner cube, player or adversary): what feeds the
/// cities next to it (190).
pub fn neutral_factory(g: &GameState, h: usize) -> bool {
    g.tokens(h, FACTORY) > 0 && cube_owner(g, h).is_none() && !corrupted(g, h)
}

/// The chips city `city` gathers this round (190, Matt 2026-11-11, amendment 8642): 1 for each unclaimed neutral
/// factory next to it, none of its own. An agent on the city neither blocks nor takes them.
pub fn city_chip_gain(w: &World, g: &GameState, city: usize) -> i64 {
    w.map.hexes[city].neighbors.iter().filter(|&&h| neutral_factory(g, h)).count() as i64
}

/// The facility city `city` builds next (190): whichever of factories and research stations it has fewer of next to
/// it, any owner (ties: a factory). True for a factory.
pub fn city_builds_factory(w: &World, g: &GameState, city: usize) -> bool {
    let near = &w.map.hexes[city].neighbors;
    let f = near.iter().filter(|&&h| g.tokens(h, FACTORY) > 0).count();
    let s = near.iter().filter(|&&h| g.tokens(h, STATION) > 0).count();
    f <= s
}

/// Where city `city` may build its next facility (190): free hexes of its type next to it (a factory on a free
/// industrial zone, a station on open land that is not one, no base); with none, those within 2 in its own
/// country; else none, and its chips wait.
pub fn city_build_sites(w: &World, g: &GameState, city: usize, factory: bool) -> Vec<usize> {
    let fits = |h: usize| {
        if factory {
            w.map.hexes[h].industrial && !has_building(g, h) && g.tokens(h, LAND_CORRUPTION) == 0
        } else {
            buildable(w, g, h) && !w.map.hexes[h].industrial && !is_base(w, h)
        }
    };
    let near: Vec<usize> = (0..g.hexes.len()).filter(|&h| w.map.adjacent(city, h) && fits(h)).collect();
    if !near.is_empty() {
        return near;
    }
    let country = w.map.hexes[city].country;
    (0..g.hexes.len())
        .filter(|&h| w.map.hexes[h].country == country && w.map.distance(city, h) == 2 && fits(h))
        .collect()
}

/// City growth (190, Matt 2026-11-11, amendment 8642; it replaces r75's factory a city a round): every city, owned,
/// neutral or corrupted, gathers chips onto itself from the unclaimed neutral factories next to it, at most
/// `board.city_chip_cap`. At `board.city_build_chips` it spends them on a **neutral** facility, the type it has
/// fewer of next to it, on a site [`city_build_sites`] finds: an aligned city's owner picks it, a corrupted city's
/// is the one next to an agent if any, else the lowest-numbered, as a neutral city's is. With no site, the chips
/// wait. Cities gather first, then build in hex order.
fn grow_cities(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let cap = w.int("board.city_chip_cap");
    let at = w.int("board.city_build_chips");
    let cities: Vec<usize> = (0..g.hexes.len()).filter(|&h| is_city(&w, h)).collect();
    for &city in &cities {
        let have = g.tokens(city, CITY_CHIP);
        let add = city_chip_gain(&w, g, city).min(cap - have).max(0);
        if add > 0 {
            cx.add_tokens(g, city, CITY_CHIP, add);
            cx.tally(g, "board.city_chips", add);
            cx.per_round(g, "map.upkeep", add);
        }
    }
    for &city in &cities {
        if g.tokens(city, CITY_CHIP) < at {
            continue;
        }
        let factory = city_builds_factory(&w, g, city);
        let sites = city_build_sites(&w, g, city, factory);
        let pick = if sites.is_empty() {
            None
        } else if let (Some(s), false) = (cube_owner(g, city), corrupted(g, city)) {
            let opts = sites.iter().map(|&z| hex_opt(&w, z)).collect();
            let kind = if factory { "factory" } else { "research station" };
            let prompt = || format!("{} builds a neutral {kind} on which hex?", hex_name(&w, city));
            cx.choose(s, prompt, opts)?.map(|i| sites[i])
        } else {
            sites.iter().copied().min_by_key(|&z| (!next_to(&w, z, |x| g.tokens(x, ADV_AMBASSADOR) > 0), w.map.hexes[z].number))
        };
        let Some(z) = pick else {
            cx.tally(g, "board.city_waits", 1);
            continue;
        };
        cx.add_tokens(g, city, CITY_CHIP, -at);
        let (token, name) = if factory { (FACTORY, "factory") } else { (STATION, "station") };
        cx.add_tokens(g, z, token, 1);
        cx.tally(g, &format!("board.{name}_built_by_city"), 1);
        cx.per_round(g, &format!("map.built.{name}"), 1);
        cx.per_round(g, "map.upkeep", at + 1);
        cx.note(Some(z), || format!("{} builds a neutral {name} at {}", hex_name(&w, city), hex_name(&w, z)));
    }
    Ok(())
}

/// The income step, the board's part (r75, Matt's ruling 6: one step in a
/// printed order; economy's upkeep, the board's and cards' income, follows it at
/// once): factories make chips or an adversary troop, cities energy, stations
/// research cubes, then full megaprojects complete. One opposing piece blocks a
/// site; an agent on a city stays and the city is dark (r37 neutral, r38
/// aligned too). A neutral city's energy is no longer sold here: the Buy energy
/// standard action buys it in the action phase.
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

/// Whether city `h` is blocked from producing: a blackout on it (its country, or
/// near a threat, 135) or an agent standing on it.
pub fn city_dark(w: &World, g: &GameState, h: usize) -> bool {
    hex_flagged(w, g, RoundFlag::Blackout, h) || g.tokens(h, ADV_AMBASSADOR) > 0
}

/// Cities make energy: diverted by the adversary's, free to an aligned city's
/// owner; a neutral city's makes nothing here (r75: it is bought by the Buy
/// energy action).
pub(super) fn city_energy(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let divert = w.int("board.divert_discord");
    for h in 0..g.hexes.len() {
        if !is_city(&w, h) {
            continue;
        }
        let energy = city_output(&w, h);
        // A blackout on its country or (135) near a threat.
        if hex_flagged(&w, g, RoundFlag::Blackout, h) {
            continue;
        }
        if corrupted(g, h) && player_ambassador(g, h).is_some() {
            // 137: a player ambassador on an adversary city blocks its diversion.
            cx.tally(g, "board.blocked.city_adversary", 1);
        } else if corrupted(g, h) {
            alignment::discord_from(g, cx, "city", energy * divert);
            cx.tally(g, "board.energy_diverted", energy);
        } else if g.tokens(h, ADV_AMBASSADOR) > 0 && cube_owner(g, h).is_none() {
            // r37 (Matt): on a neutral city the ambassador stays and the city is dark:
            // no sale, and the unbought energy is lost as usual.
            cx.tally(g, "board.city_dark", 1);
        } else if g.tokens(h, ADV_AMBASSADOR) > 0 {
            // r38 (Matt): on an aligned city too the ambassador stays and the city is
            // dark: its owner gets no energy until a player removes it.
            alignment::sway(g, cx, Side::Discord, energy * w.int("board.ambassador_discord"));
            cx.tally(g, "board.city_dark_aligned", 1);
        } else if let Some(s) = cube_owner(g, h) {
            // An aligned city's cube goes free to the player who aligned it.
            let n = energy + w.int("board.aligned_bonus");
            cx.add_resource(g, s, Resource::Energy, n);
            cx.tally(g, "board.city_energy_aligned", n);
        }
    }
    Ok(())
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
        if corrupted(g, h) {
            if g.var(&project_done(c)) == 0 {
                if player_ambassador(g, h).is_some() {
                    cx.tally(g, "board.blocked.station_adversary", 1);
                } else {
                    cx.add_key(g, &project_key(c, None), cubes);
                    cx.tally(g, "board.adversary_cubes", cubes);
                }
            }
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
                Color::Seat(s) => project_key(c, Some(s)),
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
