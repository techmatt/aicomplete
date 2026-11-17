# 258: card follow-ups after 257

Base: main 11f8539 (257's shipped judge on r117). **Rules change: r118, then r119** (bumped from the constant: 117 to
118 for the pass; 118 to 119 for amendment 11877's second pass, after r118's fresh read). **The judge changes too:**
`w.sink` 0.5 (amendment 11877). Matt's ruling: unplayed cards get stronger, not cheaper. Every card edit is one
overlay, [cards.overlay](cards.overlay).

Files: r118: [screen.md](screen.md), [screen-use.md](screen-use.md) (dev screen), [read.md](read.md),
[read-use.md](read-use.md), [design-check.txt](design-check.txt) (fresh read), [groups.md](groups.md) (group probe).
r119: [sink.md](sink.md), [sink-use.md](sink-use.md) (`w.sink` re-screen), [confirm.md](confirm.md),
[confirm-use.md](confirm-use.md), [confirm-sink.md](confirm-sink.md), [design-check-confirm.txt](design-check-confirm.txt)
(the confirming read). Scripts: `merge.py` (r117's native run and a farmed run as one read), `agent_arms.py` (agent
options as arms read_lib pairs), `use258.py` (256's Use tables on these cards), `arms.py` (the probe's arms),
`jobs_rows.py` (design_check_jobs.json rows), `pricing_notes.py`. Specs in `experiments/farm/`: `s258-screen`,
`f258-read`, `f258-oracle`, `s258-groups`, `s258-sink`, `f258-confirm`, `f258-oracle2`. r117's side ran natively
from `git archive 11f8539` on the same specs.

## The cards (give / get)

