# 294: the smoke tables

The judge (`strategist`), two players, the three board pairs (bp Builder + Protector, br Builder + Researcher, pr
Protector + Researcher), 48 seeds a cell. Every r141 file is the final build (commit 66c7ebc0, the code this item
ships); the tables are `smoke_table.py`'s. A win is the end reason `liberated` (`purge` at r140); `deck_empty` is
the round cap, `research` slot 7, `discord_deck` the Discord deck, `node` a seat with no node. No game timed out.

## Before and after on the same seeds (50328-50375), then the fresh read (50376-50423)

`r140` is 293's kept read (`293/f293-r140.jsonl`, farm spec `read`), `r141` the same seeds on this build, `fresh`
this build on seeds no build has seen. At r140 troops came from threats and stage-III Musters and marched, so its
troop columns read differently: "Strike hits" there are the march's strikes, and it tallied no peak, guard or
Strike-icon counter (those cells read 0). Troop arrivals are `map.troops_placed.<source>.rNN` on both sides.

### d0

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 48 | 44 (92%), 8 by r5 | 7.1 (4-11) | research 4 | 1:3 2:9 3:7 4:10 5:8 6:7 7:4 | 20.8 / 50 | 18.0 / 42 | 9.0 / 16 | 0.04 / 2 |
| r140 br | 48 | 41 (85%), 8 by r5 | 7.2 (4-14) | deck_empty 1, research 6 | 2:15 3:8 4:8 5:9 6:2 7:6 | 20.0 / 40 | 19.5 / 49 | 8.7 / 16 | 0.02 / 1 |
| r140 pr | 48 | 44 (92%), 13 by r5 | 6.8 (4-14) | deck_empty 2, research 2 | 1:3 2:7 3:21 4:6 5:5 6:4 7:2 | 18.1 / 44 | 15.8 / 36 | 8.0 / 15 | 0.00 / 0 |
| r140 all | 144 | 129 (90%), 29 by r5 | 7.0 (4-14) | deck_empty 3, research 12 | 1:6 2:31 3:36 4:24 5:22 6:13 7:12 | 19.7 / 50 | 17.8 / 49 | 8.5 / 16 | 0.02 / 2 |
| r141 bp | 48 | 47 (98%), 30 by r5 | 5.1 (1-14) | deck_empty 1 | 0:9 1:12 2:17 3:6 4:2 5:1 6:1 | 13.8 / 39 | 12.5 / 33 | 9.7 / 30 | 0.02 / 1 |
| r141 br | 48 | 47 (98%), 24 by r5 | 5.6 (2-14) | deck_empty 1 | 0:3 1:14 2:15 3:12 4:3 6:1 | 12.8 / 39 | 15.4 / 36 | 11.3 / 26 | 0.04 / 2 |
| r141 pr | 48 | 47 (98%), 31 by r5 | 5.3 (2-14) | deck_empty 1 | 0:7 1:14 2:17 3:5 4:2 5:1 6:2 | 14.1 / 47 | 13.6 / 46 | 10.1 / 25 | 0.00 / 0 |
| r141 all | 144 | 141 (98%), 85 by r5 | 5.3 (1-14) | deck_empty 3 | 0:19 1:40 2:49 3:23 4:7 5:2 6:4 | 13.6 / 47 | 13.8 / 46 | 10.4 / 30 | 0.02 / 2 |
| fresh bp | 48 | 47 (98%), 23 by r5 | 5.6 (1-14) | deck_empty 1 | 0:3 1:14 2:12 3:12 4:5 5:2 | 14.7 / 36 | 14.5 / 29 | 11.5 / 23 | 0.06 / 1 |
| fresh br | 48 | 47 (98%), 21 by r5 | 6.1 (2-14) | deck_empty 1 | 0:3 1:11 2:13 3:11 4:5 5:4 6:1 | 15.6 / 62 | 17.5 / 40 | 12.5 / 28 | 0.00 / 0 |
| fresh pr | 48 | 48 (100%), 29 by r5 | 5.3 (1-13) | - | 0:6 1:15 2:13 3:11 4:3 | 14.7 / 54 | 13.2 / 40 | 10.3 / 23 | 0.00 / 0 |
| fresh all | 144 | 142 (99%), 73 by r5 | 5.7 (1-14) | deck_empty 2 | 0:12 1:40 2:38 3:34 4:13 5:6 6:1 | 15.0 / 62 | 15.1 / 40 | 11.4 / 28 | 0.02 / 1 |

| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max | Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game | Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 40 / 48 | 2.5 (1-6); 31 by r3 | muster 0.02, threat 1.56 | 0.00 / 0 | 0.00 | 0.98 | 0.90 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.19 |
| r140 br | 39 / 48 | 2.8 (1-11); 29 by r3 | muster 0.38, threat 1.52 | 0.00 / 0 | 0.00 | 1.71 | 1.35 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.02 |
| r140 pr | 43 / 48 | 3.3 (1-11); 31 by r3 | muster 0.17, threat 1.33 | 0.00 / 0 | 0.00 | 0.83 | 0.69 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.12 |
| r140 all | 122 / 144 | 2.9 (1-11); 91 by r3 | muster 0.19, threat 1.47 | 0.00 / 0 | 0.00 | 1.17 | 0.98 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.11 |
| r141 bp | 5 / 48 | 7.4 (6-9); 0 by r3 | project 0.04, research 0.15 | 0.00 / 0 | 0.40 | 0.00 | 0.00 | 0.25 / 0.00 | 0.00 | 0.00, in 0 games | 0.00 |
| r141 br | 4 / 48 | 6.0 (6-6); 0 by r3 | research 0.21 | 0.02 / 1 | 0.44 | 0.00 | 0.00 | 0.27 / 0.00 | 0.02 | 0.02, in 1 games | 0.04 |
| r141 pr | 5 / 48 | 6.6 (6-8); 0 by r3 | research 0.19 | 0.00 / 0 | 0.40 | 0.02 | 0.02 | 0.21 / 0.00 | 0.00 | 0.00, in 0 games | 0.08 |
| r141 all | 14 / 144 | 6.7 (6-9); 0 by r3 | project 0.01, research 0.18 | 0.01 / 1 | 0.41 | 0.01 | 0.01 | 0.24 / 0.00 | 0.01 | 0.01, in 1 games | 0.04 |
| fresh bp | 8 / 48 | 6.9 (6-9); 0 by r3 | project 0.04, research 0.25 | 0.02 / 1 | 0.44 | 0.02 | 0.02 | 0.21 / 0.02 | 0.00 | 0.00, in 0 games | 0.06 |
| fresh br | 10 / 48 | 7.2 (6-8); 0 by r3 | research 0.46 | 0.04 / 1 | 0.54 | 0.02 | 0.02 | 0.31 / 0.02 | 0.02 | 0.02, in 1 games | 0.10 |
| fresh pr | 2 / 48 | 7.5 (7-8); 0 by r3 | research 0.04 | 0.00 / 0 | 0.33 | 0.00 | 0.00 | 0.19 / 0.00 | 0.00 | 0.00, in 0 games | 0.00 |
| fresh all | 20 / 144 | 7.1 (6-9); 0 by r3 | project 0.01, research 0.25 | 0.02 / 1 | 0.44 | 0.01 | 0.01 | 0.24 / 0.01 | 0.01 | 0.01, in 1 games | 0.06 |

### d1

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 48 | 37 (77%), 2 by r5 | 7.8 (5-13) | discord_deck 1, research 10 | 2:6 3:2 4:10 5:12 6:8 7:10 | 18.9 / 40 | 27.9 / 67 | 10.7 / 17 | 0.27 / 4 |
| r140 br | 48 | 33 (69%), 4 by r5 | 7.1 (4-13) | discord_deck 1, research 14 | 1:1 2:3 3:10 4:10 5:7 6:3 7:14 | 16.2 / 38 | 26.1 / 65 | 10.7 / 20 | 0.17 / 8 |
| r140 pr | 48 | 36 (75%), 7 by r5 | 7.3 (4-13) | deck_empty 1, discord_deck 1, research 10 | 1:1 2:7 3:6 4:11 5:9 6:4 7:10 | 16.2 / 34 | 24.3 / 65 | 10.0 / 18 | 0.23 / 3 |
| r140 all | 144 | 106 (74%), 13 by r5 | 7.4 (4-13) | deck_empty 1, discord_deck 3, research 34 | 1:2 2:16 3:18 4:31 5:28 6:15 7:34 | 17.1 / 40 | 26.1 / 67 | 10.5 / 20 | 0.22 / 8 |
| r141 bp | 48 | 41 (85%), 17 by r5 | 6.6 (1-14) | deck_empty 2, discord_deck 1, research 4 | 0:2 1:8 2:11 3:11 4:6 5:4 6:2 7:4 | 14.4 / 44 | 22.6 / 65 | 15.1 / 41 | 0.23 / 2 |
| r141 br | 48 | 40 (83%), 14 by r5 | 7.4 (2-14) | deck_empty 5, discord_deck 1, research 2 | 1:8 2:11 3:13 4:5 5:7 6:2 7:2 | 15.0 / 34 | 25.4 / 67 | 15.7 / 30 | 0.25 / 4 |
| r141 pr | 48 | 45 (94%), 15 by r5 | 6.4 (2-14) | deck_empty 2, research 1 | 0:2 1:11 2:13 3:10 4:9 5:1 6:1 7:1 | 14.0 / 39 | 21.6 / 58 | 13.3 / 32 | 0.12 / 2 |
| r141 all | 144 | 126 (88%), 46 by r5 | 6.8 (1-14) | deck_empty 9, discord_deck 2, research 7 | 0:4 1:27 2:35 3:34 4:20 5:12 6:5 7:7 | 14.5 / 44 | 23.2 / 67 | 14.7 / 41 | 0.20 / 4 |
| fresh bp | 48 | 45 (94%), 21 by r5 | 6.3 (2-14) | deck_empty 1, discord_deck 1, research 1 | 0:1 1:8 2:10 3:16 4:9 5:1 6:2 7:1 | 13.8 / 42 | 23.4 / 68 | 14.6 / 32 | 0.10 / 3 |
| fresh br | 48 | 46 (96%), 17 by r5 | 6.8 (2-14) | deck_empty 1, research 1 | 1:8 2:10 3:14 4:5 5:8 6:2 7:1 | 14.1 / 45 | 23.0 / 53 | 15.4 / 30 | 0.06 / 1 |
| fresh pr | 48 | 46 (96%), 20 by r5 | 6.1 (1-14) | deck_empty 1, research 1 | 0:2 1:9 2:13 3:16 4:2 5:4 6:1 7:1 | 13.4 / 42 | 20.8 / 61 | 13.0 / 28 | 0.02 / 1 |
| fresh all | 144 | 137 (95%), 58 by r5 | 6.4 (1-14) | deck_empty 3, discord_deck 1, research 3 | 0:3 1:25 2:33 3:46 4:16 5:13 6:5 7:3 | 13.8 / 45 | 22.4 / 68 | 14.3 / 32 | 0.06 / 3 |

| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max | Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game | Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 42 / 48 | 2.3 (1-5); 36 by r3 | muster 0.12, project 0.65, threat 1.75 | 0.00 / 0 | 0.00 | 1.19 | 1.04 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.31 |
| r140 br | 44 / 48 | 2.5 (1-5); 35 by r3 | muster 0.12, project 0.67, threat 1.75 | 0.00 / 0 | 0.00 | 1.98 | 1.62 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.06 |
| r140 pr | 41 / 48 | 2.8 (1-10); 30 by r3 | muster 0.12, project 0.46, threat 1.46 | 0.00 / 0 | 0.00 | 0.96 | 0.79 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.06 |
| r140 all | 127 / 144 | 2.6 (1-10); 101 by r3 | muster 0.12, project 0.59, threat 1.65 | 0.00 / 0 | 0.00 | 1.38 | 1.15 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.15 |
| r141 bp | 20 / 48 | 6.1 (3-9); 1 by r3 | discord 0.02, project 0.35, research 0.75 | 0.17 / 2 | 0.83 | 0.04 | 0.04 | 0.42 / 0.00 | 0.23 | 0.19, in 5 games | 0.29 |
| r141 br | 19 / 48 | 5.9 (4-8); 0 by r3 | project 0.54, research 0.83 | 0.29 / 4 | 1.21 | 0.23 | 0.23 | 0.42 / 0.00 | 0.27 | 0.21, in 5 games | 0.48 |
| r141 pr | 18 / 48 | 7.1 (4-13); 0 by r3 | project 0.42, research 0.44 | 0.02 / 1 | 0.67 | 0.00 | 0.00 | 0.35 / 0.00 | 0.04 | 0.04, in 1 games | 0.10 |
| r141 all | 57 / 144 | 6.3 (3-13); 1 by r3 | discord 0.01, project 0.44, research 0.67 | 0.16 / 4 | 0.90 | 0.09 | 0.09 | 0.40 / 0.00 | 0.18 | 0.15, in 11 games | 0.29 |
| fresh bp | 15 / 48 | 5.9 (3-8); 1 by r3 | project 0.40, research 0.52 | 0.08 / 3 | 0.56 | 0.02 | 0.02 | 0.23 / 0.00 | 0.08 | 0.04, in 1 games | 0.21 |
| fresh br | 19 / 48 | 5.9 (3-9); 1 by r3 | project 0.75, research 0.79 | 0.21 / 4 | 0.85 | 0.02 | 0.02 | 0.33 / 0.00 | 0.25 | 0.17, in 4 games | 0.42 |
| fresh pr | 11 / 48 | 5.5 (3-7); 1 by r3 | project 0.23, research 0.23 | 0.06 / 1 | 0.65 | 0.00 | 0.00 | 0.25 / 0.00 | 0.02 | 0.00, in 0 games | 0.08 |
| fresh all | 45 / 144 | 5.8 (3-9); 3 by r3 | project 0.46, research 0.51 | 0.12 / 4 | 0.69 | 0.01 | 0.01 | 0.27 / 0.00 | 0.12 | 0.07, in 5 games | 0.24 |

