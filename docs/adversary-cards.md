# Adversary cards (generated)

*Written by `python scripts/content/load_adversary.py` from `drafts/adversary/` (the source); edit the drafts, not this page. The rules are in [rules/threats.md](rules/threats.md) and [rules/adversary.md](rules/adversary.md).*

## The Militarist (board)

Project slots: Military, Open, Open.

- **Troops where the Protector stands.** Each build: in each country where the Protector has a node, 1 adversary troop lands.
- **Warmongering.** Each round, a military base with an adversary troop on or next to it makes 1 extra corp troop, even past its cap; at most 1 extra a base in each production, whatever makes it.
- **Build.** Each board: create 2 adversary ambassadors on the lowest-numbered corrupted city.
- **Difficulty.** Militarist, harder: every event that lands troops lands 1 more in each country, and the adversary gains 4 compute for each threat that executes uncountered.

## Event deck

| Card | Stage | Text |
|---|---|---|
| Probing Strikes | 1 | Each country: a Military 1 threat. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered coastal cities. Corrupt cities. |
| Factory Drive | 1 | Each country: Military 1 or Economic 1. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered factories. Each board: a neutral factory opens on its lowest-numbered free industrial zone. Corrupt factories. |
| Long Patrols | 1 | Each country: Military 1 or Cyber 1. The adversary gains 3 compute. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors. Each board: a neutral research station opens on its lowest-numbered empty station site. Corrupt research stations. |
| War Drums | 1 | Each country: Military 1 or Influence 1. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on the lowest-numbered factory. Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors. Sway 1 toward Discord. Corrupt cities. |
| Signal Jamming | 1 | Each country: Military 1 or Grid 1. The adversary gains 2 compute. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered coastal cities. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered botnet hexes. Corrupt cities. |
| Mobilization | 2 | Each country: a Military 2 threat. The adversary gains 3 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered coastal cities. Each board: create 1 adversary ambassador on the lowest-numbered factory. Corrupt cities. |
| Station Raids | 2 | Each country: Military 2 or Cyber 2. The adversary gains 4 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Each board: create 1 adversary ambassador on the lowest-numbered factory. Corrupt research stations. |
| Fifth Column | 2 | Each country: Influence 2 or Cyber 2. The adversary gains 3 compute. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Each board: create 1 adversary ambassador on the lowest-numbered factory. Corrupt cities (1 fewer). |
| Border War | 2 | Each country: a Military 2 threat. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 2 adversary ambassadors on the lowest-numbered factory. Each player with a node on or next to a front's base destroys 1 of those nodes. Corrupt factories. |
| Supply Lines | 2 | Each country: Military 1 or Economic 1, and Military 2 or Economic 2. The adversary gains 4 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 3 lowest-numbered factories. Corrupt factories. |
| Forced March | 2 | Each country: any tier-2 threat. The adversary gains 3 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered factories. Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors. Every adversary troop takes 1 more step this round. Corrupt cities. |
| Total Mobilization | 3 | Each country: a Military 2 threat, and Economic 2 or Grid 2. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Each board: create 1 adversary ambassador on each of the 3 lowest-numbered factories. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Remove 2 Unity from the Unity track. Corrupt factories. Corrupt cities. |
| Blitzkrieg | 3 | Each country: any tier-2 threat, and Military 2 or Influence 2. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Each board: create 2 adversary ambassadors on the lowest-numbered factory. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Sway 1 toward Discord. Corrupt cities. Corrupt land. |
| Occupation | 3 | Each country: Military 2 or Cyber 2, and an Influence 2 threat. The adversary gains 5 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Corrupt research stations. Corrupt cities (1 fewer). |
| Naval Blockade | 3 | Each country: Economic 2 or Grid 2, and Military 2 or Cyber 2. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Each board: create 2 adversary ambassadors on each of the 2 lowest-numbered coastal cities. Corrupt cities. Corrupt land. |

