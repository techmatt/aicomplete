## Wins and rounds

| run | games | win % | bp / br / pr | rounds | gap vs base ± SE |
|---|---|---|---|---|---|
| E/strategist,w.energy_cap=2 | 288 | 70.1 | 68 / 68 / 75 | 5.77 | +0.7 ± 2.9 (288) |
| E/strategist,w.energy_cap=2,w.energy_idle=0.05 | 288 | 69.1 | 64 / 66 / 78 | 5.81 | -0.3 ± 2.8 (288) |
| E/strategist,w.energy_idle=0.05 | 288 | 72.6 | 68 / 70 / 80 | 5.92 | +3.1 ± 3.0 (288) |
| r51/greedy | 288 | 51.7 | 55 / 44 / 56 | 4.76 |  |
| r51/strategist | 288 | 69.4 | 68 / 67 / 74 | 5.94 |  |
| r52/greedy | 288 | 49.7 | 51 / 47 / 51 | 4.78 | -2.1 ± 3.4 (288) |
| r52/strategist | 288 | 71.9 | 69 / 67 / 80 | 5.97 | +2.4 ± 2.9 (288) |

## Strategist lead over greedy (paired on pair and seed)

- r51: +17.7 ± 3.8 (288 pairs)
- r52: +22.2 ± 3.7 (288 pairs)

## Energy and credits per board (seat-games)

| run | board | seats | energy held after last actions | energy flags | credits gained mean / p99 / max |
|---|---|---|---|---|---|
| E/strategist,w.energy_cap=2 | Builder | 192 | 2.35 | 60 | 218 / 738 / 753 |
| E/strategist,w.energy_cap=2 | Protector | 192 | 1.96 | 17 | 198 / 806 / 877 |
| E/strategist,w.energy_cap=2 | Researcher | 192 | 1.72 | 11 | 237 / 892 / 1104 |
| E/strategist,w.energy_cap=2,w.energy_idle=0.05 | Builder | 192 | 2.20 | 63 | 216 / 720 / 753 |
| E/strategist,w.energy_cap=2,w.energy_idle=0.05 | Protector | 192 | 2.05 | 16 | 203 / 655 / 808 |
| E/strategist,w.energy_cap=2,w.energy_idle=0.05 | Researcher | 192 | 1.73 | 9 | 251 / 1527 / 1768 |
| E/strategist,w.energy_idle=0.05 | Builder | 192 | 2.79 | 59 | 224 / 636 / 1120 |
| E/strategist,w.energy_idle=0.05 | Protector | 192 | 1.84 | 17 | 216 / 720 / 1638 |
| E/strategist,w.energy_idle=0.05 | Researcher | 192 | 1.91 | 8 | 257 / 833 / 1441 |
| r51/greedy | Builder | 192 | 0.56 | 6 | 129 / 353 / 632 |
| r51/greedy | Protector | 192 | 0.78 | 3 | 143 / 444 / 530 |
| r51/greedy | Researcher | 192 | 1.62 | 1 | 123 / 285 / 332 |
| r51/strategist | Builder | 192 | 3.51 | 62 | 222 / 716 / 1017 |
| r51/strategist | Protector | 192 | 2.62 | 18 | 213 / 761 / 927 |
| r51/strategist | Researcher | 192 | 2.61 | 18 | 247 / 1075 / 1327 |
| r52/greedy | Builder | 192 | 0.68 | 10 | 131 / 344 / 509 |
| r52/greedy | Protector | 192 | 0.84 | 1 | 143 / 343 / 422 |
| r52/greedy | Researcher | 192 | 1.34 | 1 | 116 / 279 / 287 |
| r52/strategist | Builder | 192 | 2.91 | 61 | 234 / 1120 / 1164 |
| r52/strategist | Protector | 192 | 2.02 | 20 | 220 / 915 / 1546 |
| r52/strategist | Researcher | 192 | 1.92 | 9 | 262 / 1053 / 1069 |

## Cards per board: drafted / kept / played / Action uses, a seat-game; uses per build

