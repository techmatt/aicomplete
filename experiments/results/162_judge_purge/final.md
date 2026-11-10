# 162: the final read (fresh seeds 40264-40359: set a 40264-40311, set b 40312-40359; the three pairs, d0-d3; 288 games a level and agent)

old = engine:strategy=1,w.clock=0 (157's strategist); new = the strategist (greedy's flat weights with strategy, w.purge 3.6). Paired on pair and seed, across engines (Matt, 7325); win % computed from the game lines (judges162.py). Farmed: 24 shards, main box 16 (native), the laptop tab 8 (wasm); set a 864 native / 864 wasm games, set b 1440 / 288.

## Against greedy
| Level | Agent | Games | Win % | SE | Gap to greedy | Gap SE | Sets | Freed | Amb bought | Liberates |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 | old | 288 | 96.2 | 1.1 | -2.4 | 1.3 | 97.9 / 94.4 | 1.31 | 0.47 | 2.82 |
| d0 | new | 288 | 98.3 | 0.8 | -0.3 | 0.9 | 99.3 / 97.2 | 2.46 | 1.50 | 4.20 |
| d0 | greedy | 288 | 98.6 | 0.7 | - | - | 99.3 / 97.9 | 2.39 | 0.91 | 4.63 |
| d1 | old | 288 | 79.5 | 2.4 | -8.7 | 2.7 | 78.5 / 80.6 | 3.22 | 0.87 | 5.97 |
| d1 | new | 288 | 93.1 | 1.5 | +4.9 | 2.3 | 91.7 / 94.4 | 4.56 | 2.51 | 6.95 |
| d1 | greedy | 288 | 88.2 | 1.9 | - | - | 88.9 / 87.5 | 4.12 | 1.55 | 7.39 |
| d2 | old | 288 | 65.6 | 2.8 | -13.2 | 3.4 | 68.1 / 63.2 | 3.38 | 0.94 | 6.02 |
| d2 | new | 288 | 80.2 | 2.3 | +1.4 | 3.0 | 81.2 / 79.2 | 5.26 | 2.89 | 7.15 |
| d2 | greedy | 288 | 78.8 | 2.4 | - | - | 80.6 / 77.1 | 4.95 | 1.94 | 7.71 |
| d3 | old | 288 | 51.0 | 2.9 | -15.6 | 3.7 | 56.2 / 45.8 | 2.97 | 0.79 | 5.40 |
| d3 | new | 288 | 65.6 | 2.8 | -1.0 | 3.2 | 66.0 / 65.3 | 4.70 | 2.67 | 6.30 |
| d3 | greedy | 288 | 66.7 | 2.8 | - | - | 67.4 / 66.0 | 4.52 | 1.67 | 7.06 |

## The ladder: new against old on the same seeds
| Level | Agent | Games | Win % | SE | Gap to old | Gap SE | Sets | Freed | Amb bought | Liberates |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 | old | 288 | 96.2 | 1.1 | - | - | 97.9 / 94.4 | 1.31 | 0.47 | 2.82 |
| d0 | new | 288 | 98.3 | 0.8 | +2.1 | 1.4 | 99.3 / 97.2 | 2.46 | 1.50 | 4.20 |
| d0 | greedy | 288 | 98.6 | 0.7 | +2.4 | 1.3 | 99.3 / 97.9 | 2.39 | 0.91 | 4.63 |
| d1 | old | 288 | 79.5 | 2.4 | - | - | 78.5 / 80.6 | 3.22 | 0.87 | 5.97 |
| d1 | new | 288 | 93.1 | 1.5 | +13.5 | 2.8 | 91.7 / 94.4 | 4.56 | 2.51 | 6.95 |
| d1 | greedy | 288 | 88.2 | 1.9 | +8.7 | 2.7 | 88.9 / 87.5 | 4.12 | 1.55 | 7.39 |
| d2 | old | 288 | 65.6 | 2.8 | - | - | 68.1 / 63.2 | 3.38 | 0.94 | 6.02 |
| d2 | new | 288 | 80.2 | 2.3 | +14.6 | 3.4 | 81.2 / 79.2 | 5.26 | 2.89 | 7.15 |
| d2 | greedy | 288 | 78.8 | 2.4 | +13.2 | 3.4 | 80.6 / 77.1 | 4.95 | 1.94 | 7.71 |
| d3 | old | 288 | 51.0 | 2.9 | - | - | 56.2 / 45.8 | 2.97 | 0.79 | 5.40 |
| d3 | new | 288 | 65.6 | 2.8 | +14.6 | 3.6 | 66.0 / 65.3 | 4.70 | 2.67 | 6.30 |
| d3 | greedy | 288 | 66.7 | 2.8 | +15.6 | 3.7 | 67.4 / 66.0 | 4.52 | 1.67 | 7.06 |

## The oracle row (local, the read's first 8 seeds a pair, 40264-40271)

| Level | oracle | new | old | greedy |
|---|---|---|---|---|
| d3 | 100.0 (24 of 24) | 75.0 | 54.2 | 83.3 |
| d4 | 83.3 (20 of 24: bp 8/8, br 6/8, pr 6/8) | - | - | - |

## Cost (s a game, main-box shards under load)

| Agent | Level | Games timed | s a game (mean) | s a game (max) |
|---|---|---|---|---|
| engine:strategy=1,w.clock=0 | d0 | 206 | 3.3 | 40.1 |
| engine:strategy=1,w.clock=0 | d1 | 205 | 3.6 | 43.9 |
| engine:strategy=1,w.clock=0 | d2 | 204 | 3.0 | 25.2 |
| engine:strategy=1,w.clock=0 | d3 | 202 | 1.5 | 6.5 |
| greedy | d0 | 205 | 2.5 | 30.6 |
| greedy | d1 | 205 | 4.6 | 71.2 |
| greedy | d2 | 203 | 6.8 | 69.5 |
| greedy | d3 | 202 | 6.3 | 119.4 |
| strategist | d0 | 205 | 2.3 | 47.7 |
| strategist | d1 | 205 | 4.3 | 42.7 |
| strategist | d2 | 204 | 6.1 | 60.3 |
| strategist | d3 | 202 | 5.2 | 75.7 |

## After the read: a third fresh set at d3 alone (40360-40407, 144 games an agent; farmed: main box 11 shards, tab 1)

| Level | Agent | Games | Win % | SE | Gap to greedy | Gap SE | Sets | Freed | Amb bought | Liberates |
|---|---|---|---|---|---|---|---|---|---|---|
| d3 | strategist | 144 | 69.4 | 3.8 | -4.9 | 4.2 | 69.4 | 4.66 | 2.38 | 6.22 |
| d3 | greedy | 144 | 74.3 | 3.6 | - | - | 74.3 | 4.37 | 1.69 | 7.01 |

Pooled over the three fresh sets (432 games): the strategist 66.9, greedy 69.2, gap -2.3 (2.5).
