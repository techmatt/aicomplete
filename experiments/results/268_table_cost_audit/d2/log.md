# Hand-play log: d2, seed 268003 (Builder in Horizon, Protector in Cintamani)

Played from `docs/rulebook.md` and `docs/player-aid.md`, the card lists (`docs/cards.md`,
`docs/adversary-cards.md`), the printed map (`map2.txt`, `content/map.json` countries 0-1) and the table script.
Two reading aids that are not rules: the script names powers by index, so `content/ais.json`'s power order and track
spaces were read once to decode them (a human holds the board and sees the names), and `card.py` here prints a card's
row from `docs/cards.md` with its section (= first tag), standing in for picking the card up.

## Counting conventions and rates

- **Lookup:** reopening a rulebook section or table mid-game (first full read before setup not counted). 20 s each.
- **Modifier:** one arithmetic adjustment of a printed number (x countries, discount, d2 corrupt number 3 instead of
  the printed 5, track-space costs added, income sums of several sources = 1 per source added). 5 s each.
- **Memory:** an item held with nothing on the table to show it. Peak = most held at once in the round.
- **Bookkeeping:** one per site / city / piece / threat-card checked against a condition. 10 s each.
- **Components:** map pieces (agent, ambassador, node, troop, cube, botnet, chip-on-city, Fallout, firewall,
  location token) 1 each; cards 1 each moved (dealt, drawn, passed packet = 1, played, discarded); a resource payment or
  gain on a player board 1 per transaction whatever the amount; **compute 1 per cube** (asked for, so its volume shows);
  a track marker moved N spaces 1. 3 s each.
- **Decision:** one per choice made; option counts are what a human sees at the table beside the engine's count.
  15 s per non-trivial decision, 5 s per trivial one (folded: I count 15 s on half the decisions).
- **Minutes** = (20 L + 5 M + 10 B + 3 C + 15 D x 0.6 + 20 s per new card face read) / 60, for 2 practised players
  (simultaneous action phase not discounted: the two players' moves are both counted, so time is table-time if they
  take turns explaining moves, a fair model for a co-op game).

---

## Round 0: setup

Done before reading `check.md` "After setup"; it matched on every piece.

| step | lookups (§) | modifiers | memory | bookkeeping | components | decisions | notes |
|---|---|---|---|---|---|---|---|
| Choose players, level | §21 (level 2 = what) | 0 | 0 | 0 | 0 | 2 (boards: 3 options, pick 2; level given) | |
| 1 Lay out map | §2 (fronts, bases) | 0 | 0 | 1 (find base hexes H15, C13) | 2 tiles + 2 base markers = 4 | 0 | Base hexes are printed on the tile; the rulebook says "the layout for your player count shows where". |
| 2 Country places | §3 | 0 | 0 | 2 countries x (cities, corrupted, stations, factories, sites) = 10 reads of map.json fields | adversary cubes 2, botnets 2, stations 5 (H4, H10, C2, C11, C12), factories 5 (H2, H8, H13, C3, C14), megaproject cards 2 = 16 | 0 | "Its marked city" = `corrupted[0]` only (H7, C10); the JSON lists a second corrupted city that does not start corrupted at this config: a human reading a tile with two marks would ask. |
| 3 Adversary | §21 table, §17 | compute income 5 x 2 = 10; research threshold 4 x 2 = 8; 3 project costs x 2 (40, 14, 44); tip 3x2 = 6, 4x2 = 8 = 7 | 0 | 0 | agent supply 48 counted out (1), event deck 13 cards dealt from 3 stage decks + 3 shuffles (count 13), top card up 1, pool agents 2, markers (compute income, agent income, threshold) 3, project deck shuffle + 3 dealt = 3 = 23 | 0 | Level 2 sets four separate dials on the adversary board; §3 says "1 a country at levels 0-3" for the pool but §21 also lists "Agent income +0", two places to read one number. |
| 4 Shuffle decks | — | 0 | 0 | 0 | 6 shuffles (Unity, Discord, threat 1, threat 2, tier 1, tier 2), Unity/Discord markers 2 + tip counters 2 = 4 (+6 shuffles) | 0 | |
| 5 Players | §5 table (starting credits) | 0 | 0 | 0 | 2 boards, 28 node tokens onto tracks (7+7 per board), 2 setup nodes (H9, C6), 2 ambassadors, credits 2 (40, 46), energy 2, insight 2 = 38 | 0 | 28 node tokens loaded onto board tracks is the biggest single setup chore. |
| **total** | **5** | **7** | **0** | **11** | **85 (+6 shuffles)** | **2** | est. 12 min with the rulebook open the first time; about 6 practised. |

State after setup: Horizon: H7 corrupted city; H4 corrupted station + botnet; H10 station; H2, H8, H13 factories;
Builder node H9. Cintamani: C10 corrupted city; C12 corrupted station + botnet; C2, C11 stations; C3, C14
factories; Protector node C6. Pools 1/1. Event top: War Drums. Projects: Accelerated Timetable 0/40, Purge Lists
0/14, Grand Offensive 0/44. Builder 40 cr 3 en 1 amb; Protector 46 cr 3 en 1 amb. Discord 0/8, Unity 0/6.

---

