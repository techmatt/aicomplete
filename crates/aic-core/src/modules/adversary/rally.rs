//! 275: the adversary's card-driven lines: Infiltrate X (it replaced Rally, the agent pools and agent income),
//! Surge, Spill and Grow (it replaced city growth), botnets' seeding, out-of-turn Infiltrates and creation lines.
//! Every step acts only on the sites its line names; each arrival is checked where it lands ([`arrive_check`]).

use super::*;
use crate::op::SiteType;

/// 151: the income-list entry for this difficulty (the last entry past the list's end).
pub(super) fn income_at(w: &World, param: &str) -> i64 {
    let l = w.list(param);
    let d = w.int("adversary.difficulty").max(0) as usize;
    l.get(d).or(l.last()).copied().unwrap_or(0)
}

/// 151: "increase income": compute for the rest of the game (231, Matt: "+N compute a round" is N a country).
pub fn increase_income(g: &mut GameState, cx: &mut Cx, compute: i64) {
    let compute = compute * cx.world.map.countries.len() as i64;
    cx.add_key(g, INCOME_COMPUTE, compute);
    cx.tally(g, "adversary.income_raised_compute", compute);
}

/// 275: the site types a card's Infiltrate lines name, in card order (stage III cards print two).
pub fn infiltrate_sites(c: &AdversaryCard) -> Vec<SiteType> {
    c.standing.iter().filter_map(|e| if let Op::Infiltrate { site } = e { Some(*site) } else { None }).collect()
}

/// 275: the site type a card's Grow line names.
pub fn grow_site(c: &AdversaryCard) -> Option<SiteType> {
    c.standing.iter().chain(&c.twists).find_map(|e| if let Op::Grow { site } = e { Some(*site) } else { None })
}

/// 275: agents an Infiltrate lands on each city at this level (`infiltrate_agents`, by difficulty); a station,
/// factory or botnet hex takes at most `infiltrate_facility_cap` of them ([`infiltrate_agents_on`], 282).
pub fn infiltrate_agents(w: &World) -> i64 {
    income_at(w, "adversary.infiltrate_agents").max(0)
}

/// 282: agents an Infiltrate lands on each site of type `site`: [`infiltrate_agents`] on a city, at most
/// `infiltrate_facility_cap` on a research station, factory or botnet hex.
pub fn infiltrate_agents_on(w: &World, site: SiteType) -> i64 {
    let n = infiltrate_agents(w);
    match site {
        SiteType::City | SiteType::CoastalCity => n,
        SiteType::Station | SiteType::Factory | SiteType::Botnet => n.min(w.int("adversary.infiltrate_facility_cap").max(0)),
    }
}

/// 275: agents the adversary has put on the board so far this game (each arrival, a canceled one included): a
/// line's placements are the difference (the supply's isn't: an arrival corruption returns agents mid-line).
fn arrived(g: &GameState) -> i64 {
    g.stats.counters.get("adversary.arrivals").copied().unwrap_or(0)
}

/// 275: the countries a line acts in: `scope`, or every country, less the cordoned ones (178).
fn countries_in(w: &World, g: &GameState, scope: Option<usize>) -> Vec<usize> {
    let all: Vec<usize> = match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    };
    all.into_iter().filter(|&c| !board::flagged(g, crate::content::RoundFlag::Cordon, c)).collect()
}

/// 275, Infiltrate X in country `scope` or every country: [`infiltrate_agents_on`] on each site of the type not yet
/// the adversary's (`plan::infiltrate_plan`), in hex order. A firewalled or no-Infiltrate site's agents stay in
/// the supply; a cordoned country takes none. Tallied `adversary.infiltrated` and by source; each site named is
/// one check (`map.adversary_checks`).
pub fn infiltrate(g: &mut GameState, cx: &mut Cx, site: SiteType, scope: Option<usize>, source: &str) -> Res {
    let w = cx.world;
    let n = infiltrate_agents_on(&w, site);
    for c in countries_in(&w, g, scope) {
        let sites = super::plan::infiltrate_sites(&w, g, site, c);
        cx.per_round(g, CHECKS, sites.len() as i64);
        // 285: a line with nothing to act on says so (the turn panel showed nothing at all).
        if sites.is_empty() {
            cx.note(None, || format!("Infiltrate ({}): none in {} not yet the adversary's", site.plural(), country_name(&w, c)));
        }
        let before = arrived(g);
        for h in sites {
            if let Some(wh) = super::plan::prohibition_on(g, Step::Infiltrate, h) {
                cx.tally(g, "adversary.infiltrate_blocked", n);
                prohibit_tally(g, cx, Step::Infiltrate, wh, n);
                cx.note(Some(h), || format!("Infiltrate ({}): a no-Infiltrate token keeps its agents off {}", site.plural(), hex_name(&w, h)));
                continue;
            }
            place_line(g, cx, h, n, &|| format!("Infiltrate ({})", site.plural()))?;
            if g.is_over() {
                return Ok(());
            }
        }
        let made = arrived(g) - before;
        cx.tally(g, "adversary.infiltrated", made);
        cx.tally(g, &format!("adversary.infiltrated.{source}"), made);
    }
    Ok(())
}

