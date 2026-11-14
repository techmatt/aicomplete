# Adversary cards (generated)

*Written by `python scripts/content/load_adversary.py` from `drafts/adversary/` (the source); edit the drafts, not this page. The rules are in [rules/threats.md](rules/threats.md) and [rules/adversary.md](rules/adversary.md).*

## The Militarist (board)

Project slots: Open, Open, Open.

- **Garrisons.** Income: +1 Agent a country a round.
- **Warmongering.** Each round, a military base with an adversary Troop on or next to it makes 1 extra Drone, even past its cap; at most 1 extra a base in each production, whatever makes it.
- **War Takes Ground.** When a Drone destroys a factory, it also destroys a Node on that hex.
- **Difficulty.** Militarist, harder: every Muster lands 1 more Troop a country, and the adversary gains 4 compute for each threat that executes uncountered.

## Event deck

| Card | Stage | Text |
|---|---|---|
| Probing Strikes | 1 | Threat, each country: Security 1. Rally: coastal cities. Corrupt: cities. |
| Factory Drive | 1 | Threat, each country: Security 1 or Industry 1. Rally: factories. Each country: a neutral factory opens on its lowest-numbered free site. Corrupt: factories. |
| Long Patrols | 1 | Threat, each country: Security 1 or Cyber 1. Rally: research stations. Income: +1 Agent a country a round. Botnets spread: each country, a Botnet onto its lowest-numbered site next to a Botnet (none: its lowest-numbered research station). Corrupt: research stations. |
| War Drums | 1 | Threat, each country: Security 1 or Diplomacy 1. Rally: cities. Sway 1 toward Discord. Corrupt: cities. |
| Signal Jamming | 1 | Threat, each country: Security 1 or Power 1. Rally: Botnet hexes. Income: +1 compute a country a round. Botnets spread: each country, a Botnet onto its lowest-numbered site next to a Botnet (none: its lowest-numbered research station). Corrupt: cities. |
| Mobilization | 2 | Threat, each country: Security 2. Rally: coastal cities. Income: +1 Agent a country a round. Corrupt: cities. |
| Station Raids | 2 | Threat, each country: Security 2 or Cyber 2. Rally: research stations. Botnets spread: each country, a Botnet onto its lowest-numbered site next to a Botnet (none: its lowest-numbered research station). Income: +1 Agent a country a round. Corrupt: research stations. |
| Fifth Column | 2 | Threat, each country: Diplomacy 2 or Cyber 2. Rally: cities. Income: +1 Agent a country a round. Botnets spread: each country, a Botnet onto its lowest-numbered site next to a Botnet (none: its lowest-numbered research station). Corrupt: cities (1 fewer). |
| Border War | 2 | Threat, each country: Security 2. Rally: factories. Each player with a Node on or next to a front's base destroys 1 of those Nodes. Income: +1 Agent a country a round. Corrupt: factories. |
| Supply Lines | 2 | Threat, each country: Security 1 or Industry 1, and Security 2 or Industry 2. Rally: factories. Income: +1 compute and +1 Agent a country a round. Corrupt: factories. |
| Forced March | 2 | Threat, each country: any tier-2 threat. Rally: cities. Income: +1 Agent a country a round. Every adversary Troop takes 1 more step this round. Corrupt: cities. |
| Total Mobilization | 3 | Threat, each country: Security 2, and Industry 2 or Power 2. Rally: factories. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Remove 2 Unity from the Unity track. Corrupt: factories. Corrupt: cities. |
| Blitzkrieg | 3 | Threat, each country: any tier-2 threat, and Security 2 or Diplomacy 2. Rally: cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Sway 1 toward Discord. Corrupt: cities. Corrupt: land. |
| Occupation | 3 | Threat, each country: Security 2 or Cyber 2, and Diplomacy 2. Rally: research stations. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Income: +1 Agent a country a round. Corrupt: research stations. Corrupt: cities (1 fewer). |
| Naval Blockade | 3 | Threat, each country: Industry 2 or Power 2, and Security 2 or Cyber 2. Rally: coastal cities. Muster 1: each country, 1 Troop on its lowest-numbered factory (else city). Until the end of next round, cities make no energy. Income: +1 Agent a country a round. Corrupt: cities. Corrupt: land. |

