# 162: a judge that plays the purge at least as well as greedy (r69)

No rules, content, prices or adversary dials changed: `RULES_REVISION` stays 69. Sim player code only.

Scripts: [specs162.py](specs162.py) (the farm specs `s162-screen`, `s162-read`, and the oracle row's
[s162-oracle.json](s162-oracle.json); `s162-retune` and `s162-d3` in `experiments/farm/` were cut by hand from
`s162-screen` and `s162-read`, `s162-oracle-d4.json` from the oracle's), [judges162.py](judges162.py) (agents against one another in one run, paired on
pair and seed, across engines: Matt, message 7325), [speed162.py](speed162.py) (two binaries side by side, one thread
each, game by game), [cost162.py](cost162.py) (a farmed run's s a game by agent, from the main box's shard log).
Tables: [screen.md](screen.md), [final.md](final.md), [cost.md](cost.md), [speed.md](speed.md).

## Part 1: the flat search, faster, the same games

**Where greedy's time went** (a throwaway tally of scorings by prompt text, greedy, d2, seeds 1000-1001): chains of
"discard which card (k of n)?" (threats.execute: Blackout, Manhunt, Raiding Parties, Commandeer; the adversary's
Purge the Unaligned), **64-88% of a greedy game**. Greedy keeps big hands (16-37 cards), and every order of the
same discards was scored again: one Manhunt decision, 37 options, 48k applies and 47k evaluations.

**The change** (`agents/search.rs`, exact by construction):
- `Seen`, the transposition memo the loss prompts had, takes discard prompts too, and a position matching but
  for a discard pile's order. Only the reshuffle (`cards::deck::top_of`) reads that order, on the `cards`
  stream, so such a match is used only when no step under the stored value moved that stream (`dirty`).
- `Seen::chains`: inside a chain, the value of a pick is keyed before any replay by the chain's cards as a set
  (with the snapshot, the trigger and the answers before the chain), under the same clean-stream rule.
- One const writes and reads the prompt (`cards::DISCARD_PROMPT`), with a note to its sites that nothing may draw
  or ask between a chain's picks. A read-only review found no way for either memo to change a score.

**Same games:** `same_results.py` on 16 seeds a pair (1000-1015), the three pairs, d2, greedy and the strategist
(157's), the change against a clean build of main: **96 of 96 the same**, both steps.

**Speed at equal threads** (one thread each, old and new side by side on a quiet box, 8 games from seed 1000,
countries 2, d2; [speed.md](speed.md)): **greedy 1.86×** a game (1.11-2.97× by game), **the strategist 1.13×**
(157's judge; 1.01-1.25×). Short of the 2× aim for greedy; what is left in a discard chain is the replay itself
(`apply_to` replaying threats.execute from its snapshot for each leaf).

## Part 2: the judge

**157's diagnosis, re-read at r69** (the screen's old judge, dev seeds): it buys 0.7-0.8 ambassadors a game to
greedy's 1.5-1.9 and frees 2.7-3.0 cities to greedy's 4.3-5.1. The prices hold: in horizon mode a supply
ambassador is the table's row, 0.6 × 4 = 2.4, against 3.6 for the 6 credits that buy one.

**(a) The purge term** (`w.purge`, any mode, 0 off; `eval.rs`): each supply ambassador of a seat, up to the
Liberate bill (`board::liberate_cost`) of the corrupted cities in its reach, counts `w.purge` more, and each city
freed counts `w.purge` × `liberate_at` (5), so spending a seat's ambassadors on a Liberate costs nothing in the
term. Screened at 3.6: an ambassador toward the bill at 2.4 + 3.6 = 6, a city's 30 over its 5-ambassador bill.
**(b)** greedy's flat weights with strategy (157's `FLAT`).

**The screen** (dev seeds 36800-36831, 96 games a cell, the factorial with greedy beside it; [screen.md](screen.md)):

