## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| before/greedy | 432 | 49.1 | 4.91 | -21.5 ± 2.9 | -1.11 | 53 / 44 / 51 |
| before/strategist | 432 | 70.6 | 6.02 | - | - | 63 / 70 / 78 |
| after/greedy | 432 | 48.1 | 4.94 | -22.5 ± 3.1 | -1.07 | 53 / 44 / 48 |
| after/strategist | 432 | 74.3 | 6.04 | +3.7 ± 2.5 | +0.02 | 69 / 72 / 82 |

## Lost at the limits, a game

| cell | hold_lost.credits | hold_lost.energy | hold_lost.chips | hold_lost.ambassadors | production_lost.credits | energy: games cut |
|---|---|---|---|---|---|---|
| before/greedy | 0.00 | 0.57 | 0.05 | 0.20 | 0.00 | 14% |
| before/strategist | 1.41 | 4.37 | 1.27 | 0.46 | 0.42 | 56% |
| after/greedy | 0.00 | 0.53 | 0.20 | 0.36 | 0.00 | 14% |
| after/strategist | 2.19 | 3.35 | 1.41 | 0.26 | 0.52 | 51% |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | ambassadors |
|---|---|---|---|---|---|---|
| before/greedy | Builder | 42.7 / 116 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| before/greedy | Protector | 48.7 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.4 / 10 |
| before/greedy | Researcher | 43.3 / 65 | 16.5 / 48 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| before/strategist | Builder | 65.4 / 150 | 9.4 / 29 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| before/strategist | Protector | 56.8 / 150 | 11.5 / 31 | 6.8 / 10 | 3.3 / 10 | 4.5 / 10 |
| before/strategist | Researcher | 65.2 / 150 | 28.8 / 78 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |
| after/greedy | Builder | 42.8 / 113 | 8.2 / 16 | 3.8 / 10 | 3.1 / 10 | 4.5 / 10 |
| after/greedy | Protector | 49.0 / 91 | 10.0 / 21 | 5.4 / 10 | 2.8 / 10 | 4.3 / 10 |
| after/greedy | Researcher | 43.5 / 67 | 14.9 / 62 | 6.2 / 10 | 2.4 / 10 | 4.7 / 10 |
| after/strategist | Builder | 64.2 / 150 | 10.3 / 29 | 8.3 / 10 | 4.8 / 10 | 3.7 / 10 |
| after/strategist | Protector | 56.9 / 147 | 12.4 / 29 | 6.6 / 10 | 3.5 / 10 | 4.6 / 10 |
| after/strategist | Researcher | 65.8 / 150 | 21.6 / 57 | 6.6 / 10 | 3.4 / 10 | 4.3 / 10 |

## The Researcher: keeps against insight