/// "The adversary builds now" (an out-of-turn Infiltrate: Grand Offensive, the Offensive Discord card), in
/// country `scope` or every country: this round's card's Infiltrate lines again (none: nothing).
pub fn build_all(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    let Some(id) = g.pile(round_pile(g)).first().cloned() else {
        return Ok(());
    };
    for site in infiltrate_sites(card(&w, &id)) {
        infiltrate(g, cx, site, scope, "build")?;
    }
    Ok(())
}

/// 275, Surge N: X: in each country in scope, `n` agents onto the X not yet the adversary's holding the most
/// agents (ties: the lowest-numbered); none of the type: nothing.
pub fn surge(g: &mut GameState, cx: &mut Cx, site: SiteType, n: i64, scope: Option<usize>, source: &str) -> Res {
    let w = cx.world;
    for c in countries_in(&w, g, scope) {
        let sites = super::plan::infiltrate_sites(&w, g, site, c);
        cx.per_round(g, CHECKS, sites.len() as i64);
        let Some(h) = super::plan::most_agents(&w, g, &sites) else {
            // 285: said, as an Infiltrate with no site is (a Surge with every site firewalled, too).
            cx.note(None, || {
                if sites.is_empty() {
                    format!("Surge {n} ({}): none in {} not yet the adversary's", site.plural(), country_name(&w, c))
                } else {
                    format!("Surge {n} ({}): every one in {} is firewalled", site.plural(), country_name(&w, c))
                }
            });
            continue;
        };
        let before = arrived(g);
        place_line(g, cx, h, n, &|| format!("Surge {n} ({})", site.plural()))?;
        let made = arrived(g) - before;
        cx.tally(g, "adversary.surged", made);
        cx.tally(g, &format!("adversary.surged.{source}"), made);
        if g.is_over() {
            return Ok(());
        }
    }
    Ok(())
}

