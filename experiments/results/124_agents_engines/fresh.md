## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| before/strategist | 144 | 74.3 | 6.23 | - | - | 81 / 67 / 75 |
| before/greedy | 144 | 45.8 | 4.83 | -28.5 ± 5.5 | -1.40 | 40 / 40 / 58 |
| after/strategist | 144 | 76.4 | 6.24 | +2.1 ± 3.5 | +0.01 | 77 / 67 / 85 |
| after/greedy | 144 | 52.1 | 4.87 | -22.2 ± 5.6 | -1.36 | 42 / 46 / 69 |

## Lost at the limits, a game

| cell | hold_lost.credits | hold_lost.energy | hold_lost.chips | hold_lost.ambassadors | production_lost.credits | energy: games cut |
|---|---|---|---|---|---|---|
| before/strategist | 3.28 | 5.38 | 1.92 | 0.58 | 0.67 | 65% |
| before/greedy | 0.00 | 0.37 | 0.12 | 0.28 | 0.00 | 10% |
| after/strategist | 2.47 | 4.29 | 1.73 | 0.42 | 0.71 | 62% |
| after/greedy | 0.00 | 0.28 | 0.11 | 0.26 | 0.00 | 8% |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | ambassadors |
|---|---|---|---|---|---|---|
| before/strategist | Builder | 68.4 / 150 | 10.2 / 25 | 9.0 / 10 | 5.1 / 10 | 4.2 / 10 |
| before/strategist | Protector | 61.8 / 150 | 12.8 / 39 | 6.9 / 10 | 3.5 / 10 | 5.2 / 10 |
| before/strategist | Researcher | 62.5 / 146 | 28.2 / 72 | 7.0 / 10 | 3.3 / 10 | 4.7 / 10 |
| before/greedy | Builder | 42.3 / 84 | 8.3 / 22 | 3.4 / 10 | 2.7 / 10 | 4.5 / 10 |
| before/greedy | Protector | 48.2 / 72 | 10.0 / 29 | 5.3 / 10 | 2.4 / 10 | 4.4 / 10 |
| before/greedy | Researcher | 42.6 / 69 | 16.6 / 34 | 6.3 / 10 | 1.9 / 10 | 4.9 / 10 |
| after/strategist | Builder | 65.1 / 150 | 10.9 / 30 | 8.7 / 10 | 4.8 / 10 | 3.9 / 10 |
| after/strategist | Protector | 57.1 / 142 | 13.0 / 28 | 6.8 / 10 | 3.3 / 10 | 4.8 / 10 |
| after/strategist | Researcher | 61.9 / 150 | 22.6 / 89 | 7.1 / 10 | 3.6 / 10 | 5.2 / 10 |
| after/greedy | Builder | 42.5 / 78 | 8.5 / 23 | 3.5 / 10 | 2.8 / 10 | 4.6 / 10 |
| after/greedy | Protector | 47.8 / 69 | 9.8 / 19 | 5.3 / 10 | 2.4 / 10 | 4.5 / 10 |
| after/greedy | Researcher | 42.9 / 66 | 14.6 / 29 | 6.2 / 10 | 1.7 / 10 | 4.7 / 10 |

## The Researcher: keeps against insight

