# 199 screens (dev seeds 36800-36831, strategist, three pairs, d0-d3, 384 games a cell)

Every new cell runs r87's rulings with r86's card-play track unless named (`screen_spec.py`). `r86` is the old binary (r86 with 199's stats-only counters) on the same seeds. `f10`/`f16`: the eval's claimed factory 10/16 (agent spec). **`f16` and `f16t1` both ran with t1 already in content** (content was edited while that farm ran: the working tree is what a farm reads), so `f16` is t1 + f16 and `f16t1` the same games: read f16 only against `t1`. Screen 1's unspent insight column read 0 (that build lacked the per-seat counter): screen 2's rows carry it.

## Screens 1-3

### Insight a seat-round, by board (to keeps / to plays / unspent at the action phase's end / bound)

| lever | d | games | win % | gap ± SE | Builder keep / play / unsp / bound | Protector | Researcher | Focus a Builder game | carried (Patience) / Synthesis a Researcher game |
|---|---|---|---|---|---|---|---|---|---|
| base | all | 384 | 66.1 | - | 2.14 / 1.59 / 4.9 / 11% | 2.48 / 1.66 / 5.9 / 9% | 3.04 / 1.48 / 15.1 / 11% | 0.01 | 112.6 / 0.00 |
| f10 | all | 384 | 70.1 | +3.9 ± 2.0 | 2.13 / 1.60 / 4.8 / 11% | 2.47 / 1.64 / 5.9 / 9% | 3.04 / 1.49 / 14.6 / 11% | 0.01 | 108.2 / 0.00 |
| f10t1 | all | 384 | 69.8 | +3.6 ± 2.5 | 1.96 / 1.57 / 3.3 / 16% | 2.29 / 1.51 / 3.8 / 20% | 1.92 / 1.49 / 5.7 / 18% | 0.03 | 42.0 / 0.00 |
| f16 | all | 384 | 62.0 | -4.2 ± 2.8 | 1.97 / 1.52 / 3.6 / 16% | 2.26 / 1.45 / 4.2 / 18% | 1.99 / 1.48 / 6.5 / 17% | 0.02 | 49.0 / 0.00 |
| f16t1 | all | 384 | 62.0 | -4.2 ± 2.8 | 1.97 / 1.52 / 3.6 / 16% | 2.26 / 1.45 / 4.2 / 18% | 1.99 / 1.48 / 6.5 / 17% | 0.02 | 49.0 / 0.00 |
| k3 | all | 384 | 62.0 | -4.2 ± 2.8 | 2.55 / 1.61 / 4.6 / 16% | 3.00 / 1.56 / 5.7 / 15% | 3.71 / 1.41 / 14.1 / 11% | 0.02 | 110.0 / 0.00 |
| r86 | all | 384 | 66.1 | +0.0 ± 1.5 | 2.11 / 1.58 / 0.0 / 11% | 2.46 / 1.60 / 0.0 / 9% | 3.04 / 1.51 / 0.0 / 11% | 0.00 | 111.9 / 0.00 |
| s1 | all | 384 | 66.1 | +0.0 ± 0.0 | 2.14 / 1.59 / 0.0 / 11% | 2.48 / 1.66 / 0.0 / 9% | 3.04 / 1.48 / 0.0 / 11% | 0.01 | 112.6 / 0.00 |
| s1t1 | all | 384 | 70.3 | +4.2 ± 2.6 | 1.99 / 1.58 / 0.0 / 16% | 2.29 / 1.55 / 0.0 / 20% | 1.94 / 1.54 / 0.0 / 18% | 0.02 | 43.3 / 0.00 |
| s1t1k | all | 384 | 70.3 | +4.2 ± 2.7 | 2.27 / 1.51 / 0.0 / 18% | 2.46 / 1.49 / 0.0 / 19% | 1.94 / 1.55 / 0.0 / 18% | 0.04 | 42.6 / 0.00 |
| s1t2 | all | 384 | 65.4 | -0.8 ± 2.6 | 1.88 / 1.39 / 0.0 / 31% | 2.23 / 1.51 / 0.0 / 23% | 1.91 / 1.56 / 0.0 / 16% | 0.03 | 41.2 / 0.00 |
| t1 | all | 384 | 70.3 | +4.2 ± 2.6 | 1.99 / 1.58 / 3.3 / 16% | 2.29 / 1.55 / 3.7 / 20% | 1.94 / 1.54 / 5.9 / 18% | 0.02 | 43.3 / 0.00 |
| t1k3 | all | 384 | 58.3 | -7.8 ± 2.8 | 2.41 / 1.45 / 3.1 / 27% | 2.55 / 1.41 / 3.7 / 31% | 2.15 / 1.53 / 6.2 / 23% | 0.02 | 49.2 / 0.00 |

### The keep plan (per game, both seats)

| lever | d | kept | played from keeps | dead: never had | dead: held back | dead in the last round | never-had share | late plays (share of kept plays) | late value: reasoned / printed | mean wait | passed: short of insight / unaffordable / chosen (a seat-game) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| base | all | 35.0 | 13.8 | 16.92 | 1.28 | 2.90 | 53% | 0.89 (6%) | 27.7 / 17.5 | 0.55 | 1.03 / 7.68 / 5.69 |
| f10 | all | 34.7 | 13.5 | 16.71 | 1.27 | 3.10 | 53% | 0.84 (6%) | 28.5 / 17.5 | 0.54 | 1.03 / 7.69 / 5.57 |
| f10t1 | all | 29.3 | 12.7 | 12.76 | 1.32 | 2.47 | 47% | 0.77 (6%) | 25.1 / 16.3 | 0.51 | 1.67 / 7.99 / 7.61 |
| f16 | all | 30.2 | 13.1 | 13.38 | 1.50 | 2.13 | 48% | 0.84 (6%) | 25.8 / 16.5 | 0.52 | 1.66 / 8.06 / 7.72 |
| f16t1 | all | 30.2 | 13.1 | 13.38 | 1.50 | 2.13 | 48% | 0.84 (6%) | 25.8 / 16.5 | 0.52 | 1.66 / 8.06 / 7.72 |
| k3 | all | 36.1 | 13.9 | 17.85 | 1.43 | 2.83 | 54% | 0.89 (6%) | 25.7 / 16.3 | 0.53 | 1.39 / 7.42 / 6.31 |
| r86 | all | 34.7 | 13.9 | 16.57 | 1.27 | 2.92 | 52% | 0.90 (6%) | 28.7 / 18.2 | 0.56 | 1.03 / 7.78 / 5.66 |
| s1 | all | 35.0 | 13.8 | 16.92 | 1.28 | 2.90 | 53% | 0.89 (6%) | 27.7 / 17.5 | 0.55 | 1.03 / 7.68 / 5.69 |
| s1t1 | all | 29.6 | 13.0 | 12.76 | 1.24 | 2.57 | 47% | 0.85 (7%) | 26.7 / 17.1 | 0.53 | 1.66 / 7.94 / 7.65 |
| s1t1k | all | 29.2 | 12.8 | 12.60 | 1.39 | 2.30 | 47% | 0.78 (6%) | 26.6 / 16.9 | 0.50 | 1.71 / 7.99 / 7.80 |
| s1t2 | all | 29.3 | 12.8 | 12.93 | 1.33 | 2.29 | 48% | 0.83 (6%) | 25.6 / 16.9 | 0.54 | 2.40 / 7.61 / 7.70 |
| t1 | all | 29.6 | 13.0 | 12.76 | 1.24 | 2.57 | 47% | 0.85 (7%) | 26.7 / 17.1 | 0.53 | 1.66 / 7.94 / 7.65 |
| t1k3 | all | 30.6 | 13.0 | 13.84 | 1.46 | 2.19 | 49% | 0.84 (6%) | 24.2 / 15.7 | 0.54 | 2.77 / 6.81 / 8.46 |

### Tier-2 keeps and plays by round (per game, both seats; tier 1 in brackets)

| lever | d | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| base | all | 0.74 / 0.83 (3.0 / 2.0) | 1.20 / 0.62 (1.5 / 2.1) | 1.63 / 0.77 (1.3 / 3.2) | 2.90 / 1.29 (1.4 / 3.2) | 3.14 / 1.34 (1.4 / 3.8) | 3.07 / 1.45 (1.3 / 3.8) | 2.99 / 1.36 (1.0 / 3.9) | 2.69 / 1.10 (0.7 / 2.8) | 1.83 / 0.75 (0.4 / 1.8) | 1.38 / 0.56 (0.2 / 1.2) |
| f10 | all | 0.74 / 0.83 (3.0 / 2.0) | 1.22 / 0.63 (1.5 / 2.1) | 1.64 / 0.76 (1.3 / 3.1) | 2.86 / 1.24 (1.4 / 3.2) | 3.16 / 1.42 (1.4 / 3.8) | 3.00 / 1.41 (1.3 / 3.8) | 2.98 / 1.38 (0.9 / 4.0) | 2.69 / 1.07 (0.7 / 2.8) | 1.78 / 0.69 (0.4 / 1.8) | 1.50 / 0.52 (0.2 / 1.2) |
| f10t1 | all | 0.74 / 0.83 (3.0 / 2.0) | 0.98 / 0.56 (1.8 / 2.1) | 1.03 / 0.61 (1.5 / 3.4) | 1.49 / 0.90 (1.5 / 3.6) | 2.10 / 1.16 (1.3 / 3.7) | 2.42 / 1.32 (1.0 / 4.2) | 2.52 / 1.29 (1.0 / 4.0) | 2.22 / 1.01 (0.6 / 3.3) | 1.49 / 0.64 (0.3 / 1.8) | 1.08 / 0.39 (0.2 / 0.9) |
| f16 | all | 0.74 / 0.83 (3.0 / 2.0) | 1.02 / 0.57 (1.8 / 2.1) | 1.06 / 0.64 (1.5 / 3.5) | 1.54 / 0.92 (1.4 / 3.6) | 2.09 / 1.15 (1.2 / 3.8) | 2.32 / 1.26 (1.1 / 4.1) | 2.52 / 1.25 (1.0 / 3.8) | 2.03 / 0.97 (0.6 / 3.3) | 1.58 / 0.65 (0.4 / 2.1) | 1.38 / 0.51 (0.2 / 1.3) |
| f16t1 | all | 0.74 / 0.83 (3.0 / 2.0) | 1.02 / 0.57 (1.8 / 2.1) | 1.06 / 0.64 (1.5 / 3.5) | 1.54 / 0.92 (1.4 / 3.6) | 2.09 / 1.15 (1.2 / 3.8) | 2.32 / 1.26 (1.1 / 4.1) | 2.52 / 1.25 (1.0 / 3.8) | 2.03 / 0.97 (0.6 / 3.3) | 1.58 / 0.65 (0.4 / 2.1) | 1.38 / 0.51 (0.2 / 1.3) |
| k3 | all | 0.35 / 0.74 (3.2 / 2.2) | 1.01 / 0.45 (1.8 / 2.1) | 1.17 / 0.63 (1.7 / 3.2) | 2.20 / 1.00 (1.8 / 3.4) | 2.83 / 1.12 (1.6 / 3.7) | 2.98 / 1.31 (1.5 / 4.1) | 3.00 / 1.32 (1.3 / 4.1) | 2.81 / 1.29 (0.9 / 3.0) | 2.10 / 0.77 (0.5 / 2.2) | 1.64 / 0.58 (0.3 / 1.3) |
| r86 | all | 0.74 / 0.83 (3.0 / 2.0) | 1.20 / 0.62 (1.5 / 2.1) | 1.63 / 0.77 (1.3 / 3.1) | 2.90 / 1.29 (1.4 / 3.1) | 3.18 / 1.38 (1.4 / 3.7) | 3.07 / 1.46 (1.3 / 3.8) | 2.91 / 1.35 (1.0 / 3.7) | 2.60 / 1.12 (0.7 / 2.8) | 1.84 / 0.72 (0.4 / 1.8) | 1.32 / 0.54 (0.2 / 1.2) |
| s1 | all | 0.74 / 0.83 (3.0 / 2.0) | 1.20 / 0.62 (1.5 / 2.1) | 1.63 / 0.77 (1.3 / 3.2) | 2.90 / 1.29 (1.4 / 3.2) | 3.14 / 1.34 (1.4 / 3.8) | 3.07 / 1.45 (1.3 / 3.8) | 2.99 / 1.36 (1.0 / 3.9) | 2.69 / 1.10 (0.7 / 2.8) | 1.83 / 0.75 (0.4 / 1.8) | 1.38 / 0.56 (0.2 / 1.2) |
| s1t1 | all | 0.74 / 0.83 (3.0 / 2.0) | 0.97 / 0.56 (1.8 / 2.1) | 1.04 / 0.61 (1.5 / 3.4) | 1.50 / 0.91 (1.5 / 3.6) | 2.10 / 1.18 (1.3 / 3.9) | 2.42 / 1.30 (1.1 / 4.3) | 2.58 / 1.28 (1.0 / 4.0) | 2.13 / 1.08 (0.7 / 3.2) | 1.57 / 0.59 (0.3 / 1.8) | 1.07 / 0.44 (0.2 / 0.9) |
| s1t1k | all | 0.74 / 0.83 (3.0 / 2.0) | 0.97 / 0.56 (1.8 / 2.1) | 1.03 / 0.61 (1.5 / 3.4) | 1.47 / 0.90 (1.5 / 3.6) | 2.06 / 1.12 (1.3 / 3.7) | 2.33 / 1.26 (1.1 / 4.1) | 2.44 / 1.30 (1.0 / 3.9) | 2.00 / 1.02 (0.7 / 3.2) | 1.55 / 0.66 (0.3 / 1.6) | 0.96 / 0.36 (0.2 / 1.0) |
| s1t2 | all | 0.74 / 0.83 (3.0 / 2.0) | 0.79 / 0.47 (2.0 / 2.0) | 0.95 / 0.62 (1.7 / 3.4) | 1.40 / 0.78 (1.5 / 3.5) | 2.03 / 0.98 (1.2 / 3.7) | 2.45 / 1.36 (1.1 / 4.0) | 2.51 / 1.23 (1.1 / 4.1) | 2.05 / 1.07 (0.7 / 3.4) | 1.50 / 0.63 (0.3 / 1.9) | 1.12 / 0.46 (0.2 / 1.1) |
| t1 | all | 0.74 / 0.83 (3.0 / 2.0) | 0.97 / 0.56 (1.8 / 2.1) | 1.04 / 0.61 (1.5 / 3.4) | 1.50 / 0.91 (1.5 / 3.6) | 2.10 / 1.18 (1.3 / 3.9) | 2.42 / 1.30 (1.1 / 4.3) | 2.58 / 1.28 (1.0 / 4.0) | 2.13 / 1.08 (0.7 / 3.2) | 1.57 / 0.59 (0.3 / 1.8) | 1.07 / 0.44 (0.2 / 0.9) |
| t1k3 | all | 0.35 / 0.74 (3.2 / 2.2) | 0.58 / 0.34 (2.4 / 2.1) | 0.70 / 0.46 (2.1 / 3.6) | 1.03 / 0.66 (1.9 / 3.8) | 1.48 / 0.72 (1.7 / 3.8) | 2.05 / 1.06 (1.4 / 4.5) | 2.27 / 1.22 (1.2 / 4.3) | 2.26 / 1.00 (0.9 / 3.3) | 1.68 / 0.79 (0.5 / 2.1) | 1.35 / 0.49 (0.3 / 1.2) |

### Claims and chips (per game, both seats; held a seat-round)

| lever | d | factory claims | chips held | seat-rounds at 0 | Mint | Scrap | chips spent | spent on (chips a game) | chip converter uses (by country) | idle (a seat holding chips could) |
|---|---|---|---|---|---|---|---|---|---|---|
| base | all | 0.02 | 0.28 | 91% | 2.60 | 0.33 | 6.75 | own:power:Procure 2.86, own:standard:station 1.99, own:convert:1:chips 1.88, own:play:orbital_mass_driver 0.01, own:use:parts_exchange 0.01 | 1: 0.94 | 0.66 |
| f10 | all | 0.46 | 0.39 | 89% | 2.54 | 0.38 | 7.72 | own:power:Procure 2.92, own:standard:station 2.41, own:convert:1:chips 2.38, own:play:orbital_mass_driver 0.01, own:use:parts_exchange 0.01 | 1: 1.19 | 0.76 |
| f10t1 | all | 0.49 | 0.49 | 89% | 2.48 | 0.43 | 8.08 | own:power:Procure 2.94, own:convert:1:chips 2.59, own:standard:station 2.51, own:use:parts_exchange 0.03, phase:threats.execute 0.01, own:play:orbital_mass_driver 0.01 | 1: 1.30 | 0.88 |
| f16 | all | 2.28 | 0.94 | 84% | 2.53 | 0.54 | 10.32 | own:convert:1:chips 3.96, own:standard:station 3.19, own:power:Procure 3.13, own:use:parts_exchange 0.02, own:play:orbital_mass_driver 0.01, phase:threats.execute 0.01 | 1: 1.98 | 1.45 |
| f16t1 | all | 2.28 | 0.94 | 84% | 2.53 | 0.54 | 10.32 | own:convert:1:chips 3.96, own:standard:station 3.19, own:power:Procure 3.13, own:use:parts_exchange 0.02, own:play:orbital_mass_driver 0.01, phase:threats.execute 0.01 | 1: 1.98 | 1.45 |
| k3 | all | 0.02 | 0.32 | 91% | 2.61 | 0.48 | 7.23 | own:power:Procure 2.98, own:standard:station 2.32, own:convert:1:chips 1.90, own:play:orbital_mass_driver 0.01, own:use:parts_exchange 0.01, phase:threats.execute 0.01 | 1: 0.95 | 0.71 |
| r86 | all | 0.00 | 0.41 | 86% | 2.60 | 0.35 | 5.69 | own:power:Procure 2.99, own:standard:station 2.67, own:play:orbital_mass_driver 0.03, own:use:parts_exchange 0.01 | - | 0.00 |
| s1 | all | 0.02 | 0.28 | 91% | 2.60 | 0.33 | 6.75 | own:power:Procure 2.86, own:standard:station 1.99, own:convert:1:chips 1.88, own:play:orbital_mass_driver 0.01, own:use:parts_exchange 0.01 | 1: 0.94 | 0.66 |
| s1t1 | all | 0.00 | 0.32 | 90% | 2.60 | 0.39 | 7.29 | own:power:Procure 2.98, own:standard:station 2.17, own:convert:1:chips 2.12, own:use:parts_exchange 0.01, own:play:orbital_mass_driver 0.01, phase:threats.execute 0.01 | 1: 1.06 | 0.70 |
| s1t1k | all | 0.00 | 0.35 | 90% | 2.60 | 0.38 | 7.02 | own:power:Procure 2.92, own:standard:station 2.13, own:convert:1:chips 1.93, own:play:orbital_mass_driver 0.02, own:use:parts_exchange 0.01 | 1: 0.97 | 0.70 |
| s1t2 | all | 0.01 | 0.31 | 91% | 2.79 | 0.36 | 7.37 | own:power:Procure 3.08, own:standard:station 2.20, own:convert:1:chips 2.07, own:play:orbital_mass_driver 0.02, phase:threats.execute 0.01 | 1: 1.03 | 0.67 |
| t1 | all | 0.00 | 0.32 | 90% | 2.60 | 0.39 | 7.29 | own:power:Procure 2.98, own:standard:station 2.17, own:convert:1:chips 2.12, own:use:parts_exchange 0.01, own:play:orbital_mass_driver 0.01, phase:threats.execute 0.01 | 1: 1.06 | 0.70 |
| t1k3 | all | 0.01 | 0.41 | 89% | 2.67 | 0.49 | 7.83 | own:power:Procure 3.00, own:standard:station 2.48, own:convert:1:chips 2.34, own:play:orbital_mass_driver 0.01, own:use:parts_exchange 0.01 | 1: 1.17 | 0.83 |

### Insight spent a seat-round by board (the game rows: keeps, plays, powers, counters, standard)

| lever | d | Builder | Protector | Researcher |
|---|---|---|---|---|
| base | all | 4.45 | 4.88 | 5.64 |
| f10 | all | 4.48 | 4.89 | 5.63 |
| f10t1 | all | 4.22 | 4.69 | 4.02 |
| f16 | all | 4.12 | 4.65 | 4.08 |
| f16t1 | all | 4.12 | 4.65 | 4.08 |
| k3 | all | 4.83 | 5.37 | 6.23 |
| r86 | all | 4.40 | 4.78 | 5.68 |
| s1 | all | 4.45 | 4.88 | 5.64 |
| s1t1 | all | 4.24 | 4.68 | 4.14 |
| s1t1k | all | 4.47 | 4.78 | 4.14 |
| s1t2 | all | 3.89 | 4.65 | 4.09 |
| t1 | all | 4.24 | 4.68 | 4.14 |
| t1k3 | all | 4.50 | 4.89 | 4.36 |


## Screens 4-6 (r87's content, track t1, eval owned_factory 10: `base` is r87 as shipped)

`pr2`/`pr3`: the keep plan's `plan_rounds` 2/3 (4); `t3`, `t4`: early-heavy tracks (`screen_spec.py`); `deal5`: a 5-card draft hand; `t4deal5`: both. Screen 6's dead-keep columns predate the fix that counts a kept card leaving the hand unplayed (sold, discarded) when it leaves.

### Insight a seat-round, by board (to keeps / to plays / unspent at the action phase's end / bound)

| lever | d | games | win % | gap ± SE | Builder keep / play / unsp / bound | Protector | Researcher | Focus a Builder game | carried (Patience) / Synthesis a Researcher game |
|---|---|---|---|---|---|---|---|---|---|
| base | all | 384 | 69.8 | - | 1.96 / 1.57 / 3.3 / 16% | 2.29 / 1.51 / 3.8 / 20% | 1.92 / 1.49 / 5.7 / 18% | 0.03 | 42.0 / 0.00 |
| pr2 | all | 384 | 60.2 | -9.6 ± 2.7 | 1.40 / 1.48 / 4.0 / 8% | 1.76 / 1.32 / 4.4 / 12% | 1.73 / 1.60 / 6.5 / 13% | 0.02 | 49.7 / 0.00 |
| pr3 | all | 384 | 63.8 | -6.0 ± 2.5 | 1.71 / 1.53 / 3.7 / 13% | 2.05 / 1.46 / 3.9 / 17% | 1.83 / 1.56 / 6.5 / 14% | 0.03 | 48.6 / 0.00 |
| s6 | all | 384 | 68.2 | -1.6 ± 1.6 | 1.96 / 1.59 / 3.2 / 16% | 2.30 / 1.47 / 3.8 / 20% | 1.94 / 1.51 / 5.7 / 18% | 0.02 | 41.9 / 0.00 |
| s6deal5 | all | 384 | 70.1 | +0.3 ± 2.7 | 2.13 / 1.57 / 3.3 / 18% | 2.52 / 1.54 / 3.7 / 25% | 2.14 / 1.53 / 5.0 / 21% | 0.03 | 36.8 / 0.00 |
| s6t4deal5 | all | 384 | 71.4 | +1.6 ± 2.8 | 2.28 / 1.71 / 4.3 / 12% | 2.68 / 1.61 / 5.3 / 11% | 3.01 / 1.60 / 8.8 / 13% | 0.04 | 64.4 / 0.00 |
| t3 | all | 384 | 65.4 | -4.4 ± 2.5 | 1.99 / 1.59 / 3.7 / 12% | 2.34 / 1.43 / 5.0 / 10% | 2.24 / 1.51 / 7.5 / 15% | 0.04 | 54.9 / 0.00 |
| t4 | all | 384 | 71.9 | +2.1 ± 2.5 | 2.05 / 1.65 / 4.3 / 11% | 2.35 / 1.57 / 5.6 / 9% | 2.60 / 1.56 / 9.9 / 11% | 0.01 | 72.4 / 0.00 |

### The keep plan (per game, both seats)

| lever | d | kept | played from keeps | dead: never had | dead: held back | dead in the last round | never-had share | late plays (share of kept plays) | late value: reasoned / printed | mean wait | passed: short of insight / unaffordable / chosen (a seat-game) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| base | all | 29.3 | 12.7 | 12.76 | 1.32 | 2.47 | 47% | 0.77 (6%) | 25.1 / 16.3 | 0.51 | 1.67 / 7.99 / 7.61 |
| pr2 | all | 24.7 | 11.2 | 10.51 | 1.07 | 1.94 | 46% | 0.48 (4%) | 26.9 / 16.8 | 0.38 | 1.05 / 9.35 / 10.21 |
| pr3 | all | 27.5 | 12.1 | 11.79 | 1.28 | 2.23 | 47% | 0.63 (5%) | 25.9 / 16.3 | 0.46 | 1.38 / 8.54 / 8.87 |
| s6 | all | 29.3 | 12.7 | 2.98 | 11.10 | 2.42 | 11% | 0.81 (6%) | 24.8 / 16.2 | 0.52 | 1.69 / 8.03 / 7.55 |
| s6deal5 | all | 32.4 | 13.8 | 3.31 | 12.44 | 2.79 | 11% | 0.84 (6%) | 25.7 / 16.4 | 0.56 | 2.86 / 9.75 / 10.81 |
| s6t4deal5 | all | 36.3 | 14.6 | 4.72 | 13.72 | 3.20 | 14% | 1.02 (7%) | 26.5 / 16.8 | 0.57 | 1.65 / 10.28 / 9.14 |
| t3 | all | 30.2 | 12.7 | 13.61 | 1.20 | 2.60 | 49% | 0.74 (6%) | 25.3 / 16.5 | 0.53 | 1.19 / 8.50 / 6.91 |
| t4 | all | 31.8 | 13.1 | 14.69 | 1.16 | 2.77 | 51% | 0.79 (6%) | 27.6 / 17.2 | 0.52 | 1.04 / 8.20 / 6.33 |

### Tier-2 keeps and plays by round (per game, both seats; tier 1 in brackets)

| lever | d | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| base | all | 0.74 / 0.83 (3.0 / 2.0) | 0.98 / 0.56 (1.8 / 2.1) | 1.03 / 0.61 (1.5 / 3.4) | 1.49 / 0.90 (1.5 / 3.6) | 2.10 / 1.16 (1.3 / 3.7) | 2.42 / 1.32 (1.0 / 4.2) | 2.52 / 1.29 (1.0 / 4.0) | 2.22 / 1.01 (0.6 / 3.3) | 1.49 / 0.64 (0.3 / 1.8) | 1.08 / 0.39 (0.2 / 0.9) |
| pr2 | all | 0.59 / 0.83 (2.5 / 2.2) | 0.62 / 0.51 (1.2 / 2.1) | 0.70 / 0.53 (1.0 / 3.4) | 1.07 / 0.70 (1.1 / 3.7) | 1.74 / 0.93 (1.0 / 3.9) | 2.00 / 1.16 (0.9 / 4.2) | 2.26 / 1.16 (0.8 / 3.8) | 2.14 / 1.03 (0.6 / 2.8) | 1.49 / 0.67 (0.4 / 2.1) | 1.12 / 0.43 (0.2 / 1.3) |
| pr3 | all | 0.68 / 0.85 (2.8 / 2.1) | 0.80 / 0.52 (1.4 / 2.1) | 0.88 / 0.58 (1.4 / 3.3) | 1.40 / 0.84 (1.2 / 3.6) | 1.96 / 1.08 (1.1 / 3.8) | 2.32 / 1.26 (1.0 / 4.3) | 2.46 / 1.28 (0.8 / 4.1) | 2.16 / 1.02 (0.7 / 3.3) | 1.57 / 0.67 (0.3 / 2.0) | 1.12 / 0.40 (0.2 / 1.0) |
| s6 | all | 0.74 / 0.82 (3.0 / 2.0) | 1.01 / 0.59 (1.8 / 2.1) | 1.04 / 0.63 (1.6 / 3.3) | 1.53 / 0.91 (1.5 / 3.6) | 2.06 / 1.16 (1.2 / 3.6) | 2.43 / 1.35 (1.0 / 4.1) | 2.53 / 1.31 (0.9 / 4.0) | 2.17 / 1.00 (0.6 / 3.3) | 1.46 / 0.61 (0.3 / 1.8) | 1.08 / 0.39 (0.2 / 0.9) |
| s6deal5 | all | 0.72 / 0.84 (3.5 / 2.0) | 1.03 / 0.57 (2.1 / 2.0) | 1.07 / 0.66 (1.8 / 3.5) | 1.77 / 0.95 (1.5 / 3.6) | 2.30 / 1.25 (1.4 / 3.8) | 2.62 / 1.41 (1.1 / 4.8) | 2.66 / 1.33 (0.9 / 4.3) | 2.39 / 1.13 (0.7 / 3.2) | 1.82 / 0.72 (0.3 / 1.9) | 1.32 / 0.50 (0.2 / 1.0) |
| s6t4deal5 | all | 0.72 / 0.85 (3.5 / 2.0) | 1.37 / 0.70 (1.6 / 2.1) | 1.59 / 0.86 (1.5 / 3.1) | 2.82 / 1.40 (1.5 / 4.4) | 3.26 / 1.52 (1.5 / 4.2) | 3.30 / 1.49 (1.4 / 4.4) | 3.36 / 1.46 (1.0 / 3.9) | 2.52 / 1.02 (0.6 / 2.8) | 1.83 / 0.74 (0.2 / 1.5) | 1.33 / 0.49 (0.1 / 0.9) |
| t3 | all | 0.74 / 0.83 (3.0 / 2.0) | 1.10 / 0.59 (1.7 / 2.1) | 1.18 / 0.66 (1.2 / 2.9) | 1.90 / 1.11 (1.3 / 3.7) | 2.54 / 1.20 (1.2 / 3.8) | 2.68 / 1.39 (1.1 / 3.8) | 2.51 / 1.18 (1.0 / 3.8) | 2.23 / 1.04 (0.7 / 2.9) | 1.54 / 0.62 (0.3 / 1.7) | 1.05 / 0.39 (0.2 / 1.0) |
| t4 | all | 0.74 / 0.83 (3.0 / 2.0) | 1.22 / 0.64 (1.5 / 2.1) | 1.37 / 0.78 (1.2 / 3.1) | 2.47 / 1.26 (1.3 / 3.6) | 2.78 / 1.34 (1.4 / 3.8) | 2.92 / 1.35 (1.1 / 4.2) | 2.73 / 1.27 (0.9 / 3.6) | 2.28 / 1.04 (0.7 / 2.8) | 1.55 / 0.62 (0.3 / 1.7) | 1.12 / 0.41 (0.1 / 1.2) |

### Claims and chips (per game, both seats; held a seat-round)

| lever | d | factory claims | chips held | seat-rounds at 0 | Mint | Scrap | chips spent | spent on (chips a game) | chip converter uses (by country) | idle (a seat holding chips could) |
|---|---|---|---|---|---|---|---|---|---|---|
| base | all | 0.49 | 0.49 | 89% | 2.48 | 0.43 | 8.08 | own:power:Procure 2.94, own:convert:1:chips 2.59, own:standard:station 2.51, own:use:parts_exchange 0.03, phase:threats.execute 0.01, own:play:orbital_mass_driver 0.01 | 1: 1.30 | 0.88 |
| pr2 | all | 0.45 | 0.39 | 89% | 2.76 | 0.40 | 8.38 | own:power:Procure 3.19, own:convert:1:chips 2.64, own:standard:station 2.53, own:use:parts_exchange 0.01, own:play:orbital_mass_driver 0.01 | 1: 1.32 | 0.88 |
| pr3 | all | 0.52 | 0.46 | 88% | 2.66 | 0.44 | 8.40 | own:power:Procure 3.07, own:standard:station 2.67, own:convert:1:chips 2.63, own:use:parts_exchange 0.01, own:play:orbital_mass_driver 0.01 | 1: 1.32 | 0.96 |
| s6 | all | 0.48 | 0.49 | 88% | 2.51 | 0.42 | 7.99 | own:power:Procure 2.95, own:convert:1:chips 2.53, own:standard:station 2.46, own:use:parts_exchange 0.03, phase:threats.execute 0.01, own:play:orbital_mass_driver 0.01 | 1: 1.27 | 0.89 |
| s6deal5 | all | 0.49 | 0.55 | 87% | 2.49 | 0.48 | 8.34 | own:power:Procure 2.89, own:standard:station 2.75, own:convert:1:chips 2.66, own:use:parts_exchange 0.02, phase:threats.execute 0.01, own:play:orbital_mass_driver 0.01 | 1: 1.33 | 0.98 |
| s6t4deal5 | all | 0.53 | 0.63 | 87% | 2.42 | 0.52 | 7.91 | own:power:Procure 2.78, own:standard:station 2.75, own:convert:1:chips 2.35, own:play:orbital_mass_driver 0.02, phase:threats.execute 0.01 | 1: 1.18 | 1.16 |
| t3 | all | 0.47 | 0.41 | 89% | 2.57 | 0.39 | 7.68 | own:power:Procure 2.99, own:standard:station 2.54, own:convert:1:chips 2.12, phase:threats.execute 0.02, own:play:orbital_mass_driver 0.01 | 1: 1.06 | 0.75 |
| t4 | all | 0.42 | 0.42 | 89% | 2.59 | 0.44 | 7.71 | own:power:Procure 2.98, own:standard:station 2.42, own:convert:1:chips 2.28, phase:threats.execute 0.01, own:play:orbital_mass_driver 0.01, own:use:parts_exchange 0.01 | 1: 1.14 | 0.80 |

### Insight spent a seat-round by board (the game rows: keeps, plays, powers, counters, standard)

| lever | d | Builder | Protector | Researcher |
|---|---|---|---|---|
| base | all | 4.22 | 4.69 | 4.02 |
| pr2 | all | 3.58 | 4.03 | 3.97 |
| pr3 | all | 3.93 | 4.44 | 3.92 |
| s6 | all | 4.25 | 4.62 | 4.05 |
| s6deal5 | all | 4.40 | 4.96 | 4.24 |
| s6t4deal5 | all | 4.78 | 5.22 | 5.44 |
| t3 | all | 4.27 | 4.67 | 4.52 |
| t4 | all | 4.42 | 4.82 | 4.98 |

