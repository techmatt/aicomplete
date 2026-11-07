# 139 death audit (r58, the fresh read's games)

136's `death_audit.py` on every last-node loss of the fresh read (seeds 36584-36599 and 36648-36679, the three pairs), records saved by `death_runs.py` on the frozen binary. A cell is `d<D>_<pair>_<first seed of the range>`. d0 is the final build's (X0); d1 and d2 play the same under both freezes. "Discord: X" as a fatal source is a Discord card's Corrupt line taking the node on the hex it corrupts.

## d2 (46 losses)

| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |
|---|---|---|---|---|---|---|---|---|---|---|
| d2_bp_36584 | 36587 | 8 | 0 Builder | 2 2 1 0 | r6: +0 e0>0 c0 st1 / r7: +0 e0>0 c1 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #8 0, Cintamani #14 1; exposed 1/1 |
| d2_bp_36584 | 36591 | 8 | 0 Builder | 3 2 1 0 | r6: +0 e0>0 c0 st0 / r7: +0 e0>0 c11 st0 | 0 / 1 | corrupt | corruption | 1 | Cintamani #10 0; exposed 0/2 |
| d2_bp_36584 | 36593 | 5 | 0 Builder | 2 2 1 0 | r3: +0 e0>0 c0 st0 / r4: +0 e0>0 c4 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #10 0, Cintamani #2 0; exposed 2/2 |
| d2_bp_36584 | 36594 | 8 | 1 Protector | 3 4 3 0 | r7: +1 e2>0 c0 st2 / r8: +0 e2>0 (o2) c0 st2 | 0 / 1 | execute | threat: General Mobilization; threat: Scorched Earth | 3 |  |
| d2_bp_36584 | 36599 | 5 | 0 Builder | 2 2 2 0 | r4: +0 e0>0 c2 st0 / r5: +0 e0>0 c1 st0 | 0 / 1 | execute | threat: Thermal Runaway | 2 |  |
| d2_bp_36648 | 36648 | 4 | 0 Builder | 2 2 1 0 | r3: +0 e0>0 c0 st0 / r4: +0 e0>0 c1 st0 | 0 / 1 | execute | threat: Shock Doctrine | 1 |  |
| d2_bp_36648 | 36660 | 4 | 1 Protector | 1 2 3 0 | r3: +1 e5>3 c4 st0 / r4: +0 e5>0 (o5) c0 st0 | 1 / 1 | execute | threat: Overload | 3 |  |
| d2_bp_36648 | 36662 | 8 | 0 Builder | 1 1 1 0 | r6: +1 e9>4 (o5) c0 st1 / r7: +2 e5>0 c0 st2 | 0 / 1 | corrupt | corruption | 1 |  |
| d2_bp_36648 | 36669 | 4 | 0 Builder | 2 2 2 0 | r3: +0 e0>0 c0 st0 / r4: +0 e0>0 c6 st0 | 0 / 1 | execute | threat: Backdoor; threat: Wiper Malware | 2 |  |
| d2_bp_36648 | 36673 | 7 | 0 Builder | 1 1 1 0 | r6: +0 e0>0 c6 st0 / r7: +0 e0>0 c2 st0 | 0 / 1 | execute | threat: Dark Cities | 1 |  |
| d2_bp_36648 | 36677 | 5 | 0 Builder | 1 1 1 0 | r3: +0 e0>0 c3 st0 / r4: +0 e0>0 c4 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #10 0, Cintamani #2 0; exposed 1/1 |
| d2_br_36584 | 36590 | 6 | 1 Researcher | 2 2 2 0 | r5: +0 e0>0 c0 st0 / r6: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Lab Takeover | 2 |  |
| d2_br_36584 | 36593 | 4 | 0 Builder | 2 2 2 0 | r3: +0 e0>0 c2 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Purge Protocols; threat: Wiper Malware | 2 |  |
| d2_br_36584 | 36598 | 4 | 1 Researcher | 2 1 1 0 | r3: +0 e0>0 c0 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Siege | 1 |  |
| d2_br_36648 | 36648 | 5 | 0 Builder | 1 1 1 0 | r4: +0 e0>0 c1 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Garrison State | 1 |  |
| d2_br_36648 | 36649 | 9 | 0 Builder | 1 1 1 0 | r7: +0 e0>0 c2 st0 / r8: +2 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 1 |  |
| d2_br_36648 | 36650 | 5 | 0 Builder | 4 4 3 0 | r4: +0 e0>0 c0 st0 / r5: +1 e0>0 c0 st1 | 0 / 1 | execute | threat: Sleeper Cells; threat: Total War | 3 |  |
| d2_br_36648 | 36651 | 5 | 0 Builder | 2 2 2 0 | r4: +0 e0>0 c3 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Sleeper Cells | 2 |  |
| d2_br_36648 | 36652 | 4 | 1 Researcher | 2 2 2 0 | r3: +0 e0>0 c9 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Purge Protocols | 1 |  |
| d2_br_36648 | 36654 | 9 | 0 Builder | 2 4 1 0 | r7: +2 e3>0 c0 st2 / r8: +0 e0>0 c0 st2 | 0 / 1 | corrupt | corruption | 1 | Cintamani #10 0; exposed 1/4; run-up placements out(6/6 out),IN(6/8 out) |
| d2_br_36648 | 36658 | 6 | 0 Builder | 3 2 1 0 | r5: +0 e0>0 c0 st0 / r6: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Dark Cities | 1 |  |
| d2_br_36648 | 36660 | 5 | 1 Researcher | 2 2 2 0 | r4: +0 e0>0 c0 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Siege | 1 |  |
| d2_br_36648 | 36661 | 4 | 1 Researcher | 2 2 2 0 | r3: +0 e0>0 c8 st0 / r4: +0 e0>0 c5 st0 | 0 / 1 | execute | threat: Shock Doctrine | 1 |  |
| d2_br_36648 | 36662 | 8 | 0 Builder | 2 3 4 0 | r7: +1 e5>1 (o1) c4 st0 / r8: +1 e1>0 c0 st1 | 0 / 1 | execute | threat: Lab Takeover; threat: Spearhead | 3 |  |
| d2_br_36648 | 36663 | 8 | 1 Researcher | 1 3 4 0 | r7: +1 e12>0 (o11) c0 st0 / r8: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Capital Siphon; threat: General Mobilization | 3 |  |
| d2_br_36648 | 36665 | 5 | 0 Builder | 1 1 3 0 | r4: +2 e0>0 c0 st0 / r5: +1 e3>0 (o1) c0 st0 | 0 / 1 | execute | threat: Conscription; threat: Data Breach | 3 |  |
| d2_br_36648 | 36667 | 6 | 1 Researcher | 2 2 1 0 | r5: +1 e0>0 (o1) c0 q1 st0 / r6: +1 e5>0 (o3) c0 q1 st0 | 0 / 1 | execute | threat: Hostile Takeover | 2 |  |
| d2_br_36648 | 36668 | 8 | 1 Researcher | 1 1 3 0 | r7: +2 e9>0 (o8) c0 q3 st0 / r8: +1 e0>0 c0 st1 | 0 / 1 | execute | threat: Trade Blockade; threat: War Economy | 3 |  |
| d2_br_36648 | 36669 | 5 | 0 Builder | 2 2 1 0 | r4: +0 e0>0 c4 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Factory Occupation | 1 |  |
| d2_br_36648 | 36670 | 8 | 0 Builder | 1 1 1 0 | r6: +1 e0>1 c1 st0 / r7: +0 e0>0 c0 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #5 0, Horizon #10 0, Cintamani #11 0; exposed 1/1; run-up placements IN(0/6 out) |
| d2_br_36648 | 36671 | 8 | 1 Researcher | 2 2 1 0 | r7: +0 e0>0 c2 st0 / r8: +0 e0>0 c16 st0 | 0 / 1 | execute | threat: Cascade Failure | 1 |  |
| d2_br_36648 | 36672 | 7 | 1 Researcher | 3 1 1 0 | r6: +0 e0>0 c0 st1 / r7: +0 e0>0 c5 st1 | 0 / 1 | execute | threat: Cascade Failure | 1 |  |
| d2_br_36648 | 36673 | 8 | 1 Researcher | 1 4 2 0 | r6: +3 e0>0 c0 st0 / r7: +1 e2>0 (o1) c0 st0 | 0 / 1 | corrupt | corruption | 2 |  |
| d2_br_36648 | 36674 | 6 | 0 Builder | 3 2 1 0 | r5: +1 e3>2 c0 st0 / r6: +1 e2>0 c0 st0 | 0 / 1 | execute | threat: Overrun | 2 |  |
| d2_br_36648 | 36676 | 8 | 1 Researcher | 1 1 2 0 | r7: +2 e0>0 c1 q1 st0 / r8: +0 e0>0 c1 st0 | 0 / 1 | execute | threat: General Mobilization; threat: Shock Doctrine | 2 |  |
| d2_pr_36584 | 36584 | 6 | 1 Researcher | 3 1 1 0 | r4: +0 e0>0 c0 st0 / r5: +0 e0>0 c2 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d2_pr_36584 | 36593 | 5 | 0 Protector | 2 2 1 0 | r3: +0 e2>0 (o2) c1 st0 / r4: +0 e2>0 (o2) c0 st0 | 2 / 1 | corrupt | corruption | 1 | Horizon #10 0, Cintamani #2 0; exposed 2/2 |
| d2_pr_36584 | 36599 | 5 | 0 Protector | 2 2 2 0 | r4: +1 e3>0 c0 st0 / r5: +0 e2>0 (o4) c0 st0 | 1 / 1 | execute | threat: Thermal Runaway | 2 |  |
| d2_pr_36648 | 36651 | 4 | 1 Researcher | 2 2 2 0 | r3: +0 e0>0 c2 st0 / r4: +0 e0>0 c1 st0 | 0 / 1 | execute | threat: Shock Doctrine | 2 |  |
| d2_pr_36648 | 36654 | 8 | 0 Protector | 2 2 4 0 | r7: +2 e6>0 c1 st2 / r8: +0 e2>0 (o2) c0 st2 | 0 / 1 | execute | threat: Last Ultimatum | 3 |  |
| d2_pr_36648 | 36663 | 7 | 1 Researcher | 1 1 2 0 | r6: +1 e1>0 c1 st0 / r7: +1 e1>0 c0 st0 | 0 / 1 | execute | threat: Cascade Failure; threat: Factory Occupation | 3 |  |
| d2_pr_36648 | 36664 | 8 | 0 Protector | 4 2 2 0 | r7: +1 e4>0 c4 st2 / r8: +0 e2>0 (o2) c0 st2 | 0 / 1 | execute | threat: Trade Blockade | 2 |  |
| d2_pr_36648 | 36665 | 6 | 0 Protector | 3 3 1 0 | r5: +1 e2>1 c0 st1 / r6: +1 e3>0 (o1) c0 st2 | 0 / 1 | execute | threat: Dark Cities | 2 |  |
| d2_pr_36648 | 36672 | 4 | 1 Researcher | 2 3 2 0 | r3: +0 e0>0 c0 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Conscription; threat: Total War | 2 |  |
| d2_pr_36648 | 36673 | 6 | 1 Researcher | 3 1 1 0 | r5: +0 e0>0 c9 st0 / r6: +0 e0>0 c7 st0 | 0 / 1 | execute | threat: Shock Doctrine | 1 |  |
| d2_pr_36648 | 36678 | 5 | 0 Protector | 2 2 1 0 | r3: +1 e2>0 c0 st0 / r4: +1 e2>0 (o1) c0 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #5 0, Horizon #10 0, Cintamani #2 0; exposed 3/3; run-up placements out(3/8 out),IN(3/8 out) |

