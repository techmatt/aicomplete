# 283 screens (dev seeds 36800-36831, three pairs, d0-d4, 480 games an arm per agent, r135)

Each arm paired against base (the agents as built) on the same pair, level and seed: win % and the gap ± one SE. Arms: `make_screen.py`; reads: `scripts/bench/read_lib.py FILE --agent-arms`.

## Screen a (base, T, Tnk, K, D, A4, A4s)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | base | 99.0 | 84.4 | 79.2 | 59.4 | 28.1 | 70.0 |  | 480 |
| strategist | T (table at the anchors, keep 8) | 96.9 | 86.5 | 83.3 | 57.3 | 24.0 | 69.6 | -0.4 ± 2.0 | 480 |
| strategist | Tnk (T without the keep rows) | 96.9 | 86.5 | 83.3 | 57.3 | 24.0 | 69.6 | -0.4 ± 2.0 | 480 |
| strategist | K (keep_units) | 96.9 | 83.3 | 80.2 | 57.3 | 29.2 | 69.4 | -0.6 ± 1.0 | 480 |
| strategist | D (D106 research_delay) | 99.0 | 84.4 | 79.2 | 59.4 | 28.1 | 70.0 | +0.0 ± 0.0 | 480 |
| strategist | A4 (w.antimatter 4) | 95.8 | 87.5 | 79.2 | 52.1 | 29.2 | 68.8 | -1.2 ± 1.5 | 480 |
| strategist | A4s (A4 + w.sink 1) | 91.7 | 83.3 | 80.2 | 55.2 | 30.2 | 68.1 | -1.9 ± 1.7 | 480 |
| greedy | base | 96.9 | 84.4 | 79.2 | 49.0 | 22.9 | 66.5 |  | 480 |
| greedy | T (table at the anchors, keep 8) | 94.8 | 80.2 | 81.2 | 44.8 | 20.8 | 64.4 | -2.1 ± 2.1 | 480 |
| greedy | Tnk (T without the keep rows) | 94.8 | 80.2 | 81.2 | 44.8 | 20.8 | 64.4 | -2.1 ± 2.1 | 480 |
| greedy | K (keep_units) | 95.8 | 86.5 | 79.2 | 46.9 | 22.9 | 66.2 | -0.2 ± 1.2 | 480 |
| greedy | D (D106 research_delay) | 96.9 | 84.4 | 79.2 | 49.0 | 22.9 | 66.5 | +0.0 ± 0.0 | 480 |
| greedy | A4 (w.antimatter 4) | 96.9 | 88.5 | 77.1 | 47.9 | 26.0 | 67.3 | +0.8 ± 1.6 | 480 |
| greedy | A4s (A4 + w.sink 1) | 94.8 | 91.7 | 78.1 | 44.8 | 25.0 | 66.9 | +0.4 ± 1.9 | 480 |

