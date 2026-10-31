//! Runs the built binary's `experiment` command on a tiny grid: it writes
//! the report, the machine-readable results and records that replay, and
//! its results do not depend on the thread count.

mod common;

use std::path::{Path, PathBuf};

use aic_core::SavedGame;
use common::{fresh_dir, sim};

/// `aic-sim experiment <spec> --out <out> <extra>`.
fn experiment(spec: &Path, out: &Path, extra: &[&str]) -> std::process::Output {
    sim()
        .arg("experiment")
        .arg(spec)
        .arg("--out")
        .arg(out)
        .args(extra)
        .output()
        .expect("runs aic-sim")
}

const SPEC: &str = r#"{
  "name": "test-grid",
  "configs": ["configs/baseline.json"],
  "difficulties": [0, 4],
  "agents": ["random", "greedy"],
  "games": 3,
  "winprob": {"games": 1, "rollouts": 2, "agent": "random"},
  "closeness": {"every": 1},
  "records": 2
}"#;

/// One CSV line's fields; quoted fields may hold commas and doubled quotes.
fn fields(line: &str) -> Vec<String> {
    let (mut out, mut cur, mut quoted) = (Vec::new(), String::new(), false);
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                cur.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// games.csv without the timing column, which varies run to run.
fn results_without_timing(dir: &Path) -> Vec<Vec<String>> {
    let text = std::fs::read_to_string(dir.join("games.csv")).unwrap();
    let head = fields(text.lines().next().unwrap());
    let ms = head.iter().position(|h| h == "ms").unwrap();
    text.lines()
        .skip(1)
        .map(|l| {
            let mut f = fields(l);
            assert_eq!(f.len(), head.len(), "{l}");
            f.remove(ms);
            f
        })
        .collect()
}

/// Fast lane: the grid with random only and a closeness probe every fourth
/// choice (2–4 s; greedy games are seconds each even optimized).
#[test]
fn experiment_writes_report_and_records_random() {
    let body = SPEC
        .replace(r#"["random", "greedy"]"#, r#"["random"]"#)
        .replace(
            r#""closeness": {"every": 1}"#,
            r#""closeness": {"every": 4}"#,
        );
    writes_report_and_records("aic-sim-experiment-random", &body, 2);
}

/// Slow lane: the whole grid, greedy included, every choice probed.
#[test]
#[ignore = "slow lane: greedy experiment games, twice"]
fn experiment_writes_report_and_records() {
    writes_report_and_records("aic-sim-experiment", SPEC, 4);
}

fn writes_report_and_records(dir: &str, body: &str, cells: usize) {
    let root = fresh_dir(dir, true);
    let spec = root.join("spec.json");
    std::fs::write(&spec, body).unwrap();
    // Both runs at once: the one-thread run is the long one.
    let runs = std::thread::scope(|s| {
        ["1", "3"]
            .map(|threads| {
                let out = root.join(format!("out-{threads}"));
                let spec = &spec;
                s.spawn(move || {
                    let res = experiment(spec, &out, &["--threads", threads]);
                    (out, res)
                })
            })
            .map(|h| h.join().unwrap())
    });
    let mut outs = Vec::new();
    for (out, res) in runs {
        assert!(
            res.status.success(),
            "experiment failed: {}",
            String::from_utf8_lossy(&res.stderr)
        );
        for f in ["report.md", "results.json", "games.csv", "cards.csv"] {
            assert!(out.join(f).exists(), "{f} missing");
        }
        let report = std::fs::read_to_string(out.join("report.md")).unwrap();
        for section in [
            "## Findings",
            "## Win rate by skill tier",
            "## Luck share",
            "## Pick and play rates",
            "## Timing",
            "## Saved records",
        ] {
            assert!(report.contains(section), "report lacks {section}");
        }
        let results: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(out.join("results.json")).unwrap())
                .unwrap();
        assert_eq!(results["cells"].as_array().unwrap().len(), cells);
        assert_eq!(results["format"], "aic-experiment/2");
        // 3 games per cell, one row each.
        assert_eq!(
            std::fs::read_to_string(out.join("games.csv"))
                .unwrap()
                .lines()
                .count(),
            3 * cells + 1
        );
        let records: Vec<PathBuf> = std::fs::read_dir(out.join("records"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        assert!(!records.is_empty(), "no records saved");
        for p in &records {
            let rec = SavedGame::from_json(&std::fs::read_to_string(p).unwrap()).unwrap();
            let (_, warnings) = rec.replay().unwrap();
            assert!(warnings.is_empty(), "{}: {warnings:?}", p.display());
            assert!(rec.note.contains("aic-sim experiment test-grid"));
        }
        outs.push(out);
    }
    assert_eq!(
        results_without_timing(&outs[0]),
        results_without_timing(&outs[1])
    );
}

const VARIANT_SPEC: &str = r#"{
  "name": "test-variants",
  "configs": ["configs/baseline.json"],
  "variants": [
    {"name": "base"},
    {"name": "threats_off", "set": ["threats.variant=off"]}
  ],
  "difficulties": [0],
  "agents": ["random", "greedy"],
  "games": 3,
  "closeness": {"every": 0},
  "records": 0
}"#;

/// Two agents and two variants, one switching a module off: the
/// comparison sections appear, with their machine-readable twins.
#[test]
fn experiment_reports_comparisons() {
    let root = fresh_dir("aic-sim-experiment-variants", true);
    let spec = root.join("spec.json");
    std::fs::write(&spec, VARIANT_SPEC).unwrap();
    let out = root.join("out");
    let res = experiment(&spec, &out, &[]);
    assert!(
        res.status.success(),
        "experiment failed: {}",
        String::from_utf8_lossy(&res.stderr)
    );
    let report = std::fs::read_to_string(out.join("report.md")).unwrap();
    for section in [
        "## Strategy health",
        "## Head to head against greedy",
        "## Variants against the base",
        "## Removing a module",
        "Strategy health at difficulty 0",
        // greedy wins most here, so it is the top agent itself.
        "Top agent (greedy",
        "What a module is worth",
        "| threats (threats_off) |",
    ] {
        assert!(report.contains(section), "report lacks {section}\n{report}");
    }
    let results: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("results.json")).unwrap()).unwrap();
    // random against greedy, in each of the two variants.
    let h2h = results["head_to_head"].as_array().unwrap();
    assert_eq!(h2h.len(), 2);
    for h in h2h {
        assert_eq!(h["agent"], "random");
        assert_eq!(h["greedy"], "greedy");
        assert_eq!(h["paired"]["games"], 3);
    }
    assert_eq!(results["variants"].as_array().unwrap().len(), 2);
    let removals = results["removals"].as_array().unwrap();
    assert_eq!(removals.len(), 2);
    assert_eq!(removals[0]["modules"][0], "threats");
    assert_eq!(results["top_agent"]["agent"], "greedy");
    for c in results["cells"].as_array().unwrap() {
        assert!(c["buildings_played_per_game"].as_f64().unwrap() >= 0.0);
        assert!(c["tier2_played_per_game"].is_number());
    }
}

