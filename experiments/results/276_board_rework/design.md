# 276: the boards as built (r131)

Matt's rulings (2026-11-18): node placement is the board's **Growth** (pick one printed option a round, Spirit Island's
"pick one"); powers are **tiered** and stay sacrifice-paid where a tag pays (a tier sacrifices more of the tag for a
stronger effect); a few may be **threshold** powers ("with N tags on your cards", nothing sacrificed); **Mint goes**.
Message 12618 (Matt, via the planner): Fortify stays chip-paid and Patrol energy-paid (resource-paid powers stand).

**One use a round, one marker.** Growth is one use: pick one option. A tiered power is one use: pick one tier. So a
board asks about 3-4 power decisions a round where it asked 8-10.

**Engine.** A power may name a `group` (`content/ais.json`): powers sharing a group are one use a round between them
(Growth's options, a power's tiers); `needs` makes a threshold power (tags on your cards, board tags counting, none
sacrificed). Every placement still pays the track step's energy (locked); each Growth option prints its own price.

## The boards, before (r130) and after (r131)

Uses are the judge's per game on 275's read (r130, d2, a game is about 10 rounds); "decisions a round" counts each
power or group a player weighs once a round (a yes/no, most with a target).

### Builder (Breadth: 5 slots)

| Before (r130) | Cost | Uses a game (d2) | After (r131) | Cost | Where the job went |
|---|---|---|---|---|---|
| Expand | step only | 5.6 | **Growth: Buy In** | nothing | place 1 node; gain 1 chip (Buy In's chip, now without the fresh-factory check) |
| Buy In | 2 energy | 2.4 | **Growth: Sprawl** | 1 energy | place 2 nodes (energy placing nodes) |
| Annex | 1 chip | 2.6 | **Growth: Annex** | nothing | place 1 node within 2 of yours; gain 3 credits (the Builder's top route keeps its reach) |
| Mint | 2 credits | 6.2 | (gone) | | Matt's ruling: chips need an engine. Chip routes kept: Buy In, Scrap I, Full Capacity, cards |
| Scrap | 1 Industry | 2.2 | **Scrap I / II** | 1 / 2 Industry | I: gain 1 chip. II: gain 1 chip; play a building from your hand, paying no credits (Rush Build's job) |
| Rush Build | 1 Industry + 1 energy | 0.7 | (into Scrap II) | | |
| Overcharge | 1 Power | 1.4 | **Overcharge I / II** | 1 / 2 Power | I: gain 2 energy. II: gain 2 energy and 1 antimatter (Teardown's antimatter) |
| Teardown | 1 chip | 0.4 | (into Overcharge II) | | the antimatter burn becomes a 2-Power sacrifice; no factory is destroyed |
| Procure | 2 chips | 0.04 | (gone) | | the judge's 0.04 a game (band 0.40-0.90); the Builder draws from cards |
| Fortify | 1 chip | 2.8 | **Fortify I / II** | 1 / 2 chips | I: remove 1 troop on the target. II: remove up to 2 on it; harden a facility on or next to it. The chip sink |

Decisions a round: 10 powers → **4** (Growth, Scrap, Overcharge, Fortify). Targets: Fortify's and Annex's placement.

### Protector (Wall)

| Before (r130) | Cost | Uses a game (d2) | After (r131) | Cost | Where the job went |
|---|---|---|---|---|---|
| Expand | step only | 5.4 | **Growth: Accord** | 1 energy | place 1 node; gain 1 ambassador (Accord's and Outpost's ambassador, no fresh-city check; free on the dev screen, it was always taken over the third option) |
| Accord | 2 energy | 1.3 | **Growth: Outpost** | 2 energy | place 2 nodes (energy placing nodes) |
| Outpost | 3 energy | 3.3 | **Growth: Levy** | nothing | place 1 node; gain 3 credits (the dev screen's Stockpile, a node and 2 energy, was never picked: the Protector's energy doesn't bind) |
| Bulwark | 1 energy | 1.5 | (into Patrol II) | | hardening rides on Patrol II (Bulwark 0.17 a round, under its band) |
| Patrol | 1 energy | 4.5 | **Patrol I / II** | 1 / 2 energy | I: push up to 3 agents or troops from the target. II: and harden a facility on or next to it |
| Interdict | 1 Security | 1.3 | **Interdict I / II** | 1 / 2 Security | I: remove up to 2 troops on the target. II: up to 2 troops and up to 2 agents on it; draw 1 card (Debrief's jobs) |
| Debrief | 1 Security | 1.5 | (into Interdict II) | | |
| Reclaim | 1 Security + 4 credits + 2 energy | 0.02 | (gone) | | 0.02 a game; Liberate turns a corrupted site neutral; Sanctuary's 2 antimatter stays |

Decisions a round: 8 powers → **3** (Growth, Patrol, Interdict).

### Researcher (Patience)

| Before (r130) | Cost | Uses a game (d2, pooled) | After (r131) | Cost | Where the job went |
|---|---|---|---|---|---|
| Expand | step only | 5.1 | **Growth: Field Survey** | nothing | place 1 node; draw 1 card (Field Survey, now free) |
| Peer Review | 2 insight | 1.6 | **Growth: Peer Review** | 2 insight | place 2 nodes (the Researcher places with insight); its fresh-station insight goes |
| Field Survey | 3 insight | 3.4 | (Growth: Field Survey) | | |
| Field Station | 1 Industry | 0.7 | **Growth: Field Station** | 1 insight | Target: an open hex. Build a research station on it, paying no chips; place 1 node |
| Synthesis | 1 insight | 6.7 | **Lab I** (threshold) | 1 insight, with 2 Science on your cards | draw 1 card. A threshold, not a sacrifice: the Science AI hoards Science |
| Grant | 2 credits | 3.2 | **Lab II** (threshold) | 1 insight + 2 credits, with 4 Science | draw 1 card; add 1 research cube in your color to an open megaproject |
| Disrupt | 1 Cyber + 1 cube | 1.6 | **Hack I / II** | 1 / 2 Cyber | I: remove up to 2 troops in one country (no cube). II: and remove 3 compute from one adversary project (Audit's job) |
| Audit | 1 Cyber | 1.2 | (into Hack II) | | |
| Meltdown | 2 cubes | 0.4 | (gone) | | 0.2-0.5 a game; Open Archive's 2 antimatter and the plants stay; cubes go to converters and megaprojects |

Decisions a round: 9 powers → **3** (Growth, Lab, Hack).

## Give / get at the unit anchors (`configs/agents/values.json`)

Credit 1, chip 2.67, energy 3, insight 1.67, ambassador 2.67, node 5.33, drawn card 2.67, antimatter 12, troop removed
4.67, hardening 1, station 6.67, a building free 5.33, a megaproject cube 4, compute 0.73. A sacrificed tag is a played
card's leftover (late-game liquidation), not priced here; each placement also pays its track step.

| Board | Option | Give | Get | Net |
|---|---|---|---|---|
| Builder | Sprawl | 1 energy (3) | 2 nodes (10.67) | +7.67 |
| Builder | Buy In | 0 | node + chip (8) | +8 |
| Builder | Annex | 0 | node within 2 + 3 credits (8.33 + reach) | +8.33 |
| Builder | Scrap I / II | 1 / 2 Industry | chip (2.67) / chip + free building (8) | 2.67 a tag / 4 a tag |
| Builder | Overcharge I / II | 1 / 2 Power | 2 energy (6) / 2 energy + antimatter (18) | 6 a tag / 9 a tag |
| Builder | Fortify I / II | 1 / 2 chips (2.67 / 5.33) | 1 troop (4.67) / 1.48 troops + hardening (7.9) | +2.0 / +2.6 |
| Protector | Outpost | 2 energy (6) | 2 nodes (10.67) | +4.67 |
| Protector | Accord | 1 energy (3) | node + ambassador (8) | +5 |
| Protector | Levy | 0 | node + 3 credits (8.33) | +8.33 |
| Protector | Patrol I / II | 1 / 2 energy (3 / 6) | push up to 3 (2.8) / push + hardening (3.8) | -0.2 / -2.2 at the table; a push is worth where it lands |
| Protector | Interdict I / II | 1 / 2 Security | 1.48 troops (6.9) / troops + 2 agents + card (6.9 + agents + 2.67) | 6.9 a tag / about 7 a tag |
| Researcher | Peer Review | 2 insight (3.33) | 2 nodes (10.67) | +7.33 |
| Researcher | Field Survey | 0 | node + card (8) | +8 |
| Researcher | Field Station | 1 insight (1.67) | station + node (12) | +10.33 |
| Researcher | Lab I | 1 insight (1.67) | card (2.67) | +1.0 (a draw, as Synthesis) |
| Researcher | Lab II | 1 insight + 2 credits (3.67) | card + cube (6.67) | +3.0 |
| Researcher | Hack I / II | 1 / 2 Cyber | 1.48 troops (6.9) / troops + 3 compute (9.1) | 6.9 a tag / 4.6 a tag |

Patrol's table net is under 0, as r130's Patrol (1 energy for a push) was: the table prices a push at its average, and
the Protector pushes troops off its nodes' hexes and agents off a site one short of falling, where it is worth more.
No loop: each is once a round; chips come only from Buy In, Scrap I, Full Capacity and cards, so Fortify can't
recycle.
