## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| r49/strategist | 144 | 74.3 | 6.15 | - | - | 65 / 75 / 83 |
| r49/greedy | 144 | 51.4 | 4.78 | -22.9 ± 5.0 | -1.38 | 56 / 46 / 52 |
| r50/strategist | 144 | 73.6 | 5.90 | -0.7 ± 5.2 | -0.26 | 71 / 67 / 83 |
| r50/greedy | 144 | 47.9 | 4.72 | -26.4 ± 5.6 | -1.44 | 54 / 38 / 52 |

## Energy a game, per seat of each board

Lost: cut at the 10-energy limit (`economy.hold_lost_seat`); games cut: share of seat-games losing any. Spent: every fall between actions (`spent`). The game total is both seats.

| cell | game total lost | board | lost | games cut | gained | spent | end held |
|---|---|---|---|---|---|---|---|
| r49/strategist | 4.32 | Builder | 3.93 | 50% | 19.2 | 15.6 | 4.6 |
| r49/strategist | 4.32 | Protector | 2.22 | 27% | 21.1 | 17.9 | 4.1 |
| r49/strategist | 4.32 | Researcher | 0.33 | 8% | 11.8 | 9.8 | 3.0 |
| r49/greedy | 0.62 | Builder | 0.60 | 10% | 7.4 | 6.8 | 1.6 |
| r49/greedy | 0.62 | Protector | 0.30 | 6% | 13.1 | 11.0 | 3.1 |
| r49/greedy | 0.62 | Researcher | 0.02 | 2% | 8.6 | 6.3 | 3.3 |
| r50/strategist | 3.36 | Builder | 3.17 | 49% | 17.0 | 14.1 | 3.8 |
| r50/strategist | 3.36 | Protector | 1.53 | 27% | 19.0 | 16.5 | 3.5 |
| r50/strategist | 3.36 | Researcher | 0.34 | 9% | 12.9 | 10.3 | 3.6 |
| r50/greedy | 0.51 | Builder | 0.24 | 4% | 7.1 | 6.7 | 1.4 |
| r50/greedy | 0.51 | Protector | 0.41 | 11% | 13.4 | 11.5 | 2.9 |
| r50/greedy | 0.51 | Researcher | 0.11 | 5% | 9.4 | 7.2 | 3.2 |

## Where energy goes (spent a seat-game, by use; top uses)

