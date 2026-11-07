## out/read_r55 (rules r55)

| Difficulty | Pair | Agent | Games | Win % | Rounds | Ends |
|---|---|---|---|---|---|---|
| d1 | bp | strategist | 48 | 79.2 ±5.9 | 8.31 | unity_deck 27, purge 11, node 7, deck_empty 2, discord_deck 1 |
| d2 | bp | strategist | 48 | 68.8 ±6.7 | 8.02 | unity_deck 20, purge 13, node 13, discord_deck 1, deck_empty 1 |
| d0 | bp | strategist:w.node_exposure=1 | 48 | 87.5 ±4.8 | 8.40 | unity_deck 27, purge 15, node 3, deck_empty 3 |
| d1 | br | strategist | 48 | 54.2 ±7.2 | 7.79 | node 21, unity_deck 18, purge 8, deck_empty 1 |
| d1 | bp | strategist:w.node_exposure=1 | 48 | 75.0 ±6.2 | 8.42 | unity_deck 28, node 11, purge 8, deck_empty 1 |
| d0 | bp | strategist | 48 | 79.2 ±5.9 | 8.35 | purge 21, unity_deck 17, deck_empty 5, node 5 |
| d0 | br | strategist:w.node_exposure=1 | 48 | 85.4 ±5.1 | 8.48 | unity_deck 23, purge 18, node 5, discord_deck 1, deck_empty 1 |
| d2 | br | strategist:w.node_exposure=1 | 48 | 43.8 ±7.2 | 7.17 | node 24, unity_deck 14, purge 7, discord_deck 3 |
| d0 | br | strategist | 48 | 72.9 ±6.4 | 8.40 | purge 18, unity_deck 17, node 9, deck_empty 2, discord_deck 2 |
| d2 | br | strategist | 48 | 45.8 ±7.2 | 7.19 | node 26, unity_deck 13, purge 9 |
| d1 | pr | strategist:w.node_exposure=1 | 48 | 81.2 ±5.6 | 7.94 | unity_deck 20, purge 19, node 7, discord_deck 2 |
| d1 | br | strategist:w.node_exposure=1 | 48 | 54.2 ±7.2 | 7.94 | unity_deck 18, node 18, purge 8, discord_deck 4 |
| d2 | pr | strategist:w.node_exposure=1 | 48 | 70.8 ±6.6 | 7.85 | unity_deck 25, node 13, purge 9, discord_deck 1 |
| d0 | pr | strategist | 48 | 87.5 ±4.8 | 8.44 | unity_deck 26, purge 16, deck_empty 4, node 2 |
| d0 | pr | strategist:w.node_exposure=1 | 48 | 85.4 ±5.1 | 8.40 | unity_deck 28, purge 13, node 5, deck_empty 2 |
| d1 | pr | strategist | 48 | 81.2 ±5.6 | 8.21 | unity_deck 23, purge 16, node 9 |
| d2 | pr | strategist | 48 | 77.1 ±6.1 | 8.04 | unity_deck 21, purge 16, node 10, discord_deck 1 |
| d2 | bp | strategist:w.node_exposure=1 | 48 | 64.6 ±6.9 | 7.71 | unity_deck 22, node 14, purge 9, deck_empty 2, discord_deck 1 |

Pooled over the pairs:

| Difficulty | Agent | Games | Win % | Rounds | Losses by end | Nodes lost a game, by cause |
|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 79.9 ±3.3 | 8.40 | node 16, deck_empty 11, discord_deck 2 | corruption 3.15, discord 0.70, twist 0.63, counter 0.20, war 0.16, project 0.12 |
| d0 | strategist:w.node_exposure=1 | 144 | 86.1 ±2.9 | 8.42 | node 13, deck_empty 6, discord_deck 1 | corruption 3.26, discord 1.07, twist 0.74, project 0.31, counter 0.30, war 0.17 |
| d1 | strategist | 144 | 71.5 ±3.8 | 8.10 | node 37, deck_empty 3, discord_deck 1 | corruption 4.13, discord 1.19, twist 0.67, counter 0.40, war 0.17, project 0.12 |
| d1 | strategist:w.node_exposure=1 | 144 | 70.1 ±3.8 | 8.10 | node 36, discord_deck 6, deck_empty 1 | corruption 3.60, discord 1.68, twist 0.74, counter 0.46, project 0.29, war 0.21 |
| d2 | strategist | 144 | 63.9 ±4.0 | 7.75 | node 49, discord_deck 2, deck_empty 1 | corruption 4.61, discord 1.33, twist 0.78, counter 0.32, war 0.17, project 0.13 |
| d2 | strategist:w.node_exposure=1 | 144 | 59.7 ±4.1 | 7.58 | node 51, discord_deck 5, deck_empty 2 | corruption 3.94, discord 1.49, twist 0.92, counter 0.47, project 0.39, war 0.17 |