Paired gaps by level:

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | T (table at the anchors, keep 8) | -2.1 ± 2.1 | +2.1 ± 4.4 | +4.2 ± 4.7 | -2.1 ± 5.7 | -4.2 ± 4.2 |
| strategist | Tnk (T without the keep rows) | -2.1 ± 2.1 | +2.1 ± 4.4 | +4.2 ± 4.7 | -2.1 ± 5.7 | -4.2 ± 4.2 |
| strategist | K (keep_units) | -2.1 ± 1.5 | -1.0 ± 2.8 | +1.0 ± 1.8 | -2.1 ± 2.6 | +1.0 ± 1.8 |
| strategist | D (D106 research_delay) | +0.0 ± 0.0 | +0.0 ± 0.0 | +0.0 ± 0.0 | +0.0 ± 0.0 | +0.0 ± 0.0 |
| strategist | A4 (w.antimatter 4) | -3.1 ± 1.8 | +3.1 ± 4.3 | +0.0 ± 3.6 | -7.3 ± 3.7 | +1.0 ± 2.8 |
| strategist | A4s (A4 + w.sink 1) | -7.3 ± 2.7 | -1.0 ± 4.6 | +1.0 ± 3.5 | -4.2 ± 4.2 | +2.1 ± 3.6 |
| greedy | T (table at the anchors, keep 8) | -2.1 ± 2.6 | -4.2 ± 5.1 | +2.1 ± 4.9 | -4.2 ± 5.7 | -2.1 ± 4.2 |
| greedy | Tnk (T without the keep rows) | -2.1 ± 2.6 | -4.2 ± 5.1 | +2.1 ± 4.9 | -4.2 ± 5.7 | -2.1 ± 4.2 |
| greedy | K (keep_units) | -1.0 ± 1.0 | +2.1 ± 2.6 | +0.0 ± 3.0 | -2.1 ± 3.0 | +0.0 ± 3.0 |
| greedy | D (D106 research_delay) | +0.0 ± 0.0 | +0.0 ± 0.0 | +0.0 ± 0.0 | +0.0 ± 0.0 | +0.0 ± 0.0 |
| greedy | A4 (w.antimatter 4) | +0.0 ± 1.5 | +4.2 ± 4.4 | -2.1 ± 3.9 | -1.0 ± 4.1 | +3.1 ± 3.5 |
| greedy | A4s (A4 + w.sink 1) | -2.1 ± 2.6 | +7.3 ± 4.0 | -1.0 ± 4.3 | -4.2 ± 5.1 | +2.1 ± 4.4 |

## Screen b (base, Tr, Trm, Th, Tf, A3, A3s)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | base | 99.0 | 84.4 | 79.2 | 59.4 | 28.1 | 70.0 |  | 480 |
| strategist | Tr (resources and drawn card at the anchors) | 99.0 | 91.7 | 84.4 | 55.2 | 29.2 | 71.9 | +1.9 ± 1.8 | 480 |
| strategist | Trm (agent removed 3.33, drain 1.33, discard 10) | 94.8 | 84.4 | 77.1 | 49.0 | 26.0 | 66.2 | -3.8 ± 2.0 | 480 |
| strategist | Th (horizon_t2 3.5) | 99.0 | 88.5 | 81.2 | 59.4 | 28.1 | 71.2 | +1.2 ± 0.7 | 480 |
| strategist | Tf (tag_share 0.5) | 95.8 | 92.7 | 83.3 | 57.3 | 28.1 | 71.5 | +1.5 ± 1.6 | 480 |
| strategist | A3 (w.antimatter 3) | 95.8 | 89.6 | 79.2 | 54.2 | 31.2 | 70.0 | +0.0 ± 1.4 | 480 |
| strategist | A3s (A3 + w.sink 1) | 92.7 | 90.6 | 77.1 | 55.2 | 29.2 | 69.0 | -1.0 ± 1.6 | 480 |
| greedy | base | 96.9 | 84.4 | 79.2 | 49.0 | 22.9 | 66.5 |  | 480 |
| greedy | Tr (resources and drawn card at the anchors) | 93.8 | 89.6 | 81.2 | 47.9 | 21.9 | 66.9 | +0.4 ± 2.1 | 480 |
| greedy | Trm (agent removed 3.33, drain 1.33, discard 10) | 94.8 | 87.5 | 71.9 | 42.7 | 25.0 | 64.4 | -2.1 ± 2.0 | 480 |
| greedy | Th (horizon_t2 3.5) | 96.9 | 84.4 | 79.2 | 51.0 | 24.0 | 67.1 | +0.6 ± 0.6 | 480 |
| greedy | Tf (tag_share 0.5) | 96.9 | 86.5 | 71.9 | 47.9 | 20.8 | 64.8 | -1.7 ± 1.3 | 480 |
| greedy | A3 (w.antimatter 3) | 94.8 | 93.8 | 79.2 | 46.9 | 24.0 | 67.7 | +1.2 ± 1.4 | 480 |
| greedy | A3s (A3 + w.sink 1) | 93.8 | 88.5 | 78.1 | 46.9 | 20.8 | 65.6 | -0.8 ± 1.8 | 480 |

