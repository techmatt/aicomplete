# 306: finale buildings (r148 → r149)

Matt (2026-11-20): repeatable tier-2 "finale" buildings, hard to get and needing building up to, whose once-a-round
Action spends a late-game resource (antimatter, or large amounts of energy); a free engine beside a plant is the
intended payoff. Rules revision 148 to 149, bumped from the constant. Cards: [finale.overlay](finale.overlay).
Tables: [fresh.md](fresh.md) and [screen.md](screen.md) (`make_tables.sh`; readers [finale.py](finale.py),
[am_paired.py](am_paired.py)).

## The buildings

Pay = credits + the tier-2 keep 8, and per use 18 an antimatter or 3 an energy; get at the unit anchors (a
liberation by card with the claim rider 26, 302's bill; a troop 3.33; an ambassador 5.33; a Sway 2.67; an insight
2.67; a project discarded at half its cost at 2 countries, 24-36). Fresh read columns: per 100 games, and uses per
seat that built it.

| Card | Tags, hard to get | Action | Get / pay a use | Card at 3 uses | Fresh: kept / built / uses (a builder) | Built by r3 |
|---|---|---|---|---|---|---|
| Siegebreaker Reactor | Security, Power; needs 3 Power, 4 cr | 1 antimatter: remove 2 troops on or next to the target corrupted city, rescue it | 32.7 / 18 = **1.81** | 98 / 66 = 1.48 | 10.4 / 7.5 / 6.0 (0.80) | 3% |
| Concord Reactor | Diplomacy; 12 cr, needs 2 Diplomacy | 1 antimatter: gain 5 ambassadors, sway 4 toward Unity | 37.3 / 18 = **2.07** | 112 / 74 = 1.51 | 19.0 / 17.0 / 22.7 (1.34) | 27% |
| Singularity Exploit (converted) | Cyber; needs 3 Cyber, 4 cr | 1 antimatter: discard an adversary project, gain 2 insight | 29.3-41.3 / 18 = **1.63-2.30** | 88-124 / 66 = 1.33-1.88 | 13.3 / 10.1 / 9.4 (0.94) | 15% |
| Fusion Torch Array | Power, Industry; needs 3 Power, 8 cr | 5 energy: rescue the target corrupted city | 26 / 15 = **1.73** | 78 / 61 = 1.28 | 12.5 / 8.1 / 7.0 (0.89) | 4% |

The table (`values.json`, `effect_ledger.py`) reads each above its price: reasoned 29.2 / 48.0 / 30.7 / 23.1 for
printed 4 / 12 / 4 / 8. Singularity Exploit takes an `overrides` row (+30): the table prices a discarded project at
5.33 whatever its compute, so its Action read 0; the row lifts its keep, not a use, and the judge used it whenever
it could (skips about 1% of the rounds it was offered). Faces fit at 7.5 pt (`cards:fit`); `play_read.py`: no card
kept and never played; `loop_smoke.py --run`: no loop.

**Why these, and their hard to get.** Each spends what the late game holds and does what it wants: two win-route
engines (a liberation a round: Siegebreaker on antimatter, with the troop clearing a guarded city needs; Fusion
Torch on the energy that piles up late, 3.7 held at round 6 and 6.6 at round 7 on r148), ambassadors and Unity
(Concord: the Liberate action's currency), and the adversary's projects (Singularity: the compute sink as an engine).
"Hard to get" is a tag chain where the seat's engine is (3 Power, the power plants' tag; 3 Cyber, Dark Fiber
Collider's), or a big price over a short need (Concord: 12 credits, 2 Diplomacy, Treaty Isotope Bank's tag), plus
the fuel. Siegebreaker and Fusion Torch share an effect (guideline 11): the fuel is the point, and the troop clause
is Siegebreaker's.

**Cut on the way.** Siegebreaker's third clause (2 ambassadors placed on the city, 2.41 a use) and Singularity's botnet
and 1-antimatter build cost failed `cards:fit` (6.3 and 6.9 pt). Singularity's draw 1 rider (screened) became 2
insight after the diff review: with Command-and-Control Grid it was a second repeatable Cyber draw, against the
draw-chain ruling. Siegebreaker and Singularity sit under the antimatter events' 2.0 at their low end (1.81, 1.63):
Siegebreaker's 2.41 didn't fit.

## The result (fresh, the judge, seeds 51360-51455, 288 games a level, paired)

r148 is main at 065566d7 (`farm-seeds --at`), r149 this build.

