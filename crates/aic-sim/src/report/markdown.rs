//! `report.md`: the run, the findings, then the tables, the timing and the
//! saved records.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use aic_core::content_hash;

use crate::experiment::{PEAKS, UNSPENT, UNSPENT_HIGH};
use crate::gate::{Decided, GateState};

use super::findings::findings;
use super::records::CATEGORIES;
use super::stats::{agent_order, time_split, CellSummary, Paired};
use super::{Run, SavedRecord};

/// `+4.2 ±3.1` (percentage points).
pub(super) fn pts(x: f64, se: f64) -> String {
    // `+ 0.0` turns a negated zero into a plain one.
    format!("{:+.1} ±{:.1}", 100.0 * x + 0.0, 100.0 * se)
}

pub(super) fn p_value(p: f64) -> String {
    if p < 0.001 {
        "<0.001".into()
    } else {
        format!("{p:.3}")
    }
}

pub(super) fn pct(x: f64) -> String {
    format!("{:.0}%", 100.0 * x)
}

fn opt(x: Option<f64>, digits: usize) -> String {
    x.map_or("–".into(), |v| format!("{v:.digits$}"))
}

/// One report being written: its run, and whether cells need their config
/// or variant named.
struct Md<'a> {
    run: &'a Run<'a>,
    multi_c: bool,
    multi_v: bool,
}

impl Md<'_> {
    /// `c ` with several configs.
    fn cfg(&self, c: &str) -> String {
        if self.multi_c {
            format!("{c} ")
        } else {
            String::new()
        }
    }

    /// `config variant `, each only if there are several.
    fn prefix(&self, config: &str, variant: &str) -> String {
        let v = if self.multi_v {
            format!("{variant} ")
        } else {
            String::new()
        };
        format!("{}{v}", self.cfg(config))
    }

    /// Short cell label for tables.
    fn short(&self, c: &CellSummary) -> String {
        let mut s = String::new();
        if self.multi_c {
            s.push_str(&c.config_name);
            s.push(' ');
        }
        if self.multi_v {
            s.push_str(&c.variant);
            s.push(' ');
        }
        format!("{s}d{} {}", c.difficulty, c.agent)
    }
}

/// A pair's seeds: `both / only a / only b / neither`.
fn paired_cells(p: &Paired) -> String {
    format!("{} / {} / {} / {}", p.both, p.only_a, p.only_b, p.neither)
}

/// `wins with 2+ buildings played (standing)`.
fn wins_2plus(s: &CellSummary) -> String {
    match (s.wins_2plus_buildings_played, s.wins_2plus_buildings_standing) {
        (Some(a), Some(b)) => format!("{} ({})", pct(a), pct(b)),
        _ => "–".into(),
    }
}

/// A cell's strategy-health numbers, joined by `sep` with `last` before the wins.
fn health_row(s: &CellSummary, sep: &str, last: &str) -> String {
    format!(
        "{:.2}{sep}{:.2}{sep}{:.1}{sep}{:.2}{last}{}",
        s.buildings_played_per_game,
        s.tier2_played_per_game,
        s.cards_played_per_game,
        s.economy_buildings_standing_per_game,
        wins_2plus(s)
    )
}

pub fn markdown(run: &Run) -> String {
    let md = Md {
        run,
        multi_c: run.spec.configs.len() > 1,
        multi_v: run.spec.variants.len() > 1,
    };
    let mut m = String::new();
    let w = &mut m;
    header(&md, w);
    measures(&md, w);
    findings_and_gate(&md, w);
    win_table(&md, w);
    head_to_head(&md, w);
    agent_pairs(&md, w);
    variant_diffs(&md, w);
    fresh_confirmation(&md, w);
    removals(&md, w);
    game_length(&md, w);
    decisions(&md, w);
    win_probability(&md, w);
    luck(&md, w);
    build_variety(&md, w);
    strategy_health(&md, w);
    card_rates(&md, w);
    timing(&md, w);
    saved_records(&md, w);
    m
}

