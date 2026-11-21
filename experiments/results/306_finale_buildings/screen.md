# 306: the screen (seeds 51264-51359, 303's spent fresh set, 1,152 games an arm, paired)

r148 is main at 065566d7 (it reproduces 303's kept r148 read on these seeds exactly). Arms on r149's first deck
(Siegebreaker Reactor needing 3 Security): `base`; `eased` (Siegebreaker, Fusion Torch, Singularity need 2,
Concord 8 credits); `idleoff` (the judge with w.am_cap=0); `am_act` (a keep lever, reverted); `pair`
(Siegebreaker needs 3 Power, Fusion Torch 3 Industry); `up` (each finale gains 1 antimatter when built); `both`;
`final`, r149 as shipped (Siegebreaker 3 Power, Singularity gaining 2 insight).

spent a seat-game: 1.26 -> 1.29, paired gap +0.03 +- 0.04 (1152 games)
made a seat-game: 2.71 -> 2.49, paired gap -0.22 +- 0.08 (1152 games)

Pooled with the fresh read (2,304 games, r148 against r149 as shipped):

spent a seat-game: 1.27 -> 1.38, paired gap +0.11 +- 0.03 (2304 games)
made a seat-game: 2.64 -> 2.66, paired gap +0.02 +- 0.06 (2304 games)

| agent | arm | d0 | d1 | d2 | d3 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|
| strategist | r148/base | 94.1 | 91.3 | 86.5 | 68.8 | 85.2 |  | 1152 |
| strategist | base/base | 96.9 | 94.8 | 87.2 | 73.3 | 88.0 | +2.9 ± 1.3 | 1152 |
| strategist | eased/base | 95.5 | 94.8 | 88.9 | 76.0 | 88.8 | +3.6 ± 1.3 | 1152 |
| strategist | idleoff/base | 96.9 | 93.1 | 87.8 | 72.6 | 87.6 | +2.4 ± 1.3 | 1152 |
| strategist | am_act/base | 96.9 | 94.4 | 87.5 | 73.3 | 88.0 | +2.9 ± 1.3 | 1152 |
| strategist | pair/base | 96.5 | 92.0 | 88.2 | 74.0 | 87.7 | +2.5 ± 1.3 | 1152 |
| strategist | up/base | 97.6 | 95.1 | 87.5 | 74.7 | 88.7 | +3.6 ± 1.3 | 1152 |
| strategist | both/base | 97.2 | 94.1 | 87.5 | 75.3 | 88.5 | +3.4 ± 1.3 | 1152 |
| strategist | final/base | 96.9 | 94.8 | 87.5 | 72.2 | 87.8 | +2.7 ± 1.3 | 1152 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|
| strategist | base/base | +2.8 ± 1.7 | +3.5 ± 2.1 | +0.7 ± 2.6 | +4.5 ± 3.5 |
| strategist | eased/base | +1.4 ± 1.8 | +3.5 ± 2.1 | +2.4 ± 2.6 | +7.3 ± 3.3 |
| strategist | idleoff/base | +2.8 ± 1.7 | +1.7 ± 2.2 | +1.4 ± 2.6 | +3.8 ± 3.6 |
| strategist | am_act/base | +2.8 ± 1.7 | +3.1 ± 2.1 | +1.0 ± 2.6 | +4.5 ± 3.5 |
| strategist | pair/base | +2.4 ± 1.7 | +0.7 ± 2.3 | +1.7 ± 2.6 | +5.2 ± 3.4 |
| strategist | up/base | +3.5 ± 1.6 | +3.8 ± 2.0 | +1.0 ± 2.7 | +5.9 ± 3.4 |
| strategist | both/base | +3.1 ± 1.7 | +2.8 ± 2.1 | +1.0 ± 2.7 | +6.6 ± 3.4 |

| Read | d | games | win % | antimatter gained / spent a seat-game | spent ÷ gained |
| r148 | all | 1152 | | 2.71 / 1.26 | 46% |
| base | all | 1152 | | 2.49 / 1.29 | 52% |
| eased | all | 1152 | | 2.44 / 1.31 | 53% |
| idleoff | all | 1152 | | 2.86 / 1.23 | 43% |
| am_act | all | 1152 | | 2.49 / 1.31 | 53% |
| pair | all | 1152 | | 2.50 / 1.32 | 53% |
| up | all | 1152 | | 2.64 / 1.39 | 53% |
| both | all | 1152 | | 2.69 / 1.43 | 53% |
| final | all | 1152 | | 2.49 / 1.29 | 52% |

| Card | Read | offered / kept / played / kept-never-played per 100 games | Action uses per 100 games | uses a seat that built it | first built: mean round, by r3 | first use: mean round |
|---|---|---|---|---|---|---|
| siegebreaker_reactor | base | 39.3 / 15.8 / 11.8 / 5.2 | 8.6 | 0.73 | 5.4, 19% | 6.0 |
| siegebreaker_reactor | eased | 38.7 / 23.1 / 20.4 / 4.8 | 15.6 | 0.77 | 5.0, 23% | 5.8 |
| siegebreaker_reactor | pair | 39.5 / 17.3 / 15.0 / 4.3 | 11.7 | 0.78 | 5.4, 13% | 5.7 |
| siegebreaker_reactor | up | 39.1 / 15.5 / 11.6 / 5.2 | 13.6 | 1.19 | 5.3, 19% | 5.2 |
| siegebreaker_reactor | final | 38.7 / 16.4 / 14.9 / 3.6 | 11.3 | 0.76 | 5.5, 13% | 5.7 |
| concord_reactor | base | 32.6 / 17.3 / 16.2 / 3.4 | 19.6 | 1.23 | 5.1, 27% | 6.2 |
| concord_reactor | eased | 31.9 / 17.0 / 15.9 / 3.5 | 17.7 | 1.12 | 5.0, 30% | 6.1 |
| concord_reactor | pair | 32.7 / 17.8 / 16.1 / 4.2 | 19.4 | 1.21 | 5.2, 27% | 6.2 |
| concord_reactor | up | 31.9 / 17.4 / 16.5 / 3.4 | 25.9 | 1.58 | 5.2, 26% | 5.2 |
| concord_reactor | final | 32.6 / 17.1 / 16.1 / 3.6 | 19.6 | 1.24 | 5.1, 27% | 6.2 |
| fusion_torch_array | base | 34.4 / 16.0 / 12.1 / 5.3 | 12.9 | 1.08 | 5.7, 12% | 6.1 |
| fusion_torch_array | eased | 33.2 / 20.7 / 18.4 / 4.3 | 19.4 | 1.05 | 5.3, 19% | 5.8 |
| fusion_torch_array | pair | 34.2 / 12.8 / 9.0 / 5.6 | 9.7 | 1.08 | 5.1, 30% | 5.8 |
| fusion_torch_array | up | 33.4 / 15.4 / 12.1 / 4.9 | 12.5 | 1.04 | 5.6, 13% | 5.8 |
| fusion_torch_array | final | 34.1 / 15.5 / 11.5 / 5.3 | 11.7 | 1.02 | 5.7, 12% | 6.0 |
| singularity_exploit | base | 36.5 / 16.8 / 12.3 / 4.5 | 11.0 | 0.89 | 5.6, 6% | 6.3 |
| singularity_exploit | eased | 36.1 / 22.5 / 17.4 / 5.2 | 16.6 | 0.95 | 5.1, 13% | 6.0 |
| singularity_exploit | pair | 36.4 / 16.6 / 11.8 / 4.9 | 11.5 | 0.98 | 5.5, 7% | 6.3 |
| singularity_exploit | up | 35.9 / 15.8 / 12.0 / 3.9 | 15.1 | 1.26 | 5.6, 7% | 5.5 |
| singularity_exploit | final | 36.5 / 16.7 / 11.8 / 4.9 | 9.3 | 0.79 | 5.5, 7% | 6.3 |
