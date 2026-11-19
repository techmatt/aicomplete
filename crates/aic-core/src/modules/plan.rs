//! Read-only forecasts of rules steps: what a step would do on the current
//! table, without doing it. Not a module: no phases, no state. Everything here
//! reads public information only.
//!
//! The adversary's creation lines and its card lines' targets live here (138; 015's
//! deferred call 9: the agents' heuristic used to copy the build):
//! `adversary::create` places what [`create_plan`] says, and (275) `adversary::infiltrate`, `surge`, `spill` and
//! `grow` act on what [`infiltrate_sites`], [`most_agents`] and [`grow_plan`] name, so the forecast in
//! [`next_infiltrate`] can't drift from the rule. The troops' forecast is `board::troop_plan`.

use crate::content::{Op, RoundFlag};
use crate::module::World;
use crate::op::{CorruptSite, CreateAt, CreateIf, SiteType, Step, Where};
use crate::state::GameState;

use super::{adversary, board};
use super::common::{
    buildable, is_base, cities_unowned, city_host, corrupted, cube_owner, firewalled, has_building, is_city, is_ocean, next_to, node_owner, player_ambassador, ADV_AMBASSADOR,
    FALLOUT, BOTNET, FACTORY, STATION, prohibit_token,
};

/// A creation line's printed words (138): `count` on each of the `sites`
/// lowest-numbered `at` meeting `if_`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateLine {
    pub count: i64,
    pub at: CreateAt,
    pub if_: CreateIf,
    pub sites: i64,
}

impl CreateLine {
    pub fn of(op: &Op) -> Option<CreateLine> {
        match op {
            Op::Create { count, at, if_, sites } => Some(CreateLine { count: *count, at: *at, if_: *if_, sites: *sites }),
            _ => None,
        }
    }
}

/// Whether hex `h` is a site a creation line at `at` with `if_` lands on
/// (`near`: the threat's hexes; `location`: its token).
fn create_fits(w: &World, g: &GameState, h: usize, line: CreateLine, near: &[usize], location: Option<usize>) -> bool {
    let site = match line.at {
        CreateAt::City => is_city(w, h),
        CreateAt::CoastalCity => is_city(w, h) && w.map.hexes[h].coastal,
        CreateAt::Station => g.tokens(h, STATION) > 0,
        CreateAt::Factory => g.tokens(h, FACTORY) > 0,
        CreateAt::Botnet => g.tokens(h, BOTNET) > 0,
        CreateAt::Location => location == Some(h),
    };
    site && match line.if_ {
        CreateIf::Any => true,
        CreateIf::NoAmbassadors => g.tokens(h, ADV_AMBASSADOR) == 0,
        CreateIf::Ambassadors => g.tokens(h, ADV_AMBASSADOR) > 0,
        CreateIf::Clean => !corrupted(g, h),
        CreateIf::Adversary => corrupted(g, h),
        CreateIf::Player => cube_owner(g, h).is_some(),
        CreateIf::Near => near.contains(&h),
    }
}

/// What creation line `line` places in country `scope` or every country:
/// `(hex, ambassadors)`, in hex order. In each country (not a cordoned one,
/// 068) the `sites` lowest-numbered matching sites each gain `count`. The
/// supply is not checked: the plan is what the line asks for.
pub fn create_plan(
    w: &World,
    g: &GameState,
    line: CreateLine,
    scope: Option<usize>,
    near: &[usize],
    location: Option<usize>,
) -> Vec<(usize, i64)> {
    let countries: Vec<usize> = match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    };
    let mut plan = Vec::new();
    for c in countries {
        if board::flagged(g, RoundFlag::Cordon, c) {
            continue;
        }
        let mut hexes: Vec<usize> =
            // 158: a firewalled hex takes no arrival; the line goes to the next site.
            w.map.countries[c].hexes.iter().copied().filter(|&h| !firewalled(g, h) && create_fits(w, g, h, line, near, location)).collect();
        hexes.sort_by_key(|&h| w.map.hexes[h].number);
        for h in hexes.into_iter().take(line.sites.max(0) as usize) {
            if line.count > 0 {
                plan.push((h, line.count));
            }
        }
    }
    plan.sort_unstable();
    plan
}

