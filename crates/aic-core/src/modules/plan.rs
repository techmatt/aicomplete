//! Read-only forecasts of rules steps, for agents and clients: what a step
//! would do on the current table, without doing it. Not a module: no
//! phases, no state. Everything here reads public information only.
//!
//! The adversary's build step lives here (015's deferred call 9: the agents'
//! heuristic used to copy it): `adversary::build_all` places what
//! [`build_plan`] says, and the build's escalation what [`escalation_hexes`]
//! says, so the forecast in [`next_build`] can't drift from the rule. The
//! troops' forecast is `board::troop_plan`.

use crate::content::Op;
use crate::op::Site;
use crate::module::World;
use crate::state::GameState;

use super::adversary;
use super::common::{clean_city, corrupted, corrupted_city, is_base, scope_hexes, FACTORY, STATION};

/// What the adversary's build step places with `twists` in force, in country
/// `scope` or everywhere: `(hex, ambassadors)` for every hex that gains any,
/// in hex order. Each corrupted city gains `city_build` (plus `build_bonus`
/// at difficulty 3 or more in play, plus city-build twists); each factory
/// next to a corrupted city `factory_build`, and next to a clean city the
/// clean-factory twists; each corrupted station the station twists; each
/// site the card names its arrivals (027). The supply is not checked: the
/// plan is what the step asks for.
pub fn build_plan(
    w: &World,
    g: &GameState,
    twists: &[&Op],
    scope: Option<usize>,
) -> Vec<(usize, i64)> {
    let sum = |f: fn(&Op) -> Option<i64>| -> i64 { twists.iter().filter_map(|t| f(t)).sum() };
    let mut city = w.int("adversary.city_build");
    if adversary::level(w, 3) {
        city += w.int("adversary.build_bonus");
    }
    city += sum(|t| match t {
        Op::CityBuild { amount } => Some(*amount),
        _ => None,
    });
    let clean = sum(|t| match t {
        Op::CleanFactories { amount } => Some(*amount),
        _ => None,
    });
    let stations = sum(|t| match t {
        Op::StationBuild { amount } => Some(*amount),
        _ => None,
    });
    let factory = w.int("adversary.factory_build");
    let arrivals = w.int("adversary.arrivals");
    let sites: Vec<Site> = twists
        .iter()
        .filter_map(|t| match t {
            Op::Arrivals { site } => Some(*site),
            _ => None,
        })
        .collect();
    let mut plan = Vec::new();
    for h in scope_hexes(w, scope) {
        let mut n = 0;
        if corrupted_city(w, g, h) {
            n += city;
        }
        if g.tokens(h, FACTORY) > 0 {
            let adj =
                |pred: &dyn Fn(usize) -> bool| w.map.hexes[h].neighbors.iter().any(|&x| pred(x));
            if adj(&|x| corrupted_city(w, g, x)) {
                n += factory;
            }
            if adj(&|x| clean_city(w, g, x)) {
                n += clean;
            }
        }
        if g.tokens(h, STATION) > 0 && corrupted(g, h) {
            n += stations;
        }
        n += arrivals * sites.iter().filter(|&&s| is_site(w, g, h, s)).count() as i64;
        if n > 0 {
            plan.push((h, n));
        }
    }
    plan
}

/// Whether hex `h` is a site of type `site`.
fn is_site(w: &World, g: &GameState, h: usize, site: Site) -> bool {
    match site {
        Site::Coastal => w.map.hexes[h].coastal,
        Site::Industrial => w.map.hexes[h].industrial,
        Site::Stations => g.tokens(h, STATION) > 0,
        Site::Bases => site_base(w, h),
    }
}

/// Whether arrivals and escalation land on base hex `h` (053: each country's
/// first base, its lowest-numbered, at `base_sites_per_country` 1, so what
/// reaches bases counts countries, not fronts).
fn site_base(w: &World, h: usize) -> bool {
    let at = &w.map.hexes[h];
    is_base(w, h)
        && (w.int("adversary.base_sites_per_country") == 0
            || !(0..w.map.hexes.len()).any(|x| {
                is_base(w, x) && w.map.hexes[x].country == at.country && w.map.hexes[x].number < at.number
            }))
}

/// How many ambassadors `twists`' arrivals bring in `scope` (a tally).
pub fn arrival_count(w: &World, g: &GameState, twists: &[&Op], scope: Option<usize>) -> i64 {
    let per = w.int("adversary.arrivals");
    let hexes = scope_hexes(w, scope);
    twists
        .iter()
        .filter_map(|t| match t {
            Op::Arrivals { site } => Some(*site),
            _ => None,
        })
        .map(|s| per * hexes.iter().filter(|&&h| is_site(w, g, h, s)).count() as i64)
        .sum()
}

/// What an escalating card places in its build: `(hex, ambassadors)` for
/// each base it reaches ([`site_base`]), in hex order (none at `escalation_ambassadors` 0).
pub fn escalation_hexes(w: &World, g: &GameState) -> Vec<(usize, i64)> {
    let n = w.int("adversary.escalation_ambassadors");
    if n <= 0 {
        return Vec::new();
    }
    (0..g.hexes.len())
        .filter(|&h| site_base(w, h))
        .map(|h| (h, n))
        .collect()
}

/// The next round's build, read off the face-up adversary card (public):
/// [`build_plan`] with that card's twists, plus its escalation when it
/// carries one. Empty when no card is face up (the deck has run out). Read
/// on the table as it stands now, so anything that changes before the
/// adversary phase (a corruption) is not foreseen, and nor are ongoing
/// projects, which can place ambassadors during the build itself.
pub fn next_build(w: &World, g: &GameState) -> Vec<(usize, i64)> {
    let Some(card) = g.pile(adversary::NEXT).first().map(|id| adversary::card(w, id)) else {
        return Vec::new();
    };
    let twists: Vec<&Op> = card.twists.iter().collect();
    let mut plan = build_plan(w, g, &twists, None);
    if card.escalation {
        for (h, n) in escalation_hexes(w, g) {
            match plan.binary_search_by_key(&h, |&(x, _)| x) {
                Ok(i) => plan[i].1 += n,
                Err(i) => plan.insert(i, (h, n)),
            }
        }
    }
    plan
}
