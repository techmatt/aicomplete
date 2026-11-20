# 294: the brake screen (for Matt's question on early d0 wins)

The judge at d0 on the dev seeds (36800-36831), the three pairs, farm spec `brake294`: the shipped build (`bp`,
`br`, `pr`), then two brakes as a factorial, neither shipped: `_c2`, two corrupted cities a country at setup
(`board.start_corrupted` 2; the map prints 2), and `_l6`, a city's Liberate at 6 ambassadors (`board.liberate_at`
6; baseline 5). Read at d0 only: what either does at d1-d3 is not known.

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |
|---|---|---|---|---|---|---|---|---|---|
| r141 bp | 32 | 32 (100%), 20 by r5 | 4.8 (2-8) | - | 0:2 1:13 2:5 3:7 4:5 | 13.6 / 27 | 13.2 / 27 | 10.8 / 23 | 0.03 / 1 |
| r141 br | 32 | 32 (100%), 16 by r5 | 5.8 (2-10) | - | 0:1 1:11 2:10 3:7 4:1 5:2 | 12.2 / 23 | 16.4 / 32 | 11.3 / 23 | 0.00 / 0 |
| r141 pr | 32 | 31 (97%), 20 by r5 | 5.2 (2-14) | deck_empty 1 | 0:3 1:9 2:13 3:3 4:3 5:1 | 13.4 / 45 | 13.0 / 28 | 10.4 / 23 | 0.00 / 0 |
| r141 bp_c2 | 32 | 30 (94%), 11 by r5 | 6.6 (4-14) | deck_empty 1, research 1 | 1:5 2:11 3:7 4:5 5:2 6:1 7:1 | 13.0 / 37 | 27.3 / 52 | 14.4 / 31 | 0.06 / 1 |
| r141 br_c2 | 32 | 29 (91%), 7 by r5 | 7.1 (4-13) | research 3 | 0:1 1:3 2:9 3:10 4:1 5:3 6:2 7:3 | 12.7 / 23 | 31.2 / 60 | 15.8 / 37 | 0.09 / 2 |
| r141 pr_c2 | 32 | 30 (94%), 7 by r5 | 7.3 (4-13) | research 2 | 1:8 2:5 3:9 4:3 5:3 6:2 7:2 | 16.2 / 39 | 30.5 / 55 | 15.1 / 32 | 0.09 / 2 |
| r141 bp_l6 | 32 | 31 (97%), 17 by r5 | 5.8 (2-14) | deck_empty 1 | 0:1 1:11 2:7 3:8 4:4 5:1 | 14.6 / 38 | 15.2 / 39 | 11.7 / 22 | 0.03 / 1 |
| r141 br_l6 | 32 | 31 (97%), 13 by r5 | 6.5 (2-14) | deck_empty 1 | 1:10 2:9 3:5 4:7 5:1 | 13.9 / 33 | 17.4 / 39 | 12.8 / 23 | 0.06 / 1 |
| r141 pr_l6 | 32 | 32 (100%), 13 by r5 | 5.8 (2-11) | - | 1:10 2:10 3:9 4:3 | 13.6 / 24 | 15.4 / 31 | 11.8 / 21 | 0.00 / 0 |
| r141 bp_c2_l6 | 32 | 29 (91%), 4 by r5 | 8.1 (4-14) | deck_empty 3 | 1:5 2:9 3:10 4:3 5:4 6:1 | 15.8 / 41 | 32.2 / 54 | 14.8 / 27 | 0.12 / 2 |
| r141 br_c2_l6 | 32 | 29 (91%), 4 by r5 | 8.4 (4-14) | deck_empty 2, research 1 | 0:1 1:7 2:10 3:7 4:2 5:1 6:3 7:1 | 15.0 / 30 | 33.5 / 53 | 14.6 / 34 | 0.06 / 1 |
| r141 pr_c2_l6 | 32 | 28 (88%), 6 by r5 | 7.9 (4-14) | deck_empty 3, research 1 | 0:1 1:4 2:8 3:7 4:7 5:3 6:1 7:1 | 15.3 / 28 | 31.2 / 47 | 15.2 / 28 | 0.06 / 1 |

| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max | Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game | Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r141 bp | 5 / 32 | 6.4 (6-7); 0 by r3 | research 0.22 | 0.00 / 0 | 0.16 | 0.00 | 0.00 | 0.16 / 0.00 | 0.00 | 0.00, in 0 games | 0.09 |
| r141 br | 3 / 32 | 6.3 (6-7); 0 by r3 | research 0.19 | 0.00 / 0 | 0.47 | 0.00 | 0.00 | 0.16 / 0.00 | 0.00 | 0.00, in 0 games | 0.09 |
| r141 pr | 4 / 32 | 6.8 (6-8); 0 by r3 | research 0.16 | 0.00 / 0 | 0.44 | 0.00 | 0.00 | 0.09 / 0.00 | 0.00 | 0.00, in 0 games | 0.12 |
| r141 bp_c2 | 8 / 32 | 6.5 (6-8); 0 by r3 | research 0.44 | 0.06 / 1 | 0.62 | 0.00 | 0.00 | 0.22 / 0.03 | 0.06 | 0.06, in 1 games | 0.16 |
| r141 br_c2 | 9 / 32 | 6.1 (5-7); 0 by r3 | research 0.88 | 0.03 / 1 | 0.91 | 0.12 | 0.12 | 0.22 / 0.00 | 0.03 | 0.00, in 0 games | 0.25 |
| r141 pr_c2 | 10 / 32 | 6.6 (5-9); 0 by r3 | research 0.66 | 0.00 / 0 | 1.16 | 0.09 | 0.09 | 0.22 / 0.00 | 0.00 | 0.00, in 0 games | 0.06 |
| r141 bp_l6 | 5 / 32 | 7.0 (6-8); 0 by r3 | research 0.19 | 0.00 / 0 | 0.44 | 0.03 | 0.03 | 0.19 / 0.00 | 0.00 | 0.00, in 0 games | 0.06 |
| r141 br_l6 | 8 / 32 | 7.5 (6-12); 0 by r3 | research 0.50 | 0.03 / 1 | 0.78 | 0.00 | 0.00 | 0.28 / 0.00 | 0.03 | 0.03, in 1 games | 0.16 |
| r141 pr_l6 | 3 / 32 | 6.7 (6-8); 0 by r3 | research 0.12 | 0.00 / 0 | 0.41 | 0.00 | 0.00 | 0.09 / 0.00 | 0.00 | 0.00, in 0 games | 0.06 |
| r141 bp_c2_l6 | 8 / 32 | 7.6 (6-9); 0 by r3 | project 0.06, research 0.47 | 0.06 / 1 | 1.31 | 0.09 | 0.06 | 0.44 / 0.00 | 0.22 | 0.22, in 2 games | 0.19 |
| r141 br_c2_l6 | 8 / 32 | 7.2 (6-9); 0 by r3 | discord 0.06, research 0.69 | 0.06 / 1 | 1.50 | 0.06 | 0.06 | 0.41 / 0.00 | 0.03 | 0.03, in 1 games | 0.03 |
| r141 pr_c2_l6 | 11 / 32 | 6.7 (5-8); 0 by r3 | research 0.56 | 0.06 / 1 | 1.28 | 0.00 | 0.00 | 0.31 / 0.00 | 0.03 | 0.03, in 1 games | 0.09 |
