# 310 test cleanup: test speed (r150)

No play change. The cargo lanes only (the web checks are separate commands, not part of either lane).

## Timings

This machine (12 logical cores), otherwise idle, test build already built, below-normal priority. Before: the base
commit (1928a6b) archived to `out/310_test_cleanup/base/` and built in its own target dir; after: this branch. Same
script both sides (`out/310_test_cleanup/lanes.sh`), one run each unless noted.

| Lane | Before: command | Before | After: command | After |
|---|---|---|---|---|
| fast | `cargo test --workspace` | 6.7, 6.8 s | `cargo test-fast` | 4.5, 4.3 s |
| fast, 6 threads | `cargo test --workspace -- --test-threads=6` | 6.8 s | `cargo test-fast --test-threads=6` | 4.4 s |
| full | `cargo test-all` | 45.5 s | `cargo test-all` | 30.2 s |
| full, 6 threads | `cargo test-all --test-threads=6` | 46.1 s | `cargo test-all --test-threads=6` | 30.3 s |
| slow | `cargo test-slow` | 42.7 s | `cargo test-slow` | 29.2 s |

The steps one at a time (full run / fast lane): binaries side by side 37.7 / 4.6 s (with the two threaded tests and
line tables); dependencies at `opt-level` 3 34.1 / 4.4 s; the priority hint 30.2 / 4.4 s.

