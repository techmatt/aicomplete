# ai-boards: engine-verified rules (172 verifier)

**Your AI board (rules revision 73)**

Each player plays one AI board. Seats get boards in the order Builder, Protector, Researcher, and the list starts over after that (`ai.boards` [0,1,2]). Every board has: a special rule; printed income; one printed tag; Movement; two node tracks of 7 spaces each (economy and card-play, 14 node tokens in all); building slots; starting credits; and 8 powers (content/ais.json; ai.rs:39-41, 113-116).

| | Builder | Protector | Researcher |
|---|---|---|---|
| Starting credits (`ai.start_credits`) | 40 | 46 | 40 |
| Starting energy (`ai.start_energy`) | 3 | 3 | 3 |
| Building slots (`ai.slots`) | 5 | 4 | 3 |
| Credits each round | 15 | 13 | 11 |
| Insight each round | 5 | 5 | 5 |
| Energy each round | 1 | 2 | 1 |
| Printed tag | Industry | Security | Science |
| Printed Movement | 1 | 1 | 2 |

**Setup.** Each player puts 1 node on their home country's printed node hex. That token comes off the economy track, so economy space 1 (+1 insight) starts uncovered. Nothing on the card-play track is uncovered at setup (board/mod.rs:293-299; ai.rs:45-52).

**Income.** Each round you get your board's printed income plus everything printed on your uncovered spaces. Insight arrives at the start of the round. Credits, energy, chips and cubes arrive in upkeep (phase 16) (economy.rs:131-138, 189-243). A space's income starts with the next collection: a credit space placed during the actions pays in that same round's upkeep, and an insight space pays from the next round on. Unspent insight is lost when the next round starts. The Researcher is the exception (Patience, below).

**Placing a node**
1. You place nodes only with a power or a card. There is no growth phase.
2. You choose which track to take the next token from. You are only offered tracks whose next space you can afford.
3. You choose the hex. It must be a land hex next to one of your nodes, with no node of any player, no corruption marker and no land corruption on it. A corrupted site or a hex with agents on it is allowed. If you have no nodes left, you may use any such hex in your home country (node.rs:35-59).
4. You pay the space's printed energy cost: 0 on spaces 1-3, then 1, 2, 3 and 5 on spaces 4-7. A node power pays this on top of its own cost. A card pays only this cost, and a card that prints "free" pays nothing (node.rs:92-98, 123-182).
5. The space is uncovered for good. A one-time effect, such as the Researcher's free station, happens at once and can use the node you just placed for its reach.

Uncovered spaces can give you income, the keep upgrade (tier-2 keeps cost 1 insight instead of 2), a tag, Movement, a free research station, or a capstone. A tag on your board, printed or revealed by a space, counts like a tag on your cards for requirements, discounts and per-tag effects, but it can never be sacrificed (tableau.rs:64-85).

**Losing a node.** The node leaves the board for good, and its space stays uncovered. A corruption marker goes on its hex unless the hex already has a marker, a Botnet or the adversary's cube. A shield on the hex goes with the node unless a player-owned, uncorrupted facility shares the hex (node.rs:195-215). While the adversary card in play is stage I (`board.last_safe_stage` 2), nothing removes your last node (node.rs:187-189).

**Capstones.** Space 7 of each track is a capstone. It is a standing rule from the moment you uncover it (tableau.rs:14-16).
- **Full Capacity** (Builder): 1 chip each upkeep for every 2 Industry tags you have. Board tags count.
- **Sanctuary** (Protector): when a hex with your node is corrupted, your node is not destroyed (adversary.rs:1210).
- **Open Archive** (Researcher): 1 research cube each upkeep.
- **Master Plan** (Builder): your buildings cost 2 credits less, down to the discount floor.
- **Aegis Doctrine** (Protector): once each round, it stops one destruction of any piece, anyone's, in a country where you have a node. It works only on a destruction that nothing else stopped first, and it is used up then (destroy.rs:139-151; resets at round start, cards/mod.rs:304).
- **Open Science** (Researcher): in upkeep, you hand out 2 insight from the supply to other players, one at a time. It reaches them at the start of the next round. In a solo game it goes to you (economy.rs:163-185).

