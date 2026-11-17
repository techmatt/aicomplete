## Wins (paired against fresh/off, same pair, level and seed)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | fresh/off | 96.5 | 93.8 | 86.1 | 59.0 | 18.1 | 70.7 |  | 720 |
| strategist | fresh/on | 95.8 | 92.4 | 81.2 | 51.4 | 19.4 | 68.1 | -2.6 ± 1.5 | 720 |
| greedy | fresh/off | 89.6 | 84.0 | 69.4 | 50.7 | 18.1 | 62.4 |  | 720 |
| greedy | fresh/on | 89.6 | 79.2 | 75.7 | 44.4 | 13.9 | 60.6 | -1.8 ± 1.9 | 720 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | fresh/on | -0.7 ± 2.1 | -1.4 ± 3.0 | -4.9 ± 4.0 | -7.6 ± 4.3 | +1.4 ± 3.4 |
| greedy | fresh/on | +0.0 ± 3.7 | -4.9 ± 4.3 | +6.2 ± 4.4 | -6.2 ± 4.9 | -4.2 ± 3.8 |

## Losses by reason a game; d3 losses (node / deck / other) of d3 games

| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |
|---|---|---|---|---|---|---|---|---|---|
| strategist | fresh/off | 8.76 | 0.179 | 0.099 | 0.015 | 29 | 30 | 0 | 144 |
| strategist | fresh/on | 9.01 | 0.207 | 0.100 | 0.013 | 34 | 34 | 2 | 144 |
| greedy | fresh/off | 9.36 | 0.203 | 0.158 | 0.015 | 37 | 33 | 1 | 144 |
| greedy | fresh/on | 9.48 | 0.217 | 0.167 | 0.011 | 36 | 42 | 2 | 144 |

## Nodes placed a seat-game by round, per board

| agent | arm | board | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | fresh/off | builder | 3.18 | 2.38 | 1.25 | 1.15 | 0.88 | 0.68 | 0.51 | 0.31 | 0.18 | 0.08 |
| strategist | fresh/off | protector | 2.34 | 2.23 | 1.64 | 1.23 | 1.05 | 0.86 | 0.66 | 0.46 | 0.26 | 0.13 |
| strategist | fresh/off | researcher | 1.42 | 2.59 | 2.28 | 2.04 | 1.63 | 0.67 | 0.29 | 0.17 | 0.10 | 0.04 |
| strategist | fresh/on | builder | 3.20 | 2.39 | 1.30 | 1.09 | 0.91 | 0.65 | 0.50 | 0.41 | 0.20 | 0.09 |
| strategist | fresh/on | protector | 2.36 | 2.18 | 1.61 | 1.24 | 1.07 | 0.88 | 0.71 | 0.53 | 0.29 | 0.16 |
| strategist | fresh/on | researcher | 1.41 | 2.62 | 2.36 | 2.14 | 1.59 | 0.62 | 0.32 | 0.18 | 0.11 | 0.04 |
| greedy | fresh/off | builder | 3.12 | 2.07 | 1.19 | 1.05 | 0.86 | 0.65 | 0.51 | 0.38 | 0.25 | 0.19 |
| greedy | fresh/off | protector | 2.33 | 2.24 | 1.66 | 1.20 | 1.02 | 0.85 | 0.66 | 0.49 | 0.33 | 0.20 |
| greedy | fresh/off | researcher | 1.48 | 2.60 | 2.27 | 2.05 | 1.88 | 0.71 | 0.31 | 0.21 | 0.14 | 0.07 |
| greedy | fresh/on | builder | 3.11 | 2.14 | 1.21 | 1.04 | 0.89 | 0.63 | 0.52 | 0.43 | 0.28 | 0.19 |
| greedy | fresh/on | protector | 2.35 | 2.22 | 1.58 | 1.19 | 0.96 | 0.85 | 0.68 | 0.58 | 0.36 | 0.18 |
| greedy | fresh/on | researcher | 1.50 | 2.61 | 2.30 | 2.05 | 1.86 | 0.76 | 0.33 | 0.24 | 0.12 | 0.06 |

