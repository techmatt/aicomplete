# 248: Liberate with antimatter reverted; how much the game leans on standard actions

Base: main fe39ad6, r111 (247). **Rules change: r112** (bumped from the constant, 111 to 112). Matt's direction
(2026-11-16, locked): cards are the engine and beat standard actions; a standard action is a rare fallback, as a
standard project is in Terraforming Mars. This item measures where the game stands; the design changes come later.

Tables: [probe.md](probe.md) (arms a, b, bd, c: wins, rounds, endings, standard-action uses, liberations, resource
use; arm a's bands), [jobs.md](jobs.md) (each action off alone, and back alone into b), [oracle.md](oracle.md) (the
oracle spot-check), [nodeless.txt](nodeless.txt) (who runs out of nodes), [price-table.md](price-table.md) (each
standard action against the cards for its job), [design-check.txt](design-check.txt). Specs: `mkspec.py` writes
`experiments/farm/p248.json`, `p248j.json`, `experiments/archive/farm/smoke248.json`, `p248-oracle.json`. Readers:
`probe.py`, `jobs.py`, `oracle.py`, `nodeless.py`, `price_cards.py`.

## The revert (r112)

Gone: the standard action, `board.liberate_antimatter`, `Price.antimatter`, `site_supply_cost_less` and `pay_site`'s
`less`, and the strategist's `w.purge_am` term (its code is r110's again; a diff review found the revert exact).
Kept from 247: Buy In and Accord on 2 energy, the energy-paid building Actions, Tidal Grid's 2 chips → 1 antimatter,
`board.node_energy` (0). Docs: economy.md (the paragraph is now one line: no standard action spends antimatter),
the rulebook's Liberate row and Claim-and-Liberate list, rules README.

**The probe switch:** `board.std_off`, a list of places in `STANDARD` from 1 (0: none; the shipped game `[0]`); Sell
goes with `board.sell_credits=0`. The agents' Liberate terms (eval's purge, freeing and station-freeing terms, the
move rule's `seat_short`) now read whether Liberate is offered, so in arm c the judge doesn't save ambassadors for an
action it can't take. The 16-seed smoke: arms a, b and bd play identically with and without the gate (576 of 576
games), arm c 23.4% → 20.3% (noise; the hoarding was not what sank c).

## Bands on r112 (fresh 48376-48567, 2,304 games, the strategist)

| | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| r112 | 96.9 ± 0.7 | 95.5 ± 0.9 | 90.5 ± 1.2 | 72.0 ± 1.9 |
| band (soft) | 90+ met | 80-95 unmet (over 0.5) | 75-90 unmet (over 0.5) | 55-70 unmet (over 2.0) |
| r111 (247's fresh read, other seeds) | 96.7 | 97.0 | 95.7 | 83.0 |
| r110 (the same read) | 94.1 | 87.5 | 79.3 | 54.2 |

Rounds d0-d3: 8.98 / 8.50 / 8.62 / 8.37 (won games 8.84 / 8.21 / 8.16 / 7.88). Pairs (first half): bp 91.1, br 85.7,
pr 87.0. **Antimatter spent: 43%** (3.12 gained, 1.33 spent a seat-game; 247 read 36% at r110 and 73% at r111): the
majors' cards are its only spend again. The gap to r110 is 247's kept half (Buy In and Accord on energy).

## The probe (48376-48471, 1,152 games an arm, paired)

| arm | d0 | d1 | d2 | d3 | all (gap ± SE) | node-loss ends |
|---|---|---|---|---|---|---|
| a: baseline | 97.2 | 94.8 | 87.8 | 71.9 | 87.9 | 3.7% |
| b: all off but Liberate, Dispatch | 78.1 | 66.0 | 57.6 | 40.6 | 60.6 (-27.3 ± 1.5) | 33.6% |
| bd: b, Dispatch off too | 76.7 | 66.3 | 56.2 | 41.7 | 60.2 (-27.7 ± 1.6) | 35.2% |
| c: all off, Liberate too (Dispatch kept) | 42.4 | 23.3 | 13.5 | 3.1 | 20.6 (-67.4 ± 1.4) | 43.0% |

Rounds (all / won), d3: a 8.37 / 7.88, b 6.55 / 7.74, c 7.16 / 8.89; d0: c 10.58 / 10.63 (c's games run long and a
third end on an empty adversary deck). **Dispatch kept in b and c:** it is the board's own per-round allowance (printed
Dispatches, uncovered Dispatch spaces), not a bought action; bd reads it off: it changes nothing measurable.

