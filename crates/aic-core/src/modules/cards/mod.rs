//! Cards and draft (021): the tier-1 and tier-2 decks (one copy of each
//! card; discards reshuffle when the draft finds a deck empty, never for a
//! draw, 111), the opening deal, the
//! Terraforming Mars-style draft with the tier chosen card by card, keeping
//! drafted cards for insight, playing buildings (slots that stack and splay
//! down), programs (they stay in your play area) and events, token engines,
//! building abilities and upkeep, and the sacrifices that pay AI-board powers.

use std::collections::BTreeMap;

use crate::action::Action;
use crate::content::{Ability, CardDef, Op, Tag, TagOrAny, TokenKind, TAGS};
use crate::key::{Key, KeyBuf};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{adversary, ai, alignment, economy, effects};

mod deck;
mod play;
mod price;
mod sacrifice;
mod tableau;
mod triggers;
mod view;

pub use deck::*;
pub use play::*;
pub use price::*;
pub use sacrifice::*;
pub use tableau::*;
pub use triggers::*;
use view::*;

const DRAFT_PARAMS: &[ParamSpec] = &[
    ParamSpec::int("deal", 0, 10, "cards each player drafts from each round after the first, choosing each one's tier"),
    ParamSpec::int("opening_deal", 0, 20, "cards dealt to each player in round 1 (the opening)"),
    ParamSpec::int("opening_free", 0, 20, "opening cards each player keeps free"),
    ParamSpec::int("opening_keep_credits", 0, 20, "credits for each opening card kept beyond the free ones"),
    ParamSpec::int("opening_deal_tier2", 0, 10, "tier-2 cards also dealt to each player in the opening (108)"),
    ParamSpec::int("opening_keep_tier2", 0, 10, "insight to keep an opening tier-2 card; tier-2 keeps take none of the free ones (108)"),
    ParamSpec::int("keep_tier1", 0, 10, "insight to keep a drafted tier-1 card"),
    ParamSpec::int("keep_tier2", 0, 10, "insight to keep a drafted tier-2 card"),
    ParamSpec::int("keep_tier2_upgraded", 0, 10, "insight to keep a tier-2 card once the AI board's keep upgrade is uncovered"),
    ParamSpec::int("insight_credits", 1, 20, "credits for 1 insight, usable only for keep costs"),
    ParamSpec::int("science_every", 0, 10, "1 insight off tier-2 keeps and card plays per this many Science tags (0: off)"),
    ParamSpec::int("industry_every", 0, 10, "1 credit off buildings per this many Industry tags (0: off)"),
    ParamSpec::int("discount_floor", 0, 5, "tag and building discounts stop at this cost; a cost printed under it stays"),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "cards",
    summary: "Card decks, the draft, keeping cards, playing buildings, programs and events",
    requires: &["ai", "adversary", "alignment", "board"],
    variants: &[VariantDef {
        name: "draft",
        summary: "An opening deal, some kept free; then each round a Terraforming Mars draft, the tier chosen card by card; keeps cost insight",
        params: DRAFT_PARAMS,
        phases: PHASES,
        rng: &["cards"],
        uses_cards: true,
        build,
    }, VariantDef {
        name: "off",
        summary: "No draft: players have no cards",
        params: &[],
        phases: PHASES,
        rng: &["cards"],
        uses_cards: true,
        build: build_off,
    }],
};

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::decision(
        "tier",
        "each player builds their draft hand card by card, choosing each card's tier",
    ),
    PhaseSpec::auto("deal", "round 1: deal the opening hand"),
    PhaseSpec::decision(
        "draft",
        "take one card and pass the rest to the next seat, until the hands are gone",
    ),
    PhaseSpec::decision(
        "keep",
        "keep any drafted cards (insight; the opening: free, then credits); the rest are discarded",
    ),
    PhaseSpec::decision(
        "play",
        "play cards, use abilities",
    ),
];

pub fn hand(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.hand.").num(seat)
}