- r49/strategist, Builder (96): own:power:Deploy 8.51; own:power:Buy In 4.41; own:play:power_to_the_people 0.50; own:power:Rush Build 0.48; own:convert:0 0.42; own:pay:sabotage 0.36; own:play:cryptojacking_campaign 0.33; own:pay:commandeer 0.30
- r49/strategist, Protector (96): own:power:Deploy 10.35; own:power:Accord 3.45; own:power:Bulwark 2.36; own:convert:0 0.42; own:play:cryptojacking_campaign 0.35; own:pay:sabotage 0.28; own:power:Shelter 0.26; own:pay:commandeer 0.20
- r49/strategist, Researcher (96): own:power:Deploy 6.32; own:power:Fieldwork 2.10; own:power:Peer Review 0.31; own:play:cryptojacking_campaign 0.29; own:power:Particle Run 0.19; own:pay:commandeer 0.18; own:pay:blackout 0.17; own:convert:0 0.06
- r49/greedy, Builder (96): own:power:Deploy 4.77; own:power:Buy In 1.27; own:power:Rush Build 0.18; own:pay:sabotage 0.16; own:pay:commandeer 0.08; own:convert:0 0.08; own:use:demand_response_grid 0.05; own:pay:blackout 0.05
- r49/greedy, Protector (96): own:power:Deploy 7.03; own:power:Accord 1.66; own:power:Bulwark 1.56; own:power:Shelter 0.45; own:pay:commandeer 0.15; own:pay:sabotage 0.11; own:pay:blackout 0.04; own:play:cryo_compute_burn 0.03
- r49/greedy, Researcher (96): own:power:Deploy 3.70; own:power:Fieldwork 2.34; own:power:Particle Run 0.11; phase:threats.execute 0.04; own:play:cryo_compute_burn 0.03; own:pay:blackout 0.02; own:power:Peer Review 0.02; own:pay:commandeer 0.01
- r50/strategist, Builder (96): own:power:Deploy 7.33; own:power:Buy In 3.26; own:play:citywide_microgrid 0.75; own:pay:sabotage 0.51; own:power:Rush Build 0.47; own:pay:commandeer 0.26; own:convert:0 0.25; own:pay:blackout 0.23
- r50/strategist, Protector (96): own:power:Deploy 9.17; own:power:Accord 2.77; own:power:Bulwark 1.97; own:play:citywide_microgrid 0.50; own:pay:sabotage 0.30; own:power:Shelter 0.28; own:pay:commandeer 0.24; own:use:particle_accelerator 0.23
- r50/strategist, Researcher (96): own:power:Deploy 5.89; own:power:Fieldwork 1.90; own:power:Peer Review 0.46; own:play:cryptojacking_campaign 0.31; own:pay:commandeer 0.25; own:play:foundation_model_campus 0.25; own:power:Particle Run 0.19; own:play:lights_out_assembly 0.19
- r50/greedy, Builder (96): own:power:Deploy 4.65; own:power:Buy In 1.23; own:play:lights_out_assembly 0.19; own:power:Rush Build 0.14; own:pay:sabotage 0.10; own:pay:commandeer 0.10; own:play:fabrication_megaplex 0.10; own:pay:blackout 0.06
- r50/greedy, Protector (96): own:power:Deploy 6.76; own:power:Accord 1.59; own:power:Bulwark 1.47; own:power:Shelter 0.58; own:play:fabrication_megaplex 0.42; own:play:lights_out_assembly 0.19; own:pay:commandeer 0.15; phase:threats.execute 0.08
- r50/greedy, Researcher (96): own:power:Deploy 3.84; own:power:Fieldwork 2.42; own:power:Particle Run 0.28; phase:threats.execute 0.12; own:play:fabrication_megaplex 0.12; own:play:lights_out_assembly 0.09; own:play:citywide_microgrid 0.08; own:power:Peer Review 0.07

## Energy lost a game by round (both seats)

| cell | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 |
|---|---|---|---|---|---|---|---|---|---|
| r49/strategist | 0.00 | 0.46 | 1.19 | 1.00 | 0.45 | 0.43 | 0.58 | 0.20 | 0.00 |
| r49/greedy | 0.00 | 0.19 | 0.38 | 0.04 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| r50/strategist | 0.00 | 0.45 | 1.58 | 0.56 | 0.08 | 0.25 | 0.26 | 0.17 | 0.00 |
| r50/greedy | 0.00 | 0.15 | 0.28 | 0.06 | 0.00 | 0.00 | 0.00 | 0.00 | 0.01 |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| r49/strategist | Builder | 71.9 / 150 | 11.7 / 26 | 8.4 / 10 | 4.8 / 10 | 0.6 / 9 | 1.4 / 4 | 4.0 / 10 |
| r49/strategist | Protector | 61.7 / 149 | 13.1 / 30 | 6.6 / 10 | 3.9 / 10 | 0.3 / 8 | 1.9 / 15 | 4.4 / 10 |
| r49/strategist | Researcher | 62.0 / 150 | 22.5 / 47 | 6.7 / 10 | 3.3 / 10 | 0.5 / 5 | 2.6 / 7 | 4.9 / 10 |
| r49/greedy | Builder | 43.1 / 98 | 8.2 / 15 | 3.8 / 10 | 3.1 / 10 | 0.1 / 3 | 0.3 / 3 | 4.2 / 10 |
| r49/greedy | Protector | 48.8 / 94 | 9.8 / 20 | 5.0 / 10 | 2.3 / 10 | 0.1 / 3 | 0.6 / 7 | 4.2 / 10 |
| r49/greedy | Researcher | 42.9 / 77 | 14.0 / 31 | 5.5 / 10 | 1.8 / 10 | 0.2 / 2 | 0.7 / 7 | 4.4 / 10 |
| r50/strategist | Builder | 60.9 / 150 | 10.1 / 24 | 8.0 / 10 | 4.3 / 10 | 0.2 / 5 | 1.2 / 4 | 3.6 / 10 |
| r50/strategist | Protector | 56.9 / 150 | 12.0 / 21 | 6.5 / 10 | 3.4 / 10 | 0.2 / 4 | 1.4 / 8 | 4.7 / 10 |
| r50/strategist | Researcher | 64.5 / 150 | 21.0 / 43 | 6.9 / 10 | 3.3 / 10 | 0.5 / 3 | 2.7 / 8 | 4.2 / 10 |
| r50/greedy | Builder | 42.1 / 112 | 8.5 / 16 | 3.2 / 10 | 2.7 / 10 | 0.0 / 2 | 0.2 / 3 | 4.3 / 10 |
| r50/greedy | Protector | 48.3 / 79 | 9.4 / 18 | 5.1 / 10 | 2.8 / 10 | 0.1 / 3 | 0.6 / 6 | 4.0 / 10 |
| r50/greedy | Researcher | 44.6 / 76 | 14.3 / 29 | 5.8 / 10 | 1.8 / 10 | 0.4 / 8 | 0.8 / 13 | 4.6 / 10 |

