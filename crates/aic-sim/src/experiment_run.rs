//! `aic-sim experiment`: the driver around [`crate::experiment`]. Plays
//! the grid, streams every game to `games.jsonl` as it finishes, rewrites the
//! report from the games so far as it goes, and writes the cycle report.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use crate::agents::AgentSpec;
use crate::cli::{die, Options};
use crate::report::file_stem;
use crate::{experiment, gate, prof, report};

/// `aic-sim experiment`: plays the grid and writes the cycle report.
/// Returns the exit code: 1 if any game panicked or failed its replay check.
pub fn run_experiment(path: &str, o: &Options) -> i32 {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut spec: experiment::Spec =
        serde_json::from_str(&text).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    apply_flags(&mut spec, o);
    let cells = experiment::cells(&spec).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let dir = o
        .out
        .clone()
        .unwrap_or_else(|| format!("out/experiments/{}", file_stem(&spec.name)));
    let dir = dir.trim_end_matches(['/', '\\']).to_string();
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| die(&format!("--out {dir}: {e}")));
    let total = spec.total_games(&cells);
    // Every finished game goes to games.jsonl at once (a header line, then
    // one game per line), so a stopped or killed run keeps what it played;
    // --resume reads it back and plays only the rest.
    let stream_path = format!("{dir}/games.jsonl");
    let prior = if o.resume && std::path::Path::new(&stream_path).exists() {
        experiment::read_stream(&stream_path, &spec, &cells)
            .unwrap_or_else(|e| die(&format!("--resume: {e}")))
    } else {
        Vec::new()
    };
    let resumed = prior.len();
    let stream = std::sync::Mutex::new(open_stream(&stream_path, o.resume, &spec, &cells, &prior));
    // A gate stops its checks early: `games` is each one's cap.
    let at_most = if spec.gate.is_some() { "at most " } else { "" };
    eprintln!(
        "{}: {} cells x {}{} seeds = {}{total} games on {} threads{}; streaming to {stream_path}",
        spec.name,
        cells.len(),
        at_most,
        spec.games,
        at_most,
        o.threads,
        if resumed > 0 {
            format!(" ({resumed} already played)")
        } else {
            String::new()
        }
    );
    let start = Instant::now();
    let last = std::sync::Mutex::new(Instant::now());
    let on_game = |g: &experiment::GameResult, done: usize, all: usize| {
        let line = serde_json::to_string(g).expect("a game result serializes");
        {
            use std::io::Write;
            let mut w = stream.lock().expect("stream lock");
            if let Err(e) = writeln!(w, "{line}").and_then(|_| w.flush()) {
                eprintln!("cannot write {stream_path}: {e}");
                exit(2)
            }
        }
        let mut l = last.lock().expect("progress lock");
        if done == all || l.elapsed().as_secs() >= 10 {
            *l = Instant::now();
            let elapsed = start.elapsed().as_secs_f64();
            let played = done.saturating_sub(resumed).max(1) as f64;
            let left = all.saturating_sub(done) as f64 * elapsed / played / 60.0;
            // A gate's `all` is its cap, so for a gate this is the most it can take.
            eprintln!("  {done}/{all} games, {elapsed:.0} s, about {left:.0} min left at this rate");
        }
    };
    let failed_path = format!("{dir}/failed.jsonl");
    // A resumed run plays its failed games again, so only this run's are listed.
    let _ = std::fs::remove_file(&failed_path);
    let on_failed = |f: &experiment::FailedGame| {
        eprintln!("  GAME FAILED, the run goes on: {}: {}", f.command(), f.message);
        let line = serde_json::to_string(f).expect("a failed game serializes");
        let wrote = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&failed_path)
            .and_then(|mut w| std::io::Write::write_all(&mut w, format!("{line}\n").as_bytes()));
        if let Err(e) = wrote {
            eprintln!("cannot write {failed_path}: {e}");
        }
    };
    let results = std::sync::Mutex::new(prior);
    let failed = std::sync::Mutex::new(Vec::new());
    let reports = |games: &mut Vec<experiment::GameResult>, failed: &[experiment::FailedGame]| {
        experiment::sort(games);
        let wall_s = start.elapsed().as_secs_f64();
        write_reports(
            &dir, &spec, &cells, games, failed, wall_s, o.threads, total, resumed,
        )
    };
    let sinks = experiment::Sinks {
        results: &results,
        failed: &failed,
        on_game: &on_game,
        on_failed: &on_failed,
    };
    // Meanwhile, the report of the games finished so far.
    let report_so_far = || {
        let mut snap = results.lock().expect("results lock").clone();
        let lost = failed.lock().expect("failed lock").clone();
        // A file held open elsewhere (a CSV in a spreadsheet) skips this
        // report; it must not stop the run.
        if !snap.is_empty() && snap.len() < total {
            if let Err(e) = reports(&mut snap, &lost) {
                eprintln!("partial report skipped: {e}");
            }
        }
    };
    while_playing(o.report_every, report_so_far, || {
        experiment::run(&spec, &cells, o.threads, &sinks)
    });
    let mut games = results.into_inner().expect("results lock");
    let failed = failed.into_inner().expect("failed lock");
    let wall_s = start.elapsed().as_secs_f64();
    let report_start = Instant::now();
    let (sums, records) = reports(&mut games, &failed).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(2)
    });
    print_timing(&games, wall_s, report_start.elapsed().as_secs_f64());
    println!(
        "{} games in {wall_s:.1} s; report {dir}/report.md, {} record(s) in {dir}/records",
        games.len(),
        records
    );
    print_cells(&sums);
    let bad = games.iter().filter(|g| !g.replay_ok).count();
    if bad > 0 {
        eprintln!("REPLAY MISMATCH in {bad} game(s)");
    }
    if !failed.is_empty() {
        eprintln!("{} game(s) PANICKED: {failed_path}", failed.len());
        for f in &failed {
            eprintln!("  {}: {}", f.command(), f.message);
        }
    }
    (bad > 0 || !failed.is_empty()) as i32
}