/// Seat `s`'s active buildings: the top card of each slot's stack.
pub fn built(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.built.").num(seat)
}

/// The cards splayed under seat `s`'s active building `top`, bottom first:
/// their effects are off, their tags still count.
pub fn under(seat: usize, top: &str) -> KeyBuf {
    KeyBuf::new("cards.under.").num(seat).text(".").text(top)
}

/// The event seat `s` is resolving (111): out of the hand, not yet discarded,
/// so its own draw can't reshuffle it back.
pub fn resolving(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.resolving.").num(seat)
}

/// Seat `s`'s play area: programs played (their tags count).
pub fn played(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.played.").num(seat)
}

/// Tokens on seat `s`'s card `card` (021: tokens live only on cards).
pub fn tokens_key(seat: usize, card: &str) -> KeyBuf {
    KeyBuf::new("cards.tokens.").num(seat).text(".").text(card)
}

/// Prefix of every seat's draft packet pile.
pub const PACKETS: &str = "cards.packet.";

/// The draft hand in front of seat `s`.
pub fn packet(seat: usize) -> KeyBuf {
    KeyBuf::new(PACKETS).num(seat)
}

/// Cards seat `s` has drafted this round and not yet kept.
pub fn drafted(seat: usize) -> KeyBuf {
    KeyBuf::new("cards.drafted.").num(seat)
}

fn deck(tier: u8) -> KeyBuf {
    KeyBuf::new("cards.deck.").num(tier as usize)
}

fn discard_pile(tier: u8) -> KeyBuf {
    KeyBuf::new("cards.discard.").num(tier as usize)
}

/// The var counting the cards seat `seat` has picked (the agents read it too).
pub fn picked_key(seat: usize) -> String {
    format!("cards.picked.{seat}")
}

/// Set once seat `s`'s Aegis Doctrine has saved a facility this round (module `board`).
pub fn civic_used(seat: usize) -> String {
    format!("cards.civic_used.{seat}")
}

/// Drafted cards `seat` has kept this round.
pub fn kept_key(seat: usize) -> String {
    format!("cards.kept_round.{seat}")
}

pub fn def<'w>(w: &World<'w>, id: &str) -> &'w CardDef {
    w.content.card(id)
}

#[derive(Debug)]
struct Cards {
    deal: i64,
    opening_deal: i64,
    /// 108: tier-2 cards dealt in the opening besides `opening_deal` tier-1 ones.
    opening_deal_tier2: i64,
    insight_credits: i64,
    /// Selected cards (content indices).
    cards: Vec<usize>,
    /// Variant `off`: no draft.
    off: bool,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Cards {
        deal: b.int("deal")?,
        opening_deal: b.int("opening_deal")?,
        opening_deal_tier2: b.int("opening_deal_tier2")?,
        insight_credits: b.int("insight_credits")?,
        cards: b.cards.clone(),
        off: false,
    }))
}

fn build_off(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Cards {
        deal: 0,
        opening_deal: 0,
        opening_deal_tier2: 0,
        insight_credits: 1,
        cards: b.cards.clone(),
        off: true,
    }))
}

