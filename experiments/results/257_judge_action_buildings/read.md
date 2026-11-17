# 257 fresh read (49212-49259, three pairs, d0-d4, 48 seeds a cell; 720 games an arm)

The shipped judge (`strategist:w.sink=0`: r117's judge to the bit, checked on 256's seeds 49020-49025 at d2, every
game's action count equal to 256's read) against the new one (`strategist`, `w.sink` 2), both farmed from 228e919
(`experiments/farm/f257-read.json`). `bench/paired.py --against-agent strategist:w.sink=0`.

| slice | n | shipped | new | gap ± SE | rounds (shipped → new) |
|---|---|---|---|---|---|
| d0 | 144 | 87.5 | 93.1 | +5.6 ± 2.6 | 9.78 → 9.66 |
| d1 | 144 | 90.3 | 92.4 | +2.1 ± 2.5 | 9.36 → 9.39 |
| d2 | 144 | 90.3 | 89.6 | -0.7 ± 2.5 | 9.09 → 9.15 |
| d3 | 144 | 59.0 | 57.6 | -1.4 ± 2.6 | 8.86 → 8.91 |
| d4 | 144 | 27.8 | 25.0 | -2.8 ± 2.6 | 8.01 → 8.09 |
| bp | 240 | 71.2 | 72.9 | +1.7 ± 2.0 | 8.99 → 9.01 |
| br | 240 | 69.2 | 68.8 | -0.4 ± 2.0 | 8.88 → 8.90 |
| pr | 240 | 72.5 | 72.9 | +0.4 ± 2.0 | 9.20 → 9.21 |
| all | 720 | 71.0 | 71.5 | +0.6 ± 1.1 | 9.02 → 9.04 |

## Cells (win %, ends)

| cell | shipped | new |
|---|---|---|
| bp d0 | 90 (purge 43, deck_empty 5) | 90 (purge 43, deck_empty 5) |
| bp d1 | 88 (purge 42, deck_empty 5, node 1) | 92 (purge 44, node 2, deck_empty 2) |
| bp d2 | 92 (purge 44, node 3, discord_deck 1) | 92 (purge 44, node 3, deck_empty 1) |
| bp d3 | 56 (purge 27, node 11, deck_empty 7, discord_deck 3) | 58 (purge 28, node 11, deck_empty 6, discord_deck 3) |
| bp d4 | 31 (node 27, purge 15, deck_empty 4, discord_deck 2) | 33 (node 25, purge 16, deck_empty 5, discord_deck 2) |
| br d0 | 98 (purge 47, deck_empty 1) | 100 (purge 48) |
| br d1 | 88 (purge 42, deck_empty 4, node 2) | 94 (purge 45, deck_empty 2, node 1) |
| br d2 | 88 (purge 42, deck_empty 4, node 2) | 83 (purge 40, deck_empty 5, node 3) |
| br d3 | 52 (purge 25, node 16, deck_empty 6, discord_deck 1) | 50 (purge 24, node 12, deck_empty 11, discord_deck 1) |
| br d4 | 21 (node 32, purge 10, deck_empty 3, discord_deck 3) | 17 (node 35, purge 8, deck_empty 3, discord_deck 2) |
| pr d0 | 75 (purge 36, deck_empty 12) | 90 (purge 43, deck_empty 5) |
| pr d1 | 96 (purge 46, deck_empty 1, node 1) | 92 (purge 44, deck_empty 3, node 1) |
| pr d2 | 92 (purge 44, deck_empty 3, node 1) | 94 (purge 45, deck_empty 2, node 1) |
| pr d3 | 69 (purge 33, deck_empty 9, node 6) | 65 (purge 31, deck_empty 11, node 6) |
| pr d4 | 31 (node 24, purge 15, deck_empty 6, discord_deck 3) | 25 (node 26, purge 12, deck_empty 6, discord_deck 4) |

Use: [read-use.md](read-use.md) (`use257.py`). design_check, new against shipped: [design-check.txt](design-check.txt).
