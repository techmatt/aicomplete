//! Aggregates an experiment's games into the measures of
//! `docs/principles.md` and writes the cycle report: `report.md` (tables,
//! findings, the saved records), `results.json` (everything, machine
//! readable), `games.csv` and `cards.csv` (one row per game, per card), and
//! `records/` (game records worth a human look, openable in the viewer).
//!
//! - `stats`: the per-cell and per-group measures, paired comparisons;
//! - `records`: picking, writing and pruning the saved records;
//! - `tables`: `games.csv`, `cards.csv` and `results.json`;
//! - `markdown`: `report.md`, with its findings in `findings`.

mod findings;
mod markdown;
mod records;
mod stats;
mod tables;

use std::collections::BTreeMap;

use crate::experiment::{CardCounts, Cell, FailedGame, GameResult, Spec};

pub use markdown::markdown;
pub use records::{file_stem, prune_records, save_records, write_whole, SavedRecord};
pub use stats::{
    agent_pairs, card_table, fresh_confirmation, groups, head_to_head, summarize, time_split,
    top_agent, variant_diffs, AgentPair, CellSummary, FreshRow, GroupSummary, HeadToHead,
    VariantDiff,
};
pub use tables::{cards_csv, games_csv, results_json};

/// One report's inputs: the spec and grid, the games and what was made of
/// them.
pub struct Run<'a> {
    pub spec: &'a Spec,
    pub cells: &'a [Cell],
    pub games: &'a [GameResult],
    /// Games that panicked: in no measure.
    pub failed: &'a [FailedGame],
    pub sums: &'a [CellSummary],
    pub groups: &'a [GroupSummary],
    pub cards: &'a BTreeMap<String, BTreeMap<String, CardCounts>>,
    pub head_to_head: &'a [HeadToHead],
    pub variant_diffs: &'a [VariantDiff],
    pub agent_pairs: &'a [AgentPair],
    /// Empty unless the spec names `selection_seeds`.
    pub fresh_confirmation: &'a [FreshRow],
    /// The strongest agent, and the difficulty that says so.
    pub top_agent: Option<(String, i64)>,
    pub records: &'a [SavedRecord],
    pub wall_s: f64,
    pub threads: usize,
    pub out_dir: &'a str,
    /// Games the run plans: the grid's, or under a gate the gate's plan as
    /// things stand; fewer in `games` is a partial report (a run still
    /// going, or stopped).
    pub planned: usize,
    /// Of `games`, those read back from an earlier run (`--resume`).
    pub resumed: usize,
    /// The dominance gate, for a spec with one.
    pub gate: Option<&'a crate::gate::GateState>,
}
