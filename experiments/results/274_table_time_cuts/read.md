# 274: the tables

## Paired fresh read (f274-read, seeds 49836-49883, r129 against r128)

Three board pairs, 48 seeds a cell, 1,440 games a side; both sides farmed from their own commits
(`farm-seeds --at 55db51d` for r128). Read by `experiments/results/273_smaller_numbers/paired.py NEW OLD`.

| Agent | Level | win % new | old | gap ± SE | rounds new | old | gap ± SE |
|---|---|---|---|---|---|---|---|
| strategist | d0 | 94 | 94 | -0.7 ± 0.7 | 8.47 | 8.44 | +0.02 ± 0.02 |
| strategist | d1 | 76 | 76 | +0.0 ± 1.4 | 10.83 | 10.77 | +0.06 ± 0.06 |
| strategist | d2 | 67 | 62 | +4.9 ± 1.8 | 10.58 | 10.70 | -0.12 ± 0.07 |
| strategist | d3 | 18 | 24 | -6.2 ± 2.3 | 9.42 | 9.21 | +0.22 ± 0.07 |
| strategist | d4 | 5 | 5 | +0.0 ± 1.4 | 8.06 | 8.01 | +0.05 ± 0.07 |
| strategist | d0-d4 | 52 | 52 | -0.4 ± 0.7 | 9.47 | 9.43 | +0.04 ± 0.03 |
| greedy | d0 | 91 | 91 | +0.0 ± 0.0 | 8.26 | 8.26 | +0.00 ± 0.01 |
| greedy | d1 | 74 | 74 | +0.0 ± 1.7 | 10.68 | 10.65 | +0.03 ± 0.04 |
| greedy | d2 | 56 | 56 | +0.0 ± 2.2 | 10.73 | 10.83 | -0.10 ± 0.11 |
| greedy | d3 | 19 | 19 | -0.7 ± 2.1 | 9.47 | 9.42 | +0.05 ± 0.08 |
| greedy | d4 | 5 | 4 | +0.7 ± 1.6 | 8.30 | 8.31 | -0.01 ± 0.09 |
| greedy | d0-d4 | 49 | 49 | +0.0 ± 0.8 | 9.49 | 9.49 | -0.01 ± 0.03 |

## The jam floor on (s274-jam, the same seeds, against r129's floor-off side)

A screen on spent seeds (question 1), paired by cell and seed: `paired.py s274-jam.jsonl f274-r129.jsonl`.

| Agent | Level | win % floor on | off | gap ± SE | rounds on | off |
|---|---|---|---|---|---|---|
| strategist | d0 | 94 | 94 | +0.7 ± 2.5 | 8.60 | 8.47 |
| strategist | d1 | 69 | 76 | -6.9 ± 4.6 | 10.84 | 10.83 |
| strategist | d2 | 62 | 67 | -4.2 ± 3.8 | 10.63 | 10.58 |
| strategist | d3 | 22 | 18 | +4.2 ± 3.1 | 9.18 | 9.42 |
| strategist | d4 | 2 | 5 | -2.8 ± 1.7 | 7.91 | 8.06 |
| strategist | d0-d4 | 50 | 52 | -1.8 ± 1.5 | 9.43 | 9.47 |
| greedy | d0-d4 | 46 | 49 | -3.2 ± 1.5 | 9.53 | 9.49 |

## Resource use (the judge, r129, levels pooled)

`experiments/results/273_smaller_numbers/resource_use.py READ --agent strategist`: 1,440 seats, 13,640 seat-rounds.

| Resource | gained / seat-round | spent / seat-round | spent ÷ gained | peak held (mean) | left at end (mean) | spent on (share) |
|---|---|---|---|---|---|---|
| credits | 19.08 | 20.12 | 1.05 | 33.2 | 7.5 | play 69%, standard 22%, pay 5%, power 3% |
| insight | 6.13 | 5.04 | 0.82 | 16.2 | - | keep 57%, play 21%, power 14%, pay 4% |
| energy | 5.10 | 4.11 | 0.81 | 14.9 | 10.9 | power 61%, use 18%, pay 11%, play 8% |
| chips | 0.87 | 0.69 | 0.79 | 3.5 | 1.6 | convert 64%, power 25%, standard 8%, threats.execute 2% |
| antimatter | 0.37 | 0.17 | 0.47 | 2.4 | 1.8 | play 59%, use 40%, adversary.compute 1% |
| research_cubes | 0.78 | 0.61 | 0.78 | 2.9 | - | convert 41%, use 29%, standard 12%, power 10% |
| card_tokens | 0.23 | 0.14 | 0.63 | 1.3 | - | play 49%, use 35%, power 9%, adversary.compute 6% |
| ambassadors | 1.87 | 1.69 | 0.90 | 7.3 | 2.7 | standard 96%, play 3%, threats.execute 1%, adversary.compute 0% |

