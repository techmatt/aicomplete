# Speed

What a run costs and where it goes. Current rates are in [experiments.md](experiments.md); the bench scripts are in [scripts.md](scripts.md).

**Speed.** The agents are 99% of a run, and since 030 nearly all of that is the
heuristic and the rules, not state copies: a clone is 1.8 µs (it was 21), because names
are interned `Key`s, maps are sorted `FlatMap`s and hexes, piles, RNG streams and
per-round counters are shared until written (`crates/aic-core/src/key.rs`, *No hash-order
leaks* in [architecture.md](architecture.md)). `aic-sim` uses **mimalloc** (on 12 threads Windows'
heap serialized the clones), and searches call `Rules::apply_to` (one state copy, not
two). Legal-action generation is 0.1% of a run: caching it buys nothing. For what 12
threads are worth here, see *This machine* in the [README](../README.md).

**Where a planner game goes (069, r28).** Prompts: chains of Loss prompts ("destroy which of your presence", in `board.produce`, `board.bases`, `adversary.build`) were a few thousand scorings a batch of games and 46% of the time scored, each option replaying the phase over 8 × 7 × 6 orders of one set of picks. `search.rs`'s `Seen` values each position once per scoring, among Loss answers only, confirmed field by field, with the same answers as a set (a loop's progress lives only in them: a table alone matched a Shelter-absorbed hit, 7% of the gain went on checking that). On every prompt it lost (26 games a minute at c2 against 32). Menus have no such shortcut; a prompt's own options rarely repeat a parent's work, because auto-phase prompts draw on the guess (a reuse of the parent's resolved tree saved nothing). Next up, unclaimed: option labels are `String`s rebuilt on every replay (33 `hex_opt`-like sites), about 200 allocations an `apply` and `eval`, perhaps 4 µs of 50.

To see the split: every run prints it (`time: ...` on stderr, and in the report's
*Timing*); `cargo build --release -p aic-sim --features profile` adds the search's own
split (apply, clone, legal, heuristic, determinization, planner lines) with call counts,
and allocations a game, from `experiment`. For a long screen, `cargo build --profile
fast -p aic-sim` (`target/fast/`) is 8% faster than release and takes minutes to build.
There is no sampling profiler here: to go finer, copy the tree and add timers to the
copy. Timers in the core break its no-clock rule. Count work by prompt before optimizing a part: 069's first two
passes took 1.65 × and a tally of scorings by prompt text then found 1.2 × more in one place.

Pricing a hand goes through `cards::prices`, and anything counting several tags through
`cards::tag_counts`: a tag count walks the seat's cards.

Compare two binaries with
`pair`, not two `table` runs: a test suite elsewhere on the box halved a whole table.

**`sweep`, the cheap profile (069).** `planner:deals=1,lead=0` under the name `sweep`, for price screens: 1.43 × (c2) and 1.53 × (c3) a game, and -5.2 ±5.0 / -3.1 ±4.3 against the planner (96 paired seeds). Keep and play rates by card hold: rank correlation 0.985-0.991 over the 145-149 cards, and across profiles on half-sets the same as the planner against itself (0.85-0.94). Win deltas by card are noise at 96 seeds under either (half-sets of one agent read about 0), so a price verdict still takes a paired overlay on the planner. Not for a gate: wins move.

**Rollout levers (069, 48 + 96 paired seeds).** None adopted, rows in `experiments/results/screen-069-speed/speed.md`: `answers=1` is 1.5-1.9 × faster and behind at c3 on both seed sets; `powers=2` alone saves nothing; `answers=1,powers=2` passed the screen and missed fresh c2 by -10.4 ±5.8; `horizon=1` is 2.0-2.4 × faster and -14.6 ±8.4 at c2.

`planner:budget=4` runs 25-29% faster and wins 19 and 28 points fewer at 2 and 3 countries;
`prune=3` and `alts=1` saved no time on 32 seeds at d2 when last measured (030, the planner of that
day); not remeasured since, and a change to what agents know or to `deals` reopens it. **`prune` is strength, not only
speed (054):** a dropped action stays out for the whole phase, and at `prune=6` the line alone
(`planner:alts=0`, the check that the rollout policy is still the engine) won 29% at c3 against 81%
at 12 and the engine's 78, for about 17% more time. The default `deals=4,lead=0` costs 1.57 × (c2) and
1.54 × (c3) an action against one guess (`deals=1,lead=0`; 064, 48 seeds, `ms_per_decision` in `results.json`;
060 read 1.51 ×), and `deals=4,lead=60` about 1.25-1.28 × a game against one guess (061, 48 seeds). `python
scripts/bench/same_results.py DIR_A DIR_B` compares two experiment output directories,
timing aside: the check that a speed-up or a late fix changed no game (`games.csv` and
`results.json`). `same_play.py` is the wider one.

**The agent tiers at r34 (082).** One thread, a quiet box, c2 (Builder + Protector), d2, from seed 1000, the best of two
passes of `python scripts/bench/per_decision.py SIM.exe --old-agent A --agent B` (16 games for the greedy rows, 8 for the
planners; the same seeds within a row pair, so a row's games are the same games):

| agent | ms / decision | ms / game | rounds | decisions / game |
|---|---|---|---|---|
| engine before 082 (`engine:plan_table=0,w.table=0,w.packet_keep=1`) | 1.2 | 551 | 7.62 | 463 |
| engine (greedy, no strategy, on the value table) | 1.4 | 703 | 7.62 | 510 |
| strategist (greedy with strategy) | 1.8 | 968 | 7.94 | 539 |
| sweep before 082 (`sweep:plan_table=0,w.table=0,w.packet_keep=1`) | 14.5-15.1 | 6987-7270 | 8.12 | 482 |
| sweep (on the table) | 11.8 | 4763 | 6.62 | 403 |
| planner with strategy rollouts (`planner:strategy=1`, 4 deals) | 19.4 | 9706 | 7.62 | 500 |

The table costs the engine 1.16 × a decision (14.9 µs a heuristic call against 13.5, `--features profile`); the
strategist's games are longer and its hands bigger (1.31 × the engine's decision). The strategist is about 11 × faster a
game than the sweep before 082 (613 against 6987 ms on 8 shared games). Under load, 6 threads beside another item, the read
of `experiments/screen-082.json` (1,152 games: four greedy agents and four planners) took 17 min.
