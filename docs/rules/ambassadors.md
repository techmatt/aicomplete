# Ambassadors and sites

*The one site model (cities, research stations, factories), claiming, liberating, blocking, armor, canceling and removing agents. Terms and conventions are in [README.md](README.md).*

## One site model (Matt, 2026-11-06; candidate)

Cities, research stations and factories follow one set of rules and differ only in output:

| Owner | City | Research station | Factory |
|---|---|---|---|
| Neutral | `corp_energy` energy to every player with a node on or next to it ([economy.md](economy.md)) | a neutral cube onto its megaproject ([board.md](board.md)) | nothing |
| Player | none: **no player owns a city** (254) | `owned_station_cubes` research cubes in its owner's color | `factory_chips` chips to its owner |
| Adversary (r140: one output, in its Income beat, [adversary.md](adversary.md)) | `divert_discord` (1) Discord | `station_cubes` (1) research cube onto the adversary's track | 1 agent, pushed along its zone's arrow onto its city |

- **One opposing piece blocks a site's output while it stands.** An agent on a neutral or player
  site, a player ambassador on an adversary site: that round the site makes nothing, and the piece stays
  (137; 139 dropped the dial that used it up). The table marks it with a **"blocked" chip** as the piece arrives
  and reads production from the tracks (r133, [troops.md](troops.md)). On a
  neutral city this is the **blocked** city (r37, r38): it pays no one (with `cities_unowned` 0 an
  aligned one gave its owner nothing and swayed `ambassador_discord` (0) a unit).
- **Your own ambassador is armor:** when your node, or a site you own, would be destroyed, your ambassador on its hex
  may be used up instead, the owner choosing which protection to spend, or none ([troops.md](troops.md), *Destruction*).
  Only your own guards your piece (r75, Matt's ruling 5): another player's never does, and a neutral site has none.
  One of your ambassadors on your site's hex can cover the site or your node there. Against troops it answers a
  Strike on its hex; troops otherwise ignore ambassadors (they garrison, guard and strike past them,
  [troops.md](troops.md)).
- **Liberate's payment rule** (r75, Matt's ruling 2; r142): Liberate is the act of one of your ready ambassadors
  standing on the site. The bill is the site's price; **your own ambassadors on the site pay first (the one acting
  among them), then your supply**, never another player's. A botnet on or next to it doesn't stop it.
- **Only cards claim** (262, Matt, 2026-11-17: the Claim standard action is cut; it was used 0.03 a seat-game while
  cards claimed 1.72 a game). A claim puts your cube on a neutral **research station or factory** with no agent on
  it. **A city is never claimed** (254, Matt's candidate, `cities_unowned` 1; r114 bound one to each player at
  setup). Difficulty lowers only the adversary's numbers.
  **A claim pays back slowly** (142; 252): a player's station makes `owned_station_cubes` 1 cube and a
  factory `factory_chips` 1 chip a round, which the corps' chip converters ([board.md](board.md)) spend late. The risk is the adversary's: Asset Seizure turns it neutral.
