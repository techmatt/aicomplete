## Wins (paired against screen/base, same pair, level and seed)

| agent | arm | d0 | d1 | d2 | d3 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|
| strategist | screen/base | 94.8 | 95.8 | 83.3 | 53.1 | 81.8 |  | 384 |
| strategist | screen/on | 92.7 | 89.6 | 76.0 | 50.0 | 77.1 | -4.7 ± 2.2 | 384 |
| strategist | screen/host | 92.7 | 91.7 | 85.4 | 56.2 | 81.5 | -0.3 ± 2.2 | 384 |
| strategist | screen/lib2 | 96.9 | 90.6 | 83.3 | 51.0 | 80.5 | -1.3 ± 2.3 | 384 |
| strategist | screen/hostlib2 | 96.9 | 92.7 | 86.5 | 56.2 | 83.1 | +1.3 ± 2.1 | 384 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|
| strategist | screen/on | -2.1 ± 3.0 | -6.2 ± 3.6 | -7.3 ± 5.4 | -3.1 ± 5.4 |
| strategist | screen/host | -2.1 ± 3.6 | -4.2 ± 2.9 | +2.1 ± 4.7 | +3.1 ± 6.0 |
| strategist | screen/lib2 | +2.1 ± 3.0 | -5.2 ± 3.7 | +0.0 ± 4.4 | -2.1 ± 6.3 |
| strategist | screen/hostlib2 | +2.1 ± 3.0 | -3.1 ± 2.8 | +3.1 ± 4.8 | +3.1 ± 5.8 |

## Losses by reason a game; d3 losses (node / deck / other) of d3 games

| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |
|---|---|---|---|---|---|---|---|---|---|
| strategist | screen/base | 9.22 | 0.099 | 0.076 | 0.008 | 28 | 15 | 2 | 96 |
| strategist | screen/on | 9.55 | 0.125 | 0.099 | 0.005 | 31 | 15 | 2 | 96 |
| strategist | screen/host | 9.22 | 0.094 | 0.091 | 0.000 | 22 | 20 | 0 | 96 |
| strategist | screen/lib2 | 9.39 | 0.104 | 0.089 | 0.003 | 27 | 19 | 1 | 96 |
| strategist | screen/hostlib2 | 9.12 | 0.096 | 0.073 | 0.000 | 25 | 17 | 0 | 96 |

## Nodes placed a seat-game by round, per board

| agent | arm | board | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | screen/base | builder | 3.16 | 2.33 | 1.26 | 1.05 | 1.04 | 0.73 | 0.50 | 0.35 | 0.24 | 0.10 |
| strategist | screen/base | protector | 2.30 | 2.30 | 1.64 | 1.32 | 1.17 | 0.91 | 0.69 | 0.49 | 0.30 | 0.15 |
| strategist | screen/base | researcher | 1.73 | 2.56 | 2.18 | 2.00 | 1.67 | 0.56 | 0.30 | 0.18 | 0.14 | 0.09 |
| strategist | screen/on | builder | 3.16 | 2.09 | 1.07 | 0.94 | 0.93 | 0.66 | 0.56 | 0.45 | 0.33 | 0.19 |
| strategist | screen/on | protector | 2.55 | 2.07 | 1.39 | 1.18 | 1.04 | 0.88 | 0.75 | 0.51 | 0.38 | 0.25 |
| strategist | screen/on | researcher | 1.71 | 2.62 | 2.12 | 1.77 | 1.31 | 0.51 | 0.27 | 0.32 | 0.19 | 0.14 |
| strategist | screen/host | builder | 3.16 | 2.36 | 1.26 | 1.06 | 0.96 | 0.80 | 0.62 | 0.39 | 0.26 | 0.11 |
| strategist | screen/host | protector | 2.38 | 2.25 | 1.59 | 1.36 | 1.12 | 0.95 | 0.73 | 0.48 | 0.27 | 0.16 |
| strategist | screen/host | researcher | 1.71 | 2.53 | 2.22 | 2.18 | 1.73 | 0.56 | 0.36 | 0.20 | 0.14 | 0.06 |
| strategist | screen/lib2 | builder | 3.16 | 2.20 | 1.21 | 1.09 | 0.92 | 0.73 | 0.59 | 0.42 | 0.28 | 0.18 |
| strategist | screen/lib2 | protector | 2.70 | 2.09 | 1.46 | 1.28 | 1.14 | 0.87 | 0.73 | 0.50 | 0.30 | 0.18 |
| strategist | screen/lib2 | researcher | 1.71 | 2.62 | 2.18 | 1.89 | 1.44 | 0.59 | 0.29 | 0.22 | 0.27 | 0.10 |
| strategist | screen/hostlib2 | builder | 3.16 | 2.41 | 1.45 | 1.20 | 1.08 | 0.79 | 0.57 | 0.42 | 0.22 | 0.12 |
| strategist | screen/hostlib2 | protector | 2.52 | 2.34 | 1.77 | 1.40 | 1.12 | 0.90 | 0.65 | 0.49 | 0.24 | 0.10 |
| strategist | screen/hostlib2 | researcher | 1.71 | 2.51 | 2.28 | 2.19 | 1.89 | 0.61 | 0.32 | 0.22 | 0.11 | 0.07 |

