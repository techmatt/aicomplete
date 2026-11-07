# 136: death audit (the 135 read, r54)

Every strategist game of 135's read (`experiments/read-135.json`: seeds 36392-36439, 48 a cell, pairs bp / br / pr,
d0-d2) that ended `node` (a seat with no nodes left), replayed on the frozen r54 binary (`out/r54`) with `--trace`
and read from the event log by [death_audit.py](death_audit.py). The cells reproduce 135's numbers (d2: B+P 64.6,
B+R 50.0, P+R 70.8). 116 node losses in 432 games: d0 25, d1 39, d2 52 (135's "52 of 55" at d2).

## Cause of the fatal hit (the removals in the phase that took the last node)

| | d0 | d1 | d2 | all |
|---|---|---|---|---|
| corruption (a city corrupting takes nodes on and next to it) | 17 | 14 | 33 | 64 |
| Discord: Two Fronts | 1 | 6 | 7 | 14 |
| Discord: Offensive | 0 | 8 | 4 | 12 |
| Discord: Propaganda / Firestorm / Incursion | 2 / 0 / 0 | 0 / 2 / 1 | 3 / 2 / 0 | 5 / 4 / 1 |
| adversary card: Border War ("each player destroys 1") | 5 | 8 | 2 | 15 |
| war | 0 | 0 | 1 | 1 |
| threats (uncountered or a node counter) | 0 | 0 | 0 | 0 |

Over the whole game the losing seat lost 3.38 nodes a game to corruption, 1.33 to Discord cards, 0.47 to adversary
cards, 0.20 to node counters it chose, 0.05 to war; it placed 4.4.

## Patterns

- **The fatal hit is one blow, not a slide.** In 82 of 116 the fatal phase took every node the seat held at the end
  of the round before. Nodes at the end of round R-1: 1 (41), 2 (30), 3 (24), 4+ (21); at R-2: mostly 3-4. Deaths
  come in rounds 4-7 (16 / 43 / 48 / 9), all after stage I's last-node safety ends.
- **Corruption takes the cluster.** Nodes go only next to the seat's own nodes (`board/node.rs` `node_hexes`), so a
  seat is one blob; at the last action phase before a fatal corruption 186 of the seat's 198 nodes stood within 1 of
  the city that corrupted, and in 52 of 64 every node did. In 36 of 64 two or more cities corrupted in the same
  phase (d2: 25 of 33): a lost node's marker counts toward `corrupt_at` on a city next door, so one corruption tips
  the next (`board.node_marker` 1, `corruption_reach` 1).
- **Corruption is often not visible a round ahead.** The fatal cities' adversary ambassadors + marker at the end of
  the action phase before: 0 (25), 1 (14), 2 (42), 3 (28) of 4; in 12 deaths every corrupting city stood at 0 (the
  build's gains, arrivals, spread 2 and the cascade move a city 4 in one round).
- **It is seat 0's country.** Seat 0 (Horizon) is the loser in 92 of 116, whatever its board: Builder 71 (as seat 0
  only), Protector 21 as seat 0 against 2 as seat 1 (bp), Researcher 22 (seat 1 only). A swap at d2 (`ai.boards=1,0`,
  same seeds) moves it with the seat: seat 0 Protector 13, seat 1 Builder 9 (bp: 16 and 1). Horizon #14 alone is the
  fatal city in 47 deaths, #6 in 25: Horizon #14 stands within the spread range (2) of both starting corrupted
  cities, Horizon #7 and Cintamani #10, and next to Cintamani #14's setup factory; Cintamani has no such city.
- **The Builder is energy-starved.** Energy when the run-up action phase began: 0 in 85 of 232, 1 in 43; placements
  in it: 0 in 104, 1 in 99, 2+ in 29. Every placement costs 1 energy + the track step, so the Builder places about one
  node a round, and corruption takes 2-4 at once.

## Was there a node-safe line?

Two tests, each a plausible line, not a proof (the replay can't play the alternative out):

1. **Energy:** in the two action phases before the hit, the seat's energy left plus energy spent off nodes (cards,
   counters, Bulwark, Fieldwork) would pay an unused node power (1 + the cheapest step; Buy In also 6 credits, Peer
   Review 3 cubes; Accord not counted) enough times to beat the hit (1 more node for a positional hit, N + 1 - held for
   "destroy N"). Yes in **14** of 116 (d2: 3). Energy spent off nodes in those phases: cards 81, threat counters ~60,
   Bulwark 13, Fieldwork 12. 72 of 116 ended both phases at 0 energy, and both node powers were mostly used.
2. **Placement (corruption only, hindsight):** a placement in the run-up went inside the eventual blast while a legal
   hex outside it existed (the script's legal-hex count matches the prompt's option count in 103 of 103). Yes in
   **42** of 64 corruption deaths (d2: 20 of 33); 82 of 103 run-up placements went inside the blast, 60 of them with
   an outside hex legal. With foresight only (a clean city at adversary ambassadors + marker >= 2): 119 of the losing
   seats' 514 placements went next to such a city, 95 of them with a legal hex that wasn't.

Either: **50 of 116** (d0 14 of 25, d1 14 of 39, d2 22 of 52). The 52 count-hit deaths (Discord, Border War) had 1
(32) or 2 (20) nodes and needed 1-2 more; the energy for them existed in 6.

## Conclusion: mostly rules, with one agent planning fault

- **Rules (about 57%, 66 of 116 with no safe line found):** the fatal hits are big and arrive at once (corruption
  takes the whole blob, cascading through markers; Two Fronts / Offensive / Border War take 1-2 from a seat holding
  1-2), placement is adjacency-bound, the Builder can't afford more than ~1 node a round, and the 2-country map puts
  seat 0 next to a city fed by both starting corruptions (79% of losses are seat 0). Candidates for the parent to flag
  to Matt, not to fix here: Horizon #14's position (map), the marker cascade, corruption not being absorbable.
- **Agents (50 of 116, 43%):** the agent places nodes into the reach of a city about to corrupt when a hex outside it
  is legal, and never keeps a node out of a known blast or ahead of a known count hit. Concretely:
  - `crates/aic-sim/src/agents/eval.rs:1216-1230`: `node_danger` charges `danger_share(count)` on the raw count of the
    seat's node hexes (`danger_share`, `eval.rs:398`); a node next to a city at 3 of 4 counts as safe as any other.
    `thin` (same count) feeds `rounds_left`'s risk at `eval.rs:1359` the same way.
  - `eval.rs:1199-1201`: `w.node` (4) per held hex, flat, with no exposure; `reach` (+0.5 per city in reach,
    `eval.rs:1207`), the city energy and `converter_city` make a hex next to a city the better pick.
  - `eval.rs:1140-1145`: a city's `pressure` and `at_risk` charge the city, never the nodes it will take with it.
  - `eval.rs:1248`: `discord_danger` reads the Discord deck's size only, not that the next tip's card makes each
    player destroy 1-2 against a seat holding 1-2; the face-up adversary card's "each player destroys 1" (Border War,
    stage II) is not read either. `guide.rs` `shelter_saves` reads these destroy lines, but only to price a Shelter.

