# 151 screens (dev seeds, the strategist, the three pairs)

Specs: `screen151-s1.json` .. `s6.json` beside this file (written by [specs151.py](specs151.py)); read with
`ladder151.py --screen DIR`. Win % (games); gap against the screen's base on the same seeds ± its paired SE.
"stated" is the prompt's numbers (compute 5 a round, ambassadors 1 a board at every level; Garrisons adds its +1).

| Screen | Base | Lever | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|---|
| 1 (36800-36831, 96 a cell) | compute 5, ambassadors 1,1,1,2 | base | 95.8 | 86.5 | 83.3 | 52.1 |
| | | stated | 95.8 | 86.5 | 83.3 | 61.5 (+9.4 ±4.7) |
| | | ambassadors 2,2,2,3 | 95.8 (+0.0) | 81.2 (−5.2 ±4.3) | 63.5 (−19.8 ±5.3) | 55.2 (+3.1 ±4.3) |
| | | compute 8 | 95.8 | 90.6 (+4.2) | 79.2 (−4.2) | 52.1 (+0.0) |
| 2 (36800-36863, 192) | ambassadors 2,2,2,3 | base | | | | 53.6 |
| | | stated | | | | 61.5 (+7.8 ±3.8) |
| | | ambassadors 4 at d3 / 5 at d3 | | | | 54.7 (+1.0) / 51.0 (−2.6) |
| | | compute 10 at d3 | | | | 47.4 (−6.2 ±3.9) |
| | | the smaller step (stage2_removed 1, stage1_added 1) | | | | 70.3 (+16.7 ±4.1) |
| 3 (after the review's fixes) | same | base | | 79.2 | 68.8 | 52.6 |
| | | stated | | 88.0 | 79.2 | 59.2 |
| | | compute 10 / 12 at d3 | | | | 46.4 (−6.2 ±3.7) / 39.1 (−13.5 ±4.1) |
| | | compute 7 at d2 | | | 63.5 (−5.2 ±3.6) | |
| 4 (compute 5,5,8,10) | | base | | 80.7 | 69.3 | |
| | | stated | | 88.5 | 81.8 | |
| | | compute 6 at d1 / 10 at d2 | | 82.3 (+1.6 ±2.6) | 62.5 (−6.8 ±3.5) | |
| 5 (compute 5,5,10,10, the search fix) | | base | | 80.7 | 57.3 | 44.3 |
| | | stated | | 87.0 | 83.9 | 56.2 |
| | | ambassadors 3 from d1 (compute 7 at d2) | | 79.7 (−1.0 ±2.9) | 62.0 (+4.7) | 44.3 |
| | | `counter_bonus_credits` 4 | | 80.7 (+0.0) | 53.6 (−3.6) | 44.8 |
| | | compute 11 at d3 | | | | 47.9 (+3.6 ±3.3) |
| 6 | same | `counter_bonus_all` 2 | | 80.2 (−0.5 ±2.2) | 49.0 (−8.3 ±3.3) | 38.5 (−5.7 ±3.4) |
| | | `counter_bonus_credits` 6 | | 74.0 (−6.8 ±3.7) | 52.1 (−5.2 ±3.8) | 41.7 (−2.6 ±4.2) |
| | | `corrupt_cut` 2,1,1,0 | | 77.6 (−3.1 ±2.7) | 57.8 (+0.5) | 45.3 (+1.0) |

The prompt's smaller d3 step (one stage-II card out, one stage-I card back, against `stage2_removed` 2) made d3
16.7 points easier on the same seeds: a longer game is an easier one. Not taken.

**The rally read on and off** (`smoke151-rally.json`, 36800-36815, d0 and d2, 48 games a cell): the strategist
d0 93.8 / 97.9, d2 79.2 / 72.9 (on / off); greedy d0 81.2 / 68.8, d2 81.2 / 89.6. Neutral within the noise; kept
(the rule's forecast, as 147's corruption risk is). Cost per decision on the same games (`per_decision.py`, seed
1000, one thread, the box busy): greedy 1.19× (17.0 → 20.3 ms), the strategist 0.97× (2.5 → 2.4 ms).