## Threats, tier 1

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Orbital Strike | Military | base | 2 adversary troops land here. | 3 energy; 12 credits |
| Commandeer | Military | base, factory | The nearest player pays 3 energy or discards 3 cards. The adversary gains 2 compute. | 3 ambassadors; 12 credits |
| Supply Raid | Military + Economic | factory | 1 adversary troop lands here. The nearest player pays 3 chips or discards 1 cards. | 2 Industry tags; 12 credits |
| Eviction | Military | city | The nearest player pays 2 ambassadors or discards 2 cards. The adversary gains 4 compute. | 2 Security tags; 12 credits |
| Call-Up | Military | base | 1 adversary troop lands here. Sway 2 toward Discord. | 3 ambassadors; 12 credits |
| Provocation | Influence + Military | base | A tier-1 threat is dealt into this country (it executes next round). Sway 2 toward Discord. | 3 ambassadors; 12 credits |
| Agitators | Influence | city | 3 adversary ambassadors are created here. Sway 2 toward Discord. | 2 Diplomacy tags; 12 credits |
| Defection | Influence | city | The nearest player pays 2 ambassadors or discards 2 cards. Sway 1 toward Discord. | 2 Diplomacy tags; 12 credits |
| Smear Campaign | Influence | city, station | 2 Unity comes off the Unity track. The nearest player gets 4 less insight next round. | 3 ambassadors; 12 credits |
| Propaganda Blitz | Influence | coastal city | Sway 3 toward Discord. The adversary gains 3 compute. | 3 ambassadors; 12 credits |
| Sabotage | Cyber | station | The nearest player loses 4 tokens from their cards. The adversary gains 6 compute. | 2 Cyber tags; 12 credits |
| Insider Access | Cyber + Influence | station | 2 adversary ambassadors are created here. The nearest player pays 3 research cubes or discards 2 cards. | 3 ambassadors; 12 credits |
| Wiper Malware | Cyber | station | The nearest player discards 3 cards. | 3 research cubes; 12 credits |
| Cryptojacking | Cyber | station, factory | The adversary gains 6 compute. The nearest player pays 2 research cubes or discards 1 cards. | 2 Cyber tags; 12 credits |
| Backdoor | Cyber | station | A botnet lands on the station here. | 6 insight; 12 credits |
| Requisition | Economic | factory, city | The nearest player pays 5 chips or discards 2 cards. The adversary gains 3 compute. | 2 Industry tags; 12 credits |
| Shakedown | Economic + Influence | city | Theft: the nearest player pays 5 chips or discards 2 cards; the adversary gains 4 compute. | 3 ambassadors; 12 credits |
| Embargo | Economic | factory | 2 adversary ambassadors are created here. The nearest player pays 2 energy or discards 2 cards. | 6 chips; 12 credits |
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
| Total War | Military | base | 3 adversary troops land here and march at once. | 4 energy; 17 credits |
| Siege | Military | factory | The factory here is destroyed. The adversary gains 4 compute. | 4 ambassadors; 17 credits |
| Manhunt | Military | base, city | The nearest player pays 3 ambassadors or discards 3 cards. 1 adversary troop lands on the base here. | 4 energy; 17 credits |
| Conscription | Military + Economic | base | 2 adversary troops land here. The nearest player pays 4 chips or discards 2 cards. | 4 energy; 17 credits |
| Casus Belli | Military | base | This base's front has a second war step next round. Sway 3 toward Discord. | 4 ambassadors; 17 credits |
| Overrun | Influence | city | The city here is corrupted. | 7 insight; 17 credits |
| Insurrection | Influence | city | 4 adversary ambassadors are created here. 3 Unity comes off the Unity track. | 3 Diplomacy tags; 17 credits |
| Deepfake Wave | Influence + Cyber | city, station | Sway 3 toward Discord. 2 Unity comes off the Unity track. | 4 ambassadors; 17 credits |
| Turncoats | Influence | city | The nearest player pays 4 ambassadors or discards 3 cards. The adversary gains 3 compute. | 3 Diplomacy tags; 17 credits |
| Propaganda Ministry | Influence | city, coastal city | Sway 4 toward Discord. The adversary gains 4 compute. | 3 Diplomacy tags; 17 credits |
| Skimming Operation | Economic + Influence | coastal city | Theft: the nearest player pays 8 chips or discards 3 cards; the adversary gains 6 compute. | 4 ambassadors; 17 credits |
| Dark Cities | Grid + Influence | city, coastal city | The nearest player destroys 1 of their nodes closest to it. 3 Unity comes off the Unity track. | 3 Power tags; 17 credits |
| Station Seizure | Cyber | station | The station here is corrupted, even a player's. The adversary gains 4 compute. | 3 Cyber tags; 17 credits |
| Sleeper Cells | Cyber | station | A botnet lands on the station here. 2 adversary ambassadors are created here. | 3 Cyber tags; 17 credits |
| Purge Protocols | Cyber | station | The nearest player destroys 1 of their nodes closest to it and discards 3 cards. | 5 research cubes; 17 credits |
| Data Breach | Cyber + Economic | station, factory | The nearest player pays 5 research cubes or discards 3 cards. The adversary gains 5 compute. | 7 chips; 17 credits |
| Zero Day | Cyber | station | The adversary gains 10 compute. The nearest player turns 1 building face down until the end of next round. | 5 research cubes; 17 credits |
| Asset Freeze | Economic | factory | The nearest player turns 2 buildings face down until the end of next round and discards 2 cards. | 4 energy; 17 credits |
| Mass Production | Economic | factory | An adversary factory is built here. The adversary gains 5 compute. | 7 chips; 17 credits |
| Factory Occupation | Economic + Military | factory | The adversary seizes the factory here, even a player's. The adversary gains 5 compute. | 4 energy; 17 credits |
| Trade Blockade | Economic | coastal city, factory | 4 adversary ambassadors are created here. The nearest player pays 6 chips or discards 3 cards. | 4 energy; 17 credits |
| Grid Collapse | Grid | city, coastal city | The nearest player destroys 1 of their nodes closest to it and pays 4 energy or discards 3 cards. | 5 research cubes; 17 credits |
| Power Rationing | Grid | city | The nearest player pays 4 energy or discards 3 cards. | 4 ambassadors; 17 credits |
| Thermal Runaway | Grid | station | The nearest player pays 6 research cubes or discards 3 cards. The adversary gains 4 compute. | 3 Power tags; 17 credits |
| Overload | Grid | factory, station | The nearest player destroys 2 of their nodes closest to it (never their last). The adversary gains 6 compute. | 3 Power tags; 17 credits |
| Garrison State | Universal | city, base | 3 adversary ambassadors are created on the city here. 2 adversary troops land on the base here. | 7 insight; 17 credits |
| Shock Doctrine | Universal | city, factory | The nearest player pays 2 ambassadors or discards 2 cards. 3 Unity comes off the Unity track. | 4 energy; 17 credits |
| Full Spectrum Assault | Universal | station | A botnet lands on the station here. A tier-1 threat is dealt into this country (it executes next round). | 4 ambassadors; 17 credits |
| Last Ultimatum | Universal | city, base | A tier-2 threat is dealt into this country (it executes next round). The nearest player destroys 1 of their nodes closest to it. | 5 research cubes; 17 credits |
| Iron Curtain | Universal | factory, city | 2 adversary troops land on the factory here. 2 Unity comes off the Unity track. | 4 energy; 17 credits |
| War Economy | Military + Economic | factory, base | An adversary factory is built here. 1 adversary troop lands on the factory here. | 4 energy; 17 credits |
| Cascade Failure | Grid + Cyber | station, city | A botnet lands on the station here. The nearest player pays 3 energy or discards 3 cards. | 7 insight; 17 credits |

