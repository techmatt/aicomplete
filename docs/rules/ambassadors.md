# Ambassadors and sites

*The one site model (cities, research stations, factories), claiming, liberating, blocking, armor, canceling and removing agents. Terms and conventions are in [README.md](README.md).*

## One site model (Matt, 2026-11-06; candidate)

Cities, research stations and factories follow one set of rules and differ only in output:

| Owner | City | Research station | Factory |
|---|---|---|---|
| Neutral | its energy, bought by the Buy energy action ([economy.md](economy.md)) | a neutral cube onto its megaproject ([board.md](board.md)) | nothing, except that **at a Discord tip it arms its country's bases** ([war.md](war.md)) |
| Player | none: **no player owns a city** (254); a neutral city with your node on or next to it makes its energy for you (hosting, [economy.md](economy.md)) | `owned_station_cubes` research cubes in its owner's color | `factory_chips` chips to its owner |
| Adversary | its energy diverted: `divert_discord` Discord a unit | `station_cubes` adversary cubes; after its megaproject, research cubes onto the adversary's track (259) | 1 adversary troop on its hex |

- **One opposing piece blocks a site's output while it stands.** An agent on a neutral or player
  site, a player ambassador on an adversary site: that round the site makes nothing, and the piece stays
  (137; 139 dropped the dial that used it up). On a
  neutral city this is the **dark** city (r37, r38): nothing to buy, nothing to host (with `cities_unowned` 0 an
  aligned one gave its owner nothing and swayed `ambassador_discord` (0) a unit).
