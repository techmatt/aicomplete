//! `run` and `compare`: seeded games for one or two configs, summarized
//! side by side and replay-checked, with `--save` picking records to keep;
//! `replay` checks saved records; `--trace` prints one game.

use std::collections::BTreeMap;
use std::process::exit;
use std::time::Instant;

use aic_core::{play, replay, Action, Config, Event, PlayedGame, Outcome, Rules, SavedGame};
use serde::Serialize;

use crate::agents::AgentSpec;
use crate::cli::{load, rules, Options};
use crate::experiment::{adversary_ops, end_reason};
use crate::report::file_stem;

/// `aic-sim run` and `aic-sim compare`.
pub fn run(o: &Options) {
    let mut sides = Vec::new();
    let mut records = Vec::new();
    for (i, path) in o.paths.iter().enumerate() {
        let own = if i == 0 { &o.set_a } else { &o.set_b };
        let overrides: Vec<&String> = o.set.iter().chain(own).collect();
        let config = load(path, &overrides);
        let r = rules(path, &config);
        if o.trace {
            trace(&r, &o.agent, o.seed, o.max_actions, o.json);
        }
        let label = if o.command == "run" {
            "run".to_string()
        } else {
            ["A", "B"][i].to_string()
        };
        let (summary, saved) = simulate(label, path, overrides, &r, o);
        sides.push(summary);
        records.push(saved);
    }
    if let Some(dir) = &o.save {
        let outside: Vec<String> = o
            .save_seeds
            .iter()
            .filter(|&&s| !o.in_range(s))
            .map(|s| s.to_string())
            .collect();
        if !outside.is_empty() {
            eprintln!(
                "warning: --save-seed {} outside the seeds played ({}..={}), not saved",
                outside.join(", "),
                o.seed,
                o.last_seed()
            );
        }
        for (side, saved) in sides.iter_mut().zip(&records) {
            let paths = write_records(dir, side, saved);
            if let Some(rep) = side.saved.as_mut() {
                rep.paths = paths;
            }
        }
    }
    let diff = if sides.len() == 2 {
        sides[0].config.diff(&sides[1].config)
    } else {
        Vec::new()
    };
    if o.json {
        let out = serde_json::json!({
            "games": o.games,
            "first_seed": o.seed.to_string(),
            "last_seed": o.last_seed().to_string(),
            "agent": o.agent.label(),
            "max_actions": o.max_actions,
            "rules_revision": aic_core::RULES_REVISION,
            "diff": diff,
            "results": sides,
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        print_table(&sides, &diff, o);
        print_saved(&sides);
    }
    let bad: u64 = sides.iter().map(|s| s.games - s.replay_ok).sum();
    if bad > 0 {
        eprintln!("REPLAY MISMATCH in {bad} game(s)");
        exit(1);
    }
}

/// `aic-sim replay`: checks each saved record.
pub fn replay_all(o: &Options) {
    let bad = o.paths.iter().filter(|p| !replay_record(p)).count();
    if bad > 0 {
        eprintln!(
            "{bad} of {} record(s) failed or had warnings",
            o.paths.len()
        );
        exit(1);
    }
}

/// Replays one saved record and prints what it found. True if it replays
/// with no warnings.
fn replay_record(path: &str) -> bool {
    let loaded = std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|t| SavedGame::from_json(&t));
    let rec = match loaded {
        Ok(r) => r,
        Err(e) => {
            println!("{path}: ERROR {e}");
            return false;
        }
    };
    match rec.replay() {
        Err(e) => {
            println!("{path}: seed {}: ERROR {e}", rec.seed);
            false
        }
        Ok((g, warnings)) => {
            println!(
                "{path}: seed {}, {}, round {}, {} actions, fingerprint {}{}",
                rec.seed,
                describe(g.outcome.as_ref(), rec.actions.len()),
                g.round,
                rec.actions.len(),
                g.fingerprint(),
                if warnings.is_empty() { ", ok" } else { "" }
            );
            if !rec.note.is_empty() {
                println!("  note: {}", rec.note);
            }
            for w in &warnings {
                println!("  WARNING {w}");
            }
            warnings.is_empty()
        }
    }
}

/// `win (unity_deck)`, `loss (discord_deck)`, `not over (100000 actions)`: a game with no
/// outcome hit the action cap, or (a viewer bug report) was saved mid-game
/// (020: both read "timeout").
fn describe(outcome: Option<&Outcome>, actions: usize) -> String {
    match outcome {
        Some(o) => format!("{} ({})", if o.won { "win" } else { "loss" }, o.reason),
        None => format!("not over ({actions} actions)"),
    }
}

/// The `--save-when` words that are no end reason: [`when_match`] reads them.
const SELECTORS: [&str; 4] = ["all", "win", "loss", "timeout"];

/// Which `--save-when` selector a game matches first, if any.
fn when_match<'a>(when: &'a [String], outcome: Option<&Outcome>) -> Option<&'a String> {
    when.iter().find(|w| match (w.as_str(), outcome) {
        ("all", _) => true,
        ("win", Some(o)) => o.won,
        ("loss", Some(o)) => !o.won,
        ("timeout", None) => true,
        (code, Some(o)) => o.reason == code,
        _ => false,
    })
}

