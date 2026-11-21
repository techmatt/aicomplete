//! 283: the test lanes' front. `cargo test-fast`, `cargo test-all` and `cargo test-slow` (`.cargo/config.toml`)
//! run this. As plain aliases (`test --workspace -- --include-ignored`) every word typed after the alias went to
//! the test binaries, so `cargo test-all -j 3` stopped in a second on libtest's "Unrecognized option: 'j'" (it
//! read as a pass), and `--no-fail-fast` the same. Here cargo's own flags go to cargo; anything else
//! (`--test-threads=N`, a name filter, `--nocapture`) and everything after a `--` goes to the tests. No
//! dependencies, so it builds in a second.
//!
//! 310: cargo runs the test binaries one after another, so a lane took the sum of each binary's slowest test (the
//! full run: aic-sim's lib 26 s, then experiment 8 s, then aic-core 7 s, ...). This builds them with
//! `cargo test --workspace --no-run [CARGO FLAGS]`, reads the binaries from cargo's JSON, and runs them all at once,
//! each with `[--include-ignored|--ignored] [TEST ARGS]` from its package's folder (as cargo does), printing each
//! one's output whole as it ends and a summary last. Nothing is shared between binaries: each test's scratch
//! folder has its own name (`tests/common/mod.rs`'s `fresh_dir`). The binary slowest in the lane's last unfiltered
//! run keeps this process's priority and the rest start a class lower, so under the full load its longest test (the
//! rollout's whole game) keeps its cores: full run 34 -> 30 s.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::time::Instant;

/// Cargo flags that take a value (`-j 3`, `--jobs=3` and `-j3` are all read).
/// `--test NAME` and `--bench NAME` are cargo's here (a test target); libtest's own `--test` is the default.
const WITH_VALUE: &[&str] = &[
    "-j", "--jobs", "-p", "--package", "-F", "--features", "--target", "--target-dir", "--profile", "--exclude",
    "--test", "--bench", "--bin", "--example",
];
/// Cargo flags that stand alone.
const ALONE: &[&str] = &[
    "--no-fail-fast", "--release", "--offline", "--locked", "--frozen", "-q", "--quiet", "-v", "--verbose",
    "--all-features", "--no-default-features", "--lib", "--bins", "--tests", "--benches", "--examples", "--doc",
    "--all-targets", "--no-run", "--workspace",
];

