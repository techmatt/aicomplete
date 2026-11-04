# Scripts

What each script under `scripts/` is for, and how to run it. Run them from the repo root.

Current:

- `wasm-smoke.mjs`: plays one game through the wasm package in Node. `wasm-parity.mjs`:
  replays aic-sim records of every variant in wasm.
- `configs/shared_row.py`: regenerates `configs/shared_row.json` from the baseline.
- `cards/cards_doc.py`: regenerates `docs/cards.md`.
- `cards/screen.py`: the screen summary: a table of named columns over experiment outputs
  (`--columns a,b,c`, `--preset tiers|041|034|028|018|016`, `--sources`, `--list`) and `--gate DIR`,
  each check against its own agent in one sign, agent minus check, with the paired SE, and games
  per check when `results.json` has a `gate` key, then the unspent-resources readout. It reads a close run that holds its planner row
  too, gate key or not; report.md's agent pairs read the other way, check minus reference. `--preset tiers` is cards offered, kept and played per game by tier: run it on the
  first screen of a card rework, since agents draft tier 2 about three picks in four
  ([card-pricing.md](card-pricing.md)) and a tier-1 change may never show.
  `scripts/screens.py` holds what it shares with the kept scripts below, and is the way to read a run from Python: `load(dir)` gives the games with their cells, `cnt(game, key)` a counter (a power is `ai.power.<board>.<k>`).
- `cards/threat_read.py DIR... [--cards]`: per cell, threats dealt, countered by source type, paid abroad or by ambassador
  alone, sources made, and the typed counters and capstones played: who counters what, from `games.jsonl`.
- `cards/play_read.py DIR... [--min PCT] [--agents A,B]`: per agent, pooled over its cells and DIRs (each with
  `games.jsonl` and `results.json`): the draft cards kept in more than `--min` percent of games (default 1) and played
  in none, and every draft card no game kept. AI-unique cards (there are none) are left out. Seconds.
- `cards/board_read.py DIR... [--powers] [--cards]`: per cell, pooled across DIRs: wins ± SE, the win mix, Patience's carried insight, cubes bought, free stations, the Researcher's second node, strikes a Wall stopped, troop hits, nodes lost and what seats leave unspent; `--powers` every board power a game, `--cards` the six research-cube cards kept, played and used
- `cards/use_read.py DIR... [--min PCT] [--agents A,B]`: per agent and pair, pooled over DIRs: wins, each dear
  power's uses a game (a board power sacrificing two tags or more), Grant, Action buildings' uses, tokens gained / spent,
  each liberate card kept / played (Popular Uprising's Action uses); then per agent the draft cards kept in over `--min`
  percent of games and never played, and those never kept. Seconds.
- `cards/liberation_read.py DIR... [--cards]`: per cell (config, pair, agent): wins ± SE, rounds, cities corrupted a
  game, cities liberated a game (by a card / by the fallback), the share of games with a liberation, Grant uses, strikes a
  Wall stopped and troops landed by the Wall pressure; `--cards` each liberate card's kept / played (Popular Uprising's
  Action uses) a game. Seconds.
- `configs/no_liberate.py`: writes `experiments/configs/no_liberate.json`, the baseline with the seven liberate
  capstones out of the deck (the liberation-off arm of `screen-080-off.json`); rerun after editing the baseline.
- `cards/economy_read.py DIR... [--rounds 10]`: per cell, pooled across DIRs, per seat and game: draft cards
  kept (opening keeps apart) and played, kept and never played, nodes placed, credit income a round, credits and
  insight left unspent a round; each board's dear power (2 or more of one tag) used a game with the seat-rounds it
  lacked the tags; then tags by round: each seat's most-held tag on its cards in play (`ai.tag_top.<seat>`),
  mean and share at 3 or more. A variant's seats follow its `ai.boards=` override. Reads r32 outputs too (no tag
  table). Seconds.
- `cards/corp_read.py DIR...`: per cell, pooled across DIRs: wins, completions and their mean round, cubes paid by color, cubes stored and converted, uses per converter, stations claimed and built, adversary compute by source
- `cards/price_read.py`: per-card rates and stratified win deltas from games.jsonl
  ([experiments.md](experiments.md), *Reading prices*). `kept` is a share of games, opening deal and draws
  included: never a share of offered, which would pass 100.
- `cards/effect_ledger.py`: rewrites `experiments/results/effect-ledger/`, where card prices disagree
  with each other and with the value table; rerun at each card revision. Its docstring says how it reads.
- `run_sim.ps1 NAME [-NoBuild] ARGS...`: builds release, stops if the build failed, copies
  the exe to `out/NAME.exe`, starts `aic-sim ARGS` below normal and returns. Output goes to
  `out/NAME.out` and `.err`; `out/NAME.exit` appears at the end, holding the exit code: wait on
  that file. Use it for any long run.
- Kept for the results docs that cite them, each saying so at its top: `cards/screen_034.py`
  (a shim for `screen.py --preset 034`), `cards/screen_028.py` (r17), `cards/screen_027.py`
  (r16), `cards/screen_024.py` (r15); `screen_023.py` and `screen_022.py` still read r15; `cards/screen_021.py` (r12) runs, but its
  gate is not the current one.
