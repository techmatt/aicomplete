//! The serializable game state. Everything the rules read is something a
//! physical copy could hold: tokens on hexes, resources on seat boards,
//! exhausted cards, a round and phase marker, numbered tracks.
//! Two parts are not game components: `rng` (the dice) and `stats`
//! (observation: rules never read it, but the simulator and agents do, so
//! rollouts can't drop it: `planner` keys off `stats.actions`, the heuristic
//! reads `adversary.projects_resolved`). Geometry lives in [`crate::Map`].

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::config::Config;
use crate::hex::Hex;
use crate::key::{FlatMap, Key, Table};
use crate::rng::{as_string, RngStreams};
use crate::vocab::Resource;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// Decimal string in JSON.
    #[serde(with = "as_string")]
    pub seed: u64,
    /// The resolved config this game was built from (round filled in), so a
    /// state is self-describing and replayable. Shared, never changed by
    /// play: cloning a state (search does it constantly) does not copy it.
    pub config: Arc<Config>,
    /// 1-based once play starts.
    pub round: u32,
    /// Index into `config.round` of the current phase.
    pub phase: usize,
    pub seats: Vec<Seat>,
    /// Per-hex contents, aligned with `Map::hexes`. Each hex, each pile, each
    /// RNG stream and each per-round counter is shared between copies of a
    /// state until one changes it (`Arc::make_mut`): an action touches a few
    /// and a search copies all of them (030). The table itself is shared too
    /// (104): most actions touch no hex, and a copy and a drop of the table
    /// counted every hex's share.
    pub hexes: Arc<Vec<Arc<HexState>>>,
    /// Global tracks owned by modules, keyed `"<module>.<name>"`.
    pub vars: Table<i64>,
    /// Ordered card piles (decks, discards, hands, slots, building rows),
    /// keyed `"<module>.<name>"`; index 0 is the top. Missing means empty.
    #[serde(default)]
    pub piles: Table<Arc<Vec<Key>>>,
    /// A choice the players must make before the game goes on: the rules
    /// stopped in the middle of resolving something (see [`Pending`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<Arc<Pending>>,
    pub rng: RngStreams,
    pub stats: Stats,
    pub outcome: Option<Outcome>,
}

/// One seat per player (as many players as countries). All seats are driven
/// by the same team agent; hands are open.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Seat {
    pub resources: FlatMap<Resource, i64>,
    /// Passed in the current decision phase.
    pub passed: bool,
    /// Ids of this seat's used (face-down) cards, sorted. Set and refreshed
    /// by the module that owns each card, via `Cx::exhaust` / `Cx::refresh`.
    pub exhausted: Vec<Key>,
}

/// A choice in the middle of resolving a phase or an action ("players
/// choose"). The state shown is partly resolved; `snapshot` is the state
/// before the phase or action began. Answering (`Action::Choose`) replays the
/// phase or action from `snapshot` with every choice made so far, so a
/// module's resolution code is written straight through and never has to
/// store where it stopped. At a table this is the step the players are in
/// the middle of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pending {
    pub trigger: Trigger,
    /// Choices made so far in this resolution, in order.
    pub made: Vec<Pick>,
    /// The question now open.
    pub ask: Ask,
    /// Shared: a search clones pending states far more often than it
    /// answers them (`Arc::make_mut` to change one).
    pub snapshot: Arc<GameState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Trigger {
    /// The current (automatic) phase.
    Phase,
    /// A player action.
    Action { action: Action },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pick {
    pub option: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Hex>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ask {
    /// The seat that answers (a team choice goes to the seat that started it,
    /// or seat 0 in an adversary step).
    pub seat: u8,
    pub prompt: String,
    pub options: Vec<Opt>,
    /// What kind of question it is. Written only when not plain, so a plain
    /// ask's JSON, and with it a state's fingerprint, doesn't carry it.
    #[serde(default, skip_serializing_if = "AskKind::is_plain")]
    pub kind: AskKind,
}

/// What an agent may need to know about an [`Ask`] beyond its words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AskKind {
    #[default]
    Plain,
    /// Which of your things to lose, or where a loss lands.
    Loss,
}

impl AskKind {
    fn is_plain(&self) -> bool {
        *self == AskKind::Plain
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opt {
    pub option: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Hex>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HexState {
    pub tokens: Table<i64>,
}

/// Counters the simulator aggregates. Filled by the engine and modules;
/// deterministic and replayed like the rest of the state.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Stats {
    /// Per-round counters, keyed `"<module>.<name>"`: entry `r` is round
    /// `r`, entry 0 setup (017). `adversary.ops` counts adversary operations
    /// (token placements and moves), a proxy for table time.
    pub per_round: Table<Arc<Vec<i64>>>,
    /// Player actions applied, passes included.
    pub actions: u32,
    /// Free-form counters, keyed `"<module>.<name>"`.
    pub counters: Table<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub won: bool,
    /// The round in which the game ended.
    pub round: u32,
    /// Stable reason code (`"purge"`, `"unity_deck"`, `"deck_empty"`,
    /// `"discord_deck"`, `"node"`, `"round_limit"`), for aggregation.
    pub reason: String,
    /// Human-readable detail.
    pub detail: String,
}

impl GameState {
    /// Where `part` (`"threats.counter"`, ...) sits in the round, if it does.
    pub fn phase_index(&self, part: &str) -> Option<usize> {
        self.config.round.iter().position(|p| p.split('+').any(|q| q == part))
    }

    pub fn tokens(&self, hex: usize, kind: &str) -> i64 {
        // A scan, not a search: a hex holds a few kinds, names of other lengths
        // are refused without reading them, and rules and heuristics ask for
        // tens of thousands of these a position.
        let found = self.hexes[hex].tokens.iter().find(|(k, _)| k.as_str() == kind);
        found.map_or(0, |(_, n)| *n)
    }

    pub fn resource(&self, seat: usize, r: Resource) -> i64 {
        self.seats[seat].resources.get(&r).copied().unwrap_or(0)
    }

    pub fn var(&self, key: &str) -> i64 {
        self.vars.get(key).copied().unwrap_or(0)
    }

    /// A card pile (empty if absent).
    pub fn pile(&self, key: &str) -> &[Key] {
        self.piles.get(key).map_or(&[], |v| v.as_slice())
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    /// Serialized JSON (stable: every map is ordered).
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("state serializes")
    }

    /// FNV-1a of the JSON form less `stats`, as 16 hex digits; for replay
    /// checks and logs. 148: measures stay out, so a measures-only tally moves
    /// no fingerprint (147's stage tally moved `GOLDEN_FINGERPRINT` and no game).
    pub fn fingerprint(&self) -> String {
        let mut play = self.clone();
        play.stats = Stats::default();
        format!("{:016x}", crate::hash::fnv1a64(play.to_json().as_bytes()))
    }
}
