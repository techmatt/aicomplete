# 292: overhaul 1 of 7, the cuts (r139)

Matt (2026-11-19). `RULES_REVISION` 138 → 139 (bumped from the constant). Base: main at 04d89433 (r138, after 291).
The first of seven items rebuilding the adversary and the map side; the later ones replace the event cards, troops,
the win condition and ambassador movement, all left as they are here. Every number is a **candidate**.

## Rulings and amendments

- Locked: the draft passes cards; no card-play limit and no turn budget; resources are not merged; no rule may depend
  on a player's home country except the threat rule.
- Amendment 13357: no tag line on the event cards, but each adversary keeps its lean: the threat decks are built at
  setup with a mix the adversary's board names, and nothing is filtered during a round.
- Amendment 13371: overturned standing rulings are rewritten in place in `docs/rulings.md`.

## What was cut and what replaced it

| Change | Before (r138) | Now (r139) |
|---|---|---|
| War | fronts, a military base each side, drones (armed at a Discord tip by neutral factories, stopped by an ambassador on the base), a War step every round, Warmongering, War Takes Ground, a destroyed factory swaying Discord | gone; a base hex is open land; nothing replaces the Discord from war |
| Threat harm | the player whose node is nearest a token, at any distance; the players pick on a tie | the player of the country it was dealt in (their home country); no distance, no tie |
| Threat dealing | turn up cards until one carries a listed tag and has a site in the country, discarding the misses (2.60 cards rejected a round) | the next card, always; decks built once at setup by the board's **War Rooms** line |
| A dealt threat with no site of its types | skipped the country, or rejected | its token goes on the country's lowest-numbered city, one not the adversary's first; none of its located lines acts there |
| City energy | one host a city, one city a player (two with Twin City Compact), the players choosing the matching; Buy energy for an unhosted city, 4 credits | each city neither corrupted nor blocked (an agent on it, a blackout) pays 1 energy to every player with a node on or next to it |
| Unity | each Unity swayed could move 1 Discord off instead, the acting player choosing; Envoy: an ambassador for 1 Discord off | Unity swayed always goes onto the Unity track; no Envoy |

**The deck's design.** One deck a tier, no tag line, the dealt card the top card. The board prints the mix: War
Rooms, "Setup: build each threat deck from every Security and Universal threat of its tier and as many of its other
threats, taken at random; shuffle it. The rest stay in the box." (content: the board's `threat_deck`, its tag and
`others` a lean card). The Militarist's decks are 12 cards at tier 1 (6 Security + 6 of the other 19) and 24 at tier
2 (7 Security + 5 Universal + 12 of the other 20): half of each deck is its lean and the other half changes from
game to game. Before, 13 of the 16 event cards named Security in a pull, alone or beside one other tag. The typed jam
and counter cards now meet their tag by the deck's mix alone (Security half the deck, each other tag about an
eighth), not by what this round's card names.

## Cards and threats redesigned (old line → new line)

| Card | Old | New |
|---|---|---|
| Twin City Compact (tier 2 program, 4 cr) | Place 2 Nodes. You may host 1 more city from now on. | Place 2 Nodes. Gain 3 energy. |
| Border War (stage II event) | Each player with a Node on or next to a front's base destroys 1 of those Nodes. | Each player with a Node on or next to a factory destroys 1 of those Nodes. |
| Casus Belli (tier 2, base → city) | This base's front has a second war step next round. 2 Fallout land next to here. | Sway 3 toward Discord. 2 Fallout land next to here. |
| Orbital Strike (tier 1) | location: base | factory; text unchanged |
| Commandeer (tier 1) | base, factory | factory |
| Call-Up (tier 1) | base | coastal city |
| Provocation (tier 1) | base | city |
| Total War, Conscription (tier 2) | base | factory |
| Manhunt (tier 2) | base, city: "... 1 adversary Troop lands on the base here." | city: "... 1 adversary Troop lands here." |
| Garrison State (tier 2) | city, base: "... 1 adversary Troop lands on the base here." | city, factory: "... on the factory here." |
| Last Ultimatum (tier 2) | city, base | city |
| Arsenal (tier 2) | factory, base | factory |
| 36 threats | "The nearest player pays / discards / destroys ..." | "This country's player ..." |
| All 16 event cards | "Threat, each country: Security 1 or Cyber 1." | "Threat, each country: a tier-1 threat." (a tier-2 threat; a tier-1 and a tier-2 threat; 2 tier-2 threats) |
| The Militarist's board | Warmongering; War Takes Ground | both cut; **War Rooms** (the deck setup line) added |
| 8 Unity cards (24 tier lines) | "... or city a player hosts (at most N)" | "... or free city with a player's Node on or next to it (at most N)" |

**The anchor check (give / get).** Twin City Compact, on its r118 note's basis (pay 18 at 6 credits, so 16 at
today's 4; get 27, of which the second host slot was 11: 0.55 energy a seat-round over 4.5 rounds at 4.5): the slot
becomes 3 energy once, 13.5, so get 29.5 ÷ pay 16 = **1.84×**, from 1.69× with hosting. (A first cut printed +1
energy production, 20 at the anchors and 2.25×: richer than "similar worth", so it was replaced before the final
read.) Casus Belli: its war step was 8 at `values.json`; 3 Discord at 2.667 is 8, so it stays 32 uncountered, 2.00×
its cheapest counter. Border War: nodes by a factory are more common than nodes by a base (two base hexes a front),
so it bites more seats; factories are where the card infiltrates and grows. The Unity cards count a free city beside
a player's node whether or not an agent or a blackout blocks its energy (r138 counted a hosted one, which a blocked
city never was): a little more generous, and what the card says.

