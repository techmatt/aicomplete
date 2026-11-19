# 276: power use by level on the fresh read (f276-read), r131 against r130

`python read.py NEW=SPEC OLD=SPEC [--agent greedy] --uses` (run from the repo root). Uses a seat-game; "all a round" is every board power use a seat-round, Growth and tiers included.

### Builder (strategist): uses a seat-game by level; "all a round" is every power use a seat-round

| Power | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| annex | 0.48 | 0.70 | 0.66 | 0.69 | 0.50 |
| buy_in | 2.21 | 2.43 | 2.23 | 1.76 | 1.20 |
| fortify_i | 0.54 | 1.14 | 1.31 | 1.25 | 0.85 |
| fortify_ii | 0.34 | 0.77 | 0.77 | 0.68 | 0.53 |
| overcharge_i | 1.29 | 1.54 | 1.29 | 0.66 | 0.67 |
| overcharge_ii | 0.12 | 0.11 | 0.14 | 0.08 | 0.01 |
| scrap_i | 1.91 | 1.96 | 1.65 | 1.20 | 1.07 |
| scrap_ii | 0.39 | 0.36 | 0.32 | 0.11 | 0.17 |
| sprawl | 3.68 | 3.58 | 3.46 | 2.98 | 2.66 |
| **all a round, r131** | 1.22 | 1.37 | 1.32 | 1.32 | 1.31 |
| **all a round, r130** | 2.39 | 2.43 | 2.49 | 2.55 | 2.51 |

### Protector (strategist): uses a seat-game by level; "all a round" is every power use a seat-round

| Power | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| accord | 3.49 | 4.33 | 4.70 | 3.74 | 2.94 |
| interdict_i | 0.89 | 1.66 | 1.99 | 1.94 | 1.72 |
| interdict_ii | 0.72 | 0.65 | 0.54 | 0.28 | 0.24 |
| levy | 0.56 | 0.47 | 0.29 | 0.41 | 0.24 |
| outpost | 2.54 | 2.27 | 2.24 | 1.95 | 1.90 |
| patrol_i | 2.99 | 4.05 | 3.70 | 2.92 | 2.30 |
| patrol_ii | 1.43 | 1.17 | 1.16 | 0.44 | 0.21 |
| **all a round, r131** | 1.47 | 1.60 | 1.57 | 1.52 | 1.48 |
| **all a round, r130** | 2.09 | 2.00 | 1.91 | 1.85 | 1.71 |

### Researcher (strategist): uses a seat-game by level; "all a round" is every power use a seat-round

| Power | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| field_station | 1.20 | 1.34 | 1.40 | 1.14 | 0.84 |
| field_survey | 2.04 | 1.93 | 2.10 | 1.61 | 1.27 |
| hack_i | 1.64 | 2.43 | 2.74 | 1.96 | 1.42 |
| hack_ii | 0.38 | 0.36 | 0.25 | 0.10 | 0.15 |
| lab_i | 5.49 | 5.47 | 5.69 | 4.18 | 2.69 |
| lab_ii | 0.24 | 0.21 | 0.19 | 0.12 | 0.19 |
| peer_review | 2.89 | 3.02 | 2.94 | 2.67 | 2.28 |
| **all a round, r131** | 1.55 | 1.60 | 1.64 | 1.56 | 1.46 |
| **all a round, r130** | 2.54 | 2.45 | 2.46 | 2.47 | 2.42 |

### Resource use (strategist, pooled over levels): a seat-round

| Resource | Gained (r130) | Spent (r130) | Spent of gained (r130) | Top uses (r131) |
|---|---|---|---|---|
| credits | 17.89 (18.01) | 19.13 (19.30) | 107% (107%) | play 71%, standard 23%, pay 4% |
| insight | 6.11 (6.05) | 4.90 (4.92) | 80% (81%) | keep 62%, play 21%, power 10% |
| energy | 4.40 (4.66) | 3.69 (3.97) | 84% (85%) | power 60%, use 17%, pay 11% |
| chips | 0.67 (0.85) | 0.48 (0.65) | 72% (76%) | convert 66%, power 19%, standard 9% |
| antimatter | 0.30 (0.32) | 0.16 (0.17) | 52% (52%) | play 63%, use 35%, phase 2% |
| research_cubes | 0.63 (0.62) | 0.48 (0.47) | 75% (76%) | convert 40%, use 28%, standard 19% |
| ambassadors | 1.96 (1.89) | 1.72 (1.68) | 88% (89%) | standard 95%, play 3%, phase 2% |


### Builder (greedy): uses a seat-game by level; "all a round" is every power use a seat-round

| Power | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| annex | 0.40 | 0.72 | 0.69 | 0.67 | 0.50 |
| buy_in | 2.72 | 2.62 | 2.76 | 1.78 | 1.24 |
| fortify_i | 0.58 | 1.08 | 1.26 | 1.09 | 0.88 |
| fortify_ii | 0.42 | 1.06 | 1.01 | 0.62 | 0.45 |
| overcharge_i | 1.41 | 1.59 | 1.48 | 0.72 | 0.65 |
| overcharge_ii | 0.17 | 0.15 | 0.17 | 0.07 | 0.02 |
| scrap_i | 2.19 | 2.29 | 2.15 | 1.50 | 1.11 |
| scrap_ii | 0.30 | 0.31 | 0.33 | 0.12 | 0.07 |
| sprawl | 3.48 | 3.50 | 3.43 | 3.01 | 2.75 |
| **all a round, r131** | 1.29 | 1.36 | 1.38 | 1.29 | 1.27 |
| **all a round, r130** | 2.70 | 2.80 | 2.76 | 2.71 | 2.56 |

### Protector (greedy): uses a seat-game by level; "all a round" is every power use a seat-round

| Power | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| accord | 3.73 | 4.24 | 4.30 | 3.30 | 2.62 |
| interdict_i | 0.81 | 1.43 | 1.56 | 1.79 | 1.61 |
| interdict_ii | 1.41 | 0.84 | 0.78 | 0.35 | 0.21 |
| levy | 0.53 | 0.55 | 0.59 | 0.54 | 0.36 |
| outpost | 2.75 | 2.52 | 2.47 | 2.35 | 2.17 |
| patrol_i | 3.36 | 4.26 | 4.43 | 3.24 | 2.42 |
| patrol_ii | 1.45 | 1.07 | 1.04 | 0.30 | 0.11 |
| **all a round, r131** | 1.55 | 1.55 | 1.53 | 1.48 | 1.46 |
| **all a round, r130** | 2.16 | 2.01 | 1.92 | 1.79 | 1.73 |

### Researcher (greedy): uses a seat-game by level; "all a round" is every power use a seat-round

| Power | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| field_station | 0.19 | 0.23 | 0.22 | 0.09 | 0.02 |
| field_survey | 4.39 | 4.71 | 4.30 | 3.28 | 2.42 |
| hack_i | 1.35 | 2.64 | 2.86 | 1.84 | 1.28 |
| hack_ii | 0.69 | 0.42 | 0.31 | 0.15 | 0.06 |
| lab_i | 5.43 | 6.01 | 5.62 | 4.04 | 2.94 |
| lab_ii | 0.08 | 0.19 | 0.15 | 0.05 | 0.05 |
| peer_review | 2.36 | 2.46 | 2.58 | 2.25 | 2.12 |
| **all a round, r131** | 1.64 | 1.66 | 1.64 | 1.53 | 1.46 |
| **all a round, r130** | 2.71 | 2.60 | 2.64 | 2.54 | 2.60 |
