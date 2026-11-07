# Factories, the clock, war and troops

*The production/war clock, factories, troops, war, the march and node loss. Terms and conventions are in [README.md](README.md).*

## The production/war clock

A printed round track (`board.clock`) says what each round's production holds: 1 a
**production step**, 2 a **war step**, 3 both; the last entry repeats. As Matt
confirmed, the clock gates only factories, troops, chips and war; the every-round
income (cities' energy, the node track, stations, upkeep) and the adversary troops' march
run every round. **Production is one snapshot** (135): troops, chips, energy, stations, income and upkeep all read
the board as the step starts, so their order no longer matters; **war comes after it**, so what war destroys is gone
for next round's production. **War brought forward** (Casus Belli, General Mobilization; candidate): the fronts next
to the threat's base fight twice in next round's war step (their troops cancel once, the surplus strikes twice); with
no war step that round they fight once. Declaration of War (an ongoing project): every war step, the front with the
most corp troops fights twice.

## Factories

- **Ownership:** an owner cube, as stations and cities have. A factory a city builds: an
  aligned city's is that player's, a corrupted city's is the adversary's, and a clean
  city's goes to **the player whose node is next to the zone the players picked** (Matt:
  the most nodes on or next to it; players choose on a tie, as for city energy), neutral
  only if no node is there. A player ambassador claims a neutral factory as it claims a
  station ([ambassadors.md](ambassadors.md), needs a site claim). Cards that build a factory (Prefab Plant Kit) build the
  player's own, on a free zone in reach. An owned factory keeps its owner if its city
  changes hands later.
- **Growth** (each production step, after troops and chips): every city builds one
  factory. A clean or aligned city builds on a free industrial zone next to it, where the
  players choose (an aligned city's owner, else seat 0). A **corrupted** city builds at a
  free industrial zone in its own country **next to an adversary ambassador if there is
  one, otherwise the lowest-numbered free zone** (Matt; the lowest number among several
  next to one). A city with no such zone builds nothing. Factories destroyed in war regrow
  this way.
- **Troops** (each production step): each base makes 1 troop for its corp; every factory
  makes `factory_units` troop for its megacorp **at each of that corp's bases** (Matt: at
  every front base, not just the nearest), so at 3 and 4 countries a factory makes 2. An
  adversary-owned factory, or one holding an adversary ambassador (used up), makes an
  adversary troop on its hex instead (**locked**). A player ambassador on a factory stops
  its corp troops (136: it no longer sways Unity); on an adversary-owned factory it
  stops its troop. Either way it stays. **An adversary factory frees as a corrupted station does**
  (Matt, 2026-11-06): the Free a station action frees it to neutral. Threats build them (Mass Production, Shadow
  Factories, War Economy), seize them (Hostile Takeover), occupy them (a standing troop: it makes nothing) and destroy
  them (Scorched Earth).
- **Chips**: each player-owned factory makes `factory_chips` chip for its owner on each
  production step.

## Troops and war

**Corp troops.** There are no player troops. Troops belong to the megacorp of their
territory and wait at its bases.

- **Peacetime:** at most `unit_cap` troops wait at each base. A troop at a base **absorbs
  an adversary-troop strike** on a facility on or next to the base: 1 troop per hit.
- **Extra corp troops** (135, candidate) go past the cap: Warmongering (the Militarist's board: a base with an
  adversary troop on or next to it), Conscription (3 rounds), Arms Race (ongoing), General Mobilization (2 now).
  A base gains at most 1 extra a production step from all of them together (Matt, 2026-11-06).
- **On a war step**, on each front the troops at the two bases cancel 1 for 1. The surplus
  crosses, acts at once and is used up: each destroys the rival corp's factory nearest its
  base (not an adversary-owned one; ties: lower hex number), **and any node on its hex**:
  the node goes with the factory, unabsorbed, and only the factory sways. All troops at the
  front are spent.

