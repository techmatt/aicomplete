# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, its project deck, its card deck and round cap, and difficulty. Terms and conventions: [README.md](README.md).*

## Each round

The adversary is a **board** (the Militarist's: project slots, three rules, a difficulty line;
[`content/adversary.json`](../../content/adversary.json)), an **income** and an **event deck** (below). Its steps in
the round:

1. **Event** (`adversary.event`): next round's face-up event card becomes this round's (none left: below).
2. **Threat** (`threats.deal`): the card's first step deals its pulls in each country ([threats.md](threats.md)).
3. **Build** (`adversary.build`), the card's next steps, always in this order (151, Matt, locked): **Rally X** (below),
   **Muster N** (below), then its other lines, at most two (an income increase, a Discord sway, a site opening). Then
   the board's rule **Troops where the Protector stands**: in each country where a seat with Wall has a node, Muster
   `wall_troops` (1) on the lowest-numbered adversary or neutral factory there, never a player's (none: none lands),
   giving Wall strikes to stop ([war.md](war.md)). Ongoing **projects** act (below).
4. **Corrupt** (`adversary.corrupt`): the card's last step, its **Corrupt X** lines, in order (below).
5. **Natural corruption** (`adversary.natural`): every city, research station or factory already holding its corrupt
   number of agents corrupts, with no move (a city's corruption markers count); then every hex with a player's node
   holding a node's number (`corrupt_node_at` 2) loses the node (below).
6. **March** (`board.march`, [war.md](war.md)), then **reveal** (`adversary.reveal`): the top card goes face up as
   next round's card, then the agent income; then the **compute step** (`adversary.compute`, below), which ends the
   adversary turn: firewalls, cordons and its prohibitions come off.

**Income (151, Matt, locked; numbers candidates).** Two incomes, lists by difficulty (step 6): **compute income**
(`compute_income`, not per country), added in the compute step, and **agent income** (`ambassador_income`) into **each
country's pool** (the supply caps it when it lands; setup fills the pools once). Boards, events and projects **raise**
an income for the rest of the game (the Militarist's **Garrisons**). No troop income: troops are printed (Muster).

**Rally X (151, Matt, locked; 175: no tiers, candidate).** Every event prints one Rally X: each country's whole pool
lands **evenly on every site of type X** in that country (city, coastal city, research station, factory or botnet hex),
whoever owns it; the remainder goes one each to the lowest-numbered such sites that aren't the adversary's or
firewalled (then the adversary's, if too few). A cordoned country's pool waits for its next Rally. A project may print
"Rally 3: botnet hexes": 3 a country, by the same rule. An arrival on player ambassadors cancels one (the players
choose whose). A threat still places its own agents "here".

**A blocked landing stays in the supply (175, one rule).** A **firewall** ([ambassadors.md](ambassadors.md)) lasts
until the end of the next adversary turn: arrivals stay off its hex (its Rally share stays in the supply; a Corrupt
line moves no agent onto it) and the site doesn't corrupt, by a line or naturally (agents already on it stay). A Unity
**prohibition** (r71) switches Rally, Muster or Corrupt off where its condition holds as the step runs, in the next
adversary turn ([alignment.md](alignment.md)): a Rally share or Muster troop stays in the supply, a site takes no
agents and doesn't corrupt (naturally either). So does a Muster onto a hex Wall holds.

**Muster N (175).** N troops on each country's lowest-numbered factory, else its lowest-numbered city. Every Muster is
the same: an event's line, a project's ("Muster 3", Armored Columns, Declaration of War) and the Protector landing
(adversary or neutral factories only); a Muster prohibition stops each, and the difficulty surcharge (+1) adds to each.

**Corrupt X (175, Matt's ruling; numbers candidates).** X is a type: **cities** (`corrupt_at` 4), **coastal cities**
(a city's number), **research stations** (`corrupt_station_at` 3), **factories** (`corrupt_factory_at` 3), **open
land** (`corrupt_land_at` 3; an empty land hex: no city, base or building) or **nodes** (`corrupt_node_at` 2; a hex
with a player's node). Only an X not yet the adversary's counts. In each country: **every agent next to an X moves
onto it** (across borders too; the players choose for an agent next to several; agents already on an X stay); then
**every X holding its number corrupts** (a city's markers count), and its agents go back to the supply. A line may
print "(1 fewer)"; difficulty lowers a city's, station's and factory's number (`corrupt_cut`; not land's or a node's).
- **A corrupted site** takes the adversary's cube (a player cube comes off; [ambassadors.md](ambassadors.md)).
  **Corruption destroys only a node on the corrupted hex itself**; its owner may spend a protection, such as their
  own ambassador there ([war.md](war.md)). Player ambassadors left on the hex are lost. Sanctuary (capstone): corruption never destroys your nodes.
- **A corrupted node hex** loses the node (the protections apply, [war.md](war.md)); its agents go to the supply and
  nothing else on the hex changes. A last node is no exception (175).
- **Land corruption** is a hex token, printed on two late cards: nothing is built there (no node, factory or research
  station), and each drips `blight_discord` Discord a round. The Clear action takes one in reach off (no reward).

**The Militarist's board rules** (Garrisons, Troops where the Protector stands, Warmongering; its difficulty line)
are printed in [adversary-cards.md](../adversary-cards.md).

**Botnets.** Each country starts with a botnet (on its research station); more come only from printed lines. A botnet stays
until removed: it makes `botnet_compute` compute in the compute step (none with a player ambassador on it) and counts
against the purge until Unity eases it ([alignment.md](alignment.md)). **Hard to remove:** tier-2 cards, Decommission
(a Unity card) and the dear Take down a botnet action (`botnet_credits` 5 credits, your ambassador on its hex, which
stays; r75). Removing one sways `alignment.botnet` Unity.

## The project deck (candidate)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../../content/projects.json)), shuffled at setup and dealt into the adversary board's
**slots** (the Militarist's: one Military slot and two open ones) by **the draw rule** every tagged deck uses: turn up
the top card; it goes into the first empty slot it fits (a slot's tag, or any in an open slot; Universal fits
every slot), else to the project discard, which reshuffles when the deck runs out. **Each slot refills as it
empties** (175), at once.
Costs are printed per player count (the table below).

- **The compute step (175, Matt's ruling).** All compute is added **once a round** (step 6, after the agent income):
  the compute income, each botnet and each corrupted research station (`station_cubes`) without a player ambassador
  on it, and everything made since the last step (uncountered threats, from d2 the board's 4 more each; cards;
  projects; Cintamani), which **waits beside the projects** (public).
- **Spending:** each point onto the **cheapest** visible project (ties: the first slot; `compute_every` on every
  one, first). A project holding its cost **resolves** then and there: a one-shot effect, or an **ongoing** one acting
  in every build from the next round. **No project resolves
  outside the compute step.** Targets are fixed (the lowest-numbered); a resource loss is "each player pays N or
  discards M cards". With no project left, compute is lost.
- **Counterplay:** an ambassador on a botnet or a corrupted station; Cyber's hacks; the drain standard project
  ([economy.md](economy.md)), off a project, not the waiting compute.

<!-- generated: projects -->
| Project | Tags | 2 players | 3 players | 4 players | Resolves |
|---|---|---|---|---|---|
| Raiding Parties | military | 8 | 12 | 16 | once |
| War Footing | military | 14 | 21 | 28 | once |
| Purge the Unaligned | military | 14 | 21 | 28 | once |
| Armored Columns | military | 20 | 30 | 40 | once |
| Arms Race | military | 22 | 33 | 44 | ongoing |
| Declaration of War | military | 36 | 54 | 72 | ongoing |
| Disinformation | influence | 10 | 15 | 20 | once |
| Recruitment Drive | influence | 22 | 33 | 44 | once |
| Infiltration Drive | influence | 36 | 54 | 72 | once |
| Data Heist | cyber | 12 | 18 | 24 | once |
| Botnet Swarm | cyber | 12 | 18 | 24 | once |
| Station Coup | cyber | 18 | 27 | 36 | once |
| Market Crash | economic | 10 | 15 | 20 | once |
| Shadow Factories | economic | 14 | 21 | 28 | once |
| Capital Flight | economic | 16 | 24 | 32 | once |
| Rolling Blackouts | grid | 12 | 18 | 24 | once |
| Grid Capture | grid | 18 | 27 | 36 | ongoing |
| False Flag | universal | 12 | 18 | 24 | once |
| Grand Offensive | universal | 20 | 30 | 40 | once |
| Propaganda Network | universal | 28 | 42 | 56 | ongoing |
<!-- /generated -->

No project destroys a node directly; a Corrupt line or flip a project prints is
corruption. Texts: [adversary-cards.md](../adversary-cards.md).

## The event deck (the clock and the stager)

A game deals `stage_cards` at random per stage (counts below), each stage shuffled on its own, stage I on top: as many
rounds as cards, **the round cap** (**locked** that a cap exists and can shorten; the number a candidate). A card
prints its steps in the order of *Each round* (`drafts/adversary/check.py` refuses another, or more than two other
lines).

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
| Blitzkrieg | 3 | tier 2 (any); tier 2 (military or influence) | cities | 2 | cities, open land |
| Occupation | 3 | tier 2 (military or cyber); tier 2 (influence) | research stations | 1 | research stations, cities (1 fewer) |
| Naval Blockade | 3 | tier 2 (economic or grid); tier 2 (military or cyber) | coastal cities | 2 | cities, open land |
<!-- /generated -->

**For new players: the adversary's turn.** Its card, in order: a **threat** in each country; **rally** (each country's
pool lands evenly on every site named, the odd ones on the lowest-numbered not the adversary's); **muster** (troops on
each country's lowest-numbered factory); **corrupt** ("Corrupt cities": every agent next to a city moves onto it, then
every city holding 4 corrupts, destroying a node on that hex only, unless your ambassador stands there). Then any hex
with your node and 2 agents loses the node. Then **income**: agents into the pools, and all the round's compute onto
the projects at once. The card ahead and the pools are face up: armor or clear sites a round ahead. **What resisted
this:** the even spread asks a little division, and an agent next to two sites asks the players to choose.

Players **lose** when step 1 finds no card to turn face up, after the round's Sway step and any tip
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

**The ladder raises intensity, not count** (142, Matt): no level adds a threat (`extra_threats_at` 5 is past
the top level). d4's "tier-2 threats sooner" deals a stage-I card's pulls from tier 2 once a stage-II
card is `tier2_early` cards ahead. Discord tips don't raise the level. **The players' claim and liberate costs are
the same at every difficulty.**