fn header(md: &Md, w: &mut String) {
    let run = md.run;
    let spec = md.run.spec;
    let _ = writeln!(w, "# Cycle report: {}\n", spec.name);
    if run.games.len() + run.failed.len() < run.planned {
        let _ = writeln!(
            w,
            "**Partial: {} of {} games.** The run is still going or was stopped; every \
             table below covers only the games finished (`--resume` plays the rest).\n",
            run.games.len(),
            run.planned
        );
    }
    if !spec.note.is_empty() {
        let _ = writeln!(w, "{}\n", spec.note);
    }
    let _ = writeln!(
        w,
        "{} games ({} cells × {}{} seeds, {}..={}) on {} threads in {:.1} s. Engine {}, rules r{}, content {}.\n",
        run.games.len(),
        run.cells.len(),
        if spec.gate.is_some() { "at most " } else { "" },
        spec.games,
        spec.first_seed,
        spec.first_seed + spec.games - 1,
        run.threads,
        run.wall_s,
        aic_core::VERSION,
        aic_core::RULES_REVISION,
        content_hash()
    );
    if run.resumed > 0 {
        let _ = writeln!(
            w,
            "{} of the games were played by an earlier, stopped run (`--resume`); the time is this run's.\n",
            run.resumed
        );
    }
}

fn measures(md: &Md, w: &mut String) {
    let spec = md.run.spec;
    let sums = md.run.sums;
    let _ = writeln!(w, "- Configs: {}", spec.configs.join(", "));
    let vs: Vec<String> = spec
        .variants
        .iter()
        .map(|v| {
            if v.set.is_empty() {
                v.name.clone()
            } else {
                format!("{} ({})", v.name, v.set.join(" "))
            }
        })
        .collect();
    let _ = writeln!(w, "- Variants: {}", vs.join("; "));
    let mut overlays: Vec<String> = Vec::new();
    for s in sums.iter().filter(|s| s.content_hash != content_hash()) {
        let o = format!("{} {}", s.variant, s.content_hash);
        if !overlays.contains(&o) {
            overlays.push(o);
        }
    }
    if !overlays.is_empty() {
        let _ = writeln!(w, "- Overlaid content: {}", overlays.join("; "));
    }
    let ds: Vec<String> = spec.difficulties.iter().map(|d| d.to_string()).collect();
    let _ = writeln!(w, "- Difficulties: {}", ds.join(", "));
    let _ = writeln!(w, "- Agents (weakest first): {}", spec.agents.join(", "));
    let _ = writeln!(
        w,
        "- Win probability: {}",
        if spec.winprob.games > 0 {
            format!(
                "first {} seeds per cell, {} `{}` playouts at the start of each round",
                spec.winprob.games, spec.winprob.rollouts, spec.winprob.agent
            )
        } else {
            "off".into()
        }
    );
    let _ = writeln!(
        w,
        "- Decision closeness: {}",
        if spec.closeness.every > 0 {
            format!(
                "every {} real choice(s) probed by 1-ply greedy scoring; close = best two within {} points",
                spec.closeness.every, spec.closeness.gap
            )
        } else {
            "off".into()
        }
    );
    if spec.winprob.games == 0 || spec.closeness.every == 0 {
        let _ = writeln!(
            w,
            "- Off measures are opt-in (docs/experiments.md): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`."
        );
    }
    let bad: usize = sums.iter().map(|s| s.games - s.replay_ok).sum();
    let _ = writeln!(
        w,
        "- Replay check: {}\n",
        if bad == 0 {
            "every game replays from seed + actions".to_string()
        } else {
            format!("**{bad} game(s) failed to replay**")
        }
    );
}

fn failed_games(run: &Run, w: &mut String) {
    if run.failed.is_empty() {
        return;
    }
    let _ = writeln!(
        w,
        "## Failed games\n\n**{} game(s) panicked.** No measure counts them; a gate gives their seeds to neither side. `--resume` plays them again; `failed.jsonl` has them too.\n",
        run.failed.len()
    );
    let _ = writeln!(w, "| variant | d | agent | seed | panic | to play it again |");
    let _ = writeln!(w, "|---|---|---|---|---|---|");
    for f in run.failed {
        let _ = writeln!(
            w,
            "| {} | {} | {} | {} | {} | `{}` |",
            f.variant,
            f.difficulty,
            f.agent,
            f.seed,
            f.message.replace('|', "/"),
            f.command()
        );
    }
    let _ = writeln!(w);
}

fn findings_and_gate(md: &Md, w: &mut String) {
    let run = md.run;
    failed_games(run, w);
    // Findings first: what a reader needs before the tables.
    let _ = writeln!(w, "## Findings\n");
    for f in findings(run) {
        let _ = writeln!(w, "- {f}");
    }

    if let Some(g) = run.gate {
        let _ = write!(w, "{}", gate_section(g));
    }
}

