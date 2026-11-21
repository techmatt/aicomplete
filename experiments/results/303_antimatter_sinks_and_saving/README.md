# 303: more antimatter sinks, and the sim players' saving (r147 → r148)

Matt (2026-11-20): add more antimatter cards to the deck, and improve the sim players' antimatter use. Rules revision
147 to 148, bumped from the constant. Cards: [sinks.overlay](sinks.overlay). Tables: [fresh.md](fresh.md) and
[screen.md](screen.md) (`make_tables.sh`; readers [am_cards.py](am_cards.py), [am_spend.py](am_spend.py)).

## The result (fresh, the judge, seeds 51264-51359, 288 games a level, paired)

r147 is main at 2c78093c (`farm-seeds --at`), r148 this build.

| | d0 (at least 90) | d1 (80-95) | d2 (75-90) | d3 (55-70) | all |
|---|---|---|---|---|---|
| Wins r147 → r148 | 91.3 → **94.1** | 89.9 → **91.3** | 83.7 → **86.5** | 66.7 → **68.8** | +2.3 ± 1.4 |
| Antimatter spent ÷ gained | 33 → **46%** | 34 → **45%** | 37 → **46%** | 34 → **48%** | 35 → **46%** |
| Gained / spent a seat-game | 3.55 / 1.16 → 2.63 / 1.21 | 3.53 / 1.21 → 2.77 / 1.26 | 3.38 / 1.25 → 2.79 / 1.29 | 3.01 / 1.02 → 2.65 / 1.28 | 3.37 / 1.16 → 2.71 / 1.26 |

Every level in band; the levels' gaps (+1.4 to +2.8) sit within an SE or so. Games are a little shorter (7.20 → 7.04
rounds; 10+ at d0-d2 13-17% → 10-12%). Antimatter held at the end of round 8 falls 3.07 → 2.10; seat-games ending
with any held, 60 → 55%.

**What moved the ratio: mostly less made, a little more spent.** Spent rises 9% (1.16 → 1.26 a seat-game); made
falls 20% (3.37 → 2.71). Spent is bounded by what the sinks are for: the six liberate sinks share one demand (a game
needs only so many cities freed), so the two new ones displace the old ones (The Nuclear Option 18.6 → 14.9 plays in
100 games, Hostile Buyout 21.3 → 15.0, Peacekeeping Mission 15.9 → 11.6) more than they add. Made falls because the
new cards take keeps the plants had (Treaty Isotope Bank 112 → 66 antimatter made in 100 games, Antiproton Foundry
53 → 23) and because the judge no longer runs a plant into a pile it can't spend.

## The two halves (dev screen, seeds 36800-36831, 384 games an arm: [screen.md](screen.md))

A factorial on r147's deck and on the screened deck (three cards: Clearance Strike too, dropped since; the lever
before the review's fixes), the judge with idle antimatter off, at 0.5 and at 0.2:

| Arm | Spent ÷ gained | Gained / spent a seat-game | Wins |
|---|---|---|---|
| r147's deck, lever off | 33% | 3.66 / 1.21 | 84.9 |
| lever alone (`am_idle` 0.2) | **44%** | 2.96 / 1.29 | 86.5 (+1.6 ± 1.6) |
| cards alone | **42%** | 3.06 / 1.29 | 86.7 |
| both | **50%** | 2.63 / 1.33 | 87.2 |

Each half lifts the ratio about 9-11 points and the two add to about 17 (the fresh read's 11 is on the final two
cards and the fixed lever). Each half adds about +0.08 spent a seat-game; the rest is less made. The lever's one
real new spend is Orbital Relay Launcher's Action (5 → 25 uses in 100 games on r147's deck; 2.7 → 9.5 in the fresh
read): a repeatable sink the judge left idle while its antimatter was worth a full 12 credits.

## The cards: give / get at the anchors, before any sim

Pay = credits + the tier-2 keep 8 + 18 an antimatter. Get at the unit anchors: a liberation by card with the claim
rider 26 (302's bill: 20 + 6), a research cube 4, a chip income 13 a unit (287's Orbital Mass Driver). The table
(values.json, `effect_ledger.py`) reads each well above its keep.

