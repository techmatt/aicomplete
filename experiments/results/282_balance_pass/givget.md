# 282 give / get: cards, converters, projects

Anchors (card-guidelines, r128 credits): insight 2.67, card drawn 3.33, energy 3, chip 4, antimatter 12, agent or troop
removed 3.33, ambassador gained 5.33, +1 credit income 4, +1 chip income 13, a compute 1.33, research cube 4; a botnet
10, a city liberated 30, Fallout lifted 8 and a node 5.33 (values.json, no anchor). Keep: 1 insight tier 1 (2.67), 3 tier 2 (8).
**Pay** = credits + 2.67 x (keep + play insight) + other costs. **Get**: an engine at a committed seat's rate (1 card of
its tag a round), shown at 3 and 4 rounds, `mid` at 3.5. `income_per_tag` pays at the tag count frozen when played
(card-grammar.md), taken as 4-5 (needs 3 plus itself). **Net** = values.json's reading for the seat: the ledger's
reasoned value less printed (credits, insight 1.67, other resources at table rates) less the keep (1.67 / 4.33), after
both overlays. `FLAG` marks a net at or under 0: the table row to fix (never the price). `+N` is the override (credits)
that would put the table's get at 1.2 x its give.

## Repriced (prices.overlay)

| Card | T | Cost | Pay | Get | Get / pay | Net |
|---|---|---|---|---|---|---|
| foundation_model_campus | 2 | 3 -> 11 | 11 + 6 (2 en) + 13.33 = 30.33 | 2 x (4-5 tags) x (3-4 rds) = 24-40, + draw 2 6.67 = 30.7-46.7, mid 38.7 | 1.01-1.54, mid 1.28 | 11.7 |
| vertical_monopoly | 2 | 3 -> 11 | 11 + 13.33 = 24.33 | 2 x (4-5) x (3-4) = 24-40, mid 31.5 | 0.99-1.64, mid 1.29 | 12.0 |
| orbital_mass_driver | 2 | 2 -> 15 | 15 + 9 (3 en) + 4 (chip) + 13.33 = 41.33 | 9 cr income 27-36 + 2 chip income 19.5-26 = 46.5-62 | 1.13-1.50 (was 2.19 at 2) | 2.5 |
| orbital_embassy | 2 | 2 -> 5 | 5 + 13.33 = 18.33, + the Diplomacy building it buries | draw 2 6.67 + 1 ambassador a Diplomacy play 5.33 x (3-4) = 22.7-28 | 1.24-1.53 before the burial | -3.3 FLAG +6 |
| zero_day_auction | 2 | 2 -> 4 | 4 + 13.33 = 17.33 | 2 antimatter 24 + draw 3 10 = 34 | 1.96 (needs 5 Cyber: "about half", ruling 16) | 21.3 |
| dark_fiber_collider | 2 | 2 -> 7 | 7 + 10.67 = 17.67 | Action 2 insight -> 1 am: 12 - 5.33 = 6.67 x (3-4) = 20-26.7 | 1.13-1.51, mid 1.32 | 3.2 |
| lure_network | 2 | 3 -> 12 | 12 + 10.67 = 22.67 | Action up to 3 agents: 2.5-3 x 3.33 x (3-4) = 25-40 | 1.10-1.76, mid 1.32 | -11.9 FLAG +15.5 |
| open_source_uprising | 2 | 2 -> 10 | 10 + 10.67 = 20.67 | convert 3-4 agents at 7.07 (removal 3.33 + 0.7 x an ambassador 5.33) + Sway Unity 2.67 = 23.9-31 | 1.16-1.50, mid 1.33 | -3.0 FLAG +6 |
| kettle_operation | 2 | insight 2 -> 1 (cost 2) | 2 + 10.67 = 12.67 | up to 6 agents or troops, 4-6 near at 3.33, less Sway Discord 2.67 = 10.7-17.3 | 0.84-1.37, mid 1.10 (was 0.91 at insight 2) | 20.1 |
| command_and_control_grid | 2 | 13 -> 4; draw 2 -> 1 (Matt) | 4 + 13.33 = 17.33 | Action draw 1 (no energy, effects.overlay) 3.33 x (3-4) = 10-13.3; with Reuse 2 a round 20-26.7 | 0.58-0.77 alone; 1.15-1.54 with Reuse | -6.3 FLAG +8.7 |
| simulation_cluster | 1 | 2 -> 4 | 4 + 8 = 12 | Reuse one Action a round, 3.33-6.67 x (3-4) = 10-26.7, mid 17.5 | 0.83-2.2, mid 1.46 (was 1.75) | 1.3 |
| patent_library | 1 | 2 -> 4 | 4 + 5.33 = 9.33 | Action 2 cr -> 2 insight: 3.33 x (3-4) = 10-13.3 | 1.07-1.43, mid 1.25 (was 1.59) | -2.9 FLAG +4.4 |
| bug_bounty_board | 1 | 2 -> 5 | 5 + 5.33 = 10.33 | Action 3 cr -> drain 5 (6.67): 3.67 x (3-4) = 11-14.7 | 1.07-1.42, mid 1.24 (was 1.75) | -5.8 FLAG +7.5 |
| field_sensor_grid | 1 | 2 -> 7 | 7 + 8 = 15 | lift 1.5-3 Fallout at 8 = 12-24, + draw 1 3.33 = 15.3-27.3 | 1.02-1.82, mid 1.42 (was 2.1) | 6.2 |
| joint_task_force | 1 | 2 -> 7 | 7 + 5.33 = 12.33 | 2 agents (6.67) a teammate's Security play, 0.5-1 a round x 3.5 = 11.7-23.3 | 0.95-1.89, mid 1.42 (was 2.4) | -6.1 FLAG +8.1 |

