//! Presence: the two tracks, what a placement costs and pays, and losing a presence.

use super::*;

/// Presence tokens seat `s` has taken off its AI board's track (the
/// starting one included); AI boards read their uncovered spaces from it.
pub fn track_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.track.").num(seat)
}

pub fn track_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track_key(seat)).max(0) as usize
}

/// Tokens seat `s` has taken off its board's second presence track (021).
pub fn track2_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.track2.").num(seat)
}

pub fn track2_taken(g: &GameState, seat: usize) -> usize {
    g.var(&track2_key(seat)).max(0) as usize
}

/// Where seat `s` may place presence: a land hex holding no presence,
/// adjacent to one of its own (with none left, any such hex in its home
/// country).
pub fn presence_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    let mine = presence(seat);
    let free = |h: usize| !is_ocean(w, h) && presence_owner(g, h).is_none() && g.tokens(h, MARKER) == 0;
    if presence_count(g, seat) == 0 {
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
                    .neighbours
                    .iter()
                    .any(|&x| g.tokens(x, &mine) > 0)
        })
        .collect()
}

/// Seat `s`'s uses of the rich action this round (034).
pub(super) fn rich_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.rich.").num(seat)
}

/// Seat `s`'s uses of the standard placement this round (046).
pub fn placed_key(seat: usize) -> KeyBuf {
    KeyBuf::new("board.placed.").num(seat)
}

/// Whether seat `s` may still take the standard placement this round.
pub(super) fn placement_open(w: &World, g: &GameState, seat: usize) -> bool {
    let cap = w.int("board.presence_action_cap");
    cap == 0 || g.var(&placed_key(seat)) < cap
}

/// The cheapest next step's printed cost (034: printed on the track, each
/// track's last steps cost more), or `None` with both tracks empty.
pub(super) fn cheapest_step(w: &World, g: &GameState, seat: usize) -> Option<i64> {
    let b = ai::board(w, seat);
    let first = b.presence.get(track_taken(g, seat)).map(|u| u.cost);
    let second = b.presence2.get(track2_taken(g, seat)).map(|u| u.cost);
    first.into_iter().chain(second).min()
}

/// Who is placing a presence, which sets its price (041: every placement
/// pays its track step through [`place_presence`] unless a card says free).
#[derive(Clone, Copy, PartialEq)]
pub enum Placing {
    /// Growth, and a card: the step's cost in energy.
    Step,
    /// A card that says "free".
    Free,
    /// The standard action: `board.presence_energy` more.
    Action,
    /// The rich action: `board.presence_credits` and its rise, the step at
    /// `board.energy_credits` credits an energy.
    Rich,
}

/// What seat `s` pays to place on a step printing `step`.
pub fn placing_price(w: &World, g: &GameState, seat: usize, how: Placing, step: i64) -> Price {
    match how {
        Placing::Step => Price { energy: step, ..Price::default() },
        Placing::Free => Price::default(),
        Placing::Action => Price { energy: w.int("board.presence_energy") + step, ..Price::default() },
        Placing::Rich => Price {
            credits: w.int("board.presence_credits")
                + g.var(&rich_key(seat)) * w.int("board.presence_rise")
                + step * w.int("board.energy_credits"),
            ..Price::default()
        },
    }
}

/// What a placement pays, for its prompt: ", costs 3 energy", or nothing if
/// free (044: every prompt printed the bare step cost, whatever was charged).
pub(super) fn price_words(w: &World, g: &GameState, seat: usize, how: Placing, step: i64) -> String {
    let parts = placing_price(w, g, seat, how, step).words();
    if parts.is_empty() {
        String::new()
    } else {
        format!(", costs {}", parts.join(" + "))
    }
}

/// Whether seat `s` has a token and can pay its cheapest step `how`, with
/// `gained` more energy than it holds.
pub fn can_pay_placement(w: &World, g: &GameState, seat: usize, how: Placing, gained: i64) -> bool {
    cheapest_step(w, g, seat).is_some_and(|step| {
        let mut price = placing_price(w, g, seat, how, step);
        price.energy = (price.energy - gained).max(0);
        price.affordable(g, seat)
    })
}

