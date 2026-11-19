//! Node: the two tracks, what a placement costs and pays, and losing a node.

use crate::key::KeyBuf;
use crate::content::Uncover;
use crate::op::NodeSpot;

use super::*;

/// What a node placement's hex prompt says (118: the agents' wall turtle matches
/// on it; a rename once turned it off unseen).
pub const NODE_PROMPT: &str = "place a node";

/// Nodes seat `s` has taken off its AI board's track (the
/// starting one included); AI boards read their uncovered spaces from it.
pub fn track_key(seat: usize) -> KeyBuf {
    KeyBuf::new(TRACK).num(seat)
}

/// Prefix of [`track_key`].
pub(super) const TRACK: &str = "board.track.";
/// Prefix of [`track2_key`].
pub(super) const TRACK2: &str = "board.track2.";

pub fn track_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track_key(seat)).max(0) as usize
}

/// Tokens seat `s` has taken off its board's second node track (021).
pub fn track2_key(seat: usize) -> KeyBuf {
    KeyBuf::new(TRACK2).num(seat)
}

pub fn track2_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track2_key(seat)).max(0) as usize
}

/// Where seat `s` may place nodes: a land hex holding no nodes and no
/// Fallout, adjacent to one of its own (with none left, any such
/// hex in its home country).
pub fn node_sites(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    node_sites_within(w, g, seat, 1)
}

