## out/readF (rules r58)

| Difficulty | Agent | Games | Win % | Rounds mean | median | p10-p90 | min-max | Ends | Unity tips | Discord tips | Unity wins before r6 | Earliest Unity win | Envoy | Unity held off | Discord off |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 77.8 ±3.5 | 9.09 | 9 | 8-11 | 5-12 | unity_deck 97, node 16, purge 15, discord_deck 12, deck_empty 4 | 7.98 | 3.81 | 0 | r8 | 0.21 | 0.88 | 1.09 |
| d1 | strategist | 144 | 61.8 ±4.0 | 8.77 | 9 | 6-10 | 5-12 | unity_deck 87, node 28, discord_deck 27, purge 2 | 7.31 | 4.59 | 0 | r8 | 0.10 | 0.92 | 1.03 |
| d2 | strategist | 144 | 51.4 ±4.2 | 8.20 | 9 | 5-10 | 4-11 | unity_deck 72, node 46, discord_deck 24, purge 2 | 6.36 | 4.60 | 0 | r8 | 0.10 | 0.71 | 0.81 |

## out/readO (rules r57)

| Difficulty | Agent | Games | Win % | Rounds mean | median | p10-p90 | min-max | Ends | Unity tips | Discord tips | Unity wins before r6 | Earliest Unity win | Envoy | Unity held off | Discord off |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 91.7 ±2.3 | 8.90 | 9 | 7-11 | 5-12 | unity_deck 102, purge 30, deck_empty 7, node 5 | 8.29 | 2.34 | 0 | r8 | 0.38 | 2.16 | 2.54 |
| d1 | strategist | 144 | 75.7 ±3.6 | 8.62 | 9 | 6-10 | 5-12 | unity_deck 94, node 31, purge 15, deck_empty 3, discord_deck 1 | 7.60 | 3.14 | 0 | r8 | 0.09 | 1.09 | 1.18 |
| d2 | strategist | 144 | 52.8 ±4.2 | 7.62 | 8 | 5-10 | 4-12 | unity_deck 72, node 64, purge 4, deck_empty 2, discord_deck 2 | 6.03 | 3.72 | 0 | r7 | 0.12 | 0.79 | 0.92 |

Paired NEW - OLD (OLD agent `strategist`), same pair and seed:

| Difficulty | NEW agent | Games | Win gap (points) | Rounds gap |
|---|---|---|---|---|
| d0 | strategist | 144 | -13.9 ±4.0 | +0.19 ±0.15 |
| d1 | strategist | 144 | -13.9 ±4.6 | +0.15 ±0.16 |
| d2 | strategist | 144 | -1.4 ±5.0 | +0.58 ±0.18 |

### out/readF: war, sites and armor a game

| Difficulty | Agent | Games | war fights | factories destroyed in war | Discord from war | tip troop factories | corp troops made | claim city | claim station | claim factory | aligned (r55 align) | owned st/fa (r55 claim) | liberate city | liberate station | liberate factory | liberated cities (any route) | r55 freed st/fa | blocks | blocked by adversary | armor used | base ambassador used | station compute |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 6.59 | 1.12 | 1.12 | 10.07 | 18.35 | 0.13 | 0.07 | 0.10 | 0.00 | 0.00 | 0.99 | 1.89 | 1.19 | 2.93 | 0.00 | 13.34 | 2.94 | 0.56 | 0.32 | 18.97 |
| d1 | strategist | 144 | 6.55 | 0.84 | 0.84 | 10.52 | 19.11 | 0.08 | 0.04 | 0.06 | 0.00 | 0.00 | 0.86 | 1.94 | 1.22 | 2.64 | 0.00 | 14.11 | 1.63 | 0.53 | 0.24 | 22.15 |
| d2 | strategist | 144 | 6.01 | 0.80 | 0.80 | 9.51 | 17.94 | 0.06 | 0.04 | 0.01 | 0.00 | 0.00 | 0.78 | 1.77 | 1.12 | 2.17 | 0.00 | 12.15 | 1.65 | 0.60 | 0.17 | 21.69 |

### out/readO: war, sites and armor a game