## Nodes by source, a seat-game (share)

| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |
|---|---|---|---|---|---|---|
| strategist | fresh/off | 10.94 | 5.04 (46%) | 5.19 (47%) | 0.00 (0%) | 0.71 (6%) |
| strategist | fresh/on | 11.10 | 5.09 (46%) | 5.29 (48%) | 0.00 (0%) | 0.73 (7%) |
| greedy | fresh/off | 11.10 | 5.09 (46%) | 5.12 (46%) | 0.00 (0%) | 0.89 (8%) |
| greedy | fresh/on | 11.17 | 5.13 (46%) | 5.16 (46%) | 0.00 (0%) | 0.87 (8%) |

## Board powers a seat-game (that board's seat-games)

| agent | arm | board | power | uses |
|---|---|---|---|---|
| strategist | fresh/off | builder | Annex | 2.13 |
| strategist | fresh/off | builder | Buy In | 2.43 |
| strategist | fresh/off | builder | Expand | 5.22 |
| strategist | fresh/off | builder | Fortify | 0.64 |
| strategist | fresh/off | builder | Mint | 3.40 |
| strategist | fresh/off | builder | Overcharge | 1.54 |
| strategist | fresh/off | builder | Procure | 4.80 |
| strategist | fresh/off | builder | Rush Build | 0.93 |
| strategist | fresh/off | builder | Scrap | 1.95 |
| strategist | fresh/off | builder | Teardown | 0.04 |
| strategist | fresh/off | protector | Accord | 1.71 |
| strategist | fresh/off | protector | Bulwark | 1.34 |
| strategist | fresh/off | protector | Debrief | 2.35 |
| strategist | fresh/off | protector | Expand | 4.99 |
| strategist | fresh/off | protector | Interdict | 0.91 |
| strategist | fresh/off | protector | Outpost | 3.62 |
| strategist | fresh/off | protector | Patrol | 1.97 |
| strategist | fresh/off | protector | Reclaim | 0.09 |
| strategist | fresh/off | researcher | Audit | 1.63 |
| strategist | fresh/off | researcher | Disrupt | 0.72 |
| strategist | fresh/off | researcher | Expand | 4.91 |
| strategist | fresh/off | researcher | Field Station | 0.88 |
| strategist | fresh/off | researcher | Field Survey | 3.72 |
| strategist | fresh/off | researcher | Grant | 2.09 |
| strategist | fresh/off | researcher | Meltdown | 0.19 |
| strategist | fresh/off | researcher | Peer Review | 1.96 |
| strategist | fresh/off | researcher | Synthesis | 5.50 |
| strategist | fresh/on | builder | Annex | 2.12 |
| strategist | fresh/on | builder | Buy In | 2.48 |
| strategist | fresh/on | builder | Expand | 5.39 |
| strategist | fresh/on | builder | Fortify | 0.76 |
| strategist | fresh/on | builder | Mint | 3.52 |
| strategist | fresh/on | builder | Overcharge | 1.53 |
| strategist | fresh/on | builder | Procure | 5.05 |
| strategist | fresh/on | builder | Rush Build | 0.93 |
| strategist | fresh/on | builder | Scrap | 1.93 |
| strategist | fresh/on | builder | Teardown | 0.04 |
| strategist | fresh/on | protector | Accord | 1.87 |
| strategist | fresh/on | protector | Bulwark | 1.41 |
| strategist | fresh/on | protector | Debrief | 2.38 |
| strategist | fresh/on | protector | Expand | 4.92 |
| strategist | fresh/on | protector | Interdict | 1.11 |
| strategist | fresh/on | protector | Outpost | 3.65 |
| strategist | fresh/on | protector | Patrol | 2.01 |
| strategist | fresh/on | protector | Reclaim | 0.09 |
| strategist | fresh/on | researcher | Audit | 1.81 |
| strategist | fresh/on | researcher | Disrupt | 0.84 |
| strategist | fresh/on | researcher | Expand | 4.95 |
| strategist | fresh/on | researcher | Field Station | 0.89 |
| strategist | fresh/on | researcher | Field Survey | 3.78 |
| strategist | fresh/on | researcher | Grant | 2.07 |
| strategist | fresh/on | researcher | Meltdown | 0.16 |
| strategist | fresh/on | researcher | Peer Review | 1.96 |
| strategist | fresh/on | researcher | Synthesis | 5.65 |
| greedy | fresh/off | builder | Annex | 1.66 |
| greedy | fresh/off | builder | Buy In | 2.36 |
| greedy | fresh/off | builder | Expand | 5.42 |
| greedy | fresh/off | builder | Fortify | 0.58 |
| greedy | fresh/off | builder | Mint | 3.07 |
| greedy | fresh/off | builder | Overcharge | 1.48 |
| greedy | fresh/off | builder | Procure | 5.58 |
| greedy | fresh/off | builder | Rush Build | 0.96 |
| greedy | fresh/off | builder | Scrap | 2.20 |
| greedy | fresh/off | builder | Teardown | 0.15 |
| greedy | fresh/off | protector | Accord | 1.97 |
| greedy | fresh/off | protector | Bulwark | 1.34 |
| greedy | fresh/off | protector | Debrief | 3.50 |
| greedy | fresh/off | protector | Expand | 5.11 |
| greedy | fresh/off | protector | Interdict | 0.95 |
| greedy | fresh/off | protector | Outpost | 3.22 |
| greedy | fresh/off | protector | Patrol | 1.84 |
| greedy | fresh/off | protector | Reclaim | 0.10 |
| greedy | fresh/off | researcher | Audit | 2.18 |
| greedy | fresh/off | researcher | Disrupt | 1.07 |
| greedy | fresh/off | researcher | Expand | 4.74 |
| greedy | fresh/off | researcher | Field Station | 1.10 |
| greedy | fresh/off | researcher | Field Survey | 4.52 |
| greedy | fresh/off | researcher | Grant | 2.55 |
| greedy | fresh/off | researcher | Meltdown | 0.21 |
| greedy | fresh/off | researcher | Peer Review | 1.62 |
| greedy | fresh/off | researcher | Synthesis | 6.25 |
| greedy | fresh/on | builder | Annex | 1.64 |
| greedy | fresh/on | builder | Buy In | 2.46 |
| greedy | fresh/on | builder | Expand | 5.58 |
| greedy | fresh/on | builder | Fortify | 0.58 |
| greedy | fresh/on | builder | Mint | 3.37 |
| greedy | fresh/on | builder | Overcharge | 1.49 |
| greedy | fresh/on | builder | Procure | 5.73 |
| greedy | fresh/on | builder | Rush Build | 1.03 |
| greedy | fresh/on | builder | Scrap | 2.06 |
| greedy | fresh/on | builder | Teardown | 0.17 |
| greedy | fresh/on | protector | Accord | 2.08 |
| greedy | fresh/on | protector | Bulwark | 1.34 |
| greedy | fresh/on | protector | Debrief | 3.57 |
| greedy | fresh/on | protector | Expand | 5.04 |
| greedy | fresh/on | protector | Interdict | 1.15 |
| greedy | fresh/on | protector | Outpost | 3.16 |
| greedy | fresh/on | protector | Patrol | 1.86 |
| greedy | fresh/on | protector | Reclaim | 0.09 |
| greedy | fresh/on | researcher | Audit | 2.19 |
| greedy | fresh/on | researcher | Disrupt | 1.16 |
| greedy | fresh/on | researcher | Expand | 4.79 |
| greedy | fresh/on | researcher | Field Station | 1.13 |
| greedy | fresh/on | researcher | Field Survey | 4.53 |
| greedy | fresh/on | researcher | Grant | 2.46 |
| greedy | fresh/on | researcher | Meltdown | 0.25 |
| greedy | fresh/on | researcher | Peer Review | 1.63 |
| greedy | fresh/on | researcher | Synthesis | 6.44 |

