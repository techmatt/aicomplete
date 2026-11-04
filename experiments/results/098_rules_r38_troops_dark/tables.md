# 098 tables

From `scripts/cards/r38_read.py` (per game) and `scripts/bench/paired.py`, seeds 31900-31947. r37: the r37 binary (with the r38 counters patched in, play unchanged); f38: the final r38 build; t0: r37 binary with alignment.troop 0; split: r38 build with r37's troop-card prices (-old) and also alignment.troop 1 (-old-t1); f38-terms: w.city_dark_aligned 0.

| run | cell | games | win % | wins: purge / Unity deck / other | losses by reason | Unity Sway | Unity tips | Discord tips | troops removed |
|---|---|---|---|---|---|---|---|---|---|
| r37-d2 | bp | 48 | 43.8 | 4 / 17 / 0 | discord_deck 15, node 12 | 26.10 | 4.35 | 5.46 | 5.40 |
| r37-d2 | br | 48 | 68.8 | 7 / 26 / 0 | discord_deck 9, node 6 | 29.23 | 5.00 | 5.02 | 1.65 |
| r37-d2 | pr | 48 | 70.8 | 4 / 30 / 0 | discord_deck 8, node 6 | 28.92 | 5.08 | 4.90 | 6.17 |
| t0-d2 | bp | 48 | 31.2 | 5 / 10 / 0 | discord_deck 20, node 13 | 22.62 | 3.69 | 5.69 | 4.73 |
| t0-d2 | br | 48 | 45.8 | 8 / 14 / 0 | discord_deck 18, node 8 | 27.08 | 4.50 | 5.33 | 1.52 |
| t0-d2 | pr | 48 | 45.8 | 5 / 17 / 0 | discord_deck 18, node 8 | 26.38 | 4.42 | 5.44 | 6.00 |
| split-d2 | bp-old | 48 | 18.8 | 1 / 8 / 0 | node 20, discord_deck 19 | 21.29 | 3.48 | 5.60 | 4.62 |
| split-d2 | bp-old-t1 | 48 | 39.6 | 1 / 18 / 0 | discord_deck 17, node 12 | 25.58 | 4.35 | 5.44 | 5.08 |
| split-d2 | br-old | 48 | 43.8 | 4 / 17 / 0 | discord_deck 14, node 12, deck_empty 1 | 24.56 | 4.10 | 5.27 | 1.58 |
| split-d2 | br-old-t1 | 48 | 50.0 | 5 / 19 / 0 | discord_deck 13, node 10, deck_empty 1 | 25.50 | 4.35 | 5.17 | 1.75 |
| split-d2 | pr-old | 48 | 35.4 | 3 / 14 / 0 | discord_deck 22, node 9 | 24.40 | 4.17 | 5.77 | 5.25 |
| split-d2 | pr-old-t1 | 48 | 52.1 | 5 / 20 / 0 | discord_deck 14, node 9 | 26.96 | 4.60 | 5.12 | 5.35 |
| f38 | bp | 48 | 22.9 | 2 / 9 / 0 | node 23, discord_deck 14 | 20.92 | 3.29 | 5.46 | 5.71 |
| f38 | br | 48 | 35.4 | 3 / 14 / 0 | discord_deck 18, node 13 | 23.92 | 4.00 | 5.40 | 1.81 |
| f38 | pr | 48 | 37.5 | 1 / 17 / 0 | discord_deck 21, node 9 | 25.56 | 4.33 | 6.04 | 7.54 |
| f38-terms | bp | 48 | 14.6 | 1 / 6 / 0 | node 22, discord_deck 19 | 19.31 | 3.04 | 5.54 | 5.69 |
| f38-terms | br | 48 | 31.2 | 4 / 11 / 0 | node 18, discord_deck 14, deck_empty 1 | 24.06 | 4.00 | 5.40 | 1.56 |
| f38-terms | pr | 48 | 29.2 | 0 / 14 / 0 | discord_deck 19, node 15 | 24.04 | 3.98 | 5.73 | 6.62 |
| r37-d0 | bp | 48 | 77.1 | 9 / 28 / 0 | discord_deck 7, node 4 | 30.25 | 5.17 | 4.96 | 5.69 |
| r37-d0 | br | 48 | 93.8 | 15 / 30 / 0 | discord_deck 3 | 30.73 | 5.40 | 4.48 | 1.77 |
| r37-d0 | pr | 48 | 85.4 | 8 / 33 / 0 | discord_deck 5, node 2 | 31.21 | 5.42 | 4.60 | 6.38 |
| t0-d0 | bp | 48 | 54.2 | 8 / 18 / 0 | discord_deck 21, node 1 | 28.38 | 4.75 | 5.67 | 5.38 |
| t0-d0 | br | 48 | 89.6 | 19 / 24 / 0 | discord_deck 5 | 30.60 | 5.17 | 4.56 | 1.46 |
| t0-d0 | pr | 48 | 60.4 | 6 / 23 / 0 | discord_deck 16, node 3 | 29.21 | 4.88 | 5.33 | 6.60 |
| split-d0 | bp-old | 48 | 60.4 | 7 / 22 / 0 | discord_deck 14, node 3, deck_empty 2 | 29.12 | 4.94 | 5.52 | 6.42 |
| split-d0 | bp-old-t1 | 48 | 75.0 | 6 / 30 / 0 | discord_deck 6, node 6 | 29.79 | 5.06 | 5.04 | 5.29 |
| split-d0 | br-old | 48 | 68.8 | 6 / 27 / 0 | discord_deck 12, node 3 | 31.17 | 5.25 | 5.10 | 2.02 |
| split-d0 | br-old-t1 | 48 | 75.0 | 6 / 30 / 0 | discord_deck 8, node 4 | 29.54 | 5.25 | 4.94 | 1.77 |
| split-d0 | pr-old | 48 | 70.8 | 3 / 31 / 0 | discord_deck 9, node 4, deck_empty 1 | 29.21 | 5.10 | 5.08 | 6.81 |
| split-d0 | pr-old-t1 | 48 | 70.8 | 4 / 30 / 0 | discord_deck 8, node 6 | 29.21 | 5.15 | 5.02 | 5.48 |
| f38-d0 | bp | 48 | 58.3 | 7 / 21 / 0 | discord_deck 14, node 6 | 28.27 | 4.73 | 5.21 | 6.77 |
| f38-d0 | br | 48 | 79.2 | 13 / 25 / 0 | discord_deck 9, node 1 | 30.85 | 5.12 | 4.73 | 1.81 |
| f38-d0 | pr | 48 | 58.3 | 9 / 19 / 0 | discord_deck 16, node 4 | 29.19 | 4.79 | 5.38 | 6.92 |

