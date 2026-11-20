# 297: content, old -> new

Written by `scripts/content/sweep_297.py`. Player cards: [README.md](README.md), *Player cards*.


## Event cards

| Card | Old | New | Why |
|---|---|---|---|
| War Drums | Threat, each country: a tier-1 threat. Infiltrate: cities. Sway 1 toward Discord. | Threat, each country: a tier-1 threat. Infiltrate: cities. | a 1-Sway rider nobody plans around: the card is Infiltrate X alone |
| Signal Jamming | Threat, each country: a tier-1 threat. Infiltrate: botnet hexes. Income: +1 compute a country a round. Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one of its Botnets (a country with none: its lowest-numbered research station). | Threat, each country: a tier-1 threat. Infiltrate: botnet hexes. Income: +1 compute a country a round. | two other lines became one: it keeps Funding; Long Patrols and Station Raids carry Botnets spread |
| Fifth Column | Threat, each country: a tier-2 threat. Infiltrate: cities. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one of its Botnets (a country with none: its lowest-numbered research station). | Threat, each country: a tier-2 threat. Infiltrate: cities. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. | its Strike icon is its one extra; Botnets spread stays on Long Patrols and Station Raids |
| Border War | Threat, each country: a tier-2 threat. Infiltrate: factories. Each player with a Node on or next to a factory destroys 1 of those Nodes. | Threat, each country: a tier-2 threat. Infiltrate: factories. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. | the one node loss an event printed with no troop behind it becomes a third stage-II Strike icon: nodes fall to troops, which the table sees coming |
| Total Mobilization | Threat, each country: 2 tier-2 threats. Infiltrate: factories. Infiltrate: cities. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. Remove 2 Unity from the Unity track. | Threat, each country: 2 tier-2 threats. Infiltrate: factories. Infiltrate: cities. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. | a stage-III card already reads two Infiltrates and a Strike |
| Blitzkrieg | Threat, each country: 2 tier-2 threats. Infiltrate: cities. Infiltrate: research stations. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. Sway 1 toward Discord. Each country: 1 Fallout lands next to its lowest-numbered corrupted city. | Threat, each country: 2 tier-2 threats. Infiltrate: cities. Infiltrate: research stations. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. Each country: 1 Fallout lands next to its lowest-numbered corrupted city. | two other lines became one: it keeps its Fallout |
| Naval Blockade | Threat, each country: 2 tier-2 threats. Infiltrate: coastal cities. Infiltrate: factories. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. Until the end of next round, cities make no energy and no Discord. Each country: 1 Fallout lands next to its lowest-numbered corrupted city. | Threat, each country: 2 tier-2 threats. Infiltrate: coastal cities. Infiltrate: factories. Strike: each Troop destroys 1 player Node, research station or factory on or next to it. Until the end of next round, cities make no energy and no Discord. | two other lines became one: it keeps the Blackout a blockade is |

## Threats

| Card | Old | New | Why |
|---|---|---|---|
| Weapons Test | Orbital Strike: 2 cubes go onto the adversary's research track. | Weapons Test: 2 cubes go onto the adversary's research track. | an orbital strike that only moved cubes; the test range is what the cubes are |
| Supply Raid | 2 cubes go onto the adversary's research track. This country's player pays 2 chips or discards 1 card. | This country's player pays 2 chips or discards 1 card. The adversary gains 4 compute. | a raid takes supplies: its 294 cube line goes, the chips stay and the haul is compute |
| Call-Up | 2 cubes go onto the adversary's research track. 1 Fallout lands next to here. | 1 Agent is created here. 1 Fallout lands next to here. | a call-up raises people, not research: an agent on its coastal city (one, so it never takes the city with the anchor agent at tier 1) beside the Fallout it had |
| Weapons Program | Total War: 4 cubes go onto the adversary's research track. | Weapons Program: 4 cubes go onto the adversary's research track. | the card is the row's biggest feeder; no war is declared by 4 cubes |
| Manhunt | This country's player pays 3 Ambassadors or discards 3 cards. 3 cubes go onto the adversary's research track. | This country's player pays 3 Ambassadors or discards 3 cards. 2 Agents are created here. | a manhunt takes ambassadors and leaves its hunters in the city; its 294 cube line goes |
| Conscription | 3 cubes go onto the adversary's research track. This country's player pays 2 chips or discards 2 cards. | 2 Agents are created here. This country's player pays 2 chips or discards 2 cards. | conscripts fill the factory: agents on it, not cubes on the track |
| Closed City | Garrison State: 3 Agents are created on the city here. 3 cubes go onto the adversary's research track. | Closed City: 3 Agents are created on the city here. 3 cubes go onto the adversary's research track. | garrison is the troops' word now, and this threat lands none: agents hold a city while its labs work |
| Brain Drain | Iron Curtain: 3 cubes go onto the adversary's research track. Remove 2 Unity from the Unity track. | Brain Drain: 3 cubes go onto the adversary's research track. Remove 2 Unity from the Unity track. | an iron curtain that moved cubes and Unity: researchers leaving is both lines |

