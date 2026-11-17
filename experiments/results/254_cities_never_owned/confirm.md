## Wins (paired against confirm/off, same pair, level and seed)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm/off | 94.4 | 91.7 | 87.5 | 56.9 | 20.1 | 70.1 |  | 720 |
| strategist | confirm/on | 94.4 | 94.4 | 90.3 | 59.7 | 20.8 | 71.9 | +1.8 ± 1.5 | 720 |
| greedy | confirm/off | 88.9 | 86.8 | 84.0 | 56.9 | 22.9 | 67.9 |  | 720 |
| greedy | confirm/on | 89.6 | 90.3 | 83.3 | 62.5 | 24.3 | 70.0 | +2.1 ± 1.8 | 720 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | confirm/on | +0.0 ± 2.6 | +2.8 ± 2.8 | +2.8 ± 3.3 | +2.8 ± 4.4 | +0.7 ± 3.8 |
| greedy | confirm/on | +0.7 ± 3.3 | +3.5 ± 4.0 | -0.7 ± 4.0 | +5.6 ± 4.7 | +1.4 ± 3.8 |

## Losses by reason a game; d3 losses (node / deck / other) of d3 games

| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |
|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm/off | 8.88 | 0.176 | 0.097 | 0.025 | 30 | 30 | 2 | 144 |
| strategist | confirm/on | 8.74 | 0.182 | 0.085 | 0.014 | 33 | 23 | 2 | 144 |
| greedy | confirm/off | 9.23 | 0.181 | 0.122 | 0.018 | 33 | 26 | 3 | 144 |
| greedy | confirm/on | 9.07 | 0.160 | 0.119 | 0.021 | 27 | 25 | 2 | 144 |

## Nodes placed a seat-game by round, per board

| agent | arm | board | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm/off | builder | 3.12 | 2.33 | 1.19 | 1.07 | 0.88 | 0.78 | 0.56 | 0.40 | 0.20 | 0.11 |
| strategist | confirm/off | protector | 2.20 | 2.24 | 1.56 | 1.19 | 1.03 | 0.91 | 0.71 | 0.52 | 0.28 | 0.15 |
| strategist | confirm/off | researcher | 1.50 | 2.57 | 2.27 | 2.02 | 1.77 | 0.70 | 0.27 | 0.17 | 0.13 | 0.04 |
| strategist | confirm/on | builder | 3.12 | 2.28 | 1.22 | 1.09 | 0.99 | 0.80 | 0.50 | 0.34 | 0.18 | 0.11 |
| strategist | confirm/on | protector | 2.41 | 2.10 | 1.52 | 1.20 | 1.02 | 0.89 | 0.70 | 0.48 | 0.29 | 0.10 |
| strategist | confirm/on | researcher | 1.50 | 2.64 | 2.22 | 2.00 | 1.72 | 0.69 | 0.29 | 0.20 | 0.12 | 0.07 |
| greedy | confirm/off | builder | 3.15 | 1.98 | 1.11 | 1.03 | 0.90 | 0.66 | 0.52 | 0.43 | 0.27 | 0.18 |
| greedy | confirm/off | protector | 2.18 | 2.26 | 1.62 | 1.18 | 1.04 | 0.89 | 0.73 | 0.51 | 0.35 | 0.19 |
| greedy | confirm/off | researcher | 1.48 | 2.63 | 2.17 | 2.02 | 1.79 | 0.75 | 0.33 | 0.23 | 0.11 | 0.07 |
| greedy | confirm/on | builder | 3.11 | 2.01 | 1.17 | 1.00 | 1.00 | 0.75 | 0.60 | 0.39 | 0.33 | 0.14 |
| greedy | confirm/on | protector | 2.38 | 2.16 | 1.54 | 1.16 | 1.05 | 0.85 | 0.72 | 0.51 | 0.28 | 0.17 |
| greedy | confirm/on | researcher | 1.48 | 2.61 | 2.26 | 1.97 | 1.93 | 0.75 | 0.37 | 0.20 | 0.17 | 0.07 |

## Nodes by source, a seat-game (share)

| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |
|---|---|---|---|---|---|---|
| strategist | confirm/off | 11.03 | 5.17 (47%) | 5.16 (47%) | 0.00 (0%) | 0.70 (6%) |
| strategist | confirm/on | 10.98 | 5.07 (46%) | 5.21 (47%) | 0.00 (0%) | 0.71 (6%) |
| greedy | confirm/off | 11.04 | 5.09 (46%) | 5.07 (46%) | 0.00 (0%) | 0.87 (8%) |
| greedy | confirm/on | 11.14 | 5.12 (46%) | 5.18 (47%) | 0.00 (0%) | 0.84 (8%) |

## Board powers a seat-game (that board's seat-games)

| agent | arm | board | power | uses |
|---|---|---|---|---|
| strategist | confirm/off | builder | Annex | 2.16 |
| strategist | confirm/off | builder | Buy In | 2.42 |
| strategist | confirm/off | builder | Expand | 5.46 |
| strategist | confirm/off | builder | Fortify | 0.70 |
| strategist | confirm/off | builder | Mint | 3.39 |
| strategist | confirm/off | builder | Overcharge | 1.56 |
| strategist | confirm/off | builder | Procure | 5.03 |
| strategist | confirm/off | builder | Rush Build | 1.02 |
| strategist | confirm/off | builder | Scrap | 2.01 |
| strategist | confirm/off | builder | Teardown | 0.04 |
| strategist | confirm/off | protector | Accord | 1.68 |
| strategist | confirm/off | protector | Bulwark | 1.34 |
| strategist | confirm/off | protector | Debrief | 2.52 |
| strategist | confirm/off | protector | Expand | 5.08 |
| strategist | confirm/off | protector | Interdict | 1.05 |
| strategist | confirm/off | protector | Outpost | 3.56 |
| strategist | confirm/off | protector | Patrol | 1.76 |
| strategist | confirm/off | protector | Reclaim | 0.10 |
| strategist | confirm/off | researcher | Audit | 1.86 |
| strategist | confirm/off | researcher | Disrupt | 0.93 |
| strategist | confirm/off | researcher | Expand | 4.97 |
| strategist | confirm/off | researcher | Field Station | 1.00 |
| strategist | confirm/off | researcher | Field Survey | 3.73 |
| strategist | confirm/off | researcher | Grant | 2.30 |
| strategist | confirm/off | researcher | Meltdown | 0.24 |
| strategist | confirm/off | researcher | Peer Review | 1.94 |
| strategist | confirm/off | researcher | Synthesis | 5.62 |
| strategist | confirm/on | builder | Annex | 2.14 |
| strategist | confirm/on | builder | Buy In | 2.45 |
| strategist | confirm/on | builder | Expand | 5.35 |
| strategist | confirm/on | builder | Fortify | 0.65 |
| strategist | confirm/on | builder | Mint | 3.26 |
| strategist | confirm/on | builder | Overcharge | 1.50 |
| strategist | confirm/on | builder | Procure | 4.64 |
| strategist | confirm/on | builder | Rush Build | 0.96 |
| strategist | confirm/on | builder | Scrap | 1.92 |
| strategist | confirm/on | builder | Teardown | 0.06 |
| strategist | confirm/on | protector | Accord | 1.83 |
| strategist | confirm/on | protector | Bulwark | 1.28 |
| strategist | confirm/on | protector | Debrief | 2.46 |
| strategist | confirm/on | protector | Expand | 4.91 |
| strategist | confirm/on | protector | Interdict | 1.05 |
| strategist | confirm/on | protector | Outpost | 3.46 |
| strategist | confirm/on | protector | Patrol | 1.76 |
| strategist | confirm/on | protector | Reclaim | 0.07 |
| strategist | confirm/on | researcher | Audit | 1.73 |
| strategist | confirm/on | researcher | Disrupt | 0.87 |
| strategist | confirm/on | researcher | Expand | 4.94 |
| strategist | confirm/on | researcher | Field Station | 0.93 |
| strategist | confirm/on | researcher | Field Survey | 3.75 |
| strategist | confirm/on | researcher | Grant | 2.12 |
| strategist | confirm/on | researcher | Meltdown | 0.19 |
| strategist | confirm/on | researcher | Peer Review | 1.99 |
| strategist | confirm/on | researcher | Synthesis | 5.51 |
| greedy | confirm/off | builder | Annex | 1.73 |
| greedy | confirm/off | builder | Buy In | 2.33 |
| greedy | confirm/off | builder | Expand | 5.37 |
| greedy | confirm/off | builder | Fortify | 0.57 |
| greedy | confirm/off | builder | Mint | 3.20 |
| greedy | confirm/off | builder | Overcharge | 1.58 |
| greedy | confirm/off | builder | Procure | 5.32 |
| greedy | confirm/off | builder | Rush Build | 0.80 |
| greedy | confirm/off | builder | Scrap | 2.26 |
| greedy | confirm/off | builder | Teardown | 0.18 |
| greedy | confirm/off | protector | Accord | 2.02 |
| greedy | confirm/off | protector | Bulwark | 1.51 |
| greedy | confirm/off | protector | Debrief | 3.65 |
| greedy | confirm/off | protector | Expand | 5.19 |
| greedy | confirm/off | protector | Interdict | 1.08 |
| greedy | confirm/off | protector | Outpost | 3.15 |
| greedy | confirm/off | protector | Patrol | 1.89 |
| greedy | confirm/off | protector | Reclaim | 0.11 |
| greedy | confirm/off | researcher | Audit | 2.19 |
| greedy | confirm/off | researcher | Disrupt | 1.00 |
| greedy | confirm/off | researcher | Expand | 4.72 |
| greedy | confirm/off | researcher | Field Station | 1.06 |
| greedy | confirm/off | researcher | Field Survey | 4.38 |
| greedy | confirm/off | researcher | Grant | 2.66 |
| greedy | confirm/off | researcher | Meltdown | 0.21 |
| greedy | confirm/off | researcher | Peer Review | 1.61 |
| greedy | confirm/off | researcher | Synthesis | 6.01 |
| greedy | confirm/on | builder | Annex | 1.79 |
| greedy | confirm/on | builder | Buy In | 2.41 |
| greedy | confirm/on | builder | Expand | 5.50 |
| greedy | confirm/on | builder | Fortify | 0.55 |
| greedy | confirm/on | builder | Mint | 3.00 |
| greedy | confirm/on | builder | Overcharge | 1.70 |
| greedy | confirm/on | builder | Procure | 5.34 |
| greedy | confirm/on | builder | Rush Build | 0.85 |
| greedy | confirm/on | builder | Scrap | 2.32 |
| greedy | confirm/on | builder | Teardown | 0.16 |
| greedy | confirm/on | protector | Accord | 2.02 |
| greedy | confirm/on | protector | Bulwark | 1.43 |
| greedy | confirm/on | protector | Debrief | 3.60 |
| greedy | confirm/on | protector | Expand | 5.04 |
| greedy | confirm/on | protector | Interdict | 1.03 |
| greedy | confirm/on | protector | Outpost | 3.21 |
| greedy | confirm/on | protector | Patrol | 1.82 |
| greedy | confirm/on | protector | Reclaim | 0.06 |
| greedy | confirm/on | researcher | Audit | 2.25 |
| greedy | confirm/on | researcher | Disrupt | 0.97 |
| greedy | confirm/on | researcher | Expand | 4.82 |
| greedy | confirm/on | researcher | Field Station | 1.06 |
| greedy | confirm/on | researcher | Field Survey | 4.45 |
| greedy | confirm/on | researcher | Grant | 2.60 |
| greedy | confirm/on | researcher | Meltdown | 0.23 |
| greedy | confirm/on | researcher | Peer Review | 1.68 |
| greedy | confirm/on | researcher | Synthesis | 6.02 |

