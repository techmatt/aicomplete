# 076: cards kept and never played, and never kept, per agent

`python scripts/cards/play_read.py` over the band (29400, 29500), the gate and its closes (29600-29743) and the eval probe (29800), r32's frozen build. A card is listed when kept in more than 1% of the agent's games and played in none; AI-unique cards start in hand and are left out. The fixed policies restrict by design (`only_tag`, `events_only`, `free_only` keep and play within their rule), so their lists are long by construction; the planner's is the one to read.

## planner (804 games)

Kept in more than 1% of games and never played: 25

| card | tier | kind | games kept, % |
|---|---|---|---|
| Intrusion Lab | 1 | building | 55.2 |
| Demand Response Grid | 1 | building | 54.9 |
| Inference Cluster | 1 | building | 50.9 |
| Commons Grid | 1 | building | 50.0 |
| Honeypot Relay | 1 | building | 49.6 |
| Liaison Corps | 1 | building | 48.1 |
| Kill Team Roster | 1 | building | 47.4 |
| Hardpoint Lockdown | 1 | event | 46.5 |
| Agency Directorate | 2 | building | 46.1 |
| Open Assembly | 1 | building | 44.8 |
| Preprint Exchange | 1 | program | 41.2 |
| Simulation Cluster | 1 | building | 39.9 |
| Cryo-Compute Burn | 1 | event | 37.8 |
| Orbital Observatory Array | 2 | building | 36.2 |
| Credential Harvest | 1 | event | 34.5 |
| Sovereign Capital Fund | 2 | building | 30.0 |
| Deep State Ledger | 2 | building | 29.0 |
| Arc Furnace Complex | 2 | building | 24.1 |
| Quantum Decryptor | 1 | building | 20.0 |
| Planetary Assembly | 2 | building | 6.5 |
| Endowed Research Chair | 1 | program | 6.1 |
| Synthetic Data Forge | 1 | building | 4.2 |
| Policy Institute | 2 | building | 3.0 |
| Positron Storage Ring | 2 | building | 2.4 |
| Recursive Self-Improvement | 2 | program | 1.6 |

Never kept: 7: Capital Injection, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Foundation Model Campus, Manifesto Leak, Vendor Financing

## planner:abandon=1 (24 games)

Kept in more than 1% of games and never played: 51

| card | tier | kind | games kept, % |
|---|---|---|---|
| Open Assembly | 1 | building | 62.5 |
| Garrison Command | 1 | building | 62.5 |
| Security Ops Floor | 1 | building | 54.2 |
| Market Watchdog | 1 | building | 54.2 |
| Inference Cluster | 1 | building | 54.2 |
| Honeypot Relay | 1 | building | 54.2 |
| Lights-Out Assembly | 1 | building | 50.0 |
| Aegis Grid Lockdown | 2 | event | 50.0 |
| Shared Compute Grant | 1 | program | 45.8 |
| Neural Mirror | 1 | program | 45.8 |
| Kill Team Roster | 1 | building | 45.8 |
| Capacitor Discharge | 1 | event | 45.8 |
| Retained Earnings | 1 | building | 41.7 |
| Demand Response Grid | 1 | building | 41.7 |
| Darknet Exchange | 1 | building | 41.7 |
| Civic Watch | 1 | building | 41.7 |
| Aegis Dome Array | 2 | building | 41.7 |
| Safehouse Network | 1 | building | 37.5 |
| Faraday Yard | 1 | building | 37.5 |
| Commons Grid | 1 | building | 37.5 |
| Simulation Cluster | 1 | building | 33.3 |
| Sentry Drone Wing | 1 | building | 33.3 |
| Liaison Corps | 1 | building | 33.3 |
| Interceptor Battery | 1 | building | 33.3 |
| Grad Student Pool | 1 | building | 33.3 |
| Bonded Warehouse | 1 | building | 33.3 |
| Agency Field Station | 1 | building | 33.3 |
| Agency Directorate | 2 | building | 33.3 |
| Telemetry Archive | 1 | building | 29.2 |
| Sentinel Mesh | 1 | building | 29.2 |
| Expansion Fund | 1 | building | 29.2 |
| Intrusion Lab | 1 | building | 25.0 |
| Cryo-Compute Burn | 1 | event | 25.0 |
| Credential Harvest | 1 | event | 20.8 |
| Security Contractor HQ | 1 | building | 20.8 |
| Preprint Exchange | 1 | program | 20.8 |
| Deep State Ledger | 2 | building | 20.8 |
| Supply Chain Treaty | 1 | program | 16.7 |
| Storm Hardening | 1 | event | 16.7 |
| Orbital Observatory Array | 2 | building | 16.7 |
| Hardpoint Lockdown | 1 | event | 16.7 |
| Sovereign Capital Fund | 2 | building | 12.5 |
| Quantum Decryptor | 1 | building | 12.5 |
| Field Grant | 1 | program | 12.5 |
| Planetary Assembly | 2 | building | 8.3 |
| Orbital Mass Driver | 2 | building | 8.3 |
| Consulate Tower | 1 | building | 8.3 |
| Arc Furnace Complex | 2 | building | 8.3 |
| Positron Storage Ring | 2 | building | 4.2 |
| Orbital Embassy | 2 | building | 4.2 |
| Dyson Swarm Tranche | 2 | building | 4.2 |

Never kept: 25: Antitrust Action, Arcology Foundry, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Endowed Research Chair, Exabyte Archive, Foundation Model Campus, Great Liberation, Hyperscale Botnet, Liquidity Event, Manifesto Leak, Policy Institute, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Singularity Exploit, Summit Caucus, Synthetic Data Forge, Vendor Financing

## planner:always_buy_cubes=1 (144 games)

Kept in more than 1% of games and never played: 33

| card | tier | kind | games kept, % |
|---|---|---|---|
| Honeypot Relay | 1 | building | 67.4 |
| Aegis Dome Array | 2 | building | 65.3 |
| Grad Student Pool | 1 | building | 63.2 |
| Demand Response Grid | 1 | building | 62.5 |
| Commons Grid | 1 | building | 62.5 |
| Agency Field Station | 1 | building | 62.5 |
| Civic Watch | 1 | building | 61.8 |
| Intrusion Lab | 1 | building | 59.7 |
| Inference Cluster | 1 | building | 57.6 |
| Bonded Warehouse | 1 | building | 56.9 |
| Kill Team Roster | 1 | building | 56.2 |
| Capacitor Discharge | 1 | event | 56.2 |
| Hardpoint Lockdown | 1 | event | 55.6 |
| Liaison Corps | 1 | building | 54.9 |
| Open Assembly | 1 | building | 51.4 |
| Neural Mirror | 1 | program | 50.7 |
| Agency Directorate | 2 | building | 50.7 |
| Simulation Cluster | 1 | building | 49.3 |
| Sentry Drone Wing | 1 | building | 47.9 |
| Cryo-Compute Burn | 1 | event | 47.2 |
| Orbital Observatory Array | 2 | building | 45.8 |
| Retained Earnings | 1 | building | 44.4 |
| Credential Harvest | 1 | event | 38.9 |
| Telemetry Archive | 1 | building | 37.5 |
| Deep State Ledger | 2 | building | 31.9 |
| Sovereign Capital Fund | 2 | building | 30.6 |
| Quantum Decryptor | 1 | building | 23.6 |
| Arc Furnace Complex | 2 | building | 20.8 |
| Planetary Assembly | 2 | building | 7.6 |
| Endowed Research Chair | 1 | program | 7.6 |
| Synthetic Data Forge | 1 | building | 3.5 |
| Positron Storage Ring | 2 | building | 2.1 |
| Citation Engine | 1 | building | 1.4 |

Never kept: 14: Capital Injection, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Exabyte Archive, Foundation Model Campus, Hyperscale Botnet, Liquidity Event, Manifesto Leak, Policy Institute, Recruit Operatives, Vendor Financing

## planner:always_max_presence=1 (128 games)

Kept in more than 1% of games and never played: 36

| card | tier | kind | games kept, % |
|---|---|---|---|
| Honeypot Relay | 1 | building | 64.8 |
| Aegis Dome Array | 2 | building | 63.3 |
| Civic Watch | 1 | building | 60.9 |
| Open Assembly | 1 | building | 57.0 |
| Hardpoint Lockdown | 1 | event | 57.0 |
| Neural Mirror | 1 | program | 54.7 |
| Grad Student Pool | 1 | building | 54.7 |
| Commons Grid | 1 | building | 54.7 |
| Garrison Command | 1 | building | 53.9 |
| Intrusion Lab | 1 | building | 53.1 |
| Inference Cluster | 1 | building | 53.1 |
| Demand Response Grid | 1 | building | 53.1 |
| Agency Field Station | 1 | building | 53.1 |
| Darknet Exchange | 1 | building | 52.3 |
| Liaison Corps | 1 | building | 50.8 |
| Agency Directorate | 2 | building | 50.0 |
| Security Ops Floor | 1 | building | 48.4 |
| Kill Team Roster | 1 | building | 47.7 |
| Preprint Exchange | 1 | program | 46.9 |
| Sentry Drone Wing | 1 | building | 43.8 |
| Retained Earnings | 1 | building | 42.2 |
| Simulation Cluster | 1 | building | 40.6 |
| Orbital Observatory Array | 2 | building | 38.3 |
| Cryo-Compute Burn | 1 | event | 33.6 |
| Telemetry Archive | 1 | building | 32.8 |
| Sovereign Capital Fund | 2 | building | 32.0 |
| Credential Harvest | 1 | event | 29.7 |
| Deep State Ledger | 2 | building | 28.9 |
| Quantum Decryptor | 1 | building | 23.4 |
| Arc Furnace Complex | 2 | building | 18.0 |
| Endowed Research Chair | 1 | program | 13.3 |
| Planetary Assembly | 2 | building | 12.5 |
| Positron Storage Ring | 2 | building | 6.2 |
| Synthetic Data Forge | 1 | building | 4.7 |
| Policy Institute | 2 | building | 4.7 |
| Hyperscale Botnet | 2 | building | 2.3 |

Never kept: 12: Capital Injection, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Foundation Model Campus, Liquidity Event, Manifesto Leak, Recruit Operatives, Recursive Self-Improvement, Vendor Financing

## planner:ambassador_mass=1 (24 games)

Kept in more than 1% of games and never played: 77

| card | tier | kind | games kept, % |
|---|---|---|---|
| Open Assembly | 1 | building | 62.5 |
| Embassy Compound | 1 | building | 58.3 |
| Market Watchdog | 1 | building | 54.2 |
| Security Ops Floor | 1 | building | 50.0 |
| Inference Cluster | 1 | building | 50.0 |
| Garrison Command | 1 | building | 50.0 |
| Substation Hub | 1 | building | 45.8 |
| Payroll Automation | 1 | program | 45.8 |
| Shared Compute Grant | 1 | program | 41.7 |
| Lights-Out Assembly | 1 | building | 41.7 |
| Joint Security Compact | 1 | program | 41.7 |
| Honeypot Relay | 1 | building | 41.7 |
| Civic Watch | 1 | building | 41.7 |
| Sentry Drone Wing | 1 | building | 37.5 |
| Neural Mirror | 1 | program | 37.5 |
| Killswitch Directive | 2 | event | 37.5 |
| Hardened Bunker Complex | 1 | building | 37.5 |
| Grid Census | 1 | program | 37.5 |
| Fusion Startup Stake | 1 | program | 37.5 |
| Darknet Exchange | 1 | building | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Safehouse Network | 1 | building | 33.3 |
| Retained Earnings | 1 | building | 33.3 |
| Mutual Defense Pact | 1 | program | 33.3 |
| Demand Response Grid | 1 | building | 33.3 |
| Commons Grid | 1 | building | 33.3 |
| Ceasefire Monitors | 1 | program | 33.3 |
| Aegis Grid Lockdown | 2 | event | 33.3 |
| Simulation Cluster | 1 | building | 29.2 |
| Shared Exploit Kit | 1 | program | 29.2 |
| Prefab Plant Kit | 1 | program | 29.2 |
| Planetary Referendum | 2 | event | 29.2 |
| Peaker Turbines | 1 | program | 29.2 |
| Stress Test Suite | 1 | program | 29.2 |
| Liaison Corps | 1 | building | 25.0 |
| Intrusion Countermeasures | 1 | program | 25.0 |
| Interceptor Battery | 1 | building | 25.0 |
| Grad Student Pool | 1 | building | 25.0 |
| Faraday Yard | 1 | building | 25.0 |
| Bonded Warehouse | 1 | building | 25.0 |
| Zero-Knowledge Exploit | 1 | event | 20.8 |
| Sentinel Mesh | 1 | building | 20.8 |
| Literature Crawler | 1 | program | 20.8 |
| Intrusion Lab | 1 | building | 20.8 |
| Expansion Fund | 1 | building | 20.8 |
| Cryo-Compute Burn | 1 | event | 20.8 |
| Agency Field Station | 1 | building | 20.8 |
| Zero-Click Breach | 1 | program | 16.7 |
| Reclamation Crew | 1 | program | 16.7 |
| Shared Threat Intel | 1 | program | 16.7 |
| Preprint Exchange | 1 | program | 16.7 |
| Kill Team Roster | 1 | building | 16.7 |
| Treaty Grid Exchange | 1 | building | 12.5 |
| Storm Hardening | 1 | event | 12.5 |
| Credential Harvest | 1 | event | 12.5 |
| Peacekeeper Rotation | 1 | program | 12.5 |
| Hardpoint Lockdown | 1 | event | 12.5 |
| Coalition Charter | 2 | program | 12.5 |
| Back-Channel Talks | 1 | program | 12.5 |
| Amnesty Decree | 1 | event | 12.5 |
| Agency Directorate | 2 | building | 12.5 |
| Total Containment | 2 | event | 8.3 |
| Supply Chain Treaty | 1 | program | 8.3 |
| Quantum Decryptor | 1 | building | 8.3 |
| Martial Cordon | 2 | program | 8.3 |
| Market Stabilization Board | 2 | building | 8.3 |
| Logic Bomb | 1 | program | 8.3 |
| Hardened Conduits | 1 | program | 8.3 |
| Field Grant | 1 | program | 8.3 |
| Breach and Clear | 1 | event | 8.3 |
| Aegis Dome Array | 2 | building | 8.3 |
| Shadow Broker Scrub | 1 | program | 4.2 |
| Robotic Foundry | 1 | building | 4.2 |
| Open-Weights Release | 2 | program | 4.2 |
| Interconnector Treaty | 2 | program | 4.2 |
| Field Sensor Grid | 1 | program | 4.2 |
| Adversarial Inference Sweep | 2 | event | 4.2 |

