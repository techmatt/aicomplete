## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | 432 | 72.9 | 6.05 | +2.3 ± 2.3 | +0.03 | 67 / 72 / 81 |
| strategist:w.insight_cap=10,keep_spare=1 | 432 | 70.6 | 5.97 | +0.0 ± 1.8 | -0.05 | 65 / 70 / 77 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | 432 | 72.7 | 6.02 | +2.1 ± 2.2 | +0.00 | 67 / 71 / 81 |
| strategist | 432 | 70.6 | 6.02 | - | - | 63 / 70 / 78 |
| strategist:w.insight_income=1 | 432 | 71.8 | 6.06 | +1.2 ± 1.8 | +0.04 | 65 / 70 / 80 |
| strategist:w.limit_room=1,plan_limits=1 | 432 | 69.7 | 6.02 | -0.9 ± 0.7 | -0.00 | 63 / 69 / 77 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | 432 | 70.1 | 6.00 | -0.5 ± 0.8 | -0.02 | 62 / 69 / 78 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | 432 | 71.3 | 5.97 | +0.7 ± 2.3 | -0.04 | 64 / 68 / 82 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | 432 | 70.8 | 6.02 | +0.2 ± 2.4 | +0.00 | 60 / 72 / 80 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | 432 | 70.8 | 6.00 | +0.2 ± 2.4 | -0.02 | 60 / 73 / 79 |
| greedy | 432 | 49.1 | 4.91 | -21.5 ± 2.9 | -1.11 | 53 / 44 / 51 |
| greedy:w.insight_cap=10,keep_spare=1 | 432 | 48.6 | 4.93 | -22.0 ± 3.1 | -1.09 | 53 / 44 / 49 |
| greedy:plan_limits=1 | 432 | 49.1 | 4.91 | -21.5 ± 2.9 | -1.11 | 53 / 44 / 51 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 432 | 48.1 | 4.94 | -22.5 ± 3.1 | -1.07 | 53 / 44 / 48 |

## Lost at the limits, a game

| cell | hold_lost.credits | hold_lost.energy | hold_lost.chips | hold_lost.ambassadors | production_lost.credits | energy: games cut |
|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | 2.18 | 3.88 | 1.40 | 0.36 | 0.47 | 53% |
| strategist:w.insight_cap=10,keep_spare=1 | 2.33 | 4.65 | 1.28 | 0.46 | 0.43 | 58% |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | 1.55 | 4.32 | 1.32 | 0.42 | 0.46 | 59% |
| strategist | 1.41 | 4.37 | 1.27 | 0.46 | 0.42 | 56% |
| strategist:w.insight_income=1 | 1.53 | 4.61 | 1.47 | 0.36 | 0.46 | 59% |
| strategist:w.limit_room=1,plan_limits=1 | 1.42 | 4.13 | 1.37 | 0.22 | 0.67 | 56% |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | 0.83 | 3.69 | 1.34 | 0.23 | 0.71 | 55% |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | 1.01 | 3.70 | 1.32 | 0.33 | 0.68 | 57% |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | 1.07 | 3.79 | 1.66 | 0.42 | 0.94 | 55% |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | 0.47 | 3.59 | 1.20 | 0.40 | 0.68 | 55% |
| greedy | 0.00 | 0.57 | 0.05 | 0.20 | 0.00 | 14% |
| greedy:w.insight_cap=10,keep_spare=1 | 0.00 | 0.53 | 0.19 | 0.28 | 0.00 | 14% |
| greedy:plan_limits=1 | 0.00 | 0.56 | 0.05 | 0.20 | 0.00 | 14% |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 0.00 | 0.53 | 0.20 | 0.36 | 0.00 | 14% |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | ambassadors |
|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | Builder | 64.2 / 150 | 10.4 / 29 | 8.2 / 10 | 4.9 / 10 | 3.6 / 10 |
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | Protector | 57.4 / 150 | 12.4 / 29 | 6.6 / 10 | 3.6 / 10 | 4.5 / 10 |
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | Researcher | 65.6 / 150 | 21.6 / 61 | 6.7 / 10 | 3.4 / 10 | 4.4 / 10 |
| strategist:w.insight_cap=10,keep_spare=1 | Builder | 66.3 / 150 | 9.3 / 22 | 8.5 / 10 | 5.0 / 10 | 3.7 / 10 |
| strategist:w.insight_cap=10,keep_spare=1 | Protector | 56.5 / 150 | 11.3 / 27 | 6.8 / 10 | 3.2 / 10 | 4.5 / 10 |
| strategist:w.insight_cap=10,keep_spare=1 | Researcher | 63.8 / 150 | 19.5 / 77 | 6.8 / 10 | 3.5 / 10 | 4.6 / 10 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | Builder | 67.8 / 150 | 9.7 / 25 | 8.6 / 10 | 5.1 / 10 | 3.7 / 10 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | Protector | 56.5 / 150 | 12.0 / 31 | 6.6 / 10 | 3.6 / 10 | 4.5 / 10 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | Researcher | 65.1 / 150 | 20.9 / 55 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |
| strategist | Builder | 65.4 / 150 | 9.4 / 29 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| strategist | Protector | 56.8 / 150 | 11.5 / 31 | 6.8 / 10 | 3.3 / 10 | 4.5 / 10 |
| strategist | Researcher | 65.2 / 150 | 28.8 / 78 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |
| strategist:w.insight_income=1 | Builder | 65.3 / 150 | 9.5 / 22 | 8.5 / 10 | 5.0 / 10 | 3.8 / 10 |
| strategist:w.insight_income=1 | Protector | 57.5 / 150 | 12.5 / 36 | 6.6 / 10 | 3.5 / 10 | 4.5 / 10 |
| strategist:w.insight_income=1 | Researcher | 63.5 / 150 | 32.6 / 104 | 6.8 / 10 | 3.6 / 10 | 4.5 / 10 |
| strategist:w.limit_room=1,plan_limits=1 | Builder | 65.5 / 150 | 9.4 / 27 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| strategist:w.limit_room=1,plan_limits=1 | Protector | 56.9 / 150 | 11.6 / 36 | 6.7 / 10 | 3.3 / 10 | 4.6 / 10 |
| strategist:w.limit_room=1,plan_limits=1 | Researcher | 64.4 / 150 | 28.5 / 97 | 6.9 / 10 | 3.8 / 10 | 4.5 / 10 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | Builder | 65.5 / 150 | 9.3 / 30 | 8.4 / 10 | 5.0 / 10 | 3.7 / 10 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | Protector | 56.9 / 150 | 11.2 / 29 | 6.7 / 10 | 3.2 / 10 | 4.5 / 10 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | Researcher | 64.0 / 150 | 28.5 / 99 | 6.9 / 10 | 3.7 / 10 | 4.5 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | Builder | 66.3 / 150 | 9.8 / 26 | 8.6 / 10 | 5.0 / 10 | 3.7 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | Protector | 56.1 / 150 | 11.6 / 26 | 6.5 / 10 | 3.4 / 10 | 4.6 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | Researcher | 64.4 / 150 | 20.9 / 55 | 6.8 / 10 | 3.6 / 10 | 4.2 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | Builder | 65.4 / 150 | 10.0 / 30 | 8.5 / 10 | 5.1 / 10 | 3.7 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | Protector | 57.0 / 139 | 11.9 / 34 | 6.5 / 10 | 3.6 / 10 | 4.5 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | Researcher | 67.5 / 150 | 20.7 / 61 | 6.9 / 10 | 4.0 / 10 | 4.6 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | Builder | 64.0 / 150 | 9.8 / 37 | 8.5 / 10 | 5.0 / 10 | 3.6 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | Protector | 56.7 / 150 | 11.7 / 31 | 6.5 / 10 | 3.5 / 10 | 4.6 / 10 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | Researcher | 65.4 / 150 | 20.6 / 48 | 6.9 / 10 | 3.9 / 10 | 4.6 / 10 |
| greedy | Builder | 42.7 / 116 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| greedy | Protector | 48.7 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.4 / 10 |
| greedy | Researcher | 43.3 / 65 | 16.5 / 48 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| greedy:w.insight_cap=10,keep_spare=1 | Builder | 42.8 / 116 | 8.2 / 16 | 3.8 / 10 | 3.1 / 10 | 4.5 / 10 |
| greedy:w.insight_cap=10,keep_spare=1 | Protector | 49.0 / 88 | 10.0 / 26 | 5.4 / 10 | 2.8 / 10 | 4.3 / 10 |
| greedy:w.insight_cap=10,keep_spare=1 | Researcher | 43.6 / 67 | 14.7 / 62 | 6.2 / 10 | 2.4 / 10 | 4.8 / 10 |
| greedy:plan_limits=1 | Builder | 42.7 / 116 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| greedy:plan_limits=1 | Protector | 48.6 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.4 / 10 |
| greedy:plan_limits=1 | Researcher | 43.3 / 65 | 16.5 / 48 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Builder | 42.8 / 113 | 8.2 / 16 | 3.8 / 10 | 3.1 / 10 | 4.5 / 10 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Protector | 49.0 / 91 | 10.0 / 21 | 5.4 / 10 | 2.8 / 10 | 4.3 / 10 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Researcher | 43.5 / 67 | 14.9 / 62 | 6.2 / 10 | 2.4 / 10 | 4.7 / 10 |

