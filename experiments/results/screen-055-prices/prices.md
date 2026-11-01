# Price pass 1: reasoned, measured, repriced at d2 (055, r24)

Judged by the `planner`, d2. The revision is bumped from the constant, 23 (054 took none). Every price
is a candidate. Specs: `experiments/screen-055-prices.json`, `experiments/gate.json`,
`experiments/archive/gate-close-r24-c2.json`, `-c3.json`. Read with `scripts/cards/price_read.py`.
`results.json` and `games.csv` here are the final measure (r24, seeds from 9000, 288 games), for the
card viewer; `card-prices.csv` is the reasoned price of each of the 150 cards at r23's costs; its `revision` and
`note` columns say so and the viewer prints them, since the prices are floored at 0 (the tables below are not), no
`needs` is priced, and 63 of the 150 cards have moved a cost side since r23 (33 tier 1, 30 tier 2).
`price-read-r23.csv` and `price-read-r24.csv` are the per-card tables, before and after.

Seeds, each used once for its purpose: 7000+ baseline and both sweeps (paired), 8000+ a candidate that
did not ship, **9000+ the verdict, never used to pick anything**, 10000+ the gate.

## Verdict

| Asked | Result | |
|---|---|---|
| Planner at c2 and c3 inside 60-75 | **67 ±7, 65 ±7**, 48 fresh seeds | **met** |
| Tier-2 rush no longer leads the planner | **-8 ±8 at c2, +4 ±8 at c3**, paired, fresh | **met at c3; at c2 level, one SE behind** |
| Reasoning and measure agree, or explained | 14 agree, 5 disagree, 9 measured only; each explained below | **met**; only 4 of the 14 agreed cases fixed: 9 left, 1 moved against its line, each with its reason |
| Always-Consolidate behind growth | -2 ±10 at c2, -8 ±9 at c3 | **not met**: not a price problem |

**The baseline had moved.** 054 read 69 / 71 on 48 seeds. On 160 fresh seeds r23 with 054's planner is
**79 ±3 / 75 ±3**, above the band at c2. So the pass had room to take the game down, which is why
`needs` could go in without help. Before, on those seeds: tier-2 rush 86 / 82 (gap -7 ±4, -8 ±4),
always-Consolidate 79 / 84 (0 ±4, -9 ±4). After, on other seeds (9000+, 48), rush 75 / 60. The final
content was never paired with the base on one seed set: this is a before and after across seed sets, not
a measured drop.

| fresh, 48 a cell, seeds 9000+ | c2 planner | c3 planner | c2 rush | c3 rush | c2 Consolidate | c3 Consolidate |
|---|---|---|---|---|---|---|
| win % | **67** | **65** | 75 | 60 | 69 | 73 |
| gap, planner minus check | | | -8 ±8 | +4 ±8 | -2 ±10 | -8 ±9 |
| wins: purge / last Unity card | 7 / 25 | 7 / 24 | 7 / 29 | 5 / 24 | 22 / 11 | 30 / 5 |
| losses: Discord / presence / cap | 4 / 11 / 1 | 7 / 10 / 0 | 0 / 12 / 0 | 8 / 11 / 0 | 0 / 13 / 2 | 0 / 8 / 5 |
| presence lost a game | 14.2 | 22.0 | 12.3 | 22.3 | 9.8 | 11.0 |

Tier-1 and tier-2 cards per game barely moved (across seed sets): planner played 14.9 / 7.3 at c2 and 24.6 / 11.2 at c3,
from 13.8 / 8.3 and 23.6 / 12.9. Agent time is unchanged (ms a decision 42 and 80 before, 39 and 79
after): the speed check has nothing to report, and its baseline is re-recorded for r24.

## What moved the game

Paired on seeds from 7000 against the base; the planner's and the rush's win %, c2 / c3, 48 seeds for sweep 1, 40 for sweep 2, ±8-10.

**Seed sets (062):** the base rows and the 160-seed baseline above are not the same read, and the before
and after figures above are 7000+ against 9000+, so read a before/after as across seed sets, not as paired.

