//! Player actions. Every action names its seat: within a decision phase any
//! seat may act in any order until every seat has passed. Cards, threats
//! and choices are named by content id or option text, so action logs
//! survive content reordering.

use serde::{Deserialize, Serialize};

use crate::hex::Hex;
use crate::key::Key;
use crate::vocab::Resource;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// The seat is done for this phase (the engine).
    Pass { seat: u8 },
    /// Answer the open choice (`GameState::pending`; the engine).
    Choose {
        seat: u8,
        option: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target: Option<Hex>,
    },
    /// Draft: the next card of your draft hand comes from this tier (module `cards`, 021: card by card).
    Tier { seat: u8, tier: u8 },
    /// Draft: take a card from the hand in front of you (module `cards`).
    Draft { seat: u8, card: Key },
    /// Draft: buy a drafted card into your hand (module `cards`).
    Keep { seat: u8, card: Key },
    /// Play a card from your hand (module `cards`).
    Play { seat: u8, card: Key },
    /// Use a building's once-a-round ability (module `cards`).
    Use { seat: u8, card: Key },
    /// Use an AI-board power, paid by its printed cost and tags (module `ai`, 021).
    Power { seat: u8, power: u8 },
    /// Put `amount` of a resource on a threat's counter (module `threats`).
    Pay {
        seat: u8,
        threat: Key,
        resource: Resource,
        amount: i64,
    },
    /// Take back everything you put on a threat's counter in a resource
    /// (module `threats`).
    Withdraw {
        seat: u8,
        threat: Key,
        resource: Resource,
    },
    /// Sway toward Discord to produce one antimatter (module `alignment`).
    Antimatter { seat: u8 },
    /// A standard action (module `board`): deliberately poorly priced.
    Standard { seat: u8, act: String },
    /// Sell a card from your hand (module `board`).
    Sell { seat: u8, card: Key },
    /// Use a completed megaproject's converter, once a round (module `board`, 074).
    Convert { seat: u8, country: u8 },
}

impl Action {
    pub fn seat(&self) -> u8 {
        match self {
            Action::Pass { seat }
            | Action::Choose { seat, .. }
            | Action::Tier { seat, .. }
            | Action::Draft { seat, .. }
            | Action::Keep { seat, .. }
            | Action::Play { seat, .. }
            | Action::Use { seat, .. }
            | Action::Power { seat, .. }
            | Action::Pay { seat, .. }
            | Action::Withdraw { seat, .. }
            | Action::Antimatter { seat }
            | Action::Standard { seat, .. }
            | Action::Sell { seat, .. }
            | Action::Convert { seat, .. } => *seat,
        }
    }
}
