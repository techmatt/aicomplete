# 078: the starting-economy scaffold (r33)

Matt's brief (2026-11-03) and messages #3446, #3451 and #3464; simulation is a ballpark check (amendment #3415): two players,
the three board pairs (bp Builder + Protector, br Builder + Researcher, pr Protector + Researcher), d2, the `sweep` profile, target
45-85 a pair. Specs: `experiments/screen-078*.json`, `use-078.json`. Every number below is a win rate in percent unless named.

## What the sizes did

| read | agent, seeds | bp | br | pr | ends |
|---|---|---|---|---|---|
| r32 rules (base binary), r32 sizes | engine, 29900, 16 | 50 | 44 | 25 | |
| r33 structure, r32 credits (ctl; keeps r33's +2 insight card-play spaces) | engine, 29900, 32 | 56 | 22 | 31 | |
| Matt's sizes, ambassador project 9, r32 pace | engine, 29900, 32 | 100 | 100 | 100 | purge by round 6.5 |
| Matt's sizes, ambassador project 9, r32 pace | sweep, 30000, 48 | 100 | 98 | 100 | purge by round 6 |
| the smallest size screened (start 25/32/25, printed 20/17/14, track +1-2) | engine, 29900, 32 | 84 | 66 | 69 | |

At the 9-credit project the extra credits bought ambassadors: 11-14 a game under the sweep, 16 under the engine (6 at r32), and
those ambassadors purged the board. Every size of the scaffold read 66-100 under the engine until the adversary moved.

## The adversary (Matt, #3446, #3451), on Matt's sizes

Engine, 29900, 32 seeds a cell, the three pairs' mean: a = ambassador project 14, r = arrivals 2, c = city_build 2, f = factory_build 1.

| | none | f | c | cf | r | rf | rc | rcf |
|---|---|---|---|---|---|---|---|---|
| project 9 | 100 | 94 | 97 | 81 | 88 | 61 | 56 | 25 |
| project 14 | 88 | 67 | 73 | 44 | 40 | 22 | 19 | 11 |

With the project at 20 or 24 (engine, 32 seeds): r32 pace 72 / 70, city_build 2 52 / 57, factory_build 1 38 / 48, both 26 / 23.

The sweep, 30100, 24 seeds a pair:

| candidate | bp | br | pr |
|---|---|---|---|
| project 14, factory_build 1 | 75 | 75 | 79 |
| project 14, city_build 2 | 75 | 83 | 96 |
| project 14, city_build 2, factory_build 1 | 62 | 58 | 58 |
| project 14, arrivals 2 | 67 | 46 | 71 |
| project 12, arrivals 2 | 67 | 67 | 83 |
| project 24, city_build 2 | 54 | 42 | 67 |
| **project 20, city_build 2 (frozen)** | 71 | 75 | 62 |
| project 24, r32 pace | 79 | 79 | 83 |
| project 24, factory_build 1 | 29 | 67 | 58 |

## The frozen rules, fresh seeds

Sweep, 30200, 48 a pair: **bp 70.8 ±6.6, br 64.6 ±6.9, pr 62.5 ±7.0**; 22-23 of the 48 won by purge, 7-12 by the Unity deck, 13-15
lost to the Discord deck. The seats bought 3.6-6.1 ambassadors a game at 20 credits; 0.7 cities corrupted a game.

**Throughput** (`scripts/cards/economy_read.py`), per seat and game: 18-19 draft cards kept (4 in the opening), 12-14 played,
8.4-8.6 kept and never played; 10 presence; 28-32 credits of income a round; 9-10 credits and 2.6 insight left unspent a round
(the Researcher carries about 10 insight by Patience). r32 under the engine on 29900: 8-9 draft cards played a seat, plus 1-2
unique cards. **Tags:** a seat holds 3 of one tag on its cards in play by round 5 in 60-93% of games (the Protector beside the
Researcher 23%), by round 7 in 82-97% (Protector + Researcher's Protector 42%).

**The dear powers at 2 tags:** Rush Build, Shelter and Particle Run 0.00 uses a game in every pair, with the seat short of the tag
in 3.5-6.9 seat-rounds a game; Recovery 0.06. The sweep prefers keeping the cards that carry the tags.

**Kept and never played (over 10% of games):** Orbital Observatory Array 44%, Hardpoint Lockdown 43%, Inference Cluster 40%, Demand
Response Grid 38%, Simulation Cluster 37%, Agency Directorate 20%, Policy Institute 17%, Quantum Decryptor 16%, Arc Furnace Complex 13%.
Never kept: Foundation Model Campus, Manifesto Leak, Summit Caucus.

## The tier-2 rush (the loop the brief flagged)

Sweep, 30200, 48 a pair, paired against the frozen sweep:

| agent | bp | br | pr | pooled, 144 seeds |
|---|---|---|---|---|
| `sweep:tier2_rush=1` | +8.3 ±7.8 | +8.3 ±8.3 | +22.9 ±9.1 | **+13.2 ±4.9** |
| `sweep:w.packet_keep=0` (draft keep price off; diagnostic) | +8.3 ±6.5 | +8.3 ±8.8 | +12.5 ±7.7 | +9.7 ±4.4 |
| `sweep:w.packet_budget=0` (diagnostic) | -4.2 ±8.4 | +8.3 ±7.8 | +10.4 ±8.5 | +4.9 ±4.8 |
| rush against `w.packet_keep=0` | 0.0 ±7.3 | 0.0 ±8.4 | +10.4 ±6.8 | +3.5 ±4.3 |

Tier 2 kept / played per game (team): the sweep 10.3-11.6 / 5.9-6.8; the rush 13.6-14.9 / 8.4-9.1; `w.packet_keep=0` 12.7-14.7 /
7.3-8.7. **Verdict (MESSAGE #3464):** the planner under-drafts tier 2 at this income. Its draft charges the keep's insight, and with
the charge off it drafts as the rush does and closes most of the gap. This is an agent item, not a price item; pr's 10.4 ±6.8
residual (1.5 SE) is the only hint that tier 2 is also cheap there. Nothing adopted.

**Engine checks on the frozen rules** (29900, 16 seeds, direction only): `always_max_presence` 69 / 44 / 62 against the engine's
44 / 44 / 69; `tier2_rush` 62 / 62 / 75.