**Powers**
- You use powers in the action phase. Each one works once a round (ai.rs:165-199, 387-394).
- A power is offered only when you can pay its resources and its tags, and when its effect has something to act on. For a node power, that means a token left, an affordable next space after paying the power, and a legal hex.
- **Order of payment:** you pay the resources, the power counts as used, you sacrifice the tags, then the effect happens (ai.rs:412-441).
- **Sacrifice:** to pay a tag, discard a program from your play area, or a building together with its whole stack. Events can't be sacrificed. What you can sacrifice is fixed when you use the power. You then pick cards one at a time, and only cards that still help pay are offered.
- **How tags pay:** every tag on a discarded card pays, its own kind first and then "any". Tags left over are wasted (sacrifice.rs:233-404).
- **Rush Build:** if any sacrifice would still leave you a building you can play, you are offered only those sacrifices (ai.rs:429-431).

Deploy is on every board: 1 energy, place 1 node.

Builder:
- Mint: 3 credits → 1 chip.
- Buy In: 1 energy + 6 credits → place 1 node.
- Scrap: 1 Industry → 2 chips.
- Overcharge: 1 Power → 2 energy.
- Rush Build: 1 Industry + 1 energy → play a building from your hand without paying its credits. You still pay its insight and other costs, and you must meet its requirements.
- Focus: 5 credits → 1 insight.
- Procure: 2 chips → draw 2.

Protector:
- Bulwark: 1 energy → put a hardening token on a facility in reach.
- Accord: 1 Diplomacy + 1 energy → place 1 node.
- Stand Down: 1 Security → 3 credits.
- Recovery: 1 Security → clear a corruption marker in reach. Like the standard action, this also sways Unity 2 (`clear_unity`).
- Shelter: 1 Security + 1 energy → shelter any one country not already sheltered. It doesn't have to be in reach.
- Debrief: 1 Diplomacy → draw 2.
- Patrol: 1 energy → push up to 3 agents or troops off one hex in reach. Each goes to a land hex next to it, and they can go to different hexes. A troop can't be pushed onto a Protector node. An agent can't be pushed onto a firewall. An agent pushed onto a player's ambassador removes both (moves.rs:46-72, 173-210).

Researcher:
- Fieldwork: 1 energy → draw 3.
- Peer Review: 3 research cubes → place 1 node. The space's energy cost still applies.
- Grant: 3 credits → 1 cube in your color on any megaproject that isn't complete.
- Field Station: 1 Industry → a free research station on an open, non-industrial hex in reach.
- Audit: 1 Cyber → remove 3 compute from one adversary project.
- Particle Run: 1 Power + 1 energy → 1 antimatter.
- Synthesis: no cost → discard a card from your hand to gain 2 insight.

"In reach" means your node's hex or a hex next to it (common.rs:193-196).

**Special rules**
- **Breadth (Builder):** 5 slots instead of 4 or 3. That is all it does; there is no other effect.
- **Wall (Protector):** adversary troops can't enter a hex with your node, land on it, or be pushed onto it. They can't strike anything on it either: the node, or a facility on that hex. This stops troops only. War, threats and corruption can still destroy things there (troops.rs:213-233, 270-272; adversary.rs:1310-1321). Troops path around walled hexes. A troop already standing on one walks off.
- **Wall pressure:** in each round's build (phase 4), 1 troop (`adversary.wall_troops`) lands in every country where you have a node. It goes on the lowest-numbered factory hex (anyone's) that has no Protector node. If there is none, it goes on the lowest-numbered such city. It marches in phase 7 of the same round (adversary.rs:1057-1072, 1323).
- **Vigil (Protector):** gain 1 insight when a marching troop stands next to your node and that hex holds something the troop would strike if your Wall weren't there. At baseline that means a player-owned, uncorrupted factory or station, since troops never strike nodes (`board.troop_node` 4). Vigil is checked before each of the troop's steps and pays at most once per troop per march (troops.rs:146-169, 238-250).
- **Patience (Researcher):** at the start of the round, your unspent insight carries over, minus all the insight other players gave you this round. The given insight counts as spent last (economy.rs:189-218).

**Movement.** Each round you get your board's Movement plus 1 for each uncovered Movement space: economy space 4 on every board, and the Researcher's card-play space 2. Max: Builder 2, Protector 2, Researcher 4. Each Movement is one free Move. A Move takes one of your ambassadors, from your supply or from anywhere on the board, and puts it on a hex in reach that holds a piece (a city, base, facility or node). It can't stay on the hex it is already on. Unused Movement is lost at cleanup. A new space counts as soon as it is uncovered (ai.rs:102-104; standard.rs:189-191, 365-374, 453-458). Cards that move ambassadors are separate moves and don't use your Movement.