Anchors: pay = credits + 4 × (keep + play insight) + other costs (energy 4.5, cube 6, chip 6, antimatter 18); keep 4
tier 1, 12 tier 2; an Action at 4 uses; a node 8, its track step paid on top as by every route (256). values.json's
reading per card is its line in `docs/card-pricing-notes.json` (the effect ledger's printed and reasoned).

| Card | Change (rev) | anchors: pay → get | Where it matters; loops and caps |
|---|---|---|---|
| Mesh Repeater Network (t1 bldg) | Action free: 1 node (was 1 energy); 17 cr (was 6) (r118) | 21 → 32 (1.52) | From round ~4.5: a node a round beside Expand, energy (binding) no longer split. Once a round. |
| Research Reactor (t1 bldg) | Action 1 cube → 3 energy (was 2) (r118) | 36 → 54 (1.5) | Once cubes flow; nothing makes cubes from energy at 1:1 or better: no loop. |
| Survey Drone Fleet (t2 bldg) | needs 1 Science (was 2), 2 cubes when built (r118); Action also 2 energy (r119) | 48 → 112 (2.33) | Built earlier, with its fuel; the 2 energy pays its nodes' track steps (257: nodes had nowhere to go late). Once a round. |
| Orbital Relay Launcher (t2 bldg) | Action 1 antimatter → 3 nodes (was 2) and 3 energy (r118) | 96 → 150 (1.56) | A plant's 2 energy → 1 antimatter, then Orbital: +1 energy and 3 nodes a round, each once a round. |
| Antiproton Foundry (t2 bldg) | Action 1 chip → 1 antimatter **and 2 energy**; 1 antimatter when built; 12 cr (was 10) (r119; r118 had 2 when built) | 52 → 126 (2.42) | The chip-fed plant; with Orbital, 1 chip → 3 nodes and 5 energy a round across two Actions, each once a round. |
| Positron Storage Ring (t2 bldg) | Action 2 energy → 1 antimatter **and draw 1** (r119) | 75 → 118 (1.57) | 2 energy → 1 antimatter → Orbital's 3 energy: +1 energy a round, capped by both Actions. |
| Microgrid Lab Compact (t2 bldg) | claims a station or factory when built too (r118) | 66 → 72 (1.09) | Capped by the neutral sites in reach. |
| Field Assay Team (t1 bldg) | Action 1 cube → 3 damage on agents (was 2) (r118) | 38 → 60 (1.58) | The cube sink on a target; once a round. |
| Parts Exchange (t1 bldg) | Action 1 chip → draw 3 (was 2) (r119) | 41 → 60 (1.46) | Chips 60% spent: a use for the spare. |
| Bug Bounty Board (t1 bldg) | Action 4 cr → drain 4 (was 3), draw 1 (r119) | 34 → 52 (1.53) | Projects hold 7-22 compute a board. |
| Treaty Isotope Bank (t2 bldg) | Action 8 cr → 1 antimatter **and 1 ambassador** (r119) | 54 → 112 (2.07) | Credits bind (103%): a dear use, once a round. |
| Synthetic Data Forge (t1 bldg) | needs 1 Science (was 2; kept 0.03 a game) (r119) | 14 → 16 (1.14) | **Flag:** at values.json it reads 9.25 against its printed 10 (as since 191): a loss at the table, left for a pricing pass. |
| Dyson Swarm Tranche (t2 program) | needs 1 Power (was 2); +3 energy income (was 2) (r118) | 46 → 87 (1.89) | Kept, not played at r117: the 2-Power need. |
| Exascale Training Cluster (t2 bldg) | 26 cr and 2 energy (was 15 and 5: the same at the anchors, energy binds); +4 insight income (was 3); draw 4 (was 6 cubes, which don't bind) (r118) | 55 → 68 (1.24) | Kept, not played at r117. |
| Citywide Microgrid (t2 bldg) | Action sway 3 (was 2); 26 cr and 4 energy (was 8 and 8: the same at the anchors) (r118) | 60 → 81 (1.35) | Its plays had halved since its Action took energy (247). |
| **Twin City Compact** (new t2 program, Cyber + Power) | 6 cr: place 2 nodes; **you may host 1 more city** (op `host_more`) (r118) | 18 → 27 (1.5) | A second host slot for every seat added 0.55 energy a seat-round (r117, greedy d2, 32 dev seeds: hosted energy 13.5 → 23.6 a game); one seat's over ~4.5 rounds, about 11. Useful alone (its 2 nodes). Capped by the cities next to your nodes. |

Microgrid Lab Compact (18 against 18.75) and Bug Bounty Board (12.5 against 13.3) sit near break-even at values.json
before the keep; the anchors give both a visible gain.

**`host_more`:** each seat's host cap is `board.host_city` (1) plus its `host_more` (var `board.host_more.<seat>`,
board bookkeeping, shown as a seat value once raised); the matching takes a cap a seat (`common::host_caps`,
`match_hosts`), the same as r117's with no card played. The agents read the same caps (eval's `host_matching`), value
it as energy income at `values.json`'s `host_more` 0.6, and the strategist files it under income and energy.

