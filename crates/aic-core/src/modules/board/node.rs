//! Node: the two tracks, what a placement costs and pays, and losing a node.

use crate::module::Hot;
use crate::key::KeyBuf;
use crate::content::Uncover;

use super::*;

/// What a node placement's hex prompt says (118: the agents' wall turtle matches
/// on it; a rename once turned it off unseen).
pub const NODE_PROMPT: &str = "place a node";

/// Nodes seat `s` has taken off its AI board's track (the
/// starting one included); AI boards read their uncovered spaces from it.
pub fn track_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.track.").num(seat)
}

pub fn track_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track_key(seat)).max(0) as usize
}

/// Tokens seat `s` has taken off its board's second node track (021).
pub fn track2_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.track2.").num(seat)
}

pub fn track2_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track2_key(seat)).max(0) as usize
}

/// Where seat `s` may place nodes: a land hex holding no nodes and no
/// corruption marker, adjacent to one of its own (with none left, any such
/// hex in its home country).
pub fn node_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    let mine = node(seat);
    let free = |h: usize| !is_ocean(w, h) && node_owner(g, h).is_none() && g.tokens(h, MARKER) == 0;
    if node_count(g, seat) == 0 {
        let c = home_country(w, seat);
        return w.map.countries[c]
            .hexes
            .iter()
            .copied()
            .filter(|&h| free(h))
            .collect();
    }
    (0..g.hexes.len())
        .filter(|&h| {
            free(h)
                && w.map.hexes[h]
                    .neighbors
                    .iter()
                    .any(|&x| g.tokens(x, mine) > 0)
        })
        .collect()
}

/// The city a seat starts bound to (066): the clean city of country `c` nearest
/// hex `start`, ties to the lower number. Its energy is the seat's first source.
pub(super) fn bound_city(w: &World, g: &GameState, c: usize, start: usize) -> Option<usize> {
    w.content.map.countries[c]
        .cities
        .iter()
        .map(|&n| (w.map.distance(start, w.map.numbered(c, n)), n))
        .filter(|&(_, n)| clean_city(w, g, w.map.numbered(c, n)))
        .min()
        .map(|(_, n)| w.map.numbered(c, n))
}

/// The cheapest next step's printed cost (034: printed on the track, each
/// track's last steps cost more), or `None` with both tracks empty.
fn cheapest_step(w: &World, g: &GameState, seat: usize) -> Option<i64> {
    let b = ai::board(w, seat);
    let first = b.economy_track.get(track_taken(g, seat)).map(|u| u.cost);
    let second = b.play_track.get(track2_taken(g, seat)).map(|u| u.cost);
    first.into_iter().chain(second).min()
}

/// How a node is paid for (041, locked: every placement pays its track
/// step through [`place_node`] unless a card says free).
#[derive(Clone, Copy, PartialEq)]
pub enum Placing {
    /// A node power or a card: the step's cost in energy.
    Step,
    /// A card that says "free".
    Free,
}

/// What a placement pays on a step printing `step`.
fn placing_price(how: Placing, step: i64) -> Price {
    match how {
        Placing::Step => Price { energy: step, ..Price::default() },
        Placing::Free => Price::default(),
    }
}

/// What a placement pays, for its prompt: ", costs 3 energy", or nothing if
/// free (044: every prompt printed the bare step cost, whatever was charged).
fn price_words(how: Placing, step: i64) -> String {
    let parts = placing_price(how, step).words();
    if parts.is_empty() {
        String::new()
    } else {
        format!(", costs {}", parts.join(" + "))
    }
}

/// Whether seat `s` has a token and can pay its cheapest step `how`, with
/// `gained` more energy than it holds (less, when the power placing it pays
/// energy first).
pub fn can_pay_placement(w: &World, g: &GameState, seat: usize, how: Placing, gained: i64) -> bool {
    cheapest_step(w, g, seat).is_some_and(|step| {
        g.resource(seat, Resource::Energy) + gained >= placing_price(how, step).energy
    })
}

