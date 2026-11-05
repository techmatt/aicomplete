//! The report's findings: plain statements read off the tables.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use super::markdown::{p_value, pct, pts};
use super::stats::{
    agent_order, mean, stopped_short, CellSummary, GroupSummary, HeadToHead, VariantDiff,
};
use super::Run;
use crate::experiment::CardCounts;

/// Plain statements read off the tables, against the targets in
/// `docs/principles.md`.
pub(super) fn findings(run: &Run) -> Vec<String> {
    let mut f = Vec::new();
    let sums = run.sums;
    let ds: BTreeSet<i64> = sums.iter().map(|s| s.difficulty).collect();
    let lo = *ds.iter().next().unwrap_or(&0);
    let hi = *ds.iter().last().unwrap_or(&0);
    let top_tier = sums.iter().map(|s| s.tier).max().unwrap_or(0);
    // "The strongest agent" below: the top agent by win rate (the ladder's
    // top tier is not always the strongest: 008's engine sits above greedy
    // and two agents share its tier), else the top tier.
    let top_label = run.top_agent.clone().map(|(l, _)| l);
    let strongest = |s: &CellSummary| match &top_label {
        Some(l) => s.agent == *l,
        None => s.tier == top_tier,
    };
    let any_win = sums.iter().any(|s| s.wins > 0);
    if !any_win {
        f.push(format!(
            "**No agent won a game** ({} games). Targets: at difficulty 0 moderately skilled play essentially always wins; at the top difficulty the strongest agent wins narrowly. The ladder can only be read off survival (rounds, margins) until the rules are tuned.",
            run.games.len()
        ));
    }
    for g in run.groups {
        let wr: Vec<String> = g
            .win_rates
            .iter()
            .map(|(a, p)| format!("{a} {:.0}%", 100.0 * p))
            .collect();
        let rr: Vec<String> = g
            .survival
            .iter()
            .map(|(a, r)| format!("{a} {r:.2}"))
            .collect();
        let head = format!(
            "Difficulty {}{}",
            g.difficulty,
            if run.spec.variants.len() > 1 || run.spec.configs.len() > 1 {
                format!(" ({}, {})", g.config, g.variant)
            } else {
                String::new()
            }
        );
        if any_win {
            f.push(format!(
                "{head}: win rates {}; skill gap {:+.0} pts; {}.",
                wr.join(", "),
                100.0 * g.skill_gap,
                if g.ordered {
                    "stronger agents never do worse"
                } else {
                    "**a stronger agent does worse than a weaker one**"
                }
            ));
        }
        f.push(format!(
            "{head}: survival (rounds; wins count extra) {}; {}.",
            rr.join(", "),
            if g.ordered_survival {
                "the ladder is ordered"
            } else {
                "**the ladder is not ordered on survival**"
            }
        ));
    }
    if ds.len() > 1 {
        // Mean over the configs and variants played at each difficulty.
        let gap = |d: i64, of: fn(&GroupSummary) -> f64| {
            mean(run.groups.iter().filter(|g| g.difficulty == d).map(of)).unwrap_or(0.0)
        };
        let win = |g: &GroupSummary| g.skill_gap;
        let survival = |g: &GroupSummary| g.skill_gap_survival;
        // Say when it blends several configs or variants (020: unlabeled).
        let slices = run.groups.iter().filter(|g| g.difficulty == lo).count();
        let over = if slices > 1 { format!(" (mean over {slices} configs/variants)") } else { String::new() };
        f.push(format!(
            "Skill gap from difficulty {lo} to {hi}{over}: {:+.0} → {:+.0} pts win rate, {:+.2} → {:+.2} survival (target: widens with difficulty).",
            100.0 * gap(lo, win),
            100.0 * gap(hi, win),
            gap(lo, survival),
            gap(hi, survival)
        ));
    }
    let tops: Vec<&CellSummary> = sums.iter().filter(|s| strongest(s)).collect();
    // One line per config and variant, named like the other findings (020: it took
    // the first cell at each difficulty, the first slice only, unlabeled).
    let mut slices: Vec<(&str, &str)> = Vec::new();
    for s in &tops {
        if !slices.contains(&(s.config.as_str(), s.variant.as_str())) {
            slices.push((s.config.as_str(), s.variant.as_str()));
        }
    }
    for (config, variant) in slices {
        let at = |d: i64| tops.iter().find(|s| s.config == config && s.variant == variant && s.difficulty == d);
        if let (Some(lo_s), Some(hi_s)) = (at(lo), at(hi)) {
            // 099: one difficulty read "7.3 rounds at difficulty 2, 7.3 at 2".
            if lo == hi {
                f.push(format!(
                    "Game length for the strongest agent ({}; {config}, {variant}): {:.1} rounds at difficulty {lo} (target 8–10).",
                    hi_s.agent, hi_s.rounds_mean
                ));
                continue;
            }
            f.push(format!(
                "Game length for the strongest agent ({}; {config}, {variant}): {:.1} rounds at difficulty {lo}, {:.1} at {hi} (target 8–10).",
                hi_s.agent, lo_s.rounds_mean, hi_s.rounds_mean
            ));
        }
    }
    let ends: BTreeMap<&str, usize> =
        sums.iter()
            .flat_map(|s| s.ended.iter())
            .fold(BTreeMap::new(), |mut m, (k, v)| {
                *m.entry(k.as_str()).or_insert(0) += v;
                m
            });
    let e: Vec<String> = ends.iter().map(|(k, v)| format!("{k} {v}")).collect();
    f.push(format!("Games ended by: {}.", e.join(", ")));
    let ops = mean(sums.iter().map(|s| s.adversary_ops_per_round)).unwrap_or(0.0);
    let ops_max = sums
        .iter()
        .map(|s| s.adversary_ops_max_round)
        .max()
        .unwrap_or(0);
    f.push(format!("Adversary operations: {ops:.1} per round on average across cells, at most {ops_max} in one round (target: low)."));
    if run.spec.closeness.every > 0 {
        let close = mean(tops.iter().map(|s| s.close_share)).unwrap_or(0.0);
        let forced = mean(tops.iter().map(|s| s.forced_share)).unwrap_or(0.0);
        f.push(format!(
            "Decisions (strongest agent, mean over its cells): {} forced, {} of real choices close (best two within {} points).",
            pct(forced),
            pct(close),
            run.spec.closeness.gap
        ));
    }
    if let Some(top) = run
        .cards
        .iter()
        .find(|(l, _)| sums.iter().any(|s| s.agent == **l && strongest(s)))
    {
        // Every dealt card is drafted by someone, so keeping is the choice.
        // `kept` counts the opening deal and `offered` does not, so this reads
        // high for a card the deal hands out, and the thresholds are rough.
        let rate = |n: &CardCounts| {
            if n.offered >= 10 {
                Some(n.kept as f64 / n.offered as f64)
            } else {
                None
            }
        };
        let mut dead: Vec<String> = top
            .1
            .iter()
            .filter(|(_, n)| rate(n).is_some_and(|r| r < 0.1))
            .map(|(c, _)| c.clone())
            .collect();
        let mut must: Vec<String> = top
            .1
            .iter()
            .filter(|(_, n)| rate(n).is_some_and(|r| r > 0.9))
            .map(|(c, _)| c.clone())
            .collect();
        dead.sort();
        must.sort();
        f.push(format!(
            "Cards ({}, offered at least 10 times): keeps per offer (opening deal included) under 10%: {}; over 90%: {}.",
            top.0,
            if dead.is_empty() {
                "none".into()
            } else {
                dead.join(", ")
            },
            if must.is_empty() {
                "none".into()
            } else {
                must.join(", ")
            }
        ));
    }
    if let Some(l) = mean(run.groups.iter().filter_map(|g| g.luck_survival.map(|v| v[0]))) {
        f.push(format!("Luck share of survival (seed's share of the variance): {} on average across groups (target: most variance from decisions).", pct(l)));
    }
    if run.spec.winprob.games > 0 {
        let lock = mean(sums.iter().filter_map(|s| s.early_lock_in_share)).unwrap_or(0.0);
        let unc = mean(sums.iter().filter_map(|s| s.uncertain_share)).unwrap_or(0.0);
        f.push(format!(
            "Win probability: {} of checkpoints uncertain, {} of games decided by round 2 (target: uncertain into the late game). Playouts use `{}`, so they measure that agent's chances.",
            pct(unc),
            pct(lock),
            run.spec.winprob.agent
        ));
    }
    let gives = mean(
        sums.iter()
            .filter(|s| strongest(s))
            .map(|s| s.gives_per_game),
    )
    .unwrap_or(0.0);
    // Over the cells that dealt threats (020: a threats-off cell's 0% halved it).
    let countered = mean(
        sums.iter()
            .filter(|s| strongest(s) && s.threats_dealt_per_game > 0.0)
            .map(|s| s.countered_share),
    )
    .unwrap_or(0.0);
    f.push(format!(
        "Help and counters (strongest agent, mean over its cells): {gives:.2} gifts per game; {} of threats countered.",
        pct(countered)
    ));
    f.extend(comparison_findings(run));
    f
}