| run | board | demand_response_grid | citywide_microgrid | particle_accelerator |
|---|---|---|---|---|
| E/strategist,w.energy_cap=2 | Builder | 0.08/0.10/0.06/0.07 (1.3) | 0.34/0.08/0.02/0.03 (1.7) | 0.05/0.08/0.11/0.04 (0.4) |
| E/strategist,w.energy_cap=2 | Protector | 0.10/0.17/0.07/0.11 (1.6) | 0.34/0.29/0.11/0.04 (0.4) | 0.06/0.11/0.15/0.08 (0.6) |
| E/strategist,w.energy_cap=2 | Researcher | 0.06/0.16/0.05/0.05 (1.0) | 0.08/0.04/0.01/0.01 (1.0) | 0.08/0.15/0.08/0.02 (0.2) |
| E/strategist,w.energy_cap=2,w.energy_idle=0.05 | Builder | 0.08/0.11/0.09/0.12 (1.4) | 0.35/0.07/0.02/0.03 (1.7) | 0.05/0.09/0.12/0.05 (0.4) |
| E/strategist,w.energy_cap=2,w.energy_idle=0.05 | Protector | 0.09/0.15/0.07/0.08 (1.2) | 0.34/0.29/0.11/0.05 (0.4) | 0.06/0.11/0.14/0.06 (0.4) |
| E/strategist,w.energy_cap=2,w.energy_idle=0.05 | Researcher | 0.05/0.16/0.08/0.06 (0.8) | 0.08/0.04/0.01/0.01 (1.0) | 0.08/0.15/0.09/0.02 (0.2) |
| E/strategist,w.energy_idle=0.05 | Builder | 0.12/0.14/0.09/0.15 (1.6) | 0.36/0.09/0.04/0.03 (0.8) | 0.06/0.10/0.11/0.04 (0.3) |
| E/strategist,w.energy_idle=0.05 | Protector | 0.04/0.12/0.05/0.09 (1.8) | 0.36/0.31/0.11/0.04 (0.4) | 0.08/0.11/0.11/0.05 (0.4) |
| E/strategist,w.energy_idle=0.05 | Researcher | 0.09/0.16/0.07/0.05 (0.8) | 0.06/0.03/0.00/0 | 0.07/0.14/0.09/0.01 (0.1) |
| r51/greedy | Builder | 0.03/0.04/0.01/0.01 (2.0) | 0.23/0.02/0.01/0.02 (1.5) | 0.04/0.10/0.00/0 |
| r51/greedy | Protector | 0.04/0.10/0.01/0.01 (2.0) | 0.20/0.14/0.02/0.01 (0.3) | 0.04/0.10/0.01/0.01 (1.0) |
| r51/greedy | Researcher | 0.05/0.15/0.01/0.01 (2.0) | 0.29/0.11/0.01/0.02 (1.5) | 0.04/0.11/0.01/0.00 (0.0) |
| r51/strategist | Builder | 0.11/0.11/0.04/0.07 (2.0) | 0.40/0.11/0.07/0.04 (0.6) | 0.07/0.12/0.15/0.04 (0.3) |
| r51/strategist | Protector | 0.06/0.09/0.04/0.05 (1.4) | 0.34/0.30/0.08/0.05 (0.6) | 0.05/0.10/0.12/0.06 (0.5) |
| r51/strategist | Researcher | 0.06/0.16/0.04/0.03 (0.6) | 0.05/0.03/0.00/0 | 0.08/0.14/0.17/0.03 (0.2) |
| r52/greedy | Builder | 0.05/0.06/0.00/0 | 0.28/0.02/0.00/0 | 0.06/0.09/0.00/0 |
| r52/greedy | Protector | 0.07/0.11/0.01/0.01 (1.0) | 0.25/0.15/0.02/0.03 (1.5) | 0.04/0.09/0.01/0.01 (1.0) |
| r52/greedy | Researcher | 0.05/0.14/0.00/0 | 0.19/0.08/0.02/0.03 (1.5) | 0.02/0.09/0.01/0.00 (0.0) |
| r52/strategist | Builder | 0.12/0.14/0.07/0.11 (1.5) | 0.36/0.10/0.04/0.03 (0.6) | 0.07/0.11/0.11/0.04 (0.3) |
| r52/strategist | Protector | 0.05/0.12/0.04/0.08 (2.0) | 0.36/0.31/0.11/0.05 (0.4) | 0.07/0.10/0.11/0.05 (0.4) |
| r52/strategist | Researcher | 0.08/0.16/0.05/0.03 (0.6) | 0.06/0.03/0.00/0 | 0.08/0.15/0.09/0.01 (0.1) |

## Cards, all boards: kept / played a game (both seats),

| card | E/strategist,w.energy_cap=2 | E/strategist,w.energy_cap=2,w.energy_idle=0.05 | E/strategist,w.energy_idle=0.05 | r51/greedy | r51/strategist | r52/greedy | r52/strategist |
|---|---|---|---|---|---|---|---|
| demand_response_grid | 0.29 / 0.12 / 0.16 uses | 0.28 / 0.16 / 0.18 uses | 0.28 / 0.14 / 0.20 uses | 0.19 / 0.01 / 0.02 uses | 0.24 / 0.08 / 0.10 uses | 0.20 / 0.00 / 0.00 uses | 0.28 / 0.11 / 0.15 uses |
| citywide_microgrid | 0.27 / 0.09 / 0.05 uses | 0.27 / 0.09 / 0.05 uses | 0.29 / 0.10 / 0.05 uses | 0.18 / 0.02 / 0.02 uses | 0.30 / 0.10 / 0.06 uses | 0.17 / 0.03 / 0.04 uses | 0.30 / 0.10 / 0.05 uses |
| particle_accelerator | 0.23 / 0.23 / 0.09 uses | 0.23 / 0.23 / 0.08 uses | 0.23 / 0.21 / 0.06 uses | 0.21 / 0.01 / 0.00 uses | 0.24 / 0.29 / 0.09 uses | 0.18 / 0.01 / 0.00 uses | 0.24 / 0.20 / 0.06 uses |
