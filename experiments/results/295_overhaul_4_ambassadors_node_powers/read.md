# 295: the before-and-after read

The judge (strategist) on the three board pairs (B+P, B+R, P+R), d0-d3, 48 fresh seeds a cell (50424-50471, taken
in the ledger), 576 games a side: r141 is main at 3bd6b5d6 (`farm-seeds --at`), r142 this item's build at d704584f.
The same seeds on both sides. **The comparison changes several things at once**: an ambassador's one action a round,
Liberate as its act, the node powers, and two corrupted cities a country at setup; `levers.md` separates the last.

## A level a row, the pairs pooled

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r141 d0 | 144 | 142 (99%) | 5.8 (1-14) | 29 (20%) | 68 (47%) | 1 | liberated 142, deck_empty 2 | 3.29 (1.02 by card) | 0.59 | 2.81 | 8.6 |
| r141 d1 | 144 | 134 (93%) | 7.0 (1-14) | 15 (10%) | 40 (28%) | 1 | liberated 134, deck_empty 5, research 3, discord_deck 2 | 5.40 (1.38 by card) | 0.90 | 4.75 | 12.1 |
| r141 d2 | 144 | 128 (89%) | 6.8 (1-13) | 12 (8%) | 35 (24%) | 1 | liberated 128, research 12, discord_deck 4 | 5.60 (1.38 by card) | 0.65 | 4.78 | 11.1 |
| r141 d3 | 144 | 118 (82%) | 7.0 (1-12) | 9 (6%) | 27 (19%) | 1 | liberated 118, research 18, deck_empty 4, discord_deck 4 | 5.88 (1.35 by card) | 0.98 | 5.41 | 11.8 |
| r142 d0 | 144 | 130 (90%) | 8.3 (3-14) | 1 (1%) | 16 (11%) | 3 | liberated 130, deck_empty 10, research 3, discord_deck 1 | 5.19 (1.60 by card) | 1.33 | 4.74 | 18.1 |
| r142 d1 | 144 | 120 (83%) | 9.0 (5-14) | 0 (0%) | 13 (9%) | 5 | liberated 120, deck_empty 14, research 6, discord_deck 4 | 6.78 (1.97 by card) | 1.38 | 6.01 | 24.9 |
| r142 d2 | 144 | 96 (67%) | 8.7 (4-14) | 0 (0%) | 9 (6%) | 4 | liberated 96, research 24, deck_empty 14, discord_deck 10 | 6.52 (1.81 by card) | 1.28 | 5.82 | 22.3 |
| r142 d3 | 144 | 73 (51%) | 8.3 (4-12) | 0 (0%) | 5 (3%) | 4 | liberated 73, research 42, deck_empty 16, discord_deck 13 | 6.32 (1.69 by card) | 1.31 | 5.82 | 20.8 |

