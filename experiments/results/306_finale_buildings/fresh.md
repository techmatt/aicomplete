# 306: the fresh read

The judge, `read.json` (two seats, the three pairs, d0-d3), fresh seeds 51360-51455, 288 games a level: r148
is main at 065566d7 farmed with `--at`; r149 is this item's build.

| Read | Level | Games | Wins | Rounds | 6-8 | 10+ | Cap losses of losses | Won by r3 | <=5 | 6 | 7 | 8 | 9 | 10 | 11 | 12+ |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| r148 | d0 | 288 | 98% | 6.5 | 58% | 5% | 5 of 5 | 6 | 31 | 26 | 18 | 14 | 6 | 3 | 2 | 0 |
| r148 | d1 | 288 | 97% (miss) | 6.5 | 57% | 7% | 8 of 8 | 10 | 30 | 25 | 20 | 12 | 7 | 4 | 2 | 0 |
| r148 | d2 | 288 | 87% | 6.9 | 50% (miss) | 13% (miss) | 27 of 37 | 6 | 26 | 22 | 18 | 10 | 11 | 5 | 8 | 0 |
| r148 | d3 | 288 | 73% | 7.1 | 52% | 15% (miss) | 59 of 79 | 8 | 20 | 22 | 18 | 12 | 12 | 15 | 0 | 0 |
| r149 | d0 | 288 | 97% | 6.5 | 57% | 8% | 10 of 10 | 3 | 31 | 27 | 18 | 12 | 5 | 4 | 3 | 0 |
| r149 | d1 | 288 | 97% (miss) | 6.7 | 58% | 7% | 10 of 10 | 7 | 27 | 24 | 17 | 17 | 8 | 4 | 3 | 0 |
| r149 | d2 | 288 | 90% | 6.8 | 56% | 10% | 17 of 29 | 6 | 26 | 16 | 26 | 14 | 9 | 6 | 4 | 0 |
| r149 | d3 | 288 | 74% | 7.0 | 52% | 14% (miss) | 52 of 74 | 2 | 24 | 17 | 19 | 16 | 10 | 14 | 0 | 0 |

## Wins, paired (bench/read_lib.py)

| agent | arm | d0 | d1 | d2 | d3 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|
| strategist | r148/base | 98.3 | 97.2 | 87.2 | 72.6 | 88.8 |  | 1152 |
| strategist | r149/base | 96.5 | 96.5 | 89.9 | 74.3 | 89.3 | +0.5 ± 1.1 | 1152 |

### Paired gaps by level

| agent | arm | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|
| strategist | r149/base | -1.7 ± 1.2 | -0.7 ± 1.4 | +2.8 ± 2.6 | +1.7 ± 3.2 |


## The finale buildings and the older antimatter Actions (finale.py)

| Card | Read | offered / kept / played / kept-never-played per 100 games | Action uses per 100 games | uses a seat that built it | first built: mean round, by r3 | first use: mean round |
|---|---|---|---|---|---|---|
| siegebreaker_reactor | r148 | 0.0 / 0.0 / 0.0 / 0.0 | 0.0 | - | - | - |
| siegebreaker_reactor | r149 | 28.0 / 10.4 / 7.5 / 3.1 | 6.0 | 0.80 | 5.9, 3% | 6.0 |
| concord_reactor | r148 | 0.0 / 0.0 / 0.0 / 0.0 | 0.0 | - | - | - |
| concord_reactor | r149 | 35.1 / 19.0 / 17.0 / 4.0 | 22.7 | 1.34 | 5.0, 27% | 5.7 |
| fusion_torch_array | r148 | 0.0 / 0.0 / 0.0 / 0.0 | 0.0 | - | - | - |
| fusion_torch_array | r149 | 29.3 / 12.5 / 8.1 / 5.0 | 7.0 | 0.89 | 6.3, 4% | 6.5 |
| singularity_exploit | r148 | 30.8 / 8.4 / 8.1 / 3.3 | 0.0 | 0.00 | 6.0, 5% | - |
| singularity_exploit | r149 | 28.4 / 13.3 / 10.1 / 4.1 | 9.4 | 0.94 | 6.2, 15% | 6.5 |
| kinetic_interdiction | r148 | 35.2 / 18.5 / 15.8 / 4.5 | 13.2 | 0.83 | 4.7, 36% | 6.0 |
| kinetic_interdiction | r149 | 27.3 / 17.1 / 16.1 / 4.4 | 15.6 | 0.97 | 4.6, 31% | 5.7 |
| orbital_relay_launcher | r148 | 44.2 / 38.8 / 34.7 / 10.3 | 14.3 | 0.41 | 3.3, 52% | 5.1 |
| orbital_relay_launcher | r149 | 41.3 / 37.0 / 29.9 / 9.2 | 15.5 | 0.52 | 3.4, 55% | 5.1 |
| containment_reactor | r148 | 24.0 / 19.6 / 25.8 / 4.6 | 15.6 | 0.61 | 2.8, 68% | 4.4 |
| containment_reactor | r149 | 25.2 / 20.0 / 27.3 / 2.9 | 16.8 | 0.62 | 2.6, 71% | 4.3 |

