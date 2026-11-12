New out/ladder/r83-strategist (2160 games), old out/ladder/r82-strategist (2160 games). Per seat a game unless marked.

## Spent a game, and the share of seat-rounds it ran out

| resource | spent new | spent old | ran out new | ran out old |
|---|---|---|---|---|
| credits | 229.3 | 231.2 | 28% | 28% |
| insight | 38.7 | 38.8 | 5% | 4% |
| energy | 24.7 | 24.9 | 43% | 44% |
| chips | 2.0 | 4.2 | 92% | 66% |
| antimatter | 0.1 | 0.1 | 80% | 80% |
| research_cubes | 3.0 | 3.4 | 88% | 88% |
| ambassadors | 17.3 | 17.8 | 46% | 45% |

## What each resource buys (per seat a game)

| resource | use | new | old |
|---|---|---|---|
| credits | play | 156.05 | 156.45 |
| credits | standard | 53.22 | 53.99 |
| credits | power | 12.85 | 13.11 |
| credits | pay | 4.39 | 4.71 |
| credits | phase | 2.49 | 2.70 |
| credits | use | 0.14 | 0.15 |
| insight | keep | 18.87 | 18.84 |
| insight | play | 15.89 | 16.00 |
| insight | pay | 3.41 | 3.32 |
| insight | phase | 0.43 | 0.49 |
| insight | power | 0.14 | 0.12 |
| energy | power | 18.51 | 18.54 |
| energy | pay | 3.16 | 3.19 |
| energy | play | 2.14 | 2.13 |
| energy | phase | 0.54 | 0.59 |
| energy | convert | 0.29 | 0.32 |
| chips | power | 1.14 | 1.39 |
| chips | standard | 0.80 | 2.67 |
| antimatter | play | 0.11 | 0.11 |
| research_cubes | standard | 1.00 | 1.20 |
| research_cubes | convert | 0.78 | 0.82 |
| research_cubes | pay | 0.60 | 0.69 |
| research_cubes | power | 0.46 | 0.55 |
| research_cubes | play | 0.10 | 0.13 |
| card_tokens | play | 0.59 | 0.54 |
| ambassadors | standard | 16.83 | 17.28 |
| ambassadors | play | 0.38 | 0.37 |
| ambassadors | phase | 0.08 | 0.11 |

## Insight: keeps against plays (per game, all seats)

| measure | new | old |
|---|---|---|
| cards drafted (offered) | 57.79 | 57.34 |
| cards kept | 30.82 | 30.72 |
| cards played | 34.75 | 34.90 |
| insight carried (Patience) | 39.74 | 37.73 |
| insight left after keeps | 112.51 | 109.52 |
| insight to keeps | 37.74 | 37.69 |
| insight to plays | 33.10 | 33.34 |
| insight unspent at round end | 73.82 | 71.37 |
| rounds | 7.44 | 7.37 |
| tier-2 chosen in hand-building | 39.32 | 38.87 |

## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)

`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:
`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).
`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource
(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.

| counter | new | old |
|---|---|---|
| limit.keep_passed | 13.38 | 13.20 |
| limit.keep_short.insight | 0.10 | 0.11 |
| limit.play.held | 6.02 | 5.30 |
| limit.play.needs_tags | 18.87 | 17.96 |
| limit.play.short.antimatter | 0.55 | 0.47 |
| limit.play.short.chips | 0.02 | 0.02 |
| limit.play.short.credits | 73.92 | 71.55 |
| limit.play.short.energy | 5.42 | 5.36 |
| limit.play.short.insight | 9.75 | 9.65 |
| limit.play.short.research_cubes | 0.46 | 0.43 |
