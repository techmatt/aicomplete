## Wins (paired against confirm2/off, same pair, level and seed)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm2/off | 97.9 | 93.1 | 84.7 | 47.9 | 22.2 | 69.2 |  | 720 |
| strategist | confirm2/on | 94.4 | 93.8 | 83.3 | 54.9 | 20.1 | 69.3 | +0.1 ± 1.7 | 720 |
| greedy | confirm2/off | 90.3 | 86.8 | 77.1 | 41.0 | 13.2 | 61.7 |  | 720 |
| greedy | confirm2/on | 89.6 | 89.6 | 70.8 | 50.7 | 16.7 | 63.5 | +1.8 ± 1.9 | 720 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | confirm2/on | -3.5 ± 2.1 | +0.7 ± 2.9 | -1.4 ± 3.7 | +6.9 ± 5.1 | -2.1 ± 4.0 |
| greedy | confirm2/on | -0.7 ± 3.5 | +2.8 ± 3.7 | -6.2 ± 4.8 | +9.7 ± 5.1 | +3.5 ± 3.3 |

## Losses by reason a game; d3 losses (node / deck / other) of d3 games

| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |
|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm2/off | 8.92 | 0.211 | 0.083 | 0.014 | 51 | 21 | 3 | 144 |
| strategist | confirm2/on | 8.90 | 0.196 | 0.097 | 0.014 | 32 | 32 | 1 | 144 |
| greedy | confirm2/off | 9.51 | 0.208 | 0.149 | 0.026 | 47 | 33 | 5 | 144 |
| greedy | confirm2/on | 9.39 | 0.214 | 0.138 | 0.014 | 39 | 29 | 3 | 144 |

## Nodes placed a seat-game by round, per board

| agent | arm | board | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm2/off | builder | 3.10 | 2.37 | 1.24 | 1.09 | 0.84 | 0.61 | 0.48 | 0.34 | 0.20 | 0.10 |
| strategist | confirm2/off | protector | 2.33 | 2.20 | 1.55 | 1.23 | 1.02 | 0.86 | 0.67 | 0.47 | 0.35 | 0.15 |
| strategist | confirm2/off | researcher | 1.82 | 2.49 | 2.19 | 2.03 | 1.67 | 0.59 | 0.27 | 0.16 | 0.09 | 0.06 |
| strategist | confirm2/on | builder | 3.11 | 2.44 | 1.31 | 1.11 | 0.95 | 0.67 | 0.45 | 0.35 | 0.19 | 0.11 |
| strategist | confirm2/on | protector | 2.62 | 2.03 | 1.47 | 1.23 | 1.08 | 0.90 | 0.68 | 0.49 | 0.26 | 0.13 |
| strategist | confirm2/on | researcher | 1.82 | 2.47 | 2.24 | 2.03 | 1.55 | 0.59 | 0.28 | 0.19 | 0.14 | 0.07 |
| greedy | confirm2/off | builder | 3.20 | 2.13 | 1.16 | 0.96 | 0.80 | 0.61 | 0.57 | 0.45 | 0.30 | 0.16 |
| greedy | confirm2/off | protector | 2.41 | 2.22 | 1.57 | 1.21 | 1.01 | 0.85 | 0.71 | 0.52 | 0.30 | 0.17 |
| greedy | confirm2/off | researcher | 1.78 | 2.51 | 2.17 | 1.96 | 1.73 | 0.75 | 0.32 | 0.23 | 0.13 | 0.06 |
| greedy | confirm2/on | builder | 3.19 | 2.12 | 1.18 | 1.01 | 0.81 | 0.69 | 0.56 | 0.42 | 0.29 | 0.12 |
| greedy | confirm2/on | protector | 2.61 | 2.13 | 1.45 | 1.19 | 1.01 | 0.92 | 0.72 | 0.53 | 0.32 | 0.19 |
| greedy | confirm2/on | researcher | 1.78 | 2.55 | 2.18 | 1.97 | 1.76 | 0.74 | 0.30 | 0.25 | 0.12 | 0.07 |

## Nodes by source, a seat-game (share)

| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |
|---|---|---|---|---|---|---|
| strategist | confirm2/off | 10.91 | 5.01 (46%) | 5.04 (46%) | 0.00 (0%) | 0.86 (8%) |
| strategist | confirm2/on | 11.03 | 5.03 (46%) | 5.18 (47%) | 0.00 (0%) | 0.82 (7%) |
| greedy | confirm2/off | 11.13 | 5.06 (45%) | 5.05 (45%) | 0.00 (0%) | 1.01 (9%) |
| greedy | confirm2/on | 11.18 | 5.03 (45%) | 5.13 (46%) | 0.00 (0%) | 1.02 (9%) |

## Board powers a seat-game (that board's seat-games)

| agent | arm | board | power | uses |
|---|---|---|---|---|
| strategist | confirm2/off | builder | Annex | 2.08 |
| strategist | confirm2/off | builder | Buy In | 2.39 |
| strategist | confirm2/off | builder | Expand | 5.19 |
| strategist | confirm2/off | builder | Fortify | 0.69 |
| strategist | confirm2/off | builder | Mint | 2.92 |
| strategist | confirm2/off | builder | Overcharge | 1.37 |
| strategist | confirm2/off | builder | Procure | 4.76 |
| strategist | confirm2/off | builder | Rush Build | 0.83 |
| strategist | confirm2/off | builder | Scrap | 1.80 |
| strategist | confirm2/off | builder | Teardown | 0.05 |
| strategist | confirm2/off | protector | Accord | 1.76 |
| strategist | confirm2/off | protector | Bulwark | 1.28 |
| strategist | confirm2/off | protector | Debrief | 2.35 |
| strategist | confirm2/off | protector | Expand | 4.93 |
| strategist | confirm2/off | protector | Interdict | 1.00 |
| strategist | confirm2/off | protector | Outpost | 3.41 |
| strategist | confirm2/off | protector | Patrol | 2.06 |
| strategist | confirm2/off | protector | Reclaim | 0.09 |
| strategist | confirm2/off | researcher | Audit | 2.00 |
| strategist | confirm2/off | researcher | Disrupt | 0.81 |
| strategist | confirm2/off | researcher | Expand | 4.90 |
| strategist | confirm2/off | researcher | Field Station | 0.94 |
| strategist | confirm2/off | researcher | Field Survey | 3.60 |
| strategist | confirm2/off | researcher | Grant | 2.30 |
| strategist | confirm2/off | researcher | Meltdown | 0.20 |
| strategist | confirm2/off | researcher | Peer Review | 1.89 |
| strategist | confirm2/off | researcher | Synthesis | 5.85 |
| strategist | confirm2/on | builder | Annex | 2.15 |
| strategist | confirm2/on | builder | Buy In | 2.48 |
| strategist | confirm2/on | builder | Expand | 5.30 |
| strategist | confirm2/on | builder | Fortify | 0.65 |
| strategist | confirm2/on | builder | Mint | 2.84 |
| strategist | confirm2/on | builder | Overcharge | 1.43 |
| strategist | confirm2/on | builder | Procure | 4.75 |
| strategist | confirm2/on | builder | Rush Build | 0.92 |
| strategist | confirm2/on | builder | Scrap | 1.78 |
| strategist | confirm2/on | builder | Teardown | 0.05 |
| strategist | confirm2/on | protector | Accord | 1.89 |
| strategist | confirm2/on | protector | Bulwark | 1.33 |
| strategist | confirm2/on | protector | Debrief | 2.49 |
| strategist | confirm2/on | protector | Expand | 4.90 |
| strategist | confirm2/on | protector | Interdict | 1.05 |
| strategist | confirm2/on | protector | Outpost | 3.51 |
| strategist | confirm2/on | protector | Patrol | 2.04 |
| strategist | confirm2/on | protector | Reclaim | 0.09 |
| strategist | confirm2/on | researcher | Audit | 1.96 |
| strategist | confirm2/on | researcher | Disrupt | 0.86 |
| strategist | confirm2/on | researcher | Expand | 4.88 |
| strategist | confirm2/on | researcher | Field Station | 0.92 |
| strategist | confirm2/on | researcher | Field Survey | 3.67 |
| strategist | confirm2/on | researcher | Grant | 2.02 |
| strategist | confirm2/on | researcher | Meltdown | 0.19 |
| strategist | confirm2/on | researcher | Peer Review | 1.83 |
| strategist | confirm2/on | researcher | Synthesis | 5.65 |
| greedy | confirm2/off | builder | Annex | 1.76 |
| greedy | confirm2/off | builder | Buy In | 2.37 |
| greedy | confirm2/off | builder | Expand | 5.37 |
| greedy | confirm2/off | builder | Fortify | 0.59 |
| greedy | confirm2/off | builder | Mint | 3.21 |
| greedy | confirm2/off | builder | Overcharge | 1.51 |
| greedy | confirm2/off | builder | Procure | 5.75 |
| greedy | confirm2/off | builder | Rush Build | 0.94 |
| greedy | confirm2/off | builder | Scrap | 2.11 |
| greedy | confirm2/off | builder | Teardown | 0.17 |
| greedy | confirm2/off | protector | Accord | 2.00 |
| greedy | confirm2/off | protector | Bulwark | 1.36 |
| greedy | confirm2/off | protector | Debrief | 3.61 |
| greedy | confirm2/off | protector | Expand | 5.23 |
| greedy | confirm2/off | protector | Interdict | 1.04 |
| greedy | confirm2/off | protector | Outpost | 3.05 |
| greedy | confirm2/off | protector | Patrol | 1.94 |
| greedy | confirm2/off | protector | Reclaim | 0.11 |
| greedy | confirm2/off | researcher | Audit | 2.58 |
| greedy | confirm2/off | researcher | Disrupt | 1.11 |
| greedy | confirm2/off | researcher | Expand | 4.59 |
| greedy | confirm2/off | researcher | Field Station | 1.15 |
| greedy | confirm2/off | researcher | Field Survey | 4.58 |
| greedy | confirm2/off | researcher | Grant | 2.62 |
| greedy | confirm2/off | researcher | Meltdown | 0.28 |
| greedy | confirm2/off | researcher | Peer Review | 1.40 |
| greedy | confirm2/off | researcher | Synthesis | 6.31 |
| greedy | confirm2/on | builder | Annex | 1.68 |
| greedy | confirm2/on | builder | Buy In | 2.36 |
| greedy | confirm2/on | builder | Expand | 5.49 |
| greedy | confirm2/on | builder | Fortify | 0.52 |
| greedy | confirm2/on | builder | Mint | 2.80 |
| greedy | confirm2/on | builder | Overcharge | 1.47 |
| greedy | confirm2/on | builder | Procure | 5.46 |
| greedy | confirm2/on | builder | Rush Build | 0.94 |
| greedy | confirm2/on | builder | Scrap | 1.96 |
| greedy | confirm2/on | builder | Teardown | 0.18 |
| greedy | confirm2/on | protector | Accord | 2.08 |
| greedy | confirm2/on | protector | Bulwark | 1.43 |
| greedy | confirm2/on | protector | Debrief | 3.57 |
| greedy | confirm2/on | protector | Expand | 5.03 |
| greedy | confirm2/on | protector | Interdict | 0.96 |
| greedy | confirm2/on | protector | Outpost | 3.18 |
| greedy | confirm2/on | protector | Patrol | 2.09 |
| greedy | confirm2/on | protector | Reclaim | 0.10 |
| greedy | confirm2/on | researcher | Audit | 2.49 |
| greedy | confirm2/on | researcher | Disrupt | 1.04 |
| greedy | confirm2/on | researcher | Expand | 4.57 |
| greedy | confirm2/on | researcher | Field Station | 1.12 |
| greedy | confirm2/on | researcher | Field Survey | 4.62 |
| greedy | confirm2/on | researcher | Grant | 2.48 |
| greedy | confirm2/on | researcher | Meltdown | 0.23 |
| greedy | confirm2/on | researcher | Peer Review | 1.47 |
| greedy | confirm2/on | researcher | Synthesis | 6.13 |

