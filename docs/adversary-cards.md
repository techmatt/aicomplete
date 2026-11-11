# Adversary cards (generated)

*Written by `python scripts/content/load_adversary.py` from `drafts/adversary/` (the source); edit the drafts, not this page. The rules are in [rules/threats.md](rules/threats.md) and [rules/adversary.md](rules/adversary.md).*

## The Militarist (board)

Project slots: Military, Open, Open.

- **Garrisons.** Income: +1 Agent a country a round.
- **Troops where the Protector stands.** Each round: Muster 1 in each country where the Protector has a Node.
- **Warmongering.** Each round, a military base with an adversary Troop on or next to it makes 1 extra Drone, even past its cap; at most 1 extra a base in each production, whatever makes it.
- **Difficulty.** Militarist, harder: every Muster lands 1 more Troop a country, and the adversary gains 4 compute for each threat that executes uncountered.

## Event deck

| Card | Stage | Text |
|---|---|---|
| Probing Strikes | 1 | Threat, each country: Military 1. Rally: coastal cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Corrupt: cities. |
| Factory Drive | 1 | Threat, each country: Military 1 or Economic 1. Rally: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Each country: a neutral factory opens on its lowest-numbered free site. Corrupt: factories. |
| Long Patrols | 1 | Threat, each country: Military 1 or Cyber 1. Rally: research stations. Income: +1 Agent a country a round. Corrupt: research stations. |
| War Drums | 1 | Threat, each country: Military 1 or Influence 1. Rally: cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Sway 1 toward Discord. Corrupt: cities. |
| Signal Jamming | 1 | Threat, each country: Military 1 or Grid 1. Rally: Botnet hexes. Income: +2 compute a round. Corrupt: cities. |
| Mobilization | 2 | Threat, each country: Military 2. Rally: coastal cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Corrupt: cities. |
| Station Raids | 2 | Threat, each country: Military 2 or Cyber 2. Rally: research stations. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Corrupt: research stations. |
| Fifth Column | 2 | Threat, each country: Influence 2 or Cyber 2. Rally: cities. Income: +1 Agent a country a round. Corrupt: cities (1 fewer). |
| Border War | 2 | Threat, each country: Military 2. Rally: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Each player with a Node on or next to a front's base destroys 1 of those Nodes. Corrupt: factories. |
| Supply Lines | 2 | Threat, each country: Military 1 or Economic 1, and Military 2 or Economic 2. Rally: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +2 compute a round. Corrupt: factories. |
| Forced March | 2 | Threat, each country: any tier-2 threat. Rally: cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Every adversary Troop takes 1 more step this round. Corrupt: cities. |
| Total Mobilization | 3 | Threat, each country: Military 2, and Economic 2 or Grid 2. Rally: factories. Muster 2: each country, 2 Troops on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Remove 2 Unity from the Unity track. Corrupt: factories. Corrupt: cities. |
| Blitzkrieg | 3 | Threat, each country: any tier-2 threat, and Military 2 or Influence 2. Rally: cities. Muster 2: each country, 2 Troops on its lowest-numbered factory (else city). Sway 1 toward Discord. Corrupt: cities. Corrupt: land. |
| Occupation | 3 | Threat, each country: Military 2 or Cyber 2, and Influence 2. Rally: research stations. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Corrupt: research stations. Corrupt: cities (1 fewer). |
| Naval Blockade | 3 | Threat, each country: Economic 2 or Grid 2, and Military 2 or Cyber 2. Rally: coastal cities. Muster 2: each country, 2 Troops on its lowest-numbered factory (else city). Until the end of next round, cities make no energy. Corrupt: cities. Corrupt: land. |