fn win_table(md: &Md, w: &mut String) {
    let run = md.run;
    let sums = md.run.sums;
    let _ = writeln!(w, "\n## Win rate by skill tier × difficulty\n");
    let _ = writeln!(w, "Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.\n");
    for g in run.groups {
        let mut head = format!(
            "| {}difficulty {} |",
            md.prefix(&g.config, &g.variant),
            g.difficulty
        );
        let mut sep = String::from("|---|");
        let mut row = String::from("| win % (survival) |");
        for (a, _) in &g.win_rates {
            let s = sums
                .iter()
                .find(|s| {
                    s.agent == *a
                        && s.difficulty == g.difficulty
                        && s.variant == g.variant
                        && s.config == g.config
                })
                .expect("cell");
            let _ = write!(head, " {a} |");
            sep.push_str("---|");
            let _ = write!(
                row,
                " {:.1} ±{:.1} ({:.2}) |",
                100.0 * s.win_rate,
                100.0 * s.win_rate_se,
                s.survival_mean
            );
        }
        let _ = writeln!(
            w,
            "{head} skill gap |\n{sep}---|\n{row} {:+.1} pts ({:+.2} survival) |\n",
            100.0 * g.skill_gap,
            g.skill_gap_survival
        );
    }
}

fn head_to_head(md: &Md, w: &mut String) {
    let run = md.run;
    let h2h = run.head_to_head;
    if let Some(first) = h2h.first() {
        let _ = writeln!(w, "## Head to head against greedy\n");
        let _ = writeln!(w, "Each agent and `{}` play the same seeds in every cell. Won by both / only the agent / only greedy / neither. Difference: the agent's win rate minus greedy's, ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won. Beats greedy: ahead with p < 0.05.\n", first.greedy);
        let _ = writeln!(w, "| cell | games | agent win % | greedy win % | both / only agent / only greedy / neither | difference (pts) | p | verdict |");
        let _ = writeln!(w, "|---|---|---|---|---|---|---|---|");
        for h in h2h {
            let p = &h.paired;
            let _ = writeln!(
                w,
                "| {}d{} {} | {} | {:.1} | {:.1} | {} | {} | {} | {} |",
                md.prefix(&h.config, &h.variant),
                h.difficulty,
                h.agent,
                p.games,
                100.0 * p.a_win_rate,
                100.0 * p.b_win_rate,
                paired_cells(p),
                pts(p.diff, p.diff_se),
                p_value(p.sign_p),
                h.verdict
            );
        }
        let _ = writeln!(w);
    }
}

fn agent_pairs(md: &Md, w: &mut String) {
    let run = md.run;
    if !run.agent_pairs.is_empty() {
        let _ = writeln!(w, "## Agents paired, per variant\n");
        let _ = writeln!(w, "A against B on the same seeds. An agent labeled `x:...` is a check against `x`; a bare agent is paired with the next bare agent listed. A minus B: the win-rate difference ± the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.\n");
        let _ = writeln!(w, "| variant | d | A | B | games | A win % | B win % | both / only A / only B / neither | A minus B (pts ± se) | p |");
        let _ = writeln!(w, "|---|---|---|---|---|---|---|---|---|---|");
        for x in run.agent_pairs {
            let p = &x.paired;
            let _ = writeln!(
                w,
                "| {}{} | {} | {} | {} | {} | {:.1} | {:.1} | {} | {} | {} |",
                md.cfg(&x.config),
                x.variant,
                x.difficulty,
                x.a,
                x.b,
                p.games,
                100.0 * p.a_win_rate,
                100.0 * p.b_win_rate,
                paired_cells(p),
                pts(p.diff, p.diff_se),
                p_value(p.sign_p)
            );
        }
        let _ = writeln!(w);
    }
}

