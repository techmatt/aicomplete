# 257 dev screens (the judge, dev seeds 36800-36831, three pairs)

Paired gaps against the shipped judge (`strategist` at the screen's build, w.sink 0), `bench/paired.py --against-agent`.

## Screen 1 (s257-screen, d2-d4): w.sink 0.5 / 1, w.card_left 0.6, both

| cell | slice | n | base win | win | gap ± SE | base rounds | rounds |
|---|---|---|---|---|---|---|---|
| strategist:w.sink=0.5 | d2 | 96 | 82.3 | 83.3 | +1.0 ± 1.8 | 9.42 | 9.46 |
| strategist:w.sink=0.5 | d3 | 96 | 60.4 | 62.5 | +2.1 ± 1.5 | 9.03 | 8.98 |
| strategist:w.sink=0.5 | d4 | 96 | 22.9 | 20.8 | -2.1 ± 1.5 | 8.10 | 8.26 |
| strategist:w.sink=0.5 | all | 288 | 55.2 | 55.6 | +0.3 ± 0.9 | 8.85 | 8.90 |
| strategist:w.sink=1 | d2 | 96 | 82.3 | 82.3 | +0.0 ± 3.0 | 9.42 | 9.34 |
| strategist:w.sink=1 | d3 | 96 | 60.4 | 62.5 | +2.1 ± 2.1 | 9.03 | 8.94 |
| strategist:w.sink=1 | d4 | 96 | 22.9 | 21.9 | -1.0 ± 2.8 | 8.10 | 8.14 |
| strategist:w.sink=1 | all | 288 | 55.2 | 55.6 | +0.3 ± 1.5 | 8.85 | 8.81 |
| strategist:w.card_left=0.6 | d2 | 96 | 82.3 | 79.2 | -3.1 ± 4.0 | 9.42 | 9.21 |
| strategist:w.card_left=0.6 | d3 | 96 | 60.4 | 53.1 | -7.3 ± 5.4 | 9.03 | 8.94 |
| strategist:w.card_left=0.6 | d4 | 96 | 22.9 | 27.1 | +4.2 ± 4.9 | 8.10 | 8.11 |
| strategist:w.card_left=0.6 | all | 288 | 55.2 | 53.1 | -2.1 ± 2.8 | 8.85 | 8.75 |
| strategist:w.sink=0.5,w.card_left=0.6 | d2 | 96 | 82.3 | 79.2 | -3.1 ± 4.0 | 9.42 | 9.22 |
| strategist:w.sink=0.5,w.card_left=0.6 | d3 | 96 | 60.4 | 56.2 | -4.2 ± 5.3 | 9.03 | 8.83 |
| strategist:w.sink=0.5,w.card_left=0.6 | d4 | 96 | 22.9 | 28.1 | +5.2 ± 4.8 | 8.10 | 8.14 |
| strategist:w.sink=0.5,w.card_left=0.6 | all | 288 | 55.2 | 54.5 | -0.7 ± 2.7 | 8.85 | 8.73 |

## Screen 2 (s257-screen2, d2-d4): w.sink 1 / 2; greedy with w.sink 1 (against greedy in the last rows)

| cell | slice | n | base win | win | gap ± SE | base rounds | rounds |
|---|---|---|---|---|---|---|---|
| strategist:w.sink=1 | d2 | 96 | 82.3 | 82.3 | +0.0 ± 3.0 | 9.42 | 9.34 |
| strategist:w.sink=1 | d3 | 96 | 60.4 | 62.5 | +2.1 ± 2.1 | 9.03 | 8.94 |
| strategist:w.sink=1 | d4 | 96 | 22.9 | 21.9 | -1.0 ± 2.8 | 8.10 | 8.14 |
| strategist:w.sink=1 | all | 288 | 55.2 | 55.6 | +0.3 ± 1.5 | 8.85 | 8.81 |
| strategist:w.sink=2 | d2 | 96 | 82.3 | 84.4 | +2.1 ± 3.0 | 9.42 | 9.23 |
| strategist:w.sink=2 | d3 | 96 | 60.4 | 64.6 | +4.2 ± 3.6 | 9.03 | 8.90 |
| strategist:w.sink=2 | d4 | 96 | 22.9 | 22.9 | +0.0 ± 3.9 | 8.10 | 8.16 |
| strategist:w.sink=2 | all | 288 | 55.2 | 57.3 | +2.1 ± 2.0 | 8.85 | 8.76 |
| greedy:w.sink=1 | d2 | 96 | 82.3 | 85.4 | +3.1 ± 4.3 | 9.42 | 9.24 |
| greedy:w.sink=1 | d3 | 96 | 60.4 | 56.2 | -4.2 ± 5.5 | 9.03 | 9.11 |
| greedy:w.sink=1 | d4 | 96 | 22.9 | 20.8 | -2.1 ± 5.5 | 8.10 | 8.40 |
| greedy:w.sink=1 | all | 288 | 55.2 | 54.2 | -1.0 ± 3.0 | 8.85 | 8.92 |
| greedy:w.sink=1 | d2 | 96 | 83.3 | 85.4 | +2.1 ± 2.6 | 9.30 | 9.24 |
| greedy:w.sink=1 | d3 | 96 | 57.3 | 56.2 | -1.0 ± 2.3 | 9.05 | 9.11 |
| greedy:w.sink=1 | d4 | 96 | 21.9 | 20.8 | -1.0 ± 1.0 | 8.46 | 8.40 |
| greedy:w.sink=1 | all | 288 | 54.2 | 54.2 | +0.0 ± 1.2 | 8.94 | 8.92 |

## Screen 3 (s257-screen3, d0-d4): w.sink 2 / 3

| cell | slice | n | base win | win | gap ± SE | base rounds | rounds |
|---|---|---|---|---|---|---|---|
| strategist:w.sink=2 | d0 | 96 | 95.8 | 94.8 | -1.0 ± 3.1 | 9.66 | 9.59 |
| strategist:w.sink=2 | d1 | 96 | 89.6 | 90.6 | +1.0 ± 3.5 | 9.34 | 9.25 |
| strategist:w.sink=2 | d2 | 96 | 82.3 | 84.4 | +2.1 ± 3.0 | 9.42 | 9.23 |
| strategist:w.sink=2 | d3 | 96 | 60.4 | 64.6 | +4.2 ± 3.6 | 9.03 | 8.90 |
| strategist:w.sink=2 | d4 | 96 | 22.9 | 22.9 | +0.0 ± 3.9 | 8.10 | 8.16 |
| strategist:w.sink=2 | all | 480 | 70.2 | 71.5 | +1.2 ± 1.5 | 9.11 | 9.03 |
| strategist:w.sink=3 | d0 | 96 | 95.8 | 91.7 | -4.2 ± 3.3 | 9.66 | 9.93 |
| strategist:w.sink=3 | d1 | 96 | 89.6 | 90.6 | +1.0 ± 3.8 | 9.34 | 9.29 |
| strategist:w.sink=3 | d2 | 96 | 82.3 | 79.2 | -3.1 ± 3.1 | 9.42 | 9.11 |
| strategist:w.sink=3 | d3 | 96 | 60.4 | 60.4 | +0.0 ± 4.4 | 9.03 | 8.92 |
| strategist:w.sink=3 | d4 | 96 | 22.9 | 25.0 | +2.1 ± 4.2 | 8.10 | 8.01 |
| strategist:w.sink=3 | all | 480 | 70.2 | 69.4 | -0.8 ± 1.7 | 9.11 | 9.05 |

## Use, screen 3 (use257.py)

### strategist (480 games)

| building | kept | built | uses | built in round | rounds stood | uses a round stood |
|---|---|---|---|---|---|---|
| mesh_repeater_network | 0.22 | 0.59 | 0.25 | 4.7 | 5.7 | 0.08 |
| research_reactor | 0.37 | 0.69 | 0.27 | 4.1 | 6.1 | 0.07 |
| survey_drone_fleet | 0.32 | 0.20 | 0.07 | 5.6 | 5.2 | 0.07 |
| orbital_relay_launcher | 0.32 | 0.22 | 0.19 | 4.8 | 6.2 | 0.14 |
| positron_storage_ring | 0.33 | 0.25 | 0.40 | 7.2 | 4.2 | 0.39 |
| wafer_fab_line | 0.20 | 0.48 | 0.85 | 5.1 | 5.4 | 0.34 |
| antiproton_foundry | 0.42 | 0.37 | 0.09 | 5.5 | 5.5 | 0.05 |

### strategist:w.sink=2 (480 games)

| building | kept | built | uses | built in round | rounds stood | uses a round stood |
|---|---|---|---|---|---|---|
| mesh_repeater_network | 0.21 | 0.59 | 0.25 | 4.8 | 5.5 | 0.08 |
| research_reactor | 0.36 | 0.71 | 0.75 | 4.3 | 5.8 | 0.19 |
| survey_drone_fleet | 0.30 | 0.26 | 0.15 | 6.2 | 4.8 | 0.12 |
| orbital_relay_launcher | 0.32 | 0.28 | 0.80 | 5.6 | 5.4 | 0.53 |
| positron_storage_ring | 0.30 | 0.23 | 0.48 | 7.2 | 4.2 | 0.53 |
| wafer_fab_line | 0.22 | 0.47 | 0.81 | 5.2 | 5.3 | 0.33 |
| antiproton_foundry | 0.41 | 0.34 | 0.15 | 5.2 | 5.6 | 0.08 |

### strategist:w.sink=3 (480 games)

| building | kept | built | uses | built in round | rounds stood | uses a round stood |
|---|---|---|---|---|---|---|
| mesh_repeater_network | 0.20 | 0.55 | 0.25 | 4.5 | 5.8 | 0.08 |
| research_reactor | 0.34 | 0.72 | 0.83 | 4.4 | 5.9 | 0.20 |
| survey_drone_fleet | 0.31 | 0.26 | 0.14 | 6.0 | 4.9 | 0.11 |
| orbital_relay_launcher | 0.31 | 0.28 | 0.85 | 5.6 | 5.6 | 0.54 |
| positron_storage_ring | 0.32 | 0.25 | 0.47 | 7.3 | 4.0 | 0.49 |
| wafer_fab_line | 0.20 | 0.46 | 0.74 | 5.0 | 5.5 | 0.31 |
| antiproton_foundry | 0.42 | 0.36 | 0.17 | 5.4 | 5.5 | 0.09 |

### Resources: spent a seat-game (spent / gained)

| agent | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|
| strategist | 348.2 (104%) | 50.3 (83%) | 35.6 (88%) | 5.8 (62%) | 1.4 (43%) | 3.7 (62%) | 18.6 (79%) |
| strategist:w.sink=2 | 350.7 (104%) | 50.1 (83%) | 36.9 (85%) | 5.8 (63%) | 2.0 (52%) | 4.1 (63%) | 18.5 (78%) |
| strategist:w.sink=3 | 346.3 (104%) | 49.9 (83%) | 36.8 (84%) | 5.8 (62%) | 2.2 (54%) | 4.2 (64%) | 18.2 (79%) |

## Use, screen 2: greedy with and without w.sink 1

### greedy (288 games)

### greedy:w.sink=1 (288 games)

