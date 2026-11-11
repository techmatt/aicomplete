# Ambassadors and sites

*The one site model (cities, research stations, factories), claiming, liberating, blocking, armor, canceling and removing agents. Terms and conventions are in [README.md](README.md).*

## One site model (Matt, 2026-11-06; candidate)

Cities, research stations and factories follow one set of rules and differ only in output:

| Owner | City | Research station | Factory |
|---|---|---|---|
| Neutral | its energy, bought by the Buy energy action ([economy.md](economy.md)) | a neutral cube onto its megaproject ([board.md](board.md)) | nothing, except **on each Discord tip it makes 1 drone** ([war.md](war.md)) |
| Player | its energy to its owner | `owned_station_cubes` research cubes in its owner's color | `factory_chips` chips to its owner |
| Adversary | its energy diverted: `divert_discord` Discord a unit | `station_cubes` adversary compute (in the adversary's compute step) | 1 adversary troop on its hex |

- **One opposing piece blocks a site's output while it stands.** An agent on a neutral or player
  site, a player ambassador on an adversary site: that round the site makes nothing, and the piece stays
  (137; 139 dropped the dial that used it up). On a
  clean or aligned city this is the **dark** city (r37, r38): nothing to buy, no energy, and an aligned one sways
  `ambassador_discord` (0) a unit.
- **Your own ambassador is armor:** when your node, or a site you own, would be destroyed, your ambassador on its hex
  may be used up instead, the owner choosing which protection to spend, or none ([war.md](war.md), *Destruction*).
  Only your own guards your piece (r75, Matt's ruling 5): another player's never does, and a neutral site has none.
  One of your ambassadors on your site's hex can cover the site or your node there. Against troops it answers a
  strike from next door; a troop standing on the hex removes ambassadors first, one a step (175, [war.md](war.md)).
- **Claim and Liberate share one payment rule** (r75, Matt's ruling 2): the bill is the site's price plus one for
  each agent on it (they cancel); **your own ambassadors standing on the site pay first, then your supply**, never
  another player's. Both need the site in reach; a botnet on or next to it stops neither.
- **Claiming** (the Claim action, [economy.md](economy.md)): a neutral site in reach with no embargo on it,
  `claim_city` 5, `claim_station` 3, `claim_factory` 3 (Liberate's prices). Your cube goes on.
  Difficulty lowers only the adversary's numbers. The bound city at setup is free ([board.md](board.md)).
  **A claim pays back in about 3 rounds** (142, Matt): a player's station makes `owned_station_cubes` 2 cubes and a
  factory `factory_chips` 2 chips a round (a neutral station `station_cubes` 1). Give / get at `values.json`: 3
  ambassadors (12) for 2 chips a round (4) or 2 cubes a round (6) pays back in 3 or 2 rounds.
- **Claiming a station with energy** (142): Power Purchase Agreement (an event: 2 credits and 2 energy) and Microgrid
  Lab Compact (a building: Action, pay 2 energy) claim a neutral research station in reach as the Claim action does
  (`claim_station`; 148, Matt): the energy replaces the `claim_station` ambassadors, each agent on it
  still costs one of yours (they cancel), paid as the action pays (your ambassadors on the station first, then your
  supply), and it needs the site in reach with no embargo. 2 energy (9)
  is cheaper than 3 ambassadors (12).
- **Liberating an adversary site** (the Liberate action) takes massed ambassadors and **makes it yours** (r75, Matt's
  ruling 2; it turned neutral): `liberate_at` 5 and `free_city` 5 Unity for a city, `liberate_facility_at` 3 and
  `free_facility` 2 Unity for a station or factory, plus one for each agent on it. The agents cancel, the adversary's
  cube comes off and yours goes on, as a claim's does. It needs the site **in reach** (your node on or next to it:
  getting close is the risk); a botnet on it stays. No agents land on it. Its factories keep their owners. A
  **liberate card** ([player-cards.md](player-cards.md), *The liberate capstones*) liberates a city exactly so, for
  its own price instead of the bill: the agents come off, Unity sways and your cube goes on. The price is the same all
  game (r67; late strength comes from cards).

## Moving pieces: Gather and Push (r70; Matt, 2026-11-09, *Moving pieces*; numbers candidates)

Pieces on the board move: **ambassadors** (any seat's), **agents** and **troops**. Botnets and corruption markers don't.

- **Gather up to N** pieces into a land hex in your reach, each from a hex next to it, one at a time; **Push up to N**
  pieces from a land hex in your reach, each to a land hex next to it, the player choosing where. "Up to": stop
  after any piece, so a move is never forced into a bad one. Distance 1. The card names which pieces it moves.
- **Arrivals:** a moved piece arrives as any does. An agent arriving on standing ambassadors cancels one of them; an
  ambassador arriving on agents cancels one; a firewalled hex takes no agent; no troop enters the Protector's node
  (Wall). Troops otherwise just move.
- **A strike** (Kettle Operation, Lure Network's Action, Honeypot Relay) moves nothing: "remove up to N Agents (or
  Troops) on or next to a hex in reach, your choice": the hex itself counts, and the player picks the pieces, batch by
  batch, and may stop.
- **Where:** Kettle Operation, Lure Network, Honeypot Relay (strikes), Checkpoint Network (push; Action: push),
  Exodus Corridors and Rally the Allies (gather ambassadors), and the Protector's Patrol power (1 energy: push up to 3 agents or
  troops). What a card moves is set by its price, not its tier: about 2 credits a piece (`values.json` `gather`, `push`).

## Convert and firewall (r69; Matt, 2026-11-09; candidate)

Ten tier-2 cards (drafts, [player-cards.md](player-cards.md)) act on the agents where they stand,
for the last rounds, and never free a city: the players still reach it and Liberate.

- **Convert** (Mass Defection, Double Agents, Open Source Uprising, Embassy of Exiles' Action): up to N agents on or next to one city in your reach leave the adversary's stack (to its supply) and become
  your ambassadors **where each stood**. All the converted agents come off first, then the ambassadors go on, so two
  converted on one hex both stay; an agent the card left on that hex meets one of them, 1 for 1, as any contact
  does. A converted ambassador is on the board, not in your supply: it is armor or a blocker, and moves like
  any other; standing on the city, it pays toward a Liberate there.
- **Firewall** (Firewall Array's Action, Air Gap): a token on one land hex in your reach. While it stands no
  agent arrives there: the site's Rally share stays in the supply (175: one rule for a blocked landing), a creation
  line takes the next site, a threat's or Discord card's placement there doesn't happen, and a Corrupt line moves no
  agent onto it. Nothing on it corrupts: a Corrupt line, natural corruption, a station flip (Station Coup, Lab
  Takeover, Station Seizure: a forced flip too) and a threat corrupting its city (Overrun, Regime Change) all skip it
  (r79, Matt's ruling on D77, candidate; agents already on it stay). Troops are not stopped. It comes off at the end of the next adversary turn (its compute step), so it
  lasts one round; the building places a fresh one each round.

A **base** is no site: it is a front's troop pile ([war.md](war.md)). A player ambassador standing on a base is used
up instead of the next drone made there; an agent on one does nothing.
A **node**'s hex corrupts at `corrupt_node_at` (2) agents, whatever else stands there (175, [adversary.md](adversary.md)):
the node is lost unless something absorbs it, and the agents go to the supply. One agent on it costs nothing.

- **Every contact is 1 for 1** (r75, Matt's ruling 1). An agent arriving (a Rally, a creation line, a Corrupt line's
  move, a Gather or Push, a fallback landing) on a hex holding player ambassadors cancels one of them; with several
  players' there, the players choose whose (seat 0 asks for the team). An ambassador arriving on agents cancels one.
  Both go (the agent to the adversary's supply), with no Unity, so a position is held only until the next arrival.
  Claim and Liberate pay one ambassador for each agent on the site, the same rate.
- **Remove agents** (the standard action, [economy.md](economy.md)): spend `remove_ambassadors` (1) ambassador from
  your supply, used up, and remove up to `remove_count` (3) agents in reach, one at a time from any hexes in reach,
  your choice, stopping when you like. It places nothing (r75; it stood on one hex and removed 3 there). The supply
  caps it: each use needs a new ambassador.
- **Consumed or standing:** claiming, liberating, Remove agents, Envoy, armor and the troop rule use a player
  ambassador up; blocking and moving leave it standing. A consumed ambassador is gone: new ones come from cards,
  Followers and the standard project. Player ambassadors move through cards (any player's Gather of
  ambassadors moves yours too) and Dispatch. A Dispatch, like a card's "move up to N ambassadors", puts one onto a
  piece in your reach, taken from your supply or from any hex on the board, however far; it is free, and a seat has
  its board's Dispatches a round ([ai-boards.md](ai-boards.md)). A Dispatch may also put one on a botnet's hex in
  reach, where the Take down a botnet action can use it (it stays).

Agents can stand with no piece under them (on an empty industrial zone; on a hex whose node was lost) until removed.

**Your supply has no limit** ([economy.md](economy.md), *No limits*). Orbital Embassy gives 1 ambassador a
Diplomacy tag on each card you play (127; at 1 a tag of any kind it piled 52-84 in long games).

**For new players.** Every city, research station and factory belongs to a player, to the adversary, or to nobody,
and makes something for its owner. Spend your ambassadors to claim a neutral one or to take one back from the
adversary: 5 for a city, 3 for a station or factory either way, one more for each agent sitting on it; your own
ambassadors standing on it pay first. Either way it becomes yours, and taking one back also sways Unity your way. A
station or factory you own makes 2 a round, so a claim pays back in about 3 rounds; a few cards claim a station for
energy instead. Every meeting of an agent and an ambassador is one for one. Stand one ambassador on an adversary site
to shut it off, and clear agents off yours, which they shut off the same way. An ambassador of yours on your own piece
can take a hit for it.