rec_d2_bp_36584: losses 5, boards {'Builder': 4, 'Protector': 1}, fatal cause {'corruption': 3, 'threat': 2}, safe line (extra >= needed) 0
rec_d2_bp_36648: losses 6, boards {'Builder': 5, 'Protector': 1}, fatal cause {'threat': 4, 'corruption': 2}, safe line (extra >= needed) 1
rec_d2_br_36584: losses 3, boards {'Researcher': 2, 'Builder': 1}, fatal cause {'threat': 3}, safe line (extra >= needed) 0
rec_d2_br_36648: losses 21, boards {'Builder': 11, 'Researcher': 10}, fatal cause {'threat': 17, 'corruption': 4}, safe line (extra >= needed) 0
rec_d2_pr_36584: losses 3, boards {'Researcher': 1, 'Protector': 2}, fatal cause {'adversary card': 1, 'corruption': 1, 'threat': 1}, safe line (extra >= needed) 2
rec_d2_pr_36648: losses 8, boards {'Researcher': 4, 'Protector': 4}, fatal cause {'threat': 7, 'corruption': 1}, safe line (extra >= needed) 0
all: losses 46, boards {'Builder': 21, 'Protector': 8, 'Researcher': 17}, fatal cause {'threat': 34, 'corruption': 11, 'adversary card': 1}, safe line (extra >= needed) 3
fatal sources: {'corruption': 11, 'threat: Shock Doctrine': 5, 'threat: General Mobilization': 3, 'threat: Dark Cities': 3, 'threat: Cascade Failure': 3, 'threat: Thermal Runaway': 2, 'threat: Wiper Malware': 2, 'threat: Lab Takeover': 2, 'threat: Purge Protocols': 2, 'threat: Siege': 2, 'threat: Sleeper Cells': 2, 'threat: Total War': 2, 'threat: Conscription': 2, 'threat: Trade Blockade': 2, 'threat: Factory Occupation': 2, 'threat: Scorched Earth': 1, 'threat: Overload': 1, 'threat: Backdoor': 1, 'threat: Garrison State': 1, 'threat: Spearhead': 1, 'threat: Capital Siphon': 1, 'threat: Data Breach': 1, 'threat: Hostile Takeover': 1, 'threat: War Economy': 1, 'threat: Overrun': 1, 'adversary card: Border War': 1, 'threat: Last Ultimatum': 1}
energy left at the action phase's end (sum, the 2 phases): Counter({0: 40, 1: 3, 3: 1, 4: 1, 2: 1})
energy spent off nodes in those phases (all losses): {'card': 13, 'counter thermal_runaway': 9, 'counter data_breach': 7, 'counter grid_collapse': 5, 'counter conscription': 5, 'counter t3_hostile_takeover': 3, 'counter t3_scorched_earth': 2, 'power Fieldwork': 2, 'counter trade_blockade': 2, 'counter t3_spearhead': 1, 'counter t3_iron_curtain': 1, 'counter power_rationing': 1, 'counter dark_cities': 1}
corruption deaths: run-up placements inside the blast 3 of 5 ; of those with a legal hex outside it 2 ; my legal count == the prompt's 5 of 5
loser's placements, whole game: 148 ; into a threatened city's reach 3 ; of those with a safe legal hex 3
corruption deaths with a node-safe placement: 2 of 8
corruption deaths: nodes within 1 of the city at the last action phase's end 11 of 16 ; seats with every node exposed 6 of 8
fatal corrupting cities, adversary ambassadors + marker at the prior action phase's end: {0: 15, 1: 1}
fatal hit took every node held at the end of round R-1: 36
all removals of the losing seat, whole game: {'threat': 134, 'corruption': 36, 'adversary card': 12, 'other': 7, 'war': 4, 'Discord card': 1}
unattributed: {'other: counter:call_up': 4, 'other: counter:factory_occupation': 4, 'other: counter:orbital_strike': 2, 'other: counter:total_war': 2, 'other:adversary.natural': 1, 'other: counter:eviction': 1}

