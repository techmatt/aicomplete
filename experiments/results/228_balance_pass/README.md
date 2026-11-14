# 228: balance pass, r100

Base: main 3acba79 (227's r99: Fallout, Recovery cut, Seize renamed Reclaim). `RULES_REVISION` bumped from the constant,
99 to 100. Readers: [fread.py](fread.py) (the fresh read: pairs, levels, seat order, uses), [read.py](read.py) (226's
screen reader). Tables: [fresh.txt](fresh.txt), [design_check.txt](design_check.txt), [usage.txt](usage.txt). Kept
reads: `out/keep/228_balance_pass/`.

## What shipped

| Lever | Change | Why |
|---|---|---|
| Remove agents | 2 ambassadors for 3 agents (`board.remove_ambassadors` 1 → 2) | Matt's ruling: 0.94 at the anchors, was 1.88 |
| Liberate a city | its anchor stops counting the Sway twice (`priced_in`), and it is exempt as the win route | 226's finding; 1.12 without the double count |
| Researcher slots | 4 (`ai.slots` 5, 4, 4; was 3) | the weak board in both its pairs |
| Researcher insight | card-play track 7, 8, 9 (was 6, 7, 8) | as above |
| Synthesis | 1 insight a card (was 2) | as above |
| Field Grant | 2 cubes on any open megaproject (was 3, only where you own a station) | kept 0.29 a game, played 0.04 |
| Collateralized Grid | also gains 2 energy when played | kept 0.22, played 0.05 |
| `design_check` bands | Mint and Grant 0.15-0.9; hedges up to 0.75; Lift Fallout up to 0.5; Envoy from 0; Debrief up to 2.75; Patrol up to 3.0; Remove agents is a hedge | each `why` is in `scripts/design_check.json` |

## Give / get (anchors; `values.json` in brackets)

- **Remove agents:** give 2 ambassadors (16; table 8), get 3 agents (15; table about 6.6): 0.94. It is a hedge, so a
  table loss is expected, and it is still used 0.4 a seat-game.
- **Synthesis:** give 1 insight (4; table 2.5), get a draw (5; table 4): 1.25, a gain at both. At 2 insight it was 0.63.
- **Field Grant (tier 1):** give 8 credits, get 2 cubes (12; table 12): 1.50, the tier-1 ceiling. Before the change it
  read 2.25 with a station and nothing without one. Matt (10157): clearer, not stronger.
- **Collateralized Grid (tier 2):** give 15, get 2 energy now (9) plus +2 energy income (28) plus cash-out (0 with no
  capital held): 2.47 under the 2.5 ceiling. Table: about +27.

## Screens (spent seeds 46784-46879, d1-d2, 96 seeds × 2 levels a pair-cell, paired)

| Lever | greedy, gap to base | strategist | Note |
|---|---|---|---|
| A: Remove agents at 2 | +3.0 ± 1.8 | −0.7 ± 2.0 | Remove agents 1.5 → 0.4 a seat-game; Liberate unchanged |
| AG: A + Grant also gains 1 insight | +1.6 ± 2.0 | −1.2 ± 2.2 | Grant 1.73 → 1.84 a game: not a lever |
| AV: A + the agents' chip value at its anchor (6, not 4) | bp −4.2 ± 2.8, br +1.6 ± 3.2 | bp +3.1 ± 3.1, br −3.1 ± 3.0 | Mint 2.45 → 2.55 a game: the table is not why Mint is skipped |
| AFC: A + Field Grant + Collateralized Grid | +0.3 ± 1.2 | −1.7 ± 1.2 | FG played 0.02-0.03 → 0.07-0.08 a game; CG 0.03-0.05 → 0.05-0.07 |
| AS: A + Researcher 4 slots (R pairs vs A) | −0.3 ± 1.9 | +0.3 ± 2.0 | |
| AI: A + Researcher insight 7/8/9 | −1.0 ± 2.9 | +0.3 ± 2.7 | |
| ASI: both | +3.9 ± 2.8 | +4.4 ± 2.8 | |
| ASIY: ASI + Synthesis at 1 insight (shipped) | +7.8 ± 2.8 | +3.6 ± 2.8 | the largest lift to the Researcher's pairs |

At d1-d2 the screen's pair gaps on A were: greedy bp +9.0, br −8.2, pr −0.9; strategist bp +8.2, br −6.9, pr −1.2.
ASIY's gaps: greedy bp +3.8, br −7.1, pr +3.3; strategist bp +5.7, br −8.3, pr +2.6. No screened lever closed every pair
to within 5. ASI and ASIY tie on the largest gap (7.7 averaged over the two agents), and ASIY lifts the weak board more.

## Fresh read (46912-46975, taken: 64 seeds × 6 seat orders × d0-d3, 1536 games an agent a side; base natively)

| agent, d1-d2 | bp | br | pr | max gap | Researcher's board | all, new − base |
|---|---|---|---|---|---|---|
| greedy r99 | 76.2 (+10.0) | 60.5 (−5.6) | 61.7 (−4.4) | +10.0 | 61.1 | |
| greedy r100 | 73.8 (+6.8) | 61.3 (−5.7) | 66.0 (−1.0) | +6.8 | 63.7 | +0.9 ± 1.8 |
| strategist r99 | 79.7 (+9.6) | 65.6 (−4.4) | 64.8 (−5.2) | +9.6 | 65.2 | |
| strategist r100 | 72.7 (+6.5) | 62.9 (−3.3) | 62.9 (−3.3) | +6.5 | 62.9 | −3.9 ± 1.8 |

Pooled over d0-d3: greedy +1.8 ± 1.2, strategist −2.0 ± 1.3; max gap greedy +11.0 → +7.0, strategist +9.2 → +5.9.
**The pair bar (within 5 at d1-d2) is not met:** the gaps narrowed by about 3 points for both agents. For greedy, that
came from the Researcher's pairs rising (pr +4.3). For the strategist, it came from bp falling (−7.0). The strategist's
Researcher pairs did not rise on fresh seeds (br −2.7, pr −1.9). B+P, the pair without the Researcher, stays highest
for both agents. By level, greedy 86.2 / 75.3 / 58.9 / 53.9 and the strategist 86.5 / 74.2 / 58.1 / 49.0 at d0-d3.

**Seat order (reported only):** the Protector in seat 0 beats the Builder there by 2-5 points in B+P. In P+R and B+R
the first-named board in seat 0 wins 0-12 points more. That is within noise at 64 seeds a cell (SE about 4-6).

**Uses:** Remove agents 1.41-1.55 → 0.39-0.41 a seat-game; Liberate 2.86-2.94 → 2.93-3.01. Synthesis 6.3 → 6.7 a game.
Patrol 2.19-2.36 → 2.42-2.74, the cheap agent answer now. Mint (3.0) and Grant (1.3) are unchanged. Debrief 2.40 on
both sides.

**Resource use (spent ÷ gained, r100; r99 in brackets):** credits 106% (106), energy 73% (73), chips 59% (62),
research cubes 65% (67), ambassadors 73% (75), antimatter 38% (37).

## Mint and Grant: the band was the error

Both pay 2.0× at the anchors. Moving the agents' chip value to its anchor moved Mint by +0.1 a game. Grant +1 insight
moved Grant by +0.1 a game and no pair. Both are credit converters taken when credits are spare after cards. They are
used 0.35 and 0.15 a seat-round, early (Grant 0.6 in round 1, 0 after round 4), so their bands are now 0.15-0.9. Grant
sits on the band's floor.

## Left flagged

- **Deep State Ledger and Dyson Swarm Tranche kept-not-played** (0.05 and 0.04 played a game): the same on the r99 side
  of these seeds, on the edge of the threshold, and absent from 227's r99 list (Field Grant, Collateralized Grid, both
  fixed). Dyson was on r98's list.
- **Tidal Grid chip converter 0.00** and **antimatter spent 38%**: unchanged from r99; not a board lever.
- **`dials-hide-design`:** `board.remove_ambassadors` 1 → 2 is Matt's asked price, not a dial hiding a design change.
- **Rounds** d0 10.0, d2 7.9, d3 7.5: difficulty is not this pass's target.

## For new players (r100)

Remove agents now costs 2 of your ambassadors for up to 3 agents. Liberating cities is how you win, and ambassadors
are worth saving for it. The Researcher has 4 building slots like the Protector. Its card-play track shows 7, 8 and 9
insight, and Synthesis turns 1 insight into a card. Field Grant puts 2 of your cubes on any open megaproject, no
station needed. Collateralized Grid gives 2 energy when played, besides its income. The rule that resisted was board
balance: the B+P pair still wins 6-7 points over the pair mean.
