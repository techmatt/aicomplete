//! Every module the engine knows: the current ruleset, one module per
//! rules area. To add one: write `modules/<id>.rs` with a `pub static DEF:
//! ModuleDef`, add it to [`REGISTRY`], and document it in a file of `docs/rules/`
//! (its `README.md` lists them) and in `docs/architecture.md`.
//!
//! The rules areas lean on each other (a card's damage destroys adversary
//! pieces and sways toward Unity), so modules call each other's public
//! functions and read each other's parameters. `ModuleDef::requires` lists
//! only the modules whose parameters a module reads, so the optional ones can
//! be switched off: an optional module's public functions must still work
//! with that module absent. `common`, `effects` and `plan` are shared code,
//! not modules.

use crate::module::ModuleDef;

pub mod adversary;
pub mod ai;
pub mod alignment;
pub mod board;
pub mod cards;
pub mod common;
pub mod economy;
pub mod effects;
pub mod plan;
pub mod threats;

pub static REGISTRY: &[&ModuleDef] = &[
    &board::DEF,
    &adversary::DEF,
    &threats::DEF,
    &alignment::DEF,
    &economy::DEF,
    &ai::DEF,
    &cards::DEF,
];

pub fn find(id: &str) -> Option<&'static ModuleDef> {
    REGISTRY.iter().copied().find(|m| m.id == id)
}
