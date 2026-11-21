//! American English everywhere (docs/card-guidelines.md): `aic-sim docs --check`, so the
//! fast lane, refuses a British spelling in what the repo says. A stem matches inside
//! identifiers too (`kind_colour`), any case. `tools/` is not read but for the card-art
//! README: its scene ids are file stems in the shared art store. `experiments/results/`
//! is a record of its day. `scripts/`, `crates/*/tests` and `web/scripts` are not read
//! either. It is a list, not a dictionary: a word outside it passes.

use std::fs;
use std::path::Path;

/// British stems. Whole-enough to miss "organism", "realistic" and "programmer".
const BRITISH: &[&str] = &[
    "colour", "neighbour", "behaviour", "armour", "favour", "flavour", "harbour", "honour", "labour", "rumour",
    "centre", "metres", "litre", "fibre", "theatre", "defence", "offence", "licence", "pretence", "grey",
    "mobilis", "modelling", "modelled", "labelled", "labelling", "cancelled", "cancelling", "travelled", "bevelled",
    "stylis", "summaris", "organise", "organisation", "recognis", "optimise", "optimisation", "normalise", "prioritis",
    "analogue", "catalogue", "dialogue box", "artefact", "misspelt", "towards", "afterwards", "whilst", "amongst",
    "mould", "manoeuvre", "judgement",
    "burnt", "learnt", "spelt", "dreamt", "utilis", "authoris", "neutralis", "stabilis", "centralis", "customis",
    "visualis", "serialis", "randomis", "memoris", "categoris", "synchronis", "tokenis", "digitis", "weaponis",
    "privatis", "realise", "realisation", "specialise", "characterise", "analysed", "analysing", "paralysed", "paralysing", "catalyse",
    "programme ", "programmes", "aluminium", "ageing", "skilful", "wilful", "fuelled", "fuelling", "signalled", "signalling",
    "levelled", "levelling", "dialled", "storey", "draught", "sceptic", "cheque", "plough", "calibre",
    "sabre", "sombre", "spectre", "lustre", "meagre", "vigour", "valour", "savour", "odour", "humour", "glamour",
    "tumour", "endeavour", "splendour", "vapour", "rigour", "saviour", "backwards", "upwards",
    "downwards", "onwards",
];

/// Where the repo speaks to a reader beside [`super::ROOT_DOCS`]: paths from its root.
const ROOTS: [&str; 11] = [
    "content",
    "docs",
    "configs",
    "crates/aic-core/src",
    "crates/aic-sim/src",
    "crates/aic-wasm/src",
    "web/card",
    "web/cards",
    "web/src",
    "web/index.html",
    "web/cards.html",
];
const THIS_FILE: &str = "spelling.rs";
const TEXT: [&str; 8] = ["rs", "json", "md", "js", "ts", "css", "html", "mjs"];

/// The British stems in `text`.
pub fn british(text: &str) -> Vec<&'static str> {
    let low = text.to_lowercase();
    BRITISH.iter().copied().filter(|b| low.contains(b)).collect()
}

/// Every British spelling in `ROOT_DOCS` and under `ROOTS`, one line each.
pub fn failures() -> Vec<String> {
    let mut files = Vec::new();
    for root in super::ROOT_DOCS.into_iter().chain(ROOTS) {
        list(Path::new(root), &mut files);
    }
    let mut out = Vec::new();
    for path in files.iter().filter(|p| !p.ends_with(THIS_FILE)) {
        let Ok(text) = fs::read_to_string(path) else { continue };
        for (i, line) in text.lines().enumerate() {
            for b in british(line) {
                out.push(format!("{}:{}: British spelling {b:?}", path.display(), i + 1));
            }
        }
    }
    out
}

/// 169, the house words (r70; Matt, amendment 7657): printed text calls the adversary's pieces agents and the
/// Security token Operatives; 184, r77's renames (178): corp troops are Drones, the node power was Expand (gone at r91) and
/// the troop line Muster (a substring match: "deployed" is refused too; r141: Muster is gone, and Redeploy, the
/// troops' icon, is let through). An old name, or the open naming note, in what a player reads is refused. Code
/// names may stay, so the crates are not read. 304: the words war, the Envoy and Dispatches took with them (r139,
/// r142) too: "drone", "envoy" and "military base" in any case, "Dispatch" and "Growth" capitalized (the house words;
/// plain "growth" is English). The card names that keep one ([`KEEP_NAMES`]) are let through.
const STALE: &[&str] = &[
    "adversary ambassador", "adversary's ambassador", "enemy ambassador", "adv. ambassador", "matt to confirm",
    "agents token", "security agent",
    "corp troop", "deploy", "drone", "envoy", "military base",
];