## Threats, tier 1

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Orbital Strike | Military | base | 2 adversary Troops land here. | 3 energy; 12 credits |
| Commandeer | Military | base, factory | The nearest player pays 3 energy or discards 3 cards. The adversary gains 2 compute. | 3 Ambassadors; 12 credits |
| Supply Raid | Military + Economic | factory | 1 adversary Troop lands here. The nearest player pays 3 chips or discards 1 card. | 2 Industry tags; 12 credits |
| Eviction | Military | city | The nearest player pays 2 Ambassadors or discards 2 cards. The adversary gains 4 compute. | 2 Security tags; 12 credits |
| Call-Up | Military | base | 1 adversary Troop lands here. Sway 2 toward Discord. | 3 Ambassadors; 12 credits |
| Provocation | Influence + Military | base | A tier-1 threat is dealt into this country (it executes next round). Sway 2 toward Discord. | 3 Ambassadors; 12 credits |
| Agitators | Influence | city | 3 Agents are created here. Sway 2 toward Discord. | 2 Diplomacy tags; 12 credits |
| Defection | Influence | city | The nearest player pays 2 Ambassadors or discards 2 cards. Sway 1 toward Discord. | 2 Diplomacy tags; 12 credits |
| Smear Campaign | Influence | city, station | 2 Unity comes off the Unity track. The nearest player gets 4 less insight next round. | 3 Ambassadors; 12 credits |
| Propaganda Blitz | Influence | coastal city | Sway 3 toward Discord. The adversary gains 3 compute. | 3 Ambassadors; 12 credits |
| Sabotage | Cyber | station | The nearest player loses 4 tokens from their cards. The adversary gains 6 compute. | 2 Cyber tags; 12 credits |
| Insider Access | Cyber + Influence | station | 2 Agents are created here. The nearest player pays 3 research cubes or discards 2 cards. | 3 Ambassadors; 12 credits |
| Wiper Malware | Cyber | station | The nearest player discards 3 cards. | 3 research cubes; 12 credits |
| Cryptojacking | Cyber | station, factory | The adversary gains 6 compute. The nearest player pays 2 research cubes or discards 1 card. | 2 Cyber tags; 12 credits |
| Backdoor | Cyber | station | A Botnet lands on the station here. | 6 insight; 12 credits |
| Requisition | Economic | factory, city | The nearest player pays 5 chips or discards 2 cards. The adversary gains 3 compute. | 2 Industry tags; 12 credits |
| Shakedown | Economic + Influence | city | Theft: the nearest player pays 5 chips or discards 2 cards; the adversary gains 4 compute. | 3 Ambassadors; 12 credits |
| Embargo | Economic | factory | 2 Agents are created here. The nearest player pays 2 energy or discards 2 cards. | 6 chips; 12 credits |
| Credit Crunch | Economic | coastal city | The nearest player discards 2 cards. The adversary gains 4 compute. | 6 chips; 12 credits |
| Chip Shortage | Economic | factory | The nearest player pays 3 energy or discards 3 cards. | 6 chips; 12 credits |
| Blackout | Grid + Cyber | city | The nearest player pays 3 energy or discards 3 cards. | 2 Power tags; 12 credits |
| Load Shedding | Grid | city, station | The nearest player turns 1 building face down until the end of next round. The adversary gains 3 compute. | 3 research cubes; 12 credits |
| Power Surcharge | Grid | station | The nearest player pays 2 energy or discards 2 cards. The adversary gains 4 compute. | 3 research cubes; 12 credits |
| Cooling Failure | Grid | station | The nearest player pays 3 research cubes or discards 2 cards. The adversary gains 3 compute. | 2 Power tags; 12 credits |
| Grid Tap | Grid | city | Theft: the nearest player pays 2 energy or discards 2 cards; the adversary gains 4 compute. | 3 research cubes; 12 credits |