## Threats, tier 1

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Orbital Strike | Security | base | 1 adversary Troop lands here. | 3 energy; 12 credits |
| Commandeer | Security | base, factory | The nearest player pays 3 energy or discards 3 cards. The adversary gains 2 compute. | 3 Ambassadors; 12 credits |
| Supply Raid | Security + Industry | factory | 1 adversary Troop lands here. The nearest player pays 2 chips or discards 1 card. | 2 Industry tags; 12 credits |
| Eviction | Security | city | The nearest player pays 2 Ambassadors or discards 2 cards. The adversary gains 4 compute. | 2 Security tags; 12 credits |
| Call-Up | Security | base | 1 adversary Troop lands here. 1 Fallout lands next to here. | 3 Ambassadors; 12 credits |
| Provocation | Diplomacy + Security | base | A tier-1 threat is dealt into this country (it executes next round). Sway 2 toward Discord. | 3 Ambassadors; 12 credits |
| Agitators | Diplomacy | city | 3 Agents are created here. Sway 2 toward Discord. | 2 Diplomacy tags; 12 credits |
| Defection | Diplomacy | city | The nearest player pays 2 Ambassadors or discards 2 cards. Sway 1 toward Discord. | 2 Diplomacy tags; 12 credits |
| Smear Campaign | Diplomacy | city, station | 2 Unity comes off the Unity track. The nearest player pays 2 energy or discards 2 cards. | 3 Ambassadors; 12 credits |
| Propaganda Blitz | Diplomacy | coastal city | Sway 3 toward Discord. The adversary gains 3 compute. | 3 Ambassadors; 12 credits |
| Sabotage | Cyber | station | The nearest player loses 4 tokens from their cards. The adversary gains 6 compute. | 2 Cyber tags; 12 credits |
| Insider Access | Cyber + Diplomacy | station | 2 Agents are created here. The nearest player pays 3 research cubes or discards 2 cards. | 3 Ambassadors; 12 credits |
| Wiper Malware | Cyber | station | The nearest player discards 3 cards. | 3 research cubes; 12 credits |
| Cryptojacking | Cyber | station, factory | The adversary gains 6 compute. The nearest player pays 2 research cubes or discards 1 card. | 2 Cyber tags; 12 credits |
| Backdoor | Cyber | station | A Botnet lands on the station here. | 3 insight; 12 credits |
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
| Total War | Security | base | 2 adversary Troops land here and march at once. | 4 energy; 17 credits |
| Siege | Security | factory | The factory here is destroyed. 1 Fallout lands on its hex. | 4 Ambassadors; 17 credits |
| Manhunt | Security | base, city | The nearest player pays 3 Ambassadors or discards 3 cards. 1 adversary Troop lands on the base here. | 4 energy; 17 credits |
| Conscription | Security + Industry | base | 1 adversary Troop lands here. The nearest player pays 2 chips or discards 2 cards. | 4 energy; 17 credits |
| Casus Belli | Security | base | This base's front has a second war step next round. 2 Fallout land next to here. | 4 Ambassadors; 17 credits |
| Overrun | Diplomacy | city | The city here is corrupted. | 4 insight; 17 credits |
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
| Factory Occupation | Industry + Security | factory | The adversary corrupts the factory here, even a player's. The adversary gains 5 compute. | 4 energy; 17 credits |
| Trade Blockade | Industry | coastal city, factory | 4 Agents are created here. The nearest player pays 3 chips or discards 3 cards. | 4 energy; 17 credits |
| Grid Collapse | Power | city, coastal city | The nearest player destroys 1 of their Nodes closest to it and pays 4 energy or discards 3 cards. | 5 research cubes; 17 credits |
| Power Rationing | Power | city | The nearest player pays 4 energy or discards 3 cards. | 4 Ambassadors; 17 credits |
| Thermal Runaway | Power | station | The nearest player pays 6 research cubes or discards 3 cards. The adversary gains 4 compute. | 3 Power tags; 17 credits |
| Overload | Power | factory, station | The nearest player destroys 2 of their Nodes closest to it. The adversary gains 6 compute. | 3 Power tags; 17 credits |
| Garrison State | Universal | city, base | 3 Agents are created on the city here. 1 adversary Troop lands on the base here. | 4 insight; 17 credits |
| Shock Doctrine | Universal | city, factory | The nearest player pays 2 Ambassadors or discards 2 cards. 3 Unity comes off the Unity track. | 4 energy; 17 credits |
| Full Spectrum Assault | Universal | station | A Botnet lands on the station here. A tier-1 threat is dealt into this country (it executes next round). | 4 Ambassadors; 17 credits |
| Last Ultimatum | Universal | city, base | A tier-2 threat is dealt into this country (it executes next round). The nearest player destroys 1 of their Nodes closest to it. | 5 research cubes; 17 credits |
| Iron Curtain | Universal | factory, city | 1 adversary Troop lands on the factory here. 2 Unity comes off the Unity track. | 4 energy; 17 credits |
| War Economy | Security + Industry | factory, base | An adversary factory is built on the free industrial zone nearest here. 1 adversary Troop lands on the factory here. | 4 energy; 17 credits |
| Cascade Failure | Power + Cyber | station, city | A Botnet lands on the station here. The nearest player pays 3 energy or discards 3 cards. | 4 insight; 17 credits |

## Projects

Each project resolves once, at once, in the compute step. Its compute in play is its per-board cost times the countries.