Flag rules: `peak` is 122's (insight peak over 3x a round's insight income and over 10); `rise` is the fixed one (the peak's rise over the starting holding, same test). Keep rate: kept / offered after the opening; tier-2 share of its tier choices; insight at keep: mean held at its keep decisions.

| cell | rule | seats | n | insight peak | offered | kept | keep rate | tier-2 share | insight at keep |
|---|---|---|---|---|---|---|---|---|---|
| before/strategist | peak | not flagged | 38 | 19.3 | 18.1 | 11.6 | 64% | 59% | nan |
| before/strategist | peak | flagged | 58 | 34.1 | 20.3 | 10.4 | 51% | 56% | nan |
| before/strategist | peak | all | 96 | 28.2 | 19.5 | 10.9 | 56% | 58% | nan |
| before/greedy | peak | not flagged | 42 | 12.7 | 13.4 | 4.9 | 37% | 75% | nan |
| before/greedy | peak | flagged | 54 | 19.7 | 15.0 | 3.8 | 25% | 77% | nan |
| before/greedy | peak | all | 96 | 16.6 | 14.3 | 4.3 | 30% | 76% | nan |
| after/strategist | peak | not flagged | 58 | 18.6 | 19.3 | 13.6 | 70% | 62% | 13.6 |
| after/strategist | peak | flagged | 38 | 28.7 | 21.5 | 15.7 | 73% | 60% | 18.0 |
| after/strategist | peak | all | 96 | 22.6 | 20.2 | 14.4 | 72% | 61% | 15.3 |
| after/strategist | rise | not flagged | 87 | 20.6 | 19.8 | 14.0 | 71% | 62% | 14.6 |
| after/strategist | rise | flagged | 9 | 42.6 | 23.6 | 18.3 | 78% | 59% | 22.1 |
| after/greedy | peak | not flagged | 55 | 12.9 | 14.9 | 6.4 | 43% | 75% | 10.1 |
| after/greedy | peak | flagged | 41 | 17.0 | 14.7 | 6.9 | 47% | 80% | 12.9 |
| after/greedy | peak | all | 96 | 14.6 | 14.8 | 6.6 | 44% | 77% | 11.3 |
| after/greedy | rise | not flagged | 94 | 14.4 | 14.8 | 6.5 | 44% | 78% | 11.2 |
| after/greedy | rise | flagged | 2 | 25.0 | 18.0 | 10.5 | 58% | 69% | 17.1 |

## Plays a game (kept, offered): the r41 engine cards and the two named

| card | before/strategist | before/greedy | after/strategist | after/greedy |
|---|---|---|---|---|
| telemetry_archive | 0.049 (0.09, 0.15) | 0.056 (0.05, 0.05) | 0.049 (0.12, 0.17) | 0.042 (0.04, 0.05) |
| citation_engine | 0.111 (0.11, 0.19) | 0.118 (0.04, 0.10) | 0.083 (0.09, 0.14) | 0.132 (0.03, 0.07) |
| grad_student_pool | 0.347 (0.26, 0.15) | 0.368 (0.23, 0.08) | 0.319 (0.30, 0.15) | 0.382 (0.24, 0.07) |
| foundation_model_campus | 0.188 (0.38, 0.58) | 0.083 (0.12, 0.52) | 0.194 (0.40, 0.61) | 0.090 (0.13, 0.52) |
| data_licensing_deal | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) |
| retained_earnings | 0.278 (0.15, 0.15) | 0.021 (0.06, 0.04) | 0.278 (0.16, 0.14) | 0.035 (0.06, 0.03) |
| lights_out_assembly | 0.528 (0.36, 0.17) | 0.514 (0.26, 0.08) | 0.486 (0.39, 0.19) | 0.507 (0.26, 0.07) |
| honeypot_relay | 0.167 (0.23, 0.13) | 0.389 (0.17, 0.06) | 0.160 (0.26, 0.14) | 0.375 (0.19, 0.07) |
| neural_mirror | 0.208 (0.08, 0.11) | 0.000 (0.05, 0.06) | 0.229 (0.10, 0.12) | 0.014 (0.05, 0.05) |
| intrusion_lab | 0.028 (0.10, 0.22) | 0.000 (0.01, 0.06) | 0.014 (0.13, 0.23) | 0.000 (0.01, 0.04) |
| darknet_exchange | 0.208 (0.12, 0.18) | 0.250 (0.10, 0.11) | 0.188 (0.11, 0.13) | 0.257 (0.10, 0.10) |
| tidal_barrage | 0.021 (0.01, 0.17) | 0.000 (0.00, 0.08) | 0.021 (0.02, 0.11) | 0.000 (0.00, 0.08) |
| substation_hub | 0.194 (0.10, 0.19) | 0.035 (0.01, 0.08) | 0.174 (0.10, 0.17) | 0.042 (0.01, 0.09) |
| positron_storage_ring | 0.104 (0.30, 0.51) | 0.000 (0.05, 0.48) | 0.083 (0.29, 0.53) | 0.000 (0.03, 0.47) |
| continental_supergrid | 0.062 (0.23, 0.59) | 0.000 (0.00, 0.53) | 0.062 (0.22, 0.62) | 0.000 (0.00, 0.54) |
| open_assembly | 0.431 (0.31, 0.14) | 0.528 (0.22, 0.05) | 0.438 (0.30, 0.15) | 0.500 (0.24, 0.06) |
| peace_dividend | 0.257 (0.10, 0.13) | 0.014 (0.01, 0.11) | 0.215 (0.10, 0.13) | 0.021 (0.01, 0.09) |
| consulate_tower | 0.229 (0.11, 0.19) | 0.229 (0.04, 0.09) | 0.167 (0.12, 0.17) | 0.257 (0.03, 0.09) |
| orbital_embassy | 0.229 (0.44, 0.74) | 0.132 (0.21, 0.65) | 0.167 (0.43, 0.72) | 0.104 (0.20, 0.64) |
| agency_field_station | 0.111 (0.14, 0.26) | 0.438 (0.05, 0.15) | 0.160 (0.14, 0.27) | 0.431 (0.06, 0.15) |
| security_contractor_hq | 0.153 (0.12, 0.13) | 0.271 (0.01, 0.06) | 0.097 (0.10, 0.13) | 0.292 (0.02, 0.05) |
| agency_directorate | 0.090 (0.23, 0.52) | 0.028 (0.10, 0.41) | 0.056 (0.38, 0.55) | 0.028 (0.14, 0.43) |
| security_ops_floor | 0.104 (0.21, 0.14) | 0.264 (0.14, 0.07) | 0.090 (0.20, 0.11) | 0.257 (0.14, 0.06) |
| vertical_monopoly | 0.229 (0.36, 0.47) | 0.049 (0.08, 0.41) | 0.215 (0.39, 0.51) | 0.062 (0.14, 0.44) |
| power_to_the_people | 0.146 (0.26, 0.60) | 0.007 (0.03, 0.53) | 0.153 (0.38, 0.63) | 0.014 (0.13, 0.53) |
| research_royalties | 0.188 (0.08, 0.19) | 0.007 (0.01, 0.06) | 0.188 (0.08, 0.18) | 0.007 (0.01, 0.06) |
| interdisciplinary_institute | 0.278 (0.43, 0.55) | 0.188 (0.28, 0.51) | 0.264 (0.48, 0.57) | 0.181 (0.26, 0.54) |
| standardized_tooling | 0.410 (0.30, 0.12) | 0.438 (0.22, 0.04) | 0.389 (0.28, 0.10) | 0.451 (0.23, 0.03) |
| economies_of_scale | 0.236 (0.08, 0.17) | 0.000 (0.02, 0.08) | 0.194 (0.08, 0.16) | 0.000 (0.01, 0.06) |
| capital_projects_office | 0.465 (0.64, 0.60) | 0.521 (0.58, 0.51) | 0.521 (0.66, 0.60) | 0.569 (0.63, 0.56) |
| conglomerate_headquarters | 0.292 (0.42, 0.56) | 0.062 (0.34, 0.48) | 0.312 (0.45, 0.56) | 0.076 (0.35, 0.49) |
| ransomware_syndicate | 0.194 (0.04, 0.13) | 0.000 (0.02, 0.10) | 0.146 (0.10, 0.18) | 0.000 (0.03, 0.09) |
| exploit_broker | 0.035 (0.03, 0.09) | 0.104 (0.00, 0.09) | 0.042 (0.08, 0.17) | 0.146 (0.03, 0.09) |
| command_and_control_grid | 0.208 (0.41, 0.60) | 0.104 (0.24, 0.56) | 0.188 (0.44, 0.61) | 0.132 (0.33, 0.56) |
| capacity_market | 0.146 (0.05, 0.12) | 0.000 (0.01, 0.05) | 0.181 (0.08, 0.14) | 0.000 (0.01, 0.05) |
| dispatch_control_room | 0.014 (0.02, 0.15) | 0.000 (0.00, 0.06) | 0.021 (0.01, 0.15) | 0.000 (0.00, 0.07) |
| smart_grid_backbone | 0.042 (0.15, 0.56) | 0.000 (0.01, 0.50) | 0.062 (0.11, 0.55) | 0.000 (0.01, 0.55) |
| reconciliation_office | 0.243 (0.24, 0.10) | 0.361 (0.16, 0.03) | 0.215 (0.28, 0.15) | 0.361 (0.17, 0.05) |
| protocol_office | 0.299 (0.22, 0.19) | 0.347 (0.11, 0.09) | 0.243 (0.26, 0.19) | 0.382 (0.12, 0.08) |
| joint_task_force | 0.035 (0.06, 0.15) | 0.132 (0.03, 0.08) | 0.035 (0.04, 0.10) | 0.125 (0.03, 0.08) |
| defense_appropriations | 0.160 (0.10, 0.18) | 0.000 (0.02, 0.10) | 0.194 (0.08, 0.15) | 0.007 (0.03, 0.08) |
| homeland_security_act | 0.111 (0.28, 0.64) | 0.000 (0.06, 0.58) | 0.090 (0.28, 0.65) | 0.000 (0.03, 0.58) |
| preprint_server | 0.028 (0.07, 0.19) | 0.181 (0.02, 0.06) | 0.049 (0.05, 0.17) | 0.181 (0.02, 0.09) |
| supplier_network | 0.208 (0.19, 0.13) | 0.382 (0.15, 0.08) | 0.208 (0.18, 0.12) | 0.368 (0.14, 0.07) |
| fusion_research_campus | 0.111 (0.22, 0.64) | 0.007 (0.02, 0.58) | 0.111 (0.36, 0.67) | 0.007 (0.06, 0.62) |
| hyperscale_botnet | 0.021 (0.38, 0.56) | 0.132 (0.28, 0.56) | 0.153 (0.49, 0.62) | 0.118 (0.35, 0.56) |
| moonshot_grant | 0.007 (0.03, 0.15) | 0.000 (0.01, 0.06) | 0.049 (0.05, 0.11) | 0.007 (0.01, 0.06) |

Played 0 times: before/strategist: data_licensing_deal; before/greedy: data_licensing_deal, neural_mirror, intrusion_lab, tidal_barrage, positron_storage_ring, continental_supergrid, economies_of_scale, ransomware_syndicate, capacity_market, dispatch_control_room, smart_grid_backbone, defense_appropriations, homeland_security_act, moonshot_grant; after/strategist: data_licensing_deal; after/greedy: data_licensing_deal, intrusion_lab, tidal_barrage, positron_storage_ring, continental_supergrid, economies_of_scale, ransomware_syndicate, capacity_market, dispatch_control_room, smart_grid_backbone, homeland_security_act