## Standard actions a seat-game

| agent | arm | ambassador | antimatter | botnet | buy_energy | claim | envoy | insight | liberate | lift | move | remove | station | troop | beyond Dispatch and Liberate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm2/off | 0.76 | 0.03 | 0.85 | 3.15 | 0.04 | 0.04 | 0.41 | 2.25 | 0.10 | 9.19 | 0.54 | 0.22 | 0.60 | **6.72** |
| strategist | confirm2/on | 0.75 | 0.03 | 0.83 | 4.15 | 0.03 | 0.05 | 0.42 | 2.32 | 0.10 | 9.09 | 0.51 | 0.22 | 0.57 | **7.65** |
| greedy | confirm2/off | 0.59 | 0.07 | 0.80 | 2.65 | 0.03 | 0.06 | 0.44 | 2.28 | 0.09 | 10.86 | 0.50 | 0.18 | 0.56 | **5.99** |
| greedy | confirm2/on | 0.48 | 0.05 | 0.75 | 3.38 | 0.04 | 0.04 | 0.43 | 2.24 | 0.09 | 10.50 | 0.47 | 0.18 | 0.53 | **6.47** |

## Liberations, claims, retakes and neutral sites, a game

| agent | arm | adv.retakes | city_energy_aligned | city_energy_bought | city_energy_hosted | claimed.factory | claimed.station | claimed_liberated | factories_built.player | liberated.city | liberated.factory | liberated.station | liberated_ambassadors | liberated_botnet | liberated_card | liberated_troops | node_lost | stations_built_free | effects.per_site.owned | effects.per_site.owned_hosted |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm2/off | 0.92 | 11.32 | 6.29 | 0.00 | 0.08 | 0.47 | 0.06 | 0.08 | 3.47 | 1.12 | 0.75 | 0.50 | 0.06 | 0.84 | 0.07 | 10.91 | 1.29 | 4.32 | 0.00 |
| strategist | confirm2/on | 0.92 | 0.00 | 8.29 | 12.11 | 0.06 | 0.47 | 0.06 | 0.09 | 3.47 | 1.19 | 0.82 | 0.52 | 0.07 | 0.84 | 0.07 | 11.11 | 1.27 | 4.74 | 3.67 |
| greedy | confirm2/off | 0.88 | 11.79 | 5.30 | 0.00 | 0.07 | 0.58 | 0.08 | 0.08 | 3.34 | 1.19 | 0.78 | 0.48 | 0.05 | 0.74 | 0.07 | 11.65 | 1.43 | 4.60 | 0.00 |
| greedy | confirm2/on | 0.83 | 0.00 | 6.76 | 13.25 | 0.08 | 0.55 | 0.08 | 0.08 | 3.28 | 1.15 | 0.73 | 0.49 | 0.05 | 0.68 | 0.07 | 11.50 | 1.41 | 5.09 | 3.81 |

