//! AI boards: which AI each seat plays, its nodes track (read from the
//! board module, which owns nodes), the keep upgrade, income, credit
//! production, starting resources and powers.

use crate::action::Action;
use crate::content::{AiBoard, Innate, Op, Power, Resource, Tag, Uncover};
use crate::key::KeyBuf;
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::{BoardView, CardView, InnateView, TrackMark, TrackStep, TrackView, ViewOut};

use super::{board, cards, effects};

pub static DEF: ModuleDef = ModuleDef {
    id: "ai",
    summary: "AI boards: the node track, powers, income, credit production, starting resources",
    requires: &["board"],
    variants: &[VariantDef {
        name: "boards",
        summary: "Each seat plays an AI board from content/ais.json",
        params: &[
            ParamSpec::list("boards", 0, 99, "AI board (index into content/ais.json) of each seat; the list repeats for more seats"),
            ParamSpec::list("start_credits", 0, 100, "credits each AI board starts with, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board"),
            ParamSpec::int("start_energy", 0, 20, "energy each player starts with"),
            ParamSpec::list("slots", 1, 10, "building slots of each AI board, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board"),
        ],
        phases: &[
            PhaseSpec::decision("power", "use AI-board powers, each once a round: place nodes, or sacrifice played cards' tags (combine with the action phase)"),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// The AI board seat `s` plays.
pub fn board<'w>(w: &World<'w>, seat: usize) -> &'w AiBoard {
    &w.content.ais[board_index(w, seat)]
}

/// Spaces uncovered on seat `s`'s node track (the board module counts
/// the tokens taken off it, the starting one included).
pub fn uncovered(w: &World, g: &GameState, seat: usize) -> usize {
    board::track_taken(g, seat).clamp(1, board(w, seat).economy_track.len())
}

/// Spaces uncovered on seat `s`'s second node track (021; none at setup).
fn uncovered2(w: &World, g: &GameState, seat: usize) -> usize {
    board::track2_taken(g, seat).min(board(w, seat).play_track.len())
}

/// Income from the AI board and its nodes track: `(resource, amount)`
/// pairs (insight sets the round's insight; the rest arrive in upkeep).
#[cfg(test)]
pub fn track_income(w: &World, g: &GameState, seat: usize) -> Vec<(Resource, i64)> {
    let mut out = Vec::new();
    each_track_income(w, g, seat, |r, n| out.push((r, n)));
    out
}

/// r82 (amendment 8615): the insight seat `s`'s card-play track shows: the total on the rightmost uncovered
/// space printing one, else the board's printed start.
pub fn track_insight(w: &World, g: &GameState, seat: usize) -> i64 {
    let b = board(w, seat);
    b.play_track[..uncovered2(w, g, seat)].iter().rev().find_map(|u| u.insight).unwrap_or(b.insight)
}

/// What uncovering card-play track space `i` of board `b` adds to the round's insight (its total less the
/// total shown before it): the increment the agents and the option keys read.
pub fn insight_gain(b: &AiBoard, i: usize) -> i64 {
    match b.play_track.get(i).and_then(|u| u.insight) {
        Some(n) => n - b.play_track[..i].iter().rev().find_map(|u| u.insight).unwrap_or(b.insight),
        None => 0,
    }
}

/// The tests' `track_income` a pair at a time, for a caller that only adds them up.
pub fn each_track_income(w: &World, g: &GameState, seat: usize, mut f: impl FnMut(Resource, i64)) {
    let b = board(w, seat);
    b.income.iter().for_each(|(r, n)| f(*r, *n));
    f(Resource::Insight, track_insight(w, g, seat));
    for u in uncovered_spaces(w, g, seat) {
        u.income.iter().for_each(|(r, n)| f(*r, *n));
    }
}

/// Spaces seat `s` has uncovered on both tracks.
fn uncovered_spaces<'w>(w: &World<'w>, g: &GameState, seat: usize) -> impl Iterator<Item = &'w Uncover> {
    let b = board(w, seat);
    b.economy_track[..uncovered(w, g, seat)].iter().chain(&b.play_track[..uncovered2(w, g, seat)])
}

