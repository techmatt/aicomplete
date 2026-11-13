# 214: anchor check, tier-2 cards (set A, 31 cards, r94)

Conventions as anchors_b.md: pay = credits + 4 × (keep + play insight) + am 18, en 4.5, ch 6. Keep 2 = 8 + 4 × play
insight; keep 3 adds 4. Get at the anchors (agent or troop 5, energy 4.5, insight 4, draw 5, ambassador 8, chip 6,
antimatter 18, +1 credit income 4, +1 insight income 12, +1 energy income 14, liberate 45, botnet 16, compute 2); Sway
toward Unity +4, toward Discord -4; an ambassador gathered 2; a node 8. Engines: a committed seat over 3.5 rounds, 3 tags
and 1 card of the tag a round; a building's own play fires its trigger once; building tags 1.5; "up to N" at N.
Set A's extra assumptions: a building's per-tag income counts needs + itself at play, +1 a round (needs 3: 4+5+6+3.5 =
18.5 tag-rounds; needs 2: 15); a program's per-tag income is fixed at needs + itself. A project discarded 24 (card-pricing,
12 compute at 2); a corruption marker cleared 10 (`clear_markers`: 2 Unity + the hex); a station freed 15
(`cleanse_station`); an Operative toward a "draw 3 at 4" 3.75. A standing counter (6 less) counters each threat of its
type free: 2 such threats over 3.5 rounds, each at its 12-credit counter. "Over X" (covering a building) is not priced.

| card id | kind, needs | printed cost (cr, am, other; play ins) | pay at keep 2 (arithmetic) | get (arithmetic) | ratio keep 2 | ratio keep 3 | verdict |
|---|---|---|---|---|---|---|---|
| foundation_model_campus | building, 3 science | 13 cr, 2 en; 2 | 13+9+16 = 38 | 3 cr × 18.5 tag-rounds 55.5 + draw 4 20 + tags 1.5 = 77 | 2.03 | 1.83 | in band |
| orbital_observatory_array | building, 1 cyber | 10 cr; 2 | 10+16 = 26 | Action (draw 3 15 − 2 cr) 13 × 3.5 = 45.5 + tags 1.5 = 47 | 1.81 | 1.57 | in band |
| recursive_self_improvement | program, 2 science | 5 cr, 1 am; 2 | 5+18+16 = 39 | +3 ins income 36 + draw 3 15 = 51 | 1.31 | 1.19 | in band |
| open_weights_release | program (help) | 16 cr; 1 | 16+12 = 28 | 4 markers 40 + another player 6 ins 24 = 64 | 2.29 | 2.00 | in band |
| collateralized_grid | program | 15 cr; 0 | 15+8 = 23 | +2 en income 28 + 3 Capital → 3 en 13.5 less the tokens' 9 (feed_capital 3) = 4.5 → 32.5 | 1.41 | 1.20 | in band |
| arcology_foundry | program, 3 industry | 11 cr, 2 en; 1 | 11+9+12 = 32 | +7 cr income 28 + +1 chip income 19 = 47 | 1.47 | 1.31 | in band |
| orbital_mass_driver | building, 3 industry | 15 cr, 3 en, 1 ch; 2 | 15+13.5+6+16 = 50.5 | (13 cr + 2 ch 12) 25 × 3.5 = 87.5 + draw 3 15 + tags 1.5 = 104 | 2.06 | 1.91 | in band |
| botnet_takedown | event, 1 cyber | 11 cr; 1 | 11+12 = 23 | botnet 16 + 6 compute 12 = 28 | 1.22 | 1.04 | in band |
| killswitch_directive | event | 16 cr; 1 | 16+12 = 28 | project 24 + 4 agents 20 = 44 | 1.57 | 1.38 | in band |
| singularity_exploit | program, 3 cyber | 8 cr, 1 am; 1 | 8+18+12 = 38 | botnet 16 + project 24 = 40 | 1.05 | 0.95 | in at 2 only |
| hyperscale_botnet | building, 2 cyber | 18 cr; 1 | 18+12 = 30 | 4 ins 16 × 3.5 = 56 + draw 3 15 + tags 1.5 = 72.5 | 2.42 | 2.13 | in band |
| positron_storage_ring | building, 2 power | 9 cr, 2 en; 2 | 9+9+16 = 34 | 1 am 18 × 3.5 = 63 + 1 Power card/rd 2 en 9 × 3.5 = 31.5 + draw 2 10 + tags 1.5 = 106 | 3.12 | 2.79 | over 2.5 |
| dyson_swarm_tranche | program, 2 power | 12 cr, 1 am; 1 | 12+18+12 = 42 | 10 en 45 + +2 en income 28 = 73 | 1.74 | 1.59 | in band |
| continental_supergrid | program, 3 power | 20 cr; 2 | 20+16 = 36 | +2 en income 28 + 4 Power tags × 4 = 16 = 44 | 1.22 | 1.10 | in band |
| beamed_power_constellation | program, 3 power | 14 cr; 1 | 14+12 = 26 | 3 nodes 24 + 1 en × 8 nodes (5 held + 3) 36 = 60 | 2.31 | 2.00 | in band |
| coalition_charter | program (help) | 18 cr; 1 | 18+12 = 30 | another player 4 ins 16 + Unity 6 24 = 40 | 1.33 | 1.18 | in band |
| liberation_accords | program, 1 diplomacy | 15 cr; 2 | 15+16 = 31 | 6 agents 30 + 4 amb 32 = 62 | 2.00 | 1.77 | in band |
| exodus_corridors | program, 1 diplomacy | 18 cr; 1 | 18+12 = 30 | gather 6 × 2 = 12 + 4 amb 32 = 44 | 1.47 | 1.29 | in band |
| great_liberation | event, 4 diplomacy | 32 cr, 1 am; 2 | 32+18+16 = 66 | 2 cities 90 + 1 station 15 = 105 | 1.59 | 1.50 | in band |
| orbital_embassy | building, 2 diplomacy, over diplomacy | 10 cr; 2 | 10+16 = 26 | 1 Diplomacy tag/rd 8 × 3.5 = 28 + own play 8 + draw 4 20 + tags 1.5 = 57.5 | 2.21 | 1.92 | in band |
| agency_directorate | building, 2 security, over security | 16 cr; 2 | 16+16 = 32 | own play 3+1 = 4: draw 3 15 + 3 tags/rd × 3.5 = 10.5 Operatives = 2.6 fires × 15 = 39.4 + tags 1.5 = 55.9 | 1.75 | 1.55 | in band |
| martial_cordon | program, 1 diplomacy | 18 cr; 1 | 18+12 = 30 | 6 agents 30 − Discord 1 4 = 26 | 0.87 | 0.76 | under 1.0 |
| deep_state_ledger | program, 2 security | 18 cr; 2 | 18+16 = 34 | +3 ins income 36 + 2 Operatives × 3.75 = 7.5 = 43.5 | 1.28 | 1.14 | in band |
| kinetic_interdiction | event, 2 security | 8 cr, 1 am; 0 | 8+18+8 = 34 | 2 troops 10 + 6 agents 30 − Discord 1 4 = 36 | 1.06 | 0.95 | in at 2 only |
| annihilation_lance | event | 2 cr, 2 am; 0 | 2+36+8 = 46 | 8-piece hex 40 + 2 troops 10 − Discord 2 8 = 42 | 0.91 | 0.84 | under 1.0 |
| fusion_commons | event, 2 science | 4 cr, 2 am; 0 | 4+36+8 = 48 | 6 en 27 + draw 3 15 + Unity 3 12 = 54 | 1.13 | 1.04 | in band |
| vertical_monopoly | building, 3 industry | 20 cr; 2 | 20+16 = 36 | 3 cr × 18.5 tag-rounds 55.5 + draw 4 20 + tags 1.5 = 77 | 2.14 | 1.93 | in band |
| market_stabilization_board | building, 2 industry | 13 cr; 0 | 13+8 = 21 | 2 Industry threats countered 24 + tags 1.5 = 25.5 | 1.21 | 1.02 | in band |
| autonomous_defense_grid | building, 2 cyber | 13 cr; 0 | 13+8 = 21 | 2 Cyber threats countered 24 + tags 1.5 = 25.5 | 1.21 | 1.02 | in band |
| nuclear_option | event | 23 cr, 1 am; 0 | 23+18+8 = 49 | liberate 45 − Discord 2 8 = 37 | 0.76 | 0.70 | under 1.0 |
| power_to_the_people | building, 2 power, over power | 22 cr, 4 en; 0 | 22+18+8 = 48 | 1 cr × 15 tag-rounds 15 + liberate 45 + tags 1.5 = 61.5 | 1.28 | 1.18 | in band |