/// 275, Spill: X into Y (event cards' lines, from `spill_at`; below it the line does nothing): in each country,
/// every X not yet the adversary's holding 2 or more agents moves 1 onto the Y next to it (not yet the
/// adversary's) holding the most agents (ties: the lower country, then number: 285, §2); none: it stays. The move is an
/// arrival.
pub fn spill(g: &mut GameState, cx: &mut Cx, site: SiteType, to: SiteType, scope: Option<usize>) -> Res {
    let w = cx.world;
    if !level(&w, w.int("adversary.spill_at")) {
        // 285: said, as the botnets' "don't spread at this difficulty" is.
        cx.note(None, || "Spill: not at this difficulty".to_string());
        return Ok(());
    }
    for c in countries_in(&w, g, scope) {
        let from: Vec<usize> = super::plan::infiltrate_sites(&w, g, site, c)
            .into_iter()
            .filter(|&h| g.tokens(h, ADV_AMBASSADOR) >= 2)
            .collect();
        cx.per_round(g, CHECKS, from.len() as i64);
        for h in from {
            // Its own hex is never its target (a city next to itself). 285: nor a site across a border in a
            // cordoned country.
            let near: Vec<usize> = w.map.hexes[h]
                .neighbors
                .iter()
                .copied()
                .filter(|&x| super::plan::infiltrate_fits(&w, g, x, to) && !firewalled(g, x) && !super::plan::cordoned(&w, g, x))
                .collect();
            let Some(t) = super::plan::most_agents(&w, g, &near) else {
                continue;
            };
            if g.tokens(h, ADV_AMBASSADOR) < 2 {
                continue;
            }
            cx.tally(g, "adversary.spilled", 1);
            // 285: named in the log (the turn's summary nets a move to nothing), before the arrival's own note.
            let landed = move_one(g, cx, h, t)?;
            cx.note(Some(t), || {
                let (from, to) = (hex_name(&w, h), hex_name(&w, t));
                if landed {
                    format!("Spill: an agent moves from {from} onto {to}")
                } else {
                    format!("Spill: an agent moving from {from} onto {to} cancels a player ambassador there")
                }
            });
            if landed {
                arrive_check(g, cx, t)?;
            }
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// 275: one agent moves from `from` onto `to` (a Spill): on a player ambassador the two cancel (r75: the players
/// choose whose). Whether it landed.
fn move_one(g: &mut GameState, cx: &mut Cx, from: usize, to: usize) -> Res<bool> {
    cx.add_tokens(g, from, ADV_AMBASSADOR, -1);
    cx.per_round(g, OPS, 1);
    if board::agent_cancels(g, cx, to)?.is_some() {
        cx.add_key(g, SUPPLY, 1);
        cx.tally(g, "adversary.canceled", 1);
        // 285: apart, as a Spill's agent comes from the board, not the supply (no `arrivals` tally to cancel).
        cx.tally(g, "adversary.spill_canceled", 1);
        return Ok(false);
    }
    cx.add_tokens(g, to, ADV_AMBASSADOR, 1);
    Ok(true)
}

/// 275, Grow: X (the adversary's growth; it replaced 190's city growth and its chips): in each country in scope,
/// the X holding the most agents (ties: the lowest-numbered) among those with a free hex next to it builds one
/// neutral facility there (`plan::grow_plan`): the type it has fewer of next to it (ties: a factory), on the
/// first free hex of that type (the lower country, then number: 285). A site holding `grow_infest_at` agents (1), or the adversary's, builds it
/// with 1 agent on it.
pub fn grow(g: &mut GameState, cx: &mut Cx, site: SiteType, scope: Option<usize>) -> Res {
    let w = cx.world;
    for c in countries_in(&w, g, scope) {
        let sites = super::plan::grow_sites(&w, g, site, c);
        cx.per_round(g, CHECKS, sites.len() as i64);
        let Some((at, z, factory)) = super::plan::grow_plan(&w, g, site, c) else {
            cx.tally(g, "adversary.grow_none", 1);
            // 285: said, as the botnets' "don't spread at this difficulty" is.
            cx.note(None, || format!("Grow ({}): none in {} has room to grow", site.plural(), country_name(&w, c)));
            continue;
        };
        let (token, name) = if factory { (FACTORY, "factory") } else { (STATION, "station") };
        cx.add_tokens(g, z, token, 1);
        cx.per_round(g, OPS, 1);
        cx.tally(g, &format!("adversary.grew.{name}"), 1);
        cx.per_round(g, &format!("map.built.{name}"), 1);
        let infested = g.tokens(at, ADV_AMBASSADOR) >= w.int("adversary.grow_infest_at").max(1) || corrupted(g, at);
        // 285: the note follows the agent's placement (a firewall, a cordon, an empty supply or an ambassador there
        // stops it); a fresh site's one agent corrupts nothing, so an arrival's own note rarely comes first.
        let canceled = |g: &GameState| g.stats.counters.get("adversary.canceled").copied().unwrap_or(0);
        let (sent, gone) = (arrived(g), canceled(g));
        if infested {
            place_named(g, cx, z, 1)?;
        }
        let landed = arrived(g) > sent && canceled(g) == gone;
        let agent = match (infested, landed) {
            (false, _) => "",
            (true, true) => ", an agent on it",
            (true, false) if arrived(g) > sent => ", its agent canceled by an ambassador there",
            (true, false) => ", no agent lands on it",
        };
        cx.note(Some(z), || format!("{} grows a neutral {name} at {}{agent}", hex_name(&w, at), hex_name(&w, z)));
        if landed {
            cx.tally(g, "adversary.grew_infested", 1);
        }
        if g.is_over() {
            return Ok(());
        }
    }
    Ok(())
}

/// What one pass over the botnets made: compute, and compute a player ambassador or a hack stopped.
pub struct BotnetPass {
    pub compute: i64,
    pub blocked: i64,
}

/// 275: botnets seed, in the Income beat: each botnet with no player ambassador on it places
/// `adversary.botnet_seed` agents on the site next to it not yet the adversary's holding the most agents (ties:
/// the lower country, then number); none: on its own hex, if that site is not yet the adversary's. r133 (280, one move a
/// botnet): the same read adds its `botnet_compute` (none while hacked; a cordon stops the seed, not the compute),
/// so the compute step reads no botnet again. In hex order, each botnet read as its turn comes; a botnet passed as
/// blocked whose ambassador a later seed cancels (or whose site that seed corrupts) adds its compute then, as r132's
/// second read did: play is unchanged.
pub fn botnets_seed(g: &mut GameState, cx: &mut Cx) -> Res<BotnetPass> {
    let w = cx.world;
    let n = w.int("adversary.botnet_seed");
    let per = w.int("adversary.botnet_compute");
    let mut pass = BotnetPass { compute: 0, blocked: 0 };
    // Botnets passed under a player ambassador: (hex, compute, hacked).
    let mut stood: Vec<(usize, i64, bool)> = Vec::new();
    let botnets: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, BOTNET) > 0).collect();
    for b in botnets {
        let made = g.tokens(b, BOTNET) * per;
        if g.tokens(b, BOTNET) == 0 || made == 0 && n <= 0 {
            continue;
        }
        let stood_on = player_ambassador(g, b).is_some();
        // A hack's cut (021) lasts this one step, on a botnet that makes compute.
        let hacked = made != 0 && g.var(&super::cut_key(b)) > 0;
        if hacked {
            cx.set_key(g, &super::cut_key(b), 0);
        }
        if stood_on || hacked {
            pass.blocked += made;
        } else {
            pass.compute += made;
        }
        if stood_on {
            stood.push((b, made, hacked));
        }
        if stood_on || n <= 0 {
            continue;
        }
        cx.per_round(g, CHECKS, 1);
        let c = w.map.hexes[b].country as usize;
        if board::flagged(g, crate::content::RoundFlag::Cordon, c) {
            continue;
        }
        let near: Vec<usize> =
            w.map.hexes[b].neighbors.iter().copied().filter(|&x| super::plan::seed_fits(&w, g, x)).collect();
        let target = super::plan::most_agents(&w, g, &near).or(Some(b).filter(|&x| super::plan::seed_fits(&w, g, x)));
        let Some(t) = target else {
            // 285: said (a botnet with no seed note read as missed).
            cx.note(Some(b), || format!("The botnet at {} has nowhere to seed", hex_name(&w, b)));
            continue;
        };
        let before = arrived(g);
        place_noted(g, cx, t, n, &|| format!("The botnet at {} seeds", hex_name(&w, b)))?;
        cx.tally(g, "adversary.seeded", arrived(g) - before);
        if g.is_over() {
            return Ok(pass);
        }
        // The seed canceled the last ambassador on a botnet already passed as blocked: it adds its compute now.
        if let Some(i) = stood.iter().position(|&(h, _, _)| h == t) {
            if player_ambassador(g, t).is_none() {
                let (_, made, hacked) = stood.remove(i);
                if !hacked {
                    pass.blocked -= made;
                    pass.compute += made;
                }
            }
        }
    }
    Ok(pass)
}

/// 138, a creation line: `count` agents on each of the
/// `sites` lowest-numbered matching sites in each country in scope (one
/// source with the agents' forecast, [`super::plan::create_plan`]). A
/// cordoned country gains none. Tallied by `source` (event, threat, project,
/// discord, unity, other).
pub fn create(g: &mut GameState, cx: &mut Cx, line: super::plan::CreateLine, scope: Option<usize>, near: &[usize], location: Option<usize>, source: &str) -> Res {
    let w = cx.world;
    let mut made = 0;
    let plan = super::plan::create_plan(&w, g, line, scope, near, location);
    // 285: a line with nothing to act on says so (Firestorm with no city holding agents was silent).
    let cordon = match scope {
        Some(c) => board::flagged(g, crate::content::RoundFlag::Cordon, c),
        None => (0..w.map.countries.len()).all(|c| board::flagged(g, crate::content::RoundFlag::Cordon, c)),
    };
    if plan.is_empty() {
        let why = if cordon { "a cordon keeps its agents out" } else { "no site for its agents" };
        cx.note(None, || format!("{}: {why}", match source {
            "threat" => "A threat's line",
            "discord" => "A Discord card's line",
            "unity" => "A Unity card's line",
            "project" => "A project's line",
            "event" => "The adversary card's line",
            _ => "A creation line",
        }));
    }
    for (h, n) in plan {
        let before = arrived(g);
        // 285: named in the log (the turn panel showed "+2 agent" with no hex).
        let what = || match source {
            "threat" => "A threat's line".to_string(),
            "discord" => "A Discord card's line".to_string(),
            "unity" => "A Unity card's line".to_string(),
            "project" => "A project's line".to_string(),
            "event" => "The adversary card's line".to_string(),
            _ => "A creation line".to_string(),
        };
        place_noted(g, cx, h, n, &what)?;
        made += arrived(g) - before;
        if g.is_over() {
            break;
        }
    }
    cx.tally(g, "adversary.created", made);
    cx.tally(g, &format!("adversary.created.{source}"), made);
    Ok(())
}