**Adversary troops (candidate): the march.** Adversary troops come from adversary
ambassadors on factories, from adversary-owned factories (production steps), from the build's
`wall_troops` landing where the Protector stands, and from projects and threats (Armored Columns, Orbital Strike,
Total War). They are the adversary's
mobile threat to player infrastructure. **Targets:** player-owned research stations and
factories, **not nodes** (`troop_node`; a troop on or next to a node only stands there).
Cities are never targets (corruption is the adversary's way into cities), nor are neutral or
adversary-owned facilities.

In the adversary phase (`board.march`, after the corruption step) **every adversary troop
takes `troop_steps` steps** (Forced March's twist: more), one troop at a time, in hex-number order fixed when the
march starts. **Standing troops** (Orbital Strike, Factory Occupation) never march or strike until removed; troops a
threat lands to "march at once" (Total War, Spearhead) take their steps as it executes. A step is one of:

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
factories and on any factory holding an adversary ambassador (production rounds), from some threats and
projects, and every round 1 lands on a factory (else a city) in each country where the Protector has a node.
Every round, in the adversary's turn,
each troop takes two steps. If a research station or factory you own is on or next to the
troop's hex, it destroys that piece and leaves the board; otherwise it walks one hex toward
the nearest such piece. So a troop two hexes from your piece hits it next round, and one
three or four hexes away a round later; the arrows on the board show where each troop is
heading. Troops never attack cities. To stop a hit, remove the troop with a card, keep a
corp troop at a military base next to the target, harden the target, stand an ambassador on
it, or put the Protector's node on it or in its way. Troops don't hit nodes.

**This is the game's one movement rule** (an exception to "no movement rules"): spread
places ambassadors, it does not walk them. **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch ([README.md](README.md#off-variants)): troops never move or strike.

**Node destruction.** A player with no nodes on the board loses the game for everyone ([alignment.md](alignment.md)).
**It is a slide, not a cliff:** a seat loses tokens a few at a time from the middle of the
game on, from several sources: uncountered threats (Eviction offers it as a choice;
Agitators, Orbital Strike and Defection in tier 1 and Station Seizure, Overrun, Sleeper
Cells, Insurrection in tier 2 make each player here destroy one of theirs here; Manhunt and
Siege two), a city that becomes corrupted (nodes on its hex and, by `corruption_reach`, next
to it), war that destroys a factory (nodes on its hex and, by `war_reach`, next to it:
0, an adversary dial kept below the players' reach 1, so war takes only the factory's hex),
stage-III adversary cards (each player destroys two) and Border War (one), Discord cards,
and Purge the Unaligned. **Discord steps:** the tier-2 cards and the tier-1 card left for
the last tip make each player destroy nodes of theirs at their printed steps ([alignment.md](alignment.md); the
brief's 1, 2, 3; **Candidate**). A hit is each player's own pick of which token, and it is
absorbed as any destruction is (below), so a token shielded by an ambassador is a good pick.
**While this round's adversary card is stage I, a player's last node can't be destroyed**
(candidate, **Early fragility**): each player starts with 1, and a war in round 2 would
otherwise end the game. Eviction goes further: it takes a node only from a player with 2 or
more, at any stage. Offensive's target, on the board, is the node hex with the most
adversary ambassadors and troops (ties: country, then lower hex number). Purge the Unaligned
has each player give up any one of their ambassadors (supply or a hex) or nodes, never a last node; a node lost
this way leaves the game. A city that corrupts takes the node unabsorbed: corruption is not
a destruction shelters and shields answer.

**A lost node (locked, as in Spirit Island).** A destroyed node, or one lost to Purge the
Unaligned, **leaves the game**: it does not go back on its track, and what it uncovered stays
uncovered. Each lost token is a board location you can no longer act from, so getting all
your nodes out is always a tradeoff.

**Corruption markers (candidate; `node_marker`).** A lost node leaves a corruption marker on
its hex, unless the hex is a corrupted site, holds a botnet or already holds a marker (the
cap on the loop: a marker adds no corruption where there is some). A marked hex can't take a
node. On a city that is not corrupted it counts 1 toward `corrupt_at`; since no node stands
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
corp troop at a base next to it (adversary strikes only), a hardening token on the hex, a
player ambassador on the hex, or the Aegis Doctrine capstone (once a
round, for its owner's countries). A destroyed node leaves the game (above). A destroyed
station or factory takes its cube with it.
