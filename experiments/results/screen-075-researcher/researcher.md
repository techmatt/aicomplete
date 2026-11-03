# 075: the Researcher, the open board rulings, the corps amended (r31)

Every number is a candidate. Readouts: `python scripts/cards/board_read.py DIR... --powers --cards` and
`scripts/cards/corp_read.py`. Planner, d2, 48 seeds a cell unless a row says otherwise; ±1 SE. Screens ran on
seeds 27000-28100; the band on 25900 and 26000; the gate on 28200.

## The band (frozen build)

Planner, d2, 96 a cell each, the frozen build (the two copies differ only in the docs generator).

| Cell | 25900 | 26000 | **Pooled, 192** | Win mix (purge, Unity deck) | Losses (Discord deck, presence, round cap) |
|---|---|---|---|---|---|
| c2 | 68.8 ±4.7 | 64.6 ±4.9 | **66.7 ±3.4** | 98, 30 | 51, 7, 6 |
| c3 | 77.1 ±4.3 | 81.2 ±4.0 | **79.2 ±2.9** | 114, 38 | 19, 12, 9 |

**c3 sits 4.2 points over 75, 1.45 SE: inside 2 SE, so no lever after the freeze.** The structural cause, as measured:
the three-seat game took more from the cheaper cards than the two-seat one (+23 against +8, s5), and the Researcher
adds to c3 (Patience alone 15.6 ±11.1 paired, b1). r30: 12.5 ±2.4 / 7.3 ±1.9.

## Reasoning before simulating

- **Patience** matters when insight binds: a tier-2 keep is 3 insight (2 upgraded) and a big play 2, so a round with
  two tier-2 cards needs 7-10 where income is 5-7. Printed 4 insight like the others, so the carry is the whole
  difference. The planner values an insight a Patience seat holds at `patience` 1.0 (others' 0.5, lost at the round's
  end), and the keep plan carries it into later rounds' budgets.
- **The second presence** needs cubes, which reach a board only after its country's card completes (round 3.4 at r30,
  not 4-5): 3 cubes is one presence every 1.5-3 rounds from round 4 with one or two stations, about 1-2 a game.
- **Field Station** needs a played Industry card: the Researcher prints no Industry, so it waits for a card or the
  card-play track's Industry space. The economy track's third space builds a station free, so the first comes early.
- **Grant** (4 credits a cube) pays only when a completion matters: a cube on the card is a cube nearer completion and
  pays the card's payout (about 9) at completion. The eval values a credit at 0.6 and a cube on a card at 2.
