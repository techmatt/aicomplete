# Scripts

What each script under `scripts/` is for, and how to run it. Run them from the repo root.

Current:

- `wasm-smoke.mjs`: plays one game through the wasm package in Node. `wasm-parity.mjs`
  (020): replays aic-sim records of every variant in wasm.
- `configs/shared_row.py`: regenerates `configs/shared_row.json` from the baseline.
- `cards/cards_doc.py`: regenerates `docs/cards.md`.
- `cards/screen.py`: the screen summary: a table of named columns over experiment outputs
  (`--columns a,b,c`, `--preset tiers|041|034|028|018|016`, `--sources`, `--list`) and `--gate DIR`,
  each check against its own agent in one sign, agent minus check, with the paired SE, and games
  per check when `results.json` has a `gate` key, then the unspent-resources readout (068). It reads a close run that holds its planner row
  too, gate key or not; report.md's agent pairs read the other way, check minus reference. `--preset tiers` (052) is cards offered, kept and played per game by tier: run it on the
  first screen of a card rework, since agents draft tier 2 about three picks in four
  ([card-pricing.md](card-pricing.md)) and a tier-1 change may never show.
  `scripts/screens.py` holds what it shares with the kept scripts below, and is the way to read a run from Python: `load(dir)` gives the games with their cells, `cnt(game, key)` a counter (a power is `ai.power.<board>.<k>`).
- `cards/threat_read.py DIR... [--cards]` (070): per cell, threats dealt, countered by source type, paid abroad or by ambassador
  alone, sources made, and the typed counters and capstones played: who counters what, from `games.jsonl`.
- `cards/play_read.py DIR... [--min PCT] [--agents A,B]` (076): per agent, pooled over its cells and DIRs (each with
  `games.jsonl` and `results.json`): the draft cards kept in more than `--min` percent of games (default 1) and played
  in none, and every draft card no game kept. AI-unique cards (none since 078) are left out. Seconds.
- `cards/board_read.py DIR... [--powers] [--cards]` (075): per cell, pooled across DIRs: wins ± SE, the win mix, Patience's carried insight, cubes bought, free stations, the Researcher's second presence, strikes a Wall stopped, troop hits, presence lost and what seats leave unspent; `--powers` every board power a game, `--cards` the six research-cube cards kept, played and used
- `cards/economy_read.py DIR... [--rounds 10]` (078): per cell, pooled across DIRs, per seat and game: draft cards
  kept (opening keeps apart) and played, kept and never played, presence placed, credit income a round, credits and
  insight left unspent a round; each board's dear power (2 or more of one tag) used a game with the seat-rounds it
  lacked the tags; then tags by round: each seat's most-held tag on its cards in play (`ai.tag_top.<seat>`, 078),
  mean and share at 3 or more. A variant's seats follow its `ai.boards=` override. Reads r32 outputs too (no tag
  table). Seconds.
- `cards/corp_read.py DIR...` (074): per cell, pooled across DIRs: wins, completions and their mean round, cubes paid by color, cubes stored and converted, uses per converter, stations claimed and built, adversary compute by source
- `cards/price_read.py` (055): per-card rates and stratified win deltas from games.jsonl
  ([experiments.md](experiments.md), *Reading prices*). `kept` is a share of games, opening deal and draws
  included: never a share of offered, which would pass 100. Older price-read CSVs still have
  the `keep_rate` column this removed (065).
- `run_sim.ps1 NAME [-NoBuild] ARGS...` (052): builds release, stops if the build failed, copies
  the exe to `out/NAME.exe`, starts `aic-sim ARGS` below normal and returns. Output goes to
  `out/NAME.out` and `.err`; `out/NAME.exit` appears at the end, holding the exit code: wait on
  that file. Use it for any long run.
- Kept for the results docs that cite them, each saying so at its top: `cards/screen_034.py`
  (a shim for `screen.py --preset 034`), `cards/screen_028.py` (r17), `cards/screen_027.py`
  (r16), `cards/screen_024.py` (r15); `screen_023.py` and `screen_022.py` still read r15; `cards/screen_021.py` (r12) runs, but its
  021 gate is no longer the gate.
- `bench/same_results.py` (012): compares two experiment outputs. `--map OLD=NEW` (026)
  renames in the old output first, for a rename that must play the same.