/// Runs `play`, and `report` every `every` seconds (0: never) until it ends.
/// However it ends: a panic out of `play` that left the reporter polling would
/// keep the process, and its exe, alive with nothing running (046).
fn while_playing(every: u64, report: impl Fn() + Sync, play: impl FnOnce()) {
    struct Stop<'a>(&'a AtomicBool);
    impl Drop for Stop<'_> {
        fn drop(&mut self) {
            self.0.store(false, Ordering::Relaxed);
        }
    }
    let playing = AtomicBool::new(true);
    std::thread::scope(|s| {
        if every > 0 {
            s.spawn(|| {
                let mut since = Instant::now();
                while playing.load(Ordering::Relaxed) {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    if since.elapsed().as_secs() >= every {
                        if catch_unwind(AssertUnwindSafe(&report)).is_err() {
                            eprintln!("partial report skipped: it panicked");
                        }
                        since = Instant::now();
                    }
                }
            });
        }
        let _stop = Stop(&playing);
        play();
    });
}

/// What the command line overrides in the spec.
fn apply_flags(spec: &mut experiment::Spec, o: &Options) {
    if o.given.contains("--games") {
        spec.games = o.games;
    }
    if o.given.contains("--seed") {
        spec.first_seed = o.seed;
    }
    if o.given.contains("--max-actions") {
        spec.max_actions = o.max_actions;
    }
    if let Some(n) = o.winprob {
        spec.winprob.games = n;
    }
    if let Some(n) = o.closeness {
        spec.closeness.every = n;
    }
    if let Some(keep) = &o.variants {
        experiment::keep_variants(spec, keep)
            .unwrap_or_else(|e| die(&format!("--variants: {e}")));
    }
    if o.quick {
        quick(spec, o);
    }
}