/// The capstones seat `s` has uncovered (068): a track's last space, so a
/// hot path needn't walk the tracks.
pub fn capstones<'w>(w: &World<'w>, g: &GameState, seat: usize) -> impl Iterator<Item = &'w Innate> {
    let b = board(w, seat);
    let end = |t: &'w [Uncover], up: usize| t.last().filter(|_| up >= t.len()).and_then(|u| u.capstone.as_ref());
    end(&b.economy_track, uncovered(w, g, seat)).into_iter().chain(end(&b.play_track, uncovered2(w, g, seat)))
}

/// The tags printed on seat `s`'s board (068; 150: they count like the tags
/// on its cards, for everything but a sacrifice).
pub fn printed_tags<'w>(w: &World<'w>, seat: usize) -> impl Iterator<Item = &'w Tag> {
    board(w, seat).tags.iter()
}

/// The tags seat `s`'s tracks have revealed (149: they count like the tags on
/// its cards, for everything but a sacrifice).
pub fn revealed_tags<'w>(w: &World<'w>, g: &GameState, seat: usize) -> impl Iterator<Item = &'w Tag> {
    uncovered_spaces(w, g, seat).flat_map(|u| &u.tags)
}

/// Whether seat `s` has uncovered its board's keep upgrade (021: tier-2
/// keeps cost `cards.keep_tier2_upgraded`).
pub fn keep2(w: &World, g: &GameState, seat: usize) -> bool {
    uncovered_spaces(w, g, seat).any(|u| u.keep2)
}

/// The AI board (index into content/ais.json) seat `s` plays.
fn board_index(w: &World, seat: usize) -> usize {
    let list = w.ai_boards;
    list[seat % list.len()] as usize
}

/// `key`'s value for seat `s`'s board: the list is by board (093), one value
/// for every board or one per board in content/ais.json.
fn by_board(w: &World, key: &str, seat: usize) -> i64 {
    let list = w.list(key);
    list[board_index(w, seat) % list.len()]
}

/// Credits seat `s`'s AI board starts with (`ai.start_credits`, 068; by board, 093).
pub fn start_credits(w: &World, seat: usize) -> i64 {
    by_board(w, "ai.start_credits", seat)
}

/// Building slots of seat `s`'s AI board (`ai.slots`, 021; by board, 093).
pub fn slots(w: &World, seat: usize) -> i64 {
    by_board(w, "ai.slots", seat)
}

/// Whether power `k` of seat `s`'s board places nodes.
pub fn power_places(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.effects.iter().any(|e| matches!(e, Op::PlaceNode { .. })))
}

/// Whether power `k` of seat `s`'s board plays a building for free (068).
pub fn power_builds(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.effects.iter().any(|e| matches!(e, Op::FreeBuilding)))
}

/// Whether power `k` of seat `s`'s board buys a research cube (075).
pub fn power_buys_cube(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.effects.iter().any(|e| matches!(e, Op::ProjectCube { .. })))
}

/// Whether power `k` of seat `s`'s board only turns one resource into another
/// (075, a power menu's conversions): it sacrifices nothing and places nothing.
pub fn power_converts(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| {
        // 285: only gains (r131's Fortify I, Patrol I and Lab read as conversions: they target, push or draw).
        p.tags.is_empty() && !p.cost.is_empty() && p.effects.iter().all(|e| matches!(e, Op::Gain { .. }))
    })
}

/// Whether power `k` of seat `s`'s board sacrifices more than one tag: its
/// sacrifice prompts come before what it does (068).
pub fn power_sacrifices_many(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.tags.iter().map(|(_, n)| n).sum::<i64>() > 1)
}

/// 198: a power's name as its counters key it (`ai.power.<board>.<slug>`, `ai.dear.<board>.<slug>.<why>`), so a
/// cut power doesn't relabel the rest across revisions: lowercased, each run of other characters one `_` (Peer
/// Review: `peer_review`). `scripts/powers.py` mirrors it.
pub fn power_slug(name: &str) -> String {
    let mut slug = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.is_empty() && !slug.ends_with('_') {
            slug.push('_');
        }
    }
    slug.trim_end_matches('_').to_string()
}

/// Set once seat `s` has used power `k` this round.
pub(crate) fn power_used(seat: usize, k: usize) -> KeyBuf {
    KeyBuf::new("ai.power_used.").num(seat).text(".").num(k)
}

