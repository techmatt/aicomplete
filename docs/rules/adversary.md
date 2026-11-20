# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, projects, event deck and difficulty. Terms: [README.md](README.md). Source:
`drafts/adversary/`, loaded into `content/` by `scripts/content/load_adversary.py`: edit the drafts.*

## Each round

The adversary is a **board** (the Militarist's slots, rules and named levels:
[adversary-cards.md](../adversary-cards.md); **War Rooms**: [threats.md](threats.md)), an **income** and an **event deck**. Its turn, after the round's Sway step, is **two beats** (phases: [economy.md](economy.md)); threats execute after production ([threats.md](threats.md)).

1. **Card**: this round's card moves to the board's **last-round space** (the one there goes to the discard) and
   next round's face-up card becomes this round's (none left: a loss, [alignment.md](alignment.md)) and deals its
   threats; then, in this order: **Escalate** (below), **the board's one rule** (the Militarist's **Commandeered Plants**:
   each factory not yet the adversary's that holds an agent takes 1 more, every round), **Infiltrate X**,
   **Strike**, then **Redeploy** (where the card shows the icon: [troops.md](troops.md)), at most two other lines
   (a compute increase, a Discord sway, a site opening, Botnets spread, Blackout, Fallout, Border War's node loss,
   Unity back).
2. **Income**: the top card goes face up as next round's; **outputs** (below); the **compute step**; the
   **research check**; firewalls, cordons and prohibition tokens come off.

**Escalate.** Each site of the type last round's card infiltrated (a stage-III card names two: both), not yet the
adversary's, that holds an agent takes 1 more, in every country, in hex order. Round 1 has no last-round card:
nothing escalates. It lands like any arrival (below) and picks nothing.

**One output a corrupted site**, in the Income beat. A player ambassador on the site blocks it.
- **City:** `divert_discord` (1) Discord (none under a blackout).
- **Factory:** its agents move along its zone's printed arrow onto its city ([board.md](board.md)), an arrival
  there, unless that city is the adversary's, firewalled, under a no-Infiltrate token or in a cordoned country.
  Then a factory holding no agent makes 1 on itself: it feeds its city 1 agent a round, and one whose city has
  fallen keeps a single agent (a candidate). A cordoned country's factories don't act. It makes no troop.
- **Station:** `station_cubes` (1) research cube onto the adversary's research track, from round 1.
- **Botnet:** `botnet_compute` (1) compute (none when hacked); it places no agent, and a cordon never stops it.

**Research track (every adversary).** A public row of **six icon slots**, content (the board's research row in
`content/adversary.json`): a row changes by editing it. Cubes come from corrupted stations
and from projects' and threats' "cubes onto the research track" lines ([harms.md](../harms.md)), and go nowhere else. In the research check, while the track holds **K cubes** (`research_at` a country, times
the countries; `research_start` a country at setup) it spends K and its marker advances one slot, whose icon
happens once as the marker arrives (in each country where it prints so); one check can advance several. **Slot 7
is a loss** (end reason `research`). Counterintelligence Sweep and Arms Inspectors take cubes off, never a slot back.
`research_at` all 0 is the off arm: no track, and stations make compute. **K is 2 a country** (Matt, 2026-11-19),
and the row is fed to reach its late slots when the players don't deal with it. The
Militarist's row, **War Research**: New factory, Cells: factories, Funding, **Garrison, Strike, Garrison** (each
icon's text: [adversary-cards.md](../adversary-cards.md)): a Garrison makes 1 troop a country
([troops.md](troops.md)).

**Boards differ** by their row, their event mix and their one rule.

**Income (locked).** Only **compute income** (`compute_income` by difficulty, **a country**: every compute source
scales with the boards), added in the compute step; events raise it for good. The cards place agents.

**Infiltrate X (locked).** Every event prints one Infiltrate X (two at stage III): **`infiltrate_agents` agents (1;
2 at d4) onto each site of type X** not yet the adversary's, in every country; a line naming research stations,
factories or botnet hexes lands at most `infiltrate_facility_cap` (1) on each, a city line the full number (a botnet
on it or not). A cordoned country takes none; an arrival on player ambassadors cancels one.

