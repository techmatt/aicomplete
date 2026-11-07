# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, its project deck, its card deck and round cap, and difficulty. Terms and conventions: [README.md](README.md).*

## Each round

The adversary is a **board** (the Militarist's: its project slots, two rules and a difficulty line,
[`content/adversary.json`](../../content/adversary.json)) and an **event deck** (below). Its steps, in the round
([economy.md](economy.md)):

1. **Event** (`adversary.event`): next round's face-up event card becomes this round's (none left: players lose).
2. **Threats** (`threats.deal`): the card's top line deals its pulls ([threats.md](threats.md)).
3. **Build** (`adversary.build`), the rest of the card top to bottom. Its **standing lines**: compute, adversary
   ambassadors (`infiltrate`: each country's clean city with the most gains more) and troops ("in each country, 1 troop
   lands on a factory, else a city"). Then its **twists** (immediate ones resolve now:
   Sway, a Unity pull, node losses, botnet ambassadors; the rest are read where they act). Then the **build**:
   **arrivals (candidate):** the card names a site type, and every matching site in every country gains `arrivals`
   adversary ambassadors, whatever the board holds (below); each corrupted city gains `city_build` (2, Matt: faster
   than the party can easily deal with) and each factory next to a corrupted city `factory_build`; the twists add
   more. **Troops where the Protector stands** (the board's rule): in each country where a seat with Wall has a node,
   `wall_troops` troops land as any landing does (on a factory, players choose, else a city; never a hex Wall holds),
   giving Wall strikes to stop ([war.md](war.md)). A stage-II card's **Escalation** gives each country's lowest-numbered
   factory `escalation_ambassadors` (137: it was the first base; candidate). Ongoing **projects** act (below). Last, each **botnet** makes `botnet_compute`
   **adversary compute**, onto the projects (none under a player ambassador).
4. **Spread** (`adversary.spread`): every hex holding several adversary ambassadors, fixed
   when the step starts, keeps one and sends each other one to a hex within `spread_range`.
   **Why 2:** spread is the adversary's movement, a dial on its pressure, not a player's
   reach, so the reach-1 rule doesn't bind it; Long Patrols' and Blitzkrieg's +1 is their
   whole escalation.
   **Spread never targets an empty hex**: a destination holds a piece or is player-owned
   (node, an aligned city, an owned station or factory); it is not ocean, and it holds no
   adversary ambassador unless it is an uncorrupted city, clean or aligned (which
   collects any number), or a piece without a botnet (below); a hex holding nodes of a seat
   with `spread_shield` (the Sanctuary capstone; no card prints it) is no destination.
   **Priority:** player-owned hexes, then clean cities, then other pieces; then the nearest;
   players choose among equals. An uncorrupted city never spreads: it holds its ambassadors. Each ambassador moves at most once a spread, in hex order. **A surplus with nowhere to go returns to the supply.**
5. **Corrupt** (`adversary.corrupt`): an uncorrupted city holding `corrupt_at` (4)
   adversary ambassadors becomes corrupted (an aligned city loses its player cube); its
   ambassadors return to the supply; player ambassadors waiting there are lost (on any
   corruption; they would free it in the same step).
6. **March** (`board.march`, [war.md](war.md)), then **reveal** (`adversary.reveal`): the top card goes face up as
   next round's card.

**The Militarist's board rules.** *Troops where the Protector stands* (above). *Warmongering:* each round, a
military base with an adversary troop on or next to it makes 1 extra corp troop, even past its cap; a base gains at most
1 extra corp troop a round from every such rule together (Warmongering, Conscription, Arms Race; Matt,
2026-11-06). Its difficulty line, from `board_difficulty_at`: every standing line that lands troops lands 1 more in
each country.

**Botnets.** A botnet is made when a spreading adversary ambassador lands on a piece that
already holds one (not an uncorrupted city, which collects them, nor a piece that already
has a botnet): both ambassadors return to the supply and a botnet goes on the
piece. Each board starts with a botnet (on its research station). A botnet stays until
removed: it makes `botnet_compute` adversary compute a round, it **blocks site claims** (a
site with a botnet on or next to it can't be claimed), and it counts against the purge until
Unity eases it ([alignment.md](alignment.md)). **Botnets are hard to remove (candidate, Matt):** only tier-2
effects do it, no standard action: the tier-2 cards **Botnet Takedown**, **Singularity
Exploit** (also discards a project) and **Decapitation Strike** (also removes 6 adversary ambassadors),
each removing a botnet in your reach; **Root Access**, one on or next to the city it liberates; and the
tier-2 Unity card **Decommission** (any botnet, players choose). Removing a botnet
sways `alignment.botnet` Unity ([alignment.md](alignment.md)).

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
- **Counterplay:** a player ambassador on a botnet stops its compute, not used up;
  Cyber's hacks drain 3-6 compute from one project (Threat Hunt, Logic Bomb, Bug Bounty Board, ...) or
  discard one with its compute (Killswitch Directive, Singularity Exploit); the drain standard project
  ([economy.md](economy.md)) does it for credits. No card drains every project (compute sits on the cheapest).

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

Compute goes on one point at a time: a full project resolves before the next lands; Purge the Unaligned's loss is any one of the player's ambassadors (supply
or a hex) or nodes, their pick, never their last node; a node lost so leaves the game like
a destroyed one ([war.md](war.md)). Ongoing projects act in the round's build step only, not in an
out-of-turn build (Grand Offensive).

## The event deck (the clock and the stager)

[`content/adversary.json`](../../content/adversary.json) prints the Militarist's event cards: 5 stage I, 6 stage II,
4 stage III. A game deals `stage_cards` at random per stage, each stage shuffled on its own, stage I on top, so each
stage deals N of its N+1 printed cards at every difficulty: as many rounds as cards, **the round cap** (**locked** that
a cap exists and can shorten; the number a candidate). **11 rounds through d2:** difficulty 2 swaps a stage-I card for
a stage-III card (`stage1_removed`, `stage3_added`), a harder end, not a shorter game; difficulty 3 removes a stage-II
card (`stage2_removed`): 10. The next round's card is always face up (the `peek` op shows more; no content prints
it). A card prints, top to bottom: its pulls, its standing lines, its twists, its arrivals and (stage II) Escalation.

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
| Card | Stage | Pulls (each country) | Escalation |
|---|---|---|---|
| Probing Strikes | 1 | tier 1 (military) |  |
| Factory Drive | 1 | tier 1 (military or economic) |  |
| Long Patrols | 1 | tier 1 (military or cyber) |  |
| War Drums | 1 | tier 1 (military or influence) |  |
| Signal Jamming | 1 | tier 1 (military or grid) |  |
| Mobilization | 2 | tier 2 (military) | yes |
| Station Raids | 2 | tier 2 (military or cyber) | yes |
| Fifth Column | 2 | tier 2 (influence or cyber) | yes |
| Border War | 2 | tier 2 (military) | yes |
| Supply Lines | 2 | tier 1 (military or economic) | yes |
| Forced March | 2 | tier 2 (any) | yes |
| Total Mobilization | 3 | tier 3 (military); tier 2 (economic or grid) |  |
| Blitzkrieg | 3 | tier 3 (any); tier 1 (military or influence) |  |
| Occupation | 3 | tier 3 (military or cyber); tier 2 (influence) |  |
| Naval Blockade | 3 | tier 3 (economic or grid); tier 1 (military or cyber) |  |
<!-- /generated -->

**Arrivals: the explore step that never stops.** The site types are **coastal cities**,
**industrial zones** (built on or not: an ambassador may stand on an empty zone, the one
exception to "ambassadors sit on pieces"), **research stations** (whoever owns them) and **each country's lowest-numbered factory**
(137: it took the cards that named each country's first base). A coastal city is next to ocean or printed coastal on its
piece (`coast` in `content/map.json`); every piece has two. Horizon gets both
from ocean, Cintamani prints one more, Onyx and Biomia print both. Next round's card is
face up, so its site type shows a round ahead. Each stage's cards spread the types. An
out-of-turn "the adversary builds now" brings the card's arrivals too, in its scope.

**For new players.** Stage II cards send an extra adversary ambassador to each country's lowest-numbered factory,
where it shuts the factory off until someone removes it. **What resisted this:** "lowest-numbered" is a printed
number a player has to look up.

**The supply is a cap.** A placement it can't make doesn't happen; running out doesn't lose.

Players **lose** when a round starts with no card to turn face up (after the round cap's
last round), a quiet backstop.

## Difficulty (`difficulty`, 0–4; each level includes the ones below)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules |  |
| 1 | Extra threats each round in all, countries in turn | `extra_threats` 1, `counter_bonus` 0, `counter_bonus_all` 0 |
| 2 | A stage-I card swapped for a stage-III card: stage II (escalation, tier-2 threats) sooner and a harder end, at the same round cap (the table above) | `stage1_removed` 1, `stage3_added` 1 |
| 3 | Larger builds: each corrupted city gains more, and a stage-II card removed (a shorter round cap) | `build_bonus` 1, `stage2_removed` 1 |
| 4 | A still shorter Discord deck: the removed cards count as tips already made, so every printed step still acts; tier-2 threats sooner; optionally more corrupted cities per country at setup | `discord_removed` 1, `tier2_early` 1, `extra_corrupted` 0 |
<!-- /generated -->

Difficulty 1's extra threats are more pulls of the card's first pull, countries in turn (country 0 in round 1);
difficulty 4's "tier-2 threats sooner" deals a stage-I card's tier-1 pulls from tier 2 once a stage-II card is
`tier2_early` cards ahead (the face-up card first).

Difficulty is only the configured level: Discord tips don't raise it ([alignment.md](alignment.md)). **The
players' claim and liberate costs are the same at every difficulty** (138's Corrupt X will lower only the adversary's): difficulty comes from the
adversary and decks (Discord deck size, threat strength, round cap). The target is in
`docs/principles.md`; reads are indexed in [experiments/history.md](../../experiments/history.md).
