## Wins (paired against r2/off, same pair, level and seed)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | r2/off | 94.4 | 95.8 | 81.2 | 54.2 | 19.4 | 69.0 |  | 720 |
| strategist | r2/on | 88.9 | 91.0 | 79.9 | 45.8 | 19.4 | 65.0 | -4.0 ± 1.6 | 720 |
| strategist | r2/reach | 91.7 | 88.2 | 74.3 | 51.4 | 21.5 | 65.4 | -3.6 ± 1.7 | 720 |
| greedy | r2/off | 91.7 | 88.2 | 77.1 | 54.2 | 17.4 | 65.7 |  | 720 |
| greedy | r2/on | 88.9 | 81.9 | 71.5 | 38.2 | 18.8 | 59.9 | -5.8 ± 2.0 | 720 |
| greedy | r2/reach | 81.2 | 78.5 | 74.3 | 50.0 | 20.8 | 61.0 | -4.7 ± 1.8 | 720 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | r2/on | -5.6 ± 2.9 | -4.9 ± 2.8 | -1.4 ± 3.8 | -8.3 ± 4.6 | +0.0 ± 3.7 |
| strategist | r2/reach | -2.8 ± 3.1 | -7.6 ± 2.8 | -6.9 ± 4.5 | -2.8 ± 4.3 | +2.1 ± 3.5 |
| greedy | r2/on | -2.8 ± 3.4 | -6.2 ± 4.3 | -5.6 ± 4.9 | -16.0 ± 5.1 | +1.4 ± 3.8 |
| greedy | r2/reach | -10.4 ± 3.7 | -9.7 ± 4.1 | -2.8 ± 4.1 | -4.2 ± 4.6 | +3.5 ± 3.7 |

## Losses by reason a game; d3 losses (node / deck / other) of d3 games

| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |
|---|---|---|---|---|---|---|---|---|---|
| strategist | r2/off | 8.87 | 0.207 | 0.086 | 0.017 | 41 | 23 | 2 | 144 |
| strategist | r2/on | 8.99 | 0.225 | 0.111 | 0.014 | 44 | 33 | 1 | 144 |
| strategist | r2/reach | 9.09 | 0.206 | 0.128 | 0.013 | 41 | 28 | 1 | 144 |
| greedy | r2/off | 9.33 | 0.200 | 0.125 | 0.018 | 38 | 24 | 4 | 144 |
| greedy | r2/on | 9.48 | 0.229 | 0.160 | 0.013 | 45 | 42 | 2 | 144 |
| greedy | r2/reach | 9.51 | 0.212 | 0.167 | 0.011 | 40 | 32 | 0 | 144 |

## Nodes placed a seat-game by round, per board