## A pair and level a row

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r141 bp d0 | 48 | 48 (100%) | 5.5 (1-12) | 10 (21%) | 22 (46%) | 1 | liberated 48 | 3.12 (0.75 by card) | 0.52 | 2.88 | 7.3 |
| r141 bp d1 | 48 | 46 (96%) | 6.8 (1-14) | 6 (12%) | 15 (31%) | 1 | liberated 46, deck_empty 1, research 1 | 5.25 (1.12 by card) | 0.81 | 4.79 | 10.8 |
| r141 bp d2 | 48 | 43 (90%) | 6.5 (1-11) | 5 (10%) | 13 (27%) | 1 | liberated 43, research 4, discord_deck 1 | 5.48 (1.10 by card) | 0.56 | 4.85 | 9.9 |
| r141 bp d3 | 48 | 40 (83%) | 6.9 (1-12) | 4 (8%) | 10 (21%) | 1 | liberated 40, research 6, deck_empty 1, discord_deck 1 | 5.98 (1.25 by card) | 0.85 | 5.52 | 11.2 |
| r141 br d0 | 48 | 47 (98%) | 6.1 (2-14) | 9 (19%) | 18 (38%) | 2 | liberated 47, deck_empty 1 | 3.58 (1.38 by card) | 0.60 | 2.77 | 9.7 |
| r141 br d1 | 48 | 44 (92%) | 7.2 (2-14) | 5 (10%) | 11 (23%) | 2 | liberated 44, discord_deck 2, deck_empty 1, research 1 | 5.65 (1.52 by card) | 0.88 | 4.79 | 12.1 |
| r141 br d2 | 48 | 39 (81%) | 7.0 (2-11) | 4 (8%) | 10 (21%) | 2 | liberated 39, research 7, discord_deck 2 | 5.58 (1.54 by card) | 0.58 | 4.52 | 11.5 |
| r141 br d3 | 48 | 37 (77%) | 7.2 (1-11) | 2 (4%) | 8 (17%) | 1 | liberated 37, research 7, discord_deck 3, deck_empty 1 | 5.69 (1.46 by card) | 1.08 | 5.21 | 12.3 |
| r141 pr d0 | 48 | 47 (98%) | 5.7 (2-14) | 10 (21%) | 28 (58%) | 2 | liberated 47, deck_empty 1 | 3.17 (0.94 by card) | 0.65 | 2.77 | 8.9 |
| r141 pr d1 | 48 | 44 (92%) | 7.0 (2-14) | 4 (8%) | 14 (29%) | 2 | liberated 44, deck_empty 3, research 1 | 5.31 (1.48 by card) | 1.00 | 4.67 | 13.5 |
| r141 pr d2 | 48 | 46 (96%) | 6.8 (2-13) | 3 (6%) | 12 (25%) | 2 | liberated 46, discord_deck 1, research 1 | 5.75 (1.48 by card) | 0.79 | 4.96 | 12.0 |
| r141 pr d3 | 48 | 41 (85%) | 6.9 (1-12) | 3 (6%) | 9 (19%) | 1 | liberated 41, research 5, deck_empty 2 | 5.96 (1.35 by card) | 1.00 | 5.50 | 11.8 |
| r142 bp d0 | 48 | 43 (90%) | 8.2 (4-14) | 0 (0%) | 5 (10%) | 4 | liberated 43, deck_empty 2, research 2, discord_deck 1 | 5.10 (1.50 by card) | 1.33 | 4.77 | 16.2 |
| r142 bp d1 | 48 | 38 (79%) | 9.5 (5-14) | 0 (0%) | 2 (4%) | 5 | liberated 38, deck_empty 5, research 3, discord_deck 2 | 7.23 (1.83 by card) | 1.33 | 6.58 | 24.1 |
| r142 bp d2 | 48 | 31 (65%) | 8.8 (5-14) | 0 (0%) | 2 (4%) | 5 | liberated 31, research 9, deck_empty 5, discord_deck 3 | 6.44 (1.62 by card) | 1.56 | 6.23 | 20.9 |
| r142 bp d3 | 48 | 18 (38%) | 8.6 (5-12) | 0 (0%) | 1 (2%) | 5 | liberated 18, research 17, discord_deck 7, deck_empty 6 | 5.90 (1.31 by card) | 1.31 | 5.77 | 18.9 |
| r142 br d0 | 48 | 45 (94%) | 8.2 (3-14) | 1 (2%) | 7 (15%) | 3 | liberated 45, deck_empty 2, research 1 | 5.27 (1.69 by card) | 1.19 | 4.58 | 18.1 |
| r142 br d1 | 48 | 41 (85%) | 9.1 (5-14) | 0 (0%) | 7 (15%) | 5 | liberated 41, deck_empty 4, research 2, discord_deck 1 | 6.56 (2.15 by card) | 1.46 | 5.73 | 26.1 |
| r142 br d2 | 48 | 34 (71%) | 8.4 (4-14) | 0 (0%) | 2 (4%) | 4 | liberated 34, research 9, discord_deck 4, deck_empty 1 | 6.33 (1.79 by card) | 0.92 | 5.31 | 19.5 |
| r142 br d3 | 48 | 28 (58%) | 8.2 (5-12) | 0 (0%) | 1 (2%) | 5 | liberated 28, research 14, deck_empty 3, discord_deck 3 | 6.23 (2.00 by card) | 1.29 | 5.40 | 20.9 |
| r142 pr d0 | 48 | 42 (88%) | 8.5 (5-14) | 0 (0%) | 4 (8%) | 5 | liberated 42, deck_empty 6 | 5.21 (1.62 by card) | 1.48 | 4.88 | 20.0 |
| r142 pr d1 | 48 | 41 (85%) | 8.4 (5-14) | 0 (0%) | 4 (8%) | 5 | liberated 41, deck_empty 5, discord_deck 1, research 1 | 6.54 (1.92 by card) | 1.33 | 5.73 | 24.6 |
| r142 pr d2 | 48 | 31 (65%) | 8.9 (4-14) | 0 (0%) | 5 (10%) | 4 | liberated 31, deck_empty 8, research 6, discord_deck 3 | 6.79 (2.02 by card) | 1.38 | 5.92 | 26.5 |
| r142 pr d3 | 48 | 27 (56%) | 8.2 (4-12) | 0 (0%) | 3 (6%) | 4 | liberated 27, research 11, deck_empty 7, discord_deck 3 | 6.83 (1.75 by card) | 1.31 | 6.29 | 22.6 |

