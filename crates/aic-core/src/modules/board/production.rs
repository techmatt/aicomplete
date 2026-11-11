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

/// 137: a Discord tip. Each neutral factory makes 1 corp troop for each of its
/// corp's bases; an agent on the factory blocks it (one opposing ambassador
/// blocks a site's output), and an occupied factory makes nothing. r75 (Matt's
/// ruling 8): a player ambassador on it always stops 1 troop, used up (it
/// stopped them all); the players choose which base's troop when it can't stop
/// every one. A full base takes none (145), so it uses no ambassador.
pub fn tip_corp_troops(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let cap = w.int("board.unit_cap");
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 || corrupted(g, h) || cube_owner(g, h).is_some() || occupied(g, h) {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 {
            cx.tally(g, "board.blocked.factory_neutral", 1);
            continue;
        }
        let mut open: Vec<usize> =
            corp_bases(&w, w.map.hexes[h].country).into_iter().filter(|&b| g.tokens(b, UNIT) < cap).collect();
        while !open.is_empty() {
            let Some(s) = player_ambassador(g, h) else { break };
            let standing: i64 = (0..g.seats.len()).map(|x| g.tokens(h, ambassador(x))).sum();
            let i = if open.len() as i64 > standing {
                let opts = open.iter().map(|&b| hex_opt(&w, b)).collect();
                let prompt = || format!("The ambassador on the factory at {} stops the troop for which base?", hex_name(&w, h));
                cx.choose(0, prompt, opts)?.unwrap_or(0)
            } else {
                0
            };
            open.remove(i);
            cx.add_tokens(g, h, ambassador(s), -1);
            cx.tally(g, "board.factory_ambassador_used", 1);
        }
        if open.is_empty() {
            continue;
        }
        cx.tally(g, "board.tip_factory_troops", 1);
        for b in open {
            add_unit(g, cx, b);
        }
    }
    Ok(())
}

/// Growth, after the income step (r75: it was production's first part): bases
/// make their extra corp troops (corp troops otherwise come from Discord tips at
/// neutral factories, [`tip_corp_troops`]), then the cities grow factories.
pub(super) fn produce_units(g: &mut GameState, cx: &mut Cx) -> Res {
    extra_units(g, cx);
    grow_factories(g, cx)
}

/// The measure's tallies taken as production starts, not a rule: what each
/// seat left unspent, its dear powers, its top tag, its idle converters.
fn measure_round(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
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
    }
}

/// The extra corp troops a base asks for this production step (135): (past
/// its cap, held to it), from the threats registered on it (Conscription),
/// the ongoing projects (Arms Race) and the adversary board's rules
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

/// Each city builds one factory on a free zone next to it, or none (r75, Matt's
/// ruling 4: a corrupted city searched its whole country). A corrupted city's
/// factory is the adversary's, on the zone next to an agent if there is one,
/// else the lowest-numbered; an aligned city's is its owner's, a neutral city's
/// neutral (claimable), where the players choose (the owner, else seat 0).
fn grow_factories(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for city in 0..g.hexes.len() {
        if !is_city(&w, city) {
            continue;
        }
        let near: Vec<usize> = free_zones(&w, g).into_iter().filter(|&z| w.map.adjacent(city, z)).collect();
        let at = if near.is_empty() {
            None
        } else if corrupted(g, city) {
            near.iter().copied().min_by_key(|&z| (!next_to(&w, z, |x| g.tokens(x, ADV_AMBASSADOR) > 0), w.map.hexes[z].number))
        } else {
            let who = cube_owner(g, city).unwrap_or(0);
            let opts = near.iter().map(|&z| hex_opt(&w, z)).collect();
            let prompt = || format!("{} builds a factory on which zone?", hex_name(&w, city));
            cx.choose(who, prompt, opts)?.map(|i| near[i])
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
        } else if let Some(s) = cube_owner(g, city) {
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