| agent | arm | board | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r2/off | builder | 3.30 | 2.20 | 1.29 | 1.01 | 0.85 | 0.68 | 0.47 | 0.35 | 0.22 | 0.11 |
| strategist | r2/off | protector | 2.49 | 2.27 | 1.49 | 1.26 | 1.07 | 0.92 | 0.68 | 0.44 | 0.23 | 0.12 |
| strategist | r2/off | researcher | 1.62 | 2.65 | 2.25 | 2.02 | 1.70 | 0.64 | 0.26 | 0.21 | 0.09 | 0.06 |
| strategist | r2/on | builder | 3.31 | 2.20 | 1.25 | 1.02 | 0.94 | 0.64 | 0.50 | 0.34 | 0.21 | 0.14 |
| strategist | r2/on | protector | 2.69 | 2.09 | 1.47 | 1.21 | 1.08 | 0.89 | 0.76 | 0.50 | 0.28 | 0.12 |
| strategist | r2/on | researcher | 1.62 | 2.61 | 2.26 | 2.06 | 1.72 | 0.63 | 0.30 | 0.19 | 0.13 | 0.07 |
| strategist | r2/reach | builder | 3.30 | 2.17 | 1.28 | 1.05 | 0.95 | 0.69 | 0.52 | 0.33 | 0.24 | 0.13 |
| strategist | r2/reach | protector | 2.67 | 2.08 | 1.55 | 1.20 | 1.07 | 0.89 | 0.72 | 0.50 | 0.31 | 0.14 |
| strategist | r2/reach | researcher | 1.61 | 2.62 | 2.23 | 1.97 | 1.73 | 0.67 | 0.31 | 0.20 | 0.10 | 0.08 |
| greedy | r2/off | builder | 3.26 | 2.23 | 1.15 | 0.94 | 0.82 | 0.66 | 0.53 | 0.43 | 0.31 | 0.14 |
| greedy | r2/off | protector | 2.50 | 2.22 | 1.52 | 1.23 | 1.04 | 0.86 | 0.72 | 0.53 | 0.29 | 0.18 |
| greedy | r2/off | researcher | 1.50 | 2.44 | 2.28 | 2.00 | 1.70 | 0.85 | 0.34 | 0.24 | 0.15 | 0.06 |
| greedy | r2/on | builder | 3.22 | 2.21 | 1.15 | 0.96 | 0.84 | 0.62 | 0.54 | 0.42 | 0.27 | 0.16 |
| greedy | r2/on | protector | 2.81 | 2.03 | 1.42 | 1.18 | 1.02 | 0.87 | 0.69 | 0.52 | 0.33 | 0.19 |
| greedy | r2/on | researcher | 1.50 | 2.51 | 2.32 | 1.90 | 1.87 | 0.73 | 0.36 | 0.21 | 0.15 | 0.09 |
| greedy | r2/reach | builder | 3.20 | 2.22 | 1.21 | 0.93 | 0.83 | 0.70 | 0.58 | 0.44 | 0.28 | 0.18 |
| greedy | r2/reach | protector | 2.75 | 2.08 | 1.47 | 1.17 | 1.04 | 0.88 | 0.71 | 0.56 | 0.36 | 0.21 |
| greedy | r2/reach | researcher | 1.51 | 2.52 | 2.28 | 1.90 | 1.77 | 0.76 | 0.39 | 0.28 | 0.13 | 0.10 |

## Nodes by source, a seat-game (share)

| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |
|---|---|---|---|---|---|---|
| strategist | r2/off | 11.04 | 5.03 (46%) | 5.09 (46%) | 0.00 (0%) | 0.91 (8%) |
| strategist | r2/on | 11.12 | 5.07 (46%) | 5.11 (46%) | 0.00 (0%) | 0.93 (8%) |
| strategist | r2/reach | 11.17 | 5.11 (46%) | 5.15 (46%) | 0.00 (0%) | 0.92 (8%) |
| greedy | r2/off | 11.12 | 5.08 (46%) | 5.01 (45%) | 0.00 (0%) | 1.03 (9%) |
| greedy | r2/on | 11.14 | 5.10 (46%) | 5.04 (45%) | 0.00 (0%) | 1.00 (9%) |
| greedy | r2/reach | 11.25 | 5.11 (45%) | 5.16 (46%) | 0.00 (0%) | 0.99 (9%) |

## Board powers a seat-game (that board's seat-games)