- `bench/same_results.py`: compares two experiment outputs. `--map OLD=NEW`
  renames in the old output first, for a rename that must play the same. 086's (presence →
  node, node → botnet; `experiments/same-086.json`, 48 strategist games, about 12 s on 4
  threads) shields the card ids first: `--map node_severance=QQ1 --map node_swarm=QQ2 --map
  "Node Severance=QQ3" --map "Node Swarm=QQ4" --map NODE=BOTNET --map node=botnet --map Node=Botnet
  --map PRESENCE=NODE --map presence=node --map Presence=Node`, then each `QQn` back. The spec's
  own note is then the one difference.
- `bench/same_play.py`: proves a refactor left play unchanged: a matrix of configs,
  countries, difficulties and variants, with every agent but the beam (rollout and
  planner included, which are what a speedup is for), on the old and new binaries, ~20 min a
  run on 12 threads; every saved game replays on the new binary. `run ... --threads N`
  caps it on a shared machine; `compare` takes under a minute. Keep a copy of the old binary. Takes `--map` too: records are then compared by field, since a renamed
  state's fingerprint moves. Each difference prints its first differing field or trace line. An
  old binary that refuses today's configs plays its own: `mkdir -p out/old && git archive REV
  configs | tar -x -C out/old`, then `run ... --root out/old`.
- `bench/speed.py`: games a minute per agent. `--threads N` caps the many-thread
  column of `table`, `pair` and `specs` (default: leaves 4 hardware threads free). `--games N` sets that
  column's games in `table`, `pair` and `specs`; each row prints its games and a rough time
  before it starts. `pair OLD NEW --out
  FILE` runs each row on both binaries in turn, so the ratio holds on a busy machine: use it
  for a before and after. An OLD built on another baseline refuses today's config: `git show
  <rev>:configs/baseline.json > out/old-baseline.json`, then `--old-config
  out/old-baseline.json`. Each row hashes what its games came to on both binaries: a row whose
  games differ prints NOT LIKE FOR LIKE (its ratio is speed times game length); across a config or
  rules change, a row whose games match (a config-key rename) prints like for like. `table SIM --out FILE` is one binary's rates (`rollout` alone takes ~20 min;
  `--only "planner c2"` for a row, and both keep the file's other rows). `compare BEFORE.json AFTER.json` reads two `table`
  files. `check SIM` is
  the standing check (about 2 min, on a quiet machine, always one thread): fails at 1.3 ×
  `scripts/bench/speed_baseline.json`, `--update` to move the baseline. Its
  planner row is four seeds: one seed can misread a change by several times. It
  holds only on the same games: the baseline records its revision and a hash of what each
  agent's check games came to, from `run --json`. Against another revision, or the same revision
  with other games (a default, weight or config moved), `check` prints NO VERDICT, says which, and
  exits 0. Read `per_decision.py` then, and `--update`.
  A `--update` compares a run with itself: run a plain `check` after it.
- `bench/per_decision.py OLD_ROOT NEW_ROOT`: rounds, decisions and ms per game and per
  decision on the same seeds, for telling longer games from a slower agent. 6 games is
  too few across rules revisions; `--games 16` or more. Bad arguments stop it before anything runs: each root is
  an `.exe` or a directory holding `aic-sim.exe`.
- `bench/gate_stop_rate.py`: how often the gate's early stop flips a verdict.
- `bench/gate_close.py`: `plan GATE_DIR OUT_DIR` picks the rows within 2 SE by gap against paired SE and writes specs that carry them on to the cap, reusing their games; `read` merges the runs (`--min-seeds N` leaves out rows short of N seeds, and says how many).
- `bench/paired.py DIR... [--pair A B]... [--against-base]`: any run's cells with win %, SE,
  rounds and ends, and paired gaps between any two cells, pooled over several `--pair`s. A
  `--pair` cell is its exact name (`bp/strategist`, or `DIR/bp/strategist`). `sweep` follows the
  engine's defaults: against a pre-082 run, name the old ones (`sweep:plan_table=0,w.table=0,w.packet_keep=1`).
  `--against-base` reads a price sweep: every variant `cN-x` against `cN`, per agent, and
  each lever pooled over its sizes. The spec's variants are `c2`, `c3`, `c2-x`, `c3-x`, each with
  its `countries=N` and the lever's overlay in `set`. report.md's variant section pairs only
  against the first variant, so it reads a c3 lever against c2.
- Two agent specs on one binary: `bench/speed.py specs SIM A B`, `bench/per_decision.py
  SIM.exe --old-agent A --agent B`, and `bench/first_divergence.py --cells DIR A B` for where two
  cells of one run first play differently.
- `bench/first_divergence.py OLD NEW`: where two runs of one spec first play differently,
  game by game.
- `tune/`: `sweep.py`, `log_table.py`, `engine_sweep.py`.

`--preset 016` and `018` cover what `scripts/board/` and `scripts/alignment/` read; git has those for the
results docs that cite them.
