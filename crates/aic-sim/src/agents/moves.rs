//! 166: the sim players' move heuristics for Gather and Push (`effects::moves`), one sentence each:
//!
//! - **push**: push agents away from the next Corrupt site (off it, or off its neighbors when the line ahead
//!   gathers from next door), onto empty land with no clean site next to it;
//! - **cancel**: gather (or, with no empty land to push to, push) agents onto a hex where a spare ambassador
//!   stands (no armor, no Liberate it pays toward), so they cancel, as `effects::moves::arrive` resolves it;
//! - **liberate**: gather a seat's own ambassadors that hold nothing onto an adversary site, when that seat, in
//!   reach, can then pay its Liberate (r75: only the acting seat's ambassadors on the site pay);
//! - **strike** (178): answer every strike batch prompt (`effects::STRIKE_PIECE`): the largest batch of troops,
//!   then of agents, never a stop.
//!
//! Each answers only its own prompts; anything else (a strike's hex, a Gather that strikes) is left to the
//! agent's search. The `move_rules` dial turns them off (`Forced`), for the on/off read.

use aic_core::content::{Op, Piece};
use aic_core::op::CorruptSite;
use std::sync::atomic::{AtomicU64, Ordering};
use aic_core::modules::{board, cards, common, effects, plan};
use aic_core::module::World;
use aic_core::{Action, GameState};

/// Which heuristic answered: the names [`report`] prints, indexed as [`pick`] returns them.
pub const NAMES: [&str; 4] = ["push", "cancel", "liberate", "strike"];

/// How often each heuristic answered a prompt: one process-wide count, pooled across every cell and game the
/// process played and including the answers given inside planner rollouts, not a per-game tally (a run prints
/// it divided by its game count: fires a game).
static FIRES: [AtomicU64; 4] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];

/// Counts one answer by heuristic `which`.
pub fn fired(which: usize) {
    FIRES[which].fetch_add(1, Ordering::Relaxed);
}

/// "push N, cancel N, liberate N, strike N": `FIRES` over `games` games (per game, rollouts' answers
/// included), or None if none fired.
pub fn report(games: usize) -> Option<String> {
    let n: Vec<u64> = FIRES.iter().map(|f| f.load(Ordering::Relaxed)).collect();
    (n.iter().sum::<u64>() > 0).then(|| {
        let per = |k: u64| k as f64 / games.max(1) as f64;
        let each: Vec<String> = NAMES.iter().zip(&n).map(|(name, &k)| format!("{name} {:.2}", per(k))).collect();
        format!("move heuristics a game: {}", each.join(", "))
    })
}

/// The option a move heuristic picks for the open prompt, and which heuristic (an index into [`NAMES`]).
/// `last_node` (177, the eval's `w.last_node` on): the push never lands agents where a last node can tip.
pub fn pick(w: &World, g: &GameState, legal: &[Action], last_node: bool) -> Option<(usize, usize)> {
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
    if prompt.contains(effects::STRIKE_PIECE) {
        return strike_piece(legal);
    }
    if prompt.contains(effects::PUSH_FROM) {
        return push_from(w, g, legal);
    }
    if prompt.contains(effects::PUSH_PIECE) {
        let from = named_hex(w, prompt, " (from ")?;
        return push_piece(w, g, legal, from, last_node);
    }
    None
}

/// 175: the open prompt is a Corrupt X line's converging agent ("Corrupt cities (at 4): the agent at .. moves
/// onto which?", `adversary::corrupt_sites`): the option [`plan::converge_pick`] takes, against the number the
/// prompt prints (the resolving line's, cut included). `last_node` (177): unless a target corrupts anyway, not
/// onto a target this agent tips while it holds a seat's last node, if another will do.
pub fn converge(w: &World, g: &GameState, legal: &[Action], last_node: bool) -> Option<usize> {
    let p = g.pending.as_ref()?;
    let (site, n) = aic_core::modules::adversary::read_converge_prompt(&p.ask.prompt)?;
    let to: Vec<usize> = legal.iter().map(|a| option_hex(w, a)).collect::<Option<Vec<_>>>()?;
    let counts: Vec<i64> = (0..g.hexes.len()).map(|h| g.tokens(h, common::ADV_AMBASSADOR)).collect();
    let amb: Vec<i64> =
        (0..g.hexes.len()).map(|h| (0..g.seats.len()).map(|s| g.tokens(h, common::ambassador(s))).sum()).collect();
    let first = plan::converge_pick(w, g, site, &to, &counts, &amb, n);
    let held = |t: usize| plan::held(w, g, t, &counts, site);
    if !last_node || to.iter().any(|&t| held(t) >= n) {
        return Some(first);
    }
    // As `plan::converge_pick` reads it: one more arrival, less the ambassadors still to cancel, reaches `n`.
    let tips = |t: usize| held(t) - amb[t] + 1 >= n && last_node_at(w, g, t);
    if !tips(to[first]) {
        return Some(first);
    }
    (0..to.len()).filter(|&i| !tips(to[i])).min_by_key(|&i| held(to[i]) - amb[to[i]]).or(Some(first))
}

