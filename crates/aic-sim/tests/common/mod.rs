//! Helpers shared by the integration tests.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The built binary, run from the workspace root (configs/ is relative to it).
pub fn sim() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_aic-sim"));
    c.current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
    c
}

/// `name` under the test temp dir, emptied of an earlier run; `create`
/// makes it again, empty, or leaves it absent for the binary to make.
/// Each test binary compiles this file alone, and not all of them call it.
#[allow(dead_code)]
pub fn fresh_dir(name: &str, create: bool) -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&d);
    if create {
        std::fs::create_dir_all(&d).unwrap();
    }
    d
}

/// Writes the experiment spec `body` as `spec.json` in `dir`.
#[allow(dead_code)]
pub fn spec_in(dir: &Path, body: impl AsRef<str>) -> PathBuf {
    let spec = dir.join("spec.json");
    std::fs::write(&spec, body.as_ref()).unwrap();
    spec
}

/// The JSON file at `path`.
#[allow(dead_code)]
pub fn read_json(path: impl AsRef<Path>) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// Fails with the binary's stderr unless it exited cleanly.
#[allow(dead_code)]
pub fn assert_ok(res: &Output) {
    assert!(res.status.success(), "{}", String::from_utf8_lossy(&res.stderr));
}
