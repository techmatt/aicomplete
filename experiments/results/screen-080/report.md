# Cycle report: screen-080

080: the r34 ballpark read, liberation on: 2 players, the three board pairs, d2, 48 seeds a cell on the fresh set 30300; the sweep planner, sweep with always_liberate (the row that tells a bad price from an agent that won't play the card), and the engine. Target 45-85 a pair. Read with scripts/cards/liberation_read.py --cards; the off arm is screen-080-off.json on the same seeds.

432 games (9 cells × 48 seeds, 30300..=30347) on 5 threads in 405.4 s. Engine 0.1.0, rules r34, content 87229a9e0d536395.

- Configs: configs/baseline.json
- Variants: bp; br (ai.boards=0,2 ai.start_credits=40,40 ai.slots=5,2); pr (ai.boards=1,2 ai.start_credits=46,40 ai.slots=3,2)
- Difficulties: 2
- Agents (weakest first): sweep, sweep:always_liberate=1, engine
- Win probability: off
- Decision closeness: off
- Off measures are opt-in (docs/experiments.md): screen without them, then rerun the finalists with `--variants A,B --winprob N --closeness N`.
- Replay check: every game replays from seed + actions

## Findings

- Difficulty 2 (configs/baseline.json, bp): win rates engine 56%, planner:deals=1 71%, planner:deals=1,always_liberate=1 71%; skill gap +15 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, bp): survival (rounds; wins count extra) engine 11.04, planner:deals=1 11.58, planner:deals=1,always_liberate=1 11.48; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, br): win rates engine 79%, planner:deals=1 77%, planner:deals=1,always_liberate=1 92%; skill gap +12 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, br): survival (rounds; wins count extra) engine 12.12, planner:deals=1 12.10, planner:deals=1,always_liberate=1 12.52; the ladder is ordered.
- Difficulty 2 (configs/baseline.json, pr): win rates engine 60%, planner:deals=1 77%, planner:deals=1,always_liberate=1 88%; skill gap +27 pts; stronger agents never do worse.
- Difficulty 2 (configs/baseline.json, pr): survival (rounds; wins count extra) engine 11.21, planner:deals=1 11.67, planner:deals=1,always_liberate=1 12.19; the ladder is ordered.
- Game length for the strongest agent (planner:deals=1,always_liberate=1; configs/baseline.json, bp): 7.4 rounds at difficulty 2, 7.4 at 2 (target 8–10).
- Game length for the strongest agent (planner:deals=1,always_liberate=1; configs/baseline.json, br): 7.2 rounds at difficulty 2, 7.2 at 2 (target 8–10).
- Game length for the strongest agent (planner:deals=1,always_liberate=1; configs/baseline.json, pr): 7.1 rounds at difficulty 2, 7.1 at 2 (target 8–10).
- Games ended by: deck_empty 2, discord_deck 76, presence 32, purge 199, unity_deck 123.
- Adversary operations: 25.0 per round on average across cells, at most 98 in one round (target: low).
- Cards (planner:deals=1,always_liberate=1, offered at least 10 times): keeps per offer (opening deal included) under 10%: adaptive_malware, breakthrough_lab, breakthrough_leak, collateralized_grid, continental_supergrid, data_licensing_deal, dataset_scraper, dyson_swarm_tranche, exabyte_archive, force_protection_order, foundation_model_campus, hyperscale_botnet, kinetic_interdiction, liquidity_event, manifesto_leak, materials_hedge_desk, orbital_embassy, orbital_mass_driver, recursive_self_improvement, singularity_exploit, sovereign_capital_fund, summit_caucus, vendor_financing, vertical_monopoly; over 90%: aegis_grid_lockdown, agency_field_station, amnesty_decree, baseload_contract, benchmark_suite, blackout_script, bonded_warehouse, bridge_loan, brownout_relief, bulk_silicon_procurement, ceasefire_monitors, civic_watch, coalition_charter, commons_grid, consular_pouch, darknet_exchange, demand_response_grid, diplomatic_expulsion, embassy_compound, eviction_worm, expansion_fund, faraday_yard, forensic_audit, fusion_startup_stake, garrison_command, grad_student_pool, great_liberation, grid_census, hardened_bunker_complex, hardened_conduits, hardpoint_lockdown, hostile_buyout, inference_cluster, interceptor_battery, intrusion_countermeasures, intrusion_lab, joint_fabrication_pact, joint_security_compact, kill_team_roster, killswitch_directive, liaison_corps, lights_out_assembly, literature_crawler, market_watchdog, mutual_aid_pact, mutual_defense_pact, neural_mirror, nuclear_option, open_assembly, payroll_automation, peace_dividend, peacekeeper_rotation, peacekeeping_mission, peaker_turbines, planetary_referendum, popular_uprising, power_to_the_people, predictive_modeling, preprint_exchange, red_team_audit, relay_mast_rollout, retained_earnings, root_access, safehouse_network, security_ops_floor, sentinel_mesh, sentry_drone_wing, shared_compute_grant, shared_exploit_kit, shared_threat_intel, simulation_cluster, storm_hardening, substation_hub, supply_chain_treaty, telemetry_archive, threat_forecast_engine, treaty_grid_exchange, zero_click_breach, zero_knowledge_exploit.
- Luck share of survival (seed's share of the variance): 49% on average across groups (target: most variance from decisions).
- Help and counters (strongest agent): 8.36 gifts per game; 88% of threats countered.
- Strategy health at difficulty 2 (bp): the top agent there (planner:deals=1,always_liberate=1, 71% wins) plays 4.62 buildings and 6.27 tier-2 cards per game (of 20.9 cards); 94% of its wins had 2+ buildings played (88% with 2+ standing at the end). It builds (healthy: 2+ buildings per game); it plays tier 2.
- Variants against the base (bp) at difficulty 2, planner:deals=1,always_liberate=1, win rate difference in pts (bold: sign test p < 0.05): **br +20.8 ±7.9**; pr +16.7 ±7.5.

## Win rate by skill tier × difficulty

Win % ± standard error, then mean survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2). Skill gap: the top ladder tier's win rate minus the bottom tier's, each tier by its best agent (survival in brackets); the top tier is not always the strongest agent.

