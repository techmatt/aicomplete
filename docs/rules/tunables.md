# Tunables

*Every baseline config parameter, generated from the code and `configs/baseline.json` (`aic-sim docs`). Terms and
conventions: [README.md](README.md).*

<!-- generated: tunables -->
| Parameter | Baseline | Range | What it does |
|---|---|---|---|
| `board.start_corrupted` | 1 | 0..3 | cities per country that start corrupted |
| `board.start_ambassadors` | 1 | 0..10 | player ambassadors each player starts with |
| `board.corp_energy` | 1 | 0..10 | energy cubes each city of country i makes a round (a megacorp parameter; the list repeats) |
| `board.aligned_bonus` | 0 | 0..10 | extra energy an aligned city gives its owner |
| `board.station_cubes` | 1 | 0..5 | research cubes a neutral station makes a round (an adversary one: that many onto the adversary's research track, in its Income beat; compute with adversary.research_at 0) |
| `board.owned_station_cubes` | 1 | 0..5 | research cubes a player-owned station makes its owner a round |
| `board.divert_discord` | 1 | 0..5 | Discord per unit of a corrupted city's energy diverted to the adversary |
| `board.ambassador_discord` | 0 | 0..5 | Discord per unit of energy, each round, of an aligned city an agent darkens |
| `board.liberate_at` | 5 | 1..10 | the Liberate action: ambassadors that liberate an adversary city |
| `board.claim_station` | 4 | 1..10 | the Liberate action: ambassadors that liberate an adversary research station (262 cut the Claim action that also paid it; the name stays) |
| `board.claim_factory` | 3 | 1..10 | the Liberate action: ambassadors that liberate an adversary factory (262 cut the Claim action that also paid it; the name stays) |
| `board.start_nodes` | 1 | 1..3 | nodes each player starts with: its start hex, then the lowest-numbered free land hexes next to it (the extras off no track) |
| `board.troop_spent` | 1 | 0..1 | 1: an adversary troop that strikes a target is used up |
| `board.troop_steps` | 1 | 0..5 | steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest |
| `board.troop_node` | 0 | 0..4 | adversary troops strike nodes from this stage of adversary card on (0: always, 4: never) |
| `board.vigil_cap` | 2 | 1..10 | troops a round whose march ending next to a seat's node pays its Vigil (later ones that round pay nothing) |
| `board.station_chips` | 2 | 0..20 | standard action: chips to build a research station |
| `board.station_ambassadors` | 0 | 0..5 | standard action: ambassadors a research station also uses up (candidate; 0: none) |
| `board.troop_credits` | 7 | 0..50 | standard action Remove a troop: credits for 1 troop in reach (0: no such action) |
| `board.troop_per_round` | 1 | 1..5 | times a seat may take Remove a troop a round |
| `board.factory_build_chips` | 0 | 0..20 | standard action Build a factory: chips for a factory of yours (0: no such action) |
| `board.factory_chips` | 1 | 0..5 | chips a player-owned factory makes its owner each round |
| `board.remove_ambassadors` | 2 | 1..5 | standard action Remove agents: your ambassadors from the supply, used up (r75: it places nothing) |
| `board.remove_count` | 3 | 1..3 | standard action Remove agents: agents it removes, at most, on or next to its target (the player's choice) |
| `board.land_cubes` | 2 | 0..10 | standard action Lift Fallout: research cubes to lift 1 Fallout token in reach (0: no such action) |
| `board.ambassador_credits` | 16 | 1..50 | standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit) |
| `board.sell_credits` | 1 | 0..20 | standard action: credits for selling a card from hand |
| `board.cube_insight` | 1 | 0..5 | standard action Turn a cube into insight: insight for `insight_cubes` research cubes, `insight_per_round` times a round (0: no such action) |
| `board.insight_cubes` | 1 | 1..5 | standard action Turn a cube into insight: research cubes a use (252: taxed) |
| `board.insight_per_round` | 1 | 0..5 | 253: times a seat may Turn a cube into insight a round (0: no limit) |
| `board.liberate_neutral` | 1 | 0..1 | 252 (Matt's ruling 4): 1, Liberate leaves the site neutral; 0, it becomes the liberator's (r112, a screen's off arm) |
| `board.cities_unowned` | 1 | 0..1 | 254 (Matt's candidate): 1, players never own a city (no bound city at setup, no city claims, a liberated city is its country's); 0, r114's rule |
| `board.antimatter_cubes` | 5 | 0..10 | standard action, the desperate source: research cubes for 1 antimatter and `antimatter_energy` energy, no limit a round (0: no such action) |
| `board.antimatter_energy` | 2 | 0..5 | 262 (Matt, 2026-11-17): standard action Make antimatter: energy it pays beside its antimatter (a binding resource; still a poor hedge at the anchors) |
| `board.botnet_credits` | 13 | 1..50 | standard action: credits to take down a botnet in reach on a hex holding your ambassador (it stays) |
| `board.std_off` | 0 | 0..11 | a probe's switch: standard actions not offered, by place in the table from 1 (station, remove, lift, dispatch, ambassador, liberate, insight, botnet, antimatter, troop, factory; 0: none off) |
| `adversary.difficulty` | 0 | 0..4 | difficulty level 0-4 (each level includes the ones below) |
| `adversary.stage_cards` | 5, 5, 3 | 0..20 | adversary cards dealt from stage I, II and III (before difficulty) |
| `adversary.supply_per_country` | 24 | 0..40 | agents in the supply per country |
| `adversary.corrupt_at` | 5 | 1..20 | a city's corrupt number: the agents on it that corrupt it (coastal cities too) |
| `adversary.corrupt_station_at` | 3 | 1..20 | a research station's corrupt number |
| `adversary.corrupt_factory_at` | 3 | 1..20 | a factory's corrupt number |
| `adversary.corrupt_node_at` | 2 | 1..20 | a node's corrupt number: a hex with a player's node and this many agents loses the node |
| `adversary.corrupt_cut` | 2, 0, 0 | 0..5 | difficulty corrupt_cut_at+: how much lower each corrupt number is (city, station, factory) |
| `adversary.corrupt_cut_at` | 1 | 0..5 | the difficulty level from which corrupt_cut applies |
| `adversary.city_cut_at` | 5 | 0..5 | 263: the difficulty level from which a city (coastal too) corrupts with one agent fewer again, after corrupt_cut (5: never) |
| `adversary.blight_discord` | 1 | 0..5 | Discord each Fallout token sways each round |
| `adversary.counter_pct` | 100, 100 | 50..300 | every resource counter item of a tier 1, 2 threat costs this percent of its printed amount (rounded up) |
| `adversary.counter_credit_pct` | 100 | 50..300 | every credits counter item costs this percent of its amount after counter_pct (rounded up; 100 = unchanged) |
| `adversary.jam_floor` | 0 | 0..1 | 274: 1, a jam never takes a threat's resource item below 1 (a jam alone never counters); 0, below to 0 |
| `adversary.stage1_removed` | 2 | 0..3 | difficulty stage_swap_at+: stage-I cards removed |
| `adversary.stage3_added` | 1 | 0..3 | difficulty stage_swap_at+: stage-III cards added (the cap holds, its end gets harder) |
| `adversary.stage2_added` | 1 | 0..3 | 267: difficulty stage_swap_at+: stage-II cards added back (tier-2 threats sooner, built into the stack) |
| `adversary.stage_swap_at` | 4 | 0..5 | the difficulty level from which stage1_removed, stage2_added and stage3_added apply |
| `adversary.stage2_removed` | 2 | 0..3 | difficulty 3+: stage-II cards removed (a shorter round cap) |
| `adversary.stage1_added` | 0 | 0..3 | difficulty 3+: stage-I cards added back (with stage2_removed, a smaller d3 step) |
| `adversary.extra_corrupted` | 0 | 0..3 | difficulty 4: more cities per country start corrupted |
| `adversary.compute_income` | 0, 3, 5, 10, 15 | 0..30 | the adversary's base compute income a country a round, at difficulty 0-4 (times the countries: per board) |
| `adversary.infiltrate_agents` | 1, 1, 1, 1, 2 | 0..5 | 275: agents an Infiltrate line lands on each city it names, at difficulty 0-4 |
| `adversary.infiltrate_facility_cap` | 1 | 0..5 | 282: at most this many of an Infiltrate line's agents land on each research station, factory or botnet hex it names (cities take every one) |
| `adversary.threat_agents` | 1 | 0..5 | 275: agents an uncountered threat lands on its anchor (its first location token) as it executes |
| `adversary.discord_removed` | 0 | 0..3 | difficulty 4+: Discord cards removed at setup (from tier 1) |
| `adversary.station_flip_at` | 2 | 0..5 | agents on a player's research station before an effect can flip it (short of that, one lands there instead) |
| `adversary.botnet_compute` | 1 | 0..5 | a botnet's one output: adversary compute each makes a round (none with a player ambassador on it) |
| `adversary.botnet_spread_at` | 1 | 0..5 | 263: the difficulty level from which a 'Botnets spread' line (event cards, projects) spreads botnets; below it the line does nothing |
| `adversary.board_difficulty_at` | 2 | 0..5 | the difficulty level from which the adversary board's `difficulty` line applies (267: the Militarist's War Footing, level 2's named rule) |
| `adversary.research_at` | 1, 1, 1, 1, 1 | 0..30 | 293: K, the research cubes a country that advance the adversary's research track one slot, at difficulty 0-4 (times the countries; 0: no track) |
| `adversary.research_start` | 0 | 0..30 | 293: research cubes a country on the adversary's track at setup |
| `alignment.tip_per_player` | 3 | 1..50 | the Unity track tips when it holds this many Sway per player |
| `alignment.discord_tip_per_player` | 4 | 1..50 | the Discord track tips when it holds this many Sway per player |
| `alignment.carry` | 2 | 0..2 | Sway past a tip stays toward the next tip: 2 both sides (r75, Matt's ruling 7), 1 Unity's only (r48), 0 neither (a tip empties its track) |
| `alignment.momentum_every` | 0 | 0..10 | Momentum: a side's track holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off) |
| `alignment.troop` | 0 | 0..10 | Sway toward Unity for removing an adversary troop |
| `alignment.free_city` | 5 | 0..10 | Sway toward Unity for freeing a corrupted city |
| `alignment.free_facility` | 2 | 0..10 | Sway toward Unity for liberating an adversary station or factory |
| `alignment.botnet` | 0 | 0..10 | Sway toward Unity for removing a botnet |
| `alignment.purge_at` | 3, 7, 7 | 1..50 | Unity tips at which the purge asks less: agents, then troops, then botnets stop counting (tips, not cards drawn, since the Unity deck reshuffles) |
| `alignment.tier_at` | 3, 6 | 1..50 | the tip (its side's count, this tip included) from which its cards resolve strength II, then strength III |
| `alignment.node_loss` | 1 | 0..1 | 1: a player with no nodes on the board loses the game for everyone |
| `alignment.clear_win_stage` | 1 | 1..3 | the bar-0 win counts only once this round's adversary card is of this stage (1: any time) |
| `ai.boards` | 0, 1, 2 | 0..99 | AI board (index into content/ais.json) of each seat; the list repeats for more seats |
| `ai.start_credits` | 27, 31, 27 | 0..100 | credits each AI board starts with, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `ai.start_energy` | 3 | 0..20 | energy each player starts with |
| `ai.slots` | 5, 4, 4 | 1..10 | building slots of each AI board, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `cards.deal` | 4 | 0..10 | cards each player drafts from each round, choosing each one's tier |
| `cards.opening_deal` | 2 | 0..20 | tier-1 cards dealt into each player's hand in round 1, kept free (the opening; r82) |
| `cards.opening_deal_tier2` | 2 | 0..10 | tier-2 cards also dealt into each player's hand in the opening, kept free (r82) |
| `cards.keep_tier1` | 1 | 0..10 | insight to keep a drafted tier-1 card |
| `cards.keep_tier2` | 3 | 0..10 | insight to keep a drafted tier-2 card |
| `cards.keep_tier2_upgraded` | 2 | 0..10 | insight to keep a tier-2 card once the AI board's keep upgrade is uncovered |
| `cards.science_every` | 2 | 0..10 | 1 insight off tier-2 keeps and card plays per this many Science tags (0: off) |
| `cards.industry_every` | 3 | 0..10 | 1 credit off buildings per this many Industry tags (0: off) |
| `cards.discount_floor` | 0 | 0..5 | tag and building discounts stop at this cost; a cost printed under it stays |
| `cards.no_pass` | 0 | 0..1 | 278 (table-time prototype): 1, each player keeps from the hand they built; no picks or passing |
<!-- /generated -->
