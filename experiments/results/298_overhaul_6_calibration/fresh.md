# 298: the fresh read

Seeds 50688-50783 (two independent sets of 48, pooled), two players, the three pairs pooled, 288 games a row; r143
is main at eca621c2be farmed with `--at`, r144 this item's build at 050cda02 (`read.json`, the judge;
`read295g.json`, greedy). Written by `make_tables.sh` with 297's `smoke_table.py`.

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 / 5 | How games ended | Slot reached: median; games at slot 4+ | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| d0 r143-judge | 288 | 92% | 8.1 (4-14) | 0 / 41 | liberated 266, deck_empty 17, research 5 | 2; 45 | 17 / 75 | 25 / 53 | 13 / 32 |
| d0 r144-judge | 288 | 93% | 8.4 (4-14) | 0 / 34 | liberated 267, deck_empty 20, research 1 | 1; 10 | 15 / 52 | 26 / 61 | 13 / 49 |
| d0 r143-greedy | 288 | 95% | 8.0 (3-14) | 2 / 31 | liberated 275, deck_empty 12, research 1 | 2; 32 | 15 / 50 | 21 / 47 | 11 / 33 |
| d0 r144-greedy | 288 | 96% | 8.0 (4-14) | 0 / 29 | liberated 277, deck_empty 11 | 1; 3 | 11 / 34 | 21 / 52 | 11 / 37 |
| d1 r143-judge | 288 | 89% | 8.5 (4-14) | 0 / 16 | liberated 255, deck_empty 18, research 14, discord_deck 1 | 3; 77 | 16 / 44 | 31 / 68 | 15 / 34 |
| d1 r144-judge | 288 | 89% | 8.9 (4-14) | 0 / 18 | liberated 256, deck_empty 31, discord_deck 1 | 2; 16 | 15 / 60 | 32 / 65 | 15 / 38 |
| d1 r143-greedy | 288 | 92% | 8.4 (4-14) | 0 / 16 | liberated 265, deck_empty 18, research 4, discord_deck 1 | 2; 55 | 15 / 55 | 28 / 69 | 13 / 34 |
| d1 r144-greedy | 288 | 90% | 8.5 (4-14) | 0 / 15 | liberated 260, deck_empty 26, research 1, discord_deck 1 | 1; 12 | 12 / 42 | 28 / 66 | 13 / 44 |
| d2 r143-judge | 288 | 83% | 8.6 (3-14) | 1 / 21 | liberated 238, deck_empty 18, research 22, discord_deck 10 | 3; 120 | 18 / 67 | 36 / 71 | 17 / 41 |
| d2 r144-judge | 288 | 81% | 9.0 (4-14) | 0 / 15 | liberated 234, deck_empty 24, research 26, discord_deck 4 | 3; 130 | 17 / 54 | 36 / 70 | 18 / 42 |
| d2 r143-greedy | 288 | 85% | 8.8 (4-14) | 0 / 12 | liberated 244, deck_empty 17, research 26, discord_deck 1 | 3; 111 | 16 / 48 | 32 / 65 | 16 / 38 |
| d2 r144-greedy | 288 | 84% | 8.6 (3-14) | 1 / 11 | liberated 242, deck_empty 14, research 27, discord_deck 5 | 3; 107 | 13 / 46 | 32 / 69 | 16 / 40 |
| d3 r143-judge | 288 | 62% | 8.7 (3-12) | 1 / 17 | liberated 180, deck_empty 48, research 48, discord_deck 12 | 4; 165 | 19 / 56 | 41 / 77 | 20 / 41 |
| d3 r144-judge | 288 | 64% | 8.5 (4-12) | 0 / 17 | liberated 183, deck_empty 40, research 52, discord_deck 13 | 4; 167 | 17 / 60 | 39 / 73 | 20 / 41 |
| d3 r143-greedy | 288 | 66% | 8.7 (4-12) | 0 / 14 | liberated 189, deck_empty 48, research 40, discord_deck 11 | 4; 157 | 17 / 45 | 37 / 68 | 19 / 36 |
| d3 r144-greedy | 288 | 65% | 8.8 (4-12) | 0 / 15 | liberated 188, deck_empty 55, research 41, discord_deck 4 | 4; 150 | 16 / 49 | 36 / 70 | 19 / 37 |

