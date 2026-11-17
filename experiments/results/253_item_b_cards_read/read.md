## Wins (paired against r112/base, same pair, level and seed)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | r112/base | 91.7 | 94.4 | 88.9 | 63.9 | 43.8 | 76.5 |  | 720 |
| strategist | r113/base | 93.1 | 93.1 | 82.6 | 54.9 | 22.9 | 69.3 | -7.2 ± 1.8 | 720 |
| strategist | r114/base | 97.2 | 91.7 | 86.8 | 54.9 | 20.8 | 70.3 | -6.2 ± 2.0 | 720 |
| strategist | r114/r4off | 96.5 | 91.7 | 86.1 | 60.4 | 29.2 | 72.8 | -3.8 ± 1.9 | 720 |
| greedy | r112/base | 93.1 | 88.9 | 83.3 | 63.9 | 39.6 | 73.8 |  | 720 |
| greedy | r114/base | 84.0 | 79.9 | 77.1 | 45.1 | 15.3 | 60.3 | -13.5 ± 2.2 | 720 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | r113/base | +1.4 ± 3.1 | -1.4 ± 2.8 | -6.2 ± 4.0 | -9.0 ± 5.0 | -20.8 ± 4.5 |
| strategist | r114/base | +5.6 ± 2.7 | -2.8 ± 3.1 | -2.1 ± 3.9 | -9.0 ± 5.8 | -22.9 ± 5.2 |
| strategist | r114/r4off | +4.9 ± 2.8 | -2.8 ± 3.1 | -2.8 ± 3.5 | -3.5 ± 5.7 | -14.6 ± 5.6 |
| greedy | r114/base | -9.0 ± 3.5 | -9.0 ± 4.3 | -6.2 ± 4.7 | -18.8 ± 6.0 | -24.3 ± 5.1 |

## Losses by reason a game; d3 losses (node / deck / other) of d3 games

| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |
|---|---|---|---|---|---|---|---|---|---|
| strategist | r112/base | 8.51 | 0.097 | 0.122 | 0.015 | 18 | 33 | 1 | 144 |
| strategist | r113/base | 8.86 | 0.189 | 0.097 | 0.021 | 41 | 24 | 0 | 144 |
| strategist | r114/base | 8.90 | 0.210 | 0.068 | 0.019 | 40 | 20 | 5 | 144 |
| strategist | r114/r4off | 8.69 | 0.146 | 0.099 | 0.028 | 28 | 25 | 4 | 144 |
| greedy | r112/base | 8.94 | 0.104 | 0.149 | 0.010 | 17 | 34 | 1 | 144 |
| greedy | r114/base | 9.52 | 0.224 | 0.161 | 0.013 | 45 | 32 | 2 | 144 |

## Nodes placed a seat-game by round, per board

| agent | arm | board | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r112/base | builder | 1.92 | 1.24 | 1.41 | 1.40 | 1.21 | 1.01 | 0.69 | 0.38 | 0.20 | 0.13 |
| strategist | r112/base | protector | 2.63 | 1.46 | 1.23 | 1.22 | 1.15 | 0.93 | 0.68 | 0.46 | 0.27 | 0.11 |
| strategist | r112/base | researcher | 2.10 | 1.27 | 1.54 | 1.48 | 1.33 | 1.04 | 0.67 | 0.40 | 0.21 | 0.09 |
| strategist | r113/base | builder | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| strategist | r113/base | protector | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| strategist | r113/base | researcher | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| strategist | r114/base | builder | 3.18 | 2.37 | 1.24 | 1.05 | 0.84 | 0.64 | 0.49 | 0.35 | 0.20 | 0.13 |
| strategist | r114/base | protector | 2.29 | 2.25 | 1.61 | 1.14 | 1.04 | 0.84 | 0.75 | 0.48 | 0.29 | 0.15 |
| strategist | r114/base | researcher | 1.61 | 2.58 | 2.20 | 1.89 | 1.52 | 0.55 | 0.30 | 0.22 | 0.11 | 0.05 |
| strategist | r114/r4off | builder | 3.18 | 2.37 | 1.27 | 1.07 | 0.91 | 0.66 | 0.55 | 0.33 | 0.17 | 0.10 |
| strategist | r114/r4off | protector | 2.26 | 2.27 | 1.64 | 1.22 | 1.10 | 0.87 | 0.73 | 0.48 | 0.27 | 0.11 |
| strategist | r114/r4off | researcher | 1.60 | 2.58 | 2.18 | 1.92 | 1.62 | 0.69 | 0.37 | 0.22 | 0.12 | 0.06 |
| greedy | r112/base | builder | 2.02 | 1.25 | 1.34 | 1.31 | 1.14 | 0.91 | 0.60 | 0.44 | 0.26 | 0.20 |
| greedy | r112/base | protector | 2.69 | 1.57 | 1.31 | 1.22 | 1.06 | 0.97 | 0.71 | 0.43 | 0.31 | 0.19 |
| greedy | r112/base | researcher | 2.15 | 1.43 | 1.60 | 1.45 | 1.28 | 0.97 | 0.70 | 0.50 | 0.23 | 0.14 |
| greedy | r114/base | builder | 3.10 | 2.14 | 1.12 | 0.98 | 0.78 | 0.66 | 0.57 | 0.46 | 0.29 | 0.16 |
| greedy | r114/base | protector | 2.27 | 2.24 | 1.64 | 1.20 | 1.02 | 0.85 | 0.75 | 0.55 | 0.35 | 0.20 |
| greedy | r114/base | researcher | 1.57 | 2.52 | 2.28 | 1.88 | 1.64 | 0.67 | 0.36 | 0.24 | 0.17 | 0.09 |

