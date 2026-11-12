# 199 fresh read: the keep plan, insight by board, chips (strategist)

r87's final build on 44479-44622 (d3 at compute 22; its d0-d2 games are the first read's: 0 of 432 differ) against r86 (with 199's stats-only counters patched in, `out/r86`) on the same seeds; 1728 games a side. Read by `plan_read.py` (definitions in its docstring). A dead keep is "never had" if it was never playable at the start or end of a play phase; a kept card sold or discarded unplayed counts when it leaves.

## Pooled

### Insight a seat-round, by board (to keeps / to plays / unspent at the action phase's end / bound)

| lever | d | games | win % | gap ± SE | Builder keep / play / unsp / bound | Protector | Researcher | Focus a Builder game | carried (Patience) / Synthesis a Researcher game |
|---|---|---|---|---|---|---|---|---|---|
| r86 | all | 1728 | 68.7 | - | 2.25 / 1.67 / 4.9 / 12% | 2.53 / 1.63 / 6.0 / 10% | 3.09 / 1.53 / 14.6 / 10% | 0.00 | 106.3 / 0.00 |
| r87 | all | 1728 | 68.3 | -0.3 ± 1.2 | 2.03 / 1.58 / 3.3 / 18% | 2.33 / 1.55 / 3.8 / 20% | 1.95 / 1.55 / 5.4 / 19% | 0.04 | 39.3 / 0.00 |

### The keep plan (per game, both seats)

| lever | d | kept | played from keeps | dead: never had | dead: held back | dead in the last round | never-had share | late plays (share of kept plays) | late value: reasoned / printed | mean wait | passed: short of insight / unaffordable / chosen (a seat-game) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r86 | all | 34.8 | 13.9 | 5.40 | 12.56 | 3.04 | 17% | 0.86 (6%) | 28.7 / 18.4 | 0.53 | 1.05 / 7.70 / 5.34 |
| r87 | all | 28.6 | 12.5 | 3.01 | 10.75 | 2.35 | 11% | 0.64 (5%) | 28.0 / 18.4 | 0.47 | 1.72 / 8.03 / 7.34 |

### Tier-2 keeps and plays by round (per game, both seats; tier 1 in brackets)

| lever | d | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r86 | all | 0.85 / 0.84 (2.8 / 2.4) | 1.39 / 0.74 (1.4 / 2.1) | 1.93 / 0.94 (1.5 / 2.9) | 2.90 / 1.26 (1.5 / 3.6) | 2.97 / 1.29 (1.4 / 3.8) | 2.90 / 1.30 (1.3 / 3.8) | 2.80 / 1.28 (1.0 / 3.7) | 2.50 / 1.03 (0.7 / 2.9) | 1.86 / 0.75 (0.4 / 1.8) | 1.31 / 0.49 (0.2 / 1.0) |
| r87 | all | 0.86 / 0.84 (2.8 / 2.4) | 1.10 / 0.61 (1.9 / 2.1) | 1.22 / 0.80 (1.5 / 3.3) | 1.68 / 0.94 (1.4 / 3.6) | 1.96 / 1.09 (1.2 / 4.0) | 2.14 / 1.13 (1.1 / 4.3) | 2.20 / 1.12 (0.8 / 3.8) | 1.94 / 0.93 (0.6 / 3.1) | 1.52 / 0.63 (0.3 / 1.9) | 1.12 / 0.44 (0.2 / 1.1) |

### Claims and chips (per game, both seats; held a seat-round)

| lever | d | factory claims | chips held | seat-rounds at 0 | Mint | Scrap | chips spent | spent on (chips a game) | chip converter uses (by country) | idle (a seat holding chips could) |
|---|---|---|---|---|---|---|---|---|---|---|
| r86 | all | 0.00 | 0.60 | 85% | 2.51 | 0.45 | 5.80 | own:standard:station 2.89, own:power:Procure 2.86, own:play:orbital_mass_driver 0.03, own:use:parts_exchange 0.01, phase:threats.execute 0.01, own:power:Rush Build 0.00 | - | 0.00 |
| r87 | all | 0.49 | 0.50 | 88% | 2.58 | 0.50 | 8.31 | own:power:Procure 2.91, own:standard:station 2.71, own:convert:1:chips 2.66, own:play:orbital_mass_driver 0.02, phase:threats.execute 0.01, own:use:parts_exchange 0.01 | 1: 1.33 | 0.91 |

### Insight spent a seat-round by board (the game rows: keeps, plays, powers, counters, standard)

| lever | d | Builder | Protector | Researcher |
|---|---|---|---|---|
| r86 | all | 4.62 | 5.09 | 5.83 |
| r87 | all | 4.30 | 4.81 | 4.18 |

## By difficulty

### Insight a seat-round, by board (to keeps / to plays / unspent at the action phase's end / bound)

| lever | d | games | win % | gap ± SE | Builder keep / play / unsp / bound | Protector | Researcher | Focus a Builder game | carried (Patience) / Synthesis a Researcher game |
|---|---|---|---|---|---|---|---|---|---|
| r86 | 0 | 432 | 88.7 | - | 2.28 / 1.88 / 5.7 / 11% | 2.54 / 1.85 / 6.9 / 9% | 3.10 / 1.80 / 18.0 / 9% | 0.00 | 155.5 / 0.00 |
| r87 | 0 | 432 | 91.7 | +3.0 ± 2.0 | 2.01 / 1.75 / 4.0 / 16% | 2.24 / 1.70 / 4.7 / 19% | 2.00 / 1.72 / 6.7 / 16% | 0.07 | 57.1 / 0.00 |
| r86 | 1 | 432 | 71.5 | - | 2.25 / 1.67 / 5.0 / 11% | 2.53 / 1.62 / 6.0 / 10% | 3.09 / 1.48 / 15.5 / 10% | 0.00 | 119.1 / 0.00 |
| r87 | 1 | 432 | 74.3 | +2.8 ± 2.7 | 1.98 / 1.61 / 3.4 / 17% | 2.35 / 1.61 / 3.9 / 21% | 1.96 / 1.54 / 5.6 / 19% | 0.04 | 42.4 / 0.00 |
| r86 | 2 | 432 | 63.2 | - | 2.21 / 1.53 / 4.4 / 12% | 2.51 / 1.50 / 5.5 / 10% | 3.06 / 1.39 / 12.1 / 11% | 0.00 | 79.2 / 0.00 |
| r87 | 2 | 432 | 62.3 | -0.9 ± 2.4 | 2.03 / 1.49 / 2.9 / 19% | 2.37 / 1.43 / 3.3 / 20% | 1.90 / 1.46 / 4.7 / 20% | 0.03 | 32.2 / 0.00 |
| r86 | 3 | 432 | 51.4 | - | 2.22 / 1.52 / 4.2 / 12% | 2.51 / 1.47 / 5.2 / 11% | 3.11 / 1.39 / 11.4 / 11% | 0.00 | 71.6 / 0.00 |
| r87 | 3 | 432 | 45.1 | -6.2 ± 2.7 | 2.09 / 1.41 / 2.6 / 20% | 2.37 / 1.38 / 3.1 / 21% | 1.92 / 1.43 / 4.0 / 21% | 0.03 | 25.6 / 0.00 |

### The keep plan (per game, both seats)

| lever | d | kept | played from keeps | dead: never had | dead: held back | dead in the last round | never-had share | late plays (share of kept plays) | late value: reasoned / printed | mean wait | passed: short of insight / unaffordable / chosen (a seat-game) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r86 | 0 | 43.0 | 16.3 | 5.49 | 17.19 | 3.94 | 14% | 1.40 (9%) | 27.0 / 16.8 | 0.66 | 1.09 / 8.10 / 6.70 |
| r87 | 0 | 34.3 | 13.7 | 3.35 | 14.04 | 3.25 | 11% | 0.94 (7%) | 27.6 / 17.7 | 0.59 | 1.83 / 8.38 / 9.51 |
| r86 | 1 | 36.2 | 14.6 | 5.82 | 12.79 | 2.97 | 18% | 0.99 (7%) | 29.8 / 19.1 | 0.55 | 1.10 / 7.87 / 5.54 |
| r87 | 1 | 29.7 | 13.5 | 3.04 | 10.80 | 2.35 | 11% | 0.74 (5%) | 28.2 / 18.4 | 0.50 | 1.78 / 8.20 / 7.62 |
| r86 | 2 | 30.9 | 12.5 | 5.17 | 10.35 | 2.95 | 18% | 0.59 (5%) | 29.9 / 19.8 | 0.44 | 1.00 / 7.47 / 4.70 |
| r87 | 2 | 26.3 | 11.7 | 2.81 | 9.47 | 2.31 | 12% | 0.49 (4%) | 29.1 / 19.7 | 0.40 | 1.61 / 7.90 / 6.53 |
| r86 | 3 | 29.3 | 12.0 | 5.11 | 9.89 | 2.30 | 19% | 0.46 (4%) | 29.7 / 19.9 | 0.41 | 1.00 / 7.37 / 4.42 |
| r87 | 3 | 24.2 | 11.2 | 2.85 | 8.69 | 1.50 | 13% | 0.39 (3%) | 27.2 / 18.6 | 0.37 | 1.67 / 7.63 / 5.71 |

### Tier-2 keeps and plays by round (per game, both seats; tier 1 in brackets)

| lever | d | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r86 | 0 | 0.85 / 0.81 (2.9 / 2.4) | 1.40 / 0.76 (1.3 / 2.2) | 1.92 / 0.93 (1.5 / 3.2) | 2.89 / 1.23 (1.6 / 4.3) | 3.02 / 1.25 (1.4 / 4.7) | 3.13 / 1.36 (1.3 / 5.3) | 3.56 / 1.54 (1.1 / 5.6) | 3.83 / 1.35 (0.9 / 4.9) | 3.33 / 1.16 (0.6 / 3.2) | 2.60 / 0.80 (0.3 / 2.0) |
| r87 | 0 | 0.85 / 0.81 (2.9 / 2.4) | 1.09 / 0.60 (1.8 / 2.1) | 1.17 / 0.77 (1.6 / 3.6) | 1.59 / 0.85 (1.5 / 4.2) | 1.94 / 1.03 (1.1 / 5.2) | 2.21 / 1.07 (1.1 / 5.8) | 2.62 / 1.25 (0.9 / 5.4) | 2.77 / 1.07 (0.6 / 4.7) | 2.62 / 0.89 (0.4 / 3.2) | 2.19 / 0.72 (0.2 / 1.9) |
| r86 | 1 | 0.88 / 0.83 (2.8 / 2.4) | 1.41 / 0.78 (1.4 / 2.1) | 1.95 / 0.99 (1.5 / 2.8) | 2.98 / 1.28 (1.4 / 3.7) | 2.99 / 1.30 (1.4 / 4.0) | 2.91 / 1.30 (1.2 / 3.8) | 2.85 / 1.28 (1.1 / 3.5) | 2.46 / 1.09 (0.7 / 2.9) | 2.23 / 1.00 (0.5 / 2.4) | 1.84 / 0.78 (0.3 / 1.4) |
| r87 | 1 | 0.88 / 0.83 (2.8 / 2.5) | 1.12 / 0.62 (1.9 / 2.2) | 1.17 / 0.83 (1.6 / 3.5) | 1.74 / 0.95 (1.3 / 3.8) | 1.94 / 1.15 (1.2 / 4.0) | 2.17 / 1.16 (1.0 / 4.5) | 2.20 / 1.17 (0.8 / 3.9) | 1.98 / 1.06 (0.6 / 3.3) | 1.83 / 0.88 (0.4 / 2.4) | 1.59 / 0.70 (0.2 / 1.7) |
| r86 | 2 | 0.83 / 0.86 (2.8 / 2.5) | 1.35 / 0.70 (1.4 / 2.0) | 1.92 / 0.94 (1.4 / 2.7) | 2.84 / 1.21 (1.4 / 3.2) | 2.96 / 1.31 (1.4 / 3.1) | 2.73 / 1.24 (1.3 / 3.1) | 2.37 / 1.12 (1.0 / 2.9) | 1.79 / 0.80 (0.6 / 2.0) | 1.31 / 0.56 (0.4 / 1.2) | 0.81 / 0.37 (0.2 / 0.7) |
| r87 | 2 | 0.83 / 0.85 (2.8 / 2.5) | 1.05 / 0.60 (1.9 / 2.0) | 1.26 / 0.79 (1.4 / 3.1) | 1.63 / 0.97 (1.4 / 3.3) | 1.98 / 1.05 (1.2 / 3.6) | 2.13 / 1.09 (1.1 / 3.6) | 2.01 / 0.99 (0.9 / 3.2) | 1.55 / 0.80 (0.6 / 2.2) | 1.20 / 0.58 (0.4 / 1.4) | 0.72 / 0.33 (0.2 / 0.8) |
| r86 | 3 | 0.84 / 0.86 (2.8 / 2.4) | 1.40 / 0.73 (1.4 / 1.9) | 1.92 / 0.90 (1.4 / 2.7) | 2.88 / 1.33 (1.5 / 3.3) | 2.92 / 1.29 (1.4 / 3.5) | 2.83 / 1.31 (1.2 / 3.2) | 2.41 / 1.17 (0.9 / 2.7) | 1.93 / 0.90 (0.7 / 2.0) | 0.56 / 0.26 (0.2 / 0.4) | 0.00 / 0.00 (0.0 / 0.0) |
| r87 | 3 | 0.87 / 0.88 (2.8 / 2.4) | 1.13 / 0.62 (1.9 / 2.0) | 1.27 / 0.80 (1.5 / 3.2) | 1.75 / 1.00 (1.4 / 3.2) | 1.98 / 1.14 (1.2 / 3.4) | 2.05 / 1.20 (1.1 / 3.2) | 1.97 / 1.08 (0.8 / 2.8) | 1.44 / 0.78 (0.6 / 2.2) | 0.43 / 0.18 (0.1 / 0.5) | 0.00 / 0.00 (0.0 / 0.0) |

### Claims and chips (per game, both seats; held a seat-round)

| lever | d | factory claims | chips held | seat-rounds at 0 | Mint | Scrap | chips spent | spent on (chips a game) | chip converter uses (by country) | idle (a seat holding chips could) |
|---|---|---|---|---|---|---|---|---|---|---|
| r86 | 0 | 0.00 | 1.07 | 77% | 2.97 | 0.71 | 7.67 | own:standard:station 4.36, own:power:Procure 3.23, own:play:orbital_mass_driver 0.04, own:use:parts_exchange 0.03, phase:threats.execute 0.01, own:power:Rush Build 0.00 | - | 0.00 |
| r87 | 0 | 1.39 | 1.00 | 82% | 3.02 | 0.76 | 11.17 | own:convert:1:chips 4.06, own:standard:station 3.87, own:power:Procure 3.21, own:play:orbital_mass_driver 0.03, own:use:parts_exchange 0.01 | 1: 2.03 | 2.15 |
| r86 | 1 | 0.00 | 0.67 | 85% | 2.53 | 0.53 | 5.87 | own:power:Procure 2.97, own:standard:station 2.85, own:play:orbital_mass_driver 0.03, own:use:parts_exchange 0.01, phase:threats.execute 0.00 | - | 0.00 |
| r87 | 1 | 0.37 | 0.52 | 87% | 2.59 | 0.59 | 8.61 | own:power:Procure 2.99, own:convert:1:chips 2.85, own:standard:station 2.73, own:play:orbital_mass_driver 0.02, phase:threats.execute 0.01, own:use:parts_exchange 0.00 | 1: 1.43 | 1.03 |
| r86 | 2 | 0.00 | 0.29 | 89% | 2.30 | 0.32 | 4.90 | own:power:Procure 2.65, own:standard:station 2.21, own:use:parts_exchange 0.01, own:play:orbital_mass_driver 0.01, phase:threats.execute 0.01 | - | 0.00 |
| r87 | 2 | 0.12 | 0.20 | 92% | 2.51 | 0.37 | 7.11 | own:power:Procure 2.85, own:standard:station 2.19, own:convert:1:chips 2.03, own:use:parts_exchange 0.02, own:play:orbital_mass_driver 0.02, phase:threats.execute 0.00 | 1: 1.01 | 0.27 |
| r86 | 3 | 0.00 | 0.21 | 90% | 2.23 | 0.23 | 4.76 | own:power:Procure 2.58, own:standard:station 2.15, phase:threats.execute 0.02, own:play:orbital_mass_driver 0.02 | - | 0.00 |
| r87 | 3 | 0.07 | 0.12 | 93% | 2.20 | 0.29 | 6.36 | own:power:Procure 2.60, own:standard:station 2.05, own:convert:1:chips 1.69, phase:threats.execute 0.02, own:play:orbital_mass_driver 0.01 | 1: 0.84 | 0.19 |

### Insight spent a seat-round by board (the game rows: keeps, plays, powers, counters, standard)

| lever | d | Builder | Protector | Researcher |
|---|---|---|---|---|
| r86 | 0 | 4.84 | 5.18 | 5.73 |
| r87 | 0 | 4.43 | 4.84 | 4.37 |
| r86 | 1 | 4.63 | 5.10 | 5.89 |
| r87 | 1 | 4.38 | 4.96 | 4.24 |
| r86 | 2 | 4.50 | 5.05 | 5.84 |
| r87 | 2 | 4.18 | 4.73 | 4.09 |
| r86 | 3 | 4.47 | 4.97 | 5.89 |
| r87 | 3 | 4.18 | 4.66 | 3.96 |

## Dead keeps still in hand at the end, by when they were kept, price and tag requirement (r87, then r86)

1728 games; dead keeps a game by rounds kept before the end (5: 5 or more), never-had in brackets

| rounds before the end | 0 | 1 | 2 | 3 | 4 | 5 |
|---|---|---|---|---|---|---|
| dead keeps a game | 2.35 (0.69) | 2.37 (0.80) | 1.78 (0.55) | 1.42 (0.38) | 1.17 (0.28) | 3.12 (0.38) |

| printed credits | 10-19 | 0-9 | 20-29 | 30+ |
|---|---|---|---|---|
| dead keeps a game | 7.72 | 1.94 | 1.87 | 0.68 |

| by tag requirement | needs | needs, never had | no needs | no needs, never had |
|---|---|---|---|---|
| dead keeps a game | 5.85 | 2.57 | 6.36 | 0.51 |

(deck: 64 of 185 cards print `needs`)

1728 games; dead keeps a game by rounds kept before the end (5: 5 or more), never-had in brackets

| rounds before the end | 0 | 1 | 2 | 3 | 4 | 5 |
|---|---|---|---|---|---|---|
| dead keeps a game | 3.04 (1.11) | 3.17 (1.27) | 2.61 (1.02) | 2.15 (0.80) | 1.73 (0.59) | 4.28 (1.01) |

| printed credits | 10-19 | 20-29 | 0-9 | 30+ |
|---|---|---|---|---|
| dead keeps a game | 10.65 | 2.52 | 2.47 | 1.35 |

| by tag requirement | needs | needs, never had | no needs | no needs, never had |
|---|---|---|---|---|
| dead keeps a game | 9.11 | 4.85 | 7.88 | 0.96 |

(deck: 64 of 185 cards print `needs`)
