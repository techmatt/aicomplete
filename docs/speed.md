# Speed

What a run costs and where it goes. Current rates: [experiments.md](experiments.md); bench scripts: [scripts.md](scripts.md).

**Where the time goes (profiled at r112, 251).** The agents are 97% of a run (legal moves 1.6%, the game's own
applies 0.7%, the replay check 0.5%, writing lines under 0.1%). Every agent is one machine: the
strategist scores each legal action by applying it to a copy and resolving the action's own prompts best-first
(`search::settled`, 79% of its time); the heuristic (`Eval::value`, 50%, 28 µs a call) outweighs the rules
(`Rules::apply_to`, 33%, 15 µs), and the binary search by name is 16% self. The real oracle (249's `worlds`) is
the judge's games over again: its lines are 99.5% of its time (one line about a judge game), its own sampling and
search under 0.2%, so what makes the judge cheaper makes it cheaper in proportion. A judge game allocates about 6 M
times (2.6 GB); a real-oracle game 417 M (176 GB). Shares, hot spots and output: [251's write-up](../experiments/results/251_profiling/README.md). A standard read (`read_specs.py read`: three sets from 36800, the three pairs, d0-d3, 1,728 games; 6
threads, a quiet box) takes 276 s at 2.00 ms a decision; greedy on set a (576 games) 99 s at 2.04.
No prompt is over 4% of a game; legal-action generation is about 1%. **A read's wall time is its slowest games**
(at most 4.9 s now): read `games.csv`'s `ms` column for the tail before
profiling the mean (`experiments/results/174_speed/tail174.py`).

**What a rule costs the agents.** The strategist resolves every prompt of every legal action it scores, so a free
power usable at every decision with N picks (pick a card, or stop) costs about N² evaluations a decision: a
Synthesis discarding any number of cards doubled a Researcher pair's agent time; one card a use cost 1.3×. Prefer
one pick a use. A chain of prompts answered one by one ("discard which card (k of n)") is worse: at `choices` 2
each answer scored every set of three cards of the hand, H³/6 positions, each replaying the phase from its
snapshot (at r73, 50-95% of a game; 5 s a decision in long games).

**What keeps it cheap.**

- **State:** a clone is about 1.1 µs: names are interned `Key`s, maps are sorted `FlatMap`s, and hexes, piles, RNG
  streams, per-round counters and the table of hexes are shared until written (`crates/aic-core/src/key.rs`, *No
  hash-order leaks* in [architecture.md](architecture.md)): an action moving no token copies none of it. Searches
  call `Rules::apply_to` (one copy, not two). `aic-sim` uses **mimalloc** (Windows' heap serializes the clones on
  12 threads).
- **Lookups by name:** a `FlatMap` compares names eight bytes at a time (`Probe`; `str`'s compare calls `memcmp`).
  A card is found by its key's address (`Content::card_index`, open-addressed, hashed by a multiply, no
  `HashMap`); names, counters named by a card or side included, are built on the stack, digits written in place
  (`KeyBuf`), and interned only when new (`entry_str`, `insert_str`). `ai.boards` and the int parameters read after
  every action (`alignment`'s tip sizes and loss rules, `board`'s stage gates) are read once with the rules
  (`World::ai_boards`, `World::hot`). The heuristic's per-card facts are a `Vec` by card index, read by reference.
- **A rule's legality test inside a prompt:** Scrap II offers only the sacrifices that leave a free building,
  tested for every source on every replay, so it reads the seat's tags and prices once a hand (`FreeTest` in
  `cards/play.rs`), stops at the first building, tests each source once, and removes the source's cards from the
  state and puts them back, no copy (`ai::builds_after`); naive, it took 70% of a long game. A check like it in a
  new rule gets the same treatment.
- **The heuristic:** a seat's tags, prices and income are kept a decision long, by everything they read
  (`tableau_parts` in `eval.rs`; a debug build checks each hit against a fresh reading): anything making those four
  read more adds it there. A hand is priced through `cards::prices`, several tags counted through `cards::tag_counts`.
- **Labels and prompts:** an option's label stays a `String` (a replayed prompt matches answers by label), but a
  hex's is joined by hand, without the formatter. A prompt's text is built only when really asked (`Cx::choose`
  takes a closure, `PromptText`): a replay never reads it.
