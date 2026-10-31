//! The machine-readable files: `games.csv` and `cards.csv` (one row per
//! game, per agent × difficulty × variant × config × card) and
//! `results.json` (everything).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use aic_core::content_hash;

use super::stats::{economy_standing, plays, standing};
use super::Run;
use crate::experiment::{CardCounts, Cell, GameResult};

/// A CSV text field, quoted (with `"` doubled).
fn q(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

pub fn games_csv(cells: &[Cell], games: &[GameResult]) -> String {
    let mut s = String::from(
        "config,variant,difficulty,agent,seed,won,reason,rounds,decisions,options,forced,probed,close,gap_mean,deck_left,supply_left,supply_start,cities_corrupted_end,cities_aligned_end,blockers,level,unity_tips,adversary_ops,adversary_ops_max,setup_ops,gives,pays,withdraws,threats_dealt,threats_countered,corruptions,first_corruption_round,std_actions,war_destroyed,cards_played,buildings_played,tier2_played,buildings_standing,economy_buildings_standing,build,winprob,ms,replay_ok\n",
    );
    for g in games {
        let c = &cells[g.cell];
        let ops: u32 = g.adversary_ops.iter().sum();
        let wp: Vec<String> = g.winprob.iter().map(|p| format!("{p:.3}")).collect();
        let cnt = |k: &str| g.counters.get(k).copied().unwrap_or(0);
        let pl = plays(g, c.rules.world().content);
        let _ = writeln!(
            s,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.1},{}",
            q(&c.config_path),
            q(&c.variant),
            c.difficulty,
            q(&c.agent.label()),
            g.seed,
            g.won as u8,
            q(&g.reason),
            g.rounds,
            g.decisions,
            g.options_total,
            g.forced,
            g.probed,
            g.close,
            g.gap_sum / g.probed.max(1) as f64,
            g.margin.deck_left,
            g.margin.supply_left,
            g.margin.supply_start,
            g.margin.cities_corrupted_end,
            g.margin.cities_aligned_end,
            g.margin.blockers,
            g.margin.level,
            g.margin.unity_tips,
            ops,
            g.adversary_ops.iter().max().unwrap_or(&0),
            g.setup_ops,
            g.gives,
            g.pays,
            g.withdraws,
            cnt("threats.dealt"),
            cnt("threats.countered"),
            cnt("adversary.corruptions"),
            cnt("adversary.first_corruption_round"),
            cnt("board.std_actions"),
            cnt("board.war_destroyed"),
            pl[0],
            pl[1],
            pl[2],
            standing(&g.build),
            economy_standing(&g.build, c.rules.world().content),
            q(&g.build),
            wp.join(";"),
            g.ms,
            g.replay_ok as u8
        );
    }
    s
}

pub fn cards_csv(cells: &[Cell], games: &[GameResult]) -> String {
    type Key = (String, String, String, i64, String);
    let mut per: BTreeMap<Key, CardCounts> = BTreeMap::new();
    for g in games {
        let c = &cells[g.cell];
        for (card, n) in &g.cards {
            let key = (
                c.agent.label(),
                c.variant.clone(),
                c.config_path.clone(),
                c.difficulty,
                card.clone(),
            );
            *per.entry(key).or_default() += *n;
        }
    }
    let mut s = String::from(
        "agent,variant,config,difficulty,card,offered,drafted,kept,played,pick_position_sum,drawn\n",
    );
    for ((a, v, cfg, d, card), n) in per {
        let _ = writeln!(
            s,
            "{},{},{},{d},{card},{},{},{},{},{},{}",
            q(&a),
            q(&v),
            q(&cfg),
            n.offered,
            n.drafted,
            n.kept,
            n.played,
            n.pick_sum,
            n.drawn
        );
    }
    s
}

pub fn results_json(run: &Run) -> String {
    let vds = run.variant_diffs;
    // What a module is worth: the base's win rate minus the variant's that
    // switches it off, on the same seeds.
    let removals: Vec<serde_json::Value> = vds
        .iter()
        .filter(|v| !v.off.is_empty())
        .map(|v| {
            serde_json::json!({
                "config": v.config,
                "variant": v.variant,
                "modules": v.off,
                "difficulty": v.difficulty,
                "agent": v.agent,
                "base_win_rate": v.paired.b_win_rate,
                "off_win_rate": v.paired.a_win_rate,
                "worth": -v.paired.diff,
                "worth_se": v.paired.diff_se,
                "sign_p": v.paired.sign_p,
            })
        })
        .collect();
    let v = serde_json::json!({
        "format": "aic-experiment/2",
        "spec": run.spec,
        "wall_seconds": run.wall_s,
        "games_played": run.games.len(),
        "games_failed": run.failed,
        "games_planned": run.planned,
        "games_resumed": run.resumed,
        "threads": run.threads,
        "engine_version": aic_core::VERSION,
        "rules_revision": aic_core::RULES_REVISION,
        "content_hash": content_hash(),
        "cells": run.sums,
        "groups": run.groups,
        "top_agent": run.top_agent.as_ref().map(|(a, d)| serde_json::json!({"agent": a, "difficulty": d})),
        "head_to_head": run.head_to_head,
        "agent_pairs": run.agent_pairs,
        "variants": vds,
        "fresh_confirmation": run.fresh_confirmation,
        "removals": removals,
        "cards": run.cards,
        "records": run.records,
        "gate": run.gate,
    });
    serde_json::to_string_pretty(&v).expect("results serialize")
}