fn main() {
    let mut args = std::env::args().skip(1);
    let lane: &[&str] = match args.next().as_deref() {
        Some("fast") => &[],
        Some("all") => &["--include-ignored"],
        Some("slow") => &["--ignored"],
        other => {
            eprintln!("aic-lanes: the first word is the lane, fast, all or slow (not {other:?}); run it as cargo test-fast, cargo test-all or cargo test-slow");
            exit(2);
        }
    };
    let (cargo, mut tests) = split(args.collect());
    if cargo.iter().any(|a| a == "-q" || a == "--quiet") {
        tests.insert(0, "-q".into());
    }
    let package = cargo.iter().any(|a| {
        a == "-p" || a == "--package" || a == "--workspace" || a.starts_with("--package=") || (a.starts_with("-p") && a.len() > 2)
    });
    let mut cmd = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
    cmd.arg("test");
    if !package {
        cmd.arg("--workspace");
    }
    cmd.args(cargo.iter().filter(|a| *a != "--no-fail-fast"))
        .args(["--no-run", "--message-format=json-render-diagnostics"])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    let built = cmd.output().unwrap_or_else(|e| {
        eprintln!("aic-lanes: could not run cargo: {e}");
        exit(2)
    });
    if !built.status.success() {
        exit(built.status.code().unwrap_or(1));
    }
    let bins = test_binaries(&String::from_utf8_lossy(&built.stdout));
    if cargo.iter().any(|a| a == "--no-run") {
        return;
    }
    // The binary that took longest last time (this lane, no filter) keeps this process's priority, the rest run a
    // class lower: under the full load its longest test keeps its cores (310, see `times_path`).
    let lane_word = match lane.first().copied() {
        Some("--include-ignored") => "all",
        Some(_) => "slow",
        None => "fast",
    };
    let unfiltered = tests.iter().all(|t| t == "-q" || t.starts_with("--test-threads"));
    let mut times = load_times();
    let slowest = bins
        .iter()
        .map(|b| times.get(&format!("{lane_word} {}", b.name)).copied().unwrap_or(0.0))
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .filter(|(_, t)| *t > 0.0)
        .map(|(i, _)| i);
    let start = Instant::now();
    let runs = std::thread::scope(|s| {
        let handles: Vec<_> = bins
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let args: Vec<&str> = lane.iter().copied().chain(tests.iter().map(String::as_str)).collect();
                let lower = slowest.is_some_and(|k| k != i);
                s.spawn(move || run(b, &args, lower))
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>()
    });
    if unfiltered {
        for r in &runs {
            times.insert(format!("{lane_word} {}", r.name), r.secs);
        }
        save_times(&times);
    }
    let failed: Vec<&Run> = runs.iter().filter(|r| !r.ok).collect();
    let [passed, fails, ignored] = runs.iter().fold([0; 3], |t, r| [t[0] + r.counts[0], t[1] + r.counts[1], t[2] + r.counts[2]]);
    println!(
        "\naic-lanes: {} test binaries side by side in {:.1} s: {passed} passed, {fails} failed, {ignored} ignored",
        runs.len(),
        start.elapsed().as_secs_f64()
    );
    if !failed.is_empty() {
        println!("aic-lanes: FAILED: {}", failed.iter().map(|r| r.name.as_str()).collect::<Vec<_>>().join(", "));
        exit(101);
    }
}

/// A test binary cargo built: its target's name, the file, and its package's folder (the tests' working directory).
struct Bin {
    name: String,
    exe: PathBuf,
    dir: PathBuf,
}

struct Run {
    name: String,
    ok: bool,
    /// passed, failed, ignored, from libtest's `test result:` line(s).
    counts: [u64; 3],
    secs: f64,
}

/// Runs one binary (`lower`: a priority class below this process's), then prints its output whole (one lock, so
/// binaries' outputs never interleave).
fn run(b: &Bin, args: &[&str], lower: bool) -> Run {
    let t = Instant::now();
    let mut cmd = Command::new(&b.exe);
    cmd.args(args).current_dir(&b.dir).env("CARGO_MANIFEST_DIR", &b.dir);
    if lower {
        priority::lower(&mut cmd);
    }
    let out = cmd.output();
    let secs = t.elapsed().as_secs_f64();
    let mut o = std::io::stdout().lock();
    let _ = writeln!(o, "     Running {} ({})", b.name, b.exe.display());
    let Ok(out) = out else {
        let _ = writeln!(o, "aic-lanes: could not run {}", b.exe.display());
        return Run { name: b.name.clone(), ok: false, counts: [0, 1, 0], secs };
    };
    let text = String::from_utf8_lossy(&out.stdout);
    let _ = o.write_all(text.as_bytes());
    let _ = o.write_all(&out.stderr);
    let _ = writeln!(o, "     ({}: {secs:.1} s)", b.name);
    let mut counts = [0; 3];
    for line in text.lines().filter(|l| l.starts_with("test result:")) {
        for (i, word) in ["passed", "failed", "ignored"].iter().enumerate() {
            counts[i] += count_before(line, word);
        }
    }
    Run { name: b.name.clone(), ok: out.status.success(), counts, secs }
}

/// Each binary's wall time in its last unfiltered run, `<lane> <binary>\t<seconds>` a line, beside this exe
/// (`target/debug/aic-lanes-times.txt`). Only a priority hint: a missing or stale file changes no result.
fn times_path() -> Option<PathBuf> {
    Some(std::env::current_exe().ok()?.parent()?.join("aic-lanes-times.txt"))
}

