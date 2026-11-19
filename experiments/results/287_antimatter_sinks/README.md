# 287: powerful antimatter sinks (r138)

Matt (2026-11-19). `RULES_REVISION` 137 → 138 (bumped from the constant). Base: main at 35c98d12 (r137, after 286).
Edits: [sinks.overlay](sinks.overlay). Every number is a **candidate**.

## Rulings and amendments

- Locked: antimatter sinks are powerful, to justify the investment (antimatter 18 at the anchors).
- Amendment 13116: aim each sink at 2.0-2.5× at the anchors, two near 2.5; err high (scaled back later if overwhelming).
- Amendment 13123: a strong route, not the only one; no strong non-antimatter end-game card converted (Orbital Mass
  Driver, the engine-step sink, stays fine); report antimatter by level and in d3-d4 wins; the band stays.

## The anchor check (give / get, before any sim)

Pay = credits + 2.67 × (keep + play insight) + 18 an antimatter; a tier-2 keep is 3 insight (8).

| Card | Was | Now | Pay | Get at the anchors | × |
|---|---|---|---|---|---|
| Orbital Mass Driver (building, Industry) | 15 cr, 1 chip, 3 energy, 2 ins: 9 credit + 2 chip income (1.13-1.50, kept 11-14%, played 0.01-0.02 a game) | 4 cr, 1 ins, 1 am: 13 credit + 2 chip income | 4 + 8 + 2.67 + 18 = 32.67 | 13 × 4 + 2 × 13 = 78 | **2.39** |
| Fusion Commons (event, Diplomacy) | 2 am, 1 ins: 6 energy, Unity 3, draw 1 (0.63) | 1 am, 1 ins: 12 energy, Unity 12 | 8 + 2.67 + 18 = 28.67 | 36 + 12 × 2.67 = 68 | **2.37** |
| Singularity Exploit (program, Cyber) | 3 cr, 1 ins, 1 am: botnet + discard a project | 0 cr, 0 ins, 1 am | 8 + 18 = 26 | botnet 10 + the leftmost project's compute (cost a board × 2 countries; only the leftmost holds any): half full (24) 32 → 1.62; three quarters (36) 48 → **2.23** | 1.62-2.23 |
| Kinetic Interdiction (Security; was an event, now a building) | event, 0 cr, 1 am: 2 troops + 6 agents on or next to it, Discord 1 (about 0.9) | building, 4 cr; Action: 1 am → 10 agents in the target's country | 18 a use (4 cr + keep 8 once) | at the p90 country 33.3 → **1.85** a use; median (6) 1.11; with the build, 3 uses 66 for 100 (1.52) | 1.11-1.85 |
| Containment Reactor (tier 1, Matt's numbers) | Action: 1 am → 4 energy + 1 Unity (0.81) | 1 am → 6 energy + 1 Unity | 18 a use (5 cr + keep 2.67 once) | 18 + 2.67 = 20.67 | **1.15** a use |

Two sinks sit near the ceiling (Orbital Mass Driver 2.39, Fusion Commons 2.37). **Kinetic Interdiction stops at
1.85 a use** (message 13138 asked for 2.0+ or the reason): as an event the floor is 26 (keep + antimatter) and a
removal is capped by the board (card-guidelines 22: 10 agents and 3 troops are the p90 country; more is a disguised
"remove all"), so 1.67; a cordon rider (1.82) set at 5.8 pt. As a building the keep is paid once and each antimatter
buys a sweep: with 3 troops too, 2.41 a use (screened: s287-ki, below), but the Action set at 6.3 pt; agents alone
fit. **Singularity Exploit reaches 2.2 only when held** for a full project (only the leftmost project holds compute;
a third effect breaks ruling 8): that is the plan a skilled player makes around it.

The agents' table (values.json) nets, reasoned − printed − keep 6.93: Orbital Mass Driver +42.9, Fusion Commons
+40.4, Containment Reactor +0.5 (tier-1 keep), **Singularity Exploit −13.6** (the table prices a discard at 5.33 and
a botnet low), **Kinetic Interdiction as the building −10.3** (an agent removed in a country at 1.33: 10 read 13.3
against the Action's antimatter 18; it was +0.6 as the event). Both `overrides` rows were screened on and off: only
Kinetic Interdiction's ships (+20, values.json).

## Screens (dev seeds 36800-36831, 32 a cell, three pairs, d0-d4; paired against r137's cards on the same build)

Specs: [make_specs.py](make_specs.py). Reads: `read_lib.py --base old`, [am_cards.py](am_cards.py) (antimatter per
card, level and won game).

| Arm | Judge gap ± SE | Greedy | Judge spent share | Note |
|---|---|---|---|---|
| r137 cards (`old`) | base, 70.2% | base, 66.5% | 46% | Containment Reactor never kept (0.2%) |
| Matt's Containment Reactor alone (`cr`, the asked-for numbers) | -0.4 ± 1.1 | -0.2 ± 1.0 | 48% | kept 0.2% → 20%, used 0.08 a judge seat-game |
| Sinks, Kinetic Interdiction the event (`base`) | +2.7 ± 1.8 | +0.6 ± 1.8 | 47% | Orbital Mass Driver plays 0.03 → 0.06 a game |
| + Singularity Exploit `overrides` +20 (`s287-ov`) | +2.7 ± 1.8 | +0.6 ± 1.8 | 47% | its plays 0.06 → 0.06: not shipped |
| Kinetic Interdiction a building, troops too (`ki`) | +2.3 ± 1.9 | +0.4 ± 1.9 | 48% | 2.41 a use; set at 6.3 pt |
| Kinetic Interdiction a building, agents alone (`fin`) | +1.7 ± 1.8 | +0.4 ± 1.8 | 45% | kept 0.2%: the table reads it at 0.67 |
| `fin` + its `overrides` +20 (shipped) | +1.5 ± 1.9 | +0.6 ± 1.9 | **50%** | kept 33%, its Action 0.17 a judge seat-game |

No arm moves wins measurably; the sinks spend more only where the agents read them (Containment Reactor, Kinetic
Interdiction with its row). Fusion Commons (2.37) and Orbital Mass Driver (2.39) are kept 15-24% of games and played
about half as often: their needs (Science 2; Industry 3) and the antimatter in hand bind, not their value.

Cryptojacking Campaign (tier 1, 15 cr, 2 energy: +5 credit income) has no antimatter fit; at r137 it is kept in 34%
of games and played 0.13-0.14 a game (no longer "never played"), so it is left alone. The read's design_check no
longer lists it kept-not-played.

## Fresh read (seeds 50220-50267, 48 a cell, 1,440 games a side; r137 farmed `--at` main 35c98d12)

Full table: [read.md](read.md) (`fresh_read.py`). Paired (`read_lib.py`): judge **+1.7 ± 1.4**, greedy +0.3 ± 1.4.

| Agent | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| Judge win % r137 → r138 (gap ± SE) | 94.4 → **95.1** (+0.7 ± 1.8) | 87.5 → **86.1** (-1.4 ± 3.1) | 81.9 → **81.2** (-0.7 ± 3.3) | 52.8 → **57.6** (+4.9 ± 3.6) | 29.9 → **34.7** (+4.9 ± 3.6) |
| Soft band | 90+ met | 80-95 met | 75-90 met | 55-70 **met** (was 2.2 under) | none |
| Greedy | 93.8 → 93.8 | 84.0 → 86.1 | 75.7 → 73.6 | 49.3 → 47.2 | 17.4 → 20.8 (+3.5 ± 2.9) |
| Judge rounds | 8.33 → 8.38 | 9.67 → 9.64 | 9.38 → 9.41 | 9.15 → 8.98 | 8.70 → 8.45 |

## Antimatter (fresh read, a seat-game, r137 → r138)

[am-fresh.txt](am-fresh.txt) (`am_cards.py`).

| | Judge | Greedy |
|---|---|---|
| Gained → spent (share) | 3.26 → 1.33 (41%) / 3.35 → 1.46 (**44%**) | 2.41 → 1.22 (50%) / 2.39 → 1.22 (51%) |
| d3-d4 wins that spent any | 81% → **85%** | 75% → 78% |
| Share of their antimatter d3-d4 wins spent | 44% → **56%** | 50% → 57% |
| Share spent by level, d0 / d1 / d2 / d3 / d4 | 37 / 43 / 44 / 41 / 38% → 39 / 43 / 45 / 46 / 48% | 44 / 52 / 54 / 51 / 53% → 43 / 53 / 55 / 50 / 53% |
| Share spent in wins, d0 → d4 | 36 / 44 / 46 / 42 / 50% → 40 / 46 / 43 / 51 / **66%** | 43 / 51 / 52 / 50 / 49% → 42 / 52 / 56 / 55 / 62% |
| Top spends | Orbital Relay Launcher 0.16, Hostile Buyout 0.14 | → **Kinetic Interdiction 0.16**, Orbital Relay Launcher 0.14, Hostile Buyout 0.14, Containment Reactor 0.11 |

Amendment 13123's shape: the share rises with difficulty and in hard wins (d4 judge wins 50% → 66%) and stays low
at d0-d1, where the game is won without it. design_check's pooled antimatter band (50%) still warns (47% pooled, both
agents): **the band reads against Matt's goal**, since it counts d0-d1 seats that need no antimatter; not changed.

## Per card (fresh read; kept = share of games, plays a game; r137 → r138, judge / greedy)

| Sink | Give / get | Kept judge | Plays judge | Kept greedy | Plays greedy |
|---|---|---|---|---|---|
| Kinetic Interdiction (building, Action) | 1.85 a use | 23% → 29% | 0.14 → 0.24 (uses 0.16 a seat-game) | 22% → 27% | 0.12 → 0.21 |
| Orbital Mass Driver | 2.39 | 16% → 18% | 0.02 → 0.07 | 20% → 22% | 0.04 → 0.09 |
| Fusion Commons | 2.37 | 18% → 18% | 0.09 → 0.12 | 18% → 16% | 0.08 → 0.11 |
| Singularity Exploit | 1.62-2.23 | 22% → 21% | 0.13 → 0.12 | 25% → 25% | 0.14 → 0.13 |
| Containment Reactor | 1.15 a use | 0% → 23% | 0.05 → 0.28 (uses 0.11) | 0% → 25% | 0.02 → 0.25 |

Annihilation Lance, which the review saw as near-dominated by the old Kinetic event, holds (kept 15-17%, 0.06-0.07).

## Checks

`cards:fit` 0 under 7.5 pt; loop smoke (16 seeds a cell) on the first and the final build: no timeouts, no card at
the flag; design_check on the read:
[design-check.txt](design-check.txt) (Make antimatter's standard-over-anchor exempt, Matt 287; the warnings are the
usage ones r137's side shows, antimatter spent 47% among them). `per_decision.py` r137 → r138 (6 games, best of 2;
ratios only): the judge 1.02× a decision before the overrides row, 1.00× after; greedy 0.91-0.92×. `speed.py check` gives
no verdict across a revision.

## For new players (r138)

- Antimatter is worth saving for: each tier-2 card that spends it pays big, and a few pay double what they cost.
- **Kinetic Interdiction** is now a building: once a round, pay 1 antimatter to remove up to 10 Agents in the
  target's country.
- **Orbital Mass Driver** costs 4 credits, 1 insight and 1 antimatter, and raises your income by 13 credits and 2
  chips a round.
- **Fusion Commons** costs 1 antimatter (was 2) and gives 12 energy and 12 Unity: two Unity tips at two players.
- **Singularity Exploit** costs only its antimatter: hold it until the leftmost project is nearly full.
- **Containment Reactor**'s Action turns 1 antimatter into 6 energy (was 4) and 1 Unity.
- Rules that resisted: Kinetic Interdiction stops at 1.85× a use (its troop clause didn't fit the card), and
  Singularity Exploit pays 2× only when held for a full project.