/// Whether hex `h` is a target of Corrupt `site` (not yet the adversary's;
/// land: an empty land hex, no city, base, building or Fallout; 175: a node: a hex with a player's node).
pub fn corrupt_target(w: &World, g: &GameState, h: usize, site: CorruptSite) -> bool {
    match site {
        CorruptSite::City => is_city(w, h) && !corrupted(g, h),
        CorruptSite::CoastalCity => is_city(w, h) && w.map.hexes[h].coastal && !corrupted(g, h),
        CorruptSite::Station => g.tokens(h, STATION) > 0 && !corrupted(g, h),
        CorruptSite::Factory => g.tokens(h, FACTORY) > 0 && !corrupted(g, h),
        CorruptSite::Node => node_owner(g, h).is_some(),
    }
}

/// 275: whether hex `h` is a site of type `at` (an Infiltrate, Surge, Spill or Grow line's type).
pub fn site_fits(w: &World, g: &GameState, h: usize, at: SiteType) -> bool {
    match at {
        SiteType::City => is_city(w, h),
        SiteType::CoastalCity => is_city(w, h) && w.map.hexes[h].coastal,
        SiteType::Station => g.tokens(h, STATION) > 0,
        SiteType::Factory => g.tokens(h, FACTORY) > 0,
        SiteType::Botnet => g.tokens(h, BOTNET) > 0,
    }
}

/// 275: whether an Infiltrate, Surge or Spill of type `at` lands on hex `h`: a site of the type not yet the
/// adversary's (a corrupted one takes no agent).
pub fn infiltrate_fits(w: &World, g: &GameState, h: usize, at: SiteType) -> bool {
    site_fits(w, g, h, at) && !corrupted(g, h)
}

/// 275, the sites an Infiltrate, Surge or Spill of type `at` names in country `c`: every one not yet the
/// adversary's, firewalled ones included (their agents stay in the supply), in hex order.
pub fn infiltrate_sites(w: &World, g: &GameState, at: SiteType, c: usize) -> Vec<usize> {
    let mut all: Vec<usize> = w.map.countries[c].hexes.iter().copied().filter(|&h| infiltrate_fits(w, g, h, at)).collect();
    all.sort_by_key(|&h| w.map.hexes[h].number);
    all
}

/// 275: of `sites`, the one holding the most agents (ties: the lowest-numbered); none: empty. A firewalled one is
/// passed over (no arrival lands there).
pub fn most_agents(w: &World, g: &GameState, sites: &[usize]) -> Option<usize> {
    sites
        .iter()
        .copied()
        .filter(|&h| !firewalled(g, h))
        .min_by_key(|&h| (std::cmp::Reverse(g.tokens(h, ADV_AMBASSADOR)), w.map.hexes[h].number))
}

/// 275: whether a botnet's seed may land on hex `h`: a city, research station or factory not yet the adversary's.
pub fn seed_fits(w: &World, g: &GameState, h: usize) -> bool {
    (is_city(w, h) || g.tokens(h, STATION) > 0 || g.tokens(h, FACTORY) > 0) && !corrupted(g, h) && !firewalled(g, h)
}

/// 275: the free hexes next to `at` where a Grow builds a factory (`true`: a free industrial zone) or a research
/// station (open land that is no industrial zone, no base), in hex order.
fn grow_hexes(w: &World, g: &GameState, at: usize, factory: bool) -> Vec<usize> {
    let mut v: Vec<usize> = w.map.hexes[at]
        .neighbors
        .iter()
        .copied()
        .filter(|&h| {
            if factory {
                w.map.hexes[h].industrial && !has_building(g, h) && g.tokens(h, FALLOUT) == 0
            } else {
                buildable(w, g, h) && !w.map.hexes[h].industrial && !is_base(w, h)
            }
        })
        .collect();
    v.sort_by_key(|&h| w.map.hexes[h].number);
    v
}