## d1 (28 losses)

| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |
|---|---|---|---|---|---|---|---|---|---|---|
| d1_bp_36584 | 36587 | 8 | 0 Builder | 1 1 1 0 | r6: +0 e0>0 c2 st0 / r7: +0 e0>0 c1 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #10 0, Cintamani #1 0; exposed 1/1 |
| d1_bp_36584 | 36591 | 8 | 1 Protector | 3 3 1 0 | r6: +1 e4>2 c0 st2 / r7: +1 e5>2 c0 st2 | 0 / 1 | build | adv: Border War | 1 |  |
| d1_bp_36584 | 36593 | 6 | 0 Builder | 2 1 1 0 | r4: +0 e0>0 c10 st0 / r5: +1 e1>0 c0 st0 | 0 / 1 | corrupt | corruption | 1 | Horizon #10 0, Cintamani #2 0; exposed 2/2; run-up placements IN(1/4 out) |
| d1_bp_36584 | 36594 | 6 | 0 Builder | 5 5 3 0 | r5: +0 e0>0 c1 st0 / r6: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Shock Doctrine; threat: Sleeper Cells | 3 |  |
| d1_bp_36584 | 36597 | 5 | 0 Builder | 3 3 3 0 | r4: +0 e0>0 c0 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Sleeper Cells | 2 |  |
| d1_bp_36648 | 36656 | 8 | 0 Builder | 2 2 1 0 | r7: +0 e0>0 c0 st1 / r8: +0 e0>0 c0 st1 | 0 / 1 | execute | threat: Garrison State | 1 |  |
| d1_bp_36648 | 36660 | 6 | 1 Protector | 3 4 3 0 | r5: +0 e3>0 (o3) c0 st0 / r6: +1 e3>0 c0 st0 | 1 / 1 | execute | threat: Full Spectrum Assault; threat: Siege | 4 |  |
| d1_bp_36648 | 36668 | 9 | 1 Protector | 3 3 3 0 | r8: +0 e2>2 c1 st2 / r9: +1 e4>0 c0 st2 | 0 / 1 | execute | threat: Border Provocation; threat: Cascade Failure | 4 |  |
| d1_bp_36648 | 36673 | 9 | 0 Builder | 1 1 3 0 | r8: +2 e3>0 c2 st0 / r9: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Capital Siphon | 3 |  |
| d1_br_36584 | 36586 | 7 | 0 Builder | 2 2 2 0 | r6: +0 e0>0 c2 st0 / r7: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Casus Belli; threat: Deepfake Wave | 2 |  |
| d1_br_36584 | 36593 | 6 | 0 Builder | 2 2 2 0 | r5: +0 e0>0 c0 st0 / r6: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Sleeper Cells | 1 |  |
| d1_br_36584 | 36598 | 5 | 1 Researcher | 1 1 1 0 | r4: +0 e0>0 c0 st0 / r5: +1 e0>0 c0 st0 | 0 / 1 | execute | threat: Siege | 2 |  |
| d1_br_36648 | 36648 | 6 | 0 Builder | 1 1 1 0 | r5: +0 e0>0 c0 st0 / r6: +1 e5>4 c0 st0 | 0 / 1 | execute | threat: Lab Takeover | 2 |  |
| d1_br_36648 | 36649 | 5 | 0 Builder | 2 2 2 0 | r4: +0 e0>0 c3 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Factory Occupation | 2 |  |
| d1_br_36648 | 36650 | 6 | 1 Researcher | 1 1 1 0 | r5: +0 e0>0 c4 st0 / r6: +1 e0>0 (o1) c0 st0 | 0 / 1 | execute | Discord: Firestorm; threat: Total War | 2 |  |
| d1_br_36648 | 36660 | 5 | 1 Researcher | 2 2 1 0 | r4: +0 e0>0 c0 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Overload | 1 |  |
| d1_br_36648 | 36661 | 5 | 1 Researcher | 2 2 2 0 | r4: +0 e0>0 c4 st0 / r5: +0 e0>0 c1 st0 | 0 / 1 | execute | threat: Purge Protocols | 1 |  |
| d1_br_36648 | 36665 | 7 | 0 Builder | 1 2 2 0 | r6: +1 e0>0 c2 st0 / r7: +0 e0>0 c5 st0 | 0 / 1 | execute | threat: Lab Takeover; threat: Shock Doctrine | 2 |  |
| d1_br_36648 | 36671 | 8 | 1 Researcher | 2 2 2 0 | r7: +0 e0>0 c10 st0 / r8: +0 e0>0 c13 st0 | 0 / 1 | execute | threat: Cascade Failure; threat: Shock Doctrine | 2 |  |
| d1_br_36648 | 36675 | 5 | 0 Builder | 4 4 4 0 | r4: +0 e0>0 c4 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Full Spectrum Assault; threat: Grid Collapse | 4 |  |
| d1_br_36648 | 36678 | 6 | 1 Researcher | 1 1 1 0 | r5: +0 e0>0 c1 st0 / r6: +1 e6>4 (o1) c0 st0 | 0 / 1 | execute | threat: Factory Occupation; threat: Total War | 2 |  |
| d1_pr_36584 | 36584 | 6 | 1 Researcher | 3 2 2 0 | r5: +0 e0>0 c6 st0 / r6: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Station Seizure | 2 |  |
| d1_pr_36584 | 36593 | 5 | 0 Protector | 2 2 1 0 | r4: +0 e2>0 (o2) c0 st0 / r5: +0 e2>0 (o2) c0 st0 | 2 / 1 | execute | threat: Purge Protocols | 1 |  |
| d1_pr_36648 | 36650 | 9 | 1 Researcher | 3 3 3 0 | r8: +0 e0>0 c12 st0 / r9: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Iron Curtain; threat: Power Rationing | 3 |  |
| d1_pr_36648 | 36656 | 5 | 0 Protector | 2 2 4 0 | r4: +2 e5>0 (o3) c1 st1 / r5: +1 e2>0 c0 st2 | 0 / 1 | execute | threat: Asset Freeze; threat: Sleeper Cells; threat: Turncoats | 5 |  |
| d1_pr_36648 | 36667 | 9 | 1 Researcher | 3 3 4 0 | r8: +1 e3>0 c12 q1 st0 / r9: +1 e1>0 c10 st0 | 0 / 1 | execute | threat: Capital Siphon; threat: Conscription; threat: Factory Occupation; threat: Iron Curtain; threat: Total War | 5 |  |
| d1_pr_36648 | 36670 | 7 | 1 Researcher | 3 3 1 0 | r5: +0 e0>0 c2 st0 / r6: +0 e0>0 c1 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d1_pr_36648 | 36678 | 5 | 0 Protector | 2 2 1 0 | r4: +1 e2>1 c0 st1 / r5: +1 e3>1 c0 st2 | 0 / 1 | execute | threat: Garrison State; threat: Sabotage | 2 |  |

