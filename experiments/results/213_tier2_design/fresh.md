# 213 fresh read (seeds 46336-46431, 96 seeds x 3 pairs x d0-d4)

Arms: base (r94 as shipped), bt (Breakthrough back on), not2 (tier 2 off: experiments/configs/no_tier2.json), r93 (6f18850's binary and tree on the same seeds), f212 (212's fresh read, seeds 46240-46335: other seeds). Reader: read213.py.

## Win rate (pairs pooled), paired against base

| arm | d0 | d1 | d2 | d3 | d4 | rounds |
|---|---|---|---|---|---|---|
| base (1440) | 84.4 | 63.5 | 48.3 | 34.4 | 15.6 | 7.28 |
| r93 (1440) | 92.0 (+7.6 ± 2.7) | 71.9 (+8.3 ± 3.8) | 53.8 (+5.6 ± 3.9) | 43.8 (+9.4 ± 3.7) | 19.8 (+4.2 ± 2.9) | 6.94 |
| bt (1440) | 88.9 (+4.5 ± 2.3) | 64.2 (+0.7 ± 2.6) | 53.1 (+4.9 ± 2.3) | 39.6 (+5.2 ± 2.3) | 17.4 (+1.7 ± 1.8) | 7.08 |
| not2 (1440) | 49.0 (-35.4 ± 3.6) | 37.5 (-26.0 ± 3.6) | 26.0 (-22.2 ± 3.6) | 16.3 (-18.1 ± 3.2) | 5.6 (-10.1 ± 2.3) | 7.99 |
| f212 (1440) | 86.8 | 69.1 | 56.6 | 44.4 | 17.7 | 7.09 |

## Tier 2 and antimatter, a game (seat-game for resources)

| arm | tier-2 keeps r1-4 | tier-2 plays | am gained / spent a seat | won games spending am | top am sources | top am spends |
|---|---|---|---|---|---|---|
| base | 4.83 | 9.22 | 2.90 / 1.27 | 91% of 709 | standard:antimatter 31%, phase:economy.upkeep 23%, convert:0 11%, use:penning_trap_array 8% | play:accelerator_diplomacy 16%, play:antimatter_accord 10%, play:catalyzed_fab_run 9%, play:annihilation_lance 9% |
| r93 | 4.68 | 8.57 | 4.50 / 2.59 | 95% of 810 | convert:0 69%, phase:economy.upkeep 14%, standard:antimatter 7%, use:penning_trap_array 3% | standard:breakthrough 85%, play:fusion_commons 4%, play:annihilation_lance 3%, use:containment_reactor 3% |
| bt | 4.83 | 8.56 | 2.78 / 2.21 | 96% of 758 | standard:antimatter 31%, phase:economy.upkeep 22%, convert:0 10%, use:penning_trap_array 7% | standard:breakthrough 54%, play:accelerator_diplomacy 8%, play:antimatter_accord 5%, play:catalyzed_fab_run 5% |
| not2 | 0.00 | 0.00 | 2.83 / 0.46 | 79% of 387 | standard:antimatter 38%, phase:economy.upkeep 30%, convert:0 13%, use:cyclotron_lease 9% | play:accelerator_diplomacy 66%, use:containment_reactor 33%, use:simulation_cluster 1% |
| f212 | 4.28 | 8.75 | 4.97 / 2.95 | 95% of 791 | convert:0 68%, phase:economy.upkeep 13%, standard:antimatter 8%, use:penning_trap_array 4% | standard:breakthrough 82%, play:annihilation_lance 4%, play:fusion_commons 4%, use:containment_reactor 4% |

## Touched cards, plays a game

| card | base | r93 | bt | not2 | f212 |
|---|---|---|---|---|---|
| Dyson Swarm Tranche | 0.09 | 0.01 | 0.09 | 0.00 | 0.01 |
| Recursive Self-Improvement | 0.11 | 0.04 | 0.10 | 0.00 | 0.03 |
| Peacekeeping Mission | 0.07 | 0.01 | 0.06 | 0.00 | 0.02 |
| Root Access | 0.09 | 0.03 | 0.06 | 0.00 | 0.03 |
| Hostile Buyout | 0.21 | 0.05 | 0.17 | 0.00 | 0.04 |
| Decapitation Strike | 0.11 | 0.02 | 0.09 | 0.00 | 0.02 |
| Collider Consortium | 0.16 | 0.00 | 0.14 | 0.00 | 0.00 |
| Catalyzed Fab Run | 0.23 | 0.00 | 0.20 | 0.00 | 0.00 |
| Antimatter Accord | 0.24 | 0.00 | 0.21 | 0.00 | 0.00 |
| Cyclotron Lease | 0.10 | 0.00 | 0.10 | 0.12 | 0.00 |
| Accelerator Diplomacy | 0.40 | 0.00 | 0.34 | 0.61 | 0.00 |
| The Nuclear Option | 0.10 | 0.14 | 0.08 | 0.00 | 0.13 |
| Annihilation Lance | 0.11 | 0.09 | 0.08 | 0.00 | 0.13 |
| Fusion Commons | 0.11 | 0.10 | 0.08 | 0.00 | 0.12 |
| Kinetic Interdiction | 0.06 | 0.06 | 0.05 | 0.00 | 0.06 |
| Great Liberation | 0.02 | 0.04 | 0.01 | 0.00 | 0.03 |
| Singularity Exploit | 0.07 | 0.05 | 0.05 | 0.00 | 0.08 |
| Penning Trap Array | 0.21 | 0.17 | 0.19 | 0.00 | 0.17 |
| Antiproton Foundry | 0.23 | 0.22 | 0.23 | 0.00 | 0.26 |
| Positron Storage Ring | 0.08 | 0.11 | 0.08 | 0.00 | 0.12 |
| Zero-Day Auction | 0.05 | 0.03 | 0.05 | 0.00 | 0.03 |

## Keeps by round, a game (tier 2 / tier 1) and tier-2 draft picks

```
base kept_t2 r1-5 0.69 0.98 1.29 1.87 1.99 | kept_t1 2.99 2.37 1.99 1.66 1.44 | tier-2 picks 27.4 of 54.7
r93 kept_t2 r1-5 0.68 1.01 1.27 1.73 2.01 | kept_t1 2.92 2.16 1.84 1.49 1.37 | tier-2 picks 28.6 of 52.5
bt kept_t2 r1-5 0.69 0.97 1.29 1.88 2.02 | kept_t1 2.99 2.37 1.99 1.64 1.47 | tier-2 picks 26.4 of 53.2
f212 kept_t2 r1-5 0.74 0.86 1.06 1.62 1.95 | kept_t1 2.92 2.31 1.97 1.58 1.38 | tier-2 picks 28.9 of 53.4
```
