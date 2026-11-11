# 176: the fresh ladder, r75 against r74 (the same seeds)

Seeds 42133-42276 (sets a, b, c of 48), the three pairs, d0-d4. Strategist on sets a-c (432 games a level), greedy on set a (144). r75: `out/r75final.exe` (md5 a83446a3…); r74: main 17d6c2b built from `git archive`, its configs. Run: `run176.sh`; specs: `specs176.py fresh 42133`.

## Strategist (`scripts/bench/ladder.py NEW OLD --measures`)

| d | r75 win % | ±SE | r74 win % | gap | ±SE (paired) | sets (r75) | band | verdict |
|---|---|---|---|---|---|---|---|---|
| d0 | **90.3** | 1.4 | 94.0 | -3.7 | 1.8 | 91.0 / 89.6 / 90.3 | 90-100 | met | (432 games)
| d1 | **77.3** | 2.0 | 69.4 | +7.9 | 2.8 | 75.7 / 78.5 / 77.8 | 70-80 | met | (432 games)
| d2 | **63.4** | 2.3 | 59.5 | +3.9 | 2.6 | 56.2 / 63.9 / 70.1 | 55-65 | met | (432 games)
| d3 | **50.0** | 2.4 | 41.7 | +8.3 | 2.7 | 48.6 / 52.1 / 49.3 | 40-50 | met | (432 games)
| d4 | **44.2** | 2.4 | 36.1 | +8.1 | 2.5 | 47.2 / 41.0 / 44.4 | - | - | (432 games)

| side | d | games | corruption nodes lost | ambassadors placed |   of them rallied | corruptions | compute made | projects revealed | revealed, share of deck | distinct revealed, share | projects resolved | resolved, share of deck | project reshuffles | rounds |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| r75 | d0 | 432 | 0.74 | 24.76 | 22.67 | 1.84 | 75.45 | 7.12 | 0.36 | 0.36 | 2.47 | 0.12 | 0.00 | 7.84 |
| r75 | d1 | 432 | 2.58 | 69.91 | 66.30 | 13.37 | 98.80 | 9.40 | 0.47 | 0.47 | 4.25 | 0.21 | 0.01 | 8.00 |
| r75 | d2 | 432 | 3.17 | 89.72 | 84.04 | 15.12 | 290.12 | 32.48 | 1.62 | 0.94 | 15.12 | 0.76 | 3.29 | 7.80 |
| r75 | d3 | 432 | 3.21 | 89.39 | 82.97 | 15.33 | 333.03 | 36.38 | 1.82 | 0.96 | 16.81 | 0.84 | 4.23 | 7.58 |
| r75 | d4 | 432 | 2.70 | 71.62 | 63.12 | 12.95 | 285.12 | 31.83 | 1.59 | 0.93 | 14.66 | 0.73 | 3.14 | 7.39 |
| r74 | d0 | 432 | 0.66 | 24.06 | 22.08 | 1.62 | 78.72 | 7.15 | 0.36 | 0.36 | 2.50 | 0.13 | 0.00 | 7.66 |
| r74 | d1 | 432 | 2.91 | 76.51 | 72.46 | 13.93 | 113.29 | 10.11 | 0.51 | 0.50 | 4.72 | 0.24 | 0.03 | 8.61 |
| r74 | d2 | 432 | 3.75 | 100.51 | 94.01 | 16.38 | 333.12 | 35.26 | 1.76 | 0.95 | 16.21 | 0.81 | 3.96 | 8.71 |
| r74 | d3 | 432 | 3.82 | 98.07 | 90.68 | 16.21 | 375.46 | 37.31 | 1.87 | 0.98 | 17.75 | 0.89 | 4.47 | 8.45 |
| r74 | d4 | 432 | 3.25 | 81.26 | 71.28 | 14.34 | 327.43 | 35.40 | 1.77 | 0.95 | 16.20 | 0.81 | 4.06 | 8.21 |

## Greedy

| d | r75 win % | ±SE | r74 win % | gap | ±SE (paired) | sets (r75) | band | verdict |
|---|---|---|---|---|---|---|---|---|
| d0 | **91.7** | 2.3 | 94.4 | -2.8 | 3.1 | 91.7 | 90-100 | met | (144 games)
| d1 | **75.0** | 3.6 | 81.9 | -6.9 | 4.4 | 75.0 | 70-80 | met | (144 games)
| d2 | **70.1** | 3.8 | 59.7 | +10.4 | 4.1 | 70.1 | 55-65 | **unmet** | (144 games)
| d3 | **54.2** | 4.2 | 40.3 | +13.9 | 4.6 | 54.2 | 40-50 | **unmet** | (144 games)
| d4 | **51.4** | 4.2 | 46.5 | +4.9 | 4.9 | 51.4 | - | - | (144 games)

## Losses by cause (`losses175.py`)

r75:

| agent | variant | d | games | win % | rounds | round cap % of losses | discord_deck % of losses | last node % of losses | node corruptions/game | nodes lost/game |
|---|---|---|---|---|---|---|---|---|---|---|
| greedy | all | 0 | 144 | 91.7 | 7.54 | 100 | 0 | 0 | 0.44 | 1.78 |
| greedy | all | 1 | 144 | 75.0 | 8.22 | 72 | 28 | 0 | 1.40 | 4.11 |
| greedy | all | 2 | 144 | 70.1 | 7.74 | 9 | 88 | 2 | 1.58 | 5.03 |
| greedy | all | 3 | 144 | 54.2 | 7.72 | 39 | 58 | 3 | 1.70 | 5.28 |
| greedy | all | 4 | 144 | 51.4 | 7.73 | 27 | 71 | 1 | 1.29 | 6.19 |
| strategist | all | 0 | 432 | 90.3 | 7.84 | 100 | 0 | 0 | 0.68 | 2.25 |
| strategist | all | 1 | 432 | 77.3 | 8.00 | 70 | 30 | 0 | 1.34 | 4.31 |
| strategist | all | 2 | 432 | 63.4 | 7.80 | 8 | 90 | 2 | 1.73 | 5.39 |
| strategist | all | 3 | 432 | 50.0 | 7.58 | 28 | 68 | 4 | 1.67 | 5.13 |
| strategist | all | 4 | 432 | 44.2 | 7.39 | 17 | 80 | 2 | 1.24 | 5.70 |