| run | cell | dark neutral city-rounds | dark aligned city-rounds | energy from aligned cities | energy bought from neutral | nodes placed | nodes lost | corruptions |
|---|---|---|---|---|---|---|---|---|
| r37-d2 | bp | 5.67 | 0.00 | 7.71 | 2.35 | 18.00 | 9.00 | 2.85 |
| r37-d2 | br | 6.00 | 0.00 | 8.21 | 2.98 | 17.15 | 8.42 | 2.58 |
| r37-d2 | pr | 5.60 | 0.00 | 8.75 | 2.38 | 16.60 | 6.75 | 2.38 |
| t0-d2 | bp | 5.81 | 0.00 | 8.27 | 2.77 | 18.42 | 9.44 | 2.90 |
| t0-d2 | br | 6.04 | 0.00 | 8.21 | 3.04 | 17.42 | 8.77 | 2.75 |
| t0-d2 | pr | 6.02 | 0.00 | 9.90 | 2.52 | 17.65 | 8.31 | 2.56 |
| split-d2 | bp-old | 5.90 | 2.98 | 8.33 | 2.25 | 17.73 | 10.98 | 3.67 |
| split-d2 | bp-old-t1 | 6.02 | 2.69 | 8.35 | 2.17 | 17.88 | 9.79 | 3.33 |
| split-d2 | br-old | 6.17 | 2.81 | 8.69 | 2.44 | 17.33 | 9.79 | 3.27 |
| split-d2 | br-old-t1 | 5.77 | 2.81 | 8.35 | 2.46 | 17.27 | 9.73 | 3.25 |
| split-d2 | pr-old | 5.35 | 2.67 | 9.23 | 2.83 | 17.81 | 9.98 | 3.31 |
| split-d2 | pr-old-t1 | 5.29 | 2.56 | 8.08 | 2.54 | 16.73 | 8.40 | 3.04 |
| f38 | bp | 5.62 | 2.83 | 8.42 | 2.42 | 17.23 | 11.35 | 3.81 |
| f38 | br | 6.42 | 3.19 | 8.08 | 2.46 | 17.08 | 10.60 | 3.48 |
| f38 | pr | 6.02 | 2.90 | 9.90 | 2.85 | 18.75 | 10.04 | 3.42 |
| f38-terms | bp | 5.06 | 4.06 | 6.98 | 2.48 | 16.75 | 10.54 | 3.60 |
| f38-terms | br | 6.06 | 4.21 | 7.08 | 2.71 | 16.98 | 10.10 | 3.23 |
| f38-terms | pr | 6.04 | 4.33 | 7.54 | 2.44 | 17.21 | 10.46 | 3.65 |
| r37-d0 | bp | 5.75 | 0.00 | 10.73 | 2.06 | 18.38 | 6.50 | 2.19 |
| r37-d0 | br | 6.17 | 0.00 | 11.56 | 2.46 | 18.88 | 5.19 | 1.73 |
| r37-d0 | pr | 5.83 | 0.00 | 10.00 | 2.62 | 17.54 | 5.56 | 1.85 |
| t0-d0 | bp | 6.35 | 0.00 | 11.94 | 2.50 | 20.08 | 7.15 | 2.29 |
| t0-d0 | br | 6.17 | 0.00 | 11.79 | 2.77 | 19.19 | 5.19 | 1.69 |
| t0-d0 | pr | 6.48 | 0.00 | 11.48 | 3.35 | 19.73 | 7.44 | 2.38 |
| split-d0 | bp-old | 6.21 | 3.17 | 12.17 | 2.56 | 20.25 | 9.65 | 2.96 |
| split-d0 | bp-old-t1 | 5.81 | 2.94 | 11.25 | 2.00 | 19.00 | 8.69 | 2.85 |
| split-d0 | br-old | 6.54 | 3.56 | 11.60 | 3.06 | 19.21 | 7.96 | 2.71 |
| split-d0 | br-old-t1 | 6.10 | 3.44 | 11.67 | 2.90 | 18.46 | 7.52 | 2.46 |
| split-d0 | pr-old | 6.31 | 3.10 | 11.71 | 3.29 | 18.94 | 8.25 | 2.73 |
| split-d0 | pr-old-t1 | 5.19 | 2.90 | 10.15 | 3.23 | 18.00 | 7.48 | 2.58 |
| f38-d0 | bp | 6.42 | 3.40 | 11.71 | 2.67 | 19.58 | 8.62 | 2.75 |
| f38-d0 | br | 6.15 | 2.98 | 11.81 | 3.23 | 18.62 | 6.38 | 2.19 |
| f38-d0 | pr | 5.88 | 3.15 | 11.50 | 3.10 | 19.50 | 7.90 | 2.77 |