## The piles, a level a row (the pairs pooled)

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r141 d0 | 144 | 142 (99%), 68 by r5 | 5.8 (1-14) | deck_empty 2 | 0:12 1:50 2:37 3:30 4:14 5:1 | 14.2 / 50 | 15.4 / 42 | 10.7 / 21 | 0.01 / 2 |
| r142 d0 | 144 | 130 (90%), 16 by r5 | 8.3 (3-14) | deck_empty 10, discord_deck 1, research 3 | 0:3 1:16 2:44 3:44 4:25 5:4 6:5 7:3 | 19.7 / 59 | 26.7 / 66 | 14.8 / 31 | 0.06 / 2 |
| r141 d1 | 144 | 134 (93%), 40 by r5 | 7.0 (1-14) | deck_empty 5, discord_deck 2, research 3 | 0:3 1:21 2:28 3:41 4:30 5:13 6:5 7:3 | 15.0 / 54 | 26.2 / 67 | 15.2 / 33 | 0.17 / 4 |
| r142 d1 | 144 | 120 (83%), 13 by r5 | 9.0 (5-14) | deck_empty 14, discord_deck 4, research 6 | 0:2 1:14 2:28 3:35 4:26 5:19 6:14 7:6 | 19.3 / 55 | 35.5 / 66 | 17.5 / 34 | 0.13 / 2 |
| r141 d2 | 144 | 128 (89%), 35 by r5 | 6.8 (1-13) | discord_deck 4, research 12 | 0:1 1:24 2:21 3:29 4:26 5:19 6:12 7:12 | 14.5 / 35 | 29.6 / 76 | 17.6 / 37 | 0.27 / 4 |
| r142 d2 | 144 | 96 (67%), 9 by r5 | 8.7 (4-14) | deck_empty 14, discord_deck 10, research 24 | 0:3 1:5 2:20 3:24 4:34 5:22 6:12 7:24 | 19.1 / 56 | 40.4 / 67 | 20.5 / 42 | 0.33 / 4 |
| r141 d3 | 144 | 118 (82%), 27 by r5 | 7.0 (1-12) | deck_empty 4, discord_deck 4, research 18 | 0:4 1:11 2:14 3:27 4:25 5:20 6:25 7:18 | 15.7 / 48 | 32.0 / 66 | 20.0 / 38 | 0.33 / 4 |
| r142 d3 | 144 | 73 (51%), 5 by r5 | 8.3 (4-12) | deck_empty 16, discord_deck 13, research 42 | 0:1 1:4 2:13 3:20 4:21 5:24 6:19 7:42 | 18.2 / 54 | 43.3 / 71 | 23.4 / 46 | 0.68 / 6 |

## Greedy on the same seeds, r142 (spec `read295g`)

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| greedy d0 | 144 | 135 (94%) | 7.9 (3-14) | 2 (1%) | 24 (17%) | 3 | liberated 135, deck_empty 8, research 1 | 5.15 (1.53 by card) | 0.94 | 4.42 | 18.8 |
| greedy d1 | 144 | 123 (85%) | 8.7 (4-14) | 0 (0%) | 6 (4%) | 4 | liberated 123, deck_empty 11, research 7, discord_deck 3 | 6.51 (1.94 by card) | 1.43 | 5.83 | 25.1 |
| greedy d2 | 144 | 111 (77%) | 8.7 (4-14) | 0 (0%) | 9 (6%) | 4 | liberated 111, research 13, discord_deck 11, deck_empty 9 | 6.54 (1.99 by card) | 1.24 | 5.69 | 26.0 |
| greedy d3 | 144 | 64 (44%) | 9.0 (5-12) | 0 (0%) | 3 (2%) | 5 | liberated 64, research 46, deck_empty 23, discord_deck 11 | 6.24 (1.65 by card) | 1.35 | 5.83 | 25.3 |