| Card | Tag, kind | Text | Pay | Get | Get / pay | Table net | Fresh: offered / kept / played in 100 games |
|---|---|---|---|---|---|---|---|
| Beamline Liberation | Science, tier-2 event | 0 cr, 1 am: Rescue the target corrupted city; gain 7 research cubes | 26 | 26 + 28 = 54 | **2.08** | +27 | 40.4 / 21.4 / 17.9 |
| Reactor Retooling | Industry, tier-2 event | 3 cr, 1 am: Rescue it; +3 chip income | 29 | 26 + 39 = 65 | **2.24** | +38 | 40.2 / 21.5 / 18.2 |

**Why these two, and why liberations.** The liberate capstones are the sinks the judge plays: kept about half the
time offered and played almost every time kept (302). Science and Industry had none (The Nuclear Option is Power,
Hostile Buyout Diplomacy, Root Access Cyber, Peacekeeping Mission Security), and neither card has `needs`, so any
seat holding antimatter can play one. Each rider is its tag's resource (cubes Science's, chip income Industry's);
`design_check`'s tag story raises nothing. Faces fit (`cards:fit`: 0 boxes under 7.5 pt). Neither is kept and never
played (`play_read.py`: 0 cards); `loop_smoke.py --run`: no loop.

**Clearance Strike, dropped.** A third, Security (4 troops and 6 agents on or next to a corrupted site, gain 2
ambassadors), ran in the screen (kept 15, played 10 in 100 games). The diff review found its 2.31 priced each troop
at the table's 7.33 (a whole removal), not the anchor's 3.33: 44 for 26, **1.69**, under the sink floor, and 4
troops is a disguised "remove all" on a board holding 1-2. No honest Security effect reached 2.0 without a remove-all
number or a fourth line, so it went before the fresh read, with its `values.json` row (+16).

## The sim players' saving

`am_save` (211) already keeps a hand card's antimatter; it never told a seat its *pile* was idle. New weights, the
judge's at `w.am_cap=1`, `w.am_idle=0.2` (`eval.rs`, mirroring 128's `energy_cap` / `energy_idle`): antimatter held
beyond 1, or beyond what the hand's cards whose `needs` are met spend, counts a fifth of a held one's 12 credits; the
keep plan prices a card's antimatter, up to that idle count, at a fifth. So the judge spends a pile it holds on a
repeatable sink (Orbital Relay Launcher above), keeps sinks when it sits on idle antimatter, and stops running a plant
into a pile it can't spend. 0.2 over 0.5: 0.5 moved the ratio 3 points (36%) and Orbital Relay Launcher to 7 uses in 100 games, against 44% and
25.
Off by default; greedy and the engine don't read it (flat mode only: horizon mode and the table's hand terms ignore
it, as `agents.md` says). Fast-lane test: `idle_antimatter_counts_a_share`.

Known limits (diff review, kept): the idle count reads the hand at the keep, so a card kept on the discount is priced
in full at play (it is then a claim); plays-per-keep for the sinks didn't fall (the new two 84-85%). `sink_terms`
still values a converter's fuel against the full weight.

## Checks

`speed.py check`: no verdict across a revision (baseline r144); every agent at 0.57-0.98× the baseline's time
(engine 0.65, greedy 0.57), so no slowdown. `design_check.py` on the fresh read: kept-not-played and usage as on main
but the antimatter usage line (d3-d4 spent 34 → 48% of gained, its band 50-110%) and Cryptojacking Campaign leaving
the kept-not-played list.

## For Matt

- Spent per seat rises only 9%: the liberate sinks compete for one demand, and the steady plant output has few
  repeatable sinks to go to (Containment Reactor, Kinetic Interdiction, Orbital Relay Launcher). Lifting spend itself
  likely needs repeatable antimatter Actions (a building, priced per use), which at 2.0× a use (36 credits of get
  for 1 antimatter) is a strong engine with a plant beside it: a design call.
