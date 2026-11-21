# 303: the dev screen (seeds 36800-36831, 384 games an arm)

`s303.json`: `nocards` farmed on b3134094 (the agent lever alone, r147's deck), `cards` on 71557a95 (the
three screened cards, Clearance Strike among them, and the lever before the diff review's fixes). Arms: the judge
off, and `w.am_cap=1` with `w.am_idle` 0.5 or 0.2.

| Sink, per 100 games | off offered / kept / played | idle05 offered / kept / played | idle02 offered / kept / played | cards-off offered / kept / played | cards-idle05 offered / kept / played | cards-idle02 offered / kept / played |
|---|---|---|---|---|---|---|
| recursive_self_improvement | 40.1 / 19.0 / 9.6 | 38.3 / 18.0 / 10.2 | 38.3 / 19.3 / 12.2 | 29.4 / 12.8 / 8.3 | 30.2 / 13.3 / 8.9 | 29.7 / 13.5 / 9.1 |
| orbital_mass_driver | 32.8 / 11.5 / 6.8 | 31.2 / 10.4 / 6.0 | 32.8 / 11.7 / 5.2 | 41.7 / 12.2 / 5.5 | 41.1 / 11.7 / 6.0 | 41.4 / 11.7 / 6.5 |
| singularity_exploit | 35.4 / 10.2 / 8.6 | 34.9 / 10.2 / 10.2 | 35.7 / 9.4 / 8.6 | 28.4 / 6.0 / 8.6 | 28.6 / 6.2 / 9.1 | 28.9 / 6.5 / 8.6 |
| dyson_swarm_tranche | 38.3 / 8.3 / 3.9 | 36.2 / 13.8 / 5.2 | 37.5 / 11.7 / 4.4 | 47.4 / 9.6 / 3.6 | 48.2 / 14.1 / 7.6 | 47.9 / 13.3 / 6.0 |
| great_liberation | 47.1 / 15.9 / 5.5 | 46.9 / 15.1 / 5.2 | 46.9 / 15.4 / 6.0 | 37.8 / 7.6 / 2.3 | 39.1 / 8.9 / 2.1 | 38.0 / 7.6 / 2.6 |
| annihilation_lance | 39.8 / 0.0 / 0.3 | 38.0 / 0.0 / 0.3 | 39.8 / 6.8 / 3.4 | 26.8 / 0.0 / 0.3 | 27.1 / 0.0 / 0.5 | 26.8 / 3.1 / 1.0 |
| fusion_commons | 38.5 / 18.0 / 14.8 | 38.5 / 17.7 / 14.6 | 39.3 / 18.5 / 14.8 | 29.2 / 10.9 / 10.2 | 29.9 / 11.5 / 10.4 | 29.4 / 11.2 / 10.2 |
| nuclear_option | 40.9 / 27.1 / 27.9 | 40.1 / 26.8 / 27.9 | 39.8 / 26.3 / 25.8 | 33.6 / 14.8 / 13.3 | 33.6 / 14.8 / 13.5 | 32.0 / 13.5 / 12.2 |
| hostile_buyout | 41.7 / 26.3 / 22.9 | 40.4 / 25.5 / 20.8 | 41.4 / 26.8 / 21.4 | 48.4 / 27.6 / 25.0 | 48.7 / 27.9 / 25.0 | 48.7 / 27.9 / 25.8 |
| root_access | 34.1 / 14.3 / 15.9 | 33.1 / 13.0 / 14.3 | 32.6 / 13.0 / 14.1 | 43.2 / 17.7 / 14.3 | 45.1 / 19.5 / 14.3 | 44.0 / 18.2 / 12.8 |
| peacekeeping_mission | 38.3 / 19.0 / 17.2 | 38.5 / 19.0 / 17.7 | 38.0 / 17.7 / 15.9 | 42.7 / 16.1 / 13.5 | 42.7 / 15.4 / 13.0 | 43.5 / 15.6 / 13.8 |
| decapitation_strike | 31.5 / 10.4 / 5.5 | 31.0 / 9.4 / 5.2 | 30.7 / 9.4 / 5.2 | 45.1 / 18.8 / 10.4 | 44.8 / 18.2 / 9.1 | 45.1 / 18.5 / 9.9 |
| collider_consortium | 46.4 / 19.5 / 10.2 | 45.8 / 19.0 / 9.9 | 45.6 / 19.0 / 9.9 | 39.8 / 23.2 / 14.1 | 39.8 / 22.7 / 12.5 | 39.1 / 22.4 / 12.2 |
| catalyzed_fab_run | 38.3 / 20.1 / 12.8 | 37.5 / 19.8 / 12.8 | 38.0 / 20.3 / 13.3 | 31.5 / 17.4 / 17.4 | 30.7 / 16.7 / 16.9 | 32.0 / 17.4 / 17.4 |
| antimatter_accord | 41.1 / 24.2 / 18.5 | 40.1 / 22.7 / 17.2 | 39.3 / 22.4 / 17.7 | 24.0 / 18.5 / 21.1 | 25.0 / 19.3 / 22.7 | 25.3 / 19.5 / 22.4 |
| accelerator_diplomacy | 23.2 / 8.9 / 13.3 | 24.2 / 10.2 / 13.8 | 22.9 / 8.6 / 14.3 | 20.8 / 10.2 / 11.5 | 21.1 / 10.4 / 12.0 | 21.6 / 10.7 / 12.0 |
| beamline_liberation | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 | 38.3 / 24.5 / 19.8 | 38.3 / 25.3 / 19.5 | 37.5 / 24.2 / 19.5 |
| reactor_retooling | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 | 40.6 / 22.1 / 19.5 | 39.8 / 21.6 / 20.1 | 40.1 / 21.9 / 20.1 |
| clearance_strike | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 | 0.0 / 0.0 / 0.0 | 22.9 / 15.1 / 10.2 | 25.0 / 16.7 / 10.7 | 24.5 / 16.1 / 10.7 |

| Read | d | games | win % | antimatter gained / spent a seat-game | spent ÷ gained |
|---|---|---|---|---|---|
| off | d0 | 96 | 93.8 | 3.48 / 1.17 | 34% |
| off | d1 | 96 | 91.7 | 3.92 / 1.26 | 32% |
| off | d2 | 96 | 83.3 | 3.62 / 1.26 | 35% |
| off | d3 | 96 | 70.8 | 3.61 / 1.15 | 32% |
| off | all | 384 | | 3.66 / 1.21 | 33% |
| idle05 | d0 | 96 | 96.9 | 3.20 / 1.16 | 36% |
| idle05 | d1 | 96 | 90.6 | 3.62 / 1.22 | 34% |
| idle05 | d2 | 96 | 90.6 | 3.05 / 1.18 | 39% |
| idle05 | d3 | 96 | 70.8 | 3.38 / 1.20 | 35% |
| idle05 | all | 384 | | 3.32 / 1.19 | 36% |
| idle02 | d0 | 96 | 93.8 | 3.12 / 1.31 | 42% |
| idle02 | d1 | 96 | 91.7 | 2.90 / 1.23 | 42% |
| idle02 | d2 | 96 | 88.5 | 2.73 / 1.28 | 47% |
| idle02 | d3 | 96 | 71.9 | 3.09 / 1.34 | 44% |
| idle02 | all | 384 | | 2.96 / 1.29 | 44% |
| cards-off | d0 | 96 | 95.8 | 2.79 / 1.12 | 40% |
| cards-off | d1 | 96 | 93.8 | 3.19 / 1.40 | 44% |
| cards-off | d2 | 96 | 86.5 | 3.09 / 1.27 | 41% |
| cards-off | d3 | 96 | 70.8 | 3.17 / 1.37 | 43% |
| cards-off | all | 384 | | 3.06 / 1.29 | 42% |
| cards-idle05 | d0 | 96 | 94.8 | 2.77 / 1.26 | 45% |
| cards-idle05 | d1 | 96 | 95.8 | 3.00 / 1.39 | 46% |
| cards-idle05 | d2 | 96 | 89.6 | 3.00 / 1.29 | 43% |
| cards-idle05 | d3 | 96 | 68.8 | 3.01 / 1.34 | 44% |
| cards-idle05 | all | 384 | | 2.95 / 1.32 | 45% |
| cards-idle02 | d0 | 96 | 94.8 | 2.55 / 1.26 | 49% |
| cards-idle02 | d1 | 96 | 97.9 | 2.74 / 1.42 | 52% |
| cards-idle02 | d2 | 96 | 90.6 | 2.55 / 1.31 | 51% |
| cards-idle02 | d3 | 96 | 65.6 | 2.67 / 1.31 | 49% |
| cards-idle02 | all | 384 | | 2.63 / 1.33 | 50% |

Unspent antimatter at the end of each round (mean over seats reaching it); seat-games ending with any held:
- off: r1 0.05 r2 0.11 r3 0.42 r4 0.83 r5 1.44 r6 2.22 r7 2.94 r8 3.99 r9 4.91 r10 5.88; held at the end 59%
- idle05: r1 0.04 r2 0.09 r3 0.38 r4 0.76 r5 1.31 r6 2.01 r7 2.69 r8 3.56 r9 4.85 r10 5.07; held at the end 58%
- idle02: r1 0.04 r2 0.09 r3 0.35 r4 0.67 r5 1.12 r6 1.64 r7 2.14 r8 2.73 r9 3.49 r10 4.17; held at the end 57%
- cards-off: r1 0.04 r2 0.09 r3 0.20 r4 0.56 r5 0.96 r6 1.49 r7 2.18 r8 2.88 r9 3.87 r10 4.47; held at the end 56%
- cards-idle05: r1 0.04 r2 0.08 r3 0.19 r4 0.52 r5 0.88 r6 1.38 r7 2.08 r8 2.89 r9 4.00 r10 4.26; held at the end 55%
- cards-idle02: r1 0.04 r2 0.08 r3 0.19 r4 0.48 r5 0.79 r6 1.17 r7 1.72 r8 2.23 r9 2.75 r10 3.32; held at the end 54%

## off: 384 games, 7.04 rounds; spent 241.9, made 731.8 per 100 games
- spent: own:use:kinetic_interdiction 32.0, own:play:nuclear_option 27.9, own:play:hostile_buyout 22.9, own:play:antimatter_accord 18.5, own:play:peacekeeping_mission 17.2, own:play:root_access 15.9, own:play:fusion_commons 14.8, own:play:accelerator_diplomacy 13.3, own:play:catalyzed_fab_run 12.8, own:play:collider_consortium 10.2, own:play:recursive_self_improvement 9.6, own:play:singularity_exploit 8.6, own:use:containment_reactor 8.3, own:play:orbital_mass_driver 6.8
- made: own:use:treaty_isotope_bank 119.3, own:use:dark_fiber_collider 99.2, own:use:cyclotron_lease 80.7, own:use:antiproton_foundry 68.5, own:use:penning_trap_array 68.5, own:power:Field Survey 57.3, own:power:Levy 47.9, own:use:positron_storage_ring 32.0, own:power:Accord 26.6, own:power:Peer Review 18.2, own:use:survey_drone_fleet 17.7, own:power:Overcharge II 16.4, own:power:Field Station 16.1, own:use:simulation_cluster 15.1
## idle02: 384 games, 6.92 rounds; spent 258.3, made 592.4 per 100 games
- spent: own:use:kinetic_interdiction 29.9, own:play:nuclear_option 25.8, own:use:orbital_relay_launcher 25.3, own:play:hostile_buyout 21.4, own:play:antimatter_accord 17.7, own:play:peacekeeping_mission 15.9, own:play:fusion_commons 14.8, own:play:accelerator_diplomacy 14.3, own:play:root_access 14.1, own:play:catalyzed_fab_run 13.3, own:play:recursive_self_improvement 12.2, own:play:collider_consortium 9.9, own:play:singularity_exploit 8.6, own:use:containment_reactor 8.1
- made: own:use:treaty_isotope_bank 101.8, own:use:dark_fiber_collider 85.9, own:use:penning_trap_array 62.0, own:use:cyclotron_lease 58.9, own:power:Field Survey 51.6, own:power:Levy 40.1, own:power:Accord 30.2, own:use:positron_storage_ring 28.6, own:use:survey_drone_fleet 17.2, own:use:antiproton_foundry 16.9, own:power:Field Station 16.7, own:power:Peer Review 14.6, own:play:zero_day_auction 13.5, own:power:Overcharge II 11.2
## cards-off: 384 games, 6.97 rounds; spent 258.1, made 612.0 per 100 games
- spent: own:play:hostile_buyout 25.0, own:play:antimatter_accord 21.1, own:play:beamline_liberation 19.8, own:play:reactor_retooling 19.5, own:play:catalyzed_fab_run 17.4, own:use:containment_reactor 14.8, own:play:root_access 14.3, own:play:collider_consortium 14.1, own:play:peacekeeping_mission 13.5, own:play:nuclear_option 13.3, own:play:accelerator_diplomacy 11.5, own:use:kinetic_interdiction 10.9, own:play:decapitation_strike 10.4, own:play:fusion_commons 10.2
- made: own:use:dark_fiber_collider 94.8, own:use:cyclotron_lease 77.6, own:use:treaty_isotope_bank 74.7, own:power:Field Survey 64.1, own:use:antiproton_foundry 49.7, own:use:penning_trap_array 47.1, own:power:Levy 39.6, own:use:positron_storage_ring 28.9, own:power:Accord 26.6, own:power:Field Station 16.7, own:use:survey_drone_fleet 13.0, own:power:Overcharge II 12.5, own:power:Peer Review 10.9, own:power:Outpost 10.4
## cards-idle02: 384 games, 6.94 rounds; spent 265.1, made 525.5 per 100 games
- spent: own:play:hostile_buyout 25.8, own:play:antimatter_accord 22.4, own:play:reactor_retooling 20.1, own:play:beamline_liberation 19.5, own:play:catalyzed_fab_run 17.4, own:play:peacekeeping_mission 13.8, own:use:containment_reactor 13.3, own:play:root_access 12.8, own:play:collider_consortium 12.2, own:play:nuclear_option 12.2, own:play:accelerator_diplomacy 12.0, own:use:kinetic_interdiction 11.2, own:play:clearance_strike 10.7, own:play:fusion_commons 10.2
- made: own:use:dark_fiber_collider 84.6, own:use:treaty_isotope_bank 65.9, own:use:cyclotron_lease 64.8, own:power:Field Survey 58.9, own:use:penning_trap_array 42.7, own:power:Levy 33.3, own:power:Accord 27.1, own:use:positron_storage_ring 24.5, own:use:antiproton_foundry 22.1, own:power:Field Station 17.7, own:power:Outpost 13.5, own:power:Peer Review 12.0, own:use:survey_drone_fleet 9.9, own:power:Overcharge II 9.1
