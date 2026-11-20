# 295: the early-win lever screen

The judge at d0 on the dev seeds (36800-36831), the three pairs, 32 games a pair and 96 a cell, farm spec
`levers295`, under r142's rules (an ambassador moves in, then acts). Cells: `asked` the prompt's numbers (`liberate_at`
5, one corrupted city a country, arrives ready); `c2` two corrupted cities a country at setup
(`board.start_corrupted` 2); `l6`, `l7` a city's Liberate at 6 and 7 ambassadors; `a0` the arrives-spent arm
(`board.arrive_ready` 0); and their crosses. The pick is `c2`.

## The pick at 504 games a cell (spec `levers295c`, fresh seeds 50472-50639, d0, the shipped build, pairs pooled)

The size tuning-items.md asks of a retuned value: 168 seeds a pair, paired across the three cells.

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| fresh asked | 504 | 489 (97%) | 5.9 (1-14) | 112 (22%) | 224 (44%) | 1 | liberated 489, deck_empty 11, research 4 | 3.11 (1.08 by card) | 0.60 | 2.52 | 12.6 |
| fresh c2 | 504 | 464 (92%) | 8.0 (3-14) | 3 (1%) | 70 (14%) | 3 | liberated 464, deck_empty 24, research 15, discord_deck 1 | 5.35 (1.61 by card) | 1.11 | 4.66 | 16.4 |
| fresh l7 | 504 | 470 (93%) | 7.3 (1-14) | 44 (9%) | 127 (25%) | 1 | liberated 470, deck_empty 31, research 3 | 3.31 (1.53 by card) | 0.84 | 2.45 | 14.9 |

## The same, a pair a row

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| fresh bp | 168 | 164 (98%) | 5.5 (1-14) | 53 (32%) | 93 (55%) | 1 | liberated 164, deck_empty 3, research 1 | 2.88 (0.83 by card) | 0.52 | 2.46 | 9.9 |
| fresh br | 168 | 161 (96%) | 6.7 (1-14) | 19 (11%) | 54 (32%) | 1 | liberated 161, deck_empty 5, research 2 | 3.46 (1.42 by card) | 0.68 | 2.58 | 15.3 |
| fresh pr | 168 | 164 (98%) | 5.7 (1-14) | 40 (24%) | 77 (46%) | 1 | liberated 164, deck_empty 3, research 1 | 2.98 (0.98 by card) | 0.60 | 2.52 | 12.6 |
| fresh bp_c2 | 168 | 151 (90%) | 8.0 (3-14) | 1 (1%) | 21 (12%) | 3 | liberated 151, research 10, deck_empty 7 | 5.27 (1.40 by card) | 1.13 | 4.85 | 14.7 |
| fresh br_c2 | 168 | 155 (92%) | 8.0 (3-14) | 1 (1%) | 25 (15%) | 3 | liberated 155, deck_empty 9, research 4 | 5.45 (1.80 by card) | 1.10 | 4.53 | 17.1 |
| fresh pr_c2 | 168 | 158 (94%) | 7.9 (3-14) | 1 (1%) | 24 (14%) | 3 | liberated 158, deck_empty 8, discord_deck 1, research 1 | 5.33 (1.62 by card) | 1.10 | 4.60 | 17.4 |
| fresh bp_l7 | 168 | 157 (93%) | 7.2 (1-14) | 19 (11%) | 43 (26%) | 1 | liberated 157, deck_empty 11 | 3.18 (1.33 by card) | 0.87 | 2.54 | 13.6 |
| fresh br_l7 | 168 | 156 (93%) | 7.8 (1-14) | 10 (6%) | 29 (17%) | 1 | liberated 156, deck_empty 10, research 2 | 3.54 (1.77 by card) | 0.83 | 2.43 | 16.3 |
| fresh pr_l7 | 168 | 157 (93%) | 7.0 (1-14) | 15 (9%) | 55 (33%) | 1 | liberated 157, deck_empty 10, research 1 | 3.22 (1.49 by card) | 0.81 | 2.38 | 14.7 |