/// [`node_sites`] within `range` of one of the seat's nodes (252: a top node route's range 2).
pub fn node_sites_within(w: &World, g: &GameState, seat: usize, range: i64) -> Vec<usize> {
    let mine = node(seat);
    // 138: no node goes on Fallout.
    let free = |h: usize| {
        !is_ocean(w, h) && node_owner(g, h).is_none() && g.tokens(h, FALLOUT) == 0
    };
    if node_count(g, seat) == 0 {
        let c = home_country(w, seat);
        return w.map.countries[c]
            .hexes
            .iter()
            .copied()
            .filter(|&h| free(h))
            .collect();
    }
    let near = |h: usize| {
        w.map.hexes[h].neighbors.iter().any(|&x| {
            g.tokens(x, mine) > 0 || (range > 1 && w.map.hexes[x].neighbors.iter().any(|&y| g.tokens(y, mine) > 0))
        })
    };
    (0..g.hexes.len()).filter(|&h| free(h) && near(h)).collect()
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

/// What a placement pays on a step printing `step` (041, locked: every placement pays its track step
/// through [`place_node`]; r81, friction #16: a card that said "free" pays it too).
fn placing_price(step: i64) -> Price {
    Price { energy: step, ..Price::default() }
}

/// What a placement pays, for its prompt: ", costs 3 energy", or nothing on a space that prints no
/// cost (044: every prompt printed the bare step cost, whatever was charged).
fn price_words(step: i64) -> String {
    let parts = placing_price(step).words();
    if parts.is_empty() {
        String::new()
    } else {
        format!(", costs {}", parts.join(" + "))
    }
}

/// Whether seat `s` has a token and can pay its cheapest step, with
/// `gained` more energy than it holds (less, when the power placing it pays
/// energy first).
pub fn can_pay_placement(w: &World, g: &GameState, seat: usize, gained: i64) -> bool {
    cheapest_step(w, g, seat).is_some_and(|step| {
        g.resource(seat, Resource::Energy) + gained >= placing_price(step).energy
    })
}

/// 276: whether seat `s` has `n` tokens and can pay the `n` cheapest next steps (each from whichever track's next
/// step is cheaper), with `gained` more energy than it holds: a two-node Growth option is offered only when both
/// its nodes can go down (it was paid in full for one).
pub fn can_pay_placements(w: &World, g: &GameState, seat: usize, n: i64, gained: i64) -> bool {
    let b = ai::board(w, seat);
    let (mut t1, mut t2) = (track_taken(g, seat), track2_taken(g, seat));
    let mut cost = 0;
    for _ in 0..n {
        let first = b.economy_track.get(t1).map(|u| u.cost);
        let second = b.play_track.get(t2).map(|u| u.cost);
        match (first, second) {
            (Some(a), Some(c)) if c < a => {
                cost += c;
                t2 += 1;
            }
            (Some(a), _) => {
                cost += a;
                t1 += 1;
            }
            (None, Some(c)) => {
                cost += c;
                t2 += 1;
            }
            (None, None) => return false,
        }
    }
    g.resource(seat, Resource::Energy) + gained >= cost
}

/// Places one node for `seat`: the next token off its track (uncovering
/// the space under it) onto a hex next to its node, paying its step; only
/// tracks whose next step the seat can pay are offered. Returns the hex it went on (212: a rider's "it").
pub fn place_node(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res<Option<usize>> {
    place_node_within(g, cx, seat, 1)
}

/// [`place_node`] within `range` of one of the seat's nodes (252).
pub fn place_node_within(g: &mut GameState, cx: &mut Cx, seat: usize, range: i64) -> Res<Option<usize>> {
    let w = cx.world;
    if !ai::can_place(&w, g, seat) {
        return Ok(None);
    }
    let hexes = node_sites_within(&w, g, seat, range);
    // 261: no free hex, no question about which track's token would go there.
    if hexes.is_empty() {
        cx.note(None, || format!("Seat {seat}'s node has no free hex to go on: nothing placed"));
        return Ok(None);
    }
    // 021: with two tracks the player chooses which one's next token to take.
    let b = ai::board(&w, seat);
    let (t1, t2) = (track_taken(g, seat), track2_taken(g, seat));
    let can_pay = |u: &Uncover| placing_price(u.cost).affordable(g, seat);
    let mut tracks: Vec<(bool, &Uncover)> = Vec::new();
    if let Some(u) = b.economy_track.get(t1).filter(|u| can_pay(u)) {
        tracks.push((false, u));
    }
    if let Some(u) = b.play_track.get(t2).filter(|u| can_pay(u)) {
        tracks.push((true, u));
    }
    let gain2 = ai::insight_gain(b, t2);
    // 261: the tracks by the names the board prints, and a space that only costs says so (it read "uncovers ,"):
    // the options in their record-key words, the hex prompt in people's.
    let nothing = |l: String| if l.is_empty() { "nothing".to_string() } else { l };
    let what = |two: bool, u: &Uncover| nothing(ai::uncover_label(u, if two { gain2 } else { 0 }));
    let words = |two: bool, u: &Uncover| nothing(ai::uncover_words(u, if two { gain2 } else { 0 }));
    let track_name = |two: bool| if two { "card-play track" } else { "economy track" };
    let opts = tracks
        .iter()
        .map(|(two, u)| opt(format!("{}: uncovers {}{}", track_name(*two), what(*two, u), price_words(u.cost))))
        .collect();
    let prompt = || format!("Seat {seat}: take a node off which track?");
    let Some(k) = cx.choose(seat, prompt, opts)? else {
        return Ok(None);
    };
    let second = tracks[k].0;
    let step = tracks[k].1.cost;
    let capstone = tracks[k].1.capstone.is_some();
    let uncovers = tracks[k].1.effects.as_slice();
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || {
        let what = format!("uncovers {}{}", words(second, tracks[k].1), price_words(step));
        format!("Seat {seat}: {NODE_PROMPT} ({what}) on which hex?")
    };
    let Some(j) = cx.choose(seat, prompt, opts)? else {
        return Ok(None);
    };
    placing_price(step).pay(g, cx, seat);
    if second {
        cx.add_key(g, &track2_key(seat), 1);
    } else {
        cx.add_key(g, &track_key(seat), 1);
    }
    cx.add_tokens(g, hexes[j], node(seat), 1);
    cx.tally(g, "board.node_placed", 1);
    cx.per_round(g, "board.node_placed", 1);
    // 253: by seat, for nodes placed by round per board (stats only).
    cx.per_round(g, &format!("board.node_placed.s{seat}"), 1);
    tally_placement(&w, g, cx, seat, hexes[j]);
    if capstone {
        cx.tally(g, "board.capstones", 1);
        cx.per_round(g, "board.node_capstones", 1);
    }
    effects::run(g, cx, uncovers, &mut effects::Ctx::seat(seat))?;
    Ok(Some(hexes[j]))
}

/// 212: whether hex `x` holds a `spot` (a city, a research station or a factory, anyone's or none's).
pub fn is_spot(w: &World, g: &GameState, x: usize, spot: NodeSpot) -> bool {
    match spot {
        NodeSpot::City => is_city(w, x),
        NodeSpot::Station => g.tokens(x, STATION) > 0,
        NodeSpot::Factory => g.tokens(x, FACTORY) > 0,
    }
}

/// 212: the hexes on or next to `h` holding one of `spots` (a node power's rider: "if it is on or next to");
/// with `fresh`, only those no other node of `seat` (on a hex but `h`) is on or next to.
pub fn spots_near(w: &World, g: &GameState, seat: usize, h: usize, spots: &[NodeSpot], fresh: bool) -> Vec<usize> {
    let around = |x: usize| std::iter::once(x).chain(w.map.hexes[x].neighbors.iter().copied());
    let mine = node(seat);
    around(h)
        .filter(|&x| spots.iter().any(|&s| is_spot(w, g, x, s)))
        .filter(|&x| !fresh || !around(x).any(|y| y != h && g.tokens(y, mine) > 0))
        .collect()
}

/// 210 (stats only): where seat `seat`'s placement on hex `h` landed: `board.node_at.<seat>.<kind>` for each of a
/// factory, a city and a research station on the hex or next to it (`own_<kind>`: one holding the seat's cube; 261:
/// a city the seat hosts, as no city holds one since 254),
/// `none` for none of the three, and `n` for every
/// placement (the placement-spread measure; the spread at the game's end is the farm's `margin.node_spread`).
fn tally_placement(w: &World, g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let near = |f: &dyn Fn(usize) -> bool| f(h) || w.map.hexes[h].neighbors.iter().any(|&x| f(x));
    let own = |x: usize| g.tokens(x, cube(seat)) > 0;
    // 261: no city is owned since 254 (`cities_unowned`): "own city" is one the seat hosts after the placement.
    let hosted = hosted_cities(w, g, seat);
    let own_city = |x: usize| own(x) || hosted.contains(&x);
    let kinds = [
        ("factory", near(&|x| g.tokens(x, FACTORY) > 0)),
        ("city", near(&|x| is_city(w, x))),
        ("station", near(&|x| g.tokens(x, STATION) > 0)),
        ("own_factory", near(&|x| g.tokens(x, FACTORY) > 0 && own(x))),
        ("own_city", near(&|x| is_city(w, x) && own_city(x))),
        ("own_station", near(&|x| g.tokens(x, STATION) > 0 && own(x))),
    ];
    let mut any = false;
    for (k, hit) in kinds {
        if hit {
            any |= !k.starts_with("own_");
            cx.tally(g, &format!("board.node_at.{seat}.{k}"), 1);
        }
    }
    if !any {
        cx.tally(g, &format!("board.node_at.{seat}.none"), 1);
    }
    cx.tally(g, &format!("board.node_at.{seat}.n"), 1);
}

/// Seat `s` loses its nodes on hex `h` (175: a last node too: no stage protects it).
/// It leaves the game, as in Spirit Island (034: what it uncovered stays
/// uncovered), and leaves no mark (r98, Matt's ruling: losing it is punishment
/// enough). The tally names `source`. Returns whether it went.
pub fn lose_node(g: &mut GameState, cx: &mut Cx, h: usize, s: usize, source: &str) -> bool {
    if node_count(g, s) <= 1 {
        cx.tally(g, "board.last_node_lost", 1);
    }
    cx.add_tokens(g, h, node(s), -1);
    // 041: a shield on the hex goes with the token it shielded.
    if !hardenable(g, h) {
        let shields = g.tokens(h, HARDEN);
        cx.add_tokens(g, h, HARDEN, -shields);
    }
    cx.tally(g, "board.node_lost", 1);
    cx.tally(g, &KeyBuf::new("board.node_lost.").text(source), 1);
    true
}
