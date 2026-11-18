# 267 dev screens

Dev seeds 36800-36831, three pairs, judge (strategist) and greedy pooled: 192 games a cell. `base` is r124 (main
f4ba70b, the asked-for ladder as it stood); every other arm is the r125 engine (no counter surcharge, no Muster +1,
no tier-2 look-ahead, sites print station and factory at 2, a city falls at 3 from level 1) with:

- `new`: no recovery (compute income 0, 3, 5, 13, 18 as r124);
- `st3`: station and factory printed at 3 (r124's d0 numbers, with no cut from d1);
- `ag`: agent income 0, 1, 1, 1, 2;
- compute income `ciA` 0, 6, 10, 18, 24; `ciB` 0, 9, 14, 22, 28; `c1` 0, 6, 8, 15, 20; `c2` 0, 8, 9, 16, 21;
  `c3` 0, 5, 7, 14, 22; `c4` (d1 only) 7. Picked here: `c1` with d1 at 7 (`c4`): 0, 7, 8, 15, 20; screen 4 then set d4 18 and screen 5 replaced d1-d2
  (shipped: **0, 3, 5, 15, 18** with the research marker at 4 from d1).

`python scripts/cards/spread_read.py` over `s267-base`, `s267-1`, `s267-2`, `s267-3` (`--pool-agents`):

| arm | agent | d | games | win % | ±SE | rounds | won in | deck-empty | botnets at cap | new cities a game | games with one | retakes | freed |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | both | d0 | 192 | 93.2 | 1.8 | 8.80 | 8.42 | 13 (7%) | 1.1 | 0.12 | 10% | 0.00 | 2.10 |
| base | both | d1 | 192 | 89.6 | 2.2 | 9.63 | 9.15 | 18 (9%) | 4.4 | 1.45 | 70% | 0.33 | 3.64 |
| base | both | d2 | 192 | 79.2 | 2.9 | 9.33 | 8.78 | 18 (9%) | 4.4 | 1.73 | 84% | 0.47 | 3.81 |
| base | both | d3 | 192 | 55.7 | 3.6 | 8.52 | 7.63 | 26 (14%) | 7.0 | 2.04 | 84% | 0.69 | 3.73 |
| base | both | d4 | 192 | 25.5 | 3.1 | 7.70 | 7.00 | 18 (9%) | 7.1 | 2.81 | 94% | 1.18 | 3.56 |
| new | both | d0 | 192 | 95.3 | 1.5 | 8.67 | 8.40 | 9 (5%) | 1.0 | 0.12 | 10% | 0.00 | 2.10 |
| new | both | d1 | 192 | 92.2 | 1.9 | 9.48 | 9.13 | 14 (7%) | 3.9 | 1.34 | 66% | 0.29 | 3.54 |
| new | both | d2 | 192 | 83.3 | 2.7 | 9.39 | 8.78 | 19 (10%) | 5.3 | 1.71 | 77% | 0.48 | 3.84 |
| new | both | d3 | 192 | 62.0 | 3.5 | 8.92 | 8.36 | 31 (16%) | 6.9 | 2.10 | 85% | 0.81 | 4.01 |
| new | both | d4 | 192 | 29.7 | 3.3 | 8.04 | 7.11 | 23 (12%) | 6.7 | 2.95 | 97% | 1.35 | 3.85 |
| st3 | both | d0 | 192 | 93.2 | 1.8 | 8.80 | 8.42 | 13 (7%) | 1.1 | 0.12 | 10% | 0.00 | 2.10 |
| st3 | both | d1 | 192 | 88.0 | 2.3 | 9.70 | 9.14 | 23 (12%) | 3.4 | 1.44 | 72% | 0.28 | 3.60 |
| st3 | both | d2 | 192 | 90.6 | 2.1 | 9.38 | 9.01 | 16 (8%) | 4.4 | 1.62 | 77% | 0.49 | 3.95 |
| st3 | both | d3 | 192 | 67.2 | 3.4 | 9.11 | 8.31 | 44 (23%) | 6.7 | 2.14 | 84% | 0.81 | 4.32 |
| st3 | both | d4 | 192 | 34.4 | 3.4 | 8.83 | 7.62 | 37 (19%) | 6.1 | 3.18 | 99% | 1.83 | 4.82 |
| ag | both | d1 | 192 | 83.9 | 2.7 | 9.24 | 8.57 | 21 (11%) | 3.4 | 1.97 | 88% | 0.68 | 4.35 |
| ag | both | d2 | 192 | 73.4 | 3.2 | 9.18 | 8.55 | 21 (11%) | 4.3 | 2.41 | 94% | 0.97 | 4.71 |
| ag | both | d3 | 192 | 49.5 | 3.6 | 8.49 | 7.76 | 31 (16%) | 8.1 | 2.67 | 94% | 1.08 | 4.31 |
| ag | both | d4 | 192 | 18.2 | 2.8 | 7.69 | 6.94 | 15 (8%) | 5.3 | 3.34 | 100% | 1.38 | 3.52 |
| ciA | both | d1 | 192 | 91.1 | 2.1 | 9.25 | 8.88 | 14 (7%) | 5.1 | 1.26 | 66% | 0.27 | 3.39 |
| ciA | both | d2 | 192 | 76.6 | 3.1 | 9.13 | 8.43 | 22 (11%) | 4.8 | 2.06 | 85% | 0.69 | 4.26 |
| ciA | both | d3 | 192 | 44.3 | 3.6 | 8.64 | 7.98 | 27 (14%) | 9.4 | 2.43 | 89% | 1.01 | 3.99 |
| ciA | both | d4 | 192 | 17.2 | 2.7 | 7.33 | 6.76 | 17 (9%) | 6.6 | 2.99 | 95% | 1.18 | 3.29 |
| ciB | both | d1 | 192 | 82.3 | 2.8 | 9.47 | 8.82 | 23 (12%) | 6.1 | 1.75 | 79% | 0.49 | 3.90 |
| ciB | both | d2 | 192 | 63.5 | 3.5 | 9.22 | 8.59 | 21 (11%) | 6.0 | 2.26 | 84% | 0.99 | 4.32 |
| ciB | both | d3 | 192 | 28.6 | 3.3 | 8.11 | 7.76 | 24 (12%) | 7.0 | 2.51 | 86% | 1.15 | 3.61 |
| ciB | both | d4 | 192 | 12.0 | 2.3 | 6.81 | 6.17 | 8 (4%) | 5.8 | 2.98 | 94% | 1.13 | 2.73 |
| c1 | both | d1 | 192 | 91.1 | 2.1 | 9.25 | 8.88 | 14 (7%) | 5.1 | 1.26 | 66% | 0.27 | 3.39 |
| c1 | both | d2 | 192 | 80.7 | 2.8 | 9.36 | 8.76 | 22 (11%) | 5.6 | 1.89 | 84% | 0.67 | 4.18 |
| c1 | both | d3 | 192 | 56.2 | 3.6 | 8.81 | 8.14 | 32 (17%) | 6.2 | 2.19 | 85% | 0.78 | 3.94 |
| c1 | both | d4 | 192 | 25.0 | 3.1 | 7.86 | 7.21 | 18 (9%) | 7.1 | 3.05 | 96% | 1.44 | 3.89 |
| c2 | both | d1 | 192 | 86.5 | 2.5 | 9.50 | 8.95 | 19 (10%) | 5.5 | 1.71 | 78% | 0.48 | 3.94 |
| c2 | both | d2 | 192 | 80.7 | 2.8 | 9.23 | 8.72 | 17 (9%) | 5.2 | 1.93 | 85% | 0.64 | 4.13 |
| c2 | both | d3 | 192 | 47.9 | 3.6 | 8.86 | 8.07 | 40 (21%) | 8.5 | 2.33 | 88% | 0.93 | 4.04 |
| c2 | both | d4 | 192 | 26.6 | 3.2 | 7.70 | 7.25 | 16 (8%) | 8.1 | 2.97 | 95% | 1.38 | 3.57 |
| c3 | both | d1 | 192 | 91.1 | 2.1 | 9.25 | 8.87 | 16 (8%) | 3.2 | 1.31 | 68% | 0.26 | 3.44 |
| c3 | both | d2 | 192 | 84.9 | 2.6 | 9.41 | 8.88 | 17 (9%) | 4.4 | 1.83 | 81% | 0.57 | 4.07 |
| c3 | both | d3 | 192 | 58.3 | 3.6 | 8.80 | 8.15 | 32 (17%) | 6.7 | 2.12 | 86% | 0.78 | 3.95 |
| c3 | both | d4 | 192 | 26.0 | 3.2 | 7.56 | 7.36 | 14 (7%) | 7.1 | 3.03 | 96% | 1.18 | 3.45 |
| c4 | both | d1 | 192 | 90.6 | 2.1 | 9.20 | 8.86 | 12 (6%) | 5.0 | 1.53 | 72% | 0.40 | 3.74 |

**Screen 4 (`s267-4`, d4 only):** after the diff review, d4's "tier-2 sooner" built into the stack (3 / 4 / 4:
`stage1_removed` 2, `stage2_added` 1), compute income at d4 20, 18 and 16. Shipped: 18 (base 25.5, 7.70 rounds).

