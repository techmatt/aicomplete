# Adversary cards (generated)

*Written by `python scripts/content/load_adversary.py` from `drafts/adversary/` (the source); edit the drafts, not this page. The rules are in [rules/threats.md](rules/threats.md) and [rules/adversary.md](rules/adversary.md).*

## The Militarist (board)

Project slots: Military, Open, Open.

- **Troops where the Protector stands.** Each build: in each country where the Protector has a node, 1 adversary troop lands.
- **Warmongering.** Each round, a military base with an adversary troop on or next to it makes 1 extra corp troop, even past its cap; at most 1 extra a base each round, whatever makes it.
- **Build.** Each board: create 2 adversary ambassadors on the lowest-numbered corrupted city.
- **Difficulty.** Militarist, harder: every event that lands troops lands 1 more in each country.

## Event deck

| Card | Stage | Text |
|---|---|---|
| Probing Strikes | 1 | Each country: a Military 1 threat. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered coastal cities. Corrupt cities. |
| Factory Drive | 1 | Each country: Military 1 or Economic 1. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered factories. Each board: a neutral factory opens on its lowest-numbered free industrial zone. Corrupt factories. |
| Long Patrols | 1 | Each country: Military 1 or Cyber 1. The adversary gains 3 compute. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors. Each board: a neutral research station opens on its lowest-numbered empty station site. Corrupt research stations. |
| War Drums | 1 | Each country: Military 1 or Influence 1. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Sway 1 toward Discord. Each board: create 1 adversary ambassador on the lowest-numbered factory. Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors. Corrupt cities. |
| Signal Jamming | 1 | Each country: Military 1 or Grid 1. The adversary gains 2 compute. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered coastal cities. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered botnet hexes. Corrupt cities. |
| Mobilization | 2 | Each country: a Military 2 threat. The adversary gains 3 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered coastal cities. Each board: create 1 adversary ambassador on the lowest-numbered factory. Corrupt cities. |
| Station Raids | 2 | Each country: Military 2 or Cyber 2. The adversary gains 4 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Each board: create 1 adversary ambassador on the lowest-numbered factory. Corrupt research stations. |
| Fifth Column | 2 | Each country: Influence 2 or Cyber 2. The adversary gains 3 compute. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Each board: create 1 adversary ambassador on the lowest-numbered factory. Corrupt cities (1 fewer). |
| Border War | 2 | Each country: a Military 2 threat. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each player destroys 1 of their nodes. Each board: create 2 adversary ambassadors on the lowest-numbered factory. Corrupt factories. |
| Supply Lines | 2 | Each country: Military 1 or Economic 1. The adversary gains 4 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 3 lowest-numbered factories. Corrupt factories. |
| Forced March | 2 | Each country: any tier-2 threat. The adversary gains 3 compute. In each country, 1 troop lands on a factory (else a city). Every adversary troop takes 1 more step this round. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered factories. Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors. Corrupt cities. |
| Total Mobilization | 3 | Each country: a Military 3 threat, and Economic 2 or Grid 2. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Remove 2 Unity from the Unity track. Each board: create 1 adversary ambassador on each of the 3 lowest-numbered factories. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Corrupt factories. Corrupt cities. |
| Blitzkrieg | 3 | Each country: any tier-3 threat, and Military 1 or Influence 1. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Sway 1 toward Discord. Each board: create 2 adversary ambassadors on the lowest-numbered factory. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Corrupt cities. Corrupt land. |
| Occupation | 3 | Each country: Military 3 or Cyber 3, and an Influence 2 threat. The adversary gains 5 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Corrupt research stations. Corrupt cities (1 fewer). |
| Naval Blockade | 3 | Each country: Economic 3 or Grid 3, and Military 1 or Cyber 1. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Each board: create 2 adversary ambassadors on each of the 2 lowest-numbered coastal cities. Corrupt cities. Corrupt land. |

