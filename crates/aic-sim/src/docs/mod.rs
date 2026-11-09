//! `aic-sim docs [--check]`: the parts of the rules docs generated from
//! content, configs and the op schema, so a number or a card has one home.
//!
//! The files of `docs/rules/` hold blocks between `<!-- generated: NAME -->` and
//! `<!-- /generated -->`, each block in exactly one file (`run::BLOCKS`); `docs/ops.md`
//! and `docs/harms.md` are generated whole (the ledger's checks fail the run), and so
//! are `docs/player-aid.md` (the house terms, printed in `docs/rules/README.md`'s
//! *Terms* too) and `web/card/faces.json`, every card's face from its ops
//! (`aic_core::face`); `docs/agents.md` holds the `strategies` block. The run fails
//! on a harm-ledger check, a block missing or doubled, a rules file or page over
//! 15 KB (`run::DOC_BUDGET`), a stale rules-revision line, a rules file quoting words
//! a card's face no longer says, a British spelling (`spelling`), an "NNN's changes"
//! heading (`current_docs`) or a problem in `docs/card-pricing-notes.json`.
//! `--check` changes nothing and exits 1 on any of those, if anything is stale, or if
//! a rules file backticks a `module.param` or bare `snake_case` name that isn't a
//! tunable or an op. The fast
//! test lane runs the check. A block is a pure function of `Rules` or
//! `Content`, so a test can show it moves with its source.

mod blocks;
mod current_docs;
mod harms;
mod run;
mod spelling;
#[cfg(test)]
mod tests;

/// The repo's docs at its root and in `web/` and `tools/` (paths from the root):
/// read by the spelling, history-heading and page-budget checks.
const ROOT_DOCS: [&str; 4] = ["README.md", "CLAUDE.md", "web/README.md", "tools/card-art/README.md"];

pub use run::run;

/// Every op's name in `op.rs`'s source, snake_case (082: the value table's
/// coverage test).
#[cfg(test)]
pub(crate) fn op_names(src: &str) -> Vec<String> {
    run::op_docs(src).into_iter().map(|o| o.name).collect()
}

/// Escapes a table cell.
fn cell(s: &str) -> String {
    s.replace('|', "\\|")
}

fn table(head: &[&str], rows: Vec<Vec<String>>) -> String {
    let mut out = format!("| {} |\n|{}\n", head.join(" | "), "---|".repeat(head.len()));
    for r in rows {
        let r: Vec<String> = r.iter().map(|c| cell(c)).collect();
        out.push_str(&format!("| {} |\n", r.join(" | ")));
    }
    out
}