| bp difficulty 2 | engine | planner:deals=1 | planner:deals=1,always_liberate=1 | skill gap |
|---|---|---|---|---|
| win % (survival) | 56.2 ±7.2 (11.04) | 70.8 ±6.6 (11.58) | 70.8 ±6.6 (11.48) | +14.6 pts (+0.54 survival) |

| br difficulty 2 | engine | planner:deals=1 | planner:deals=1,always_liberate=1 | skill gap |
|---|---|---|---|---|
| win % (survival) | 79.2 ±5.9 (12.12) | 77.1 ±6.1 (12.10) | 91.7 ±4.0 (12.52) | +12.5 pts (+0.40 survival) |

| pr difficulty 2 | engine | planner:deals=1 | planner:deals=1,always_liberate=1 | skill gap |
|---|---|---|---|---|
| win % (survival) | 60.4 ±7.1 (11.21) | 77.1 ±6.1 (11.67) | 87.5 ±4.8 (12.19) | +27.1 pts (+0.98 survival) |

## Variants against the base

The first variant, `bp`, is the base. Both play the same seeds, so the difference (variant minus base, percentage points) carries the standard error of the per-seed difference; p: exact two-sided sign test on the seeds only one of them won.

| variant | difficulty | agent | win % base → variant | difference (pts) | p | rounds base → variant |
|---|---|---|---|---|---|---|
| br | 2 | planner:deals=1 | 70.8 → 77.1 | +6.2 ±7.5 | 0.581 | 7.75 → 8.15 |
| br | 2 | planner:deals=1,always_liberate=1 | 70.8 → 91.7 | +20.8 ±7.9 | 0.021 | 7.35 → 7.17 |
| br | 2 | engine | 56.2 → 79.2 | +22.9 ±8.5 | 0.019 | 8.17 → 8.27 |
| pr | 2 | planner:deals=1 | 70.8 → 77.1 | +6.2 ±7.5 | 0.581 | 7.75 → 7.38 |
| pr | 2 | planner:deals=1,always_liberate=1 | 70.8 → 87.5 | +16.7 ±7.5 | 0.057 | 7.35 → 7.10 |
| pr | 2 | engine | 56.2 → 60.4 | +4.2 ±10.7 | 0.845 | 8.17 → 7.83 |

## Game length, margin and end

| cell | rounds (sd, min..max) | ended by | win margin: deck left / supply left | loss distance: pieces over bar / level | supply left | cities corrupted / aligned at end |
|---|---|---|---|---|---|---|
| bp d2 planner:deals=1 | 7.75 (1.45, 5..11) | discord_deck 10, presence 4, purge 21, unity_deck 13 | 3.41 / 35.71 | 10.4 / 2.29 | 36.00 | 1.44 / 2.75 |
| bp d2 planner:deals=1,always_liberate=1 | 7.35 (1.33, 5..12) | deck_empty 1, discord_deck 6, presence 7, purge 25, unity_deck 9 | 3.82 / 34.79 | 8.9 / 2.36 | 36.08 | 1.25 / 2.50 |
| bp d2 engine | 8.17 (1.14, 6..11) | discord_deck 16, presence 5, purge 16, unity_deck 11 | 3.11 / 35.44 | 8.2 / 2.52 | 36.42 | 1.62 / 2.54 |
| br d2 planner:deals=1 | 8.15 (1.35, 5..11) | discord_deck 10, presence 1, purge 27, unity_deck 10 | 3.14 / 34.97 | 4.3 / 2.73 | 36.04 | 0.81 / 3.17 |
| br d2 planner:deals=1,always_liberate=1 | 7.17 (1.33, 4..9) | discord_deck 3, presence 1, purge 37, unity_deck 7 | 3.84 / 36.70 | 6.8 / 2.25 | 36.96 | 0.46 / 3.02 |
| br d2 engine | 8.27 (1.47, 5..11) | discord_deck 8, presence 2, purge 27, unity_deck 11 | 2.87 / 35.76 | 4.0 / 2.80 | 35.54 | 0.79 / 3.10 |
| pr d2 planner:deals=1 | 7.38 (1.41, 4..10) | discord_deck 6, presence 5, purge 16, unity_deck 21 | 3.57 / 35.49 | 11.2 / 2.27 | 35.85 | 1.54 / 2.60 |
| pr d2 planner:deals=1,always_liberate=1 | 7.10 (1.37, 4..10) | discord_deck 3, presence 3, purge 19, unity_deck 23 | 3.81 / 35.76 | 7.2 / 2.50 | 36.04 | 1.15 / 2.52 |
| pr d2 engine | 7.83 (1.66, 4..12) | deck_empty 1, discord_deck 14, presence 4, purge 11, unity_deck 18 | 3.59 / 35.17 | 6.5 / 2.53 | 36.83 | 1.85 / 2.42 |

## Decisions, adversary operations, help and counters

