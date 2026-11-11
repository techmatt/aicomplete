# 174: speed at r73

Built on r73 (main b3e50a4). `RULES_REVISION` stays 73: no rules or content change. The sim players' play changed
once, by the adopted approximation (`discards` 0); every other change is exact. Seeds: 41701-41844 (the fresh read,
three 48-seed sets, d0-d4), 41845-41988 (a d3-d4 confirm), dev seeds 36800-36847 for timing and profiles. Next
free: 41989.

## Before and after

Base is main's release binary at b3e50a4 (`out/base/`), after is this item's (`out/final/`), both release builds,
6 threads, a quiet box (nothing else ran), the same seeds. Set a of a standard read (`read_specs.py read`, seeds
36800-36847, the three pairs, d0-d3).

| read | base | after | ratio |
|---|---|---|---|
| **standard read, strategist (3 sets, 1,728 games): wall** | 409 + 445 + 436 = 1,290 s | 91 + 97 + 88 = 276 s | 4.7x |
| **standard read, strategist: ms a decision** | 9.55 | 2.00 | 4.8x |
| **greedy on set a (576 games): wall** | 531 s | 99 s | 5.4x |
| **greedy on set a: ms a decision** | 10.74 | 2.04 | 5.3x |
| standard read, slowest game | 47.2 s (set c, br d2 36905) | 4.9 s | 9.7x |
| strategist, set a (576 games): wall | 409 s | 91 s | 4.5x |
| strategist, set a: ms a decision | 8.94 | 1.94 | 4.6x |
| strategist, set a: slowest game | 43.5 s (pr d1 36804, 12 rounds) | 3.6 s | 12x |
| greedy, set a's first 8 seeds (96 games): wall | 106 s | 16 s | 6.6x |
| greedy: ms a decision | 12.1 | 1.93 | 6.3x |
| fresh read, strategist (2,160 games, d0-d4): ms a decision | 11.64 | 2.00 | 5.8x |
| fresh read, greedy (720 games, d0-d4): ms a decision | 12.12 | 2.04 | 5.9x |
| oracle row (`skilled_rows.py` on set a, 96 games, 3 threads): wall | 1,379 s | 1,179 s | 1.17x |
| oracle row: oracle ms a decision (48 games) | 171 | 139 | 1.23x |
| oracle row: its strategist ms a decision (48 games) | 10.6 | 1.96 | 5.4x |
| `same.json` (192 games, planner rows, 6 threads): wall | 617 s | 384 s | 1.61x |

The two oracle rows ran side by side, 3 threads each (the same load on both). A row's wall time is its slowest game
(base: br d4 36803, 1,068 s), which a change of lines moves either way: the after row's slowest was br d3 36801, 625 s.

The fresh-read rows are summed game ms (each side ran alone at 6 threads, base beside a 2-thread check, so wall
times there are not a pair). Exact changes alone (`discards=99` on the after binary against base, identical games,
8 games, one thread): strategist 0.92x a decision, greedy 0.95x.

## Where the time went (base, r73)

- **Discard chains.** "discard which card (k of n)?" (Trade Blockade, Manhunt, Conscription, Data Breach,
  Commandeer, Purge the Unaligned, Raiding Parties): `profile` feature's prompt split, one game each:
  95% of the slowest set-a game (pr d1 36804, 37.8 s: Trade Blockade 41%, three decisions at 5 s each), 93% of
  greedy's slowest, about 50% of a typical d2 game. At `choices` 2 each answer resolved two more discards, so
  it scored every set of three cards of the hand (H³/6 positions, the chain memo already taking each set
  once), each replaying the threat phase from its snapshot (74 µs an apply there against 22 typical).
- **Inside the replay** (`profile_win.py`, the slowest game): `threats::run` 68%, `requisition` 17% (a hand's
  labels built for every answer replayed), the binary search by name 14% self, `Cx::tally` 8.6%.
- **Inside the heuristic** (typical game): the threats' need and share 10%, of it 7% every seat's passives
  walked once per threat.
- **The oracle** (d4, a 5 s window 30 s in): `Eval::value` 50%, the troop march 13% (`troop_targets_on` building
  strings for every hex at every troop step), need and share 7%.
- **Around the reads** (`item_time.py` on 166-168): waits on sims and farm logs were most of each item's wall
  time (168: 183 min on a farm log); builds 1-9%, tests 3-11%. A release build after an `aic-sim` edit took 28 s,
  after an `aic-core` edit 46 s, cold 59 s (6 jobs); `cargo test-all` 200 s after the change (README: ~4 min).
- After 174 (`profile_win.py`, 4 games from 36800, d1): `search::settled` 67%, `Eval::value` 49%,
  `Rules::apply_to` 38%, binary search 17.5% self; no prompt over 4% of a game. The ranked list is in
  `docs/speed.md`, *What is left*.

## What was done

Exact (the same values in the same order; `same_results.py` on `experiments/same.json`, 192 games with the
planner rows: identical; the oracle row's agent at d3 with `discards=99`, 6 games: identical, 318 s → 264 s on one thread; an independent
review of the diff found no difference):

1. `Cx::choose_key` (core): a choice among keys whose replay matches the answer without building the labels;
   the three discard sites use it (`requisition`, `effects::discard`, `discard_hand`).
