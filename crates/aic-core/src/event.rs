//! Typed events emitted while applying actions: the event log, for UIs to
//! animate and for traces.

use serde::Serialize;

use crate::hex::Hex;
use crate::vocab::Resource;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    RoundStarted {
        round: u32,
    },
    /// `phase` is the round entry: `"<module>.<phase>"`, or `"a.x+b.y"` for
    /// a combined phase.
    PhaseStarted {
        phase: String,
    },
    ResourceChanged {
        seat: u8,
        resource: Resource,
        delta: i64,
        total: i64,
    },
    TokensChanged {
        hex: Hex,
        token: String,
        delta: i64,
        total: i64,
    },
    /// A module track (`GameState::vars`) changed.
    VarChanged {
        key: String,
        delta: i64,
        total: i64,
    },
    /// A card moved between piles (`""` for nowhere: the box).
    CardMoved {
        card: String,
        from: String,
        to: String,
    },
    /// A rules step worth a line in the log ("Coastal Republic #7 is corrupted").
    Note {
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hex: Option<Hex>,
    },
    /// The rules stopped for a choice.
    Asked {
        seat: u8,
        prompt: String,
    },
    Passed {
        seat: u8,
    },
    GameEnded {
        won: bool,
        round: u32,
        reason: String,
        detail: String,
    },
}
