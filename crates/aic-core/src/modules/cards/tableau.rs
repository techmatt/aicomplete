//! Cards in play: passives, tags, tokens, upkeep and building slots.

use super::*;

/// Standing effects that apply to seat `s`: its AI's innate rule and its
/// powered active buildings.
pub fn passives<'w>(w: &World<'w>, g: &GameState, seat: usize) -> Vec<&'w Op> {
    let mut out: Vec<&'w Op> = ai::board(w, seat).innate.passives.iter().collect();
    for id in g.pile(&built(seat)) {
        if g.var(&unpowered(seat, id)) == 0 {
            out.extend(def(w, id).passives.iter());
        }
    }
    out
}

pub fn has_passive(w: &World, g: &GameState, seat: usize, f: impl Fn(&Op) -> bool) -> bool {
    passives(w, g, seat).into_iter().any(f)
}

/// Every card seat `s` has in play whose tags count: active buildings, the
/// cards splayed under them, and programs in the play area.
pub fn in_play(g: &GameState, seat: usize) -> Vec<Key> {
    let mut out = Vec::new();
    for top in g.pile(&built(seat)) {
        out.extend(g.pile(&under(seat, top)).iter().cloned());
        out.push(top.clone());
    }
    out.extend(g.pile(&played(seat)).iter().cloned());
    out
}

/// Seat `s`'s tags of one kind, over every card it has in play (021).
pub fn tags(w: &World, g: &GameState, seat: usize, tag: Tag) -> i64 {
    tag_counts(w, g, seat)[tag as usize]
}

/// Seat `s`'s tags of every kind, indexed by `Tag as usize`: one pass over its
/// cards in play, for a caller that wants more than one count.
pub fn tag_counts(w: &World, g: &GameState, seat: usize) -> [i64; TAGS.len()] {
    let mut out = [0; TAGS.len()];
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
    g.pile(&built(seat))
        .iter()
        .filter(|id| {
            def(w, id)
                .tokens
                .as_ref()
                .is_some_and(|t| kind.is_none_or(|k| t.kind == k))
        })
        .cloned()
        .collect()
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

/// Building upkeep for seat `s` (in the economy's upkeep): upkeep
/// conversions, then token cards gain their tokens.
pub fn upkeep(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    for p in passives(&w, g, seat) {
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
    for id in token_cards(&w, g, seat, None) {
        let n = def(&w, &id).tokens.as_ref().map_or(0, |t| t.per_round);
        if n > 0 && g.var(&unpowered(seat, &id)) == 0 {
            add_tokens(g, cx, seat, &id, n);
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
pub(super) fn slotted(w: &World, g: &GameState, seat: usize) -> Vec<Key> {
    g.pile(&built(seat))
        .iter()
        .filter(|id| !def(w, id).slotless)
        .cloned()
        .collect()
}

/// Stacks a building `d` may be built over: tops carrying its `replaces`
/// tag, or any slotted top.
pub(super) fn stack_targets(w: &World, g: &GameState, seat: usize, d: &CardDef) -> Vec<Key> {
    slotted(w, g, seat)
        .into_iter()
        .filter(|top| {
            d.replaces
                .as_ref()
                .is_none_or(|t| def(w, top).tags.contains(t))
        })
        .collect()
}
