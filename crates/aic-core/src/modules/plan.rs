//! Read-only forecasts of rules steps: what a step would do on the current
//! table, without doing it. Not a module: no phases, no state. Everything here
//! reads public information only.
//!
//! The adversary's creation lines and its Corrupt X step live here (138; 015's
//! deferred call 9: the agents' heuristic used to copy the build):
//! `adversary::create` places what [`create_plan`] says and
//! `adversary::corrupt_sites` corrupts what [`corrupt_plan`] says, so the
//! forecasts in [`next_build`] and [`next_corruptions`] can't drift from the
//! rule. The troops' forecast is `board::troop_plan`.

use crate::content::{Op, RoundFlag};
use crate::module::World;
use crate::op::{CorruptSite, CreateAt, CreateIf};
use crate::state::GameState;

use super::{adversary, board};
use super::common::{
    corrupted, cube_owner, has_building, is_city, is_ocean, ADV_AMBASSADOR, LAND_CORRUPTION, BOTNET, FACTORY, MARKER,
    STATION,
};
use crate::content::HexKind;

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
            w.map.countries[c].hexes.iter().copied().filter(|&h| create_fits(w, g, h, line, near, location)).collect();
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
/// land: an empty land hex, no city, base, building or land corruption).
pub fn corrupt_target(w: &World, g: &GameState, h: usize, site: CorruptSite) -> bool {
    match site {
        CorruptSite::City => is_city(w, h) && !corrupted(g, h),
        CorruptSite::Station => g.tokens(h, STATION) > 0 && !corrupted(g, h),
        CorruptSite::Factory => g.tokens(h, FACTORY) > 0 && !corrupted(g, h),
        CorruptSite::Land => {
            !is_ocean(w, h) && w.map.hexes[h].kind == HexKind::Open && !has_building(g, h) && g.tokens(h, LAND_CORRUPTION) == 0
        }
    }
}

/// What a city's corruption counts on its own hex (034: a marker counts).
fn held(w: &World, g: &GameState, h: usize, counts: &[i64]) -> i64 {
    counts[h] + if is_city(w, h) { g.tokens(h, MARKER) } else { 0 }
}

/// What a Corrupt `site` line at number `n` does in country `scope` or every
/// country (138): `(target, pulls)` in order, each pull `(hex, ambassadors)`
/// returned to the supply. In each country, each target in hex-number order
/// that can gather `n` (what it holds, then its neighbors in the same
/// country, lowest-numbered first; each ambassador moves once) pulls just
/// enough in and corrupts; every ambassador on it goes too. An ambassador
/// pulled is gone for the targets after it.
pub fn corrupt_plan(w: &World, g: &GameState, site: CorruptSite, n: i64, scope: Option<usize>) -> Vec<(usize, Vec<(usize, i64)>)> {
    let mut counts: Vec<i64> = (0..g.hexes.len()).map(|h| g.tokens(h, ADV_AMBASSADOR)).collect();
    let countries: Vec<usize> = match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    };
    let mut out = Vec::new();
    for c in countries {
        let mut targets: Vec<usize> =
            w.map.countries[c].hexes.iter().copied().filter(|&h| corrupt_target(w, g, h, site)).collect();
        targets.sort_by_key(|&h| w.map.hexes[h].number);
        for t in targets {
            let mut from: Vec<usize> = w.map.hexes[t]
                .neighbors
                .iter()
                .copied()
                .filter(|&x| w.map.hexes[x].country as usize == c && counts[x] > 0)
                .collect();
            from.sort_by_key(|&x| w.map.hexes[x].number);
            let own = held(w, g, t, &counts);
            let around: i64 = from.iter().map(|&x| counts[x]).sum();
            if own + around < n {
                continue;
            }
            let mut pulls = Vec::new();
            if counts[t] > 0 {
                pulls.push((t, counts[t]));
            }
            let mut need = n - own;
            for x in from {
                if need <= 0 {
                    break;
                }
                let k = counts[x].min(need);
                pulls.push((x, k));
                counts[x] -= k;
                need -= k;
            }
            counts[t] = 0;
            out.push((t, pulls));
        }
    }
    out
}

/// Natural corruption's targets (138): every `site` already holding `n` on
/// its own hex, in hex order (land never corrupts naturally).
pub fn natural_targets(w: &World, g: &GameState, site: CorruptSite, n: i64) -> Vec<usize> {
    if site == CorruptSite::Land {
        return Vec::new();
    }
    let counts: Vec<i64> = (0..g.hexes.len()).map(|h| g.tokens(h, ADV_AMBASSADOR)).collect();
    (0..g.hexes.len()).filter(|&h| corrupt_target(w, g, h, site) && held(w, g, h, &counts) >= n).collect()
}

/// The card whose build and corrupt steps come next: the face-up one, or
/// this round's until its corrupt step has run (between this round's build
/// and reveal the face-up slot is empty).
fn card_ahead<'w>(w: &World<'w>, g: &GameState) -> Option<&'w crate::content::AdversaryCard> {
    let corrupt_ahead = g.config.round.iter().position(|p| p == "adversary.corrupt").is_some_and(|i| g.phase <= i);
    let id = match g.pile(adversary::NEXT).first() {
        None if corrupt_ahead => g.pile(adversary::CURRENT).first(),
        next => next,
    }?;
    Some(adversary::card(w, id))
}

/// The next round's build, read off the face-up adversary card (public): its
/// creation lines, then the board's (with the difficulty's `build_bonus`).
/// Empty when no card is face up (the deck has run out). Read on the table as
/// it stands now, so ongoing projects, which create during the build too, are
/// not foreseen.
pub fn next_build(w: &World, g: &GameState) -> Vec<(usize, i64)> {
    let Some(card) = g.pile(adversary::NEXT).first().map(|id| adversary::card(w, id)) else {
        return Vec::new();
    };
    let bonus = if adversary::level(w, 3) { w.int("adversary.build_bonus") } else { 0 };
    let mut plan: Vec<(usize, i64)> = Vec::new();
    let mut add = |h: usize, n: i64| match plan.binary_search_by_key(&h, |&(x, _)| x) {
        Ok(i) => plan[i].1 += n,
        Err(i) => plan.insert(i, (h, n)),
    };
    for line in card.standing.iter().filter_map(CreateLine::of) {
        for (h, n) in create_plan(w, g, line, None, &[], None) {
            add(h, n);
        }
    }
    for line in w.content.adversary_board.build.iter().filter_map(CreateLine::of) {
        let line = CreateLine { count: line.count + bonus, ..line };
        for (h, n) in create_plan(w, g, line, None, &[], None) {
            add(h, n);
        }
    }
    plan
}

/// The corrupt number of the coming corrupt step's lines for `site`: the
/// lowest any Corrupt line on the card ahead prints, else natural
/// corruption's (the agents' pressure threshold).
pub fn next_corrupt_n(w: &World, g: &GameState, site: CorruptSite) -> (i64, bool) {
    let lines: Vec<i64> = card_ahead(w, g)
        .into_iter()
        .flat_map(|c| c.twists.iter())
        .filter_map(|t| match t {
            Op::Corrupt { site: s, cut } if *s == site => Some(adversary::corrupt_n(w, site, *cut)),
            _ => None,
        })
        .collect();
    match lines.iter().min() {
        Some(&n) => (n, true),
        None => (adversary::corrupt_n(w, site, 0), false),
    }
}