| agent | arm | board | power | uses |
|---|---|---|---|---|
| strategist | r2/off | builder | Annex | 2.01 |
| strategist | r2/off | builder | Buy In | 2.35 |
| strategist | r2/off | builder | Expand | 5.25 |
| strategist | r2/off | builder | Fortify | 0.78 |
| strategist | r2/off | builder | Mint | 3.29 |
| strategist | r2/off | builder | Overcharge | 1.49 |
| strategist | r2/off | builder | Procure | 4.74 |
| strategist | r2/off | builder | Rush Build | 0.91 |
| strategist | r2/off | builder | Scrap | 1.89 |
| strategist | r2/off | builder | Teardown | 0.06 |
| strategist | r2/off | protector | Accord | 1.84 |
| strategist | r2/off | protector | Bulwark | 1.21 |
| strategist | r2/off | protector | Debrief | 2.39 |
| strategist | r2/off | protector | Expand | 4.89 |
| strategist | r2/off | protector | Interdict | 1.03 |
| strategist | r2/off | protector | Outpost | 3.43 |
| strategist | r2/off | protector | Patrol | 1.95 |
| strategist | r2/off | protector | Reclaim | 0.08 |
| strategist | r2/off | researcher | Audit | 1.89 |
| strategist | r2/off | researcher | Disrupt | 0.86 |
| strategist | r2/off | researcher | Expand | 4.96 |
| strategist | r2/off | researcher | Field Station | 0.89 |
| strategist | r2/off | researcher | Field Survey | 3.67 |
| strategist | r2/off | researcher | Grant | 2.19 |
| strategist | r2/off | researcher | Meltdown | 0.19 |
| strategist | r2/off | researcher | Peer Review | 1.99 |
| strategist | r2/off | researcher | Synthesis | 5.56 |
| strategist | r2/on | builder | Annex | 2.02 |
| strategist | r2/on | builder | Buy In | 2.33 |
| strategist | r2/on | builder | Expand | 5.27 |
| strategist | r2/on | builder | Fortify | 0.85 |
| strategist | r2/on | builder | Mint | 3.12 |
| strategist | r2/on | builder | Overcharge | 1.57 |
| strategist | r2/on | builder | Procure | 4.59 |
| strategist | r2/on | builder | Rush Build | 0.88 |
| strategist | r2/on | builder | Scrap | 1.96 |
| strategist | r2/on | builder | Teardown | 0.06 |
| strategist | r2/on | protector | Accord | 2.04 |
| strategist | r2/on | protector | Bulwark | 1.09 |
| strategist | r2/on | protector | Debrief | 2.47 |
| strategist | r2/on | protector | Expand | 4.91 |
| strategist | r2/on | protector | Interdict | 1.13 |
| strategist | r2/on | protector | Outpost | 3.26 |
| strategist | r2/on | protector | Patrol | 2.06 |
| strategist | r2/on | protector | Reclaim | 0.10 |
| strategist | r2/on | researcher | Audit | 1.79 |
| strategist | r2/on | researcher | Disrupt | 0.99 |
| strategist | r2/on | researcher | Expand | 5.04 |
| strategist | r2/on | researcher | Field Station | 0.90 |
| strategist | r2/on | researcher | Field Survey | 3.77 |
| strategist | r2/on | researcher | Grant | 2.07 |
| strategist | r2/on | researcher | Meltdown | 0.19 |
| strategist | r2/on | researcher | Peer Review | 1.93 |
| strategist | r2/on | researcher | Synthesis | 5.73 |
| strategist | r2/reach | builder | Annex | 2.03 |
| strategist | r2/reach | builder | Buy In | 2.40 |
| strategist | r2/reach | builder | Expand | 5.34 |
| strategist | r2/reach | builder | Fortify | 0.78 |
| strategist | r2/reach | builder | Mint | 2.96 |
| strategist | r2/reach | builder | Overcharge | 1.61 |
| strategist | r2/reach | builder | Procure | 4.60 |
| strategist | r2/reach | builder | Rush Build | 0.91 |
| strategist | r2/reach | builder | Scrap | 2.14 |
| strategist | r2/reach | builder | Teardown | 0.07 |
| strategist | r2/reach | protector | Accord | 1.97 |
| strategist | r2/reach | protector | Bulwark | 1.37 |
| strategist | r2/reach | protector | Debrief | 2.36 |
| strategist | r2/reach | protector | Expand | 4.96 |
| strategist | r2/reach | protector | Interdict | 1.13 |
| strategist | r2/reach | protector | Outpost | 3.47 |
| strategist | r2/reach | protector | Patrol | 2.15 |
| strategist | r2/reach | protector | Reclaim | 0.08 |
| strategist | r2/reach | researcher | Audit | 1.81 |
| strategist | r2/reach | researcher | Disrupt | 1.01 |
| strategist | r2/reach | researcher | Expand | 5.02 |
| strategist | r2/reach | researcher | Field Station | 0.87 |
| strategist | r2/reach | researcher | Field Survey | 3.73 |
| strategist | r2/reach | researcher | Grant | 2.14 |
| strategist | r2/reach | researcher | Meltdown | 0.21 |
| strategist | r2/reach | researcher | Peer Review | 1.85 |
| strategist | r2/reach | researcher | Synthesis | 5.83 |
| greedy | r2/off | builder | Annex | 1.68 |
| greedy | r2/off | builder | Buy In | 2.40 |
| greedy | r2/off | builder | Expand | 5.34 |
| greedy | r2/off | builder | Fortify | 0.55 |
| greedy | r2/off | builder | Mint | 3.12 |
| greedy | r2/off | builder | Overcharge | 1.58 |
| greedy | r2/off | builder | Procure | 5.32 |
| greedy | r2/off | builder | Rush Build | 0.97 |
| greedy | r2/off | builder | Scrap | 2.18 |
| greedy | r2/off | builder | Teardown | 0.22 |
| greedy | r2/off | protector | Accord | 2.02 |
| greedy | r2/off | protector | Bulwark | 1.34 |
| greedy | r2/off | protector | Debrief | 3.60 |
| greedy | r2/off | protector | Expand | 5.16 |
| greedy | r2/off | protector | Interdict | 1.10 |
| greedy | r2/off | protector | Outpost | 3.00 |
| greedy | r2/off | protector | Patrol | 1.82 |
| greedy | r2/off | protector | Reclaim | 0.11 |
| greedy | r2/off | researcher | Audit | 2.38 |
| greedy | r2/off | researcher | Disrupt | 0.99 |
| greedy | r2/off | researcher | Expand | 4.74 |
| greedy | r2/off | researcher | Field Station | 1.06 |
| greedy | r2/off | researcher | Field Survey | 4.50 |
| greedy | r2/off | researcher | Grant | 2.56 |
| greedy | r2/off | researcher | Meltdown | 0.23 |
| greedy | r2/off | researcher | Peer Review | 1.43 |
| greedy | r2/off | researcher | Synthesis | 5.98 |
| greedy | r2/on | builder | Annex | 1.61 |
| greedy | r2/on | builder | Buy In | 2.40 |
| greedy | r2/on | builder | Expand | 5.52 |
| greedy | r2/on | builder | Fortify | 0.67 |
| greedy | r2/on | builder | Mint | 3.14 |
| greedy | r2/on | builder | Overcharge | 1.63 |
| greedy | r2/on | builder | Procure | 5.27 |
| greedy | r2/on | builder | Rush Build | 0.98 |
| greedy | r2/on | builder | Scrap | 2.17 |
| greedy | r2/on | builder | Teardown | 0.20 |
| greedy | r2/on | protector | Accord | 2.11 |
| greedy | r2/on | protector | Bulwark | 1.32 |
| greedy | r2/on | protector | Debrief | 3.62 |
| greedy | r2/on | protector | Expand | 5.02 |
| greedy | r2/on | protector | Interdict | 1.15 |
| greedy | r2/on | protector | Outpost | 2.99 |
| greedy | r2/on | protector | Patrol | 2.03 |
| greedy | r2/on | protector | Reclaim | 0.15 |
| greedy | r2/on | researcher | Audit | 2.32 |
| greedy | r2/on | researcher | Disrupt | 1.23 |
| greedy | r2/on | researcher | Expand | 4.76 |
| greedy | r2/on | researcher | Field Station | 1.12 |
| greedy | r2/on | researcher | Field Survey | 4.63 |
| greedy | r2/on | researcher | Grant | 2.46 |
| greedy | r2/on | researcher | Meltdown | 0.20 |
| greedy | r2/on | researcher | Peer Review | 1.37 |
| greedy | r2/on | researcher | Synthesis | 6.16 |
| greedy | r2/reach | builder | Annex | 1.70 |
| greedy | r2/reach | builder | Buy In | 2.46 |
| greedy | r2/reach | builder | Expand | 5.40 |
| greedy | r2/reach | builder | Fortify | 0.56 |
| greedy | r2/reach | builder | Mint | 3.08 |
| greedy | r2/reach | builder | Overcharge | 1.67 |
| greedy | r2/reach | builder | Procure | 5.37 |
| greedy | r2/reach | builder | Rush Build | 1.01 |
| greedy | r2/reach | builder | Scrap | 2.30 |
| greedy | r2/reach | builder | Teardown | 0.18 |
| greedy | r2/reach | protector | Accord | 2.10 |
| greedy | r2/reach | protector | Bulwark | 1.52 |
| greedy | r2/reach | protector | Debrief | 3.69 |
| greedy | r2/reach | protector | Expand | 5.15 |
| greedy | r2/reach | protector | Interdict | 1.12 |
| greedy | r2/reach | protector | Outpost | 3.17 |
| greedy | r2/reach | protector | Patrol | 2.04 |
| greedy | r2/reach | protector | Reclaim | 0.13 |
| greedy | r2/reach | researcher | Audit | 2.42 |
| greedy | r2/reach | researcher | Disrupt | 1.21 |
| greedy | r2/reach | researcher | Expand | 4.76 |
| greedy | r2/reach | researcher | Field Station | 1.14 |
| greedy | r2/reach | researcher | Field Survey | 4.60 |
| greedy | r2/reach | researcher | Grant | 2.42 |
| greedy | r2/reach | researcher | Meltdown | 0.21 |
| greedy | r2/reach | researcher | Peer Review | 1.44 |
| greedy | r2/reach | researcher | Synthesis | 6.18 |