/// The console table: one line per cell.
fn print_cells(sums: &[report::CellSummary]) {
    // Name the config and variant when there are several (020: a spec with two
    // variants printed every cell twice, indistinguishable).
    let configs: std::collections::BTreeSet<&str> = sums.iter().map(|s| s.config_name.as_str()).collect();
    let variants: std::collections::BTreeSet<&str> = sums.iter().map(|s| s.variant.as_str()).collect();
    let cell = |s: &report::CellSummary| {
        let mut c = String::new();
        if configs.len() > 1 {
            c.push_str(&format!("{} ", s.config_name));
        }
        if variants.len() > 1 {
            c.push_str(&format!("{} ", s.variant));
        }
        c
    };
    let width = sums.iter().map(|s| cell(s).len()).max().unwrap_or(0);
    for s in sums {
        let cell = format!("{:width$}", cell(s));
        println!(
            "  {cell}d{} {:24} win {:5.1}%  rounds {:5.2}  agent {:8.1} ms/game",
            s.difficulty,
            s.agent,
            100.0 * s.win_rate,
            s.rounds_mean,
            s.agent_ms_per_game
        );
    }
}

/// `--quick`: the scoping run. Shallow agents only (deep search is kept for
/// later, when the rules settle), at most 16 seeds per cell unless --games
/// says otherwise, and no probes unless --winprob or --closeness asks.
fn quick(spec: &mut experiment::Spec, o: &Options) {
    if !o.given.contains("--games") {
        spec.games = spec.games.min(QUICK_GAMES);
    }
    if o.winprob.is_none() {
        spec.winprob.games = 0;
    }
    if o.closeness.is_none() {
        spec.closeness.every = 0;
    }
    let (keep, drop): (Vec<String>, Vec<String>) = spec
        .agents
        .iter()
        .cloned()
        // A spec that doesn't parse is kept, for `cells` to reject as usual.
        .partition(|a| AgentSpec::parse(a).map_or(true, |s| s.shallow()));
    if keep.is_empty() {
        die("--quick: every agent of the spec searches deep; name a shallow one (greedy, engine)");
    }
    if !drop.is_empty() {
        eprintln!("--quick: dropped deep-search agents {}", drop.join(", "));
    }
    spec.agents = keep;
}

/// Seeds per cell under `--quick`.
const QUICK_GAMES: u64 = 16;

/// `games.jsonl`, started afresh and open for appending: the header, then
/// `prior` (a resumed run's games, rewritten so a line the kill cut short
/// is gone). Written whole and renamed into place, so the old stream is
/// intact until the new one is; a fresh run (no --resume) moves an earlier
/// run's stream to `games.jsonl.bak` rather than lose it.
fn open_stream(
    path: &str,
    resume: bool,
    spec: &experiment::Spec,
    cells: &[experiment::Cell],
    prior: &[experiment::GameResult],
) -> std::io::BufWriter<std::fs::File> {
    let fail = |e: std::io::Error| -> ! {
        eprintln!("cannot write {path}: {e}");
        exit(2)
    };
    if !resume && std::fs::metadata(path).is_ok_and(|m| m.len() > 0) {
        std::fs::rename(path, format!("{path}.bak")).unwrap_or_else(|e| fail(e));
    }
    let mut text = serde_json::to_string(&experiment::StreamHeader::new(spec, cells))
        .expect("header serializes");
    text.push('\n');
    for g in prior {
        text.push_str(&serde_json::to_string(g).expect("a game result serializes"));
        text.push('\n');
    }
    report::write_whole(path, &text).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(2)
    });
    let f = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap_or_else(|e| fail(e));
    std::io::BufWriter::new(f)
}