fn load_times() -> std::collections::BTreeMap<String, f64> {
    let text = times_path().and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_default();
    text.lines()
        .filter_map(|l| l.split_once('\t'))
        .filter_map(|(k, v)| Some((k.to_string(), v.parse().ok()?)))
        .collect()
}

fn save_times(times: &std::collections::BTreeMap<String, f64>) {
    if let Some(p) = times_path() {
        let text: String = times.iter().map(|(k, v)| format!("{k}\t{v:.2}\n")).collect();
        let _ = std::fs::write(p, text);
    }
}

#[cfg(windows)]
mod priority {
    use std::os::windows::process::CommandExt;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetCurrentProcess() -> isize;
        fn GetPriorityClass(process: isize) -> u32;
    }

    /// Starts `cmd` a priority class below this process's (normal -> below normal -> idle).
    pub fn lower(cmd: &mut std::process::Command) {
        // SAFETY: GetCurrentProcess returns a pseudo-handle that needs no closing; GetPriorityClass only reads it.
        let class = unsafe { GetPriorityClass(GetCurrentProcess()) };
        let below = match class {
            0x80 => 0x8000,   // high -> above normal
            0x8000 => 0x20,   // above normal -> normal
            0x20 => 0x4000,   // normal -> below normal
            _ => 0x40,        // below normal (or unknown) -> idle
        };
        cmd.creation_flags(below);
    }
}

#[cfg(not(windows))]
mod priority {
    pub fn lower(_: &mut std::process::Command) {}
}

