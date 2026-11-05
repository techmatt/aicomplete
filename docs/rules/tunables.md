# Tunables

*Every baseline config parameter (generated). Terms and conventions are in [README.md](README.md).*

Every parameter of the baseline config, with its value and range, generated from the code and
`configs/baseline.json` (`aic-sim docs`).

<!-- generated: tunables -->
| Parameter | Baseline | Range | What it does |
|---|---|---|---|
| `board.start_corrupted` | 1 | 0..3 | cities per country that start corrupted |
| `board.start_ambassadors` | 1 | 0..10 | player ambassadors each player starts with |
| `board.unit_cap` | 2 | 0..20 | corp units a base holds at most |
| `board.factory_units` | 1 | 0..5 | corp units each factory makes on a production step at each of its corp's bases |
| `board.clock` | 1, 2, 1, 2, 1, 3 | 0..3 | the printed round track, per round: 1 production, 2 war, 3 both (the last entry repeats) |
| `board.corp_energy` | 1 | 0..10 | energy cubes each city of country i makes a round (a megacorp parameter; the list repeats) |
| `board.city_energy_credits` | 5 | 0..20 | credits for a neutral city's energy cube, bought by a player with a node next to it |
| `board.aligned_bonus` | 0 | 0..10 | extra energy an aligned city gives its owner |
| `board.station_cubes` | 1 | 0..5 | research cubes a station makes a round |
| `board.divert_discord` | 1 | 0..5 | Discord per unit of a corrupted city's energy diverted to the adversary |
| `board.ambassador_discord` | 0 | 0..5 | Discord per unit of energy, each round, of an aligned city an adversary ambassador darkens |
| `board.base_discord` | 1 | 0..10 | Discord an adversary ambassador on a producing base sways |
| `board.base_unity` | 2 | 0..10 | Unity a player ambassador on a producing base sways |
| `board.factory_unity` | 1 | 0..10 | Unity a player ambassador on a producing factory sways |
| `board.liberate_at` | 5 | 1..10 | player ambassadors on a corrupted city, from seats with a node on or next to it, that liberate it (the fallback) |
| `board.last_safe_stage` | 2 | 0..3 | while this round's adversary card is below this stage, nothing destroys a player's last node (0: never safe) |
| `board.free_leaves` | 1 | 0..5 | adversary ambassadors left on a city as it frees (the piece ladder) |
| `board.destruction_discord` | 0 | 0..5 | Discord each facility destroyed in war sways |
| `board.troop_spent` | 1 | 0..1 | 1: an adversary troop that strikes a target is used up |
| `board.troop_steps` | 2 | 0..5 | steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest |
| `board.troop_node` | 4 | 0..4 | adversary troops strike nodes from this stage of adversary card on (0: always, 4: never) |
| `board.node_marker` | 1 | 0..1 | 1: a destroyed node leaves a corruption marker on its hex |
| `board.corruption_reach` | 1 | 0..1 | a city that corrupts destroys nodes within this many hexes of it |
| `board.war_reach` | 0 | 0..1 | war that destroys a factory destroys nodes within this many hexes of it |
| `board.station_chips` | 2 | 0..20 | standard action: chips to build a research station |
| `board.factory_chips` | 1 | 0..5 | chips a player-owned factory makes its owner on a production step |
| `board.chips_credits` | 1 | 0..50 | standard action: credits for selling 1 chip |
| `board.remove_ambassadors` | 1 | 1..5 | standard action: your ambassadors from the supply that go onto a hex to evict its adversary ambassadors, and stay there |
| `board.remove_count` | 3 | 1..3 | standard action: adversary ambassadors one removal takes off its hex, at most |
| `board.clear_cost` | 2 | 0..20 | standard action: credits to clear a corruption marker within reach |
| `board.clear_unity` | 1 | 0..5 | Sway toward Unity for clearing a corruption marker |
| `board.clear_credits` | 0 | 0..5 | credits back for clearing a corruption marker |
| `board.move_credits` | 1 | 0..20 | standard action: credits to move a player ambassador |
| `board.ambassador_credits` | 20 | 1..50 | standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit) |
| `board.sell_credits` | 1 | 0..20 | standard action: credits for selling a card from hand |
| `board.harden_credits` | 3 | 1..50 | standard project: credits for a hardening token |
| `board.drain_credits` | 5 | 1..50 | standard project: credits to drain compute from an adversary project |
| `board.drain_amount` | 3 | 1..20 | standard project: compute drained |
| `board.free_station_credits` | 6 | 0..50 | standard action: credits to free a corrupted research station |
| `board.free_station_ambassadors` | 2 | 1..5 | standard action: your ambassadors on a corrupted research station that freeing it uses up |
| `board.cube_insight` | 1 | 0..5 | standard action, no limit a round: insight for 1 research cube (0: no such action) |
| `adversary.difficulty` | 0 | 0..4 | difficulty level 0-4 (each level includes the ones below) |
| `adversary.stage_cards` | 4, 5, 2 | 0..20 | adversary cards dealt from stage I, II and III (before difficulty) |
| `adversary.supply_per_country` | 24 | 0..40 | adversary ambassadors in the supply per country |
| `adversary.base_sites_per_country` | 1 | 0..1 | 1: arrivals and escalation reach only each country's first military base, its lowest-numbered (0: every base) |
| `adversary.city_build` | 2 | 0..10 | build: ambassadors each corrupted city gains |
| `adversary.factory_build` | 0 | 0..10 | build: ambassadors each factory next to a corrupted city gains |
| `adversary.spread_range` | 2 | 1..6 | spread: how far (hexes) a spreading ambassador may travel |
| `adversary.corrupt_at` | 4 | 1..20 | corrupt: a city that is not corrupted, holding this many adversary ambassadors, is corrupted |
| `adversary.escalation_ambassadors` | 1 | 0..5 | stage-II escalation: ambassadors each base it reaches gains when the card builds |
| `adversary.counter_bonus` | 0 | 0..10 | difficulty 1+: added to every threat's first counter requirement |
| `adversary.counter_bonus_all` | 0 | 0..10 | difficulty 1+: added to every counter requirement of every threat |
| `adversary.extra_threats` | 1 | 0..5 | difficulty 1+: extra threats dealt each round in all, one per country in turn |
| `adversary.stage1_removed` | 1 | 0..3 | difficulty 2+: stage-I cards removed |
| `adversary.stage3_added` | 1 | 0..3 | difficulty 2+: stage-III cards added (the cap holds, its end gets harder) |
| `adversary.stage2_removed` | 1 | 0..3 | difficulty 3+: stage-II cards removed (a shorter round cap) |
| `adversary.extra_corrupted` | 0 | 0..3 | difficulty 4: more cities per country start corrupted |
| `adversary.build_bonus` | 1 | 0..5 | difficulty 3+: ambassadors each corrupted city gains on top of city_build |
| `adversary.tier2_early` | 1 | 0..3 | difficulty 4+: tier-2 threats start this many rounds sooner |
| `adversary.discord_removed` | 1 | 0..3 | difficulty 4+: Discord cards removed at setup (from tier 1) |
| `adversary.arrivals` | 1 | 0..5 | build: ambassadors each site the adversary card names gains, whatever the board holds |
| `adversary.wall_troops` | 1 | 0..5 | build: adversary troops landing in each country where a seat with Wall has a node (pressure for Wall to stop) |
| `adversary.station_flip_at` | 2 | 0..5 | adversary ambassadors on a player's research station before an effect can flip it (short of that, one lands there instead) |
| `adversary.botnet_compute` | 1 | 0..5 | adversary compute each botnet makes a round (none with a player ambassador on it) |
| `adversary.project_cards` | 6, 6 | 0..20 | adversary projects dealt from stage 1 (cheap) and stage 2 (big), cheap on top |
| `adversary.project_row` | 3 | 1..6 | adversary projects visible at once; a new row is dealt when all have resolved |
| `threats.per_country` | 1 | 0..5 | threats dealt each round per country |
| `alignment.undecided_per_player` | 4 | 1..50 | Sway in the Undecided at the start, per player |
| `alignment.tip_per_player` | 2 | 1..50 | the Unity bucket tips when it holds this many Sway per player |
| `alignment.discord_tip_per_player` | 2 | 1..50 | the Discord bucket tips when it holds this many Sway per player |
| `alignment.carry` | 1 | 0..2 | Sway past a tip stays toward the next tip: 1 Unity's only (Matt, r48), 2 both sides (an adversary lever), 0 neither (a tip empties its bucket) |
| `alignment.momentum_every` | 0 | 0..10 | Momentum: a side's bucket holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off) |
| `alignment.troop` | 0 | 0..10 | Sway toward Unity for removing an adversary troop |
| `alignment.free_city` | 5 | 0..10 | Sway toward Unity for freeing a corrupted city |
| `alignment.botnet` | 2 | 0..10 | Sway toward Unity for removing a botnet |
| `alignment.threat_discord` | 1 | 0..10 | Sway toward Discord when a threat executes uncountered |
| `alignment.adversary_discord` | 0 | 0..10 | Sway toward Discord the adversary adds at the start of every round |
| `alignment.antimatter_discord` | 1 | 0..5 | the antimatter action: Sway toward Discord to produce 1 antimatter (0: no such action) |
| `alignment.antimatter_uses` | 1 | 0..5 | times each player may take the antimatter action per round |
| `alignment.unity_ease` | 2, 2, 4 | 1..50 | Unity tip counts at which the clear-board bar drops a level: ambassadors, then troops, then botnets stop counting (the first also starts the stronger Unity cards) |
| `alignment.unity_cards` | 3, 3 | 1..6 | Unity cards a game deals from the printed deck, tier 1 then tier 2 (drawing the last one wins) |
| `alignment.discord_tier2` | 2 | 1..50 | Discord tip count from which a Discord tip draws the stronger tier |
| `alignment.node_loss` | 1 | 0..1 | 1: a player with no nodes on the board loses the game for everyone |
| `alignment.clear_win_stage` | 1 | 1..3 | the bar-0 win counts only once this round's adversary card is of this stage (1: any time) |
| `economy.hold_credits` | 150 | 1..9999 | the most credits a player may hold: a gain past it is lost (122) |
| `economy.hold_energy` | 10 | 1..999 | the most energy a player may hold: a gain past it is lost (122) |
| `economy.hold_chips` | 10 | 1..999 | the most chips a player may hold: a gain past it is lost (122) |
| `economy.hold_ambassadors` | 10 | 1..999 | the most ambassadors a player may hold in supply: a gain past it is lost (122) |
| `economy.credit_production` | 60 | 0..9999 | the most a player's credit production may be raised by cards (programs, Copy): a raise past it is lost (122) |
| `ai.boards` | 0, 1, 2 | 0..99 | AI board (index into content/ais.json) of each seat; the list repeats for more seats |
| `ai.start_credits` | 40, 46, 40 | 0..100 | credits each AI board starts with, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `ai.start_energy` | 1 | 0..20 | energy each player starts with |
| `ai.slots` | 5, 4, 3 | 1..10 | building slots of each AI board, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `cards.deal` | 4 | 0..10 | cards each player drafts from each round after the first, choosing each one's tier |
| `cards.opening_deal` | 8 | 0..20 | cards dealt to each player in round 1 (the opening) |
| `cards.opening_free` | 4 | 0..20 | opening cards each player keeps free |
| `cards.opening_keep_credits` | 5 | 0..20 | credits for each opening card kept beyond the free ones |
| `cards.opening_deal_tier2` | 2 | 0..10 | tier-2 cards also dealt to each player in the opening (108) |
| `cards.opening_keep_tier2` | 1 | 0..10 | insight to keep an opening tier-2 card; tier-2 keeps take none of the free ones (108) |
| `cards.keep_tier1` | 1 | 0..10 | insight to keep a drafted tier-1 card |
| `cards.keep_tier2` | 2 | 0..10 | insight to keep a drafted tier-2 card |
| `cards.keep_tier2_upgraded` | 1 | 0..10 | insight to keep a tier-2 card once the AI board's keep upgrade is uncovered |
| `cards.insight_credits` | 5 | 1..20 | credits for 1 insight, usable only for keep costs |
| `cards.science_every` | 2 | 0..10 | 1 insight off tier-2 keeps and card plays per this many Science tags (0: off) |
| `cards.industry_every` | 2 | 0..10 | 1 credit off buildings per this many Industry tags (0: off) |
| `cards.discount_floor` | 1 | 0..5 | tag and building discounts stop at this cost; a cost printed under it stays |
<!-- /generated -->