| Project | Tags | Compute a board (in play: 1 / 2 / 3 / 4 countries) | Text |
|---|---|---|---|
| Disinformation | Diplomacy | 9 (9 / 18 / 27 / 36) | Sway 8 toward Discord. |
| Backdoor Research | Cyber | 7 (7 / 14 / 21 / 28) | 4 adversary research cubes onto each open megaproject. |
| Rolling Blackouts | Power | 6 (6 / 12 / 18 / 24) | Until the end of next round, cities make no energy. |
| Botnet Swarm | Cyber | 12 (12 / 24 / 36 / 48) | Each country, twice: a botnet spreads to its lowest-numbered city, research station or factory next to one of its botnets (no botnet there: its lowest-numbered research station). |
| False Flag | Universal | 7 (7 / 14 / 21 / 28) | Each country: an extra tier-2 threat, any tag. |
| Night Raid | Security | 8 (8 / 16 / 24 / 32) | Each country: 1 Agent onto each of two different hexes holding a player node (those holding the most Agents first; ties: lowest-numbered). 2 Agents on your node's hex destroy your node in the next Corrupt beat. |
| Containment Breach | Power | 7 (7 / 14 / 21 / 28) | Each player pays 1 antimatter or discards 3 cards. |
| Crackdown | Security | 6 (6 / 12 / 18 / 24) | Each country: every player Ambassador on its hex holding the most (ties: lowest-numbered) goes back to its owner's supply. |
| Lab Purge | Cyber | 9 (9 / 18 / 27 / 36) | Each open megaproject loses 4 player research cubes, from the player with the most on it first. |
| Asset Seizure | Industry | 11 (11 / 22 / 33 / 44) | Each country: its neutral factory with the most Agents on or next to it (ties: lowest-numbered) is corrupted; with none neutral, its player-owned factory picked the same way turns neutral. |
| Botnet Conscripts | Cyber | 9 (9 / 18 / 27 / 36) | Each country: its lowest-numbered botnet becomes 3 Troops on its hex. |
| Purge Lists | Diplomacy | 5 (5 / 10 / 15 / 20) | Each player loses 1 Ambassador (from their supply, else one of theirs on the board). |
| Dirty Bomb | Power | 10 (10 / 20 / 30 / 40) | Each country: 1 Fallout lands next to each of its two lowest-numbered corrupted cities. |
| Lab Seizure | Cyber | 6 (6 / 12 / 18 / 24) | Corrupt: research stations. |
| Slander Campaign | Diplomacy | 7 (7 / 14 / 21 / 28) | Pull 4 Sway back off the Unity track. |
| Coastal Raids | Security | 7 (7 / 14 / 21 / 28) | Rally 3, each country: coastal cities. |
| Plant Seizure | Industry | 7 (7 / 14 / 21 / 28) | Corrupt: factories. |
| Sabotage Teams | Power | 10 (10 / 20 / 30 / 40) | Each country: the hardened facility with the most Agents on or next to it (ties: lowest-numbered) loses its hardening. Until the end of next round, players make no energy from their nodes in upkeep. |
| Mob Rule | Diplomacy | 14 (14 / 28 / 42 / 56) | Rally 3, each country: cities. Sway 3 toward Discord. |
| Data Heist | Cyber | 12 (12 / 24 / 36 / 48) | Each player removes 3 tokens from their cards (their choice of cards). Each player pays 8 credits or discards 2 cards. |
| Hostile Takeover | Industry | 15 (15 / 30 / 45 / 60) | Rally 3, each country: factories. Corrupt: factories. |
| Station Coup | Cyber | 19 (19 / 38 / 57 / 76) | Each country: its neutral research station with the most Agents on or next to it (ties: lowest-numbered) is corrupted; with none neutral, its player-owned station picked the same way turns neutral. Then each country: 1 Fallout lands next to its lowest-numbered corrupted research station. |
| War Plants | Industry | 19 (19 / 38 / 57 / 76) | Each country: the adversary builds a factory on its free industrial zone next to an Agent, else its lowest-numbered (none free: 2 Agents on one of its factories). Every adversary factory makes 2 Troops on its hex (a player Ambassador on it stops them). |
| Recruitment Drive | Diplomacy | 20 (20 / 40 / 60 / 80) | Income: +1 Agent a country a round. Rally 2, each country: cities. |
| Propaganda Network | Universal | 18 (18 / 36 / 54 / 72) | Income: +1 compute a country a round. Sway 6 toward Discord. |
| Infiltration Drive | Diplomacy | 22 (22 / 44 / 66 / 88) | Rally 3, each country: cities. Corrupt: cities (1 fewer). Then each country: 1 Fallout lands next to its lowest-numbered corrupted city. |
| Accelerated Timetable | Universal | 20 (20 / 40 / 60 / 80) | Discard the top face-down card of the adversary's event deck, if any: the round cap comes a round sooner. Each country: an extra tier-2 threat, any tag. |
| Grand Offensive | Universal | 22 (22 / 44 / 66 / 88) | Each country rallies its Agent income now, to this round's Rally. Corrupt: cities. Corrupt: factories. |
