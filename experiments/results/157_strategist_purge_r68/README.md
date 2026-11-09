# 157: the strategist plays the purge, and the card tiers settle (r68)

Matt's rulings (2026-11-09): tiers count the current tip, both sides; no tier III frees a city directly; the
strategist stays the judge, its weights retuned for the purge first. `RULES_REVISION` 67 → 68, bumped from the
constant (the prompt also said r68).

Scripts: [specs157.py](specs157.py) (screens and reads), [run_read.sh](run_read.sh) (a read's three sets on one
binary), [ladder157.py](ladder157.py) (151's ladder, d1's band 70-80), [ends157.py](ends157.py),
[endgame157.py](endgame157.py) (the probe by Unity step, plus the last cities' stacks), [gap157.py](gap157.py)
(what late liberations would do), [partial157.py](partial157.py) (win % from a run still going, `--seeds`),
[counters157.py](counters157.py), [tiers157.py](tiers157.py), [apply_tiers.py](apply_tiers.py) with
[tier3.overlay](tier3.overlay) (the tier-III rewrite; `apply_overlay.py` refuses alignment.json's layout). Tables:
[final.md](final.md), [screens.md](screens.md), [tiers.md](tiers.md), [endgame.md](endgame.md), [gap.md](gap.md),
[greedy.md](greedy.md), [oracle.md](oracle.md), [per_decision_tier.md](per_decision_tier.md),
[s2_d2_counters.md](s2_d2_counters.md).

## The rules

- **Current-tip tiers** (`alignment::next_tier`, `tier_of`): a side's tip numbered n (1-based) resolves tier I for
  n < 3, II for 3-5, III from 6 (`tier_at` 3, 6 unchanged, read against the tip's own number). Unity tiers come one
  tip sooner; Discord's 6th tip now resolves a tier III (0.09-0.23 a game at d1-d3 in the final read) before the 7th
  loses. d4's removed cards still count as tips made.
- **No tier III frees or clears a city outright.** The four that liberated a city: Volunteer Corps (one player gains
  8 ambassadors), Popular Mandate (one player gains 6 and moves up to 3 onto pieces in reach), Citizens' Assembly
  (sway 3, one player gains 6), Reclamation (cleanse 2 sites, then up to 6 adversary ambassadors around a corrupted
  city in reach). The six that removed every adversary ambassador around a corrupted city now remove up to 6 (Think
  Tank, Ceasefire, Town Hall Accord, Mutual Aid, Disarmament Talks, Network Cleanup). All local: one player, in reach.
  The players still reach the city and Liberate it.

## Give / get (values.json, two players; a tip brings the card, so the give is 0; [tiers.md](tiers.md))

Each changed tier III, then its tier II: Volunteer Corps 32 (was 45; II 20), Popular Mandate 30 (53; 16), Reclamation
42 (52; 30), Citizens' Assembly 36 (50; 16), Think Tank 45 (39; 33), Ceasefire 26 (20; 11), Town Hall Accord 20 (14;
8), Mutual Aid 30 (24; 18), Disarmament Talks 56 (50; 33), Network Cleanup 40 (34; 18). Every tier III still reads
above its tier II. The six "up to 6" cards read higher than before only because the table priced "every" at
`token_pile` 3; in play a late city holds ~5 to clear (the end-game probe). Unity means 11 / 19 / 33 (was 34). Each
changed card resolves 0.19-0.58 times a game in the final read.

## The strategist (dev seeds 36800-36831, 96 games a level, each change on and off; [screens.md](screens.md))

Greedy on these seeds: **90.6 / 82.3 / 66.7** (d1-d3). The strategist at r68 (156's weights): 70.8 / 62.5 / 42.7.

| Change (on top of `engine:strategy=1`) | d1 | d2 | d3 | Cities freed (d2) |
|---|---|---|---|---|
| none (r67's strategist) | 70.8 | 62.5 | 42.7 | 2.26 |
| **`w.clock=0`** (kept) | **74.0** | **69.8** | **49.0** | 2.56 |
| `w.table_ambassador=2` / `=3` | 77.1 / 75.0 | 60.4 / 70.8 | 40.6 / 37.5 | 2.42 / 2.70 |
| clock 0 + `table_ambassador=2` | 76.0 | 71.9 | 43.8 | 2.62 |
| clock 0 + `freeing=4` / + `table_ambassador=2` / `freeing=6` | 64.6 / 74.0 / 55.2 | 60.4 / 57.3 / 43.8 | 37.5 / 37.5 / 22.9 | 2.05 / 2.11 / 1.54 |
| clock 0 + `win_lag=3` / + `table_ambassador=2` | 72.9 / 78.1 | 68.8 / 69.8 | 45.8 / 46.9 | 2.66 / 2.85 |
| clock 0 + `strat_scale=0` / `0.5` | 74.0 / 74.0 | 63.5 / 62.5 | 45.8 / 40.6 | 2.45 / 2.54 |
| clock 0 + `horizon=0.5` / `corrupted=45` | 72.9 / 71.9 | 62.5 / 59.4 | 47.9 / 40.6 | 2.54 / 2.62 |
| **greedy's flat weights, with strategy** | **89.6** | **83.3** | **66.7** | 4.11 |

- **Kept: the deck clock off for the strategist only** (`STRATEGIST` in `agents/mod.rs`: `strategy=1,w.clock=0`; the
  engine, planner and oracle keep it). The clock charged each round the team's Unity rate left it short of the purge's
  6th tip, so it bought Unity over liberations. +3.2 / +7.3 / +6.3.
- **Not met: the strategist at or above greedy.** No horizon-mode weight closes it: the best stays 13-18 below greedy
  at d1-d3. The engine on greedy's flat weights with strategy matches greedy (89.6 / 83.3 / 66.7 against 90.6 / 82.3 /
  66.7) but costs 20-31 s a game against the strategist's 1.5-3.6 s, so it can't judge screens (a read set would take
  ~45 min). The gap is structural, not a weight: in horizon mode (the value table) an ambassador is worth 2.4 points
  against 3.6 for the 6 credits that buy one, so the strategist buys 1.0-1.3 a game to greedy's 2.7, plays Capital
  piles (Conglomerate HQ thresholds 4-5 a game, greedy 0.2), and frees 2.5 cities a game to greedy's 4.1-4.6
  ([s2_d2_counters.md](s2_d2_counters.md)). Raising `table_ambassador` alone buys more but frees no more. Next step for
  a later item: a purge term in horizon mode (ambassadors toward the Liberate bill of the cities in reach, priced at
  the cities they free), or a faster flat search to make the flat-weight strategist affordable.

## Greedy's tier-read slowdown ([per_decision_tier.md](per_decision_tier.md))

Not the tier read. 156's 7.6x came from 2 games a side on a busy box (70 s a game against 8 s). On 6 games a side (seeds 1000-1005,
d2, one thread): **1.60x** per decision at r67 (156's binary) and **0.74x** at r68. Greedy's cost per game is
heavy-tailed (6-70 s): the profile split puts 86-91% of it in `score_choose` (prompt answers, 2-7 ms each, applies
38-50 us), which the tier read doesn't touch. Nothing cheap to fix in the tier read; the greedy row ran on 24 seeds a
level (8 a pair, 39304-39311).

## Dials (dev screens; [screens.md](screens.md))

With the new strategist the dev read was 87.5 / 74.0 / 69.8 / 49.0. `compute_income` 4,4,6,8,10 → **3,4,8,8,10**
(d0 92.7, d2 61.5 on dev; d2 at 9: 60.4, at 10: 54.2). `ambassador_income` 2 at d2 read 50.0 (too far); d0's
ambassador income 0 read 94.8. Counter dials untouched.

## Final read: r68 on fresh seeds 39304-39447 ([final.md](final.md))

Three 48-seed sets, the three pairs, d0-d3, 432 games a level, the end-game probe on every loss. The dials frozen
above (`compute_income` 3,4,8,8,10). r67 (156's binary, its configs and its strategist) on the same seeds: 90.5 / 77.5 / 70.6 / 41.7, paired gaps +0.2 ±1.8 / +0.5 ±2.2 / -5.1 ±2.6 / -0.2 ±2.5 ([ladder.md](ladder.md), with the standing measures). **The pair changes three things at once** (the rules, the strategist's clock, the dials), so a gap says nothing about any one of them.

| Level | r68 (pooled) | SE | sets | band | verdict |
|---|---|---|---|---|---|
| d0 | 90.7 | 1.4 | 88.9 / 91.0 / 92.4 | ≥90 | met (0.7 over the edge) |
| d1 | 78.0 | 2.0 | 75.7 / 75.7 / 82.6 | 70-80 | met (2.0 under the edge) |
| d2 | 65.5 | 2.3 | 64.6 / 64.6 / 67.4 | 55-65 | **unmet** (0.5 over: 0.2 SE) |
| d3 | 41.4 | 2.4 | 38.2 / 40.3 / 45.8 | 40-50 | met (1.4 over the edge) |

Every level is pooled from three independent sets. **Win routes:** every win is the purge at step 2 (6+ Unity tips)
but 4 at d0 (step 1). **Losses:** round cap 40 / 73 / 90 / 228, Discord deck 0 / 22 / 59 / 25, nodes 0. Discord's
tier III now resolves 0.0 / 0.11 / 0.23 / 0.09 times a game (set a).

**Beside it** (24 seeds a level, 39304-39311): greedy **95.8 / 83.3 / 75.0 / 62.5**, the strategist on the same
games 87.5 / 58.3 / 54.2 / 29.2 (greedy +8 / +25 / +21 / +33; [greedy.md](greedy.md)). The oracle row (same seeds, d3 and d4; [oracle.md](oracle.md)): **d3 100.0** (24 of 24; the strategist 29.2), **d4 37.5** (9 of 24, SE 9.9; the strategist 0.0), far under Matt's d4 95 (r66: 100 / 95.8): 11 of its 15 d4 losses are the Discord deck. r67's oracle on the same d4 seeds read **54.2** (13 of 24, 10 of 11 losses the Discord deck): d4 fell below Matt's 95 with 156's redesign (r66: 95.8); r68's current-tip tiers may cost it more (-16.7, inside two SE of 24 games).

## The gap readout, for the tier-2 card item ([endgame.md](endgame.md), [gap.md](gap.md); the final read)

- **By Unity step (the probe on every loss).** d3 loses 253 of 432: 119 below step 2 (fewer than 6 Unity tips: 36
  at 0-2, 83 at 3-5), with 3.8-5.6 corrupted cities and 70-95 troops still counting, out of reach of liberations;
  134 at step 2 (131 the round cap), with 2.5 corrupted cities left. d2: 59 below step 2, 90 at step 2 (2.2 cities
  left); d1: 39 and 56 (2.0); d0: 5 and 35 (1.4).
- **Where lost games end (step 2).** The cities standing are fresh corruptions: 0.3 adversary ambassadors on them, so
  the bill is the flat 5 a city (12.8 at d3's end). At the snapshot (third-last round) the team held 4.0 ambassadors at
  d3 against a 20.1 bill for the cities then (d2 5.8 / 21.8, d1 6.2 / 18.8). A loss with one city left held 4.2 at
  d3 against 5; at d0 it held 17.9 against 5, so d0's few losses are not short of ambassadors: the last city corrupts
  too late to answer.
- **What would close d3.** Step-2 losses by cities left: 44 with 1, 33 with 2, 25 with 3, 32 with 4+. One more city
  freed in the last three rounds of each step-2 loss (5 ambassadors) lifts d3 to 51.6, two (10) to 59.3, three to
  65.0. To reach the band's middle (45) takes 16 more wins: one more liberation (5 ambassadors) in the last three
  rounds in 16 of the 44 one-city losses (they held 4.2 at the snapshot). The
  119 losses short of 6 Unity tips need Unity sooner, not liberations: about half of d3's losses.

## The time gate: d2 re-read after the final read

d2 missed by 0.5 (0.2 SE) with ~2 hours left, so one d2-only dial: `compute_income` d2 8 → **10** (dev 61.5 → 54.2;
9 read 60.4), confirmed on the next free seeds, **39448-39591** (the dev seeds read 10 at 54.2, below the band; the fresh read
is the verdict, and dev and fresh disagree here by ~10 points) (three 48-seed sets, d2 only, the dial alone moving
nothing at d0, d1 or d3): **63.9** pooled (66.7 / 63.2 / 61.8, SE 2.3, 432 games), **met** (1.1 under the edge).
Final dials: `compute_income` 4,4,6,8,10 → **3,4,10,8,10**; `ambassador_income` 1,1,1,1,2, `counter_bonus_credits` 4,
`counter_more_credits` 0 unchanged. **Flag:** d2's compute income (10) now tops d3's (8); d3 stays harder by its
shorter deck (`stage2_removed`) and counter dials, and reads 22 points lower.

**Final ladder: 90.7 / 78.0 / 63.9 / 41.4**, every level inside its band, each pooled from three independent sets
(d0, d1 and d3 from 39304-39447, d2 from 39448-39591).

## Checks

- **Speed:** the strategist costs 0.82x r67's per decision (6 games, d2, [per_decision.md](per_decision.md)); a busy
  box throughout, so `speed.py check` was not read (no verdict across a revision anyway).
- **Loop smoke** (`loop_smoke.py --run`, 16 seeds): no timeouts, at most 3 plays of one card a game.
- **The frozen run replayed:** no code changed after the final read froze (one binary throughout); the post-read
  dial is d2's entry alone. Set a's d0, d1 and d3 replayed on the final config: 432 of 432 games the same ([replay157.py](replay157.py); `same_results.py` refuses the moved config hash).
- `npm run cards:fit` and `play_read.py` read player cards only (web/scripts/cards-fit.mjs has no Unity faces);
  the changed Unity cards' use is their resolutions a game (above).