## Proposals (credit or antimatter price only; target about 1.1-1.3 at keep 3)

- **singularity_exploit** 8 → 4 cr: keep 3 pay 34, 1.18; keep 2 pay 30, 1.33.
- **kinetic_interdiction** 8 → 2 cr: keep 3 pay 32, 1.13; keep 2 pay 28, 1.29 (0 cr reads 1.20 / 1.38; no card prints 1).
- **martial_cordon** 18 → 10 cr: keep 3 pay 22, 1.18; keep 2 pay 18, 1.44. The stale note was under a pay without the keep.
- **nuclear_option** 23 → 2 cr: keep 3 pay 32, 1.16; keep 2 pay 28, 1.32. The r90 note left out the keep (pay 41) and
  set 0.90 on purpose ("the price of an extreme act"); at 23 cr it now reads 0.76 / 0.70.
- **annihilation_lance** credits can't fix it: at 0 cr it reads 0.95 / 0.88. 2 → 1 am at 5 cr: keep 3 pay 35, 1.20;
  keep 2 pay 31, 1.35. It hangs on the stack: at 10 pieces get is 52 (1.13 / 1.04 at today's price), and the r90 note
  calls it a late, huge-stack card by design.
- **positron_storage_ring** 1.2 at keep 3 needs about 67 cr, over the ~50 ceiling. 49 cr plus 1 am: keep 3 pay 88, 1.20;
  keep 2 pay 84, 1.26. At 50 cr alone: 1.49 / 1.58. Smallest in-band fix: 18 cr (2.47 / 2.26). Flag: as with
  penning_trap_array, 1 antimatter a round at 18 (63 of its 106) is the excess.

Thin calls: botnet_takedown, fusion_commons, market_stabilization_board and autonomous_defense_grid sit at 1.02-1.04 at
keep 3. The two counter buildings hang on the threat rate: at 1 threat of their type, 13.5 (0.64 / 0.54); at 3, 37.5
(1.79 / 1.50). Open_weights_release assumes 4 markers in reach.