## Projects

| Card | Old | New | Why |
|---|---|---|---|
| Night Raid | Each country: 1 Agent onto each of two different hexes holding a player node (those holding the most Agents first; ties: lowest-numbered). An Agent landing on your node's hex that brings it to 2 destroys your node. (10 a board) | Each country: 1 Agent onto each of its two lowest-numbered hexes holding a player Node and an Agent. 2 Agents on your Node's hex destroy it. (10 a board) | no "most agents" pick, and no agent onto a clean node: the nodes at risk are the ones an agent already stands on, which the table can see and clear |
| Asset Seizure | Each country: its neutral factory with the most Agents on its hex (ties: lowest-numbered) is corrupted; with none neutral, its player-owned factory picked the same way turns neutral. (13 a board) | Each country: its lowest-numbered neutral factory is corrupted; with none neutral, its lowest-numbered player-owned factory turns neutral. (13 a board) | no "most agents" pick: the lowest number, as every other line on the adversary's side |
| Botnet Conscripts | Each country: its lowest-numbered botnet becomes 2 Troops, which garrison its lowest-numbered corrupted factory (with none, its lowest-numbered corrupted city; with none, its lowest-numbered city). (11 a board) | Each country holding 2 or more Botnets: its lowest-numbered Botnet becomes 1 Troop on its hex. (16 a board) | the troop stands where the botnet stood, beside whatever the players built there, so a Strike has something to hit and a Redeploy something to move; one troop, not two, dearer (11 -> 16), and only where botnets have spread: on the first screen the starting botnet turned by round 3 in every early-troop game |
| Sabotage Teams | Each country: the hardened facility with the most Agents on its hex (ties: lowest-numbered) loses its hardening. Until the end of next round, players' boards and cards make no energy income. (10 a board) | Each country: its lowest-numbered hardened facility loses its hardening. Until the end of next round, players' boards and cards make no energy income. (10 a board) | no "most agents" pick: the lowest number, as every other line on the adversary's side |
| Station Coup | Each country: its neutral research station with the most Agents on its hex (ties: lowest-numbered) is corrupted; with none neutral, its player-owned station picked the same way turns neutral. Then each country: 1 Fallout lands next to its lowest-numbered corrupted research station, and 2 cubes go onto the adversary's research track. (24 a board) | Each country: its lowest-numbered neutral research station is corrupted; with none neutral, its lowest-numbered player-owned station turns neutral. Then each country: 1 Fallout lands next to its lowest-numbered corrupted research station, and 2 cubes go onto the adversary's research track. (24 a board) | no "most agents" pick: the lowest number, as every other line on the adversary's side; it opens the adversary a research station a country, a cube a round for the row |
| War Plants | Each country: the adversary builds a factory on its free industrial zone next to an Agent, else its lowest-numbered (none free: 2 Agents on one of its factories). Every adversary factory makes 1 Troop on its hex (a player Ambassador on it stops it). (19 a board) | Every corrupted factory makes 1 Troop on its hex (a player Ambassador on it stops it). (22 a board) | one thing, by a condition the table sees: it builds no factory of its own, so a board with no corrupted factory takes no troop, and an ambassador on each one stops it; dearer (19 -> 22), so later |
| Martial Law | Each country: create 3 Agents on its lowest-numbered research station not yet the adversary's. (24 a board) | Each country: 2 Troops land on its lowest-numbered corrupted city with a Node or an Ambassador on or next to it (with none, they garrison). (24 a board) | a third troop project, and the dearest (24): martial law is soldiers in the streets where the players are working, so its troops guard the city being liberated and a Strike finds pieces beside them; its old line (3 agents on a research station) was Lab Seizure's |

