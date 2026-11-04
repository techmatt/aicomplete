# Four deals, lead 0, and d2 re-centered (064, r25 to r26)

Every read is 48 seeds a cell, d2, ± one SE of the paired gap. Each seed set did one job: 14000 picked the
planner, 15000 picked levers, 16000 and 18000 gave band verdicts, 17000 the gate.

## Verdict: `deals=4,lead=0` is the default

The rule: adopt if it leads one guess beyond 1 SE on 96 paired seeds at no more than ~1.6 × an action.

| r25, seeds 14000 | c2 | c3 | c2 ms/action | c3 ms/action | c2 s/game | c3 s/game |
|---|---|---|---|---|---|---|
| one guess | 69 | 60 | 28.7 | 60.5 | 13.6 | 44.6 |
| `deals=4,lead=0` | **81** | **73** | 45.1 | 93.4 | 21.2 | 69.5 |
| engine | 54 | 56 | 2.8 | 5.0 | 1.4 | 3.9 |
| four deals minus one guess | +12.5 ±7.7 | +12.5 ±7.7 | 1.57 × | 1.54 × | 1.56 × | 1.56 × |

**Pooled: +12.5 ±5.4 on 96 paired seeds, at 1.54-1.57 × an action.** Adopted. Rounds and decisions barely
move (8.73 against 8.81, 9.19 against 9.23), so the cost is the code, not longer games. Timed one thread on
fewer games, `per_decision.py` read 1.63 × (c2, 16 games) and 1.39 × (c3, 8 games); take the 48-seed band.
060's 1.51 × stands. Games a minute on 8 threads, from `ms_per_game`: 22.7 at c2, 6.9 at c3 (one guess 35.2,
10.8). `speed.py specs` was not run: the band's own 48-seed timing replaced it.

The default plays the spec: the r26 band `band-064-hold4` used `planner`, and `agents/mod.rs` changes one
number, `deals`, from 1 to 4 (`lead` was already 0).

## At c2 a planner now beats the engine

| r25, c2, seeds 14000 | win % | gap |
|---|---|---|
| engine | 54 | |
| line alone (`alts=0`) | 50 | -4 ±10 against the engine |
| one guess | 69 | +19 ±9 against the line, +15 ±10 against the engine |
| four deals | 81 | +31 ±10 against the line, **+27 ±7.7 against the engine** |

061's "no planner beats the engine at c2" (+0 ±9 at r25, seeds 8000) doesn't hold for four deals. The line
alone is the engine, within noise, so the rollout policy is sound and the alternatives are what add. The two
reads of the engine differ (67 on seeds 8000, 54 here, 58 on 15000): part of 061's zero was a high engine
on those seeds. One guess against the engine, pooled over sizes, is +9.4 ±6.8: not separated. Four deals is
+21.9 ±5.6.

## Re-centering d2

Four deals read 81 / 73 on seeds 14000. Base at r25 on other seeds: c2 71 (15000), 75 (16000); c3 77 (16000).
Mean about 76 / 75, so d2 needed a small push.

**Engine triage (seeds 15000, base 58 / 56; direction only).** Steps come in two sizes: under 6 points, or over 20.

| Lever | c2 | c3 | |
|---|---|---|---|
| `project_discord_per` 1 | 54 | 52 | -4 ±2.1 pooled |
| `corrupt_at` 5 | 56 | 52 | |
| `hold_base` 4 | 54 | 46 | purge wins 12 → 6, 7 → 1 |
| `base_sites_per_country` 0 | | 31 | c3 only, by construction: the wrong size |
| `troop_presence` 3 | 38 | 17 | presence clock; c3 too hard |
| `extra_threats` 2 | 21 | 33 | |
| `adversary_discord` 1 | 17 | 27 | |
| `stage1_removed` 2 | 19 | 27 | |
| `free_city` 4, `discord_tier2` 1, `threat_discord` 2, `free_leaves` 2, `presence_stage` 3, `last_safe_stage` 3, 3 + 4 Unity cards, `base_unity` 1, `project_credits` 5 | 54-58 | 52-69 | nothing beyond the noise |

**Planner at c2 (seeds 15000, base 71).** `project_discord_per` 1: 71, not one game changed. `hold_base` 4: 69 alone,
69 with it. Those two with `corrupt_at` 5: 62 (-8.3 ±7.8). `extra_threats` 2: 25, out of band and far below.

`project_discord_per` 1 is a real but small push under the engine and none at c2 under the planner: megaprojects
hold few adversary cubes in a planner game. It goes in by the standing ruling, and it costs nothing.

**Verdict (seeds 16000, nothing picked there), against base on the same seeds:**

