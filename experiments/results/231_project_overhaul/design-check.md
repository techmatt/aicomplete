warnings: design: dials-hide-design 3 dials beside 17 design files; design: kept-not-played dyson_swarm_tranche 0.04; design: usage 5 (Protector Debrief 3.02 outside 0.25-2.75, Builder Fortify 0.23 outside 0.25-2.50, Protector Patrol 3.05 outside 0.25-3.00, ...); design: rounds d0 10.5; design: rounds d1 10.0

1152 games, old side 1152
## Warnings (9)
- [dials-hide-design] since 9273640a9: 3 dial(s) moved 40%+ (adversary.compute_income[1] 13->3, adversary.compute_income[2] 17->5, adversary.compute_income[4] 30->18) in the same change as 17 design file(s) (content/adversary.json, content/projects.json, crates/aic-core/src/content.rs, crates/aic-core/src/face.rs...)
- [kept-not-played] dyson_swarm_tranche: kept 0.27 a game, played 0.04
- [usage] Protector Debrief (sacrifice): 3.02 a seat-game, outside its band 0.25-2.75 (dominant)
- [usage] Builder Fortify (troop-response): 0.23 a seat-game, outside its band 0.25-2.50 (unused)
- [usage] Protector Patrol (troop-response): 3.05 a seat-game, outside its band 0.25-3.00 (dominant)
- [usage] Tidal Grid chip converter (converter): 0.00 a seat-game, outside its band 0.30-3.00 (unused)
- [usage] Antimatter spent (resource): 40% of what is gained, spent, outside its band 50%-110% (unused)
- [rounds] d0: 10.5 rounds a game, won games 10.0 (288 games; target 8-10)
- [rounds] d1: 10.0 rounds a game, won games 9.4 (288 games; target 8-10)

## Information (45)
- Make antimatter: give 5 research_cubes = 30, get 1 antimatter = 18 at the anchors: 0.60; 0.32 uses a game (max 1.0)
- Buy a city's energy: give 5 credits = 5, get 1 energy = 4.5 at the anchors: 0.90; 4.55 uses a game (max 1.0)
- Turn a cube into insight: give 1 research_cubes = 6, get 1 insight = 4 at the anchors: 0.67; 6.99 uses a game (max 1.0)
- Build a research station: give 1 chips + 0 ambassadors = 6, get 1 station = 6 at the anchors: 1.00; 2.50 uses a game (max 1.0)
- Remove agents: give 2 ambassadors = 16, get 3 agent = 15 at the anchors: 0.94; 0.96 uses a game (max 1.0)
- Lift Fallout: give 2 research_cubes = 12, get 1 land_token = 12 at the anchors: 1.00; 0.70 uses a game (max 1.0)
- Gain a player ambassador: give 20 credits = 20, get 1 ambassadors = 8 at the anchors: 0.40; 0.97 uses a game (max 1.0)
- Take down a botnet: give 16 credits = 16, get 1 botnet + 0 sway = 16 at the anchors: 1.00; 1.43 uses a game (max 1.0)
- Liberate a city: give 5 ambassadors = 40, get 1 city_liberated + 5 sway (in the anchor) = 45 at the anchors: 1.12; 6.37 uses a game (max 1.0); exempt: the win route (Matt, 2026-11-13): never a poorly priced hedge; its 5 Sway are in the city's 45 (priced_in)
- Envoy: give 1 ambassadors = 8, get 1 sway = 4 at the anchors: 0.50; 0.00 uses a game (max 1.0)
- Sell a card: give 1 card = 5, get 1 credits = 1 at the anchors: 0.20; 1.39 uses a game (max 1.0)
- Place a node: give 6 credits + 1 energy = 10.5, get 1 node = 8 at the anchors: 0.76; 8.36 uses a game (max 1.0)
- Remove a troop: give 10 credits = 10, get 1 troop = 5 at the anchors: 0.50; 0.83 uses a game (max 1.0)
- Build a factory: off (`board.factory_build_chips` 0)
- tag-story exception ais:protector:Reclaim: a Security card retakes a corrupted station or factory by force for an antimatter: Matt's ruling (230), Reclaim stays on Security, force is its story; the cleanse and the antimatter are what the force wins back
- tag-story exception ais:researcher:Disrupt: a Cyber card hacks troops (drones) anywhere in a country, no reach: the Researcher's troop answer is remote by design (206), so it is paid in Cyber, not Security
- antimatter: 14% of its supply (4.43 a seat-game) from standard actions or tier-1 cards (max 20%): use:cyclotron_lease 10%, standard:antimatter 4%
- energy sources, phases (each pools many engines): phase:board.produce 39%, phase:economy.upkeep 38%, phase:threats.execute 2%
- energy: one source, standard:buy_energy, is 5% of its sources (2.28 of 44.79 a seat-game; max 40%)
- chips sources, phases (each pools many engines): phase:board.produce 32%, phase:economy.upkeep 14%
- chips: one source, play:joint_fabrication_pact, is 17% of its sources (2.03 of 11.67 a seat-game; max 40%)
- antimatter sources, phases (each pools many engines): phase:economy.upkeep 30%
- antimatter: one source, convert:0, is 30% of its sources (1.32 of 4.43 a seat-game; max 40%)
- research_cubes sources, phases (each pools many engines): phase:board.produce 92%, phase:threats.execute 1%, phase:economy.upkeep 1%
- research_cubes: one source, play:collider_consortium, is 4% of its sources (0.89 of 23.59 a seat-game; max 40%)
- ambassadors sources, phases (each pools many engines): phase:adversary.compute 0%
- ambassadors: one source, standard:liberate, is 12% of its sources (3.19 of 25.96 a seat-game; max 40%)
- energy uses, phases (each pools many engines): phase:threats.execute 1%
- energy: one spend, standard:node, is 13% of its uses (4.49 of 33.50 a seat-game; max 50%)
- chips uses, phases (each pools many engines): phase:threats.execute 0%
- chips: one spend, convert:1:chips, is 42% of its uses (2.66 of 6.41 a seat-game; max 50%)
- antimatter uses, phases (each pools many engines): phase:adversary.compute 0%
- antimatter: one spend, play:accelerator_diplomacy, is 19% of its uses (0.34 of 1.78 a seat-game; max 50%)
- research_cubes uses, phases (each pools many engines): phase:threats.execute 1%
- research_cubes: one spend, convert:0, is 27% of its uses (3.97 of 14.84 a seat-game; max 50%)
- ambassadors uses, phases (each pools many engines): phase:adversary.compute 0%, phase:threats.execute 0%
- ambassadors: one spend, standard:liberate, is 54% of its uses (10.45 of 19.34 a seat-game; max 50%); exempt: Liberate is the win route (Matt, 2026-11-13): ambassadors exist to free cities, so most of them going there is the design, not one spend crowding the rest
- chips: spent in 98% of won games at d3,4 (129 won; min 75%)
- energy: spent in 100% of won games at d3,4 (129 won; min 75%)
- antimatter: spent in 84% of won games at d3,4 (129 won; min 75%)
- research_cubes: spent in 97% of won games at d3,4 (129 won; min 75%)
- d2: 9.7 rounds a game, won games 9.2 (288 games; target 8-10)
- d3: 8.6 rounds a game, won games 8.0 (288 games; target 8-10)
- tokens a round, adversary ops: 31.4 against 33.5 (-6%; max +10%)
- tokens a round, player token moves: 7.6 against 9.4 (-19%; max +10%)
