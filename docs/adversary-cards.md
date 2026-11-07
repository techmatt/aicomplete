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
| Border War | 2 | Each country: a Military 2 threat. The adversary gains 2 compute. In each country, 1 troop lands on a factory (else a city). Each player with a node on or next to a front's base destroys 1 of those nodes. Each board: create 2 adversary ambassadors on the lowest-numbered factory. Corrupt factories. |
| Supply Lines | 2 | Each country: Military 1 or Economic 1. The adversary gains 4 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 1 adversary ambassador on each of the 3 lowest-numbered factories. Corrupt factories. |
| Forced March | 2 | Each country: any tier-2 threat. The adversary gains 3 compute. In each country, 1 troop lands on a factory (else a city). Every adversary troop takes 1 more step this round. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered factories. Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors. Corrupt cities. |
| Total Mobilization | 3 | Each country: a Military 3 threat, and Economic 2 or Grid 2. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Remove 2 Unity from the Unity track. Each board: create 1 adversary ambassador on each of the 3 lowest-numbered factories. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Corrupt factories. Corrupt cities. |
| Blitzkrieg | 3 | Each country: any tier-3 threat, and Military 1 or Influence 1. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Sway 1 toward Discord. Each board: create 2 adversary ambassadors on the lowest-numbered factory. Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Corrupt cities. Corrupt land. |
| Occupation | 3 | Each country: Military 3 or Cyber 3, and an Influence 2 threat. The adversary gains 5 compute. In each country, 1 troop lands on a factory (else a city). Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Corrupt research stations. Corrupt cities (1 fewer). |
| Naval Blockade | 3 | Each country: Economic 3 or Grid 3, and Military 1 or Cyber 1. The adversary gains 4 compute. In each country, 2 troops land on factories (else cities). Each board: create 2 adversary ambassadors on each of the 2 lowest-numbered coastal cities. Corrupt cities. Corrupt land. |