## Standard actions a seat-game

| agent | arm | ambassador | antimatter | botnet | buy_energy | claim | envoy | insight | liberate | lift | move | remove | station | troop | beyond Dispatch and Liberate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r2/off | 0.96 | 0.03 | 0.81 | 3.19 | 0.03 | 0.06 | 0.47 | 2.26 | 0.10 | 9.28 | 0.54 | 0.22 | 0.57 | **6.98** |
| strategist | r2/on | 0.77 | 0.03 | 0.82 | 3.85 | 0.02 | 0.07 | 0.41 | 2.20 | 0.10 | 9.13 | 0.46 | 0.20 | 0.57 | **7.29** |
| strategist | r2/reach | 0.85 | 0.03 | 0.83 | 4.01 | 0.04 | 0.05 | 0.50 | 2.26 | 0.13 | 9.32 | 0.52 | 0.21 | 0.65 | **7.84** |
| greedy | r2/off | 0.53 | 0.07 | 0.86 | 2.60 | 0.05 | 0.06 | 0.44 | 2.22 | 0.10 | 10.80 | 0.52 | 0.17 | 0.55 | **5.95** |
| greedy | r2/on | 0.38 | 0.07 | 0.84 | 3.32 | 0.04 | 0.06 | 0.42 | 2.08 | 0.09 | 10.67 | 0.49 | 0.17 | 0.53 | **6.39** |
| greedy | r2/reach | 0.49 | 0.08 | 0.84 | 3.38 | 0.05 | 0.05 | 0.42 | 2.23 | 0.11 | 10.73 | 0.46 | 0.19 | 0.56 | **6.61** |

