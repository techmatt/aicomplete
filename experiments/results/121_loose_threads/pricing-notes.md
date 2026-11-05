# Card-pricing notes: one anchor shape (121)

Matt's ruling on 119's deferred call 16: every note in `docs/card-pricing-notes.json` ends with
`anchors: pay N; get M = R.` (plain totals, R = M/N to 2 decimals). Done by
`pricing_notes_anchors.py` (this folder); `notes_before.txt` holds all 158 notes as they were.
158 notes before and after, same ids, same order, same layout (the writer reproduces the old file
byte for byte before writing).

Counts: 66 already in shape and untouched (68 less the two repriced here); 47 rewritten by regex
(13 B, `pay 29 + 16 = 45` or 117's `pay 34 + 4 x 2 = 42`, and 34 C,
`pay 12+4+4=20; get <breakdown> = M = R`: pay is the last pay number, get and ratio the last two,
the breakdown dropped, the stated ratio checked against M/N); 45 written by hand (below): 3 repriced
in 121, 13 E (r42/106 `Pay = N. Get = M`), 9 F (r43/108 `Give N ... Get`), 3 G, 17 X (ranges or
conditions; a range takes its midpoint, an alternative such as "alone / with Open Assembly" is
priced at the unconditional case with the other in words before the clause).

Judgment calls:
- F buildings (7) printed "Get B when built, then P a round, back in K rounds" with no total. Get is
  priced as B + its own play + 3.5 rounds of P (the 3.5-round horizon the r44 notes use), so these
  read 1.81-2.48; the payback time stays in words.
- E notes priced insight at 2.5 (r42's anchor); their pay and get totals are kept as printed, not
  repriced at today's 4.
- ceasefire_monitors had no pay/get at all: priced at today's unit anchors (8 cr + 2 x 4 insight = 16;
  3 ambassadors off 15 + a troop off 5 + 2 ambassadors 8 = 28).
- Two already-in-shape notes round off by a cent and were left: logic_bomb (16.9/17 = 0.99, printed
  1.00, its rationale says "reads 1.0") and liaison_corps (18/16 = 1.125, printed 1.12).
- market_stabilization_board's mojibake (a replacement character for the times sign) went with its breakdown.
- Cards with no note (breakthrough_leak, shared_compute_grant, prefab_plant_kit, shadow_broker_feed,
  transmission_corridor, hardpoint_lockdown, antitrust_action) still have none.

## Hand-written notes, old text

