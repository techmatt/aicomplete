//! The command line: usage text, options and their checks, and loading a
//! config with its overrides.

use std::collections::BTreeSet;
use std::process::exit;

use aic_core::{Config, Rules};

use crate::agents::AgentSpec;

pub const USAGE: &str = "\
usage: aic-sim run <config.json> [options]
       aic-sim compare <a.json> <b.json> [options]
       aic-sim show <config.json> [--set k=v]...     resolved config + hash
       aic-sim modules                               module registry as JSON
       aic-sim replay <record.json>...               check saved game records
       aic-sim docs [--check]                        regenerate docs from content and
                                                     configs (--check: exit 1 if stale)
       aic-sim strategies <config.json> [--games N] [--seed S] [--set k=v]... [--agent SPEC]
                                                     each seed's strategies as the agent's
                                                     seats choose them at the round-1 keep
                                                     (CSV: seed,seat,board,strategy;
                                                     --agent defaults to strategist here)
       aic-sim cards                                 every content card as CSV: id,kind,tier,
                                                     engine,liberate (the agents' card tests)

options:
  --games N         games per config (default 200)
  --seed S          first seed (default 0); seeds S..S+N
  --set K=V         override for every config: countries=N, <module>.<param>=V,
                    <module>.variant=NAME, <module>=off (drops it), <module>.<param>=
                    (empty: unsets it); V is 3 or 1,2,3; or a content overlay,
                    <table>.<id>.<path>=JSON, as cards.<id>.cost=4 (repeatable)
  --set-a K=V       override for config A only (repeatable)
  --set-b K=V       override for config B only (repeatable)
  --agent SPEC      the team agent: random (default), greedy, beam, rollout, engine,
                    planner, sweep (a planner profile), strategist (the engine
                    with strategies, the screens' judge), oracle (full-information
                    search, opt in), with budget parameters as
                    kind:key=value,... (see docs/agents.md)
  --max-actions N   per-game action cap; a capped game is a timeout (default 100000)
  --threads N       worker threads (default: all cores); results do not depend on it
  --json            print the summary as JSON instead of a table
  --trace           print the first game of each config as JSON lines (actions, events)

saving game records (run, compare):
  --save DIR        write the selected games as game records into DIR (created if needed),
                    named <config name>-<run|A|B>-seed<S>.json
  --save-when W     select games by outcome: all, win, loss, timeout, or an end reason
                    code such as discord_deck or purge (repeatable; default loss, or none
                    when only --save-seed is given)
  --save-seed S     always save this seed, whatever --save-when says (repeatable)
  --save-max N      at most the first N games --save-when selects, in seed order, per
                    config (default 20); --save-seed games are not capped

replay:
  aic-sim replay <record.json>...   replay each record and check it against what was
                    saved; exit 1 if any record fails or has warnings

experiment:
  aic-sim experiment <spec.json> [--out DIR] [--games N] [--seed S] [--max-actions N]
                    [--threads N]
                    play the spec's grid (configs x variants x difficulties x agents
                    over a seed range) and write the cycle report into DIR (default
                    out/experiments/<name>): report.md, results.json, games.csv,
                    cards.csv, records/. --games, --seed and --max-actions override
                    the spec (whose action cap defaults to 5000).
  Expensive measures are opt-in (off unless the spec or these turn them on):
    --winprob N     win-probability playouts for the first N seeds of each cell
                    (about 5x an experiment's time at N = 10 of 200)
    --closeness N   closeness probe on every N-th real choice
    --variants A,B  play only these of the spec's variants (the base is always
                    kept, first): screen without probes, then probe the finalists
  Scoping runs, results as they go:
    --quick         the cheap preset: shallow agents only (deep-search agents are
                    dropped), at most 16 seeds per cell unless --games is given, no
                    probes unless --winprob or --closeness is given
    --report-every S  rewrite the report from the games finished so far every S
                    seconds (default 60; 0 = only at the end). Every game is also
                    appended to DIR/games.jsonl as it finishes
    --resume        continue a stopped run of the same spec from DIR/games.jsonl:
                    play only the games it lacks, then report them all