fn variant_diffs(md: &Md, w: &mut String) {
    let run = md.run;
    let vds = run.variant_diffs;
    if let Some(first) = vds.first() {
        let _ = writeln!(w, "## Variants against the base\n");
        let _ = writeln!(w, "The first variant, `{}`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.\n", first.base);
        let _ = writeln!(
            w,
            "| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |"
        );
        let _ = writeln!(w, "|---|---|---|---|---|---|---|");
        for v in vds {
            let p = &v.paired;
            let _ = writeln!(
                w,
                "| {}{} | {} | {} | {:.1} → {:.1} | {} | {} | {:.2} → {:.2} |",
                md.cfg(&v.config),
                v.variant,
                v.difficulty,
                v.agent,
                100.0 * p.b_win_rate,
                100.0 * p.a_win_rate,
                pts(p.diff, p.diff_se),
                p_value(p.sign_p),
                p.b_rounds,
                p.a_rounds
            );
        }
        let _ = writeln!(w);
    }
}

fn fresh_confirmation(md: &Md, w: &mut String) {
    let run = md.run;
    let spec = md.run.spec;
    if let Some(sel) = &spec.selection_seeds {
        let _ = writeln!(w, "## Fresh-seed confirmation\n");
        let fresh = run.fresh_confirmation;
        // 118: inclusive, as the header prints seeds; none when the spec picks on none.
        let picked = if sel.games == 0 { "No selection seeds".to_string() } else { format!("Selection seeds {}..={}", sel.first, sel.first.saturating_add(sel.games - 1)) };
        let _ = writeln!(w, "{picked} picked the winner; fresh seeds are every other seed played. Variant minus base, ± the standard error of the per-seed difference, with p from the exact sign test. The two are never pooled: only the fresh row confirms.\n");
        if fresh.is_empty() {
            let _ = writeln!(w, "Nothing to confirm: no variant after the base has a pair.\n");
        } else if fresh.iter().all(|f| f.fresh.games == 0) {
            let _ = writeln!(w, "No fresh seeds yet: every game so far is on the selection seeds.\n");
        }
        if !fresh.is_empty() {
            let _ = writeln!(w, "| variant | d | agent | fresh games | fresh win % base → variant | fresh difference (pts) | fresh p | selection games | selection win % base → variant | selection difference (pts) | selection p |");
            let _ = writeln!(w, "|---|---|---|---|---|---|---|---|---|---|---|");
            let side = |p: &Paired| {
                if p.games == 0 {
                    "0 | – | – | –".to_string()
                } else {
                    format!(
                        "{} | {:.1} → {:.1} | {} | {}",
                        p.games,
                        100.0 * p.b_win_rate,
                        100.0 * p.a_win_rate,
                        pts(p.diff, p.diff_se),
                        p_value(p.sign_p)
                    )
                }
            };
            for f in fresh {
                let _ = writeln!(
                    w,
                    "| {}{} | {} | {} | {} | {} |",
                    md.cfg(&f.config),
                    f.variant,
                    f.difficulty,
                    f.agent,
                    side(&f.fresh),
                    side(&f.selection)
                );
            }
            let _ = writeln!(w);
        }
    }
}

fn removals(md: &Md, w: &mut String) {
    let vds = md.run.variant_diffs;
    if vds.iter().any(|v| !v.off.is_empty()) {
        let _ = writeln!(w, "## Removing a module\n");
        let _ = writeln!(w, "What a module is worth: the base's win rate minus the win rate with the module switched off, on the same seeds, ± the standard error of the per-seed difference. Positive: the module helps the players; negative: it holds them back.\n");
        let _ = writeln!(
            w,
            "| module (variant) | difficulty | agent | base win % | off win % | worth (pts) | p |"
        );
        let _ = writeln!(w, "|---|---|---|---|---|---|---|");
        for v in vds.iter().filter(|v| !v.off.is_empty()) {
            let p = &v.paired;
            let _ = writeln!(
                w,
                "| {}{} ({}) | {} | {} | {:.1} | {:.1} | {} | {} |",
                md.cfg(&v.config),
                v.off.join(" + "),
                v.variant,
                v.difficulty,
                v.agent,
                100.0 * p.b_win_rate,
                100.0 * p.a_win_rate,
                pts(-p.diff, p.diff_se),
                p_value(p.sign_p)
            );
        }
        let _ = writeln!(w);
    }
}

