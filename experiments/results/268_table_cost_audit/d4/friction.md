# Friction: d4 spot check (seed 268010; setup, rounds 1, 5, 10)

Fix codes: **W** wording or layout, **A** player aid, **C** component (a marker), **P** needs a rules change.
"How often" counts what I met in the 3 rounds played (R1, R5, R10) unless stated.

## Ranked by how often x how much

| rule (§) | what it cost | how often met | how hard (one line) | the smallest fix |
|---|---|---|---|---|
| Compute onto projects (§6 Income, §17) | components, bookkeeping | 38 / 73 / 46 cubes onto projects; 48 / ~180 / ~125 cube moves counting returns and the pending pile | d4's compute income 18 a country (36 at 2 players) plus War Footing makes compute the biggest single kind of component; R5 resolved 3 projects in one compute step | **C**: a compute track or dial on each project slot (one marker move per project, not one cube per compute); print each project's cost already x2 / x3 / x4 on the card (it is in adversary-cards.md, not on the slot) |
| The Income beat's chain (§6, §12, §17, §18) | lookups, bookkeeping, memory of order | R5: 3 projects, a Discord tip, Two Fronts' 4 threats and a Weapons Program muster before anyone acts; R10: 2 projects, Night Raid, research fire | the order (reveal, pools, compute, slide-and-deal after each, tips after every step, research check) is spread over §6, §12, §17 | **A**: an Income-beat flow card (6 numbered steps, "check tips after each project") |
| The march (§6, §5 Wall) | bookkeeping, decisions | R5 6 troops, R10 25 troops (22 steps, 15 tie choices) | each troop needs a shortest path "around Wall hexes" to the nearest target; in R5 the answer was "no path at all", found only by searching the whole map | **P/W**: "troops on one hex move as one stack; the players choose once" (13 identical choices in R10); **A**: print hex distances on the map aid |
| Once-a-round uses with no marker (§5, §8, §20) | memory | peak 10 / 12 / 11 held at once | powers (10 Builder, 8 Protector), building Actions, Dispatches, Buy energy per city, converters, Vigil count: nothing on the table shows which are spent | **C**: a used-cube per power space and per building (flip or cube), a Dispatch token per Dispatch, a "bought" chip for a city |
| Cities grow and build (§9 steps 7-8) | bookkeeping | ~36 neighbor checks a round for growth; 4-5 cities at 3 checked to build each round, 1 build in 3 rounds | H6, H7, H14 sat at 3 chips from R4 to R10 with no legal hex: the same failed search every round; the build rule (fewer of the two kinds, next to, then within 2 in its country, agent first, then lowest) is 5 nested tests | **P**: a city that can't build drops its chips (or stops growing at 3 with nothing to do); **C**: a "full" marker on a city that can't build until the map changes |
| Threat dealing misses (§18) | components, bookkeeping | 10 misses in R1, 4 in R5, 13 in R10 (Cintamani had no site left that isn't the adversary's) | every miss is a card turned, read for its tag and its location types, and discarded | **P**: when a country has no place a tier's deck can use, skip it; or **A**: deal by tag from a deck sorted by tag |
| Nearest / near for each threat (§10) | bookkeeping | 2 / 5 / 6 threats executing, 7 tokens in R10 | distance from each player's nearest node to each token, at any range | **A**: hex distances on the map aid |
| Hosting as a matching problem (§2, §8 Buy energy, §9 step 2) | bookkeeping, lookup, decision | R1 (H6 vs H14), R5 (H6 vs H7; Protector 1 of 4), R10 (Twin City: 2 hosts) | "a city a player hosts or could host can't be bought" depends on which city each player's one slot covers, decided only in production | **C**: a host token per slot, placed on a city at the start of the action phase; **W**: "a city with a host token can't be bought" |
| Per-player income each round (§9 step 5) | modifiers | 2 players x 7-10 printed lines, every round | board + each uncovered track space + each income program, re-added every round; Sabotage Teams then strikes the energy lines | **C**: income tracks (one marker per resource), moved when a space or program adds income |
| Level modifiers on printed numbers (§21) | modifiers, lookup | 4 city checks R1, 5 R5, 6 War Footing additions R10, 2 Musters of 3 | the map prints city 5 and the board Muster 2; at d1+ / d3+ both are wrong | **C/A**: a d1+ city-3 marker and a d3+ Muster-3 tile on the adversary board; War Footing's +4 printed on the threat-execute strip |
| Running insight with triggers (§7, cards) | modifiers | Builder R10: 6 plays, each through the Science discount and Interdisciplinary Institute's +1 per tag | a pile that changes 12 times in one turn | **C**: an insight track (dial) per player |
| Long or tricky card faces (cards) | lookups, errors | Liaison Corps (R5), Embassy of Exiles, Open Source Uprising, Station Coup project, Two Fronts II, Scorched Earth III, Aegis Grid Lockdown and Relay Mast Rollout (dead "Place 3 Nodes" late) | Liaison Corps' "move up to 4 of your Ambassadors from your supply or on or next to it onto pieces on or next to it" is 4 source-and-destination choices | **W**: split Liaison Corps' line in two ("gain 3; then move up to 4..."); late-game node lines say "if you can" |
| Rulebook length (all) | lookups | 14 / 22 / 24 lookups in R1 / R5 / R10 | 1,256 lines, 14,600 words; the d4 numbers live in §21's generated table, the round in §4, the beat details in §6, §15, §17, §18 | **A**: a two-sided adversary-turn and production aid would have absorbed most of my lookups (11 of round 1's 14 were adversary-turn, production or threat sections) |

