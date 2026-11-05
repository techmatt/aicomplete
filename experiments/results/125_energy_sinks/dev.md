## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| r49/greedy | 468 | 47.2 | 4.79 | -22.6 ± 2.8 | -1.35 | 51 / 42 / 49 |
| r49/strategist | 468 | 69.9 | 6.15 | - | - | 69 / 68 / 72 |
| r50/strategist | 468 | 68.4 | 5.98 | -1.5 ± 3.0 | -0.17 | 68 / 65 / 72 |
| r50/greedy | 468 | 44.7 | 4.90 | -25.2 ± 3.0 | -1.25 | 54 / 37 / 44 |

## Energy a game, per seat of each board

Lost: cut at the 10-energy limit (`economy.hold_lost_seat`); games cut: share of seat-games losing any. Spent: every fall between actions (`spent`). The game total is both seats.

| cell | game total lost | board | lost | games cut | gained | spent | end held |
|---|---|---|---|---|---|---|---|
| r49/greedy | 0.67 | Builder | 0.43 | 10% | 7.3 | 6.4 | 1.9 |
| r49/greedy | 0.67 | Protector | 0.54 | 11% | 13.8 | 11.5 | 3.2 |
| r49/greedy | 0.67 | Researcher | 0.04 | 2% | 9.3 | 7.0 | 3.3 |
| r49/strategist | 4.34 | Builder | 3.78 | 53% | 18.6 | 15.0 | 4.6 |
| r49/strategist | 4.34 | Protector | 1.75 | 28% | 20.8 | 17.9 | 4.0 |
| r49/strategist | 4.34 | Researcher | 0.99 | 10% | 13.1 | 10.8 | 3.3 |
| r50/strategist | 3.26 | Builder | 2.74 | 54% | 17.2 | 14.6 | 3.6 |
| r50/strategist | 3.26 | Protector | 1.46 | 26% | 19.5 | 17.0 | 3.5 |
| r50/strategist | 3.26 | Researcher | 0.69 | 16% | 13.3 | 11.2 | 3.2 |
| r50/greedy | 0.48 | Builder | 0.29 | 4% | 6.8 | 6.4 | 1.4 |
| r50/greedy | 0.48 | Protector | 0.40 | 8% | 13.6 | 11.8 | 2.8 |
| r50/greedy | 0.48 | Researcher | 0.04 | 3% | 9.4 | 7.2 | 3.2 |

## Where energy goes (spent a seat-game, by use; top uses)