- **Loss prompts** ("destroy which of your nodes", in `board.produce`, `board.bases`, `adversary.build`): a chain
  replays the phase for each option over every order of one set of picks. `search.rs`'s `Seen` values each position
  once per scoring, among Loss answers only, confirmed field by field, the same answers as a set (a loop's progress
  lives only in them): under 1%; on every prompt it would cost more than it saves.
- **Discard chains** (`cards::DISCARD_PROMPT`; hands of 16-37 cards, every order scored): `Seen` takes them too, a
  position matching but for a discard pile's order (only a reshuffle reads it, on the `cards` stream), and
  `Seen::chains` values a chain's cards as a set before any replay; both only where no step under the stored value
  moved the `cards` stream, so the same scores. `discards` 0 skips resolving the next card (*Approximations*);
  `Cx::choose_key` replays a card answer without building the hand's labels.
- **The board read:** `Eval::value` reads each hex's tokens once a position, and positions sharing the table of
  hexes (options that move no token) share that read (`TokenKeys::read_shared`, 251); counts of a seat's nodes
  come off it.
- **Threats in the heuristic:** each active threat's need and share read every seat's standing counters once a
  position (`threats::standing_counters`, `need_and_share_with`; `CounterSteps` once an `Eval`). The troop march
  tests a hex for a target without building its target list (`troops::target_here`).

**What is left, by payoff** (strategist shares at r112 after 251, `profile_win.py`, 5 games from seed 36802; the
real oracle alike; gains are estimates):

1. **The binary search by name**, 16% self: `var` 2%, the node tracks (`track_taken`, `track2_taken`, read by every
   tag count) 3%, `pile` 1.5%, `has_disabled`/`disabled` 1.3%, `near_mask`. Keys carrying their first eight bytes
   gained nothing. Next: typed slots for the hottest vars (tracks, stage, buckets, paid vars), a state-layout
   change: a shadow beside `vars` stays out of serde and `PartialEq`. About 8-10%.
2. **The heuristic's per-position reads**, `Eval::value` 50%: per-call vectors and folds 9% (hands, picks),
   `tag_counts` 3.7% (each seat with a building, every position; a tags-only store by tableau would halve it),
   threats' need and share 3%, `next_rally` 3%, `next_corrupt_n` 2.5% (five sites, one card), `near_seats` 2.2%.
   251 shares the board read by the hexes' `Arc`; the masks and sites read off it (`reach_masks`, `Sites`) could
   share the same key. About 10%.
3. **Replaying a prompt's phase for each option**: `Rules::apply_to` 33%. Resuming from the resolved part would
   redesign how prompts resolve. Large, unestimated.
4. **Copies, allocations and drops**, about 15% (the profile build's throwaway copy beside each apply included):
   memcpy 7%, mimalloc 6%, drops 5%.
5. **The real oracle's wall time**: lines (a judge game each) × `worlds` × candidates, linear in `budget` (249: twice
   the worlds and lines, about twice the time); its dials are its spec, Matt's to set.
6. **Counters** (`Cx::tally` 1.8%, redone on every replay) and **parameters by name** (`World::int` 1.3%).

**Approximations.** Adopted at r73: **`discards` 0** (greedy, engine, planner, oracle): a discard prompt's answer
is valued by the table it leaves, not by the best next card. On fresh seeds every tier moved within about one SE;
strategist 5.8×, greedy 5.9× faster a decision. Not adopted: `discards` 1 (3.7×; d0 −2.8 ±1.1); answering any
open prompt looking one prompt on (`engine:answers=1`: 0.72 × the time, lost 9.0 ±3.8, bp, d0); a seat's terms kept
by its tableau alone (lost more than 1 SE in two of six pairs). Rows and profiles in
[174's](../experiments/results/174_speed/README.md) and [103's](../experiments/results/103_agent_profiling/speed.md)
write-ups.

**Profiling.** Every run prints the split (`time: ...` on stderr and in the report's *Timing*) and, from
`experiment`, `stream:` (time turning games into lines and writing them, MB, KB a game); a release build
with `--features profile` adds (its `clone` is a throwaway copy beside each apply, so its clones and drops
read high), from `experiment`, the search's own split (apply, clone, legal, heuristic,
determinization, planner lines) with call counts, allocations a game, and the costliest prompts (`prompt N% ...`:
decisions, applies and evals by prompt text, digits as `#`). For where inside them, `scripts/bench/profile_win.py`
samples by function with stacks through Visual Studio's collector (no admin, unlike `samply`). Build with symbols
in its own target dir (`CARGO_PROFILE_RELEASE_DEBUG=true CARGO_TARGET_DIR=out/target-prof`); keep a run under 5 s
(a trace holds about 5.6 s) or read a planner game in windows (`--attach`). It finds its process by image name:
beside other `aic-sim.exe` runs, profile a renamed copy. An old binary copied out of its target dir needs its
`aic_sim.pdb` beside it for symbols. A helper's call counts: a tree copy in `out/` with a counter (timers in the
core break its no-clock rule). Tally scorings by prompt text before optimizing a part: it finds what guesses miss.
For a long screen, `cargo build --profile fast -p aic-sim` (`target/fast/`) is 8% faster than release and takes
minutes to build.

