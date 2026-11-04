# 104: agent speed, round 2 (r40)

The base is 104's own first commit, `plan_gross=1` made the default (P1, Matt's amendment 1):
`same_results.py`, the `speed.py` baseline and the tier table all compare against it.

## What landed (all class 1: the same games)

| change | where | what it saves |
|---|---|---|
| The hex table shared until written (`GameState::hexes: Arc<Vec<Arc<HexState>>>`) | `state.rs`, `Cx::add_tokens` | a copy and a drop of every hex's share on each apply that moves no token; serde and `PartialEq` read the same (`Arc`'s equality checks the pointer first) |
| Prompt texts built only when asked (`Cx::choose` takes `impl PromptText`: a `&str`, a `&String` or a closure) | `module.rs`, 58 call sites | a replayed or single-option prompt never formats its text |
| Counter names built on the stack (`KeyBuf`) | `cards.played.<id>`, `alignment.*`, `board.std.*`, `adversary.project.*`, `board.node_lost.*` | a `format!` and an allocation each tally |
| Typed slots for the int parameters read after every action (`World::hot(Hot::...)`, `HotParams`) | `alignment` tip sizes, momentum, node loss, clear-win stage; `board` troop, last-safe and liberation gates | a hash and a compare by name; an absent one falls back to `World::int`, with its panic |
| The seat-terms key walks the seat's `cards.under.<s>.` piles in one range | `eval.rs` `tableau_parts` | a pile lookup by built name for each top |

`same_results.py` against the base on `experiments/same-103.json` (strategist, engine, planner,
`planner:strategy=1`, the three pairs, 16 seeds, d2): same 192 games after each batch. An independent
review of the whole diff (three read-only reviewers by area) found nothing that could change a game.

## Times

`per_decision.py BASE NEW`, one thread, c2, d2, from seed 1000, best of two (16 games greedy, 8 planner).
The games are the same games, so the ratio is the code. Read alone on a quiet box:
[tiers-main2.txt](tiers-main2.txt). `speed.py check` against a baseline taken from the base:
random 0.94, greedy 0.95, engine 0.95, planner 0.96.

P1's own cost (orig, 103's end, against the base; other games on the same seeds, so read ms/decision;
two reads side by side, so only ratios): engine 0.92 a decision, strategist 1.03, sweep 0.98, planner
0.91, `planner:strategy=1` 0.91 ([tiers-p1.txt](tiers-p1.txt)).

Against 103's r40 table (its end, before `plan_gross=1`: other games on the same seeds, so the
per-game ratio mixes P1's game length with the code):

| agent | 103 ms/game | 104 base | 104 after | after / 103 | after / base |
|---|---|---|---|---|---|
| engine | 369 | 322 | 305 | 0.83 | 0.95 |
| strategist | 299 | 286 | 274 | 0.92 | 0.96 |
| sweep | 3109 | 3347 | 3214 | 1.03 (games 7.12 → 7.50 rounds) | 0.96 |
| planner | 5058 | 4898 | 4596 | 0.91 | 0.94 |
| `planner:strategy=1` | 5710 | 5578 | 5290 | 0.93 | 0.95 |

## Profiles (strategist, 3 runs of 4 games from seed 1000, `profile_win.py`)

Base: 5437 samples. After: 4784 samples (the same games; 0.88 of the samples).

| function | base self | after self | base total | after total |
|---|---|---|---|---|
| binary search by name | 15.3% | 17.1% | | |
| `Eval::value` | 6.6% | 6.5% | 37.5% | 39.1% |
| `Rules::apply_to` | | | 47.0% | 44.7% |
| `GameState` clone | 4.1% | 3.1% | 6.4% | 5.1% |
| drops | 4.2% | 3.5% | 5.0% | 5.1% |
| `format_inner` | 0.4% | | 2.9% | 2.0% |
| `World::int` | 1.4% | 0.7% | 2.2% | 1.1% |
| seat-terms store (`LocalKey::with` under `Eval::value`) | | | 6.7% | 7.1% |
| `tableau_parts` | | | 4.0% | 3.3% |

Shares are of a smaller whole, so a flat share is a cost cut with the rest.

## Not done

- **Typed slots for the hottest vars** (item 1): the vars are a serde'd sorted table read by name from
  every module; a shadow beside it must stay out of serde and `PartialEq` and be written by every path
  that writes `vars` (`Cx::set_key`, `add_key`, the tests and determinization). Not started in the time.
- **Resuming a prompt's phase** (item 4): the redesign; not started.
- **Incremental heuristic** (item 5): not started.
- No class-2 approximation was tried, so no screen and no seeds spent past P1's (33300-33499).