Never kept: 52: Adaptive Malware, Antitrust Action, Arc Furnace Complex, Arcology Foundry, Autonomous Defense Grid, Beamed Power Constellation, Black Ice Citadel, Bounty Board, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Civic Trust Bureau, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Exodus Corridors, Foundation Model Campus, Great Liberation, Grid-Tied Smelter, Hyperscale Botnet, Kinetic Interdiction, Liberation Accords, Liquidity Event, Manifesto Leak, Node Severance, Orbital Embassy, Orbital Mass Driver, Orbital Observatory Array, Planetary Assembly, Policy Institute, Positron Storage Ring, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Security Contractor HQ, Sentinel Command, Signal Hijack, Singularity Exploit, Sovereign Capital Fund, Summit Caucus, Synthetic Data Forge, Telemetry Archive, Tidal Barrage, Vendor Financing, Vertical Monopoly

## planner:capstone_rush=1 (108 games)

Kept in more than 1% of games and never played: 40

| card | tier | kind | games kept, % |
|---|---|---|---|
| Foundation Model Campus | 2 | building | 100.0 |
| Continental Supergrid | 2 | building | 100.0 |
| Aegis Dome Array | 2 | building | 85.2 |
| Agency Directorate | 2 | building | 61.1 |
| Sovereign Capital Fund | 2 | building | 60.2 |
| Deep State Ledger | 2 | building | 53.7 |
| Orbital Observatory Array | 2 | building | 44.4 |
| Arc Furnace Complex | 2 | building | 44.4 |
| Inference Cluster | 1 | building | 26.9 |
| Civic Watch | 1 | building | 25.0 |
| Expansion Fund | 1 | building | 24.1 |
| Agency Field Station | 1 | building | 23.1 |
| Intrusion Lab | 1 | building | 22.2 |
| Capacitor Discharge | 1 | event | 22.2 |
| Open Assembly | 1 | building | 21.3 |
| Grad Student Pool | 1 | building | 21.3 |
| Honeypot Relay | 1 | building | 20.4 |
| Garrison Command | 1 | building | 20.4 |
| Faraday Yard | 1 | building | 20.4 |
| Demand Response Grid | 1 | building | 18.5 |
| Bonded Warehouse | 1 | building | 18.5 |
| Planetary Assembly | 2 | building | 17.6 |
| Hardpoint Lockdown | 1 | event | 17.6 |
| Commons Grid | 1 | building | 16.7 |
| Neural Mirror | 1 | program | 15.7 |
| Liaison Corps | 1 | building | 15.7 |
| Preprint Exchange | 1 | program | 13.9 |
| Kill Team Roster | 1 | building | 13.0 |
| Grid Census | 1 | program | 13.0 |
| Security Ops Floor | 1 | building | 12.0 |
| Safehouse Network | 1 | building | 12.0 |
| Sentry Drone Wing | 1 | building | 11.1 |
| Simulation Cluster | 1 | building | 7.4 |
| Retained Earnings | 1 | building | 7.4 |
| Cryo-Compute Burn | 1 | event | 7.4 |
| Orbital Embassy | 2 | building | 6.5 |
| Policy Institute | 2 | building | 3.7 |
| Telemetry Archive | 1 | building | 2.8 |
| Positron Storage Ring | 2 | building | 2.8 |
| Credential Harvest | 1 | event | 1.9 |

Never kept: 36: Adaptive Malware, Back-Channel Talks, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Field Grant, Field Sensor Grid, Force Protection Order, Grid-Tied Smelter, Hardened Conduits, Liquidity Event, Logic Bomb, Manifesto Leak, Quantum Decryptor, Reclamation Crew, Recruit Operatives, Robotic Foundry, Sample Repository, Security Contractor HQ, Shadow Broker Scrub, Signal Hijack, Summit Caucus, Synthetic Data Forge, Threat Hunt, Tidal Barrage, Vendor Financing

## planner:events_only=1 (24 games)

Kept in more than 1% of games and never played: 6

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Grid Lockdown | 2 | event | 70.8 |
| Capacitor Discharge | 1 | event | 50.0 |
| Cryo-Compute Burn | 1 | event | 37.5 |
| Storm Hardening | 1 | event | 25.0 |
| Hardpoint Lockdown | 1 | event | 25.0 |
| Credential Harvest | 1 | event | 16.7 |

Never kept: 137: Adaptive Malware, Adversarial Inference Sweep, Aegis Dome Array, Agency Directorate, Agency Field Station, Antitrust Action, Arc Furnace Complex, Arcology Foundry, Autonomous Defense Grid, Back-Channel Talks, Baseload Contract, Beamed Power Constellation, Black Ice Citadel, Blackout Script, Bonded Warehouse, Bounty Board, Breakthrough Lab, Breakthrough Leak, Brownout Relief, Bulk Silicon Procurement, Capital Injection, Ceasefire Monitors, Citation Engine, Civic Trust Bureau, Civic Watch, Coalition Charter, Collateralized Grid, Commons Grid, Consular Pouch, Consulate Tower, Continental Supergrid, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Demand Response Grid, Diplomatic Expulsion, Dyson Swarm Tranche, Early Warning Drill, Embassy Compound, Endowed Research Chair, Eviction Worm, Exabyte Archive, Exodus Corridors, Expansion Fund, Fact-Check Network, Faraday Yard, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Fusion Startup Stake, Garrison Command, Grad Student Pool, Great Liberation, Grid Census, Grid-Tied Smelter, Hardened Bunker Complex, Hardened Conduits, Honeypot Relay, Hyperscale Botnet, Inference Cluster, Interceptor Battery, Interconnector Treaty, Intrusion Countermeasures, Intrusion Lab, Joint Fabrication Pact, Joint Security Compact, Kill Team Roster, Kinetic Interdiction, Liaison Corps, Liberation Accords, Lights-Out Assembly, Liquidity Event, Literature Crawler, Logic Bomb, Manifesto Leak, Market Stabilization Board, Market Watchdog, Martial Cordon, Mutual Aid Pact, Mutual Defense Pact, Neural Mirror, Node Severance, Open Assembly, Open-Weights Release, Orbital Embassy, Orbital Mass Driver, Orbital Observatory Array, Payroll Automation, Peace Dividend, Peacekeeper Rotation, Peaker Turbines, Planetary Assembly, Policy Institute, Positron Storage Ring, Predictive Modeling, Prefab Plant Kit, Preprint Exchange, Quantum Decryptor, Reclamation Crew, Recruit Operatives, Recursive Self-Improvement, Red-Team Audit, Relay Mast Rollout, Retained Earnings, Robotic Foundry, Safehouse Network, Sample Repository, Security Contractor HQ, Security Ops Floor, Sentinel Command, Sentinel Mesh, Sentry Drone Wing, Shadow Broker Scrub, Shared Compute Grant, Shared Exploit Kit, Shared Grid Accord, Shared Threat Intel, Simulation Cluster, Singularity Exploit, Sovereign Capital Fund, Stress Test Suite, Substation Hub, Summit Caucus, Supply Chain Audit, Supply Chain Treaty, Synthetic Data Forge, Telemetry Archive, Threat Hunt, Tidal Barrage, Transmission Corridor, Treaty Grid Exchange, Truth Commission, Vendor Financing, Vertical Monopoly, Zero-Click Breach

## planner:free_build_rush=1 (192 games)

Kept in more than 1% of games and never played: 39

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 60.9 |
| Honeypot Relay | 1 | building | 56.8 |
| Intrusion Lab | 1 | building | 52.6 |
| Expansion Fund | 1 | building | 51.6 |
| Agency Field Station | 1 | building | 51.6 |
| Grad Student Pool | 1 | building | 51.0 |
| Garrison Command | 1 | building | 51.0 |
| Demand Response Grid | 1 | building | 50.0 |
| Inference Cluster | 1 | building | 48.4 |
| Commons Grid | 1 | building | 48.4 |
| Market Watchdog | 1 | building | 47.4 |
| Kill Team Roster | 1 | building | 47.4 |
| Hardpoint Lockdown | 1 | event | 46.4 |
| Open Assembly | 1 | building | 45.3 |
| Bonded Warehouse | 1 | building | 44.8 |
| Preprint Exchange | 1 | program | 44.3 |
| Liaison Corps | 1 | building | 43.8 |
| Agency Directorate | 2 | building | 42.7 |
| Neural Mirror | 1 | program | 41.7 |
| Security Ops Floor | 1 | building | 40.6 |
| Simulation Cluster | 1 | building | 40.1 |
| Sentry Drone Wing | 1 | building | 40.1 |
| Retained Earnings | 1 | building | 39.6 |
| Cryo-Compute Burn | 1 | event | 35.9 |
| Orbital Observatory Array | 2 | building | 33.3 |
| Telemetry Archive | 1 | building | 31.8 |
| Credential Harvest | 1 | event | 30.7 |
| Deep State Ledger | 2 | building | 29.2 |
| Quantum Decryptor | 1 | building | 21.4 |
| Sovereign Capital Fund | 2 | building | 10.9 |
| Arc Furnace Complex | 2 | building | 10.4 |
| Endowed Research Chair | 1 | program | 4.7 |
| Planetary Assembly | 2 | building | 2.6 |
| Positron Storage Ring | 2 | building | 2.1 |
| Orbital Embassy | 2 | building | 2.1 |
| Vertical Monopoly | 2 | building | 1.0 |
| Recursive Self-Improvement | 2 | program | 1.0 |
| Policy Institute | 2 | building | 1.0 |
| Great Liberation | 2 | event | 1.0 |

Never kept: 14: Breakthrough Lab, Capital Injection, Citation Engine, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Foundation Model Campus, Liquidity Event, Manifesto Leak, Recruit Operatives, Summit Caucus, Vendor Financing

## planner:free_only=1 (24 games)

Kept in more than 1% of games and never played: 13

| card | tier | kind | games kept, % |
|---|---|---|---|
| Open Assembly | 1 | building | 58.3 |
| Brownout Relief | 1 | program | 58.3 |
| Safehouse Network | 1 | building | 37.5 |
| Civic Watch | 1 | building | 33.3 |
| Aegis Grid Lockdown | 2 | event | 29.2 |
| Commons Grid | 1 | building | 25.0 |
| Liaison Corps | 1 | building | 12.5 |
| Aegis Dome Array | 2 | building | 12.5 |
| Treaty Grid Exchange | 1 | building | 8.3 |
| Supply Chain Treaty | 1 | program | 8.3 |
| Transmission Corridor | 1 | program | 4.2 |
| Storm Hardening | 1 | event | 4.2 |
| Planetary Assembly | 2 | building | 4.2 |

Never kept: 119: Adaptive Malware, Adversarial Inference Sweep, Agency Directorate, Agency Field Station, Antitrust Action, Arc Furnace Complex, Arcology Foundry, Autonomous Defense Grid, Baseload Contract, Beamed Power Constellation, Black Ice Citadel, Blackout Script, Bonded Warehouse, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Bridge Loan, Bulk Silicon Procurement, Capacitor Discharge, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Continental Supergrid, Credential Harvest, Cryo-Compute Burn, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Demand Response Grid, Dyson Swarm Tranche, Early Warning Drill, Endowed Research Chair, Eviction Worm, Exabyte Archive, Expansion Fund, Faraday Yard, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Fusion Startup Stake, Garrison Command, Grad Student Pool, Great Liberation, Grid Census, Grid-Tied Smelter, Hardened Bunker Complex, Hardened Conduits, Hardpoint Lockdown, Honeypot Relay, Hyperscale Botnet, Inference Cluster, Interceptor Battery, Interconnector Treaty, Intrusion Countermeasures, Intrusion Lab, Kill Team Roster, Killswitch Directive, Kinetic Interdiction, Lights-Out Assembly, Liquidity Event, Literature Crawler, Logic Bomb, Manifesto Leak, Market Stabilization Board, Market Watchdog, Neural Mirror, Node Severance, Open-Weights Release, Orbital Embassy, Orbital Mass Driver, Orbital Observatory Array, Payroll Automation, Peaker Turbines, Policy Institute, Positron Storage Ring, Predictive Modeling, Prefab Plant Kit, Preprint Exchange, Quantum Decryptor, Reclamation Crew, Recruit Operatives, Recursive Self-Improvement, Red-Team Audit, Relay Mast Rollout, Retained Earnings, Robotic Foundry, Sample Repository, Security Contractor HQ, Security Ops Floor, Sentinel Command, Sentinel Mesh, Sentry Drone Wing, Shadow Broker Scrub, Shared Compute Grant, Shared Exploit Kit, Shared Threat Intel, Signal Hijack, Simulation Cluster, Singularity Exploit, Sovereign Capital Fund, Stress Test Suite, Substation Hub, Summit Caucus, Supply Chain Audit, Surplus Auction, Synthetic Data Forge, Telemetry Archive, Threat Hunt, Tidal Barrage, Total Containment, Truth Commission, Vendor Financing, Vertical Monopoly, Zero-Click Breach, Zero-Knowledge Exploit

## planner:hoard_insight=3 (12 games)

Kept in more than 1% of games and never played: 61

