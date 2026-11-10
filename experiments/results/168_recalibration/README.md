# 168: recalibrate the difficulty dials to the strategist judge (r72)

Built on r71 (167, merged at e1145c7, which ended without its delta met; calibrated as landed). `RULES_REVISION` 71 → 72,
bumped from the constant. Seeds: dev 41208-41239 (screens, 32 a cell), band check 41240-41287 (set a) and 41288-41335
(set b), retune confirm 41336-41383 (set c) and 41384-41431 (set d); oracle rows on the first 8 seeds of sets a and c.
41432-41807 unused.

## The premise had moved

The prompt quoted r69 (every band 13-16 high). r71 (167's H read, 40912-40959) read d0 94.4, d1 82.6, d2 78.5,
d3 70.1: d1 7 high, d2 18, d3 25. The dev base (41208-41239) read 80.2 / 86.5 / 76.0 at d1-d3 and d4 47.9.

## The dials

| Dial | r71 | Freeze 1 | Final (freeze 2) |
|---|---|---|---|
| `compute_income` d0-d4 | 3, 4, 10, 8, 10 | 3, 4, 13, 24, 16 | 3, 4, **18**, 24, **13** |
| `ambassador_income` d0-d4 | 0, 1, 1, 1, 2 | 0, 3, 3, 3, 2 | 0, 3, 3, 3, 2 |

Nothing else moved. d4's ambassador income (2) now sits below d1-d3's (3) and its compute below d3's: d4's extra
threats, shorter Discord deck and tier-2 timing carry it (as d2's compute sat above d3's before).

**Reachable per level:** only the two income lists. `corrupt_cut_at`, `counter_bonus_at`, `counter_more_at`,
`stage_swap_at`, `board_difficulty_at` and `extra_threats_at` switch one number on from a level; `stage2_removed` and
`stage1_added` are fixed at d3+, `discord_removed`, `tier2_early`, `extra_corrupted` at d4. No code was needed: the
incomes reached every band.

## Screens (strategist, dev seeds, paired against base; tables in `screen1.md`, `screen2.md`, `screen3.md`)

- **d1 resists everything but a large ambassador step.** Ambassador income 1 → 2, compute 4 → 7 or 10,
  `counter_more_at` 1 and `board_difficulty_at` 1 all read 0 ± 5; ambassador 1 → 3 read −8.3 ± 5.5. d1's losses are
  the round cap with 1.5-2 corrupted cities left (`ends168.py`), which income barely touches.
- **d2:** ambassador +1 −12.5; compute +6 −12.5; ambassador 3 with compute 10 / 16 −17.7 / −25.0; ambassador 2 with
  compute 28 −31.2. `counter_bonus_credits` 4 → 8 (all of d1-d3) read +1.0 at d1 and +6.3 at d2 against the same
  incomes: dropped.
- **d3:** ambassador 3 with compute 24 −28.1 (47.9 dev), the same in three cells.
- **d4** (`screen_d4.md`, `screen_d4b.md`): ambassador 2 → 1 +9.4, compute 10 → 5 +5.2, `discord_removed` 0 +4.2,
  removed Discord cards not counting as tips +3.1 ± 3.1, `tier2_early` 0 −4.2. With d3's compute (24) at d4: 22.9 to
  25.0, 33.3 with `discord_removed` 0 as well. Base d4 (47.9) read level with the new d3, so monotone needed a
  harder d4.
- **The cheap oracle at d4** (`oracle:strategy=1,alts=8,budget=48,reseeds=16,answers=1`, 6 of 8 seeds, 18 games a
  cell; the last shard was stopped once the pick was made): base 100.0, tips off 94.4, compute 24 at d4 83.3 in every
  variant.

## The Discord-tip switch

`adversary.discord_removed_tips` (0/1) was built and screened at d4: the strategist +3.1 ± 3.1, the cheap oracle
94.4 against base 100.0 (one game). It doesn't move d4 toward 95, so it was dropped and its code reverted (its two
specs moved to `specs/`, since they name the removed param). d4's removed cards still count as tips.

## The band check (freeze 1, r72, sets a + b pooled, 288 games a level; `band.md`)