/// Writes one side's records into `dir`; exits 2 on any failure.
fn write_records(dir: &str, side: &Summary, saved: &[SavedGame]) -> Vec<String> {
    let fail = |what: &str, e: std::io::Error| -> ! {
        crate::cli::fail(&format!("--save {dir}: cannot write {what}: {e}"))
    };
    std::fs::create_dir_all(dir).unwrap_or_else(|e| fail("the directory", e));
    let stem = file_stem(&side.name);
    // `dir/name` rather than Path::join, so the paths printed on Windows
    // don't mix separators.
    let base = match dir.trim_end_matches(['/', '\\']) {
        "" => dir,
        d => d,
    };
    let sep = if base.ends_with(['/', '\\']) { "" } else { "/" };
    saved
        .iter()
        .map(|rec| {
            let file = format!("{base}{sep}{stem}-{}-seed{}.json", side.label, rec.seed);
            std::fs::write(&file, rec.to_text()).unwrap_or_else(|e| fail(&file, e));
            file
        })
        .collect()
}

fn print_saved(sides: &[Summary]) {
    for s in sides {
        let Some(rep) = &s.saved else { continue };
        let mut why = Vec::new();
        if !rep.when.is_empty() {
            let capped = if rep.kept_by_when < rep.matched {
                format!(", first {} kept (--save-max {})", rep.kept_by_when, rep.max)
            } else {
                String::new()
            };
            why.push(format!(
                "--save-when {}: {} matched{capped}",
                rep.when.join(","),
                rep.matched
            ));
        }
        if !rep.seeds.is_empty() {
            why.push(format!("--save-seed {}", rep.seeds.join(",")));
        }
        if why.is_empty() {
            why.push("no --save-seed in the seeds played".to_string());
        }
        println!(
            "\nsaved {}: {} record(s) written ({})",
            s.label,
            rep.paths.len(),
            why.join("; ")
        );
        for p in &rep.paths {
            println!("  {p}");
        }
    }
}

/// Everything aggregated for one config.
#[derive(Serialize)]
struct Summary {
    label: String,
    path: String,
    overrides: Vec<String>,
    name: String,
    config_hash: String,
    content_hash: String,
    engine_version: String,
    #[serde(skip)]
    config: Config,
    games: u64,
    wins: u64,
    win_rate: f64,
    win_rate_se: f64,
    rounds_mean: f64,
    rounds_min: u32,
    rounds_max: u32,
    /// Games by end reason; `timeout` when the action cap hit first.
    ended: BTreeMap<String, u64>,
    /// Games by the round they ended in.
    end_round: BTreeMap<u32, u64>,
    decisions_mean: f64,
    options_per_decision: f64,
    forced_share: f64,
    max_options: u32,
    adversary_ops_per_round: f64,
    adversary_ops_max_round: u32,
    /// Adversary operations during setup, mean per game.
    setup_ops_per_game: f64,
    /// Mean per game, by counter name; a `*_round` counter is the mean over the
    /// games that tallied it.
    counters: BTreeMap<String, f64>,
    replay_ok: u64,
    ms_per_game: f64,
    /// With `--save`: what was selected and where it went.
    #[serde(skip_serializing_if = "Option::is_none")]
    saved: Option<SaveReport>,
}

#[derive(Serialize)]
struct SaveReport {
    /// The `--save-when` selectors (empty when only `--save-seed` was given).
    when: Vec<String>,
    /// Games any `--save-when` selector matched.
    matched: u64,
    /// `--save-max`.
    max: usize,
    /// Of `matched`, the first `max` in seed order.
    kept_by_when: u64,
    /// `--save-seed` seeds within the range played.
    seeds: Vec<String>,
    /// The files written, one per record.
    paths: Vec<String>,
}

