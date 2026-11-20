# 297: the smoke read, before and after

Written by `make_smoke.py`. Two players, the three board pairs (Builder + Protector, Builder + Researcher,
Protector + Researcher), 48 seeds a cell, d0-d3, the judge (`strategist`) and greedy. r142 is main at
2282b5387d; r143 is this item's final build (02ee710e). "Slot reached" is the research row's slot at the
game's end; `deck_empty` is the round cap, `research` slot 7, `discord_deck` a tip on an empty Discord deck.

## Same seeds as 295's read (50424-50471), the three pairs pooled, 144 games a row

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 / 5 | How games ended | Slot reached: median; games at slot 4+ | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| d0 judge r142 | 144 | 90% | 8.3 (3-14) | 1 / 16 | liberated 130, deck_empty 10, research 3, discord_deck 1 | 3; 37 | 20 / 59 | 27 / 66 | 15 / 31 |
| d0 judge r143 | 144 | 87% | 8.6 (4-14) | 0 / 16 | liberated 125, deck_empty 19 | 2; 20 | 18 / 55 | 26 / 51 | 12 / 28 |
| d0 greedy r142 | 144 | 94% | 7.9 (3-14) | 2 / 24 | liberated 135, deck_empty 8, research 1 | 3; 25 | 17 / 54 | 23 / 55 | 14 / 31 |
| d0 greedy r143 | 144 | 93% | 7.9 (4-14) | 0 / 21 | liberated 134, deck_empty 9, research 1 | 2; 13 | 15 / 54 | 22 / 55 | 12 / 29 |
| d1 judge r142 | 144 | 83% | 9.0 (5-14) | 0 / 13 | liberated 120, deck_empty 14, research 6, discord_deck 4 | 3; 65 | 19 / 55 | 36 / 66 | 17 / 34 |
| d1 judge r143 | 144 | 87% | 8.9 (4-14) | 0 / 13 | liberated 125, deck_empty 15, research 3, discord_deck 1 | 3; 37 | 17 / 50 | 33 / 67 | 14 / 34 |
| d1 greedy r142 | 144 | 85% | 8.7 (4-14) | 0 / 6 | liberated 123, deck_empty 11, research 7, discord_deck 3 | 3; 50 | 17 / 46 | 32 / 66 | 16 / 33 |
| d1 greedy r143 | 144 | 91% | 8.7 (4-14) | 0 / 13 | liberated 131, deck_empty 12, research 1 | 2; 35 | 16 / 56 | 29 / 64 | 14 / 30 |
| d2 judge r142 | 144 | 67% | 8.7 (4-14) | 0 / 9 | liberated 96, deck_empty 14, research 24, discord_deck 10 | 4; 92 | 19 / 56 | 40 / 67 | 20 / 42 |
| d2 judge r143 | 144 | 79% | 8.7 (4-14) | 0 / 11 | liberated 114, deck_empty 12, research 10, discord_deck 8 | 3; 65 | 17 / 53 | 38 / 69 | 17 / 42 |
| d2 greedy r142 | 144 | 77% | 8.7 (4-14) | 0 / 9 | liberated 111, deck_empty 9, research 13, discord_deck 11 | 4; 75 | 18 / 45 | 37 / 67 | 19 / 39 |
| d2 greedy r143 | 144 | 81% | 8.8 (4-14) | 0 / 7 | liberated 117, deck_empty 11, research 11, discord_deck 5 | 3; 54 | 17 / 49 | 34 / 73 | 16 / 37 |
| d3 judge r142 | 144 | 51% | 8.3 (4-12) | 0 / 5 | liberated 73, deck_empty 16, research 42, discord_deck 13 | 5; 106 | 18 / 54 | 43 / 71 | 23 / 46 |
| d3 judge r143 | 144 | 62% | 8.9 (5-12) | 0 / 3 | liberated 90, deck_empty 22, research 23, discord_deck 9 | 4; 75 | 19 / 49 | 42 / 70 | 20 / 43 |
| d3 greedy r142 | 144 | 44% | 9.0 (5-12) | 0 / 3 | liberated 64, deck_empty 23, research 46, discord_deck 11 | 5; 110 | 19 / 48 | 43 / 74 | 24 / 40 |
| d3 greedy r143 | 144 | 62% | 9.0 (4-12) | 0 / 3 | liberated 89, deck_empty 27, research 23, discord_deck 5 | 4; 86 | 19 / 50 | 39 / 68 | 20 / 43 |

