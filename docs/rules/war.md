# Factories, bases, war and troops

*Factories, bases, drones, war every round, the march and node loss. Terms and conventions are in [README.md](README.md).*

## Every round: production, then war at the end

**Production runs every round** after the action
phase: first **one income step** in a printed order: factories' chips and adversary troops,
cities' energy, stations' cubes, full megaprojects, then upkeep ([economy.md](economy.md)); then **growth**: extra
drones, then city growth (below). **War is fought at the end of every round** on every front, after
threats execute, so what war destroys is gone for next round's production. **War brought forward** (Casus Belli;
candidate): the front the threat's base belongs to fights twice in next round's war (their drones
cancel once, the surplus strikes twice).

## Factories

- **Ownership:** an owner cube, as stations and cities have. A facility a city builds is always **neutral**.
  A neutral factory is claimed, and an adversary one liberated, as any site is
  ([ambassadors.md](ambassadors.md)). Cards that build a factory (Prefab Plant Kit) build the
  player's own, on the card's target, a free zone in reach.
- **Output** (each round; the site table in [ambassadors.md](ambassadors.md)): a player's factory makes
  `factory_chips` (1) chip for its owner; an adversary factory makes 1 adversary troop on its hex (**locked**); a
  neutral factory makes nothing for anyone, but **feeds the cities next to it** (growth, below) and **at a Discord tip
  it arms its country's bases** (below). One opposing ambassador or agent blocks a
  factory's output while it stands. Threats build adversary factories (Mass Production, War Economy), corrupt them
  (Factory Occupation, with its cleanup) and destroy them (Siege, leaving Fallout); projects build or take them (War Plants, Asset Seizure, Hostile Takeover, Grand Offensive).
- **City growth** (candidate; each round, after the income step and the extra drones): every
  city, aligned, neutral or corrupted, puts **1 chip on itself for each unclaimed neutral factory next to it** (none
  of its own; a claimed or adversary factory feeds nothing; an agent on the city blocks nothing), holding at most
  `city_chip_cap` (3). Then each city holding `city_build_chips` (3) spends them and builds a **neutral** facility:
  whichever of factory and research station it has **fewer of next to it** (any owner's; ties: a factory), a factory
  on a free industrial zone, a station on free open land that is no zone and no base, next to the city; with none,
  within 2 hexes **in its own country**; with none there, nothing is built (it never switches kind) and its chips
  wait. An aligned city's owner picks the hex; a corrupted city's is the lowest-numbered next to an agent, else the
  lowest-numbered, as a neutral city's is. Cities gather first, then build in hex order. Claiming a factory stops it
  feeding: growth is a prize to take, and a city with no unclaimed factory beside it touches no token.

## Bases and war

**A base is a front's drone pile**: each corp has one on each of its active fronts (every printed front is
active; a 1-country game has none), with no owner and no ambassador rules beyond the drone rule below. There are no
player troops: a corp's pieces are **drones**, their own piece, which belong to the megacorp of their territory and
wait at its bases.

- **Whenever a corp makes drones, it puts 1 on each of its bases.** Bases make no drone of their own. **At a Discord
  tip** each country with at least one **unblocked neutral factory** (no agent and no player
  ambassador on it) makes drones: 1 on each of its bases, once per country however many such factories it has.
  **The drone rule:** a player ambassador on a base always stops 1 drone made there and is used
  up; a full base takes no drone and spends no ambassador; among several players' ambassadors the lowest seat's goes first.
- **Peacetime:** at most `unit_cap` drones wait at each base. Drones never protect anything.
- **Extra drones** (candidate) go past the cap: Warmongering (the Militarist's board: a base with an
  adversary troop on or next to it), at most 1 extra a base a round. No threat or project makes drones.
