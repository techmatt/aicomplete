# 214: anchor check, tier-2 cards (set B, 31 cards, r94)

Pay = credits + 4 × (keep + play insight) + am 18, en 4.5, rc 6, ch 6. Keep 2 = 8 + 4 × play insight; keep 3 adds 4.
Get at the anchors: agent or troop removed 5, energy 4.5, insight 4, draw 5, ambassador 8, chip 6, antimatter 18,
research cube 6, +1 credit income 4, +1 insight income 12, +1 energy income 14, liberate 45, botnet removed 16.
No anchor, so from `values.json`: Sway toward Unity +4, toward Discord -4; firewall 10; claim a station 20; an
ambassador gathered or moved 2. **Convert 1 agent = 10.6**: removed (5) plus an ambassador of yours where it stood,
at values.json's 0.7 of an ambassador (0.7 × 8). Engines: a committed seat over 3.5 rounds, 3 tags and 1 card of the
tag a round. A building's own play fires its trigger once. Building tags 1.5. "Up to N" is counted at the printed N.

| card id | kind, needs | printed cost (cr, am, other; play ins) | pay at keep 2 (arithmetic) | get (arithmetic) | ratio keep 2 | ratio keep 3 | verdict |
|---|---|---|---|---|---|---|---|
| hostile_buyout | event | 22 cr, 1 am; 0 | 22+18+8 = 48 | liberate 45 + 3 amb 24 = 69 | 1.44 | 1.33 | in band |
| forensic_audit | event | 27 cr, 4 rc; 1 | 27+24+12 = 63 | liberate 45 + draw 2 10 = 55 | 0.87 | 0.82 | under 1.0 |
| root_access | event, 2 cyber | 18 cr, 1 am; 0 | 18+18+8 = 44 | liberate 45 + botnet 16 = 61 | 1.39 | 1.27 | in band |
| peacekeeping_mission | event, 2 security | 20 cr, 1 am; 0 | 20+18+8 = 46 | liberate 45 + 3 troops 15 = 60 | 1.30 | 1.20 | in band |
| interdisciplinary_institute | building, 2 science | 12 cr; 2 | 12+16 = 28 | draw 3 15 + own play 2 ins 8 + 3 ins/rd 12 × 3.5 = 42 + tags 1.5 = 66.5 | 2.38 | 2.08 | in band |
| capital_projects_office | building | 16 cr; 1 | 16+12 = 28 | draw 4 20 + own play 10 + 0.875 fires × 10 × 3.5 = 30.6 + tags 1.5 = 62.1 | 2.22 | 1.94 | in band |
| conglomerate_headquarters | building, 2 industry | 18 cr; 0 | 18+8 = 26 | 2 Capital + own tag fills 3: 12; 3 tags/rd = 12 × 3.5 = 42; tags 1.5 = 55.5 | 2.13 | 1.85 | in band |
| zero_day_auction | program, 5 cyber | 14 cr; 2 | 14+16 = 30 | 2 am 36 + draw 6 30 = 66 | 2.20 | 1.94 | in band |
| command_and_control_grid | building, 2 cyber | 18 cr; 2 | 18+16 = 34 | draw 3 15 + own play 5 + 15/rd × 3.5 = 52.5 + tags 1.5 = 74 | 2.18 | 1.95 | in band |
| smart_grid_backbone | building, 3 power | 25 cr; 2 | 25+16 = 41 | 2 en 9 + own play 4.5 + 3 en/rd 13.5 × 3.5 = 47.25 + tags 1.5 = 62.25 | 1.52 | 1.38 | in band |
| homeland_security_act | program, 3 security | 18 cr; 2 | 18+16 = 34 | +2 en income 28 + 4 Security tags × 4 = 16 = 44 | 1.29 | 1.16 | in band |
| fusion_research_campus | building, 3 power | 16 cr; 2 | 16+16 = 32 | draw 3 15 + own play draw 2 10 + 10/rd × 3.5 = 35 + tags 1.5 = 61.5 | 1.92 | 1.71 | in band |
| decapitation_strike | event, 2 security | 4 cr, 1 am; 0 | 4+18+8 = 30 | 3 troops 15 + botnet 16 = 31 | 1.03 | 0.91 | in at 2 only |
| citywide_microgrid | building | 8 cr, 8 en; 1 | 8+36+12 = 56 | liberate 45 + Action sway 2 8 × 3.5 = 28 + tags 1.5 = 74.5 | 1.33 | 1.24 | in band |
| exascale_training_cluster | building | 15 cr, 5 en; 2 | 15+22.5+16 = 53.5 | +3 ins income 36 + 6 cubes 36 + tags 1.5 = 73.5 | 1.37 | 1.28 | in band |
| microgrid_lab_compact | building | 18 cr; 0 | 18+8 = 26 | Action: station 20 less 2 en 9 = 11 × 3 uses (stations run out) = 33 + tags 1.5 = 34.5 | 1.33 | 1.15 | in band |
| mass_defection | event | 9 cr; 0 | 9+8 = 17 | convert 5 × 10.6 = 53 + sway 2 8 = 61 | 3.59 | 2.90 | over 2.5 |
| embassy_of_exiles | building | 14 cr; 0 | 14+8 = 22 | 3 amb 24 + Action convert 2 21.2 × 3.5 = 74.2 + tags 1.5 = 99.7 | 4.53 | 3.83 | over 2.5 |
| double_agents | event | 8 cr; 0 | 8+8 = 16 | convert 3 31.8 + 2 troops 10 = 41.8 | 2.61 | 2.09 | over 2.5 (keep 2) |
| kettle_operation | event, 2 security | 18 cr; 2 | 18+16 = 34 | 6 removals 30 − Discord 1 4 = 26 | 0.76 | 0.68 | under 1.0 |
| lure_network | building | 12 cr; 1 | 12+12 = 24 | draw 2 10 + Action 3 agents 15 × 3.5 = 52.5 + tags 1.5 = 64 | 2.67 | 2.29 | over 2.5 (keep 2) |
| firewall_array | building, 2 cyber | 15 cr; 1 | 15+12 = 27 | draw 2 10 + Action firewall 10 × 3.5 = 35 + tags 1.5 = 46.5 | 1.72 | 1.50 | in band |
| air_gap | event | 6 cr; 0 | 6+8 = 14 | firewall 10 + 3 agents 15 = 25 | 1.79 | 1.39 | in band |
| rally_the_allies | event | 12 cr; 0 | 12+8 = 20 | 5 amb 40 + gather 3 × 2 = 6 + sway 2 8 = 54 | 2.70 | 2.25 | over 2.5 (keep 2) |
| surgical_strike | event | 14 cr; 0 | 14+8 = 22 | better of 4 agents 20 / 3 troops 15 = 20 − Discord 1 4 = 16 | 0.73 | 0.62 | under 1.0 |
| open_source_uprising | program | 9 cr; 1 | 9+12 = 21 | convert 4 42.4 + draw 2 10 + sway 1 4 = 56.4 | 2.69 | 2.26 | over 2.5 (keep 2) |
| penning_trap_array | building, 1 science | 10 cr; 1 | 10+12 = 22 | 1 am 18 + Action (2 am 36 − cube 6 − en 4.5 = 25.5) × 3.5 = 89.25 + tags 1.5 = 108.75 | 4.94 | 4.18 | over 2.5 |
| antiproton_foundry | building, 1 industry | 10 cr; 1 | 10+12 = 22 | 1 am 18 + Action (36 − chip 6 − en 4.5 = 25.5) × 3.5 = 89.25 + tags 1.5 = 108.75 | 4.94 | 4.18 | over 2.5 |
| collider_consortium | program (help) | 8 cr, 1 am; 1 | 8+18+12 = 38 | 8 cubes 48 + another player 3 cubes 18 (team) = 66 | 1.74 | 1.57 | in band |
| catalyzed_fab_run | event (help) | 2 cr, 1 am; 0 | 2+18+8 = 28 | 6 chips 36 + another player 2 chips 12 = 48 | 1.71 | 1.50 | in band |
| antimatter_accord | program | 8 cr, 1 am; 1 | 8+18+12 = 38 | 6 amb 48 + sway 4 16 = 64 | 1.68 | 1.52 | in band |

