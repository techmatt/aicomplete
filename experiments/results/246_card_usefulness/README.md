# 246: card usefulness: which neglected cards are good, and redesigns of the ones nobody plays

Base: main 4c3816f, r109 (245's judge). **Rules change: r110** (bumped from the constant, 109 to 110): six cards
redesigned, candidates for Matt's review in the card viewer. No judge heuristic shipped (none had a gain to find).
Tables: [gap.txt](gap.txt) (the card list), [cardvalue.txt](cardvalue.txt) (the good-card measure),
[table.md](table.md), [redesign-value-1.txt](redesign-value-1.txt), [redesign-value-2.txt](redesign-value-2.txt),
[bands.txt](bands.txt). Overlay of the pass: [redesign.txt](redesign.txt). Readers: `gap.py`, `cardvalue.py`,
`agree_cards.py`, `table.py`, `mkspec.py` (writes a card-value spec into `experiments/farm/`).

## The card list, re-read on 245's judge

245's kept test read (the tuned judge, 47704-47895) against 245's oracle on its 576 cells (47704-47751, d0-d3):
[gap.txt](gap.txt). The 10 largest oracle-minus-judge play gaps (raw and past the oracle's 1.11x general extra play
alike): Power to the People, Positron Storage Ring, Forensic Audit, Antiproton Foundry, The Nuclear Option, Wafer Fab
Line, Dyson Swarm Tranche, Citywide Microgrid, Catalyzed Fab Run, Treaty Isotope Bank. Played in under 10% of games by
both: Smart Grid Backbone, Baseload Contract, Field Grant, Annihilation Lance, Pumped Hydro Storage, Deep State Ledger;
plus Multiyear Defense Budget (prompt-named; judge 3.1%, oracle 12.2%). 17 cards. **Premises that moved:** Continental
Supergrid and Tidal Barrage are no longer top-10 (gaps 13.8 and 8.5); Agency Directorate and Multiyear Defense Budget
are over 10% for the oracle; Pumped Hydro Storage joins the unused.

## The good-card measure

**What a card adds to a player that plays it, without deck sight:** the engine (`engine:strategy=1`, the oracle's
own base policy: 80% of the oracle's picks, 78% of its plays, against the judge's 54% and 41%) with the card in the
deck against the card **blanked** (`cards.<id>.cost=99`: never affordable, so never kept or played; the deck order is
unchanged, so each game pairs with base on its seed, pair and level). d2-d3 (the engine wins 44.6%: no saturation),
three board pairs, 96 seeds (47488-47583, spent seeds: a measure, not a gate), 576 games an arm. **Noise:** the
paired SE is 0.9-1.8 points a card; with 17 cards, one or two at 2 SE are expected by chance. The oracle itself was
not used: at d3 it wins every seed (rounds are its only margin, and its first winning line doesn't minimize them), at
d4 a game is ~150 s, and its card use is partly deck sight.

| card | judge play % | oracle | engine | adds (win pts) | ±SE | branch | outcome |
|---|---|---|---|---|---|---|---|
| Power to the People | 14.1 | 50.5 | 32.6 | -1.2 | 1.7 | A | not good shown; judge left as is |
| Positron Storage Ring | 18.8 | 52.1 | 39.4 | -3.1 | 1.6 | A | costs the engine wins; judge right to skip it |
| Forensic Audit | 29.0 | 59.4 | 42.9 | +1.4 | 1.5 | A | no gain shown |
| Antiproton Foundry | 31.9 | 62.0 | 49.5 | -2.4 | 1.6 | A | costs the engine wins |
| The Nuclear Option | 17.7 | 45.5 | 30.2 | +0.3 | 1.4 | A | no gain shown |
| Wafer Fab Line | 29.5 | 58.3 | 46.5 | -3.1 | 1.8 | A | costs the engine wins |
| Dyson Swarm Tranche | 4.2 | 30.0 | 20.5 | -1.7 | 1.3 | A | no gain shown |
| Citywide Microgrid | 33.0 | 61.6 | 53.6 | -0.9 | 1.8 | A | no gain shown |
| Catalyzed Fab Run | 27.8 | 55.6 | 34.4 | +1.7 | 1.3 | A | no gain shown (1.3 SE) |
| Treaty Isotope Bank | 25.7 | 53.1 | 37.3 | -0.3 | 1.7 | A | no gain shown |
| Annihilation Lance | 3.0 | 9.2 | 6.4 | +1.0 | 0.9 | A | no gain shown |
| Smart Grid Backbone | 1.2 | 1.0 | 2.1 | -0.3 | 1.2 | B | redesigned (below) |
| Baseload Contract | 2.8 | 5.2 | 4.2 | +0.3 | 1.6 | B | redesigned |
| Field Grant | 8.0 | 1.6 | 1.9 | +1.0 | 1.4 | B | redesigned |
| Pumped Hydro Storage | 4.3 | 8.7 | 5.2 | +0.3 | 1.6 | B | redesigned |
| Deep State Ledger | 5.7 | 8.0 | 4.0 | +0.5 | 1.2 | B | redesigned |
| Multiyear Defense Budget | 3.1 | 12.2 | 5.4 | +1.0 | 1.6 | B | redesigned |

Judge and oracle: share of games played in, d0-d3, the oracle's 576 cells; engine: the measure's base arm, d2-d3.
**No card adds wins measurably.** The three Action buildings that spend energy or chips (Positron Storage Ring, Wafer
Fab Line, Antiproton Foundry) read 2-3 points the other way (1.5-1.9 SE): the engine wins slightly more without them.
That is 243's finding made card by card: the oracle and the engine play the energy cards 2-7x more than the judge, and
playing them is not what wins.

## Branch A: the judge (no heuristic shipped)

`aic-sim agree` now tallies plays per card (`play:<card>`). On 245's oracle games the judge plays what the oracle
played at the same decision 9-25% of the time for Wafer Fab Line, Dyson Swarm Tranche, Multiyear Defense Budget, Deep
State Ledger and Positron Storage Ring (the engine 76-89%). **The misjudgment:** the flat judge counts AI-track income
only, so a program's production and a building's Action read as nothing, and a building reads at about its cost
(`built` 4 + 0.5 × cost against 0.6 a credit). Two sentence-sized probes on 192 oracle games: count card production at
the track's rate (`prod_income`), and points per Action building (`built_action` 3-12). Play agreement stayed 41%;
Multiyear Defense Budget 9 → 20%, Treaty Isotope Bank 50 → 74% at 12, Wafer Fab Line 10 → 14%, Dyson none. With no
card shown to add wins, no win screen was owed: both retired (`eval::RETIRED`, with these numbers), greedy and the
judge unchanged.

## Branch B: six redesigns (shipped, r110 candidates)

**Why nobody played them:** energy sits spare. A judge seat holds 7.5-13 energy after its last action phase (d3-d0)
and spends about 70% of the energy it gains; the anchors price energy at 4.5 and +1 energy income at 14 (3.5x a credit
of income), so the energy-income cards read fair and did nothing. Field Grant's cubes needed an open megaproject.

| card | old | new | give / get (anchors) | values.json give / get | engine: play %, adds ±SE | judge: play %, adds ±SE |
|---|---|---|---|---|---|---|
| Field Grant (T1 program, Science) | 2 cubes onto an open megaproject, 8 cr | gain 3 research cubes, 9 cr | 13 / 18 = 1.38 | 11.5 / 10 + override 4 | 1.9 → 24.5, +3.4 ± 2.0 | 10.7 → 26.3, -0.5 ± 1.7 |
| Smart Grid Backbone (T2 building, needs 3 Power, 25 cr, 2 ins) | 1 energy a tag played; 2 energy when built | 10 credits a Power card you play (itself included); draw 3 when built | 45 / 60 = 1.33 (a Power card a round) | 36.5 / 24.3 + override 20 | 2.1 → 19.3, 0.0 ± 1.6 | 0.5 → 3.9, -1.0 ± 1.1 |
| Baseload Contract (T1 program, Power) | +2 energy income, 24 cr | +2 energy income, 2 ambassadors, 26 cr | 30 / 44 = 1.47 | 28.5 / 37.8 | 4.2 → 6.7, +3.8 ± 2.7 | 2.5 → 18.3, -1.2 ± 1.9 |
| Pumped Hydro Storage (T1 program, 28 cr) | Power; +1 energy income, 4 energy, lift 1 Fallout | Power + Diplomacy; lift 2 Fallout on or next to it, +5 credit income | 32 / 37 = 1.15 | 30.5 / 36.6 | 5.2 → 10.4, +2.1 ± 2.5 | 3.5 → 10.8, +0.4 ± 2.3 |
| Multiyear Defense Budget (T1 program, Security + Power, 34 cr) | +4 credit, +2 energy income | +8 credit, +1 energy income | 38 / 46-54 = 1.21-1.42 | 36.5 / 47.9 | 5.4 → 15.4, -0.4 ± 2.5 | 3.2 → 5.0, +2.1 ± 1.5 |
| Deep State Ledger (T2 program, needs 2 Security, 18 cr, 2 ins) | +3 insight income, 2 Operatives | Target: Agents; remove 3 on it, +3 insight income | 38 / 51 = 1.34 | 29.5 / 24.8 + override 10 | 4.0 → 18.3, +5.0 ± 2.3 | 6.1 → 12.5, -3.8 ± 2.3 |

Each redesign's measure is the originals': the new card against its blank, same seeds, engine and judge (d2-d3,
47488-; Field Grant and Smart Grid 64 seeds, 384 games an arm; the rest 40 seeds, 240). Before: the engine's rate from
the measure's base arm, the judge's from 245's test read at d2-d3. **Every redesign is played more, by both agents
(1.5-12x); none reads a win gain the noise can't explain:** the largest, Deep State Ledger, reads +5.0 ± 2.3 for the
engine and -3.8 ± 2.3 for the judge. **Three designs moved after their first measure** (the diff review and
`design_check`, before any fresh seed): Multiyear Defense Budget and Pumped Hydro Storage first printed 2 ambassadors
and +5 credit income without a Power op (tag-story); MDB took +1 energy income for the ambassadors (re-measured above:
the judge plays it 5.0% against 19.2% with ambassadors, which its flat terms see); PHS's Power-clean try (gain 1
antimatter) was played no more than the old card (engine 5.4%, judge 3.8%), so the +5 credit income design ships with
the Diplomacy tag, whose story is "green energy and cleanup" (re-measured on the final build). Deep State
Ledger first drew 3, Recursive Self-Improvement's exact effect (guideline 11), so it now removes 3 agents (re-measured
on the final build). Design checks: anchors 1.15-1.47 (tier-1 ceiling 1.5); a gain at `values.json` for each (three
`overrides` rows where the table prices cubes, a Power trigger and agent removal under the anchors); Smart Grid
Backbone's payback 1.6 rounds (`payback.py`; it pays 10 back on its own play, Citation Engine's pattern);
`npm run cards:fit` fits all six; `loop_smoke.py --run` no timeout, no card past the flag.

