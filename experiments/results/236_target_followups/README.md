# 236: target follow-ups (r107)

Matt's rulings of 2026-11-14 (locked) carried from cards (234) to board powers, standard actions and move cards.
`RULES_REVISION` bumped from the constant, 106 to 107. Base: main e192c96 (235's bug hunt, still r106). Message
10585 (Matt) confirmed the staging default: a move card's supply stays a source; board ambassadors come only from on
or next to the target; Dispatch keeps its source and prints its target.

Files: [finds.py](finds.py) (uses a seat-game of each changed power and standard action, new against base; what
Remove agents and Fortify's hardening find beside what reach held) with [finds.txt](finds.txt) (fresh read),
[fresh.txt](fresh.txt) (`ladder.py --measures`), [design_check.txt](design_check.txt). Specs: `experiments/farm/s236a.json`
(dev-seed screen), `s236b.json` (target rule on/off smoke), `f236.json` (fresh read).

## What shipped

- **Powers** (`content/ais.json`): 7 print a target and act on it. Fortify "Target: Troops. Remove 1 Troop on it;
  harden a facility on or next to it" (was a troop in reach and a facility in reach, two places); Bulwark "Target: a
  facility. Harden it"; Debrief "Target: Agents. Draw 1 card and remove 1 Agent on it"; Patrol "Target: Agents or
  Troops. Push up to 3 from it"; Interdict "Target: Troops. Remove up to 2 on it"; Reclaim "Target: a corrupted
  research station or factory. Turn it neutral"; Field Station "Target: an open hex. Build a research station on
  it". Two new target conditions: `corrupted_facility`, `station_site`. Disrupt keeps its one country (206); node
  powers (placement next to your nodes), the burns (your own piece anywhere), Grant and Audit (off the map) take none.
- **Standard actions** (`board/standard.rs`, `std_target`): every one that touches the map prints its target first,
  9 offered (Station, Remove agents, Lift, Dispatch, Claim, Liberate, Botnet, Buy energy, Remove a troop; Factory is
  off at `factory_build_chips` 0). Remove agents "Target: Agents. Remove up to 3 on or next to it" (the first on
  it; was any hexes in reach): the one that chose across reach. Remove a troop and Build a factory run a `target` op.
- **Move cards** (Liaison Corps, Transmission Corridor): board ambassadors come from on or next to the target; the
  supply stays a source (message 10585). Faces: "Move up to 2 Ambassadors from your supply or next to it onto it".
- **Count:** 16 powers and standard actions print a target (7 powers, 9 offered standard actions; 17 with the off
  Factory), beside 61 cards.
- **Content check:** `check_targets` became `check_target_lists`; `check_power_targets` runs it on every power
  (Disrupt's no-reach `remove_troops` is the one allowance) and refuses a targeted power whose text doesn't open
  "Target: <condition>.". `tests/targets.rs` `powers_and_standard_actions_name_one_target` runs the same walk over
  the standard actions' ops and their printed words.
- **Agents:** `target_ops` (`agents/moves.rs`) answers a power's target from its ops, Remove agents' as "up to
  `remove_count` agents on or next to it", and Remove a troop's from `std_ops`.

## Pricing (anchors; finds on the fresh read, the new side's tallies against what reach held on the same states)

A power or action whose choices were already one hex in reach (Bulwark, Debrief, Patrol, Interdict, Reclaim, Field
Station, every standard action but Remove agents) finds the same and keeps its price, as 234 ruled.

| item | price | finds a use (target / reach) | anchor check (pay; get; ratio) |
|---|---|---|---|
| Fortify | 2 chips, kept | troop 1 / 1; hardening 0.61 / 0.89 | 12; 5 + 0.61 x 1.5 (values.json, no anchor) = 5.9 (was 6.3); 0.49 (was 0.53): -0.4, under a chip's step (6) |
| Remove agents | 2 ambassadors for 3, kept | 2.26 / 2.50 agents (0.91) | 16; nominal 15 (0.94, under the hedge's 1.0); measured 11.3 (reach would have held 12.5): 0.71; -1.2, under an ambassador (8) |
| Liaison Corps | 12, kept | plays 0.89 a game (base 0.82); board-ambassador walks 0.168 (0.171) | the supply feeds it (Gain 3 first): no find lost |
| Transmission Corridor | 10, kept | out of the deck | as Liaison Corps |

No power or action changed price: each loss is under its cost's smallest step. **Standard actions stay hedges:**
Remove agents 0.94 nominal, 0.71 measured; Remove a troop 0.50.

## Reads

- **Dev screen s236a** (36800-36831, 768 games a side, paired against e192c96): greedy -5.2 / +3.1 / +1.0 / +7.3
  (d0-d3, SE 3.4-4.6), strategist -1.0 / +5.2 / +4.2 / -2.1: noise. Finds as below, within 0.05.
- **Smoke s236b** (36800-36815, d2, 48 games a cell, the target rule, now answering powers and standard actions, on
  / off): engine 60.4 / 54.2, strategist 81.2 / 79.2 (noise at 48 games); targets 23-26 a game.
- **Fresh read f236** (47328-47375, taken; 48 seeds a cell, 2 players, B+P / B+R / P+R, d0-d3; 144 games a row),
  against main r106 built from `git archive` (old side natively, 5 shards; new side farmed):

| agent | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| greedy r107 | 86.1 (-1.4 ±2.8) | 79.2 (-2.1 ±3.0) | 79.9 (+4.2 ±2.8) | 54.9 (-0.7 ±3.3) |
| greedy r106 | 87.5 | 81.2 | 75.7 | 55.6 |
| strategist r107 | 87.5 (-2.1 ±2.7) | 78.5 (+2.1 ±3.0) | 79.9 (-0.7 ±2.9) | 54.9 (+6.2 ±3.4) |
| strategist r106 | 89.6 | 76.4 | 80.6 | 48.6 |

Levels pooled: greedy 0.0, strategist +1.4 (SE about 1.5): the change plays the same within noise.

**Replay** (CLAUDE.md, *Reads*): the slow lane then found Patrol paid for nothing (a target with Agents or Troops
but nowhere to push them), fixed by offering such a target only where a piece can move (`effects::aim`, cards'
push, gather and firewall targets too). The fresh read's seeds replayed on the final build (da8a938) against the
frozen one (9b7df71), `farm/compare.py`: 683 of 1,152 games differ (most in options only), win 75.3 against 75.1,
paired +0.3 ±0.2 ([replay.txt](replay.txt)). Against r106 the final build reads greedy 86.1 / 79.2 / 79.9 / 55.6,
strategist 88.9 / 78.5 / 79.9 / 54.9 (d0-d3). Bands (soft,
reported): d2 sits over 50-65 on both sides (75.7-80.6 at r106 on these seeds), d3 over 35-50 on both but strategist
r106; d0-d1 in.

**Changed powers against their bands** (`design_check.json`, a seat-game but Bulwark's a seat-round; fresh read,
r107 / r106 on the same seeds):

| power | band | r107 | r106 | verdict |
|---|---|---|---|---|
| Debrief | 0.25-2.75 | 3.11 | 3.14 | over on both (234 read 3.01) |
| Audit (unchanged) | 0.25-2.00 | 1.88 | 1.88 | in (234 read 2.05 over) |
| Patrol | 0.25-3.00 | 3.02 | 2.94 | just over at r107, in at r106 (+0.08, noise) |
| Fortify | 0.25-2.50 | 0.19 | 0.21 | under on both (234 read 0.22) |
| Bulwark | 0.40-0.90 a seat-round | 0.55 | 0.56 | in |
| Interdict | 0.25-2.50 | 0.70 | 0.70 | in |
| Reclaim | 0.02-0.50 | 0.10 | 0.09 | in |
| Field Station | 0.25-2.00 | 0.65 | 0.67 | in |
| Disrupt (unchanged) | 0.25-2.50 | 0.90 | 0.93 | in |
| Remove agents (hedge) | 0.05-0.75 | 0.41 | 0.63 | in on both; its use fell by a third |

Fortify is under its band at both revisions: 2 chips buys about 0.5 of its price at the anchors. A 1-chip Fortify
(about 0.97) is the lever; not taken (the ruling's prices keep), flagged for Matt.

## Smokes and checks

- `loop_smoke.py --run`: no timeouts, no card at the play flag. `play_read.py` on f236: kept-and-never-played 0 for
  both agents. `npm run cards:fit`: the move cards' faces fit (no change in the small-box list).
- Speed (`bench/per_decision.py`, r106 -> r107, c2 d2, 16 games, one thread): greedy 0.94x a decision, strategist
  0.94x (options a decision 0.94x / 0.93x: Remove agents' later picks are fewer). `speed.py check` gives no verdict
  across a revision.

## For a new player

Every map effect now works the same way. A card, a board power or a standard action that touches the map names
**one target**, a hex in your reach, and its words open with it: "Target: Troops. Remove up to 2 on it." It acts on
the target, or on or next to it where it says so. Remove agents takes up to 3 agents on or next to its target, no
longer from anywhere in your reach. Fortify removes a troop on its target and hardens a facility on or next to it.
When a card moves your ambassadors, they come from your supply or from on or next to the target. A Dispatch still
takes one from anywhere. One exception stays: the Researcher's Disrupt removes troops anywhere in one country.
