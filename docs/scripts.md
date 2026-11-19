# Scripts

What each script under `scripts/` does, and its flags. Run from the repo root. A reader's docstring says in full
what it prints; the `cards/*_read.py` readers take seconds.

- `wasm-smoke.mjs`: one game through the wasm package in Node. `wasm-parity.mjs`: replays aic-sim records of each
  `configs/variants.json` variant in wasm.
- `farm/build.mjs BUNDLE`: the farm's wasm bundle. `farm/run.mjs BUNDLE SPEC FIRST COUNT OUT [--browser] [--port N]
  [--fingerprint]`: a shard on it in Node or headless Edge (`web/scripts/cdp.mjs`), each seed timed.
  `farm/compare.py A B [--ms A_ERR B_ERR]`: two engines' outputs of one range: games that differ, win %, paired
  gap, ms a game.
- `regen.py [--no-build] [--check | --golden]`: every generator in order (its docstring), the derived
  configs (`no_liberate.py`, `no_tier2.py`, `three_seat.py`) last. `--check` writes nothing: drafts and loader
  checks, a build (unless `--no-build`), `aic-sim docs --check`, `cards_doc.py --check`, the derived configs' `--check`, the revision lines in
  `rules/README.md` and `rulebook.md`. `--golden`: also sets the golden constants (README, *Rules changes*).
- `design_check.py [DIR] [--old DIR] [--base REV] [--config FILE] [--quiet] [--usage] [--variant V,...] [--agent A,...]`: the design
  warnings ([principles.md](principles.md)): content checks, plus play checks on DIR (a read's folder or
  `games.jsonl`; a farm out file as FILE=SPEC; one DIR, comma lists pool; an unknown flag stops it). `--old`: the previous revision's read on its seeds; `--base`: the
  dials' start (default: merge base with main); `--variant`: only those variants' (or a lever's) games, `base` the
  bare pairs; `--agent`: only those agents' (as the spec writes them), on `--old` too; `--usage`: each action's role, band and use instead (a `usage` row's `levels` reads those difficulties only). Powers are named by the read's `ai.power.*`
  counters. Table cost (268): `table_cost`'s budgets by level, and `friction-stale` off
  `docs/rulebook-friction.md`'s title. First line: the report's `warnings:`. Thresholds
  and bands: `design_check.json`; job parity's cards for each standard job, hand-read (effects as expressions over
  the anchors): `design_check_jobs.json` (`jobs`: per standard row, `{unit, cards: [{name, units, other, use, note}]}`),
  edited by hand (`experiments/results/249_real_oracle_judge/port_jobs.py` would overwrite it). A `repeat_powers.exempt` key may join names (`"Sprawl, Outpost"`).
  Never exits 1 on a warning.
- `cards/cards_doc.py`: regenerates `docs/cards.md`. `cards/deck_text.py DECK...`: a deck as plain text for review
  (`--list` the decks; README, *Card text for review*); `--ids TEXT`: content ids by display name.
- `cards/apply_overlay.py FILE... [--set ENTRY]... [--check]`: writes overlays into `content/*.json` in place:
  `cards.<id>.<path>=JSON` lines (the engine `--set`'s keys, paths as `apply_entry` in content.rs; `#` comments), a
  last step `+` appending to a list, `cards.<id>=<JSON object>` a whole row. Nothing deletes (how to drop a card: its
  docstring). `--check`: dry run; `--from REV`: over REV's content, not the tree's (a changed overlay drops what it no longer
  sets; warns of tree edits it would drop). Then `regen.py`. It writes `alignment.json` in its hand layout (`dump_alignment`); threats,
  adversary and projects: edit `drafts/adversary/`.
- `content/load_adversary.py [--check]`: `drafts/adversary/` (the source) into `content/` and
  `docs/adversary-cards.md`; writes nothing if `drafts/adversary/check.py` gives any ERROR; `--check` only compares.
- `configs/no_liberate.py`, `no_tier2.py`: the baseline less every `liberate` card, and with no tier-2 deck (opening
  tier-2 keeps from tier 1), in `experiments/configs/`; `three_seat.py`: `configs/three_seat.json`, the baseline at
  3 countries and difficulty 0 (the viewer's 3-seat game).
- `cards/loop_smoke.py [DIR...] [--run] [--out DIR] [--exe EXE]` (flags: docstring): the loop smoke (no DIR: runs
  `use.json` to `out/loop-smoke` on a copy of `AIC_SIM` or the release build): timeouts, ends, most plays of one card a game; exits 1 on a timeout or `--max-plays`.
- `cards/counter_names.py DIR... [--all]` (DIR, games file or `FILE=SPEC`): counter keys the read scripts, web scripts and `crates/aic-sim` (agents
  aside) quote that no game in DIR wrote, and each sweep `COUNTER_EVENTS*` alternative no key matches; rare events
  list too, so don't gate on it.
- `cards/screen.py`: named columns over experiment outputs (`--columns a,b,c`, `--preset tiers|041|034|018|016`,
  `--sources`, `--list`; `tiers`, offered, kept and played by tier, is a card rework's first screen); `--gate DIR`
  (gates are suspended) each check against its own agent, then unspent.
- `screens.py` reads a run from Python: `load(dir)` the games with their cells (`stream` also takes `FILE=SPEC`), `cnt(game, key)` a counter (a power is `ai.power.<board>.<name>`: `scripts/powers.py`). `screens.py assemble SPEC
  FIRST COUNT OUT_DIR SHARD... [--exe EXE]` writes farmed shards (seed order) under the spec's header as
  `OUT_DIR/games.jsonl`.
- `cards/play_read.py DIR... [--min PCT] [--agents A,B] [--variant V,...]` (DIR, games file or `FILE=SPEC`; agents by spec name, so native
  and farmed reads pool; `--agents` takes it or the label; `--variant` as design_check's): per agent: draft cards kept in over `--min` percent
  of games (default 1) and played in none, and those no game kept. Not why: a card is offered only when some part of
  it can act (`feasible` in `crates/aic-core/src/modules/effects/feasible.rs`).
- `cards/use_read.py DIR... [--min PCT] [--agents A,B]`: per agent and pair, pooled: wins, uses of dear powers, Lab II
  (Grant before 276) and Action buildings, tokens, liberate cards; then `play_read.py`'s lists.
- `cards/runaway_read.py DIR...` (flags: docstring): loops (a counter whose per-game max is over `--times` its mean where used), insight growing three rounds running
  past `--held`, peaks held over `--peak-times` income a round.
- `cards/source_read.py DIR... [--top 5] [--pool] [--resource credits,insight,energy,chips|all]`: per cell (`--pool`:
  per agent) and board, each resource's top sources a game; keeps by runaway_read's insight flag.
- Per cell, `DIR...`: `cards/threat_read.py [--cards]` threats dealt, countered, paid abroad, typed counters and
  capstones; `cards/slots_read.py [--rounds N]` build-overs and buildings standing; `cards/board_read.py [--powers]
  [--cards]` wins and the boards' measures (`--powers` each power a game, `--cards` research-cube cards);
  `cards/liberation_read.py [--cards]` cities corrupted and liberated (`--cards` each liberate card);
  `cards/spread_read.py FILE=SPEC[@TAG]... [--rounds]` length and new cities by
  level, arm (the variant's name after its pair prefix: `bp-sp` or `bp_sp` is `sp`) and round;
  `cards/economy_read.py [--rounds 10]` card throughput, income, unspent; `cards/energy_read.py` energy held and
  used; `cards/corp_read.py` completions, cubes, converters, stations, adversary compute.
- `cards/effect_stats.py READ [--screen DIR]`: each Unity and Discord card's resolutions and effects (`fx.<id>.*`),
  each prohibition's fire rate and blocks; `--screen`: an on/off screen's paired win effect (docstring).
- `cards/price_read.py DIR [--agent planner]`: per-card rates and stratified win deltas for one agent
  (experiments.md, *Reading prices*; flags: docstring).
- `cards/effect_ledger.py`: splits each card's printed cost over its effects pro rata to the reasoned values
  (`values.json`), fits a price multiplier per effect, writes CSVs, `ledger.html` and `outliers.md` (tier-2 cards
  over the ceiling) to `experiments/results/effect-ledger/`. `--stdout`: each card's fit and fair price
  (card-pricing.md, *A fair price*), writing nothing; `--reasoned-delta BASE` (a revision or cards.json): each card's reasoned change and its
  cost moved by it; other flags: docstring.
- `cards/payback.py [--cards CARDS.json] [--old CARDS.json [--old-keep-tier2 N]]`: rounds each engine building takes
  to pay back at a committed seat's fire rates (card-pricing.md *Engines*); `--old` adds an older deck.
- `run_sim.ps1 NAME [-NoBuild] ARGS...`: any long run: builds, copies the exe to `out/NAME.exe`, starts it below
  normal, detached; wait on `out/NAME.exit`.
- `bench/same_results.py A B`: compares two `aic-sim experiment` outputs (a farmed read: `farm/compare.py`).
  `--map OLD=NEW` renames in the old output first; `--ignore-content-hash` for a content refactor,
  `--ignore-config-hash` for a dead param removed (`same_play.py` can't: its fingerprints hash the config).
- `bench/same_play.py`: a refactor left play unchanged over countries, difficulties, variants and every agent but
  the beam, old binary against new (~20 min, 6 threads). `--map` as above; an old binary refusing today's configs:
  `run ... --root out/old`. `tracediff A B [--ignore KEY]...`: two `run --trace` outputs' first differing line, after
  its round and phase (`--ignore` drops events naming KEY, `.` ending a prefix, and the field KEY: `config_hash`).
- `bench/trace_lib.py`: the reader for a `run --trace` (reads.md): `walk()` a `Step` a line (round, phase, seat,
  actor, kind, action, options, event, ask), `summary()` (overrides, counters), `card_names()`, `hexes(countries)`.
  A table-time or trace script imports it (tests: `bench/test_trace_lib.py`).
- `edit/replace.py SPEC.json [--dry-run]`: exact replacements from `[{file, old, new, count}]` (count 1 or `"all"`),
  all or nothing, line endings kept; `--word OLD NEW PATH...` a whole-word rename over tracked files. The hooks refuse
  `sed -i` and heredoc edits: use this, not a one-off script.
- `bench/speed.py`: games a minute per agent: `pair OLD NEW` (each row on both in turn; `--old-config` for an OLD on
  another baseline), `table`, `compare`, `specs SIM A B`; `check SIM` (~4 min, mostly planner, one thread, quiet machine) fails at
  1.3 × `speed_baseline.json` (`--update` moves it; across a revision: NO VERDICT). Runs a copy of each SIM. [speed.md](speed.md).
- `bench/per_decision.py OLD_ROOT NEW_ROOT`: rounds, decisions and ms a game and a decision on the same seeds
  (`--games 16`+, speed.md; `--agent`, `--old-agent` the agents a change touched; `--first 1000` the first seed,
  `--countries 2`, `--pairs 2` passes, the best kept). A root is an `.exe` or a folder with `aic-sim.exe`
  and its `configs/`. `--set K=V` goes to both sides: a content overlay runs the new binary on old content.
- `bench/seeds.py next N [--take] [--doc FILE]`: the next N fresh seeds from experiments.md; `--take` moves `Next free`.
- `bench/skilled_rows.py READ.json [--games 24] [--levels 3,4] [--rows ...] [--out ...]`: the oracle's companion spec
  to a read (experiments.md, *Skilled rows*).
- `bench/read_specs.py {offs|screen|read} ...`: a rules item's specs into `out/` (`read`: three 48-seed sets;
  `screen`: a dev screen, lever `tag:a=1;b=2`; `offs`: a config per card off; docstring). `bench/run_read.sh NNN EXE ROOT TAG OUT THREADS` runs a read's three sets
  on one binary (ROOT its configs; EXE, OUT absolute or from the repo root).
- `bench/ladder.py NEW[,...] OLD[,...] [--measures] [--bands B] [--deck N]`: the paired ladder per level by revision;
  `--screen DIR[,...]`: a screen's levers (`PAIR_L`, `PAIR-L`) against base, paired across DIRs. `--measures`: the
  adversary's and the boards' standing measures; `--pairs`: win % by unordered board pair (docstring). `bench/split_read.py READ OUT` splits a lever screen.
- `bench/paired.py DIR... [--pair A B]...`: cells' win %, SE, rounds and ends, and paired gaps pooled over `--pair`s
  on the seeds both played; `--against-base`, `--against-agent A`, `--lever-suffix L`, `--pool`, `--other DIR`,
  `--same-engine` and the cell names: its docstring. After editing ladder.py, paired.py, read_lib.py or screens.py, run
  `bench/test_read_tools.py`; after apply_overlay.py or design_check.py's arguments, `test_input_checks.py` (the slow lane runs both).
- Gates are suspended: `bench/gate_stop_rate.py` and `bench/gate_close.py` (docstrings).
- `bench/shards.sh EXE DIR SPEC FIRST COUNT OUT`: a farm spec natively in `SHARDS` (6) one-thread shards, under the header into OUT.
- `bench/first_divergence.py OLD NEW`: where two runs of one spec first differ, per game (`--cells DIR A B`: a run's
  two cells).
- `bench/profile_win.py SIM OUT -- run ...`: a sampling profile, no admin (speed.md, *Profiling*; flags: docstring).
- `bench/resource_use.py NEW [OLD] [--top 8] [--limits]` (folder, games file or `FILE=SPEC`): chips, energy, cubes and antimatter
  gained and spent per seat a game by level, what buys and sources them (`--limits`: the `limit.*` table).
  `bench/resource_split.py READ...` (flags: docstring): the same per arm, board and round, and spent in won games.
- `bench/read_lib.py READ[@LABEL]... [--base ARM] [--levels D,...] [--agent-arms] [--arm NAME=SPEC]... [--counters PREFIX...]
  [--cards ID...] [--md OUT]` (same forms): a paired read's tables (its docstring) by arm (`PAIR-ARM` variants) and
  agent, against the base arm on the same pair, level and seed (`@LABEL`: that read's arms are `LABEL/arm`;
  `--agent-arms`: `strategist:w.sink=1` is arm `sink1` of `strategist`; `--arm NAME=SPEC` names it, needed where two
  collide). Its `games`, `paired_gap` and helpers are the library for an item's own reader.
- `bench/fresh_read.py NEW OLD [--md OUT] [--growth SLUG,...]` (same forms): a fresh read against the old revision's,
  old → new per agent and level (columns: docstring).
- `bench/am_read.py READ[,READ...]` (same forms): antimatter by source, made and spent, per lever and
  agent.
- `bench/projects_read.py READ [--md OUT]` (folder, games file or `FILE=SPEC`): each project's reveals, completions, compute drained and map
  moves a completion, by level.
- `bench/card_ranks.py DIR A B [--variant V]...`: Spearman of the cards' keep and play rates, agent B against A.
- `bench/item_time.py ITEM... [--top N]`: a done item's wall time: sims, builds, tests, the rest.
- `tune/`: `agent_sweep.py`, `sweep.py`, `log_table.py`, `engine_sweep.py` ([README](../scripts/tune/README.md)).
  `noop_weights.py`: the `Weights` fields that change no game of an agent (docstring).
- Past items' one-off readers: `experiments/archive/scripts/`, kept for the write-ups citing them.