## Nodes by source, a seat-game (share)

| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |
|---|---|---|---|---|---|---|
| strategist | r112/base | 10.01 | 0.00 (0%) | 4.75 (47%) | 3.84 (38%) | 1.42 (14%) |
| strategist | r113/base | 10.92 | 5.02 (46%) | 5.11 (47%) | 0.00 (0%) | 0.79 (7%) |
| strategist | r114/base | 10.84 | 4.98 (46%) | 5.10 (47%) | 0.00 (0%) | 0.76 (7%) |
| strategist | r114/r4off | 11.02 | 4.92 (45%) | 5.30 (48%) | 0.00 (0%) | 0.79 (7%) |
| greedy | r112/base | 10.24 | 0.00 (0%) | 4.88 (48%) | 3.73 (36%) | 1.62 (16%) |
| greedy | r114/base | 11.02 | 5.08 (46%) | 5.00 (45%) | 0.00 (0%) | 0.94 (9%) |

## Board powers a seat-game (that board's seat-games)

| agent | arm | board | power | uses |
|---|---|---|---|---|
| strategist | r112/base | builder | Buy In | 5.76 |
| strategist | r112/base | builder | Fortify | 0.73 |
| strategist | r112/base | builder | Mint | 2.97 |
| strategist | r112/base | builder | Overcharge | 1.70 |
| strategist | r112/base | builder | Procure | 5.25 |
| strategist | r112/base | builder | Rush Build | 1.39 |
| strategist | r112/base | builder | Scrap | 1.87 |
| strategist | r112/base | builder | Teardown | 0.07 |
| strategist | r112/base | protector | Accord | 5.60 |
| strategist | r112/base | protector | Bulwark | 3.44 |
| strategist | r112/base | protector | Debrief | 2.04 |
| strategist | r112/base | protector | Interdict | 0.85 |
| strategist | r112/base | protector | Patrol | 2.57 |
| strategist | r112/base | protector | Reclaim | 0.10 |
| strategist | r112/base | researcher | Audit | 1.27 |
| strategist | r112/base | researcher | Disrupt | 0.67 |
| strategist | r112/base | researcher | Field Station | 0.58 |
| strategist | r112/base | researcher | Grant | 1.48 |
| strategist | r112/base | researcher | Meltdown | 0.28 |
| strategist | r112/base | researcher | Peer Review | 2.88 |
| strategist | r112/base | researcher | Synthesis | 6.79 |
| strategist | r113/base | builder | Annex | 2.24 |
| strategist | r113/base | builder | Buy In | 2.36 |
| strategist | r113/base | builder | Expand | 5.19 |
| strategist | r113/base | builder | Fortify | 0.63 |
| strategist | r113/base | builder | Mint | 3.41 |
| strategist | r113/base | builder | Overcharge | 1.63 |
| strategist | r113/base | builder | Procure | 5.11 |
| strategist | r113/base | builder | Rush Build | 0.95 |
| strategist | r113/base | builder | Scrap | 2.21 |
| strategist | r113/base | builder | Teardown | 0.04 |
| strategist | r113/base | protector | Accord | 1.77 |
| strategist | r113/base | protector | Bulwark | 1.37 |
| strategist | r113/base | protector | Debrief | 2.46 |
| strategist | r113/base | protector | Expand | 4.92 |
| strategist | r113/base | protector | Interdict | 0.97 |
| strategist | r113/base | protector | Outpost | 3.58 |
| strategist | r113/base | protector | Patrol | 1.87 |
| strategist | r113/base | protector | Reclaim | 0.10 |
| strategist | r113/base | researcher | Audit | 1.90 |
| strategist | r113/base | researcher | Disrupt | 0.82 |
| strategist | r113/base | researcher | Expand | 4.93 |
| strategist | r113/base | researcher | Field Station | 0.97 |
| strategist | r113/base | researcher | Field Survey | 3.50 |
| strategist | r113/base | researcher | Grant | 2.21 |
| strategist | r113/base | researcher | Meltdown | 0.19 |
| strategist | r113/base | researcher | Peer Review | 1.90 |
| strategist | r113/base | researcher | Synthesis | 5.73 |
| strategist | r114/base | builder | Annex | 2.23 |
| strategist | r114/base | builder | Buy In | 2.44 |
| strategist | r114/base | builder | Expand | 5.11 |
| strategist | r114/base | builder | Fortify | 0.61 |
| strategist | r114/base | builder | Mint | 3.45 |
| strategist | r114/base | builder | Overcharge | 1.50 |
| strategist | r114/base | builder | Procure | 5.02 |
| strategist | r114/base | builder | Rush Build | 0.92 |
| strategist | r114/base | builder | Scrap | 1.91 |
| strategist | r114/base | builder | Teardown | 0.05 |
| strategist | r114/base | protector | Accord | 1.71 |
| strategist | r114/base | protector | Bulwark | 1.13 |
| strategist | r114/base | protector | Debrief | 2.51 |
| strategist | r114/base | protector | Expand | 5.07 |
| strategist | r114/base | protector | Interdict | 0.96 |
| strategist | r114/base | protector | Outpost | 3.48 |
| strategist | r114/base | protector | Patrol | 2.05 |
| strategist | r114/base | protector | Reclaim | 0.07 |
| strategist | r114/base | researcher | Audit | 1.77 |
| strategist | r114/base | researcher | Disrupt | 0.79 |
| strategist | r114/base | researcher | Expand | 4.78 |
| strategist | r114/base | researcher | Field Station | 0.89 |
| strategist | r114/base | researcher | Field Survey | 3.66 |
| strategist | r114/base | researcher | Grant | 2.28 |
| strategist | r114/base | researcher | Meltdown | 0.20 |
| strategist | r114/base | researcher | Peer Review | 1.81 |
| strategist | r114/base | researcher | Synthesis | 5.89 |
| strategist | r114/r4off | builder | Annex | 2.16 |
| strategist | r114/r4off | builder | Buy In | 2.58 |
| strategist | r114/r4off | builder | Expand | 5.09 |
| strategist | r114/r4off | builder | Fortify | 0.63 |
| strategist | r114/r4off | builder | Mint | 3.54 |
| strategist | r114/r4off | builder | Overcharge | 1.39 |
| strategist | r114/r4off | builder | Procure | 5.16 |
| strategist | r114/r4off | builder | Rush Build | 0.91 |
| strategist | r114/r4off | builder | Scrap | 1.79 |
| strategist | r114/r4off | builder | Teardown | 0.07 |
| strategist | r114/r4off | protector | Accord | 1.71 |
| strategist | r114/r4off | protector | Bulwark | 1.86 |
| strategist | r114/r4off | protector | Debrief | 2.48 |
| strategist | r114/r4off | protector | Expand | 4.77 |
| strategist | r114/r4off | protector | Interdict | 0.95 |
| strategist | r114/r4off | protector | Outpost | 3.83 |
| strategist | r114/r4off | protector | Patrol | 1.89 |
| strategist | r114/r4off | protector | Reclaim | 0.09 |
| strategist | r114/r4off | researcher | Audit | 1.73 |
| strategist | r114/r4off | researcher | Disrupt | 0.69 |
| strategist | r114/r4off | researcher | Expand | 4.90 |
| strategist | r114/r4off | researcher | Field Station | 0.89 |
| strategist | r114/r4off | researcher | Field Survey | 3.75 |
| strategist | r114/r4off | researcher | Grant | 2.35 |
| strategist | r114/r4off | researcher | Meltdown | 0.21 |
| strategist | r114/r4off | researcher | Peer Review | 1.88 |
| strategist | r114/r4off | researcher | Synthesis | 5.66 |
| greedy | r112/base | builder | Buy In | 5.83 |
| greedy | r112/base | builder | Fortify | 0.59 |
| greedy | r112/base | builder | Mint | 2.87 |
| greedy | r112/base | builder | Overcharge | 1.88 |
| greedy | r112/base | builder | Procure | 6.28 |
| greedy | r112/base | builder | Rush Build | 1.24 |
| greedy | r112/base | builder | Scrap | 2.18 |
| greedy | r112/base | builder | Teardown | 0.08 |
| greedy | r112/base | protector | Accord | 5.66 |
| greedy | r112/base | protector | Bulwark | 4.06 |
| greedy | r112/base | protector | Debrief | 3.48 |
| greedy | r112/base | protector | Interdict | 0.84 |
| greedy | r112/base | protector | Patrol | 2.28 |
| greedy | r112/base | protector | Reclaim | 0.07 |
| greedy | r112/base | researcher | Audit | 1.92 |
| greedy | r112/base | researcher | Disrupt | 0.88 |
| greedy | r112/base | researcher | Field Station | 0.70 |
| greedy | r112/base | researcher | Grant | 1.86 |
| greedy | r112/base | researcher | Meltdown | 0.29 |
| greedy | r112/base | researcher | Peer Review | 3.15 |
| greedy | r112/base | researcher | Synthesis | 7.23 |
| greedy | r114/base | builder | Annex | 1.62 |
| greedy | r114/base | builder | Buy In | 2.45 |
| greedy | r114/base | builder | Expand | 5.32 |
| greedy | r114/base | builder | Fortify | 0.61 |
| greedy | r114/base | builder | Mint | 3.25 |
| greedy | r114/base | builder | Overcharge | 1.55 |
| greedy | r114/base | builder | Procure | 5.81 |
| greedy | r114/base | builder | Rush Build | 0.86 |
| greedy | r114/base | builder | Scrap | 2.14 |
| greedy | r114/base | builder | Teardown | 0.15 |
| greedy | r114/base | protector | Accord | 2.07 |
| greedy | r114/base | protector | Bulwark | 1.41 |
| greedy | r114/base | protector | Debrief | 3.75 |
| greedy | r114/base | protector | Expand | 5.34 |
| greedy | r114/base | protector | Interdict | 0.91 |
| greedy | r114/base | protector | Outpost | 2.98 |
| greedy | r114/base | protector | Patrol | 2.04 |
| greedy | r114/base | protector | Reclaim | 0.13 |
| greedy | r114/base | researcher | Audit | 2.38 |
| greedy | r114/base | researcher | Disrupt | 1.10 |
| greedy | r114/base | researcher | Expand | 4.58 |
| greedy | r114/base | researcher | Field Station | 1.12 |
| greedy | r114/base | researcher | Field Survey | 4.41 |
| greedy | r114/base | researcher | Grant | 2.67 |
| greedy | r114/base | researcher | Meltdown | 0.23 |
| greedy | r114/base | researcher | Peer Review | 1.48 |
| greedy | r114/base | researcher | Synthesis | 6.57 |