- `peace_conference`: r48 (121): Sway 8 (from 5), two Unity tips at two players now that Sway carries over a tip; 28 cr (from 22); anchors: pay 36; get 44 = 1.22.
- `popular_uprising`: r45 (112): back to r43's price (28 cr, 2 ins; 111 had cut it to 22 cr, 2 ins to reach a band); anchors: pay 44; get 58.1 = 1.32.
- `endowed_research_chair`: r42 (106): Pay 8 + 2.5 + 2.5 = 13. Get 2 × 2.5 × 4 = 20 − 4 (credit income lowered by 1) + 1 = +4. Researcher, early.
- `market_stabilization_board`: r42 (106): Pay 9+2.5+6.5 = 18. Get 5 × 1.5 × 2.5 + 1 = 19.75 (+1.75). Home-country economic threats
- `peaker_turbines`: r42 (106): Pay 23. Get 14.4 + 2 x 1.5 x 4 + 1.5 = 27.9, so +4.9. The Builder in rounds 1-4.
- `hardened_bunker_complex`: r42 (106): Pay 16+5 = 21. Get 14.4 + 8 + 1 = 23.4. Protector early.
- `bounty_board`: r42 (106): Pay 10+5 = 15. Get 11 + about 5 (Security seat) + 1 = 17, plus HQ 10.
- `total_containment`: r42 (106): Pay 8+5+6.5 = 19.5. Get 16.6 + 4 + 1 = 21.6, plus HQ 23.7.
- `orbital_observatory_array`: r42 (106): Pay 10 + 5 + 6.5 = 21.5. Get (12 − 2) × 2.5 × 0.9 = 22.5 + 1.5 = +2.5. With a reuse each round, +25. Researcher (insight to keep is not needed: drawn cards cost no keep).
- `arcology_foundry`: r42 (106): Pay 16+5+6.5 = 27.5. Get (9 + 3) × 2.5 = 30 + 1 (+3.5). Rounds 4-6, Industry seat
- `planetary_referendum`: r42 (106): Pay 14+5+6.5=25.5. Get 32.
- `peacekeeper_rotation`: r42 (106): Pay 12 + 2.5 + 2.5 = 17. Get 22 (2 troops in reach) + 1.5 + 1.5 = 25 (+8), plus 20 with Security Contractor HQ. Protector.
- `fact_check_network`: r42 (106): Pay 6+2.5=8.5. Get 3.5+8+1=12.5. On an influence threat, a jam of 5 often counters it outright.
- `truth_commission`: r42 (106): Pay 8.5. Get 3.5+8+1=12.5.
- `sample_repository`: r42 (106): Pay 8 + 2.5 = 10.5. Get 3 × 4 + 1 = 13, +2.5. Researcher, early.
- `quick_reaction_force`: r42 (106): Pay 9+2.5+2.5 = 14. Get at the table 5 × 0.7 × 4 + 1 = 15 (+1). With Interceptor Battery and HQ: (10.4 - 4.5) + 14.8 = 20.7 a use, × 0.7 × 4 = 58. Rounds 3-8.
- `signal_hijack`: r43 (108): 8: fewer cheap cards, bigger and dearer. Give 11 (credits, insight to play and keep at 2.5); get 12 at the table (effect ledger, reasoned).
- `liquidity_event`: r43 (108): 7: the Capital engine; 8: fewer cheap cards, bigger and dearer. Give 12 (credits, insight to play and keep at 2.5); get 15 at the table (effect ledger, reasoned).
- `data_licensing_deal`: r43 (108): 2-3: pays up front, back within 2 rounds. Give 22 (credits, insight to play and keep at 2.5). Get 12 when built + 0 on its own play, then 10.0 a round at a committed seat: pays back in 1.1 rounds. a Dataset seat (Telemetry Archive, Synthetic Data Forge)
- `security_contractor_hq`: r43 (108): 4: needs lowered; 2-3: pays up front, back within 2 rounds. Give 19 (credits, insight to play and keep at 2.5). Get 12 when built + 0 on its own play, then 10.0 a round at a committed seat: pays back in 0.7 rounds. Security seat removing troops
- `foundation_model_campus`: r43 (108): 6: not built over, so it buries nothing it should not; 4: needs lowered; 2-3: pays up front, back within 2 rounds. Give 32 (credits, insight to play and keep at 2.5). Get 16 when built + 0 on its own play, then 12.0 a round at a committed seat: pays back in 1.3 rounds. Researcher, rounds 4-6
- `orbital_mass_driver`: r43 (108): 5: antimatter only on ultimates; 6: not built over, so it buries nothing it should not; 4: needs lowered. Give 41 (credits, insight to play and keep at 2.5). Get 12 when built + 0 on its own play, then 20.5 a round at a committed seat: pays back in 1.4 rounds. Builder, rounds 4-6
- `interdisciplinary_institute`: r43 (108): 4: needs lowered; 2-3: pays up front, back within 2 rounds. Give 22 (credits, insight to play and keep at 2.5). Get 12 when built + 5 on its own play, then 7.5 a round at a committed seat: pays back in 0.7 rounds. any multi-tag seat, mid-game
- `conglomerate_headquarters`: r43 (108): 4: needs lowered; 7: the Capital engine; 2-3: pays up front, back within 2 rounds. Give 22 (credits, insight to play and keep at 2.5). Get 8 when built + 4 on its own play, then 12.0 a round at a committed seat: pays back in 0.8 rounds. a multi-tag Industry seat, mid-game
- `command_and_control_grid`: r43 (108): 4: needs lowered; 2-3: pays up front, back within 2 rounds. Give 28 (credits, insight to play and keep at 2.5). Get 12 when built + 5 on its own play, then 15.0 a round at a committed seat: pays back in 0.7 rounds. any multi-tag seat, mid-game
- `ceasefire_monitors`: r38: a troop off is 11 (15 less its Sway, alignment.troop 0), 7 in a country; value 25 to 21, 10 to 8 at its ratio (0.5).
- `zero_click_breach`: r42 (106): 10+5+2.5 = 17.5 vs 19.8 + 1 = 20.8 (+3.3). With Darknet and Hyperscale, 5 + 2.5 + 2.5 = 10 (+10.8), and Ops Floor adds 7.5
- `adaptive_malware`: r42 (106): 6+2.5+2.5 = 11 vs 8 × 1.65 + 1 = 14.2 (+3.2). Cyber seat, mid-game
- `collateralized_grid`: r44 (111): anchors: pay 10+8+4=22; alone 28 = 1.27; with 3-4 Capital 1.89-2.09.
- `neural_mirror`: r44 (111): anchors: pay 12+4=16; get +2 insight income at 4 Cyber tags 24 = 1.50 (1.0 at 3 tags, 2.25 at 6).
- `open_assembly`: r44 (111): anchors: pay 15+4+4=23; get sway 3 at build 12 + 3.5 Diplomacy plays = 1.75 more fires x 12 = 21; total 33 = 1.43.
- `orbital_embassy`: r44 (111): anchors: pay 18+8+8=34 (+ the Diplomacy building it covers); get 3 tags a round x 4 x 3.5 rounds 42 + draw 4 20 = 62 = 1.82.
- `exploit_broker`: r44 (111): anchors: pay 12+4+4=20; get its own play 4 + 4 x 3.5 rounds = 14 + draw 2 = 10, total 28 = 1.40.
- `multiyear_defense_budget`: r44 (111): anchors: pay 30+4+4=38; get credit income 5 at 4 = 20 + energy income 2 at 14 = 28; 48 = 1.26.
- `manifesto_leak`: r44 (111): anchors: pay 8+4=12; alone sway 3 12 = 1.0; with Open Assembly 18 = 1.5.
- `spoofed_credentials`: r44 (111): anchors: pay 7+4=11; 3-4 Cyber tags 12-16 = 1.09-1.45.
- `great_liberation`: r44 (111): anchors: pay 32+7 am+8+8=55; 2 cities + a station 105 = 1.91; 3 cities 2.45; 1 city + 2 stations 1.36.
- `cryo_compute_burn`: r44 (111): anchors: pay 2+4+4.5=10.5; 3-4 Science tags 12-16 = 1.14-1.52.
- `field_grant`: r44 (111): anchors: pay 8+4+0=12; get 3 megaproject cubes x 6 (values project_cube) 18 = 1.5 (with a station; about 1.05 with none in reach yet).
- `synthetic_data_forge`: r44 (111): anchors: pay 8+4+4=16; get 4 Actions (add to 2, cash 2 x 3) 6 insight 24 = 1.5; 3 Actions about 20 = 1.25.
- `safehouse_network`: r44 (111): anchors: pay 14+4+4=22; get 2 spends over ~4 rounds = 6 cards 30 = 1.36 (feeders add spends).
- `power_to_the_people`: r44 (111): anchors: pay 18+4+8+4 energy 18=48; get city 45 + 4-5 Power tags x 1 a round 16-20 = 61-65 = 1.27-1.35 (recorded).
- `capital_projects_office`: r44 (111): anchors: pay 16+8+4=28; get draw 4 20 + 10 credits x 0.75-1 a round x 3.5 rounds 26-35 = 46-55 = 1.64-1.96.
- `joint_fabrication_pact`: r44 (111): anchors: pay 10+4+4=18; get 6 chips 18 + gift 3 chips 9x0.75=6.75 + 3 tags 2 = 26.75 = 1.49 (own share alone 20 = 1.11).
- `vertical_monopoly`: r44 (111): anchors: pay 20+8+8=36; 5 tags 52.5 + draw 4 20 = 72.5 = 2.01; 6 tags 2.31.
