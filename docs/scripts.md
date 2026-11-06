# Scripts

What each script under `scripts/` is for, and how to run it. Run them from the repo root.

Current:

- `wasm-smoke.mjs`: plays one game through the wasm package in Node. `wasm-parity.mjs`:
  replays aic-sim records of every variant in wasm.
- `regen.py [--no-build] [--check]`: builds release, then `aic-sim docs`, `cards_doc.py`, `effect_ledger.py`,
  `shared_row.py`, `no_liberate.py`; prints the content hash, rules revision and `git status`. `--check`: the two `--check`s only.
- `configs/shared_row.py`: regenerates `configs/shared_row.json` from the baseline.
- `cards/cards_doc.py`: regenerates `docs/cards.md`.
- `cards/apply_overlay.py FILE... [--check]`: writes content overlays (`cards.<id>.<path>=JSON` lines, the keys
  `--set` takes; `#` comments) into `content/*.json` in place, path rules as `apply_entry` in content.rs; a card pass
  keeps its edits as one overlay file and reruns it. `cards.<id>=<JSON object>` writes a whole row: a new card is
  appended, an existing one replaced (125; `--set` has no such form). Nothing deletes: drop a card from the
  baseline deck (then `shared_row.py`), `cards.json` and `scenes.json` by hand. `--check` prints what would change. Run `regen.py` after.
- `cards/loop_smoke.py [DIR...] [--run] [--spec S] [--games 16] [--max-plays 12]`: the loop smoke before a full read
  (runs `use.json` to `out/loop-smoke` with no DIR): timeouts, ends, the most plays of one card in a game; exits 1
  on a timeout or `--max-plays`.