/// One agent and one variant: no comparison sections.
#[test]
fn experiment_omits_comparisons_that_do_not_apply() {
    let root = fresh_dir("aic-sim-experiment-single", true);
    let spec = root.join("spec.json");
    let body = VARIANT_SPEC
        .replace(
            r#"{"name": "threats_off", "set": ["threats.variant=off"]}"#,
            r#"{"name": "threats_on", "set": []}"#,
        )
        .replace(r#""random", "greedy""#, r#""random""#);
    std::fs::write(&spec, body).unwrap();
    let out = root.join("out");
    let res = experiment(&spec, &out, &[]);
    assert!(
        res.status.success(),
        "{}",
        String::from_utf8_lossy(&res.stderr)
    );
    let report = std::fs::read_to_string(out.join("report.md")).unwrap();
    assert!(report.contains("## Strategy health"));
    assert!(report.contains("## Variants against the base"));
    assert!(!report.contains("## Head to head against greedy"));
    assert!(!report.contains("## Removing a module"));
}

/// The two-stage pattern (011): `--variants` keeps the base and the named
/// variants; `--winprob` and `--closeness` turn the probes on for that run.
/// An unknown variant is an argument error.
#[test]
fn experiment_filters_variants_and_turns_probes_on() {
    let root = fresh_dir("aic-sim-experiment-two-stage", true);
    let spec = root.join("spec.json");
    let body = r#"{
  "name": "two-stage",
  "configs": ["configs/baseline.json"],
  "variants": [
    {"name": "base", "set": []},
    {"name": "keep0", "set": ["cards.keep_tier1=0"]},
    {"name": "keep2", "set": ["cards.keep_tier1=2"]}
  ],
  "agents": ["random"],
  "games": 2,
  "winprob": {"games": 0, "rollouts": 1, "agent": "random"},
  "records": 0
}"#;
    std::fs::write(&spec, body).unwrap();
    let out = root.join("out");
    let res = experiment(
        &spec,
        &out,
        &["--variants", "keep2", "--winprob", "1", "--closeness", "2"],
    );
    assert!(
        res.status.success(),
        "{}",
        String::from_utf8_lossy(&res.stderr)
    );
    let results: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("results.json")).unwrap()).unwrap();
    let names: Vec<&str> = results["spec"]["variants"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["base", "keep2"]);
    assert_eq!(results["spec"]["winprob"]["games"], 1);
    assert_eq!(results["spec"]["closeness"]["every"], 2);
    // Without the flags the spec's defaults hold: both probes off.
    let res = experiment(&spec, &root.join("out2"), &[]);
    assert!(res.status.success());
    let results: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("out2/results.json")).unwrap())
            .unwrap();
    assert_eq!(results["spec"]["winprob"]["games"], 0);
    assert_eq!(results["spec"]["closeness"]["every"], 0);
    let report = std::fs::read_to_string(root.join("out2/report.md")).unwrap();
    assert!(report.contains("Off measures are opt-in"), "{report}");
    let res = sim()
        .arg("experiment")
        .arg(&spec)
        .args(["--variants", "nope"])
        .output()
        .unwrap();
    assert_eq!(res.status.code(), Some(2));
}