struct Played {
    record: PlayedGame,
    replay_ok: bool,
    /// A `--save-when` selector matched.
    when: bool,
}

/// A game kept for `--save`, before the global `--save-max` cut.
struct Kept {
    by_when: bool,
    by_seed: bool,
    record: SavedGame,
}

/// The note on every saved record, either side of the game's own words: what
/// was run, and its overrides.
fn save_note(o: &Options, label: &str, overrides: &[&String]) -> (String, String) {
    let side = if o.command == "compare" {
        format!(" (config {label})")
    } else {
        String::new()
    };
    let head = format!("aic-sim {} {}{side}", o.command, o.paths.join(" "));
    let tail = if overrides.is_empty() {
        String::new()
    } else {
        format!(
            ", overrides: {}",
            overrides
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    };
    (head, tail)
}

/// Plays one seed and replay-checks it, keeping it in `kept` if `--save`
/// picks it; `when_seen` counts this chunk's `--save-when` matches.
fn play_seed(
    r: &Rules,
    rebuilt: &Rules,
    o: &Options,
    (note_head, note_tail): (&str, &str),
    seed: u64,
    when_seen: &mut usize,
    kept: &mut Vec<Kept>,
) -> Played {
    let saving = o.save.is_some();
    let (g, mut record) = play(r, seed, &mut *o.agent.build(r, seed), o.max_actions);
    let replay_ok = replay(rebuilt, seed, &record.actions).is_ok_and(|h| h == g);
    let w = if saving {
        when_match(&o.save_when, record.outcome.as_ref())
    } else {
        None
    };
    let by_seed = saving && o.save_seeds.binary_search(&seed).is_ok();
    let by_when = w.is_some() && *when_seen < o.save_max;
    *when_seen += w.is_some() as usize;
    if by_when || by_seed {
        let why = match w {
            Some(w) if !by_seed => format!("--save-when {w}"),
            _ => "--save-seed".to_string(),
        };
        let note = format!(
            "{note_head}: {}, seed {seed}, picked by {why}{note_tail}",
            describe(record.outcome.as_ref(), record.actions.len()),
        );
        // The summary needs no actions; the record does.
        let actions = std::mem::take(&mut record.actions);
        kept.push(Kept {
            by_when: w.is_some(),
            by_seed,
            record: SavedGame::new(r, seed, actions, &g, &note),
        });
    }
    Played {
        record,
        replay_ok,
        when: w.is_some(),
    }
}

/// Plays every seed on `--threads` workers, each a run of consecutive seeds.
fn play_seeds(r: &Rules, rebuilt: &Rules, o: &Options, notes: (&str, &str)) -> (Vec<Played>, Vec<Kept>) {
    let seeds: Vec<u64> = (o.seed..=o.last_seed()).collect();
    let chunk = seeds.len().div_ceil(o.threads);
    std::thread::scope(|s| {
        let handles: Vec<_> = seeds
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    // Each chunk is a run of consecutive seeds, so the
                    // global first `save_max` matches are among each
                    // chunk's first `save_max`: keeping only those keeps
                    // the result independent of `--threads`.
                    let mut kept = Vec::new();
                    let mut when_seen = 0usize;
                    let played: Vec<Played> = part
                        .iter()
                        .map(|&seed| play_seed(r, rebuilt, o, notes, seed, &mut when_seen, &mut kept))
                        .collect();
                    (played, kept)
                })
            })
            .collect();
        let mut all = (Vec::new(), Vec::new());
        for h in handles {
            let (p, k) = h.join().expect("worker");
            all.0.extend(p);
            all.1.extend(k);
        }
        all
    })
}

/// The global `--save-max` cut, over games in seed order.
fn cap_saved(kept: Vec<Kept>, max: usize) -> Vec<SavedGame> {
    let mut rank = 0usize;
    kept.into_iter()
        .filter(|k| {
            let keep = k.by_seed || (k.by_when && rank < max);
            rank += k.by_when as usize;
            keep
        })
        .map(|k| k.record)
        .collect()
}

/// A reason code nobody ended with is most likely a typo: say so.
fn warn_unmatched_when(o: &Options, label: &str, played: &[Played]) {
    let reasons: std::collections::BTreeSet<&str> = played
        .iter()
        .filter_map(|p| p.record.outcome.as_ref().map(|o| o.reason.as_str()))
        .collect();
    for w in &o.save_when {
        if !SELECTORS.contains(&w.as_str()) && !reasons.contains(w.as_str())
        {
            let seen: Vec<&str> = reasons.iter().copied().collect();
            eprintln!(
                // 020: said "known:", as if a reason no game met (unity_deck, deck_empty) were unknown.
                "warning: --save-when {w:?} matched no game ({label}); a typo? Selectors: {}, or an end reason; these games ended by: {}",
                SELECTORS.join(", "),
                seen.join(", ")
            );
        }
    }
}