| Read | Games with a troop | First troop, round: mean (min-max) | Games with one by round 3 | Troops placed a game, by source | Strike icons a game | Strike hits a game | Redeploy icons / troops moved a game | Games a troop alone blocked a Liberate | Remove a troop a game | Discord taken off by Unity cards a game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 judge r142 | 41 | 6.9 (4-12) | 0 | project 0.06, research 0.56 | 1.30 | 0.02 | 0.51 / 0.00 | 5 | 0.25 | 0.0 |
| d0 judge r143 | 32 | 7.2 (4-14) | 0 | discord 0.19, research 0.25 | 1.85 | 0.01 | 0.50 / 0.01 | 2 | 0.12 | 2.1 |
| d0 greedy r142 | 30 | 6.6 (4-11) | 0 | discord 0.01, project 0.10, research 0.40 | 1.06 | 0.00 | 0.41 / 0.01 | 4 | 0.22 | 0.0 |
| d0 greedy r143 | 19 | 7.0 (5-10) | 0 | discord 0.11, research 0.20 | 1.38 | 0.03 | 0.46 / 0.01 | 2 | 0.12 | 2.3 |
| d1 judge r142 | 78 | 6.7 (2-13) | 8 | discord 0.08, project 0.49, research 1.01 | 1.78 | 0.09 | 0.61 / 0.02 | 13 | 0.55 | 0.0 |
| d1 judge r143 | 70 | 7.1 (3-12) | 3 | discord 0.50, project 0.19, research 0.54 | 1.99 | 0.10 | 0.59 / 0.02 | 8 | 0.39 | 2.5 |
| d1 greedy r142 | 67 | 6.4 (2-11) | 7 | discord 0.07, project 0.50, research 0.81 | 1.49 | 0.12 | 0.53 / 0.01 | 12 | 0.51 | 0.0 |
| d1 greedy r143 | 59 | 7.0 (4-13) | 0 | discord 0.40, project 0.19, research 0.44 | 1.88 | 0.08 | 0.52 / 0.01 | 12 | 0.33 | 2.3 |
| d2 judge r142 | 109 | 5.6 (2-11) | 16 | discord 0.13, project 1.21, research 1.52 | 1.80 | 0.25 | 0.48 / 0.03 | 28 | 0.76 | 0.0 |
| d2 judge r143 | 100 | 6.1 (2-12) | 7 | discord 0.62, project 0.88, research 1.01 | 1.99 | 0.22 | 0.55 / 0.01 | 24 | 0.74 | 2.4 |
| d2 greedy r142 | 92 | 5.6 (2-10) | 16 | discord 0.13, project 1.16, research 1.28 | 1.57 | 0.28 | 0.53 / 0.00 | 27 | 0.76 | 0.0 |
| d2 greedy r143 | 93 | 5.9 (2-9) | 6 | discord 0.53, project 0.92, research 0.81 | 1.94 | 0.18 | 0.56 / 0.00 | 20 | 0.74 | 2.8 |
| d3 judge r142 | 119 | 4.8 (1-10) | 24 | discord 0.15, project 1.50, research 2.02 | 1.99 | 0.40 | 0.52 / 0.01 | 39 | 1.03 | 0.0 |
| d3 judge r143 | 111 | 5.5 (2-10) | 8 | discord 0.74, project 1.66, research 1.33 | 2.60 | 0.38 | 0.62 / 0.02 | 29 | 1.06 | 2.9 |
| d3 greedy r142 | 126 | 5.3 (1-11) | 24 | discord 0.15, project 1.42, research 2.16 | 2.43 | 0.37 | 0.67 / 0.02 | 35 | 1.38 | 0.0 |
| d3 greedy r143 | 109 | 5.6 (2-11) | 7 | discord 0.69, project 1.60, research 1.35 | 2.67 | 0.42 | 0.70 / 0.01 | 33 | 1.12 | 2.8 |

## Fresh seeds (50640-50687), the three pairs pooled, 144 games a row

The r142 side was farmed at 2282b5387d on the same seeds. The same-seed table above is the read this item's
content was shaped on (the Botnet Conscripts condition, the troop lines that land by the players, Breach and
Clear); this one saw no change after it.

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 / 5 | How games ended | Slot reached: median; games at slot 4+ | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| d0 judge r142 | 144 | 87% | 8.4 (3-14) | 2 / 13 | liberated 125, deck_empty 14, research 5 | 3; 55 | 20 / 64 | 28 / 55 | 17 / 31 |
| d0 judge r143 | 144 | 92% | 8.1 (3-14) | 1 / 22 | liberated 133, deck_empty 9, research 2 | 2; 28 | 17 / 56 | 26 / 55 | 14 / 29 |
| d0 greedy r142 | 144 | 91% | 8.0 (3-14) | 2 / 12 | liberated 131, deck_empty 11, research 2 | 2; 28 | 16 / 65 | 23 / 49 | 14 / 30 |
| d0 greedy r143 | 144 | 94% | 8.2 (3-14) | 1 / 9 | liberated 136, deck_empty 8 | 2; 16 | 15 / 70 | 22 / 44 | 12 / 27 |
| d1 judge r142 | 144 | 81% | 8.8 (4-14) | 0 / 11 | liberated 116, deck_empty 15, research 11, discord_deck 2 | 3; 68 | 19 / 64 | 33 / 66 | 18 / 40 |
| d1 judge r143 | 144 | 86% | 8.7 (4-14) | 0 / 14 | liberated 124, deck_empty 12, research 4, discord_deck 4 | 3; 48 | 16 / 63 | 30 / 66 | 16 / 37 |
| d1 greedy r142 | 144 | 86% | 8.8 (5-14) | 0 / 5 | liberated 124, deck_empty 7, research 13 | 3; 70 | 17 / 51 | 31 / 62 | 18 / 34 |
| d1 greedy r143 | 144 | 92% | 8.7 (5-14) | 0 / 5 | liberated 132, deck_empty 11, research 1 | 2; 39 | 15 / 50 | 27 / 52 | 14 / 39 |
| d2 judge r142 | 144 | 65% | 8.4 (4-14) | 0 / 7 | liberated 93, deck_empty 6, research 39, discord_deck 6 | 4; 96 | 18 / 69 | 38 / 69 | 22 / 45 |
| d2 judge r143 | 144 | 77% | 8.6 (5-14) | 0 / 1 | liberated 111, deck_empty 9, research 20, discord_deck 4 | 4; 82 | 17 / 63 | 35 / 69 | 20 / 42 |
| d2 greedy r142 | 144 | 78% | 8.4 (4-14) | 0 / 4 | liberated 112, deck_empty 3, research 28, discord_deck 1 | 4; 83 | 16 / 43 | 35 / 66 | 21 / 40 |
| d2 greedy r143 | 144 | 81% | 8.7 (5-14) | 0 / 4 | liberated 117, deck_empty 8, research 16, discord_deck 3 | 3; 67 | 16 / 48 | 32 / 67 | 18 / 38 |
| d3 judge r142 | 144 | 49% | 8.0 (4-12) | 0 / 7 | liberated 70, deck_empty 10, research 56, discord_deck 7, node 1 | 5; 116 | 17 / 47 | 41 / 70 | 25 / 43 |
| d3 judge r143 | 144 | 65% | 8.4 (5-12) | 0 / 5 | liberated 93, deck_empty 14, research 34, discord_deck 3 | 4; 93 | 18 / 46 | 39 / 74 | 22 / 38 |
| d3 greedy r142 | 144 | 47% | 8.6 (5-12) | 0 / 2 | liberated 68, deck_empty 14, research 58, discord_deck 4 | 6; 111 | 17 / 47 | 41 / 66 | 26 / 51 |
| d3 greedy r143 | 144 | 62% | 8.8 (5-12) | 0 / 2 | liberated 89, deck_empty 21, research 30, discord_deck 4 | 4; 93 | 17 / 48 | 38 / 70 | 22 / 45 |

