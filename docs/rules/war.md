# Factories, bases, war and troops

*Factories, bases, drones, war every round, the march and node loss. Terms and conventions are in [README.md](README.md).*

## Every round: production, then war at the end

**Production runs every round** after the action
phase: first **one income step** in a printed order (r75, Matt's ruling 6): factories' chips and adversary troops,
cities' energy, stations' cubes, full megaprojects, then upkeep ([economy.md](economy.md)); then **growth**: extra
drones, then city growth (below). **War is fought at the end of every round** on every front, after
threats execute, so what war destroys is gone for next round's production. **War brought forward** (Casus Belli;
candidate): the front the threat's base belongs to fights twice in next round's war (their drones
cancel once, the surplus strikes twice).

## Factories

- **Ownership:** an owner cube, as stations and cities have. A facility a city builds is always **neutral** (r83).
  A neutral factory is claimed, and an adversary one liberated, as any site is
  ([ambassadors.md](ambassadors.md)). Cards that build a factory (Prefab Plant Kit) build the
  player's own, on a free zone in reach.
- **Output** (each round; the site table in [ambassadors.md](ambassadors.md)): a player's factory makes
  `factory_chips` (1) chip for its owner (r83); an adversary factory makes 1 adversary troop on its hex (**locked**); a
  neutral factory makes nothing for anyone, but **feeds the cities next to it** (growth, below) and **at a Discord tip
  it arms its country's bases** (below). One opposing ambassador or agent blocks a
  factory's output while it stands. Threats build adversary factories (Mass Production, War Economy), seize them
  (Factory Occupation: corruption, with its cleanup) and destroy them (Siege); projects build or take them (Shadow Factories, War Plants, Asset Seizure, Hostile Takeover).
- **City growth** (r83, Matt 2026-11-11, candidate; each round, after the income step and the extra drones): every
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

**A base is a front's drone pile** (137): each corp has one on each of its active fronts (every printed front is
active; a 1-country game has none), with no owner and no ambassador rules beyond the drone rule below. There are no
player troops: a corp's pieces are **drones**, their own piece, which belong to the megacorp of their territory and
wait at its bases.

