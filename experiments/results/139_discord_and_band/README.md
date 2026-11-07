# 139: Discord cards and the difficulty band (r58)

Rules revision 58, bumped from the constant (57 to 58; the prompt also said r58). Rules: Matt's rulings after r57
(2026-11-06, design doc *After r57*) and message 5847 (Border War). Tables: [read_r58_vs_r57.md](read_r58_vs_r57.md)
(137's `read137.py --135` on both dirs), [read_sum139.txt](read_sum139.txt) (138's `sum138.py`),
[read_discord.txt](read_discord.txt) ([read139.py](read139.py) `discord`), [death_audit.md](death_audit.md).
Specs: `experiments/screen-139-{a,b,c,d1,d2}.json`, `read-139-r58-{a,b}.json` (written by [specs.py](specs.py)); the
r57 side ran `read-139-r57-{a,b}.json` from a copy of main 0fa33f7's configs with its binary. The content rewrite is
[discord_139.py](discord_139.py) (then `load_adversary.py`; moved here from `scripts/content/` at 144, and a rerun now reverts later edits).

## Where things stood

The code matched 138's write-up at 0fa33f7 (r57): Corrupt X, bounded arrivals, `threat_nodes` 1 / 2 / 3, Claim
4 / 2 / 2, `board.clock` [3] and `blocker_stays` 1 kept as dials. 138's numbers re-read: on this item's fresh seeds r57
reads 91.7 / 75.7 / 52.8% (d0-d2; 138's own read 88.2 / 84.7 / 60.4 on 36536-36583), median d2 8; Discord cards
destroyed 1.53 nodes a game at d2 (0.91 at d1).

## The rules (each a candidate unless Matt's)

- **Discord cards destroy no nodes.** Each "each player destroys N" step became pressure of about the same weight:
  Incursion 5+ deals 3 and a tier-3 threat in each country; Blight 5+ creates 3, opens its factory, then Corrupt
  cities; Propaganda 5+ takes 8 credits, 4 Unity and 2 ambassadors from every player; Two Fronts 3+ a tier-3 threat
  in each country instead of 2 extra, 4+ 3 extra and a tier-3 a country; Firestorm 3+ adds Corrupt cities, 4+ creates
  3 then Corrupt cities (1 fewer). By feel, tier against tier: a tier-3 threat left uncountered takes 3 nodes near it,
  so the late steps still kill, but only where the table let the board go.
- **Offensive deals a threat** (new op `threat_at_node { tier }`): Corrupt cities, then a tier-2 threat, any tag, by
  the draw rule into the country of the node hex with the most adversary ambassadors and troops, its first token on
  that hex; tier 3 from the 4th Discord tip; at 4+ the adversary builds first. 0.31 / 0.68 / 0.74 a game (d0-d2).
- **Node loss is local.** Other global lines converted: Raiding Parties (a node or 10 credits) is 8 credits; Purge the
  Unaligned (an ambassador or a node) takes an ambassador; Border War (Matt's message 5847) destroys 1 node only for
  each player with a node on or next to a front's base (`each_destroy_node { bases }`). `each_destroy_node` is a
  threat's or twist's only now (the load check refuses it on a Discord card or project); `destroy_node` and
  `sacrifice` are gone. **The stage-I last-node rule is kept** (`last_safe_stage` 2): without it a round-2 war or
  Border War still ends a game, and it costs nothing printed.
- **Claim is symmetric:** `claim_station` and `claim_factory` 3 (cities 4). Give / get at `values.json`: 3 ambassadors
  (12) for a station (`build_station` 8, then a research cube a round, 3) gains from the second round held; for a
  factory (1 chip a round, 2) it gains only after 6 rounds held. Claims fell: stations 0.89 to 0.19 a game at d0,
  factories 0.38 to 0.15 (flagged, not changed: Matt's number).
- **Standing blockers stay**, with no dial (`board.blocker_stays` gone). **The clock is cut:** `board.clock`,
  `clock_step`, `production_round`, `war_round`, `Op::War`, `war_twist`, the view's `board.clock` / `board.war`
  globals and the web's war-round badge (the fronts are always drawn at war).
- **Values:** `remove_botnet` 12 to 8 (1 Sway since r55), `clear_markers` 6 to 10 (2 Sway); new `threat_at_node` -5 a
  tier; `destroy_node`, `sacrifice` and `war` rows gone.

## Calibration (dev seeds; 48 games a cell per 16 seeds, the three pairs)

Matt's levers: I3 (board credits back to 21 / 19 / 17, half of 137's +6), C2 (`alignment.carry` 2), L1
(`corrupt_cut_at` 1: the cut moves from d2 to d1; L0 puts it at d0). T1 (tier-1 counters at 200%) is the adversary
lever tried for d0; X0 (`extra_threats_at` 0, a new dial: the extra threat a round moves from d1 to every level) the
one that moved it. Cells pool 36800-36815 and 36816-36831 where both ran.

| Cell | d0 | d1 | d2 | d2 median | d2 nodes: threats / corruption |
|---|---|---|---|---|---|
| r58 as built | 89.6 | 89.6 | 60.4 | 9 | 2.65 / 2.21 |
| I3 | 91.7 | 85.4 | 52.1 | 9 | 3.69 / 2.58 |
| C2 | 85.4 | 81.2 | 52.1 | 8 | 2.81 / 2.21 |
| L1 | (= base) | 87.5 | (= base) | 9 | |
| I3+C2 | 89.6 | 79.2 | 43.8 | 8 | |
| **I3+L1 (frozen)** | 91.7 (96) | 64.6 (96) | 53.1 (96) | 9 | 3.60 / 1.81 |
| C2+L1 | 91.7 (96) | 72.9 (96) | 53.1 (96) | 8 | 3.25 / 1.98 |
| I3+C2+L1 (asked) | 87.5 (96) | 54.2 (96) | 37.5 (96) | 8 | 3.58 / 1.83 |
| L0 | 86.5 (96) | 83.3 | | | |
| I3+L0 / C2+L0 | 91.7 (96) / 87.5 (96) | | | | |
| L0+T1 / I3+L0+T1 | 89.6 (96) / 91.7 (96) | 74.0 / 65.6 (96) | 59.4 / 42.7 (96) | 9 / 8 | |
| **I3+L1+X0 (final)** | 83.3 (96; I3+L1 91.7 on the same games: -8.3 ±4.6) | = I3+L1 | = I3+L1 | 9 | |

**The asked cell** (all three levers) overshoots: d1 54, d2 38 with median 8. **I3+L1** is the cell nearest the band
at d1 and d2 with every median at 9 and uncountered threats ahead of corruption at d1 and d2; it was frozen and read
(d0 93.8 / d1 61.8 / d2 51.4 on the fresh seeds). **d0 resisted every lever that acts at all levels:** adversary-side
(L0, T1, C2) or player-side (I3), d0 stayed at 86-92 (Matt's message 5865 asked for L0 and a d0 lever: L0 read 81.2 on
the first 16 seeds and 91.7 on the next 16). At d0 the table counters 18 of 22 threats a game. **The lever that bit is
the ladder:** d1's extra threat a round (one more pull of the card's first pull, countries in turn) moved to every
level (X0), which leaves d1 and d2 exactly as they were (their games replay identically, 288 of 288) and makes d1's own
step the corrupt cut. Re-frozen with X0 and the whole fresh read rerun on the final build.

## The read (fresh seeds 36584-36599 and 36648-36679, 48 a cell, the three pairs; r57 = main 0fa33f7)

| | d0 | d1 | d2 |
|---|---|---|---|
| strategist r58 (final) | 77.8 ±3.5 | 61.8 ±4.0 | 51.4 ±4.2 |
| strategist r58 first freeze (I3+L1, no X0) | 93.8 ±2.0 | (same games) | (same games) |
| strategist r57 | 91.7 ±2.3 | 75.7 ±3.6 | 52.8 ±4.2 |
| paired r58 - r57 | -13.9 ±4.0 | -13.9 ±4.6 | -1.4 ±5.0 |
| paired final - first freeze | -16.0 ±3.6 | 0 | 0 |
| rounds r58: mean, median, p10-p90 | 9.09, 9, 8-11 | 8.77, 9, 6-10 | 8.20, 9, 5-10 |
| paired rounds r58 - r57 | +0.19 ±0.15 | +0.15 ±0.16 | +0.58 ±0.18 |
| tier 3 in games (r58 / r57) | 84% / 64% | 89% / 68% | 84% / 61% |

**How games end** (r58 / r57, of 144): d0 Unity deck 97 / 102, purge 15 / 30, cap 4 / 7, node 16 / 5, Discord deck 12 /
0; d1 87 / 94, 2 / 15, 0 / 3, 28 / 31, 27 / 1; d2 72 / 72, 2 / 4, 0 / 2, 46 / 64, 24 / 2. The Discord deck is now a
real loss (24 of 144 at d2) and node deaths fall 64 to 46 at d2.

**Nodes lost a game by cause** (r58 / r57):

| | corruption | uncountered threats (t1 / t2 / t3) | spent as counters | Discord cards | Border War (twist) | war | projects |
|---|---|---|---|---|---|---|---|
| d0 | 2.01 / 1.72 | 2.65 (0.60 / 1.53 / 0.51) / 0.39 | 0.67 / 0.35 | 0 / 0.18 | 0.71 / 0.63 | 0.15 / 0.19 | 0 / 0.03 |
| d1 | 1.96 / 1.65 | 2.82 (0.62 / 1.67 / 0.52) / 1.62 | 0.73 / 0.71 | 0 / 0.91 | 0.65 / 0.72 | 0.12 / 0.12 | 0 / 0.14 |
| d2 | 2.17 / 1.73 | 3.63 (0.49 / 2.12 / 1.02) / 1.92 | 0.60 / 0.49 | 0 / 1.53 | 0.60 / 0.76 | 0.16 / 0.17 | 0 / 0.10 |

**Threats a game** (r58, dealt / countered / uncountered; by tier dealt (uncountered)): d0 32.7 / 25.6 / 4.10, 15.9
(1.85) / 12.0 (1.72) / 4.8 (0.54); d1 32.6 / 25.2 / 4.47, 15.9 (2.19) / 11.9 (1.81) / 4.8 (0.47); d2 31.8 / 23.3 /
5.40, 12.9 (1.88) / 12.5 (2.53) / 6.5 (0.99). r57: d0 20.9 / 17.7 / 1.12, d2 27.6 / 21.7 / 3.38.

**Discord cards a game** (r58, d2; d0 in read_discord.txt: 3.81 tips, Offensive 0.54): Discord tips 4.60 (r57 3.72); drawn Incursion 0.85, Blight 0.70, Propaganda 0.74,
Offensive 0.74 (each dealt its threat onto a node hex), Two Fronts 0.70, Firestorm 0.70; ambassadors they created 3.80
(r57 2.27). Their effects now land as threats, creation and corruption: the uncountered tier-3 nodes (1.02 a game at
d2, from 0.18) are mostly theirs.

**Peak holdings** (d2, Builder / Protector / Researcher, r58): credits 70 / 60 / 80 (I3: 137's +6 halved), insight 11.1 /
16.7 / 24.5, energy 8.1 / 7.4 / 8.0, chips 5.2 / 3.6 / 4.5, ambassadors 4.5 / 5.0 / 5.2. Every cell, r57 beside it, is
in read_r58_vs_r57.md.

## The death audit (136's `death_audit.py` on r58's fresh d1 and d2 games)

The d1 and d2 games are the same under both freezes (X0 changes only d0), so this audit is the final build's.

46 last-node losses at d2 and 28 at d1 (r57's 138 audit: 49 at d2). **The fatal blow, d2:** uncountered threats 34
(Shock Doctrine 5; General Mobilization, Dark Cities, Cascade Failure 3 each; 23 other threats), corruption 11,
Border War 1, **Discord cards 0** (r57: 34, Offensive 17). **d1:** threats 23, corruption 2, Border War 2, a
Firestorm Corrupt line with a threat 1. Two "Discord card" removals in the whole d2 record set are Offensive's and
Firestorm's Corrupt lines taking the node on the hex they corrupted: local by the rule. The fatal phase is the threat
execute step in 34 of 46 (d2) and 24 of 28 (d1). **A node-safe line** (extra placements affordable against nodes
needed) existed in 3 of 46 at d2 and 2 of 28 at d1: of the 43 and 26 without one, every one came from the board
overrun near the node (threats 32 / 21, corruption 10 / 2, Border War at a base 1 / 2, the Firestorm line 0 / 1).
Corruption deaths: a node-safe placement existed in 2 of 8 with a run-up; the fatal city held 0 adversary ambassadors
at the prior action phase's end in 15 of 16 (gathered in from next door by the Corrupt line). The fatal hit took every
node held at the end of the round before in 36 of 46: seats die from 1-2 nodes, as at r57. Builder still dies most (21
of 46). **d0** (final build, 16 losses): threats 12, corruption 3, a Blight Corrupt line with a threat 1; a node-safe
line in 3, the other 13 all local. **Verdict:** the deaths are now the board's local hits, threats first, as the rule asks.

## Done when, against the read

- No Discord card or other adversary content destroys nodes except by the local routes: **met** (the load check
  refuses `each_destroy_node` outside a threat or twist; Discord cards and projects destroyed 0 nodes in 432 games);
  Offensive deals its threat (0.31-0.74 a game).
- The band within about ±5 a level, d2 median 8-10: **met** on the fresh read: d0 77.8 (about 80), d1 61.8 (65-70,
  3.2 below its floor), d2 51.4 (55), d2 median 9; tier 3 in 84% of d2 games; uncountered threats take more nodes
  than corruption at d1 (2.82 / 1.96) and d2 (3.63 / 2.17), and now at d0 too (2.65 / 2.01).
- The death audit shows most last-node losses with no node-safe line came from the board being overrun locally:
  **met** (43 of 43 at d2, 26 of 26 at d1, 13 of 13 at d0).

## Speed

`per_decision.py` (r57 against r58, strategist, d2, 6 games, one thread, box loaded): 1.2 ms a decision against 2.6
(0.46x), 560 ms a game against 870 with 1.3x the rounds. No `speed.py check` (the box was loaded and the revision
moved, where it gives no verdict).

## Not done

- `npm run e2e` (dev server on the conductor port): every check passes but `board fills its pane at 1920x1080`
  (board 453 px, floor 464). `npm run panes` on r57's tree against r58's: the fixed game's countries strip grew
  128 to 182 px at 1920 (96 to 115 at 1366, 125 to 191 at 2560) while the board's other panes held, so the rules
  moved the game it measures, not the layout (web/README); the floor is Matt's, left as is. The e2e's war-round check
  became "fronts drawn at war" (no clock). `npm run check` is clean. `npm run cards:fit` threw on 135's threat and
  adversary formats (`web/scripts/cards-export.mjs` read `counter` and an event array): fixed here; it now reports 9
  player cards with a box below 7.5 pt, none touched by this item.
- The rules docs' new-player summary covers the Discord cards and how nodes are lost: **met** (alignment.md's *How the
  game ends*, generated, and war.md's *Node destruction*).
