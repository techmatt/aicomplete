# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, its project deck, its card deck and round cap, and difficulty. Terms and conventions: [README.md](README.md).*

## Each round

The adversary is a **board** (the Militarist's: project slots, three rules, a difficulty line;
[`content/adversary.json`](../../content/adversary.json)), an **income** and an **event deck** (below). "Each board"
on a card means each country: they are always the same. Its steps in the round:

1. **Event** (`adversary.event`): next round's face-up event card becomes this round's (none left: players lose).
2. **Threat** (`threats.deal`): the card's first step deals its pulls on each board ([threats.md](threats.md)).
3. **Build** (`adversary.build`), the card's next steps, always in this order (151, Matt, locked): **Rally X** (below),
   **Deploy N** (N troops on each board's lowest-numbered factory, else city; none on a hex
   Wall holds), then its other lines, at most two (an income increase, a Discord sway, a site opening). Then the
   board's rule **Troops where the Protector stands**: on each board where a seat with Wall has a node, `wall_troops`
   troops land the same way, giving Wall strikes to stop ([war.md](war.md)). Ongoing **projects** act (below). Last,
   each **botnet** makes `botnet_compute` **adversary compute**, onto the projects (none under a player ambassador).
4. **Corrupt** (`adversary.corrupt`): the card's last step, its **Corrupt X** lines, in order (below).
5. **Natural corruption** (`adversary.natural`): every city, research station or factory already holding its corrupt
   number of agents corrupts, with no move (a city's corruption markers count).
6. **March** (`board.march`, [war.md](war.md)), then **reveal** (`adversary.reveal`): the top card goes face up as
   next round's card; then **income**.

**Income (151, Matt, locked; numbers candidates).** Every adversary has two incomes (step 6):
**compute income** (`compute_income`, fixed, not per board) goes onto the projects, and **ambassador income**
(`ambassador_income`, a board) goes into **each board's pool** (the supply caps it when it lands). Setup fills the pools once,
for round 1. Both are lists by difficulty. Boards, events and projects **raise** an income for the
rest of the game ("Income: +1 ambassador a board a round"; the Militarist's **Garrisons**); player cards may lower
one (none does yet). No troop income: troops are printed (Deploy).

**Rally X (151, Matt, locked; the spread a candidate).** Every event prints one Rally X:
each board's whole pool lands on that board's sites of type X (city, coastal city, research station, factory or
botnet hex): its **neutral** ones (no owner cube), else the players', else the adversary's. The pool **spreads
evenly** over those sites; the players choose which take the remainder, one each. A cordoned board's pool lands nowhere. A project may print "Rally 3: botnet hexes": 3 a board, by the same rule. An arrival on a
player ambassador cancels it. The card ahead and the pools are public, so next round's landing is known.
A threat still places its own agents "here". A player's
**firewall** keeps every arrival and Corrupt pull off its hex for a round ([ambassadors.md](ambassadors.md)).
A Unity **prohibition** (r71) switches Rally, Deploy or Corrupt off where its condition holds as the step runs, in
the next adversary turn: a site's Rally share or a Deploy troop stays in the supply, a site doesn't corrupt (naturally
either) ([alignment.md](alignment.md)).

**Corrupt X (candidate).** X is a site type: **city** (`corrupt_at` 4), **research station**
(`corrupt_station_at` 3), **factory** (`corrupt_factory_at` 3) or, rarely, **land** (`corrupt_land_at` 3; an empty land
hex: no city, base or building). On each board, every X not the adversary's, lowest-numbered first, that can gather its number n
of agents (on it plus next to it, in its board, one move each) pulls just enough in, lowest-numbered
hexes first, and corrupts; its ambassadors return to the supply. **Every site that qualifies corrupts**: a crowded board
cascades. A line may print "(1 fewer)"; difficulty lowers n (below).
- **A corrupted site** takes the adversary's cube (a player cube comes off; [ambassadors.md](ambassadors.md)).
  **Corruption destroys only a node on the corrupted hex itself**; its owner's ambassador there is armor, used up
  instead. Player ambassadors left on the hex are lost. Sanctuary (capstone): corruption never destroys your nodes.
- **Land corruption** is a hex token, printed on two late cards: no node may be placed
  there, and each drips `blight_discord` Discord a round.

**The Militarist's board rules** (Garrisons, Troops where the Protector stands, Warmongering: a base gains at most 1
extra corp troop a round from all such rules; its difficulty line) are printed in
[adversary-cards.md](../adversary-cards.md).

**Botnets.** Each board starts with a botnet (on its research station); more come only from printed lines. A botnet stays
until removed: it makes compute (above), **blocks site claims** on or next to it and counts against the
purge until Unity eases it ([alignment.md](alignment.md)). **Hard to remove
(candidate, Matt):** only tier-2 effects (Botnet Takedown, Singularity Exploit, Decapitation Strike, Root Access) and
Decommission (a Unity card: each botnet on, then on or next to, a player's piece), no standard action. Removing one sways `alignment.botnet` Unity.

## The project deck (candidate)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../../content/projects.json)), shuffled at setup and dealt into the adversary board's
**slots** (the Militarist's: one Military slot and two open ones) by **the draw rule** every tagged deck uses: turn up
the top card; it goes into the first empty slot it fits (a slot's tag, or any in an open slot; Universal fits
every slot), else to the project discard, which reshuffles when the deck runs out. When every slot's project has
resolved (or been discarded), the slots refill.

- **Compute sources:** compute income; botnets (above); threats executing uncountered ([threats.md](threats.md)), from
  d2 the board's 4 more each; an agent draining a lone node ([ambassadors.md](ambassadors.md)); adversary
  research stations, `station_cubes` a round; Cintamani's adversary cubes ([board.md](board.md)).
- **Spending:** each compute point goes onto the **cheapest** visible project (ties: the
  first slot); content may print `compute_every` (on every visible project). When a project holds its cost it **resolves**: a one-shot effect at once, or
  an **ongoing** one that acts in every round's build step from the next round on (Arms Race and Declaration of War
  act in production and war). Targets are fixed (the lowest-numbered); a player's resource loss is "each player
  pays N or discards M cards", their choice. One consequence, rarely two. With no project left, compute is lost.
- **Counterplay:** an ambassador on a botnet; Cyber's hacks; the drain standard project ([economy.md](economy.md)).

<!-- generated: projects -->
| Project | Tags | Compute per player | Resolves |
|---|---|---|---|
| Raiding Parties | military | 4 | once |
| War Footing | military | 7 | once |
| Purge the Unaligned | military | 7 | once |
| Armored Columns | military | 10 | once |
| Arms Race | military | 11 | ongoing |
| Declaration of War | military | 18 | ongoing |
| Disinformation | influence | 5 | once |
| Recruitment Drive | influence | 11 | once |
| Infiltration Drive | influence | 18 | once |
| Data Heist | cyber | 6 | once |
| Botnet Swarm | cyber | 6 | once |
| Station Coup | cyber | 9 | once |
| Market Crash | economic | 5 | once |
| Shadow Factories | economic | 7 | once |
| Capital Flight | economic | 8 | once |
| Rolling Blackouts | grid | 6 | once |
| Grid Capture | grid | 9 | ongoing |
| False Flag | universal | 6 | once |
| Grand Offensive | universal | 10 | once |
| Propaganda Network | universal | 14 | ongoing |
<!-- /generated -->

Compute goes on one point at a time: a full project resolves before the next lands. No project destroys a node directly;
a Corrupt line or a flip a project prints (Station Coup's too) is corruption, taking a node on the hex it corrupts. Their texts are in [adversary-cards.md](../adversary-cards.md).

## The event deck (the clock and the stager)

[`content/adversary.json`](../../content/adversary.json) prints the Militarist's event cards (below). A game deals `stage_cards` at random per stage (each difficulty's counts below), each stage
shuffled on its own, stage I on top: as many rounds as cards, **the round cap**
(**locked** that a cap exists and can shorten; the number a candidate). A card prints its steps in the order of *Each round*; the load check and `drafts/adversary/check.py` refuse a card off that order, with more than
two other lines, or saying "each country".

<!-- generated: round_cap -->
| Difficulty | Stage I cards | Stage II | Stage III | Stage II from round | Round cap |
|---|---|---|---|---|---|
| 0 | 4 | 4 | 3 | 5 | 11 |
| 1 | 4 | 4 | 3 | 5 | 11 |
| 2 | 4 | 4 | 3 | 5 | 11 |
| 3 | 4 | 2 | 3 | 5 | 9 |
| 4 | 3 | 2 | 4 | 4 | 9 |
<!-- /generated -->

<!-- generated: adversary -->
| Card | Stage | Threat (each board) | Rally | Deploy | Corrupt |
|---|---|---|---|---|---|
| Probing Strikes | 1 | tier 1 (military) | coastal cities | 1 | cities |
| Factory Drive | 1 | tier 1 (military or economic) | factories | 1 | factories |
| Long Patrols | 1 | tier 1 (military or cyber) | research stations | 0 | research stations |
| War Drums | 1 | tier 1 (military or influence) | cities | 1 | cities |
| Signal Jamming | 1 | tier 1 (military or grid) | botnet hexes | 0 | cities |
| Mobilization | 2 | tier 2 (military) | coastal cities | 1 | cities |
| Station Raids | 2 | tier 2 (military or cyber) | research stations | 1 | research stations |
| Fifth Column | 2 | tier 2 (influence or cyber) | cities | 0 | cities (1 fewer) |
| Border War | 2 | tier 2 (military) | factories | 1 | factories |
| Supply Lines | 2 | tier 1 (military or economic); tier 2 (military or economic) | factories | 1 | factories |
| Forced March | 2 | tier 2 (any) | cities | 1 | cities |
| Total Mobilization | 3 | tier 2 (military); tier 2 (economic or grid) | factories | 2 | factories, cities |
| Blitzkrieg | 3 | tier 2 (any); tier 2 (military or influence) | cities | 2 | cities, land |
| Occupation | 3 | tier 2 (military or cyber); tier 2 (influence) | research stations | 1 | research stations, cities (1 fewer) |
| Naval Blockade | 3 | tier 2 (economic or grid); tier 2 (military or cyber) | coastal cities | 2 | cities, land |
<!-- /generated -->

**For new players: the adversary's turn.** Its card, in order: a **threat** on each board; **rally** (each board's
pool lands on the sites named, neutral first, spread evenly, you placing the odd one); **deploy** (troops on each
board's lowest-numbered factory); **corrupt** ("Corrupt cities": every city that can gather 4 agents
from itself and its neighbors does, destroying a node on that hex only, unless your ambassador stands there). Then
**income**: compute onto projects, ambassadors into the pools. The card ahead and the pools are face up: armor or
clear sites a round ahead. **What resisted this:** the gathering count is
counted out by hand, and the even spread asks a little division.

Players **lose** when a round starts with no card to turn face up (step 1): a backstop with no share target, a plain loss whose cases are examined (Matt, 2026-11-08).

## Difficulty (`difficulty`, 0–4; each level includes the ones below)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules |  |
| 1 | Each site type corrupts with fewer ambassadors (city, station, factory, land; from `corrupt_cut_at`); counters cost more: every resource item, a credits item by its own step, the first option's more still (from `counter_bonus_at`) | `corrupt_cut` 1, 1, 1, 0, `corrupt_cut_at` 1, `counter_bonus_all` 1, `counter_bonus_credits` 4, `counter_bonus` 0, `counter_bonus_at` 1 |
| 2 | Counters cost more again (from `counter_more_at`); the adversary board's difficulty line (from `board_difficulty_at`) | `counter_more_all` 1, `counter_more_credits` 0, `counter_more_at` 2, `board_difficulty_at` 2 |
| 3 | The level's incomes (its entries); stage-II cards removed, stage-I cards added back | `compute_income` 3, 4, 10, 8, 10, `ambassador_income` 0, 1, 1, 1, 2, `stage2_removed` 2, `stage1_added` 0 |
| 4 | A still shorter Discord deck: the removed cards count as tips already made, so the Discord tiers come a tip sooner; tier-2 threats sooner; optionally more corrupted cities per country at setup; extra threats each round in all, countries in turn; a stage-I card swapped for a stage-III card (the table above; from `stage_swap_at`) | `discord_removed` 1, `tier2_early` 1, `extra_corrupted` 0, `extra_threats` 1, `extra_threats_at` 4, `stage1_removed` 1, `stage3_added` 1, `stage_swap_at` 4 |
<!-- /generated -->

**The ladder raises intensity, not count** (142, Matt): below d4 no level adds a threat. The extra threats are more pulls of the card's first pull (country 0 in
round 1); difficulty 4's "tier-2 threats sooner" deals a stage-I card's tier-1 pulls from tier 2 once a stage-II card is
`tier2_early` cards ahead (the face-up card first).

Difficulty is only the configured level: Discord tips don't raise it. **The players' claim and liberate costs are
the same at every difficulty**: difficulty comes from the adversary and decks. The target is in
`docs/principles.md`; reads are indexed in [experiments/history.md](../../experiments/history.md).