| Difficulty | Agent | Games | war fights | factories destroyed in war | Discord from war | tip troop factories | corp troops made | claim city | claim station | claim factory | aligned (r55 align) | owned st/fa (r55 claim) | liberate city | liberate station | liberate factory | liberated cities (any route) | r55 freed st/fa | blocks | blocked by adversary | armor used | base ambassador used | station compute |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 6.01 | 0.94 | 0.94 | 7.42 | 14.73 | 0.33 | 0.89 | 0.38 | 0.00 | 0.00 | 1.17 | 1.62 | 1.06 | 3.60 | 0.00 | 17.03 | 2.00 | 0.66 | 0.38 | 15.07 |
| d1 | strategist | 144 | 6.10 | 0.94 | 0.94 | 9.19 | 16.62 | 0.15 | 0.81 | 0.24 | 0.00 | 0.00 | 0.92 | 1.83 | 1.10 | 2.90 | 0.00 | 12.63 | 2.49 | 0.74 | 0.28 | 16.51 |
| d2 | strategist | 144 | 5.40 | 0.78 | 0.78 | 8.30 | 15.51 | 0.13 | 0.35 | 0.11 | 0.00 | 0.00 | 0.85 | 1.40 | 1.04 | 2.28 | 0.00 | 13.07 | 1.65 | 0.75 | 0.13 | 18.08 |

## out/readF (rules r58)

| Difficulty | Pair | Agent | Games | Win % | Rounds | Ends |
|---|---|---|---|---|---|---|
| d1 | bp | strategist | 48 | 64.6 ±6.9 | 8.88 | unity_deck 31, node 9, discord_deck 8 |
| d1 | br | strategist | 48 | 50.0 ±7.2 | 8.58 | unity_deck 24, discord_deck 12, node 12 |
| d2 | bp | strategist | 48 | 58.3 ±7.1 | 8.48 | unity_deck 28, node 11, discord_deck 9 |
| d1 | pr | strategist | 48 | 70.8 ±6.6 | 8.85 | unity_deck 32, node 7, discord_deck 7, purge 2 |
| d0 | bp | strategist | 48 | 83.3 ±5.4 | 9.21 | unity_deck 36, node 4, purge 4, discord_deck 3, deck_empty 1 |
| d2 | pr | strategist | 48 | 58.3 ±7.1 | 8.38 | unity_deck 26, node 11, discord_deck 9, purge 2 |
| d2 | br | strategist | 48 | 37.5 ±7.0 | 7.75 | node 24, unity_deck 18, discord_deck 6 |
| d0 | br | strategist | 48 | 66.7 ±6.8 | 9.23 | unity_deck 27, discord_deck 7, node 7, purge 5, deck_empty 2 |
| d0 | pr | strategist | 48 | 83.3 ±5.4 | 8.83 | unity_deck 34, purge 6, node 5, discord_deck 2, deck_empty 1 |

Pooled over the pairs:

| Difficulty | Agent | Games | Win % | Rounds | Losses by end | Nodes lost a game, by cause |
|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 77.8 ±3.5 | 9.09 | node 16, discord_deck 12, deck_empty 4 | corruption 2.01, threat2 1.53, twist 0.71, counter 0.67, threat1 0.60, threat3 0.51 |
| d1 | strategist | 144 | 61.8 ±4.0 | 8.77 | node 28, discord_deck 27 | corruption 1.96, threat2 1.67, counter 0.73, twist 0.65, threat1 0.62, threat3 0.52 |
| d2 | strategist | 144 | 51.4 ±4.2 | 8.20 | node 46, discord_deck 24 | corruption 2.17, threat2 2.12, threat3 1.02, counter 0.60, twist 0.60, threat1 0.49 |

Threats a game (dealt / countered / uncountered executed), and the counter kinds used a game:

| Difficulty | Agent | Dealt | Countered | Uncountered | credits | energy | insight | chips | ambassadors | research_cubes | tags | nodes | card | standing |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 32.7 | 25.6 | 4.1 | 14.96 | 1.26 | 1.09 | 0.00 | 0.05 | 0.09 | 0.67 | 0.49 | 3.06 | 3.94 |
| d1 | strategist | 32.6 | 25.2 | 4.5 | 14.90 | 0.89 | 1.10 | 0.00 | 0.01 | 0.09 | 0.72 | 0.49 | 3.10 | 3.92 |
| d2 | strategist | 31.8 | 23.3 | 5.4 | 13.85 | 0.85 | 0.95 | 0.00 | 0.02 | 0.01 | 0.63 | 0.41 | 2.71 | 3.83 |

