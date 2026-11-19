//! 286: the Python read tools' tests (`scripts/bench/test_read_tools.py`, `scripts/test_input_checks.py`,
//! `scripts/bench/test_trace_lib.py`) as a slow-lane case, so `cargo test-all` runs them (they were in no lane).
//! They need `python` on PATH (README, *Toolchain*), no build and no binary.

use std::path::Path;
use std::process::Command;

#[test]
#[ignore = "slow lane: runs Python's unittest over the read tools' tests (~3 s)"]
fn python_read_tool_tests_pass() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = Command::new(std::env::var("PYTHON").unwrap_or_else(|_| "python".into()))
        .current_dir(&root)
        .args(["-m", "unittest", "scripts/bench/test_read_tools.py", "scripts/test_input_checks.py",
               "scripts/bench/test_trace_lib.py"])
        .output()
        .unwrap_or_else(|e| panic!("could not run python (set PYTHON to its path): {e}"));
    assert!(
        out.status.success(),
        "the Python read-tool tests failed:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
