# 277: Growth as one easy option and two costly ones (r132)

**Ruling (Matt, 2026-11-18):** each board's Growth is one easy option and two more difficult or costly ones, still one
pick a round. The easy option is the floor: modest, never quietly the best pick. The two costly ones pull different ways.
`RULES_REVISION` 131 → 132 (bumped from the constant). Base: r131 (276). Every number below is a **candidate**.
Message 12682 (Matt's answer to question 1): the easy options' usage band is 0.03-0.45, Levy a node and 1 credit.

## The boards (done-when 1)

Give / get at **Matt's unit anchors** (`docs/card-guidelines.md`: chip 4, energy 3, insight 2.67, card 3.33,
ambassador 5.33; a node 5.33 and a station 6.67 from `values.json`) and at the agents' table (`values.json`: chip 2.67,
insight 1.67, card 2.67, ambassador 2.67), before each placement's track step, as before.

| Board | Option | r131 | r132 | Net at the anchors (r131 → r132) | Net at `values.json` (r131 → r132) |
|---|---|---|---|---|---|
| Builder | **Buy In** (easy) | free: node + chip | free: node + **1 credit** | +9.33 → **+6.33** | +8.00 → **+6.33** |
| Builder | Sprawl (costly: nodes) | 1 energy: 2 nodes | as was | +7.67 | +7.67 |
| Builder | Annex (costly: reach, credits) | free: node within 2 + 3 credits | **1 chip**: node within 2 + **6 credits** | +8.33 → **+7.33** and reach | +8.33 → **+8.67** and reach |
| Protector | **Levy** (easy) | free: node + 3 credits | free: node + **1 credit** | +8.33 → **+6.33** | +8.33 → **+6.33** |
| Protector | Outpost (costly: nodes) | 2 energy: 2 nodes | **1 energy**: 2 nodes | +4.67 → **+7.67** | +4.67 → **+7.67** |
| Protector | Accord (costly: ambassador) | 1 energy: node + ambassador | 1 energy: node + ambassador + **2 credits** | +7.67 → **+9.67** | +5.00 → **+7.00** |
| Researcher | **Field Survey** (easy) | free: node + card | as was | +8.67 | +8.00 |
| Researcher | Peer Review (costly: nodes) | 2 insight: 2 nodes | **1 insight**: 2 nodes + **2 credits** | +5.33 → **+10.00** | +7.33 → **+11.00** |
| Researcher | Field Station (costly: a station) | 1 insight: station + node | as was | +9.33 | +10.33 |

At r131 a free option out-earned a paid one on every board (Levy +8.33 against Outpost +4.67; Buy In +9.33 at the
anchors and Annex both free; Field Survey +8.67 against Peer Review +5.33). At r132 each costly option beats its
board's easy one when affordable under both readings (by 0.7-3.3 at the anchors, 0.7-3.0 at the table), and the two
pull different ways: two nodes for the board's currency against reach and credits (Annex), an ambassador and credits
(Accord) or a station (Field Station). Annex is a second Builder power spending chips, beside Fortify. No loop: one Growth a round;
Buy In no longer makes chips, so Annex's chip comes from Scrap I, Full Capacity and cards; nothing turns credits back
into chips.