## Threats, tier 2

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Total War | Military | base | 3 adversary Troops land here and march at once. | 4 energy; 17 credits |
| Siege | Military | factory | The factory here is destroyed. The adversary gains 4 compute. | 4 Ambassadors; 17 credits |
| Manhunt | Military | base, city | The nearest player pays 3 Ambassadors or discards 3 cards. 1 adversary Troop lands on the base here. | 4 energy; 17 credits |
| Conscription | Military + Economic | base | 2 adversary Troops land here. The nearest player pays 4 chips or discards 2 cards. | 4 energy; 17 credits |
| Casus Belli | Military | base | This base's front has a second war step next round. Sway 3 toward Discord. | 4 Ambassadors; 17 credits |
| Overrun | Influence | city | The city here is corrupted. | 7 insight; 17 credits |
| Insurrection | Influence | city | 4 Agents are created here. 3 Unity comes off the Unity track. | 3 Diplomacy tags; 17 credits |
| Deepfake Wave | Influence + Cyber | city, station | Sway 3 toward Discord. 2 Unity comes off the Unity track. | 4 Ambassadors; 17 credits |
| Turncoats | Influence | city | The nearest player pays 4 Ambassadors or discards 3 cards. The adversary gains 3 compute. | 3 Diplomacy tags; 17 credits |
| Propaganda Ministry | Influence | city, coastal city | Sway 4 toward Discord. The adversary gains 4 compute. | 3 Diplomacy tags; 17 credits |
| Skimming Operation | Economic + Influence | coastal city | Theft: the nearest player pays 8 chips or discards 3 cards; the adversary gains 6 compute. | 4 Ambassadors; 17 credits |
| Dark Cities | Grid + Influence | city, coastal city | The nearest player destroys 1 of their Nodes closest to it. 3 Unity comes off the Unity track. | 3 Power tags; 17 credits |
| Station Seizure | Cyber | station | The station here is corrupted, even a player's. The adversary gains 4 compute. | 3 Cyber tags; 17 credits |
| Sleeper Cells | Cyber | station | A Botnet lands on the station here. 2 Agents are created here. | 3 Cyber tags; 17 credits |
| Purge Protocols | Cyber | station | The nearest player destroys 1 of their Nodes closest to it and discards 3 cards. | 5 research cubes; 17 credits |
| Data Breach | Cyber + Economic | station, factory | The nearest player pays 5 research cubes or discards 3 cards. The adversary gains 5 compute. | 7 chips; 17 credits |
| Zero Day | Cyber | station | The adversary gains 10 compute. The nearest player turns 1 building face down until the end of next round. | 5 research cubes; 17 credits |
| Asset Freeze | Economic | factory | The nearest player turns 2 buildings face down until the end of next round and discards 2 cards. | 4 energy; 17 credits |
| Mass Production | Economic | factory | An adversary factory is built on the free industrial zone nearest here. The adversary gains 5 compute. | 7 chips; 17 credits |
| Factory Occupation | Economic + Military | factory | The adversary seizes the factory here, even a player's. The adversary gains 5 compute. | 4 energy; 17 credits |
| Trade Blockade | Economic | coastal city, factory | 4 Agents are created here. The nearest player pays 6 chips or discards 3 cards. | 4 energy; 17 credits |
| Grid Collapse | Grid | city, coastal city | The nearest player destroys 1 of their Nodes closest to it and pays 4 energy or discards 3 cards. | 5 research cubes; 17 credits |
| Power Rationing | Grid | city | The nearest player pays 4 energy or discards 3 cards. | 4 Ambassadors; 17 credits |
| Thermal Runaway | Grid | station | The nearest player pays 6 research cubes or discards 3 cards. The adversary gains 4 compute. | 3 Power tags; 17 credits |
| Overload | Grid | factory, station | The nearest player destroys 2 of their Nodes closest to it. The adversary gains 6 compute. | 3 Power tags; 17 credits |
| Garrison State | Universal | city, base | 3 Agents are created on the city here. 2 adversary Troops land on the base here. | 7 insight; 17 credits |
| Shock Doctrine | Universal | city, factory | The nearest player pays 2 Ambassadors or discards 2 cards. 3 Unity comes off the Unity track. | 4 energy; 17 credits |
| Full Spectrum Assault | Universal | station | A Botnet lands on the station here. A tier-1 threat is dealt into this country (it executes next round). | 4 Ambassadors; 17 credits |
| Last Ultimatum | Universal | city, base | A tier-2 threat is dealt into this country (it executes next round). The nearest player destroys 1 of their Nodes closest to it. | 5 research cubes; 17 credits |
| Iron Curtain | Universal | factory, city | 2 adversary Troops land on the factory here. 2 Unity comes off the Unity track. | 4 energy; 17 credits |
| War Economy | Military + Economic | factory, base | An adversary factory is built on the free industrial zone nearest here. 1 adversary Troop lands on the factory here. | 4 energy; 17 credits |
| Cascade Failure | Grid + Cyber | station, city | A Botnet lands on the station here. The nearest player pays 3 energy or discards 3 cards. | 7 insight; 17 credits |

