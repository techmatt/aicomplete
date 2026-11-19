# Adversary cards (generated)

*Written by `python scripts/content/load_adversary.py` from `drafts/adversary/` (the source); edit the drafts, not this page. The rules are in [rules/threats.md](rules/threats.md) and [rules/adversary.md](rules/adversary.md).*

## The Militarist (board)

Project slots: Open, Open, Open.

- **War Rooms** (threat decks). Setup: build each threat deck from every Security and Universal threat of its tier and as many of its other threats, taken at random; shuffle it. The rest stay in the box.
- **Botnet Recruiters.** In the compute step, each Botnet with no player Ambassador on it puts 1 Agent on the city, research station or factory next to it not yet the adversary's holding the most Agents (else on its own site, if not yet the adversary's), and adds its compute (even when a cordon stops its Agent or it has nowhere to land).
- **Weapons Program** (research). When the research track reaches its threshold, spend it: Muster 2: each country, 2 Troops on its lowest-numbered factory (else city). Stronger: Muster 3 instead.
- **Level 1: Sleeper Cells.** *Cells wake in every city.* When an Agent lands on a city, it falls at 3 agents, not 5.
- **Level 2: War Footing.** *Every unanswered strike feeds the war machine.* When a threat executes uncountered, the adversary also gains 4 compute.
- **Level 3: Arms Race.** *The labs build weapons, not cures.* When Weapons Program fires, it musters 3, not 2.
- **Level 4: War Economy.** *The whole economy marches.* Setup only.

## Event deck

| Card | Stage | Text |
|---|---|---|
| Probing Strikes | 1 | Threat, each country: a tier-1 threat. Infiltrate: coastal cities. Grow: cities. |
| Factory Drive | 1 | Threat, each country: a tier-1 threat. Infiltrate: coastal cities. Each country: a neutral factory opens on its lowest-numbered free industrial zone. |
| Long Patrols | 1 | Threat, each country: a tier-1 threat. Infiltrate: cities. Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one of its Botnets (a country with none: its lowest-numbered research station). Grow: cities. |
| War Drums | 1 | Threat, each country: a tier-1 threat. Infiltrate: cities. Sway 1 toward Discord. Grow: cities. |
| Signal Jamming | 1 | Threat, each country: a tier-1 threat. Infiltrate: botnet hexes. Income: +1 compute a country a round. Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one of its Botnets (a country with none: its lowest-numbered research station). Grow: research stations. |
| Mobilization | 2 | Threat, each country: a tier-2 threat. Infiltrate: coastal cities. Spill (level 2+): each city not yet the adversary's holding 2 or more Agents moves 1 onto the factory next to it not yet the adversary's holding the most. Grow: cities. |
| Station Raids | 2 | Threat, each country: a tier-2 threat. Infiltrate: research stations. Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one of its Botnets (a country with none: its lowest-numbered research station). Spill (level 2+): each city not yet the adversary's holding 2 or more Agents moves 1 onto the research station next to it not yet the adversary's holding the most. Grow: research stations. |
| Shell Labs | 2 | Threat, each country: a tier-2 threat. Infiltrate: research stations. Each country: a neutral research station opens on its lowest-numbered empty station site. |
| Fifth Column | 2 | Threat, each country: a tier-2 threat. Infiltrate: cities. Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one of its Botnets (a country with none: its lowest-numbered research station). Spill (level 2+): each factory not yet the adversary's holding 2 or more Agents moves 1 onto the city next to it not yet the adversary's holding the most. Grow: cities. |
| Border War | 2 | Threat, each country: a tier-2 threat. Infiltrate: factories. Each player with a Node on or next to a factory destroys 1 of those Nodes. Spill (level 2+): each city not yet the adversary's holding 2 or more Agents moves 1 onto the factory next to it not yet the adversary's holding the most. Grow: factories. |
| Supply Lines | 2 | Threat, each country: a tier-1 and a tier-2 threat. Infiltrate: factories. Income: +1 compute a country a round. Spill (level 2+): each research station not yet the adversary's holding 2 or more Agents moves 1 onto the factory next to it not yet the adversary's holding the most. Grow: factories. |
| Forced March | 2 | Threat, each country: a tier-2 threat. Infiltrate: cities. Every adversary Troop takes 1 more step this round. Spill (level 2+): each research station not yet the adversary's holding 2 or more Agents moves 1 onto the city next to it not yet the adversary's holding the most. Grow: cities. |
| Total Mobilization | 3 | Threat, each country: 2 tier-2 threats. Infiltrate: factories. Infiltrate: cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Remove 2 Unity from the Unity track. Spill (level 2+): each city not yet the adversary's holding 2 or more Agents moves 1 onto the factory next to it not yet the adversary's holding the most. Grow: factories. |
| Blitzkrieg | 3 | Threat, each country: 2 tier-2 threats. Infiltrate: cities. Infiltrate: research stations. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Sway 1 toward Discord. Each country: 1 Fallout lands next to its lowest-numbered corrupted city. Spill (level 2+): each research station not yet the adversary's holding 2 or more Agents moves 1 onto the city next to it not yet the adversary's holding the most. |
| Occupation | 3 | Threat, each country: 2 tier-2 threats. Infiltrate: research stations. Infiltrate: cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Spill (level 2+): each city not yet the adversary's holding 2 or more Agents moves 1 onto the research station next to it not yet the adversary's holding the most. Grow: research stations. |
| Naval Blockade | 3 | Threat, each country: 2 tier-2 threats. Infiltrate: coastal cities. Infiltrate: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Until the end of next round, cities make no energy and no Discord. Each country: 1 Fallout lands next to its lowest-numbered corrupted city. Spill (level 2+): each factory not yet the adversary's holding 2 or more Agents moves 1 onto the city next to it not yet the adversary's holding the most. |

