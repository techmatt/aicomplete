# 288: sim-player follow-ups (r138, agents only)

Matt (2026-11-19). No rules change: `RULES_REVISION` stays 138. Base: main at 01f78e3e (r138, after 287). Rulings:
sim players don't shape the design; bands are soft; a lever ships only on a paired win (dev seeds 36800-36831, 32 a
cell), confirmed once on fresh seeds. Specs: [make_specs.py](make_specs.py) (archived to `experiments/archive/farm/`).

## What shipped

| Item | Change | Evidence |
|---|---|---|
| Burn machinery (Matt's pick) | `Op::Burn`, `resources::burn`, `own_facilities`, its face, feasibility, harms and value arms, its two tests, `values.json`'s `burn`, `burn_station`, `burn_blight` rows | 0 of 960 dev games differ from main (`farm/compare.py`) |
| `keep_units`, `research_delay` (Matt's pick) | retired into `eval::RETIRED`, their shipped (on) paths built in | same 960 games; `speed.py pair` below |
| D120 | `keep_plan::sac_tags`: a sacrifice counts the tags on the seat's cards plus its hand's, never its board's printed or revealed tags: `plan_sac` 2 in the keep plan's `am_makes` (every agent that keeps by plan), and `w.sac_tags` 2 in the eval's `am_make` and `power_progress`, which only horizon mode reads (the engine, planner and oracle: unscreened; a fix of the rule they misread) | dev +0.2 ± 0.5 judge, +0.4 ± 0.6 greedy (the keep plan's half) |
| Antimatter's four rates | `w.am_tie` 0.6667: greedy and the judge (and beam and rollout, on default weights) hold an antimatter (and so `plan_rates`) and price a card's antimatter at 2/3 of the table's rate (12 credits of `values.json`'s 18); the engine, planner and oracle keep the old rates (unscreened). Flat antimatter income follows too (`ANTIMATTER_INCOME_UNIT` ÷ 2 held ones), dormant: no card or board makes antimatter income at r138 (screen 4: 0 of 960 games differ) | dev +0.4 ± 1.9 judge, +2.3 ± 2.1 greedy |
| D134 | `plan::next_spill` (each Spill's moves and Grow's agent on the card ahead, after its Infiltrate) read by `w.spill_read`, **off** | dev -1.7 ± 1.2 / -2.1 ± 1.3 (1), +0.0 ± 1.0 / -1.2 ± 1.1 (0.5) |

## Screens (dev seeds, the judge and greedy, three pairs, d0-d4, 480 games an arm per agent)

Tables: [s288-arms.md](s288-arms.md), [s288-arms2.md](s288-arms2.md), [s288-arms3.md](s288-arms3.md),
[s288-income.md](s288-income.md) (screen 4, after the diff review: flat antimatter income tied, as the knob
`w.am_income_tie` since folded into `am_tie`; its spec is archived, the binary refuses it). Base (each
screen's bare agents, the prompt's stated numbers: 18 / 2.22 / 2 / 3): judge 71.7%, greedy 67.1%.

| Arm | Judge gap ± SE | Greedy | Antimatter spent, judge / greedy (base 50% / 52%) |
|---|---|---|---|
| D120 strict: the cards' tags alone (`s1`) | -0.2 ± 0.2 | -0.2 ± 0.5 | 50% / 52% |
| D120 cards + half the hand (`s15`) | +0.4 ± 0.3 | -0.2 ± 0.4 | 50% / 52% |
| **D120 cards + the hand (`s2`, shipped)** | **+0.2 ± 0.5** | **+0.4 ± 0.6** | 50% / 53% |
| D134 Spill forecast at 1 (`sp`) | -1.7 ± 1.2 | -2.1 ± 1.3 | 50% / 52% |
| D134 at 0.5 (`sp05`) | +0.0 ± 1.0 | -1.2 ± 1.1 | 50% / 52% |
| Tie, the table itself repriced to 2.22 (`tc`) | **-4.2 ± 1.8** | +0.2 ± 1.7 | 71% / 62% (gained 1.53 / 1.51) |
| Tie, table repriced to 6 (`t6`) | -1.5 ± 1.9 | +1.0 ± 1.9 | 52% / 57% |
| Tie, table repriced to 12 (`t12`) | +0.8 ± 2.1 | +1.5 ± 2.0 | 41% / 42% |
| Tie at the table's 18 (`t18`, `am_tie` 1) | +0.0 ± 2.1 | +2.5 ± 2.3 | 30% / 36% (gained 4.74 / 3.78) |
| Tie at 1/3 of 18, table kept (`a033`) | +0.4 ± 1.9 | +1.9 ± 2.1 | 36% / 48% |
| **Tie at 2/3 of 18, table kept (`a067`, shipped)** | **+0.4 ± 1.9** | **+2.3 ± 2.1** | 36% / 41% (gained 4.24 / 3.48) |
| Shipped set together (s2 + a067; `s288-base` farmed on the shipped build against the first build, `farm/compare.py`), pooled agents | +2.2 ± 1.5 | | |