| arm | agent | d | games | win % | ±SE | rounds | won in | deck-empty | botnets at cap | new cities a game | games with one | retakes | freed |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| k16 | both | d4 | 192 | 29.7 | 3.3 | 7.90 | 7.26 | 16 (8%) | 6.2 | 2.83 | 95% | 1.33 | 3.74 |
| k18 | both | d4 | 192 | 28.1 | 3.2 | 7.93 | 7.20 | 21 (11%) | 6.4 | 2.92 | 93% | 1.39 | 3.80 |
| k20 | both | d4 | 192 | 24.0 | 3.1 | 7.66 | 7.09 | 16 (8%) | 6.7 | 2.89 | 92% | 1.40 | 3.58 |

**Screen 5 (`s267-5`, d1-d2):** after the first fresh read tripped the tokens check (compute income 7 and 8 at d1-d2:
+44% and +19% adversary ops a round), r124's compute income there with the research marker at 4 from d1 (`r1`, shipped)
and compute 5, 6 with it (`r2`), against the first read's build (`base` here). Adversary ops a round, d1 / d2: base
29.5 / 39.3, `r1` 20.5 / 32.0, `r2` 24.8 / 34.2 (r124 on these seeds 20.4 / 33.1).

| arm | agent | d | games | win % | ±SE | rounds | won in | deck-empty | botnets at cap | new cities a game | games with one | retakes | freed |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | both | d1 | 192 | 90.6 | 2.1 | 9.20 | 8.86 | 12 (6%) | 5.0 | 1.53 | 72% | 0.40 | 3.74 |
| base | both | d2 | 192 | 80.7 | 2.8 | 9.36 | 8.76 | 22 (11%) | 5.6 | 1.89 | 84% | 0.67 | 4.18 |
| r1 | both | d1 | 192 | 90.1 | 2.2 | 9.59 | 9.16 | 17 (9%) | 3.7 | 1.40 | 67% | 0.33 | 3.59 |
| r1 | both | d2 | 192 | 83.9 | 2.7 | 9.49 | 8.93 | 20 (10%) | 5.3 | 1.66 | 77% | 0.53 | 3.94 |
| r2 | both | d1 | 192 | 90.6 | 2.1 | 9.22 | 8.79 | 18 (9%) | 3.7 | 1.29 | 68% | 0.30 | 3.44 |
| r2 | both | d2 | 192 | 82.8 | 2.7 | 9.35 | 8.79 | 20 (10%) | 4.7 | 1.73 | 78% | 0.54 | 3.89 |

d0 is the same in every arm but `st3` (compute income 0 there; the level-1 levers don't reach it): station and
factory printed at 2 moved d0 93.2 → 95.3, within 1 SE, and kept d0's new-city share at 10%. `st3` shows what the
station and factory cut carried at d1-d4 (2-9 points). By agent, base against `new` at d0: judge 93.8 → 97.9,
greedy 92.7 → 92.7.