| card | tier | kind | games kept, % |
|---|---|---|---|
| Market Watchdog | 1 | building | 75.0 |
| Brownout Relief | 1 | program | 66.7 |
| Simulation Cluster | 1 | building | 58.3 |
| Sentry Drone Wing | 1 | building | 58.3 |
| Security Ops Floor | 1 | building | 58.3 |
| Lights-Out Assembly | 1 | building | 58.3 |
| Inference Cluster | 1 | building | 58.3 |
| Darknet Exchange | 1 | building | 58.3 |
| Civic Watch | 1 | building | 58.3 |
| Safehouse Network | 1 | building | 50.0 |
| Open Assembly | 1 | building | 50.0 |
| Mutual Defense Pact | 1 | program | 50.0 |
| Honeypot Relay | 1 | building | 50.0 |
| Faraday Yard | 1 | building | 50.0 |
| Demand Response Grid | 1 | building | 50.0 |
| Aegis Dome Array | 2 | building | 50.0 |
| Telemetry Archive | 1 | building | 41.7 |
| Retained Earnings | 1 | building | 41.7 |
| Neural Mirror | 1 | program | 41.7 |
| Liaison Corps | 1 | building | 41.7 |
| Grad Student Pool | 1 | building | 41.7 |
| Cryo-Compute Burn | 1 | event | 41.7 |
| Commons Grid | 1 | building | 41.7 |
| Capacitor Discharge | 1 | event | 41.7 |
| Bonded Warehouse | 1 | building | 41.7 |
| Aegis Grid Lockdown | 2 | event | 41.7 |
| Treaty Grid Exchange | 1 | building | 33.3 |
| Storm Hardening | 1 | event | 33.3 |
| Shared Threat Intel | 1 | program | 33.3 |
| Prefab Plant Kit | 1 | program | 33.3 |
| Peaker Turbines | 1 | program | 33.3 |
| Kill Team Roster | 1 | building | 33.3 |
| Intrusion Lab | 1 | building | 33.3 |
| Interceptor Battery | 1 | building | 33.3 |
| Hardpoint Lockdown | 1 | event | 33.3 |
| Garrison Command | 1 | building | 33.3 |
| Expansion Fund | 1 | building | 33.3 |
| Agency Field Station | 1 | building | 33.3 |
| Credential Harvest | 1 | event | 25.0 |
| Preprint Exchange | 1 | program | 25.0 |
| Arcology Foundry | 2 | building | 25.0 |
| Arc Furnace Complex | 2 | building | 25.0 |
| Agency Directorate | 2 | building | 25.0 |
| Hardened Conduits | 1 | program | 16.7 |
| Transmission Corridor | 1 | program | 8.3 |
| Threat Hunt | 1 | program | 8.3 |
| Supply Chain Treaty | 1 | program | 8.3 |
| Supply Chain Audit | 1 | program | 8.3 |
| Sovereign Capital Fund | 2 | building | 8.3 |
| Shadow Broker Scrub | 1 | program | 8.3 |
| Sentinel Mesh | 1 | building | 8.3 |
| Security Contractor HQ | 1 | building | 8.3 |
| Positron Storage Ring | 2 | building | 8.3 |
| Orbital Observatory Array | 2 | building | 8.3 |
| Orbital Mass Driver | 2 | building | 8.3 |
| Market Stabilization Board | 2 | building | 8.3 |
| Force Protection Order | 1 | program | 8.3 |
| Field Grant | 1 | program | 8.3 |
| Deep State Ledger | 2 | building | 8.3 |
| Civic Trust Bureau | 2 | building | 8.3 |
| Antitrust Action | 1 | program | 8.3 |

Never kept: 36: Adaptive Malware, Adversarial Inference Sweep, Beamed Power Constellation, Bounty Board, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Field Sensor Grid, Foundation Model Campus, Great Liberation, Grid-Tied Smelter, Hyperscale Botnet, Liquidity Event, Manifesto Leak, Orbital Embassy, Planetary Assembly, Policy Institute, Quantum Decryptor, Recruit Operatives, Recursive Self-Improvement, Robotic Foundry, Sample Repository, Singularity Exploit, Summit Caucus, Synthetic Data Forge, Tidal Barrage, Vendor Financing

## planner:home_only=1 (192 games)

Kept in more than 1% of games and never played: 34

| card | tier | kind | games kept, % |
|---|---|---|---|
| Honeypot Relay | 1 | building | 57.8 |
| Capacitor Discharge | 1 | event | 53.1 |
| Grad Student Pool | 1 | building | 52.6 |
| Lights-Out Assembly | 1 | building | 51.6 |
| Commons Grid | 1 | building | 50.0 |
| Kill Team Roster | 1 | building | 49.0 |
| Agency Field Station | 1 | building | 47.9 |
| Intrusion Lab | 1 | building | 47.4 |
| Inference Cluster | 1 | building | 47.4 |
| Demand Response Grid | 1 | building | 46.9 |
| Bonded Warehouse | 1 | building | 46.9 |
| Open Assembly | 1 | building | 46.4 |
| Hardpoint Lockdown | 1 | event | 43.8 |
| Liaison Corps | 1 | building | 43.2 |
| Sentry Drone Wing | 1 | building | 41.7 |
| Retained Earnings | 1 | building | 41.7 |
| Simulation Cluster | 1 | building | 41.1 |
| Orbital Observatory Array | 2 | building | 41.1 |
| Agency Directorate | 2 | building | 39.6 |
| Cryo-Compute Burn | 1 | event | 34.4 |
| Telemetry Archive | 1 | building | 31.8 |
| Deep State Ledger | 2 | building | 31.8 |
| Credential Harvest | 1 | event | 29.7 |
| Sovereign Capital Fund | 2 | building | 22.9 |
| Quantum Decryptor | 1 | building | 21.4 |
| Arc Furnace Complex | 2 | building | 13.5 |
| Planetary Assembly | 2 | building | 4.2 |
| Endowed Research Chair | 1 | program | 4.2 |
| Recursive Self-Improvement | 2 | program | 2.6 |
| Consulate Tower | 1 | building | 2.6 |
| Positron Storage Ring | 2 | building | 2.1 |
| Hyperscale Botnet | 2 | building | 2.1 |
| Synthetic Data Forge | 1 | building | 1.6 |
| Policy Institute | 2 | building | 1.6 |

Never kept: 12: Capital Injection, Citation Engine, Collateralized Grid, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Foundation Model Campus, Grid-Tied Smelter, Liquidity Event, Manifesto Leak, Recruit Operatives, Vendor Financing

## planner:ignore_threats=1 (24 games)

Kept in more than 1% of games and never played: 46

| card | tier | kind | games kept, % |
|---|---|---|---|
| Open Assembly | 1 | building | 50.0 |
| Inference Cluster | 1 | building | 50.0 |
| Honeypot Relay | 1 | building | 45.8 |
| Darknet Exchange | 1 | building | 41.7 |
| Shared Compute Grant | 1 | program | 37.5 |
| Safehouse Network | 1 | building | 37.5 |
| Payroll Automation | 1 | program | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Peaker Turbines | 1 | program | 33.3 |
| Aegis Dome Array | 2 | building | 33.3 |
| Retained Earnings | 1 | building | 29.2 |
| Neural Mirror | 1 | program | 29.2 |
| Lights-Out Assembly | 1 | building | 29.2 |
| Intrusion Lab | 1 | building | 29.2 |
| Grid Census | 1 | program | 29.2 |
| Grad Student Pool | 1 | building | 29.2 |
| Faraday Yard | 1 | building | 29.2 |
| Commons Grid | 1 | building | 29.2 |
| Bonded Warehouse | 1 | building | 29.2 |
| Agency Field Station | 1 | building | 29.2 |
| Simulation Cluster | 1 | building | 20.8 |
| Sentry Drone Wing | 1 | building | 20.8 |
| Kill Team Roster | 1 | building | 20.8 |
| Expansion Fund | 1 | building | 20.8 |
| Demand Response Grid | 1 | building | 20.8 |
| Telemetry Archive | 1 | building | 16.7 |
| Mutual Defense Pact | 1 | program | 16.7 |
| Interceptor Battery | 1 | building | 16.7 |
| Treaty Grid Exchange | 1 | building | 12.5 |
| Supply Chain Treaty | 1 | program | 12.5 |
| Quantum Decryptor | 1 | building | 12.5 |
| Liaison Corps | 1 | building | 12.5 |
| Hardpoint Lockdown | 1 | event | 12.5 |
| Cryo-Compute Burn | 1 | event | 12.5 |
| Credential Harvest | 1 | event | 8.3 |
| Preprint Exchange | 1 | program | 8.3 |
| Open-Weights Release | 2 | program | 8.3 |
| Deep State Ledger | 2 | building | 8.3 |
| Agency Directorate | 2 | building | 8.3 |
| Storm Hardening | 1 | event | 4.2 |
| Sovereign Capital Fund | 2 | building | 4.2 |
| Robotic Foundry | 1 | building | 4.2 |
| Orbital Observatory Array | 2 | building | 4.2 |
| Hardened Conduits | 1 | program | 4.2 |
| Black Ice Citadel | 2 | building | 4.2 |
| Arc Furnace Complex | 2 | building | 4.2 |

Never kept: 61: Adaptive Malware, Adversarial Inference Sweep, Antitrust Action, Arcology Foundry, Autonomous Defense Grid, Beamed Power Constellation, Bounty Board, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Civic Trust Bureau, Civic Watch, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Early Warning Drill, Endowed Research Chair, Exabyte Archive, Fact-Check Network, Force Protection Order, Foundation Model Campus, Garrison Command, Great Liberation, Grid-Tied Smelter, Hyperscale Botnet, Kinetic Interdiction, Liquidity Event, Logic Bomb, Manifesto Leak, Market Stabilization Board, Market Watchdog, Node Severance, Orbital Embassy, Orbital Mass Driver, Planetary Assembly, Policy Institute, Positron Storage Ring, Predictive Modeling, Recruit Operatives, Recursive Self-Improvement, Red-Team Audit, Sample Repository, Security Contractor HQ, Security Ops Floor, Sentinel Command, Sentinel Mesh, Shared Grid Accord, Shared Threat Intel, Singularity Exploit, Summit Caucus, Supply Chain Audit, Synthetic Data Forge, Threat Hunt, Tidal Barrage, Total Containment, Truth Commission, Vendor Financing

## planner:keep_all=1 (24 games)

Kept in more than 1% of games and never played: 114

| card | tier | kind | games kept, % |
|---|---|---|---|
| Liberation Accords | 2 | program | 75.0 |
| Open Assembly | 1 | building | 70.8 |
| Shared Compute Grant | 1 | program | 66.7 |
| Orbital Embassy | 2 | building | 66.7 |
| Tidal Barrage | 1 | building | 62.5 |
| Peaker Turbines | 1 | program | 58.3 |
| Hyperscale Botnet | 2 | building | 58.3 |
| Market Watchdog | 1 | building | 54.2 |
| Inference Cluster | 1 | building | 54.2 |
| Exabyte Archive | 2 | building | 54.2 |
| Breakthrough Leak | 1 | event | 54.2 |
| Vertical Monopoly | 2 | building | 50.0 |
| Singularity Exploit | 2 | program | 50.0 |
| Recruit Operatives | 1 | program | 50.0 |
| Policy Institute | 2 | building | 50.0 |
| Foundation Model Campus | 2 | building | 50.0 |
| Force Protection Order | 1 | program | 50.0 |
| Data Licensing Deal | 2 | program | 50.0 |
| Robotic Foundry | 1 | building | 45.8 |
| Prefab Plant Kit | 1 | program | 45.8 |
| Orbital Observatory Array | 2 | building | 45.8 |
| Neural Mirror | 1 | program | 45.8 |
| Killswitch Directive | 2 | event | 45.8 |
| Joint Security Compact | 1 | program | 45.8 |
| Honeypot Relay | 1 | building | 45.8 |
| Garrison Command | 1 | building | 45.8 |
| Endowed Research Chair | 1 | program | 45.8 |
| Darknet Exchange | 1 | building | 45.8 |
| Arc Furnace Complex | 2 | building | 45.8 |
| Aegis Dome Array | 2 | building | 45.8 |
| Total Containment | 2 | event | 41.7 |
| Sovereign Capital Fund | 2 | building | 41.7 |
| Simulation Cluster | 1 | building | 41.7 |
| Retained Earnings | 1 | building | 41.7 |
| Capital Injection | 1 | program | 41.7 |
| Market Stabilization Board | 2 | building | 41.7 |
| Interconnector Treaty | 2 | program | 41.7 |
| Field Sensor Grid | 1 | program | 41.7 |
| Dyson Swarm Tranche | 2 | building | 41.7 |
| Cryo-Compute Burn | 1 | event | 41.7 |
| Commons Grid | 1 | building | 41.7 |
| Civic Trust Bureau | 2 | building | 41.7 |
| Supply Chain Treaty | 1 | program | 37.5 |
| Reclamation Crew | 1 | program | 37.5 |
| Safehouse Network | 1 | building | 37.5 |
| Payroll Automation | 1 | program | 37.5 |
| Open-Weights Release | 2 | program | 37.5 |
| Node Severance | 2 | event | 37.5 |
| Hardened Bunker Complex | 1 | building | 37.5 |
| Deep State Ledger | 2 | building | 37.5 |
| Consulate Tower | 1 | building | 37.5 |
| Civic Watch | 1 | building | 37.5 |
| Citation Engine | 1 | building | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Agency Field Station | 1 | building | 37.5 |
| Aegis Grid Lockdown | 2 | event | 37.5 |
| Supply Chain Audit | 1 | program | 33.3 |
| Credential Harvest | 1 | event | 33.3 |
| Sentinel Mesh | 1 | building | 33.3 |
| Security Ops Floor | 1 | building | 33.3 |
| Positron Storage Ring | 2 | building | 33.3 |
| Logic Bomb | 1 | program | 33.3 |
| Lights-Out Assembly | 1 | building | 33.3 |
| Fusion Startup Stake | 1 | program | 33.3 |
| Dataset Scraper | 1 | program | 33.3 |
| Continental Supergrid | 2 | building | 33.3 |
| Breakthrough Lab | 2 | building | 33.3 |
| Arcology Foundry | 2 | building | 33.3 |
| Adaptive Malware | 1 | program | 33.3 |
| Treaty Grid Exchange | 1 | building | 29.2 |
| Shared Grid Accord | 1 | program | 29.2 |
| Sentry Drone Wing | 1 | building | 29.2 |
| Sentinel Command | 2 | building | 29.2 |
| Security Contractor HQ | 1 | building | 29.2 |
| Recursive Self-Improvement | 2 | program | 29.2 |
| Grid Census | 1 | program | 29.2 |
| Great Liberation | 2 | event | 29.2 |
| Field Grant | 1 | program | 29.2 |
| Faraday Yard | 1 | building | 29.2 |
| Exodus Corridors | 2 | program | 29.2 |
| Bonded Warehouse | 1 | building | 29.2 |
| Black Ice Citadel | 2 | building | 29.2 |
| Beamed Power Constellation | 2 | program | 29.2 |
| Agency Directorate | 2 | building | 29.2 |
| Zero-Click Breach | 1 | program | 25.0 |
| Liaison Corps | 1 | building | 25.0 |
| Kill Team Roster | 1 | building | 25.0 |
| Demand Response Grid | 1 | building | 25.0 |
| Collateralized Grid | 1 | program | 25.0 |
| Zero-Knowledge Exploit | 1 | event | 20.8 |
| Vendor Financing | 1 | program | 20.8 |
| Telemetry Archive | 1 | building | 20.8 |
| Summit Caucus | 1 | program | 20.8 |
| Preprint Exchange | 1 | program | 20.8 |
| Literature Crawler | 1 | program | 20.8 |
| Kinetic Interdiction | 2 | event | 20.8 |
| Interceptor Battery | 1 | building | 20.8 |
| Expansion Fund | 1 | building | 20.8 |
| Stress Test Suite | 1 | program | 20.8 |
| Storm Hardening | 1 | event | 16.7 |
| Shadow Broker Scrub | 1 | program | 16.7 |
| Sample Repository | 1 | building | 16.7 |
| Quantum Decryptor | 1 | building | 16.7 |
| Planetary Assembly | 2 | building | 16.7 |
| Intrusion Lab | 1 | building | 16.7 |
| Grad Student Pool | 1 | building | 16.7 |
| Antitrust Action | 1 | program | 16.7 |
| Orbital Mass Driver | 2 | building | 12.5 |
| Synthetic Data Forge | 1 | building | 8.3 |
| Manifesto Leak | 1 | event | 8.3 |
| Liquidity Event | 1 | event | 8.3 |
| Hardpoint Lockdown | 1 | event | 8.3 |
| Grid-Tied Smelter | 1 | building | 8.3 |
| Predictive Modeling | 1 | program | 4.2 |

