# Hand-play log: d4 spot check, seed 268010 (2 players, Builder + Protector, level 4)

Rounds played in full: 0 (setup), 1, 3, 5. Rounds 2 and 4 (and 6) taken from check.md as given.
Conventions: a cube, token, node, agent or card moved counts 1 each; a track marker moved any distance counts 1; a pile of
resource tokens paid or gained in one act counts 1 per resource kind per act. A printed site (station, factory) needs no
piece; its state cube does. Map adjacency read off the printed 2-country map (content/map.json, dumped with mapdump.py).

## Round 0

Setup, step by step (§3, with §21's level-4 row and §5's board table).

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human options / engine options) | notes |
|---|---|---|---|---|---|---|---|
| Round start: choose players and level | §21 (d4 row: deck 2/4/3, compute 15, Infiltrate 2 a city) | 0 | 0 | 0 | 0 | 1 (level: 5 / -) | §3 step 3 says "your level asks for (§21; levels 0-2: 4, 4 and 2)": d4's numbers are only in §21. |
| Lay out map, corrupt 2 cities a country | 0 | 0 | 0 | 4 (read each country's corruption order and its marked station) | 6 adversary cubes (H6, H7, C7, C10, H4, C12), 2 botnets (H4, C12) | 0 | Printed stations and factories need no piece: neutral H10, C2, C11; factories H2, H8, H13, C3, C14. |
| Megaproject cards | 0 | 0 | 0 | 0 | 2 cards (Tidal Grid, Polar Compute Array) | 0 | |
| Agent supply | 0 | 1 (24 x 2 = 48) | 0 | 1 (count out 48) | 1 pile | 0 | |
| Threat decks (War Rooms) | adversary-cards.md (War Rooms) | total 2 (t1: 6 Security + 6 others; t2: 12 Security/Universal + 12 others) | 0 | 57 (sort 25 t1 + 32 t2 threats by tag) | 36 cards | 0 | "every Security threat" is ambiguous: primary tag or any tag? I counted any-tag (Supply Raid, Provocation, Factory Occupation, Arsenal included). |
| Event deck | §21 | 0 | 0 | 16 (sort 16 event cards by stage) | 9 cards dealt, 1 turned | 0 | d4: 2 / 4 / 3 = 9 rounds. Round 1's card: Probing Strikes. |
| Project deck | 0 | 0 | 0 | 0 | 3 cards (Station Coup 48, Accelerated Timetable 40, Data Heist 24) | 0 | Costs x 2 countries: 3 modifiers (counted here). |
| Compute and research markers | 0 | total 4 (15 x 2 = 30 income; K = 2 x 2 = 4 cubes a slot; 3 project costs x 2 counted below) | 0 | 0 | 2 markers | 0 | Project costs: 24x2, 20x2, 12x2 = 3 more modifiers. |
| Unity / Discord decks and tracks | aid (tip thresholds: Unity 6, Discord 8) | 2 (3x2, 4x2) | 0 | 0 | 2 shuffles, 4 markers | 0 | |
| Player decks | 0 | 0 | 0 | 0 | 2 shuffles | 0 | |
| Players: boards, nodes, ambassadors, resources | §5 (credits table) | 0 | 0 | 2 | 28 nodes onto tracks, 2 nodes placed (Builder H9, Protector C6), 2 ambassadors, energy 2 piles, credits 2 piles (27, 31), insight 2 markers (3), 2 tag dials, 4 income markers | 1 (boards: default) | |
| Opening deal (round 1's C, listed here) | 0 | 0 | 0 | 0 | 8 cards | 0 | Builder: Peace Conference, Dispatch Control Room, Autonomous Defense Grid, Surgical Strike. Protector: Prefab Plant Kit, Envoy Briefings, Penning Trap Array, Vertical Monopoly. |

Setup totals: lookups 4, modifiers 12, bookkeeping 80, components about 122 (36 threat cards dominate), decisions 2.

State after setup (my reckoning): Builder 27 cr, 3 en, 0 chips, 1 amb, node H9. Protector 31 cr, 3 en, 1 amb, node C6.
Corrupted: H6, H7, C7, C10 (cities), H4, C12 (stations, botnets). Compute income 30, research 0/4 at slot 0. Projects: Station
Coup 0/48, Accelerated Timetable 0/40, Data Heist 0/24. Unity 0, Discord 0.

Check after setup: matches (engine shows stations and factories as pieces; on the table they are printed).

## Round 1

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human options / engine options) | notes |
|---|---|---|---|---|---|---|---|
| Round start | 0 | 0 | 0 | 2 (each insight from the rightmost printed space) | 2 insight markers (3, 3) | 0 | |
| A Sway | 0 | 0 | 0 | 1 (no Fallout) | 0 | 0 | |
| B Card: threats | §10 (location tokens: most agents, tie lowest) | 0 | 0 | 8 (Cryptojacking: station H10; factories H2, H8, H13. Load Shedding: city C15; stations C2, C11) | 1 event card, 2 threat cards, 4 location tokens | 0 | Cryptojacking (Horizon): anchor H10, token H2. Load Shedding (Cintamani): anchor C15, token C2. All ties at 0 agents. |
| B Card: lines | §6 (Infiltrate at level 4: "2 at level 4, on cities"; the aid's round row omits it) | 1 (2 a city) | 0 | 11 (Escalate: none in round 1; Commandeered Plants: 5 factories, none hold an agent; coastal: 6 cities checked for ocean/printed coast; 1 arrival check at C15: 2 < 3) | 2 agents (C15), 1 blocked chip (C15) | 0 | Coastal is derived, not printed, for H6, H7, C7 (next to ocean); only C15 prints it. H14 is not coastal. |
| B Income | 0 | total 2 (compute 30 + 2 botnets = 32; research 2 vs K 4) | 0 | 9 (4 corrupted cities, 2 stations, 2 botnets, 1 research check) | 1 card turned (Factory Drive), Discord marker 1 (to 4), 2 research cubes, compute marker 1 (Station Coup 32/48) | 0 | |
| C Cards | 0 | total 4 (keep costs: Builder 3; Protector 1+1+1) | 2 (each player's insight left: Builder 0, Protector 0) | 0 | 8 cards built into packets, 8 drafted, 4 discarded, 2 insight markers | 8 tier picks (2/4), 8 draft picks (4,3,2,1 visible / 8,4,6,3,4,2,2,1), 2 keep sets (Builder keeps 1 of 4, Protector 3 of 4) | Builder keeps Exascale Training Cluster (t2, 3 insight); Protector keeps Joint Fabrication Pact, Checkpoint Network, Simulation Cluster. |
| D Builder | §8 (from supply onto a node: arrives ready) | total 3 (Annex range 2; credits running 27+1+2-20+3-8 = 5; 2 chips from Pact) | 1 (Builder credits running total) | 3 (each node hex: land, free, in range) | 3 nodes, 3 used markers, 5 credit changes, 1 energy, 2 chip changes, 1 ambassador gained, 2 moves, 1 tipped, 1 threat card + 2 tokens off, 3 income-marker moves (Sprawl +2 cr, Annex +1 cr, H14 energy) = 25 | 7: Buy In H10 (track 2/2, hex 4/4), Sprawl H7 (6/6), Annex H1 (12/12), Gain ambassador, pay Cryptojacking 8 cr, move supply to H7 (4/4), move H7 to H4 (8 shown / 8) | Annex needs 1 chip: only possible after the Protector's Joint Fabrication Pact gave 2. Ambassador ends on H4 (corrupted station + botnet): blocks its cube and its compute. |
| D Protector | §7 (a building with no target still plays), §10 (part-paying a threat) | total 2 (Prefab: 2 Power tags = 2 energy; credits running 31+1-9-7-9+2-1 = 8) | 1 (Protector credits running total) | 4 (node hexes; Prefab's target zone in reach: C9 only; Checkpoint's push: no agent in reach) | 3 nodes, 3 used markers, 6 credit changes, 3 energy changes, 2 chip changes, 1 ambassador gained + 1 moved, 3 cards played, factory C9 + its cube, station C6 + its cube, 4 tag-dial turns, 4 income markers, 1 credit onto Load Shedding = 37 | 9: Accord C7 (track 2/2, hex 6/6), Levy C8 (7/7), play Pact, Checkpoint, Prefab (zone 1/-), Outpost C13 (12/12), move supply to C7 (4/4), station C6 (4/4), pay 1 cr on Load Shedding | Paying 1 of 8 credits leaves it unpaid: it executes and the credit comes back. Prefab "Build a factory": whose? I took it as the Protector's (it then makes 1 chip). |
| E Production | 0 | 0 | 0 | 13 (6 factories, 4 cities, 5 stations, 2 megaprojects; C15 blocked) | 1 chip (C9), 1 energy (Builder, H14), 4 cubes onto megaprojects (Tidal: H10 neutral; Polar: C2, C11 neutral, C6 Protector) | 0 | |
| E Income | 0 | total 2 (Builder 10+2+1 = 13 cr, 1 en; Protector 9+1+2 = 12 cr, 2+1 en) | 0 | 2 | 4 resource changes | 0 | |
| F Threats | §21 (War Footing +4 compute: not on the aid) | 1 (3 + 4 = 7 compute waiting) | 1 (Checkpoint face down until end of round 2: the face-down card shows it, the end round does not) | 2 (Load Shedding unpaid; arrival check C15: 3 agents = corrupt number 3) | 1 card face down, 1 compute-waiting marker, 1 credit back, 1 agent, adversary cube on C15, 3 agents back, blocked chip off, threat + 2 tokens off = 11 | 1 (which building face down: 1/1) | C15 corrupts: Cintamani now has 3 corrupted cities, none free. |
| G Cleanup | 0 | 0 | 0 | 2 | 1 (Builder's ambassador stands) | 0 | |

Round 1 peak memory at once: 2 (both players' insight left in C/D; or a running credit total and the face-down card's end round).

My state at end of round 1: Builder nodes H9, H10, H7, H1; 18 cr, 3 en, 1 chip, 1 amb in supply, 1 on H4. Protector nodes C6,
C7, C8, C13; 21 cr, 3 en, 2 chips, 1 amb in supply, 1 on C7; station C6, factory C9 (both the Protector's). Corrupted: H6, H7,
C7, C10, C15, H4, C12. No agents on the map. Discord 4, Unity 0, research 2 (slot 0). Station Coup 32/48, 7 compute waiting.
Megaprojects: Tidal 1 neutral; Polar 2 neutral + 1 Protector.

Check, end of round 1: all pieces, cubes, tracks and the Builder's resources match. **One error:** Protector energy, I had 3,
the engine 4. I missed that the Protector's node on C13 is next to Horizon's free city H14, across the border (1 energy).

## Round 2 (skipped; end state from check.md as given)

Notable in it: Station Coup corrupted H10 (destroying the Builder's node there) and C2; the Protector liberated C7 (Unity 5);
Discord tipped once (Offensive, nothing dealt); Fallout on C1. Research 3 cubes at slot 1 (New factory: H11, C16).
Projects now: Accelerated Timetable 22/40, Data Heist 0/24, Asset Seizure 0/26; 12 compute waiting; compute income 30.
Builder 21 cr, 2 en, 1 chip, 1 amb (on H4); in play Exascale Training Cluster (Vendor Financing scrapped, its +2 income kept).
Protector 16 cr, 5 en, 2 chips, 4 amb; in play Pact, Prefab, Envoy Briefings, Fact-Check Network, Checkpoint Network, Holographic
Assembly Hall, Simulation Cluster. Polar Compute Array 3 Protector + 3 neutral; Tidal 1 neutral. Agents: H10, C2 (1 each).

## Round 3

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human options / engine options) | notes |
|---|---|---|---|---|---|---|---|
| Round start | 0 | total 2 (Builder 7 + Exascale 4 = 11; Protector 7) | 0 | 2 | 2 insight markers, used markers cleared (2 boards, 6 markers) | 0 | "Each round, 4 insight" on a building: insight income is set at round start, not paid in production (§5, end): easy to pay twice. |
| A Sway | 0 | 0 | 0 | 1 (Fallout C1) | 1 (Discord to 1) | 0 | |
| B Card: threats | §18 (tie rule for tokens; two threats on one hex allowed?) | 0 | 0 | 10 (Weapons Test: 4 Horizon factories; Casus Belli: H14 the only free city; Provocation and Manhunt: C7 the only free Cintamani city) | 2 cards moved (last-round space, discard), 4 threat cards, 4 location tokens | 0 | Weapons Test H2, Casus Belli H14, Provocation C7, Manhunt C7. Threat ids differ from names (orbital_strike is Weapons Test): only the script's problem, not a table one. |
| B Card: lines | 0 | 1 (Funding: +1 x 2 countries) | 1 (last round's type: coastal cities, read off the last-round card) | 18 (Escalate: 4 coastal cities, none free with an agent; Commandeered Plants: 9 factories; Infiltrate factories: 8 arrival checks) | 8 agents (H2, H8, H11, H13, C3, C9, C14, C16), 1 blocked chip (C9, the Protector's), compute income marker (30 to 32) | 0 | Level 4 doubles only cities: factories take 1 each. |
| B Income | 0 | total 4 (32 + 2 botnets + 12 waiting = 46; AT takes 18, Data Heist 24, Asset Seizure 4; research 3 + 3 = 6, spend 4) | 2 (the compute left after each resolution; the research remainder) | 22 (4 cities, 4 stations: H4 blocked by the Builder's ambassador, 3 botnets: H4 blocked; 2 projects resolved; 2 extra threats placed (Overload: 4 factories tied at 1, no free station; Deepfake Wave: C7 and 3 stations); Data Heist x 2 players; Cells: factories 8 arrival checks) | 1 card turned (Station Raids), Discord marker (to 5), 3 research cubes, 4 cubes off for the slot, research marker, 1 event card discarded (Border War), 2 projects off, 2 dealt, project markers 3, 2 threat cards, 3 tokens, 6 cards discarded (Data Heist), 8 agents (Cells: factories) = 33 | 2 (Data Heist: pay 7 or discard 3, each player; discards 3 of 5 / 5) | Two projects resolve in one compute step and the second one's slide happens mid-pile: I had to write the remainder down. Accelerated Timetable's threats come in the Income beat, so they get no waits token. Every Horizon and Cintamani free factory now holds 2 agents. |
| C Cards | 0 | total 3 (Protector keeps: 2 + 2 + 1 with the keep upgrade) | 2 (insight left: Builder 10, Protector 2) | 0 | 8 cards built, 8 drafted, 4 discarded (Builder 3, Protector 1), 2 insight markers | 8 tier picks (2), 8 draft picks, 2 keep sets (Builder 1 of 4, Protector 3 of 4) | Builder keeps Moonshot Grant; Protector keeps Continental Supergrid, Lure Network, Shared Threat Intel. |
| D Builder | §8 (Take down a botnet: needs the ready ambassador on its hex) | total 2 (credits 21 + 3 - 13 - 11 = 0; Annex 2 hexes across the border) | 1 (credits running) | 3 (Annex's hex: C10 within 2 of H15 via C14; Overload near: H1 next to H2) | 1 node, 1 used marker, 3 credit changes, 1 chip, 1 energy, botnet off, 1 ambassador tipped, threat card + 1 token off, income marker = 11 | 3: Annex C10 (track 2/2, hex 12/12), take down H4's botnet, pay Overload 11 cr | Overload (2 of the Builder's nodes destroyed) paid in full. |
| D Protector | §10 (a jam to 0: is the threat paid?), §20 (Interdict II costs 2 Security: Shared Threat Intel carries 1), §8 (ambassador arriving on an agent) | total 4 (Accord 3 + space 2 = 5 energy; Casus Belli jammed 6: 4 amb to 0, 11 cr to 5; credits running 16 - 5 - 2 - 3 = 6; Shared Threat Intel's 2 insight to the Builder) | 3 (credits running; the jam on Casus Belli, no token for it; which ambassador moved) | 9 (Accord hex H10; Checkpoint target H11 and 2 push destinations; Simulation Cluster's repeat on H13; Interdict II target C9; each of 3 ambassador moves checked for agents) | 1 node, 4 used markers (Accord, Checkpoint, Simulation, Interdict), 1 ambassador gained, 2 agents pushed (H11 to H8), 2 agents removed (C9) + blocked chip off, 1 card sacrificed, 1 card drawn, 2 cards played, 5 credit changes, 1 energy, 1 insight, 3 ambassador moves, 1 agent + 1 ambassador off (H10), 1 tipped, 3 credits onto Deepfake Wave = 27 | 11: Accord H10 (11/11), Checkpoint (target 7, push 11, 6), Simulation repeat (target 6, stop), Shared Threat Intel jam (4/4), Interdict II (sacrifice 2/2, target 6/6), play Reconciliation Office, 3 moves (7, 7, 12), 3 x 1 cr on Deepfake | Pushing 2 agents onto H8 makes 4 there: no arrival, no check (§16), but the next arrival corrupts it. Paying 1 credit three times toward an 11-credit threat changes nothing. |
| E Production | 0 | total 2 (Polar: 5 Protector + 4 neutral = 9, completes; 5 x 6 = 30 credits) | 0 | 15 (9 factories, 1 player factory paid; 5 free/corrupted cities; 7 stations; 2 megaprojects) | 1 chip, 4 city-energy payments (Builder C7, H14; Protector C7, H14), 3 cubes onto Polar, 9 cubes off Polar, 30 credits, converter opens | 0 | Polar Compute Array completes: from now the Protector's stations give it research cubes; its converter (1 cube to 2 insight) opens. |
| E Income | 0 | total 2 (Builder 10+2+1+2+2+2 = 19 cr, 1 en; Protector 9+1+2+2+2 = 16 cr, 3 en) | 0 | 2 | 4 resource changes | 0 | |
| F Threats | §10 (order within a country: dealt order?), §6 (Fallout "free open land": does a node make a hex not free?) | total 3 (War Footing 4 x 5 = 20 compute; Discord 5 + 3 + 2 + 3 = 13, tip 8, 5 left; Unity 5 - 2 = 3) | 1 (Grid Tap waits: its waits token shows it) | 12 (5 unpaid threats; 5 anchor arrivals; Fallout sites for Casus Belli: 6 neighbours of H14 checked; Captured Labs: 4 corrupted stations, at most 2) | 5 threat cards + 6 tokens off, Weapons Test 2 cubes, 5 agents, H2 cube (corrupts), 3 agents back, Manhunt 2 agents, C7 cube (corrupts), the Protector's C7 node destroyed, 3 agents back, Grid Tap dealt + token + waits token, 3 discards, Discord marker x3, tip marker, 8 off, Discord card, 4 research cubes, Unity marker, 3 credits back = 44 | 3 (Manhunt: pay 3 amb or discard 3 (2/2), then 3 of 4) | Casus Belli: I treated the jammed-to-0 threat as not paid (nobody paid it) and resolved it: 3 Discord, 2 Fallout next to H14 (none free: lost). C7 corrupts on Manhunt's 2 agents (1 from Provocation's anchor + 2 = 3). |
| G Cleanup | 0 | 0 | 0 | 2 | 2 (tipped ambassadors stand: Builder H4, Protector H6) | 0 | |

Round 3 peak memory at once: 4 in the Protector's action phase (credits running total, the jam on Casus Belli with no token, the
compute remainder is past, which ambassador is which; plus each player's insight left = 5 across the table).

My state at end of round 3: Builder 19 cr, 3 en, 0 chips, 0 amb in supply, 1 on H4; nodes H1, H4, H7, H9, H15, C10. Protector
55 cr, 5 en, 3 chips, 3 amb in supply, 1 on H6; nodes C6, C8, C11, C13, H10, H14 (C7's destroyed). Corrupted: H2 (factory), H4,
H10, C2, C12 (stations), H6, H7, C7, C10, C15 (cities). Agents: H8 4, H13 2, H14 1, C3 2, C14 2, C16 2, C2 1, C7 1 (Manhunt
anchor after corruption). Botnets C2, C12. Fallout C1. Discord 5, 2 tips; Unity 3. Research 8 cubes at slot 2. Projects: Asset
Seizure 4/26, Propaganda Network 0/36, Dirty Bomb 0/24; 20 compute waiting; income 32. Polar complete; Tidal 1 neutral.
Grid Tap (Cintamani) waits for round 4.

Check, end of round 3: map, tracks and most resources match. **Four errors:** (1) Casus Belli: the engine treats a threat jammed
to 0 of a resource as paid off (Discord 2 not 5; compute waiting 16 not 20; no agent on H14); (2) I forgot Deepfake Wave's own
anchor agent on C7 (2 there, not 1); (3) Builder's supply ambassador: I carried 0, it was 1 all along (my state slip from the
round-2 read); (4) Protector credits 57 not 55: Reconciliation Office ("when you remove Agents, gain 2 credits") fired on the
ambassador cancelling H10's agent, which I did not read as "removing" one.

## Round 4 (skipped; end state from check.md as given)

Notable: Asset Seizure turned the Protector's C9 factory neutral; the Protector liberated H6; Unity tipped twice; research slots 3
(Funding: income 34) and 4 (Garrison, 2 a country under Arms Race); troops removed down to 1 on C3; Air Gap firewalled C7. End:
Builder 30 cr, 6 en, 1 amb, 1 on H4; nodes H1, H4, H5, H7, H9, H13, H15, C10; Exascale in play (Collateralized Grid scrapped,
+2 energy income kept). Protector 26 cr, 4 en, 2 chips, 1 antimatter, 2 research cubes, 3 amb (+2 on H14, 1 on H6); nodes C6, C8,
C11, C13, H10, H11, H14. Discord 7 (2 tips), Unity 3 (2 tips). Research 3 cubes at slot 4. Propaganda Network 23/36, Dirty Bomb
0/24, Mob Rule 0/42; 8 waiting. I inferred the Protector's buildings (Holographic Hall, Simulation Cluster, Antiproton Foundry)
from slot counts: Interdict's sacrifices are not in the script, so slot state was a reconstruction (it cost 10 minutes).

## Round 5

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human options / engine options) | notes |
|---|---|---|---|---|---|---|---|
| Round start | 0 | total 2 (Builder 8 + Exascale 4 = 12; Protector 8) | 0 | 2 | 2 insight markers, used markers cleared (about 8) | 0 | |
| A Sway | §12 (tip strength by tip count), aid (tip strip), §19 (Standing Army II: where) | total 2 (7 + 1 = 8 = tip; 8 - 8 = 0) | 0 | 6 (Fallout C1; Standing Army II: 4 corrupted cities checked for a node or ambassador on or next to them) | Discord marker x2, tip marker, Discord card, 2 troops (H7, C7) = 6 | 0 | C7 is firewalled: troops still land (§16). |
| B Card: threats | §18 (two tokens never share a hex) | 0 | 0 | 6 (Grid Collapse: H6, H14 tied at 0, H6; coastal: only H6, already used, so no 2nd token. Shock Doctrine: no free Cintamani city, factory C9) | 2 cards moved, 2 threat cards, 2 tokens = 6 | 0 | |
| B Card: lines | §11 (Redeploy: distance from the nearest node or ambassador; a node on the city is distance 0), §8 (agents landing on ambassadors) | 1 (2 agents a city) | 1 (last round's type: research stations) | 17 (Escalate: 6 stations; Commandeered Plants: 5 factories; Infiltrate: 2 free cities, each agent vs ambassadors; Redeploy: 3 troops x 3 corrupted cities' distances) | 4 agents (2 H6, 2 H14), 3 ambassadors off (1 H6, 2 H14), 3 agents off, 2 troops moved (C3, C7 to C10) = 12 | 0 | H6 keeps 1 agent; H14 ends empty. Both Cintamani troops go to C10, where the Builder's node stands. |
| B Income | aid (factory output), §17 (slide and refill mid-step), §6 (Fallout with no free hex) | total 5 (34 + 3 botnets + 8 = 45; PN takes 13, 32 left; Dirty Bomb 24, 8 left onto Mob Rule; income 34 + 2 = 36; Discord 4 + 8 = 12; research 3 + 3 = 6) | 2 (compute remainder; Vigil's count, capped at 2) | 35 (4 cities, 4 stations, 7 corrupted factories with their arrows, 4 botnets; 2 projects; Dirty Bomb: 3 cities' neighbours, no free hex; research check; Strike: 3 troops x targets with Wall; Vigil: 3 troops; Incursion: 4 cities, 4 threats placed) | 1 card turned, Discord marker x3, 3 research cubes, 1 agent pushed (H2 to H6), 4 agents made (H2, H11, H13, C14, C16 = 5), compute markers 3, income marker, 2 projects off + 2 dealt, 4 cubes off + research marker, 1 node destroyed (H7), 1 node destroyed (C10), 2 Protector insight, tip marker, Discord card, 4 threat cards + 4 tokens, firewall off = 36 | 1 (Strike at H7: 2 / 2) | H11 and C14 sit next to two cities: the map must print each zone's arrow; the content I have does not, so I guessed (H11 to H14, C14 to C10). Incursion II says "deal 1 extra threat for each corrupted city": no tier, no country; I followed the script's placement (2 a country, tier 2). |
| C Cards | §7 (Science discount on tier-2 keeps) | total 4 (Builder 2 + 1 + 1 + 1 = 5; Protector 2 Science = 1 off each tier-2: 1 + 1 + 1 = 3) | 2 (insight left: Builder 7, Protector 7) | 0 | 8 built, 8 drafted, 1 discarded, 2 insight markers | 8 tier picks, 8 draft picks, 2 keep sets | |
| D Builder | §16 (convert: where the new ambassadors stand, ready or tipped), §8 (liberate: who pays, guards) | total 3 (credits 30 - 5 - 20 + 1 + 2 = 8; energy 6 - 2 (space) - 2 - 2 (Sprawl + space) = 0; Liberate 3 = 1 on site + 2 supply) | 3 (credits running; which ambassadors are converted and ready; Open Market's 3 free keeps for next round, no token) | 9 (Double Agents: agents on or next to H6 = 3, troops = 1; Embassy: agents on or next to H14 = 3; liberate checks: agents 0, troops on or next 0; move onto H8's agent; node hexes) | 3 agents to 3 ambassadors + 1 troop off, 1 building, 3 ambassadors gained, 2 agents to 2 ambassadors, liberate: 3 ambassadors spent + cube off + Unity marker, tip marker + Unity card, move to H8: 1 agent + 1 ambassador off, 2 nodes, 2 used markers, 5 credit and 3 energy changes, income marker = 30 | 7: Double Agents (target 4/4), Embassy (slot 2/2), Embassy action (target 3/3, convert 3/3, 2/2), Liberate H7 (5/5), move H11 to H8 (13, then 2), Buy In H2 (track 2/2, hex 7/7), Sprawl H8 (6/6) | Energy: 6, Buy In onto card-play space 4 pays 2, Sprawl 2 + space 2: the Builder ends at 0. |
| D Protector | §19 (Disarmament Talks: troops left anywhere?), §10 (part-pay) | total 4 (Exodus 12 cr; Hall twice: 2 en, 2 Unity, 2 amb each; Unity 3 + 2 = 5, + 5 (H7) = 10, tip, 4 + 2 = 6, tip, 0 + 2 = 2; ambassadors 3 + 2 + 4 + 2 - 1 - 1 - 2 = 7) | 3 (credits running; ambassadors in supply; Unity tips in a row) | 7 (Interdict I target: C10's 2 troops; Exodus target and 6 gather sources; Disarmament Talks: no troop left; Liberate H10: no agent, no troop; Grid Collapse near) | 2 troops off, 1 card sacrificed (Logic Bomb), 2 tag dials, 4 energy, 2 Unity marker moves, 8 ambassadors gained, 1 gathered, 2 tips + 2 Unity cards, 1 building played, 2 ambassador moves, liberate: 3 ambassadors + cube off + Unity marker, 2 research cubes onto Grid Collapse, 3 credit changes, 4 used markers = 33 | 9: Interdict I (sacrifice 1/1), Holographic Hall, Exodus (target 8/8, gather 5, stop), Simulation repeat, Fusion Torch (slot 4/4), move to H14 (7/7), move to H10 (10, supply/map 2), Liberate H10, pay 2 cubes on Grid Collapse | Two Unity tips back to back in the action phase, each drawing a card that reads the board as it stands. |
| E Production | 0 | 0 | 0 | 14 (9 factories (none a player's), 7 cities: H6, H7, H14 free, 3 nodes' worth each; 7 stations; Tidal) | 6 city-energy payments, 1 neutral cube (H10 to Tidal), 2 research cubes (Protector) = 9 | 0 | |
| E Income | 0 | total 2 (Builder 22 cr, 3 en; Protector 16 cr, 3 en) | 0 | 2 | 4 | 0 | |
| F Threats | §10 (neither price possible: pay what you have and discard the hand), §18 (fallback token: located lines don't act) | total 4 (War Footing 6 x 4 + Turncoats' 3 = 27 waiting; research 2 + 3 + 3 = 8; Unity 2 - 3 - 2 floored at 0) | 1 (research at 8 = two slots next round: the loss, visible on the row) | 13 (6 unpaid threats; 6 anchor arrivals; Grid Collapse's closest node: 3 tied; Rootkit's nearest piece; Arsenal: no free zone) | 6 threat cards + 8 tokens off, 1 node off (H5), 3 + 2 + 2 cards discarded, 1 botnet (H2), 6 agents, ambassador + agent cancel (H6), 2 ambassadors paid (Turncoats), C9 corrupts (cube, 3 agents back), research cubes 6, Unity marker x2, compute marker = 38 | 5 (Grid Collapse node 3/3, pay-or-discard 2/2, 3 discards; Rootkit 4/4; Turncoats 2 discards; Shock Doctrine 1 discard) | Compute waiting: Grid Collapse 4, Rootkit 4, Turncoats 3 + 4, Shock Doctrine 4, Brain Drain 4, Arsenal 4 = 27. Rootkit on a fallback token: I skipped its "2 Agents are created here" as a located line. |
| G Cleanup | 0 | 0 | 0 | 2 | 1 | 0 | |

Round 5 peak memory at once: 5 (both players' insight left, a running credit total, Open Market's 3 free keeps for next round, the
compute remainder during the Income beat).

My state at end of round 5: Builder 30 cr, 6 en, 0 amb in supply (2 paid to Turncoats); ambassadors on H4, H2, H13; nodes H1, H2,
H4, H8, H9, H13, H15. Protector 22 cr, 6 en, 2 chips, 1 antimatter, 4 research cubes, 7 amb in supply, 1 on H14; nodes C6, C8,
C11, C13, H10, H11, H14. Liberated: H7 (city), H10 (station). Corrupted: H6? no: H6 free with 2 agents; corrupted cities C7,
C10, C15; factories H2, H8, H11, H13, C3, C9, C14, C16; stations H4, C2, C12. No troops. Discord 4 (4 tips), Unity 0 (4 tips).
Research 8 at slot 5. Mob Rule 8/42, Slander Campaign 0/18, Lab Purge 0/22; 27 waiting; income 36. Tidal 2 neutral. Botnets H2,
H4, C2, C3, C12.

Check, end of round 5: every piece, cube, track, project and compute total matches (compute onto projects 45, waiting 27,
research 8 at slot 5, Discord 4 / Unity 0 at 4 tips each). **Two errors:** (1) Builder ambassadors 2, not 0: Turncoats with
neither price possible (2 ambassadors of 4, 2 cards of 3) only discarded the hand; I also paid the 2 ambassadors, reading §10's
"pay what they have and discard their hand" as both. (2) Protector credits 23, not 22: Fusion Torch Array cost 7, not 8; the
Protector's third Industry tag (Antiproton Foundry, played in skipped round 4) gives 1 credit off each building, and I did not
recount the Industry tags.

counts.csv's minutes use the non-trivial decisions only (rounds 1 / 3 / 5: 25 / 28 / 30 of 35 / 37 / 40).

Round 6 (not played): research check 8 cubes = two slots, slot 6 (Garrison, 2 troops a country) then slot 7: the players lose.
