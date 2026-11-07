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
        if let Some(s) = player_ambassador(g, base) {
            cx.add_tokens(g, base, ambassador(s), -1);
            cx.tally(g, "board.base_ambassador_used", 1);
            continue;
        }
        if past_cap || g.tokens(base, UNIT) < w.int("board.unit_cap") {
            cx.add_tokens(g, base, UNIT, 1);
            made += 1;
        }
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

/// 137: a Discord tip. Each neutral factory makes 1 corp troop, which its corp
/// puts on each of its bases; an adversary ambassador on the factory blocks it
/// (one opposing ambassador blocks a site's output), a player ambassador on it
/// is used up instead (the troop rule), and an occupied factory makes nothing.
pub fn tip_corp_troops(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) == 0 || corrupted(g, h) || cube_owner(g, h).is_some() || occupied(g, h) {
            continue;
        }
        if g.tokens(h, ADV_AMBASSADOR) > 0 {
            cx.tally(g, "board.blocked.factory_neutral", 1);
            continue;
        }
        if let Some(s) = player_ambassador(g, h) {
            cx.add_tokens(g, h, ambassador(s), -1);
            cx.tally(g, "board.factory_ambassador_used", 1);
            continue;
        }
        cx.tally(g, "board.tip_factory_troops", 1);
        for b in corp_bases(&w, w.map.hexes[h].country) {
            add_unit(g, cx, b);
        }
    }
}

/// Production (every round; 139 cut the clock): bases make nothing of
/// their own; corp troops come from Discord tips at neutral factories
/// ([`tip_corp_troops`]) and the extra-troop rules. Player-owned factories
/// make chips, adversary factories an adversary troop; one opposing
/// ambassador blocks either while it stands. Then the cities grow factories.
pub(super) fn produce_units(g: &mut GameState, cx: &mut Cx) -> Res {
    measure_round(g, cx);
    extra_units(g, cx);
    factory_output(g, cx);
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
    let ongoing = g.pile(adversary::PROJECTS_ONGOING).iter().flat_map(|id| w.content.project(id).effects.iter());
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

/// Each city builds one factory: a corrupted city at a free zone in its
/// country next to an adversary ambassador if there is one, else the
/// lowest-numbered free zone (lowest number among equals); any other city on
/// a free zone next to it, where the players choose (an aligned city's
/// owner, else seat 0). A corrupted city's factory is the adversary's, an
/// aligned city's its owner's, and a clean city's goes to the player with
/// the most nodes next to the zone (players choose on a tie), neutral if
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
                let prompt = || format!("{} builds a factory on which zone?", hex_name(&w, city));
                cx.choose(who, prompt, opts)?.map(|i| near[i])
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
/// most nodes next to it (players choose on a tie), none if nobody has any.
fn zone_owner(g: &mut GameState, cx: &mut Cx, z: usize) -> Res<Option<usize>> {
    let w = cx.world;
    let hexes = within(&w, z, 1);
    let near: Vec<usize> = (0..g.seats.len())
        .map(|s| hexes.iter().filter(|&&x| g.tokens(x, node(s)) > 0).count())
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
    let prompt = || format!("The factory at {} goes to which player?", hex_name(&w, z));
    Ok(cx.choose(0, prompt, opts)?.map(|i| tied[i]))
}

/// Production step 3: cities make energy, stations research cubes; an
/// adversary ambassador on a station takes its output and is removed; on a
/// city it stays and the city is dark (r37 neutral, r38 aligned too). Then an adversary ambassador on a
/// lone node drains 1 of its owner's next insight into adversary compute,
/// and last full megaprojects complete.
pub(super) fn produce(g: &mut GameState, cx: &mut Cx) -> Res {
    city_energy(g, cx)?;
    station_cubes(g, cx)?;
    node_leak(g, cx)?;
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
        // A blackout on its country or (135) near a threat, or the grid down near one.
        if hex_flagged(&w, g, RoundFlag::Blackout, h) || hex_flagged(&w, g, RoundFlag::GridDown, h) {
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
        } else {
            // A neutral city's cube: a player with a node next to it may buy
            // it (players choose who), otherwise it is lost (021).
            let buyers: Vec<usize> = (0..g.seats.len())
                .filter(|&s| g.resource(s, Resource::Credits) >= price && in_reach(&w, g, s, h))
                .collect();
            // 118: a city making no energy (`corp_energy` 0) sold nothing for its price.
            if buyers.is_empty() || energy <= 0 {
                continue;
            }
            let mut opts: Vec<_> = buyers.iter().map(|&s| opt(format!("seat {s} buys"))).collect();
            opts.push(opt("nobody".to_string()));
            let prompt = || format!(
                "The energy of {} ({energy} energy) can be bought for {price} credits: by whom?",
                hex_name(&w, h)
            );
            if let Some(i) = cx.choose(0, prompt, opts)? {
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
        // 135: a dark station (Thermal Runaway, Rolling Blackouts) or the grid
        // down near it makes nothing; an adversary ambassador on it stays.
        if hex_flagged(&w, g, RoundFlag::StationDark, h) || hex_flagged(&w, g, RoundFlag::GridDown, h) {
            cx.tally(g, "board.stations_dark", 1);
            continue;
        }
        // 137: one opposing ambassador blocks the output while it stands: an
        // adversary ambassador on a neutral or player station, a player
        // ambassador on the adversary's. An adversary station makes compute.
        if corrupted(g, h) {
            if player_ambassador(g, h).is_some() {
                cx.tally(g, "board.blocked.station_adversary", 1);
            } else {
                cx.tally(g, "board.station_compute", cubes);
                adversary::gain_compute(g, cx, cubes)?;
                if g.is_over() {
                    return Ok(());
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
        // 135 (Cooling Failure): a seat with a station outage makes no cubes.
        if let Color::Seat(s) = color {
            if seat_flagged(g, RoundFlag::StationOutage, s) {
                cx.tally(g, "board.station_outage", 1);
                continue;
            }
        }
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

/// An adversary ambassador on a node (with no other piece to divert)
/// costs its owner 1 insight next round and adds 1 compute to the cheapest
/// adversary project; it stays until removed (068; 021: it was used up).
fn node_leak(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for h in 0..g.hexes.len() {
        let Some(s) = node_owner(g, h) else {
            continue;
        };
        // (020: a Sabotage flag used to skip the leak here, a data-center-era
        // leftover the rules don't have; once F1 let the flag reach production
        // it would have helped the players.)
        if g.tokens(h, ADV_AMBASSADOR) == 0 {
            continue;
        }
        // Only a lone node: on a city, base, station or factory the
        // ambassador belongs to that piece (020: the leak drained a corrupted
        // city's ambassadors so it never spread, and ate those a clean city or
        // a base was gathering).
        if is_city(&w, h) || is_base(&w, h) || has_building(g, h) {
            continue;
        }
        cx.add_key(g, &economy::insight_debt(s), 1);
        cx.tally(g, "board.node_drained", 1);
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
