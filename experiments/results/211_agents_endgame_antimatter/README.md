# 211: sim players sell at the end, save antimatter, convert for a play (no rules change, r92)

Agents only: no rules, content or dials moved, so no `RULES_REVISION` bump (r92, 210's). Built on main a30284b. The
prompt quoted r91; 210 moved the base to r92 (Lift land corruption) before this item ran, so every "before" here is
r92 with the 211 switches off, on the same binary and seeds. Tables: [screen.md](screen.md) (206's spent fresh seeds
45631-45694, 64 a pair, d2-d4), [fresh.md](fresh.md) (fresh 46144-46239, 96 a pair, d0-d4),
[design-fresh.txt](design-fresh.txt). Reader `read211.py`, specs `spec.py`.

## What changed (each a switch, on by default)

| switch | rule (a sentence a player could say) | where |
|---|---|---|
| `sell_rule` 1 | Sell a card you won't play when its credit completes a play: in the adversary deck's last round, a card the plan doesn't play this round; before it, one the plan doesn't play over its rounds; up to three sales, cheapest first, when they let the plan play a card it otherwise wouldn't (2: also a costly standard action the search then takes; 3: a probe, any card outside this round's plan) | `policy.rs` `sell_pick` |
| `am_save` | Keep the antimatter a reachable hand card needs (needs met, credits within two rounds' income): no other spend takes the seat below it | `policy.rs` `am_reserve` |
| `plan_am` | The keep plan counts the antimatter the seat's powers and built Actions make once a round from its next round, and makes a shortfall from cubes at Make antimatter's 4 | `keep_plan.rs` `am_makes` |
| `make_short` | Turn cubes into insight or antimatter (the standard actions) when, after up to three, the plan plays a card this round it can't now | `policy.rs` `make_short_pick` |

## Screen (strategist, d2-d4, 576 games a cell, paired against all off)

| cell | gap d2 / d3 / d4 | sales a seat-game | tier-2 antimatter plays a game (Lance, Commons) |
|---|---|---|---|
| off | — | 0 | 0.075, 0.059 |
| `sell_rule` 1 | +0.5 / +1.0 / +0.5 (±0.5-1.3) | 0.17-0.44, 76-85% in the last two rounds | same |
| `sell_rule` 2 | +2.1 / −4.2 / −2.6 (±2.3-3.7) | 1.4-1.8, 30-35% late | same |
| `plan_am` | +1.0 / −1.6 / −1.0 (±0.7-1.8) | 0 | 0.106, 0.083 |
| `plan_am` + `am_save` | +1.6 / −0.5 / −1.6 (±0.9-1.9) | 0 | 0.104, 0.082 |
| `make_short` | +0.5 / −1.0 / +0.0 (±1.3-2.2) | 0 | same; insight conversions B +0.6-1.3, P +0.1-0.3, R +0.01 a seat-game |
| all four | +1.0 / −1.0 / +0.0 (±1.3-2.6) | 0.11-0.44 | 0.106, 0.089 |
| `w.purge=0` | −1.6 / −7.8 / −4.7 (±3.2-3.8) | | |
| `w.last_node=0` | −2.6 / −5.2 / +0.0 (±2.2-2.9) | | |

Dev smoke (48 games, d2): `sell_rule` 3 sold 5.7 a seat-game, through every round, and read +8.3 ± 7.2.
`am_save` alone changed no game on 48 smoke games: in 24 games (d2) its reserve applied on 21 decisions and vetoed
an action on 4 (a seat rarely holds an antimatter card and antimatter it would spend elsewhere).

## Fresh read (46144-46239, 96 seeds × 3 pairs × d0-d4, 1,440 games a cell)

| d | strategist off | strategist on | gap (paired) | greedy off | greedy on | strategist on vs greedy on | sales a seat-game (strategist on) |
|---|---|---|---|---|---|---|---|
| d0 | 90.3 | 91.0 | +0.7 ± 1.4 | 90.3 | 88.5 | +2.4 ± 2.5 | 0.37 |
| d1 | 76.0 | 77.1 | +1.0 ± 2.1 | 75.7 | 77.4 | −0.3 ± 3.3 | 0.50 |
| d2 | 63.9 | 63.2 | −0.7 ± 2.0 | 64.2 | 66.3 | −3.1 ± 3.4 | 0.23 |
| d3 | 52.4 | 52.4 | +0.0 ± 1.9 | 47.6 | 50.3 | +2.1 ± 3.5 | 0.35 |
| d4 | 22.9 | 22.2 | −0.7 ± 1.8 | 24.3 | 22.6 | −0.3 ± 2.8 | 0.23 |

Sales: 0.34 a seat-game pooled (band 0.05-0.5), 70-86% in a game's last two rounds; never before the plan's dead
cards exist (0 in round 1). Tier-2 antimatter cards a game (strategist, levels pooled), off → on: Annihilation Lance
0.074 → 0.101, Fusion Commons 0.068 → 0.108, Nuclear Option 0.087 → 0.113, Kinetic Interdiction 0.036 → 0.055,
Great Liberation 0.020 → 0.019, Singularity Exploit 0.053 → 0.053. Breakthrough falls 2.6 → 2.4 a seat-game.

**The d3 gap is gone at r92 before this item:** strategist off against greedy off +4.9 ± 3.4 at d3 (d2 −0.3, d4
−1.4). Both strategist weights help (screen: dropping `w.purge` −7.8 at d3, `w.last_node` −5.2), so neither was
retuned (amendment 9445: retune only if it is the cause). 206's −6 was r91 before its dials.

**The Researcher's conversions.** Not a blind spot that costs play: the eval holds a Researcher's cube at 1.2 and its
Patience insight at 1.0 (antimatter 2.0 against 4 cubes = 4.8), so a one-step search never converts, but
`make_short` (convert when it completes a planned play) fires 0.01-0.06 a Researcher seat-game against 0.6-1.1 for the
Builder: Patience banks insight, so a planned play is almost never short of it, and its cubes go to Peer Review,
Disrupt and Lift. Make antimatter stays at 0.00-0.02 (4 cubes for 18 at the anchors is the desperate trade).

**Speed.** The fresh read's own cost, same run, 1,440 games a cell: strategist 1.716 → 1.831 ms a decision (1.07×,
options 1.02×), greedy 1.794 → 1.900 (1.06×). `per_decision.py` on 16 games had read the strategist 1.58× before the
cheap gates (and the same games 1.19× longer); the gates (a hand card short only 1-3 credits, insight or 1-2
antimatter before any plan is run) went in before the fresh read. Baseline `--update`d at r92 with 211's defaults.