## Before against after (information; nothing chased)

`experiments/farm/read.json` (the judge, the three board pairs, d0-d3), seeds 50316-50327, 12 a cell, 144 games a
side; the before side farmed at 04d89433, the after side at c8711216 (the final rules).
`python scripts/bench/cuts_read.py BEFORE AFTER`.

| pair | games finished | wins | rounds a game | energy income a seat a round | of it from cities | energy gained, every source | Discord tips a game | threat cards rejected a round |
|---|---|---|---|---|---|---|---|---|
| B+P | 48 → 48 | 38 → 48 | 8.92 → 7.06 | 3.39 → 5.09 | 0.97 → 2.62 | 5.28 → 6.51 | 2.21 → 1.83 | 2.52 → 0.00 |
| B+R | 48 → 48 | 41 → 43 | 8.29 → 7.48 | 2.53 → 4.21 | 0.91 → 2.33 | 4.65 → 6.28 | 2.04 → 2.25 | 2.65 → 0.00 |
| P+R | 48 → 48 | 46 → 47 | 7.71 → 6.90 | 3.47 → 5.22 | 0.96 → 2.38 | 5.53 → 7.10 | 1.75 → 1.75 | 2.63 → 0.00 |
| all | 144 → 144 | 125 → 138 | 8.31 → 7.15 | 3.13 → 4.82 | 0.95 → 2.44 | 5.15 → 6.62 | 2.00 → 1.94 | 2.60 → 0.00 |

Energy income is the income step's: cities in production, the board and cards in upkeep (Buy energy, 0.29 a seat a
round before, is in the last energy column only). Rejected a round is r138's `threats.turned_rejected` and, at r139,
cards turned up less cards dealt (17.44 and 17.44 a game). By level, wins of 36: d0 33 → 36, d1 32 → 34, d2 33 → 35,
d3 27 → 33. Rounds: d0 7.56 → 7.44, d1 8.78 → 7.22, d2 8.39 → 6.94, d3 8.50 → 6.97. Losses: 12 round caps, 5 node
losses and 2 Discord-deck losses became 3 round caps and 3 node losses. A dealt threat fell back to a city 0.59
times a game; threats uncountered went 10.4 → 8.1 a game. The game got easier and shorter: more energy, no war
destroying factories, fewer threats left uncountered, and no difficulty retune in this item. Two rule sets differ on
six things at once here, so no single cut is credited with any of it.

**Resource use** (a seat-game, gained → spent; `design_check.py`): energy 42.8 → 34.4 became 47.3 → 30.8 (80% spent
became 65%: energy binds less, and Sprawl is no longer its largest spend); chips 7.3 → 4.4 became 6.7 → 3.9; research
cubes 6.7 → 4.8 became 5.3 → 3.5; ambassadors 20.8 → 17.4 became 19.0 → 14.4 (Liberate 64% → 62% of their use);
antimatter 3.8 → 1.3 became 3.3 → 1.3; credits spent 111% of what a game gains (their start spent too). Insight is
unchanged by this item's rules.

**Design warnings on the after read** (reported, not chased): `friction-stale` (as on main); `rounds` at every level
(6.9-7.4 a game against the 8-10 target; before, d0 alone); `usage`: Protector Levy 0.03, Builder Fortify I 0.24,
Make antimatter 0.01, Sell a card 0.02, credits spent 111%; `kept-not-played` on 5 cards (7 before). The before
read's `victory-skips` (antimatter) and Liberate's 97% usage did not fire.

## Checks

- The player view played a whole game (`web/scripts/player-play.mjs`, three seats, seed 1, to its outcome) with its
  new removed-pieces check at 0: no base, pennant, front line, drone, host chip, Envoy or Buy energy action, War or
  bases phase, and none of their words on the page.
- Speed: `speed.py check` gives no verdict across a revision (its game times: greedy 0.75×, engine 0.82×, planner
  1.01× the r137 baseline). `per_decision.py OLD NEW --games 12 --pairs 3`: greedy 1.6 → 1.6 ms a decision (1.01×),
  the judge 1.7 → 1.8 (1.10×); other games on the same seeds, read beside a light job, so ratios only.
- A first after-read was frozen at 51e45834; its seeds replayed on the next build were identical, 144 of 144
  (`farm/compare.py`). The diff review's fixes then changed play (the Unity cards' count, the fallback, Twin City
  Compact), so the table above is the same seeds on the final rules: 138 wins against that first read's 134.
- `loop_smoke.py --run`: 96 games, no timeout, no card past 1 play a game. `play_read.py`: no card kept in over 1% of
  games and never played. `npm run cards:fit`: 208 cards, none under 7.5 pt.

## Left in though this change makes it pointless

- `board.cities_unowned`, `board.aligned_bonus`, `board.ambassador_discord` and the aligned-city branch of
  `city_energy`: r114's ownership, reached by no baseline.
- `TheftWho::Nearest` (`"who": "nearest"` on the theft threats) and `threats::nearest_nodes`: the names of the old
  rule for what is now "the player it hits".
- `alignment.unity_new` (Unity waiting to be placed): with no choice left it could go straight onto the track.
- The `drone` icon in `web/card/icons.js`.
- Dispatch onto a piece does less: with no bases, an ambassador stands only on sites, nodes and botnets.
- Fortify and the other protections of a factory now answer adversary troops alone (war was the other thing that
  destroyed one); a later overhaul item replaces troops.
