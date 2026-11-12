# 195: the project overhaul (r85)

`RULES_REVISION` 84 → 85, bumped from the constant (the prompt said 85 too). Built on main e095ed1 (192). Every
number is a candidate. Matt's direction (2026-11-11): projects are the adversary's small, visible plans on the board.

## The deck

22 projects (was 20), written by [`projects_195.py`](projects_195.py) into `drafts/adversary/projects.json`; the design
check, every card's give / get at `configs/agents/values.json`, its harm, when it matters and who plays it, is
[`design-check.md`](design-check.md) (written before the first screen).

- **Cut** (taxes and troops with no factory): Raiding Parties, Purge the Unaligned, Market Crash, Capital Flight, Grid
  Capture (each "pays or discards", or an income cut), Armored Columns, Arms Race, Declaration of War (Musters and
  drones on no adversary piece).
- **Kept from 151:** War Footing, Shadow Factories, Recruitment Drive, Infiltration Drive (18 → 14 a player),
  Disinformation, Rolling Blackouts, False Flag, Grand Offensive, Propaganda Network; **Data Heist** as the one tax
  ("cut or nearly cut"): it is the only project source of tokens stripped and credits lost, and cutting it opened two
  ledger gaps.
- **New or rewritten:** Factory Garrisons (ongoing: every adversary factory musters 1 troop a round), War Plants
  (builds an adversary factory, then every adversary factory musters 1), Asset Seizure and Station Coup (a country's
  neutral factory / station with the most agents on or next to it goes to the adversary; none neutral: a player's turns
  neutral), Hostile Takeover (Rally 2 factories, Corrupt factories), Botnet Swarm (a botnet spreads to the
  lowest-numbered site next to one), Botnet Conscripts (the lowest-numbered botnet becomes 2 troops), Backdoor
  Research (2 adversary cubes on each open megaproject), Night Raid (Corrupt nodes, then a raid), Mob Rule (Rally 2
  cities, then a raid), Sabotage Teams (strip hardening, then a raid), Accelerated Timetable (the top event card is
  discarded unseen).
- **Raid:** in each country, the node hex holding the most agents (at least 1; ties: lowest-numbered) loses its node,
  protections applying. Targets are read off the board when the project resolves; no marker.
- **Ops** (Team slot only; arms in `effects/team.rs`, `face.rs`, `harms.rs`, `value.rs`, `values.json`):
  `botnet_troops`, `spread_botnets`, `take_site`, `factory_troops`, `raid_node`, `strip_harden`, `clock`. Their faces
  are the card texts (projects print hand-written text, as before).
- **check.py:** a project-only harm (`ONE_SOURCE`: protections stripped, the clock sped up, adversary cubes on
  megaprojects) passes the drafts' two-source rule; `docs/harms.md` still lists any thin one as a gap.

## The harms ledger