## Threats, tier 1

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Orbital Strike | Military | base | 1 adversary troop lands here and stays (it doesn't march) until removed. The adversary gains 3 compute. Sway 2 toward Discord. | 3 energy; 2 Security tags; 15 credits |
| Commandeer | Military | base, factory | 1 adversary troop lands here and stands until removed. The adversary gains 3 compute. Each player near it pays 2 energy or discards 2 cards. | 3 ambassadors; 2 Security tags; 15 credits |
| Supply Raid | Military + Economic | factory | 1 adversary troop lands here and stands until removed. The adversary gains 3 compute. Each player near it loses 2 energy. | 4 chips; 2 Industry tags; 15 credits |
| Eviction | Military | city | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it loses an ambassador. | 3 energy; 2 Security tags; 15 credits |
| Call-Up | Military | base | 1 adversary troop lands here and stands until removed. This base makes 1 extra corp troop now, past its cap. The adversary gains 3 compute. | 3 ambassadors; 2 Security tags; 15 credits |
| Agitators | Influence | city | Create 1 adversary ambassador here. The adversary gains 3 compute. Sway 2 toward Discord. | 3 ambassadors; 2 Diplomacy tags; 15 credits |
| Defection | Influence | city | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it loses an ambassador. Sway 1 toward Discord. | 8 insight; 2 Diplomacy tags; 15 credits |
| Smear Campaign | Influence | city, station | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it gets 3 less insight next round. Remove 1 Unity from the Unity track. | 3 ambassadors; 2 Diplomacy tags; 15 credits |
| Propaganda Blitz | Influence | coastal city | Create 1 adversary ambassador here. The adversary gains 3 compute. Sway 2 toward Discord. Each player near it discards a card. | 8 insight; 3 ambassadors; 15 credits |
| Border Provocation | Influence + Military | base | 1 adversary troop lands here and stands until removed. The adversary gains 3 compute. Deal 1 extra tier-1 threat in this country; it executes next round. | 3 energy; 2 Security tags; 15 credits |
| Sabotage | Cyber | station | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it loses 3 tokens from their cards. | 2 research cubes; 2 Cyber tags; 15 credits |
| Insider Access | Cyber + Influence | station | Create 1 adversary ambassador here and 1 on each of its country's 2 lowest-numbered player stations. The adversary gains 3 compute. | 8 insight; 2 research cubes; 15 credits |
| Wiper Malware | Cyber | station | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it discards 2 cards or gets 2 less insight next round. | 2 research cubes; 2 Cyber tags; 15 credits |
| Cryptojacking | Cyber | station, factory | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it loses 1 research cube; for each who has none, 4 more compute. | 8 insight; 2 Cyber tags; 15 credits |
| Backdoor | Cyber | station | Create 1 adversary ambassador here. The adversary gains 3 compute, then 1 compute on every visible project. | 8 insight; 2 research cubes; 15 credits |
| Requisition | Economic | factory, city | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it discards 2 cards or pays 3 chips. | 3 energy; 2 Industry tags; 15 credits |
| Shakedown | Economic + Influence | city | Create 1 adversary ambassador here. The player near it with the most credits (ties: lowest seat) loses 4 chips. The adversary gains 4 compute. | 3 ambassadors; 2 Industry tags; 15 credits |
| Embargo | Economic | factory | Create 1 adversary ambassador here. Sites on or next to it can't be claimed next round. The adversary gains 3 compute. | 4 chips; 2 Industry tags; 15 credits |
| Credit Crunch | Economic | coastal city | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it turns 1 building face down until the end of next round. | 4 chips; 2 Industry tags; 15 credits |
| Chip Shortage | Economic | factory | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it loses 3 chips or discards 2 cards. | 3 energy; 2 Industry tags; 15 credits |
| Blackout | Grid + Cyber | city | Create 1 adversary ambassador here. The adversary gains 3 compute. Cities on or next to it produce no energy next round. | 2 research cubes; 2 Power tags; 15 credits |
| Load Shedding | Grid | city, station | Create 1 adversary ambassador here. The adversary gains 3 compute. Players near it can't use Actions that cost energy next round. | 3 energy; 2 Power tags; 15 credits |
| Power Surcharge | Grid | station | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it loses 2 energy or discards 2 cards. | 2 research cubes; 2 Power tags; 15 credits |
| Cooling Failure | Grid | station | Create 1 adversary ambassador here. The adversary gains 3 compute. Each player near it makes no research cubes next round. | 3 energy; 2 Power tags; 15 credits |
| Grid Tap | Grid | city | Create 1 adversary ambassador here. The player near it with the most credits (ties: lowest seat) loses 2 energy. The adversary gains 4 compute. | 2 research cubes; 2 Power tags; 15 credits |

## Threats, tier 2

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Total War | Military | base | 3 troops land on the base here and march at once. Each player near it destroys 1 of their nodes near it (never their last). | 4 ambassadors; 3 Security tags; 23 credits |
| Siege | Military | factory | The factory here is destroyed with the nodes on its hex (never a last node). 2 troops land here and stay. | 4 ambassadors; 4 energy; 23 credits |
| Manhunt | Military | base, city | Each player near it loses 2 ambassadors. 1 troop lands here. | 4 energy; 3 Security tags; 23 credits |
| Conscription | Military + Economic | base | For 3 rounds, the corp base here makes 1 extra troop each round, even past its cap. Remove 2 Unity from the Unity track. | 6 chips; 3 Security tags; 23 credits |
| Casus Belli | Military | base | 2 troops land on the base here. Next round, the fronts next to it fight twice. Sway 2 toward Discord. | 4 ambassadors; 3 Security tags; 23 credits |
| Overrun | Influence | city | Create 3 adversary ambassadors here. Corrupt cities (1 fewer). | 10 insight; 3 Diplomacy tags; 23 credits |
| Insurrection | Influence | city | Create 3 adversary ambassadors here. Remove 2 Unity from the Unity track. Sway 1 toward Discord. | 10 insight; 3 Diplomacy tags; 23 credits |
| Deepfake Wave | Influence + Cyber | city, station | Sway 2 toward Discord. Remove 2 Unity from the Unity track. Each player near it gets 3 less insight next round. | 3 research cubes; 4 ambassadors; 23 credits |
| Turncoats | Influence | city | In its country: create 2 adversary ambassadors on each of the 2 lowest-numbered player factories. Each player near it loses 2 ambassadors. | 10 insight; 3 Diplomacy tags; 23 credits |
| Propaganda Ministry | Influence | city, coastal city | In its country: create 1 adversary ambassador on each of the 2 lowest-numbered clean cities. Corrupt cities (1 fewer). Sway 2 toward Discord. | 10 insight; 3 Diplomacy tags; 23 credits |
| Station Seizure | Cyber | station | The station here is corrupted, even a player's. The adversary gains 4 compute. | 10 insight; 3 Cyber tags; 23 credits |
| Sleeper Cells | Cyber | station | A botnet lands on the station here (else the nearest piece without one). Create 2 adversary ambassadors here. | 10 insight; 3 Cyber tags; 23 credits |
| Purge Protocols | Cyber | station | Each player near it discards their most expensive card in hand, loses 3 tokens from their cards and destroys 1 node near it (never their last). | 3 research cubes; 10 insight; 23 credits |
| Data Breach | Cyber + Economic | station, factory | Each player near it loses 2 research cubes and gets 3 less insight next round. The adversary gains 5 compute. | 6 chips; 4 energy; 23 credits |
| Zero Day | Cyber | station | The adversary gains 8 compute. Each player near it turns 1 of their buildings face down until the end of next round (tags still count). | 3 research cubes; 3 Cyber tags; 23 credits |
| Asset Freeze | Economic | factory | Each player near it turns 2 of their buildings face down until the end of next round (tags still count) and discards 2 cards. | 4 energy; 3 Industry tags; 23 credits |
| Mass Production | Economic | factory | The adversary builds a factory on the free industrial zone nearest this threat's token. The adversary gains 5 compute. | 6 chips; 3 Industry tags; 23 credits |
| Factory Occupation | Economic + Military | factory | The adversary gains 4 compute. 1 troop lands on the factory or zone here and stays: a factory makes nothing while it stands; a zone gives 4 more compute. | 4 energy; 3 Industry tags; 23 credits |
| Skimming Operation | Economic + Influence | coastal city | Each player near it gets 4 less chips income next round. The adversary gains 5 compute. | 4 ambassadors; 4 energy; 23 credits |
| Trade Blockade | Economic | coastal city, factory | Create 3 adversary ambassadors here. Sites on or next to it can't be claimed for 2 rounds. | 6 chips; 4 energy; 23 credits |
| Grid Collapse | Grid | city, coastal city | Cities on or next to it produce no energy for 2 rounds. Each player near it destroys 1 of their nodes near it (never their last). | 3 research cubes; 3 Power tags; 23 credits |
| Power Rationing | Grid | city | Each player near it loses 3 energy and turns 1 of their buildings face down until the end of next round (tags still count). | 3 research cubes; 4 ambassadors; 23 credits |
| Thermal Runaway | Grid | station | Stations on or next to it make no research cubes for 2 rounds. Each player near it gets 3 less insight next round. The adversary gains 4 compute. | 4 energy; 3 Power tags; 23 credits |
| Dark Cities | Grid + Influence | city, coastal city | In its country: create 1 adversary ambassador on each of the 3 lowest-numbered cities on or next to it. Remove 2 Unity from the Unity track. | 10 insight; 3 Power tags; 23 credits |
| Overload | Grid | factory, station | Each player near it destroys 1 of their nodes near it (never their last). The adversary gains 5 compute. | 4 energy; 3 Power tags; 23 credits |
| Garrison State | Universal | base, city | In its country: create 3 adversary ambassadors on the lowest-numbered city on or next to it. 2 troops land on the base here. | 2 energy + 2 research cubes; 10 insight; 23 credits |
| Shock Doctrine | Universal | city, factory | Each player near it loses 1 ambassador. Remove 2 Unity from the Unity track. Sway 2 toward Discord. | 2 research cubes + 3 chips; 4 energy; 23 credits |
| Full Spectrum Assault | Universal | station | A botnet lands on the station here (else the nearest piece without one). Deal 1 extra tier-1 threat in this country. The adversary gains 4 compute. | 5 insight + 2 research cubes; 4 ambassadors; 23 credits |

## Threats, tier 3

| Threat | Tags | Locations | If not countered | Counter (one of) |
|---|---|---|---|---|
| Last Ultimatum | Universal | city, base | Each player near it destroys 1 of their nodes near it (never their last) and loses an ambassador. Deal 1 extra tier-2 threat here; it executes next round. | 3 energy + 3 research cubes; 8 chips; 4 Diplomacy tags; 33 credits |
| Capital Siphon | Universal | factory, station | The adversary gains 6 compute. Each player near it discards 2 cards and loses 2 tokens from their cards. | 5 chips + 3 research cubes; 12 insight; 4 Cyber tags; 33 credits |
| Iron Curtain | Universal | factory, city | 2 troops land on the factory or zone here. Each player near it loses 2 energy. Remove 3 Unity from the Unity track. | 3 ambassadors + 5 chips; 4 Security tags; 4 research cubes; 33 credits |
| Spearhead | Military | base | 4 troops land on the base here and march at once, 2 steps. Each player near it destroys 1 of their nodes near it (never their last). | 5 energy; 5 ambassadors; 4 Security tags; 33 credits |
| Scorched Earth | Military | factory | The factory here is destroyed with the nodes on its hex (never their last); on an empty zone, 3 troops land instead. Each player near it loses 3 energy. | 5 ambassadors; 8 chips; 4 Security tags; 33 credits |
| General Mobilization | Military | base | Next round, the fronts next to this base fight twice. Its corp makes 3 more troops now, past its cap; 2 troops land here. Sway 2 toward Discord. | 5 ambassadors; 3 energy + 5 chips; 4 Security tags; 33 credits |
| War Economy | Military + Economic | factory, base | An adversary factory lands on the zone here, else the nearest free zone, with 2 troops on it. The adversary gains 5 compute. | 8 chips; 5 energy; 4 Industry tags; 33 credits |
| Regime Change | Influence | city | The city here is corrupted, with the nodes on and next to it (never their last). Each player near it loses an ambassador. Remove 2 Unity from the Unity track. | 12 insight; 8 chips; 4 Diplomacy tags; 33 credits |
| Lab Takeover | Cyber | station | The station here is corrupted, even a player's; the adversary gains 6 compute. Each player near it gets 4 less insight next round. | 4 research cubes; 8 chips; 4 Cyber tags; 33 credits |
| Hostile Takeover | Economic | factory | The adversary seizes the factory here, even a player's; on an empty zone, it builds one there. Each player near it loses 4 chips. The adversary gains 5 compute. | 5 energy; 4 Industry tags; 12 insight; 33 credits |
| Cascade Failure | Grid + Cyber | station, city | A botnet lands on the station here, else the nearest piece without one. Next round, cities and stations on or next to it produce nothing. Each player near it loses 3 energy. | 4 research cubes; 4 Power tags; 8 chips; 33 credits |

## Projects

| Project | Tags | Compute per player | Resolves | Text |
|---|---|---|---|---|
| Raiding Parties | Military | 4 | once | Every player loses 12 credits. |
| War Footing | Military | 7 | once | Every corp base fills to its troop cap now. Sway 3 toward Discord. |
| Purge the Unaligned | Military | 7 | once | Every player loses 2 ambassadors (from their supply, else ones on the board). 1 troop lands on a factory in each country. Remove 1 Unity from the Unity track. |
| Armored Columns | Military | 10 | once | In each country, 3 troops land on factories. |
| Arms Race | Military | 11 | ongoing | Ongoing: each round, every military base makes 1 more troop; sway 1 toward Discord. |
| Declaration of War | Military | 18 | ongoing | In each country, 3 troops land on factories; sway 3 toward Discord. Ongoing: each round, the front with the most corp troops fights twice. |
| Disinformation | Influence | 5 | once | Sway 5 toward Discord. Every player loses 2 ambassadors. |
| Recruitment Drive | Influence | 11 | ongoing | Ongoing: every round, each board: create 1 adversary ambassador on each of the 3 lowest-numbered clean cities. |
| Infiltration Drive | Influence | 18 | once | Each board: create 3 adversary ambassadors on the lowest-numbered clean city. Corrupt cities (1 fewer). Every player loses an ambassador. Sway 3 toward Discord. |
| Data Heist | Cyber | 6 | once | Every player loses 5 tokens from their cards and gets 3 less insight next round. |
| Botnet Swarm | Cyber | 6 | once | Each board: create 3 adversary ambassadors on each of the 2 lowest-numbered botnet hexes. |
| Station Coup | Cyber | 9 | once | Each board: create 1 adversary ambassador on each of the 2 lowest-numbered research stations. Then in each country, a research station is corrupted (a player's holds unless 2 adversary and no player ambassadors are on it). |
| Market Crash | Economic | 5 | once | Every player gets 9 fewer credits of income next round and loses all their chips. |
| Shadow Factories | Economic | 7 | once | The adversary builds a factory in the country with the most free industrial zones (none: 2 of its ambassadors land on a factory). Each board: create 1 adversary ambassador on each of the 2 lowest-numbered factories. |
| Capital Flight | Economic | 8 | once | Every player's income falls by 6 credits for the rest of the game. |
| Rolling Blackouts | Grid | 6 | once | Every player loses 2 energy. Next round, cities make no energy and research stations make no research cubes. |
| Grid Capture | Grid | 9 | ongoing | Every player loses 2 energy. Ongoing: every round, every player loses 1 energy. |
| False Flag | Universal | 6 | once | Deal 1 extra tier-2 threat in each country, any tag. Sway 2 toward Discord. |
| Grand Offensive | Universal | 10 | once | The adversary builds now, then Corrupt cities; 1 troop lands on a factory in each country; sway 3 toward Discord. |
| Propaganda Network | Universal | 14 | ongoing | Ongoing: every round, sway 3 toward Discord, remove 2 Unity from the Unity track. Each board: create 2 adversary ambassadors on the lowest-numbered corrupted city. |
