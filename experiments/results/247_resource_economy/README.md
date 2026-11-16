# 247: resource economy: energy gets a job, antimatter and chips get spent

Base: main 3079260, r110 (246). **Rules change: r111** (bumped from the constant, 110 to 111). Matt's rulings of
2026-11-15 (energy powers things; cut supply only if new uses don't absorb it). Tables: [bands.txt](bands.txt) (the
fresh read with the standing measures), [resource-use.txt](resource-use.txt) (whole-game resource use, both sides),
[resource-diag-r111.txt](resource-diag-r111.txt) (uses and sources by board), [design-check.txt](design-check.txt),
the screens [screen-s247a.txt](screen-s247a.txt) to [screen-s247d.txt](screen-s247d.txt), the content change
[overlay.txt](overlay.txt). Readers: `arms.py` (screen arms against a base read), `diag.py` (by board and round),
`acts.py`, `smoke_read.py`, `mkspec.py`.

## Why each resource sat unspent (246's kept r110 read, 4,608 seat-games)

- **Energy is a stock, not a flow.** Held after the action phase by round: the Builder 4 at round 2, 14 at round 3,
  then 15-17 every round to the end (it gains 50 a game, mostly from its aligned cities and Tidal Grid's payout, and
  spends 64%); the Protector 7 (3 a round printed); the Researcher 4-6 and spends 85%. The Builder's only energy
  powers were Rush Build and its node steps. The strategist holds energy at 0.8 points (1.3 credits), so it spends
  energy whenever a use exists: the surplus was missing uses, not hoarding.
- **Antimatter had only card uses.** 4.2 gained a seat-game, 1.5 spent, held from round 4 and climbing; every
  spend was a major in hand. Not late arrival (the Horizon converter and the capstones make it from round 3), not
  price: no use open to every seat.
- **The Tidal Grid chip converter** made energy (2 chips for 3), the resource every seat held spare, and the
  strategist holds a chip at 4 points (6.7 credits) and an energy at 0.8: it never trades 2 chips for 3 energy.
- **Dyson Swarm Tranche** costs 1 antimatter and 12 credits for energy; the flat eval counts its +2 income as nothing
  (246) and its 10 energy at 0.8 a unit.

## The change (r111)