### d2

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 48 | 28 (58%), 0 by r5 | 7.3 (5-11) | discord_deck 1, research 19 | 2:4 3:4 4:7 5:11 6:3 7:19 | 16.9 / 30 | 30.9 / 66 | 11.6 / 19 | 0.65 / 10 |
| r140 br | 48 | 30 (62%), 3 by r5 | 6.9 (4-11) | discord_deck 1, research 17 | 2:2 3:3 4:11 5:8 6:7 7:17 | 16.3 / 31 | 30.5 / 65 | 12.0 / 20 | 0.19 / 6 |
| r140 pr | 48 | 34 (71%), 10 by r5 | 6.7 (4-13) | discord_deck 1, research 13 | 1:1 2:2 3:11 4:10 5:7 6:4 7:13 | 15.6 / 45 | 28.3 / 69 | 10.9 / 19 | 0.42 / 6 |
| r140 all | 144 | 92 (64%), 13 by r5 | 7.0 (4-13) | discord_deck 3, research 49 | 1:1 2:8 3:18 4:28 5:26 6:14 7:49 | 16.2 / 45 | 29.9 / 69 | 11.5 / 20 | 0.42 / 10 |
| r141 bp | 48 | 40 (83%), 15 by r5 | 6.8 (1-10) | discord_deck 1, research 7 | 0:2 1:6 2:7 3:13 4:4 5:7 6:2 7:7 | 13.9 / 32 | 26.0 / 64 | 17.7 / 42 | 0.27 / 2 |
| r141 br | 48 | 42 (88%), 11 by r5 | 7.0 (2-12) | discord_deck 1, research 5 | 1:5 2:5 3:18 4:7 5:8 7:5 | 14.2 / 35 | 28.9 / 67 | 17.5 / 35 | 0.21 / 3 |
| r141 pr | 48 | 44 (92%), 12 by r5 | 6.7 (2-14) | deck_empty 2, research 2 | 0:1 1:7 2:13 3:7 4:11 5:6 6:1 7:2 | 14.8 / 36 | 25.3 / 63 | 15.4 / 32 | 0.15 / 2 |
| r141 all | 144 | 126 (88%), 38 by r5 | 6.8 (1-14) | deck_empty 2, discord_deck 2, research 14 | 0:3 1:18 2:25 3:38 4:22 5:21 6:3 7:14 | 14.3 / 36 | 26.8 / 67 | 16.9 / 42 | 0.21 / 3 |
| fresh bp | 48 | 41 (85%), 13 by r5 | 6.7 (2-13) | deck_empty 1, research 6 | 0:1 1:4 2:10 3:9 4:9 5:6 6:3 7:6 | 15.4 / 49 | 26.7 / 62 | 18.2 / 41 | 0.31 / 3 |
| fresh br | 48 | 44 (92%), 9 by r5 | 7.1 (2-14) | deck_empty 2, research 2 | 1:5 2:9 3:7 4:5 5:12 6:8 7:2 | 15.6 / 40 | 28.2 / 56 | 18.3 / 35 | 0.19 / 2 |
| fresh pr | 48 | 46 (96%), 18 by r5 | 6.1 (1-13) | research 2 | 0:2 1:10 2:6 3:10 4:7 5:10 6:1 7:2 | 14.0 / 43 | 23.4 / 53 | 15.4 / 32 | 0.19 / 3 |
| fresh all | 144 | 131 (91%), 40 by r5 | 6.6 (1-14) | deck_empty 3, research 10 | 0:3 1:19 2:25 3:26 4:21 5:28 6:12 7:10 | 15.0 / 49 | 26.1 / 62 | 17.3 / 41 | 0.23 / 3 |

| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max | Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game | Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 45 / 48 | 2.4 (1-7); 40 by r3 | muster 0.02, project 1.75, threat 1.88 | 0.00 / 0 | 0.00 | 1.40 | 1.10 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.44 |
| r140 br | 46 / 48 | 2.7 (1-10); 38 by r3 | muster 0.08, project 1.83, threat 1.77 | 0.00 / 0 | 0.00 | 2.46 | 1.98 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.25 |
| r140 pr | 42 / 48 | 2.4 (1-6); 38 by r3 | muster 0.08, project 1.04, threat 1.60 | 0.00 / 0 | 0.00 | 1.15 | 0.92 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.17 |
| r140 all | 133 / 144 | 2.5 (1-10); 116 by r3 | muster 0.06, project 1.54, threat 1.75 | 0.00 / 0 | 0.00 | 1.67 | 1.33 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.28 |
| r141 bp | 27 / 48 | 5.5 (2-8); 2 by r3 | project 0.85, research 1.06 | 0.44 / 3 | 0.96 | 0.17 | 0.15 | 0.40 / 0.00 | 0.52 | 0.44, in 8 games | 0.67 |
| r141 br | 26 / 48 | 5.3 (3-9); 2 by r3 | project 1.17, research 1.04 | 0.27 / 5 | 1.02 | 0.23 | 0.21 | 0.46 / 0.00 | 0.33 | 0.12, in 4 games | 0.67 |
| r141 pr | 25 / 48 | 5.6 (3-9); 2 by r3 | project 0.60, research 0.79 | 0.21 / 2 | 0.81 | 0.10 | 0.10 | 0.38 / 0.00 | 0.10 | 0.06, in 3 games | 0.19 |
| r141 all | 78 / 144 | 5.5 (2-9); 6 by r3 | project 0.88, research 0.97 | 0.31 / 5 | 0.93 | 0.17 | 0.15 | 0.41 / 0.00 | 0.32 | 0.21, in 15 games | 0.51 |
| fresh bp | 29 / 48 | 5.0 (2-8); 5 by r3 | project 0.96, research 1.12 | 0.42 / 3 | 1.00 | 0.15 | 0.12 | 0.29 / 0.00 | 0.54 | 0.31, in 8 games | 0.73 |
| fresh br | 30 / 48 | 5.0 (2-7); 4 by r3 | discord 0.04, project 1.21, research 1.54 | 0.42 / 3 | 1.21 | 0.33 | 0.33 | 0.38 / 0.00 | 0.33 | 0.29, in 8 games | 1.06 |
| fresh pr | 24 / 48 | 5.1 (2-8); 3 by r3 | project 0.62, research 0.71 | 0.04 / 1 | 0.77 | 0.02 | 0.02 | 0.27 / 0.00 | 0.04 | 0.04, in 2 games | 0.25 |
| fresh all | 83 / 144 | 5.0 (2-8); 12 by r3 | discord 0.01, project 0.93, research 1.12 | 0.29 / 3 | 0.99 | 0.17 | 0.16 | 0.31 / 0.00 | 0.31 | 0.22, in 18 games | 0.68 |

