# 278: each prototype against base on the traced games (`breakdown.py BASE FLAG --compare`)

Base: r132, flags off; 268's seeds, the judge. Pooled minutes a round, then the per-game paired mean and SE across the 14 seeds; then each section's change. anytag and hostfix ran on builds with `pull_any_tag.patch` / `host_fixed.patch` applied.

## nopass (cards.no_pass=1)

    d2: 42.4 -> 40.4 minutes a round, pooled (-2.0; per game, paired -2.0 +/- 0.7); rounds a game 8.00 -> 7.93; game 339 -> 321 minutes
      by section: Draft -1.60, Action phase -0.26, Adversary: lines and arrivals -0.13, Sway +0.06
      by kind: decisions -1.72, components -0.13, checks -0.12
    d4: 42.9 -> 41.2 minutes a round, pooled (-1.6; per game, paired -1.6 +/- 1.1); rounds a game 6.36 -> 6.07; game 272 -> 250 minutes
      by section: Draft -1.52, Action phase -0.65, Threats executed +0.12, Adversary: march +0.14, Adversary: threats dealt +0.15, Adversary: compute and projects +0.16
      by kind: lookups +0.10, decisions -1.87, checks +0.14

## cap3 (board.turn_cap=3)

    d2: 42.4 -> 40.7 minutes a round, pooled (-1.7; per game, paired -1.9 +/- 0.7); rounds a game 8.00 -> 8.79; game 339 -> 358 minutes
      by section: Action phase -2.57, Draft -0.16, Cleanup -0.11, Adversary: lines and arrivals +0.12, Adversary: compute and projects +0.15, Production and hosting +0.19, Adversary: march +0.55
      by kind: components -0.15, checks +0.08, decisions -1.63
    d4: 42.9 -> 40.9 minutes a round, pooled (-2.0; per game, paired -1.6 +/- 0.9); rounds a game 6.36 -> 6.21; game 272 -> 254 minutes
      by section: Action phase -1.70, Draft -0.12, Adversary: march -0.11, Production and hosting -0.07, Cleanup -0.06, Adversary: threats dealt +0.16
      by kind: checks -0.15, decisions -1.21, components -0.57

## cap4 (board.turn_cap=4)

    d2: 42.4 -> 41.3 minutes a round, pooled (-1.2; per game, paired -1.4 +/- 0.5); rounds a game 8.00 -> 8.50; game 339 -> 351 minutes
      by section: Action phase -1.65, Draft -0.10, Cleanup -0.06, War +0.06, Production and hosting +0.07, Sway +0.07, Adversary: lines and arrivals +0.16, Adversary: march +0.28
      by kind: checks +0.06, decisions -1.14, components -0.11
    d4: 42.9 -> 42.9 minutes a round, pooled (+0.0; per game, paired +0.1 +/- 1.0); rounds a game 6.36 -> 6.79; game 272 -> 291 minutes
      by section: Action phase -0.73, Adversary: lines and arrivals -0.12, Threats executed +0.11, Production and hosting +0.13, Draft +0.21, Adversary: threats dealt +0.21, Adversary: march +0.25
      by kind: decisions -0.33, checks +0.48, components -0.12

## kept2 (cards.no_pass=1 board.turn_cap=3)

    d2: 42.4 -> 38.4 minutes a round, pooled (-4.0; per game, paired -4.2 +/- 0.7); rounds a game 8.00 -> 8.36; game 339 -> 321 minutes
      by section: Action phase -2.70, Draft -1.73, Threats executed -0.13, Cleanup -0.12, War +0.06, Adversary: march +0.50
      by kind: components -0.46, decisions -3.49, lookups -0.06
    d4: 42.9 -> 40.0 minutes a round, pooled (-2.8; per game, paired -2.6 +/- 0.9); rounds a game 6.36 -> 6.07; game 272 -> 243 minutes
      by section: Action phase -1.90, Draft -1.41, Production and hosting -0.08, Cleanup -0.07, Adversary: lines and arrivals -0.07, Threats executed +0.15, Adversary: threats dealt +0.19, Adversary: march +0.31
      by kind: decisions -2.69, components -0.37, checks +0.22