**A blocked landing stays in the supply:** a **firewall** or a Unity no-Infiltrate token blocks an agent's landing
on its hex ([README.md](README.md#terms)), and Wall a troop's ([troops.md](troops.md)). A **no-Strike** token stops
strikes on its hex, not landings. On a no-Corrupt token's hex agents land, but nothing corrupts or flips and no node is lost to agents.

**Arrival corruption (the one spread rule).** Whenever the adversary puts agents on a hex (any placement from the
supply: Infiltrate, Escalate, Commandeered Plants, a factory's own agent, a threat's or Discord card's creation, an
uncountered threat's `threat_agents` (1) on its anchor; or a factory's push), **that hex alone is checked**: a city,
research station or factory not yet the adversary's holding its number corrupts, and its agents go to the supply;
else 2 agents on a player's node destroy the node. **Each placement puts all its agents for a hex on, then checks it
once.** The numbers are printed on the map: **city**
`corrupt_at` 5 (from d1 `corrupt_cut` makes it 3), **station** `corrupt_station_at` 3, **factory**
`corrupt_factory_at` 3. A player's Push or Gather is no arrival; nothing else is looked at, so a liberated site
needs its full number of arrivals again. **One aftermath: whatever corrupts takes
one adversary mark.**
- **A corrupted site** takes the adversary's cube (replacing a player's). **It destroys only a node on its own hex**
  (protections apply, [troops.md](troops.md)); player ambassadors there are lost. Sanctuary (capstone): neither corruption
  nor agents destroy your nodes.
- **2 agents on a player's node hex destroy the node** (`corrupt_node_at`; a node rule, not corruption: protections
  apply; checked only when the site doesn't corrupt); its agents go to the supply and **it leaves no mark**.
- **Fallout** is the one scar: a token on free open land (no facility, node or Fallout); nothing is built there, and
  each drips `blight_discord` (1) Discord a round. All four adversity sources print it ([harms.md](../harms.md)): a
  Fallout line's next to the lowest-numbered corrupted city, a threat's on or next to its anchor, the others next to
  their site (ties: lower country, then number); none free, it is lost. Lift Fallout ([economy.md](economy.md)) and
  some cards take one in reach off.

**Botnets spread quickly**. Each country starts with one ([board.md](board.md)); more come from printed lines
(**Botnets spread (level 1+)**, Botnet Swarm; below `botnet_spread_at`, 1, the line does nothing). A botnet stays
until removed and only makes compute (above); it never counts against the win ([alignment.md](alignment.md)). Removed by tier-2 cards, Decommission and
Take down a botnet ([economy.md](economy.md)); removing one sways `alignment.botnet` (0: neither).

## The project deck (candidate)

One shared deck (`content/projects.json`), dealt into the board's **slots** (the
Militarist's: three open) by **the draw rule**: the top card goes into the first empty slot it fits, else to the
discard, which reshuffles when the deck runs out. **When a project leaves**, those to its right slide left and the
rightmost slot refills. **Fewer, bigger, one-shot, every board (locked):** a project prints **one cost a board** (in
play, times the countries), resolves **once**, at once (none is ongoing), and acts **in each country**. Small (one effect) or big (two; a
third from `THIRD_EFFECT_COST`, 18).

- **The compute step.** Once a round: the botnets' compute, the compute income, and what was made since the
  last step (uncountered threats, cards), which **waits beside the projects** (public: the warning).
- **Spending:** all of it at once onto the **leftmost** project (`compute_every` on every one first), one marker
  move a project; one holding its cost **resolves** at once. With no project left, compute is lost.
- **Targets are read off the board as it resolves**, in each country: "the X with the most agents on its hex",
  "the hex holding the most player ambassadors", "its lowest-numbered botnet"; ties: the lowest-numbered. "Turns
  neutral": the owner's cube comes off. **Factory troops** (War Plants): 1 troop on each adversary factory's hex, none under a
  player ambassador. **The clock**: the top event card is discarded.
- **Counterplay:** an ambassador on a source; Cyber's hacks; the Researcher's Hack II, off a project.

<!-- generated: projects -->
| Project | Tags | Cost a board | Size (effects) |
|---|---|---|---|
| Disinformation | diplomacy | 11 | small (1) |
| Backdoor Research | cyber | 14 | small (1) |
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
| Lab Seizure | cyber | 21 | big (2) |
| Slander Campaign | diplomacy | 9 | small (1) |
| Coastal Raids | security | 14 | small (1) |
| Plant Seizure | industry | 13 | small (1) |
| Rumor Mill | diplomacy | 8 | small (1) |
| Sabotage Teams | power | 10 | big (2) |
| Mob Rule | diplomacy | 21 | big (2) |
| Data Heist | cyber | 12 | big (2) |
| Hostile Takeover | industry | 15 | small (1) |
| Station Coup | cyber | 24 | big (3) |
| War Plants | industry | 19 | big (2) |
| Recruitment Drive | diplomacy | 24 | small (1) |
| Propaganda Network | universal | 18 | big (2) |
| Infiltration Drive | diplomacy | 22 | big (2) |
| Accelerated Timetable | universal | 20 | big (2) |
| Grand Offensive | universal | 24 | big (2) |
| Martial Law | security | 24 | small (1) |
<!-- /generated -->

Texts: [adversary-cards.md](../adversary-cards.md).

## The event deck (the clock and the stager)

A game deals `stage_cards` at random per stage (counts below), each stage shuffled on its own, stage I on top: as many
rounds as cards, **the round cap** (**locked** that a cap exists and can shorten; the number a candidate). A card
prints its lines in the Card beat's order (`drafts/adversary/check.py` refuses another).

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
| Card | Stage | Threat (each country) | Infiltrate | Troops | Escalates next round |
|---|---|---|---|---|---|
| Probing Strikes | 1 | tier 1 | coastal cities | — | coastal cities |
| Factory Drive | 1 | tier 1 | coastal cities | — | coastal cities |
| Long Patrols | 1 | tier 1 | cities | — | cities |
| War Drums | 1 | tier 1 | cities | — | cities |
| Signal Jamming | 1 | tier 1 | botnet hexes | — | botnet hexes |
| Mobilization | 2 | tier 2 | coastal cities | Strike | coastal cities |
| Station Raids | 2 | tier 2 | research stations | — | research stations |
| Shell Labs | 2 | tier 2 | research stations | — | research stations |
| Fifth Column | 2 | tier 2 | cities | Strike | cities |
| Border War | 2 | tier 2 | factories | — | factories |
| Supply Lines | 2 | tier 1; tier 2 | factories | — | factories |
| Redeployment | 2 | tier 2 | cities | Redeploy | cities |
| Total Mobilization | 3 | tier 2; tier 2 | factories, cities | Strike | factories, cities |
| Blitzkrieg | 3 | tier 2; tier 2 | cities, research stations | Strike | cities, research stations |
| Occupation | 3 | tier 2; tier 2 | research stations, cities | Strike, Redeploy | research stations, cities |
| Naval Blockade | 3 | tier 2; tier 2 | coastal cities, factories | Strike | coastal cities, factories |
<!-- /generated -->

**For new players:** about one project a round lands; the compute pile and the card ahead (its Infiltrate sites,
a Strike or Redeploy icon) are face up, as is last round's card (what escalates).

## Difficulty (`difficulty`, 0–4; each level keeps the steps below it)

<!-- generated: difficulty -->
| Level | Name | Setup (incomes and threshold a country) | Mid-game rule (new) | Rules active | Tunable |
|---|---|---|---|---|---|
| 0 | **The Militarist** | Compute income 0 a country; Infiltrate 1 a site; Research track: a slot every 2 cubes a country; Deck 5 / 5 / 3 (13 rounds); Sites print their corrupt numbers: city 5, station 3, factory 3 | — | 0 | `compute_income`, `infiltrate_agents`, `research_at`, `corrupt_at` |
| 1 | **Sleeper Cells** | Compute income 3 a country | When an Agent lands on a city, it falls at 3 agents, not 5. | 1 | `botnet_spread_at`, `corrupt_cut`, `corrupt_cut_at` |
| 2 | **War Footing** | Compute income 5 a country | When a threat executes uncountered, the adversary also gains 4 compute. | 2 | `board_difficulty_at` |
| 3 | **Arms Race** | Compute income 10 a country; Deck 5 / 3 / 3 (11 rounds) | — | 2 | `stage2_removed`, `stage1_added` |
| 4 | **War Economy** | Compute income 15 a country; Infiltrate 2 a city (1 a site from a station, factory or botnet line); Deck 3 / 4 / 4 (11 rounds) | — | 2 | `stage1_removed`, `stage2_added`, `stage3_added`, `stage_swap_at` |
<!-- /generated -->

**The ladder (Spirit Island's adversary card).** Levels are named and cumulative. Each adds
**setup**, set once, and **at most one mid-game rule**, fired by an event players already resolve (each replaces
one number, printed whole). Discord tips don't raise the level. Liberate costs the same at every level.