- **War, at the end of every round:** on each front the piles at the two bases cancel 1 for 1. Each surplus drone
  crosses, acts at once and is used up: it destroys the rival corp's factory nearest its base (not an
  adversary-owned one; ties: lower hex number), **and a node on its hex too** (the Militarist's War Takes
  Ground, `adversary.war_node`; each piece's protections apply). **Each factory destroyed in war sways `destruction_discord` (1) Discord:** runaway Discord is an
  intended loss route, which players stop by standing ambassadors on bases and neutral factories, claiming
  neutral factories and keeping the Discord track below its tip.

**Adversary troops (candidate): the march.** Troops are **few and deadly**: they come from
adversary-owned factories (each round), the stage-III event cards' Muster 1 (each country's lowest-numbered factory,
else city; no stage-I or stage-II card musters), threats (Orbital Strike, Total War)
and projects, only from adversary factories or botnets (War Plants, Botnet Conscripts). **Targets:**
player-owned research stations and factories, **and nodes** (`troop_node` 0, from the start), never cities
(corruption is the adversary's way in) or neutral or adversary-owned facilities.

In the adversary phase (`board.march`, after the corruption step) **every adversary troop
takes `troop_steps` steps** (1, so a troop needs time to arrive; Forced March's twist: more), one troop at a time, country by country and by hex number within each, in an order
fixed when the march starts. A troop a threat lands follows the troop's normal rules (no standing troops); troops a threat
lands to "march at once" (Total War) take their steps as it executes. The march stops once a seat has no node (the
loss; no troop moves or asks after it). A step is one of:

- **destroy** a target on or next to its hex (players choose which), or
- with none there, **move** one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose: a choice among bad options, as with Spirit Island's blight.

**Troops ignore ambassadors:** a march neither stops for nor removes them. Troops hit
facilities; agents meet ambassadors. The owner's own ambassador on a facility is still armor against a strike
([ambassadors.md](ambassadors.md)).

**Wall (locked).** **No troop is ever placed on the Protector's node hex** (a Muster's, a
factory's, a botnet's, a threat's or a project's: it stays in the supply; a threat's
landing, the players' choice, is offered only open hexes), and none enters it: a march goes around it. **Nor can a troop strike into it:** a
facility on a hex with the Protector's node is no target, so it stands until the node goes.
A troop already standing on a hex when the Protector places there leaves as it likes. Vigil pays 1 insight for each
of the first two that end a march next to its node each round ([ai-powers.md](ai-powers.md)). With no open way to a
target a troop stays where it is.

**A troop that strikes
is used up** (`troop_spent`), whether the target is destroyed or the hit absorbed.
Troops cross borders and never enter ocean; "nearest" and "toward" count the walk over land,
around walls (above). **Players see each hit coming:** the view draws each
troop's next target (the nearest; the arrow is solid when the hit lands in the next march). Cards remove troops, each
naming how many; a strike card's "on or next to it" (its target) counts the target itself, and the player picks which
pieces. Removing a troop sways `alignment.troop` (0).

**For new players:** [rulebook.md](../rulebook.md) §2 (sites, bases), §6 (the march), §9 (city growth) and §11
(war). Discord feeds war and war feeds Discord: keep neutral factories quiet and the Discord track low.

**This is the troops' own movement rule**; Push (Interceptor Battery, the Protector's Patrol) moves troops too
([ambassadors.md](ambassadors.md)). **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch ([README.md](README.md#off-variants)): troops never march or strike (Push still moves them).

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is local:** a seat loses nodes only where the board is overrun: an uncountered threat that prints it (the nearest
player destroys their node closest to it; Siege takes the node on its factory's hex), a site that corrupts (only a
node on its own hex), a node hex holding `corrupt_node_at` (2) agents ([adversary.md](adversary.md)) and Border War
(each player with a node on or next to a front's base destroys 1 of those). War takes a node on the hex of a factory
it destroys (War Takes Ground). No project destroys a node itself (Night Raid lands agents on node hexes, for the
2-agent rule); an adversary troop's strike (the march above). **Nothing else destroys a node:** Discord cards, the
other projects and the other event cards print board pressure instead ([alignment.md](alignment.md)); Offensive deals
a threat onto the node hex with the most agents and troops. A hit is each player's own pick of which token, absorbed
as any destruction is (below).
**No last-node safety.** Every source above takes a last node, at any stage; a player with no
nodes still loses for everyone. A spare is the tool: each player starts with `board.start_nodes` (1; extras on the
lowest-numbered free land hexes next to the start, off no track).

**A lost node (locked, as in Spirit Island).** A destroyed node, or one paid to counter a threat,
**leaves the game**: it does not go back on its track, and what it uncovered stays
uncovered.

**A lost node leaves no mark** (locked: one scar, and losing the node is punishment enough), whatever took it.
Fallout, the one scar ([adversary.md](adversary.md)), comes off by Lift Fallout or a card that says so, for no reward
but the drip and the hex back.
**Destruction.** Only war sways Discord: a factory destroyed by the drones crossing a
front sways `destruction_discord` Discord. An adversary troop's strike (in the march) sways
nothing. A Shelter card stops a destruction of its player's own nodes and facilities on the hexes in their reach (as it stands when one is struck), to the round's end,
spending nothing, and so
does the Aegis Doctrine capstone, a shelter on the Protector's own pieces once a round (the first time each round
one of its nodes or facilities would be destroyed, automatically, asking nothing). Otherwise **the piece's owner
chooses which protection to spend, or none** (seat 0 for a neutral piece): a hardening token on the hex, **the owner's own ambassador on the hex**
(armor; another player's never guards it, and a neutral site has none). Scorched Earth's node loss goes
through the same choice. A destroyed node leaves the game (above). A destroyed station
or factory takes its cube with it.
