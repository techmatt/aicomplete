# war: engine-verified rules (172 verifier)

**Where it sits in the round** (configs/baseline.json:365-384): Sway → event → threats dealt → adversary build (event Deploy, Protector landing, ongoing projects) → corrupt → natural corruption → **march** → reveal → cards and the action phase → **production** (`board.bases`, then `board.produce`) → upkeep → threats execute → **war** → cleanup. A Discord tip can come after any phase or action (alignment.rs `after`), including straight after war.

**Factories**
- Each factory carries an owner cube: a player's, the adversary's, or none (neutral). Once built, a factory keeps its owner even if the city that grew it changes hands.
- **Output, every production** (production.rs:173-203), in board order:
  - A factory occupied by a troop makes nothing. (No current content does this.)
  - A player's factory gives its owner `factory_chips` = 2 chips. An agent on it blocks this.
  - An adversary factory puts 1 adversary troop on its own hex. A player ambassador on it blocks this. Wall does not stop it, because the troop is made on the hex rather than entering it.
  - A neutral factory makes nothing in production.
- **Growth, right after output** (production.rs:213-272): every city builds one factory, cities in board order. Each later city sees the zones the earlier ones took.
  - A clean or aligned city builds on a free industrial zone (one with no building) adjacent to it. Adjacent can mean across a border. An aligned city's owner picks the zone; otherwise the players pick. The factory goes to an aligned city's owner. A clean city's factory goes to the player with the most nodes on or next to the zone; on a tie the players pick among the tied; with no nodes there it is neutral.
  - A corrupted city builds anywhere in its own country. It takes the lowest-numbered free zone that has an agent on or next to it, else the lowest-numbered free zone. The factory is the adversary's, and any corruption markers on that zone come off.
  - A city with no such zone builds nothing. Factories lost in war grow back this way.

**Bases (troop piles)**
- Every front has one military base on each side. Bases have no owner. A 1-country game has none. At 2 countries each corp has 1 base; at 3 and 4 countries each has 2 (map.rs:158-233).
- **Cap:** a base holds at most `unit_cap` = 2 corp troops, except where a rule says "past its cap".
- **Troop rule** (production.rs:14-31): when a base would get a troop and a player ambassador stands on it, that ambassador is used up and no troop is made, one ambassador per troop. A full base makes no troop, so no ambassador is used up there. If several players have ambassadors on the base, the lowest seat's goes first (common.rs:119).
- **Discord tip** (production.rs:48-74, alignment.rs:407-410), checked for each factory that is neutral (not the adversary's, no player owner):
  1. An agent on it: nothing happens.
  2. Every base of its corp is full: nothing happens, and no ambassador is used up.
  3. A player ambassador on the factory: one ambassador is used up and no troops are made.
  4. Otherwise the corp adds 1 troop to each of its bases, using the troop rule at each base.
- **Extra troops each production** (production.rs:117-167): Arms Race (ongoing project) adds 1 to every base past its cap. Warmongering (Militarist board) adds 1 past its cap to a base with an adversary troop on or next to it. A base gets at most 1 extra a production, whatever the sources.
- **War Footing** (project, when it resolves) fills every base up to its cap, using the troop rule (new_ops.rs:661-676).
- **Shielding:** when an adversary troop strikes a facility, a corp troop at a base on or next to the struck hex is removed instead (destroy.rs:476-483). This works only against adversary troops.

**War, at the end of every round** (troops.rs:21-104), fronts in order (3 countries: 0-1, 0-2, 1-2; 4 countries: 0-1, 1-2, 2-3, 3-0):
1. Both bases on the front are emptied. The bigger pile's surplus is the difference between the two piles.
2. Each surplus troop strikes once. It hits the factory in the rival country nearest its base, by straight hex distance with ties to the lower hex number. That can be a neutral factory or a player's factory, never the adversary's. Wall does not protect against war.
3. The hit can be absorbed like any destruction (see Destruction below). Absorbing costs the troop, and the next troop aims at the same factory.
4. A factory destroyed this way:
   - sways `destruction_discord` = 1 Discord;
   - takes the node on its hex (`war_reach` = 0). That node loss cannot be absorbed and is stopped only by the Early fragility rule.
5. With no factory left to hit, the remaining surplus is lost.
6. Some fronts fight more than once:
   - **Casus Belli** (threat): the one front that the threat's base belongs to fights twice in the next round's war.
   - **Declaration of War** (ongoing): the front with the most corp troops fights twice; on a tie, the first front.
   - Fighting twice means the piles cancel once and each surplus troop strikes twice. Both together make 3 strikes per troop.

**The march** (troops.rs:114-208), in the adversary phase after natural corruption:
- Each adversary troop takes `troop_steps` = 2 steps, plus 1 more under Forced March.
- Troops go one at a time, in board order (country, then printed hex number), as placed when the march starts. A troop that moves later in the order never goes twice.
- **Each step:**
  - If a target is on or next to the troop, it strikes it (the players pick which target), and the troop is used up (`troop_spent` = 1) whether or not the hit is absorbed.
  - Otherwise it moves 1 hex toward the nearest target, counting the walk over land and around Wall hexes. Where several moves are equally good, the players pick.
  - With no path to any target, it stays.
- **Targets:** player-owned research stations and factories. Not cities, neutral or adversary facilities, and not nodes (`troop_node` = 4 means nodes are never targets).
- **Timing:** a troop 0-2 hexes from a target hits it in this march; 3-4 hexes away, in the next.
- **Strikes sway nothing.** Removing a troop with a card sways `alignment.troop` = 0.
- **Where troops come from:**
  - adversary factories, in production;
  - event Deploy lines and the Protector landing, in the build: `wall_troops` = 1 per country where the Protector has a node, on that country's lowest-numbered factory (anyone's), else its lowest-numbered city, never a Wall hex;
  - projects and threats.