| cell | decisions/game | options/decision | forced | close (of real choices) | mean gap | adversary ops/round (max) | gives/game | pays/game | withdraws/game | threats countered/dealt per game |
|---|---|---|---|---|---|---|---|---|---|---|
| bp d2 planner:deals=1 | 462.2 | 14.91 | 1% | – | – | 26.84 (89) | 0.00 | 20.52 | 0.00 | 22.60 / 25.33 (89%) |
| bp d2 planner:deals=1,always_liberate=1 | 427.2 | 14.70 | 1% | – | – | 26.18 (87) | 0.00 | 18.90 | 0.00 | 21.33 / 24.21 (88%) |
| bp d2 engine | 505.4 | 15.50 | 1% | – | – | 27.84 (98) | 0.00 | 22.33 | 0.00 | 25.08 / 27.15 (92%) |
| br d2 planner:deals=1 | 493.1 | 19.94 | 1% | – | – | 21.84 (64) | 20.46 | 21.85 | 0.00 | 24.27 / 26.79 (91%) |
| br d2 planner:deals=1,always_liberate=1 | 412.6 | 17.92 | 1% | – | – | 20.82 (71) | 13.08 | 18.21 | 0.00 | 20.79 / 23.35 (89%) |
| br d2 engine | 508.6 | 20.32 | 1% | – | – | 22.32 (67) | 24.38 | 21.79 | 0.00 | 24.71 / 27.19 (91%) |
| pr d2 planner:deals=1 | 444.2 | 17.93 | 1% | – | – | 25.88 (79) | 16.08 | 19.02 | 0.00 | 21.56 / 24.35 (89%) |
| pr d2 planner:deals=1,always_liberate=1 | 418.5 | 17.07 | 1% | – | – | 25.53 (69) | 12.00 | 17.54 | 0.00 | 20.06 / 23.12 (87%) |
| pr d2 engine | 497.1 | 19.12 | 1% | – | – | 27.80 (80) | 19.90 | 20.98 | 0.00 | 23.52 / 26.04 (90%) |

## Luck share

