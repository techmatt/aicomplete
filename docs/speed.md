# Speed

What a run costs and where it goes. Current rates are in [experiments.md](experiments.md); the bench scripts are in [scripts.md](scripts.md).

**Where the time goes (profiled at r40).** The agents are 99% of a run. Every agent is one machine: the
strategist scores each legal action by applying it to a copy and resolving the action's own prompts
best-first (`search::settled`, 63-66% of its time), and that splits about evenly between the rules
(`Rules::apply_to`, 43%) and the heuristic (`Eval::value`, 40%). The planner's rollouts (62% of it) run the
same search, so a speed-up to one serves all. Per 16 strategist games: 215k applies, 172k evaluations, 41.5M
lookups by name. The profiles, with call counts, are in
[103's write-up](../experiments/results/103_agent_profiling/profile.md). Legal-action generation is 0.1%:
caching it buys nothing.

**What keeps it cheap.**

- **State:** a clone is about 1.1 µs: names are interned `Key`s, maps are sorted `FlatMap`s, and hexes,
  piles, RNG streams and per-round counters are shared until written (`crates/aic-core/src/key.rs`, *No
  hash-order leaks* in [architecture.md](architecture.md)). Searches call `Rules::apply_to` (one copy, not
  two). `aic-sim` uses **mimalloc** (on 12 threads Windows' heap serializes the clones).
- **Lookups by name:** a `FlatMap` compares names eight bytes at a time (`Probe`: `str`'s own compare calls
  `memcmp`, a tenth of the strategist's time before). A card is found by its key's address
  (`Content::card_index`), names are built on the stack with digits written in place (`KeyBuf`), and a
  counter's or pile's name is interned only when it is new (`entry_str`, `insert_str`). `ai.boards` is read
  once with the rules (`World::ai_boards`).
- **The heuristic:** a seat's tags, prices and income are kept a decision long, by everything they read
  (`tableau_parts` in `eval.rs`; a debug build checks each hit against a fresh reading). Anything that makes
  those four read more must add it there. Pricing a hand goes through `cards::prices`, anything counting
  several tags through `cards::tag_counts`.
- **Labels:** an option's label must stay a `String` (a replayed prompt matches its answers by label), but
  a hex's is joined by hand, without the formatter.
- **Loss prompts:** chains of them ("destroy which of your nodes", in `board.produce`, `board.bases`,
  `adversary.build`) replay the phase for each option over every order of one set of picks. `search.rs`'s
  `Seen` values each position once per scoring, among Loss answers only, confirmed field by field, with the
  same answers as a set (a loop's progress lives only in them). It costs under 1%; on every prompt it costs
  more than it saves.

**What is left, by payoff** (shares of the strategist after 103):

1. **The binary search by name**, 15.6% self, spread over `var` (3.3%), `pile` (2.5%) and the rules' own
   reads. Keys carrying their first eight bytes were tried and gained nothing (the bigger entries cost the
   saving back). Next: typed slots for the hottest vars, a state-layout change.
2. **Replaying a prompt's phase for each option** under `apply_to`: the rules' work itself (alignment's
   after-trigger and the effects it runs, 7-8%). Resuming from the resolved part instead of replaying is a
   redesign of how prompts resolve.
3. **State copies and drops**, about 8%: one copy an apply.
4. **The heuristic's own loops**, `Eval::value` 5.8% self; an incremental evaluation is a redesign.
5. **Parameters by name** (`World::int`, 3%): typed per-module parameters.
6. **Formatting**, 3% (prompt texts and counter names built with `format!` in the rules).

**Approximations** (screened on fresh seeds, 103): none adopted. An answer to an open prompt looking one
prompt on (`engine:answers=1`) is 0.72 × the time and lost 9.0 ±3.8 (bp, d0); keeping a seat's terms by its
tableau alone (missing the income vars) passed card ranks but lost more than 1 SE in two of six pairs, and its
exact form above gives the same time. Rows in [103's write-up](../experiments/results/103_agent_profiling/speed.md).

**Profiling.** Every run prints the split (`time: ...` on stderr, and in the report's *Timing*); `cargo
build --release -p aic-sim --features profile` adds the search's own split (apply, clone, legal, heuristic,
determinization, planner lines) with call counts, and allocations a game, from `experiment`. For where
inside them, `scripts/bench/profile_win.py` samples by function with stacks: `samply` needs an elevated
session here (it raises a UAC prompt and waits), so it uses Visual Studio's collector, which needs no
admin. Build with symbols in its own target dir (`CARGO_PROFILE_RELEASE_DEBUG=true
CARGO_TARGET_DIR=out/target-prof`), keep a run under 5 s (a session's trace holds about 5.6 s) or read a
planner game in windows (`--attach`), and run nothing else named `aic-sim.exe` beside it. Call counts of a
helper: a copy of the tree in `out/` with a counter in it. Timers in the core break its no-clock rule.
Count work by prompt before optimizing a part: a tally of scorings by prompt text finds where the cost is,
which passes made by guess miss. For a long screen, `cargo build --profile fast -p aic-sim` (`target/fast/`)
is 8% faster than release and takes minutes to build.

**Comparing.** Two binaries: `speed.py pair` or `per_decision.py OLD NEW`, never two `table` runs (a test
suite elsewhere on the box halved a whole table). `python scripts/bench/same_results.py DIR_A DIR_B` compares
two experiment outputs, timing aside: the check that a speed-up or a late fix changed no game
(`experiments/same-103.json`: strategist, engine, planner and `planner:strategy=1` on the three pairs, 192
games, about 2 min on 6 threads). `same_play.py` is the wider one.

**`sweep`, the cheap profile.** `planner:deals=1,lead=0` under the name `sweep`, for price screens. Keep and
play rates by card hold under it (rank correlation 0.985-0.991 against the planner's;
`bench/card_ranks.py`). Win deltas by card are noise at 96 seeds under either, so a price verdict still takes
a paired overlay on the planner. Not for a gate: wins move.

**Rollout levers.** None adopted: `answers=1` and `horizon=1` are faster and lose wins, `powers=2` alone
saves nothing (rows in `experiments/results/screen-069-speed/speed.md`, on an older revision).
`planner:budget=4` runs faster and wins far less at 2 and 3 countries; `prune=3` and `alts=1` saved no time
(an older planner; a change to what agents know or to `deals` reopens it). **`prune` is strength, not only
speed:** a dropped action stays out for the whole phase, and at `prune=6` the line alone
(`planner:alts=0`, the check that the rollout policy is still the engine) won 29% at c3 against 81% at 12
and the engine's 78, for about 17% more time. The default `deals=4,lead=0` costs 1.57 × (c2) and 1.54 × (c3)
an action against one guess (`deals=1,lead=0`), and `deals=4,lead=60` about 1.25-1.28 × a game.

**The agent tiers at r40.** One thread, a quiet box, c2 (Builder + Protector), d2, from seed 1000, the best
of two passes of `python scripts/bench/per_decision.py OLD NEW --agent A` (16 games for the greedy rows, 8
for the planners). Before is the build 103 started from; the games are the same games, so the ratio is the
code:

| agent | ms / decision | ms / game | before (ms / game) | after / before | rounds | decisions / game |
|---|---|---|---|---|---|---|
| engine (greedy, no strategy) | 0.78 | 369 | 485 | 0.76 | 7.31 | 474 |
| strategist (greedy with strategy) | 0.71 | 299 | 395 | 0.76 | 6.75 | 424 |
| sweep (`planner:deals=1,lead=0`) | 7.5 | 3109 | 4111 | 0.76 | 7.12 | 413 |
| planner (4 deals) | 11.9 | 5058 | 6688 | 0.76 | 7.00 | 424 |
| planner with strategy rollouts (`planner:strategy=1`) | 14.2 | 5710 | 7583 | 0.75 | 6.75 | 401 |

The strategist is about 17 × faster a game than the planner, and 10 × faster than `sweep`. Under load, 6
threads beside another item, the read of `experiments/screen-082.json` (1,152 games: four greedy agents and
four planners) took 17 min at r34.
