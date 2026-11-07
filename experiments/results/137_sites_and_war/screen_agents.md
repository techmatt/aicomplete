## out/screen-agents (rules r56)

| Difficulty | Agent | Games | Win % | Rounds mean | median | p10-p90 | min-max | Ends | Unity tips | Discord tips | Unity wins before r6 | Earliest Unity win | Envoy | Unity held off | Discord off |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 55.6 ±4.1 | 7.42 | 8 | 5-10 | 5-12 | unity_deck 70, node 59, purge 10, deck_empty 5 | 5.95 | 2.68 | 0 | r7 | 0.19 | 1.14 | 1.33 |
| d0 | strategist:w.block=0,w.armor=0 | 144 | 54.2 ±4.2 | 7.38 | 8 | 5-9 | 5-12 | unity_deck 71, node 65, purge 7, deck_empty 1 | 6.02 | 3.03 | 0 | r7 | 0.34 | 1.88 | 2.22 |
| d0 | strategist:w.freeing=0,w.station_freeing=0 | 144 | 58.3 ±4.1 | 7.62 | 8 | 5-10 | 5-12 | unity_deck 77, node 59, purge 7, deck_empty 1 | 6.12 | 2.85 | 0 | r7 | 0.46 | 1.79 | 2.25 |
| d1 | strategist | 144 | 42.4 ±4.1 | 7.28 | 7 | 5-9 | 5-12 | node 80, unity_deck 56, purge 5, deck_empty 2, discord_deck 1 | 5.25 | 3.33 | 0 | r8 | 0.13 | 0.88 | 1.01 |
| d1 | strategist:w.block=0,w.armor=0 | 144 | 36.8 ±4.0 | 6.88 | 7 | 5-9 | 5-11 | node 88, unity_deck 43, purge 10, discord_deck 3 | 4.86 | 3.46 | 0 | r7 | 0.19 | 1.07 | 1.26 |
| d1 | strategist:w.freeing=0,w.station_freeing=0 | 144 | 44.4 ±4.1 | 7.24 | 7 | 5-10 | 5-11 | node 78, unity_deck 58, purge 6, discord_deck 2 | 5.37 | 3.42 | 0 | r7 | 0.40 | 1.02 | 1.42 |
| d2 | strategist | 144 | 28.5 ±3.8 | 6.13 | 5 | 4-9 | 4-12 | node 102, unity_deck 36, purge 5, deck_empty 1 | 3.81 | 2.60 | 0 | r7 | 0.12 | 0.72 | 0.84 |
| d2 | strategist:w.block=0,w.armor=0 | 144 | 23.6 ±3.5 | 5.92 | 5 | 4-9 | 4-11 | node 110, unity_deck 31, purge 3 | 3.34 | 2.74 | 0 | r7 | 0.16 | 0.67 | 0.83 |
| d2 | strategist:w.freeing=0,w.station_freeing=0 | 144 | 25.0 ±3.6 | 6.00 | 5 | 4-9 | 4-10 | node 106, unity_deck 33, purge 3, discord_deck 2 | 3.56 | 2.76 | 0 | r8 | 0.22 | 0.61 | 0.83 |

## out/screen-r55 (rules r55)

| Difficulty | Agent | Games | Win % | Rounds mean | median | p10-p90 | min-max | Ends | Unity tips | Discord tips | Unity wins before r6 | Earliest Unity win | Envoy | Unity held off | Discord off |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 75.0 ±3.6 | 8.28 | 8 | 5-11 | 5-12 | unity_deck 58, purge 50, node 28, deck_empty 7, discord_deck 1 | 6.83 | 3.03 | 0 | r7 | 1.38 | 1.64 | 3.02 |
| d1 | strategist | 144 | 63.2 ±4.0 | 7.71 | 8 | 5-10 | 5-12 | node 48, unity_deck 48, purge 43, deck_empty 3, discord_deck 2 | 5.95 | 3.35 | 0 | r8 | 1.38 | 1.01 | 2.39 |
| d2 | strategist | 144 | 56.9 ±4.1 | 7.17 | 7.5 | 4-10 | 4-12 | node 59, unity_deck 46, purge 36, discord_deck 2, deck_empty 1 | 5.47 | 3.19 | 0 | r8 | 0.94 | 0.85 | 1.80 |

Paired NEW - OLD (OLD agent `strategist`), same pair and seed:

| Difficulty | NEW agent | Games | Win gap (points) | Rounds gap |
|---|---|---|---|---|
| d0 | strategist | 144 | -19.4 ±4.9 | -0.85 ±0.18 |
| d0 | strategist:w.block=0,w.armor=0 | 144 | -20.8 ±4.9 | -0.90 ±0.18 |
| d0 | strategist:w.freeing=0,w.station_freeing=0 | 144 | -16.7 ±4.9 | -0.66 ±0.19 |
| d1 | strategist | 144 | -20.8 ±4.9 | -0.42 ±0.22 |
| d1 | strategist:w.block=0,w.armor=0 | 144 | -26.4 ±4.7 | -0.83 ±0.18 |
| d1 | strategist:w.freeing=0,w.station_freeing=0 | 144 | -18.8 ±5.3 | -0.47 ±0.20 |
| d2 | strategist | 144 | -28.5 ±4.9 | -1.04 ±0.20 |
| d2 | strategist:w.block=0,w.armor=0 | 144 | -33.3 ±4.9 | -1.26 ±0.20 |
| d2 | strategist:w.freeing=0,w.station_freeing=0 | 144 | -31.9 ±4.8 | -1.17 ±0.21 |