## Standard actions a seat-game

| agent | arm | ambassador | antimatter | botnet | buy_energy | claim | envoy | insight | liberate | lift | move | node | remove | station | troop | beyond Dispatch and Liberate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r112/base | 0.86 | 0.14 | 0.77 | 2.18 | 0.26 | 0.06 | 3.73 | 3.11 | 0.30 | 7.40 | 3.84 | 0.38 | 1.55 | 0.44 | **14.50** |
| strategist | r113/base | 0.96 | 0.05 | 0.77 | 3.44 | 0.04 | 0.06 | 0.12 | 2.44 | 0.13 | 9.26 | 0.00 | 0.43 | 0.24 | 0.50 | **6.73** |
| strategist | r114/base | 0.87 | 0.04 | 0.87 | 3.36 | 0.04 | 0.05 | 0.42 | 2.24 | 0.11 | 8.96 | 0.00 | 0.43 | 0.23 | 0.53 | **6.95** |
| strategist | r114/r4off | 0.99 | 0.04 | 0.84 | 2.41 | 0.03 | 0.09 | 0.53 | 2.65 | 0.11 | 8.67 | 0.00 | 0.41 | 0.24 | 0.48 | **6.18** |
| greedy | r112/base | 0.28 | 0.24 | 0.80 | 2.00 | 0.27 | 0.04 | 3.58 | 3.33 | 0.30 | 8.78 | 3.73 | 0.37 | 1.34 | 0.36 | **13.31** |
| greedy | r114/base | 0.58 | 0.07 | 0.94 | 2.66 | 0.03 | 0.07 | 0.50 | 2.24 | 0.12 | 10.86 | 0.00 | 0.44 | 0.20 | 0.48 | **6.09** |