Threats a game (dealt / countered / uncountered executed), and the counter kinds used a game:

| Difficulty | Agent | Dealt | Countered | Uncountered | credits | energy | insight | chips | ambassadors | research_cubes | tags | nodes | card | standing |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 18.9 | 16.5 | 1.2 | 9.15 | 1.60 | 0.90 | 0.00 | 0.08 | 0.23 | 0.15 | 0.17 | 1.56 | 2.66 |
| d0 | strategist:w.node_exposure=1 | 19.0 | 16.4 | 1.5 | 9.06 | 1.44 | 0.76 | 0.00 | 0.06 | 0.24 | 0.12 | 0.24 | 1.74 | 2.69 |
| d1 | strategist | 26.6 | 22.3 | 2.1 | 13.10 | 1.94 | 0.97 | 0.00 | 0.19 | 0.29 | 0.28 | 0.30 | 2.18 | 3.08 |
| d1 | strategist:w.node_exposure=1 | 27.2 | 22.8 | 2.4 | 13.22 | 1.81 | 1.18 | 0.00 | 0.13 | 0.31 | 0.31 | 0.37 | 2.10 | 3.34 |
| d2 | strategist | 26.5 | 21.8 | 2.1 | 12.97 | 1.77 | 0.95 | 0.00 | 0.10 | 0.24 | 0.22 | 0.22 | 1.99 | 3.28 |
| d2 | strategist:w.node_exposure=1 | 26.0 | 20.8 | 2.6 | 11.97 | 1.71 | 1.01 | 0.00 | 0.15 | 0.19 | 0.29 | 0.35 | 2.03 | 3.08 |

Threats countered / uncountered per round (mean over the games that reached it):

| Difficulty | Agent | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 | r11 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 1.6/0.4 | 1.7/0.4 | 2.1/0.2 | 2.3/0.1 | 1.9/0.0 | 2.1/0.0 | 2.2/0.0 | 1.9/0.0 | 1.5/0.0 | 2.7/0.0 | 2.9/0.1 |
| d0 | strategist:w.node_exposure=1 | 1.5/0.5 | 1.6/0.4 | 2.1/0.3 | 2.2/0.2 | 2.0/0.1 | 2.3/0.0 | 2.1/0.0 | 1.9/0.0 | 1.3/0.0 | 3.3/0.0 | 1.9/0.1 |
| d1 | strategist | 2.3/0.7 | 2.6/0.6 | 3.0/0.3 | 3.2/0.2 | 2.7/0.2 | 3.1/0.1 | 2.9/0.1 | 2.8/0.0 | 1.9/0.0 | 2.7/0.0 | 2.5/0.0 |
| d1 | strategist:w.node_exposure=1 | 2.3/0.7 | 2.5/0.6 | 2.9/0.5 | 3.2/0.2 | 2.8/0.1 | 3.1/0.1 | 3.2/0.1 | 3.2/0.1 | 2.3/0.1 | 2.5/0.2 | 0.4/0.0 |
| d2 | strategist | 2.3/0.7 | 2.6/0.6 | 3.0/0.3 | 2.7/0.3 | 2.8/0.1 | 3.0/0.1 | 3.1/0.1 | 2.8/0.0 | 3.4/0.0 | 3.3/0.1 | 1.0/0.0 |
| d2 | strategist:w.node_exposure=1 | 2.3/0.7 | 2.5/0.6 | 2.9/0.5 | 2.5/0.5 | 2.8/0.2 | 3.1/0.1 | 3.2/0.1 | 2.8/0.1 | 2.7/0.0 | 4.4/0.1 | 2.5/0.1 |

The new ops: resolutions a game, by difficulty and agent:

| Op | d0 strategist | d0 strategist:w.node_exposure=1 | d1 strategist | d1 strategist:w.node_exposure=1 | d2 strategist | d2 strategist:w.node_exposure=1 |
|---|---|---|---|---|---|---|
| `adversary_factories` | 0.01 | 0.03 | 0.01 | 0.01 | 0.01 | 0.01 |
| `adversary_factory` | 0.11 | 0.10 | 0.08 | 0.10 | 0.11 | 0.12 |
| `ambassador_moves` | 0.67 | 0.40 | 0.47 | 0.72 | 0.49 | 0.36 |
| `ambassadors` | 0.09 | 0.10 | 0.16 | 0.18 | 0.21 | 0.20 |
| `cleansed` | 1.54 | 1.42 | 1.40 | 1.26 | 1.15 | 0.98 |
| `compute_per_missing` | 0.00 | 0.00 | 0.02 | 0.01 | 0.01 | 0.01 |
| `corp_troops` | 0.05 | 0.08 | 0.11 | 0.12 | 0.11 | 0.17 |
| `corrupt_city` | 0.00 | 0.01 | 0.02 | 0.01 | 0.02 | 0.03 |
| `deal_threats` | 0.12 | 0.15 | 0.17 | 0.18 | 0.18 | 0.21 |
| `disable_building` | 0.00 | 0.00 | 0.01 | 0.00 | 0.01 | 0.01 |
| `discard` | 0.02 | 0.02 | 0.06 | 0.07 | 0.06 | 0.06 |
| `extra_war` | 0.01 | 0.02 | 0.01 | 0.03 | 0.03 | 0.05 |
| `flag` | 0.36 | 0.56 | 0.62 | 0.81 | 0.71 | 0.73 |
| `insight_lost` | 0.39 | 0.36 | 0.49 | 0.42 | 0.42 | 0.56 |
| `lose_ambassadors_or` | 0.18 | 0.18 | 0.28 | 0.35 | 0.25 | 0.31 |
| `lose_income` | 0.15 | 0.15 | 0.10 | 0.12 | 0.08 | 0.09 |
| `lose_node_or` | 0.00 | 0.01 | 0.03 | 0.03 | 0.03 | 0.03 |
| `lose_or` | 0.06 | 0.08 | 0.03 | 0.07 | 0.03 | 0.08 |
| `march_now` | 0.01 | 0.00 | 0.04 | 0.03 | 0.01 | 0.06 |
| `occupy_factory` | 0.00 | 0.02 | 0.03 | 0.07 | 0.06 | 0.08 |
| `place_botnet` | 0.01 | 0.00 | 0.01 | 0.01 | 0.02 | 0.01 |
| `players_lose` | 0.08 | 0.08 | 0.10 | 0.10 | 0.08 | 0.08 |
| `requisition` | 0.09 | 0.08 | 0.06 | 0.06 | 0.06 | 0.06 |
| `theft` | 0.11 | 0.15 | 0.31 | 0.28 | 0.26 | 0.28 |
| `theft_taken` | 0.00 | 0.00 | 0.69 | 0.62 | 0.56 | 0.35 |
| `troops` | 1.67 | 1.68 | 1.82 | 1.87 | 1.58 | 1.66 |
| `troops_walled` | 0.06 | 0.04 | 0.10 | 0.07 | 0.04 | 0.03 |

Each resource's peak holding a game, by seat's board (mean):

| Difficulty | Agent | Board | credits | insight | energy | chips | antimatter | research_cubes | card_tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | builder | 107.5 | 14.6 | 15.6 | 15.8 | 2.7 | 3.1 | 3.1 | 5.8 |
| d0 | strategist | protector | 84.5 | 21.9 | 12.9 | 10.1 | 1.8 | 4.9 | 2.7 | 10.4 |
| d0 | strategist | researcher | 124.2 | 38.2 | 13.6 | 10.6 | 2.0 | 5.0 | 2.0 | 9.1 |
| d0 | strategist:w.node_exposure=1 | builder | 104.6 | 15.6 | 16.6 | 14.6 | 1.9 | 2.8 | 3.0 | 6.5 |
| d0 | strategist:w.node_exposure=1 | protector | 86.2 | 21.3 | 12.8 | 8.4 | 1.3 | 5.4 | 2.4 | 9.1 |
| d0 | strategist:w.node_exposure=1 | researcher | 120.6 | 39.2 | 13.9 | 9.0 | 1.7 | 4.9 | 1.7 | 8.1 |
| d1 | strategist | builder | 89.4 | 13.3 | 12.3 | 10.9 | 1.6 | 2.2 | 2.6 | 5.5 |
| d1 | strategist | protector | 80.7 | 20.2 | 10.0 | 7.1 | 0.9 | 3.6 | 2.1 | 9.0 |
| d1 | strategist | researcher | 111.9 | 31.2 | 11.4 | 7.9 | 1.4 | 4.0 | 1.1 | 7.9 |
| d1 | strategist:w.node_exposure=1 | builder | 89.0 | 13.3 | 11.8 | 9.7 | 1.0 | 2.2 | 2.4 | 5.1 |
| d1 | strategist:w.node_exposure=1 | protector | 81.2 | 19.8 | 11.1 | 7.3 | 0.9 | 3.6 | 1.6 | 8.0 |
| d1 | strategist:w.node_exposure=1 | researcher | 96.7 | 30.8 | 10.5 | 7.2 | 1.1 | 4.3 | 1.5 | 7.5 |
| d2 | strategist | builder | 82.0 | 12.6 | 10.6 | 8.7 | 0.9 | 1.6 | 2.1 | 4.7 |
| d2 | strategist | protector | 78.8 | 18.4 | 9.9 | 6.9 | 0.9 | 3.6 | 1.7 | 7.3 |
| d2 | strategist | researcher | 95.0 | 29.1 | 10.0 | 5.8 | 1.1 | 3.4 | 1.1 | 7.1 |
| d2 | strategist:w.node_exposure=1 | builder | 75.3 | 12.1 | 10.8 | 8.2 | 0.8 | 1.6 | 2.5 | 3.9 |
| d2 | strategist:w.node_exposure=1 | protector | 77.5 | 18.3 | 11.5 | 6.5 | 0.9 | 4.1 | 1.7 | 8.0 |
| d2 | strategist:w.node_exposure=1 | researcher | 87.9 | 28.4 | 8.4 | 5.9 | 1.0 | 3.3 | 1.2 | 6.3 |