- r49/greedy, Builder (312): own:power:Deploy 4.73; own:power:Buy In 1.22; own:power:Rush Build 0.17; own:pay:commandeer 0.07; phase:threats.execute 0.07; own:pay:blackout 0.06; own:convert:0 0.05; own:pay:sabotage 0.04
- r49/greedy, Protector (312): own:power:Deploy 7.30; own:power:Accord 1.72; own:power:Bulwark 1.62; own:power:Shelter 0.47; own:pay:commandeer 0.15; own:pay:sabotage 0.08; own:convert:0 0.07; own:pay:blackout 0.04
- r49/greedy, Researcher (312): own:power:Deploy 4.08; own:power:Fieldwork 2.35; own:power:Particle Run 0.27; own:power:Peer Review 0.09; phase:threats.execute 0.05; own:play:cryo_compute_burn 0.04; own:pay:commandeer 0.04; own:play:power_to_the_people 0.01
- r49/strategist, Builder (312): own:power:Deploy 8.36; own:power:Buy In 4.34; own:convert:0 0.48; own:power:Rush Build 0.45; own:pay:sabotage 0.36; own:play:power_to_the_people 0.24; own:play:cryptojacking_campaign 0.23; own:pay:blackout 0.22
- r49/strategist, Protector (312): own:power:Deploy 9.96; own:power:Accord 3.48; own:power:Bulwark 2.46; own:convert:0 0.40; own:pay:sabotage 0.35; own:power:Shelter 0.28; own:play:cryptojacking_campaign 0.26; own:pay:commandeer 0.24
- r49/strategist, Researcher (312): own:power:Deploy 6.58; own:power:Fieldwork 2.22; own:power:Peer Review 0.69; own:play:cryptojacking_campaign 0.36; own:power:Particle Run 0.25; own:pay:commandeer 0.17; own:pay:blackout 0.14; own:convert:0 0.14
- r50/strategist, Builder (312): own:power:Deploy 7.80; own:power:Buy In 3.59; own:power:Rush Build 0.56; own:play:citywide_microgrid 0.54; own:pay:sabotage 0.29; own:pay:commandeer 0.20; own:play:orbital_mass_driver 0.19; own:play:lights_out_assembly 0.19
- r50/strategist, Protector (312): own:power:Deploy 9.00; own:power:Accord 2.97; own:power:Bulwark 2.08; own:play:citywide_microgrid 0.72; own:pay:sabotage 0.38; own:play:cryptojacking_campaign 0.30; own:power:Shelter 0.24; own:pay:commandeer 0.23
- r50/strategist, Researcher (312): own:power:Deploy 6.51; own:power:Fieldwork 2.00; own:power:Peer Review 0.46; own:play:foundation_model_campus 0.36; own:play:cryptojacking_campaign 0.29; own:power:Particle Run 0.22; own:play:lights_out_assembly 0.21; own:pay:sabotage 0.18
- r50/greedy, Builder (312): own:power:Deploy 4.70; own:power:Buy In 1.10; own:play:lights_out_assembly 0.14; own:power:Rush Build 0.12; own:pay:sabotage 0.08; own:pay:commandeer 0.07; own:play:fabrication_megaplex 0.06; own:pay:blackout 0.04
- r50/greedy, Protector (312): own:power:Deploy 7.09; own:power:Bulwark 1.60; own:power:Accord 1.48; own:power:Shelter 0.50; own:play:fabrication_megaplex 0.35; own:play:lights_out_assembly 0.24; own:pay:commandeer 0.14; own:pay:sabotage 0.12
- r50/greedy, Researcher (312): own:power:Deploy 3.93; own:power:Fieldwork 2.43; own:power:Particle Run 0.31; own:play:lights_out_assembly 0.15; own:play:fabrication_megaplex 0.09; phase:threats.execute 0.08; own:play:foundation_model_campus 0.06; own:play:citywide_microgrid 0.05

## Energy lost a game by round (both seats)

| cell | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 |
|---|---|---|---|---|---|---|---|---|
| r49/greedy | 0.00 | 0.12 | 0.37 | 0.15 | 0.03 | 0.00 | 0.00 | 0.00 |
| r49/strategist | 0.00 | 0.28 | 1.88 | 0.65 | 0.38 | 0.49 | 0.56 | 0.11 |
| r50/strategist | 0.01 | 0.45 | 1.26 | 0.62 | 0.33 | 0.32 | 0.21 | 0.06 |
| r50/greedy | 0.00 | 0.12 | 0.13 | 0.23 | 0.00 | 0.00 | 0.00 | 0.00 |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | antimatter | research_cubes | ambassadors |
|---|---|---|---|---|---|---|---|---|
| r49/greedy | Builder | 41.9 / 99 | 8.3 / 33 | 3.8 / 10 | 3.0 / 10 | 0.1 / 2 | 0.3 / 4 | 4.2 / 10 |
| r49/greedy | Protector | 48.9 / 101 | 9.7 / 26 | 5.4 / 10 | 2.8 / 10 | 0.1 / 4 | 0.6 / 6 | 4.5 / 10 |
| r49/greedy | Researcher | 44.1 / 85 | 14.2 / 40 | 5.9 / 10 | 2.2 / 10 | 0.3 / 3 | 0.9 / 7 | 4.6 / 10 |
| r49/strategist | Builder | 67.6 / 150 | 11.1 / 33 | 8.5 / 10 | 5.1 / 10 | 0.5 / 10 | 1.2 / 5 | 3.9 / 10 |
| r49/strategist | Protector | 60.3 / 150 | 12.9 / 29 | 6.7 / 10 | 3.8 / 10 | 0.3 / 9 | 1.6 / 17 | 4.6 / 10 |
| r49/strategist | Researcher | 64.5 / 150 | 22.2 / 56 | 6.8 / 10 | 3.5 / 10 | 0.5 / 6 | 3.0 / 12 | 4.9 / 10 |
| r50/strategist | Builder | 62.3 / 150 | 10.4 / 29 | 8.3 / 10 | 4.7 / 10 | 0.3 / 4 | 1.2 / 9 | 3.8 / 10 |
| r50/strategist | Protector | 56.7 / 150 | 12.2 / 31 | 6.8 / 10 | 3.4 / 10 | 0.2 / 13 | 1.2 / 10 | 4.5 / 10 |
| r50/strategist | Researcher | 63.5 / 150 | 21.7 / 65 | 7.1 / 10 | 3.4 / 10 | 0.5 / 4 | 2.7 / 10 | 4.4 / 10 |
| r50/greedy | Builder | 42.3 / 109 | 8.3 / 17 | 3.2 / 10 | 2.8 / 10 | 0.1 / 2 | 0.3 / 5 | 4.2 / 10 |
| r50/greedy | Protector | 48.7 / 89 | 9.9 / 21 | 5.1 / 10 | 2.6 / 10 | 0.0 / 2 | 0.7 / 25 | 4.2 / 10 |
| r50/greedy | Researcher | 42.3 / 67 | 14.3 / 35 | 6.1 / 10 | 1.8 / 10 | 0.3 / 3 | 0.8 / 6 | 4.6 / 10 |

