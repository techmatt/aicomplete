# 207: expected use, and a warning when play strays from it (r91, no rules change)

Every player action's use, a band by role for each, and `design_check.py`'s `usage` warning when play leaves it,
in both directions. Script and config only: no rules, content or agent change, no `RULES_REVISION` bump. Every band
is a candidate for Matt, in `scripts/design_check.json`'s `usage` (roles, then one row per action).

## Data

- **Bands set on** a read already on disk: r91 main as 206 left it, the strategist. It is 206's fresh read
  `f206-s` (45631-45726) at d0, d2 and d4, plus its confirm read `c206-s` (45727-45822) at d1 and d3, the two levels
  whose dials 206 moved after its fresh read. That makes 96 seeds per pair at every level, 1,440 games
  ([design-bandset.txt](design-bandset.txt)).
- **Judged on a fresh read**: the strategist on current main (e6accb8, r91, content hash 141e66c5), seeds
  45823-45918, the three pairs, d0-d4, 1,440 games, farmed (`experiments/farm/f207-strategist.json`, 193 s).
  - [inventory.md](inventory.md): `design_check.py READ --usage`.
  - [design-fresh.txt](design-fresh.txt): the warnings.
  - Ledger: next free 45919.
- **The tokens check did not run**: no earlier revision's read exists on these seeds, and this item changes no
  play, so it has no old side to compare.
- **Measures.** Each action is measured over the seat-games that could take it (a power over its board's seats).
  - `seat_round`: uses a seat-round, which for a once-a-round power is its share of rounds.
  - `seat_game`: uses a seat-game.
  - `games`: the share of seat-games with at least one use.
  - `spent`: a resource's spent over gained.
  
  Bands are judged pooled over levels and boards. The board and level splits in the inventory are information.

## Roles and bands (candidates)

| role | measure | band | why | actions |
|---|---|---|---|---|
| node-power | seat_round | 0.3-0.9 | the board's node route, once a round: most rounds, not every round | Buy In, Accord, Peer Review |
| board-engine | seat_round | 0.4-0.9 | the board's signature "each round you may": most rounds, not all | Mint, Procure, Bulwark, Synthesis, Grant |
| sacrifice | seat_game | 0.25-2.0 | a tag sacrifice of cards you are done with: a use every game or two, never a round staple | Scrap, Overcharge, Rush Build, Stand Down, Recovery, Debrief, Field Station, Audit, Particle Run |
| troop-response | seat_game | 0.25-2.5 | answers troops when they come | Fortify, Patrol, Interdict, Disrupt |
| hedge | seat_game | 0.05-0.5 | a luck hedge or desperate source, poorly priced by design: rare, not zero | Make antimatter, Remove a troop, Gain an ambassador, Sell, Shelter, Counter a threat whole |
| win-route | games | 50-95% | a Unity route the win runs through: most games, not all (another route must exist) | Liberate, Breakthrough |
| sway-tool | seat_game | 0.1-2.0 | a secondary Sway lever: used, never the route | Claim, Envoy, Take down a botnet |
| staple | seat_round | 0.1-1.2 | a standard economy action: some rounds, never crowding the round | Place a node, Dispatch, Buy energy, cube → insight, Remove agents, Build a station |
| cards / building | seat_round | 1-4 / 0.2-1.5 | cards are the game; a building's Action is the engine running | Play a card, a building's Action |
| converter | seat_game | 0.3-3.0 | a completed megaproject's once-a-round converter | Tidal Grid and Polar Compute Array, both sides |
| threat / correction | seat_game | 0.5-5 / 0-0.5 | threats are answered; a withdrawal can be zero | Pay a counter; Withdraw |
| resource | spent | 50-110% | a resource gained is spent (engines over dials) | credits, energy, chips, antimatter, cubes, ambassadors |
| off | - | - | off at r91's tunables, or granted by no card | Clear, Project: harden, Project: drain, Build a factory, Harden for chips |

## Outside the band on the fresh read (6; the same 6 on the band-setting read)

| action | role | use (fresh; band-setting read) | band | fix idea (none built) |
|---|---|---|---|---|
| Researcher Peer Review | node-power | 0.16 a round (0.19) | 0.3-0.9 | 0 in rounds 1-3 (no cubes until a megaproject completes), then 0.28-0.39 in rounds 4-6 and falling. 3 cubes a node is dear, so the Researcher places its nodes with the standard Place a node instead: 1.01 a round, against the Builder's 0.16. Make it 2 cubes, or let an early round pay in credits |
| Builder Mint | board-engine | 0.29 a round (0.29) | 0.4-0.9 | 3 credits for a chip is cheap at the anchors (chips 6), but close to par at the agents' `values.json` (4), and Scrap and Procure's draws compete for the round. Before a price change, check whether the agent's chip value is what holds it back |
| Researcher Grant | board-engine | 0.24 a round (0.24) | 0.4-0.9 | 0.79, 0.58 and 0.33 in rounds 1-3, then 0.04, then 0: it needs an open megaproject, and both are complete by round 4. Give it a use after completion (a cube on a completed project for its converter), or call it an opening power with a band of its own |
| Remove a troop | hedge | 0.55 a seat-game (0.51) | 0.05-0.5 | just over, all from the Protector and Researcher (0.77, 0.79; the Builder 0.08) at d1-d4: troops press harder than the hedge's price assumes. Watch it; 11 credits if it stays over |
| Sell a card | hedge | 0 in 1,440 games (0) | 0.05-0.5 | 1 credit for a card (5 at the anchors) is never right. 2-3 credits, or accept it as unused by design (band 0-0.5) |
| Tidal Grid chip converter | converter | 0 in 1,440 games (0) | 0.3-3.0 | 2 chips for 3 energy (12 → 13.5 at the anchors) loses to every other use of chips. Polar's chip side is used 0.67 a seat-game. Make it 4 energy, or 1 chip for 2 |

Inside, but worth a look in the inventory:
- **The Researcher never turns a cube into insight and never makes antimatter** (0.00 on both). Its cubes go to Peer
  Review (3.4 a seat-game) and the converters.
- **Liberate is in 87% of seat-games**, near the 95% roof, and **Breakthrough in 80%**; at d4, 75% and 64%.
- **Credits are spent 108% of gained**: the start stock is spent down, inside the band.
- **Every resource is spent at least half of what is gained**: antimatter, the lowest, 58%.

## The warning

`design: usage <action> <use> outside <lo>-<hi>`, as 205's header line groups it (a check with over three warnings
gives its count and its first three). Each line says unused or dominant.
- **`power-unused` is gone**: every power has a usage row, and its 0.1 a seat-game threshold is retired.
- **`gained-not-spent` now skips a resource** that has a `spend:` row.
- **`kept-not-played` stays**: cards are not in this inventory one by one; card plays are one aggregate row.
- **A taken action with no row** is listed under Information as "no band".
- **A row no seat could take** is listed under Information too, which catches a stale row after a power is renamed.

`docs/principles.md`, *Design warnings*, has the new rule; `docs/scripts.md` has `--usage`.