- `bench/same_play.py` (025): proves a refactor left play unchanged: a matrix of configs,
  countries, difficulties and variants, with every agent but the beam (030: rollout and
  planner too, which are what a speedup is for), on the old and new binaries, ~20 min a
  run on 12 threads; every saved game replays on the new binary. `run ... --threads N`
  (042) caps it on a shared machine: ~13 min a run on 4 threads at r20, two at once, and
  `compare` under a minute. Keep a copy of the old binary. Takes `--map` too (026): records are then compared by field, since a renamed
  state's fingerprint moves. Each difference prints its first differing field or trace line. An
  old binary that refuses today's configs plays its own: `mkdir -p out/old && git archive REV
  configs | tar -x -C out/old`, then `run ... --root out/old` (065).
- `bench/speed.py` (030): games a minute per agent. `--threads N` caps the many-thread
  column of `table`, `pair` and `specs` (default: leaves 4 hardware threads free). `--games N` sets that
  column's games in `table`, `pair` and `specs`; each row prints its games and a rough time
  before it starts. `pair OLD NEW --out
  FILE` runs each row on both binaries in turn, so the ratio holds on a busy machine: use it
  for a before and after. An OLD built on another baseline refuses today's config: `git show
  <rev>:configs/baseline.json > out/old-baseline.json`, then `--old-config
  out/old-baseline.json`. Across a config or rules change `pair` prints NOT LIKE FOR LIKE: the
  two play different games, so its ratio is speed times game length. A row whose games differ at
  the same revision and config prints the same warning. `table SIM --out FILE` is one binary's rates (`rollout` alone takes ~20 min;
  `--only "planner c2"` for a row, and both keep the file's other rows). `compare BEFORE.json AFTER.json` reads two `table`
  files. `check SIM` is
  the standing check (about 2 min, on a quiet machine, always one thread): fails at 1.3 ×
  `scripts/bench/speed_baseline.json`, `--update` to move the baseline. *What r19 and r20
  cost* in [experiments/history.md](../experiments/history.md) says why it is above r18's. Its
  planner row is four seeds (050; one seed read r18 to r20 as 6.7 × against 1.75 × on 16). It
  holds only on the same games: the baseline records its revision and a hash of what each
  agent's check games came to, from `run --json`. Against another revision, or the same revision
  with other games (a default, weight or config moved), `check` prints NO VERDICT, says which, and
  exits 0. Read `per_decision.py` then, and `--update`.
  A `--update` compares a run with itself: run a plain `check` after it.
- `bench/per_decision.py OLD_ROOT NEW_ROOT` (040): rounds, decisions and ms per game and per
  decision on the same seeds, for telling longer games from a slower agent. 6 games is
  too few across rules revisions (042: the same two binaries read 1.73 × on 6 seeds, 1.66 ×
  on 16 others); `--games 16` or more. Bad arguments stop it before anything runs: each root is
  an `.exe` or a directory holding `aic-sim.exe`.
- `bench/gate_stop_rate.py`: how often the gate's early stop flips a verdict.
- `bench/gate_close.py` (068): `plan GATE_DIR OUT_DIR` picks the rows within 2 SE by gap against paired SE and writes specs that carry them on to the cap, reusing their games; `read` merges the runs (`--min-seeds N` leaves out rows short of N seeds, and says how many).
- `bench/paired.py DIR... [--pair A B]... [--against-base]` (064): any run's cells with win %, SE,
  rounds and ends, and paired gaps between any two cells, pooled over several `--pair`s.
  `--against-base` (065) reads a price sweep: every variant `cN-x` against `cN`, per agent, and
  each lever pooled over its sizes. The spec's variants are `c2`, `c3`, `c2-x`, `c3-x`, each with
  its `countries=N` and the lever's overlay in `set`. report.md's variant section pairs only
  against the first variant, so it reads a c3 lever against c2.
- Two agent specs on one binary (061): `bench/speed.py specs SIM A B`, `bench/per_decision.py
  SIM.exe --old-agent A --agent B`, and `bench/first_divergence.py --cells DIR A B` for where two
  cells of one run first play differently.
- `bench/first_divergence.py OLD NEW` (046): where two runs of one spec first play differently,
  game by game.
- `tune/` (007-008): `sweep.py`, `log_table.py`, `engine_sweep.py`.

Deleted: `scripts/board/` and `scripts/alignment/` (048 #108; `--preset 016`, `018` cover
what survives, and git has them for the results docs that cite them). See
[experiments/history.md](../experiments/history.md).
