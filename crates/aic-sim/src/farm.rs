//! The farm (160): a read's games over a seed range, for conductor's remote
//! workers. `aic-sim farm SPEC --seed F --games N --out FILE` natively, and
//! `run(first, count, args)` in the wasm bundle (`crates/aic-farm`), play the
//! same games with this code and write the same lines.
//!
//! A shard's output is its games' lines (`games.jsonl`'s, with no header), seed
//! by seed and each seed's cells in grid order, so shards concatenated in seed
//! order are one run over their whole range, byte for byte. That is why a line's
//! timings are 0 (the native command prints them on stderr) and why a line says
//! which engine played it (`engine`: `native` or `wasm`), so a paired read
//! pairs a seed only within one engine. `--header` prints the header line that
//! makes the concatenated shards a `games.jsonl` the read scripts take.
//!
//! A farmed spec is a file in `experiments/farm/` (its name is the argument),
//! and its configs are files of the repo: the bundle builds them all in, since a
//! browser reads no files.

use crate::cli::{fail, Options};
use crate::experiment::{self, Cell, GameResult, Spec, StreamHeader};

/// The engine a line was played on.
pub const ENGINE: &str = if cfg!(target_arch = "wasm32") { "wasm" } else { "native" };

/// Where a farm finds a file by its path from the repo root: the disk
/// natively, the files built in on wasm.
pub type Read<'a> = &'a dyn Fn(&str) -> Result<String, String>;

/// The folder of farmed specs, from the repo root.
pub const SPECS: &str = "experiments/farm";

/// A spec argument's path: a bare name is `experiments/farm/NAME.json`.
pub fn spec_path(arg: &str) -> String {
    if arg.contains('/') || arg.contains('\\') || arg.ends_with(".json") {
        arg.to_string()
    } else {
        format!("{SPECS}/{arg}.json")
    }
}

/// A spec ready to farm: its cells built.
pub struct Farm {
    pub spec: Spec,
    pub cells: Vec<Cell>,
}

impl Farm {
    /// The spec `arg` names (see [`spec_path`]), its files from `read`.
    pub fn load(arg: &str, read: Read) -> Result<Farm, String> {
        let path = spec_path(arg);
        let text = read(&path).map_err(|e| format!("{path}: {e}"))?;
        let spec: Spec = serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
        Farm::from_spec(spec, read).map_err(|e| format!("{path}: {e}"))
    }

    /// `spec`, refused when its games depend on more than their own seed: a
    /// gate stops on what the other seeds read, and win-probability probes
    /// run on the spec's first seeds, not a shard's.
    pub fn from_spec(spec: Spec, read: Read) -> Result<Farm, String> {
        if spec.gate.is_some() {
            return Err("a gated spec can't be farmed: its blocks depend on each other".into());
        }
        if spec.winprob.games > 0 {
            return Err("winprob probes run on the spec's first seeds: farm with winprob.games 0".into());
        }
        if spec.inject_panic.is_some() {
            return Err("inject_panic is a test's: not in a farmed spec".into());
        }
        let cells = experiment::cells_with(&spec, read)?;
        // A shard is one thread, and wasm has none to spawn: an agent's `threads`
        // dial (the oracle's; the same play at any count) stays at 1.
        if let Some(c) = cells.iter().find(|c| c.agent.params.get("threads").is_some_and(|t| *t > 1.0)) {
            return Err(format!("{}: a farmed agent plays on one thread (threads=1)", c.agent.label()));
        }
        Ok(Farm { spec, cells })
    }

    /// Plays every cell on each seed of `first .. first + count`, seed by
    /// seed, cells in grid order, handing each game to `each`: its result
    /// (timings as measured) and its line (timings 0).
    pub fn play(&self, first: u64, count: u64, each: &mut dyn FnMut(&GameResult, &str)) -> Result<(), String> {
        let end = first
            .checked_add(count)
            .ok_or_else(|| format!("seed {first} + {count} games overflows u64"))?;
        for seed in first..end {
            for ci in 0..self.cells.len() {
                let r = experiment::play_one(&self.spec, &self.cells, ci, seed);
                let l = line(&r, ENGINE);
                each(&r, &l);
            }
        }
        Ok(())
    }

    /// The `games.jsonl` header for this spec over `first .. first + count`.
    pub fn header(&self, first: u64, count: u64) -> String {
        let mut spec = self.spec.clone();
        spec.first_seed = first;
        spec.games = count;
        serde_json::to_string(&StreamHeader::new(&spec, &self.cells)).expect("a header serializes")
    }
}

/// A game's farm line: `games.jsonl`'s, its timings 0 and its engine named
/// (none when `engine` is empty).
pub fn line(r: &GameResult, engine: &str) -> String {
    let mut r = r.clone();
    r.ms = 0.0;
    r.agent_ms = 0.0;
    r.probe_ms = 0.0;
    r.setup_ms = 0.0;
    r.legal_ms = 0.0;
    r.apply_ms = 0.0;
    r.replay_ms = 0.0;
    r.engine = engine.to_string();
    serde_json::to_string(&r).expect("a game result serializes")
}

/// The spec the fingerprint plays: the strategist on the baseline at d2.
const FINGERPRINT_SPEC: &str = r#"{
  "name": "fingerprint",
  "note": "160: the farm's fingerprint: two seeds of the strategist, baseline, d2",
  "configs": ["configs/baseline.json"],
  "variants": [{"name": "bp", "set": []}],
  "difficulties": [2],
  "agents": ["strategist"],
  "games": 2,
  "records": 0
}"#;