/// 289 (D129): an "unaffordable" power's status with its numbers, for the view: what it needs, the parts, what the
/// seat holds ("unaffordable: needs 4 energy (1 + 3 for the next track space); you have 2").
fn short_words(w: &World, g: &GameState, seat: usize, k: usize) -> Option<String> {
    let p = board(w, seat).powers.get(k)?;
    let nodes: i64 = p.effects.iter().map(|e| if let Op::PlaceNode { count, .. } = e { *count } else { 0 }).sum();
    // A placing power's energy is its own and its track steps' together (an own cost short alone understated it);
    // every other resource short is named.
    let mut short: Vec<String> = p
        .cost
        .iter()
        .filter(|(r, n)| g.resource(seat, *r) < *n && !(nodes > 0 && *r == Resource::Energy))
        .map(|(r, n)| format!("{} (you have {})", r.amount(*n), g.resource(seat, *r)))
        .collect();
    let own: i64 = p.cost.iter().filter(|(r, _)| *r == Resource::Energy).map(|(_, n)| *n).sum();
    let have = g.resource(seat, Resource::Energy);
    let placing = if nodes > 0 { board::placements_energy(w, g, seat, nodes) } else { None };
    if let Some(steps) = placing.filter(|&steps| have < own + steps) {
        let what = if nodes == 1 { "the next track space".to_string() } else { format!("the next {nodes} track spaces") };
        let parts = if own > 0 { format!("{own} + {steps} for {what}") } else { format!("for {what}") };
        // The lone shortfall keeps the plain form ("needs 4 energy (1 + 3 for ...); you have 2").
        if short.is_empty() {
            return Some(format!("unaffordable: needs {} ({parts}); you have {have}", Resource::Energy.amount(own + steps)));
        }
        short.push(format!("{} ({parts}; you have {have})", Resource::Energy.amount(own + steps)));
    }
    (!short.is_empty()).then(|| format!("unaffordable: needs {}", short.join(", ")))
}

