# Ambassadors and sites

*The one site model (cities, research stations, factories), claiming, liberating, blocking, armor, canceling and evicting. Terms and conventions are in [README.md](README.md).*

## One site model (Matt, 2026-11-06; candidate)

Cities, research stations and factories follow one set of rules and differ only in output:

| Owner | City | Research station | Factory |
|---|---|---|---|
| Neutral | sells its energy ([economy.md](economy.md)) | a neutral cube onto its megaproject ([board.md](board.md)) | nothing, except **on each Discord tip it makes 1 corp troop** ([war.md](war.md)) |
| Player | its energy to its owner | `owned_station_cubes` research cubes in its owner's color | `factory_chips` chips to its owner |
| Adversary | its energy diverted: `divert_discord` Discord a unit | `station_cubes` adversary compute | 1 adversary troop on its hex |

- **One opposing ambassador blocks a site's output while it stands.** An adversary ambassador on a neutral or player
  site, a player ambassador on an adversary site: that round the site makes nothing, and the ambassador stays
  (137; 139 dropped the dial that used it up). On a
  clean or aligned city this is the **dark** city (r37, r38): no sale, no energy, and an aligned one sways
  `ambassador_discord` (0) a unit.
- **Your own ambassador is armor:** when your node, or a site you own, would be destroyed, your ambassador on its hex
  is used up instead ([war.md](war.md), *Destruction*; a neutral site has none). One ambassador on your site's hex
  covers the site and your node there.
- **Claiming is symmetric** (the Claim action, [economy.md](economy.md)): a neutral site in reach with a site claim
  ([README.md](README.md#terms)), for as many ambassadors as the adversary's Corrupt X needs for that type:
  `claim_city` 4 (Corrupt city's `corrupt_at` 4), `claim_station` 3 and `claim_factory` 3 (139, Matt: Corrupt
  station's and factory's 3), plus one for each adversary ambassador on it, which cancel. Your cube goes on.
  Difficulty lowers only the adversary's number. The bound city at setup is free ([board.md](board.md)).
  **A claim pays back in about 3 rounds** (142, Matt): a player's station makes `owned_station_cubes` 2 cubes and a
  factory `factory_chips` 2 chips a round (a neutral station `station_cubes` 1). Give / get at `values.json`: 3
  ambassadors (12) for 2 chips a round (4) or 2 cubes a round (6) pays back in 3 or 2 rounds.
- **Claiming a station with energy** (142): Power Purchase Agreement (an event: 2 credits and 2 energy) and Microgrid
  Lab Compact (a building: Action, pay 2 energy) claim a neutral research station in reach as the Claim action does
  (`claim_station`; 148, Matt): the energy replaces the `claim_station` ambassadors, each adversary ambassador on it
  still costs one of yours (they cancel), and a site claim is needed (no botnet next to it, no embargo). 2 energy (9)
  is cheaper than 3 ambassadors (12).
- **Liberating an adversary site** (the Liberate action) takes massed ambassadors from one seat's supply and turns it
  neutral: `liberate_at` 5 and `free_city` 5 Unity for a city, `liberate_facility_at` 3 and `free_facility` 2 Unity
  for a station or factory, plus one for each adversary ambassador on it, which cancel. It needs the site **in
  reach** (your node on or next to it: getting close is the risk) but no site claim: a botnet next to it doesn't stop
  it, and a botnet on it stays. A liberated city also **drops down the ladder rather than off the board**:
  `free_leaves` adversary ambassadors arrive on it as any placement does. Its factories keep their owners. A liberate
  card ([player-cards.md](player-cards.md), *The liberate capstones*) liberates a city the same way, for its own price.

A **base** is no site: it is a front's troop pile ([war.md](war.md)). A player ambassador standing on a base is used
up instead of the next corp troop made there; an adversary ambassador on one does nothing.
A **node** alone on its hex: an adversary ambassador on it costs its owner 1 insight next round and puts 1 compute on
the cheapest adversary project each production; it stays until removed.

- **Opposing ambassadors cancel.** When an ambassador arrives on a piece holding one of the
  other side, both are removed (the adversary's to its supply), with no Unity. **Two
  arrivals don't cancel:** the ambassador the Remove action moves evicts what stands there
  and stays. Any other move onto adversary ambassadors cancels one, consuming the mover:
  moved ambassadors don't evict, or one ambassador at 1 credit a hop would clear the board.
  An adversary ambassador arriving on a standing player ambassador cancels it, so a
  position is held only until the next arrival.
- **Consumed or standing:** claiming, liberating, Envoy, armor and the troop rule use a player ambassador up;
  evicting, blocking and moving leave it standing. A consumed ambassador is gone: new ones come from cards, Followers
  and the standard project. Player ambassadors move only through cards and the standard actions (Move, Remove).

**Evicting is capped by supply:** Remove takes the ambassador from your supply, never from
the board, so each eviction needs a new ambassador; that, and pieces filling up, cap the
loop. Adversary ambassadors can stand with no piece under them (on an empty
industrial zone; on a hex whose node was lost), so an evicting ambassador can too: it does
nothing there until a piece comes or it moves. An adversary ambassador left on a hex by a
lost node stays until removed.

**Your supply has no limit** ([economy.md](economy.md), *No limits*). Orbital Embassy gives 1 ambassador a
Diplomacy tag on each card you play (127; at 1 a tag of any kind it piled 52-84 in long games).

**For new players.** Every city, research station and factory belongs to a player, to the adversary, or to nobody,
and makes something for its owner. Spend your ambassadors to claim a neutral one (4 for a city, 3 for a station or
factory, as many as the adversary needs to corrupt it; a few cards claim a station for energy instead; a station or
factory you own makes 2 a round, so a claim pays back in about 3 rounds) or to take one back from the adversary (5 for a city, 3 for a station or factory: Unity sways your way), one
more for each adversary ambassador sitting on it. Stand one ambassador on an adversary site to shut it off, and clear
theirs off yours, which they shut off the same way. An ambassador of yours on your own piece takes a hit for it.
**What resisted this summary:** a liberated city takes an adversary ambassador straight back (the ladder), and a
botnet next to a site stops a claim but not a liberation.
