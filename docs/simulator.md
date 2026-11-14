# Simulator

The `aic-sim` command line: `run`, `compare`, `show`, `modules`, `replay`, `strategies`, `cards`, `farm` (below);
`experiment` is in [experiments.md](experiments.md), `docs [--check]` in architecture.md *Generated docs*.

Always run the sim in release.

```sh
S=target/release/aic-sim
$S run configs/baseline.json --games 2000                          # 2 countries, difficulty 0
for n in 1 2 3 4; do $S run configs/baseline.json --games 500 --set countries=$n; done
$S compare configs/baseline.json configs/baseline.json --set-b threats.variant=off  # off-variants
$S run configs/baseline.json --set alignment.hold_off=0 --set alignment.momentum_every=2
$S compare configs/baseline.json configs/baseline.json --set-b adversary.difficulty=4
$S run configs/baseline.json --games 1 --trace     # first game as JSON lines
$S run configs/baseline.json --json                # machine-readable summary
$S show configs/baseline.json                      # resolved config, hash, every tunable
$S map configs/baseline.json                       # each country's hexes by number: kind, flags, neighbors
$S modules                                         # registry: modules, variants, params, phases
$S run configs/baseline.json --games 2000 --save out/games              # first 20 losses as records
$S replay out/games/*.json                         # check records natively (also browser bug reports)
$S strategies configs/baseline.json --agent strategist:strategy=1 --games 48 --seed 30700 --set adversary.difficulty=2  # each seat's strategy at its round-1 keep, CSV seed,seat,board,strategy (--agent defaults to strategist:strategy=1)
$S cards                                           # every content card, CSV id,kind,tier,engine,liberate: the agents' engine and liberate tests (agents/cards.rs) for scripts
```

| Option | Default | Meaning |
|---|---|---|
| `--games N` / `--seed S` | `200` / `0` | Seeds `S` to `S+N-1`, one game per seed per config (any `u64`, up to `u64::MAX`). |
| `--set K=V` | | Override for every config. Repeatable. `countries=N`; `<module>.<param>=V` with `V` as `3` or `1,2,3`, or empty to unset; `<module>.variant=NAME` (keeps the parameters the new variant shares, drops the rest); `<module>=off` (removes the module and its phases: `threats`, `cards` and `economy` can go (economy off plays with no upkeep income at all); `board`, `adversary`, `alignment` and `ai` are required by another module, so the config is rejected: use `<module>.variant=off`). |
| `--set-a K=V` / `--set-b K=V` | | Override for config A or config B only. |
| `--agent SPEC` | `random` | The team agent (see [agents.md](agents.md)), e.g. `greedy`, `beam:width=4`. Shown in the table header and in `--json`. |
| `--max-actions N` | `100000` | Per-game action cap. A capped game ends as `timeout`. |
| `--threads N` | all cores | Results don't depend on it: agents are seeded per game. |
| `--json` | off | Print the summary as JSON: every field below, plus the diff, agent, `max_actions` and rules revision. |
| `--trace` | off | Print the first game of each config: every action (with its option count) and event. Goes to stderr with `--json`. |
| `--save DIR` | off | `run`/`compare`: write the selected games as game records into `DIR` (created if needed), named `<config name>-<run\|A\|B>-seed<S>.json`, so A and B never collide. The paths are printed after the table, or listed under `saved` in each `--json` result with the match counts. A write failure exits 2. |
| `--save-when W` | `loss` | Which games to save: `all`, `win`, `loss`, `timeout`, or an end reason code (`deck_empty`, `discord_deck`, `node`, `purge`, ...). Repeatable; a game matching any is saved. With only `--save-seed` given, no default: just those seeds. A reason code no game ended with gets a warning on stderr (likely a typo). |
| `--save-seed S` | | Always save this seed, whatever `--save-when` says and not counted against `--save-max`. Repeatable. A seed outside the range played gets a warning. |
| `--save-max N` | `20` | Per config, keep the first `N` games `--save-when` matched, in seed order. The output says how many matched, so a cap never truncates silently. |

The output shows each config's hash, content hash and engine version, plus an
`A -> B` line listing every variant and parameter difference. The rows:

- `win %` (± standard error);
- rounds (mean, min..max);
- games by end reason and by end round;
- decisions per game, options per decision, forced decisions %, and max options;
- adversary ops per round (mean, and the max in any one round), and at setup;
- every module counter, averaged per game; a `*_round` counter (the round of the first corruption, of the first project) over the games that reached it ("mean where reached"), and a round's own counter (`_rNN`, `.rNN`) over the games that reached that round;
- `replay ok`;
- `wall ms/game`: the run's wall time over its games, all threads at once (so it falls with `--threads`).

**Replay check:** each game is replayed from seed + actions through `Rules` rebuilt from
the resolved config, and must equal the final state. The exit code is 1 on any
mismatch, and 2 on a bad config or bad arguments (including a seed range that
overflows `u64`).

**Game records** (`aic-game-record/1`, `aic_core::SavedGame`) are JSON: the resolved
config, the seed and the action list (one action per line), plus a `note` saying why the
game was saved (command, outcome, seed, selector, overrides) and the outcome, rounds,
final-state fingerprint, hashes, engine version and rules revision for checking the
replay. A saved record is the game the summary counted, not a second play. Only
`config`, `seed` and `actions` are required, so a hand-written record loads too. `aic-sim
replay <record.json>...` prints each record's seed, outcome and warnings (a different
rules revision, content or config hash, or final fingerprint), and exits 1 if any record
fails to load or replay or has a warning. Browser bug reports are checked the same way.

Saving every game of a big run is slow: Windows file writes are the bottleneck. A shell glob over more than a few
hundred records hits Windows' command-line length limit ("Argument list too long"), so
replay big sets in batches.

The random agent picks uniformly among all seats' legal actions, passes and choice
answers included. Treat it as a floor, not as a model of a weak team.

## Keys in a games.jsonl line

What the read scripts quote most (`screens.cnt(game, key)`; more in experiments.md, *Counters*):

| Key | What it holds, per game |
|---|---|
| `counters['ai.power.<board>.<power>']` | a board power's uses (`modules/ai.rs`; the seats on that board summed), `.rNN` a round's; names: `scripts/powers.py` |
| `counters['board.std.<name>']` | a standard action's uses; `board.std_actions` all of them |
| `counters['fx.<card>.tN']` / `['fx.<card>.<measure>']` | an Alignment card resolved at tier N / the change it made to a `FX_MEASURES` total (`alignment.rs`): a signed delta, not a count (`fx.network_cleanup.land` −1: one Fallout token gone) |
| `sources[seat]` / `uses[seat]` | each resource gained / spent by where, `{label: [...]}`: `own:` or `other:` an action, `phase:` an automatic phase. Labels: `play:<card>`, `use:<card>` (its Action), `power:<Name>`, `standard:<act>`, `convert:<country>` (a corp converter), `phase:economy.upkeep` (income and capstones) |
| `start`, `gained`, `spent`, `peaks` | per seat, totals (`start + gained - spent` is the end) |

Each row is in `PEAKS` order (credits, insight, energy, chips, antimatter, research cubes, card tokens, ambassadors).

## The farm

A read's games, cut into seed ranges for conductor's remote workers (browser tabs on other machines, 160). The
native command and the wasm bundle (`crates/aic-farm`) run the same code (`crates/aic-sim/src/farm.rs`):

```sh
$S farm read --seed 1000 --games 4 --out out/s0.jsonl    # every cell of experiments/farm/read.json, seeds 1000..1003
$S farm read --header --seed 1000 --games 8               # the games.jsonl header for an assembled range
$S farm --fingerprint                                     # what a worker must reproduce
node scripts/farm/build.mjs out/bundle                    # the bundle: farm.mjs, aic_farm.js, aic_farm_bg.wasm
node scripts/farm/run.mjs out/bundle read 1000 2 out/w.jsonl [--browser] [--fingerprint]
```