| Cell | d1 | d2 | d3 | d1-d3 gap to greedy (SE) | Ambassadors bought, d2 | Cities freed, d2 |
|---|---|---|---|---|---|---|
| old (157's judge) | 77.1 | 57.3 | 47.9 | -14.6 (4.7) / -19.8 (6.6) / -18.8 (6.5) | 0.68 | 2.97 |
| (a) old + purge | 80.2 | 63.5 | 59.4 | -11.5 / -13.5 / -7.3 | 0.80 | 3.53 |
| (b) flat weights | 89.6 | 76.0 | 66.7 | -2.1 (3.9) / -1.0 (5.4) / +0.0 (5.7) | 1.89 | 4.83 |
| **(a) on (b)** | **96.9** | **77.1** | **71.9** | **+5.2 (3.4) / +0.0 (5.3) / +5.2 (5.4)** | 3.21 | 5.46 |
| greedy | 91.7 | 77.1 | 66.7 | | 1.94 | 5.10 |

Each change on and off: (a) on the old judge +3.1 (5.2) / +6.2 (5.1) / +11.5 (5.7); (a) on (b) +7.3 (3.7) / +1.0
(5.4) / +5.2 (5.8); (b) on the old judge +12.5 / +18.8 / +18.8. The purge term helps in both modes, the flat
weights most. **Adopted: (a) on (b) as `strategist`** (`STRATEGIST` in `agents/mod.rs`): the only cell at or above
greedy at every level on the screen, and it does it the way the diagnosis asked, buying ambassadors (2.5-3.2 a
game) and freeing cities (4.2-5.5). Cost: about 1.6× the old judge a game (final read: 2.3-6.1 s against 1.5-3.6
on the main box under load), about greedy's.

**The final read** (fresh 40264-40359, two 48-seed sets pooled, 288 games a level; [final.md](final.md)):

| Level | new | old | greedy | new − greedy (SE) | verdict | new − old (SE) | sets (new) |
|---|---|---|---|---|---|---|---|
| d0 | 98.3 | 96.2 | 98.6 | -0.3 (0.9) | (not a target) | +2.1 (1.4) | 99.3 / 97.2 |
| d1 | 93.1 | 79.5 | 88.2 | **+4.9 (2.3)** | met | +13.5 (2.8) | 91.7 / 94.4 |
| d2 | 80.2 | 65.6 | 78.8 | **+1.4 (3.0)** | met | +14.6 (3.4) | 81.2 / 79.2 |
| d3 | 65.6 | 51.0 | 66.7 | **-1.0 (3.2)** | **unmet** (0.3 SE) | +14.6 (3.6) | 66.0 / 65.3 |

**The bands move, for the recalibration item** (no dial changed here; 157's bands 90-100 / 70-80 / 55-65 / 40-50):
the old judge reads 96.2 / 79.5 / 65.6 / 51.0 on these seeds (d0 and d1 in band, d2 0.6 over, d3 1.0 over), the
new judge 98.3 / 93.1 / 80.2 / 65.6: every level above its band, d1-d3 by 13-16 points. Greedy, the new player,
reads 98.6 / 88.2 / 78.8 / 66.7. The new judge's lead over greedy, the depth measure, is +4.9 / +1.4 / -1.0.

**The oracle row** (local, d3 and d4, 40264-40271, 24 games a level): d3 **100** (the new judge 75.0, the old 54.2,
greedy 83.3 on the same games), d4 **83.3** (20 of 24; r69's 158 read 66.7 on its own seeds).

**The time gate** (1h10m left after the read): a retune screen at d3 on the dev seeds, `w.purge` 5 and 2.5 against
3.6 (paired with the screen's cells): +1.0 (5.0) and -1.0 (4.8). No lever: 3.6 stays, no confirm read. Then, as
evidence on the unmet d3 (not the verdict, which is the two sets): a **third fresh d3 set** (40360-40407, the
strategist and greedy, 144 games each): 69.4 against 74.3, -4.9 (4.2); **pooled over the three sets -2.3 (2.5)**,
432 games. The new judge sits about 2 points under greedy at d3, inside one SE: its purge lead at d1 does not
reach d3, where it liberates less (6.2-6.3 a game against greedy's 7.0-7.1) while freeing at least as many cities.

## Seeds

Dev seeds 36800-36831 for the screens; **40264-40359** for the final read (two 48-seed sets, 40264-40311 and
40312-40359); the oracle row on 40264-40271 (d3 and d4, the read's first 8 seeds a pair); the third d3 set on
40360-40407. Profiling and part 1's checks on 1000-1015 (not ledger seeds). Ledger: next free 40408.
