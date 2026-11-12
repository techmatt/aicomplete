New out/new-strat-dir (2160 games), old out/old-strat (2160 games). Per seat a game unless marked.

## Spent a game, and the share of seat-rounds it ran out

| resource | spent new | spent old | ran out new | ran out old |
|---|---|---|---|---|
| credits | 241.5 | 229.5 | 30% | 27% |
| insight | 36.5 | 38.6 | 2% | 4% |
| energy | 27.4 | 24.8 | 46% | 44% |
| chips | 1.9 | 1.9 | 93% | 92% |
| antimatter | 0.1 | 0.1 | 77% | 80% |
| research_cubes | 3.1 | 3.1 | 86% | 88% |
| ambassadors | 17.5 | 18.0 | 46% | 45% |

## What each resource buys (per seat a game)

| resource | use | new | old |
|---|---|---|---|
| credits | play | 170.54 | 156.96 |
| credits | standard | 51.36 | 52.74 |
| credits | power | 12.72 | 12.62 |
| credits | pay | 3.86 | 4.52 |
| credits | phase | 2.62 | 2.37 |
| credits | use | 0.20 | 0.18 |
| insight | keep | 20.02 | 18.73 |
| insight | play | 11.17 | 16.04 |
| insight | pay | 4.48 | 3.19 |
| insight | phase | 0.72 | 0.51 |
| insight | power | 0.15 | 0.14 |
| energy | power | 19.76 | 18.45 |
| energy | pay | 3.72 | 3.14 |
| energy | play | 2.67 | 2.19 |
| energy | phase | 0.62 | 0.58 |
| energy | convert | 0.44 | 0.31 |
| energy | use | 0.17 | 0.11 |
| chips | power | 1.15 | 1.11 |
| chips | standard | 0.76 | 0.76 |
| antimatter | play | 0.15 | 0.12 |
| research_cubes | standard | 0.93 | 1.06 |
| research_cubes | convert | 0.80 | 0.81 |
| research_cubes | pay | 0.77 | 0.64 |
| research_cubes | power | 0.40 | 0.43 |
| research_cubes | play | 0.14 | 0.11 |
| card_tokens | play | 0.74 | 0.76 |
| ambassadors | standard | 16.91 | 17.45 |
| ambassadors | play | 0.47 | 0.44 |

## Insight: keeps against plays (per game, all seats)

| measure | new | old |
|---|---|---|
| cards drafted (offered) | 61.11 | 57.84 |
| cards kept | 33.41 | 30.77 |
| cards played | 36.13 | 34.90 |
| insight carried (Patience) | 54.71 | 39.02 |
| insight left after keeps | 133.01 | 111.98 |
| insight to keeps | 40.04 | 37.45 |
| insight to plays | 23.23 | 33.34 |
| insight unspent at round end | 100.65 | 74.38 |
| rounds | 7.91 | 7.45 |
| tier-2 chosen in hand-building | 41.13 | 38.94 |

## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)

`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:
`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).
`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource
(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.

| counter | new | old |
|---|---|---|
| limit.keep_passed | 13.76 | 13.45 |
| limit.keep_short.insight | 0.09 | 0.08 |
| limit.play.held | 5.26 | 6.37 |
| limit.play.needs_tags | 21.63 | 18.76 |
| limit.play.short.antimatter | 0.49 | 0.36 |
| limit.play.short.chips | 0.03 | 0.02 |
| limit.play.short.credits | 88.58 | 73.61 |
| limit.play.short.energy | 6.46 | 5.52 |
| limit.play.short.insight | 2.77 | 9.35 |
| limit.play.short.research_cubes | 0.42 | 0.38 |
