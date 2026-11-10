# Factories, bases, war and troops

*Factories, bases, corp troops, war every round, the march and node loss. Terms and conventions are in [README.md](README.md).*

## Every round: production, then war at the end

There is no production/war clock (137; cut for good at 139, Matt). **Production runs every round** and is one snapshot (135): chips, adversary troops, energy, stations, income
and upkeep all read the board as the step starts. **War is fought at the end of every round** on every front, after
threats execute, so what war destroys is gone for next round's production. **War brought forward** (Casus Belli;
candidate): the fronts next to the threat's base fight twice in next round's war (their troops
cancel once, the surplus strikes twice). Declaration of War (an ongoing project): each round, the front with the most
corp troops fights twice.

## Factories

- **Ownership:** an owner cube, as stations and cities have. A factory a city builds: an
  aligned city's is that player's, a corrupted city's is the adversary's, and a clean
  city's goes to **the player whose node is next to the zone the players picked** (Matt:
  the most nodes on or next to it; players choose on a tie, as for city energy), neutral
  only if no node is there. A neutral factory is claimed, and an adversary one liberated, as any site is
  ([ambassadors.md](ambassadors.md)). Cards that build a factory (Prefab Plant Kit) build the
  player's own, on a free zone in reach. An owned factory keeps its owner if its city
  changes hands later.
- **Output** (each round; the site table in [ambassadors.md](ambassadors.md)): a player's factory makes
  `factory_chips` (2) chips for its owner; an adversary factory makes 1 adversary troop on its hex (**locked**); a neutral
  factory makes nothing, except that **on each Discord tip it makes 1 corp troop**. One opposing ambassador or agent blocks a
  factory's output while it stands. Threats build adversary factories (Mass Production, War Economy), seize them
  (Factory Occupation: corruption, with its cleanup) and destroy them (Siege); a project builds them (Shadow Factories).
- **Growth** (each round, after output): every city builds one
  factory. A clean or aligned city builds on a free industrial zone next to it, where the
  players choose (an aligned city's owner, else seat 0). A **corrupted** city builds at a
  free industrial zone in its own country **next to an agent if there is
  one, otherwise the lowest-numbered free zone** (Matt; the lowest number among several
  next to one). A city with no such zone builds nothing. Factories destroyed in war regrow
  this way.

## Bases and war

**A base is a front's troop pile** (137): each corp has one on each of its active fronts (every printed front is
active; a 1-country game has none), with no owner and no ambassador rules beyond the troop rule below. There are no
player troops: corp troops belong to the megacorp of their territory and wait at its bases.

- **Whenever a corp makes troops, it puts 1 on each of its bases** (so a neutral factory's tip troop is 2 at 3 and 4
  countries). Bases make no troop of their own. **The troop rule:** a player ambassador standing where a troop would
  be made (a base, or a neutral factory at a Discord tip) is used up instead of the troop; no decisions at tip time.
- **Peacetime:** at most `unit_cap` troops wait at each base. A troop at a base **absorbs
  an adversary-troop strike** on a facility on or next to the base: 1 troop per hit.
- **Extra corp troops** (135, candidate) go past the cap: Warmongering (the Militarist's board: a base with an
  adversary troop on or next to it) and Arms Race (ongoing). A base gains at most 1 extra a round from the two
  together (Matt, 2026-11-06). No threat makes corp troops (146: no special pieces).
- **War, at the end of every round:** on each front the piles at the two bases cancel 1 for 1. Each surplus troop
  crosses, acts at once and is used up: it destroys the rival corp's factory nearest its base (not an
  adversary-owned one; ties: lower hex number), **and any node on its hex** (the node goes with the factory,
  unabsorbed). **Each factory destroyed in war sways `destruction_discord` (1) Discord:** runaway Discord is an
  intended loss route, which players stop by standing ambassadors on bases and neutral factories, claiming
  neutral factories and keeping the Discord track below its tip.

