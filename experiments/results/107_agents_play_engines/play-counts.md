# 107: play counts, r42's agents → 107's agents

The fresh read (`experiments/screen-107-r42.json`, seeds 33596-33643, 48 a cell, the three pairs at d2): r42's binary
(5601d22) and 107's on the same seeds. Each cell: offered / kept / played a game for the team, the three pairs pooled
(`play_counts.py out/read-base out/read-new`). Kept counts the opening deal and draws. Groups: 106's list (r42.md,
*Play counts*), the Capital engine, the six tier-2 every-tag triggers, and every new or changed card the strategist
played under 0.05 a game on r42's agents on these seeds.

## The strategist

| card | group | before | after |
|---|---|---|---|
| Citation Engine | 106 list, under 0.05 | 0.12 / 0.03 / 0.01 | 0.13 / 0.03 / 0.01 |
| Foundation Model Campus | 106 list | 0.60 / 0.13 / 0.08 | 0.61 / 0.15 / 0.05 |
| Security Contractor HQ | 106 list | 0.26 / 0.15 / 0.06 | 0.27 / 0.16 / 0.03 |
| Orbital Mass Driver | 106 list, under 0.05 | 0.42 / 0.00 / 0.00 | 0.41 / 0.01 / 0.00 |
| Dyson Swarm Tranche | 106 list, under 0.05 | 0.56 / 0.01 / 0.00 | 0.53 / 0.00 / 0.00 |
| Continental Supergrid | 106 list, under 0.05 | 0.53 / 0.00 / 0.00 | 0.53 / 0.08 / 0.04 |
| Tidal Barrage | 106 list, under 0.05 | 0.27 / 0.02 / 0.00 | 0.26 / 0.06 / 0.01 |
| Smart Grid Backbone | 106 list, trigger, under 0.05 | 0.58 / 0.04 / 0.03 | 0.56 / 0.08 / 0.06 |
| Dispatch Control Room | 106 list, under 0.05 | 0.18 / 0.03 / 0.00 | 0.19 / 0.03 / 0.02 |
| Conglomerate Headquarters | 106 list, trigger, under 0.05 | 0.56 / 0.02 / 0.01 | 0.53 / 0.08 / 0.06 |
| Retained Earnings | 106 list, Capital, under 0.05 | 0.21 / 0.00 / 0.00 | 0.20 / 0.11 / 0.08 |
| Recursive Self-Improvement | 106 list, under 0.05 | 0.47 / 0.03 / 0.01 | 0.44 / 0.03 / 0.01 |
| Positron Storage Ring | 106 list, under 0.05 | 0.60 / 0.10 / 0.03 | 0.62 / 0.07 / 0.00 |
| Power to the People | 106 list, under 0.05 | 0.49 / 0.06 / 0.04 | 0.46 / 0.01 / 0.01 |
| Hyperscale Botnet | 106 list, under 0.05 | 0.57 / 0.22 / 0.00 | 0.57 / 0.18 / 0.00 |
| Payroll Automation | 106 list, under 0.05 | 0.24 / 0.24 / 0.01 | 0.22 / 0.02 / 0.01 |
| Capital Injection | Capital, under 0.05 | 0.13 / 0.00 / 0.00 | 0.12 / 0.00 / 0.01 |
| Vendor Financing | Capital, under 0.05 | 0.17 / 0.00 / 0.00 | 0.17 / 0.01 / 0.01 |
| Liquidity Event | Capital, under 0.05 | 0.17 / 0.00 / 0.00 | 0.19 / 0.01 / 0.00 |
| Collateralized Grid | Capital, under 0.05 | 0.53 / 0.00 / 0.00 | 0.55 / 0.03 / 0.00 |
| Interdisciplinary Institute | trigger | 0.44 / 0.26 / 0.14 | 0.41 / 0.23 / 0.12 |
| Command-and-Control Grid | trigger | 0.54 / 0.26 / 0.15 | 0.53 / 0.24 / 0.14 |
| Orbital Embassy | trigger | 0.55 / 0.19 / 0.12 | 0.55 / 0.19 / 0.14 |
| Agency Directorate | trigger | 0.42 / 0.20 / 0.08 | 0.39 / 0.28 / 0.18 |
| Capacitor Discharge | under 0.05 | 0.10 / 0.04 / 0.01 | 0.08 / 0.03 / 0.01 |
| Capacity Market | under 0.05 | 0.21 / 0.04 / 0.03 | 0.21 / 0.03 / 0.02 |
| Consulate Tower | under 0.05 | 0.12 / 0.03 / 0.03 | 0.14 / 0.08 / 0.09 |
| Economies of Scale | under 0.05 | 0.17 / 0.03 / 0.02 | 0.21 / 0.05 / 0.03 |
| Endowed Research Chair | under 0.05 | 0.15 / 0.10 / 0.02 | 0.17 / 0.11 / 0.03 |
| Exploit Broker | under 0.05 | 0.15 / 0.06 / 0.03 | 0.17 / 0.10 / 0.04 |
| Force Protection Order | under 0.05 | 0.12 / 0.21 / 0.01 | 0.12 / 0.23 / 0.01 |
| Grid Census | under 0.05 | 0.24 / 0.17 / 0.01 | 0.21 / 0.15 / 0.01 |
| Kinetic Interdiction | under 0.05 | 0.60 / 0.06 / 0.03 | 0.61 / 0.02 / 0.01 |
| Plant Overdrive | under 0.05 | 0.15 / 0.03 / 0.00 | 0.17 / 0.03 / 0.00 |
| Pumped Hydro Storage | under 0.05 | 0.21 / 0.30 / 0.03 | 0.19 / 0.33 / 0.05 |
| Recruit Operatives | under 0.05 | 0.17 / 0.03 / 0.01 | 0.17 / 0.02 / 0.01 |
| Singularity Exploit | under 0.05 | 0.39 / 0.05 / 0.04 | 0.41 / 0.03 / 0.03 |
| Vertical Monopoly | under 0.05 | 0.50 / 0.06 / 0.03 | 0.48 / 0.05 / 0.03 |
| Zero-Day Auction | under 0.05 | 0.54 / 0.06 / 0.02 | 0.53 / 0.08 / 0.03 |

