# 074: research cubes and the four corps (r30)

Read only: nothing here was tuned. Every number is a candidate. Readout: `python scripts/cards/corp_read.py DIR...`.

## The band

Planner, d2, seeds 25700 and 25800, 96 a cell each, pooled to 192, on the frozen build.

| Cell | r29 pooled | r30 | 25700 / 25800 | Win mix of 192 (purge, Unity deck) | Losses (presence, Discord deck) |
|---|---|---|---|---|---|
| c2 | 53.5 | **12.5 ±2.4** | 10.4 / 14.6 | 13, 11 | 118, 50 |
| c3 | 50.0 | **7.3 ±1.9** | 8.3 / 6.2 | 3, 11 | 123, 55 |

A move of 41 and 43 points. A first band on 25700, before a review moved the neutral hexes, read 12.5 / 10.4:
the same place. Engine: 9.4 ±2.1 / 6.2 ±1.7 on the same seeds.

## The structural cause

Planner, 96 seeds a cell, seeds 26000, content what-ifs in a throwaway build, not levers. These seeds have no base
run: the base beside them is the band, on other seeds, so read differences to about ±6.

| What-if | c2 | c3 | Purge wins of 192 | Compute a game | Adversary projects resolved |
|---|---|---|---|---|---|
| base (the band) | 12.5 | 7.3 | 16 | 62 / 100 | 3.2 / 3.8 |
| no adversary compute or Discord after completion (`after` 0) | 29 | 26 | 37 | 40 / 60 | 1.2 / 1.1 |
| a player cube pays 10 credits, as r29 | 49 | 45 | 65 | 59 / 98 | 2.2 / 3.2 |
| both | 62 | 62 | 98 | 39 / 56 | 0.9 / 0.7 |
| r29 (24 seeds, 25900) | 79 / 54 | | | 30 / 48 | 0.3 / 0.1 |

**Two causes, the larger first.**

1. **A player cube paid 10 credits and pays about 3 (the payout the item set at a cube's rate).** With 5.5-8 player
   cubes a game that is 40-55 credits a game gone, plus r29's 2 credits a round per owned station after completion.
   Those credits paid for the push to the purge: purge wins fall from 41% of r29's games to 4.
2. **After completion a corrupted station makes the adversary compute every round.** r29 turned adversary cubes
   into a little Discord and nothing after. Now each corp's corrupted station, and every ambassador diverting a
   station, adds 2-3.3 compute. Compute doubles, adversary projects resolved go from 0.3 to 3.2 a game (c2), and games
   end on presence in round 7.3, where r29 ran 9.

Smaller, unmeasured: neutral stations give arrivals and `FlipStation` targets they lacked, and an adversary ambassador
on one now diverts a cube instead of waiting. The money what-if touches only the player payout, the compute one only
the `after` lists: each changes one thing.

## Cintamani's compute against the nodes'

| Planner, a game | Nodes | All corp compute | Of it Cintamani's after line | All compute |
|---|---|---|---|---|
| c2 | 22.0 | 26.4 | 8.8 | 62.5 |
| c3 | 34.4 | 49.7 | 13.0 | 100 |

Cintamani's line alone is 40% of the nodes' at c2 and 38% at c3, off 4.7 cubes (c2), about 1.9 compute a cube, not the 3
the item reasoned. It is not the strongest adversary effect per game: Horizon's 2 a cube on 6.6 cubes, 13, is more.

## Use, a game

| | c2 planner | c2 engine | c3 planner | c3 engine |
|---|---|---|---|---|
| Completions (of 2 / 3) | 1.99 | 1.99 | 2.99 | 2.96 |
| Mean round completed | 3.4 | 3.4 | 3.4 | 3.5 |
| Paid at completion: player / adversary / neutral | 5.5 / 9.2 / 2.6 | 5.8 / 9.1 / 2.5 | 8.1 / 13.8 / 3.8 | 8.3 / 14.1 / 3.3 |
| Cubes stored | 8.0 | 8.1 | 8.7 | 9.4 |
| Cubes converted | 1.4 | 2.5 | 2.6 | 3.9 |
| Horizon / Cintamani / Onyx uses | 0.52 / 0.87 / – | 1.11 / 1.35 / – | 0.48 / 0.60 / 1.49 | 1.08 / 1.15 / 1.72 |
| Unused with cubes in hand: idle / no aligned city / can't pay | 1.07 / 4.59 / 1.31 | 0 / 4.20 / 1.32 | 1.15 / 9.95 / 1.11 | 0 / 9.24 / 1.04 |
| Neutral stations claimed | 1.15 | 1.17 | 1.78 | 1.93 |
| Stations built | 4.4 | 4.5 | 6.8 | 6.9 |

Completions by round, all cells pooled: round 2 129, 3 868, 4 859, 5 45, 6 6. The item reasoned round 4-5: a claimed or
built station in round 1-2 brings it forward. Biomia is not on the board at 2 or 3 countries: its converter is unread.

**Converters are used, thinly: the planner converts 1.4 of 8 cubes stored at c2. It is reach, not value or the eval.**
The engine leaves none idle. What stops both is no aligned city: the bound city falls, 4.6 and 10 seat-rounds a game at
c2 and c3 hold cubes with a completed card and no city there. Energy for Horizon's is a smaller stop. The planner's idle 1.1
is the eval: a converter nets about 0.2 by its weights. Stations are not idle: the agents claim both and build 2 more a seat.

**Eval changes, for agents to claim and convert at all:** a first screen at the cube's 3-credit rate converted
none at Horizon and Cintamani and built 6.4 stations a game: `research_cube` 0.3 (under what every converter nets),
`station_round` 1 a round left for a seat's first two stations, `station_spare` 1 for the rest.
A held cube's real value is its converter's: the weight is a stand-in.

## What the review found on the first build

The neutral hexes first stood beside a neighbor's node at 3 and 4 countries (Cintamani's at c3, Horizon's at c4): unclaimable
until the node fell. Moved to Horizon 10, Cintamani 2, Onyx 12, Biomia 9, the only hexes in starting reach clear of a node in every
layout; `a_neutral_station_can_be_claimed_at_setup_in_every_layout` holds it. Horizon's 10 stands next to a corrupted city, so
adversary ambassadors reach it soon: a cost of the choice, not measured alone.

## Speed

`per_decision`, planner, c2: 24.9 ms a decision against r29's 34.9, 0.71 × (4 games, one thread): shorter games.