**Comparing.** Two binaries: `speed.py pair` or `per_decision.py OLD NEW`, never two `table` runs (load can halve a
table). A `pair` of two rows (`--only engine --only greedy`, 1- and 8-thread columns) took 15+ minutes (240). `bench/same_results.py` (timing aside) checks that a
speed-up or late fix changed no game, on `experiments/same.json` (strategist, engine, planner and
`planner:strategy=1` on the three pairs, 192 games; cost: *The agent tiers*); `same_play.py` is wider.
A value-table change swings other games' 10-20 game ratio 0.5-2.4x: read the code's cost on the same games
(`per_decision.py OLD NEW --old-agent greedy --agent "greedy:t.chips=1.5"`, the new binary's `t.` arm replaying the
old table) and play's from `agent_ms / decisions` over hundreds of games. **Cost a decision grows with game
length**, not only options: late rounds hold more cards, pieces and resources, so each eval costs more (strategist
0.07 ms an option at 5.5-6 rounds, 0.14 at 8.5; r54 made long by a config costs as r55 does). Across a rules change
that moves length, split length from code by moving length on each binary with `--set`. **New prompts in an
automatic phase hit greedy hardest:** it settles every action that closes a phase by trying each answer to the
prompts that follow (`search::settled`, `choices` 2), so loss choices at threat execution cost it 3.0× a decision
while the strategist got cheaper. Its cost a decision also follows the games it plays: an eval change that moves
play can read 0.6-1.4× on 16 games with no code cost, so read code cost on identical games, the change switched off
on the new binary against the old. Beside a running sim, `per_decision.py` needs about 16 games: 6 read greedy
1.40-1.45× where a quiet 16-game rerun read 1.08×.

**`sweep`, the cheap profile** (agents.md), for price screens: card keep and play rates hold under it (rank
correlation 0.985-0.991 against the planner's; `bench/card_ranks.py`). Card win deltas are noise at 96 seeds under
either, so a price verdict still takes a paired overlay on the planner. Not for a gate: wins move.

**Rollout levers.** None adopted: `answers=1` and `horizon=1` are faster and lose wins, `powers=2` alone saves
nothing (an older revision: `experiments/results/screen-069-speed/speed.md`). `planner:budget=4` runs faster and
wins far less at 2 and 3 countries; `prune=3` and `alts=1` saved no time (an older planner; a change to what agents
know or to `deals` reopens it). **`prune` is strength, not only speed:** a dropped action stays out for the whole
phase; at `prune=6` the line alone (`planner:alts=0`: is the rollout policy still the engine?) won 29% at
c3 against 81% at 12 and the engine's 78, for about 17% more time. The default `deals=4,lead=0` costs 1.57 ×
(c2) and 1.54 × (c3) an action against one guess (`deals=1,lead=0`), and `deals=4,lead=60` about 1.25-1.28 × a game.

**The agent tiers at r73.** 6 threads, a quiet box; read rows over set a's 576 (greedy 96) games, d0-d3;
`same.json` rows over its 48 games a cell (d2); the oracle row over its 48 oracle games (d3 and d4, 3 threads):

| agent | agent ms / decision | ms / game | games |
|---|---|---|---|
| strategist (the judge) | 1.94 | 929 | read set a |
| greedy | 1.93 | 953 | read set a, 8 seeds |
| engine | 1.83 | 1076 | `same.json` |
| planner (4 deals) | 34.6 | 20769 | `same.json` |
| `planner:strategy=1` (strategy rollouts) | 38.6 | 23626 | `same.json` |
| oracle (its row's spec) | 139 | 67160 | oracle row |

The oracle row's wall time (96 games, 3 threads), about 20 min, is its slowest game. `same.json` takes about 6.5
min on 6 threads, mostly the planner rows; `same-noplanner.json` 44 s (r108).