- **Your own ambassador is armor:** when your node, or a site you own, would be destroyed, your ambassador on its hex
  may be used up instead, the owner choosing which protection to spend, or none ([war.md](war.md), *Destruction*).
  Only your own guards your piece (r75, Matt's ruling 5): another player's never does, and a neutral site has none.
  One of your ambassadors on your site's hex can cover the site or your node there. Against troops it answers a
  strike from next door; a troop standing on the hex removes ambassadors first, one a step (175, [war.md](war.md)).
- **Claim and Liberate share one payment rule** (r75, Matt's ruling 2): the bill is the site's price (a site with
  an agent on it is refused, below); **your own ambassadors standing on the site pay first, then your supply**, never
  another player's. Both need the site in reach; a botnet on or next to it stops neither.
- **Claiming** (the Claim action, [economy.md](economy.md)): a neutral **research station or factory** in reach
  with no agent on it, `claim_station` 4, `claim_factory` 3 (252: priced under the cards for the job, Matt's
  ruling 5; 3 and 2 before). Your cube goes on. **A city is never claimed** (254, Matt's candidate,
  `cities_unowned` 1; r114 claimed one for `claim_city` 5 and bound one to each player at setup).
  Difficulty lowers only the adversary's numbers.
  **A claim pays back slowly** (142; 252): a player's station makes `owned_station_cubes` 1 cube and a
  factory `factory_chips` 1 chip a round (r83; a neutral station `station_cubes` 1). Give / get at the anchors: 3
  ambassadors (24) for 1 chip a round (6) pays back in 4 rounds, 4 ambassadors (32) for 1 cube a round (6) in about
  5: a fallback; the cards that claim do it cheaper. A factory claimed by round 3 of an 8-10 round game makes 6-8 chips, which the corps' chip
  converters ([board.md](board.md)) spend late. The risk is the adversary's: Asset Seizure turns it neutral.
  Claiming a neutral factory also stops it feeding the cities next to it ([war.md](war.md), city growth).
- **Claiming with a card** (142; 256: stations and factories, op `claim_site`): Power Purchase Agreement (an event:
  2 credits and 2 energy) and Microgrid Lab Compact (a building: one claim when built, then an Action for 2 energy) claim a neutral research
  station or factory in reach as the Claim action does (148, Matt): the card's price replaces the ambassadors, and
  it needs the site in reach with no agent on it. 2 energy (9) is cheaper than 3-4 ambassadors (24-32).
  **Open Research Campus** and **Industrial Commons** open a station or factory on their target and claim it: the
  card route to an owned station. **The liberate cards' rider** (256) claims a neutral station or factory next to
  the city they free, free and without reach (the card's target puts it in range); an agent on it still stops it.
- **Liberating an adversary site: free, then claim** (Matt's ruling 4, 2026-11-16, locked; reverses r75). The
  Liberate action takes massed ambassadors and **leaves the site neutral**: Claim or a card takes it from there. It
  pays Claim's price table (r91, Matt, 2026-11-12: one table): `liberate_at` 5 and `free_city` 5 Unity for a city,
  `claim_station` 4 or `claim_factory` 3 and `free_facility` 2 Unity for a station or factory. **A site with an agent
  on it can't be claimed or liberated** (r91: no "+1 per agent"; clear them first, with Remove agents, a card or a
  power). The adversary's cube comes off. It needs the site **in reach** (your node on or next to it: getting close
  is the risk); a botnet on it stays. No agents land on it. Its factories keep their owners. A **Liberated marker**
  goes on it: **to retake it by a Corrupt line or natural corruption the adversary must pile `retake_agents` (1)
  agent more than its corrupt number** on it (a card that takes or flips a site outright ignores it; [adversary.md](adversary.md); 253, amendment 11525: the adversary commits more to take back what you freed, Matt's
  ruling 4). The marker comes off when anyone claims the site, the adversary retakes it or it is
  destroyed; a node on that hex takes no extra agent. Liberate stays the win
  route (the purge). A **liberate card** ([player-cards.md](player-cards.md),
  *The liberate capstones*) liberates a city exactly so, for its own price instead of the bill: the agents come off,
  Unity sways and it is left neutral; going straight from adversary to yours is a rare card power (Hostile Buyout
  alone, 254: on a corrupted station or factory; a city it frees is left neutral). The price is the same all game (r67; late strength comes from cards).

## Moving pieces: Gather and Push (r70; Matt, 2026-11-09, *Moving pieces*; numbers candidates)

Pieces on the board move: **ambassadors** (any seat's), **agents** and **troops**. Botnets don't.

- **Gather up to N** pieces into a land hex in your reach, each from a hex next to it, one at a time; **Push up to N**
  pieces from a land hex in your reach, each to a land hex next to it, the player choosing where. "Up to": stop
  after any piece, so a move is never forced into a bad one. Distance 1. The card names which pieces it moves.
- **Arrivals:** a moved piece arrives as any does. An agent arriving on standing ambassadors cancels one of them; an
  ambassador arriving on agents cancels one; a firewalled hex takes no agent; no troop enters the Protector's node
  (Wall). Troops otherwise just move.
- **A strike** (Kettle Operation, Lure Network's Action, Honeypot Relay) moves nothing: "remove up to N Agents (or
  Troops) on or next to it, your choice" (the card's target): the target counts, and the player picks the pieces,
  batch by batch, and may stop. A card's gather or push acts on its target too ([card-grammar.md](card-grammar.md)).
- **Where:** Kettle Operation, Lure Network, Honeypot Relay (strikes), Checkpoint Network (push; Action: push),
  Exodus Corridors and Rally the Allies (gather ambassadors), and the Protector's Patrol power (1 energy: push up to 3 agents or
  troops from its target). What a card moves is set by its price, not its tier: about 2 credits a piece (`values.json` `gather`, `push`).

## Convert and firewall (r69; Matt, 2026-11-09; candidate)

Ten tier-2 cards (drafts, [player-cards.md](player-cards.md)) act on the agents where they stand,
for the last rounds, and never free a city: the players still reach it and Liberate.

- **Convert** (Mass Defection, Double Agents, Open Source Uprising, Embassy of Exiles' Action): up to N agents on or next to one city in your reach leave the adversary's stack (to its supply) and become
  your ambassadors **where each stood**. All the converted agents come off first, then the ambassadors go on, so two
  converted on one hex both stay; an agent the card left on that hex meets one of them, 1 for 1, as any contact
  does. A converted ambassador is on the board, not in your supply: it is armor or a blocker, and moves like
  any other; standing on the city, it pays toward a Liberate there.
- **Firewall** (Firewall Array's Action, Air Gap): a token on one land hex in your reach. While it stands no
  agent arrives there: Rally's share for its site stays in the supply (r82), a creation line takes the next site, and so does an adversary-factory
  line's fallback onto a factory (r80, 184's D79), a threat's or Discord card's placement there doesn't happen, and a
  Corrupt line moves no agent onto it. Nothing on it corrupts: a Corrupt line, natural corruption, a station flip
  (Station Coup, Station Seizure, Lab Takeover: a forced flip too) and a threat corrupting its city (Overrun, Regime
  Change) all skip it (r79, Matt's ruling on D77; agents already on it stay). Troops are not stopped. It comes off at the end of the next adversary turn (its Income beat), so it
  lasts one round; the building places a fresh one each round.

A **base** is no site: it is a front's troop pile ([war.md](war.md)). A player ambassador standing on a base is used
up instead of the next drone made there; an agent on one does nothing.
**2 agents on a player's node hex destroy the node** (`corrupt_node_at`), whatever else stands there (175,
[adversary.md](adversary.md)): the node is lost unless something absorbs it, the agents go to the supply, and the hex
takes no mark (r98).
One agent on it costs nothing.

- **Every contact is 1 for 1** (r75, Matt's ruling 1). An agent arriving (a Rally, a creation line, a Corrupt line's
  move, a Gather or Push, a fallback landing) on a hex holding player ambassadors cancels one of them; with several
  players' there, the players choose whose (seat 0 asks for the team). An ambassador arriving on agents cancels one.
  Both go (the agent to the adversary's supply), with no Unity, so a position is held only until the next arrival.
  Claim and Liberate refuse a site with an agent on it (206): clear it first.
- **Remove agents** (the standard action, [economy.md](economy.md)): spend `remove_ambassadors` (2) ambassadors from
  your supply, used up; target Agents and remove up to `remove_count` (3), the first on it, the rest one at a time
  on or next to it (r107; any hexes in reach before), your choice, stopping when you like. It places nothing (r75; it stood on one hex and removed 3 there). The supply
  caps it: each use needs two new ambassadors (r100: 2 for 3 agents, 0.94 at the anchors, was 1 at 1.88).
- **Consumed or standing:** claiming, liberating, Remove agents, Envoy, armor and the troop rule use a player
  ambassador up; blocking and moving leave it standing. A consumed ambassador is gone: new ones come from cards,
  Followers and the standard project. Player ambassadors move through cards (any player's Gather of
  ambassadors moves yours too) and Dispatch. A Dispatch puts one onto a piece in your reach, taken from your supply
  or from any hex on the board, however far (a card's "move up to N" takes them from your supply or on or next to
  its target, r107); it is free, and a seat has
  its board's Dispatches a round ([ai-boards.md](ai-boards.md)). A Dispatch may also put one on a botnet's hex in
  reach, where the Take down a botnet action can use it (it stays).

Agents can stand with no piece under them (on an empty industrial zone; on a hex whose node was lost) until removed.

**Your supply has no limit** ([economy.md](economy.md), *No limits*). Orbital Embassy gives 1 ambassador a
Diplomacy tag on each card you play (127; at 1 a tag of any kind it piled 52-84 in long games).

**For new players.** Every research station and factory belongs to a player, to the adversary, or to nobody, and
makes something for its owner; a city is the adversary's or its country's, never a player's. Spend your ambassadors
to take a site back from the adversary (Liberate: 5 for a city, 4 for a station, 3 for a factory) or to claim a
neutral station (4) or factory (3), never with an agent sitting on it; your own ambassadors standing on it pay first.
A claim makes it yours; taking one back leaves it neutral and sways Unity your way, and you may claim a station or
factory after; a city stays free, and a node of yours on or next to it collects its energy each round. While a site you liberated (with Liberate or a liberate card) is unclaimed, a Liberated marker makes the
adversary bring one agent more to corrupt it again (a card that takes a site outright needs none); a station or factory a card frees takes no marker. A station you own makes 1 cube a round (a
factory 1 chip), so a claim is a slow fallback; cards claim cheaper (256: a station or factory for energy, Campus and Commons the site they open, the liberate cards one next to the city). Every meeting of an agent and an ambassador is one for one. Stand one ambassador on an adversary site
to shut it off, and clear agents off yours, which they shut off the same way. An ambassador of yours on your own piece
can take a hit for it.