## Troop-removal card plays per game

| card | r37-d2 | f38 | r37-d0 | f38-d0 |
|---|---|---|---|---|
| logic_bomb | 0.25 | 0.28 | 0.40 | 0.30 |
| signal_hijack | 0.36 | 0.33 | 0.36 | 0.41 |
| ceasefire_monitors | 0.66 | 0.64 | 0.72 | 0.73 |
| kill_team_roster | 0.39 | 0.40 | 0.40 | 0.35 |
| bounty_board | 0.32 | 0.31 | 0.36 | 0.35 |
| interceptor_battery | 0.16 | 0.38 | 0.06 | 0.40 |
| breach_and_clear | 0.43 | 0.43 | 0.53 | 0.51 |
| agency_directorate | 0.32 | 0.35 | 0.32 | 0.33 |
| kinetic_interdiction | 0.01 | 0.03 | 0.04 | 0.02 |
| total_containment | 0.37 | 0.33 | 0.36 | 0.31 |
| force_protection_order | 0.03 | 0.02 | 0.04 | 0.02 |
| peacekeeping_mission | 0.33 | 0.28 | 0.28 | 0.37 |

## Fresh seeds, 31948-31995

| run | cell | games | win % | wins: purge / Unity deck / other | losses by reason | Unity Sway | Unity tips | Discord tips | troops removed |
|---|---|---|---|---|---|---|---|---|---|
| fr37 | bp | 48 | 50.0 | 5 / 19 / 0 | node 12, discord_deck 12 | 26.15 | 4.50 | 5.29 | 5.10 |
| fr37 | br | 48 | 54.2 | 8 / 18 / 0 | discord_deck 16, node 6 | 28.02 | 4.71 | 5.62 | 2.12 |
| fr37 | pr | 48 | 60.4 | 4 / 25 / 0 | discord_deck 12, node 7 | 28.96 | 4.94 | 5.44 | 6.50 |
| frt0 | bp | 48 | 35.4 | 5 / 12 / 0 | discord_deck 18, node 13 | 22.40 | 3.81 | 5.65 | 4.48 |
| frt0 | br | 48 | 43.8 | 8 / 13 / 0 | discord_deck 19, node 8 | 26.94 | 4.33 | 5.62 | 1.92 |
| frt0 | pr | 48 | 35.4 | 2 / 15 / 0 | discord_deck 24, node 7 | 25.88 | 4.27 | 6.15 | 5.96 |
| frsplit | bp-old | 48 | 22.9 | 2 / 9 / 0 | discord_deck 23, node 14 | 23.69 | 3.79 | 5.88 | 4.29 |
| frsplit | bp-old-t1 | 48 | 35.4 | 2 / 15 / 0 | discord_deck 18, node 13 | 26.04 | 4.42 | 5.69 | 3.92 |
| frsplit | br-old | 48 | 41.7 | 3 / 17 / 0 | discord_deck 17, node 11 | 24.85 | 4.12 | 5.54 | 1.35 |
| frsplit | br-old-t1 | 48 | 35.4 | 4 / 13 / 0 | discord_deck 19, node 12 | 24.62 | 4.12 | 5.44 | 1.40 |
| frsplit | pr-old | 48 | 20.8 | 1 / 9 / 0 | discord_deck 23, node 15 | 22.94 | 3.52 | 6.17 | 5.79 |
| frsplit | pr-old-t1 | 48 | 41.7 | 1 / 19 / 0 | node 14, discord_deck 14 | 27.15 | 4.65 | 5.69 | 5.06 |
| fr38 | bp | 48 | 29.2 | 0 / 14 / 0 | discord_deck 22, node 12 | 24.12 | 3.88 | 5.77 | 5.33 |
| fr38 | br | 48 | 37.5 | 4 / 14 / 0 | discord_deck 18, node 12 | 25.65 | 4.23 | 5.60 | 1.52 |
| fr38 | pr | 48 | 33.3 | 3 / 13 / 0 | node 18, discord_deck 14 | 24.54 | 4.00 | 5.67 | 5.48 |
| fr37-d0 | bp | 48 | 66.7 | 5 / 27 / 0 | discord_deck 11, node 5 | 29.44 | 5.10 | 5.19 | 5.08 |
| fr37-d0 | br | 48 | 75.0 | 13 / 23 / 0 | discord_deck 9, node 3 | 28.58 | 4.96 | 4.90 | 1.69 |
| fr37-d0 | pr | 48 | 85.4 | 8 / 33 / 0 | discord_deck 5, node 2 | 31.40 | 5.46 | 4.90 | 6.60 |
| fr38-d0 | bp | 48 | 58.3 | 6 / 22 / 0 | discord_deck 15, node 5 | 28.29 | 4.69 | 5.58 | 6.77 |
| fr38-d0 | br | 48 | 68.8 | 10 / 23 / 0 | discord_deck 14, node 1 | 29.73 | 5.00 | 5.19 | 1.33 |
| fr38-d0 | pr | 48 | 52.1 | 6 / 19 / 0 | discord_deck 21, node 2 | 28.67 | 4.75 | 5.73 | 7.15 |