Paired gaps by level:

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | Tr (resources and drawn card at the anchors) | +0.0 ± 1.5 | +7.3 ± 4.0 | +5.2 ± 4.0 | -4.2 ± 5.5 | +1.0 ± 4.1 |
| strategist | Trm (agent removed 3.33, drain 1.33, discard 10) | -4.2 ± 2.5 | +0.0 ± 4.7 | -2.1 ± 4.4 | -10.4 ± 5.4 | -2.1 ± 4.4 |
| strategist | Th (horizon_t2 3.5) | +0.0 ± 0.0 | +4.2 ± 2.5 | +2.1 ± 2.1 | +0.0 ± 0.0 | +0.0 ± 1.5 |
| strategist | Tf (tag_share 0.5) | -3.1 ± 2.3 | +8.3 ± 3.5 | +4.2 ± 4.2 | -2.1 ± 3.9 | +0.0 ± 3.6 |
| strategist | A3 (w.antimatter 3) | -3.1 ± 1.8 | +5.2 ± 4.0 | +0.0 ± 3.6 | -5.2 ± 3.1 | +3.1 ± 3.1 |
| strategist | A3s (A3 + w.sink 1) | -6.2 ± 2.5 | +6.2 ± 3.9 | -2.1 ± 4.2 | -4.2 ± 3.9 | +1.0 ± 3.5 |
| greedy | Tr (resources and drawn card at the anchors) | -3.1 ± 3.1 | +5.2 ± 5.0 | +2.1 ± 4.9 | -1.0 ± 5.4 | -1.0 ± 4.3 |
| greedy | Trm (agent removed 3.33, drain 1.33, discard 10) | -2.1 ± 2.6 | +3.1 ± 4.8 | -7.3 ± 5.4 | -6.2 ± 5.3 | +2.1 ± 3.0 |
| greedy | Th (horizon_t2 3.5) | +0.0 ± 0.0 | +0.0 ± 2.1 | +0.0 ± 0.0 | +2.1 ± 1.5 | +1.0 ± 1.0 |
| greedy | Tf (tag_share 0.5) | +0.0 ± 1.5 | +2.1 ± 3.0 | -7.3 ± 3.4 | -1.0 ± 2.8 | -2.1 ± 3.6 |
| greedy | A3 (w.antimatter 3) | -2.1 ± 2.1 | +9.4 ± 3.7 | +0.0 ± 3.3 | -2.1 ± 3.3 | +1.0 ± 3.1 |
| greedy | A3s (A3 + w.sink 1) | -3.1 ± 2.3 | +4.2 ± 4.2 | -1.0 ± 4.6 | -2.1 ± 4.9 | -2.1 ± 4.2 |

## Screen c (base, C, CK, CP)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | base | 99.0 | 84.4 | 79.2 | 59.4 | 28.1 | 70.0 |  | 480 |
| strategist | C (Tr + keep + Th + w.antimatter 3 + D106) | 92.7 | 88.5 | 80.2 | 56.2 | 28.1 | 69.2 | -0.8 ± 2.0 | 480 |
| strategist | CK (C + keep_units) | 91.7 | 87.5 | 84.4 | 56.2 | 31.2 | 70.2 | +0.2 ± 2.1 | 480 |
| strategist | CP (C + plan_target) | 91.7 | 85.4 | 77.1 | 56.2 | 29.2 | 67.9 | -2.1 ± 2.1 | 480 |
| greedy | base | 96.9 | 84.4 | 79.2 | 49.0 | 22.9 | 66.5 |  | 480 |
| greedy | C (Tr + keep + Th + w.antimatter 3 + D106) | 93.8 | 86.5 | 82.3 | 47.9 | 22.9 | 66.7 | +0.2 ± 2.1 | 480 |
| greedy | CK (C + keep_units) | 94.8 | 86.5 | 82.3 | 51.0 | 19.8 | 66.9 | +0.4 ± 2.0 | 480 |
| greedy | CP (C + plan_target) | 94.8 | 87.5 | 79.2 | 46.9 | 25.0 | 66.7 | +0.2 ± 2.2 | 480 |

