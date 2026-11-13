# Scripts

What each script under `scripts/` is for, and how to run it. Run them from the repo root.

- `wasm-smoke.mjs`: plays one game through the wasm package in Node. `wasm-parity.mjs`:
  replays aic-sim records of each `configs/variants.json` variant in wasm.
- `farm/build.mjs BUNDLE`: the farm's wasm bundle. `farm/run.mjs BUNDLE SPEC FIRST COUNT OUT [--browser] [--port N]
  [--fingerprint]`: a shard played on it in Node or headless Edge (`web/scripts/cdp.mjs`), each seed timed.
  `farm/compare.py A B [--ms A_ERR B_ERR]`: two engines' outputs of one range: games that differ, win % and the paired
  gap, ms a game. See simulator.md, *The farm*.
- `regen.py [--no-build] [--check]`: builds release, runs `drafts/adversary/check.py`, checks `content/` against
  `drafts/adversary/` (`load_adversary.py --check`), writes `RULES_REVISION` into `docs/rules/README.md`, then `aic-sim docs`,
  `cards_doc.py`, `effect_ledger.py`, `shared_row.py`, `no_liberate.py`; prints the content hash and `git status`.
  `--check`: the `--check`s and the revision line only.
- `design_check.py [DIR] [--old DIR] [--base REV] [--quiet] [--usage]`: the design warnings ([principles.md](principles.md)):
  content checks, plus the play checks on a read's `games.jsonl` or a farm's out file (DIR; `--old` the previous
  revision's read on its seeds, `--base` the item's start for the dials, default the merge base with main). First line:
  the report's `warnings:`. Thresholds and each action's expected-use band (`usage`): `design_check.json`. `--usage`
  prints the inventory instead: each action's role, band, use pooled, by board and by level. Never exits 1.
- `configs/shared_row.py`: regenerates `configs/shared_row.json` from the baseline.
- `cards/cards_doc.py`: regenerates `docs/cards.md`. `cards/deck_text.py DECK...` prints a deck as plain text for a
  review, from the generated docs (`--list` the decks; README, *Card text for review*).
- `cards/apply_overlay.py FILE... [--check]`: writes content overlays (`cards.<id>.<path>=JSON` lines, the keys
  `--set` takes; `#` comments) into `content/*.json` in place, path rules as `apply_entry` in content.rs; rerunnable
  (CLAUDE.md). `cards.<id>=<JSON object>` writes a whole row: a new card is
  appended, an existing one replaced (`--set` has no such form). Nothing deletes: drop a card from the
  baseline deck (then `shared_row.py`), `cards.json` and `scenes.json` by hand. `--check`: a dry run. Then
  `regen.py`. It refuses hand-laid `alignment.json`; threats, adversary and projects: edit `drafts/adversary/`.
- `cards/loop_smoke.py [DIR...] [--run] [--spec S] [--games 16] [--seed S] [--threads 6] [--max-plays 12]`: the loop smoke before a full read
  (runs `use.json` to `out/loop-smoke` with no DIR): timeouts, ends, the most plays of one card in a game; exits 1
  on a timeout or `--max-plays`.
- `cards/counter_names.py DIR... [--all]`: measures gone quiet: the counter keys the read scripts, web scripts and
  `crates/aic-sim` (agents aside) quote that no game in DIR wrote (a key a rules change retired). Rare events and
  test strings list too: read it, don't gate on it.
- `cards/screen.py`: the screen summary: named columns over experiment outputs (`--columns a,b,c`, `--preset
  tiers|041|034|018|016`, `--sources`, `--list`); `--gate DIR` each check against its own agent (agent minus
  check, paired SE), then the unspent-resources readout. `--preset
  tiers` (offered, kept, played by tier): a card rework's first screen, as a tier-1 change may never show.
  `scripts/screens.py` holds what it shares with the other readers, and is the way to read a run from Python:
  `load(dir)` gives the games with their cells (`stream` also takes `FILE=SPEC`), `cnt(game, key)` a counter (a power is `ai.power.<board>.<name>`: `scripts/powers.py`).
  `python scripts/screens.py assemble SPEC FIRST COUNT OUT_DIR SHARD... [--exe EXE]` turns farmed shards (seed order)
  into an experiment output: the spec's header, then the shards, as `OUT_DIR/games.jsonl`.
