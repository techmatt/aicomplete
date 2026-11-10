# setup-board: engine-verified rules (172 verifier)

## The board

- **One country per player, 1 to 4.** The number of countries always equals the number of players (`countries` in the config; baseline 2), and seat *s* starts in country *s* (config.rs:17-19, rules.rs:336-344, common.rs:161). Country *i* is always printed piece *i*: Horizon, Cintamani, Onyx, Biomia, in that order (content/map.json). Each piece is a 4x4 block of 16 hexes, numbered 1-16 row by row (map.rs:83-98). The map is 16 hexes per player. The layout for each player count is fixed in content/map.json, and there is no layout for 5 or more (map.rs:67-70).
- **Hex kinds.** City, open land and ocean are printed (content.rs:308-316). Every piece prints 3 cities, at most 2 ocean hexes, 4 industrial zones (open hexes next to one of its cities), 1 empty station site, and the hexes for its setup pieces (tests/map.rs:25-90). A city next to ocean is coastal, and some pieces also print a city as coastal (map.rs:121-126). Every piece ends up with 2 coastal cities at 4 countries. Nothing is built on ocean.
- **Adjacency** works across country borders (map.rs:112-119).
- **Fronts are printed for each layout.** A front is a long edge two countries share. With 2 countries there is one front (0-1). With 3 there are three fronts (every pair). With 4 the fronts form a ring, 0-1, 1-2, 2-3 and 3-0, and the two pairs that meet only at the center are not fronts (content/map.json `fronts`). At 3 and 4 countries every country has two fronts. With 1 country there are no fronts and no bases.
- **Military bases (placed at setup, not printed).** Each country gets one base for each of its fronts. A base goes on an open hex of that country that touches the other country's land. It cannot go on a station hex (the botnet station or the neutral station), an industrial zone, an empty station site, or the starting-node hex. Base hexes are picked in this order (map.rs:178-216):
  1. Within each country, the front with the fewest candidate hexes picks first. If two fronts tie, the one facing the lower-numbered country picks first.
  2. Each front takes its lowest-numbered candidate hex not already taken.
  
  A base hex stops counting as open land, so no station or factory can be built on it (map.rs:220-224; common.rs:109). A base is a corp's troop pile, not a site. It starts with no troops on it (tests/setup.rs:71-74).

## Setup, step by step (the engine's module order)

**1. The board (board/mod.rs:272-304), in each country:**
1. Put an adversary cube on the first `board.start_corrupted` (1) cities in the piece's printed corruption order. At difficulty 4, `adversary.extra_corrupted` (0 in the baseline) more get one. A piece prints only 2 cities in that order, so at most 2 can start corrupted (mod.rs:274-285).
2. Put a research station on the printed station hex, with an adversary cube on it. This is the corrupted station.
3. Put a neutral research station on the printed neutral-station hex. It sits next to the starting node, and no botnet is next to it in any layout (tests/map.rs:4-22).
4. Put the two printed setup factories on their industrial zones. They start with no cube (neutral).

**Then each player (mod.rs:293-303):**
1. Place 1 node on the printed starting hex of your country. It counts as the first token taken off your node track (the track counter is set to 1), and you pay nothing for it.
2. Take `board.start_ambassadors` (1) ambassador into your supply.
3. **Bound city.** Put your cube on the clean city of your country nearest your starting node, counting straight-line hex distance. Ties go to the lower hex number. This happens after the corrupted cities are placed (node.rs:63-71). The bound city is an ordinary aligned city: it gives you its `board.corp_energy` (1) energy each production, plus `aligned_bonus` (0) (production.rs:340-344).

**2. The adversary (adversary.rs:360-401):**
1. The adversary's supply is `adversary.supply_per_country` (24) agents times the number of countries: 48 with 2 players.
2. Put 1 botnet on each country's corrupted research station.
3. Build the event deck. Shuffle each stage's cards separately, deal `stage_cards` (4/4/3) of each stage (adjusted for difficulty), and stack them with stage I on top. Turn the top card face up as the next card (adversary.rs:369-381, 906-915). The round-1 event phase makes it the current card.
4. Set the adversary's compute income and agent income from the difficulty's entry in `compute_income` and `ambassador_income`, plus any income lines printed on the adversary board. Fill each country's agent pool with one round of agent income.
5. Shuffle the project deck and deal it into the adversary board's project slots, following the slot tags.