## The new cards played a game; wins through cards

| agent | arm | hostile_buyout | open_research_campus | industrial_commons | won games with more card plays and uses than standard actions | card share of actions in won games |
|---|---|---|---|---|---|---|
| strategist | confirm2/off | 0.167 | 0.119 | 0.143 | 100% | 85% |
| strategist | confirm2/on | 0.172 | 0.113 | 0.146 | 100% | 83% |
| greedy | confirm2/off | 0.161 | 0.176 | 0.203 | 100% | 88% |
| greedy | confirm2/on | 0.163 | 0.154 | 0.188 | 100% | 86% |

## Resource use: spent a seat-game (spent / gained)

| agent | arm | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| strategist | confirm2/off | 331.7 (104%) | 48.8 (82%) | 32.6 (90%) | 4.6 (67%) | 1.2 (44%) | 3.2 (55%) | 17.8 (81%) |
| strategist | confirm2/on | 333.9 (104%) | 48.9 (83%) | 33.7 (90%) | 4.7 (66%) | 1.3 (45%) | 3.3 (58%) | 17.9 (80%) |
| greedy | confirm2/off | 357.9 (103%) | 53.7 (82%) | 32.6 (91%) | 5.4 (63%) | 1.3 (55%) | 4.0 (62%) | 19.1 (82%) |
| greedy | confirm2/on | 350.2 (104%) | 52.9 (83%) | 33.3 (90%) | 5.2 (64%) | 1.2 (52%) | 3.7 (60%) | 18.7 (80%) |
