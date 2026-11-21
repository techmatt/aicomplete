//! 275, 293: the adversary's card-driven lines: Infiltrate X and Escalate (293: it replaced Spill, Surge and Grow),
//! the one output of a corrupted factory (its agent, pushed along its arrow) and of a botnet (its compute; 293: no
//! seeding), out-of-turn Infiltrates and creation lines. Every step acts only on the sites its line names; each
//! arrival is checked where it lands ([`arrive_check`]).

use super::*;
use crate::op::SiteType;

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

/// 275: agents an Infiltrate lands on each city at this level (`infiltrate_agents`, by difficulty); a station,
/// factory or botnet hex takes at most `infiltrate_facility_cap` of them ([`infiltrate_agents_on`], 282).
pub fn infiltrate_agents(w: &World) -> i64 {
    at_level(w, "adversary.infiltrate_agents").max(0)
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

/// 293 (Matt, 2026-11-19), Escalate: X, in country `scope` or every country: each site of the type not yet the
/// adversary's that holds an agent takes 1 more, in hex order (the sites are read before any lands). It picks
/// nothing. The event card's own escalate is [`escalate_last`]; a project's or Discord card's line comes here too.
/// A firewalled or no-Infiltrate site's agent stays in the supply; a cordoned country takes none.
pub fn escalate(g: &mut GameState, cx: &mut Cx, site: SiteType, scope: Option<usize>, source: &str) -> Res {
    let w = cx.world;
    for c in countries_in(&w, g, scope) {
        let sites: Vec<usize> =
            super::plan::infiltrate_sites(&w, g, site, c).into_iter().filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 0).collect();
        cx.per_round(g, CHECKS, sites.len() as i64);
        if sites.is_empty() {
            cx.note(None, || format!("Escalate ({}): none in {} holds an agent", site.plural(), country_name(&w, c)));
        }
        let before = arrived(g);
        for h in sites {
            if let Some(wh) = super::plan::prohibition_on(g, Step::Infiltrate, h) {
                cx.tally(g, "adversary.infiltrate_blocked", 1);
                prohibit_tally(g, cx, Step::Infiltrate, wh, 1);
                cx.note(Some(h), || format!("Escalate ({}): a no-Infiltrate token keeps its agent off {}", site.plural(), hex_name(&w, h)));
                continue;
            }
            place_line(g, cx, h, 1, &|| format!("Escalate ({})", site.plural()))?;
            if g.is_over() {
                return Ok(());
            }
        }
        let made = arrived(g) - before;
        cx.tally(g, "adversary.escalated", made);
        cx.tally(g, &format!("adversary.escalated.{source}"), made);
    }
    Ok(())
}

/// 293: the site types last round's event card named (its Infiltrate lines, in card order): what escalates this
/// round. Read off the card in the board's "last round" space ([`LAST`]); none in round 1.
pub fn escalating(w: &World, g: &GameState) -> Vec<SiteType> {
    let mut out: Vec<SiteType> = Vec::new();
    for id in g.pile(LAST) {
        for site in infiltrate_sites(card(w, id)) {
            if !out.contains(&site) {
                out.push(site);
            }
        }
    }
    out
}

/// 293: the site types the next Card beat escalates, as the view marks them: between this round's flip and its
/// Card beat, last round's card's ([`escalating`]); otherwise this round's card's (it moves to the last-round
/// space at the next flip), then the board rule's own Escalate type, if it prints one. A round with no event
/// phase flips inside its Card beat, so there the card is never "flipped, Card beat ahead".
pub fn next_escalating(w: &World, g: &GameState) -> Vec<SiteType> {
    let flipped = g.phase_index("adversary.event").is_some_and(|i| g.phase > i);
    let build_ahead = g.phase_index("adversary.build").is_some_and(|i| g.phase <= i);
    let mut out = if flipped && build_ahead {
        escalating(w, g)
    } else {
        g.pile(CURRENT).iter().flat_map(|id| infiltrate_sites(card(w, id))).collect()
    };
    for e in w.content.adversary_board.rules.iter().flat_map(|r| r.effects.iter()) {
        if let Op::Escalate { site } = e {
            out.push(*site);
        }
    }
    let mut seen: Vec<SiteType> = Vec::new();
    out.retain(|s| {
        let new = !seen.contains(s);
        seen.push(*s);
        new
    });
    out
}

/// 293, the event card's escalate: each site of last round's type(s) that holds an agent takes 1 more
/// ([`escalate`]), before this round's Infiltrate.
pub fn escalate_last(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    for site in escalating(&w, g) {
        escalate(g, cx, site, None, "event")?;
        if g.is_over() {
            break;
        }
    }
    Ok(())
}

