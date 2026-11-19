//! 283: the test lanes' front. `cargo test-all` and `cargo test-slow` (`.cargo/config.toml`) run this, which runs
//! `cargo test --workspace [CARGO FLAGS] -- --include-ignored|--ignored [TEST ARGS]`. As plain aliases
//! (`test --workspace -- --include-ignored`) every word typed after the alias went to the test binaries, so
//! `cargo test-all -j 3` stopped in a second on libtest's "Unrecognized option: 'j'" (it read as a pass), and
//! `--no-fail-fast` the same. Here cargo's own flags go to cargo; anything else (`--test-threads=N`, a name filter,
//! `--nocapture`) and everything after a `--` goes to the tests. No dependencies, so it builds in a second.

use std::process::{exit, Command};

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
    let lane = match args.next().as_deref() {
        Some("all") => "--include-ignored",
        Some("slow") => "--ignored",
        other => {
            eprintln!("aic-lanes: the first word is the lane, all or slow (not {other:?}); run it as cargo test-all or cargo test-slow");
            exit(2);
        }
    };
    let (cargo, tests) = split(args.collect());
    let package = cargo.iter().any(|a| {
        a == "-p" || a == "--package" || a == "--workspace" || a.starts_with("--package=") || (a.starts_with("-p") && a.len() > 2)
    });
    let mut cmd = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
    cmd.arg("test");
    if !package {
        cmd.arg("--workspace");
    }
    cmd.args(&cargo).arg("--").arg(lane).args(&tests);
    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("aic-lanes: could not run cargo: {e}");
        exit(2)
    });
    exit(status.code().unwrap_or(1));
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
    use super::split;

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
}
