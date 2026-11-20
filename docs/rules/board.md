# The board (module `board`, variant `corps`)

*The map: country pieces, setup, research stations, the corps and their megaprojects. Terms and conventions are in [README.md](README.md).*

## Country pieces

A game has 1–4 countries, one player each. Country *i* is printed piece *i*
([`content/map.json`](../../content/map.json)), a 4×4 piece of 16 hexes placed by the
layout for that country count. Population is implicit: people live in cities.

Hex kinds: **city**, **open** land and **ocean** are printed (no military bases since r139: a former base hex is
ordinary open land). **Industrial zones** are a printed marking on open
hexes next to cities: factories go only there. **Every zone prints an arrow to its city** (r140): the nearest city
of its own country (ties: the lower hex number), always one hex away and the same in every layout (the table
below); a corrupted factory pushes its agents along it ([adversary.md](adversary.md)). Research stations go on other open hexes,
one facility per hex. **Empty station sites** are printed too (137, at most one a country; Cintamani and Biomia print none): a neutral station
opens on one when a card says so (253, Matt's ruling 3: Open Research Campus on its target; the adversary's Shell Labs
on each country's lowest-numbered, where it has one), as a neutral factory opens on a free industrial zone (Industrial Commons, Factory
Drive). A neutral site a card opens is anyone's to claim, but Campus and Commons claim theirs (256). Nothing builds on ocean or on a Fallout token (no node, factory or
station) until it is cleared ([troops.md](troops.md)).

<!-- generated: countries -->
| # | Name | Cities (corrupted first) | Ocean | Station (botnet) | Neutral stations | Empty station sites | Industrial zones (setup factories) | Zone arrows (zone → its city) | Starting node |
|---|---|---|---|---|---|---|---|---|---|
| 0 | Horizon | 7, 6, 14 | 3, 12 | 4 | 10 | 5 | 2, 8, 11, 13 (2, 8, 13) | 2→6, 8→7, 11→7, 13→14 | 9 |
| 1 | Cintamani | 10, 7, 15 | 4 | 12 | 2, 11 | — | 3, 14, 9, 16 (3, 14) | 3→7, 9→10, 14→10, 16→15 | 6 |
| 2 | Onyx | 7, 2, 10 | — | 14 | 12, 3, 5 | 8 | 1, 6, 9, 15 (1, 6) | 1→2, 6→2, 9→10, 15→10 | 11 |
| 3 | Biomia | 11, 6, 14 | 1 | 12 | 9, 10 | — | 7, 13, 3, 8 (7, 13) | 3→6, 7→6, 8→11, 13→14 | 5 |
<!-- /generated -->

Ocean hexes are on the board's outer edge in every layout the piece appears in; the
starting node touches a city.

**The layouts:** 2 countries side by side; 3 countries (top, right, bottom-left), each touching both others; 4
countries (top, left, right, bottom) in a ring (top–bottom and left–right touch only at the center). A border is
only an edge: hexes are adjacent across it, and nothing is placed on it or fights over it (r139: fronts are gone).

## Setup

- Each country: the first `start_corrupted` cities of its printed corruption order get an adversary cube
  (`adversary.extra_corrupted` more at difficulty 4, but never past the 2 each piece prints); at level 0 each city
  left free takes `adversary.setup_city_agents` (4) agents from the supply (r146: one arrival short of falling, so
  the adversary spreads before the players can liberate the starting cities; the city is blocked: no energy until
  they are cleared); its research station starts corrupted,
  with a **botnet** on it; its **neutral research stations** and its setup factories, **neutral**, on their printed
  hexes (r83, Matt 2026-11-11: each city starts with at least one neutral factory and one neutral station next to it;
  this map allows it everywhere, so a country has 1-3 stations and 2-3 factories, the table above).
- Each player (seat *s* starts in country *s*): 1 node on the starting spot (the
  first token off their track), `start_ambassadors` player ambassador,
  `ai.start_credits` credits and `ai.start_energy` energy; their AI board's credit production is income, paid
  each upkeep ([ai-boards.md](ai-boards.md)).
