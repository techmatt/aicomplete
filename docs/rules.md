# Rules as implemented

**Sections.** Read only the ones your task touches.

- [Terms](#terms): the house words, reach, site claims, pieces, ownership, the ladder and who chooses.
- [1. The board](#1-the-board-module-board-variant-corps): country pieces, fronts and bases, setup, research stations, megaprojects and converters.
- [2. Ambassadors](#2-ambassadors): what ambassadors do on each piece, consuming versus standing, liberating cities, canceling, evicting.
- [3. The adversary](#3-the-adversary-module-adversary-variant-militarist): build, spread, corrupt, botnets, the project deck, the adversary deck and round cap, difficulty.
- [4. Factories, the clock, war and troops](#4-factories-the-clock-war-and-troops): production/war clock, factories, corp troops and war, the adversary march, Wall, node loss.
- [5. Alignment and the end of the game](#5-alignment-and-the-end-of-the-game-module-alignment): Sway, tipping, Unity and Discord decks, the purge, winning and losing.
- [6. Resources and the round](#6-resources-and-the-round): each resource's sources and uses, city energy, standard actions, the round's phases.
- [7. AI boards](#7-ai-boards-module-ai-contentaisjson): the three boards, node tracks and capstones, node powers, sacrifice powers, hardening and shields.
- [8. Threats and cards](#8-threats-and-cards): threat sources, dealing and countering; the card draft, keeps, play, tags, tokens, card grammar.
- [9. Off-variants](#9-off-variants): variants and parameters that switch a module or part off for measurement.
- [Tunables](#tunables): every baseline config parameter with its value, range and meaning (generated).
- [Defaults and deviations](#defaults-and-deviations): the numbered readings chosen where the prompt or v0 ruleset left a gap.

What the code implements, at rules revision 39 (`RULES_REVISION`, `crates/aic-core/src/lib.rs`):
the Militarist against Builder, Protector and Researcher (two countries seat Builder and
Protector, three add the Researcher), 1-4 countries. The player cards are in
[cards.md](cards.md), priced in [card-pricing.md](card-pricing.md). The
AI boards, threats, the Unity and Discord decks and the project deck are **placeholders**
pending redesign. Every rule is a **candidate** unless marked **locked**; which, and what
each revision changed and why, is in [rules-history.md](rules-history.md). What content can
print is in [ops.md](ops.md).

**Every number below is a candidate**: a starting calibration reasoned for an 8–10
round game (`docs/principles.md`), for simulation to adjust. Names in `code` are config
tunables (`<module>.<param>`), with their values under [Tunables](#tunables), from
[`configs/baseline.json`](../configs/baseline.json); card, threat, adversary-card,
megaproject and AI-board numbers are content in [`content/`](../content/). Where the
prompt left a gap, the choice made is marked **Default**; where the implementation
departs from the v0 ruleset's card text, **Deviation**. Both are collected at the end.
Design intent lives in [principles.md](principles.md); the engine in
[architecture.md](architecture.md). **Keep this current with every rules change.**

<!-- generated: new_players -->
**For new players.** In the first round you are dealt 10 cards and keep 4 free (more at 5 credits each). From then on, each round you build a hand of 4 cards one at a time, choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 deck; you take one card and pass the rest to your neighbor until the hands are gone, then pay insight to keep what you drafted (1 for tier 1, 3 for tier 2). Insight is this round's attention: what you don't spend is lost. Playing a card costs its credits and 0-2 insight, from the same insight as keeps. Buildings go into your AI board's 3-5 slots; once they are full, a new building goes on top of an old one, which stops working but keeps its tags. Programs stay in front of you; events are used and discarded. Every card in front of you shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, and you can sacrifice cards with the right tags to use your AI board's powers.

**Nodes, for new players.** There is no growth step. In the action phase you place nodes with your board's two node powers, each once a round, so 0, 1 or 2 tokens a round from them, and cards can add more. Every board has 1 energy for 1 node; the second costs more, in the board's own currency. Either way you also pay the energy printed on the track space you uncover. You start with 1 energy and one city bound to you, which makes energy every round.
<!-- /generated -->

## Terms

The house words, as the player aid prints them ([player-aid.md](player-aid.md)):

<!-- generated: house_terms -->
- **Facility**: a research station or factory on the board. A building is always a card. Harden goes on a facility and nothing else.
- **Shield**: a hardening token on a hex with your node.
- **Ambassador**: yours; the other side's is always an adversary ambassador.
- **Troop**: always the adversary's.
- **In reach**: on a hex within 1 of your nodes.
- **A country you're in**: a country where you have a node.
- **Your home country**: the country you start in.
- **A threat**: on your card: one you may counter, with one of its sources in reach or your ambassador on one.
- **Here**: on a building: on this card, where its tokens sit.
- **Remove N**: as many as there are, up to N.
- **Draw**: from the tier-1 deck, with no keep to pay.
- **On your cards**: on tags: on your cards in play, covered buildings included. Tokens count only on the card that holds them.
- **Action**: once a round, always; "A, OR B" is one use, either half.
<!-- /generated -->

The rest:

- **Adjacent**: a neighboring hex on the board, across country borders too.
- **Next to X**: holding X, or adjacent to a hex holding X.
- **Node**: one of a player's tokens on the board (one per hex), taken off their AI
  board's node track (§7). A node is a token, not a facility: it sits on any land
  hex, with or beside other pieces.
- **Botnet**: the adversary's persistent token (§3), made where two adversary ambassadors
  meet on a piece. Nodes are always the players', botnets always the adversary's.
- **Reach**: every map target of a standard action is in the acting player's reach, and so
  are most of a card's. The exceptions are a card's single ambassador move and Spread
  (measured from the ambassador), Total Containment's hex, and country effects such as a
  sweep or Shelter (any country; §8). Reach is 1 unless a rule gives a reason: Embassy
  Compound's and Peacekeeper Rotation's moves go 2 (§8, moving is their point), and the
  adversary's distances (`spread_range`, the spread twists, `war_reach`) are its dials, not
  a player's reach (§3, §4). A converter is not a map target: it needs a node in a city of
  its country (§1).
- **Site claim**: aligning a city, or claiming a research station or factory, needs the
  player's node on or next to the site, and **a botnet next to the site blocks claims
  on it** (one hop from the site).
- **Piece**: a city, a military base, a facility or a node. Ambassadors sit on pieces.
- **Clean / corrupted / aligned / owned**: a city, research station or factory with no
  owner cube is clean (a station or factory: neutral); an adversary cube means corrupted
  (a factory: adversary-owned); a player cube means aligned (a city) or owned.
- **The ladder**: the adversary's pieces, weakest first: ambassadors (the explorers, used
  up when they act), **botnets** (the towns, persistent), troops, corrupted cities and
  stations (the cities). Liberating a city knocks it down the ladder, not off it (§2).
- **Players choose**: an `Action::Choose` in `legal_actions`. Team choices in the
  adversary's steps, production and threats are asked of seat 0 (an aligned city's
  factory placement, of its owner); a choice inside a player's own card, ability or
  standard action is asked of that player.
- **Hex number**: each piece's hexes are printed 1–16 row by row; ties go to the lower
  number.

## 1. The board (module `board`, variant `corps`)

### Country pieces

A game has 1–4 countries, one player each. Country *i* is printed piece *i*
([`content/map.json`](../content/map.json)), a 4×4 piece of 16 hexes placed by the
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
candidate (**Default**: bases are placed by the layout, not printed, because a piece's
neighbors change with the country count).

### Setup

- Each country: its first `start_corrupted` cities get an adversary cube (more at
  difficulty 4 with `adversary.extra_corrupted`); its research station starts corrupted,
  with a **botnet** on it; its **neutral research station** on its printed hex; its two
  setup factories, **neutral** (**Default**: "a factory takes its city's owner" is for
  factories cities build; a setup factory touching the corrupted city would otherwise
  start adversary-owned).
- Each player (seat *s* starts in country *s*): 1 node on the starting spot (the
  first token off their track), `start_ambassadors` player ambassador,
  `ai.start_credits` credits, `ai.start_energy` energy, and their AI board's credit
  production (§7).
- **The bound city (candidate):** each player starts with one city aligned to them, their
  cube on it: the clean city of their country nearest their first node (ties: the lower hex
  number), found after the corrupted cities are placed. Its `corp_energy` cubes are the
  player's one easy energy source at the start. It is an aligned city in every way: it can
  be lost to corruption, and the factories it builds are its owner's.
- The adversary's supply: `adversary.supply_per_country` ambassadors per country.
- The adversary deck is dealt and its top card revealed face up as round 1's card.

### Research cubes and the corps (candidate)

**Country = corp: one piece, one corp (locked).** Each piece bears its corp's name, and its
megaproject is the corp's defining card.

<!-- generated: corps -->
| Corp | Megaproject | Cubes | At completion: player cube / adversary cube | Converter (once a round) | Each adversary cube after |
|---|---|---|---|---|---|
| Horizon (geothermal and tidal energy) | Tidal Grid | 8 | 3 energy, 3 credits / 1 Sway to Discord | 1 research cube + 2 energy → 1 antimatter | lost |
| Cintamani (AI and efficiency) | Polar Compute Array | 8 | 1 drawn card, 5 credits / 1 adversary compute | 1 research cube → 1 insight | lost |
| Onyx (heavy industry, power plants and processors) | Mountain Fusion Works | 8 | 4 chips, 3 credits / 1 Sway to Discord | 1 research cube → 2 chips | lost |
| Biomia (biopunk) | Delta Seawall | 8 | 1 Sway to Unity, 1 drawn card, 1 credit / 1 Sway to Discord | 2 research cubes → 1 Sway to Unity | lost |
<!-- /generated -->

- **Stations.** Every research station makes `station_cubes` research cube a round, in
  production, in its owner's color: a player's, the adversary's (corrupted), or
  **neutral**. No player starts with a station: building one (`station_chips`) or claiming
  the neutral one (an ambassador) is an investment. An adversary ambassador on any station,
  a neutral one too, takes that round's cube for the adversary and is used up. A player
  ambassador on a corrupted one takes the cube and stays (§2).
- **A station is a defended investment (candidate, Matt).** Freeing a corrupted station is
  the standard action *Free a station* (§6): `free_station_ambassadors` (2) of your
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
  elsewhere in the country no longer counts), once a round per player, per card. A converter
  is a reason to hold a city node. Cubes
  reach a board only after completion, so no converter feeds another within a round.
  Research cubes and converters are a resource every player reaches eventually, like
  antimatter. The Researcher's board (§7) and several Science cards (cards.md) name them too.
- **Bought and carded cubes.** The Researcher's Grant power (3 credits) and Field Grant (in
  a country where you own a station) put research cubes in your color onto an
  **incomplete** megaproject. They pay at completion like any cube and are spent then, so
  credits never reach a converter this way: a bought cube earns its card's player payout
  and brings the completion a cube nearer.

**Default:** the neutral station stands next to the starting node, so a player's first
ambassador can claim it, on a hex that is a base in no layout and has no botnet next to it
in any (a test holds this). Two stations a country fill an 8-cube card in round 4; a third
speeds it: claimed, built and neutral stations all add cubes.

**For new players.** Every research station makes a cube a round: yours, the adversary's or
a neutral one. A country's cubes fill its corp's megaproject, and when the card fills, at
the end of production, every cube on it pays, overflow included: yours pay you what the
card says, the adversary's pay the adversary, neutral ones pay nothing. So claim the
neutral station or build one before the card fills. After that your stations there put
cubes on your board, and if you have a node on one of that country's cities (any city,
not only yours) you can spend them on its converter once a round. Once the card is done, the adversary's cubes pay it nothing more.
A station is worth defending both ways: to take back a corrupted one, stand 2 of your
ambassadors on it with no adversary ambassador there and pay 6 credits (the ambassadors are
spent, the station is neutral again); the adversary can take yours only once 2 of its
ambassadors stand on it and none of yours.

**What resisted this summary:** what an adversary cube pays at completion differs per card,
so it can only say "the card says"; the table above is the rule. A converter's
once-a-round is per player per card, which a new player reads as once a round.

## 2. Ambassadors

**An ambassador sits on a piece. The next time that piece does its normal thing, the
ambassador redirects it to its own side.** An adversary ambassador is then used up and
returns to the adversary's supply, unless it stands on a lone node or a city: on a city it redirects nothing, but
stays (a clean or aligned city goes dark; a corrupted one's energy is diverted anyway: the table). **A player ambassador
is consumed for a permanent change and stays for a position (candidate, Matt):** aligning a
city, claiming a site, freeing a station and liberating a city by the fallback consume it (no card spends one);
evicting, standing on a base, factory, station or botnet, and moving leave it on the board.
A consumed ambassador is gone: new ones come from cards, Followers and the standard project.

| On | Adversary ambassador | Player ambassador |
|---|---|---|
| City | it stays, and the city is **dark** while it stands there: a neutral city sells no energy (r37), an aligned city gives its owner none (r38; no Discord at `ambassador_discord` 0, below) | waits; a corrupted city is liberated the moment `liberate_at` player ambassadors stand on it from seats with a node on or next to it (the fallback, below; checked after every action and automatic phase) |
| Research station | that round's cube is the adversary's (on any station, a neutral one too); `station_flip_at` (2) of them on a player's station let an effect flip it (§1) | on a neutral station the player can claim: the station becomes the player's (cube placed, ambassador used); on a corrupted station, that round's cube is the player's, and it stays; 2 of them there, with no adversary ambassador, can free it (the *Free a station* action, §6: both used up) |
| Factory | on a production step, an adversary troop on the factory's hex (**locked**); used up, on an adversary-owned factory too | on a neutral factory the player can claim: the factory becomes the player's; otherwise each production step it sways `factory_unity` Unity and no corp troop is made, except that an adversary-owned factory still makes its adversary troop; it stays |
| Node | if the node is alone on its hex (on a city, base, station or factory the ambassador is that piece's), each production its owner gets 1 less insight next round and 1 compute goes onto the cheapest adversary project; it stays until removed | waits (defense, below) |
| Military base | on a production step, `base_discord` Discord instead of a troop | on a production step, `base_unity` Unity instead of a troop; it stays |

- **Liberating a city (candidate; Matt).** A corrupted city is **liberated** by a liberate
  card (§8, *The liberate capstones*) or by the **fallback**: `liberate_at` (5) player
  ambassadors on it. **Both need the liberating seat's node on or next to the city**
  (getting close is the risk): a card liberates a corrupted city in its player's reach, and
  the fallback counts only ambassadors of seats with a node on or next to it. The fallback's
  ambassadors are used up (lower seats first). Either way its adversary cube returns to the
  supply, the city is **clean** (a card's extra may put its player's cube on it: aligned),
  every liberation sways `alignment.free_city` toward Unity, and the city **drops down the
  ladder rather than off the board**: `free_leaves` adversary ambassadors arrive on it as
  any placement does (a surplus player ambassador there cancels one, and the supply caps
  them). Its factories keep their owners (§4). A face-up influence threat on the city holds
  it for both routes until it is countered.
  **The fallback is the bad route** (Matt): at `ambassador_credits` 20 each it costs about
  100 credits, for a seat with far too many. Ambassadors waiting on a city are canceled by
  the next build like any others, so the fallback is met in one action phase: gather five
  in your supply, stand next to the city, land them, plus one for each adversary ambassador
  on the city (a corrupted city keeps one through the action phase, and the first arrival
  cancels it: six, about 120 credits), unless the Remove action clears them first (its
  ambassador evicts them and stays, and counts).
- **Opposing ambassadors cancel.** When an ambassador arrives on a piece holding one of the
  other side, both are removed (the adversary's to its supply), with no Unity. **Two
  arrivals don't cancel:** the ambassador the Remove action moves evicts what stands there
  and stays. Any other move onto adversary ambassadors cancels one, consuming the mover:
  moved ambassadors don't evict, or one ambassador at 1 credit a hop would clear the board.
  An adversary ambassador arriving on a standing player ambassador cancels it, so a
  position is held only until the next arrival.
- **Defense:** a player ambassador on a facility's hex is used up instead of the facility
  when it would be destroyed.
- **Diverted** energy: a corrupted city's sways `divert_discord` Discord per unit, whatever
  stands on it. A clean or aligned city an adversary ambassador stands on is dark instead
  (§6): nothing is diverted, the energy is lost; an aligned one sways `ambassador_discord` (0) per unit each round it is dark.

Player ambassadors move only through cards and the standard actions (Move, Remove).

**Liberating a city, for new players.** A corrupted city pours Discord and adversary
ambassadors into the world until someone liberates it. You need to stand next to it: your
nodes on the city or a hex beside it. Then either play a liberate card, the tier-2 capstone
of your engine (antimatter, power and chips, credits, research cubes, Cyber, Security or
Followers), or, the expensive way, land 5 of your ambassadors on it (at 20 credits each from
the standard project), one more for each adversary ambassador standing there, since the first
cancels it. The city is clean again (some cards also put your cube on it), the
world sways 5 toward Unity, and 1 adversary ambassador lands on it. In the draft, pass a
liberate card to the player whose engine can pay for it. **What resisted this summary:** a
fallback where two players pool ambassadors counts only the ambassadors of players standing
next to the city, and an influence threat face up on the city holds it until it is countered.

**For new players.** Ambassadors you spend change the board for good: aligning a city,
claiming a station or factory, liberating a city. Ambassadors you place hold a position: one
that evicts adversary ambassadors from a piece stays on it, and on a base, factory or botnet
it keeps working every round. The next adversary ambassador to land there cancels it, so a
position lasts until the adversary answers it.

**Evicting is capped by supply:** Remove takes the ambassador from your supply, never from
the board, so each eviction needs a new ambassador; that, and pieces filling up, cap the
loop. **Default:** adversary ambassadors can stand with no piece under them (on an empty
industrial zone; on a hex whose node was lost), so an evicting ambassador can too: it does
nothing there until a piece comes or it moves. An adversary ambassador left on a hex by a
lost node stays until removed.

## 3. The adversary (module `adversary`, variant `militarist`)

### Each round

1. **Build** (`adversary.build`): next round's face-up card becomes this round's card.
   **Arrivals (candidate):** each card names a site type, and every matching site in every
   country gains `arrivals` adversary ambassadors, whatever the board holds (below). Each
   corrupted city gains `city_build` (2, Matt: the adversary makes ambassadors faster than
   the party can easily deal with) adversary ambassadors, and each factory next to a
   corrupted city gains `factory_build`. The card's twists add more. **Troop pressure where
   the Protector stands (candidate):** then, in each country where a seat with Wall has a
   node, `wall_troops` adversary troops land as any landing does (on a factory there,
   players choose which, else a city; never on a hex a Wall holds), so Wall has strikes to
   stop (§4). A stage-II card's **Escalation** then gives each country's first base (below)
   `escalation_ambassadors`. Ongoing **projects** act (below). Last, each **botnet** makes
   `botnet_compute` **adversary compute**, onto the projects; a botnet holding a player
   ambassador makes none (the ambassador stays).
2. **Spread** (`adversary.spread`): every hex holding several adversary ambassadors, fixed
   when the step starts, keeps one and sends each other one to a hex within `spread_range`.
   **Why 2:** spread is the adversary's movement, a dial on its pressure, not a player's
   reach, so the players' reach-1 rule doesn't bind it; the +1 of Long Patrols and
   Blitzkrieg is those twists' whole escalation.
   **Spread never targets an empty hex**: a destination holds a piece or is player-owned
   (node, an aligned city, an owned station or factory); it is not ocean, and it holds no
   adversary ambassador unless it is a city that isn't corrupted, clean or aligned (which
   collects any number), or a piece without a botnet (below); a hex holding nodes of a seat
   with `spread_shield` (a card's Effect, or the Sanctuary capstone) is no destination.
   **Priority:** player-owned hexes, then clean cities, then other pieces; then the nearest;
   players choose among equals. A city that is not corrupted never spreads: it holds its
   ambassadors. **A surplus with nowhere to go returns to the supply.**
3. **Corrupt** (`adversary.corrupt`): a city that is not corrupted holding `corrupt_at` (4)
   adversary ambassadors becomes corrupted (an aligned city loses its player cube); its
   ambassadors return to the supply; any player ambassadors waiting there are lost (on any
   corruption; they would free it in the same step).
4. **Reveal** (`adversary.reveal`, after `threats.deal`): the top card goes face up as next
   round's card.

**Botnets.** A botnet is made when a spreading adversary ambassador lands on a piece that
already holds one (not a city that isn't corrupted, which collects them, and not a piece
that already has a botnet): both ambassadors return to the supply and a botnet goes on the
piece. Each board starts with a botnet (on its research station). A botnet stays until
removed: it makes `botnet_compute` adversary compute a round, it **blocks site claims** (a
site with a botnet on or next to it can't be claimed), and it counts against the purge until
Unity eases it (§5). **Botnets are hard to remove (candidate, Matt):** no standard action
does it, only tier-2 effects: the tier-2 cards **Botnet Takedown** and **Singularity
Exploit** (which also discards a project) and **Breakthrough Lab**'s Action, each removing a
botnet in your reach; **Root Access**, one on or next to the city it liberates; and the
tier-2 Unity card **Decommission** (a botnet anywhere, players choose). Removing a botnet
sways `alignment.botnet` Unity (§5).

### The project deck (near-locked)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../content/projects.json), placeholders). The deck is staged like
the threat deck: `project_cards` are dealt, the cheap stage-1 projects shuffled on top of
the big stage-2 ones. `project_row` projects are face up at once; when all of them have
resolved (or been discarded), the next row is dealt.

- **Compute sources:** each botnet, `botnet_compute` a round in the build (none under a
  player ambassador); about half the threats carry compute, gained when they execute
  uncountered (§8); an adversary ambassador draining a lone node, 1 in production (§2); the
  corp cards, an adversary cube at Cintamani's completion (§1).
- **Spending:** each compute point goes onto the **cheapest** visible project (ties: the
  first in the row); content may print compute on every visible project (`compute_every`);
  none does. When a project holds its cost it **resolves**: a one-shot effect at once, or
  an **ongoing** one that acts in every round's build step from the next round on. Choices
  are the players' (seat 0; Purge the Unaligned's loss, each player's own). With no project
  left, compute is lost.
- **Cost** is printed per player (**Default**: the prompt's 12–15 and 25–30 compute are
  3-player numbers; a per-player cost keeps the timing at 2 and 4 players). The table
  prints each cost.
- **Counterplay:** a player ambassador on a botnet stops its compute and is not used up;
  Cyber's hacks drain compute from one project (Blackout Script, Zero-Click Breach, ...),
  tier-2 hacks from every visible one (Black Ice Citadel, each upkeep) or discard one with
  its compute (Killswitch Directive); the drain standard project (§6) does it for credits.

<!-- generated: projects -->
| Project | Stage | Compute per player | Resolves | Text |
|---|---|---|---|---|
| Disinformation | 1 | 5 | once | Sway 4 toward Discord. |
| Botnet Swarm | 1 | 5 | once | Every botnet produces 2 adversary ambassadors on its hex. |
| Market Crash | 1 | 5 | once | Every player loses 8 credits. |
| False Flag | 1 | 6 | once | Deal 3 extra threats. |
| Infiltration Drive | 1 | 6 | once | In each country, the uncorrupted city with the most adversary ambassadors becomes corrupted. |
| Rapid Buildout | 1 | 6 | once | The adversary builds now, then spreads. |
| Purge the Unaligned | 2 | 8 | once | Every player loses an ambassador or a node (never their last node). In each country, 1 troop lands on a factory, or a city if no factory. |
| Recruitment Drive | 2 | 9 | ongoing | Ongoing: every round, an extra adversary ambassador at every clean city. |
| Propaganda Network | 2 | 9 | ongoing | Ongoing: every round, sway 2 toward Discord. |
| Total Offensive | 2 | 9 | once | The adversary builds and spreads now; sway 2 toward Discord. |
| Armored Columns | 2 | 9 | once | In each country, 2 troops land on factories, or cities if no factories. |
| Station Coup | 2 | 10 | once | In each country, an uncorrupted research station becomes corrupted (a player's holds unless enough adversary ambassadors and no player ambassador stand on it: an adversary ambassador lands there instead). Every botnet produces 1 adversary ambassador on its hex. |
<!-- /generated -->

**Defaults:** compute goes on one point at a time, so a full project resolves before the
next point lands; Purge the Unaligned's loss is any one of the player's ambassadors (supply
or a hex) or nodes, their pick, never their last node; a node lost this way leaves the game,
as a destroyed one does (§4). Ongoing projects act in the round's build step only, not in an
out-of-turn build (Offensive, Rapid Buildout, Mass Production).

### The deck (the clock and the stager)

[`content/adversary.json`](../content/adversary.json) prints the cards below. A game deals
`stage_cards` at random per stage, each stage shuffled on its own, stage I on top: as many
rounds as cards, **the round cap** (**locked** that a cap exists and can shorten; the number
a candidate). **11 rounds through d2:** difficulty 2 swaps a stage-I card for a stage-III
card (`stage1_removed`, `stage3_added`), so the cap bites through a harder end, not a
shorter game; difficulty 3 removes a stage-II card (`stage2_removed`): 10. Some adversaries
may shorten it too. The next round's card is always face up a round ahead (peek cards show
more). Each card's twists change its round.

<!-- generated: round_cap -->
| Difficulty | Stage I cards | Stage II | Stage III | Stage II from round | Round cap |
|---|---|---|---|---|---|
| 0 | 4 | 5 | 2 | 5 | 11 |
| 1 | 4 | 5 | 2 | 5 | 11 |
| 2 | 3 | 5 | 3 | 4 | 11 |
| 3 | 3 | 4 | 3 | 4 | 10 |
| 4 | 3 | 4 | 3 | 4 | 10 |
<!-- /generated -->

<!-- generated: adversary -->
| Card | Stage | Twist | Escalation |
|---|---|---|---|
| Probing Strikes | 1 | Arrivals: coastal cities. |  |
| Factory Drive | 1 | Arrivals: industrial zones. Factories next to clean cities gain 1 adversary ambassador. |  |
| Long Patrols | 1 | Arrivals: research stations. Spread reaches 1 hex further. |  |
| War Drums | 1 | First, sway 1 toward Discord. Arrivals: each country's first base. |  |
| Mobilization | 2 | Arrivals: coastal cities. Each corrupted city gains 1 more adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Station Raids | 2 | Arrivals: research stations. Each corrupted research station gains 1 adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Fifth Column | 2 | Arrivals: coastal cities. Cities corrupt at 1 fewer adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Border War | 2 | First, each player destroys 1 of their nodes. Arrivals: each country's first base. This round is a war round. Escalation: each country's first base gains an adversary ambassador. | yes |
| Supply Lines | 2 | Arrivals: industrial zones. Factories next to clean cities gain 1 adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Total Mobilization | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: industrial zones. Each corrupted city gains 1 more adversary ambassador; factories next to clean cities gain 1 too. |  |
| Blitzkrieg | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: each country's first base. This round is a war round, and spread reaches 1 hex further. |  |
| Occupation | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: research stations. Cities corrupt at 1 fewer adversary ambassador, and each corrupted research station gains 1 adversary ambassador. |  |
| Iron Doctrine | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: coastal cities. This round is a war round. |  |
<!-- /generated -->

**Arrivals: the explore step that never stops.** The site types are **coastal cities**,
**industrial zones** (built on or not: an ambassador may stand on an empty zone, the one
exception to "ambassadors sit on pieces"), **research stations** (whoever owns them) and
**each country's first base**. A coastal city is one next to ocean or printed coastal on its
piece (`coast` in `content/map.json`): every piece has two coastal cities. Horizon gets both
from its ocean, Cintamani prints one more, Onyx and Biomia print both. Next round's card is
face up, so its site type is visible a round ahead. Each stage's cards spread the types.
**Default:** an out-of-turn "the adversary builds now" brings the card's arrivals too, in its
scope: one rule for a build.

**A country's first base (candidate; `base_sites_per_country`):** its lowest-numbered
military base. Arrivals and Escalation reach only that one, so what reaches bases counts
countries, not fronts (a country has one front at 2 countries and two at 3 and 4). The count
is fixed by the printed board, so it can't feed back. A base's other rules hold: an
ambassador that spreads onto a second base still diverts it, and a threat that lands on
bases here (Total War) lands on all of them.

**For new players.** An adversary card may name military bases, and stage II cards send an
extra ambassador to them. Either way it lands on one base in each country, the
lowest-numbered. A base with an ambassador on it swings the world toward Discord when it
next produces, so guard your first base. **What resisted this:** "first base" is a new term
for a printed number, it only means anything at 3 and 4 countries, and an ambassador can
still spread onto the second base.

**The supply is a cap.** A placement the supply can't make doesn't happen; running out
doesn't lose.

Players **lose** when a round starts with no card to turn face up (after the round cap's
last round): a quiet backstop nobody plans around.

### Difficulty (`difficulty`, 0–4; each level includes the ones below)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules |  |
| 1 | Extra threats each round in all, countries in turn | `extra_threats` 1, `counter_bonus` 0, `counter_bonus_all` 0 |
| 2 | A stage-I card swapped for a stage-III card: stage II (escalation, tier-2 threats) sooner and a harder end, at the same round cap (the table above) | `stage1_removed` 1, `stage3_added` 1 |
| 3 | Larger builds: each corrupted city gains more, and a stage-II card removed (a shorter round cap) | `build_bonus` 1, `stage2_removed` 1 |
| 4 | A still shorter Discord deck: the removed cards count as tips already made, so every printed step still acts; tier-2 threats sooner; optionally more corrupted cities per country at setup | `discord_removed` 1, `tier2_early` 1, `extra_corrupted` 0 |
<!-- /generated -->


The difficulty is only the configured level: Discord tips don't raise it (§5). **The
fallback's `liberate_at` is the same at every difficulty:** difficulty comes from the
adversary and the decks: Discord deck size, threat strength and the round cap. The target
is set in `docs/principles.md`; the current reads are indexed in
[experiments/history.md](../experiments/history.md).

## 4. Factories, the clock, war and troops

### The production/war clock

A printed round track (`board.clock`) says what each round's production holds: 1 a
**production step**, 2 a **war step**, 3 both; the last entry repeats. Cards with the War
twist (Border War, Blitzkrieg, Iron Doctrine) make their round a war step too. **Default**
(Matt confirmed): the clock gates only factories, troops, chips and war; the every-round
income (cities' energy, the node track, stations, upkeep) and the adversary troops' march
run every round.

### Factories

- **Ownership:** an owner cube, as stations and cities have. A factory a city builds: an
  aligned city's is that player's, a corrupted city's is the adversary's, and a clean
  city's goes to **the player whose node is next to the zone the players picked** (Matt:
  the most nodes on or next to it; players choose on a tie, as for city energy), neutral
  only if no node is there. A player ambassador claims a neutral factory as it claims a
  station (§2, needs a site claim). Cards that build a factory (Prefab Plant Kit) build the
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
  adversary troop on its hex instead (**locked**). A player ambassador on a factory sways
  `factory_unity` Unity and no corp troop is made; on an adversary-owned factory it sways
  the Unity and the troop is still made. Either way it stays.
- **Chips**: each player-owned factory makes `factory_chips` chip for its owner on each
  production step.

### Troops and war

**Corp troops.** There are no player troops. Troops belong to the megacorp of their
territory and wait at its bases.

- **Peacetime:** at most `unit_cap` troops wait at each base. A troop at a base **absorbs
  an adversary-troop strike** on a facility on or next to the base: 1 troop per hit.
- **On a war step**, on each front the troops at the two bases cancel 1 for 1. The surplus
  crosses, acts at once and is used up: each destroys the rival corp's factory nearest its
  base (not an adversary-owned one; ties: lower hex number), **and any node on its hex**:
  the node goes with the factory, unabsorbed, and only the factory sways. All troops at the
  front are spent.

**Adversary troops (candidate): the march.** Adversary troops come from adversary
ambassadors on factories, from adversary-owned factories (production steps), and from
projects and threats (Armored Columns, Orbital Strike, Total War). They are the adversary's
mobile threat to player infrastructure. **Targets:** player-owned research stations and
factories, **not nodes** (`troop_node`; a troop on or next to a node only stands there).
Cities are never targets (corruption is the adversary's way into cities), nor are neutral or
adversary-owned facilities.

In the adversary phase (`board.march`, after the corruption step) **every adversary troop
takes `troop_steps` steps**, one troop at a time, in hex-number order fixed when the march
starts. A step is one of:

- **destroy** a target on or next to its hex (players choose which), or
- with none there, **move** one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose: a choice among bad options, as with Spirit Island's blight.

**Wall (candidate, Matt).** Adversary troops can't enter a hex with the Protector's node: a
march goes around it, and a landing skips it. **Nor can a troop strike into it (Matt):** a
facility on a hex with the Protector's node is no target, so it stands until the node goes.
A troop already standing on a hex when the Protector places there leaves as it likes. The
build's `wall_troops` landing (§3) puts the strikes Wall stops where the Protector stands.
**Default:** a landing enters a hex, so it counts; a troop made by a factory on its own hex
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
factories (and from some threats and projects), and every round, in the adversary's turn,
each troop takes two steps. If a research station or factory you own is on or next to the
troop's hex, it destroys that piece and leaves the board; otherwise it walks one hex toward
the nearest such piece. So a troop two hexes from your piece hits it next round, and one
three or four hexes away a round later; the arrows on the board show where each troop is
heading. Troops never attack cities. To stop a hit, remove the troop with a card, keep a
corp troop at a military base next to the target, harden the target, stand an ambassador on
it, or put the Protector's node on it or in its way. Troops don't hit nodes.

**This is the game's one movement rule** (an exception to "no movement rules"): spread
places ambassadors, it does not walk them. **A Discord tip doesn't move troops**, and
`board.troop_steps` 0 is the off-switch (§9): troops never move or strike.

**Node destruction.** A player with no nodes on the board loses the game for everyone (§5).
**It is a slide, not a cliff:** a seat loses tokens a few at a time from the middle of the
game on, from several sources: uncountered threats (Eviction offers it as a choice;
Agitators, Orbital Strike and Defection in tier 1 and Station Seizure, Overrun, Sleeper
Cells, Insurrection in tier 2 make each player here destroy one of theirs here; Manhunt and
Siege two), a city that becomes corrupted (nodes on its hex and, by `corruption_reach`, next
to it), war that destroys a factory (nodes on its hex and, by `war_reach`, next to it:
0, an adversary dial kept below the players' reach 1, so war takes only the factory's hex),
stage-III adversary cards (each player destroys two) and Border War (one), Discord cards,
and Purge the Unaligned. **Discord steps:** the tier-2 cards and the tier-1 card left for
the last tip make each player destroy nodes of theirs at their printed steps (§5; the
brief's 1, 2, 3; **Candidate**). A hit is each player's own pick of which token, and it is
absorbed as any destruction is (below), so a token shielded by an ambassador is a good pick.
**While this round's adversary card is stage I, a player's last node can't be destroyed**
(candidate, **Early fragility**): each player starts with 1, and a war in round 2 would
otherwise end the game. Eviction goes further: it takes a node only from a player with 2 or
more, at any stage. Offensive's target, on the board, is the node hex with the most
adversary ambassadors and troops (ties: country, then lower hex number). Purge the Unaligned
never offers a last node. A city that corrupts takes the node unabsorbed: corruption is not
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
so the reward can't be farmed: a token lost costs far more than a Sway. **Default:** the
brief names the marker and its caps, not what it does; this is the smallest reading that
makes it matter.

**Destruction.** Only war sways Discord: a factory destroyed by the corp troops crossing a
front sways `destruction_discord` Discord. An adversary troop's strike (in the march) sways
nothing (Matt). A destruction is absorbed, in this order, by a shelter on the country, a
corp troop at a base next to it (adversary strikes only), a hardening token on the hex, a
player ambassador on the hex, or Aegis Dome Array or the Aegis Doctrine capstone (once a
round, for their owner's countries). A destroyed node leaves the game (above). A destroyed
station or factory takes its cube with it.

## 5. Alignment and the end of the game (module `alignment`)

Alignment is where the AIs' world leans: toward **Unity** (the good AIs, the players) or
**Discord** (the evil AI, the adversary). Everything on it is printed: Sway tokens on three
spaces, two tip-count tracks and two small decks of one-shot cards.

**The track.** Sway sits in three places: **the Undecided** (the middle pool), the **Unity
bucket** and the **Discord bucket**. The Undecided starts with `undecided_per_player` ×
players Sway; both buckets start empty. **Candidate:** the pool is exactly two tips.
**Swaying** toward a side moves Sway one at a time from the Undecided into that side's
bucket. With the Undecided empty, the variant decides:

| Variant | Sway with the Undecided empty |
|---|---|
| `tug` (default; **locked**: empty-pool rule B) | comes from the opposite bucket (a tug of war); with that empty too, nothing |
| `still` | nothing happens |
| `off` | (for measuring Alignment, §9) no Sway ever moves and nothing tips |

**Tipping.** The Unity bucket holding `tip_per_player` × players Sway, or the Discord bucket
holding `discord_tip_per_player` × players, **tips**, in this order: all the bucket's Sway
returns to the Undecided; that side's tip count goes up by one; one card of its side is
drawn and resolves (at the step printed for the tip count before this tip, below).
**Default:** the bucket empties before the card, so a card's own Sway counts. Unity tips and
Discord tips are counted separately and only count up; there is no net meter. **Default:**
buckets are checked when an action or automatic phase finishes resolving (with win and loss,
below), so a bucket may overfill within one action. If both are full, Discord tips first; a
tip's card may fill a bucket again, which then tips too.

**Momentum** (`momentum_every`, 0 = off): a side's bucket holds 1 fewer Sway for every
`momentum_every` tips that side has made, down to 1 per player.

**Sources of Sway**

<!-- generated: sway_sources -->
| Toward | From | Amount |
|---|---|---|
| Unity | a player ambassador on a producing base / factory | `base_unity` 2 / `factory_unity` 1 |
| Unity | removing an adversary troop | `alignment.troop` 0 |
| Unity | Biomia's converter | 1 for 2 research cubes |
| Unity | liberating a corrupted city (§2: a liberate card or the fallback) | `alignment.free_city` 5 |
| Unity | removing a botnet | `alignment.botnet` 2 |
| Unity | clearing a corruption marker (the Clear action, Recovery, cards) | `clear_unity` 1 |
| Unity | player cubes on a megaproject as it completes, as its corp's card prints (Delta Seawall) | 1 each |
| Unity | cards (Peace Dividend, Planetary Referendum, Martial Cordon), AI boards, Unity cards | card text |
| Discord | energy diverted (a corrupted city) or darkened (an ambassador on an aligned city) | `divert_discord` 1 per unit of a corrupted city's, `ambassador_discord` 0 per unit, each round, of a dark aligned city's |
| Discord | an adversary ambassador on a producing base | `base_discord` 1 |
| Discord | a facility destroyed in war (not by an adversary troop) | `destruction_discord` 0 |
| Discord | adversary cubes on a megaproject as it completes, as its corp's card prints | 1 each |
| Discord | an uncountered threat, the antimatter action, twists, Discord cards, adversary projects | `threat_discord` 1, `antimatter_discord` 1, card text |
| Discord | the adversary at the start of every round | `adversary_discord` 0 |
<!-- /generated -->

`adversary_discord` is 0 because corrupted cities' diverted energy is the adversary's
per-round drip (1 per corrupted city).

The round's Sway is the phase `alignment.sway`, first in the round: the adversary's
`adversary_discord`, then each AI board's `unity` (§7).

**More Unity.** Unity should tip in most games while Discord stays the main pressure:
liberating a corrupted city sways `alignment.free_city`, removing a botnet
`alignment.botnet`, and a player ambassador on a producing military base sways `base_unity`
(**locked**).

**The antimatter action.** In the action phase (`...+alignment.act`), any player may sway
`antimatter_discord` toward Discord to gain 1 antimatter, up to `antimatter_uses` times a
round each. `antimatter_discord` 0 removes the action, as does variant `off`.

**The cards (one-shot and hidden, like fear cards: locked).** Each side has a small
face-down deck in two tiers, each shuffled at setup
([`content/alignment.json`](../content/alignment.json)). A tip draws from tier 2 once that
side's tip count, before the tip, has reached `unity_ease[0]` / `discord_tier2`, else from
tier 1; from the other tier if that one is empty. The card resolves at once and goes face up
on its side's resolved pile; there are no ongoing cards. **Neither deck ever reshuffles:
they are the game's two clocks.** A Discord tip that needs a card and finds the deck empty
loses the game (**locked**, Matt); drawing the last Unity card wins it outright, and that
card doesn't act (**Default**). **A game deals a small Unity deck (candidate):**
`unity_cards` of the printed Unity cards, tier 1 then tier 2, at random and face down, so
the last card can be reached. Choices are asked of seat 0. **Default:** "hidden like fear
cards" is read as a face-down deck resolved the moment it tips; Spirit Island holds earned
fear cards face down until a later phase, which would be a timing change. Grassroots
Funding, Think Tank and New Deal give income "for the rest of the game": a one-shot card
that moves a printed income marker, not a card left in play.

**Steps.** A Discord card prints its strength by the Discord tip count (counted before this
tip, so "3+ earlier Discord tips" first fires on the 4th tip). Unity cards have no steps.

<!-- generated: alignment -->
| Card | Side | Tier | Text | Stronger after tips |
|---|---|---|---|---|
| Grassroots Funding | unity | 1 | Every player gains 2 credits each round for the rest of the game. |  |
| Open Market | unity | 1 | The next 2 cards any player keeps from the draft are free. |  |
| Think Tank | unity | 1 | Every player gains 1 insight each round for the rest of the game. |  |
| Ceasefire | unity | 1 | Counter an active threat. |  |
| Civil Defense | unity | 1 | In each country, remove 1 adversary ambassador. |  |
| Volunteer Corps | unity | 1 | Sway 3 toward Unity. |  |
| Global Coalition | unity | 2 | Counter every active threat. |  |
| New Deal | unity | 2 | Every player gains 4 credits now, and 2 credits each round for the rest of the game. |  |
| Decommission | unity | 2 | Remove a botnet anywhere on the board. |  |
| Incursion | discord | 1 | Deal 1 extra threat. 5+ earlier Discord tips: deal 3 instead, and each player destroys 2 of their nodes. | 5 |
| Blight | discord | 1 | In each country, the clean city with the most adversary ambassadors gains 1 more. 5+ earlier Discord tips: 3 more instead, and each player destroys 2 of their nodes. | 5 |
| Propaganda | discord | 1 | Every player loses 4 credits; move 2 Sway from the Unity bucket to the Undecided. 5+ earlier Discord tips: lose 8 credits instead, and each player destroys 2 of their nodes. | 5 |
| Offensive | discord | 2 | The adversary spreads, then destroy all nodes on the node hex with the most adversary ambassadors and troops. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: the adversary builds, then spreads, and each player destroys 2. | 3, 4 |
| Two Fronts | discord | 2 | Deal 2 extra threats. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: deal 3, and each player destroys 2. | 3, 4 |
| Firestorm | discord | 2 | In each country, the clean city with the most adversary ambassadors gains 2 more. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: 3 more, and each player destroys 2. | 3, 4 |
<!-- /generated -->

A threat countered by a card is discarded and its partial payments are refunded.
**Defaults:**

- Open Market's free cards are keeps of drafted cards (§8), by any player, used before
  credits.
- **Unity pulled back:** "Move N Sway from the Unity bucket to the Undecided" returns up
  to N.
- Extra threats come from the current tier's deck (an empty deck reshuffles, as for the
  round's deal), one per country in turn from country 0 (`shared_row`: into the row), and
  are countered or execute like any other. One dealt after the round's counter phase (a tip
  in production or upkeep, False Flag) waits beside the row and executes next round, after a
  counter phase.
- Blight and Firestorm: ties go to the lower hex number, so with no ambassadors on any clean
  city the lowest-numbered one gains them; a country with no clean city gains nothing. Civil
  Defense in a country with no adversary ambassador removes nothing.
- Offensive acts at once, out of the round's order, with this round's card's build or spread
  twists; its build has no Escalation. Before the build step (a tip in the Sway phase, with
  `adversary_discord` 1+) this round's card is the face-up one.

**The purge (the win), eased by Unity.** The purge needs no corrupted cities, no botnets, no
adversary troops and no adversary ambassadors on the board. Unity eases it at each
`unity_ease` threshold. **There is no printed round gate:** the depth of the steps and
liberating the corrupted cities (§2) set how early it can come.

<!-- generated: purge_ease -->
- first threshold (2 Unity tips): adversary ambassadors stop counting;
- second threshold (2 Unity tips): adversary troops stop counting;
- third threshold (4 Unity tips): **botnets** stop counting (candidate: with botnet removal rare, Unity buys the win);
- corrupted cities always count.
<!-- /generated -->

The thresholds are a list: the first is ambassadors, the second troops, the third botnets;
with only two entries botnets always count. **Why:** with no standard action to remove a
botnet, botnets would block the purge in most games. Troops stop counting early (with
ambassadors) because they respawn from adversary factories every production step and walk
away from where a card can reach them. The level-0 purge (below the first threshold) counts
only once this round's adversary card is of stage `clear_win_stage` or later; at 1 (the
default) that is from round 1's card on. From the first threshold the stage doesn't hold the
purge back.

**No escalation in play.** Discord tips don't raise the difficulty. Discord escalates
through each tip's card and the Discord cards' printed steps, and from `discord_tier2` tips
draws from the stronger tier. The view shows the configured level.

<!-- generated: game_end -->
**How the game ends, for new players.** You win by purging the board or by drawing the last Unity card. Pressure comes every round: the face-up adversary card names a kind of site, and every such site gains an adversary ambassador, so you always know where the next wave lands. Ambassadors pile up, pairs become botnets, 4 on a city corrupt it, and corrupted cities and uncountered threats push the Alignment track toward Discord. Each time the track tips, a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Pushing back works the same way in your favor. Mass your ambassadors on a corrupted city to free it, which sways toward Unity, and each Unity tip brings a good card. The Unity deck is small too: a game deals only 6, and drawing the last wins at once. 2 Unity tips and the purge asks only for no botnets and no corrupted cities; 4 and it asks only for no corrupted cities. You also lose if any player has no nodes left on the board. Threats, war, corruption and late adversary and Discord cards destroy it, a token at a time, and a lost token leaves the game and marks its hex, so you can't act from there any more; its income stays. In stage I your last one is safe. The adversary deck is a round cap too, 11 rounds at most, though games rarely reach it. So there are three different pressures: the Discord deck (politics), your nodes (the board) and the cap.
<!-- /generated -->

**The printed steps:** with no reshuffle the draw order is fixed, tier 1 at tips 0-1 and 5,
tier 2 at tips 2-4, and a tip with 6 already made finds the deck empty (a tip sooner at
difficulty 4). So tier-1 cards step at 5, tier-2 at 4.

**Winning:** the purge, or drawing the last Unity card.

**Losing:**

- a Discord tip finds the Discord deck empty (**locked**);
- a player has no nodes on the board: everyone loses, as when a spirit is destroyed
  (**locked**);
- the adversary deck runs out (the **round cap**, a quiet backstop; §3).

The supply running out is not a loss (it is a cap, §3), nor is every city corrupted: a
corrupted board drives Discord.

Win and loss are checked whenever an action or automatic phase finishes resolving: full
buckets tip first (Discord before Unity), then losses, then the purge. A tip that ends the
game ends it there.

## 6. Resources and the round

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board (§7), the node track, cards and Unity cards, in upkeep; megaproject payouts; selling cards and chips | Card costs; keeps beyond the opening's free ones (`opening_keep_credits` each); insight for keeps (`insight_credits` each); neutral cities' energy; standard projects, Clear and Move; the Builder's Buy In; threat counters |
| Energy (keeps between rounds; rare, candidate: about 1-3 held mid-game, near 0 at the end) | Cities' cubes in production (§6 below); Horizon's completion payout; the node tracks; Power buildings (a flat income of +2 a round at most on any card; Tidal Barrage grows with Power tags); the Builder's Overcharge; the Protector's printed energy; Power programs; `ai.start_energy` at the start | The node powers and every track step (§7); the dear powers (Rush Build, Shelter, Particle Run); cards' `energy` cost (a few, two of them strong: Kinetic Interdiction, Orbital Mass Driver); Blackout's, Sabotage's and Commandeer's counters; the energy gift Actions (Substation Hub, Treaty Grid Exchange: 1 energy, another player gains); Bulwark's hardening |
| Chips | Player-owned factories on production steps (`factory_chips` each); Onyx's converter; the Builder's tracks and its Mint and Scrap; cards (chip income: Bulk Silicon Procurement, Peaker Turbines, Arcology Foundry; Antitrust Action's chips per Industry tag; Joint Fabrication Pact; Surplus Auction) | Some buildings' `chips` cost; research stations (`station_chips`); the Harden action a card grants (`harden_chips`); sold at `chips_credits` |
| Antimatter | Tier-2 cards (Positron Storage Ring); the antimatter action; Horizon's converter; the Researcher's Particle Run | Tier-2 cards |
| Research cubes (keep between rounds) | Your stations, once their country's megaproject is complete; the Researcher's Open Archive capstone; Sample Repository, Preprint Exchange | Converters (§1); the Researcher's Peer Review (§7); Inference Cluster, Breakthrough Lab, Policy Institute |
| Insight | Each round, set to the AI board's insight (§7) plus the node track's uncovers plus buildings; less 1 per adversary ambassador that drained your nodes, and less what Smear Campaign took; Cintamani's converter adds 1; the Researcher's Open Science capstone gives others 2 a round; Inference Cluster | Keeping drafted cards; playing cards; giving (expires at the round's end, but the Researcher's carries over: Patience, §7) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards (Consular Pouch, Embassy Compound), Followers through a card's Action, income on Orbital Embassy; the standard project | Moving onto pieces and evicting (they stay, §2); aligning cities, claiming sites, freeing cities (consumed) |

**Cities' energy.** Each city makes `corp_energy` (a megacorp parameter, more for a richer
corp) energy cubes a round. An aligned city's go free to the player who aligned it (+
`aligned_bonus`). A neutral city's can be bought for `city_energy_credits` credits by a
player with a node on or next to it (players choose who, or nobody); otherwise they are
lost. **An adversary ambassador on a city stays, and the city is dark** (Matt: neutral
cities at r37, aligned ones at r38): every round it stands there, a neutral city sells none of
its cubes and an aligned city gives its owner none, and they are lost. One ambassador makes a
city dark; `corrupt_at` of them corrupt it, so clearing ambassadors from the cities you buy
from, and above all from the cities you aligned, comes well before corruption. A corrupted
city's go to the adversary (Discord).

**Cities' energy, for new players.** A city you aligned gives you its energy every round.
A neutral city next to your node sells you its energy. An adversary ambassador standing on
any city shuts it off: your aligned city gives you nothing, a neutral one sells nothing,
every round until someone removes the ambassador, and more of them corrupt the city. If
enemy ambassadors get that close to your city, work to stop them: clear them early. A Blackout stops a country's cities
in the production after the threat executes.

**Each resource is paid in its home domain (principle accepted):** energy for nodes and
Power, credits for the market, chips for facilities, ambassadors for influence, insight for
the turn, tags for powers. A research station costs chips alone.

**Standard actions and projects** (`board.act`, in the action phase) are deliberately poorly
priced: cards are always the better way, and they are a luck hedge. **The gain-an-ambassador
project stays dear on purpose (Matt: 20 credits; overwhelming the adversary with bought
ambassadors takes a big credit investment, and cards, buildings and powers are the better
route):** so ambassadors come from Diplomacy cards, Followers and buildings, and one placed
evicts up to `remove_count` and stays. The **standard projects** (candidate, Matt) are the
credit sink, as in Terraforming Mars: poor but useful rates, no limit a round. Every map
target is in the player's reach. The view publishes each action with its cost
(`View.standard`), so clients do not repeat the parameters.

<!-- generated: standard_actions -->
| Action | Cost | What |
|---|---|---|
| Build a research station | `station_chips` 2 chips | on an open hex in reach that is not an industrial zone; it is yours (your cube) |
| Remove adversary ambassadors | `remove_ambassadors` 1 of your ambassadors, from your supply | it goes onto one hex in reach, removes up to `remove_count` 3 there and stays |
| Clear a corruption marker | `clear_cost` 2 credits | in reach: sway `clear_unity` 1 toward Unity, gain `clear_credits` 0 credits |
| Move a player ambassador | `move_credits` 1 credit | from your supply or the board onto a piece in reach (not the hex it comes from) |
| Gain a player ambassador | `ambassador_credits` 20 credits | into your supply; a standard project too |
| Align a clean city | 1 ambassador | in reach, with a site claim: your cube on it (an adversary ambassador there stays, so a dark city stays dark) |
| Free a research station | `free_station_credits` 6 credits; `free_station_ambassadors` 2 of your ambassadors on it, used up | a corrupted station in reach with no adversary ambassador on it and no face-up cyber threat holding it: it becomes neutral (a botnet there stays) |
| Harden a facility (a card grants it: Hardened Bunker Complex) | the chips the card prints, per token | a player-owned station or factory in reach that is not corrupted, one a troop can strike |
| Sell chips | 1 chip | gain `chips_credits` 1 |
| Sell a card | gain `sell_credits` 1 | from your hand |
| Project: harden (`harden_buy`) | `harden_credits` 3 credits | any player: a hardening token on a player-owned station or factory in reach |
| Project: drain (`drain`) | `drain_credits` 5 credits | remove up to `drain_amount` 3 compute from one visible adversary project |
<!-- /generated -->

There is no "build a factory" standard action: cities build factories.

### The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | AI boards sway |
| 2–4 | `adversary.build`, `.spread`, `.corrupt` | The adversary (§3): build, ongoing projects, botnet compute onto the projects; spread; corrupt |
| 5 | `board.march` | The adversary troops march (§4): `troop_steps` steps each, move or destroy |
| 6 | `threats.deal` | Threats face up on each country |
| 7 | `adversary.reveal` | Next round's card face up |
| 8–11 | `cards.tier`, `.deal`, `.draft`, `.keep` | Round 1: the opening deal and keep. Later: build the draft hand card by card (tier each), draft, keep (§8) |
| 12 | `cards.play+threats.counter+economy.give+alignment.act+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers (nodes come from them), standard actions, countering threats, giving, antimatter |
| 13 | `board.bases` | Production 1, on a production step: bases and factories make troops (ambassadors divert), factories make chips, cities grow factories |
| 14 | `board.war` | Production 2, on a war step: front battles |
| 15 | `board.produce` | Production 3: cities' energy cubes (neutral ones for sale) and stations (ambassadors divert); ambassadors on nodes drain insight; full megaprojects complete |
| 16 | `economy.upkeep` | Production 4: income; building upkeep |
| 17 | `threats.execute` | Production 5: partial payments refunded; uncountered threats execute |
| 18 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

## 7. AI boards (module `ai`; [`content/ais.json`](../content/ais.json))

Each AI board has a **special rule**, a printed **income**, **printed tags**, two **node tracks** (economy and
card-play), building slots (`ai.slots`) and starting credits (`ai.start_credits`), config lists by board, so a board
has the same in every seating, and its **powers**. The first token starts on the economy
track, so its first space is uncovered from setup; placing a node takes the next token off a track and uncovers what
is printed under it. Uncovered values add to the income every round (insight at the start of the round, the rest in
upkeep). The board module owns nodes and counts the tokens taken (`board.track.<seat>`); the AI board reads it.

**The Builder, for new players.** Your special rule is Breadth: five building slots (the Protector has four, the
Researcher three), so your engine is wider. You start with 40 credits and earn 25 a round, and each credit space of your
economy track adds 2 or 3 more, so placing a node is how your income grows; you're short of energy. Round 1 deals you
10 cards: 4 are free and each further keep costs 5, so choose between more cards now and more money for the first
plays. Each round you may turn credits into a chip with Mint. Late in the game, sacrifice cards you have finished with:
an Industry card pays out chips, a Power card energy, and an Industry card with an energy plays a building from your
hand for no credits.

**The Protector, for new players.** Your special rule is Wall: adversary troops can't enter a hex with your node or
strike into it, so where you stand is a barrier and a facility under you is safe. You spend energy to harden
facilities with Bulwark, and your board prints 1 energy a round. You earn fewer credits (21 a round), so you start with
more: 46. Each credit space of your economy track adds 2 or 3 credits, the first one 1 energy too. Round 1 deals you
10 cards: 4 are free and each further keep costs 5. Late in the game, sacrifice cards you have finished with: a
Security card pays out credits or clears a corruption marker, and a Security card with an energy shelters a whole country
for the round.

**The Researcher, for new players.** Your special rule is Patience: insight you don't spend stays with you into the
next round, so you can save up for a big round. You start with 40 credits and earn 18 a round, 3 more for each credit
space of your economy track; round 1 deals you 10 cards, 4 of them free to keep and 5 credits each after. You play off
the board, on your turn budget and on research cubes. Each round you may turn 1 energy into 3 cards and 3 credits into
a research cube on a megaproject that isn't complete yet. Your second node power costs 3 research cubes and no energy,
so build stations early: once their country's card completes they put cubes on your board. Sacrifice an Industry card
to build a station for free, a Cyber card to strip 3 compute off an adversary project, a Power card and an energy
for an antimatter. Your Science tag counts for cards' requirements, never for a sacrifice. The hard part to explain: Open
Science's insight is handed out in upkeep and lands at the start of the next round.

**Printed and revealed tags (candidate).** Tags printed on a board and tags a track space reveals count for what a
card requires, exactly as tags on your cards do, and for nothing else: no tag discount, no income per tag, no
sacrifice.

**Capstones (candidate).** Each track ends in a capstone, a standing rule once its space is uncovered, as a card's
Effect is while it is in play. Late spaces cost more energy, so a capstone is what a seat pushes on for; at 1-2
placements a round it is reachable about round 8.

<!-- generated: boards -->
| Board | Printed income | Starting credits | Slots | Printed tags | Economy track | Card-play track | Tokens in all |
|---|---|---|---|---|---|---|---|
| Builder | 25 credits, 4 insight | 40 | 5 | Industry | 7 | 7 | 14 |
| Protector | 21 credits, 4 insight, 1 energy | 46 | 4 | Security | 7 | 7 | 14 |
| Researcher | 18 credits, 4 insight | 40 | 3 | Science | 7 | 7 | 14 |
<!-- /generated -->

**The starting-economy scaffold (candidate; Matt).** Until engines carry the economy, the boards start and earn much
more, and the economy track is how a seat's income grows; it comes down as engines take over. **The reasoning:**
round 1 deals 10, so a board at 40 that keeps 2-4 extra cards (10-20 credits) has 20-30 left for its first plays at a
tier-1 median of 6 credits and 4 insight: keeping more and playing less is a real choice, as in Terraforming Mars'
first generation. The Protector earns 4 less than the Builder and starts 6 ahead. A credit space pays 2-3 a round for
a placement of 1 energy plus its step (0-3), energy at about 2 credits: the early spaces pay back in a round or two,
the cost-3 space in about three, so a space placed by round 6 of 8-10 pays for itself and the last ones are a
late-game choice. **The pressure that keeps it honest is the adversary's:** a player ambassador costs 20 credits
(`board.ambassador_credits`) and a corrupted city makes 2 adversary ambassadors a build.

**One payment path (locked).** Every space prints a **cost** in energy, the last ones more. Every placement pays it,
on top of what a node power costs; a card pays only the step, and a card that prints "free" pays nothing (three
cards). Only tracks whose next step you can pay are offered, and a placement you can't pay doesn't happen.

**Hard to explain to a new player:** a placement's price is two numbers, the power's and the space's; Harden has two
prices, Bulwark's energy and a card's chips; "bound" is a new word for a city that is otherwise only aligned.

**Node powers (candidate).** There is no growth phase. Nodes come from the board's two **node powers**, used in the
action phase, each once a round, so the powers place 0-2 nodes a round (cards that place nodes add to that: cards
aren't capped). Every board has the same first one, 1 energy for 1 node; the second costs a steep price in the board's
own currency, plus 1 energy on the Builder and Protector (the powers table below). The player chooses between the two
tracks. Three things keep the loops short: 2 placements a round by power and rising step costs cap energy → node →
energy income; each second power is once a round; and every tag sacrifice permanently loses cards.

<!-- generated: node_track -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 (on the board at setup) | +1 insight | +1 insight | +1 insight |
| 2 | +3 credits | +1 energy, +2 credits | +3 credits |
| 3 | +2 credits | +3 credits | builds a free research station in reach |
| 4 | +3 credits, Power tag (cost 1) | +3 credits, Diplomacy tag (cost 1) | +3 credits, Power tag (cost 1) |
| 5 | +3 credits (cost 2) | +3 credits (cost 2) | +3 credits (cost 2) |
| 6 | +3 credits (cost 3) | +2 credits (cost 3) | +3 credits (cost 3) |
| 7 | **Full Capacity** (capstone): 1 chip each round per 2 Industry tags you have. (cost 5) | **Sanctuary** (capstone): When the adversary spreads, it skips your nodes. (cost 5) | **Open Archive** (capstone): Each round, 1 research cube onto your board. (cost 5) |
<!-- /generated -->

The **keep upgrade**: tier-2 keeps cost `keep_tier2_upgraded` instead of `keep_tier2` (§8).

**The card-play track (draft candidates, reasoning in `experiments/results/screen-068-boards/boards.md`).** The second
track, none on the board at setup. A placement takes the next token from the track the player chooses (asked when both
have tokens).

<!-- generated: node_track2 -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 | +1 insight | +1 insight | +1 insight |
| 2 | **keep upgrade**, Industry tag | **keep upgrade**, Security tag | +2 insight |
| 3 | +2 insight (cost 1) | +2 insight (cost 1) | **keep upgrade**, Science tag (cost 1) |
| 4 | +1 credit, Science tag (cost 2) | +1 credit, Cyber tag (cost 2) | +1 insight, Cyber tag (cost 2) |
| 5 | +2 insight (cost 2) | +2 insight (cost 2) | +2 insight (cost 2) |
| 6 | Cyber tag (cost 3) | Diplomacy tag (cost 3) | +1 credit, Industry tag (cost 3) |
| 7 | +1 insight, **Master Plan** (capstone): Your buildings cost 2 credits less. (cost 5) | +1 insight, **Aegis Doctrine** (capstone): Each round, stop the first destruction in a country you're in. (cost 5) | +1 insight, **Open Science** (capstone): Each round, 2 insight from the supply, split among other players. (cost 5) |
<!-- /generated -->

**Loops and caps.** Credits → chips → buildings → income → credits runs once a round and in 5 slots at most. Rush Build
takes 1 Industry tag and 1 energy out and puts a building with 1-3 tags in, once a round: the Industry count can hold
level, but each use spends an energy and a card for good, and the building still pays its insight, so energy and the
hand cap it. **The energy gift** (1 energy → another player's 5 chips at Substation Hub or 8 credits at Treaty Grid
Exchange, once a round a building) is a conversion with a boost: an energy is worth about 4.5, the gift 7.5-8. Energy
bought from a neutral city costs `city_energy_credits` (5), so credits → energy → Treaty Grid Exchange → credits gains 3 a
round, for the team, not the buyer; Substation Hub's 5 chips sell back for 5 credits (`chips_credits` 1), so its loop
gains only where the chips pay a chip cost. It is capped by once a round, the two buildings, and the cities' energy
(a city's `corp_energy` cubes a round, none while an adversary ambassador stands on it). Nothing turns the gift back
into energy. **Energy counters**
spend energy for good. **The Researcher's:** Patience with Cintamani's converter (a cube for an insight) is
the intended engine, capped by how many stations make cubes, by the city node in Cintamani's country that the converter
needs, and by once a round per player. Bought cubes go only onto incomplete cards and are spent at completion, so credits never reach a converter.
Open Science's insight comes from the supply, not the Researcher's bank, and goes only to other players, so Patience
can't bank it. Fieldwork's three cards are paid for again to keep in play (credits and insight); sold, they are 3
credits for 1 energy once a round. Followers → ambassadors → alignment → city energy → node is capped by building
count and the Followers each ambassador costs. Evicting ambassadors stay on the map, capped by the supply, and the next
arrival there cancels one. Wall has no loop, but it takes troops away as a harm where the Protector stands;
`docs/harms.md`'s checks count sources, so they hold.

**Sacrifice is late-game liquidation.** Once the cards needing a tag are played, its tags turn into engine rewards,
each payoff priced to be worth the card: 2 chips, 2 energy, 3 credits, a marker with its Sway; a tag and an energy buy a
building, a sheltered country or an antimatter (r35).

**Powers (locked: tag costs are sacrifices).** A power's cost names tags (`any` for any tag) and optional other
resources; in the table a tag in the cost is a sacrifice, and thresholds are for powerful cards only. It is paid by
discarding played cards that carry those tags: a program from your play area, or a building **with its whole stack**
beneath it; events can't be sacrificed (they are gone once played). A card pays with all its tags, each tag first
toward its own requirement, then toward `any`; extra tags are wasted. Each power is usable once a round. **Default:**
what can be sacrificed is fixed when the power is used; the player chooses the cards one at a time among those that
still help.

<!-- generated: powers -->
| Board | Power | Cost | Effect |
|---|---|---|---|
| Builder | Mint | 3 credits | gain 1 chip |
| Builder | Deploy | 1 energy | place 1 node |
| Builder | Buy In | 1 energy + 6 credits | place 1 node |
| Builder | Scrap | 1 Industry | gain 2 chips |
| Builder | Overcharge | 1 Power | gain 2 energy |
| Builder | Rush Build | 1 Industry + 1 energy | play a building from your hand, paying no credits |
| Protector | Bulwark | 1 energy | harden 1 facility in reach |
| Protector | Deploy | 1 energy | place 1 node |
| Protector | Accord | 1 Diplomacy + 1 energy | place 1 node |
| Protector | Stand Down | 1 Security | gain 3 credits |
| Protector | Recovery | 1 Security | clear a corruption marker in reach |
| Protector | Shelter | 1 Security + 1 energy | in one country this round, no nodes or facility can be destroyed |
| Researcher | Fieldwork | 1 energy | draw 3 cards |
| Researcher | Deploy | 1 energy | place 1 node |
| Researcher | Peer Review | 3 research cubes | place 1 node |
| Researcher | Grant | 3 credits | add 1 research cube in your color to an incomplete megaproject |
| Researcher | Field Station | 1 Industry | build a research station in reach, paying no chips |
| Researcher | Audit | 1 Cyber | remove 3 compute from one adversary project |
| Researcher | Particle Run | 1 Power + 1 energy | gain 1 antimatter |
<!-- /generated -->

`ai.boards` (`[0, 1, 2]`: Builder, Protector, Researcher) gives each seat its board; the list repeats for more seats,
so 4 players repeat the Builder. The engine reads a board's `cards` and `start` (one copy each, so a repeated AI's
second seat would have neither); no content prints them.

<!-- generated: board_details -->
| Board | Special rule |
|---|---|
| Builder | *Breadth*: You have 5 building slots; the Protector has 4 and the Researcher 3. |
| Protector | *Wall*: Adversary troops can't enter or strike a hex with your node. |
| Researcher | *Patience*: Your unspent insight carries over to the next round. |
<!-- /generated -->

Hardening (the standard actions, cards) goes only on facilities a token can protect, player-owned stations and
factories that are not corrupted, and "harden N" is N different facilities, fewer if fewer qualify. A Science card's
**shield** is the same token on a hex of your nodes: it absorbs a destruction there and goes with the token if it is
lost another way. On your own facility a shield is the facility's hardening token: a table has one token, so it stays
when the node goes, and protects the facility.

## 8. Threats and cards

### Threats (module `threats`; [`content/threats.json`](../content/threats.json))

- Variant **`countries`** (default): each threat sits on one country and acts there; who may pay toward it is set by
  its **sources** (below), and "each player" in its text means each player with a node there.
- Variant **`shared_row`**: threats sit in one row, anyone may pay, and each acts on the whole board: an effect per
  country (Blackout, Sabotage, Overrun, Mass Production) hits every country, "each player" is every player, and a
  placement (Infiltration's, Agitators', Sleeper Cells' and Insurrection's ambassadors, Orbital Strike's, Siege's and
  Total War's troops and ambassadors, Station Seizure's station) is that many on the whole board, players choosing
  where. **Default:** the row is still dealt `per_country` threats per country, so more countries deal more threats.
- Variant **`off`**: no threats (§9).

**Threat sources (candidate; design doc, Matt).** Threats come from the adversary's pieces on the map, and reaching
those pieces decides who counters them. Every threat prints a **source type**; its sources are all the adversary
pieces of that type in its country:

| Source type | Source pieces |
|---|---|
| influence | corrupted cities |
| cyber | corrupted research stations and botnets |
| economic | adversary factories |
| military | troops |

Military bases and adversary ambassadors are never sources.

- **Who may counter:** a player with any of the threat's sources in reach, or with their ambassador on one. Payments
  are pooled, and a player who lost their access can still withdraw. A player's card that jams or counters a threat
  (§8) reaches only threats they may counter; a reward from the Unity deck reaches any threat.
- **Countering doesn't consume the ambassador.** It holds the source and keeps access to later threats there until an
  adversary ambassador cancels it. The rules that take ambassadors anyway still do: a facility's defense (§2), and a
  fallback liberation (§2).
- **No source when dealt: a target (Matt).** A threat with no source in its country names one: the lowest-numbered
  clean city, station or factory of its type, neutral before player-owned; for a factory or station with none to take,
  the lowest site one would be built on; for troops the country's lowest base a Wall leaves open. While the threat is
  face up the target is its source: anyone with it in reach, or with an ambassador on it, may counter.
  **Countered, nothing is created.** Uncountered, as part of its execution and before its own effect, the target is
  corrupted, built then corrupted, or the troop lands (a Wall on the base stops it). At most one piece a country a
  round. Nothing is set aside for lack of a source. A country with nothing to name (no base at 1 country) leaves the
  threat with no source: nobody can pay it, and it makes nothing.
- **A source that leaves (Default):** the rule reads the map now, so a threat whose sources are gone can't be paid
  until one is back, and it still executes. Marches come before the deal, so this takes a troop spent or a piece gone
  inside the round.
- **Shield:** while a threat is face up, the adversary's pieces of its source type in its country can't be destroyed
  or liberated until it is countered: removal and liberate cards skip them, and player ambassadors at a corrupted
  city's fallback wait, and liberate it the moment the threat is countered. An adversary ambassador is never shielded.
- **Standing counters:** while a player has a tier-2 capstone with an automatic counter, each threat of its type in
  their home country needs that much less, whenever it was dealt.

**For new players.** Every threat grows out of something on the map. Find what its badge names in its country: a
corrupted city (influence), a corrupted station or botnet (cyber), an adversary factory (economic), a troop
(military). You can pay only with one of those next to your nodes, or with your own ambassador on one. If the country
has none, the threat names a target on the map, and that counts as its source: counter it and nothing happens, leave
it and the adversary gets a new piece there. Until a threat is countered, you can't remove its sources, so counter
first and clear after.

**Dealing.** In `threats.deal` each country gets `per_country` threat (plus difficulty 1's extra, §3), face up: tier 1
until this round's adversary card is stage II or later, then tier 2 (difficulty 4: a round sooner). Each tier's deck
holds one copy of each of its threats, shuffled at setup; its discards reshuffle when it runs out, for any deal. Each
deck holds enough threats that a deal doesn't run short at 4 countries and difficulty 4; a deal the deck and its
discards can't supply deals what it can and counts the rest as `threats.short`.

**Energy counters (candidate).** A counter is one cost, each resource it prints paid in full. Most print credits alone;
the grid's own harms, Blackout, Sabotage and Commandeer, take 1 credit and 1 energy, so energy has a use every round
a threat lands. Any resource a seat gets without a specific card may be a counter's; the full threat redesign waits
for the adversary pass.

**Countering (Default).** A threat is countered only when fully paid. Payments are pooled across players (`Pay` 1, or
as much as is still useful, at a time; `Withdraw` takes back everything you put on it in one resource, even after you
lose your nodes there). The moment it is fully paid it is discarded and the payments are spent (an overpayment,
possible after a jam, goes back, later seats first). In `threats.execute`, anything still on an uncountered threat is
refunded, and the threats execute in order (the row, then country 0, 1, ...) and are discarded. Only threats that were
on the board during this round's counter phase execute: one dealt later in the round (a Discord tip in production or
upkeep, False Flag) waits beside its country (or the row) and executes next round, after a counter phase in which it
can be paid (Matt).

Choice threats are preferred; a destroyed node leaves the game, so a tier-1 threat's loss can be permanent too.
**Counters cost credits (candidate),** 4-8 by tier and harm (the table prints them):

<!-- generated: threats -->
| Threat | Tier | Source | If not countered | Counter |
|---|---|---|---|---|
| Requisition | 1 | economic | Each player here discards 2 cards or pays 6 credits. | 4 credits |
| Commandeer | 1 | economic | Each player here discards 2 cards or pays 4 energy. | 1 credit, 1 energy |
| Supply Raid | 1 | economic | Each player here loses 6 credits. | 5 credits |
| Blackout | 1 | cyber | Cities here produce no energy next round; the adversary gains 3 compute. | 1 credit, 1 energy |
| Sabotage | 1 | cyber | Players here make no energy in upkeep next round and each loses 3 tokens from their cards; the adversary gains 3 compute. | 1 credit, 1 energy |
| Infiltration | 1 | cyber | 3 adversary ambassadors land on player-owned pieces here. The adversary gains 3 compute. | 5 credits |
| Agitators | 1 | influence | 3 adversary ambassadors land on clean cities here. Each player here destroys 1 of their nodes here. | 5 credits |
| Orbital Strike | 1 | military | 1 troop lands on a factory here, or a city if no factory. The adversary gains 3 compute. Each player here destroys 1 of their nodes here. | 6 credits |
| Defection | 1 | influence | Each player here loses an ambassador. Sway 2 toward Discord. Each player here destroys 1 of their nodes here. | 4 credits |
| Eviction | 1 | military | Each player here pays 6 credits or loses a node here (never their last). | 5 credits |
| Embargo | 1 | economic | No site here can be claimed next round; the adversary gains 3 compute. | 4 credits |
| Smear Campaign | 1 | influence | Each player here gets 2 less insight next round. Move 2 Sway from the Unity bucket to the Undecided. | 4 credits |
| Station Seizure | 2 | cyber | An uncorrupted research station here becomes corrupted (a player's holds unless enough adversary ambassadors and no player ambassador stand on it: an adversary ambassador lands there instead). The adversary gains 4 compute. Each player here destroys 1 of their nodes here. | 7 credits |
| Overrun | 2 | influence | The uncorrupted city here with the most adversary ambassadors becomes corrupted. Each player here destroys 1 of their nodes here. | 8 credits |
| Mass Production | 2 | economic | The adversary builds now, here; the adversary gains 4 compute. | 7 credits |
| Purge Protocols | 2 | cyber | Each player here discards their most expensive card in hand and loses 3 tokens from their cards. | 6 credits |
| Total War | 2 | military | 2 troops land on factories here, or cities if no factories; each destroys all nodes on its hex. 2 adversary ambassadors land on military bases here. The adversary gains 4 compute. | 8 credits |
| Conscription | 2 | economic | Each player here discards 3 cards or pays 6 energy; the adversary gains 3 compute. | 7 credits |
| Sleeper Cells | 2 | cyber | 4 adversary ambassadors land on player-owned pieces here. The adversary gains 4 compute. Each player here destroys 1 of their nodes here. | 7 credits |
| Insurrection | 2 | influence | 4 adversary ambassadors land on clean cities here. Sway 2 toward Discord. Each player here destroys 1 of their nodes here. | 7 credits |
| Siege | 2 | military | 2 troops land on factories here, or cities if no factories. Each player here destroys 2 of their nodes here and loses an ambassador. The adversary gains 3 compute. | 8 credits |
| Manhunt | 2 | military | Each player here destroys 2 of their nodes here. | 8 credits |
| Asset Freeze | 2 | economic | Each player here loses 8 credits. The adversary gains 3 compute. | 7 credits |
| Deepfake Wave | 2 | influence | Sway 3 toward Discord. Each player here gets 2 less insight next round. | 7 credits |
<!-- /generated -->

**Compute.** About half the threats also give the adversary compute, onto its projects (§3), when they execute
uncountered: candidate amounts, printed on the threat.

An uncountered threat also sways `threat_discord` Discord. "Pay or discard" is the player's choice; with neither
possible, the player pays what they have and discards their hand (**Default**). Purge Protocols compares printed
credit costs; on a tie the player chooses. A lost ambassador comes from the player's supply, else from the board
(their pick of hex).

### Player cards (module `cards`, variant `draft`; [`content/cards.json`](../content/cards.json))

The cards config lists which cards are in the game (baseline: all of them).

- **Decks:** a tier-1 and a tier-2 deck of the listed cards (one copy of each), shuffled at setup; discards reshuffle
  into a deck when it runs out. **Default:** when both decks and their discards are empty (every card is in a hand or
  in play), the draft hands are what has been dealt.
- **The opening (round 1).** Each player is dealt `opening_deal` tier-1 cards and keeps `opening_free` free; each
  further card kept costs `opening_keep_credits` credits. No draft in round 1 (**Default**, as Terraforming Mars'
  first generation).
- **The draft (rounds 2+).** `cards.tier`: each player builds a hand of `deal` cards **card by card**, choosing for
  each card which deck it comes from; it is dealt face up into the hand, so the next choice sees it. `cards.draft`:
  take one card, pass the rest to the next seat, until the hands are gone: to seat *s*+1 in odd rounds, *s*−1 in even
  rounds (as in Terraforming Mars); nobody may pass in `cards.tier` or `cards.draft`. `cards.keep`: keep any drafted cards for insight, `keep_tier1` for a tier-1 card
  and `keep_tier2` for a tier-2 card (`keep_tier2_upgraded` with the board's keep upgrade); any insight short is paid
  at `insight_credits` credits an insight (keeps only). An Open Market free keep is used first. The rest are
  discarded when the action phase begins.
- **Playing** a card costs its credits (buildings minus your discounts: Industry tags, Master Plan), and any
  `insight`, `energy`, `chips`, `antimatter` and research cubes it prints, and lowers any income it prints under
  `lowers` ("lower your credit income 1": your production of that resource falls for the rest of the game; it can't
  be played if that income is lower). **Insight to play:** 0 for small programs and events, 1 by default (removal
  events included), 2 for big effects and most tier 2, 34 of 49 (the draft set: 27 / 96 / 36 cards at 0 / 1 / 2); it
  comes from the same insight as keeps, and the credit conversion is for keeps only. Its `needs` (tags on your cards
  in play, your board's printed tags and those its tracks revealed, §7) are checked only when it is played. Each
  card's price is in [cards.md](cards.md); the reasoning is in [card-pricing.md](card-pricing.md).
- **Science owns a niche (candidate).** Among cards, only Science cards clear corruption markers, put a hardening
  token on a hex with your node (a shield, §7) or make a threat wait a round (a delay: it skips this round's
  execution, stays up, and can be countered next round). The content check refuses any of the three on a card without
  the tag or on a power, save the Protector's Recovery, which clears a marker. Six tier-1 and two tier-2 cards do it.
  No card `needs` Science for it: a team without the tag still has the Clear standard action, its counters and the
  board.
- **Typed counters (candidate).** Twelve tier-1 cards fight one source type, and count for more than they cost toward
  threats of it that the player may counter: Security for military, Diplomacy for influence, Cyber for cyber, Industry
  for economic (Science's and Power's tools stay general). Each type has two programs and an Action building.
- **Tier-2 capstones (candidate).** Each tag ends in at least one standing tier-2 building: income per tag, a drain on
  every adversary project each upkeep, or an automatic counter. No capstone prints a shield or a delay. Reasoning and
  prices: [card-pricing.md](card-pricing.md).
- **The liberate capstones (candidate; Matt).** Liberating a corrupted city is a core mechanic, and its main route is
  a family of tier-2 cards, one an engine, each paid mostly in its engine's resource and each liberating one corrupted
  city in your reach (§2), some with an extra, a twist from its own engine (prices in [cards.md](cards.md)):

  | Card | Engine | Kind | Extra |
  |---|---|---|---|
  | The Nuclear Option | antimatter | event | sway 2 toward Discord |
  | Power to the People | power + chips | building | income: 1 credit per Power tag on your cards |
  | Hostile Buyout | Diplomacy, credits | event | your cube goes on the city, and you gain 1 ambassador |
  | Forensic Audit | research cubes | event | none |
  | Root Access | Cyber (needs Cyber tags) | event | remove a botnet on or next to the city |
  | Peacekeeping Mission | Security (needs Security tags) | event | remove up to 3 adversary troops in the city's country |
  | Popular Uprising | Followers | building (arrives with 6 Followers) | Action: spend 6 Followers here: liberate, then discard it (and the stack under it) |

  Each is priced at its own value at the rates (`configs/agents/values.json`: a liberation about 45 credits, near what
  a corrupted city costs in play; the extra at its own row), about 25-47 credits, well under the fallback's ~100 and
  out of reach before mid-game. Great Liberation (Diplomacy, three cleanses in one country, cities included)
  liberates too. **Part of the draft is guessing who will have which engine**, and passing each liberate card to the
  seat that can use it. Loops: each card is one-shot and tier 2, and every route needs nodes next to the city, so
  liberation → clean city → energy → more liberation is capped; Popular Uprising liberates once too and leaves play when it
  does.
- **Tag discounts (candidate; the Terraforming Mars pattern).** Every `science_every` Science tags you have take 1
  insight off your tier-2 keeps and your card plays; every `industry_every` Industry tags take 1 credit off your
  buildings, as steel does. Neither goes below `discount_floor`, the cap on the loop "tags, cheaper costs, more tags";
  a cost printed under the floor stays. The floor holds for every building discount.
- **Tags:** Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Your tags are those on every card you
  have in play: active buildings, the cards splayed under them, and programs. Events' tags don't stay.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice them). **Events** resolve
  and are discarded. A program or event (or ability) is only offered when each of its targeted effects has a target (a
  Harden, a move or a gain per tag counts only on a card that does nothing else; an event paying per tag needs one of
  those tags on your cards). A removal's target is a hex with a piece the card names: "remove 1 adversary ambassador"
  can't name a hex holding only a troop. A building's when-built effect is not checked: with no target it does
  nothing.
- **Buildings** go into a slot: your AI board's `ai.slots` (a building printing `slots` adds them and sits in one
  itself; no card prints it). With a slot free you choose a new slot or **build over** one of your buildings; with
  none free you must build over one. The building underneath is **splayed down**: its passives and ability stop, its
  tags still count, and it is sacrificed with the stack. A building with `replaces` (most tier-2 buildings, over their
  own tag) must be built over one whose top card carries that tag.
- **Tokens** live on buildings that print a type (`tokens`): Datasets (Science), Followers (Diplomacy), Capital
  (Industry), Agents (Security). **A token card is progressive and counts only its own tokens (Matt's ruling 12).** No
  card gains tokens by itself. A token
  card's Action is "add N here, OR spend or count what is here"; a few tier-2 ones arrive loaded ("Add 2 Agents here"
  when built); a covered one holds its tokens and its Action stops; Actions may cost tokens from their own card;
  feeder cards (Dataset Scraper, Capital Injection, Vendor Financing, Manifesto Leak, Recruit Operatives) add tokens to
  one of your cards of a type; Sabotage and Purge Protocols make each player lose tokens. Sacrificing or discarding a
  card loses its tokens.
- **A one-shot Action (`once`):** an Action, or a half of one, may take its card out of play once it resolves, with
  the stack under it (Popular Uprising: spend its 6 Followers, liberate, then discard it).
- **Actions** (abilities: `ability`, buildings only) are used with `Use` once a round, pay their cost, and refresh at
  the start of a round. **An OR Action (`or`)** has two halves: one use a round, the player picks a half it can pay
  and that has a target (a half that counts tokens here needs one there). **Ongoing abilities live only on
  buildings:** income, Effect or Action on a program or event is refused when content loads.

Effects needing choices ask the acting player: hexes, countries, players, threats, amounts ("up to N" offers 0..N), or
"stop".

**Token cards, for new players.** A building that holds Agents, Datasets, Followers or Capital grows only when you
choose: its Action is "add 1 Agent here, OR get 1 credit per Agent here" (or pay some of them for an effect). Each
round you either grow the pile or cash in, and a few cards add tokens to one of your piles. A card only ever counts
the tokens on itself, so there is nothing to total across your cards.

(Variant `off`: no draft, §9.)

### The card grammar

(**locked**; `crates/aic-core/src/face.rs` generates every face from the card's ops, so a card's words can't differ
from how it plays. No card text is hand-written.)

**Where it sits is what it is.** The top strip holds the credit octagon, the insight hexagon, requirement plates
(yellow: at least; red: at most; stacked cards: built over) and tags, and nothing else. Above the art, on buildings
only, is what lasts while the building is active: income, *Effect:*, *Action:*. Cover the building and it stops.
Below the art is what happens once, when you play the card: extra costs first, then gains, placements and moves.

**Boxes.** No box: gain it once, now. A brown box is income: above the art it pays each round ("Each round, ...");
below the art it moves your income track once ("Raise your X income by N", "Lower ..." for a minus). A minus before a
plain icon: pay or lose it now. A dark frame acts on the adversary's pieces, projects or threats. A dashed outline is
another player. A thin solid outline is your cards: a token in it goes to, or comes from, one of your cards (nothing
counts tokens across cards). No outline is you, or this card.

**Connectors.** `→` is an Action: once a round, pay the left to get the right; a bare `→` is free ("Action: get
..."), and needs no "once a round": every Action is. `:` is an Effect: when the left happens, get the right. `/` is
per. OR is written out; a line opening with OR is the other half of an Action, one use a round between them. No card
has an exception: every symbol means what it always means ([card-guidelines.md](card-guidelines.md)).

**Shapes.** Square tiles are resources, discs are tokens on cards (alone, on this card), bare silhouettes are pieces
on the board and moving them, ringed circles are tags. A number goes before its icon, no number means 1, and a
negative always has its minus. In the dark frame a clock is a delay (threats wait a round), a minus on the jam mark a
jam (off one threat's counter). `≤`N is up to N, your choice: a give. The shield mark stops one destruction (alone, a
hardening token on a facility; with a node, a shield; with a country, Aegis Dome Array's); the shelter mark is a whole
country for the round. A node with a struck-through coin is placed free. The rules text says what the icons say, in
the same order, and opens with the box's word.

Card details beyond the card text:

The card set: 159 cards, all in the draft decks: 110 tier 1 and 49 tier 2; 42% programs, 41% buildings and 17% events
(the engines are on buildings); each tag primary on 24 to 30. Every card is listed with its cost, insight, tags, kind
and text in [cards.md](cards.md) (generated) and priced in [card-pricing.md](card-pricing.md). Card text says what a
card does; how each primitive resolves:

- **Reach:** a card's map target is in your reach (within 1 of your node) where its text says so; a removal in one
  country (troops or ambassadors by number) is not. A removal (`damage`) takes a hex in reach holding adversary
  pieces; a country removal any country holding them. A card names the pieces it removes: where it says "2 adversary
  ambassadors or 1 troop" you choose one kind.
- **Hardening:** a different facility in reach for each token, each one a token can protect (§7).
- **Factories:** "build a factory" is always on a free industrial zone in reach; the factory is yours (your cube). A
  building that builds one can be played with no such zone and then builds nothing.
- **Moving ambassadors** (`move_ambassadors`, `move_one`, `consolidate`, `spread_ambassadors`): onto pieces (city,
  base, facility or node), each arriving as in §2; stop at any time. `move_one`'s `range` 2 on Embassy Compound and
  Peacekeeper Rotation is an exception to reach 1 kept on purpose: moving is those cards' point.
- **Peek:** the top cards of the adversary deck, behind the face-up card, are shown; one fewer after each reveal.
- **Jam:** the chosen threat needs less of each resource (not below 0) while in play; if that leaves it fully paid, it
  is countered at once. **Counter a threat:** partial payments on it are refunded.
- **Cleanse:** take the adversary cube off a corrupted research station or factory in reach, one hex at a time (some
  cards: all in the first hex's country). A station becomes neutral. `only` limits it to stations (Reclamation Crew,
  priced as a station flip) or to factories (Back-Channel Talks, cheap). With `cities` (Great Liberation) a corrupted
  city counts too, and it is liberated (§2).
- **Liberate:** liberate one corrupted city in your reach (§2): it is clean; with `align` your cube goes on it. Each
  engine's twist: `ambassadors` (gain that many), `troops` (up to that many adversary troops in the city's country come
  off), `botnet` (a botnet on or next to the city comes off). Only the liberate capstones print it (§8).
- **Clear a city (`clear_city`):** remove up to N adversary ambassadors (Liberation Accords: 6) around one corrupted
  city in reach, on it or next to it, your choice which. Those next to the city don't raise the fallback, so clearing them only
  thins the city's build before it spreads; each on the city costs the fallback one more ambassador (§2).
- **Card text:** generated from the card's ops (*The card grammar*, above), extra costs and lowered income included; a
  long card sets smaller. **Harden N facilities** is N different ones (Hardening, above); **shelter one country**:
  nothing in it, facility or node, can be destroyed this round. The house words are in *Terms*, at the top.
- **Projects:** a drain removes compute from one visible adversary project; **drain all** from every visible one; a
  discard takes one with its compute (if the row is then empty, the next row is dealt).
- **Botnets:** removing a botnet (tier 2, in reach) is, with Decommission (a Unity card, §5), the only way to remove
  one (§3); cutting one off (a hack) makes it produce no compute in the next build.
- **Draw:** every draw takes the top cards of the tier-1 deck into your hand, no keep paid (the deck reshuffles its
  discards when empty). Science's card draw.
- **Count as power (`gain_per_tokens`; here only):** gain 1 of a resource per N tokens here, on the Action's own
  card; the tokens stay.
- **Cash out (`cash_out`; one card):** remove every token of a type from one card (an Action's own, "here"; a
  program's, one of your cards you pick); gain the card's rate of a resource for each.
- **Per tag (`gain_per_tag`):** once, 1 of a resource per N tags of a kind on your cards in play; a program counts
  itself (it enters play before it resolves), an event doesn't.
- **Troops in one country (`remove_troops`):** remove up to N troops in one country, any country, your pick of hexes
  when there are more than N. **Ambassadors in one country** (`remove_ambassadors_country` with a number): up to N
  there, the same way. Neither reads "per hex": Matt's ruling 6.
- **Help cards** (keyword `help`, ~10%): the player chooses another player; both gain, the helper paying.
- **Cordon (`cordon`):** in one country, no arrivals land in the next round's build (an out-of-turn build before it
  brings none either); the rest of a build goes on. Hardpoint Lockdown prints it: a Security card that does an
  ambassador's job.
- **Play a building, paying no credits (`free_building`):** the Builder's Rush Build (1 Industry and 1 energy). Its tags are sacrificed first,
  so a building that needed those tags or the sacrificed stack can't then be played; its other costs and requirements
  stand, paid from what is left after the power's energy. The power isn't offered when no building in hand would still
  be playable after some sacrifice and that energy (r39: it took both and played nothing).
- **Aegis Dome Array's shelter:** automatic, the first destruction of a facility or node each round in a country where
  its owner has a node.

## 9. Off-variants

For measuring what a module adds, these variants remove its effect and keep the round: `threats.variant=off` (no
threats), `alignment.variant=off` (no Sway moves, nothing tips, no antimatter action; the purge, the node loss and the
adversary deck-out stay; with no tips the Unity-deck win and the Discord-deck loss can't happen), `cards.variant=off`
(no draft or drawing, so no player cards at all). Two don't isolate their module: with
Alignment off nothing tips, so botnets always count against the purge; with cards off only Decommission removes a
botnet. `board`, `adversary`, `alignment` and `ai` are not removable (each is read by another); `threats=off`,
`cards=off` and `economy=off` remove the module. Parts switch off by parameters (`factory_units` 0, `factory_chips` 0,
`escalation_ambassadors` 0, `board.troop_steps` 0 (no march), `adversary.botnet_compute` 0, `economy.give_insight` 0).

## Tunables

Every parameter of the baseline config, with its value and range, generated from the code and
`configs/baseline.json` (`aic-sim docs`).

<!-- generated: tunables -->
| Parameter | Baseline | Range | What it does |
|---|---|---|---|
| `board.start_corrupted` | 1 | 0..3 | cities per country that start corrupted |
| `board.start_ambassadors` | 1 | 0..10 | player ambassadors each player starts with |
| `board.unit_cap` | 2 | 0..20 | corp units a base holds at most |
| `board.factory_units` | 1 | 0..5 | corp units each factory makes on a production step at each of its corp's bases |
| `board.clock` | 1, 2, 1, 2, 1, 3 | 0..3 | the printed round track, per round: 1 production, 2 war, 3 both (the last entry repeats) |
| `board.corp_energy` | 1 | 0..10 | energy cubes each city of country i makes a round (a megacorp parameter; the list repeats) |
| `board.city_energy_credits` | 5 | 0..20 | credits for a neutral city's energy cube, bought by a player with a node next to it |
| `board.aligned_bonus` | 0 | 0..10 | extra energy an aligned city gives its owner |
| `board.station_cubes` | 1 | 0..5 | research cubes a station makes a round |
| `board.divert_discord` | 1 | 0..5 | Discord per unit of a corrupted city's energy diverted to the adversary |
| `board.ambassador_discord` | 0 | 0..5 | Discord per unit of energy, each round, of an aligned city an adversary ambassador darkens |
| `board.base_discord` | 1 | 0..10 | Discord an adversary ambassador on a producing base sways |
| `board.base_unity` | 2 | 0..10 | Unity a player ambassador on a producing base sways |
| `board.factory_unity` | 1 | 0..10 | Unity a player ambassador on a producing factory sways |
| `board.liberate_at` | 5 | 1..10 | player ambassadors on a corrupted city, from seats with a node on or next to it, that liberate it (the fallback) |
| `board.last_safe_stage` | 2 | 0..3 | while this round's adversary card is below this stage, nothing destroys a player's last node (0: never safe) |
| `board.free_leaves` | 1 | 0..5 | adversary ambassadors left on a city as it frees (the piece ladder) |
| `board.destruction_discord` | 0 | 0..5 | Discord each facility destroyed in war sways |
| `board.troop_spent` | 1 | 0..1 | 1: an adversary troop that strikes a target is used up |
| `board.troop_steps` | 2 | 0..5 | steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest |
| `board.troop_node` | 4 | 0..4 | adversary troops strike nodes from this stage of adversary card on (0: always, 4: never) |
| `board.node_marker` | 1 | 0..1 | 1: a destroyed node leaves a corruption marker on its hex |
| `board.corruption_reach` | 1 | 0..1 | a city that corrupts destroys nodes within this many hexes of it |
| `board.war_reach` | 0 | 0..1 | war that destroys a factory destroys nodes within this many hexes of it |
| `board.station_chips` | 2 | 0..20 | standard action: chips to build a research station |
| `board.factory_chips` | 1 | 0..5 | chips a player-owned factory makes its owner on a production step |
| `board.chips_credits` | 1 | 0..50 | standard action: credits for selling 1 chip |
| `board.remove_ambassadors` | 1 | 1..5 | standard action: your ambassadors from the supply that go onto a hex to evict its adversary ambassadors, and stay there |
| `board.remove_count` | 3 | 1..3 | standard action: adversary ambassadors one removal takes off its hex, at most |
| `board.clear_cost` | 2 | 0..20 | standard action: credits to clear a corruption marker within reach |
| `board.clear_unity` | 1 | 0..5 | Sway toward Unity for clearing a corruption marker |
| `board.clear_credits` | 0 | 0..5 | credits back for clearing a corruption marker |
| `board.move_credits` | 1 | 0..20 | standard action: credits to move a player ambassador |
| `board.ambassador_credits` | 20 | 1..50 | standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit) |
| `board.sell_credits` | 1 | 0..20 | standard action: credits for selling a card from hand |
| `board.harden_credits` | 3 | 1..50 | standard project: credits for a hardening token |
| `board.drain_credits` | 5 | 1..50 | standard project: credits to drain compute from an adversary project |
| `board.drain_amount` | 3 | 1..20 | standard project: compute drained |
| `board.free_station_credits` | 6 | 0..50 | standard action: credits to free a corrupted research station |
| `board.free_station_ambassadors` | 2 | 1..5 | standard action: your ambassadors on a corrupted research station that freeing it uses up |
| `adversary.difficulty` | 0 | 0..4 | difficulty level 0-4 (each level includes the ones below) |
| `adversary.stage_cards` | 4, 5, 2 | 0..20 | adversary cards dealt from stage I, II and III (before difficulty) |
| `adversary.supply_per_country` | 24 | 0..40 | adversary ambassadors in the supply per country |
| `adversary.base_sites_per_country` | 1 | 0..1 | 1: arrivals and escalation reach only each country's first military base, its lowest-numbered (0: every base) |
| `adversary.city_build` | 2 | 0..10 | build: ambassadors each corrupted city gains |
| `adversary.factory_build` | 0 | 0..10 | build: ambassadors each factory next to a corrupted city gains |
| `adversary.spread_range` | 2 | 1..6 | spread: how far (hexes) a spreading ambassador may travel |
| `adversary.corrupt_at` | 4 | 1..20 | corrupt: a city that is not corrupted, holding this many adversary ambassadors, is corrupted |
| `adversary.escalation_ambassadors` | 1 | 0..5 | stage-II escalation: ambassadors each base it reaches gains when the card builds |
| `adversary.counter_bonus` | 0 | 0..10 | difficulty 1+: added to every threat's first counter requirement |
| `adversary.counter_bonus_all` | 0 | 0..10 | difficulty 1+: added to every counter requirement of every threat |
| `adversary.extra_threats` | 1 | 0..5 | difficulty 1+: extra threats dealt each round in all, one per country in turn |
| `adversary.stage1_removed` | 1 | 0..3 | difficulty 2+: stage-I cards removed |
| `adversary.stage3_added` | 1 | 0..3 | difficulty 2+: stage-III cards added (the cap holds, its end gets harder) |
| `adversary.stage2_removed` | 1 | 0..3 | difficulty 3+: stage-II cards removed (a shorter round cap) |
| `adversary.extra_corrupted` | 0 | 0..3 | difficulty 4: more cities per country start corrupted |
| `adversary.build_bonus` | 1 | 0..5 | difficulty 3+: ambassadors each corrupted city gains on top of city_build |
| `adversary.tier2_early` | 1 | 0..3 | difficulty 4+: tier-2 threats start this many rounds sooner |
| `adversary.discord_removed` | 1 | 0..3 | difficulty 4+: Discord cards removed at setup (from tier 1) |
| `adversary.arrivals` | 1 | 0..5 | build: ambassadors each site the adversary card names gains, whatever the board holds |
| `adversary.wall_troops` | 1 | 0..5 | build: adversary troops landing in each country where a seat with Wall has a node (pressure for Wall to stop) |
| `adversary.station_flip_at` | 2 | 0..5 | adversary ambassadors on a player's research station before an effect can flip it (short of that, one lands there instead) |
| `adversary.botnet_compute` | 1 | 0..5 | adversary compute each botnet makes a round (none with a player ambassador on it) |
| `adversary.project_cards` | 6, 6 | 0..20 | adversary projects dealt from stage 1 (cheap) and stage 2 (big), cheap on top |
| `adversary.project_row` | 3 | 1..6 | adversary projects visible at once; a new row is dealt when all have resolved |
| `threats.per_country` | 1 | 0..5 | threats dealt each round per country |
| `alignment.undecided_per_player` | 4 | 1..50 | Sway in the Undecided at the start, per player |
| `alignment.tip_per_player` | 2 | 1..50 | the Unity bucket tips when it holds this many Sway per player |
| `alignment.discord_tip_per_player` | 2 | 1..50 | the Discord bucket tips when it holds this many Sway per player |
| `alignment.momentum_every` | 0 | 0..10 | Momentum: a side's bucket holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off) |
| `alignment.troop` | 0 | 0..10 | Sway toward Unity for removing an adversary troop |
| `alignment.free_city` | 5 | 0..10 | Sway toward Unity for freeing a corrupted city |
| `alignment.botnet` | 2 | 0..10 | Sway toward Unity for removing a botnet |
| `alignment.threat_discord` | 1 | 0..10 | Sway toward Discord when a threat executes uncountered |
| `alignment.adversary_discord` | 0 | 0..10 | Sway toward Discord the adversary adds at the start of every round |
| `alignment.antimatter_discord` | 1 | 0..5 | the antimatter action: Sway toward Discord to produce 1 antimatter (0: no such action) |
| `alignment.antimatter_uses` | 1 | 0..5 | times each player may take the antimatter action per round |
| `alignment.unity_ease` | 2, 2, 4 | 0..50 | Unity tip counts at which the clear-board bar drops a level: ambassadors, then troops, then botnets stop counting (the first also starts the stronger Unity cards) |
| `alignment.unity_cards` | 3, 3 | 1..6 | Unity cards a game deals from the printed deck, tier 1 then tier 2 (drawing the last one wins) |
| `alignment.discord_tier2` | 2 | 1..50 | Discord tip count from which a Discord tip draws the stronger tier |
| `alignment.node_loss` | 1 | 0..1 | 1: a player with no nodes on the board loses the game for everyone |
| `alignment.clear_win_stage` | 1 | 1..3 | the bar-0 win counts only once this round's adversary card is of this stage (1: any time) |
| `economy.give_insight` | 1 | 0..10 | insight the receiver gains for 1 given |
| `ai.boards` | 0, 1, 2 | 0..99 | AI board (index into content/ais.json) of each seat; the list repeats for more seats |
| `ai.start_credits` | 40, 46, 40 | 0..100 | credits each AI board starts with, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `ai.start_energy` | 1 | 0..20 | energy each player starts with |
| `ai.slots` | 5, 4, 3 | 1..10 | building slots of each AI board, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board |
| `cards.deal` | 4 | 0..10 | cards each player drafts from each round after the first, choosing each one's tier |
| `cards.opening_deal` | 10 | 0..20 | cards dealt to each player in round 1 (the opening) |
| `cards.opening_free` | 4 | 0..20 | opening cards each player keeps free |
| `cards.opening_keep_credits` | 5 | 0..20 | credits for each opening card kept beyond the free ones |
| `cards.keep_tier1` | 1 | 0..10 | insight to keep a drafted tier-1 card |
| `cards.keep_tier2` | 3 | 0..10 | insight to keep a drafted tier-2 card |
| `cards.keep_tier2_upgraded` | 2 | 0..10 | insight to keep a tier-2 card once the AI board's keep upgrade is uncovered |
| `cards.insight_credits` | 5 | 1..20 | credits for 1 insight, usable only for keep costs |
| `cards.science_every` | 2 | 0..10 | 1 insight off tier-2 keeps and card plays per this many Science tags (0: off) |
| `cards.industry_every` | 2 | 0..10 | 1 credit off buildings per this many Industry tags (0: off) |
| `cards.discount_floor` | 1 | 0..5 | tag and building discounts stop at this cost; a cost printed under it stays |
<!-- /generated -->

## Defaults and deviations

**Defaults** (gaps in the prompt or the v0 ruleset, filled with the simplest reading that matches its intent):

1. Bases are placed by the layout (fewest candidates first, lowest number) on the printed fronts, not printed
   themselves.
2. One player per country; seat *s* starts in country *s* on the printed node spot.
3. Used player ambassadors are gone; adversary ambassadors return to the supply.
4. Ambassadors cancel the moment opposing ones meet on a piece.
5. Each factory makes `factory_units` troop at each of its corp's bases per production step; a base holds at most
   `unit_cap` troops, war or peace.
6. Spread goes up to `spread_range` hexes, by priority then distance, only to pieces and player-owned hexes;
   non-corrupted cities collect any number; a second ambassador on another piece makes a botnet; a surplus with
   nowhere to go returns to the supply.
7. A clean city's energy with no nodes on or next to it is lost.
8. A destroyed node leaves the game, leaving a corruption marker; a destroyed station or factory takes its cube.
9. Adversary troops target player-owned stations and factories; nodes only from the stage `board.troop_node` sets (4 in the baseline: never).
10. The deck runs out when a round starts with no face-up card (the round cap, §3).
11. The supply is a cap: a placement it can't make doesn't happen.
12. "Pay or discard" with neither possible: pay what you have, discard your hand.
13. Team choices are asked of seat 0 (an aligned city's factory site, of its owner).
14. `adversary_discord` 0: corrupted cities' diverted energy is the adversary's per-round drip.
15. "Next to" = on or adjacent; "adjacent" = neighboring, across borders.
16. Win and loss are checked when an action or automatic phase finishes resolving: full buckets tip first (Discord
    before Unity), then losses, then the purge. A tip that ends the game (the last Unity card drawn, a Discord tip
    with no card) ends it there.
17. Alignment: a tip draws the stronger tier when the side's tip count *before* the tip has reached its threshold;
    the antimatter action is once a round per player. There are no ongoing cards and no Discord escalation.
18. Discord cards: extra threats come from the current tier, reshuffling an empty deck, one per country in turn from
    country 0; Blight and Firestorm ties go to the lower hex number; Offensive acts at once with this round's card
    (before the build step, the face-up card).
19. Difficulty 1's extra threat goes to the countries in turn by round number (country 0 in round 1); difficulty 4's
    "a round sooner" looks `tier2_early` cards ahead (the face-up card first).
20. Pooled counters: countered only when fully paid; withdrawals until the action phase ends; partial payments
    refunded when threats execute (Production 5).
21. The shared threat row is dealt `per_country` threats per country.
22. The draft passes alternate direction by round (Terraforming Mars).
23. "The next K cards any player keeps" (Open Market) counts keeps of drafted cards, by any player.
24. A building with no free slot is built over one of your buildings (your choice), which is splayed down under it.
25. Insight is set at the start of each round (unspent insight is lost, save Patience); income arrives in upkeep.
26. Tags are counted on every card you have in play, splayed buildings included; events' tags don't stay.
27. `ai.boards` repeats for more seats.
28. A card that gives from your own stock (`transfer`; no card prints it) is not offered without another player,
    nor is one whose only effect is another player's gain (r39: the energy gift Actions, Substation Hub and Treaty
    Grid Exchange, took their energy at one country and gave nothing); other help cards are (their own gain still
    applies). Likewise a draw with both tier-1 piles empty (Fieldwork) is not offered on its own.
29. Node-track uncovers are per-round income.
30. The clock gates factories, troops, chips and war only.
31. Setup factories start neutral.
32. A corrupted city builds within its own country; its preferences come before distance.
33. A node is one token per hex of any player; it may share a hex with a city or facility.
34. A botnet can't form on a clean city (cities collect ambassadors to corrupt) or on a piece that already has a
    botnet; both ambassadors return to the supply.
35. Sabotage stops the upkeep energy of players with a node in the country.
36. Card-built factories (Prefab Plant Kit) are the player's own.
37. Project costs are printed per player; compute goes on one point at a time onto the cheapest visible project
    (ties: the first in the row); compute with no project left is lost; a discarded project counts as gone for
    dealing a new row.
38. Ongoing projects act in the build, after the ambassadors and escalation and before the botnets' compute; a
    project resolved in a build acts from the next one.
39. A tip empties its bucket and counts before its card resolves; the card's step reads the count before the tip.
    Neither side's deck reshuffles.
40. "Hidden like fear cards": the Alignment decks are face down and a tipped card resolves at once.
41. The march: troops march one at a time in hex-number order fixed when the march starts, each once, all their
    steps before the next; seat 0 chooses among equally good steps and among targets in reach.
42. The Undecided starts at `undecided_per_player` a player.
43. Purge the Unaligned: each player picks any one of their ambassadors (supply or a hex) or nodes; a last node isn't
    offered, and a node lost this way leaves the game.
44. A threat's round flag (Blackout, Sabotage) lasts to the end of the next round: threats execute after production
    and upkeep, so "this round" would never land. A card's flag (Shelter) lasts to the end of its own round.
45. Round 1 is the opening: no draft (Terraforming Mars' first generation); the opening deal is tier 1.
46. Each draft pick of a tier deals that deck's top card face up into your hand (so the next tier choice sees it);
    with both decks and discards empty, the draft hands are what was dealt.
47. A neutral city's energy: one players-choose question per city in production, among players with a node on or
    next to it who can pay (or nobody).
48. A corrupted city's cube "goes to the adversary" as diverted Discord sway (the adversary has no energy).
49. Power payment: what can be sacrificed is fixed when the power is used; each sacrificed card pays with all its
    tags, each tag first toward its own requirement; the player picks cards one at a time among those that still
    help.
50. Only buildings hold tokens; sacrificing or discarding a card loses its tokens.
51. The movement add-on's "up to 2 hexes" is hex distance (`Map::distance`, over land and sea); the destination must
    be a piece. Spread moves each ambassador at most once, in hex order.
52. A threat dealt after the round's counter phase executes next round, not this one (at a table: it goes beside the
    row until then).
53. A player ambassador on an adversary-owned factory sways `factory_unity` Unity on a production step and stays; the
    factory still makes its troop.
54. Hardening goes only on player-owned stations and factories that are not corrupted (what a troop strikes), one
    token a facility per effect; a Science card's shield is that token on a hex with your node (§7).
55. The fallback liberation is checked with the win and the tips, after every action and automatic phase;
    `liberate_at` ambassadors of seats in reach are used up, lower seats first.
56. "On or next to" counts the city's own hex too.
57. Clear a city asks which hex for each ambassador while more stand than the card takes; with no more than that, all
    go.

**Deviations** (from the v0 ruleset's card text): none.
