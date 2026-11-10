# ambassadors (site model, claiming, liberating, blocking, armor, canceling, evicting, moving, Gather/Push, Convert, firewall) at RULES_REVISION 73: engine-verified rules (172 verifier)

**Pieces and sites**
- A *site* is a city, research station or factory (ambassadors.rs:53-63). A *piece* is a site, a military base, or a node; ambassadors normally sit on pieces (common.rs:114-116). A site is **neutral** (no cube), **owned/aligned** (a player's cube), or **adversary** (adversary cube) (common.rs:79-87).
- Your ambassadors come from your supply, which has no limit. You start with `board.start_ambassadors` 1 (board/mod.rs:299). Your home country's bound city starts with your cube on it, free (board/mod.rs:300-302).

**What each site makes, and what blocks it** (production.rs)
- City (city_energy, 308-370). It makes `corp_energy` energy (1). A Blackout on it stops everything. The checks run in this order:
  1. Adversary city with any player's ambassador on it: blocked, nothing happens.
  2. Adversary city otherwise: `divert_discord` 1 Discord per energy.
  3. Neutral city with an agent on it: **dark**, no sale. The agent stays.
  4. Aligned city with an agent on it: dark, the owner gets nothing, and it sways `ambassador_discord` (0) Discord per energy. The agent stays.
  5. Aligned city otherwise: its owner gets the energy, plus `aligned_bonus` (0).
  6. Neutral city otherwise: any player with a node next to it and `city_energy_credits` 5 credits may buy the energy (seat 0 asks who).
- Research station (station_cubes, 384-430):
  - Adversary station with a player ambassador on it: blocked. Otherwise it makes `station_cubes` 1 adversary compute.
  - Neutral or owned station with an agent on it: blocked. The agent stays.
  - Owned station: `owned_station_cubes` 2 cubes in the owner's color onto its country's megaproject, or onto the owner's board once that megaproject is done.
  - Neutral station: `station_cubes` 1 neutral cube onto the megaproject, or nothing once it is done.
- Factory (factory_output, 173-205). A factory that a troop occupies makes nothing.
  - Owned factory: `factory_chips` 2 chips to its owner. An agent on it blocks this.
  - Adversary factory: 1 adversary troop on its hex. A player ambassador on it blocks this.
  - Neutral factory: nothing in production. On each Discord tip it makes 1 corp troop on each base of its corp (tip_corp_troops, 48-75). An agent on it blocks that. A player ambassador on it is **used up instead** (lowest seat first). If every base of that corp is full, nothing happens and no ambassador is used.
- Blockers are never used up by blocking.
- Base: a player ambassador on a base is used up instead of each corp troop made there. A full base makes no troop and uses no ambassador (production.rs:14-31). An agent on a base does nothing.
- Lone node: an agent on a node's hex with no city, base or building costs the node's owner 1 insight next round and adds 1 adversary compute to the cheapest project. The agent stays (production.rs:434-461).
- Botnet: a player ambassador on a botnet's hex stops its compute (adversary.rs:445-461; rule in adversary.md).

**Canceling** (one agent and one ambassador both come off; no Unity; the agent goes back to the adversary's supply)
- An agent that arrives on a hex holding any player's ambassador cancels one of them, the lowest seat's (adversary.rs:1111-1119; moves.rs:81-87).
- An ambassador that arrives on agents (Move, card moves, Gather/Push, Convert) cancels one agent and is itself used up (ambassadors.rs:24-31; moves.rs:88-91).
- Troops never cancel.
- Exception: the Remove action's ambassador does not cancel (standard.rs:437-446).

**Claim** (standard action; standard.rs:343-345, 465-469; ambassadors.rs:81-87, 151-163)
- Target: a neutral site with your node on or next to it, no botnet on or next to it, and no Embargo on it (ambassadors.rs:9-13).
- Cost: `claim_city` 4, `claim_station` 3 or `claim_factory` 3 ambassadors from **your supply only**, plus 1 for each agent on the site. All are used up.
- Effect: the agents there cancel, and your cube goes on. Ambassadors already standing on the site do not help.
- Difficulty never changes these numbers.
- Claim by card (Power Purchase Agreement, Microgrid Lab Compact) works on neutral research stations only. It has the same reach, botnet and Embargo conditions. The card's energy replaces the 3 ambassadors, but you still pay 1 ambassador from your supply for each agent there, and those agents cancel (resources.rs:78-92; ambassadors.rs:132-147).

**Liberate** (standard action; standard.rs:349-351, 471-476; ambassadors.rs:91-127, 169-185)
- Target: an adversary site with your node on or next to it. Botnets and Embargo do not stop it.
- Bill: `liberate_at` 5 for a city, or `liberate_facility_at` 3 for a station or factory, plus 1 for each agent on it.
- Payment comes first from ambassadors standing on the site: yours first, then other seats' in seat order. Your supply pays the rest. The action is offered only if your supply covers that rest, which may be 0. Standing ambassadors beyond the bill stay.
- Effect: the agents on the site cancel and the adversary cube comes off, so the site turns neutral.
  - City: `free_city` 5 Unity, then `free_leaves` (0) agents land on it. Its factories keep their owners.
  - Station or factory: `free_facility` 2 Unity. A botnet on it stays.
- Liberate cards and tier-2 cleanses also take a city's adversary cube off, with `free_city` Unity, at the card's price. They **do not** remove the agents on it or use standing ambassadors (removal.rs:183-185, 214-221).

**Armor** (destroy.rs:34-103)
- When a piece would be destroyed, the first protection that applies stops it, checked in this order:
  1. A last node is safe before stage II.
  2. Shelter in that country.
  3. A corp unit at an adjacent base (troop strikes only).
  4. A hardening token.
  5. **Ambassador armor:** the node owner's ambassador on that hex, or else the ambassador of the seat that owns the site on that hex. That ambassador is used up.
  6. Aegis Doctrine.
- So one ambassador of a site's owner covers the site and any node on that hex, even another player's node.
- Neutral and adversary sites get no armor.

**Remove (evict)** (standard.rs:360, 437-446)
- Cost: `remove_ambassadors` 1 from your supply.
- Target: any hex in reach that holds agents. It does not need a piece.
- Up to `remove_count` 3 agents go back to the supply, with no Unity. Your ambassador goes onto that hex and stays.
- If more than 3 agents were there, the rest stay beside it.
- Fight triggers fire (board/mod.rs:369-370).

**Move** (standard.rs:365-374, 458-463; ambassadors.rs:215-241)
- Each Move costs 1 of your Movement this round (board printed number plus uncovered Movement spaces; ai.rs:102-104). Unused Movement is lost.
- Take 1 ambassador from your supply **or from any hex on the board**, and put it onto a piece in your reach other than the hex it came from. It arrives normally, so it cancels an agent there.

**Older card moves** (effects/ambassadors.rs)
- Move up to N (Transmission Corridor 2, Liaison Corps 4): same as Move, repeated.
- Consolidate up to N (Summit Caucus 6): pick one piece in reach, then bring up to N of your ambassadors that are in reach onto it.
- Move one with range R (Peacekeeper Rotation 2): one of your ambassadors moves to a piece within R hexes of it, with no reach limit.
- Spread: an op that no card currently uses.

**Gather / Push** (moves.rs)
- Pieces that can move: ambassadors (any seat's), agents and troops, as the card names them.
- Gather up to N: pick a land hex in reach. Then move pieces from **adjacent** hexes into it, in batches, and stop whenever you like (111-169).
- Push up to N: pick a land hex in reach. Move its pieces to adjacent land hexes of your choice (173-214).
- These hexes take no arrival:
  - ocean, for any piece;
  - a firewalled hex, for agents;
  - a hex with a Wall seat's (Protector's) node, for troops (47-49).
- Arrivals cancel as above. Troops just move.
- Strike gather (Kettle Operation 6, agents and troops; Lure Network 3, agents; Honeypot Relay 2, agents):
  - Pick the hex. It needs an adjacent hex holding a named piece.
  - Up to N named pieces come off the **adjacent** hexes, never the chosen hex itself: troops first, then agents, in hex order.
  - Each troop removed sways `alignment.troop` (0) plus the player's Unity bonuses (adversary.rs:1447-1457).
- Checkpoint Network pushes 2 when played and 3 as its Action. Patrol (1 energy) pushes up to 3 agents or troops.

**Convert** (endgame.rs:6-74; Mass Defection 5, Open Source Uprising 4, Double Agents 3, Embassy of Exiles 2)
- Target: any city, whatever its owner, that is in reach and has an agent on or next to it.
- Pick one of your nodes on or next to that city.
- Up to N agents on or next to the city go to the adversary's supply. If they are more than N, you pick the hexes; the city goes first.
- For each agent taken, one of your ambassadors arrives on the chosen node's hex. If agents are there, it cancels one.

**Firewall** (Firewall Array's Action, Air Gap; endgame.rs:77-93)
- Put a token on a land hex in reach that has none.
- While it stands, no agent is placed there: rally, creation, Corrupt pulls into it, and placements all skip it (adversary.rs:1095-1100; plan.rs:87-88, 151-156, 234-236). Agents cannot be gathered or pushed in either. Troops are unaffected.
- Every firewall comes off in the adversary's reveal phase, after income (adversary.rs:500-507).

**Other uses**
- Envoy spends `envoy_ambassadors` 1 from your supply to move 1 Discord off the track.
- Gain an ambassador costs `ambassador_credits` 20 credits.

## Player count and difficulty

Player count (1-4, one seat per country; config.rs:17, state.rs:56):
- Claim, Liberate, Remove, Move and Envoy numbers do not change with player count.
- Only in multiplayer:
  - Liberate spends other seats' ambassadors standing on the site, after the acting seat's, in seat order (ambassadors.rs:113-127).
  - An arriving agent cancels the lowest-numbered seat's ambassador (common.rs:119-121; adversary.rs:1113).
  - Bases and neutral factories use up the lowest seat's ambassador first (production.rs:22, 64).
  - Gather moves any seat's ambassadors (moves.rs:32-37).
  - A site owner's ambassador armors other seats' nodes on that hex (destroy.rs:68-70).
- The adversary's agent supply is `supply_per_country` 24 × countries (adversary.rs:177). A Remove or cancel that returns agents refills it.
- Solo (countries=1): no other seats' ambassadors exist, so all the cross-seat cases above vanish.

Difficulty (adversary.difficulty 0-4):
- The player-side numbers never change: claim_city 4, claim_station 3, claim_factory 3; liberate_at 5, liberate_facility_at 3; remove 1 for up to 3. They are read from the board config only (ambassadors.rs:67-77).
- From `corrupt_cut_at` 1 up, the adversary's Corrupt numbers drop by `corrupt_cut` [1,1,1,0] (city 4→3, station 3→2, factory 3→2, land 3). Claim no longer matches Corrupt X at d1+, although the doc says the two match.
- Agent income per board each round is `ambassador_income` [0,3,3,3,2] by difficulty. This changes how many agents arrive to cancel or block, but no ambassador rule.
- `extra_corrupted` (0) at d4 would add more adversary cities to Liberate.
- The last-node-safe armor step depends on stage, not difficulty, though d4's `tier2_early` and stage swaps shift when stage II arrives.

## Mismatches (engine vs docs/rules)

- **A liberate card liberates a city the same way as the Liberate action, for its own price (README Terms adds: 'Liberating a city takes it off the ladder: no agent stays').** (docs/rules/ambassadors.md, One site model, Liberating bullet; docs/rules/README.md Terms, 'The ladder'): engine: Liberate cards (liberate_city) and tier-2 cleanses of a city call board::liberate directly. That takes the adversary cube off and sways free_city Unity, but it does not cancel the agents on the city. The agents stay, so the now-clean city goes dark. The Liberate action (liberate_site) removes them first. (crates/aic-core/src/modules/effects/removal.rs:217 and :185; crates/aic-core/src/modules/board/ambassadors.rs:204-211 vs 172-176)
- **The ambassador the Remove action moves evicts what stands there and stays.** (docs/rules/ambassadors.md, 'An agent and an ambassador cancel' bullet): engine: It evicts only up to board.remove_count (3) agents. Any further agents stay on the hex beside the newly placed ambassador. Nothing cancels, and the hex is left 'contested'. economy.md states the cap; ambassadors.md does not. (crates/aic-core/src/modules/board/standard.rs:437-446)
- **Your own ambassador is armor: when your node, or a site you own, would be destroyed, your ambassador on its hex is used up instead.** (docs/rules/ambassadors.md, One site model, armor bullet): engine: For a node hit, the engine first uses the node owner's ambassador. If there is none, it uses the ambassador of the seat that owns the site on that hex. So another player's ambassador on their own site absorbs a hit on your node there. (crates/aic-core/src/modules/board/destroy.rs:68-75)
- **A strike removes up to N agents or troops next to the hex ('next to' = holding X or adjacent, README Terms).** (docs/rules/ambassadors.md, Moving pieces, 'A strike' bullet; docs/rules/README.md Terms 'Next to X'): engine: A strike removes pieces only from hexes adjacent to the chosen hex, never from the chosen hex itself. Honeypot Relay and Lure Network strike agents only; only Kettle Operation also takes troops. (crates/aic-core/src/modules/effects/moves.rs:119-134; content/cards.json honeypot_relay, lure_network)
- **Player ambassadors move only through cards, Remove and Move.** (docs/rules/ambassadors.md, 'Consumed or standing' bullet): engine: Any seat's ambassadors can also be moved by another player's Gather or Push (Exodus Corridors, Rally the Allies). Convert also places new ambassadors on the board. The Move action and move-up-to-N cards take an ambassador from any hex on the board, with no reach limit on the source. (crates/aic-core/src/modules/effects/moves.rs:32-37; crates/aic-core/src/modules/board/ambassadors.rs:215-238)
- **Reach is 1 for every player rule: no deck card's move goes further. Spread (Exodus Corridors) is measured from the ambassador.** (docs/rules/README.md Terms, 'Reach'): engine: Peacekeeper Rotation (move_one range 2) moves an ambassador to any piece within 2 hexes of it, with no reach check. Exodus Corridors is now a gather of up to 6 ambassadors into a hex in reach. No card uses spread_ambassadors. (crates/aic-core/src/modules/effects/ambassadors.rs:45-64; crates/aic-core/src/modules/effects/targets.rs:92-94; content/cards.json exodus_corridors, peacekeeper_rotation)

## Hard to state

- [high] **Two arrivals don't cancel: the Remove action's ambassador evicts and stays, while every other ambassador arrival on agents cancels one and is used up.**: An exception to the core cancel rule. The two cases differ only by which action moved the piece. With the 3-agent cap, an agent and an ambassador can end up standing together, which the cancel rule says never happens. Proposed: Make Remove 'remove up to 3 agents from a hex in reach, then put your ambassador there; it cancels any agent left'. Or drop the placed ambassador and have Remove just spend 1 to remove up to 3.
- [high] **Liberate uses standing ambassadors (any seat's, yours first) before your supply, but Claim uses only your supply even when your ambassadors stand on the neutral site.**: The doc calls claiming 'symmetric' with liberating, yet only one of them counts pieces on the board. A player who has gathered ambassadors onto a neutral site will expect them to pay for the Claim. Proposed: One rule for both: ambassadors standing on the site pay first, then your supply. Or neither uses standing ambassadors.
- [medium] **A botnet next to a site blocks Claim but not Liberate (and Embargo blocks Claim but not Liberate).**: The doc itself names it as the rule that resisted the summary. It is an 'unless' that a player can't predict from the board. Proposed: Let botnet and Embargo block both, or neither. Or say on the action tiles: 'Claim: no botnet on or next to it'.
- [high] **Liberate cards and cleanses remove the adversary cube but leave the agents on the city, unlike the Liberate action.**: Two routes to the same named outcome ('liberated') leave different boards. The leftover agents then make the city dark. Proposed: Any liberation of a site removes the agents on it (to the supply), whatever the route.
- [medium] **Armor: the node owner's ambassador, or else the site owner's ambassador on that hex, is used up. Its place in the absorb order is last-node-safe, Shelter, base unit (troop strikes), hardening, ambassador, Aegis.**: It needs the full ordered list to state precisely. It also covers another player's node through the site owner, which no player would predict. Proposed: 'An ambassador on a hex protects its owner's pieces there.' Drop the cross-seat coverage, and print the absorb order once in war.md.
- [medium] **A player ambassador on a neutral factory is used up instead of the Discord-tip troop. On a base it is used up instead of each corp troop. On an adversary site it blocks without being used. An agent on a neutral factory blocks it.**: The same piece blocks for free in one place and is spent in another. The table names only the agent case for neutral sites, and the 'troop rule' is a forward reference to war.md. Proposed: State one line: 'A player ambassador on a base or neutral factory stops the next corp troop there and is used up.' Put it in the site table beside the neutral-factory row.
- [low] **When an agent arrives on a hex with ambassadors of several seats, the lowest seat's is canceled. Base and factory troop consumption also takes the lowest seat's.**: This is hidden digital bookkeeping: a printed game would let the players choose, or say whose. Proposed: 'The players choose whose ambassador is canceled', or 'the newest arrival's'. State it in the cancel rule.
- [medium] **A strike takes pieces only from the hexes adjacent to the chosen hex, not the hex itself, troops first, in hex order.**: It conflicts with the house meaning of 'next to', which includes the hex. The fixed removal order is engine bookkeeping. Proposed: 'Remove up to N named adversary pieces on or next to a hex in reach, your choice.' Or keep the adjacent-only rule and print 'adjacent to' on the cards.
- [medium] **The Move action and move-up-to-N cards take an ambassador from your supply or from any hex on the board, onto a piece in reach. Move-one-with-range ignores reach.**: Movement from anywhere is a teleport. The README claims reach 1 for every player move, and a player reads 'Move' as moving across the board. Proposed: Either 'place one ambassador from your supply or from anywhere' (call it Place or Deploy), or restrict the source to your reach.
- [low] **Firewall removal: it comes off in the adversary's reveal phase after income, so it lasts 'one round'.**: The timing depends on adversary phase order. A firewall placed late in a round covers the next build and corrupt steps, and one placed earlier covers the current ones. Proposed: 'Remove your firewall at the end of the adversary turn after you placed it.' Print a round-track reminder.
- [medium] **Convert: the converted ambassadors go to the node whose reach the card used, not onto the city. Each conversion then cancels with any agent on that node hex.**: A player would expect the defectors to stand where they were. 'The node whose reach the card used' is a term found only here. Proposed: 'Each converted agent becomes your ambassador on the hex it stood on.' The board stays the same and the city's Liberate bill drops.
- [low] **The Liberate action is offered only if your supply covers the bill minus the ambassadors standing on the site (any seat's, yours first, then others in seat order). Leftover standing ambassadors stay.**: Spending other seats' pieces, the seat-order tie-break, and the affordability rule all have to be stated. Proposed: 'Liberate: remove ambassadors from the site, then from your supply, until you have removed the site's number plus 1 per agent. Players whose ambassadors are spent may refuse'. Or limit it to yours only.
