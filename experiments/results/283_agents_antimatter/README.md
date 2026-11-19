# 283: the agents' table rates, antimatter's valuation, three projects at 24 (r135)

Matt (2026-11-18/19). `RULES_REVISION` 134 → 135 (bumped from the constant) for the three projects; everything else
is agent numbers and tooling. Base: main at 9a06b39 (r134). Every number below is a **candidate**.

## What shipped

| Area | r134 | r135 |
|---|---|---|
| Martial Law (compute a board) | 29: Surge 2 station + Surge 2 factory | **24: Surge 3 station** (one harm, sized up) |
| Recruitment Drive | 32: Surge 4 city + Surge 2 station | **24: Surge 5 city** (one harm, sized up) |
| Grand Offensive | 32: Infiltrate again + Surge 3 city + Surge 2 factory | **24: Infiltrate again + Surge 2 factory** |
| `values.json` resources | insight 1.67, chips 2.67, cubes 2, ambassadors 2.67, antimatter 12 | **2.67, 4, 4, 5.33, 18** (the anchors; antimatter 18 by ruling) |
| `values.json` rates | drawn card 2.67, keep 1.67 / 4.33, `horizon_t2` 2.5 | **3.33, 2.67 / 6.93** (moved with insight), **3.5** (the anchor check's 3-4 rounds) |
| Eval `keep_units` | keep price mixed insight with r127 credits | **on**: spare insight covers a keep counted in insight |
| Eval `research_delay` (D106) | a held research removal at its printed cubes | **on**: times the share of a firing still coming (`rates.research_round` 1 a country a round) |
| Liaison Corps' face | "move up to 4 ... from your supply or on or next to it onto pieces on or next to it" | "... onto pieces on or next to it. Each comes from your supply or from on or next to it." |

Matt's answer on the projects (message 12916): the bundle rule is soft, so Martial Law and Recruitment Drive keep one
effect, strengthened to fit 24 (no line borrowed from another project). Edits: [projects_24.py](projects_24.py).

## Screens (dev seeds 36800-36831, the judge and greedy, three pairs, d0-d4, 480 games an arm per agent)

Full tables: [screens.md](screens.md) ([make_screen.py](make_screen.py), arms as agent specs).

| Lever | Judge gap ± SE | Greedy | Verdict |
|---|---|---|---|
| Resources + drawn card at the anchors (Tr) | +1.9 ± 1.8 | +0.4 ± 2.1 | shipped |
| `horizon_t2` 3.5 (Th) | +1.2 ± 0.7 | +0.6 ± 0.6 | shipped |
| Keep rows at the anchors (T vs Tnk) | identical games | identical | shipped (moved with insight; a reading only) |
| Shipped table together (R) | +1.9 ± 1.8 | +0.4 ± 2.1 | shipped |
| R + `keep_units` (RK) | +1.9 ± 1.9 | +0.4 ± 1.9 | **shipped** |
| `research_delay` (D106) | +0.0 ± 0.0 | +0.0 ± 0.0 | shipped: changed no game (D106's pick (a), correct and free) |
| Agent removed 3.33, drain 1.33, discard 10 (Trm) | **-3.8 ± 2.0** | -2.1 ± 2.0 | not shipped: the agents chase removals |
| `tag_share` 0.5, a committed seat (Tf) | +1.5 ± 1.6 | -1.7 ± 1.3 | not shipped (mixed) |
| `w.antimatter` 3 (A3) / 4 (A4) | +0.0 ± 1.4 / -1.2 ± 1.5 | +1.2 / +0.8 | not shipped (below) |
| + `w.sink` 1 (A3s / A4s) | -1.0 ± 1.6 / -1.9 ± 1.7 | -0.8 / +0.4 | not shipped |
| RK + `w.antimatter` 3 (RKA) | +0.2 ± 2.1 (d0 **-7.3 ± 3.1**) | +0.4 ± 2.0 | not shipped |
| Shipped + `w.sink` 1 (screen f) | -1.0 ± 1.1 | -0.8 ± 1.5 | not shipped (spends more: below) |
| Shipped + table antimatter 12 (screen e) | -1.2 ± 1.6 | -1.0 ± 1.2 | 18 stays |
| `plan_target` (237's pick) on C | -1.3 vs C | +0.0 | left off (code kept, `plan_target=1`) |

## Item 1: the 17 cards at the table

[table_nets.py](table_nets.py) reads each card's net (effect_ledger reasoned − printed − keep) under each rate set.
**3 of the 17 now read above 0** (Patent Library +0.7, Recursive Self-Improvement +5.7, Positron Storage Ring +9.8;
all 17 were under 0 at r134). Raising insight to 2.67 raises a card's cost side as much as its gain side, and the
keep moves with it, so the rest stay under: their gain is an engine, a trigger or agent removals, which the table
still prices under the anchors. The removal rows at 3.33 would lift 2 more (Martial Cordon, Bug Bounty Board) but lost
-3.8 ± 2.0 in play; the table's net is the agents' reading, so it stays. Cards at or under 0: 62 → 79 of 208.

## Item 2: antimatter

| Antimatter (fresh read, a seat-game) | r134 judge | r135 judge | r134 greedy | r135 greedy |
|---|---|---|---|---|
| Gained → spent | 3.03 → 1.61 (53%) | 3.07 → 1.40 (**46%**) | 2.43 → 1.30 (54%) | 2.35 → 1.23 (**52%**) |
| Spent in d3-d4 won games (victory-skip, min 75%) | 85% | 83% | 85% | 81% |
| Orbital Relay Launcher burns | 0.49 | 0.37 | 0.15 | 0.15 |

Pooled, d3-d4 wins spent antimatter in 82% (r134) and 83% (r135) on these seeds (282's own seeds read 70%: the
prompt's "skip in 70%" is that number, spent in 70%). The share spent, pooled, 49% (warned, band 50%).

**The brief's lever, a higher flat weight, made more and spent less**: at `w.antimatter` 3 Antiproton Foundry's Action
reads as a gain (its use 0.09 → 0.29 a seat-game) but the judge's Orbital Relay Launcher burns fell 0.49 → 0.13, so
the spent share fell 53% → 39%; the sink (`w.sink` 1) restored neither. With the shipped table it cost the judge
1.7 points (d0 -7.3 ± 3.1). Not shipped (question 2's default changed on that evidence). The table's antimatter 18
(the ruling) has a cost too: **Containment Reactor** (1 antimatter → 4 energy + 1 Sway, 14.7 against 18 at the anchors,
0.81) went from kept 0.23 a game to never kept. Screen e sized the ruling on the shipped build: the table's antimatter
at 12 instead of 18 spends more (judge 48% → 54%, Orbital Relay Launcher 0.32 → 0.48) but wins less (-1.2 ± 1.6
judge, -1.0 ± 1.2 greedy), so 18 stays. (The shipped build replayed screen d's RK games exactly.)

Screen f, the brief's lever again on the shipped build: the sink alone (`w.sink` 1) raises Orbital Relay Launcher
burns (judge 0.32 → 0.41, greedy 0.17 → 0.36) and the spent share (48% → 49%, 53% → 58%) for -1.0 ± 1.1 judge,
-0.8 ± 1.5 greedy: the best spending lever found, but both point estimates lose, so not shipped (it would also need
fresh seeds). With `w.antimatter` 3 it loses more (-2.1 ± 1.5 judge, d0 -11.5 ± 3.3) and spends less (37%).

## Fresh read (seeds 50124-50171, 1,440 games a side; r134 farmed at 9a06b39 from f282-read's grid)

Both the projects (a rule) and the agents' table changed between the sides: a gap is the two together. Full table:
[read.md](read.md) ([read.py](read.py)).

| Agent | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| Judge win % r134 → r135 (gap ± SE) | 93.8 → **93.1** (-0.7 ± 2.5) | 83.3 → **90.3** (+6.9 ± 3.4) | 79.9 → **75.7** (-4.2 ± 3.8) | 54.2 → **49.3** (-4.9 ± 4.3) | 26.4 → **26.4** (0.0 ± 3.4) |
| Band | 90+ met | 80-95 met | 75-90 met | 55-70 **unmet** (-5.7) | none |
| Greedy | 92.4 → 91.7 | 81.9 → 82.6 | 72.9 → 73.6 | 41.7 → 47.2 | 27.1 → 20.1 (-6.9 ± 3.1) |
| Judge rounds | 8.66 → 8.37 | 9.63 → 9.26 | 9.50 → 9.28 | 9.27 → 9.06 | 8.19 → 8.42 |
| Cities corrupted by round 4 | 0.78 → 0.80 | 2.18 → 2.20 | 2.36 → 2.36 | 2.84 → 2.83 | 3.76 → 3.81 |

Pooled: judge -0.6 ± 1.6, greedy -0.1 ± 1.7. The d3 judge band stays unmet (49.3; 54.2 on r134's side of these seeds).

## Resource use (fresh read, the judge, a seat-game)

| Resource | Gained | Spent | Spent of gained | Top uses | Binds? |
|---|---|---|---|---|---|
| Credits | 172.4 | 185.3 | 107% | botnets 12.3, ambassadors 10.4, Buy energy 7.3 (card plays, spread over many rows, the most) | yes |
| Insight | 51.4 | 37.6 | 73% | keeps 25.8 | half: the rest lapses |
| Energy | 40.3 | 34.3 | 85% | Accord 4.5, Sprawl 3.3, Peer Review 3.2 | mostly |
| Chips | 6.5 | 4.5 | 68% | converters 3.2, stations 0.4 | loosely |
| Antimatter | 3.1 | 1.4 | 46% | Orbital Relay Launcher 0.37, Hostile Buyout 0.11 | no: under its band |
| Research cubes | 6.1 | 4.5 | 74% | converters 1.9, Survey Drone Fleet 0.55 | loosely |
| Ambassadors | 19.6 | 16.4 | 84% | Liberate 10.8, moves 3.8, Remove 1.0 | yes |

## Tooling

- `aic-sim farm`: the shard at the spec's `first_seed` leads with the header over the spec's range (`Farm::lead`,
  native and wasm), so a `farm-seeds` out file from that seed reads as a games file; readers skip a second header.
- `scripts/bench/resource_split.py`: 247's diag.py and arms.py (resource use per arm, board and round; wins paired
  stay `read_lib.py --agent-arms`).
- `cargo test-all` / `test-slow` run `crates/aic-lanes`: cargo's flags (`-j 3`, `--no-fail-fast`, `-p`, `--test X`)
  go to cargo, the rest to the tests.
- `speed.py` runs a copy of each SIM under `out/speed-bin/`.

## Checks

`cards:fit` 0 under 7.5 pt; loop smoke no timeouts, no card at the flag; fast lane, full lane and clippy clean.
`per_decision.py` (d2, 16 games, best of 2, r134 built from 9a06b39; a loaded box, ratios only): greedy 1.05× a
decision (1.5 → 1.6 ms), the judge 1.02× (1.6 → 1.6 ms); games 1.01-1.07× longer. `speed.py check` gives no verdict
across the revision; not re-based. design_check: [design-check.txt](design-check.txt).

## Stretch

- 237's pick (`plan_target`): built and screened, -1.3 judge on C: left off.
- Smart Grid Backbone: not a blind spot at r134/r135 on the fresh seeds (kept 0.165 → 0.17 a game, played 0.12 / 0.11).

## New-player summary (r135)

- **Martial Law** (24) puts 3 agents on each country's research station holding the most: enough to corrupt it.
- **Recruitment Drive** (24) puts 5 agents on each country's city holding the most: a city falls at any level.
- **Grand Offensive** (24) lands this round's Infiltrate lines again and 2 agents on each country's busiest factory.
- **Liaison Corps** reads in two sentences: where the ambassadors go, then where each comes from.
- **Rules that resisted:** the d3 band (49.3 against 55); antimatter under-spent (46-52% of what is gained).