- **A shard's output** is one line per game, seed by seed and each seed's cells in grid order: `games.jsonl`'s line
  with every timing 0 and `engine` naming who played it (`native` or `wasm`). Shards concatenated in seed order are
  byte for byte one run over the whole range. Each game's time goes to stderr. The file is written whole at the
  end (a `.part` renamed), so a killed shard leaves none. `--seed` and `--games` are required. A shard is one
  thread (an agent's `threads` dial must be 1). A strategist game costs about 1 s natively on one core since 174
  (178: 2,160 games in ~9 min on 4 threads), so a read seed (12 cells) about 12 s; wasm is ~1.5× native (160); the
  laptop tab is best at 2-3 cores (163, B5). A job of few shards is bound by its slowest shard.
- **A farmed spec** is a file in `experiments/farm/` (`read`: the strategist, three pairs, d0-d3, a subset: an
  item's read writes its own d0-d4 spec; `cell-d2`: one cell); the argument is its name. A gated spec, win-probability probes and `inject_panic` are refused: their games
  depend on more than their seed. The bundle reads no files: `configs/`, `experiments/configs/` and
  `experiments/farm/` are built into it (`crates/aic-farm/build.rs`), so a spec to farm is committed first: a spec
  missing from the workers' bundle fails there and the main box reruns its shards late, one at a time (167: a
  3,000-game screen took ~60 min). A finished item's specs move to `experiments/archive/farm/` (not built in).
- **A farm job outlives its `farm-seeds`** (212): killing the client leaves the broker running the job's shards on
  the main box with `target/release/aic-sim.exe`, which holds the CPU and locks that file, so `cargo build --release`
  fails "Access is denied" until the job ends. Stop a read before you launch it, not mid-run; to build meanwhile, use
  another `CARGO_TARGET_DIR`.
- **The bundle's API** (`farm.mjs`): the default export `init()` (in Node it reads the wasm beside it),
  `fingerprint()`, `run(first, count, args)` returning the shard's text (`args`: one spec name), `engine()`.
- **The fingerprint** is the rules revision, the content hash and a hash of two strategist games (baseline, d2, seeds
  36800-36801) with no engine named: the same line on native and wasm while they play alike. About 3 s.
- **`farm-seeds`** (conductor's remote client) runs the worktree's `target/release/aic-sim.exe` on the main box and
  builds the browsers' bundle from HEAD: commit a spec and its code before farming, and rebuild into another
  `CARGO_TARGET_DIR` while a job runs (the running exe can't be replaced; rename it aside, then copy). Farm lines
  carry no timings: a main-box game's ms is in the job's `main.log` (162's `cost162.py` reads it). It first runs a
  native `cargo build --release`, which fails (`Access is denied`) while another `aic-sim` runs from that exe: run a
  local oracle row from a copy (177). The shards write under conductor's job folder; the out file appears only at
  the end.
- **Reading a farmed read:** `screens.py` and `ladder.py` read the farm's out file as `OUT=SPEC.json` (no header
  needed); for shards, `python scripts/screens.py assemble SPEC FIRST COUNT OUT_DIR SHARD...` writes
  `games.jsonl` (the header, then the shards); `bench/paired.py` and `bench/ladder.py` read it with no `results.json`, and pair a seed
  across engines, counting the seeds on mixed engines (`--same-engine` leaves them out, 163). `scripts/farm/compare.py A B` counts games
  that differ between two engines' outputs. `farm-seeds`' out file has no header line: for a reader that needs one
  (`design_check.py`, `play_read.py`), write `aic-sim farm SPEC --header --seed F --games N` first, then the out file.
  A farmed run writes no `results.json` or `cards.csv`: the card viewer's stats need a native `aic-sim experiment` run
  (213 reran the dev screen's base natively, the same games).
- **An arm with cards out of the deck** is a derived config in `experiments/configs/` (`scripts/configs/no_tier2.py`,
  `no_liberate.py`, both run by `regen.py`): the farm bundles that folder, and no `--set` removes a card from a deck.
- **A ladder against the old revision:** `farm-seeds` runs only the worktree's build, so the old side runs natively
  (`aic-sim experiment` on the farm spec with `first_seed`/`games` set) from a `git archive` export of the base commit
  built into its own `CARGO_TARGET_DIR`, with its own configs; 178's 144-seed strategist read took 303 s farmed, the
  old side about 9 min on 4 threads.
- **Math:** the play path calls no float function past IEEE's exact ones (`powi(2)` in the eval), so native and wasm
  play the same games. The `libm` build flag (`cargo build --release -p aic-sim --features libm`, off by default)
  makes native call the `libm` crate, as wasm does (`src/fmath.rs`); wasm has no clock, so its timings read 0
  (`src/clock.rs`).
