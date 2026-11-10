//! Builds the repo's configs and farmed specs into the bundle (a browser reads
//! no files): every `.json` under `configs/`, `experiments/configs/` and
//! `experiments/farm/`, keyed by its path from the repo root.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const DIRS: &[&str] = &["configs", "experiments/configs", "experiments/farm"];

fn walk(root: &Path, rel: &str, out: &mut Vec<(String, PathBuf)>) {
    let dir = root.join(rel);
    let Ok(entries) = std::fs::read_dir(&dir) else { return };
    let mut entries: Vec<_> = entries.map(|e| e.expect("a dir entry").path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        let key = format!("{rel}/{name}");
        if p.is_dir() {
            walk(root, &key, out);
        } else if name.ends_with(".json") {
            out.push((key, p));
        }
    }
}

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    let mut files = Vec::new();
    for d in DIRS {
        println!("cargo:rerun-if-changed={}", root.join(d).display());
        walk(&root, d, &mut files);
    }
    let mut s = String::from("&[\n");
    for (key, path) in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let abs = path.canonicalize().unwrap();
        writeln!(s, "    ({key:?}, include_str!({:?})),", abs.display().to_string()).unwrap();
    }
    s.push(']');
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("files.rs");
    std::fs::write(out, s).unwrap();
}
