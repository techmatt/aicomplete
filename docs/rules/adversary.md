# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, its project deck, its card deck and round cap, and difficulty. Terms and conventions: [README.md](README.md).*

## Each round

The adversary is a **board** (the Militarist's: project slots, three rules, a difficulty line, printed in
[adversary-cards.md](../adversary-cards.md)), an **income** and an **event deck** (below). Its turn,
after the round's Sway step, is **four beats**, a strip on its board (r80, Matt's ruling #41; working names, one
place: `adversary::BEATS`). Threats still execute in their own step after production ([threats.md](threats.md)).

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
site, or one under a Rally prohibition token, is skipped and gets nothing; with every site skipped the pool goes back
to the supply; with no site of the type, nothing lands. A cordoned country's pool waits for its next Rally. A project
may print "Rally 3: botnet hexes": 3 a country, by the same rule. An arrival on player ambassadors cancels one (the
players choose whose). A threat still places its own agents "here".

**A blocked landing stays in the supply (175).** A **firewall** ([ambassadors.md](ambassadors.md)) lasts to the end
of the next adversary turn: Rally skips its hex, a Corrupt line moves no agent onto it, and the site doesn't corrupt
(agents already on it stay). A Unity **prohibition** (r80, Matt's ruling #53) puts a no-Rally, no-Muster or
no-Corrupt token on every hex where its condition holds **when the card resolves** ([alignment.md](alignment.md));
the step skips a hex with its token (no-Corrupt: lines, natural, flips and threats), and the tokens come off with the
firewalls. A Muster onto a hex Wall holds, or with a no-Muster token, stays in the supply.

**Muster N (175).** N troops on each country's lowest-numbered factory, anyone's, else its lowest-numbered city.
Every Muster is the same: an event's line, a project's ("Muster 3", Armored Columns, Declaration of War) and the
Protector's (r80); the difficulty surcharge (+1) adds to each.

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
- **A corrupted node hex** loses the node (the protections apply, [war.md](war.md)); when the node goes, the
  hex takes a land-corruption token, no marker. Its agents go to the supply. A last node is no exception (175).
- **Land corruption** is a hex token (open land that corrupts, a node hex that does): nothing is built there (no node,
  factory or research station), and each drips `blight_discord` Discord a round. The Clear action takes one in reach
  off (no reward).
**Botnets.** Each country starts with a botnet (on its research station); more come only from printed lines. A botnet stays
until removed: it makes `botnet_compute` compute in the compute step (none with a player ambassador on it) and counts
against the purge until Unity eases it ([alignment.md](alignment.md)). **Hard to remove:** tier-2 cards, Decommission
and the Take down a botnet action (`botnet_credits` 5, a botnet in reach with your ambassador on its hex). Removing one sways Unity.

## The project deck (candidate)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../../content/projects.json)), shuffled at setup and dealt into the adversary board's
**slots** (the Militarist's: one Security slot and two open ones) by **the draw rule** every tagged deck uses: turn up
the top card; it goes into the first empty slot it fits (a slot's tag, or any in an open slot; Universal fits
every slot), else to the project discard, which reshuffles when the deck runs out. **When a project leaves** (resolved
or discarded), the projects to its right slide one slot left and the rightmost empty slot refills at once (r80, Matt's
ruling #49): slot 1's Security tag binds only a card dealt into it (at setup, in practice), not one sliding in.
Costs are printed per player count (the table below).

- **The compute step (175, Matt's ruling).** All compute is added **once a round** (the Income beat, after the agent income):
  the compute income, each botnet and each corrupted research station (`station_cubes`) without a player ambassador
  on it, and everything made since the last step (uncountered threats, from d2 the board's 4 more each; cards;
  projects; Cintamani), which **waits beside the projects** (public).
- **Spending:** each point onto the **leftmost** visible project (r80; `compute_every` on every one, first). A
  project holding its cost **resolves** then and there: a one-shot effect, or an **ongoing** one acting in every
  Income beat from the next round, after the compute step. **No project resolves
  outside the compute step.** Targets are fixed (the lowest-numbered); a resource loss is "each player pays N or
  discards M cards". With no project left, compute is lost.
- **Counterplay:** an ambassador on a botnet or a corrupted station; Cyber's hacks; the drain standard project
  ([economy.md](economy.md)), off a project, not the waiting compute.

<!-- generated: projects -->
| Project | Tags | 1 player | 2 players | 3 players | 4 players | Resolves |
|---|---|---|---|---|---|---|
| Raiding Parties | security | 4 | 8 | 12 | 16 | once |
| War Footing | security | 7 | 14 | 21 | 28 | once |
| Purge the Unaligned | security | 7 | 14 | 21 | 28 | once |
| Armored Columns | security | 10 | 20 | 30 | 40 | once |
| Arms Race | security | 11 | 22 | 33 | 44 | ongoing |
| Declaration of War | security | 18 | 36 | 54 | 72 | ongoing |
| Disinformation | diplomacy | 5 | 10 | 15 | 20 | once |
| Recruitment Drive | diplomacy | 11 | 22 | 33 | 44 | once |
| Infiltration Drive | diplomacy | 18 | 36 | 54 | 72 | once |
| Data Heist | cyber | 6 | 12 | 18 | 24 | once |
| Botnet Swarm | cyber | 6 | 12 | 18 | 24 | once |
| Station Coup | cyber | 9 | 18 | 27 | 36 | once |
| Market Crash | industry | 5 | 10 | 15 | 20 | once |
| Shadow Factories | industry | 7 | 14 | 21 | 28 | once |
| Capital Flight | industry | 8 | 16 | 24 | 32 | once |
| Rolling Blackouts | power | 6 | 12 | 18 | 24 | once |
| Grid Capture | power | 9 | 18 | 27 | 36 | ongoing |
| False Flag | universal | 6 | 12 | 18 | 24 | once |
| Grand Offensive | universal | 10 | 20 | 30 | 40 | once |
| Propaganda Network | universal | 14 | 28 | 42 | 56 | ongoing |
<!-- /generated -->

No project destroys a node directly; a Corrupt line or flip a project prints is
corruption. Texts: [adversary-cards.md](../adversary-cards.md).

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
all the round's compute onto the leftmost project. The card ahead and the pools are face up: armor or clear sites a
round ahead. **What resisted this:** an agent next to two sites asks the players to choose.

Players **lose** when the Card beat finds no card to turn face up, after the round's Sway step and any tip
it makes ([alignment.md](alignment.md)): a plain loss (Matt, 2026-11-08).

## Difficulty (`difficulty`, 0–4; each level keeps the steps below it)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules; the incomes, an entry a level | `compute_income` 1, 4, 22, 24, 24, `ambassador_income` 0, 3, 3, 3, 3 |
| 1 | City, station, factory corrupt with fewer agents (from `corrupt_cut_at`); counters cost more: every resource item, a credits item by its own step, the first option's more still (from `counter_bonus_at`) | `corrupt_cut` 1, 1, 1, 0, `corrupt_cut_at` 1, `counter_bonus_all` 1, `counter_bonus_credits` 4, `counter_bonus` 0, `counter_bonus_at` 1 |
| 2 | Counters cost more again (from `counter_more_at`); the adversary board's difficulty line (from `board_difficulty_at`) | `counter_more_all` 1, `counter_more_credits` 0, `counter_more_at` 2, `board_difficulty_at` 2 |
| 3 | Stage-II cards removed, stage-I cards added back | `stage2_removed` 2, `stage1_added` 0 |
| 4 | Tier-2 threats sooner (the Discord deck as at level 3: `discord_removed` 0); a stage-I card swapped for a stage-III card (the table above; from `stage_swap_at`) | `discord_removed` 0, `tier2_early` 1, `extra_corrupted` 0, `stage1_removed` 1, `stage3_added` 1, `stage_swap_at` 4 |
<!-- /generated -->

**The ladder raises intensity, not count** (142, Matt; [threats.md](threats.md)). d4's "tier-2 threats sooner" deals
a stage-I card's pulls from tier 2 once a stage-II card is `tier2_early` cards ahead. Discord tips don't raise the
level. **Claim and liberate costs are the same at every difficulty.**
