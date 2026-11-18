//! 166: the sim players' move heuristics for Gather and Push (`effects::moves`), one sentence each:
//!
//! - **push**: push agents away from the next Corrupt site (off it, or off its neighbors when the line ahead
//!   gathers from next door), onto empty land with no clean site next to it;
//! - **cancel**: gather (or, with no empty land to push to, push) agents onto a hex where a spare ambassador
//!   stands (no armor, no Liberate it pays toward), so they cancel, as `effects::moves::arrive` resolves it;
//! - **liberate**: gather a seat's own ambassadors that hold nothing onto an adversary site, when that seat, in
//!   reach, can then pay its Liberate (r75: only the acting seat's ambassadors on the site pay);
//! - **strike** (178): answer every strike batch prompt (`effects::STRIKE_PIECE`): the largest batch of troops,
//!   then of agents, never a stop;
//! - **target** (234): answer the target prompt (`effects::TARGET_PROMPT`) with the hex where the card's map ops
//!   find the most ([`target`]).
//!
//! Each answers only its own prompts; anything else (a strike's hex, a Gather that strikes) is left to the
//! agent's search. Three agent behaviors (`policy::Behaviors`) switch them, for the on/off reads: `move_rules`
//! the first three ([`pick`]), `strike_rule` the strike ([`strike`], 185: its own lever), `target_rule` the
//! target ([`target`]).

use aic_core::content::{Op, Piece};
use aic_core::op::CorruptSite;
use std::sync::atomic::{AtomicU64, Ordering};
use aic_core::modules::{board, cards, common, effects, plan};
use aic_core::module::World;
use aic_core::{Action, GameState};

/// Which heuristic answered: the names [`report`] prints, indexed as [`pick`] returns them.
pub const NAMES: [&str; 5] = ["push", "cancel", "liberate", "strike", "target"];

/// [`NAMES`]' index of the target rule (234).
const TARGET: usize = 4;

/// How often each heuristic answered a prompt: one process-wide count, pooled across every cell and game the
/// process played and including the answers given inside planner rollouts, not a per-game tally (a run prints
/// it divided by its game count: fires a game).
static FIRES: [AtomicU64; NAMES.len()] = [const { AtomicU64::new(0) }; NAMES.len()];

/// Counts one answer by heuristic `which`.
pub fn fired(which: usize) {
    FIRES[which].fetch_add(1, Ordering::Relaxed);
}

/// "push N, cancel N, liberate N, strike N, target N": `FIRES` over `games` games (per game, rollouts' answers
/// included), or None if none fired.
pub fn report(games: usize) -> Option<String> {
    let n: Vec<u64> = FIRES.iter().map(|f| f.load(Ordering::Relaxed)).collect();
    (n.iter().sum::<u64>() > 0).then(|| {
        let per = |k: u64| k as f64 / games.max(1) as f64;
        let each: Vec<String> = NAMES.iter().zip(&n).map(|(name, &k)| format!("{name} {:.2}", per(k))).collect();
        format!("move heuristics a game: {}", each.join(", "))
    })
}

/// The strike rule's option for the open prompt when it is a strike batch prompt ([`strike_piece`]), and its
/// index into [`NAMES`].
pub fn strike(g: &GameState, legal: &[Action]) -> Option<(usize, usize)> {
    let p = g.pending.as_ref()?;
    if !p.ask.prompt.contains(effects::STRIKE_PIECE) {
        return None;
    }
    strike_piece(legal)
}

/// The option a push, cancel or liberate heuristic picks for the open prompt, and which heuristic (an index into
/// [`NAMES`]).
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
    if prompt.contains(effects::PUSH_FROM) {
        return push_from(w, g, legal);
    }
    if prompt.contains(effects::PUSH_PIECE) {
        let from = named_hex(w, prompt, " (from ")?;
        return push_piece(w, g, legal, from, last_node);
    }
    None
}

