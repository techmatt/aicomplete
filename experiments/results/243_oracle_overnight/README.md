# 243: oracle overnight, restarts, a wide oracle read and what it says about the cards

Base: main e12bd4c, r109. No rules change: `RULES_REVISION` stays 109, greedy and the strategist play as before
(the golden game is unchanged). Tables: [screens.txt](screens.txt) (profile, screens, gate),
[gap-table.md](gap-table.md) (the read by pair and level), [otry-table.txt](otry-table.txt) (today's oracle against
the new one on the read's games), [cards-oracle-strategist.txt](cards-oracle-strategist.txt) (192 seeds),
[cards-with-engine.txt](cards-with-engine.txt) (96 seeds, the engine beside them), [proofs-table.txt](proofs-table.txt).
Readers: `gapsum.py`, `otry.py`, `cardcmp.py`, `proofs.py` (the read's trace lines came from a build with an
`OTRY` print per try, not committed: [rd.sh](rd.sh)). Every timing here ran on a loaded box: compare ratios only.

## The search

**Profile** (d3, `--features profile`): lines are 99.6% of the oracle's time; inside them the heuristic 52%,
replaying options 29%, guessing hidden cards 1.6%. So the oracle's own bookkeeping has nothing to cut: time is
lines, and a seed costs what it takes to find its first winning line. Traces: `margin` 40 lets 1-6 alternatives
through a decision, so the round's 192 (plus 64 reseeds) run out over the first 30-45 decisions of rounds 1-2 on a
hard seed; wins are found in rounds 1-5 (reseeds at a round's start or an alternative in round 1). Easy seeds win
on the first lines (d0 1.4 s a game), hard ones spend 256 lines a round.

**One oracle's result on a hard seed swings with its lines' seeds.** Screens on dev seeds 36800-36807 looked large
(alternatives on their own policy seeds: d3 at half the time; with `truth`: d4 24/24 against 22/24), and the paired
gate (d4, 36808-36823, 48 games an arm, side by side) did not hold them: today's 42/48, `truth=1` 44/48 at 1.28x the
time, `truth=1` with own-seed alternatives 41/48. `choices=1` and `horizon=99` lost games at d3. Each arm moved
which seeds won more than how many.

**Shipped: restarts (`tries=3`).** At its first decision the oracle plays whole games on copies of the table, each a
fresh oracle; the first that wins is played, else the first. Try 1 is the oracle as specified, so every game it
wins is played move for move (the 9 such games the gate reran matched action for action; a slow-lane test holds
it); tries 2-3 move every line's policy seed (`salt`) and
switch `truth` on try 2. Wins are a superset of today's by construction; the cost is the extra tries on a seed try 1
loses. Also added: `truth` (lines score the true table, not a guess) and `prove` (a card proof, below). Dropped:
the own-seed alternatives (`salts`).

| Paired, same games | n | today's oracle | new (`tries=3`) | games a thread-hour, today / new |
|---|---|---|---|---|
| Gate, d4, dev 36808-36823 | 48 | 42 (87.5%) | 46 (95.8%) | 66 / 56 |
| Fresh read d0-d2, 47488-47679 | 1728 | 1728 | 1728 | d0 2487, d1 1443, d2 784, both |
| Fresh read d3 | 576 | 565 (98.1%) | 576 (100%) | 275 / 238 |
| Fresh d4, 47680-47703 | 72 | 58 (80.6%) | 69 (95.8%) | 43 / 24 |

Of 25 games today's oracle lost in the read, try 2 won 13 and try 3 won 9; 3 stayed lost (d4). Faster was not
reached: no lever found cuts lines without losing wins, and the restarts cost 1.15x at d3 and 1.8x at d4.

## The read: oracle against strategist, same seeds (192 fresh seeds a cell, d0-d3; 24 at d4)

| Level | oracle | strategist | gap (pts) | SE | by pair (bp / br / pr gap) |
|---|---|---|---|---|---|
| d0 | 100.0 | 90.3 | +9.7 | 1.2 | +9.9 / +7.8 / +11.5 |
| d1 | 100.0 | 83.9 | +16.1 | 1.5 | +15.6 / +15.1 / +17.7 |
| d2 | 100.0 | 77.1 | +22.9 | 1.8 | +24.5 / +19.3 / +25.0 |
| d3 | 100.0 | 56.4 | +43.6 | 2.1 | +41.7 / +44.3 / +44.8 |
| d4 (24 seeds) | 95.8 | 27.8 | +68.1 | 5.5 | +62.5 / +70.8 / +70.8 |

The gap grows at every level, most from d2 to d3 (+23 to +44): every seed is winnable with full information
through d3 (2304 of 2304), so the gap is all the strategist's shortfall. The strategist never won a
seed the oracle lost. The engine (`engine:strategy=1`, 96 of the seeds) won 63%, under the strategist's 78%.

## Cards

Per card over the read (192 seeds, both agents): drafted keeps and plays a game, the share of games played in and
of wins. The oracle plays more of everything (longer games), so a card's lift is its play-rate ratio over the median
card's (1.14-1.21). The oracle sees the deck: its card use is evidence, not a verdict.

**The oracle's keeps are its keep plan's.** Its search never chooses a keep: the keep plan wrapped around it keeps
by the engine's value table on the real table and in every line. So its keep rates measure the table, not search.

**Cards the oracle uses and the strategist ignores** (lift 2+): energy and Power cards. Dyson Swarm Tranche (strategist
3.1% of games, oracle 26.2%), Continental Supergrid (3.4 / 22.1), Power to the People (10.8 / 44.4), Tidal Barrage
(4.7 / 15.2), Moonshot Grant, Arcology Foundry, Positron Storage Ring, Capacitor Discharge, Beamed Power
Constellation, Orbital Mass Driver and about ten more ([cards-oracle-strategist.txt](cards-oracle-strategist.txt)).
**The engine with no search plays them nearly as often** (Dyson 24.1%, Supergrid 14.0%, Power to the People 38.3%):
the difference is the strategist's flat terms against the value table's worth of energy income, not search. Search
itself lifts any card's play at most 1.6x over the engine (Pumped Hydro Storage, Moonshot Grant, Annihilation Lance).
A weakness in the judge's valuation of energy cards, then; but the engine that plays them wins less (63% to 78%), so
playing them is not by itself what wins.

**Cards nobody uses, oracle included:** Smart Grid Backbone (offered 0.6-0.75 a game, played in 0.1% / 2.4% / 2.3%
of strategist / engine / oracle games: kept almost never). Played in under 10% of games by both the strategist and
the oracle: Field Grant, Baseload Contract, Multiyear Defense Budget, Annihilation Lance, Agency Directorate, Deep
State Ledger.

**Card proofs** (`prove`: a line wins only if it plays the card, and the card's draft, keep and play are taken first,
on the real table too; 8 d3 games each the oracle won in the read with the card offered and not played):

| Card | proven | note |
|---|---|---|
| Smart Grid Backbone, Deep State Ledger, Dyson Swarm Tranche, Continental Supergrid, Tidal Barrage, Baseload Contract, Annihilation Lance, Agency Directorate | 8/8 each | winnable with the card in play; cheap (2-34 s a game) |
| Multiyear Defense Budget | 7/8 | |
| Field Grant | 1/8 | won all 8 without it: it adds cubes to an open megaproject, and both complete early, so it sits dead in hand |

A proof shows a card does not cost the win, not that it helps. Field Grant is the one weak card by its rule;
Smart Grid Backbone is a keep-plan blind spot (never kept, fine when played).

**At d4** (fresh add-on games the single oracle won on its own, 3-5 a card): Baseload Contract 3/4, Multiyear
Defense Budget 3/4, Dyson Swarm Tranche 3/5, Smart Grid Backbone 2/3, Deep State Ledger 1/3 (one won without
playing it): 12 of 19 proven, 5 lost. Forcing a card at its first chance costs some hard wins, but a single
oracle's d4 result swings with its seeds (the gate), so at these counts no card stands out.

**Premises that moved:** Deep State Ledger is played, in 8.0% of the strategist's games (9.9% oracle), not never;
Dyson Swarm Tranche in 3.1%. Today's oracle at d4 read 80.6% on fresh seeds (the tier table's target is 95).

## Seeds and outputs

Fresh seeds 47488-47703 (taken; ledger next free 47704): the d0-d3 read 47488-47679, the d4 add-on 47680-47703.
Dev seeds 36800-36823 for screens and the gate. Kept: `out/keep/` (243-read-*: the read's games, results and
traces; 243-gate: the gate's arms).
