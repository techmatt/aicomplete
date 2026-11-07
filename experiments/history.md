# History

An index of past items' write-ups, newest first. Each write-up is a record of its day: its
numbers are as measured on the rules revision it names, and its specs and records may not
run or replay today. What each revision changed is one line in
[docs/rules-history.md](../docs/rules-history.md); everything else is in git.
The current recipes and costs are in [docs/experiments.md](../docs/experiments.md).

| Item | Rev | Write-up |
|---|---|---|
| 142 threat intensity and the credit economy (claiming sites, the difficulty ladder, no node counters) | r59 | [142_threat_intensity/README.md](results/142_threat_intensity/README.md) |
| 141 speed at r58: the standard read 146 → 88 s (1.66x wall, 1.42x a decision) with every game the same; Rush Build's sacrifice test (70% of a long game) and the card and fact lookups | r58 | [141_speed_r58/README.md](results/141_speed_r58/README.md) |
| 140 cleanup: purge-level enum, megaproject `after` and `corrupt_city`'s unused fields cut, one variant list, fast lane 10.1 to 4.8 s, the web e2e fixed to r58; same play | r58 | [140_cleanup_tests_web/README.md](results/140_cleanup_tests_web/README.md) |
| 139 Discord cards and the band: node loss local only (Discord cards, projects and Border War's global line converted; Offensive deals a threat onto the most crowded node hex), Claim 4 / 3 / 3, the clock and blocker dial cut; calibrated with board credits -3, the corrupt cut at d1 and the extra threat at every level; strategist 77.8 / 61.8 / 51.4% (d0-d2) against r57's 91.7 / 75.7 / 52.8 on the same seeds, d2 median 9; no Discord-card deaths | r58 | [139_discord_and_band/README.md](results/139_discord_and_band/README.md) |
| 138 corruption and arrivals: Corrupt X and natural corruption replace the spread, bounded creation lines replace arrivals, site creation, land corruption; calibrated with stage III sooner, counters 1.5x and an uncountered threat's node harm; strategist 88.2 / 84.7 / 60.4% (d0-d2; uncountered threats destroy 1 / 2 / 3 nodes by tier) against r56's 73.6 / 61.8 / 52.1 on the same seeds (as built 82.6 / 90.3 / 88.9), median 9 rounds at every level; corruption 1.6-1.8 nodes a game (r56 4.6-5.3); d0-d1 targets missed | r57 | [138_corruption_and_arrivals/README.md](results/138_corruption_and_arrivals/README.md) |
| 137 sites and war: one site model (Claim, Liberate, blocking, armor), bases as front troop piles, war every round, board income +6; strategist 73.6 / 62.5 / 49.3% (d0-d2) against r55's 74.3 / 57.6 / 43.1 on the same seeds, as asked 61.8 / 43.8 / 27.8; the purge win route shrinks (standing blockers, more botnets); the death audit continued | r56 | [137_sites_and_war/README.md](results/137_sites_and_war/README.md) |
| 136 alignment and the win: two independent tracks (Unity may hold Discord off; Envoy), all 9 Unity cards, the purge by Unity cards drawn; median game 6 → 8 rounds at every level, no Unity win before round 6, strategist 79.9 / 71.5 / 63.9% (d0-d2); the death audit of r54's node losses | r55 | [136_alignment_and_win/README.md](results/136_alignment_and_win/README.md) |
| 135 the new adversary model built and played with every 134 draft (threat decks by tier, location tokens and near, the counter menu, event pulls, project slots, production then war); d2 strategist 61.8% (r53 70.8%) | r54 | [135_adversary_model/README.md](results/135_adversary_model/README.md) |
| 134 adversary decks drafted for the new model (content for review, not loaded): tier-1/2/3 threats 25/28/11, projects 20, the Militarist's board and 14 events; picker `adversary_drafts` | r53 | [134_militarist_drafts/README.md](results/134_militarist_drafts/README.md) |
| 131 agents' value table per agent (`t.<key>=v` cells, no rebuild); chips 1.5 → 2 (Antitrust no longer a loss at the table), auto_counter 1.5 → 0.67 a point (a fire's cut plus the executions it saves); agents only | r53 | [131_agents_table/r53.md](results/131_agents_table/r53.md) |
| 130 late energy sinks: Particle Accelerator's Action gets a research-cube half; four new sinks (Directed-Energy Array, Holographic Assembly Hall, Wafer Fab Line, tier-2 Exascale Training Cluster); Positron Storage Ring draws 2; overrides count once built; energy's anchor kept | r53 | [130_late_energy_sinks/r53.md](results/130_late_energy_sinks/r53.md) |
| 129 cleanup: 124-128's audit notes checked against the code; agents' Strategic trait trimmed to one `eval()`, a generated draft-set block, past specs archived, ledger `--overlay`; no play change | r52 | [129_cleanup/notes-check.md](results/129_cleanup/notes-check.md) |
| 128 draft and price: agents value per-tag ops at their played tag count, price spare energy cheap and skip dead cube Actions; 127's cheapened cards back at the anchors (16 → 5 deck cards at 5 credits or less) | r52 | [128_draft_and_price/r52.md](results/128_draft_and_price/r52.md) |
| 127 no limits: r49's holding limits and credit-production cap removed, and the agents' limit rules with them; runaway engines fixed at their cards (Orbital Embassy, the per-tag credit programs, the zero-insight draw cards, Antitrust Action, energy costs on three factories) | r51 | [127_no_limits/r51.md](results/127_no_limits/r51.md) |
| 126 auto_counter from play: the value table's entry at the fire rate measured in play (0.30 a holder-round, the strategist), 2.5 → 1.5 a point; agents only | r50 | [126_auto_counter/r50.md](results/126_auto_counter/r50.md) |
| 125 energy sinks: energy a second cost on five power-hungry buildings, credits cut to match; Particle Accelerator and Citywide Microgrid as late sinks; per-seat loss, spending and its phase in `games.jsonl` | r50 | [125_energy_sinks/r50.md](results/125_energy_sinks/r50.md) |
| 124 agents play the engines: strategies read trigger contents, insight income at its table rate (D53), spare insight keeps, the limits in the keep plan, upkeep's loss spent; per-seat keeps and gain sources in `games.jsonl`; 102's Phase A switches folded in | r49 | [124_agents_engines/r49-agents.md](results/124_agents_engines/r49-agents.md) |
| 122 runaway engines: holding limits (150 credits, 10 energy, chips, ambassadors); each pile's engine with its disproof row; Patience uncapped (the agents decline keeps) | r49 | [122_runaway_engines/r49.md](results/122_runaway_engines/r49.md) |
| 121 the loose threads: Unity overflow (Unity only), no insight giving, played-events pile and the reshuffle, Popular Uprising tier 1, no 1-credit amounts, nothing to act on never blocks, peaks per seat (the runaway check); 117's four cards get scenes | r48 | [121_loose_threads/r48.md](results/121_loose_threads/r48.md), [read.md](results/121_loose_threads/read.md), [carry.md](results/121_loose_threads/carry.md), [art.md](results/121_loose_threads/art.md), [pricing-notes.md](results/121_loose_threads/pricing-notes.md) |
| 119 cleanup: core tests pinned off live card numbers | r47 | [119_cleanup/pinned-tests.md](results/119_cleanup/pinned-tests.md) |
| 118 bug hunt (fixed findings; the open ones are in docs/qa-findings.md) | r46 → r47 | [118_bug_hunt_r46/fixed.md](results/118_bug_hunt_r46/fixed.md), [deferred.md](results/118_bug_hunt_r46/deferred.md), [reads.md](results/118_bug_hunt_r46/reads.md) |
| 117 lower income (18 / 16 / 14), big early and late cards, the strongest tier-2 cards to 1.5×; "credits left at game end" is the last upkeep's income in won games | r46 | [117_big_cards_and_income/r46.md](results/117_big_cards_and_income/r46.md), [cards.md](results/117_big_cards_and_income/cards.md) |
| 116 agent sweep (the strategist's ten parameters, coordinate search on fixed seeds, held-out confirm; `scripts/tune/agent_sweep.py`) | r45 | [116_agent_sweep/sweep.md](results/116_agent_sweep/sweep.md), [summary-r45.md](results/116_agent_sweep/summary-r45.md) |
| 114 art for borrowed scenes (11 cards off a cut card's or old name's scene: 2 reused, 9 drawn in batch 7) | r44 | [114_art_borrowed_scenes/art.md](results/114_art_borrowed_scenes/art.md) |
| 112 price correction (111's cuts to reach a band undone: the anchor margins are a maximum; strategies' card names) | r45 | [112_price_correction/r45.md](results/112_price_correction/r45.md) |
| 111 card pass (Matt's rulings and unit anchors, removal in reach, the draw loop) | r44 | [111_card_pass/r44.md](results/111_card_pass/r44.md) |
| 110 missing card art (a scene for each of 31 deck cards: 18 reused from cut cards, 13 drawn in batch 6; cut cards' scenes marked `was`) | r43 | [110_missing_card_art/art.md](results/110_missing_card_art/art.md) |
| 108 engines worth playing (engines pay up front and back within 2 rounds, needs 2-3, no antimatter on three, five not built over, the Capital engine, card throughput, tier-2 keeps 2 / 1, the opening deals 2 tier-2 cards; the engine probe and the r42 → r43 read) | r43 | [108_engines_worth_playing/r43.md](results/108_engines_worth_playing/r43.md), [cards.md](results/108_engines_worth_playing/cards.md), [probe.md](results/108_engines_worth_playing/probe.md) |
| 107 agents play the engines (why each engine went unplayed; keep-plan and value-code fixes, each on/off; play counts and the r42 read against r42's agents) | r42 | [107_agents_play_engines/engines.md](results/107_agents_play_engines/engines.md), [play-counts.md](results/107_agents_play_engines/play-counts.md) |
| 106 card overhaul (TM-style engines: income on programs, triggers on buildings, 154 cards; three op extensions; r41 → r42 read; board-credit and dearer-deck levers) | r42 | [106_card_overhaul/r42.md](results/106_card_overhaul/r42.md), [cards.md](results/106_card_overhaul/cards.md), [play-counts.md](results/106_card_overhaul/play-counts.md) |
| 105 the engine wording and the insight economy | r41 | [105_engine_rules/r41.md](results/105_engine_rules/r41.md), [play-counts.md](results/105_engine_rules/play-counts.md) |
| 104 agent speed, round 2 (`plan_gross=1` the default) | r40 | [104_agent_speed_2/speed.md](results/104_agent_speed_2/speed.md) |
| 103 agent profiling (where the agents' time goes; exact speed-ups; two approximations screened, not landed; where an item's time goes) | r40 | [103_agent_profiling/speed.md](results/103_agent_profiling/speed.md), [profile.md](results/103_agent_profiling/profile.md), [time-split.md](results/103_agent_profiling/time-split.md) |
| 102 agents and r40 (the keep plan nets prices, D23-D35 agent fixes; D25 reprices, Align cancels, deep-engine gates lowered; band read) | r40 | [102_agents_and_r40/r40.md](results/102_agents_and_r40/r40.md) |
| 101 rules split into `docs/rules/` (where each old default went; play unchanged) | r39 | [101_rules_split/defaults.md](results/101_rules_split/defaults.md) |
| 099 bug hunt (Rush Build's offer, measures gone quiet, the keep plan's gross value: not landed) | r39 | [099_bug_hunt_r38/fixed.md](results/099_bug_hunt_r38/fixed.md) |
| 098 rules r38 (troop Sway off, dark aligned cities, troop-removal cards repriced) | r38 | [098_rules_r38_troops_dark/r38.md](results/098_rules_r38_troops_dark/r38.md) |
| 096 rules r37 (converter city node, Popular Uprising one Action, Supergrid, dark cities) | r37 | [096_rules_r37_followups/r37.md](results/096_rules_r37_followups/r37.md) |
| 093 rules r36 (slots, Uprising's gate, energy, reach audit) | r36 | [093_rules_r36/r36.md](results/093_rules_r36/r36.md), [reach.md](results/093_rules_r36/reach.md) |
| 092 test-agent heuristics | r35 | [screen-092/agents.md](results/screen-092/agents.md) (`scripts/cards/slots_read.py`) |
| 090 balance pass | r35 | [screen-090/balance.md](results/screen-090/balance.md) (`scripts/cards/energy_read.py`) |
| 088 effect ledger | r34 | [effect-ledger/outliers.md](results/effect-ledger/outliers.md) (`scripts/cards/effect_ledger.py`) |
| 082 strategy agents | r34 | [screen-082/agents.md](results/screen-082/agents.md) |
| 080 liberating cities | r34 | [screen-080/liberation.md](results/screen-080/liberation.md) |
| 078 starting economy | r33 | [screen-078/economy.md](results/screen-078/economy.md) |
| 076 card review pass | r32 | [screen-076-cards/cards.md](results/screen-076-cards/cards.md), [play.md](results/screen-076-cards/play.md) |
| 075 the Researcher | r31 | [screen-075-researcher/researcher.md](results/screen-075-researcher/researcher.md) |
| 074 research cubes, corps | r30 | [screen-074-corps/corps.md](results/screen-074-corps/corps.md) |
| 070 threat sources, capstones | r29 | [gate-070/gate.md](results/gate-070/gate.md), [counters.md](results/gate-070/counters.md) |
| 069 planner speed-up | r28 | [screen-069-speed/speed.md](results/screen-069-speed/speed.md) |
| 068 Builder and Protector boards | r28 | [screen-068-boards/boards.md](results/screen-068-boards/boards.md) |
| 066 presence powers | r27 | [screen-066-presence/presence.md](results/screen-066-presence/presence.md) |
| 064 four-deal planner | r26 | [screen-064-deals/deals.md](results/screen-064-deals/deals.md) |
| 061 one-guess planner | r25 | [screen-061-one-guess/one-guess.md](results/screen-061-one-guess/one-guess.md) |
| 059 card design pass | r25 | [screen-059-cards/design.md](results/screen-059-cards/design.md), [wording.md](results/screen-059-cards/wording.md) |
| 055 first price pass | r24 | [screen-055-prices/prices.md](results/screen-055-prices/prices.md) |
| 054 planner deals | r23 | [screen-054-deals/deals.md](results/screen-054-deals/deals.md) |
| 053 calibration at d2 | r23 | [screen-053-calibration/calibration.md](results/screen-053-calibration/calibration.md) |
| 046 Science niche | r22 | [screen-046-niche/niche.md](results/screen-046-niche/niche.md) |
| 041 growth as a choice | r20 | [screen-041-growth/growth.md](results/screen-041-growth/growth.md) |
| 034 loss pressures, gate early-stopping | r19 | [screen-034-clocks/clocks.md](results/screen-034-clocks/clocks.md), [gate-034-earlystop/gate.md](results/gate-034-earlystop/gate.md) |
| 028 rollout planner | r17 | [screen-028-rollout/rollout.md](results/screen-028-rollout/rollout.md) |
| 027 endgame | r16 | [screen-027-endgame/endgame.md](results/screen-027-endgame/endgame.md) |
| 024 adversary | r15 | [screen-024-adversary/adversary.md](results/screen-024-adversary/adversary.md) |
| 023 player card set | r14 | [screen-023-cards/cards.md](results/screen-023-cards/cards.md) |
| 022 insight costs | r13 | [screen-022-insight/insight.md](results/screen-022-insight/insight.md), [pricing.md](results/screen-022-insight/pricing.md) |
| 021 card primitives | r12 | [screen-021-cards/cards.md](results/screen-021-cards/cards.md) |
| 019 troops | r10 | [screen-019-troops/troops.md](results/screen-019-troops/troops.md) |
| 018 calibration | r9 | [screen-018-calibration/calibration.md](results/screen-018-calibration/calibration.md) |
| 017 adversary projects | r8 | [screen-017-projects/projects.md](results/screen-017-projects/projects.md) |
| 016 presence | r7 | [screen-016-presence/presence.md](results/screen-016-presence/presence.md) |
| 014 board model | r6 | [screen-014-board/board.md](results/screen-014-board/board.md), the design as handed over: [design-014-board.md](results/screen-014-board/design-014-board.md) |
| 013 Alignment | r5 | [screen-013-alignment/alignment.md](results/screen-013-alignment/alignment.md) |
| 011 proposals, agent fixes | r4 | [proposals-011/proposals-011.md](results/proposals-011/proposals-011.md), `screen-011-*/report.md` |
| 009 engine payback sweep | r3 | [payback-009/payback-009.md](results/payback-009/payback-009.md), [payback-009-records](results/payback-009-records/README.md) |
| 008 engine agent | r3 | [engine-agent-008/engine-008.md](results/engine-agent-008/engine-008.md) |
| 007 tuning run | r2 | [tuning-007/change-log.md](results/tuning-007/change-log.md), `tune-before/`, `tune-after/`, `tune-agents-007/` |
| v0 the first ruleset | r1 | [ladder-v0/ruleset-v0.md](results/ladder-v0/ruleset-v0.md) (the v0 design draft as handed over), [ladder-v0/report.md](results/ladder-v0/report.md) |

Other committed results without an item write-up: `cards-t2cut/`, `engine-sweep-008/`,
`variants-modules/`, `variants-spread/` (each holds its `report.md` where it has one).
