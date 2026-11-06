# Speed

What a run costs and where it goes. Current rates are in [experiments.md](experiments.md); the bench scripts are in [scripts.md](scripts.md).

**Where the time goes (profiled at r40).** The agents are 99% of a run. Every agent is one machine: the
strategist scores each legal action by applying it to a copy and resolving the action's own prompts
best-first (`search::settled`, 67% of its time), and that splits about evenly between the rules
(`Rules::apply_to`, 45%) and the heuristic (`Eval::value`, 39%). The planner's rollouts (62% of it) run the
same search, so a speed-up to one serves all. Per 16 strategist games: 215k applies, 172k evaluations, 41.5M
lookups by name (103). The profiles, with call counts, are in
[103's write-up](../experiments/results/103_agent_profiling/profile.md), 104's in
[its own](../experiments/results/104_agent_speed_2/speed.md). Legal-action generation is 0.1%: caching it
buys nothing.

**What a rule costs the agents.** The strategist resolves every prompt of every legal action it scores, so a
free power usable at every decision with N picks (pick a card, or stop) costs about N² evaluations a decision:
105's first Synthesis (discard any number of cards) doubled a Researcher pair's agent time; one card a use cost
1.3×. Prefer one pick a use.

**What keeps it cheap.**

- **State:** a clone is about 1.1 µs: names are interned `Key`s, maps are sorted `FlatMap`s, and hexes,
  piles, RNG streams and per-round counters are shared until written (`crates/aic-core/src/key.rs`, *No
  hash-order leaks* in [architecture.md](architecture.md)); the table of hexes is shared too (104), so an
  action that moves no token copies none of it. Searches call `Rules::apply_to` (one copy, not
  two). `aic-sim` uses **mimalloc** (on 12 threads Windows' heap serializes the clones).
- **Lookups by name:** a `FlatMap` compares names eight bytes at a time (`Probe`: `str`'s own compare calls
  `memcmp`, a tenth of the strategist's time before). A card is found by its key's address
  (`Content::card_index`), names are built on the stack with digits written in place (`KeyBuf`), and a
  counter's or pile's name is interned only when it is new (`entry_str`, `insert_str`); a counter named
  by a card or a side is built on the stack too. `ai.boards` and the int parameters read after every
  action (`alignment`'s tip sizes and loss rules, `board`'s stage gates) are read once with the rules
  (`World::ai_boards`, `World::hot`).
- **The heuristic:** a seat's tags, prices and income are kept a decision long, by everything they read
  (`tableau_parts` in `eval.rs`; a debug build checks each hit against a fresh reading). Anything that makes
  those four read more must add it there. Pricing a hand goes through `cards::prices`, anything counting
  several tags through `cards::tag_counts`.
- **Labels and prompts:** an option's label must stay a `String` (a replayed prompt matches its answers by
  label), but a hex's is joined by hand, without the formatter. A prompt's own text is built only when the
  question is really asked (`Cx::choose` takes a closure, `PromptText`): a replay never reads it.
- **Loss prompts:** chains of them ("destroy which of your nodes", in `board.produce`, `board.bases`,
  `adversary.build`) replay the phase for each option over every order of one set of picks. `search.rs`'s
  `Seen` values each position once per scoring, among Loss answers only, confirmed field by field, with the
  same answers as a set (a loop's progress lives only in them). It costs under 1%; on every prompt it costs
  more than it saves.

**What is left, by payoff** (shares of the strategist after 104, `profile_win.py`, 12 games from seed 1000):

1. **Replaying a prompt's phase for each option**: `Rules::apply_to` is 45% and most of it is the replay
   from the snapshot under `search::settled` (67%). Resuming from the resolved part instead is a redesign
   of how prompts resolve, and the largest payoff left.
2. **The binary search by name**, 17.1% self (more of a smaller whole): `var` 3.1%, `pile` 1.8%, the
   node tracks 2.0%, `alignment`'s after-trigger 0.9%, `tag_counts` 0.8%. Keys carrying their first
   eight bytes gained nothing (103). Next: typed slots for the hottest vars (the tracks, the stage, the
   buckets), a state-layout change: a shadow kept beside `vars` must stay out of serde and `PartialEq`.
3. **The heuristic's own loops**, `Eval::value` 6.5% self, its hex scan 2.1%, `Eval::fact` 1.9%; an
   incremental evaluation is a redesign.
4. **State copies and drops**, about 7% (clone 3.1%, drops 3.5%): piles, seats and counters are still
   copied a table at a time; most actions write all three, so sharing them saves less than the hexes did.
5. **Cards by address** (`Content::card_index`, 2.6%) and **counters** (`Cx::tally` 2.2%, redone on
   every replay).
6. **Parameters by name** (`World::int`, 0.7% self) and **formatting** (2.0%, mostly option labels,
   which must stay strings).

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
planner game in windows (`--attach`), and run nothing else named `aic-sim.exe` beside it. An old binary
copied out of its target dir needs its `aic_sim.pdb` beside it, or the profile has no symbols (105). Call counts of a
helper: a copy of the tree in `out/` with a counter in it. Timers in the core break its no-clock rule.
Count work by prompt before optimizing a part: a tally of scorings by prompt text finds where the cost is,
which passes made by guess miss. For a long screen, `cargo build --profile fast -p aic-sim` (`target/fast/`)
is 8% faster than release and takes minutes to build.

**Comparing.** Two binaries: `speed.py pair` or `per_decision.py OLD NEW`, never two `table` runs (a test
suite elsewhere on the box halved a whole table). `python scripts/bench/same_results.py DIR_A DIR_B` compares
two experiment outputs, timing aside: the check that a speed-up or a late fix changed no game
(`experiments/same.json`: strategist, engine, planner and `planner:strategy=1` on the three pairs, 192
games, about 2 min on 6 threads). `same_play.py` is the wider one. After a value-table change, other games swing a
10-20 game ratio 0.5-2.4x (131): read the code's cost on the same games (`per_decision.py OLD NEW --old-agent greedy
--agent "greedy:t.chips=1.5"`, the new binary's `t.` arm replaying the old table) and play's from a run's
`agent_ms / decisions` over hundreds of games.

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
for the planners). 104's before/after on the same games, and 103's table, are in
[104's write-up](../experiments/results/104_agent_speed_2/speed.md):

| agent | ms / decision | ms / game | rounds | decisions / game |
|---|---|---|---|---|
| engine (greedy, no strategy) | 0.69 | 305 | 7.12 | 445 |
| strategist (greedy with strategy) | 0.69 | 274 | 6.50 | 400 |
| sweep (`planner:deals=1,lead=0`) | 7.4 | 3214 | 7.50 | 436 |
| planner (4 deals) | 10.4 | 4596 | 7.25 | 440 |
| planner with strategy rollouts (`planner:strategy=1`) | 12.5 | 5290 | 7.25 | 424 |

The strategist is about 17 × faster a game than the planner, and 12 × faster than `sweep`. Under load, 6
threads beside another item, the read of `experiments/archive/screen-082.json` (1,152 games: four greedy agents and
four planners) took 17 min at r34.