Closed: **adversary cubes on megaprojects** (Backdoor Research beside corruption's stations), **protections stripped**
(Sabotage Teams, and a new `RULE_HARMS` row for the march: a strike absorbed spends the hardening token or ambassador,
r75, which the ledger never listed), **node destroyed, tier-2 threats over half** (three raiding projects: tier-2 6 of
12 printed node destructions, exactly half). Gained a source, still a gap: **the clock sped up** (Accelerated
Timetable only). Still open: antimatter lost, pieces pushed or reach cut, a megaproject or converter denied, the card
market disrupted.

## Ladder (fresh seeds 46000-46143, paired against r84 on the same seeds)

| d | r85 strategist | r84 | gap ± SE | band | r85 greedy | r84 greedy | gap ± SE |
|---|---|---|---|---|---|---|---|
| d0 | 83.1 | 79.9 | +3.2 ± 2.5 | 90-100 unmet | 84.0 | 86.1 | −2.1 ± 4.0 |
| d1 | 81.7 | 78.2 | +3.5 ± 2.6 | 70-80 unmet (over by 1.7) | 75.0 | 74.3 | +0.7 ± 4.6 |
| d2 | 73.1 | 64.1 | +9.0 ± 2.7 | 55-65 unmet (over by 8.1) | 75.0 | 69.4 | +5.6 ± 4.5 |
| d3 | 50.2 | 41.7 | +8.6 ± 2.7 | 40-50 unmet (over by 0.2) | 52.1 | 41.7 | +10.4 ± 4.7 |

Strategist 432 games a level (three pairs × 144 seeds), greedy 144 (48 seeds), on the final build. The first read
(frozen, before the review's `strip_harden` fix) replayed on the final build: 50 of 1,728 strategist games and 21 of
576 greedy games differ (`same_games.py`: a farmed read has no games.csv for `same_results.py`); the frozen read was
83.1 / 81.5 / 73.1 / 50.5 (greedy d3 50.7). Only greedy d3 moved by more than 10 (+10.4 ± 4.7); its driver is below.
No dial moved (the balance pass calibrates). Tables: [`ladder-strategist.md`](ladder-strategist.md) (with the standing
measures), [`ladder-greedy.md`](ladder-greedy.md).

**What drives the gap: the cut taxes.** Screen on the dev seeds 36800-36831 (96 games a cell, d2-d3,
[`screen_spec.py`](screen_spec.py), [`screen-drivers.md`](screen-drivers.md)): r84's five tax cards put back onto
five r85 ids read **−14.6 ± 4.9 at d2, −9.4 ± 5.8 at d3**; r84's three untied troop cards put back onto three ids read
−2.1 ± 4.2 and +5.2 ± 4.5 (nothing). Each lever replaces new cards' effects, so it reads "the old part instead of
those new cards". Losses move from the Discord deck (d2 121 → 71, d3 177 → 106 of 432) to nodes (2 → 13, 4 → 20) and,
at d3, the clock (71 → 88).

## Each project

[`projects-strategist.md`](projects-strategist.md), per difficulty ([`projects_read.py`](projects_read.py)): revealed,
completes a game and as a share of reveals, compute drained from it a game, token moves at its completion (an ongoing
one's per round). At d2 every project completes 0.49-0.61 times a game (71-86% of reveals); the players drain
0.45-1.87 compute a game from each (most from Factory Garrisons, Propaganda Network and Accelerated Timetable; least
from Disinformation and Sabotage Teams). Token moves a completion: 0 (Data Heist, Disinformation, Rolling Blackouts,
False Flag, Backdoor Research's cubes go on a card, Accelerated Timetable), 1.5-4 (Sabotage Teams, Botnet Swarm,
Shadow Factories, Asset Seizure, Station Coup, War Footing), 4.5-5.6 (Botnet Conscripts, War Plants, Mob Rule), 7-10
(Hostile Takeover, Infiltration Drive, Night Raid, Grand Offensive: a Corrupt line's agents moving in and back to the
supply); Factory Garrisons about 4 a round over 1.7 rounds. At d0 projects barely matter (about 1 completion a game,
as at r84).

Raids and the rest a game at d2 / d3: node raids 1.78 / 1.72, sites taken 2.07 / 2.00 (turned neutral 0.01 / 0.06),
botnets spread 1.15 / 1.26, event cards discarded by the clock 0.57 / 0.59, hardening stripped 0.32 / 0.38.

## Troops on the map

[`troops-strategist.md`](troops-strategist.md) ([`troops_read.py`](troops_read.py)): by round, r85 against r84, the
same within about 1 a round to round 6; later r85 has more at d3 (r7 49 vs 41). By source at d2: projects' Musters
(Armored Columns, Declaration of War) are gone, factory troops 8.4 and botnet troops 1.7 a game take their place,
factory output 13.3 (11.2). **The early troops are the event cards' Musters** (12 a game at d0, 19 at d2-d3), not
projects: "troops uncommon until late" needs the event deck (Needs a decision).

## Speed

`speed.py check` gives no verdict across the revision (greedy 1.52×, planner 1.40×, engine 0.57× the r84 baseline per
game). `per_decision.py` r84 → r85, c2 d2, 6 games: greedy 1.41× a decision on **1.60× the options a decision** (ms
an option 0.080 → 0.068: the code is no slower; the players hold more without the taxes, so each decision has more
to weigh), strategist 1.05× a decision on 1.15× the options. Kept: the baseline is updated at r85 (`--update`).

## Seeds

Fresh: 46000-46143 (strategist 144 seeds, greedy the first 48), clear of 194 running beside; docs/experiments.md's
ledger updated. Dev: 36800-36831 (the
driver screen; the engine smoke at 16 seeds, d0-d3, engine and strategist: no timeouts, every project completes).

## The review

A three-lens review (ops, invariants, docs) with adversarial verification found: `strip_harden` took a Science card's
node shield on a bare node hex (fixed: a facility's hardening only, `board::hardenable`; this changed play, hence the
replay above); the clock's "unseen" discard was digital-only, since the discard pile is public (printed face up now);
stale lines in the rulebook (projects muster, add drones, never tax) and doc comments (fixed); a botnet's troops ignore
Wall (kept and printed: made on its own hex, as a factory's output). Refuted: game-over mid-loop, a peek leak, guard
over-tally (the flip paths' convention).