fn save_report(o: &Options, matched: u64) -> SaveReport {
    SaveReport {
        when: o.save_when.clone(),
        matched,
        max: o.save_max,
        kept_by_when: matched.min(o.save_max as u64),
        seeds: o
            .save_seeds
            .iter()
            .filter(|&&s| o.in_range(s))
            .map(|s| s.to_string())
            .collect(),
        paths: Vec::new(),
    }
}

/// One config's games, aggregated.
fn summarize(
    label: String,
    path: &str,
    overrides: Vec<&String>,
    r: &Rules,
    played: &[Played],
    elapsed: f64,
    saved: Option<SaveReport>,
) -> Summary {
    let n = played.len() as f64;
    let wins = played
        .iter()
        .filter(|p| p.record.outcome.as_ref().is_some_and(|o| o.won))
        .count() as u64;
    let p = wins as f64 / n;
    let mut ended = BTreeMap::new();
    let mut end_round = BTreeMap::new();
    let mut counters: BTreeMap<String, f64> = BTreeMap::new();
    // Games that tallied each counter: a `*_round` counter ("the round of the first
    // corruption") is a mean over the games that reached it (020: averaged over all
    // games, a game without one counted as round 0).
    let mut reached: BTreeMap<String, f64> = BTreeMap::new();
    let (mut rounds, mut decisions, mut options, mut forced, mut ops) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut rmin, mut rmax, mut max_options, mut ops_max) = (u32::MAX, 0u32, 0u32, 0u32);
    let mut setup_ops = 0u64;
    for g in played {
        let rec = &g.record;
        *ended.entry(end_reason(rec.outcome.as_ref())).or_insert(0) += 1;
        *end_round.entry(rec.rounds).or_insert(0) += 1;
        rounds += rec.rounds as u64;
        rmin = rmin.min(rec.rounds);
        rmax = rmax.max(rec.rounds);
        decisions += rec.decisions as u64;
        options += rec.options_total;
        forced += rec.forced as u64;
        max_options = max_options.max(rec.max_options);
        let (setup, rounds_ops) = adversary_ops(&rec.stats);
        ops += rounds_ops.iter().map(|&x| x as u64).sum::<u64>();
        ops_max = ops_max.max(rounds_ops.iter().copied().max().unwrap_or(0) as u32);
        setup_ops += setup as u64;
        for (k, v) in &rec.stats.counters {
            *counters.entry(k.to_string()).or_insert(0.0) += *v as f64;
            *reached.entry(k.to_string()).or_insert(0.0) += 1.0;
        }
    }
    for (k, v) in counters.iter_mut() {
        *v /= if is_round_counter(k) {
            reached[k]
        } else if let Some((r, tally)) = per_round_counter(k) {
            // A round's own counter, per game that reached that round (020: games that
            // had ended counted as 0, so the curves fell away late in the game). A level
            // tallied every round (`adversary.ambassadors_r06`) is over the games that
            // tallied it (a game can end in round 6 before the tally); an event count
            // (`alignment.tip_unity.r03`, tallied only when it happens) over the games
            // that reached the round.
            if tally == RoundTally::Level {
                reached[k]
            } else {
                played.iter().filter(|g| g.record.rounds >= r).count().max(1) as f64
            }
        } else {
            n
        };
    }
    Summary {
        label,
        path: path.to_string(),
        overrides: overrides.into_iter().cloned().collect(),
        name: r.config.name.clone(),
        config_hash: r.config_hash.clone(),
        content_hash: r.content_hash.to_string(),
        engine_version: aic_core::VERSION.to_string(),
        config: (*r.config).clone(),
        games: played.len() as u64,
        wins,
        win_rate: p,
        win_rate_se: (p * (1.0 - p) / n).sqrt(),
        rounds_mean: rounds as f64 / n,
        rounds_min: rmin,
        rounds_max: rmax,
        ended,
        end_round,
        decisions_mean: decisions as f64 / n,
        options_per_decision: options as f64 / (decisions.max(1) as f64),
        forced_share: forced as f64 / (decisions.max(1) as f64),
        max_options,
        adversary_ops_per_round: ops as f64 / (rounds.max(1) as f64),
        adversary_ops_max_round: ops_max,
        setup_ops_per_game: setup_ops as f64 / n,
        counters,
        replay_ok: played.iter().filter(|p| p.replay_ok).count() as u64,
        ms_per_game: elapsed * 1000.0 / n,
        saved,
    }
}

