# Factories, troops and node loss

*Factories, production from the tracks, adversary troops (garrison, guarding, Strike, Redeploy), Wall and node loss. Terms and conventions are in [README.md](README.md).*

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
  factory's output while it stands. Threats build adversary factories (Mass Production, Arsenal; Factory
  Occupation on an empty zone), corrupt them (Factory Occupation, unless a firewall or Corrupt prohibition guards it)
  and destroy them (Siege, leaving Fallout); no threat makes a troop. Projects build or take them (War Plants, Asset
  Seizure, Plant Seizure, Hostile Takeover, Grand Offensive). New neutral factories and stations come from
  site-opening lines (Factory Drive, Shell Labs, the research row's New factory icon: [adversary.md](adversary.md));
  cities hold no chips.

## Adversary troops

There are no player troops. Adversary troops are **late and significant**: the players see them coming and must
plan and invest to meet them (Matt, 2026-11-19). **They garrison, guard and strike, and never move on their own.**

**Where troops come from.** Only these make one: the research row's two **Garrison** slots (4 and 6: 1 troop a
country), the projects **Botnet Conscripts** (each country's lowest-numbered botnet becomes 2 troops) and **War
Plants** (every adversary factory makes 1 troop on its own hex; a player ambassador on it stops it), and the
Discord card **Firestorm** at strength III (1 troop a country). No threat, no event card and no factory's round
output makes one.

**Garrison.** A new troop goes, in its country, onto the lowest-numbered corrupted factory; with none, the
lowest-numbered corrupted city; with none, the lowest-numbered city. Nobody chooses. It stays on its hex until a
Redeploy, a player's Push or Gather ([ambassadors.md](ambassadors.md)), or a removal.

**Guarding.** A corrupted site (city, research station or factory) with a troop on or next to it **can't be
liberated**: not by the Liberate action, and not by any card that liberates or frees a site (it is no target for
Rescue, Hostile Buyout, Great Liberation, Reclamation Crew or Back-Channel Talks, and Unity's Reclamation leaves
its cube). **The answer is to deal with the troop first, then liberate:** remove it (the standard action Remove a
troop, the boards' troop powers in [ai-powers.md](ai-powers.md), troop-removal cards, Unity's Peacekeepers and
Disarmament Talks), or move it away with a Push or Gather (Checkpoint Network, the Protector's Patrol). A troop on
or next to one of your ambassadors is in your reach for anything that targets troops (r142), so the ambassador
you moved onto a guarded city brings its guards into range.

**Strike.** An icon. On an event card it is a line of the Card beat, after Infiltrate
([adversary.md](adversary.md)); the card lies face up a round ahead, so **the table sees a Strike coming for a full
round**. It is also slot 5 of the Militarist's research row, which is public, as is its cube count. On a Strike,
**every troop destroys one target on or next to its hex**, hex by hex, country by country and by hex number within
each; where a troop has several targets, the players choose which. **Targets:** nodes (`troop_node` 0: from the
start) and player-owned research stations and factories; never cities (corruption is the adversary's way in) or
neutral or adversary-owned facilities. A protection absorbs a hit as it does any destruction (*Destruction* below).
**The troop is not used up:** it stays and strikes again on the next icon. A troop with no target does nothing. A
strike that takes a seat's last node ends the game there (the loss; no later troop strikes). The event cards
printing Strike: stage II's Mobilization and Fifth Column, and all four stage-III cards.

**Redeploy.** An icon, rare: stage II's Redeployment and stage III's Occupation (after its Strike). Each troop
not on or next to a corrupted city (one on a corrupted factory whose city is free included) is picked up and put on
the nearest corrupted city, counted as a walk over land (troops never cross ocean; ties: the lower country, then the lower hex number). With
none to reach, it stays. No path is walked and nothing is struck on the way.

**Ambassadors don't stop troops.** A troop garrisons, guards and strikes whatever ambassadors stand by it, and
removes none. The owner's own ambassador on a struck piece's hex is still armor against the strike
([ambassadors.md](ambassadors.md)).

**Wall (locked).** **No troop is ever placed on the Protector's node hex** (a Garrison's, a project's or a Discord
card's: it stays in the supply), none Redeploys onto it (a corrupted city under that node is no destination), and
**none strikes it:** a node or facility on a hex with the Protector's node is no target, so it stands until the
node goes. A troop already on a hex when the Protector places there stays. **Vigil:** when troops Strike, the
Protector gains 1 insight for each troop on or next to its nodes, at most `board.vigil_cap` (2) a round
([ai-powers.md](ai-powers.md)).

**A no-Strike token** (Unity's Demilitarized Zone, [alignment.md](alignment.md)) makes the pieces on its hex no
target in the next adversary turn.

**Players see each hit coming:** the view draws, for each garrison, what it guards and what it can strike (the
arrow is solid when a Strike icon is shown for the next Card beat). Cards remove troops, each naming how many; a
strike card's "on or next to it" (its target) counts the target itself, and the player picks which pieces. Removing
a troop sways `alignment.troop` (0). **A Discord tip doesn't move troops.**

**For new players:** [rulebook.md](../rulebook.md) §2 (sites); its troop rules are r138's, frozen, and not these. Adversary
factories feed agents to their cities: liberate or block one and its agents stop coming.

## Node loss

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is local:** a seat loses nodes only where the board is overrun: an uncountered threat that prints it (the
player of the country it was dealt in destroys their node closest to its tokens; Siege takes the node on its
factory's hex), a site that corrupts (only a
node on its own hex), an arrival leaving `corrupt_node_at` (2) agents on a node hex ([adversary.md](adversary.md)),
Border War (each player with a node on or next to a factory destroys 1 of those nodes) and an adversary troop's
Strike (above). No project destroys a node itself (Night Raid lands agents on node hexes, for the
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