## Proposals (credit or antimatter price only; target about 1.1-1.3 at keep 3)

- **forensic_audit** 27 → 6 cr: keep 3 pay 46, 1.20; keep 2 pay 42, 1.31. Its 4 cubes (24 at the anchor, 12 at r45's
  3 a cube) are what dropped it; the stale note priced them at 3.
- **decapitation_strike** credits alone can't reach 1.1: 4 → 0 cr reads 1.03 at keep 3, 1.19 at keep 2 (the smallest
  in-band fix). For 1.2 at keep 3: 0 am, 14 cr (keep 3 pay 26, 1.19; keep 2 pay 22, 1.41), but then it is no longer a major.
- **kettle_operation** 18 → 2 cr: keep 3 pay 22, 1.18; keep 2 pay 18, 1.44. The old note counted the Discord Sway as a gain.
- **surgical_strike** 14 → 2 cr: keep 3 pay 14, 1.14; keep 2 pay 10, 1.60 (1 cr would be closer but may not print).
- **mass_defection** 9 → 37 cr: keep 3 pay 49, 1.24; keep 2 pay 45, 1.36. Smallest in-band fix: 17 cr (2.44 / 2.10).
- **embassy_of_exiles** 1.1-1.3 at keep 3 needs about 70 cr, over the ~50 ceiling. At 50 cr: keep 3 pay 62, 1.61; keep 2
  pay 58, 1.72. Smallest in-band fix: 32 cr (2.49 / 2.27). Flag: the Action's 3.5 rounds of convert 2 is the excess.
- **double_agents** 8 → 23 cr: keep 3 pay 35, 1.19; keep 2 pay 31, 1.35. Smallest in-band fix: 9 cr (2.46 / 1.99).
- **lure_network** 12 → 37 cr: keep 3 pay 53, 1.21; keep 2 pay 49, 1.31. Smallest in-band fix: 14 cr (2.46 / 2.13).
- **rally_the_allies** 12 → 33 cr: keep 3 pay 45, 1.20; keep 2 pay 41, 1.32. Smallest in-band fix: 14 cr (2.45 / 2.08).
- **open_source_uprising** 9 → 31 cr: keep 3 pay 47, 1.20; keep 2 pay 43, 1.31. Smallest in-band fix: 11 cr (2.45 / 2.09).
- **penning_trap_array**, **antiproton_foundry** (same numbers): 1.2 at keep 3 needs about 75 cr. Within the ~50 cap:
  50 cr and 1 am to build (keep 3 pay 84, 1.29; keep 2 pay 80, 1.36). Smallest in-band fix: 32 cr (2.47 / 2.27).
  Flag: the r90 antimatter anchor (18) on a once-a-round 10.5-cost Action makes 25.5 a use; if a seat only spends
  about 2 antimatter a game (2 uses), get is 70.5 (3.20 / 2.71) and 43 cr reads 1.19 / 1.28.

The convert cards hang on the convert value: at removal only (5 an agent), mass_defection reads 1.94 / 1.57,
open_source_uprising 1.62 / 1.36, double_agents 1.56 / 1.25 and embassy_of_exiles 2.75 / 2.33 (in band except Embassy at keep 2).