Two-way split of outcome variance over seeds × agents: the seed's share (luck), the agent's share (skill) and the rest (interaction, and the agents' own randomness). On win/loss, and on survival (rounds reached; a win counts its rounds plus the adversary cards left plus 2, so it ranks above any loss), useful while nothing wins.

| group | win: luck / skill / rest | survival: luck / skill / rest |
|---|---|---|
| bp d2 | 44% / 2% / 54% | 50% / 1% / 49% |
| br d2 | 39% / 3% / 58% | 40% / 1% / 58% |
| pr d2 | 46% / 7% / 48% | 56% / 3% / 41% |

## Build variety

Distinct end-of-game builds (each seat's AI and buildings) and the mean Jaccard distance between two games' building sets (0 = always the same, 1 = nothing shared). Among wins where there are any.

| cell | distinct builds (all) | distance (all) | distinct builds (wins) | distance (wins) |
|---|---|---|---|---|
| bp d2 planner:deals=1 | 48 of 48 | 0.94 | 34 of 34 | 0.94 |
| bp d2 planner:deals=1,always_liberate=1 | 48 of 48 | 0.95 | 34 of 34 | 0.94 |
| bp d2 engine | 48 of 48 | 0.94 | 27 of 27 | 0.93 |
| br d2 planner:deals=1 | 48 of 48 | 0.93 | 37 of 37 | 0.94 |
| br d2 planner:deals=1,always_liberate=1 | 48 of 48 | 0.95 | 44 of 44 | 0.95 |
| br d2 engine | 48 of 48 | 0.94 | 38 of 38 | 0.94 |
| pr d2 planner:deals=1 | 48 of 48 | 0.96 | 37 of 37 | 0.96 |
| pr d2 planner:deals=1,always_liberate=1 | 46 of 48 | 0.95 | 41 of 42 | 0.94 |
| pr d2 engine | 47 of 48 | 0.96 | 29 of 29 | 0.95 |

## Strategy health

Per game, over the whole team: buildings played (any tier, AI unique buildings too), tier-2 cards played (buildings and programs), all cards played, economy buildings (printing standing income) standing at the end; then the share of wins with 2 or more buildings played, and in brackets with 2 or more buildings standing at the end.

| difficulty 2: buildings / tier 2 / cards / economy standing per game, wins with 2+ buildings | bp | br | pr |
|---|---|---|---|
| planner:deals=1 | 5.23 / 6.50 / 22.1 / 1.38, 97% (97%) | 5.15 / 7.10 / 27.6 / 2.15, 100% (100%) | 4.46 / 6.10 / 25.2 / 1.62, 97% (95%) |
| planner:deals=1,always_liberate=1 | 4.62 / 6.27 / 20.9 / 1.46, 94% (88%) | 4.71 / 6.83 / 24.3 / 1.69, 98% (98%) | 4.40 / 6.79 / 24.1 / 1.33, 98% (90%) |
| engine | 5.90 / 6.56 / 24.1 / 1.69, 93% (93%) | 5.94 / 7.96 / 28.5 / 1.96, 97% (97%) | 5.04 / 6.81 / 27.8 / 1.60, 97% (93%) |


## Unspent resources

Mean left at game end, per seat (credits / chips / energy / antimatter / ambassadors). `!`: over the flag, a seat's mean above credits 12, chips 4, energy 4, antimatter 2, ambassadors 3.

| cell | agent | seat 0 | seat 1 |
|---|---|---|---|
| bp d2 | planner:deals=1 | 27.0! / 4.3! / 6.3! / 0.8 / 0.1 | 31.1! / 2.1 / 9.5! / 0.0 / 0.7 |
| bp d2 | planner:deals=1,always_liberate=1 | 26.2! / 4.0 / 6.3! / 0.7 / 0.8 | 35.6! / 1.5 / 6.7! / 0.2 / 0.2 |
| bp d2 | engine | 24.8! / 5.1! / 6.0! / 1.4 / 0.5 | 26.0! / 2.0 / 7.0! / 0.2 / 0.4 |
| br d2 | planner:deals=1 | 29.3! / 9.8! / 8.8! / 0.9 / 0.2 | 32.8! / 5.9! / 10.6! / 0.0 / 0.3 |
| br d2 | planner:deals=1,always_liberate=1 | 35.1! / 7.2! / 7.4! / 0.4 / 0.2 | 33.8! / 4.3! / 7.8! / 0.1 / 0.1 |
| br d2 | engine | 29.9! / 9.8! / 7.9! / 1.6 / 0.4 | 29.5! / 6.5! / 9.5! / 0.1 / 0.3 |
| pr d2 | planner:deals=1 | 24.8! / 3.0 / 11.4! / 0.6 / 0.4 | 31.9! / 4.0 / 9.0! / 0.0 / 0.9 |
| pr d2 | planner:deals=1,always_liberate=1 | 25.1! / 3.6 / 12.4! / 0.5 / 0.4 | 29.8! / 3.6 / 6.5! / 0.2 / 0.5 |
| pr d2 | engine | 21.2! / 3.9 / 9.3! / 0.9 / 0.5 | 23.1! / 4.7! / 7.2! / 0.0 / 0.4 |

45 seat means over the flag.

## Pick and play rates per card

Over all cells of each agent. Offered: in a draft packet (each round's deal). Every dealt card is drafted by someone (packets pass until empty), so pick is the mean pick position, 1 = a seat's first pick of the round (lower = wanted sooner). Keep: keeps per offer; keeps count the opening deal and offers don't, so it can pass 100%. Play: played per card gained (kept or drawn; unique AI cards start in hand, so their plays are per game). Per variant, config and difficulty in `cards.csv`.

| card | engine: pick (offered) / keeps per offer / play | planner:deals=1: pick (offered) / keeps per offer / play | planner:deals=1,always_liberate=1: pick (offered) / keeps per offer / play |
|---|---|---|---|
| adaptive_malware | 2.8 (28) / 11% / 26% | 2.8 (29) / 10% / 19% | 2.8 (32) / 9% / 11% |
| adversarial_inference_sweep | 1.9 (97) / 65% / 87% | 1.9 (94) / 63% / 81% | 2.3 (81) / 65% / 72% |
| aegis_dome_array | 1.8 (84) / 86% / 4% | 1.9 (86) / 79% / 6% | 2.0 (75) / 85% / 9% |
| aegis_grid_lockdown | 2.1 (82) / 91% / 28% | 2.2 (78) / 92% / 17% | 2.0 (73) / 95% / 14% |
| agency_directorate | 1.7 (89) / 25% / 0% | 1.7 (86) / 29% / 0% | 2.0 (82) / 24% / 0% |
| agency_field_station | 3.0 (29) / 183% / 27% | 2.9 (22) / 209% / 32% | 3.1 (26) / 192% / 25% |
| amnesty_decree | 3.1 (42) / 126% / 83% | 3.1 (40) / 123% / 89% | 3.1 (42) / 107% / 82% |
| antitrust_action | 3.2 (43) / 44% / 13% | 2.9 (40) / 40% / 16% | 3.1 (38) / 32% / 19% |
| arc_furnace_complex | 1.8 (93) / 17% / 0% | 1.7 (94) / 17% / 0% | 2.0 (82) / 12% / 0% |
| arcology_foundry | 1.6 (89) / 15% / 31% | 1.4 (86) / 7% / 33% | 1.9 (79) / 10% / 25% |
| autonomous_defense_grid | 2.0 (108) / 27% / 59% | 2.1 (105) / 24% / 64% | 2.1 (94) / 17% / 44% |
| back_channel_talks | 2.9 (21) / 52% / 96% | 3.0 (25) / 64% / 92% | 3.0 (24) / 71% / 96% |
| baseload_contract | 2.9 (26) / 215% / 88% | 2.7 (29) / 203% / 88% | 2.8 (20) / 250% / 87% |
| beamed_power_constellation | 1.8 (72) / 44% / 72% | 1.7 (70) / 40% / 82% | 2.0 (65) / 46% / 57% |
| benchmark_suite | 2.9 (31) / 223% / 93% | 2.8 (29) / 234% / 89% | 3.0 (24) / 262% / 91% |
| black_ice_citadel | 1.9 (96) / 51% / 63% | 2.0 (86) / 43% / 54% | 2.1 (87) / 47% / 44% |
| blackout_script | 3.0 (40) / 138% / 81% | 3.2 (40) / 140% / 75% | 3.3 (35) / 146% / 74% |
| bonded_warehouse | 2.9 (36) / 150% / 11% | 3.0 (28) / 164% / 8% | 3.0 (30) / 157% / 7% |
| bounty_board | 3.2 (24) / 54% / 77% | 3.2 (24) / 50% / 77% | 3.3 (17) / 65% / 84% |
| breach_and_clear | 2.9 (38) / 76% / 75% | 2.8 (36) / 75% / 82% | 2.8 (35) / 74% / 79% |
| breakthrough_lab | 1.8 (93) / 6% / 0% | 1.9 (87) / 9% / 0% | 2.1 (80) / 4% / 0% |
| breakthrough_leak | 3.1 (38) / 16% / 46% | 2.8 (33) / 21% / 68% | 2.9 (34) / 6% / 56% |
| bridge_loan | 3.0 (47) / 143% / 60% | 3.1 (42) / 148% / 19% | 3.3 (38) / 155% / 38% |
| brownout_relief | 2.8 (34) / 138% / 94% | 3.2 (33) / 145% / 94% | 3.3 (33) / 142% / 92% |
| bulk_silicon_procurement | 3.2 (39) / 136% / 80% | 2.7 (34) / 141% / 86% | 3.1 (32) / 144% / 79% |
| capacitor_discharge | 3.3 (31) / 55% / 33% | 2.9 (34) / 41% / 34% | 2.9 (33) / 52% / 11% |
| ceasefire_monitors | 3.0 (36) / 153% / 96% | 3.0 (26) / 177% / 99% | 3.0 (26) / 181% / 95% |
| citation_engine | 2.8 (36) / 25% / 12% | 2.9 (32) / 16% / 15% | 2.9 (34) / 18% / 14% |
| civic_trust_bureau | 2.1 (79) / 61% / 60% | 1.9 (78) / 65% / 59% | 2.2 (69) / 48% / 58% |
| civic_watch | 2.9 (29) / 172% / 35% | 3.0 (32) / 166% / 28% | 3.2 (26) / 181% / 22% |
| coalition_charter | 1.6 (97) / 99% / 98% | 1.7 (87) / 98% / 94% | 2.0 (85) / 100% / 87% |
| collateralized_grid | 2.9 (28) / 7% / 7% | 3.1 (35) / 6% / 0% | 3.2 (25) / 4% / 0% |
| commons_grid | 3.1 (38) / 147% / 1% | 3.1 (38) / 147% / 1% | 3.1 (35) / 151% / 1% |
| consular_pouch | 2.9 (32) / 203% / 95% | 2.6 (25) / 228% / 95% | 2.9 (24) / 238% / 96% |
| consulate_tower | 3.1 (26) / 42% / 66% | 2.8 (31) / 42% / 55% | 2.8 (20) / 40% / 44% |
| continental_supergrid | 1.8 (91) / 4% / 75% | 1.7 (85) / 2% / 50% | 2.0 (80) / 4% / 33% |
| cryo_compute_burn | 3.3 (27) / 37% / 10% | 2.8 (24) / 38% / 3% | 3.1 (27) / 30% / 8% |
| darknet_exchange | 3.1 (25) / 264% / 36% | 3.1 (26) / 258% / 20% | 3.1 (18) / 328% / 24% |
| data_licensing_deal | 2.1 (81) / 1% / 0% | 2.1 (78) / 1% / 0% | 2.2 (68) / 0% / – |
| dataset_scraper | 2.9 (32) / 3% / 0% | 3.2 (33) / 0% / 0% | 3.3 (28) / 0% / 0% |
| deep_state_ledger | 2.1 (74) / 43% / 0% | 1.9 (73) / 32% / 0% | 2.0 (67) / 27% / 0% |
| demand_response_grid | 2.8 (35) / 143% / 3% | 2.6 (32) / 147% / 0% | 2.4 (32) / 147% / 0% |
| diplomatic_expulsion | 3.2 (41) / 173% / 99% | 2.7 (38) / 179% / 99% | 2.8 (40) / 175% / 99% |
| dyson_swarm_tranche | 3.3 (64) / 3% / 100% | 3.0 (66) / 2% / 0% | 2.7 (57) / 5% / 33% |
| embassy_compound | 2.7 (36) / 183% / 91% | 2.8 (34) / 188% / 92% | 2.7 (29) / 203% / 91% |
| endowed_research_chair | 3.3 (26) / 42% / 0% | 3.0 (25) / 24% / 0% | 3.0 (26) / 50% / 0% |
| eviction_worm | 2.9 (27) / 207% / 97% | 3.1 (24) / 225% / 99% | 3.1 (24) / 225% / 93% |
| exabyte_archive | 2.0 (84) / 2% / 0% | 1.9 (80) / 2% / 0% | 2.4 (73) / 4% / 0% |
| exodus_corridors | 1.7 (91) / 80% / 92% | 1.6 (93) / 89% / 90% | 1.9 (83) / 84% / 87% |
| expansion_fund | 3.0 (27) / 174% / 23% | 3.0 (36) / 156% / 17% | 2.9 (28) / 164% / 11% |
| fact_check_network | 3.2 (26) / 62% / 27% | 3.0 (22) / 73% / 33% | 3.3 (20) / 85% / 43% |
| faraday_yard | 2.8 (36) / 192% / 51% | 2.6 (30) / 210% / 54% | 2.9 (31) / 206% / 55% |
| field_grant | 3.0 (37) / 35% / 45% | 3.1 (40) / 22% / 52% | 3.2 (32) / 16% / 41% |
| field_sensor_grid | 2.9 (34) / 15% / 22% | 3.0 (44) / 20% / 13% | 3.2 (37) / 16% / 28% |
| force_protection_order | 3.3 (31) / 0% / 14% | 3.0 (30) / 0% / 11% | 3.1 (25) / 8% / 19% |
| forensic_audit | 2.1 (94) / 30% / 46% | 2.4 (86) / 28% / 50% | 1.1 (76) / 100% / 34% |
| foundation_model_campus | 3.2 (94) / 0% / – | 3.3 (84) / 0% / – | 3.0 (84) / 0% / – |
| fusion_startup_stake | 3.0 (34) / 176% / 79% | 2.7 (34) / 174% / 81% | 3.1 (34) / 176% / 79% |
| garrison_command | 3.2 (27) / 219% / 43% | 3.1 (27) / 222% / 23% | 3.0 (27) / 219% / 31% |
| grad_student_pool | 2.9 (32) / 159% / 12% | 3.1 (35) / 154% / 9% | 2.9 (27) / 170% / 9% |
| great_liberation | 3.3 (105) / 9% / 67% | 3.4 (102) / 2% / 0% | 1.1 (87) / 100% / 38% |
| grid_census | 3.0 (31) / 232% / 68% | 2.7 (35) / 217% / 68% | 2.8 (32) / 228% / 51% |
| grid_tied_smelter | 2.9 (35) / 26% / 46% | 3.0 (40) / 20% / 28% | 3.1 (34) / 21% / 19% |
| hardened_bunker_complex | 3.3 (21) / 214% / 58% | 3.3 (14) / 271% / 72% | 3.4 (17) / 241% / 63% |
| hardened_conduits | 3.2 (41) / 93% / 88% | 2.9 (32) / 81% / 93% | 2.6 (36) / 94% / 82% |
| hardpoint_lockdown | 3.1 (29) / 162% / 0% | 3.2 (32) / 156% / 0% | 3.0 (20) / 190% / 0% |
| honeypot_relay | 2.9 (21) / 205% / 20% | 3.0 (26) / 185% / 10% | 3.0 (8) / 350% / 11% |
| hostile_buyout | 1.7 (99) / 99% / 84% | 1.6 (93) / 98% / 92% | 1.1 (86) / 100% / 73% |
| hyperscale_botnet | 1.8 (95) / 7% / 14% | 1.8 (90) / 1% / 0% | 2.1 (83) / 1% / 0% |
| inference_cluster | 2.9 (38) / 163% / 2% | 3.1 (33) / 173% / 0% | 3.0 (35) / 169% / 1% |
| interceptor_battery | 3.2 (35) / 154% / 53% | 3.0 (43) / 149% / 44% | 3.1 (31) / 165% / 42% |
| interconnector_treaty | 1.7 (91) / 79% / 71% | 1.8 (78) / 76% / 73% | 2.1 (72) / 75% / 70% |
| intrusion_countermeasures | 3.0 (30) / 180% / 75% | 3.0 (30) / 187% / 82% | 2.8 (25) / 200% / 68% |
| intrusion_lab | 3.1 (44) / 139% / 6% | 2.9 (37) / 149% / 4% | 3.0 (40) / 145% / 2% |
| joint_fabrication_pact | 2.8 (25) / 156% / 94% | 2.8 (24) / 154% / 95% | 2.8 (22) / 164% / 98% |
| joint_security_compact | 2.8 (22) / 168% / 82% | 3.2 (25) / 160% / 76% | 2.7 (19) / 179% / 75% |
| kill_team_roster | 3.0 (21) / 190% / 11% | 2.9 (20) / 200% / 4% | 3.2 (23) / 187% / 2% |
| killswitch_directive | 1.8 (91) / 96% / 44% | 1.7 (85) / 95% / 36% | 2.1 (75) / 93% / 30% |
| kinetic_interdiction | 3.4 (86) / 10% / 89% | 3.4 (87) / 9% / 75% | 3.3 (83) / 10% / 62% |
| liaison_corps | 3.1 (29) / 128% / 10% | 3.1 (32) / 125% / 2% | 3.0 (27) / 130% / 2% |
| liberation_accords | 1.6 (90) / 52% / 87% | 1.6 (82) / 50% / 90% | 1.9 (78) / 46% / 89% |
| lights_out_assembly | 3.2 (37) / 141% / 12% | 3.1 (37) / 141% / 11% | 3.0 (41) / 134% / 13% |
| liquidity_event | 3.2 (21) / 0% / 0% | 2.8 (24) / 4% / 0% | 2.9 (22) / 0% / 0% |
| literature_crawler | 3.2 (25) / 184% / 81% | 3.0 (19) / 211% / 91% | 3.2 (19) / 216% / 79% |
| logic_bomb | 2.9 (35) / 34% / 71% | 3.0 (42) / 36% / 61% | 3.0 (34) / 35% / 59% |
| manifesto_leak | 3.0 (34) / 3% / 12% | 3.1 (34) / 6% / 8% | 3.1 (32) / 6% / 27% |
| market_stabilization_board | 2.2 (67) / 36% / 54% | 2.1 (64) / 36% / 57% | 2.4 (53) / 28% / 60% |
| market_watchdog | 3.2 (32) / 119% / 15% | 3.4 (24) / 125% / 21% | 3.3 (26) / 123% / 11% |
| martial_cordon | 1.6 (97) / 79% / 96% | 1.5 (91) / 79% / 96% | 1.9 (84) / 74% / 94% |
| materials_hedge_desk | 2.9 (44) / 5% / 0% | 2.9 (42) / 5% / 0% | 3.0 (43) / 2% / 0% |
| mutual_aid_pact | 3.0 (25) / 168% / 93% | 3.4 (25) / 160% / 93% | 3.3 (25) / 168% / 96% |
| mutual_defense_pact | 3.2 (30) / 163% / 34% | 3.0 (32) / 159% / 19% | 3.0 (28) / 168% / 14% |
| neural_mirror | 2.9 (35) / 103% / 31% | 3.1 (38) / 105% / 18% | 2.9 (31) / 106% / 22% |
| node_severance | 2.0 (86) / 33% / 93% | 1.9 (77) / 34% / 88% | 2.0 (73) / 30% / 82% |
| nuclear_option | 3.2 (86) / 28% / 67% | 3.4 (77) / 21% / 62% | 1.2 (75) / 100% / 57% |
| open_assembly | 3.2 (36) / 133% / 4% | 3.2 (42) / 133% / 5% | 3.1 (28) / 146% / 5% |
| open_weights_release | 1.9 (93) / 16% / 33% | 1.7 (80) / 11% / 22% | 2.1 (78) / 21% / 38% |
| orbital_embassy | 3.3 (99) / 14% / 21% | 3.5 (85) / 4% / 0% | 3.5 (81) / 6% / 40% |
| orbital_mass_driver | 3.3 (92) / 4% / 25% | 3.3 (84) / 1% / 100% | 3.0 (76) / 3% / 100% |
| orbital_observatory_array | 1.6 (95) / 48% / 0% | 1.4 (93) / 57% / 0% | 1.9 (82) / 44% / 0% |
| payroll_automation | 3.3 (36) / 114% / 61% | 2.9 (35) / 117% / 64% | 3.2 (32) / 119% / 57% |
| peace_dividend | 2.9 (46) / 117% / 99% | 2.8 (45) / 118% / 98% | 3.0 (43) / 121% / 91% |
| peacekeeper_rotation | 3.1 (30) / 127% / 96% | 2.8 (30) / 130% / 91% | 2.9 (33) / 127% / 93% |
| peacekeeping_mission | 1.8 (81) / 58% / 79% | 1.9 (80) / 66% / 75% | 1.2 (72) / 100% / 61% |
| peaker_turbines | 2.9 (23) / 130% / 73% | 3.2 (26) / 123% / 71% | 2.9 (13) / 162% / 77% |
| planetary_assembly | 1.6 (80) / 16% / 0% | 1.8 (76) / 17% / 0% | 2.1 (68) / 18% / 0% |
| planetary_referendum | 1.8 (99) / 88% / 99% | 1.9 (92) / 96% / 95% | 2.0 (82) / 94% / 92% |
| policy_institute | 2.1 (91) / 19% / 0% | 2.2 (79) / 22% / 0% | 2.0 (76) / 17% / 0% |
| popular_uprising | 1.9 (96) / 100% / 53% | 2.0 (89) / 100% / 62% | 1.1 (76) / 100% / 100% |
| positron_storage_ring | 1.5 (88) / 15% / 23% | 1.4 (92) / 13% / 25% | 1.9 (81) / 17% / 21% |
| power_to_the_people | 1.6 (96) / 24% / 61% | 1.6 (91) / 33% / 70% | 1.1 (81) / 100% / 48% |
| predictive_modeling | 3.2 (40) / 175% / 69% | 3.2 (38) / 179% / 72% | 3.4 (28) / 207% / 77% |
| prefab_plant_kit | 3.1 (24) / 62% / 40% | 3.2 (27) / 41% / 38% | 2.9 (28) / 43% / 25% |
| preprint_exchange | 2.7 (37) / 111% / 38% | 2.9 (39) / 115% / 37% | 2.9 (36) / 117% / 43% |
| quantum_decryptor | 2.7 (39) / 64% / 0% | 2.7 (34) / 56% / 2% | 2.8 (38) / 58% / 0% |
| recruit_operatives | 3.0 (29) / 24% / 0% | 3.3 (23) / 17% / 0% | 3.2 (22) / 14% / 0% |
| recursive_self_improvement | 3.5 (80) / 1% / 0% | 3.5 (79) / 1% / 0% | 3.3 (77) / 4% / 33% |
| red_team_audit | 2.9 (31) / 87% / 26% | 3.0 (34) / 82% / 16% | 3.1 (33) / 91% / 20% |
| relay_mast_rollout | 3.0 (40) / 105% / 80% | 3.2 (41) / 115% / 76% | 3.1 (38) / 116% / 81% |
| retained_earnings | 2.9 (30) / 117% / 3% | 2.9 (29) / 128% / 2% | 3.0 (27) / 122% / 4% |
| robotic_foundry | 2.7 (33) / 58% / 22% | 3.0 (34) / 62% / 32% | 2.9 (26) / 50% / 34% |
| root_access | 1.5 (93) / 38% / 66% | 1.6 (88) / 35% / 74% | 1.1 (83) / 100% / 46% |
| safehouse_network | 2.9 (31) / 135% / 28% | 3.0 (34) / 138% / 15% | 3.0 (33) / 142% / 16% |
| sample_repository | 2.6 (48) / 15% / 0% | 2.4 (35) / 3% / 7% | 2.8 (40) / 12% / 8% |
| security_contractor_hq | 2.9 (39) / 23% / 38% | 2.6 (36) / 17% / 42% | 2.7 (39) / 23% / 29% |
| security_ops_floor | 3.1 (43) / 109% / 22% | 3.1 (36) / 117% / 16% | 3.2 (35) / 114% / 10% |
| sentinel_command | 2.0 (95) / 66% / 60% | 1.9 (79) / 62% / 65% | 2.4 (80) / 57% / 78% |
| sentinel_mesh | 3.3 (34) / 94% / 20% | 3.1 (24) / 92% / 26% | 3.2 (21) / 100% / 29% |
| sentry_drone_wing | 3.3 (30) / 107% / 15% | 3.2 (26) / 112% / 4% | 3.1 (27) / 111% / 4% |
| shadow_broker_feed | 3.0 (27) / 11% / 10% | 2.7 (25) / 8% / 22% | 2.9 (25) / 20% / 25% |
| shared_compute_grant | 2.9 (34) / 112% / 61% | 2.8 (29) / 121% / 53% | 3.1 (32) / 119% / 54% |
| shared_exploit_kit | 3.1 (36) / 117% / 63% | 3.0 (34) / 121% / 74% | 3.1 (36) / 122% / 57% |
| shared_grid_accord | 2.9 (33) / 73% / 92% | 3.2 (33) / 88% / 93% | 3.0 (31) / 81% / 78% |
| shared_threat_intel | 3.1 (38) / 105% / 4% | 2.9 (37) / 97% / 17% | 3.1 (30) / 107% / 17% |
| signal_hijack | 3.1 (41) / 56% / 98% | 2.9 (39) / 54% / 88% | 3.0 (34) / 56% / 88% |
| simulation_cluster | 3.1 (36) / 100% / 2% | 3.0 (38) / 97% / 0% | 2.8 (37) / 97% / 0% |
| singularity_exploit | 3.1 (90) / 8% / 100% | 3.5 (82) / 4% / 100% | 3.5 (79) / 5% / 100% |
| sovereign_capital_fund | 1.9 (94) / 12% / 0% | 2.0 (89) / 10% / 0% | 2.1 (84) / 10% / 0% |
| spoofed_credentials | 2.9 (30) / 70% / 0% | 3.2 (26) / 58% / 6% | 3.1 (25) / 44% / 0% |
| station_prefab_order | 2.8 (26) / 46% / 75% | 2.8 (27) / 52% / 75% | 3.0 (25) / 44% / 70% |
| storm_hardening | 3.3 (31) / 135% / 10% | 3.2 (31) / 126% / 12% | 3.4 (24) / 146% / 11% |
| substation_hub | 2.9 (40) / 108% / 64% | 3.0 (39) / 110% / 55% | 2.8 (35) / 109% / 51% |
| summit_caucus | 3.0 (43) / 2% / 71% | 2.7 (38) / 3% / 73% | 2.9 (32) / 3% / 44% |
| supply_chain_audit | 2.6 (30) / 53% / 30% | 2.8 (24) / 58% / 32% | 2.5 (21) / 48% / 48% |
| supply_chain_treaty | 2.7 (48) / 98% / 68% | 2.9 (45) / 98% / 66% | 3.0 (45) / 100% / 55% |
| surplus_auction | 3.1 (28) / 100% / 76% | 3.1 (40) / 82% / 81% | 3.0 (31) / 84% / 70% |
| synthetic_data_forge | 3.1 (32) / 31% / 0% | 3.1 (41) / 37% / 0% | 3.0 (30) / 33% / 0% |
| telemetry_archive | 3.1 (35) / 94% / 5% | 3.2 (37) / 100% / 2% | 2.9 (32) / 103% / 2% |
| threat_forecast_engine | 3.1 (29) / 179% / 50% | 3.1 (28) / 182% / 64% | 3.2 (27) / 189% / 71% |
| threat_hunt | 3.2 (42) / 29% / 23% | 3.4 (33) / 39% / 10% | 3.3 (41) / 37% / 31% |
| tidal_barrage | 3.1 (32) / 19% / 62% | 3.0 (31) / 19% / 65% | 3.0 (29) / 14% / 50% |
| total_containment | 1.7 (98) / 42% / 88% | 1.6 (100) / 45% / 98% | 1.9 (97) / 37% / 86% |
| transmission_corridor | 3.2 (39) / 15% / 72% | 3.0 (39) / 18% / 76% | 3.2 (38) / 13% / 72% |
| treaty_grid_exchange | 3.0 (38) / 103% / 60% | 2.9 (38) / 103% / 58% | 2.9 (28) / 107% / 60% |
| truth_commission | 2.9 (35) / 46% / 79% | 3.2 (36) / 53% / 59% | 3.4 (36) / 56% / 74% |
| vendor_financing | 2.7 (30) / 0% / 0% | 3.1 (33) / 3% / 0% | 3.1 (29) / 0% / 0% |
| vertical_monopoly | 1.5 (86) / 5% / 50% | 1.7 (82) / 2% / 50% | 1.8 (71) / 4% / 67% |
| zero_click_breach | 2.9 (34) / 100% / 62% | 3.1 (34) / 94% / 50% | 2.8 (30) / 100% / 62% |
| zero_knowledge_exploit | 3.0 (22) / 109% / 81% | 3.0 (22) / 100% / 61% | 2.6 (18) / 106% / 51% |

## Timing

Per game on one thread (5 threads ran at once, so a lone run is somewhat faster). Agent: the agent's own decisions. Probes: closeness scoring and win-probability playouts. Total: everything, rules and replay check included.

| cell | agent ms/game | agent ms/decision | probes ms/game | total ms/game |
|---|---|---|---|---|
| bp d2 planner:deals=1 | 7192.2 | 15.559 | 0.0 | 7214.4 |
| bp d2 planner:deals=1,always_liberate=1 | 5569.5 | 13.037 | 0.0 | 5589.1 |
| bp d2 engine | 714.2 | 1.413 | 0.0 | 739.5 |
| br d2 planner:deals=1 | 9306.7 | 18.874 | 0.0 | 9331.6 |
| br d2 planner:deals=1,always_liberate=1 | 5783.5 | 14.017 | 0.0 | 5803.6 |
| br d2 engine | 950.8 | 1.869 | 0.0 | 977.9 |
| pr d2 planner:deals=1 | 6382.9 | 14.368 | 0.0 | 6405.2 |
| pr d2 planner:deals=1,always_liberate=1 | 5143.6 | 12.291 | 0.0 | 5163.9 |
| pr d2 engine | 751.1 | 1.511 | 0.0 | 778.3 |

Where the games' time went (summed over workers): agents 99.5%, probes 0.0%, apply 0.1%, legal 0.2%, replay check 0.1%, setup 0.0%, other 0.0%. Apply: the chosen actions and the measures' bookkeeping; legal: generating the legal actions; setup: building the game and the agent.

## Saved records

None matched.