**3. Threats:** each tier's threat deck is shuffled (threats/mod.rs:384-392).

**4. Alignment:** the Unity and Discord tracks start at 0, and the Unity and Discord decks are shuffled (alignment.rs:487-505).

**5. AI boards (ai.rs:380-385):**
- Seat *s* plays the board at position s mod 3 in `ai.boards` (0 Builder, 1 Protector, 2 Researcher) (ai.rs:113-116).
- Each player takes `ai.start_credits` for their board (Builder 40, Protector 46, Researcher 40).
- Each player takes `ai.start_energy` (3) energy.
- A board's printed credit production (Builder 15, Protector 13, Researcher 11) is not handed out at setup. It is income paid in each upkeep (economy.rs:136).

**6. Cards:** the tier-1 and tier-2 decks are shuffled (cards/mod.rs:279-295). No player has cards at setup. The opening hand is dealt in round 1:
- 8 tier-1 cards (`opening_deal`), of which you keep 4 free (`opening_free`) and more at 5 credits each.
- 2 tier-2 cards (`opening_deal_tier2`), each kept for 1 insight.

**Round 1 starts with the adversary's turn.** It plays the event, its Rally and Deploy, its Corrupt step and natural corruption before any player acts (baseline `round` order). A bound city can be corrupted before your first action (comment in tests/setup.rs:95-96).

## Research stations, megaprojects, converters

- **Each country's corp has one megaproject: an 8-cube card.**
- **Station output, each production (production.rs:384-428):**

| Station | No opposing piece on it | Opposing piece on it |
|---|---|---|
| Player's | `owned_station_cubes` (**2**) cubes in the owner's color | No output: an agent blocks it |
| Neutral | `station_cubes` (**1**) neutral cube | No output: an agent blocks it |
| Adversary's | `station_cubes` (1) adversary compute instead of cubes | No output: a player ambassador blocks it |

- **Where the cubes go.** Until the country's megaproject completes, station cubes go onto it. Afterward, a player's station puts its cubes on its owner's board as research cubes, kept between rounds. A neutral station's cube is lost.
- **Production order** (production.rs:300-305):
  1. Cities make energy.
  2. Stations produce.
  3. An agent on a lone node drains 1 of its owner's insight into adversary compute.
  4. Megaprojects are checked for completion.
  
  Production comes before upkeep, the threats and war.
- **Completion.** Countries are checked in piece order. A megaproject holding at least its size in cubes, of any color, completes. Every cube on it pays and is then spent, overflow included. Player cubes pay their owners the printed player payout, one payout per cube, seats in order. Neutral cubes pay nothing. Each adversary cube would pay the printed adversary payout, but no rule or card places one (production.rs:464-506). The megaproject is then complete for the rest of the game.
- **Buying cubes.** The Researcher's Grant, Field Grant (only in a country where you own an uncorrupted station) and Moonshot Grant put cubes in your color onto an incomplete megaproject (targets.rs:44-51; resources.rs:95-106).
- **Converters.** In the action phase, a completed megaproject's converter can be used by any player with a node on a city of that country, owned or not. Use it as often as you can pay. The exception is Onyx's converter, which each player can use once a round (mod.rs:412-435). A converter that would do nothing is not offered (mod.rs:430-433). Every player can also always turn 1 research cube into `cube_insight` (1) insight, with no limit (standard.rs:395, 425).
- **Building a station.** It costs `station_chips` (2) chips and goes on an open, empty, non-industrial hex in your reach (standard.rs:126, 335-339). Claiming a neutral station takes `claim_station` (3) ambassadors.
- **Defended stations.** An effect that flips a player's station takes it only when `station_flip_at` (2) agents and no player ambassador stand on it, and those agents are used up. Otherwise 1 agent lands there, and a player ambassador already on the station cancels that agent (adversary.rs:1362-1381).

