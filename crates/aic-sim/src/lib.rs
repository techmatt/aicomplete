//! The simulator: the sim players (`agents/`), experiments and their reports,
//! and the CLI (`main`, run by `src/main.rs`). A library so the farm's wasm
//! bundle (`crates/aic-farm`) plays a read's games with the same code (160);
//! `farm` is all it uses.

mod agents;
mod cli;
mod clock;
mod docs;
mod endgame;
mod experiment;
mod experiment_run;
pub mod farm;
mod fmath;
mod gate;
mod play;
mod prof;
mod report;

use std::process::exit;

use cli::{load, parse_args, rules, Options};

/// The search clones game states by the hundred thousand, and the system
/// allocator serializes them across workers: on 12 threads a state clone
/// took 134 us against 24 us on one (012). mimalloc keeps a heap per thread.
/// wasm keeps its own allocator: mimalloc doesn't build there.
#[cfg(all(not(feature = "profile"), not(target_arch = "wasm32")))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "profile")]
#[global_allocator]
static ALLOC: prof::Counting = prof::Counting;

/// The `aic-sim` command line.
pub fn main() {
    let o = parse_args();
    match o.command.as_str() {
        "modules" => println!(
            "{}",
            serde_json::to_string_pretty(&aic_core::describe_modules()).unwrap()
        ),
        "cards" => print!("{}", agents::policy::cards_csv()),
        "show" => show(&o),
        "map" => map(&o),
        "docs" => docs::run(&o),
        "run" | "compare" => play::run(&o),
        "replay" => play::replay_all(&o),
        "experiment" => exit(experiment_run::run_experiment(&o.paths[0], &o)),
        "farm" => exit(farm::cli(&o)),
        "strategies" => {
            let path = &o.paths[0];
            let r = rules(path, &load(path, &o.set.iter().collect::<Vec<_>>()));
            // 102 (D36): the strategies the screens' judge plays, unless --agent says.
            let agent = if o.given.contains("--agent") {
                o.agent.clone()
            } else {
                agents::AgentSpec::parse("strategist").expect("strategist is an agent")
            };
            agents::policy::print_choices(&r, &agent, o.seed, o.games);
        }
        other => unreachable!("command {other} is checked in parse_args"),
    }
}

/// `aic-sim map` (198): every country's hexes by printed number, with kind, flags and neighbors, so a printed
/// setup hex can be checked by number (the axes differ by layout, so "next to" can't be read off the numbers).
fn map(o: &Options) {
    let path = &o.paths[0];
    let r = rules(path, &load(path, &o.set.iter().collect::<Vec<_>>()));
    let w = r.world();
    let name = |h: usize| {
        let c = &w.map.hexes[h];
        let country = w.content.map.countries.get(c.country as usize).map_or("?", |p| p.name.as_str());
        format!("{country} #{}", c.number)
    };
    for (ci, geo) in w.map.countries.iter().enumerate() {
        let country = w.content.map.countries.get(ci).map_or("?", |p| p.name.as_str());
        println!("{country} (country {ci})");
        let mut hexes = geo.hexes.clone();
        hexes.sort_by_key(|&h| w.map.hexes[h].number);
        for h in hexes {
            let c = &w.map.hexes[h];
            let mut flags = Vec::new();
            for (on, f) in [(c.coastal, "coastal"), (c.industrial, "industrial"), (c.station_site, "station site")] {
                if on {
                    flags.push(f);
                }
            }
            let mut by: Vec<usize> = c.neighbors.clone();
            by.sort_by_key(|&x| (w.map.hexes[x].country, w.map.hexes[x].number));
            let near: Vec<String> = by.into_iter().map(name).collect();
            let flags = if flags.is_empty() { String::new() } else { format!(" ({})", flags.join(", ")) };
            println!("  #{} {}{flags}: {}", c.number, c.kind.name(), near.join(", "));
        }
    }
}

/// `aic-sim show`: the resolved config, its hashes and the tunables.
fn show(o: &Options) {
    let path = &o.paths[0];
    let r = rules(path, &load(path, &o.set.iter().collect::<Vec<_>>()));
    println!(
        "config hash {}  content hash {}  rules revision {}",
        r.config_hash,
        r.content_hash,
        aic_core::RULES_REVISION
    );
    println!("{}", r.config.to_json_pretty());
    println!("\ntunables (--set <key>=<value>):");
    for t in r.tunables() {
        println!(
            "  {:28} {:>12}  {} {}..={}  {}",
            t.key,
            t.value.to_string(),
            t.kind,
            t.min,
            t.max,
            t.doc
        );
    }
}
