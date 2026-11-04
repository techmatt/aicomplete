# 106 Security section proposal

26 cards (27 today; 7 cut, 6 new): tier 1 has 19 (11 programs, 6 buildings, 2 events) and tier 2 has 7 (4 programs, 1
building, 2 events). That is 15 programs, 7 buildings and 4 events (review added Quick Reaction Force, the assigned
Security reuse, as the seventh building). The median printed cost is 10 (3 to 42). The big cards are Multiyear Defense
Budget at 24 (tier 1), Homeland Security Act at 18 and Peacekeeping Mission at 42 (both tier 2).

**The engine:** Security pays for fighting troops. Security Contractor HQ pays 10 credits for each troop removed. Six
feeders remove troops: Interceptor Battery (energy to troops, the Protector's engine), two Agents trigger buildings,
Joint Task Force (a teammate's Security play), and the programs and events (Bounty Board, Peacekeeper Rotation, Force
Protection Order, Counterinsurgency Campaign, Breach and Clear, Total Containment, Kinetic Interdiction). Flat and
per-tag income moved to programs.

**Pricing basis.** Pay = credits + 2.5 × insight + keep (2.5 tier 1, 6.5 tier 2) + other costs at `values.json`. Get =
the effect at `values.json` (remove_troops at 7 a troop × `troops_removed`: 1 = 7, 2 = 10.4, 3 = 12.25, 4 = 16.6, 6 =
25.2; a troop in reach 11; feed_agents 2; tag 1 + 0.5 each more). Income uses a 4-round horizon on tier 1 and 2.5 on tier
2, with energy × 4.5 × 0.8. Engines are priced at the Security seat's fire rate, which is the Protector (4 slots, energy
income): about 1 Security card and 2 Security tags a round, about 3 tags of any kind a round, and 1 to 1.5 troops removed
a round once fed. The effect ledger reads the triggers at its generic rates (1.5 plays, 30% tagged, fires_troop 0.3), so
its `fair_credits` sits below these prices for Agency Field Station (5.7), Security Contractor HQ (10.7) and Bounty
Board (5.8). That gap is the engine premium the brief asks for.

**Supply caps (for every troop loop).** Troops on the board cap all removal, and so cap HQ's pay. Plays a round, cards
in hand and insight cap every play trigger. No card here adds tokens on a `place_tokens` event, and neither threshold
places tokens (`Content::check`).