## The build that ships (commit 1fdf34dd, after the diff review's fixes), pairs pooled

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r142 asked | 96 | 96 (100%) | 5.9 (1-13) | 22 (23%) | 47 (49%) | 1 | liberated 96 | 3.19 (1.11 by card) | 0.66 | 2.65 | 12.7 |
| r142 c2 | 96 | 86 (90%) | 8.1 (4-14) | 0 (0%) | 13 (14%) | 4 | liberated 86, deck_empty 8, research 2 | 5.52 (1.62 by card) | 1.29 | 5.04 | 16.9 |
| r142 l6 | 96 | 90 (94%) | 6.4 (2-14) | 18 (19%) | 38 (40%) | 2 | liberated 90, deck_empty 5, research 1 | 3.11 (1.30 by card) | 0.79 | 2.49 | 14.4 |
| r142 l7 | 96 | 91 (95%) | 7.0 (2-14) | 6 (6%) | 25 (26%) | 2 | liberated 91, deck_empty 4, research 1 | 3.29 (1.57 by card) | 0.84 | 2.42 | 14.1 |
| r142 c2_l6 | 96 | 84 (88%) | 9.1 (4-14) | 0 (0%) | 7 (7%) | 4 | liberated 84, deck_empty 6, research 6 | 5.53 (1.92 by card) | 1.60 | 5.04 | 19.2 |
| r142 a0 | 96 | 93 (97%) | 6.1 (2-14) | 19 (20%) | 42 (44%) | 2 | liberated 93, deck_empty 3 | 3.23 (1.22 by card) | 0.60 | 2.50 | 10.1 |
| r142 c2_a0 | 96 | 79 (82%) | 8.9 (5-14) | 0 (0%) | 11 (11%) | 5 | liberated 79, deck_empty 13, research 4 | 5.32 (1.96 by card) | 1.19 | 4.34 | 16.4 |
| r142 c2_l6_a0 | 96 | 81 (84%) | 9.4 (4-14) | 0 (0%) | 5 (5%) | 4 | liberated 81, research 8, deck_empty 7 | 5.34 (2.23 by card) | 1.39 | 4.32 | 16.5 |

