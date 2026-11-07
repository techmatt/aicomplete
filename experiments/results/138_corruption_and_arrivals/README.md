# 138: Corrupt X and bounded arrivals (r57)

Rules revision 57, bumped from the constant (56 to 57), as the prompt said. Rules: Corrupt X, natural corruption,
creation lines, site creation, land corruption, botnets and the new-player summary in
[adversary.md](../../../docs/rules/adversary.md); the threat dials in [threats.md](../../../docs/rules/threats.md).
Where every old job went: [jobs.md](jobs.md). Tables: [read_r57_vs_r56.md](read_r57_vs_r56.md) (137's `read137.py
--135` on both dirs), [read_sum138.txt](read_sum138.txt) ([sum138.py](sum138.py): corruptions by site type,
ambassadors created by source, threats by tier). Specs: `experiments/read-138.json`, `screen-138-{a,b,c,d}.json`,
`smoke-138.json`, `smoke-138-agents.json`; the r56 side ran `out/r56src/read-138-r56.json` (main f68c760's tree).
The content rewrite is `scripts/content/arrivals_138.py` (rerunnable over the drafts, then `load_adversary.py`).

## Where things stood

135 (r54), 136 (r55) and 137 (r56) ended done, and the code matched 137's write-up at f68c760: one site model, Claim
4/2/2, armor, war every round, board income +6; 137's fresh read 73.6 / 62.5 / 49.3% (d0-d2), 7.05 rounds at d2,
corruption 5.8 nodes a game at d2 and the fatal blow in 76% of last-node losses. Nothing this item needs was missing:
137 printed the empty station sites and the armor rule that Corrupt X and site creation use.

## The build (each number a candidate)

- **Corrupt X** (`Op::Corrupt { site, cut }`, phase `adversary.corrupt`, which replaced `adversary.spread`): in each
  country, each site of the type not yet the adversary's, lowest-numbered first, that can gather n (on it plus next to
  it in its country) pulls just enough in, lowest-numbered hexes first, and corrupts; every ambassador on it returns.
  One forecaster (`plan::corrupt_plan`) is the rule and the agents' read. n: city `corrupt_at` 4, station
  `corrupt_station_at` 3, factory `corrupt_factory_at` 3, land `corrupt_land_at` 3; `corrupt_cut` 1/1/1/0 from
  `corrupt_cut_at` 2 (the difficulty line). Every event card prints one or two Corrupt lines; threats (Overrun,
  Propaganda Ministry), projects (Infiltration Drive, Grand Offensive) and Offensive print them too.
- **Natural corruption** (phase `adversary.natural`): any city, station or factory already holding its n corrupts.
- **Node loss:** a corruption destroys only the node on its own hex, through `destroy_piece`, so the owner's
  ambassador there is armor; `board.corruption_reach` is gone. Sanctuary (`corruption_shield`): corruption never
  destroys your nodes.
- **Land corruption** (`LAND_CORRUPTION` hex token; Blitzkrieg, Naval Blockade): no node may go there; 1 Discord a
  round each (`blight_discord`).
- **Bounded arrivals** (`Op::Create { count, at, if, sites }`): "create N per board on the lowest-numbered X
  [condition]". Event cards: 1 a site in stage I on 1-2 sites, more in II and III; the board's line (new content field
  `build`): 2 on the lowest-numbered corrupted city each build (`build_bonus` adds at d3). Threats create in their own
  country. Tallied `adversary.created.<source>`.
- **Site creation** (`Op::CreateSite`): Factory Drive and Blight open a neutral factory on each board's lowest free
  zone; Long Patrols and Civil Defense (Unity) a neutral station on its empty station site.
- **Botnets:** setup and the three printed `place_botnet` threats only; botnet hexes are a creation target.
- **Agents:** the eval's corruption pressure reads the coming card's Corrupt line (`plan::next_corrupt_n`) and, when
  one gathers, counts the ambassadors next door; new term `w.site_pressure` 1 (a player's station or factory filling
  toward its n). Values: `create` -2 an ambassador a site a country, `create_site` 3, `corrupt` -10. The site-pressure
  term on / off (dev seeds 36800-36815, B+P, 16 games): 81.2 / 81.2% at d0, 81.2 / 75.0% at d2: kept.

## Calibration (dev seeds 36800-36815, 48 games a cell: the three pairs)

| Cell | d0 | d1 | d2 | median rounds d0/d1/d2 | tier 3 in d2 games | d2 nodes: threats / corruption |
|---|---|---|---|---|---|---|
| r57 as built | 79.2 | 87.5 | 89.6 | 9 / 9 / 9 | 67% | 0 / 2.31 |
| T: `threat_nodes` 1,1,2 | 79.2 | | 89.6 | 9 / - / 9 | 63% | 0.63 / 2.31 |
| S: `stage_cards` 4,4,3 | 81.2 | 91.7 | 87.5 | 9 / 9 / 9 | 98% | 0 / 2.69 |
| S+T | 81.2 | | 87.5 | 9 / - / 9 | 98% | 0.46 / 2.67 |
| S + `counter_pct` 150 | 87.5 | 83.3 | 58.3 | 9 / 9 / 9 | 83% | 0.10 / 2.92 |
| **S + 150 + T (frozen)** | 87.5 | 77.1 | 62.5 | 9 / 9 / 9 | 69% | 1.63 / 2.12 |
| S + 200 + T | 87.5 | 77.1 | 37.5 | 9 / 9 / 7.5 | 50% | 3.02 / 1.88 |
| S + 300/200/150 + T | 85.4 | 39.6 | 22.9 | 9 / 8 / 5 | 29% | 3.62 / 1.06 |
| S + 300/175/125 + T 1,2,2 | 83.3 | 41.7 | 25.0 | 9 / 8 / 6 | 33% | 3.91 / 1.19 |
| S + 250/250/150 + T | 85.4 | 54.2 | 20.8 | 9 / 8 / 5 | 33% | 3.62 / 1.21 |

**d0 doesn't move with threat levers:** d0 deals one tier-1 threat a country a round and the table pays it even at 3x;
its wins are Unity-deck wins around round 9. Every lever that put d1 at 55% put d2 below 25% with a median of 5-6
rounds. The frozen cell is the one that keeps every level's median at 9 and tier 3 in most d2 games.

**After the read, the d0-d1 miss** (screen D, same dev seeds, on the frozen r57, a factorial; question 2 to Matt):

| Cell | d0 | d1 | d2 | median rounds d0/d1/d2 |
|---|---|---|---|---|
| r57 frozen | 87.5 | 77.1 | 62.5 | 9 / 9 / 9 |
| U4: Unity tips at 4 a player (alignment) | 91.7 | 77.1 | 41.7 | 9 / 9 / 8 |
| I3: board credits -3 (21 / 19 / 17; data only, it pulls 137's lever back) | 87.5 | 70.8 | 39.6 | 9 / 9 / 7.5 |
| U4 + I3 | 81.2 | 56.2 | 31.2 | 10 / 9 / 7 |

Nothing reaches d0 70%: U4 shifts d0's wins from the Unity deck to the purge without losing any; only both together
put d1 at the target, at the cost of d2 (31%, median 7). Not adopted (the default of question 2: U4 only if it alone
moved d0-d1). **What d0 lacks** is pressure that a well-funded table can't buy off: d0 deals 21 threats a game and
counters 18, and corruption, now confined to its own hex, takes 1.6 nodes a game. The levers left are the player side
(137's income +6 was sized against r56's corruption, 5.8 nodes a game at d2, which r57 no longer has) or more threats
at d0 (a second pull on stage-I cards).

**Then the threat harm** (screen E, same dev seeds, on the frozen r57): `threat_nodes` 1,2,3 (H123) read 87.5 / 77.1
/ 52.1% (median 9 / 9 / 8), d2 nodes to uncountered threats 2.00 against corruption 1.69; 2,2,3 read 87.5 / 77.1 /
50.0 (2.39 against 1.85). **H123 adopted** after its fresh confirm (below): r57's final numbers are the read's frozen
cells with `threat_nodes` 1,2,3.

## The read (fresh seeds 36536-36583, 48 a cell, the three pairs; r56 = main f68c760 on the same seeds)

| | d0 | d1 | d2 |
|---|---|---|---|
| strategist r57 (frozen) | 89.6 ±2.5 | 88.2 ±2.7 | 63.9 ±4.0 |
| strategist r57 as built | 82.6 ±3.2 | 90.3 ±2.5 | 88.9 ±2.6 |
| strategist r56 | 73.6 ±3.7 | 61.8 ±4.0 | 52.1 ±4.2 |
| paired r57 - r56 | +16.0 ±4.6 | +26.4 ±4.9 | +11.8 ±5.1 |
| paired as built - r56 | +9.0 ±4.6 | +28.5 ±4.8 | +36.8 ±4.7 |
| rounds r57: mean, median, p10-p90 | 9.01, 9, 7-11 | 8.81, 9, 8-10 | 8.15, 9, 5-10 |
| rounds r56: mean, median | 8.47, 8.5 | 7.90, 8 | 7.44, 8 |
| paired rounds r57 - r56 | +0.54 ±0.17 | +0.92 ±0.18 | +0.72 ±0.21 |

**How games end** (r57 / r56, of 144): d0 Unity deck 93 / 86, purge 36 / 20, round cap 12 / 10, node 3 / 28; d1
105 / 73, 22 / 16, 3 / 4, 14 / 51; d2 88 / 62, 4 / 13, 3 / 6, 49 / 63.

**Nodes lost a game by cause** (r57 / r56):

| | corruption | uncountered threats (t1 / t2 / t3) | spent as counters | Discord cards | twists | war | projects |
|---|---|---|---|---|---|---|---|
| d0 | 1.57 / 4.61 | 0.39 (0.24 / 0.14 / 0.01) / 0 | 0.27 / 0.13 | 0.19 / 0.22 | 0.49 / 0.43 | 0.35 / 0.45 | 0.15 / 0.21 |
| d1 | 1.48 / 5.26 | 1.13 (0.52 / 0.53 / 0.08) / 0 | 0.62 / 0.22 | 0.72 / 0.44 | 0.54 / 0.45 | 0.12 / 0.46 | 0.17 / 0.31 |
| d2 | 1.78 / 4.97 | 1.49 (0.42 / 0.85 / 0.22) / 0.04 | 0.50 / 0.24 | 1.53 / 0.49 | 0.61 / 0.60 | 0.17 / 0.44 | 0.22 / 0.31 |

**Threats a game** (r57; dealt / countered / uncountered): d0 21.2 / 18.1 / 1.03, d1 30.1 / 24.9 / 2.41, d2 29.5 /
23.9 / 3.04 (r56: 18.7 / 15.9 / 1.10, 25.6 / 21.4 / 1.84, 24.9 / 20.1 / 1.84). **By tier, dealt (uncountered):**
d0 11.1 (0.76) / 7.8 (0.25) / 2.3 (0.02); d1 15.8 (1.51) / 11.4 (0.81) / 3.0 (0.09); d2 13.3 (1.55) / 11.5 (1.33) /
4.7 (0.16). **Tier 3 appears** in 64% / 69% / 72% of games (as built: 44 / 29 / 67%).

**Corruptions a game by site type** (r57; city / station / factory / land; natural of them): d0 2.84 / 2.56 / 2.85 /
0.27 (0.08), d1 3.04 / 2.81 / 3.27 / 0.35 (0.15), d2 3.89 / 3.19 / 3.88 / 0.81 (0.48). r56 corrupted 3.1-3.7 cities a
game (its only kind), but each took the nodes around it.

**Adversary ambassadors created a game by source** (r57; event / board / Discord / project / threat): d0 37.0 / 27.0 /
1.1 / 0.3 / 0.2 (65.6), d1 38.8 / 29.1 / 1.9 / 0.5 / 0.4 (70.7), d2 38.0 / 28.6 / 2.5 / 0.6 / 0.5 (70.2); about 28 a
game are pulled in by Corrupt lines. r56's arrivals alone were 37-41 a game (its city builds, infiltrate and escalation
not tallied); r56 formed 7.5-8.3 botnets a game, r57 0.01-0.19 (the printed threats). Sites created: 1.6 stations and
0.5 factories a game.

**Peak holdings** (d2, Builder / Protector / Researcher, r57 vs r56): credits 91 / 81 / 98 vs 99 / 84 / 98; insight
12.6 / 23.5 / 28.6 vs 13.9 / 17.1 / 28.5; energy 9.7 / 10.1 / 10.0 vs 12.9 / 12.6 / 11.9; chips 10.8 / 4.9 / 5.1 vs
11.6 / 5.6 / 5.3; ambassadors 4.6 / 6.6 / 6.8 vs 5.1 / 7.1 / 6.0. Every cell is in read_r57_vs_r56.md.

**The final r57** (H123 confirmed on the read's fresh seeds, `experiments/read-138-h123.json`, paired with the
frozen cells and with r56 by [paired.py](paired.py)):

| | d0 | d1 | d2 |
|---|---|---|---|
| strategist r57 final (threat_nodes 1,2,3) | 88.2 | 84.7 | 60.4 |
| paired final - frozen | -1.4 ±1.0 | -3.5 ±2.1 | -3.5 ±2.1 |
| paired final - r56 | +14.6 ±4.7 | +22.9 ±5.2 | +8.3 ±5.1 |
| rounds final: mean, median | 8.99, 9 | 8.67, 9 | 7.96, 9 |
| paired rounds final - r56 | +0.53 ±0.17 | +0.78 ±0.18 | +0.52 ±0.22 |
| nodes a game: uncountered threats / corruption | 0.50 / 1.56 | 1.49 / 1.44 | 1.81 / 1.64 |
| tier 3 in games | 62% | 64% | 68% |

How final games end (of 144): d0 Unity deck 91, purge 36, round cap 13, node 4; d1 101 / 21 / 4 / 18; d2 83 / 4 / 3
/ 54. The tables above (corruptions, creation, peaks) are the frozen cells; the final's are within a few percent
(`sum138.py out/readH`).

## The death audit, 136-138 ([death_audit.md](death_audit.md); 136's `death_audit.py` on r57's fresh d2 games)

49 last-node losses at d2 (r56's 137 audit: 72 at d2, 125 at d1-d2). **The fatal blow:** Discord cards 34 (22 alone,
11 with a threat, 1 with production; Offensive 17, Firestorm 7, Two Fronts 6), threats 18 (7 alone), Border War 5,
**corruption 3**. Across the three audits corruption's share of last-node losses goes 55% (136, r54) to 76% (137, r56)
to 6% (138, r57): its cut to the corrupted hex itself did what the 136 audit asked. The killer now is the Discord
deck's node steps, Offensive most of all, whose target ("the node hex with the most adversary ambassadors and troops")
grew with the creation lines. Seat 0 still dies most (Builder 28 of 49) but no longer from one city's blast: Horizon
#14 is no longer the cause (corruption took 3). A node-safe line (extra placements affordable against nodes needed)
existed in 3 of 49 (137: 61 of 95 corruption deaths): the deaths are now the rules' hits, not the agents' placements.
**Verdict:** the map cascade is gone; what remains is the Discord deck's node hits and uncountered threats, the
intended tests.

## Done when, against the read (the final r57)

- Corrupt X and bounded arrivals replace the spread; a full game runs at 1-4 countries (`smoke-138`, greedy and
  strategist, d0 and d2, no timeout or panic). **Met.**
- Median d2 game length 9 (mean 7.96). **Met.** Tier-3 threats appear in 68% of d2 games. **Met.**
- Uncountered threats take more nodes a game than corruption: d2 1.81 against 1.64, d1 1.49 against 1.44. **Met at d1
  and d2**; at d0 0.50 against 1.56 (d0 rarely leaves a threat uncountered).
- The d0-d1 strategist targets (about 70 / 55): 88.2 / 84.7. **Not met**: no adversary lever screened moves d0
  (screens B-E); screen D's player-side numbers are for Matt.

## Not done

- `web/`: `npm run check`, `cards:fit` and the e2e were not run (no node_modules in the worktree); the engine types
  and panels drop `escalation`, the e2e configs drop the retired params.
- `speed.py`: no timing read (the box was loaded by this item's runs all session; Matt's message 5789).
- Claim stays 2 / 2 for stations and factories against Corrupt X's 3 / 3 (question 1 to Matt; default kept).
- Shadow Factories' fallback (2 ambassadors on a factory, players choose) is the one placement left outside the
  creation shape.