## Unity cards

| Card | Old | New | Why |
|---|---|---|---|
| Volunteer Corps I | Sway 1 toward Unity for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 2). | Remove 1 Discord from the Discord track for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 2). | removes Discord by what the players hold; it was Unity swaying Unity (the same count, the same worth) |
| Volunteer Corps II | Sway 1 toward Unity for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 3). | Remove 1 Discord from the Discord track for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 3). |  |
| Volunteer Corps III | Sway 1 toward Unity for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 4). | Remove 1 Discord from the Discord track for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 4). |  |
| Safe Corridors I | Every player gains 2 Ambassadors for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 2). | Remove 1 Discord from the Discord track for each hex holding an Ambassador (at most 2). | removes Discord by ambassadors standing on the map; it was a fifth card handing out supply ambassadors |
| Safe Corridors II | Every player gains 2 Ambassadors for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 3). | Remove 1 Discord from the Discord track for each hex holding an Ambassador (at most 3). |  |
| Safe Corridors III | Every player gains 2 Ambassadors for each research station or factory a player owns, or free city with a player's Node on or next to it (at most 4). | Remove 1 Discord from the Discord track for each hex holding an Ambassador (at most 4). |  |
| Reconciliation Panel I | Remove 1 Agent from each hex on or next to a corrupted city. | Remove 1 Discord from the Discord track for each free city with no Agent on it (at most 2). | removes Discord by the cities the players keep clear of agents; it was the third card removing agents by corrupted cities (a first cut counted corrupted cities, which a game still running always has) |
| Reconciliation Panel II | Remove up to 2 Agents from each hex on or next to a corrupted city. | Remove 1 Discord from the Discord track for each free city with no Agent on it (at most 3). |  |
| Reconciliation Panel III | Remove up to 3 Agents from each hex on or next to a corrupted city. | Remove 1 Discord from the Discord track for each free city with no Agent on it (at most 4). |  |

## Discord cards

