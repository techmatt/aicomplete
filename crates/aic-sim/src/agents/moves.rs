//! 166: the sim players' move heuristics for Gather and Push (`effects::moves`), one sentence each:
//!
//! - **push**: push agents away from the next Corrupt site (off it, or off its neighbors when the line ahead
//!   gathers from next door), onto empty land with no clean site next to it;
//! - **cancel**: gather (or, with no empty land to push to, push) agents onto a hex where a spare ambassador
//!   stands (no armor, no Liberate it pays toward), so they cancel, as `effects::moves::arrive` resolves it;
//! - **liberate**: gather ambassadors that hold nothing onto an adversary site, when a seat in reach can then pay
//!   its Liberate.
//!
//! Each answers only its own prompts; anything else (a strike's target, troops) is left to the agent's search.
//! The `move_rules` dial turns them off (`Forced`), for the on/off read.

use aic_core::content::{Op, Piece};
use aic_core::op::CorruptSite;
use std::sync::atomic::{AtomicU64, Ordering};
use aic_core::modules::{board, cards, common, effects, plan};
use aic_core::module::World;
use aic_core::{Action, GameState};

/// Which heuristic answered (the per-game tally: `agents.move.<name>`).
pub const NAMES: [&str; 3] = ["push", "cancel", "liberate"];

/// How often each heuristic answered a prompt, over every game this process played (a run prints it
/// beside its game count: fires a game).
static FIRES: [AtomicU64; 3] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];

/// Counts one answer by heuristic `which`.
pub fn fired(which: usize) {
    FIRES[which].fetch_add(1, Ordering::Relaxed);
}

/// "push N, cancel N, liberate N" over `games` games (per game), or None if none fired.
pub fn report(games: usize) -> Option<String> {
    let n: Vec<u64> = FIRES.iter().map(|f| f.load(Ordering::Relaxed)).collect();
    (n.iter().sum::<u64>() > 0).then(|| {
        let per = |k: u64| k as f64 / games.max(1) as f64;
        let each: Vec<String> = NAMES.iter().zip(&n).map(|(name, &k)| format!("{name} {:.2}", per(k))).collect();
        format!("move heuristics a game: {}", each.join(", "))
    })
}

/// The option a move heuristic picks for the open prompt, and which heuristic (an index into [`NAMES`]).
pub fn pick(w: &World, g: &GameState, legal: &[Action]) -> Option<(usize, usize)> {
    let p = g.pending.as_ref()?;
    let prompt = p.ask.prompt.as_str();
    let gather = gather_op(w, g);
    if prompt.contains(effects::GATHER_INTO) {
        let (pieces, strike) = gather?;
        if strike {
            return None;
        }
        return gather_into(w, g, legal, &pieces);
    }
    if prompt.contains(effects::GATHER_PIECE) {
        if gather.is_some_and(|(_, strike)| strike) {
            return None;
        }
        let to = named_hex(w, prompt, " (into ")?;
        return gather_piece(w, g, legal, to);
    }
    if prompt.contains(effects::PUSH_FROM) {
        return push_from(w, g, legal);
    }
    if prompt.contains(effects::PUSH_PIECE) {
        let from = named_hex(w, prompt, " (from ")?;
        return push_piece(w, g, legal, from);
    }
    None
}

/// The Gather the open resolution plays: its pieces and strike, read from the card played or used.
fn gather_op(w: &World, g: &GameState) -> Option<(Vec<Piece>, bool)> {
    let p = g.pending.as_ref()?;
    let card = match &p.trigger {
        aic_core::state::Trigger::Action { action: Action::Play { card, .. } | Action::Use { card, .. } } => card,
        _ => return None,
    };
    cards::def(w, card).all_ops().into_iter().find_map(|o| match o {
        Op::Gather { pieces, strike, .. } => Some((pieces.clone(), *strike)),
        _ => None,
    })
}

/// The hex a prompt names after `marker` ("gather which piece? (into Ardena 3)").
fn named_hex(w: &World, prompt: &str, marker: &str) -> Option<usize> {
    let at = prompt.find(marker)? + marker.len();
    let name = prompt[at..].strip_suffix(')')?;
    (0..w.map.hexes.len()).find(|&h| common::hex_name(w, h) == name)
}

fn option_hex(w: &World, a: &Action) -> Option<usize> {
    match a {
        Action::Choose { target: Some(t), .. } => w.map.find(*t),
        _ => None,
    }
}

fn option_label(a: &Action) -> &str {
    match a {
        Action::Choose { option, .. } => option.as_str(),
        _ => "",
    }
}

fn stop(legal: &[Action]) -> Option<usize> {
    legal.iter().position(|a| option_label(a) == effects::STOP)
}

fn standing(g: &GameState, h: usize) -> i64 {
    board::liberate_standing(g, h)
}

