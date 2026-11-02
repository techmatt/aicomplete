# Screen 066: presence powers replace growth (r27)

Planner at d2 unless a line says otherwise. Specs: `experiments/screen-066/`, `experiments/band-066.json`,
`experiments/band.json`, `experiments/gate.json`, `experiments/gate-close-066-*.json`. Screen seeds
18000-19100; band 20200 and 20300; gate 21000.

## r27 as designed against r26, same 24 seeds (18000)

| | r26 c2 | r27 c2 | r26 c3 | r27 c3 |
|---|---|---|---|---|
| planner win % | 83 | 8 | 71 | 0 |
| engine win % | 54 | 8 | 50 | 4 |
| rounds | 9.0 | 7.9 | 9.3 | 8.3 |
| presence placed / lost | 23.6 / 11.9 | 16.8 / 14.6 | 35.2 / 22.2 | 24.9 / 20.9 |
| adversary ambassadors removed | 46 | 17 | 66 | 32 |
| cities freed | 1.4 | 0.3 | 1.6 | 0.3 |
| Sway toward Unity | 28 | 16 | 40 | 24 |
| base Discord | 2.9 | 6.9 | 9.3 | 12.6 |
| threats executed | 0.5 | 2.5 | 0.4 | 1.7 |
| credit income | 278 | 209 | 442 | 332 |

(r27's engine and planner rows here are the first read, before the rollout fix below.)

Three things went at once: a removal went from 1 energy to a 6-credit ambassador, Consolidate's 4 credits and
2-3 energy a round went with no replacement, and placements halved.

## What moved it

All paired on the seeds of their row; 24-32 seeds, about ±10 a cell, so single steps are rough.

| Lever | Read |
|---|---|
| `ambassador_credits` 4, 3, 2 alone | no gain: presence still ended games. **Refused by Matt (2462): the project stays at 6** |
| Track income and 26 start credits, with cheap ambassadors | 29 → 62 c2: the strongest single step |
| Extra energy (Builder's space 2, start 2-3) | none; with income, −16 c2 |
| Buy In 10 credits against 6 (with cheap ambassadors) | c2 0, c3 −19 ±10 |
| Accord 2 Diplomacy against 1 | wins none; used 0.1 a game at 2, 1.7 at 1 |
| `remove_count` 3 against 2, 1, ambassador at 2 | 2 and 3 alike, 1 −25 c2 |
| At `ambassador_credits` 6, `remove_count` 3: raised gains on four one-shots (G1) | c2 +4, c3 −13, ±10 |
| Buildings' Actions that make an ambassador a round (G2) | c2 −4, c3 −9, ±10 |
| A priced ambassador on eight more Diplomacy cards (G3) | +4 / +4, ±10: not adopted |
| 2 start ambassadors | none: not adopted |
| Start credits 26 → 30 | +29 / +21 ±12 |
| Start credits 30, 34, 38 | c2 34, 47, 69; c3 50, 47, 50 |
| Buy In 4, 3 at 38 credits (stopped short, 25-28 seeds) | c2 36, 75; c3 64, 88: not adopted, not a steep price |

**The cards didn't close the gap: start credits and `remove_count` did.** Each ambassador card is played under
once every two games in a 158-card deck with one copy of each, so a raised gain reaches few games. The team
still buys 12 ambassadors a game at c2 (24 at c3) from the 6-credit project, about 70 credits, which 38 start
credits and the track income pay for: the project is the rule of these builds, not the exception.

## The band (frozen build; fresh seeds; nothing picked on them)

| Seeds | c2 | c3 |
|---|---|---|
| 20200, 48 | 67 | 62 |
| 20300, 48 | 52 | 60 |
| **96** | **59 ±5** | **61 ±5** |
| 20200, Buy In at 10, 48 | 54 | 58 |

Target 60-75: c2 just under, no lever added. On the same 48 seeds Buy In at 10 reads 54 / 58 against 67 / 62.

Void reads, kept for the record: 52 / 71 (seed 20000) and 62 / 79 (seed 20100), then 59 / 76 on 80 seeds, on
`ambassador_credits` 2, before amendment 2462.

## Powers, a game (frozen, seed 20300)

| | Retool | Deploy | Buy In | Scrap | Lockdown | Deploy | Accord | Stand Down |
|---|---|---|---|---|---|---|---|---|
| c2 planner | 0.4 | 7.6 | 1.8 | 2.8 | 1.2 | 7.9 | 2.1 | 0.7 |
| c3 planner | 0.6 | 16.1 | 3.3 | 3.6 | 1.1 | 8.2 | 2.1 | 0.6 |

| c2 engine | 1.1 | 8.4 | 1.6 | 2.7 | 1.3 | 9.0 | 1.8 | 0.8 |
| c3 engine | 1.0 | 16.0 | 3.5 | 3.9 | 1.4 | 8.5 | 1.8 | 0.5 |

(c3 seats two Builders.) Placements a game 21 / 32; r26 took 24 / 35. The engine wins 33 / 56 on these seeds, the
planner 52 / 60. Planner and engine first differ at a power in 39 of 48 c2 games and 44 of 48 at c3
(`bench/first_divergence.py`), counting a power against a play or a payment as well as two different powers: the
powers are where the search earns its gap. Scrap and Stand Down are used about 2.8 and 0.7 times a game, Bulwark's hardening 8.5:
the Protector's energy has somewhere to go, and Sentinel Post's +1 energy on top of the bound city gives it the
second energy that the Builder has to buy; Buy In is used 1.8 a game against Accord's 2.1.

A replay of seed 20300 on the final build (only test code changed after the freeze): same 96 games
(`bench/same_results.py`). Cost: planner 43.6 ms a decision against r26's 40.9 (1.07), with 15% more decisions.

## The gate (`experiments/gate.json`, seed 21000, 8 seeds a check, early-stopped), then its closes

Paired, planner minus check, points: above 0 the planner leads. Done-when: no check ahead by more than 1 SE.
Closes are on the gate's own seeds, 21000..=21047, so they overlap its first 8; 21048..=21095 only for the
checks not clearly behind.

| Check | c2 | c3 |
|---|---|---|
| planner (win %) | 58, 96 seeds | 57, 96 seeds |
| always_max_presence | +7.3 ±6.7 (96) | +16.7 ±10.5 (48) |
| never_second_presence | +20.8 ±9.4 (48) | **−2.1 ±6.6** (96) |
| no_powers | +29.2 ±9.4 (48) | +3.1 ±5.6 (96) |
| tier2_rush | **−1.0 ±6.0** (96) | +20.8 ±8.4 (48) |
| science_board | +18.8 ±9.3 (48) | +10.4 ±7.0 (96) |
| never_remove | +27.1 ±7.7 (48) | +18.8 ±7.1 (48) |
| unity_farm | +20.8 ±9.9 (48) | +37.5 ±9.7 (48) |
| abandon | +47.9 ±8.9 (48) | gate only: +25 ±24 (8) |

Passes the bar: no check is ahead by more than 1 SE. Two sit level. **At c3 the second presence is worth nothing
measured** (never_second_presence 59 against 57), and neither are the sacrifice powers (no_powers 54 against 57).

Dropped to fit, the prompt's lowest stakes: the `never_tag` checks stayed at the gate's 8 seeds, where c2 tags 1-3
read level or ahead (50, 50, 62 against 50) and c3 all behind. Every other check was clearly behind in the gate.
