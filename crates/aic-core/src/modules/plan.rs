//! Read-only forecasts of rules steps: what a step would do on the current
//! table, without doing it. Not a module: no phases, no state. Everything here
//! reads public information only.
//!
//! The adversary's creation lines and its Corrupt X step live here (138; 015's
//! deferred call 9: the agents' heuristic used to copy the build):
//! `adversary::create` places what [`create_plan`] says and
//! `adversary::corrupt_sites` corrupts what [`corrupt_plan`] says, so the
//! forecasts in [`next_rally`] and [`next_corrupt_n`] can't drift from the
//! rule (151: `adversary::rally` places what [`rally_plan`] says). The troops' forecast is `board::troop_plan`.

use crate::content::{Op, RoundFlag};
use crate::module::World;
use crate::op::{CorruptSite, CreateAt, CreateIf, RallyAt, Step, Where};
use crate::state::GameState;

use super::{adversary, board};
use super::common::{
    corrupted, cube_owner, firewalled, has_building, is_city, is_ocean, next_to, node_owner, player_ambassador, ADV_AMBASSADOR,
    LAND_CORRUPTION, BOTNET, FACTORY, STATION, prohibit_token,
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
/// land: an empty land hex, no city, base, building or land corruption; 175: a node: a hex with a player's node).
pub fn corrupt_target(w: &World, g: &GameState, h: usize, site: CorruptSite) -> bool {
    match site {
        CorruptSite::City => is_city(w, h) && !corrupted(g, h),
        CorruptSite::CoastalCity => is_city(w, h) && w.map.hexes[h].coastal && !corrupted(g, h),
        CorruptSite::Station => g.tokens(h, STATION) > 0 && !corrupted(g, h),
        CorruptSite::Factory => g.tokens(h, FACTORY) > 0 && !corrupted(g, h),
        CorruptSite::Land => {
            !is_ocean(w, h) && w.map.hexes[h].kind == HexKind::Open && !has_building(g, h) && g.tokens(h, LAND_CORRUPTION) == 0
        }
        CorruptSite::Node => node_owner(g, h).is_some(),
    }
}

/// What a `site`'s corruption counts on its own hex: its agents (r80, ruling #42: a marker no longer counts as an
/// agent on a city, 034-r79).
pub fn held(_w: &World, _g: &GameState, h: usize, counts: &[i64], _site: CorruptSite) -> i64 {
    counts[h]
}

/// 175, a Corrupt X line's convergence: the targets in scope (hex order, country by country), the ones a
/// prohibition switches off (they take no agents and don't corrupt), and each hex whose agents move: one not
/// itself a target, with the open targets next to it (any country; not a firewalled or prohibited one). An
/// agent with one such target moves there; with several, players choose.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Converge {
    /// Every target in scope, in order (the blocked ones included).
    pub targets: Vec<usize>,
    /// Targets a Corrupt prohibition holds, with its condition.
    pub blocked: Vec<(usize, Where)>,
    /// `(hex, open targets next to it)`, in hex order; only hexes holding agents.
    pub sources: Vec<(usize, Vec<usize>)>,
}

/// 175: what a Corrupt `site` line in country `scope` or every country moves (see [`Converge`]).
pub fn converge(w: &World, g: &GameState, site: CorruptSite, scope: Option<usize>) -> Converge {
    let countries: Vec<usize> = match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    };
    let mut out = Converge::default();
    let mut is_target = vec![false; g.hexes.len()];
    let mut open = vec![false; g.hexes.len()];
    for c in countries {
        let mut ts: Vec<usize> =
            w.map.countries[c].hexes.iter().copied().filter(|&h| corrupt_target(w, g, h, site)).collect();
        ts.sort_by_key(|&h| w.map.hexes[h].number);
        for t in ts {
            is_target[t] = true;
            match prohibition_on(w, g, Step::Corrupt, t, g.round as i64) {
                Some(wh) => out.blocked.push((t, wh)),
                // 158: a firewall keeps arrivals off its hex; 178: nor does the site corrupt (`adversary::corrupt`).
                None => open[t] = !firewalled(g, t),
            }
            out.targets.push(t);
        }
    }
    for (h, &target) in is_target.iter().enumerate() {
        // An agent on a site of the type stays on it.
        if target || g.tokens(h, ADV_AMBASSADOR) == 0 {
            continue;
        }
        let mut to: Vec<usize> = w.map.hexes[h].neighbors.iter().copied().filter(|&x| open[x]).collect();
        if to.is_empty() {
            continue;
        }
        to.sort_by_key(|&x| (w.map.hexes[x].country, w.map.hexes[x].number));
        out.sources.push((h, to));
    }
    out
}

