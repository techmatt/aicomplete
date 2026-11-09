# 157: the oracle row beside the final read (skilled_rows.py, seeds 39304-39311, 8 a pair, d3 and d4, the probe on every loss)

| Agent | Level | Games | Win % | SE | agent s/game |
|---|---|---|---|---|---|
| engine:strategy=1,w.clock=0 | d3 | 24 | 29.2 | 9.3 | 1.7 |
| engine:strategy=1,w.clock=0 | d4 | 24 | 0.0 | 0.0 | 0.8 |
| oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 | d3 | 24 | 100.0 | 0.0 | 47.6 |
| oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 | d4 | 24 | 37.5 | 9.9 | 394.7 |


### Read

| Cell | Games | Win % | SE | Purge at step 0 / 1 / 2 | Unity-deck wins | Round cap | Discord deck | Nodes | Other | Unity tips at end | Cities liberated | Unity tiers I / II / III a game |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ('engine:strategy=1,w.clock=0', 'd3') | 24 | 29.2 | 9.3 | 0 / 0 / 7 | 0 | 16 | 1 | 0 | 0 | 5.3 | 1.08 | 1.8 / 2.2 / 1.3 |
| ('engine:strategy=1,w.clock=0', 'd4') | 24 | 0.0 | 0.0 | 0 / 0 / 0 | 0 | 2 | 22 | 0 | 0 | 1.6 | 0.42 | 1.1 / 0.3 / 0.2 |
| ('oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1', 'd3') | 24 | 100.0 | 0.0 | 0 / 0 / 24 | 0 | 0 | 0 | 0 | 0 | 10.0 | 3.21 | 2.0 / 3.0 / 5.0 |
| ('oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1', 'd4') | 24 | 37.5 | 9.9 | 0 / 0 / 9 | 0 | 3 | 11 | 1 | 0 | 6.8 | 2.08 | 2.0 / 2.4 / 2.4 |

## The oracle at d4 on r67 (156's binary and configs), same seeds

| Agent | Level | Games | Win % | SE | agent s/game |
|---|---|---|---|---|---|
| oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 | d4 | 24 | 54.2 | 10.2 | 218.4 |

| Cell | Games | Win % | SE | Purge at step 0 / 1 / 2 | Unity-deck wins | Round cap | Discord deck | Nodes | Other | Unity tips at end | Cities liberated | Unity tiers I / II / III a game |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ('oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1', 'd4') | 24 | 54.2 | 10.2 | 0 / 0 / 13 | 0 | 1 | 10 | 0 | 0 | 8.3 | 2.29 | 3.0 / 2.2 / 3.0 |
