# 240: balance pass, r109 (Fortify's price; Researcher levers screened)

Base: main 74bd2f0 (239's docs trim, r108). `RULES_REVISION` bumped from the constant, 108 to 109. Reader:
[read.py](read.py) (screen arms and paired pairs; `bench/ladder.py --pairs` gives the same pair table). Tables:
[screen.txt](screen.txt), [fresh.txt](fresh.txt) (d1-d2), [fresh-d0d3.txt](fresh-d0d3.txt), [ladder.txt](ladder.txt)
(levels, measures, pairs, seat order), [design_check.txt](design_check.txt), [resource_use.txt](resource_use.txt),
[loop_smoke.txt](loop_smoke.txt), [speed.txt](speed.txt). Specs: `experiments/farm/s240.json` (screen),
`f240.json` (fresh read). Kept reads: `out/keep/` (f240-new, f240-base-r108, s240).

## What shipped

| Lever | Change | Why |
|---|---|---|
| Fortify | 1 chip (was 2) | a price fix to the unit anchors, not a cut for its band |
| `design_check` bands | Debrief and Patrol 0.25-3.5 (were 2.75 and 3.0) | Matt: overuse alone is no reason to nerf; read 3.04 and 3.37 at r108 |

**Not shipped (Matt, message 10729):** Grant adding 2 cubes and Peer Review's insight always. The levers were there to
cut B+P's lead, and at r108 that lead is gone (below); they would widen the spread on the pairs already on top, and
Grant at 2 cubes for 3 credits breaks the pricing ruling. Grant at 6 credits for 2 cubes (2.0x, today's rate) stays a
candidate.

## Give / get (anchors; `values.json` in brackets)

- **Fortify at 1 chip:** give 1 chip, 6 [4]. Get 1 troop removed, 5 [11], plus a facility hardened, 1.5, found on or
  next to the target in 45% of uses (268 of 596 on f240): about 0.7. Total 5.7 [11.7]: **0.95 at the anchors** (2
  chips read 0.47), a gain at the table. No `values.json` row moves.
- **Grant at 2 cubes (screened, not shipped):** give 3 credits, get 2 cubes 12 [12]: 4.0 (1 cube: 2.0).
- **Peer Review's insight always (screened, not shipped):** give 1 cube 6 [3], get a node (Place a node's 6 credits)
  plus 1 insight 4 [2.5]: 1.67 (with the fresh-station rider, about 72% of placements: 1.48).

## Base pairs at r108 (the premise had moved)

228's B+P lead (+6.5 to +6.8 at d1-d2, r100) is gone. At r108, d1-d2, both agents pooled:

| read | seeds | bp | br | pr | B+P gap to the pair mean |
|---|---|---|---|---|---|
| f238 (238's fresh read, r108) | 47376-47423 | 69.8 | 76.6 | 72.4 | −3.1 |
| s240 base cells | 47328-47391 | 73.4 | 76.2 | 78.5 | −2.6 |
| f240 r108 side | 47424-47487 | 74.2 | 75.0 | 70.3 | +1.0 |

Each pair's win SE is about 3 pooled (4.5 per agent), so the three seed sets disagree within noise. Across all three
reads, no pair sits more than 4 points from the pair mean, and B+P is near it or under it.

## Screen (s240: spent seeds 47328-47391, d1-d2, 64 seeds × 2 levels a cell, paired)

F = Fortify 1 chip; G = Grant 2 cubes; P = Peer Review's insight always. F touches only Builder pairs and G, P only
Researcher pairs, so the cells that would replay base were not run. Paired gaps against base:

| arm | greedy bp / br / pr | strategist bp / br / pr | pooled bp gap to the mean |
|---|---|---|---|
| base | 71.1 / 74.2 / 74.2 | 75.8 / 78.1 / 82.8 | −2.6 |
| F (shipped) | +2.3 ±2.3 / +1.6 ±1.1 / = | +1.6 ±2.2 / −1.6 ±1.9 / = | −1.3 |
| FG | = / −0.8 ±3.8 / +7.8 ±4.7 | = / +2.3 ±3.9 / −7.8 ±4.1 | −1.6 |
| FP | = / +4.7 ±2.2 / −3.1 ±2.9 | = / −3.9 ±3.2 / −3.9 ±2.8 | −0.3 |
| FGP (asked) | = / −1.6 ±4.0 / +4.7 ±4.6 | = / +0.0 ±4.2 / −3.9 ±4.1 | −1.2 |

**Each lever on the pair spread.** G moves the Researcher pairs in opposite directions for the two agents (pr +7.8
greedy, −7.8 strategist), and pooled it is +0.8 on br and +0.0 on pr. P pooled is +0.4 br and −3.5 pr. Neither is
beyond about 1 SE pooled. Use barely moves: Grant 1.70 → 1.75-1.84 a seat-game with G, and Peer Review 3.2-3.4 either
way. Neither lever lifts the Researcher, and B+P was not leading anyway. Fortify goes from 0.18-0.20 to 0.45-0.49 a
seat-game with F.

## Fresh read (f240: 47424-47487, taken; 64 seeds × bp / br / pr × d0-d3, 1,536 games an agent a side)

The new side was farmed; the r108 side ran natively on the same seeds, on a saved r108 binary and tree.

| agent | d0 | d1 | d2 | d3 | new − r108 |
|---|---|---|---|---|---|
| greedy r109 (gap) | 84.9 (+0.0) | 76.0 (−0.5) | 71.9 (+1.6) | 48.4 (+1.0) | +0.5 ±0.8 |
| strategist r109 (gap) | 86.5 (−1.0) | 76.6 (+0.5) | 71.9 (+2.1) | 47.4 (+3.1) | +1.2 ±0.9 |

Level bands (soft): greedy d0 is 0.1 under 85-100, and d2 is above 50-65 for both agents, the same misses as r108 on
these seeds. The rest are met.

**Pairs, d1-d2:**

| agent | side | bp (gap) | br (gap) | pr (gap) |
|---|---|---|---|---|
| greedy | r108 | 75.0 (+1.6) | 74.2 (+0.8) | 71.1 (−2.3) |
| greedy | r109 | 71.9 (−2.1) | 78.9 (+4.9) | 71.1 (−2.9) |
| strategist | r108 | 73.4 (+0.5) | 75.8 (+2.9) | 69.5 (−3.4) |
| strategist | r109 | 77.3 (+3.1) | 75.8 (+1.6) | 69.5 (−4.7) |

pr replays exactly, since it has no Builder. Pooled, B+P's lead is +1.0 at r108 and +0.5 at r109; the largest gap is
greedy br +4.9. Every pair is within 5.

**Uses, a seat-game (pooled):** Fortify 0.22 → **0.58** (band 0.25-2.50: met). Debrief 3.21 both sides (band
0.25-3.5). Patrol 3.50 → 3.52: **0.02 over its new 3.5 ceiling** (strategist 3.64, greedy 3.39). It was 3.50 on the
r108 side of these seeds, so this is seed noise, not Fortify. The band is soft and is reported, not chased. Grant 1.78,
Peer Review 3.5, Mint 2.7: unchanged.

**Resource use (spent ÷ gained, all levels, r109; r108 in brackets):** chips 54% (54), energy 77% (77), research
cubes 62% (62), antimatter 39% (39). Chips go to Fortify 0.19 a seat-game (was 0.14).

**design_check** (fresh, against r108): the same warnings as r108's own read, none from this item. Deep State Ledger
and Dyson Swarm Tranche are kept-not-played. Usage is outside band for Audit 2.02 (band to 2.0), Patrol 3.52, Sell a
card 0.76, Tidal Grid converter 0.00 and antimatter 39% spent. Rounds are d0 10.4 and d1 10.2. **loop_smoke** (final
build): no timeouts, and the most plays of one card is 6.

**Speed** ([speed.txt](speed.txt)): `speed.py pair` r108 → r109 gives one-thread ratios of 1.05× and 0.99× for greedy
(c2, c3) and 0.79× for engine c2. The games differ, so each ratio is speed times game length. Engine c3's old side
failed: the r108 binary read the current config, and `pair` needs `--old-config`. A price change adds no cost per
decision. `check` gives no verdict across the bump, so the baseline is re-based at r109 (`--update`) on a quiet box.

## Candidates (report-only)

- **Grant at 6 credits for 2 cubes** (2.0x, today's rate). Matt holds it as a candidate.
- **Patrol's ceiling.** It sits on 3.5 at r108 and r109. If the band should hold the r108 reads with room to spare,
  3.75 would.
- **Audit 2.02** over its sacrifice band of 0.25-2.0, on both sides; not this item's lever.

## For new players (r109)

Fortify, the Builder's troop answer, costs 1 chip: remove a troop on the target and harden a facility on or next to
it. Nothing else changed in play. No rule resisted: Fortify's use is now in its band, and no board pair leads its
pair mean by more than 5.