`cargo test --workspace` (cargo's own runner, a binary at a time) on this branch: 6.0-6.2 s.

The edit loop, one edit then the fast lane (a string in a hot function's body, toggled; two samples for aic-sim):

| Edit | Before (`cargo test --workspace`) | After (`cargo test-fast`) |
|---|---|---|
| `aic-sim/src/agents/eval.rs` | 17.4, 17.3 s | 11.9, 11.9 s |
| `aic-core/src/rules.rs` | 20.0 s | 13.9 s |

Cold test build: 2 min 12 s before, ~2.5 min after (dependencies at `opt-level` 3 cost ~40 s once; line tables
saved ~30 s of it).

The browser check, outside both lanes (staging's default: it stays outside), once alone against the dev server on
the item's port: `AIC_URL=http://localhost:62634/ npm run e2e`, 30.3 s and 31.1 s (web/README said ~15 s; now ~30 s).

## Where the time went (before)

Per binary and per test, libtest `--report-time` (`RUSTC_BOOTSTRAP=1` on the test binary only), run directly:
`out/310_test_cleanup/profile.py`.

| Binary | Fast: wall / sum of tests / slowest | Full: wall / sum / slowest |
|---|---|---|
| aic-sim lib | 3.1 / 11.3 / 2.3 s | 26.1 / 57.8 / 23.9 s (`rollout_replays_and_repeats`) |
| experiment | 1.6 / 6.0 / 1.6 s | 8.5 / 17.9 / 7.9 s |
| aic-core lib | 1.0 / 4.6 / 0.9 s | 7.3 / 26.2 / 6.1 s |
| python_read_tools | (slow) | 2.2 s |
| the other 9 | 1.1 s in all | 1.1 s in all |

Cargo runs test binaries one after another, so a lane took the sum of each binary's wall time: 6.75 s fast, 45 s full,
against a longest binary of 3.1 s and 26 s. No sleeps or polls of note (one 100 ms poll in a slow-lane kill test);
collection and process start are ~0.01 s a binary; the CLI tests' subprocesses are the work itself.

The edit loop (cargo `--timings`): after an aic-sim edit the test build waits on the plain `aic-sim` rlib (8.7 s
for the eval.rs edit, the crate's unit-test build 7.1 s beside it), then the binaries that link it (~1 s, in
parallel). After an aic-core edit it waits on aic-core's unit-test build (12 s).

## What changed

- **aic-lanes runs the test binaries side by side** (`crates/aic-lanes/src/main.rs`): `cargo test --workspace --no-run
  --message-format=json-render-diagnostics [cargo flags]`, the test executables read from cargo's JSON (no
  dependencies: a small string reader, unit-tested on Windows-shaped lines), each run from its package folder with
  the lane's flag and the test args, output printed whole per binary, then a summary line. New alias `cargo test-fast`
  (no lane flag); `test-all` and `test-slow` use the same path. Fast lane -1.5 to -2 s, full run -8 s. With every
  binary at once the 12 cores saturate: each binary runs 2-4x slower than alone, so the lane is bound by total CPU
  and the longest test rather than the sum.
- **Test profile `debug = "line-tables-only"`** (`Cargo.toml`): panics and backtraces keep file:line, a debugger loses
  locals. Test build after an aic-sim edit 10.4 → 8.0 s, after an aic-core edit 13.2 → 10.6 s.
- **Test profile, third-party dependencies at `opt-level` 3** (`[profile.test.package."*"]`): the workspace's own
  crates stay at 1, so edit-loop compiles are unchanged. Full run 37.7 → 34.1 s, fast 4.6 → 4.4 s (the RNG,
  allocator and serde are in the hot paths).
- **A priority hint** (leftover budget): aic-lanes keeps each binary's wall time from the last unfiltered run of each
  lane (`target/debug/aic-lanes-times.txt`) and starts the slowest one at its own priority class, the rest one class
  lower (`GetPriorityClass` / `creation_flags`; Windows only). The rollout game then keeps its cores while the rest
  fill the others: full run 34.1 → 30.2 s (two samples each), fast lane unchanged. The first run, with no file, runs
  every binary at the same class. It decides only who waits for a core, never what runs.
- **Two fast-lane tests threaded** (`agents/mod.rs`): `the_real_oracle_ignores_hidden_information` runs its two
  decisions side by side (2.3 s → ~1.2 s), `decisions_ignore_hidden_information` an agent a thread (1.3 s in turn).
  Same assertions.
- Nothing moved between lanes, no test deleted, no assertion changed.

Not taken:
- **One integration test binary** (staging's question): after an aic-sim edit the five integration binaries link in
  parallel in ~1 s, behind the rlib; merging them saves under 1 s, short of the 3 s bar.
- **The seat-terms cache check** (232 named it as why search agents are slow in the test build): disabling it left
  the rollout and planner whole games at 23 s either way. It isn't the cost now.

## Given up

Nothing in the tests. The test build's debugger view: locals and types are gone from the test binaries' debug info
(`debug = "line-tables-only"`); a debugger session wants `CARGO_PROFILE_TEST_DEBUG=2` for that run. `cargo test-all`'s
output is grouped by binary, printed as each ends, rather than streamed test by test.

## Left

The slowest tests now (full run, alone / in the side-by-side run):

| Test | Alone | What would speed it |
|---|---|---|
| aic-sim `rollout_replays_and_repeats` (slow) | 24 s / ~29 s | A whole game is serial; its two plays already run side by side. Cheaper params lose the depth it covers (232's probes). The full run waits on it. |
| aic-sim `planner_replays_and_repeats` (slow) | 8.4 s | `budget=1` (232: 9 s at r102), at some coverage. |
| experiment `experiment_writes_report_and_records` (slow) | 7.9 s | Probing every choice is its point (232). |
| aic-core `rush_build_always_builds_more_seeds`, `..._more_seeds` replay, `invariants_..._every_variant` (slow) | 5-6 s each | Split by variant into tests (they loop). |
| aic-sim `rollout_replays_and_repeats_opening` (fast) | 1.7 s | Already two plays side by side. |
| experiment `experiment_reports_comparisons` (fast) | 1.6 s | One greedy game is its floor. |

The rest of the edit loop is codegen of the edited crate, twice (its rlib and its unit-test build, side by side):
6.7 s and 5.5 s after the eval.rs edit with line tables. An edit at the crate root (`lib.rs`) showed the rlib at 8.8 s
against 2.2 s for the unit-test build: worth a look at what the rlib codegens that the test build doesn't.
