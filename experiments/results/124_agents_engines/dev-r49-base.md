## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| greedy | 432 | 49.1 | 4.91 | - | - | 53 / 44 / 51 |
| strategist | 432 | 70.6 | 6.02 | +21.5 ± 2.9 | +1.11 | 63 / 70 / 78 |

## Lost at the limits, a game

| cell | hold_lost.credits | hold_lost.energy | hold_lost.chips | hold_lost.ambassadors | production_lost.credits | energy: games cut |
|---|---|---|---|---|---|---|
| greedy | 0.00 | 0.57 | 0.05 | 0.20 | 0.00 | 14% |
| strategist | 1.41 | 4.37 | 1.27 | 0.46 | 0.42 | 56% |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | ambassadors |
|---|---|---|---|---|---|---|
| greedy | Builder | 42.7 / 116 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| greedy | Protector | 48.7 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.4 / 10 |
| greedy | Researcher | 43.3 / 65 | 16.5 / 48 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| strategist | Builder | 65.4 / 150 | 9.4 / 29 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| strategist | Protector | 56.8 / 150 | 11.5 / 31 | 6.8 / 10 | 3.3 / 10 | 4.5 / 10 |
| strategist | Researcher | 65.2 / 150 | 28.8 / 78 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |

## The Researcher: keeps against insight

Flag rules: `peak` is 122's (insight peak over 3x a round's insight income and over 10); `rise` is the fixed one (the peak's rise over the starting holding, same test). Keep rate: kept / offered after the opening; tier-2 share of its tier choices; insight at keep: mean held at its keep decisions.

| cell | rule | seats | n | insight peak | offered | kept | keep rate | tier-2 share | insight at keep |
|---|---|---|---|---|---|---|---|---|---|
| greedy | peak | not flagged | 139 | 12.1 | 13.8 | 5.0 | 36% | 76% | nan |
| greedy | peak | flagged | 149 | 20.7 | 15.1 | 4.2 | 28% | 77% | nan |
| greedy | peak | all | 288 | 16.5 | 14.5 | 4.6 | 32% | 77% | nan |
| strategist | peak | not flagged | 118 | 18.3 | 17.7 | 11.8 | 67% | 58% | nan |
| strategist | peak | flagged | 170 | 36.0 | 20.9 | 11.3 | 54% | 58% | nan |
| strategist | peak | all | 288 | 28.8 | 19.6 | 11.5 | 59% | 58% | nan |

## Plays a game (kept, offered): the r41 engine cards and the two named

| card | greedy | strategist |
|---|---|---|
| telemetry_archive | 0.007 (0.09, 0.08) | 0.032 (0.20, 0.17) |
| citation_engine | 0.141 (0.05, 0.09) | 0.069 (0.10, 0.15) |
| grad_student_pool | 0.398 (0.17, 0.09) | 0.308 (0.22, 0.17) |
| foundation_model_campus | 0.058 (0.12, 0.49) | 0.160 (0.31, 0.51) |
| data_licensing_deal | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) |
| retained_earnings | 0.005 (0.10, 0.07) | 0.225 (0.19, 0.16) |
| lights_out_assembly | 0.410 (0.23, 0.09) | 0.410 (0.27, 0.14) |
| honeypot_relay | 0.412 (0.20, 0.08) | 0.225 (0.24, 0.16) |
| neural_mirror | 0.019 (0.06, 0.07) | 0.190 (0.08, 0.13) |
| intrusion_lab | 0.023 (0.02, 0.07) | 0.035 (0.07, 0.13) |
| darknet_exchange | 0.368 (0.17, 0.07) | 0.248 (0.20, 0.16) |
| tidal_barrage | 0.000 (0.00, 0.10) | 0.021 (0.01, 0.12) |
| substation_hub | 0.037 (0.02, 0.06) | 0.139 (0.10, 0.13) |
| positron_storage_ring | 0.000 (0.06, 0.49) | 0.058 (0.23, 0.53) |
| continental_supergrid | 0.000 (0.00, 0.52) | 0.051 (0.16, 0.53) |
| open_assembly | 0.502 (0.16, 0.07) | 0.405 (0.22, 0.14) |
| peace_dividend | 0.025 (0.02, 0.07) | 0.183 (0.11, 0.17) |
| consulate_tower | 0.269 (0.03, 0.08) | 0.208 (0.09, 0.13) |
| orbital_embassy | 0.090 (0.16, 0.48) | 0.155 (0.30, 0.51) |
| agency_field_station | 0.312 (0.03, 0.08) | 0.123 (0.12, 0.15) |
| security_contractor_hq | 0.250 (0.04, 0.07) | 0.102 (0.09, 0.12) |
| agency_directorate | 0.046 (0.15, 0.51) | 0.051 (0.27, 0.53) |
| security_ops_floor | 0.181 (0.05, 0.09) | 0.035 (0.11, 0.14) |
| vertical_monopoly | 0.088 (0.11, 0.52) | 0.243 (0.37, 0.56) |
| power_to_the_people | 0.014 (0.03, 0.50) | 0.086 (0.18, 0.54) |
| research_royalties | 0.002 (0.02, 0.08) | 0.213 (0.10, 0.17) |
| interdisciplinary_institute | 0.213 (0.31, 0.53) | 0.236 (0.40, 0.54) |
| standardized_tooling | 0.458 (0.20, 0.07) | 0.384 (0.25, 0.16) |
| economies_of_scale | 0.009 (0.03, 0.07) | 0.236 (0.13, 0.19) |
| capital_projects_office | 0.507 (0.54, 0.46) | 0.410 (0.56, 0.50) |
| conglomerate_headquarters | 0.037 (0.28, 0.52) | 0.336 (0.45, 0.56) |
| ransomware_syndicate | 0.000 (0.02, 0.07) | 0.183 (0.07, 0.15) |
| exploit_broker | 0.183 (0.01, 0.07) | 0.065 (0.06, 0.17) |
| command_and_control_grid | 0.132 (0.25, 0.52) | 0.218 (0.38, 0.54) |
| capacity_market | 0.002 (0.00, 0.06) | 0.157 (0.06, 0.13) |
| dispatch_control_room | 0.000 (0.00, 0.10) | 0.012 (0.03, 0.17) |
| smart_grid_backbone | 0.000 (0.01, 0.57) | 0.028 (0.08, 0.58) |
| reconciliation_office | 0.396 (0.18, 0.08) | 0.231 (0.24, 0.16) |
| protocol_office | 0.463 (0.23, 0.07) | 0.380 (0.30, 0.16) |
| joint_task_force | 0.118 (0.02, 0.05) | 0.021 (0.08, 0.18) |
| defense_appropriations | 0.007 (0.03, 0.09) | 0.111 (0.08, 0.17) |
| homeland_security_act | 0.000 (0.04, 0.49) | 0.076 (0.19, 0.50) |
| preprint_server | 0.227 (0.03, 0.07) | 0.100 (0.11, 0.14) |
| supplier_network | 0.310 (0.10, 0.08) | 0.160 (0.13, 0.12) |
| fusion_research_campus | 0.005 (0.01, 0.55) | 0.088 (0.19, 0.54) |
| hyperscale_botnet | 0.116 (0.25, 0.48) | 0.009 (0.36, 0.49) |
| moonshot_grant | 0.007 (0.00, 0.07) | 0.019 (0.03, 0.16) |

Played 0 times: greedy: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; strategist: data_licensing_deal