- **Whenever a corp makes drones, it puts 1 on each of its bases.** Bases make no drone of their own. **At a Discord
  tip** (r80, Matt's ruling #50) each country with at least one **unblocked neutral factory** (no agent and no player
  ambassador on it) makes drones: 1 on each of its bases, once per country however many such factories it has.
  **The drone rule** (r75, Matt's ruling 8): a player ambassador on a base always stops 1 drone made there and is used
  up; a full base takes no drone and spends no ambassador; among several players' ambassadors the lowest seat's goes first.
- **Peacetime:** at most `unit_cap` drones wait at each base. Drones never protect anything (r81, friction #51).
- **Extra drones** (135, candidate) go past the cap: Warmongering (the Militarist's board: a base with an
  adversary troop on or next to it), at most 1 extra a base a round (Matt, 2026-11-06). War Footing (a project) fills every base to its cap. No threat makes drones
  (146: no special pieces).
- **War, at the end of every round:** on each front the piles at the two bases cancel 1 for 1. Each surplus drone
  crosses, acts at once and is used up: it destroys the rival corp's factory nearest its base (not an
  adversary-owned one; ties: lower hex number), **and a node on its hex too** (r83: the Militarist's War Takes
  Ground, `adversary.war_node`; each piece's protections apply). **Each factory destroyed in war sways `destruction_discord` (1) Discord:** runaway Discord is an
  intended loss route, which players stop by standing ambassadors on bases and neutral factories, claiming
  neutral factories and keeping the Discord track below its tip.

**Adversary troops (candidate): the march.** Troops come from adversary-owned factories (each round), most event
cards' Muster (each country's lowest-numbered factory, else city), the Muster's `wall_troops` landing where the
Protector stands, threats (Orbital Strike, Total War) and projects, r85 only from adversary factories or botnets
(Factory Garrisons, War Plants, Botnet Conscripts). **Targets:** player-owned
research stations and factories, **not nodes** (`troop_node`; a troop on or next to a node only stands there), never
cities (corruption is the adversary's way in) or neutral or adversary-owned facilities.

In the adversary phase (`board.march`, after the corruption step) **every adversary troop
takes `troop_steps` steps** (Forced March's twist: more), one troop at a time, country by country and by hex number within each, in an order
fixed when the march starts. A troop a threat lands follows the troop's normal rules (146: no standing troops); troops a threat
lands to "march at once" (Total War) take their steps as it executes. A step is one of:

- **destroy** a target on or next to its hex (players choose which), or
- with none there, **move** one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose: a choice among bad options, as with Spirit Island's blight.

**Troops ignore ambassadors** (r83, Matt's ruling #44): a march neither stops for nor removes them. Troops hit
facilities; agents meet ambassadors. The owner's own ambassador on a facility is still armor against a strike
([ambassadors.md](ambassadors.md)).

**Wall (candidate, Matt).** Adversary troops can't enter a hex with the Protector's node: a
march goes around it, and a Muster there stays in the supply (175: one rule for a blocked landing; a threat's
landing, the players' choice, is offered only open hexes). **Nor can a troop strike into it (Matt):** a
facility on a hex with the Protector's node is no target, so it stands until the node goes.
A troop already standing on a hex when the Protector places there leaves as it likes. The
Muster's `wall_troops` landing ([adversary.md](adversary.md)) brings troops near the Protector; Vigil pays 1 insight
for each of the first two that end a march next to its node each round ([ai-powers.md](ai-powers.md)). A landing enters a hex, so it counts; a troop made by a factory on its own hex
doesn't enter. With no open way to a target a troop stays where it is.

**A troop that strikes
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
projects, and every round a Muster 1 in each country where the Protector has a node.
Every round, in the adversary's turn,
each troop takes two steps. If a research station or factory you own is on or next to the
troop's hex, it destroys that piece and leaves the board; otherwise it walks one hex toward
the nearest such piece. So a troop two hexes from your piece hits it next round, and one
three or four hexes away a round later; the arrows on the board show where each troop is
heading. Troops never attack cities, and walk past ambassadors. To stop a hit, remove the troop with a card, harden the target, stand your own ambassador on it (armor against a strike from next door), or put the Protector's
node on it or in its way. Troops don't hit nodes.

**Sites, bases and war, for new players.** Cities, research stations and factories work alike: whoever owns one gets
its output (energy, research cubes, chips), and the adversary's make Discord, compute or raiders. Claim a neutral site,
or take one back from the adversary, with your ambassadors (5 for a city, 3 for a station or factory). One of your ambassadors standing on an adversary site shuts it off, and one of
its agents shuts off yours; your own ambassador on your piece takes a hit for it. Each border between countries has a
military base on each side, a pile of drones: every Discord tip, a country with a neutral factory free of agents and
ambassadors adds a drone to each of its bases (your ambassador on a base stops it), and at the end of every round the two piles cancel and
each drone left over destroys a factory across the border, and a node on it, swaying 1 Discord. **Cities grow:** each
unclaimed neutral factory puts a chip on each city next to it; at 3 the city builds a neutral factory or research
station beside it, whichever it has fewer of. Claim the factory and that city stops growing. So Discord feeds war and war feeds
Discord: keep neutral factories quiet and the Discord track low. **What resisted this summary:** the extra
drones (Warmongering) ignore the pile's cap, and the agent count you pay for
grows by one for each agent already on the site.

**This is the troops' own movement rule**; Push (Interceptor Battery, the Protector's Patrol) moves troops too
([ambassadors.md](ambassadors.md)). **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch ([README.md](README.md#off-variants)): troops never march or strike (Push still moves them).

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is a slide, not a cliff, and it is local (139, Matt):** a seat loses tokens a few at a time from the middle of
the game on, only where the board is overrun: an uncountered threat that prints it (146: the nearest player destroys
their node closest to it; Siege takes the node on its factory's hex), a site that corrupts (138: only a node on its
own hex), a node hex holding `corrupt_node_at` (2) agents (175, [adversary.md](adversary.md)) and Border War (each
player with a node on or next to a front's base destroys 1 of those). War takes a node on the hex of a factory it
destroys (r83, the Militarist's War Takes Ground). **Nothing else destroys a node:** Discord cards, projects and the other event
cards print board pressure instead ([alignment.md](alignment.md)); Offensive deals a threat onto the node hex with the
most agents and troops. A hit is each player's own pick of which token, absorbed as any destruction is (below).
**No last-node safety (175, Matt's ruling).** Every source above takes a last node, at any stage; a player with no
nodes still loses for everyone. A spare is the tool: each player starts with `board.start_nodes` (1; extras on the
lowest-numbered free land hexes next to the start, off no track; 177 calibrates the tool set).

**A lost node (locked, as in Spirit Island).** A destroyed node, or one paid to counter a threat,
**leaves the game**: it does not go back on its track, and what it uncovered stays
uncovered. Each lost token is a board location you can no longer act from, so getting all
your nodes out is always a tradeoff.

**Corruption markers (candidate; `node_marker`).** A node lost to anything but corruption (a threat, Border War)
leaves a corruption marker on its hex, unless the hex is a corrupted site, holds a botnet or a marker
already. Corruption leaves its own mark instead (r80, Matt's ruling #42: a site's cube, a node hex's land
corruption; [adversary.md](adversary.md)). A marked hex can't take a node. **A marker only blocks** (r80): it never
counts toward corruption and never destroys nodes. It doesn't count against the purge. **Clearing
one is worth an action (candidate):** the Clear standard action (`clear_cost` credits) takes
a marker off in reach and pays `clear_unity` Sway toward Unity and `clear_credits` credits;
Science cards clear them too, for the same reward. Clear takes a land corruption token off too (in reach,
no reward), which lets the hex take a node or factory again.
**Destruction.** Only war sways Discord: a factory destroyed by the drones crossing a
front sways `destruction_discord` Discord. An adversary troop's strike (in the march) sways
nothing (Matt). A shelter on the country stops a destruction, spending nothing, and so
does the Aegis Doctrine capstone, a shelter on the Protector's own pieces once a round (r82: the first time each round
one of its nodes or facilities would be destroyed, automatically, asking nothing). Otherwise **the piece's owner
chooses which protection to spend, or none** (r75, Matt's ruling 5; seat 0 for a neutral piece): a hardening token on the hex, **the owner's own ambassador on the hex**
(armor, 137; another player's never guards it, and a neutral site has none). Scorched Earth's node loss goes
through the same choice. A destroyed node leaves the game (above). A destroyed station
or factory takes its cube with it.
