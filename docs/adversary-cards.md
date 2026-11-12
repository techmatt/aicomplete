# Adversary cards (generated)

*Written by `python scripts/content/load_adversary.py` from `drafts/adversary/` (the source); edit the drafts, not this page. The rules are in [rules/threats.md](rules/threats.md) and [rules/adversary.md](rules/adversary.md).*

## The Militarist (board)

Project slots: Open, Open, Open.

- **Garrisons.** Income: +1 Agent a country a round.
- **Troops where the Protector stands.** Each round: Muster 1 in each country where the Protector has a Node.
- **Warmongering.** Each round, a military base with an adversary Troop on or next to it makes 1 extra Drone, even past its cap; at most 1 extra a base in each production, whatever makes it.
- **War Takes Ground.** When a Drone destroys a factory, it also destroys a Node on that hex.
- **Difficulty.** Militarist, harder: every Muster lands 1 more Troop a country, and the adversary gains 4 compute for each threat that executes uncountered.

## Event deck

| Card | Stage | Text |
|---|---|---|
| Probing Strikes | 1 | Threat, each country: Security 1. Rally: coastal cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Corrupt: cities. |
| Factory Drive | 1 | Threat, each country: Security 1 or Industry 1. Rally: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Each country: a neutral factory opens on its lowest-numbered free site. Corrupt: factories. |
| Long Patrols | 1 | Threat, each country: Security 1 or Cyber 1. Rally: research stations. Income: +1 Agent a country a round. Corrupt: research stations. |
| War Drums | 1 | Threat, each country: Security 1 or Diplomacy 1. Rally: cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Sway 1 toward Discord. Corrupt: cities. |
| Signal Jamming | 1 | Threat, each country: Security 1 or Power 1. Rally: Botnet hexes. Income: +2 compute a round. Corrupt: cities. |
| Mobilization | 2 | Threat, each country: Security 2. Rally: coastal cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Corrupt: cities. |
| Station Raids | 2 | Threat, each country: Security 2 or Cyber 2. Rally: research stations. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Corrupt: research stations. |
| Fifth Column | 2 | Threat, each country: Diplomacy 2 or Cyber 2. Rally: cities. Income: +1 Agent a country a round. Corrupt: cities (1 fewer). |
| Border War | 2 | Threat, each country: Security 2. Rally: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Each player with a Node on or next to a front's base destroys 1 of those Nodes. Corrupt: factories. |
| Supply Lines | 2 | Threat, each country: Security 1 or Industry 1, and Security 2 or Industry 2. Rally: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +2 compute a round. Corrupt: factories. |
| Forced March | 2 | Threat, each country: any tier-2 threat. Rally: cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Every adversary Troop takes 1 more step this round. Corrupt: cities. |
| Total Mobilization | 3 | Threat, each country: Security 2, and Industry 2 or Power 2. Rally: factories. Muster 2: each country, 2 Troops on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Remove 2 Unity from the Unity track. Corrupt: factories. Corrupt: cities. |
| Blitzkrieg | 3 | Threat, each country: any tier-2 threat, and Security 2 or Diplomacy 2. Rally: cities. Muster 2: each country, 2 Troops on its lowest-numbered factory (else city). Sway 1 toward Discord. Corrupt: cities. Corrupt: land. |
| Occupation | 3 | Threat, each country: Security 2 or Cyber 2, and Diplomacy 2. Rally: research stations. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Corrupt: research stations. Corrupt: cities (1 fewer). |
| Naval Blockade | 3 | Threat, each country: Industry 2 or Power 2, and Security 2 or Cyber 2. Rally: coastal cities. Muster 2: each country, 2 Troops on its lowest-numbered factory (else city). Until the end of next round, cities make no energy. Corrupt: cities. Corrupt: land. |

