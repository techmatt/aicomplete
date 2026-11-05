# 117: cost per decision, r45 against r46

`python scripts/bench/per_decision.py out/r45 out/r46 --games 16` (the copied binaries with their configs), on a box with outside load (Matt, message 4866): read the ratios only. The standing `speed.py check` was stopped for the same reason; across a revision it gives no verdict anyway.

planner, c2, d2, 16 games from seed 1000, one thread, best of 2
| | rules | wins | rounds | decisions | options/decision | ms/game | ms/decision |
|---|---|---|---|---|---|---|---|
| old | r45 | 15 | 7.44 | 748 | 36.32 | 30354 | 40.6 |
| new | r46 | 12 | 7.06 | 476 | 20.53 | 13228 | 27.8 |
| new / old | | | 0.95 | 0.64 | 0.57 | 0.44 | 0.68 |