fn simulate(
    label: String,
    path: &str,
    overrides: Vec<&String>,
    r: &Rules,
    o: &Options,
) -> (Summary, Vec<SavedGame>) {
    // Replays go through rules rebuilt from the resolved config, as a
    // saved state would: proof that config + seed + actions is enough.
    let rebuilt = Rules::new(&r.config).expect("a resolved config is valid");
    let (note_head, note_tail) = save_note(o, &label, &overrides);
    let start = Instant::now();
    let (mut played, mut kept) = play_seeds(r, &rebuilt, o, (&note_head, &note_tail));
    let elapsed = start.elapsed().as_secs_f64();
    played.sort_by_key(|p| p.record.seed);
    kept.sort_by_key(|k| k.record.seed);
    let saved_games = cap_saved(kept, o.save_max);
    let matched = played.iter().filter(|p| p.when).count() as u64;
    warn_unmatched_when(o, &label, &played);
    let saved = o.save.is_some().then(|| save_report(o, matched));
    let summary = summarize(label, path, overrides, r, &played, elapsed, saved);
    (summary, saved_games)
}

fn print_table(sides: &[Summary], diff: &[String], o: &Options) {
    let mut rows: Vec<(String, Vec<String>)> = Vec::new();
    let mut row = |name: &str, f: &dyn Fn(&Summary) -> String| {
        rows.push((name.to_string(), sides.iter().map(f).collect()))
    };
    row("config", &|s| s.path.clone());
    row("name", &|s| s.name.clone());
    row("config hash", &|s| s.config_hash.clone());
    row("games", &|s| s.games.to_string());
    row("win %", &|s| {
        format!("{:.1} ±{:.1}", 100.0 * s.win_rate, 100.0 * s.win_rate_se)
    });
    row("rounds mean", &|s| format!("{:.2}", s.rounds_mean));
    row("rounds min..max", &|s| {
        format!("{}..{}", s.rounds_min, s.rounds_max)
    });
    let mut reasons: Vec<&String> = sides.iter().flat_map(|s| s.ended.keys()).collect();
    reasons.sort();
    reasons.dedup();
    for r in reasons {
        row(&format!("ended: {r}"), &|s| {
            s.ended.get(r).copied().unwrap_or(0).to_string()
        });
    }
    let mut ends: Vec<u32> = sides
        .iter()
        .flat_map(|s| s.end_round.keys().copied())
        .collect();
    ends.sort();
    ends.dedup();
    for e in ends {
        row(&format!("ended in round {e}"), &|s| {
            s.end_round.get(&e).copied().unwrap_or(0).to_string()
        });
    }
    row("decisions/game", &|s| format!("{:.1}", s.decisions_mean));
    row("options/decision", &|s| {
        format!("{:.2}", s.options_per_decision)
    });
    row("forced decisions %", &|s| {
        format!("{:.1}", 100.0 * s.forced_share)
    });
    row("max options", &|s| s.max_options.to_string());
    row("adversary ops/round", &|s| {
        format!("{:.2}", s.adversary_ops_per_round)
    });
    row("adversary ops max/round", &|s| {
        s.adversary_ops_max_round.to_string()
    });
    row("adversary ops at setup", &|s| {
        format!("{:.2}", s.setup_ops_per_game)
    });
    let mut names: Vec<&String> = sides.iter().flat_map(|s| s.counters.keys()).collect();
    names.sort();
    names.dedup();
    for k in names {
        let unit = if is_round_counter(k) {
            "mean where reached"
        } else if per_round_counter(k).is_some() {
            "/game reaching it"
        } else {
            "/game"
        };
        row(&format!("{k} {unit}"), &|s| {
            s.counters.get(k).map_or("-".into(), |v| format!("{v:.2}"))
        });
    }
    row("replay ok", &|s| format!("{}/{}", s.replay_ok, s.games));
    // Wall time over games, all threads at once (020: read as one game's cost).
    row("wall ms/game (all threads, incl. replay)", &|s| {
        format!("{:.3}", s.ms_per_game)
    });

    // 044: an overlay on one side gives it its own content hash.
    let content = if sides.iter().all(|s| s.content_hash == sides[0].content_hash) {
        sides[0].content_hash.to_string()
    } else {
        sides.iter().map(|s| format!("{} {}", s.label, s.content_hash)).collect::<Vec<_>>().join(", ")
    };
    println!(
        "seeds {}..={}, agent {} (max {} actions), content {}, engine {} rules r{}",
        o.seed,
        o.last_seed(),
        o.agent.label(),
        o.max_actions,
        content,
        aic_core::VERSION,
        aic_core::RULES_REVISION
    );
    for s in sides {
        if !s.overrides.is_empty() {
            println!("{} overrides: {}", s.label, s.overrides.join(" "));
        }
    }
    if !diff.is_empty() {
        println!("A -> B: {}", diff.join("; "));
    }
    println!();
    let w0 = rows.iter().map(|r| r.0.chars().count()).max().unwrap_or(0);
    let widths: Vec<usize> = (0..sides.len())
        .map(|i| {
            rows.iter()
                .map(|r| r.1[i].chars().count())
                .max()
                .unwrap_or(0)
                .max(10)
        })
        .collect();
    let header: Vec<String> = sides
        .iter()
        .zip(&widths)
        .map(|(s, w)| format!("{:>w$}", s.label, w = w))
        .collect();
    println!("{:w0$}  {}", "", header.join("  "));
    for (name, vals) in &rows {
        let cells: Vec<String> = vals
            .iter()
            .zip(&widths)
            .map(|(v, w)| format!("{v:>w$}"))
            .collect();
        println!("{name:w0$}  {}", cells.join("  "));
    }
}