## The Researcher: keeps against insight

Flag rules: `peak` is 122's (insight peak over 3x a round's insight income and over 10); `rise` is the fixed one (the peak's rise over the starting holding, same test). Keep rate: kept / offered after the opening; tier-2 share of its tier choices; insight at keep: mean held at its keep decisions.

| cell | rule | seats | n | insight peak | offered | kept | keep rate | tier-2 share | insight at keep |
|---|---|---|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | peak | not flagged | 208 | 18.8 | 18.7 | 13.8 | 73% | 60% | 14.0 |
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | peak | flagged | 80 | 29.0 | 21.9 | 16.3 | 74% | 62% | 18.4 |
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | peak | all | 288 | 21.6 | 19.6 | 14.5 | 74% | 61% | 15.2 |
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | rise | not flagged | 265 | 20.2 | 19.2 | 14.1 | 73% | 60% | 14.6 |
| strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | rise | flagged | 23 | 38.3 | 24.2 | 18.7 | 77% | 64% | 22.0 |
| strategist:w.insight_cap=10,keep_spare=1 | peak | not flagged | 230 | 17.2 | 18.6 | 13.1 | 70% | 60% | 13.1 |
| strategist:w.insight_cap=10,keep_spare=1 | peak | flagged | 58 | 28.4 | 21.4 | 16.0 | 74% | 61% | 17.6 |
| strategist:w.insight_cap=10,keep_spare=1 | peak | all | 288 | 19.5 | 19.2 | 13.7 | 71% | 60% | 14.0 |
| strategist:w.insight_cap=10,keep_spare=1 | rise | not flagged | 275 | 18.3 | 18.9 | 13.4 | 71% | 60% | 13.6 |
| strategist:w.insight_cap=10,keep_spare=1 | rise | flagged | 13 | 43.1 | 25.2 | 19.9 | 79% | 64% | 22.3 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | peak | not flagged | 225 | 18.5 | 19.1 | 13.8 | 72% | 60% | 13.8 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | peak | flagged | 63 | 29.3 | 21.5 | 16.0 | 74% | 61% | 18.4 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | peak | all | 288 | 20.9 | 19.7 | 14.3 | 73% | 61% | 14.8 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | rise | not flagged | 268 | 19.5 | 19.3 | 14.0 | 72% | 60% | 14.3 |
| strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | rise | flagged | 20 | 39.7 | 24.0 | 18.3 | 76% | 65% | 21.8 |
| strategist | peak | not flagged | 118 | 18.3 | 17.7 | 11.8 | 67% | 58% | 13.0 |
| strategist | peak | flagged | 170 | 36.0 | 20.9 | 11.3 | 54% | 58% | 20.3 |
| strategist | peak | all | 288 | 28.8 | 19.6 | 11.5 | 59% | 58% | 17.3 |
| strategist | rise | not flagged | 194 | 21.6 | 17.7 | 11.1 | 62% | 58% | 14.3 |
| strategist | rise | flagged | 94 | 43.6 | 23.4 | 12.5 | 53% | 59% | 23.4 |
| strategist:w.insight_income=1 | peak | not flagged | 100 | 18.8 | 16.8 | 10.9 | 65% | 59% | 13.3 |
| strategist:w.insight_income=1 | peak | flagged | 188 | 39.9 | 21.6 | 12.1 | 56% | 58% | 21.8 |
| strategist:w.insight_income=1 | peak | all | 288 | 32.6 | 19.9 | 11.7 | 59% | 58% | 18.8 |
| strategist:w.insight_income=1 | rise | not flagged | 178 | 22.4 | 17.5 | 10.9 | 62% | 57% | 14.9 |
| strategist:w.insight_income=1 | rise | flagged | 110 | 49.0 | 23.7 | 13.0 | 55% | 59% | 25.2 |
| strategist:w.limit_room=1,plan_limits=1 | peak | not flagged | 122 | 18.6 | 17.6 | 11.7 | 66% | 59% | 13.1 |
| strategist:w.limit_room=1,plan_limits=1 | peak | flagged | 166 | 35.8 | 21.0 | 11.5 | 55% | 58% | 20.3 |
| strategist:w.limit_room=1,plan_limits=1 | peak | all | 288 | 28.5 | 19.6 | 11.6 | 59% | 58% | 17.2 |
| strategist:w.limit_room=1,plan_limits=1 | rise | not flagged | 198 | 21.5 | 17.7 | 11.2 | 63% | 58% | 14.3 |
| strategist:w.limit_room=1,plan_limits=1 | rise | flagged | 90 | 43.8 | 23.6 | 12.4 | 53% | 59% | 23.6 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 121 | 18.5 | 17.5 | 11.6 | 66% | 58% | 13.0 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | peak | flagged | 167 | 35.7 | 20.9 | 11.3 | 54% | 58% | 20.2 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 28.5 | 19.5 | 11.4 | 59% | 58% | 17.2 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 198 | 21.4 | 17.6 | 10.9 | 62% | 58% | 14.3 |
| strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | rise | flagged | 90 | 44.0 | 23.7 | 12.4 | 52% | 59% | 23.5 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | peak | not flagged | 218 | 18.3 | 18.9 | 13.5 | 71% | 61% | 13.7 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | peak | flagged | 70 | 29.0 | 21.1 | 15.5 | 73% | 62% | 17.8 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | peak | all | 288 | 20.9 | 19.4 | 14.0 | 72% | 62% | 14.7 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | rise | not flagged | 264 | 19.2 | 19.0 | 13.6 | 71% | 61% | 14.1 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | rise | flagged | 24 | 39.2 | 24.2 | 18.5 | 76% | 67% | 21.2 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | peak | not flagged | 223 | 18.5 | 19.2 | 13.9 | 72% | 62% | 13.7 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | peak | flagged | 65 | 28.1 | 21.0 | 15.2 | 73% | 63% | 17.9 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | peak | all | 288 | 20.7 | 19.6 | 14.2 | 72% | 62% | 14.7 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | rise | not flagged | 272 | 19.5 | 19.4 | 13.9 | 72% | 62% | 14.2 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | rise | flagged | 16 | 40.5 | 24.0 | 18.7 | 78% | 70% | 22.0 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | peak | not flagged | 221 | 18.5 | 19.2 | 13.8 | 72% | 62% | 13.6 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | peak | flagged | 67 | 27.4 | 21.0 | 15.2 | 72% | 62% | 17.8 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | peak | all | 288 | 20.6 | 19.6 | 14.1 | 72% | 62% | 14.6 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | rise | not flagged | 274 | 19.8 | 19.4 | 14.0 | 72% | 62% | 14.3 |
| strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | rise | flagged | 14 | 35.7 | 22.3 | 17.1 | 77% | 65% | 20.9 |
| greedy | peak | not flagged | 139 | 12.1 | 13.8 | 5.0 | 36% | 76% | 9.5 |
| greedy | peak | flagged | 149 | 20.7 | 15.1 | 4.2 | 28% | 77% | 13.6 |
| greedy | peak | all | 288 | 16.5 | 14.5 | 4.6 | 32% | 77% | 11.6 |
| greedy | rise | not flagged | 249 | 14.9 | 14.1 | 4.6 | 33% | 76% | 10.9 |
| greedy | rise | flagged | 39 | 26.8 | 16.9 | 4.3 | 25% | 78% | 16.2 |
| greedy:w.insight_cap=10,keep_spare=1 | peak | not flagged | 184 | 13.2 | 14.6 | 6.5 | 44% | 76% | 10.1 |
| greedy:w.insight_cap=10,keep_spare=1 | peak | flagged | 104 | 17.4 | 14.5 | 6.8 | 47% | 79% | 12.7 |
| greedy:w.insight_cap=10,keep_spare=1 | peak | all | 288 | 14.7 | 14.6 | 6.6 | 45% | 77% | 11.0 |
| greedy:w.insight_cap=10,keep_spare=1 | rise | not flagged | 284 | 14.4 | 14.5 | 6.6 | 45% | 77% | 10.9 |
| greedy:w.insight_cap=10,keep_spare=1 | rise | flagged | 4 | 35.5 | 18.0 | 9.8 | 54% | 82% | 19.6 |
| greedy:plan_limits=1 | peak | not flagged | 139 | 12.1 | 13.8 | 5.0 | 36% | 76% | 9.5 |
| greedy:plan_limits=1 | peak | flagged | 149 | 20.7 | 15.1 | 4.2 | 28% | 77% | 13.6 |
| greedy:plan_limits=1 | peak | all | 288 | 16.5 | 14.5 | 4.6 | 32% | 77% | 11.6 |
| greedy:plan_limits=1 | rise | not flagged | 249 | 14.9 | 14.1 | 4.6 | 33% | 76% | 10.9 |
| greedy:plan_limits=1 | rise | flagged | 39 | 26.8 | 16.9 | 4.3 | 25% | 78% | 16.2 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 181 | 13.2 | 14.6 | 6.5 | 45% | 76% | 10.1 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | flagged | 107 | 17.7 | 14.7 | 6.9 | 47% | 78% | 12.7 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 14.9 | 14.6 | 6.7 | 46% | 77% | 11.1 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 283 | 14.5 | 14.5 | 6.6 | 45% | 77% | 11.0 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | flagged | 5 | 38.0 | 21.4 | 11.8 | 55% | 85% | 19.7 |

## Plays a game (kept, offered): the r41 engine cards and the two named

| card | strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1 | strategist:w.insight_cap=10,keep_spare=1 | strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1 | strategist | strategist:w.insight_income=1 | strategist:w.limit_room=1,plan_limits=1 | strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1 | strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1 | strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1 | greedy | greedy:w.insight_cap=10,keep_spare=1 | greedy:plan_limits=1 | greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| telemetry_archive | 0.035 (0.19, 0.17) | 0.032 (0.20, 0.18) | 0.030 (0.19, 0.18) | 0.032 (0.20, 0.17) | 0.035 (0.19, 0.18) | 0.032 (0.20, 0.17) | 0.032 (0.20, 0.17) | 0.030 (0.19, 0.16) | 0.028 (0.19, 0.16) | 0.025 (0.19, 0.16) | 0.007 (0.09, 0.08) | 0.007 (0.10, 0.09) | 0.007 (0.09, 0.08) | 0.007 (0.10, 0.09) |
| citation_engine | 0.053 (0.10, 0.16) | 0.060 (0.11, 0.17) | 0.058 (0.12, 0.18) | 0.069 (0.10, 0.15) | 0.065 (0.11, 0.16) | 0.069 (0.11, 0.16) | 0.065 (0.12, 0.16) | 0.049 (0.10, 0.16) | 0.067 (0.11, 0.16) | 0.074 (0.11, 0.16) | 0.141 (0.05, 0.09) | 0.155 (0.04, 0.06) | 0.141 (0.05, 0.09) | 0.155 (0.04, 0.06) |
| grad_student_pool | 0.315 (0.22, 0.12) | 0.315 (0.20, 0.13) | 0.310 (0.23, 0.14) | 0.308 (0.22, 0.17) | 0.310 (0.24, 0.16) | 0.306 (0.22, 0.17) | 0.306 (0.21, 0.16) | 0.315 (0.21, 0.13) | 0.333 (0.21, 0.15) | 0.312 (0.20, 0.14) | 0.398 (0.17, 0.09) | 0.380 (0.17, 0.08) | 0.398 (0.17, 0.09) | 0.380 (0.17, 0.08) |
| foundation_model_campus | 0.206 (0.40, 0.55) | 0.181 (0.32, 0.50) | 0.185 (0.37, 0.53) | 0.160 (0.31, 0.51) | 0.150 (0.33, 0.51) | 0.162 (0.31, 0.50) | 0.174 (0.30, 0.49) | 0.171 (0.36, 0.53) | 0.215 (0.38, 0.54) | 0.197 (0.37, 0.53) | 0.058 (0.12, 0.49) | 0.076 (0.14, 0.50) | 0.058 (0.11, 0.49) | 0.074 (0.14, 0.49) |
| data_licensing_deal | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) |
| retained_earnings | 0.220 (0.21, 0.18) | 0.243 (0.20, 0.17) | 0.238 (0.20, 0.15) | 0.225 (0.19, 0.16) | 0.234 (0.19, 0.16) | 0.218 (0.19, 0.17) | 0.218 (0.19, 0.17) | 0.248 (0.22, 0.15) | 0.243 (0.22, 0.15) | 0.241 (0.21, 0.13) | 0.005 (0.10, 0.07) | 0.005 (0.10, 0.08) | 0.005 (0.10, 0.07) | 0.005 (0.10, 0.08) |
| lights_out_assembly | 0.419 (0.30, 0.16) | 0.421 (0.27, 0.13) | 0.440 (0.28, 0.14) | 0.410 (0.27, 0.14) | 0.414 (0.28, 0.15) | 0.400 (0.27, 0.14) | 0.417 (0.27, 0.14) | 0.403 (0.28, 0.14) | 0.414 (0.30, 0.16) | 0.412 (0.30, 0.15) | 0.410 (0.23, 0.09) | 0.428 (0.22, 0.08) | 0.410 (0.23, 0.09) | 0.431 (0.22, 0.08) |
| honeypot_relay | 0.201 (0.25, 0.15) | 0.231 (0.26, 0.16) | 0.215 (0.26, 0.15) | 0.225 (0.24, 0.16) | 0.215 (0.25, 0.15) | 0.222 (0.24, 0.16) | 0.222 (0.25, 0.16) | 0.215 (0.25, 0.15) | 0.227 (0.26, 0.13) | 0.229 (0.25, 0.13) | 0.412 (0.20, 0.08) | 0.419 (0.21, 0.07) | 0.412 (0.20, 0.08) | 0.426 (0.21, 0.07) |
| neural_mirror | 0.243 (0.11, 0.15) | 0.185 (0.09, 0.12) | 0.243 (0.11, 0.13) | 0.190 (0.08, 0.13) | 0.250 (0.07, 0.12) | 0.183 (0.09, 0.13) | 0.194 (0.08, 0.13) | 0.252 (0.10, 0.11) | 0.250 (0.10, 0.12) | 0.252 (0.11, 0.12) | 0.019 (0.06, 0.07) | 0.021 (0.06, 0.07) | 0.019 (0.06, 0.07) | 0.023 (0.06, 0.07) |
| intrusion_lab | 0.019 (0.09, 0.15) | 0.025 (0.07, 0.13) | 0.032 (0.09, 0.16) | 0.035 (0.07, 0.13) | 0.028 (0.08, 0.16) | 0.023 (0.08, 0.14) | 0.028 (0.07, 0.13) | 0.053 (0.08, 0.13) | 0.019 (0.09, 0.16) | 0.044 (0.09, 0.16) | 0.023 (0.02, 0.07) | 0.023 (0.03, 0.07) | 0.023 (0.02, 0.07) | 0.021 (0.03, 0.07) |
| darknet_exchange | 0.269 (0.21, 0.13) | 0.257 (0.22, 0.16) | 0.275 (0.22, 0.15) | 0.248 (0.20, 0.16) | 0.259 (0.22, 0.18) | 0.231 (0.21, 0.17) | 0.231 (0.20, 0.16) | 0.266 (0.22, 0.15) | 0.278 (0.20, 0.14) | 0.269 (0.19, 0.13) | 0.368 (0.17, 0.07) | 0.387 (0.18, 0.08) | 0.368 (0.17, 0.07) | 0.384 (0.18, 0.08) |
| tidal_barrage | 0.016 (0.02, 0.14) | 0.021 (0.01, 0.14) | 0.025 (0.02, 0.15) | 0.021 (0.01, 0.12) | 0.030 (0.02, 0.13) | 0.030 (0.01, 0.12) | 0.023 (0.01, 0.12) | 0.019 (0.01, 0.13) | 0.023 (0.01, 0.13) | 0.016 (0.01, 0.13) | 0.000 (0.00, 0.10) | 0.000 (0.00, 0.09) | 0.000 (0.00, 0.10) | 0.000 (0.00, 0.09) |
| substation_hub | 0.123 (0.10, 0.14) | 0.148 (0.10, 0.13) | 0.150 (0.10, 0.15) | 0.139 (0.10, 0.13) | 0.150 (0.11, 0.15) | 0.139 (0.10, 0.12) | 0.132 (0.10, 0.12) | 0.150 (0.09, 0.14) | 0.137 (0.09, 0.13) | 0.127 (0.09, 0.13) | 0.037 (0.02, 0.06) | 0.037 (0.02, 0.06) | 0.037 (0.02, 0.06) | 0.039 (0.02, 0.06) |
| positron_storage_ring | 0.067 (0.23, 0.54) | 0.056 (0.20, 0.53) | 0.062 (0.23, 0.54) | 0.058 (0.23, 0.53) | 0.058 (0.22, 0.52) | 0.060 (0.23, 0.53) | 0.058 (0.23, 0.53) | 0.049 (0.24, 0.55) | 0.083 (0.26, 0.54) | 0.076 (0.26, 0.54) | 0.000 (0.06, 0.49) | 0.000 (0.05, 0.49) | 0.000 (0.06, 0.49) | 0.000 (0.05, 0.49) |
| continental_supergrid | 0.042 (0.17, 0.56) | 0.053 (0.17, 0.54) | 0.046 (0.16, 0.55) | 0.051 (0.16, 0.53) | 0.037 (0.16, 0.54) | 0.053 (0.16, 0.53) | 0.044 (0.15, 0.54) | 0.049 (0.17, 0.56) | 0.035 (0.16, 0.55) | 0.037 (0.16, 0.55) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) |
| open_assembly | 0.382 (0.20, 0.14) | 0.396 (0.21, 0.14) | 0.370 (0.21, 0.16) | 0.405 (0.22, 0.14) | 0.398 (0.22, 0.16) | 0.403 (0.21, 0.14) | 0.403 (0.21, 0.14) | 0.382 (0.21, 0.14) | 0.403 (0.21, 0.15) | 0.398 (0.20, 0.14) | 0.502 (0.16, 0.07) | 0.481 (0.16, 0.08) | 0.502 (0.16, 0.07) | 0.479 (0.16, 0.08) |
| peace_dividend | 0.190 (0.09, 0.13) | 0.171 (0.12, 0.16) | 0.167 (0.11, 0.15) | 0.183 (0.11, 0.17) | 0.174 (0.10, 0.16) | 0.176 (0.10, 0.16) | 0.188 (0.10, 0.16) | 0.199 (0.11, 0.15) | 0.201 (0.10, 0.15) | 0.199 (0.10, 0.14) | 0.025 (0.02, 0.07) | 0.012 (0.03, 0.08) | 0.025 (0.02, 0.07) | 0.012 (0.03, 0.08) |
| consulate_tower | 0.164 (0.09, 0.12) | 0.171 (0.09, 0.12) | 0.185 (0.09, 0.12) | 0.208 (0.09, 0.13) | 0.215 (0.09, 0.12) | 0.178 (0.09, 0.13) | 0.188 (0.09, 0.13) | 0.157 (0.08, 0.11) | 0.144 (0.09, 0.13) | 0.125 (0.08, 0.12) | 0.269 (0.03, 0.08) | 0.269 (0.03, 0.08) | 0.266 (0.03, 0.08) | 0.259 (0.03, 0.08) |
| orbital_embassy | 0.120 (0.38, 0.55) | 0.146 (0.37, 0.53) | 0.150 (0.37, 0.52) | 0.155 (0.30, 0.51) | 0.146 (0.31, 0.53) | 0.150 (0.30, 0.51) | 0.148 (0.30, 0.51) | 0.150 (0.34, 0.51) | 0.148 (0.38, 0.54) | 0.157 (0.36, 0.52) | 0.090 (0.16, 0.48) | 0.083 (0.18, 0.47) | 0.090 (0.16, 0.48) | 0.081 (0.18, 0.47) |
| agency_field_station | 0.104 (0.10, 0.12) | 0.109 (0.11, 0.13) | 0.120 (0.12, 0.13) | 0.123 (0.12, 0.15) | 0.125 (0.12, 0.13) | 0.120 (0.12, 0.15) | 0.123 (0.12, 0.15) | 0.097 (0.09, 0.14) | 0.102 (0.10, 0.14) | 0.102 (0.09, 0.13) | 0.312 (0.03, 0.08) | 0.338 (0.03, 0.09) | 0.312 (0.03, 0.08) | 0.343 (0.03, 0.09) |
| security_contractor_hq | 0.102 (0.10, 0.12) | 0.100 (0.11, 0.13) | 0.118 (0.10, 0.12) | 0.102 (0.09, 0.12) | 0.118 (0.10, 0.13) | 0.100 (0.08, 0.12) | 0.100 (0.08, 0.12) | 0.104 (0.11, 0.13) | 0.113 (0.11, 0.13) | 0.111 (0.12, 0.14) | 0.250 (0.04, 0.07) | 0.255 (0.04, 0.07) | 0.250 (0.04, 0.07) | 0.255 (0.04, 0.07) |
| agency_directorate | 0.060 (0.37, 0.54) | 0.056 (0.32, 0.53) | 0.056 (0.32, 0.53) | 0.051 (0.27, 0.53) | 0.044 (0.27, 0.53) | 0.042 (0.28, 0.53) | 0.049 (0.27, 0.53) | 0.042 (0.36, 0.53) | 0.042 (0.38, 0.57) | 0.037 (0.38, 0.56) | 0.046 (0.15, 0.51) | 0.051 (0.17, 0.50) | 0.046 (0.15, 0.51) | 0.051 (0.17, 0.50) |
| security_ops_floor | 0.025 (0.10, 0.14) | 0.030 (0.10, 0.13) | 0.035 (0.09, 0.14) | 0.035 (0.11, 0.14) | 0.032 (0.10, 0.16) | 0.035 (0.11, 0.13) | 0.030 (0.09, 0.12) | 0.037 (0.09, 0.11) | 0.039 (0.09, 0.12) | 0.035 (0.09, 0.12) | 0.181 (0.05, 0.09) | 0.204 (0.05, 0.09) | 0.181 (0.05, 0.09) | 0.204 (0.05, 0.09) |
| vertical_monopoly | 0.285 (0.43, 0.57) | 0.262 (0.40, 0.56) | 0.282 (0.42, 0.58) | 0.243 (0.37, 0.56) | 0.229 (0.35, 0.55) | 0.245 (0.37, 0.56) | 0.241 (0.36, 0.55) | 0.285 (0.43, 0.58) | 0.264 (0.45, 0.60) | 0.262 (0.44, 0.59) | 0.088 (0.11, 0.52) | 0.074 (0.13, 0.52) | 0.088 (0.11, 0.52) | 0.074 (0.14, 0.53) |
| power_to_the_people | 0.083 (0.28, 0.58) | 0.074 (0.23, 0.54) | 0.079 (0.24, 0.55) | 0.086 (0.18, 0.54) | 0.086 (0.17, 0.55) | 0.088 (0.18, 0.54) | 0.093 (0.19, 0.55) | 0.081 (0.22, 0.54) | 0.088 (0.25, 0.56) | 0.081 (0.24, 0.56) | 0.014 (0.03, 0.50) | 0.007 (0.08, 0.50) | 0.014 (0.03, 0.50) | 0.009 (0.09, 0.51) |
| research_royalties | 0.206 (0.08, 0.15) | 0.199 (0.09, 0.16) | 0.206 (0.08, 0.15) | 0.213 (0.10, 0.17) | 0.206 (0.09, 0.16) | 0.208 (0.10, 0.17) | 0.218 (0.09, 0.17) | 0.206 (0.08, 0.15) | 0.197 (0.09, 0.16) | 0.183 (0.08, 0.15) | 0.002 (0.02, 0.08) | 0.002 (0.03, 0.08) | 0.002 (0.02, 0.08) | 0.005 (0.03, 0.08) |
| interdisciplinary_institute | 0.238 (0.47, 0.59) | 0.227 (0.44, 0.55) | 0.257 (0.45, 0.55) | 0.236 (0.40, 0.54) | 0.275 (0.43, 0.56) | 0.236 (0.39, 0.54) | 0.234 (0.38, 0.52) | 0.255 (0.46, 0.56) | 0.264 (0.48, 0.59) | 0.255 (0.46, 0.57) | 0.213 (0.31, 0.53) | 0.225 (0.35, 0.55) | 0.213 (0.31, 0.53) | 0.227 (0.34, 0.55) |
| standardized_tooling | 0.375 (0.25, 0.15) | 0.366 (0.25, 0.16) | 0.382 (0.26, 0.15) | 0.384 (0.25, 0.16) | 0.377 (0.25, 0.15) | 0.380 (0.25, 0.16) | 0.373 (0.25, 0.15) | 0.370 (0.27, 0.16) | 0.382 (0.26, 0.15) | 0.375 (0.25, 0.15) | 0.458 (0.20, 0.07) | 0.444 (0.20, 0.06) | 0.458 (0.20, 0.07) | 0.444 (0.20, 0.06) |
| economies_of_scale | 0.229 (0.14, 0.16) | 0.229 (0.14, 0.17) | 0.218 (0.13, 0.17) | 0.236 (0.13, 0.19) | 0.234 (0.14, 0.19) | 0.241 (0.14, 0.19) | 0.234 (0.13, 0.18) | 0.229 (0.13, 0.15) | 0.243 (0.13, 0.15) | 0.222 (0.13, 0.15) | 0.009 (0.03, 0.07) | 0.005 (0.03, 0.07) | 0.009 (0.03, 0.07) | 0.005 (0.03, 0.07) |
| capital_projects_office | 0.470 (0.62, 0.54) | 0.424 (0.60, 0.52) | 0.447 (0.60, 0.52) | 0.410 (0.56, 0.50) | 0.426 (0.59, 0.52) | 0.410 (0.56, 0.50) | 0.412 (0.57, 0.50) | 0.435 (0.59, 0.52) | 0.456 (0.59, 0.52) | 0.468 (0.60, 0.53) | 0.507 (0.54, 0.46) | 0.532 (0.57, 0.49) | 0.507 (0.54, 0.46) | 0.532 (0.57, 0.49) |
| conglomerate_headquarters | 0.356 (0.50, 0.58) | 0.359 (0.50, 0.57) | 0.363 (0.50, 0.57) | 0.336 (0.45, 0.56) | 0.324 (0.46, 0.55) | 0.338 (0.44, 0.56) | 0.331 (0.45, 0.57) | 0.373 (0.50, 0.57) | 0.380 (0.52, 0.59) | 0.389 (0.53, 0.59) | 0.037 (0.28, 0.52) | 0.028 (0.34, 0.53) | 0.037 (0.28, 0.52) | 0.028 (0.35, 0.54) |
| ransomware_syndicate | 0.183 (0.08, 0.15) | 0.160 (0.08, 0.15) | 0.192 (0.07, 0.15) | 0.183 (0.07, 0.15) | 0.174 (0.08, 0.18) | 0.190 (0.07, 0.16) | 0.185 (0.07, 0.16) | 0.190 (0.07, 0.15) | 0.197 (0.07, 0.14) | 0.192 (0.07, 0.13) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) |
| exploit_broker | 0.056 (0.06, 0.13) | 0.053 (0.08, 0.17) | 0.046 (0.07, 0.16) | 0.065 (0.06, 0.17) | 0.051 (0.06, 0.16) | 0.042 (0.07, 0.16) | 0.046 (0.06, 0.16) | 0.046 (0.07, 0.17) | 0.053 (0.08, 0.16) | 0.053 (0.07, 0.16) | 0.183 (0.01, 0.07) | 0.183 (0.02, 0.06) | 0.183 (0.01, 0.07) | 0.181 (0.02, 0.06) |
| command_and_control_grid | 0.218 (0.43, 0.55) | 0.206 (0.41, 0.54) | 0.218 (0.42, 0.55) | 0.218 (0.38, 0.54) | 0.204 (0.38, 0.55) | 0.215 (0.38, 0.54) | 0.211 (0.38, 0.54) | 0.211 (0.44, 0.56) | 0.231 (0.40, 0.52) | 0.220 (0.40, 0.52) | 0.132 (0.25, 0.52) | 0.150 (0.28, 0.50) | 0.130 (0.25, 0.52) | 0.146 (0.28, 0.51) |
| capacity_market | 0.113 (0.06, 0.12) | 0.160 (0.07, 0.12) | 0.164 (0.07, 0.13) | 0.157 (0.06, 0.13) | 0.160 (0.06, 0.13) | 0.160 (0.06, 0.12) | 0.160 (0.05, 0.11) | 0.153 (0.06, 0.11) | 0.157 (0.08, 0.13) | 0.155 (0.07, 0.12) | 0.002 (0.00, 0.06) | 0.000 (0.01, 0.07) | 0.002 (0.00, 0.06) | 0.000 (0.01, 0.07) |
| dispatch_control_room | 0.012 (0.03, 0.19) | 0.014 (0.03, 0.17) | 0.007 (0.03, 0.16) | 0.012 (0.03, 0.17) | 0.012 (0.03, 0.18) | 0.007 (0.03, 0.16) | 0.007 (0.03, 0.16) | 0.016 (0.03, 0.14) | 0.009 (0.03, 0.15) | 0.012 (0.03, 0.15) | 0.000 (0.00, 0.10) | 0.002 (0.00, 0.09) | 0.000 (0.00, 0.10) | 0.002 (0.00, 0.09) |
| smart_grid_backbone | 0.030 (0.10, 0.60) | 0.035 (0.09, 0.59) | 0.030 (0.09, 0.56) | 0.028 (0.08, 0.58) | 0.028 (0.08, 0.59) | 0.023 (0.08, 0.57) | 0.021 (0.08, 0.58) | 0.025 (0.07, 0.58) | 0.019 (0.06, 0.59) | 0.023 (0.08, 0.60) | 0.000 (0.01, 0.57) | 0.000 (0.00, 0.55) | 0.000 (0.01, 0.57) | 0.000 (0.00, 0.56) |
| reconciliation_office | 0.185 (0.23, 0.13) | 0.222 (0.25, 0.15) | 0.211 (0.27, 0.17) | 0.231 (0.24, 0.16) | 0.194 (0.27, 0.18) | 0.215 (0.25, 0.16) | 0.218 (0.25, 0.16) | 0.208 (0.27, 0.17) | 0.167 (0.26, 0.16) | 0.183 (0.26, 0.16) | 0.396 (0.18, 0.08) | 0.389 (0.19, 0.10) | 0.396 (0.18, 0.08) | 0.387 (0.19, 0.09) |
| protocol_office | 0.368 (0.30, 0.13) | 0.363 (0.30, 0.14) | 0.373 (0.31, 0.16) | 0.380 (0.30, 0.16) | 0.350 (0.31, 0.15) | 0.361 (0.30, 0.16) | 0.368 (0.30, 0.16) | 0.396 (0.29, 0.13) | 0.373 (0.31, 0.14) | 0.382 (0.30, 0.14) | 0.463 (0.23, 0.07) | 0.472 (0.24, 0.07) | 0.463 (0.23, 0.07) | 0.465 (0.24, 0.07) |
| joint_task_force | 0.021 (0.08, 0.17) | 0.021 (0.09, 0.18) | 0.016 (0.09, 0.16) | 0.021 (0.08, 0.18) | 0.021 (0.10, 0.19) | 0.021 (0.09, 0.18) | 0.019 (0.09, 0.18) | 0.021 (0.08, 0.16) | 0.014 (0.06, 0.17) | 0.012 (0.06, 0.16) | 0.118 (0.02, 0.05) | 0.148 (0.02, 0.06) | 0.118 (0.02, 0.05) | 0.148 (0.02, 0.06) |
| defense_appropriations | 0.127 (0.08, 0.14) | 0.095 (0.07, 0.16) | 0.111 (0.07, 0.15) | 0.111 (0.08, 0.17) | 0.123 (0.08, 0.17) | 0.123 (0.07, 0.17) | 0.120 (0.07, 0.17) | 0.102 (0.07, 0.16) | 0.123 (0.08, 0.14) | 0.104 (0.08, 0.15) | 0.007 (0.03, 0.09) | 0.005 (0.04, 0.10) | 0.007 (0.03, 0.09) | 0.005 (0.04, 0.10) |
| homeland_security_act | 0.060 (0.20, 0.53) | 0.088 (0.20, 0.51) | 0.058 (0.19, 0.51) | 0.076 (0.19, 0.50) | 0.072 (0.22, 0.52) | 0.081 (0.22, 0.51) | 0.076 (0.21, 0.51) | 0.046 (0.18, 0.53) | 0.060 (0.19, 0.53) | 0.056 (0.18, 0.53) | 0.000 (0.04, 0.49) | 0.000 (0.06, 0.49) | 0.000 (0.04, 0.49) | 0.000 (0.06, 0.50) |
| preprint_server | 0.072 (0.10, 0.14) | 0.079 (0.09, 0.13) | 0.062 (0.09, 0.14) | 0.100 (0.11, 0.14) | 0.076 (0.10, 0.15) | 0.079 (0.10, 0.13) | 0.079 (0.10, 0.12) | 0.069 (0.10, 0.14) | 0.069 (0.09, 0.14) | 0.062 (0.09, 0.14) | 0.227 (0.03, 0.07) | 0.241 (0.03, 0.07) | 0.227 (0.03, 0.07) | 0.243 (0.03, 0.07) |
| supplier_network | 0.164 (0.12, 0.10) | 0.167 (0.13, 0.12) | 0.181 (0.12, 0.11) | 0.160 (0.13, 0.12) | 0.181 (0.13, 0.13) | 0.164 (0.13, 0.13) | 0.164 (0.13, 0.13) | 0.169 (0.12, 0.12) | 0.164 (0.12, 0.13) | 0.141 (0.12, 0.14) | 0.310 (0.10, 0.08) | 0.317 (0.10, 0.08) | 0.310 (0.10, 0.08) | 0.324 (0.10, 0.08) |
| fusion_research_campus | 0.104 (0.31, 0.58) | 0.097 (0.28, 0.55) | 0.086 (0.31, 0.59) | 0.088 (0.19, 0.54) | 0.083 (0.21, 0.57) | 0.090 (0.20, 0.55) | 0.090 (0.20, 0.55) | 0.086 (0.30, 0.58) | 0.090 (0.29, 0.57) | 0.086 (0.30, 0.58) | 0.005 (0.01, 0.55) | 0.016 (0.06, 0.56) | 0.005 (0.01, 0.55) | 0.016 (0.06, 0.56) |
| hyperscale_botnet | 0.132 (0.39, 0.52) | 0.005 (0.39, 0.49) | 0.049 (0.41, 0.52) | 0.009 (0.36, 0.49) | 0.056 (0.35, 0.50) | 0.007 (0.34, 0.47) | 0.007 (0.32, 0.47) | 0.046 (0.43, 0.54) | 0.049 (0.40, 0.52) | 0.051 (0.40, 0.51) | 0.116 (0.25, 0.48) | 0.111 (0.28, 0.46) | 0.116 (0.25, 0.48) | 0.111 (0.28, 0.46) |
| moonshot_grant | 0.019 (0.07, 0.15) | 0.012 (0.07, 0.15) | 0.021 (0.07, 0.16) | 0.019 (0.03, 0.16) | 0.030 (0.02, 0.14) | 0.012 (0.03, 0.16) | 0.016 (0.03, 0.16) | 0.019 (0.07, 0.17) | 0.021 (0.07, 0.17) | 0.016 (0.07, 0.16) | 0.007 (0.00, 0.07) | 0.005 (0.02, 0.08) | 0.007 (0.00, 0.07) | 0.005 (0.02, 0.08) |

Played 0 times: strategist:w.insight_income=1.5,w.insight_cap=10,keep_spare=1: data_licensing_deal; strategist:w.insight_cap=10,keep_spare=1: data_licensing_deal; strategist:w.insight_income=1,w.insight_cap=10,keep_spare=1: data_licensing_deal; strategist: data_licensing_deal; strategist:w.insight_income=1: data_licensing_deal; strategist:w.limit_room=1,plan_limits=1: data_licensing_deal; strategist:w.limit_room=1,plan_limits=1,w.upkeep_loss=1: data_licensing_deal; strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.threshold_tokens=1: data_licensing_deal; strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1: data_licensing_deal; strategist:w.strat_contents=1,w.insight_income=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1,w.team_share=1,plan_team=1,w.threshold_tokens=1: data_licensing_deal; greedy: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; greedy:w.insight_cap=10,keep_spare=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, capacity_market, smart_grid_backbone, homeland_security_act; greedy:plan_limits=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, capacity_market, smart_grid_backbone, homeland_security_act
