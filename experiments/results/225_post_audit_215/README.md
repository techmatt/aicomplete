# 225: post-audit of 215 — speed since 215's baseline

`scripts/bench/speed_baseline.json` is still 215's r96 one. 215 updated it at 3:06pm on 2026-11-13, on a box that had
reported no free threads earlier that afternoon. On a quiet box (nothing beside it, 8:41pm), `speed.py check` on r98
reads random 0.94×, greedy 1.69×, engine 0.85× and planner 1.45× a game. It gives no verdict across revisions, so
`per_decision.py` is the read below. Each row is c2, d2, 6 games from seed 1000, one thread, best of 2. Each build is
`git archive` of the commit, built in its own target dir.

| old → new | agent | rounds | decisions | options / decision | ms / decision |
|---|---|---|---|---|---|
| r96 (215, 0d45395) → r98 (223, 9b628e9) | greedy | 1.38 | 1.46 | 1.17 | **1.33** |
| | engine | 0.84 | 0.76 | 0.99 | 0.95 |
| | planner | 1.05 | 1.12 | 0.98 | **1.30** |
| r96 → r97 (219, 971a46b) | greedy | 1.11 | 1.10 | 0.96 | 1.01 |
| | planner | 1.02 | 0.94 | 0.91 | 0.91 |
| r97 (222, e438864) → r98 (223) | greedy | 1.24 | 1.33 | 1.23 | **1.30** |
| | planner | 1.03 | 1.19 | 1.09 | **1.42** |

**The cost is 223's** (`cards.tag_income_once` 1, r98). Greedy and the planner pay about 1.3-1.4× a decision, with
1.1-1.2× options a decision, and greedy's games run longer on these seeds. 219 cost nothing a decision. 222 measured
greedy unchanged and the strategist cheaper. 223's write-up records no speed read. Nobody re-based the baseline after
r96. Keeping 223's slowdown (and `--update`) or finding its cause is the planner's call.
