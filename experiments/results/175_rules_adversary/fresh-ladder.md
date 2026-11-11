# 175 fresh ladder: r74 against r73, seeds 41989-42132

Strategist on sets a-c (432 games a level), greedy on set a (144), the three pairs, d0-d4; paired on (pair, level, seed).

| agent | d | games | old win % | new win % | gap | gap SE |
|---|---|---|---|---|---|---|
| greedy | 0 | 144 | 90.3 | 93.8 | +3.5 | 2.7 |
| greedy | 1 | 144 | 72.9 | 79.2 | +6.2 | 4.9 |
| greedy | 2 | 144 | 73.6 | 63.2 | -10.4 | 4.6 |
| greedy | 3 | 144 | 50.7 | 36.8 | -13.9 | 5.0 |
| greedy | 4 | 144 | 43.1 | 39.6 | -3.5 | 4.3 |
| strategist | 0 | 432 | 94.4 | 89.6 | -4.9 | 1.7 |
| strategist | 1 | 432 | 73.1 | 72.5 | -0.7 | 2.8 |
| strategist | 2 | 432 | 59.3 | 53.0 | -6.2 | 2.7 |
| strategist | 3 | 432 | 38.9 | 35.2 | -3.7 | 2.8 |
| strategist | 4 | 432 | 31.7 | 40.7 | +9.0 | 2.7 |

| agent | side | games | ms/decision | decisions/game | rounds/game |
|---|---|---|---|---|---|
| greedy | old | 720 | 1.81 | 491 | 8.05 |
| greedy | new | 720 | 1.68 | 508 | 8.30 |
| strategist | old | 2160 | 1.58 | 471 | 8.06 |
| strategist | new | 2160 | 1.51 | 497 | 8.45 |

## Losses by cause, r74

| agent | variant | d | games | win % | rounds | round cap % of losses | discord_deck % of losses | last node % of losses | node corruptions/game | nodes lost/game |
|---|---|---|---|---|---|---|---|---|---|---|
| greedy | all | 0 | 144 | 93.8 | 7.49 | 100 | 0 | 0 | 0.51 | 1.56 |
| greedy | all | 1 | 144 | 79.2 | 8.22 | 93 | 7 | 0 | 1.42 | 4.15 |
| greedy | all | 2 | 144 | 63.2 | 8.88 | 23 | 70 | 8 | 2.37 | 6.19 |
| greedy | all | 3 | 144 | 36.8 | 8.61 | 70 | 25 | 4 | 2.32 | 6.35 |
| greedy | all | 4 | 144 | 39.6 | 8.31 | 66 | 29 | 6 | 1.62 | 6.48 |
| strategist | all | 0 | 432 | 89.6 | 7.99 | 100 | 0 | 0 | 0.59 | 1.98 |
| strategist | all | 1 | 432 | 72.5 | 8.70 | 86 | 13 | 1 | 1.63 | 4.81 |
| strategist | all | 2 | 432 | 53.0 | 8.96 | 28 | 67 | 6 | 2.23 | 6.44 |
| strategist | all | 3 | 432 | 35.2 | 8.56 | 66 | 26 | 8 | 2.03 | 6.29 |
| strategist | all | 4 | 432 | 40.7 | 8.03 | 49 | 43 | 8 | 1.50 | 6.56 |

## Losses by cause, r73

| agent | variant | d | games | win % | rounds | round cap % of losses | discord_deck % of losses | last node % of losses | node corruptions/game | nodes lost/game |
|---|---|---|---|---|---|---|---|---|---|---|
| greedy | all | 0 | 144 | 90.3 | 7.51 | 100 | 0 | 0 | 0.00 | 1.39 |
| greedy | all | 1 | 144 | 72.9 | 8.53 | 100 | 0 | 0 | 0.00 | 3.76 |
| greedy | all | 2 | 144 | 73.6 | 8.29 | 42 | 58 | 0 | 0.00 | 4.74 |
| greedy | all | 3 | 144 | 50.7 | 8.26 | 70 | 30 | 0 | 0.00 | 4.90 |
| greedy | all | 4 | 144 | 43.1 | 7.66 | 29 | 71 | 0 | 0.00 | 5.74 |
| strategist | all | 0 | 432 | 94.4 | 7.41 | 100 | 0 | 0 | 0.00 | 1.50 |
| strategist | all | 1 | 432 | 73.1 | 8.39 | 80 | 20 | 0 | 0.00 | 4.22 |
| strategist | all | 2 | 432 | 59.3 | 8.57 | 31 | 69 | 0 | 0.00 | 5.46 |
| strategist | all | 3 | 432 | 38.9 | 8.25 | 53 | 47 | 0 | 0.00 | 5.18 |
| strategist | all | 4 | 432 | 31.7 | 7.67 | 25 | 75 | 1 | 0.00 | 5.92 |

