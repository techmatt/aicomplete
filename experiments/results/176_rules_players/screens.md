# 176: screens on the dev seeds (36800-36815, 16 a pair, d0-d4)

## The first r75 build against r74 (strategist and greedy)

Before the agents' Buy energy rule and botnet-reach term, and before the review's fixes. `specs176.py screen 36800 --games 16 --agents 'strategist;greedy' --tag smoke`; the prompt's numbers cell (`--tag smoke5 --set board.city_energy_credits=5`) played the same (energy bought 0.01 a game either way: the agents did not buy at all then).

| agent | d | games | old win % | new win % | gap | gap SE |
|---|---|---|---|---|---|---|
| greedy | 0 | 48 | 95.8 | 95.8 | +0.0 | 3.0 |
| greedy | 1 | 48 | 72.9 | 85.4 | +12.5 | 7.7 |
| greedy | 2 | 48 | 50.0 | 70.8 | +20.8 | 7.9 |
| greedy | 3 | 48 | 29.2 | 58.3 | +29.2 | 6.6 |
| greedy | 4 | 48 | 37.5 | 52.1 | +14.6 | 7.9 |
| strategist | 0 | 48 | 91.7 | 93.8 | +2.1 | 4.7 |
| strategist | 1 | 48 | 66.7 | 75.0 | +8.3 | 6.5 |
| strategist | 2 | 48 | 50.0 | 75.0 | +25.0 | 8.2 |
| strategist | 3 | 48 | 31.2 | 50.0 | +18.8 | 8.2 |
| strategist | 4 | 48 | 29.2 | 41.7 | +12.5 | 7.7 |

## The two new agent heuristics on and off (`--tag onoff`)

| agent | d0 | d1 | d2 | d3 | d4 | all |
|---|---|---|---|---|---|---|
| greedy | 91.7 | 79.2 | 60.4 | 39.6 | 39.6 | 62.1 |
| greedy:energy_rule=0 | 79.2 | 75.0 | 64.6 | 45.8 | 41.7 | 61.2 |
| greedy:w.botnet_reach=0 | 85.4 | 77.1 | 66.7 | 50.0 | 50.0 | 65.8 |
| strategist | 83.3 | 75.0 | 70.8 | 50.0 | 43.8 | 64.6 |
| strategist:energy_rule=0 | 77.1 | 81.2 | 62.5 | 43.8 | 43.8 | 61.7 |
| strategist:w.botnet_reach=0 | 79.2 | 77.1 | 66.7 | 47.9 | 43.8 | 62.9 |

| off spec | d | games | gap (off - base) | SE |
|---|---|---|---|---|
| greedy:energy_rule=0 | 0 | 48 | -12.5 | 6.4 |
| greedy:energy_rule=0 | 1 | 48 | -4.2 | 6.6 |
| greedy:energy_rule=0 | 2 | 48 | +4.2 | 7.3 |
| greedy:energy_rule=0 | 3 | 48 | +6.2 | 5.5 |
| greedy:energy_rule=0 | 4 | 48 | +2.1 | 6.3 |
| greedy:energy_rule=0 | all | 240 | -0.8 | 2.9 |
| greedy:w.botnet_reach=0 | 0 | 48 | -6.2 | 5.5 |
| greedy:w.botnet_reach=0 | 1 | 48 | -2.1 | 7.0 |
| greedy:w.botnet_reach=0 | 2 | 48 | +6.2 | 4.6 |
| greedy:w.botnet_reach=0 | 3 | 48 | +10.4 | 7.4 |
| greedy:w.botnet_reach=0 | 4 | 48 | +10.4 | 5.4 |
| greedy:w.botnet_reach=0 | all | 240 | +3.8 | 2.7 |
| strategist:energy_rule=0 | 0 | 48 | -6.2 | 8.1 |
| strategist:energy_rule=0 | 1 | 48 | +6.2 | 6.9 |
| strategist:energy_rule=0 | 2 | 48 | -8.3 | 7.2 |
| strategist:energy_rule=0 | 3 | 48 | -6.2 | 6.9 |
| strategist:energy_rule=0 | 4 | 48 | +0.0 | 7.3 |
| strategist:energy_rule=0 | all | 240 | -2.9 | 3.3 |
| strategist:w.botnet_reach=0 | 0 | 48 | -4.2 | 7.9 |
| strategist:w.botnet_reach=0 | 1 | 48 | +2.1 | 6.3 |
| strategist:w.botnet_reach=0 | 2 | 48 | -4.2 | 2.9 |
| strategist:w.botnet_reach=0 | 3 | 48 | -2.1 | 5.6 |
| strategist:w.botnet_reach=0 | 4 | 48 | +0.0 | 3.0 |
| strategist:w.botnet_reach=0 | all | 240 | -1.7 | 2.4 |

