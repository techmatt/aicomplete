//! The farm's wasm bundle (160): conductor's remote workers play a read's
//! games in a browser tab with `aic_sim::farm`, the code the native
//! `aic-sim farm` runs. No rules and no sim code live here.
//!
//! JS sees (through `farm.mjs`, which `scripts/farm/build.mjs` writes beside
//! the wasm): `init()`, `fingerprint() -> string` and
//! `run(first, count, args) -> string`, a shard's game lines. Errors are thrown.

use aic_sim::farm::{self, Farm};
use wasm_bindgen::prelude::*;

/// Every config and farmed spec, by its path from the repo root (`build.rs`).
static FILES: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/files.rs"));

/// A file built into the bundle.
pub fn read(path: &str) -> Result<String, String> {
    let path = path.trim_start_matches("./").replace('\\', "/");
    FILES
        .iter()
        .find(|(p, _)| *p == path)
        .map(|(_, text)| text.to_string())
        .ok_or_else(|| format!("{path} is not built into the bundle (configs/, experiments/configs/, experiments/farm/)"))
}

/// A seed or count from JS: a whole number a double holds exactly.
fn whole(what: &str, v: f64) -> Result<u64, String> {
    if v.fract() != 0.0 || !(0.0..=9_007_199_254_740_991.0).contains(&v) {
        return Err(format!("{what} must be a whole number from 0 to 2^53 - 1, not {v}"));
    }
    Ok(v as u64)
}

/// The shard's lines: every cell of the spec `args` names, on seeds
/// `first .. first + count`.
pub fn shard(first: f64, count: f64, args: &str) -> Result<String, String> {
    let (first, count) = (whole("first", first)?, whole("count", count)?);
    let words: Vec<&str> = args.split_whitespace().collect();
    let [spec] = words[..] else {
        return Err(format!("args must be one spec name, not {args:?}"));
    };
    let f = Farm::load(spec, &read)?;
    let mut out = String::new();
    f.play(first, count, &mut |_, l| {
        out.push_str(l);
        out.push('\n');
    })?;
    Ok(out)
}

/// The worker's fingerprint (`aic-sim farm --fingerprint`'s line).
#[wasm_bindgen]
pub fn fingerprint() -> Result<String, JsError> {
    farm::fingerprint(&read).map_err(|e| JsError::new(&e))
}

/// A shard's output: its game lines, seed by seed.
#[wasm_bindgen]
pub fn run(first: f64, count: f64, args: &str) -> Result<String, JsError> {
    shard(first, count, args).map_err(|e| JsError::new(&e))
}

/// The engine this build plays as (`native` or `wasm`).
#[wasm_bindgen]
pub fn engine() -> String {
    farm::ENGINE.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bundle_holds_the_configs_and_farm_specs() {
        assert!(read("configs/baseline.json").unwrap().contains("modules"));
        assert!(read("./configs/baseline.json").is_ok());
        assert!(read("experiments/farm/read.json").is_ok());
        assert!(read("nope.json").is_err());
    }

    #[test]
    fn args_and_seeds_are_checked() {
        assert!(shard(1.5, 1.0, "read").is_err());
        assert!(shard(1.0, 1.0, "read extra").is_err());
        assert!(shard(1.0, 1.0, "").is_err());
    }
}