**Usage** (design_check's bands, levels and agents pooled; a seat-round of the board):

| Board | Option | Band | r131 | r132 | Of the board's seat-rounds, by level d0-d4 (r132) |
|---|---|---|---|---|---|
| Builder | Buy In (easy) | 0.03-0.45 | 0.26 | **0.24** | 0.23 / 0.25 / 0.27 / 0.25 / 0.21 |
| Builder | Sprawl | 0.05-0.90 | 0.40 | **0.41** | 0.42 / 0.37 / 0.38 / 0.43 / 0.47 |
| Builder | Annex | 0.05-0.90 | 0.07 | **0.05** (at the floor) | 0.06 / 0.06 / 0.06 / 0.04 / 0.05 |
| Protector | Levy (easy) | 0.03-0.45 | 0.04 | **0.04** | 0.04 at every level |
| Protector | Outpost | 0.05-0.90 | 0.28 | **0.32** | 0.36 / 0.29 / 0.30 / 0.33 / 0.36 |
| Protector | Accord | 0.05-0.90 | 0.45 | **0.40** | 0.36 / 0.41 / 0.41 / 0.41 / 0.40 |
| Researcher | Field Survey (easy) | 0.03-0.45 | 0.34 | **0.29** | 0.31 / 0.29 / 0.29 / 0.27 / 0.26 |
| Researcher | Peer Review | 0.05-0.90 | 0.33 | **0.39** | 0.38 / 0.35 / 0.35 / 0.41 / 0.46 |
| Researcher | Field Station | 0.05-0.90 | 0.07 | **0.06** | 0.07 / 0.07 / 0.06 / 0.06 / 0.03 |

Every option is inside its band; on every board the easy option is taken less than the board's top costly one (Buy In
0.24 against Sprawl 0.41, Levy 0.04 against Accord 0.40, Field Survey 0.29 against Peer Review 0.39). Annex is at its
floor and the weak spot: a chip competes with Fortify (Fortify I + II 1.76 → 1.31 a seat-game, both in band). Patrol
I, over its band at r131 on these seeds (3.52), is in at r132 (3.46). Growth taken: the Builder 0.70 a seat-round
(r131 0.73), the Protector 0.76 (0.77), the Researcher 0.74 (0.74).

## Candidates (dev seeds 36800-36831, 960 games a side, against r131 on the same seeds)

- **A** (Annex 1 chip for 4 credits): Annex 0.03 a seat-round, under its 0.05 floor (r131 free with 3: 0.07); Fortify
  fell 1.88 → 1.45 a seat-game, the chips going to Annex. **B** (Annex 6 credits, shipped): Annex 0.06, Fortify 1.38.
- **Levy** is the floor: 0.04 a seat-round at 1 credit, 0.05 at r131's 3 (the rider barely moves it): it is taken
  when Outpost's and Accord's energy can't be paid. Question 1 / message 12682: the easy band is 0.03-0.45.
- **Peer Review** (diff review): at 1 insight for 2 nodes it netted +8.0 at the anchors, under Field Survey's +8.67
  (276's write-up, which I followed, priced at `values.json`); it gains 2 credits. The fix was decided at 4:38pm from
  the diff review, while the fresh seeds' first new side (1cac316) was still farming (done 4:42pm, read after the
  fix was committed at 4:43pm); the shipped side re-farmed the same seeds at 4c7ee82. The dev screens ran without
  the credits.
- **The second step:** a two-node option pays two track steps. Once the next steps cost energy its edge narrows
  (Sprawl and Outpost +4.67 after a 1-energy second step, under the easy +6.33); measured use still favors them.
- **Buy In** barely moved (0.28 → 0.27) from a chip to a credit; Field Survey fell 0.34 → 0.29 as Peer Review went
  to 1 insight (0.32 → 0.36).

## Paired read (done-when 2)

