## out/read_cnt (rules r54)

| Difficulty | Pair | Agent | Games | Win % | Rounds | Ends |
|---|---|---|---|---|---|---|
| d2 | bp | strategist:old_counters=1 | 48 | 60.4 ±7.1 | 6.08 | unity_deck 29, node 18, discord_deck 1 |
| d2 | br | strategist:old_counters=1 | 48 | 52.1 ±7.2 | 5.65 | unity_deck 24, node 22, purge 1, discord_deck 1 |
| d2 | bp | strategist | 48 | 64.6 ±6.9 | 6.00 | unity_deck 31, node 17 |
| d2 | br | strategist | 48 | 50.0 ±7.2 | 5.52 | node 23, unity_deck 22, purge 2, discord_deck 1 |
| d2 | pr | strategist:old_counters=1 | 48 | 68.8 ±6.7 | 6.00 | unity_deck 32, node 14, discord_deck 1, purge 1 |
| d2 | pr | strategist | 48 | 70.8 ±6.6 | 6.06 | unity_deck 33, node 12, discord_deck 2, purge 1 |

Pooled over the pairs:

| Difficulty | Agent | Games | Win % | Rounds | Losses by end | Nodes lost a game, by cause |
|---|---|---|---|---|---|---|
| d2 | strategist | 144 | 61.8 ±4.0 | 5.86 | node 52, discord_deck 3 | corruption 3.65, discord 1.69, twist 0.56, counter 0.28, project 0.19, war 0.12 |
| d2 | strategist:old_counters=1 | 144 | 60.4 ±4.1 | 5.91 | node 54, discord_deck 3 | corruption 3.62, discord 2.01, twist 0.58, project 0.22, war 0.17 |

Threats a game (dealt / countered / uncountered executed), and the counter kinds used a game:

| Difficulty | Agent | Dealt | Countered | Uncountered | credits | energy | insight | chips | ambassadors | research_cubes | tags | nodes | card | standing |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| d2 | strategist | 19.9 | 16.3 | 1.6 | 9.93 | 1.00 | 0.72 | 0.00 | 0.08 | 0.10 | 0.11 | 0.23 | 2.46 | 1.70 |
| d2 | strategist:old_counters=1 | 20.0 | 16.3 | 1.9 | 10.46 | 1.10 | 0.70 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 2.36 | 1.67 |

Threats countered / uncountered per round (mean over the games that reached it):

| Difficulty | Agent | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 |
|---|---|---|---|---|---|---|---|---|---|---|
| d2 | strategist | 2.5/0.5 | 2.7/0.4 | 2.9/0.4 | 2.9/0.2 | 3.1/0.1 | 3.0/0.1 | 2.4/0.1 | 1.0/0.0 | 2.5/0.0 |
| d2 | strategist:old_counters=1 | 2.5/0.5 | 2.6/0.5 | 2.9/0.4 | 2.9/0.2 | 3.1/0.2 | 2.8/0.1 | 2.4/0.0 | 0.9/0.0 | 5.0/0.0 |

The new ops: resolutions a game, by difficulty and agent:

| Op | d2 strategist | d2 strategist:old_counters=1 |
|---|---|---|
| `adversary_factories` | 0.01 | 0.02 |
| `adversary_factory` | 0.07 | 0.08 |
| `ambassador_moves` | 0.23 | 0.26 |
| `ambassadors` | 0.12 | 0.20 |
| `cleansed` | 0.51 | 0.58 |
| `corp_troops` | 0.10 | 0.15 |
| `corrupt_city` | 0.02 | 0.02 |
| `deal_threats` | 0.10 | 0.11 |
| `discard` | 0.01 | 0.01 |
| `extra_war` | 0.03 | 0.04 |
| `flag` | 0.46 | 0.44 |
| `insight_lost` | 0.35 | 0.35 |
| `lose_ambassadors_or` | 0.25 | 0.26 |
| `lose_income` | 0.12 | 0.13 |
| `lose_node_or` | 0.01 | 0.01 |
| `lose_or` | 0.02 | 0.03 |
| `march_now` | 0.01 | 0.04 |
| `occupy_factory` | 0.01 | 0.03 |
| `place_botnet` | 0.01 | 0.01 |
| `players_lose` | 0.05 | 0.05 |
| `requisition` | 0.06 | 0.06 |
| `theft` | 0.10 | 0.10 |
| `theft_taken` | 0.06 | 0.06 |
| `troops` | 1.41 | 1.52 |
| `troops_walled` | 0.03 | 0.05 |

Each resource's peak holding a game, by seat's board (mean):

| Difficulty | Agent | Board | credits | insight | energy | chips | antimatter | research_cubes | card_tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|---|---|
| d2 | strategist | builder | 54.6 | 9.4 | 9.2 | 3.6 | 0.3 | 0.8 | 1.6 | 2.9 |
| d2 | strategist | protector | 61.3 | 12.6 | 8.6 | 3.5 | 0.5 | 1.3 | 1.4 | 4.9 |
| d2 | strategist | researcher | 57.0 | 19.6 | 6.7 | 2.8 | 0.5 | 1.8 | 1.2 | 3.9 |
| d2 | strategist:old_counters=1 | builder | 56.5 | 9.7 | 9.2 | 3.9 | 0.3 | 0.9 | 1.7 | 3.1 |
| d2 | strategist:old_counters=1 | protector | 59.8 | 12.5 | 8.6 | 3.4 | 0.4 | 1.2 | 1.4 | 5.0 |
| d2 | strategist:old_counters=1 | researcher | 56.8 | 19.9 | 6.8 | 2.7 | 0.4 | 1.8 | 1.1 | 4.0 |