## Nodes by source, a seat-game (share)

| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |
|---|---|---|---|---|---|---|
| strategist | screen/base | 11.23 | 5.15 (46%) | 5.30 (47%) | 0.00 (0%) | 0.77 (7%) |
| strategist | screen/on | 10.88 | 5.28 (49%) | 4.83 (44%) | 0.00 (0%) | 0.77 (7%) |
| strategist | screen/host | 11.37 | 5.21 (46%) | 5.35 (47%) | 0.00 (0%) | 0.80 (7%) |
| strategist | screen/lib2 | 11.20 | 5.31 (47%) | 5.11 (46%) | 0.00 (0%) | 0.77 (7%) |
| strategist | screen/hostlib2 | 11.65 | 5.27 (45%) | 5.57 (48%) | 0.00 (0%) | 0.82 (7%) |

## Board powers a seat-game (that board's seat-games)

| agent | arm | board | power | uses |
|---|---|---|---|---|
| strategist | screen/base | builder | Annex | 2.00 |
| strategist | screen/base | builder | Buy In | 2.45 |
| strategist | screen/base | builder | Expand | 5.50 |
| strategist | screen/base | builder | Fortify | 0.59 |
| strategist | screen/base | builder | Mint | 3.26 |
| strategist | screen/base | builder | Overcharge | 1.48 |
| strategist | screen/base | builder | Procure | 5.08 |
| strategist | screen/base | builder | Rush Build | 1.05 |
| strategist | screen/base | builder | Scrap | 1.94 |
| strategist | screen/base | builder | Teardown | 0.04 |
| strategist | screen/base | protector | Accord | 1.88 |
| strategist | screen/base | protector | Bulwark | 1.79 |
| strategist | screen/base | protector | Debrief | 2.55 |
| strategist | screen/base | protector | Expand | 5.05 |
| strategist | screen/base | protector | Interdict | 0.94 |
| strategist | screen/base | protector | Outpost | 3.77 |
| strategist | screen/base | protector | Patrol | 1.93 |
| strategist | screen/base | protector | Reclaim | 0.08 |
| strategist | screen/base | researcher | Audit | 1.91 |
| strategist | screen/base | researcher | Disrupt | 0.72 |
| strategist | screen/base | researcher | Expand | 4.91 |
| strategist | screen/base | researcher | Field Station | 1.04 |
| strategist | screen/base | researcher | Field Survey | 3.71 |
| strategist | screen/base | researcher | Grant | 2.04 |
| strategist | screen/base | researcher | Meltdown | 0.21 |
| strategist | screen/base | researcher | Peer Review | 2.09 |
| strategist | screen/base | researcher | Synthesis | 6.24 |
| strategist | screen/on | builder | Annex | 1.93 |
| strategist | screen/on | builder | Buy In | 2.21 |
| strategist | screen/on | builder | Expand | 5.60 |
| strategist | screen/on | builder | Fortify | 0.71 |
| strategist | screen/on | builder | Mint | 3.31 |
| strategist | screen/on | builder | Overcharge | 1.62 |
| strategist | screen/on | builder | Procure | 5.08 |
| strategist | screen/on | builder | Rush Build | 1.00 |
| strategist | screen/on | builder | Scrap | 1.94 |
| strategist | screen/on | builder | Teardown | 0.09 |
| strategist | screen/on | protector | Accord | 1.86 |
| strategist | screen/on | protector | Bulwark | 1.49 |
| strategist | screen/on | protector | Debrief | 2.60 |
| strategist | screen/on | protector | Expand | 5.55 |
| strategist | screen/on | protector | Interdict | 1.02 |
| strategist | screen/on | protector | Outpost | 2.97 |
| strategist | screen/on | protector | Patrol | 2.10 |
| strategist | screen/on | protector | Reclaim | 0.06 |
| strategist | screen/on | researcher | Audit | 2.00 |
| strategist | screen/on | researcher | Disrupt | 1.03 |
| strategist | screen/on | researcher | Expand | 4.69 |
| strategist | screen/on | researcher | Field Station | 0.98 |
| strategist | screen/on | researcher | Field Survey | 3.68 |
| strategist | screen/on | researcher | Grant | 1.96 |
| strategist | screen/on | researcher | Meltdown | 0.26 |
| strategist | screen/on | researcher | Peer Review | 1.84 |
| strategist | screen/on | researcher | Synthesis | 6.61 |
| strategist | screen/host | builder | Annex | 2.02 |
| strategist | screen/host | builder | Buy In | 2.59 |
| strategist | screen/host | builder | Expand | 5.59 |
| strategist | screen/host | builder | Fortify | 0.79 |
| strategist | screen/host | builder | Mint | 3.34 |
| strategist | screen/host | builder | Overcharge | 1.53 |
| strategist | screen/host | builder | Procure | 5.09 |
| strategist | screen/host | builder | Rush Build | 0.98 |
| strategist | screen/host | builder | Scrap | 2.05 |
| strategist | screen/host | builder | Teardown | 0.07 |
| strategist | screen/host | protector | Accord | 1.88 |
| strategist | screen/host | protector | Bulwark | 1.64 |
| strategist | screen/host | protector | Debrief | 2.37 |
| strategist | screen/host | protector | Expand | 5.09 |
| strategist | screen/host | protector | Interdict | 0.88 |
| strategist | screen/host | protector | Outpost | 3.63 |
| strategist | screen/host | protector | Patrol | 1.92 |
| strategist | screen/host | protector | Reclaim | 0.11 |
| strategist | screen/host | researcher | Audit | 1.77 |
| strategist | screen/host | researcher | Disrupt | 0.82 |
| strategist | screen/host | researcher | Expand | 4.96 |
| strategist | screen/host | researcher | Field Station | 1.06 |
| strategist | screen/host | researcher | Field Survey | 3.84 |
| strategist | screen/host | researcher | Grant | 2.00 |
| strategist | screen/host | researcher | Meltdown | 0.17 |
| strategist | screen/host | researcher | Peer Review | 2.09 |
| strategist | screen/host | researcher | Synthesis | 6.19 |
| strategist | screen/lib2 | builder | Annex | 2.08 |
| strategist | screen/lib2 | builder | Buy In | 2.39 |
| strategist | screen/lib2 | builder | Expand | 5.66 |
| strategist | screen/lib2 | builder | Fortify | 0.62 |
| strategist | screen/lib2 | builder | Mint | 3.14 |
| strategist | screen/lib2 | builder | Overcharge | 1.50 |
| strategist | screen/lib2 | builder | Procure | 5.13 |
| strategist | screen/lib2 | builder | Rush Build | 1.04 |
| strategist | screen/lib2 | builder | Scrap | 1.98 |
| strategist | screen/lib2 | builder | Teardown | 0.05 |
| strategist | screen/lib2 | protector | Accord | 2.01 |
| strategist | screen/lib2 | protector | Bulwark | 1.60 |
| strategist | screen/lib2 | protector | Debrief | 2.35 |
| strategist | screen/lib2 | protector | Expand | 5.54 |
| strategist | screen/lib2 | protector | Interdict | 0.88 |
| strategist | screen/lib2 | protector | Outpost | 3.14 |
| strategist | screen/lib2 | protector | Patrol | 2.15 |
| strategist | screen/lib2 | protector | Reclaim | 0.09 |
| strategist | screen/lib2 | researcher | Audit | 1.94 |
| strategist | screen/lib2 | researcher | Disrupt | 0.93 |
| strategist | screen/lib2 | researcher | Expand | 4.74 |
| strategist | screen/lib2 | researcher | Field Station | 0.92 |
| strategist | screen/lib2 | researcher | Field Survey | 3.81 |
| strategist | screen/lib2 | researcher | Grant | 1.95 |
| strategist | screen/lib2 | researcher | Meltdown | 0.22 |
| strategist | screen/lib2 | researcher | Peer Review | 1.92 |
| strategist | screen/lib2 | researcher | Synthesis | 6.33 |
| strategist | screen/hostlib2 | builder | Annex | 2.11 |
| strategist | screen/hostlib2 | builder | Buy In | 2.74 |
| strategist | screen/hostlib2 | builder | Expand | 5.72 |
| strategist | screen/hostlib2 | builder | Fortify | 0.60 |
| strategist | screen/hostlib2 | builder | Mint | 3.38 |
| strategist | screen/hostlib2 | builder | Overcharge | 1.67 |
| strategist | screen/hostlib2 | builder | Procure | 5.11 |
| strategist | screen/hostlib2 | builder | Rush Build | 1.05 |
| strategist | screen/hostlib2 | builder | Scrap | 1.95 |
| strategist | screen/hostlib2 | builder | Teardown | 0.07 |
| strategist | screen/hostlib2 | protector | Accord | 2.00 |
| strategist | screen/hostlib2 | protector | Bulwark | 1.96 |
| strategist | screen/hostlib2 | protector | Debrief | 2.39 |
| strategist | screen/hostlib2 | protector | Expand | 4.98 |
| strategist | screen/hostlib2 | protector | Interdict | 0.87 |
| strategist | screen/hostlib2 | protector | Outpost | 3.80 |
| strategist | screen/hostlib2 | protector | Patrol | 1.91 |
| strategist | screen/hostlib2 | protector | Reclaim | 0.07 |
| strategist | screen/hostlib2 | researcher | Audit | 1.83 |
| strategist | screen/hostlib2 | researcher | Disrupt | 0.76 |
| strategist | screen/hostlib2 | researcher | Expand | 5.10 |
| strategist | screen/hostlib2 | researcher | Field Station | 1.04 |
| strategist | screen/hostlib2 | researcher | Field Survey | 3.93 |
| strategist | screen/hostlib2 | researcher | Grant | 1.95 |
| strategist | screen/hostlib2 | researcher | Meltdown | 0.17 |
| strategist | screen/hostlib2 | researcher | Peer Review | 2.13 |
| strategist | screen/hostlib2 | researcher | Synthesis | 6.05 |