fn game_length(md: &Md, w: &mut String) {
    let sums = md.run.sums;
    let _ = writeln!(w, "## Game length, margin and end\n");
    let _ = writeln!(w, "| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |");
    let _ = writeln!(w, "|---|---|---|---|---|---|---|");
    for s in sums {
        let ended: Vec<String> = s.ended.iter().map(|(k, v)| format!("{k} {v}")).collect();
        let _ = writeln!(
            w,
            "| {} | {:.2} ({:.2}, {}..{}) | {} | {} / {} | {} / {} | {:.2} | {:.2} / {:.2} |",
            md.short(s),
            s.rounds_mean,
            s.rounds_sd,
            s.rounds_min,
            s.rounds_max,
            ended.join(", "),
            opt(s.win_deck_left, 2),
            opt(s.win_supply_left, 2),
            opt(s.loss_blockers, 1),
            opt(s.loss_level, 2),
            s.supply_left_mean,
            s.cities_corrupted_mean,
            s.cities_aligned_mean
        );
    }
}

fn decisions(md: &Md, w: &mut String) {
    let spec = md.run.spec;
    let sums = md.run.sums;
    let _ = writeln!(
        w,
        "\n## Decisions, adversary operations and counters\n"
    );
    let _ = writeln!(w, "| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | pays/game | withdraws/game | threats countered/dealt per game |");
    let _ = writeln!(w, "|---|---|---|---|---|---|---|---|---|---|");
    for s in sums {
        let _ = writeln!(
            w,
            "| {} | {:.1} | {:.2} | {} | {} | {} | {:.2} ({}) | {:.2} | {:.2} | {:.2} / {:.2} ({}) |",
            md.short(s),
            s.decisions_mean,
            s.options_per_decision,
            pct(s.forced_share),
            if spec.closeness.every > 0 { pct(s.close_share) } else { "–".into() },
            if spec.closeness.every > 0 { format!("{:.1}", s.gap_mean) } else { "–".into() },
            s.adversary_ops_per_round,
            s.adversary_ops_max_round,
            s.pays_per_game,
            s.withdraws_per_game,
            s.threats_countered_per_game,
            s.threats_dealt_per_game,
            pct(s.countered_share)
        );
    }
}

fn win_probability(md: &Md, w: &mut String) {
    let spec = md.run.spec;
    let sums = md.run.sums;
    if spec.winprob.games > 0 {
        let _ = writeln!(w, "\n## Win probability by round\n");
        let _ = writeln!(w, "Mean estimated win probability at the start of each round (games still running in brackets). Uncertain: checkpoints with 0.1 < p < 0.9. Early lock-in: games whose result was already decided (p ≤ 0.05 for a loss, ≥ 0.95 for a win, to the end) by round 2.\n");
        let maxr = sums
            .iter()
            .map(|s| s.winprob_by_round.len())
            .max()
            .unwrap_or(0);
        let mut head = String::from("| cell | uncertain | early lock-in |");
        let mut sep = String::from("|---|---|---|");
        for r in 1..=maxr {
            let _ = write!(head, " r{r} |");
            sep.push_str("---|");
        }
        let _ = writeln!(w, "{head}\n{sep}");
        for s in sums {
            let mut row = format!(
                "| {} | {} | {} |",
                md.short(s),
                s.uncertain_share.map_or("–".into(), pct),
                s.early_lock_in_share.map_or("–".into(), pct)
            );
            for r in 0..maxr {
                match s.winprob_by_round.get(r) {
                    Some((p, n)) if *n > 0 => {
                        let _ = write!(row, " {p:.2} ({n}) |");
                    }
                    _ => row.push_str(" |"),
                }
            }
            let _ = writeln!(w, "{row}");
        }
    }
}

fn luck(md: &Md, w: &mut String) {
    let run = md.run;
    let _ = writeln!(w, "\n## Luck share\n");
    let _ = writeln!(w, "Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.\n");
    let _ = writeln!(
        w,
        "| group | win: luck / skill / rest | survival: luck / skill / rest |"
    );
    let _ = writeln!(w, "|---|---|---|");
    for g in run.groups {
        let f = |x: Option<[f64; 3]>| {
            x.map_or("– (no variance)".into(), |v| {
                format!("{} / {} / {}", pct(v[0]), pct(v[1]), pct(v[2]))
            })
        };
        let _ = writeln!(
            w,
            "| {}d{} | {} | {} |",
            md.prefix(&g.config, &g.variant),
            g.difficulty,
            f(g.luck_win),
            f(g.luck_survival)
        );
    }
}