/// Writes the cycle report of `games` (sorted) into `dir`: report.md,
/// results.json, games.csv, cards.csv and records/. Each file is written
/// whole and then renamed into place (records too; stale ones go last), so
/// a run killed mid-write leaves the last complete report. Returns the cell
/// summaries and the records saved.
#[allow(clippy::too_many_arguments)]
fn write_reports(
    dir: &str,
    spec: &experiment::Spec,
    cells: &[experiment::Cell],
    games: &[experiment::GameResult],
    failed: &[experiment::FailedGame],
    wall_s: f64,
    threads: usize,
    planned: usize,
    resumed: usize,
) -> Result<(Vec<report::CellSummary>, usize), String> {
    let sums = report::summarize(cells, games);
    let groups = report::groups(cells, &sums, games);
    let cards = report::card_table(cells, games);
    // A gate's stopped checks are finished, not missing.
    let gate = spec.gate.as_ref().map(|g| gate::evaluate(g, spec, cells, games, failed));
    let planned = gate.as_ref().map_or(planned, |g| g.planned());
    let h2h = report::head_to_head(cells, games);
    let variants = report::variant_diffs(spec, cells, games);
    let pairs = report::agent_pairs(cells, games);
    let fresh = report::fresh_confirmation(spec, cells, games);
    let records = report::save_records(dir, spec, cells, games, &sums)
        .map_err(|e| format!("cannot write records: {e}"))?;
    let run = report::Run {
        spec,
        cells,
        games,
        failed,
        sums: &sums,
        groups: &groups,
        cards: &cards,
        head_to_head: &h2h,
        variant_diffs: &variants,
        agent_pairs: &pairs,
        fresh_confirmation: &fresh,
        top_agent: report::top_agent(cells, &sums),
        records: &records,
        wall_s,
        threads,
        out_dir: dir,
        planned,
        resumed,
        gate: gate.as_ref(),
    };
    // report.md last: once it is there, so is the rest of its set.
    for (name, body) in [
        ("results.json", report::results_json(&run)),
        ("games.csv", report::games_csv(cells, games)),
        ("cards.csv", report::cards_csv(cells, games)),
        ("report.md", report::markdown(&run)),
    ] {
        report::write_whole(&format!("{dir}/{name}"), &body)?;
    }
    report::prune_records(dir, &records)?;
    Ok((sums, records.len()))
}

/// Where a run's time went, on stderr: each part's share of the games' own
/// time (summed over workers), the report's writing, and, in a `profile`
/// build, the agents' search split.
fn print_timing(games: &[experiment::GameResult], wall_s: f64, report_s: f64) {
    let sum = |f: fn(&experiment::GameResult) -> f64| games.iter().map(f).sum::<f64>();
    let total = sum(|g| g.ms);
    let line: Vec<String> = report::time_split(games)
        .iter()
        .map(|(n, share)| format!("{n} {:.1}%", 100.0 * share))
        .collect();
    eprintln!(
        "time: wall {wall_s:.1} s, games {:.1} s summed over workers: {}; report {report_s:.2} s",
        total / 1000.0,
        line.join(", ")
    );
    if let Some(t) = prof::totals() {
        let agent = sum(|g| g.agent_ms + g.probe_ms).max(1e-9);
        let line: Vec<String> = t
            .iter()
            .map(|(n, ms, calls)| {
                format!(
                    "{n} {:.1}% ({calls} calls, {:.2} us each)",
                    100.0 * ms / agent,
                    1000.0 * ms / (*calls).max(1) as f64
                )
            })
            .collect();
        eprintln!("search (of agents + probes): {}", line.join(", "));
    }
    if let Some((allocs, bytes)) = prof::allocated() {
        let n = games.len().max(1) as f64;
        eprintln!(
            "allocated: {:.0} k allocations and {:.0} MB a game",
            allocs as f64 / n / 1e3,
            bytes as f64 / n / 1e6
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_panic_out_of_play_stops_the_reporter() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let died = catch_unwind(|| while_playing(1, || {}, || panic!("a worker died"))).is_err();
            tx.send(died).unwrap();
        });
        let died = rx.recv_timeout(std::time::Duration::from_secs(10));
        assert_eq!(died, Ok(true), "the panic comes out, and the reporter doesn't hold the process");
    }
}