| | d0 (at least 90) | d1 (80-95) | d2 (75-90) | d3 (55-70) | all |
|---|---|---|---|---|---|
| Wins r148 → r149 | 98.3 → **96.5** | 97.2 → **96.5** | 87.2 → **89.9** | 72.6 → **74.3** | +0.5 ± 1.1 |
| Antimatter gained / spent a seat-game | 2.40 / 1.19 → 2.61 / 1.38 | 2.42 / 1.21 → 2.85 / 1.49 | 2.76 / 1.41 → 2.88 / 1.54 | 2.70 / 1.32 → 2.94 / 1.50 | 2.57 / 1.28 → **2.82 / 1.48** |

Spent rises **+0.19 ± 0.04** a seat-game (paired; 1.28 → 1.48, against 303's quoted 1.26 on its seeds), made +0.25 ±
0.08, spent ÷ gained 50 → 52%. d1 sits above its 95 ceiling on these seeds on both sides (97.2 → 96.5); every other
level is in band. Games run 6.75 → 6.76 rounds; won by round 3, 30 → 18 of 1,152.

**The two seed sets disagree on spend.** The same r149 build on the screen's seeds (51264-51359, 303's spent fresh
set; `r149final` below) reads spent +0.03 ± 0.04 and made −0.22 ± 0.08 (wins +2.7 ± 1.3), where the fresh seeds read
+0.19 and +0.25: about 2.8 SE apart, mostly Treaty Isotope Bank (68 → 98 antimatter made in 100 games on fresh, 66 →
64 on the screen) feeding Concord Reactor. Pooled (2,304 games): spent **+0.11 ± 0.03** a seat-game (1.27 → 1.38),
made +0.02 ± 0.06. So spend is up, but by about a tenth, not the fresh read's fifth.

## The screen (seeds 51264-51359, 1,152 games an arm: [screen.md](screen.md))

On r149's first deck (Siegebreaker needing 3 Security, Singularity drawing 1): wins +2.9 ± 1.3, spent 1.26 → 1.29.
Levers, each on the same seeds:

| Arm | Spent a seat-game | Made | Wins vs r148 |
|---|---|---|---|
| r148 | 1.26 | 2.71 | |
| r149, first deck | 1.29 | 2.49 | +2.9 ± 1.3 |
| eased (needs 2, Concord 8 credits) | 1.31 | 2.44 | +3.6 ± 1.3 |
| the judge with idle antimatter off (`w.am_cap=0`) | 1.23 | 2.86 | +2.4 ± 1.3 |
| a keep lever (`w.am_act`: a building's antimatter Actions off an idle pile) | 1.31 | 2.49 | +2.9 ± 1.3 |
| pair (Siegebreaker 3 Power, Fusion Torch 3 Industry) | 1.32 | 2.50 | +2.5 ± 1.3 |
| up (each finale gains 1 antimatter when built) | 1.39 | 2.64 | +3.6 ± 1.3 |
| both | 1.43 | 2.69 | +3.4 ± 1.3 |
| r149 as shipped (`r149final`) | 1.29 | 2.49 | +2.7 ± 1.3 |

**Why spend is supply-bound.** A seat that built a finale played its Action in about 1 round of the 3 left (built in
round 5.1-5.9 of games of 6.8); offered, it was taken (skips under 1% of offers); the other rounds it held no
antimatter. The seats holding idle antimatter rarely hold a finale, and finales take keeps from plants (Dark Fiber
Collider 85 → 69 made, Penning Trap Array 61 → 48 on the screen). Neither easier needs, nor the judge's idle rule
off, nor a keep rule pricing an Action's antimatter off an idle pile moved spend; only free antimatter on the build
did. Matt (message 14106): no up-front gain, "a supply-bound spend is the design working". `w.am_act` was reverted
(no effect); pairing kept for Siegebreaker only (Fusion Torch on Industry was kept less: 12.8 vs 16 in 100 games).
The shipped build's Siegebreaker and Singularity changes ran only as that last row and the fresh read.

## Checks

`speed.py check`: no verdict across a revision (baseline r148); engine 0.72×, greedy 0.98×, planner 0.90× its
time; no agent changed. `design_check.py --quiet` on the fresh read: as on main but its kept-not-played noise
(Multiyear Defense Budget, Arcology Foundry at 0.03-0.04 a game) and d2's rounds line clearing.

## For Matt

- The spend bar (Matt, message 14106: his, and too strict): +0.19 ± 0.04 on the fresh seeds but +0.03 on the
  screen's, +0.11 ± 0.03 pooled; supply-bound. A finale's owner can pay its Action in about 1 round of 3 after
  building it; when it can, it does. The one lever that moved spend on the screen is each finale gaining 1
  antimatter when built (+0.08 a seat-game; with the plant pairing +0.12): data, not taken.
- Siegebreaker Reactor at 1.81 a use is under the antimatter events' 2.0-2.5 (its 2.41 version didn't fit the face):
  a stronger effect that fits, or leave it as an engine priced by its build-up.