**Uses a game in arm a (two seats):** Dispatch 15.7, Place a node 7.8, Turn a cube into insight 7.1, Liberate 6.2,
Buy energy 4.5, Build a station 2.9, Take down a botnet 1.7, Remove a troop 0.9, Gain an ambassador 0.8, Remove
agents 0.7, Claim 0.6, Lift Fallout 0.6, Sell 0.4, Make antimatter 0.3, Envoy 0.1. Dispatch and Liberate aside that
is 28 a game, 14 a seat-game, spread through the game (5.2 in round 1, about 3 a round to round 7), only 17% in the
last two rounds: the opposite of a rare end-game fallback. By board: the Researcher places 6.8 nodes a seat-game with
Place a node; the Builder and Protector turn 6.3 and 4.3 cubes into insight; the Builder builds 2.1 stations.
In b: Dispatch 11.8, Liberate 4.9; in c: Dispatch 24.1.

**Resource use (per seat-game, arm a):** standard actions take 19% of credit spend (60 of 321), 12% of energy, 26% of
chips (stations), 39% of research cubes (insight, Make antimatter, Lift) and 95% of ambassadors (Dispatch, Liberate,
Remove agents, Claim). In b credits gained fall 303 → 209 and spent 321 → 226 (fewer nodes, less track income; cards
played a game 57.5 → 46.1), energy spent 36 → 23.

## Which jobs the game leans on (job screen, spent 48376-48423, 576 games an arm, paired)

| job | off alone from a | back alone into b | read |
|---|---|---|---|
| Place a node | **-26.2 ± 2.2** | **+28.1 ± 2.2** (b+node 85.9 vs a 87.7) | carries the Researcher |
| Build a research station | **-7.5 ± 1.7** | **+8.0 ± 1.6** | real, second |
| Turn a cube into insight, Buy energy, Take down a botnet, Claim | +0.5, +1.4, +1.7, -0.5 | -0.9, -1.7, -2.4, -1.4 | nothing measurable |
| Remove agents, Remove a troop, Lift Fallout, Gain an ambassador, Make antimatter, Sell, Dispatch | -1.7 to +1.9 (SE 0.5-1.5) | | nothing measurable |

b's whole loss is Place a node (with it back, b reads 85.9, within 2 of a) and a station's worth. **Who runs out:** in b
the losses end on node loss (a seat with no nodes loses for everyone), 332 of 387 by the Researcher, at round 4.5 on
average; -node alone: 172 of 182 Researcher, round 4.0. Its own node power, Peer Review, costs a research cube, and it
uses it no more without Place a node (1.94 → 1.80 a game; Accord 3.79 → 4.14, Buy In 4.03 → 3.97).

## Oracle spot-check (48376-48379 a pair, d0-d3: 12 seeds a level, 48 an arm)

| arm | the judge | the oracle | oracle wins where the judge lost |
|---|---|---|---|
| a | 46/48 | 48/48 | 2 |
| b | 32/48 | 47/48 | 15 |
| c | 8/48 | 41/48 | 33 |