impl Module for Cards {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for tier in [1, 2] {
            let mut ids: Vec<Key> = self
                .cards
                .iter()
                .map(|&i| &w.content.cards[i])
                .filter(|c| c.tier == tier)
                .map(|c| Key::new(&c.id))
                .collect();
            shuffle(&mut ids, cx.rng(g, "cards"));
            cx.set_pile(g, &deck(tier), ids);
        }
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            for id in g.seats[s].exhausted.clone() {
                cx.refresh(g, s, &id);
            }
            cx.set_key(g, &civic_used(s), 0);
            cx.set_key(g, &kept_key(s), 0);
        }
    }

    fn begin(&self, phase: usize, g: &mut GameState, cx: &mut Cx) {
        match phase {
            0 => {
                // The opening has no draft (round 1 deals the opening hand).
                // With both decks and discards empty there is nothing to deal.
                let dry = !tier_open(g, 1) && !tier_open(g, 2);
                for s in 0..g.seats.len() {
                    if self.off || g.round <= 1 || self.deal <= 0 || dry {
                        g.seats[s].passed = true;
                    }
                }
            }
            2 => {
                if (0..g.seats.len()).all(|s| g.pile(&packet(s)).is_empty()) {
                    pass_all(g);
                }
            }
            3 => {
                for s in 0..g.seats.len() {
                    if g.pile(&drafted(s)).is_empty() {
                        g.seats[s].passed = true;
                    }
                }
            }
            4 => {
                for s in 0..g.seats.len() {
                    let hand_n = g.pile(&hand(s)).len() as i64;
                    cx.per_round(g, "cards.hand_size", hand_n);
                    let ins = g.resource(s, Resource::Insight);
                    cx.per_round(g, "cards.insight_after_keep", ins);
                    for id in g.pile(&drafted(s)).to_vec() {
                        discard(g, cx, &id, &drafted(s));
                    }
                }
            }
            _ => {}
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        // The opening (round 1): each seat is dealt its opening hand to keep from.
        if self.off || g.round > 1 {
            return Ok(());
        }
        for s in 0..g.seats.len() {
            for (tier, n) in [(1, self.opening_deal), (2, self.opening_deal_tier2)] {
                for _ in 0..n {
                    if !deal_top(g, cx, tier, &drafted(s))? {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn legal(&self, phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        match phase {
            0 => {
                for tier in [1u8, 2] {
                    if tier_open(g, tier) {
                        out.push(Action::Tier { seat, tier });
                    }
                }
            }
            2 => {
                if g.var(&picked_key(s)) == 0 {
                    for id in g.pile(&packet(s)) {
                        out.push(Action::Draft {
                            seat,
                            card: *id,
                        });
                    }
                }
            }
            3 => {
                let free = alignment::free_purchases(g) > 0;
                for id in g.pile(&drafted(s)) {
                    let (_, cr) = self.keep_price(w, g, s, id);
                    if free || g.resource(s, Resource::Credits) >= cr {
                        out.push(Action::Keep {
                            seat,
                            card: *id,
                        });
                    }
                }
            }
            4 => {
                for id in g.pile(&hand(s)) {
                    if blocked(w, g, s, id).is_none() {
                        out.push(Action::Play {
                            seat,
                            card: *id,
                        });
                    }
                }
                for id in g.pile(&built(s)) {
                    if def(w, id).ability.is_some() && ability_blocked(w, g, s, id).is_none() {
                        out.push(Action::Use {
                            seat,
                            card: *id,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn can_pass(&self, phase: usize, _g: &GameState, _seat: u8) -> bool {
        !matches!(phase, 0 | 2)
    }

    fn owns(&self, phase: usize, a: &Action) -> bool {
        matches!(
            (phase, a),
            (0, Action::Tier { .. })
                | (2, Action::Draft { .. })
                | (3, Action::Keep { .. })
                | (4, Action::Play { .. } | Action::Use { .. })
        )
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let s = a.seat() as usize;
        match a {
            Action::Tier { tier, .. } => self.take_tier(g, cx, s, *tier)?,
            Action::Draft { card, .. } => draft_pick(g, cx, s, card)?,
            Action::Keep { card, .. } => self.keep_card(g, cx, s, card)?,
            // 105: what a play or an Action removes fires the seat's fight triggers.
            Action::Play { card, .. } => {
                let before = fight_watch(&cx.world, g, s);
                play_card(g, cx, s, card)?;
                fight_fire(g, cx, s, before)?;
            }
            Action::Use { card, .. } => {
                let before = fight_watch(&cx.world, g, s);
                use_ability(g, cx, s, card)?;
                fight_fire(g, cx, s, before)?;
            }
            _ => return Err(Halt::Illegal(format!("not a card action: {a:?}"))),
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        cards_view(self, g, w, out);
    }
}
