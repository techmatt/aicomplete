# Factories, troops and node loss

*Factories, production from the tracks, the adversary march, Wall and node loss. Terms and conventions are in [README.md](README.md).*

## Every round: production

**Production runs every round** after the action
phase, as **one income step** in a printed order: factories' chips,
cities' energy, stations' cubes, full megaprojects, then upkeep ([economy.md](economy.md)). **Production is read from
the tracks** (r133, a table procedure). Each
player's income tracks show their factories' chips and their cities' energy. **The adversary's corrupted sites
produce in its own Income beat**, one output each ([adversary.md](adversary.md)). Until a
country's megaproject completes, every player's and neutral station there puts its cube onto that
megaproject, read off the map; once it completes, a player's stations move to their income tracks (research cubes on
their board) and a neutral one's cube goes nowhere. A marker moves only when a site changes hands (claimed, liberated,
corrupted, built, destroyed), a megaproject completes or, for a city's energy, a node on or next to it is placed or
lost ([economy.md](economy.md)). **A "blocked" chip** goes on a producing site when an opposing
piece arrives (an agent on a city, a player's or neutral station or a player's factory, a player ambassador on an
adversary site; a blackout's cities) and comes off when the last one leaves: in production each track
pays what it shows less what its chipped sites make, once, at its own step (chips with factories, cubes with
stations, not again at upkeep), and an adversary site under a chip makes no output.

**There is no war** (r139): no fronts, no military bases, no megacorp armies and no War step in the round. Megacorp
conflict is card content only (Border War, Casus Belli, some Discord cards), each card doing what it prints.

## Factories

- **Ownership:** an owner cube, as stations and cities have. A facility a site-opening line builds is always
  **neutral** ([adversary.md](adversary.md)).
  A neutral factory is claimed, and an adversary one liberated, as any site is
  ([ambassadors.md](ambassadors.md)). Cards that build a factory (Prefab Plant Kit) build the
  player's own, on the card's target, a free zone in reach.
- **Output** (each round; the site table in [ambassadors.md](ambassadors.md)): a player's factory makes
  `factory_chips` (1) chip for its owner; an adversary factory makes 1 agent, pushed along its zone's printed
  arrow onto its city ([adversary.md](adversary.md), [board.md](board.md)), and no troop; a
  neutral factory makes nothing for anyone. One opposing ambassador or agent blocks a
  factory's output while it stands (a troop occupying an adversary factory blocks it too). Threats build adversary factories (Mass Production, Arsenal; Factory
  Occupation on an empty zone), corrupt them (Factory Occupation, unless a firewall or Corrupt prohibition guards it)
  and destroy them (Siege, leaving Fallout); projects build or take them (War Plants, Asset Seizure, Plant Seizure,
  Hostile Takeover, Grand Offensive). New neutral factories and stations come from site-opening lines
  (Factory Drive, Shell Labs, the research row's icons: [adversary.md](adversary.md)); cities hold no chips.

## Adversary troops

**The march (candidate).** There are no player troops. Adversary troops are **few and deadly**: they come from
the stage-III event cards' Muster 1 (each country's lowest-numbered factory,
else city; no stage-I or stage-II card musters), threats (Orbital Strike, Total War, Call-Up and others:
each prints it) and projects, only from adversary factories or botnets (War Plants, Botnet Conscripts). No
factory's round output and no research slot makes one ([adversary.md](adversary.md)). A Muster
lands what it prints at every level (267). **Targets:**
player-owned research stations and factories, **and nodes** (`troop_node` 0, from the start), never cities
(corruption is the adversary's way in) or neutral or adversary-owned facilities.

In the adversary phase (`board.march`, the March beat, after the Card beat) **every adversary troop
takes `troop_steps` steps** (1, so a troop needs time to arrive; Forced March's twist: more). **The troops on one hex
march as one stack** (274): one stack at a time, country by country and by hex number within each, in an order
fixed when the march starts; troops already on a hex a stack enters march later on their own. A troop a threat lands
follows the troop's normal rules (no standing troops); troops a threat
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

**Wall (locked).** **No troop is ever placed on the Protector's node hex** (a Muster's,
a threat's or a project's: it stays in the supply; a threat's
landing, the players' choice, is offered only open hexes), and none enters it: a march goes around it. **Nor can a
troop strike into it:** a
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

**For new players:** [rulebook.md](../rulebook.md) §2 (sites) and §6 (the march). Adversary factories feed
agents to their cities: liberate or block one and its agents stop coming.

**This is the troops' own movement rule**; Push (Checkpoint Network, the Protector's Patrol) moves troops too
([ambassadors.md](ambassadors.md)). **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch ([README.md](README.md#off-variants)): troops never march or strike (Push
still moves them).

## Node loss

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is local:** a seat loses nodes only where the board is overrun: an uncountered threat that prints it (the
player of the country it was dealt in destroys their node closest to its tokens; Siege takes the node on its
factory's hex), a site that corrupts (only a
node on its own hex), an arrival leaving `corrupt_node_at` (2) agents on a node hex ([adversary.md](adversary.md)),
Border War (each player with a node on or next to a factory destroys 1 of those nodes) and an adversary troop's
strike (the march above). No project destroys a node itself (Night Raid lands agents on node hexes, for the
2-agent rule). **Nothing else destroys a node:** Discord cards, the
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

**Destruction.** No destruction sways Discord: a card that destroys and sways prints both. A Shelter card stops a
destruction of its player's own nodes and facilities on the hexes in their reach (as it stands when one is struck),
to the round's end, spending nothing, and so
does the Aegis Doctrine capstone, a shelter on the Protector's own pieces once a round (the first time each round
one of its nodes or facilities would be destroyed, automatically, asking nothing). Otherwise **the piece's owner
chooses which protection to spend, or none** (seat 0 for a neutral piece): a hardening token on the hex, **the
owner's own ambassador on the hex**
(armor; another player's never guards it, and a neutral site has none). A destroyed node leaves the game (above). A
destroyed station or factory takes its cube with it.