**Winnable without standard actions; the judge can't find it.** The oracle (full information, `tries=3`) wins c's
seeds with card liberations alone (4.3 cities freed a won game, all by cards; the judge's 8 wins free 2.8) in 11.2
rounds. It never loses a seed the judge wins. Its 7 losses in c and 1 in b are all early node loss (rounds 3-7) in the
Researcher's pairs: the node job, not the win route. With full information this is a floor on skill, not a fair
player; the gap says the strategist reads standard actions far better than the cards that replace them (246: its flat
eval counts card income and Actions as nothing).

## Price check (at the unit anchors; [price-table.md](price-table.md))

Under par and far dearer than the cards for their job (1.5-10× the median card's unit price): Remove agents, Remove a
troop, Gain an ambassador, Buy energy, Turn a cube into insight, Envoy, Make antimatter, Lift Fallout (at par, 1.00,
but 6× the cards). **Flagged:**
- **Liberate a city** (exempt): 40 for 45 (1.13), at the median liberate card (38.4); only four cards beat it.
- **Liberate a station or factory**: 1.17 and 1.69, and it makes the site yours with Sway; the cards that free
  facilities only make them neutral. The exemption is argued for the city; design_check prices only the city.
- **Take down a botnet**: 1.00, within 7% of the three cards (all tier 2, two on antimatter, tag needs): in practice
  the route. Its anchor (16) was set from this price.
- **Build a research station**: 1.00 at the station anchor (6, set from the price), 3.3 at values.json's owned station
  (20); **no card builds a station** (only the Researcher's Field Station).
- **Claim a factory**: 1.19, over par (still 4.6× Prefab Plant Kit). design_check has no Claim rows.
- **Place a node**: 1.33 if the track step is left out (as ai-powers.md prices Buy In and Accord), 0.76 with one step
  (design_check's row); 5× the median of the four node cards, 1.3-1.9× the node powers.
- **Gaps, no card does it:** Claim a city, Build a station, Sell a card.

## What the results say, job by job (findings and options, not designs)

1. **Node placement is the job the game cannot do without today.** It is not a price problem (6 credits a node is
   dear against cards) but a supply one: four node cards in the deck, and the Researcher's own route (Peer Review, a
   cube) doesn't carry it; the Researcher places 6.8 nodes a seat-game with the standard action. Taxed or removed, the
   Researcher runs out of nodes by round 4. Options: a Researcher node power that doesn't wait on cubes; more and
   bigger node cards (a Science building that places a node each round, a tier-1 node card in each tag); or keep Place
   a node as the board's fallback but make cards beat it so it stays rare.
2. **Station building is real (7-8 points) and has no card.** Options: station cards (a tier-1 "build a research
   station in reach" with a rider; a Science engine paying chips for stations), or price the action at the owned
   station's worth.
3. **Covered, or not needed by the judge:** cubes to insight (7 a game, the Builder's and Protector's cube sink),
   Buy energy (4.5), botnets (1.6), Claim, Remove agents, Remove a troop, Lift Fallout, Gain an ambassador, Make
   antimatter, Sell, Dispatch: each off alone costs nothing measurable. Insight from cubes is heavy use with no win
   value at the margin: a tax or removal would only move a cube sink; a card or converter for late cubes would take
   it. Botnets: the price check says the action is the practical route (the three cards are tier 2 and gated).
4. **Liberate:** the oracle wins without it (41/48) by card liberations; the judge falls to 21%. The cards cover the
   job in principle; at the anchors the city action sits at the cards' median, so "cards beat it" isn't true today.
   Options: more and cheaper liberate cards (or bigger ones: two cities), or price the action above them (a 6th
   ambassador) once the judge reads card liberations (its purge term counts only ambassadors toward Liberate bills).
5. **The judge is the bottleneck for any "cards replace standard actions" design:** it spends 60 credits a seat-game
   on standard actions and, without them, plays fewer cards (57.5 → 46.1 a game, on less income) rather than more, where the oracle wins c's seeds. Any change moving
   these jobs onto cards needs the strategist's card terms (income, Actions, card liberations) first, or the bands
   will read the agent, not the game.

## Seeds and outputs

Fresh, taken: 48376-48567 (ledger next free 48568): the probe 48376-48471 (p248), arm a's second half 48472-48567
(read.json). The job screen and the oracle re-used the probe's spent seeds (paired diagnostics, no confirmation).
Kept (`out/keep/`): `248-probe`, `248-read-r112`, `248-jobs`, `248-oracle`. The job screen ran natively after
`farm-seeds` lost its broker (0 of 24 shards in). Speed: random, greedy, engine and planner 0.94-1.00× r111's
baseline per game (a removal); the baseline is re-based on r112.

**Warnings (design_check on the fresh half, against 247's r111 read):** Dyson Swarm Tranche kept not played (0.05);
Liberate used in 96% of seat-games (band to 95%); the Tidal Grid chip converter unused (0.00: it makes antimatter,
which has no general use again); antimatter spent 43% (band 50-110%). The last two are the revert's direct effect.