## anytag (adversary.pull_any_tag=1 (patch))

    d2: 42.4 -> 42.1 minutes a round, pooled (-0.3; per game, paired -0.6 +/- 0.7); rounds a game 8.00 -> 7.93; game 339 -> 334 minutes
      by section: Adversary: threats dealt -0.63, Adversary: lines and arrivals -0.10, Threats executed -0.05, Adversary: march +0.20, Draft +0.26
      by kind: checks -0.47, lookups +0.11, decisions +0.12
    d4: 42.9 -> 43.6 minutes a round, pooled (+0.8; per game, paired +1.0 +/- 1.1); rounds a game 6.36 -> 6.29; game 272 -> 274 minutes
      by section: Adversary: threats dealt -0.35, Adversary: lines and arrivals +0.09, Adversary: march +0.18, Action phase +0.26, Adversary: compute and projects +0.28, Draft +0.30
      by kind: decisions +0.55, components +0.22, checks -0.12, lookups +0.14

## hostfix (board.host_fixed=1 (patch))

    d2: 42.4 -> 42.7 minutes a round, pooled (+0.2; per game, paired +0.2 +/- 0.7); rounds a game 8.00 -> 9.79; game 339 -> 418 minutes
      by section: Action phase -0.13, Threats executed -0.05, Draft +0.10, Adversary: march +0.37
      by kind: components +0.38, decisions +0.29, checks -0.39
    d4: 42.9 -> 43.1 minutes a round, pooled (+0.3; per game, paired -0.0 +/- 0.8); rounds a game 6.36 -> 6.36; game 272 -> 274 minutes
      by section: Action phase -0.31, Production and hosting -0.20, Adversary: lines and arrivals -0.12, Adversary: compute and projects +0.05, Threats executed +0.09, Draft +0.12, Adversary: march +0.30, Adversary: threats dealt +0.39
      by kind: components +0.12, decisions +0.10, checks +0.06

## stdoff (board.std_off=2,3,7,8,9,11,12,13)

    d2: 42.4 -> 43.6 minutes a round, pooled (+1.1; per game, paired +0.7 +/- 0.6); rounds a game 8.00 -> 8.64; game 339 -> 377 minutes
      by section: Threats executed -0.24, Production and hosting +0.06, Adversary: lines and arrivals +0.17, Action phase +1.02
      by kind: decisions +0.48, components +0.37, checks +0.35
    d4: 42.9 -> 43.5 minutes a round, pooled (+0.6; per game, paired +0.5 +/- 0.4); rounds a game 6.36 -> 6.57; game 272 -> 286 minutes
      by section: Threats executed -0.07, Production and hosting +0.07, Adversary: march +0.12, Draft +0.19, Action phase +0.23
      by kind: components +0.13, decisions +0.25, checks +0.22

## jam (adversary.jam_floor=1)

    d2: 42.4 -> 43.0 minutes a round, pooled (+0.6; per game, paired +0.5 +/- 0.6); rounds a game 8.00 -> 8.57; game 339 -> 368 minutes
      by section: Adversary: lines and arrivals -0.06, Adversary: compute and projects +0.05, Action phase +0.05, Adversary: march +0.33
      by kind: components +0.15, decisions +0.08, checks +0.30
    d4: 42.9 -> 44.1 minutes a round, pooled (+1.2; per game, paired +1.3 +/- 0.6); rounds a game 6.36 -> 7.00; game 272 -> 309 minutes
      by section: Adversary: compute and projects +0.05, Production and hosting +0.08, Action phase +0.16, Draft +0.16, Adversary: march +0.25, Adversary: threats dealt +0.26, Threats executed +0.28
      by kind: lookups +0.06, decisions +0.33, checks +0.57, components +0.29

## combo (no_pass + pull_any_tag + host_fixed + std_off)

    d2: 42.4 -> 40.2 minutes a round, pooled (-2.3; per game, paired -2.3 +/- 0.5); rounds a game 8.00 -> 8.64; game 339 -> 347 minutes
      by section: Draft -1.71, Adversary: threats dealt -0.56, Action phase -0.33, Adversary: march +0.34
      by kind: checks -0.89, decisions -1.43, components +0.06
    d4: 42.9 -> 38.6 minutes a round, pooled (-4.3; per game, paired -4.0 +/- 0.9); rounds a game 6.36 -> 5.71; game 272 -> 220 minutes
      by section: Draft -1.68, Action phase -1.30, Adversary: threats dealt -0.49, Production and hosting -0.36, Adversary: march -0.16, Threats executed -0.07, Sway -0.07, Adversary: lines and arrivals -0.06
      by kind: components -0.80, decisions -2.29, lookups -0.10, checks -1.09