### d3

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 48 | 23 (48%), 2 by r5 | 6.6 (4-10) | discord_deck 1, research 24 | 1:1 3:5 4:10 5:3 6:5 7:24 | 15.8 / 24 | 33.1 / 65 | 12.4 / 19 | 0.73 / 7 |
| r140 br | 48 | 21 (44%), 2 by r5 | 6.4 (4-10) | discord_deck 1, node 1, research 25 | 2:1 3:2 4:12 5:4 6:4 7:25 | 15.1 / 30 | 33.3 / 65 | 12.5 / 18 | 0.52 / 6 |
| r140 pr | 48 | 28 (58%), 4 by r5 | 6.6 (4-12) | deck_empty 1, research 19 | 2:2 3:6 4:6 5:9 6:6 7:19 | 15.9 / 44 | 32.3 / 68 | 11.8 / 17 | 0.58 / 6 |
| r140 all | 144 | 72 (50%), 8 by r5 | 6.6 (4-12) | deck_empty 1, discord_deck 2, node 1, research 68 | 1:1 2:3 3:13 4:28 5:16 6:15 7:68 | 15.6 / 44 | 32.9 / 68 | 12.2 / 19 | 0.61 / 7 |
| r141 bp | 48 | 38 (79%), 7 by r5 | 6.8 (1-10) | discord_deck 1, research 9 | 0:1 1:6 2:5 3:11 4:4 5:5 6:7 7:9 | 13.9 / 39 | 30.2 / 68 | 19.3 / 36 | 0.42 / 4 |
| r141 br | 48 | 39 (81%), 11 by r5 | 6.8 (2-12) | deck_empty 1, discord_deck 2, research 6 | 1:4 2:6 3:9 4:11 5:7 6:5 7:6 | 14.7 / 35 | 31.9 / 69 | 19.9 / 43 | 0.40 / 5 |
| r141 pr | 48 | 38 (79%), 10 by r5 | 6.6 (2-9) | discord_deck 1, research 9 | 0:2 1:7 2:6 3:6 4:10 5:5 6:3 7:9 | 14.2 / 34 | 29.6 / 64 | 19.0 / 42 | 0.31 / 4 |
| r141 all | 144 | 115 (80%), 28 by r5 | 6.7 (1-12) | deck_empty 1, discord_deck 4, research 24 | 0:3 1:17 2:17 3:26 4:25 5:17 6:15 7:24 | 14.3 / 39 | 30.5 / 69 | 19.4 / 43 | 0.38 / 5 |
| fresh bp | 48 | 32 (67%), 12 by r5 | 6.4 (1-12) | deck_empty 1, discord_deck 2, research 13 | 0:3 1:4 2:4 3:6 4:9 5:6 6:3 7:13 | 14.1 / 32 | 31.7 / 73 | 20.4 / 37 | 0.58 / 5 |
| fresh br | 48 | 34 (71%), 11 by r5 | 6.7 (1-10) | discord_deck 3, research 11 | 0:2 1:2 2:5 3:7 4:6 5:6 6:9 7:11 | 15.3 / 39 | 33.3 / 65 | 21.9 / 41 | 0.54 / 7 |
| fresh pr | 48 | 39 (81%), 18 by r5 | 6.2 (1-12) | deck_empty 1, discord_deck 1, research 7 | 0:2 1:6 2:7 3:10 4:8 5:4 6:4 7:7 | 15.7 / 42 | 28.8 / 65 | 17.9 / 34 | 0.12 / 2 |
| fresh all | 144 | 105 (73%), 41 by r5 | 6.4 (1-12) | deck_empty 2, discord_deck 6, research 31 | 0:7 1:12 2:16 3:23 4:23 5:16 6:16 7:31 | 15.0 / 42 | 31.3 / 73 | 20.0 / 41 | 0.42 / 7 |

| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max | Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game | Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 44 / 48 | 2.2 (1-5); 40 by r3 | muster 0.12, project 2.04, threat 1.90 | 0.00 / 0 | 0.00 | 1.50 | 1.15 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.65 |
| r140 br | 46 / 48 | 2.6 (1-7); 37 by r3 | muster 0.17, project 3.06, threat 1.85 | 0.00 / 0 | 0.00 | 3.08 | 2.50 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.33 |
| r140 pr | 44 / 48 | 2.6 (1-8); 35 by r3 | muster 0.21, project 2.23, threat 1.48 | 0.00 / 0 | 0.00 | 1.44 | 1.12 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.27 |
| r140 all | 134 / 144 | 2.5 (1-8); 112 by r3 | muster 0.17, project 2.44, threat 1.74 | 0.00 / 0 | 0.00 | 2.01 | 1.59 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.42 |
| r141 bp | 33 / 48 | 5.1 (2-8); 4 by r3 | discord 0.10, project 1.31, research 1.40 | 0.62 / 5 | 1.23 | 0.35 | 0.29 | 0.38 / 0.00 | 0.73 | 0.48, in 14 games | 0.88 |
| r141 br | 36 / 48 | 4.8 (2-9); 8 by r3 | discord 0.04, project 1.42, research 1.67 | 0.56 / 7 | 1.25 | 0.33 | 0.27 | 0.35 / 0.02 | 0.60 | 0.52, in 12 games | 0.88 |
| r141 pr | 37 / 48 | 4.9 (2-8); 7 by r3 | discord 0.02, project 1.19, research 1.27 | 0.27 / 2 | 1.02 | 0.12 | 0.10 | 0.33 / 0.00 | 0.27 | 0.21, in 5 games | 0.67 |
| r141 all | 106 / 144 | 4.9 (2-9); 19 by r3 | discord 0.06, project 1.31, research 1.44 | 0.49 / 7 | 1.17 | 0.27 | 0.22 | 0.35 / 0.01 | 0.53 | 0.40, in 31 games | 0.81 |
| fresh bp | 34 / 48 | 4.6 (2-8); 7 by r3 | discord 0.04, project 1.00, research 1.75 | 0.56 / 5 | 1.08 | 0.17 | 0.15 | 0.38 / 0.00 | 0.79 | 0.69, in 15 games | 0.94 |
| fresh br | 36 / 48 | 4.5 (2-7); 10 by r3 | discord 0.04, project 1.44, research 2.17 | 0.58 / 7 | 1.35 | 0.38 | 0.35 | 0.35 / 0.00 | 0.62 | 0.40, in 9 games | 1.12 |
| fresh pr | 29 / 48 | 4.2 (2-7); 10 by r3 | discord 0.02, project 0.83, research 1.06 | 0.19 / 3 | 0.98 | 0.12 | 0.12 | 0.33 / 0.00 | 0.27 | 0.27, in 7 games | 0.56 |
| fresh all | 99 / 144 | 4.4 (2-8); 27 by r3 | discord 0.03, project 1.09, research 1.66 | 0.44 / 7 | 1.14 | 0.22 | 0.21 | 0.35 / 0.00 | 0.56 | 0.45, in 31 games | 0.88 |

## The dev screen (seeds 36800-36831, d0): the asked numbers, the hands-off arm and the starting-cube lever

