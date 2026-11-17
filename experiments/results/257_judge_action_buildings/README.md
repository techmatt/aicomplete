# 257: the judge keeps and uses Action buildings

Base: main 88f4486 (256's r117). **No rules change**: the sim players only; `RULES_REVISION` stays 117. Matt's ruling
(2026-11-17): fix the player, not the cards. Files: [screen.md](screen.md) (three dev screens), [read.md](read.md)
(the first fresh read), [read-use.md](read-use.md) (`use257.py`), [read-d34-use.md](read-d34-use.md) and
[screen-d34-use.md](screen-d34-use.md) (the d3-d4 read and its dose screen), [design-check.txt](design-check.txt)
(sink 2 against shipped, first read). Specs: `experiments/archive/farm/s257-screen.json` (runs only at 487ce17: its
`w.card_left` was removed), and in `experiments/farm/` `s257-screen2.json`, `s257-screen3.json`, `f257-read.json`,
`f257-d34.json`, `s257-d34.json` (an arm named `strategist` there carried `w.sink` 2: now `strategist:w.sink=2`).

## Where the gap was: use, not keep or build

**Keeps are bounded by offers.** Each card is one copy in a 205-card deck, offered 0.26-0.55 times a game; the
judge keeps 54-90% of the offers of the seven named buildings (fresh read, shipped judge: Orbital Relay Launcher
0.45 of 0.50, Survey Drone Fleet 0.22 of 0.41, Mesh Repeater Network 0.22 of 0.32), inside the range of other
Action buildings (10-92% on 256's read), and builds most of what it keeps (cards drawn by effects add to builds). 256's "kept 0.2-0.5 a
game" is the offer rate.

**Uses were the gap.** A throwaway log of every decision with a legal `Use` (its score against the chosen action's;
24 games, d3, dev seeds) showed why the judge, which scores the state after each action at its flat weights, left
converters idle: a held antimatter reads `w.antimatter` 2 points, below the chip (`w.chips` 4) or 2 energy (1.6)
that makes one, so Antiproton Foundry was never used (beaten by 2.0-3.3 every time) and Positron Storage Ring 1 in
15 times, and no seat with Orbital Relay Launcher made antimatter for it: its Action was legal 4 seat-rounds in 24
games. Research Reactor (a cube 0.3 → 2 energy) was used 5 of 10 times it was legal. Mesh Repeater Network and Wafer
Fab Line lose their energy to the board's node routes (Expand at the step's energy alone): energy binds (85%).

**Outcome: the judge is unchanged.** The lever below runs the converters (Orbital's uses 0.26 → 1.14 a game) at
+0.6 ± 1.1 overall, but a second fresh read at d3-d4 put it at **-3.8 ± 1.5**, and the cost falls with the dose
(sink 0.5 -0.5 ± 0.9, 1 -2.3 ± 1.3, 2 -3.8 ± 1.5 on those seeds): at d3-d4, fueling and running these converters
costs this player more than they make. `w.sink` stays in the code, off, for 258 to re-screen on stronger cards.

## The lever: `w.sink` (the converters' fuel)

A held antimatter, research cube or chip that a built converter of the seat burns (one branch, one fuel: Orbital,
Research Reactor, Survey Drone Fleet, Containment Reactor, Penning Trap Array, Antiproton Foundry, Parts Exchange,
Field Assay Team) counts `sink` × what a use makes at the judge's own flat weights (a node at `w.node` 4, each
resource at its flat unit) per unit burned, beyond its flat weight, for two uses' fuel a converter, best converter
first; a converter whose use makes no more than the flat weight (Field Assay's damage, Parts Exchange's draws) adds
nothing. A converter used this round counts the fuel it burned as still held, so a use is judged by what it makes
alone. Orbital at sink 2: an antimatter held reads 18.8 points over its flat 2. Default 0 for every agent.

**Two drafts that failed (dev, d3, 24 games):** fuel at the value table's rates (antimatter 18 credits) read above
what the use visibly makes, so Research Reactor sat on its cubes (used 3 of 18 legal seat-rounds, beaten by Pass);
at the eval's units without the burned-fuel credit, Orbital lost to Pass whenever its nodes' track steps were
unaffordable. **Known limit** (diff review): the burned-fuel credit lapses as the next round starts, so the action
that ends a round reads that credit lower; per-round action counts on screen 3 are unchanged (standard actions
4.31 → 4.30 a round, passes 3.43 → 3.44), so it costs nothing measurable.

## Screens (dev seeds 36800-36831, three pairs, paired against the shipped judge)

| arm | levels | gap ± SE (all) | d3 | d4 | Orbital uses a game | Research Reactor |
|---|---|---|---|---|---|---|
| sink 0.5 | d2-d4 | +0.3 ± 0.9 | +2.1 ± 1.5 | -2.1 ± 1.5 | 0.19 → 0.41 | 0.25 → 0.29 |
| sink 1 | d2-d4 | +0.3 ± 1.5 | +2.1 ± 2.1 | -1.0 ± 2.8 | 0.58 | 0.51 |
| **sink 2** | d0-d4 | **+1.2 ± 1.5** | +4.2 ± 3.6 | +0.0 ± 3.9 | 0.19 → 0.80 | 0.27 → 0.75 |
| sink 3 | d0-d4 | -0.8 ± 1.7 | +0.0 ± 4.4 | +2.1 ± 4.2 | 0.85 | 0.83 |
| card_left 0.6 | d2-d4 | -2.1 ± 2.8 | -7.3 ± 5.4 | +4.2 ± 4.9 | | |
| card_left 0.6 + sink 0.5 | d2-d4 | -0.7 ± 2.7 | -4.2 ± 5.3 | +5.2 ± 4.8 | | |
| greedy + sink 1 (against greedy) | d2-d4 | +0.0 ± 1.2 | -1.0 ± 2.3 | -1.0 ± 1.0 | 0.03 → 0.22 | 0.17 → 0.33 |

`w.card_left` (249's `card_action` rounds as 0.6 × the adversary cards left, not 3: a building counted longer early
and less late) read noisy and negative at d3, and was removed. Greedy shares the gap (Orbital 0.03 uses a game) and
the fix lifts its uses at no cost, but greedy is the fast lane's and the farm's reference player: left for Matt.

## The fresh reads (paired against the shipped judge)

**First** (49212-49259, d0-d4, 720 games an arm; [read.md](read.md)), the judge with `w.sink` 2:

| | d0 | d1 | d2 | d3 | d4 | all |
|---|---|---|---|---|---|---|
| shipped judge | 87.5 | 90.3 | 90.3 | 59.0 | 27.8 | 71.0 |
| sink 2 | 93.1 | 92.4 | 89.6 | 57.6 | 25.0 | 71.5 |
| gap ± SE | +5.6 ± 2.6 | +2.1 ± 2.5 | -0.7 ± 2.5 | -1.4 ± 2.6 | -2.8 ± 2.6 | +0.6 ± 1.1 |

**Second** (49260-49355, d3-d4 only, 96 seeds a cell, 576 games an arm; `f257-d34.json`; uses:
[read-d34-use.md](read-d34-use.md)), to tighten the first read's d3-d4: shipped 58.7 / 33.0, sink 2 55.2 / 28.8,
**-3.8 ± 1.5** pooled (d3 -3.5, d4 -4.2; deck_empty losses 80 → 96, rounds 8.29 → 8.43: the converters slow the
purge). **Dose screen on those seeds** (`s257-d34.json`; [screen-d34-use.md](screen-d34-use.md); spent by the read,
so a screen, not confirmation): sink 0.5 -0.5 ± 0.9 (d3 +0.3, d4 -1.4; Orbital's uses 0.20 → 0.43), sink 1 -2.3 ± 1.3
(0.73). The shipped judge's arm replays the second read game for game (576 of 576).

**Ship rule:** no worse beyond noise, **not met** at d3-d4 (-3.8 ± 1.5, 2.5 SE); better where 256 lost, not met. The
judge stays as shipped. (An ask on the first read alone, unanswered, had defaulted to shipping; the second read
reversed it.) The shipped judge reads d3 59.0 and 58.7 on these seeds, inside its band: 256's 47.9 was on its own
seeds. **Bands (the judge as shipped, soft; first read):** d0 ≥ 90 missed by 2.5 (87.5, pr 75: 12 deck_empty
losses), d1 80-95 met (90.3), d2 75-90 just above (90.3), d3 55-70 met (59.0), d4 unbanded (27.8).

## Use against 256 (the judge, a game; 256: its fresh read 49020-49067; 257: the first fresh read)

| building | 256 kept / built / uses | 257 shipped | **257 sink 2** | sink 2: uses a round stood |
|---|---|---|---|---|
| Mesh Repeater Network | 0.20 / 0.51 / 0.12 | 0.22 / 0.57 / 0.28 | 0.23 / 0.59 / 0.28 | 0.09 |
| Research Reactor | 0.26 / 0.59 / 0.23 | 0.17 / 0.60 / 0.23 | 0.18 / 0.64 / **0.48** | 0.17 |
| Survey Drone Fleet | 0.33 / 0.23 / 0.05 | 0.22 / 0.13 / 0.02 | 0.23 / 0.19 / 0.05 | 0.06 |
| Orbital Relay Launcher | 0.52 / 0.35 / 0.26 | 0.45 / 0.32 / 0.26 | 0.44 / 0.37 / **1.14** | 0.51 |
| Positron Storage Ring | 0.27 / 0.22 / 0.32 | 0.34 / 0.28 / 0.43 | 0.35 / 0.27 / 0.47 | 0.46 |
| Wafer Fab Line | 0.17 / 0.51 / 0.89 | 0.17 / 0.50 / 0.62 | 0.18 / 0.50 / 0.61 | 0.29 |
| Antiproton Foundry | 0.44 / 0.40 / 0.06 | 0.32 / 0.30 / 0.07 | 0.31 / 0.30 / 0.14 | 0.09 |

**Spent of gained (a seat-game, shipped → sink 2):** credits 103%, insight 83%, energy 85%, ambassadors 75% bind; chips 61% →
62%; **antimatter 46% → 56%** (1.4 → 2.0 spent; design_check's "antimatter unused" warning clears), **research cubes
60% → 60%** (3.5 → 3.7 spent). Cubes still don't bind: the cube converters are few and late (below).

## Cards weak for anyone (for 258)

The converters as a group are the first finding: run more, they cost the judge d3-d4 wins by dose (above), so
what they make (nodes, energy, antimatter) doesn't repay their fuel and the turns around it. Action buildings used
under ~0.12 times a round they stood, or kept rarely, with the judge at sink 2
([read-use.md](read-use.md); every Action building's row: `use257.py --all`):

- **Survey Drone Fleet** (0.06 a round stood): needs 2 Science, built late (round 6.8), and its cube competes with
  the board's converters; late in a game the owner's node track is often spent (seats ending on 17-51 energy with
  nothing to place), so its 2 nodes have nowhere to go.
- **Mesh Repeater Network** (0.09): a second node a round for 1 energy plus the step, beside Expand's node at the step
  alone; energy binds (85% spent), so the board's routes take it first.
- **Antiproton Foundry** (0.09): a chip (4 points, the scarcer currency) for an antimatter, which has few uses unless an
  antimatter converter stands beside it; used twice as often with Orbital's pull, still rarely.
- **Research Reactor** (0.17): cube → 2 energy now runs, but cubes are few (60% spent, 3.7 a seat-game).
- **The jammers** Sentinel Mesh (0.05), Market Watchdog (0.07), Civic Watch (0.08): each jams one tag's threat; a
  target exists only when that tag's threat is up.
- **Field Assay Team** (0.07, a cube for 2 damage on agents), **Parts Exchange** (0.08, a chip for 2 draws), **Bug
  Bounty Board** (0.09, 4 credits for a project drain and a draw), **Treaty Isotope Bank** (0.10, 8 credits for an
  antimatter): weak trades at the judge's weights.
- **Kept rarely:** Synthetic Data Forge (0.03 a game: needs 2 Science, a token pile before any insight), Microgrid Lab
  Compact (0.07: 18 credits), Bug Bounty Board (0.13).

## Speed

`speed.py specs` on one binary (228e919), shipped judge against sink 2, c2, the box shared (ratios only): 0.93× the
time on one thread (2 games), 1.10× on 8 (24 games). The shipped judge doesn't reach the term (`w.sink` 0).

## For new players

No rule and no simulated player changed. What 257 learned: the converter buildings (Orbital Relay Launcher, Research
Reactor, Survey Drone Fleet, Antiproton Foundry) pay too little for their fuel to be worth running at the harder
levels; item 258 makes them stronger.
