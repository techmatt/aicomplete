//! Destruction: what absorbs it, in order, and what it leaves.

use super::*;

/// What destroys a piece: it decides the corp units' help, the Sway and the
/// tally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// An adversary troop's strike.
    Troop,
    /// Corp troops crossing a front.
    War,
    /// A threat, Discord card or project, named for the tally.
    Effect(&'static str),
}

impl Cause {
    pub(super) fn name(self) -> &'static str {
        match self {
            Cause::Troop => "march",
            Cause::War => "war",
            Cause::Effect(n) => n,
        }
    }
}

/// A piece of `kind` on hex `h` would be destroyed. A presence safe as its
/// owner's last stays. A Shelter Order in
/// its country, a corp unit at a base next to it (adversary strikes only,
/// [`Cause::Troop`]), a hardening token, a player ambassador on the hex, or an
/// Aegis Dome Array absorbs it (in that order); otherwise it goes, and a war
/// destruction ([`Cause::War`]) sways toward Discord. Returns whether it was
/// destroyed.
pub fn destroy_piece(
    g: &mut GameState,
    cx: &mut Cx,
    h: usize,
    kind: &str,
    cause: Cause,
) -> bool {
    let w = cx.world;
    let c = w.map.hexes[h].country as usize;
    let owner = (0..g.seats.len()).find(|&s| kind == presence(s));
    if owner.is_some_and(|s| last_presence_safe(&w, g, s)) {
        cx.tally(g, "board.last_presence_safe", 1);
        return false;
    }
    let by_troop = cause == Cause::Troop;
    if flagged(g, RoundFlag::Shelter, c) {
        return false;
    }
    if by_troop {
        let guard = within(&w, h, 1).into_iter().find(|&b| is_base(&w, b) && g.tokens(b, UNIT) > 0);
        if let Some(b) = guard {
            cx.add_tokens(g, b, UNIT, -1);
            cx.tally(g, "board.base_absorbed", 1);
            return false;
        }
    }
    if g.tokens(h, HARDEN) > 0 {
        cx.add_tokens(g, h, HARDEN, -1);
        cx.tally(g, "board.hardening_used", 1);
        return false;
    }
    if let Some(s) = player_ambassador(g, h) {
        cx.add_tokens(g, h, &ambassador(s), -1);
        cx.tally(g, "board.ambassador_shield", 1);
        return false;
    }
    if civic_shelter(g, cx, c) {
        return false;
    }
    match owner {
        Some(s) => {
            lose_presence(g, cx, h, s, cause.name());
        }
        None => {
            cx.add_tokens(g, h, kind, -1);
        }
    }
    if kind == STATION || kind == FACTORY {
        if let Some(s) = cube_owner(g, h) {
            cx.add_tokens(g, h, &cube(s), -1);
        }
    }
    if kind == FACTORY {
        cx.tally(g, "board.factories_destroyed", 1);
    }
    cx.tally(g, "board.destroyed", 1);
    cx.note(Some(h), || {
        format!("{} at {} destroyed", piece_name(kind), hex_name(&w, h))
    });
    if cause == Cause::War {
        alignment::sway(g, cx, Side::Discord, w.int("board.destruction_discord"));
    }
    true
}

/// Seat `s` destroys one of its presence in `hexes`, its choice (028): a hit
/// on the player, not a hex. Absorbed as any destruction is, so a player may
/// pick a token it has shielded. Safe as a last presence in stage I.
pub fn destroy_own_presence(
    g: &mut GameState,
    cx: &mut Cx,
    s: usize,
    hexes: &[usize],
    cause: Cause,
) -> Res {
    let w = cx.world;
    let mine: Vec<usize> = hexes.iter().copied().filter(|&h| g.tokens(h, presence(s)) > 0).collect();
    if mine.is_empty() {
        return Ok(());
    }
    if last_presence_safe(&w, g, s) {
        cx.tally(g, "board.last_presence_safe", 1);
        return Ok(());
    }
    let opts = mine.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!("Seat {s}: destroy which of your presence?");
    if let Some(i) = cx.choose(s, &prompt, opts)? {
        destroy_piece(g, cx, mine[i], presence(s), cause);
    }
    Ok(())
}

/// Uses a seat's `CivicShelter` passive (Aegis Dome Array) for a facility or
/// presence destruction in `country`, if one applies.
pub(super) fn civic_shelter(g: &mut GameState, cx: &mut Cx, country: usize) -> bool {
    let w = cx.world;
    for s in 0..g.seats.len() {
        if g.var(&cards::civic_used(s)) == 0
            && has_presence_in(&w, g, s, country)
            && cards::has_passive(&w, g, s, |p| matches!(p, Op::CivicShelter))
        {
            cx.set_key(g, &cards::civic_used(s), 1);
            return true;
        }
    }
    false
}

/// Whether a hardening token on hex `h` can protect something: a
/// player-owned facility that is not corrupted (what troops strike; 022:
/// hardening went on any facility, the adversary's too, where nothing used it).
pub fn hardenable(g: &GameState, h: usize) -> bool {
    has_building(g, h) && cube_owner(g, h).is_some() && !corrupted(g, h)
}
