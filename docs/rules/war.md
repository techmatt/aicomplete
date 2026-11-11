# Factories, bases, war and troops

*Factories, bases, drones, war every round, the march and node loss. Terms and conventions are in [README.md](README.md).*

## Every round: production, then war at the end

**Production runs every round** after the action
phase: first **one income step** in a printed order (r75, Matt's ruling 6): factories' chips and adversary troops,
cities' energy, stations' cubes, full megaprojects, then upkeep ([economy.md](economy.md)); then **growth**: extra
drones, then new factories (below). **War is fought at the end of every round** on every front, after
threats execute, so what war destroys is gone for next round's production. **War brought forward** (Casus Belli;
candidate): the front the threat's base belongs to fights twice in next round's war (their drones
cancel once, the surplus strikes twice). Declaration of War (an ongoing project): each round, the front with the most
drones fights twice.

## Factories

- **Ownership:** an owner cube, as stations and cities have. A factory a city builds is its city's owner's: an
  aligned city's is that player's, a corrupted city's the adversary's, and a neutral city's **neutral** (r75, Matt's
  ruling 4). A neutral factory is claimed, and an adversary one liberated, as any site is
  ([ambassadors.md](ambassadors.md)). Cards that build a factory (Prefab Plant Kit) build the
  player's own, on a free zone in reach. An owned factory keeps its owner if its city
  changes hands later.
- **Output** (each round; the site table in [ambassadors.md](ambassadors.md)): a player's factory makes
  `factory_chips` (2) chips for its owner; an adversary factory makes 1 adversary troop on its hex (**locked**); a neutral
  factory makes nothing, except that **on each Discord tip it makes 1 drone**. One opposing ambassador or agent blocks a
  factory's output while it stands. Threats build adversary factories (Mass Production, War Economy), seize them
  (Factory Occupation: corruption, with its cleanup) and destroy them (Siege); a project builds them (Shadow Factories).
- **Growth** (each round, after the income step and the extra drones): every city builds one factory **on a
  free industrial zone next to it** (no facility, no land corruption), or none if it has none (r75). A neutral or aligned city's zone is the players' choice (an aligned city's owner, else seat 0); a
  **corrupted** city's is the lowest-numbered on or next to an agent, else the lowest-numbered. Cities build in hex
  order, blocked or not. Factories destroyed in war regrow this way.

## Bases and war

**A base is a front's drone pile** (137): each corp has one on each of its active fronts (every printed front is
active; a 1-country game has none), with no owner and no ambassador rules beyond the drone rule below. There are no
player troops: a corp's pieces are **drones**, their own piece, which belong to the megacorp of their territory and
wait at its bases.

- **Whenever a corp makes drones, it puts 1 on each of its bases** (so a neutral factory's tip drone is 2 at 3 and 4
  countries). Bases make no drone of their own. **The drone rule** (r75, Matt's ruling 8): one player ambassador
  always stops 1 drone and is used up. On a base it stops a drone made there; on a neutral factory at a Discord
  tip it stops 1 of that factory's drones (the players choose which base's, when it can't stop all), so each
  ambassador there stops one. A full base takes no drone and spends no ambassador; among several players' ambassadors
  the lowest seat's goes first. An agent on a neutral factory blocks its tip drones outright.
- **Peacetime:** at most `unit_cap` drones wait at each base. A drone at a base **absorbs
  an adversary-troop strike** on a facility on or next to the base: 1 drone per hit.
- **Extra drones** (135, candidate) go past the cap: Warmongering (the Militarist's board: a base with an
  adversary troop on or next to it) and Arms Race (ongoing). A base gains at most 1 extra a round from the two
  together (Matt, 2026-11-06). War Footing (a project) fills every base to its cap. No threat makes drones
  (146: no special pieces).
- **War, at the end of every round:** on each front the piles at the two bases cancel 1 for 1. Each surplus drone
  crosses, acts at once and is used up: it destroys the rival corp's factory nearest its base (not an
  adversary-owned one; ties: lower hex number), **and any node on its hex**, guarded like any node (r75). **Each factory destroyed in war sways `destruction_discord` (1) Discord:** runaway Discord is an
  intended loss route, which players stop by standing ambassadors on bases and neutral factories, claiming
  neutral factories and keeping the Discord track below its tip.