fn build_variety(md: &Md, w: &mut String) {
    let sums = md.run.sums;
    let _ = writeln!(w, "\n## Build variety\n");
    let _ = writeln!(w, "Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.\n");
    let _ = writeln!(w, "| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |");
    let _ = writeln!(w, "|---|---|---|---|---|");
    for s in sums {
        let _ = writeln!(
            w,
            "| {} | {} of {} | {:.2} | {} of {} | {} |",
            md.short(s),
            s.distinct_builds,
            s.games,
            s.build_distance,
            s.distinct_builds_wins,
            s.wins,
            opt(s.build_distance_wins, 2)
        );
    }
}

/// Resources left at game end (068): per cell and seat, each mean with a `!`
/// if it passes [`UNSPENT_HIGH`], and the flagged ones in a line.
fn unspent(md: &Md, w: &mut String) {
    let sums = md.run.sums;
    if sums.iter().all(|s| s.unspent.is_empty()) {
        return;
    }
    let names: Vec<String> = UNSPENT.iter().map(|r| r.name().to_string()).collect();
    let high: Vec<String> = names.iter().zip(UNSPENT_HIGH).map(|(n, h)| format!("{n} {h}")).collect();
    let _ = writeln!(w, "\n## Unspent resources\n");
    let _ = writeln!(w, "Mean left at game end, per seat ({}). `!`: over the flag, a seat's mean above {}.\n", names.join(" / "), high.join(", "));
    // Seats follow the config's countries, which can differ between cells.
    let most = sums.iter().map(|s| s.unspent.len()).max().unwrap_or(0);
    let seat_heads: String = (0..most).map(|i| format!(" seat {i} |")).collect();
    let _ = writeln!(w, "| cell | agent |{seat_heads}");
    let _ = writeln!(w, "|---|---|{}", "---|".repeat(most));
    let mut flagged = 0;
    for s in sums {
        let mut seats: Vec<String> = s
            .unspent
            .iter()
            .map(|seat| {
                let parts: Vec<String> = seat
                    .iter()
                    .zip(UNSPENT_HIGH)
                    .map(|(x, h)| {
                        flagged += (*x > h) as usize;
                        format!("{x:.1}{}", if *x > h { "!" } else { "" })
                    })
                    .collect();
                parts.join(" / ")
            })
            .collect();
        seats.resize(most, String::new());
        let cell = md.prefix(&s.config, &s.variant);
        let _ = writeln!(w, "| {cell}d{} | {} | {} |", s.difficulty, s.agent, seats.join(" | "));
    }
    let _ = writeln!(w, "\n{flagged} seat means over the flag.\n");
    seat_table(
        md,
        w,
        "Held after the last completed action phase",
        &format!("Mean per seat ({}); unlike the table above, never what a seat holds mid-round when a game ends in one (121).", names.join(" / ")),
        |s| &s.held_after,
    );
    let peaks = PEAKS.join(" / ");
    seat_table(
        md,
        w,
        "Peak holdings",
        &format!("Mean of each seat's highest holding after any action ({peaks}): the runaway check (121; `scripts/cards/runaway_read.py` sets it against a round's income)."),
        |s| &s.peaks,
    );
    seat_table(md, w, "Peak holdings, highest game", &format!("The highest peak over the cell's games ({peaks})."), |s| &s.peaks_max);
}

/// A per-seat table of one summary field, each cell its columns joined by `/`.
fn seat_table(md: &Md, w: &mut String, head: &str, what: &str, field: impl Fn(&CellSummary) -> &Vec<Vec<f64>>) {
    let sums = md.run.sums;
    if sums.iter().all(|s| field(s).is_empty()) {
        return;
    }
    let _ = writeln!(w, "\n## {head}\n\n{what}\n");
    let most = sums.iter().map(|s| field(s).len()).max().unwrap_or(0);
    let seat_heads: String = (0..most).map(|i| format!(" seat {i} |")).collect();
    let _ = writeln!(w, "| cell | agent |{seat_heads}");
    let _ = writeln!(w, "|---|---|{}", "---|".repeat(most));
    for s in sums {
        let mut seats: Vec<String> = field(s)
            .iter()
            .map(|seat| seat.iter().map(|x| format!("{x:.1}")).collect::<Vec<_>>().join(" / "))
            .collect();
        seats.resize(most, String::new());
        let cell = md.prefix(&s.config, &s.variant);
        let _ = writeln!(w, "| {cell}d{} | {} | {} |", s.difficulty, s.agent, seats.join(" | "));
    }
}