Before, every card played under 0.05 a game: 33
Capacitor Discharge, Capacity Market, Capital Injection, Citation Engine, Collateralized Grid, Conglomerate Headquarters, Consulate Tower, Continental Supergrid, Dispatch Control Room, Dyson Swarm Tranche, Economies of Scale, Endowed Research Chair, Exploit Broker, Force Protection Order, Grid Census, Hyperscale Botnet, Kinetic Interdiction, Liquidity Event, Orbital Mass Driver, Payroll Automation, Plant Overdrive, Positron Storage Ring, Power to the People, Pumped Hydro Storage, Recruit Operatives, Recursive Self-Improvement, Retained Earnings, Singularity Exploit, Smart Grid Backbone, Tidal Barrage, Vendor Financing, Vertical Monopoly, Zero-Day Auction

Of those, played 0.05 a game or more after: 4

## Greedy

| card | group | before | after |
|---|---|---|---|
| Citation Engine | 106 list, under 0.05 | 0.08 / 0.00 / 0.00 | 0.08 / 0.00 / 0.00 |
| Foundation Model Campus | 106 list, under 0.05 | 0.54 / 0.00 / 0.00 | 0.54 / 0.00 / 0.00 |
| Security Contractor HQ | 106 list, under 0.05 | 0.20 / 0.04 / 0.00 | 0.19 / 0.03 / 0.00 |
| Orbital Mass Driver | 106 list, under 0.05 | 0.34 / 0.00 / 0.00 | 0.33 / 0.00 / 0.00 |
| Dyson Swarm Tranche | 106 list, under 0.05 | 0.53 / 0.00 / 0.00 | 0.53 / 0.00 / 0.00 |
| Continental Supergrid | 106 list, under 0.05 | 0.51 / 0.00 / 0.00 | 0.50 / 0.00 / 0.00 |
| Tidal Barrage | 106 list, under 0.05 | 0.18 / 0.00 / 0.00 | 0.21 / 0.00 / 0.00 |
| Smart Grid Backbone | 106 list, trigger, under 0.05 | 0.48 / 0.00 / 0.00 | 0.50 / 0.00 / 0.00 |
| Dispatch Control Room | 106 list, under 0.05 | 0.09 / 0.00 / 0.00 | 0.10 / 0.00 / 0.00 |
| Conglomerate Headquarters | 106 list, trigger, under 0.05 | 0.50 / 0.00 / 0.00 | 0.51 / 0.00 / 0.00 |
| Retained Earnings | 106 list, Capital, under 0.05 | 0.15 / 0.00 / 0.00 | 0.16 / 0.03 / 0.00 |
| Recursive Self-Improvement | 106 list, under 0.05 | 0.44 / 0.02 / 0.01 | 0.46 / 0.01 / 0.01 |
| Positron Storage Ring | 106 list, under 0.05 | 0.55 / 0.00 / 0.00 | 0.55 / 0.00 / 0.00 |
| Power to the People | 106 list, under 0.05 | 0.45 / 0.00 / 0.00 | 0.46 / 0.00 / 0.00 |
| Hyperscale Botnet | 106 list, under 0.05 | 0.52 / 0.01 / 0.00 | 0.52 / 0.01 / 0.00 |
| Payroll Automation | 106 list, under 0.05 | 0.21 / 0.20 / 0.00 | 0.19 / 0.00 / 0.00 |
| Capital Injection | Capital, under 0.05 | 0.10 / 0.00 / 0.00 | 0.11 / 0.00 / 0.00 |
| Vendor Financing | Capital, under 0.05 | 0.13 / 0.00 / 0.00 | 0.15 / 0.00 / 0.00 |
| Liquidity Event | Capital, under 0.05 | 0.08 / 0.00 / 0.00 | 0.08 / 0.00 / 0.00 |
| Collateralized Grid | Capital, under 0.05 | 0.48 / 0.00 / 0.00 | 0.47 / 0.01 / 0.00 |
| Interdisciplinary Institute | trigger, under 0.05 | 0.39 / 0.06 / 0.03 | 0.39 / 0.07 / 0.03 |
| Command-and-Control Grid | trigger, under 0.05 | 0.49 / 0.03 / 0.00 | 0.48 / 0.03 / 0.00 |
| Orbital Embassy | trigger, under 0.05 | 0.46 / 0.01 / 0.01 | 0.46 / 0.06 / 0.01 |
| Agency Directorate | trigger, under 0.05 | 0.36 / 0.00 / 0.00 | 0.38 / 0.17 / 0.03 |
| Adaptive Malware | under 0.05 | 0.19 / 0.03 / 0.00 | 0.19 / 0.03 / 0.00 |
| Arcology Foundry | under 0.05 | 0.41 / 0.03 / 0.00 | 0.40 / 0.03 / 0.00 |
| Autonomous Defense Grid | under 0.05 | 0.44 / 0.17 / 0.03 | 0.44 / 0.19 / 0.03 |
| Baseload Contract | under 0.05 | 0.13 / 0.14 / 0.00 | 0.15 / 0.15 / 0.00 |
| Beamed Power Constellation | under 0.05 | 0.56 / 0.00 / 0.00 | 0.57 / 0.01 / 0.00 |
| Bounty Board | under 0.05 | 0.13 / 0.13 / 0.05 | 0.15 / 0.15 / 0.06 |
| Capacitor Discharge | under 0.05 | 0.08 / 0.00 / 0.00 | 0.08 / 0.00 / 0.00 |
| Capacity Market | under 0.05 | 0.08 / 0.00 / 0.00 | 0.08 / 0.00 / 0.00 |
| Capital Projects Office | under 0.05 | 0.46 / 0.04 / 0.00 | 0.47 / 0.16 / 0.00 |
| Civic Watch | under 0.05 | 0.13 / 0.18 / 0.00 | 0.15 / 0.20 / 0.00 |
| Consulate Tower | under 0.05 | 0.11 / 0.01 / 0.00 | 0.11 / 0.01 / 0.00 |
| Cryptojacking Campaign | under 0.05 | 0.20 / 0.17 / 0.00 | 0.21 / 0.18 / 0.00 |
| Darknet Exchange | under 0.05 | 0.17 / 0.26 / 0.00 | 0.17 / 0.28 / 0.00 |
| Data Licensing Deal | under 0.05 | 0.51 / 0.48 / 0.03 | 0.51 / 0.46 / 0.03 |
| Dataset Scraper | under 0.05 | 0.11 / 0.03 / 0.00 | 0.11 / 0.03 / 0.00 |
| Deep State Ledger | under 0.05 | 0.60 / 0.02 / 0.00 | 0.62 / 0.03 / 0.01 |
| Defense Appropriations | under 0.05 | 0.13 / 0.03 / 0.01 | 0.15 / 0.05 / 0.01 |
| Demand Response Grid | under 0.05 | 0.11 / 0.28 / 0.00 | 0.11 / 0.28 / 0.00 |
| Economies of Scale | under 0.05 | 0.12 / 0.00 / 0.00 | 0.14 / 0.00 / 0.00 |
| Endowed Research Chair | under 0.05 | 0.10 / 0.07 / 0.00 | 0.11 / 0.07 / 0.00 |
| Exploit Broker | under 0.05 | 0.12 / 0.04 / 0.00 | 0.12 / 0.03 / 0.00 |
| Faraday Yard | under 0.05 | 0.17 / 0.22 / 0.00 | 0.16 / 0.22 / 0.00 |
| Force Protection Order | under 0.05 | 0.03 / 0.17 / 0.03 | 0.04 / 0.17 / 0.03 |
| Fusion Startup Stake | under 0.05 | 0.15 / 0.48 / 0.00 | 0.17 / 0.48 / 0.00 |
| Garrison Command | under 0.05 | 0.22 / 0.42 / 0.03 | 0.21 / 0.43 / 0.05 |
| Gigafactory Charter | under 0.05 | 0.15 / 0.25 / 0.01 | 0.13 / 0.22 / 0.00 |
| Grad Student Pool | under 0.05 | 0.10 / 0.24 / 0.01 | 0.10 / 0.24 / 0.01 |
| Grid Census | under 0.05 | 0.17 / 0.06 / 0.00 | 0.17 / 0.07 / 0.00 |
| Hardened Bunker Complex | under 0.05 | 0.15 / 0.35 / 0.00 | 0.16 / 0.37 / 0.00 |
| Homeland Security Act | under 0.05 | 0.50 / 0.02 / 0.00 | 0.53 / 0.01 / 0.00 |
| Honeypot Relay | under 0.05 | 0.13 / 0.34 / 0.00 | 0.12 / 0.33 / 0.00 |
| Interconnector Treaty | under 0.05 | 0.13 / 0.04 / 0.00 | 0.12 / 0.02 / 0.00 |
| Joint Task Force | under 0.05 | 0.19 / 0.07 / 0.01 | 0.19 / 0.07 / 0.01 |
| Kinetic Interdiction | under 0.05 | 0.54 / 0.02 / 0.00 | 0.54 / 0.01 / 0.00 |
| Lights-Out Assembly | under 0.05 | 0.19 / 0.39 / 0.00 | 0.16 / 0.37 / 0.00 |
| Market Stabilization Board | under 0.05 | 0.57 / 0.17 / 0.01 | 0.58 / 0.19 / 0.01 |
| Multiyear Defense Budget | under 0.05 | 0.12 / 0.27 / 0.01 | 0.12 / 0.26 / 0.01 |
| Neural Mirror | under 0.05 | 0.19 / 0.15 / 0.00 | 0.24 / 0.17 / 0.00 |
| Orbital Observatory Array | under 0.05 | 0.39 / 0.30 / 0.01 | 0.42 / 0.33 / 0.01 |
| Peace Conference | under 0.05 | 0.15 / 0.03 / 0.05 | 0.15 / 0.03 / 0.05 |
| Peace Dividend | under 0.05 | 0.10 / 0.03 / 0.00 | 0.10 / 0.02 / 0.00 |
| Peaker Turbines | under 0.05 | 0.05 / 0.20 / 0.00 | 0.06 / 0.19 / 0.00 |
| Plant Overdrive | under 0.05 | 0.12 / 0.00 / 0.00 | 0.10 / 0.00 / 0.00 |
| Popular Uprising | under 0.05 | 0.48 / 0.00 / 0.00 | 0.47 / 0.05 / 0.00 |
| Protocol Office | under 0.05 | 0.19 / 0.24 / 0.01 | 0.21 / 0.20 / 0.01 |
| Pumped Hydro Storage | under 0.05 | 0.14 / 0.13 / 0.01 | 0.14 / 0.12 / 0.01 |
| Quick Reaction Force | under 0.05 | 0.15 / 0.09 / 0.01 | 0.13 / 0.07 / 0.01 |
| Ransomware Syndicate | under 0.05 | 0.12 / 0.01 / 0.00 | 0.12 / 0.01 / 0.00 |
| Reclamation Crew | under 0.05 | 0.23 / 0.21 / 0.01 | 0.22 / 0.21 / 0.01 |
| Reconciliation Office | under 0.05 | 0.10 / 0.33 / 0.04 | 0.10 / 0.33 / 0.03 |
| Recruit Operatives | under 0.05 | 0.08 / 0.00 / 0.00 | 0.09 / 0.00 / 0.00 |
| Research Royalties | under 0.05 | 0.11 / 0.00 / 0.00 | 0.12 / 0.00 / 0.00 |
| Robotic Foundry | under 0.05 | 0.30 / 0.44 / 0.00 | 0.31 / 0.45 / 0.00 |
| Sample Repository | under 0.05 | 0.07 / 0.03 / 0.00 | 0.09 / 0.03 / 0.00 |
| Security Ops Floor | under 0.05 | 0.17 / 0.36 / 0.01 | 0.17 / 0.35 / 0.00 |
| Shared Threat Intel | under 0.05 | 0.12 / 0.21 / 0.04 | 0.15 / 0.24 / 0.03 |
| Singularity Exploit | under 0.05 | 0.37 / 0.01 / 0.00 | 0.39 / 0.01 / 0.01 |
| Standardized Tooling | under 0.05 | 0.21 / 0.30 / 0.00 | 0.18 / 0.26 / 0.00 |
| Substation Hub | under 0.05 | 0.11 / 0.06 / 0.00 | 0.09 / 0.06 / 0.00 |
| Surplus Auction | under 0.05 | 0.13 / 0.17 / 0.01 | 0.13 / 0.17 / 0.01 |
| University Spinout | under 0.05 | 0.17 / 0.05 / 0.00 | 0.18 / 0.05 / 0.00 |
| Vertical Monopoly | under 0.05 | 0.46 / 0.00 / 0.00 | 0.47 / 0.00 / 0.00 |
| Zero-Day Auction | under 0.05 | 0.48 / 0.01 / 0.00 | 0.51 / 0.01 / 0.00 |

