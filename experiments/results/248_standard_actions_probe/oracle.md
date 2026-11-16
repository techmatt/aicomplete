48 seed-cells an arm and agent (4 seeds a pair, 12 a level)

| arm | agent | d0 | d1 | d2 | d3 | all | oracle wins where the judge lost |
|---|---|---|---|---|---|---|---|
| a | strategist | 12/12 | 12/12 | 12/12 | 10/12 | 46/48 |  |
| a | oracle | 12/12 | 12/12 | 12/12 | 12/12 | 48/48 | 2 (judge wins it loses: 0) |
| b | strategist | 12/12 | 9/12 | 6/12 | 5/12 | 32/48 |  |
| b | oracle | 12/12 | 12/12 | 11/12 | 12/12 | 47/48 | 15 (judge wins it loses: 0) |
| c | strategist | 5/12 | 3/12 | 0/12 | 0/12 | 8/48 |  |
| c | oracle | 12/12 | 11/12 | 10/12 | 8/12 | 41/48 | 33 (judge wins it loses: 0) |

## How each wins (won games, means)

| arm | agent | rounds | cities freed | by Liberate | by a card | Dispatches | cards played |
|---|---|---|---|---|---|---|---|
| a | strategist | 8.04 | 3.28 | 5.65 | 0.59 | 16.2 | 49.0 |
| a | oracle | 10.02 | 4.96 | 6.27 | 2.25 | 16.0 | 70.6 |
| b | strategist | 8.84 | 3.78 | 5.69 | 0.91 | 16.1 | 54.8 |
| b | oracle | 10.51 | 5.66 | 7.43 | 2.28 | 18.6 | 61.6 |
| c | strategist | 10.75 | 2.75 | 0.00 | 2.75 | 25.9 | 111.1 |
| c | oracle | 11.17 | 4.34 | 0.00 | 4.34 | 25.2 | 81.0 |

## The oracle's losses

- a: none
- b: seed 48376 br d2 (node, round 4)
- c: seed 48376 pr d3 (node, round 7), seed 48377 br d1 (node, round 4), seed 48377 br d2 (node, round 3), seed 48377 br d3 (node, round 3), seed 48377 pr d2 (node, round 4), seed 48377 pr d3 (node, round 4), seed 48378 br d3 (node, round 3)