- `cards/threat_read.py DIR... [--cards]`: per cell, threats dealt, countered by kind of option, paid abroad, and the
  typed counters and capstones played, from `games.jsonl`.
- `content/load_adversary.py [--check]`: `drafts/adversary/` (the source) into `content/` and `docs/adversary-cards.md`.
  Writing runs `drafts/adversary/check.py`'s checks first and writes nothing on any ERROR; `--check` only compares.
- `cards/play_read.py DIR... [--min PCT] [--agents A,B]`: per agent, pooled over its cells and DIRs: the draft cards kept in more than `--min` percent of games (default 1) and played
  in none, and every draft card no game kept. Seconds. A farmed read (no `results.json`) reads from its header
  spec, as do `screens.load` and `dealt` (182): agents by the spec's names, not resolved; an unknown one stops it.
- `cards/runaway_read.py DIR... [--times 5] [--floor 20] [--held 150] [--peak-times 3] [--peak-floor 10] [--top 15]`: loops:
  each trigger, threshold, reuse, discount or converter counter whose per-game maximum is over `--times` its mean
  where used; games whose insight grew three rounds running past `--held`; per cell, seat and resource the mean
  peak held, and games whose rise is over `--peak-times` its income a round. A bug hunt runs it on its fresh seeds.
- `cards/source_read.py DIR... [--top 5] [--pool] [--resource credits,insight,energy,chips|all]`: per cell (`--pool`:
  per agent) and board, each resource's top sources of gains a game (`sources`: `own:`/`other:` an action, `phase:`
  an automatic phase), and the keep counters (`keeps`: picks offered, kept, keep rate, tier-2 share, insight at the
  keep) split by runaway_read's insight flag. Seconds.
- `cards/slots_read.py DIR... [--rounds N]`: per cell, each seat's build-overs a game and its buildings standing as
  each round starts, from the `slots.*` counters.
- `cards/board_read.py DIR... [--powers] [--cards]`: per cell, pooled across DIRs: wins ± SE, purge wins, Patience's
  carried insight, cubes bought, free stations, the Researcher's second node, strikes a Wall stopped, troop hits, nodes
  lost and what seats leave unspent; `--powers` every board power a game, `--cards` the deck's research-cube cards
  (read from content) kept, played and used.