fn agents(g: &GameState, h: usize) -> i64 {
    g.tokens(h, common::ADV_AMBASSADOR)
}

fn agents_next_to(w: &World, g: &GameState, h: usize) -> i64 {
    w.map.hexes[h].neighbors.iter().map(|&x| agents(g, x)).sum()
}

/// What a Liberate on `h` still needs beyond the ambassadors standing there (0: not an adversary site).
fn liberate_short(w: &World, g: &GameState, h: usize) -> i64 {
    board::liberate_cost(w, g, h).map_or(0, |n| (n - standing(g, h)).max(0))
}

/// An ambassador on `x` is armor or a holding there (a node or a player's site): a gather leaves it.
fn holds(g: &GameState, x: usize) -> bool {
    (0..g.seats.len()).any(|s| g.tokens(x, common::node(s)) > 0 || g.tokens(x, common::cube(s)) > 0)
}

/// The ambassadors next to `h` a gather may take without leaving armor or a holding.
fn free_ambassadors_next_to(w: &World, g: &GameState, h: usize) -> i64 {
    w.map.hexes[h].neighbors.iter().filter(|&&x| !holds(g, x)).map(|&x| standing(g, x)).sum()
}

/// Whether gathering `more` ambassadors onto adversary site `h` lets a seat in reach pay its Liberate now.
fn payable(w: &World, g: &GameState, h: usize, more: i64) -> bool {
    let best = (0..g.seats.len())
        .filter(|&s| common::in_reach(w, g, s, h))
        .map(|s| g.resource(s, aic_core::content::Resource::Ambassadors))
        .max();
    best.is_some_and(|b| b + more >= liberate_short(w, g, h))
}

/// An ambassador stands on `h` that holds nothing: no node or player's site there, no adversary site it pays toward.
fn spare(w: &World, g: &GameState, h: usize) -> bool {
    standing(g, h) > 0 && !holds(g, h) && board::liberate_cost(w, g, h).is_none()
}

/// A hex an agent may be pushed to without harm: land with no site, and no clean site next to it.
fn safe(w: &World, g: &GameState, d: usize) -> bool {
    board::site_kind(w, g, d).is_none()
        && !w.map.hexes[d].neighbors.iter().any(|&x| board::site_kind(w, g, x).is_some() && !common::corrupted(g, x))
}

/// The gather's target: the hex where gathering cancels the most agents, or brings the most ambassadors
/// toward a Liberate bill.
fn gather_into(w: &World, g: &GameState, legal: &[Action], pieces: &[Piece]) -> Option<(usize, usize)> {
    let mut best: Option<(i64, usize, usize)> = None;
    for (i, a) in legal.iter().enumerate() {
        let Some(h) = option_hex(w, a) else { continue };
        let mut score = (0, 0);
        // Only an ambassador that holds nothing (no armor, no Liberate it pays toward) is traded for agents.
        if pieces.contains(&Piece::Agents) && spare(w, g, h) {
            score = (agents_next_to(w, g, h).min(standing(g, h)), 1);
        }
        if pieces.contains(&Piece::Ambassadors) {
            let lib = free_ambassadors_next_to(w, g, h).min(liberate_short(w, g, h));
            if lib > score.0 && payable(w, g, h, lib) {
                score = (lib, 2);
            }
        }
        if score.0 > 0 && best.is_none_or(|b| score.0 > b.0) {
            best = Some((score.0, i, score.1));
        }
    }
    best.map(|(_, i, which)| (i, which))
}

/// A batch option's size and piece word: "2 agents from X" is (2, "agents").
fn batch(a: &Action) -> Option<(i64, &str)> {
    let mut words = option_label(a).split(' ');
    let n = words.next()?.parse().ok()?;
    let word = words.next()?;
    Some((n, word))
}

fn is_agents(a: &Action) -> bool {
    batch(a).is_some_and(|(_, w)| w.starts_with("agent"))
}

/// The gather's next batch into `to`: agents while a spare ambassador stands there to cancel them, ambassadors
/// that hold nothing while the site's Liberate bill is short; stop before stacking agents on a clean site.
fn gather_piece(w: &World, g: &GameState, legal: &[Action], to: usize) -> Option<(usize, usize)> {
    // The largest batch of `ok` options no bigger than `cap` (the batch of 1 when every batch is bigger).
    let largest = |ok: &dyn Fn(&Action) -> bool, cap: i64| {
        (0..legal.len())
            .filter(|&i| ok(&legal[i]))
            .filter_map(|i| batch(&legal[i]).map(|(n, _)| (n.min(cap) - (n > cap) as i64 * 100, std::cmp::Reverse(i))))
            .max()
            .map(|(_, std::cmp::Reverse(i))| i)
    };
    if spare(w, g, to) {
        if let Some(i) = largest(&is_agents, standing(g, to)) {
            return Some((i, 1));
        }
    }
    let short = liberate_short(w, g, to);
    if short > 0 && payable(w, g, to, free_ambassadors_next_to(w, g, to)) {
        let free = |a: &Action| {
            batch(a).is_some_and(|_| option_label(a).contains("ambassador")) && option_hex(w, a).is_some_and(|x| !holds(g, x))
        };
        if let Some(i) = largest(&free, short) {
            return Some((i, 2));
        }
    }
    // No more agents onto a site once nothing there cancels them (they would count toward its Corrupt line);
    // anything else is the search's.
    if legal.iter().any(is_agents) && board::site_kind(w, g, to).is_some() && !common::corrupted(g, to) {
        return stop(legal).map(|i| (i, 1));
    }
    None
}