#[test]
fn experiment_rejects_bad_specs() {
    let root = fresh_dir("aic-sim-experiment-bad", true);
    for (name, body) in [
        ("agent", SPEC.replace("\"greedy\"", "\"greedy:nope=1\"")),
        ("field", SPEC.replace("\"records\": 2", "\"recordz\": 2")),
        ("config", SPEC.replace("baseline.json", "missing.json")),
    ] {
        let spec = root.join(format!("{name}.json"));
        std::fs::write(&spec, body).unwrap();
        let res = experiment(&spec, &root.join("out"), &[]);
        assert_eq!(
            res.status.code(),
            Some(2),
            "{name}: {}",
            String::from_utf8_lossy(&res.stderr)
        );
    }
}

const STREAM_SPEC: &str = r#"{
  "name": "test-stream",
  "configs": ["configs/baseline.json"],
  "difficulties": [0, 4],
  "agents": ["random"],
  "games": 6,
  "records": 1
}"#;

/// Every game is streamed to games.jsonl as it finishes; a run killed
/// mid-write (its last line cut short) resumes with only the missing games
/// and reports the same as an unbroken run; another spec can't resume it.
#[test]
fn experiment_streams_games_and_resumes() {
    let root = fresh_dir("aic-sim-experiment-stream", true);
    let spec = root.join("spec.json");
    std::fs::write(&spec, STREAM_SPEC).unwrap();
    let whole = root.join("whole");
    let res = experiment(&spec, &whole, &[]);
    assert!(
        res.status.success(),
        "{}",
        String::from_utf8_lossy(&res.stderr)
    );
    let stream = std::fs::read_to_string(whole.join("games.jsonl")).unwrap();
    let lines: Vec<&str> = stream.lines().collect();
    assert_eq!(lines.len(), 1 + 12, "a header, then one line per game");
    assert!(
        lines[0].contains("\"format\":\"aic-games/2\""),
        "{}",
        lines[0]
    );

    // As a kill leaves it: five games, and half of a sixth.
    let cut = root.join("cut");
    std::fs::create_dir_all(&cut).unwrap();
    let mut text = lines[..6].join("\n");
    text.push('\n');
    text.push_str(&lines[6][..lines[6].len() / 2]);
    std::fs::write(cut.join("games.jsonl"), text).unwrap();
    let res = experiment(&spec, &cut, &["--resume", "--threads", "2"]);
    let err = String::from_utf8_lossy(&res.stderr);
    assert!(res.status.success(), "{err}");
    assert!(err.contains("(5 already played)"), "{err}");
    assert_eq!(results_without_timing(&whole), results_without_timing(&cut));
    let resumed = std::fs::read_to_string(cut.join("games.jsonl")).unwrap();
    assert_eq!(resumed.lines().count(), 1 + 12);
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(cut.join("results.json")).unwrap()).unwrap();
    assert_eq!(json["games_resumed"], 5);
    assert_eq!(json["games_played"], json["games_planned"]);
    assert!(!std::fs::read_to_string(cut.join("report.md"))
        .unwrap()
        .contains("Partial"));

    // A different grid is not the same experiment.
    let res = experiment(&spec, &cut, &["--resume", "--games", "7"]);
    assert_eq!(res.status.code(), Some(2));
    // 020: and it says the spec (here --games) is what differs.
    let err = String::from_utf8_lossy(&res.stderr).to_string();
    assert!(err.contains("another run") && err.contains("the spec"), "{err}");

    // Nor is the same spec over an edited config file.
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/baseline.json");
    let cfg = root.join("cfg.json");
    std::fs::copy(&base, &cfg).unwrap();
    let cfg_path = cfg.to_string_lossy().replace('\\', "/");
    let spec2 = root.join("spec2.json");
    std::fs::write(
        &spec2,
        STREAM_SPEC.replace("configs/baseline.json", &cfg_path),
    )
    .unwrap();
    let edited = root.join("edited");
    assert!(experiment(&spec2, &edited, &[]).status.success());
    let mut c: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&cfg).unwrap()).unwrap();
    c["countries"] = serde_json::json!(3);
    std::fs::write(&cfg, c.to_string()).unwrap();
    let res = experiment(&spec2, &edited, &["--resume"]);
    assert_eq!(
        res.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&res.stderr)
    );
    // A fresh run keeps the earlier stream aside.
    assert!(experiment(&spec2, &edited, &[]).status.success());
    assert!(edited.join("games.jsonl.bak").exists());
}