/// 234 (Matt, 2026-11-14): the target rule, one sentence: **target the hex where the card's map ops find the
/// most, each op's finds (up to its number, in its area around the hex) priced at its `values.json` row**; a
/// Gather's or Push's target is its move heuristic's hex; with nothing to tell the hexes apart, the search's.
/// Answers only the target prompt (`effects::TARGET_PROMPT`), for the card the open resolution plays or uses, or
/// the seat's building whose trigger it is.
pub fn target(w: &World, g: &GameState, legal: &[Action]) -> Option<(usize, usize)> {
    let p = g.pending.as_ref()?;
    if !p.ask.prompt.contains(effects::TARGET_PROMPT) {
        return None;
    }
    let ops = target_ops(w, g, &p.ask.prompt)?;
    // A move's target is its own heuristic's (its legal options are the target's hexes).
    for o in &ops {
        match o {
            Op::Gather { pieces, strike: false, .. } => return gather_into(w, g, legal, pieces).map(|(i, _)| (i, TARGET)),
            Op::Push { .. } => return push_from(w, g, legal).map(|(i, _)| (i, TARGET)),
            _ => {}
        }
    }
    let t = super::value::table();
    let mut scores: Vec<(f64, usize)> = Vec::new();
    for (i, a) in legal.iter().enumerate() {
        let Some(h) = option_hex(w, a) else { continue };
        // Every Agent and Troop on the target gone first: what comes after it finds the rest.
        let cleared = ops.iter().any(|o| matches!(o, Op::DestroyAll));
        scores.push((ops.iter().map(|o| finds(w, g, t, o, h, cleared)).sum(), i));
    }
    let best = scores.iter().map(|s| s.0).fold(f64::NEG_INFINITY, f64::max);
    // A tie at the top is the search's.
    if !best.is_finite() || best <= 0.0 || scores.iter().filter(|s| s.0 == best).count() > 1 {
        return None;
    }
    scores.iter().find(|s| s.0 == best).map(|s| (s.1, TARGET))
}

/// The ops after the target of the card whose target prompt is open: the building the prompt names (" [id]", an
/// Action's or an Effect's), else the card played or used (its condition, "(words)", must match).
fn target_ops(w: &World, g: &GameState, prompt: &str) -> Option<Vec<Op>> {
    let p = g.pending.as_ref()?;
    let after = |list: &[Op]| -> Option<Vec<Op>> {
        let at = list.iter().position(|o| matches!(o, Op::Target { on } if prompt.contains(&format!("({})", on.words()))))?;
        Some(list[at + 1..].to_vec())
    };
    let lists = |id: &aic_core::Key| -> Vec<Vec<Op>> {
        let d = cards::def(w, id);
        let mut out = vec![d.effects.clone()];
        out.extend(d.ability.iter().flat_map(|a| a.branches()).map(|b| b.effects.clone()));
        for o in &d.passives {
            if let Op::When { then, .. } | Op::Threshold { then, .. } = o {
                out.push(then.clone());
            }
        }
        out
    };
    // 236: a board power's target, and Remove agents' (the first agent on it, up to `remove_count` on or next to it).
    match &p.trigger {
        aic_core::state::Trigger::Action { action: Action::Power { seat, power } } => {
            let b = aic_core::modules::ai::board(w, *seat as usize);
            return b.powers.get(*power as usize).and_then(|pw| after(&pw.effects));
        }
        aic_core::state::Trigger::Action { action: Action::Standard { act, .. } } if act == "remove" => {
            let max = w.int("board.remove_count");
            return Some(vec![Op::RemoveAmbassadorsCountry { max, at: aic_core::content::Area::Near }]);
        }
        // Remove a troop and Build a factory run ops, as a card does.
        aic_core::state::Trigger::Action { action: Action::Standard { act, .. } } => {
            return board::StandardAction::parse(act).and_then(|a| after(board::std_ops(a)));
        }
        _ => {}
    }
    // The building whose Action or Effect resolves names itself (by its printed name, among the seat's cards).
    if let Some(name) = prompt.rsplit_once(" [").and_then(|(_, rest)| rest.strip_suffix(']')) {
        let named = cards::in_play(g, p.ask.seat as usize).into_iter().find(|c| cards::def(w, c).name == name);
        return named.and_then(|c| lists(&c).iter().find_map(|l| after(l)));
    }
    if let aic_core::state::Trigger::Action { action: Action::Play { card, .. } | Action::Use { card, .. } } = &p.trigger {
        return lists(card).iter().find_map(|l| after(l));
    }
    None
}