Flag rules: `peak` is 122's (insight peak over 3x a round's insight income and over 10); `rise` is the fixed one (the peak's rise over the starting holding, same test). Keep rate: kept / offered after the opening; tier-2 share of its tier choices; insight at keep: mean held at its keep decisions.

| cell | rule | seats | n | insight peak | offered | kept | keep rate | tier-2 share | insight at keep |
|---|---|---|---|---|---|---|---|---|---|
| before/greedy | peak | not flagged | 139 | 12.1 | 13.8 | 5.0 | 36% | 76% | nan |
| before/greedy | peak | flagged | 149 | 20.7 | 15.1 | 4.2 | 28% | 77% | nan |
| before/greedy | peak | all | 288 | 16.5 | 14.5 | 4.6 | 32% | 77% | nan |
| before/strategist | peak | not flagged | 118 | 18.3 | 17.7 | 11.8 | 67% | 58% | nan |
| before/strategist | peak | flagged | 170 | 36.0 | 20.9 | 11.3 | 54% | 58% | nan |
| before/strategist | peak | all | 288 | 28.8 | 19.6 | 11.5 | 59% | 58% | nan |
| after/greedy | peak | not flagged | 181 | 13.2 | 14.6 | 6.5 | 45% | 76% | 10.1 |
| after/greedy | peak | flagged | 107 | 17.7 | 14.7 | 6.9 | 47% | 78% | 12.7 |
| after/greedy | peak | all | 288 | 14.9 | 14.6 | 6.7 | 46% | 77% | 11.1 |
| after/greedy | rise | not flagged | 283 | 14.5 | 14.5 | 6.6 | 45% | 77% | 11.0 |
| after/greedy | rise | flagged | 5 | 38.0 | 21.4 | 11.8 | 55% | 85% | 19.7 |
| after/strategist | peak | not flagged | 206 | 19.0 | 18.7 | 13.7 | 73% | 61% | 14.0 |
| after/strategist | peak | flagged | 82 | 28.0 | 21.5 | 15.9 | 74% | 59% | 18.1 |
| after/strategist | peak | all | 288 | 21.6 | 19.5 | 14.3 | 73% | 61% | 15.2 |
| after/strategist | rise | not flagged | 267 | 20.1 | 19.1 | 13.9 | 73% | 60% | 14.6 |
| after/strategist | rise | flagged | 21 | 39.5 | 24.4 | 19.1 | 78% | 66% | 23.0 |

## Plays a game (kept, offered): the r41 engine cards and the two named

| card | before/greedy | before/strategist | after/greedy | after/strategist |
|---|---|---|---|---|
| telemetry_archive | 0.007 (0.09, 0.08) | 0.032 (0.20, 0.17) | 0.007 (0.10, 0.09) | 0.028 (0.19, 0.16) |
| citation_engine | 0.141 (0.05, 0.09) | 0.069 (0.10, 0.15) | 0.155 (0.04, 0.06) | 0.046 (0.10, 0.15) |
| grad_student_pool | 0.398 (0.17, 0.09) | 0.308 (0.22, 0.17) | 0.380 (0.17, 0.08) | 0.292 (0.21, 0.11) |
| foundation_model_campus | 0.058 (0.12, 0.49) | 0.160 (0.31, 0.51) | 0.074 (0.14, 0.49) | 0.204 (0.37, 0.52) |
| data_licensing_deal | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) |
| retained_earnings | 0.005 (0.10, 0.07) | 0.225 (0.19, 0.16) | 0.005 (0.10, 0.08) | 0.252 (0.23, 0.17) |
| lights_out_assembly | 0.410 (0.23, 0.09) | 0.410 (0.27, 0.14) | 0.431 (0.22, 0.08) | 0.431 (0.30, 0.16) |
| honeypot_relay | 0.412 (0.20, 0.08) | 0.225 (0.24, 0.16) | 0.426 (0.21, 0.07) | 0.204 (0.25, 0.14) |
| neural_mirror | 0.019 (0.06, 0.07) | 0.190 (0.08, 0.13) | 0.023 (0.06, 0.07) | 0.255 (0.11, 0.14) |
| intrusion_lab | 0.023 (0.02, 0.07) | 0.035 (0.07, 0.13) | 0.021 (0.03, 0.07) | 0.028 (0.08, 0.15) |
| darknet_exchange | 0.368 (0.17, 0.07) | 0.248 (0.20, 0.16) | 0.384 (0.18, 0.08) | 0.282 (0.21, 0.12) |
| tidal_barrage | 0.000 (0.00, 0.10) | 0.021 (0.01, 0.12) | 0.000 (0.00, 0.09) | 0.019 (0.01, 0.14) |
| substation_hub | 0.037 (0.02, 0.06) | 0.139 (0.10, 0.13) | 0.039 (0.02, 0.06) | 0.130 (0.09, 0.14) |
| positron_storage_ring | 0.000 (0.06, 0.49) | 0.058 (0.23, 0.53) | 0.000 (0.05, 0.49) | 0.069 (0.25, 0.54) |
| continental_supergrid | 0.000 (0.00, 0.52) | 0.051 (0.16, 0.53) | 0.000 (0.00, 0.52) | 0.053 (0.16, 0.56) |
| open_assembly | 0.502 (0.16, 0.07) | 0.405 (0.22, 0.14) | 0.479 (0.16, 0.08) | 0.375 (0.21, 0.15) |
| peace_dividend | 0.025 (0.02, 0.07) | 0.183 (0.11, 0.17) | 0.012 (0.03, 0.08) | 0.185 (0.09, 0.12) |
| consulate_tower | 0.269 (0.03, 0.08) | 0.208 (0.09, 0.13) | 0.259 (0.03, 0.08) | 0.169 (0.10, 0.14) |
| orbital_embassy | 0.090 (0.16, 0.48) | 0.155 (0.30, 0.51) | 0.081 (0.18, 0.47) | 0.132 (0.38, 0.55) |
| agency_field_station | 0.312 (0.03, 0.08) | 0.123 (0.12, 0.15) | 0.343 (0.03, 0.09) | 0.095 (0.09, 0.13) |
| security_contractor_hq | 0.250 (0.04, 0.07) | 0.102 (0.09, 0.12) | 0.255 (0.04, 0.07) | 0.095 (0.10, 0.13) |
| agency_directorate | 0.046 (0.15, 0.51) | 0.051 (0.27, 0.53) | 0.051 (0.17, 0.50) | 0.056 (0.40, 0.56) |
| security_ops_floor | 0.181 (0.05, 0.09) | 0.035 (0.11, 0.14) | 0.204 (0.05, 0.09) | 0.028 (0.10, 0.14) |
| vertical_monopoly | 0.088 (0.11, 0.52) | 0.243 (0.37, 0.56) | 0.074 (0.14, 0.53) | 0.296 (0.45, 0.58) |
| power_to_the_people | 0.014 (0.03, 0.50) | 0.086 (0.18, 0.54) | 0.009 (0.09, 0.51) | 0.093 (0.25, 0.57) |
| research_royalties | 0.002 (0.02, 0.08) | 0.213 (0.10, 0.17) | 0.005 (0.03, 0.08) | 0.201 (0.08, 0.15) |
| interdisciplinary_institute | 0.213 (0.31, 0.53) | 0.236 (0.40, 0.54) | 0.227 (0.34, 0.55) | 0.250 (0.48, 0.59) |
| standardized_tooling | 0.458 (0.20, 0.07) | 0.384 (0.25, 0.16) | 0.444 (0.20, 0.06) | 0.384 (0.26, 0.15) |
| economies_of_scale | 0.009 (0.03, 0.07) | 0.236 (0.13, 0.19) | 0.005 (0.03, 0.07) | 0.211 (0.13, 0.16) |
| capital_projects_office | 0.507 (0.54, 0.46) | 0.410 (0.56, 0.50) | 0.532 (0.57, 0.49) | 0.470 (0.62, 0.53) |
| conglomerate_headquarters | 0.037 (0.28, 0.52) | 0.336 (0.45, 0.56) | 0.028 (0.35, 0.54) | 0.345 (0.49, 0.58) |
| ransomware_syndicate | 0.000 (0.02, 0.07) | 0.183 (0.07, 0.15) | 0.000 (0.02, 0.07) | 0.192 (0.09, 0.17) |
| exploit_broker | 0.183 (0.01, 0.07) | 0.065 (0.06, 0.17) | 0.181 (0.02, 0.06) | 0.051 (0.06, 0.14) |
| command_and_control_grid | 0.132 (0.25, 0.52) | 0.218 (0.38, 0.54) | 0.146 (0.28, 0.51) | 0.243 (0.44, 0.56) |
| capacity_market | 0.002 (0.00, 0.06) | 0.157 (0.06, 0.13) | 0.000 (0.01, 0.07) | 0.130 (0.06, 0.12) |
| dispatch_control_room | 0.000 (0.00, 0.10) | 0.012 (0.03, 0.17) | 0.002 (0.00, 0.09) | 0.012 (0.03, 0.17) |
| smart_grid_backbone | 0.000 (0.01, 0.57) | 0.028 (0.08, 0.58) | 0.000 (0.00, 0.56) | 0.023 (0.08, 0.59) |
| reconciliation_office | 0.396 (0.18, 0.08) | 0.231 (0.24, 0.16) | 0.387 (0.19, 0.09) | 0.167 (0.25, 0.15) |
| protocol_office | 0.463 (0.23, 0.07) | 0.380 (0.30, 0.16) | 0.465 (0.24, 0.07) | 0.356 (0.31, 0.14) |
| joint_task_force | 0.118 (0.02, 0.05) | 0.021 (0.08, 0.18) | 0.148 (0.02, 0.06) | 0.016 (0.07, 0.16) |
| defense_appropriations | 0.007 (0.03, 0.09) | 0.111 (0.08, 0.17) | 0.005 (0.04, 0.10) | 0.130 (0.07, 0.14) |
| homeland_security_act | 0.000 (0.04, 0.49) | 0.076 (0.19, 0.50) | 0.000 (0.06, 0.50) | 0.049 (0.18, 0.52) |
| preprint_server | 0.227 (0.03, 0.07) | 0.100 (0.11, 0.14) | 0.243 (0.03, 0.07) | 0.069 (0.11, 0.14) |
| supplier_network | 0.310 (0.10, 0.08) | 0.160 (0.13, 0.12) | 0.324 (0.10, 0.08) | 0.157 (0.12, 0.11) |
| fusion_research_campus | 0.005 (0.01, 0.55) | 0.088 (0.19, 0.54) | 0.016 (0.06, 0.56) | 0.100 (0.28, 0.57) |
| hyperscale_botnet | 0.116 (0.25, 0.48) | 0.009 (0.36, 0.49) | 0.111 (0.28, 0.46) | 0.120 (0.39, 0.51) |
| moonshot_grant | 0.007 (0.00, 0.07) | 0.019 (0.03, 0.16) | 0.005 (0.02, 0.08) | 0.023 (0.07, 0.15) |

Played 0 times: before/greedy: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; before/strategist: data_licensing_deal; after/greedy: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, capacity_market, smart_grid_backbone, homeland_security_act; after/strategist: data_licensing_deal
