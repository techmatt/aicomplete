# 299: shorter games (r144 → r145)

Matt's rulings of 2026-11-20: most games Spirit Island length, about **6-8 rounds**, 10-11 an exceptional game, a
spread rather than one length (replacing 7-10); early troops are allowed, Martial Law stays at 36. His answer to
question 1 (message 13762): running out the deck is a fair loss route, the cap is the lever (about round 10-11), cap
losses are fine at any level, and K and the ladder are not distorted to avoid them. Rules revision 144 to 145, bumped
from the constant. Numbers only, no new mechanic.

- Tables: [fresh.md](fresh.md) (the round histograms per level and seat count, r144 against r145, the bars marked;
  the alternative build beside; the adversary's spread), [screens.md](screens.md) (seven screens and two three-seat
  ones). Readers: `hist.py` (histogram and bars), `spread.py` (cities corrupted a game), `make_tables.sh`.
- Bars (judge, fresh seeds, d0-d3): over half of games in rounds 6-8; under ~10% at 10+; win rates in band; round-2/3
  wins rare. A cap game records the deck's cards + 1 rounds (it ends in that round's Card beat).

## What changed (r145, shipped as 036a1e2e; its play is 878be62d's, read fresh)

| Number | r144 | r145 | Why |
|---|---|---|---|
| `board.liberate_at` (Liberate a city) | 5 | **3** | the pace lever: about a round and a half off every game at every level |
| `adversary.stage_cards` (the event deck) | 5 / 5 / 3 | **4 / 4 / 2** | the round cap at 10 (Matt: the cap is the lever, about round 10-11) |
| `adversary.stage2_removed` (level 3+) | 2 | **1** | keeps level 3's one-card step: cap 9 at d3-d4 (were 11) |

## The fresh read (two seats: seeds 50784-50879, 288 games a level; three seats: 50880-50927, 48 a level)

| Two seats | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| Wins, r144 → r145 (band) | 95 → **94%** (≥90) | 91 → **89%** (80-95) | 84 → **85%** (75-90) | 63 → **67%** (55-70) |
| Mean rounds | 8.4 → 6.4 | 8.6 → 7.1 | 8.6 → 7.2 | 8.5 → 7.4 |
| In rounds 6-8 (bar: over 50%) | 48 → **51%** met | 49 → **48%** missed | 44 → **55%** met | 45 → **53%** met |
| At 10+ (bar: under ~10%) | 31 → **10%** missed (on the line) | 32 → **16%** missed | 32 → **15%** missed | 30 → **15%** missed |
| Cap losses of losses | 14 of 14 → 17 of 17 | 21 of 26 → 31 of 31 | 12 of 47 → 28 of 44 | 37 of 106 → 63 of 94 |
| Won by round 3 (bar: rare) | 3 → **16** of 288 | 1 → 7 | 2 → 6 | 1 → 3 |

| Three seats (48 a level) | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| Wins, r144 → r145 (all four miss their band at both) | 88 → 73% | 65 → 58% | 62 → 58% | 48 → 46% |
| Mean rounds | 9.9 → 8.6 | 11.4 → 8.9 | 10.9 → 8.7 | 10.2 → 8.1 |
| In 6-8 / at 10+ | 31 → 35% / 62 → 40% | 12 → 31% / 79 → 48% | 23 → 35% / 69 → 40% | 15 → 29% / 69 → 31% |

**Two seats:** every level's win rate stays in band; mean length drops 1.1-2.0 rounds; the 6-8 bar is met at d0, d2
and d3 and missed at d1 by 2 points (about 1 SE); the 10+ share halves but stays 10-16% (missed); round-3 wins rise
from rare (0.3-1%) to 1-6% (missed at d0-d2). **Three seats** shorten by 1.3-2.5 rounds, but most of what is left at
10+ is cap losses, and wins fall at d0 (15 points, further below the two-seat band) and d1-d2 (4-7 points; 48 games a level, SE about 7).