**Adversary troops (candidate): the march.** Troops come from adversary-owned factories (each round), most event
cards' Muster (each country's lowest-numbered factory, else city), the Muster's `wall_troops` landing where the
Protector stands, and projects and threats (Armored Columns, Orbital Strike, Total War). **Targets:** player-owned
research stations and factories, **not nodes** (`troop_node`; a troop on or next to a node only stands there), never
cities (corruption is the adversary's way in) or neutral or adversary-owned facilities.

In the adversary phase (`board.march`, after the corruption step) **every adversary troop
takes `troop_steps` steps** (Forced March's twist: more), one troop at a time, country by country and by hex number within each, in an order
fixed when the march starts. A troop a threat lands follows the troop's normal rules (146: no standing troops); troops a threat
lands to "march at once" (Total War) take their steps as it executes. A step is one of:

- **remove an ambassador** (175, Matt's ruling: ambassadors are non-combatants): a troop that starts a step on a hex
  with an ambassador, any player's, removes one (players choose whose) instead of moving or striking, so it can
  remove two in a march; ambassador armor still answers a strike from next door ([ambassadors.md](ambassadors.md)), or
- **destroy** a target on or next to its hex (players choose which), or
- with none there, **move** one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose: a choice among bad options, as with Spirit Island's blight.

**Wall (candidate, Matt).** Adversary troops can't enter a hex with the Protector's node: a
march goes around it, and a Muster there stays in the supply (175: one rule for a blocked landing; a threat's
landing, the players' choice, is offered only open hexes). **Nor can a troop strike into it (Matt):** a
facility on a hex with the Protector's node is no target, so it stands until the node goes.
A troop already standing on a hex when the Protector places there leaves as it likes. The
Muster's `wall_troops` landing ([adversary.md](adversary.md)) brings troops near the Protector; Vigil pays 1 insight
for the first that ends a march next to its node each round ([ai-powers.md](ai-powers.md)). A landing enters a hex, so it counts; a troop made by a factory on its own hex
doesn't enter. With no open way to a target a troop stays where it is.

So a troop moves twice, or moves then destroys, or destroys at once. **A troop that strikes
is used up** (`troop_spent`), whether the target is destroyed or the hit absorbed.
Troops cross borders and never enter ocean; "nearest" and "toward" count the walk over land,
around walls (above). **Players see each hit coming:** the view draws each
troop's next target (the nearest; the arrow is solid when the hit lands in the next march):
an adversary factory within two hexes of your
station or factory is a known danger. Cards remove troops, each card naming how many; a strike card's "on or next to a
hex in reach" counts the hex itself, and the player picks which pieces, nothing moving.
Removing a troop sways `alignment.troop` (0 since r38: stopping its harm already pays).

**For new players.** Adversary troops are the adversary's raiders. They appear on adversary
factories every round, from most event cards (Muster: on each country's lowest-numbered factory, else city), some threats and
projects, and every round 1 lands on an adversary or neutral factory in each country where the Protector has a node.
Every round, in the adversary's turn,
each troop takes two steps. If a research station or factory you own is on or next to the
troop's hex, it destroys that piece and leaves the board; otherwise it walks one hex toward
the nearest such piece. So a troop two hexes from your piece hits it next round, and one
three or four hexes away a round later; the arrows on the board show where each troop is
heading. Troops never attack cities. **Ambassadors don't fight** (175): a troop that starts a step on a hex with an
ambassador (any player's) removes one, the players choosing whose, instead of moving or striking, so in two steps it
can remove two. To stop a hit, remove the troop with a card, keep a drone at a military base next to the target,
harden the target, stand your own ambassador on it (armor against a strike from next door), or put the Protector's
node on it or in its way. Troops don't hit nodes.

**Sites, bases and war, for new players.** Cities, research stations and factories work alike: whoever owns one gets
its output (energy, research cubes, chips), and the adversary's make Discord, compute or raiders. Claim a neutral site,
or take one back from the adversary, with your ambassadors (5 for a city, 3 for a station or factory). One of your ambassadors standing on an adversary site shuts it off, and one of
its agents shuts off yours; your own ambassador on your piece takes a hit for it. Each border between countries has a
military base on each side, a pile of drones: every Discord tip, each neutral factory adds a drone to each of its
corp's bases (each of your ambassadors standing there stops one), and at the end of every round the two piles cancel and
each drone left over destroys a factory across the border, swaying 1 Discord. So Discord feeds war and war feeds
Discord: keep neutral factories quiet and the Discord track low. **What resisted this summary:** the extra
drones (Warmongering, Arms Race) ignore the pile's cap, and the agent count you pay for
grows by one for each agent already on the site.

**This is the troops' own movement rule**; Push (Interceptor Battery, the Protector's Patrol) moves troops too
([ambassadors.md](ambassadors.md)). **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch ([README.md](README.md#off-variants)): troops never march or strike (Push still moves them).

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is a slide, not a cliff, and it is local (139, Matt):** a seat loses tokens a few at a time from the middle of
the game on, only where the board is overrun: an uncountered threat that prints it (146: the nearest player destroys
their node closest to it; Siege takes the node on its factory's hex), a site that corrupts (138: only a node on its
own hex), a node hex holding `corrupt_node_at` (2) agents (175, [adversary.md](adversary.md)), war that destroys a
factory (nodes on its hex and, by `war_reach` 0, none next to it) and Border War (each player with a node on or next
to a front's base destroys 1 of those). **Nothing else destroys a node:** Discord cards, projects and the other event
cards print board pressure instead ([alignment.md](alignment.md)); Offensive deals a threat onto the node hex with the
most agents and troops. A hit is each player's own pick of which token, absorbed as any destruction is (below).
**No last-node safety (175, Matt's ruling).** Every source above takes a last node, at any stage; a player with no
nodes still loses for everyone. A spare is the tool: each player starts with `board.start_nodes` (1; extras on the
lowest-numbered free land hexes next to the start, off no track; 177 calibrates the tool set).

**A lost node (locked, as in Spirit Island).** A destroyed node, or one paid to counter a threat,
**leaves the game**: it does not go back on its track, and what it uncovered stays
uncovered. Each lost token is a board location you can no longer act from, so getting all
your nodes out is always a tradeoff.

**Corruption markers (candidate; `node_marker`).** A lost node leaves a corruption marker on
its hex, unless the hex is a corrupted site, holds a botnet or already holds a marker (the
cap on the loop: a marker adds no corruption where there is some). A marked hex can't take a
node. On a city that is not corrupted it counts 1 toward its Corrupt number (not when gathered from next door); since no node stands
there, a marker never destroys nodes itself. It doesn't count against the purge. **Clearing
one is worth an action (candidate):** the Clear standard action (`clear_cost` credits) takes
a marker off in reach and pays `clear_unity` Sway toward Unity and `clear_credits` credits;
Science cards clear them too, for the same reward. Clear takes a land corruption token off too (in reach,
no reward), which lets the hex take a node or factory again. Markers come only from destroyed nodes,
so the reward can't be farmed: a token lost costs far more than a Sway.

**Destruction.** Only war sways Discord: a factory destroyed by the drones crossing a
front sways `destruction_discord` Discord. An adversary troop's strike (in the march) sways
nothing (Matt). A shelter on the country stops a destruction, spending nothing. Otherwise **the piece's owner
chooses which protection to spend, or none** (r75, Matt's ruling 5; seat 0 for a neutral piece): a drone at a
base next to it (adversary strikes only), a hardening token on the hex, **the owner's own ambassador on the hex**
(armor, 137; another player's never guards it, and a neutral site has none), or the Aegis Doctrine capstone (once a
round, the Protector's own nodes and facilities, when the Protector chooses). War's node on a destroyed factory's
hex, and Scorched Earth's, go through the same choice. A destroyed node leaves the game (above). A destroyed station
or factory takes its cube with it.