**Adversary troops (candidate): the march.** Adversary troops come from adversary-owned factories (each round),
from the build's
`wall_troops` landing where the Protector stands, and from projects and threats (Armored Columns, Orbital Strike,
Total War). They are the adversary's
mobile threat to player infrastructure. **Targets:** player-owned research stations and
factories, **not nodes** (`troop_node`; a troop on or next to a node only stands there).
Cities are never targets (corruption is the adversary's way into cities), nor are neutral or
adversary-owned facilities.

In the adversary phase (`board.march`, after the corruption step) **every adversary troop
takes `troop_steps` steps** (Forced March's twist: more), one troop at a time, in hex-number order fixed when the
march starts. A troop a threat lands follows the troop's normal rules (146: no standing troops); troops a threat
lands to "march at once" (Total War) take their steps as it executes. A step is one of:

- **destroy** a target on or next to its hex (players choose which), or
- with none there, **move** one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose: a choice among bad options, as with Spirit Island's blight.

**Wall (candidate, Matt).** Adversary troops can't enter a hex with the Protector's node: a
march goes around it, and a landing skips it. **Nor can a troop strike into it (Matt):** a
facility on a hex with the Protector's node is no target, so it stands until the node goes.
A troop already standing on a hex when the Protector places there leaves as it likes. The
build's `wall_troops` landing ([adversary.md](adversary.md)) puts the strikes Wall stops where the Protector stands.
A landing enters a hex, so it counts; a troop made by a factory on its own hex
doesn't enter. With no open way to a target a troop stays where it is.

So a troop moves twice, or moves then destroys, or destroys at once. **A troop that strikes
is used up** (`troop_spent`), whether the target is destroyed or the hit absorbed, as an
ambassador is used up when it acts. With no target on the board a troop stays where it is.
Troops cross borders and never enter ocean; "nearest" and "toward" count the walk over land,
around walls (above). A troop two hexes from a target hits it in the next march; three or
four hexes away, in the march after. **Players see each hit coming:** the view draws each
troop's next target (the nearest; the arrow is solid when the hit lands in the next march),
and every troop on the board was there through an action phase except those that arrive
between the action phase and the march (built in production, placed by an uncountered
threat or by Armored Columns in the build): an adversary factory within two hexes of your
station or factory is a known danger. Cards remove troops, each card naming how many.
Removing a troop sways `alignment.troop` toward Unity: 0 since r38 (Matt: removing one already
pays by stopping its harm, so the Sway counted it twice); a card meant to build Unity that way
prints its Sway (none does).

**For new players.** Adversary troops are the adversary's raiders. They appear on adversary
factories every round, from most event cards (Deploy: on each board's lowest-numbered factory), some threats and
projects, and every round 1 lands the same way on each board where the Protector has a node.
Every round, in the adversary's turn,
each troop takes two steps. If a research station or factory you own is on or next to the
troop's hex, it destroys that piece and leaves the board; otherwise it walks one hex toward
the nearest such piece. So a troop two hexes from your piece hits it next round, and one
three or four hexes away a round later; the arrows on the board show where each troop is
heading. Troops never attack cities. To stop a hit, remove the troop with a card, keep a
corp troop at a military base next to the target, harden the target, stand your own ambassador on
it, or put the Protector's node on it or in its way. Troops don't hit nodes.

**Sites, bases and war, for new players.** Cities, research stations and factories work alike: whoever owns one gets
its output (energy, research cubes, chips), and the adversary's make Discord, compute or raiders. Claim a neutral site
with your ambassadors (4 for a city, 3 for a station or factory); take one back from the adversary with more (5 for a
city, 3 for a station or factory). One of your ambassadors standing on an adversary site shuts it off, and one of
its agents shuts off yours; your own ambassador on your piece takes a hit for it. Each border between countries has a
military base on each side, a pile of corp troops: every Discord tip, each neutral factory adds a troop to each of its
corp's bases (unless your ambassador stands there to stop it), and at the end of every round the two piles cancel and
each troop left over destroys a factory across the border, swaying 1 Discord. So Discord feeds war and war feeds
Discord: keep neutral factories quiet and the Discord track low. **What resisted this summary:** the extra corp
troops (Warmongering, Arms Race) ignore the pile's cap, and the agent count you pay for
grows by one for each agent already on the site.

**This is the troops' own movement rule** (an exception to "no movement rules"); a Corrupt line's pull moves agents one
hex onto the site that corrupts ([adversary.md](adversary.md)), and Gather and Push (cards, the Protector's Patrol) move
ambassadors, agents and troops ([ambassadors.md](ambassadors.md)). **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch ([README.md](README.md#off-variants)): troops never march or strike (Gather and Push still move them).

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is a slide, not a cliff, and it is local (139, Matt):** a seat loses tokens a few at a time from the middle of
the game on, only where the board is overrun: an uncountered threat that prints it (146: Dark Cities, Purge
Protocols, Grid Collapse, Overload and Last Ultimatum make the nearest player destroy their node closest to it, never
their last; Siege takes the node on its factory's hex), a site that becomes
corrupted (138: only a node on its own hex), war that destroys a factory (nodes on its hex and, by `war_reach`, next
to it: 0, an adversary dial kept below the players' reach 1, so war takes only the factory's hex) and Border War (each
player with a node on or next to a front's base destroys 1 of those; Matt's message 5847). **Nothing else destroys a
node:** Discord cards, projects and the other event cards print board pressure instead (more or higher-tier threats,
creation, Corrupt lines, credits, Unity or ambassadors lost; [alignment.md](alignment.md)); Offensive deals a threat
onto the node hex with the most agents and troops (ties: country, then lower hex number), which
destroys nodes only if it goes uncountered. A hit is each player's own pick of which token, and it is absorbed as any
destruction is (below), so a token shielded by an ambassador is a good pick.
**While this round's adversary card is stage I, a player's last node can't be destroyed**
(candidate, **Early fragility**, kept at 139): each player starts with 1, and a war in round 2 would
otherwise end the game. A threat's printed node line never takes a last node, at any stage. A site that corrupts destroys the node on its hex as any destruction (138): an
ambassador there, a shield or a shelter answers it.

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
Science cards clear them too, for the same reward. Markers come only from destroyed nodes,
so the reward can't be farmed: a token lost costs far more than a Sway. The
brief names the marker and its caps, not what it does; this is the smallest reading that
makes it matter.

**Destruction.** Only war sways Discord: a factory destroyed by the corp troops crossing a
front sways `destruction_discord` Discord. An adversary troop's strike (in the march) sways
nothing (Matt). A destruction is absorbed, in this order, by a shelter on the country, a
corp troop at a base next to it (adversary strikes only), a hardening token on the hex, **the owner's own
ambassador on the hex** (armor, 137: a node's owner's, or the owner of the site there; a neutral site has none), or the
Aegis Doctrine capstone (once a
round, for its owner's countries). A destroyed node leaves the game (above). A destroyed
station or factory takes its cube with it.
