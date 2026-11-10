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
    LAND_CORRUPTION, BOTNET, FACTORY, MARKER, STATION,
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
    corrupt_plan_blocked(w, g, site, n, scope).0
}

/// [`corrupt_plan`], and the targets a Corrupt prohibition (167) kept from corrupting: `(target, condition,
/// agents it would have gathered)`. A blocked target pulls nothing, so its neighbors' agents stay for the next.
#[allow(clippy::type_complexity)]
pub fn corrupt_plan_blocked(
    w: &World,
    g: &GameState,
    site: CorruptSite,
    n: i64,
    scope: Option<usize>,
) -> (Vec<(usize, Vec<(usize, i64)>)>, Vec<(usize, Where, i64)>) {
    let mut blocked = Vec::new();
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
            // 158: nothing is pulled onto a firewalled target: it corrupts only on what it holds.
            let mut from: Vec<usize> = w.map.hexes[t]
                .neighbors
                .iter()
                .copied()
                .filter(|&x| !firewalled(g, t) && w.map.hexes[x].country as usize == c && counts[x] > 0)
                .collect();
            from.sort_by_key(|&x| w.map.hexes[x].number);
            let own = held(w, g, t, &counts);
            let around: i64 = from.iter().map(|&x| counts[x]).sum();
            if own + around < n {
                continue;
            }
            if let Some(wh) = prohibition_on(w, g, Step::Corrupt, t, g.round as i64) {
                blocked.push((t, wh, n));
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
    (out, blocked)
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

/// 151: where a Rally lands in one board: every site in `sites` (hex order) gains `each`, and `extra` of them
/// (players choose, one each) one more.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RallyPlan {
    pub sites: Vec<usize>,
    pub each: i64,
    pub extra: i64,
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

/// 151, Rally X's eligible sites in board `c`: its neutral `at` sites (no owner cube), else the players' (an
/// aligned city, a player's station or factory), else the adversary's; in hex order. Empty: none of the type.
pub fn rally_sites(w: &World, g: &GameState, at: RallyAt, c: usize) -> Vec<usize> {
    // 158: a firewalled site is no Rally site this round.
    let mut all: Vec<usize> =
        w.map.countries[c].hexes.iter().copied().filter(|&h| !firewalled(g, h) && rally_fits(w, g, h, at)).collect();
    all.sort_by_key(|&h| w.map.hexes[h].number);
    let neutral = |h: &usize| !corrupted(g, *h) && cube_owner(g, *h).is_none();
    let allied = |h: &usize| cube_owner(g, *h).is_some();
    for class in [&neutral as &dyn Fn(&usize) -> bool, &allied, &|h: &usize| corrupted(g, *h)] {
        let these: Vec<usize> = all.iter().copied().filter(|h| class(h)).collect();
        if !these.is_empty() {
            return these;
        }
    }
    Vec::new()
}

/// 151: `n` ambassadors rallying to `at` in board `c`, spread evenly over its eligible sites. None: no site.
pub fn rally_plan(w: &World, g: &GameState, at: RallyAt, c: usize, n: i64) -> Option<RallyPlan> {
    let sites = rally_sites(w, g, at, c);
    if sites.is_empty() || n <= 0 {
        return None;
    }
    let k = sites.len() as i64;
    Some(RallyPlan { each: n / k, extra: n % k, sites })
}

/// 151, the agents' forecast of the coming Rally (public: the card ahead and the pools): the expected
/// ambassadors landing on each hex, the remainder spread as a share. Before this round's build the card is
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
        // A cordon counts down at cleanup: before this round's build it blocks this round's Rally; after it, only
        // one played this round (still at 2) lasts to next round's.
        let cordon = g.var(&board::flag(RoundFlag::Cordon, c));
        if cordon >= if build_ahead { 1 } else { 2 } {
            continue;
        }
        let n = g.var(&adversary::pool_key(c));
        if let Some(p) = rally_plan(w, g, site, c, n) {
            // 167: a prohibited site's share stays in the supply; the remainder goes to the others.
            let round = if build_ahead { g.round as i64 } else { g.round as i64 + 1 };
            let open: Vec<usize> = p.sites.iter().copied().filter(|&h| !prohibited_in(w, g, Step::Rally, h, round)).collect();
            if open.is_empty() {
                continue;
            }
            let share = p.each as f64 + p.extra.min(open.len() as i64) as f64 / open.len() as f64;
            out.extend(open.iter().map(|&h| (h, share)));
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
        Step::Deploy => ("prohibit.deploy.round", "prohibit.deploy.where"),
        Step::Corrupt => ("prohibit.corrupt.round", "prohibit.corrupt.where"),
    }
}

/// 167: the condition of a `step` prohibition acting in the adversary turn of `round` that holds on `h`.
pub fn prohibition_on(w: &World, g: &GameState, step: Step, h: usize, round: i64) -> Option<Where> {
    let (rk, wk) = prohibit_keys(step);
    if g.var(rk) != round {
        return None;
    }
    let mask = g.var(wk);
    Where::ALL.iter().enumerate().find(|(i, wh)| mask & (1 << i) != 0 && holds(w, g, h, **wh)).map(|(_, wh)| *wh)
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
            Op::Corrupt { site: s, cut } if *s == site => Some(adversary::corrupt_n(w, site, *cut)),
            _ => None,
        })
        .collect();
    match lines.iter().min() {
        Some(&n) => (n, true),
        None => (adversary::corrupt_n(w, site, 0), false),
    }
}
