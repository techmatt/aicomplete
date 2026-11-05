## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | 432 | 72.0 | 5.97 | +1.4 ± 2.2 | -0.05 | 65 / 69 / 82 |
| c/strategist | 432 | 70.6 | 6.02 | - | - | 63 / 70 / 78 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | 432 | 73.1 | 6.01 | +2.5 ± 2.2 | -0.01 | 65 / 72 / 83 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 432 | 73.4 | 6.00 | +2.8 ± 2.2 | -0.02 | 66 / 71 / 83 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | 432 | 71.3 | 6.01 | +0.7 ± 2.4 | -0.01 | 62 / 72 / 80 |
| c/strategist:plan_limits=1 | 432 | 70.4 | 6.02 | -0.2 ± 0.2 | +0.00 | 63 / 70 / 78 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | 432 | 70.6 | 6.01 | +0.0 ± 0.7 | -0.01 | 62 / 70 / 79 |
| c/greedy | 432 | 49.1 | 4.91 | -21.5 ± 2.9 | -1.11 | 53 / 44 / 51 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 432 | 48.1 | 4.94 | -22.5 ± 3.1 | -1.07 | 53 / 44 / 48 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | 432 | 71.8 | 6.05 | +1.2 ± 2.5 | +0.03 | 65 / 69 / 81 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 432 | 74.3 | 6.04 | +3.7 ± 2.5 | +0.02 | 69 / 72 / 82 |

## Lost at the limits, a game

| cell | hold_lost.credits | hold_lost.energy | hold_lost.chips | hold_lost.ambassadors | production_lost.credits | energy: games cut |
|---|---|---|---|---|---|---|
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | 1.09 | 3.66 | 1.50 | 0.42 | 0.45 | 56% |
| c/strategist | 1.41 | 4.37 | 1.27 | 0.46 | 0.42 | 56% |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | 2.42 | 4.66 | 1.53 | 0.41 | 0.72 | 58% |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 0.98 | 3.71 | 1.44 | 0.40 | 0.62 | 57% |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | 1.03 | 3.70 | 1.55 | 0.47 | 0.39 | 55% |
| c/strategist:plan_limits=1 | 1.73 | 4.40 | 1.32 | 0.48 | 0.48 | 56% |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | 1.82 | 4.00 | 1.59 | 0.37 | 0.57 | 56% |
| c/greedy | 0.00 | 0.57 | 0.05 | 0.20 | 0.00 | 14% |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 0.00 | 0.53 | 0.20 | 0.36 | 0.00 | 14% |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | 1.89 | 3.29 | 1.31 | 0.35 | 0.45 | 53% |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 2.19 | 3.35 | 1.41 | 0.26 | 0.52 | 51% |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | ambassadors |
|---|---|---|---|---|---|---|
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | Builder | 66.0 / 150 | 9.6 / 24 | 8.6 / 10 | 5.0 / 10 | 3.7 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | Protector | 55.8 / 149 | 11.6 / 25 | 6.5 / 10 | 3.5 / 10 | 4.6 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | Researcher | 65.2 / 150 | 21.0 / 59 | 6.8 / 10 | 3.8 / 10 | 4.4 / 10 |
| c/strategist | Builder | 65.4 / 150 | 9.4 / 29 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| c/strategist | Protector | 56.8 / 150 | 11.5 / 31 | 6.8 / 10 | 3.3 / 10 | 4.5 / 10 |
| c/strategist | Researcher | 65.2 / 150 | 28.8 / 78 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | Builder | 67.4 / 150 | 9.7 / 25 | 8.6 / 10 | 5.1 / 10 | 3.7 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | Protector | 56.7 / 150 | 11.8 / 29 | 6.6 / 10 | 3.7 / 10 | 4.6 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | Researcher | 65.9 / 150 | 20.9 / 53 | 6.9 / 10 | 4.0 / 10 | 4.4 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Builder | 67.6 / 150 | 9.8 / 36 | 8.6 / 10 | 5.1 / 10 | 3.7 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Protector | 56.2 / 149 | 11.7 / 32 | 6.6 / 10 | 3.6 / 10 | 4.5 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Researcher | 65.6 / 150 | 20.9 / 50 | 6.9 / 10 | 4.0 / 10 | 4.3 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | Builder | 65.4 / 150 | 10.0 / 30 | 8.5 / 10 | 5.2 / 10 | 3.7 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | Protector | 56.7 / 146 | 11.7 / 27 | 6.5 / 10 | 3.5 / 10 | 4.6 / 10 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | Researcher | 67.1 / 150 | 20.6 / 50 | 7.0 / 10 | 4.0 / 10 | 4.6 / 10 |
| c/strategist:plan_limits=1 | Builder | 65.2 / 150 | 9.3 / 23 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| c/strategist:plan_limits=1 | Protector | 56.8 / 150 | 11.6 / 31 | 6.8 / 10 | 3.4 / 10 | 4.6 / 10 |
| c/strategist:plan_limits=1 | Researcher | 65.4 / 150 | 28.9 / 91 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | Builder | 65.3 / 150 | 9.4 / 34 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | Protector | 56.5 / 150 | 11.2 / 31 | 6.7 / 10 | 3.2 / 10 | 4.6 / 10 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | Researcher | 64.5 / 150 | 28.8 / 92 | 6.9 / 10 | 3.8 / 10 | 4.6 / 10 |
| c/greedy | Builder | 42.7 / 116 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| c/greedy | Protector | 48.7 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.4 / 10 |
| c/greedy | Researcher | 43.3 / 65 | 16.5 / 48 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Builder | 42.8 / 113 | 8.2 / 16 | 3.8 / 10 | 3.1 / 10 | 4.5 / 10 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Protector | 49.0 / 91 | 10.0 / 21 | 5.4 / 10 | 2.8 / 10 | 4.3 / 10 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Researcher | 43.5 / 67 | 14.9 / 62 | 6.2 / 10 | 2.4 / 10 | 4.7 / 10 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | Builder | 63.9 / 150 | 10.4 / 29 | 8.2 / 10 | 5.1 / 10 | 3.7 / 10 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | Protector | 56.8 / 150 | 12.6 / 31 | 6.6 / 10 | 3.6 / 10 | 4.5 / 10 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | Researcher | 65.3 / 150 | 21.9 / 97 | 6.7 / 10 | 3.5 / 10 | 4.3 / 10 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Builder | 64.2 / 150 | 10.3 / 29 | 8.3 / 10 | 4.8 / 10 | 3.7 / 10 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Protector | 56.9 / 147 | 12.4 / 29 | 6.6 / 10 | 3.5 / 10 | 4.6 / 10 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Researcher | 65.8 / 150 | 21.6 / 57 | 6.6 / 10 | 3.4 / 10 | 4.3 / 10 |