/// What op `o` finds acting on target `h`, at its row: pieces removed or converted, sites freed, tokens lifted,
/// facilities hardened, each up to its number.
fn finds(w: &World, g: &GameState, t: &super::value::Table, o: &Op, h: usize, cleared: bool) -> f64 {
    use aic_core::content::Area;
    let near = |at: Area| common::area_of(w, h, at);
    let count = |at: Area, f: &dyn Fn(usize) -> i64| -> i64 { near(at).into_iter().map(f).sum() };
    // A removal after "remove every Agent and Troop on it" finds none on the target.
    let gone = |x: usize| cleared && x == h && !matches!(o, Op::DestroyAll);
    let troops = |x: usize| if gone(x) { 0 } else { g.tokens(x, common::TROOP) };
    let agents = |g: &GameState, x: usize| if gone(x) { 0 } else { agents(g, x) };
    match o {
        Op::RemoveAmbassadorsCountry { max, at, .. } => {
            let n = count(*at, &|x| agents(g, x));
            (if *max > 0 { n.min(*max) } else { n }) as f64 * t.op("remove_ambassadors_country")
        }
        Op::RemoveTroops { count: k, at, .. } => count(*at, &troops).min(*k) as f64 * t.op("remove_troop"),
        Op::Damage { ambassadors, troops: k } => {
            let a = agents(g, h).min(*ambassadors) as f64 * t.op("remove_ambassador");
            let b = troops(h).min(*k) as f64 * t.op("remove_troop");
            a.max(b)
        }
        Op::DestroyAll => agents(g, h) as f64 * t.op("remove_ambassador") + troops(h) as f64 * t.op("remove_troop"),
        Op::Gather { max, pieces, strike: true } => {
            let take = |x: usize| {
                (if pieces.contains(&Piece::Troops) { troops(x) } else { 0 }) * 5
                    + if pieces.contains(&Piece::Agents) { agents(g, x) } else { 0 }
            };
            count(Area::Near, &take).min(*max * 5) as f64 * t.op("remove_ambassador")
        }
        Op::Convert { max } => count(Area::Near, &|x| agents(g, x)).min(*max) as f64 * t.op("convert"),
        Op::LiftFallout { count: k, at } => count(*at, &|x| g.tokens(x, common::FALLOUT)).min(*k) as f64 * t.op("lift_fallout"),
        Op::Cleanse { amount, at, cities, only, .. } => {
            count(*at, &|x| i64::from(effects::cleansable(w, g, x, *cities, *only))).min(*amount) as f64 * t.op("cleanse")
        }
        Op::Harden { count: k, at } => count(*at, &|x| i64::from(board::hardenable(g, x))).min(*k) as f64 * t.op("harden"),
        Op::ClearCity { max } => count(Area::Near, &|x| agents(g, x)).min(if *max > 0 { *max } else { i64::MAX }) as f64 * t.op("clear_city"),
        // Liberating: the agents on the site come off with it (261: not those next to it, `liberate_site`), the
        // twist's troops in its country; with `own` (Hostile Buyout) a station or factory becomes the seat's, at
        // `liberate_own`'s row, which hedges for a city the card might take (as read on 48 dev seeds: a seen-site
        // price is an agents item's).
        Op::Liberate { troops: k, own, .. } => {
            let owned = *own && matches!(board::site_kind(w, g, h), Some(board::SiteKind::Station | board::SiteKind::Factory));
            agents(g, h) as f64 * t.op("remove_ambassador") * 0.5
                + count(Area::Country, &troops).min(*k) as f64 * t.op("liberate_troops")
                + if owned { t.op("liberate_own") } else { 0.0 }
        }
        // 261: a liberate card's rider claims a neutral station or factory next to the city it frees (256): seen
        // here, so at its full row (`claim_near` discounts it only where the card is priced unseen).
        Op::ClaimSite { near: true } => {
            if near(Area::Near).into_iter().any(|x| board::card_claimable_near(w, g, x)) { t.op("claim_site") } else { 0.0 }
        }
        Op::RemoveBotnet => count(Area::Near, &troops) as f64 * 0.1,
        _ => 0.0,
    }
}

/// 177: hex `h` holds a seat's only node, which a corruption there would destroy (`adversary::corrupt_hex`,
/// `board::destroy_piece`): not sheltered, and the seat has no Sanctuary (`CorruptionShield`).
fn last_node_at(w: &World, g: &GameState, h: usize) -> bool {
    let Some(s) = common::node_owner(g, h) else { return false };
    common::node_count(g, s) == 1
        && !board::sheltered(w, g, h, Some(s))
        && !cards::has_passive(w, g, s, |p| matches!(p, Op::CorruptionShield))
}