| Read | Games with a troop | First troop, round: mean (min-max) | Games with one by round 3 | Troops placed a game, by source | Strike icons a game | Strike hits a game | Redeploy icons / troops moved a game | Games a troop alone blocked a Liberate | Remove a troop a game | Discord taken off by Unity cards a game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 judge r142 | 55 | 7.0 (5-13) | 0 | discord 0.01, project 0.25, research 0.81 | 1.37 | 0.11 | 0.57 / 0.02 | 10 | 0.30 | 0.0 |
| d0 judge r143 | 54 | 6.2 (3-12) | 4 | discord 0.29, project 0.14, research 0.37 | 1.53 | 0.10 | 0.47 / 0.00 | 5 | 0.24 | 2.5 |
| d0 greedy r142 | 28 | 7.0 (5-9) | 0 | project 0.14, research 0.38 | 1.10 | 0.03 | 0.44 / 0.01 | 2 | 0.17 | 0.0 |
| d0 greedy r143 | 30 | 6.3 (3-12) | 2 | discord 0.18, project 0.08, research 0.17 | 1.52 | 0.06 | 0.51 / 0.01 | 4 | 0.19 | 2.2 |
| d1 judge r142 | 74 | 6.5 (3-12) | 4 | discord 0.01, project 0.46, research 1.10 | 1.57 | 0.12 | 0.59 / 0.00 | 10 | 0.51 | 0.0 |
| d1 judge r143 | 76 | 5.7 (2-13) | 7 | discord 0.35, project 0.34, research 0.68 | 1.96 | 0.08 | 0.65 / 0.00 | 11 | 0.41 | 2.7 |
| d1 greedy r142 | 78 | 6.8 (3-12) | 6 | discord 0.03, project 0.40, research 1.11 | 1.60 | 0.12 | 0.62 / 0.01 | 15 | 0.60 | 0.0 |
| d1 greedy r143 | 65 | 6.6 (2-12) | 3 | discord 0.20, project 0.32, research 0.53 | 1.67 | 0.06 | 0.60 / 0.00 | 8 | 0.31 | 2.7 |
| d2 judge r142 | 112 | 5.8 (2-13) | 6 | discord 0.06, project 1.14, research 1.76 | 1.55 | 0.22 | 0.49 / 0.00 | 20 | 0.87 | 0.0 |
| d2 judge r143 | 109 | 5.3 (2-10) | 17 | discord 0.63, project 0.92, research 1.40 | 1.98 | 0.37 | 0.56 / 0.01 | 29 | 0.92 | 2.8 |
| d2 greedy r142 | 95 | 6.0 (2-12) | 6 | discord 0.06, project 0.91, research 1.50 | 1.55 | 0.35 | 0.46 / 0.01 | 18 | 0.76 | 0.0 |
| d2 greedy r143 | 98 | 5.8 (2-13) | 15 | discord 0.44, project 0.69, research 1.14 | 1.97 | 0.32 | 0.61 / 0.01 | 15 | 0.83 | 2.9 |
| d3 judge r142 | 133 | 4.8 (1-9) | 23 | discord 0.11, project 1.97, research 2.23 | 1.82 | 0.44 | 0.47 / 0.04 | 44 | 1.17 | 0.0 |
| d3 judge r143 | 122 | 4.8 (2-9) | 23 | discord 0.71, project 1.70, research 1.80 | 2.28 | 0.55 | 0.56 / 0.01 | 30 | 1.25 | 2.6 |
| d3 greedy r142 | 126 | 5.2 (1-10) | 18 | discord 0.07, project 1.85, research 2.33 | 2.19 | 0.43 | 0.64 / 0.02 | 39 | 1.43 | 0.0 |
| d3 greedy r143 | 121 | 5.2 (2-10) | 21 | discord 0.65, project 1.70, research 1.68 | 2.53 | 0.64 | 0.64 / 0.01 | 36 | 1.38 | 2.5 |