Paired gaps by level:

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | C (Tr + keep + Th + w.antimatter 3 + D106) | -6.2 ± 2.9 | +4.2 ± 4.7 | +1.0 ± 4.3 | -3.1 ± 5.6 | +0.0 ± 4.4 |
| strategist | CK (C + keep_units) | -7.3 ± 3.1 | +3.1 ± 4.3 | +5.2 ± 4.3 | -3.1 ± 6.4 | +3.1 ± 4.3 |
| strategist | CP (C + plan_target) | -7.3 ± 3.1 | +1.0 ± 4.8 | -2.1 ± 4.7 | -3.1 ± 5.4 | +1.0 ± 4.8 |
| greedy | C (Tr + keep + Th + w.antimatter 3 + D106) | -3.1 ± 3.1 | +2.1 ± 5.1 | +3.1 ± 5.2 | -1.0 ± 5.2 | +0.0 ± 3.9 |
| greedy | CK (C + keep_units) | -2.1 ± 3.0 | +2.1 ± 5.1 | +3.1 ± 5.0 | +2.1 ± 4.7 | -3.1 ± 4.0 |
| greedy | CP (C + plan_target) | -2.1 ± 3.0 | +3.1 ± 5.0 | +0.0 ± 5.7 | -2.1 ± 5.9 | +2.1 ± 3.9 |

## Screen d (base, R, RK, RKA)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | base | 99.0 | 84.4 | 79.2 | 59.4 | 28.1 | 70.0 |  | 480 |
| strategist | R (Tr + keep 2.67 / 6.93 + Th) | 99.0 | 91.7 | 85.4 | 56.2 | 27.1 | 71.9 | +1.9 ± 1.8 | 480 |
| strategist | R (Tr + keep 2.67 / 6.93 + Th) | 99.0 | 89.6 | 85.4 | 59.4 | 26.0 | 71.9 | +1.9 ± 1.9 | 480 |
| strategist | R (Tr + keep 2.67 / 6.93 + Th) | 91.7 | 87.5 | 84.4 | 56.2 | 31.2 | 70.2 | +0.2 ± 2.1 | 480 |
| greedy | base | 96.9 | 84.4 | 79.2 | 49.0 | 22.9 | 66.5 |  | 480 |
| greedy | R (Tr + keep 2.67 / 6.93 + Th) | 93.8 | 90.6 | 81.2 | 46.9 | 21.9 | 66.9 | +0.4 ± 2.1 | 480 |
| greedy | R (Tr + keep 2.67 / 6.93 + Th) | 95.8 | 87.5 | 82.3 | 49.0 | 19.8 | 66.9 | +0.4 ± 1.9 | 480 |
| greedy | R (Tr + keep 2.67 / 6.93 + Th) | 94.8 | 86.5 | 82.3 | 51.0 | 19.8 | 66.9 | +0.4 ± 2.0 | 480 |

Paired gaps by level:

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | R (Tr + keep 2.67 / 6.93 + Th) | +0.0 ± 1.5 | +7.3 ± 4.0 | +6.2 ± 3.9 | -3.1 ± 5.6 | -1.0 ± 4.1 |
| strategist | R (Tr + keep 2.67 / 6.93 + Th) | +0.0 ± 1.5 | +5.2 ± 4.3 | +6.2 ± 3.9 | +0.0 ± 5.7 | -2.1 ± 4.2 |
| strategist | R (Tr + keep 2.67 / 6.93 + Th) | -7.3 ± 3.1 | +3.1 ± 4.3 | +5.2 ± 4.3 | -3.1 ± 6.4 | +3.1 ± 4.3 |
| greedy | R (Tr + keep 2.67 / 6.93 + Th) | -3.1 ± 3.1 | +6.2 ± 4.9 | +2.1 ± 4.9 | -2.1 ± 5.5 | -1.0 ± 4.3 |
| greedy | R (Tr + keep 2.67 / 6.93 + Th) | -1.0 ± 2.8 | +3.1 ± 4.8 | +3.1 ± 4.6 | +0.0 ± 4.9 | -3.1 ± 4.0 |
| greedy | R (Tr + keep 2.67 / 6.93 + Th) | -2.1 ± 3.0 | +2.1 ± 5.1 | +3.1 ± 5.0 | +2.1 ± 4.7 | -3.1 ± 4.0 |

