# 214: tier-2 pricing and insight (r95)

`RULES_REVISION` 94 → 95, bumped from the constant (the prompt named none). Built on main 93548f2 (213, r94). Old
side: r94 on the same seeds, farmed from its tree before any edit (`f214-r94`), and r94 put back by overlay on the
final build (`s214-c`'s `r94` cells: they replay r94's games exactly, 81.8 / 64.6 / 48.8 / 36.3 both ways). Overlays:
[overlay.txt](overlay.txt) (prices), [overlay-ais.txt](overlay-ais.txt) (tracks), rerunnable with
`scripts/cards/apply_overlay.py` (`null` now drops a field). Specs: [spec.py](spec.py). Reads:
[insight_read.py](insight_read.py) (slack, bound, insight spend by board; tier-2 keeps by round),
[card_read.py](card_read.py), [resources.py](resources.py), [keep_read.py](keep_read.py) (a farmed file as a read folder).

## What changed

- **A tier-2 keep costs 3 insight** (2 with the keep upgrade; was 2 and 1).
- **The card-play tracks are early-heavy, late-flat** (199's `t4`): Builder 3, 7, 8, 9, 10; Protector 3, 7, 8, 9;
  Researcher 3, 6, 7, 8 (r94: 3, 5, 7, 8, 9, 10 / 3, 5, 6, 7, 8, 9 / 3, 4, 5, 6, 7, 8). Same tops, more insight early.
- **Every tier-2 card at 1.0-2.5 at the anchors with a keep of 3**: 14 repriced (below); 48 keep their price.

## The anchor check (all 62 tier-2 cards in the deck)

[anchors_a.md](anchors_a.md) and [anchors_b.md](anchors_b.md): each card's pay and get at the anchors, keep 2 and keep 3,
the arithmetic shown. Units with no anchor: a convert 10.6 (removal 5 + 0.7 of an ambassador, values.json's
`target_share`), an engine 3.5 rounds at a committed seat, a building's tags 1.5, Sway ±4. At keep 3 before repricing:
48 in band, 6 over 2.5, 8 under 1.0. The over-ceiling cards hang on an uncertain unit (a convert's worth, an engine's
use count), so each is priced to sit in band under **both** readings (convert 10.6 or 5; 3.5 uses or the 2 a seat
makes of antimatter); the under-1.0 cards are cut toward 1.2, or to just in band where the card is dear by design
(The Nuclear Option, Annihilation Lance).

| card | was | now | keep 3 | keep 2 | plays a game, r94 → r95 (fresh) |
|---|---|---|---|---|---|
| Mass Defection | 9 cr (3.59 / 2.90) | 18 cr | 2.03 (1.10 at convert 5) | 2.35 | 0.35 → 0.26 |
| Embassy of Exiles | 14 cr (4.53 / 3.83) | 30 cr | 2.37 (1.62 at 2 uses) | 2.62 | 0.42 → 0.16 |
| Penning Trap Array | 10 cr (4.94 / 4.18) | 30 cr | 2.36 (1.53 at 2 uses) | 2.59 | 0.18 → 0.05 |
| Antiproton Foundry | 10 cr (4.94 / 4.18) | 30 cr | 2.36 (1.53) | 2.59 | 0.21 → 0.08 |
| Positron Storage Ring | 9 cr (3.12 / 2.79) | 18 cr | 2.26 (1.68) | 2.47 | 0.11 → 0.05 |
| Forensic Audit | 27 cr (0.87 / 0.82) | 6 cr | 1.20 | 1.31 | 0.10 → 0.14 |
| Kettle Operation | 18 cr (0.76 / 0.68) | 2 cr | 1.18 | 1.44 | 0.08 → 0.09 |
| Surgical Strike | 14 cr (0.73 / 0.62) | 2 cr | 1.14 | 1.60 | 0.17 → 0.30 |
| Decapitation Strike | 4 cr (1.03 / 0.91) | 0 cr | 1.03 | 1.19 | 0.08 → 0.09 |
| Singularity Exploit | 8 cr (1.05 / 0.95) | 4 cr | 1.05 | 1.18 | 0.06 → 0.05 |
| Kinetic Interdiction | 8 cr (1.06 / 0.95) | 2 cr | 1.13 | 1.29 | 0.07 → 0.07 |
| Martial Cordon | 18 cr (0.87 / 0.76) | 8 cr | 1.08 | 1.30 | 0.06 → 0.13 |
| The Nuclear Option | 23 cr (0.76 / 0.70) | 6 cr | 1.03 | 1.16 | 0.17 → 0.18 |
| Annihilation Lance | 2 cr, 2 am (0.91 / 0.84) | 10 cr, 1 am | 1.05 | 1.17 | 0.12 → 0.08 |

The four raised over-ceiling cards read 2.47-2.62 at keep 2: priced for keep 3, they go back over the ceiling if
the keep returns to 2. Thin in band at keep 3 (left alone): Botnet Takedown and Fusion Commons 1.04, Market
Stabilization Board and Autonomous Defense Grid 1.02. Expensive tier-1 cards were not re-checked (the prompt allowed
moving them; none was asked). Martial Cordon went to 8, not the screened 10: the helper's keep-3 sum left out its play
insight (fixed before the fresh read). The give / get at `values.json` for these cards is in the effect ledger
(`experiments/results/effect-ledger/`), regenerated.

## Screens (spent fresh seeds 46240-46431: 192 seeds × 3 pairs, 576 games a cell and level, d0-d3)

| cell | d0 | d1 | d2 | d3 | bound | slack | tier-2 keeps rounds 1-4 a game |
|---|---|---|---|---|---|---|---|
| r94 (base) | 81.8 | 64.6 | 48.8 | 36.3 | 22.8% | 4.9% | 4.80 |
| `k3` (keep 3) | −4.9 ± 2.2 | −7.3 ± 2.4 | −10.2 ± 2.4 | −3.0 ± 2.4 | 33.2% | 3.9% | 2.74 |
| `t4` (track) | +0.5 ± 2.0 | +0.0 ± 2.4 | +2.3 ± 2.4 | +2.1 ± 2.3 | 10.6% | 7.6% | 6.06 |
| `t4k3` | −1.6 ± 2.1 | −5.7 ± 2.4 | −4.0 ± 2.4 | −0.5 ± 2.3 | 16.2% | 5.4% | 4.35 |
| `t4k3` + prices (screen 2) | −6.9 ± 2.2 | −8.7 ± 2.5 | −6.8 ± 2.6 | −5.6 ± 2.3 | | | |
| final build (screen 3) | 75.9 (−5.9 ± 2.2) | 56.1 (−8.5 ± 2.5) | 43.1 (−5.7 ± 2.6) | 30.0 (−6.2 ± 2.3) | 16.2% | 5.4% | 4.25 |

`k3` alone binds hardest but halves early tier 2; `t4` alone brings tier 2 earliest but insight binds half as often;
together they keep insight binding (16%) with early tier 2 near r94's. Taken: `t4k3` (Matt asked, design question 1,
default taken). The prices cost about 3-5 more. Greedy beside it (screen 3, `s214-cg`): r95 80.2 / 63.0 / 45.3 /
35.8, r94 85.2 / 68.9 / 55.0 / 42.0: greedy leads the strategist by 3-7 at both revisions on these seeds.