## The Researcher: keeps against insight

Flag rules: `peak` is 122's (insight peak over 3x a round's insight income and over 10); `rise` is the fixed one (the peak's rise over the starting holding, same test). Keep rate: kept / offered after the opening; tier-2 share of its tier choices; insight at keep: mean held at its keep decisions.

| cell | rule | seats | n | insight peak | offered | kept | keep rate | tier-2 share | insight at keep |
|---|---|---|---|---|---|---|---|---|---|
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | peak | not flagged | 216 | 18.4 | 18.9 | 13.6 | 72% | 62% | 13.7 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | peak | flagged | 72 | 28.8 | 21.1 | 15.4 | 73% | 62% | 17.9 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | peak | all | 288 | 21.0 | 19.5 | 14.0 | 72% | 62% | 14.8 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | rise | not flagged | 267 | 19.7 | 19.1 | 13.7 | 72% | 61% | 14.2 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | rise | flagged | 21 | 37.9 | 24.2 | 18.1 | 75% | 65% | 21.3 |
| c/strategist | peak | not flagged | 118 | 18.3 | 17.7 | 11.8 | 67% | 58% | 13.0 |
| c/strategist | peak | flagged | 170 | 36.0 | 20.9 | 11.3 | 54% | 58% | 20.3 |
| c/strategist | peak | all | 288 | 28.8 | 19.6 | 11.5 | 59% | 58% | 17.3 |
| c/strategist | rise | not flagged | 194 | 21.6 | 17.7 | 11.1 | 62% | 58% | 14.3 |
| c/strategist | rise | flagged | 94 | 43.6 | 23.4 | 12.5 | 53% | 59% | 23.4 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | peak | not flagged | 225 | 18.7 | 19.2 | 13.8 | 72% | 61% | 13.8 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | peak | flagged | 63 | 28.8 | 21.5 | 15.8 | 74% | 62% | 18.2 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | peak | all | 288 | 20.9 | 19.7 | 14.3 | 72% | 61% | 14.8 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | rise | not flagged | 270 | 19.7 | 19.4 | 14.0 | 72% | 61% | 14.3 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | rise | flagged | 18 | 38.8 | 24.4 | 18.3 | 75% | 65% | 21.7 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 219 | 18.7 | 19.0 | 13.6 | 72% | 62% | 13.8 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | flagged | 69 | 28.2 | 21.2 | 15.5 | 73% | 61% | 17.8 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 20.9 | 19.6 | 14.1 | 72% | 62% | 14.7 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 270 | 19.9 | 19.3 | 13.9 | 72% | 61% | 14.4 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | flagged | 18 | 36.3 | 24.0 | 17.3 | 72% | 65% | 20.4 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | peak | not flagged | 225 | 18.7 | 19.3 | 14.0 | 72% | 62% | 13.7 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | peak | flagged | 63 | 27.4 | 20.4 | 14.8 | 73% | 62% | 17.8 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | peak | all | 288 | 20.6 | 19.6 | 14.2 | 72% | 62% | 14.6 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | rise | not flagged | 276 | 19.9 | 19.4 | 14.0 | 72% | 62% | 14.3 |
| c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | rise | flagged | 12 | 38.3 | 22.7 | 17.1 | 75% | 69% | 22.6 |
| c/strategist:plan_limits=1 | peak | not flagged | 119 | 18.6 | 17.8 | 11.8 | 66% | 58% | 12.9 |
| c/strategist:plan_limits=1 | peak | flagged | 169 | 36.2 | 20.9 | 11.3 | 54% | 58% | 20.4 |
| c/strategist:plan_limits=1 | peak | all | 288 | 28.9 | 19.6 | 11.5 | 59% | 58% | 17.3 |
| c/strategist:plan_limits=1 | rise | not flagged | 195 | 21.7 | 17.8 | 11.1 | 62% | 58% | 14.3 |
| c/strategist:plan_limits=1 | rise | flagged | 93 | 44.1 | 23.6 | 12.4 | 53% | 59% | 23.5 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 120 | 18.6 | 17.5 | 11.6 | 66% | 58% | 12.9 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | peak | flagged | 168 | 36.1 | 21.0 | 11.3 | 54% | 58% | 20.4 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 28.8 | 19.5 | 11.4 | 58% | 58% | 17.3 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 194 | 21.4 | 17.5 | 10.9 | 62% | 57% | 14.2 |
| c/strategist:plan_limits=1,w.upkeep_loss=1 | rise | flagged | 94 | 44.1 | 23.8 | 12.5 | 53% | 59% | 23.6 |
| c/greedy | peak | not flagged | 139 | 12.1 | 13.8 | 5.0 | 36% | 76% | 9.5 |
| c/greedy | peak | flagged | 149 | 20.7 | 15.1 | 4.2 | 28% | 77% | 13.6 |
| c/greedy | peak | all | 288 | 16.5 | 14.5 | 4.6 | 32% | 77% | 11.6 |
| c/greedy | rise | not flagged | 249 | 14.9 | 14.1 | 4.6 | 33% | 76% | 10.9 |
| c/greedy | rise | flagged | 39 | 26.8 | 16.9 | 4.3 | 25% | 78% | 16.2 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 181 | 13.2 | 14.6 | 6.5 | 45% | 76% | 10.1 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | flagged | 107 | 17.7 | 14.7 | 6.9 | 47% | 78% | 12.7 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 14.9 | 14.6 | 6.7 | 46% | 77% | 11.1 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 283 | 14.5 | 14.5 | 6.6 | 45% | 77% | 11.0 |
| c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | flagged | 5 | 38.0 | 21.4 | 11.8 | 55% | 85% | 19.7 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | peak | not flagged | 203 | 19.2 | 18.8 | 13.7 | 73% | 61% | 14.0 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | peak | flagged | 85 | 28.2 | 21.5 | 15.9 | 74% | 62% | 18.3 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | peak | all | 288 | 21.9 | 19.6 | 14.3 | 73% | 61% | 15.2 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | rise | not flagged | 266 | 20.4 | 19.2 | 14.0 | 73% | 61% | 14.7 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | rise | flagged | 22 | 39.2 | 24.5 | 18.5 | 75% | 65% | 22.5 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 206 | 19.0 | 18.7 | 13.7 | 73% | 61% | 14.0 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | flagged | 82 | 28.0 | 21.5 | 15.9 | 74% | 59% | 18.1 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 21.6 | 19.5 | 14.3 | 73% | 61% | 15.2 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 267 | 20.1 | 19.1 | 13.9 | 73% | 60% | 14.6 |
| c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | flagged | 21 | 39.5 | 24.4 | 19.1 | 78% | 66% | 23.0 |

## Plays a game (kept, offered): the r41 engine cards and the two named

| card | c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | c/strategist | c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1 | c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | c/strategist:plan_limits=1 | c/strategist:plan_limits=1,w.upkeep_loss=1 | c/greedy | c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| telemetry_archive | 0.028 (0.19, 0.16) | 0.032 (0.20, 0.17) | 0.028 (0.19, 0.17) | 0.025 (0.19, 0.17) | 0.030 (0.19, 0.16) | 0.032 (0.20, 0.17) | 0.035 (0.21, 0.17) | 0.007 (0.09, 0.08) | 0.007 (0.10, 0.09) | 0.028 (0.19, 0.16) | 0.028 (0.19, 0.16) |
| citation_engine | 0.044 (0.10, 0.16) | 0.069 (0.10, 0.15) | 0.065 (0.10, 0.16) | 0.058 (0.10, 0.16) | 0.081 (0.09, 0.15) | 0.069 (0.10, 0.15) | 0.072 (0.11, 0.16) | 0.141 (0.05, 0.09) | 0.155 (0.04, 0.06) | 0.046 (0.12, 0.18) | 0.046 (0.10, 0.15) |
| grad_student_pool | 0.317 (0.21, 0.13) | 0.308 (0.22, 0.17) | 0.310 (0.23, 0.15) | 0.315 (0.22, 0.13) | 0.324 (0.22, 0.15) | 0.306 (0.22, 0.16) | 0.306 (0.21, 0.15) | 0.398 (0.17, 0.09) | 0.380 (0.17, 0.08) | 0.301 (0.21, 0.12) | 0.292 (0.21, 0.11) |
| foundation_model_campus | 0.181 (0.37, 0.53) | 0.160 (0.31, 0.51) | 0.176 (0.38, 0.53) | 0.174 (0.39, 0.54) | 0.220 (0.38, 0.54) | 0.160 (0.30, 0.50) | 0.167 (0.29, 0.49) | 0.058 (0.12, 0.49) | 0.074 (0.14, 0.49) | 0.234 (0.42, 0.57) | 0.204 (0.37, 0.52) |
| data_licensing_deal | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) |
| retained_earnings | 0.245 (0.22, 0.16) | 0.225 (0.19, 0.16) | 0.241 (0.22, 0.15) | 0.236 (0.22, 0.15) | 0.234 (0.22, 0.15) | 0.229 (0.19, 0.16) | 0.218 (0.19, 0.16) | 0.005 (0.10, 0.07) | 0.005 (0.10, 0.08) | 0.222 (0.22, 0.16) | 0.252 (0.23, 0.17) |
| lights_out_assembly | 0.421 (0.28, 0.14) | 0.410 (0.27, 0.14) | 0.440 (0.27, 0.12) | 0.440 (0.28, 0.14) | 0.405 (0.29, 0.15) | 0.412 (0.28, 0.15) | 0.410 (0.28, 0.15) | 0.410 (0.23, 0.09) | 0.431 (0.22, 0.08) | 0.396 (0.32, 0.18) | 0.431 (0.30, 0.16) |
| honeypot_relay | 0.215 (0.25, 0.15) | 0.225 (0.24, 0.16) | 0.231 (0.26, 0.16) | 0.225 (0.26, 0.16) | 0.231 (0.25, 0.13) | 0.236 (0.24, 0.16) | 0.234 (0.24, 0.16) | 0.412 (0.20, 0.08) | 0.426 (0.21, 0.07) | 0.206 (0.24, 0.12) | 0.204 (0.25, 0.14) |
| neural_mirror | 0.252 (0.10, 0.11) | 0.190 (0.08, 0.13) | 0.264 (0.10, 0.11) | 0.257 (0.10, 0.12) | 0.255 (0.10, 0.13) | 0.188 (0.08, 0.13) | 0.188 (0.09, 0.13) | 0.019 (0.06, 0.07) | 0.023 (0.06, 0.07) | 0.257 (0.14, 0.16) | 0.255 (0.11, 0.14) |
| intrusion_lab | 0.051 (0.09, 0.15) | 0.035 (0.07, 0.13) | 0.025 (0.09, 0.15) | 0.028 (0.08, 0.15) | 0.021 (0.08, 0.16) | 0.037 (0.07, 0.13) | 0.028 (0.07, 0.14) | 0.023 (0.02, 0.07) | 0.021 (0.03, 0.07) | 0.044 (0.10, 0.17) | 0.028 (0.08, 0.15) |
| darknet_exchange | 0.271 (0.22, 0.15) | 0.248 (0.20, 0.16) | 0.285 (0.21, 0.13) | 0.282 (0.22, 0.14) | 0.257 (0.20, 0.14) | 0.243 (0.20, 0.16) | 0.241 (0.20, 0.15) | 0.368 (0.17, 0.07) | 0.384 (0.18, 0.08) | 0.250 (0.20, 0.13) | 0.282 (0.21, 0.12) |
| tidal_barrage | 0.021 (0.01, 0.14) | 0.021 (0.01, 0.12) | 0.023 (0.02, 0.15) | 0.021 (0.01, 0.15) | 0.021 (0.01, 0.13) | 0.021 (0.01, 0.12) | 0.025 (0.01, 0.12) | 0.000 (0.00, 0.10) | 0.000 (0.00, 0.09) | 0.021 (0.01, 0.13) | 0.019 (0.01, 0.14) |
| substation_hub | 0.150 (0.09, 0.13) | 0.139 (0.10, 0.13) | 0.150 (0.10, 0.15) | 0.150 (0.09, 0.14) | 0.139 (0.09, 0.14) | 0.137 (0.09, 0.13) | 0.130 (0.10, 0.14) | 0.037 (0.02, 0.06) | 0.039 (0.02, 0.06) | 0.106 (0.09, 0.15) | 0.130 (0.09, 0.14) |
| positron_storage_ring | 0.053 (0.25, 0.55) | 0.058 (0.23, 0.53) | 0.065 (0.24, 0.54) | 0.060 (0.23, 0.53) | 0.072 (0.25, 0.55) | 0.058 (0.23, 0.53) | 0.058 (0.23, 0.53) | 0.000 (0.06, 0.49) | 0.000 (0.05, 0.49) | 0.056 (0.22, 0.56) | 0.069 (0.25, 0.54) |
| continental_supergrid | 0.049 (0.17, 0.55) | 0.051 (0.16, 0.53) | 0.046 (0.18, 0.56) | 0.039 (0.18, 0.55) | 0.037 (0.17, 0.56) | 0.053 (0.16, 0.53) | 0.053 (0.15, 0.53) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) | 0.039 (0.16, 0.55) | 0.053 (0.16, 0.56) |
| open_assembly | 0.370 (0.21, 0.14) | 0.405 (0.22, 0.14) | 0.370 (0.21, 0.14) | 0.366 (0.21, 0.13) | 0.387 (0.20, 0.15) | 0.405 (0.22, 0.16) | 0.400 (0.22, 0.15) | 0.502 (0.16, 0.07) | 0.479 (0.16, 0.08) | 0.384 (0.21, 0.16) | 0.375 (0.21, 0.15) |
| peace_dividend | 0.188 (0.11, 0.15) | 0.183 (0.11, 0.17) | 0.176 (0.11, 0.14) | 0.190 (0.10, 0.14) | 0.188 (0.10, 0.15) | 0.181 (0.11, 0.17) | 0.190 (0.11, 0.16) | 0.025 (0.02, 0.07) | 0.012 (0.03, 0.08) | 0.167 (0.10, 0.13) | 0.185 (0.09, 0.12) |
| consulate_tower | 0.183 (0.07, 0.10) | 0.208 (0.09, 0.13) | 0.188 (0.08, 0.12) | 0.192 (0.09, 0.12) | 0.162 (0.09, 0.13) | 0.199 (0.09, 0.13) | 0.194 (0.09, 0.13) | 0.269 (0.03, 0.08) | 0.259 (0.03, 0.08) | 0.125 (0.07, 0.12) | 0.169 (0.10, 0.14) |
| orbital_embassy | 0.148 (0.34, 0.51) | 0.155 (0.30, 0.51) | 0.167 (0.37, 0.53) | 0.155 (0.38, 0.54) | 0.153 (0.38, 0.55) | 0.160 (0.30, 0.51) | 0.157 (0.29, 0.51) | 0.090 (0.16, 0.48) | 0.081 (0.18, 0.47) | 0.139 (0.38, 0.56) | 0.132 (0.38, 0.55) |
| agency_field_station | 0.095 (0.08, 0.13) | 0.123 (0.12, 0.15) | 0.095 (0.09, 0.13) | 0.097 (0.08, 0.12) | 0.104 (0.10, 0.14) | 0.120 (0.12, 0.15) | 0.125 (0.12, 0.15) | 0.312 (0.03, 0.08) | 0.343 (0.03, 0.09) | 0.079 (0.09, 0.15) | 0.095 (0.09, 0.13) |
| security_contractor_hq | 0.111 (0.11, 0.13) | 0.102 (0.09, 0.12) | 0.109 (0.10, 0.12) | 0.102 (0.11, 0.13) | 0.120 (0.11, 0.13) | 0.111 (0.09, 0.12) | 0.104 (0.09, 0.12) | 0.250 (0.04, 0.07) | 0.255 (0.04, 0.07) | 0.090 (0.11, 0.13) | 0.095 (0.10, 0.13) |
| agency_directorate | 0.049 (0.36, 0.54) | 0.051 (0.27, 0.53) | 0.046 (0.35, 0.54) | 0.051 (0.36, 0.53) | 0.042 (0.38, 0.57) | 0.046 (0.27, 0.52) | 0.058 (0.27, 0.52) | 0.046 (0.15, 0.51) | 0.051 (0.17, 0.50) | 0.051 (0.37, 0.55) | 0.056 (0.40, 0.56) |
| security_ops_floor | 0.037 (0.10, 0.12) | 0.035 (0.11, 0.14) | 0.037 (0.10, 0.14) | 0.032 (0.10, 0.14) | 0.037 (0.10, 0.12) | 0.035 (0.11, 0.13) | 0.030 (0.11, 0.13) | 0.181 (0.05, 0.09) | 0.204 (0.05, 0.09) | 0.028 (0.12, 0.15) | 0.028 (0.10, 0.14) |
| vertical_monopoly | 0.278 (0.43, 0.58) | 0.243 (0.37, 0.56) | 0.292 (0.44, 0.59) | 0.278 (0.44, 0.59) | 0.259 (0.45, 0.60) | 0.248 (0.37, 0.56) | 0.250 (0.37, 0.56) | 0.088 (0.11, 0.52) | 0.074 (0.14, 0.53) | 0.280 (0.43, 0.58) | 0.296 (0.45, 0.58) |
| power_to_the_people | 0.076 (0.23, 0.54) | 0.086 (0.18, 0.54) | 0.083 (0.24, 0.54) | 0.088 (0.24, 0.55) | 0.090 (0.25, 0.56) | 0.086 (0.18, 0.54) | 0.093 (0.18, 0.53) | 0.014 (0.03, 0.50) | 0.009 (0.09, 0.51) | 0.079 (0.23, 0.56) | 0.093 (0.25, 0.57) |
| research_royalties | 0.194 (0.08, 0.15) | 0.213 (0.10, 0.17) | 0.218 (0.08, 0.15) | 0.204 (0.08, 0.15) | 0.197 (0.09, 0.16) | 0.213 (0.09, 0.17) | 0.208 (0.09, 0.16) | 0.002 (0.02, 0.08) | 0.005 (0.03, 0.08) | 0.208 (0.08, 0.15) | 0.201 (0.08, 0.15) |
| interdisciplinary_institute | 0.250 (0.46, 0.56) | 0.236 (0.40, 0.54) | 0.255 (0.47, 0.57) | 0.259 (0.47, 0.57) | 0.259 (0.47, 0.58) | 0.236 (0.40, 0.53) | 0.234 (0.40, 0.54) | 0.213 (0.31, 0.53) | 0.227 (0.34, 0.55) | 0.273 (0.47, 0.59) | 0.250 (0.48, 0.59) |
| standardized_tooling | 0.366 (0.27, 0.16) | 0.384 (0.25, 0.16) | 0.373 (0.27, 0.15) | 0.361 (0.27, 0.16) | 0.377 (0.26, 0.15) | 0.387 (0.25, 0.16) | 0.361 (0.25, 0.16) | 0.458 (0.20, 0.07) | 0.444 (0.20, 0.06) | 0.363 (0.24, 0.14) | 0.384 (0.26, 0.15) |
| economies_of_scale | 0.234 (0.13, 0.16) | 0.236 (0.13, 0.19) | 0.227 (0.13, 0.16) | 0.231 (0.14, 0.16) | 0.234 (0.13, 0.15) | 0.236 (0.13, 0.18) | 0.222 (0.13, 0.18) | 0.009 (0.03, 0.07) | 0.005 (0.03, 0.07) | 0.218 (0.12, 0.13) | 0.211 (0.13, 0.16) |
| capital_projects_office | 0.442 (0.59, 0.52) | 0.410 (0.56, 0.50) | 0.447 (0.59, 0.52) | 0.444 (0.59, 0.52) | 0.463 (0.60, 0.53) | 0.410 (0.56, 0.50) | 0.407 (0.57, 0.50) | 0.507 (0.54, 0.46) | 0.532 (0.57, 0.49) | 0.481 (0.63, 0.55) | 0.470 (0.62, 0.53) |
| conglomerate_headquarters | 0.370 (0.50, 0.57) | 0.336 (0.45, 0.56) | 0.370 (0.50, 0.57) | 0.363 (0.49, 0.56) | 0.380 (0.50, 0.58) | 0.338 (0.45, 0.56) | 0.343 (0.45, 0.56) | 0.037 (0.28, 0.52) | 0.028 (0.35, 0.54) | 0.370 (0.50, 0.57) | 0.345 (0.49, 0.58) |
| ransomware_syndicate | 0.185 (0.07, 0.15) | 0.183 (0.07, 0.15) | 0.208 (0.07, 0.14) | 0.197 (0.07, 0.14) | 0.197 (0.07, 0.13) | 0.178 (0.07, 0.16) | 0.169 (0.07, 0.15) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) | 0.183 (0.08, 0.15) | 0.192 (0.09, 0.17) |
| exploit_broker | 0.049 (0.08, 0.17) | 0.065 (0.06, 0.17) | 0.044 (0.07, 0.17) | 0.049 (0.07, 0.17) | 0.065 (0.07, 0.16) | 0.056 (0.06, 0.17) | 0.053 (0.06, 0.16) | 0.183 (0.01, 0.07) | 0.181 (0.02, 0.06) | 0.049 (0.07, 0.16) | 0.051 (0.06, 0.14) |
| command_and_control_grid | 0.208 (0.43, 0.55) | 0.218 (0.38, 0.54) | 0.215 (0.44, 0.56) | 0.218 (0.43, 0.56) | 0.222 (0.41, 0.52) | 0.215 (0.38, 0.54) | 0.204 (0.39, 0.55) | 0.132 (0.25, 0.52) | 0.146 (0.28, 0.51) | 0.220 (0.43, 0.55) | 0.243 (0.44, 0.56) |
| capacity_market | 0.139 (0.07, 0.13) | 0.157 (0.06, 0.13) | 0.155 (0.08, 0.14) | 0.155 (0.07, 0.13) | 0.150 (0.08, 0.13) | 0.162 (0.06, 0.12) | 0.155 (0.06, 0.12) | 0.002 (0.00, 0.06) | 0.000 (0.01, 0.07) | 0.120 (0.08, 0.14) | 0.130 (0.06, 0.12) |
| dispatch_control_room | 0.014 (0.02, 0.14) | 0.012 (0.03, 0.17) | 0.016 (0.03, 0.16) | 0.012 (0.02, 0.15) | 0.012 (0.03, 0.15) | 0.012 (0.02, 0.16) | 0.007 (0.02, 0.17) | 0.000 (0.00, 0.10) | 0.002 (0.00, 0.09) | 0.014 (0.03, 0.15) | 0.012 (0.03, 0.17) |
| smart_grid_backbone | 0.021 (0.06, 0.57) | 0.028 (0.08, 0.58) | 0.021 (0.09, 0.58) | 0.019 (0.06, 0.58) | 0.021 (0.06, 0.59) | 0.023 (0.07, 0.58) | 0.023 (0.08, 0.59) | 0.000 (0.01, 0.57) | 0.000 (0.00, 0.56) | 0.019 (0.06, 0.61) | 0.023 (0.08, 0.59) |
| reconciliation_office | 0.208 (0.27, 0.17) | 0.231 (0.24, 0.16) | 0.215 (0.27, 0.16) | 0.204 (0.26, 0.17) | 0.181 (0.26, 0.16) | 0.227 (0.24, 0.15) | 0.227 (0.24, 0.15) | 0.396 (0.18, 0.08) | 0.387 (0.19, 0.09) | 0.148 (0.26, 0.15) | 0.167 (0.25, 0.15) |
| protocol_office | 0.387 (0.29, 0.13) | 0.380 (0.30, 0.16) | 0.391 (0.30, 0.14) | 0.389 (0.30, 0.14) | 0.373 (0.30, 0.13) | 0.382 (0.30, 0.15) | 0.377 (0.30, 0.16) | 0.463 (0.23, 0.07) | 0.465 (0.24, 0.07) | 0.356 (0.30, 0.14) | 0.356 (0.31, 0.14) |
| joint_task_force | 0.023 (0.07, 0.15) | 0.021 (0.08, 0.18) | 0.019 (0.08, 0.17) | 0.019 (0.08, 0.17) | 0.016 (0.06, 0.16) | 0.025 (0.08, 0.18) | 0.023 (0.08, 0.18) | 0.118 (0.02, 0.05) | 0.148 (0.02, 0.06) | 0.009 (0.08, 0.16) | 0.016 (0.07, 0.16) |
| defense_appropriations | 0.102 (0.07, 0.16) | 0.111 (0.08, 0.17) | 0.111 (0.07, 0.15) | 0.125 (0.07, 0.15) | 0.111 (0.09, 0.14) | 0.109 (0.07, 0.17) | 0.113 (0.07, 0.17) | 0.007 (0.03, 0.09) | 0.005 (0.04, 0.10) | 0.125 (0.08, 0.13) | 0.130 (0.07, 0.14) |
| homeland_security_act | 0.042 (0.18, 0.52) | 0.076 (0.19, 0.50) | 0.046 (0.19, 0.53) | 0.042 (0.18, 0.52) | 0.056 (0.19, 0.53) | 0.086 (0.20, 0.50) | 0.076 (0.20, 0.52) | 0.000 (0.04, 0.49) | 0.000 (0.06, 0.50) | 0.051 (0.18, 0.53) | 0.049 (0.18, 0.52) |
| preprint_server | 0.067 (0.10, 0.15) | 0.100 (0.11, 0.14) | 0.072 (0.10, 0.14) | 0.067 (0.11, 0.15) | 0.072 (0.10, 0.16) | 0.097 (0.11, 0.14) | 0.095 (0.11, 0.13) | 0.227 (0.03, 0.07) | 0.243 (0.03, 0.07) | 0.049 (0.09, 0.13) | 0.069 (0.11, 0.14) |
| supplier_network | 0.162 (0.12, 0.11) | 0.160 (0.13, 0.12) | 0.174 (0.13, 0.12) | 0.174 (0.12, 0.11) | 0.164 (0.12, 0.13) | 0.160 (0.13, 0.12) | 0.167 (0.13, 0.12) | 0.310 (0.10, 0.08) | 0.324 (0.10, 0.08) | 0.150 (0.12, 0.12) | 0.157 (0.12, 0.11) |
| fusion_research_campus | 0.093 (0.30, 0.58) | 0.088 (0.19, 0.54) | 0.088 (0.29, 0.58) | 0.095 (0.29, 0.57) | 0.093 (0.28, 0.57) | 0.095 (0.21, 0.55) | 0.093 (0.20, 0.56) | 0.005 (0.01, 0.55) | 0.016 (0.06, 0.56) | 0.095 (0.32, 0.57) | 0.100 (0.28, 0.57) |
| hyperscale_botnet | 0.051 (0.43, 0.54) | 0.009 (0.36, 0.49) | 0.056 (0.42, 0.53) | 0.053 (0.42, 0.53) | 0.046 (0.41, 0.52) | 0.012 (0.36, 0.49) | 0.007 (0.36, 0.49) | 0.116 (0.25, 0.48) | 0.111 (0.28, 0.46) | 0.141 (0.40, 0.52) | 0.120 (0.39, 0.51) |
| moonshot_grant | 0.030 (0.08, 0.17) | 0.019 (0.03, 0.16) | 0.025 (0.07, 0.16) | 0.032 (0.08, 0.17) | 0.030 (0.06, 0.16) | 0.019 (0.03, 0.16) | 0.019 (0.03, 0.15) | 0.007 (0.00, 0.07) | 0.005 (0.02, 0.08) | 0.019 (0.07, 0.14) | 0.023 (0.07, 0.15) |

Played 0 times: c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1: data_licensing_deal; c/strategist: data_licensing_deal; c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1: data_licensing_deal; c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1: data_licensing_deal; c/strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1: data_licensing_deal; c/strategist:plan_limits=1: data_licensing_deal; c/strategist:plan_limits=1,w.upkeep_loss=1: data_licensing_deal; c/greedy: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; c/greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, capacity_market, smart_grid_backbone, homeland_security_act; c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1: data_licensing_deal; c15/strategist:w.strat_contents=1,w.insight_income=1.5,w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1: data_licensing_deal