Threats countered / uncountered per round (mean over the games that reached it):

| Difficulty | Agent | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 | r11 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 2.6/0.4 | 2.7/0.4 | 2.6/0.6 | 3.0/0.4 | 2.7/0.7 | 2.9/0.6 | 3.2/0.3 | 3.0/0.3 | 3.4/0.4 | 2.3/0.6 | 1.4/0.6 |
| d1 | strategist | 2.6/0.4 | 2.5/0.6 | 2.6/0.7 | 3.0/0.4 | 2.8/0.7 | 3.1/0.6 | 3.1/0.6 | 3.2/0.3 | 3.3/0.4 | 2.8/0.3 | 2.1/0.1 |
| d2 | strategist | 2.6/0.4 | 2.5/0.6 | 2.7/0.6 | 2.6/0.8 | 2.5/0.9 | 2.9/0.8 | 3.2/0.4 | 4.3/0.8 | 3.1/0.6 | 1.8/0.4 | 0.4/0.2 |

The new ops: resolutions a game, by difficulty and agent:

| Op | d0 strategist | d1 strategist | d2 strategist |
|---|---|---|---|
| `adversary_factories` | 0.02 | 0.01 | 0.03 |
| `adversary_factory` | 0.22 | 0.24 | 0.26 |
| `ambassador_moves` | 0.35 | 0.34 | 0.24 |
| `cleansed` | 1.42 | 1.44 | 1.10 |
| `compute_per_missing` | 0.01 | 0.00 | 0.01 |
| `corp_troops` | 0.28 | 0.33 | 0.44 |
| `corrupt` | 1.08 | 1.38 | 1.53 |
| `corrupt_city` | 0.05 | 0.03 | 0.07 |
| `create` | 26.44 | 25.89 | 24.96 |
| `create_site` | 2.77 | 2.78 | 2.47 |
| `deal_threats` | 0.62 | 0.86 | 0.88 |
| `destroy_facility` | 0.03 | 0.02 | 0.07 |
| `disable_building` | 0.01 | 0.02 | 0.06 |
| `discard` | 0.10 | 0.08 | 0.10 |
| `extra_war` | 0.20 | 0.18 | 0.28 |
| `flag` | 0.51 | 0.53 | 0.65 |
| `flip_station` | 0.05 | 0.02 | 0.08 |
| `insight_lost` | 0.38 | 0.57 | 0.56 |
| `lose_ambassadors_or` | 0.19 | 0.27 | 0.24 |
| `lose_income` | 0.19 | 0.21 | 0.20 |
| `lose_node_or` | 0.09 | 0.10 | 0.15 |
| `lose_or` | 0.15 | 0.13 | 0.14 |
| `march_now` | 0.17 | 0.16 | 0.33 |
| `occupy_factory` | 0.08 | 0.05 | 0.11 |
| `place_botnet` | 0.19 | 0.26 | 0.35 |
| `players_lose` | 0.15 | 0.17 | 0.16 |
| `requisition` | 0.10 | 0.11 | 0.10 |
| `seize_factory` | 0.03 | 0.03 | 0.08 |
| `theft` | 0.24 | 0.24 | 0.28 |
| `theft_taken` | 1.48 | 1.28 | 1.65 |
| `troops` | 0.62 | 0.69 | 1.10 |
| `troops_walled` | 0.03 | 0.03 | 0.08 |

Each resource's peak holding a game, by seat's board (mean):

| Difficulty | Agent | Board | credits | insight | energy | chips | antimatter | research_cubes | card_tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | builder | 83.6 | 12.2 | 9.9 | 6.7 | 0.7 | 1.5 | 2.3 | 4.7 |
| d0 | strategist | protector | 70.1 | 21.7 | 9.3 | 5.6 | 0.8 | 2.5 | 1.4 | 5.9 |
| d0 | strategist | researcher | 87.2 | 31.1 | 8.9 | 5.4 | 1.0 | 2.9 | 1.6 | 6.3 |
| d1 | strategist | builder | 73.1 | 11.7 | 8.3 | 6.7 | 0.3 | 1.2 | 1.8 | 4.2 |
| d1 | strategist | protector | 64.2 | 18.4 | 8.3 | 3.8 | 0.3 | 1.8 | 1.5 | 5.5 |
| d1 | strategist | researcher | 85.1 | 26.3 | 10.0 | 4.5 | 0.8 | 2.2 | 1.4 | 6.0 |
| d2 | strategist | builder | 69.8 | 11.1 | 8.1 | 5.2 | 0.4 | 0.8 | 1.5 | 4.5 |
| d2 | strategist | protector | 59.9 | 16.7 | 7.4 | 3.6 | 0.4 | 1.4 | 1.1 | 5.0 |
| d2 | strategist | researcher | 80.4 | 24.5 | 8.0 | 4.5 | 0.7 | 2.2 | 1.4 | 5.2 |