- **Claiming with a card** (142; 256: stations and factories, op `claim_site`): Power Purchase Agreement (an event:
  2 credits and 2 energy) and Microgrid Lab Compact (a building: one claim when built, then an Action for 2 energy) claim a neutral research
  station or factory in reach (148, Matt): the card's price is the price, and it needs the site in reach with no
  agent on it.
  **Open Research Campus** and **Industrial Commons** open a station or factory on their target and claim it: the
  card route to an owned station. **The liberate cards' rider** (256) claims a neutral station or factory next to
  the city they free, free and without reach (the card's target puts it in range); an agent on it still stops it.
- **Liberating an adversary site: move in, then act** (r142, Matt 2026-11-19; free, then claim: ruling 4,
  2026-11-16, locked). One of your ambassadors moves onto the corrupted site (it is tipped), and the next round,
  ready again, it acts: Liberate. One that arrives ready on the site (a card places it there, or your node stands
  on the site's hex) may act the same round. The action takes massed ambassadors and **leaves the site neutral**: a
  card may claim it from there. Its price table (r91, Matt, 2026-11-12: one table): `liberate_at` 3 and `free_city` 5 Unity for a city,
  `claim_station` 3 or `claim_factory` 3 (Liberate's since 262 cut Claim; the names stay) and `free_facility` 2
  Unity for a station or factory. **A site with an agent
  on it can't be claimed or liberated** (r91: no "+1 per agent"; clear them first, with Remove agents, a card or a
  power). **Guarding** (r141, Matt 2026-11-19): **a corrupted site with a troop on or next to it can't be liberated**,
  by the action or by a liberate card: remove the troop or Push or Gather it away first ([troops.md](troops.md)).
  The adversary's cube comes off. The action needs **no reach** (your node need not be near: the round your
  ambassador stands there tipped is the risk); a botnet on it stays. It takes no marker:
  cleared of agents, it corrupts again only when arrivals bring it back to its corrupt number
  ([adversary.md](adversary.md)). Liberate is the win route (no corrupted city, [alignment.md](alignment.md)). A **liberate card** ([player-cards.md](player-cards.md),
  *The liberate capstones*) liberates a city in its reach for its own price instead of the bill, with no ambassador
  on it: the agents come off, Unity sways and it is left neutral; going straight from adversary to yours is a rare card power (Hostile Buyout
  alone, 254: on a corrupted station or factory; a city it frees is left neutral). The price is the same all game (r67; late strength comes from cards).

## Moving pieces: Gather and Push (r70; Matt, 2026-11-09, *Moving pieces*; numbers candidates)

Pieces on the board move: **ambassadors** (any seat's), **agents** and **troops**. Botnets don't.

- **Gather up to N** pieces into a land hex in your reach, each from a hex next to it, one at a time; **Push up to N**
  pieces from a land hex in your reach (troops also from a hex on or next to your ambassador, r150), each to a land
  hex next to it, the player choosing where. "Up to": stop
  after any piece, so a move is never forced into a bad one. Distance 1. The card names which pieces it moves.
- **Arrivals:** a moved piece arrives as any does. An agent arriving on standing ambassadors cancels one of them; an
  ambassador arriving on agents cancels one; a firewalled hex takes no agent; no troop enters the Protector's node
  (Wall). Troops otherwise just move: a moved troop garrisons its new hex, guarding and striking from there, so
  pushing one away from a corrupted site opens it to a Liberate.
- **A strike** (Kettle Operation, Lure Network's Action, Honeypot Relay) moves nothing: "remove up to N Agents (or
  Troops) on or next to it, your choice" (the card's target): the target counts, and the player picks the pieces,
  batch by batch, and may stop (r150: around a troop reached only through your ambassador, agents only where your
  nodes reach). A card's gather or push acts on its target too ([card-grammar.md](card-grammar.md)).
- **Where:** Kettle Operation, Lure Network, Honeypot Relay (strikes), Checkpoint Network (push; Action: push),
  Exodus Corridors and Rally the Allies (gather ambassadors), and the Protector's Patrol power (I, 1 energy: push up to 3 agents or
  troops from its target; II, 2 energy, also hardens a facility on or next to it). What a card moves is set by its price, not its tier: about 1.3 credits a piece (`values.json` `gather`, `push`).

## Convert and firewall (r69; Matt, 2026-11-09; candidate)

Ten tier-2 cards (drafts, [player-cards.md](player-cards.md)) act on the agents where they stand,
for the last rounds, and never free a city: the players still reach it and Liberate.

- **Convert** (Mass Defection, Double Agents, Open Source Uprising, Embassy of Exiles' Action): up to N agents on or next to one city in your reach leave the adversary's stack (to its supply) and become
  your ambassadors **where each stood**. All the converted agents come off first, then the ambassadors go on, so two
  converted on one hex both stay; an agent the card left on that hex meets one of them, 1 for 1, as any contact
  does. A converted ambassador is on the board, not in your supply: it is armor or a blocker, and moves like
  any other; standing on the city, it pays toward a Liberate there.
- **Firewall** (Firewall Array's Action, Air Gap): a token on one land hex in your reach. While it stands no
  agent arrives there: an Infiltrate's or Escalate's agent for its site stays in the supply, a creation line takes the next site, and so does an adversary-factory
  line's fallback onto a factory (r80, 184's D79), a threat's or Discord card's placement there doesn't happen, and a
  factory pushes no agent onto it. Nothing on it corrupts: an arrival's check (a node's loss too), a station flip
  (Station Coup, Station Seizure: a forced flip too) and a threat corrupting its city (Overrun) all skip it (r79, Matt's ruling on D77; agents already on it stay). Troops are not stopped. It comes off at the end of the next adversary turn (its Income beat), so it
  lasts one round; the building places a fresh one each round.

**2 agents on a player's node hex destroy the node** (`corrupt_node_at`), whatever else stands there, checked when
the adversary puts agents there (an arrival, [adversary.md](adversary.md)): the node is lost unless something absorbs it, the agents go to the supply, and the hex
takes no mark (r98).
One agent on it costs nothing.

- **Every contact is 1 for 1** (r75, Matt's ruling 1). An agent arriving (an Infiltrate, an Escalate, a factory's
  push, a creation line, a Gather or Push, a fallback landing) on a hex holding player ambassadors cancels one of them; with several
  players' there, the players choose whose (seat 0 asks for the team). An ambassador arriving on agents cancels one.
  Both go (the agent to the adversary's supply), with no Unity, so a position is held only until the next arrival.
  A claim and Liberate refuse a site with an agent on it (206): clear it first.
- **Remove agents** (the standard action, [economy.md](economy.md)): spend `remove_ambassadors` (3) ambassadors from
  your supply, used up; target Agents and remove up to `remove_count` (3), the first on it, the rest one at a time
  on or next to it, your choice, stopping when you like. It places nothing. The supply
  caps it: each use needs three new ambassadors.
- **Consumed or standing:** liberating, Remove agents and armor use a player
  ambassador up; blocking and moving leave it standing. A consumed ambassador is gone: new ones come from cards,
  Followers and the standard project, into your supply.
- **One action a round** (r142, Matt 2026-11-19). An ambassador on the map is **ready** or **tipped**. Each does
  one thing a round, then is tipped over until cleanup stands every one up:
  - **Move** (the Move an ambassador action, free): onto a land hex next to it, or onto any of your nodes from
    anywhere on the map. The hex needs no piece and no reach. An ambassador leaves your supply the same way, onto
    one of your nodes: its *arrival*.
  - **Act** where it stands: **Liberate** the site, or **Take down a botnet** on its hex (`botnet_credits` 13
    credits; it stays, tipped; no reach needed).
- **It arrives ready** (`board.arrive_ready` 1; to try): an ambassador coming onto the map, from your supply or
  placed by a card or power (Convert, "place"), stands ready: it may still move or act that round. **A card's move
  is not its own action:** one a card moves from another hex (Gather, Push, "move up to N", which takes them from
  your supply or on or next to its target, r107) stays as it was, a tipped one tipped. Any player's Gather of
  ambassadors moves yours too. A tipped ambassador blocks, armors and cancels like a ready one. When one of several
  of yours on a hex leaves it (canceled, spent, used as armor, moved by a card), a ready one leaves first.
- **Troops and reach:** a troop on or next to one of your ambassadors is in your reach for anything that targets
  troops ("Target: Troops", and "Target: Agents or Troops" where a troop stands: Remove a troop, Fortify, Interdict,
  Patrol, cards), so a guard beside a city your ambassador stands on can be removed or pushed though it is far from
  your nodes.

Agents can stand with no piece under them (on an empty industrial zone; on a hex whose node was lost) until removed.

**Your supply has no limit** ([economy.md](economy.md), *No limits*). Orbital Embassy gives 1 ambassador a
Diplomacy tag on each card you play (127; at 1 a tag of any kind it piled 52-84 in long games).

**For new players.** Every research station and factory belongs to a player, to the adversary, or to nobody, and
makes something for its owner; a city is the adversary's or its country's, never a player's. Each of your ambassadors
on the map does one thing a round, then is tipped over: move (to a hex next to it, or to any of your nodes; from
your supply onto one of your nodes) or act where it stands. To take a site back from the adversary, move one onto
it, and next round it acts (Liberate: 3 ambassadors, any site), never with an
agent sitting on it or a troop on or next to it; yours standing on it pay first, your supply the rest. Taking one back leaves it neutral and sways
Unity your way; only cards claim a neutral station or factory, making it yours; a city stays free, and pays 1 energy each round to every player with a node on or next to it. A site you free starts clean: the adversary must land its full corrupt number of
agents on it again to take it back. A station you own makes 1 cube a round (a
factory 1 chip), a slow payback (a card claims a station or factory for energy, Campus and Commons the site they open, the liberate cards one next to the city). Every meeting of an agent and an ambassador is one for one. Stand one ambassador on an adversary site
to shut it off, and clear agents off yours, which they shut off the same way. An ambassador of yours on your own piece
can take a hit for it.