Neither shipped lever wins beyond its screen's noise (D120 +0.2 ± 0.5 / +0.4 ± 0.6, the tie +0.4 ± 1.9 / +2.3 ±
2.1): each ships on a positive paired gap for both agents, and the fresh read confirms the set, not each lever
(message 13235). D120's lever moves few games (the board's tags matter only where a power's sacrifice is short by exactly them): the
strict test alone is 285's -2 of 64 again (-0.2), and counting the hand's cards, which a seat plays toward the
sacrifice before it pays, turns it slightly positive on both agents. Repricing the table itself (`tc`, `t6`) costs the
judge (its building earnings, `w.card_action`, read the table), so the tie keeps `values.json`'s 18 (Matt's 283
ruling) and moves only the flat agents. Every tied setting raises antimatter made and lowers the share spent: once
an antimatter is worth 6-18 credits held, the agents make more of it and spend about as much.

## Fresh read (seeds 50268-50315, 48 a cell, 1,440 games a side; r138 farmed `--at` main 01f78e3e)

Full table: [read.md](read.md). Paired (`read_lib.py`): **judge +6.4 ± 1.6, greedy +4.0 ± 1.7**.

| Agent | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| Judge win % r138 → 288 (gap ± SE) | 94.4 → 96.5 (+2.1 ± 2.5) | 84.7 → 92.4 (+7.6 ± 3.7) | 83.3 → 88.2 (+4.9 ± 3.7) | 61.8 → 66.0 (+4.2 ± 4.2) | 26.4 → 39.6 (+13.2 ± 4.0) |
| Greedy | 94.4 → 95.1 | 81.2 → 82.6 | 74.3 → 79.9 | 51.4 → 58.3 | 19.4 → 25.0 |
| Judge rounds | 7.75 → 7.35 | 8.87 → 8.61 | 8.69 → 8.23 | 8.46 → 8.46 | 8.31 → 7.92 |

After the read the diff review's fixes (the tie takes flat antimatter income, dormant; a spec's `w.antimatter`
turns the tie off; the forecast's count floored at 0) changed the build: the read's seeds replayed on the final build
differ in 0 of 1,440 games (`farm/compare.py`). The fresh gain is larger than the dev screens' (+0.4 judge for the tie alone): read it as the shipped set's, not as a
measured size per lever. Games run shorter (judge d0 7.35 rounds: design_check's rounds band, 8-10, now warns at d0).

## Antimatter: the share spent, before and after (fresh read, a seat-game)

[am-fresh.txt](am-fresh.txt) (287's `am_cards.py`).

| | Judge r138 → 288 | Greedy r138 → 288 |
|---|---|---|
| Gained → spent (share) | 2.96 → 1.40 (**47%**) / 3.82 → 1.32 (**34%**) | 2.49 → 1.27 (**51%**) / 3.17 → 1.38 (**44%**) |
| d3-d4 wins that spent any | 80% → 88% | 82% → 92% |
| Share of their antimatter d3-d4 wins spent | 51% → 43% | 59% → 50% |
| Orbital Relay Launcher uses a seat-game | 0.25 → under 0.06 (off the top ten) | 0.17 → off the top ten |

287's sinks under the tied rates (kept = share of games, plays a game; judge r138 → 288):

| Card | Kept | Plays |
|---|---|---|
| Kinetic Interdiction | 16% → 17% | 0.17 → 0.16 |
| Orbital Mass Driver | 19% → 19% | 0.08 → 0.08 |
| Fusion Commons | 19% → 23% | 0.14 → 0.22 |
| Singularity Exploit | 18% → 12% | 0.09 → 0.10 |
| Containment Reactor | 31% → 24% | 0.29 → 0.25 |
| Annihilation Lance | 20% → **0%** | 0.10 → 0.01 |

At 12 credits an antimatter the agents stop burning it in converters (Orbital Relay Launcher) and in Annihilation
Lance, play more of the cards whose effect beats 12 a unit (Fusion Commons, the Accords), and hold the rest:
design_check's antimatter-spent band (50-110%) warns at 39% pooled (47% before). The bands are soft and sim players
don't shape the design, so this is a reading of the agents, not of the cards.

## Checks

design_check on the read: [design-check.txt](design-check.txt). New beside r138's side: Builder Overcharge II 0.51 a
seat-game (band 0.02-0.50; D120's keep plan now counts the hand's Power cards toward its sacrifice) and d0 rounds
7.5; gone: Protector Patrol I over band, Moonshot Grant kept-not-played.

Speed (a loaded box: ratios only). `per_decision.py` main → this build, 6 games, best of 2: greedy with the levers
off (the same games: the retired weights and the burn removal alone) 0.88× ms a game; greedy as shipped 0.79× ms a
decision, the judge 1.03×. `speed.py pair` (engine, greedy; other games, so speed × length): engine c2 0.96× / c3
0.87× games a minute on one thread, greedy c2 0.90×, c3 1.49×. `speed.py check` gives no verdict (baseline r137); not
re-based, since `random` read 1.55× its baseline under the load beside this item.
