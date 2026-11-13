# 206 dials (strategist, compute_income)

Screened on the spent fresh seeds 45631-45726 (base: f206-strategist d1 81.9, d3 36.8):

| read | lever | d | win % | rounds | troops removed | nodes lost | lost to nodes % | gap vs base (paired) |
|---|---|---|---|---|---|---|---|---|
| d1 | k12 | d1 |  80.6 ±2.3 | 7.67 | 4.75 | 7.36 |  5.2 |  |
| d1 | k16 | d1 |  64.6 ±2.8 | 6.99 | 5.25 | 7.42 | 21.2 |  |
| d3 | k21 | d3 |  45.1 ±2.9 | 6.65 | 6.06 | 9.23 | 27.4 |  |
| d3 | k18 | d3 |  44.8 ±2.9 | 6.70 | 5.91 | 8.95 | 27.4 |  |

Confirm on 45727-45822 (d1 13, d3 21), r91 against r90 on the same seeds:

| read | lever | d | win % | rounds | troops removed | nodes lost | lost to nodes % | gap vs base (paired) |
|---|---|---|---|---|---|---|---|---|
| r91 | base | d1 |  79.2 ±2.4 | 7.56 | 4.94 | 7.22 |  7.6 |  |
| r91 | base | d3 |  46.9 ±2.9 | 6.90 | 6.35 | 9.66 | 26.4 |  |
| r90 | base | d1 |  78.5 ±2.4 | 7.60 | 2.15 | 6.94 |  5.9 |  |
| r90 | base | d3 |  43.4 ±2.9 | 6.28 | 2.62 | 9.46 | 37.8 |  |

| d | r91 win % | ±SE | r90 win % | gap | ±SE (paired) | sets (r91) | band | verdict |
|---|---|---|---|---|---|---|---|---|
| d1 | **79.2** | 2.4 | 78.5 | +0.7 | 3.1 | 79.2 | 70-80 | met | (288 games)
| d3 | **46.9** | 2.9 | 43.4 | +3.5 | 3.6 | 46.9 | 40-50 | met | (288 games)