**Shelter, hardening and shields**
- **Shelter:** a sheltered country takes no destruction of any piece until cleanup (phase 19) (board/mod.rs:229-231, 502-514).
- **What stops a destruction, in order:** last-node safety, then Shelter, then a corp unit at an adjacent base (troop strikes only), then a hardening token on the hex, then your own ambassador on the hex, then Aegis Doctrine. The first one that applies stops it and is used up (destroy.rs:34-103).
- **Hardening:** a token can go only on a player-owned, uncorrupted factory or station. "Harden N" means N different facilities. Separate harden actions can stack more tokens on the same facility (resources.rs:108-122; destroy.rs:156-158).
- **Shield:** a Science card's shield is the same token, placed on any hex holding your node that has no token yet (science.rs:24-39; targets.rs:87-89).

## Player count and difficulty

**Seating by player count.** Seats get boards in the order Builder, Protector, Researcher, and the list starts over (`ai.boards` [0,1,2]; ai.rs:113-116).
- 1 player: Builder.
- 2 players: Builder and Protector.
- 3 players: all three boards.
- 4 players: a second Builder (seat 3).

Each board keeps its own starting credits and slots whichever seat plays it. Home country is seat number mod the number of countries (common.rs:161-163). Since the Researcher appears only at 3+ players and the Protector at 2+, Wall, Vigil, Wall pressure, Patience and Open Science aren't in play at lower counts unless the config is changed.

**Solo.** Open Science's insight goes to the player themself and isn't counted as 'received', so Patience would keep it (economy.rs:166, 211). This only matters if a solo config seats the Researcher. 'Another player' card gifts also resolve to yourself solo.

**Wall pressure.** 1 troop per country holding a Protector node, regardless of player count. Only one Protector exists at 4 players or fewer.

**Megaprojects.** Adversary project cost scales with player count (adversary.rs project_cost), while Audit's 3 compute is fixed. Audit is relatively weaker at higher counts.

**Difficulty.** Nothing in this area changes with difficulty: board numbers, tracks, powers, Movement and `wall_troops` are the same at every level. Two thresholds depend on the adversary card's stage, not on difficulty:
- Last-node safety holds only while the card is stage I (`board.last_safe_stage` 2).
- Troops never strike nodes at baseline (`board.troop_node` 4), so Vigil pays only for facilities.

## Mismatches (engine vs docs/rules)

- **Protector, for new players: 'where you stand is a barrier and a facility under you is safe'** (docs/rules/ai-boards.md, The Protector, for new players): engine: The Wall only stops adversary troops (entering, landing, pushes, strikes). War (front surplus), threats, corruption and Discord effects still destroy a facility on the Protector's node hex. destroy_piece has no Wall check. (crates/aic-core/src/modules/board/troops.rs:213-218; crates/aic-core/src/modules/board/destroy.rs:34-103)
- **Vigil is an Effect 'because it happens in the march (phase 7) ... the insight pays this round's keeps and plays'; and a shelter 'never meets' the march** (docs/rules/ai-powers.md, Insight conversions (Protector, Vigil) and Shelters, hardening and shields): engine: Vigil also fires during a threat's immediate march (march_now, run when threats execute in phase 17). Insight gained there comes after the actions and is lost at the next round start. A Shelter set during the actions does cover that march's strikes. (crates/aic-core/src/modules/effects/new_ops.rs:125-133; crates/aic-core/src/modules/board/troops.rs:133-169)
- **Full Capacity: '1 chip each round per 2 Industry tags on your cards'** (docs/rules/ai-boards.md node_track table, space 7 (content/ais.json capstone text)): engine: Counts the board's printed Industry tag and any revealed Industry tags as well as card tags (IncomePerTag reads tag_counts). The Builder's printed Industry always counts toward the pairs. (crates/aic-core/src/modules/economy.rs:139-146; crates/aic-core/src/modules/cards/tableau.rs:68-72)
- **Aegis Doctrine: 'Each round, stop the first destruction in a country you're in'** (docs/rules/ai-boards.md node_track2 table, Protector space 7): engine: One use per round in total, not one per country. It stops the first destruction not already stopped by last-node safety, Shelter, a base unit, a hardening token or armor. It covers anyone's piece, neutral factories hit by war included. 'In' means having a node in the country. (crates/aic-core/src/modules/board/destroy.rs:76-78, 139-151; crates/aic-core/src/modules/cards/mod.rs:304)