## Standard actions a seat-game

| agent | arm | ambassador | antimatter | botnet | buy_energy | claim | envoy | insight | liberate | lift | move | remove | station | troop | beyond Dispatch and Liberate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | screen/base | 0.76 | 0.04 | 0.92 | 3.51 | 0.05 | 0.01 | 0.47 | 2.27 | 0.10 | 9.91 | 0.54 | 0.25 | 0.47 | **7.12** |
| strategist | screen/on | 0.68 | 0.03 | 0.87 | 4.51 | 0.04 | 0.02 | 0.46 | 2.22 | 0.10 | 9.81 | 0.52 | 0.26 | 0.55 | **8.04** |
| strategist | screen/host | 0.90 | 0.04 | 0.92 | 3.46 | 0.03 | 0.03 | 0.42 | 2.26 | 0.11 | 9.66 | 0.47 | 0.23 | 0.46 | **7.08** |
| strategist | screen/lib2 | 0.63 | 0.02 | 0.84 | 4.18 | 0.03 | 0.02 | 0.46 | 2.15 | 0.09 | 9.71 | 0.49 | 0.22 | 0.53 | **7.51** |
| strategist | screen/hostlib2 | 0.75 | 0.03 | 0.99 | 3.37 | 0.02 | 0.02 | 0.44 | 2.21 | 0.11 | 9.69 | 0.51 | 0.25 | 0.42 | **6.91** |