/// Places one presence for `seat`: the next token off its track (uncovering
/// the space under it) onto a hex next to its presence, paying `how`; only
/// tracks whose next step the seat can pay are offered. Returns whether placed.
pub fn place_presence(g: &mut GameState, cx: &mut Cx, seat: usize, how: Placing) -> Res<bool> {
    let w = cx.world;
    if !ai::can_place(&w, g, seat) {
        return Ok(false);
    }
    let hexes = presence_hexes(&w, g, seat);
    // 021: with two tracks the player chooses which one's next token to take.
    let b = ai::board(&w, seat);
    let (t1, t2) = (track_taken(g, seat), track2_taken(g, seat));
    let can_pay = |u: &crate::content::Uncover| placing_price(&w, g, seat, how, u.cost).affordable(g, seat);
    let mut tracks: Vec<(bool, &crate::content::Uncover)> = Vec::new();
    if let Some(u) = b.presence.get(t1).filter(|u| can_pay(u)) {
        tracks.push((false, u));
    }
    if let Some(u) = b.presence2.get(t2).filter(|u| can_pay(u)) {
        tracks.push((true, u));
    }
    let opts = tracks
        .iter()
        .map(|(two, u)| {
            opt(format!(
                "track {}: uncovers {}{}",
                if *two { 2 } else { 1 },
                ai::uncover_label(u),
                price_words(&w, g, seat, how, u.cost)
            ))
        })
        .collect();
    let prompt = format!("Seat {seat}: take presence off which track?");
    let Some(k) = cx.choose(seat, &prompt, opts)? else {
        return Ok(false);
    };
    let second = tracks[k].0;
    let step = tracks[k].1.cost;
    let what = format!("uncovers {}{}", ai::uncover_words(tracks[k].1), price_words(&w, g, seat, how, step));
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!("Seat {seat}: place presence ({what}) on which hex?");
    let Some(j) = cx.choose(seat, &prompt, opts)? else {
        return Ok(false);
    };
    placing_price(&w, g, seat, how, step).pay(g, cx, seat);
    if how == Placing::Rich {
        cx.add_key(g, &rich_key(seat), 1);
    }
    if second {
        cx.add_key(g, &track2_key(seat), 1);
    } else {
        cx.add_key(g, &track_key(seat), 1);
    }
    cx.add_tokens(g, hexes[j], &presence(seat), 1);
    cx.tally(g, "board.presence_placed", 1);
    cx.per_round(g, "board.presence_placed", 1);
    Ok(true)
}

/// Whether seat `s`'s presence is safe as its last (027): while this round's
/// adversary card is below `last_safe_stage` (stage II), nothing destroys a
/// player's last presence.
pub(super) fn last_presence_safe(w: &World, g: &GameState, s: usize) -> bool {
    g.var(adversary::STAGE) < w.int("board.last_safe_stage") && presence_count(g, s) <= 1
}

/// Seat `s` loses its presence on hex `h`, unless it is safe as their last.
/// It leaves the game, as in Spirit Island (034: what it uncovered stays
/// uncovered), and leaves a corruption marker on the hex unless the hex has
/// corruption already. The tally names `source`. Returns whether it went.
pub fn lose_presence(g: &mut GameState, cx: &mut Cx, h: usize, s: usize, source: &str) -> bool {
    let w = cx.world;
    if last_presence_safe(&w, g, s) {
        cx.tally(g, "board.last_presence_safe", 1);
        return false;
    }
    cx.add_tokens(g, h, &presence(s), -1);
    // 041: a shield on the hex goes with the token it shielded.
    if !hardenable(g, h) {
        let shields = g.tokens(h, HARDEN);
        cx.add_tokens(g, h, HARDEN, -shields);
    }
    let tainted = g.tokens(h, MARKER) > 0 || g.tokens(h, NODE) > 0 || corrupted(g, h);
    if w.int("board.presence_marker") > 0 && !tainted {
        cx.add_tokens(g, h, MARKER, 1);
        cx.tally(g, "board.markers", 1);
    }
    cx.tally(g, "board.presence_lost", 1);
    cx.tally(g, &format!("board.presence_lost.{source}"), 1);
    true
}
