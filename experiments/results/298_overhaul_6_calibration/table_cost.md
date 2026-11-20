# 298: table cost re-read at r143 (prompt step 8)

The sim's minutes are a yardstick, not a clock: Matt's ruling holds them wrong until he plays and corrects them.

**Recipe.** 32 judge games (`strategist`) a level at d2 and d4, seeds 36800-36831, two players, r143's release build:
`aic-sim run --trace` per seed plus the same games as an experiment (278's `traces.sh`, seeds and threads changed),
read with `python scripts/bench/breakdown.py DIR --level L` (the fixed script; 278's and 280's copies stay as their
items' records). Before = 280's copy with `--tracks` (cut 2 is the rule) on the same traces. **Rates:** 268's, 15 s a
decision, 3 s a component, 10 s a check, 20 s a lookup or new face, 5 s a modifier (lookups and modifiers modeled
from 268's r125 hand log, 278); in the draft only, 6 s a decision and 0 s a face (298).

## What made the draft 9.4 minutes

At r143 the old script prices it at **9.57** (d2) / 9.12 (d4): decisions 21.1 x 15 s = 5.3 min (both seats summed),
new faces 7.7 x 20 s = 2.6, card moves 26 x 3 s = 1.3, modeled rules 0.4. Two faults:

1. **Seats summed.** Both seats tier, pick and keep their own hands at the same time. The rulebook's action phase is
   also "all players act at once", and was summed the same way.
2. **A first-time reader's rates.** 268's 15 s was timed on action-phase choices across the board, its 20 s on a
   reader who doesn't know the card; Matt's ruling makes drafting players fast and skilled.

**The fix: the model and the rate, both.** Model: in the draft and the action phase each seat's rows stay, and an
"overlap" row takes off the faster seat, so a section costs its shared rows plus the slower seat (the draft's tier
choices, picks and keeps are three steps both finish before the next; the action phase is one step). The draft's
card moves belong to the seat whose hand they enter or leave; each hand passed is one component. Rate, the draft
only: 6 s a decision (two decks, at most 4 known cards, or keep one), and a face first shown in a draft hand is read
inside the choice that shows it. Also: r142's powers 0-2 are named node powers (278 called them "Growth"), and a
tiered power's tiers count one used marker, as printed (cleanup -0.04).

| Draft a round (d2 / d4) | minutes |
|---|---|
| 278's model and rates on r143 | 9.86 / 9.39 (280's copy 9.57 / 9.12; the gap is the hands passed) |
| + seats at once | 5.37 / 5.16 |
| + faces read inside the pick | 4.08 / 3.94 |
| + 6 s a draft decision (**the fix**) | **2.38 / 2.34**, in Matt's 1-3 band |
| the rates alone, seats still summed | 4.10 / 3.98 |

Neither part alone reaches the band. 6 s is an estimate, not a measurement: at 8 s the draft is 2.8 / 2.7, at 10 s 3.1.

## The round and the game

| Section, minutes a round (d2 / d4) | 278 | 280 after | r143, 280's model | r143, fixed |
|---|---|---|---|---|
| Adversary: threats dealt | 2.01 / 2.10 | 2.01 / 2.10 | 1.27 / 1.49 | 1.27 / 1.49 |
| Adversary: lines and arrivals | 3.47 / 3.75 | 2.97 / 3.38 | 2.95 / 3.61 | 2.95 / 3.61 |
| Adversary: march | 1.38 / 2.76 | 1.39 / 2.78 | 0.50 / 0.70 | 0.50 / 0.70 |
| Adversary: compute and projects | 2.21 / 2.97 | 2.34 / 3.09 | 2.03 / 2.87 | 2.03 / 2.87 |
| Draft | 9.42 / 8.59 | 9.42 / 8.59 | 9.57 / 9.12 | **2.38 / 2.34** |
| Action phase | 15.89 / 13.97 | 16.06 / 14.11 | 15.99 / 15.49 | **9.39 / 9.04** |
| Production and hosting | 4.20 / 4.43 | 2.26 / 2.79 | 1.96 / 2.15 | 1.96 / 2.15 |
| Threats executed | 1.91 / 2.18 | 1.95 / 2.20 | 1.61 / 1.96 | 1.61 / 1.96 |
| Everything else | 1.91 / 2.15 | 1.90 / 2.16 | 1.42 / 1.51 | 1.31 / 1.44 |
| **Round** | **42.4 / 42.9** | **40.3 / 41.2** | **37.3 / 38.9** | **23.4 / 25.6** |
| Rounds a game; **game** | 8.00, 339 / 6.36, 272 | 323 / 262 | 8.94, 333 / 8.75, 340 | **209 / 224** |

The action phase's overlap is 6.6 of the 8.3 minutes the fix's overlap takes off at d2 (6.5 of 8.1 at d4). With
the action phase still summed (seats taking turns in practice), the fixed round is **30.0 / 32.1**, a game 269 / 281.
Whether two co-operating players act in parallel is Matt's call at the table; the draft's parallel play is not in
doubt. By kind at d2 (fixed): decisions 10.6, components 7.8, checks 5.1, lookups 4.6 (modeled), faces 1.5,
modifiers 2.3, overlap -8.3. 278's and 280's tables are 268's seeds at r133 and earlier: a section's change since
mixes the build with the seeds. Games are longer (8.9 rounds against 8.0 / 6.4), so a game moved less than a round.

## 293's per-site checks and 295's node-power memory

- **293.** The adversary's site, arrival and botnet checks are 11.9 a round at d2 (280: 12.0) and 14.1 at d4 (280:
  11.3): **+0.0 / +0.5 min a round**. More sites are read but fewer arrivals (3.0 / 4.1 arrival checks against 7.6),
  and the March's troop reads went with r141's icons (march 1.4 / 2.8 to 0.5 / 0.7).
- **295.** 2.3 node-power uses a round at d2 (both seats; 2.1 at d4), 4.5 used markers set and cleared (0.22 min).
  Their rows cost 2.07 min against 280's Growth 1.99 (+0.08), and the board uses a seat weighs at its phase's start
  rose from 1.9-2.0 to 2.8-3.5 a seat (+0.40 min at d2, +0.39 at d4): **about +0.5 min a round summed over the
  seats, about half that with the seats at once**.

Full tables (not kept): `out/298_overhaul_6_calibration/breakdown/new-d{2,4}.md` (fixed), `old-d{2,4}.md` (280's).