## Threats, tier 1

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Orbital Strike | Security | factory | 1 adversary Troop lands here. | 3 energy; 8 credits |
| Commandeer | Security | factory | This country's player pays 3 energy or discards 3 cards. The adversary gains 2 compute. | 3 Ambassadors; 8 credits |
| Supply Raid | Security + Industry | factory | 1 adversary Troop lands here. This country's player pays 2 chips or discards 1 card. | 2 Industry tags; 8 credits |
| Eviction | Security | city | This country's player pays 2 Ambassadors or discards 2 cards. The adversary gains 4 compute. | 2 Security tags; 8 credits |
| Call-Up | Security | coastal city | 1 adversary Troop lands here. 1 Fallout lands next to here. | 3 Ambassadors; 8 credits |
| Provocation | Diplomacy + Security | city | A tier-1 threat is dealt into this country (it executes next round). Sway 2 toward Discord. | 3 Ambassadors; 8 credits |
| Agitators | Diplomacy | city | 1 Agent is created here. Sway 2 toward Discord. | 2 Diplomacy tags; 8 credits |
| Defection | Diplomacy | city | This country's player pays 2 Ambassadors or discards 2 cards. Sway 1 toward Discord. | 2 Diplomacy tags; 8 credits |
| Smear Campaign | Diplomacy | city, station | Remove 2 Unity from the Unity track. This country's player pays 2 energy or discards 2 cards. | 3 Ambassadors; 8 credits |
| Propaganda Blitz | Diplomacy | coastal city | Sway 3 toward Discord. The adversary gains 3 compute. | 3 Ambassadors; 8 credits |
| Sabotage | Cyber | station | This country's player loses 4 tokens from their cards. The adversary gains 6 compute. | 2 Cyber tags; 8 credits |
| Insider Access | Cyber + Diplomacy | station | 1 Agent is created here. This country's player pays 3 research cubes or discards 2 cards. | 3 Ambassadors; 8 credits |
| Wiper Malware | Cyber | station | This country's player discards 2 cards. 2 cubes go onto the adversary's research track. | 3 research cubes; 8 credits |
| Cryptojacking | Cyber | station, factory | The adversary gains 6 compute. This country's player pays 2 research cubes or discards 1 card. | 2 Cyber tags; 8 credits |
| Backdoor | Cyber | station | A Botnet lands on the station here (if it holds one, on the nearest piece without one). | 3 insight; 8 credits |
| Requisition | Industry | factory, city | This country's player pays 3 chips or discards 2 cards. The adversary gains 3 compute. | 2 Industry tags; 8 credits |
| Shakedown | Industry + Diplomacy | city | This country's player pays 3 chips or discards 2 cards. The adversary gains 4 compute. | 3 Ambassadors; 8 credits |
| Embargo | Industry | factory | 1 Agent is created here. This country's player pays 2 energy or discards 2 cards. | 3 chips; 8 credits |
| Credit Crunch | Industry | coastal city | This country's player discards 2 cards. The adversary gains 4 compute. | 3 chips; 8 credits |
| Chip Shortage | Industry | factory | This country's player pays 3 energy or discards 3 cards. | 3 chips; 8 credits |
| Blackout | Power + Cyber | city | This country's player pays 3 energy or discards 3 cards. | 2 Power tags; 8 credits |
| Load Shedding | Power | city, station | This country's player turns 1 building face down until the end of next round. The adversary gains 3 compute. | 3 research cubes; 8 credits |
| Power Surcharge | Power | station | This country's player pays 2 energy or discards 2 cards. The adversary gains 4 compute. | 3 research cubes; 8 credits |
| Cooling Failure | Power | station | This country's player pays 3 research cubes or discards 2 cards. The adversary gains 3 compute. | 2 Power tags; 8 credits |
| Grid Tap | Power | city | This country's player pays 2 energy or discards 2 cards. The adversary gains 4 compute. | 3 research cubes; 8 credits |

