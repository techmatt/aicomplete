//! The saved records: which games are worth a human look, written as game
//! records; and writing any report file whole.

use std::collections::BTreeSet;

use aic_core::SavedGame;
use serde::Serialize;

use super::stats::{lock_in_round, measured_rank};
use super::CellSummary;
use crate::experiment::{Cell, GameResult, Spec};

#[derive(Debug, Clone, Serialize)]
pub struct SavedRecord {
    pub category: String,
    pub path: String,
    pub why: String,
}

/// `s` with every character but ASCII letters, digits, `-` and `_` made `_`.
fn file_safe(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// A file-name-safe version of a config or experiment name: [`file_safe`],
/// at most 64 characters, and `game` if nothing but `_` is left.
pub fn file_stem(name: &str) -> String {
    let s: String = file_safe(name).chars().take(64).collect();
    if s.trim_matches('_').is_empty() {
        "game".to_string()
    } else {
        s
    }
}

const CLOSE_LOSS: &str = "close-loss";
const EARLY_LOCK_IN: &str = "early-lock-in";
const SURPRISING_WIN: &str = "surprising-win";
/// The record categories, each the start of its files' names.
pub(super) const CATEGORIES: [&str; 3] = [CLOSE_LOSS, EARLY_LOCK_IN, SURPRISING_WIN];

/// Picks and writes the records worth a look. Returns what was saved.
pub fn save_records(
    dir: &str,
    spec: &Spec,
    cells: &[Cell],
    games: &[GameResult],
    sums: &[CellSummary],
) -> Result<Vec<SavedRecord>, String> {
    let rec_dir = format!("{dir}/records");
    std::fs::create_dir_all(&rec_dir).map_err(|e| format!("{rec_dir}: {e}"))?;
    let top_d = cells.iter().map(|c| c.difficulty).max().unwrap_or(0);
    // Agents ranked by measured strength, not by ladder tier (011): the
    // ladder's top tier is not always the strongest agent.
    // An agent not measured yet (a partial report) ranks `None`, not weakest.
    let ranks = measured_rank(cells, sums);
    let rank = |c: &Cell| ranks.get(&c.agent.label()).copied();
    let top_rank = ranks.values().copied().max();
    let mut picked: Vec<(String, usize, String)> = Vec::new();
    let mut taken = BTreeSet::new();
    let mut take =
        |cat: &str, list: Vec<(usize, String)>, picked: &mut Vec<(String, usize, String)>| {
            let mut n = 0;
            for (i, why) in list {
                if n == spec.records {
                    break;
                }
                if taken.insert(i) {
                    picked.push((cat.to_string(), i, why));
                    n += 1;
                }
            }
        };
    // Close losses at the top difficulty: fewest pieces over the bar, then
    // the longest games, measured-stronger agents first.
    let mut close: Vec<(usize, &GameResult)> = games
        .iter()
        .enumerate()
        .filter(|(_, g)| !g.won && cells[g.cell].difficulty == top_d && g.reason != "timeout")
        .collect();
    close.sort_by_key(|(_, g)| {
        (
            g.margin.blockers,
            std::cmp::Reverse(g.rounds),
            std::cmp::Reverse(rank(&cells[g.cell])),
            g.seed,
        )
    });
    take(
        CLOSE_LOSS,
        close
            .iter()
            .map(|(i, g)| {
                (*i, format!(
                    "close loss at difficulty {top_d}: {} in round {}, {} adversary piece(s) over the level-{} bar, supply {} of {}, {} cities corrupted",
                    g.reason,
                    g.rounds,
                    g.margin.blockers,
                    g.margin.level,
                    g.margin.supply_left,
                    g.margin.supply_start,
                    g.margin.cities_corrupted_end
                ))
            })
            .collect(),
        &mut picked,
    );
    // Early lock-ins: decided by round 2 in the longest games.
    let mut lock: Vec<(usize, &GameResult, usize)> = games
        .iter()
        .enumerate()
        .filter_map(|(i, g)| Some((i, g, lock_in_round(g)?)))
        .filter(|(_, g, r)| *r <= 2 && g.rounds >= 3)
        .collect();
    lock.sort_by_key(|(_, g, r)| {
        (
            *r,
            std::cmp::Reverse(g.rounds),
            std::cmp::Reverse(rank(&cells[g.cell])),
            g.seed,
        )
    });
    take(
        EARLY_LOCK_IN,
        lock.iter()
            .map(|(i, g, r)| {
                let ps: Vec<String> = g.winprob.iter().map(|p| format!("{p:.2}")).collect();
                (
                    *i,
                    format!(
                        "outcome ({}) decided from round {r} of {}: win probability by round {}",
                        if g.won { "win" } else { "loss" },
                        g.rounds,
                        ps.join(" ")
                    ),
                )
            })
            .collect(),
        &mut picked,
    );
    // Surprising wins: by the weakest agent, by a weaker agent at the top
    // difficulty, or after the playouts gave them little chance.
    let mut surprise: Vec<(usize, String, (u8, u64))> = Vec::new();
    for (i, g) in games.iter().enumerate().filter(|(_, g)| g.won) {
        let c = &cells[g.cell];
        let low = g.winprob.iter().copied().fold(f64::INFINITY, f64::min);
        let why = if rank(c) == Some(0) && Some(0) < top_rank {
            Some((
                0,
                format!("won by the weakest agent as measured ({})", c.agent.label()),
            ))
        } else if low <= 0.25 {
            Some((1, format!("won after the playouts gave it {low:.2}")))
        } else if c.difficulty == top_d && rank(c).is_some() && rank(c) < top_rank {
            Some((
                2,
                format!(
                    "won at difficulty {top_d} by {}, below the top agent",
                    c.agent.label()
                ),
            ))
        } else {
            None
        };
        if let Some((order, w)) = why {
            surprise.push((i, w, (order, g.seed)));
        }
    }
    surprise.sort_by_key(|s| s.2);
    take(
        SURPRISING_WIN,
        surprise.into_iter().map(|(i, w, _)| (i, w)).collect(),
        &mut picked,
    );

    let mut out = Vec::new();
    for (cat, i, why) in picked {
        let g = &games[i];
        let c = &cells[g.cell];
        let Some(last) = g.final_state.as_deref() else {
            continue;
        };
        let note = format!(
            "aic-sim experiment {}: {cat}: {}, variant {} ({}), difficulty {}, agent {}, seed {}: {why}",
            spec.name,
            c.config_path,
            c.variant,
            c.overrides.join(" "),
            c.difficulty,
            c.agent.label(),
            g.seed
        );
        let rec = SavedGame::new(&c.rules, g.seed, g.actions.clone(), last, &note);
        // The cell number keeps two agents of one kind (or two configs of
        // one name) apart; the label says which agent it was.
        let label: String = file_safe(&c.agent.label()).chars().take(48).collect();
        let file = format!(
            "{rec_dir}/{cat}-c{}-{}-{}-d{}-{label}-seed{}.json",
            g.cell,
            file_safe(&c.rules.config.name),
            file_safe(&c.variant),
            c.difficulty,
            g.seed
        );
        write_whole(&file, &rec.to_text())?;
        out.push(SavedRecord {
            category: cat,
            path: file,
            why,
        });
    }
    Ok(out)
}

/// Removes the records an earlier report of this experiment saved that the
/// latest one (`keep`) did not, so none sit there unlisted. Called once the
/// latest report is in place.
pub fn prune_records(dir: &str, keep: &[SavedRecord]) -> Result<(), String> {
    let rec_dir = format!("{dir}/records");
    let keep: BTreeSet<std::path::PathBuf> = keep.iter().map(|r| r.path.clone().into()).collect();
    for e in std::fs::read_dir(&rec_dir)
        .map_err(|e| format!("{rec_dir}: {e}"))?
        .flatten()
    {
        let name = e.file_name().to_string_lossy().to_string();
        let ours = CATEGORIES.iter().any(|c| name.starts_with(&format!("{c}-")));
        let path = std::path::PathBuf::from(format!("{rec_dir}/{name}"));
        if ours && name.ends_with(".json") && !keep.contains(&path) {
            std::fs::remove_file(e.path()).map_err(|err| format!("{name}: {err}"))?;
        }
    }
    Ok(())
}

/// Writes `body` to `path` whole: into `path.tmp`, then renamed over
/// `path`, so a run killed mid-write never leaves half a file.
pub fn write_whole(path: &str, body: &str) -> Result<(), String> {
    let tmp = format!("{path}.tmp");
    std::fs::write(&tmp, body)
        .and_then(|_| std::fs::rename(&tmp, path))
        .map_err(|e| {
            let _ = std::fs::remove_file(&tmp);
            format!("cannot write {path}: {e}")
        })
}