- **No player owns a city** (254, Matt's candidate, `cities_unowned` 1): a city is the adversary's (corrupted) or
  its country's (neutral), and no one starts with one. A city that is neither corrupted nor blocked pays its energy
  in the income step to every player with a node on or next to it ([economy.md](economy.md)). Every country's start
  spot is next to a city, so a seat whose nearest city starts clean is paid from round 1.
  With `cities_unowned` 0 (r114, a screen's off arm) each player starts with the clean city of their country
  nearest their first node (ties: the lower hex number) aligned to them, their cube on it.
- The adversary's supply: `adversary.supply_per_country` agents per country.
- The adversary deck is dealt and its top card revealed face up as round 1's card.

## Research cubes and the corps (candidate)

**Country = corp: one piece, one corp (locked).** Each piece bears its corp's name, and its
megaproject is the corp's defining card.

<!-- generated: corps -->
| Corp | Megaproject | Cubes | At completion: each player cube | Converter (as often as you can pay, unless marked) | Chip converter |
|---|---|---|---|---|---|
| Horizon (geothermal and tidal energy) | Tidal Grid | 8 | 3 energy, 2 credits | 2 research cubes → 4 energy (once a round) | 1 chip → 2 energy (once a round) |
| Cintamani (AI and efficiency) | Polar Compute Array | 8 | 6 credits | 1 research cube → 2 insight (once a round) | 1 chip → 1 drawn card, 2 credits (once a round) |
| Onyx (heavy industry, power plants and processors) | Mountain Fusion Works | 8 | 2 chips, 2 credits | 1 research cube → 1 chip (once a round) | none |
| Biomia (biopunk) | Delta Seawall | 8 | 1 Sway to Unity, 3 credits | 1 research cube → 1 Sway to Unity (once a round) | 2 chips → 2 Sway to Unity, 1 drawn card (once a round) |
<!-- /generated -->

- **Stations** follow the one site model ([ambassadors.md](ambassadors.md)). In production a **neutral** one makes
  `station_cubes` (1) research cube a round, a player's `owned_station_cubes` (1; 252, Matt: every building produces 1) in its owner's color; an adversary
  station (corrupted) has **one output** (r140): `station_cubes` (1) research cube a round onto the adversary's
  research track, in its Income beat, from round 1 ([adversary.md](adversary.md)); it never feeds a megaproject.
  No player starts with a station: building one (`station_chips`, 2 chips since 252, Matt) or claiming a neutral one
  with a card is an investment. Site-opening lines (Shell Labs, the research row's New lab) open neutral stations too ([adversary.md](adversary.md)).
  One opposing ambassador or agent on a station blocks its output while it stands; the Liberate action frees an
  adversary station (left neutral) for a card to claim.
- **A station is a defended investment (candidate, Matt).** The adversary needs weight to take
  yours: an effect that flips a player's station (Station Seizure, after landing 2 agents; Station Coup instead turns a player's
  station neutral with no agent test, `take_site`) takes it
  only when `adversary.station_flip_at` (2) agents and no player ambassador
  stand on it, those agents used up; short of that one agent lands on it
  instead (a player ambassador there cancels it). A neutral station flips as before. Cards
  that free a station are priced as a station flip, a cube a round both ways (Reclamation
  Crew); factories stay cheap to free (Back-Channel Talks). A card that liberates a site does it as the Liberate
  action does, unpaid; a card that claims a neutral station or factory does it unpaid, never on a station with an agent on it
  (206).
- **Until completion** a country's player and neutral cubes go onto its megaproject; both fill it. **A megaproject
  takes no adversary cube** (r140): the adversary's cubes go only onto its research track.
- **Completion** is checked once, at the end of production (`board.produce`),
  before upkeep and threats, so a whole round's cubes land first. A card holding its size
  or more completes, and **every cube on it pays, overflow included** (9 cubes on an 8-cube
  card pay 9 times); the payout spends them: a player cube pays its owner the card's player
  payout, a neutral cube nothing. Countries complete in piece order.
- **After completion** that country's stations put their cubes on their owners' boards: a
  player stores **research cubes**, kept between rounds. A neutral station's cube is lost.
- **Chip converters** (r87, Matt: chips hoarded early pay late): three cards also print one paid in chips, used as the
  converter is (a city of the country in reach, once a round); Onyx's, which makes chips, doesn't. Give / get at
  `values.json` and the anchors: 1 chip, 4, for 2 energy 6 or a card and 2 credits 5.33; 2 chips, 8, for 2 Sway and a
  card 8.67.
- **Horizon pays energy** (262, Matt, 2026-11-17: Horizon is the energy corp, and energy binds; both its converters
  sat unused making antimatter, 0.07 and 0.00 a seat-game): 2 cubes, 8, for 4 energy 12; 1 chip, 4, for 2 energy 6. Polar Compute Array's chip converter: 1 chip for a drawn card and 2 credits.
  Megaprojects complete by round 3 in about 97% of games (199), so they open mid-game.
- **Converters.** Each completed card prints one converter, used in the action phase by any
  player with **a city of that country in reach** (r75, Matt's ruling 9: any city, owned or not; r37 asked for a
  node on the city), **once a round** per player each (r75: only Onyx's was). Every player also has the cube
  converter, a standard action: `insight_cubes` (1) cube for `cube_insight` (1) insight, `insight_per_round` (1) a
  round ([economy.md](economy.md)). A converter is a reason to reach a city. Cubes
  reach a board from stations after completion, 1 a round from the Researcher's Open Archive
  capstone or a Cryo Sample Vault (r88), or from cards (Field Grant, Particle Accelerator, Collider Consortium,
  Survey Drone Fleet); no converter makes cubes, so none feeds another within a round.
  Research cubes and converters are a resource every player reaches eventually, like
  antimatter. The Researcher's board ([ai-boards.md](ai-boards.md)) and several Science cards (cards.md) name them too.
- **Bought and carded cubes.** The Researcher's Lab II power (1 insight and 2 credits, 1 cube) and Moonshot Grant
  (8 cubes on any megaproject that isn't complete, no station needed: 8 complete one) put
  research cubes in your color onto an **incomplete** megaproject. They pay at completion like any cube and are spent then, so
  credits never reach a converter this way: a bought cube earns its card's player payout
  and brings the completion a cube nearer.

The first neutral station stands next to the starting node, within a first card's reach (since 262 only cards
claim). At setup a country's
neutral stations make 1 cube a round each (the adversary's feeds its research track instead): one alone takes 8 rounds to fill an
8-cube card, three about 3. Claiming one turns its 1 a round to its owner's color; each station a player builds there adds 1.

**For new players.** A research station makes a cube a round, neutral or yours; the adversary's put
theirs on its research track. A country's cubes fill its corp's megaproject, and when the card fills, at
the end of production, every cube on it pays, overflow included: yours pay you what the
card says, neutral ones pay nothing. So claim the neutral station with a card (Power Purchase Agreement, Microgrid Lab Compact) or build one (2 chips)
before the card fills. After that your stations there put cubes on your board, and if one of that country's cities (any city, not
only yours) is in your reach you can spend them on its converter once a round, or anywhere, 1 for 1 insight once a round. Take
back an adversary station with 4 of your ambassadors; it is left neutral, to claim after. The adversary can take yours only once 2 of
its agents stand on it and none of yours.