## Fresh read (46432-46527, 96 seeds × 3 pairs, d0-d4; r94 farmed on the same seeds before the first edit)

| d | r95 strategist | r94 | r95 − r94 (paired) | band (9543) | r95 greedy |
|---|---|---|---|---|---|
| d0 | 77.1 | 76.0 | +1.0 ± 3.2 | ≥ 85 | 77.8 |
| d1 | 57.6 | 63.2 | −5.6 ± 3.3 | 65-80 | 62.2 |
| d2 | 34.4 | 47.2 | −12.8 ± 3.4 | 50-65 | 49.7 |
| d3 | 30.6 | 33.0 | −2.4 ± 3.2 | 35-50 | 33.0 |
| d4 | 13.9 | 18.1 | −4.2 ± 2.4 | - | 14.9 |

**Every band unmet** (by 7.9, 7.4, 15.6, 4.4), reported, not dialled (amendment 9543). r94 itself misses on these
seeds (76.0 / 63.2 / 47.2 / 33.0) and on screen 3's (81.8 / 64.6 / 48.8 / 36.3); the asked levers cost about 6 more
(keep 3 about 3, the prices about 3-5). No allowed lever reaches the bands with insight still binding and every
tier-2 card in band: `t4` at keep 2, the best cell, read +0 to +2 over r94. The fresh d2 gap (−12.8) is twice
screen 3's (−5.7 ± 2.6 on 576 games), and greedy's 49.7 there against the strategist's 34.4 is larger than screen 3's
greedy lead (2.2): read d2 with screen 3. Rounds 7.55 (r94 7.62).