/// 275: what Grow site `at` builds: the type it has fewer of next to it (any owner; ties: a factory) if it has a
/// free hex for it, else the other type; `(hex, factory)`, none: it can't grow.
pub fn grow_at(w: &World, g: &GameState, at: usize) -> Option<(usize, bool)> {
    let near = &w.map.hexes[at].neighbors;
    let f = near.iter().filter(|&&h| g.tokens(h, FACTORY) > 0).count();
    let s = near.iter().filter(|&&h| g.tokens(h, STATION) > 0).count();
    let first = f <= s;
    [first, !first].into_iter().find_map(|factory| grow_hexes(w, g, at, factory).first().map(|&z| (z, factory)))
}

/// 275, Grow X's candidates in country `c`: every site of the type (the adversary's too) with a free hex next to
/// it and fewer than `adversary.grow_cap` facilities next to it (any owner; 0: no cap), in hex order.
pub fn grow_sites(w: &World, g: &GameState, at: SiteType, c: usize) -> Vec<usize> {
    let cap = w.int("adversary.grow_cap");
    let room = |h: usize| {
        cap <= 0 || (w.map.hexes[h].neighbors.iter().filter(|&&x| g.tokens(x, FACTORY) + g.tokens(x, STATION) > 0).count() as i64) < cap
    };
    let mut all: Vec<usize> = w.map.countries[c]
        .hexes
        .iter()
        .copied()
        .filter(|&h| site_fits(w, g, h, at) && room(h) && grow_at(w, g, h).is_some())
        .collect();
    all.sort_by_key(|&h| w.map.hexes[h].number);
    all
}

/// 275, what Grow X builds in country `c`: `(the site that grows, the hex it builds on, factory)`: the candidate
/// holding the most agents (ties: the lowest-numbered). None: no site of the type can grow.
pub fn grow_plan(w: &World, g: &GameState, at: SiteType, c: usize) -> Option<(usize, usize, bool)> {
    let sites = grow_sites(w, g, at, c);
    let site = sites.iter().copied().min_by_key(|&h| (std::cmp::Reverse(g.tokens(h, ADV_AMBASSADOR)), w.map.hexes[h].number))?;
    let (z, factory) = grow_at(w, g, site)?;
    Some((site, z, factory))
}

/// 275, the agents' forecast of the botnets' next seeding (public: the botnets and the agents on the board): the
/// agents each botnet with no player ambassador on it would land now, by hex (as `adversary::botnets_seed` picks;
/// a cordoned country's botnets seed nothing: a cordon set now still stands at the next seeding).
pub fn next_seeds(w: &World, g: &GameState) -> Vec<(usize, f64)> {
    let n = w.int("adversary.botnet_seed");
    if n <= 0 {
        return Vec::new();
    }
    (0..g.hexes.len())
        .filter(|&b| g.tokens(b, BOTNET) > 0 && player_ambassador(g, b).is_none())
        .filter(|&b| !board::flagged(g, RoundFlag::Cordon, w.map.hexes[b].country as usize))
        .filter_map(|b| {
            let near: Vec<usize> = w.map.hexes[b].neighbors.iter().copied().filter(|&x| seed_fits(w, g, x)).collect();
            most_agents(w, g, &near).or(Some(b).filter(|&x| seed_fits(w, g, x))).map(|t| (t, n as f64))
        })
        .collect()
}

/// The card whose Card beat comes next: the face-up one, or this round's until its build has run.
fn card_ahead<'w>(w: &World<'w>, g: &GameState) -> Option<&'w crate::content::AdversaryCard> {
    let build_ahead = g.config.round.iter().position(|p| p == "adversary.build").is_some_and(|i| g.phase <= i);
    let id = match g.pile(adversary::NEXT).first() {
        None if build_ahead => g.pile(adversary::CURRENT).first(),
        next => next,
    }?;
    Some(adversary::card(w, id))
}

/// 275, the agents' forecast of the coming Infiltrate lines (public: the card ahead): the agents landing on each
/// hex, exact (`infiltrate_agents_on` every site of each named type not yet the adversary's, less a prohibited,
/// firewalled or cordoned one). Empty when no card is ahead.
pub fn next_infiltrate(w: &World, g: &GameState) -> Vec<(usize, f64)> {
    let build_ahead = g.config.round.iter().position(|p| p == "adversary.build").is_some_and(|i| g.phase <= i);
    let Some(card) = card_ahead(w, g) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for site in adversary::infiltrate_sites(card) {
        let n = adversary::infiltrate_agents_on(w, site) as f64;
        for c in 0..w.map.countries.len() {
            // A cordon counts down at cleanup and comes off at the end of the adversary turn (178): before this
            // round's build it blocks this round's lines; after it, only one played this round (still at 2) lasts.
            let cordon = g.var(&board::flag(RoundFlag::Cordon, c));
            if cordon >= if build_ahead { 1 } else { 2 } {
                continue;
            }
            for h in infiltrate_sites(w, g, site, c) {
                if !prohibited_now(g, Step::Infiltrate, h) && !firewalled(g, h) {
                    out.push((h, n));
                }
            }
        }
    }
    out
}