/// 175: the players' pick for one converging agent, as the forecasts assume it (the engine asks): a target
/// already at its number if one is open (it corrupts anyway), else the one holding fewest, then the first.
#[allow(clippy::too_many_arguments)]
pub fn converge_pick(w: &World, g: &GameState, site: CorruptSite, to: &[usize], counts: &[i64], amb: &[i64], n: i64) -> usize {
    // What a target will hold after one more arrival is read as its agents, less the ambassadors still to cancel.
    let after = |t: usize| held(w, g, t, counts, site) - amb[t];
    if let Some(i) = to.iter().position(|&t| held(w, g, t, counts, site) >= n) {
        return i;
    }
    (0..to.len()).min_by_key(|&i| after(to[i])).unwrap_or(0)
}

/// 175, what a Corrupt `site` line at number `n` does in country `scope` or every country, the players
/// choosing by [`converge_pick`]: `(target, agents it holds)` for every target that corrupts, in order. One
/// pick at a time, single-target agents first, so a choice sees where the forced ones went.
pub fn corrupt_plan(w: &World, g: &GameState, site: CorruptSite, n: i64, scope: Option<usize>) -> Vec<(usize, i64)> {
    let cv = converge(w, g, site, scope);
    let mut counts: Vec<i64> = (0..g.hexes.len()).map(|h| g.tokens(h, ADV_AMBASSADOR)).collect();
    let mut amb: Vec<i64> =
        (0..g.hexes.len()).map(|h| (0..g.seats.len()).map(|s| g.tokens(h, super::common::ambassador(s))).sum()).collect();
    let mut flexible = Vec::new();
    for (h, to) in &cv.sources {
        if to.len() == 1 {
            for _ in 0..counts[*h] {
                arrive_count(&mut counts, &mut amb, *h, to[0]);
            }
        } else {
            flexible.push((*h, to.clone()));
        }
    }
    for (h, to) in flexible {
        for _ in 0..counts[h] {
            let i = converge_pick(w, g, site, &to, &counts, &amb, n);
            arrive_count(&mut counts, &mut amb, h, to[i]);
        }
    }
    cv.targets
        .iter()
        .copied()
        .filter(|t| !cv.blocked.iter().any(|(b, _)| b == t))
        .filter(|&t| held(w, g, t, &counts, site) >= n)
        .map(|t| (t, counts[t]))
        .collect()
}

/// One agent moves from `from` onto `to` in a forecast: it cancels a player ambassador there if one stands.
fn arrive_count(counts: &mut [i64], amb: &mut [i64], from: usize, to: usize) {
    counts[from] -= 1;
    if amb[to] > 0 {
        amb[to] -= 1;
    } else {
        counts[to] += 1;
    }
}

/// Natural corruption's targets (138): every `site` already holding `n` on
/// its own hex, in hex order (land never corrupts naturally; 175: a node hex does).
pub fn natural_targets(w: &World, g: &GameState, site: CorruptSite, n: i64) -> Vec<usize> {
    if site == CorruptSite::Land {
        return Vec::new();
    }
    let counts: Vec<i64> = (0..g.hexes.len()).map(|h| g.tokens(h, ADV_AMBASSADOR)).collect();
    (0..g.hexes.len()).filter(|&h| corrupt_target(w, g, h, site) && held(w, g, h, &counts, site) >= n).collect()
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

/// 175: where a Rally lands in one board: `(site, agents)` for every site of the type, in hex order. A blocked
/// one's share (a firewall, a prohibition) stays in the supply when the Rally runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RallyPlan {
    pub shares: Vec<(usize, i64)>,
}

/// Whether hex `h` is a site of Rally type `at`.
pub fn rally_fits(w: &World, g: &GameState, h: usize, at: RallyAt) -> bool {
    match at {
        RallyAt::City => is_city(w, h),
        RallyAt::CoastalCity => is_city(w, h) && w.map.hexes[h].coastal,
        RallyAt::Station => g.tokens(h, STATION) > 0,
        RallyAt::Factory => g.tokens(h, FACTORY) > 0,
        RallyAt::Botnet => g.tokens(h, BOTNET) > 0,
    }
}

/// 175, Rally X's sites in board `c`: every site of type `at`, whoever owns it (no tiers), firewalled ones
/// included (their share stays in the supply); in hex order. Empty: none of the type.
pub fn rally_sites(w: &World, g: &GameState, at: RallyAt, c: usize) -> Vec<usize> {
    let mut all: Vec<usize> = w.map.countries[c].hexes.iter().copied().filter(|&h| rally_fits(w, g, h, at)).collect();
    all.sort_by_key(|&h| w.map.hexes[h].number);
    all
}

/// r80 (ruling #43), `n` agents rallying to `at` in board `c`: dealt round-robin, one agent onto each site of
/// the type in hex order, skipping firewalled ones (and, since #53, those under a Rally prohibition's token),
/// until the pool is empty. None: no site of the type; every site skipped: no shares (the pool goes back to
/// the supply).
pub fn rally_plan(w: &World, g: &GameState, at: RallyAt, c: usize, n: i64) -> Option<RallyPlan> {
    let sites = rally_sites(w, g, at, c);
    if sites.is_empty() || n <= 0 {
        return None;
    }
    let open: Vec<usize> = sites.into_iter().filter(|&h| !firewalled(g, h) && !prohibited_now(g, Step::Rally, h)).collect();
    if open.is_empty() {
        return Some(RallyPlan { shares: Vec::new() });
    }
    let k = open.len() as i64;
    let shares = open.iter().enumerate().map(|(i, &h)| (h, n / k + i64::from((i as i64) < n % k))).collect();
    Some(RallyPlan { shares })
}