## The same, a pair a row

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r142 bp | 32 | 32 (100%) | 5.8 (1-13) | 11 (34%) | 15 (47%) | 1 | liberated 32 | 3.25 (1.03 by card) | 0.69 | 2.81 | 12.1 |
| r142 br | 32 | 32 (100%) | 6.2 (2-13) | 4 (12%) | 16 (50%) | 2 | liberated 32 | 3.34 (1.19 by card) | 0.75 | 2.75 | 13.7 |
| r142 pr | 32 | 32 (100%) | 5.7 (2-13) | 7 (22%) | 16 (50%) | 2 | liberated 32 | 2.97 (1.12 by card) | 0.53 | 2.38 | 12.2 |
| r142 bp_c2 | 32 | 28 (88%) | 7.7 (4-14) | 0 (0%) | 4 (12%) | 4 | liberated 28, deck_empty 2, research 2 | 5.34 (1.47 by card) | 1.41 | 5.22 | 13.6 |
| r142 br_c2 | 32 | 30 (94%) | 8.6 (4-14) | 0 (0%) | 2 (6%) | 4 | liberated 30, deck_empty 2 | 5.78 (1.78 by card) | 1.09 | 4.91 | 19.4 |
| r142 pr_c2 | 32 | 28 (88%) | 8.1 (4-14) | 0 (0%) | 7 (22%) | 4 | liberated 28, deck_empty 4 | 5.44 (1.62 by card) | 1.38 | 5.00 | 17.6 |
| r142 bp_l6 | 32 | 29 (91%) | 6.3 (2-14) | 9 (28%) | 14 (44%) | 2 | liberated 29, deck_empty 2, research 1 | 3.12 (1.41 by card) | 0.62 | 2.25 | 12.1 |
| r142 br_l6 | 32 | 31 (97%) | 6.9 (2-14) | 4 (12%) | 11 (34%) | 2 | liberated 31, deck_empty 1 | 3.38 (1.28 by card) | 0.97 | 2.88 | 17.2 |
| r142 pr_l6 | 32 | 30 (94%) | 6.1 (2-14) | 5 (16%) | 13 (41%) | 2 | liberated 30, deck_empty 2 | 2.84 (1.22 by card) | 0.78 | 2.34 | 13.9 |
| r142 bp_l7 | 32 | 31 (97%) | 6.7 (2-14) | 3 (9%) | 10 (31%) | 2 | liberated 31, deck_empty 1 | 3.28 (1.50 by card) | 0.94 | 2.59 | 12.6 |
| r142 br_l7 | 32 | 31 (97%) | 7.5 (2-12) | 1 (3%) | 4 (12%) | 2 | liberated 31, research 1 | 3.50 (1.91 by card) | 0.81 | 2.25 | 14.7 |
| r142 pr_l7 | 32 | 29 (91%) | 6.8 (3-14) | 2 (6%) | 11 (34%) | 3 | liberated 29, deck_empty 3 | 3.09 (1.31 by card) | 0.78 | 2.41 | 15.1 |
| r142 bp_c2_l6 | 32 | 29 (91%) | 9.4 (5-14) | 0 (0%) | 1 (3%) | 5 | liberated 29, research 2, deck_empty 1 | 5.62 (1.84 by card) | 2.00 | 5.62 | 18.5 |
| r142 br_c2_l6 | 32 | 24 (75%) | 9.7 (5-14) | 0 (0%) | 1 (3%) | 5 | liberated 24, deck_empty 4, research 4 | 5.62 (2.44 by card) | 1.75 | 4.75 | 21.3 |
| r142 pr_c2_l6 | 32 | 31 (97%) | 8.2 (4-14) | 0 (0%) | 5 (16%) | 4 | liberated 31, deck_empty 1 | 5.34 (1.47 by card) | 1.06 | 4.75 | 17.8 |
| r142 bp_a0 | 32 | 31 (97%) | 5.8 (2-14) | 10 (31%) | 16 (50%) | 2 | liberated 31, deck_empty 1 | 3.06 (0.94 by card) | 0.47 | 2.47 | 8.8 |
| r142 br_a0 | 32 | 31 (97%) | 6.4 (2-14) | 4 (12%) | 13 (41%) | 2 | liberated 31, deck_empty 1 | 3.41 (1.41 by card) | 0.75 | 2.66 | 10.4 |
| r142 pr_a0 | 32 | 31 (97%) | 6.3 (2-14) | 5 (16%) | 13 (41%) | 2 | liberated 31, deck_empty 1 | 3.22 (1.31 by card) | 0.59 | 2.38 | 11.0 |
| r142 bp_c2_a0 | 32 | 23 (72%) | 9.2 (5-14) | 0 (0%) | 2 (6%) | 5 | liberated 23, deck_empty 6, research 3 | 4.97 (1.81 by card) | 1.03 | 3.94 | 15.3 |
| r142 br_c2_a0 | 32 | 29 (91%) | 9.2 (5-14) | 0 (0%) | 3 (9%) | 5 | liberated 29, deck_empty 2, research 1 | 5.62 (2.28 by card) | 1.19 | 4.34 | 18.2 |
| r142 pr_c2_a0 | 32 | 27 (84%) | 8.4 (5-14) | 0 (0%) | 6 (19%) | 5 | liberated 27, deck_empty 5 | 5.38 (1.78 by card) | 1.34 | 4.75 | 15.7 |
| r142 bp_c2_l6_a0 | 32 | 27 (84%) | 9.6 (5-14) | 0 (0%) | 1 (3%) | 5 | liberated 27, research 4, deck_empty 1 | 5.28 (2.22 by card) | 1.34 | 4.25 | 14.9 |
| r142 br_c2_l6_a0 | 32 | 27 (84%) | 9.0 (4-14) | 0 (0%) | 2 (6%) | 4 | liberated 27, research 3, deck_empty 2 | 5.56 (2.44 by card) | 1.25 | 4.28 | 15.7 |
| r142 pr_c2_l6_a0 | 32 | 27 (84%) | 9.5 (4-14) | 0 (0%) | 2 (6%) | 4 | liberated 27, deck_empty 4, research 1 | 5.19 (2.03 by card) | 1.56 | 4.44 | 18.8 |

