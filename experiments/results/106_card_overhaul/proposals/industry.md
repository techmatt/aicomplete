# 106 Industry section (proposal)

25 cards (from 26): 18 tier 1, 7 tier 2; 12 programs, 9 buildings, 4 events. Median printed cost 9; big cards at
40 (Gigafactory Charter, tier 1) and 22 (Orbital Mass Driver, tier 2), with 16, 13 and 12 below them. 7 cut, 6 new.

**Over the kind target, on purpose:** 9 buildings against about 6. The agreed ideas alone need 7 buildings (two
rebates, the discount, the 2-per-tag building, the two trigger engines (the tier-1 one is the Capital building), plus the
typed Action counter and the standing counter, and the over-X income capstone the copy wants). Industry is the
Builder's tag (5 slots). Every flat-income building is now a program; what is left on a building is a trigger, a
discount, a per-tag income or an Action.

**The engine, played well:** Standardized Tooling (Industry cards 4 less) + Lights-Out Assembly or Capital Projects
Office (rebate on the printed cost) make a 10-credit Industry card cost 6 and return 4-10. Retained Earnings and
Conglomerate Headquarters bank Capital on every play; Capital Injection and Vendor Financing fill them past their
At N at once (the threshold repeats while 4 or 3 remain); Liquidity Event and Collateralized Grid cash out instead.
Vertical Monopoly pays 2 per Industry tag, and Payroll Automation copies it (or the Mass Driver) into permanent
income. Gigafactory Charter at 40 nets 28-32 on that seat.

Rates: `values.json`. Pay = credits + 2.5 × insight + keep (2.5 / 6.5) + other costs. Income at 4 rounds (tier 1) or
2.5 (tier 2). "Seat" = a Builder or Industry seat playing about 1.2 Industry cards and 1.5 cards a round, 4-6
Industry tags in play by mid-game, median card near 10.

