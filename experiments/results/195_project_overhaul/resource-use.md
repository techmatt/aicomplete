New out/r85-strat-final (1728 games), old out/r84-strat (1728 games). Per seat a game unless marked.

## Spent a game, and the share of seat-rounds it ran out

| resource | spent new | spent old | ran out new | ran out old |
|---|---|---|---|---|
| credits | 244.5 | 252.3 | 31% | 29% |
| insight | 35.2 | 37.1 | 2% | 1% |
| energy | 26.3 | 27.8 | 40% | 40% |
| chips | 1.8 | 2.1 | 93% | 93% |
| antimatter | 0.2 | 0.2 | 77% | 76% |
| research_cubes | 2.6 | 3.1 | 86% | 85% |
| ambassadors | 16.7 | 16.9 | 48% | 49% |

## What each resource buys (per seat a game)

| resource | use | new | old |
|---|---|---|---|
| credits | play | 178.89 | 182.02 |
| credits | standard | 47.71 | 49.94 |
| credits | power | 11.88 | 12.71 |
| credits | pay | 4.60 | 5.42 |
| credits | phase | 1.05 | 1.93 |
| credits | use | 0.21 | 0.18 |
| insight | keep | 19.09 | 20.35 |
| insight | play | 11.93 | 12.09 |
| insight | pay | 3.27 | 3.69 |
| insight | phase | 0.75 | 0.79 |
| insight | power | 0.16 | 0.15 |
| energy | power | 19.25 | 20.05 |
| energy | pay | 3.11 | 3.39 |
| energy | play | 3.10 | 3.07 |
| energy | convert | 0.47 | 0.53 |
| energy | phase | 0.26 | 0.54 |
| energy | use | 0.14 | 0.18 |
| chips | power | 1.01 | 1.17 |
| chips | standard | 0.81 | 0.86 |
| antimatter | play | 0.15 | 0.16 |
| research_cubes | standard | 0.75 | 0.85 |
| research_cubes | convert | 0.75 | 0.84 |
| research_cubes | pay | 0.59 | 0.81 |
| research_cubes | power | 0.37 | 0.44 |
| research_cubes | play | 0.10 | 0.14 |
| card_tokens | play | 0.76 | 0.83 |
| ambassadors | standard | 16.12 | 16.30 |
| ambassadors | play | 0.59 | 0.54 |

## Insight: keeps against plays (per game, all seats)

| measure | new | old |
|---|---|---|
| cards drafted (offered) | 58.56 | 61.64 |
| cards kept | 32.06 | 34.21 |
| cards played | 38.50 | 38.89 |
| insight carried (Patience) | 51.99 | 58.47 |
| insight left after keeps | 132.08 | 142.94 |
| insight to keeps | 38.18 | 40.70 |
| insight to plays | 24.94 | 25.21 |
| insight unspent at round end | 97.08 | 107.93 |
| rounds | 7.54 | 7.93 |
| tier-2 chosen in hand-building | 40.24 | 42.35 |

## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)

`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:
`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).
`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource
(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.

| counter | new | old |
|---|---|---|
| limit.keep_passed | 13.15 | 13.60 |
| limit.keep_short.insight | 0.10 | 0.11 |
| limit.play.held | 5.87 | 6.64 |
| limit.play.needs_tags | 22.20 | 22.27 |
| limit.play.short.antimatter | 0.54 | 0.61 |
| limit.play.short.chips | 0.06 | 0.05 |
| limit.play.short.credits | 94.62 | 94.77 |
| limit.play.short.energy | 6.42 | 6.53 |
| limit.play.short.insight | 2.91 | 2.81 |
| limit.play.short.research_cubes | 0.43 | 0.50 |