farm (conductor's remote workers; docs/simulator.md, The farm):
  aic-sim farm <spec> --seed F --games N --out FILE
                    every cell of the spec on seeds F..F+N, seed by seed, one game line
                    each (games.jsonl's, timings 0, engine named) into FILE; two
                    halves concatenated are the whole range. <spec>: a name in
                    experiments/farm/ or a path. Each game's time goes to stderr
  aic-sim farm <spec> --header --seed F --games N
                    the games.jsonl header line for those seeds
  aic-sim farm --fingerprint
                    a short check a worker must reproduce";

const RUN_FLAGS: &[&str] = &[
    "--games", "--seed", "--set", "--set-a", "--agent", "--max-actions", "--threads", "--json",
    "--trace", "--save", "--save-when", "--save-seed", "--save-max",
];
const COMPARE_FLAGS: &[&str] = &[
    "--games", "--seed", "--set", "--set-a", "--set-b", "--agent", "--max-actions", "--threads",
    "--json", "--trace", "--save", "--save-when", "--save-seed", "--save-max",
];

/// One command: the paths it takes (fewest, most, and what to say when they
/// don't fit) and the options it reads. A command refuses every other option
/// rather than silently drop it (020: `show --set-a` printed the unmodified
/// config; `replay --json` printed text).
const COMMANDS: &[(&str, usize, usize, &str, &[&str])] = &[
    ("run", 1, 1, "run takes 1 config path(s)", RUN_FLAGS),
    ("compare", 2, 2, "compare takes 2 config path(s)", COMPARE_FLAGS),
    ("show", 1, 1, "show takes one config", &["--set"]),
    ("modules", 0, 0, "modules takes no paths", &[]),
    ("cards", 0, 0, "cards takes no paths", &[]),
    ("replay", 1, usize::MAX, "replay takes one or more game records", &[]),
    ("docs", 0, 0, "docs takes no paths", &["--check"]),
    (
        "farm",
        0,
        1,
        "farm takes one spec",
        &["--seed", "--games", "--out", "--header", "--fingerprint"],
    ),
    ("strategies", 1, 1, "strategies takes one config", &["--games", "--seed", "--set", "--agent"]),
    (
        "experiment",
        1,
        1,
        "experiment takes one spec file",
        &[
            "--games", "--seed", "--max-actions", "--threads", "--out", "--winprob", "--closeness",
            "--variants", "--quick", "--resume", "--report-every",
        ],
    ),
];

/// What to do instead of a refused option, where there's something to say.
fn hint(command: &str, flag: &str) -> &'static str {
    match (command, flag) {
        ("experiment", "--set" | "--set-a" | "--set-b") => " (use the spec's variants)",
        ("experiment", "--agent") => " (use the spec's agents)",
        ("experiment", "--json") => " (results.json is always written)",
        ("experiment", "--save") => " (records are always saved)",
        ("show", "--set-a" | "--set-b") | ("run", "--set-b") => " (one config: use --set)",
        ("replay", "--set" | "--set-a" | "--set-b") => " (a record carries its own config)",
        ("farm", "--threads") => " (a shard is one thread: run more shards side by side)",
        _ => "",
    }
}

pub struct Options {
    pub command: String,
    pub paths: Vec<String>,
    pub games: u64,
    pub seed: u64,
    pub set: Vec<String>,
    pub set_a: Vec<String>,
    pub set_b: Vec<String>,
    pub max_actions: usize,
    pub threads: usize,
    pub json: bool,
    pub trace: bool,
    pub check: bool,
    pub save: Option<String>,
    pub save_when: Vec<String>,
    pub save_seeds: Vec<u64>,
    pub save_max: usize,
    pub agent: AgentSpec,
    /// Every option given (`experiment`: --games, --seed and --max-actions
    /// override the spec only if here).
    pub given: BTreeSet<String>,
    /// `experiment`: output directory.
    pub out: Option<String>,
    /// `experiment`: probe overrides and a variant filter (two-stage runs).
    pub winprob: Option<u64>,
    pub closeness: Option<usize>,
    pub variants: Option<Vec<String>>,
    /// `experiment`: the scoping preset, continuing a stopped run, and how
    /// often (seconds) the report of the games so far is rewritten.
    pub quick: bool,
    pub resume: bool,
    pub report_every: u64,
    /// `farm`: print the header line, or the fingerprint, instead of playing.
    pub header: bool,
    pub fingerprint: bool,
}

/// A bad command line: the message, then the usage.
pub fn die(msg: &str) -> ! {
    eprintln!("{msg}\n\n{USAGE}");
    exit(2)
}

/// Anything else that stops a run (a missing file, a bad spec): the usage
/// would only bury the message.
pub fn fail(msg: &str) -> ! {
    eprintln!("{msg}");
    exit(2)
}

pub fn parse_args() -> Options {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| die("missing command"));
    // 020: `aic-sim --help` said "unknown command" and exited 2.
    if command == "-h" || command == "--help" {
        println!("{USAGE}");
        exit(0);
    }
    let Some(&(_, fewest, most, paths_msg, flags)) = COMMANDS.iter().find(|c| c.0 == command) else {
        die(&format!("unknown command {command:?}"))
    };
    let mut o = Options {
        command,
        paths: Vec::new(),
        games: 200,
        seed: 0,
        set: Vec::new(),
        set_a: Vec::new(),
        set_b: Vec::new(),
        max_actions: 100_000,
        threads: std::thread::available_parallelism().map_or(1, |n| n.get()),
        json: false,
        trace: false,
        check: false,
        save: None,
        save_when: Vec::new(),
        save_seeds: Vec::new(),
        save_max: 20,
        agent: AgentSpec::parse("random").expect("random is an agent"),
        given: BTreeSet::new(),
        out: None,
        winprob: None,
        closeness: None,
        variants: None,
        quick: false,
        resume: false,
        report_every: 60,
        header: false,
        fingerprint: false,
    };
    while let Some(a) = args.next() {
        if a.starts_with("--") {
            o.given.insert(a.clone());
        }
        let mut value = |name: &str| {
            args.next()
                .unwrap_or_else(|| die(&format!("{name} needs a value")))
        };
        fn num<T: std::str::FromStr>(name: &str, v: String) -> T {
            v.parse()
                .unwrap_or_else(|_| die(&format!("{name}: not a number: {v:?}")))
        }
        match a.as_str() {
            "--games" => o.games = num("--games", value("--games")),
            "--seed" => o.seed = num("--seed", value("--seed")),
            "--out" => o.out = Some(value("--out")),
            "--winprob" => o.winprob = Some(num("--winprob", value("--winprob"))),
            "--closeness" => o.closeness = Some(num("--closeness", value("--closeness"))),
            "--variants" => {
                o.variants = Some(
                    value("--variants")
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                )
            }
            "--quick" => o.quick = true,
            "--resume" => o.resume = true,
            "--report-every" => o.report_every = num("--report-every", value("--report-every")),
            "--set" => o.set.push(value("--set")),
            "--set-a" => o.set_a.push(value("--set-a")),
            "--set-b" => o.set_b.push(value("--set-b")),
            "--max-actions" => o.max_actions = num("--max-actions", value("--max-actions")),
            "--agent" => o.agent = AgentSpec::parse(&value("--agent")).unwrap_or_else(|e| die(&e)),
            "--threads" => o.threads = num::<usize>("--threads", value("--threads")).max(1),
            "--json" => o.json = true,
            "--trace" => o.trace = true,
            "--check" => o.check = true,
            "--header" => o.header = true,
            "--fingerprint" => o.fingerprint = true,
            "--save" => {
                let d = value("--save");
                if d.is_empty() {
                    die("--save needs a directory");
                }
                o.save = Some(d)
            }
            "--save-when" => {
                let w = value("--save-when");
                if w.is_empty() {
                    die("--save-when needs a value");
                }
                o.save_when.push(w)
            }
            "--save-seed" => o.save_seeds.push(num("--save-seed", value("--save-seed"))),
            "--save-max" => o.save_max = num("--save-max", value("--save-max")),
            "-h" | "--help" => {
                println!("{USAGE}");
                exit(0)
            }
            s if s.starts_with("--") => die(&format!("unknown option {s:?}")),
            _ => o.paths.push(a),
        }
    }
    for flag in &o.given {
        if !flags.contains(&flag.as_str()) {
            die(&format!(
                "{} does not take {flag}{}",
                o.command,
                hint(&o.command, flag)
            ));
        }
    }
    if o.paths.len() < fewest || o.paths.len() > most {
        die(paths_msg);
    }
    if o.out
        .as_deref()
        .is_some_and(|d| d.trim_end_matches(['/', '\\']).is_empty())
    {
        die("--out needs a directory");
    }
    let save_options = ["--save-when", "--save-seed", "--save-max"];
    if o.save.is_none() && save_options.iter().any(|f| o.given.contains(*f)) {
        die("--save-when, --save-seed and --save-max need --save DIR");
    }
    if o.save.is_some() && o.save_when.is_empty() && o.save_seeds.is_empty() {
        o.save_when.push("loss".to_string());
    }
    o.save_seeds.sort_unstable();
    o.save_seeds.dedup();
    if o.games == 0 {
        die("--games must be at least 1");
    }
    if o.seed.checked_add(o.games - 1).is_none() {
        die(&format!(
            "--seed {} + --games {} overflows u64",
            o.seed, o.games
        ));
    }
    o
}

impl Options {
    /// The last seed played (inclusive), so `u64::MAX` itself is reachable.
    pub fn last_seed(&self) -> u64 {
        self.seed + (self.games - 1)
    }

    /// Whether `seed` is one of those played.
    pub fn in_range(&self, seed: u64) -> bool {
        (self.seed..=self.last_seed()).contains(&seed)
    }
}

pub fn load(path: &str, overrides: &[&String]) -> Config {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| fail(&format!("{path}: {e}")));
    let mut c = Config::from_json(&text).unwrap_or_else(|e| fail(&format!("{path}: {e}")));
    for o in overrides {
        c.apply_override(o)
            .unwrap_or_else(|e| fail(&format!("{path}: {e}")));
    }
    c
}

pub fn rules(path: &str, c: &Config) -> Rules {
    Rules::new(c).unwrap_or_else(|e| fail(&format!("{path}: invalid config: {e}")))
}