/// Why seat `s` can't use its board's power `k` now, or `None` if it can.
fn power_blocked(w: &World, g: &GameState, seat: usize, k: usize) -> Option<&'static str> {
    let Some(p) = board(w, seat).powers.get(k) else {
        return Some("no such power");
    };
    if g.var(&power_used(seat, k)) > 0 {
        return Some("used");
    }
    // 276: a threshold power counts the tags on your cards (board tags too), sacrificing none.
    if p.needs.iter().any(|(t, n)| cards::tags(w, g, seat, *t) < *n) {
        return Some("needs tags");
    }
    if p.cost.iter().any(|(r, n)| g.resource(seat, *r) < *n) {
        return Some("unaffordable");
    }
    // 066: a power that places pays its energy first, and the step after it.
    let spent: i64 = p.cost.iter().filter(|(r, _)| *r == Resource::Energy).map(|(_, n)| *n).sum();
    // 118: with every node token placed the power read "unaffordable".
    if power_places(w, seat, k) && !can_place(w, g, seat) {
        // 308: "on your tracks": a bare "no node left" read as the node-loss condition beside the map's nodes.
        return Some("no node token left on your tracks");
    }
    // 261: say which price: Expand prints none, so a bare "unaffordable" left a player looking for one.
    if power_places(w, seat, k) && !board::can_pay_placement(w, g, seat, -spent) {
        return Some("unaffordable: the next track space's energy, after the power's own");
    }
    // 276 (diff review): a two-node Growth option needs both its nodes' steps, or it was paid in full for one.
    let nodes: i64 = p.effects.iter().map(|e| if let Op::PlaceNode { count, .. } = e { *count } else { 0 }).sum();
    // 285: with fewer node tokens left than it places, it read "unaffordable" whatever the seat's energy.
    let left = (board(w, seat).economy_track.len() - board::track_taken(g, seat).min(board(w, seat).economy_track.len()))
        + (board(w, seat).play_track.len() - board::track2_taken(g, seat).min(board(w, seat).play_track.len()));
    if nodes > left as i64 {
        return Some("too few node tokens left on your tracks");
    }
    if nodes > 1 && !board::can_pay_placements(w, g, seat, nodes, -spent) {
        return Some("unaffordable: the next track spaces' energy for every node, after the power's own");
    }
    // 261: a node route is used for its node (118), as with no token or no step: with no free hex in its range,
    // Outpost and Field Survey were paid in full for their ambassador or card alone.
    let range = p.effects.iter().find_map(|e| if let Op::PlaceNode { range, .. } = e { Some(*range) } else { None });
    if range.is_some_and(|r| board::node_sites_within(w, g, seat, r).is_empty()) {
        return Some("no target");
    }
    // 286 (D121): a two-node option needs a hex for its second node once the first is placed.
    if nodes > 1 && range.is_some_and(|r| !board::second_node_site(w, g, seat, r)) {
        return Some("no target for the second node");
    }
    if !p.tags.is_empty() && !cards::TagNeed::new(&p.tags).payable(&cards::sources(w, g, seat)) {
        return Some("needs tags");
    }
    if !effects::feasible_after(w, g, seat, &p.effects, spent) {
        return Some("no target");
    }
    // 276: Lab II's cube is what its 2 credits buy (Grant's job): with no open megaproject it isn't offered, though
    // its draw could act.
    if let Some(cube) = p.effects.iter().find(|e| matches!(e, Op::ProjectCube { .. })) {
        if !effects::feasible_after(w, g, seat, std::slice::from_ref(cube), spent) {
            return Some("no target");
        }
    }
    if p.effects.iter().any(|e| matches!(e, Op::FreeBuilding)) && !free_building_after(w, g, seat, p) {
        return Some("no target");
    }
    // 286 (D128): Patrol II's extra energy buys its hardening (the push is Patrol I's): with no facility to harden
    // on or next to any target it isn't offered, though its push could act (the any-part rule above passed it). Only
    // where the hardening is its whole difference from a tier in its group (Fortify II also removes a second troop).
    let unhardened: Vec<&Op> = p.effects.iter().filter(|e| !matches!(e, Op::Harden { .. })).collect();
    let hardening_only = unhardened.len() < p.effects.len()
        && p.group.is_some()
        && board(w, seat).powers.iter().any(|q| q.group == p.group && q.effects.iter().eq(unhardened.iter().copied()));
    if hardening_only {
        let harden: Vec<Op> =
            p.effects.iter().filter(|e| matches!(e, Op::Target { .. } | Op::Harden { .. })).cloned().collect();
        if !effects::feasible_after(w, g, seat, &harden, spent) {
            return Some("no facility to harden");
        }
    }
    // 285: Field Station's insight buys its station (its node is Field Survey's, free): with no open station site in
    // reach it isn't offered, though its node could land (the any-part rule above passed it).
    if p.effects.iter().any(|e| matches!(e, Op::BuildStation)) {
        let station: Vec<Op> = p.effects.iter().filter(|e| !matches!(e, Op::PlaceNode { .. })).cloned().collect();
        if !effects::feasible_after(w, g, seat, &station, spent) {
            return Some("no target");
        }
    }
    None
}

/// 099: whether a building is still free to play once a free-building power
/// (Scrap II; Rush Build before 276) is paid: its resources spent and its sacrifice made, since a
/// building may have needed the tags it sacrifices, or its energy, or the stack
/// it covers (docs/rules/card-grammar.md). Without this the power took its energy and card
/// and played nothing. 276: a sacrifice of two or more sources is tried whole (Scrap II sacrifices 2 Industry).
fn free_building_after(w: &World, g: &GameState, seat: usize, p: &Power) -> bool {
    let mut paid = g.clone();
    for (r, n) in &p.cost {
        *paid.seats[seat].resources.entry(*r).or_insert(0) -= n;
    }
    let need = cards::TagNeed::new(&p.tags);
    if need.met() {
        return cards::free_building_exists(w, &paid, seat);
    }
    let sources = cards::sources(w, &paid, seat);
    sources.iter().filter(|s| need.helps(&s.tags)).any(|s| builds_after(w, &mut paid, seat, &need, s, &sources))
}