`bp`, `br`, `pr`: the asked numbers (K 2, no starting cubes). `_off`: Dispatch and Liberate switched off
(`board.std_off=4,6`), so no ambassador is dispatched onto a corrupted station and nothing is liberated by the
action: how far the row runs when players leave its stations alone (cards still liberate and block, so 55-70% of
these games are still won). `_s1`, `_s2`: 1 and 2 starting cubes a country (`adversary.research_start`), on and off.
`r140` rows: 293's K screen's K 1 arm on the same seeds (`293/s293-k.jsonl`).

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r141 bp | 32 | 32 (100%), 20 by r5 | 4.8 (2-8) | - | 0:2 1:13 2:5 3:7 4:5 | 13.6 / 27 | 13.2 / 27 | 10.8 / 23 | 0.03 / 1 |
| r141 br | 32 | 32 (100%), 16 by r5 | 5.8 (2-10) | - | 0:1 1:11 2:10 3:7 4:1 5:2 | 12.2 / 23 | 16.4 / 32 | 11.3 / 23 | 0.00 / 0 |
| r141 pr | 32 | 31 (97%), 20 by r5 | 5.2 (2-14) | deck_empty 1 | 0:3 1:9 2:13 3:3 4:3 5:1 | 13.4 / 45 | 13.0 / 28 | 10.4 / 23 | 0.00 / 0 |
| r141 bp_off | 32 | 13 (41%), 0 by r5 | 11.3 (6-14) | deck_empty 10, research 9 | 2:2 3:2 4:9 5:4 6:6 7:9 | 39.7 / 79 | 30.7 / 51 | 24.0 / 37 | 0.16 / 2 |
| r141 br_off | 32 | 18 (56%), 1 by r5 | 11.1 (4-14) | deck_empty 12, research 2 | 2:2 3:6 4:5 5:8 6:9 7:2 | 42.4 / 86 | 31.5 / 48 | 23.1 / 35 | 0.03 / 1 |
| r141 pr_off | 32 | 20 (62%), 2 by r5 | 9.7 (5-14) | deck_empty 7, research 5 | 2:3 3:7 4:9 5:6 6:2 7:5 | 35.9 / 76 | 27.2 / 51 | 21.2 / 32 | 0.06 / 1 |
| r141 bp_s1 | 32 | 31 (97%), 19 by r5 | 5.3 (2-14) | deck_empty 1 | 1:6 2:11 3:7 4:5 5:2 6:1 | 15.5 / 60 | 13.8 / 38 | 11.4 / 33 | 0.12 / 2 |
| r141 br_s1 | 32 | 32 (100%), 16 by r5 | 5.8 (2-12) | - | 1:5 2:8 3:13 4:3 5:3 | 13.3 / 23 | 16.5 / 32 | 11.7 / 21 | 0.00 / 0 |
| r141 pr_s1 | 32 | 32 (100%), 20 by r5 | 5.1 (2-10) | - | 1:6 2:9 3:8 4:5 5:4 | 14.2 / 29 | 13.9 / 42 | 11.1 / 22 | 0.06 / 1 |
| r141 bp_off_s1 | 32 | 13 (41%), 1 by r5 | 10.2 (5-14) | deck_empty 6, research 13 | 2:1 3:4 4:6 5:2 6:6 7:13 | 37.6 / 81 | 28.8 / 58 | 23.4 / 38 | 0.06 / 1 |
| r141 br_off_s1 | 32 | 17 (53%), 1 by r5 | 10.2 (4-14) | deck_empty 4, research 11 | 3:5 4:6 5:8 6:2 7:11 | 37.4 / 76 | 30.4 / 48 | 22.2 / 29 | 0.25 / 2 |
| r141 pr_off_s1 | 32 | 16 (50%), 3 by r5 | 9.7 (5-14) | deck_empty 8, research 8 | 3:7 4:3 5:11 6:3 7:8 | 36.6 / 82 | 27.0 / 47 | 21.2 / 33 | 0.12 / 2 |
| r141 bp_s2 | 32 | 31 (97%), 19 by r5 | 5.6 (2-14) | deck_empty 1 | 1:2 2:10 3:8 4:8 5:2 6:2 | 17.1 / 60 | 15.0 / 38 | 11.7 / 33 | 0.22 / 2 |
| r141 br_s2 | 32 | 31 (97%), 16 by r5 | 6.2 (2-14) | deck_empty 1 | 1:1 2:10 3:7 4:7 5:6 6:1 | 17.0 / 41 | 17.6 / 39 | 12.9 / 24 | 0.06 / 2 |
| r141 pr_s2 | 32 | 31 (97%), 17 by r5 | 5.7 (2-14) | deck_empty 1 | 1:3 2:7 3:8 4:10 5:3 6:1 | 15.9 / 39 | 13.8 / 26 | 11.7 / 23 | 0.16 / 2 |
| r141 bp_off_s2 | 32 | 14 (44%), 0 by r5 | 10.1 (6-14) | deck_empty 7, research 11 | 3:2 4:2 5:9 6:8 7:11 | 37.2 / 79 | 28.4 / 48 | 22.3 / 38 | 0.06 / 1 |
| r141 br_off_s2 | 32 | 16 (50%), 1 by r5 | 9.7 (5-14) | deck_empty 3, research 13 | 3:1 4:3 5:8 6:7 7:13 | 34.8 / 76 | 28.1 / 45 | 23.5 / 34 | 0.34 / 2 |
| r141 pr_off_s2 | 32 | 15 (47%), 0 by r5 | 10.5 (6-14) | deck_empty 4, research 13 | 3:1 5:10 6:8 7:13 | 39.9 / 81 | 29.2 / 44 | 23.3 / 30 | 0.16 / 2 |

| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max | Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game | Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r141 bp | 5 / 32 | 6.4 (6-7); 0 by r3 | research 0.22 | 0.00 / 0 | 0.16 | 0.00 | 0.00 | 0.16 / 0.00 | 0.00 | 0.00, in 0 games | 0.09 |
| r141 br | 3 / 32 | 6.3 (6-7); 0 by r3 | research 0.19 | 0.00 / 0 | 0.47 | 0.00 | 0.00 | 0.16 / 0.00 | 0.00 | 0.00, in 0 games | 0.09 |
| r141 pr | 4 / 32 | 6.8 (6-8); 0 by r3 | research 0.16 | 0.00 / 0 | 0.44 | 0.00 | 0.00 | 0.09 / 0.00 | 0.00 | 0.00, in 0 games | 0.12 |
| r141 bp_off | 28 / 32 | 7.1 (5-13); 0 by r3 | project 0.06, research 1.81 | 0.09 / 1 | 3.44 | 0.09 | 0.09 | 0.84 / 0.00 | 0.16 | 0.16, in 2 games | 0.59 |
| r141 br_off | 26 / 32 | 7.1 (5-11); 0 by r3 | project 0.22, research 2.19 | 0.16 / 1 | 3.44 | 0.12 | 0.12 | 0.81 / 0.00 | 0.06 | 0.06, in 2 games | 0.69 |
| r141 pr_off | 23 / 32 | 6.8 (5-11); 0 by r3 | project 0.09, research 1.09 | 0.06 / 1 | 2.41 | 0.03 | 0.03 | 0.66 / 0.00 | 0.12 | 0.06, in 2 games | 0.19 |
| r141 bp_s1 | 8 / 32 | 5.8 (5-7); 0 by r3 | research 0.47 | 0.12 / 2 | 0.41 | 0.00 | 0.00 | 0.19 / 0.00 | 0.44 | 0.28, in 3 games | 0.12 |
| r141 br_s1 | 6 / 32 | 5.5 (5-7); 0 by r3 | research 0.38 | 0.00 / 0 | 0.50 | 0.00 | 0.00 | 0.16 / 0.00 | 0.00 | 0.00, in 0 games | 0.09 |
| r141 pr_s1 | 8 / 32 | 6.0 (5-7); 0 by r3 | research 0.38 | 0.03 / 1 | 0.34 | 0.00 | 0.00 | 0.09 / 0.00 | 0.06 | 0.06, in 1 games | 0.06 |
| r141 bp_off_s1 | 27 / 32 | 5.9 (4-10); 0 by r3 | project 0.06, research 2.16 | 0.09 / 1 | 2.59 | 0.06 | 0.03 | 0.69 / 0.00 | 0.12 | 0.09, in 3 games | 0.72 |
| r141 br_off_s1 | 29 / 32 | 6.1 (5-14); 0 by r3 | discord 0.06, project 0.25, research 2.50 | 0.16 / 1 | 2.75 | 0.12 | 0.12 | 0.72 / 0.00 | 0.12 | 0.12, in 4 games | 1.00 |
| r141 pr_off_s1 | 26 / 32 | 6.2 (4-11); 0 by r3 | project 0.12, research 1.47 | 0.16 / 1 | 2.78 | 0.00 | 0.00 | 0.66 / 0.00 | 0.09 | 0.09, in 2 games | 0.47 |
| r141 bp_s2 | 12 / 32 | 5.2 (4-9); 0 by r3 | research 0.69 | 0.16 / 2 | 0.44 | 0.00 | 0.00 | 0.25 / 0.00 | 0.44 | 0.25, in 3 games | 0.19 |
| r141 br_s2 | 14 / 32 | 5.5 (4-8); 0 by r3 | research 0.94 | 0.12 / 1 | 0.78 | 0.06 | 0.06 | 0.19 / 0.00 | 0.03 | 0.03, in 1 games | 0.34 |
| r141 pr_s2 | 13 / 32 | 5.5 (4-7); 0 by r3 | research 0.59 | 0.06 / 1 | 0.53 | 0.00 | 0.00 | 0.19 / 0.00 | 0.06 | 0.06, in 1 games | 0.09 |
| r141 bp_off_s2 | 30 / 32 | 5.0 (4-8); 0 by r3 | project 0.25, research 2.22 | 0.25 / 2 | 2.91 | 0.19 | 0.16 | 0.66 / 0.00 | 0.38 | 0.31, in 5 games | 0.84 |
| r141 br_off_s2 | 31 / 32 | 5.5 (4-11); 0 by r3 | project 0.25, research 3.19 | 0.22 / 1 | 2.66 | 0.12 | 0.09 | 0.56 / 0.03 | 0.19 | 0.16, in 4 games | 1.06 |
| r141 pr_off_s2 | 31 / 32 | 5.1 (4-9); 0 by r3 | project 0.12, research 2.09 | 0.22 / 1 | 3.47 | 0.09 | 0.06 | 0.78 / 0.00 | 0.16 | 0.16, in 5 games | 0.53 |

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 32 | 27 (84%), 9 by r5 | 6.7 (5-10) | research 5 | 1:1 2:5 3:5 4:6 5:6 6:4 7:5 | 18.5 / 33 | 17.1 / 39 | 9.6 / 17 | 0.00 / 0 |
| r140 br | 32 | 25 (78%), 3 by r5 | 7.2 (5-12) | research 7 | 1:1 2:3 3:5 4:4 5:4 6:8 7:7 | 18.5 / 33 | 20.1 / 34 | 10.7 / 16 | 0.00 / 0 |
| r140 pr | 32 | 25 (78%), 5 by r5 | 7.2 (3-12) | research 7 | 1:1 2:5 3:7 4:6 5:4 6:2 7:7 | 20.1 / 44 | 16.3 / 40 | 9.2 / 17 | 0.03 / 1 |
| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max | Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game | Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r140 bp | 25 / 32 | 2.6 (1-6); 19 by r3 | project 0.12, threat 1.22 | 0.00 / 0 | 0.00 | 0.75 | 0.72 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.34 |
| r140 br | 27 / 32 | 2.6 (1-11); 22 by r3 | muster 0.12, threat 1.25 | 0.00 / 0 | 0.00 | 1.31 | 1.22 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.00 |
| r140 pr | 24 / 32 | 2.2 (1-5); 20 by r3 | muster 0.09, threat 1.22 | 0.00 / 0 | 0.00 | 0.66 | 0.53 | 0.00 / 0.00 | 0.00 | 0.00, in 0 games | 0.00 |
