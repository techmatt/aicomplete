# Scripts

What each script under `scripts/` does, and its flags. Run from the repo root. A reader's docstring says in full
what it prints; the `cards/*_read.py` readers take seconds.

- `wasm-smoke.mjs`: one game through the wasm package in Node. `wasm-parity.mjs`: replays aic-sim records of each
  `configs/variants.json` variant in wasm.
- `farm/build.mjs BUNDLE`: the farm's wasm bundle. `farm/run.mjs BUNDLE SPEC FIRST COUNT OUT [--browser] [--port N]
  [--fingerprint]`: a shard on it in Node or headless Edge (`web/scripts/cdp.mjs`), each seed timed.
  `farm/compare.py A B [--ms A_ERR B_ERR]`: two engines' outputs of one range: games that differ, win %, paired
  gap, ms a game.
- `regen.py [--no-build] [--check]`: builds release, runs `drafts/adversary/check.py`, checks `content/` against
  `drafts/adversary/` (`load_adversary.py --check`), writes `RULES_REVISION` into `docs/rules/README.md`, runs `aic-sim
  docs`, `cards_doc.py`, `effect_ledger.py`, `no_liberate.py`, `no_tier2.py`; warns of a content file
  `apply_overlay.py` would refuse (not `json.dumps(indent=1)`); prints the content hash and `git status`. `--check`:
  the checks and the revision line only.
- `design_check.py [DIR] [--old DIR] [--base REV] [--config FILE] [--quiet] [--usage] [--variant V,...]`: the design
  warnings ([principles.md](principles.md)): content checks, plus play checks on DIR (a read's folder or
  `games.jsonl`; a farm out file as FILE=SPEC). `--old`: the previous revision's read on its seeds; `--base`: the
  dials' start (default: merge base with main); `--variant`: only those variants' (or a lever's) games, `base` the
  bare pairs; `--usage`: each action's role, band and use instead. First line: the report's `warnings:`. Thresholds
  and bands: `design_check.json`; job parity's cards for each standard job, hand-read (units and other effects as
  expressions over the anchors; costs from content): `design_check_jobs.json`, written by 249's `port_jobs.py`
  from 248's table (a new card for a job is a row there). Never exits 1.
- `cards/cards_doc.py`: regenerates `docs/cards.md`. `cards/deck_text.py DECK...`: a deck as plain text for review
  (`--list` the decks; README, *Card text for review*).
- `cards/apply_overlay.py FILE... [--set ENTRY]... [--check]`: writes overlays into `content/*.json` in place:
  `cards.<id>.<path>=JSON` lines (the engine `--set`'s keys, paths as `apply_entry` in content.rs; `#` comments), a
  last step `+` appending to a list, `cards.<id>=<JSON object>` a whole row. Nothing deletes (how to drop a card: its
  docstring). `--check`: dry run. Then `regen.py`. It refuses hand-laid `alignment.json`; threats, adversary and
  projects: edit `drafts/adversary/`.
- `content/load_adversary.py [--check]`: `drafts/adversary/` (the source) into `content/` and
  `docs/adversary-cards.md`; writes nothing if `drafts/adversary/check.py` gives any ERROR; `--check` only compares.
- `configs/no_liberate.py`, `configs/no_tier2.py` (run by `regen.py`): the baseline less every `liberate` card, and
  with no tier-2 deck (opening tier-2 keeps from tier 1), in `experiments/configs/`. `configs/three_seat.py` (also
  run by `regen.py`): `configs/three_seat.json`, the baseline at 3 countries and difficulty 0, the viewer's 3-seat game.
- `cards/loop_smoke.py [DIR...] [--run] [--spec S] [--games 16] [--seed S] [--threads 6] [--max-plays 12]`: the loop
  smoke (no DIR: runs `use.json` to `out/loop-smoke`): timeouts, ends, most plays of one card a game; exits 1 on a
  timeout or `--max-plays`.
- `cards/counter_names.py DIR... [--all]` (DIR, games file or `FILE=SPEC`): counter keys the read scripts, web scripts and `crates/aic-sim` (agents
  aside) quote that no game in DIR wrote; rare events and test strings list too, so don't gate on it.
- `cards/screen.py`: named columns over experiment outputs (`--columns a,b,c`, `--preset tiers|041|034|018|016`,
  `--sources`, `--list`; `tiers`, offered, kept and played by tier, is a card rework's first screen); `--gate DIR`
  (gates are suspended) each check against its own agent, then unspent.
- `screens.py` reads a run from Python: `load(dir)` the games with their cells (`stream` also takes `FILE=SPEC`, and stops on a
  headerless file passed without its `=SPEC`),
  `cnt(game, key)` a counter (a power is `ai.power.<board>.<name>`: `scripts/powers.py`). `screens.py assemble SPEC
  FIRST COUNT OUT_DIR SHARD... [--exe EXE]` writes farmed shards (seed order) under the spec's header as
  `OUT_DIR/games.jsonl`.
- `cards/play_read.py DIR... [--min PCT] [--agents A,B]` (DIR, games file or `FILE=SPEC`): per agent, pooled: draft cards kept in over `--min` percent
  of games (default 1) and played in none, and those no game kept. Not why: a card is offered only when some part of
  it can act (`feasible` in `crates/aic-core/src/modules/effects/feasible.rs`).
- `cards/use_read.py DIR... [--min PCT] [--agents A,B]`: per agent and pair, pooled: wins, uses of dear powers, Grant
  and Action buildings, tokens, liberate cards; then `play_read.py`'s lists.
- `cards/runaway_read.py DIR... [--times 5] [--floor 20] [--held 150] [--peak-times 3] [--peak-floor 10] [--top 15]`:
  loops (a counter whose per-game max is over `--times` its mean where used), insight growing three rounds running
  past `--held`, peaks held over `--peak-times` income a round.
- `cards/source_read.py DIR... [--top 5] [--pool] [--resource credits,insight,energy,chips|all]`: per cell (`--pool`:
  per agent) and board, each resource's top sources a game; keeps by runaway_read's insight flag.
- Per cell, `DIR...`: `cards/threat_read.py [--cards]` threats dealt, countered, paid abroad, typed counters and
  capstones; `cards/slots_read.py [--rounds N]` build-overs and buildings standing; `cards/board_read.py [--powers]
  [--cards]` wins and the boards' measures (`--powers` each power a game, `--cards` research-cube cards);
  `cards/liberation_read.py [--cards]` cities corrupted and liberated (`--cards` each liberate card; also a games
  file or `FILE=SPEC`);
  `cards/economy_read.py [--rounds 10]` card throughput, income, unspent; `cards/energy_read.py` energy held and
  used; `cards/corp_read.py` completions, cubes, converters, stations, adversary compute.
- `cards/effect_stats.py READ [--screen DIR] [--flag 5] [--out MD]`: each Unity and Discord card's resolutions and
  effects (`fx.<id>.*`), each prohibition's fire rate and blocks; `--screen` an on/off screen (`PAIR_<id>`) for the
  paired win effect, flagged past ±`--flag`.
- `cards/price_read.py DIR [--agent planner] [--csv OUT] [--flags 20]`: per-card rates and stratified win deltas
  for one agent (experiments.md, *Reading prices*); `--flags` the rows each flag table lists.
- `cards/effect_ledger.py`: splits each card's printed cost over its effects pro rata to the reasoned values
  (`values.json`), fits a price multiplier per effect, writes CSVs, `ledger.html` and `outliers.md` (tier-2 cards
  over the ceiling) to `experiments/results/effect-ledger/`. `--stdout`: each card's fit and fair price
  (card-pricing.md, *A fair price*), writing nothing; `--cards FILE`, `--values FILE`: proposed files; `--overlay
  FILE` (with `--stdout`): an unwritten overlay's `cards.` lines; `--hold-ratio OLD.csv`: the price keeping an older
  `cards.csv`'s printed / reasoned ratio.
- `cards/payback.py [--cards CARDS.json] [--old CARDS.json [--old-keep-tier2 N]]`: rounds each engine building takes
  to pay back at a committed seat's fire rates (card-pricing.md *Engines*); `--old` adds an older deck.
- `run_sim.ps1 NAME [-NoBuild] ARGS...`: any long run: builds, copies the exe to `out/NAME.exe`, starts it below
  normal, detached; wait on `out/NAME.exit`.
- `bench/same_results.py A B`: compares two `aic-sim experiment` outputs (a farmed read: `farm/compare.py`).
  `--map OLD=NEW` renames in the old output first; `--ignore-content-hash` for a content refactor,
  `--ignore-config-hash` for a dead param removed (`same_play.py` can't: its fingerprints hash the config).
- `bench/same_play.py`: a refactor left play unchanged over countries, difficulties, variants and every agent but
  the beam, old binary against new (~20 min, 6 threads). `--map` as above; an old binary refusing today's configs:
  `run ... --root out/old`.
- `bench/speed.py`: games a minute per agent: `pair OLD NEW` (each row on both in turn; `--old-config` for an OLD on
  another baseline), `table`, `compare`, `specs SIM A B`; `check SIM` (~2 min, one thread, quiet machine) fails at
  1.3 × `speed_baseline.json` (`--update` moves it; across a revision: NO VERDICT). [speed.md](speed.md).
- `bench/per_decision.py OLD_ROOT NEW_ROOT`: rounds, decisions and ms a game and a decision on the same seeds
  (`--games 16`+, speed.md; `--agent`, `--old-agent` the agents a change touched; `--first 1000` the first seed,
  `--countries 2`, `--pairs 2` passes, the best kept). A root is an `.exe` or a folder with `aic-sim.exe`
  and its `configs/`. `--set K=V` goes to both sides: a content overlay runs the new binary on old content.
- `bench/seeds.py next N [--take] [--doc FILE]`: the next N fresh seeds from experiments.md; `--take` moves `Next free`.
- `bench/skilled_rows.py READ.json [--games 24] [--levels 3,4] [--rows ...] [--out ...]`: the oracle's companion spec
  to a read (experiments.md, *Skilled rows*).
- `bench/read_specs.py {offs|screen|read} ... --tag NNN [--cards a,b,c] [--out-dir DIR]`: a rules item's specs into
  `out/`: `read TAG SEED [AGENT] [--probe] [--levels 0,1,2,3] [SET...]` three 48-seed sets, the three pairs, d0-d3;
  `screen NAME AGENTS LEVELS SEEDS [--offs] [--first N] [LEVER...]` a dev screen (lever `tag:a=1;b=2`); `offs` a
  config per `--cards` card switched off. `bench/run_read.sh NNN EXE ROOT TAG OUT THREADS` runs a read's three sets
  on one binary (ROOT its configs; EXE, OUT absolute or from the repo root).
- `bench/ladder.py NEW[,...] OLD[,...] [--measures] [--bands B] [--deck N]`: the paired ladder per level by revision;
  `--screen DIR[,...]`: a screen's levers (`PAIR_L`, `PAIR-L`) against base, paired across DIRs. `FILE=SPEC.json`,
  engines, `--same-engine` as `paired.py`. `--measures`: the adversary's and the boards' standing measures, and a
  screen's rounds. `--pairs [1-2,0-3]`: win % by unordered board pair (`bp`, `pb` pool), gaps to the mean, paired
  gap, seat order. `bench/split_read.py READ OUT` splits a lever screen.
- `bench/paired.py DIR... [--pair A B]... [--against-base] [--same-engine]`: cells' win %, SE, rounds and ends, and
  paired gaps pooled over `--pair`s on the seeds both played (`--same-engine`: one engine's seeds only). A `--pair`
  cell is its exact name (`bp/strategist`, `DIR/bp/strategist`), led by config stem and `dN` where they vary
  (`no_liberate/d2/bp/strategist`). `--against-base`: a price sweep's `cN-x` against `cN`, pooled over sizes. By
  difficulty, pair and all: `--against-agent A` each agent against A, `--lever-suffix L` each `PAIR-L` (or `PAIR_L`)
  cell against `PAIR`. `--pool`: the DIRs are one read's seed ranges; `--other DIR`: another build's read on the
  same seeds, as `other:AGENT`. After editing ladder.py, paired.py or screens.py, run `bench/test_read_tools.py`.
- Gates are suspended: `bench/gate_stop_rate.py`, how often early stops flip a verdict; `bench/gate_close.py plan
  GATE_DIR OUT_DIR [--z 2] [--cap 96] [--fill 48]`, specs carrying the rows within z paired SE on to the cap, reusing
  their games, and `read GATE_DIR [CLOSE_DIR...] [--min-seeds N]`, merging the runs (checks on fewer seeds left out).
- `bench/shards.sh EXE DIR SPEC FIRST COUNT OUT`: a farm spec natively as `SHARDS` (default 6) one-thread `aic-sim
  farm` shards side by side in DIR, concatenated under the header into OUT: a paired read's base side.
- `bench/first_divergence.py OLD NEW`: where two runs of one spec first differ, per game (`--cells DIR A B`: a run's
  two cells).
- `bench/profile_win.py SIM OUT -- run ...`: a sampling profile, no admin (speed.md, *Profiling*). `--runs K` on the
  next seeds each; `--attach S1,S2,...` reads 5 s windows of a long run; `--rate 1000` samples a second.
- `bench/resource_use.py NEW [OLD] [--top 8] [--limits]` (folder, games file or `FILE=SPEC`): chips, energy, cubes and antimatter
  gained and spent per seat a game by level, what buys and sources them (`--limits`: the `limit.*` table).
- `bench/am_read.py READ[,READ...]` (same forms): antimatter by source, made and spent, burns by round, per lever
  (`PAIR_LEVER` variants) and agent (230's reader; stops if a plant or burn it names left content).
- `bench/projects_read.py READ [--md OUT]` (folder, games file or `FILE=SPEC`): each project's reveals, completions, compute drained and map
  moves a completion, by level.
- `bench/card_ranks.py DIR A B [--variant V]...`: Spearman of the cards' keep and play rates, agent B against A.
- `bench/item_time.py ITEM... [--top N]`: a done item's wall time: sims, builds, tests, the rest.
- `tune/`: `agent_sweep.py`, `sweep.py`, `log_table.py`, `engine_sweep.py` ([README](../scripts/tune/README.md)).
  `noop_weights.py spec [--agent A] [--seeds N]` / `read OUT SPEC`: each `Weights` field moved (to 0, or 1 where 0),
  one pair at d1-d2; lists the weights that change no game of the agent.
- Past items' one-off readers: `experiments/archive/scripts/`, kept for the write-ups citing them.
