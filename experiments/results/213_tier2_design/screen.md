# 213 screen 1 (dev seeds 36800-36831, 32 seeds x 3 pairs x d0-d4; cards and powers, Breakthrough off; Tidal Grid as r93)

Arms: base (r94 first build), bt (Breakthrough back on), tg (Tidal Grid converter 2 cubes + 1 energy -> 1), not2 (tier 2 off; opening 2 tier-1 cards only), r93 (6f18850). Reader: read213.py.

## Win rate (pairs pooled), paired against base

| arm | d0 | d1 | d2 | d3 | d4 | rounds |
|---|---|---|---|---|---|---|
| base (480) | 85.4 | 69.8 | 59.4 | 47.9 | 25.0 | 7.45 |
| bt (480) | 91.7 (+6.2 ± 3.6) | 75.0 (+5.2 ± 5.0) | 71.9 (+12.5 ± 4.5) | 59.4 (+11.5 ± 4.7) | 31.2 (+6.2 ± 3.6) | 6.96 |
| tg (480) | 85.4 (+0.0 ± 4.2) | 64.6 (-5.2 ± 3.7) | 52.1 (-7.3 ± 4.5) | 42.7 (-5.2 ± 4.0) | 24.0 (-1.0 ± 3.5) | 7.62 |
| not2 (480) | 56.2 (-29.2 ± 5.9) | 43.8 (-26.0 ± 6.5) | 27.1 (-32.3 ± 6.0) | 18.8 (-29.2 ± 6.3) | 11.5 (-13.5 ± 5.0) | 8.16 |
| r93 (480) | 91.7 (+6.2 ± 4.4) | 70.8 (+1.0 ± 6.0) | 60.4 (+1.0 ± 6.9) | 43.8 (-4.2 ± 6.1) | 19.8 (-5.2 ± 5.2) | 7.04 |
| not2bt (192) | 71.9 (-13.5 ± 5.3) | 63.5 (-6.2 ± 6.1) | - | - | - | 8.17 |

## Tier 2 and antimatter, a game (seat-game for resources)

| arm | tier-2 keeps r1-4 | tier-2 plays | am gained / spent a seat | won games spending am | top am sources | top am spends |
|---|---|---|---|---|---|---|
| base | 4.92 | 9.93 | 5.71 / 1.57 | 90% of 276 | convert:0 65%, phase:economy.upkeep 12%, standard:antimatter 11%, use:cyclotron_lease 4% | play:accelerator_diplomacy 14%, play:antimatter_accord 12%, play:annihilation_lance 10%, play:fusion_commons 9% |
| bt | 4.96 | 8.71 | 4.89 / 3.32 | 97% of 316 | convert:0 65%, phase:economy.upkeep 11%, standard:antimatter 10%, use:cyclotron_lease 5% | standard:breakthrough 67%, play:accelerator_diplomacy 5%, play:antimatter_accord 4%, play:fusion_commons 3% |
| tg | 4.90 | 9.71 | 2.88 / 1.15 | 81% of 258 | standard:antimatter 37%, phase:economy.upkeep 25%, convert:0 12%, use:cyclotron_lease 9% | play:accelerator_diplomacy 16%, play:antimatter_accord 13%, play:collider_consortium 8%, play:hostile_buyout 8% |
| not2 | 0.00 | 0.00 | 6.64 / 0.55 | 77% of 151 | convert:0 74%, phase:economy.upkeep 13%, standard:antimatter 8%, use:cyclotron_lease 2% | play:accelerator_diplomacy 55%, use:containment_reactor 42%, use:simulation_cluster 2% |
| r93 | 4.84 | 8.57 | 4.67 / 2.69 | 94% of 275 | convert:0 68%, phase:economy.upkeep 13%, standard:antimatter 7%, use:penning_trap_array 5% | standard:breakthrough 85%, play:fusion_commons 4%, play:annihilation_lance 3%, use:containment_reactor 3% |
| not2bt | 0.00 | 0.00 | 6.83 / 3.93 | 98% of 130 | convert:0 73%, phase:economy.upkeep 12%, standard:antimatter 7%, use:cyclotron_lease 5% | standard:breakthrough 88%, play:accelerator_diplomacy 9%, use:containment_reactor 3%, use:simulation_cluster 0% |

## Touched cards, plays a game

| card | base | bt | tg | not2 | r93 | not2bt |
|---|---|---|---|---|---|---|
| Dyson Swarm Tranche | 0.05 | 0.05 | 0.06 | 0.00 | 0.04 | 0.00 |
| Recursive Self-Improvement | 0.14 | 0.09 | 0.11 | 0.00 | 0.02 | 0.00 |
| Peacekeeping Mission | 0.11 | 0.07 | 0.08 | 0.00 | 0.02 | 0.00 |
| Root Access | 0.11 | 0.09 | 0.09 | 0.00 | 0.02 | 0.00 |
| Hostile Buyout | 0.23 | 0.17 | 0.17 | 0.00 | 0.03 | 0.00 |
| Decapitation Strike | 0.11 | 0.07 | 0.09 | 0.00 | 0.00 | 0.00 |
| Collider Consortium | 0.17 | 0.14 | 0.19 | 0.00 | 0.00 | 0.00 |
| Catalyzed Fab Run | 0.23 | 0.18 | 0.17 | 0.00 | 0.00 | 0.00 |
| Antimatter Accord | 0.38 | 0.28 | 0.30 | 0.00 | 0.00 | 0.00 |
| Cyclotron Lease | 0.15 | 0.14 | 0.16 | 0.07 | 0.00 | 0.15 |
| Accelerator Diplomacy | 0.45 | 0.36 | 0.36 | 0.61 | 0.00 | 0.69 |
| The Nuclear Option | 0.12 | 0.12 | 0.10 | 0.00 | 0.12 | 0.00 |
| Annihilation Lance | 0.15 | 0.07 | 0.09 | 0.00 | 0.09 | 0.00 |
| Fusion Commons | 0.15 | 0.09 | 0.06 | 0.00 | 0.10 | 0.00 |
| Kinetic Interdiction | 0.08 | 0.06 | 0.04 | 0.00 | 0.04 | 0.00 |
| Great Liberation | 0.03 | 0.02 | 0.02 | 0.00 | 0.04 | 0.00 |
| Singularity Exploit | 0.08 | 0.06 | 0.08 | 0.00 | 0.07 | 0.00 |
| Penning Trap Array | 0.14 | 0.12 | 0.15 | 0.00 | 0.20 | 0.00 |
| Antiproton Foundry | 0.27 | 0.25 | 0.27 | 0.00 | 0.25 | 0.00 |
| Positron Storage Ring | 0.07 | 0.07 | 0.08 | 0.00 | 0.09 | 0.00 |
| Zero-Day Auction | 0.03 | 0.03 | 0.04 | 0.00 | 0.04 | 0.00 |