## Player count and difficulty

**Player count.** Countries always equal players, 1 to 4 (config.rs:17-19; Map::build fails at 5 or more). The map is 16 hexes per player, and pieces are taken in order: Horizon, then Cintamani, Onyx, Biomia.
- **Fronts and bases.** With 1 player there are no fronts and no bases, so no war happens. 2 players have 1 front (2 bases). 3 players have 3 fronts (each country 2 bases). 4 players have a ring of 4 fronts (each country 2 bases; countries 0-2 and 1-3 are not fronts).
- **Coastal cities** depend on the layout, because ocean adjacency changes. Each piece has 2 at 4 players.
- **AI boards** are taken in order Builder, Protector, Researcher, then Builder again (seat s mod 3, ai.rs:113-116):
  - 1 player: Builder.
  - 2 players: Builder and Protector.
  - 3 players: all three.
  - 4 players: a second Builder.
- **Starting credits** depend on the board: Builder 40, Protector 46, Researcher 40. Starting energy is 3 for everyone, and every player gets 1 ambassador and 1 node.
- **Adversary supply** is 24 agents per country: 24, 48, 72 or 96.
- **Unity and Discord tips** are per player (`tip_per_player` 3); that belongs to the alignment area.
- **Solo-specific code** exists only in card triggers and effects that count 'others' (cards/triggers.rs:91; economy.rs:166; effects/mod.rs:71), not in the board setup.

**Difficulty** (`adversary.difficulty`, baseline 0):
- At level 4 or higher, `extra_corrupted` (0) more cities start corrupted, capped at 2 a country (board/mod.rs:275-277).
- The event deck changes by level (adversary.rs:256-260): level 3 or higher adds `stage1_added` (0) stage-I cards and removes `stage2_removed` (2) stage-II cards. At `stage_swap_at` (4) or higher, `stage1_removed` (1) stage-I card is swapped for `stage3_added` (1) stage-III card. The deck is 11 at d0 to d2, and 9 at d3 and d4.
- Adversary income comes from the per-level lists `compute_income` [3,4,18,24,13] and `ambassador_income` [0,3,3,3,2] (adversary.rs:115-119). So at d0 the agent pools start empty.
- At level 4 the Discord deck loses `discord_removed` (1) card, and the removed cards count as tips already made (alignment.rs:500-509).
- From `board_difficulty_at` (2), the adversary board's own difficulty line applies (adversary.rs:305-310).
- Nothing else in board setup (stations, factories, nodes, bound city, megaprojects) depends on difficulty.

## Mismatches (engine vs docs/rules)

- **Each station makes `station_cubes` research cube a round, in production, in its owner's color: a player's or neutral** (docs/rules/board.md, Research cubes and the corps, 'Stations' bullet): engine: A player-owned station makes `board.owned_station_cubes` (2); only a neutral station makes `station_cubes` (1). An adversary station makes `station_cubes` compute. (crates/aic-core/src/modules/board/production.rs:413-414)
- **Two stations a country fill an 8-cube card in round 4** (docs/rules/board.md, paragraph after Bought and carded cubes): engine: At setup each country has one neutral station (1 cube a round) and one adversary station, which makes compute rather than cubes. So untouched, a megaproject fills only in round 8. Two neutral stations take 4 rounds. A claimed station plus the neutral one make 3 a round and fill the card in 3 rounds. (crates/aic-core/src/modules/board/production.rs:395-421; board/mod.rs:286-288)
- **An adversary cube, which only a card can put there now, pays the card's adversary payout** (docs/rules/board.md, Completion bullet; the printed megaproject texts in content/map.json ('each adversary cube 1 Sway to Discord', 'An adversary cube made here afterward is lost')): engine: No op or rule writes an adversary cube to a megaproject. The only writer to a megaproject is a player color (ProjectCube) or a station's player or neutral color. The adversary count is only read and reset, so the adversary payout and its printed text never apply. (crates/aic-core/src/modules/board/production.rs:471,481,499-501; effects/resources.rs:101)
- **Each player starts with ... ai.start_credits credits, ai.start_energy energy, and their AI board's credit production** (docs/rules/board.md, Setup, second bullet): engine: Setup grants only start_credits and start_energy. The board's credit production (Builder 15, Protector 13, Researcher 11) is an income paid each upkeep, not a setup grant. (crates/aic-core/src/modules/ai.rs:380-385; economy.rs:136)
- **Each base goes on an open hex that touches the other country's land and holds no setup piece or industrial zone; the front with the fewest candidates picks first** (docs/rules/board.md, Fronts paragraph): engine: The engine also rules out the printed empty station site and the starting-node hex, neither of which holds a piece at setup. Fronts tied on candidate count pick in order of the lower facing country index. The doc gives no tie rule. (crates/aic-core/src/map.rs:190-198, 208)
- **Each country's first `start_corrupted` cities get an adversary cube (more at difficulty 4 with extra_corrupted)** (docs/rules/board.md, Setup, first bullet): engine: The count comes from the piece's printed `corrupted` list, which holds only 2 cities. So start_corrupted plus extra_corrupted is silently capped at 2, although the parameter allows up to 3. (crates/aic-core/src/modules/board/mod.rs:285; content/map.json corrupted lists; board/mod.rs:45)