## Standard actions a seat-game

| agent | arm | ambassador | antimatter | botnet | buy_energy | claim | envoy | insight | liberate | lift | move | remove | station | troop | beyond Dispatch and Liberate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm/off | 1.07 | 0.05 | 0.87 | 3.46 | 0.03 | 0.06 | 0.51 | 2.29 | 0.15 | 9.02 | 0.44 | 0.23 | 0.57 | **7.45** |
| strategist | confirm/on | 1.15 | 0.05 | 0.89 | 4.32 | 0.04 | 0.06 | 0.47 | 2.33 | 0.12 | 8.77 | 0.40 | 0.21 | 0.56 | **8.27** |
| greedy | confirm/off | 0.63 | 0.06 | 0.91 | 2.88 | 0.03 | 0.05 | 0.50 | 2.18 | 0.10 | 10.53 | 0.38 | 0.19 | 0.56 | **6.30** |
| greedy | confirm/on | 0.63 | 0.07 | 0.90 | 3.64 | 0.05 | 0.08 | 0.46 | 2.20 | 0.11 | 10.29 | 0.34 | 0.19 | 0.53 | **7.00** |

## Liberations, claims, retakes and neutral sites, a game

| agent | arm | adv.retakes | city_energy_aligned | city_energy_bought | city_energy_hosted | claimed.factory | claimed.station | claimed_liberated | factories_built.player | liberated.city | liberated.factory | liberated.station | liberated_ambassadors | liberated_botnet | liberated_card | liberated_troops | node_lost | stations_built_free | effects.per_site.owned | effects.per_site.owned_hosted |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | confirm/off | 0.82 | 11.06 | 6.92 | 0.00 | 0.07 | 0.59 | 0.07 | 0.12 | 3.28 | 1.25 | 0.81 | 0.46 | 0.07 | 0.77 | 0.09 | 10.61 | 1.33 | 4.72 | 0.00 |
| strategist | confirm/on | 0.73 | 0.00 | 8.65 | 11.30 | 0.09 | 0.58 | 0.07 | 0.12 | 3.20 | 1.35 | 0.82 | 0.44 | 0.06 | 0.70 | 0.06 | 10.56 | 1.28 | 5.09 | 3.89 |
| greedy | confirm/off | 0.72 | 11.29 | 5.75 | 0.00 | 0.07 | 0.61 | 0.06 | 0.06 | 3.21 | 1.21 | 0.71 | 0.48 | 0.06 | 0.78 | 0.12 | 10.49 | 1.38 | 4.77 | 0.00 |
| greedy | confirm/on | 0.72 | 0.00 | 7.28 | 12.18 | 0.09 | 0.61 | 0.07 | 0.06 | 3.24 | 1.13 | 0.74 | 0.47 | 0.08 | 0.72 | 0.07 | 10.35 | 1.37 | 5.30 | 4.13 |

