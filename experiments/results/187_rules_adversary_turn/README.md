# 187: friction round 3, the adversary side (r80)

Built on main 950a048 (r79, 185). `RULES_REVISION` 79 → 80, bumped from the constant (the prompt also said r80).
Matt's rulings of 2026-11-11 on `docs/rulebook-friction.md`'s adversary rows, all candidates (Matt will playtest).

## What changed (engine, content, tests)

| Ruling | Built | Test |
|---|---|---|
| #41 four beats | `adversary::BEATS` (Card, Corrupt, March, Income; one constant, the round phases each runs). Play change: ongoing projects act in the Income beat after compute (they acted in the build between Muster and Corrupt). | `the_four_beats_cover_the_adversary_turn_in_order`, `an_ongoing_project_acts_in_the_income_beat` |
| #42 one aftermath | A node hex that corrupts takes land corruption (no marker); markers no longer count as agents (`plan::held`, Corrupt lines). | `a_corrupted_node_hex_takes_land_corruption_not_a_marker`, `a_marker_no_longer_counts_toward_corrupting_a_city` |
| #43 round-robin Rally | `plan::rally_plan`: one agent at a time in hex order, skipping firewalled (and no-Rally) sites; all skipped: the pool goes back to the supply. | `rally_deals_round_robin_in_hex_order_skipping_firewalls` |
| #46 location tokens | `near::place_tokens`: most agents on the site's own hex, tie lowest number, printed order, no reorder (a token with no site left stays off). | `a_location_token_goes_where_most_agents_stand_on_the_hex` |
| #48 Protector's landing | `wall_pressure` is a normal Muster: lowest-numbered factory (anyone's), else city. Board and Wall text reworded. | `the_protectors_landing_is_a_normal_muster` |
| #31 event lines | `content::EVENT_LINE_KINDS` (10 kinds, 7 "other lines", each with its icon named); `event_rule_break` refuses any other. Faces print no icon row yet (no art change). | `event_lines_come_from_the_fixed_list` |
| #49 leftmost compute | `compute_target`: the leftmost project; a resolved or discarded one's right neighbors slide left, the right slot refills. Eval's waiting-compute read follows. | `compute_fills_the_leftmost_project_and_the_rest_slide_left` |
| #50 tip drones | Once a country with an unblocked neutral factory (no agent, no player ambassador on it): 1 drone a base; an ambassador on the base stops it (used up). | `what_stops_a_tip_troop` |
| #52 war | A strike takes only the factory; `board.war_reach` removed (config, guide forecast). | `at_war_units_cancel_and_the_surplus_destroys_rival_buildings` |
| #53 prohibitions | `no_rally` / `no_muster` / `no_corrupt` hex tokens placed where the condition holds at resolution; they come off with the firewalls. | `a_corrupt_prohibition_stops_a_covered_city`, `a_rally_prohibition_keeps_the_share_in_the_supply` |
| D79 (a) | The adversary-factory fallback skips firewalled factories. | `the_adversary_factory_fallback_skips_a_firewalled_factory` |
| D86 (a) | Docs only (the engine already did it). | `a_reshuffled_deck_with_no_fitting_card_deals_the_next_anyway` |
| 185's audit | Corrupt X lines and natural corruption go through `adversary::guard_holds` (tallies `guard_held.line`, `.natural`). | existing |

## Ladder (fresh seeds, paired)

Strategist 43183-43326 (144 a pair, 432 a level), greedy 43183-43230 (48 a pair). r80 farmed (`farm-seeds`, main
box, 368 s and 123 s), r79 natively from `out/r79` (the base binary and its configs). Specs `experiments/farm/f187-*`.
Tables: `ladder-strategist.md`, `ladder-greedy.md`, `pairs-*.md`, `ends.txt` (`by_pair.py`, `ends.py` here).

| d | r80 strategist | r79 | gap ± SE | band | r80 greedy | r79 greedy |
|---|---|---|---|---|---|---|
| d0 | 89.4 | 93.3 | −3.9 ± 1.9 | 90-100 **unmet by 0.6** | 88.2 | 93.8 |
| d1 | 72.7 | 74.1 | −1.4 ± 2.6 | 70-80 met | 79.9 | 80.6 |
| d2 | 49.3 | 55.3 | −6.0 ± 2.8 | 55-65 **unmet by 5.7** | 59.7 | 52.1 |
| d3 | 41.4 | 43.3 | −1.9 ± 2.7 | 40-50 met | 53.5 | 49.3 |
| d4 | 37.5 | 43.8 | −6.2 ± 2.6 | - | 52.8 | 51.4 |

Pairs carrying the misses: d0 bp −6.9 ± 3.2 and pr −6.2 ± 3.6 (br +1.4): the Protector pairs; d2 br −11.1 ± 4.9
(bp, pr −3.5). Ends: d2 Discord-deck losses 186 → 208 of 432, Discord tips 5.11 → 5.32 a game; d0 deck-empty
losses 29 → 46. **Attribution (indicative: dev seeds 36800-36831, 96 games a cell, levers that switch a mechanic
off, not r79 put back; `s187-attrib.json`, `attrib-screen.md`):** land corruption's Discord drip off
(`blight_discord=0`) +3.1 ± 2.8 at d0, +6.2 ± 3.2 at d2: #42 puts land corruption on every corrupted node hex
(1.6-2.1 a game). The Protector's Muster off (`wall_troops=0`) +5.2 ± 3.4 at d0, +1.0 at d2: #48 lands it on players'
factories. Nine rulings moved in one revision, so the ladder alone can't split the rest. No dial moved (the prompt).

Project measures: revealed a game 32 → 18 at d2, reshuffles 3.3 → 0.1: compute on the leftmost stops the cheapest
project churning the deck; resolved per game about the same (14.9 against 15.8).

## Notes

- Loop smoke (16 seeds, use.json): no timeouts, most plays of one card 4.
- `docs/harms.md`'s node-destruction balance check failed once war stopped taking nodes (events' Corrupt lines 15
  of 28); its slack is 2 marks now (`harms.rs`), flagged for Matt.
- The rulebook's Part 1: 7,969 words against r79's 8,060 (`wc_part1.py`, regex `\b[\w'-]+\b`, Sources lines out).
