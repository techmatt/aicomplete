# 142: the death audit (136's death_audit.py on the final build's games on the first independent seeds, 36776-36799 and 36832-36855)

d1 and d2 are the build before M13 (M13 acts only at d3, so those games are the final build's); d3 is the final build.

## d1

| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |
|---|---|---|---|---|---|---|---|---|---|---|
| d1_br_36776 | 36793 | 7 | 0 Builder | 3 2 2 0 | r6: +0 e1>0 (o1) c28 st1 / r7: +1 e2>0 c0 st1 | 0 / 1 | execute | threat: Capital Siphon; threat: Propaganda Ministry | 3 |  |
| d1_br_36776 | 36796 | 8 | 0 Builder | 3 1 2 0 | r7: +1 e2>0 c0 st2 / r8: +1 e7>4 c4 st2 | 0 / 1 | execute | threat: Cascade Failure | 3 |  |
| d1_br_36832 | 36850 | 7 | 0 Builder | 3 3 1 0 | r6: +1 e2>0 c0 st1 / r7: +0 e0>0 c2 st1 | 0 / 1 | execute | threat: Last Ultimatum | 1 |  |
| d1_br_36832 | 36854 | 6 | 0 Builder | 2 3 1 0 | r4: +1 e2>0 c1 st1 / r5: +0 e1>1 c0 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #2 1, Horizon #8 0, Cintamani #3 0, Cintamani #14 0; exposed 3/3; run-up placements IN(2/7 out) |

rec_d1_br_36776: losses 2, boards {'Builder': 2}, fatal cause {'threat': 2}, safe line (extra >= needed) 0
rec_d1_br_36832: losses 2, boards {'Builder': 2}, fatal cause {'threat': 1, 'corruption': 1}, safe line (extra >= needed) 0
all: losses 4, boards {'Builder': 4}, fatal cause {'threat': 3, 'corruption': 1}, safe line (extra >= needed) 0
fatal sources: {'threat: Capital Siphon': 1, 'threat: Propaganda Ministry': 1, 'threat: Cascade Failure': 1, 'threat: Last Ultimatum': 1, 'corruption': 1}
energy left at the action phase's end (sum, the 2 phases): Counter({0: 2, 4: 1, 1: 1})
energy spent off nodes in those phases (all losses): {'card': 1}
corruption deaths: run-up placements inside the blast 1 of 1 ; of those with a legal hex outside it 1 ; my legal count == the prompt's 1 of 1
loser's placements, whole game: 22 ; into a threatened city's reach 0 ; of those with a safe legal hex 0
corruption deaths with a node-safe placement: 1 of 1
corruption deaths: nodes within 1 of the city at the last action phase's end 3 of 3 ; seats with every node exposed 1 of 1
fatal corrupting cities, adversary ambassadors + marker at the prior action phase's end: {0: 3, 1: 1}
fatal hit took every node held at the end of round R-1: 2
all removals of the losing seat, whole game: {'threat': 20, 'corruption': 4, 'war': 1, 'adversary card': 1}
unattributed: {'other:adversary.natural': 1}

## d2

| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |
|---|---|---|---|---|---|---|---|---|---|---|
| d2_bp_36776 | 36784 | 6 | 0 Builder | 3 2 1 0 | r5: +0 e1>0 (o1) c0 st1 / r6: +1 e2>0 c2 st2 | 0 / 1 | execute | threat: Capital Siphon | 2 |  |
| d2_bp_36832 | 36837 | 5 | 0 Builder | 3 1 2 0 | r4: +1 e1>0 c2 st1 / r5: +0 e1>1 c1 st1 | 0 / 1 | execute | threat: Full Spectrum Assault | 1 |  |
| d2_bp_36832 | 36844 | 6 | 1 Protector | 3 4 1 0 | r5: +0 e3>0 (o3) c0 st0 / r6: +2 e6>0 (o1) c0 st0 | 1 / 1 | execute | threat: Purge Protocols | 3 |  |
| d2_bp_36832 | 36847 | 7 | 1 Protector | 4 2 2 0 | r6: +1 e4>0 c0 st2 / r7: +1 e3>0 (o2) c0 st2 | 0 / 1 | execute | threat: War Economy | 3 |  |
| d2_br_36776 | 36781 | 6 | 1 Researcher | 4 4 1 0 | r4: +0 e0>0 c0 st1 / r5: +0 e4>0 (o4) c0 st1 | 1 / 1 | build | adv: Border War | 1 |  |
| d2_br_36776 | 36794 | 7 | 1 Researcher | 3 2 2 0 | r6: +1 e1>0 c0 st1 / r7: +0 e1>0 (o1) c0 st1 | 0 / 1 | execute | threat: Iron Curtain | 2 |  |
| d2_br_36776 | 36796 | 7 | 0 Builder | 1 1 1 0 | r6: +0 e4>0 (o4) c10 st0 / r7: +1 e3>0 c0 st0 | 2 / 1 | execute | threat: Cascade Failure | 2 |  |
| d2_br_36832 | 36835 | 7 | 0 Builder | 1 2 2 0 | r6: +0 e1>1 c2 st1 / r7: +1 e2>0 c1 st2 | 0 / 1 | execute | threat: War Economy | 3 |  |
| d2_br_36832 | 36848 | 6 | 0 Builder | 2 4 2 0 | r5: +0 e1>1 c0 st1 / r6: +1 e2>0 c0 st2 | 0 / 1 | execute | threat: Cascade Failure | 3 |  |
| d2_br_36832 | 36850 | 6 | 0 Builder | 4 5 4 0 | r5: +0 e1>0 (o1) c0 st1 / r6: +1 e2>0 c0 st2 | 0 / 1 | execute | threat: Cascade Failure; threat: Total War | 4 |  |
| d2_br_36832 | 36854 | 5 | 1 Researcher | 3 3 2 0 | r4: +1 e1>0 c0 st0 / r5: +1 e1>0 c0 st1 | 0 / 1 | execute | threat: Cascade Failure; threat: Casus Belli | 3 |  |
| d2_pr_36832 | 36839 | 6 | 0 Protector | 4 3 3 0 | r5: +1 e8>5 c0 st0 / r6: +0 e8>0 (o8) c0 st0 | 1 / 1 | execute | threat: Regime Change | 3 |  |

rec_d2_bp_36776: losses 1, boards {'Builder': 1}, fatal cause {'threat': 1}, safe line (extra >= needed) 0
rec_d2_bp_36832: losses 3, boards {'Builder': 1, 'Protector': 2}, fatal cause {'threat': 3}, safe line (extra >= needed) 1
rec_d2_br_36776: losses 3, boards {'Researcher': 2, 'Builder': 1}, fatal cause {'threat': 2, 'adversary card': 1}, safe line (extra >= needed) 2
rec_d2_br_36832: losses 4, boards {'Builder': 3, 'Researcher': 1}, fatal cause {'threat': 4}, safe line (extra >= needed) 0
rec_d2_pr_36832: losses 1, boards {'Protector': 1}, fatal cause {'threat': 1}, safe line (extra >= needed) 1
all: losses 12, boards {'Builder': 6, 'Protector': 3, 'Researcher': 3}, fatal cause {'threat': 11, 'adversary card': 1}, safe line (extra >= needed) 4
fatal sources: {'threat: Cascade Failure': 4, 'threat: War Economy': 2, 'threat: Capital Siphon': 1, 'threat: Full Spectrum Assault': 1, 'threat: Purge Protocols': 1, 'adversary card: Border War': 1, 'threat: Iron Curtain': 1, 'threat: Total War': 1, 'threat: Casus Belli': 1, 'threat: Regime Change': 1}
energy left at the action phase's end (sum, the 2 phases): Counter({0: 8, 1: 3, 5: 1})
energy spent off nodes in those phases (all losses): {'card': 12, 'counter shock_doctrine': 5, 'counter siege': 3, 'counter data_breach': 2, 'power Shelter': 1, 'counter orbital_strike': 1, 'counter provocation': 1}
corruption deaths: run-up placements inside the blast 0 of 0 ; of those with a legal hex outside it 0 ; my legal count == the prompt's 0 of 0
loser's placements, whole game: 67 ; into a threatened city's reach 1 ; of those with a safe legal hex 1
corruption deaths with a node-safe placement: 0 of 0
corruption deaths: nodes within 1 of the city at the last action phase's end 0 of 0 ; seats with every node exposed 0 of 0
fatal corrupting cities, adversary ambassadors + marker at the prior action phase's end: {}
fatal hit took every node held at the end of round R-1: 4
all removals of the losing seat, whole game: {'threat': 70, 'corruption': 6, 'adversary card': 3}
unattributed: {}

## d3

| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |
|---|---|---|---|---|---|---|---|---|---|---|
| d3_bp_36776 | 36777 | 7 | 1 Protector | 3 2 2 0 | r6: +1 e3>1 c5 st2 / r7: +1 e4>1 c2 st2 | 0 / 1 | execute | threat: Sleeper Cells | 3 |  |
| d3_bp_36776 | 36781 | 6 | 0 Builder | 2 2 1 0 | r5: +0 e4>0 (o4) c0 st0 / r6: +1 e5>2 c0 st0 | 1 / 1 | execute | threat: Manhunt | 2 |  |
| d3_bp_36776 | 36788 | 7 | 1 Protector | 6 5 5 0 | r6: +1 e6>2 c6 st2 / r7: +0 e5>0 (o5) c0 st2 | 1 / 1 | execute | threat: Last Ultimatum; threat: Station Seizure | 5 |  |
| d3_bp_36776 | 36793 | 8 | 1 Protector | 5 6 4 0 | r7: +0 e3>0 (o3) c11 st2 / r8: +0 e4>0 (o4) c0 st2 | 2 / 1 | execute | threat: Cascade Failure; threat: General Mobilization | 3 |  |
| d3_bp_36776 | 36797 | 5 | 0 Builder | 3 1 1 0 | r3: +1 e6>4 c0 st0 / r4: +1 e5>0 (o2) c0 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d3_bp_36776 | 36798 | 5 | 0 Builder | 4 3 2 0 | r4: +0 e3>0 (o3) c0 st0 / r5: +1 e2>0 (o1) c0 st1 | 1 / 1 | execute | threat: Data Breach | 3 |  |
| d3_bp_36776 | 36799 | 6 | 1 Protector | 4 1 1 0 | r5: +2 e9>4 c0 st0 / r6: +2 e7>4 c0 st2 | 0 / 1 | execute | threat: Scorched Earth | 3 |  |
| d3_bp_36832 | 36832 | 5 | 1 Protector | 2 2 2 0 | r4: +1 e5>0 (o2) c0 st0 / r5: +1 e4>0 c0 st0 | 0 / 1 | execute | threat: Defection; threat: Manhunt | 3 |  |
| d3_bp_36832 | 36838 | 8 | 1 Protector | 6 4 4 0 | r7: +1 e7>5 c0 st2 / r8: +2 e8>0 (o2) c10 st3 | 0 / 1 | execute | threat: Capital Siphon; threat: Data Breach; threat: Lab Takeover; threat: War Economy | 6 |  |
| d3_bp_36832 | 36839 | 7 | 1 Protector | 5 5 1 0 | r5: +1 e8>5 c1 st0 / r6: +0 e8>0 (o8) c0 st0 | 1 / 1 | build | adv: Border War | 1 |  |
| d3_bp_36832 | 36840 | 6 | 0 Builder | 2 2 2 0 | r5: +0 e0>0 c3 st1 / r6: +0 e1>1 c12 st1 | 0 / 1 | execute | threat: Station Seizure | 1 |  |
| d3_bp_36832 | 36841 | 6 | 0 Builder | 4 1 1 0 | r5: +0 e1>1 c0 st1 / r6: +1 e2>0 c1 st2 | 0 / 1 | execute | threat: General Mobilization | 2 |  |
| d3_bp_36832 | 36842 | 5 | 0 Builder | 1 1 2 0 | r4: +1 e1>0 c1 st1 / r5: +1 e1>1 c0 st2 | 0 / 1 | execute | threat: Full Spectrum Assault | 3 |  |
| d3_bp_36832 | 36848 | 5 | 1 Protector | 2 2 1 0 | r4: +0 e4>0 (o4) c0 st1 / r5: +2 e7>0 (o2) c0 st2 | 1 / 1 | execute | threat: Call-Up; threat: Load Shedding; threat: Requisition | 3 |  |
| d3_bp_36832 | 36850 | 6 | 0 Builder | 2 1 1 0 | r4: +1 e1>0 c0 st1 / r5: +0 e1>0 (o1) c2 st1 | 0 / 1 | build | adv: Border War | 1 |  |
| d3_bp_36832 | 36853 | 7 | 1 Protector | 5 6 6 0 | r6: +1 e9>1 (o7) c14 st1 / r7: +1 e4>0 (o2) c0 st2 | 0 / 1 | execute | threat: Data Breach; threat: Propaganda Ministry; threat: Spearhead; threat: Station Seizure | 7 |  |
| d3_bp_36832 | 36854 | 5 | 0 Builder | 1 2 3 0 | r4: +1 e1>0 c1 st1 / r5: +0 e1>0 (o1) c0 st1 | 0 / 1 | execute | threat: Conscription | 3 |  |
| d3_br_36776 | 36776 | 5 | 0 Builder | 1 1 1 0 | r4: +1 e1>0 c0 st1 / r5: +1 e0>0 c0 st2 | 0 / 1 | execute | threat: Casus Belli | 2 |  |
| d3_br_36776 | 36777 | 5 | 0 Builder | 1 1 1 0 | r4: +0 e1>1 c1 st1 / r5: +1 e2>0 c1 st1 | 0 / 1 | execute | threat: Garrison State | 2 |  |
| d3_br_36776 | 36782 | 5 | 1 Researcher | 1 2 1 0 | r4: +1 e1>0 c0 st1 / r5: +0 e0>0 c1 st1 | 0 / 1 | execute | threat: Border Provocation | 1 |  |
| d3_br_36776 | 36783 | 6 | 1 Researcher | 2 2 1 0 | r5: +1 e3>0 c0 st0 / r6: +1 e1>0 c0 st0 | 0 / 1 | execute | threat: Full Spectrum Assault; threat: General Mobilization | 2 |  |
| d3_br_36776 | 36784 | 6 | 0 Builder | 2 3 1 0 | r5: +1 e2>0 c0 st2 / r6: +2 e7>0 c0 st2 | 0 / 1 | execute | threat: Capital Siphon; threat: General Mobilization | 3 |  |
| d3_br_36776 | 36785 | 5 | 1 Researcher | 2 3 2 0 | r4: +0 e0>0 c0 st0 / r5: +0 e0>0 c0 st0 | 0 / 1 | execute | threat: Data Breach | 2 |  |
| d3_br_36776 | 36786 | 5 | 1 Researcher | 1 2 1 0 | r4: +1 e1>0 c0 st1 / r5: +0 e1>0 (o1) c2 st1 | 0 / 1 | execute | threat: Siege | 1 |  |
| d3_br_36776 | 36788 | 7 | 1 Researcher | 3 2 1 0 | r6: +0 e1>0 (o1) c0 st1 / r7: +0 e1>0 (o1) c0 st1 | 0 / 1 | execute | threat: Shock Doctrine | 1 |  |
| d3_br_36776 | 36794 | 6 | 1 Researcher | 1 2 2 0 | r5: +1 e1>0 c12 st0 / r6: +1 e1>0 c1 st1 | 0 / 1 | execute | threat: Iron Curtain | 3 |  |
| d3_br_36776 | 36798 | 6 | 1 Researcher | 2 1 1 0 | r5: +0 e0>0 c1 st1 / r6: +0 e0>0 c5 st1 | 0 / 1 | execute | threat: War Economy | 1 |  |
| d3_br_36832 | 36832 | 5 | 1 Researcher | 2 2 2 0 | r4: +0 e0>0 c8 st0 / r5: +0 e0>0 c2 st0 | 0 / 1 | execute | threat: Casus Belli | 2 |  |
| d3_br_36832 | 36833 | 7 | 0 Builder | 1 1 1 0 | r6: +0 e1>1 c7 st1 / r7: +1 e4>0 (o4) c1 st2 | 0 / 1 | execute | threat: Spearhead | 2 |  |
| d3_br_36832 | 36837 | 6 | 0 Builder | 2 3 1 0 | r5: +0 e1>0 (o1) c7 st1 / r6: +2 e1>0 (o1) c0 st2 | 0 / 1 | execute | threat: General Mobilization; threat: Overload | 3 |  |
| d3_br_36832 | 36842 | 6 | 0 Builder | 1 2 1 0 | r5: +1 e1>0 c0 st1 / r6: +0 e1>1 c0 st1 | 0 / 1 | execute | threat: Shakedown | 1 |  |
| d3_br_36832 | 36843 | 6 | 1 Researcher | 2 2 3 0 | r5: +1 e1>0 c0 st1 / r6: +0 e1>0 (o1) c0 st1 | 0 / 1 | execute | threat: Purge Protocols | 2 |  |
| d3_br_36832 | 36848 | 5 | 0 Builder | 1 2 2 0 | r4: +1 e1>0 c0 st1 / r5: +0 e1>1 c0 st1 | 0 / 1 | execute | threat: Cascade Failure; threat: Embargo | 2 |  |
| d3_br_36832 | 36850 | 5 | 0 Builder | 1 2 1 0 | r3: +1 e2>0 c3 st0 / r4: +1 e1>0 c0 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #2 1, Horizon #8 0, Cintamani #3 1; exposed 2/3; run-up placements IN(4/5 out),IN(5/7 out) |
| d3_pr_36776 | 36777 | 5 | 0 Protector | 2 1 2 0 | r4: +1 e7>5 c2 st0 / r5: +0 e8>0 (o8) c0 st0 | 1 / 1 | execute | threat: Purge Protocols | 1 |  |
| d3_pr_36776 | 36782 | 8 | 1 Researcher | 1 1 1 0 | r6: +0 e1>0 (o1) c0 st1 / r7: +0 e1>0 (o1) c0 st1 | 0 / 1 | corrupt | corruption | 1 | Horizon #7 1; exposed 0/1 |
| d3_pr_36776 | 36785 | 5 | 1 Researcher | 2 3 3 0 | r4: +1 e1>0 c0 st1 / r5: +0 e1>0 (o1) c0 st1 | 0 / 1 | execute | threat: Data Breach; threat: Supply Raid | 3 |  |
| d3_pr_36776 | 36796 | 9 | 0 Protector | 6 1 1 0 | r7: +0 e3>2 (o3) c5 st3 / r8: +0 e3>0 (o3) c6 st3 | 1 / 1 | corrupt | corruption | 1 |  |
| d3_pr_36776 | 36798 | 7 | 1 Researcher | 3 1 1 0 | r6: +0 e0>0 c0 st1 / r7: +0 e0>0 c0 st1 | 0 / 1 | execute | threat: War Economy | 1 |  |
| d3_pr_36832 | 36834 | 6 | 0 Protector | 3 2 1 0 | r5: +1 e5>1 c0 st2 / r6: +1 e4>0 (o1) c0 st2 | 0 / 1 | execute | threat: Manhunt; threat: Turncoats | 2 |  |
| d3_pr_36832 | 36836 | 5 | 1 Researcher | 2 3 1 0 | r3: +1 e2>0 c2 st0 / r4: +0 e0>0 c0 st0 | 0 / 1 | build | adv: Border War | 1 |  |
| d3_pr_36832 | 36837 | 6 | 0 Protector | 2 3 2 0 | r5: +2 e5>0 c0 st1 / r6: +1 e3>0 (o1) c0 st2 | 0 / 1 | execute | threat: Overload | 3 |  |
| d3_pr_36832 | 36842 | 5 | 0 Protector | 1 1 1 0 | r3: +1 e5>6 c0 st0 / r4: +1 e9>6 c0 st0 | 0 / 1 | natural | other:adversary.natural | 1 |  |
| d3_pr_36832 | 36844 | 6 | 1 Researcher | 3 1 1 0 | r4: +1 e1>0 c0 st1 / r5: +0 e0>0 c13 st1 | 0 / 1 | corrupt | corruption | 1 | Cintamani #10 0; exposed 1/1; run-up placements IN(3/8 out) |
| d3_pr_36832 | 36848 | 6 | 1 Researcher | 1 2 1 0 | r5: +1 e1>0 c1 st1 / r6: +0 e1>0 (o1) c2 st1 | 0 / 1 | execute | threat: Zero Day | 1 |  |
| d3_pr_36832 | 36849 | 8 | 0 Protector | 4 3 4 0 | r7: +1 e3>0 c6 q2 st3 / r8: +1 e4>0 c12 st5 | 0 / 1 | execute | threat: Lab Takeover; threat: Regime Change | 5 |  |
| d3_pr_36832 | 36850 | 5 | 0 Protector | 1 3 1 0 | r3: +2 e7>2 c2 st0 / r4: +0 e5>0 (o5) c4 st0 | 1 / 1 | corrupt | corruption | 1 | Horizon #2 1, Horizon #8 0, Horizon #11 0, Cintamani #3 1; exposed 3/3; run-up placements IN(0/5 out),IN(0/5 out) |
| d3_pr_36832 | 36854 | 8 | 0 Protector | 1 2 2 0 | r6: +2 e7>0 (o5) c0 st1 / r7: +0 e8>0 (o8) c0 st1 | 1 / 1 | corrupt | corruption | 2 | Horizon #9 0, Horizon #10 0, Cintamani #11 0; exposed 2/2; run-up placements IN(0/6 out),IN(2/8 out) |

rec_d3_bp_36776: losses 7, boards {'Protector': 4, 'Builder': 3}, fatal cause {'threat': 6, 'adversary card': 1}, safe line (extra >= needed) 4
rec_d3_bp_36832: losses 10, boards {'Protector': 5, 'Builder': 5}, fatal cause {'threat': 8, 'adversary card': 2}, safe line (extra >= needed) 2
rec_d3_br_36776: losses 10, boards {'Builder': 3, 'Researcher': 7}, fatal cause {'threat': 10}, safe line (extra >= needed) 0
rec_d3_br_36832: losses 7, boards {'Researcher': 2, 'Builder': 5}, fatal cause {'threat': 6, 'corruption': 1}, safe line (extra >= needed) 0
rec_d3_pr_36776: losses 5, boards {'Protector': 2, 'Researcher': 3}, fatal cause {'threat': 3, 'corruption': 2}, safe line (extra >= needed) 2
rec_d3_pr_36832: losses 9, boards {'Protector': 6, 'Researcher': 3}, fatal cause {'threat': 4, 'corruption': 3, 'adversary card': 1, 'other': 1}, safe line (extra >= needed) 2
all: losses 48, boards {'Protector': 17, 'Builder': 16, 'Researcher': 15}, fatal cause {'threat': 37, 'corruption': 6, 'adversary card': 4, 'other': 1}, safe line (extra >= needed) 10
fatal sources: {'corruption': 6, 'threat: General Mobilization': 5, 'threat: Data Breach': 5, 'adversary card: Border War': 4, 'threat: Manhunt': 3, 'threat: Station Seizure': 3, 'threat: War Economy': 3, 'threat: Cascade Failure': 2, 'threat: Capital Siphon': 2, 'threat: Lab Takeover': 2, 'threat: Full Spectrum Assault': 2, 'threat: Spearhead': 2, 'threat: Casus Belli': 2, 'threat: Overload': 2, 'threat: Purge Protocols': 2, 'threat: Sleeper Cells': 1, 'threat: Last Ultimatum': 1, 'threat: Scorched Earth': 1, 'threat: Defection': 1, 'threat: Call-Up': 1, 'threat: Load Shedding': 1, 'threat: Requisition': 1, 'threat: Propaganda Ministry': 1, 'threat: Conscription': 1, 'threat: Garrison State': 1, 'threat: Border Provocation': 1, 'threat: Siege': 1, 'threat: Shock Doctrine': 1, 'threat: Iron Curtain': 1, 'threat: Shakedown': 1, 'threat: Embargo': 1, 'threat: Supply Raid': 1, 'threat: Turncoats': 1, 'other:adversary.natural': 1, 'threat: Zero Day': 1, 'threat: Regime Change': 1}
energy left at the action phase's end (sum, the 2 phases): Counter({0: 28, 1: 9, 2: 5, 5: 3, 4: 1, 8: 1, 12: 1})
energy spent off nodes in those phases (all losses): {'card': 33, 'counter eviction': 14, 'counter siege': 8, 'counter t3_spearhead': 7, 'counter t3_last_ultimatum': 5, 'power Fieldwork': 5, 'counter cooling_failure': 4, 'counter garrison_state': 4, 'counter manhunt': 4, 'counter data_breach': 3, 'counter t3_general_mobilization': 2, 'counter orbital_strike': 2, 'counter load_shedding': 2, 'power Rush Build': 2, 'counter t3_hostile_takeover': 1, 'power Shelter': 1, 'counter shock_doctrine': 1, 'counter overload': 1}
corruption deaths: run-up placements inside the blast 7 of 7 ; of those with a legal hex outside it 4 ; my legal count == the prompt's 7 of 7
loser's placements, whole game: 276 ; into a threatened city's reach 7 ; of those with a safe legal hex 6
corruption deaths with a node-safe placement: 3 of 5
corruption deaths: nodes within 1 of the city at the last action phase's end 8 of 10 ; seats with every node exposed 3 of 5
fatal corrupting cities, adversary ambassadors + marker at the prior action phase's end: {0: 7, 1: 5}
fatal hit took every node held at the end of round R-1: 24
all removals of the losing seat, whole game: {'threat': 283, 'corruption': 31, 'adversary card': 8, 'war': 1, 'other': 1}
unattributed: {'other:adversary.natural': 1}

