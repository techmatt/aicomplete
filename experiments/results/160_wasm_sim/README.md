# 160: the sim in wasm, ready for remote workers (r69, no rules change)

The goal: a read's games farmed out to browser tabs on other machines through conductor's remote workers. This item
makes the sim fit conductor's `[remote]` contract and measures the cost. It farms nothing yet and changes no rules.

## The premise: gating only

The sim players and a read's game loop compile to `wasm32-unknown-unknown` with no redesign. What it took:

- **A library.** `aic-sim` became `src/lib.rs` (all the code) plus a `main.rs` that calls it, so the bundle crate
  (`crates/aic-farm`) links the same code.
- **The allocator.** mimalloc is a non-wasm dependency; wasm keeps Rust's default allocator.
- **The clock.** `Instant::now` panics on wasm32. `src/clock.rs` gives wasm a clock that reads 0, used by
  `experiment.rs` (a game's timings). Farm lines carry no timings anyway.
- **Threads.** The farm loop calls `play_one` directly, so it spawns no thread. An agent with `threads` > 1 (the
  oracle's dial) is refused by the farm.
- **Files.** `cells_with` takes a file reader. The bundle builds `configs/`, `experiments/configs/` and
  `experiments/farm/` in (`crates/aic-farm/build.rs`).
- **Float math.** The play path calls no float function beyond IEEE's exact ones: one `powi(2)` in the eval. Search
  hashes are lookups confirmed by equality, and shuffles go through rand's portable `gen_index`. So native and wasm
  were expected to play alike, and they do (below).

## What was built

- `aic-sim farm SPEC --seed F --games N --out FILE` (`src/farm.rs`). It plays every cell of a spec in
  `experiments/farm/` on each seed in turn, cells in grid order. Each line is `games.jsonl`'s, with every timing 0
  and `engine` set to `native` or `wasm`. Each game's time goes to stderr.
  - `--header` prints the stream header for an assembled range.
  - `--fingerprint` prints `aic-farm rules r69 content <hash> games 2 lines <fnv64>`. It plays two strategist
    games (baseline, d2, seeds 36800-36801) in about 3 s, and the engine is not part of the hash.
  - `--seed` and `--games` are required: a shard's range is never a default.
- The bundle: `node scripts/farm/build.mjs BUNDLE` writes `farm.mjs`, `aic_farm.js` and `aic_farm_bg.wasm`
  (3.3 MB). `farm.mjs` exports `init()` as its default, plus `fingerprint()`, `run(first, count, args)` and
  `engine()`. `init()` fetches the wasm in a browser and reads it from disk in Node.
- `scripts/farm/run.mjs` plays a shard in Node or in headless Edge and times each game. `scripts/farm/compare.py`
  counts the games two outputs differ on and gives win %, the paired gap and ms.
- `screens.assemble` turns header plus shards into a read directory. `paired.py` and `ladder.py` read one without
  `results.json`, pair a seed only within one engine, and count the seeds left out as mixed.
- `ladder.py` also keys each game by its agent's place in the spec. Before, a directory holding two agents kept one
  game a seed, whichever came last in the file. A single-agent read prints the same as before.
- `web/scripts/cdp.mjs` now retries `DevToolsActivePort` on EBUSY. One of six parallel browsers lost that race.
- `experiments/farm/read.json` is the standard read: strategist, three pairs, d0-d3, 12 cells a seed.
  `experiments/farm/cell-d2.json` is one cell: strategist, bp, d2.

## Checks (done-when)

| Check | Result |
|---|---|
| Native farm: two halves concatenated against one run | `read`, seeds 36800-36801, 24 games: byte-identical (`cmp`); also a fast-lane test (`halves_concatenate_to_the_whole`) |
| `same_results.py` on a frozen set, default build | `use.json`, seeds 36800-36815, engine and strategist, 3 pairs, 96 games: `same: 96 games` (base binary from ef5c1a8, new release build) |
| Bundle builds; `init`, `fingerprint`, `run` in Node; same schema | built; Node fingerprint = native fingerprint (`cae3b3f9c87a1fd8`); `run` over `read` 36800-36801: 24 lines, same keys as native, 0 games differ |
| `paired.py` mixed-engine count on a test pair | [test_pair160.py](test_pair160.py): b = seed 36800 wasm, 36801 native, against all native: `on 1 seeds (1 seeds on mixed engines left out)` per pair; `ladder.py` line `d0 3, d1 3, d2 3, d3 3` |

## Measurements (cell-d2: strategist, bp, d2; fresh seeds 39880-40071)

Ratios are against native std on the same box. Shards ran side by side, one process each: 6 at once, except the
browser round, where one of six failed to start (rerun alone, games only). The one-core rows ran one process at a
time on a quiet box, seeds 39880-39895.

| Measure | native std | native `libm` | wasm, Node | wasm, headless Edge |
|---|---|---|---|---|
| ms a game, one core, quiet (16 games) | 2798 / 2736 (start / end) | 2719 (0.98) | 4266 (1.54) | 4038 (1.46) |
| ms a game, 6 at once (192 games) | 2919 | 2805 (0.96) | 5541 (1.90) | 4740 on 160 games, 5 at once (1.56 against native's 3040 on those shards) |
| games that differ from native std (192) | - | 0 | 0 | 0 |
| games that differ from native `libm` (192) | 0 | - | 0 | 0 |
| win % (192), ± SE | 60.4 ± 3.5 | 60.4 ± 3.5 | 60.4 ± 3.5 | 60.4 ± 3.5 |
| paired gap against native std | - | +0.0 ± 0.0 | +0.0 ± 0.0 | +0.0 ± 0.0 |

- **No bias, and no noise either.** All four engines play every game the same (games 192 of 192). The engine
  pairing in the read scripts is a guard for a future divergence, not a present one.
- **`libm` costs nothing measurable:** 2-4% faster, within the run-to-run spread. Nothing in the play path calls it,
  so it changes no game. The default stays std math (Matt's ruling).
- **wasm costs about 1.5× native on one core** (Node 1.54, browser 1.46). With six at once Node falls to 1.9×,
  while native barely slows (2.77 to 2.92 s), so a tab that shares its machine pays more. Matt's toy project found
  parity. A suspect for the gap here, untested: the allocator. The search clones states by the hundred thousand,
  on mimalloc natively and on Rust's default (dlmalloc) in wasm. A tab is still a whole extra core:
  a laptop's 4 to 8 tabs at 1.5× add about 2.5 to 5 of this box's cores. That is untested.
- **LTO hurts the bundle.** `lto = "fat"` with `codegen-units = 1` gave 6311 ms a game against 4479 for the default
  release on the same 16 seeds, one core. The bundle keeps the default profile.
- **Cost of a read seed:** `read` (12 cells) runs about 28 s a seed natively, varying 15-40 s by seed (36800-36801).
  So about 40 s in a tab.

## Speed of the native build

`speed.py check` gives no verdict: its baseline is r66. `speed.py pair` compared this branch's start (5a1d65c)
with the final build, the engine and greedy at c2 and c3, 16 games. Games a minute were 1.00-1.01× old on one
thread and 0.99-1.02× on 8. The library split and the clock shim cost nothing.

## Not done

- No wasm allocator swap was tried. It is the next lever on the 1.5× (a bump or size-class allocator in
  `aic-farm` only; play can't change).
- Farm shards are single-threaded on both engines. The native command takes no `--threads`: conductor's `main_slots`
  is the parallelism.
