# 103: what landed, what was screened, r40

The profile is [profile.md](profile.md); where an item's time goes, [time-split.md](time-split.md).

## Class 1: identical games

Each checked with `bench/same_results.py` against the start of the item (`experiments/same-103.json`:
the strategist, the engine, the planner and `planner:strategy=1` on the three pairs, 16 seeds from 1000,
d2, 192 games): identical after every step. Speed is `per_decision.py OLD NEW --agent strategist`, 16
games, one thread, best of 2, each step against the one before.

| step | what | strategist after / before |
|---|---|---|
| 1-3 | `FlatMap` compares names eight bytes at a time (`Probe`, `cmp_bytes`); a card by its key's address (`Content::card_at`); `hex_name` and the flag keys without `format!`; counters, piles and tokens interned only when new (`entry_str`, `insert_str`); `ai.boards` read once (`World::ai_boards`); sacrifice sources' piles as `KeyBuf`s and labels joined by hand; `has_token_card` for the emptiness checks | about 0.83 (derived: the whole is 0.76) |
| (tried) | `Key` carrying its first eight bytes, so the search reads fewer strings | 0.99: reverted |
| 4 | the seat-terms store: tags, prices and income kept a decision long, keyed by all they read (`tableau_parts`) | 0.94 |
| 5 | `KeyBuf::num` writes its digits in place | 0.97 |
| 6 | a threat's standing counters and `has_passive` walk the passives instead of listing them; `threats::on` on the stack; the store capped at 4096 tableaus (a beam never empties it) | 1.00 |
| all | every agent, start of item against the end (docs/speed.md's tier table) | 0.76 |

The engine, the strategist, `sweep` and the planner all read 0.76 × a game and a decision on the same games.

## Class 2: approximations, fresh seeds 33100-33299

The rule (Matt, 2026-11-04): the paired win rate, strategist at d2 and greedy (`engine`) at d0, each board
pair, not lower by more than 1 SE, and card keep and play rates keeping their rank order (Spearman against
the base, the bar `sweep` held: 0.985). Cells: `screen-103-*-cells.txt` here.

**The seat-terms cache by tableau alone** (`w.seat_cache`, specs in `experiments/archive/`): a seat's terms
kept by its piles in play and tracks, missing the production and alignment-bonus vars. 0.93 × a decision on
the smoke (16 seeds), 0.91-0.94 × a game in the screen.

| | bp | br | pr | pooled | keep ranks | play ranks |
|---|---|---|---|---|---|---|
| strategist d2 | +1.0 ±1.2 | +1.0 ±1.7 | **-2.0 ±1.7** | +0.0 ±0.9 | 0.999 | 0.997 |
| engine d0 | **-2.0 ±1.4** | +2.0 ±1.9 | +0.5 ±1.5 | +0.2 ±0.9 | 0.999 | 0.997 |

Two cells past 1 SE: not landed. Keyed by everything the four functions read, the same store is exact
and as fast (0.94), so it landed in class 1 instead (step 4).

**An answer looks one prompt on** (`engine:answers=1`, `screen-103-answers*.json`): an answer to an open
prompt (`Choose`, 37% of the strategist's scoring time) resolves one level of the prompts after it, not two.
0.72 × a decision on the smoke, 0.72-0.78 × a game in the screen.

| | bp | br | pr | pooled | keep ranks | play ranks |
|---|---|---|---|---|---|---|
| strategist d2 | +0.5 ±3.3 | +2.0 ±3.7 | **-6.0 ±3.8** | -1.2 ±2.1 | 0.997 | 0.987 |
| engine d0 | **-9.0 ±3.8** | **-4.0 ±3.5** | +0.0 ±3.9 | -4.3 ±2.2 | 0.996 | **0.979** |

Not landed: three cells past 1 SE and the engine's play ranks below the bar. `answers` stays an engine
parameter at 99 (no change), as the planner's is for rollouts.

## Not landed, by estimated payoff

1. **Typed slots for the hottest vars** (alignment buckets and tips, tracks, project cubes): the binary
   search by name is 15.6% self after 103; perhaps half of that. Needs a state-layout change (serde and
   records keep the names).
2. **Resuming a prompt's phase instead of replaying it** for each option: the rules' share under
   `settled` (43%) is mostly replays. A redesign of how prompts resolve; large and risky.
3. **Typed module parameters** instead of `World::int` by name: 3%.
4. **Prompt texts built only when asked** (`format!` in the rules, 3%): `Cx::choose` takes the text
   whether or not the answer is already made; a closure would skip it on replays.
5. **An incremental heuristic** (`Eval::value`'s own loops, 5.8% self): a redesign.
6. **Approximations:** none of the two screened passed; a coarser one (`choices=1`) would cost more wins.