| id | status | reasoning | value check (pay vs get, when, for whom) | loop and cap |
|---|---|---|---|---|
| security_contractor_hq | changed (cost 9 to 12) | The demo: fighting troops pays 10 credits a troop. It is the hub every feeder below serves. | Pay 12+2.5+2.5 = 17. Get, at the Protector's 1-1.5 troops a round from round 3: 10-15 a round × 4 = 40-60. At the generic 0.3 a round it is 12, so it is a seat card. | Credits buy feeders, feeders remove troops, troops pay credits. Capped by the troops on the board and by plays a round (Interceptor fires once a round). |
| agency_field_station | changed (rework) | The Agents token building: 1 Agent per Security tag on each card you play. At 3 Agents, remove 2 troops in one country. It is also the tier-1 Security trigger engine. | Pay 10+2.5+2.5 = 15. Get, at 2 Security tags a round: 0.67 fires × 10.4 = 7 a round × 4 = 28, plus HQ's 0.67 × 1.48 × 10 = 9.9 a round. The generic seat gets about 6. Strongest from rounds 2-6. | Tags give Agents, Agents remove troops, HQ pays credits for more Security cards. Capped by plays and troops on the board. The threshold places no tokens. |
| interceptor_battery | changed (3 to 8, removes 2 not 1) | Energy into troops: the Protector's energy income becomes HQ pay. | Pay 8+5 = 13. Get (10.4-4.5) × 4 × 0.7 = 16.5, and with HQ +14.8 a use. | Once a round (an Action), paid in energy. |
| joint_task_force | new | The teammate trigger: when another player plays a Security card, remove 1 troop in one country. It turns the table's Security plays into HQ pay. | Pay 8+2.5+2.5 = 13. Get, at 0.5-0.9 teammate Security plays a round: 3.5-6.3 a round × 4 = 14-25, plus HQ 5-9 a round. Best mid-game in a 3-4 seat team. | Only others' plays fire it, so it cannot loop on its own. Capped by their plays and the troops on the board. |
| garrison_command | changed (jam 3 to 5, cost 4 to 6) | Typed counter Action building, kept. Jam 3 at 0.7 netted below 0. | Pay 6+5 = 11. Get, for the Protector facing military threats: 5 × 4 × 0.7 = 14. The generic seat (× typed 0.7) gets 9.8. | Once a round. |
| quick_reaction_force | new | The assigned Security reuse (review): "Action: use another of your buildings' Actions again this round" (paying its cost again). Needs 2 Security tags. On the Protector it doubles Interceptor Battery: 1 more energy into 2 more troops, and HQ pays for them. | Pay 9+2.5+2.5 = 14. Get at the table 5 × 0.7 × 4 + 1 = 15 (+1). With Interceptor Battery and HQ: (10.4 - 4.5) + 14.8 = 20.7 a use, × 0.7 × 4 = 58. Rounds 3-8. | Energy → troops → HQ credits → cards. Capped by once a round (an Action), energy, troops on the board; never a reuse of a reuse (`reuse`'s rule). |
| defense_appropriations | new | Per-tag income as a program: raise your credit income by 1 per Security tag on your cards (production_per_tag). Needs 3, so it counts 4 or more. Review: cost 12 to 10 (it netted 0 at its minimum count; now the same price as Diplomacy's Peace Dividend, the same shape). | Pay 10+2.5+2.5 = 15. Get 4-6 tags × 4 = 16-24, plus a tag 1: +2 to +10. A mid-game card for the Security seat. | None (one-off). |
| multiyear_defense_budget | new | The TM-scale "cost big, gain big income" program, where flat Security income now lives. | Pay 24+2.5+2.5 = 29. Get 8 × 4 = 32, plus 1. Rounds 1-4; any board, best on the Builder or Protector. | None. |
| hardened_bunker_complex | changed (building to program) | Bunker-style flat income moved to a program: raise energy income by 1 and credit income by 2. The Harden action passive is dropped: a program holds none. | Pay 16+5 = 21. Get 14.4 + 8 + 1 = 23.4. Protector early. | None. |
| counterinsurgency_campaign | new | A big troop feeder with income: remove 3 troops in one country, raise your credit income by 2. | Pay 13+5 = 18. Get 12.25 + 8 + 1 = 21.25, plus HQ 17.5. | None. |
| recruit_operatives | changed (3 to 5 Agents, cost 6 to 9) | Feeds the Agents engines. Agents placed here wait for that card's next trigger to reach its threshold (the threshold reads a trigger of its own card). | Pay 9+2.5 = 11.5 (review: the keep was missing). Get 10 + 1 at feed_agents 2 (-0.5 at the table; a seat card). On Field Station 5 Agents is about 1.7 fires × 10.4 = 17 + 1, plus HQ. | It adds tokens, but nothing triggers on placing Agents. |
| bounty_board | changed (cost 3 to 10) | 1 troop in reach plus 1 credit per Security tag: a cheap feeder priced too low. | Pay 10+5 = 15. Get 11 + about 5 (Security seat) + 1 = 17, plus HQ 10. | None. |
| peacekeeper_rotation | changed (2 troops or 5 ambassadors in a hex, cost 3 to 12) | Was 3 ambassadors for 3 (it netted below 0). Now a troop feeder that still moves an ambassador. | Pay 12+5 = 17. Get 22 (2 troops in reach) + 1.5 + 1.5 = 25, plus HQ 20. | None. |
| force_protection_order | changed (cost 3 to 6) | Typed counter program, kept. Its troop rider (11) made it far underpriced. | Pay 6+5 = 11. Get 3.5 + 11 + 1 = 15.5. | None. |
| shared_threat_intel | changed (jam 4 to 6, cost 6 to 7) | Help card. It netted below 0 at jam 4. Review: the proposed untyped jam 8 out-jammed every typed counter (5) and is worth more than a full counter at the rates (8 vs counter_threat 7), against ruling 14; jam 6. | Pay 7+2.5 = 9.5. Get 6 + 3.75 (gift) + 1.5 = 11.25 (+1.75). | None. |
| diplomatic_expulsion | changed (4 to 8 ambassadors, cost 7 to 10) | Tier-1 ambassador sweep. It was break-even. | Pay 10+5 = 15. Get 16 + 4 + 1.5 = 21.5. | None. |
| threat_forecast_engine | changed (shield 2 to 3, cost 6 to 8) | Carries Science, so the niche check holds. It was break-even. | Pay 8+5 = 13. Get 2.5 + 12 + 2 = 16.5. | None. |
| breach_and_clear | changed (3 to 4 troops, cost 6 to 9) | Event troop burst for HQ, made bigger. | Pay 9+5 = 14. Get 16.6, plus HQ 23.7. | None. |
| hardpoint_lockdown | kept | Shelter and cordon at 5. | Pay 5+2.5 = 7.5. Get 4 + 5 = 9 (an event, no tag). | None. |
| agency_directorate | changed (rework) | Tier-2 Security trigger engine, built over a Security building: 1 Agent per tag of any kind on every card you play (`play_tag` with no tag). At 3 Agents, remove 3 troops in one country. Adds 2 Agents when played (the over-X one-off). | Pay 16+5+6.5 = 27.5, plus the building it covers. Get, at 3 tags a round: 1 fire × 12.25 × 2.5 = 30.6, plus the one-off 2 Agents (about 8), = 38.6, plus HQ 17.5 a round. Rounds 4-8. | Plays give Agents, troops pay credits, credits buy plays. Capped by plays a round, insight and troops on the board. The threshold places no tokens. |
| homeland_security_act | new | Big tier-2 per-tag program: raise your energy income by 2 and your credit income by 1 per Security tag on your cards. Needs 4, so it counts 5 or more. Review: cost 22 to 18 (it netted -1.5 at its minimum count). | Pay 18+5+6.5 = 29.5. Get 18 (energy) + 5-7 × 2.5 + 1.5 = 32-37 (+2.5 to +7.5). For a Security-heavy seat at rounds 4-6. | None. |
| deep_state_ledger | changed (building to program) | The flat tier-2 income moved to a program (credit income 4), plus 4 Agents for an engine card. | Pay 10+5+6.5 = 21.5. Get 10 + 8-14 (Agents, generic to Field Station) + 1.5 = 19.5-25.5. | It adds tokens; nothing triggers on placing Agents. |
| martial_cordon | changed (8 to 10 ambassadors, cost 12 to 14) | Tier-2 ambassador sweep. | Pay 14+5+6.5 = 25.5. Get 20 + 8 + 1.5 = 29.5. | None. |
| total_containment | changed (event to program, cost 8 kept) | It stays in play, so its tag counts for the per-tag cards. Kinetic Interdiction now covers the event burst. | Pay 8+5+6.5 = 19.5. Get 16.6 + 4 + 1 = 21.6, plus HQ 23.7. | None. |
| kinetic_interdiction | changed (energy 2 to 1, insight 2 to 1) | The antimatter bomb: 6 troops. It netted -3 at energy 2. | Pay 3+7+4.5+2.5+6.5 = 23.5. Get 25.2, plus HQ 36. | None. |
| peacekeeping_mission | kept | Security's liberate capstone. | It is priced at the capstone method (liberate 45, troops 3 × 3 = 9 = 54) against 42+2.5+6.5 = 51. | None. |
| sentry_drone_wing | cut | A trivial Agents building (harden 3 a token). | Agency Field Station covers the Agents role. Harden is left to the Protector's Bulwark. | |
| kill_team_roster | cut | Agents and troops by Action. | Agency Field Station and Agency Directorate do this as triggers. | |
| safehouse_network | cut | Shelter by token. | Hardpoint Lockdown and Total Containment cover it. | |
| mutual_defense_pact | cut | Shelter plus a 4-credit gift, below break-even (7 vs 7.5). | Hardpoint Lockdown covers shelter, Shared Threat Intel covers help. | |
| intrusion_countermeasures | cut | Drain 8 for 4 + 1 insight, below break-even. Drains are Cyber's job. | The Cyber section covers it. | |
| aegis_dome_array | cut | A tier-2 building with a one-off harden 3 (not allowed under the rule) and a weak passive (civic_shelter 1.6 a round). | Hardpoint Lockdown and Total Containment cover shelter. **Flag:** card-grammar.md names Aegis Dome Array's shield-with-country mark. | |
| sentinel_command | cut | A standing military counter that carried a one-off production 2. | Garrison Command, the protected Action building, covers it (now jam 5). **Flag:** the other tags' 13-credit auto_counter buildings lose their Security twin. If symmetry matters, restore it without the production at 11. | |

**Ideas carried:**
- Pay for fighting troops: HQ, with ten feeders.
- Agents per Security tag with a payoff: Agency Field Station.
- Per-tag income program: Defense Appropriations and Homeland Security Act.
- Tier-1 and tier-2 Security trigger engines: Agency Field Station and Agency Directorate.
- Teammate trigger: Joint Task Force.
- Bunker-style income moved to programs: Hardened Bunker Complex, Deep State Ledger and Multiyear Defense Budget.
- Security reuse: Quick Reaction Force (added in review; it was not carried). Its target is Interceptor Battery (a second 1 energy → 2 troops, firing HQ again) or Garrison Command, or any other tag's Action on the seat.

**Risks:**
- Agency Directorate's Effect, "add 1 Agent here per tag on it; At 3: remove 3 troops in one country", is near 90
  characters and may fail `cards:fit`. Agency Field Station's ("add 1 Agent here per Security tag on it; at 3, remove 2
  troops in one country") is about 107, well over the ~80 building box (review). If `cards:fit` refuses them, the fix is
  a shorter `play_tag` / `remove_troops` phrase in `face.rs`, not these cards.
- Garrison Command (jam 5, cost 6) now differs from Civic Watch and Cyber's Sentinel Mesh (jam 4, cost 4): an
  integrator call on one shape for the typed Action buildings (review).
- The 13-credit `auto_counter` capstones: Sentinel Command and Diplomacy's Civic Trust Bureau are cut, Cyber kept
  Autonomous Defense Grid (at 9, the one-off dropped). The brief's typed-counter set (two programs and an Action
  building) stands in both sections; the integrator decides whether the standing shape survives on every tag.
- If troops are scarce in play, the HQ engine starves. Read `remove_troop` fires per game at the smoke.