| Lever | planner | rush | |
|---|---|---|---|
| Base, 48 seeds (sweep 1: the next four rows) | 77 / 67 | 85 / 77 | |
| Dearer tier-2 one-shots (7 cards, +2 to +4) and the confident tier-1 fixes | 65 / 62 | 71 / 62 | down together |
| Nine `needs`, three on Sway cards | 46 / 62 | 46 / 35 | separates; breaks c2 |
| Both | 46 / 48 | 27 / 35 | too far |
| Both, and 15 tier-1 cuts of 1-2 credits | 44 / 44 | 25 / 33 | **the cuts did nothing** |
| Base, 40 seeds (sweep 2: the next three rows) | 80 / 65 | 90 / 78 | subtract these, not the row above |
| Six `needs`, none on Sway cards | 65 / 70 | 72 / 58 | **the base of what shipped** |
| Those, half the price rises, the tier-1 fixes | 52 / 55 | 62 / 65 | price costs the planner more |
| Those, and presence payoffs (Grid Census 8, Field Sensor 2, Beamed 8) | 48 / 58 | 65 / 60 | no help; not shipped |

**Why `needs` and not credits.** The rush plays 8 tier-1 cards a game to the planner's 14 (at c3, 12 to
24), so tags in play are what it lacks. Both agents spend credits on the same one-shots: the rush plays
10 tier-2 cards to the planner's 8, so a credit rise costs the rush only about a fifth more.

**Tier 2 is a dozen one-shots.** Coalition Charter, Planetary Referendum, Node Severance, Martial
Cordon, Mesh Intrusion Pact, Great Liberation, Kinetic Interdiction, Asteroid Salvage Rights, Liberation
Accords and Exodus Corridors are each played in over half of games. All 17 tier-2 buildings and
Recursive Self-Improvement are played in at most 1% of games, and kept in 2-96%. "Tier 2 is simply
better" meant these one-shots.