## Plays a game of the changed cards (kept, offered); Actions used a game; plays by board a seat-game

| card | cell | played | kept | offered | Action uses | Builder | Protector | Researcher |
|---|---|---|---|---|---|---|---|---|
| fabrication_megaplex | r49/greedy | 0.207 | 0.12 | 0.06 | 0.00 | 0.067 | 0.109 | 0.074 |
| fabrication_megaplex | r49/strategist | 0.207 | 0.21 | 0.15 | 0.00 | 0.119 | 0.048 | 0.096 |
| fabrication_megaplex | r50/strategist | 0.177 | 0.14 | 0.17 | 0.00 | 0.090 | 0.080 | 0.074 |
| fabrication_megaplex | r50/greedy | 0.171 | 0.09 | 0.08 | 0.00 | 0.032 | 0.173 | 0.045 |
| lights_out_assembly | r49/greedy | 0.393 | 0.19 | 0.03 | 0.00 | 0.163 | 0.212 | 0.208 |
| lights_out_assembly | r49/strategist | 0.427 | 0.26 | 0.10 | 0.00 | 0.221 | 0.176 | 0.218 |
| lights_out_assembly | r50/strategist | 0.393 | 0.31 | 0.15 | 0.00 | 0.189 | 0.170 | 0.218 |
| lights_out_assembly | r50/greedy | 0.355 | 0.24 | 0.07 | 0.00 | 0.138 | 0.244 | 0.151 |
| orbital_mass_driver | r49/greedy | 0.006 | 0.10 | 0.53 | 0.00 | 0.003 | 0.000 | 0.006 |
| orbital_mass_driver | r49/strategist | 0.141 | 0.28 | 0.59 | 0.00 | 0.115 | 0.032 | 0.058 |
| orbital_mass_driver | r50/strategist | 0.081 | 0.23 | 0.59 | 0.00 | 0.064 | 0.016 | 0.038 |
| orbital_mass_driver | r50/greedy | 0.002 | 0.09 | 0.53 | 0.00 | 0.000 | 0.003 | 0.000 |
| foundation_model_campus | r49/greedy | 0.077 | 0.14 | 0.47 | 0.00 | 0.019 | 0.045 | 0.045 |
| foundation_model_campus | r49/strategist | 0.233 | 0.38 | 0.54 | 0.00 | 0.061 | 0.058 | 0.224 |
| foundation_model_campus | r50/strategist | 0.182 | 0.33 | 0.54 | 0.00 | 0.058 | 0.035 | 0.179 |
| foundation_model_campus | r50/greedy | 0.038 | 0.10 | 0.46 | 0.00 | 0.006 | 0.022 | 0.029 |
| positron_storage_ring | r49/greedy | 0.000 | 0.04 | 0.49 | 0.00 | 0.000 | 0.000 | 0.000 |
| positron_storage_ring | r49/strategist | 0.066 | 0.25 | 0.55 | 0.00 | 0.067 | 0.010 | 0.013 |
| positron_storage_ring | r50/strategist | 0.058 | 0.22 | 0.51 | 0.00 | 0.054 | 0.016 | 0.010 |
| positron_storage_ring | r50/greedy | 0.000 | 0.05 | 0.46 | 0.00 | 0.000 | 0.000 | 0.000 |
| particle_accelerator | r49/greedy | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| particle_accelerator | r49/strategist | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| particle_accelerator | r50/strategist | 0.288 | 0.26 | 0.15 | 0.09 | 0.144 | 0.115 | 0.141 |
| particle_accelerator | r50/greedy | 0.000 | 0.18 | 0.06 | 0.00 | 0.000 | 0.000 | 0.000 |
| citywide_microgrid | r49/greedy | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| citywide_microgrid | r49/strategist | 0.000 | 0.00 | 0.00 | 0.00 | 0.000 | 0.000 | 0.000 |
| citywide_microgrid | r50/strategist | 0.111 | 0.38 | 0.60 | 0.09 | 0.067 | 0.090 | 0.010 |
| citywide_microgrid | r50/greedy | 0.013 | 0.21 | 0.53 | 0.01 | 0.000 | 0.013 | 0.006 |
| moonshot_grant | r49/greedy | 0.004 | 0.01 | 0.09 | 0.00 | 0.000 | 0.006 | 0.000 |
| moonshot_grant | r49/strategist | 0.026 | 0.07 | 0.18 | 0.00 | 0.006 | 0.006 | 0.026 |
| moonshot_grant | r50/strategist | 0.013 | 0.06 | 0.16 | 0.00 | 0.010 | 0.006 | 0.003 |
| moonshot_grant | r50/greedy | 0.006 | 0.01 | 0.12 | 0.00 | 0.003 | 0.006 | 0.000 |