## Threats, tier 1

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Orbital Strike | Security | base | 2 adversary Troops land here. | 3 energy; 12 credits |
| Commandeer | Security | base, factory | The nearest player pays 3 energy or discards 3 cards. The adversary gains 2 compute. | 3 Ambassadors; 12 credits |
| Supply Raid | Security + Industry | factory | 1 adversary Troop lands here. The nearest player pays 2 chips or discards 1 card. | 2 Industry tags; 12 credits |
| Eviction | Security | city | The nearest player pays 2 Ambassadors or discards 2 cards. The adversary gains 4 compute. | 2 Security tags; 12 credits |
| Call-Up | Security | base | 1 adversary Troop lands here. Sway 2 toward Discord. | 3 Ambassadors; 12 credits |
| Provocation | Diplomacy + Security | base | A tier-1 threat is dealt into this country (it executes next round). Sway 2 toward Discord. | 3 Ambassadors; 12 credits |
| Agitators | Diplomacy | city | 3 Agents are created here. Sway 2 toward Discord. | 2 Diplomacy tags; 12 credits |
| Defection | Diplomacy | city | The nearest player pays 2 Ambassadors or discards 2 cards. Sway 1 toward Discord. | 2 Diplomacy tags; 12 credits |
| Smear Campaign | Diplomacy | city, station | 2 Unity comes off the Unity track. The nearest player pays 2 energy or discards 2 cards. | 3 Ambassadors; 12 credits |
| Propaganda Blitz | Diplomacy | coastal city | Sway 3 toward Discord. The adversary gains 3 compute. | 3 Ambassadors; 12 credits |
| Sabotage | Cyber | station | The nearest player loses 4 tokens from their cards. The adversary gains 6 compute. | 2 Cyber tags; 12 credits |
| Insider Access | Cyber + Diplomacy | station | 2 Agents are created here. The nearest player pays 3 research cubes or discards 2 cards. | 3 Ambassadors; 12 credits |
| Wiper Malware | Cyber | station | The nearest player discards 3 cards. | 3 research cubes; 12 credits |
| Cryptojacking | Cyber | station, factory | The adversary gains 6 compute. The nearest player pays 2 research cubes or discards 1 card. | 2 Cyber tags; 12 credits |
| Backdoor | Cyber | station | A Botnet lands on the station here. | 6 insight; 12 credits |
| Requisition | Industry | factory, city | The nearest player pays 3 chips or discards 2 cards. The adversary gains 3 compute. | 2 Industry tags; 12 credits |
| Shakedown | Industry + Diplomacy | city | Theft: the nearest player pays 3 chips or discards 2 cards; the adversary gains 4 compute. | 3 Ambassadors; 12 credits |
| Embargo | Industry | factory | 2 Agents are created here. The nearest player pays 2 energy or discards 2 cards. | 3 chips; 12 credits |
| Credit Crunch | Industry | coastal city | The nearest player discards 2 cards. The adversary gains 4 compute. | 3 chips; 12 credits |
| Chip Shortage | Industry | factory | The nearest player pays 3 energy or discards 3 cards. | 3 chips; 12 credits |
| Blackout | Power + Cyber | city | The nearest player pays 3 energy or discards 3 cards. | 2 Power tags; 12 credits |
| Load Shedding | Power | city, station | The nearest player turns 1 building face down until the end of next round. The adversary gains 3 compute. | 3 research cubes; 12 credits |
| Power Surcharge | Power | station | The nearest player pays 2 energy or discards 2 cards. The adversary gains 4 compute. | 3 research cubes; 12 credits |
| Cooling Failure | Power | station | The nearest player pays 3 research cubes or discards 2 cards. The adversary gains 3 compute. | 2 Power tags; 12 credits |
| Grid Tap | Power | city | Theft: the nearest player pays 2 energy or discards 2 cards; the adversary gains 4 compute. | 3 research cubes; 12 credits |