/// Places one node for `seat`: the next token off its track (uncovering
/// the space under it) onto a hex next to its node, paying `how`; only
/// tracks whose next step the seat can pay are offered. Returns whether placed.
pub fn place_node(g: &mut GameState, cx: &mut Cx, seat: usize, how: Placing) -> Res<bool> {
    let w = cx.world;
    if !ai::can_place(&w, g, seat) {
        return Ok(false);
    }
    let hexes = node_hexes(&w, g, seat);
    // 021: with two tracks the player chooses which one's next token to take.
    let b = ai::board(&w, seat);
    let (t1, t2) = (track_taken(g, seat), track2_taken(g, seat));
    let can_pay = |u: &Uncover| placing_price(how, u.cost).affordable(g, seat);
    let mut tracks: Vec<(bool, &Uncover)> = Vec::new();
    if let Some(u) = b.economy_track.get(t1).filter(|u| can_pay(u)) {
        tracks.push((false, u));
    }
    if let Some(u) = b.play_track.get(t2).filter(|u| can_pay(u)) {
        tracks.push((true, u));
    }
    let opts = tracks
        .iter()
        .map(|(two, u)| {
            opt(format!(
                "track {}: uncovers {}{}",
                if *two { 2 } else { 1 },
                ai::uncover_label(u),
                price_words(how, u.cost)
            ))
        })
        .collect();
    let prompt = || format!("Seat {seat}: take a node off which track?");
    let Some(k) = cx.choose(seat, prompt, opts)? else {
        return Ok(false);
    };
    let second = tracks[k].0;
    let step = tracks[k].1.cost;
    let capstone = tracks[k].1.capstone.is_some();
    let uncovers = tracks[k].1.effects.as_slice();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || {
        let what = format!("uncovers {}{}", ai::uncover_words(tracks[k].1), price_words(how, step));
        format!("Seat {seat}: {NODE_PROMPT} ({what}) on which hex?")
    };
    let Some(j) = cx.choose(seat, prompt, opts)? else {
        return Ok(false);
    };
    placing_price(how, step).pay(g, cx, seat);
    if second {
        cx.add_key(g, &track2_key(seat), 1);
    } else {
        cx.add_key(g, &track_key(seat), 1);
    }
    cx.add_tokens(g, hexes[j], node(seat), 1);
    cx.tally(g, "board.node_placed", 1);
    cx.per_round(g, "board.node_placed", 1);
    if capstone {
        cx.tally(g, "board.capstones", 1);
        cx.per_round(g, "board.node_capstones", 1);
    }
    effects::run(g, cx, uncovers, &mut effects::Ctx::seat(seat))?;
    Ok(true)
}

/// Whether seat `s`'s node is safe as its last (027): while this round's
/// adversary card is below `last_safe_stage` (stage II), nothing destroys a
/// player's last node. Public for the agents' forecasts (092's shelter read).
pub fn last_node_safe(w: &World, g: &GameState, s: usize) -> bool {
    g.var(adversary::STAGE) < w.hot(Hot::LastSafeStage) && node_count(g, s) <= 1
}

/// Seat `s` loses its nodes on hex `h`, unless it is safe as their last.
/// It leaves the game, as in Spirit Island (034: what it uncovered stays
/// uncovered), and leaves a corruption marker on the hex unless the hex has
/// corruption already. The tally names `source`. Returns whether it went.
pub fn lose_node(g: &mut GameState, cx: &mut Cx, h: usize, s: usize, source: &str) -> bool {
    let w = cx.world;
    if last_node_safe(&w, g, s) {
        cx.tally(g, "board.last_node_safe", 1);
        return false;
    }
    cx.add_tokens(g, h, node(s), -1);
    // 041: a shield on the hex goes with the token it shielded.
    if !hardenable(g, h) {
        let shields = g.tokens(h, HARDEN);
        cx.add_tokens(g, h, HARDEN, -shields);
    }
    let tainted = g.tokens(h, MARKER) > 0 || g.tokens(h, BOTNET) > 0 || corrupted(g, h);
    if w.int("board.node_marker") > 0 && !tainted {
        cx.add_tokens(g, h, MARKER, 1);
        cx.tally(g, "board.markers", 1);
    }
    cx.tally(g, "board.node_lost", 1);
    cx.tally(g, &KeyBuf::new("board.node_lost.").text(source), 1);
    true
}