## Plays a game of the changed cards (kept, offered); Actions used a game; plays by board a seat-game

| card | cell | played | kept | offered | Action uses | Builder | Protector | Researcher |
|---|---|---|---|---|---|---|---|---|
| fabrication_megaplex | r49/strategist | 0.326 | 0.28 | 0.15 | 0.00 | 0.125 | 0.062 | 0.198 |
| fabrication_megaplex | r49/greedy | 0.382 | 0.20 | 0.06 | 0.00 | 0.115 | 0.219 | 0.198 |
| fabrication_megaplex | r50/strategist | 0.118 | 0.15 | 0.18 | 0.00 | 0.062 | 0.042 | 0.052 |
| fabrication_megaplex | r50/greedy | 0.215 | 0.10 | 0.07 | 0.00 | 0.052 | 0.208 | 0.062 |
| lights_out_assembly | r49/strategist | 0.438 | 0.26 | 0.15 | 0.00 | 0.281 | 0.177 | 0.188 |
| lights_out_assembly | r49/greedy | 0.361 | 0.16 | 0.07 | 0.00 | 0.167 | 0.188 | 0.177 |
| lights_out_assembly | r50/strategist | 0.375 | 0.32 | 0.15 | 0.00 | 0.198 | 0.156 | 0.188 |
| lights_out_assembly | r50/greedy | 0.319 | 0.27 | 0.10 | 0.00 | 0.188 | 0.188 | 0.094 |
| orbital_mass_driver | r49/strategist | 0.125 | 0.30 | 0.62 | 0.00 | 0.125 | 0.021 | 0.042 |
| orbital_mass_driver | r49/greedy | 0.014 | 0.11 | 0.56 | 0.00 | 0.000 | 0.010 | 0.010 |
| orbital_mass_driver | r50/strategist | 0.049 | 0.14 | 0.51 | 0.00 | 0.021 | 0.000 | 0.052 |
| orbital_mass_driver | r50/greedy | 0.014 | 0.06 | 0.49 | 0.00 | 0.010 | 0.010 | 0.000 |
| foundation_model_campus | r49/strategist | 0.250 | 0.42 | 0.59 | 0.00 | 0.052 | 0.073 | 0.229 |
| foundation_model_campus | r49/greedy | 0.056 | 0.17 | 0.56 | 0.00 | 0.000 | 0.042 | 0.042 |
| foundation_model_campus | r50/strategist | 0.146 | 0.31 | 0.61 | 0.00 | 0.062 | 0.031 | 0.125 |
| foundation_model_campus | r50/greedy | 0.042 | 0.08 | 0.49 | 0.00 | 0.021 | 0.021 | 0.021 |
| positron_storage_ring | r49/strategist | 0.146 | 0.32 | 0.60 | 0.00 | 0.177 | 0.010 | 0.010 |
| positron_storage_ring | r49/greedy | 0.000 | 0.03 | 0.57 | 0.00 | 0.000 | 0.000 | 0.000 |
| positron_storage_ring | r50/strategist | 0.076 | 0.23 | 0.59 | 0.00 | 0.062 | 0.031 | 0.021 |
| positron_storage_ring | r50/greedy | 0.000 | 0.06 | 0.51 | 0.00 | 0.000 | 0.000 | 0.000 |
| particle_accelerator | r49/strategist | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| particle_accelerator | r49/greedy | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| particle_accelerator | r50/strategist | 0.424 | 0.40 | 0.11 | 0.12 | 0.260 | 0.188 | 0.177 |
| particle_accelerator | r50/greedy | 0.007 | 0.35 | 0.08 | 0.00 | 0.010 | 0.000 | 0.000 |
| citywide_microgrid | r49/strategist | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| citywide_microgrid | r49/greedy | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| citywide_microgrid | r50/strategist | 0.111 | 0.44 | 0.67 | 0.05 | 0.094 | 0.062 | 0.010 |
| citywide_microgrid | r50/greedy | 0.014 | 0.25 | 0.63 | 0.01 | 0.000 | 0.010 | 0.010 |
| moonshot_grant | r49/strategist | 0.028 | 0.07 | 0.17 | 0.00 | 0.021 | 0.021 | 0.000 |
| moonshot_grant | r49/greedy | 0.000 | 0.01 | 0.08 | 0.00 | 0.000 | 0.000 | 0.000 |
| moonshot_grant | r50/strategist | 0.021 | 0.10 | 0.22 | 0.00 | 0.010 | 0.000 | 0.021 |
| moonshot_grant | r50/greedy | 0.000 | 0.00 | 0.06 | 0.00 | 0.000 | 0.000 | 0.000 |

