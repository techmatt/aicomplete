# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, projects, event deck and difficulty. Terms: [README.md](README.md). Source:
`drafts/adversary/`, loaded into `content/` by `scripts/content/load_adversary.py`: edit the drafts.*

## Each round

The adversary is a **board** (the Militarist's slots, rules and named levels:
[adversary-cards.md](../adversary-cards.md); **War Takes Ground**: [war.md](war.md)), an **income** and an **event
deck**. Its turn, after the round's Sway step, is **four beats** (phases: [economy.md](economy.md)); threats execute after production ([threats.md](threats.md)).

1. **Card**: next round's face-up event card becomes this round's (none left: a loss,
   [alignment.md](alignment.md)) and deals its threat pulls; then, in this order (locked): **Rally X**, **Muster N**, at most two other lines (an income increase, a Discord
   sway, a site opening, a botnet spread), its twists.
2. **Corrupt**: the card's **Corrupt X** lines, in order; then **natural corruption**: every city, research station
   or factory already holding its number of agents corrupts, with no move; then 2 agents on a player's node hex
   destroy the node (below).
3. **March** ([war.md](war.md)).
4. **Income**: the top card goes face up as next round's, the agent income into the pools, the **compute step**,
   the **research check** (below); firewalls, cordons and prohibition tokens come off.

**Research (259, Matt's ruling: every adversary).** Each adversary board prints a **research rule**: one public
**research track**. Cubes come from its corrupted stations once their country's megaproject is complete (`station_cubes`
a round, in place of compute; none under a player ambassador), from projects' "cubes onto the research track" lines
(Backdoor Research, Lab Seizure, Station Coup: 2 a country) and from threats' uncountered lines (Wiper Malware,
Data Breach, Zero Day). Once a round, after the compute step, a track holding **`research_at` a country** (5, 4, 4, 4,
4 at d0-d4, times the countries) spends that many, keeps the rest, and fires the rule once; from
`research_strong_at` (3) its stronger effect. The Militarist's is **Weapons Program**: Muster 2 (each country, 2
troops on its lowest-numbered factory, else city), stronger Muster 3 (level 3's rule, Arms Race).
Players take cubes off with Counterintelligence Sweep (4) and Arms Inspectors (2 when built, 262, and an Action, 2). `research_at` all 0 is
the off arm: stations make compute again.

**Income (locked).** Two, by difficulty: **compute income** (`compute_income`, **a
country**: every compute source scales with the boards), added in the compute step, and **agent income**
(`ambassador_income`) into **each country's pool** (the supply caps it). Boards, events and projects **raise** an
income for good (the Militarist's **Garrisons**; the event cards' +1 agent); "+N compute" is N a country. No troop
income.

**Rally X (locked).** Every event prints one Rally X: each country's whole pool is **dealt one agent at a time onto
each site of type X** in that country (city, coastal city, research station, factory or botnet hex), whoever owns it,
in hex order, round and round until the pool is empty; with no site of the type, nothing lands. A cordoned country's
pool waits. A project's "Rally 2: factories" is 2 a country. An arrival on player ambassadors cancels one (players
choose whose).