## The standard ladder read and measures (scripts/bench/ladder.py --measures, strategist; d4 has no band)

| d | r74 win % | ±SE | r73 win % | gap | ±SE (paired) | sets (r74) | band | verdict |
|---|---|---|---|---|---|---|---|---|
| d0 | **89.6** | 1.5 | 94.4 | -4.9 | 1.7 | 88.9 / 90.3 / 89.6 | 90-100 | **unmet** | (432 games)
| d1 | **72.5** | 2.1 | 73.1 | -0.7 | 2.8 | 68.1 / 73.6 / 75.7 | 70-80 | met | (432 games)
| d2 | **53.0** | 2.4 | 59.3 | -6.2 | 2.7 | 48.6 / 53.5 / 56.9 | 55-65 | **unmet** | (432 games)
| d3 | **35.2** | 2.3 | 38.9 | -3.7 | 2.8 | 36.8 / 34.0 / 34.7 | 40-50 | **unmet** | (432 games)
| d4 | **40.7** | 2.4 | 31.7 | +9.0 | 2.7 | 41.7 / 37.5 / 43.1 | 0-100 | met | (432 games)

| side | d | games | corruption nodes lost | ambassadors placed |   of them rallied | corruptions | compute made | projects revealed | revealed, share of deck | distinct revealed, share | projects resolved | resolved, share of deck | project reshuffles | rounds |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| r74 | d0 | 432 | 0.55 | 25.25 | 23.13 | 1.61 | 81.43 | 6.92 | 0.35 | 0.35 | 2.55 | 0.13 | 0.00 | 7.99 |
| r74 | d1 | 432 | 2.84 | 77.72 | 73.33 | 14.27 | 114.63 | 9.83 | 0.49 | 0.48 | 4.87 | 0.24 | 0.02 | 8.70 |
| r74 | d2 | 432 | 3.83 | 104.91 | 97.62 | 16.89 | 345.28 | 34.03 | 1.70 | 0.95 | 16.65 | 0.83 | 3.88 | 8.96 |
| r74 | d3 | 432 | 3.76 | 99.96 | 92.21 | 16.38 | 379.92 | 35.43 | 1.77 | 0.98 | 17.89 | 0.89 | 4.14 | 8.56 |
| r74 | d4 | 432 | 3.10 | 77.83 | 68.65 | 13.80 | 311.64 | 32.56 | 1.63 | 0.94 | 15.56 | 0.78 | 3.44 | 8.03 |
| r73 | d0 | 432 | 0.29 | 23.22 | 21.31 | 1.92 | 72.10 | 4.28 | 0.21 | 0.21 | 1.87 | 0.09 | 0.00 | 7.41 |
| r73 | d1 | 432 | 2.37 | 79.49 | 75.76 | 15.06 | 105.09 | 6.41 | 0.32 | 0.32 | 3.76 | 0.19 | 0.00 | 8.39 |
| r73 | d2 | 432 | 3.09 | 102.53 | 96.10 | 16.68 | 321.18 | 18.12 | 0.91 | 0.86 | 15.03 | 0.75 | 0.41 | 8.57 |
| r73 | d3 | 432 | 3.02 | 100.15 | 92.76 | 16.78 | 363.38 | 19.93 | 1.00 | 0.92 | 17.13 | 0.86 | 0.64 | 8.25 |
| r73 | d4 | 432 | 2.71 | 79.47 | 70.23 | 15.47 | 304.37 | 18.12 | 0.91 | 0.86 | 14.93 | 0.75 | 0.39 | 7.67 |
