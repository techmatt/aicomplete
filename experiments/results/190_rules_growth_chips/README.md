# 190: factory growth and chips (r83)

Rules revision 82 → 83 (bumped from the constant, as the prompt said). Matt's rulings of 2026-11-11, amendments 8616,
8627 and 8642 (his answers to the staged questions). All candidates.

## What was built

- **City growth (amendment 8642's variant: factory-only, no floor).** A city makes no chip of its own; it gathers 1 a
  round from each unclaimed neutral factory next to it (claimed or adversary factories feed nothing; an agent on the
  city blocks nothing), at most 3. At 3 it spends them on a **neutral** facility of the kind it has fewer of next to
  it (ties: a factory), next to it, else within 2 in its own country, else its chips wait (no switching kind). An
  aligned city's owner picks the hex; a neutral or corrupted city's is the lowest-numbered (a corrupted city's next to
  an agent first). r75's factory a city a round is gone. Params `board.city_build_chips` 3, `board.city_chip_cap` 3.
- **Why factory-only:** Matt's cost "too much manual labor for a human": a city with no unclaimed factory beside it
  touches no token, and claiming a factory both pays its claimer and stops its cities growing, so growth is a prize
  to take. The planner's note ruled out a per-city floor unless factory-only stalls play (it doesn't: below).
- **Setup:** each city starts with a neutral factory and a neutral station next to it (every city, every layout; a
  test). `content/map.json`: Horizon's factories 2, 8, **13**; new `more_stations`: Cintamani **11**, Onyx **3, 5**,
  Biomia **10** (Cintamani's and Biomia's empty station sites became the stations). Onyx is the one 120-degree piece:
  adjacency was checked on each piece's own axes.
- **Chips:** a player's factory makes 1. Every chip price and gain halves, rounded up, and a chip is worth 4 at
  `values.json` (was 2), so each keeps its worth in credits (give / get below).