| run | cell | dark neutral city-rounds | dark aligned city-rounds | energy from aligned cities | energy bought from neutral | nodes placed | nodes lost | corruptions |
|---|---|---|---|---|---|---|---|---|
| fr37 | bp | 5.77 | 0.00 | 8.02 | 1.79 | 17.65 | 8.90 | 2.90 |
| fr37 | br | 5.08 | 0.00 | 8.96 | 2.62 | 17.46 | 8.96 | 2.69 |
| fr37 | pr | 5.67 | 0.00 | 8.06 | 2.92 | 17.25 | 7.67 | 2.48 |
| frt0 | bp | 5.94 | 0.00 | 8.23 | 1.83 | 18.17 | 10.04 | 3.25 |
| frt0 | br | 5.58 | 0.00 | 9.35 | 2.67 | 17.88 | 8.90 | 2.60 |
| frt0 | pr | 6.40 | 0.00 | 8.40 | 2.54 | 18.19 | 9.73 | 3.04 |
| frsplit | bp-old | 5.94 | 2.54 | 8.50 | 2.35 | 19.23 | 11.17 | 3.62 |
| frsplit | bp-old-t1 | 5.65 | 2.67 | 8.29 | 2.21 | 18.44 | 11.02 | 3.44 |
| frsplit | br-old | 5.54 | 2.79 | 9.10 | 2.29 | 17.17 | 9.42 | 3.15 |
| frsplit | br-old-t1 | 5.44 | 2.56 | 9.19 | 2.35 | 17.15 | 8.85 | 2.96 |
| frsplit | pr-old | 5.73 | 2.71 | 7.77 | 2.25 | 17.90 | 10.60 | 3.81 |
| frsplit | pr-old-t1 | 5.50 | 2.58 | 7.58 | 2.15 | 17.44 | 10.04 | 3.71 |
| fr38 | bp | 6.50 | 2.98 | 8.85 | 2.29 | 19.12 | 11.27 | 3.65 |
| fr38 | br | 5.56 | 2.65 | 8.88 | 2.48 | 16.92 | 9.21 | 3.38 |
| fr38 | pr | 6.00 | 2.83 | 8.35 | 2.19 | 16.85 | 10.60 | 3.67 |
| fr37-d0 | bp | 6.21 | 0.00 | 10.92 | 2.92 | 19.52 | 7.29 | 2.10 |
| fr37-d0 | br | 5.94 | 0.00 | 10.62 | 3.33 | 17.96 | 4.96 | 1.60 |
| fr37-d0 | pr | 5.31 | 0.00 | 10.06 | 3.19 | 18.50 | 5.94 | 2.00 |
| fr38-d0 | bp | 6.69 | 3.10 | 11.40 | 3.23 | 20.58 | 10.90 | 3.27 |
| fr38-d0 | br | 6.17 | 3.15 | 11.06 | 3.71 | 19.06 | 7.67 | 2.46 |
| fr38-d0 | pr | 6.06 | 3.10 | 10.38 | 3.38 | 19.38 | 9.25 | 3.12 |
