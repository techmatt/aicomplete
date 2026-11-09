//! AI Complete rules engine.
//!
//! Pure and deterministic: no I/O, no wall clock, no global or thread RNG,
//! and no hash-order leaks (every map in the state is ordered, a `FlatMap`;
//! hash maps are for lookups only and are never iterated into a result). All
//! randomness comes from seeded per-purpose streams held in [`GameState`]; the state is
//! serde-serializable and a game replays exactly from config + seed +
//! action list.
//!
//! A game is built from a [`Config`]: modules, each in one variant with
//! named integer parameters. [`Rules::new`] compiles a config once; its
//! `new_game`, `legal_actions_into` and `apply_mut` are the hot-loop path.
//! The free functions below are the convenient (slower) path that rebuilds
//! the rules from the config embedded in the state on every call.
//!
//! What the rules are is in `docs/rules/` (start at its `README.md`); how modules, configs and
//! tunables fit together is in `docs/architecture.md`.

pub mod action;
pub mod config;
pub mod content;
pub mod driver;
pub mod event;
pub mod face;
mod hash;
pub mod hex;
pub mod key;
pub mod map;
pub mod module;
pub mod modules;
pub mod op;
pub mod record;
pub mod rng;
pub mod rules;
pub mod state;
pub mod view;
pub mod vocab;

pub use action::Action;
pub use config::Config;
pub use content::{content, content_hash, Content};
pub use driver::{play, replay, Agent, PlayedGame};
pub use event::Event;
pub use hash::fnv1a64;
pub use key::Key;
pub use map::Map;
pub use record::SavedGame;
pub use rules::Rules;
pub use state::{GameState, Outcome, Stats};
pub use view::View;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Bump on **any** change to how the rules play out (module behavior,
/// engine loop, RNG use), so results from different rules are never
/// mistaken for the same experiment. Recorded with every result. The
/// `golden_game` test (`tests/mod.rs`) fails until it is bumped and
/// `GOLDEN_REVISION`, `GOLDEN_FINGERPRINT` and `GOLDEN_CONTENT_HASH` there are
/// updated, with `GOLDEN` and `GOLDEN_CONTENT` in `scripts/wasm-smoke.mjs`.
pub const RULES_REVISION: u32 = 69;

/// The rules of the config embedded in `state`, checked against it.
fn rules_for(state: &GameState) -> Result<Rules, String> {
    let rules = Rules::new(&state.config)?;
    rules.check_state(state)?;
    Ok(rules)
}

/// Legal actions in `state` (empty once the game is over).
pub fn legal_actions(state: &GameState) -> Result<Vec<Action>, String> {
    Ok(rules_for(state)?.legal_actions(state))
}

/// Applies an action, returning the next state and the events it produced.
pub fn apply(state: &GameState, action: &Action) -> Result<(GameState, Vec<Event>), String> {
    rules_for(state)?.apply(state, action)
}

/// The generic view of a state (see `view.rs`), for clients.
pub fn view(state: &GameState) -> Result<View, String> {
    Ok(rules_for(state)?.view(state))
}

/// The module registry as JSON-friendly data: every module, its variants,
/// their parameters and phases. For UIs, docs and the sim's `modules` command.
pub fn describe_modules() -> serde_json::Value {
    use serde_json::json;
    let modules: Vec<_> = modules::REGISTRY
        .iter()
        .map(|m| {
            let variants: Vec<_> = m
                .variants
                .iter()
                .map(|v| {
                    let params: Vec<_> = v
                        .params
                        .iter()
                        .map(|p| json!({"name": p.name, "kind": p.kind.name(), "min": p.min, "max": p.max, "doc": p.doc}))
                        .collect();
                    let phases: Vec<_> = v
                        .phases
                        .iter()
                        .map(|p| json!({"name": p.name, "kind": p.kind.name(), "doc": p.doc}))
                        .collect();
                    json!({
                        "name": v.name, "summary": v.summary, "params": params, "phases": phases,
                        "rng": v.rng, "uses_cards": v.uses_cards,
                    })
                })
                .collect();
            json!({"id": m.id, "summary": m.summary, "variants": variants})
        })
        .collect();
    json!(modules)
}

#[cfg(test)]
mod tests;
