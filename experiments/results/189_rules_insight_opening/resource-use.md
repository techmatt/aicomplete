New out/r82L (2160 games), old out/r81L (2160 games). Per seat a game unless marked.

## Spent a game, and the share of seat-rounds it ran out

| resource | spent new | spent old | ran out new | ran out old |
|---|---|---|---|---|
| credits | 227.2 | 235.5 | 28% | 28% |
| insight | 38.1 | 37.0 | 4% | 3% |
| energy | 24.8 | 23.4 | 44% | 29% |
| chips | 4.3 | 4.8 | 64% | 60% |
| antimatter | 0.1 | 0.1 | 81% | 76% |
| research_cubes | 3.6 | 3.9 | 87% | 87% |
| ambassadors | 18.0 | 18.8 | 45% | 44% |

## What each resource buys (per seat a game)

| resource | use | new | old |
|---|---|---|---|
| credits | play | 153.06 | 164.08 |
| credits | standard | 53.83 | 50.98 |
| credits | power | 12.80 | 12.35 |
| credits | pay | 4.59 | 4.63 |
| credits | phase | 2.60 | 2.04 |
| credits | use | 0.19 | 0.12 |
| credits | keep | 0.00 | 1.15 |
| insight | keep | 18.62 | 16.47 |
| insight | play | 15.56 | 16.72 |
| insight | pay | 3.25 | 2.94 |
| insight | phase | 0.50 | 0.73 |
| insight | power | 0.13 | 0.18 |
| energy | power | 18.42 | 14.79 |
| energy | pay | 3.19 | 4.39 |
| energy | play | 2.17 | 3.00 |
| energy | phase | 0.58 | 0.52 |
| energy | convert | 0.37 | 0.51 |
| energy | use | 0.11 | 0.16 |
| chips | standard | 2.74 | 3.13 |
| chips | power | 1.44 | 1.55 |
| antimatter | play | 0.11 | 0.13 |
| research_cubes | standard | 1.26 | 1.30 |
| research_cubes | convert | 0.88 | 1.03 |
| research_cubes | pay | 0.76 | 0.81 |
| research_cubes | power | 0.57 | 0.55 |
| research_cubes | play | 0.13 | 0.13 |
| card_tokens | play | 0.76 | 0.74 |
| ambassadors | standard | 17.47 | 18.18 |
| ambassadors | play | 0.40 | 0.50 |

## Insight: keeps against plays (per game, all seats)

| measure | new | old |
|---|---|---|
| cards drafted (offered) | 56.79 | 48.59 |
| cards kept | 30.31 | 27.03 |
| cards played | 33.81 | 36.99 |
| insight carried (Patience) | 38.11 | 34.55 |
| insight left after keeps | 109.17 | 110.27 |
| insight to keeps | 37.24 | 32.94 |
| insight to plays | 32.37 | 34.71 |
| insight unspent at round end | 72.65 | 71.82 |
| rounds | 7.32 | 7.26 |
| tier-2 chosen in hand-building | 38.54 | 33.47 |

## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)

`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:
`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).
`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource
(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.

| counter | new | old |
|---|---|---|
| limit.keep_passed | 13.14 | 15.88 |
| limit.keep_short.insight | 0.09 | 0.00 |
| limit.keep_short.insight_bought | 0.00 | 0.15 |
| limit.play.held | 5.53 | 6.45 |
| limit.play.needs_tags | 17.98 | 13.18 |
| limit.play.short.antimatter | 0.44 | 0.17 |
| limit.play.short.chips | 0.02 | 0.03 |
| limit.play.short.credits | 70.92 | 77.57 |
| limit.play.short.energy | 5.42 | 3.83 |
| limit.play.short.insight | 8.84 | 8.84 |
| limit.play.short.research_cubes | 0.43 | 0.15 |