## out/readO (rules r57)

| Difficulty | Pair | Agent | Games | Win % | Rounds | Ends |
|---|---|---|---|---|---|---|
| d2 | bp | strategist | 48 | 54.2 ±7.2 | 7.90 | unity_deck 24, node 19, discord_deck 2, purge 2, deck_empty 1 |
| d0 | br | strategist | 48 | 89.6 ±4.4 | 9.17 | unity_deck 35, purge 8, node 3, deck_empty 2 |
| d2 | br | strategist | 48 | 37.5 ±7.0 | 7.02 | node 30, unity_deck 18 |
| d1 | bp | strategist | 48 | 81.2 ±5.6 | 8.77 | unity_deck 35, node 8, purge 4, deck_empty 1 |
| d1 | br | strategist | 48 | 62.5 ±7.0 | 8.38 | unity_deck 26, node 17, purge 4, deck_empty 1 |
| d0 | bp | strategist | 48 | 89.6 ±4.4 | 8.81 | unity_deck 33, purge 10, deck_empty 3, node 2 |
| d1 | pr | strategist | 48 | 83.3 ±5.4 | 8.71 | unity_deck 33, purge 7, node 6, deck_empty 1, discord_deck 1 |
| d0 | pr | strategist | 48 | 95.8 ±2.9 | 8.71 | unity_deck 34, purge 12, deck_empty 2 |
| d2 | pr | strategist | 48 | 66.7 ±6.8 | 7.94 | unity_deck 30, node 15, purge 2, deck_empty 1 |

Pooled over the pairs:

| Difficulty | Agent | Games | Win % | Rounds | Losses by end | Nodes lost a game, by cause |
|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 91.7 ±2.3 | 8.90 | deck_empty 7, node 5 | corruption 1.72, twist 0.63, counter 0.35, war 0.19, threat2 0.19, discord 0.18 |
| d1 | strategist | 144 | 75.7 ±3.6 | 8.62 | node 31, deck_empty 3, discord_deck 1 | corruption 1.65, threat2 1.07, discord 0.91, twist 0.72, counter 0.71, threat1 0.36 |
| d2 | strategist | 144 | 52.8 ±4.2 | 7.62 | node 64, deck_empty 2, discord_deck 2 | corruption 1.73, discord 1.53, threat2 1.37, twist 0.76, counter 0.49, threat1 0.37 |

Threats a game (dealt / countered / uncountered executed), and the counter kinds used a game:

| Difficulty | Agent | Dealt | Countered | Uncountered | credits | energy | insight | chips | ambassadors | research_cubes | tags | nodes | card | standing |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 20.9 | 17.7 | 1.1 | 10.44 | 1.08 | 0.72 | 0.00 | 0.03 | 0.12 | 0.29 | 0.27 | 1.93 | 2.82 |
| d1 | strategist | 29.5 | 24.0 | 2.7 | 14.65 | 1.10 | 1.00 | 0.00 | 0.03 | 0.12 | 0.50 | 0.52 | 2.53 | 3.58 |
| d2 | strategist | 27.6 | 21.7 | 3.4 | 13.22 | 1.01 | 0.82 | 0.00 | 0.05 | 0.06 | 0.39 | 0.33 | 2.13 | 3.69 |

Threats countered / uncountered per round (mean over the games that reached it):

| Difficulty | Agent | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 | r11 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 1.8/0.2 | 1.8/0.3 | 1.9/0.2 | 2.1/0.1 | 2.0/0.1 | 2.1/0.1 | 2.1/0.0 | 1.9/0.0 | 2.3/0.1 | 2.3/0.0 | 1.8/0.1 |
| d1 | strategist | 2.6/0.4 | 2.7/0.4 | 2.8/0.4 | 3.0/0.3 | 2.6/0.5 | 2.9/0.4 | 3.1/0.1 | 2.9/0.2 | 2.8/0.2 | 1.9/0.2 | 2.3/0.3 |
| d2 | strategist | 2.6/0.4 | 2.5/0.5 | 2.7/0.6 | 2.7/0.6 | 2.6/0.5 | 2.9/0.4 | 3.0/0.2 | 4.5/0.3 | 2.8/0.3 | 2.3/0.0 | 2.0/0.0 |