**Wall** (the Protector's innate):
- Adversary troops cannot enter a hex with the Protector's node, cannot land on one, and cannot strike anything on it. A facility there is not a target at all.
- A troop already on such a hex walks off normally.
- Vigil pays at most once per troop per march (troops.rs:157-169).

**Destruction** (destroy.rs:458-527): checked in this order, the first that applies saves the piece and is used up:
1. Early fragility, for a node: while the round's adversary card is stage I (`last_safe_stage` = 2), a player's last node cannot be destroyed (node.rs:187-189).
2. A Shelter on the country.
3. A corp troop at a base on or next to the hex (adversary troops only).
4. A hardening token on the hex.
5. The owner's own ambassador on the hex: for a node, the node owner's, or else the hex's site owner's.
6. Aegis Doctrine: once a round, for any piece in a country where its holder has a node. It is spent automatically.

Otherwise the piece goes and the owner's cube goes with it.

**Lost nodes** (node.rs:195-215):
- A lost node leaves the game.
- Hardening on that hex goes too, unless a player facility still stands there.
- It leaves a corruption marker (`node_marker` = 1), unless the hex already has corruption, a botnet or a marker. A marked hex cannot take a node.
- Nodes are destroyed only by:
  - war;
  - corruption of their own hex;
  - Deploy-style landings that print "destroy";
  - threat node lines (never a last node, team.rs:96-99, 115);
  - Border War (an event line, which can take a last node at stage II or later).

## Player count and difficulty

**Player count.** One player per country (board.md:7). Seats map to AI boards [Builder, Protector, Researcher], with seat 3 wrapping back to Builder (ai.rs:113-116; configs/baseline.json:157-161).

**Solo (1 country, Builder only):**
- No fronts, so there are no bases and no war.
- Neutral factories make no tip troops.
- Arms Race, Warmongering, War Footing, Declaration of War and Casus Belli do nothing on the board.
- No base shielding.
- Border War's node line has no base to read.
- No Protector, so no Wall and no `wall_troops` landing.
- The march, adversary factories, Deploy and growth all still run.

**2 countries:**
- One front, 1 base per corp, so a neutral factory's tip makes 1 troop.
- The Protector is seated: 1 troop lands on each board where it has a node.

**3 countries:**
- Three fronts (0-1, 0-2, 1-2), 2 bases per corp, so a tip makes 2 troops per neutral factory.
- One ambassador on that factory stops both.

**4 countries:**
- A ring of four fronts (0-1, 1-2, 2-3, 3-0), 2 bases per corp.
- Still only one Protector.
- War resolves front by front in that order.

**Difficulty:**
- From `adversary.board_difficulty_at` = 2: each event Deploy lands 1 more troop per board, and the adversary gains 4 compute per uncountered threat (adversary.rs:306-345; Militarist difficulty line).
- At difficulty 4: `extra_corrupted` (0 in baseline, so no change) and the stage swap (stage I minus 1 card, stage III plus 1 card). This shortens stage I, and with it the rounds when a last node is safe.
- At difficulty 3 and up: `stage2_removed` = 2 shortens stage II.
- Forced March (a stage-II Militarist event) gives every troop +1 step that round.
- No war or production number (`unit_cap` 2, `factory_chips` 2, `destruction_discord` 1, `troop_steps` 2, `war_reach` 0, `wall_troops` 1) changes with difficulty or player count.

## Mismatches (engine vs docs/rules)

- **Casus Belli: "the fronts next to the threat's base fight twice in next round's war"** (war.md, Every round section, "War brought forward"): engine: Only the single front whose base pair contains the threat's base fights twice; each base belongs to exactly one front (place_bases gives each (country, rival) pair its own hex). (crates/aic-core/src/modules/effects/new_ops.rs:91-98; crates/aic-core/src/map.rs:198-231)
- **Corp troops come only from Discord tips at neutral factories and the extra-troop rules (Warmongering, Arms Race); "No threat makes corp troops"** (war.md, Bases and war: "Whenever a corp makes troops", "Extra corp troops"): engine: The War Footing project (content/projects.json war_footing) also fills every base to its cap when it resolves (corp_troops fill, when now), applying the troop rule. war.md never names it. (crates/aic-core/src/modules/effects/new_ops.rs:661-676)
- **Every troop on the board was there through an action phase except those built in production, placed by an uncountered threat, or by Armored Columns in the build** (war.md, The march: "Players see each hit coming"): engine: The build also lands troops from the event card's Deploy lines (plus 1 per board at difficulty 2 and up) and the Protector landing (wall_troops), both after the action phase and before the march. Declaration of War's Deploy 3 also lands troops when it resolves. (crates/aic-core/src/modules/adversary.rs:334-345, 435, 1057-1072)
- **A corrupted city builds at a free zone in its country "next to an agent if there is one"** (war.md, Factories, Growth): engine: It prefers a zone with an agent on it or next to it (next_to counts the hex itself), lowest number among those, then the lowest-numbered free zone. (crates/aic-core/src/modules/board/production.rs:220-230; crates/aic-core/src/modules/common.rs:133-135)
- **Troops march "in hex-number order"** (war.md, The march): engine: Order is board index: country by country, then the printed number within each country. Hex numbers repeat across countries, so 'hex-number order' alone is ambiguous. (crates/aic-core/src/modules/board/troops.rs:124-126; crates/aic-core/src/map.rs:75-90)
- **The troop rule: a player ambassador standing where a troop would be made is used up instead of the troop** (war.md, Bases and war, troop rule): engine: The doc never states three conditions the engine applies. A full base makes nothing and uses no ambassador. A neutral factory whose corp's bases are all full uses no ambassador. An agent on a neutral factory blocks the tip before any ambassador is checked. When several players have ambassadors there, the lowest seat's is used. (crates/aic-core/src/modules/board/production.rs:18-21, 54-68; crates/aic-core/src/modules/common.rs:118-121)

## Hard to state

- [medium] **Troop rule at full bases: an ambassador is used up only if a troop would really have been made (145), and one ambassador on a neutral factory cancels the troops for all of its corp's bases**: It needs two exceptions (full base, all bases full) and a lowest-seat tie rule. The 'one ambassador, two troops' leverage at 3-4 countries is not visible from the board. Proposed: Print it as: 'A Discord tip: each neutral factory with no agent on it adds 1 troop to each of its corp's bases that has room. An ambassador on the factory stops all of them; one on a base stops that one. Either ambassador is discarded only if it stopped a troop.'
- [high] **War hits can be absorbed and the troop is still spent, but the node on a destroyed factory's hex is lost with no absorption; Aegis Doctrine spends itself automatically on any piece in its countries, including neutral factories in war**: The absorption ladder has six steps (last-node safety, Shelter, base troop for adversary strikes only, hardening, own ambassador with a site-owner fallback, Aegis). The war node bypasses all but the first, so the reader meets an exception inside an exception. Proposed: Make the war node loss absorbable like any destruction, or say once: 'war's node loss can't be prevented.' Let Aegis's holder choose whether to use it.
- [high] **Early fragility (last node safe only while the adversary card is stage I) and threats never taking a last node, while war and Border War can take a last node from stage II**: There are three different last-node protections depending on source and stage. Border War is an event line, not a threat, so the 'never a last node' rule doesn't cover it. The doc's 'a slide, not a cliff' is then false for war and Border War. Proposed: One rule: 'Nothing destroys a player's last node.' Or: 'Only stage-I cards protect a last node.' Either way, the same for every source.
- [medium] **Wall: no entering, landing or striking into a Protector-node hex, but a factory there makes troops in place, a troop already there walks off normally, and war ignores Wall**: It takes three exceptions to explain what Wall does not stop. Proposed: State Wall as 'adversary troops treat this hex as ocean and never target what is on it'. Add one line that war (corp troops) ignores Wall.
- [low] **War targeting uses straight hex distance; the march uses walking distance over land around Walls**: There are two meanings of 'nearest' in one chapter, and a player measuring war by walking distance could pick the wrong factory. Proposed: Use one measure for both, or print 'nearest, counting hexes, ocean included' on the war rule.
- [medium] **Factory growth order: cities build one at a time in board order, the players pick zones for clean and aligned cities, and corrupted cities take a zone anywhere in their country**: The order matters when cities compete for zones and is never stated. Corrupted growth reaches far from the city while clean growth must be adjacent, and the ownership rule for a clean city's factory (most nodes on or next to the zone, players pick on a tie) is a separate sub-rule. Proposed: Print the order ('corrupted cities first, then the rest, by hex number'). Let a corrupted city also build only adjacent to itself, or explain why it reaches.
- [medium] **Bases shield facilities from adversary strikes, but war empties every base each round, so a base holds troops during the march only if a Discord tip or War Footing came after the last war**: The defensive use of bases depends on timing (tips arrive after any phase, extras at production after the march), which a player wouldn't predict. Proposed: Say directly: 'Bases are emptied by war at round end; troops a tip adds afterwards stay to guard during the next march.' Or move the extra troops to before the march.
- [low] **Casus Belli / Declaration of War doubling: the piles cancel once, the surplus strikes twice; ties for 'most corp troops' go to the first front**: 'Fights twice' suggests two full battles. The tie-break for the front is hidden. Proposed: Print 'each surplus troop on that front strikes twice'. Number the fronts on the board for ties.
- [low] **Protector landing: wall_troops land on the lowest-numbered factory of the country, which may be a player's factory, and strike it in the same round's march**: It reads as 'pressure for Wall', but it can land straight on and destroy a player's own factory with no warning beyond the board. Proposed: Land on the lowest-numbered adversary or neutral factory, else city.