## Liberations, claims, retakes and neutral sites, a game

| agent | arm | adv.retakes | city_energy_aligned | city_energy_bought | city_energy_hosted | claimed.factory | claimed.station | claimed_liberated | factories_built.player | liberated.city | liberated.factory | liberated.station | liberated_ambassadors | liberated_botnet | liberated_card | liberated_troops | node_lost | stations_built_free |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r2/off | 0.86 | 11.12 | 6.39 | 0.00 | 0.05 | 0.45 | 0.04 | 0.07 | 3.38 | 1.16 | 0.82 | 0.76 | 0.07 | 0.84 | 0.09 | 10.75 | 1.26 |
| strategist | r2/on | 0.88 | 0.00 | 7.70 | 9.38 | 0.05 | 0.49 | 0.05 | 0.07 | 3.30 | 1.20 | 0.71 | 0.78 | 0.06 | 0.82 | 0.11 | 11.25 | 1.26 |
| strategist | r2/reach | 0.96 | 0.00 | 8.03 | 11.73 | 0.07 | 0.52 | 0.06 | 0.07 | 3.45 | 1.30 | 0.68 | 0.81 | 0.07 | 0.91 | 0.11 | 11.19 | 1.24 |
| greedy | r2/off | 0.82 | 11.59 | 5.20 | 0.00 | 0.09 | 0.55 | 0.07 | 0.05 | 3.21 | 1.16 | 0.82 | 0.56 | 0.07 | 0.74 | 0.08 | 11.12 | 1.37 |
| greedy | r2/on | 0.85 | 0.00 | 6.63 | 10.29 | 0.07 | 0.56 | 0.04 | 0.06 | 3.27 | 1.08 | 0.62 | 0.59 | 0.07 | 0.82 | 0.12 | 11.53 | 1.41 |
| greedy | r2/reach | 0.88 | 0.00 | 6.76 | 12.66 | 0.10 | 0.57 | 0.07 | 0.07 | 3.41 | 1.24 | 0.66 | 0.59 | 0.09 | 0.84 | 0.10 | 11.65 | 1.42 |