## Round 1 (War Drums)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | §5 (insight set) | 0 | — | 0 | insight 2 | 0 | |
| A Sway | — | 0 | — | 1 (Fallout count 0) | 0 | 0 | |
| B Card: threats | §18 (deal, tokens) | 0 | — | 4 threat cards tested for tag + location (Requisition, Embargo out; Call-Up H, Propaganda Blitz C); tokens: H base 1, C coastal cities 2 (C7, C15 agents compared) = 7 | event card 1, threat cards 4 (2 discarded), location tokens 2 = 7 | 0 | Call-Up on base H15; Propaganda Blitz on C7 (tie, lowest). |
| B Card: Rally cities, sway | §6 (Rally) | 0 | — | 6 cities in hex order | agents 2 (H6, C7), Discord marker 1 = 3 | 0 | Discord 1. |
| B Corrupt | §15 | city corrupts at 3 (d2), 1 | — | targets 4 (H6, H14, C7, C15); agents 2 checked for adjacency; natural corruption 12 sites; node 2-agent check 2 nodes = 20 | 0 | 0 | nothing corrupts. |
| B March | — | 0 | — | 0 troops | 0 | 0 | |
| B Income | §17 (compute step), §6 (research) | compute 5 x 2 = 10, 1 | — | botnets 2 (ambassador on them?), research: 2 corrupted stations (megaproject complete?) = 4 | next card 1, pool agents 2, compute 12 cubes = 15 | 0 | Accelerated Timetable 12/40. Next: Long Patrols. |
| C Cards | §7 (opening, keep cost) | 0 | — | 0 | opening 8 dealt, build-a-hand 8, packets passed 6, keeps paid 2, discards 5 = 29 | build 8 (2 / 4, 4, 2), draft 8 (4,3,2,1 each / 8,6,4,2 / 4,3,2,1), keep 2 (B: subset of 4, 3 insight / 10; P: 1 of 4 / 5) = 18 | B keeps Patent Library, Mesh Repeater; P keeps Fabrication Megaplex. 4 tier-2 opening cards are long to read (Orbital Embassy, Market Stabilization, Forensic Audit, Air Gap). |
| D Builder | §20 (powers), §5 (track costs, where), §8 (Dispatch) | Mesh node: track space 4 costs 1 energy; Buy In chip test; Patent Library 3 cr = 2 | Expand used, Buy In used, Annex used, Mint used, Procure used, Mesh Action used, Patent Library Action used, Dispatch spent = 8 | Buy In: does C13 bring a factory into reach (C14 yes) 1; Annex range 2 count 1 = 2 | nodes 4 (H13, C13, H7, H5), cards played 3, drawn 4, ambassador 1 (to H4 botnet), resource transactions 12 = 24 | 11: node route + hex x4 (Expand H13 4 / 4; Buy In C13 ~5 / 5; Annex H7 ~14 / 14; Mesh H5 ~8 / 8), plays 3 (of 7 in hand / ~35), Dispatch hex (~13 / 13), Mint, Procure, pass = 11 | Builder ends 0 cr, 0 en, 0 chips. |
| D Protector | §10 (near), §16 (firewall), §8 (Buy energy, hosting) | Grad Student Pool -5 on Shared Compute Grant (floor 1) = 1 | Outpost used, Expand used, city bought (C15) for all players, host plan (C7) = 4 | Buy energy: who hosts / could host each of 4 neutral cities = 4; near Propaganda Blitz 1 = 5 | nodes 2 (C10, C14), cards played 4, drawn 3, firewall 1, agent removed 1, threat + token discarded 2, ambassador gained 1, transactions 10 = 24 | 9: node route + hex x2 (Outpost C10 6 / 6; Expand C14 8 / 8), 4 plays, Air Gap target 1, pay threat 1, buy which city (2 / 1) = 9 | Air Gap on C7 (1 agent). Shared Compute Grant gives Builder 4 insight; nothing left to spend it on (Builder's hand had nothing cheap) so it lapses. |
| E1-2 Factories, cities | §9 | 0 | host choice | 5 factories; 6 cities (blocked? host? corrupted?) = 11 | Discord marker 1 (+2: H7, C10), energy 2 = 3 | 1 host match (P: C7 or C15) | Discord 3. Builder hosts H14 (only one), H6 blocked by agent. |
| E3-4 Stations, megaprojects | §9 | 0 | — | 5 stations, 2 megaprojects = 7 | cubes 4 (H10 neutral on Tidal; C2, C11 neutral, C12 adversary on Polar) = 4 | 0 | Tidal 1 (n), Polar 3 (2 n, 1 adv). |
| E5 Income | §5 table | B 15 + 3 + 2 + 3 cr, 1 en = 4; P 13 + 2 cr, 2 + 1 en = 2; total 6 | — | 0 | 4 | 0 | B 23 cr 2 en; P 19 cr 5 en. |
| E6-8 Growth | §9 | 0 | — | drones 2 bases; grow 6 cities x neighbors; build 6 = 14 | city chips 7 (H6 1, H7 1, H14 2, C7 1, C10 1, C15 1) | 0 | |
| F Threats | §18, §6 (Fallout placement), §21 (War Footing) | War Footing +4 = 1 | War Footing compute owed next compute step (pile) | 1 | troop 1 (H15), Fallout 1, compute 4, threat + token off 2 = 8 | 1 Fallout hex (2 / 0) | Fallout: H11 (lowest of H11, H16 free open next to H15). |
| G War | §11 | 0 | — | 2 bases | 0 | 0 | |
| H Cleanup | — | 0 | firewall on C7 till next Income beat (token shows it) | 0 | 0 | 0 | |
| **total** | **17** | **12** | **peak 12** (8 B + 4 P during D) | **81** | **127** | **40** | |

State end of round 1 (mine): H6 1 agent, 1 chip; H7 corrupt, B node, 1 chip; H4 corrupt st, botnet, B amb; H5 B node;
H9 B node; H13 B node, factory; H14 2 chips; H15 base, 1 troop; H11 Fallout; C13 B node; C6, C10 (corrupt), C14
(factory) P nodes; C10 1 chip; C7 firewall, 1 chip; C15 1 chip. Pools 1/1. Tidal 1n; Polar 2n+1a. Accelerated
Timetable 12/40, pending compute 4. Discord 3, Unity 0. B: 23 cr 2 en 0 ch 0 amb (1 on H4), hand 7; P: 19 cr 5 en
2 amb, hand 4 (Fabrication Megaplex, Forensic Audit, Dispatch Control Room, Telemetry Archive).

**Check (after writing the above):** every piece matches (Fallout on H11, troop H15, chips, nodes, ambassador H4).
**Error 1:** compute on the leftmost project: mine 12 (10 income + 2 botnets), engine `adversary.compute=16`
(`compute_waiting=4` matches my War Footing 4). The rulebook gives me no way to reach 16 (§21's table says compute
income 5 a country at level 2; "levels are cumulative ... adds its setup" could be read as 3 + 5 = 8 a country, which
gives 16 + 2 botnets = 18, not 16 either). Carried the engine's 16; diagnosed next round.

## Round 2 (Long Patrols)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | §5 (insight 7 from card-play space 1) | 0 | Dispatches B 2, P 1 | 0 | insight 2 | 0 | Both on 7 insight. |
| A Sway | — | 0 | — | 1 | Discord 1 (H11 Fallout) | 0 | Discord 4. |
| B Card: threats | — | 0 | — | 2 threat cards + location: Blackout H cities (H6 1 agent vs H14 0), Provocation C base = 4 | card 1, threats 2, tokens 2 = 5 | 0 | Blackout on H6, Provocation on C13. |
| B Rally stations | §8 (agent meets ambassador: is the ambassador lost or back to supply?) | 0 | — | 5 stations dealt in order (1 per country, first site gets it) = 2 dealt | agents 2, H4 agent + B's ambassador canceled 2 = 4 | 0 | H4's agent cancels B's ambassador; C2 gets 1. |
| B Income line, Botnets spread | §6 (Botnets spread) | agent income 1 -> 2 = 1 | — | spread: sites next to each botnet, lowest without one: H (H7, H8) 2, C (C11, C15, C16) 3 = 5 | income marker 1, botnets 2 = 3 | 0 | **Reading choice:** "lowest-numbered site without one next to one of its Botnets" parsed as "a site, next to one of its botnets, that has none" (spread to a neighbor): H7, C11. The other parse (a site with no botnet next to it) gives H2, C2. |
| B Corrupt stations | §15 | station 2 = 0 (printed) | — | targets 3 (H10, C2, C11); agents on map 2: H6's agent next to exactly one target (H10) moves; natural 12 sites; nodes 13 x 2-agent = 28 | agent 1 (H6 -> H10) | 0 | An agent on a city moves onto a station for a station line: easy to miss. |
| B March | §6 (march), §5 (Wall) | 0 | — | 1 troop: targets on or next to H15 (C14 is the Protector's: Wall), nearest targets by distance (B's H13, C13 at 2) = 3 | troop 1 | 1 (2 / 2): H11 or H14 | H11 (Fallout; troops may enter). From H11 it is next to B's node H7. |
| B Income | §17 | compute 5 x 2 = 1 | — | 4 botnets (any ambassador?), research 2 = 6 | next card 1, pool agents 4, compute 10 + 4 + 4 waiting = 18 cubes, firewall off 1 = 24 | 0 | Accelerated Timetable 16 + 18 = 34/40 (engine base). Next: Probing Strikes. |
| C Cards | §7 (keep upgrade not yet) | 0 | — | 0 | hand-build 8, passes 6, keeps paid 3, discards 5 = 22 | build 8 (2 / 4), draft 8 (4,3,2,1 / 8,4,6,3,4,2,2,1), keep 3 (B 1 of 4 tier-2-heavy / 10; P 2 of 4 / 5,4) = 19 | 7 of 8 cards seen are tier 2: long texts (Power to the People, Command-and-Control Grid, Antiproton Foundry, Microgrid Lab Compact). |
| D Builder | §7 (discounts: Industry; a building triggering itself), §13 (Shelter) | CPO triggers on itself +10, MSB +10, BBB +10 = 3; Industry discount -1 on MSB, BBB = 2; Mesh node space 5 costs 2 en = 1; RS income per 2 Cyber = 1; total 7 | Mesh used, PL used, Mint used, Fortify used, Shelter on all B's reach this round (event gone, no marker), CPO payouts taken, Dispatches 2 unused = 7 | Industry tags counted 2x, Cyber tags counted 2x, Fortify: facility on or next to H11 = 5 | nodes 2 (H1, H2), cards played 5, drawn 6, troop removed 1, hardening 1, transactions 15 = 30 | 12: 5 plays (of 9 in hand / ~40), BBB over which building (5 / 5), 2 node hexes (8 / 8, 5 / 5), PL, Mint, Fortify target + hardened facility, pass | Fortify removes the H11 troop; hardening put on H8 (neutral factory next to H11: no own facility). Aegis Grid Lockdown "Place 3 Nodes": only 1 affordable (0 energy; next spaces cost). |
| D Protector | §5 (Accord), §8 (Buy energy) | Grad Student Pool -5 on Microgrid (Science tag) = 1; Outpost/Expand/Accord track costs 1 + 0 = 1; total 2 | Outpost, Expand, Accord used; city bought (C7); host plan (C15) = 5 | Accord: cities newly in reach (H6, H7) 2; buy: 2 neutral cities hostability 2 = 4 | nodes 3 (C11, H14, H10), ambassadors 2 (Outpost to supply, Accord onto H7), cube 1 (claim C14), cards played 1, transactions 7 = 14 | 8: 3 node routes + hexes (9 / 9, 10 / 10, 10 / 10), Accord city (2 / 2), Microgrid target (3 / 3), buy city (2 / 2), threats left unpaid (2 threats), pass | Protector leaves Provocation unpaid with 3 ambassadors in supply and C13 next to its nodes. |
| E1-2 Factories, cities | §9 | 0 | host matching | 5 factories; 6 cities = 11 | P chip 1, Discord 1 (C10; H7 blocked by P's ambassador), energy 2 = 4 | 1 host match: 4 neutral cities, 2 hosts (B: H6 or H14; P: H6, H14, C7, C15) | Discord 5. |
| E3-4 Stations | §9 | 0 | — | 7 stations (H10, C2 blocked by agents) | cubes 3 (H4 adv, C11 n, C12 adv) | 0 | Tidal 1n+1a, Polar 3n+2a. |
| E5 Income | — | B 15+3+2+3+3+1 (RS) = 5 sources; P 13+2+3+3, 2+1 = 5; total 10 | — | 0 | 4 | 0 | B 27 cr 2 en 0 ch; P 22 cr 4 en 1 ch. |
| E6-8 Growth | §9 (build: which hex) | 0 | — | 2 bases; 6 cities grow; H14 builds: count factories vs stations next to it (2 vs 1), free open land next to it (none), within 2 in its country (H9, H16), next to an agent (H9) = 12 | city chips 4, chips off 3, station 1 = 8 | 0 | **H14 builds a neutral station on H9** (B's starting node hex). The longest single resolution of the round: 4 sub-tests. |
| F Threats | §10 (nearest, tie), §18 (dealt after the action phase: waits) | War Footing +4 x 2 = 2 | Agitators waits (token shows it) | nearest to H6: B (1) vs P (1) tie = 2; Agitators location (C7 vs C15) 2 = 4 | B discards 3 cards, Agitators card 1 + token 1, Discord 1 (+2), compute 8, threats + tokens off 4 = 18 | 3: who takes Blackout (2 / 2), B pays 3 energy or discards 3 (B has 2 energy: discard), which 3 (9 / 9,8,7) | Discord 7 (Provocation +2). Agitators waits on C7. |
| G War, H Cleanup | — | 0 | Shelter ends | 2 | 0 | 0 | |
| **total** | **13** | **28** | **peak 12** (B 7 + P 5) | **91** | **170** | **44** | |

State end of round 2 (mine): agents H10 1, C2 1; botnets H4, H7, C12, C11; P amb on H7; B node H7, H1, H2, H5,
H9, H13, C13; P nodes C6, C10, C11, C14 (P's factory), H10, H14; station H9 (new, neutral); hardening H8; chips H6
2, H7 2, H14 0, C7 2, C10 1, C15 1; Fallout H11; no troops. Discord 7, Unity 0. Agent income 2/country, pools 2/2.
Accelerated Timetable 34/40, waiting 8. Tidal 1n 1a; Polar 3n 2a. Agitators on C7 (waits). B 27 cr 2 en 0 ch
0 amb, hand 6; P 22 cr 4 en 1 ch 3 amb, hand 5.

**Check.** (Process note: my first write of this section failed on an edit anchor and I had already opened the
round-2 check; the table above is the text composed before that read, unchanged.) Pieces all match, including the
contested botnet-spread parse (H7, C11) and the built station on H9.
**Error 2:** Fortify's hardening: I put it on the neutral factory H8; the engine placed none (Fortify hardens a
facility on or next to the troop, and §13 says hardening goes on a *player-owned* facility; with none of B's there,
nothing). Removed.
**Error 1, diagnosed:** engine compute 16 -> 38 (+22) vs mine +18: the engine's compute income is **14 a round (7 a
country)**, not §21's 5 a country (10). Both rounds fit 7: R1 14 + 2 botnets = 16; R2 14 + 4 botnets + 4 waiting =
22. Nothing in the rulebook gives 7; carried 7 a country from here and logged it as a rulebook/engine disagreement.

## Round 3 (Probing Strikes; Discord tips; Unity tips three times)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | — | 0 | Dispatches B 2, P 2; keep upgrade both | 0 | insight 2 | 0 | Insight 7 each. |
| A Sway + **Discord tip 1** | §12 (tipping), §19 (tip drones) | tip at 4 x 2 = 8: 8 - 8 = 0 = 1 | — | Fallout 1; drones: neutral factories with no agent/amb per country (H2, H8, H13; C3) = 4; Propaganda: count corrupted cities (H7, C10) = 2; total 7 | Discord 2 moves, tip counter 1, Discord card 1, drones 2 (H15, C13), credits 2 = 8 | 0 | Propaganda I: each player loses 2 x 2 = 4 credits (B 23, P 18). |
| B Card: threats | — | 0 | — | **10 threat cards turned** for one Security fit per country (Grid Tap, Shakedown, Power Surcharge, Sabotage, Credit Crunch, Insider Access, Defection out; Eviction H; Load Shedding out; Supply Raid C) + locations 4 = 14 | card 1, threats 10 (8 discarded), tokens 2 = 13 | 0 | Eviction on H6, Supply Raid on C3. |
| B Rally coastal cities | §2 (which cities are coastal) | 0 | — | 4 coastal cities (H6, H7, C7, C15) = 4 | agents 4, H7 agent + P's ambassador canceled 2 = 6 | 0 | |
| B Corrupt cities | §15 | city at 3 = 1 | — | targets 4; agents 5: H10's next to H6 and H14 (2 targets: stays), C2's next to none; natural 12; nodes 2-agent 15 = 32 | 0 | 0 | |
| B March | — | 0 | — | 0 | 0 | 0 | |
| B Income + **project resolves** | §17 (overflow, slide, refill), §18 (deal tokens) | compute 7 x 2 = 14 (engine rate) = 1 | — | 4 botnets; Accelerated Timetable overflow 24; 2 threat locations x 2 tokens = 9 | next card 1, pool 4, compute 26 cubes, AT off 1, Lab Purge dealt 1, event card discarded 1, threats 2 + tokens 4 = 40 | 0 | Accelerated Timetable resolves: Signal Jamming discarded (round cap 12), Shock Doctrine (H6 + H2) and War Economy (C3 + C13). **By §17 the 24 overflow then fills Purge Lists (14) and it resolves too; the table reveals only AT resolving.** Followed the table; logged. |
| C Cards | §7 (keep upgrade, Science discount on tier-2 keeps) | P: 2 Science tags take 1 off each tier-2 keep (2 -> 1) x 2 = 2 | — | Science tags count 1 | build 8, passes 6, keeps 4, discards 4 = 22 | build 8, draft 8, keep 4 = 20 | 7 of 8 tier 2. P keeps Rally the Allies, Fusion Research Campus for 1 insight each; B keeps Collateralized Grid, Coalition Charter for 2 each. |
| D Builder | §7 (CPO), §12 (Unity vs Discord choice), §7 (Sacrifice) | CPO +10 x 2 = 2; Mesh node space 6 costs 3, Expand card-play space 3 costs 1 = 2; total 4 | Mesh, Expand, Scrap, Procure used; Dispatch 1 of 2 used; Red-Team Audit's jam (on a threat; nothing marks which: event discarded) = 6 | Unity tip 1 (Citizens' Assembly): count corrupted sites 4 (max 2) = 4 | nodes 2 (H4, C9), ambassador 1 (H4), cards played 3, drawn 4, sacrificed 1, Unity marker 2 + tip counter 1 + Unity card 1, transactions 14 = 30 | 10: 3 plays (of 9 / ~45), Unity to Discord? (trivial: Discord 0), 2 node hexes (4 / 4, 3 / 3), Dispatch hex (14 / 14), Scrap sacrifice (3 / 3), Procure, RTA target (4 threats), pay Eviction 2 cr, pass | Coalition Charter's 6 Unity tips Unity (8 -> 2), Citizens' Assembly +2. Scrap sacrifices Collateralized Grid, losing the +2 energy income it just bought. |
| D Protector | §8 (Liberate), §12 (tips mid-action), §16 (Push) | Liberate 5 x 2; Outpost + space 3 (1) = 1; total 3 | Patrol, Outpost, Bulwark used; city bought; Shared insight 4 received (9 total, unspent) = 5 | Mutual Aid: owned/hosted count 1; Popular Mandate: corrupted cities 0; Patrol target (5 options); Liberate: agents on H7? C10? 2; buy-energy hostability 6 cities = 10 | ambassadors gained 5 + 1 + 1, spent 10 (liberates), node 1 (H15), agent pushed 1, adversary cubes off 2, Liberated markers 2, hardening 1 (C14), Unity marker 2 + tip counters 2 + Unity cards 2, Rally the Allies 1, transactions 10 = 42 | 9: Rally target (none: no ambassador on the map, gain only), Patrol hex + push (5 / 5, 7 / 7), 2 Liberates (3 / 3, 2 / 2), Outpost hex (10 / 10), Bulwark target, buy city, pay Eviction 1 cr | Two Liberates (+5 Unity each) tip Unity twice more: Mutual Aid I (each player 1 energy + 1 ambassador), Popular Mandate II (0: no corrupted city left). **No corrupted city on the map now.** |
| E1-2 Factories, cities | §9 | 0 | host matching | 5 factories; 7 cities = 12 | P chip 1, energy 2 = 3 | 1 host match: 3 unblocked neutral cities (H7, H14, C10), 2 hosts | No Discord (no corrupted city). |
| E3-4 Stations | — | 0 | — | 7 stations | cubes 4 (H9 n, H10 n, C11 n, C12 a) | 0 | Tidal 3n 1a; Polar 4n 3a. |
| E5 Income | — | B 15 + 3+2+3+3+3 + RS 1, 1 en = 7; P 13+2+3+3, 2+1 = 5; total 12 | — | 0 | 4 | 0 | B 30 cr 3 en; P 21 cr 4 en 2 ch. |
| E6-8 Growth | §9 | 0 | — | 2 bases; grow 6 cities; build: H6 (3: wants factory; no free zone within 2), H7 (3: factory; none), C7 (3: tie -> factory; C9 within 2) = 3 x 4 sub-tests = 20 | chips 4, chips off 3, factory 1 (C9) = 8 | 0 | H6 and H7 sit on 3 chips with nowhere to build. C7 builds a neutral factory on C9 (Builder's node hex). |
| F Threats | §10 (jam), §18 | MSB -6 on Supply Raid, War Economy, Shock Doctrine (Universal = every tag); RTA -6 on Eviction = 4; War Footing +4 x 2 = 2; total 6 | refunds owed (B 2, P 1) | nearest: Eviction (B 1 vs P 1 tie), Shock Doctrine (B 0), Supply Raid (P 1 vs B 2), War Economy (none) = 6 | refunds 2, B discards 2, P discards 1, troop 1 (C3), compute 12, threats + tokens off 4 + 6 = 28 | 4: who takes Eviction (2 / 2), B discards which 2 (9 / 9, 8), P pay 2 chips or discard 1 (2 / 2), which (6 / 6) | **Shock Doctrine and War Economy cost 4 energy - 6 = 0: paid off at nothing.** Market Stabilization Board makes every energy-priced Industry or Universal threat free. Eviction (12 - 6 = 6) was only part-paid (3): refunded, executes. |
| G War | §11 | 0 | — | 2 bases, cancel 1 v 1 | drones off 2 | 0 | |
| H Cleanup | — | 0 | — | 0 | 0 | 0 | |
| **total** | **13** | **35** | **peak 13** (B 6 + P 5 + refunds 2) | **133** | **206** | **52** | |

State end of round 3 (mine): agents H6 1, H11 1 (pushed), C2 1, C7 1, C15 1; troop C3; botnets H4, H7, C11, C12; B
amb on H4; B nodes H1, H2, H4, H5, H7, H9, H13, C9, C13; P nodes C6, C10, C11, C14, H10, H14, H15; H7 and C10 neutral
(Liberated markers); C9 factory (new, neutral); hardening C14; chips H6 3, H7 3, H14 1, C7 0, C10 1, C15 1; Fallout
H11. Discord 0 (1 tip), Unity 2 (3 tips). Projects: Purge Lists 0/14, Grand Offensive 0/44, Lab Purge 0/22 (rest
of compute: see check). Waiting 12. Pools 2/2. Next event: Factory Drive. Tidal 3n 1a, Polar 4n 3a.

**Check.** Pieces match except C7: **Error 3:** I forgot **Agitators**, the threat Provocation dealt into Cintamani
last round with a *waits* token: it executes this round's F (3 agents created on C7, Sway 2 Discord, +4 War
Footing). Engine: C7 4 agents, Discord 2, waiting 16 (mine 0 and 12). The waits token was on the table, but F is
resolved from the threats dealt *this* round, and nothing in the step list reminds you to sweep waiting threats.
Corrected. **Compute (Error 1 continued):** the engine's `adversary.compute` reads 16, 38, 68: it is a running total
(Accelerated Timetable resolved when it passed 40). This round's +30 is 4 above my 14 + 4 botnets + 8 waiting;
whatever the engine adds, I can't reproduce it from the book. Purge Lists did **not** resolve on the overflow, so
either overflow doesn't carry past one resolution or the counter isn't what sits on the projects: §17 says it carries.
Carried the engine's numbers.

## Round 4 (Factory Drive; Polar Compute Array completes)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | — | 0 | Dispatches B 2, P 2 | 0 | insight 2 | 0 | Insight 8 each (card-play space 3). |
| A Sway | — | 0 | — | 1 | Discord 1 | 0 | Discord 3. |
| B Card: threats | — | 0 | — | 7 threat cards turned (Backdoor, Cooling Failure, Smear Campaign out; Chip Shortage H; Cryptojacking, Wiper Malware out; Commandeer C) + locations 3 (H factories 3-way tie, C base, C factories 3-way tie) = 10 | card 1, threats 7, tokens 3 = 11 | 0 | Chip Shortage on H2; Commandeer on C13 + C3. |
| B Rally factories, factory opens | §6 | 0 | — | 6 factories dealt to; free industrial zones per country (H none; C16) = 10 | agents 4, factory 1 (C16) = 5 | 0 | |
| B Corrupt factories | §15, §13 | factory 2 = 0 | — | targets 7; 6 agents' neighbors: H6's -> H2, H11's -> H8, C2's -> C3, C7's 4 -> C3, C15's next to C14 + C16 (stays) = 13; corrupt 7; natural 13; nodes 2-agent 17 = 50 | agents moved 7, adversary cubes 3, agents back 10, B node destroyed (H2) 1 = 21 | 1 (protect H2 node? nothing to spend) | **H2, H8, C3 corrupt.** Builder loses the node on H2. A city line's agents feed a factory line: the 4 Agitators agents on C7 walk onto C3. |
| B March | §6, §5 (Wall: do the Protector's nodes count as targets?) | 0 | — | 1 troop: on/next to C3 nothing strikable; paths around Wall hexes C6, C10, C11 to the nearest target (B's C9: C3-C2-C5) = 4 | troop 1 (C3 -> C2) | 0 (one best step) | Ends next to P's C6: **Vigil +1 insight** (P 9). |
| B Income + Purge Lists | §17 | compute 7 x 2 = 1 | — | botnets 4 (H4 has B's ambassador) | next card 1, pool 4, compute ~33, Purge Lists off 1, Asset Seizure dealt 1, B's ambassador lost 1 = ~41 | 0 | Purge Lists: B has none in supply, loses the one on H4; P has none anywhere. Next: Fifth Column. |
| C Cards | §7 | 0 | — | Science tags 1 | build 8, passes 6, keeps 3, discards 5 = 22 | build 8, draft 8, keep 3 = 19 | Demand Response Grid (drafted, discarded) is not in `docs/cards.md`: a card with no printed text. |
| D Builder | §7 (Industry discount, "over diplomacy" plate, tags of covered buildings), §20 (Rush Build), §8 (Dispatch from map) | CPO +10 x 6 (OA, OE, JFP, AF, NM, Cyclotron) = 6; Industry -1 x 3 then -2 x 1 = 4; Open Assembly At 2 (1 on play + 1 own trigger) = 1; Orbital Embassy ambassador per Diplomacy tag x 2 = 2; NM per 2 Cyber, Research Royalties per 2 Science = 2; track space cost 2 = 1; total 16 | Mesh, Rush Build, Scrap, Mint, Procure used; Dispatch 2 of 2; city bought (shared); CPO / OE payouts; Industry tag count now 4 then 3 (changes as JFP is played then sacrificed) = 9 | tags counted: Industry 4x, Diplomacy 1x, Cyber 1x, Science 1x = 7; Eviction Worm target agents in reach 2 = 9 | node 1 (H2), cards played 9, drawn 6, sacrificed 2, ambassadors 2 gained + 2 placed, agent removed 1, compute off 4, station 1, Unity marker 1, followers 2, transactions 30 = 61 | 17: 9 plays (of ~13 / ~40), 3 build-over choices (5 / 5 each), Unity on track or off Discord (4 / 4), 2 Dispatch hexes (16 / 16) + which ambassador (2 / 2), station hex (4 / 4), Rush Build sacrifice (4 / 4), Scrap sacrifice (3 / 3), node hex (4 / 4) | Builder plays 9 cards and 5 powers in one action phase. **The round's heaviest step: every play moves the credit total by a discount, a +10 and a tag count.** |
| D Protector | §16 (Push), §8 (Buy energy twice) | GSP -5 on Telemetry Archive = 1; Telemetry At 4 (2 + 2 own trigger) = 1; track cost 2 = 1; total 3 | Patrol, Expand, Bulwark used; Microgrid Action used; cities bought (H14, C7); host plan; Vigil insight = 7 | buy: 5 unblocked neutral cities vs 2 host slots 5; Microgrid targets 7 = 12 | node 1 (H16), agent pushed 1, cube 1 (claim C11), hardening 1 (C6), station 1 (H16), datasets 4, cards played 1, transactions 10 = 20 | 9: Patrol hex + push (3 / 3, 5 / 5), node hex (9 / 9), TA slot, 2 buys (3 / 3, 2 / 2), Microgrid target (7 / 7), Bulwark target (3 / 3), station hex (3 / 3) | P ends with 11 insight unspent. |
| E1-2 Factories, cities | §9 | 0 | host matching | 7 factories; 6 cities = 13 | troops 3 (H2, H8, C3), P chip 1, energy 2 = 6 | 1 host match (5 neutral unblocked, 2 hosts) | Three corrupted factories each make a troop; H2's lands on the Builder's node hex. |
| E3-4 Stations + **Polar completes** | §9, §8 (payout table) | Polar 11 cubes: payouts B 1 x (draw 1, 5 cr), P 1 x, adversary 4 x 1 compute = 3 | — | 9 stations; 2 megaprojects; cubes sorted by color 11 = 22 | cubes 8 placed, 11 off, cards drawn 2, credits 2, compute 4 = 27 | 0 | Converters open: Cintamani's (1 research cube -> 2 insight; 2 chips -> draw 3). |
| E5 Income | — | B 15 + 14 + 2 + RS 1 + AF 7 + RRoy 2 cr, 1 + 2 en, 1 ch = 8; P 13 + 11, 2 + 1 = 4; total 12 | — | 0 | 6 | 0 | B 47 cr 5 en 3 ch; P 29 cr 4 en 3 ch. |
| E6-8 Growth | §9 | 0 | — | 2 bases (troop on or next?); grow 6; build H6 and H7 (3 chips, want factories, no free zone) 2 x 4 = 16 | chips 3 | 0 | |
| F Threats | §10 (jam to 0) | MSB -6 on Chip Shortage (3 chips -> 0) = 1; War Footing +4 = 1; total 2 | — | nearest to Commandeer (B on C13: 0) 1; Chip Shortage paid at 0 1 = 2 | B discards 3, compute 6, threats + tokens off 2 + 3 = 14 | 2: pay 3 energy or discard 3 (2 / 2), which 3 (7 / 7,6,5) | Chip Shortage free again (MSB). B discards 3 rather than pay 3 of 5 energy. |
| G War, H | — | 0 | — | 2 | 0 | 0 | |
| **total** | **14** | **38** | **peak 16** (B 9 + P 7) | **150** | **246** | **51** | |

**Error 4 (found by my own arithmetic this round):** in round 3 I dropped Collateralized Grid's +2 energy income
when Scrap sacrificed it. A "raise your income" line is a lasting track raise that outlives the card: §23 says so in
the icon legend (*Brown box with a track arrow*), §7's *Sacrifice* paragraph doesn't. Builder had 5 energy at the
start of round 4, not 3 (the round's plays only add up with 5).

State end of round 4 (mine): agents C15 1; troops H2, H8, C2, C3; botnets H4, H7, C11, C12; corrupted: H2, H8, C3
factories, H4, C12 stations; B amb H4, H7; B nodes H1, H2, H4, H5, H7, H9, H13, C9, C13 (H2 re-placed); B station
C6 (hardened); P nodes C6, C10, C11 (P's station), C14 (P's factory, hardened), H10, H14, H15, H16 (P's station);
factory C16 (neutral); chips H6 3, H7 3, H14 2, C10 2, C15 2. Discord 3, Unity 5. Polar complete. Tidal 5n 1a 1P.
Waiting 10. Projects: Grand Offensive (-4 Eviction Worm), Lab Purge, Asset Seizure.

**Check.** Every piece matches (the three factory corruptions, the Builder's re-placed H2 node, troops, stations,
hardening, chips); Discord 3, Unity 5, waiting 10 match. Compute: the engine counter moved 68 -> 91 (+23); my
arrivals were 14 + 3 botnets + 16 waiting - 4 (Eviction Worm) = 29. The counter's increments (16, 22, 30, 23) fit no
single income rate, so I stop trying to reproduce it: **a player can't audit the adversary's compute from the book
alone** (logged in friction). No new errors.

## Round 5 (Fifth Column; Grand Offensive resolves; Tidal Grid completes)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | — | Neural Mirror +2 insight income = 1 | Dispatches B 2, P 2 | 0 | insight 2 | 0 | B 10 insight, P 8. |
| A Sway | — | 0 | — | 1 | Discord 1 | 0 | Discord 4. |
| B Card: threats (tier 2) | — | 0 | — | 5 threat cards (Factory Occupation, Power Rationing, Grid Collapse out; Zero Day H; Purge Protocols C) + locations 6 = 11 | card 1, threats 5, tokens 2 = 8 | 0 | Zero Day on H9, Purge Protocols on C2. |
| B Rally cities, income, Botnets spread | §6 | agent income 2 -> 3 = 1 | — | 6 cities dealt; spread: sites next to 2 botnets per country (H6, H8, H10; C7, C10, C14, C15, C16) = 14 | agents 4, H7 agent + B's ambassador canceled 2, income marker 1, botnets 2 (H6, C7) = 9 | 0 | |
| B Corrupt cities (1 fewer) | §15 ("1 fewer"), §8 (Liberated +1) | 3 - 1 = 2; Liberated H7, C10: 2 + 1 = 3 = 3 | — | targets 6; agents all on targets; corrupt 6; natural 13; nodes 2-agent 20 = 45 | 0 | 0 | Nothing: every city holds 1. Three modifiers stacked on one printed number (5 -> 3 at d2 -> 2 for "1 fewer" -> 3 with a Liberated marker). |
| B March (4 troops) | §6, §5 (Wall, Vigil) | 0 | Vigil count (2 of 2) | H2 troop: 3 targets; H8 troop: 2 targets; C2, C3 troops: Wall hexes, paths to B's C9 = 4 troops x ~3 = 12 | B nodes destroyed 2 (H2, H7), troops used up 2, troops moved 2 (C2 -> C5, C3 -> C2) = 6 | 2: H2 troop's target (3 / 3), H8 troop's target (2 / 2) | **Builder loses nodes on H2 and H7.** Both Cintamani troops end next to P's C6: Vigil +2 insight. |
| B Income + **Grand Offensive** | §17, §15 | compute 7 x 2 = 1; GO's Corrupt cities (1 fewer) 2, Liberated 3 = 2; Corrupt factories (1 fewer): 2 - 1 = 1 = 1; total 4 | — | GO Rally: 6 cities; Corrupt cities: 6 targets + 7 agents; Corrupt factories: 4 targets, agents on H14 (2 targets), C10 (2 targets) = 6; natural 13 = 38 | next card 1, pool 6, GO agents 6, adversary cubes 3 (H6, C7, C15), agents back 6, compute ~30, GO off + War Plants dealt 2 = ~54 | 0 | **H6, C7, C15 corrupt** (2 agents each at the reduced number). C10 holds at 2 thanks to its Liberated marker. Next: Station Raids. |
| C Cards | §7 (Science discount on keeps) | B: 4 Science tags = 2 off each card, floor 1: tier-2 keep 2 -> 1 = 1 | — | Science 4 | build 8, passes 6, keeps 4, discards 4 = 22 | build 8, draft 8, keep 4 = 20 | 3 cards (Capital Injection, Stress Test Suite, Credential Harvest) are not in `docs/cards.md` under the ids the table uses: their engine ids (materials_hedge_desk, benchmark_suite, spoofed_credentials) are old names. |
| D Builder (grouped) | §8 (Rescue), §12 (Unity vs Discord), §7 (Science discount on plays; tags on covered buildings for SCHQ's plate), §16 (Liaison Corps' move) | CPO +10 x 8 = 8; Industry -2 x 5 buildings = 5; Science -1 insight x 4 plays = 4; SCHQ +10 on troop removal = 1; Credential Harvest per Cyber tag (5) = 1; Unity 5 vs Discord, Unity 1 vs Discord = 2; total 21 | Cyclotron, RR, Rush Build, Scrap, Expand, Procure used; Dispatch 2 of 2; city bought; Security tag count for SCHQ; B's 1 research cube on Zero Day (to come back) = 11 | Security tags 2, Cyber tags 5, Science tags 4, Industry 4; NO: facility next to H6 (2); LC: pieces on or next to C10 (8) = 21 | antimatter 2, cube off + Liberated marker on H6, claim cube H9, Discord/Unity markers 3, ambassadors gained 3 + 2, placed 6, agents removed 2, troop removed 1, cards played 14, drawn 15, buildings stacked 5, sacrificed 2, node 1, research cubes 3 in + 1 out, transactions ~35 = ~95 | ~26: 14 plays (of ~17 / ~50), 5 build-over choices (5 / 5), NO claim (2 / 2), Unity split x 2 (6 / 6, 2 / 2), LC target + 4 moves (16 / 16, 8, 8, 8, 7), DE target, CM target, 2 Dispatches (16 / 16 + 4 / 4, 5 / 5), node hex, 2 sacrifices, Zero Day pay | **Builder: 14 cards played, 15 drawn, 6 powers, in one action phase.** The Nuclear Option rescues H6 (+5 Unity, all put against Discord), claims H9. Five buildings stacked in one slot this round alone. |
| D Protector | §13 (hardening a node), §8 (converter) | 0 | Patrol, Expand, Debrief used; Cintamani chip converter used; Vigil insight 2 = 5 | STS: 3 node hexes of 7; Debrief target 1 = 4 | agent pushed 1, agent removed 1, node 1 (H6), hardening 3, ambassador gained 1, cards drawn 4, sacrificed 1, transactions 6 = 18 | 7: Patrol (5 / 5, 7 / 7), node hex (10 / 10), STS 3 hexes (7 / 7, 7, 6), Debrief sacrifice, converter | |
| E1-2 Factories, cities | §9 | 0 | host matching | 7 factories; 6 cities = 13 | troops 3, P chip 1, Discord 1 (C15; C7 blocked by B's ambassadors), energy 2 = 7 | 1 host match (3 unblocked neutral: H6, H14, C10) | |
| E3-4 Stations + **Tidal completes** | §9 (stations after completion), §8 (payout) | Tidal 10 cubes: B 1 x (3 en, 3 cr), P 2 x, adversary 1 Discord = 3 | C12's research cube is owed at the next Income beat | 9 stations; Polar complete: C2 neutral cube lost, C6/C11 to boards, C12 to research track next Income = 13 | cubes 4 + 10 off, research cubes 2, transactions 4, Discord 1 = 21 | 0 | Discord 2. Both converters now open. |
| E5 Income | — | B 15 + 14 + 2 + RS 1 + AF 7 + RRoy 2 + GC 10 + VF 3 = 8, en 1 + 2, ch 1 = 2; P 13 + 13, 2 + 1 = 4; total 14 | — | 0 | 6 | 0 | B 57 cr 9 en 4 ch 3 rc; P 34 cr 10 en 3 ch 1 rc. Sacrificed programs' raises stay (Error 4's lesson). |
| E6-8 Growth | §9 | 0 | — | 2 bases; grow 6; 5 cities at 3 chips, none can build (H6, H7 want factories; H14 a station; C10 tie -> factory; C15 factory) 5 x 4 = 27 | chips 3 | 0 | Five cities stuck at 3 chips, re-tested every round for nothing. |
| F Threats | §13 (protection order, hardening on a hex with another player's facility) | War Footing +4 x 2 = 2 | — | Zero Day nearest (B 0); Purge Protocols nearest (P 1 vs B 2) and P's closest node (C6) = 4 | research track 4, building face down 1, refund 1, hardening spent 1, P discards 3, compute 8, threats + tokens off 4 = 22 | 3: which building face down (5 / 5), spend the hardening (2 / 2), which 3 to discard (10 / 10, 9, 8) | Zero Day's building stays face down "until the end of next round": a timer with no marker. P saves C6 with the hardening token Bulwark put on B's station there. |
| G War, H | — | 0 | Agency Field Station face down till end of round 6 | 2 | 0 | 0 | |
| **total** | **14** | **59** | **peak 18** (B 11 + P 5 + Vigil + C12 cube owed) | **227** | **~367** | **~68** | |

State end of round 5 (mine): agents H7 1; troops H2, H8, C2 (C5's removed), C3; botnets H4, H6, H7, C7, C11, C12;
corrupted H2, H8, C3, H4, C12, C7, C15; neutral liberated H6, C10 (marker on both), H7; B amb H4, H6, H9, C7 x2,
C10?, C11; B nodes H1, H2, H4, H5, H9, H13, C9, C13; B stations H9, C6; P nodes C6, C10, C11, C14, H6, H10, H14,
H15, H16; hardening H6, H10, H14, C14; chips 3 on H6, H7, H14, C10, C15. Discord 2, Unity 5. Research track 4.
Both megaprojects complete. Waiting ~8.

**Check.** Corruptions (H6 rescued, C7, C15), troops, the two destroyed Builder nodes, hardenings, chips, Discord 2,
Unity 5 and waiting 8 match. Three mismatches:
- **Error 5:** research track: mine 4, engine 5. Polar completed in round 4's production, so C12 already fed the
  research track in **this** round's Income beat (1), before Zero Day's 4. I had pushed it a round later. The
  station's two-stage behavior (megaproject cube until complete, research-track cube after, but in a *different
  phase*: Income beat, not production) is a memory item with no marker.
- **Error 6:** agents: I left 1 agent on H7 (Grand Offensive's rally) and took Debrief's agent off H11; the engine
  has neither H7's nor H11's agent. I can't find the second removal in the cards played (candidates: Debrief on one,
  nothing on the other). Corrected to the engine's.
- **Error 7:** B's ambassadors: I put 2 on C7 from Liaison Corps (as the table's four moves read: C10, C7, C7, C11);
  the engine has 1 on C7. I also could not account for a 4th ambassador to move (B had 3 after the gain), so my
  ambassador supply count was already off by one. Corrected: B ambassadors on H4, H6, H9, C7, C11.

From here, to keep inside the time cap, I stopped carrying each player's exact credits / energy / insight (the
check doesn't show them, and the team's choices come from the script); every payment is still counted as a
component and every discount or payout as a modifier.

## Round 6 (Station Raids; Unity tips twice)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | — | B insight 8 + NM 2 = 1 | Dispatches B 2, P 2; Agency Field Station face down (Zero Day) | 0 | insight 2 | 0 | |
| A Sway | — | 0 | — | 1 | Discord 1 | 0 | Discord 3. |
| B Card: threats | — | 0 | — | 2 threat cards (Total War H, Last Ultimatum C), locations 3 (H base; C city: only C10 not corrupted; C base) = 5 | card 1, threats 2, tokens 3 = 6 | 0 | |
| B Rally stations, spread, income | §6 | agent income 3 -> 4 = 1 | — | 8 stations dealt in order (3 per country, 6 dealt); spread: sites next to 3 botnets per country = 10 sites seen = 16 | agents 6, 3 cancel B's ambassadors (H4, H9, C11) = 6 off, botnets 2 (H2, C3), income marker 1 = 15 | 0 | Rally on stations strips three of the Builder's ambassadors. |
| B Corrupt stations | §15 | 0 | — | targets 6; 3 agents on targets; natural 13; nodes 2-agent 21 = 43 | 0 | 0 | Nothing. |
| B March (4 troops) | §6, §5 | 0 | Vigil 2 of 2 | H2 troop 3 targets; H8 troop 1 target (H4); C2, C3: Wall routes = 10 | B nodes destroyed 2 (H2, H4), troops used 2, moved 2 (C2 -> C5, C3 -> C2) = 6 | 1: H2 troop's target (3 / 3) | **Builder loses H2 again and H4.** H8's troop has one target, so no choice: it takes H4. Vigil +2 for P. |
| B Income + Lab Purge | §17, §6 (research) | compute 7 x 2 = 1 | Neighborhood Watch tokens' end (next Income beat) | research: H4, C12 (megaprojects complete, no ambassador) 2; Lab Purge: open megaprojects 0 = 3 | next card 1, pool 8, compute ~25, research cubes 2, project off + Infiltration Drive dealt 2 = ~38 | 0 | Lab Purge does nothing (both megaprojects complete). Research 7 of 8. Next: Supply Lines. |
| C Cards | §7 | P: Science 3 -> tier-2 keep 2 - 1 = 1 | — | 1 | build 8, passes 6, keeps 6, discards 2 = 22 | build 8, draft 8, keep 6 = 22 | Builder keeps all 4 (tier 1). Interceptor Battery is not in `docs/cards.md`. |
| D Builder (grouped) | §12 (tip resolves mid-action), §20 (Annex, Overcharge, Fortify), §5 (capstones: Master Plan, Full Capacity), §8 (Remove agents) | CPO +10 x ~5; Industry -2 / -4 with Master Plan on buildings x 3; Science insight -2 x ~5; Darknet -5 on Cyber cards x 1; Unity 2 vs Discord; Defense Appropriations per 2 Security; ERC +2 insight income; track costs 3, 5, 5 = ~22 | Fortify, Overcharge, Procure, Annex used; RR, Parts Exchange, Cyclotron Actions used; Cintamani converter used; Dispatch 2 of 2; city bought; Master Plan now on (new discount to remember) = 12 | Fortify facility next to C5 1; Remove agents: on or next to C2 (2 agents) 2; SGA: Fallout in reach? (no) 1; Neighborhood Watch: every site on or next to an ambassador (B's on H6, C7) ~11 = 15 | nodes 3 (H2, card-play capstone hex, C7), troop removed 1, hardening 1, ambassadors gained 3 + 2, placed 2, spent 2 (Remove agents), agents removed 2, Unity marker 2, tip counter 1, Unity card 1, **no-Rally tokens ~11**, cards played 11, drawn 16, stacked 2, sacrificed 1, compute off 5, transactions ~30 = ~105 | ~24: 11 plays (of ~20 / ~70), 2 build-over (5 / 5), 3 node hexes (2 / 2, 1 / 1, 5 / 5), Unity split x 2 (3 / 3, 2 / 2), 2 Dispatches (15 / 15 + 3, 15 + 4), Remove agents (3 / 3, 2 / 2), Fortify target, Overcharge sacrifice (4 / 4), converter, pass | Relay Mast Rollout places 2 of its 3 nodes (both card-play capstone spaces, 3 + 5 energy). Neighborhood Watch II puts a no-Rally token on every site on or next to an ambassador **as it resolves**: ~11 tokens, and the set depends on when in the action phase it tips. |
| D Protector (grouped) | §8 (Liberate), §12 (Disarmament Talks), §5 (Sanctuary capstone) | Outpost 3 + space 5 = 1; Liberate 5 = 1; Unity 5 on track (tip) = 1; TIB 8 cr; Science keep = 4 | Summit Caucus (no target), Outpost, Expand, Patrol, Bulwark used; TIB Action used; chip converter used; city bought; Vigil 2 insight; Total War part-pay (to come back) = 10 | Liberate C15 agents? 1; Disarmament Talks: troops on or next to an ambassador (none) 3; Patrol 2; Bulwark 5; buy 2 = 13 | nodes 2 (H7, H4), ambassadors gained 3 + 1 + 1 + 1, spent 5, placed 2, antimatter 2 + 1, cube off + Liberated marker (C15), Unity marker 1, tip counter 1, Unity card 1, agent pushed 1, hardening 1, cards played 2, drawn 3, stacked 1, transactions ~12 = ~40 | ~12: Summit Caucus, Outpost hex (8 / 8), TIB slot (4 / 4), Expand hex (9 / 9), Liberate hex (7 / 7), Unity split (2 / 2), Patrol (2 / 2, 7 / 7), Bulwark (5 / 5), buy (2 / 2), 2 Dispatches (25 / 25 + 2 / 2), pay Total War | Liberate C15 tips Unity (5th): Disarmament Talks finds no troop on or next to an ambassador. Sanctuary uncovered: P's nodes immune to corruption and agents. |
| E1-2 Factories, cities | §9 (ambassador blocks a corrupted factory) | 0 | host matching | 7 factories; 6 cities = 13 | troops 2 (H8, C3; H2 blocked by B's ambassador), P chip 1, energy 2 = 5 | 1 host match: 5 neutral unblocked cities, 2 hosts | No Discord: C7 blocked by B's ambassador, C15 liberated. |
| E3-4 Stations | §9 | 0 | — | 9 stations (complete megaprojects: owners' cubes to boards, neutral lost) = 9 | research cubes 4 (B 2, P 2) | 0 | |
| E5 Income | — | B ~10 income sources, P ~6 = 16 | — | 0 | ~6 | 0 | Builder's income line is now ten separate raises to add. |
| E6-8 Growth | §9 | 0 | — | 2 bases (troop on or next?); grow 6; 5 cities at 3 re-tested, none can build = 28 | 0 | 0 | |
| F Threats | §5 (Wall stops a threat's troops), §10 (jam) | MSB -6 on Last Ultimatum (Universal: 5 rc -> 0, free) = 1; WF +4 = 1 | — | Total War: base H15 has P's node (Wall) 1; LU free 1 = 2 | refund 1, compute 4, threats + tokens off 2 + 3 = 10 | 0 | Total War's 2 troops never land (Wall); Last Ultimatum costs nothing. |
| G, H | — | 0 | — | 2 | 0 | 0 | |
| **total** | **15** | **~50** | **peak 22** (B 12 + P 10) | **~165** | **~265** | **~68** | |

State end of round 6 (mine): agents H11 1 (pushed), C2/C6 removed, H10 none; troops H8 x2, C2, C3; botnets H2, H4,
H6, H7, C3, C7, C11, C12; corrupted H2, H8, C3 factories, H4, C12 stations, C7 city; liberated H6, H7, C10, C15;
B nodes H1, H2, H5, H9, H13, C7, C9, C13 + 1 (relay node, hex unknown); P nodes add H4, H7. Unity 1 (5 tips), Discord
1 (1 tip). Research 7. No-Rally tokens out.

**Check.** Matches on Unity 1 (5 tips), Discord 1, research 7, agents (H11), hardenings, the destroyed nodes; the
unknown relay node is on **C5**. Mismatches:
- **Error 8:** H8 troops: mine 2, engine 1. The H8 troop that struck H4 is used up; I left it on the map and added
  the factory's new one. A strike removing the striker is easy to drop when the strike is automatic (one target, no
  choice to prompt the memory).
- No-Rally tokens: engine 12 (on every **hex** on or next to the two ambassadors, open H5 and C8 included); I placed
  ~11 on sites only, as the card reads ("no Rally lands on a site on or next to an Ambassador"). Not an error in
  effect (a Rally only lands on sites), but the component count is 12.

## Round 7 (Supply Lines; four threats, all free)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | — | insight income raises (NM, ERC) = 2 | Dispatches B 2, P 2 | 0 | insight 2 | 0 | |
| A Sway | — | 0 | — | 1 | Discord 1 | 0 | Discord 2. |
| B Card: threats (2 per country, two tiers) | §18 (two deals per card) | 0 | — | 5 threat cards over 2 decks (Orbital Strike H, Embargo C; Iron Curtain H, Deepfake Wave out, Skimming Operation C) + locations 6 = 11 | card 1, threats 5, tokens 6 = 12 | 0 | Orbital Strike H15; Iron Curtain H13 + H6; Embargo C9; Skimming Operation C15. |
| B Rally factories, income | §12 (no-Rally token: share to supply) | compute +1, agents 4 -> 5 = 2 | — | 7 factories dealt (4 each, 2 no-Rally hexes skip) = 8 | agents 5 placed, 3 returned (no-Rally), H13 agent + B's ambassador canceled 2, markers 2 = 12 | 0 | |
| B Corrupt factories | §15 | 0 | — | targets 4; agents 5 (H8's and H11's next to no target); natural 13; nodes 2-agent 18 = 40 | 0 | 0 | |
| B March (3 troops) | §6, §5 (Wall routing), §13 | 0 | — | H8: boxed in by Wall hexes H4, H7 and dead-end H11: no way to a target, stays; C2: B's C5; C3: B's C7 (protection: B's ambassador) = 3 x 3 = 9 | B nodes destroyed 2 (C5, C7), troops used 2 = 4 | 1: spend the ambassador on C7? (2 / 2): no | Path-finding around Wall hexes is the hard part: H8's troop has no legal path at all. |
| B Income | §17, §6 | compute (7 + 1) x 2 = 1 | — | botnets 8 (ambassadors on any?), research H4, C12 (P's ambassadors block) = 10 | next card 1, pool 10, compute ~30, no-Rally tokens off 12 = ~53 | 0 | No project resolves. Next: Forced March. |
| C Cards | §7 | Science discounts on 4 tier-2 keeps = 4 | — | Science tags 2 | build 8, passes 6, keeps 7, discards 1 = 22 | build 8, draft 8, keep 7 = 23 | |
| D Builder (grouped) | §7 (stacking), §8 (claim needs no agent), §20 (Scrap, Overcharge, Mint, Procure), §8 (converter) | CPO +10 x ~6; Industry/Master Plan discounts x 6 buildings; Science insight x ~6; Honeypot +3; Capacity Market, Economies of Scale per 2 tags x 2; Counterintel Sweep -4 research = ~22 | FAT, RR, PU, Cyclotron Actions used; Scrap, Mint, Procure, Overcharge used; Horizon chip converter used; Dispatch 2 of 2; Popular Uprising follower count (on the card: no); 3 threat jams (FPO, FCN, MSB) and which threat each covers = 12 | Field Sensor Grid / Exodus / Honeypot / FAT / BCT targets 5; TCC: no node left on either track 1; BCT: corrupted factory next to H2 (none) 1 = 7 | cards played 16, drawn ~17, stacked 6, sacrificed 2, agents removed 2, ambassadors gained 6, placed 2, cubes (claim H13) 1, adversary cube off (H2) 1, Unity 1, research cubes off 4, follower 1, transactions ~35 = ~95 | ~28: 16 plays (of ~20 / ~100), 6 build-over (5 / 5), 5 targets (2-8 / same), Unity split (3 / 3), 2 Dispatches (13 / 13 + 4), 2 sacrifices (4 / 4, 5 / 5), converter | Builder has used all 14 nodes from both tracks: Twin City Compact's "Place 2 Nodes" does nothing. Six buildings played into one slot this round (each covers the last). |
| D Protector (grouped) | §8 (Lift Fallout), §16 | 0 + Logic Bomb -6 compute = 1 | Patrol, Expand, Debrief, Bulwark used; TIB, Microgrid Actions used; Cintamani + Horizon converters used; city bought = 9 | Patrol 5; Microgrid target 4; Debrief target 3; Bulwark 7; buy 2 = 21 | agent pushed 1, removed 1, Fallout lifted 1, node 1 (C12), ambassador 1, cube (claim H2) 1, hardening 1, cards played 2, drawn 6, compute off 6, research cubes 2 off, transactions ~12 = ~35 | ~11: Patrol (5 / 5, 5 / 5), II slot (4 / 4), Dispatch (24 / 24 + 3), Microgrid (4 / 4), Expand hex (9 / 9), Lift, buy (2 / 2), Debrief (3 / 3), Bulwark (7 / 7), 2 converters | Lift Fallout on H11 ends the only Discord source left. P claims H2 the turn after Back-Channel Talks frees it. |
| E1-2 Factories, cities | §9; Twin City Compact (does the host slot outlive the sacrifice?) | 0 | host matching (now B hosts 2) | 8 factories; 6 cities = 14 | troops 2 (H8, C3), chips 3, energy 3 = 8 | 1 host match: 5 neutral cities, 3 host slots | C7 blocked by B's ambassador: no Discord. |
| E3-4 Stations | — | 0 | — | 9 stations | research cubes 4 | 0 | |
| E5 Income | — | B ~14 sources, P ~7 = 21 | — | 0 | ~8 | 0 | |
| E6-8 Growth | §9 | 0 | — | 2 bases; grow 6; 5 cities test builds: H6 finds H11 (Fallout just lifted, next to an agent, within 2) = 32 | factory 1 (H11), chips off 3 = 4 | 0 | **H6 builds a neutral factory on H11** the round its Fallout is lifted; H7, H14, C10, C15 still stuck. |
| F Threats | §10 (jam to 0) | FPO -6 (Orbital Strike 3 en -> 0), MSB -6 x 3 (Iron Curtain, Embargo, Skimming 4 amb -> 0), FCN -5 = 5 | — | 4 threats x which jam applies = 4 | threats + tokens off 4 + 6 = 10 | 0 | **All four threats paid at nothing.** |
| G, H | — | 0 | — | 2 | 0 | 0 | |
| **total** | **16** | **~61** | **peak 21** (B 12 + P 9) | **~170** | **~320** | **~80** | |

State end of round 7 (mine): agents H11 1, C16 1 (C9's pushed to C5, then removed by Honeypot; C14's by FAT; H8's by
Debrief); troops H8 x2, C3 1; botnets H2, H4, H6, H7, C3, C7, C11, C12; corrupted H8, C3 factories, H4, C12 stations,
C7 city; H2 P's factory (hardened), H13 B's factory, H11 neutral factory (new); Fallout none; B nodes H1, H2, H5, H9,
H13, C9, C13; P nodes + C12; B ambassadors C7, C10 x2, H2, H6; P ambassadors H4, H7, C12. Unity 3 (5 tips), Discord
2 (1 tip). Research 3.

**Check.** Every piece matches (H11's new factory, H2 claimed by P and hardened, both destroyed Builder nodes, H8's
two troops, agents on H11 and C16), and the tracks (Unity 3 / 5 tips, Discord 2, research 3). One open point:
the engine shows `compute_waiting=4`, so one of the four threats did execute (War Footing's 4); the likeliest is
Orbital Strike, whose troop would land on H15, the Protector's node (Wall), leaving no trace on the map. A threat
that executes with no visible effect still feeds the adversary: I had all four as paid. **Error 9** (minor: which
jam covers which threat is a memory item; I assigned Force Protection Order to Orbital Strike, the engine evidently
spent it elsewhere or not at all).

## Round 8 (Forced March; Asset Seizure; the purge in the action phase)

| step | lookups (§) | modifiers | memory items held | bookkeeping | components | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | — | insight raises = 2 | Dispatches B 2, P 2 | 0 | insight 2 | 0 | |
| A Sway | — | 0 | — | 1 | 0 | 0 | No Fallout left: the first round with no Sway. |
| B Card: threats | §18 ("any tier-2") | 0 | — | 2 threat cards (Thermal Runaway H, Data Breach C), locations 3 = 5 | card 1, threats 2, tokens 3 = 6 | 0 | Thermal Runaway H9; Data Breach C2 + C16. |
| B Rally cities, income, Forced March | §6 | agent income 5 -> 6 = 1; Forced March +1 step = 1 | Forced March: troops take 2 steps this round (card shows it) | 6 cities dealt 5 each in rounds = 10 deals | agents 10 placed, 5 cancel ambassadors (B's on H6, C7, C10 x2; P's on H7) = 10 off, income marker 1 = 21 | 0 | Five ambassadors lost to one Rally. |
| B Corrupt cities | §15 | city 3, Liberated +1 = 2 | — | targets 5; agents: C7's next to C10 only (moves), H11's next to H7 + H14 (stays), C16's next to C15 only (moves); natural 13; nodes 2-agent 20 = 45 | agents moved 2 | 0 | C15 reaches 2 (of 3): holds. |
| B March (3 troops, 2 steps) | §6, §5 | 0 | Vigil | H8's two: no path (Wall hexes H4, H7; H11 a dead end of Wall hexes) 2; C3's: two steps C2 -> C5 = 3 | troop moved 1 (x2 steps) | 0 | Vigil +1 (C5 next to C6). |
| B Income + **Asset Seizure** | §17 ("on or next to" counts 7 hexes) | compute rate = 1 | — | Asset Seizure: agents on and around each neutral factory: H11 (3), C9 (1), C16 (2) = 3 x 7 hexes = 21 | next card 1, pool 12, compute ~30, adversary cubes 2 (H11, C16), project off + Lab Seizure 2 = ~47 | 0 | **H11 and C16 corrupt.** Counting agents on seven hexes for each neutral factory is the costliest single count of the game (21 hex reads). |
| C Cards | §7 | Science keeps x 4 = 4 | — | 2 | build 8, passes 6, keeps 7, discards 1 = 22 | 23 | |
| D Protector (part) | §16 (Convert), §12 | 0 | OSU target set | OSU: agents on or next to H6 (H6, H7) 7 hexes | agents converted 2, ambassadors placed 2, Unity 1, cards drawn 4, troop removed 1 = 10 | 4: OSU target (6 / 6), Unity split (2 / 2), B&C target (2 / 2), Outpost hex (9 / 9) | |
| D Builder (part) | §16 (Convert), §12 (tips III), §14 (purge table: 7 tips) | CPO +10 x ~6; discounts x ~8; Unity cards' "at most 4" counts x 2; DSL insight +3 = ~18 | 9 cards' once-a-round and this-round effects (Hardpoint Lockdown shelter + cordon, Predictive Modeling's 2 delays and which threats) = 8 | MD: agents on or next to H14 (7 hexes); Volunteer Corps: owned facilities + hosted cities 6; Rebuilding Fund: corrupted cities 1; Threat Forecast Engine 3 node hexes = 17 | agents converted 2, ambassadors 2 + 2 + 3, Unity 3 moves, tip counters 2, Unity cards 2, hardening 3, cards played 15, drawn ~10, stacked 4, agents removed 1-3, transactions ~30 = ~80 | ~25: 15 plays (of ~20 / ~105), 4 build-over (5 / 5), targets 4 (2-14 / same), Unity split x 3 (3 / 3), Dispatch + which ambassador (13 / 13, 3 / 3) | Unity tips 6 (Volunteer Corps III, +4) and 7 (Rebuilding Fund III): **the purge now asks only for no corrupted city**, and C7 is the last. |
| D Protector: Liberate C7 | §8, §14 | Liberate 5 = 1 | — | agents on C7 (0, moved off in Corrupt) 1; purge check: corrupted cities on the map (H7, H6, H14, C7, C10, C15) 6 = 7 | ambassadors 5 spent, adversary cube off 1, Liberated marker 1, Unity 1, tip counter 1, Unity card 1 = 10 | 2: Liberate hex (7 / 7), Unity split (3 / 3) | Unity tip 8 (Town Hall Accord). **The map holds no corrupted city: the team wins** (purge, round 8). Production, threats and war are never reached. |
| **total** | **13** | **~33** | **peak 13** (B 8 + P 4 + Forced March) | **~120** | **~208** | **~58** | |

**Check.** Final map matches: H11 and C16 corrupted, C7 liberated, the Protector's node on H8 beside two troops,
C15's two agents, three new hardenings on H1, H5, H13, Unity 2 with 8 tips, Discord 2 with 1 tip, research 3.
**Error 10:** I kept the agent on H11 after Asset Seizure corrupted its factory and had Mass Defection convert it
(2 ambassadors); the engine has none there. When a site corrupts its agents go back to the supply (§6, for Corrupt
lines); I didn't carry that to a project that "is corrupted" by name. Mass Defection converted only H14's.

---

## Totals

| round | lookups | modifiers | memory peak | bookkeeping | components | decisions | errors | minutes |
|---|---|---|---|---|---|---|---|---|
| 0 setup | 5 | 7 | 0 | 11 | 85 | 2 | 0 | 9 |
| 1 | 17 | 12 | 12 | 81 | 127 | 40 | 1 | 40 |
| 2 | 13 | 28 | 12 | 91 | 170 | 44 | 1 | 42 |
| 3 | 13 | 35 | 13 | 133 | 206 | 52 | 2 | 52 |
| 4 | 14 | 38 | 16 | 150 | 246 | 51 | 0 | 59 |
| 5 | 14 | 59 | 18 | 227 | 367 | 68 | 3 | 85 |
| 6 | 15 | 50 | 22 | 165 | 265 | 68 | 1 | 69 |
| 7 | 16 | 61 | 21 | 170 | 320 | 80 | 1 | 77 |
| 8 | 13 | 33 | 13 | 120 | 208 | 58 | 1 | 56 |
| **game** | **120** | **323** | **22** | **1148** | **1994** | **463** | **10** | **~490** |

Minutes use the rates above with **20 s per new card face read** (about 23 faces in round 1, 30 in round 7); 10 s
for every bookkeeping step overstates the trivial ones (most natural-corruption and node checks are "empty, no"):
at 3 s for those, the game falls to about 6 hours. Either way it is far over a board game's evening, and the
Builder's action phase (14-16 cards and 5-6 powers a round from round 5) is a third of it.

Where the counts come from (game totals, rough): natural corruption + node checks 13-21 a round = ~280 of the 1148
bookkeeping steps; city growth and building tests ~170; Rally deals ~60; threat cards turned 45 (10 in round 3
alone); compute cubes ~240 of the 1994 components; cards (dealt, drawn, played, stacked) ~700 components.