## Liberations, claims, retakes and neutral sites, a game

| agent | arm | adv.retakes | adv.sites_created.factory | adv.sites_created.station | claimed.factory | claimed.station | claimed_by.ambassadors.factory | claimed_by.card.station | claimed_liberated | liberated.city | liberated.factory | liberated.station | liberated_ambassadors | liberated_botnet | liberated_card | liberated_troops | node_lost | sites_opened.factory | sites_opened.station |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r112/base | 0.00 | 1.34 | 0.00 | 0.52 | 0.89 | 0.52 | 0.89 | 0.00 | 3.89 | 2.12 | 1.30 | 0.42 | 0.08 | 1.09 | 0.10 | 7.81 | 0.00 | 0.00 |
| strategist | r113/base | 0.00 | 1.32 | 0.00 | 0.07 | 0.54 | 0.07 | 0.54 | 0.00 | 3.64 | 1.13 | 0.82 | 0.25 | 0.08 | 0.71 | 0.08 | 10.43 | 0.00 | 0.00 |
| strategist | r114/base | 0.84 | 1.38 | 0.07 | 0.07 | 0.46 | 0.07 | 0.46 | 0.05 | 3.39 | 1.09 | 0.71 | 0.30 | 0.09 | 0.71 | 0.07 | 10.60 | 0.16 | 0.12 |
| strategist | r114/r4off | 0.00 | 1.38 | 0.07 | 0.07 | 0.45 | 0.07 | 0.45 | 0.00 | 3.78 | 1.39 | 0.92 | 0.38 | 0.09 | 0.79 | 0.10 | 10.04 | 0.22 | 0.11 |
| greedy | r112/base | 0.00 | 1.36 | 0.00 | 0.54 | 0.82 | 0.54 | 0.82 | 0.00 | 3.82 | 2.43 | 1.54 | 0.40 | 0.09 | 1.12 | 0.11 | 8.33 | 0.00 | 0.00 |
| greedy | r114/base | 0.89 | 1.33 | 0.07 | 0.07 | 0.63 | 0.07 | 0.63 | 0.04 | 3.35 | 1.19 | 0.60 | 0.36 | 0.07 | 0.67 | 0.07 | 11.26 | 0.29 | 0.14 |

