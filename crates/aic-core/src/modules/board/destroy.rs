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

/// One protection a destruction may spend (r75, Matt's ruling 5: the owner chooses which, or none).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guard {
    /// A corp troop at base `.0` next to the hex (adversary troops' strikes only).
    Base(usize),
    /// A hardening token on the hex.
    Harden,
    /// The owner's own ambassador on the hex (137: armor).
    Ambassador(usize),
    /// The Aegis Doctrine capstone (the Protector's own pieces, once a round).
    Aegis(usize),
}

/// The seat whose piece of `kind` on hex `h` is: a node's owner, a station's or factory's cube owner.
fn piece_owner(g: &GameState, h: usize, kind: &str) -> Option<usize> {
    (0..g.seats.len())
        .find(|&s| kind == node(s))
        .or_else(|| if kind == STATION || kind == FACTORY { cube_owner(g, h) } else { None })
}

/// The protections a destruction of the `owner`'s piece on hex `h` could spend, in print order (read-only:
/// the agents' shelter forecast counts from it too).
pub fn guards(w: &World, g: &GameState, h: usize, owner: Option<usize>, cause: Cause) -> Vec<Guard> {
    let mut out = Vec::new();
    if cause == Cause::Troop {
        // Each base next to it with a corp troop is an option of its own (the owner chooses which).
        out.extend(within(w, h, 1).into_iter().filter(|&b| is_base(w, b) && g.tokens(b, UNIT) > 0).map(Guard::Base));
    }
    if g.tokens(h, HARDEN) > 0 {
        out.push(Guard::Harden);
    }
    if let Some(s) = owner {
        if g.tokens(h, ambassador(s)) > 0 {
            out.push(Guard::Ambassador(s));
        }
        if aegis_ready(w, g, s) {
            out.push(Guard::Aegis(s));
        }
    }
    out
}

/// A piece of `kind` on hex `h` would be destroyed (175: a last node too). A Shelter Order in its country
/// keeps it, spending nothing. Otherwise r75 (Matt's ruling 5): its owner (seat 0 for a neutral piece)
/// chooses which protection to spend, or none: a corp troop at a base next to it (adversary strikes only,
/// [`Cause::Troop`]), a hardening token, the owner's own ambassador on the hex (137: armor; only your own
/// guards your piece), or the Aegis Doctrine capstone (the Protector's own pieces). Unprotected it goes, and
/// a war destruction ([`Cause::War`]) sways toward Discord. Returns whether it was destroyed.
pub fn destroy_piece(g: &mut GameState, cx: &mut Cx, h: usize, kind: &str, cause: Cause) -> Res<bool> {
    let w = cx.world;
    let c = w.map.hexes[h].country as usize;
    if flagged(g, RoundFlag::Shelter, c) {
        return Ok(false);
    }
    let owner = piece_owner(g, h, kind);
    let options = guards(&w, g, h, owner, cause);
    if !options.is_empty() {
        let mut opts: Vec<crate::state::Opt> = options
            .iter()
            .map(|o| {
                opt(match o {
                    Guard::Base(b) => format!("spend the {} at {}", crate::modules::common::DRONE, hex_name(&w, *b)),
                    Guard::Harden => "spend the hardening token".to_string(),
                    Guard::Ambassador(_) => "spend your ambassador there".to_string(),
                    Guard::Aegis(_) => "use the Aegis Doctrine".to_string(),
                })
            })
            .collect();
        opts.push(opt(LET_IT_GO));
        let who = owner.unwrap_or(0);
        // 184: a node is "seat N's node" ("the seat 3's node" read wrong): "the" only before a plain piece name.
        let prompt = || {
            let piece = piece_name(kind);
            let the = if piece.starts_with("seat ") { "" } else { "the " };
            format!("Seat {who}: {the}{piece} at {} would be destroyed: spend which protection?", hex_name(&w, h))
        };
        let pick = cx.choose_loss(who, prompt, opts)?.and_then(|i| options.get(i).copied());
        match pick {
            Some(Guard::Base(b)) => {
                cx.add_tokens(g, b, UNIT, -1);
                cx.tally(g, "board.base_absorbed", 1);
                return Ok(false);
            }
            Some(Guard::Harden) => {
                cx.add_tokens(g, h, HARDEN, -1);
                cx.tally(g, "board.hardening_used", 1);
                return Ok(false);
            }
            Some(Guard::Ambassador(s)) => {
                cx.add_tokens(g, h, ambassador(s), -1);
                cx.tally(g, "board.ambassador_shield", 1);
                return Ok(false);
            }
            Some(Guard::Aegis(s)) => {
                cx.set_key(g, &cards::civic_used(s), 1);
                cx.tally(g, "board.aegis_used", 1);
                return Ok(false);
            }
            None => cx.tally(g, "board.protection_declined", 1),
        }
    }
    match (0..g.seats.len()).find(|&s| kind == node(s)) {
        Some(s) => {
            lose_node(g, cx, h, s, cause.name());
        }
        None => {
            cx.add_tokens(g, h, kind, -1);
        }
    }
    if kind == STATION || kind == FACTORY {
        if let Some(s) = cube_owner(g, h) {
            cx.add_tokens(g, h, cube(s), -1);
        }
        // r75: a declined hardening token goes with the facility it guarded (as `lose_node` strips one).
        if !hardenable(g, h) && node_owner(g, h).is_none() {
            let n = g.tokens(h, HARDEN);
            cx.add_tokens(g, h, HARDEN, -n);
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
        alignment::discord_from(g, cx, "war", w.int("board.destruction_discord"));
    }
    Ok(true)
}

/// The option that spends no protection (r75: "or none").
pub const LET_IT_GO: &str = "spend none: it is destroyed";

/// Seat `s` destroys one of its nodes in `hexes`, its choice (028): a hit
/// on the player, not a hex. Absorbed as any destruction is, so a player may
/// pick a token it has shielded.
pub fn destroy_own_node(
    g: &mut GameState,
    cx: &mut Cx,
    s: usize,
    hexes: &[usize],
    cause: Cause,
    why: Option<&str>,
) -> Res {
    let w = cx.world;
    let mine: Vec<usize> = hexes.iter().copied().filter(|&h| g.tokens(h, node(s)) > 0).collect();
    if mine.is_empty() {
        return Ok(());
    }
    let opts = mine.iter().map(|&h| hex_opt(&w, h)).collect();
    // 118: named for what destroys it (Agitators, Orbital Strike, Offensive).
    let prompt = || match why {
        Some(n) => format!("{n}: seat {s}: destroy which of your nodes?"),
        None => format!("Seat {s}: destroy which of your nodes?"),
    };
    if let Some(i) = cx.choose_loss(s, prompt, opts)? {
        destroy_piece(g, cx, mine[i], node(s), cause)?;
    }
    Ok(())
}

/// Whether seat `s` holds the Aegis Doctrine capstone (`CivicShelter`) unused this round: r75 (Matt's
/// ruling 5) it guards only `s`'s own pieces, when `s` chooses (it was spent on the first destruction of
/// anyone's piece in a country `s` was in).
fn aegis_ready(w: &World, g: &GameState, s: usize) -> bool {
    g.var(&cards::civic_used(s)) == 0 && cards::has_passive(w, g, s, |p| matches!(p, Op::CivicShelter))
}

/// Whether a hardening token on hex `h` can protect something: a
/// player-owned facility that is not corrupted (what troops strike; 022:
/// hardening went on any facility, the adversary's too, where nothing used it).
pub fn hardenable(g: &GameState, h: usize) -> bool {
    has_building(g, h) && cube_owner(g, h).is_some() && !corrupted(g, h)
}
