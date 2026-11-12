//! The decks, the discard piles and dealing.

use super::*;

/// Discards `card` from pile `from` to its tier's discard pile.
pub fn discard(g: &mut GameState, cx: &mut Cx, card: &str, from: &str) {
    let w = cx.world;
    let to = discard_pile(def(&w, card).tier);
    cx.move_card(g, card, from, &to, true)
        .expect("card to discard is there");
    // 199 (stats only): a kept card leaving the hand unplayed (sold, discarded) is a dead keep now; its keys clear,
    // so the card coming back un-kept isn't read as a keep.
    if let Some(s) = from.strip_prefix("cards.hand.").and_then(|n| n.parse::<usize>().ok()) {
        play::keep_left(g, cx, s, card);
    }
}

/// Seat `s` draws up to `n` tier-1 cards into its hand, no keep paid (023's
/// `draw`). r48 (Matt, 2026-11-05): if you must draw and can't, reshuffle the
/// discard and draw. 111's loop (Breakthrough Leak and Capital Injection drew
/// each other back from the discard) can't return: a played event goes to its
/// player's played-events pile ([`events`]), never the discard.
pub fn draw(g: &mut GameState, cx: &mut Cx, seat: usize, n: i64) -> Res {
    for _ in 0..n.max(0) {
        if !deal_top(g, cx, 1, &hand(seat))? {
            break;
        }
        cx.tally(g, "cards.drawn", 1);
    }
    Ok(())
}

/// A draw has a card to take: the tier-1 deck, or its discard to reshuffle (r48).
pub(crate) fn can_draw(g: &GameState) -> bool {
    tier_open(g, 1)
}

/// The top card of a tier's deck, reshuffling its discards if it is empty.
fn top_of(g: &mut GameState, cx: &mut Cx, tier: u8) -> Option<Key> {
    if g.pile(&deck(tier)).is_empty() {
        let mut cards = g.pile(&discard_pile(tier)).to_vec();
        if cards.is_empty() {
            return None;
        }
        shuffle(&mut cards, cx.deck_rng(g));
        cx.set_pile(g, &discard_pile(tier), Vec::new());
        cx.set_pile(g, &deck(tier), cards);
        cx.note(None, || format!("Tier-{tier} discards reshuffled into the deck"));
    }
    g.pile(&deck(tier)).first().cloned()
}

/// Moves the top card of a tier's deck (see [`top_of`]) to pile `to`; false
/// with no card to move.
pub(super) fn deal_top(g: &mut GameState, cx: &mut Cx, tier: u8, to: &str) -> Res<bool> {
    let Some(id) = top_of(g, cx, tier) else {
        return Ok(false);
    };
    cx.move_card(g, &id, &deck(tier), to, false)?;
    Ok(true)
}

pub(super) fn all_picked(g: &GameState) -> bool {
    (0..g.seats.len()).all(|s| g.var(&picked_key(s)) > 0 || g.pile(&packet(s)).is_empty())
}

pub(crate) fn tier_open(g: &GameState, tier: u8) -> bool {
    !(g.pile(&deck(tier)).is_empty() && g.pile(&discard_pile(tier)).is_empty())
}

/// Every seat is done with this phase.
pub(super) fn pass_all(g: &mut GameState) {
    for seat in g.seats.iter_mut() {
        seat.passed = true;
    }
}
