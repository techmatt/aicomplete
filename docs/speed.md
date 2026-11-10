# Speed

What a run costs and where it goes. Current rates are in [experiments.md](experiments.md); the bench scripts are in [scripts.md](scripts.md).

**Where the time goes (profiled at r58, 141).** The agents are 97% of a run. Every agent is one machine: the
strategist scores each legal action by applying it to a copy and resolving the action's own prompts
best-first (`search::settled`, 72% of its time), and the heuristic (`Eval::value`, 54%) now outweighs the
rules (`Rules::apply_to`, 36%). The planner's rollouts run the same search, so a speed-up to one serves all.
The r58 standard read (`read-139-r58-{a,b}`, 432 games, 6 threads) takes 88 s at 2.14 ms a decision (141:
146 s and 3.04 before). **A read's wall time is its slowest games:** at r58 one d0 game of 11 rounds took 62
s of a 74 s half-read. Read `games.csv`'s `ms` column for the tail before profiling the mean. Profiles from
103, 104 and 141 are in [103's](../experiments/results/103_agent_profiling/profile.md),
[104's](../experiments/results/104_agent_speed_2/speed.md) and
[141's](../experiments/results/141_speed_r58/README.md) write-ups. Legal-action generation is about 1%.

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
  (`World::ai_boards`, `World::hot`). `Content::card_index` is an open-addressed table on the key's
  address with a multiply for a hash (141: the `HashMap` was a sixth of a long game), and the heuristic's
  per-card facts are a `Vec` by that index, read by reference (copying a `Fact` out was a tenth).
- **A rule's legality test inside a prompt:** Rush Build offers only the sacrifices that leave a free
  building (099). That test runs for every source on every replay of the prompt, so it reads the seat's tags
  and prices once a hand (`FreeTest` in `cards/play.rs`), stops at the first building, tests each source
  once, and removes the source's cards from the state itself and puts them back, with no copy
  (`ai::builds_after`). Before 141 it was 70% of a long game. A check like it in a new rule gets the same
  treatment.
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
- **Discard chains** ("discard which card (k of n)?", `cards::DISCARD_PROMPT`): 64-88% of a greedy game at r69
  (162), hands of 16-37 cards and every order of the same cards scored. `Seen` takes them too, a position
  matching but for a discard pile's order (only a reshuffle reads it, on the `cards` stream), and `Seen::chains`
  values a chain's cards as a set before any replay; both only where no step under the stored value moved the
  `cards` stream, so the same scores. Greedy 1.86× a game, the strategist 1.13× (one thread, quiet box, 8 games,
  `experiments/results/162_judge_purge/speed162.py`). What is left there is the replay itself (`apply_to`).

**What is left, by payoff** (shares of the strategist after 141, `profile_win.py`, 4 games from seed 36650,
d1):

1. **The heuristic's per-seat terms**, `Eval::value` 54% total: most of it is the seat terms rebuilt on every
   call (`hand_worth` 11% over hand, draft and packet; `mix_counts`, `hand_tags`, the seat-terms store's
   key). A hand's worth by its cards, resources and rounds left, kept a decision long as the tableau terms
   are, is the next exact payoff. Under it, `Worth::per_round` and `CardValue::total` are about 3%.
2. **Replaying a prompt's phase for each option**: `Rules::apply_to` is 36%, mostly the replay from the
   snapshot under `search::settled`. Resuming from the resolved part is a redesign of how prompts resolve.
3. **The binary search by name**, 18% self: `var`, `pile`, `troop_targets_on`, `threats::near::reach`,
   the node tracks. Keys carrying their first eight bytes gained nothing (103). Next: typed slots for the
   hottest vars (the tracks, the stage, the buckets), a state-layout change: a shadow kept beside `vars`
   must stay out of serde and `PartialEq`.
4. **Copies, allocations and drops**, about 18%: `memcpy` 7%, mimalloc 5.5%, drops 3.4%, clone 2.8%,
   from `Eval::value`'s per-call vectors (tokens, seats, packets, hand) and the state copies.
5. **The Rush Build test** in long games: after 141 the slowest r58 game (bp d0, seed 36589) is still 16 ms
   a decision against 2 typical. A source's test still prices and counts the hand again after each removal.
6. **Counters** (`Cx::tally` 1.1%, redone on every replay) and **parameters by name** (`World::int` 1.1%).

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
planner game in windows (`--attach`), and run nothing else named `aic-sim.exe` beside it: it finds its process by image name, so with other runs
going, profile a renamed copy (148 used `aicprof62.exe`). An old binary
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
`agent_ms / decisions` over hundreds of games. **Cost a decision grows with game length**, not only with options:
late rounds hold more cards, pieces and resources, so each option's eval costs more (136: strategist 0.07 ms an
option at 5.5-6 rounds, 0.14 at 8.5; r54 made long by a config costs as r55 does). Across a rules change that moves
length, split length from code by moving length on each binary with `--set` (136's write-up, *Speed*).
**Greedy is the agent new prompts in an automatic phase hit hardest.** It settles every action that closes a phase
by trying each answer to the prompts that follow (`search::settled`, `choices` 2), so r61's loss choices at threat
execution cost it 3.0× a decision while the strategist got cheaper (147). Its cost a decision also follows which
games it plays: an eval change that moves play can read 0.6-1.4× on 16 games with no code cost. Read the code's cost
on identical games, the change switched off on the new binary against the old one (147: 0.92×). Beside a running
sim, `per_decision.py` needs about 16 games: 6 read greedy 1.40-1.45× where a quiet 16-game rerun read 1.08× (149).

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

**Stale for greedy:** at r68 (157) the flat-weight agents (greedy, the engine on greedy's weights) cost 6-70 s a
game against the strategist's 1.5-3.6 s, 86-91% of it in `score_choose` (prompt answers): discard chains, which
162 cut (above). At r40 the strategist was
about 17 × faster a game than the planner, and 12 × faster than `sweep`. Under load, 6
threads beside another item, the read of `experiments/archive/screen-082.json` (1,152 games: four greedy agents and
four planners) took 17 min at r34.
