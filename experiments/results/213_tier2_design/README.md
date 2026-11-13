# 213: tier 2 as the majors (r94)

`RULES_REVISION` 93 → 94, bumped from the constant. Built on main 6f18850 (212, r93). Old side: r93 = 6f18850's
binary and tree (`out/r93src`). Every number is a candidate. Overlay: `overlay.txt` (cards, rerunnable with
`scripts/cards/apply_overlay.py`); `add_powers.py` (the two board powers). Specs: `spec.py`. Card list: `t2list.py`.

## What changed

- **Breakthrough is off** (`board.breakthrough_unity` 0; the action and its parameters stay in code, "0: no such
  action", for a later cleanup to delete).
- **Every board has its own antimatter source, each costly:** the Builder's new **Refine** (2 chips + 1 energy), the
  Protector's new **Joint Facility** (12 credits + 1 energy), the Researcher's Particle Run (1 Power + 1 energy), as
  before. Each gives 1 antimatter, once a round.
- **Horizon's Tidal Grid converter** 2 research cubes + 1 energy → 1 antimatter (was 1 cube + 1 energy → 2; it made
  65-68% of all antimatter, the one-source warning). A lever beside the asked work, screened on and off (screen 1).
- **Six tier-2 cards reworked into antimatter majors** (each was played 0.02-0.05 a game at r93), **three new tier-2
  majors**, and **two new expensive tier-1 cards** (an antimatter source and a spend). Cards that spend antimatter:
  7 → 17 (15 tier 2, 2 tier 1).
- **Agents (`am_source`, a behavior, on):** with `make_short`, a board power or built Action that gains antimatter is
  used, before Make antimatter's cubes, when it lets the keep plan play a card this round it can't now. Screen 2 left
  Refine and Joint Facility at 0.00 a seat-game without it. An eval arm beside it (`w.am_power_earn`: an antimatter
  card in hand counting the board's antimatter powers as income) played the same games as base and was cut.
- **Viewer:** the five new cards take unused drawn scenes (`scenes.py`); their pricing notes are in
  `docs/card-pricing-notes.json` (`notes.py`). The tier-2-off arm is `experiments/configs/no_tier2.json`
  (`scripts/configs/no_tier2.py`, run by regen.py): every tier-2 card out, the opening's 2 tier-2 cards dealt from
  tier 1.

## Give / get (before the first screen)

Anchors: pay = credits + 4 × (keep + play insight) + other costs at the anchors (antimatter 18); get at the anchors;
ratio get ÷ pay (ceiling 1.5 tier 1, 2.5 tier 2). Table: the effect ledger's printed (credits + insight to play + other
costs at `values.json`, keep excluded) against its reasoned value.

| card | tier, kind, tags | price | effect | anchors pay → get (ratio) | table give → get |
|---|---|---|---|---|---|
| Dyson Swarm Tranche | 2 program, Power/Industry, needs 2 Power | 12 cr, 1 am, 1 ins | 10 energy; +2 energy income | 42 → 73 (1.74) | 32.5 → 64.5 |
| Recursive Self-Improvement | 2 program, Science/Cyber, needs 2 Science | 5 cr, 1 am, 2 ins | +3 insight income; draw 3 | 39 → 51 (1.31) | 28 → 32.3 |
| Peacekeeping Mission | 2 event, Security, needs 2 Security | 20 cr, 1 am | liberate a city in reach; up to 3 troops in its country | 46 → 60 (1.30) | 38 → 59 |
| Root Access | 2 event, Cyber, needs 2 Cyber | 18 cr, 1 am | liberate a city in reach; a botnet on or next to it | 44 → 61 (1.39) | 36 → 57 |
| Hostile Buyout | 2 event, Diplomacy | 22 cr, 1 am | liberate a city in reach; 3 ambassadors | 48 → 69 (1.44) | 40 → 62 |
| Decapitation Strike | 2 event, Security, needs 2 Security | 4 cr, 1 am | up to 3 troops in one country; a botnet in reach | 30 → 31 (1.03) | 22 → 20.3 + 6 override = 26.3 |
| Collider Consortium (new) | 2 program, Science (help) | 8 cr, 1 am, 1 ins | 8 research cubes; another player 3 | 38 → 66 (1.74) | 28.5 → 31.8 |
| Catalyzed Fab Run (new) | 2 event, Industry/Power (help) | 2 cr, 1 am | 6 chips; another player 2 | 28 → 48 (1.71) | 20 → 30 |
| Antimatter Accord (new) | 2 program, Diplomacy | 8 cr, 1 am, 1 ins | 6 ambassadors; 4 Unity | 38 → 64 (1.68) | 28.5 → 41 |
| Cyclotron Lease (new) | 1 building, Power | 22 cr | Action: 2 energy → 1 antimatter | 26 + 9 a use → 18 a use (4 uses 1.16) | 22 → 25.2 |
| Accelerator Diplomacy (new) | 1 event, Diplomacy | 4 cr, 1 am | 5 Unity; 2 ambassadors | 26 → 36 (1.38) | 22 → 28 |
| Refine (Builder power) | board | 2 chips + 1 energy | 1 antimatter | 16.5 → 18 (1.09) | 12.5 → 18 |
| Joint Facility (Protector power) | board | 12 credits + 1 energy | 1 antimatter | 16.5 → 18 (1.09) | 16.5 → 18 |