/// 293: one agent moves from `from` onto `to` (a factory's agent, along its arrow): on a player ambassador the two
/// cancel (r75: the players choose whose). Whether it landed.
fn move_one(g: &mut GameState, cx: &mut Cx, from: usize, to: usize) -> Res<bool> {
    cx.add_tokens(g, from, ADV_AMBASSADOR, -1);
    cx.per_round(g, OPS, 1);
    if board::agent_cancels(g, cx, to)?.is_some() {
        cx.add_key(g, SUPPLY, 1);
        cx.tally(g, "adversary.canceled", 1);
        // Apart, as a moved agent comes from the board, not the supply (no `arrivals` tally to cancel).
        cx.tally(g, "adversary.push_canceled", 1);
        return Ok(false);
    }
    cx.add_tokens(g, to, ADV_AMBASSADOR, 1);
    Ok(true)
}

/// 293 (Matt, 2026-11-19: one output a corrupted site), a corrupted factory's output, in the Income beat, in hex
/// order: its agents move along its zone's printed arrow onto its city (`plan::factory_arrow`; an arrival), unless
/// that city is already the adversary's, firewalled, under a no-Infiltrate token or in a cordoned country; then a
/// factory holding no agent makes 1 on itself. So a factory feeds its city one agent a round, and one whose city
/// has fallen keeps a single guard. A player ambassador on it, or a troop occupying it (135), blocks both; a
/// cordoned country's factories don't act.
pub fn factories_output(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let factories: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, FACTORY) > 0 && corrupted(g, h)).collect();
    for h in factories {
        if g.tokens(h, FACTORY) == 0 || !corrupted(g, h) {
            continue;
        }
        if board::occupied(g, h) {
            cx.tally(g, "board.factories_occupied", 1);
            continue;
        }
        if player_ambassador(g, h).is_some() {
            cx.tally(g, "board.blocked.factory_adversary", 1);
            continue;
        }
        if super::plan::cordoned(&w, g, h) {
            continue;
        }
        cx.per_round(g, CHECKS, 1);
        let held = g.tokens(h, ADV_AMBASSADOR);
        let city = super::plan::factory_arrow(&w, h).filter(|&t| super::plan::push_lands(&w, g, t));
        if let (Some(t), true) = (city, held > 0) {
            let mut landed: i64 = 0;
            for _ in 0..held {
                if move_one(g, cx, h, t)? {
                    landed += 1;
                }
            }
            cx.tally(g, "adversary.factory_pushed", landed);
            cx.note(Some(t), || {
                let (from, to) = (hex_name(&w, h), hex_name(&w, t));
                let n = if held == 1 { "its agent".to_string() } else { format!("its {held} agents") };
                if landed == held {
                    format!("The factory at {from} pushes {n} along its arrow onto {to}")
                } else {
                    format!("The factory at {from} pushes {n} along its arrow onto {to}: {} canceled by player ambassadors", held - landed)
                }
            });
            if landed > 0 {
                arrive_check(g, cx, t)?;
            }
            if g.is_over() {
                return Ok(());
            }
        }
        if g.tokens(h, ADV_AMBASSADOR) == 0 && g.tokens(h, FACTORY) > 0 && corrupted(g, h) {
            let before = arrived(g);
            place_noted(g, cx, h, 1, &|| format!("The factory at {} makes an agent", hex_name(&w, h)))?;
            cx.tally(g, "adversary.factory_agents", arrived(g) - before);
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// What one pass over the botnets made: compute, and compute a player ambassador or a hack stopped.
pub struct BotnetPass {
    pub compute: i64,
    pub blocked: i64,
}

/// 293 (Matt, 2026-11-19): a botnet's one output is its `botnet_compute` compute (no seeding): none under a player
/// ambassador or while hacked (021: a hack's cut lasts this one step).
pub fn botnets_compute(g: &mut GameState, cx: &mut Cx) -> BotnetPass {
    let per = cx.world.int("adversary.botnet_compute");
    let mut pass = BotnetPass { compute: 0, blocked: 0 };
    let botnets: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, BOTNET) > 0).collect();
    for b in botnets {
        let made = g.tokens(b, BOTNET) * per;
        if made == 0 {
            continue;
        }
        cx.per_round(g, CHECKS, 1);
        let hacked = g.var(&super::cut_key(b)) > 0;
        if hacked {
            cx.set_key(g, &super::cut_key(b), 0);
        }
        if hacked || player_ambassador(g, b).is_some() {
            pass.blocked += made;
        } else {
            pass.compute += made;
        }
    }
    pass
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