- `cards/use_read.py DIR... [--min PCT] [--agents A,B]`: per agent and pair, pooled over DIRs: wins, each dear
  power's uses a game (`ai::power_is_dear`), Grant, Action buildings' uses, tokens gained / spent, each liberate card
  kept / played (Popular Uprising's Action uses); then per agent `play_read.py`'s two lists. Seconds.
- `cards/liberation_read.py DIR... [--cards]`: per cell (config, pair, agent): wins ± SE, rounds, cities corrupted a
  game, cities liberated a game (card / action), the share of games with a liberation, Grant uses,
  strikes a Wall stopped and troops landed by the Wall pressure; `--cards` each liberate card's kept / played a game.
  Seconds.
- `cards/effect_stats.py READ [--screen DIR] [--flag 5] [--out MD]`: each Unity and Discord card's resolutions a game
  by tier and what one did (the engine's `fx.<id>.*`), each prohibition's fire rate and blocks; `--screen` an on/off
  screen (`PAIR_<id>`: its tiers blanked) for the paired win effect, flagged past ±5. Seconds.
- `configs/no_liberate.py`: writes `experiments/configs/no_liberate.json`, the baseline less every card printing
  `liberate` (read from content); `regen.py` reruns it.
- `cards/economy_read.py DIR... [--rounds 10]`: per cell, pooled across DIRs, per seat and game: draft cards
  kept (opening keeps apart) and played, kept and never played, nodes placed, credit income a round, credits and
  insight left unspent a round; each board's first dear power used a game; each seat's most-held tag by round
  (`ai.tag_top.<seat>`). Seconds.
- `cards/energy_read.py DIR...`: per cell: win %, the energy a seat holds as the action phase ends at rounds 3 and 6
  and its last round (`unspent.<seat>.energy.rNN`), then each energy mechanic's use a game.
- `cards/corp_read.py DIR...`: per cell, pooled across DIRs: wins, completions and their mean round, cubes paid by
  color, cubes stored and converted, uses per converter, stations claimed and built, adversary compute by source.
- `cards/price_read.py`: per-card rates and stratified win deltas from games.jsonl
  ([experiments.md](experiments.md), *Reading prices*).
- `cards/effect_ledger.py`: the effect ledger. It splits each card's printed cost over its effects pro rata to the
  reasoned values (`configs/agents/values.json`, an income valued over its tier's horizon), fits a price multiplier
  per effect, and writes CSVs, `ledger.html` and `outliers.md` to `experiments/results/effect-ledger/`; rerun at each
  card revision. `outliers.md` lists the tier-2 cards over the 2.5 ceiling at the table. `--stdout` prints each card's fit and fair price (card-pricing.md, *A fair price*), writing nothing;
  `--cards FILE`, `--values FILE` read proposed files; `--overlay FILE` (with `--stdout`) prices an overlay before it
  is written; `--hold-ratio OLD.csv` adds the price keeping an older `cards.csv`'s printed / reasoned ratio (repricing
  after a value change). Seconds.
- `cards/payback.py [--cards CARDS.json] [--old CARDS.json [--old-keep-tier2 N]]`: rounds each engine building takes to pay back its
  price at a committed seat's fire rates (card-pricing.md *Engines*), at `values.json`'s rates; `--old` adds an older
  deck's column.
- `run_sim.ps1 NAME [-NoBuild] ARGS...`: any long run: builds, copies the exe to `out/NAME.exe`, starts it below
  normal, detached; wait on `out/NAME.exit` (its header says more).
- `bench/same_results.py`: compares two experiment outputs. A cell's agent string that differs
  (`strategist` against `engine:strategy=1`) is listed and is no difference when the games match. `--map OLD=NEW`
  renames in the old output first (to swap two names, map one to a placeholder first).
- `bench/same_play.py`: proves a refactor left play unchanged: a matrix of configs, countries, difficulties and
  `configs/variants.json`, with every agent but the beam, on the old and new binaries (~20 min a run on 6 threads);
  every saved game replays on the new binary.
  `--map` as above (records then compared by field). An old binary that refuses today's configs plays its own
  (README, *This machine*): `run ... --root out/old`.
- `bench/speed.py` (flags: `--help`): games a minute per agent. `pair OLD NEW --out FILE` runs each row on both
  binaries in turn, so the ratio holds on a busy machine (`--old-config` for an OLD on another baseline; a row whose
  games differ prints NOT LIKE FOR LIKE). `table SIM --out FILE` is one binary's rates, `compare` reads two. `check
  SIM` is the standing check (about 2 min, quiet machine, one thread): fails at 1.3 × `speed_baseline.json`,
  `--update` moves it; across a revision it prints NO VERDICT (CLAUDE.md). `--games N` sets the many-thread column's
  games (`table`'s greedy 200: about 3 min on one thread since 174); `pair` sizes rows from the baseline, ~10 s a pass.
- `bench/per_decision.py OLD_ROOT NEW_ROOT`: rounds, decisions and ms a game and a decision on the same seeds, to
  tell longer games from a slower agent. Across revisions use `--games 16`+ (~8 min for the default `--agent
  planner`; name the agents a change touched). A root is an `.exe` or a directory with `aic-sim.exe` (or
  `target/release/`'s) and its `configs/`.
  `--set K=V` (repeatable) goes to both sides after the built-in ones: a content overlay runs the new binary on old
  content.
- `bench/seeds.py next N [--take] [--doc FILE]`: the next N fresh seeds from experiments.md's ledger; `--take` moves its `Next free` past them.
- `bench/gate_stop_rate.py`: how often the gate's early stop flips a verdict (at `z` 2, about 1%).
- `bench/skilled_rows.py READ.json [--games 24] [--levels 3,4] [--rows ...] [--out ...]`: the oracle's companion spec to a read (experiments.md, *Skilled rows*).
- `bench/read_specs.py {offs|screen|read} ... --tag NNN [--cards a,b,c] [--out-dir DIR]`: a rules item's specs into `out/`
  (or `experiments/farm`): `read TAG SEED [AGENT] [--probe] [--levels 0,1,2,3,4] [SET...]` three 48-seed sets,
  the three pairs, d0-d3; `screen NAME AGENTS LEVELS SEEDS [--offs] [--first N] [LEVER...]` a dev screen (`tag:a=1;b=2` a lever); `offs` a config per `--cards` card switched off.
  `bench/run_read.sh NNN EXE ROOT TAG OUT THREADS` runs a read's three sets on one binary (ROOT its configs; EXE, OUT absolute or from the repo root).
- `bench/ladder.py NEW[,...] OLD[,...] [--measures] [--bands 90-100,70-80,55-65,40-50] [--deck N]`: the paired ladder
  read per difficulty, sides labeled by rules revision; `--screen DIR[,...]` a screen's levers against base, paired
  across DIRs. Reads a farmed read (no `results.json`), or a farm's bare out file as `FILE=SPEC.json` where a DIR
  goes; an agent is named as its spec writes it, native or farmed; engines and `--same-engine` as `paired.py`. Each
  win % is its side's whole rate. `--measures` ends with 210's table by board: slack and bound seat-rounds, land
  tokens a round, node placement spread.
- `bench/gate_close.py`: `plan GATE_DIR OUT_DIR` picks the rows within 2 SE by gap against paired SE and writes specs
  that carry them on to the cap, reusing their games; `read` merges the runs (`--min-seeds N` leaves out rows short of
  N seeds, and says how many).
- `bench/paired.py DIR... [--pair A B]... [--against-base] [--same-engine]`: any run's cells with win %, SE, rounds
  and ends, and paired gaps pooled over `--pair`s on the seeds both played (`--same-engine`: one engine's seeds only).
  `bench/test_read_tools.py` tests both. A `--pair` cell is its exact name
  (`bp/strategist`, or `DIR/bp/strategist`); a spec with several configs or difficulties leads it with the config's
  file stem and `dN` (`no_liberate/d2/bp/strategist`; `screens.cell_labels`). `--against-base` reads a price sweep:
  every variant `cN-x` against `cN`, per agent, each lever pooled over its sizes.
- `bench/first_divergence.py OLD NEW`: where two runs of one spec first play differently, game by game; `--cells DIR
  A B` two cells of one run.
- `bench/profile_win.py SIM OUT -- run ...`: a sampling profile, no admin (docs/speed.md, *Profiling*). `--runs K` on the next seeds each; `--attach S1,S2,...` reads 5 s windows of a long
  run; writes `self.csv`, `total.csv`, `callers.csv`, `edges.csv`.
- `bench/card_ranks.py DIR A B [--variant V]...`: Spearman of each card's keep and play rates, agent B against A
  (as `cards.csv` names them); an approximation's bar.
- `bench/item_time.py ITEM... [--top N]`: a done conductor item's wall time by sims, builds, tests and the rest.
- `tune/`: `agent_sweep.py` (the strategist's parameter sweep), `sweep.py`, `log_table.py`, `engine_sweep.py`; see
  [scripts/tune/README.md](../scripts/tune/README.md).
- Past items' one-off readers are in `experiments/archive/scripts/`, kept for the write-ups that cite them; 034's is
  `cards/screen.py --preset 034`.