## Projects

| Project | Tags | Compute (1 / 2 / 3 / 4 players) | Resolves | Text |
|---|---|---|---|---|
| Raiding Parties | Military | 4 / 8 / 12 / 16 | once | Each player pays 5 chips or discards 3 cards. |
| War Footing | Military | 7 / 14 / 21 / 28 | once | Every military base fills with Drones to its cap. |
| Purge the Unaligned | Military | 7 / 14 / 21 / 28 | once | Each player pays 2 Ambassadors or discards 2 cards. |
| Armored Columns | Military | 10 / 20 / 30 / 40 | once | Muster 3: each country, 3 Troops on its lowest-numbered factory (else city). |
| Arms Race | Military | 11 / 22 / 33 / 44 | ongoing | Ongoing: Each round, every military base makes 1 extra Drone, even past its cap (at most 1 extra a base each production). |
| Declaration of War | Military | 18 / 36 / 54 / 72 | ongoing | Now: Muster 3: each country, 3 Troops on its lowest-numbered factory (else city). Ongoing: Each round, the front with the most Drones fights twice. |
| Disinformation | Influence | 5 / 10 / 15 / 20 | once | Sway 5 toward Discord. |
| Recruitment Drive | Influence | 11 / 22 / 33 / 44 | once | Income: +1 Agent a country a round. |
| Infiltration Drive | Influence | 18 / 36 / 54 / 72 | once | Rally 3, each country: cities. Corrupt: cities (1 fewer). |
| Data Heist | Cyber | 6 / 12 / 18 / 24 | once | Each player removes 3 tokens from their cards (their choice of cards). Each player pays 3 insight next round or discards 2 cards. |
| Botnet Swarm | Cyber | 6 / 12 / 18 / 24 | once | Rally 3, each country: Botnet hexes. |
| Station Coup | Cyber | 9 / 18 / 27 / 36 | once | Rally 2, each country: research stations. Each country: its lowest-numbered research station not the adversary's flips (a player's holds unless 2 Agents and no player Ambassador are on it; one that holds takes 1 Agent). |
| Market Crash | Economic | 5 / 10 / 15 / 20 | once | Each player pays 10 credits or discards 3 cards. |
| Shadow Factories | Economic | 7 / 14 / 21 / 28 | once | The adversary builds a factory in the country with the most free industrial zones (none free: 2 Agents on a factory). |
| Capital Flight | Economic | 8 / 16 / 24 / 32 | once | Each player's credit income falls by 4 for the rest of the game. |
| Rolling Blackouts | Grid | 6 / 12 / 18 / 24 | once | Until the end of next round, cities make no energy. |
| Grid Capture | Grid | 9 / 18 / 27 / 36 | ongoing | Ongoing: Each player pays 1 energy or discards 1 card. |
| False Flag | Universal | 6 / 12 / 18 / 24 | once | Each country: an extra tier-2 threat, any tag. |
| Grand Offensive | Universal | 10 / 20 / 30 / 40 | once | Each country rallies its Agent income now, to this round's Rally. Corrupt: cities. |
| Propaganda Network | Universal | 14 / 28 / 42 / 56 | ongoing | Now: Income: +2 compute a round. Ongoing: Sway 2 toward Discord. |