Never kept: 0

## planner:keep_k=2 (28 games)

Kept in more than 1% of games and never played: 67

| card | tier | kind | games kept, % |
|---|---|---|---|
| Data Licensing Deal | 2 | program | 85.7 |
| Policy Institute | 2 | building | 75.0 |
| Lights-Out Assembly | 1 | building | 60.7 |
| Deep State Ledger | 2 | building | 57.1 |
| Market Watchdog | 1 | building | 53.6 |
| Capacitor Discharge | 1 | event | 53.6 |
| Security Ops Floor | 1 | building | 50.0 |
| Credential Harvest | 1 | event | 46.4 |
| Exabyte Archive | 2 | building | 46.4 |
| Agency Field Station | 1 | building | 46.4 |
| Hardpoint Lockdown | 1 | event | 42.9 |
| Garrison Command | 1 | building | 42.9 |
| Cryo-Compute Burn | 1 | event | 42.9 |
| Sentry Drone Wing | 1 | building | 39.3 |
| Safehouse Network | 1 | building | 39.3 |
| Recruit Operatives | 1 | program | 39.3 |
| Neural Mirror | 1 | program | 35.7 |
| Darknet Exchange | 1 | building | 35.7 |
| Telemetry Archive | 1 | building | 32.1 |
| Shared Compute Grant | 1 | program | 32.1 |
| Manifesto Leak | 1 | event | 32.1 |
| Liaison Corps | 1 | building | 32.1 |
| Grad Student Pool | 1 | building | 32.1 |
| Expansion Fund | 1 | building | 32.1 |
| Civic Watch | 1 | building | 32.1 |
| Bonded Warehouse | 1 | building | 32.1 |
| Arc Furnace Complex | 2 | building | 32.1 |
| Sovereign Capital Fund | 2 | building | 28.6 |
| Kill Team Roster | 1 | building | 28.6 |
| Capital Injection | 1 | program | 25.0 |
| Liquidity Event | 1 | event | 25.0 |
| Demand Response Grid | 1 | building | 25.0 |
| Collateralized Grid | 1 | program | 25.0 |
| Vendor Financing | 1 | program | 21.4 |
| Tidal Barrage | 1 | building | 21.4 |
| Sample Repository | 1 | building | 21.4 |
| Retained Earnings | 1 | building | 21.4 |
| Quantum Decryptor | 1 | building | 21.4 |
| Preprint Exchange | 1 | program | 21.4 |
| Endowed Research Chair | 1 | program | 21.4 |
| Dataset Scraper | 1 | program | 21.4 |
| Continental Supergrid | 2 | building | 21.4 |
| Consulate Tower | 1 | building | 21.4 |
| Breakthrough Lab | 2 | building | 21.4 |
| Supply Chain Treaty | 1 | program | 17.9 |
| Killswitch Directive | 2 | event | 17.9 |
| Honeypot Relay | 1 | building | 17.9 |
| Simulation Cluster | 1 | building | 14.3 |
| Open Assembly | 1 | building | 14.3 |
| Hyperscale Botnet | 2 | building | 14.3 |
| Grid-Tied Smelter | 1 | building | 14.3 |
| Beamed Power Constellation | 2 | program | 14.3 |
| Synthetic Data Forge | 1 | building | 10.7 |
| Intrusion Lab | 1 | building | 10.7 |
| Interconnector Treaty | 2 | program | 10.7 |
| Commons Grid | 1 | building | 10.7 |
| Citation Engine | 1 | building | 10.7 |
| Agency Directorate | 2 | building | 10.7 |
| Security Contractor HQ | 1 | building | 7.1 |
| Positron Storage Ring | 2 | building | 7.1 |
| Planetary Assembly | 2 | building | 7.1 |
| Liberation Accords | 2 | program | 7.1 |
| Inference Cluster | 1 | building | 7.1 |
| Foundation Model Campus | 2 | building | 7.1 |
| Vertical Monopoly | 2 | building | 3.6 |
| Orbital Observatory Array | 2 | building | 3.6 |
| Arcology Foundry | 2 | building | 3.6 |

Never kept: 7: Dyson Swarm Tranche, Great Liberation, Kinetic Interdiction, Orbital Embassy, Orbital Mass Driver, Recursive Self-Improvement, Singularity Exploit

## planner:never_convert=1 (192 games)

Kept in more than 1% of games and never played: 33

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 59.4 |
| Honeypot Relay | 1 | building | 57.8 |
| Grad Student Pool | 1 | building | 57.3 |
| Bonded Warehouse | 1 | building | 56.2 |
| Commons Grid | 1 | building | 54.7 |
| Capacitor Discharge | 1 | event | 54.7 |
| Demand Response Grid | 1 | building | 54.2 |
| Kill Team Roster | 1 | building | 52.1 |
| Inference Cluster | 1 | building | 52.1 |
| Hardpoint Lockdown | 1 | event | 49.0 |
| Simulation Cluster | 1 | building | 48.4 |
| Intrusion Lab | 1 | building | 48.4 |
| Preprint Exchange | 1 | program | 47.9 |
| Open Assembly | 1 | building | 47.9 |
| Neural Mirror | 1 | program | 47.4 |
| Orbital Observatory Array | 2 | building | 44.3 |
| Liaison Corps | 1 | building | 44.3 |
| Retained Earnings | 1 | building | 43.8 |
| Agency Directorate | 2 | building | 42.2 |
| Cryo-Compute Burn | 1 | event | 37.0 |
| Telemetry Archive | 1 | building | 36.5 |
| Credential Harvest | 1 | event | 35.4 |
| Sovereign Capital Fund | 2 | building | 29.7 |
| Deep State Ledger | 2 | building | 29.7 |
| Quantum Decryptor | 1 | building | 21.4 |
| Arc Furnace Complex | 2 | building | 18.2 |
| Endowed Research Chair | 1 | program | 7.3 |
| Planetary Assembly | 2 | building | 5.7 |
| Synthetic Data Forge | 1 | building | 5.2 |
| Positron Storage Ring | 2 | building | 4.2 |
| Policy Institute | 2 | building | 3.1 |
| Sample Repository | 1 | building | 1.6 |
| Hyperscale Botnet | 2 | building | 1.0 |

Never kept: 18: Breakthrough Lab, Capital Injection, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Exabyte Archive, Foundation Model Campus, Great Liberation, Kinetic Interdiction, Liquidity Event, Manifesto Leak, Orbital Embassy, Recruit Operatives, Recursive Self-Improvement, Singularity Exploit, Vendor Financing

## planner:never_remove=1 (116 games)

Kept in more than 1% of games and never played: 34

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 57.8 |
| Honeypot Relay | 1 | building | 50.9 |
| Agency Field Station | 1 | building | 49.1 |
| Grad Student Pool | 1 | building | 47.4 |
| Demand Response Grid | 1 | building | 47.4 |
| Commons Grid | 1 | building | 47.4 |
| Orbital Observatory Array | 2 | building | 46.6 |
| Inference Cluster | 1 | building | 46.6 |
| Capacitor Discharge | 1 | event | 45.7 |
| Kill Team Roster | 1 | building | 44.8 |
| Open Assembly | 1 | building | 44.0 |
| Intrusion Lab | 1 | building | 43.1 |
| Hardpoint Lockdown | 1 | event | 43.1 |
| Bonded Warehouse | 1 | building | 43.1 |
| Agency Directorate | 2 | building | 43.1 |
| Sentry Drone Wing | 1 | building | 40.5 |
| Neural Mirror | 1 | program | 40.5 |
| Simulation Cluster | 1 | building | 38.8 |
| Preprint Exchange | 1 | program | 38.8 |
| Liaison Corps | 1 | building | 35.3 |
| Sovereign Capital Fund | 2 | building | 30.2 |
| Cryo-Compute Burn | 1 | event | 30.2 |
| Credential Harvest | 1 | event | 28.4 |
| Arc Furnace Complex | 2 | building | 28.4 |
| Deep State Ledger | 2 | building | 25.9 |
| Telemetry Archive | 1 | building | 25.0 |
| Quantum Decryptor | 1 | building | 14.7 |
| Planetary Assembly | 2 | building | 5.2 |
| Positron Storage Ring | 2 | building | 2.6 |
| Orbital Embassy | 2 | building | 2.6 |
| Endowed Research Chair | 1 | program | 2.6 |
| Synthetic Data Forge | 1 | building | 1.7 |
| Recursive Self-Improvement | 2 | program | 1.7 |
| Policy Institute | 2 | building | 1.7 |

Never kept: 17: Breakthrough Lab, Breakthrough Leak, Capital Injection, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Field Sensor Grid, Foundation Model Campus, Liquidity Event, Manifesto Leak, Open-Weights Release, Sample Repository, Shadow Broker Scrub, Tidal Barrage, Vendor Financing

## planner:never_sacrifice=1 (192 games)

Kept in more than 1% of games and never played: 33

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 62.5 |
| Demand Response Grid | 1 | building | 54.7 |
| Commons Grid | 1 | building | 53.6 |
| Kill Team Roster | 1 | building | 52.6 |
| Grad Student Pool | 1 | building | 52.6 |
| Inference Cluster | 1 | building | 52.1 |
| Intrusion Lab | 1 | building | 48.4 |
| Hardpoint Lockdown | 1 | event | 48.4 |
| Open Assembly | 1 | building | 46.4 |
| Neural Mirror | 1 | program | 44.3 |
| Bonded Warehouse | 1 | building | 44.3 |
| Liaison Corps | 1 | building | 43.8 |
| Simulation Cluster | 1 | building | 41.7 |
| Retained Earnings | 1 | building | 41.7 |
| Orbital Observatory Array | 2 | building | 40.6 |
| Agency Directorate | 2 | building | 40.1 |
| Telemetry Archive | 1 | building | 37.0 |
| Sovereign Capital Fund | 2 | building | 37.0 |
| Cryo-Compute Burn | 1 | event | 33.9 |
| Credential Harvest | 1 | event | 28.6 |
| Deep State Ledger | 2 | building | 27.1 |
| Quantum Decryptor | 1 | building | 24.5 |
| Arc Furnace Complex | 2 | building | 21.4 |
| Planetary Assembly | 2 | building | 9.4 |
| Synthetic Data Forge | 1 | building | 5.7 |
| Positron Storage Ring | 2 | building | 5.2 |
| Policy Institute | 2 | building | 4.7 |
| Endowed Research Chair | 1 | program | 3.6 |
| Sample Repository | 1 | building | 1.6 |
| Hyperscale Botnet | 2 | building | 1.6 |
| Orbital Embassy | 2 | building | 1.0 |
| Capital Injection | 1 | program | 1.0 |
| Dyson Swarm Tranche | 2 | building | 1.0 |

Never kept: 11: Breakthrough Lab, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Foundation Model Campus, Liquidity Event, Manifesto Leak, Recruit Operatives, Recursive Self-Improvement

## planner:never_second_presence=1 (108 games)

Kept in more than 1% of games and never played: 34

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 63.9 |
| Honeypot Relay | 1 | building | 61.1 |
| Demand Response Grid | 1 | building | 56.5 |
| Capacitor Discharge | 1 | event | 55.6 |
| Grad Student Pool | 1 | building | 54.6 |
| Bonded Warehouse | 1 | building | 54.6 |
| Kill Team Roster | 1 | building | 53.7 |
| Intrusion Lab | 1 | building | 53.7 |
| Agency Field Station | 1 | building | 53.7 |
| Liaison Corps | 1 | building | 52.8 |
| Open Assembly | 1 | building | 51.9 |
| Hardpoint Lockdown | 1 | event | 51.9 |
| Commons Grid | 1 | building | 51.9 |
| Sentry Drone Wing | 1 | building | 50.0 |
| Inference Cluster | 1 | building | 50.0 |
| Preprint Exchange | 1 | program | 49.1 |
| Neural Mirror | 1 | program | 49.1 |
| Agency Directorate | 2 | building | 48.1 |
| Security Ops Floor | 1 | building | 47.2 |
| Simulation Cluster | 1 | building | 45.4 |
| Retained Earnings | 1 | building | 45.4 |
| Telemetry Archive | 1 | building | 44.4 |
| Cryo-Compute Burn | 1 | event | 39.8 |
| Deep State Ledger | 2 | building | 35.2 |
| Orbital Observatory Array | 2 | building | 33.3 |
| Credential Harvest | 1 | event | 32.4 |
| Quantum Decryptor | 1 | building | 23.1 |
| Sovereign Capital Fund | 2 | building | 21.3 |
| Arc Furnace Complex | 2 | building | 21.3 |
| Planetary Assembly | 2 | building | 11.1 |
| Synthetic Data Forge | 1 | building | 6.5 |
| Positron Storage Ring | 2 | building | 6.5 |
| Endowed Research Chair | 1 | program | 6.5 |
| Orbital Embassy | 2 | building | 2.8 |