## The new cards played a game; wins through cards

| agent | arm | hostile_buyout | open_research_campus | industrial_commons | won games with more card plays and uses than standard actions | card share of actions in won games |
|---|---|---|---|---|---|---|
| strategist | r2/off | 0.254 | 0.126 | 0.204 | 100% | 85% |
| strategist | r2/on | 0.258 | 0.133 | 0.218 | 100% | 85% |
| strategist | r2/reach | 0.269 | 0.115 | 0.206 | 100% | 85% |
| greedy | r2/off | 0.186 | 0.139 | 0.235 | 100% | 87% |
| greedy | r2/on | 0.196 | 0.163 | 0.261 | 100% | 87% |
| greedy | r2/reach | 0.197 | 0.165 | 0.222 | 100% | 87% |

## Resource use: spent a seat-game (spent / gained)

| agent | arm | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| strategist | r2/off | 333.9 (104%) | 48.6 (83%) | 33.0 (88%) | 4.5 (66%) | 1.3 (44%) | 3.3 (56%) | 17.8 (77%) |
| strategist | r2/on | 337.5 (104%) | 49.7 (84%) | 33.5 (88%) | 4.6 (66%) | 1.3 (45%) | 3.2 (55%) | 17.2 (80%) |
| strategist | r2/reach | 348.6 (104%) | 50.1 (84%) | 34.6 (86%) | 4.6 (63%) | 1.4 (44%) | 3.6 (57%) | 17.8 (80%) |
| greedy | r2/off | 349.8 (104%) | 52.7 (83%) | 32.1 (89%) | 5.1 (63%) | 1.2 (54%) | 3.7 (59%) | 18.9 (81%) |
| greedy | r2/on | 352.4 (103%) | 53.4 (83%) | 32.7 (90%) | 5.2 (64%) | 1.3 (54%) | 3.7 (59%) | 18.1 (82%) |
| greedy | r2/reach | 358.4 (103%) | 53.9 (83%) | 33.9 (89%) | 5.3 (61%) | 1.3 (55%) | 3.8 (61%) | 18.8 (84%) |
