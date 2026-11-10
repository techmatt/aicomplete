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
$S modules                                         # registry: modules, variants, params, phases
$S run configs/baseline.json --games 2000 --save out/games              # first 20 losses as records
$S replay out/games/*.json                         # check records natively (also browser bug reports)
$S strategies configs/baseline.json --agent strategist --games 48 --seed 30700 --set adversary.difficulty=2  # each seat's strategy at its round-1 keep, CSV seed,seat,board,strategy (--agent defaults to strategist)
$S cards                                           # every content card, CSV id,kind,tier,engine,liberate: the agents' engine and liberate tests (policy.rs) for scripts
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
  thread (an agent's `threads` dial must be 1). A read seed (12 cells) costs about 28 s natively, ~1.5× in wasm (160).
- **A farmed spec** is a file in `experiments/farm/` (`read`: the strategist, three pairs, d0-d3; `cell-d2`: one
  cell); the argument is its name. A gated spec, win-probability probes and `inject_panic` are refused: their games
  depend on more than their seed. The bundle reads no files: `configs/`, `experiments/configs/` and
  `experiments/farm/` are built into it (`crates/aic-farm/build.rs`), so a spec to farm is committed first.
- **The bundle's API** (`farm.mjs`): the default export `init()` (in Node it reads the wasm beside it),
  `fingerprint()`, `run(first, count, args)` returning the shard's text (`args`: one spec name), `engine()`.
- **The fingerprint** is the rules revision, the content hash and a hash of two strategist games (baseline, d2, seeds
  36800-36801) with no engine named: the same line on native and wasm while they play alike. About 3 s.
- **Reading a farmed read:** `screens.assemble(spec, first, count, shards, out_dir)` writes `games.jsonl` (the
  header, then the shards); `bench/paired.py` and `bench/ladder.py` read it with no `results.json`, and pair a seed
  only within one engine, counting the seeds left out for mixed engines. `scripts/farm/compare.py A B` counts games
  that differ between two engines' outputs.
- **Math:** the play path calls no float function past IEEE's exact ones (`powi(2)` in the eval), so native and wasm
  play the same games. The `libm` build flag (`cargo build --release -p aic-sim --features libm`, off by default)
  makes native call the `libm` crate, as wasm does (`src/fmath.rs`); wasm has no clock, so its timings read 0
  (`src/clock.rs`).