| | was | now | give / get, values.json (anchors) |
|---|---|---|---|
| **Liberate with antimatter** (standard action, `board.liberate_antimatter` 2) | none | 1 antimatter pays 2 of a Liberate's ambassadors: a city 3 + 1 antimatter, a station 1 + 1, a factory the antimatter alone | 18 for 8 (18 for 16 = 0.89: at par or under, the standard-action stance; Liberate's win-route exemption) |
| **Buy In** (Builder node power) | 6 credits | 2 energy (+ the step's) | 9 for a node 8 + a chip at 72% 2.9 = 10.9, 1.21 (9 for 12.3, 1.37) |
| **Accord** (Protector node power) | sacrifice a Diplomacy card | 2 energy (+ the step's) | 9 for a node 8 + an ambassador at 72% 4.3 = 12.3, 1.37 (9 for 13.8, 1.53) |
| **Tidal Grid chip converter** | 2 chips for 3 energy | 2 chips for 1 antimatter | 8 for 18, 2.25 (12 for 18, 1.5); once a round, as converters are |
| **Orbital Observatory Array** Action | pay 2 credits, draw 3 | pay 1 energy, draw 3 | 4.5 for 12, 2.7 (4.5 for 15, 3.3) |
| **Citywide Microgrid** Action | sway 2 | pay 1 energy, sway 2 | 4.5 for 8, 1.78 (4.5 for 8) |

Agents: the strategist's purge term counts each antimatter as 2 ambassadors toward one Liberate in its reach
(`w.purge_am` 1; the diff review capped it at one antimatter a city in the bill). The antimatter reserve
(`am_save`) does not block Liberate with antimatter: a liberation is a major's worth. `board.node_energy` (0) is a
param for Place a node's energy, screened and not shipped. Loops: none (the converter is once a round; a Liberate
needs a corrupted site and its bill). Card fit: both cards fit (`npm run cards:fit`).

**Supply not cut.** The new uses absorbed the surplus (below); the Protector's printed energy (S: 2 to 1) was
screened and not needed once Accord ran on energy.

## Screens (spent seeds 47896-47959, 768 games an arm, paired with 246's kept r110 read)

The build with the new params at 0 replays 246's r110 read exactly (12 of 12 games checked), so its read is the base.

| arm (s247a-d) | wins d0 / d1 / d2 / d3 (paired gap, all) | energy spent share, held after actions | antimatter spent | Tidal chip conv. |
|---|---|---|---|---|
| base (r110) | 93.8 / 86.5 / 76.6 / 56.8 | 76%, 9.70 (Builder 17.7) | 36% | 0.00 |
| L: Liberate with antimatter | 95.8 / 91.7 / 89.1 / 71.9 (+8.7 ± 1.4) | 74%, 10.0 | 68% | 0.00 |
| L + Buy In 2 energy | 96.4 / 96.9 / 91.7 / 78.1 (+12.4) | 84%, 6.0 (Builder 6.6) | 70% | 0.00 |
| + three credit Actions on energy | +13.9 | 84%, 5.7 | 71% | 0.00 |
| + Tidal chips for antimatter (LBCT) | +14.2 | 84%, 5.6 | 71% | 0.18 |
| LBCT, agent's antimatter term off | +8.5 | 84%, 6.0 | 67% | 0.00 |
| LBCT, strategist chip 3 points (not 4) | +14.3 | 83%, 5.8 | 71% | 0.22 |
| Buy In 3 energy (A3) | +11.7 | 86%, 5.1 (Builder 4.8) | 71% | 0.21 |
| A3 + Accord 2 energy | +13.5 | 88%, 3.9 (Protector 2.6) | 72% | 0.18 |
| A3 + Protector prints 1 energy | +10.7 | 88%, 4.3 | 74% | 0.22 |
| A3 + Place a node 3 credits + 1 energy | +12.1 | 89%, 3.7 (Researcher 1.9) | 72% | 0.20 |
| Q: L, Buy In 2, Accord 2, Tidal, Orbital | +14.8 | 87%, 4.4 | 69% | 0.17 |
| Q + nine free Actions on 1 energy | +15.0 | 87%, 4.6 | 71% | 0.20 |

Picked: Q with two of the free Actions (the shipped set). Buy In 3 energy cut the Builder's stock further but prices
under par at both tables (13.5 for 12.3); Place a node on energy hit the Researcher, the energy-short board (held 1.9);
the Protector's supply cut wasn't needed once Accord took energy. Seven of the nine free Actions stay free: at
1.0-1.3 now (the jam buildings, Firewall Array, Checkpoint Network) an energy a use puts them under par, and Lure
Network and Embassy of Exiles read under their price at values.json once they pay it (agent removal at 2.2 there).

## The fresh read (48088-48279, 192 seeds x 3 pairs x d0-d3, the strategist, paired against r110)

| | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| r111 | 96.7 ± 0.7 | 97.0 ± 0.7 | 95.7 ± 0.8 | 83.0 ± 1.6 |
| r110, same seeds | 94.1 | 87.5 | 79.3 | 54.2 |
| gap ± paired SE | +2.6 ± 1.1 | +9.5 ± 1.5 | +16.3 ± 1.8 | +28.8 ± 2.2 |
| band (soft) | 90+ met | 80-95 **unmet** (over) | 75-90 **unmet** (over) | 55-70 **unmet** (over) |

Pairs: bp 94.7, br 92.3, pr 92.3 (r110 79.9, 81.1, 75.3). Rounds: d3 7.8 (won 7.5), under the 8-10 target.

| per seat a game | r110 | r111 |
|---|---|---|
| energy gained / spent (share) | 41.9 / 31.7 (76%) | 40.1 / 34.4 (86%) |
| energy held after the last action phase, d0 / d1 / d2 / d3 | 12.4 / 10.6 / 8.8 / 6.7 | 5.9 / 5.3 / 4.5 / 3.5 |
| the same, Builder / Protector / Researcher | 16.8 / 7.4 / 4.6 | 7.1 / 3.0 / 4.3 |
| antimatter gained / spent (share) | 4.08 / 1.48 (36%) | 3.82 / 2.77 (73%) |
| Liberate / Liberate with antimatter | 3.06 / 0 | 1.88 / 1.58 |
| Tidal Grid chip converter | 0.00 | 0.14 |
| chips gained / spent | 10.3 / 5.8 (56%) | 10.3 / 5.9 (57%) |
| Dyson Swarm Tranche kept / played a game | 0.23 / 0.05 | 0.20 / 0.05 |

Energy's biggest spends are now Accord (6.4 a seat-game with its steps), Buy In (6.0) and Place a node (4.4);
antimatter's, Liberate with antimatter (1.58 of 2.77). **Surprise:** the burns. Teardown (1 chip: destroy your
factory, 1 antimatter) runs 1.38 a Builder seat-game and Meltdown 0.52 a Researcher one (bands 0.02-0.50): the
strategist turns its own facilities into Liberate's antimatter. The agent's antimatter term drives it (term off:
Teardown 0.02, s247b); it is legal, strong play under the new rule, so it is reported, not patched.