The new ops: resolutions a game, by difficulty and agent:

| Op | d0 strategist | d1 strategist | d2 strategist |
|---|---|---|---|
| `adversary_factories` | 0.01 | 0.02 | 0.03 |
| `adversary_factory` | 0.12 | 0.13 | 0.17 |
| `ambassador_moves` | 0.49 | 0.42 | 0.26 |
| `cleansed` | 1.54 | 1.46 | 1.01 |
| `compute_per_missing` | 0.00 | 0.01 | 0.01 |
| `corp_troops` | 0.06 | 0.14 | 0.23 |
| `corrupt` | 0.23 | 0.59 | 0.63 |
| `corrupt_city` | 0.00 | 0.02 | 0.01 |
| `create` | 24.36 | 24.56 | 22.31 |
| `create_site` | 2.68 | 2.67 | 2.25 |
| `deal_threats` | 0.09 | 0.24 | 0.33 |
| `destroy_facility` | 0.00 | 0.01 | 0.02 |
| `disable_building` | 0.01 | 0.01 | 0.03 |
| `discard` | 0.01 | 0.05 | 0.03 |
| `extra_war` | 0.02 | 0.08 | 0.12 |
| `flag` | 0.17 | 0.41 | 0.42 |
| `flip_station` | 0.00 | 0.00 | 0.01 |
| `insight_lost` | 0.25 | 0.35 | 0.49 |
| `lose_ambassadors_or` | 0.13 | 0.17 | 0.21 |
| `lose_income` | 0.10 | 0.19 | 0.19 |
| `lose_node_or` | 0.00 | 0.02 | 0.11 |
| `lose_or` | 0.05 | 0.08 | 0.06 |
| `march_now` | 0.01 | 0.06 | 0.12 |
| `occupy_factory` | 0.01 | 0.05 | 0.06 |
| `place_botnet` | 0.03 | 0.14 | 0.17 |
| `players_lose` | 0.10 | 0.17 | 0.14 |
| `requisition` | 0.06 | 0.09 | 0.09 |
| `seize_factory` | 0.00 | 0.01 | 0.01 |
| `theft` | 0.11 | 0.16 | 0.16 |
| `theft_taken` | 0.00 | 0.44 | 0.44 |
| `troops` | 0.08 | 0.33 | 0.57 |
| `troops_walled` | 0.00 | 0.02 | 0.00 |

Each resource's peak holding a game, by seat's board (mean):

| Difficulty | Agent | Board | credits | insight | energy | chips | antimatter | research_cubes | card_tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | builder | 107.3 | 15.7 | 14.3 | 15.6 | 2.4 | 2.6 | 3.2 | 6.0 |
| d0 | strategist | protector | 90.1 | 26.1 | 11.8 | 7.6 | 1.8 | 3.9 | 2.2 | 7.8 |
| d0 | strategist | researcher | 135.8 | 38.1 | 14.1 | 7.6 | 2.1 | 3.7 | 1.9 | 9.0 |
| d1 | strategist | builder | 85.1 | 12.6 | 11.4 | 8.1 | 0.9 | 1.6 | 2.1 | 4.7 |
| d1 | strategist | protector | 75.0 | 22.8 | 10.2 | 5.6 | 0.8 | 2.8 | 1.5 | 6.2 |
| d1 | strategist | researcher | 97.3 | 31.0 | 10.8 | 5.3 | 1.0 | 3.3 | 1.4 | 6.1 |
| d2 | strategist | builder | 70.7 | 11.9 | 9.5 | 6.2 | 0.8 | 1.0 | 1.3 | 4.6 |
| d2 | strategist | protector | 65.4 | 17.5 | 8.9 | 4.0 | 0.6 | 1.5 | 1.1 | 5.2 |
| d2 | strategist | researcher | 90.5 | 25.2 | 9.1 | 4.8 | 0.7 | 1.9 | 1.2 | 5.5 |