## Proposed fix (smallest): count nodes that survive what is showing

One new eval weight, `node_exposure` (0 off, 1 on; eval weights live in `eval::Weights`, set by `w.node_exposure=1`
in the agent spec, not in `configs/agents/strategies.json`). In the per-seat loop at `eval.rs:1216`:

- `exposed[s]` = the seat's node hexes within `board.corruption_reach` of a clean city, each weighted by that city's
  `p²` = ((adversary ambassadors + marker + 1) / corrupt_at)², capped at 1 (the +1 is the next spread: 39 of 109
  fatal cities stood at 0-1, so the term must start early), taking the max over cities per node;
- `hit[s]` = the count a showing card makes each player destroy: the face-up adversary card's destroy line, and the
  Discord card the next tip would draw times the Discord bucket's fill (as `discord_tip` reads it);
- charge `danger_share(count - exposed - hit)` instead of `danger_share(count)`, and feed that `thin` to
  `rounds_left`; add `- w.node * exposed` so the placement prompt prefers the hex outside the reach.

With it the greedy pick sees a node outside a pressured city as worth up to 30-60 points when the seat is thin
(60 placements in the run-ups had such a hex legal), Remove on that city gains value, and a node placement beats a
card for the energy when the safe count is 1. On/off read: the 135 spec with agents `strategist` and
`strategist:w.node_exposure=1` (or the new default with `w.node_exposure=0`), dev seeds first (16-seed engine smoke,
CLAUDE.md), then fresh; rerun this audit on the new binary to read the safe-line count and seat 0's share.

## Commands

```sh
cd out/r54   # the frozen binary reads its own configs
for d in 0 1 2; do for p in "bp:" "br:--set ai.boards=0,2" "pr:--set ai.boards=1,2"; do n=${p%%:*}; s=${p#*:}
  nice -n 10 ./aic-sim.exe run configs/baseline.json --games 48 --seed 36392 --agent strategist \
    --set adversary.difficulty=$d $s --threads 3 --save ../death/rec_d${d}_$n --save-when node --save-max 48 \
    > ../death/sum_d${d}_$n.txt; done; done
cd ../..
python experiments/results/136_alignment_and_win/death_audit.py --bin out/r54/aic-sim.exe --out out/death \
  out/death/rec_d2_bp out/death/rec_d2_br out/death/rec_d2_pr out/death/rec_d1_* out/death/rec_d0_*
```

The script replays each record's seed and overrides with `run --games 1 --seed S --trace` (traces cached in
`out/death/traces/`, so delete them for a new binary) and writes `out/death/audit.json`. For a new agent spec, change
`--agent strategist` in both. About 2 minutes for the runs and 1 for the audit on 3 threads.

## Every loss

Columns: nodes at the end of rounds R-3..R; for each of the two action phases before the hit, nodes placed, energy at
its start > at its end, energy spent off nodes (o), credits and cubes left, the cheapest next step; extra placements
the energy test found / needed; for corruption, each fatal city's adversary ambassadors + marker at the action phase
before, how many of the seat's nodes were within 1 of them, and each run-up placement (IN or out of the blast, legal
hexes outside it / legal hexes).