Decapitation Strike's table read is fixed with a `values.json` override (+6, its `why` there): the table prices a botnet
removed at 8 against its anchor 16. The sources sit just above par at the anchors (antimatter is never cheap); the
majors at 1.3-1.75, under tier 2's ceiling.

**A place to matter.** Antimatter comes about 1 a round from a committed source, so a seat holds its first by round
2-3 and plays a major in rounds 3-7, the middle of an 8-10 round game; the liberation majors (Peacekeeping, Root
Access, Hostile Buyout, beside The Nuclear Option and Great Liberation) are the purge's end game.

**Loops and caps.** Every new major is one-shot (a program or event), so none repeats: Collider Consortium's 8 cubes
make at most 2 antimatter back through Make antimatter (4 cubes each), once; Catalyzed Fab Run's chips feed Refine
once a round. The two sources are once a round (a power, a building's Action). Gifts say "another player"; nothing
transfers.

**Can sim players play it.** The 16-seed smoke (d2) played every new and reworked card (0.02-0.35 a game); in the
fresh read 0.07-0.40 a game each, against 0.00-0.05 at r93 ([fresh.md](fresh.md), *Touched cards*).

## Screens (dev seeds 36800-36831)

[screen.md](screen.md): the first build (cards, powers, Breakthrough off) read 85.4 / 69.8 / 59.4 / 47.9 / 25.0 (r93
91.7 / 70.8 / 60.4 / 43.8 / 19.8), but antimatter piled (5.7 made, 1.6 spent a seat-game) and Tidal Grid's converter
still made 65% of it. The `tg` lever (converter 2 cubes + 1 energy → 1) cut supply to 2.9 and its share to 12%, at
0 / −5.2 / −7.3 / −5.2 / −1.0 (± 3.5-4.5): taken, the one-source warning's fix. Breakthrough back on (`bt`) read +0 /
+4.2 / +11.5 / +15.6 / +11.5 there. [screen2.md](screen2.md): with `tg` in, `am_source` on against off (`src0`) +0 /
+2.1 / +5.2 / +1.0 / +2.1 (± 2.1-3.0), and it moves Refine and Joint Facility from 0.00 to 0.09-0.10 a seat-game.

## Fresh read (46336-46431, 96 seeds × 3 pairs × d0-d4; r93 on the same seeds from its own tree)

| d | r94 | r93 | r94 − r93 (paired) | band (9542) | | Breakthrough on | tier 2 off |
|---|---|---|---|---|---|---|---|
| d0 | 84.4 | 92.0 | −7.6 ± 2.7 | ≥ 85 | −0.6 | 88.9 | 49.0 (−35.4 ± 3.6) |
| d1 | 63.5 | 71.9 | −8.3 ± 3.8 | 65-80 | −1.5 | 64.2 | 37.5 (−26.0 ± 3.6) |
| d2 | 48.3 | 53.8 | −5.6 ± 3.9 | 50-65 | −1.7 | 53.1 | 26.0 (−22.2 ± 3.6) |
| d3 | 34.4 | 43.8 | −9.4 ± 3.7 | 35-50 | −0.6 | 39.6 | 16.3 (−18.1 ± 3.2) |
| d4 | 15.6 | 19.8 | −4.2 ± 2.9 | - | | 17.4 | 5.6 |

Each band misses by 0.6-1.7 points: reported, not chased (9542). The drop against r93 is the antimatter route's: Breakthrough
off is about 5 of it at d0, d2 and d3 (the `bt` column), the costlier Tidal Grid converter about 5 at d1-d3 (screen 1).
Rounds 7.28 (r93 6.94).

**Tier 2 off** (every tier-2 card out, the opening's 2 tier-2 cards dealt from tier 1): d2 −22, d3 −18, a clear drop;
d0 49.0, far under the bar of 85. r93 with tier 2 off read d0 about 73 (dev seeds) and r94's tier-2-off arm with
Breakthrough back on 72 (dev seeds, d0-d1 only): tier 1 alone has never carried d0 to 85; the losses are the round cap
(deck_empty 41 of 96 at d0, dev): tier 1 purges too slowly. Matt's question (default taken, no answer): report it.

**The oracle without tier 2** (24 seeds a level, the fresh read's first 8 a pair; `f213-not2-skilled`): d3 75%
(18 of 24), d4 58% (14 of 24); the strategist on the same games 17% and 4%. Direction: a perfect player still wins
most of the hard seeds with tier 1 alone; the sim players can't.

**Tier-2 keeps by round 4** (a game, rounds 1-4): 4.83 against 212's read 4.28 (seeds 46240-46335); against r93 on
the same seeds 4.68, +0.15 ± 0.07 (by level −0.03, +0.11, +0.17, +0.36, +0.14). The rise is in rounds 2-4; round 1's
falls (0.69 against 0.74), the tier-1 draft drawing the new dear tier-1 cards. Tier-2 plays 9.22 a game (r93 8.57).

**Antimatter** (a seat-game): made 2.90, spent 1.27 (r93 4.50 / 2.59); 91% of won games spend it (r93 95%, all but 4
points Breakthrough then). Sources: Make antimatter 31%, income (Positron Storage Ring, the capstones) 23%, Tidal Grid
11%, Penning Trap Array 8%, then Cyclotron Lease, Particle Run, Refine, Joint Facility, Antiproton Foundry 3-5% each.
Spends: Accelerator Diplomacy 16%, Antimatter Accord 10%, Catalyzed Fab Run 9%, Annihilation Lance 9%, no card over
16%. The one-source and one-spend warnings are clear; spent is 44% of made (the `usage` band 50-110%).

**Resource use** (fresh, a seat-game, gained / spent, r94 against r93): see [resources.md](resources.md).

## Checks

- Design check ([design-fresh.txt](design-fresh.txt), r94 base against r93 on the fresh seeds): new at r94:
  `same-effect` (Refine, Joint Facility, Particle Run all "gain 1 antimatter": the costs differ, the effect doesn't),
  `cheap-source` antimatter 37% (Make antimatter 31% and Cyclotron Lease 5%, max 20%), `usage` Make antimatter 0.91 a
  seat-game (hedge band 0.05-0.5), Breakthrough unused (its row is off), antimatter spent 44%; `dials-hide-design`
  (Breakthrough's 2 → 0 beside the content: the asked change). Gone: `one-source` and `one-spend` antimatter.
- `play_read.py` (fresh, 2,880 games): no card kept in over 1% of games and never played; none never kept.
- `npm run cards:fit`: Peacekeeping Mission 6.3 pt and Root Access 6.9 pt fall under 7.5 in the effect box ("Pay 1
  antimatter." added); Kinetic Interdiction and Annihilation Lance, whose texts 213 left alone, read 5.8 pt.
- `loop_smoke.py --run` (16 seeds): no timeouts, most plays of one card a game 2.
- Speed (`per_decision.py`, 8 games, d2, r93 → r94, the box loaded: ratios only): strategist 0.84× a decision (0.71×
  a game), greedy 0.84× (0.91×). The `speed.py` baseline is not updated (no verdict across a revision; a loaded box).
- Viewer: the new cards render with their scenes, pricing lines and r94 stats (`s213b/results.json`, the dev screen's
  base run natively, the same games as the farmed one); `npm run cards:check` fails only on cards 213 left alone
  (Annihilation Lance, Fusion Commons, Containment Reactor name no scene; four scene images not served).
- `bench/same_results.py`: not needed; no core rules file changed after the fresh read (the `cut()` edit in
  standard.rs only hides the off Breakthrough from the docs, and was in the binary that farmed the tier-2-off arm).

## Seeds and runs

Dev 36800-36831 (screen 1: 1,440 r94 + 480 tier 2 off + 480 r93 + 192 r93 tier 2 off; screen 2: 2,400). Fresh
46336-46431 (2,880 r94 base and Breakthrough on, 1,440 tier 2 off, farmed; 1,440 r93 natively from `out/r93src`);
the oracle row on its first 8 seeds a pair, d3-d4, tier 2 off. Ledger next free 46432. Kept: `out/keep/f213`
(base and `bt`), `out/keep/f213-not2`, `out/keep/f213-r93`, `out/keep/f213-not2-skilled`.
