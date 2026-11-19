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

/// The leading string literals of each `join(root, 'a', 'b', ...)` in `src`, as `a/b` (a template or a variable ends
/// one: `join(root, 'content', `${name}.json`)` reads under `content`).
fn joined_paths(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (i, _) in src.match_indices("join(root,") {
        let mut rest = &src[i + "join(root,".len()..];
        let mut parts = Vec::new();
        loop {
            rest = rest.trim_start_matches([' ', '\n', '\r', ',']);
            let Some(r) = rest.strip_prefix('\'') else { break };
            let Some(end) = r.find('\'') else { break };
            parts.push(&r[..end]);
            rest = &r[end + 1..];
        }
        if !parts.is_empty() {
            out.push(parts.join("/"));
        }
    }
    out
}

/// 285: `cards:check` copies what the card export reads into a fixture root; 267 made the export read
/// `web/card/adversary.json`, the copy list never gained it, and the check crashed at its export for 18 items. Every path
/// the export joins onto its root (the `out/` it writes aside) is copied, or lies under or above a copied one.
#[test]
fn cards_check_copies_what_the_card_export_reads() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |f: &str| std::fs::read_to_string(root.join(f)).unwrap();
    let export = read("web/scripts/cards-export.mjs");
    let check = read("web/scripts/cards-check.mjs");
    let list_at = check.find("for (const p of [").expect("cards-check.mjs's copy list");
    let list = &check[list_at..list_at + check[list_at..].find(']').unwrap()];
    let copied: Vec<&str> = list.split('\'').skip(1).step_by(2).collect();
    assert!(copied.len() > 5, "copy list: {copied:?}");
    let needed: Vec<String> = joined_paths(&export).into_iter().filter(|p| !p.starts_with("out")).collect();
    assert!(needed.iter().any(|p| p == "web/card/faces.json"), "parsed reads: {needed:?}");
    let missing: Vec<&String> = needed
        .iter()
        .filter(|p| !copied.iter().any(|c| p.as_str() == *c || p.starts_with(&format!("{c}/")) || c.starts_with(&format!("{p}/"))))
        .collect();
    assert!(missing.is_empty(), "cards-export.mjs reads {missing:?}, which cards-check.mjs doesn't copy ({copied:?})");
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