## Smoke and jam-floor screen (s274-smoke, dev seeds 36800-36815, judge, three pairs)

`counters.py smoke.jsonl experiments/farm/s274-smoke.json`, per game-round; `jam` is `adversary.jam_floor=1`.
No timeouts, no forced pulls at d2.

| Arm | Level | Win % | Rounds | Troop moves | Stack steps | Stacks marched | Strikes | Project candidates read | Threats dealt | Turned and rejected | Skipped (literal) | Tag-limited skip would fire |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | d2 | 65 | 9.9 | 1.09 | 0.64 | 1.48 | 0.82 | 0.16 | 2.69 | 2.48 | 0 | 0 |
| jam on | d2 | 71 | 10.0 | 1.37 | 0.85 | 1.56 | 0.80 | 0.22 | 2.79 | 2.54 | 0 | 0 |
| base | d4 | 21 | 8.1 | 3.80 | 1.88 | 3.08 | 1.47 | 0.49 | 3.35 | 3.52 | 0 | 0 |
| jam on | d4 | 12 | 8.3 | 4.31 | 2.08 | 3.27 | 1.46 | 0.47 | 3.44 | 3.99 | 0 | 0 |

(48 games a cell: SE about 7 points on win %; the jam arm is neither better nor worse at this size.)

## Minutes a round each cut removes, by traced game

`minutes.py BEFORE AFTER --hexes-read N` (N 0.16 at d2, 0.49 at d4, the smoke's), on each traced seed (judge,
2 countries); before and after play the same troop moves in 13 of 14 d2 games and 8 of 14 at d4. The per-round table
for 268003 is in the README.

| Seed | d2: march checks / choices cut | compute components cut | minutes cut | d4: march checks / choices | compute | minutes cut |
|---|---|---|---|---|---|---|
| 268003 | 0.7 / 0.0 | 20.4 | 2.7 | 5.3 / 0.8 | 40.7 | 5.0 |
| 268010 | 0.0 / 0.0 | 17.2 | 2.4 | 2.7 / 0.0 | 50.9 | 4.9 |
| 36800 | 1.4 / 0.6 | 19.0 | 2.9 | 1.3 / -0.2 | 48.8 | 4.5 |
| 36801 | 0.0 / 0.0 | 24.5 | 2.8 | 0.1 / 0.1 | 52.0 | 4.6 |
| 36802 | 0.4 / 0.0 | 25.0 | 2.9 | 3.8 / 0.7 | 44.9 | 5.0 |
| 36803 | 0.0 / 0.0 | 24.5 | 2.8 | 0.1 / 0.0 | 55.4 | 4.7 |
| 36804 | 1.0 / 0.3 | 24.6 | 3.1 | 1.9 / 0.8 | 52.7 | 5.0 |
| 36805 | 0.0 / 0.0 | 17.1 | 2.4 | 0.0 / 0.0 | 44.1 | 4.1 |
| 36806 | 0.0 / 0.0 | 21.8 | 2.7 | 0.9 / 0.0 | 45.1 | 4.3 |
| 36807 | 0.1 / 0.0 | 21.9 | 2.7 | 0.0 / 0.0 | 44.7 | 4.1 |
| 36808 | 0.0 / 0.0 | 20.9 | 2.6 | 1.8 / 0.3 | 42.6 | 4.4 |
| 36809 | 0.0 / 0.0 | 18.3 | 2.5 | 1.2 / 0.2 | 46.8 | 4.5 |
| 36810 | 0.0 / 0.0 | 21.2 | 2.6 | 0.9 / 0.1 | 40.9 | 4.1 |
| 36811 | 0.1 / 0.1 | 21.7 | 2.7 | 8.6 / 2.3 | 50.4 | 6.4 |

(A negative choice count, 36800 at d4: a Total War troop's immediate march asks outside the March beat and the
reader pairs its choice with the next stack's move. Small; left.)