## Screen e (base = the shipped build, am12)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | base | 99.0 | 89.6 | 85.4 | 59.4 | 26.0 | 71.9 |  | 480 |
| strategist | am12 (the table's antimatter 12, not the ruling's 18) | 92.7 | 84.4 | 86.5 | 58.3 | 31.2 | 70.6 | -1.2 ± 1.6 | 480 |
| greedy | base | 95.8 | 87.5 | 82.3 | 49.0 | 19.8 | 66.9 |  | 480 |
| greedy | am12 (the table's antimatter 12, not the ruling's 18) | 97.9 | 83.3 | 77.1 | 49.0 | 21.9 | 65.8 | -1.0 ± 1.2 | 480 |

Paired gaps by level:

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | am12 (the table's antimatter 12, not the ruling's 18) | -6.2 ± 2.5 | -5.2 ± 3.1 | +1.0 ± 3.8 | -1.0 ± 4.6 | +5.2 ± 4.0 |
| greedy | am12 (the table's antimatter 12, not the ruling's 18) | +2.1 ± 2.1 | -4.2 ± 3.6 | -5.2 ± 3.4 | +0.0 ± 2.6 | +2.1 ± 1.5 |

## Screen f (base = the shipped build, S1, A3s)

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap ± SE (all) | n |
|---|---|---|---|---|---|---|---|---|---|
| strategist | base | 99.0 | 89.6 | 85.4 | 59.4 | 26.0 | 71.9 |  | 480 |
| strategist | S1 (w.sink 1) | 95.8 | 89.6 | 80.2 | 60.4 | 28.1 | 70.8 | -1.0 ± 1.1 | 480 |
| strategist | A3s (w.antimatter 3 + w.sink 1) | 87.5 | 89.6 | 83.3 | 56.2 | 32.3 | 69.8 | -2.1 ± 1.5 | 480 |
| greedy | base | 95.8 | 87.5 | 82.3 | 49.0 | 19.8 | 66.9 |  | 480 |
| greedy | S1 (w.sink 1) | 96.9 | 85.4 | 81.2 | 45.8 | 20.8 | 66.0 | -0.8 ± 1.5 | 480 |
| greedy | A3s (w.antimatter 3 + w.sink 1) | 91.7 | 85.4 | 79.2 | 50.0 | 20.8 | 65.4 | -1.5 ± 1.8 | 480 |

Paired gaps by level:

| agent | arm | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|---|
| strategist | S1 (w.sink 1) | -3.1 ± 2.3 | +0.0 ± 2.6 | -5.2 ± 2.7 | +1.0 ± 3.1 | +2.1 ± 1.5 |
| strategist | A3s (w.antimatter 3 + w.sink 1) | -11.5 ± 3.3 | +0.0 ± 2.6 | -2.1 ± 3.0 | -3.1 ± 4.6 | +6.2 ± 3.2 |
| greedy | S1 (w.sink 1) | +1.0 ± 1.8 | -2.1 ± 3.3 | -1.0 ± 3.8 | -3.1 ± 4.6 | +1.0 ± 2.3 |
| greedy | A3s (w.antimatter 3 + w.sink 1) | -4.2 ± 2.5 | -2.1 ± 4.2 | -3.1 ± 4.8 | +1.0 ± 4.8 | +1.0 ± 3.5 |