Paired (am_paired.py):

spent a seat-game: 1.28 -> 1.48, paired gap +0.19 +- 0.04 (1152 games)
made a seat-game: 2.57 -> 2.82, paired gap +0.25 +- 0.08 (1152 games)

## Antimatter: the sinks per 100 games, and spent ÷ gained (303's am_cards.py)

| Sink, per 100 games | r148 offered / kept / played | r149 offered / kept / played |
|---|---|---|
| recursive_self_improvement | 36.5 / 15.3 / 9.2 | 37.5 / 15.8 / 9.7 |
| orbital_mass_driver | 38.2 / 9.6 / 4.8 | 31.7 / 10.2 / 4.5 |
| dyson_swarm_tranche | 37.2 / 8.7 / 4.7 | 33.3 / 12.4 / 4.3 |
| great_liberation | 38.7 / 12.2 / 5.0 | 38.8 / 9.5 / 4.0 |
| annihilation_lance | 33.9 / 3.0 / 1.1 | 38.6 / 3.8 / 1.3 |
| fusion_commons | 40.0 / 17.2 / 14.8 | 30.6 / 13.5 / 11.5 |
| nuclear_option | 41.6 / 23.1 / 21.4 | 33.5 / 20.7 / 18.7 |
| hostile_buyout | 36.5 / 22.4 / 20.1 | 33.7 / 22.0 / 19.1 |
| root_access | 43.6 / 15.8 / 12.5 | 37.4 / 18.8 / 14.8 |
| peacekeeping_mission | 45.6 / 18.1 / 12.2 | 34.4 / 11.1 / 9.0 |
| decapitation_strike | 38.3 / 13.9 / 7.7 | 30.4 / 15.1 / 10.4 |
| collider_consortium | 32.7 / 19.4 / 10.8 | 29.1 / 14.1 / 12.0 |
| catalyzed_fab_run | 39.4 / 17.0 / 12.1 | 34.3 / 19.3 / 14.5 |
| antimatter_accord | 34.4 / 16.8 / 14.1 | 33.7 / 20.1 / 17.7 |
| accelerator_diplomacy | 19.7 / 9.7 / 14.2 | 16.8 / 8.6 / 15.9 |
| beamline_liberation | 36.0 / 22.9 / 24.6 | 28.2 / 18.8 / 16.4 |
| reactor_retooling | 35.6 / 16.8 / 14.1 | 37.5 / 24.4 / 21.5 |
| clearance_strike | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 |

| Read | d | games | win % | antimatter gained / spent a seat-game | spent ÷ gained |
|---|---|---|---|---|---|
| r148 | d0 | 288 | 98.3 | 2.40 / 1.19 | 50% |
| r148 | d1 | 288 | 97.2 | 2.42 / 1.21 | 50% |
| r148 | d2 | 288 | 87.2 | 2.76 / 1.41 | 51% |
| r148 | d3 | 288 | 72.6 | 2.70 / 1.32 | 49% |
| r148 | all | 1152 | | 2.57 / 1.28 | 50% |
| r149 | d0 | 288 | 96.5 | 2.61 / 1.38 | 53% |
| r149 | d1 | 288 | 96.5 | 2.85 / 1.49 | 52% |
| r149 | d2 | 288 | 89.9 | 2.88 / 1.54 | 54% |
| r149 | d3 | 288 | 74.3 | 2.94 / 1.50 | 51% |
| r149 | all | 1152 | | 2.82 / 1.48 | 52% |