## Effect changes (effects.overlay; each Matt's call: too weak at the 2-credit floor)

| Card | T | Change | Pay | Get after (before) | Get / pay after (before) | Net after |
|---|---|---|---|---|---|---|
| command_and_control_grid | 2 | Action's 1 energy dropped | above | per use 3.33 (0.33: break-even, ruling's "no break-even conversions") | above (about 0.1) | above |
| agency_directorate | 2 | Security play adds 2 (1); at 4 gain 4 insight (2) | 2 + 13.33 = 15.33, + the Security building it buries | 2 fires in 3-4 rds x 10.67 = 21.3 (5.33-8) | 1.39 (0.35-0.52) | -1.2 FLAG +3.2 |
| interdisciplinary_institute | 2 | 2 insight a Science tag (1) | 15.33 | 2 x 2.67 x 1 tag x (3-4) = 16-21.3 (8-10.7) | 1.04-1.39 (0.52-0.70) | -4.9 FLAG +6.9 |
| capital_projects_office | 2 | 4 credits an Industry play (2) | 2 + 10.67 = 12.67 | 4 x (3-4) = 12-16 (6-8) | 0.95-1.26 (0.47-0.63); payback 2.2 rounds, no when-built gain | -2.8 FLAG +4.4 |
| fusion_research_campus | 2 | 2 energy a Power play (1) | 15.33 | 2 x 3 x (3-4) = 18-24 (9-12) | 1.17-1.57 (0.59-0.78) | -1.9 FLAG +3.9 |
| orbital_observatory_array | 2 | 1 energy -> 3 insight (2) | 15.33 | (8 - 3) x (3-4) = 15-20 (7-9.3) | 0.98-1.30 (0.46-0.61) | -5.2 FLAG +7.1 |
| lights_out_assembly | 1 | 3 credits an Industry play (1: a printed 1-credit gain); insight 2 -> 1 | 2 + 3 (en) + 5.33 = 10.33 | 3 x (3-4) = 9-12 (3-4) | 0.87-1.16 (0.23-0.31) | -1.9 FLAG +3.6 |
| parts_exchange | 1 | 1 chip -> 5 credits (3: a loss, chip 4) | 2 + 5.33 = 7.33 | (5 - 4) x (3-4) = 3-4 (a loss) | 0.41-0.55: still weak; the converter now gains | 1.9 |

## Left at their price (tier 2 at 2-3 credits)