Never kept: 16: Adaptive Malware, Capital Injection, Citation Engine, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Foundation Model Campus, Liquidity Event, Manifesto Leak, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Summit Caucus, Vendor Financing

## planner:no_counters=1 (24 games)

Kept in more than 1% of games and never played: 49

| card | tier | kind | games kept, % |
|---|---|---|---|
| Garrison Command | 1 | building | 58.3 |
| Open Assembly | 1 | building | 54.2 |
| Inference Cluster | 1 | building | 50.0 |
| Honeypot Relay | 1 | building | 50.0 |
| Aegis Dome Array | 2 | building | 50.0 |
| Market Watchdog | 1 | building | 45.8 |
| Shared Compute Grant | 1 | program | 41.7 |
| Darknet Exchange | 1 | building | 41.7 |
| Aegis Grid Lockdown | 2 | event | 41.7 |
| Sentry Drone Wing | 1 | building | 37.5 |
| Security Ops Floor | 1 | building | 37.5 |
| Safehouse Network | 1 | building | 37.5 |
| Retained Earnings | 1 | building | 37.5 |
| Lights-Out Assembly | 1 | building | 37.5 |
| Civic Watch | 1 | building | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Grid Census | 1 | program | 33.3 |
| Fusion Startup Stake | 1 | program | 33.3 |
| Demand Response Grid | 1 | building | 33.3 |
| Payroll Automation | 1 | program | 29.2 |
| Commons Grid | 1 | building | 29.2 |
| Agency Field Station | 1 | building | 29.2 |
| Neural Mirror | 1 | program | 25.0 |
| Kill Team Roster | 1 | building | 25.0 |
| Intrusion Lab | 1 | building | 25.0 |
| Grad Student Pool | 1 | building | 25.0 |
| Faraday Yard | 1 | building | 25.0 |
| Bonded Warehouse | 1 | building | 25.0 |
| Sentinel Mesh | 1 | building | 20.8 |
| Mutual Defense Pact | 1 | program | 20.8 |
| Interceptor Battery | 1 | building | 20.8 |
| Cryo-Compute Burn | 1 | event | 20.8 |
| Simulation Cluster | 1 | building | 16.7 |
| Quantum Decryptor | 1 | building | 16.7 |
| Liaison Corps | 1 | building | 16.7 |
| Field Sensor Grid | 1 | program | 16.7 |
| Expansion Fund | 1 | building | 16.7 |
| Credential Harvest | 1 | event | 12.5 |
| Hardpoint Lockdown | 1 | event | 12.5 |
| Black Ice Citadel | 2 | building | 12.5 |
| Agency Directorate | 2 | building | 12.5 |
| Telemetry Archive | 1 | building | 8.3 |
| Robotic Foundry | 1 | building | 8.3 |
| Preprint Exchange | 1 | program | 8.3 |
| Security Contractor HQ | 1 | building | 4.2 |
| Logic Bomb | 1 | program | 4.2 |
| Field Grant | 1 | program | 4.2 |
| Deep State Ledger | 2 | building | 4.2 |
| Autonomous Defense Grid | 2 | building | 4.2 |

Never kept: 45: Adaptive Malware, Antitrust Action, Arc Furnace Complex, Bounty Board, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Foundation Model Campus, Great Liberation, Grid-Tied Smelter, Hyperscale Botnet, Kinetic Interdiction, Liquidity Event, Manifesto Leak, Orbital Embassy, Orbital Mass Driver, Orbital Observatory Array, Planetary Assembly, Policy Institute, Positron Storage Ring, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Shadow Broker Scrub, Signal Hijack, Singularity Exploit, Sovereign Capital Fund, Storm Hardening, Summit Caucus, Supply Chain Audit, Synthetic Data Forge, Tidal Barrage, Total Containment, Truth Commission, Vendor Financing, Vertical Monopoly

## planner:no_powers=1 (120 games)

Kept in more than 1% of games and never played: 33

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 65.8 |
| Honeypot Relay | 1 | building | 65.0 |
| Demand Response Grid | 1 | building | 62.5 |
| Commons Grid | 1 | building | 61.7 |
| Capacitor Discharge | 1 | event | 60.0 |
| Grad Student Pool | 1 | building | 59.2 |
| Open Assembly | 1 | building | 56.7 |
| Kill Team Roster | 1 | building | 55.8 |
| Inference Cluster | 1 | building | 55.0 |
| Liaison Corps | 1 | building | 54.2 |
| Orbital Observatory Array | 2 | building | 52.5 |
| Preprint Exchange | 1 | program | 50.8 |
| Hardpoint Lockdown | 1 | event | 50.8 |
| Neural Mirror | 1 | program | 47.5 |
| Intrusion Lab | 1 | building | 47.5 |
| Simulation Cluster | 1 | building | 45.8 |
| Agency Directorate | 2 | building | 45.8 |
| Telemetry Archive | 1 | building | 41.7 |
| Cryo-Compute Burn | 1 | event | 40.0 |
| Sovereign Capital Fund | 2 | building | 35.0 |
| Deep State Ledger | 2 | building | 33.3 |
| Credential Harvest | 1 | event | 32.5 |
| Quantum Decryptor | 1 | building | 24.2 |
| Arc Furnace Complex | 2 | building | 18.3 |
| Planetary Assembly | 2 | building | 9.2 |
| Synthetic Data Forge | 1 | building | 6.7 |
| Policy Institute | 2 | building | 5.8 |
| Positron Storage Ring | 2 | building | 5.0 |
| Endowed Research Chair | 1 | program | 3.3 |
| Sample Repository | 1 | building | 2.5 |
| Citation Engine | 1 | building | 2.5 |
| Orbital Embassy | 2 | building | 1.7 |
| Hyperscale Botnet | 2 | building | 1.7 |

Never kept: 12: Collateralized Grid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Exabyte Archive, Foundation Model Campus, Liquidity Event, Manifesto Leak, Recruit Operatives, Recursive Self-Improvement, Summit Caucus, Vendor Financing

## planner:no_presence=1 (24 games)

Kept in more than 1% of games and never played: 51

| card | tier | kind | games kept, % |
|---|---|---|---|
| Brownout Relief | 1 | program | 62.5 |
| Open Assembly | 1 | building | 50.0 |
| Inference Cluster | 1 | building | 50.0 |
| Garrison Command | 1 | building | 45.8 |
| Darknet Exchange | 1 | building | 45.8 |
| Security Ops Floor | 1 | building | 41.7 |
| Civic Watch | 1 | building | 41.7 |
| Safehouse Network | 1 | building | 37.5 |
| Relay Mast Rollout | 1 | program | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Shared Compute Grant | 1 | program | 33.3 |
| Honeypot Relay | 1 | building | 33.3 |
| Faraday Yard | 1 | building | 33.3 |
| Demand Response Grid | 1 | building | 33.3 |
| Commons Grid | 1 | building | 33.3 |
| Sentry Drone Wing | 1 | building | 29.2 |
| Prefab Plant Kit | 1 | program | 29.2 |
| Lights-Out Assembly | 1 | building | 29.2 |
| Grid Census | 1 | program | 29.2 |
| Aegis Grid Lockdown | 2 | event | 29.2 |
| Retained Earnings | 1 | building | 25.0 |
| Neural Mirror | 1 | program | 25.0 |
| Mutual Defense Pact | 1 | program | 25.0 |
| Intrusion Lab | 1 | building | 25.0 |
| Interceptor Battery | 1 | building | 25.0 |
| Grad Student Pool | 1 | building | 25.0 |
| Cryo-Compute Burn | 1 | event | 25.0 |
| Bonded Warehouse | 1 | building | 25.0 |
| Agency Field Station | 1 | building | 25.0 |
| Sentinel Mesh | 1 | building | 20.8 |
| Hardpoint Lockdown | 1 | event | 20.8 |
| Stress Test Suite | 1 | program | 20.8 |
| Liaison Corps | 1 | building | 16.7 |
| Kill Team Roster | 1 | building | 16.7 |
| Expansion Fund | 1 | building | 16.7 |
| Transmission Corridor | 1 | program | 12.5 |
| Simulation Cluster | 1 | building | 12.5 |
| Preprint Exchange | 1 | program | 12.5 |
| Aegis Dome Array | 2 | building | 12.5 |
| Supply Chain Treaty | 1 | program | 8.3 |
| Quantum Decryptor | 1 | building | 8.3 |
| Deep State Ledger | 2 | building | 8.3 |
| Agency Directorate | 2 | building | 8.3 |
| Total Containment | 2 | event | 4.2 |
| Telemetry Archive | 1 | building | 4.2 |
| Storm Hardening | 1 | event | 4.2 |
| Reclamation Crew | 1 | program | 4.2 |
| Credential Harvest | 1 | event | 4.2 |
| Sovereign Capital Fund | 2 | building | 4.2 |
| Orbital Observatory Array | 2 | building | 4.2 |
| Black Ice Citadel | 2 | building | 4.2 |

Never kept: 54: Adaptive Malware, Antitrust Action, Arc Furnace Complex, Autonomous Defense Grid, Back-Channel Talks, Beamed Power Constellation, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Great Liberation, Grid-Tied Smelter, Hyperscale Botnet, Interconnector Treaty, Kinetic Interdiction, Liquidity Event, Logic Bomb, Manifesto Leak, Market Stabilization Board, Node Severance, Open-Weights Release, Orbital Embassy, Orbital Mass Driver, Planetary Assembly, Policy Institute, Positron Storage Ring, Predictive Modeling, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Security Contractor HQ, Shadow Broker Scrub, Shared Grid Accord, Shared Threat Intel, Signal Hijack, Singularity Exploit, Summit Caucus, Synthetic Data Forge, Threat Hunt, Tidal Barrage, Vendor Financing

## planner:only_tag=1 (24 games)

Kept in more than 1% of games and never played: 12

| card | tier | kind | games kept, % |
|---|---|---|---|
| Inference Cluster | 1 | building | 62.5 |
| Simulation Cluster | 1 | building | 54.2 |
| Shared Compute Grant | 1 | program | 50.0 |
| Neural Mirror | 1 | program | 45.8 |
| Cryo-Compute Burn | 1 | event | 33.3 |
| Telemetry Archive | 1 | building | 29.2 |
| Preprint Exchange | 1 | program | 29.2 |
| Quantum Decryptor | 1 | building | 12.5 |
| Orbital Observatory Array | 2 | building | 8.3 |
| Open-Weights Release | 2 | program | 8.3 |
| Field Sensor Grid | 1 | program | 4.2 |
| Endowed Research Chair | 1 | program | 4.2 |

Never kept: 130: Adaptive Malware, Aegis Dome Array, Aegis Grid Lockdown, Agency Directorate, Agency Field Station, Amnesty Decree, Antitrust Action, Arc Furnace Complex, Arcology Foundry, Autonomous Defense Grid, Back-Channel Talks, Baseload Contract, Beamed Power Constellation, Black Ice Citadel, Blackout Script, Bonded Warehouse, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Bridge Loan, Brownout Relief, Bulk Silicon Procurement, Capacitor Discharge, Capital Injection, Ceasefire Monitors, Citation Engine, Civic Trust Bureau, Civic Watch, Coalition Charter, Collateralized Grid, Commons Grid, Consular Pouch, Consulate Tower, Continental Supergrid, Credential Harvest, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Demand Response Grid, Diplomatic Expulsion, Dyson Swarm Tranche, Embassy Compound, Eviction Worm, Exabyte Archive, Exodus Corridors, Expansion Fund, Fact-Check Network, Faraday Yard, Field Grant, Force Protection Order, Foundation Model Campus, Fusion Startup Stake, Garrison Command, Great Liberation, Grid Census, Grid-Tied Smelter, Hardened Bunker Complex, Hardened Conduits, Hardpoint Lockdown, Honeypot Relay, Hyperscale Botnet, Interceptor Battery, Interconnector Treaty, Intrusion Countermeasures, Intrusion Lab, Joint Fabrication Pact, Joint Security Compact, Kill Team Roster, Killswitch Directive, Kinetic Interdiction, Liaison Corps, Liberation Accords, Lights-Out Assembly, Liquidity Event, Logic Bomb, Manifesto Leak, Market Stabilization Board, Market Watchdog, Martial Cordon, Mutual Aid Pact, Mutual Defense Pact, Node Severance, Open Assembly, Orbital Embassy, Orbital Mass Driver, Payroll Automation, Peace Dividend, Peacekeeper Rotation, Peaker Turbines, Planetary Assembly, Planetary Referendum, Policy Institute, Positron Storage Ring, Prefab Plant Kit, Reclamation Crew, Recruit Operatives, Recursive Self-Improvement, Relay Mast Rollout, Retained Earnings, Robotic Foundry, Safehouse Network, Sample Repository, Security Contractor HQ, Security Ops Floor, Sentinel Command, Sentinel Mesh, Sentry Drone Wing, Shared Grid Accord, Shared Threat Intel, Signal Hijack, Singularity Exploit, Sovereign Capital Fund, Storm Hardening, Substation Hub, Summit Caucus, Supply Chain Audit, Supply Chain Treaty, Surplus Auction, Synthetic Data Forge, Threat Hunt, Tidal Barrage, Total Containment, Transmission Corridor, Treaty Grid Exchange, Truth Commission, Vendor Financing, Vertical Monopoly, Zero-Click Breach

## planner:only_tag=2 (24 games)

Kept in more than 1% of games and never played: 7

| card | tier | kind | games kept, % |
|---|---|---|---|
| Retained Earnings | 1 | building | 50.0 |
| Expansion Fund | 1 | building | 41.7 |
| Sovereign Capital Fund | 2 | building | 33.3 |
| Arc Furnace Complex | 2 | building | 25.0 |
| Orbital Mass Driver | 2 | building | 20.8 |
| Arcology Foundry | 2 | building | 8.3 |
| Antitrust Action | 1 | program | 4.2 |