/// How close each uncorrupted site is to the next Corrupt line of its type: (site, agents it counts, its number).
fn at_risk(w: &World, g: &GameState) -> Vec<(usize, i64, i64, bool)> {
    let lines = [
        (board::SiteKind::City, plan::next_corrupt_n(w, g, CorruptSite::City)),
        (board::SiteKind::Station, plan::next_corrupt_n(w, g, CorruptSite::Station)),
        (board::SiteKind::Factory, plan::next_corrupt_n(w, g, CorruptSite::Factory)),
    ];
    let mut out = Vec::new();
    for h in 0..g.hexes.len() {
        let Some(kind) = board::site_kind(w, g, h) else { continue };
        if common::corrupted(g, h) || common::firewalled(g, h) {
            continue;
        }
        let Some(&(_, (n, gathers))) = lines.iter().find(|(k, _)| *k == kind) else { continue };
        let count = agents(g, h) + if gathers { agents_next_to(w, g, h) } else { 0 };
        if count > 0 && count + 2 >= n {
            out.push((h, count, n, gathers));
        }
    }
    out
}

/// The pressure a hex's agents put on the sites at risk: each site counts its own agents (and its
/// neighbors' when the line gathers), weighted by how near its number it is.
fn pressure(w: &World, risk: &[(usize, i64, i64, bool)], h: usize) -> f64 {
    risk.iter()
        .filter(|&&(s, _, _, gathers)| s == h || (gathers && w.map.hexes[s].neighbors.contains(&h)))
        .map(|&(_, count, n, _)| count as f64 / n.max(1) as f64)
        .sum()
}

/// The push's hex: the one in options whose agents press hardest on a site at risk.
fn push_from(w: &World, g: &GameState, legal: &[Action]) -> Option<(usize, usize)> {
    let risk = at_risk(w, g);
    let mut best: Option<(f64, usize)> = None;
    for (i, a) in legal.iter().enumerate() {
        let Some(h) = option_hex(w, a) else { continue };
        if agents(g, h) == 0 {
            continue;
        }
        let p = pressure(w, &risk, h);
        if p > 0.0 && best.is_none_or(|b| p > b.0) {
            best = Some((p, i));
        }
    }
    best.map(|(_, i)| (i, 0))
}

/// The push's next batch from `from`: the most agents onto empty land away from every clean site; anything else
/// (troops, ambassadors, a move that helps no site) is the search's.
fn push_piece(w: &World, g: &GameState, legal: &[Action], from: usize) -> Option<(usize, usize)> {
    let risk = at_risk(w, g);
    let here = pressure(w, &risk, from);
    let mut best: Option<(f64, usize)> = None;
    for (i, a) in legal.iter().enumerate() {
        if !is_agents(a) {
            continue;
        }
        let Some(d) = option_hex(w, a) else { continue };
        let n = batch(a).map_or(1, |(n, _)| n) as f64;
        // Onto empty land away from every clean site only. Not onto a standing ambassador: canceling spends an
        // ambassador a Liberate or armor needs (166's dev screens: the games the push heuristic lost each had one).
        let gain = if safe(w, g, d) && pressure(w, &risk, d) == 0.0 && standing(g, d) == 0 {
            here * n
        } else {
            continue;
        };
        if gain > 0.0 && best.is_none_or(|b| gain > b.0) {
            best = Some((gain, i));
        }
    }
    if let Some((_, i)) = best {
        return Some((i, 0));
    }
    // Next, the cancel rule (`effects::moves::arrive`: an agent arriving on a standing ambassador cancels both):
    // agents off a site at risk onto a spare ambassador next door, no more than stand there.
    (here > 0.0)
        .then(|| {
            (0..legal.len())
                .filter(|&i| is_agents(&legal[i]))
                .filter_map(|i| {
                    let d = option_hex(w, &legal[i])?;
                    let n = batch(&legal[i])?.0;
                    (spare(w, g, d) && n <= standing(g, d)).then_some((n, std::cmp::Reverse(i)))
                })
                .max()
                .map(|(_, std::cmp::Reverse(i))| (i, 1))
        })
        .flatten()
}
