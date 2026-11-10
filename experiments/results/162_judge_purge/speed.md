# 162: part 1's speed, one thread each, old (main) and new side by side, quiet box (7:16pm), countries 2, d2

```
seed 1000: old 2664 ms, new 1666 ms, 1.60x, decisions 470
seed 1001: old 5280 ms, new 2317 ms, 2.28x, decisions 624
seed 1002: old 648 ms, new 586 ms, 1.11x, decisions 449
seed 1003: old 2314 ms, new 1526 ms, 1.52x, decisions 501
seed 1004: old 4732 ms, new 2976 ms, 1.59x, decisions 691
seed 1005: old 2928 ms, new 2101 ms, 1.39x, decisions 602
seed 1006: old 2409 ms, new 1587 ms, 1.52x, decisions 384
seed 1007: old 7312 ms, new 2460 ms, 2.97x, decisions 487
greedy: 8 games, old 28.3 s, new 15.2 s, 1.86x

seed 1000: old 810 ms, new 805 ms, 1.01x, decisions 577
seed 1001: old 1707 ms, new 1654 ms, 1.03x, decisions 697
seed 1002: old 948 ms, new 875 ms, 1.08x, decisions 596
seed 1003: old 670 ms, new 575 ms, 1.17x, decisions 475
seed 1004: old 1231 ms, new 1185 ms, 1.04x, decisions 691
seed 1005: old 346 ms, new 290 ms, 1.19x, decisions 385
seed 1006: old 13042 ms, new 11319 ms, 1.15x, decisions 872
seed 1007: old 2003 ms, new 1603 ms, 1.25x, decisions 741
strategist: 8 games, old 20.8 s, new 18.3 s, 1.13x
```

First step only (the Seen extension, before Seen::chains), under load: greedy 1.25x (6 games).