rec_d1_bp_36584: losses 5, boards {'Builder': 4, 'Protector': 1}, fatal cause {'corruption': 2, 'threat': 2, 'adversary card': 1}, safe line (extra >= needed) 0
rec_d1_bp_36648: losses 4, boards {'Builder': 2, 'Protector': 2}, fatal cause {'threat': 4}, safe line (extra >= needed) 1
rec_d1_br_36584: losses 3, boards {'Builder': 2, 'Researcher': 1}, fatal cause {'threat': 3}, safe line (extra >= needed) 0
rec_d1_br_36648: losses 9, boards {'Builder': 4, 'Researcher': 5}, fatal cause {'threat': 8, 'Discord card+threat': 1}, safe line (extra >= needed) 0
rec_d1_pr_36584: losses 2, boards {'Researcher': 1, 'Protector': 1}, fatal cause {'threat': 2}, safe line (extra >= needed) 1
rec_d1_pr_36648: losses 5, boards {'Researcher': 3, 'Protector': 2}, fatal cause {'threat': 4, 'adversary card': 1}, safe line (extra >= needed) 0
all: losses 28, boards {'Builder': 12, 'Protector': 6, 'Researcher': 10}, fatal cause {'threat': 23, 'corruption': 2, 'adversary card': 2, 'Discord card+threat': 1}, safe line (extra >= needed) 2
fatal sources: {'threat: Sleeper Cells': 4, 'threat: Shock Doctrine': 3, 'threat: Factory Occupation': 3, 'threat: Total War': 3, 'corruption': 2, 'adversary card: Border War': 2, 'threat: Garrison State': 2, 'threat: Full Spectrum Assault': 2, 'threat: Siege': 2, 'threat: Cascade Failure': 2, 'threat: Capital Siphon': 2, 'threat: Lab Takeover': 2, 'threat: Purge Protocols': 2, 'threat: Iron Curtain': 2, 'threat: Border Provocation': 1, 'threat: Casus Belli': 1, 'threat: Deepfake Wave': 1, 'Discord card: Firestorm': 1, 'threat: Overload': 1, 'threat: Grid Collapse': 1, 'threat: Station Seizure': 1, 'threat: Power Rationing': 1, 'threat: Asset Freeze': 1, 'threat: Turncoats': 1, 'threat: Conscription': 1, 'threat: Sabotage': 1}
energy left at the action phase's end (sum, the 2 phases): Counter({0: 23, 4: 3, 2: 2})
energy spent off nodes in those phases (all losses): {'card': 4, 'counter overload': 3, 'counter grid_tap': 3, 'power Fieldwork': 2}
corruption deaths: run-up placements inside the blast 1 of 1 ; of those with a legal hex outside it 1 ; my legal count == the prompt's 1 of 1
loser's placements, whole game: 91 ; into a threatened city's reach 1 ; of those with a safe legal hex 1
corruption deaths with a node-safe placement: 1 of 2
corruption deaths: nodes within 1 of the city at the last action phase's end 3 of 3 ; seats with every node exposed 2 of 2
fatal corrupting cities, adversary ambassadors + marker at the prior action phase's end: {0: 4}
fatal hit took every node held at the end of round R-1: 16
all removals of the losing seat, whole game: {'threat': 88, 'corruption': 15, 'adversary card': 7, 'war': 4, 'other': 4, 'Discord card': 1}
unattributed: {'other: counter:casus_belli': 6, 'other: counter:factory_occupation': 4, 'other: counter:call_up': 2, 'other: counter:siege': 2, 'other: counter:eviction': 2, 'other: counter:orbital_strike': 1}