| | c2 | c3 |
|---|---|---|
| r25 base | 75 | 77 |
| **r26: `project_discord_per` 1, `corrupt_at` 5** | **75** | **71** |
| gap | +0 ±5.2 | -6.2 ±6.9 |

Inside 60-75 at both sizes, c2 on the top edge. Pooled -3.1 ±4.3: this is not a measured push. The gate's rerun
gives a second fresh read of r26: 67 / 52 on seeds 17000, so 71 / 62 over 96 seeds (*The gate*, below).

**Adding `hold_base` 4 (seeds 18000, not paired with r26):** 62 / 48, c3 out of band. Rejected by the rule set
before the run (both sizes in band and nearer 67). Its 48 is one seed set, and "c3 is ±15 on any one set" applies;
a further lever between these would need its own fresh read, and the gate had to start.

**New player summary:** in the rules, *Megaprojects* (now `docs/rules/board.md`).

## The gate

`experiments/gate.json`, 8 seeds, `--seed 17000`, r26: 432 of 512 games, 30 min on 8 threads, 0 panics.
**Not passed by the tool's reading.** The planner read 75 / 62 on those 8. The prompt says 8 seeds read the
base about 20 low; 061 read it 21 high (88 / 88 against 67 / 75). It swings both ways: the base read 67 at 48
seeds here.

Level or ahead at 8 seeds went to 48, paired, on the gate's own seeds. The planner row of those runs is the gate's
base at 48 seeds, so there was no separate base run. `experiments/gate-base-064.json` was not needed and is deleted.

| check minus planner | c2 | c3 |
|---|---|---|
| planner, 48 seeds | 67 | 52 |
| always-Consolidate | (12 behind at 8 seeds) | **+17.5 ±9.4**, stopped at 40 seeds |
| tier-2 rush | (25 behind at 8 seeds) | +4.2 ±9.9 |
| no-powers | +6.2 ±8.1 | (12 behind at 8 seeds) |
| never_tag=3 | (33 behind at 6 seeds) | -2.1 ±8.1 |
| science_board | (38 behind at 8 seeds) | -15.6 ±10.1, stopped at 32 seeds |
| never_tag=6 | (67 behind at 6 seeds) | -21.9 ±11.7, stopped at 32 seeds |
| never_remove | -14.6 ±7.3 | (12 behind at 8 seeds) |

**The gate fails on one check: always-Consolidate at c3, 72 against 52, ahead by 1.9 SE.** 061 had it at +8
±7.8 at r25 with one guess, on other seeds. 25 of its 29 wins are by purge (the planner's: 6 of 25); it loses
half its presence doing so. This is 055's finding, the Consolidate bundle pays too well for a purge plan,
which Matt parked for the rules review. `corrupt_at` 5 may feed it: c2's purge wins rose 13 to 19 on seeds
16000. That is not separated: no row screens `corrupt_at` alone under this check.

The other ahead-or-level checks sit within 1 SE, the same picture as 061: fixed rules that keep tempo give up
little to a two-round search. Tier-2 rush is now +4 at c3 and well behind at c2, better than 061's -10 at c2.

**A second fresh read of r26 at c3: 52 ±7.** With 16000's 71, two fresh sets of 48 differ by 19, which is the README's
±15. Pooled over 96 fresh seeds r26 reads **71 / 62**: both in band, c3 under the middle, c2 over it. So
adding push at c3 has no room, and the rejected `hold_base` 4's 48 at c3 may be partly these seeds' kind of low.
The done-when's 48-seed read (75 / 71) passes, but the 96-seed read is the better number. The r25 base on
those 17000 seeds was not run.

## Commands

    aic-sim experiment experiments/archive/band-064.json --seed 14000 --threads 8
    aic-sim experiment experiments/archive/band-064-alts0.json --seed 14000 --threads 8
    aic-sim experiment experiments/archive/triage-064.json --seed 15000   # and -064b, c, d
    aic-sim experiment experiments/archive/confirm-064-c2.json --seed 15000 --threads 8
    aic-sim experiment experiments/band-064-final.json --seed 16000 --threads 8
    aic-sim experiment experiments/gate.json --seed 17000 --threads 8
    aic-sim experiment experiments/gate-close-064-c2.json --seed 17000 --threads 8
    aic-sim experiment experiments/gate-close-064-c3.json --seed 17000 --threads 8
    aic-sim experiment experiments/band-064-hold4.json --seed 18000 --threads 8
    python scripts/bench/paired.py DIR --pair A B

`band-064-final.json` holds both revisions by overlay, so it replays at r26. `triage-064*` and `confirm-064-c2`
overlay r25's configs: their base is r25, and at r26 they would read otherwise.
