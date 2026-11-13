# Tunables

*Every baseline config parameter (generated). Terms and conventions are in [README.md](README.md).*

Every parameter of the baseline config, with its value and range, generated from the code and
`configs/baseline.json` (`aic-sim docs`).

<!-- generated: tunables -->
| Parameter | Baseline | Range | What it does |
|---|---|---|---|
| `board.start_corrupted` | 1 | 0..3 | cities per country that start corrupted |
| `board.start_ambassadors` | 1 | 0..10 | player ambassadors each player starts with |
| `board.unit_cap` | 2 | 0..20 | Drones a base holds at most |
| `board.corp_energy` | 1 | 0..10 | energy cubes each city of country i makes a round (a megacorp parameter; the list repeats) |
| `board.city_energy_credits` | 5 | 0..20 | standard action Buy energy: credits for a neutral city's energy, once per city a round, the city in reach and producing |
| `board.aligned_bonus` | 0 | 0..10 | extra energy an aligned city gives its owner |
| `board.station_cubes` | 1 | 0..5 | research cubes a neutral station makes a round (an adversary one: compute) |
| `board.owned_station_cubes` | 2 | 0..5 | research cubes a player-owned station makes its owner a round (142) |
| `board.divert_discord` | 1 | 0..5 | Discord per unit of a corrupted city's energy diverted to the adversary |
| `board.ambassador_discord` | 0 | 0..5 | Discord per unit of energy, each round, of an aligned city an agent darkens |
| `board.liberate_at` | 5 | 1..10 | the Liberate action: ambassadors that liberate an adversary city |
| `board.claim_city` | 5 | 1..10 | the Claim action: ambassadors that claim a neutral city (206: Liberate's table too, its city price `liberate_at`) |
| `board.claim_station` | 3 | 1..10 | the Claim action: ambassadors that claim a neutral research station |
| `board.claim_factory` | 2 | 1..10 | the Claim action: ambassadors that claim a neutral factory |
| `board.start_nodes` | 1 | 1..3 | 175: nodes each player starts with: its start hex, then the lowest-numbered free land hexes next to it (the extras off no track) |
| `board.destruction_discord` | 1 | 0..5 | Discord each facility destroyed in war sways |
| `board.troop_spent` | 1 | 0..1 | 1: an adversary troop that strikes a target is used up |
| `board.troop_steps` | 1 | 0..5 | steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest |
| `board.troop_node` | 0 | 0..4 | adversary troops strike nodes from this stage of adversary card on (0: always, 4: never) |
| `board.vigil_cap` | 2 | 1..10 | 185: troops a round whose march ending next to a seat's node pays its Vigil (later ones that round pay nothing) |
| `board.node_marker` | 1 | 0..1 | 1: a destroyed node leaves a corruption marker on its hex |
| `board.station_chips` | 1 | 0..20 | standard action: chips to build a research station |
| `board.station_ambassadors` | 0 | 0..5 | standard action: ambassadors a research station also uses up (206, candidate; 0: none) |
| `board.node_credits` | 6 | 0..30 | 206: standard action Place a node: credits, beside the track step's energy (0: no such action) |
| `board.troop_credits` | 10 | 0..50 | 206: standard action Remove a troop: credits for 1 troop in reach (0: no such action) |
| `board.node_per_round` | 2 | 1..5 | 206: times a seat may take Place a node a round |
| `board.troop_per_round` | 1 | 1..5 | 206: times a seat may take Remove a troop a round |
| `board.factory_build_chips` | 0 | 0..20 | 206: standard action Build a factory: chips for a factory of yours (0: no such action) |
| `board.factory_chips` | 1 | 0..5 | chips a player-owned factory makes its owner each round |
| `board.city_build_chips` | 3 | 1..10 | 190: chips a city spends to build a neutral factory or research station |
| `board.city_chip_cap` | 3 | 1..10 | 190: chips a city holds at most while they wait (one each round from each unclaimed neutral factory next to it) |
| `board.remove_ambassadors` | 1 | 1..5 | standard action Remove agents: your ambassadors from the supply, used up (r75: it places nothing) |
| `board.remove_count` | 3 | 1..3 | standard action Remove agents: agents it removes, at most, from any hexes in reach (the player's choice) |
| `board.land_cubes` | 2 | 0..10 | 210: standard action Lift land corruption: research cubes to lift 1 land-corruption token in reach (0: no such action) |
| `board.clear_unity` | 2 | 0..5 | Sway toward Unity for clearing a corruption marker |
| `board.clear_credits` | 0 | 0..5 | credits back for clearing a corruption marker |
| `board.envoy_ambassadors` | 1 | 0..5 | standard action Envoy: your ambassadors from the supply, used up, to move 1 Discord off the Discord track (0: no such action) |
| `board.ambassador_credits` | 20 | 1..50 | standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit) |
| `board.sell_credits` | 1 | 0..20 | standard action: credits for selling a card from hand |
| `board.cube_insight` | 1 | 0..5 | standard action, no limit a round: insight for 1 research cube (0: no such action) |
| `board.antimatter_cubes` | 4 | 0..10 | standard action, the desperate source (204): research cubes for 1 antimatter, no limit a round (0: no such action) |
| `board.breakthrough_antimatter` | 1 | 1..10 | standard action Breakthrough (204): antimatter it spends |
| `board.breakthrough_unity` | 0 | 0..10 | standard action Breakthrough (204): Sway toward Unity, once a round (0: no such action) |
| `board.botnet_credits` | 16 | 1..50 | standard action: credits to take down a botnet in reach on a hex holding your ambassador (it stays) |
| `adversary.difficulty` | 0 | 0..4 | difficulty level 0-4 (each level includes the ones below) |
| `adversary.stage_cards` | 4, 4, 3 | 0..20 | adversary cards dealt from stage I, II and III (before difficulty) |
| `adversary.supply_per_country` | 24 | 0..40 | agents in the supply per country |
| `adversary.corrupt_at` | 4 | 1..20 | a city's corrupt number: the agents on it that corrupt it (coastal cities too) |
| `adversary.corrupt_station_at` | 3 | 1..20 | a research station's corrupt number |
| `adversary.corrupt_factory_at` | 3 | 1..20 | a factory's corrupt number |
| `adversary.corrupt_land_at` | 3 | 1..20 | open land's corrupt number (only a Corrupt open land line; it takes land corruption) |
| `adversary.corrupt_node_at` | 2 | 1..20 | 175: a node's corrupt number: a hex with a player's node and this many agents loses the node |
| `adversary.corrupt_cut` | 1, 1, 1, 0 | 0..5 | difficulty corrupt_cut_at+: how much lower each corrupt number is (city, station, factory, land) |
| `adversary.corrupt_cut_at` | 1 | 0..5 | the difficulty level from which corrupt_cut applies |
| `adversary.blight_discord` | 1 | 0..5 | Discord each land-corruption token sways each round |
| `adversary.blight_node_discord` | 0 | 0..5 | 192: Discord each land-corruption token a corrupted node hex took (placed face down) sways each round, in place of blight_discord |
| `adversary.counter_pct` | 100, 100 | 50..300 | every resource counter item of a tier 1, 2 threat costs this percent of its printed amount (rounded up) |
| `adversary.counter_credit_pct` | 100 | 50..300 | every credits counter item costs this percent of its amount after counter_pct (rounded up; 100 = unchanged; 142's dial) |
| `adversary.counter_bonus` | 0 | 0..10 | difficulty counter_bonus_at+: added to every resource item of each threat's first counter option (142; its credits before) |
| `adversary.counter_bonus_all` | 1 | 0..10 | difficulty counter_bonus_at+: added to every resource counter item of every threat |
| `adversary.counter_bonus_credits` | 4 | 0..20 | difficulty counter_bonus_at+: added to every credits counter item, in place of counter_bonus_all (142) |
| `adversary.counter_more_all` | 1 | 0..10 | difficulty counter_more_at+: added again to every resource counter item but credits (142's second step) |
| `adversary.counter_more_credits` | 0 | 0..20 | difficulty counter_more_at+: added again to every credits counter item (142) |
| `adversary.counter_more_at` | 2 | 0..5 | the difficulty level from which counter_more_all and counter_more_credits apply (142) |
| `adversary.counter_bonus_at` | 1 | 0..5 | the difficulty level from which counter_bonus and counter_bonus_all apply (142; 1 before) |
| `adversary.extra_threats` | 1 | 0..5 | difficulty extra_threats_at+: extra threats dealt each round in all, one per country in turn |
| `adversary.extra_threats_at` | 5 | 0..5 | the difficulty level from which extra_threats are dealt (139) |
| `adversary.stage1_removed` | 1 | 0..3 | difficulty stage_swap_at+: stage-I cards removed |
| `adversary.stage3_added` | 1 | 0..3 | difficulty stage_swap_at+: stage-III cards added (the cap holds, its end gets harder) |
| `adversary.stage_swap_at` | 4 | 0..5 | the difficulty level from which stage1_removed and stage3_added apply (142; 2 before) |
| `adversary.stage2_removed` | 2 | 0..3 | difficulty 3+: stage-II cards removed (a shorter round cap) |
| `adversary.stage1_added` | 0 | 0..3 | 151: difficulty 3+: stage-I cards added back (with stage2_removed, a smaller d3 step) |
| `adversary.extra_corrupted` | 0 | 0..3 | difficulty 4: more cities per country start corrupted |
| `adversary.compute_income` | 0, 13, 17, 21, 30 | 0..30 | 151: the adversary's base compute income a round, at difficulty 0-4 (fixed, not per board) |
| `adversary.ambassador_income` | 0, 0, 0, 0, 1 | 0..10 | 151: the agent income a country a round (into each country's pool), at difficulty 0-4 |
| `adversary.tier2_early` | 1 | 0..3 | difficulty 4+: tier-2 threats start this many rounds sooner |
| `adversary.discord_removed` | 0 | 0..3 | difficulty 4+: Discord cards removed at setup (from tier 1) |
| `adversary.wall_troops` | 0 | 0..5 | build: adversary troops landing in each country where a seat with Wall has a node (pressure for Wall to stop) |
| `adversary.wall_landing` | 0 | 0..1 | 192: 1: the Protector's Muster skips players' factories (the lowest-numbered neutral or adversary factory, else city); 0: anyone's |
| `adversary.war_node` | 1 | 0..1 | build (r83, the Militarist's War Takes Ground): 1: a drone strike that destroys a factory destroys a node on its hex too |
| `adversary.station_flip_at` | 2 | 0..5 | agents on a player's research station before an effect can flip it (short of that, one lands there instead) |
| `adversary.botnet_compute` | 1 | 0..5 | adversary compute each botnet makes a round (none with a player ambassador on it) |
| `adversary.board_difficulty_at` | 2 | 0..5 | the difficulty level from which the adversary board's own difficulty line applies (135) |
| `alignment.tip_per_player` | 3 | 1..50 | the Unity track tips when it holds this many Sway per player |
| `alignment.discord_tip_per_player` | 3 | 1..50 | the Discord track tips when it holds this many Sway per player |
| `alignment.carry` | 2 | 0..2 | Sway past a tip stays toward the next tip: 2 both sides (r75, Matt's ruling 7), 1 Unity's only (r48), 0 neither (a tip empties its track) |
| `alignment.hold_off` | 1 | 0..1 | 1: each Unity swayed may instead move 1 Discord off the Discord track, the players' choice (136) |
| `alignment.momentum_every` | 0 | 0..10 | Momentum: a side's track holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off) |
| `alignment.troop` | 0 | 0..10 | Sway toward Unity for removing an adversary troop |
| `alignment.free_city` | 5 | 0..10 | Sway toward Unity for freeing a corrupted city |
| `alignment.free_facility` | 2 | 0..10 | Sway toward Unity for liberating an adversary station or factory (137) |
| `alignment.botnet` | 0 | 0..10 | Sway toward Unity for removing a botnet |
| `alignment.purge_at` | 3, 6, 6 | 1..50 | Unity tips at which the purge asks less: agents, then troops, then botnets stop counting (136; 156: tips, not cards drawn, since the Unity deck reshuffles) |
| `alignment.tier_at` | 3, 6 | 1..50 | 157: the tip (its side's count, this tip included) from which its cards resolve strength II, then strength III |
| `alignment.node_loss` | 1 | 0..1 | 1: a player with no nodes on the board loses the game for everyone |
| `alignment.clear_win_stage` | 1 | 1..3 | the bar-0 win counts only once this round's adversary card is of this stage (1: any time) |
| `ai.boards` | 0, 1, 2 | 0..99 | AI board (index into content/ais.json) of each seat; the list repeats for more seats |
| `ai.start_credits` | 40, 46, 40 | 0..100 | credits each AI board starts with, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `ai.start_energy` | 3 | 0..20 | energy each player starts with |
| `ai.slots` | 5, 4, 3 | 1..10 | building slots of each AI board, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `cards.deal` | 4 | 0..10 | cards each player drafts from each round, choosing each one's tier |
| `cards.opening_deal` | 2 | 0..20 | tier-1 cards dealt into each player's hand in round 1, kept free (the opening; r82) |
| `cards.opening_deal_tier2` | 2 | 0..10 | tier-2 cards also dealt into each player's hand in the opening, kept free (r82) |
| `cards.keep_tier1` | 1 | 0..10 | insight to keep a drafted tier-1 card |
| `cards.keep_tier2` | 2 | 0..10 | insight to keep a drafted tier-2 card |
| `cards.keep_tier2_upgraded` | 1 | 0..10 | insight to keep a tier-2 card once the AI board's keep upgrade is uncovered |
| `cards.science_every` | 2 | 0..10 | 1 insight off tier-2 keeps and card plays per this many Science tags (0: off) |
| `cards.industry_every` | 2 | 0..10 | 1 credit off buildings per this many Industry tags (0: off) |
| `cards.discount_floor` | 1 | 0..5 | tag and building discounts stop at this cost; a cost printed under it stays |
<!-- /generated -->