/// Findings for strategy health, the head-to-head against greedy, the
/// variants and the removals.
fn comparison_findings(run: &Run) -> Vec<String> {
    let mut f = Vec::new();
    let sums = run.sums;
    let order = agent_order(run.cells);
    let base = run.spec.variants.first().map_or("", |v| v.name.as_str());
    let multi_c = run.spec.configs.len() > 1;
    let multi_v = run.spec.variants.len() > 1;
    let where_ = |c: &str| match (multi_c, multi_v) {
        (true, true) => format!(" ({c}, {base})"),
        (true, false) => format!(" ({c})"),
        (false, true) => format!(" ({base})"),
        _ => String::new(),
    };
    // Strategy health: the top agent of each difficulty (base variant).
    for c in &run.spec.configs {
        for &d in &run.spec.difficulties {
            let mut top: Option<&CellSummary> = None;
            // 044: a gate check stopped early can't be the top agent on its few seeds.
            let most = sums
                .iter()
                .filter(|s| s.config == *c && s.variant == base && s.difficulty == d)
                .map(|s| s.games)
                .max()
                .unwrap_or(0);
            for a in &order {
                let l = a.label();
                if let Some(s) = sums.iter().find(|s| {
                    s.config == *c && s.variant == base && s.difficulty == d && s.agent == l
                }) {
                    if stopped_short(s.games, most) {
                        continue;
                    }
                    // Ties go to the later agent.
                    match top {
                        Some(t) if t.win_rate > s.win_rate => {}
                        _ => top = Some(s),
                    }
                }
            }
            let Some(s) = top else { continue };
            f.push(format!(
                "Strategy health at difficulty {d}{}: the top agent there ({}, {:.0}% wins) plays {:.2} buildings and {:.2} tier-2 cards per game (of {:.1} cards); {} of its wins had 2+ buildings played ({} with 2+ standing at the end). {}; {}.",
                where_(c),
                s.agent,
                100.0 * s.win_rate,
                s.buildings_played_per_game,
                s.tier2_played_per_game,
                s.cards_played_per_game,
                s.wins_2plus_buildings_played.map_or("–".into(), pct),
                s.wins_2plus_buildings_standing.map_or("–".into(), pct),
                if s.buildings_played_per_game >= 2.0 {
                    "It builds (healthy: 2+ buildings per game)"
                } else {
                    "**It hardly builds** (under 2 buildings per game)"
                },
                if s.tier2_played_per_game >= 0.5 {
                    "it plays tier 2"
                } else {
                    "**it hardly plays tier 2** (under 0.5 per game)"
                }
            ));
        }
    }
    // Head to head: the top agent against greedy at the top difficulty.
    let h2h = run.head_to_head;
    if let (Some((top, hi)), Some(first)) = (run.top_agent.clone(), h2h.first()) {
        let gl = &first.greedy;
        let line = |h: &HeadToHead| {
            let p = &h.paired;
            format!(
                "{} {:.0}% vs greedy {:.0}% over {} seeds (only {} {}, only greedy {}), {} pts, sign test p {}: {}",
                h.agent,
                100.0 * p.a_win_rate,
                100.0 * p.b_win_rate,
                p.games,
                h.agent,
                p.only_a,
                p.only_b,
                pts(p.diff, p.diff_se),
                p_value(p.sign_p),
                h.verdict
            )
        };
        let at_top = |a: &str| -> Vec<&HeadToHead> {
            h2h.iter()
                .filter(|h| h.agent == a && h.difficulty == hi && h.variant == base)
                .collect()
        };
        if top == *gl {
            // Greedy is the top agent: report the best of the rest.
            let rest: Vec<&HeadToHead> = h2h
                .iter()
                .filter(|h| h.difficulty == hi && h.variant == base)
                .collect();
            let best = rest
                .iter()
                .copied()
                .fold(None::<&HeadToHead>, |b, h| match b {
                    Some(b) if h.paired.diff < b.paired.diff => Some(b),
                    _ => Some(h),
                });
            let mut s = format!(
                "Top agent ({top}: highest win rate at difficulty {hi}, ties to the later agent) is greedy itself"
            );
            if let Some(b) = best {
                let _ = write!(
                    s,
                    "; the closest other agent at difficulty {hi}{}: {}",
                    where_(&b.config),
                    line(b)
                );
            }
            f.push(format!("{s}."));
        } else {
            for h in at_top(&top) {
                f.push(format!(
                    "Top agent ({top}: highest win rate at difficulty {hi}, ties to the later agent) vs greedy at difficulty {hi}{}: {}.",
                    where_(&h.config),
                    line(h)
                ));
            }
        }
    }
    // Variants: each against the base for the top agent at each difficulty.
    if let Some((a, _)) = &run.top_agent {
        f.extend(per_slice(run, a, |v| {
            let p = &v.paired;
            Some(format!("{} {}", v.variant, pts(p.diff, p.diff_se)))
        }, |d, at, parts| {
            format!("Variants against the base ({base}) at difficulty {d}{at}, {a}, win rate difference in pts (bold: sign test p < 0.05): {parts}.")
        }));
        f.extend(per_slice(run, a, |v| {
            let p = &v.paired;
            (!v.off.is_empty()).then(|| format!("{} {}", v.off.join(" + "), pts(-p.diff, p.diff_se)))
        }, |d, at, parts| {
            format!("What a module is worth (base win rate minus with it off) at difficulty {d}{at}, {a}: {parts} pts.")
        }));
    }
    f
}

/// One line per config and difficulty where `part` gives agent `a` something:
/// each variant diff's `part` (bold when the sign test clears p < 0.05), joined
/// and put in a sentence by `line`, given the difficulty and where it was played.
fn per_slice(
    run: &Run,
    a: &str,
    part: impl Fn(&VariantDiff) -> Option<String>,
    line: impl Fn(i64, &str, &str) -> String,
) -> Vec<String> {
    let mut f = Vec::new();
    for c in &run.spec.configs {
        for &d in &run.spec.difficulties {
            let parts: Vec<String> = run
                .variant_diffs
                .iter()
                .filter(|v| v.config == *c && v.difficulty == d && v.agent == a)
                .filter_map(|v| {
                    let s = part(v)?;
                    Some(if v.paired.sign_p < 0.05 {
                        format!("**{s}**")
                    } else {
                        s
                    })
                })
                .collect();
            if !parts.is_empty() {
                let at = if run.spec.configs.len() > 1 { format!(" ({c})") } else { String::new() };
                f.push(line(d, &at, &parts.join("; ")));
            }
        }
    }
    f
}
