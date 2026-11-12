# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, its project deck, its card deck and round cap, and difficulty. Terms and conventions: [README.md](README.md).*

## Each round

The adversary is a **board** (the Militarist's: project slots, four rules, a difficulty line, printed in
[adversary-cards.md](../adversary-cards.md); r83's **War Takes Ground** is in [war.md](war.md)), an **income** and an
**event deck** (below). Its turn, after the round's Sway step, is **four beats** (r80, ruling #41; `adversary::BEATS`). Threats still execute in their own step after production ([threats.md](threats.md)).

1. **Card**: next round's face-up event card becomes this round's (`adversary.event`; none left: below) and deals its
   threat pulls (`threats.deal`); then the build (`adversary.build`), in this order (151, Matt, locked): **Rally X**,
   **Muster N**, its other lines, at most two (an income increase, a Discord sway, a site opening), and its twists.
   Then the board's **Troops where the Protector stands** (r80, ruling #48): a normal **Muster** `wall_troops` (1) in
   each country where a seat with Wall has a node, giving Wall strikes to stop ([war.md](war.md)).
2. **Corrupt**: the card's **Corrupt X** lines, in order (`adversary.corrupt`); then **natural corruption**
   (`adversary.natural`): every city, research station or factory already holding its number of agents corrupts,
   with no move; then every hex with a player's node holding a node's number (`corrupt_node_at` 2).
3. **March** (`board.march`, [war.md](war.md)).
4. **Income**: the top card goes face up as next round's (`adversary.reveal`), the agent income into the pools, the
   **compute step** (`adversary.compute`); ongoing **projects** act; firewalls, cordons and prohibition tokens come off.

**Income (151, Matt, locked; numbers candidates).** Two incomes, lists by difficulty: **compute income**
(`compute_income`, not per country), added in the compute step, and **agent income** (`ambassador_income`) into **each
country's pool** (the supply caps it when it lands; setup fills the pools once). Boards, events and projects **raise**
an income for the rest of the game (the Militarist's **Garrisons**). No troop income.

**Rally X (151, Matt, locked; r80: round-robin, Matt's ruling #43).** Every event prints one Rally X: each country's
whole pool is **dealt one agent at a time onto each site of type X** in that country (city, coastal city, research
station, factory or botnet hex), whoever owns it, in hex order, round and round until the pool is empty. A firewalled
or no-Rally site keeps its turn and its share stays in the supply (r82; candidate); with no site of the type, nothing
lands. A cordoned country's pool waits for its next Rally. A project's "Rally 2: factories" is 2 a country, by the same
rule. An arrival on player ambassadors cancels one (players choose whose). A threat places its own agents "here".

**A blocked landing stays in the supply (175).** A **firewall** ([ambassadors.md](ambassadors.md)), to the end of the
next adversary turn: Rally's share for its hex stays in the supply, no agent moves onto it, its site doesn't corrupt.
A Unity **prohibition** (r80, ruling #53) puts a no-Rally, no-Muster or no-Corrupt token on every hex where its
condition holds **when the card resolves** ([alignment.md](alignment.md)); that step skips the hex (no-Corrupt: lines,
natural, flips, threats and projects), and the tokens come off with the firewalls. A Muster onto a hex Wall holds, or with a no-Muster token, stays in the supply.

**Muster N (175).** N troops on each country's lowest-numbered factory, anyone's, else its lowest-numbered city:
an event's line and the Protector's (r80), each +1 from the difficulty surcharge. No project musters (r85).

**Corrupt X (175, Matt's ruling; numbers candidates).** X is a type: **cities** (`corrupt_at` 4), **coastal cities**
(a city's number), **research stations** (`corrupt_station_at` 3), **factories** (`corrupt_factory_at` 3), **open
land** (`corrupt_land_at` 3; an empty land hex: no city, base or building) or **nodes** (`corrupt_node_at` 2; a hex
with a player's node). Only an X not yet the adversary's counts. In each country: **every agent next to an X moves
onto it** (across borders too; the players choose for an agent next to several; agents already on an X stay); then
**every X holding its number of agents corrupts** (markers never count, r80), and its agents go back to the supply.
A line may print "(1 fewer)"; difficulty lowers a city's, station's and factory's number (`corrupt_cut`; not land's
or a node's). **One aftermath (r80, Matt's ruling #42): whatever corrupts takes one adversary mark.**
- **A corrupted site** takes the adversary's cube (a player cube and any marker come off; [ambassadors.md](ambassadors.md)).
  **Corruption destroys only a node on the corrupted hex itself**; its owner may spend a protection, such as their
  own ambassador there ([war.md](war.md)). Player ambassadors left on the hex are lost. Sanctuary (capstone): corruption never destroys your nodes.
- **A corrupted node hex** loses the node (the protections apply, [war.md](war.md)) and **corrupts like open land**:
  it takes a land-corruption token, no marker, whether or not a protection saves the node (r83, Matt's ruling #56).
  Its agents go to the supply. A last node is no exception (175).
- **Land corruption** is a hex token: nothing is built there, and an open-land token drips `blight_discord` (1)
  Discord a round; a node hex's goes **face down** (r84, candidate): it drips `blight_node_discord` (0). The Clear
  action takes one in reach off (no reward).
**Botnets.** Each country starts with one (on its research station); more come only from printed lines. A botnet stays
until removed: it makes `botnet_compute` compute (none with a player ambassador on it) and counts against the purge
until Unity eases it ([alignment.md](alignment.md)). Removed by tier-2 cards, Decommission and the Take down a botnet
action (`botnet_credits` 5, in reach with your ambassador on it); removing one sways Unity.

## The project deck (candidate)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../../content/projects.json)), shuffled at setup and dealt into the adversary board's
**slots** (the Militarist's: three open slots, r83, ruling #57) by **the draw rule**: turn up the top card; it goes
into the first empty slot it fits, else to the discard, which reshuffles when the deck runs out. **When a project
leaves**, those to its right slide one slot left and the rightmost slot refills at once (r80, ruling #49). Costs are
printed per player count (the table below).

- **The compute step (175).** All compute is added **once a round** (the Income beat, after the agent income): the
  compute income, each botnet and corrupted station (`station_cubes`) with no player ambassador on it, and all made
  since the last step (uncountered threats, cards, Cintamani), which **waits beside the projects** (public).
- **Spending:** each point onto the **leftmost** visible project (r80; `compute_every` on every one, first). A
  project holding its cost **resolves** then and there: a one-shot effect, or an **ongoing** one acting in every
  Income beat from the next round, after the compute step. **No project resolves
  outside the compute step.** With no project left, compute is lost.
- **Board verbs (r85, 195; Matt, 2026-11-11).** A project is a small, visible plan on the map (factories, stations,
  botnets, troops from adversary factories, node raids, hardening, megaproject cubes, the clock); one tax is left
  (Data Heist). A botnet's troops are made on its hex, as a factory's: Wall doesn't stop them. **Targets are read off the board as it resolves**, in each country, no marker: "the X with the most
  agents on or next to it" (the hex and its neighbors), "the node hex holding the most agents" (at least 1, the hex
  alone), "its lowest-numbered botnet"; ties: the lowest-numbered. "Turns neutral": the owner's cube comes off.
  **Factory troops** are an adversary factory's own output (a player ambassador on it stops them). **The clock**:
  the top event card is discarded, the round cap a round nearer.
- **Counterplay:** an ambassador on a botnet or a corrupted station; Cyber's hacks; the drain standard project
  ([economy.md](economy.md)), off a project, not the waiting compute.

<!-- generated: projects -->
| Project | Tags | 1 player | 2 players | 3 players | 4 players | Resolves |
|---|---|---|---|---|---|---|
| Factory Garrisons | security | 12 | 24 | 36 | 48 | ongoing |
| Night Raid | security | 9 | 18 | 27 | 36 | once |
| War Footing | security | 7 | 14 | 21 | 28 | once |
| Shadow Factories | industry | 7 | 14 | 21 | 28 | once |
| Hostile Takeover | industry | 8 | 16 | 24 | 32 | once |
| Asset Seizure | industry | 9 | 18 | 27 | 36 | once |
| War Plants | industry | 11 | 22 | 33 | 44 | once |
| Botnet Swarm | cyber | 9 | 18 | 27 | 36 | once |
| Station Coup | cyber | 9 | 18 | 27 | 36 | once |
| Backdoor Research | cyber | 5 | 10 | 15 | 20 | once |
| Data Heist | cyber | 6 | 12 | 18 | 24 | once |
| Botnet Conscripts | cyber | 7 | 14 | 21 | 28 | once |
| Recruitment Drive | diplomacy | 11 | 22 | 33 | 44 | once |
| Infiltration Drive | diplomacy | 14 | 28 | 42 | 56 | once |
| Mob Rule | diplomacy | 8 | 16 | 24 | 32 | once |
| Disinformation | diplomacy | 5 | 10 | 15 | 20 | once |
| Rolling Blackouts | power | 6 | 12 | 18 | 24 | once |
| Sabotage Teams | power | 6 | 12 | 18 | 24 | once |
| False Flag | universal | 6 | 12 | 18 | 24 | once |
| Grand Offensive | universal | 10 | 20 | 30 | 40 | once |
| Propaganda Network | universal | 14 | 28 | 42 | 56 | ongoing |
| Accelerated Timetable | universal | 14 | 28 | 42 | 56 | once |
<!-- /generated -->

Three projects raid a node (Night Raid, Mob Rule, Sabotage Teams; r85): a protection saves it as from any
destruction. Texts: [adversary-cards.md](../adversary-cards.md).

## The event deck (the clock and the stager)

A game deals `stage_cards` at random per stage (counts below), each stage shuffled on its own, stage I on top: as many
rounds as cards, **the round cap** (**locked** that a cap exists and can shorten; the number a candidate). A card
prints its lines in the Card and Corrupt beats' order (`check.py` refuses another, or more than two other lines).

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
| Card | Stage | Threat (each country) | Rally | Muster | Corrupt |
|---|---|---|---|---|---|
| Probing Strikes | 1 | tier 1 (security) | coastal cities | 1 | cities |
| Factory Drive | 1 | tier 1 (security or industry) | factories | 1 | factories |
| Long Patrols | 1 | tier 1 (security or cyber) | research stations | 0 | research stations |
| War Drums | 1 | tier 1 (security or diplomacy) | cities | 1 | cities |
| Signal Jamming | 1 | tier 1 (security or power) | botnet hexes | 0 | cities |
| Mobilization | 2 | tier 2 (security) | coastal cities | 1 | cities |
| Station Raids | 2 | tier 2 (security or cyber) | research stations | 1 | research stations |
| Fifth Column | 2 | tier 2 (diplomacy or cyber) | cities | 0 | cities (1 fewer) |
| Border War | 2 | tier 2 (security) | factories | 1 | factories |
| Supply Lines | 2 | tier 1 (security or industry); tier 2 (security or industry) | factories | 1 | factories |
| Forced March | 2 | tier 2 (any) | cities | 1 | cities |
| Total Mobilization | 3 | tier 2 (security); tier 2 (industry or power) | factories | 2 | factories, cities |
| Blitzkrieg | 3 | tier 2 (any); tier 2 (security or diplomacy) | cities | 2 | cities, open land |
| Occupation | 3 | tier 2 (security or cyber); tier 2 (diplomacy) | research stations | 1 | research stations, cities (1 fewer) |
| Naval Blockade | 3 | tier 2 (industry or power); tier 2 (security or cyber) | coastal cities | 2 | cities, open land |
<!-- /generated -->

**For new players: the adversary's turn** is four beats. **Card:** a **threat** in each country; **rally** (each
country's pool dealt one agent at a time onto the sites named, in number order); **muster** (troops on each country's
lowest-numbered factory). **Corrupt:** "Corrupt cities": every agent next to a city moves onto it, then every city
holding 4 corrupts, destroying a node on that hex only, unless your ambassador stands there; then any hex with your
node and 2 agents loses the node and is corrupted land. **March:** the troops. **Income:** agents into the pools, and
all the round's compute onto the leftmost project, which when full does one small thing on the map (takes a factory,
spreads a botnet, raids the node hex with the most agents): look at it and the board to see where. The card ahead and
the pools are face up: armor or clear sites a round ahead. **What resisted this:** an agent next to two sites asks the players to choose.

Players **lose** when the Card beat finds no card to turn face up, after the round's Sway step and any tip
it makes ([alignment.md](alignment.md)): a plain loss (Matt, 2026-11-08).

## Difficulty (`difficulty`, 0–4; each level keeps the steps below it)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules; the incomes, an entry a level | `compute_income` 1, 4, 19, 24, 24, `ambassador_income` 0, 2, 2, 3, 3 |
| 1 | City, station, factory corrupt with fewer agents (from `corrupt_cut_at`); counters cost more: every resource item, a credits item by its own step, the first option's more still (from `counter_bonus_at`) | `corrupt_cut` 1, 1, 1, 0, `corrupt_cut_at` 1, `counter_bonus_all` 1, `counter_bonus_credits` 4, `counter_bonus` 0, `counter_bonus_at` 1 |
| 2 | Counters cost more again (from `counter_more_at`); the adversary board's difficulty line (from `board_difficulty_at`) | `counter_more_all` 1, `counter_more_credits` 0, `counter_more_at` 2, `board_difficulty_at` 2 |
| 3 | Stage-II cards removed, stage-I cards added back | `stage2_removed` 2, `stage1_added` 0 |
| 4 | Tier-2 threats sooner (the Discord deck as at level 3: `discord_removed` 0); a stage-I card swapped for a stage-III card (the table above; from `stage_swap_at`) | `discord_removed` 0, `tier2_early` 1, `extra_corrupted` 0, `stage1_removed` 1, `stage3_added` 1, `stage_swap_at` 4 |
<!-- /generated -->

**The ladder raises intensity, not count** (142, Matt; [threats.md](threats.md)). d4's "tier-2 threats sooner" deals
a stage-I card's pulls from tier 2 once a stage-II card is `tier2_early` cards ahead. Discord tips don't raise the
level. **Claim and liberate costs are the same at every difficulty.**
