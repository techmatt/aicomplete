# 110: art for the cards that had none (r43)

Every one of the 154 cards in the r43 deck (`configs/baseline.json`) now names a scene in
`tools/card-art/scenes.json`, drawn and cut, and the card viewer serves it (`npm run cards:check`
now fails otherwise). 31 cards changed scene: 30 had none (106's 28, 108's Patent Library and
Parts Exchange), and Capital Injection (id `materials_hedge_desk`) still wore the hedge-desk
scene of its old name. No other renamed card's scene contradicts its name: Stress Test Suite,
Reclamation Crew, Shadow Broker Scrub and Early Warning Drill were redrawn for their names in
batch 5; Credential Harvest keeps `forged_badge`, a forged pass at a gate, which fits stolen
credentials broadly.

Reused scenes are scenes already drawn and kept for a card that was cut or never in the deck.
New scenes are batch 6 (`art-originals/scenes/b6/`), the batch-5 preamble, emphasis, lighting
and framing unchanged. `assign.py` makes the pool edit and `reused.json`;
`reuse_picker.mjs` adds the reused rows to `picks/scene_batch_6.json`.

| Card | Scene | Source | Fit |
|---|---|---|---|
| Research Royalties | `royalties_from_inventions` | new | inventions on plinths paying coins into a tray |
| University Spinout | `founder_pitch` | reused (Venture Capital) | founder pitching a prototype to investors: good |
| Interdisciplinary Institute | `policy_institute_hall` | reused (Policy Institute) | scientists presenting to a mixed hall; "interdisciplinary" only by the mixed audience |
| Patent Library | `crawler_in_the_stacks` | reused (Literature Crawler) | a robot pulling volumes in a great library; a library, not patents in particular |
| Standardized Tooling | `matched_tool_wall` | new | identical tool heads over identical machines |
| Economies of Scale | `stacked_container_wing` | reused (Modular Expansion) | a crane adding one more identical factory module: scale, broadly |
| Plant Overdrive | `arc_furnace` | reused (Arc Furnace Complex) | a furnace at full blast; reads as industry pushed hard, energy only implied |
| Gigafactory Charter | `gigafactory_groundbreaking` | new | ribbon cut at a vast new factory |
| Capital Projects Office | `fund_site_plans` | reused (Expansion Fund) | a team unrolling expansion plans on a staked lot: good |
| Conglomerate Headquarters | `conglomerate_tower_plaza` | new | tower joined to factory, lab, warehouse and shipyard |
| Capital Injection | `cash_into_the_works` | new | an armored truck unloading gold bars into a plant (replaces `hedge_desk_commodities`, its old name's scene) |
| Parts Exchange | `chip_market_stalls` | reused (Chip Spot Market) | traders swapping trays of chips: good |
| Ransomware Syndicate | `padlocked_server_room` | new | server room chained shut, a briefcase changing hands |
| Cryptojacking Campaign | `siphon_pipes` | reused (Compute Siphon) | compute drained off into a hidden tank: good |
| Exploit Broker | `broker_in_the_cafe` | reused (Shadow Broker Feed) | a folder slid across a cafe table: good |
| Zero-Day Auction | `zero_day_bidders` | new | masked bidders and a glowing key in a case |
| Command-and-Control Grid | `loft_command_console` | new | one console threading out to the city's devices |
| Pumped Hydro Storage | `mountain_reservoir_pipes` | new | upper reservoir, penstocks, turbine hall |
| Capacity Market | `power_auction` | reused (Surplus Auction) | bidders on power plant: good |
| Dispatch Control Room | `dispatch_wall_map` | new | operators raising a line on a grid wall map |
| Smart Grid Backbone | `armored_conduit_trench` | reused (Hardened Conduits) | a trunk conduit laid in a trench; "smart" not shown |
| Trade Delegation | `envoy_jet` | reused (Shuttle Diplomacy) | an envoy boarding a jet; trade only by the card's name |
| Peace Conference | `council_chamber` | reused (Planetary Assembly) | delegates in a round chamber: good |
| Reconciliation Office | `trust_bureau_counters` | reused (Civic Trust Bureau) | public service counters; an office, reconciliation not shown |
| Protocol Office | `walled_embassy` | reused (Embassy Compound) | an embassy and its gate; protocol by setting |
| Joint Task Force | `shoulder_to_shoulder_shields` | reused (Mutual Defense Pact) | two flags' troops on one wall: good |
| Quick Reaction Force | `strike_team_rooftop` | reused (Strike Team) | fast-roping from a helicopter: good |
| Defense Appropriations | `budget_before_the_dais` | new | a committee voting with equipment before it |
| Multiyear Defense Budget | `depot_rows_to_horizon` | new | vehicle rows to the horizon, an officer with a bound plan |
| Counterinsurgency Campaign | `triage_ops_table` | reused (Threat Triage) | officers moving markers on a map table: a campaign planned |
| Homeland Security Act | `act_on_the_capitol_steps` | new, drawn twice | a signed act shown above guards, crews and engineers; the first draw put red crosses on the crews, redrawn with plain uniforms |

Considered and not used: `treaty_port_cranes` for Trade Delegation, because it flies real
nations' flags (Japan, the Netherlands). It is now `was: supply_chain_treaty` like other cut cards.
`dark_control_room` for Dispatch Control Room shows a blackout, against that card's job.

25 scenes still named a card 106 or 108 cut through `card` (25 viewer warnings, each shown
twice in `cards:check`). They now carry `was` instead, as the pool's note asks. No scene was
deleted. Adversary, Discord and Unity scenes are untouched.
