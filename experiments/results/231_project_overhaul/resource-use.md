New out/231_project_overhaul/f231b-new (1152 games), old out/231_project_overhaul/f231b-base (1152 games). Per seat a game unless marked.

## Spent a game, and the share of seat-rounds it ran out

| resource | spent new | spent old | ran out new | ran out old |
|---|---|---|---|---|
| credits | 351.6 | 314.1 | 41% | 41% |
| insight | 52.5 | 47.2 | 8% | 9% |
| energy | 33.5 | 30.4 | 17% | 16% |
| chips | 6.4 | 5.9 | 75% | 76% |
| antimatter | 1.8 | 1.5 | 66% | 65% |
| research_cubes | 14.8 | 13.3 | 76% | 77% |
| ambassadors | 19.3 | 17.4 | 67% | 68% |

## What each resource buys (per seat a game)

| resource | use | new | old |
|---|---|---|---|
| credits | play | 272.30 | 241.13 |
| credits | standard | 61.57 | 56.53 |
| credits | power | 11.30 | 10.67 |
| credits | pay | 4.05 | 3.32 |
| credits | use | 1.75 | 1.61 |
| credits | phase | 0.64 | 0.85 |
| insight | keep | 28.41 | 25.81 |
| insight | play | 16.64 | 14.87 |
| insight | power | 3.07 | 2.79 |
| insight | pay | 2.22 | 1.87 |
| insight | phase | 1.67 | 1.45 |
| insight | use | 0.53 | 0.45 |
| energy | power | 9.37 | 8.43 |
| energy | play | 9.10 | 8.15 |
| energy | pay | 5.48 | 4.91 |
| energy | standard | 4.49 | 4.33 |
| energy | use | 3.34 | 3.07 |
| energy | convert | 1.32 | 1.22 |
| energy | phase | 0.39 | 0.29 |
| chips | convert | 2.66 | 2.31 |
| chips | power | 2.31 | 2.25 |
| chips | standard | 1.25 | 1.21 |
| chips | use | 0.13 | 0.12 |
| antimatter | play | 1.64 | 1.42 |
| antimatter | use | 0.13 | 0.11 |
| research_cubes | convert | 5.56 | 4.99 |
| research_cubes | standard | 5.01 | 4.34 |
| research_cubes | power | 1.83 | 1.74 |
| research_cubes | pay | 1.69 | 1.47 |
| research_cubes | play | 0.48 | 0.48 |
| research_cubes | use | 0.19 | 0.18 |
| card_tokens | play | 3.05 | 2.07 |
| card_tokens | use | 0.27 | 0.24 |
| card_tokens | power | 0.10 | 0.07 |
| ambassadors | standard | 18.09 | 16.43 |
| ambassadors | play | 1.15 | 0.98 |

## Insight: keeps against plays (per game, all seats)

| measure | new | old |
|---|---|---|
| cards drafted (offered) | 75.58 | 69.09 |
| cards kept | 43.57 | 39.03 |
| cards played | 66.07 | 58.33 |
| insight carried (Patience) | 60.85 | 55.09 |
| insight left after keeps | 149.08 | 133.76 |
| insight to keeps | 56.82 | 51.62 |
| insight to plays | 34.58 | 30.87 |
| insight unspent at round end | 118.76 | 106.49 |
| rounds | 9.73 | 9.01 |
| tier-2 chosen in hand-building | 36.49 | 32.84 |

## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)

`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:
`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).
`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource
(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.

| counter | new | old |
|---|---|---|
| limit.keep_passed | 14.34 | 13.37 |
| limit.keep_short.insight | 1.67 | 1.66 |
| limit.pass.chosen | 8.33 | 7.72 |
| limit.pass.unaffordable | 6.01 | 5.65 |
| limit.play.held | 0.47 | 0.43 |
| limit.play.needs_tags | 19.07 | 18.20 |
| limit.play.short.antimatter | 1.99 | 1.80 |
| limit.play.short.chips | 0.06 | 0.04 |
| limit.play.short.credits | 130.61 | 114.72 |
| limit.play.short.energy | 3.68 | 3.24 |
| limit.play.short.insight | 4.58 | 4.35 |
| limit.play.short.research_cubes | 0.32 | 0.28 |