/// 167: whether board condition `wh` holds on hex `h` now.
pub fn holds(w: &World, g: &GameState, h: usize, wh: Where) -> bool {
    let amb = |x: usize| player_ambassador(g, x).is_some();
    match wh {
        Where::Ambassador => amb(h),
        Where::ByAmbassador => next_to(w, h, amb),
        Where::ByPlayer => next_to(w, h, |x| amb(x) || node_owner(g, x).is_some()),
        Where::NearPlayer => next_to(w, h, |y| next_to(w, y, |x| amb(x) || node_owner(g, x).is_some())),
        Where::AmbassadorCity => is_city(w, h) && amb(h),
        Where::AmbassadorStation => g.tokens(h, STATION) > 0 && amb(h),
        Where::AmbassadorFactory => g.tokens(h, FACTORY) > 0 && amb(h),
        Where::Contested => g.tokens(h, ADV_AMBASSADOR) > 0 && amb(h),
        Where::CorruptedCity => is_city(w, h) && corrupted(g, h),
        Where::ByCorruptedCity => next_to(w, h, |x| is_city(w, x) && corrupted(g, x)),
        Where::Corrupted => corrupted(g, h),
        // 254: with no city owned (`cities_unowned`), a hosted city counts as a player's for the Unity cards that pay
        // per site a player owns (the bound cities were most of those sites).
        Where::Owned => {
            cube_owner(g, h).is_some() || (cities_unowned(w) && is_city(w, h) && city_host(w, g, h).is_some())
        }
    }
}

/// 167: the hexes where `wh` holds, in board order.
pub fn sites_where(w: &World, g: &GameState, wh: Where) -> Vec<usize> {
    (0..g.hexes.len()).filter(|&h| !is_ocean(w, h) && holds(w, g, h, wh)).collect()
}

/// 167: a prohibition's state: the round of the adversary turn it acts in, and the conditions (a bit per
/// [`Where::ALL`] index). Static keys: [`prohibition_on`] runs inside the agents' forecasts.
pub fn prohibit_keys(step: Step) -> (&'static str, &'static str) {
    match step {
        Step::Infiltrate => ("prohibit.infiltrate.round", "prohibit.infiltrate.where"),
        Step::Muster => ("prohibit.muster.round", "prohibit.muster.where"),
        Step::Corrupt => ("prohibit.corrupt.round", "prohibit.corrupt.where"),
    }
}

/// r80 (ruling #53): whether hex `h` holds a `step` prohibition's token (placed when the card resolved, off at
/// the end of the next adversary turn).
pub fn prohibited_now(g: &GameState, step: Step, h: usize) -> bool {
    g.tokens(h, prohibit_token(step)) > 0
}

/// The condition of the `step` prohibition whose token stands on `h` (r80, ruling #53: the token decides, placed
/// where the condition held when the card resolved; the condition is only its label). A token standing now acts
/// in the next adversary turn, whichever round that is.
pub fn prohibition_on(g: &GameState, step: Step, h: usize) -> Option<Where> {
    if !prohibited_now(g, step, h) {
        return None;
    }
    let mask = g.var(prohibit_keys(step).1);
    Some(Where::ALL.iter().enumerate().find(|(i, _)| mask & (1 << i) != 0).map_or(Where::ALL[0], |(_, wh)| *wh))
}

/// 275: the corrupt number an arrival on a `site` meets (arrival corruption: no Corrupt line, no gathering).
pub fn next_corrupt_n(w: &World, _g: &GameState, site: CorruptSite) -> i64 {
    adversary::corrupt_n(w, site, 0)
}
