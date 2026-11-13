# 206: standard actions priced as hedges, board powers that differ, troop answers (r91)

`RULES_REVISION` 90 → 91, bumped from the constant (the prompt said r91 too). Built on main e188cbf (204, r90). Every
number is a candidate. Old side: r90 = e188cbf's binary and configs (`out/r90`), same seeds. Tables:
[fresh-strategist.md](fresh-strategist.md), [fresh-greedy.md](fresh-greedy.md), [insight-fresh.md](insight-fresh.md),
screens [screen1.md](screen1.md) (levers, before the per-round limits), [screen2.md](screen2.md) (limits; stage-I
Musters), [screen3.md](screen3.md) (Place a node twice; price 4; troop hedge twice), [dials.md](dials.md); design
checks [design-screen1.txt](design-screen1.txt), [design-fresh.txt](design-fresh.txt). Readers: `read206.py`,
`insight206.py`; specs: `spec.py`.

## What changed

| change | r90 | r91 | why |
|---|---|---|---|
| Clear a corruption marker | 2 credits, 2 Unity | **cut** (`clear_cost` 0) | amendment 9208: no standard action removes corruption |
| Project: drain | 5 credits, 3 compute | **cut** | 9208 |
| Project: harden | 3 credits | **cut** (candidate) | 9208; screen: back at 3 credits reads −1.0 to +6.2 (noise), nodes lost the same |
| Take down a botnet | 3 credits | **16** | 9257: 5.33 over the anchors; 16 is par (botnet 16) |
| Buy a city's energy | 4 credits | **5** | 1.12 over; 5 is 0.90 |
| Place a node (new) | Expand on all 3 boards (1 energy) | **6 credits + the step, twice a round** | locked: one poorly priced node action; once a round read d2 47.9, twice 63.5 (screen 3) |
| Remove a troop (new) | — | **10 credits, once a round**, 1 in reach | 9247's hedge (0.5 at the anchors); twice a round reads ±2 |
| Claim / Liberate | +1 ambassador an agent; Liberate's factory 3 | **no agent on the site; one table 5 / 3 / 2** | 9208; 5/3/3 read −2 to −9 (screen 1) |
| Station | 1 chip | **1 chip** (2 chips or +1 ambassador screened) | 9208/9212: 2 chips −5 to −12, +1 ambassador −4 to −13 |
| Build a factory (new, off) | — | `factory_build_chips` 0 | 9212: at 3 chips used 0.01 a seat-game (no free industrial zone in reach): no read either way; Claim a factory stays |
| Node powers | Expand + Buy In / Accord / Peer Review | Buy In +1 chip, Accord +1 ambassador, Peer Review +1 insight | locked: genuinely different; 9309 asked for different targets: riders instead (a target filter on `place_node` was more code than the time allowed) |
| Builder | Focus, Pair Forge | **cut**; new **Fortify** (2 chips: 1 troop in reach, harden a facility) | Focus unused 0.01; Pair Forge unused and the same effect as Particle Run |
| Protector | Debrief draw 2 | Debrief draw 1 + 1 agent in a hex in reach; new **Interdict** (Security + 1 energy: 2 troops in a hex) | Debrief ≠ Procure; 9247 |
| Researcher | Fieldwork (1 energy: 3 cards), Synthesis (card → 2 insight) | **Synthesis 2 insight → 1 card**; new **Disrupt** (Cyber + 1 cube: 2 troops in a country) | locked; 9309: Synthesis replaces Fieldwork |
| Ambassador anchor | values.json 4 | **8** (card-guidelines) | 9309 |