**The judge's plays** ([judge-plays.txt](judge-plays.txt), share of games, r110 → r111): Citywide Microgrid
35.1 → 18.9 (its 8 energy to play now competes with the node powers), Orbital Observatory Array 19.0 → 15.9; Dyson
Swarm Tranche 5.4 → 4.8, Positron Storage Ring 14.4 → 13.7, Wafer Fab Line 27.9 → 28.5, Antiproton Foundry
27.9 → 27.6. `play_read.py`: no card kept and never played.

## The leads: what the energy and antimatter cards add now (fresh 48280-48375)

246's good-card measure ([cardvalue.txt](cardvalue.txt)): the engine (`engine:strategy=1`) with each card blanked
against the base deck, d2-d3, three pairs, 96 seeds, 576 games an arm, on r111's final build. The base arm wins
71.0% (246's r110 engine: 44.6%).

| card | adds on r111 (± SE) | 246 on r110 | played by the engine |
|---|---|---|---|
| Positron Storage Ring | +1.7 ± 1.5 | -3.1 ± 1.6 | 40.3% |
| Wafer Fab Line | -1.9 ± 1.4 | -3.1 ± 1.8 | 48.1% |
| Antiproton Foundry | -0.7 ± 1.6 | -2.4 ± 1.6 | 55.7% |
| Dyson Swarm Tranche | +0.2 ± 1.2 | -1.7 ± 1.3 | 23.3% |

None costs the engine measurably on r111. 246's r110 finding was not re-measured on fresh r110 seeds: these cards
were not changed (the condition the brief set for confirming first), and their r111 reads are what the change left.
Positron Storage Ring and Antiproton Foundry make antimatter, which now has a use; both moved the right way, but no
read here separates that from noise (the shifts are about 2 SE).

**Dyson Swarm Tranche stays kept, not played** (kept 0.20 a game, played 0.05; the judge plays it in 4.8% of games).
Why: it costs 1 antimatter, now worth 2 ambassadors in a Liberate, for 10 energy and +2 energy income; the
strategist's flat eval reads energy at 0.8 a unit and a card's income as nothing (246), so the play never scores. The
engine plays it in 23% of games and it adds nothing measurable there. A redesign would give it an effect the board
terms see (nodes, Sway); not done: the brief asked for a re-read, not a redesign.

## Tidal Grid's chip converter

Changed (2 chips for 1 antimatter, was for 3 energy): 0.14 a seat-game on the fresh read, under its 0.30-3.00 band.
The cause is the judge: it holds a chip at 4 points (6.7 credits), so it trades 2 chips only for something its eval
scores above 8; an antimatter scores 2, plus the purge term's share while a Liberate bill in reach is short (the
converter's uses). The strategist at 3 points a chip (s247b) used it 0.22. A human with spare chips and a city to free
has a clear trade (12 for 18 at the anchors); the done-when's "changed with a reason" is the bar met.

## 241's picks (code and docs only; play unchanged)

The golden game replays unchanged (fingerprint as before; content hash moved), and the frozen fresh read's 2,304
games replay on the final build with the same actions and outcomes (`farm/compare.py`: only the renamed tallies
differ). **Country Shelter is gone:** `Op::Shelter` is the reach Shelter alone (a unit op; the three cards drop
`"reach": true`), `RoundFlag::Shelter`, `board::shelter` and `unsheltered` gone, `values.json`'s two shelter rows one.
**RemoveAmbassadorsCountry** keeps its name and loses `reach`, `unity_per` (one card printed 0) and its country
branch. **CorpTroops** loses `permanent`, `fill` and `rounds` (printed nowhere). **Tallies:** `cards.troops_in_reach`
gone; the removal area tallies are keyed by source (`card.`, `power.`, `standard.remove_troops_at.<at>`; powers and
standard actions now name their effect context). **`BEATS`** and `beat_of` are test-only. **`hand_worth`'s** flat
fallback is documented. **Doc retention:** rules-history.md drops r96-r97 (12 KB); experiments/history.md is one
line a row (46 KB to 20 KB). **Not done:** "move `project_worth` into value.rs": no `project_worth` exists in the
tree or its history.

## Seeds and outputs

Screens on spent seeds 47896-47959. Fresh, taken: 48088-48279 (the read), 48280-48375 (the leads); ledger next free
48376. Kept (`out/keep/`): `247-read-r111`, `247-read-r110` (the fresh read, both sides), `247-cardvalue` (the leads).
Specs: `experiments/archive/farm/s247*.json`, `smoke247.json`, `v247l.json`. Speed: per decision the engine 1.10x
and greedy 1.08x r110's (10-16% more options a decision: the new action, energy-paid powers); the baseline is
re-based on r111.