| Card | Old | New | Why |
|---|---|---|---|
| Offensive I | Each country: create 2 Agents on its lowest-numbered city not yet the adversary's; then deal a tier-2 threat with its token on the Node hex with the most Agents and Troops. | Deal a tier-2 threat with its token on the Node hex with the most Agents and Troops. | one verb: 293's patch line ("create 2 Agents on its lowest-numbered city", Surge's heir) and strength III's second Infiltrate go, and the strengths widen the count |
| Offensive II | Each country: create 2 Agents on its lowest-numbered city not yet the adversary's; then deal two tier-2 threats with their tokens on the Node hex with the most Agents and Troops. | Deal two tier-2 threats with their tokens on the Node hex with the most Agents and Troops. |  |
| Offensive III | Each country: this round's Infiltrate lines land again; then, each country: create 2 Agents on its lowest-numbered city not yet the adversary's; then deal two tier-2 threats with their tokens on the Node hex with the most Agents and Troops. | Deal three tier-2 threats with their tokens on the Node hex with the most Agents and Troops. |  |

## Unity cards

| Card | Old | New | Why |
|---|---|---|---|
| Town Hall Accord I | Each player removes up to 3 Agents from one hex on or next to a corrupted city. | Remove 2 Discord from the Discord track for each city holding an Ambassador (at most 1). | removes Discord by ambassadors in cities; it was the second card removing agents by corrupted cities |
| Town Hall Accord II | Each player removes up to 4 Agents from one hex on or next to a corrupted city. | Remove 2 Discord from the Discord track for each city holding an Ambassador (at most 2). |  |
| Town Hall Accord III | Each player removes up to 5 Agents from one hex on or next to a corrupted city. | Remove 2 Discord from the Discord track for each city holding an Ambassador (at most 3). |  |
| Demilitarized Zone I | In the next adversary turn, no Troop strikes a hex on or next to an Ambassador or a Node. | In the next adversary turn, no Troop strikes a hex on or next to an Ambassador. | strengths II and III were the same line: each strength now widens the one before |
| Demilitarized Zone II | In the next adversary turn, no Troop strikes a hex within 2 of an Ambassador or a Node. | In the next adversary turn, no Troop strikes a hex on or next to an Ambassador or a Node. |  |

## Discord cards

| Card | Old | New | Why |
|---|---|---|---|
| Captured Labs I (was Blight) | Each country: create 1 Agent on the lowest-numbered city holding Agents, and a neutral factory opens on its lowest-numbered free industrial zone. | For each corrupted research station (at most 2), 2 cubes go onto the adversary's research track. | the Discord deck's research feeder, by the stations the players left corrupted; Blight (agents on a city and a neutral factory) was Firestorm with a rider, under the old name of Fallout |
| Captured Labs II (was Blight) | Each country: create 3 Agents on the lowest-numbered city holding Agents, and a neutral factory opens on its lowest-numbered free industrial zone. | For each corrupted research station (at most 3), 2 cubes go onto the adversary's research track. |  |
| Captured Labs III (was Blight) | Each country: create 4 Agents on the lowest-numbered city holding Agents, and a neutral factory opens on its lowest-numbered free industrial zone. | For each corrupted research station (at most 4), 2 cubes go onto the adversary's research track. |  |
| Standing Army I (was Two Fronts) | Deal 1 extra threat for each corrupted city, research station or factory (at most 2). | Each country: 1 Troop lands on its lowest-numbered corrupted factory (a country with none takes no Troop). | the Discord deck's troop card: fronts went with war, and its extra threats were Incursion's; strength I lands a troop only where a factory has fallen, so an early tip rarely brings one; from strength II (the third tip) its troops land on a corrupted city the players stand beside |
| Standing Army II (was Two Fronts) | For each corrupted city, research station or factory (at most 2), deal a tier-2 threat in each country piece. | Each country: 1 Troop lands on its lowest-numbered corrupted city with a Node or an Ambassador on or next to it (with none, it garrisons). |  |
| Standing Army III (was Two Fronts) | For each corrupted city, research station or factory (at most 3), deal a tier-2 threat in each country piece. | Each country: 2 Troops land on its lowest-numbered corrupted city with a Node or an Ambassador on or next to it (with none, they garrison). |  |
| Firestorm III | Each country: create 2 Agents on the lowest-numbered city holding Agents; then 1 Troop garrisons its lowest-numbered corrupted factory (with none, its lowest-numbered corrupted city; with none, its lowest-numbered city). | Each country: create 4 Agents on the lowest-numbered city holding Agents. | one verb again: 294's troop rider on strength III moved to Standing Army |

## Board powers

| Card | Old | New | Why |
|---|---|---|---|
| Outpost (Protector) | 2 energy: place 1 Node; gain 2 credits | 2 energy: place 1 Node within 2 of yours; gain 2 credits | used 0.05 a seat-round beside Accord at the same price (an ambassador beats 2 credits): an outpost is now what its name says, a node two hexes out (a shield, the first idea, is the cards': the content check refuses one on a board) |

## Player cards

| Card | Old | New | Why |
|---|---|---|---|
| Breach and Clear (tier 1 event, 2 cr, 1 insight) | Target: Troops. Remove 1 Troop on it. | Target: Agents or Troops. Remove 3 Agents or 2 Troops on it. | kept 0.21 a game and played 0.03: no card at all with no troop in reach, and under the standard action's price with one; stronger, not cheaper |
| Peacekeeping Mission (tier 2 event, 13 cr, 1 antimatter) | Target: a corrupted city. Pay 1 antimatter. Rescue it. Remove 3 Troops in its country. | Target: a corrupted city. Pay 1 antimatter. Remove 3 Troops on or next to it. Rescue it. | a guarded city was no Rescue target, so its troop half could never open its own target: the troops go first, and it is the one Rescue that takes a guarded city |