NEW agents on the same seeds: strategist:w.block=0,w.armor=0 - strategist:

| Difficulty | Win gap | Rounds gap | Node losses (on / off) |
|---|---|---|---|
| d0 | -1.4 ±4.8 | -0.04 | 65 / 59 |
| d1 | -5.6 ±4.6 | -0.41 | 88 / 80 |
| d2 | -4.9 ±4.0 | -0.22 | 110 / 102 |

### out/screen-agents: war, sites and armor a game

| Difficulty | Agent | Games | war fights | factories destroyed in war | Discord from war | tip troop factories | corp troops made | claim city | claim station | claim factory | aligned (r55 align) | owned st/fa (r55 claim) | liberate city | liberate station | liberate factory | liberated cities (any route) | r55 freed st/fa | blocks | blocked by adversary | armor used | base ambassador used | station compute |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 4.66 | 1.94 | 1.94 | 5.45 | 11.08 | 0.02 | 0.40 | 0.16 | 0.00 | 0.00 | 0.58 | 0.34 | 0.77 | 2.08 | 0.00 | 7.44 | 12.47 | 0.47 | 0.56 | 10.22 |
| d0 | strategist:w.block=0,w.armor=0 | 144 | 4.96 | 2.00 | 2.00 | 6.62 | 12.12 | 0.06 | 0.56 | 0.15 | 0.00 | 0.00 | 0.69 | 0.31 | 0.85 | 2.16 | 0.00 | 1.30 | 12.65 | 0.15 | 0.68 | 10.85 |
| d0 | strategist:w.freeing=0,w.station_freeing=0 | 144 | 5.05 | 1.97 | 1.97 | 5.10 | 11.65 | 0.03 | 0.41 | 0.24 | 0.00 | 0.00 | 0.46 | 0.24 | 0.69 | 2.03 | 0.00 | 7.73 | 13.05 | 0.61 | 0.65 | 10.49 |
| d1 | strategist | 144 | 4.81 | 2.10 | 2.10 | 5.67 | 11.99 | 0.01 | 0.32 | 0.09 | 0.00 | 0.00 | 0.58 | 0.24 | 0.67 | 1.79 | 0.00 | 5.70 | 14.10 | 0.36 | 0.62 | 10.72 |
| d1 | strategist:w.block=0,w.armor=0 | 144 | 4.58 | 1.96 | 1.96 | 6.51 | 11.99 | 0.03 | 0.48 | 0.08 | 0.00 | 0.00 | 0.63 | 0.28 | 0.71 | 1.57 | 0.00 | 1.10 | 14.80 | 0.11 | 0.72 | 10.49 |
| d1 | strategist:w.freeing=0,w.station_freeing=0 | 144 | 4.81 | 1.96 | 1.96 | 5.38 | 12.03 | 0.02 | 0.32 | 0.07 | 0.00 | 0.00 | 0.36 | 0.19 | 0.63 | 1.79 | 0.00 | 5.40 | 15.31 | 0.44 | 0.64 | 10.90 |
| d2 | strategist | 144 | 3.73 | 1.68 | 1.68 | 4.12 | 9.38 | 0.01 | 0.28 | 0.05 | 0.00 | 0.00 | 0.42 | 0.13 | 0.44 | 1.22 | 0.00 | 4.28 | 11.15 | 0.33 | 0.38 | 9.03 |
| d2 | strategist:w.block=0,w.armor=0 | 144 | 3.66 | 1.62 | 1.62 | 4.60 | 9.38 | 0.03 | 0.43 | 0.03 | 0.00 | 0.00 | 0.39 | 0.20 | 0.45 | 1.06 | 0.00 | 0.78 | 11.90 | 0.04 | 0.45 | 8.97 |
| d2 | strategist:w.freeing=0,w.station_freeing=0 | 144 | 3.67 | 1.78 | 1.78 | 4.25 | 9.21 | 0.01 | 0.34 | 0.05 | 0.00 | 0.00 | 0.22 | 0.17 | 0.36 | 1.06 | 0.00 | 3.81 | 12.19 | 0.36 | 0.43 | 9.02 |

### out/screen-r55: war, sites and armor a game

| Difficulty | Agent | Games | war fights | factories destroyed in war | Discord from war | tip troop factories | corp troops made | claim city | claim station | claim factory | aligned (r55 align) | owned st/fa (r55 claim) | liberate city | liberate station | liberate factory | liberated cities (any route) | r55 freed st/fa | blocks | blocked by adversary | armor used | base ambassador used | station compute |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 0.00 | 1.78 | 0.00 | 0.00 | 18.58 | 0.00 | 0.00 | 0.00 | 0.84 | 3.01 | 0.00 | 0.00 | 0.00 | 2.77 | 0.01 | 0.00 | 0.00 | 0.47 | 0.00 | 0.00 |
| d1 | strategist | 144 | 0.00 | 1.81 | 0.00 | 0.00 | 16.73 | 0.00 | 0.00 | 0.00 | 0.61 | 2.53 | 0.00 | 0.00 | 0.00 | 2.40 | 0.01 | 0.00 | 0.00 | 0.47 | 0.00 | 0.00 |
| d2 | strategist | 144 | 0.00 | 1.63 | 0.00 | 0.00 | 15.42 | 0.00 | 0.00 | 0.00 | 0.51 | 2.31 | 0.00 | 0.00 | 0.00 | 2.06 | 0.01 | 0.00 | 0.00 | 0.35 | 0.00 | 0.00 |