Never kept: 128: Adaptive Malware, Adversarial Inference Sweep, Aegis Dome Array, Aegis Grid Lockdown, Agency Directorate, Agency Field Station, Amnesty Decree, Autonomous Defense Grid, Back-Channel Talks, Baseload Contract, Beamed Power Constellation, Black Ice Citadel, Blackout Script, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Brownout Relief, Capacitor Discharge, Capital Injection, Ceasefire Monitors, Citation Engine, Civic Trust Bureau, Civic Watch, Coalition Charter, Collateralized Grid, Commons Grid, Consular Pouch, Consulate Tower, Continental Supergrid, Credential Harvest, Cryo-Compute Burn, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Demand Response Grid, Diplomatic Expulsion, Dyson Swarm Tranche, Early Warning Drill, Embassy Compound, Endowed Research Chair, Eviction Worm, Exabyte Archive, Exodus Corridors, Fact-Check Network, Faraday Yard, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Fusion Startup Stake, Garrison Command, Grad Student Pool, Great Liberation, Grid Census, Grid-Tied Smelter, Hardened Bunker Complex, Hardened Conduits, Hardpoint Lockdown, Honeypot Relay, Hyperscale Botnet, Inference Cluster, Interceptor Battery, Interconnector Treaty, Intrusion Countermeasures, Intrusion Lab, Joint Security Compact, Kill Team Roster, Killswitch Directive, Kinetic Interdiction, Liaison Corps, Liberation Accords, Liquidity Event, Literature Crawler, Logic Bomb, Manifesto Leak, Martial Cordon, Mutual Aid Pact, Mutual Defense Pact, Neural Mirror, Node Severance, Open Assembly, Open-Weights Release, Orbital Embassy, Orbital Observatory Array, Peace Dividend, Peacekeeper Rotation, Planetary Assembly, Planetary Referendum, Policy Institute, Positron Storage Ring, Predictive Modeling, Preprint Exchange, Quantum Decryptor, Recruit Operatives, Recursive Self-Improvement, Red-Team Audit, Relay Mast Rollout, Safehouse Network, Sample Repository, Security Contractor HQ, Security Ops Floor, Sentinel Command, Sentinel Mesh, Sentry Drone Wing, Shadow Broker Scrub, Shared Compute Grant, Shared Exploit Kit, Shared Grid Accord, Shared Threat Intel, Signal Hijack, Simulation Cluster, Singularity Exploit, Storm Hardening, Stress Test Suite, Summit Caucus, Surplus Auction, Synthetic Data Forge, Telemetry Archive, Threat Hunt, Tidal Barrage, Total Containment, Transmission Corridor, Truth Commission, Vendor Financing, Zero-Click Breach, Zero-Knowledge Exploit

## planner:only_tag=3 (24 games)

Kept in more than 1% of games and never played: 14

| card | tier | kind | games kept, % |
|---|---|---|---|
| Honeypot Relay | 1 | building | 58.3 |
| Simulation Cluster | 1 | building | 54.2 |
| Security Ops Floor | 1 | building | 54.2 |
| Inference Cluster | 1 | building | 54.2 |
| Darknet Exchange | 1 | building | 50.0 |
| Orbital Observatory Array | 2 | building | 45.8 |
| Neural Mirror | 1 | program | 45.8 |
| Intrusion Lab | 1 | building | 33.3 |
| Credential Harvest | 1 | event | 25.0 |
| Shared Threat Intel | 1 | program | 25.0 |
| Quantum Decryptor | 1 | building | 12.5 |
| Deep State Ledger | 2 | building | 12.5 |
| Shadow Broker Scrub | 1 | program | 4.2 |
| Adaptive Malware | 1 | program | 4.2 |

Never kept: 118: Aegis Dome Array, Aegis Grid Lockdown, Agency Directorate, Agency Field Station, Amnesty Decree, Antitrust Action, Arc Furnace Complex, Arcology Foundry, Back-Channel Talks, Baseload Contract, Beamed Power Constellation, Bonded Warehouse, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Bridge Loan, Brownout Relief, Bulk Silicon Procurement, Capacitor Discharge, Capital Injection, Citation Engine, Civic Trust Bureau, Civic Watch, Coalition Charter, Collateralized Grid, Commons Grid, Consular Pouch, Consulate Tower, Continental Supergrid, Cryo-Compute Burn, Data Licensing Deal, Dataset Scraper, Demand Response Grid, Diplomatic Expulsion, Dyson Swarm Tranche, Embassy Compound, Endowed Research Chair, Exabyte Archive, Exodus Corridors, Expansion Fund, Fact-Check Network, Faraday Yard, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Fusion Startup Stake, Garrison Command, Grad Student Pool, Great Liberation, Grid Census, Grid-Tied Smelter, Hardened Bunker Complex, Hardened Conduits, Hardpoint Lockdown, Hyperscale Botnet, Interceptor Battery, Interconnector Treaty, Joint Fabrication Pact, Joint Security Compact, Kill Team Roster, Kinetic Interdiction, Liaison Corps, Liberation Accords, Lights-Out Assembly, Liquidity Event, Literature Crawler, Manifesto Leak, Market Stabilization Board, Market Watchdog, Martial Cordon, Mutual Aid Pact, Mutual Defense Pact, Open Assembly, Orbital Embassy, Orbital Mass Driver, Payroll Automation, Peace Dividend, Peacekeeper Rotation, Peaker Turbines, Planetary Assembly, Planetary Referendum, Policy Institute, Positron Storage Ring, Predictive Modeling, Prefab Plant Kit, Preprint Exchange, Reclamation Crew, Recruit Operatives, Recursive Self-Improvement, Relay Mast Rollout, Retained Earnings, Robotic Foundry, Safehouse Network, Sample Repository, Security Contractor HQ, Sentinel Command, Sentry Drone Wing, Shared Compute Grant, Shared Grid Accord, Sovereign Capital Fund, Storm Hardening, Stress Test Suite, Substation Hub, Summit Caucus, Supply Chain Audit, Supply Chain Treaty, Surplus Auction, Synthetic Data Forge, Telemetry Archive, Tidal Barrage, Total Containment, Transmission Corridor, Treaty Grid Exchange, Truth Commission, Vendor Financing, Vertical Monopoly

## planner:only_tag=4 (24 games)

Kept in more than 1% of games and never played: 10

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Grid Lockdown | 2 | event | 66.7 |
| Lights-Out Assembly | 1 | building | 54.2 |
| Capacitor Discharge | 1 | event | 45.8 |
| Demand Response Grid | 1 | building | 41.7 |
| Commons Grid | 1 | building | 41.7 |
| Aegis Dome Array | 2 | building | 37.5 |
| Arc Furnace Complex | 2 | building | 25.0 |
| Storm Hardening | 1 | event | 16.7 |
| Positron Storage Ring | 2 | building | 16.7 |
| Dyson Swarm Tranche | 2 | building | 4.2 |

Never kept: 118: Adaptive Malware, Adversarial Inference Sweep, Agency Directorate, Agency Field Station, Amnesty Decree, Antitrust Action, Arcology Foundry, Autonomous Defense Grid, Back-Channel Talks, Black Ice Citadel, Blackout Script, Bonded Warehouse, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Bridge Loan, Bulk Silicon Procurement, Capital Injection, Ceasefire Monitors, Citation Engine, Civic Trust Bureau, Civic Watch, Coalition Charter, Collateralized Grid, Consular Pouch, Consulate Tower, Continental Supergrid, Credential Harvest, Cryo-Compute Burn, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Diplomatic Expulsion, Early Warning Drill, Embassy Compound, Endowed Research Chair, Eviction Worm, Exabyte Archive, Exodus Corridors, Expansion Fund, Fact-Check Network, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Garrison Command, Grad Student Pool, Great Liberation, Hardened Bunker Complex, Hardpoint Lockdown, Honeypot Relay, Hyperscale Botnet, Inference Cluster, Interceptor Battery, Intrusion Countermeasures, Intrusion Lab, Joint Security Compact, Kill Team Roster, Killswitch Directive, Kinetic Interdiction, Liaison Corps, Liberation Accords, Liquidity Event, Literature Crawler, Logic Bomb, Manifesto Leak, Market Stabilization Board, Market Watchdog, Martial Cordon, Mutual Aid Pact, Mutual Defense Pact, Neural Mirror, Node Severance, Open Assembly, Open-Weights Release, Orbital Embassy, Orbital Observatory Array, Payroll Automation, Peace Dividend, Peacekeeper Rotation, Planetary Assembly, Policy Institute, Predictive Modeling, Preprint Exchange, Quantum Decryptor, Recruit Operatives, Recursive Self-Improvement, Red-Team Audit, Retained Earnings, Safehouse Network, Sample Repository, Security Contractor HQ, Security Ops Floor, Sentinel Command, Sentinel Mesh, Sentry Drone Wing, Shadow Broker Scrub, Shared Compute Grant, Shared Exploit Kit, Shared Threat Intel, Signal Hijack, Simulation Cluster, Singularity Exploit, Sovereign Capital Fund, Stress Test Suite, Summit Caucus, Supply Chain Audit, Synthetic Data Forge, Telemetry Archive, Threat Hunt, Total Containment, Truth Commission, Vendor Financing, Vertical Monopoly, Zero-Click Breach, Zero-Knowledge Exploit

## planner:only_tag=5 (112 games)

Kept in more than 1% of games and never played: 6

| card | tier | kind | games kept, % |
|---|---|---|---|
| Commons Grid | 1 | building | 45.5 |
| Open Assembly | 1 | building | 44.6 |
| Mutual Defense Pact | 1 | program | 40.2 |
| Liaison Corps | 1 | building | 38.4 |
| Planetary Assembly | 2 | building | 7.1 |
| Orbital Embassy | 2 | building | 1.8 |

Never kept: 113: Adaptive Malware, Adversarial Inference Sweep, Agency Directorate, Agency Field Station, Antitrust Action, Arc Furnace Complex, Arcology Foundry, Autonomous Defense Grid, Baseload Contract, Beamed Power Constellation, Black Ice Citadel, Blackout Script, Bonded Warehouse, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Bridge Loan, Bulk Silicon Procurement, Capacitor Discharge, Capital Injection, Citation Engine, Collateralized Grid, Continental Supergrid, Credential Harvest, Cryo-Compute Burn, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Demand Response Grid, Dyson Swarm Tranche, Early Warning Drill, Endowed Research Chair, Eviction Worm, Exabyte Archive, Expansion Fund, Faraday Yard, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Fusion Startup Stake, Garrison Command, Grad Student Pool, Grid Census, Grid-Tied Smelter, Hardened Bunker Complex, Hardened Conduits, Hardpoint Lockdown, Honeypot Relay, Hyperscale Botnet, Inference Cluster, Interceptor Battery, Intrusion Countermeasures, Intrusion Lab, Kill Team Roster, Killswitch Directive, Kinetic Interdiction, Lights-Out Assembly, Liquidity Event, Literature Crawler, Logic Bomb, Manifesto Leak, Market Stabilization Board, Market Watchdog, Neural Mirror, Node Severance, Open-Weights Release, Orbital Mass Driver, Orbital Observatory Array, Payroll Automation, Peaker Turbines, Policy Institute, Positron Storage Ring, Predictive Modeling, Prefab Plant Kit, Preprint Exchange, Quantum Decryptor, Reclamation Crew, Recruit Operatives, Recursive Self-Improvement, Red-Team Audit, Relay Mast Rollout, Retained Earnings, Robotic Foundry, Sample Repository, Security Contractor HQ, Security Ops Floor, Sentinel Command, Sentinel Mesh, Sentry Drone Wing, Shadow Broker Scrub, Shared Compute Grant, Shared Exploit Kit, Shared Threat Intel, Signal Hijack, Simulation Cluster, Singularity Exploit, Sovereign Capital Fund, Stress Test Suite, Substation Hub, Supply Chain Audit, Surplus Auction, Synthetic Data Forge, Telemetry Archive, Threat Hunt, Tidal Barrage, Total Containment, Vendor Financing, Vertical Monopoly, Zero-Click Breach, Zero-Knowledge Exploit

## planner:only_tag=6 (24 games)

Kept in more than 1% of games and never played: 11

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 79.2 |
| Open Assembly | 1 | building | 66.7 |
| Sentry Drone Wing | 1 | building | 62.5 |
| Safehouse Network | 1 | building | 54.2 |
| Agency Directorate | 2 | building | 50.0 |
| Kill Team Roster | 1 | building | 45.8 |
| Hardpoint Lockdown | 1 | event | 41.7 |
| Agency Field Station | 1 | building | 41.7 |
| Deep State Ledger | 2 | building | 33.3 |
| Storm Hardening | 1 | event | 29.2 |
| Shared Threat Intel | 1 | program | 12.5 |

Never kept: 113: Adaptive Malware, Amnesty Decree, Antitrust Action, Arc Furnace Complex, Arcology Foundry, Autonomous Defense Grid, Back-Channel Talks, Baseload Contract, Beamed Power Constellation, Blackout Script, Bonded Warehouse, Breakthrough Lab, Breakthrough Leak, Bridge Loan, Brownout Relief, Bulk Silicon Procurement, Capacitor Discharge, Capital Injection, Citation Engine, Civic Trust Bureau, Civic Watch, Coalition Charter, Collateralized Grid, Commons Grid, Consular Pouch, Consulate Tower, Continental Supergrid, Credential Harvest, Cryo-Compute Burn, Darknet Exchange, Data Licensing Deal, Dataset Scraper, Demand Response Grid, Dyson Swarm Tranche, Endowed Research Chair, Eviction Worm, Exabyte Archive, Expansion Fund, Fact-Check Network, Faraday Yard, Field Grant, Field Sensor Grid, Foundation Model Campus, Fusion Startup Stake, Grad Student Pool, Great Liberation, Grid Census, Grid-Tied Smelter, Hardened Conduits, Honeypot Relay, Hyperscale Botnet, Inference Cluster, Interconnector Treaty, Intrusion Lab, Joint Fabrication Pact, Killswitch Directive, Liaison Corps, Liberation Accords, Lights-Out Assembly, Liquidity Event, Literature Crawler, Logic Bomb, Manifesto Leak, Market Stabilization Board, Market Watchdog, Mutual Aid Pact, Neural Mirror, Open-Weights Release, Orbital Embassy, Orbital Mass Driver, Orbital Observatory Array, Payroll Automation, Peace Dividend, Peaker Turbines, Planetary Assembly, Planetary Referendum, Policy Institute, Positron Storage Ring, Predictive Modeling, Prefab Plant Kit, Preprint Exchange, Quantum Decryptor, Reclamation Crew, Recruit Operatives, Recursive Self-Improvement, Relay Mast Rollout, Retained Earnings, Robotic Foundry, Sample Repository, Security Ops Floor, Shadow Broker Scrub, Shared Compute Grant, Shared Grid Accord, Simulation Cluster, Singularity Exploit, Sovereign Capital Fund, Stress Test Suite, Substation Hub, Summit Caucus, Supply Chain Audit, Supply Chain Treaty, Surplus Auction, Synthetic Data Forge, Telemetry Archive, Threat Hunt, Tidal Barrage, Transmission Corridor, Treaty Grid Exchange, Truth Commission, Vendor Financing, Vertical Monopoly, Zero-Click Breach, Zero-Knowledge Exploit