## Hard to state

- [high] **A node placement costs two prices: the power's cost plus the uncovered space's energy cost (Peer Review is '3 cubes, no energy' yet still pays the space's energy)**: Two numbers in two places for one action. The Researcher text says 'no energy', which is false from space 4 on. Proposed: Print one total on each track space and make every node power cost only its tag or currency, or say on every node power 'plus the space's cost'.
- [high] **Vigil: gain insight when a troop is next to your node and that hex holds something it would strike but for the Wall; once a troop a march, checked before each step**: The 'stop' is counterfactual and doesn't depend on what the troop actually does. Once-per-troop bookkeeping is required, and it also fires in threat marches after the actions, where the insight is wasted. Proposed: 'When a troop ends its march next to your node's facility, gain 1 insight', or 'gain 1 insight for each troop that lands in your country (Wall pressure)'.
- [high] **Aegis Doctrine: stops the first otherwise-unstopped destruction of any piece in any country where you have a node, once a round**: It needs a precedence list (Shelter, base unit, hardening, armor, then Aegis). It is spent on other players' pieces and neutral factories, and 'a country you're in' is undefined. Proposed: 'Once a round, when one of your nodes or your facilities would be destroyed, it isn't.' The player chooses when, so it isn't burned automatically.
- [medium] **Patience: carry over unspent insight minus all insight other players gave you this round**: Players must track received insight separately, and the rule assumes gifted insight is spent last. Proposed: Give the Researcher a carry-over cap (for example 'keep up to 3 unspent insight'), or have gifts arrive as separate tokens that are discarded at round end.
- [medium] **Wall pressure: each build, 1 troop lands on the lowest-numbered factory hex (anyone's, not a Protector node hex), else the lowest-numbered such city, in each country with a Protector node**: It is hidden on the adversary side, with a tie-break by hex number and a fallback. The player would not predict it from the board. Proposed: Print it on the Protector board ('Each round, 1 troop lands in each country with your node') and let the Protector or the adversary card choose the hex.
- [medium] **Movement: moves can come from the supply or from any of your ambassadors anywhere on the board, onto a piece in reach, not the hex it is on; card moves are separate**: 'Movement' reads like distance, but each point is one teleport into reach. The 'add to it' cards are actually separate effects. Proposed: Rename to 'Free Moves: N each round. Each puts one of your ambassadors (supply or board) on a piece next to your node.' State that card moves are extra.
- [medium] **Board tags count like card tags for everything except sacrifice; Full Capacity's 'on your cards' then counts board tags too**: It is an exception rule, and the capstone wording contradicts it. Proposed: Reword capstones and per-tag cards to 'your tags'. Keep one line: 'Tags on your board count but can't be sacrificed.'
- [low] **Sacrifice: tags pay their own kind first, then 'any'; extra tags are wasted; a building goes with its whole stack; Rush Build offers only sacrifices that leave a building playable**: Several sub-clauses, plus a Rush Build-only filter. Proposed: No board power needs 'any' and every cost is 1 tag, so state it as 'discard a card (or stack) showing that tag'. Drop the 'any' paragraph from the player rulebook.
- [low] **A lost node never returns to its track, and what it uncovered stays; the last node is safe only in stage I; a marker is left unless the hex already has one, a Botnet or adversary cube**: Three clauses with exceptions, and a stage condition the player must look up. Proposed: 'A destroyed node is removed from the game and leaves a corruption marker. Its space stays uncovered. In stage I your last node can't be destroyed.' Drop the marker exception and let the markers not stack.
- [low] **Hardening: only on a player-owned uncorrupted facility, N different facilities per effect but stackable across effects; a shield is the same token on your node hex with no token yet**: There are two names for one token, different stacking rules for each, and the token absorbs any destruction on the hex. Proposed: One token, 'Shield': any hex with your node or a player facility, max 1 per hex, absorbs one destruction there.
- [low] **Shelter targets any country (no reach), lasts to cleanup, and stops every destruction there before hardening**: No reach unlike every other Protector power. It isn't obvious that it shields other players' pieces too. Proposed: Keep it, but print 'any country' on the power. That fits the cooperative intent.
