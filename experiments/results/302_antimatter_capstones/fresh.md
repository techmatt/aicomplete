# 302: the fresh read

The judge, `read.json` (two seats, the three pairs, d0-d3), fresh seeds 51168-51263, 288 games a level: r146
is main at 05d7043f farmed with `--at`; r147 is this item's build (0b0c8d77). Columns as in 301's
[screens.md](../301_d0_early_wins_and_liberate_pricing/screens.md).

| Read | Level | Games | Wins | Rounds | 6-8 | 10+ | Cap losses of losses | Won by r3 | <=5 | 6 | 7 | 8 | 9 | 10 | 11 | 12+ |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| r146 | d0 | 288 | 90% | 7.1 | 54% | 14% (miss) | 29 of 30 | 3 | 25 | 18 | 22 | 14 | 7 | 4 | 10 | 0 |
| r146 | d1 | 288 | 88% | 7.2 | 49% (miss) | 16% (miss) | 36 of 36 | 2 | 25 | 16 | 19 | 14 | 11 | 3 | 12 | 0 |
| r146 | d2 | 288 | 80% | 7.3 | 47% (miss) | 19% (miss) | 40 of 59 | 1 | 23 | 16 | 17 | 14 | 11 | 8 | 11 | 0 |
| r146 | d3 | 288 | 69% | 7.4 | 49% (miss) | 16% (miss) | 58 of 89 | 2 | 18 | 16 | 17 | 17 | 17 | 16 | 0 | 0 |
| r147 | d0 | 288 | 92% | 7.0 | 54% | 14% (miss) | 22 of 23 | 3 | 25 | 20 | 18 | 15 | 7 | 6 | 8 | 0 |
| r147 | d1 | 288 | 86% | 7.2 | 50% | 18% (miss) | 39 of 39 | 2 | 24 | 18 | 19 | 14 | 8 | 5 | 13 | 0 |
| r147 | d2 | 288 | 82% | 7.3 | 47% (miss) | 18% (miss) | 38 of 52 | 2 | 24 | 14 | 20 | 13 | 11 | 7 | 11 | 0 |
| r147 | d3 | 288 | 67% | 7.4 | 50% (miss) | 18% (miss) | 65 of 96 | 3 | 18 | 16 | 16 | 17 | 14 | 18 | 0 | 0 |

## The liberate cards, per 100 games

| Per 100 games | r146 | r147 |
|---|---|---|
| citywide_microgrid played | 32.6 | 32.0 |
| power_to_the_people played | 15.5 | 16.1 |
| hostile_buyout played | 23.3 | 22.0 |
| root_access played | 19.1 | 19.7 |
| peacekeeping_mission played | 14.2 | 15.1 |
| forensic_audit played | 9.8 | 9.9 |
| popular_uprising played | 18.2 | 18.1 |
| nuclear_option played | 20.9 | 22.1 |
| great_liberation played | 6.2 | 6.4 |
| Liberate actions | 588 | 594 |
| cities freed by card | 134 | 134 |
| cities liberated | 617 | 619 |
| sites cleansed by card | 122 | 119 |

## Antimatter, per seat a game (bench/resource_use.py; new = r147)

| d | side | games | win % | chips gained / spent | energy gained / spent | research_cubes gained / spent | antimatter gained / spent | antimatter spent: games / won games |
|---|---|---|---|---|---|---|---|---|
| d0 | new | 288 | 92.0 | 5.38 / 3.07 | 40.41 / 30.19 | 7.40 / 4.37 | 3.57 / 1.31 | 86% / 85% |
| d0 | old | 288 | 89.6 | 5.29 / 3.08 | 39.84 / 30.06 | 7.32 / 4.23 | 3.59 / 1.30 | 85% / 84% |
| d1 | new | 288 | 86.5 | 5.19 / 3.19 | 42.11 / 32.02 | 8.08 / 4.80 | 3.79 / 1.29 | 89% / 88% |
| d1 | old | 288 | 87.5 | 5.17 / 3.13 | 40.97 / 31.69 | 8.08 / 4.72 | 3.70 / 1.27 | 90% / 88% |
| d2 | new | 288 | 81.9 | 4.94 / 3.17 | 41.16 / 31.20 | 7.97 / 4.70 | 3.66 / 1.31 | 85% / 83% |
| d2 | old | 288 | 79.5 | 4.99 / 3.17 | 40.84 / 31.40 | 8.00 / 4.69 | 3.74 / 1.32 | 85% / 83% |
| d3 | new | 288 | 66.7 | 4.73 / 3.11 | 39.15 / 31.20 | 7.63 / 4.46 | 3.60 / 1.19 | 81% / 81% |
| d3 | old | 288 | 69.1 | 4.67 / 3.04 | 38.06 / 30.56 | 7.45 / 4.39 | 3.50 / 1.19 | 82% / 83% |

