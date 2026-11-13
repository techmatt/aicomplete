# 204: every source of Sway, by narrative (r90)

Matt's ruling (2026-11-12): Unity is trust and cooperation; Discord is fear, force and collateral harm; an act that
tells neither is neutral (Sway 0, amendment 9256). Content sways carry their tag as `narrative` on the op
(`apply_sway.py` writes them); standard actions carry it in `scripts/design_check.json`'s rows; rule sways (configs)
are listed here. "r89" is the side and amount before the pass.

## Standard actions and rules (configs)

| source | r89 | story | r90 | why |
|---|---|---|---|---|
| Liberate a city / a facility (`free_city`, `free_facility`) | Unity 5 / 2 | trust | Unity 5 / 2 | ambassadors massed until the agents leave: the people choose you |
| Clear a corruption marker (`clear_unity`) | Unity 2 | trust | Unity 2 | cleaning up corruption restores trust |
| Envoy (hold 1 Discord off) | Unity 1 (hold-off) | trust | unchanged | diplomacy calms the fear |
| Take down a botnet (`botnet`, every botnet removal) | Unity 1 | neutral | **0** | a network takedown tells no story of trust or of force (9256) |
| Removing a troop (`troop`) | 0 | force | 0 | unchanged: force would lean Discord, but troops are the deadly threat (201), so no tax |
| The antimatter action (`antimatter_discord`) | off (Discord 1 at r88) | — | **gone** | no cheap antimatter (ruling); the code and its two tunables are deleted (9256) |
| A corrupted city's energy diverted (`divert_discord`) | Discord 1 | fear | unchanged | the adversary feeds on a captured city |
| A facility destroyed in war (`destruction_discord`) | Discord 1 | collateral | unchanged | war's collateral harm |
| Land corruption (`blight_discord`) | Discord 1 a round | fear | unchanged | a poisoned land frightens |
| The adversary board's drip | Discord 0 (the Militarist) | fear | unchanged | — |

## Player cards

| card | r89 | story | r90 | why |
|---|---|---|---|---|
| Shared Grid Accord | Unity 2 | cooperation | Unity 2 | a shared grid |
| Open Assembly | Unity 3 | trust | Unity 3 | open government |
| Manifesto Leak | Unity 3 | trust | Unity 3 | truth exposed |
| Back-Channel Talks | Unity 2 | cooperation | Unity 2 | negotiation |
| Coalition Charter | Unity 6 | cooperation | Unity 6 | a coalition founded |
| Planetary Referendum | Unity 8 | trust | Unity 8 | the people vote |
| Diplomatic Expulsion | Unity 1 | trust | Unity 1 | agents expelled by law, not force |
| **Martial Cordon** | Unity 2 | force | **Discord 1** | a cordon by soldiers is force |
| Truth Commission | Unity 2 | trust | Unity 2 | reconciliation |
| **The Nuclear Option** | Discord 1 | collateral | **Discord 2** | the most extreme act on the table: the ruling's "an extreme act raises Discord" |
| Peace Conference | Unity 8 | cooperation | Unity 8 | peace |
| Citywide Microgrid | Unity 2 | cooperation | Unity 2 | shared power for a city |
| Holographic Assembly Hall | Unity 2 | trust | Unity 2 | open assembly |
| Mass Defection | Unity 2 | trust | Unity 2 | agents choose your side |
| **Kettle Operation** | Unity 2 | force | **Discord 1** | kettling a crowd is force |
| Rally the Allies | Unity 2 | cooperation | Unity 2 | allies |
| **Surgical Strike** | Unity 1 | force | **Discord 1** | a strike is force, however precise |
| Open-Source Uprising | Unity 1 | cooperation | Unity 1 | the many cooperate |
| **Kinetic Interdiction** (ultimate) | none | force | **Discord 1** | kinetic removal of troops and 6 agents is force |
| **Annihilation Lance** (new) | — | collateral | **Discord 2** | the big stack-clear: an antimatter strike's collateral harm |
| **Fusion Commons** (new) | — | cooperation | **Unity 3** | an antimatter breakthrough shared with the cities |

## Unity and Discord cards, the map, the adversary

| source | r89 | story | r90 | why |
|---|---|---|---|---|
| Volunteer Corps (Unity card) | Unity 1 a site | cooperation | unchanged | volunteers |
| Citizens' Assembly (Unity card) | Unity 1 a corrupted site | trust | unchanged | the citizens speak |
| Biomia's Delta Seawall (completion, converters) | Unity 1 / 1 a cube / 2 a use | cooperation | unchanged | a seawall built together |
| Horizon, Onyx, Biomia: an adversary cube at completion | Discord 1 | fear | unchanged | the adversary owns the work |
| War Drums, Blitzkrieg (events) | Discord 1 | fear | unchanged | — |
| Disinformation, Propaganda Network (projects) | Discord 5 / 2 | fear | unchanged | — |
| Call-Up, Provocation, Agitators, Defection, Propaganda Blitz (tier-1 threats) | Discord 2, 2, 2, 1, 3 | fear | unchanged | — |
| Casus Belli, Deepfake Wave, Propaganda Ministry (tier-2 threats) | Discord 3, 3, 4 | fear | unchanged | — |

Moved: 6 of 47 sources (Martial Cordon, Kettle Operation, Surgical Strike, Kinetic Interdiction, The Nuclear Option,
the botnet takedown), under the 30% that would need Matt's reading first.

## Unity cards against amendment 9154 (helping, not clearing; tier III no panacea)

Of 23 Unity cards, 9 help (income, keeps, ambassadors, energy), 2 counter threats, 3 forbid an adversary step, 9 clear
(agents, troops, botnets, markers, facility cubes). None clears a city at any strength, so the one-card limit holds.
Three tier-III strengths reached every hex within 2 of any ambassador or node: a near-panacea. Each now stays by the
corrupted cities, as its I and II do:

| card | III at r89 | III at r90 | why |
|---|---|---|---|
| Civil Defense | each country, up to 6 agents within 2 of an ambassador or node | up to 4 on or next to a corrupted city | III swept whole countries |
| Town Hall Accord | each player, up to 6 agents from one hex within 2 | up to 5 from one hex on or next to a corrupted city | the same, one hex a player |
| Reconciliation Panel | up to 3 agents from every hex within 2 | up to 3 from every hex on or next to a corrupted city | every hex near you at once was a panacea |

Kept: the helping cards as they are (they already lead the deck), Decommission, Peacekeepers and Disarmament Talks
(botnets and troops, the deadly threat, 201), Network Cleanup and Reclamation (markers and facility cubes, not cities).