## d0 (16 losses, final build)

| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |
|---|---|---|---|---|---|---|---|---|---|---|
| d0_bp_36584 | 36586 | 8 | 1 Protector | 2 2 2 0 | r7: +0 e3>1 (o2) c7 st2 / r8: +1 e4>0 c2 st2 | 1 / 1 | execute | threat: Casus Belli; threat: Shock Doctrine | 3 |  |
| d0_bp_36584 | 36593 | 5 | 0 Builder | 2 2 2 0 | r4: +0 e0>0 c0 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Purge Protocols; threat: Supply Raid | 2 |  |
| d0_bp_36584 | 36594 | 6 | 1 Protector | 5 4 4 0 | r5: +2 e3>0 c0 st2 / r6: +1 e3>0 c0 st2 | 0 / 1 | execute | threat: Turncoats | 2 |  |
| d0_bp_36648 | 36649 | 6 | 0 Builder | 3 3 1 0 | r5: +0 e0>0 c0 st0 / r6: +1 e0>0 (o1) c0 st0 | 0 / 1 | execute | threat: Sleeper Cells | 2 |  |
| d0_br_36584 | 36592 | 5 | 0 Builder | 2 1 1 0 | r4: +0 e0>0 c4 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Full Spectrum Assault | 1 |  |
| d0_br_36584 | 36593 | 5 | 0 Builder | 2 2 1 0 | r4: +0 e0>0 c1 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Sleeper Cells | 1 |  |
| d0_br_36648 | 36650 | 6 | 1 Researcher | 3 3 1 0 | r5: +1 e9>6 (o1) c0 st1 / r6: +1 e5>1 (o1) c0 st1 | 0 / 1 | execute | threat: Shock Doctrine; threat: Total War | 2 |  |
| d0_br_36648 | 36652 | 9 | 0 Builder | 5 3 3 0 | r7: +1 e0>0 c0 st1 / r8: +0 e0>0 c9 st1 | 0 / 1 | corrupt | corruption | 3 | Cintamani #15 2; exposed 0/3; run-up placements out(7/9 out) |
| d0_br_36648 | 36668 | 10 | 0 Builder | 2 2 3 0 | r9: +1 e2>0 c23 st0 / r10: +1 e1>0 c12 st1 | 0 / 1 | execute | Discord: Blight; threat: Dark Cities | 2 |  |
| d0_br_36648 | 36673 | 9 | 0 Builder | 2 1 2 0 | r7: +2 e6>0 (o3) c0 st1 / r8: +1 e3>1 c11 st2 | 0 / 1 | corrupt | corruption | 2 |  |
| d0_br_36648 | 36675 | 7 | 0 Builder | 4 3 1 0 | r6: +0 e0>0 c0 st0 / r7: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Factory Occupation | 1 |  |
| d0_pr_36584 | 36593 | 5 | 0 Protector | 2 2 1 0 | r4: +0 e2>0 (o2) c0 st0 / r5: +0 e2>0 (o2) c0 st0 | 2 / 1 | execute | threat: Purge Protocols | 1 |  |
| d0_pr_36648 | 36649 | 9 | 0 Protector | 3 3 3 0 | r8: +0 e2>0 (o2) c1 st0 / r9: +0 e2>0 (o2) c0 st0 | 2 / 1 | execute | threat: Deepfake Wave; threat: Spearhead | 3 |  |
| d0_pr_36648 | 36654 | 6 | 1 Researcher | 4 3 3 0 | r5: +0 e0>0 c8 st0 / r6: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Lab Takeover | 2 |  |
| d0_pr_36648 | 36657 | 6 | 0 Protector | 2 3 2 0 | r5: +1 e5>3 c0 st0 / r6: +1 e6>0 (o3) c0 st0 | 0 / 1 | execute | threat: Full Spectrum Assault | 2 |  |
| d0_pr_36648 | 36665 | 10 | 0 Protector | 5 5 1 0 | r8: +1 e3>0 c0 st2 / r9: +0 e2>0 (o2) c0 st2 | 0 / 1 | corrupt | corruption | 1 | Horizon #7 0; exposed 2/5; run-up placements out(5/9 out) |