| Read | Games with a troop | First troop, round: mean (min-max) | Games with one by round 3 | Troops placed a game, by source | Strike icons a game | Strike hits a game | Redeploy icons / troops moved a game | Games a troop alone blocked a Liberate | Remove a troop a game | Discord taken off by Unity cards a game |
|---|---|---|---|---|---|---|---|---|---|---|
| d0 r143-judge | 77 | 6.4 (3-10) | 2 | discord 0.27, project 0.01, research 0.34 | 1.65 | 0.07 | 0.51 / 0.00 | 9 | 0.20 | 2.0 |
| d0 r144-judge | 55 | 6.3 (3-11) | 3 | discord 0.28, project 0.01, research 0.07 | 1.78 | 0.03 | 0.56 / 0.01 | 6 | 0.14 | 2.0 |
| d0 r143-greedy | 57 | 6.9 (3-14) | 4 | discord 0.16, research 0.22 | 1.47 | 0.03 | 0.47 / 0.00 | 8 | 0.17 | 2.0 |
| d0 r144-greedy | 36 | 5.9 (3-11) | 3 | discord 0.19, research 0.02 | 1.49 | 0.00 | 0.47 / 0.00 | 2 | 0.07 | 2.2 |
| d1 r143-judge | 136 | 5.9 (3-12) | 9 | discord 0.40, project 0.29, research 0.59 | 1.84 | 0.12 | 0.56 / 0.00 | 22 | 0.39 | 2.4 |
| d1 r144-judge | 98 | 6.3 (3-13) | 8 | discord 0.43, project 0.16, research 0.11 | 2.01 | 0.05 | 0.63 / 0.01 | 9 | 0.20 | 2.4 |
| d1 r143-greedy | 116 | 6.4 (3-12) | 4 | discord 0.36, project 0.25, research 0.41 | 1.77 | 0.06 | 0.56 / 0.00 | 13 | 0.38 | 2.5 |
| d1 r144-greedy | 81 | 6.1 (3-11) | 3 | discord 0.35, project 0.11, research 0.08 | 1.74 | 0.04 | 0.57 / 0.00 | 6 | 0.19 | 2.5 |
| d2 r143-judge | 204 | 5.7 (2-13) | 23 | discord 0.55, project 1.10, research 0.99 | 2.04 | 0.25 | 0.58 / 0.02 | 47 | 0.82 | 2.5 |
| d2 r144-judge | 199 | 5.7 (2-13) | 15 | discord 0.59, project 0.80, research 1.09 | 2.38 | 0.22 | 0.65 / 0.03 | 34 | 0.74 | 2.5 |
| d2 r143-greedy | 191 | 5.8 (2-12) | 20 | discord 0.49, project 0.82, research 0.96 | 2.15 | 0.22 | 0.60 / 0.01 | 35 | 0.70 | 2.8 |
| d2 r144-greedy | 178 | 5.8 (2-11) | 12 | discord 0.43, project 0.64, research 0.92 | 2.09 | 0.19 | 0.59 / 0.02 | 30 | 0.64 | 2.6 |
| d3 r143-judge | 254 | 4.9 (2-11) | 56 | discord 0.89, project 2.11, research 1.45 | 2.67 | 0.69 | 0.68 / 0.04 | 80 | 1.32 | 2.5 |
| d3 r144-judge | 251 | 5.1 (2-11) | 44 | discord 0.83, project 1.72, research 3.01 | 2.53 | 1.02 | 0.63 / 0.22 | 89 | 1.07 | 2.3 |
| d3 r143-greedy | 249 | 5.1 (2-10) | 46 | discord 0.66, project 1.82, research 1.39 | 2.57 | 0.48 | 0.69 / 0.01 | 70 | 1.23 | 2.5 |
| d3 r144-greedy | 247 | 5.4 (2-11) | 32 | discord 0.67, project 1.60, research 2.83 | 2.63 | 0.80 | 0.70 / 0.16 | 89 | 1.09 | 2.6 |