fn strategy_health(md: &Md, w: &mut String) {
    let run = md.run;
    let spec = md.run.spec;
    let sums = md.run.sums;
    let _ = writeln!(w, "\n## Strategy health\n");
    let _ = writeln!(w, "Per game, over the whole team: buildings played (any tier), tier-2 cards played (buildings and programs), all cards played, economy buildings (printing standing income) standing at the end; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.\n");
    let order = agent_order(run.cells);
    let mut gkeys: Vec<(String, String)> = Vec::new();
    for c in run.cells {
        let k = (c.config_path.clone(), c.variant.clone());
        if !gkeys.contains(&k) {
            gkeys.push(k);
        }
    }
    for &d in &spec.difficulties {
        let cell_of = |a: &str, k: &(String, String)| {
            sums.iter()
                .find(|s| s.difficulty == d && s.agent == a && s.config == k.0 && s.variant == k.1)
        };
        if gkeys.len() == 1 {
            let _ = writeln!(w, "| difficulty {d} | buildings played/game | tier 2 played/game | cards played/game | economy buildings standing/game | wins with 2+ buildings played (standing) |");
            let _ = writeln!(w, "|---|---|---|---|---|---|");
            for a in &order {
                let l = a.label();
                if let Some(s) = cell_of(&l, &gkeys[0]) {
                    let _ = writeln!(w, "| {l} | {} |", health_row(s, " | ", " | "));
                }
            }
        } else {
            let mut head = format!(
                "| difficulty {d}: buildings / tier 2 / cards / economy standing per game, wins with 2+ buildings |"
            );
            let mut sep = String::from("|---|");
            for (c, v) in &gkeys {
                let _ = write!(head, " {}{} |", md.cfg(c), v);
                sep.push_str("---|");
            }
            let _ = writeln!(w, "{head}\n{sep}");
            for a in &order {
                let l = a.label();
                let mut row = format!("| {l} |");
                for k in &gkeys {
                    match cell_of(&l, k) {
                        Some(s) => {
                            let _ = write!(row, " {} |", health_row(s, " / ", ", "));
                        }
                        None => row.push_str(" |"),
                    }
                }
                let _ = writeln!(w, "{row}");
            }
        }
        let _ = writeln!(w);
    }
    unspent(md, w);
}

fn card_rates(md: &Md, w: &mut String) {
    let run = md.run;
    let _ = writeln!(w, "## Pick and play rates per card\n");
    let _ = writeln!(w, "Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: keeps per offer; keeps count the opening deal and offers don't, so it can pass 100%. Play: played per card gained (kept or drawn). Per variant, config and difficulty in `cards.csv`.\n");
    let mut order: Vec<&String> = run.cards.keys().collect();
    order.sort_by_key(|l| {
        run.cells
            .iter()
            .find(|c| c.agent.label() == **l)
            .map_or(0, |c| c.agent.tier())
    });
    let mut head = String::from("| card |");
    let mut sep = String::from("|---|");
    for l in &order {
        let _ = write!(head, " {l}: pick (offered) / keeps per offer / play |");
        sep.push_str("---|");
    }
    let _ = writeln!(w, "{head}\n{sep}");
    let all_cards: BTreeSet<&String> = run.cards.values().flat_map(|t| t.keys()).collect();
    for card in all_cards {
        let mut row = format!("| {card} |");
        for l in &order {
            let n = run.cards[*l].get(card).copied().unwrap_or_default();
            let r = |a: u32, b: u32| {
                if b == 0 {
                    "–".to_string()
                } else {
                    pct(a as f64 / b as f64)
                }
            };
            // In hand by keeping or drawing.
            let play = r(n.played, n.kept + n.drawn);
            let pos = if n.drafted == 0 {
                "–".to_string()
            } else {
                format!("{:.1}", n.pick_sum as f64 / n.drafted as f64)
            };
            let _ = write!(
                row,
                " {} ({}) / {} / {} |",
                pos,
                n.offered,
                r(n.kept, n.offered),
                play
            );
        }
        let _ = writeln!(w, "{row}");
    }
}