rec_d0_bp_36584: losses 3, boards {'Protector': 2, 'Builder': 1}, fatal cause {'threat': 3}, safe line (extra >= needed) 1
rec_d0_bp_36648: losses 1, boards {'Builder': 1}, fatal cause {'threat': 1}, safe line (extra >= needed) 0
rec_d0_br_36584: losses 2, boards {'Builder': 2}, fatal cause {'threat': 2}, safe line (extra >= needed) 0
rec_d0_br_36648: losses 5, boards {'Researcher': 1, 'Builder': 4}, fatal cause {'threat': 2, 'corruption': 2, 'Discord card+threat': 1}, safe line (extra >= needed) 0
rec_d0_pr_36584: losses 1, boards {'Protector': 1}, fatal cause {'threat': 1}, safe line (extra >= needed) 1
rec_d0_pr_36648: losses 4, boards {'Protector': 3, 'Researcher': 1}, fatal cause {'threat': 3, 'corruption': 1}, safe line (extra >= needed) 1
all: losses 16, boards {'Protector': 6, 'Builder': 8, 'Researcher': 2}, fatal cause {'threat': 12, 'corruption': 3, 'Discord card+threat': 1}, safe line (extra >= needed) 3
fatal sources: {'corruption': 3, 'threat: Shock Doctrine': 2, 'threat: Purge Protocols': 2, 'threat: Sleeper Cells': 2, 'threat: Full Spectrum Assault': 2, 'threat: Casus Belli': 1, 'threat: Supply Raid': 1, 'threat: Turncoats': 1, 'threat: Total War': 1, 'Discord card: Blight': 1, 'threat: Dark Cities': 1, 'threat: Factory Occupation': 1, 'threat: Deepfake Wave': 1, 'threat: Spearhead': 1, 'threat: Lab Takeover': 1}
energy left at the action phase's end (sum, the 2 phases): Counter({0: 12, 1: 2, 7: 1, 3: 1})
energy spent off nodes in those phases (all losses): {'card': 10, 'counter t3_cascade_failure': 3, 'counter conscription': 3, 'power Fieldwork': 2, 'counter commandeer': 2, 'power Rush Build': 1}
corruption deaths: run-up placements inside the blast 0 of 2 ; of those with a legal hex outside it 0 ; my legal count == the prompt's 2 of 2
loser's placements, whole game: 68 ; into a threatened city's reach 0 ; of those with a safe legal hex 0
corruption deaths with a node-safe placement: 0 of 2
corruption deaths: nodes within 1 of the city at the last action phase's end 2 of 8 ; seats with every node exposed 0 of 2
fatal corrupting cities, adversary ambassadors + marker at the prior action phase's end: {0: 1, 2: 1}
fatal hit took every node held at the end of round R-1: 10
all removals of the losing seat, whole game: {'threat': 55, 'corruption': 15, 'other': 8, 'adversary card': 3, 'war': 2, 'Discord card': 1}
unattributed: {'other: counter:factory_occupation': 4, 'other: counter:call_up': 2, 'other: counter:total_war': 2, 'other: counter:eviction': 1}