/// Whether a building is still free to play once source `s` is sacrificed toward `need` and, when that leaves
/// some of `need` unpaid, another source of `pool` pays the rest the same way (276): tried on `g` itself, its
/// piles put back after (141: a copy of the state for each source was a sixth of a long game's time). `g` leaves
/// as it came.
fn builds_after(
    w: &World,
    g: &mut GameState,
    seat: usize,
    need: &cards::TagNeed,
    s: &cards::SacrificeSource,
    pool: &[cards::SacrificeSource],
) -> bool {
    let mut saved = Vec::with_capacity(s.cards.len());
    for (pile, card) in &s.cards {
        if let Some(cards_in) = g.piles.get_mut(pile.as_str()) {
            saved.push((pile, cards_in.clone()));
            std::sync::Arc::make_mut(cards_in).retain(|c| c != card);
        }
    }
    let mut left = need.clone();
    left.pay(&s.tags);
    let free = if left.met() {
        cards::free_building_exists(w, g, seat)
    } else {
        let rest: Vec<cards::SacrificeSource> = pool.iter().filter(|x| x.label != s.label).cloned().collect();
        rest.iter().filter(|x| left.helps(&x.tags)).any(|x| builds_after(w, g, seat, &left, x, &rest))
    };
    for (pile, old) in saved.into_iter().rev() {
        *g.piles.get_mut(pile.as_str()).expect("a pile just written") = old;
    }
    free
}

/// A dear power: one that sacrifices 2 or more tags (075), or a tag and an
/// energy and places no node (r35's Rush Build, Shelter). 099: the 2-tag test alone matched no power after r35, and
/// the measure went silent.
pub fn power_is_dear(p: &Power) -> bool {
    let tags: i64 = p.tags.iter().map(|(_, n)| n).sum();
    let places = p.effects.iter().any(|e| matches!(e, Op::PlaceNode { .. }));
    tags >= 2 || (tags >= 1 && !places && p.cost.iter().any(|(r, n)| *r == Resource::Energy && *n > 0))
}

/// 075: what seat `s` could still do with its dear powers as the action phase
/// ends, and why it can't: the measure's, so a power near unused has a cause.
pub fn tally_dear_powers(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    let b = board(&w, seat);
    for (k, p) in b.powers.iter().enumerate() {
        if !power_is_dear(p) {
            continue;
        }
        let why = power_blocked(&w, g, seat, k);
        let key = match why {
            None => "idle",
            Some("used") => continue,
            Some("needs tags") => "no_tags",
            Some("no target") | Some("no node token left on your tracks") | Some("too few node tokens left on your tracks") => "no_target",
            Some(_) => "unpaid",
        };
        let slug = power_slug(&p.name);
        cx.tally(g, &format!("ai.dear.{}.{slug}.{key}", b.id), 1);
        // 099: the cost is checked first, so a round short of both read "unpaid"
        // alone: the tags are counted on their own as well.
        if key == "unpaid" && !cards::TagNeed::new(&p.tags).payable(&cards::sources(&w, g, seat)) {
            cx.tally(g, &format!("ai.dear.{}.{slug}.no_tags", b.id), 1);
        }
    }
}

/// 276 (stats only): the board uses seat `s` can make as its action phase begins, a group (a power's tiers)
/// counted once: the yes/no items a player weighs that round (`ai.offered.<board>`).
pub fn tally_offered(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    let b = board(&w, seat);
    let mut seen: Vec<&str> = Vec::new();
    for (k, p) in b.powers.iter().enumerate() {
        let key = p.use_key();
        if !seen.contains(&key) && power_blocked(&w, g, seat, k).is_none() {
            seen.push(key);
        }
    }
    let key = format!("ai.offered.{}", b.id);
    cx.tally(g, &key, seen.len() as i64);
    cx.per_round(g, &key, seen.len() as i64);
}

/// Whether seat `s` has a node left on its track.
pub fn can_place(w: &World, g: &GameState, seat: usize) -> bool {
    board::track_taken(g, seat) < board(w, seat).economy_track.len()
        || board::track2_taken(g, seat) < board(w, seat).play_track.len()
}

/// Whether uncovering `u` builds a research station (075).
pub fn builds_station(u: &Uncover) -> bool {
    u.effects.iter().any(|e| matches!(e, Op::BuildStation))
}

/// 263: what uncovering a space gains once (the capstones' antimatter lump), for its printed words.
pub fn once_gains(u: &Uncover) -> Vec<(Resource, i64)> {
    u.effects.iter().filter_map(|e| if let Op::Gain { resource, amount } = e { Some((*resource, *amount)) } else { None }).collect()
}

