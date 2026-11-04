# 103: where the strategist's and the planner's time goes (r40)

## How it was read

`samply` needs an elevated session on Windows (it raised a UAC prompt and waited), so the
profiles come from Visual Studio's Standard Collector, which samples for any user, read with
`xperf -symbols`: `scripts/bench/profile_win.py` (docs/scripts.md). One thread, c2, bp, d2, a
release build with debug symbols in its own target dir (`CARGO_PROFILE_RELEASE_DEBUG=true`; debug
info changes no code), 1 kHz.

- **Strategist:** 12 runs of 4 games, seeds 1000-1047: 20,860 samples, all with stacks.
- **Planner** (`planner`, 4 deals): a session holds about 5.6 s of trace, so 12 runs of 3 games
  from seeds 1000 + 3i, each read in a 5 s window from 0, 5, 10, 15, 20 or 25 s in:
  67,510 samples, 97% with stacks.
- **Call counts** don't come from sampling: a copy of the start-of-item tree with a counter on
  each helper below (`out/`, never committed), 16 strategist games and 4 planner games, seeds from
  1000. Apply and eval counts are `--features profile`'s.

Inlined functions count toward their caller. "Total" is any frame on the stack.

## Before: the start of the item

| function | what | strategist total | planner total | calls a game, strategist | planner |
|---|---|---|---|---|---|
| `search::settled` | an action's own prompts resolved, best first | 65.8% | 72.9% | | |
| `Planner::line_from` | the planner's rollouts | | 61.6% | | |
| `Rules::apply_to` | one action on a copy | 42.4% | 46.7% | 13.4k | 216k |
| `Eval::value` | the heuristic | 43.7% | 41.0% | 10.7k | 170k |
| `FlatMap::get` | a state lookup by name: vars, piles, counters | 19.0% | 17.8% | 2.59M | 47.4M |
| `vcruntime140` | the C runtime: `memcmp` (half of it under `FlatMap::get`), `memcpy` | 22.2% | 21.8% | | |
| `format_inner` | `format!`: labels and keys | 6.3% | 7.7% | | |
| `economy::income_with` | a seat's income | 4.8% | 4.6% | 22.7k | 381k |
| `tableau::tag_counts` | a seat's tags | 4.5% | 4.1% | 34.9k | 567k |
| `GameState` clone | a copy for each apply | 4.5% | 4.6% | 13.4k | 216k |
| `mi_malloc` | allocation | 4.6% | 6.0% | | |
| `World::list` | a list parameter by name | 3.0% | 2.9% | 712k | 12.5M |
| `World::int` | an int parameter by name | 2.7% | 2.4% | 439k | 7.4M |
| `Content::card` | a card by id | 2.7% | 2.9% | 629k | 11.7M |
| `common::hex_name` | a hex's label | 2.7% | 3.2% | 121k | 2.33M |
| `sacrifice::sources` | what a seat could sacrifice | 2.3% | 2.5% | 5.0k | 69k |
| `board::flagged` | a country flag (key by `format!`) | 1.9% | 2.1% | 79k | 1.84M |
| `KeyBuf::num` | a number in a built name | (self 1.9%) | (self 1.6%) | 1.10M | 18.1M |
| `Key::new` | interning a name | (self 1.8%) | (self 1.8%) | 256k | 5.95M |

The planner is the strategist's machinery run many times: 62% of it is rollouts, scored by the
same greedy search, so every fix below serves both. Its own code (`play_on`, `other_deals`,
determinization at 1.9%) costs little beside them.

## Against the known leftovers (docs/speed.md before 103)

- **Option labels rebuilt as strings:** real, 6-8% (`format_inner`, mostly `hex_name` and the
  flag keys), more than the 4 µs of 50 guessed. The labels themselves have to stay: a replayed
  prompt matches its answers by label. What was cut is how they are built.
- **`tag_counts` walking the hand:** 4.5%, but the walk was cheap; its cost was the pile and card
  lookups by name under it, and the evaluator asking it again for every position. Both cut.
- **The Loss-prompt cache:** under 1% (its table's drop is 0.4%; the hash and the field-by-field
  check don't show). Not a cost.
- **Not on the list:** looking things up by name was the largest single cost: `FlatMap`'s binary
  search through `memcmp`, parameters and cards by hashed string, about 30% together.

## After (the class-1 batches and the seat-terms cache, before `KeyBuf::num`)

16,163 samples, the same runs. The binary search is now the top self cost (15.6%, inlined into
its callers: `var` 3.3%, `pile` 2.5%); the C runtime is 8.9% (from 22.2%); `income_with` 0.8%
(from 4.8%), `World::list` 0.4% (from 3.0%), formatting 3.1% (from 6.3%). What is left is spread:
the rules' own work under `apply_to` (alignment's after-trigger and the effects it runs, 7-8%),
the heuristic's own loops, state copies and drops (about 8%), `World::int` (3.0%).
