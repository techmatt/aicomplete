# 303: the fresh read

The judge, `read.json` (two seats, the three pairs, d0-d3), fresh seeds 51264-51359, 288 games a level: r147
is main at 2c78093c farmed with `--at`; r148 is this item's build.

| Read | Level | Games | Wins | Rounds | 6-8 | 10+ | Cap losses of losses | Won by r3 | <=5 | 6 | 7 | 8 | 9 | 10 | 11 | 12+ |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| r147 | d0 | 288 | 91% | 7.1 | 52% | 13% (miss) | 25 of 25 | 1 | 25 | 18 | 19 | 15 | 10 | 5 | 9 | 0 |
| r147 | d1 | 288 | 90% | 7.2 | 55% | 14% (miss) | 29 of 29 | 3 | 22 | 19 | 16 | 19 | 9 | 6 | 9 | 0 |
| r147 | d2 | 288 | 84% | 7.3 | 48% (miss) | 17% (miss) | 31 of 47 | 6 | 22 | 18 | 16 | 13 | 14 | 8 | 9 | 0 |
| r147 | d3 | 288 | 67% | 7.3 | 56% | 14% (miss) | 61 of 96 | 7 | 17 | 16 | 22 | 19 | 13 | 14 | 0 | 0 |
| r148 | d0 | 288 | 94% | 6.8 | 60% | 10% | 17 of 17 | 2 | 23 | 29 | 18 | 13 | 7 | 4 | 6 | 0 |
| r148 | d1 | 288 | 91% | 6.9 | 52% | 10% (miss) | 24 of 25 | 8 | 28 | 20 | 16 | 15 | 10 | 3 | 8 | 0 |
| r148 | d2 | 288 | 86% | 7.1 | 49% (miss) | 12% (miss) | 23 of 39 | 4 | 25 | 17 | 17 | 16 | 14 | 6 | 6 | 0 |
| r148 | d3 | 288 | 69% | 7.4 | 51% | 16% (miss) | 65 of 90 | 3 | 17 | 17 | 21 | 14 | 16 | 16 | 0 | 0 |

## Wins, paired (bench/read_lib.py)

| agent | arm | d0 | d1 | d2 | d3 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|
| strategist | r147/base | 91.3 | 89.9 | 83.7 | 66.7 | 82.9 |  | 1152 |
| strategist | r148/base | 94.1 | 91.3 | 86.5 | 68.8 | 85.2 | +2.3 ± 1.4 | 1152 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|
| strategist | r148/base | +2.8 ± 2.1 | +1.4 ± 2.3 | +2.8 ± 2.8 | +2.1 ± 3.5 |


## Antimatter: the sinks per 100 games, and spent ÷ gained (am_cards.py)

| Sink, per 100 games | r147 offered / kept / played | r148 offered / kept / played |
|---|---|---|
| recursive_self_improvement | 45.2 / 14.7 / 10.4 | 36.5 / 15.2 / 10.9 |
| orbital_mass_driver | 34.2 / 9.6 / 5.9 | 36.8 / 11.0 / 5.8 |
| singularity_exploit | 38.3 / 9.1 / 6.2 | 36.8 / 9.8 / 8.5 |
| dyson_swarm_tranche | 48.0 / 11.8 / 4.1 | 26.7 / 7.4 / 3.3 |
| great_liberation | 46.8 / 12.4 / 6.2 | 40.9 / 10.7 / 4.5 |
| annihilation_lance | 39.5 / 0.0 / 0.8 | 38.3 / 4.6 / 3.1 |
| fusion_commons | 38.9 / 17.5 / 15.5 | 39.8 / 18.1 / 15.4 |
| nuclear_option | 43.9 / 23.3 / 18.6 | 32.6 / 18.0 / 14.9 |
| hostile_buyout | 38.0 / 21.3 / 21.3 | 36.4 / 18.0 / 15.0 |
| root_access | 34.8 / 17.2 / 15.9 | 45.5 / 19.3 / 16.0 |
| peacekeeping_mission | 40.9 / 18.5 / 15.9 | 35.5 / 13.9 / 11.6 |
| decapitation_strike | 33.8 / 15.3 / 8.5 | 36.0 / 14.3 / 7.4 |
| collider_consortium | 38.7 / 19.9 / 15.0 | 34.5 / 19.0 / 11.3 |
| catalyzed_fab_run | 31.6 / 19.0 / 19.2 | 35.4 / 19.2 / 13.4 |
| antimatter_accord | 29.4 / 19.2 / 15.1 | 34.5 / 17.9 / 13.2 |
| accelerator_diplomacy | 28.0 / 12.2 / 20.7 | 27.5 / 11.8 / 17.5 |
| beamline_liberation | 0.0 / 0.0 / 0.0 | 40.4 / 21.4 / 17.9 |
| reactor_retooling | 0.0 / 0.0 / 0.0 | 40.2 / 21.5 / 18.2 |
| clearance_strike | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 |

