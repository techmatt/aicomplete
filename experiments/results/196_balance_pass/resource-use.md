New out/fn-s (1728 games), old out/fresh-old-s (1728 games). Per seat a game unless marked.

## Spent a game, and the share of seat-rounds it ran out

| resource | spent new | spent old | ran out new | ran out old |
|---|---|---|---|---|
| credits | 245.4 | 249.1 | 30% | 31% |
| insight | 36.4 | 35.5 | 7% | 1% |
| energy | 27.7 | 26.5 | 35% | 39% |
| chips | 2.8 | 2.0 | 86% | 91% |
| antimatter | 0.1 | 0.1 | 75% | 76% |
| research_cubes | 3.8 | 3.1 | 78% | 83% |
| ambassadors | 16.6 | 16.9 | 50% | 48% |

## What each resource buys (per seat a game)

| resource | use | new | old |
|---|---|---|---|
| credits | play | 175.05 | 181.83 |
| credits | standard | 51.17 | 48.89 |
| credits | power | 12.77 | 11.84 |
| credits | pay | 4.80 | 5.21 |
| credits | phase | 1.38 | 1.05 |
| credits | use | 0.14 | 0.19 |
| insight | keep | 20.00 | 19.24 |
| insight | play | 11.57 | 12.10 |
| insight | pay | 3.79 | 3.29 |
| insight | phase | 0.85 | 0.73 |
| insight | power | 0.22 | 0.16 |
| energy | power | 19.85 | 19.35 |
| energy | pay | 3.48 | 2.92 |
| energy | play | 3.15 | 3.19 |
| energy | convert | 0.69 | 0.59 |
| energy | phase | 0.33 | 0.27 |
| energy | use | 0.18 | 0.20 |
| chips | power | 1.44 | 1.07 |
| chips | standard | 1.34 | 0.90 |
| antimatter | play | 0.15 | 0.15 |
| research_cubes | standard | 1.12 | 0.88 |
| research_cubes | convert | 1.00 | 0.88 |
| research_cubes | pay | 0.83 | 0.72 |
| research_cubes | power | 0.69 | 0.45 |
| research_cubes | play | 0.17 | 0.15 |
| card_tokens | play | 0.83 | 0.93 |
| ambassadors | standard | 16.00 | 16.30 |
| ambassadors | play | 0.57 | 0.57 |

## Insight: keeps against plays (per game, all seats)

| measure | new | old |
|---|---|---|
| cards drafted (offered) | 61.94 | 58.64 |
| cards kept | 33.57 | 32.15 |
| cards played | 37.84 | 39.34 |
| insight carried (Patience) | 66.35 | 55.13 |
| insight left after keeps | 153.26 | 135.61 |
| insight to keeps | 40.01 | 38.47 |
| insight to plays | 24.24 | 25.29 |
| insight unspent at round end | 118.20 | 101.90 |
| rounds | 8.03 | 7.58 |
| tier-2 chosen in hand-building | 41.07 | 40.29 |

## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)

`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:
`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).
`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource
(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.

| counter | new | old |
|---|---|---|
| limit.keep_passed | 13.14 | 13.15 |
| limit.keep_short.insight | 1.05 | 0.10 |
| limit.play.held | 6.72 | 6.85 |
| limit.play.needs_tags | 25.41 | 22.59 |
| limit.play.short.antimatter | 0.55 | 0.49 |
| limit.play.short.chips | 0.05 | 0.06 |
| limit.play.short.credits | 101.43 | 98.42 |
| limit.play.short.energy | 6.41 | 6.44 |
| limit.play.short.insight | 2.97 | 2.67 |
| limit.play.short.research_cubes | 0.52 | 0.53 |
