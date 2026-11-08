# 149: board movement (r63)

Matt's board direction (2026-11-05, design doc rev 24), built on 148's r62. `RULES_REVISION` 62 → 63, bumped from the
constant (the prompt's r63 agreed). Scripts: [boards149.py](boards149.py) (writes the boards' Movement and tags into
`content/ais.json`, rerunnable), [specs149.py](specs149.py) (every screen and read spec), [ladder149.py](ladder149.py)
(the paired ladder read, `--moves` for moves per board), [pool149.py](pool149.py) (a screen pooled over the pairs, each
lever paired against base), [lost149.py](lost149.py) (Movement used and lost), [move_read.py](move_read.py).
Tables: [f1.md](f1.md), [f2.md](f2.md), [f3.md](f3.md), [screens.md](screens.md), [moves_dev.md](moves_dev.md).

## What changed

- **No paid Move, no chip sale.** `StandardAction::Chips`, `chips_credits` and `move_credits` are gone. Move is free
  and uses 1 of the seat's **Movement** this round (`board::moves_left`: `ai::movement` less the var
  `board.moved.<seat>`, cleared in cleanup, which also tallies `board.moves_lost.<seat>`). A card still sells for
  `sell_credits` 1 (already 1 at r62: nothing to change).
- **Movement:** `AiBoard.moves` (printed) and `Uncover.moves` (a Movement space). Builder and Protector 1, +1 on economy
  space 4 (where their Power / Diplomacy tag was); the Researcher 2, +1 on economy space 4 and card-play space 2.
  Cards that move ambassadors don't use Movement.
- **Revealed tags** count in `cards::tag_counts`, so like any tag (requirements, discounts, per-tag income and
  programs, capstones); sacrifices pick cards, so never them. Printed tags count for requirements only
  (`ai::printed_tags` in `required_tags`). Two a board on space 6 of each track (cost 3): Builder Power and Cyber,
  Protector Cyber and Diplomacy, Researcher Power and Industry; r62's other eight (spaces 2-4) are gone.
- **Agents:** `w.movement` (engine profile 1.0) values a seat's Movement over the rounds left at `rates.movement` 1.4
  credits, never the Movement left this round (it is lost unused, so holding it is worth nothing). The move rows
  `move_ambassadors` and `move_one` 1.5 → 2, `spread_ambassadors` 3 → 4, `consolidate` 1 → 1.5. `chips` dropped from
  the turtle and free-only policy lists. A free Move is planned by the search like any free action.
- **Difficulty dials:** `board_difficulty_at` 3 → 2 (d2), `build_bonus` 2 → 3 (d3).
- Measures: `board.amb_moves.<seat>` (every ambassador move a seat makes), `board.moves_lost.<seat>`. The view shows
  each seat's Movement left of its Movement, and a Movement mark on the track space.

## Give / get, before the first screen (`values.json`)

| Space | Give | Get | When it matters |
|---|---|---|---|
| Printed Movement 1 (Researcher 2) | nothing | 1.4 a round each, about 12 a game | every round |
| Economy space 4: Movement +1, 3 credits (cost 1) | 1 energy + the power's 1: 9 | the node (8) + 4.4 a round | third economy placement, round 2-3; pays back in about a round |
| Researcher card-play space 2: Movement +1, 2 insight (cost 0) | the power's energy: 4.5 | 5 + 1.4 = 6.4 a round | its first card-play placement, round 1-3 |
| Space 6: a tag, 2-3 credits (cost 3) | 3 energy + the power's: 18 | node (6, late) + 8-12 credits + the tag (1, and what it pays: 0.5 a round on a per-2-tags program, a discount, a requirement) ≈ 18 | about round 6, 3-4 rounds left: even on the way to a capstone, dear for the tag alone ("late and costly", as asked) |

The economy-4 space traded a tag (`tags.first` 1, once) for Movement +1 (1.4 a round): a gain. No conversion here
is break-even at the table except space 6, which is the asked "costly".