| Read | d | games | win % | antimatter gained / spent a seat-game | spent ÷ gained |
|---|---|---|---|---|---|
| r147 | d0 | 288 | 91.3 | 3.55 / 1.16 | 33% |
| r147 | d1 | 288 | 89.9 | 3.53 / 1.21 | 34% |
| r147 | d2 | 288 | 83.7 | 3.38 / 1.25 | 37% |
| r147 | d3 | 288 | 66.7 | 3.01 / 1.02 | 34% |
| r147 | all | 1152 | | 3.37 / 1.16 | 35% |
| r148 | d0 | 288 | 94.1 | 2.63 / 1.21 | 46% |
| r148 | d1 | 288 | 91.3 | 2.77 / 1.26 | 45% |
| r148 | d2 | 288 | 86.5 | 2.79 / 1.29 | 46% |
| r148 | d3 | 288 | 68.8 | 2.65 / 1.28 | 48% |
| r148 | all | 1152 | | 2.71 / 1.26 | 46% |

Unspent antimatter at the end of each round (mean over seats reaching it); seat-games ending with any held:
- r147: r1 0.04 r2 0.12 r3 0.29 r4 0.63 r5 1.17 r6 1.74 r7 2.50 r8 3.07 r9 3.81 r10 5.03; held at the end 60%
- r148: r1 0.03 r2 0.09 r3 0.24 r4 0.54 r5 0.93 r6 1.31 r7 1.70 r8 2.10 r9 2.66 r10 3.78; held at the end 55%

## Antimatter by use and source, per 100 games (am_spend.py)

## r147: 1152 games, 7.20 rounds; spent 232.6, made 673.9 per 100 games
- spent: own:play:hostile_buyout 21.3, own:play:accelerator_diplomacy 20.7, own:play:catalyzed_fab_run 19.2, own:play:nuclear_option 18.6, own:play:peacekeeping_mission 15.9, own:play:root_access 15.9, own:play:fusion_commons 15.5, own:play:antimatter_accord 15.1, own:play:collider_consortium 15.0, own:use:containment_reactor 14.8, own:use:kinetic_interdiction 12.8, own:play:recursive_self_improvement 10.4, own:play:decapitation_strike 8.5, own:play:singularity_exploit 6.2, own:play:great_liberation 6.2, own:play:orbital_mass_driver 5.9, own:play:dyson_swarm_tranche 4.1, phase:adversary.compute 3.0, own:use:orbital_relay_launcher 2.7, own:play:annihilation_lance 0.8
- made: own:use:treaty_isotope_bank 112.2, own:use:dark_fiber_collider 99.8, own:power:Field Survey 67.0, own:use:penning_trap_array 66.2, own:use:antiproton_foundry 52.8, own:use:cyclotron_lease 50.5, own:power:Levy 46.0, own:use:positron_storage_ring 35.8, own:power:Accord 26.4, own:power:Overcharge II 20.6, own:use:survey_drone_fleet 16.0, own:power:Field Station 15.1, own:play:zero_day_auction 12.0, own:use:simulation_cluster 11.2, own:power:Outpost 10.1, own:power:Peer Review 7.6, own:standard:antimatter 5.6, own:play:threat_hunt 2.0, own:play:twin_city_compact 1.9, own:power:Hack II 1.6
## r148: 1152 games, 7.04 rounds; spent 251.5, made 542.1 per 100 games
- spent: own:play:reactor_retooling 18.2, own:play:beamline_liberation 17.9, own:play:accelerator_diplomacy 17.5, own:use:kinetic_interdiction 16.4, own:play:root_access 16.0, own:play:fusion_commons 15.4, own:play:hostile_buyout 15.0, own:play:nuclear_option 14.9, own:use:containment_reactor 14.8, own:play:catalyzed_fab_run 13.4, own:play:antimatter_accord 13.2, own:play:peacekeeping_mission 11.6, own:play:collider_consortium 11.3, own:play:recursive_self_improvement 10.9, own:use:orbital_relay_launcher 9.5, own:play:singularity_exploit 8.5, own:play:decapitation_strike 7.4, own:play:orbital_mass_driver 5.8, own:play:great_liberation 4.5, own:play:dyson_swarm_tranche 3.3
- made: own:use:dark_fiber_collider 85.1, own:use:treaty_isotope_bank 66.1, own:use:penning_trap_array 60.5, own:power:Field Survey 55.6, own:use:cyclotron_lease 45.1, own:power:Levy 40.3, own:power:Accord 26.9, own:use:antiproton_foundry 23.0, own:use:positron_storage_ring 21.8, own:power:Field Station 20.1, own:power:Overcharge II 16.6, own:use:survey_drone_fleet 15.8, own:power:Peer Review 12.5, own:play:zero_day_auction 9.7, own:use:simulation_cluster 9.2, own:power:Outpost 9.0, own:standard:antimatter 7.3, own:play:beamed_power_constellation 2.3, own:play:relay_mast_rollout 1.9, own:play:twin_city_compact 1.7
