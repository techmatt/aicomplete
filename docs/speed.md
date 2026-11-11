# Speed

What a run costs and where it goes. Current rates are in [experiments.md](experiments.md); the bench scripts are in [scripts.md](scripts.md).

**Where the time goes (profiled at r73, 174).** The agents are 96-99% of a run. Every agent is one machine: the
strategist scores each legal action by applying it to a copy and resolving the action's own prompts best-first
(`search::settled`, 67% of its time); the heuristic (`Eval::value`, 49%) outweighs the rules (`Rules::apply_to`,
38%), and the binary search by name is 17.5% self. The planner's and oracle's lines run the same search (the oracle
at d4: `Eval::value` 50%, the troop march 13%). A standard read (`read_specs.py read`, three sets from 36800, the
three pairs, d0-d3, 1,728 games, 6 threads, a quiet box) takes 276 s at 2.00 ms a decision, greedy on set a (576
games) 99 s at 2.04 (174: 1,290 s at 9.55 and 531 s at 10.74 before). No prompt is over 4% of a game. **A read's wall time is its slowest
games:** before 174 one 12-round game took 47 s of a set's 436 s; after, the slowest is 4.9 s. Read
`games.csv`'s `ms` column for the tail before profiling the mean (`experiments/results/174_speed/tail174.py`).
Profiles: [174's write-up](../experiments/results/174_speed/README.md) and those of 103, 104, 141. Legal-action
generation is about 1%.

**What a rule costs the agents.** The strategist resolves every prompt of every legal action it scores, so a
free power usable at every decision with N picks (pick a card, or stop) costs about N² evaluations a decision:
105's first Synthesis (discard any number of cards) doubled a Researcher pair's agent time; one card a use cost
1.3×. Prefer one pick a use. A chain of prompts a player answers one by one ("discard which card (k of n)") is
worse: at `choices` 2 each answer scored every set of three cards of the hand, H³/6 positions, each replaying the
phase from its snapshot (174: 50-95% of a game at r73; Trade Blockade, Manhunt and Conscription took 5 s a
decision in long games).

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
  `cards` stream, so the same scores. Since 174 the agents' `discards` (default 0) values each answer of a
  discard prompt by the table it leaves, without resolving the next card (*Approximations*); `Cx::choose_key`
  replays a card answer without building the hand's labels.
- **Threats in the heuristic:** each active threat's need and share read every seat's standing counters; they are
  read once a position (`threats::standing_counters`, `need_and_share_with`, the config's `CounterSteps` once an
  `Eval`). The troop march tests a hex for a target without building its target list (`troops::target_here`).

**What is left, by payoff** (shares of the strategist after 174, `profile_win.py`, 4 games from seed 36800, d1;
the oracle at d4 alike):

1. **The heuristic's per-position reads**, `Eval::value` 49%: its per-call vectors (`from_iter` 10%: tokens,
   threats, hands, picks), `guide::shelter_saves` 4.9%, the threats' need and share 4.7% (the paid vars by
   name), `plan::next_rally` 4.3%, the hex tokens read 4.2%, `next_corrupt_n` 2.8%, `near_seats` 2.4%. Most
   read the board, not a seat's tableau, so the tableau store doesn't reach them: a per-decision store keyed
   by the hexes' `Arc` (shared until written) would.
2. **Replaying a prompt's phase for each option**: `Rules::apply_to` 38% (card fights and plays 11%, the
   troop march 13% of the oracle's d4). Resuming from the resolved part is a redesign of how prompts resolve.
3. **The binary search by name**, 17.5% self: `var` (the heuristic's), `pile`, `troop_targets_on`,
   `near_mask`, the node tracks. Keys carrying their first eight bytes gained nothing (103). Next: typed slots
   for the hottest vars (the tracks, the stage, the buckets, the paid vars), a state-layout change: a shadow
   kept beside `vars` must stay out of serde and `PartialEq`.
4. **Copies, allocations and drops**, about 15%: `memcpy` 6.7%, mimalloc 6%, drops 2.2%, clone 1.7%.
5. **The oracle row's d4 games** (its wall time): lines × the engine policy's cost a decision; items 1-3
   reach it. Its search dials (`alts`, `budget`, `reseeds`) are the row's spec, Matt's to set.
6. **Counters** (`Cx::tally` 1.2%, redone on every replay) and **parameters by name** (`World::int` 1.7%).

**Approximations.** Adopted (174, r73): **`discards` 0** (greedy, engine, planner, oracle): a discard prompt's
answer is valued by the table it leaves, not by the best next card (`choices` 2 had it score every set of three
cards). On fresh seeds 41701-41844 (432 strategist games a level, paired on base) every tier moved within about
one SE (strategist d0-d4: −1.4, −0.9, −2.1, −0.5, +0.2; greedy within its SE); the strategist 5.8×, greedy 5.9× a
decision. `discards` 1 (3.7×) read d0 −2.8 ±1.1: not adopted. 103's: an answer to any open prompt looking one
prompt on (`engine:answers=1`) is 0.72 × the time and lost 9.0 ±3.8 (bp, d0); keeping a seat's terms by its
tableau alone lost more than 1 SE in two of six pairs. Rows in [174's](../experiments/results/174_speed/README.md)
and [103's](../experiments/results/103_agent_profiling/speed.md) write-ups.

**Profiling.** Every run prints the split (`time: ...` on stderr, and in the report's *Timing*); `cargo
build --release -p aic-sim --features profile` adds the search's own split (apply, clone, legal, heuristic,
determinization, planner lines) with call counts, allocations a game, and the costliest prompts answered
(`prompt N% ...`: decisions, applies and evals by prompt text, digits as `#`), from `experiment`. For where
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
games; its cost: *The agent tiers*). `same_play.py` is the wider one. After a value-table change, other games swing a
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

**The agent tiers at r73 (174).** Agent ms a game, 6 threads, a quiet box; the read rows over set a's 576 (greedy
96) games, d0-d3, the `same.json` rows over its 48 games a cell (d2), the oracle row over its 48 oracle games
(d3 and d4, 3 threads); before and after 174 on the same seeds:

| agent | ms / decision before | after | ms / game after | games |
|---|---|---|---|---|
| strategist (`engine:strategy=1`, the judge) | 8.94 | 1.94 | 929 | read set a |
| greedy (flat terms) | 12.1 | 1.93 | 953 | read set a, 8 seeds |
| engine (horizon mode) | 2.43 | 1.83 | 1076 | `same.json` |
| planner (4 deals) | 54.3 | 34.6 | 20769 | `same.json` |
| planner with strategy rollouts (`planner:strategy=1`) | 55.4 | 38.6 | 23626 | `same.json` |
| oracle (the row's spec, d3 and d4) | 171 | 139 | 67160 | oracle row |

The oracle row's wall time (96 games: 48 oracle, 48 strategist, 3 threads) went 1,379 s to 1,179 s; it is its
slowest game (before: br d4 seed 36803, 1,068 s), which a change of lines moves either way. `same.json` takes about
6.5 min on 6 threads (10 min before 174), most of it the planner rows.