## Standard actions a seat-game

| agent | arm | ambassador | antimatter | botnet | buy_energy | claim | envoy | insight | liberate | lift | move | remove | station | troop | beyond Dispatch and Liberate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | fresh/off | 0.66 | 0.03 | 0.79 | 3.16 | 0.03 | 0.06 | 0.49 | 2.35 | 0.10 | 8.91 | 0.47 | 0.24 | 0.57 | **6.59** |
| strategist | fresh/on | 0.65 | 0.03 | 0.82 | 3.10 | 0.02 | 0.03 | 0.53 | 2.25 | 0.12 | 9.14 | 0.41 | 0.22 | 0.58 | **6.50** |
| greedy | fresh/off | 0.48 | 0.06 | 0.77 | 2.59 | 0.04 | 0.05 | 0.49 | 2.31 | 0.12 | 10.91 | 0.41 | 0.18 | 0.53 | **5.71** |
| greedy | fresh/on | 0.47 | 0.05 | 0.83 | 2.50 | 0.03 | 0.04 | 0.49 | 2.19 | 0.12 | 10.89 | 0.43 | 0.17 | 0.52 | **5.66** |

## Liberations, claims, retakes and neutral sites, a game

| agent | arm | adv.retakes | city_energy_aligned | city_energy_bought | city_energy_hosted | claimed.factory | claimed.station | claimed_liberated | factories_built.player | liberated.city | liberated.factory | liberated.station | liberated_ambassadors | liberated_botnet | liberated_card | liberated_troops | node_lost | stations_built_free |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | fresh/off | 0.95 | 10.76 | 6.32 | 0.00 | 0.07 | 0.50 | 0.07 | 0.11 | 3.36 | 1.18 | 0.89 | 0.62 | 0.06 | 0.74 | 0.08 | 10.34 | 1.25 |
| strategist | fresh/on | 0.88 | 0.00 | 6.19 | 12.37 | 0.05 | 0.52 | 0.04 | 0.12 | 3.37 | 1.19 | 0.71 | 0.60 | 0.06 | 0.76 | 0.08 | 11.08 | 1.26 |
| greedy | fresh/off | 0.94 | 11.36 | 5.18 | 0.00 | 0.08 | 0.55 | 0.07 | 0.05 | 3.23 | 1.30 | 0.76 | 0.46 | 0.06 | 0.67 | 0.07 | 11.01 | 1.40 |
| greedy | fresh/on | 0.89 | 0.00 | 5.00 | 13.03 | 0.06 | 0.55 | 0.05 | 0.05 | 3.22 | 1.26 | 0.63 | 0.55 | 0.07 | 0.72 | 0.09 | 11.42 | 1.42 |