/// 177: hex `h` holds a seat's only node, which a corruption there would destroy (`adversary::corrupt_hex`,
/// `board::destroy_piece`): not sheltered, and the seat has no Sanctuary (`CorruptionShield`).
fn last_node_at(w: &World, g: &GameState, h: usize) -> bool {
    let Some(s) = common::node_owner(g, h) else { return false };
    common::node_count(g, s) == 1
        && !board::flagged(g, aic_core::content::RoundFlag::Shelter, w.map.hexes[h].country as usize)
        && !cards::has_passive(w, g, s, |p| matches!(p, Op::CorruptionShield))
}

/// 177: pushing `n` agents from `from` onto `d` lets a Corrupt nodes line gather a last node's hex next to `d` up
/// to its number (`adversary.corrupt_node_at`): its agents and those next door (not on another node hex, which
/// stay: `plan::converge`), after the move.
fn tips_last_node(w: &World, g: &GameState, from: usize, d: usize, n: i64) -> bool {
    let at = aic_core::modules::adversary::corrupt_n(w, CorruptSite::Node, 0);
    let moves = |y: usize| common::node_owner(g, y).is_none();
    w.map.hexes[d].neighbors.iter().any(|&x| {
        if !last_node_at(w, g, x) || common::firewalled(g, x) {
            return false;
        }
        let next = &w.map.hexes[x].neighbors;
        let near: i64 = next.iter().filter(|&&y| moves(y)).map(|&y| agents(g, y)).sum();
        let gone = if next.contains(&from) && moves(from) { n } else { 0 };
        agents(g, x) + near - gone + n >= at
    })
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

/// The player ambassadors standing on `h`, any seat's (what an agent arriving there cancels).
fn standing(g: &GameState, h: usize) -> i64 {
    (0..g.seats.len()).map(|s| g.tokens(h, common::ambassador(s))).sum()
}

fn agents(g: &GameState, h: usize) -> i64 {
    g.tokens(h, common::ADV_AMBASSADOR)
}

fn agents_next_to(w: &World, g: &GameState, h: usize) -> i64 {
    w.map.hexes[h].neighbors.iter().map(|&x| agents(g, x)).sum()
}

/// r75: what seat `s`'s Liberate on `h` still needs beyond its own ambassadors standing there and its supply
/// (`None`: not an adversary site, or not in its reach). Only the acting seat's own ambassadors on the site pay.
fn seat_short(w: &World, g: &GameState, h: usize, s: usize) -> Option<i64> {
    if !common::in_reach(w, g, s, h) {
        return None;
    }
    let held = g.tokens(h, common::ambassador(s)) + g.resource(s, aic_core::content::Resource::Ambassadors);
    board::liberate_cost(w, g, h).map(|n| (n - held).max(0))
}

/// An ambassador on `x` is armor or a holding there (a node or a player's site): a gather leaves it.
fn holds(g: &GameState, x: usize) -> bool {
    (0..g.seats.len()).any(|s| g.tokens(x, common::node(s)) > 0 || g.tokens(x, common::cube(s)) > 0)
}

/// Seat `s`'s ambassadors next to `h` a gather may take without leaving armor or a holding.
fn free_own_next_to(w: &World, g: &GameState, h: usize, s: usize) -> i64 {
    w.map.hexes[h].neighbors.iter().filter(|&&x| !holds(g, x)).map(|&x| g.tokens(x, common::ambassador(s))).sum()
}

/// r75: the seat a gather onto adversary site `h` lets pay its Liberate now, and how many of its ambassadors it
/// needs gathered: a seat in reach short of the bill whose free ambassadors next door cover it (the smallest
/// shortfall first, then the lowest seat).
fn liberator(w: &World, g: &GameState, h: usize) -> Option<(usize, i64)> {
    (0..g.seats.len())
        .filter_map(|s| seat_short(w, g, h, s).map(|short| (s, short)))
        .filter(|&(s, short)| short > 0 && free_own_next_to(w, g, h, s) >= short)
        .min_by_key(|&(s, short)| (short, s))
}

/// An ambassador stands on `h` that holds nothing: no node or player's site there, no site whose Claim or Liberate
/// it pays toward (r75: your ambassadors on a site pay its bill first).
fn spare(w: &World, g: &GameState, h: usize) -> bool {
    standing(g, h) > 0 && !holds(g, h) && board::liberate_cost(w, g, h).is_none() && board::claim_cost(w, g, h).is_none()
}

/// A hex an agent may be pushed to without harm: land with no site, and no clean site next to it.
fn safe(w: &World, g: &GameState, d: usize) -> bool {
    // 175: a player's node hex corrupts at 2 agents: never a landing spot.
    board::site_kind(w, g, d).is_none()
        && common::node_owner(g, d).is_none()
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
            if let Some((_, lib)) = liberator(w, g, h) {
                if lib > score.0 {
                    score = (lib, 2);
                }
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

/// 178: a strike's next batch (Matt's ruling #27 lets the player choose what comes off on or next to the hex):
/// the largest batch of troops, then of agents, never a stop. It is the order the engine fixed before r77, so a
/// strike costs one search answer (its hex) as it did.
fn strike_piece(legal: &[Action]) -> Option<(usize, usize)> {
    let best = |word: &str| {
        (0..legal.len())
            .filter_map(|i| batch(&legal[i]).filter(|(_, w)| w.starts_with(word)).map(|(n, _)| (n, std::cmp::Reverse(i))))
            .max()
            .map(|(_, std::cmp::Reverse(i))| i)
    };
    best("troop").or_else(|| best("agent")).map(|i| (i, 3))
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
    if let Some((s, short)) = liberator(w, g, to) {
        // Only the paying seat's own ambassadors count toward its bill (r75).
        let mine = format!("seat {s}'s ambassador");
        let free = |a: &Action| {
            batch(a).is_some_and(|_| option_label(a).contains(&mine)) && option_hex(w, a).is_some_and(|x| !holds(g, x))
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
    let (node_n, node_gathers) = plan::next_corrupt_n(w, g, CorruptSite::Node);
    let mut out = Vec::new();
    for h in 0..g.hexes.len() {
        // 175: a player's node hex corrupts at its own number (2), whatever site it holds.
        if common::node_owner(g, h).is_some() && !common::firewalled(g, h) {
            // 184: an agent on a neighboring node hex stays on its own target (plan::converge), as tips_last_node reads.
            let near: i64 = w.map.hexes[h].neighbors.iter().filter(|&&x| common::node_owner(g, x).is_none()).map(|&x| agents(g, x)).sum();
            let count = agents(g, h) + if node_gathers { near } else { 0 };
            if count > 0 && count + 1 >= node_n {
                out.push((h, count, node_n, node_gathers));
            }
        }
        let Some(kind) = board::site_kind(w, g, h) else { continue };
        if common::corrupted(g, h) || common::firewalled(g, h) {
            continue;
        }
        let Some(&(_, (n, gathers))) = lines.iter().find(|(k, _)| *k == kind) else { continue };
        // 175: agents on a clean site of the same kind next door stay there (the line moves only the others).
        let next: i64 = w.map.hexes[h]
            .neighbors
            .iter()
            .filter(|&&x| !(board::site_kind(w, g, x) == Some(kind) && !common::corrupted(g, x)))
            .map(|&x| agents(g, x))
            .sum();
        let count = agents(g, h) + if gathers { next } else { 0 };
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
fn push_piece(w: &World, g: &GameState, legal: &[Action], from: usize, last_node: bool) -> Option<(usize, usize)> {
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
        // 177: nor next to a last node they would bring to its number.
        let gain = if safe(w, g, d)
            && pressure(w, &risk, d) == 0.0
            && standing(g, d) == 0
            && !(last_node && tips_last_node(w, g, from, d, n as i64))
        {
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