**Move cards** (only Liaison Corps and Summit Caucus in the deck; Transmission Corridor is out of it; Amnesty Decree
removes, it doesn't move). At the anchors (card-guidelines: a move has no anchor, so `values.json`): Liaison Corps
pay 16, get 18 → 20 (1.12 → 1.25); Summit Caucus pay 18, get 18 → 21 (1.00 → 1.17). Both under the tier-1 ceiling
of about 1.5, so **no price changed** (none lowered). Effect ledger fair price: Liaison Corps 10.3 → 11.6 (printed 8),
Summit Caucus 10.3 → 12.6 (printed 10). Plays per game (dev seeds, d0/d2): Liaison 0.82 → 0.90, Summit 0.64 → 0.72.

## Movement: before and after, per board

r62's paid Move is used about 0.5-0.8 a round a seat (Move actions 6.7-10.2 a game a table, dev seeds); chips were
sold 0.07 a game. Ambassador moves a game per seat (F2, fresh seeds, 288 games a level; r62 on the same seeds; F3 in [f3.md](f3.md) reads alike):

| d | Builder r62 → r63 | Protector | Researcher |
|---|---|---|---|
| d0 | 5.2 → 4.9 | 7.7 → 7.5 | 7.7 → 9.0 |
| d1 | 4.2 → 4.3 | 6.1 → 6.2 | 6.2 → 7.9 |
| d2 | 3.4 → 3.6 | 5.4 → 5.6 | 5.6 → 6.7 |
| d3 | 3.2 → 3.4 | 5.4 → 5.7 | 6.0 → 6.9 |

**Every board reaches its sites without the paid Move:** moves hold level or rise on every board, the Researcher's
by 1-2 a game, and Movement is left unused every round (lost a round: Builder 1.16, Protector 1.14, Researcher 2.17,
F2). Movement 9 on every board (screen 1) gained nothing (d2 −4.2 ±4.2, d0 +0.0 ±3.0): the cap doesn't bind. Claims
(0.3 a game) and liberations (about 4) are unchanged: they come from the supply, not from moves.

## Screens (dev seeds 36800-, the strategist, the three pairs pooled; [screens.md](screens.md))

- **The first r63 build against r62** (32 seeds, d2): 53.1 against 67.7. Restoring r62's eight revealed tags gave back
  +8.3 ±5.1 (d2), Movement uncapped nothing, the Movement term off −1.0 (on/off, below). Losing the early tags is
  what costs: the Builder's pair most (bp d2 44 → 69 with them).
- **Which tags** (64 seeds): the boards' own kinds on space 6 +3.1 ±2.4 (d2), 149's kinds a space earlier (space 5,
  cost 2) +2.1 ±2.3: both inside noise, so the asked layout (space 6, cost 3) stands.
- **The Movement term on / off** (32 seeds): d0 91.7 / 92.7, d2 53.1 / 52.1: no gap. Greedy uses no horizon term,
  so its line is unchanged by it; its cost a decision against r62 (`per_decision.py`, 16 games, quiet box): 9.7 →
  10.5 ms, **1.08×**, and 0.99× a game; the strategist 2.1 → 1.9 ms (0.93×). `speed.py check`: random 0.98×,
  greedy 1.12×, engine 0.96×, planner 0.89× a game (no verdict across a revision; baseline not updated).
- **d2 dials** (after F1 read d2 70.8): `counter_more_credits` 6 / 9 +0.5 / +0.0 at d2 and −9 to −10 at d3;
  `counter_more_all` 2 −3.1 / −11.5; `board_difficulty_at` 2 **−12.0 ±3.9** (taken: d0, d1, d3 untouched);
  `extra_threats_at` 2 −35.4.
- **d3 dials** (after F2 read d3 54.2): `build_bonus` 3 **−3.1 ±3.2** (taken), `stage2_removed` 2 −13.5 ±3.7.

## The ladder (strategist, paired against r62's binary and configs on the same fresh seeds)

| Read | Build | Seeds | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|---|
| F1 (frozen) | r63, r62's dials | 37624-37719 | 94.1 (r62 94.8) | 75.7 (80.9) | **70.8** (66.3) | 48.3 (46.9) |
| F2 | + `board_difficulty_at` 2 | 37720-37815 | 94.4 (90.6) | 79.2 (80.9) | 60.8 (71.9) | **54.2** (54.2) |
| F3 (final) | + `build_bonus` 3 | 37816-37911 | **89.9** (89.9) | **82.3** (78.8) | **55.6** (68.8) | **46.5** (48.3) |

Each cell is 288 games (two 48-seed sets × three pairs); SE about 1.8 at d0, 2.9 at d2-d3. **F3 is the final read**
(fresh seeds, the final build and dials): d1 70-85, d2 55-65 and d3 40-50 met; **d0 unmet** by 0.1 (0.06 SE). What
bound it: the seed set (r62's binary reads the same 89.9 on the same games; F1 and F2 read r63 at 94.1 and 94.4),
and no difficulty dial reaches d0, which is the base rules (the dials start at d1). Both retuned dials act only from
d2 / d3 (`adversary::level`), so d0 and d1 play the same rules on all three fresh reads: over F1-F3's 864 games, d0
reads **92.8** and d1 79.1 (no d0 or d1 lever was picked on any of them). The final build replays F3's set a
exactly (`bench/same_results.py`, 576 games; only docs text changed after the reads). Paired gaps against r62 (F3):
d0 +0.0 ±2.1, d1 +3.5 ±2.7, d2 −13.2 ±3.3 (the d2 dial), d3 −1.7 ±3.4. F3's sets: d0 88.9 / 91.0, d1 84.7 / 79.9,
d2 56.2 / 54.9, d3 50.0 / 43.1. d2 sits at its band's floor on F3 (60.8 on F2): the d2 dial (−12 on the dev seeds)
is near a full band's width, and a d2 dial half its size doesn't exist among the screened ones.

## Smokes

Engine smoke (`use.json`, 16 seeds): no deck card kept and never played above 6% (Telemetry Archive 6.2%, Smart Grid
Backbone 4.2%, as at r62); powers all used where their board sits. `loop_smoke.py --run`: no timeouts, no card at the
play flag (most 3 a game). `npm run cards:fit`: fits (no card face changed). `bench/same_results.py`: not needed (no
core rules file changed after a frozen run; the dials are configs, and F2/F3 are fresh reads on the dialed baseline).
