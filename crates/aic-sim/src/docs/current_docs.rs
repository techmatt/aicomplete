//! Docs state the current game (CLAUDE.md): an item updates the current text in place and
//! never appends an "NNN's changes" section; history goes in the item's write-up under
//! `experiments/results/`. `aic-sim docs --check`, so the fast lane, refuses a Markdown
//! heading like `## 080's changes` in what the repo says today. `experiments/results/` is
//! a record of its day and is not read.

use std::fs;
use std::path::Path;

/// Where the repo's current docs live beside [`super::ROOT_DOCS`]: paths from its root. A
/// directory is read whole but for `experiments/results/`; `experiments` itself only at its
/// top level.
const DIRS: [&str; 1] = ["docs"];
const TOP_LEVEL: [&str; 1] = ["experiments"];

/// Whether `line` is a Markdown heading naming an item's changes: `#`s, a space, then
/// somewhere three digits followed by `'s changes` (any case, straight or curly quote).
pub fn history_heading(line: &str) -> bool {
    let t = line.trim_start();
    let hashes = t.bytes().take_while(|&b| b == b'#').count();
    if hashes == 0 || hashes > 6 || !t[hashes..].starts_with(' ') {
        return false;
    }
    let low = t.to_lowercase();
    let b = low.as_bytes();
    for marker in ["'s changes", "\u{2019}s changes"] {
        let mut from = 0;
        while let Some(at) = low[from..].find(marker) {
            let at = from + at;
            if at >= 3 && b[at - 3..at].iter().all(u8::is_ascii_digit) && (at == 3 || !b[at - 4].is_ascii_digit()) {
                return true;
            }
            from = at + marker.len();
        }
    }
    false
}

fn read(path: &Path, out: &mut Vec<String>) {
    let Ok(text) = fs::read_to_string(path) else { return };
    let mut fenced = false;
    for (i, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
        } else if !fenced && history_heading(line) {
            out.push(format!(
                "{}:{}: a history heading ({}): docs state the current game; fold it into the current text, history goes in the item's write-up",
                path.display(),
                i + 1,
                line.trim()
            ));
        }
    }
}

fn walk(path: &Path, deep: bool, out: &mut Vec<String>) {
    let Ok(dir) = fs::read_dir(path) else { return };
    let mut entries: Vec<_> = dir.flatten().map(|e| e.path()).collect();
    entries.sort();
    for e in entries {
        if e.is_dir() {
            if deep {
                walk(&e, deep, out);
            }
        } else if e.extension().and_then(|x| x.to_str()) == Some("md") {
            read(&e, out);
        }
    }
}

/// Every history heading in the current docs, one line each.
pub fn failures() -> Vec<String> {
    let mut out = Vec::new();
    for f in super::ROOT_DOCS {
        read(Path::new(f), &mut out);
    }
    for d in DIRS {
        walk(Path::new(d), true, &mut out);
    }
    for d in TOP_LEVEL {
        walk(Path::new(d), false, &mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_items_changes_heading_is_refused() {
        assert!(history_heading("## 080's changes (r34): the liberate capstones"));
        assert!(history_heading("### The 024\u{2019}s Changes"));
        assert!(history_heading("# 082's changes"));
    }

    #[test]
    fn prose_and_other_headings_pass() {
        assert!(!history_heading("080's changes are in its write-up."));
        assert!(!history_heading("## The rates"));
        assert!(!history_heading("## 1080's changes"));
        assert!(!history_heading("##080's changes"));
        assert!(!history_heading("## Rates known to be off at r34"));
    }
}