**Always-Consolidate is not a price problem.** Its wins are purges (22 of 33 at c2, 30 of 35 at c3; the
planner's 7 of 32 and 7 of 31), and it loses half the presence. What it has is credits, energy and few
pieces for Discord cards to hit, and it spends them clearing the board for the purge (no corrupted
cities, nodes, troops or adversary ambassadors). The bundle comes from the growth rule. Payoffs for presence moved nothing,
and dearer presence cards (Pylon Drop 2, Relay Mast 3, Brownout 3, Transmission 3, on the 8000 read, c3
gap -25 ±8) did not touch it either. Develop's draw is a blind tier-1 card, so it gains from tier-1
prices, and tier-1 prices moved nothing. **To answer it, look at the Consolidate bundle or the purge
(rules, out of scope here).**

**The 8000 read, for the record.** The candidate with those four dearer presence cards read planner 73 /
58, rush +2 ±9 / -6 ±10. Those four were never on both lists, so they came out, and the verdict is the
9000 read alone.

## Gate

gate-r23 on seeds from 10000: 432 of 512 games, 27 min on 8 threads, 0 panics. No one-seed preflight:
the same binary had just played 288 games, and 4 min mattered at that point. **Not passed by the tool's
reading**, which fails any check level or ahead on the sign. The planner itself read 50 at c2 on 8 seeds
(67 at 48), so most of the 10 level-or-ahead checks at c2 (4 at c3) are that one number.

The checks level or ahead at 8 seeds, at 48, paired on the verdict seeds; gap = planner minus check, ± one SE (67 / 65). A blank is a size where the check was behind at 8 seeds.

| check | c2 | c3 |
|---|---|---|
| tier-2-rush | -8 ±8 | +4 ±8 |
| always-Consolidate | -2 ±10 | -8 ±9 |
| no-powers | -6 ±8 | -4 ±8 |
| ignore-Science | +8 ±8 |  |
| ignore-Industry | +15 ±9 |  |
| ignore-Cyber | +23 ±9 | +17 ±8 |
| ignore-Security | +27 ±9 |  |
| keep two | +29 ±9 |  |
| science-board | +17 ±9 |  |
| Unity farm | +54 ±8 |  |
| always-Expand |  | +21 ±7 |


**Three checks stay level at 48 seeds:** tier-2-rush and always-Consolidate (above) and no-powers (-6 ±8,
-4 ±8), which is AI-board work, out of scope, and level at 053 too. **None is ahead beyond one SE**, and
at 053's reading no check is beaten. Ignoring Industry went from +4 ±9 at c2 (053) to +15 ±9, ignoring Cyber
from +8 ±9 to +23 ±9. Ignoring Science is +8 ±8. **This does not show the gates made tags matter (062):** the
comparison crosses 054's planner change, so the planner moved as well as the rules, and the claim first
written here is unsupported. Those checks were never read at r23 with 054's planner.

## The reasoning list

Written before any simulation was read, from the card alone. The value side was priced by tag, then
cross-checked between tags (40 corrections). **First pass, with insight at 4: 85 of 150 too dear, 37 of
42 tier 2.** That is a broken rate, not a broken set: unspent insight is lost each round, so credits bind,
so insight dropped to 2.5, which centers tier 1 exactly (median gap 0.0). **Tier 2 then still reads 8.7
dear on paper**, against a measure that says it wins. The disagreement is real and is the biggest single
finding of the list: rates price a card's whole catalogue, and the agents only play the 15 cheap
one-shots. A blanket tier-2 reading says nothing; use the outliers within the tier.

Fair is within 2 credits at tier 1 and 3 at tier 2. Tier 1: 22 too cheap, 25 too dear, 61 fair. Tier 2:
4 too cheap, 32 too dear, 6 fair. Reasoned 0 or less means the non-credit cost alone exceeds the value,
so credits can't fix it. Value lines are from the first pass; the cost side was rebased. Insight income stayed at 8 / 6, built at 4, so
those rows mix two rates. Rates: top of
[docs/card-pricing.md](../../../docs/card-pricing.md), with the ones that were missing (Sway, tag, gift,
presence, income per tag).


**Tier 1, too cheap (22).** Printed (credits + insight), reasoned credits, the value.

| card | printed | reasoned | value, from the card |
|---|---|---|---|
| Field Sensor Grid | 2 + 0 | 9 | Value: clear 2 x 6 x 0.7 (8.4) + insight 4 (any presence) + tag 1 = 13.4 |
| Benchmark Suite | 2 + 0 | 9 | Value insight 4 + 2 x 4 = 8 + tag 1 = 13 |
| Synthetic Data Forge | 4 + 1 | 11 | Value: 12 Datasets = 3 uses at 8, x 0.7 = 16.8 |
| Bonded Warehouse | 5 + 1 | 10 | Value unchanged at 15.35 |
| Materials Hedge Desk | 3 + 1 | 8 | Value unchanged at 13.25 |
| Shadow Broker Feed | 2 + 0 | 7 | Value clear 6x0.7=4.2 + blind draw 4 + tags 1.5=9.7. 9.7-3=6.7, rounds to 7. A second Science tag at no insight on top |
| Open Assembly | 2 + 1 | 7 | Cost 2 + 4 insight + keep 3 = 9. 3 uses of Sway 4 + harden 1.5 = 5.5 |
| Brownout Relief | 2 + 1 | 6 | Value gift (4 + 4) x 0.75 = 6 + presence 4 + tags 1.5 = 11.5 |
| Shared Exploit Kit | 3 + 0 | 7 | Value drain 3.3 + gift 3 + credit 1 + 3 tags 2 = 9.3 |
| Joint Fabrication Pact | 1 + 0 | 5 | Value 2 chips 3 + gift 3 x 0.75 = 2.25 + 3 tags 2 = 7.25 |
| Peace Dividend | 2 + 0 | 6 | Value: Sway 2 = 8 x 0.85 (a clean city holds an adversary ambassador once arrivals start) + 3 credits x 0.15 = 7.25 + tag 1 = 8.25 |
| Open Preprint | 1 + 0 | 4 | Value: draw 4 + gift 1 insight 4 x 0.75 (3) = 7, event so no tag |
| Pylon Drop | 1 + 0 | 4 | Value: 1 energy 2 + presence 4 + tag 1 = 7. 7 - 3 = 4. The energy can pay the step |
| Grad Student Pool | 3 + 1 | 6 | Value: 2 + 6 Datasets = 4 draws at 4, x 0.7 = 11.2 |
| Breakthrough Leak | 2 + 1 | 5 | Value: draw 3 x 4 = 12, event so no tag |
| Blackstart Protocol | 1 + 0 | 4 | Value: 2 energy 4 + harden 1 1.5 + tag 1 = 6.5. 6.5 - 3 = 3.5, rounds to 4. Good in any round: the energy always pays, and with no building to harden  |
| Grid Census | 1 + 1 | 4 | Value: about 4 presence at play x 2 = 8 + tag 1 = 9. 9 - 7 = 2. Weak in round 1 (1 starting presence), reaches the cap of 5 (value 11, reasoned 4) if  |
| Diplomatic Expulsion | 2 + 0 | 5 | Value remove 1 worth 2 x 0.9 (presence country only) = 1.8 + Sway 4 + tags 1.5 = 7.3 |
| Citation Engine | 4 + 1 | 6 | Value: its own tag plus the needed tag give 2 at once, about 2.5 on average up to max 3, x 5 = 12.5 |
| Grid-Tied Smelter | 4 + 1 | 6 | Value unchanged at 11.5 |
| Capacitor Discharge | 0 + 0 | 2 | Value: 2 energy 4 + 1 credit 1 = 5, no tag (event). 5 - 3 = 2. Never dead, but a keep buys only tempo |
| Joint Security Compact | 3 + 1 | 6 | Value gift (4 + 4) x 0.75 = 6 + harden 3 + tags 1.5 = 10.5 |

**Tier 1, too dear (25).** Printed (credits + insight), reasoned credits, the value.

| card | printed | reasoned | value, from the card |
|---|---|---|---|
| Intrusion Countermeasures | 3 + 1 | 1 | Value drain 3 x 1.1 = 3.3 + jam 1 + tags 1.5 = 5.8 |
| Lights-Out Assembly | 3 + 1 | 1 | Value unchanged at 5.75 |
| Adaptive Malware | 3 + 1 | 1 | Value 0.5x(remove 2: 5)+0.5x(drain 4: 4.4)=4.7, +1 tag=5.7. 5.7-7=-1.3, floor 0. The fallback means it almost always has a target, but the insight alo |
| Shared Compute Grant | 2 + 1 | -0 | Value 2 + move 1.5 + tag 1 = 4.5 |
| Flash Exploit | 3 + 1 | 0 | Value drain 5 (5.5), event so no tag. 5.5-7=-1.5, floor 0 |
| Neural Mirror | 4 + 1 | 2 | Value insight income 8 + tags 1.5=9.5. 9.5-10=-0.5, floor 0. Dead after round 6: tier-2 timing gives 6 for the insight |
| Darknet Exchange | 4 + 1 | 1 | value now: mean 1.5 x 0.8 = 1.2 x 5 = 6. Tag 1 - slot 1 = 0 |
| Baseload Contract | 4 + 1 | 1 | Value: energy income 5 + tag 1 = 6. 6 - 7 non-credit = -1, floored to 0. Pure income pays only if played by round 3 |
| Storm Hardening | 1 + 1 | -2 | Value: shelter 4 + harden 2 x 1.5 = 3, no tag (event) = 7. 7 - 11 floors to 0. Fires only in a round with a strike coming |
| Security Contractor HQ | 4 + 1 | 1 | Mean 1.5 x 0.8 = 1.2 x 5 = 6 |
| Faraday Yard | 5 + 1 | 2 | Value: 1 energy 5 + action (2 x 1.5 - 2 = 1) x 5 uses x 0.7 = 3.5, x 0.5 for targets = 1.75, + tag 1 - slot 1 = 6.75. 6.75 - 7 floors to 0 |
| Embassy Compound | 5 + 1 | 2 | Value: 2 ambassadors 6 + 2 tags 1.5 - slot 1 = 6.5 |
| Collateralized Grid | 3 + 1 | -1 | Value unchanged at 4.3 |
| Breach and Clear | 3 + 1 | -1 | Value remove 2 worth 5 + 0.6 x 2 credits on a troop (1.2) = 6.2, event so no tag. 6.2 - 9 = -2.8: the insight is too high for one removal |
| Amnesty Decree | 4 + 1 | 0 | Value: 2 ambassadors at 2 each x 0.9 (needs a corrupted city in reach) = 3.6 + move 1.5 = 5.1, event so no tag. 5.1 - 7 is below 0, so the insight + k |
| Hardened Bunker Complex | 5 + 1 | 1 | Value energy income 5 + tag 1 - slot 1 + Harden access 1 = 6 |
| Kill Team Roster | 4 + 1 | 0 | 4 + energy 2 + insight 4 + keep 3 = 13. 6 Agents by end buys 2 firings, in rounds 6 and 9: 5 x 2 x 0.7 = 7 |
| Rootkit Cache | 2 + 0 | -2 | value 0. -3, floor 0. Written for node_compute 3 |
| Threat Triage | 4 + 1 | -2 | Value jam 2 + tag 1 = 3 |
| Agency Field Station | 4 + 1 | -2 | Value tag 1 - slot 1 + engine 2.5 |
| Honeypot Relay | 3 + 1 | -5 | value 0. 0-7, floor 0. The action only matches the 1-energy standard action, so the card buys nothing but a tag |
| Demand Response Grid | 3 + 1 | -5 | Value 0. 0 - 7 floors to 0 |
| Interceptor Battery | 5 + 1 | -4 | Value tag 1 - slot 1 + 1.5 for on-demand removal with spare energy = 1.5. It would need a 2-energy use, giving (5 - 4) x 5 x 0.7 = 3.5 |
| Intrusion Lab | 5 + 1 | -4 | value 0.7. 0.7-7, floor 0 |
| Sentinel Mesh | 5 + 1 | -4 | value 0.5. 0.5-7, floor 0 |

**Tier 2, too cheap (4).** Printed (credits + insight), reasoned credits, the value.

| card | printed | reasoned | value, from the card |
|---|---|---|---|
| Open-Weights Release | 4 + 1 | 18 | Value clear 2 x 6 = 12 + shield 8 + gift 2 insight 8 x 0.75 = 6 + 2 tags 1.5 = 27.5 |
| Beamed Power Constellation | 5 + 2 | 15 | Value: 3 free presence 5 x 3 = 15 + 6 energy at cap 12 + tag 1 = 28. 28 - 19.5 = 8.5, rounds to 9. Pays at once, so never dead late |
| Asteroid Salvage Rights | 5 + 2 | 12 | Value: 10 credits + 4 chips 6 + antimatter 9 = 25 |
| Great Liberation | 14 + 2 | 21 | Value: 2 cities 50 x 0.4 (both in one country and reach) + city + cube 29 x 0.6 = 37.4 + Sway 4 = 41.4, event so no tag |

**Tier 2, too dear (32).** Printed (credits + insight), reasoned credits, the value.

| card | printed | reasoned | value, from the card |
|---|---|---|---|
| Adversarial Inference Sweep | 5 + 1 | 2 | Value: every project 1.1 x 1.5 (1.65) + 2 delays 5 + draw 4 = 10.65 |
| Aegis Grid Lockdown | 5 + 2 | 2 | Value: shelter 4 + about 2 of 4 hardens 3 + 2 free presence (3 + 2) x 2 = 10, no tag = 17. 17 - 22 floors to 0 |
| Martial Cordon | 8 + 1 | 4 | Value 4 ambassadors x 2 = 8 + 1 Sway 4 + move 1.5 + tags 1.5 = 15 |
| Killswitch Directive | 10 + 2 | 6 | Value discard 15-20, midpoint 17.5, event so no tag. 17.5-18=-0.5, floor 0. Reliable in late rounds: it can be held for a big stage-2 project |
| Node Severance | 6 + 2 | 0 | Value remove a node 12, event so no tag. 12-18=-6, floor 0 |
| Kinetic Interdiction | 6 + 2 | 0 | Value remove 3 worth 6 x 3 full hexes = 18 + partial hexes about 3 = 21, event so no tag |
| Exodus Corridors | 7 + 2 | 0 | Value: 2 ambassadors 6 + multi-move 1 each, assume 4 on board = 4 + 2 tags 1.5 = 11.5 |
| Liberation Accords | 9 + 2 | 2 | Value: all around a city 2 each, assume 4 = 8 + 2 ambassadors 6 + tag 1 = 15 |
| Sovereign Capital Fund | 7 + 2 | -0 | Value: 6 Capital / 3 = 2 antimatter x 9 x 0.7 = 12.6 + tag 1 - slot 1 = 12.6 |
| Foundation Model Campus | 15 + 2 | 6 | Value: 2 x 6 insight (12) + 5 tags x 3 (15) + draw 2 (8) + 2 tags 1.5 = 36.5 |
| Full Automation Mandate | 12 + 1 | 4 | Value: 4 x 3 = 12 + chip income 2.5 + tag 1 = 15.5 |
| Planetary Assembly | 12 + 2 | 3 | Cost 12 + 8 insight + keep 10 + 1.5 chip + 3 lowered income + needs 3 + replaces 2 = 39.5. 3 uses of Sway 2 = 8 + ambassador 3 = 11 |
| Orbital Observatory Array | 9 + 2 | 0 | Value: insight 6 + 4-tag insight 6 x 0.4 (2.4) + 1 use draw 2 8 x 0.7 (5.6) + 2 tags 1.5 - slot 1 = 14.5 |
| Recursive Self-Improvement | 10 + 2 | -1 | Value: 2 x 6 insight (12) + draw 3 (12) + 2 tags 1.5 = 25.5 |
| Total Containment | 9 + 2 | -2 | Value 1 troop 5 + 3 ambassadors 2 each = 6 -> about 11 + shelter 4 = 15, including 0 for its no-reach hex |
| Cascade Failure | 9 + 2 | -2 | Value drain 5 x1.1 x1.5=8.25 + tag 1=9.25. 9.25-18, floor 0 |
| Hyperscale Botnet | 13 + 2 | 1 | Value insight income 6 + 4 Cyber tags x3=12 + tag 1=19, no slot charge. 19-24.5=-5.5, floor 0. At the max 6 tags, value 25, reasoned 0.5 |
| Continental Supergrid | 10 + 2 | -2 | Value: about 3.5 Power tags x 3.5 = 12.25 + harden passive 1 + tags 1.5 - slot 1 = 13.75. 13.75 - 22.5 floors to 0 |
| Arc Furnace Complex | 10 + 2 | -3 | Credits 6 x 3 = 18, less energy 2 x 3.5 = 7, = 11 + chip income 2.5 + tags 1.5 = 15 |
| Deep State Ledger | 8 + 2 | -6 | Value 5.5 is under a third of the total |
| Singularity Exploit | 14 + 2 | -1 | Value remove node 12 + drain 4 x1.1 x1.5=6.6 + tag 1=19.6. 19.6-27=-7.4, floor 0 |
| Positron Storage Ring | 11 + 2 | -4 | Value: 1 net energy 3.5 + antimatter income 12 + tag 1 = 16.5 (4 energy 14, less 3 spent 10.5, plus 12). 16.5 - 27 floors to 0 |
| Exabyte Archive | 8 + 2 | -7 | Value: Action 1+2+3+4 with up to 2 more credits from another holder = 12 x 0.7 = 8.4 + tag 1 = 9.4 |
| Orbital Embassy | 14 + 2 | -2 | Value: 4 credit income 12 + insight income 6 + 2 ambassadors 6 + 2 tags 1.5 = 25.5, no slot charge since it replaces |
| Agency Directorate | 12 + 2 | -4 | 1.5 firings x 14 x 0.7 = 14.7 + tag 1 = 15.7 |
| Arcology Foundry | 16 + 2 | 0 | Value: 5 x 3 = 15 + 2 x 2.5 = 5 + tag 1 = 21 |
| Threat Neutralization Mandate | 7 + 2 | -9 | Value counter 7 + tag 1 = 8 |
| Black Ice Citadel | 9 + 2 | -8 | Value credit income 3 + action (4.4-2)x2.5 uses x0.7=4.2 + tags 1.5=8.7, no slot charge. 8.7-23=-14.3, floor 0 |
| Aegis Dome Array | 10 + 2 | -8 | Value civic shelter 4 x 0.6 x 3 rounds = 7 + spread skip 1 x 3 = 3 + tags 2 - slot 1 = 11 |
| Antiproton Offtake Deal | 8 + 2 | -10 | Value: antimatter income 12 + tag 1 = 13. 13 - 30 floors to 0. Late, with about 3 incomes |
| Dyson Swarm Tranche | 16 + 2 | -10 | Value: 5 energy x 3.5 = 17.5 + 2 credits x 3 = 6 + tags 1.5 = 25. 25 - 41 floors to 0 |
| Orbital Mass Driver | 25 + 2 | -2 | Value: 7 x 3 = 21 + 2 x 2.5 = 5 + discount 3 + tags 1.5 = 30.5 |


## The two lists compared

**Both lists flag it.**

| card | t | printed | reasoned | reasoning says | measured, against its tier |
|---|---|---|---|---|---|
| Great Liberation | 2 | 14 + 2 | 21 | too cheap | +20 (kept, z 3.8) |
| Black Ice Citadel | 2 | 9 + 2 | -8 | too dear | -20 (kept, z -3.2) |
| Full Automation Mandate | 2 | 12 + 1 | 4 | too dear | -14 (kept, z -3.0) |
| Beamed Power Constellation | 2 | 5 + 2 | 15 | too cheap | +14 (played, z 2.9) |
| Rootkit Cache | 1 | 2 + 0 | -2 | too dear | -14 (played, z -2.6) |
| Planetary Assembly | 2 | 12 + 2 | 3 | too dear | -13 (kept, z -2.5) |
| Arcology Foundry | 2 | 16 + 2 | 0 | too dear | -11 (kept, z -2.4) |
| Adversarial Inference Sweep | 2 | 5 + 1 | 2 | too dear | -12 (played, z -2.4) |
| Recursive Self-Improvement | 2 | 10 + 2 | -1 | too dear | -19 (kept, z -2.3) |
| Darknet Exchange | 1 | 4 + 1 | 1 | too dear | -11 (kept, z -2.3) |
| Intrusion Lab | 1 | 5 + 1 | -4 | too dear | -11 (kept, z -2.3) |
| Deep State Ledger | 2 | 8 + 2 | -6 | too dear | -24 (kept, z -2.2) |
| Grad Student Pool | 1 | 3 + 1 | 6 | too cheap | +12 (played, z 2.1) |
| Bonded Warehouse | 1 | 5 + 1 | 10 | too cheap | +10 (played, z 2.0) |

**The lists disagree.**

| card | t | printed | reasoned | reasoning says | measured, against its tier |
|---|---|---|---|---|---|
| Martial Cordon | 2 | 8 + 1 | 4 | too dear | +15 (kept, z 2.7) |
| Shared Exploit Kit | 1 | 3 + 0 | 7 | too cheap | -14 (kept, z -2.6) |
| Dyson Swarm Tranche | 2 | 16 + 2 | -10 | too dear | +13 (kept, z 2.2) |
| Open-Weights Release | 2 | 4 + 1 | 18 | too cheap | -11 (played, z -2.1) |
| Cascade Failure | 2 | 9 + 2 | -2 | too dear | +16 (kept, z 2.0) |

**The measure alone flags it.**

| card | t | printed | reasoned | reasoning says | measured, against its tier |
|---|---|---|---|---|---|
| Predictive Modeling | 1 | 2 + 0 | 2 | fair | -27 (played, z -4.2) |
| Substation Hub | 1 | 5 + 1 | 3 | fair | +21 (played, z 3.2) |
| Safehouse Network | 1 | 3 + 1 | 4 | fair | -19 (played, z -2.7) |
| Compute Siphon | 1 | 4 + 1 | 3 | fair | +13 (kept, z 2.6) |
| Fusion Startup Stake | 1 | 6 + 1 | 6 | fair | +13 (played, z 2.2) |
| Tidal Barrage | 1 | 5 + 1 | 5 | fair | +12 (played, z 2.2) |
| Endowed Research Chair | 1 | 5 + 1 | 4 | fair | +13 (kept, z 2.2) |
| Shuttle Diplomacy | 1 | 2 + 0 | 3 | fair | -13 (played, z -2.2) |
| Shared Threat Intel | 1 | 2 + 0 | 4 | fair | -10 (kept, z -2.1) |

**How the measured column reads.** Win % of games the card was kept (or played) minus not, stratified on
rounds, less its tier's mean, 160 seeds a size. A flag needs the same sign at both sizes and |z| ≥ 2;
150 cards × 2 sizes put about 15 past 2 SE by chance, so a lone flag is a lead, not a finding. Being dealt
reads as a placebo: 0 at tier 1, but +12 and +56 at tier 2, where almost every card is dealt almost
every game. Tier-2 raw deltas are poor, and only the comparison within the tier stands.

## The agreed cases

**Fixed:** Great Liberation, Beamed Power, Rootkit Cache, Darknet Exchange. Peace Dividend (+9, z 1.8)
and Security Contractor HQ (the same text as Darknet Exchange) went with them, just under the bar.

**Moved against its line:** Adversarial Inference Sweep, too dear on both lists, took a `needs` all the
same, because the rush played it 12 points more than the planner.

**Not fixed, left for Matt:**
- Black Ice Citadel, Full Automation Mandate, Planetary Assembly, Arcology Foundry, Recursive
  Self-Improvement, Deep State Ledger, Intrusion Lab: too dear on both lists, but each is played in under
  7% of games. They are kept and not played, so a price cut can't be read, and cuts to tier 2 run
  against the structural fix. This is mostly an agent's blind spot: see below.
- Grad Student Pool and Bonded Warehouse: 3-5 too cheap on both, tier 1, played 22-31%. Not swept; the
  pass ran out of sweeps.

## The disagreements, explained

- **Martial Cordon** (reasoned dear, +15). The rate prices 4 ambassadors at 2 each; it sweeps a country
  in one play, and it is played 96% of the time. The rate undervalues a sweep: use 3 each at tier-2
  timing. It stayed at 8.
- **Shared Exploit Kit** (reasoned cheap, -14). A help card at 0.75 × face; the gift lands when the other
  seat has no use for an insight. The gift rate may be high. Unchanged.
- **Open-Weights Release** (reasoned 18, -11). Clear × 0.7 and shield at 4 assume markers stand and hexes
  get hit; it is kept 47% of games and played late. Unchanged; both rates may be high for tier-2 timing.
- **Dyson Swarm Tranche, Cascade Failure** (reasoned dear, +13 and +16, on *kept*). Played 0 and 2%: being
  kept in won games is not value. Noise or reverse cause.

**Measured only, reasoned fair:** Predictive Modeling (-27), Safehouse Network (-19), Shuttle Diplomacy
(-13), Shared Threat Intel (-10), against Substation Hub (+21), Compute Siphon (+13), Fusion Startup
Stake (+13), Tidal Barrage (+12), Endowed Research Chair (+13). Income buildings reading strong on the
played line is what early, steady income would do; no pattern in the weak ones. Above the expected false
rate only just: left for a second pass, not priced.

## What price can't reach

**Kept and never played.** Cryo-Compute Burn (1 credit, kept 48% of games, played 0), Threat Triage,
Hardpoint Lockdown, Neural Mirror, Endowed Research Chair, Deep State Ledger (kept 96%, played 0), and
almost every tier-2 building. The agents keep on one value and play on another. For these cards the
measure says nothing about the price, and a real player would likely read them differently. Tier-2
income in particular has never been priced by play.

**Unasked.** Unity, Discord, threat, adversary and AI-unique cards were not priced. `needs` counted as a
price: cards.md lists it among the costs. Sway rate of 4 is reasoned, not fitted.