/// The seeds the fingerprint plays (screens' dev seeds: any seed would do).
const FINGERPRINT_SEEDS: (u64, u64) = (36800, 2);

/// A short check that a worker computes as this build does: the rules
/// revision, the content hash, and a hash of the fingerprint's game lines
/// with no engine named, so a native and a wasm worker that play alike print
/// the same.
pub fn fingerprint(read: Read) -> Result<String, String> {
    let spec: Spec = serde_json::from_str(FINGERPRINT_SPEC).expect("the fingerprint spec parses");
    let farm = Farm::from_spec(spec, read)?;
    let mut h = Fnv::default();
    let mut games = 0;
    farm.play(FINGERPRINT_SEEDS.0, FINGERPRINT_SEEDS.1, &mut |r, _| {
        h.add(line(r, "").as_bytes());
        h.add(b"\n");
        games += 1;
    })?;
    Ok(format!(
        "aic-farm rules r{} content {} games {games} lines {:016x}",
        aic_core::RULES_REVISION,
        aic_core::content_hash(),
        h.0
    ))
}

/// FNV-1a, 64 bits: a fixed hash with nothing platform-sized in it.
struct Fnv(u64);

impl Default for Fnv {
    fn default() -> Fnv {
        Fnv(0xcbf2_9ce4_8422_2325)
    }
}

impl Fnv {
    fn add(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}

/// A file from the disk, by its path from the working directory (the repo root).
pub fn disk(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

/// `aic-sim farm`.
pub fn cli(o: &Options) -> i32 {
    if o.fingerprint {
        match fingerprint(&disk) {
            Ok(f) => {
                println!("{f}");
                return 0;
            }
            Err(e) => fail(&e),
        }
    }
    let Some(arg) = o.paths.first() else {
        fail("farm takes a spec (a name in experiments/farm/, or a path), or --fingerprint")
    };
    // The range is the shard's, never a default (200 games from seed 0) or the spec's.
    if !o.given.contains("--seed") || !o.given.contains("--games") {
        fail("farm needs --seed F and --games N: the shard's seed range");
    }
    let farm = Farm::load(arg, &disk).unwrap_or_else(|e| fail(&e));
    if o.header {
        println!("{}", farm.header(o.seed, o.games));
        return 0;
    }
    let Some(out) = &o.out else { fail("farm needs --out FILE (or --header)") };
    let start = crate::clock::Instant::now();
    let mut text = String::new();
    let mut games = 0usize;
    let mut game_ms = 0.0;
    farm.play(o.seed, o.games, &mut |r, l| {
        text.push_str(l);
        text.push('\n');
        games += 1;
        game_ms += r.ms;
        eprintln!("farm: cell {} seed {} {:.1} ms", r.cell, r.seed, r.ms);
    })
    .unwrap_or_else(|e| fail(&e));
    // The whole shard or nothing: a part file renamed, so a killed shard leaves
    // no output to mistake for a finished one.
    let part = format!("{out}.part");
    std::fs::write(&part, &text)
        .and_then(|_| std::fs::rename(&part, out))
        .unwrap_or_else(|e| fail(&format!("{out}: {e}")));
    let wall = start.elapsed().as_secs_f64();
    eprintln!(
        "farm: {} {games} games, seeds {}..{}, {wall:.1} s wall, {:.1} ms a game ({ENGINE})",
        farm.spec.name,
        o.seed,
        o.seed + o.games - 1,
        game_ms / games.max(1) as f64
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny() -> Farm {
        let spec: Spec = serde_json::from_str(
            r#"{"name": "tiny", "configs": ["../../configs/baseline.json"],
                "variants": [{"name": "bp", "set": []}, {"name": "br", "set": ["ai.boards=0,2"]}],
                "difficulties": [2], "agents": ["random"], "games": 1, "records": 0}"#,
        )
        .unwrap();
        Farm::from_spec(spec, &disk).unwrap()
    }

    fn shard(f: &Farm, first: u64, count: u64) -> String {
        let mut s = String::new();
        f.play(first, count, &mut |_, l| {
            s.push_str(l);
            s.push('\n');
        })
        .unwrap();
        s
    }

    /// Two halves concatenated are the whole range, byte for byte.
    #[test]
    fn halves_concatenate_to_the_whole() {
        let f = tiny();
        let whole = shard(&f, 500, 4);
        assert_eq!(shard(&f, 500, 1) + &shard(&f, 501, 3), whole);
        assert_eq!(whole.lines().count(), 8);
        let g: serde_json::Value = serde_json::from_str(whole.lines().next().unwrap()).unwrap();
        assert_eq!(g["engine"], "native");
        assert_eq!(g["ms"], 0.0);
    }

    #[test]
    fn a_threaded_agent_is_refused() {
        let spec = |agent: &str| -> Spec {
            serde_json::from_str(&format!(
                r#"{{"name": "t", "configs": ["../../configs/baseline.json"], "agents": ["{agent}"], "games": 1}}"#
            ))
            .unwrap()
        };
        assert!(Farm::from_spec(spec("oracle:threads=2"), &disk).is_err());
        assert!(Farm::from_spec(spec("oracle"), &disk).is_ok());
    }

    #[test]
    fn spec_names_resolve_in_the_farm_folder() {
        assert_eq!(spec_path("read"), "experiments/farm/read.json");
        assert_eq!(spec_path("out/x.json"), "out/x.json");
    }
}