## The new cards played a game; wins through cards

| agent | arm | hostile_buyout | open_research_campus | industrial_commons | won games with more card plays and uses than standard actions | card share of actions in won games |
|---|---|---|---|---|---|---|
| strategist | fresh/off | 0.208 | 0.153 | 0.169 | 100% | 86% |
| strategist | fresh/on | 0.201 | 0.167 | 0.190 | 100% | 86% |
| greedy | fresh/off | 0.154 | 0.142 | 0.237 | 100% | 88% |
| greedy | fresh/on | 0.185 | 0.165 | 0.264 | 100% | 88% |

## Resource use: spent a seat-game (spent / gained)

| agent | arm | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| strategist | fresh/off | 326.1 (104%) | 48.4 (83%) | 32.5 (89%) | 4.9 (66%) | 1.3 (44%) | 3.5 (60%) | 17.6 (78%) |
| strategist | fresh/on | 330.8 (104%) | 49.4 (83%) | 33.7 (88%) | 5.1 (67%) | 1.3 (44%) | 3.6 (60%) | 17.2 (82%) |
| greedy | fresh/off | 349.6 (103%) | 53.4 (83%) | 32.3 (91%) | 5.5 (64%) | 1.2 (53%) | 3.9 (60%) | 18.8 (81%) |
| greedy | fresh/on | 347.7 (103%) | 53.8 (83%) | 32.9 (90%) | 5.7 (65%) | 1.3 (53%) | 4.0 (61%) | 18.3 (84%) |
