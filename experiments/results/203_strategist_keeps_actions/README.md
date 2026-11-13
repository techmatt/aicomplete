# 203: the strategist keeps and plays what it can afford (r89, agents only)

No rules, content or dial change: one agent behavior, `kept_rule` 3 (`agents/policy.rs`, `behavior_table!`), on for
every agent that has behaviors (the strategist and greedy share it).

**The rule, one sentence:** where the search would pass, take a standard action, sell a card or take the antimatter
action, play instead the hand card the keep plan plays this round (`keep_plan::plan_now`: the plan's round 0 from
what the seat holds now, each card worth its price at the plan's rates). Threat counters, powers, Uses and the
search's own plays stand.

## The gap, measured fresh on r89

The prompt's numbers (199 at r87: ~10.8 held-back dead keeps a game; 200 at r88: plants played 0.03-0.08) still
hold at r89: held back 7.2-10.9 a game by level; Penning Trap Array and Antiproton Foundry together kept 0.48-0.77 and
played 0.05-0.13 a game. **The cause is the play choice, not the budget:** a held-back keep was playable at the
end of some action phase (`keep_ok`), so the seat could pay and the search chose not to. The flat heuristic
scores a card in hand at `hand` 3 + `hand_cost` 0.4 × cost, and a built building without income at
`built` 4 + `built_cost` 0.5 × cost + `ability` 0.5 a round, less the credits paid (0.6 each to the cap of 12).
At cost 10-18 that difference is near zero or negative, so the search found a standard action or a pass
as good as the play. Action buildings and programs are most of the dead keeps (`tables.md`, by card).

## Screens (dev seeds 36800-36831, 96 games a level, paired on base)

| rule | d0 | d1 | d2 | d3 | held back a game | plants played |
|---|---|---|---|---|---|---|
| 1: instead of a pass | −6.2 ± 4.4 | +2.1 | −3.1 | +2.1 | ~unchanged | 0.15-0.24 |
| 2: before any pick | +0.0 | +1.0 | −9.4 ± 6.1 | −9.4 ± 5.8 | −2.0 | 0.30-0.57 |
| **3: instead of a pass, standard action, sale or antimatter action** | +3.1 | +8.3 ± 5.3 | +4.2 | +4.2 | −2.1 to −3.4 | 0.34-0.46 |
| 4: 3, and instead of a power | −2.1 | +1.0 | +4.2 | +2.1 | −2.1 | 0.29-0.47 |

Rule 2 bought cards ahead of threat counters and lost at d2-d3; rule 1 seldom fired (by the pass the credits
were spent). Greedy on and off (dev, 96 a level): +0.0 / +2.1 / +7.3 / +1.0, held back −2.4 to −3.1.

## Fresh read (45343-45438; strategist 96 seeds a pair, greedy 48; d0-d4; against r89 main 780bf6c)

| agent, d | win r89 → new | gap ± SE (paired) | held-back dead keeps a game | plants kept / played a game | plant Action uses |
|---|---|---|---|---|---|
| strategist d0 | 83.7 → 87.2 | +3.5 ± 2.7 | 10.90 → 8.62 | 0.77 / 0.13 → 0.81 / 0.33 | 0.27 → 0.33 |
| strategist d1 | 75.3 → 78.8 | +3.5 ± 3.3 | 10.80 → 8.55 | 0.73 / 0.12 → 0.83 / 0.37 | 0.16 → 0.28 |
| strategist d2 | 54.9 → 61.8 | +6.9 ± 3.3 | 9.66 → 7.38 | 0.72 / 0.08 → 0.73 / 0.36 | 0.15 → 0.20 |
| strategist d3 | 39.6 → 48.3 | +8.7 ± 3.1 | 8.02 → 6.66 | 0.54 / 0.10 → 0.63 / 0.25 | 0.07 → 0.21 |
| strategist d4 | 22.2 → 23.6 | +1.4 ± 2.7 | 7.20 → 5.73 | 0.48 / 0.05 → 0.53 / 0.23 | 0.05 → 0.17 |
| greedy d0 | 88.9 → 88.9 | +0.0 ± 3.3 | 9.91 → 8.06 | 0.78 / 0.15 → 0.81 / 0.43 | 0.26 → 0.44 |
| greedy d1 | 76.4 → 82.6 | +6.2 ± 4.3 | 10.47 → 8.51 | 0.72 / 0.10 → 0.82 / 0.42 | 0.14 → 0.31 |
| greedy d2 | 70.8 → 63.2 | −7.6 ± 4.0 | 8.77 → 7.24 | 0.66 / 0.10 → 0.76 / 0.39 | 0.12 → 0.24 |
| greedy d3 | 45.1 → 49.3 | +4.2 ± 4.5 | 8.50 → 6.83 | 0.57 / 0.09 → 0.60 / 0.26 | 0.12 → 0.11 |
| greedy d4 | 25.7 → 22.2 | −3.5 ± 3.7 | 7.15 → 6.22 | 0.51 / 0.07 → 0.55 / 0.27 | 0.07 → 0.22 |

Kept cards played a game rise by about half (strategist 6.9-10.9 → 9.8-15.6). Dead keeps that never had the
resources barely move (2.2-2.8 → 2.2-2.7). `design_check.py`'s kept-not-played list falls from 33 cards to 3.

**Bands (strategist; reported, not recalibrated):** d0 87.2 under 90-100 (r89 83.7 was under too); d1 78.8,
d2 61.8, d3 48.3 in band. d2 and d3 rise 6.9 and 8.7, to the band's upper half. Greedy reads beside it: d1 82.6
over 70-80, d2 −7.6 (1.9 SE).

## Cost

`per_decision.py`, kept_rule 0 against 3 on one binary (c2, d2): strategist 1.0 → 1.5 ms a decision (1.51 ×, 16
games: the plan runs at each pass or standard-action pick with a play legal), greedy 0.92 ×, engine 0.85 ×, planner
1.12 × (2 games). Kept: the strategist still decides in under half of greedy's time. `speed.py check` gave no verdict
(other games at the same revision), so the baseline was `--update`d on this build.

Every agent with behaviors plays the rule (engine and planner too); only the strategist and greedy were read.

## What is left

- 5.7-8.6 held-back dead keeps a game remain: cards the plan keeps for a later round and, re-read at that round, values
  at or under their price, or plays behind other cards. The keep plan's budget still ignores the standard actions and powers
  that come after the cards. That spending is now mostly the credits left once the plan's cards are paid, so
  no lever was built for it. A keep-side lever, `plan_soon` (keep a card only if the plan plays it within 1 or 2 rounds),
  cut held-back dead keeps by keeping less and lost: 1 round −13.5 / −10.4 / −6.2 / −5.2, 2 rounds −2.1 / −9.4 /
  +1.0 / −4.2 (dev seeds, on kept_rule 3, `tables.md`). It was removed. Fewer dead keeps is not the goal when they
  come from fewer keeps.
- Plant Actions are used 0.11-0.44 a game: at r89 antimatter has almost no spend (Annihilate off; design_check:
  gained-not-spent 4%), so a built plant is worth little until the next rules item gives antimatter a payoff.

## Files

`keep_read.py` (dead keeps, plants, paired win gap), `dead_cards.py` (held back by card), `cost_read.py` (agent ms a
decision), `read_spec.py` (specs `experiments/farm/f203-*.json`), `tables.md` (every table). Seeds: dev 36800-36831;
fresh 45343-45438 spent (ledger next free 45439).
