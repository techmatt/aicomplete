//! Thin wasm-bindgen layer over `aic-core`. No rules live here.
//!
//! Configs, states, actions and events cross the boundary as JSON strings.
//! Seeds are decimal strings. `apply` returns an [`Applied`] whose `state`
//! and `events` are separate JSON strings. Errors (bad JSON, bad seed, an
//! invalid config, an illegal action) are thrown as JavaScript strings.

use aic_core::{Action, Config, GameState, Rules, SavedGame};
use serde::Serialize;
use wasm_bindgen::prelude::*;

/// The result of `apply`: `state` (JSON string, pass it back as-is) and
/// `events` (JSON array string).
#[wasm_bindgen(getter_with_clone)]
pub struct Applied {
    pub state: String,
    pub events: String,
}

fn to_json<T: Serialize + ?Sized>(value: &T) -> String {
    serde_json::to_string(value).expect("engine types serialize")
}

fn parse<T: for<'de> serde::Deserialize<'de>>(what: &str, json: &str) -> Result<T, JsValue> {
    serde_json::from_str(json).map_err(|e| JsValue::from_str(&format!("bad {what} JSON: {e}")))
}

fn err(e: String) -> JsValue {
    JsValue::from_str(&e)
}

fn rules(config_json: &str) -> Result<Rules, JsValue> {
    Rules::new(&Config::from_json(config_json).map_err(err)?).map_err(err)
}

#[wasm_bindgen]
pub fn version() -> String {
    aic_core::VERSION.to_string()
}

/// The rules revision this engine plays (`aic_core::RULES_REVISION`).
#[wasm_bindgen]
pub fn rules_revision() -> u32 {
    aic_core::RULES_REVISION
}

/// All content as JSON: the map (piece shape, layouts, country pieces),
/// player cards, threats, adversary cards, the adversary's project deck, AI
/// boards and the Unity and Discord decks.
#[wasm_bindgen]
pub fn content() -> String {
    to_json(aic_core::content())
}

/// Validates a config. Returns `{"config": <resolved>, "hash": "<16 hex>"}`.
#[wasm_bindgen]
pub fn resolve_config(config_json: &str) -> Result<String, JsValue> {
    let r = rules(config_json)?;
    Ok(to_json(
        &serde_json::json!({ "config": r.config, "hash": r.config_hash }),
    ))
}

/// The board for a config: hexes (axial `[q, r]`, country, piece position,
/// neighbors, printed number, kind, whether industrial or coastal, and for a
/// military base the country it faces), countries (hex indices, color) and
/// fronts (the two countries and their bases). `state.hexes[i]` is `hexes[i]`.
#[wasm_bindgen]
pub fn board(config_json: &str) -> Result<String, JsValue> {
    Ok(to_json(&rules(config_json)?.map))
}

/// Starts a game. `seed` is a decimal string (any `u64`).
#[wasm_bindgen]
pub fn new_game(config_json: &str, seed: &str) -> Result<String, JsValue> {
    let seed = parse_seed(seed)?;
    Ok(to_json(&rules(config_json)?.new_game(seed)))
}

fn parse_seed(seed: &str) -> Result<u64, JsValue> {
    seed.trim().parse().map_err(|_| {
        JsValue::from_str(&format!("bad seed {seed:?}: expected a decimal u64 string"))
    })
}

/// Starts a game and returns an `Applied` whose `events` are setup's (and
/// the first round's, up to the first decision).
#[wasm_bindgen]
pub fn start(config_json: &str, seed: &str) -> Result<Applied, JsValue> {
    let seed = parse_seed(seed)?;
    let mut events = Vec::new();
    let state = rules(config_json)?.new_game_logged(seed, Some(&mut events));
    Ok(Applied {
        state: to_json(&state),
        events: to_json(&events),
    })
}

/// The generic view of a state (`aic_core::View`): tokens on hexes,
/// per-seat and per-country values, card groups, boards, piles, globals.
#[wasm_bindgen]
pub fn view(state_json: &str) -> Result<String, JsValue> {
    let state: GameState = parse("state", state_json)?;
    Ok(to_json(&aic_core::view(&state).map_err(err)?))
}

/// A game record (`aic_core::SavedGame` text: config + seed + actions,
/// plus identity) for the game that `actions` took to `state`.
#[wasm_bindgen]
pub fn record_text(state_json: &str, actions_json: &str, note: &str) -> Result<String, JsValue> {
    let state: GameState = parse("state", state_json)?;
    let actions: Vec<Action> = parse("actions", actions_json)?;
    let r = Rules::new(&state.config).map_err(err)?;
    Ok(SavedGame::new(&r, state.seed, actions, &state, note).to_text())
}

/// Parses a game record. Returns the record as compact JSON (config, seed
/// as a decimal string, actions, ...) plus `"config_error"` if its config is
/// invalid for this engine. Replay it with `start` + `apply`.
#[wasm_bindgen]
pub fn parse_record(text: &str) -> Result<String, JsValue> {
    let rec = SavedGame::from_json(text).map_err(err)?;
    let mut v = serde_json::to_value(&rec).expect("records serialize");
    if let Err(e) = Rules::new(&rec.config) {
        v["config_error"] = e.into();
    }
    Ok(v.to_string())
}

/// Replays a record natively and returns `{"ok": true, "warnings": [...]}`
/// or `{"ok": false, "error": "..."}`: the same check `aic-sim replay` does.
#[wasm_bindgen]
pub fn check_record(text: &str) -> String {
    let out = SavedGame::from_json(text).and_then(|r| r.replay());
    match out {
        Ok((_, warnings)) => serde_json::json!({ "ok": true, "warnings": warnings }),
        Err(e) => serde_json::json!({ "ok": false, "error": e }),
    }
    .to_string()
}

/// Legal actions for a state, as a JSON array.
#[wasm_bindgen]
pub fn legal_actions(state_json: &str) -> Result<String, JsValue> {
    let state: GameState = parse("state", state_json)?;
    Ok(to_json(&aic_core::legal_actions(&state).map_err(err)?))
}

/// Applies an action. Returns an `Applied` (`.state`, `.events`).
#[wasm_bindgen]
pub fn apply(state_json: &str, action_json: &str) -> Result<Applied, JsValue> {
    let state: GameState = parse("state", state_json)?;
    let action: Action = parse("action", action_json)?;
    let (state, events) = aic_core::apply(&state, &action).map_err(err)?;
    Ok(Applied {
        state: to_json(&state),
        events: to_json(&events),
    })
}