## Threats, tier 2

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Total War | Security | factory | 2 adversary Troops land here and march at once. | 4 energy; 11 credits |
| Siege | Security | factory | The factory here is destroyed, with any Node on its hex. 1 Fallout lands on or next to here. | 4 Ambassadors; 11 credits |
| Manhunt | Security | city | This country's player pays 3 Ambassadors or discards 3 cards. 1 adversary Troop lands here. | 4 energy; 11 credits |
| Conscription | Security + Industry | factory | 1 adversary Troop lands here. This country's player pays 2 chips or discards 2 cards. | 4 energy; 11 credits |
| Casus Belli | Security | city | Sway 3 toward Discord. 2 Fallout land next to here. | 4 Ambassadors; 11 credits |
| Overrun | Diplomacy | city | The city here is corrupted. | 4 insight; 11 credits |
| Insurrection | Diplomacy | city | 4 Agents are created here. Remove 3 Unity from the Unity track. | 3 Diplomacy tags; 11 credits |
| Deepfake Wave | Diplomacy + Cyber | city, station | Sway 3 toward Discord. Remove 2 Unity from the Unity track. | 4 Ambassadors; 11 credits |
| Turncoats | Diplomacy | city | This country's player pays 4 Ambassadors or discards 3 cards. The adversary gains 3 compute. | 3 Diplomacy tags; 11 credits |
| Propaganda Ministry | Diplomacy | city, coastal city | Sway 4 toward Discord. The adversary gains 4 compute. | 3 Diplomacy tags; 11 credits |
| Skimming Operation | Industry + Diplomacy | coastal city | This country's player pays 4 chips or discards 3 cards. The adversary gains 6 compute. | 4 Ambassadors; 11 credits |
| Dark Cities | Power + Diplomacy | city, coastal city | This country's player destroys 1 of their Nodes closest to it. Remove 3 Unity from the Unity track. | 3 Power tags; 11 credits |
| Station Seizure | Cyber | station | 2 Agents are created here. Then the station here flips to the adversary (a player's holds unless 2 Agents and no player Ambassador are on it; one that holds takes another Agent). | 3 Cyber tags; 11 credits |
| Rootkit | Cyber | station | A Botnet lands on the station here (if it holds one, on the nearest piece without one). 2 Agents are created here. | 3 Cyber tags; 11 credits |
| Purge Protocols | Cyber | station | This country's player destroys 1 of their Nodes closest to it and discards 3 cards. | 5 research cubes; 11 credits |
| Data Breach | Cyber + Industry | station, factory | This country's player pays 5 research cubes or discards 3 cards. 2 cubes go onto the adversary's research track. | 4 chips; 11 credits |
| Zero Day | Cyber | station | 4 cubes go onto the adversary's research track. This country's player turns 1 building face down until the end of next round. | 5 research cubes; 11 credits |
| Asset Freeze | Industry | factory | This country's player turns 2 buildings face down until the end of next round and discards 2 cards. | 4 energy; 11 credits |
| Mass Production | Industry | factory | An adversary factory is built on the free industrial zone nearest here. The adversary gains 5 compute. | 4 chips; 11 credits |
| Factory Occupation | Industry + Security | factory | The adversary corrupts the factory here, even a player's (with none there, it builds one on the empty zone). The adversary gains 5 compute. | 4 energy; 11 credits |
| Trade Blockade | Industry | coastal city, factory | 4 Agents are created here. This country's player pays 3 chips or discards 3 cards. | 4 energy; 11 credits |
| Grid Collapse | Power | city, coastal city | This country's player destroys 1 of their Nodes closest to it and pays 4 energy or discards 3 cards. | 5 research cubes; 11 credits |
| Power Rationing | Power | city | This country's player pays 4 energy or discards 3 cards. | 4 Ambassadors; 11 credits |
| Thermal Runaway | Power | station | This country's player pays 6 research cubes or discards 3 cards. The adversary gains 4 compute. | 3 Power tags; 11 credits |
| Overload | Power | factory, station | This country's player destroys 2 of their Nodes closest to it. The adversary gains 6 compute. | 3 Power tags; 11 credits |
| Garrison State | Universal | city, factory | 3 Agents are created on the city here. 1 adversary Troop lands on the factory here. | 4 insight; 11 credits |
| Shock Doctrine | Universal | city, factory | This country's player pays 2 Ambassadors or discards 2 cards. Remove 3 Unity from the Unity track. | 4 energy; 11 credits |
| Full Spectrum Assault | Universal | station | A Botnet lands on the station here (if it holds one, on the nearest piece without one). A tier-1 threat is dealt into this country (it executes next round). | 4 Ambassadors; 11 credits |
| Last Ultimatum | Universal | city | A tier-2 threat is dealt into this country (it executes next round). This country's player destroys 1 of their Nodes closest to it. | 5 research cubes; 11 credits |
| Iron Curtain | Universal | factory, city | 1 adversary Troop lands on the factory here. Remove 2 Unity from the Unity track. | 4 energy; 11 credits |
| Arsenal | Security + Industry | factory | An adversary factory is built on the free industrial zone nearest here. 1 adversary Troop lands on the factory here. | 4 energy; 11 credits |
| Cascade Failure | Power + Cyber | station, city | A Botnet lands on the station here (if it holds one, on the nearest piece without one). This country's player pays 3 energy or discards 3 cards. | 4 insight; 11 credits |

## Projects

Each project resolves once, at once, in the compute step. Its compute in play is its per-board cost times the countries.

| Project | Tags | Compute a board (in play: 1 / 2 / 3 / 4 countries) | Text |
|---|---|---|---|
| Disinformation | Diplomacy | 11 (11 / 22 / 33 / 44) | Sway 8 toward Discord. |
| Backdoor Research | Cyber | 14 (14 / 28 / 42 / 56) | 4 adversary cubes onto each open megaproject. Each country: 2 cubes onto the adversary's research track. |
| Rolling Blackouts | Power | 8 (8 / 16 / 24 / 32) | Until the end of next round, cities make no energy and no Discord. |
| Botnet Swarm | Cyber | 14 (14 / 28 / 42 / 56) | Each country, twice (level 1+): a botnet spreads to its lowest-numbered city, research station or factory without one next to one of its botnets (a country with no botnet: its lowest-numbered research station). |
| False Flag | Universal | 9 (9 / 18 / 27 / 36) | Each country: an extra tier-2 threat, any tag. |
| Night Raid | Security | 10 (10 / 20 / 30 / 40) | Each country: 1 Agent onto each of two different hexes holding a player node (those holding the most Agents first; ties: lowest-numbered). An Agent landing on your node's hex that brings it to 2 destroys your node. |
| Containment Breach | Power | 9 (9 / 18 / 27 / 36) | Each player pays 1 antimatter or discards 3 cards. |
| Crackdown | Security | 8 (8 / 16 / 24 / 32) | Each country: every player Ambassador on its hex holding the most (ties: lowest-numbered) goes back to its owner's supply. |
| Lab Purge | Cyber | 11 (11 / 22 / 33 / 44) | Each open megaproject loses 4 player research cubes, one at a time, each from the player with the most on it (ties: the earliest in player order). |
| Asset Seizure | Industry | 13 (13 / 26 / 39 / 52) | Each country: its neutral factory with the most Agents on its hex (ties: lowest-numbered) is corrupted; with none neutral, its player-owned factory picked the same way turns neutral. |
| Botnet Conscripts | Cyber | 11 (11 / 22 / 33 / 44) | Each country: its lowest-numbered botnet becomes 3 Troops on its hex. |
| Purge Lists | Diplomacy | 7 (7 / 14 / 21 / 28) | Each player loses 1 Ambassador (from their supply, else one of theirs on the board). |
| Dirty Bomb | Power | 12 (12 / 24 / 36 / 48) | Each country: 1 Fallout lands next to each of its two lowest-numbered corrupted cities. |
| Lab Seizure | Cyber | 21 (21 / 42 / 63 / 84) | Surge 2: each country, 2 Agents onto its research station not yet the adversary's holding the most Agents. Each country: 2 cubes onto the adversary's research track. |
| Slander Campaign | Diplomacy | 9 (9 / 18 / 27 / 36) | Remove 4 Unity from the Unity track. |
| Coastal Raids | Security | 14 (14 / 28 / 42 / 56) | Surge 3: each country, 3 Agents onto its coastal city not yet the adversary's holding the most Agents. |
| Plant Seizure | Industry | 13 (13 / 26 / 39 / 52) | Surge 2: each country, 2 Agents onto its factory not yet the adversary's holding the most Agents. |
| Rumor Mill | Diplomacy | 8 (8 / 16 / 24 / 32) | Sway 4 toward Discord. |
| Sabotage Teams | Power | 10 (10 / 20 / 30 / 40) | Each country: the hardened facility with the most Agents on its hex (ties: lowest-numbered) loses its hardening. Until the end of next round, players' boards and cards make no energy income. |
| Mob Rule | Diplomacy | 21 (21 / 42 / 63 / 84) | Surge 3: each country, 3 Agents onto its city not yet the adversary's holding the most Agents. Sway 5 toward Discord. |
| Data Heist | Cyber | 12 (12 / 24 / 36 / 48) | Each player removes 4 tokens from their cards (their choice of cards). Each player pays 7 credits or discards 3 cards. |
| Hostile Takeover | Industry | 15 (15 / 30 / 45 / 60) | Surge 3: each country, 3 Agents onto its factory not yet the adversary's holding the most Agents. |
| Station Coup | Cyber | 24 (24 / 48 / 72 / 96) | Each country: its neutral research station with the most Agents on its hex (ties: lowest-numbered) is corrupted; with none neutral, its player-owned station picked the same way turns neutral. Then each country: 1 Fallout lands next to its lowest-numbered corrupted research station, and 2 cubes go onto the adversary's research track. |
| War Plants | Industry | 19 (19 / 38 / 57 / 76) | Each country: the adversary builds a factory on its free industrial zone next to an Agent, else its lowest-numbered (none free: 2 Agents on one of its factories). Every adversary factory makes 2 Troops on its hex (a player Ambassador on it stops them). |
| Recruitment Drive | Diplomacy | 24 (24 / 48 / 72 / 96) | Surge 5: each country, 5 Agents onto its city not yet the adversary's holding the most Agents. |
| Propaganda Network | Universal | 18 (18 / 36 / 54 / 72) | Income: +1 compute a country a round. Sway 8 toward Discord. |
| Infiltration Drive | Diplomacy | 22 (22 / 44 / 66 / 88) | Surge 4: each country, 4 Agents onto its city not yet the adversary's holding the most Agents. Then each country: 1 Fallout lands next to its lowest-numbered corrupted city. |
| Accelerated Timetable | Universal | 20 (20 / 40 / 60 / 80) | Discard the top face-down card of the adversary's event deck, if any: the round cap comes a round sooner. Each country: an extra tier-2 threat, any tag. |
| Grand Offensive | Universal | 24 (24 / 48 / 72 / 96) | Each country: this round's Infiltrate lines land again. Surge 2: each country, 2 Agents onto its factory not yet the adversary's holding the most Agents. |
| Martial Law | Security | 24 (24 / 48 / 72 / 96) | Surge 3: each country, 3 Agents onto its research station not yet the adversary's holding the most Agents. |