**Give / get at the anchors** (written late: the first screen ran before them, a CLAUDE.md slip). Place a node 6 + 4.5
(a step's energy) → 8: 0.76. Remove a troop 10 → 5: 0.5 (values.json 11: the agents see 1.1). Botnet 16 → 16: 1.0.
Buy energy 5 → 4.5: 0.90. Buy In 6 → node 8 + chip 6: 2.3. Accord a tag → node 8 + ambassador 8. Peer Review 3 cubes
(18) → node 8 + insight 4: 0.67 at the anchors, 1.3 at values.json (cube 3). Synthesis 2 insight (8) → a card (5):
0.63, a draw outlet, not a gain (used 4.1 a Researcher game). Debrief a tag → card 5 + agent 5. Fortify 2 chips (12)
→ troop 5 + harden 3: 0.67 (values.json 8 → 14). Interdict a tag + 4.5 → up to 2 troops 10. Disrupt a tag + a cube
(6) → up to 2 troops 10. Standard actions are a loss at the table by Matt's locked ruling (the anchors win over
CLAUDE.md's "a gain a player can see" for them).

## Fresh read (45631-45726, 96 a pair, r91 against r90 on the same seeds)

| d | r91 strategist | r90 | gap ± SE | r91 greedy | r90 greedy | rounds r91 (r90) | troops removed | nodes lost | lost to node loss |
|---|---|---|---|---|---|---|---|---|---|
| d0 | 90.3 | 89.2 | +1.0 ± 2.5 | 89.9 | 93.1 | 7.76 (7.45) | 1.67 (0.61) | 4.62 (4.64) | 0.0% (1.0) |
| d1 | 81.9 | 84.4 | −2.4 ± 2.9 | 73.6 | 79.5 | 7.61 (7.28) | 3.98 (1.87) | 6.45 (6.40) | 4.5% (3.8) |
| d2 | 55.6 | 57.6 | −2.1 ± 3.2 | 52.1 | 57.3 | 6.94 (6.58) | 5.81 (2.85) | 8.95 (8.48) | 27.1% (30.9) |
| d3 | 36.8 | 41.0 | −4.2 ± 3.5 | 43.1 | 43.8 | 6.65 (6.06) | 6.14 (2.62) | 9.81 (8.87) | 34.4% (38.9) |
| d4 | 20.5 | 20.1 | +0.3 ± 2.8 | 19.4 | 22.2 | 6.02 (5.69) | 6.58 (2.49) | 10.66 (9.82) | 56.6% (64.2) |

Troops removed rose 2.2-2.6× at every level; games lost to node loss fell at d2-d4; nodes lost a game rose slightly
(about the same per round: games run 0.3-0.6 rounds longer). Node protection without the Harden project held.

**Dials, last** (on the spent fresh seeds, 204's route): `compute_income` d1 8 → 12 read 80.6, → 16 read 64.6; d3 25
→ 21 read 45.1, → 18 read 44.8. Taken: d1 13, d3 21, confirmed on the next free seeds 45727-45822
([dials.md](dials.md)): d1 79.2 ± 2.4 (r90 78.5, +0.7 ± 3.1), d3 46.9 ± 2.9 (r90 43.4, +3.5 ± 3.6); rounds 7.56 and
6.90 (r90 7.60, 6.28). Final ladder: d0 90.3, d1 79.2, d2 55.6, d3 46.9 (each band met), d4 20.5.

**Replay (Matt's message 9365):** the fresh strategist seeds replayed on the final build (after the fast lane, docs
and cards:fit went green, before the dial): `bench/same_results.py` "same: 1440 games".

Seeds: dev 36800-36831; fresh 45631-45726 and confirm 45727-45822 spent (ledger next free 45823).

## Standard actions and powers, uses a seat-game (fresh strategist, levels pooled)

r90: ambassador 0.53, antimatter 0.36, botnet 1.65, breakthrough 1.68, buy energy 2.02, claim 0.16, clear 1.23,
drain 1.69, envoy 0.16, harden project 0.68, cube→insight 0.73, liberate 2.18, dispatch 4.61, remove agents 1.06,
station 0.64. r91: ambassador 0.30, antimatter 0.40, botnet 0.41, breakthrough 2.57, buy energy 1.87, claim 0.14,
envoy 0.14, cube→insight 1.36, liberate 2.49, dispatch 4.61, **place a node 4.28**, remove agents 1.15, station 1.00,
**remove a troop 0.50**. Powers (a game of their board): Fortify 0.39, Interdict 0.70, Disrupt 0.79, Synthesis 4.07,
Recovery 0.44 (r90 0.22), Debrief 1.05.

## Insight (fresh, a seat-round, by board)

| board | kept r91 (r90) | available | spent | unspent share | bound share |
|---|---|---|---|---|---|
| Builder | 1.84 (2.07) | 5.31 (6.07) | 4.23 (4.39) | 0.20 (0.28) | 0.35 (0.28) |
| Protector | 1.99 (1.94) | 5.54 (5.42) | 4.07 (4.12) | 0.27 (0.24) | 0.51 (0.56) |
| Researcher | 1.85 (1.82) | 6.98 (8.80) | 4.66 (3.67) | 0.33 (0.58) | 0.33 (0.27) |

Insight to keeps 61% (r90 58%), to plays 39%. Patience's bank: the Researcher's available insight a round fell 8.80 →
6.98 and its unspent share 58% → 33%, since Synthesis now spends it on draws. Matt's target (about 6 a round to
drafting, 2-4 to plays) is far off: seats spend about 4.2 in all.

**The binding lever, screened last on the dev seeds** ([screen-insight.md](screen-insight.md), final r91 with the
dials): a 5-card draft hand (`cards.deal=5`) makes insight bind in 60-84% of seat-rounds (base 38-49%) and keeps 2.0-2.3
a seat-round (base 1.8-2.0), wins −12.5 / +9.4 / −5.2 / +2.1 / −7.3 (SE about 5); with 199's t4 track, available insight
rises to 7.2-9.7 but binds less (29-37%). Not adopted: it needs its own fresh read; deal 5 alone is the next step.

## Amendment 9257 and 9261

`troop_steps` 2 against 1 on r90's dev seeds: −3.1, −4.2, −19.8, −8.3, −7.3 (d0-d4): troop_steps 1 holds. The stage-II
Muster re-read was not run. Stage-I Musters back (screen 2, every troop power and the hedge in): one card (War Drums)
−7 to −25, two (and Probing Strikes) −12 to −31, rounds −0.4 to −1.6: none kept.
