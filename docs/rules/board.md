# The board (module `board`, variant `corps`)

*The map: country pieces, fronts and bases, setup, research stations, the corps and their megaprojects. Terms and conventions are in [README.md](README.md).*

## Country pieces

A game has 1–4 countries, one player each. Country *i* is printed piece *i*
([`content/map.json`](../../content/map.json)), a 4×4 piece of 16 hexes placed by the
layout for that country count. Population is implicit: people live in cities.

Hex kinds: **city**, **open** land and **ocean** are printed; a **military base** goes
on an open hex of each front (below). **Industrial zones** are a printed marking on open
hexes next to cities: factories go only there. Research stations go on other open hexes,
one facility per hex. Nothing builds on ocean.

<!-- generated: countries -->
| # | Name | Cities (corrupted first) | Ocean | Station (botnet) | Neutral station | Industrial zones (setup factories) | Starting node |
|---|---|---|---|---|---|---|---|
| 0 | Horizon | 7, 6, 14 | 3, 12 | 4 | 10 | 2, 8, 11, 13 (2, 8) | 9 |
| 1 | Cintamani | 10, 7, 15 | 4 | 12 | 2 | 3, 14, 9, 16 (3, 14) | 6 |
| 2 | Onyx | 7, 2, 10 | — | 14 | 12 | 1, 6, 9, 15 (1, 6) | 11 |
| 3 | Biomia | 11, 6, 14 | 1 | 12 | 9 | 7, 13, 3, 8 (7, 13) | 5 |
<!-- /generated -->

Ocean hexes are on the board's outer edge in every layout the piece appears in; the
starting node touches a city.

**Fronts are printed** (Matt): a front is a long edge shared by two country pieces, marked
on the tiles; pieces that only touch at the middle of the map are not neighbors. Each
layout lists its fronts (`fronts` in `content/map.json`): 2 countries, one front; 3
countries (top, right, bottom-left), all three pairs; 4 countries (top, left, right,
bottom), a ring: top–left, left–bottom, bottom–right, right–top (top–bottom and left–right
touch only at the center). So at 3 and 4 countries every country has exactly two fronts (a
test checks this). Each country gets one military base per front, on an open hex that
touches the other country's land and holds no setup piece or industrial zone; a country's
fronts with the fewest candidate hexes pick first, each taking its lowest-numbered free
candidate (bases are placed by the layout, not printed, because a piece's
neighbors change with the country count).

## Setup

- Each country: its first `start_corrupted` cities get an adversary cube (more at
  difficulty 4 with `adversary.extra_corrupted`); its research station starts corrupted,
  with a **botnet** on it; its **neutral research station** on its printed hex; its two
  setup factories, **neutral** ("a factory takes its city's owner" is for
  factories cities build; a setup factory touching the corrupted city would otherwise
  start adversary-owned).
- Each player (seat *s* starts in country *s*): 1 node on the starting spot (the
  first token off their track), `start_ambassadors` player ambassador,
  `ai.start_credits` credits, `ai.start_energy` energy, and their AI board's credit
  production ([ai-boards.md](ai-boards.md)).
- **The bound city (candidate):** each player starts with one city aligned to them, their
  cube on it: the clean city of their country nearest their first node (ties: the lower hex
  number), found after the corrupted cities are placed. Its `corp_energy` cubes are the
  player's one easy energy source at the start. It is an aligned city in every way: it can
  be lost to corruption, and the factories it builds are its owner's.
- The adversary's supply: `adversary.supply_per_country` ambassadors per country.
- The adversary deck is dealt and its top card revealed face up as round 1's card.

## Research cubes and the corps (candidate)

**Country = corp: one piece, one corp (locked).** Each piece bears its corp's name, and its
megaproject is the corp's defining card.

<!-- generated: corps -->
| Corp | Megaproject | Cubes | At completion: player cube / adversary cube | Converter (as often as you can pay, unless marked) | Each adversary cube after |
|---|---|---|---|---|---|
| Horizon (geothermal and tidal energy) | Tidal Grid | 8 | 3 energy, 3 credits / 1 Sway to Discord | 1 research cube + 2 energy → 1 antimatter | lost |
| Cintamani (AI and efficiency) | Polar Compute Array | 8 | 1 drawn card, 5 credits / 1 adversary compute | 1 research cube → 2 insight | lost |
| Onyx (heavy industry, power plants and processors) | Mountain Fusion Works | 8 | 4 chips, 3 credits / 1 Sway to Discord | 1 research cube → 2 chips (once a round) | lost |
| Biomia (biopunk) | Delta Seawall | 8 | 1 Sway to Unity, 1 drawn card / 1 Sway to Discord | 2 research cubes → 1 Sway to Unity | lost |
<!-- /generated -->