Before, every card played under 0.05 a game: 87
Adaptive Malware, Agency Directorate, Arcology Foundry, Autonomous Defense Grid, Baseload Contract, Beamed Power Constellation, Bounty Board, Capacitor Discharge, Capacity Market, Capital Injection, Capital Projects Office, Citation Engine, Civic Watch, Collateralized Grid, Command-and-Control Grid, Conglomerate Headquarters, Consulate Tower, Continental Supergrid, Cryptojacking Campaign, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Defense Appropriations, Demand Response Grid, Dispatch Control Room, Dyson Swarm Tranche, Economies of Scale, Endowed Research Chair, Exploit Broker, Faraday Yard, Force Protection Order, Foundation Model Campus, Fusion Startup Stake, Garrison Command, Gigafactory Charter, Grad Student Pool, Grid Census, Hardened Bunker Complex, Homeland Security Act, Honeypot Relay, Hyperscale Botnet, Interconnector Treaty, Interdisciplinary Institute, Joint Task Force, Kinetic Interdiction, Lights-Out Assembly, Liquidity Event, Market Stabilization Board, Multiyear Defense Budget, Neural Mirror, Orbital Embassy, Orbital Mass Driver, Orbital Observatory Array, Payroll Automation, Peace Conference, Peace Dividend, Peaker Turbines, Plant Overdrive, Popular Uprising, Positron Storage Ring, Power to the People, Protocol Office, Pumped Hydro Storage, Quick Reaction Force, Ransomware Syndicate, Reclamation Crew, Reconciliation Office, Recruit Operatives, Recursive Self-Improvement, Research Royalties, Retained Earnings, Robotic Foundry, Sample Repository, Security Contractor HQ, Security Ops Floor, Shared Threat Intel, Singularity Exploit, Smart Grid Backbone, Standardized Tooling, Substation Hub, Surplus Auction, Tidal Barrage, University Spinout, Vendor Financing, Vertical Monopoly, Zero-Day Auction

Of those, played 0.05 a game or more after: 1