| Card | T | Cost | Pay | Get | Get / pay | Net |
|---|---|---|---|---|---|---|
| recursive_self_improvement | 2 | 3 | 3 + 12 (am) + 13.33 = 28.33 | +3 insight income 24 + draw 3 10 = 34 | 1.20 | -1.2 FLAG +5.7 |
| singularity_exploit | 2 | 3 | 3 + 12 + 10.67 = 25.67 | botnet 10 + project discarded ~16 = 26 | 1.01 | -9.7 FLAG +13.9 |
| positron_storage_ring | 2 | 2 | 2 + 6 (en) + 13.33 = 21.33 | Action 2 en -> 1 am, 6 x (3-4) = 18-24 | 0.84-1.13 (floor: can't go cheaper) | -4.5 FLAG +7.6 |
| martial_cordon | 2 | 2 | 2 + 10.67 = 12.67 | 4-6 agents at 3.33 less Sway Discord 2.67 = 10.7-17.3 | 0.84-1.37 | -3.3 FLAG +4.9 |
| forensic_audit | 2 | 2 | 2 + 16 (4 cubes) + 10.67 = 28.67 | a city liberated 30 + a claim near ~4 = 30-34 | 1.05-1.19 (unplayed: the 4 cubes, not the price) | 18.0 |
| conglomerate_headquarters | 2 | 2 | 2 + 8 = 10 | 8 credits at 3 Capital: 1-2 fires = 8-16 | 0.8-1.6, mid 1.2 | 1.3 |
| surgical_strike | 2 | 2 | 2 + 8 = 10 | 3-4 agents on the target less Sway 2.67 = 7.3-10.7 | 0.73-1.07 | 13.0 |
| catalyzed_fab_run | 2 | 2 | 2 + 12 + 8 = 22 | 6 chips 24 + a gift of 2 at 0.75 = 30 | 1.36 | 1.7 |

Tier 1 from 271's list left at 2 (none over 1.5): breakthrough_leak 0.91, zero_knowledge_exploit 0.91, dataset_scraper
0.82, predictive_modeling ~0.9, red_team_audit / force_protection_order ~1.0-1.4 (jam 6), recruit_operatives ~0.8,
relay_mast_rollout 0.95 (3 nodes at 5.33 less a track step's energy each), standardized_tooling 1.23-1.64 (mid 1.43; the
discount bites harder at `discount_floor` 0: watch), security_contractor_hq ~1.2 (troop removals unmeasured),
reconciliation_office 0.7, breach_and_clear 0.45 (weak: 1 troop). Not in the deck, skipped: quick_reaction_force (the
same Reuse as Simulation Cluster: 4 if it returns), shadow_broker_feed (7 if it returns).

## Converters (map.json, prices.overlay)

| Converter | Old -> new | Give | Get | Get / give | values.json |
|---|---|---|---|---|---|
| Tidal Grid cubes | 3 cubes -> 5 energy => 2 cubes -> 4 energy | 8 (was 12) | 12 (15) | 1.5 (1.25) | 4 -> 12 (6 -> 15) |
| Tidal Grid chips | 2 chips -> 3 energy => 1 chip -> 2 energy | 4 (8) | 6 (9) | 1.5 (1.13) | 2.67 -> 6 (5.33 -> 9) |
| Polar chips | 1 chip -> 1 card + 1 credit => + 2 credits | 4 | 5.33 (4.33) | 1.33 (1.08) | 2.67 -> 4.67 |

Tidal's loop with Mint (3 credits -> 1 chip): 3 credits -> 2 energy once a round, energy at 1.5 credits against
Buy energy's 3: a gain a player sees, capped by once a round. Polar's chip converter was already 41% of chip spends
at 258's read (max 50%): the +1 credit may push it over. Mountain Fusion Works (1 cube -> 1 chip) is break-even at
the anchors (4 -> 4), left for Matt. Delta Seawall's (2 chips -> 2 Sway + 1 card, 8 -> 8.67) passes.

## Alignment (content/alignment.json, by hand: apply_overlay refuses the file)

- **New Deal**: "1 credit and N Ambassadors" per site -> **2 credits** and N (I-III; caps 2/3/4 kept). Folded up,
  not to 0: with no credit, strength I (1 Ambassador a site) would be a weaker Safe Corridors (2 a site).
- **Rebuilding Fund**: the same, 1 -> **2 credits** a corrupted city (same shape as New Deal).
- **Propaganda**: the credit loss is the whole effect, so it can't fold to 0: **lose 2 credits per corrupted city, at
  most 1 / 2 / 3** (totals 2 / 4 / 6 a player, were 2 / 3 / 4). Matt's call: keeping caps 2/3/4 doubles it (4/6/8).
  Each tier's `text` and `per_site` `amount` (and Propaganda's `max`) change together.

## Projects (drafts/adversary/projects.json `cost`, a country; then load_adversary.py)

A capture at the deck's own prices: a station corrupted outright about **16** (Station Coup 24, less its track line's
5 (259) and Fallout's ~3); a factory **13** (Asset Seizure). A city Surge has no project to anchor (card anchors put a
city at 3x a station: 30 against 10): **16** for Surge 4 (corrupts at level 0 with 1 agent there), **14** for Surge 3
(needs 2 at level 0; certain from level 1, corrupt_at 3). Other lines: Sway 5 Discord 7 (Disinformation 11 for 8),
track 2 a country 5, Fallout 4, Infiltrate again ~5.

| Project | Lines | Old -> new |
|---|---|---|
| lab_seizure | station capture 16 + track 5 | 13 -> 21 |
| coastal_raids | city Surge 3, 14 | 9 -> 14 |
| plant_seizure | factory capture 13 | 9 -> 13 |
| mob_rule | city Surge 3, 14 + Sway 5, 7 | 14 -> 21 |
| hostile_takeover | factory capture 13 + 1 agent | 15 (left: already over) |
| infiltration_drive | city Surge 4, 16 + Fallout 4 | 22 (left: already over) |
| martial_law | station 16 + factory 13 | 15 -> 29 |
| recruitment_drive | city Surge 4, 16 + station 16 | 20 -> 32 |
| grand_offensive | Infiltrate again 5 + city Surge 3, 14 + factory 13 | 22 -> 32 |

Martial Law, Recruitment Drive and Grand Offensive land above the deck's 7-24 (two captures each). Matt's call: ship
29-32, or hold them at 24 and cut one Surge line each (a cost of 24 with two captures underprices again).