f277-read, fresh seeds 49980-50027 (taken), 1,440 games a side, the judge and greedy, three pairs, d0-d4. r132
farmed at 4c7ee82, r131 at 1d7a601 (276's own f276-read spec, the same grid). Gaps paired by seed, ± SE
([read.py](read.py)).

| Agent | Level | Win % r131 → r132 | Gap ± SE | Rounds r131 → r132 | Gap ± SE |
|---|---|---|---|---|---|
| judge | d0 | 88.2 → 88.9 | +0.7 ± 3.3 | 8.79 → 8.75 | −0.04 ± 0.24 |
| judge | d1 | 55.6 → 61.1 | +5.6 ± 4.3 | 9.88 → 9.81 | −0.07 ± 0.20 |
| judge | d2 | 37.5 → 52.1 | **+14.6 ± 4.5** | 9.18 → 9.21 | +0.03 ± 0.23 |
| judge | d3 | 8.3 → 18.8 | **+10.4 ± 3.4** | 8.03 → 7.81 | −0.22 ± 0.15 |
| judge | d4 | 2.8 → 2.8 | +0.0 ± 1.4 | 6.13 → 6.26 | +0.12 ± 0.13 |
| greedy | d0 | 86.1 → 87.5 | +1.4 ± 3.8 | 8.98 → 8.60 | −0.38 ± 0.25 |
| greedy | d1 | 59.0 → 61.8 | +2.8 ± 4.6 | 10.18 → 10.16 | −0.02 ± 0.22 |
| greedy | d2 | 36.1 → 36.1 | +0.0 ± 4.3 | 9.76 → 9.88 | +0.12 ± 0.22 |
| greedy | d3 | 6.2 → 10.4 | +4.2 ± 2.9 | 8.25 → 8.12 | −0.13 ± 0.15 |
| greedy | d4 | 2.8 → 2.1 | −0.7 ± 1.8 | 6.33 → 6.49 | +0.16 ± 0.14 |

**The judge wins more at d2-d3** (+14.6 and +10.4, about 3 SE each; d0-d3 pooled +7.8 ± 2.0, every pair up: bp +5,
br +8, pr +10); greedy doesn't move (d0-d3 +2.1 ± 2.0). Rounds move about 1.5 SE at most (judge d3 −0.22, greedy d0 −0.38). The costly
options got richer (Outpost and Peer Review cheaper, Accord and Peer Review 2 credits, Annex 6 credits) while the easy
ones got poorer, and the judge, which picks the costly ones when it can pay, gains; greedy less. Not chased (Matt:
win-rate drops wait until the overhaul is done; a rise is reported the same way). br stays the weakest pair (42.7%
d0-d3).

**Before the Peer Review fix**, the same seeds at 1cac316 (Peer Review 1 insight, no credits): judge d2 +8.3 ± 4.1,
d3 +4.2 ± 2.9, greedy d0-d4 within ±5.6. That pair of reads changed one thing (Peer Review's 2 credits), so about half
the judge's d2-d3 gain is Peer Review's credits: br and pr seat the Researcher.

## Checks

- **design_check** on the fresh read with `--old` r131: the report's warnings line. Against r131's own read on the
  same seeds: gone, Patrol I's usage miss (3.52 → 3.46), rounds d1 and table-components d1; new, rounds d3 8.0
  (r131 8.1, the target's floor 8), kept-not-played agency_directorate 0.03, victory-skips antimatter 71%.
- **Dev screens** (36800-36831): A (Annex 4 credits) and B (6) as above; `--quiet`: job-parity and friction-stale,
  both as on main.
- **Loop smoke** (16 seeds): no timeouts, no card at the play flag.
- **Speed:** `speed.py check` gives no verdict across the revision (a shared box: engine 1.39×, planner 1.24×, greedy
  0.70× a game). `per_decision.py` (d2, 6 games, best of 2, r131 built from 1d7a601): the judge 0.97× a decision (1.7
  → 1.7 ms), greedy 0.89× (1.5 → 1.3 ms). No agent code changed. The baseline is not re-based (the box was shared).
- **Diff review:** 6 found, 5 fixed (Peer Review under Field Survey at Matt's anchors; economy.md's insight row; the
  chip sources naming the Builder's tracks; band notes copied onto Buy In and Field Survey; the Builder's Growth said
  to spend city energy), 1 answered with a note, no rule change (a two-node option's second step: *Candidates*).

## New-player summary (r132)

- **Growth: one easy option, two costly ones.** Each round your board may take one Growth option. The easy one is free
  (Buy In and Levy: a node and a credit; Field Survey: a node and a card): take it when you can't pay for the others.
- **The costly two pull different ways.** More nodes (Sprawl and Outpost, 1 energy for 2 nodes; Peer Review, 1 insight
  for 2 nodes and 2 credits) or something else: Annex (1 chip: a node up to 2 hexes away and 6 credits), Accord (1
  energy: a node, an ambassador and 2 credits), Field Station (1 insight: a research station and a node).
- **Every node still pays its track space's energy**, so a two-node option pays two spaces.
- **Rules that resisted:** Annex (a chip competes with Fortify: 0.03 a seat-round at 4 credits, 0.05 at 6, at its
  floor); Levy is taken only when energy is short (0.04, whatever its rider: the floor band 0.03, Matt's answer 12682).

## Files

- [read.py](read.py): win % and rounds by level, paired. Specs: `experiments/farm/f277-read.json` (new side; the old
  side ran 276's `f276-read.json` at 1d7a601). Read outputs: `out/keep/277/` (kept by conductor).