| id | status | reasoning | value check (pay vs get, when, for whom) | loop and cap |
|---|---|---|---|---|
| payroll_automation | changed (7 → 12 credits) | The copy demo. Its targets are now the per-tag and over-X income buildings, so it copies 12-20 a round, not 3 | Pay 12+5+2.5 = 19.5. Get: Vertical Monopoly at 6 Industry tags, 12 × 4 = 48 (+28); Mass Driver 16 credits + 3 chips, 20.5 × 4 = 82. Table (3 a round) reads 13: the agents will undervalue it. Rounds 5-7, Builder | None: a one-off copy, needs an income building |
| retained_earnings | changed (Action bank → Capital trigger) | **The Capital building and the tier-1 trigger engine.** Effect: when you play an Industry card, add 1 Capital here. At 4: raise your credit income by 3. Uses `play` with the Industry tag, not `play_tag`: the same count (no card prints a tag twice) on a box near 80 characters | Pay 7+2.5+2.5 = 12. Seat: 1.2 Capital a round (+1 for itself), a threshold every ~3 rounds, +3 income for ~5 rounds left = 4.5 a round × 4 = 18 + tag 1 (+7); each feeder adds a threshold at once. **Ledger reads 1** (it doesn't price production inside a threshold): flag for the agents | Capital → income → Industry cards → Capital. Cap: plays (hand, insight); `then` places no tokens (Content::check) |
| expansion_fund | cut | Slow token card (1 Capital a use, spend 1 → factory) | Factory: Prefab Plant Kit; Capital pile: Retained Earnings | |
| prefab_plant_kit | kept | A factory and energy per Power tag: a real Builder play | Pay 9+5 = 14. Get factory 4 + energy at ~2.5 Power tags 11 + tags 1.5 = 16.5 (+2.5). Rounds 2-5 | None |
| bulk_silicon_procurement | cut | 3 credits for "raise chip income by 2": a small card | Chip income: Arcology Foundry, Orbital Mass Driver; chips now: Joint Fabrication Pact, Antitrust Action | |
| station_prefab_order | changed (11 → 10) | Reclamation Crew, the station flip | Pay 10+5 = 15. Get station 15 + tags 1.5 (+1.5). Whenever a station is corrupted in reach | None |
| collateralized_grid | changed (tier 1 → 2, 6 → 10) | 2 energy a Capital is a tier-2 effect | Pay 10+2.5+6.5 = 19. Get: a natural pile of 3, 6 energy = 27 + 1.5 (+9.5); a fed pile of 7, 14 energy = 63. Mid-late, a seat with a Capital building | None: one card's pile, once |
| joint_fabrication_pact | changed (6 → 9; 4/4 → 6/6 chips) | Three tags (feeds Conglomerate HQ); bigger at TM scale | Pay 9+5 = 14. Get 9 + gift 6.75 + tags 2 = 17.75 (+3.75). Any round; the helped seat gets 6 chips | None |
| supply_chain_treaty | cut | Help + income 2 at 9: nets -1 at the table | Help: Vendor Financing, Joint Fabrication Pact; income: Robotic Foundry, Economies of Scale | |
| vendor_financing | changed (6 → 9; gift 3 → 5; Capital 3 → 4) | The help feeder: 4 Capital fills a Retained Earnings pile past At 4 on the next Industry play | Pay 9+5 = 14. Get gift 3.75 + 4 Capital × 3 = 12 + tag 1 = 16.75 (+2.75); into Retained Earnings, +3 income now. Needs a Capital building | Capital in, threshold out: cap is the one-off |
| lights_out_assembly | changed (at 12 → 10; 5 → 7) | The tier-1 rebate demo, retuned to the new median: at 10 it fires on most Industry plays | Pay 7+5 = 12. Seat: 1.5 plays × 60% at 10+ × 4 = 3.6 a round × 4 = 14.4 + 1.5 (+3.9). Table 15.5 (+3.5) | Rebate reads the printed cost, so with Tooling a 10-card costs 6 and returns 4. Cap: plays (hand, insight) |
| materials_hedge_desk | changed (program → event; 7 → 10; 4 → 6 Capital) | Capital Injection: the accelerator. 6 Capital is two Conglomerate thresholds (20 credits) on the next play. An event: no tag left, as an injection shouldn't count | Pay 10+5 = 15. Get 6 × 3 = 18 (+3); into Conglomerate HQ, 20 on the next play (+5); into Retained, +3 or +6 income. Needs a Capital building | Capital → threshold; cap: one-off, `then` places no tokens |
| liquidity_event | changed (rate 3 → 4; 3 → 5) | The other side of the decision: cash the pile now, or wait for At N | Pay 5+5 = 10. Pile 3: 12 (+2); fed pile 7: 28 (+18). Needs a pile | None: one card's pile, once |
| bridge_loan | changed (9 → 15; lowers 1 → 2) | The rare 0-credit card, reasoned: a loan costs income, not credits. TM's scale | Pay keep 2.5 + 2 income × 4 = 10.5. Get 15 (+4.5). Early, to reach a 40-card or a tier-2 building a round sooner | None (0 printed: fires no rebate) |
| plant_overdrive | new | Per-tag event: gain 1 energy per Industry tag on your cards. A late spike for the tag-heavy seat | Pay 9+5 = 14. Seat at 4 Industry tags (an event doesn't count itself) 18 (+4); 6 tags 27 (+13); table at 3 tags 13.5 (-0.5) | Energy → Rush Build sacrifices an Industry tag, so it shrinks its own count; one-off |
| robotic_foundry | changed (building → program; 12 + 1 chip → 13; 4 → 5 income) | Flat income moves to a program (locked direction); frees a slot | Pay 13+5 = 18. Get 5 × 4 = 20 + 1.5 (+3.5). Rounds 2-4 | None |
| gigafactory_charter | new | **The TM "cost 40, gain 10 income" card:** raise your credit income by 12 | Pay 40+2.5+2.5 = 45. Get 48 + 1 (+4). Seat: Tooling 4 off and a rebate 4-8 back → effective 28-32 (+12 to +16). Rounds 3-5, after a saving round; Builder | None: once |
| economies_of_scale | new | Per-tag income as a program: raise your credit income by 1 per Industry tag on your cards (`production_per_tag`; counts itself) | Pay 10+5 = 15. At the minimum 4 tags: 16 + 1 (+2); at 6: 24 + 1 (+10). Hand-priced (extension op). Mid-game | None: counted once at play |
| grid_tied_smelter | cut | A Power teammate trigger with an Industry first tag | Industry triggers: Retained Earnings, Conglomerate HQ. The Power teammate trigger is Power's Substation Hub (power, industry; you both gain 1 energy) | |
| bonded_warehouse | cut | Third Capital Action bank, duplicating the old Retained Earnings | Capital piles: Retained Earnings, Conglomerate HQ; cash-out: Liquidity Event | |
| arcology_foundry | changed (over-X building → tier-2 program; 15 + 2 chips → 16; needs 2 → 3) | Flat income moves to a program; same 9 credits + 2 chips. Tier 2 is power: the needs-3 rung above Robotic Foundry | Pay 16+5+6.5 = 27.5. Get (9 + 3) × 2.5 = 30 + 1 (+3.5). Rounds 4-6, Industry seat | None |
| orbital_mass_driver | changed (needs Industry 3 + Power 1 → Industry 4; energy 2 off; chips 3 → 2; 20 → 22; 13 → 16 income) | The over-X income capstone (allowed: tier 2 built over an Industry building). Fixes the mixed `needs`. Payroll's best copy. Review: income 14 → 16, since at 14 it netted about -1 once the covered building (Market Watchdog, about 5) was counted; the cost stays 22, the section's one tier-2 card at 20+ | Pay 22 + antimatter 7 + chips 3 + 5 + 6.5 = 43.5. Get (16 + 4.5) × 2.5 = 51.25 + 1.5 = 52.75 (+9.25; +4 after covering Market Watchdog); with a Payroll copy, 20.5 × 4 = 82 more | None: income |
| arc_furnace_complex | cut | Over-X chips income + 3 energy → 24 credits: an energy sink, not an Industry engine | Over-X slot: Capital Projects Office, Mass Driver; chips income: Mass Driver, Arcology | |
| sovereign_capital_fund | cut | Tier-2 Capital bank with a when-built one-off (a building's one-off goes) | Tier-2 Capital engine: Conglomerate Headquarters | |
| capital_projects_office | new | **The tier-2 rebate,** over an Industry building (Market Watchdog the cheap one to cover; over Lights-Out it trades a 4 rebate for a 10): when you play a card printed at 10 credits or more, gain 10 credits. Review: 8 → 10, since at 8 over Lights-Out it netted about -3 | Pay 10+2.5+6.5 = 19. Seat: 1.2 plays at 10+ × 10 × 2.5 = 30 + 1 (+12; +7 over Market Watchdog, +3 over Lights-Out). Table 23 (+4). Rounds 4-7 | With Tooling, a 10-credit Industry card costs 6 and returns 10: credits up, but each play costs insight and a card. Cap: plays (hand, insight, the deal of 4); covering Tooling stops the discount |
| conglomerate_headquarters | new | **The tier-2 trigger engine, every tag:** when you play a card, add 1 Capital here per tag on it. At 3: gain 10 credits. Second Capital pile, so feeders and cash-outs have two homes | Pay 10+2.5+6.5 = 19 (needs Industry 4 is the rest). Seat: 1.5 plays × 1.8 tags = 2.7 Capital, 0.9 thresholds × 10 = 9 a round × 2.5 = 22.5 + itself + tag (+4.5); 3-tag cards (Joint Fabrication Pact) 10 credits a play. Hand-priced (`play_tag` with no tag) | Capital → credits → cards → tags → Capital. Cap: plays (hand, insight); `then` places no tokens |
| standardized_tooling | new | **The Industry credit discount:** your Industry cards cost 4 credits less to play | Pay 8+5 = 13. Seat: 1.2 Industry plays × 4 × 4 = 19.2 + 1 (+7). Table 8.2 (generic 30% tagged). Round 1-3, Builder | Cheaper Industry cards → more Industry tags → `industry_every` building discount. Cap: `discount_floor` 1 and plays |
| antitrust_action | kept | Typed counter program (economic jam 5, chips per Industry tag) | Pay 6+0+2.5 = 8.5. Get 3.5 + 4.5 + 1 = 9 (+0.5); more against a live economic threat | None |
| supply_chain_audit | kept | Typed counter program (jam 5, credit income 1) | Pay 4+5 = 9. Get 3.5 + 5 + 1 = 9.5 (+0.5) | None |
| market_watchdog | changed (jam 3 → 4; 4 → 3) | Typed counter Action building; jam 3 at 4 credits netted -2. Same fix as Cyber's Sentinel Mesh (jam 4) | Pay 3+5 = 8. Get 4 × 0.7 × 4 × 0.7 = 7.8 + 1 = 8.8 (+0.8); stacking on a standing economic threat counters it in 2 rounds | None: once a round |
| vertical_monopoly | changed (4 + 1 a tag → 2 a tag; 14 → 12) | **The building paying 2 per Industry tag** (`income_per_tag` `amount` 2), and Payroll's per-tag copy | Pay 12+5+6.5 = 23.5. At the minimum 5 tags 10 × 2.5 = 25 + 1 (+2.5); at 7 tags 35 (+12.5). Hand-priced (extension). Rounds 4-7 | None: income, recounted each round |
| market_stabilization_board | changed (rider off; 13 → 9) | Standing economic counter; the one-off "raise credit income by 2" went (a building's one-off). Same as Cyber's Autonomous Defense Grid | Pay 9+2.5+6.5 = 18. Get 5 × 1.5 × 2.5 + 1 = 19.75 (+1.75). Home-country economic threats | None |

## Flags

- **Capital has two homes** (Retained Earnings, Conglomerate HQ) and four cards that use them (Vendor Financing,
  Capital Injection, Liquidity Event, Collateralized Grid). No other section prints Capital. A seat holding a
  feeder or cash-out without either building can't use it. Smoke-read it with `play_read.py`.
- **The agents can't see the Retained Earnings and Payroll payoffs.** The ledger reads Retained Earnings at 1
  (production inside a threshold) and Payroll at 13 (copy at 3 a round). Review value.rs's `threshold` and
  `copy_income` arms before the first screen.
- **Fit:** Retained Earnings' and Conglomerate HQ's Effect boxes run about 90-100 characters against about 80.
  Telemetry Archive's shape, with a longer payoff. Check with `npm run cards:fit`.
- **Kind mix:** 9 buildings, against the target of about 6 (reason above). Left as is in review: the only clean
  conversion is Orbital Mass Driver to a program, which drops Payroll's best copy target. Cyber (9) and Science (8)
  run over too, so the deck lands near 33% buildings unless the merge trims one or two per section.
- **Over-X value checks** (review) now count the covered building: Capital Projects Office and Orbital Mass Driver
  were retuned because they netted below 0 on their natural cover.