## out/read_r54 (rules r54)

| Difficulty | Pair | Agent | Games | Win % | Rounds | Ends |
|---|---|---|---|---|---|---|
| d2 | br | strategist | 48 | 50.0 ±7.2 | 5.52 | unity_deck 24, node 22, discord_deck 2 |
| d1 | br | strategist | 48 | 66.7 ±6.8 | 6.00 | unity_deck 32, node 15, discord_deck 1 |
| d0 | pr | strategist | 48 | 81.2 ±5.6 | 6.19 | unity_deck 36, node 9, purge 3 |
| d1 | bp | strategist | 48 | 68.8 ±6.7 | 6.33 | unity_deck 33, node 14, discord_deck 1 |
| d0 | br | strategist | 48 | 77.1 ±6.1 | 6.52 | unity_deck 34, node 10, purge 3, discord_deck 1 |
| d0 | bp | strategist | 48 | 89.6 ±4.4 | 6.38 | unity_deck 37, purge 6, node 4, discord_deck 1 |
| d2 | bp | strategist | 48 | 64.6 ±6.9 | 6.10 | unity_deck 31, node 16, discord_deck 1 |
| d1 | pr | strategist | 48 | 77.1 ±6.1 | 6.08 | unity_deck 37, node 10, discord_deck 1 |
| d2 | pr | strategist | 48 | 68.8 ±6.7 | 5.96 | unity_deck 33, node 15 |

Pooled over the pairs:

| Difficulty | Agent | Games | Win % | Rounds | Losses by end | Nodes lost a game, by cause |
|---|---|---|---|---|---|---|
| d0 | strategist | 144 | 82.6 ±3.2 | 6.36 | node 23, discord_deck 2 | corruption 2.85, discord 1.67, twist 0.42, project 0.08, counter 0.08, war 0.06 |
| d1 | strategist | 144 | 70.8 ±3.8 | 6.14 | node 39, discord_deck 3 | corruption 3.28, discord 2.08, twist 0.43, counter 0.21, war 0.10, project 0.06 |
| d2 | strategist | 144 | 61.1 ±4.1 | 5.86 | node 53, discord_deck 3 | corruption 3.74, discord 1.87, twist 0.60, counter 0.22, war 0.09, project 0.08 |

Threats a game (dealt / countered / uncountered executed), and the counter kinds used a game:

| Difficulty | Agent | Dealt | Countered | Uncountered | credits | energy | insight | chips | ambassadors | research_cubes | tags | nodes | card | standing |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 15.1 | 12.9 | 1.0 | 7.18 | 1.08 | 0.59 | 0.00 | 0.03 | 0.08 | 0.13 | 0.07 | 2.10 | 1.61 |
| d1 | strategist | 21.0 | 17.2 | 1.9 | 10.01 | 1.32 | 0.75 | 0.00 | 0.06 | 0.09 | 0.25 | 0.17 | 2.74 | 1.82 |
| d2 | strategist | 20.1 | 15.9 | 2.1 | 9.44 | 1.08 | 0.73 | 0.00 | 0.02 | 0.05 | 0.15 | 0.17 | 2.42 | 1.86 |