## Insight by board (strategist, a seat-round, levels pooled)

| board | slack: 213 (f213) / r94 / r95 | bound: 213 / r94 / r95 | to keeps r94 → r95 | to plays | unspent |
|---|---|---|---|---|---|
| Builder | 2.8% / 2.6% / 2.6% | 22.6% / 23.7% / 20.6% | 2.27 → 2.83 | 1.69 → 1.66 | 2.09 → 2.77 |
| Protector | 2.9% / 2.6% / 3.8% | 28.2% / 28.4% / 17.2% | 2.36 → 3.10 | 1.17 → 1.23 | 2.15 → 3.16 |
| Researcher | 8.0% / 8.2% / 9.2% | 17.5% / 20.4% / 13.7% | 2.09 → 2.63 | 1.16 → 1.16 | 3.31 → 4.50 (banked) |
| all | 4.6% / 4.5% / 5.2% | 22.7% / 24.2% / 17.2% | 2.24 → 2.85 | 1.34 → 1.35 | 2.52 → 3.48 |

213 is its fresh read's base cells (46336-46431), r94 and r95 this item's fresh seeds. Insight still binds in one
seat-round in six (one in four at r94); keeps take 0.6 more a seat-round (toward Matt's ~6), plays the same. Tier-2
keeps by round 4: 4.34 a game (r94 4.75); kept 13.1 (14.7), played 8.3 (9.6). The Researcher's bank grows by a third.

## Resource use (fresh, a seat-game, gained / spent)

| | credits | insight | energy | chips | antimatter | research cubes | card tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|
| r95 | 212.7 / 229.7 | 44.6 / 37.8 | 31.9 / 23.3 | 7.0 / 4.9 | 2.7 / 1.2 | 13.7 / 9.0 | 1.5 / 1.2 | 15.9 / 12.8 |
| r94 | 226.6 / 243.4 | 38.4 / 32.6 | 33.8 / 25.2 | 7.7 / 5.3 | 3.2 / 1.4 | 15.0 / 10.1 | 1.7 / 1.4 | 17.6 / 13.8 |

Insight up 6 a seat-game (the track); antimatter made 3.2 → 2.7 (the dearer plants and Positron Storage Ring), spent
43% of what is made (r94 42%).

## Checks

- Design check ([design-fresh.txt](design-fresh.txt), r95 fresh against r94 on its seeds): new at r95 `kept-not-played`
  Deep State Ledger (kept 0.14, played 0.03 a game; a card left alone, 1.14 at keep 3: per Matt's rule it would get
  more power, not a lower price, outside 214's levers); `dials-hide-design` (the keep 2 → 3, the asked change). The
  rest stand as at r94 (same-effect, standard-over-anchor, cheap-source antimatter 42% (r94 38%), one-spend
  ambassadors, usage, rounds d2-d4). Screen: [design-screen.txt](design-screen.txt).
- `play_read.py` (fresh, 1,440 strategist games): none kept in over 1% and never played; none never kept.
- `loop_smoke.py --run` (16 seeds): no timeouts, most plays of one card a game 3.
- `npm run cards:fit`: no change against r94 (the same four effect boxes under 7.5 pt).
- `bench/same_results.py`: not needed; no core rules file changed after the fresh read.
- Speed (`per_decision.py`, r94 built from its tree in `out/r94src`, 6 games d2, one thread, a quiet box): strategist
  1.07× a decision (1.26× a game: 7.5 rounds against 6.3), greedy 1.19× a decision (1.18× a game). The engine code is
  r94's: the cost is content (more insight early, 4-10% more options a decision). `speed.py` baseline updated to r95.

## Seeds and runs

Spent fresh 46240-46431 (screens 1-3: 9,216 + 2,304 + 9,216 games, greedy in screen 3). Fresh 46432-46527 (1,440
r95 strategist, 1,440 r95 greedy, 1,440 r94 strategist). Ledger next free 46528. Kept: `out/keep/f214`,
`out/keep/f214g`, `out/keep/f214-r94`, `out/keep/s214-a`, `out/keep/s214-c`, `out/keep/s214-cg`.
