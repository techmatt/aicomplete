# Ambassadors and sites

*The one site model (cities, research stations, factories), claiming, liberating, blocking, armor, canceling and evicting. Terms and conventions are in [README.md](README.md).*

## One site model (Matt, 2026-11-06; candidate)

Cities, research stations and factories follow one set of rules and differ only in output:

| Owner | City | Research station | Factory |
|---|---|---|---|
| Neutral | sells its energy ([economy.md](economy.md)) | a neutral cube onto its megaproject ([board.md](board.md)) | nothing, except **on each Discord tip it makes 1 corp troop** ([war.md](war.md)) |
| Player | its energy to its owner | `owned_station_cubes` research cubes in its owner's color | `factory_chips` chips to its owner |
| Adversary | its energy diverted: `divert_discord` Discord a unit | `station_cubes` adversary compute (in the adversary's compute step) | 1 adversary troop on its hex |

- **One opposing piece blocks a site's output while it stands.** An agent on a neutral or player
  site, a player ambassador on an adversary site: that round the site makes nothing, and the piece stays
  (137; 139 dropped the dial that used it up). On a
  clean or aligned city this is the **dark** city (r37, r38): no sale, no energy, and an aligned one sways
  `ambassador_discord` (0) a unit.
- **Your own ambassador is armor:** when your node, or a site you own, would be destroyed, your ambassador on its hex
  is used up instead ([war.md](war.md), *Destruction*; a neutral site has none). One ambassador on your site's hex
  covers the site and your node there. Against troops it answers a strike from next door; a troop standing on the
  hex removes ambassadors first, one a step (175: ambassadors are non-combatants, [war.md](war.md)).
- **Claiming is symmetric** (the Claim action, [economy.md](economy.md)): a neutral site in reach with a site claim
  ([README.md](README.md#terms)), for as many ambassadors as the adversary's Corrupt X needs for that type:
  `claim_city` 4 (Corrupt city's `corrupt_at` 4), `claim_station` 3 and `claim_factory` 3 (139, Matt: Corrupt
  station's and factory's 3), plus one for each agent on it, which cancel. Your cube goes on.
  Difficulty lowers only the adversary's number. The bound city at setup is free ([board.md](board.md)).
  **A claim pays back in about 3 rounds** (142, Matt): a player's station makes `owned_station_cubes` 2 cubes and a
  factory `factory_chips` 2 chips a round (a neutral station `station_cubes` 1). Give / get at `values.json`: 3
  ambassadors (12) for 2 chips a round (4) or 2 cubes a round (6) pays back in 3 or 2 rounds.
- **Claiming a station with energy** (142): Power Purchase Agreement (an event: 2 credits and 2 energy) and Microgrid
  Lab Compact (a building: Action, pay 2 energy) claim a neutral research station in reach as the Claim action does
  (`claim_station`; 148, Matt): the energy replaces the `claim_station` ambassadors, each agent on it
  still costs one of yours (they cancel), and a site claim is needed (no botnet next to it, no embargo). 2 energy (9)
  is cheaper than 3 ambassadors (12).
- **Liberating an adversary site** (the Liberate action) takes massed ambassadors and turns it
  neutral: `liberate_at` 5 and `free_city` 5 Unity for a city, `liberate_facility_at` 3 and `free_facility` 2 Unity
  for a station or factory, plus one for each agent on it, which cancel. **The ambassadors standing on the site pay
  first, any seat's** (r70; Matt, 2026-11-09; candidate): the acting seat's supply pays the rest. It needs the site **in
  reach** (your node on or next to it: getting close is the risk) but no site claim: a botnet next to it doesn't stop
  it, and a botnet on it stays. A liberated city turns clean and takes `free_leaves` agents back (r67:
  0, Matt, 2026-11-08: none; it was 1). Its factories keep their owners. A liberate card ([player-cards.md](player-cards.md),
  *The liberate capstones*) liberates a city the same way, for its own price (r68: no Unity tier III does). The price is
  the same all game (r67: 154's late liberation, 3 ambassadors from stage III, is gone; late strength comes from
  cards instead).

## Moving pieces: Gather and Push (r70; Matt, 2026-11-09, *Moving pieces*; numbers candidates)

Pieces on the board move: **ambassadors** (any seat's), **agents** and **troops**. Botnets and corruption markers don't.

- **Gather up to N** pieces into a land hex in your reach, each from a hex next to it, one at a time; **Push up to N**
  pieces from a land hex in your reach, each to a land hex next to it, the player choosing where. "Up to": stop
  after any piece, so a move is never forced into a bad one. Distance 1. The card names which pieces it moves.
- **Arrivals:** a moved piece arrives as any does. An agent arriving on a standing ambassador cancels both; an
  ambassador arriving on agents cancels one; a firewalled hex takes no agent; no troop enters the Protector's node
  (Wall). Troops otherwise just move.
- **A strike** (Kettle Operation, Lure Network's Action) removes up to N agents or troops next to the hex instead of
  landing them, troops first, the player choosing only the hex, so it never raises a Liberate bill.
- **Where:** Kettle Operation, Lure Network, Honeypot Relay (strikes), Checkpoint Network (push; Action: push),
  Exodus Corridors and Rally the Allies (gather ambassadors), and the Protector's Patrol power (1 energy: push up to 3 agents or
  troops). What a card moves is set by its price, not its tier: about 2 credits a piece (`values.json` `gather`, `push`).

## Convert and firewall (r69; Matt, 2026-11-09; candidate)

Ten tier-2 cards (drafts, [player-cards.md](player-cards.md)) act on the agents where they stand,
for the last rounds, and never free a city: the players still reach it and Liberate.

- **Convert** (Mass Defection, Double Agents, Open Source Uprising, Embassy of Exiles' Action): up to N agents on or next to one city in your reach leave the adversary's stack (to its supply) and become yours, on
  **the node whose reach the card used** (a node on or next to the city; you pick among several). Each one taken off
  the city lowers its Liberate bill by 1; it arrives as any ambassador does (onto an agent there, the
  two cancel). A converted ambassador is on the board, not in your supply: it is armor or a blocker, and moves like
  any other; standing on the city, it pays toward a Liberate there.
- **Firewall** (Firewall Array's Action, Air Gap): a token on one land hex in your reach. While it stands no
  agent arrives there: the site's Rally share stays in the supply (175: one rule for a blocked landing), a creation
  line takes the next site, a threat's or Discord card's placement there doesn't happen, and a Corrupt line moves no
  agent onto it (it corrupts only on what it already holds). Troops are not stopped. It comes off at the end of
  the next adversary turn (after its income), so it lasts one round; the building places a fresh one each round.

A **base** is no site: it is a front's troop pile ([war.md](war.md)). A player ambassador standing on a base is used
up instead of the next corp troop made there; an agent on one does nothing.
A **node**'s hex corrupts at `corrupt_node_at` (2) agents, whatever else stands there (175, [adversary.md](adversary.md)):
the node is lost unless something absorbs it, and the agents go to the supply. One agent on it costs nothing.

- **An agent and an ambassador cancel.** When an agent arrives on a hex holding a player ambassador, or an
  ambassador on agents, both are removed (the agent to the adversary's supply), with no Unity. **Two
  arrivals don't cancel:** the ambassador the Remove action moves evicts what stands there
  and stays. Any other move onto agents cancels one, consuming the mover:
  moved ambassadors don't evict, or one ambassador a hop would clear the board.
  An agent arriving on a standing player ambassador cancels it, so a
  position is held only until the next arrival.
- **Consumed or standing:** claiming, liberating, Envoy, armor and the troop rule use a player ambassador up;
  evicting, blocking and moving leave it standing. A consumed ambassador is gone: new ones come from cards, Followers
  and the standard project. Player ambassadors move only through cards, Remove and Move; Move is free and paid from
  the board's Movement, so a seat makes as many free moves a round as its Movement ([ai-boards.md](ai-boards.md)).

**Evicting is capped by supply:** Remove takes the ambassador from your supply, never from
the board, so each eviction needs a new ambassador; that, and pieces filling up, cap the
loop. Agents can stand with no piece under them (on an empty
industrial zone; on a hex whose node was lost), so an evicting ambassador can too: it does
nothing there until a piece comes or it moves. An agent left on a hex by a
lost node stays until removed.

**Your supply has no limit** ([economy.md](economy.md), *No limits*). Orbital Embassy gives 1 ambassador a
Diplomacy tag on each card you play (127; at 1 a tag of any kind it piled 52-84 in long games).

**For new players.** Every city, research station and factory belongs to a player, to the adversary, or to nobody,
and makes something for its owner. Spend your ambassadors to claim a neutral one (4 for a city, 3 for a station or
factory, as many as the adversary needs to corrupt it; a few cards claim a station for energy instead; a station or
factory you own makes 2 a round, so a claim pays back in about 3 rounds) or to take one back from the adversary (5 for a city, 3 for a station or factory: Unity sways your way; any ambassadors standing on it pay first), one
more for each agent sitting on it. Stand one ambassador on an adversary site to shut it off, and clear
agents off yours, which they shut off the same way. An ambassador of yours on your own piece takes a hit for it.
**What resisted this summary:** a botnet next to a site stops a claim but not a liberation.
