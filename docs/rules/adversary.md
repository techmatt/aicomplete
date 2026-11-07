# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, its project deck, its card deck and round cap, and difficulty. Terms and conventions: [README.md](README.md).*

## Each round

The adversary is a **board** (the Militarist's: its project slots, two rules and a difficulty line,
[`content/adversary.json`](../../content/adversary.json)) and an **event deck** (below). Its steps, in the round
([economy.md](economy.md)):

1. **Event** (`adversary.event`): next round's face-up event card becomes this round's (none left: players lose).
2. **Threats** (`threats.deal`): the card's top line deals its pulls ([threats.md](threats.md)).
3. **Build** (`adversary.build`), the rest of the card top to bottom. Its **standing lines**: compute, troops ("in each
   country, 1 troop lands on a factory, else a city") and its **creation lines** (below). Then its **twists**
   (immediate ones resolve now: Sway, a Unity pull, node losses; the rest are read where they act). Then the
   **board's creation line** (the Militarist's: create 2 on the lowest-numbered corrupted city; `build_bonus` more
   from difficulty 3). **Troops where the Protector stands** (the board's rule): in each country where a seat with Wall
   has a node, `wall_troops` troops land as any landing does (on a factory, players choose, else a city; never a hex
   Wall holds), giving Wall strikes to stop ([war.md](war.md)). Ongoing **projects** act (below). Last, each **botnet**
   makes `botnet_compute` **adversary compute**, onto the projects (none under a player ambassador).
4. **Corrupt** (`adversary.corrupt`): the card's **Corrupt X** lines, in order (below).
5. **Natural corruption** (`adversary.natural`): every city, research station or factory already holding its corrupt
   number of adversary ambassadors corrupts, with no move (a city's corruption markers count).
6. **March** (`board.march`, [war.md](war.md)), then **reveal** (`adversary.reveal`): the top card goes face up as
   next round's card.

**Creation lines (138, candidate): bounded arrivals.** Every line that makes adversary ambassadors has one shape:
*create N per board on the lowest-numbered X [condition]*, or *on each of the K lowest-numbered Xs*. X is a city, coastal
city, research station, factory or botnet hex (a threat may say "here"); the conditions are clean (not the adversary's),
corrupted, a player's, holding adversary ambassadors, with none, or on or next to a threat. "Per board" is each country;
a threat's line reaches its own country. Sources: the event cards and the board each build, threats, projects and
Discord cards; stage I creates 1 a site, later stages more. Only a Corrupt line moves them. A cordoned country gains none; Trade Blockade doubles a creation on or next to its flag.
**Site creation (candidate):** a few tier-1 events, Discord and Unity cards open a **neutral** research station on each
board's lowest-numbered empty station site, or a neutral factory on its lowest-numbered free industrial zone: new
targets for both sides.

**Corrupt X (138, candidate).** X is a site type: **city** (`corrupt_at` 4), **research station**
(`corrupt_station_at` 3), **factory** (`corrupt_factory_at` 3) or, rarely, **land** (`corrupt_land_at` 3; an empty land
hex: no city, base or building). In each country, take every X not already the adversary's, lowest-numbered first: if it
can gather its number n of adversary ambassadors with one move each (those on it plus those next to it in its country),
it pulls them in (just enough, from the lowest-numbered hexes first) and corrupts; every ambassador on it returns to
the supply. An ambassador pulled is gone for the next site, but **every site that qualifies corrupts**, so a crowded
board cascades. A line may print "(1 fewer)". Content prints Corrupt lines on events (every card has one or two),
threats, projects and Discord cards. **Difficulty lowers n** by `corrupt_cut` (city, station, factory, land) from
`corrupt_cut_at`.
- **A corrupted site** takes the adversary's cube (a player cube comes off; [ambassadors.md](ambassadors.md)).
  **Corruption destroys only a node on the corrupted hex itself**; its owner's ambassador there is armor, used up
  instead (137). Player ambassadors left on the hex are lost. Sanctuary (capstone): corruption never destroys your nodes.
- **Land corruption** is a hex token, printed on two late cards (Blitzkrieg, Naval Blockade): no node may be placed
  there, and each drips `blight_discord` Discord a round.

**The Militarist's board rules.** *Troops where the Protector stands* (above). *Warmongering:* each round, a
military base with an adversary troop on or next to it makes 1 extra corp troop, even past its cap; a base gains at most
1 extra corp troop a round from every such rule together (Warmongering, Conscription, Arms Race; Matt,
2026-11-06). Its difficulty line, from `board_difficulty_at`: every standing line that lands troops lands 1 more in
each country.

**Botnets.** Each board starts with a botnet (on its research station); more come only from printed lines (Sleeper
Cells, Full Spectrum Assault, Cascade Failure place one; 138: no botnet forms from ambassadors meeting). A botnet stays
until removed: it makes `botnet_compute` adversary compute a round, it **blocks site claims** (a site with a botnet on
or next to it can't be claimed), it counts against the purge until Unity eases it ([alignment.md](alignment.md)), and
a few creation lines name botnet hexes (Signal Jamming, Botnet Swarm). **Botnets are hard to remove (candidate,
Matt):** only tier-2 effects do it (Botnet Takedown, Singularity Exploit, Decapitation Strike and Root Access in your
reach; the Unity card Decommission anywhere), no standard action. Removing one sways `alignment.botnet` Unity.

## The project deck (candidate)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../../content/projects.json)), shuffled at setup and dealt into the adversary board's
**slots** (the Militarist's: one Military slot and two open ones) by **the draw rule** every tagged deck uses: turn up
the top card; it goes into the first empty slot it fits (a slot's tag, or any project in an open slot; Universal fits
every slot), else to the project discard; the discard reshuffles when the deck runs out; a slot still empty after one
reshuffle takes the next card anyway (counted: a content bug). When every slot's project has
resolved (or been discarded), the slots refill.

- **Compute sources:** each botnet, `botnet_compute` a round in the build (none under a
  player ambassador); most threats carry compute, gained when they execute uncountered
  ([threats.md](threats.md)); the event cards' standing lines; an adversary ambassador draining a lone node, 1 in
  production ([ambassadors.md](ambassadors.md)); each adversary research station, `station_cubes` a round in
  production (137); the corp cards, an adversary cube at Cintamani's completion ([board.md](board.md)).
- **Spending:** each compute point goes onto the **cheapest** visible project (ties: the
  first slot); content may print `compute_every` (on every visible project). When a project holds its cost it **resolves**: a one-shot effect at once, or
  an **ongoing** one that acts in every round's build step from the next round on (Arms Race and Declaration of War
  act where they say: production and war). Choices are the players' (seat 0; a player's own loss, each
  player's). With no project left, compute is lost.
- **Cost** is printed per player (the table below), which keeps the timing at 2, 3 and 4
  players.
- **Counterplay:** a player ambassador on a botnet stops its compute, not used up; Cyber's hacks drain 3-6 compute
  from one project or discard one; the drain standard project ([economy.md](economy.md)) does it for credits.

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
| Recruitment Drive | influence | 11 | ongoing |
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

Compute goes on one point at a time: a full project resolves before the next lands. Purge the Unaligned's loss is
any one of the player's ambassadors or nodes, their pick, never their last node. Ongoing projects act in the round's
build step only.

## The event deck (the clock and the stager)

[`content/adversary.json`](../../content/adversary.json) prints the Militarist's event cards: 5 stage I, 6 stage II,
4 stage III. A game deals `stage_cards` at random per stage, each stage shuffled on its own, stage I on top, so each
stage deals N of its N+1 printed cards at every difficulty: as many rounds as cards, **the round cap** (**locked** that
a cap exists and can shorten; the number a candidate). **11 rounds through d2:** difficulty 2 swaps a stage-I card for
a stage-III card (`stage1_removed`, `stage3_added`), a harder end, not a shorter game; difficulty 3 removes a stage-II
card (`stage2_removed`): 10. The next round's card is always face up (the `peek` op shows more; no content prints
it). A card prints, top to bottom: its pulls, its standing lines (creation lines among them), its twists and its Corrupt lines.

<!-- generated: round_cap -->
| Difficulty | Stage I cards | Stage II | Stage III | Stage II from round | Round cap |
|---|---|---|---|---|---|
| 0 | 4 | 4 | 3 | 5 | 11 |
| 1 | 4 | 4 | 3 | 5 | 11 |
| 2 | 3 | 4 | 4 | 4 | 11 |
| 3 | 3 | 3 | 4 | 4 | 10 |
| 4 | 3 | 3 | 4 | 4 | 10 |
<!-- /generated -->

<!-- generated: adversary -->
| Card | Stage | Pulls (each country) | Corrupt |
|---|---|---|---|
| Probing Strikes | 1 | tier 1 (military) | cities |
| Factory Drive | 1 | tier 1 (military or economic) | factories |
| Long Patrols | 1 | tier 1 (military or cyber) | research stations |
| War Drums | 1 | tier 1 (military or influence) | cities |
| Signal Jamming | 1 | tier 1 (military or grid) | cities |
| Mobilization | 2 | tier 2 (military) | cities |
| Station Raids | 2 | tier 2 (military or cyber) | research stations |
| Fifth Column | 2 | tier 2 (influence or cyber) | cities (1 fewer) |
| Border War | 2 | tier 2 (military) | factories |
| Supply Lines | 2 | tier 1 (military or economic) | factories |
| Forced March | 2 | tier 2 (any) | cities |
| Total Mobilization | 3 | tier 3 (military); tier 2 (economic or grid) | factories, cities |
| Blitzkrieg | 3 | tier 3 (any); tier 1 (military or influence) | cities, land |
| Occupation | 3 | tier 3 (military or cyber); tier 2 (influence) | research stations, cities (1 fewer) |
| Naval Blockade | 3 | tier 3 (economic or grid); tier 1 (military or cyber) | cities, land |
<!-- /generated -->

**For new players: corruption and arrivals.** Each round the adversary's card says where its new ambassadors go:
always the lowest-numbered sites of a kind, in every country (the board adds 2 on its lowest-numbered corrupted city).
Then the card says what it corrupts: "Corrupt cities" means every city that can gather 4 adversary ambassadors from
itself and the hexes next to it does, pulling them in. A corrupted site turns against you and destroys a node on that
hex only, unless your own ambassador stands there. Next round's card is face up, so you can see both coming: thin out the
ambassadors around a site before its Corrupt line, or stand an ambassador on your node. **What resisted this:** the
gathering count (on it plus next to it, in its own country, each ambassador counted once, lowest-numbered first) is the
one step a player has to count out by hand, and "lowest-numbered" is a printed number to look up.

**The supply is a cap.** A placement it can't make doesn't happen; running out doesn't lose.

Players **lose** when a round starts with no card to turn face up (after the round cap's
last round), a quiet backstop.

## Difficulty (`difficulty`, 0–4; each level includes the ones below)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules |  |
| 1 | Extra threats each round in all, countries in turn | `extra_threats` 1, `counter_bonus` 0, `counter_bonus_all` 0 |
| 2 | A stage-I card swapped for a stage-III card: stage II (tier-2 threats) sooner and a harder end, at the same round cap (the table above); each site type corrupts with fewer ambassadors (city, station, factory, land; from `corrupt_cut_at`) | `stage1_removed` 1, `stage3_added` 1, `corrupt_cut` 1, 1, 1, 0, `corrupt_cut_at` 2 |
| 3 | Larger builds: each site the board's creation line reaches gains more, and a stage-II card removed (a shorter round cap) | `build_bonus` 1, `stage2_removed` 1 |
| 4 | A still shorter Discord deck: the removed cards count as tips already made, so every printed step still acts; tier-2 threats sooner; optionally more corrupted cities per country at setup | `discord_removed` 1, `tier2_early` 1, `extra_corrupted` 0 |
<!-- /generated -->

Difficulty 1's extra threats are more pulls of the card's first pull, countries in turn (country 0 in round 1);
difficulty 4's "tier-2 threats sooner" deals a stage-I card's tier-1 pulls from tier 2 once a stage-II card is
`tier2_early` cards ahead (the face-up card first).

Difficulty is only the configured level: Discord tips don't raise it ([alignment.md](alignment.md)). **The
players' claim and liberate costs are the same at every difficulty** (Corrupt X's numbers fall; the claim costs don't): difficulty comes from the
adversary and decks (Discord deck size, threat strength, round cap). The target is in
`docs/principles.md`; reads are indexed in [experiments/history.md](../../experiments/history.md).
