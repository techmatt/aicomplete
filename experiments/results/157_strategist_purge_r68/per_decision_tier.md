# 157: greedy with the tier read on (new) and off (old), 6 games from seed 1000, d2, one thread

## r67 (156's binary)

| | rules | wins | rounds | decisions | options/decision | ms/game | ms/decision |
|---|---|---|---|---|---|---|---|
| old | r67 | 4 | 8.67 | 516 | 26.82 | 15089 | 29.3 |
| new | r67 | 4 | 9.00 | 512 | 26.07 | 23934 | 46.8 |
| new / old | | | 1.04 | 0.99 | 0.97 | 1.59 | 1.60 |

## r68

| | rules | wins | rounds | decisions | options/decision | ms/game | ms/decision |
|---|---|---|---|---|---|---|---|
| old | r68 | 5 | 8.50 | 526 | 28.63 | 6311 | 12.0 |
| new | r68 | 4 | 8.17 | 473 | 23.88 | 4185 | 8.9 |
| new / old | | | 0.96 | 0.90 | 0.83 | 0.66 | 0.74 |

## Profile split (`--features profile`, d2, one game each of greedy and greedy:w.tier_read=0, r68)

Seed 1000:

```
search (of agents + probes): apply 68.5% (184666 calls, 50.20 us each), apply_mut 0.0% (0 calls, 0.00 us each), clone 1.7% (184666 calls, 1.27 us each), legal 0.1% (18934 calls, 0.45 us each), eval 26.4% (165853 calls, 21.52 us each), determinize 0.1% (596 calls, 16.14 us each), line 0.0% (0 calls, 0.00 us each), sync 0.0% (0 calls, 0.00 us each), policy 0.2% (115 calls, 205.89 us each), top 0.0% (0 calls, 0.00 us each), extend 0.0% (0 calls, 0.00 us each), score_power 1.3% (881 calls, 204.25 us each), score_play 3.5% (3197 calls, 147.99 us each), score_standard 5.3% (1945 calls, 365.42 us each), score_choose 86.2% (2051 calls, 5688.31 us each), score_other 3.4% (16179 calls, 28.20 us each)
  d2 greedy                   win   0.0%  rounds 11.00  agent   8157.4 ms/game
  d2 greedy:w.tier_read=0     win 100.0%  rounds  9.00  agent   5378.1 ms/game
```

Seed 1001:

```
search (of agents + probes): apply 60.8% (320040 calls, 37.40 us each), apply_mut 0.0% (0 calls, 0.00 us each), clone 2.0% (320040 calls, 1.25 us each), legal 0.1% (23314 calls, 0.64 us each), eval 33.1% (296940 calls, 21.99 us each), determinize 0.1% (577 calls, 17.21 us each), line 0.0% (0 calls, 0.00 us each), sync 0.0% (0 calls, 0.00 us each), policy 0.1% (111 calls, 179.42 us each), top 0.0% (0 calls, 0.00 us each), extend 0.0% (0 calls, 0.00 us each), score_power 1.5% (1712 calls, 176.66 us each), score_play 3.4% (5288 calls, 125.26 us each), score_standard 1.8% (2151 calls, 163.77 us each), score_choose 91.1% (2509 calls, 7154.76 us each), score_other 2.0% (16763 calls, 23.07 us each)
  d2 greedy                   win 100.0%  rounds  8.00  agent   6574.4 ms/game
  d2 greedy:w.tier_read=0     win 100.0%  rounds  9.00  agent  13127.4 ms/game
```
