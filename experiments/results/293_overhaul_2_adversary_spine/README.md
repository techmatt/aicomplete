# 293: overhaul 2 of 7, the adversary spine (r139 → r140)

Matt's rulings of 2026-11-19, amendment #13416 (Grow is open to be reshaped) and his answer #13421 (a slot's icon
happens once, as the marker arrives). The rules as built: [docs/rules/adversary.md](../../../docs/rules/adversary.md).
Everything here is a candidate for his review.

## The adversary's whole turn, as a new player is told it

1. **Card.** Flip the event card and deal its threats. Each site of *last round's* type that still holds an agent
   takes 1 more (escalate); the Militarist then adds 1 to each factory holding an agent (its one rule); then every
   site of *this* card's type takes 1 (Infiltrate). A site holding its printed number corrupts.
2. **March.** Troops move and strike, as before.
3. **Income.** Each corrupted site makes one thing: a city 1 Discord, a research station 1 research cube, a factory
   1 agent (it walks down its zone's arrow to its city next round), a botnet 1 compute. Compute fills the leftmost
   project. Every cube a country on the research track moves its marker one slot, and that slot's icon happens
   once; slot 7 loses the game.

## The icon vocabulary

Each icon is one line an event card already printed at r139, so a row adds no new verb to the table. All ten are
content (`board.icons` in `drafts/adversary/militarist.json`); a row names six of them.

| Icon | What it does | Why it is in |
|---|---|---|
| New factory | Each country: a neutral factory opens on its lowest-numbered free industrial zone | Grow's heir: the adversary's growth is now a slot, not a line on every card; a claim target for players too |
| New lab | Each country: a neutral research station opens on its lowest-numbered empty station site | the station twin (only pieces printing an empty site take one) |
| Cells: cities / factories / research stations | Infiltrate: that type | an extra Infiltrate, which also sets up next round's escalate where the card's type matches |
| Funding | +1 compute a country a round | the project engine speeds up |
| Botnet | Botnets spread (level 1+) | a second compute source; nothing at level 0, where the line is off |
| Fallout | Each country: a Fallout next to its lowest-numbered corrupted city | the land scar, as on all four adversity sources |
| Dread | Sway 3 toward Discord | the political clock |
| Blackout | Until the end of next round, cities make no energy and no Discord | an economy hit for a late slot |

## The Militarist

- **Row, War Research:** 1 New factory · 2 Cells: factories · 3 Funding · 4 Fallout · 5 Cells: cities · 6 Dread ·
  7 loss. It opens a factory, then goes after factories: its factories are what feed agents into cities.
- **Event mix:** War Rooms, 292's Security-heavy threat decks, unchanged; its 16 event cards keep their Infiltrate
  types and their other lines (below, *What resisted*).
- **One rule, Commandeered Plants:** every round, after last round's sites escalate, each factory not yet the
  adversary's that holds an agent takes 1 more. An infiltrated factory falls in two rounds unless its agent is
  cleared, then feeds its city an agent a round.
- **K 1 a country, no starting cubes** (the screen below).

**A contrasting board, on paper only: the Technocrat.** Row, *Black Budget*: 1 New lab · 2 Cells: research stations ·
3 Funding · 4 Botnet · 5 Blackout · 6 Funding · 7 loss: it grows the stations that feed its own track, so its clock
accelerates unless players stand on its labs. Event mix: Cyber-heavy threat decks. One rule, *Open Ports*: every
round, each research station not yet the adversary's that holds an agent takes 1 more. Where the Militarist floods
cities through factories, this one races its track.

## How the map shows the growing threat

- **Last round's card** stays face up in the board's last-round space: its site type is what escalates.
- **Agents on sites** are the count: a site of that type holding an agent takes 1 more. The view rings each such
  site with a "+1" mark (firewalled, no-Infiltrate and cordoned sites are left out, as the rule leaves them).
- **Zone arrows** are printed on the map, one per industrial zone, to its nearest city of its own country (ties: the
  lower number), so each is one hex long and the same in every layout:

| Piece | Zone → city |
|---|---|
| Horizon | 2→6, 8→7, 11→7, 13→14 |
| Cintamani | 3→7, 9→10, 14→10, 16→15 |
| Onyx | 1→2, 6→2, 9→10, 15→10 |
| Biomia | 3→6, 7→6, 8→11, 13→14 |

- **The research marker** sits on its row on the adversary board, the cubes toward the next slot beside it.

## What Grow became (amendment #13416)

The Grow line left all 12 event cards that printed it. A new neutral facility now comes from a site-opening line
(Factory Drive, Shell Labs, as before) or the research row's New factory slot. No "most agents" pick is left on the
event card; four projects keep one (Night Raid, Asset Seizure, Sabotage Teams, Station Coup: not named by the ruling).

## Adaptations (each a candidate)

- **Escalate runs before Infiltrate**, so clearing a site's agents stops its escalate and a card's own Infiltrate is
  never doubled the same round.
- **A factory keeps at most one agent.** It pushes its agent to its city unless that city is already the
  adversary's (or firewalled, under a no-Infiltrate token, cordoned); it makes a new one only when it holds none. A
  factory whose city has fallen keeps a single guard: without this the pile on a corrupted city only grew, and a site
  holding an agent can't be liberated.
- **All four outputs sit in the Income beat.** A corrupted city's Discord moved there from production.
- **Research-cube lines rescaled to K 1:** Wiper Malware and Data Breach 2 → 1 cube, Zero-Day 4 → 2, Lab Seizure and
  Station Coup 2 → 1 a country, Backdoor Research "4 cubes on each megaproject + 2 a country" → 1 a country. At K 1 a
  cube a country is a whole slot.
- **A slot's advance is not capped at one a round:** the track advances while it holds K cubes.
- **Level 3, Arms Race, has no named rule** (it was "Weapons Program musters 3"); the ladder is otherwise untouched.

## Content, old → new

| Card | r139 | r140 |
|---|---|---|
| 12 event cards | "... Spill (level 2+): ... Grow: X." | both lines cut |
| Lab Seizure, Plant Seizure, Grand Offensive, Offensive I-III (Discord) | Surge 2: X | create 2 Agents on its lowest-numbered X not yet the adversary's |
| Coastal Raids, Mob Rule, Hostile Takeover, Martial Law | Surge 3: X | create 3 Agents on its lowest-numbered X ... |
| Infiltration Drive / Recruitment Drive | Surge 4 / Surge 5: cities | create 4 / 5 Agents on its lowest-numbered city ... |
| Backdoor Research | 4 adversary cubes onto each open megaproject; 2 cubes a country onto the track | 1 cube a country onto the track |
| Lab Seizure, Station Coup | 2 cubes a country onto the track | 1 cube a country |
| Wiper Malware, Data Breach / Zero-Day | 2 / 4 cubes onto the track | 1 / 2 cubes |
| The four megaprojects | "each adversary cube 1 Sway to Discord" (Polar Compute Array: 1 compute); "Afterward an adversary station here puts its cubes onto the adversary's research track" | both sentences cut |
| Militarist board | Botnet Recruiters; Weapons Program (Muster 2, 3 at d3) | Commandeered Plants; the War Research row |

A first pass reworded Surge as Infiltrate + Escalate lines; the diff review showed that changed worth (Recruitment
Drive would take every unguarded city from level 1; Martial Law no longer took a station), so Surge keeps its agents
and type and only its pick changed, to the creation line's lowest-numbered site. Ops removed: `surge`, `spill`,
`grow`, `adversary_cubes`, `botnet_cubes` (the last was a palette op, printed on no card). Op added: `escalate` (the
board rule's line).

## What resisted a clean rule

1. **K "about 3 a country" can't end a game.** With one output a station and one corrupted station a country at
   setup, the judge's games make about 9 cubes (max 24) at two countries; K 3 needs 42 for slot 7. Only K 1 (14
   cubes) reaches it. The alternatives are in *For Matt* in the report.
2. **"An event card is Infiltrate X plus escalate"** is true of 4 of the 16 cards. The other 12 keep up to two other
   lines (Muster, Botnets spread, +1 compute, site opening, Discord, Fallout, Blackout, Border War's node loss,
   Forced March, Unity back): troop lines are item 3's, and the rest are content item 5 redesigns.
3. **Factories no longer make troops**, by the ruling, while troop rules stay "working": troops now come only from
   stage-III Musters, threats and two projects (War Plants, Botnet Conscripts), so the march is rarer until item 3.
4. **The round-1 slot.** At K 1, two unblocked starting stations reach slot 1 in round 1's Income beat, so most
   games open a neutral factory in each country at once.
5. **Stage-III cards name two types**, so two types escalate the round after.

## The smoke read: before and after on the same seeds

The judge (`strategist`), the two-player baseline (d0), the three board pairs, fresh seeds 50328-50375 (48 a pair,
144 games a side), `experiments/farm/read.json`; r139 farmed at 4126a21b9a, r140 at 17712794. "Slot reached" on the
r139 rows is how often its research rule fired (a different rule: Weapons Program at 5 cubes a country).

| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max | Supply low |
|---|---|---|---|---|---|---|---|---|---|
| r139 bp | 48 | 47 (98%) | 7.6 (4-14) | deck_empty 1 | 0:47 1:1 | 18.0 / 58 | 13.5 / 32 | 4.0 / 19 | 33 |
| r139 br | 48 | 46 (96%) | 7.5 (4-14) | deck_empty 2 | 0:44 1:4 | 16.7 / 53 | 14.5 / 30 | 4.5 / 19 | 25 |
| r139 pr | 48 | 48 (100%) | 7.2 (3-12) | - | 0:45 1:3 | 15.9 / 40 | 13.6 / 29 | 4.1 / 14 | 32 |
| r139 all | 144 | 141 (98%) | 7.4 (3-14) | deck_empty 3 | 0:136 1:8 | 16.9 / 58 | 13.9 / 32 | 4.2 / 19 | 25 |
| r140 bp | 48 | 44 (92%) | 7.1 (4-11) | research 4 | 1:3 2:9 3:7 4:10 5:8 6:7 7:4 | 20.8 / 50 | 18.0 / 42 | 9.0 / 16 | 30 |
| r140 br | 48 | 41 (85%) | 7.2 (4-14) | deck_empty 1, research 6 | 2:15 3:8 4:8 5:9 6:2 7:6 | 20.0 / 40 | 19.5 / 49 | 8.7 / 16 | 31 |
| r140 pr | 48 | 44 (92%) | 6.8 (4-14) | deck_empty 2, research 2 | 1:3 2:7 3:21 4:6 5:5 6:4 7:2 | 18.1 / 44 | 15.8 / 36 | 8.0 / 15 | 31 |
| r140 all | 144 | 129 (90%) | 7.0 (4-14) | deck_empty 3, research 12 | 1:6 2:31 3:36 4:24 5:22 6:13 7:12 | 19.7 / 50 | 17.8 / 49 | 8.5 / 16 | 30 |

- **Games finish, rounds are sane:** 7.4 → 7.0 a game; every game ended by its rules (no timeout).
- **No pile runs away:** the peak of agents on the board is 19.7 a game (16.9 before) and its worst game lower
  (50 against 58); the supply never ran under 30 of 48; Discord swayed rises 13.9 → 17.8 (corrupted cities' Discord
  5.97 → 10.85 a game: more cities fall, 0.86 → 1.65); research cubes are spent as they come (8.5 made a game).
- **The research row ends games sometimes, not usually:** 12 of 144 (8%), in rounds 6-11; every game reaches slot 1
  and the median game slot 3-4.
- **How each loss happened:** r139: 3 by the round cap (`deck_empty`). r140: 12 by slot 7 (`research`), 3 by the
  round cap. Paired, 127 seeds won on both sides, 14 only before, 2 only after.
- **What moved underneath** (a game, d0): factories corrupted 0.53 → 3.26 (Commandeered Plants), nodes lost to two
  agents 1.80 → 3.27, troop strikes 1.81 → 1.17 (factories make no troops), agents from factories 7.9 made and 2.0
  pushed onto a city, escalate 6.0 from the cards and 7.2 from the board rule, stations blocked by an ambassador 4.1
  times, projects resolved 0.53 → 0.55.

**The harder levels, for information only (no difficulty work in this item).** The same read's d1-d3 cells: wins
94 → 74%, 91 → 64%, 79 → 50%, with slot 7 ending 34, 49 and 68 of 144 games. More stations fall at the harder levels,
so K 1 is a much faster clock there; `research_at` is a list by level, ready for item 6.

## The K screen (dev seeds 36800-36831, d0, 96 games an arm)

`experiments/farm/spine293.json` (the judge) and `spine293g.json` (greedy); `k_screen.py` prints the table. Run
before the review's content fixes (Surge then read as Infiltrate + Escalate, cube lines at their r139 sizes), so
the fresh read above is the confirmation on the final build.

| Arm | Wins (bp / br / pr) | Rounds | Losses | Slot reached (slot:games) | Cubes made: mean / max |
|---|---|---|---|---|---|
| K 3, start 0 (the asked number) | 90 (32 / 29 / 29) | 7.7 | deck_empty 6 | 0:33 1:49 2:11 3:3 | 9.2 / 22 |
| K 4 | 93 (32 / 30 / 31) | 7.5 | deck_empty 3 | 0:59 1:33 2:4 | 9.2 / 23 |
| K 5 | 92 (32 / 30 / 30) | 7.7 | deck_empty 4 | 0:75 1:20 2:1 | 9.3 / 24 |
| K 3, start 1 | 95 (32 / 32 / 31) | 7.3 | deck_empty 1 | 0:18 1:56 2:19 3:3 | 9.0 / 19 |
| K 2 | 94 (32 / 31 / 31) | 7.1 | deck_empty 2 | 0:15 1:39 2:26 3:11 4:5 | 9.0 / 20 |
| **K 1 (shipped)** | 77 (27 / 25 / 25) | 7.0 | research 19 | 1:3 2:13 3:17 4:16 5:14 6:14 7:19 | 9.8 / 17 |
| K 2, start 1 | 95 (32 / 32 / 31) | 7.2 | deck_empty 1 | 1:29 2:43 3:14 4:8 5:1 6:1 | 9.2 / 24 |
| greedy, K 3 | 92 (32 / 30 / 30) | 7.4 | deck_empty 4 | 0:23 1:60 2:10 3:3 | 9.5 / 28 |
| greedy, K 2 | 91 (31 / 31 / 29) | 7.5 | deck_empty 5 | 0:11 1:37 2:29 3:13 4:5 6:1 | 9.8 / 26 |
| greedy, K 1 | 79 (26 / 25 / 28) | 7.1 | research 17 | 1:4 2:8 3:17 4:17 5:11 6:22 7:17 | 10.2 / 16 |

Cube flow is about 9 a game whatever K is: the two starting stations make 2 a round when nobody stands on them.
Slot 7 needs 7 × K × 2 cubes, so only K 1 (14) is in reach; K 2 (28) and K 3 (42) never ended a game. The fresh
read's 8% is lower than the screen's 20% mostly because the cube lines were then rescaled to K 1.

## Checks

- **Diff review** (Ultracode workflow, four lenses, each finding given to a skeptic): 37 found, 32 confirmed. Fixed:
  the view's escalate marks and zone arrows now follow the rule's blockers (firewall, no-Infiltrate, cordon, an
  occupying troop) and a round with no event phase; a board rule printing a line nothing reads is refused at load;
  K 1 made the r139-sized cube lines jump 2-3 slots (rescaled); the Infiltrate + Escalate rewording of Surge changed
  worth (a creation line instead); stale text in phase docs, the Blackout hover, the harms ledger, value notes and
  agent comments; the harms ledger reads the row's icons; `design_check.json` no longer counts factory agents
  twice; `ladder.py` reads the new cancel counter; seven tests added (the beats through the real phases, the block
  cases, every piece's arrows, the view's marks). Left for later items: the agents' forecast does not see Escalate,
  the board rule or a factory's push (item 6 reteaches the judge); Arms Race keeps its name with no rule.
- **Player view:** `web/scripts/player-play.mjs --config three_seat.json --seed 1` plays to its outcome with its
  per-step checks (the row's slots and cubes against the engine, zone arrows drawn, "+1" marks equal to the view's,
  the last-round box) and no Spill, Surge, Grow or seeding on screen; `e2e.mjs`, `e2e-corps.mjs`,
  `e2e-researcher.mjs`, `e2e-sources.mjs` pass.
- **Speed** (`bench/per_decision.py`, r139 against r140, 16 games from seed 1000, d2, one thread, ratios): the judge
  0.79 × a decision (2.1 → 1.6 ms), greedy 0.86 ×, engine 0.94 ×. `speed.py check` gives no verdict across the
  revision (its engine row reads 1.58 × a game on other games); the baseline is still r139's, not re-based here
  because another item was running beside this one.
- **`loop_smoke.py --run`:** no timeouts, no card at the play flag. `play_read.py` and `cards:fit` cover player
  cards, which this item did not change.
- **Frozen runs:** the K screen was read before the review's fixes and is not replayed (its arms are spec overrides
  of a superseded build); the fresh read was farmed on the final commit, after the last rules change.
- **Design warnings** on the fresh read against r139's: new or moved: `dials-hide-design` (`research_at` 5 → 1 in the
  same change as content: this item's own K), `rounds` at every level (7.0, 7.4, 7.0, 6.6 against the 8-10 band; r139
  warned at d0 and d3), `table-components-rise` d0 +12% (23.0 a round, budget 24), `table-checks-rise` at every level
  (d0 10.6 → 14.8 per-site checks a round, +40%; budget 49): the new turn reads more sites than the old one (each
  escalating site, each factory holding an agent, each corrupted factory twice), though each read is simpler.
  Unchanged from main: `friction-stale`, `kept-not-played`, `usage` (more rows at the harder levels' shorter games).

