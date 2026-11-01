# The planner samples Unity deals (054, no rules change, r23)

Fresh seeds from 6000 are the verdict (never used to pick anything). Tuning used 1000-1031 and 2000-2031.
Specs: `experiments/band.json`, `experiments/gate-exploit.json`, `experiments/archive/gate-close-054.json`.

## Verdict (fresh, 48 seeds a cell, d2)

| | c2 | c3 |
|---|---|---|
| planner (4 deals, `lead` 60, `prune` 12) | **69** | **71** |
| engine | 65 | 60 |
| paired gap | +4 ±9 | +10 ±9 |

Both sizes are in the 60-75 band: no re-centre. The planner is ahead, not beyond the noise.
r23's planner on seeds 5000 read 71 / 62 against the engine's 77 / 71: another seed set, so the two
engine rows differ by noise alone, and only the paired gap compares (it was -6 / -9, now +4 / +10).

## Where the strength came from

Tuning, c2 / c3 win %, each cell 32 seeds; ms is a decision, same run.

| agent | seeds 1000+ | seeds 2000+ | pooled |
|---|---|---|---|
| r23 planner (`deals=1,lead=0,prune=6`) | 62 / 50 | | |
| `deals=4` raw, `prune` 6, about 2.7 × time | 62 / 78 | | |
| `deals=4,lead=30`, `prune` 6 | 75 / 62 | | |
| one guess, `prune=12` | 72 / 81 | 84 / 91 | **78 / 86** |
| `deals=4,lead=60,prune=12` (shipped) | 72 / 69 | 88 / 75 | **80 / 72** |
| `deals=4,lead=120,prune=12` | | 81 / 78 | |
| engine | 69 / 78 | 75 / 78 | 72 / 78 |

**The find was `prune`, not the deals.** `planner:alts=0`, the line alone, should be about the engine.
At `prune=6` it won 29% at c3 (14 seeds), 81 at 12, 75 at 0 (16 seeds): a dropped action stays out for
the whole phase, so the rollout policy was never the engine, and the shortlist was carrying it.

**Retracted (060): "sampling cost about 14 at c3".** No `prune` 12 row here separated `deals` from `lead`:
every sampled row had a lead and no one-guess row did. 060's scratch 2×2 did (c3, d2, seeds 1000-1031, r25,
`prune` 12, win %):

| | `lead` 0 | `lead` 60 |
|---|---|---|
| one guess | 56 | 41 |
| 4 deals | 72 | 59 |

4 deals over 1: +16 ±11 at `lead` 0, +19 ±8 at `lead` 60. `lead` 60 over 0: -16 ±11 at one guess, -12 ±12 at
4 deals. So `lead` lost the points and sampling gained them. That is 32 tuning seeds at r25, so it doesn't
compare directly with the r23 figure it retracts. 064 read `deals=4,lead=0` on fresh seeds and made it the
default. What this section first said: sampling added nothing at c2 and cost about 14 at c3 against one
guess (tuning, about ±6). `deals=4,lead=0,prune=12` was not run here: over the time limit. The sampled agent
shipped because the prompt asked for it and the question to Matt went to its default.

## Cost

Same binary, same load, seeds 1000+, first 9-10 games a cell (the run hit its time limit): against r23's
settings a decision costs **1.28 × at c2, 1.29 × at c3**, inside 1.5 ×. `prune=12` alone: 1.10 / 1.02.
Raw 4 deals was about 2.7 ×: a two-round line usually forks early, so a tail is most of a line. Two
savings pay for it: a line plays its other deals only when challenged, and `lead` makes that rare.

## Where play first differs

`planner` r23 against now, seeds 1000+, 8 a cell: every game differs, at a `pay` in 14 of 16, `pay` or
`play` in 2: a rollout choice in the first action phase. Engine: the same in all 16. The old defaults
replay r23 exactly: 32 games, `same_results.py`, planner and engine.

## The exploit gate (engine, rules and engine play unchanged from r23)

`gate-exploit` from 6000: 0 panics, **not clean**: `tier2_rush` c2 and `always_consolidate` c3 stopped at 8
seeds, -50 ±21. At 48 paired seeds (`gate-close-054`; engine minus check, 65 / 60):

| check | c2 | c3 |
|---|---|---|
| tier-2-rush | -15 ±8 | -15 ±9 |
| always-Consolidate | +6 ±10 | -15 ±9 |
| no-powers | +4 ±8 | -17 ±9 |
| science-board | +27 ±9 | +4 ±9 |
| never Science | +8 ±9 | +10 ±9 |
| never Industry | +2 ±8 | +21 ±10 |

None is beyond 2 SE, and four sit ahead of the engine at 1.7-1.9 SE. These are 053's standing findings
read harder at d2 under the engine; `gate.json` under the planner, which decides, was not run. Tier 2 being
simply better is still the first thing to fix, by price, not by agent.
