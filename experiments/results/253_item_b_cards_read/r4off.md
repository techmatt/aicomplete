## Wins (paired against r114/base, same pair, level and seed)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | r114/base | 97.2 | 91.7 | 86.8 | 54.9 | 20.8 | 70.3 |  | 720 |
| strategist | r114/r4off | 96.5 | 91.7 | 86.1 | 60.4 | 29.2 | 72.8 | +2.5 ± 1.4 | 720 |
| greedy | r114/base | 84.0 | 79.9 | 77.1 | 45.1 | 15.3 | 60.3 |  | 720 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | r114/r4off | -0.7 ± 2.1 | +0.0 ± 2.8 | -0.7 ± 3.2 | +5.6 ± 3.4 | +8.3 ± 3.9 |

## Losses by reason a game; d3 losses (node / deck / other) of d3 games

| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |
|---|---|---|---|---|---|---|---|---|---|
| strategist | r114/base | 8.90 | 0.210 | 0.068 | 0.019 | 40 | 20 | 5 | 144 |
| strategist | r114/r4off | 8.69 | 0.146 | 0.099 | 0.028 | 28 | 25 | 4 | 144 |
| greedy | r114/base | 9.52 | 0.224 | 0.161 | 0.013 | 45 | 32 | 2 | 144 |

## Nodes placed a seat-game by round, per board

| agent | arm | board | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r114/base | builder | 3.18 | 2.37 | 1.24 | 1.05 | 0.84 | 0.64 | 0.49 | 0.35 | 0.20 | 0.13 |
| strategist | r114/base | protector | 2.29 | 2.25 | 1.61 | 1.14 | 1.04 | 0.84 | 0.75 | 0.48 | 0.29 | 0.15 |
| strategist | r114/base | researcher | 1.61 | 2.58 | 2.20 | 1.89 | 1.52 | 0.55 | 0.30 | 0.22 | 0.11 | 0.05 |
| strategist | r114/r4off | builder | 3.18 | 2.37 | 1.27 | 1.07 | 0.91 | 0.66 | 0.55 | 0.33 | 0.17 | 0.10 |
| strategist | r114/r4off | protector | 2.26 | 2.27 | 1.64 | 1.22 | 1.10 | 0.87 | 0.73 | 0.48 | 0.27 | 0.11 |
| strategist | r114/r4off | researcher | 1.60 | 2.58 | 2.18 | 1.92 | 1.62 | 0.69 | 0.37 | 0.22 | 0.12 | 0.06 |
| greedy | r114/base | builder | 3.10 | 2.14 | 1.12 | 0.98 | 0.78 | 0.66 | 0.57 | 0.46 | 0.29 | 0.16 |
| greedy | r114/base | protector | 2.27 | 2.24 | 1.64 | 1.20 | 1.02 | 0.85 | 0.75 | 0.55 | 0.35 | 0.20 |
| greedy | r114/base | researcher | 1.57 | 2.52 | 2.28 | 1.88 | 1.64 | 0.67 | 0.36 | 0.24 | 0.17 | 0.09 |

## Nodes by source, a seat-game (share)

| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |
|---|---|---|---|---|---|---|
| strategist | r114/base | 10.84 | 4.98 (46%) | 5.10 (47%) | 0.00 (0%) | 0.76 (7%) |
| strategist | r114/r4off | 11.02 | 4.92 (45%) | 5.30 (48%) | 0.00 (0%) | 0.79 (7%) |
| greedy | r114/base | 11.02 | 5.08 (46%) | 5.00 (45%) | 0.00 (0%) | 0.94 (9%) |

## Standard actions a seat-game

| agent | arm | ambassador | antimatter | botnet | buy_energy | claim | envoy | insight | liberate | lift | move | remove | station | troop | beyond Dispatch and Liberate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r114/base | 0.87 | 0.04 | 0.87 | 3.36 | 0.04 | 0.05 | 0.42 | 2.24 | 0.11 | 8.96 | 0.43 | 0.23 | 0.53 | **6.95** |
| strategist | r114/r4off | 0.99 | 0.04 | 0.84 | 2.41 | 0.03 | 0.09 | 0.53 | 2.65 | 0.11 | 8.67 | 0.41 | 0.24 | 0.48 | **6.18** |
| greedy | r114/base | 0.58 | 0.07 | 0.94 | 2.66 | 0.03 | 0.07 | 0.50 | 2.24 | 0.12 | 10.86 | 0.44 | 0.20 | 0.48 | **6.09** |

## Liberations, claims, retakes and neutral sites, a game

| agent | arm | adv.retakes | adv.sites_created.factory | adv.sites_created.station | claimed.factory | claimed.station | claimed_by.ambassadors.factory | claimed_by.card.station | claimed_liberated | liberated.city | liberated.factory | liberated.station | liberated_ambassadors | liberated_botnet | liberated_card | liberated_troops | node_lost | sites_opened.factory | sites_opened.station |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| strategist | r114/base | 0.84 | 1.38 | 0.07 | 0.07 | 0.46 | 0.07 | 0.46 | 0.05 | 3.39 | 1.09 | 0.71 | 0.30 | 0.09 | 0.71 | 0.07 | 10.60 | 0.16 | 0.12 |
| strategist | r114/r4off | 0.00 | 1.38 | 0.07 | 0.07 | 0.45 | 0.07 | 0.45 | 0.00 | 3.78 | 1.39 | 0.92 | 0.38 | 0.09 | 0.79 | 0.10 | 10.04 | 0.22 | 0.11 |
| greedy | r114/base | 0.89 | 1.33 | 0.07 | 0.07 | 0.63 | 0.07 | 0.63 | 0.04 | 3.35 | 1.19 | 0.60 | 0.36 | 0.07 | 0.67 | 0.07 | 11.26 | 0.29 | 0.14 |

## The new cards played a game; wins through cards

| agent | arm | open_research_campus | industrial_commons | won games with more card plays and uses than standard actions | card share of actions in won games |
|---|---|---|---|---|---|
| strategist | r114/base | 0.115 | 0.160 | 100% | 85% |
| strategist | r114/r4off | 0.106 | 0.215 | 100% | 86% |
| greedy | r114/base | 0.138 | 0.289 | 100% | 87% |

## Resource use: spent a seat-game (spent / gained)

| agent | arm | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| strategist | r114/base | 332.3 (104%) | 48.9 (84%) | 32.1 (90%) | 4.9 (67%) | 1.2 (44%) | 3.4 (58%) | 17.3 (82%) |
| strategist | r114/r4off | 325.5 (104%) | 47.9 (83%) | 33.8 (87%) | 5.3 (60%) | 1.3 (42%) | 3.9 (56%) | 18.6 (79%) |
| greedy | r114/base | 361.0 (103%) | 53.8 (83%) | 32.1 (90%) | 5.5 (64%) | 1.2 (52%) | 4.1 (61%) | 18.7 (85%) |