## Energy-cost cards by board: plays a seat-game, and held at the end of an action phase short of energy

Held: energy-cost cards in hand as the seat passed its action phase, summed over rounds; short: of those, needing more energy than it held. A seat-game.

| cell | board | energy-cost plays | held at pass | short | seat-games with one short |
|---|---|---|---|---|---|
| r49/greedy | Builder | 0.02 | 0.55 | 0.42 | 19% |
| r49/greedy | Protector | 0.04 | 1.30 | 1.03 | 40% |
| r49/greedy | Researcher | 0.05 | 0.95 | 0.70 | 31% |
| r49/strategist | Builder | 0.21 | 0.45 | 0.26 | 14% |
| r49/strategist | Protector | 0.17 | 0.63 | 0.52 | 23% |
| r49/strategist | Researcher | 0.24 | 0.91 | 0.62 | 30% |
| r50/strategist | Builder | 0.68 | 1.81 | 1.33 | 49% |
| r50/strategist | Protector | 0.59 | 2.53 | 2.19 | 66% |
| r50/strategist | Researcher | 0.75 | 2.88 | 2.14 | 62% |
| r50/greedy | Builder | 0.19 | 1.65 | 1.44 | 43% |
| r50/greedy | Protector | 0.47 | 2.72 | 2.42 | 68% |
| r50/greedy | Researcher | 0.27 | 2.29 | 1.81 | 59% |

Energy-cost cards (per label): r49/greedy: cryo_compute_burn, cryptojacking_campaign, power_to_the_people; r49/strategist: cryo_compute_burn, cryptojacking_campaign, power_to_the_people; r50/strategist: citywide_microgrid, cryo_compute_burn, cryptojacking_campaign, fabrication_megaplex, foundation_model_campus, lights_out_assembly, orbital_mass_driver, positron_storage_ring, power_to_the_people; r50/greedy: citywide_microgrid, cryo_compute_burn, cryptojacking_campaign, fabrication_megaplex, foundation_model_campus, lights_out_assembly, orbital_mass_driver, positron_storage_ring, power_to_the_people
