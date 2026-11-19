# Factories, bases, war and troops

*Factories, bases, drones, war every round, the march and node loss. Terms and conventions are in [README.md](README.md).*

## Every round: production, then war at the end

**Production runs every round** after the action
phase: first **one income step** in a printed order: factories' chips and adversary troops,
cities' energy, stations' cubes, full megaprojects, then upkeep ([economy.md](economy.md)); then extra
drones. **Production is read from the tracks** (r133, a table procedure: what each site makes is unchanged). Each
player's income tracks show their factories' chips and stations' cubes; the adversary's board its corrupted cities'
Discord and stations' cubes; each megaproject its neutral stations. A marker moves only when a site changes hands
(claimed, liberated, corrupted, built, destroyed) or a megaproject completes (its stations' cubes then go to their
owners' boards, a neutral one's nowhere, an adversary one's to the compute step: research, or compute with `research_at` 0). **A "blocked" chip** goes on a producing site when an opposing
piece arrives (an agent on a player's or neutral station or player's factory, a player ambassador on an adversary
site; a blackout's cities, an occupied factory) and comes off when the last one leaves: in production each track
pays what it shows less what its chipped sites make, and each adversary factory without a chip makes its troop. A neutral city is read
when it is hosted ([economy.md](economy.md)), as before. **War is fought at the end of every round** on every front, after
threats execute, so what war destroys is gone for next round's production. **War brought forward** (Casus Belli;
candidate): the front the threat's base belongs to fights twice in next round's war (their drones
cancel once, the surplus strikes twice).

## Factories

- **Ownership:** an owner cube, as stations and cities have. A facility an event card's Grow line builds is always
  **neutral** ([adversary.md](adversary.md)).
  A neutral factory is claimed, and an adversary one liberated, as any site is
  ([ambassadors.md](ambassadors.md)). Cards that build a factory (Prefab Plant Kit) build the
  player's own, on the card's target, a free zone in reach.
- **Output** (each round; the site table in [ambassadors.md](ambassadors.md)): a player's factory makes
  `factory_chips` (1) chip for its owner; an adversary factory makes 1 adversary troop on its hex (**locked**); a
  neutral factory makes nothing for anyone, but **at a Discord tip it arms its country's bases** (below). One opposing ambassador or agent blocks a
  factory's output while it stands. Threats build adversary factories (Mass Production, War Economy; Factory
  Occupation on an empty zone), corrupt them (Factory Occupation, unless a firewall or Corrupt prohibition guards it)
  and destroy them (Siege, leaving Fallout); projects build or take them (War Plants, Asset Seizure, Plant Seizure,
  Hostile Takeover, Grand Offensive). New neutral factories and stations come from the event cards' **Grow** lines
  (one a country, beside the named site holding the most agents: [adversary.md](adversary.md)); cities hold no chips.

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
else city; no stage-I or stage-II card musters), the research track's Weapons Program (Muster 2 in each country, 3
from `research_strong_at` 3; [adversary.md](adversary.md)), threats (Orbital Strike, Total War, Call-Up and others:
each prints it) and projects, only from adversary factories or botnets (War Plants, Botnet Conscripts). A Muster
lands what it prints at every level (267). **Targets:**
player-owned research stations and factories, **and nodes** (`troop_node` 0, from the start), never cities
(corruption is the adversary's way in) or neutral or adversary-owned facilities.

In the adversary phase (`board.march`, the March beat, after the Card beat) **every adversary troop
takes `troop_steps` steps** (1, so a troop needs time to arrive; Forced March's twist: more). **The troops on one hex march as one stack** (274): one stack at a time, country by country and by hex number within each, in an order
fixed when the march starts; troops already on a hex a stack enters march later on their own. A troop a threat lands follows the troop's normal rules (no standing troops); troops a threat
lands to "march at once" (Total War) take their steps as it executes. The march stops once a seat has no node (the
loss; no troop moves or asks after it). A stack's step:

- while a target is on or next to its hex, one troop of the stack **destroys** one (players choose which; a strike
  stays one troop's, and uses it up), then
- the troops left **move** together one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose, once for the stack: a choice among bad options, as with Spirit Island's blight. With more than one
  step (Forced March), the stack takes each step together, so a troop left after a strike walks on from there.

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

**For new players:** [rulebook.md](../rulebook.md) §2 (sites, bases), §6 (the march) and §11
(war). Discord feeds war and war feeds Discord: keep neutral factories quiet and the Discord track low.

**This is the troops' own movement rule**; Push (Checkpoint Network, the Protector's Patrol) moves troops too
([ambassadors.md](ambassadors.md)). **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch ([README.md](README.md#off-variants)): troops never march or strike (Push still moves them).

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is local:** a seat loses nodes only where the board is overrun: an uncountered threat that prints it (the nearest
player destroys their node closest to it; Siege takes the node on its factory's hex), a site that corrupts (only a
node on its own hex), an arrival leaving `corrupt_node_at` (2) agents on a node hex ([adversary.md](adversary.md)) and Border War
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
(armor; another player's never guards it, and a neutral site has none). A destroyed node leaves the game (above). A destroyed station
or factory takes its cube with it.