/// 151, the agents' forecast of the coming Rally (public: the card ahead and the pools): the expected
/// ambassadors landing on each hex (r80: the round-robin deal, exact). Before this round's build the card is
/// this round's; after it, the face-up one with the pools income has filled. Empty when nothing is coming
/// (no card, empty pools, between this round's rally and the income).
pub fn next_rally(w: &World, g: &GameState) -> Vec<(usize, f64)> {
    let build_ahead = g.config.round.iter().position(|p| p == "adversary.build").is_some_and(|i| g.phase <= i);
    let id = match g.pile(adversary::NEXT).first() {
        None if build_ahead => g.pile(adversary::CURRENT).first(),
        next => next,
    };
    let Some(site) = id.and_then(|id| adversary::rally_site(adversary::card(w, id))) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for c in 0..w.map.countries.len() {
        // A cordon counts down at cleanup and comes off at the end of the adversary turn (178): before this
        // round's build it blocks this round's Rally; after it, only one played this round (still at 2) lasts to
        // next round's.
        let cordon = g.var(&board::flag(RoundFlag::Cordon, c));
        if cordon >= if build_ahead { 1 } else { 2 } {
            continue;
        }
        let n = g.var(&adversary::pool_key(c));
        if let Some(p) = rally_plan(w, g, site, c, n) {
            // 175: a blocked site's share stays in the supply (a prohibition ahead; a firewall standing now, which
            // comes off after the turn it was placed before).
            let round = if build_ahead { g.round as i64 } else { g.round as i64 + 1 };
            out.extend(
                p.shares
                    .iter()
                    .filter(|&&(h, k)| k > 0 && !prohibited_in(w, g, Step::Rally, h, round) && !firewalled(g, h))
                    .map(|&(h, k)| (h, k as f64)),
            );
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
        Where::Owned => cube_owner(g, h).is_some(),
    }
}

/// 167: the hexes where `wh` holds, in board order.
pub fn sites_where(w: &World, g: &GameState, wh: Where) -> Vec<usize> {
    (0..g.hexes.len()).filter(|&h| !is_ocean(w, h) && holds(w, g, h, wh)).collect()
}

/// 167: a prohibition's state: the round of the adversary turn it acts in, and the conditions (a bit per
/// [`Where::ALL`] index). Static keys: [`prohibited_in`] runs inside the agents' forecasts.
pub fn prohibit_keys(step: Step) -> (&'static str, &'static str) {
    match step {
        Step::Rally => ("prohibit.rally.round", "prohibit.rally.where"),
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
/// where the condition held when the card resolved; the condition is only its label). `round` is kept for the
/// forecasts' call shape: a token standing now acts in the next adversary turn, whichever round that is.
pub fn prohibition_on(_w: &World, g: &GameState, step: Step, h: usize, _round: i64) -> Option<Where> {
    if !prohibited_now(g, step, h) {
        return None;
    }
    let mask = g.var(prohibit_keys(step).1);
    Some(Where::ALL.iter().enumerate().find(|(i, _)| mask & (1 << i) != 0).map_or(Where::ALL[0], |(_, wh)| *wh))
}

/// 167: whether a `step` prohibition stops that step on hex `h` in the adversary turn of `round`.
pub fn prohibited_in(w: &World, g: &GameState, step: Step, h: usize, round: i64) -> bool {
    prohibition_on(w, g, step, h, round).is_some()
}

/// 167: the same, in this round's adversary turn (the step running now).
pub fn prohibited(w: &World, g: &GameState, step: Step, h: usize) -> bool {
    prohibited_in(w, g, step, h, g.round as i64)
}

/// The corrupt number of the coming corrupt step's lines for `site`: the
/// lowest any Corrupt line on the card ahead prints, else natural
/// corruption's (the agents' pressure threshold).
pub fn next_corrupt_n(w: &World, g: &GameState, site: CorruptSite) -> (i64, bool) {
    let lines: Vec<i64> = card_ahead(w, g)
        .into_iter()
        .flat_map(|c| c.twists.iter())
        .filter_map(|t| match t {
            // 175: a coastal-city line is read as a city line (the forecast doesn't split cities by coast).
            Op::Corrupt { site: s, cut } if *s == site || (site == CorruptSite::City && *s == CorruptSite::CoastalCity) => {
                Some(adversary::corrupt_n(w, site, *cut))
            }
            _ => None,
        })
        .collect();
    match lines.iter().min() {
        Some(&n) => (n, true),
        None => (adversary::corrupt_n(w, site, 0), false),
    }
}