**Not changed:** the jammers (Sentinel Mesh, Market Watchdog, Civic Watch): their use needs that tag's threat up, so a
bigger jam pays only in the rounds it can act; their weakness is the precondition, not the pay. **Tidal Grid's chip
converter** (a board rule; staging default): 0.00 uses a seat-game on r117 and r119. Proposal for Matt: "2 chips → 3
energy" (Horizon is the energy corp; energy binds, antimatter doesn't). **Liberate faces:** still below 7.5 pt
(Popular Uprising 5.4, Root Access and Peacekeeping Mission 6.3, Nuclear Option 6.9; Forensic Audit fits). Joining
the rider to the liberation's row made Peacekeeping worse (5.8 pt): the text's length sets the size, so no layout or
icon change fits them without art or cut words; the recorded exception stands. **Two new exceptions (r119),
recorded here:** Positron Storage Ring's top box 6.3 → 5.8 pt and Survey Drone Fleet's 7.5 → 6.9 pt, each Action now
printing a second gain (a card; 2 energy) in a building's ~80-character top box. `cards:fit` ran after the r119 read,
not before its freeze (the miss is this item's); the words are already the house terms, so the fit needs a shorter
effect or a face pass, both play changes after a frozen read: left for the face pass.

## r118: dev screen, fresh read, group probe

Dev screen (36800-36831, 480 games an arm, before Citywide's and Field Assay's edits): the judge +0.2 ± 2.3, with
`w.sink` 1 -2.1 ± 2.2, greedy -0.8 ± 2.4 ([screen.md](screen.md)). **Fresh read** (49356-49403, 720 games an arm,
r117 natively on the same seeds; [read.md](read.md)): the judge **-4.6 ± 1.9** (d3 -9.0 ± 5.3, d4 -10.4 ± 4.8),
greedy **-4.9 ± 2.0**; the full oracle (d2-d3, 72 games) 72/72. **Group probe** on those seeds (spent: a diagnostic;
the judge, d3-d4, 288 games an arm; [groups.md](groups.md)): each group restored to r117 gave back +1.4 to +4.2 (nodes
+4.2 ± 2.8, sinks +3.8 ± 2.2, Twin City priced out +3.5 ± 2.8, flags +1.4 ± 2.6): spread over every group, none
alone, as 256's was. r118's design_check against r117: new Researcher Audit dominant (2.18), antimatter spent 49%
and skipped in 26% of d3-d4 wins (Antiproton's 2 when built fed the surplus), d0 10.1 rounds.

## r119 (amendment 11877): `w.sink` re-screen and the confirming read

**`w.sink` on the stronger converters** (dev seeds, the judge, d3-d4, 192 games an arm; [sink.md](sink.md)): 0.5
-0.5 ± 2.4, 1 -1.0 ± 2.9, 2 -1.0 ± 3.0 against the judge without it (257 on r117's: -0.5 / -2.3 / -3.8). 0.5, the
least costly, rode in the confirming read.

**Confirming read** (49404-49451, three pairs, d0-d4, 48 seeds a cell, 720 games an arm; r117 natively on the same
seeds; [confirm.md](confirm.md)):

| agent | arm | d0 | d1 | d2 | d3 | d4 | all | gap to r117 ± SE |
|---|---|---|---|---|---|---|---|---|
| judge | r117 | 95.1 | 91.0 | 84.7 | 54.2 | 18.8 | 68.8 | |
| judge | r119, `w.sink` 0 | 87.5 | 91.7 | 81.9 | 49.3 | 23.6 | 66.8 | -1.9 ± 1.8 (d0 -7.6 ± 3.3, d3 -4.9 ± 5.2, d4 +4.9 ± 4.4) |
| **judge** | **r119, `w.sink` 0.5 (shipped)** | 86.8 | 87.5 | 81.2 | 52.8 | 27.1 | 67.1 | **-1.7 ± 1.9** (d0 -8.3 ± 3.3, d3 -1.4 ± 5.3, d4 +8.3 ± 4.8) |
| greedy | r117 | 85.4 | 80.6 | 79.2 | 50.0 | 24.3 | 63.9 | |
| **greedy** | **r119** | 87.5 | 81.9 | 77.8 | 50.0 | 23.6 | 64.2 | **+0.3 ± 2.1** |
| full oracle r119 (d2-d3, 12 seeds a level and pair, 49404-49415) | | | | 100 | 100 | | 72/72 | 10.2 rounds |

**`w.sink` 0.5 against the judge without it, same seeds** ([confirm-sink.md](confirm-sink.md)): d3 +3.5 ± 3.2, d4
+3.5 ± 2.7, overall +0.3 ± 1.3 (d1 -4.2 ± 2.6). Amendment 11877's bar (a fresh d3-d4 read no worse) is met: **the
judge ships `w.sink` 0.5** (`STRATEGIST`, `agents/mod.rs`); `strategist:w.sink=0` is r119's judge before it. The
shipped string equals the read's `strategist:w.sink=0.5` arm (the alias appends it), so that arm is the shipped judge
game for game. Greedy unchanged.

Bands (the shipped judge, soft): d0 ≥ 90 **missed by 3.2** (86.8; r117 95.1 on these seeds: the pass's d0 cost,
-8.3 ± 3.3, the one level where both r118 and r119 lose beyond 2 SE), d1 80-95 met (87.5), d2 75-90 met (81.2), d3
55-70 **missed by 2.2** (52.8; r117 54.2 on these seeds, also under), d4 unbanded (27.1). Rounds 9.10 → 8.94; losses
a game on a lost node 0.179 → 0.188, empty deck 0.101 → 0.129.

## Use (the confirming read; r117 → r119 for the judge without `w.sink`, then with it)

- **Every changed card is kept and played** (`play_read.py`: none kept and never played, none never kept, all three
  agents). Kept / played a game (judge): Dyson 0.17 / 0.05 → 0.21 / 0.07, Exascale 0.34 / 0.10 → 0.37 / 0.14 (greedy
  0.09 → 0.32 plays), Twin City Compact 0.43 / 0.37, Synthetic Data Forge kept 0.02 → 0.09. Hosted energy 12.2 → 13.4
  a game.
- **Action uses a game (judge):** Mesh Repeater Network 0.14 → 0.52, Research Reactor 0.21 → 0.62 (0.66 with sink),
  Survey Drone Fleet 0.03 → 0.55 (0.84), Orbital Relay Launcher 0.28 → 0.69 (1.21), Positron 0.54 → 0.64, Parts
  Exchange 0.20 → 0.72, Treaty Isotope Bank 0.11 → 0.55, Citywide Microgrid 0.08 → 0.73, Bug Bounty Board 0.23 →
  0.27, Synthetic Data Forge 0.12 → 0.28; **Antiproton Foundry 0.07 → 0.07 (0.10)** and Field Assay Team 0.12 → 0.12
  stay rare (below).
- **Resources spent of gained (judge, sink 0.5):** credits 105%, energy 86%, insight 82%, ambassadors 81% bind; chips
  65%; **research cubes 62% → 63% and antimatter 47% → 51% still don't bind** (greedy 62%, 53%).
- **Claims and nodes:** owned stations and factories made 2.90 → 3.07 a game (256: 2.93); card and rider claims 1.54 →
  1.72; **the Claim action 0.03 a seat-game (as r117: unused)**. **Cards' share of nodes 8% → 14%** (0.91 → 1.56 a
  seat-game; 256: 9%); greedy 9% → 12%.
- `loop_smoke.py` (r119): no timeout, no card at the play flag.

**design_check (the confirming read against r117 on its seeds):** **no new flag.** As on r117: Protector Bulwark,
Make antimatter, Claim a site, both Tidal Grid converters unused. Gone: r117's Researcher Audit dominant (2.07) and
d0 10.0 rounds on these seeds. The content check: no warning.

**Still weak for anyone:** Antiproton Foundry's Action (a chip is the scarcer currency, and antimatter has no pull
without Orbital), Field Assay Team (needs agents on a target in reach, 0.12 a game), the jammers (a typed threat must
be up), the Claim action (cards claim now), cubes and antimatter as resources (their sinks run, supply outruns them).

## Speed

`per_decision.py`, r117 against r118, 6 games (c2, d2, seed 1000, one thread, the box shared, ratios only): the judge
1.10× a decision at 1.04× the options, greedy 0.99× at 0.95×. r117 against the final build (r119, the judge with
`w.sink` 0.5), the same way: the judge 0.92× a decision at 1.00× the options, greedy 1.16× at 1.13× (other games:
more Actions on the table). `speed.py check` gives no verdict across a revision.

## For new players

Twin City Compact places 2 nodes and lets you host a second free city from then on. Mesh Repeater Network's Action
places a node for free; Orbital Relay Launcher turns an antimatter into 3 nodes and 3 energy; Research Reactor turns
a research cube into 3 energy; Survey Drone Fleet needs one Science tag, comes with 2 cubes and its Action adds 2
energy; Field Assay Team's cube deals 3 damage; Microgrid Lab Compact claims a station or factory when built;
Antiproton Foundry's chip makes an antimatter and 2 energy; Positron Storage Ring's Action also draws a card; Parts
Exchange draws 3; Bug Bounty Board drains 4; Treaty Isotope Bank's credits buy an antimatter and an ambassador;
Synthetic Data Forge needs one Science tag; Dyson Swarm Tranche needs one Power tag and raises energy income by 3;
Exascale Training Cluster costs more credits and less energy, raises insight income by 4 and draws 4; Citywide
Microgrid costs 26 credits and 4 energy and its Action sways 3. No rule resisted a summary.
