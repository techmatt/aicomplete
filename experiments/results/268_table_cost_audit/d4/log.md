# Hand-play log: d4 spot check (seed 268010, 2 players, Builder in Horizon, Protector in Cintamani)

Scope: setup (round 0), then rounds 1, 5 and 10 resolved by hand from `docs/rulebook.md` and `docs/player-aid.md`,
team decisions taken from `d4/script.md`. Rounds 5 and 10 start from check.md's "End of round 4" / "End of round 9"
board. Each round's check section was read only after its log section was written.

## Conventions and rates

- **Components:** every piece placed, removed or moved counts 1 (an agent, an ambassador, a node, a cube, a token, a
  card). A **resource change** (a player's credits, energy, chips, insight, cubes, as one pile of tokens) counts 1 per
  change, whatever the amount. A **track marker** (Discord, Unity, compute income, research) counts 1 per move however
  many spaces. **Compute counts per cube**: the adversary's compute goes onto project cards, and the brief asks for
  compute volume, so 38 compute = 38 cubes moved (if the table used a dial per project it would be 1 move; both are
  given where it matters).
- **Lookups:** I read the rulebook and player aid once before setup; each later reopening of a section to resolve
  something counts 1.
- **Modifiers:** each arithmetic adjustment of a printed number (x countries, level change of a corrupt number, War
  Footing's +4, an income sum of several printed lines counts 1 per added line).
- **Decisions:** each choice the table makes; the big ones list human-visible options / engine options.
- **Rates for minutes:** 3 s a component, 10 s a bookkeeping step, 20 s a lookup, 15 s a decision, 5 s a modifier;
  memory items cost no time directly (they cost errors). Practised players: lookups are what a practised table would
  still reopen at this level (the d4 rules are new to most tables).

## Power numbering in the script (decoded once, from the boards)

The script names powers by index. Read off the printed boards, left to right: Builder 0 Mint, 1 Expand, 2 Buy In,
3 Annex, 4 Scrap, 5 Overcharge, 6 Rush Build, 7 Procure, 8 Fortify, 9 Teardown; Protector 0 Bulwark, 1 Expand,
2 Accord, 3 Outpost, 4 Debrief, 5 Patrol, 6 Interdict, 7 Reclaim. (A cost of its own: the rulebook's §20 table lists
the powers in a different order from the board.)

---

## Round 0: setup (level 4, War Economy)

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions | notes |
|---|---|---|---|---|---|---|---|
| Choose players, level | §21 table | - | - | - | - | 2 (players 1-4, level 0-4) | |
| 1 Lay out map | §3, §22 | - | - | - | 2 tiles, 2 bases | - | 2-player layout: one front, bases H15, C13 |
| 2 Each country: marked corrupted city | map.json marks | - | - | 2 | 2 adv cubes (H7, C10) | - | |
| 2 Marked corrupted station + botnet | - | - | - | 2 | 2 stations, 2 cubes, 2 botnets (H4, C12) | - | |
| 2 Other stations, factories neutral | - | - | - | 8 | 3 stations (H10, C2, C11), 5 factories (H2, H8, H13, C3, C14) | - | Cintamani prints two neutral stations (C2 next to the start hex, C11 further off): the tile marks, not §3, say how many |
| 2 Megaproject cards | - | - | - | - | 2 cards | - | |
| 3 Agent supply | §3 | 24 x 2 = 48 | - | - | 48 agents to supply | - | |
| 3 Event deck 3 / 4 / 4 | §21 (level row) | 3 (each stage's count read off the d4 row, not §3's 5/5/3) | - | - | 11 dealt from 17, 3 shuffles, 1 turned up | - | d4 differs from d2 here: fewer stage I, more stage II/III |
| 3 Pools | §3, §21 | 1 (Garrisons 1 + d4 marker 1 = 2 a country; §3 says "1 a country at levels 0-3" and leaves level 4 to §21) | - | 2 | 4 agents | - | |
| 3 Markers: compute income 18, agent income +1, research threshold 4 (x2 countries) | §21 | 3 (18 x 2 = 36, 4 x 2 = 8, +1 x 2) | - | - | 3 markers | - | the d2 setup sets compute 5 (10); d4's 36 is a 7-step-longer track or a dial |
| 3 Projects | §17 | - | - | - | 3 project cards dealt (Station Coup, Accelerated Timetable, Data Heist) | - | Station Coup costs 24 x 2 = 48 |
| 4 Shuffle decks | - | - | - | - | 6 shuffles (Unity, Discord, threat I, threat II, player I, II) | - | |
| 5 Boards, first nodes, starting stock | §3, §5 | - | - | - | 2 boards, 2 nodes (H9, C6), 2 ambassadors, energy 3 + 3, credits 40 + 46 (4 resource piles) | 1 (which two boards) | |
| **Totals** | **6** | **8** | **0** | **14** | **~100** (48 agents, 4 pool agents, 17 event cards handled, 6 shuffles, ~25 pieces, 4 resource piles) | **3** | |

Setup time at the stated rates: ~100 components x 3 s + 14 x 10 s + 6 x 20 s + 8 x 5 s + 3 x 15 s = 645 s, about
**11 minutes**, plus the shuffles (6, ~1 minute). What d4 adds over d2 at setup: 1 more agent per pool (2 agents),
a different deck mix (3 numbers read off a table instead of the §3 default), and the compute marker at 18 a country
(36): three modifiers and one lookup more than d2. Small.

---

## Round 1 (Probing Strikes; next: Factory Drive)

Starting stock: Builder 40 cr, 3 en, 1 amb, node H9, insight 3, 1 Dispatch. Protector 46 cr, 3 en, 1 amb, node C6,
insight 3, 1 Dispatch. Projects: Station Coup 0/48, Accelerated Timetable 0/40, Data Heist 0/24.

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | - | - | 0 | 2 (insight, Dispatches each) | 2 (insight to 3 each) | - | |
| A Sway | - | - | 0 | 1 (Fallout count: 0) | 0 | - | |
| B Card: threats | §18 (deal), §10 (tokens) | - | 0 | 12 (each threat card: tag fits? location exists?) + 3 (factory with most agents: H2, H8, H13 all 0, lowest H2) | 12 threat cards turned (10 discarded), 3 location tokens (Commandeer H15 base + H2 factory; Orbital Strike C13 base) | - | 10 misses before 2 fits: Security 1 is a narrow filter. Each miss is a card read for its tag and location |
| B Card: Rally coastal cities | §6 (deal order) | - | 0 | 6 (is each city coastal? H14 and C10 need a neighbor scan for ocean) + 4 (deals) | 4 agents (H6, H7, C7, C15) | - | Pool 2 a country at d4 (1 at d2) |
| B Corrupt cities | §15 | 4 (each target's printed 5 read as 3, Sleeper Cells) | 0 | 4 (agents: next to how many targets?) + 4 (targets at 3?) | 1 (H7's agent onto H6, its only neighboring target) | - | H6 now 2 agents: safe at 3. C7's and C15's agents already on targets: stay |
| B Natural corruption | - | (same 3) | 0 | 3 (hexes with agents) + 2 (nodes with 2 agents) | 0 | - | |
| B March | - | - | 0 | 1 (no troops) | 0 | - | |
| B Income | §17, §6 | 2 (18 x 2 countries = 36; + 2 botnets) | 0 | 2 (botnets: ambassador on?) + 1 (research track vs 8) | 1 card turned (Factory Drive), 4 agents to pools, **38 compute cubes onto Station Coup (38/48)** | - | Compute volume: d2's same beat would move 10 + 2 = 12 |
| C Cards: opening deal | §7 | - | 0 | - | 8 cards | - | Long tier-2 faces: Embassy of Exiles, Lure Network, Dyson Swarm Tranche |
| C Build a hand | - | - | 0 | - | 8 cards | 8 tier choices (2 / 4, 2) | Builder all tier 1, Protector all tier 2 |
| C Draft | - | - | 0 | - | 8 cards taken, ~6 passes | 8 picks (4, 3, 2, 1 each: 6 non-trivial / engine 8, 4, 6, 3, 4, 2, 2, 1) | |
| C Keep | §7 (keep prices) | - | 0 | 2 (insight left each) | 5 discards, 2 insight changes | 2 keep sets (Builder 4 cards -> 16 subsets, affordable within 3 insight ~8 / engine 10, 7, 6; Protector 1 tier-2 affordable, 5 / engine 5, 1) | Builder keeps Joint Fabrication Pact and Simulation Cluster (2 insight, 1 left); Protector keeps Botnet Takedown (3, 0 left) |
| D Builder | §20 (Buy In, Annex), §8 (Buy energy host rule), §16 (convert) | 2 (Buy In 2 en + space 0; Annex track space 3 free) | Builder: Expand, Buy In, Annex, Mint, Procure used; Embassy Action used; Dispatch spent; H14 bought this round = **8** | 1 (Buy In: new node brings H8 factory into reach? yes, +1 chip) + 2 (H6 and H14: hosted or could be hosted?) | 3 nodes (H10, H7, H6), Embassy to slot, 3 amb gained, 2 agents off H6 + 2 amb on, 1 amb H6 -> H4, 2 cards drawn, 9 resource changes = **23** | 10 actions (each "what next": ~12 affordable / engine 22-30) + 7 chooses (track 2/2, hex 4/4, 6/6, 11/11, Embassy target 2/2, Dispatch hex 9/9, which ambassador 2/2) | Embassy of Exiles 30 cr leaves 10. Buy energy on H14 hinges on the hosting rule: the Builder's nodes are next to both H6 and H14; H14 is buyable only if the Builder's one host slot is "spent" on H6, decided before production. I took it that way (the engine let it) |
| D Protector | §20 (Outpost) | 1 (Outpost 3 en + space 0) | Protector: Outpost, Expand used = **2** (table peak 10 with the Builder's 8) | - | 2 nodes (C10, C13), 3 amb gained, 3 resource changes = **8** | 4 actions (~8 / engine 24-29) + 4 chooses (track 2/2, hex 6/6, track 2/2, hex 8/8) | Node on C10, a corrupted city (allowed), and on C13, the base. Two Gain an ambassador at 20 cr each: 46 -> 6 |
| E1 Factories | §9 | - | H14 bought (carried into hosting) | 5 | 0 | - | all neutral |
| E2 Cities | §9 (host rule) | - | same | 6 + 1 (host matching: H6 or H14 for the Builder) | 1 en (H6 to Builder), Discord marker +2 (H7, C10) | 1 (which city the Builder hosts: 2 / engine none, automatic) | H14 bought earlier "may still be hosted" but the Builder's single slot is on H6. C7, C15 blocked by agents |
| E3 Stations | - | - | - | 5 (H4 blocked by Builder amb; H10, C2, C11 neutral; C12 adv) | 4 cubes (1 neutral to Tidal Grid; 2 neutral + 1 adv to Polar) | - | |
| E4 Megaprojects | - | - | - | 2 | 0 | - | Tidal 1/8, Polar 3/8 |
| E5 Income | - | 3 (Builder 15 + 3 + 2 cr; Protector 13 + 2 cr, 2 + 1 en) | - | 2 | 4 resource changes | - | Builder 2 -> 22 cr, en 2 -> 4; Protector 6 -> 21 cr, en 0 -> 3 |
| E6 Extra drones | - | - | - | 2 bases | 0 | - | no troops |
| E7 Cities grow | §9 | - | - | 6 cities x up to 6 neighbors (H14 counts C14 across the border) | 7 city chips (H6 1, H7 1, H14 2, C7 1, C10 1, C15 1) | - | |
| E8 Cities build | - | - | - | 6 | 0 | - | none at 3 |
| F Threats | §10 (nearest), §5 (Wall), §21 (War Footing) | 2 (War Footing +4, twice) | - | 4 (2 players x Commandeer's 2 tokens: nearest node) | 3 cards discarded, 2 + 4 + 4 = 10 compute to the pending pile, 2 threat cards + 3 tokens off | 2 (pay 3 en or discard 3: 2/2; which 3 of 7: 35 / engine 7, 6, 5) | Commandeer hits the Builder (H6 next to H2). Orbital Strike's troop would land on C13, the Protector's node: Wall keeps it in the supply. Both uncountered: War Footing gives 8 |
| G War | - | - | - | 2 | 0 | - | bases empty |
| H Cleanup | - | - | cleared | - | - | - | |

**Round 1 totals:** lookups 14; modifiers 17; memory peak 10 (Builder 5 powers + Embassy Action + Dispatch + H14
bought; Protector 2 powers), memory total 11 (+ H14's purchase carried into production); bookkeeping 101; components
~205 (of which 38 compute cubes onto Station Coup and 10 to the pending pile, 39 cards); decisions 49; errors: see
the check below.

**Compute volume, round 1:** 38 cubes onto projects + 10 to the pending pile = 48 cubes handled. A d2 round 1 on the
same board: 12 + 2 (War Footing only) = 14.

**Concurrent load, round 1:** peak 10 at the end of the action phase. At d2 the same round carries the same 10:
nothing in d4's round 1 adds a memory item (the extra agents, compute and War Footing compute are all on the table).

### Check, round 1

Every hex in check.md's "End of round 1" matches mine (nodes, ambassadors, the 4 agents, the 7 city chips, Discord 2,
pools 2 / 2). check.md's `adversary.compute=48` is the running total made (38 + the 10 waiting), which also matches.
**Errors: 0.** Not checkable from check.md: players' resource piles and the project cards' compute.

---

## Round 5 (Border War; next: Forced March)

**Start state** (check.md "End of round 4", plus what the table shows that check.md doesn't, rebuilt from the script):
no agents on the map; pools 3 / 3. Troops H2 1, H15 1, C3 1, C7 2, C14 2. Fallout H16, C1, C9. Corrupted: stations H4
(botnet, Builder amb), C2, C12 (botnet, Protector amb), factory C14. Liberated (neutral, marker): cities H7, C10,
station H10, factories H8, C3. Builder nodes H1, H2, H4, H6, H7, H9, H10 (economy spaces 1-6, card-play 1-2: insight 7,
keep upgrade, 2 Dispatches). Protector nodes C6, C10, C12, C13, H8, H11, H14 (economy 1-6, card-play 1-2: insight 7,
keep upgrade, 2 Dispatches). Discord 1 (2 tips), Unity 5 (3 tips), research track 10 (threshold 8). Projects, left to
right: Dirty Bomb 13/24, Mob Rule 0/28, Slander Campaign 0/18; pending compute pile 33. Polar Compute Array complete
(rebuilt: 10 cubes by round 4's production), Tidal Grid 5/8. **Carried from round 4 with no marker:** Open Market
(Unity, strength II) made "the next card any player keeps" free for 2 keeps (2 hosted cities, no owned facilities); one
Builder building is face down (round 4 threat) until the end of this round.

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | §5 (insight: rightmost space that shows one) | - | Open Market 2 free keeps; face-down building until end of round = **2** | 4 (insight, Dispatches) | 2 insight markers (7, 7) | - | Card-play space 2 shows no insight: the total stays 7 |
| A Sway | - | - | 2 | 3 (Fallout tokens) | Discord marker 1 -> 4 | - | |
| B Card: threats (Security 2, tier 2) | §18 | - | 2 | 6 (each card: tag? location?) + 2 (token sites by agents: all 0, lowest) | 6 threat cards (4 discarded), 3 tokens (Full Spectrum Assault on H9; Factory Occupation on C3) | - | |
| B Rally factories | - | - | 2 | 4 Horizon factories + 2 Cintamani (deal order) | 6 agents (H2, H8, H11; C3 x2, C14) | - | pools 3 each: d2 would deal 2 a country |
| B Other lines: Border War nodes, income +1 | §2 (fronts, bases) | - | 2 | 14 (each player's nodes: on or next to H15 or C13?) | 1 node off (C10), agent income marker +1 (4 a country) | 1 (which Protector node: 4 / 4) | Builder has none near a base. Protector loses C10 (hosted city's node) |
| B Corrupt factories | §15, §8 (Liberated) | 5 (factory 2; H8 and C3 Liberated: 3) | 2 | 6 agents (next to how many targets?) + 5 targets | 0 | - | C3 holds 2 = the printed number, but its Liberated marker makes it 3: safe. Easy to miss: the marker is the only thing between C3 and corruption |
| B Natural corruption | - | (same) | 2 | 5 sites with agents + 2 node hexes with agents | 0 | - | |
| B March | §6 (march), §5 (Wall, Vigil) | - | 2 + **Vigil count** = 3 | 6 troops: each needs targets on/next to, then a path "around Wall hexes" | 1 troop off (H2's strikes H6's node), 1 node off | 1 (which target: 3 / 3) | The other 5 troops find no path: the Protector's nodes on H8, H11, H14 and C13 wall off every route from Cintamani and H15 into west Horizon. Working that out is a shortest-path search on the map (~5 min at a table). Vigil: troops that stay next to C6, C13, H11, H14 pay the Protector 2 insight; whether a troop that doesn't move "ends its march" is not said |
| B Income: reveal, pools | - | - | 3 | - | 1 card (Forced March), 8 agents to pools | - | pools 4 / 4 (d2's at this round: ~2-3) |
| B Income: compute | §17, §6 | 3 (18 + 1 + 1 = 20 a country x 2; botnets 0, both under ambassadors) | 3 | 2 botnets | **73 cubes** (40 + 33 pending): 11 complete Dirty Bomb, 28 complete Mob Rule, 18 complete Slander Campaign, 16 onto Lab Purge; 70 cubes back off resolved projects; 3 new project cards dealt | - | **Three projects resolve in one compute step.** Dirty Bomb finds no corrupted city (does nothing); Mob Rule rallies 3 a country onto cities (6 agents) and sways 5 Discord; Slander removes 4 Unity (5 -> 1) |
| B Income: Discord tip | §12, §19, §11 | 1 (strength II: tip 3) | 3 | 6 factories (neutral, no agent, no amb?) | Discord 9 -> 1, tip marker, 1 drone (H15: H13 qualifies), 1 Discord card | - | Two Fronts II: 4 corrupted sites, at most 2: **4 extra tier-2 threats**, dealt now (before the action phase: they execute this round) |
| B Income: Two Fronts' threats | §18 | - | 3 | 4 threat cards + 6 token sites | 4 threat cards, 5 tokens (Data Breach H9, H2; Turncoats C7; Garrison State H6, H15; Mass Production C3) | - | 6 threats now on the map in 2 countries |
| B Income: research | §6, §21 (Arms Race) | 2 (threshold 4 x 2; Muster 3 not 2) | 3 | 4 corrupted stations (complete megaproject? ambassador?) | 1 cube (C2) -> 11, 8 off (3 left), 6 troops (H2 x3, C3 x3) | - | Weapons Program fires. Arms Race's 3 is the d4/d3 modifier; d2 musters 2 |
| C Cards | §7 (keep), Unity card text (Open Market) | 2 (keep upgrade 3 -> 2; Open Market: 2 keeps free) | 3 -> 2 (Open Market used up) | 2 | 8 cards built into hands, 8 drafted, 1 discard, 2 insight changes | 8 tier (2) + 8 draft (4,3,2,1) + 2 keep sets (Builder ~8 / 10, 9, 8, 7; Protector ~6 / 5, 4, 3, 2) | Builder keeps Open Source Uprising, Air Gap (free), National Grid Buildout (1): 6 left. Protector 7 + 2 Vigil = 9, keeps 4 for 6: 3 left. The free keeps are "any player's": the table must agree whose |
| D Builder | §16 (convert), §8 (Buy energy host rule), §10 (pay toward a threat), §20 (Fortify) | 4 (Expand: space 3 pays 1 en; Buy In 2 + space 4's 2; threat paid 4 insight) | Builder: Expand, Buy In, Mint, Fortify used; 2 Dispatches spent; city bought; + carried = **8** | 4 (Open Source Uprising: agents on/next to H7) + 2 (H6 / H7 hostable?) + 3 (Garrison State: near? via H2, H7, H9, H10) | Open Source Uprising: 4 agents off, 4 amb on; Liaison Corps: 3 amb gained, 4 moved (one cancels H2's agent: 2 off); 2 nodes (H13, H6); 2 Dispatch moves; 1 troop off (Fortify); 4 cards drawn; Unity marker; ~10 resource changes = **~35** | 13 actions (~15 affordable / engine 22-64) + 12 chooses (OSU target 3/3; Liaison target 11/11 and 4 moves 3,3,3,2; track 2/2; hexes 3/3, 2/2; Dispatch 12/12 + which amb 6/6, twice; Unity: track or Discord 2/2) | Liaison Corps "up to 4 of your Ambassadors from your supply or on or next to it onto pieces on or next to it": 4 moves, each a source and a destination. The longest single resolution of the round |
| D Protector | §16 (convert, remove), §20 (Outpost) | 2 (Outpost 3 + space 3's 1; Eviction Worm -4 compute) | Protector: Outpost used; Lure Network used; 2 Dispatches spent = **4** (table peak **12** with the Builder's 8) | 3 (Double Agents: agents on/next to H14) + 5 (Lure: agents on/next to C7) | Double Agents: 2 agents off, 2 amb on, 2 troops off; Treaty Isotope Bank to slot; Lure: 3 agents off; Eviction Worm: 1 agent off, 4 compute cubes off Lab Purge; node C14; 2 Dispatches (1 cancels C10's agent: 2 off); ~8 resource changes = **~32** | 8 actions (~12 / engine 28-63) + 10 chooses (DA target 4/4, troop hex 2/2; slot 2/2; Lure target 4/4, pieces 5/5, 3/3; EW target 2/2; node hex 12/12; Dispatch 18/18 + which 4/4, 18/18 + 5/5) | |
| E1-E2 Factories, cities | §9 (host) | - | bought city (into hosting) | 6 factories + 6 cities + host matching (Builder: H6 or H7; Protector: one of H14, C7, C10, C15) | 2 energy (one hosted city each), 0 Discord | 2 (which city each hosts: 2 / -, 4 / -) | Four neutral unblocked cities make nothing: one host each. C10's node is gone but C6 and C13 still host it |
| E3-E4 Stations, megaprojects | §9 | - | - | 7 stations (owner? blocked? project complete?) + 2 projects | 2 neutral cubes to Tidal (7/8); 2 Cintamani neutral cubes made and lost | - | A neutral station whose megaproject is complete still has to be checked to make nothing |
| E5 Income | - | ~10 (Builder: board + 6 track lines + programs; Protector: board + 6 track lines + programs) | - | 2 | 4 resource changes | - | Each income is now a sum of 7+ printed lines; nobody writes it down, so it is recomputed every round |
| E6 Extra drones | §19 | - | - | 2 bases (troop on or next to?) | 2 drones (H15 -> 2, C13 -> 1) | - | the troop left on C14 is next to both bases |
| E7 Cities grow | - | - | - | 6 cities x 6 neighbors | 1 chip (C7 2 -> 3) | - | H6, H7, H14 already hold 3 |
| E8 Cities build | §9 (build) | - | - | 4 cities at 3: count factories vs stations next to each, then free zones next to it, then within 2 in its country | 0 | - | **None of the 4 can build** (no free zone), so this repeats every round for nothing: ~16 neighbor checks a round |
| F Threats | §10, §18 | 5 (War Footing +4 x 5) | - | 5 threats: nearest node to each token | Full Spectrum Assault: botnet on H9, 1 tier-1 threat dealt (waits: Chip Shortage, token H2); Data Breach: Builder discards 3, 2 research cubes (-> 5); Factory Occupation: C3 corrupted (marker off, cube on); Turncoats: Protector pays or discards; Mass Production: adversary factory on C16 (factory + cube); **33 compute to pending**; 5 threat cards, 7 tokens off = **~60** | 3 (Builder: which 3 of 9: 9,8,7 / 9,8,7; Protector: pay 4 amb or discard 3) | Mass Production "on the free industrial zone nearest here": C16 is 4 hexes from C3, the only free zone left on the map |
| G War | §11 | - | - | 2 bases, 3 rival factories | 3 drones off (2 cancel 1; the third finds no target) | - | Cintamani's factories are all the adversary's: Horizon's spare drone strikes nothing |
| H Cleanup | - | - | face-down building turns up | - | 1 card | - | |

**Round 5 totals:** lookups 22; modifiers 34; memory peak 12 (action phase: Builder's 4 powers, 2 Dispatches, bought
city; Protector's Outpost, Lure Network Action, 2 Dispatches; Vigil count; the face-down building's end round; Open
Market's free keeps had just been spent), memory total 14; bookkeeping ~200; components ~420 counting compute per
cube (73 onto projects, 70 off resolved ones, 4 removed, 33 to pending = 180 cube moves; ~240 without them);
decisions 64.

**Compute volume, round 5:** 73 cubes onto projects, three projects resolved in one compute step, 33 more made by
threats for next round. A d2 round 5 (compute income 5 + 1 a country, ~10 from threats): ~22 cubes, about one project.

**Concurrent load, round 5:** peak 12. Against d2's same round: the same player-side items (powers, Dispatches,
bought city, Vigil), so ~10; d4 adds the Vigil count's weight (more troops, so Vigil always pays out) and nothing else
held in the head. What d4 adds is **on the table and in the procedure**: 3 projects resolving in a row, a tip and 4
extra threats, all inside one Income beat, before anyone has acted.

### Check, round 5

Matches: every node; the 6 troops' fates (H2 2 after Fortify, C3 4, C7 2, C14 1 under the Protector's new node, H15
none); C3 and C16 corrupted; the botnet on H9; H6's 2 Builder ambassadors, H7, H9, H14's and C14's ambassadors; C7's
Protector ambassador; pools 4 / 4; research 5; Discord 1 after 3 tips; Unity 2; compute made 73 with 33 waiting. So the
three-project compute step, the tip, Two Fronts and the Weapons Program (with Arms Race's 3) all came out right.

**Error 1 (Liaison Corps sourcing).** I put 2 Builder ambassadors on H1 and 1 on H2; the engine has 1 and 1. The
Builder's supply was empty before the card, so its 3 gained ambassadors covered moves 1-3 and the 4th had to come from
H1 (on or next to the target), not the supply. I wasn't tracking the supply count because nothing on the table forces
it; Liaison Corps' "from your supply or on or next to it" makes the source a decision per ambassador, and the order of
gain-then-move matters. Corrected to H1 1, H2 1.

---

## Round 10 (Blitzkrieg; the event deck's last card: Accelerated Timetable discarded one in round 2)

**Protocol slip, stated up front:** while reading round 9's compute numbers I grepped check.md's `Tracks` lines and
saw the end-of-round-10 tracks line (compute made 66, waiting 24, pools 10, research 6, Discord 0, Unity 0) before
resolving this round. I did not read round 10's hexes. Below, every track number is derived step by step from the
rules; where my reading disagrees with that line (Discord), I kept my reading and report it at the check as a
mismatch, not a match.

**Start state** (check.md "End of round 9" plus the script): the map is mostly the adversary's. Cintamani: every city
(C7 1 agent, C10 7 agents + 2 troops, C15 4 agents + 3 troops), every station (C2, C6, C11, C12) and both factories
(C3, C16) corrupted; 13 troops on C6, 1 each on C3, C13, C16; Fallout C1, C8, C9; the Protector's last Cintamani node
on C12. Horizon: H7 corrupted (Builder node + amb), H9 corrupted (botnet), H6 neutral (botnet, Liberated), H14 neutral;
Builder nodes only H4 (own station, hardened, 2 troops) and H7; Protector nodes H8, H10 (Builder amb, botnet), H11,
H14, H15; Fallout H5, H16; 1 agent on H1. Discord 4 (6 tips: **one card left in the Discord deck**), Unity 1 (4 tips),
research 5, pools 9 / 9. Projects: Backdoor Research 9/28, Night Raid 0/20, Rumor Mill 0/16; pending 4. Both
megaprojects complete. **Carried in with no marker:** Naval Blockade (round 9) "until the end of next round, cities
make no energy"; Sabotage Teams (round 9) "players' boards and cards make no energy income" (§2 gives this one a
round-flag marker); Dark Cities and Shock Doctrine delayed a round by Predictive Modeling (Builder has 1 energy on
Shock Doctrine). Builder card-play spaces 1-6 (insight 9), Protector 1-6 (insight 8). Twin City Compact: the Protector
hosts 2 cities.

| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human / engine) | notes |
|---|---|---|---|---|---|---|---|
| Round start | §5 | - | Naval Blockade, delayed threats (2) = **3** | 4 | 2 insight markers (9, 8) | - | |
| A Sway + Discord tip 7 | §12, §19, Discord card | 1 (strength III: tip 7) | 3 | 5 Fallout + 6 neutral factories (arm a base?) + Scorched Earth: 3 corrupted cities, 2 corrupted factories, each one's neighbors for free open land (~20) | Discord 4 -> 9 -> 1, tip marker, 1 drone (H15: H2 qualifies), 1 Discord card, 1 Fallout (C14, freed when its factory went) | - | **The 7th and last Discord card.** Any later tip loses. Scorched Earth finds only one free open hex next to its 3 named sites: what counts as "open land" (no base, station, factory or Fallout) has to be checked hex by hex |
| B Card: threats (any tier-2; Security 2 or Diplomacy 2) | §18 | - | 3 | 17 cards checked (tag fits? a place of its type not the adversary's?) + token sites | 17 threat cards turned (13 discarded), 4 threats, 5 tokens (Purge Protocols H4; Siege H2; Conscription C13; Manhunt C13) | - | Cintamani has no site that isn't the adversary's, so 12 cards in a row fail the location test. 6 threats now stand: 4 new + 2 delayed |
| B Rally cities | §6, §8 (contact) | - | 3 | 6 cities x 3 deals | 18 agents; 1 cancels the Builder's ambassador on H7 (2 off) | - | 9 a country: d2's pool here is ~4 |
| B Muster 1, income, sway | §6, §5 (Wall) | - | 3 | 2 (lowest factory each) | 2 troops (H2, C3), income marker +1 (10), Discord +1 (2) | - | |
| B Corrupt cities | §15, §8 (Liberated) | 3 (city 3; H6 Liberated 4) | 3 | ~30 agents (next to exactly one target: H6, H14) + 2 targets | 2 agents H7 -> H6; **H6 corrupts** (5 >= 4), **H14 corrupts** (3): 2 cubes, 8 agents off, Protector node off H14, Liberated marker off | - | The Protector loses H14 with no choice: corruption takes the node on the hex |
| B Corrupt land | §15 | 1 (land 3) | 3 | open land targets (H1, C5: "open" excludes bases, stations, factories, Fallout) + agents next to exactly one | 2 agents (C2 -> C5, C6 -> C5) | - | "Corrupt: land" is the hardest line to scan: every agent on the map against every open hex |
| B Natural corruption | - | (same) | 3 | ~10 hexes with agents + 7 node hexes | 0 | - | |
| B March | §6, §5 (Wall) | - | 3 + Vigil count = **4** | **25 troops**, each: target on/next to? else a shortest path around 5 Wall hexes, ties chosen | 22 troop steps (H2 -> H6; C3 x2 -> C6; C6 x13 -> C9; C10 x2 -> C13; C13 -> H13; C15 x3 -> C14; C16 -> C15); H4's 2 strike: hardening token spent, then the station destroyed (Builder cube off) | 2 strike targets (3 / 3) + protection (2 / 2) + 15 ties (2 / 2 each, engine asks each troop separately) | **The heaviest single step of the game.** A human would move the C6 stack of 13 as one (same choice for all), but the rules say each troop chooses: 13 identical decisions. Vigil pays 2 (troops end next to H15 and C12) |
| B Income: reveal, pools | §14 | - | 4 | - | no card to reveal (the deck is empty: round 11 loses), 20 agents to pools | - | |
| B Income: compute | §17 | 2 (20 a country x 2; botnets 2: H6, H9; H10 under the Builder's amb) | 4 | 3 botnets | **46 cubes**: 19 complete Backdoor Research, 20 complete Night Raid, 7 onto Rumor Mill; 48 off resolved projects; 2 project cards dealt | - | Backdoor Research: both megaprojects complete, so only its 4 research cubes act. Night Raid: Horizon's node hexes all hold 0 agents, so the lowest two (H4, H7); Cintamani has one node hex (C12): 3 agents placed |
| B Income: research | §6, §21 (Arms Race) | 2 (threshold 8; Muster 3) | 4 | 6 corrupted stations (complete? amb?) | 4 + 5 cubes (-> 14), 8 off (6 left), 6 troops (H2 x3, C3 x3) | - | Weapons Program fires again |
| C Cards | §7 | 2 (keep prices with keep upgrade; Science discount on tier-2 keeps) | 4 | 2 | 8 + 8 cards, 1 discard, 2 insight changes | 8 tier + 8 draft + 2 keep sets (Builder ~8 / 10, 9, 8, 7; Protector ~6 / 5, 4, 3, 2) | |
| D Builder | §8 (Liberate: no agents), §13 (Shelter), §5 (node needs a payable space), §20 (Fortify), Unity split §12 | ~14: every play's insight after the Science discount, then Interdisciplinary Institute's +1 per tag (SGB 1, Aegis 3, Relay 1, Recruit 1, Field Sensor 2, Nuclear 1); Smart Grid Backbone's +10 cr twice; Security Contractor HQ +10 | Builder: Fortify used, DFC Action used, 2 Dispatches spent, **Shelter on this round**; running insight total = **5** (table peak **11**: + Protector's 2, Vigil, Naval Blockade, delayed threats) | 3 (rescue: facilities next to H7 with no agent) + 3 (Field Sensor Grid: Fallout in reach? none) + 2 (Aegis / Relay: can any node be placed? econ 7 and card-play 7 cost 5 en each: no) | 9 cards played or sold, 8 cards drawn, 2 buildings covered, 1 cube off (H7) + Liberated marker + Builder cube on H8, Unity + Discord markers, 1 troop off, 2 Dispatches (one cancels H4's agent: 2 off), 1 antimatter, ~20 resource changes = **~50** | 13 actions (~20 affordable: 15 cards in hand / engine 42-85) + 7 chooses (rescue target 2/2, claim 3/3, Unity split 5/5, slot 5/5, Dispatch 6/6 + which 2/2, 6/6) | Nuclear Option: Night Raid put an agent on H7 and §8 says a site with an agent can't be liberated; the recorded play rescued H7 anyway (see the check). Aegis Grid Lockdown and Relay Mast Rollout each print "Place 3 Nodes" that can't happen (the Builder's only spaces left cost 5 energy, and energy income is off): a human has to work out that the line is dead. Field Sensor Grid's lift finds no Fallout in reach: its draw alone makes it playable |
| D Protector | §16, §20 (Interdict), §13 (hardening on nodes) | 3 (Open Assembly self-trigger: +1 Follower, at 2 sway 3; Unity split) | Protector: Interdict used, Lure used = 2 | 9 (Lure: agents on or next to C11) + 3 (Singularity: botnets in reach) | Singularity: botnet off, project discarded (7 cubes off, 1 dealt); Open Assembly over Patent Library, 2 Followers on and off; Discord -3; Lure: 3 agents off; Stress Test Suite: 3 hardening tokens; Interdict: 2 troops off, 1 Security card sacrificed; ~8 resource changes = **~30** | 5 actions (~10 / engine 46-83) + 8 chooses (botnet 3/3, project 3/3, slot 4/4, Unity split 4/4, Lure target 4/4, pieces 9/9, harden 5/5, 5/5, 4/4 -> 3 picks; Interdict target 3/3) | Open Assembly triggers itself: its primary tag (Diplomacy) is only in cards.md's section heading, not in its row. A human reading the row sees "other tags: -" and misses the trigger |
| E1 Factories | §9 | - | Shelter; Naval Blockade; Sabotage Teams flag | 6 | 1 chip (H8 to the Builder), 2 troops (C3, C16) | - | |
| E2 Cities | §9, round flags | - | same | 6 cities: blocked? energy off? corrupted? | Discord +3 (C7, C10, C15; H6 blocked by the Builder's ambassador) | - | **Naval Blockade says "cities make no energy"**: a corrupted city makes Discord, not energy, so I made 3 Discord. Ambiguous wording (see the check) |
| E3-E5 Stations, megaprojects, income | - | ~10 (two incomes of 8+ lines each; Sabotage Teams strikes every energy line) | - | 7 stations + 2 | 2 resource changes (credits only) | - | |
| E6 Extra drones | §19 | - | - | 2 bases | 2 drones (H15 -> 2, C13 -> 1) | - | |
| E7-E8 Cities grow, build | §9 | - | - | 7 cities x 6 neighbors; 5 cities at 3 checked to build | **H7 builds a neutral station on H4** (its station destroyed this round: fewer stations than factories next to H7; H4 is free open land, the node doesn't count); 3 chips off, 1 station | - | Found only by a full scan: the only build of the three rounds I played |
| F Threats | §10, §13 (Shelter), §21 | 6 (War Footing +4 x 6) | Shelter | 6 threats x nearest node (Builder vs Protector distances to 7 tokens) | Dark Cities, Purge Protocols: the Builder's nearest node would be destroyed, Shelter saves it; Shock Doctrine: 2 cards, its 2 energy back; Purge Protocols: 3 cards; Siege: H2's factory off, 1 Fallout (H1: the lower of H1 and H2 by the tie rule); Conscription, Manhunt: 2 troops on C13, Protector discards 5; Unity -6 (5 -> 0); **24 compute to pending**; 6 threat cards, 8 tokens off = **~70** | 6 (Shock: pay 2 amb or discard 2/2; 2 + 3 + 2 + 3 card picks / engine 17-6 options each) | Siege's "on or next to here" Fallout: H2 itself or H1, no rule says which; I used the lower-hex tie rule |
| G War | §11 | - | - | 2 bases + 2 rival factories | 3 drones off | - | Horizon's spare drone has no Cintamani factory that isn't the adversary's |
| H Cleanup | - | - | Shelter, Naval Blockade, Sabotage Teams end | - | round flag off | - | |

**Round 10 totals:** lookups 24; modifiers 44; memory peak 11 (Builder: Fortify, Dark Fiber Collider, 2 Dispatches,
Shelter; Protector: Interdict, Lure Network; Vigil count; Naval Blockade; 2 delayed threats' timing), memory total
13; bookkeeping ~250; components ~560 counting compute per cube (46 onto projects, 48 off resolved ones, 7 off the
discarded one, 24 to pending = 125 cube moves) and 22 troop steps; decisions 87 (15 of them the per-troop march
ties).

**Compute volume, round 10:** 46 onto projects, 2 resolved, 24 more made. Smaller than round 5 because the
ambassador on H10's botnet and Singularity Exploit cut it, but the War Footing compute (+4 per uncountered threat,
6 threats) is now a third of the total.

**Concurrent load, round 10:** peak 11. The d2 game for this audit ended in round 8, so its round 8 is the
comparison: the same player-side items (~8: powers, Dispatches, Actions, Vigil) and fewer lasting adversary effects,
so ~8-9. The d4 extras held in the head are the "until the end of next round" stage-III effects (Naval Blockade;
Sabotage Teams has a flag) and the delayed threats, which stage III and Two Fronts / Incursion bring in numbers.

### Check, round 10

Matches: all 25 troops' ends (C9 13, C13 4, C3 4, C6 2, C14 1, C15 1, C16 1, H2 3, H13 1, H6 none after Fortify), the
H4 strike (hardening spent, then the station lost), H6 and H14 corrupted with the Protector's H14 node gone, C5's 2
agents from Corrupt land, every agent stack (C10 10, C15 7, C7 1 after Lure, C11, C12 Night Raid, H1), H4's agent
canceled by the Dispatch, the new neutral station on H4 (H7's build), H8 claimed and hardened, H10 and C12 hardened,
pools 10, research 6, Unity 0, 24 compute waiting.

**Error 2 (Siege's Fallout).** I put it on H1 (the lower hex of H1, H2 by the general tie rule); the engine put it on
H2, the token's own hex, whose factory the same threat had just destroyed. "1 Fallout lands on or next to here" gives no
order; the engine reads "on" first. Corrected to H2.

**Error 3 (Naval Blockade and corrupted cities).** I made 3 Discord from C7, C10 and C15 (corrupted cities make
Discord, and the card only says "cities make no energy"); the engine made none (Discord 0). So in play "make no
energy" stops a corrupted city's Discord too. The wording says the opposite to a reader: a rule I'd have got wrong
every time. Corrected to Discord 0.

**Error 4 (rescuing a city with an agent on it).** By rulebook §8 ("No agents: a site with an agent on it can't be
liberated or claimed") Night Raid's agent on H7 should have blocked The Nuclear Option; the recorded play rescued H7
and the engine's H7 holds no agent. The **player aid** says the opposite: "Liberate: a corrupted site: the agents on
it come off and the adversary's cube comes off". The rulebook and the aid contradict each other; the engine follows
the aid for a card's liberation. I had kept the agent on H7. Corrected to none.

Round 10 errors: 3 (all three are wording problems, not slips of bookkeeping).

---

## Summary: where d4 differs

### Per-round counts (also in counts.csv)

| round | lookups | modifiers | memory peak / total | bookkeeping | components (compute per cube) | of which compute cubes | decisions | errors | minutes |
|---|---|---|---|---|---|---|---|---|---|
| 0 setup | 6 | 8 | 0 / 0 | 14 | ~100 | 0 | 3 | 0 | 12 |
| 1 | 14 | 17 | 10 / 11 | 101 | ~205 | 48 | 49 | 0 | 45 |
| 5 | 22 | 34 | 12 / 14 | ~200 | ~420 | ~180 | 64 | 1 | 80 |
| 10 | 24 | 44 | 11 / 13 | ~250 | ~560 | ~125 | 87 | 3 | 103 |

Minutes are at the stated rates; bookkeeping (10 s each) is half of every round. With compute on a dial instead of
cubes, round 5 drops to ~71 minutes and round 10 to ~97.

### Compute volume (cubes onto projects a round)

| round | income (a country x 2) | botnets | pending (threats, War Footing) | onto projects | projects resolved | d2 estimate |
|---|---|---|---|---|---|---|
| 1 | 36 | 2 | 0 | 38 | 0 | 12 |
| 5 | 40 | 0 | 33 | 73 | **3** (Dirty Bomb, Mob Rule, Slander Campaign) | ~22, about 1 |
| 10 | 40 | 2 | 4 | 46 | 2 (Backdoor Research, Night Raid) | ~25, about 1 |

The engine's running total (check.md `adversary.compute`) reached 600 by round 10: about 60 a round, every round,
against d2's 16-30 (d2's check.md: 170 by its last round, 8). War Footing alone made 8, 20 and 24 of the pending piles in rounds 1, 5 and 10. In cubes, a d4
round moves 50-180 compute cubes (onto projects, back off resolved ones, into the pending pile); that single kind of
component is a third of round 5's components.

### The level's setup and mid-game rules (§21)

- **Setup** cost 3 modifiers and 1 lookup more than d2: compute marker 18 a country (x2), agent income +1 (x2), the deck
  mix 3 / 4 / 4 read off the table (§3's 5 / 5 / 3 is wrong for d4), and the pool of 2 a country (§3 says "1 a country
  at levels 0-3" and leaves level 4 for the reader to work out).
- **Mid-game rules in force at d4: 3** (Sleeper Cells' city 3, War Footing's +4, Arms Race's Muster 3); d2 has 2.
  None is hard alone; each is a modifier on a printed number (the map's city 5, the board's Muster 2) applied every
  time: 4 city checks in round 1, 6 War Footing additions in round 10, 2 Weapons Program firings in my three rounds.
  The printed numbers are the wrong ones at every level above 0.
- **The deck's stage mix** is the real d4 change at the table: stage II from round 4 and stage III from round 7 bring
  Muster lines, two threats a country, round-long effects (Naval Blockade, Sabotage Teams) and late Discord tips at
  strength III. The round count (11, cut to 10 by Accelerated Timetable) is the loss condition that ended this game.

### Troops and the research track

- **Troops on the map:** round 1 none; round 5 6 (+6 mustered in its Income beat); round 10 25 marching (+8 landed).
  The march is per troop with a path "around Wall hexes": in round 5 working out that the Protector's 4 nodes wall off
  every route was a shortest-path search; in round 10, 22 steps and 15 tie decisions, 13 of them identical for one
  stack of 13 on C6.
- **Research track:** threshold 8 (4 x 2); it fired in rounds 5 and 10 (and every few rounds between: 5 or 6 corrupted
  stations each add 1 a round once their megaproject is complete, plus Backdoor Research / Lab Seizure / Station Coup
  2 a country and Data Breach-type threats). Each firing is 6 troops (Arms Race). Checking it costs one bookkeeping step
  per corrupted station (complete megaproject? ambassador on it?) every Income beat: 4 in round 5, 6 in round 10.

### Modifiers a player carries at once

| | Builder | Protector | Both (the level and the table) |
|---|---|---|---|
| round 1 | 0 (keep prices only) | 0 | city 3 (Sleeper Cells), War Footing +4, Muster 3 (Arms Race), x2 countries on compute, threshold and projects |
| round 5 | keep upgrade (tier-2 keeps 2), Open Market's free keeps, Industry discount check | keep upgrade, Vigil, Wall, Fact-Check Network's Diplomacy jam | the 3 level rules, Liberated +1 on 5 sites, x2 |
| round 10 | keep upgrade, Science discount (-1 insight on plays of 2+ and tier-2 keeps), Industry building discount, Interdisciplinary Institute (+1 insight per tag played), Smart Grid Backbone (+10 cr per Power card), Security Contractor HQ (+10 cr per troop removal) = **6** | keep upgrade, Twin City Compact (2 hosts), Open Assembly (Followers, self-trigger), Lights-Out Assembly (+5 cr on 10+ cards), Vigil, Wall = **6** | the 3 level rules, Liberated +1, Naval Blockade (cities make nothing), Sabotage Teams (no energy income), Shelter this round |

By round 10 each player resolves every card play through about 6 standing modifiers of their own plus the table's 3-6:
the Builder's 6 card plays in round 10 each touched 2-4 of them (14 modifiers for the Builder's turn alone).

### Concurrent load against d2

| round | d4 peak (this log) | made of | d2 same round (estimate) | engine's memory variables d4 / d2 (counts.json) |
|---|---|---|---|---|
| 1 | 10 | Builder 5 powers, Embassy Action, Dispatch, bought city; Protector 2 powers | 10 | 9 / 9 |
| 5 | 12 | Builder 4 powers, 2 Dispatches, bought city; Protector Outpost, Lure Action, 2 Dispatches; Vigil count; face-down building's end round | ~11 | 12 / 15 |
| 10 | 11 | Builder Fortify, Dark Fiber Collider, 2 Dispatches, Shelter; Protector Interdict, Lure; Vigil; Naval Blockade; 2 delayed threats | ~9 (d2 round 8) | 12 / 15 |

**The concurrent load is not what d4 raises.** The items held in the head are the players' own once-a-round uses,
which are the same at every level (d2's engine count is even higher in rounds 5-8). What d4 raises is **volume**:
compute (47-81 made a round against d2's 16-30, 2-4x; 406 by round 8 against d2's 170), agents (pools 2 -> 10 a
country against d2's 1 -> 6), troops (25 marching in round 10), threats (6 standing in rounds 5 and 10), and the number of
adversary steps resolved before players act (round 5's Income beat: 3 projects, a tip, 4 threats, a Weapons Program
muster). The engine's tokens moved per round (counts.json) say the same: d4 rounds 8-10 move 98-135 tokens, d2's
busiest round 57.