```
greedy dall (240 games)
  remove 2.47, claim 0.00, liberate 6.75, lib.city 3.79, lib.station 1.82, lib.factory 1.13, claim.city 0.00, claim.station 0.10, claim.factory 0.00, own_on_site 2.37, botnet_std 1.49, botnets_down 1.82, buy_energy 2.56, energy_bought 2.56, move 10.17, armor 0.14, harden_used 2.27, base_absorbed 0.09, aegis 0.05, declined 0.34, factory_amb 0.13, base_amb 0.07, held_off 3.79, canceled 6.68, fac.neutral 2.45, fac.player 1.35, fac.adv 1.98, cubes_conv 2.70, tips_discord 3.71, tips_unity 7.72, converter 1.26
greedy:energy_rule=0 dall (240 games)
  remove 2.36, claim 0.00, liberate 6.99, lib.city 3.83, lib.station 1.90, lib.factory 1.27, claim.city 0.00, claim.station 0.12, claim.factory 0.00, own_on_site 2.32, botnet_std 1.53, botnets_down 1.85, buy_energy 0.02, energy_bought 0.02, move 10.21, armor 0.18, harden_used 2.39, base_absorbed 0.12, aegis 0.05, declined 0.35, factory_amb 0.12, base_amb 0.10, held_off 3.96, canceled 6.99, fac.neutral 2.47, fac.player 1.58, fac.adv 2.00, cubes_conv 2.76, tips_discord 3.58, tips_unity 7.97, converter 1.35
greedy:w.botnet_reach=0 dall (240 games)
  remove 2.78, claim 0.00, liberate 7.27, lib.city 4.13, lib.station 1.84, lib.factory 1.30, claim.city 0.00, claim.station 0.14, claim.factory 0.00, own_on_site 2.10, botnet_std 0.28, botnets_down 1.04, buy_energy 2.55, energy_bought 2.55, move 9.04, armor 0.14, harden_used 2.49, base_absorbed 0.15, aegis 0.08, declined 0.38, factory_amb 0.09, base_amb 0.05, held_off 3.91, canceled 6.90, fac.neutral 2.42, fac.player 1.46, fac.adv 2.00, cubes_conv 2.83, tips_discord 3.58, tips_unity 8.30, converter 1.32
strategist dall (240 games)
  remove 2.64, claim 0.01, liberate 6.98, lib.city 4.23, lib.station 1.53, lib.factory 1.21, claim.city 0.00, claim.station 0.10, claim.factory 0.01, own_on_site 2.01, botnet_std 1.27, botnets_down 1.59, buy_energy 2.91, energy_bought 2.91, move 8.62, armor 0.15, harden_used 2.26, base_absorbed 0.10, aegis 0.08, declined 0.33, factory_amb 0.14, base_amb 0.08, held_off 4.40, canceled 6.24, fac.neutral 2.51, fac.player 1.56, fac.adv 2.09, cubes_conv 2.78, tips_discord 3.84, tips_unity 7.73, converter 1.25
strategist:energy_rule=0 dall (240 games)
  remove 2.69, claim 0.00, liberate 6.70, lib.city 4.10, lib.station 1.54, lib.factory 1.07, claim.city 0.00, claim.station 0.09, claim.factory 0.00, own_on_site 1.63, botnet_std 1.21, botnets_down 1.59, buy_energy 0.00, energy_bought 0.00, move 7.92, armor 0.14, harden_used 2.05, base_absorbed 0.08, aegis 0.03, declined 0.40, factory_amb 0.12, base_amb 0.07, held_off 4.02, canceled 5.81, fac.neutral 2.50, fac.player 1.54, fac.adv 1.97, cubes_conv 2.28, tips_discord 3.92, tips_unity 7.49, converter 1.05
strategist:w.botnet_reac dall (240 games)
  remove 2.67, claim 0.00, liberate 6.95, lib.city 4.24, lib.station 1.46, lib.factory 1.25, claim.city 0.00, claim.station 0.10, claim.factory 0.00, own_on_site 2.00, botnet_std 0.33, botnets_down 0.81, buy_energy 2.71, energy_bought 2.71, move 6.93, armor 0.13, harden_used 2.12, base_absorbed 0.17, aegis 0.07, declined 0.33, factory_amb 0.10, base_amb 0.10, held_off 4.14, canceled 5.55, fac.neutral 2.46, fac.player 1.44, fac.adv 2.03, cubes_conv 2.36, tips_discord 3.85, tips_unity 7.48, converter 1.05
```

## The prompt's number: Buy energy at 5 credits against 4 (the final binary)

`--tag final` and `--tag final5 --set board.city_energy_credits=5`, strategist and greedy; old = 4 (as built), new = 5. Energy bought a game: strategist 2.92 at 4, 2.05 at 5; greedy 2.58 at 4, 2.00 at 5.

| agent | d | games | old win % | new win % | gap | gap SE |
|---|---|---|---|---|---|---|
| greedy | 0 | 48 | 89.6 | 91.7 | +2.1 | 4.7 |
| greedy | 1 | 48 | 79.2 | 77.1 | -2.1 | 7.6 |
| greedy | 2 | 48 | 60.4 | 58.3 | -2.1 | 4.7 |
| greedy | 3 | 48 | 39.6 | 39.6 | +0.0 | 6.0 |
| greedy | 4 | 48 | 39.6 | 43.8 | +4.2 | 4.2 |
| strategist | 0 | 48 | 81.2 | 85.4 | +4.2 | 6.6 |
| strategist | 1 | 48 | 75.0 | 66.7 | -8.3 | 5.8 |
| strategist | 2 | 48 | 70.8 | 62.5 | -8.3 | 5.8 |
| strategist | 3 | 48 | 50.0 | 47.9 | -2.1 | 5.6 |
| strategist | 4 | 48 | 43.8 | 43.8 | +0.0 | 6.0 |