- `cards/screen.py`: the screen summary: a table of named columns over experiment outputs
  (`--columns a,b,c`, `--preset tiers|041|034|028|018|016`, `--sources`, `--list`) and `--gate DIR`,
  each check against its own agent in one sign, agent minus check, with the paired SE, and games
  per check when `results.json` has a `gate` key, then the unspent-resources readout. It reads a close run that holds
  its planner row too, gate key or not; report.md's agent pairs read the other way, check minus reference. `--preset
  tiers` is cards offered, kept and played per game by tier: run it on the first screen of a card rework, since agents
  draft tier 2 about three picks in four ([card-pricing.md](card-pricing.md)) and a tier-1 change may never show.
  `scripts/screens.py` holds what it shares with the other readers, and is the way to read a run from Python:
  `load(dir)` gives the games with their cells, `cnt(game, key)` a counter (a power is `ai.power.<board>.<k>`), read
  under its former name (`RENAMED`) in older runs.
- `cards/threat_read.py DIR... [--cards]`: per cell, threats dealt, countered by kind of option, paid abroad, and the
  typed counters and capstones played, from `games.jsonl`.
- `content/load_adversary.py [--check]`: `drafts/adversary/` (the source) into `content/` and `docs/adversary-cards.md`.
- `cards/play_read.py DIR... [--min PCT] [--agents A,B]`: per agent, pooled over its cells and DIRs (each with
  `games.jsonl` and `results.json`): the draft cards kept in more than `--min` percent of games (default 1) and played
  in none, and every draft card no game kept. Seconds.
- `cards/runaway_read.py DIR... [--times 5] [--floor 20] [--held 150] [--peak-times 3] [--peak-floor 10]`: loops:
  each trigger, threshold, reuse, discount or converter counter whose per-game maximum is over `--times` its mean
  where used; the games whose insight after the keep grew three rounds running past `--held`; per cell, seat and
  resource the mean peak held, and games whose rise (peak less `start`, setup's holding; before 124 the peak) is over
  `--peak-times` its income a round and `--peak-floor`. A bug hunt runs it on its fresh seeds.
- `cards/source_read.py DIR... [--top 5] [--pool] [--resource credits,insight,energy,chips|all]`: per cell (`--pool`:
  per agent) and board, each resource's top sources of gains a game (`sources`: `own:`/`other:` an action, `phase:`
  an automatic phase), and the keep counters (`keeps`: picks offered, kept, keep rate, tier-2 share, insight at the
  keep) split by runaway_read's insight flag. Runs from 124 on. Seconds.
- `cards/slots_read.py DIR... [--rounds N]`: per cell, each seat's build-overs a game and its buildings standing as
  each round starts, from the `slots.*` counters.
- `cards/board_read.py DIR... [--powers] [--cards]`: per cell, pooled across DIRs: wins ± SE, the win mix, Patience's
  carried insight, cubes bought, free stations, the Researcher's second node, strikes a Wall stopped, troop hits, nodes
  lost and what seats leave unspent; `--powers` every board power a game, `--cards` the deck's research-cube cards
  (read from content) kept, played and used.
- `cards/use_read.py DIR... [--min PCT] [--agents A,B]`: per agent and pair, pooled over DIRs: wins, each dear
  power's uses a game (`ai::power_is_dear`: two tags or more, or a tag and an energy and no node), Grant, Action
  buildings' uses, tokens gained / spent, each liberate card kept / played (Popular Uprising's Action uses); then per
  agent the draft cards kept in over `--min` percent of games and never played, and those never kept. Seconds.
- `cards/liberation_read.py DIR... [--cards]`: per cell (config, pair, agent): wins ± SE, rounds, cities corrupted a
  game, cities liberated a game (by a card / by the fallback), the share of games with a liberation, Grant uses,
  strikes a Wall stopped and troops landed by the Wall pressure; `--cards` each liberate card's kept / played a game.
  Seconds.
- `configs/no_liberate.py`: writes `experiments/configs/no_liberate.json`, the baseline with the seven liberate
  capstones out of the deck; rerun after editing the baseline (`regen.py` does).
- `cards/economy_read.py DIR... [--rounds 10]`: per cell, pooled across DIRs, per seat and game: draft cards
  kept (opening keeps apart) and played, kept and never played, nodes placed, credit income a round, credits and
  insight left unspent a round (these three per seat-round that reached upkeep); each board's first dear power (as
  `use_read.py`) used a game with the seat-rounds it lacked the tags; then each seat's most-held tag on its cards in
  play by round (`ai.tag_top.<seat>`), mean and share at 3 or more. A variant's seats follow its `ai.boards=`
  override. Seconds.
- `cards/energy_read.py DIR...`: per cell: win %, the energy a seat holds as the action phase ends at rounds 3 and 6
  and its last round (`unspent.<seat>.energy.rNN`), then each energy mechanic's use a game. Seconds.
- `cards/corp_read.py DIR...`: per cell, pooled across DIRs: wins, completions and their mean round, cubes paid by
  color, cubes stored and converted, uses per converter, stations claimed and built, adversary compute by source.
- `cards/price_read.py`: per-card rates and stratified win deltas from games.jsonl
  ([experiments.md](experiments.md), *Reading prices*). `kept` is a share of games, opening deal and draws
  included: never a share of offered, which would pass 100.
- `cards/effect_ledger.py`: the effect ledger. It splits each card's printed cost over its effects pro rata to the
  reasoned values (`configs/agents/values.json`, an income valued over its tier's horizon), fits a price multiplier
  per effect, and writes CSVs, a sortable `ledger.html` and `outliers.md` to `experiments/results/effect-ledger/`;
  rerun at each card revision. `--stdout` writes nothing and prints each card's fit and fair credit price
  (card-pricing.md, *A fair price*); `--cards FILE` and `--values FILE` read a proposed `cards.json` or
  `values.json`; `--overlay FILE` (with `--stdout`) applies a card pass's overlay first, to price it before
  writing it; `--hold-ratio OLD.csv` adds the credit price that keeps each card's printed / reasoned ratio of an
  older `cards.csv` (`git show REV:experiments/results/effect-ledger/cards.csv`), for repricing after a value change.
  Its Python mirror of `agents/value.rs` must change with it (the fast-lane `effect_ledger_mirrors_the_table` checks
  the committed `cards.csv`). "Remove up to N troops" counts at what it removes in play (`values.json`
  `troops_removed`, which value.rs reads too, from the engine's `cards.troops_removed.<N>` /
  `cards.remove_troops.<N>` counters; up to 6 removes about 3.6). Seconds.
- `cards/payback.py [--old CARDS.json [--old-keep-tier2 N]]`: rounds each engine building takes to pay back its
  price at a committed seat's fire rates (card-pricing.md *Engines*), at `values.json`'s rates; `--old` adds an older
  deck's column.
- `run_sim.ps1 NAME [-NoBuild] ARGS...`: builds release, stops if the build failed, copies
  the exe to `out/NAME.exe`, starts `aic-sim ARGS` below normal and returns. Output goes to
  `out/NAME.out` and `.err`; `out/NAME.exit` appears at the end, holding the exit code: wait on
  that file. Use it for any long run.
- `bench/same_results.py`: compares two experiment outputs. A cell's agent string that differs
  (`strategist` against `engine:strategy=1`) is listed and is no difference when the games match. `--map OLD=NEW`
  renames in the old output first, for a rename that must play the same. To swap two names, map one to a placeholder
  (`QQ1`) first and back last.
- `bench/same_play.py`: proves a refactor left play unchanged: a matrix of configs, countries, difficulties and
  variants, with every agent but the beam (rollout and planner included), on the old and new binaries, ~20 min a run
  on 12 threads; every saved game replays on the new binary. `run ... --threads N` caps it on a shared machine;
  `compare` takes under a minute. Keep a copy of the old binary. Takes `--map` too: records are then compared by
  field, since a renamed state's fingerprint moves. Each difference prints its first differing field or trace line.
  An old binary that refuses today's configs plays its own: `mkdir -p out/old && git archive REV configs | tar -x -C
  out/old`, then `run ... --root out/old`.
- `bench/speed.py`: games a minute per agent. In `table`, `pair` and `specs`, `--threads N` caps the many-thread
  column (default: leaves 4 hardware threads free) and `--games N` sets its games; each row prints its games and a
  rough time first. `pair OLD NEW --out FILE` runs each row on both binaries in turn, so the ratio holds on a busy
  machine: use it for a before and after. An OLD built on another baseline takes `--old-config` (its
  `configs/baseline.json`, from `git show`). A row whose games differ on the two prints NOT LIKE FOR LIKE (its ratio is
  speed times game length). `table SIM --out FILE` is one binary's rates (`rollout` alone ~20 min; `--only "planner c2"` for a
  row, keeping the file's others). `compare BEFORE.json AFTER.json` reads two `table` files. `check SIM` is the
  standing check (about 2 min, quiet machine, one thread): fails at 1.3 × `scripts/bench/speed_baseline.json`,
  `--update` moves it. It holds only on the same games (the baseline records its
  revision and a hash of each agent's check games): on other games or another revision it prints NO VERDICT, says
  which, and exits 0; read `per_decision.py`, then `--update`, then a plain `check`.
- `bench/per_decision.py OLD_ROOT NEW_ROOT`: rounds, decisions and ms per game and per decision on the same seeds,
  for telling longer games from a slower agent. Use `--games 16` or more across rules revisions (about 8 min for the
  default `--agent planner`; name the agents a change touched). A `speed.py check --update` and the check after it
  are about 4 min more. Each root is an `.exe` or a directory holding `aic-sim.exe` and the `configs/` it plays.
  `--set K=V` (repeatable) goes to both sides after the built-in ones: a content overlay runs the new binary on old
  content.
- `bench/gate_stop_rate.py`: how often the gate's early stop flips a verdict.
- `bench/gate_close.py`: `plan GATE_DIR OUT_DIR` picks the rows within 2 SE by gap against paired SE and writes specs
  that carry them on to the cap, reusing their games; `read` merges the runs (`--min-seeds N` leaves out rows short of
  N seeds, and says how many).
- `bench/paired.py DIR... [--pair A B]... [--against-base]`: any run's cells with win %, SE, rounds and ends, and
  paired gaps between any two cells, pooled over several `--pair`s. A `--pair` cell is its exact name
  (`bp/strategist`, or `DIR/bp/strategist`); a spec with several configs or difficulties leads it with the config's
  file stem and `dN` (`no_liberate/d2/bp/strategist`; `screens.cell_labels`). `--against-base` reads a price sweep:
  every variant `cN-x` against `cN`, per agent, and each lever pooled over its sizes. The spec's variants are `c2`,
  `c3`, `c2-x`, `c3-x`, each with its `countries=N` and the lever's overlay in `set`. report.md's variant section
  pairs only against the first variant, so it reads a c3 lever against c2.
- Two agent specs on one binary: `bench/speed.py specs SIM A B`, `bench/per_decision.py SIM.exe --old-agent A
  --agent B`, and `bench/first_divergence.py --cells DIR A B` for where two cells of one run first play differently.
- `bench/first_divergence.py OLD NEW`: where two runs of one spec first play differently, game by game.
- `bench/profile_win.py SIM OUT -- run ...`: a sampling profile, no admin: Visual Studio's collector and `xperf`
  (docs/speed.md, *Profiling*). `--runs K` on the next seeds each; `--attach S1,S2,...` reads 5 s windows of a long
  run; writes `self.csv`, `total.csv`, `callers.csv`, `edges.csv`.
- `bench/card_ranks.py DIR A B`: Spearman of each card's keep and play rates, agent B against A (as `cards.csv` names
  them: the strategist is `engine:strategy=1`); an approximation's bar.
- `bench/item_time.py ITEM...`: how a done conductor item's wall time split between sims, builds, tests and the
  rest, from its run.json and main transcript (read only); `--top N` lists the unclassified calls.
- `tune/agent_sweep.py`: the strategist's parameter sweep: coordinate search over `tune/agent_sweep.json`'s grids on
  fixed seeds, a profile around the best, a paired confirm on fresh seeds against hand-set and greedy, and the
  counters that differ. Resumes from its cache; flags and recipe in [scripts/tune/README.md](../scripts/tune/README.md).
- `tune/`: `sweep.py`, `log_table.py`, `engine_sweep.py`.
- Past-revision reads, each described at its top: `cards/r37_read.py` (096), `cards/r38_read.py` (098). Kept for
  the results docs that cite them, each saying so at its top: `cards/screen_0NN.py` (034 a shim for `--preset 034`).