/// `--quick`: deep-search agents are dropped, seeds capped at 16 and the
/// probes turned off; the shallow agents stay.
#[test]
fn experiment_quick_keeps_shallow_agents() {
    let root = fresh_dir("aic-sim-experiment-quick", true);
    let spec = root.join("spec.json");
    let body = STREAM_SPEC
        .replace(
            r#""agents": ["random"]"#,
            r#""agents": ["random", "beam:width=2", "greedy:samples=2", "engine:width=2"]"#,
        )
        .replace(
            r#""games": 6"#,
            r#""games": 40, "winprob": {"games": 2}, "closeness": {"every": 1}"#,
        )
        .replace(r#""difficulties": [0, 4]"#, r#""difficulties": [0]"#);
    std::fs::write(&spec, body).unwrap();
    let out = root.join("out");
    let res = experiment(&spec, &out, &["--quick"]);
    let err = String::from_utf8_lossy(&res.stderr);
    assert!(res.status.success(), "{err}");
    assert!(
        err.contains("dropped deep-search agents beam:width=2, greedy:samples=2, engine:width=2"),
        "{err}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("results.json")).unwrap()).unwrap();
    assert_eq!(json["spec"]["agents"], serde_json::json!(["random"]));
    assert_eq!(json["spec"]["games"], 16);
    assert_eq!(json["spec"]["winprob"]["games"], 0);
    assert_eq!(json["spec"]["closeness"]["every"], 0);
}

/// A run killed partway leaves readable results: the games so far in
/// games.jsonl and, after `--report-every`, a report marked partial.
#[test]
#[ignore = "slow lane: greedy games, killed partway"]
fn experiment_killed_partway_leaves_a_partial_report() {
    let root = fresh_dir("aic-sim-experiment-killed", true);
    let spec = root.join("spec.json");
    let body = STREAM_SPEC
        .replace(r#""agents": ["random"]"#, r#""agents": ["greedy"]"#)
        .replace(r#""games": 6"#, r#""games": 200"#);
    std::fs::write(&spec, body).unwrap();
    let out = root.join("out");
    let mut child = sim()
        .arg("experiment")
        .arg(&spec)
        .arg("--out")
        .arg(&out)
        .args(["--report-every", "1", "--threads", "2"])
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let report = out.join("report.md");
    let start = std::time::Instant::now();
    while !report.exists() {
        assert!(start.elapsed().as_secs() < 120, "no partial report");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    child.kill().unwrap();
    child.wait().unwrap();
    let text = std::fs::read_to_string(&report).unwrap();
    assert!(
        text.contains("**Partial: "),
        "{}",
        &text[..300.min(text.len())]
    );
    assert!(text.contains("of 400 games.**"));
    let games = std::fs::read_to_string(out.join("games.jsonl")).unwrap();
    assert!(games.lines().count() >= 2, "games streamed");
    assert!(
        std::fs::read_to_string(out.join("games.csv"))
            .unwrap()
            .lines()
            .count()
            >= 2
    );
}

/// 046: a panicking game killed a gate 25 minutes in. One now fails alone: the
/// gate plays on to its end, lists the game, and exits 1.
#[test]
fn a_gate_with_a_panicking_game_finishes_and_lists_it() {
    let dir = fresh_dir("panic-gate", true);
    let spec = dir.join("spec.json");
    std::fs::write(
        &spec,
        r#"{"name": "panic-gate", "configs": ["configs/baseline.json"], "difficulties": [2],
            "agents": ["greedy", "greedy:no_counters=1"], "games": 4, "first_seed": 50, "records": 0,
            "gate": {"z": 2.0, "min_games": 2, "every": 2},
            "inject_panic": {"cell": 1, "seed": 51}}"#,
    )
    .unwrap();
    let out = dir.join("out");
    let run = experiment(&spec, &out, &["--threads", "1"]);
    let stderr = String::from_utf8_lossy(&run.stderr);
    assert_eq!(run.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("1 game(s) PANICKED"), "{stderr}");

    let failed = std::fs::read_to_string(out.join("failed.jsonl")).unwrap();
    let f: serde_json::Value = serde_json::from_str(failed.trim()).unwrap();
    assert_eq!(f["seed"], "51");
    assert_eq!(f["config"], "configs/baseline.json");
    assert_eq!(f["agent"], "greedy:no_counters=1");
    assert!(f["message"].as_str().unwrap().contains("injected panic"), "{f}");

    let report = std::fs::read_to_string(out.join("report.md")).unwrap();
    assert!(report.contains("## Failed games"), "{report}");
    assert!(report.contains("1 game(s) panicked, see *Failed games*"), "{report}");
    assert!(!report.contains("**Partial: "), "a finished run isn't partial: {report}");
    let results: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("results.json")).unwrap()).unwrap();
    assert_eq!(results["games_failed"].as_array().unwrap().len(), 1);
    assert_eq!(results["gate"]["finished"], true);
    assert_eq!(results["gate"]["passes"], false);
    let played = std::fs::read_to_string(out.join("games.jsonl")).unwrap().lines().count() - 1;
    assert_eq!(played, results["games_played"].as_u64().unwrap() as usize);

    // --resume plays it again, and it panics again: listed afresh, not twice.
    let again = experiment(&spec, &out, &["--threads", "1", "--resume"]);
    assert_eq!(again.status.code(), Some(1));
    let failed = std::fs::read_to_string(out.join("failed.jsonl")).unwrap();
    assert_eq!(failed.lines().count(), 1);
}
