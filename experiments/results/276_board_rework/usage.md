# 276: usage bands on the fresh read (f276-read, r131, seeds 49932-49979, the judge and greedy pooled)

`python scripts/design_check.py out/276_board_rework/f276-new.jsonl=experiments/farm/f276-read.json --usage`, the power rows (columns: pooled, then by board seat, then d0-d4).

| action | key | role | measure | band | use | in? | builder | protector | researcher | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Builder Buy In | `power:builder:Buy In` | node-power | seat_round | 0.10-0.90 | 0.26 | in | 0.26 | - | - | 0.27 | 0.27 | 0.27 | 0.24 | 0.21 |
| Builder Sprawl | `power:builder:Sprawl` | node-power | seat_round | 0.05-0.90 | 0.40 | in | 0.40 | - | - | 0.40 | 0.37 | 0.37 | 0.41 | 0.46 |
| Builder Annex | `power:builder:Annex` | node-power | seat_round | 0.05-0.90 | 0.07 | in | 0.07 | - | - | 0.05 | 0.07 | 0.07 | 0.09 | 0.08 |
| Protector Accord | `power:protector:Accord` | node-power | seat_round | 0.10-0.90 | 0.44 | in | - | 0.44 | - | 0.41 | 0.46 | 0.47 | 0.45 | 0.43 |
| Protector Outpost | `power:protector:Outpost` | node-power | seat_round | 0.05-0.90 | 0.27 | in | - | 0.27 | - | 0.30 | 0.26 | 0.24 | 0.27 | 0.31 |
| Protector Levy | `power:protector:Levy` | node-power | seat_round | 0.05-0.90 | 0.05 | in | - | 0.05 | - | 0.06 | 0.05 | 0.05 | 0.06 | 0.05 |
| Researcher Field Survey | `power:researcher:Field Survey` | node-power | seat_round | 0.10-0.90 | 0.34 | in | - | - | 0.34 | 0.36 | 0.35 | 0.34 | 0.32 | 0.30 |
| Researcher Peer Review | `power:researcher:Peer Review` | node-power | seat_round | 0.05-0.90 | 0.31 | in | - | - | 0.31 | 0.29 | 0.28 | 0.29 | 0.32 | 0.36 |
| Researcher Field Station | `power:researcher:Field Station` | node-power | seat_round | 0.05-0.90 | 0.08 | in | - | - | 0.08 | 0.08 | 0.08 | 0.08 | 0.08 | 0.07 |
| Researcher Lab I | `power:researcher:Lab I` | board-engine | seat_round | 0.40-0.90 | 0.57 | in | - | - | 0.57 | 0.61 | 0.60 | 0.59 | 0.54 | 0.46 |
| Researcher Lab II | `power:researcher:Lab II` | sacrifice | seat_game | 0.05-2.00 | 0.15 | in | - | - | 0.15 | 0.16 | 0.20 | 0.17 | 0.09 | 0.12 |
| Builder Scrap I | `power:builder:Scrap I` | sacrifice | seat_game | 0.25-2.00 | 1.70 | in | 1.70 | - | - | 2.05 | 2.12 | 1.90 | 1.35 | 1.09 |
| Builder Scrap II | `power:builder:Scrap II` | sacrifice | seat_game | 0.05-2.00 | 0.25 | in | 0.25 | - | - | 0.34 | 0.34 | 0.33 | 0.12 | 0.12 |
| Builder Overcharge I | `power:builder:Overcharge I` | sacrifice | seat_game | 0.25-2.00 | 1.13 | in | 1.13 | - | - | 1.35 | 1.57 | 1.39 | 0.69 | 0.66 |
| Builder Overcharge II | `power:builder:Overcharge II` | burn | seat_game | 0.02-0.50 | 0.10 | in | 0.10 | - | - | 0.15 | 0.13 | 0.15 | 0.08 | 0.02 |
| Builder Fortify I | `power:builder:Fortify I` | troop-response | seat_game | 0.25-2.50 | 1.00 | in | 1.00 | - | - | 0.56 | 1.11 | 1.29 | 1.17 | 0.86 |
| Builder Fortify II | `power:builder:Fortify II` | troop-response | seat_game | 0.05-2.50 | 0.67 | in | 0.67 | - | - | 0.38 | 0.92 | 0.89 | 0.65 | 0.49 |
| Protector Patrol I | `power:protector:Patrol I` | troop-response | seat_game | 0.25-3.50 | 3.37 | in | - | 3.37 | - | 3.18 | 4.16 | 4.06 | 3.08 | 2.36 |
| Protector Patrol II | `power:protector:Patrol II` | troop-response | seat_game | 0.05-2.50 | 0.84 | in | - | 0.84 | - | 1.44 | 1.12 | 1.10 | 0.37 | 0.16 |
| Protector Interdict I | `power:protector:Interdict I` | troop-response | seat_game | 0.25-2.50 | 1.54 | in | - | 1.54 | - | 0.85 | 1.54 | 1.78 | 1.86 | 1.67 |
| Protector Interdict II | `power:protector:Interdict II` | sacrifice | seat_game | 0.05-2.00 | 0.60 | in | - | 0.60 | - | 1.06 | 0.74 | 0.66 | 0.32 | 0.22 |
| Researcher Hack I | `power:researcher:Hack I` | troop-response | seat_game | 0.25-2.50 | 2.02 | in | - | - | 2.02 | 1.49 | 2.53 | 2.80 | 1.90 | 1.35 |
| Researcher Hack II | `power:researcher:Hack II` | sacrifice | seat_game | 0.05-2.00 | 0.29 | in | - | - | 0.29 | 0.53 | 0.39 | 0.28 | 0.12 | 0.10 |