**A blocked landing stays in the supply:** a **firewall** or a Unity **prohibition** token (no-Rally, no-Muster,
no-Corrupt) blocks its step on its hex ([README.md](README.md#terms)); so does Wall for a Muster ([war.md](war.md)).

**Muster N.** N troops on each country's lowest-numbered factory, anyone's, else its lowest-numbered city. No
project musters.

**Corrupt X.** X is a type: **cities** (`corrupt_at` 5, printed on the map; from d1 Sleeper Cells makes it 3: `corrupt_cut`), **coastal cities** (a city's number),
**research stations** (`corrupt_station_at` 2, printed), **factories** (`corrupt_factory_at` 2, printed), **open land**
(`corrupt_land_at` 3; an empty land hex; it takes Fallout). Only an X not yet the adversary's counts. In each country:
**every agent next to exactly one X moves onto it** (across borders too; one next to two or more stays); then **every
X holding its number of agents corrupts**, and its agents go to the supply. A line may print "(1 fewer)"; of the
ladder only Sleeper Cells lowers one, a city's. A neutral site holding a
**Liberated marker** (a player liberated it, [ambassadors.md](ambassadors.md)) needs `retake_agents` (1) agent more, in
Corrupt lines and natural corruption alike (253); a node on that hex takes none. **One aftermath: whatever
corrupts takes one adversary mark.**
- **A corrupted site** takes the adversary's cube (replacing a player's). **It destroys only a node on its own hex**
  (protections apply, [war.md](war.md)); player ambassadors there are lost. Sanctuary (capstone): neither corruption
  nor agents destroy your nodes.
- **2 agents on a player's node hex destroy the node** (`corrupt_node_at`; a node rule, not corruption: protections
  apply; a city hex loses its node at 2, though the city corrupts at 5 at d0 and 3 from d1); its agents go to the supply and **it leaves
  no mark**.
- **Fallout** is the one scar: a hex token on free open land; nothing is built there, and each drips `blight_discord`
  (1) Discord a round. All four adversity sources print it (threats, projects by a country's corrupted stations or
  cities, Scorched Earth; which ones: [harms.md](../harms.md)); also Corrupt open land and Teardown (on their own hex,
  Teardown's under a node there). The rest land on a free open hex (no facility, node or Fallout), a threat's on or
  next to its anchor, the others next to it (ties: lower country, then number); none free, it is lost. Lift Fallout
  (a standard action, [economy.md](economy.md)) and some cards take one in reach off.
**Botnets spread quickly**. Each country starts with one ([board.md](board.md)); more come from printed lines,
among them four event cards' **Botnets spread (level 1+)** and Botnet Swarm's (as printed; with no such site, none;
below `botnet_spread_at`, 1, the line does nothing: at d0 the setup botnets are the only ones, 263; the cards print
the level since 270, so it is no level's rule). A botnet stays until removed:
it makes `botnet_compute` compute (none with a player ambassador on it) and counts against the purge until Unity
eases it ([alignment.md](alignment.md)). Removed by tier-2 cards, Decommission and the Take down a botnet action
([economy.md](economy.md)); removing one sways `alignment.botnet` (0: neither).

## The project deck (candidate)

One shared deck (`content/projects.json`), dealt into the board's **slots** (the
Militarist's: three open) by **the draw rule**: the top card goes into the first empty slot it fits, else to the
discard, which reshuffles when the deck runs out. **When a project leaves**, those to its right slide left and the
rightmost slot refills. **Fewer, bigger, one-shot, every board (locked):** a project prints **one cost a board** (in
play, times the countries), resolves **once**, at once (none is ongoing: a lasting effect rides on what is already
tracked: an income, pieces, Fallout, a round flag), and acts **in each country**. Small (one effect) or big (two; a
third from `THIRD_EFFECT_COST`, 18). The deck never reshuffles in a d3 game.

- **The compute step.** Once a round, after the agent income: the compute income, each botnet with no player
  ambassador on it (a corrupted station's output goes to the research track, above), and all made since the last step
  (uncountered threats, cards, Cintamani), which **waits beside the projects** (public: the warning).
- **Spending:** each point onto the **leftmost** project (`compute_every` on every one first); one holding its cost
  **resolves** at once. With no project left, compute is lost.
- **Targets are read off the board as it resolves**, in each country: "the X with the most agents on its hex"
  (the hex only, as Night Raid), "the hex holding the most player ambassadors", "its lowest-numbered botnet"; ties: the lowest-numbered. "Turns
  neutral": the owner's cube comes off. **Factory troops**: an adversary factory's own output ([war.md](war.md)).
  **The clock**: the top event card is discarded.
- **Counterplay:** an ambassador on a source; Cyber's hacks; Audit, off a project.

<!-- generated: projects -->
| Project | Tags | Cost a board | Size (effects) |
|---|---|---|---|
| Disinformation | diplomacy | 11 | small (1) |
| Backdoor Research | cyber | 14 | big (2) |
| Rolling Blackouts | power | 8 | small (1) |
| Botnet Swarm | cyber | 14 | small (1) |
| False Flag | universal | 9 | small (1) |
| Night Raid | security | 10 | small (1) |
| Containment Breach | power | 9 | small (1) |
| Crackdown | security | 8 | small (1) |
| Lab Purge | cyber | 11 | small (1) |
| Asset Seizure | industry | 13 | small (1) |
| Botnet Conscripts | cyber | 11 | small (1) |
| Purge Lists | diplomacy | 7 | small (1) |
| Dirty Bomb | power | 12 | small (1) |
| Lab Seizure | cyber | 13 | big (2) |
| Slander Campaign | diplomacy | 9 | small (1) |
| Coastal Raids | security | 9 | small (1) |
| Plant Seizure | industry | 9 | small (1) |
| Rumor Mill | diplomacy | 8 | small (1) |
| Sabotage Teams | power | 10 | big (2) |
| Mob Rule | diplomacy | 14 | big (2) |
| Data Heist | cyber | 12 | big (2) |
| Hostile Takeover | industry | 15 | big (2) |
| Station Coup | cyber | 24 | big (3) |
| War Plants | industry | 19 | big (2) |
| Recruitment Drive | diplomacy | 20 | big (2) |
| Propaganda Network | universal | 18 | big (2) |
| Infiltration Drive | diplomacy | 22 | big (3) |
| Accelerated Timetable | universal | 20 | big (2) |
| Grand Offensive | universal | 22 | big (3) |
| Martial Law | security | 15 | big (2) |
<!-- /generated -->

Texts: [adversary-cards.md](../adversary-cards.md).

## The event deck (the clock and the stager)

A game deals `stage_cards` at random per stage (counts below), each stage shuffled on its own, stage I on top: as many
rounds as cards, **the round cap** (**locked** that a cap exists and can shorten; the number a candidate). A card
prints its lines in the Card and Corrupt beats' order (`drafts/adversary/check.py` refuses another).

<!-- generated: round_cap -->
| Difficulty | Stage I cards | Stage II | Stage III | Stage II from round | Round cap |
|---|---|---|---|---|---|
| 0 | 5 | 5 | 3 | 6 | 13 |
| 1 | 5 | 5 | 3 | 6 | 13 |
| 2 | 5 | 5 | 3 | 6 | 13 |
| 3 | 5 | 3 | 3 | 6 | 11 |
| 4 | 3 | 4 | 4 | 4 | 11 |
<!-- /generated -->

<!-- generated: adversary -->
| Card | Stage | Threat (each country) | Rally | Muster | Corrupt |
|---|---|---|---|---|---|
| Probing Strikes | 1 | tier 1 (security) | coastal cities | 0 | cities |
| Factory Drive | 1 | tier 1 (security or industry) | factories | 0 | factories |
| Long Patrols | 1 | tier 1 (security or cyber) | research stations | 0 | research stations |
| War Drums | 1 | tier 1 (security or diplomacy) | cities | 0 | cities |
| Signal Jamming | 1 | tier 1 (security or power) | botnet hexes | 0 | cities |
| Mobilization | 2 | tier 2 (security) | coastal cities | 0 | cities |
| Station Raids | 2 | tier 2 (security or cyber) | research stations | 0 | research stations |
| Shell Labs | 2 | tier 2 (cyber or industry) | research stations | 0 | research stations |
| Fifth Column | 2 | tier 2 (diplomacy or cyber) | cities | 0 | cities (1 fewer) |
| Border War | 2 | tier 2 (security) | factories | 0 | factories |
| Supply Lines | 2 | tier 1 (security or industry); tier 2 (security or industry) | factories | 0 | factories |
| Forced March | 2 | tier 2 (any) | cities | 0 | cities |
| Total Mobilization | 3 | tier 2 (security); tier 2 (industry or power) | factories | 1 | factories, cities |
| Blitzkrieg | 3 | tier 2 (any); tier 2 (security or diplomacy) | cities | 1 | cities, open land |
| Occupation | 3 | tier 2 (security or cyber); tier 2 (diplomacy) | research stations | 1 | research stations, cities (1 fewer) |
| Naval Blockade | 3 | tier 2 (industry or power); tier 2 (security or cyber) | coastal cities | 1 | cities, open land |
<!-- /generated -->

**For new players:** [rulebook.md](../rulebook.md) §6. About one project a round lands; the compute pile, the card
ahead and the pools are face up.

## Difficulty (`difficulty`, 0–4; each level keeps the steps below it)

<!-- generated: difficulty -->
| Level | Name | Setup (incomes and threshold a country) | Mid-game rule (new) | Rules active | Tunable |
|---|---|---|---|---|---|
| 0 | **The Militarist** | Compute income 0; Agent income +0; Research threshold 5; Deck 5 / 5 / 3 (13 rounds); Sites print their corrupt numbers: city 5, station 2, factory 2, land 3 | — | 0 | `compute_income`, `ambassador_income`, `research_at`, `corrupt_at` |
| 1 | **Sleeper Cells** | Compute income 3; Research threshold 4 | When a Corrupt step checks a city, it falls at 3 agents, not 5. | 1 | `botnet_spread_at`, `corrupt_cut`, `corrupt_cut_at` |
| 2 | **War Footing** | Compute income 5 | When a threat executes uncountered, the adversary also gains 4 compute. | 2 | `board_difficulty_at` |
| 3 | **Arms Race** | Compute income 15; Deck 5 / 3 / 3 (11 rounds) | When Weapons Program fires, it musters 3, not 2. | 3 | `stage2_removed`, `stage1_added`, `research_strong_at` |
| 4 | **War Economy** | Compute income 18; Agent income +1; Deck 3 / 4 / 4 (11 rounds) | — | 3 | `stage1_removed`, `stage2_added`, `stage3_added`, `stage_swap_at` |
<!-- /generated -->

**The ladder (267, Matt's ruling: Spirit Island's adversary card).** Levels are named and cumulative. Each adds
**setup**, set once (income and research markers, the deck's build; sites print their corrupt numbers), and **at
most one mid-game rule**, fired by an event players already resolve (Sleeper Cells and Arms Race each replace one
number as they fire, printed whole). `design_check.py` flags over about one new at a level or three active at d4.
The table, §21 and the reference card (`web/card/adversary.json`) are generated. **No arithmetic
modifier is left:** no counter or Muster surcharge, no tier-2 look-ahead (d4 builds the deck 3 / 4 / 4).
Discord tips don't raise the level. Claim and liberate cost the same at every level.