/// The number just before ` word` in `line` ("409 passed;" -> 409).
fn count_before(line: &str, word: &str) -> u64 {
    line.find(&format!(" {word}"))
        .and_then(|at| line[..at].rsplit(' ').next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// The test binaries in cargo's `--message-format=json` lines: artifacts built with the test profile flag and an
/// executable.
fn test_binaries(json: &str) -> Vec<Bin> {
    let mut bins = Vec::new();
    for line in json.lines().filter(|l| l.contains(r#""reason":"compiler-artifact""#)) {
        let Some(exe) = string_after(line, r#""executable":"#) else { continue };
        let test = line.find(r#""profile":"#).is_some_and(|at| line[at..].contains(r#""test":true"#));
        if !test {
            continue;
        }
        let manifest = string_after(line, r#""manifest_path":"#).unwrap_or_default();
        let name = line
            .find(r#""target":{"#)
            .and_then(|at| string_after(&line[at..], r#""name":"#))
            .unwrap_or_default();
        let dir = Path::new(&manifest).parent().map(Path::to_path_buf).unwrap_or_default();
        bins.push(Bin { name, exe: PathBuf::from(exe), dir });
    }
    bins
}

/// The JSON string right after the first `key` in `line` (`None` for a `null`), escapes undone.
fn string_after(line: &str, key: &str) -> Option<String> {
    let rest = &line[line.find(key)? + key.len()..];
    let mut chars = rest.strip_prefix('"')?.chars();
    let mut s = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(s),
            '\\' => match chars.next()? {
                'n' => s.push('\n'),
                't' => s.push('\t'),
                'u' => {
                    let hex: String = chars.by_ref().take(4).collect();
                    s.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
                }
                other => s.push(other),
            },
            _ => s.push(c),
        }
    }
    None
}

/// (cargo's flags, the tests' words), in the order typed.
fn split(words: Vec<String>) -> (Vec<String>, Vec<String>) {
    let (mut cargo, mut tests) = (Vec::new(), Vec::new());
    let mut it = words.into_iter();
    while let Some(a) = it.next() {
        if a == "--" {
            tests.extend(it.by_ref());
            break;
        }
        if WITH_VALUE.contains(&a.as_str()) {
            cargo.push(a);
            match it.next() {
                Some(v) => cargo.push(v),
                None => {
                    eprintln!("aic-lanes: {} takes a value", cargo.last().unwrap());
                    exit(2);
                }
            }
        } else if ALONE.contains(&a.as_str())
            || WITH_VALUE.iter().any(|f| a.starts_with(&format!("{f}=")))
            || (a.starts_with("-j") && a[2..].parse::<u32>().is_ok())
            || (a.starts_with("-p") && a.len() > 2 && !a.starts_with("--"))
        {
            cargo.push(a);
        } else {
            tests.push(a);
        }
    }
    (cargo, tests)
}

#[cfg(test)]
mod tests {
    use super::{count_before, split, test_binaries};

    fn s(words: &[&str]) -> (Vec<String>, Vec<String>) {
        split(words.iter().map(|w| w.to_string()).collect())
    }

    #[test]
    fn cargo_flags_go_to_cargo_and_the_rest_to_the_tests() {
        let (c, t) = s(&["-j", "3", "--test-threads=2", "golden", "--no-fail-fast", "-j4", "--jobs=5"]);
        assert_eq!(c, ["-j", "3", "--no-fail-fast", "-j4", "--jobs=5"]);
        assert_eq!(t, ["--test-threads=2", "golden"]);
        let (c, t) = s(&["-p", "aic-core", "--", "-j", "x"]);
        assert_eq!(c, ["-p", "aic-core"]);
        assert_eq!(t, ["-j", "x"]);
        // A test target is cargo's, not a libtest name filter; a bare word is a filter.
        let (c, t) = s(&["--test", "experiment", "golden", "-paic-core"]);
        assert_eq!(c, ["--test", "experiment", "-paic-core"]);
        assert_eq!(t, ["golden"]);
    }

    /// 310: cargo's JSON lines as `cargo test --no-run --message-format=json` writes them on Windows (trimmed): the
    /// test artifacts with an executable are read, the plain `aic-sim` binary and a library are not.
    #[test]
    fn test_binaries_are_read_from_cargos_json() {
        let json = [
            r#"{"reason":"compiler-artifact","package_id":"path+file:///C:/r/crates/aic-sim#0.1.0","manifest_path":"C:\\r\\crates\\aic-sim\\Cargo.toml","target":{"kind":["test"],"crate_types":["bin"],"name":"experiment","src_path":"C:\\r\\x.rs"},"profile":{"opt_level":"1","test":true},"features":[],"filenames":["C:\\r\\t.exe"],"executable":"C:\\r\\target\\debug\\experiment-1.exe","fresh":true}"#,
            r#"{"reason":"compiler-artifact","package_id":"p","manifest_path":"C:\\r\\crates\\aic-sim\\Cargo.toml","target":{"kind":["bin"],"name":"aic-sim"},"profile":{"opt_level":"1","test":false},"executable":"C:\\r\\target\\debug\\aic-sim.exe","fresh":true}"#,
            r#"{"reason":"compiler-artifact","package_id":"p","manifest_path":"C:\\r\\crates\\aic-core\\Cargo.toml","target":{"kind":["lib"],"name":"aic_core"},"profile":{"opt_level":"1","test":false},"executable":null,"fresh":true}"#,
            r#"{"reason":"build-finished","success":true}"#,
        ]
        .join("\n");
        let bins = test_binaries(&json);
        assert_eq!(bins.len(), 1);
        assert_eq!(bins[0].name, "experiment");
        assert_eq!(bins[0].exe.to_str(), Some(r"C:\r\target\debug\experiment-1.exe"));
        assert_eq!(bins[0].dir.to_str(), Some(r"C:\r\crates\aic-sim"));
    }

    #[test]
    fn result_counts_are_read() {
        let line = "test result: ok. 409 passed; 2 failed; 4 ignored; 0 measured; 0 filtered out; finished in 1.00s";
        assert_eq!([count_before(line, "passed"), count_before(line, "failed"), count_before(line, "ignored")], [409, 2, 4]);
    }
}