Threats countered / uncountered per round (mean over the games that reached it):

| Difficulty | Agent | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | 1.6/0.4 | 1.9/0.3 | 2.2/0.2 | 2.3/0.1 | 2.2/0.0 | 2.3/0.0 | 1.6/0.0 | 1.3/0.0 | 2.0/0.0 | 1.6/0.0 |
| d1 | strategist | 2.3/0.7 | 2.6/0.7 | 3.1/0.3 | 3.3/0.1 | 2.9/0.1 | 3.0/0.0 | 2.4/0.0 | 1.8/0.0 | 1.5/0.0 |  |
| d2 | strategist | 2.3/0.7 | 2.6/0.7 | 3.1/0.3 | 2.7/0.2 | 2.9/0.1 | 2.9/0.1 | 2.7/0.0 | 1.8/0.0 |  |  |

The new ops: resolutions a game, by difficulty and agent:

| Op | d0 strategist | d1 strategist | d2 strategist |
|---|---|---|---|
| `adversary_factories` | 0.00 | 0.01 | 0.02 |
| `adversary_factory` | 0.06 | 0.08 | 0.08 |
| `ambassador_moves` | 0.30 | 0.28 | 0.25 |
| `ambassadors` | 0.08 | 0.15 | 0.19 |
| `cleansed` | 0.75 | 0.57 | 0.46 |
| `compute_per_missing` | 0.00 | 0.02 | 0.01 |
| `corp_troops` | 0.04 | 0.05 | 0.10 |
| `corrupt_city` | 0.00 | 0.01 | 0.03 |
| `deal_threats` | 0.08 | 0.19 | 0.20 |
| `disable_building` | 0.00 | 0.00 | 0.01 |
| `discard` | 0.02 | 0.05 | 0.05 |
| `extra_war` | 0.01 | 0.01 | 0.01 |
| `flag` | 0.38 | 0.55 | 0.54 |
| `insight_lost` | 0.17 | 0.44 | 0.39 |
| `lose_ambassadors_or` | 0.15 | 0.32 | 0.30 |
| `lose_income` | 0.12 | 0.08 | 0.07 |
| `lose_or` | 0.03 | 0.03 | 0.03 |
| `march_now` | 0.01 | 0.00 | 0.01 |
| `occupy_factory` | 0.00 | 0.00 | 0.04 |
| `place_botnet` | 0.01 | 0.00 | 0.01 |
| `players_lose` | 0.08 | 0.08 | 0.07 |
| `requisition` | 0.06 | 0.07 | 0.07 |
| `theft` | 0.09 | 0.24 | 0.26 |
| `theft_taken` | 0.00 | 0.28 | 0.33 |
| `troops` | 1.67 | 1.76 | 1.51 |
| `troops_walled` | 0.09 | 0.10 | 0.06 |

Each resource's peak holding a game, by seat's board (mean):

| Difficulty | Agent | Board | credits | insight | energy | chips | antimatter | research_cubes | card_tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 | strategist | builder | 68.9 | 10.8 | 12.4 | 6.8 | 0.7 | 1.6 | 1.9 | 3.8 |
| d0 | strategist | protector | 61.6 | 13.6 | 9.0 | 4.8 | 0.4 | 1.7 | 1.4 | 5.1 |
| d0 | strategist | researcher | 75.3 | 25.5 | 9.2 | 5.9 | 0.9 | 3.3 | 0.7 | 5.7 |
| d1 | strategist | builder | 54.9 | 10.0 | 10.1 | 4.9 | 0.5 | 1.2 | 1.6 | 3.5 |
| d1 | strategist | protector | 54.1 | 12.9 | 7.9 | 3.7 | 0.3 | 1.3 | 1.1 | 4.5 |
| d1 | strategist | researcher | 63.5 | 21.1 | 8.1 | 3.6 | 0.5 | 2.3 | 0.8 | 5.1 |
| d2 | strategist | builder | 48.8 | 9.4 | 8.7 | 3.8 | 0.3 | 0.7 | 1.3 | 2.8 |
| d2 | strategist | protector | 53.2 | 12.6 | 7.6 | 3.4 | 0.2 | 1.2 | 0.9 | 4.2 |
| d2 | strategist | researcher | 64.1 | 20.7 | 7.9 | 3.1 | 0.4 | 1.6 | 1.0 | 4.6 |

