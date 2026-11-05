# 116: the strategist's parameter sweep, r45

How much better than its hand-set values could the judge (the strategist, `engine:strategy=1`) play
at r45, and which of its parameters matter? Report only: no default changed. All tables are in
[summary-r45.md](summary-r45.md), which `python scripts/tune/agent_sweep.py` writes (recipe:
[scripts/tune/README.md](../../../scripts/tune/README.md)).

**Setup.**
- Two players at d2, on the three board pairs (B+P, B+R, P+R). Greedy is unchanged, as the reference.
- Sweep seeds 33932-34031: 300 games a setting, every setting on the same seeds.
- Confirm seeds 34032-34127: 288 games a setting, paired against the hand-set strategist.
- r45 binary, 4 threads at below-normal priority, 1.2-2 games/s on a busy box (2.5-4 min a setting).
- 21 settings in the search, 13 more in the profile (the rest were cached), 3 in the confirm: about
  12,000 games in 2 hours.

## The parameters

| parameter | what it is | hand-set | grid |
|---|---|---|---|
| `w.insight` | an insight held, in heuristic points | 0.5 | 0.2 0.35 0.5 0.75 1.0 |
| `w.energy` | an energy held (also the keep plan's energy price, through `plan_rates`) | 0.8 | 0.4 0.6 0.8 1.1 1.5 |
| `w.credits` | a credit held, up to `credit_cap` | 0.6 | 0.35 0.45 0.6 0.75 0.9 |
| `w.chips` | a chip held | 2 | 1 1.5 2 2.75 3.5 |
| `w.table` | a card's worth: points per value-table credit | 0.6 | 0.35 0.45 0.6 0.75 0.9 |
| `w.horizon` | income valued over the rounds left | 1 | 0.5 0.75 1 1.25 1.5 |
| `w.credit_cap` | credits past this are worth `credit_idle` (0.15) | 12 | 6 9 12 16 20 |
| `w.packet_keep` | keep threshold: share of the keep price charged in the draft | 0.5 | 0 0.25 0.5 0.75 1 |
| `plan_credit` | keep threshold: what a credit paid for a keep costs the keep plan | 3 | 1 2 3 4 5 |
| `w.strat_scale` | the strategy weights: how far the multipliers reach from 1 (new, 116) | 1 | 0 0.5 1 1.5 2 |

`w.strat_scale` is the one new knob. At 1 it plays game for game as before: 144 of 144 games were the
same on strategist, greedy and planner, old binary against new (`same_results.py`). At 0 every
multiplier is 1, but the strategy is still chosen, and the keep plan's `plan_reach` still reads its
tags. So 0 is not quite `strategy=0`.

The value table's prices (`values.json`: insight 2.5, energy 4.5, chips 1.5) can't be set from a spec
(they are compiled in, process-wide). The sweep varies the heuristic's own terms for each resource.

## Search

The plan was one pass of coordinate search at ±2 grid steps, then a second pass. At the measured
1.2 games/s that is 40 settings a pass, which doesn't fit. So it ran one pass at ±1, accepting a step
past 1.5 SE.

Only `w.credit_cap` moved, 12 → 16, at +4.3 ±2.6. That is the best of 20 tests, each with a one-in-15
chance of passing 1.5 SE on noise alone. No other ±1 step gained by more than +2.3.

## Held-out confirm (34032-34127, 288 games a setting)

| | hand-set | best (`w.credit_cap=16`) | greedy |
|---|---|---|---|
| won % | 74.3 | 76.4 | 57.3 |
| credits left a seat at game end | 47.6 | 48.4 | 14.7 |
| rounds | 6.93 | 6.88 | 5.25 |

- **Best less hand-set: +2.1 ±2.6** (paired). By pair: bp +0.0 ±4.4, br −3.1 ±4.0, pr +9.4 ±4.9.
  On the sweep seeds it was +4.3 ±2.6: the held-out gain is half of that and within 1 SE of zero. It
  is not confirmed.
- **Strategist less greedy:** +17.0 ±3.6 hand-set, +19.1 ±3.6 swept (sweep seeds: +15.7 ±3.5, +20.0 ±3.3).
- **Credits left:** +0.8 ±3.1 a seat, swept less hand-set.
  The strategist ends with about 48 credits a seat, greedy with about 15.

## Which parameters matter

This is ±1 grid step around the best (`w.credit_cap=16`, the others hand-set), on the sweep seeds,
paired: won-% change from the best.

**Read it against the hand-set level, not against 0.** The best was picked on these seeds at +4.3
over hand-set, so every neighbor starts about 4 points down from selection alone. The hand-set
setting itself reads −4.3 here. A step matters when it falls clearly below that level.

| parameter | down one step | up one step | reading on these seeds alone (not held out: see below) |
|---|---|---|---|
| `w.horizon` (1) | 0.75: +0.0 ±2.9 | 1.25: **−14.3 ±2.9** | matters, one way: valuing income more loses |
| `w.credits` (0.6) | 0.45: −1.0 ±2.6 | 0.75: **−10.7 ±2.8** | matters, one way: valuing a held credit more loses |
| `w.table` (0.6) | 0.45: −6.7 ±2.9 | 0.75: −7.0 ±2.8 | some: both ways about 2.5 below hand-set |
| `w.credit_cap` (16) | 12: −4.3 ±2.6 | 20: −7.3 ±2.5 | some: 20 below hand-set by about 3 |
| `w.energy` (0.8) | 0.6: −3.3 ±2.5 | 1.1: −6.0 ±2.8 | weak: up costs about 2 below hand-set |
| `w.insight` (0.5) | 0.35: −3.7 ±1.9 | 0.75: −5.0 ±2.1 | flat at hand-set level |
| `w.chips` (2) | 1.5: −4.3 ±2.4 | 2.75: −2.0 ±2.4 | flat |
| `w.packet_keep` (0.5) | 0.25: −4.7 ±2.1 | 0.75: −4.3 ±2.3 | flat at hand-set level |
| `plan_credit` (3) | 2: −2.3 ±2.1 | 4: −1.7 ±1.5 | flat |
| `w.strat_scale` (1) | 0.5: −1.7 ±2.6 | 1.5: −4.0 ±2.5 | flat |

The script's own *matters* column (any point more than 2 SE below the best) flags seven of the ten.
That column doesn't correct for selection.

**The large readings don't hold on other seeds.** Three points were replayed alone against the
hand-set strategist, two on the confirm seeds and one also on fresh seeds:

| step from hand-set | sweep seeds (around the best) | confirm seeds 34032-34127 | fresh 34128-34223 |
|---|---|---|---|
| `w.horizon` 1.25 | −14.3 ±2.9 | −1.7 ±2.8 | |
| `w.credits` 0.75 | −10.7 ±2.8 | +1.0 ±2.8 | |
| `w.horizon` 0.75 | +0.0 ±2.9 | +5.9 ±2.7 | −1.4 ±2.7 |

Specs: `check-steps.json`, `check-horizon.json` (288 games each cell). The sweep-seed column was read
around `w.credit_cap=16`, the others around hand-set, so that column changed two things. Still, the
held-out reads put both large effects within 1 SE of zero. The one held-out lead (+5.9) was a
post-hoc pick, and it went away on fresh seeds.

**Conclusion: no parameter matters at one grid step.** All ten are on a plateau around their hand-set
values: `w.insight`, `w.energy`, `w.credits`, `w.chips`, `w.table`, `w.horizon`, `w.credit_cap`,
`w.packet_keep`, `plan_credit` and `w.strat_scale`. A move of 25-50% in any one of them shifts the win
rate by less than this design can see, about ±5 points. The 10-14 point readings on one seed set are
selection and seed-set noise. Their size is a warning: a 300-game screen of many settings will throw
up a 3-4 SE outlier. Wider steps (the grid's outer points) were not played.

**Credits left at game end** move little across the whole profile: 42-52 a seat at every point
(summary-r45.md, the figures in brackets). The strategist's 48 unspent credits a seat aren't a
valuation these ten parameters set. Valuing credits lower (0.45) still leaves 47. The one point that
spends more, `w.horizon` 1.25, leaves 42. A likely cause, not tested here: late in the game there
is nothing worth buying with them at the agent's prices. If so, that is a question for the economy,
not for the agent's weights.

The outer grid points (±2 steps) were not played: the second pass and the ±2 reach didn't fit the
time (see *Search*).

## What the swept agent does differently

On the confirm games, paired, the swept setting (`w.credit_cap=16`) differs from hand-set only in small
ways. It holds about one credit more after round 1 (7.7 against 6.7, +0.9 ±0.2) and after round 2
(16.1 against 14.7, +1.3 ±0.5). It plays about the same cards (6.28 against 6.35 in round 1). Draft
picks shift a little: fewer Deep State Ledger picks, more Grad Student Pool. The full list is in
summary-r45.md.

**As a player's rule:** the swept agent's only difference is *keep a cushion of up to about 16
credits rather than spending down to 12*. The held-out seeds don't support it (+2.1 ±2.6). No other
rule came out of the sweep either: within a step of its values the judge plays as well as it can on
these terms. Its real gap is the credits it ends with, about 48 a seat whatever these weights say.

## Decisions

- The swept agent is the strategist; greedy is the reference and unchanged. Seeds are the prompt's
  blocks, free in the ledger at r45.
- Coordinate search, not CMA-ES. At 300 games a setting one SE is about 2.5 points, and CMA-ES spends
  its samples on settings too close together for that noise to separate. A coordinate pass also gives
  each parameter's own read.
- Grids are five points per parameter. The hand-set value is the middle point, so the ±1 pass tests
  plausible moves of about 25-50%.
- The confirm ran before the profile, so the held-out read was safe whatever the time left.