## By pair, same seeds

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 / 5 | How games ended | Slot reached: median; games at slot 4+ | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| d0 judge r142 bp | 48 | 90% | 8.2 (4-14) | 0 / 5 | liberated 43, deck_empty 2, research 2, discord_deck 1 | 3; 15 | 18 / 43 | 27 / 66 | 15 / 31 |
| d0 judge r142 br | 48 | 94% | 8.2 (3-14) | 1 / 7 | liberated 45, deck_empty 2, research 1 | 3; 12 | 19 / 50 | 28 / 48 | 15 / 30 |
| d0 judge r142 pr | 48 | 88% | 8.5 (5-14) | 0 / 4 | liberated 42, deck_empty 6 | 3; 10 | 22 / 59 | 26 / 37 | 14 / 27 |
| d0 judge r142 | 144 | 90% | 8.3 (3-14) | 1 / 16 | liberated 130, deck_empty 10, research 3, discord_deck 1 | 3; 37 | 20 / 59 | 27 / 66 | 15 / 31 |
| d0 judge r143 bp | 48 | 88% | 8.9 (4-14) | 0 / 2 | liberated 42, deck_empty 6 | 3; 11 | 18 / 45 | 28 / 51 | 14 / 28 |
| d0 judge r143 br | 48 | 85% | 8.7 (4-14) | 0 / 8 | liberated 41, deck_empty 7 | 2; 6 | 20 / 55 | 26 / 46 | 13 / 26 |
| d0 judge r143 pr | 48 | 88% | 8.2 (4-14) | 0 / 6 | liberated 42, deck_empty 6 | 2; 3 | 17 / 46 | 23 / 47 | 11 / 24 |
| d0 judge r143 | 144 | 87% | 8.6 (4-14) | 0 / 16 | liberated 125, deck_empty 19 | 2; 20 | 18 / 55 | 26 / 51 | 12 / 28 |
| d0 greedy r142 bp | 48 | 98% | 7.9 (4-13) | 0 / 8 | liberated 47, research 1 | 3; 12 | 15 / 35 | 24 / 49 | 14 / 31 |
| d0 greedy r142 br | 48 | 88% | 8.1 (3-14) | 2 / 9 | liberated 42, deck_empty 6 | 3; 6 | 17 / 42 | 23 / 55 | 14 / 28 |
| d0 greedy r142 pr | 48 | 96% | 7.6 (4-14) | 0 / 7 | liberated 46, deck_empty 2 | 2; 7 | 18 / 54 | 21 / 45 | 13 / 25 |
| d0 greedy r142 | 144 | 94% | 7.9 (3-14) | 2 / 24 | liberated 135, deck_empty 8, research 1 | 3; 25 | 17 / 54 | 23 / 55 | 14 / 31 |
| d0 greedy r143 bp | 48 | 96% | 8.3 (4-14) | 0 / 4 | liberated 46, deck_empty 1, research 1 | 2; 5 | 16 / 54 | 25 / 42 | 13 / 29 |
| d0 greedy r143 br | 48 | 85% | 8.6 (5-14) | 0 / 7 | liberated 41, deck_empty 7 | 2; 7 | 16 / 41 | 23 / 55 | 12 / 23 |
| d0 greedy r143 pr | 48 | 98% | 6.9 (4-14) | 0 / 10 | liberated 47, deck_empty 1 | 2; 1 | 13 / 37 | 18 / 35 | 10 / 19 |
| d0 greedy r143 | 144 | 93% | 7.9 (4-14) | 0 / 21 | liberated 134, deck_empty 9, research 1 | 2; 13 | 15 / 54 | 22 / 55 | 12 / 29 |
| d1 judge r142 bp | 48 | 79% | 9.5 (5-14) | 0 / 2 | liberated 38, deck_empty 5, research 3, discord_deck 2 | 4; 26 | 21 / 48 | 39 / 66 | 19 / 33 |
| d1 judge r142 br | 48 | 85% | 9.1 (5-14) | 0 / 7 | liberated 41, deck_empty 4, research 2, discord_deck 1 | 3; 23 | 18 / 44 | 37 / 64 | 18 / 34 |
| d1 judge r142 pr | 48 | 85% | 8.4 (5-14) | 0 / 4 | liberated 41, deck_empty 5, research 1, discord_deck 1 | 3; 16 | 19 / 55 | 31 / 65 | 16 / 32 |
| d1 judge r142 | 144 | 83% | 9.0 (5-14) | 0 / 13 | liberated 120, deck_empty 14, research 6, discord_deck 4 | 3; 65 | 19 / 55 | 36 / 66 | 17 / 34 |
| d1 judge r143 bp | 48 | 83% | 9.2 (4-14) | 0 / 2 | liberated 40, deck_empty 6, research 1, discord_deck 1 | 3; 15 | 17 / 40 | 34 / 67 | 15 / 29 |
| d1 judge r143 br | 48 | 85% | 9.2 (5-14) | 0 / 2 | liberated 41, deck_empty 5, research 2 | 3; 12 | 18 / 50 | 35 / 63 | 16 / 34 |
| d1 judge r143 pr | 48 | 92% | 8.2 (4-14) | 0 / 9 | liberated 44, deck_empty 4 | 2; 10 | 15 / 39 | 30 / 60 | 12 / 25 |
| d1 judge r143 | 144 | 87% | 8.9 (4-14) | 0 / 13 | liberated 125, deck_empty 15, research 3, discord_deck 1 | 3; 37 | 17 / 50 | 33 / 67 | 14 / 34 |
| d1 greedy r142 bp | 48 | 79% | 9.0 (4-14) | 0 / 3 | liberated 38, deck_empty 4, research 4, discord_deck 2 | 3; 20 | 17 / 44 | 34 / 66 | 17 / 33 |
| d1 greedy r142 br | 48 | 81% | 9.2 (5-14) | 0 / 1 | liberated 39, deck_empty 5, research 3, discord_deck 1 | 3; 19 | 19 / 46 | 33 / 66 | 17 / 32 |
| d1 greedy r142 pr | 48 | 96% | 7.9 (5-14) | 0 / 2 | liberated 46, deck_empty 2 | 3; 11 | 16 / 42 | 28 / 53 | 14 / 27 |
| d1 greedy r142 | 144 | 85% | 8.7 (4-14) | 0 / 6 | liberated 123, deck_empty 11, research 7, discord_deck 3 | 3; 50 | 17 / 46 | 32 / 66 | 16 / 33 |
| d1 greedy r143 bp | 48 | 94% | 8.9 (4-14) | 0 / 4 | liberated 45, deck_empty 3 | 3; 15 | 16 / 50 | 31 / 60 | 14 / 25 |
| d1 greedy r143 br | 48 | 90% | 8.9 (5-14) | 0 / 2 | liberated 43, deck_empty 4, research 1 | 3; 12 | 17 / 56 | 31 / 64 | 15 / 30 |
| d1 greedy r143 pr | 48 | 90% | 8.4 (5-14) | 0 / 7 | liberated 43, deck_empty 5 | 2; 8 | 16 / 47 | 27 / 50 | 12 / 28 |
| d1 greedy r143 | 144 | 91% | 8.7 (4-14) | 0 / 13 | liberated 131, deck_empty 12, research 1 | 2; 35 | 16 / 56 | 29 / 64 | 14 / 30 |
| d2 judge r142 bp | 48 | 65% | 8.8 (5-14) | 0 / 2 | liberated 31, deck_empty 5, research 9, discord_deck 3 | 4; 33 | 20 / 47 | 42 / 67 | 22 / 37 |
| d2 judge r142 br | 48 | 71% | 8.4 (4-14) | 0 / 2 | liberated 34, deck_empty 1, research 9, discord_deck 4 | 4; 29 | 18 / 56 | 40 / 66 | 21 / 35 |
| d2 judge r142 pr | 48 | 65% | 8.9 (4-14) | 0 / 5 | liberated 31, deck_empty 8, research 6, discord_deck 3 | 4; 30 | 20 / 52 | 39 / 65 | 19 / 42 |
| d2 judge r142 | 144 | 67% | 8.7 (4-14) | 0 / 9 | liberated 96, deck_empty 14, research 24, discord_deck 10 | 4; 92 | 19 / 56 | 40 / 67 | 20 / 42 |
| d2 judge r143 bp | 48 | 73% | 9.1 (5-14) | 0 / 1 | liberated 35, deck_empty 3, research 6, discord_deck 4 | 4; 25 | 17 / 44 | 39 / 68 | 18 / 42 |
| d2 judge r143 br | 48 | 79% | 9.1 (4-14) | 0 / 4 | liberated 38, deck_empty 6, research 1, discord_deck 3 | 3; 23 | 18 / 46 | 39 / 67 | 18 / 39 |
| d2 judge r143 pr | 48 | 85% | 7.9 (4-14) | 0 / 6 | liberated 41, deck_empty 3, research 3, discord_deck 1 | 3; 17 | 16 / 53 | 34 / 69 | 15 / 31 |
| d2 judge r143 | 144 | 79% | 8.7 (4-14) | 0 / 11 | liberated 114, deck_empty 12, research 10, discord_deck 8 | 3; 65 | 17 / 53 | 38 / 69 | 17 / 42 |
| d2 greedy r142 bp | 48 | 67% | 9.2 (4-14) | 0 / 2 | liberated 32, deck_empty 3, research 8, discord_deck 5 | 4; 33 | 18 / 44 | 41 / 67 | 21 / 38 |
| d2 greedy r142 br | 48 | 79% | 8.9 (4-14) | 0 / 2 | liberated 38, deck_empty 4, research 2, discord_deck 4 | 3; 23 | 18 / 45 | 36 / 65 | 19 / 34 |
| d2 greedy r142 pr | 48 | 85% | 8.1 (4-14) | 0 / 5 | liberated 41, deck_empty 2, research 3, discord_deck 2 | 3; 19 | 17 / 33 | 33 / 67 | 17 / 39 |
| d2 greedy r142 | 144 | 77% | 8.7 (4-14) | 0 / 9 | liberated 111, deck_empty 9, research 13, discord_deck 11 | 4; 75 | 18 / 45 | 37 / 67 | 19 / 39 |
| d2 greedy r143 bp | 48 | 75% | 9.2 (5-14) | 0 / 2 | liberated 36, deck_empty 5, research 3, discord_deck 4 | 3; 20 | 17 / 49 | 37 / 73 | 18 / 35 |
| d2 greedy r143 br | 48 | 81% | 8.9 (5-14) | 0 / 1 | liberated 39, deck_empty 3, research 5, discord_deck 1 | 3; 16 | 17 / 46 | 34 / 69 | 17 / 37 |
| d2 greedy r143 pr | 48 | 88% | 8.3 (4-14) | 0 / 4 | liberated 42, deck_empty 3, research 3 | 3; 18 | 16 / 36 | 32 / 55 | 15 / 33 |
| d2 greedy r143 | 144 | 81% | 8.8 (4-14) | 0 / 7 | liberated 117, deck_empty 11, research 11, discord_deck 5 | 3; 54 | 17 / 49 | 34 / 73 | 16 / 37 |
| d3 judge r142 bp | 48 | 38% | 8.6 (5-12) | 0 / 1 | liberated 18, deck_empty 6, research 17, discord_deck 7 | 6; 40 | 18 / 35 | 46 / 71 | 26 / 46 |
| d3 judge r142 br | 48 | 58% | 8.2 (5-12) | 0 / 1 | liberated 28, deck_empty 3, research 14, discord_deck 3 | 5; 33 | 18 / 35 | 43 / 66 | 23 / 39 |
| d3 judge r142 pr | 48 | 56% | 8.2 (4-12) | 0 / 3 | liberated 27, deck_empty 7, research 11, discord_deck 3 | 5; 33 | 19 / 54 | 41 / 64 | 22 / 34 |
| d3 judge r142 | 144 | 51% | 8.3 (4-12) | 0 / 5 | liberated 73, deck_empty 16, research 42, discord_deck 13 | 5; 106 | 18 / 54 | 43 / 71 | 23 / 46 |
| d3 judge r143 bp | 48 | 52% | 9.1 (6-12) | 0 / 0 | liberated 25, deck_empty 8, research 12, discord_deck 3 | 4; 31 | 19 / 49 | 44 / 70 | 22 / 43 |
| d3 judge r143 br | 48 | 58% | 8.9 (5-12) | 0 / 1 | liberated 28, deck_empty 6, research 10, discord_deck 4 | 4; 24 | 19 / 49 | 42 / 70 | 20 / 43 |
| d3 judge r143 pr | 48 | 77% | 8.7 (5-12) | 0 / 2 | liberated 37, deck_empty 8, research 1, discord_deck 2 | 3; 20 | 19 / 38 | 39 / 68 | 17 / 39 |
| d3 judge r143 | 144 | 62% | 8.9 (5-12) | 0 / 3 | liberated 90, deck_empty 22, research 23, discord_deck 9 | 4; 75 | 19 / 49 | 42 / 70 | 20 / 43 |
| d3 greedy r142 bp | 48 | 31% | 9.2 (6-12) | 0 / 0 | liberated 15, deck_empty 7, research 22, discord_deck 4 | 6; 37 | 19 / 38 | 45 / 67 | 26 / 40 |
| d3 greedy r142 br | 48 | 50% | 8.9 (5-12) | 0 / 1 | liberated 24, deck_empty 8, research 12, discord_deck 4 | 5; 40 | 20 / 44 | 45 / 74 | 25 / 38 |
| d3 greedy r142 pr | 48 | 52% | 8.7 (5-12) | 0 / 2 | liberated 25, deck_empty 8, research 12, discord_deck 3 | 5; 33 | 20 / 48 | 40 / 65 | 22 / 36 |
| d3 greedy r142 | 144 | 44% | 9.0 (5-12) | 0 / 3 | liberated 64, deck_empty 23, research 46, discord_deck 11 | 5; 110 | 19 / 48 | 43 / 74 | 24 / 40 |
| d3 greedy r143 bp | 48 | 44% | 9.6 (6-12) | 0 / 0 | liberated 21, deck_empty 13, research 11, discord_deck 3 | 4; 34 | 19 / 44 | 43 / 68 | 22 / 43 |
| d3 greedy r143 br | 48 | 67% | 9.1 (6-12) | 0 / 0 | liberated 32, deck_empty 8, research 8 | 4; 26 | 19 / 44 | 40 / 67 | 20 / 42 |
| d3 greedy r143 pr | 48 | 75% | 8.4 (4-12) | 0 / 3 | liberated 36, deck_empty 6, research 4, discord_deck 2 | 4; 26 | 19 / 50 | 35 / 66 | 17 / 30 |
| d3 greedy r143 | 144 | 62% | 9.0 (4-12) | 0 / 3 | liberated 89, deck_empty 27, research 23, discord_deck 5 | 4; 86 | 19 / 50 | 39 / 68 | 20 / 43 |

