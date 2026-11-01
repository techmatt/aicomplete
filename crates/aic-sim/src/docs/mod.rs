//! `aic-sim docs [--check]`: the parts of the rules docs generated from
//! content, configs and the op schema, so a number or a card has one home.
//!
//! `docs/rules.md` holds blocks between `<!-- generated: NAME -->` and
//! `<!-- /generated -->`; `docs/ops.md` and `docs/harms.md` are generated
//! whole (the ledger's checks fail the run), and so are `docs/player-aid.md`
//! (the house terms, printed in rules.md's *Terms* too) and
//! `web/card/faces.json`, every card's face from its ops (`aic_core::face`).
//! `--check` changes nothing and exits 1 if anything is stale, if `rules.md`
//! backticks a `module.param` or bare `snake_case` name that isn't a tunable,
//! or on a British spelling (`crate::spelling`). The fast
//! test lane runs the check. A block is a pure function of `Rules` or
//! `Content`, so a test can show it moves with its source.

mod blocks;
mod harms;
mod run;
#[cfg(test)]
mod tests;

pub use run::run;

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