## Threats, tier 2

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Total War | Security | base | 3 adversary Troops land here and march at once. | 4 energy; 17 credits |
| Siege | Security | factory | The factory here is destroyed. The adversary gains 4 compute. | 4 Ambassadors; 17 credits |
| Manhunt | Security | base, city | The nearest player pays 3 Ambassadors or discards 3 cards. 1 adversary Troop lands on the base here. | 4 energy; 17 credits |
| Conscription | Security + Industry | base | 2 adversary Troops land here. The nearest player pays 2 chips or discards 2 cards. | 4 energy; 17 credits |
| Casus Belli | Security | base | This base's front has a second war step next round. Sway 3 toward Discord. | 4 Ambassadors; 17 credits |
| Overrun | Diplomacy | city | The city here is corrupted. | 7 insight; 17 credits |
| Insurrection | Diplomacy | city | 4 Agents are created here. 3 Unity comes off the Unity track. | 3 Diplomacy tags; 17 credits |
| Deepfake Wave | Diplomacy + Cyber | city, station | Sway 3 toward Discord. 2 Unity comes off the Unity track. | 4 Ambassadors; 17 credits |
| Turncoats | Diplomacy | city | The nearest player pays 4 Ambassadors or discards 3 cards. The adversary gains 3 compute. | 3 Diplomacy tags; 17 credits |
| Propaganda Ministry | Diplomacy | city, coastal city | Sway 4 toward Discord. The adversary gains 4 compute. | 3 Diplomacy tags; 17 credits |
| Skimming Operation | Industry + Diplomacy | coastal city | Theft: the nearest player pays 4 chips or discards 3 cards; the adversary gains 6 compute. | 4 Ambassadors; 17 credits |
| Dark Cities | Power + Diplomacy | city, coastal city | The nearest player destroys 1 of their Nodes closest to it. 3 Unity comes off the Unity track. | 3 Power tags; 17 credits |
| Station Seizure | Cyber | station | 2 Agents are created here. Then the station here flips to the adversary (a player's holds unless 2 Agents and no player Ambassador are on it; one that holds takes another Agent). | 3 Cyber tags; 17 credits |
| Sleeper Cells | Cyber | station | A Botnet lands on the station here. 2 Agents are created here. | 3 Cyber tags; 17 credits |
| Purge Protocols | Cyber | station | The nearest player destroys 1 of their Nodes closest to it and discards 3 cards. | 5 research cubes; 17 credits |
| Data Breach | Cyber + Industry | station, factory | The nearest player pays 5 research cubes or discards 3 cards. The adversary gains 5 compute. | 4 chips; 17 credits |
| Zero Day | Cyber | station | The adversary gains 10 compute. The nearest player turns 1 building face down until the end of next round. | 5 research cubes; 17 credits |
| Asset Freeze | Industry | factory | The nearest player turns 2 buildings face down until the end of next round and discards 2 cards. | 4 energy; 17 credits |
| Mass Production | Industry | factory | An adversary factory is built on the free industrial zone nearest here. The adversary gains 5 compute. | 4 chips; 17 credits |
| Factory Occupation | Industry + Security | factory | The adversary seizes the factory here, even a player's. The adversary gains 5 compute. | 4 energy; 17 credits |
| Trade Blockade | Industry | coastal city, factory | 4 Agents are created here. The nearest player pays 3 chips or discards 3 cards. | 4 energy; 17 credits |
| Grid Collapse | Power | city, coastal city | The nearest player destroys 1 of their Nodes closest to it and pays 4 energy or discards 3 cards. | 5 research cubes; 17 credits |
| Power Rationing | Power | city | The nearest player pays 4 energy or discards 3 cards. | 4 Ambassadors; 17 credits |
| Thermal Runaway | Power | station | The nearest player pays 6 research cubes or discards 3 cards. The adversary gains 4 compute. | 3 Power tags; 17 credits |
| Overload | Power | factory, station | The nearest player destroys 2 of their Nodes closest to it. The adversary gains 6 compute. | 3 Power tags; 17 credits |
| Garrison State | Universal | city, base | 3 Agents are created on the city here. 2 adversary Troops land on the base here. | 7 insight; 17 credits |
| Shock Doctrine | Universal | city, factory | The nearest player pays 2 Ambassadors or discards 2 cards. 3 Unity comes off the Unity track. | 4 energy; 17 credits |
| Full Spectrum Assault | Universal | station | A Botnet lands on the station here. A tier-1 threat is dealt into this country (it executes next round). | 4 Ambassadors; 17 credits |
| Last Ultimatum | Universal | city, base | A tier-2 threat is dealt into this country (it executes next round). The nearest player destroys 1 of their Nodes closest to it. | 5 research cubes; 17 credits |
| Iron Curtain | Universal | factory, city | 2 adversary Troops land on the factory here. 2 Unity comes off the Unity track. | 4 energy; 17 credits |
| War Economy | Security + Industry | factory, base | An adversary factory is built on the free industrial zone nearest here. 1 adversary Troop lands on the factory here. | 4 energy; 17 credits |
| Cascade Failure | Power + Cyber | station, city | A Botnet lands on the station here. The nearest player pays 3 energy or discards 3 cards. | 7 insight; 17 credits |

## Projects

| Project | Tags | Compute (1 / 2 / 3 / 4 players) | Resolves | Text |
|---|---|---|---|---|
| Raiding Parties | Security | 4 / 8 / 12 / 16 | once | Each player pays 3 chips or discards 3 cards. |
| War Footing | Security | 7 / 14 / 21 / 28 | once | Every military base fills with Drones to its cap. |
| Purge the Unaligned | Security | 7 / 14 / 21 / 28 | once | Each player pays 2 Ambassadors or discards 2 cards. |
| Armored Columns | Security | 10 / 20 / 30 / 40 | once | Muster 3: each country, 3 Troops on its lowest-numbered factory (else city). |
| Arms Race | Security | 11 / 22 / 33 / 44 | ongoing | Ongoing: Each round, every military base makes 1 extra Drone, even past its cap (at most 1 extra a base each production). |
| Declaration of War | Security | 18 / 36 / 54 / 72 | ongoing | Now: Muster 3: each country, 3 Troops on its lowest-numbered factory (else city). Ongoing: Each round, the front with the most Drones fights twice. |
| Disinformation | Diplomacy | 5 / 10 / 15 / 20 | once | Sway 5 toward Discord. |
| Recruitment Drive | Diplomacy | 11 / 22 / 33 / 44 | once | Income: +1 Agent a country a round. |
| Infiltration Drive | Diplomacy | 18 / 36 / 54 / 72 | once | Rally 3, each country: cities. Corrupt: cities (1 fewer). |
| Data Heist | Cyber | 6 / 12 / 18 / 24 | once | Each player removes 3 tokens from their cards (their choice of cards). Each player pays 8 credits or discards 2 cards. |
| Botnet Swarm | Cyber | 6 / 12 / 18 / 24 | once | Rally 3, each country: Botnet hexes. |
| Station Coup | Cyber | 9 / 18 / 27 / 36 | once | Rally 2, each country: research stations. Each country: its lowest-numbered research station not the adversary's flips (a player's holds unless 2 Agents and no player Ambassador are on it; one that holds takes 1 Agent). |
| Market Crash | Industry | 5 / 10 / 15 / 20 | once | Each player pays 10 credits or discards 3 cards. |
| Shadow Factories | Industry | 7 / 14 / 21 / 28 | once | The adversary builds a factory in the country with the most free industrial zones (none free: 2 Agents on a factory). |
| Capital Flight | Industry | 8 / 16 / 24 / 32 | once | Each player's credit income falls by 4 for the rest of the game. |
| Rolling Blackouts | Power | 6 / 12 / 18 / 24 | once | Until the end of next round, cities make no energy. |
| Grid Capture | Power | 9 / 18 / 27 / 36 | ongoing | Ongoing: Each player pays 1 energy or discards 1 card. |
| False Flag | Universal | 6 / 12 / 18 / 24 | once | Each country: an extra tier-2 threat, any tag. |
| Grand Offensive | Universal | 10 / 20 / 30 / 40 | once | Each country rallies its Agent income now, to this round's Rally. Corrupt: cities. |
| Propaganda Network | Universal | 14 / 28 / 42 / 56 | ongoing | Now: Income: +2 compute a round. Ongoing: Sway 2 toward Discord. |
