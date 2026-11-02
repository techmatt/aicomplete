# 069: planner speedup on r28

All rates: 8 threads, below normal, `--profile fast`, d2, each binary in turn on the frozen seeds
(24000, 48 a cell, `experiments/screen-068.json`). Every step passed `same_results` against the
r28 binary on those 192 games.

| | before: games a min | before: ms a decision | after: games a min | after: ms a decision | × |
|---|---|---|---|---|---|
| planner c2 | 15.4 | 63.2 | 30.7 | 31.6 | 1.99 |
| planner c3 | 8.6 | 82.0 | 15.6 | 45.3 | 1.81 |
| engine c2 | 113 | 7.93 | 258 | 3.49 | 2.28 |
| engine c3 | 53.4 | 11.9 | 124 | 5.10 | 2.32 |

r27 on the same seed numbers (other games): planner 24.4 / 8.6, 40.6 / 76.2 ms a decision; engine 371 /
125, 2.60 / 4.94 ms a decision. Same decisions a game (484 and 494 at c2; 732 and 678 at c3), options a
decision 17.4 to 18.5 and 24.5 to 25.5.

**Cause of the r28 slowdown:** not more decisions, longer rollouts, dearer calls or clones. Calls
a decision went up (`apply` 1.46 ×, per-call cost flat), nearly all of it in Loss prompts: strikes on more
presence, chains of "destroy which of your presence" scored over every order of the same picks, and at
top level to a depth of 6. The engine's 3.1 × a decision fell to 1.3-1.4 × r27 once they were counted once.
c3 had not doubled: 1.07 × a decision, games shorter. The prompt's 45 / 85 s was 31 / 56 s here.

Top costs, c2 and c3 (8 seeds each, r28 after the first pass): `apply` 49-56% (86% of calls are answers,
each a replay from the snapshot), `eval` 33-42%, rollouts 85-92%, top-level scoring 10% before, 2% after.
By scoring: Loss prompts in `board.produce`, `board.bases`, `adversary.build` 46%, menus 47%, the rest 7%.

| Pass | c2 | c3 | what |
|---|---|---|---|
| 1 | 22.1 (release) | 12.0 (release) | a prompt scored its options twice (`stay`, no pass on the menu); top-level scoring lazy; word-at-a-time FNV; heuristic reads each hex's tokens once |
| 2 | 26.0 | 14.0 | (with pass 1) no check on replay; `KeyBuf` for hot keys; tags counted once a seat in `eval` |
| 3 | 32.2 | 15.6 | verified transpositions among Loss answers |
| 4 | 33.1 | 16.1 | income and hand in `eval`: no temporary lists, cards found by interned id, production a key walk |
| 5 | 30.7 | 15.6 | transpositions also ask for the same answers as a set, trigger and snapshot: a review found a table alone can match at another loop count |
| dropped | 24.2* | 13.0* | reusing a parent's resolved tree at its prompts: identical, saved nothing |
| dropped | 26.2 | 14.5 | transpositions on every prompt: identical, slower |

\\* release build, against 24.4 / 13.0 for release before it.

## Levers (step 3), planner, against the planner on the same seeds

Screen, seeds 24100, 48 a cell (`experiments/screen-069-speed.json`); rule: not behind by over 1 SE, 1.3 × or better, both sizes.

| Agent | c2 gap | c2 × | c3 gap | c3 × | Verdict |
|---|---|---|---|---|---|
| base | win 75.0 | 30.3 ms | win 68.8 | 50.2 ms | |
| `answers=1` | -4.2 ±5.1 | 1.70 | -12.5 ±7.1 | 1.65 | out, c3 |
| `powers=2` | +2.1 ±7.6 | 0.94 | +10.4 ±8.5 | 0.91 | out, no faster |
| `horizon=1` | -14.6 ±8.4 | 2.36 | -8.3 ±8.8 | 2.06 | out, c2 |
| `answers=1,powers=2` | -4.2 ±7.3 | 1.62 | -2.1 ±8.1 | 1.56 | to fresh seeds |
| `answers=1,horizon=1` | -27.1 ±10.2 | 4.11 | -12.5 ±8.2 | 3.83 | out |
| `powers=2,horizon=1` | -10.4 ±8.5 | 2.38 | -18.8 ±8.8 | 2.07 | out |
| all three | -22.9 ±9.1 | 3.96 | -10.4 ±8.0 | 3.78 | out |

Fresh seeds alone, 24200, 96 a cell (`experiments/screen-069-confirm.json`), base 72.9 / 83.3:

| Agent | c2 gap | c2 × | c3 gap | c3 × | Verdict |
|---|---|---|---|---|---|
| `answers=1` | +1.0 ±4.1 | 1.88 | -7.3 ±4.3 | 1.53 | out, c3 |
| `answers=1,powers=2` | -10.4 ±5.8 | 1.79 | -4.2 ±5.1 | 1.50 | out, c2 |

Nothing adopted; the planner's defaults are as they were. `powers=2` read alone (+2 and +10) is not
evidence for it: it has no speed case, and on `answers=1` it read +2 and +10 on the screen and -11 and +3 fresh.

## `sweep` (step 4), seeds 24400, 96 a cell (`experiments/screen-069-cheap.json`)

`planner:deals=1,lead=0` against the planner: -5.2 ±5.0 (c2), -3.1 ±4.3 (c3), 1.43 × and 1.53 × a game.

| Rank correlation over cards | c2 kept | c2 played | c3 kept | c3 played |
|---|---|---|---|---|
| planner against `sweep` | 0.985 | 0.986 | 0.988 | 0.991 |
| halves: planner against itself | 0.855 | 0.900 | 0.881 | 0.930 |
| halves: across the two | 0.849 | 0.904 | 0.891 | 0.936 |

Win deltas by card kept: half-sets of one agent read 0.01 and -0.03, so at 96 seeds they carry nothing
to hold. Adopted as a named profile on keep and play rates. Not the default.