<!-- table -->
| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |
|---|---|---|---|---|---|---|---|---|---|---|
| d2_bp | 36396 | 7 | 1 Protector | 5 6 4 0 | r5: +1 e4>2 (o1) c0 st0 / r6: +1 e5>0 (o1) c4 st0 | 0 / 1 | corrupt | corruption | 4 | Cintamani #7 0, Cintamani #10 0; exposed 4/4; run-up placements out(5/9 out),IN(2/6 out) |
| d2_bp | 36397 | 4 | 0 Builder | 2 3 3 0 | r2: +1 e1>0 c2 st0 / r3: +0 e0>0 c4 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 1; exposed 3/3; run-up placements IN(1/5 out) |
| d2_bp | 36398 | 6 | 0 Builder | 4 3 3 0 | r4: +0 e0>0 c4 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 2, Horizon #14 3, Cintamani #15 2; exposed 3/3 |
| d2_bp | 36402 | 6 | 0 Builder | 5 3 3 0 | r5: +0 e0>0 c2 st1 / r6: +0 e0>0 c12 st1 | 0 / 1 | war | war | 1 |  |
| d2_bp | 36403 | 4 | 0 Builder | 2 3 2 0 | r2: +1 e1>0 c0 st0 / r3: +0 e0>0 c2 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #6 3, Horizon #14 2; exposed 3/3; run-up placements IN(0/5 out) |
| d2_bp | 36407 | 6 | 0 Builder | 1 1 1 0 | r4: +2 e9>5 (o1) c1 st0 / r5: +2 e5>1 c12 st1 | 0 / 1 | build | adv: Border War | 1 |  |
| d2_bp | 36408 | 6 | 0 Builder | 4 4 3 0 | r4: +1 e2>0 c1 st0 / r5: +1 e2>1 (o1) c0 st1 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 1, Horizon #14 1; exposed 5/5; run-up placements IN(2/7 out),IN(3/8 out) |
| d2_bp | 36413 | 4 | 0 Builder | 2 3 1 0 | r2: +1 e1>0 c0 st0 / r3: +0 e0>0 c1 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d2_bp | 36416 | 4 | 0 Builder | 2 4 2 0 | r3: +0 e0>0 c0 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | produce | Discord: Offensive | 2 |  |
| d2_bp | 36420 | 6 | 0 Builder | 6 6 1 0 | r5: +1 e3>0 (o2) c0 st2 / r6: +0 e0>0 c12 st2 | 0 / 2 | bases | Discord: Offensive | 1 |  |
| d2_bp | 36423 | 4 | 0 Builder | 2 3 2 0 | r2: +1 e1>0 c10 st0 / r3: +0 e0>0 c10 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #6 2, Cintamani #15 3; exposed 3/3; run-up placements IN(1/5 out) |
| d2_bp | 36425 | 7 | 0 Builder | 1 2 1 0 | r5: +1 e1>0 c3 st1 / r6: +2 e10>5 (o1) c22 st2 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 0, Cintamani #10 2; exposed 3/3; run-up placements IN(1/4 out),IN(1/3 out),IN(1/5 out) |
| d2_bp | 36427 | 6 | 0 Builder | 5 5 1 0 | r4: +0 e1>1 c0 st1 / r5: +0 e1>1 c1 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 3, Cintamani #15 3; exposed 2/3 |
| d2_bp | 36428 | 5 | 0 Builder | 3 3 2 0 | r3: +0 e0>0 c2 st0 / r4: +3 e0>1 c0 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 2, Cintamani #15 2; exposed 2/2; run-up placements IN(0/6 out),IN(2/7 out),out(2/8 out) |
| d2_bp | 36429 | 5 | 0 Builder | 4 4 3 0 | r3: +0 e0>0 c2 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 2, Horizon #14 1, Cintamani #15 3; exposed 3/4 |
| d2_bp | 36432 | 5 | 0 Builder | 3 3 2 0 | r4: +0 e0>0 c0 st0 / r5: +1 e2>0 (o1) c0 st0 | 0 / 2 | execute | Discord: Two Fronts | 1 |  |
| d2_bp | 36437 | 6 | 0 Builder | 3 4 2 0 | r5: +1 e1>0 c16 st1 / r6: +1 e5>0 (o3) c1 st1 | 0 / 1 | produce | Discord: Propaganda | 2 |  |
| d2_br | 36395 | 5 | 0 Builder | 3 5 3 0 | r4: +2 e4>0 c7 st1 / r5: +0 e0>0 c6 st1 | 0 / 1 | produce | Discord: Two Fronts | 2 |  |
| d2_br | 36396 | 5 | 1 Researcher | 3 3 3 0 | r3: +1 e1>0 c2 st0 / r4: +1 e6>3 (o1) c3 st0 | 0 / 1 | corrupt | corruption | 3 | Cintamani #7 3; exposed 3/3; run-up placements out(4/7 out),IN(4/7 out) |
| d2_br | 36397 | 4 | 0 Builder | 2 3 3 0 | r2: +1 e1>0 c4 st0 / r3: +0 e0>0 c11 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 1; exposed 3/3; run-up placements IN(1/5 out) |
| d2_br | 36402 | 4 | 0 Builder | 2 4 5 0 | r2: +2 e3>0 (o1) c1 st0 / r3: +1 e1>0 c0 st1 | 0 / 1 | corrupt | corruption | 5 | Horizon #6 2, Horizon #14 3, Cintamani #7 3; exposed 5/5; run-up placements IN(0/5 out),IN(0/7 out),IN(0/8 out) |
| d2_br | 36404 | 4 | 0 Builder | 2 3 3 0 | r2: +1 e1>0 c11 st0 / r3: +0 e0>0 c13 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 3, Horizon #14 2, Cintamani #15 3; exposed 3/3; run-up placements IN(0/5 out) |
| d2_br | 36409 | 4 | 0 Builder | 2 4 1 0 | r2: +2 e2>0 c0 st0 / r3: +0 e0>0 c1 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 0, Cintamani #7 0; exposed 2/2; run-up placements out(2/5 out),IN(3/6 out) |
| d2_br | 36410 | 5 | 0 Builder | 3 5 5 0 | r3: +2 e1>0 c1 st0 / r4: +0 e0>0 c1 st0 | 0 / 1 | corrupt | corruption | 4 | Horizon #6 2, Horizon #14 2; exposed 5/5; run-up placements IN(2/7 out),IN(2/8 out) |
| d2_br | 36411 | 6 | 1 Researcher | 7 3 4 0 | r5: +1 e8>5 c0 q2 st2 / r6: +2 e8>5 (o3) c2 st3 | 0 / 1 | produce | Discord: Propaganda | 2 |  |
| d2_br | 36412 | 6 | 0 Builder | 4 5 1 0 | r5: +1 e4>0 c12 st0 / r6: +1 e1>0 c1 st1 | 0 / 1 | bases | Discord: Firestorm | 2 |  |
| d2_br | 36413 | 4 | 0 Builder | 2 3 3 0 | r2: +1 e1>0 c1 st0 / r3: +0 e1>0 (o1) c1 st0 | 1 / 1 | corrupt | corruption | 2 | Horizon #6 3; exposed 3/3; run-up placements IN(1/5 out) |
| d2_br | 36416 | 5 | 1 Researcher | 2 3 2 0 | r3: +1 e1>0 c2 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 2, Cintamani #10 2; exposed 1/2; run-up placements IN(3/6 out) |
| d2_br | 36418 | 4 | 1 Researcher | 2 3 3 0 | r2: +1 e1>0 c2 st0 / r3: +1 e1>0 c1 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 2, Cintamani #7 1; exposed 2/3; run-up placements IN(2/6 out),IN(2/7 out) |
| d2_br | 36420 | 6 | 0 Builder | 4 5 1 0 | r5: +0 e1>1 c1 st2 / r6: +1 e1>0 c1 st2 | 0 / 1 | produce | Discord: Offensive | 2 |  |
| d2_br | 36421 | 6 | 0 Builder | 4 1 2 0 | r4: +0 e0>0 c2 st0 / r5: +2 e6>5 c1 st1 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 0; exposed 3/3; run-up placements IN(0/3 out),IN(1/5 out) |
| d2_br | 36425 | 5 | 1 Researcher | 3 4 2 0 | r4: +0 e0>0 c0 st0 / r5: +1 e6>5 c9 st1 | 0 / 1 | produce | Discord: Two Fronts | 2 |  |
| d2_br | 36427 | 5 | 0 Builder | 4 5 6 0 | r3: +1 e2>0 c1 st0 / r4: +1 e1>0 c0 st1 | 0 / 1 | corrupt | corruption | 6 | Horizon #6 2, Horizon #14 2, Cintamani #15 2; exposed 6/6; run-up placements IN(1/9 out),IN(1/10 out) |
| d2_br | 36428 | 5 | 0 Builder | 3 4 2 0 | r3: +1 e1>0 c2 st0 / r4: +1 e18>14 (o2) c0 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 1, Cintamani #15 3; exposed 3/3; run-up placements out(2/7 out),IN(0/5 out) |
| d2_br | 36429 | 5 | 0 Builder | 3 3 2 0 | r3: +0 e0>0 c0 st0 / r4: +1 e1>0 c1 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #6 2, Horizon #14 2, Cintamani #7 1, Cintamani #15 2; exposed 2/2; run-up placements IN(0/7 out) |
| d2_br | 36430 | 4 | 1 Researcher | 2 3 4 0 | r3: +1 e1>0 c1 st0 / r4: +0 e0>0 c0 st0 | 0 / 2 | execute | Discord: Firestorm | 1 |  |
| d2_br | 36436 | 4 | 0 Builder | 2 3 3 0 | r2: +1 e1>0 c13 st0 / r3: +1 e1>0 c15 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 1, Cintamani #15 3; exposed 4/4; run-up placements IN(1/5 out),IN(2/5 out) |
| d2_br | 36437 | 6 | 1 Researcher | 3 3 2 0 | r5: +0 e1>0 (o1) c0 st1 / r6: +0 e1>0 (o1) c8 st1 | 0 / 2 | produce | Discord: Two Fronts | 1 |  |
| d2_br | 36438 | 6 | 0 Builder | 3 4 5 0 | r4: +2 e4>0 (o2) c3 st1 / r5: +1 e0>0 c3 st1 | 0 / 1 | corrupt | corruption | 5 | Horizon #6 0, Horizon #14 2; exposed 5/5; run-up placements IN(1/7 out),out(1/7 out),IN(0/6 out) |
| d2_br | 36439 | 5 | 0 Builder | 3 3 1 0 | r3: +0 e0>0 c0 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 0, Cintamani #7 2; exposed 1/1 |
| d2_pr | 36398 | 6 | 0 Protector | 5 5 3 0 | r4: +1 e9>5 (o3) c0 st1 / r5: +0 e8>0 (o8) c6 st1 | 1 / 1 | corrupt | corruption | 3 | Horizon #6 0; exposed 3/3; run-up placements out(6/9 out) |
| d2_pr | 36402 | 4 | 0 Protector | 2 3 2 0 | r2: +1 e4>1 (o2) c2 st0 / r3: +1 e3>1 c2 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #6 3, Cintamani #7 3; exposed 2/2; run-up placements out(2/5 out),IN(1/4 out) |
| d2_pr | 36404 | 5 | 0 Protector | 3 3 1 0 | r3: +0 e2>0 (o2) c1 st0 / r4: +1 e5>1 (o2) c8 st0 | 1 / 1 | corrupt | corruption | 1 | Horizon #6 2, Cintamani #7 3; exposed 1/2; run-up placements out(1/4 out) |
| d2_pr | 36409 | 6 | 0 Protector | 4 3 3 0 | r5: +1 e5>2 (o1) c3 st2 / r6: +1 e3>0 c18 st2 | 0 / 1 | produce | Discord: Propaganda | 2 |  |
| d2_pr | 36410 | 6 | 1 Researcher | 2 2 1 0 | r5: +1 e3>0 (o1) c0 st0 / r6: +0 e0>0 c8 st0 | 0 / 2 | produce | Discord: Two Fronts | 1 |  |
| d2_pr | 36420 | 6 | 0 Protector | 5 5 1 0 | r5: +0 e2>2 c0 st2 / r6: +1 e4>0 c7 st2 | 0 / 1 | produce | Discord: Offensive | 2 |  |
| d2_pr | 36426 | 6 | 1 Researcher | 3 3 1 0 | r5: +0 e0>0 c13 st0 / r6: +0 e0>0 c13 q1 st0 | 0 / 2 | produce | Discord: Two Fronts | 1 |  |
| d2_pr | 36428 | 5 | 0 Protector | 4 4 3 0 | r3: +1 e3>0 (o2) c1 st0 / r4: +1 e3>0 c1 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #14 0, Cintamani #15 2; exposed 3/3; run-up placements out(2/8 out),IN(0/7 out) |
| d2_pr | 36429 | 4 | 0 Protector | 2 2 2 0 | r2: +1 e3>0 (o2) c1 st0 / r3: +1 e2>0 c3 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 0; exposed 2/2; run-up placements IN(2/5 out),IN(0/2 out) |
| d2_pr | 36431 | 4 | 0 Protector | 2 4 2 0 | r2: +2 e3>0 c6 st0 / r3: +1 e3>0 c8 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 1, Cintamani #7 3, Cintamani #15 2; exposed 2/2; run-up placements out(2/5 out),out(3/6 out),IN(0/3 out) |
| d2_pr | 36432 | 6 | 1 Researcher | 3 2 2 0 | r4: +1 e7>5 c4 st0 / r5: +1 e5>0 (o2) c0 st0 | 0 / 1 | corrupt | corruption | 2 | Cintamani #7 3; exposed 3/3; run-up placements IN(3/6 out),IN(4/7 out) |
| d2_pr | 36433 | 5 | 1 Researcher | 4 4 2 0 | r4: +0 e0>0 c2 st0 / r5: +0 e0>0 c15 q1 st0 | 0 / 2 | produce | Discord: Two Fronts | 1 |  |
| d1_bp | 36396 | 6 | 0 Builder | 4 3 3 0 | r5: +1 e1>0 c2 st1 / r6: +2 e0>2 c36 st2 | 0 / 1 | produce | Discord: Offensive | 2 |  |
| d1_bp | 36398 | 6 | 0 Builder | 4 1 1 0 | r4: +0 e0>0 c2 st0 / r5: +0 e0>0 c2 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d1_bp | 36403 | 5 | 0 Builder | 3 2 2 0 | r4: +2 e0>0 (o1) c0 st0 / r5: +0 e0>0 c1 st0 | 0 / 1 | bases | Discord: Firestorm | 2 |  |
| d1_bp | 36407 | 5 | 0 Builder | 3 1 1 0 | r4: +2 e9>5 (o1) c2 st0 / r5: +2 e5>1 c8 st1 | 0 / 1 | produce | Discord: Incursion | 2 |  |
| d1_bp | 36409 | 6 | 0 Builder | 4 1 1 0 | r4: +0 e0>0 c7 st0 / r5: +0 e0>0 c3 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d1_bp | 36415 | 6 | 0 Builder | 5 7 3 0 | r5: +1 e2>1 (o2) c11 st1 / r6: +3 e4>0 (o4) c1 st2 | 2 / 1 | produce | Discord: Offensive | 2 |  |
| d1_bp | 36416 | 5 | 0 Builder | 4 2 2 0 | r4: +0 e0>0 c2 st0 / r5: +0 e0>0 c3 st0 | 0 / 2 | produce | Discord: Offensive | 1 |  |
| d1_bp | 36417 | 6 | 1 Protector | 5 5 1 0 | r5: +1 e2>0 (o1) c13 st1 / r6: +1 e3>0 (o1) c5 st2 | 0 / 1 | produce | Discord: Offensive | 2 |  |
| d1_bp | 36420 | 6 | 0 Builder | 6 4 4 0 | r4: +1 e2>0 c9 st2 / r5: +0 e1>1 c1 st2 | 0 / 1 | corrupt | corruption | 4 | Horizon #14 2; exposed 4/4; run-up placements IN(3/7 out) |
| d1_bp | 36424 | 6 | 0 Builder | 4 5 1 0 | r4: +1 e9>5 (o2) c9 st0 / r5: +0 e5>5 c10 st0 | 3 / 1 | build | adv: Border War | 1 |  |
| d1_bp | 36427 | 5 | 0 Builder | 4 5 4 0 | r4: +0 e1>1 c1 st1 / r5: +0 e1>1 c9 st1 | 0 / 1 | produce | Discord: Offensive | 2 |  |
| d1_bp | 36429 | 6 | 0 Builder | 4 2 1 0 | r4: +0 e0>0 c1 st0 / r5: +1 e0>0 c2 st0 | 0 / 2 | build | Discord: Offensive | 1 |  |
| d1_bp | 36431 | 6 | 0 Builder | 3 1 2 0 | r4: +0 e0>0 c1 st0 / r5: +2 e3>0 c9 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 0, Cintamani #10 0; exposed 2/3; run-up placements IN(2/5 out),out(2/7 out) |
| d1_bp | 36432 | 5 | 0 Builder | 3 3 2 0 | r3: +0 e0>0 c2 st0 / r4: +0 e0>0 c2 st0 | 0 / 1 | build | adv: Border War | 2 |  |
| d1_br | 36395 | 6 | 0 Builder | 5 3 2 0 | r5: +0 e0>0 c6 st1 / r6: +0 e0>0 c7 st1 | 0 / 1 | produce | Discord: Two Fronts | 2 |  |
| d1_br | 36397 | 6 | 1 Researcher | 4 4 1 0 | r4: +1 e4>3 (o1) c1 st0 / r5: +1 e3>0 (o2) c3 q2 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #6 0, Cintamani #7 0; exposed 2/3; run-up placements IN(5/8 out),out(6/10 out) |
| d1_br | 36400 | 5 | 1 Researcher | 3 4 3 0 | r4: +0 e0>0 c2 st0 / r5: +0 e0>0 c3 st0 | 0 / 2 | produce | Discord: Two Fronts | 1 |  |
| d1_br | 36402 | 6 | 0 Builder | 5 2 2 0 | r4: +0 e0>0 c1 st1 / r5: +0 e0>0 c0 st1 | 0 / 1 | corrupt | corruption | 2 | Horizon #6 2; exposed 2/2 |
| d1_br | 36407 | 7 | 1 Researcher | 3 3 1 0 | r5: +0 e3>0 (o3) c2 q2 st0 / r6: +0 e0>0 c2 q2 st0 | 1 / 1 | build | adv: Border War | 1 |  |
| d1_br | 36410 | 6 | 1 Researcher | 2 2 1 0 | r4: +0 e0>0 c1 st0 / r5: +1 e6>3 (o1) c4 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d1_br | 36413 | 6 | 0 Builder | 3 3 2 0 | r5: +0 e0>0 c4 st0 / r6: +0 e0>0 c9 st0 | 0 / 2 | bases | Discord: Two Fronts | 1 |  |
| d1_br | 36416 | 5 | 0 Builder | 3 2 2 0 | r4: +0 e0>0 c2 st0 / r5: +0 e0>0 c1 st0 | 0 / 2 | produce | Discord: Offensive | 1 |  |
| d1_br | 36420 | 5 | 0 Builder | 3 4 6 0 | r3: +1 e1>0 c1 st1 / r4: +2 e6>1 c21 st2 | 0 / 1 | corrupt | corruption | 5 | Horizon #14 2; exposed 5/6; run-up placements IN(3/7 out),out(5/8 out),IN(7/10 out) |
| d1_br | 36421 | 6 | 0 Builder | 4 4 1 0 | r5: +1 e6>5 (o4) c0 st1 / r6: +1 e5>7 (o4) c0 st1 | 0 / 1 | produce | Discord: Offensive | 2 |  |
| d1_br | 36427 | 5 | 0 Builder | 4 5 2 0 | r3: +1 e2>0 c1 st0 / r4: +1 e1>0 c3 st1 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 1; exposed 2/2; run-up placements IN(4/9 out),IN(0/6 out) |
| d1_br | 36430 | 6 | 1 Researcher | 4 4 1 0 | r5: +0 e3>0 (o3) c0 st1 / r6: +0 e0>0 c3 q1 st1 | 1 / 2 | produce | Discord: Two Fronts | 1 |  |
| d1_br | 36431 | 5 | 0 Builder | 3 4 2 0 | r3: +1 e1>0 c2 st0 / r4: +2 e9>0 (o4) c1 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 0; exposed 3/3; run-up placements out(3/6 out),IN(1/4 out),IN(1/6 out) |
| d1_br | 36432 | 5 | 0 Builder | 3 3 4 0 | r4: +2 e6>0 (o3) c2 st0 / r5: +0 e0>0 c1 st0 | 0 / 2 | produce | Discord: Two Fronts | 1 |  |
| d1_br | 36433 | 6 | 1 Researcher | 3 3 2 0 | r5: +1 e3>0 (o2) c20 q1 st0 / r6: +0 e0>0 c0 q2 st0 | 0 / 2 | produce | Discord: Firestorm | 1 |  |
| d1_br | 36437 | 7 | 0 Builder | 5 3 1 0 | r5: +1 e7>5 c0 st1 / r6: +0 e6>6 c11 st1 | 2 / 1 | corrupt | corruption | 1 | Horizon #6 2; exposed 2/3; run-up placements IN(5/8 out) |
| d1_br | 36439 | 5 | 0 Builder | 3 3 1 0 | r3: +0 e0>0 c1 st0 / r4: +0 e0>0 c1 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 2; exposed 1/1 |
| d1_pr | 36400 | 5 | 0 Protector | 5 6 4 0 | r3: +1 e5>2 (o2) c1 st1 / r4: +2 e18>9 (o4) c3 st2 | 0 / 1 | corrupt | corruption | 4 | Horizon #14 0, Cintamani #7 2; exposed 4/4; run-up placements out(6/10 out),IN(1/7 out),IN(3/8 out) |
| d1_pr | 36404 | 5 | 0 Protector | 3 3 1 0 | r3: +0 e2>0 (o2) c1 st0 / r4: +1 e5>1 (o2) c14 st0 | 1 / 1 | corrupt | corruption | 1 | Horizon #6 2, Cintamani #7 3; exposed 1/2; run-up placements out(1/4 out) |
| d1_pr | 36409 | 6 | 1 Researcher | 4 2 1 0 | r4: +0 e0>0 c1 st0 / r5: +1 e8>2 (o4) c4 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d1_pr | 36412 | 7 | 1 Researcher | 2 2 1 0 | r5: +1 e6>5 (o1) c6 q1 st0 / r6: +1 e5>5 c9 q1 st0 | 0 / 1 | corrupt | corruption | 1 | Cintamani #7 0; exposed 2/3; run-up placements IN(2/4 out),IN(2/4 out) |
| d1_pr | 36420 | 5 | 0 Protector | 4 5 6 0 | r4: +1 e3>0 (o1) c11 st2 / r5: +0 e2>2 c4 st2 | 0 / 2 | produce | Discord: Two Fronts | 1 |  |
| d1_pr | 36428 | 6 | 0 Protector | 4 2 2 0 | r4: +1 e3>0 c1 st0 / r5: +1 e6>1 (o1) c4 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 2; exposed 3/3; run-up placements IN(0/3 out),IN(0/4 out) |
| d1_pr | 36432 | 7 | 0 Protector | 3 3 4 0 | r5: +0 e8>0 (o8) c7 st0 / r6: +1 e6>0 (o3) c21 st0 | 1 / 1 | corrupt | corruption | 4 | Horizon #7 2, Horizon #14 3, Cintamani #7 3; exposed 4/4; run-up placements IN(1/6 out) |
| d1_pr | 36433 | 6 | 1 Researcher | 4 4 1 0 | r4: +0 e0>0 c0 st0 / r5: +0 e0>0 c4 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d0_bp | 36393 | 6 | 0 Builder | 6 3 4 0 | r5: +1 e12>6 (o4) c0 st2 / r6: +1 e7>5 (o2) c10 st2 | 1 / 1 | produce | Discord: Two Fronts | 2 |  |
| d0_bp | 36398 | 7 | 0 Builder | 2 2 1 0 | r5: +0 e0>0 c2 st1 / r6: +2 e0>0 c0 st2 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 3, Cintamani #7 1; exposed 3/4; run-up placements out(2/7 out),IN(2/7 out) |
| d0_bp | 36400 | 5 | 0 Builder | 5 5 1 0 | r3: +0 e0>0 c0 st1 / r4: +0 e0>0 c2 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 2; exposed 1/1 |
| d0_bp | 36402 | 5 | 0 Builder | 3 3 1 0 | r3: +0 e0>0 c8 st0 / r4: +0 e0>0 c10 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 3; exposed 1/1 |
| d0_bp | 36407 | 7 | 0 Builder | 2 2 1 0 | r5: +1 e3>0 (o2) c0 st0 / r6: +0 e0>0 c8 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d0_bp | 36408 | 5 | 0 Builder | 4 5 4 0 | r3: +1 e1>0 c2 st1 / r4: +0 e1>1 c12 st1 | 0 / 1 | corrupt | corruption | 3 | Horizon #6 2, Horizon #14 0; exposed 5/5; run-up placements IN(1/9 out) |
| d0_bp | 36413 | 5 | 0 Builder | 3 4 1 0 | r3: +1 e1>0 c1 st0 / r4: +0 e0>0 c2 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d0_bp | 36420 | 6 | 0 Builder | 6 3 3 0 | r4: +0 e1>1 c1 st1 / r5: +2 e5>0 c11 st2 | 0 / 1 | corrupt | corruption | 3 | Horizon #14 3; exposed 4/4; run-up placements IN(2/6 out),IN(4/7 out) |
| d0_bp | 36432 | 5 | 0 Builder | 3 4 3 0 | r3: +1 e0>1 c0 st0 / r4: +0 e2>0 (o2) c2 st0 | 1 / 1 | corrupt | corruption | 1 | Horizon #6 2, Horizon #14 3; exposed 4/4; run-up placements IN(1/7 out) |
| d0_bp | 36433 | 6 | 0 Builder | 5 4 1 0 | r4: +0 e0>0 c2 st1 / r5: +0 e0>0 c4 st1 | 0 / 1 | build | adv: Border War | 1 |  |
| d0_br | 36400 | 5 | 0 Builder | 5 5 1 0 | r3: +0 e0>0 c0 st1 / r4: +1 e3>1 c2 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 2; exposed 2/2; run-up placements IN(0/6 out) |
| d0_br | 36402 | 5 | 0 Builder | 3 3 1 0 | r3: +0 e0>0 c1 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 3; exposed 1/1 |
| d0_br | 36414 | 6 | 0 Builder | 3 3 3 0 | r4: +2 e0>0 c1 st0 / r5: +1 e3>0 (o2) c0 st1 | 0 / 1 | corrupt | corruption | 3 | Horizon #14 0; exposed 3/3; run-up placements IN(0/3 out),IN(2/5 out),IN(4/6 out) |
| d0_br | 36416 | 6 | 1 Researcher | 4 4 1 0 | r4: +1 e6>4 (o1) c0 st1 / r5: +1 e4>1 (o1) c2 st1 | 0 / 1 | build | adv: Border War | 1 |  |
| d0_br | 36417 | 7 | 0 Builder | 5 5 4 0 | r6: +1 e4>0 c13 st0 / r7: +0 e0>0 c18 st0 | 0 / 2 | produce | Discord: Propaganda | 1 |  |
| d0_br | 36439 | 5 | 0 Builder | 3 4 1 0 | r3: +1 e1>0 c0 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #14 2; exposed 1/1; run-up placements out(3/8 out) |
| d0_pr | 36395 | 5 | 0 Protector | 3 4 2 0 | r3: +1 e4>1 (o1) c15 st0 / r4: +1 e3>0 c2 st0 | 0 / 1 | corrupt | corruption | 2 | Horizon #14 2; exposed 2/2; run-up placements IN(3/6 out),IN(0/3 out) |
| d0_pr | 36402 | 5 | 0 Protector | 3 4 3 0 | r3: +1 e2>0 c2 st0 / r4: +1 e14>6 (o5) c3 st0 | 0 / 1 | corrupt | corruption | 3 | Horizon #14 3; exposed 3/3; run-up placements IN(3/6 out),IN(1/6 out) |
| d0_pr | 36404 | 6 | 0 Protector | 4 4 3 0 | r4: +0 e6>0 (o6) c7 st0 / r5: +2 e7>0 (o6) c5 st0 | 1 / 1 | corrupt | corruption | 3 | Horizon #6 1, Cintamani #7 0; exposed 3/3; run-up placements IN(0/2 out),IN(1/3 out) |
| d0_pr | 36411 | 5 | 0 Protector | 4 6 5 0 | r3: +2 e12>5 (o3) c0 st0 / r4: +2 e8>2 (o1) c4 st1 | 0 / 1 | corrupt | corruption | 5 | Horizon #14 0, Cintamani #10 0; exposed 5/5; run-up placements IN(3/8 out),out(3/9 out),IN(0/6 out),IN(0/5 out) |
| d0_pr | 36420 | 5 | 0 Protector | 3 4 5 0 | r3: +1 e3>0 (o2) c4 st0 / r4: +1 e2>0 c0 st0 | 0 / 1 | corrupt | corruption | 5 | Horizon #14 2; exposed 5/5; run-up placements IN(3/7 out),IN(5/8 out) |
| d0_pr | 36428 | 6 | 1 Researcher | 4 5 5 0 | r5: +0 e1>0 (o1) c4 st1 / r6: +1 e1>0 (o1) c0 st1 | 0 / 1 | produce | Discord: Propaganda | 2 |  |
| d0_pr | 36429 | 6 | 0 Protector | 3 4 4 0 | r4: +2 e2>0 c0 st1 / r5: +0 e2>0 (o2) c0 st1 | 1 / 1 | corrupt | corruption | 4 | Horizon #14 0; exposed 4/4; run-up placements IN(1/6 out),IN(3/7 out) |
| d0_pr | 36430 | 5 | 0 Protector | 4 3 3 0 | r3: +1 e3>0 (o2) c2 st0 / r4: +1 e2>0 (o1) c2 st1 | 0 / 1 | corrupt | corruption | 3 | Horizon #14 1, Cintamani #7 0; exposed 4/4; run-up placements IN(1/5 out),IN(2/7 out) |
| d0_pr | 36433 | 6 | 0 Protector | 3 3 1 0 | r4: +0 e2>0 (o2) c1 st0 / r5: +0 e2>0 (o2) c2 st0 | 2 / 1 | build | adv: Border War | 1 |  |