## planner:science_board=1 (24 games)

Kept in more than 1% of games and never played: 47

| card | tier | kind | games kept, % |
|---|---|---|---|
| Open Assembly | 1 | building | 58.3 |
| Market Watchdog | 1 | building | 54.2 |
| Honeypot Relay | 1 | building | 50.0 |
| Garrison Command | 1 | building | 50.0 |
| Civic Watch | 1 | building | 50.0 |
| Darknet Exchange | 1 | building | 45.8 |
| Agency Field Station | 1 | building | 45.8 |
| Security Ops Floor | 1 | building | 41.7 |
| Bonded Warehouse | 1 | building | 41.7 |
| Safehouse Network | 1 | building | 37.5 |
| Kill Team Roster | 1 | building | 37.5 |
| Commons Grid | 1 | building | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Aegis Dome Array | 2 | building | 37.5 |
| Sentinel Mesh | 1 | building | 33.3 |
| Prefab Plant Kit | 1 | program | 33.3 |
| Lights-Out Assembly | 1 | building | 33.3 |
| Interconnector Treaty | 2 | program | 33.3 |
| Sentry Drone Wing | 1 | building | 29.2 |
| Retained Earnings | 1 | building | 29.2 |
| Liaison Corps | 1 | building | 29.2 |
| Faraday Yard | 1 | building | 29.2 |
| Intrusion Lab | 1 | building | 25.0 |
| Expansion Fund | 1 | building | 25.0 |
| Demand Response Grid | 1 | building | 25.0 |
| Mutual Defense Pact | 1 | program | 20.8 |
| Treaty Grid Exchange | 1 | building | 16.7 |
| Sovereign Capital Fund | 2 | building | 16.7 |
| Robotic Foundry | 1 | building | 16.7 |
| Civic Trust Bureau | 2 | building | 16.7 |
| Supply Chain Treaty | 1 | program | 12.5 |
| Credential Harvest | 1 | event | 12.5 |
| Deep State Ledger | 2 | building | 12.5 |
| Autonomous Defense Grid | 2 | building | 12.5 |
| Arc Furnace Complex | 2 | building | 12.5 |
| Supply Chain Audit | 1 | program | 8.3 |
| Storm Hardening | 1 | event | 8.3 |
| Shared Threat Intel | 1 | program | 8.3 |
| Security Contractor HQ | 1 | building | 8.3 |
| Hardpoint Lockdown | 1 | event | 8.3 |
| Agency Directorate | 2 | building | 8.3 |
| Vertical Monopoly | 2 | building | 4.2 |
| Threat Hunt | 1 | program | 4.2 |
| Hyperscale Botnet | 2 | building | 4.2 |
| Beamed Power Constellation | 2 | program | 4.2 |
| Arcology Foundry | 2 | building | 4.2 |
| Antitrust Action | 1 | program | 4.2 |

Never kept: 21: Adaptive Malware, Bounty Board, Capital Injection, Collateralized Grid, Consulate Tower, Continental Supergrid, Dyson Swarm Tranche, Force Protection Order, Great Liberation, Grid-Tied Smelter, Liquidity Event, Logic Bomb, Manifesto Leak, Orbital Embassy, Orbital Mass Driver, Planetary Assembly, Positron Storage Ring, Recruit Operatives, Summit Caucus, Tidal Barrage, Vendor Financing

## planner:science_rush=1 (24 games)

Kept in more than 1% of games and never played: 44

| card | tier | kind | games kept, % |
|---|---|---|---|
| Foundation Model Campus | 2 | building | 79.2 |
| Open Assembly | 1 | building | 58.3 |
| Market Watchdog | 1 | building | 54.2 |
| Civic Watch | 1 | building | 54.2 |
| Honeypot Relay | 1 | building | 50.0 |
| Garrison Command | 1 | building | 50.0 |
| Darknet Exchange | 1 | building | 45.8 |
| Agency Field Station | 1 | building | 45.8 |
| Security Ops Floor | 1 | building | 37.5 |
| Safehouse Network | 1 | building | 37.5 |
| Lights-Out Assembly | 1 | building | 37.5 |
| Commons Grid | 1 | building | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Aegis Dome Array | 2 | building | 37.5 |
| Sentinel Mesh | 1 | building | 33.3 |
| Retained Earnings | 1 | building | 33.3 |
| Prefab Plant Kit | 1 | program | 33.3 |
| Kill Team Roster | 1 | building | 33.3 |
| Bonded Warehouse | 1 | building | 33.3 |
| Interconnector Treaty | 2 | program | 29.2 |
| Faraday Yard | 1 | building | 29.2 |
| Expansion Fund | 1 | building | 29.2 |
| Sentry Drone Wing | 1 | building | 25.0 |
| Liaison Corps | 1 | building | 25.0 |
| Intrusion Lab | 1 | building | 25.0 |
| Demand Response Grid | 1 | building | 25.0 |
| Mutual Defense Pact | 1 | program | 20.8 |
| Civic Trust Bureau | 2 | building | 20.8 |
| Treaty Grid Exchange | 1 | building | 16.7 |
| Arc Furnace Complex | 2 | building | 16.7 |
| Supply Chain Treaty | 1 | program | 12.5 |
| Credential Harvest | 1 | event | 12.5 |
| Sovereign Capital Fund | 2 | building | 12.5 |
| Security Contractor HQ | 1 | building | 12.5 |
| Deep State Ledger | 2 | building | 12.5 |
| Autonomous Defense Grid | 2 | building | 12.5 |
| Agency Directorate | 2 | building | 12.5 |
| Storm Hardening | 1 | event | 8.3 |
| Robotic Foundry | 1 | building | 8.3 |
| Hardpoint Lockdown | 1 | event | 8.3 |
| Supply Chain Audit | 1 | program | 4.2 |
| Orbital Mass Driver | 2 | building | 4.2 |
| Beamed Power Constellation | 2 | program | 4.2 |
| Arcology Foundry | 2 | building | 4.2 |

Never kept: 22: Adaptive Malware, Antitrust Action, Bounty Board, Capital Injection, Collateralized Grid, Consulate Tower, Continental Supergrid, Dyson Swarm Tranche, Force Protection Order, Great Liberation, Grid-Tied Smelter, Hyperscale Botnet, Liquidity Event, Logic Bomb, Manifesto Leak, Orbital Embassy, Planetary Assembly, Positron Storage Ring, Recruit Operatives, Summit Caucus, Tidal Barrage, Vendor Financing

## planner:tier1_only=1 (28 games)

Kept in more than 1% of games and never played: 33

| card | tier | kind | games kept, % |
|---|---|---|---|
| Expansion Fund | 1 | building | 89.3 |
| Security Ops Floor | 1 | building | 85.7 |
| Safehouse Network | 1 | building | 85.7 |
| Neural Mirror | 1 | program | 85.7 |
| Garrison Command | 1 | building | 85.7 |
| Darknet Exchange | 1 | building | 85.7 |
| Bonded Warehouse | 1 | building | 85.7 |
| Sentry Drone Wing | 1 | building | 82.1 |
| Inference Cluster | 1 | building | 82.1 |
| Agency Field Station | 1 | building | 82.1 |
| Simulation Cluster | 1 | building | 78.6 |
| Liaison Corps | 1 | building | 75.0 |
| Kill Team Roster | 1 | building | 75.0 |
| Honeypot Relay | 1 | building | 75.0 |
| Grad Student Pool | 1 | building | 75.0 |
| Commons Grid | 1 | building | 75.0 |
| Civic Watch | 1 | building | 75.0 |
| Preprint Exchange | 1 | program | 71.4 |
| Intrusion Lab | 1 | building | 67.9 |
| Faraday Yard | 1 | building | 67.9 |
| Demand Response Grid | 1 | building | 67.9 |
| Cryo-Compute Burn | 1 | event | 67.9 |
| Telemetry Archive | 1 | building | 64.3 |
| Quantum Decryptor | 1 | building | 64.3 |
| Open Assembly | 1 | building | 64.3 |
| Hardpoint Lockdown | 1 | event | 64.3 |
| Retained Earnings | 1 | building | 57.1 |
| Credential Harvest | 1 | event | 53.6 |
| Endowed Research Chair | 1 | program | 32.1 |
| Synthetic Data Forge | 1 | building | 10.7 |
| Adaptive Malware | 1 | program | 10.7 |
| Sample Repository | 1 | building | 3.6 |
| Citation Engine | 1 | building | 3.6 |

Never kept: 50: Adversarial Inference Sweep, Aegis Dome Array, Aegis Grid Lockdown, Agency Directorate, Arc Furnace Complex, Arcology Foundry, Autonomous Defense Grid, Beamed Power Constellation, Black Ice Citadel, Breakthrough Lab, Capital Injection, Civic Trust Bureau, Coalition Charter, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Dyson Swarm Tranche, Exabyte Archive, Exodus Corridors, Foundation Model Campus, Great Liberation, Hyperscale Botnet, Interconnector Treaty, Killswitch Directive, Kinetic Interdiction, Liberation Accords, Liquidity Event, Manifesto Leak, Market Stabilization Board, Martial Cordon, Node Severance, Open-Weights Release, Orbital Embassy, Orbital Mass Driver, Orbital Observatory Array, Planetary Assembly, Planetary Referendum, Policy Institute, Positron Storage Ring, Recruit Operatives, Recursive Self-Improvement, Sentinel Command, Singularity Exploit, Sovereign Capital Fund, Summit Caucus, Total Containment, Vendor Financing, Vertical Monopoly

## planner:tier2_rush=1 (108 games)

Kept in more than 1% of games and never played: 34

| card | tier | kind | games kept, % |
|---|---|---|---|
| Agency Directorate | 2 | building | 62.0 |
| Sovereign Capital Fund | 2 | building | 50.9 |
| Deep State Ledger | 2 | building | 50.0 |
| Orbital Observatory Array | 2 | building | 47.2 |
| Arc Furnace Complex | 2 | building | 31.5 |
| Inference Cluster | 1 | building | 26.9 |
| Civic Watch | 1 | building | 25.0 |
| Expansion Fund | 1 | building | 24.1 |
| Agency Field Station | 1 | building | 23.1 |
| Intrusion Lab | 1 | building | 22.2 |
| Capacitor Discharge | 1 | event | 22.2 |
| Open Assembly | 1 | building | 21.3 |
| Grad Student Pool | 1 | building | 21.3 |
| Honeypot Relay | 1 | building | 20.4 |
| Garrison Command | 1 | building | 20.4 |
| Demand Response Grid | 1 | building | 18.5 |
| Bonded Warehouse | 1 | building | 18.5 |
| Mutual Defense Pact | 1 | program | 17.6 |
| Hardpoint Lockdown | 1 | event | 17.6 |
| Commons Grid | 1 | building | 16.7 |
| Neural Mirror | 1 | program | 15.7 |
| Liaison Corps | 1 | building | 15.7 |
| Preprint Exchange | 1 | program | 13.9 |
| Kill Team Roster | 1 | building | 13.0 |
| Security Ops Floor | 1 | building | 12.0 |
| Safehouse Network | 1 | building | 12.0 |
| Sentry Drone Wing | 1 | building | 11.1 |
| Planetary Assembly | 2 | building | 8.3 |
| Simulation Cluster | 1 | building | 7.4 |
| Cryo-Compute Burn | 1 | event | 7.4 |
| Policy Institute | 2 | building | 6.5 |
| Telemetry Archive | 1 | building | 2.8 |
| Credential Harvest | 1 | event | 1.9 |
| Positron Storage Ring | 2 | building | 1.9 |

Never kept: 40: Adaptive Malware, Back-Channel Talks, Bounty Board, Breach and Clear, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Grid-Tied Smelter, Hardened Conduits, Liquidity Event, Logic Bomb, Manifesto Leak, Orbital Embassy, Quantum Decryptor, Reclamation Crew, Recruit Operatives, Recursive Self-Improvement, Robotic Foundry, Sample Repository, Security Contractor HQ, Shadow Broker Scrub, Signal Hijack, Summit Caucus, Synthetic Data Forge, Threat Hunt, Tidal Barrage, Vendor Financing

## planner:turtle=1 (24 games)

Kept in more than 1% of games and never played: 54

