# 248: price check of the standard actions against the cards for each job

Task 3 of 248 (r112 standard actions; Liberate with antimatter gone, Build a factory off). Locked rule: a standard
action sits at or under par at the unit anchors and is never the better route (Liberate exempt); Matt, 2026-11-16:
cards are the engine and beat standard actions, which are a rare fallback.

**Method.** Unit anchors (card-guidelines.md *Unit anchors*; design_check.json `extra_anchors`; values.json where
none): energy 4.5, insight 4, card drawn 5, ambassador 8, chip 6, agent 5, troop 5, antimatter 18, research cube 6,
sway 4, node 8, city liberated 45, botnet 16, compute 2, harden 3, credit 1; income +1 credit 4, +1 insight 12, +1
energy 14 (tier 1) / 9 (tier 2), +1 chip 19; keep 4 / 12 (tier 1 / 2); play insight 4 each. Units with no anchor take
values.json: an owned station 20 (`claim_station`), a factory 19 (+1 chip income), a station freed 15, a factory
freed 6, a moved ambassador 2, a project discarded 24 (card-pricing.md), a typed jam 0.7 a point, a gift 0.75 × face.
Card pay = credits + 4 × (keep + play insight) + printed energy, chips, antimatter, cubes at the anchors, read from
`content/cards.json` (deck cards only). **Unit price = (pay − the card's other effects at the anchors) ÷ job units**:
what the card charges per unit of the job; negative means its other effects already pay for the job. Repeatable cards:
4 rounds of uses (an Action 4 times, its use cost added; a trigger at 1 card of its tag a round); "a use" is the
marginal price per unit once built. Per-tag effects at 5 tags (values.json `tag_count`). Counts are printed counts
(say where the board rarely holds them). The track step's energy is paid by every node route, so it is left out of
all of them. Script: `price_cards.py` here (costs from content, readings in its table); costs checked against
`docs/cards.md`.

## Summary

| Action | Its give / get at the anchors, ratio | Cards for the job: count; best, median per unit | Standard per unit | Flag |
|---|---|---|---|---|
| Place a node | 6 cr (+ step) / node 8: **1.33** without the step, 0.76 with 1 step energy (design_check's row) | 4 cards: -0.7, 1.2 a node; powers 3.1-4.7 | 6.0 | over par on the powers' reading (step left out); 5× the median card |
| Build a research station | 1 chip 6 / station: 1.00 at the station anchor 6, **3.3** at an owned station's 20 | **0 cards** (Field Station power only) | 6 | **gap; cheapest route to an owned station** |
| Remove agents | 2 amb 16 / 3 agents 15: 0.94 | 23: -5.2, 2.0 an agent | 5.3 | none (2.7× median) |
| Remove a troop | 10 cr / troop 5: 0.50 | 7: -9.5, 4.0 a troop; powers 3.0 | 10.0 | none (2.5×) |
| Lift Fallout | 2 cubes 12 / lift 12: **1.00** (at the cap) | 5: -3.8, 2.0 a lift | 12.0 | at par; 6× median |
| Dispatch | 1 Dispatch (board allotment, not bought) / an ambassador onto a piece (2-6) | 5 movers; all pay their moves from their ambassador gain (≤0 a move) | free | not comparable (allotment) |
| Gain a player ambassador | 20 cr / amb 8: 0.40 | 17: 0.2, 2.0 an ambassador | 20.0 | none (10×) |
| Claim a city | 5 amb 40 / +1 energy income 14 (+ a site for Unity cards): ~0.35-0.45 | **0 cards** | 40 | **gap** (only route; harmlessly dear) |
| Claim a station | 3 amb 24 / owned station 20: 0.83 | 2: 15.0, 19.5 a station | 24.0 | borderline: 1.23× median, ties Microgrid Lab Compact at 2 uses |
| Claim a factory | 2 amb 16 / factory 19: **1.19** | 1: 3.5 (Prefab Plant Kit) | 16.0 | **over par** (not near the card) |
| Liberate a city (exempt) | 5 amb 40 / city 45: 1.13 | 9: 28.0, 38.4 a city | 40.0 | **near: 1.04× median; ties Great Liberation, beats 3 of 9** |
| Liberate a station / factory | 3 amb 24 / station 20 + 2 Sway 8: **1.17**; 2 amb 16 / factory 19 + 8: **1.69** | freeing cards (to neutral): Reclamation Crew 16, Back-Channel Talks 15 for 2 | 24 / 16 | **over par, better route than the cards; shares the city's exemption without its reason** |
| Take down a botnet | 16 cr / botnet 16: **1.00** | 3 (all tier 2): 14.0, 15.0 a botnet | 16.0 | **near: 1.07× median; the practical route** |
| Buy a city's energy | 5 cr / energy 4.5: 0.90 | 13: 0.3, 3.3 an energy | 5.0 | none (1.5×) |
| Sell a card | card 5 / 1 cr: 0.20 | **0 cards** turn a hand card into anything (sacrifice powers take played cards) | 5 a credit | gap, harmless (a dump) |
| Turn a cube into insight | cube 6 / insight 4: 0.67 | 10: 1.2, 2.4 an insight | 6.0 | none (2.5×) |
| Envoy | amb 8 / 1 Discord off 4: 0.50 | 8: -1.5, 2.6 a sway | 8.0 | none (3.1×) |
| Make antimatter | 5 cubes 30 / antimatter 18: 0.60 | 8: 2.0, 10.0 an antimatter; burns 27-28 | 30.0 | none (3×) |

## Flags

1. **Liberate a city (exempt), near.** 40 a city against the cards' median 38.4: as cheap as the median liberate card,
   tied with Great Liberation, cheaper than Peacekeeping Mission (42.5), The Nuclear Option (44) and Citywide Microgrid
   (46). By Matt's 2026-11-16 direction the cards should beat it; today only Hostile Buyout (28), Popular Uprising
   (30), Power to the People (32) and Forensic Audit (36) do clearly. The exemption is the open question, not a bug.
2. **Liberate a station or factory, over par.** The exemption is argued for the city (the win route), but the same
   action liberates a station (1.17) or factory (1.69) and makes it yours with Sway. The cards that free facilities
   (Reclamation Crew, Back-Channel Talks, the Protector's Reclaim) only turn them neutral, so the standard action is
   the better route to an owned facility. design_check's Liberate row prices only the city.
3. **Take down a botnet, near.** 16 against 14-15 on the three cards, all tier 2, two paying antimatter, each needing
   Cyber or Security tags: in practice the standard action is the route. Its ratio sits exactly at the 1.0 cap because
   the botnet anchor (16, "past checks") was set from this price.
4. **Build a research station: a gap and an anchor mismatch.** No deck card builds a station (only the Researcher's
   Field Station). The design_check anchor for a station (6) equals its 1-chip price, while values.json prices an
   owned station 20 (`claim_station`; 8 for a free build) and Claim charges 3 ambassadors (24), Power Purchase
   Agreement 15. A built station is owned like a claimed one, so on any reading but the 6, building is far the cheapest
   route to an owned station (bounded by open hexes in reach and chips).
5. **Claim a factory over par** (1.19: 2 ambassadors for +1 chip income 19; r87, Matt: an early factory is worth
   claiming). It stays 4.6× dearer than Prefab Plant Kit per factory, so cards are still the better route. design_check
   has no Claim row, so `standard-over-anchor` never sees it.
6. **Place a node depends on the step.** design_check counts 1 step energy on the standard side (0.76); ai-powers.md
   prices Buy In and Accord without the step (a node 8 + rider for 9). On that reading Place a node is 1.33, over par;
   either way it is 5× the median card per node and 1.3-1.9× the node powers.
7. **Gaps (the standard action is the only route):** Claim a city (no card claims a neutral city; Accord's rider puts
   an ambassador toward it), Build a research station (flag 4), Sell a card. Claim a city and Sell a card are dear, so
   harmless.
8. **Claim a station, borderline:** 24 a station, 1.23× the two cards' median; equal to Microgrid Lab Compact when it
   claims 2 stations, 1.6× Power Purchase Agreement.

Not flagged: Remove agents, Remove a troop, Lift Fallout (at the cap but 6× the cards), Gain an ambassador, Buy
energy, Turn a cube into insight, Envoy, Make antimatter: each at 1.5-10× the median card per unit.

**design_check rows.** The 12 live rows match baseline.json's values (station_ambassadors 0). Missing: Dispatch,
Claim (city 5, station 3, factory 2) and Liberate's station and factory sizes; a Claim-factory row would trip
`standard-over-anchor` (1.19). The working tree's economy.md generated table still lists Liberate with antimatter
(not yet regenerated).

**Side finding (not this item's job): cards over ruling 22's ceiling at these readings.** Aegis Grid Lockdown 3.50
(4 credits for 3 nodes and a shelter; each node still pays its step), Field Sensor Grid 2.83 (at 3 lifts; Fallout
rarely stands 3 near, 1.50 at 1), National Grid Buildout 2.00, Diplomatic Expulsion 2.00, Ceasefire Monitors 1.94,
Prefab Plant Kit 1.91, Relay Mast Rollout 1.86, Envoy Briefings 1.82, Intrusion Lab 2.40 at tier 1; Embassy of Exiles
3.05 and Open Source Uprising 2.64 at tier 2 (convert counted as an agent removed plus an ambassador placed).

## Per-job detail

Each line: card (printed cost): pay; what it does; other effects at the anchors; get ÷ pay; unit price.
Powers are in italics, their pay the power's own cost (a tag sacrifice unpriced). The table at the end is the
script's.

### Place a node: 6 cr (+ the track step, every route) for node 8; 6.0 per node

- **National Grid Buildout** (22 cr, T1 keep 1, play 0 ins): pay 26.0; place 3; +2 energy income 28; other 28.0; ratio 2.00; **-0.7 per node**
- **Beamed Power Constellation** (14 cr, T2 keep 3, play 1 ins): pay 30.0; place 3; 1 energy a node (~6 nodes) 27; other 27.0; ratio 1.70; **1.0 per node**
- **Aegis Grid Lockdown** (4 cr, T1 keep 1, play 0 ins): pay 8.0; place 3; shelter (values 4); other 4.0; ratio 3.50; **1.3 per node**
- **Relay Mast Rollout** (6 cr, T1 keep 1, play 1 ins): pay 14.0; place 2; draw 2; other 10.0; ratio 1.86; **2.0 per node**
- *Buy In (Builder)* (power): pay 9.0; 2 energy; chip by a fresh factory (72%); **4.7 per node**
- *Accord (Protector)* (power): pay 9.0; 2 energy; ambassador by a fresh city (72%); **3.2 per node**
- *Peer Review (Researcher)* (power): pay 6.0; 1 cube; insight by a fresh station (72%); **3.1 per node**

### Remove agents: 2 amb 16 for 3 agents 15; 5.3 per agent

23 cards; the best 4, the median and the worst:

- **Open Source Uprising** (9 cr, T2 keep 3, play 1 ins): pay 25.0; convert 4 (32); draw 2; sway 1; other 46.0; ratio 2.64; **-5.2 per agent**
- **Double Agents** (8 cr, T2 keep 3, play 0 ins): pay 20.0; convert 3 (24); 2 troops 10; other 34.0; ratio 2.45; **-4.7 per agent**
- **Mass Defection** (18 cr, T2 keep 3, play 0 ins): pay 30.0; convert 5 (ambassadors where they stood 40); sway 2; other 48.0; ratio 2.43; **-3.6 per agent**
- **Liberation Accords** (11 cr, T2 keep 3, play 2 ins): pay 31.0; 6 agents; 4 ambassadors 32; other 32.0; ratio 2.00; **-0.2 per agent**
- **Diplomatic Expulsion** (8 cr, T1 keep 1, play 0 ins): pay 12.0; 4 agents; sway 1; other 4.0; ratio 2.00; **2.0 per agent**
- **Field Assay Team** (6 cr, T1 keep 1, play 1 ins): pay 38.0; Action 1 cube: 2 agents, x4; other 0.0; ratio 1.05; **4.8 per agent**; a use 3.0 per agent

### Remove a troop: 10 cr for troop 5; 10.0 per troop

- **Double Agents** (8 cr, T2 keep 3, play 0 ins): pay 20.0; 2 troops; convert 3 (39); other 39.0; ratio 2.45; **-9.5 per troop**
- **Kinetic Interdiction** (0 cr, 1 am, T2 keep 3, play 0 ins): pay 30.0; 2 troops; 6 agents 30; Discord -4; other 26.0; ratio 1.20; **2.0 per troop**
- **Kettle Operation** (2 cr, T2 keep 3, play 2 ins): pay 22.0; ~2 troops (p90 3 a country) + 4 agents 20; -4; other 16.0; ratio 1.18; **3.0 per troop**
- **Breach and Clear** (6 cr, T1 keep 1, play 1 ins): pay 14.0; 1 troop; draw 2; other 10.0; ratio 1.07; **4.0 per troop**
- **Decapitation Strike** (0 cr, 1 am, T2 keep 3, play 0 ins): pay 30.0; 3 troops; botnet 16; other 16.0; ratio 1.03; **4.7 per troop**
- **Annihilation Lance** (8 cr, 1 am, T2 keep 3, play 0 ins): pay 38.0; ~1 on it + 2 near; ~6 agents 30; Discord -8; other 22.0; ratio 0.97; **5.3 per troop**
- **Surgical Strike** (2 cr, T2 keep 3, play 0 ins): pay 14.0; 3 troops (or 4 agents); Discord -4; other -4.0; ratio 0.79; **6.0 per troop**
- *Fortify (Builder)* (power): pay 6.0; 1 chip: 1 troop; harden 3; **3.0 per troop**
- *Disrupt (Researcher)* (power): pay 6.0; 1 Cyber sacrifice + 1 cube: up to 2 (sacrifice unpriced); **3.0 per troop**

### Lift Fallout: 2 cubes 12 for Fallout lifted 12; 12.0 per Fallout lifted

- **Shared Grid Accord** (7 cr, T1 keep 1, play 0 ins): pay 11.0; lift 1; gift 2 energy 6.75; sway 2; other 14.8; ratio 2.43; **-3.8 per Fallout lifted**
- **Field Sensor Grid** (6 cr, T1 keep 1, play 2 ins): pay 18.0; lift 3 near; draw 3; other 15.0; ratio 2.83; **1.0 per Fallout lifted**
- **Fusion Startup Stake** (24 cr, T1 keep 1, play 0 ins): pay 28.0; lift 1; +1 energy, +3 credit income 26; other 26.0; ratio 1.36; **2.0 per Fallout lifted**
- **Open-Weights Release** (16 cr, T2 keep 3, play 1 ins): pay 32.0; lift 4 near; gift 6 insight 18; other 18.0; ratio 2.06; **3.5 per Fallout lifted**
- **Pumped Hydro Storage** (28 cr, T1 keep 1, play 0 ins): pay 32.0; lift 2 near; +5 credit income 20; other 20.0; ratio 1.38; **6.0 per Fallout lifted**

### Gain a player ambassador: 20 cr for amb 8; 20.0 per ambassador

17 cards; the best 4, the median and the worst:

- **Embassy of Exiles** (30 cr, T2 keep 3, play 0 ins): pay 42.0; 3; Action convert 2 (agents 10 + 2 placed), x4; other 40.0; ratio 3.05; **0.2 per ambassador**
- **Liberation Accords** (11 cr, T2 keep 3, play 2 ins): pay 31.0; 4; 6 agents 30; other 30.0; ratio 2.00; **0.2 per ambassador**
- **Ceasefire Monitors** (12 cr, T1 keep 1, play 0 ins): pay 16.0; 2; 3 agents 15; other 15.0; ratio 1.94; **0.5 per ambassador**
- **Envoy Briefings** (9 cr, T1 keep 1, play 1 ins): pay 17.0; 2; draw 3; other 15.0; ratio 1.82; **1.0 per ambassador**
- **Rally the Allies** (12 cr, T2 keep 3, play 0 ins): pay 24.0; 5; gather 3 (6); sway 2; other 14.0; ratio 2.25; **2.0 per ambassador**
- **Exodus Corridors** (18 cr, T2 keep 3, play 1 ins): pay 34.0; 4; gather 6 (12); other 12.0; ratio 1.29; **5.5 per ambassador**

### Take down a botnet: 16 cr for botnet 16 (Sway 0); 16.0 per botnet

- **Singularity Exploit** (4 cr, 1 am, T2 keep 3, play 1 ins): pay 38.0; botnet; discard a project 24; other 24.0; ratio 1.05; **14.0 per botnet**
- **Botnet Takedown** (11 cr, T2 keep 3, play 1 ins): pay 27.0; botnet; 6 compute 12; other 12.0; ratio 1.04; **15.0 per botnet**
- **Decapitation Strike** (0 cr, 1 am, T2 keep 3, play 0 ins): pay 30.0; botnet; 3 troops 15; other 15.0; ratio 1.03; **15.0 per botnet**

### Buy a city's energy: 5 cr for energy 4.5; 5.0 per energy

13 cards; the best 4, the median and the worst:

- **National Grid Buildout** (22 cr, T1 keep 1, play 0 ins): pay 26.0; +2 energy income (6.2); 3 nodes 24; other 24.0; ratio 2.00; **0.3 per energy**
- **Baseload Contract** (26 cr, T1 keep 1, play 0 ins): pay 30.0; +2 energy income (6.2); 2 ambassadors 16; other 16.0; ratio 1.47; **2.2 per energy**
- **Peaker Turbines** (22 cr, T1 keep 1, play 0 ins): pay 26.0; +1 energy income (3.1); +1 chip income 19; other 19.0; ratio 1.27; **2.2 per energy**
- **Capacitor Discharge** (10 cr, T1 keep 1, play 0 ins): pay 14.0; 1 a Power tag (5); other 0.0; ratio 1.61; **2.8 per energy**
- **Dyson Swarm Tranche** (12 cr, 1 am, T2 keep 3, play 1 ins): pay 46.0; 10; +2 income at tier 2 (4); other 0.0; ratio 1.37; **3.3 per energy**
- **Substation Hub** (16 cr, T1 keep 1, play 0 ins): pay 20.0; 2 when built; 1 an other player's Power play (~2); other 0.0; ratio 0.90; **5.0 per energy**

### Turn a cube into insight: cube 6 for insight 4; 6.0 per insight

10 cards; the best 4, the median and the worst:

- **Hyperscale Botnet** (18 cr, T2 keep 3, play 1 ins): pay 34.0; 4 a round x4; draw 3; other 15.0; ratio 2.32; **1.2 per insight**
- **Interdisciplinary Institute** (12 cr, T2 keep 3, play 2 ins): pay 32.0; 1 a tag played (~3 a round) x4; draw 3; other 15.0; ratio 1.97; **1.4 per insight**
- **Exascale Training Cluster** (15 cr, 5 en, T2 keep 3, play 2 ins): pay 57.5; 3 a round x4; 6 cubes 36; other 36.0; ratio 1.46; **1.8 per insight**
- **Cryo-Compute Burn** (2 cr, 1 en, T1 keep 1, play 0 ins): pay 10.5; 1 a Science tag (5); other 0.0; ratio 1.90; **2.1 per insight**
- **Neural Mirror** (12 cr, T1 keep 1, play 0 ins): pay 16.0; +1 a 2 Cyber tags (+2 at 5: 6); other 0.0; ratio 1.50; **2.7 per insight**
- **Frontier Lab Charter** (28 cr, T1 keep 1, play 0 ins): pay 32.0; +2 insight income (6); +2 credit income 8; other 8.0; ratio 1.00; **4.0 per insight**

### Envoy: amb 8 for 1 Discord off 4; 8.0 per sway to Unity

- **Antimatter Accord** (8 cr, 1 am, T2 keep 3, play 1 ins): pay 42.0; sway 4; 6 ambassadors 48; other 48.0; ratio 1.52; **-1.5 per sway to Unity**
- **Holographic Assembly Hall** (13 cr, T1 keep 1, play 0 ins): pay 53.0; Action 2 energy: sway 2 + 2 amb, x4; other 64.0; ratio 1.81; **-1.4 per sway to Unity**; a use 4.5 per sway to Unity
- **Peace Conference** (32 cr, T1 keep 1, play 0 ins): pay 36.0; sway 8; 3 ambassadors 24; other 24.0; ratio 1.56; **1.5 per sway to Unity**
- **Accelerator Diplomacy** (4 cr, 1 am, T1 keep 1, play 0 ins): pay 26.0; sway 5; 2 ambassadors 16; other 16.0; ratio 1.38; **2.0 per sway to Unity**
- **Manifesto Leak** (8 cr, T1 keep 1, play 0 ins): pay 12.0; sway 3; a Follower 2.5; other 2.5; ratio 1.21; **3.2 per sway to Unity**
- **Open Assembly** (17 cr, T1 keep 1, play 0 ins): pay 21.0; 3 at 2 Followers, 1 a Diplomacy play: ~2 fires in 4 rounds; other 0.0; ratio 1.14; **3.5 per sway to Unity**
- **Coalition Charter** (18 cr, T2 keep 3, play 1 ins): pay 34.0; sway 6; gift 4 insight 12; other 12.0; ratio 1.06; **3.7 per sway to Unity**
- **Citywide Microgrid** (8 cr, 8 en, T2 keep 3, play 1 ins): pay 78.0; Action 1 energy: sway 2, x4; liberates (city less its sway, 25); other 25.0; ratio 0.73; **6.6 per sway to Unity**; a use 2.2 per sway to Unity

### Make antimatter: 5 cubes 30 for antimatter 18; 30.0 per antimatter

- **Zero-Day Auction** (14 cr, T2 keep 3, play 2 ins): pay 34.0; 2; draw 6; other 30.0; ratio 1.94; **2.0 per antimatter**
- **Positron Storage Ring** (10 cr, 2 en, T2 keep 3, play 2 ins): pay 75.0; Action 2 energy x4; 2 energy a Power play (8); draw 2; other 46.0; ratio 1.57; **7.2 per antimatter**; a use 9.0 per antimatter
- **Intrusion Lab** (11 cr, T1 keep 1, play 0 ins): pay 15.0; 1 at 2 Operatives, 1 a compute removal: ~2 in 4 rounds; other 0.0; ratio 2.40; **7.5 per antimatter**
- **Penning Trap Array** (10 cr, T2 keep 3, play 1 ins): pay 50.0; 1 when built; Action 1 cube, x4; other 0.0; ratio 1.80; **10.0 per antimatter**; a use 4.8 per antimatter
- **Antiproton Foundry** (10 cr, T2 keep 3, play 1 ins): pay 50.0; 1 when built; Action 1 chip, x4; other 0.0; ratio 1.80; **10.0 per antimatter**; a use 4.8 per antimatter
- **Dark Fiber Collider** (6 cr, T2 keep 3, play 1 ins): pay 54.0; Action 2 insight, x4; draw 2; other 10.0; ratio 1.52; **11.0 per antimatter**; a use 8.0 per antimatter
- **Treaty Isotope Bank** (6 cr, T2 keep 3, play 1 ins): pay 54.0; Action 8 credits, x4; 1 ambassador; other 8.0; ratio 1.48; **11.5 per antimatter**; a use 8.0 per antimatter
- **Cyclotron Lease** (22 cr, T1 keep 1, play 0 ins): pay 62.0; Action 2 energy, x4; other 0.0; ratio 1.16; **15.5 per antimatter**; a use 9.0 per antimatter
- *Teardown (Builder)* (power): pay 6.0; 1 chip; your factory gone (-10) and a Fallout (-12); **28.0 per antimatter**
- *Meltdown (Researcher)* (power): pay 12.0; 2 cubes; your station gone (-15); **27.0 per antimatter**

### Liberate a city (exempt): 5 amb 40 for city 45 (its 5 Sway in); 40.0 per city liberated

9 cards; the best 4, the median and the worst:

- **Hostile Buyout** (22 cr, 1 am, T2 keep 3, play 0 ins): pay 52.0; 3 ambassadors 24; other 24.0; ratio 1.33; **28.0 per city liberated**
- **Popular Uprising** (26 cr, T1 keep 1, play 0 ins): pay 30.0; Action: a Follower, or 4 Followers: liberate (once in ~5 rounds); other 0.0; ratio 1.50; **30.0 per city liberated**
- **Power to the People** (22 cr, 4 en, T2 keep 3, play 0 ins): pay 52.0; 1 cr a Power tag a round (5) x4; other 20.0; ratio 1.25; **32.0 per city liberated**
- **Forensic Audit** (6 cr, 4 rc, T2 keep 3, play 1 ins): pay 46.0; draw 2; other 10.0; ratio 1.20; **36.0 per city liberated**
- **Root Access** (18 cr, 1 am, T2 keep 3, play 0 ins): pay 48.0; a botnet near (x0.6) ~10; other 9.6; ratio 1.14; **38.4 per city liberated**
- **Citywide Microgrid** (8 cr, 8 en, T2 keep 3, play 1 ins): pay 78.0; Action 1 energy: sway 2, x4; other 32.0; ratio 0.99; **46.0 per city liberated**; a use 18.0 per city liberated

### Claim a site: station: 3 amb 24 for owned station 20; 24.0 per owned station

- **Power Purchase Agreement** (2 cr, 2 en, T1 keep 1, play 0 ins): pay 15.0; 2 energy instead of 3 ambassadors; other 0.0; ratio 1.33; **15.0 per owned station**
- **Microgrid Lab Compact** (18 cr, T2 keep 3, play 0 ins): pay 48.0; Action 2 energy: claim, ~2 neutral stations in reach (2 uses); other 0.0; ratio 0.83; **24.0 per owned station**; a use 9.0 per owned station
- *Field Station (Researcher)* (power): pay 0.0; 1 Industry sacrifice: build a station (sacrifice unpriced); **0.0 per owned station**

### Claim a site: factory: 2 amb 16 for factory 19; 16.0 per owned factory

- **Prefab Plant Kit** (13 cr, T1 keep 1, play 0 ins): pay 17.0; build a factory; 1 energy a Power tag (~3) 13.5; other 13.5; ratio 1.91; **3.5 per owned factory**

| job | standard per unit | cards | best | median | standard / median |
|---|---|---|---|---|---|
| Place a node | 6.0 | 4 | -0.7 | 1.2 | 5.14 |
| Remove agents | 5.3 | 23 | -5.2 | 2.0 | 2.67 |
| Remove a troop | 10.0 | 7 | -9.5 | 4.0 | 2.50 |
| Lift Fallout | 12.0 | 5 | -3.8 | 2.0 | 6.00 |
| Gain a player ambassador | 20.0 | 17 | 0.2 | 2.0 | 10.00 |
| Take down a botnet | 16.0 | 3 | 14.0 | 15.0 | 1.07 |
| Buy a city's energy | 5.0 | 13 | 0.3 | 3.3 | 1.52 |
| Turn a cube into insight | 6.0 | 10 | 1.2 | 2.4 | 2.47 |
| Envoy | 8.0 | 8 | -1.5 | 2.6 | 3.10 |
| Make antimatter | 30.0 | 8 | 2.0 | 10.0 | 3.00 |
| Liberate a city (exempt) | 40.0 | 9 | 28.0 | 38.4 | 1.04 |
| Claim a site: station | 24.0 | 2 | 15.0 | 19.5 | 1.23 |
| Claim a site: factory | 16.0 | 1 | 3.5 | 3.5 | 4.57 |
