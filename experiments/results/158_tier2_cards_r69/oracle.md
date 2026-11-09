  bp d3 engine:strategy=1,w.clock=0 win  12.5%  rounds  9.88  agent   1584.1 ms/game
  bp d3 oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 win 100.0%  rounds  8.38  agent  18830.2 ms/game
  bp d4 engine:strategy=1,w.clock=0 win   0.0%  rounds  8.62  agent    804.3 ms/game
  bp d4 oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 win  37.5%  rounds  9.12  agent 357735.2 ms/game
  br d3 engine:strategy=1,w.clock=0 win  25.0%  rounds  9.62  agent   1816.9 ms/game
  br d3 oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 win 100.0%  rounds  8.62  agent  37620.3 ms/game
  br d4 engine:strategy=1,w.clock=0 win   0.0%  rounds  8.38  agent    736.5 ms/game
  br d4 oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 win  87.5%  rounds  8.62  agent 544919.4 ms/game
  pr d3 engine:strategy=1,w.clock=0 win  50.0%  rounds  9.12  agent   1651.2 ms/game
  pr d3 oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 win 100.0%  rounds  7.50  agent  21692.1 ms/game
  pr d4 engine:strategy=1,w.clock=0 win   0.0%  rounds  8.50  agent    792.2 ms/game
  pr d4 oracle:alts=32,budget=192,reseeds=64,answers=1,strategy=1 win  75.0%  rounds  8.38  agent 280232.9 ms/game

### d3 (paired against d3/oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1)

| group | games | win % | SE | diff | paired SE | ends |
|---|---|---|---|---|---|---|
| d3/oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1 | 24 | 100.0 | 0.0 | +0.0 | 0.0 | purge 24 |
| d3/strategist | 24 | 29.2 | 9.3 | -70.8 | 9.5 | deck_empty 16, purge 7, discord_deck 1 |

### d4 (paired against d4/oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1)

| group | games | win % | SE | diff | paired SE | ends |
|---|---|---|---|---|---|---|
| d4/oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1 | 24 | 66.7 | 9.6 | +0.0 | 0.0 | purge 16, discord_deck 5, deck_empty 3 |
| d4/strategist | 24 | 0.0 | 0.0 | -66.7 | 9.8 | discord_deck 19, deck_empty 5 |
