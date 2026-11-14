# 230: antimatter follow-ups (r101)

Matt's rulings of 2026-11-13 and amendment 10186. `RULES_REVISION` bumped from the constant, 100 to 101. Base: main
65918be (229's r100, no play change from 228). Specs: [spec.py](spec.py) (`experiments/farm/s230a`, `s230b-c31`,
`s230c`, `s230d`, `f230`). Readers: [am.py](am.py) (antimatter supply by source, spent / made, cheap share, burns by
round, per lever and agent), [fread.py](fread.py) (the fresh read, paired). Tables: [screens.txt](screens.txt),
[fresh.txt](fresh.txt), [fresh-am.txt](fresh-am.txt), [design_check.txt](design_check.txt),
[per_decision.txt](per_decision.txt). Kept reads: `out/keep/230_antimatter_followups/`.

## What shipped

| # | Ruling | Built |
|---|---|---|
| 1 | Dark Fiber Collider Cyber + Science | content; `needs` stays Cyber 1; price kept (6). Reclaim stays Security: a `tag_story` exception in `scripts/design_check.json` records why (Matt's ruling; parked P3) |
| 2 | Horizon's converter dearer first | 3 research cubes + 1 energy for 1 antimatter (was 2 cubes). The capstones' free antimatter was screened off (below) and kept |
| 3 | `hand_worth` counts antimatter the seat can make | `w.am_make` 1: `keep_plan::am_makes` (plants' Actions and non-burn powers, a round's worth) beside antimatter income, in `SeatData`. `hand_worth` runs only for table-mode agents (`w.table` > 0: `engine`, the planner), so **greedy and the strategist play the same games with it on or off** (screen 2: identical to the bit) |
| 4 | No early burn in `make_short`'s source path | behavior `burn_late` 1: no Teardown, Meltdown or Reclaim as `make_short`'s source while the adversary deck holds more than two cards (`Eval::cards_left`). `burn_late` 2 also keeps them out of the search: screened, not shipped |
| amendment 10186 | the cheap source under ~15% | behavior `am_last` 1: the search never takes Make antimatter; only `make_short` does (a card the keep plan plays this round needs it). An agent heuristic, not a plant change: the plants were already the better route at `values.json` (below) |

## Give / get (values.json: antimatter 18, cube 3, energy 4.5; anchors: cube 6)

- **Horizon's converter:** give 3 cubes 9 + 1 energy 4.5 = 13.5, get 18: **+4.5** a use (was +7.5). At the anchors
  give 18 + 4.5 = 22.5 for 18 (0.80; was 1.09): the anchors price it under the plants, as ruling 2 asks.
- **The plants** (unchanged): +9 to +15 a use at `values.json`; **Make antimatter** 5 cubes 15 for 18, +3.
- **Dark Fiber Collider:** the Science tag adds a tag for `needs` and per-tag payoffs at no price change: the anchor
  ratio was 2.05 at 3.5 uses (219), under the 2.5 ceiling with a tag's worth on top.
- The flat agents' weights (antimatter 2.0, cube 0.3, chip 4.0, credits 0.6, energy 0.8) price Make antimatter at
  +0.5 and the converter at 2 cubes + 2 energy at a loss (2.2 for 2.0): why that cell's converter was never used.

## Screens (spent seeds 46976-47007, 229's; d1-d2, 192 games a cell an agent; base = main, checked identical)

| cell | agent | win | plants | converter | upkeep | burns | Make | cheap | spent / made | burns r1-4 |
|---|---|---|---|---|---|---|---|---|---|---|
| base | greedy | 62.5 | 12% | 34% | 26% | 7% | 12% | 17% | 37% | 31% |
| base | strategist | 59.9 | 13% | 31% | 27% | 8% | 13% | 17% | 37% | 35% |
| retag only | strategist | 58.9 | 13% | 31% | 27% | 8% | 13% | 18% | 38% | 35% |
| converter 3c+1e | greedy | 63.0 | 15% | 19% | 28% | 9% | 19% | 27% | 41% | 29% |
| converter 3c+1e | strategist | 58.9 | 14% | 18% | 31% | 9% | 18% | 23% | 39% | 34% |
| converter 2c+2e | greedy | 59.9 | 18% | 0% | 34% | 11% | 26% | 34% | 43% | 27% |
| converter 2c+2e | strategist | 60.9 | 18% | 0% | 36% | 11% | 24% | 30% | 44% | 32% |
| 3c+1e, capstones' antimatter off | strategist | 57.3 | 20% | 26% | 0% | 13% | 27% | 35% | 50% | 31% |
| 3c+1e + `burn_late` 1 | greedy | 64.6 | 15% | 20% | 29% | 5% | 20% | 28% | 39% | 11% |
| 3c+1e + `burn_late` 1 | strategist | 56.8 | 16% | 19% | 31% | 4% | 20% | 25% | 39% | 14% |
| 3c+1e + `burn_late` 2 | strategist | 54.2 | 15% | 19% | 32% | 2% | 21% | 27% | 38% | 0% |
| **3c+1e + `burn_late` 1 + `am_last`** | greedy | 65.1 | 16% | 31% | 30% | 6% | 5% | 14% | 41% | 11% |
| **3c+1e + `burn_late` 1 + `am_last`** | strategist | 58.9 | 17% | 30% | 33% | 5% | 4% | 10% | 39% | 13% |
| 3c+1e + `burn_late` 2 + `am_last` | strategist | 56.8 | 18% | 30% | 34% | 2% | 5% | 11% | 39% | 0% |

SE on a win cell about 3.5. Every win move above is inside 1 SE of its neighbor. Full output: [screens.txt](screens.txt).

- **Ruling 2's order held and the rate alone did not move supply to the plants.** A dearer converter hands its share
  to Make antimatter (the agents' cube is worth 0.3, so 5 cubes for an antimatter still reads as a gain to them);
  the plants' antimatter stays 0.53-0.61 a seat-game in every cell. At 2 cubes + 2 energy the converter is never used.
- **The capstones' free antimatter** was all of upkeep's antimatter (upkeep 0% with it off). Off, the plants reach
  20-22%, but Make antimatter becomes the largest source (26-30%) and the cheap share 35-39%: kept on.
- **`am_last`** takes Make antimatter from 0.70-0.77 to 0.16-0.21 a seat-game; the converter takes most of it back.
- **`burn_late` 1** ends Teardown before the last cards (47% → 0% in rounds 1-4); Reclaim still fires early 40-55%,
  through the search. `burn_late` 2 ends that too but cuts Meltdown, already late, 0.37 → 0.09 a seat-game, and read
  1-2 points lower: the ruling's sentence (the source path) shipped.
- **`w.am_make`, engine smoke** (`s230d`, 12 seeds, d2, 36 games a side): 8 of 36 games differ; win 33.3 / 27.8,
  plants 23% / 22%: no read at this size.

## Fresh read (47008-47039, taken: 32 seeds × 3 pairs × d0-d3, 384 games an agent a side; both sides native)

Base: main's exe (65918be, r100), built before any edit. New: e59ef28 (r101). [fresh.txt](fresh.txt) has every slice.

| agent | d0 | d1 | d2 | d3 | all, new − base | rounds base → new |
|---|---|---|---|---|---|---|
| greedy r100 | 91.7 | 61.5 | 56.2 | 45.8 | | 8.65 |
| greedy r101 | 87.5 | 69.8 | 52.1 | 40.6 | −1.3 ± 2.1 | 8.71 |
| strategist r100 | 83.3 | 65.6 | 52.1 | 38.5 | | 9.07 |
| strategist r101 | 82.3 | 71.9 | 55.2 | 52.1 | **+5.5 ± 2.3** | 8.95 |

By pair, all levels (new − base): greedy bp −5.5 ± 4.2, br +3.9 ± 3.0, pr −2.3 ± 3.8; strategist bp +2.3 ± 3.9,
br +9.4 ± 4.4, pr +4.7 ± 3.8. The strategist's lift is in the Researcher's pairs and at d3 (+13.5 ± 4.6). The
screens read the same build +2 (strategist) and +2.6 (greedy) over base on 46976-47007, so the fresh +5.5 is
likely partly seed luck.

**Antimatter, a seat-game ([fresh-am.txt](fresh-am.txt)); greedy / strategist:**

| | made | spent (÷ made) | plants | Horizon's converter | upkeep | burns | Make antimatter | other | cheap |
|---|---|---|---|---|---|---|---|---|---|
| r100 | 4.32 / 4.66 | 38% / 36% | 18% / 17% | 33% / 34% | 24% / 25% | 6% / 6% | 11% / 12% | 8% / 7% | 17% / 20% |
| r101 | 3.64 / 3.78 | 41% / 41% | 23% / 20% | 32% / 33% | 27% / 29% | 4% / 4% | 4% / 4% | 10% / 9% | 14% / 15% |

Plants' antimatter a seat-game 0.78 → 0.85 (greedy), 0.77 → 0.77 (strategist): their share rose because the total
fell, not because they made more. **Burns in rounds 1-4:** 37% → 13% (greedy), 32% → 9% (strategist); Teardown
57% → 0% and 56% → 7%; Reclaim 50% → 56% and 62% → 33% (it fires through the search, outside the ruled path);
burns a game 0.51 → 0.27 and 0.56 → 0.29.

**Resource use (spent ÷ gained, both agents):** credits 105% (105), insight 81% (80), energy 76% (77), chips 57%
(58), research cubes 62% (65), ambassadors 73% (73), antimatter 41% (37). r100 in brackets.

## The plants-largest target: unmet

A reachability screen after the fresh read (`s230e`, 46976-47007, d1-d2, the shipped agents), a seat-game:

| cell | plants | converter | upkeep | Make | made | cheap | win greedy / strategist |
|---|---|---|---|---|---|---|---|
| shipped (3c+1e) | 0.54-0.63 (16-17%) | 30-31% | 30-33% | 4-5% | 3.28-3.60 | 10-14% | 65.1 / 58.9 |
| capstones' antimatter off | 0.55-0.67 (24-25%) | 43-44% | 0% | 8% | 2.34-2.65 | 18-20% | 63.0 / 57.8 |
| converter 2c+2e (unused) | 0.56-0.64 (21-23%) | 0% | 43% | 12% | 2.65-2.76 | 21% | 64.6 / 59.9 |
| both | 0.61-0.66 (36-39%) | 0% | 0% | 23-26% | 1.68-1.70 | 39-42% | 62.0 / 60.4 |

**The plants make 0.53-0.67 antimatter a seat-game in every cell of every screen**: no lever in scope moved their
output, only the other sources' size. They lead only when both the converter and the capstones' antimatter are
gone. Then supply halves, Make antimatter returns as the second source and the cheap share is 39-42%, against
amendment 10186. What limits them is how often they are built and used. Antiproton Foundry's chip and Treaty Isotope
Bank's 8 credits read as losses at the flat agents' weights (chip 4.0, credits 0.6, antimatter 2.0); 222 found those
two built in 19-30% of games, their Actions almost never used. Moving that is a plant price or `values.json`
question (the pricing rulings stand), so it is left for Matt.

## Checks

- `design_check` (fresh, `--old` base, [design_check.txt](design_check.txt)): the Dark Fiber Collider `tag-story`
  warning is gone; Reclaim's is listed as an exception; cheap-source 14% (max 20%). Left, all also at r100 on these
  seeds: kept-not-played Deep State Ledger and Dyson Swarm Tranche, Tidal Grid's chip converter unused, antimatter
  spent 41% (band 50-110%), rounds d0 10.2 and d3 7.6.
- **Speed** (`per_decision.py`, c2 d2, 16 games, one thread, best of 2, a quiet box, [per_decision.txt](per_decision.txt)):
  greedy **1.24×** a decision (1.15× options a decision, 1.05× decisions: the games differ, so most of it is
  state, not code; wins 11 → 6 on those 16 games), the strategist **0.98×**. No `--update`: no verdict across a revision.
- Ruling 3's lever (`w.am_make`) is inert for greedy and the strategist: flat agents never call `hand_worth`.
- **Dark Fiber Collider after the retag** (fresh, both agents): played 0.36 → 0.37 a game, its Action's antimatter
  0.45 → 0.44 a game. `play_read.py` (fresh): no card kept in over 1% of games and never played, none never kept.

## Seeds and runs

Screens on 229's spent seeds 46976-47007 (`s230a`: 3,072 games; `s230b-c31`: 3,072; `s230c`: 1,536; `s230d`: 72;
`s230e`: 1,536; all d1-d2 but `s230d`). Fresh 47008-47039, taken (ledger next free 47040). The broker went away on
`s230b`'s first launch with no shard in; every later run was native, `scripts/bench/shards.sh` at 6 shards: 3,072
d1-d2 games in about 8 min, 768 d0-d3 games a side in about 3-4 min. Kept: `out/keep/230_antimatter_followups/`
(the fresh sides and every screen).

## For new players (r101)

Horizon's converter now takes 3 research cubes and 1 energy for an antimatter (was 2 cubes): the plants, the
buildings that turn one resource into antimatter once a round, are the cheaper way to make it. Dark Fiber Collider
shows Science beside Cyber. The rule that resisted: the plants still make only about a fifth of the antimatter;
Horizon's converter and the Protector's and Researcher's capstones make more.
