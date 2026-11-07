# 141: speed at r58

Simulation made cheaper at r58 with play unchanged. No rules change and no revision bump: every game of the
standard read and of `same_play.py`'s matrix comes out the same.

## Before and after

The box was quiet (nothing else ran). Base is main e47fa7b's release binary (`out/base/`), and after is this
item's (`out/final/`). Both are release builds with the same flags, run at 6 threads.

| read | base | after | ratio |
|---|---|---|---|
| standard read wall (`read-139-r58-a` + `-b`, 432 strategist games) | 74 + 72 = 146 s | 34 + 54 = 88 s | 1.66x |
| summed game time (all workers) | 707.9 s | 499.2 s | 1.42x |
| ms a decision (432 games, 232,780 decisions) | 3.04 | 2.14 | 1.42x |
| slowest game (bp d0, seed 36589, 11 rounds) | 62.2 s | 19.0 s | 3.3x |
| `per_decision.py` strategist c2 d2, 6 games from 1000, one thread | 1.2 ms (554 ms/game) | 1.0 ms (494) | 0.89 |

An intermediate build with only the first two fixes read 38 + 60 = 98 s (1.49x) and 2.40 ms a decision.

**Where the gain is.** It is mostly in the long games. Typical d2 games are 11% cheaper a decision (the last
row). The read is 1.42x a decision because a few 10-12 round d0/d1 games were 30-47 ms a decision. Its wall
time gains more (1.66x) because each half-read used to wait on its slowest game: base half a summed 298 s
over 6 workers (50 s each) but took 74 s, because one game took 62 s.

## Profile (r58, `profile_win.py`)

Base, 4 games at d2 from seed 36584: `search::settled` 71%, `Eval::value` 54%, `Rules::apply_to` 36%;
self: binary search by name 18%, `memcpy` 9% (2.7% under `Eval::fact`), `Eval::fact` 6.7%,
`card_index` 4.2%. Under `ai::act`, Rush Build's `sacrifice_keeping` was 8.6%.

The slowest game, in a 5 s window 25 s in: `sacrifice_keeping` 70%, `ai::builds_after` 66%,
`free_building_exists` 58%, `tag_counts` 52% (`card_index` under it 26% self). Rush Build offers only the
sacrifices that leave a building to play free (099). The strategist replays the prompt for every option, and
each replay tested every source twice (once in `any`, once in `retain`). Each test copied the state and then
recounted the seat's tags for every card in hand (`requirements_met` read `required_tags` for each card).

After, 4 games at d1 from seed 36650: `settled` 72%, `Eval::value` 54%, `apply_to` 36%, `hand_worth` 11%,
`sacrifice_keeping` 4%; self: binary search 18%, `memcpy` 7%, mimalloc 5.5%, drops 3.4%, clone 2.8%,
`card_index` 2.0%.

## What was done

All four changes are exact: the same values in the same order, so the same decisions.

1. **Rush Build's test** (`cards/play.rs`, `ai.rs`, `cards/sacrifice.rs`):
   - `FreeTest` reads the seat's tags and prices once a hand, lazily, not once a card.
   - `free_building_exists` stops at the first building found.
   - `sacrifice_keeping` tests each source once.
   - `builds_after` removes the source's cards from the state itself and puts the same piles back, instead
     of copying the state. The state leaves bit for bit as it came, the same `Arc`s included.
   - The slowest game went 56 s → 23 s (the first two) → 17 s (one thread).
2. **`Content::card_index`** is now an open-addressed table on the interned id's (address, length), hashed
   by a multiply, instead of a `HashMap` with the byte-wise FNV hasher. The first of a repeated id still wins
   (ids are unique: the parser refuses a duplicate).
3. **`Eval::fact`** returns a reference. A `Fact` holds three `Worth`s of tag arrays, several hundred bytes,
   and it was copied out on every call.
4. **The heuristic's facts** are a `Vec` by card index, found through (2), instead of their own `HashMap`
   by address (10% self before).

## Same play

- **Standard read:** `same_results.py` base against after: all 144 + 288 games the same (also the
  1.49x build).
- **`same_play.py`:** `run` on base and on after, then `compare`: 71 summaries, 71 traces and 597 records,
  0 differences.
- **Test lanes:** `cargo test-all` green, clippy clean.

## Speed baseline

`speed.py check` gave no verdict (baseline r55, binary r58). `--update` from the after binary at r58: random
7.2, greedy 303.5, engine 402.2, planner 9,755.8 ms a game (one thread).

## Next payoffs, ranked

These are in `docs/speed.md`, *What is left*:

1. **The heuristic's per-seat terms** (`hand_worth` 11%, `mix_counts`, `hand_tags`, the store's key),
   rebuilt on every call: keep a hand's worth a decision long, as the tableau terms are.
2. **Replaying a prompt's phase for each option** (`apply_to` 36%): resuming from the resolved part is a
   redesign.
3. **Binary search by name** (18% self): typed slots for the hottest vars.
4. **Per-call vectors and copies in `Eval::value`** (about 18% across `memcpy`, allocation, drops).
5. **Rush Build in long games:** still 14-16 ms a decision in the slowest game, since a source's test prices
   and counts the hand after each removal.

Not taken: the `fast` profile (LTO, one codegen unit; 8% at 030). The prompt asked for the same binary
flags before and after.