- **Starting credits** (a free variable): 20 by the sheet, 15 frozen (c3 read high; 15 keeps it near the Builder's).

## What the screens found, in order

1. **The first build** (24 seeds, use-075): the six new cards were kept and never played (0 plays in 480 games): the
   eval valued a held cube at 0.3, under any price. `research_cube_power` 1.2 and prices at the cube's rate fixed it.
   The Researcher carried about 20 insight a round: credits bind, not insight.
2. **Income and prices on r30's corps** (s1-s3, discarded with amendment #3106): +3 to +7 credits read c2 29-37,
   c3 37-58; cards 1 cheaper +16 / +33.
3. **The corps (#3106, s4, seeds 27500):** after lines off 33 / 38; payouts about 6 a cube 33 / 46; about 9 a cube
   46 / 48; size 10 moved completion to round 4.1-4.2 and read 40 / 29 (9) and 25 / 29 (6): **size stays 8**.
4. **The base band** on fresh seeds (bb1, 27600, 96 a cell): **27.1 ±4.5 / 35.4 ±4.9**.
5. **Why under 074's 62 / 62** (w1, 27600's first 48, the same build): 10 credits a cube read 54.2 / 62.5 where the
   flavor payouts read 29.2 / 33.3; 074's c3 lineup (Builder, Protector, Builder) 60.4 at 10 credits and 20.8 with
   flavor. **The flavor payouts are the gap, not the Researcher.**
6. **Income and prices on the amended corps** (s5, 27700): base 35.4 / 41.7; cards 1 cheaper 43.8 / 64.6; Builder
   and Protector +3 credits 45.8 / 64.6; both (**X**) 62.5 / 83.3; cards 1 cheaper and 1 energy printed 58.3 / 70.8.
7. **Bigger flavor payouts beside X** (s7, 27900, #3153): X 77.1 / 75.0; credit-led about 10 a cube 39.6 / 45.8;
   flavor-led about 10 a cube (5 energy, 2 cards, 6 chips) 31.2 / 31.2; about 12 a cube 41.7 / 52.1. **X frozen.**
   Energy and chips past what presence and stations take buy little: the most flavor-led payout read lowest.

## Credits held (the 068 pile)

| Read | Credits a seat holds at upkeep, every round | At the last action phase: losses / wins |
|---|---|---|
| bb1 (base) | 4.1 / 4.0 | |
| s5 X | 5.0 / 4.9 | 7.4 / 20.8 (c2), 9.2 / 17.2 (c3) |
| s7 X | 4.8 / 4.5 | |

068 flagged 13-18 at the last action phase. At r31 seats spend down every round; only winners end high, because a purge
ends the game mid-round with that round's income in hand. "Cut printed income, raise card costs" answers a pile r31
doesn't have: the frozen rules went the other way (cards 1 cheaper, +3 credits on two boards), the only change in band.

## Behavior before the freeze (b1, seeds 28100, 32 a cell, the frozen build)

Planner (engine). Paired differences on the same seeds.

| | c2 | c3 |
|---|---|---|
| Win % | 65.6 ±8.4 (59.4) | 75.0 ±7.7 (59.4) |
| Patience: insight carried a game | | 145 (218) |
| Patience off (paired) | | -15.6 ±11.1 (-6.2 ±11.8) |
| Peer Review (second presence), a game | | 0.78 (0.75) |
| Field Station, a game | | 1.81 (2.00) |
| Grant (bought cubes), a game | | **0.00 (0.00)** |
| Wall: strikes stopped a game (troop steps next to a walled facility) | 7.6 (8.5) | 11.3 (12.5) |
| Wall off (paired) | +0.0 ±9.0 (-15.6 ±7.9) | +6.2 ±8.9 (+9.4 ±10.4) |
| Rush Build at 3 Industry / at 2, a game | 0.00 / 0.09 (0.00 / 0.09) | 0.00 / 0.09 (0.03 / 0.12) |
| Shelter at 3 Security / at 2 | 0 / 0 | 0 / 0 |
| Particle Run (3 Power) / at 2 | | 0 / 0 |
| Recovery | 0.22 (0.03) | 0.03 (0.00) |

- **Grant is unused (0.00), plainly:** the eval prices 4 credits at 2.4 and a cube on a card at 2.0, and the planner's
  two-round lines rarely end on a completion the cube decides. The cubes that land on cards come from Field Grant.
- **Wall at the planner is worth nothing measurable;** at the engine it is +15.6 at c2. The planner already keeps troops
  off facilities (hardening, corp troops, removal), so a stopped strike replaces a hit it would have absorbed anyway.
- **The dear powers are unused at 3 tags and at 2:** a seat holds the tags in 1-2 rounds a game of 7-8 (`ai.dear.*`:
  no tags 4-7 seat-rounds a game), and when it can pay, the eval prices three played cards (each a program, tags and
  a building's worth) over a free building, a shelter or an antimatter. Timing: when they are payable it is late
  (rounds 6+), when the payoff is smallest. 2 tags instead of 3 moved Rush Build to 0.09 and nothing else.
- **Patience** is used (145 insight carried a game) and worth 15.6 ±11.1 to the planner; under the engine 6 ±12.

## The new cards (b1, a game, c3 planner)

| Card | Kept | Played | Used |
|---|---|---|---|
| Field Grant | 0.16 | 0.12 | |
| Sample Repository | 0.09 | 0.03 | |
| Preprint Exchange | 0.53 | 0.56 | |
| Inference Cluster | 0.38 | 0.12 | 0.16 |
| Breakthrough Lab | 0.41 | 0 | |
| Policy Institute | 0.47 | 0 | |

The two tier-2 cards need 2 Science tags in play (the board's printed tag counts once) and are never played: kept for
their keep value, not reached. A candidate for 076: 1 Science each.

## Speed

Planner, a game, thread time: c2 19.9 s, c3 51.0 s on the frozen build (r30's band: 11.5 and 34): longer games
(8.8 rounds against 7.3) and a third board with seven powers. The gate took 3.3 h on 8 threads: the first pass 89 min (1,436 of
3,072 games at the cap), the Researcher's checks 17 min, the closes 92 min (23 rows to 96 seeds, 1,200 games).

## The gate (seeds 28200)

`experiments/gate.json` (gate-075: gate-070's 61 checks and `never_convert`, c2 and c3) and `experiments/gate-075-c3.json`
(`always_buy_cubes`, `hoard_insight=3` at c3, on the same seeds), early-stopped from 12 seeds at 2 SE, capped at 48; every
row within 2 SE then closed to 96 paired seeds with `experiments/gate-close-075/` (`gate_close.py plan`, plus c2
`home_only`, which led at 16 seeds, and `always_buy_cubes`). **No check is ahead of the planner by more than 1 SE.**

| Row (96 paired seeds) | Gap to the planner | Gap / SE |
|---|---|---|
| c2 `tier2_rush` | -3.1 ±6.0 | -0.5 |
| c2 `no_powers`, `science_board` | +1.0 ±5.8 | +0.2 |
| c2 `free_build_rush`, `never_remove` | +3.1 ±5.0, ±4.0 | +0.6, +0.8 |
| c2 `home_only` (+25 ahead at 16 seeds) | +4.2 ±5.3 | +0.8 |
| c2 `science_rush` | +4.2 ±5.7 | +0.7 |
| c2 `never_sacrifice`, `never_convert` | +7.3 ±5.6, +8.3 ±4.8 | +1.3, +1.7 |
| c2 `capstone_rush`, `only_tag=5` | +15.6 ±5.8, +25.0 ±5.9 | +2.7, +4.2 |
| c3 `never_second_presence` | -1.0 ±5.8 | -0.2 |
| c3 `free_build_rush`, `never_sacrifice` | 0.0 ±5.3, +1.0 ±6.5 | 0.0, +0.2 |
| c3 `always_buy_cubes` | +2.1 ±6.3 | +0.3 |
| c3 `capstone_rush`, `tier2_rush`, `never_remove` | +5.2, +7.3, +8.3 | +0.8, +1.2, +1.6 |
| c3 `always_max_presence`, `no_powers`, `home_only` | +9.4, +10.4, +11.5 | +1.4, +1.6, +2.0 |
| c3 `science_board`, `wall_turtle` | +22.9, +24.0 | +3.4, +3.6 |

Every other row stopped early, behind: `hoard_insight=3` +31 ±12 at 16 seeds, `never_convert` at c3 +29 ±12 at 28. The
planner on the gate's seeds (96): 63.5 ±4.9 / 72.9 ±4.5. Read with `python scripts/bench/gate_close.py read out/gate
out/gatec3 out/close/* --min-seeds 96`; `scripts/cards/screen.py --gate DIR` for one run and its unspent readout.