/// What uncovering a space gives, in the words of its option (a record key:
/// `+1 credits, T2 keep -1`); `gain`: the insight it adds ([`insight_gain`]; r82 prints the total).
pub fn uncover_label(u: &Uncover, gain: i64) -> String {
    let mut parts: Vec<String> = Vec::new();
    if gain != 0 {
        parts.push(format!("+{}", Resource::Insight.amount(gain)));
    }
    parts.extend(u.income.iter().map(|(r, n)| format!("+{}", r.amount(*n))));
    if u.keep2 {
        parts.push("T2 keep -1".into());
    }
    parts.extend(u.tags.iter().map(|t| format!("{t} tag")));
    if builds_station(u) {
        parts.push("free station".into());
    }
    parts.extend(once_gains(u).into_iter().map(|(r, n)| format!("{} once", r.amount(n))));
    if let Some(c) = &u.capstone {
        parts.push(format!("capstone {}", c.name));
    }
    parts.join(", ")
}

/// The same for people to read, in a prompt (`+1 credit, T2 keep`; the track step's hover has the rest); `gain`
/// as [`uncover_label`] (261: the insight a space adds, as the track question words it, not its new total).
pub fn uncover_words(u: &Uncover, gain: i64) -> String {
    let mut parts: Vec<String> = Vec::new();
    // 265 (D115): a track space raises a level paid every round ("+4 insight" read as a one-off beside the
    // track's "insight 7"); a one-off gain says "once" below.
    if gain != 0 {
        parts.push(format!("+{} a round", Resource::Insight.words(gain)));
    }
    parts.extend(u.income.iter().map(|(r, n)| format!("+{} a round", r.words(*n))));
    if u.keep2 {
        parts.push("T2 keep".into());
    }
    parts.extend(u.tags.iter().map(|t| format!("{t} tag")));
    if builds_station(u) {
        parts.push("free station".into());
    }
    parts.extend(once_gains(u).into_iter().map(|(r, n)| format!("{} once", r.words(n))));
    if let Some(c) = &u.capstone {
        parts.push(format!("capstone: {}", c.name));
    }
    parts.join(", ")
}

#[derive(Debug)]
struct Ai {
    start_energy: i64,
}

/// What a power costs, in words: `pay 1 energy, sacrifice 2 diplomacy`.
fn power_cost(p: &Power) -> String {
    let mut cost = Vec::new();
    if !p.cost.is_empty() {
        let paid: Vec<String> = p.cost.iter().map(|(r, n)| r.words(*n)).collect();
        cost.push(format!("pay {}", paid.join(" + ")));
    }
    if !p.tags.is_empty() {
        let tags: Vec<String> = p.tags.iter().map(|(t, n)| format!("{n} {t}")).collect();
        cost.push(format!("sacrifice {}", tags.join(" + ")));
    }
    cost.join(", ")
}

/// 235: when one source alone can pay a power's tags, the power takes it unasked (a single option): name it, so the
/// button says which card goes (Audit took an 11-credit building just built).
fn lone_sacrifice(w: &World, g: &GameState, s: usize, p: &Power) -> String {
    if p.tags.is_empty() {
        return String::new();
    }
    let need = cards::TagNeed::new(&p.tags);
    let helping: Vec<cards::SacrificeSource> =
        cards::sources(w, g, s).into_iter().filter(|x| need.helps(&x.tags)).collect();
    match helping.as_slice() {
        [only] => match only.cards.last() {
            Some((_, top)) => format!(" Sacrifices: {}.", cards::def(w, top).name),
            None => String::new(),
        },
        _ => String::new(),
    }
}

fn power_text(p: &Power) -> String {
    let cap = |s: &str| {
        let mut first = s.chars();
        first.next().map_or(String::new(), |c| c.to_uppercase().chain(first).collect::<String>())
    };
    let cost = power_cost(p);
    // 276: a group is one use a round between its tiers (r142: a node power is its own use).
    let once = match p.group.as_deref() {
        Some(g) => format!("(once a round, one tier of {g})"),
        None => "(once a round)".to_string(),
    };
    let needs = power_needs(p);
    let lead = if needs.is_empty() { String::new() } else { format!("{needs}, ") };
    // 118: a power with no price (Synthesis) printed ": discard a card...".
    if cost.is_empty() {
        return format!("{lead}{} {once}", if lead.is_empty() { cap(&p.text) } else { p.text.clone() });
    }
    format!("{lead}{}: {} {once}", if lead.is_empty() { cap(&cost) } else { cost }, p.text)
}

