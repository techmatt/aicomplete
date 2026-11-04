# 105: play counts at r41

Per game, from the final-build read (`experiments/screen-105-r41.json`, seeds 33500-33547, 48 games a cell, d2;
`out/play_counts_105.py` reads its `games.jsonl`). Two countries seat Horizon and Cintamani only: Onyx's and Biomia's
converters are not on the map. "Fired" counts a card's triggers and Action uses.

| Per game | bp strategist | bp greedy | br strategist | br greedy | pr strategist | pr greedy |
|---|---|---|---|---|---|---|
| win % | 21 | 4 | 42 | 4 | 25 | 4 |
| Focus (Builder, 5 credits → 1 insight) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Synthesis (Researcher, a card → 2 insight) | 0.00 | 0.00 | 5.98 | 0.00 | 5.77 | 0.00 |
| Vigil (Protector, insight per troop stopped) | 4.46 | 3.69 | 0.00 | 0.00 | 6.92 | 5.88 |
| Cube converter (standard action) | 3.35 | 2.12 | 2.92 | 2.27 | 1.69 | 0.58 |
| Horizon converter | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Cintamani converter (1 cube → 2 insight) | 1.69 | 1.04 | 3.35 | 2.29 | 3.21 | 1.29 |
| Onyx converter (once a round) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Insight paid on threat counters | 7.31 | 5.33 | 10.15 | 5.94 | 9.10 | 6.29 |
| Triggers fired, all | 9.12 | 4.79 | 4.96 | 0.69 | 11.92 | 6.08 |
| Thresholds reached | 0.17 | 0.02 | 0.27 | 0.04 | 0.21 | 0.00 |
| Copies (Payroll Automation) | 0.15 | 0.00 | 0.15 | 0.00 | 0.04 | 0.00 |
| Reuses (Simulation Cluster) | 1.02 | 0.00 | 1.35 | 0.02 | 1.12 | 0.00 |
| Credits saved by card discounts | 1.67 | 0.62 | 2.75 | 0.69 | 2.46 | 0.60 |
| Insight saved by card discounts | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |

| Card: kept / played / fired a game | bp strategist | bp greedy | br strategist | br greedy | pr strategist | pr greedy |
|---|---|---|---|---|---|---|
| citation_engine | 0.04 / 0.02 / 0.10 | 0.00 / 0.00 / 0.00 | 0.04 / 0.02 / 0.12 | 0.02 / 0.04 / 0.15 | 0.02 / 0.04 / 0.15 | 0.00 / 0.00 / 0.00 |
| quantum_decryptor | 0.06 / 0.02 / 0.02 | 0.02 / 0.00 / 0.00 | 0.06 / 0.00 / 0.00 | 0.02 / 0.00 / 0.00 | 0.02 / 0.00 / 0.00 | 0.00 / 0.00 / 0.00 |
| grid_tied_smelter | 0.06 / 0.02 / 0.04 | 0.00 / 0.00 / 0.00 | 0.04 / 0.02 / 0.06 | 0.02 / 0.00 / 0.00 | 0.02 / 0.06 / 0.08 | 0.00 / 0.00 / 0.00 |
| consulate_tower | 0.02 / 0.02 / 0.04 | 0.02 / 0.00 / 0.00 | 0.15 / 0.00 / 0.00 | 0.02 / 0.00 / 0.00 | 0.10 / 0.04 / 0.17 | 0.02 / 0.00 / 0.00 |
| lights_out_assembly | 0.33 / 0.31 / 0.52 | 0.27 / 0.04 / 0.04 | 0.31 / 0.31 / 0.67 | 0.21 / 0.00 / 0.00 | 0.29 / 0.23 / 0.42 | 0.25 / 0.00 / 0.00 |
| darknet_exchange | 0.25 / 0.27 / 0.00 | 0.21 / 0.15 / 0.00 | 0.27 / 0.29 / 0.00 | 0.27 / 0.12 / 0.00 | 0.21 / 0.25 / 0.00 | 0.21 / 0.10 / 0.00 |
| hyperscale_botnet | 0.15 / 0.00 / 0.00 | 0.02 / 0.00 / 0.00 | 0.02 / 0.00 / 0.00 | 0.02 / 0.00 / 0.00 | 0.02 / 0.00 / 0.00 | 0.00 / 0.00 / 0.00 |
| payroll_automation | 0.40 / 0.15 / 0.00 | 0.44 / 0.00 / 0.00 | 0.48 / 0.15 / 0.00 | 0.40 / 0.00 / 0.00 | 0.35 / 0.04 / 0.00 | 0.33 / 0.00 / 0.00 |
| security_contractor_hq | 0.19 / 0.12 / 0.27 | 0.04 / 0.00 / 0.00 | 0.10 / 0.04 / 0.04 | 0.00 / 0.00 / 0.00 | 0.12 / 0.10 / 0.27 | 0.02 / 0.00 / 0.00 |
| security_ops_floor | 0.29 / 0.25 / 1.04 | 0.23 / 0.06 / 0.40 | 0.27 / 0.21 / 0.96 | 0.25 / 0.06 / 0.23 | 0.25 / 0.21 / 0.73 | 0.21 / 0.02 / 0.06 |
| honeypot_relay | 0.31 / 0.25 / 1.40 | 0.27 / 0.02 / 0.35 | 0.31 / 0.23 / 1.52 | 0.31 / 0.00 / 0.00 | 0.35 / 0.29 / 1.85 | 0.31 / 0.00 / 0.00 |
| telemetry_archive | 0.33 / 0.35 / 1.04 | 0.38 / 0.19 / 0.31 | 0.38 / 0.35 / 1.10 | 0.38 / 0.17 / 0.29 | 0.44 / 0.38 / 1.15 | 0.31 / 0.10 / 0.15 |
| simulation_cluster | 0.42 / 0.31 / 1.02 | 0.33 / 0.02 / 0.00 | 0.35 / 0.33 / 1.35 | 0.42 / 0.02 / 0.02 | 0.38 / 0.35 / 1.12 | 0.38 / 0.00 / 0.00 |
| data_licensing_deal | 0.40 / 0.27 / 0.19 | 0.38 / 0.00 / 0.00 | 0.56 / 0.29 / 0.48 | 0.52 / 0.02 / 0.02 | 0.42 / 0.25 / 0.19 | 0.48 / 0.00 / 0.00 |
