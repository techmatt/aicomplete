# 232 test cleanup: test speed (r102)

No play change. The cargo lanes only (the web checks are separate commands, not part of either lane).

## Timings

This machine, otherwise idle, test build already built, `--test-threads=6`. Before: the base commit (4850fd8)
archived to `out/` and built in its own target dir; after: this branch. Same commands both sides.

| Lane | Command | Before | After |
|---|---|---|---|
| fast | `cargo test --workspace -- --test-threads=6` | 16.4 s | 6.1 s |
| slow | `cargo test-slow --test-threads=6` | 4 min 53 s | 50 s |
| full | `cargo test-all --test-threads=6` | 5 min 37 s | 53 s |

A rebuild after an `aic-sim` edit is 7-15 s more (unchanged); a cold test build ~2.5 min.

## Per test (libtest `--report-time`, the full run, test binaries run directly)

`RUSTC_BOOTSTRAP=1 <test exe> --include-ignored --test-threads=6 -Z unstable-options --report-time` from the
crate's folder (the env var is set for the test binary only, so cargo does not rebuild).

| Test | Before | After |
|---|---|---|
| agents `rollout_replays_and_repeats` (slow) | 243 s | 24 s |
| agents `beam_replays_and_repeats` (slow) | 125 s | 10 s |
| agents `planner_replays_and_repeats` (slow) | 44 s | 24 s |
| experiment `experiment_writes_report_and_records` (slow) | 19 s | 18 s |
| agents `oracle_replays_and_repeats` (slow) | 13 s | 9 s |
| agents `oracle_replays_and_repeats_opening` (fast) | 12.3 s | 2.0 s |
| agents `engine_replays_and_repeats` (slow) | 10.7 s | 8.1 s |
| core `random_games_replay_exactly_in_every_variant_more_seeds` (slow) | 10.5 s | 5.8 s |
| core `invariants_hold_in_random_games_every_variant` (slow) | 6.6 s | 4.5 s |
| core `rush_build_always_builds_more_seeds` (slow) | 6.2 s | 4.0 s |
| agents `rollout_replays_and_repeats_opening` (fast) | 4.3 s | 2.2 s |

The rollout's whole game ran ~25 min at 230's revision; at r102 it was 243-287 s, so the slow lane had
already shrunk to ~5 min before this item (the README's 3.5 min was the older figure).

## Probes (test build)

- `opt-level = 3` for the test profile: rollout's whole game 110 s → 111 s, beam 70 → 56 s; not taken (a longer
  rebuild for little). The search agents are slow in the test build because the seat-terms cache's
  `debug_assert!(same_terms(..))` (agents/eval.rs) recomputes the terms on every cache hit: kept, it is the
  cache's correctness check.
- Rollout whole game, two plays side by side: default 110 s; `depth=30` 109 s; `branch=2` 134 s and
  `width=1,candidates=1` 116 s (both run three at once); `choices=1` 25 s.
- Beam whole game: `depth=30` 70 s; `depth=30,choices=1` 10 s; `depth=15` 8 s. Planner `budget=1` 9 s (not taken).
- Oracle opening: 15 actions 1.6 s, 20 actions 1.7 s, 25 actions 4.2 s, 30 actions 5.8 s; the game is in
  round 2 (`round` 1) from action 16.
- The greedy experiment grid's one-thread run: 15.7 s; closeness every 4 instead of every choice 11.1 s; no
  probes 8.9 s. Not taken: probing every choice is that test's point.