r74:

| agent | variant | d | games | win % | rounds | round cap % of losses | discord_deck % of losses | last node % of losses | node corruptions/game | nodes lost/game |
|---|---|---|---|---|---|---|---|---|---|---|
| greedy | all | 0 | 144 | 94.4 | 7.60 | 100 | 0 | 0 | 0.58 | 1.69 |
| greedy | all | 1 | 144 | 81.9 | 8.07 | 81 | 19 | 0 | 1.34 | 3.88 |
| greedy | all | 2 | 144 | 59.7 | 8.90 | 38 | 62 | 0 | 2.23 | 6.46 |
| greedy | all | 3 | 144 | 40.3 | 8.60 | 73 | 26 | 1 | 2.05 | 6.25 |
| greedy | all | 4 | 144 | 46.5 | 8.15 | 53 | 43 | 4 | 1.61 | 6.18 |
| strategist | all | 0 | 432 | 94.0 | 7.66 | 100 | 0 | 0 | 0.66 | 1.93 |
| strategist | all | 1 | 432 | 69.4 | 8.61 | 81 | 19 | 0 | 1.72 | 4.75 |
| strategist | all | 2 | 432 | 59.5 | 8.71 | 30 | 65 | 6 | 2.19 | 6.26 |
| strategist | all | 3 | 432 | 41.7 | 8.45 | 63 | 30 | 7 | 2.09 | 6.13 |
| strategist | all | 4 | 432 | 36.1 | 8.21 | 41 | 53 | 5 | 1.61 | 6.76 |

## Agent cost a decision (`pairs175.py`, the games' own clock, 6 threads, both sides on a quiet box)

| agent | side | games | ms/decision | decisions/game | rounds/game |
|---|---|---|---|---|---|
| greedy | old | 720 | 1.75 | 501 | 8.27 |
| greedy | new | 720 | 1.80 | 484 | 7.79 |
| strategist | old | 2160 | 1.69 | 491 | 8.33 |
| strategist | new | 2160 | 1.70 | 463 | 7.72 |

## Mechanic use a game (`use176.py`, all levels)

r75:
```
greedy dall (720 games)
  remove 2.55, claim 0.00, liberate 7.16, lib.city 4.00, lib.station 1.79, lib.factory 1.36, claim.city 0.00, claim.station 0.13, claim.factory 0.00, own_on_site 2.15, botnet_std 1.51, botnets_down 1.91, buy_energy 2.47, energy_bought 2.47, move 10.30, armor 0.18, harden_used 2.66, base_absorbed 0.11, aegis 0.07, declined 0.40, factory_amb 0.09, base_amb 0.07, held_off 4.02, canceled 6.94, fac.neutral 2.45, fac.player 1.19, fac.adv 2.43, cubes_conv 2.23, tips_discord 3.16, tips_unity 8.38, converter 1.22
strategist dall (2160 games)
  remove 2.54, claim 0.00, liberate 7.05, lib.city 4.15, lib.station 1.61, lib.factory 1.29, claim.city 0.00, claim.station 0.11, claim.factory 0.00, own_on_site 1.84, botnet_std 1.25, botnets_down 1.62, buy_energy 2.74, energy_bought 2.74, move 8.08, armor 0.13, harden_used 2.24, base_absorbed 0.12, aegis 0.06, declined 0.38, factory_amb 0.08, base_amb 0.08, held_off 4.59, canceled 5.64, fac.neutral 2.41, fac.player 1.22, fac.adv 2.33, cubes_conv 2.13, tips_discord 3.55, tips_unity 7.87, converter 1.03
```
r74:
```
greedy dall (720 games)
  remove 5.67, claim 0.00, liberate 6.65, lib.city 4.03, lib.station 1.73, lib.factory 0.89, claim.city 0.00, claim.station 0.16, claim.factory 0.00, own_on_site 0.00, botnet_std 0.00, botnets_down 0.92, buy_energy 0.00, energy_bought 2.73, move 11.97, armor 0.25, harden_used 2.42, base_absorbed 0.11, aegis 0.00, declined 0.00, factory_amb 0.06, base_amb 0.07, held_off 3.87, canceled 9.26, fac.neutral 0.19, fac.player 1.90, fac.adv 3.61, cubes_conv 1.99, tips_discord 2.90, tips_unity 8.07, converter 1.43
strategist dall (2160 games)
  remove 5.43, claim 0.00, liberate 6.73, lib.city 4.22, lib.station 1.60, lib.factory 0.91, claim.city 0.00, claim.station 0.12, claim.factory 0.00, own_on_site 0.00, botnet_std 0.00, botnets_down 0.81, buy_energy 0.00, energy_bought 2.48, move 11.22, armor 0.25, harden_used 2.24, base_absorbed 0.14, aegis 0.00, declined 0.00, factory_amb 0.10, base_amb 0.07, held_off 4.04, canceled 8.42, fac.neutral 0.26, fac.player 2.10, fac.adv 3.50, cubes_conv 2.01, tips_discord 3.29, tips_unity 7.90, converter 1.49
```