## Liberations, claims, retakes and neutral sites, a game

| agent | arm | adv.retakes | city_energy_aligned | city_energy_bought | city_energy_hosted | claimed.factory | claimed.station | claimed_liberated | factories_built.player | liberate_energy | liberated.city | liberated.factory | liberated.station | liberated_ambassadors | liberated_botnet | liberated_card | liberated_troops | node_lost | stations_built_free |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | screen/base | 0.74 | 11.94 | 7.02 | 0.00 | 0.10 | 0.59 | 0.08 | 0.09 | 0.00 | 3.45 | 1.16 | 0.89 | 0.39 | 0.07 | 0.96 | 0.13 | 9.69 | 1.36 |
| strategist | screen/on | 0.86 | 0.00 | 9.01 | 0.00 | 0.08 | 0.53 | 0.05 | 0.11 | 0.00 | 3.55 | 1.11 | 0.79 | 0.50 | 0.06 | 1.02 | 0.11 | 10.67 | 1.32 |
| strategist | screen/host | 0.68 | 0.00 | 6.93 | 13.75 | 0.06 | 0.56 | 0.07 | 0.11 | 0.00 | 3.47 | 1.28 | 0.70 | 0.45 | 0.07 | 0.93 | 0.15 | 9.87 | 1.38 |
| strategist | screen/lib2 | 0.68 | 0.00 | 8.36 | 0.00 | 0.07 | 0.58 | 0.07 | 0.10 | 7.09 | 3.54 | 1.10 | 0.71 | 0.54 | 0.07 | 1.05 | 0.10 | 10.55 | 1.28 |
| strategist | screen/hostlib2 | 0.70 | 0.00 | 6.75 | 13.76 | 0.05 | 0.65 | 0.06 | 0.10 | 6.70 | 3.35 | 1.20 | 0.74 | 0.43 | 0.07 | 0.88 | 0.10 | 9.71 | 1.36 |