| Level | Band | Strategist | SE | Sets a / b | Greedy | Greedy SE | Strategist − greedy |
|---|---|---|---|---|---|---|---|
| d0 | ≥ 90 | 91.7 | 1.6 | 88.9 / 94.4 | 92.4 | 1.6 | −0.7 |
| d1 | 70-80 | 75.7 | 2.5 | 78.5 / 72.9 | 74.0 | 2.6 | +1.7 |
| d2 | 55-65 | **66.0** | 2.8 | 62.5 / 69.4 | 71.2 | 2.7 | −5.2 |
| d3 | 40-50 | 42.0 | 2.9 | 34.7 / 49.3 | 46.2 | 2.9 | −4.2 |
| d4 | below d3 | 28.5 | 2.7 | 26.4 / 30.6 | 34.0 | 2.8 | −5.5 |

d2 missed by 1.0 (0.36 SE), so it was retuned (the time gate allowed it), and d4's compute was eased toward the oracle.

## The oracle row (freeze 1; `oracle.md`; 41240-41247, 8 a pair)

| Level | Oracle | Strategist (same games) |
|---|---|---|
| d3 | 100.0 (24 of 24) | 37.5 |
| d4 | 83.3 ± 7.8 (20 of 24; losses: 2 round cap, 2 Discord deck) | 25.0 |

## The retune (freeze 2: d2 compute 13 → 18, d4 16 → 13; fresh sets c + d pooled, d2 and d4 only; `retune.md`)

Each level reads only its own income entries, so d0, d1 and d3 keep their freeze-1 reads; d2 and d4 were re-read on
fresh seeds. The d4 step is a guess between screen points (compute 10 and 24), not a screened value.

| Level | Band | Strategist | SE | Sets c / d | Greedy | Greedy SE | Strategist − greedy |
|---|---|---|---|---|---|---|---|
| d2 | 55-65 | 62.8 | 2.9 | 59.0 / 66.7 | 68.1 | 2.8 | −5.3 |
| d4 | below d3 (42.0) | 37.5 | 2.9 | 33.3 / 41.7 | 40.6 | 2.9 | −3.1 |

**The oracle row on the final build** (`oracle2.md`; 41336-41343, 8 a pair, d4): **95.8 ± 4.2** (23 of 24; the one
loss is a node loss), the strategist 37.5 on the same games. That is 0.8 above 95. The move from 83.3 rests on two
changes at once: compute 16 → 13 *and* a different seed set (41240- against 41336-). The cheap-oracle screen puts
compute 10 at 100 and 24 at 83 on the same seeds, so the direction holds; its size is not measured.

## Freeze line (final dials)

| Done when | Value | Met |
|---|---|---|
| d0 ≥ 90 | 91.7 ± 1.6 | met |
| d1 70-80 | 75.7 ± 2.5 | met |
| d2 55-65 | 62.8 ± 2.9 (freeze 1: 66.0 ± 2.8, unmet) | met |
| d3 40-50 | 42.0 ± 2.9 | met (0.7 SE above the floor) |
| Monotone d0 > d1 > d2 > d3 > d4 | 91.7 > 75.7 > 62.8 > 42.0 > 37.5 | met (d3 − d4 4.5, across seed sets, about 1.1 SE) |
| d4 oracle toward 95 | 95.8 ± 4.2 (24 games); the dial: `compute_income` d4 | met by value |
| Greedy beside the strategist | 92.4 / 74.0 / 68.1 / 46.2 / 40.6 | reported |

Greedy sits above the strategist at d2-d4 by 3-5 points (as at r69 d3, 162). The bands are the strategist's.

## Farm (Matt, message 7665: shards by worker and wall time)

| Unit | Seeds | Shards: main box (native) / laptop (wasm) | Wall |
|---|---|---|---|
| screen 1 | 32 | 16 / 0 | 1897 s |
| screen 2 | 32 | 16 / 0 | 1186 s |
| d4 screen | 32 | 16 / 0 | 990 s |
| screen 3 | 32 | 16 / 0 | 1465 s |
| d4 screen 2 | 32 | 12 / 4 | 420 s |
| cheap oracle d4 | 8 (6 kept) | main box 3 done, stopped at the pick | ~1940 s a 2-seed shard |
| band set a | 48 | 15 / 9 | 1373 s |
| band set b | 48 | 20 / 4 | 1391 s |
| retune set c | 48 | 21 / 3 | 683 s |
| retune set d | 48 | 21 / 3 | 678 s |

Oracle rows ran locally on 3 threads: freeze 1 (d3 + d4, 96 games) about 43 min, freeze 2 (d4, 48 games) about 23 min.

The laptop joined from 7:37am. Units 1-4 ran on the main box alone, beside each other at times (12 aic-sim processes:
the load messages 7645, 7659, 7677).