/// 304: the stale house words matched as printed (capitalized), not lowercased.
const STALE_CASED: &[&str] = &["Dispatch", "Growth"];

/// 304: card names (and their ids) that keep a stale word: stripped before the match.
const KEEP_NAMES: &[&str] = &[
    "survey drone fleet", "survey_drone_fleet", "envoy briefings", "envoy_briefings", "dispatch control room",
    "dispatch_control_room",
];

/// What a player reads: the content, the rules, the generated faces, the viewer's words.
const PLAYER_TEXT: [&str; 8] = [
    "content", "docs/rules", "docs/player-aid.md", "docs/rulebook.md", "docs/cards.md", "docs/adversary-cards.md",
    "docs/card-pricing-notes.json", "web/src",
];

/// The stale house words in `text`.
pub fn stale(text: &str) -> Vec<&'static str> {
    // r141: "Redeploy" is a house word (the troops' icon); the old troop and node verb "deploy" is still refused.
    let mut low = text.to_lowercase().replace("redeploy", "");
    let mut cased = text.to_string();
    for k in KEEP_NAMES {
        low = low.replace(k, "");
        while let Some(i) = cased.to_ascii_lowercase().find(k) {
            cased.replace_range(i..i + k.len(), "");
        }
    }
    let lower = STALE.iter().copied().filter(|b| low.contains(b));
    lower.chain(STALE_CASED.iter().copied().filter(|b| cased.contains(b))).collect()
}

/// Every stale house word in what a player reads, one line each (`docs --check`).
pub fn stale_failures() -> Vec<String> {
    let mut files = Vec::new();
    for root in PLAYER_TEXT {
        list(Path::new(root), &mut files);
    }
    let mut out = Vec::new();
    for path in files {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        for (i, line) in text.lines().enumerate() {
            for b in stale(line) {
                let now = match b {
                    "corp troop" => "gone with war at r139 (they were Drones from r77)",
                    "deploy" => "troops garrison (r141; the line was Muster) and nodes are placed by node powers (r142); Redeploy is the troops' icon",
                    "drone" | "military base" => "gone with war at r139",
                    "envoy" => "the Envoy standard action is gone",
                    "Dispatch" | "Growth" => "gone at r142: every ambassador moves once a round; boards print node powers",
                    _ => "agents, Operatives: docs/rules/README.md, Terms",
                };
                out.push(format!("{}:{}: stale house word {b:?} ({now})", path.display(), i + 1));
            }
        }
    }
    out
}

/// The text files at or under `path`, sorted and depth first.
fn list(path: &Path, out: &mut Vec<std::path::PathBuf>) {
    if path.is_dir() {
        // Generated by wasm-pack and git-ignored.
        if path.ends_with("wasm") {
            return;
        }
        let Ok(dir) = fs::read_dir(path) else { return };
        let mut entries: Vec<_> = dir.flatten().map(|e| e.path()).collect();
        entries.sort();
        for e in entries {
            list(&e, out);
        }
    } else if TEXT.contains(&path.extension().and_then(|e| e.to_str()).unwrap_or("")) {
        out.push(path.to_path_buf());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_house_words_match_in_any_case() {
        assert_eq!(stale("Income: +1 Adversary ambassador"), ["adversary ambassador"]);
        assert!(stale("an agent cancels one of your ambassadors; Operatives (Security)").is_empty());
    }

    #[test]
    fn a_stem_matches_in_any_case_and_inside_a_name() {
        assert_eq!(british("Total Mobilisation"), ["mobilis"]);
        assert_eq!(british("kind_COLOUR"), ["colour"]);
    }

    #[test]
    fn american_words_pass() {
        assert!(british("Mobilization, Armored, Modeling; organism, realistic, programmer, center, analyses, checkerboard").is_empty());
    }
}