/// 276: a threshold power's condition in words: `With 2 Science on your cards`; empty for any other power.
pub fn power_needs(p: &Power) -> String {
    if p.needs.is_empty() {
        return String::new();
    }
    let tags: Vec<String> = p
        .needs
        .iter()
        .map(|(t, n)| {
            let t = t.to_string();
            let mut c = t.chars();
            let t: String = c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default();
            format!("{n} {t}")
        })
        .collect();
    format!("With {} on your cards", tags.join(" and "))
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let boards = b.list("boards")?;
    if let Some(&x) = boards
        .iter()
        .find(|&&x| x as usize >= b.world.content.ais.len())
    {
        return Err(format!(
            "boards: no AI board {x} (content/ais.json has {})",
            b.world.content.ais.len()
        ));
    }
    let n = b.world.content.ais.len();
    for key in ["start_credits", "slots"] {
        let len = b.list(key)?.len();
        if len != 1 && len != n {
            return Err(format!("{key}: one value, or one per AI board ({n}), not {len}"));
        }
    }
    Ok(Box::new(Ai {
        start_energy: b.int("start_energy")?,
    }))
}

impl Module for Ai {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            cx.add_resource(g, s, Resource::Credits, start_credits(&cx.world, s));
            cx.add_resource(g, s, Resource::Energy, self.start_energy);
        }
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for s in 0..g.seats.len() {
            for k in 0..board(&w, s).powers.len() {
                cx.set_key(g, &power_used(s, k), 0);
            }
        }
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        for k in 0..board(w, seat as usize).powers.len() {
            if power_blocked(w, g, seat as usize, k).is_none() {
                out.push(Action::Power { seat, power: k as u8 });
            }
        }
    }

    fn can_pass(&self, _phase: usize, _g: &GameState, _seat: u8) -> bool {
        true
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Power { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let Action::Power { seat, power } = a else {
            return Err(Halt::Illegal("not a power".into()));
        };
        let (s, k) = (*seat as usize, *power as usize);
        // An answer replays this from a table that passed it.
        if !cx.replaying() {
            if let Some(why) = power_blocked(&w, g, s, k) {
                return Err(Halt::Illegal(format!("can't use power {k}: {why}")));
            }
        }
        let p = &board(&w, s).powers[k];
        for (r, n) in &p.cost {
            cx.add_resource(g, s, *r, -n);
        }
        // 276: a tier uses its whole group for the round.
        let b = board(&w, s);
        for (j, q) in b.powers.iter().enumerate() {
            if p.shares_use(q) {
                cx.set_key(g, &power_used(s, j), 1);
            }
        }
        if p.effects.iter().any(|e| matches!(e, Op::FreeBuilding)) {
            // 099: only a sacrifice that leaves a building to play (its energy is paid).
            cards::sacrifice_keeping(g, cx, s, &p.tags, &|st: &mut GameState, need, src| {
                let pool = cards::sources(&w, st, s);
                builds_after(&w, st, s, need, src, &pool)
            })?;
        } else {
            cards::sacrifice_for(g, cx, s, &p.tags)?;
        }
        let key = format!("ai.power.{}.{}", board(&w, s).id, power_slug(&p.name));
        cx.tally(g, &key, 1);
        cx.per_round(g, &key, 1);
        cx.tally(g, "ai.powers_used", 1);
        cx.per_round(g, "ai.powers_used", 1);
        // 105: what a power removes fires the seat's fight triggers.
        cards::fighting(g, cx, s, |g, cx| effects::run(g, cx, &p.effects, &mut effects::Ctx::seat(s).named("power")))
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        // Powers used show as their status on the board below.
        out.bookkeeping(&["ai.power_used."]);
        let track_view = |label: &str, steps: &[Uncover], up: usize| TrackView {
            label: label.into(),
            steps: steps
                .iter()
                .enumerate()
                .map(|(i, u)| {
                    // r82 (amendment 8615): a printed insight total, Spirit Island style.
                    let mut marks: Vec<TrackMark> = u
                        .insight
                        .map(|n| TrackMark {
                            key: Resource::Insight.to_string(),
                            label: format!("insight {n}"),
                            doc: format!("your insight each round is {n} while this is the rightmost total uncovered"),
                        })
                        .into_iter()
                        .collect();
                    marks.extend(u.income.iter().map(|(r, n)| TrackMark {
                        key: r.to_string(),
                        label: format!("+{}", r.words(*n)),
                        doc: format!("+{} every round once uncovered", r.words(*n)),
                    }));
                    if u.keep2 {
                        marks.push(TrackMark {
                            key: "tier2".into(),
                            label: "T2 keep".into(),
                            // 118: the hover printed the config key, not the price.
                            // With the cards module off there is no keep to price.
                            doc: match w.variant("cards") {
                                Some(v) if v != "off" => format!(
                                    "uncovering this lowers a tier-2 keep from {} to {} insight, before Science discounts",
                                    w.int("cards.keep_tier2"),
                                    w.int("cards.keep_tier2_upgraded")
                                ),
                                _ => "uncovering this lowers a tier-2 keep".into(),
                            },
                        });
                    }
                    for t in &u.tags {
                        marks.push(TrackMark {
                            key: t.to_string(),
                            label: format!("{t} tag"),
                            doc: "once uncovered this tag counts like one on your cards (it can't be sacrificed)".into(),
                        });
                    }
                    if builds_station(u) {
                        marks.push(TrackMark {
                            key: "station".into(),
                            label: "free station".into(),
                            doc: "uncovering this builds a research station in reach, paying no chips".into(),
                        });
                    }
                    if let Some(c) = &u.capstone {
                        marks.push(TrackMark {
                            key: "capstone".into(),
                            label: c.name.clone(),
                            doc: format!("capstone: {}", c.text),
                        });
                    }
                    // 044: the step's printed cost reached no client.
                    if u.cost > 0 {
                        marks.push(TrackMark {
                            key: "cost".into(),
                            label: format!("pay {}", Resource::Energy.words(u.cost)),
                            doc: "placing this token costs this much energy, on top of what the power or card placing it costs (every placement pays it)".into(),
                        });
                    }
                    TrackStep {
                        value: i as i64 + 1,
                        marks,
                    }
                })
                .collect(),
            uncovered: up,
            current: up.saturating_sub(1),
        };
        for s in 0..g.seats.len() {
            let b = board(w, s);
            let up = uncovered(w, g, s);
            let mut tracks = vec![track_view("economy track", &b.economy_track, up)];
            if !b.play_track.is_empty() {
                tracks.push(track_view("card-play track", &b.play_track, uncovered2(w, g, s)));
            }
            // The board's powers (021), with their status this round.
            let powers: Vec<CardView> = b
                .powers
                .iter()
                .enumerate()
                .map(|(k, p)| CardView {
                    id: format!("{}.power.{k}", b.id),
                    name: p.name.clone(),
                    text: format!("{}{}", power_text(p), lone_sacrifice(w, g, s, p)),
                    // 285: as the hand's and buildings' (nothing is used while a question is open).
                    // A lasting state (used this round) still shows.
                    status: match power_blocked(w, g, s, k) {
                        // 289 (D129): nothing is ready once the game is over (the end screen read "ready").
                        _ if g.is_over() => "game over".into(),
                        Some("used") => "used".into(),
                        _ if g.pending.is_some() => "waiting on the question".into(),
                        Some(why) if why.starts_with("unaffordable") => short_words(w, g, s, k).unwrap_or_else(|| why.into()),
                        why => why.unwrap_or("ready").into(),
                    },
                    cost: None,
                    // 320: a node power, as the rules read one (the player view grouped them by their text).
                    node: power_places(w, s, k),
                    ..Default::default()
                })
                .collect();
            out.board(
                s,
                "board",
                BoardView {
                    key: String::new(),
                    source: String::new(),
                    id: b.id.clone(),
                    name: b.name.clone(),
                    text: b.text.clone(),
                    innate: Some(InnateView {
                        name: b.innate.name.clone(),
                        text: b.innate.text.clone(),
                    }),
                    tags: b.tags.iter().map(|t| t.to_string()).collect(),
                    tracks,
                    slots: Some(slots(w, s)),
                    cards: powers,
                },
            );
            out.seat(
                s,
                "credit_production",
                "credit production",
                b.credits,
                None,
                "credits this AI board makes every round",
            );
        }
    }
}