## Threats, tier 1

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Orbital Strike | Military | base | 1 adversary troop lands here and stays (it doesn't march) until removed. The adversary gains 2 compute. Sway 1 toward Discord. | 7 credits; 1 nodes near it; 2 Security tags |
| Commandeer | Military | base, factory | Each player near it pays 2 energy or discards 2 cards. | 2 energy; 7 credits |
| Supply Raid | Military + Economic | factory | Each player near it loses 8 credits. The adversary gains 2 compute. | 7 credits; 2 Security tags; 2 Industry tags |
| Eviction | Military | city | Each player near it pays 8 credits or loses 2 ambassadors. | 6 credits; 1 nodes near it |
| Call-Up | Military | base | In its country: create 2 adversary ambassadors on the lowest-numbered factory. Sway 1 toward Discord. | 7 credits; 1 nodes near it; 2 Security tags |
| Agitators | Influence | city | In its country: create 1 adversary ambassador on each of the 2 lowest-numbered clean cities. Sway 1 toward Discord. | 7 credits; 2 ambassadors; 2 Diplomacy tags |
| Defection | Influence | city | Each player near it loses an ambassador. Sway 1 toward Discord. | 6 credits; 1 ambassadors; 2 Diplomacy tags |
| Smear Campaign | Influence | city, station | Each player near it gets 2 less insight next round. Remove 1 Unity from the Unity track. | 5 insight; 7 credits; 2 Diplomacy tags |
| Propaganda Blitz | Influence | coastal city | Sway 2 toward Discord. Each player near it discards a card. | 7 credits; 2 ambassadors; 2 Diplomacy tags |
| Border Provocation | Influence + Military | base | Deal 1 extra tier-1 threat in this country; it executes next round. The adversary gains 2 compute. | 6 credits; 2 ambassadors; 2 Security tags |
| Sabotage | Cyber | station | Each player near it loses 3 tokens from their cards. The adversary gains 2 compute. | 5 insight; 1 research cubes; 2 Cyber tags |
| Insider Access | Cyber + Influence | station | In its country: create 1 adversary ambassador on each of the 2 lowest-numbered player research stations. The adversary gains 2 compute. | 5 insight; 8 credits; 2 ambassadors |
| Wiper Malware | Cyber | station | Each player near it discards 2 cards or gets 2 less insight next round. | 5 insight; 1 research cubes; 2 Cyber tags |
| Cryptojacking | Cyber | station, factory | Each player near it loses 1 research cube; for each who has none, the adversary gains 4 compute. | 5 insight; 6 credits; 2 Cyber tags |
| Backdoor | Cyber | station | The adversary gains 2 compute on every visible project. | 5 insight; 1 research cubes; 2 Cyber tags |
| Requisition | Economic | factory, city | Each player near it discards 2 cards or pays 8 credits. | 6 credits; 2 Industry tags |
| Shakedown | Economic + Influence | city | The player near it with the most credits (ties: lowest seat) loses 8 credits. The adversary gains 4 compute. | 8 credits; 2 ambassadors; 2 Industry tags |
| Embargo | Economic | factory | Sites on or next to it can't be claimed next round. The adversary gains 3 compute. | 6 credits; 2 Industry tags |
| Credit Crunch | Economic | coastal city | Each player near it gets 8 fewer credits of income next round. | 7 credits; 1 energy; 2 Industry tags |
| Chip Shortage | Economic | factory | Each player near it loses 3 chips or 8 credits. | 6 credits; 2 energy; 2 Industry tags |
| Blackout | Grid + Cyber | city | Cities on or next to it produce no energy next round. | 2 energy; 7 credits; 2 Power tags |
| Load Shedding | Grid | city, station | Players near it can't use Actions that cost energy next round. | 2 energy; 7 credits; 2 Power tags |
| Power Surcharge | Grid | station | Each player near it loses 2 energy or 10 credits. | 2 energy; 7 credits; 2 Power tags |
| Cooling Failure | Grid | station | Each player near it makes no research cubes next round. | 2 energy; 7 credits; 2 Power tags |
| Grid Tap | Grid | city | The player near it with the most credits (ties: lowest seat) loses 2 energy. The adversary gains 4 compute. | 2 energy; 8 credits; 2 Power tags |

## Threats, tier 2

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Total War | Military | base | 2 troops land on the base here and march at once. | 12 credits; 3 Security tags; 2 nodes near it |
| Siege | Military | factory | 1 troop lands on the factory or zone here. Each player near it destroys 1 of their nodes near it (never their last). | 12 credits; 3 energy; 2 nodes near it |
| Manhunt | Military | base, city | Each player near it pays 14 credits or destroys 2 of their nodes near it (never their last). | 10 credits; 3 Security tags |
| Conscription | Military + Economic | base | For 3 rounds, the corp base here makes 1 extra troop each round, even past its cap. | 12 credits; 3 energy; 3 Security tags |
| Casus Belli | Military | base | Next round, the fronts next to this base fight twice. Sway 2 toward Discord. | 12 credits; 3 Security tags; 2 nodes near it |
| Overrun | Influence | city | In its country: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Corrupt cities (1 fewer). | 12 credits; 3 ambassadors; 3 Diplomacy tags |
| Insurrection | Influence | city | Create 3 adversary ambassadors here. Sway 2 toward Discord. | 11 credits; 3 ambassadors; 3 Diplomacy tags |
| Deepfake Wave | Influence + Cyber | city, station | Sway 2 toward Discord. Remove 1 Unity from the Unity track. Each player near it gets 2 less insight next round. | 3 ambassadors; 11 credits; 6 insight |
| Turncoats | Influence | city | In its country: create 1 adversary ambassador on each of the 2 lowest-numbered player factories. Each player near it loses 2 ambassadors. | 11 credits; 3 Diplomacy tags |
| Propaganda Ministry | Influence | city, coastal city | In its country: create 1 adversary ambassador on each of the 2 lowest-numbered clean cities. Corrupt cities. Sway 1 toward Discord. | 10 credits; 2 ambassadors; 3 Diplomacy tags |
| Station Seizure | Cyber | station | The station here is corrupted (a player's holds unless 2 adversary and no player ambassadors are on it: an adversary ambassador lands). The adversary gains 3 compute. | 2 research cubes; 12 credits; 3 Cyber tags |
| Sleeper Cells | Cyber | station | A botnet lands on the station here, else on the nearest piece without one (players choose among equals). The adversary gains 3 compute. | 2 research cubes; 12 credits; 3 Cyber tags |
| Purge Protocols | Cyber | station | Each player near it discards their most expensive card in hand and loses 3 tokens from their cards. | 2 research cubes; 12 credits |
| Data Breach | Cyber + Economic | station, factory | Each player near it loses 2 research cubes or gets 3 less insight next round. The adversary gains 2 compute. | 11 credits; 3 energy; 6 insight |
| Zero Day | Cyber | station | The adversary gains 8 compute. Each player near it turns 1 of their buildings face down until the end of next round (it does nothing; its tags still count). | 11 credits; 2 research cubes; 3 Cyber tags |
| Asset Freeze | Economic | factory | Each player near it loses 12 credits or discards 3 cards. | 12 credits; 3 energy; 3 Industry tags |
| Mass Production | Economic | factory | The adversary builds a factory on the free industrial zone nearest this threat's token. The adversary gains 3 compute. | 11 credits; 3 energy; 3 Industry tags |
| Factory Occupation | Economic + Military | factory | The adversary gains 3 compute. 1 troop lands on the factory or zone here and stays: a factory makes nothing while it stands; a zone gives 3 more compute. | 12 credits; 2 nodes near it; 3 Industry tags |
| Skimming Operation | Economic + Influence | coastal city | The player near it with the most credits (ties: lowest seat) loses 15 credits. The adversary gains 3 compute. | 3 ambassadors; 11 credits |
| Trade Blockade | Economic | coastal city, factory | Sites on or next to it can't be claimed through next round, and the next adversary build on or next to it happens twice. | 11 credits; 2 energy; 3 Industry tags |
| Grid Collapse | Grid | city, coastal city | Cities on or next to it produce no energy for 2 rounds. The adversary gains 3 compute. | 3 energy; 12 credits; 3 Power tags |
| Power Rationing | Grid | city | Each player near it loses 3 energy or 12 credits. | 10 credits; 2 energy; 3 Power tags |
| Thermal Runaway | Grid | station | Research stations on or next to it make no research cubes for 2 rounds. Each player near it gets 2 less insight next round. | 3 energy; 2 research cubes; 3 Power tags |
| Dark Cities | Grid + Influence | city, coastal city | In its country: create 1 adversary ambassador on each of the 3 lowest-numbered cities on or next to it. Sway 1 toward Discord. | 3 energy; 3 ambassadors |
| Overload | Grid | factory, station | Each player near it destroys 1 of their nodes near it (never their last) and loses 2 energy. | 3 energy; 12 credits; 3 Power tags |
| Garrison State | Universal | base, city | In its country: create 2 adversary ambassadors on the lowest-numbered city on or next to it. 1 troop lands on the base here. The adversary gains 2 compute. | 12 credits; 3 ambassadors |
| Shock Doctrine | Universal | city, factory | Each player near it loses 6 credits and 1 energy. Sway 2 toward Discord. | 3 ambassadors; 3 insight + 1 ambassadors |
| Full Spectrum Assault | Universal | station | A botnet lands on the station here, else on the nearest piece without one (players choose among equals). Deal 1 extra tier-1 threat in this country; it executes next round. | 12 credits; 8 insight; 3 Security tags |

## Threats, tier 3

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Last Ultimatum | Universal | city, base | Each player near it pays 20 credits or loses 2 nodes near it (never their last). Deal 1 extra tier-2 threat in this country; it executes next round. | 15 credits; 4 ambassadors; 3 Security tags; 3 Diplomacy tags |
| Capital Siphon | Universal | factory, station | The player near it with the most credits (ties: lowest seat) loses 15; the adversary gains 4 compute. Each player near it discards 1 card. | 15 credits; 4 energy; 3 research cubes; 3 Cyber tags |
| Iron Curtain | Universal | factory, city | 1 troop lands on the factory or zone here. Each player near it loses 2 energy. Remove 2 Unity from the Unity track. | 16 credits; 4 energy; 3 Security tags; 3 Power tags |
| Spearhead | Military | base | 3 troops land on the base here and march at once, 2 steps. | 15 credits; 4 energy; 3 Security tags; 3 nodes near it |
| Scorched Earth | Military | factory | The factory here is destroyed, with the nodes on its hex (never their last). On an empty zone, 2 troops land there instead. | 16 credits; 4 energy; 3 Security tags |
| General Mobilization | Military | base | Next round, the fronts next to this base fight twice. Its corp makes 2 more troops here now, past its cap. Sway 1 toward Discord. | 14 credits; 3 energy; 3 Security tags; 6 credits + 2 energy |
| War Economy | Military + Economic | factory, base | An adversary factory lands on the zone here, else the nearest free zone, with 1 troop on it. Each player near it loses 1 energy. | 16 credits; 4 energy; 3 Security tags; 3 Industry tags |
| Regime Change | Influence | city | The city here is corrupted: nodes on and next to it are destroyed (never their last), and player ambassadors waiting there are lost. | 27 credits; 7 ambassadors; 5 Diplomacy tags; 14 credits + 3 ambassadors |
| Lab Takeover | Cyber | station | The station here is corrupted, even a player's; the adversary gains 4 compute. | 16 credits; 3 research cubes; 3 Cyber tags; 6 credits + 2 insight |
| Hostile Takeover | Economic | factory | The adversary seizes the factory here, even a player's; on an empty zone, it builds one there. | 16 credits; 4 energy; 3 Industry tags |
| Cascade Failure | Grid + Cyber | station, city | A botnet lands on the station here, else on the nearest piece without one (players choose among equals). Next round, cities and stations on or next to it produce nothing. | 4 energy; 16 credits; 3 Power tags; 3 research cubes |

## Projects

| Project | Tags | Compute per player | Resolves | Text |
|---|---|---|---|---|
| Raiding Parties | Military | 4 | once | Every player loses a node or pays 10 credits (never their last node). |
| War Footing | Military | 7 | once | Every corp base fills to its troop cap now. Sway 1 toward Discord. |
| Purge the Unaligned | Military | 7 | once | Every player loses an ambassador or a node (never their last node). 1 troop lands on a factory in each country. |
| Armored Columns | Military | 10 | once | In each country, 2 troops land on factories. |
| Arms Race | Military | 11 | ongoing | Ongoing: each round, every military base makes 1 more troop. |
| Declaration of War | Military | 18 | ongoing | In each country, 2 troops land on factories. Ongoing: each round, the front with the most corp troops fights twice. |
| Disinformation | Influence | 5 | once | Sway 4 toward Discord. Every player loses an ambassador. |
| Recruitment Drive | Influence | 11 | ongoing | Ongoing: every round, each board: create 1 adversary ambassador on each of the 2 lowest-numbered clean cities. |
| Infiltration Drive | Influence | 18 | once | Corrupt cities (1 fewer). |
| Data Heist | Cyber | 6 | once | Every player loses 3 tokens from their cards and gets 2 less insight next round. |
| Botnet Swarm | Cyber | 6 | once | Each board: create 2 adversary ambassadors on each of the 2 lowest-numbered botnet hexes. |
| Station Coup | Cyber | 9 | once | In each country, a research station is corrupted. A player's holds unless 2 adversary and no player ambassadors are on it: an adversary ambassador lands. |
| Market Crash | Economic | 5 | once | Every player gets 6 fewer credits of income next round and loses all their chips. |
| Shadow Factories | Economic | 7 | once | The adversary builds a factory in the country with the most free industrial zones. None: 2 of its ambassadors land on a factory. |
| Capital Flight | Economic | 8 | once | Every player's income falls by 4 credits for the rest of the game. |
| Rolling Blackouts | Grid | 6 | once | Next round, cities make no energy and research stations make no research cubes. |
| Grid Capture | Grid | 9 | ongoing | Ongoing: every round, every player loses 1 energy. |
| False Flag | Universal | 6 | once | Deal 1 extra tier-2 threat in each country, any tag. |
| Grand Offensive | Universal | 10 | once | The adversary builds now, then Corrupt cities; sway 2 toward Discord. |
| Propaganda Network | Universal | 14 | ongoing | Ongoing: every round, sway 2 toward Discord, remove 1 Unity from the Unity track. Each board: create 1 adversary ambassador on the lowest-numbered corrupted city. |