## The new cards played a game; wins through cards

| agent | arm | open_research_campus | industrial_commons | won games with more card plays and uses than standard actions | card share of actions in won games |
|---|---|---|---|---|---|
| strategist | r112/base | 0.000 | 0.000 | 100% | 72% |
| strategist | r113/base | 0.000 | 0.000 | 100% | 86% |
| strategist | r114/base | 0.115 | 0.160 | 100% | 85% |
| strategist | r114/r4off | 0.106 | 0.215 | 100% | 86% |
| greedy | r112/base | 0.000 | 0.000 | 100% | 74% |
| greedy | r114/base | 0.138 | 0.289 | 100% | 87% |

## Resource use: spent a seat-game (spent / gained)

| agent | arm | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| strategist | r112/base | 332.2 (104%) | 46.4 (81%) | 37.1 (83%) | 5.7 (55%) | 1.5 (40%) | 12.9 (59%) | 17.2 (75%) |
| strategist | r113/base | 346.4 (103%) | 49.5 (83%) | 33.2 (88%) | 5.1 (65%) | 1.2 (46%) | 3.6 (55%) | 18.5 (81%) |
| strategist | r114/base | 332.3 (104%) | 48.9 (84%) | 32.1 (90%) | 4.9 (67%) | 1.2 (44%) | 3.4 (58%) | 17.3 (82%) |
| strategist | r114/r4off | 325.5 (104%) | 47.9 (83%) | 33.8 (87%) | 5.3 (60%) | 1.3 (42%) | 3.9 (56%) | 18.6 (79%) |
| greedy | r112/base | 354.1 (104%) | 50.3 (81%) | 37.3 (85%) | 6.5 (55%) | 1.6 (47%) | 13.9 (61%) | 18.6 (78%) |
| greedy | r114/base | 361.0 (103%) | 53.8 (83%) | 32.1 (90%) | 5.5 (64%) | 1.2 (52%) | 4.1 (61%) | 18.7 (85%) |