## Errors (each is a friction row too)

| rule (§) | what it cost | how often met | how hard (one line) | the smallest fix |
|---|---|---|---|---|
| Liberate with an agent on the site (§8 vs the player aid) | error | once (R10, Nuclear Option on H7 under Night Raid's agent) | §8: "a site with an agent on it can't be liberated"; the aid: "Liberate: the agents on it come off"; the engine followed the aid for a card | **W**: make them agree (for the action and for cards separately, if they differ) |
| Naval Blockade "cities make no energy" (event card) | error | once (R10: 3 Discord I made, the engine none) | a corrupted city makes Discord, not energy, so the text reads as leaving Discord on; the engine stops it | **W**: "cities make nothing (no energy, no Discord)" or the reverse, whichever is meant; and give it a round flag (§2 lists one only for Rolling Blackouts and Sabotage Teams) **C** |
| Fallout "on or next to here" (Siege, §6) | error | once (R10) | no order between the token's hex and its neighbors; the engine uses the hex itself first | **W**: "on its hex, else next to it (lowest number)" |
| Liaison Corps' source (card) | error | once (R5) | the 4th ambassador had to come from the map because the supply was empty; nothing on the table tracks the supply count well | **W** (card) and **C**: ambassadors in supply kept on the board, not in hand |

## Smaller costs met once or twice

| rule (§) | what it cost | how often met | how hard (one line) | the smallest fix |
|---|---|---|---|---|
| Vigil (§5) | memory, ambiguity | R5, R10 | does a troop that doesn't move "end its march"? Unsaid; I paid it | **W**: "each troop, moved or not, ..." |
| Open Market (Unity II) | memory across rounds | R4 -> R5 | "the next N cards any player keeps are free" outlasts the round with nothing on the table | **C**: put N free-keep tokens out when it resolves |
| Delayed threats (Predictive Modeling) | memory | R9 -> R10, 2 threats | a delayed threat looks like any other on the map | **C**: the waits token, as for late threats |
| Corrupt land (§15) | bookkeeping | R10 | every agent on the map against every open hex; "open land" excludes bases, stations, factories and Fallout | **A**: list "open land" in the aid |
| Primary tag hidden (cards.md) | error risk | R10 (Open Assembly triggers itself) | the row's "other tags" omits the primary tag, which only the section heading gives | **W** (doc layout): print all tags in the row |
| §3 pools at level 4 | modifier | setup | §3: "1 a country at levels 0-3"; level 4's 2 must be read off §21 | **W**: "1 a country, plus §21's agent income" |
| Purge table, Unity tip table (§14, §12) | lookup | 2 Discord tip strengths looked up (R5 II, R10 III); the purge was never in reach at d4 | Unity tips 4 by R10 and Unity at 0: the purge table never mattered | none needed at d4 |
| Counting agents on seven hexes (§17) | bookkeeping | 0 in my rounds (no "on or next to" project resolved); Night Raid counted 8 node hexes | the d4 project flow resolves ~1.5 projects a round, so these counts come often across a game (Station Coup R2, Asset Seizure R4) | **A**: a 7-hex template |

## Setup and turn time

- Setup: ~100 components, 14 bookkeeping, 6 lookups, ~12 minutes; d4 adds only 3 modifiers and a deck mix to d2's.
- Rounds at the stated rates: R1 45 min, R5 80 min, R10 103 min (71 and 97 with compute on dials). The adversary's
  turn and production are 60-70% of each round; the players' own action phase is the smaller part.
- A full d4 game at this pace (10 rounds averaging ~75 minutes) is 12+ hours of table time: far past a board game's
  budget, mostly from bookkeeping volume (compute, agents, troops, failed city builds), not from rules held in mind.
