New out/fresh-new-s (2160 games), old out/fresh-base-s (2160 games). Per seat a game unless marked.

## Spent a game, and the share of seat-rounds it ran out

| resource | spent new | spent old | ran out new | ran out old |
|---|---|---|---|---|
| credits | 249.8 | 239.3 | 29% | 29% |
| insight | 37.6 | 36.2 | 2% | 2% |
| energy | 27.5 | 26.6 | 45% | 46% |
| chips | 2.1 | 2.0 | 93% | 93% |
| antimatter | 0.1 | 0.1 | 77% | 78% |
| research_cubes | 3.1 | 3.0 | 85% | 86% |
| ambassadors | 17.4 | 17.1 | 49% | 47% |

## What each resource buys (per seat a game)

| resource | use | new | old |
|---|---|---|---|
| credits | play | 179.04 | 167.66 |
| credits | standard | 50.37 | 51.47 |
| credits | power | 12.77 | 12.68 |
| credits | pay | 4.88 | 4.26 |
| credits | phase | 2.37 | 2.84 |
| credits | use | 0.21 | 0.18 |
| insight | keep | 20.34 | 19.82 |
| insight | play | 11.94 | 11.16 |
| insight | pay | 4.49 | 4.47 |
| insight | phase | 0.71 | 0.59 |
| insight | power | 0.15 | 0.14 |
| energy | power | 19.82 | 19.23 |
| energy | pay | 3.63 | 3.59 |
| energy | play | 2.85 | 2.60 |
| energy | phase | 0.58 | 0.64 |
| energy | convert | 0.46 | 0.36 |
| energy | use | 0.19 | 0.17 |
| chips | power | 1.21 | 1.17 |
| chips | standard | 0.87 | 0.79 |
| antimatter | play | 0.14 | 0.12 |
| research_cubes | standard | 0.88 | 0.95 |
| research_cubes | pay | 0.82 | 0.73 |
| research_cubes | convert | 0.81 | 0.74 |
| research_cubes | power | 0.46 | 0.41 |
| research_cubes | play | 0.10 | 0.10 |
| card_tokens | play | 0.81 | 0.68 |
| ambassadors | standard | 16.78 | 16.49 |
| ambassadors | play | 0.55 | 0.47 |

## Insight: keeps against plays (per game, all seats)

| measure | new | old |
|---|---|---|
| cards drafted (offered) | 61.52 | 60.39 |
| cards kept | 33.97 | 32.94 |
| cards played | 38.53 | 35.69 |
| insight carried (Patience) | 57.41 | 55.00 |
| insight left after keeps | 140.61 | 131.85 |
| insight to keeps | 40.69 | 39.63 |
| insight to plays | 24.94 | 23.29 |
| insight unspent at round end | 104.75 | 98.42 |
| rounds | 7.95 | 7.81 |
| tier-2 chosen in hand-building | 41.99 | 40.56 |

## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)

`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:
`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).
`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource
(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.

| counter | new | old |
|---|---|---|
| limit.keep_passed | 13.67 | 13.64 |
| limit.keep_short.insight | 0.10 | 0.09 |
| limit.play.held | 6.32 | 5.32 |
| limit.play.needs_tags | 21.99 | 21.33 |
| limit.play.short.antimatter | 0.57 | 0.55 |
| limit.play.short.chips | 0.06 | 0.06 |
| limit.play.short.credits | 91.54 | 86.64 |
| limit.play.short.energy | 6.57 | 6.19 |
| limit.play.short.insight | 2.84 | 2.79 |
| limit.play.short.research_cubes | 0.50 | 0.43 |