## The new cards played a game; wins through cards

| agent | arm | hostile_buyout | open_research_campus | industrial_commons | won games with more card plays and uses than standard actions | card share of actions in won games |
|---|---|---|---|---|---|---|
| strategist | confirm/off | 0.153 | 0.128 | 0.178 | 100% | 85% |
| strategist | confirm/on | 0.147 | 0.133 | 0.171 | 100% | 83% |
| greedy | confirm/off | 0.160 | 0.172 | 0.250 | 100% | 87% |
| greedy | confirm/on | 0.157 | 0.153 | 0.217 | 100% | 86% |

## Resource use: spent a seat-game (spent / gained)

| agent | arm | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| strategist | confirm/off | 354.4 (103%) | 50.7 (83%) | 33.3 (87%) | 5.0 (64%) | 1.4 (46%) | 3.8 (56%) | 17.5 (74%) |
| strategist | confirm/on | 349.0 (103%) | 49.7 (83%) | 33.3 (86%) | 4.8 (62%) | 1.3 (44%) | 3.6 (56%) | 17.3 (73%) |
| greedy | confirm/off | 364.5 (103%) | 53.2 (83%) | 32.3 (89%) | 5.4 (62%) | 1.2 (56%) | 3.7 (58%) | 18.0 (77%) |
| greedy | confirm/on | 363.4 (103%) | 52.7 (83%) | 32.8 (88%) | 5.4 (60%) | 1.2 (55%) | 3.7 (61%) | 17.9 (75%) |
