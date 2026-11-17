# 252: Item B, part 1: node routes, free then claim, standard actions under the cards

Base: main 8b96eb0, r112. **Rules change: r113** (bumped from the constant, 112 to 113). Matt's rulings 2026-11-16
(locked). Amendment 11503 split the item: this one does rulings 1, 2, 4, 5 and 6, design_check, the docs and a
32-seed direction screen; **253** does the neutral-site cards (ruling 3) and the full paired read (the judge and
greedy at the standing scale, the full oracle at d2-d3, nodes by round and board, standard-action and resource use).

Tables: [screen.md](screen.md) (the paired screen), [design-check.txt](design-check.txt). Scripts: `routes.py` (the
node routes in `content/ais.json`), `dials.py` (the board dials in both configs), `screen.py` (the screen's reader).

## The changes (every number a candidate)

| Ruling | Change | Give / get at the unit anchors |
|---|---|---|
| 1 | Place a node cut (`board.node_credits` 0, the action's off switch) | |
| 1 | **Expand** on every board: a node for the track step's energy alone, once a round | nothing for a node (8): the base route, by ruling |
| 1 | Builder: Buy In (2 energy) middle; **Annex** 1 chip, a node within 2 (`place_node`'s new `range`) | 6 for 8 and a reach no other route has: 1.33 |
| 1 | Protector: Accord (2 energy) middle; **Outpost** 3 energy, a node and an ambassador | 13.5 for 16: 1.19 |
| 1 | Researcher: **Peer Review 2 insight** (was 1 research cube); **Field Survey** 3 insight, a node and a card | 8 for 8 + 2.9; 12 for 13: 1.08 |
| 4 | Liberate leaves the site neutral (reverses r75); the `liberate` op's `own` keeps "yours" for Hostile Buyout alone | |
| 2 | Build a research station 2 chips (1); an owned station makes 1 cube a round (2) | 12 for an owned station (12): 1.00 |
| 5 | Claim / Liberate a station 4 ambassadors (3), a factory 3 (2) | claim station 32 for 12, factory 24 for 19 |
| 5 | Take down a botnet 20 credits (16) | 20 for 15: 1.33 × the median card |
| 5 | Turn a cube into insight: 2 cubes an insight (`board.insight_cubes`, new) | 12 for 4 |
| 6 | design_check: Sell a card exempt from job-no-card (P1) | |

**Why these numbers.** Each route rises in its own board's currency and sits at 1.0-1.5× at the anchors; every route is
once a round and every placement still pays the step's rising energy, which caps energy → node → energy income.
Claim's station and factory prices are the smallest steps that clear job parity (1.25 × the median card: 24.4 and
the factory's over-anchor 1.19); Liberate shares Claim's table (standing ruling), so it moved with it. The botnet
price is the smallest whole step over 1.25 × the median card (18.75). The cube converter's tax is the default
(2:1) of the staging question; it was 3.4 uses a seat-game at r112 and is a fallback now.

**Anchors no longer circular.** The station anchor (6, set from Build a research station's 1 chip) is gone: the row's
get is now an owned station (`station_owned`, values.json's `claim_station`, re-derived to 12 for 1 cube a round).
The botnet anchor is the median card's unit price for the job (15), not the action's 16. New anchors: a freed
station 15 and a freed factory 6 (248's price table, values.json's `cleanse_station` and `cleanse`).

**Agents.** The node routes are board powers, which the judge and greedy already value and take (`place_node` in
value.rs); no heuristic needed a change for them to play every route (the screen: Expand ~5 a seat-game on every
board, the top routes 1.6-4.3). value.rs and the effect ledger price a liberation without the seat's cube unless
`own`; values.json's `claim_station` 20 → 12, `build_station` 8 → 10, `burn_station` -15 → -8 for 1 cube a round.
The fixed policy `never_second_node` now means "Expand only": it rules out the middle and top routes (it was "never
Buy In and the like" beside Place a node). Speed (`speed.py check`, one thread): random 0.98×, greedy 0.91×, engine
0.94×, planner 0.84× the r112 baseline: no verdict across a revision (other games), nothing slower.

## The screen (direction only; amendment 11503)

251's `i251-judges` spec (greedy and the judge, three pairs, d0-d3) on the dev seeds 36800-36831 (spent), against
251's kept r112 read of the same spec and seeds (`251-identity/new-judges.jsonl`, current code at r112), so the pair
differs only by this item. 768 games a side.

| | d0 | d1 | d2 | d3 | all |
|---|---|---|---|---|---|
| judge r112 → r113 | 96.9 → 97.9 | 93.8 → 91.7 | 92.7 → 91.7 | 68.8 → 56.2 | 88.0 → 84.4 (-3.6 ± 2.1) |
| greedy | 93.8 → 88.5 | 90.6 → 84.4 | 79.2 → 83.3 | 62.5 → 52.1 | 81.5 → 77.1 (-4.4 ± 2.7) |

- **d3 drops about 12 points** (judge -12.5 ± 6.0, greedy -10.4 ± 6.4: 2 SE, 32 seeds). Its driver, on this read:
  node loss (the judge's losses ending on a lost node 16 → 29 of 768, deck-out flat 30 → 31) and ruling 4 (owned
  sites from liberating fall: claims of factories 0.76 → 0.09 a game, liberated stations and factories halve). The
  full read (253) confirms or clears it on fresh seeds.
- **Nodes:** more early, fewer late (judge, both seats: round 2 2.5 → 4.7, round 6 1.7 → 1.4, round 8 0.73 → 0.62;
  21.8 a game against 19.9). Placements go on through round 8 (0.62 a game, both seats; per board by round is
  253's read). By route, a seat-game: Expand ~5 on every board, the middle routes 1.6-2.3, the top routes 2.1
  (Annex), 3.5 (Outpost), 3.7 (Field Survey); cards make up the rest (routes about 20.2 of the judge's 21.8 a game, cards about 1.6:
  Expand 46%, middle routes 18%, top routes 28%, cards 7%).
- **Standard actions beyond Dispatch and Liberate:** judge 13.75 → 6.29 a seat-game (248's 14), greedy 13.1 → 5.5.
  Cube to insight 3.6 → 0.1, Place a node 3.7 → 0, Build a station 1.4 → 0.2; Buy energy rose 2.3 → 3.3 (liberated
  cities stay neutral and sell their energy). Dispatch rose 7.9 → 9.5.
- **Resource use (judge, spent of gained):** research cubes spent fall 11.7 → 3.4 a seat-game (54%: the converter
  and Peer Review no longer drink them; the cubes now pile, a 253 question); energy 86% → 91%, chips 57% → 66%,
  antimatter 36% → 43%, ambassadors 73% → 78%, insight 82% → 83%, credits flat.

## design_check (r113, the screen against r112)

No standard-over-anchor, circular-anchor or job-parity flag on any standard action. Left:
- **job-no-card Build a research station:** only the Researcher's Field Station power builds an owned station; a card
  that does is Item C's (253's neutral-station cards leave it neutral, a different outcome).
- **job-no-card Claim a city:** no card claims a city: Item C's.
- **dials-hide-design:** this item's own dials beside its design files, as the rulings ask.
- Usage on the screen: Claim a site 0.04 a seat-game (unused, its band 0.10): claiming after liberating is rare at
  4-5 ambassadors; Bulwark 0.16 a seat-round (Outpost and Expand take the Protector's energy); kept-not-played
  Exascale Training Cluster, Citywide Microgrid, Dyson Swarm Tranche; Scrap and Audit just over their bands; player
  token moves +11%. For 253's full read.

## For 253: what to check first

1. The d3 drop and its driver (node loss up, owned sites down): on fresh seeds, with ruling 4 on and off.
2. Research cubes piling (spent 3.4 of 6.3 gained a seat-game): the converter tax may be too steep once Peer Review
   left cubes; 1 cube an insight with a once-a-round cap is the other shape.
3. Claim after Liberate: 0.05 a seat-game. Free-then-claim works as a rule, but almost no one claims; the price table
   (shared) is the lever, a ruling's shape is Matt's.
4. Late nodes (rounds 6-8) fell slightly: the step's 3 and 5 energy is the cap; check the Researcher's nodes by round.

Candidates I'm least sure of: Outpost at 3 energy (taken 3.5 a seat-game, maybe too cheap for a node and an
ambassador), Field Survey at 3 insight (3.7), claim_station 4, the cube tax.

## Seeds and outputs

Spent seeds re-used: the dev seeds 36800-36831 (the screen; 251's kept r112 side). No fresh seeds taken; the ledger
is unchanged (next free 48780). The screen's output: `out/keep/252-screen/games.jsonl`.