## The new cards played a game; wins through cards

| agent | arm | hostile_buyout | open_research_campus | industrial_commons | won games with more card plays and uses than standard actions | card share of actions in won games |
|---|---|---|---|---|---|---|
| strategist | screen/base | 0.130 | 0.104 | 0.130 | 100% | 85% |
| strategist | screen/on | 0.167 | 0.102 | 0.130 | 100% | 85% |
| strategist | screen/host | 0.148 | 0.120 | 0.161 | 100% | 86% |
| strategist | screen/lib2 | 0.180 | 0.128 | 0.156 | 100% | 85% |
| strategist | screen/hostlib2 | 0.143 | 0.120 | 0.148 | 100% | 86% |

## Resource use: spent a seat-game (spent / gained)

| agent | arm | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| strategist | screen/base | 347.2 (104%) | 51.6 (83%) | 35.2 (90%) | 5.2 (67%) | 1.5 (41%) | 4.0 (57%) | 18.3 (78%) |
| strategist | screen/on | 364.0 (104%) | 53.3 (84%) | 31.9 (88%) | 5.1 (65%) | 1.5 (44%) | 4.0 (63%) | 18.0 (82%) |
| strategist | screen/host | 355.8 (104%) | 51.8 (83%) | 36.0 (87%) | 5.4 (66%) | 1.4 (37%) | 4.0 (57%) | 17.8 (80%) |
| strategist | screen/lib2 | 351.2 (104%) | 52.3 (84%) | 34.2 (89%) | 5.0 (66%) | 1.5 (43%) | 4.0 (62%) | 17.6 (83%) |
| strategist | screen/hostlib2 | 351.7 (104%) | 51.8 (83%) | 38.3 (86%) | 5.4 (65%) | 1.4 (36%) | 4.0 (56%) | 17.6 (80%) |
