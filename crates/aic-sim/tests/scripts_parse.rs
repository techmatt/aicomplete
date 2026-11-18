//! 261: every Node script the repo runs parses (`node --check`): e2e:researcher held a duplicate `const` from 252
//! to 261, so the lane stopped at load and no check in it ran. Skipped, with a note, where `node` isn't on PATH.

use std::path::{Path, PathBuf};
use std::process::Command;

fn scripts(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    // `node --check` parses only the file it is given, not its imports: every folder holding a module is listed.
    for dir in ["web/scripts", "web/card", "scripts", "scripts/farm", "tools/card-art"] {
        let Ok(entries) = std::fs::read_dir(root.join(dir)) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            // web/card's `.js` are modules the scripts import (web/package.json is `"type": "module"`).
            let js = dir == "web/card" && p.extension().is_some_and(|x| x == "js");
            if js || p.extension().is_some_and(|x| x == "mjs") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

#[test]
fn node_scripts_parse() {
    if Command::new("node").arg("--version").output().is_err() {
        eprintln!("node not on PATH: Node scripts not checked");
        return;
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let files = scripts(&root);
    assert!(files.len() > 20, "found only {} scripts under {}", files.len(), root.display());
    // A process each, side by side: about 0.1 s apiece in turn.
    let bad: Vec<String> = std::thread::scope(|s| {
        let runs: Vec<_> = files
            .chunks(files.len().div_ceil(8))
            .map(|chunk| {
                s.spawn(move || {
                    chunk
                        .iter()
                        .filter_map(|f| {
                            let out = Command::new("node").arg("--check").arg(f).output().unwrap();
                            (!out.status.success())
                                .then(|| format!("{}: {}", f.display(), String::from_utf8_lossy(&out.stderr)))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        runs.into_iter().flat_map(|r| r.join().unwrap()).collect()
    });
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
