//! Simulation CLI: plays seeded games for one or two configs with a
//! selectable team agent (random by default), prints win rate, game length
//! and counters side by side, and replay-checks every game; runs experiment
//! grids into a cycle report. See docs/simulator.md for usage.

mod agents;
mod cli;
mod current_docs;
mod docs;
mod spelling;
mod experiment;
mod experiment_run;
mod gate;
mod play;
mod prof;
mod report;

use std::process::exit;

use cli::{load, parse_args, rules, Options};

/// The search clones game states by the hundred thousand, and the system
/// allocator serializes them across workers: on 12 threads a state clone
/// took 134 us against 24 us on one (012). mimalloc keeps a heap per thread.
#[cfg(not(feature = "profile"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "profile")]
#[global_allocator]
static ALLOC: prof::Counting = prof::Counting;

fn main() {
    let o = parse_args();
    match o.command.as_str() {
        "modules" => println!(
            "{}",
            serde_json::to_string_pretty(&aic_core::describe_modules()).unwrap()
        ),
        "show" => show(&o),
        "docs" => docs::run(&o),
        "run" | "compare" => play::run(&o),
        "replay" => play::replay_all(&o),
        "experiment" => exit(experiment_run::run_experiment(&o.paths[0], &o)),
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