## Energy-cost cards by board: plays a seat-game, and held at the end of an action phase short of energy

Held: energy-cost cards in hand as the seat passed its action phase, summed over rounds; short: of those, needing more energy than it held. A seat-game.

| cell | board | energy-cost plays | held at pass | short | seat-games with one short |
|---|---|---|---|---|---|
| r49/strategist | Builder | 0.38 | 0.72 | 0.36 | 22% |
| r49/strategist | Protector | 0.20 | 0.88 | 0.69 | 29% |
| r49/strategist | Researcher | 0.18 | 0.89 | 0.64 | 25% |
| r49/greedy | Builder | 0.04 | 0.86 | 0.64 | 30% |
| r49/greedy | Protector | 0.03 | 1.57 | 1.27 | 45% |
| r49/greedy | Researcher | 0.03 | 1.54 | 1.14 | 48% |
| r50/strategist | Builder | 0.65 | 1.86 | 1.47 | 56% |
| r50/strategist | Protector | 0.49 | 2.72 | 2.30 | 70% |
| r50/strategist | Researcher | 0.68 | 2.71 | 2.14 | 61% |
| r50/greedy | Builder | 0.28 | 1.89 | 1.67 | 59% |
| r50/greedy | Protector | 0.44 | 2.84 | 2.43 | 77% |
| r50/greedy | Researcher | 0.22 | 2.38 | 1.85 | 62% |

Energy-cost cards (per label): r49/strategist: cryo_compute_burn, cryptojacking_campaign, power_to_the_people; r49/greedy: cryo_compute_burn, cryptojacking_campaign, power_to_the_people; r50/strategist: citywide_microgrid, cryo_compute_burn, cryptojacking_campaign, fabrication_megaplex, foundation_model_campus, lights_out_assembly, orbital_mass_driver, positron_storage_ring, power_to_the_people; r50/greedy: citywide_microgrid, cryo_compute_burn, cryptojacking_campaign, fabrication_megaplex, foundation_model_campus, lights_out_assembly, orbital_mass_driver, positron_storage_ring, power_to_the_people