## Hard to state

- [high] **Station output differs by owner: a player's station makes 2, a neutral station 1, an adversary station 1 compute; each goes to the megaproject before completion and to the owner's board after (lost if neutral)**: It has three owner cases, a before/after-completion split, and a 'lost' exception. A player cannot predict from the board why their claimed station makes double. Proposed: Print the output on the station token by color: player 2 cubes, neutral 1 cube, adversary 1 compute. Then state one line: 'Cubes go onto this country's megaproject; once it is done, onto your board (a neutral station's are lost).'
- [medium] **Onyx's converter is once a round; the others are as often as you can pay**: It is an exception in one card out of four, and the docs already flag it as resisting the summary. Proposed: Either make every converter once a round per player, or make Onyx's unlimited with a higher price (for example 2 cubes for 3 chips). At minimum, print 'once a round' prominently on that card only.
- [high] **Military base placement: open hex touching the other country, not on a station, zone, station site or start hex; the front with the fewest candidates picks first, lowest number**: It is an algorithm, not a rule a player would follow at the table. It has a priority order and an unstated tie-break. Bases are not printed because neighbors change with player count. Proposed: Print the base hexes in a setup diagram for each player count (four small diagrams), or print a base icon with a player-count marker on the tiles. Drop the algorithm from the rulebook.
- [medium] **Bound city: the clean city nearest your starting node by hex distance, ties to the lower hex number, chosen after corruption**: It depends on setup order and difficulty (extra corrupted cities), plus a tie-break. A player cannot see it until the corruption is placed. Proposed: Print a 'home city' marker on each piece, using a city that is never in the corruption list. With the current pieces only the third city qualifies, so check that distance is acceptable.
- [high] **Adversary cubes on a megaproject pay the adversary payout; the printed text says 'each adversary cube ...' and 'an adversary cube made here afterward is lost'**: It refers to a piece that no rule creates, so a player will look for how it happens. Proposed: Delete the adversary payout and the 'adversary cube ... lost' sentence from the megaproject cards until some effect puts adversary cubes there.
- [medium] **Converter access needs a node on a city of that country (any city, owned or not); a node elsewhere in the country does not count**: It is a condition unlike standard reach (within 1 hex) and unlike 'a country you're in'. The doc already lists it as an exception to reach. Proposed: Use 'a country you're in' (any node there), or state it on the card as 'with your node on a city of this country'.
- [low] **Round 1 begins with the adversary's turn (event, Rally, Corrupt), before players act, so a bound city can be lost before your first action**: Setup appears complete, but the adversary acts first. A player would not predict losing the bound city before acting. Proposed: State in setup that the adversary takes the first turn, or skip Corrupt in round 1.
- [low] **start_corrupted plus extra_corrupted capped at the 2 printed corruption-order cities**: The cap is hidden in content. Proposed: Print the corruption order (1st, 2nd) on the city hexes and state 'at most 2'.