## Projects

| Project | Tags | Compute per player | Resolves | Text |
|---|---|---|---|---|
| Raiding Parties | Military | 4 | once | Every player loses 12 credits. |
| War Footing | Military | 7 | once | Every military base fills with corp troops to its cap now. Sway 3 toward Discord. |
| Purge the Unaligned | Military | 7 | once | Every player loses 2 ambassadors (from their supply, else ones on the board). 1 troop lands on a factory (else a city) in each country. Remove 1 Unity from the Unity track. |
| Armored Columns | Military | 10 | once | In each country, 3 troops land on factories (else cities). |
| Arms Race | Military | 11 | ongoing | Ongoing: each round, every military base makes 1 extra corp troop, even past its cap (at most 1 extra a base in each production, whatever makes it); sway 1 toward Discord. |
| Declaration of War | Military | 18 | ongoing | In each country, 3 troops land on factories (else cities); sway 3 toward Discord. Ongoing: each round, the front with the most corp troops fights twice. |
| Disinformation | Influence | 5 | once | Sway 5 toward Discord. Every player loses 2 ambassadors. |
| Recruitment Drive | Influence | 11 | ongoing | Ongoing: every round, each board: create 1 adversary ambassador on each of the 3 lowest-numbered clean cities. |
| Infiltration Drive | Influence | 18 | once | Each board: create 3 adversary ambassadors on the lowest-numbered clean city. Corrupt cities (1 fewer). Every player loses an ambassador. Sway 3 toward Discord. |
| Data Heist | Cyber | 6 | once | Every player loses 5 tokens from their cards and gets 3 less insight next round. |
| Botnet Swarm | Cyber | 6 | once | Each board: create 3 adversary ambassadors on each of the 2 lowest-numbered botnet hexes. |
| Station Coup | Cyber | 9 | once | Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Then in each country, players choose a research station that is not the adversary's; it flips to the adversary, but a player's holds unless 2 adversary ambassadors and no player ambassador are on it (those 2 return to the supply as it flips). If it holds, 1 adversary ambassador lands on it (or cancels a player ambassador there). |
| Market Crash | Economic | 5 | once | Every player gets 9 fewer credits of income next round and loses all their chips. |
| Shadow Factories | Economic | 7 | once | The adversary builds a factory in the country with the most free industrial zones (tie: the first country), on a free zone next to an adversary ambassador, else the lowest-numbered one (none: 2 of its ambassadors land on a factory). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered factories. |
| Capital Flight | Economic | 8 | once | Every player's income falls by 6 credits for the rest of the game. |
| Rolling Blackouts | Grid | 6 | once | Every player loses 2 energy. Next round, cities make no energy and research stations produce nothing. |
| Grid Capture | Grid | 9 | ongoing | Every player loses 2 energy. Ongoing: every round, every player loses 1 energy. |
| False Flag | Universal | 6 | once | Deal 1 extra tier-2 threat in each country, any tag. Sway 2 toward Discord. |
| Grand Offensive | Universal | 10 | once | The adversary builds now, then Corrupt cities; 1 troop lands on a factory (else a city) in each country; sway 3 toward Discord. |
| Propaganda Network | Universal | 14 | ongoing | Ongoing: every round, sway 3 toward Discord, remove 2 Unity from the Unity track. Each board: create 2 adversary ambassadors on the lowest-numbered corrupted city. |
