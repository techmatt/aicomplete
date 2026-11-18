# 267 speed

`python scripts/bench/per_decision.py r124 r125 --agent AGENT --games 8 --set adversary.difficulty=2` (one thread, the box loaded: ratios only; other rules, so read ms a decision).

both with --set adversary.difficulty=2
| | rules | wins | rounds | decisions | options/decision | ms/game | ms/decision |
|---|---|---|---|---|---|---|---|
| old | r124 | 6 | 9.00 | 538 | 19.01 | 1016 | 1.9 |
| new | r125 | 7 | 8.50 | 513 | 19.41 | 937 | 1.8 |
| new / old | | | 0.94 | 0.95 | 1.02 | 0.92 | 0.97 |
--
both with --set adversary.difficulty=2
| | rules | wins | rounds | decisions | options/decision | ms/game | ms/decision |
|---|---|---|---|---|---|---|---|
| old | r124 | 8 | 8.75 | 560 | 24.66 | 1400 | 2.5 |
| new | r125 | 7 | 7.88 | 466 | 20.72 | 834 | 1.8 |
| new / old | | | 0.90 | 0.83 | 0.84 | 0.60 | 0.72 |