**Why the bars don't meet together.** A level's length has a spread of about 2 rounds (SD) in every arm tried, so a
three-round window holds about half the games wherever the mean sits: shortening moves the whole distribution, and the
left tail (round-3 wins) grows as the right (10+) shrinks. The 10+ share has a floor: at a 10-card deck every cap loss
is a 10+ game, and cap losses are most of a level's losses (d1: all 31), so at d1-d2 the 10+ share is at least the
level's loss rate (5-25% by its band) plus its wins at round 10. Meeting 10+ under 10% at d1-d3 with their bands
needs losses that come before round 10 (research or Discord), which Matt ruled out chasing through K or the ladder.
Long games are the ones where the adversary corrupts more cities mid-game (won games of 11+ rounds: 2.7 new
corruptions against 1.3 in games of 6 or fewer, r144).

## Levers screened (two seats, the judge; [screens.md](screens.md))

Liberate 4 and 3; two, one and no starting ambassadors; decks of 11 (5/4/2) and 10 (4/4/2) cards; a station at 3; a
freed city swaying 3 Unity; Unity tipping at 2 a player; cities corrupting at 6 agents (4 from d1); supply
ambassadors arriving spent (`arrive_ready` 0, r142's to-try), alone and with Liberate 2. Results, on 298's spent seeds
(288 a level): Liberate is the one lever that moves the mean (5 → 4 about a round, → 3 another half); the deck turns
the tail into cap losses; starting ambassadors, the station price, Unity's pace and arriving spent change little or
lengthen; arriving spent cuts round-3 wins to 0-1% but costs d0 its band (85%) and lengthens the tail (24% at 10+).
No arm met 10+ under 10% at d1-d3.

**The alternative, built and not kept** (question 2; Matt's messages 13794 and 13811): Liberate 4, cities at 6
agents (4 from d1), the same deck. Fresh (seeds 50928-51023, beside r144 on them): wins 91/87/83/62%; 6-8 50/52/59/55%;
10+ 16/19/16/14%; round-3 wins 11/6/3/2 of 288 (its screen had 2/2/0/0); three seats 69/67/46/50% wins. It cost the
adversary's spread a quarter of its mid-game cities, which rulings.md says must grow with difficulty:

| Cities the adversary corrupts a game (setup's not counted) | d1 | d2 | d3 |
|---|---|---|---|
| r144 → r145 shipped (seeds 50784-50879) | 2.15 → 2.18 | 2.58 → 2.70 | 3.17 → 3.22 |
| r144 → alternative (seeds 50928-51023) | 2.55 → 1.82 | 2.94 → 2.32 | 3.49 → 2.79 |

**Paired on r145's own seeds** (50784-50879, the alternative farmed with `--at` after the freeze): wins 92/88/83/60%
against r145's 94/89/85/67; 6-8 49/52/49/52% against 51/48/55/53; 10+ 17/18/20/17% against 10/16/15/15; round-3
wins **2**/4/6/2 against **16**/7/6/3; cities corrupted a game at d1-d3 1.65/2.09/2.49 against 2.18/2.70/3.22. It
fixes d0's round-3 wins, lengthens the tail at every level and costs the spread a quarter; r145 ships Liberate 3.

## Content effects and notes

- With Liberate at 3 a city is cheaper to liberate than a station (4); the liberate capstones were priced against the
  action's 5 ambassadors (card-pricing.md) and now pay over the bill.
- Docs: the length ruling replaces 7-10 in rulings.md, principles.md (*Games run Spirit Island length*) and
  `design_check.json`'s `rounds` (now the bars: over half in 6-8, under 10% at 10+); the rulebook, teach, aid and rules
  pages print the new cap and price.
- The frozen read's build (878be62d) and the shipped one (036a1e2e) share configs, content and goldens; only tests
  and docs differ, so no replay was needed.
