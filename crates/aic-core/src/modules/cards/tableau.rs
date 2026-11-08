//! Cards in play: passives, tags, tokens, upkeep and building slots.

use super::*;

/// Standing effects that apply to seat `s`: its AI's innate rule and its
/// active buildings.
pub fn passives<'w>(w: &World<'w>, g: &GameState, seat: usize) -> Vec<&'w Op> {
    let mut out = Vec::new();
    each_passive(w, g, seat, |p| out.push(p));
    out
}

/// Each of [`passives`] in turn, with no list to build.
pub fn each_passive<'w>(w: &World<'w>, g: &GameState, seat: usize, mut f: impl FnMut(&'w Op)) {
    ai::board(w, seat).innate.passives.iter().for_each(&mut f);
    ai::capstones(w, g, seat).flat_map(|c| c.passives.iter()).for_each(&mut f);
    // 135: a face-down building does nothing.
    let down = has_disabled(g, seat);
    for id in g.pile(&built(seat)) {
        if !(down && disabled(g, seat, id)) {
            def(w, id).passives.iter().for_each(&mut f);
        }
    }
}

pub fn has_passive(w: &World, g: &GameState, seat: usize, f: impl Fn(&Op) -> bool) -> bool {
    // A walk, not a list: the agents ask it of every seat in every eval (103).
    let mut found = false;
    each_passive(w, g, seat, |p| found = found || f(p));
    found
}

/// [`has_passive`] for a hot path (105): the innate and the active buildings,
/// and the tracks read only when one of the board's capstones could match.
pub fn holds(w: &World, g: &GameState, seat: usize, f: impl Fn(&Op) -> bool) -> bool {
    let b = ai::board(w, seat);
    let down = has_disabled(g, seat);
    let on = |id: &Key| !(down && disabled(g, seat, id)) && def(w, id).passives.iter().any(&f);
    if b.innate.passives.iter().any(&f) || g.pile(&built(seat)).iter().any(on) {
        return true;
    }
    let caps = || b.economy_track.iter().chain(&b.play_track).filter_map(|u| u.capstone.as_ref());
    caps().flat_map(|c| c.passives.iter()).any(&f) && ai::capstones(w, g, seat).flat_map(|c| c.passives.iter()).any(&f)
}

/// Every card seat `s` has in play whose tags count: active buildings, the
/// cards splayed under them, and programs in the play area.
pub fn in_play(g: &GameState, seat: usize) -> Vec<Key> {
    let mut out = Vec::new();
    for top in g.pile(&built(seat)) {
        out.extend(g.pile(&under(seat, top)).iter().cloned());
        out.push(*top);
    }
    out.extend(g.pile(&played(seat)).iter().cloned());
    out
}

/// Seat `s`'s tags of one kind: its cards in play and its board's (021, 150).
pub fn tags(w: &World, g: &GameState, seat: usize, tag: Tag) -> i64 {
    tag_counts(w, g, seat)[tag as usize]
}

/// Seat `s`'s tags of every kind, indexed by `Tag as usize`: one pass over its
/// cards in play, for a caller that wants more than one count, its board's
/// printed tags and the tags its tracks have revealed. Board tags count like
/// any tag (149 revealed, 150 printed: one tag rule); a sacrifice discards
/// cards, so never them.
pub fn tag_counts(w: &World, g: &GameState, seat: usize) -> [i64; TAGS.len()] {
    let mut out = [0; TAGS.len()];
    for t in ai::printed_tags(w, seat).chain(ai::revealed_tags(w, g, seat)) {
        out[*t as usize] += 1;
    }
    let mut count = |ids: &[Key]| {
        for t in ids.iter().flat_map(|id| &def(w, id).tags) {
            out[*t as usize] += 1;
        }
    };
    let tops = g.pile(&built(seat));
    for top in tops {
        count(g.pile(&under(seat, top)));
    }
    count(tops);
    count(g.pile(&played(seat)));
    out
}

/// Active buildings of seat `s` holding tokens of `kind`.
pub fn token_cards(w: &World, g: &GameState, seat: usize, kind: Option<TokenKind>) -> Vec<Key> {
    g.pile(&built(seat)).iter().filter(|id| holds_tokens(w, id, kind)).cloned().collect()
}

/// Whether seat `s` has an active building holding tokens of `kind`: [`token_cards`]
/// with nothing collected (103: the agents ask it of every card in hand).
pub fn has_token_card(w: &World, g: &GameState, seat: usize, kind: Option<TokenKind>) -> bool {
    g.pile(&built(seat)).iter().any(|id| holds_tokens(w, id, kind))
}

fn holds_tokens(w: &World, id: &str, kind: Option<TokenKind>) -> bool {
    def(w, id).tokens.as_ref().is_some_and(|t| kind.is_none_or(|k| t.kind == k))
}

/// Adds `n` tokens to seat `s`'s card (negative removes; never below 0).
pub fn add_tokens(g: &mut GameState, cx: &mut Cx, seat: usize, card: &str, n: i64) {
    let k = tokens_key(seat, card);
    let n = n.max(-g.var(&k));
    if n == 0 {
        return;
    }
    cx.add_key(g, &k, n);
    if n > 0 {
        cx.tally(g, "cards.tokens_gained", n);
        cx.per_round(g, "cards.tokens_gained", n);
    } else {
        cx.tally(g, "cards.tokens_spent", -n);
        cx.per_round(g, "cards.tokens_spent", -n);
    }
}

/// Building upkeep for seat `s` (in the economy's upkeep): project drains and
/// upkeep conversions. No card gains tokens by itself (Matt's ruling 12).
pub fn upkeep(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    for p in passives(&w, g, seat) {
        if let Op::ProjectDrain { amount } = p {
            for id in adversary::projects(g) {
                adversary::drain_project(g, cx, &id, *amount);
            }
        }
        if let Op::ConvertUpkeep {
            from,
            amount,
            to,
            gain,
        } = p
        {
            if g.resource(seat, *from) >= *amount {
                cx.add_resource(g, seat, *from, -amount);
                cx.add_resource(g, seat, *to, *gain);
            }
        }
    }
}

/// Building slots seat `s` has: the AI board's (`ai.slots`) plus expansions.
pub fn capacity(w: &World, g: &GameState, seat: usize) -> i64 {
    let extra: i64 = passives(w, g, seat)
        .iter()
        .map(|p| match p {
            Op::Slots { amount } => *amount,
            _ => 0,
        })
        .sum();
    ai::slots(w, seat) + extra
}

/// Active buildings that take a slot (each the top of a stack).
pub(super) fn slotted(g: &GameState, seat: usize) -> Vec<Key> {
    g.pile(&built(seat)).to_vec()
}

/// Stacks a building `d` may be built over: tops carrying its `replaces`
/// tag, or any slotted top.
pub(super) fn stack_targets(w: &World, g: &GameState, seat: usize, d: &CardDef) -> Vec<Key> {
    slotted(g, seat)
        .into_iter()
        .filter(|top| {
            d.replaces
                .as_ref()
                .is_none_or(|t| def(w, top).tags.contains(t))
        })
        .collect()
}