- **Stations.** Every research station makes `station_cubes` research cube a round, in
  production, in its owner's color: a player's, the adversary's (corrupted), or
  **neutral**. No player starts with a station: building one (`station_chips`) or claiming
  the neutral one (an ambassador) is an investment. An adversary ambassador on any station,
  a neutral one too, takes that round's cube for the adversary and is used up. A player
  ambassador on a corrupted one takes the cube and stays ([ambassadors.md](ambassadors.md)).
- **A station is a defended investment (candidate, Matt).** Freeing a corrupted station is
  the standard action *Free a station* ([economy.md](economy.md)): `free_station_ambassadors` (2) of your
  ambassadors stand on it, no adversary ambassador does, and you pay
  `free_station_credits` (6); the ambassadors are used up and the station becomes neutral
  (a botnet on it stays, and still blocks claiming it). The adversary needs the same weight
  for yours: an effect that flips a player's station (Station Coup, Station Seizure) takes it
  only when `adversary.station_flip_at` (2) adversary ambassadors and no player ambassador
  stand on it, those ambassadors used up; short of that one adversary ambassador lands on it
  instead (a player ambassador there cancels it). A neutral station flips as before. Cards
  that free a station are priced as a station flip, a cube a round both ways (Reclamation
  Crew); factories stay cheap to free (Back-Channel Talks).
- **Until completion** a country's cubes go onto its megaproject. Cubes of any color fill it.
- **Completion** is checked once, at the end of the production step (`board.produce`),
  before upkeep and threats, so a whole round's cubes land first. A card holding its size
  or more completes, and **every cube on it pays, overflow included** (9 cubes on an 8-cube
  card pay 9 times); the payout spends them: a player cube pays its owner the card's player
  payout, an adversary cube the card's adversary payout, a neutral cube nothing. Countries
  complete in piece order.
- **After completion** that country's stations put their cubes on their owners' boards: a
  player stores **research cubes**, kept between rounds. A neutral station's cube is lost,
  **and so is an adversary cube (Matt).** Content can print an after line (the *Each
  adversary cube after* column); none does.
- **Converters.** Each completed card prints one converter, used in the action phase by any
  player with a **node in a city of that country** (Matt, r37: any city, owned or not; a node
  elsewhere in the country no longer counts), as often as the player can pay (r41; the
  printed texts say so since 118, and Cintamani's prints its 2 insight a cube); only Onyx's
  is once a round per player, its loop ([ai-powers.md](ai-powers.md)). Every player also has the cube converter, a
  standard action: 1 cube for `cube_insight` insight ([economy.md](economy.md)). A converter
  is a reason to hold a city node. Cubes
  reach a board from stations after completion, or 1 a round from the Researcher's Open Archive
  capstone; no converter makes cubes, so none feeds another within a round.
  Research cubes and converters are a resource every player reaches eventually, like
  antimatter. The Researcher's board ([ai-boards.md](ai-boards.md)) and several Science cards (cards.md) name them too.
- **Bought and carded cubes.** The Researcher's Grant power (3 credits), Field Grant (in
  a country where you own a station) and Moonshot Grant (8 cubes on any megaproject that
  isn't complete, no station needed: 8 complete one) put research cubes in your color onto an
  **incomplete** megaproject. They pay at completion like any cube and are spent then, so
  credits never reach a converter this way: a bought cube earns its card's player payout
  and brings the completion a cube nearer.

The neutral station stands next to the starting node, so a player's first
ambassador can claim it, on a hex that is a base in no layout and has no botnet next to it
in any (a test holds this). Two stations a country fill an 8-cube card in round 4; a third
speeds it: claimed, built and neutral stations all add cubes.

**For new players.** Every research station makes a cube a round: yours, the adversary's or
a neutral one. A country's cubes fill its corp's megaproject, and when the card fills, at
the end of production, every cube on it pays, overflow included: yours pay you what the
card says, the adversary's pay the adversary, neutral ones pay nothing. So claim the
neutral station or build one before the card fills. After that your stations there put
cubes on your board, and if you have a node on one of that country's cities (any city,
not only yours) you can spend them on its converter, as often as you can pay (Onyx's once a round), or
anywhere for 1 insight each. Once the card is done, the adversary's cubes pay it nothing more.
A station is worth defending both ways: to take back a corrupted one, stand 2 of your
ambassadors on it with no adversary ambassador there and pay 6 credits (the ambassadors are
spent, the station is neutral again); the adversary can take yours only once 2 of its
ambassadors stand on it and none of yours.

**What resisted this summary:** what an adversary cube pays at completion differs per card,
so it can only say "the card says"; the table above is the rule. Onyx's converter
alone keeps once a round, an exception a new player has to be told.