2. `threats::standing_counters` and `need_and_share_with`: the heuristic walks every seat's passives once a
   position, not once a threat; `CounterSteps` reads the config's counter steps once an `Eval`.
3. `troops::target_here`: whether a hex holds a troop target, without building its target strings (the
   march's `target_gaps` read every hex this way at every step).

Approximation, adopted: **`discards` 0** (a new agent parameter on greedy, engine, planner and oracle; 99 is the
old play): an answer to a discard prompt is valued by the table it leaves (`search::settled` caps a discard
prompt's levels at `discards`).

## Approximations (fresh seeds 41701-41844, paired on base; `fresh-strategist.md`, `fresh-greedy.md`)

Strategist, 432 games a level (3 sets x 3 pairs x 48 seeds), and greedy on set a (144 a level):

| Arm | Speed (ms a decision, x) | d0 | d1 | d2 | d3 | d4 | Adopted |
|---|---|---|---|---|---|---|---|
| strategist base | 11.64 | 95.8 | 72.7 | 60.4 | 40.7 | 33.6 | |
| strategist `discards=1` | 3.12 (3.7x) | −2.8 ±1.1 | +0.9 ±2.0 | +1.4 ±2.2 | −1.4 ±2.0 | −1.2 ±1.7 | no: d0 2.5 SE |
| strategist `discards=0` | 2.00 (5.8x) | −1.4 ±1.2 | −0.9 ±2.2 | −2.1 ±2.4 | −0.5 ±2.1 | +0.2 ±2.0 | **yes** |
| greedy base | 12.12 | 93.1 | 76.4 | 63.9 | 43.8 | 40.3 | |
| greedy `discards=1` | 3.15 (3.8x) | +2.1 ±2.1 | +0.0 ±3.3 | +2.1 ±4.0 | −4.2 ±3.7 | +2.8 ±2.8 | |
| greedy `discards=0` | 2.04 (5.9x) | +1.4 ±2.4 | −0.7 ±3.3 | +4.9 ±4.0 | −0.7 ±3.6 | +1.4 ±3.1 | **yes** |

Gaps are paired on the same pair, level and seed (± the gap's SE). The strategist led greedy at d0 only before
(set a: +4.9) and still does (+4.9 against greedy's `discards=0`). Bands (d0 90-100, d1 70-80, d2 55-65, d3 40-50):
the strategist's base met all four on these seeds (d3 40.7); with `discards=0` 94.4 / 71.8 / 58.3 / 40.3, all four
met. The engine, planner and oracle take the same default without a strength read of their own; the oracle row
below is the only one read.

## The ladder after 174 (r73, `discards` 0; the fresh read is the re-read: the final build plays it exactly)

| Level | Band | Strategist | SE | Greedy (set a) | SE |
|---|---|---|---|---|---|
| d0 | 90-100 | 94.4 | 1.1 | 94.4 | 1.9 |
| d1 | 70-80 | 71.8 | 2.2 | 75.7 | 3.6 |
| d2 | 55-65 | 58.3 | 2.4 | 68.8 | 3.9 |
| d3 | 40-50 | 40.3 | 2.4 | 43.1 | 4.1 |
| d4 | below d3 | 33.8 | 2.3 | 41.7 | 4.1 |

With the confirm read (41845-41988) the strategist's d3 is 40.3 ±1.7 and d4 35.4 ±1.6 over 864 games each.
Every band holds (d3 at its floor, as before 174: base 40.7 and 42.1 on the two reads); the strategist leads greedy
at d0 only, as before.

The oracle row (seeds 36800-36807 a pair, `oracle-row.md`): the oracle 100.0 / 95.8 → 95.8 / 100.0 at d3 / d4 (24
games each, one game each way). Its strategist rows read 50.0 → 33.3 and 45.8 → 29.2 (−16.7 ±9.8 each, paired),
and set a's timing runs on the same dev seeds read d3 −9.7 ±3.9 (144 games; d0-d2 +2.1, 0.0, +0.7). Those are the seeds the item profiled and timed on, so the d3 gap was checked on the
next fresh seeds before the adoption stood: 41845-41988, d3 and d4, 432 games a level (`confirm-d3d4.md`): d3 42.1
→ 40.3 (−1.9 ±2.1), d4 37.0 → 37.0 (0.0 ±1.9), 1.80 ms a decision against 11.61. Both fresh reads together, d3 is
−1.2 ±1.5 and d4 +0.1 ±1.4 (864 games each). The dev-seed reading did not repeat on fresh seeds.

## Speed baseline

`speed.py check` gave no verdict (baseline r69, binary r73): against the r69 numbers greedy read 0.32x, the
engine 0.23x, the planner 0.51x. `--update` from the after binary at r73: random 11.6, greedy 547.8, engine 378.7,
planner 12,782.8 ms a game (one thread).

## Next payoffs

In `docs/speed.md`, *What is left, by payoff*: the heuristic's per-position board reads (49%: vectors, shelter
saves, threats, the Rally forecast), the replay under `settled` (38%), typed slots for the hottest vars (binary
search 17.5% self), allocations (about 15%), the oracle row's d4 lines.

## Files

`specs174.py` (timing and profile specs), `fresh174.py` (the fresh read), `oracle_spec174.py` (one oracle game;
`exact`: the oracle's identical-games check), `run174.sh` (specs on one binary), `tail174.py` (ms a decision and
the slowest games), `paired174.py` (an arm against base on the same seeds).