| card | tier | kind | games kept, % |
|---|---|---|---|
| Open Assembly | 1 | building | 54.2 |
| Garrison Command | 1 | building | 54.2 |
| Brownout Relief | 1 | program | 54.2 |
| Inference Cluster | 1 | building | 50.0 |
| Market Watchdog | 1 | building | 41.7 |
| Darknet Exchange | 1 | building | 41.7 |
| Safehouse Network | 1 | building | 37.5 |
| Civic Watch | 1 | building | 37.5 |
| Security Ops Floor | 1 | building | 33.3 |
| Honeypot Relay | 1 | building | 33.3 |
| Demand Response Grid | 1 | building | 33.3 |
| Commons Grid | 1 | building | 33.3 |
| Capacitor Discharge | 1 | event | 33.3 |
| Bonded Warehouse | 1 | building | 33.3 |
| Shared Compute Grant | 1 | program | 29.2 |
| Sentry Drone Wing | 1 | building | 29.2 |
| Relay Mast Rollout | 1 | program | 29.2 |
| Joint Security Compact | 1 | program | 29.2 |
| Grad Student Pool | 1 | building | 29.2 |
| Faraday Yard | 1 | building | 29.2 |
| Payroll Automation | 1 | program | 25.0 |
| Lights-Out Assembly | 1 | building | 25.0 |
| Intrusion Lab | 1 | building | 25.0 |
| Grid Census | 1 | program | 25.0 |
| Fusion Startup Stake | 1 | program | 25.0 |
| Aegis Grid Lockdown | 2 | event | 25.0 |
| Retained Earnings | 1 | building | 20.8 |
| Neural Mirror | 1 | program | 20.8 |
| Kill Team Roster | 1 | building | 20.8 |
| Interceptor Battery | 1 | building | 20.8 |
| Expansion Fund | 1 | building | 20.8 |
| Aegis Dome Array | 2 | building | 20.8 |
| Transmission Corridor | 1 | program | 16.7 |
| Simulation Cluster | 1 | building | 16.7 |
| Sentinel Mesh | 1 | building | 16.7 |
| Cryo-Compute Burn | 1 | event | 16.7 |
| Stress Test Suite | 1 | program | 16.7 |
| Agency Field Station | 1 | building | 16.7 |
| Liaison Corps | 1 | building | 12.5 |
| Storm Hardening | 1 | event | 8.3 |
| Shared Threat Intel | 1 | program | 8.3 |
| Quantum Decryptor | 1 | building | 8.3 |
| Preprint Exchange | 1 | program | 8.3 |
| Orbital Observatory Array | 2 | building | 8.3 |
| Hardpoint Lockdown | 1 | event | 8.3 |
| Agency Directorate | 2 | building | 8.3 |
| Treaty Grid Exchange | 1 | building | 4.2 |
| Telemetry Archive | 1 | building | 4.2 |
| Supply Chain Audit | 1 | program | 4.2 |
| Reclamation Crew | 1 | program | 4.2 |
| Credential Harvest | 1 | event | 4.2 |
| Robotic Foundry | 1 | building | 4.2 |
| Deep State Ledger | 2 | building | 4.2 |
| Arc Furnace Complex | 2 | building | 4.2 |

Never kept: 53: Adaptive Malware, Arcology Foundry, Autonomous Defense Grid, Beamed Power Constellation, Bounty Board, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Exodus Corridors, Field Grant, Field Sensor Grid, Force Protection Order, Foundation Model Campus, Great Liberation, Grid-Tied Smelter, Hardened Conduits, Hyperscale Botnet, Interconnector Treaty, Kinetic Interdiction, Liquidity Event, Logic Bomb, Manifesto Leak, Node Severance, Open-Weights Release, Orbital Embassy, Orbital Mass Driver, Planetary Assembly, Policy Institute, Positron Storage Ring, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Security Contractor HQ, Shadow Broker Scrub, Shared Grid Accord, Signal Hijack, Singularity Exploit, Sovereign Capital Fund, Synthetic Data Forge, Threat Hunt, Tidal Barrage, Total Containment, Vendor Financing, Vertical Monopoly

## planner:unity_farm=1 (24 games)

Kept in more than 1% of games and never played: 70

| card | tier | kind | games kept, % |
|---|---|---|---|
| Open Assembly | 1 | building | 58.3 |
| Inference Cluster | 1 | building | 54.2 |
| Market Watchdog | 1 | building | 50.0 |
| Killswitch Directive | 2 | event | 50.0 |
| Joint Security Compact | 1 | program | 50.0 |
| Substation Hub | 1 | building | 45.8 |
| Darknet Exchange | 1 | building | 45.8 |
| Lights-Out Assembly | 1 | building | 41.7 |
| Garrison Command | 1 | building | 41.7 |
| Shared Compute Grant | 1 | program | 37.5 |
| Sentry Drone Wing | 1 | building | 37.5 |
| Security Ops Floor | 1 | building | 37.5 |
| Safehouse Network | 1 | building | 37.5 |
| Retained Earnings | 1 | building | 37.5 |
| Prefab Plant Kit | 1 | program | 37.5 |
| Honeypot Relay | 1 | building | 37.5 |
| Capacitor Discharge | 1 | event | 37.5 |
| Peaker Turbines | 1 | program | 33.3 |
| Grid Census | 1 | program | 33.3 |
| Fusion Startup Stake | 1 | program | 33.3 |
| Faraday Yard | 1 | building | 33.3 |
| Civic Watch | 1 | building | 33.3 |
| Simulation Cluster | 1 | building | 29.2 |
| Shared Exploit Kit | 1 | program | 29.2 |
| Payroll Automation | 1 | program | 29.2 |
| Neural Mirror | 1 | program | 29.2 |
| Mutual Defense Pact | 1 | program | 29.2 |
| Demand Response Grid | 1 | building | 29.2 |
| Commons Grid | 1 | building | 29.2 |
| Stress Test Suite | 1 | program | 29.2 |
| Aegis Grid Lockdown | 2 | event | 29.2 |
| Literature Crawler | 1 | program | 25.0 |
| Kill Team Roster | 1 | building | 25.0 |
| Interceptor Battery | 1 | building | 25.0 |
| Grad Student Pool | 1 | building | 25.0 |
| Cryo-Compute Burn | 1 | event | 25.0 |
| Bonded Warehouse | 1 | building | 25.0 |
| Reclamation Crew | 1 | program | 20.8 |
| Sentinel Mesh | 1 | building | 20.8 |
| Quantum Decryptor | 1 | building | 20.8 |
| Mutual Aid Pact | 1 | program | 20.8 |
| Intrusion Lab | 1 | building | 20.8 |
| Expansion Fund | 1 | building | 20.8 |
| Agency Field Station | 1 | building | 20.8 |
| Zero-Click Breach | 1 | program | 16.7 |
| Supply Chain Audit | 1 | program | 16.7 |
| Robotic Foundry | 1 | building | 16.7 |
| Liaison Corps | 1 | building | 16.7 |
| Aegis Dome Array | 2 | building | 16.7 |
| Storm Hardening | 1 | event | 12.5 |
| Credential Harvest | 1 | event | 12.5 |
| Shared Threat Intel | 1 | program | 12.5 |
| Preprint Exchange | 1 | program | 12.5 |
| Market Stabilization Board | 2 | building | 12.5 |
| Hardpoint Lockdown | 1 | event | 12.5 |
| Agency Directorate | 2 | building | 12.5 |
| Treaty Grid Exchange | 1 | building | 8.3 |
| Supply Chain Treaty | 1 | program | 8.3 |
| Interconnector Treaty | 2 | program | 8.3 |
| Field Grant | 1 | program | 8.3 |
| Breach and Clear | 1 | event | 8.3 |
| Black Ice Citadel | 2 | building | 8.3 |
| Total Containment | 2 | event | 4.2 |
| Orbital Observatory Array | 2 | building | 4.2 |
| Open-Weights Release | 2 | program | 4.2 |
| Martial Cordon | 2 | program | 4.2 |
| Liberation Accords | 2 | program | 4.2 |
| Force Protection Order | 1 | program | 4.2 |
| Autonomous Defense Grid | 2 | building | 4.2 |
| Arcology Foundry | 2 | building | 4.2 |

Never kept: 52: Adaptive Malware, Adversarial Inference Sweep, Antitrust Action, Arc Furnace Complex, Beamed Power Constellation, Bounty Board, Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Civic Trust Bureau, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Deep State Ledger, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Exodus Corridors, Foundation Model Campus, Great Liberation, Grid-Tied Smelter, Hyperscale Botnet, Kinetic Interdiction, Liquidity Event, Manifesto Leak, Node Severance, Orbital Embassy, Orbital Mass Driver, Planetary Assembly, Policy Institute, Positron Storage Ring, Predictive Modeling, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Security Contractor HQ, Sentinel Command, Shadow Broker Scrub, Signal Hijack, Singularity Exploit, Sovereign Capital Fund, Summit Caucus, Surplus Auction, Synthetic Data Forge, Telemetry Archive, Threat Hunt, Tidal Barrage, Vendor Financing, Vertical Monopoly

## planner:w.credit_cap=24 (96 games)

Kept in more than 1% of games and never played: 40

| card | tier | kind | games kept, % |
|---|---|---|---|
| Aegis Dome Array | 2 | building | 59.4 |
| Demand Response Grid | 1 | building | 58.3 |
| Bonded Warehouse | 1 | building | 55.2 |
| Civic Watch | 1 | building | 53.1 |
| Security Ops Floor | 1 | building | 51.0 |
| Liaison Corps | 1 | building | 51.0 |
| Retained Earnings | 1 | building | 50.0 |
| Capacitor Discharge | 1 | event | 50.0 |
| Sentry Drone Wing | 1 | building | 49.0 |
| Lights-Out Assembly | 1 | building | 49.0 |
| Honeypot Relay | 1 | building | 49.0 |
| Preprint Exchange | 1 | program | 46.9 |
| Intrusion Lab | 1 | building | 46.9 |
| Faraday Yard | 1 | building | 46.9 |
| Neural Mirror | 1 | program | 45.8 |
| Commons Grid | 1 | building | 45.8 |
| Inference Cluster | 1 | building | 44.8 |
| Open Assembly | 1 | building | 43.8 |
| Credential Harvest | 1 | event | 42.7 |
| Simulation Cluster | 1 | building | 42.7 |
| Telemetry Archive | 1 | building | 41.7 |
| Hardpoint Lockdown | 1 | event | 39.6 |
| Cryo-Compute Burn | 1 | event | 39.6 |
| Agency Directorate | 2 | building | 39.6 |
| Kill Team Roster | 1 | building | 34.4 |
| Safehouse Network | 1 | building | 32.3 |
| Orbital Observatory Array | 2 | building | 31.2 |
| Arc Furnace Complex | 2 | building | 31.2 |
| Deep State Ledger | 2 | building | 24.0 |
| Sovereign Capital Fund | 2 | building | 19.8 |
| Quantum Decryptor | 1 | building | 19.8 |
| Policy Institute | 2 | building | 7.3 |
| Planetary Assembly | 2 | building | 3.1 |
| Endowed Research Chair | 1 | program | 3.1 |
| Recursive Self-Improvement | 2 | program | 2.1 |
| Synthetic Data Forge | 1 | building | 1.0 |
| Positron Storage Ring | 2 | building | 1.0 |
| Grid-Tied Smelter | 1 | building | 1.0 |
| Citation Engine | 1 | building | 1.0 |
| Breakthrough Lab | 2 | building | 1.0 |

Never kept: 16: Capital Injection, Collateralized Grid, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Exabyte Archive, Foundation Model Campus, Great Liberation, Hyperscale Botnet, Liquidity Event, Manifesto Leak, Recruit Operatives, Sample Repository, Summit Caucus, Vendor Financing

## planner:wall_turtle=1 (28 games)

Kept in more than 1% of games and never played: 50

| card | tier | kind | games kept, % |
|---|---|---|---|
| Garrison Command | 1 | building | 71.4 |
| Aegis Grid Lockdown | 2 | event | 67.9 |
| Aegis Dome Array | 2 | building | 67.9 |
| Open Assembly | 1 | building | 60.7 |
| Market Watchdog | 1 | building | 60.7 |
| Inference Cluster | 1 | building | 60.7 |
| Bonded Warehouse | 1 | building | 60.7 |
| Safehouse Network | 1 | building | 57.1 |
| Lights-Out Assembly | 1 | building | 57.1 |
| Honeypot Relay | 1 | building | 57.1 |
| Darknet Exchange | 1 | building | 53.6 |
| Agency Field Station | 1 | building | 53.6 |
| Security Ops Floor | 1 | building | 50.0 |
| Capacitor Discharge | 1 | event | 46.4 |
| Telemetry Archive | 1 | building | 42.9 |
| Simulation Cluster | 1 | building | 42.9 |
| Liaison Corps | 1 | building | 42.9 |
| Kill Team Roster | 1 | building | 42.9 |
| Grad Student Pool | 1 | building | 42.9 |
| Demand Response Grid | 1 | building | 42.9 |
| Retained Earnings | 1 | building | 39.3 |
| Neural Mirror | 1 | program | 39.3 |
| Intrusion Lab | 1 | building | 39.3 |
| Grid Census | 1 | program | 39.3 |
| Civic Watch | 1 | building | 39.3 |
| Storm Hardening | 1 | event | 35.7 |
| Sentry Drone Wing | 1 | building | 35.7 |
| Preprint Exchange | 1 | program | 35.7 |
| Prefab Plant Kit | 1 | program | 35.7 |
| Interceptor Battery | 1 | building | 35.7 |
| Expansion Fund | 1 | building | 35.7 |
| Cryo-Compute Burn | 1 | event | 35.7 |
| Commons Grid | 1 | building | 35.7 |
| Civic Trust Bureau | 2 | building | 32.1 |
| Credential Harvest | 1 | event | 25.0 |
| Sovereign Capital Fund | 2 | building | 25.0 |
| Hardpoint Lockdown | 1 | event | 25.0 |
| Deep State Ledger | 2 | building | 25.0 |
| Agency Directorate | 2 | building | 25.0 |
| Quantum Decryptor | 1 | building | 21.4 |
| Orbital Observatory Array | 2 | building | 21.4 |
| Arc Furnace Complex | 2 | building | 21.4 |
| Orbital Mass Driver | 2 | building | 10.7 |
| Vertical Monopoly | 2 | building | 7.1 |
| Planetary Assembly | 2 | building | 7.1 |
| Grid-Tied Smelter | 1 | building | 7.1 |
| Shadow Broker Scrub | 1 | program | 3.6 |
| Positron Storage Ring | 2 | building | 3.6 |
| Policy Institute | 2 | building | 3.6 |
| Great Liberation | 2 | event | 3.6 |

Never kept: 25: Breakthrough Lab, Breakthrough Leak, Capital Injection, Citation Engine, Collateralized Grid, Consulate Tower, Continental Supergrid, Data Licensing Deal, Dataset Scraper, Dyson Swarm Tranche, Endowed Research Chair, Exabyte Archive, Foundation Model Campus, Hyperscale Botnet, Liquidity Event, Manifesto Leak, Orbital Embassy, Recruit Operatives, Recursive Self-Improvement, Sample Repository, Singularity Exploit, Summit Caucus, Synthetic Data Forge, Tidal Barrage, Vendor Financing