- **Amendment 8616:** War Takes Ground (the Militarist's fourth rule, `adversary.war_node` 1): a drone that destroys a
  factory destroys a node on its hex too, each piece's protections applying; only the factory sways Discord. **M1:**
  Station Seizure lands 2 agents, then the normal flip check; its 4 compute went (see Decisions). **#57:** every project
  slot is open. **#55:** a corrupted station's one output (1 cube onto an open megaproject, else 1 compute) was already
  the engine's rule since 185: docs restated, no code change.
- **Amendment 8627:** #44 troops ignore ambassadors (175's removal step gone; the owner's ambassador is still armor).
  #56 a node hex always takes land corruption, saved node or not.

## For new players (r83's changes)

Every city starts with a neutral factory and a neutral research station beside it. A neutral factory nobody has
claimed puts a chip on each city next to it every round; when a city holds 3 it builds another neutral factory or
station beside it, whichever it has fewer of. Claim the factory (3 ambassadors) and it pays you a chip a round and
stops feeding the city, so the map grows where nobody acts. Chips are scarcer and worth more: a factory makes 1, and
everything priced in chips costs about half what it did. The adversary has a fourth rule: when a drone destroys a
factory in war, it destroys a node on that hex too. Its troops now walk past ambassadors (your own ambassador still
shields your facility), a node hex that corrupts always leaves land corruption, Station Seizure lands 2 agents and
then flips the station only if they are enough, and all three project slots take any project. **What resisted the
summary:** which kind a city builds counts every facility beside it, anyone's; and a city with nowhere free beside
it looks two hexes out, in its own country only.

## Give / get (at `values.json`, chip 4 since r83; r82's chip was 2)

Each line keeps its worth in credits: halved chips at a doubled chip.

| item | r82 | r83 | give / get at r83 |
|---|---|---|---|
| a player's factory | 2 chips (4) | 1 chip (4) | claim 3 ambassadors (12) for 4 a round: pays back in 3 rounds, as at r82 |
| Build a research station | 2 chips (4) | 1 chip (4) | 4 for a station making 2 cubes a round (6): a gain at once |
| Builder Procure | 2 chips (4): draw 2 | 1 chip (4): draw 2 (8 at the anchor 4 a card) | gain 4 |
| Builder Scrap | 1 Industry: 2 chips (4) | 1 Industry: 1 chip (4) | unchanged worth |
| Builder Mint | 3 credits: 1 chip (2, a loss) | unchanged print: 1 chip (4) | **a gain of 1** (r82 lost 1): flagged, can't halve |
| Builder Full Capacity | 1 chip a round per 2 Industry | per 4 | half the chips at twice the worth: unchanged |
| Onyx's project | completion 4 chips; converter 1 cube → 2 chips | 2 chips; 1 cube → 1 chip | 1 cube (3) → 1 chip (4): gain 1, as r82's 3 → 4 |
| Orbital Mass Driver | cost 15 credits + 2 chips; income 3 chips | 15 + 1 chip; 2 chips | income 8 a round (r82 6): +2 from rounding 3 → 2; flagged |
| Joint Fabrication Pact | 6 chips, another player 3 | 3, another player 2 | 12 for you (r82 12); the gift 8 (r82 6) |
| Vendor Financing's gift | 2 chips | 1 | unchanged worth (4) |
| Arcology Foundry, Peaker Turbines | 2 chips a round | 1 | unchanged worth |
| Wafer Fab Line's Action | 2 energy: draw 2 + 2 chips | draw 2 + 1 chip | unchanged worth (its `overrides` row +10 stands) |
| Antitrust Action | 1 chip per 2 Industry | per 4 | unchanged worth |
| Fabrication Megaplex | 1 chip a round (2) | 1 chip (4) | **+2 a round**: flagged, can't halve |
| Threat chip payments and counters | 3, 4, 5, 6, 7, 8 chips | 2, 2, 3, 3, 4, 4 | worth unchanged but for rounding up (3 → 2 is 8 against 6; 7 → 4 is 16 against 14) |
| Raiding Parties (project) | each player 5 chips or 3 cards | 3 chips or 3 cards | 12 against r82's 10 |

## The map by round (48 seeds 43615-43662, strategist, three pairs, d0-d4: 720 games a side, paired)

Full table: [`map-by-round.md`](map-by-round.md) (`map_read.py`; the `map.*` counters are new in 190, patched into
the r82 tree as stats only, same games). Per game still running at that round:

| | r1 | r2 | r3 | r4 | r5 | r6 | r8 | r10 |
|---|---|---|---|---|---|---|---|---|
| neutral factories r83 / r82 | 4.9 / 3.8 | 4.8 / 5.3 | 4.6 / 4.6 | 5.1 / 4.1 | 4.5 / 3.5 | 3.8 / 3.1 | 3.0 / 2.5 | 2.0 / 1.9 |
| player factories r83 / r82 | 0.04 / 0.04 | 0.03 / 0.36 | 0.05 / 0.33 | 0.06 / 0.32 | 0.10 / 0.31 | 0.13 / 0.35 | 0.28 / 0.62 | 0.42 / 0.91 |
| adversary factories r83 / r82 | 0.4 / 0.5 | 0.9 / 2.2 | 1.5 / 2.6 | 1.9 / 2.9 | 2.2 / 2.9 | 2.3 / 2.9 | 2.1 / 2.1 | 0.7 / 0.6 |
| neutral stations r83 / r82 | 2.4 / 1.7 | 1.7 / 1.0 | 2.4 / 0.8 | 2.8 / 0.6 | 3.1 / 0.5 | 2.7 / 0.5 | 2.2 / 0.5 | 1.6 / 0.4 |
| city builds, factory + station (r83) | 0.0 | 1.1 | 2.2 | 1.0 | 0.3 | 0.3 | 0.2 | 0.2 |
| growth token moves r83 / r82 | 7.0 / 3.7 | 10.1 / 0.1 | 14.0 / 0.4 | 8.1 / 0.3 | 4.7 / 0.3 | 2.9 / 0.4 | 1.6 / 0.3 | 1.2 / 0.4 |
| adversary troops on the map r83 / r82 | 3.1 / 3.1 | 6.5 / 6.3 | 10.9 / 11.7 | 15.2 / 16.3 | 19.7 / 20.3 | 22.7 / 24.6 | 27.2 / 25.2 | 11.1 / 9.5 |

- **Neutral claims: none.** The agents claim no neutral factory at r83 or r82 (0.00 a game), and 0.18 neutral
  stations a game (r82 0.09). Matt's "neutral ones get claimed fast" doesn't hold for the agents, so nothing in the
  sim slows growth by claiming, and starting with 2 factories would only speed it. Whether people claim them is a
  playtest question; for the agents it is a decision to review (the eval's `owned_factory` 4 against 3 ambassadors).
- **No runaway.** In a country nobody touches the loop runs as computed (first builds in round 2-3, about one a city
  every 1-2 rounds) and stops on the map: cities wait 11.3 times a game with chips at their cap and no free site, and
  builds fall to about 0.2 a round from round 5. Neutral factories peak about 5 (2 countries) because each country
  prints 4 industrial zones.
- **Board upkeep** (chips placed, spent and facilities placed, all cities): 7-14 token moves a round in rounds 1-3,
  about 8 in round 4, then 1-5. Early it is far more than r82's (one factory a city in round 1, then little), because
  every city starts beside an unclaimed factory. A city with no unclaimed factory beside it touches nothing.
- **Players own almost no factories** (0.04-0.4 a game against r82's 0.3-0.9): cities no longer build their owner's,
  and the agents don't claim. Factory chips made: 0.84 a game (r82 5.14).
- **Troops** (Matt's answer 5): adversary factories grow from threats and projects only, 0.56 built a game (r82
  1.96); troops built by factories 11.1 a game (15.0); troops on the map about the same through round 7 and higher
  late (27 against 25 by round 8): projects and threats carry them. War Takes Ground took 0.31 nodes a game.

## Ladder (fresh seeds 43615-43758, paired against r82; no dial moved)

Strategist 144 seeds a pair (432 a level), greedy 43615-43662. r83 farmed at d7ecd70 (`farm-seeds`, main box, 524 s
and 145 s), r82 natively from a clean build of d08124f in `out/r82src` with the stats-only `map.*` counters patched in.
Tables: [`ladder-strategist.md`](ladder-strategist.md), [`ladder-greedy.md`](ladder-greedy.md).

| d | r83 strategist | r82 | gap ± SE | band | r83 greedy | r82 greedy |
|---|---|---|---|---|---|---|
| d0 | 89.1 | 89.6 | −0.5 ± 2.0 | 90-100 **unmet by 0.9** | 92.4 | 84.7 |
| d1 | 74.8 | 77.3 | −2.5 ± 2.5 | 70-80 met | 75.7 | 83.3 |
| d2 | 57.2 | 56.9 | +0.2 ± 2.4 | 55-65 met | 60.4 | 54.9 |
| d3 | 52.1 | 52.1 | +0.0 ± 2.6 | 40-50 **unmet by 2.1** | 51.4 | 47.9 |
| d4 | 44.2 | 47.5 | −3.2 ± 2.7 | - | 46.5 | 44.4 |

Every gap is inside 1.3 SE: r83 plays as hard as r82. Both misses are r82's too on these seeds (89.6, 52.1). All
rulings changed at once, so no gap is attributed to one. Adversary measures: corruptions up about 2 a game at d1-d4
(16.8 against 14.8 at d2: more neutral sites to corrupt), corruption node losses up about 0.9.

## Resource use (strategist, the ladder's 2,160 games a side; [`resource-use.md`](resource-use.md))

| resource | spent r83 | r82 | ran out r83 | r82 |
|---|---|---|---|---|
| **chips** | **2.0** | 4.2 | **92%** | 66% |
| credits | 229 | 231 | 28% | 28% |
| insight | 38.7 | 38.8 | 5% | 4% |
| energy | 24.7 | 24.9 | 43% | 44% |
| research cubes | 3.0 | 3.4 | 88% | 88% |
| ambassadors | 17.3 | 17.8 | 46% | 45% |

Chips: half as many spent, at twice the worth (8.0 against 8.4 credits a seat a game), so the chip economy holds in
value; but seats sit at 0 chips 92% of rounds. Stations bought with chips 0.8 a seat (r82 1.3); Procure used more
(3.5 a Builder game at d0 bp against 2.6) at its 1-chip price. Chips stop a card play 0.02 times a seat a game, as at
r82: only Orbital Mass Driver costs chips. **Cards after the edit** (plays a game): Joint Fabrication Pact 0.35 (r82
0.70), Antitrust Action 0.07 (0.12), Vendor Financing 0.01 (0.03); Fabrication Megaplex 0.28 (0.27), Wafer Fab Line
and Orbital Mass Driver 0.02 (unchanged). Arcology Foundry is kept and never played at both revisions (r82 0.001).

## Speed (quiet box, `per_decision.py` r82 → r83, 16 games, seed 1000, d2)

Strategist 0.94× a decision (0.91× the options). Greedy **1.36×** a decision on 1.16× the options and 1.08× the
decisions. Growth itself is about 1.03× of that (r83 with growth on against `board.city_build_chips=10`, which never
builds: 1.87 against 1.81 ms a decision, same seeds); the rest comes with the larger starting board (three more
neutral stations and a factory a game: more sites to dispatch to, claim and value). Kept: the board is the ruling.
`speed.py check --update` set the baseline at r83.

## Number check (before any sim)

With factory-only growth a city with one unclaimed neutral factory beside it gathers 1 a round and builds on its
third round: **a build every ~3 rounds a city early**, Matt's target. Each city starts with 1 factory and 1 station
next to it, a tie, so its first build is a factory: 2 unclaimed then, so the next build comes 1.5 rounds later, then a
station (2 factories, 1 station), and so on. **The loop** is bounded twice: by the cap (one build a city a round at
most) and by the map (each country prints 4 industrial zones; factories go only there). **Claims slow it**: each
claim takes a factory out of its cities' count; a city with every neighbor factory claimed stops for good.