## The first screen (commit 9601438e, before the diff review's fixes), pairs pooled

The levers were picked here; the build changed after it (a card's move no longer stands a tipped ambassador up, Peer
Review's benefit, the troop reach for "Agents or Troops"), so the screen above reran it.

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| first asked | 96 | 94 (98%) | 6.1 (1-14) | 23 (24%) | 43 (45%) | 1 | liberated 94, deck_empty 1, research 1 | 3.19 (1.18 by card) | 0.79 | 2.69 | 13.3 |
| first c2 | 96 | 86 (90%) | 8.1 (4-14) | 0 (0%) | 14 (15%) | 4 | liberated 86, deck_empty 8, research 2 | 5.44 (1.65 by card) | 1.31 | 4.97 | 16.9 |
| first l6 | 96 | 88 (92%) | 6.4 (2-14) | 20 (21%) | 44 (46%) | 2 | liberated 88, deck_empty 7, research 1 | 3.07 (1.25 by card) | 0.88 | 2.56 | 14.0 |
| first l7 | 96 | 85 (89%) | 7.5 (2-14) | 6 (6%) | 24 (25%) | 2 | liberated 85, deck_empty 10, research 1 | 3.29 (1.46 by card) | 1.18 | 2.82 | 16.1 |
| first c2_l6 | 96 | 88 (92%) | 8.9 (4-14) | 0 (0%) | 4 (4%) | 4 | liberated 88, deck_empty 6, research 2 | 5.54 (1.91 by card) | 1.65 | 5.07 | 18.3 |
| first a0 | 96 | 92 (96%) | 6.5 (2-14) | 18 (19%) | 40 (42%) | 2 | liberated 92, deck_empty 3, research 1 | 3.20 (1.11 by card) | 0.89 | 2.76 | 11.1 |
| first c2_a0 | 96 | 81 (84%) | 8.8 (4-14) | 0 (0%) | 10 (10%) | 4 | liberated 81, deck_empty 12, research 3 | 5.28 (1.94 by card) | 1.06 | 4.24 | 16.2 |
| first c2_l6_a0 | 96 | 81 (84%) | 9.5 (4-14) | 0 (0%) | 7 (7%) | 4 | liberated 81, deck_empty 9, research 5, discord_deck 1 | 5.36 (2.23 by card) | 1.45 | 4.35 | 18.0 |

## The two levers above d0 (spec `levers295d`, the shipped build, d0-d3, the same dev seeds, pairs pooled)

For information: 294's brake screen read d0 only. `asked` is r142's rules with one corrupted city a country, so its
gap to r141 is what the ambassador and node-power rules cost on their own, and `c2`'s gap to it is the lever's.

| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended | Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |
|---|---|---|---|---|---|---|---|---|---|---|---|
| dev asked d0 | 96 | 96 (100%) | 5.9 (1-13) | 22 (23%) | 47 (49%) | 1 | liberated 96 | 3.19 (1.11 by card) | 0.66 | 2.65 | 12.7 |
| dev asked d1 | 96 | 82 (85%) | 7.9 (1-14) | 6 (6%) | 20 (21%) | 1 | liberated 82, deck_empty 10, research 4 | 5.50 (2.01 by card) | 1.16 | 4.46 | 21.6 |
| dev asked d2 | 96 | 79 (82%) | 7.9 (1-14) | 6 (6%) | 16 (17%) | 1 | liberated 79, research 10, deck_empty 5, discord_deck 2 | 5.81 (1.84 by card) | 1.59 | 5.36 | 21.9 |
| dev asked d3 | 96 | 62 (65%) | 8.0 (1-12) | 2 (2%) | 6 (6%) | 1 | liberated 62, research 29, deck_empty 3, discord_deck 2 | 5.89 (1.70 by card) | 1.30 | 5.31 | 21.1 |
| dev c2 d0 | 96 | 86 (90%) | 8.1 (4-14) | 0 (0%) | 13 (14%) | 4 | liberated 86, deck_empty 8, research 2 | 5.52 (1.62 by card) | 1.29 | 5.04 | 16.9 |
| dev c2 d1 | 96 | 85 (89%) | 8.8 (5-14) | 0 (0%) | 3 (3%) | 5 | liberated 85, deck_empty 6, research 3, discord_deck 2 | 6.79 (2.11 by card) | 1.29 | 5.78 | 23.2 |
| dev c2 d2 | 96 | 70 (73%) | 8.6 (4-14) | 0 (0%) | 5 (5%) | 4 | liberated 70, research 18, deck_empty 8 | 7.02 (2.10 by card) | 1.66 | 6.43 | 22.0 |
| dev c2 d3 | 96 | 48 (50%) | 8.0 (5-12) | 0 (0%) | 4 (4%) | 5 | liberated 48, research 31, discord_deck 11, deck_empty 6 | 6.10 (1.58 by card) | 1.09 | 5.45 | 19.7 |
| dev l7 d0 | 96 | 91 (95%) | 7.0 (2-14) | 6 (6%) | 25 (26%) | 2 | liberated 91, deck_empty 4, research 1 | 3.29 (1.57 by card) | 0.84 | 2.42 | 14.1 |
| dev l7 d1 | 96 | 76 (79%) | 9.3 (2-14) | 3 (3%) | 11 (11%) | 2 | liberated 76, deck_empty 17, research 2, discord_deck 1 | 5.53 (2.36 by card) | 1.66 | 4.59 | 25.3 |
| dev l7 d2 | 96 | 70 (73%) | 8.9 (2-14) | 3 (3%) | 7 (7%) | 2 | liberated 70, research 17, deck_empty 8, discord_deck 1 | 5.78 (2.46 by card) | 1.54 | 4.68 | 23.2 |
| dev l7 d3 | 96 | 56 (58%) | 8.7 (3-12) | 1 (1%) | 2 (2%) | 3 | liberated 56, research 32, deck_empty 8 | 5.57 (2.01 by card) | 1.41 | 4.83 | 21.4 |
| dev c2_l6 d0 | 96 | 84 (88%) | 9.1 (4-14) | 0 (0%) | 7 (7%) | 4 | liberated 84, deck_empty 6, research 6 | 5.53 (1.92 by card) | 1.60 | 5.04 | 19.2 |
| dev c2_l6 d1 | 96 | 71 (74%) | 9.6 (6-14) | 0 (0%) | 0 (0%) | 6 | liberated 71, research 14, deck_empty 10, discord_deck 1 | 6.61 (2.21 by card) | 1.61 | 5.78 | 25.0 |
| dev c2_l6 d2 | 96 | 61 (64%) | 8.8 (5-14) | 0 (0%) | 2 (2%) | 5 | liberated 61, research 26, discord_deck 7, deck_empty 2 | 6.31 (2.24 by card) | 1.56 | 5.47 | 20.4 |
| dev c2_l6 d3 | 96 | 27 (28%) | 8.4 (5-12) | 0 (0%) | 1 (1%) | 5 | research 51, liberated 27, discord_deck 10, deck_empty 7, node 1 | 5.19 (1.55 by card) | 1.16 | 4.60 | 19.5 |