| Read | Games with a troop | First troop, round: mean (min-max) | Games with one by round 3 | Troops placed a game, by source | Strike icons a game | Strike hits a game | Redeploy icons / troops moved a game | Games a troop alone blocked a Liberate | Remove a troop a game | Discord taken off by Unity cards a game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 judge r142 bp | 17 | 7.2 (5-12) | 0 | project 0.06, research 0.79 | 1.33 | 0.02 | 0.54 / 0.00 | 5 | 0.33 | 0.0 |
| d0 judge r142 br | 12 | 6.7 (4-10) | 0 | project 0.04, research 0.58 | 1.23 | 0.04 | 0.50 / 0.00 | 0 | 0.31 | 0.0 |
| d0 judge r142 pr | 12 | 6.6 (4-10) | 0 | project 0.08, research 0.31 | 1.33 | 0.00 | 0.50 / 0.00 | 0 | 0.10 | 0.0 |
| d0 judge r142 | 41 | 6.9 (4-12) | 0 | project 0.06, research 0.56 | 1.30 | 0.02 | 0.51 / 0.00 | 5 | 0.25 | 0.0 |
| d0 judge r143 bp | 16 | 6.7 (4-10) | 0 | discord 0.31, research 0.38 | 1.92 | 0.00 | 0.58 / 0.02 | 2 | 0.17 | 2.0 |
| d0 judge r143 br | 9 | 7.2 (5-12) | 0 | discord 0.15, research 0.29 | 1.94 | 0.00 | 0.56 / 0.00 | 0 | 0.12 | 2.6 |
| d0 judge r143 pr | 7 | 8.3 (5-14) | 0 | discord 0.12, research 0.08 | 1.71 | 0.02 | 0.35 / 0.00 | 0 | 0.06 | 1.6 |
| d0 judge r143 | 32 | 7.2 (4-14) | 0 | discord 0.19, research 0.25 | 1.85 | 0.01 | 0.50 / 0.01 | 2 | 0.12 | 2.1 |
| d0 greedy r142 bp | 13 | 6.5 (4-9) | 0 | project 0.12, research 0.65 | 1.15 | 0.00 | 0.40 / 0.00 | 1 | 0.40 | 0.0 |
| d0 greedy r142 br | 8 | 7.4 (4-11) | 0 | discord 0.04, project 0.08, research 0.29 | 1.23 | 0.00 | 0.46 / 0.00 | 1 | 0.12 | 0.0 |
| d0 greedy r142 pr | 9 | 6.1 (4-9) | 0 | project 0.08, research 0.25 | 0.79 | 0.00 | 0.38 / 0.02 | 2 | 0.15 | 0.0 |
| d0 greedy r142 | 30 | 6.6 (4-11) | 0 | discord 0.01, project 0.10, research 0.40 | 1.06 | 0.00 | 0.41 / 0.01 | 4 | 0.22 | 0.0 |
| d0 greedy r143 bp | 8 | 7.1 (5-10) | 0 | discord 0.17, research 0.29 | 1.62 | 0.02 | 0.48 / 0.00 | 1 | 0.19 | 2.2 |
| d0 greedy r143 br | 9 | 7.0 (5-8) | 0 | discord 0.12, research 0.29 | 1.77 | 0.08 | 0.60 / 0.02 | 1 | 0.15 | 2.5 |
| d0 greedy r143 pr | 2 | 6.5 (6-7) | 0 | discord 0.04, research 0.02 | 0.73 | 0.00 | 0.29 / 0.00 | 0 | 0.02 | 2.1 |
| d0 greedy r143 | 19 | 7.0 (5-10) | 0 | discord 0.11, research 0.20 | 1.38 | 0.03 | 0.46 / 0.01 | 2 | 0.12 | 2.3 |
| d1 judge r142 bp | 31 | 6.6 (3-12) | 3 | discord 0.17, project 0.54, research 1.29 | 1.94 | 0.17 | 0.67 / 0.04 | 7 | 0.83 | 0.0 |
| d1 judge r142 br | 27 | 6.8 (2-12) | 2 | discord 0.04, project 0.52, research 1.21 | 1.96 | 0.08 | 0.71 / 0.00 | 4 | 0.46 | 0.0 |
| d1 judge r142 pr | 20 | 6.5 (2-13) | 3 | discord 0.02, project 0.42, research 0.54 | 1.44 | 0.02 | 0.46 / 0.02 | 2 | 0.35 | 0.0 |
| d1 judge r142 | 78 | 6.7 (2-13) | 8 | discord 0.08, project 0.49, research 1.01 | 1.78 | 0.09 | 0.61 / 0.02 | 13 | 0.55 | 0.0 |
| d1 judge r143 bp | 26 | 7.0 (3-10) | 1 | discord 0.58, project 0.17, research 0.58 | 2.08 | 0.08 | 0.69 / 0.04 | 4 | 0.58 | 2.1 |
| d1 judge r143 br | 27 | 7.2 (3-12) | 1 | discord 0.71, project 0.27, research 0.71 | 2.25 | 0.21 | 0.65 / 0.02 | 4 | 0.50 | 2.7 |
| d1 judge r143 pr | 17 | 7.1 (3-11) | 1 | discord 0.21, project 0.12, research 0.33 | 1.62 | 0.00 | 0.44 / 0.00 | 0 | 0.08 | 2.6 |
| d1 judge r143 | 70 | 7.1 (3-12) | 3 | discord 0.50, project 0.19, research 0.54 | 1.99 | 0.10 | 0.59 / 0.02 | 8 | 0.39 | 2.5 |
| d1 greedy r142 bp | 27 | 6.8 (2-11) | 3 | discord 0.08, project 0.56, research 1.12 | 1.67 | 0.12 | 0.60 / 0.02 | 7 | 0.77 | 0.0 |
| d1 greedy r142 br | 24 | 6.5 (2-9) | 2 | discord 0.08, project 0.58, research 0.92 | 1.75 | 0.15 | 0.60 / 0.00 | 5 | 0.54 | 0.0 |
| d1 greedy r142 pr | 16 | 5.9 (2-8) | 2 | discord 0.04, project 0.35, research 0.38 | 1.04 | 0.08 | 0.40 / 0.00 | 0 | 0.23 | 0.0 |
| d1 greedy r142 | 67 | 6.4 (2-11) | 7 | discord 0.07, project 0.50, research 0.81 | 1.49 | 0.12 | 0.53 / 0.01 | 12 | 0.51 | 0.0 |
| d1 greedy r143 bp | 24 | 7.3 (4-13) | 0 | discord 0.40, project 0.17, research 0.44 | 1.92 | 0.04 | 0.52 / 0.00 | 3 | 0.52 | 2.2 |
| d1 greedy r143 br | 22 | 6.7 (4-12) | 0 | discord 0.62, project 0.23, research 0.62 | 2.02 | 0.17 | 0.56 / 0.02 | 8 | 0.35 | 2.2 |
| d1 greedy r143 pr | 13 | 7.0 (5-12) | 0 | discord 0.17, project 0.19, research 0.25 | 1.69 | 0.02 | 0.48 / 0.00 | 1 | 0.12 | 2.6 |
| d1 greedy r143 | 59 | 7.0 (4-13) | 0 | discord 0.40, project 0.19, research 0.44 | 1.88 | 0.08 | 0.52 / 0.01 | 12 | 0.33 | 2.3 |
| d2 judge r142 bp | 38 | 5.6 (2-10) | 6 | discord 0.17, project 1.19, research 1.81 | 1.90 | 0.31 | 0.50 / 0.06 | 8 | 1.12 | 0.0 |
| d2 judge r142 br | 36 | 5.8 (2-11) | 5 | discord 0.17, project 1.33, research 1.71 | 1.65 | 0.40 | 0.46 / 0.02 | 12 | 0.75 | 0.0 |
| d2 judge r142 pr | 35 | 5.5 (2-11) | 5 | discord 0.06, project 1.10, research 1.04 | 1.85 | 0.04 | 0.48 / 0.00 | 8 | 0.40 | 0.0 |
| d2 judge r142 | 109 | 5.6 (2-11) | 16 | discord 0.13, project 1.21, research 1.52 | 1.80 | 0.25 | 0.48 / 0.03 | 28 | 0.76 | 0.0 |
| d2 judge r143 bp | 39 | 6.3 (2-12) | 3 | discord 0.79, project 0.75, research 1.21 | 2.25 | 0.23 | 0.60 / 0.00 | 12 | 1.04 | 2.1 |
| d2 judge r143 br | 33 | 6.1 (2-11) | 2 | discord 0.75, project 1.12, research 1.25 | 2.23 | 0.31 | 0.65 / 0.00 | 9 | 0.83 | 2.6 |
| d2 judge r143 pr | 28 | 5.8 (2-12) | 2 | discord 0.31, project 0.75, research 0.56 | 1.48 | 0.10 | 0.40 / 0.04 | 3 | 0.35 | 2.5 |
| d2 judge r143 | 100 | 6.1 (2-12) | 7 | discord 0.62, project 0.88, research 1.01 | 1.99 | 0.22 | 0.55 / 0.01 | 24 | 0.74 | 2.4 |
| d2 greedy r142 bp | 36 | 5.8 (2-10) | 6 | discord 0.15, project 1.02, research 1.73 | 1.81 | 0.19 | 0.60 / 0.00 | 11 | 1.02 | 0.0 |
| d2 greedy r142 br | 29 | 5.8 (2-9) | 5 | discord 0.17, project 1.46, research 1.33 | 1.71 | 0.44 | 0.52 / 0.00 | 10 | 0.81 | 0.0 |
| d2 greedy r142 pr | 27 | 5.3 (2-8) | 5 | discord 0.08, project 1.00, research 0.79 | 1.19 | 0.21 | 0.48 / 0.00 | 6 | 0.46 | 0.0 |
| d2 greedy r142 | 92 | 5.6 (2-10) | 16 | discord 0.13, project 1.16, research 1.28 | 1.57 | 0.28 | 0.53 / 0.00 | 27 | 0.76 | 0.0 |
| d2 greedy r143 bp | 37 | 6.2 (2-9) | 2 | discord 0.73, project 0.71, research 0.92 | 2.27 | 0.15 | 0.62 / 0.00 | 10 | 1.08 | 2.5 |
| d2 greedy r143 br | 30 | 5.9 (2-9) | 2 | discord 0.58, project 1.23, research 0.92 | 1.98 | 0.31 | 0.58 / 0.00 | 6 | 0.83 | 2.6 |
| d2 greedy r143 pr | 26 | 5.5 (2-8) | 2 | discord 0.27, project 0.81, research 0.58 | 1.58 | 0.08 | 0.46 / 0.00 | 4 | 0.29 | 3.1 |
| d2 greedy r143 | 93 | 5.9 (2-9) | 6 | discord 0.53, project 0.92, research 0.81 | 1.94 | 0.18 | 0.56 / 0.00 | 20 | 0.74 | 2.8 |
| d3 judge r142 bp | 42 | 5.1 (1-10) | 8 | discord 0.19, project 1.52, research 2.40 | 2.27 | 0.52 | 0.60 / 0.00 | 16 | 1.42 | 0.0 |
| d3 judge r142 br | 38 | 4.6 (1-8) | 8 | discord 0.17, project 1.60, research 2.21 | 1.81 | 0.52 | 0.50 / 0.04 | 15 | 1.02 | 0.0 |
| d3 judge r142 pr | 39 | 4.8 (1-10) | 8 | discord 0.08, project 1.38, research 1.46 | 1.90 | 0.17 | 0.46 / 0.00 | 8 | 0.67 | 0.0 |
| d3 judge r142 | 119 | 4.8 (1-10) | 24 | discord 0.15, project 1.50, research 2.02 | 1.99 | 0.40 | 0.52 / 0.01 | 39 | 1.03 | 0.0 |
| d3 judge r143 bp | 39 | 5.6 (2-10) | 2 | discord 0.92, project 1.69, research 1.77 | 2.81 | 0.46 | 0.71 / 0.02 | 17 | 1.62 | 3.0 |
| d3 judge r143 br | 38 | 5.5 (2-10) | 3 | discord 0.81, project 2.12, research 1.54 | 2.58 | 0.54 | 0.56 / 0.02 | 9 | 1.02 | 2.5 |
| d3 judge r143 pr | 34 | 5.4 (2-9) | 3 | discord 0.50, project 1.17, research 0.69 | 2.40 | 0.12 | 0.58 / 0.02 | 3 | 0.52 | 3.2 |
| d3 judge r143 | 111 | 5.5 (2-10) | 8 | discord 0.74, project 1.66, research 1.33 | 2.60 | 0.38 | 0.62 / 0.02 | 29 | 1.06 | 2.9 |
| d3 greedy r142 bp | 44 | 5.2 (1-11) | 8 | discord 0.21, project 1.44, research 2.40 | 2.67 | 0.31 | 0.77 / 0.04 | 17 | 1.71 | 0.0 |
| d3 greedy r142 br | 42 | 5.2 (1-10) | 8 | discord 0.17, project 1.56, research 2.54 | 2.44 | 0.56 | 0.65 / 0.00 | 13 | 1.58 | 0.0 |
| d3 greedy r142 pr | 40 | 5.3 (1-10) | 8 | discord 0.08, project 1.25, research 1.54 | 2.19 | 0.23 | 0.60 / 0.02 | 5 | 0.85 | 0.0 |
| d3 greedy r142 | 126 | 5.3 (1-11) | 24 | discord 0.15, project 1.42, research 2.16 | 2.43 | 0.37 | 0.67 / 0.02 | 35 | 1.38 | 0.0 |
| d3 greedy r143 bp | 40 | 5.9 (2-9) | 2 | discord 0.85, project 1.81, research 1.56 | 3.17 | 0.54 | 0.85 / 0.02 | 15 | 1.46 | 2.7 |
| d3 greedy r143 br | 35 | 5.4 (2-8) | 2 | discord 0.81, project 1.90, research 1.58 | 2.73 | 0.62 | 0.71 / 0.00 | 11 | 1.42 | 2.9 |
| d3 greedy r143 pr | 34 | 5.5 (2-11) | 3 | discord 0.42, project 1.08, research 0.92 | 2.10 | 0.10 | 0.54 / 0.00 | 7 | 0.48 | 2.7 |
| d3 greedy r143 | 109 | 5.6 (2-11) | 7 | discord 0.69, project 1.60, research 1.35 | 2.67 | 0.42 | 0.70 / 0.01 | 33 | 1.12 | 2.8 |