/// 177: pushing `n` agents from `from` onto `d` lets a node line gather a last node's hex next to `d` up
/// to its number (`adversary.corrupt_node_at`): its agents and those next door that converge onto it
/// (`plan::converge_map`: not those on another node hex, nor, r98, those next to two open node hexes), after the move.
fn tips_last_node(w: &World, g: &GameState, from: usize, d: usize, n: i64) -> bool {
    let at = aic_core::modules::adversary::corrupt_n(w, CorruptSite::Node, 0);
    // The push moves no node, so where each hex's agents converge is the same after it; only the counts move.
    let after = |y: usize| agents(g, y) - if y == from { n } else { 0 } + if y == d { n } else { 0 };
    let mut to: Option<Vec<Option<usize>>> = None;
    w.map.hexes[d].neighbors.iter().any(|&x| {
        if !last_node_at(w, g, x) || common::firewalled(g, x) {
            return false;
        }
        let to = to.get_or_insert_with(|| plan::converge_map(w, g, CorruptSite::Node));
        let near: i64 = w.map.hexes[x].neighbors.iter().filter(|&&y| to[y] == Some(x)).map(|&y| after(y)).sum();
        after(x) + near >= at
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
/// (`None`: not an adversary site, or not in its reach, or 248: no Liberate offered). Only the acting seat's own
/// ambassadors on the site pay.
fn seat_short(w: &World, g: &GameState, h: usize, s: usize) -> Option<i64> {
    if !common::in_reach(w, g, s, h) || !board::std_offered(w, board::StandardAction::Liberate) {
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

/// An ambassador stands on `h` that holds nothing: no node or player's site there, no site whose Liberate it pays
/// toward (r75: your ambassadors on a site pay its bill first; 262 cut the Claim action, which they paid too).
fn spare(w: &World, g: &GameState, h: usize) -> bool {
    standing(g, h) > 0 && !holds(g, h) && board::liberate_cost(w, g, h).is_none()
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
        (board::SiteKind::City, CorruptSite::City),
        (board::SiteKind::Station, CorruptSite::Station),
        (board::SiteKind::Factory, CorruptSite::Factory),
    ]
    .map(|(k, site)| (k, site, plan::next_corrupt_n(w, g, site)));
    let (node_n, node_gathers) = plan::next_corrupt_n(w, g, CorruptSite::Node);
    // 237 (229's pick): the agents next door that a gathering line moves onto `h`, as the engine moves them
    // (`plan::converge_map`: not those on a target of their own, nor, r98, those next to two open targets).
    // One map a gathering line, read once.
    let maps: Vec<(CorruptSite, Vec<Option<usize>>)> = lines
        .iter()
        .filter(|(_, _, (_, gathers))| *gathers)
        .map(|&(_, site, _)| site)
        .chain(node_gathers.then_some(CorruptSite::Node))
        .map(|site| (site, plan::converge_map(w, g, site)))
        .collect();
    let gathered = |h: usize, site: CorruptSite| -> i64 {
        let Some((_, to)) = maps.iter().find(|(s, _)| *s == site) else { return 0 };
        w.map.hexes[h].neighbors.iter().filter(|&&x| to[x] == Some(h)).map(|&x| agents(g, x)).sum()
    };
    let mut out = Vec::new();
    for h in 0..g.hexes.len() {
        // 175: a player's node hex corrupts at its own number (2), whatever site it holds.
        if common::node_owner(g, h).is_some() && !common::firewalled(g, h) {
            let count = agents(g, h) + if node_gathers { gathered(h, CorruptSite::Node) } else { 0 };
            if count > 0 && count + 1 >= node_n {
                out.push((h, count, node_n, node_gathers));
            }
        }
        let Some(kind) = board::site_kind(w, g, h) else { continue };
        if common::corrupted(g, h) || common::firewalled(g, h) {
            continue;
        }
        let Some(&(_, site, (n, gathers))) = lines.iter().find(|(k, _, _)| *k == kind) else { continue };
        // 253: a liberated neutral site needs the retake's agents more (the engine's `retake_extra`).
        let n = n + common::retake_extra(w, g, h, site);
        let count = agents(g, h) + if gathers { gathered(h, site) } else { 0 };
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