Unspent antimatter at the end of each round (mean over seats reaching it); seat-games ending with any held:
- r148: r1 0.05 r2 0.10 r3 0.25 r4 0.53 r5 0.93 r6 1.32 r7 1.66 r8 2.10 r9 2.50 r10 3.10; held at the end 54%
- r149: r1 0.06 r2 0.16 r3 0.32 r4 0.62 r5 0.98 r6 1.42 r7 1.71 r8 1.95 r9 2.21 r10 2.80; held at the end 55%

## Antimatter by use and source, per 100 games (303's am_spend.py)

## r148: 1152 games, 6.75 rounds; spent 256.5, made 513.8 per 100 games
- spent: own:play:beamline_liberation 24.6, own:play:nuclear_option 21.4, own:play:hostile_buyout 20.1, own:use:containment_reactor 15.6, own:play:fusion_commons 14.8, own:use:orbital_relay_launcher 14.3, own:play:accelerator_diplomacy 14.2, own:play:antimatter_accord 14.1, own:play:reactor_retooling 14.1, own:use:kinetic_interdiction 13.2, own:play:root_access 12.5, own:play:peacekeeping_mission 12.2, own:play:catalyzed_fab_run 12.1, own:play:collider_consortium 10.8, own:play:recursive_self_improvement 9.2, own:play:singularity_exploit 8.1, own:play:decapitation_strike 7.7, own:play:great_liberation 5.0, own:play:orbital_mass_driver 4.8, own:play:dyson_swarm_tranche 4.7, phase:adversary.compute 1.7, own:play:annihilation_lance 1.1, own:use:simulation_cluster 0.3
- made: own:use:dark_fiber_collider 71.3, own:use:treaty_isotope_bank 68.4, own:use:penning_trap_array 54.4, own:power:Field Survey 53.5, own:use:cyclotron_lease 52.7, own:power:Levy 40.3, own:power:Accord 24.7, own:use:antiproton_foundry 22.0, own:power:Field Station 19.3, own:use:positron_storage_ring 16.9, own:power:Overcharge II 13.8, own:use:survey_drone_fleet 13.0, own:power:Peer Review 10.6, own:play:zero_day_auction 9.2, own:use:simulation_cluster 8.1, own:power:Outpost 8.0, own:standard:antimatter 6.8, own:play:zero_knowledge_exploit 3.0, own:play:logic_bomb 2.5, own:power:Hack II 2.1, own:play:relay_mast_rollout 2.1, own:play:recruit_operatives 1.9, own:play:botnet_takedown 1.5, own:play:killswitch_directive 1.3
## r149: 1152 games, 6.76 rounds; spent 295.4, made 563.6 per 100 games
- spent: own:use:concord_reactor 22.7, own:play:reactor_retooling 21.5, own:play:hostile_buyout 19.1, own:play:nuclear_option 18.7, own:play:antimatter_accord 17.7, own:use:containment_reactor 16.8, own:play:beamline_liberation 16.4, own:play:accelerator_diplomacy 15.9, own:use:kinetic_interdiction 15.6, own:use:orbital_relay_launcher 15.5, own:play:root_access 14.8, own:play:catalyzed_fab_run 14.5, own:play:collider_consortium 12.0, own:play:fusion_commons 11.5, own:play:decapitation_strike 10.4, own:play:recursive_self_improvement 9.7, own:use:singularity_exploit 9.4, own:play:peacekeeping_mission 9.0, own:use:siegebreaker_reactor 6.0, own:play:orbital_mass_driver 4.5, own:play:dyson_swarm_tranche 4.3, own:play:great_liberation 4.0, phase:adversary.compute 3.0, own:play:annihilation_lance 1.3
- made: own:use:treaty_isotope_bank 97.6, own:use:dark_fiber_collider 72.9, own:use:penning_trap_array 63.5, own:use:cyclotron_lease 56.8, own:power:Field Survey 52.1, own:power:Levy 41.0, own:power:Accord 28.3, own:use:antiproton_foundry 24.5, own:power:Field Station 19.4, own:power:Peer Review 15.5, own:use:survey_drone_fleet 14.8, own:use:positron_storage_ring 13.3, own:power:Overcharge II 12.4, own:use:simulation_cluster 9.9, own:power:Outpost 8.3, own:standard:antimatter 6.6, own:play:zero_day_auction 6.6, own:play:relay_mast_rollout 2.1, own:play:twin_city_compact 2.1, own:power:Hack II 1.9, own:play:threat_hunt 1.6, own:play:zero_knowledge_exploit 1.6, own:play:recruit_operatives 1.6, own:play:killswitch_directive 1.3