/// Prints one game as JSON lines: `{"action": ...}` then its events. To
/// stderr when `to_stderr` (with `--json`, so stdout stays one document).
fn trace(r: &Rules, agent: &AgentSpec, seed: u64, max_actions: usize, to_stderr: bool) {
    let out = |v: serde_json::Value| {
        if to_stderr {
            eprintln!("{v}")
        } else {
            println!("{v}")
        }
    };
    let mut events: Vec<Event> = Vec::new();
    let mut g = r.new_game_logged(seed, Some(&mut events));
    out(
        serde_json::json!({"trace": r.config.name, "seed": seed.to_string(), "config_hash": r.config_hash}),
    );
    let line = |e: &Event| out(serde_json::json!({ "event": e }));
    events.iter().for_each(line);
    let mut agent = agent.build(r, seed);
    for n in 0.. {
        let legal: Vec<Action> = r.legal_actions(&g);
        if legal.is_empty() {
            break;
        }
        // Same cap as driver::play, so the trace is the game the summary counts.
        if n == max_actions {
            out(serde_json::json!({ "timeout": true, "actions": n }));
            break;
        }
        let a = legal[aic_core::Agent::choose(&mut *agent, r, &g, &legal)].clone();
        out(serde_json::json!({ "action": a, "options": legal.len() }));
        events.clear();
        r.apply_mut(&mut g, &a, Some(&mut events)).expect("legal");
        events.iter().for_each(line);
    }
}

/// A counter that records a round ("the round of the first corruption"): averaged
/// over the games that reached it, not over all games.
fn is_round_counter(k: &str) -> bool {
    k.ends_with("_round")
}

/// How a per-round counter was tallied.
#[derive(Debug, PartialEq)]
enum RoundTally {
    /// Every round, a level (`_rNN`).
    Level,
    /// Only when it happens (`.rNN`).
    Event,
}

/// The round of a per-round counter and how it was tallied, from its `_rNN`
/// or `.rNN` suffix.
fn per_round_counter(k: &str) -> Option<(u32, RoundTally)> {
    let tail = k.rsplit(['_', '.']).next()?;
    let digits = tail.strip_prefix('r')?;
    if digits.len() != 2 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let tally = if k.strip_suffix(tail)?.ends_with('_') { RoundTally::Level } else { RoundTally::Event };
    Some((digits.parse().ok()?, tally))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 020: which counters average over the games that reached them.
    #[test]
    fn counters_that_average_over_reached_games() {
        assert!(is_round_counter("adversary.first_corruption_round"));
        assert!(!is_round_counter("board.war_rounds"));
        assert_eq!(per_round_counter("adversary.ambassadors_r06"), Some((6, RoundTally::Level)));
        assert_eq!(per_round_counter("alignment.tip_unity.r11"), Some((11, RoundTally::Event)));
        assert_eq!(per_round_counter("board.war_rounds"), None);
        assert_eq!(per_round_counter("adversary.botnets_peak.c0"), None);
    }
}