## Bands on the final build (fresh 47896-48087, 192 seeds × 3 pairs a level, paired against r109)

| | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| r110 judge, final build | 93.8 ± 1.0 | 87.2 ± 1.4 | 81.8 ± 1.6 | 56.4 ± 2.1 |
| r109 judge, same seeds | 94.1 | 88.0 | 81.1 | 56.9 |
| gap ± paired SE | -0.3 ± 1.0 | -0.9 ± 1.4 | +0.7 ± 1.5 | -0.5 ± 1.8 |
| band (P1, Matt 2026-11-15) | 90+ met | 80-95 met | 75-90 met | 55-70 met |

All four bands met; no level moved past 1 SE ([bands.txt](bands.txt), with the standing measures). The read was frozen
on the build before Pumped Hydro Storage's Diplomacy tag (93.6 / 86.1 / 82.1 / 56.4); its seeds were replayed on the
final build (`same_results.py`: 2,624 rows differ, as a content change makes them) and the table above is the replay.
**Resource use** ([resource-use.txt](resource-use.txt)): research cubes gained a seat-game +0.1 to +0.7 (Field Grant),
spent +0.1 to +0.5; energy, chips and antimatter within 2%; antimatter still spent 35% of what is gained (a standing
warning). **The judge's plays on these seeds** ([judge-plays.txt](judge-plays.txt), share of games, d0-d3, r109 →
r110): Field Grant 10.1 → 30.6, Smart Grid Backbone 1.3 → 9.2, Baseload Contract 3.9 → 20.7, Pumped Hydro Storage
5.6 → 16.2, Multiyear Defense Budget 4.9 → 10.4, Deep State Ledger 9.2 → 15.6.

`design_check` (final build against r109, [design-check.txt](design-check.txt)): no warning from this item; three stand
from r109 (Dyson Swarm Tranche kept not played, Tidal Grid converter unused, antimatter 35% spent). `speed.py check`:
greedy 0.95x, engine 0.98x, planner 1.02x the r109 baseline (no verdict across a revision; no agent code changed, so
the baseline was not re-based).

## Seeds and outputs

The measure and the redesign reads used spent seeds 47488-47583 (243's). Fresh, taken: 47896-48087 (the band
re-read; ledger next free 48088). Kept (`out/keep/`): `246-cardvalue` (the 10,368-game measure), `246-read-r110`
and `246-read-r109` (the band re-read, both sides). Specs: `experiments/archive/farm/v246*.json`.