fn timing(md: &Md, w: &mut String) {
    let run = md.run;
    let sums = md.run.sums;
    let _ = writeln!(w, "\n## Timing\n");
    let _ = writeln!(w, "Per game on one thread ({} threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.\n", run.threads);
    let _ = writeln!(
        w,
        "| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |"
    );
    let _ = writeln!(w, "|---|---|---|---|---|");
    for s in sums {
        let _ = writeln!(
            w,
            "| {} | {:.1} | {:.3} | {:.1} | {:.1} |",
            md.short(s),
            s.agent_ms_per_game,
            s.agent_ms_per_decision,
            s.probe_ms_per_game,
            s.ms_per_game
        );
    }
    let split: Vec<String> = time_split(run.games)
        .iter()
        .map(|(n, share)| format!("{n} {:.1}%", 100.0 * share))
        .collect();
    let _ = writeln!(
        w,
        "\nWhere the games' time went (summed over workers): {}. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.",
        split.join(", ")
    );
}

fn saved_records(md: &Md, w: &mut String) {
    let run = md.run;
    let _ = writeln!(w, "\n## Saved records\n");
    // 118: a spec with records off read "None matched", as if no game had qualified.
    if run.spec.records == 0 {
        let _ = writeln!(w, "Records are off in this spec (`records`: 0).");
    } else if run.records.is_empty() {
        let _ = writeln!(w, "None matched.");
    } else {
        let _ = writeln!(w, "Open in the viewer (*Open record…* or drop the file), or check with `aic-sim replay`.\n");
        for cat in CATEGORIES {
            let rs: Vec<&SavedRecord> = run.records.iter().filter(|r| r.category == cat).collect();
            let _ = writeln!(
                w,
                "**{cat}**{}\n",
                if rs.is_empty() { ": none matched" } else { "" }
            );
            for r in rs {
                let rel = r
                    .path
                    .strip_prefix(&format!("{}/", run.out_dir))
                    .unwrap_or(&r.path);
                let _ = writeln!(w, "- [`{rel}`]({rel}): {}", r.why);
            }
            let _ = writeln!(w);
        }
    }
}

/// The gate's section.
pub fn gate_section(s: &GateState) -> String {
    let mut w = String::new();
    let g = &s.gate;
    let _ = writeln!(w, "\n## Gate\n");
    let _ = writeln!(
        w,
        "Each check against its own agent on the same seeds; gap: the agent's win % minus the check's, ± one standard error (unpaired, as the gate tables have it). The verdict is the gap's sign at the cap of {} seeds. A check stops, looking at {} seeds and every {} after, once the gap clears {} standard errors (sign test) or the seeds left can't turn it back at that noise.\n",
        s.cap, g.min_games, g.every, g.z
    );
    let _ = writeln!(
        w,
        "{} of {} games ({:.0}%), {:.0} s of game time of about {:.0} s at the cap ({:.0}%). {}\n",
        s.games,
        s.games_cap,
        100.0 * s.games as f64 / s.games_cap.max(1) as f64,
        s.seconds,
        s.seconds_at_cap,
        100.0 * s.seconds / s.seconds_at_cap.max(1e-9),
        match (s.finished, s.passes, s.failed) {
            (false, ..) => "**Not finished.**".to_string(),
            (true, true, _) => "**Passes:** every check loses ground.".to_string(),
            (true, false, 0) => "**Does not pass:** see the ✗ rows.".to_string(),
            (true, false, n) => format!("**Does not pass:** {n} game(s) panicked, see *Failed games*."),
        }
    );
    let _ = writeln!(w, "| variant | d | check | games | stopped | win % | gap to its agent | paired se |");
    let _ = writeln!(w, "|---|---|---|---|---|---|---|---|");
    // 044: with several configs, rows of one check differ only by config.
    let several = s.rows.iter().any(|r| r.config != s.rows[0].config);
    for r in &s.rows {
        let variant = if several { format!("{} ({})", r.variant, r.config) } else { r.variant.clone() };
        let stop = match r.decided {
            Some(Decided::ClearsNoise) => "clears the noise",
            Some(Decided::SignSettled) => "sign settled",
            None if r.games < r.cap => "running",
            None => "cap",
        };
        let _ = writeln!(
            w,
            "| {} | {} | {} | {} | {stop} | {:.0} | {:+.0} ±{:.0}{} | {:.0} |",
            variant,
            r.difficulty,
            r.check,
            r.games,
            100.0 * r.check_win_rate,
            100.0 * r.gap + 0.0,
            100.0 * r.se,
            if r.loses { "" } else { " ✗" },
            100.0 * r.paired_se
        );
    }
    w
}
