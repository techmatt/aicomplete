## out/read_r55 (rules r55)

| Difficulty | Agent | Games | Win % | Rounds mean | median | p10-p90 | min-max | Ends | Unity tips | Discord tips | Unity wins before r6 | Earliest Unity win | Envoy | Unity held off | Discord off |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 79.9 ±3.3 | 8.40 | 8 | 6-11 | 5-12 | unity_deck 60, purge 55, node 16, deck_empty 11, discord_deck 2 | 7.28 | 2.89 | 0 | r7 | 2.04 | 1.97 | 4.01 |
| d0 | strategist:w.node_exposure=1 | 144 | 86.1 ±2.9 | 8.42 | 9 | 6-10 | 5-12 | unity_deck 78, purge 46, node 13, deck_empty 6, discord_deck 1 | 7.74 | 3.20 | 0 | r7 | 1.61 | 2.14 | 3.75 |
| d1 | strategist | 144 | 71.5 ±3.8 | 8.10 | 8 | 6-10 | 5-12 | unity_deck 68, node 37, purge 35, deck_empty 3, discord_deck 1 | 6.88 | 3.35 | 0 | r7 | 1.36 | 1.35 | 2.71 |
| d1 | strategist:w.node_exposure=1 | 144 | 70.1 ±3.8 | 8.10 | 8 | 6-10 | 5-12 | unity_deck 66, node 36, purge 35, discord_deck 6, deck_empty 1 | 6.97 | 3.73 | 0 | r7 | 1.42 | 0.97 | 2.39 |
| d2 | strategist | 144 | 63.9 ±4.0 | 7.75 | 8 | 4-10 | 4-12 | unity_deck 54, node 49, purge 38, discord_deck 2, deck_empty 1 | 6.12 | 3.33 | 0 | r7 | 0.95 | 1.08 | 2.03 |
| d2 | strategist:w.node_exposure=1 | 144 | 59.7 ±4.1 | 7.58 | 8 | 4-10 | 4-12 | unity_deck 61, node 51, purge 25, discord_deck 5, deck_empty 2 | 6.09 | 3.49 | 0 | r7 | 1.01 | 0.90 | 1.91 |

## out/read_r54 (rules r54)

| Difficulty | Agent | Games | Win % | Rounds mean | median | p10-p90 | min-max | Ends | Unity tips | Discord tips | Unity wins before r6 | Earliest Unity win | Envoy | Unity held off | Discord off |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 82.6 ±3.2 | 6.36 | 6 | 5-7 | 3-11 | unity_deck 107, node 23, purge 12, discord_deck 2 | 5.31 | 3.87 | 19 | r3 | 0.00 | 0.00 | 0.00 |
| d1 | strategist | 144 | 70.8 ±3.8 | 6.14 | 6 | 5-8 | 4-9 | unity_deck 102, node 39, discord_deck 3 | 4.94 | 4.10 | 21 | r4 | 0.00 | 0.00 | 0.00 |
| d2 | strategist | 144 | 61.1 ±4.1 | 5.86 | 6 | 4-8 | 4-8 | unity_deck 88, node 53, discord_deck 3 | 4.40 | 3.94 | 16 | r4 | 0.00 | 0.00 | 0.00 |

Paired NEW - OLD (OLD agent `strategist`), same pair and seed:

| Difficulty | NEW agent | Games | Win gap (points) | Rounds gap |
|---|---|---|---|---|
| d0 | strategist | 144 | -2.8 ±4.3 | +2.03 ±0.16 |
| d0 | strategist:w.node_exposure=1 | 144 | +3.5 ±4.0 | +2.06 ±0.14 |
| d1 | strategist | 144 | +0.7 ±4.4 | +1.97 ±0.14 |
| d1 | strategist:w.node_exposure=1 | 144 | -0.7 ±5.1 | +1.96 ±0.16 |
| d2 | strategist | 144 | +2.8 ±5.2 | +1.89 ±0.16 |
| d2 | strategist:w.node_exposure=1 | 144 | -1.4 ±5.0 | +1.72 ±0.18 |

NEW agents on the same seeds: strategist:w.node_exposure=1 - strategist:

| Difficulty | Win gap | Rounds gap | Node losses (on / off) |
|---|---|---|---|
| d0 | +6.2 ±3.8 | +0.03 | 13 / 16 |
| d1 | -1.4 ±5.0 | -0.01 | 36 / 37 |
| d2 | -4.2 ±4.6 | -0.17 | 51 / 49 |

