## Win and rounds

| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | bp / br / pr win % |
|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5 | 432 | 67.8 | 6.07 | -2.8 ± 2.2 | +0.06 | 67 / 63 / 73 |
| strategist:w.threshold_tokens=1 | 432 | 70.6 | 6.03 | +0.0 ± 0.9 | +0.01 | 62 / 69 / 80 |
| strategist:w.insight_cap=10,keep_spare=1 | 432 | 70.4 | 5.96 | -0.2 ± 1.9 | -0.06 | 63 / 70 / 78 |
| strategist | 432 | 70.6 | 6.02 | - | - | 63 / 70 / 78 |
| strategist:w.strat_contents=1 | 432 | 73.1 | 6.04 | +2.5 ± 1.2 | +0.03 | 65 / 73 / 82 |
| strategist:w.team_share=1,plan_team=1 | 432 | 69.7 | 6.01 | -0.9 ± 1.9 | -0.01 | 65 / 66 / 78 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | 432 | 71.1 | 6.03 | +0.5 ± 2.5 | +0.01 | 65 / 68 / 81 |
| strategist:w.limit_room=1,plan_limits=1 | 432 | 70.1 | 6.02 | -0.5 ± 0.7 | +0.00 | 63 / 69 / 78 |
| strategist:w.patience=0.5 | 432 | 69.7 | 6.08 | -0.9 ± 1.5 | +0.06 | 63 / 69 / 77 |
| strategist:w.upkeep_loss=1 | 432 | 71.1 | 6.01 | +0.5 ± 0.7 | -0.00 | 62 / 71 / 80 |
| greedy | 432 | 49.1 | 4.91 | -21.5 ± 2.9 | -1.11 | 53 / 44 / 51 |
| greedy:w.insight_cap=10,keep_spare=1 | 432 | 48.6 | 4.93 | -22.0 ± 3.1 | -1.09 | 53 / 44 / 49 |
| greedy:plan_limits=1 | 432 | 49.1 | 4.91 | -21.5 ± 2.9 | -1.11 | 53 / 44 / 51 |
| greedy:w.upkeep_loss=1 | 432 | 49.1 | 4.91 | -21.5 ± 2.9 | -1.11 | 53 / 44 / 51 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 432 | 48.1 | 4.94 | -22.5 ± 3.1 | -1.07 | 53 / 44 / 48 |

## Lost at the limits, a game

| cell | hold_lost.credits | hold_lost.energy | hold_lost.chips | hold_lost.ambassadors | production_lost.credits | energy: games cut |
|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5 | 2.66 | 4.55 | 1.80 | 0.36 | 0.81 | 55% |
| strategist:w.threshold_tokens=1 | 1.79 | 4.11 | 1.40 | 0.39 | 0.41 | 55% |
| strategist:w.insight_cap=10,keep_spare=1 | 1.73 | 4.52 | 1.24 | 0.41 | 0.48 | 58% |
| strategist | 1.41 | 4.37 | 1.27 | 0.46 | 0.42 | 56% |
| strategist:w.strat_contents=1 | 1.38 | 4.48 | 1.26 | 0.41 | 0.33 | 58% |
| strategist:w.team_share=1,plan_team=1 | 2.02 | 4.02 | 1.61 | 0.53 | 0.66 | 56% |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | 1.25 | 3.22 | 1.07 | 0.35 | 0.47 | 52% |
| strategist:w.limit_room=1,plan_limits=1 | 1.69 | 4.28 | 1.50 | 0.25 | 0.50 | 57% |
| strategist:w.patience=0.5 | 1.37 | 4.88 | 1.44 | 0.25 | 0.51 | 59% |
| strategist:w.upkeep_loss=1 | 1.57 | 3.89 | 1.58 | 0.32 | 0.55 | 56% |
| greedy | 0.00 | 0.57 | 0.05 | 0.20 | 0.00 | 14% |
| greedy:w.insight_cap=10,keep_spare=1 | 0.00 | 0.53 | 0.19 | 0.28 | 0.00 | 14% |
| greedy:plan_limits=1 | 0.00 | 0.56 | 0.05 | 0.20 | 0.00 | 14% |
| greedy:w.upkeep_loss=1 | 0.00 | 0.54 | 0.04 | 0.15 | 0.00 | 14% |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | 0.00 | 0.53 | 0.20 | 0.36 | 0.00 | 14% |

## Peaks per seat (mean / highest)

| cell | board | credits | insight | energy | chips | ambassadors |
|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5 | Builder | 65.2 / 150 | 10.5 / 33 | 8.1 / 10 | 4.8 / 10 | 3.7 / 10 |
| strategist:w.insight_income=1.5 | Protector | 57.3 / 150 | 12.7 / 37 | 6.6 / 10 | 3.5 / 10 | 4.5 / 10 |
| strategist:w.insight_income=1.5 | Researcher | 61.9 / 150 | 35.0 / 146 | 6.7 / 10 | 3.5 / 10 | 4.4 / 10 |
| strategist:w.threshold_tokens=1 | Builder | 65.9 / 150 | 9.4 / 25 | 8.5 / 10 | 4.9 / 10 | 3.8 / 10 |
| strategist:w.threshold_tokens=1 | Protector | 56.9 / 150 | 11.5 / 27 | 6.7 / 10 | 3.4 / 10 | 4.6 / 10 |
| strategist:w.threshold_tokens=1 | Researcher | 65.2 / 150 | 28.8 / 123 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |
| strategist:w.insight_cap=10,keep_spare=1 | Builder | 65.2 / 150 | 9.1 / 22 | 8.5 / 10 | 5.0 / 10 | 3.7 / 10 |
| strategist:w.insight_cap=10,keep_spare=1 | Protector | 56.7 / 150 | 11.0 / 27 | 6.8 / 10 | 3.3 / 10 | 4.4 / 10 |
| strategist:w.insight_cap=10,keep_spare=1 | Researcher | 64.2 / 150 | 19.3 / 76 | 6.8 / 10 | 3.6 / 10 | 4.6 / 10 |
| strategist | Builder | 65.4 / 150 | 9.4 / 29 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| strategist | Protector | 56.8 / 150 | 11.5 / 31 | 6.8 / 10 | 3.3 / 10 | 4.5 / 10 |
| strategist | Researcher | 65.2 / 150 | 28.8 / 78 | 6.9 / 10 | 3.9 / 10 | 4.5 / 10 |
| strategist:w.strat_contents=1 | Builder | 66.4 / 150 | 9.4 / 23 | 8.5 / 10 | 5.1 / 10 | 3.7 / 10 |
| strategist:w.strat_contents=1 | Protector | 56.7 / 150 | 11.6 / 32 | 6.7 / 10 | 3.4 / 10 | 4.5 / 10 |
| strategist:w.strat_contents=1 | Researcher | 65.7 / 150 | 29.6 / 81 | 6.9 / 10 | 3.8 / 10 | 4.6 / 10 |
| strategist:w.team_share=1,plan_team=1 | Builder | 63.8 / 150 | 9.4 / 40 | 8.4 / 10 | 4.9 / 10 | 3.5 / 10 |
| strategist:w.team_share=1,plan_team=1 | Protector | 57.0 / 150 | 11.4 / 30 | 6.7 / 10 | 3.3 / 10 | 4.5 / 10 |
| strategist:w.team_share=1,plan_team=1 | Researcher | 65.1 / 150 | 29.4 / 104 | 7.1 / 10 | 3.9 / 10 | 4.7 / 10 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | Builder | 64.3 / 150 | 10.0 / 24 | 8.3 / 10 | 5.1 / 10 | 3.6 / 10 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | Protector | 56.9 / 150 | 12.4 / 30 | 6.6 / 10 | 3.5 / 10 | 4.6 / 10 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | Researcher | 63.6 / 150 | 21.7 / 69 | 6.6 / 10 | 3.5 / 10 | 4.3 / 10 |
| strategist:w.limit_room=1,plan_limits=1 | Builder | 65.5 / 150 | 9.3 / 23 | 8.5 / 10 | 5.0 / 10 | 3.6 / 10 |
| strategist:w.limit_room=1,plan_limits=1 | Protector | 57.0 / 150 | 11.5 / 37 | 6.7 / 10 | 3.3 / 10 | 4.6 / 10 |
| strategist:w.limit_room=1,plan_limits=1 | Researcher | 64.6 / 150 | 28.6 / 97 | 6.9 / 10 | 3.8 / 10 | 4.5 / 10 |
| strategist:w.patience=0.5 | Builder | 65.7 / 150 | 9.4 / 29 | 8.5 / 10 | 5.0 / 10 | 3.7 / 10 |
| strategist:w.patience=0.5 | Protector | 56.9 / 150 | 11.6 / 31 | 6.8 / 10 | 3.5 / 10 | 4.6 / 10 |
| strategist:w.patience=0.5 | Researcher | 65.6 / 150 | 24.6 / 89 | 6.8 / 10 | 3.9 / 10 | 4.5 / 10 |
| strategist:w.upkeep_loss=1 | Builder | 65.9 / 150 | 9.5 / 34 | 8.5 / 10 | 4.9 / 10 | 3.7 / 10 |
| strategist:w.upkeep_loss=1 | Protector | 56.7 / 150 | 11.1 / 31 | 6.7 / 10 | 3.2 / 10 | 4.5 / 10 |
| strategist:w.upkeep_loss=1 | Researcher | 64.4 / 150 | 28.9 / 97 | 6.9 / 10 | 3.8 / 10 | 4.6 / 10 |
| greedy | Builder | 42.7 / 116 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| greedy | Protector | 48.7 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.4 / 10 |
| greedy | Researcher | 43.3 / 65 | 16.5 / 48 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| greedy:w.insight_cap=10,keep_spare=1 | Builder | 42.8 / 116 | 8.2 / 16 | 3.8 / 10 | 3.1 / 10 | 4.5 / 10 |
| greedy:w.insight_cap=10,keep_spare=1 | Protector | 49.0 / 88 | 10.0 / 26 | 5.4 / 10 | 2.8 / 10 | 4.3 / 10 |
| greedy:w.insight_cap=10,keep_spare=1 | Researcher | 43.6 / 67 | 14.7 / 62 | 6.2 / 10 | 2.4 / 10 | 4.8 / 10 |
| greedy:plan_limits=1 | Builder | 42.7 / 116 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| greedy:plan_limits=1 | Protector | 48.6 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.4 / 10 |
| greedy:plan_limits=1 | Researcher | 43.3 / 65 | 16.5 / 48 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| greedy:w.upkeep_loss=1 | Builder | 42.6 / 113 | 8.2 / 16 | 3.7 / 10 | 3.0 / 10 | 4.6 / 10 |
| greedy:w.upkeep_loss=1 | Protector | 48.8 / 85 | 10.1 / 26 | 5.3 / 10 | 2.7 / 10 | 4.5 / 10 |
| greedy:w.upkeep_loss=1 | Researcher | 43.3 / 65 | 16.7 / 63 | 6.1 / 10 | 2.2 / 10 | 4.6 / 10 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Builder | 42.8 / 113 | 8.2 / 16 | 3.8 / 10 | 3.1 / 10 | 4.5 / 10 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Protector | 49.0 / 91 | 10.0 / 21 | 5.4 / 10 | 2.8 / 10 | 4.3 / 10 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | Researcher | 43.5 / 67 | 14.9 / 62 | 6.2 / 10 | 2.4 / 10 | 4.7 / 10 |

## The Researcher: keeps against insight

Flag rules: `peak` is 122's (insight peak over 3x a round's insight income and over 10); `rise` is the fixed one (the peak's rise over the starting holding, same test). Keep rate: kept / offered after the opening; tier-2 share of its tier choices; insight at keep: mean held at its keep decisions.

| cell | rule | seats | n | insight peak | offered | kept | keep rate | tier-2 share | insight at keep |
|---|---|---|---|---|---|---|---|---|---|
| strategist:w.insight_income=1.5 | peak | not flagged | 96 | 19.8 | 16.4 | 10.7 | 65% | 59% | 13.7 |
| strategist:w.insight_income=1.5 | peak | flagged | 192 | 42.6 | 21.2 | 11.7 | 55% | 57% | 22.4 |
| strategist:w.insight_income=1.5 | peak | all | 288 | 35.0 | 19.6 | 11.4 | 58% | 57% | 19.5 |
| strategist:w.insight_income=1.5 | rise | not flagged | 167 | 23.4 | 17.0 | 10.5 | 62% | 57% | 15.2 |
| strategist:w.insight_income=1.5 | rise | flagged | 121 | 51.1 | 23.3 | 12.6 | 54% | 58% | 25.4 |
| strategist:w.threshold_tokens=1 | peak | not flagged | 122 | 17.9 | 17.6 | 11.5 | 65% | 59% | 12.8 |
| strategist:w.threshold_tokens=1 | peak | flagged | 166 | 36.8 | 21.2 | 11.5 | 54% | 58% | 20.6 |
| strategist:w.threshold_tokens=1 | peak | all | 288 | 28.8 | 19.7 | 11.5 | 58% | 58% | 17.3 |
| strategist:w.threshold_tokens=1 | rise | not flagged | 190 | 20.9 | 17.6 | 10.8 | 61% | 58% | 14.1 |
| strategist:w.threshold_tokens=1 | rise | flagged | 98 | 44.1 | 23.8 | 12.8 | 54% | 59% | 23.5 |
| strategist:w.insight_cap=10,keep_spare=1 | peak | not flagged | 231 | 17.3 | 18.7 | 13.1 | 70% | 60% | 13.1 |
| strategist:w.insight_cap=10,keep_spare=1 | peak | flagged | 57 | 27.6 | 21.4 | 16.0 | 75% | 61% | 17.3 |
| strategist:w.insight_cap=10,keep_spare=1 | peak | all | 288 | 19.3 | 19.2 | 13.7 | 71% | 60% | 14.0 |
| strategist:w.insight_cap=10,keep_spare=1 | rise | not flagged | 274 | 18.3 | 19.0 | 13.4 | 71% | 60% | 13.6 |
| strategist:w.insight_cap=10,keep_spare=1 | rise | flagged | 14 | 39.4 | 24.0 | 19.3 | 80% | 61% | 21.2 |
| strategist | peak | not flagged | 118 | 18.3 | 17.7 | 11.8 | 67% | 58% | 13.0 |
| strategist | peak | flagged | 170 | 36.0 | 20.9 | 11.3 | 54% | 58% | 20.3 |
| strategist | peak | all | 288 | 28.8 | 19.6 | 11.5 | 59% | 58% | 17.3 |
| strategist | rise | not flagged | 194 | 21.6 | 17.7 | 11.1 | 62% | 58% | 14.3 |
| strategist | rise | flagged | 94 | 43.6 | 23.4 | 12.5 | 53% | 59% | 23.4 |
| strategist:w.strat_contents=1 | peak | not flagged | 113 | 18.2 | 17.3 | 11.6 | 67% | 59% | 12.7 |
| strategist:w.strat_contents=1 | peak | flagged | 175 | 37.0 | 21.5 | 11.7 | 55% | 59% | 20.6 |
| strategist:w.strat_contents=1 | peak | all | 288 | 29.6 | 19.8 | 11.7 | 59% | 59% | 17.5 |
| strategist:w.strat_contents=1 | rise | not flagged | 190 | 21.7 | 17.7 | 11.1 | 62% | 58% | 14.2 |
| strategist:w.strat_contents=1 | rise | flagged | 98 | 45.1 | 23.9 | 12.9 | 54% | 59% | 23.9 |
| strategist:w.team_share=1,plan_team=1 | peak | not flagged | 120 | 18.3 | 17.2 | 11.1 | 65% | 58% | 13.1 |
| strategist:w.team_share=1,plan_team=1 | peak | flagged | 168 | 37.4 | 21.2 | 11.8 | 56% | 58% | 20.6 |
| strategist:w.team_share=1,plan_team=1 | peak | all | 288 | 29.4 | 19.5 | 11.5 | 59% | 58% | 17.5 |
| strategist:w.team_share=1,plan_team=1 | rise | not flagged | 193 | 21.8 | 17.5 | 10.8 | 62% | 58% | 14.4 |
| strategist:w.team_share=1,plan_team=1 | rise | flagged | 95 | 44.9 | 23.7 | 13.0 | 55% | 58% | 23.7 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 206 | 19.2 | 18.8 | 13.7 | 73% | 61% | 14.0 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | peak | flagged | 82 | 27.8 | 21.1 | 15.6 | 74% | 61% | 18.2 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 21.7 | 19.5 | 14.3 | 73% | 61% | 15.2 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 267 | 20.3 | 19.1 | 14.0 | 73% | 61% | 14.7 |
| strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | rise | flagged | 21 | 39.1 | 23.8 | 18.2 | 76% | 62% | 22.3 |
| strategist:w.limit_room=1,plan_limits=1 | peak | not flagged | 120 | 18.0 | 17.5 | 11.5 | 66% | 59% | 12.9 |
| strategist:w.limit_room=1,plan_limits=1 | peak | flagged | 168 | 36.2 | 21.1 | 11.6 | 55% | 58% | 20.4 |
| strategist:w.limit_room=1,plan_limits=1 | peak | all | 288 | 28.6 | 19.6 | 11.6 | 59% | 58% | 17.3 |
| strategist:w.limit_room=1,plan_limits=1 | rise | not flagged | 195 | 21.1 | 17.6 | 11.0 | 62% | 58% | 14.2 |
| strategist:w.limit_room=1,plan_limits=1 | rise | flagged | 93 | 44.5 | 23.8 | 12.7 | 53% | 59% | 23.7 |
| strategist:w.patience=0.5 | peak | not flagged | 137 | 16.4 | 18.0 | 11.7 | 65% | 61% | 11.7 |
| strategist:w.patience=0.5 | peak | flagged | 151 | 32.0 | 21.7 | 11.4 | 53% | 58% | 19.3 |
| strategist:w.patience=0.5 | peak | all | 288 | 24.6 | 20.0 | 11.6 | 58% | 59% | 15.7 |
| strategist:w.patience=0.5 | rise | not flagged | 213 | 19.2 | 18.3 | 11.1 | 61% | 59% | 13.2 |
| strategist:w.patience=0.5 | rise | flagged | 75 | 39.9 | 24.5 | 12.9 | 52% | 59% | 22.6 |
| strategist:w.upkeep_loss=1 | peak | not flagged | 118 | 18.3 | 17.5 | 11.5 | 66% | 58% | 12.9 |
| strategist:w.upkeep_loss=1 | peak | flagged | 170 | 36.3 | 21.1 | 11.5 | 54% | 58% | 20.4 |
| strategist:w.upkeep_loss=1 | peak | all | 288 | 28.9 | 19.6 | 11.5 | 59% | 58% | 17.3 |
| strategist:w.upkeep_loss=1 | rise | not flagged | 192 | 21.2 | 17.5 | 10.9 | 62% | 58% | 14.2 |
| strategist:w.upkeep_loss=1 | rise | flagged | 96 | 44.3 | 23.8 | 12.8 | 54% | 59% | 23.5 |
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
| greedy:w.upkeep_loss=1 | peak | not flagged | 139 | 12.2 | 13.8 | 5.0 | 36% | 76% | 9.5 |
| greedy:w.upkeep_loss=1 | peak | flagged | 149 | 20.9 | 15.1 | 4.2 | 28% | 77% | 13.7 |
| greedy:w.upkeep_loss=1 | peak | all | 288 | 16.7 | 14.5 | 4.6 | 32% | 77% | 11.7 |
| greedy:w.upkeep_loss=1 | rise | not flagged | 251 | 15.0 | 14.1 | 4.6 | 33% | 76% | 11.0 |
| greedy:w.upkeep_loss=1 | rise | flagged | 37 | 27.7 | 16.9 | 4.4 | 26% | 79% | 16.2 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | not flagged | 181 | 13.2 | 14.6 | 6.5 | 45% | 76% | 10.1 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | flagged | 107 | 17.7 | 14.7 | 6.9 | 47% | 78% | 12.7 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | peak | all | 288 | 14.9 | 14.6 | 6.7 | 46% | 77% | 11.1 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | not flagged | 283 | 14.5 | 14.5 | 6.6 | 45% | 77% | 11.0 |
| greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 | rise | flagged | 5 | 38.0 | 21.4 | 11.8 | 55% | 85% | 19.7 |

## Plays a game (kept, offered): the r41 engine cards and the two named

| card | strategist:w.insight_income=1.5 | strategist:w.threshold_tokens=1 | strategist:w.insight_cap=10,keep_spare=1 | strategist | strategist:w.strat_contents=1 | strategist:w.team_share=1,plan_team=1 | strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1 | strategist:w.limit_room=1,plan_limits=1 | strategist:w.patience=0.5 | strategist:w.upkeep_loss=1 | greedy | greedy:w.insight_cap=10,keep_spare=1 | greedy:plan_limits=1 | greedy:w.upkeep_loss=1 | greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| telemetry_archive | 0.037 (0.20, 0.19) | 0.032 (0.20, 0.17) | 0.035 (0.19, 0.17) | 0.032 (0.20, 0.17) | 0.035 (0.20, 0.18) | 0.053 (0.18, 0.16) | 0.025 (0.19, 0.17) | 0.035 (0.20, 0.18) | 0.032 (0.18, 0.16) | 0.035 (0.20, 0.17) | 0.007 (0.09, 0.08) | 0.007 (0.10, 0.09) | 0.007 (0.09, 0.08) | 0.007 (0.09, 0.08) | 0.007 (0.10, 0.09) |
| citation_engine | 0.069 (0.11, 0.15) | 0.060 (0.10, 0.15) | 0.060 (0.11, 0.17) | 0.069 (0.10, 0.15) | 0.056 (0.10, 0.15) | 0.076 (0.09, 0.16) | 0.056 (0.10, 0.16) | 0.074 (0.11, 0.15) | 0.065 (0.09, 0.15) | 0.069 (0.11, 0.16) | 0.141 (0.05, 0.09) | 0.155 (0.04, 0.06) | 0.141 (0.05, 0.09) | 0.144 (0.05, 0.09) | 0.155 (0.04, 0.06) |
| grad_student_pool | 0.312 (0.22, 0.12) | 0.317 (0.21, 0.16) | 0.319 (0.21, 0.13) | 0.308 (0.22, 0.17) | 0.299 (0.23, 0.16) | 0.310 (0.21, 0.14) | 0.299 (0.22, 0.13) | 0.308 (0.22, 0.17) | 0.312 (0.22, 0.15) | 0.308 (0.21, 0.15) | 0.398 (0.17, 0.09) | 0.380 (0.17, 0.08) | 0.398 (0.17, 0.09) | 0.398 (0.17, 0.08) | 0.380 (0.17, 0.08) |
| foundation_model_campus | 0.171 (0.34, 0.51) | 0.155 (0.31, 0.52) | 0.171 (0.31, 0.50) | 0.160 (0.31, 0.51) | 0.160 (0.32, 0.52) | 0.183 (0.34, 0.53) | 0.194 (0.41, 0.57) | 0.164 (0.30, 0.50) | 0.188 (0.33, 0.51) | 0.164 (0.29, 0.49) | 0.058 (0.12, 0.49) | 0.076 (0.14, 0.50) | 0.058 (0.11, 0.49) | 0.062 (0.11, 0.48) | 0.074 (0.14, 0.49) |
| data_licensing_deal | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) | 0.000 (0.00, 0.00) |
| retained_earnings | 0.211 (0.21, 0.18) | 0.222 (0.19, 0.16) | 0.234 (0.19, 0.16) | 0.225 (0.19, 0.16) | 0.243 (0.21, 0.16) | 0.220 (0.18, 0.14) | 0.220 (0.24, 0.17) | 0.218 (0.19, 0.17) | 0.222 (0.19, 0.17) | 0.215 (0.19, 0.16) | 0.005 (0.10, 0.07) | 0.005 (0.10, 0.08) | 0.005 (0.10, 0.07) | 0.005 (0.10, 0.07) | 0.005 (0.10, 0.08) |
| lights_out_assembly | 0.435 (0.29, 0.17) | 0.403 (0.28, 0.15) | 0.428 (0.28, 0.13) | 0.410 (0.27, 0.14) | 0.403 (0.27, 0.14) | 0.412 (0.30, 0.17) | 0.391 (0.32, 0.19) | 0.400 (0.27, 0.14) | 0.428 (0.28, 0.14) | 0.410 (0.28, 0.14) | 0.410 (0.23, 0.09) | 0.428 (0.22, 0.08) | 0.410 (0.23, 0.09) | 0.414 (0.23, 0.10) | 0.431 (0.22, 0.08) |
| honeypot_relay | 0.201 (0.24, 0.15) | 0.231 (0.23, 0.14) | 0.220 (0.25, 0.15) | 0.225 (0.24, 0.16) | 0.225 (0.25, 0.17) | 0.218 (0.24, 0.15) | 0.199 (0.24, 0.12) | 0.225 (0.24, 0.15) | 0.218 (0.24, 0.16) | 0.225 (0.24, 0.16) | 0.412 (0.20, 0.08) | 0.419 (0.21, 0.07) | 0.412 (0.20, 0.08) | 0.414 (0.20, 0.08) | 0.426 (0.21, 0.07) |
| neural_mirror | 0.248 (0.08, 0.15) | 0.192 (0.08, 0.12) | 0.181 (0.09, 0.12) | 0.190 (0.08, 0.13) | 0.188 (0.08, 0.12) | 0.188 (0.09, 0.14) | 0.236 (0.12, 0.15) | 0.183 (0.09, 0.13) | 0.190 (0.09, 0.12) | 0.194 (0.09, 0.13) | 0.019 (0.06, 0.07) | 0.021 (0.06, 0.07) | 0.019 (0.06, 0.07) | 0.019 (0.05, 0.07) | 0.023 (0.06, 0.07) |
| intrusion_lab | 0.023 (0.09, 0.17) | 0.060 (0.08, 0.14) | 0.023 (0.07, 0.13) | 0.035 (0.07, 0.13) | 0.028 (0.08, 0.13) | 0.030 (0.09, 0.16) | 0.044 (0.10, 0.17) | 0.025 (0.07, 0.14) | 0.032 (0.08, 0.15) | 0.028 (0.07, 0.13) | 0.023 (0.02, 0.07) | 0.023 (0.03, 0.07) | 0.023 (0.02, 0.07) | 0.019 (0.03, 0.07) | 0.021 (0.03, 0.07) |
| darknet_exchange | 0.245 (0.21, 0.15) | 0.238 (0.20, 0.16) | 0.252 (0.22, 0.16) | 0.248 (0.20, 0.16) | 0.259 (0.20, 0.15) | 0.271 (0.20, 0.15) | 0.252 (0.21, 0.13) | 0.231 (0.21, 0.17) | 0.266 (0.21, 0.17) | 0.245 (0.20, 0.15) | 0.368 (0.17, 0.07) | 0.387 (0.18, 0.08) | 0.368 (0.17, 0.07) | 0.363 (0.17, 0.07) | 0.384 (0.18, 0.08) |
| tidal_barrage | 0.025 (0.02, 0.14) | 0.025 (0.02, 0.12) | 0.021 (0.01, 0.14) | 0.021 (0.01, 0.12) | 0.021 (0.01, 0.11) | 0.021 (0.02, 0.14) | 0.014 (0.01, 0.13) | 0.030 (0.01, 0.12) | 0.030 (0.01, 0.13) | 0.028 (0.01, 0.12) | 0.000 (0.00, 0.10) | 0.000 (0.00, 0.09) | 0.000 (0.00, 0.10) | 0.000 (0.00, 0.10) | 0.000 (0.00, 0.09) |
| substation_hub | 0.144 (0.10, 0.13) | 0.146 (0.10, 0.14) | 0.141 (0.10, 0.13) | 0.139 (0.10, 0.13) | 0.141 (0.11, 0.15) | 0.116 (0.10, 0.13) | 0.106 (0.10, 0.15) | 0.144 (0.10, 0.12) | 0.144 (0.11, 0.14) | 0.132 (0.12, 0.14) | 0.037 (0.02, 0.06) | 0.037 (0.02, 0.06) | 0.037 (0.02, 0.06) | 0.037 (0.02, 0.06) | 0.039 (0.02, 0.06) |
| positron_storage_ring | 0.067 (0.25, 0.52) | 0.051 (0.24, 0.53) | 0.056 (0.19, 0.52) | 0.058 (0.23, 0.53) | 0.065 (0.23, 0.53) | 0.079 (0.25, 0.52) | 0.060 (0.23, 0.54) | 0.058 (0.24, 0.53) | 0.062 (0.20, 0.52) | 0.060 (0.24, 0.53) | 0.000 (0.06, 0.49) | 0.000 (0.05, 0.49) | 0.000 (0.06, 0.49) | 0.000 (0.06, 0.50) | 0.000 (0.05, 0.49) |
| continental_supergrid | 0.056 (0.17, 0.54) | 0.042 (0.16, 0.53) | 0.051 (0.17, 0.54) | 0.051 (0.16, 0.53) | 0.051 (0.16, 0.53) | 0.046 (0.15, 0.53) | 0.039 (0.16, 0.56) | 0.049 (0.16, 0.53) | 0.058 (0.17, 0.53) | 0.049 (0.16, 0.53) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) | 0.000 (0.00, 0.52) |
| open_assembly | 0.380 (0.22, 0.15) | 0.405 (0.22, 0.15) | 0.396 (0.21, 0.14) | 0.405 (0.22, 0.14) | 0.407 (0.22, 0.14) | 0.424 (0.22, 0.17) | 0.389 (0.22, 0.16) | 0.405 (0.21, 0.15) | 0.403 (0.22, 0.16) | 0.400 (0.22, 0.14) | 0.502 (0.16, 0.07) | 0.481 (0.16, 0.08) | 0.502 (0.16, 0.07) | 0.502 (0.16, 0.07) | 0.479 (0.16, 0.08) |
| peace_dividend | 0.190 (0.09, 0.15) | 0.178 (0.12, 0.18) | 0.178 (0.12, 0.17) | 0.183 (0.11, 0.17) | 0.199 (0.12, 0.16) | 0.204 (0.08, 0.14) | 0.167 (0.09, 0.12) | 0.176 (0.10, 0.16) | 0.201 (0.13, 0.17) | 0.197 (0.11, 0.16) | 0.025 (0.02, 0.07) | 0.012 (0.03, 0.08) | 0.025 (0.02, 0.07) | 0.023 (0.02, 0.07) | 0.012 (0.03, 0.08) |
| consulate_tower | 0.169 (0.11, 0.15) | 0.201 (0.08, 0.12) | 0.176 (0.08, 0.12) | 0.208 (0.09, 0.13) | 0.213 (0.09, 0.12) | 0.137 (0.09, 0.14) | 0.120 (0.09, 0.13) | 0.190 (0.09, 0.13) | 0.174 (0.07, 0.12) | 0.201 (0.09, 0.13) | 0.269 (0.03, 0.08) | 0.269 (0.03, 0.08) | 0.266 (0.03, 0.08) | 0.266 (0.03, 0.08) | 0.259 (0.03, 0.08) |
| orbital_embassy | 0.139 (0.28, 0.53) | 0.157 (0.31, 0.52) | 0.150 (0.37, 0.53) | 0.155 (0.30, 0.51) | 0.157 (0.31, 0.52) | 0.150 (0.33, 0.53) | 0.134 (0.39, 0.55) | 0.153 (0.30, 0.51) | 0.153 (0.32, 0.55) | 0.153 (0.30, 0.51) | 0.090 (0.16, 0.48) | 0.083 (0.18, 0.47) | 0.090 (0.16, 0.48) | 0.088 (0.16, 0.48) | 0.081 (0.18, 0.47) |
| agency_field_station | 0.120 (0.11, 0.14) | 0.123 (0.13, 0.16) | 0.116 (0.11, 0.12) | 0.123 (0.12, 0.15) | 0.102 (0.09, 0.14) | 0.120 (0.12, 0.14) | 0.079 (0.09, 0.14) | 0.123 (0.12, 0.15) | 0.137 (0.12, 0.15) | 0.123 (0.12, 0.15) | 0.312 (0.03, 0.08) | 0.338 (0.03, 0.09) | 0.312 (0.03, 0.08) | 0.317 (0.03, 0.08) | 0.343 (0.03, 0.09) |
| security_contractor_hq | 0.113 (0.11, 0.14) | 0.106 (0.10, 0.13) | 0.102 (0.10, 0.12) | 0.102 (0.09, 0.12) | 0.093 (0.09, 0.13) | 0.120 (0.09, 0.12) | 0.093 (0.11, 0.13) | 0.104 (0.09, 0.12) | 0.104 (0.11, 0.13) | 0.102 (0.09, 0.12) | 0.250 (0.04, 0.07) | 0.255 (0.04, 0.07) | 0.250 (0.04, 0.07) | 0.248 (0.04, 0.07) | 0.255 (0.04, 0.07) |
| agency_directorate | 0.030 (0.28, 0.54) | 0.046 (0.28, 0.53) | 0.060 (0.32, 0.52) | 0.051 (0.27, 0.53) | 0.046 (0.31, 0.53) | 0.053 (0.28, 0.54) | 0.042 (0.36, 0.54) | 0.042 (0.28, 0.53) | 0.065 (0.29, 0.56) | 0.058 (0.27, 0.53) | 0.046 (0.15, 0.51) | 0.051 (0.17, 0.50) | 0.046 (0.15, 0.51) | 0.049 (0.15, 0.50) | 0.051 (0.17, 0.50) |
| security_ops_floor | 0.030 (0.11, 0.13) | 0.037 (0.11, 0.12) | 0.032 (0.11, 0.13) | 0.035 (0.11, 0.14) | 0.037 (0.12, 0.13) | 0.032 (0.11, 0.14) | 0.030 (0.12, 0.15) | 0.035 (0.10, 0.13) | 0.039 (0.11, 0.12) | 0.032 (0.11, 0.13) | 0.181 (0.05, 0.09) | 0.204 (0.05, 0.09) | 0.181 (0.05, 0.09) | 0.183 (0.05, 0.09) | 0.204 (0.05, 0.09) |
| vertical_monopoly | 0.238 (0.36, 0.55) | 0.238 (0.36, 0.55) | 0.264 (0.40, 0.56) | 0.243 (0.37, 0.56) | 0.250 (0.39, 0.58) | 0.231 (0.37, 0.57) | 0.275 (0.42, 0.57) | 0.248 (0.36, 0.56) | 0.248 (0.37, 0.56) | 0.245 (0.37, 0.56) | 0.088 (0.11, 0.52) | 0.074 (0.13, 0.52) | 0.088 (0.11, 0.52) | 0.088 (0.12, 0.53) | 0.074 (0.14, 0.53) |
| power_to_the_people | 0.079 (0.18, 0.55) | 0.090 (0.19, 0.55) | 0.076 (0.22, 0.53) | 0.086 (0.18, 0.54) | 0.079 (0.19, 0.55) | 0.074 (0.18, 0.52) | 0.093 (0.24, 0.56) | 0.088 (0.18, 0.54) | 0.093 (0.18, 0.54) | 0.095 (0.19, 0.54) | 0.014 (0.03, 0.50) | 0.007 (0.08, 0.50) | 0.014 (0.03, 0.50) | 0.014 (0.03, 0.50) | 0.009 (0.09, 0.51) |
| research_royalties | 0.192 (0.09, 0.16) | 0.227 (0.10, 0.18) | 0.199 (0.09, 0.16) | 0.213 (0.10, 0.17) | 0.206 (0.11, 0.18) | 0.211 (0.08, 0.16) | 0.192 (0.07, 0.14) | 0.208 (0.10, 0.17) | 0.238 (0.10, 0.18) | 0.206 (0.09, 0.16) | 0.002 (0.02, 0.08) | 0.002 (0.03, 0.08) | 0.002 (0.02, 0.08) | 0.005 (0.02, 0.08) | 0.005 (0.03, 0.08) |
| interdisciplinary_institute | 0.262 (0.42, 0.56) | 0.234 (0.41, 0.55) | 0.236 (0.43, 0.54) | 0.236 (0.40, 0.54) | 0.236 (0.41, 0.55) | 0.255 (0.42, 0.56) | 0.271 (0.46, 0.58) | 0.238 (0.40, 0.54) | 0.220 (0.41, 0.56) | 0.231 (0.40, 0.54) | 0.213 (0.31, 0.53) | 0.225 (0.35, 0.55) | 0.213 (0.31, 0.53) | 0.215 (0.31, 0.54) | 0.227 (0.34, 0.55) |
| standardized_tooling | 0.377 (0.25, 0.15) | 0.394 (0.25, 0.16) | 0.359 (0.26, 0.16) | 0.384 (0.25, 0.16) | 0.375 (0.26, 0.16) | 0.389 (0.26, 0.16) | 0.356 (0.25, 0.15) | 0.382 (0.26, 0.16) | 0.382 (0.24, 0.15) | 0.363 (0.25, 0.16) | 0.458 (0.20, 0.07) | 0.444 (0.20, 0.06) | 0.458 (0.20, 0.07) | 0.454 (0.20, 0.07) | 0.444 (0.20, 0.06) |
| economies_of_scale | 0.248 (0.11, 0.16) | 0.227 (0.14, 0.19) | 0.229 (0.13, 0.16) | 0.236 (0.13, 0.19) | 0.245 (0.14, 0.17) | 0.208 (0.12, 0.16) | 0.225 (0.12, 0.14) | 0.234 (0.14, 0.19) | 0.227 (0.13, 0.19) | 0.227 (0.13, 0.19) | 0.009 (0.03, 0.07) | 0.005 (0.03, 0.07) | 0.009 (0.03, 0.07) | 0.009 (0.03, 0.07) | 0.005 (0.03, 0.07) |
| capital_projects_office | 0.451 (0.57, 0.49) | 0.433 (0.56, 0.49) | 0.419 (0.59, 0.51) | 0.410 (0.56, 0.50) | 0.428 (0.58, 0.52) | 0.426 (0.56, 0.49) | 0.463 (0.63, 0.54) | 0.410 (0.56, 0.50) | 0.444 (0.58, 0.52) | 0.407 (0.57, 0.50) | 0.507 (0.54, 0.46) | 0.532 (0.57, 0.49) | 0.507 (0.54, 0.46) | 0.509 (0.55, 0.47) | 0.532 (0.57, 0.49) |
| conglomerate_headquarters | 0.338 (0.47, 0.56) | 0.331 (0.46, 0.57) | 0.366 (0.50, 0.57) | 0.336 (0.45, 0.56) | 0.350 (0.44, 0.55) | 0.322 (0.45, 0.56) | 0.370 (0.50, 0.56) | 0.338 (0.44, 0.56) | 0.338 (0.47, 0.56) | 0.338 (0.45, 0.56) | 0.037 (0.28, 0.52) | 0.028 (0.34, 0.53) | 0.037 (0.28, 0.52) | 0.037 (0.28, 0.53) | 0.028 (0.35, 0.54) |
| ransomware_syndicate | 0.183 (0.07, 0.15) | 0.188 (0.06, 0.14) | 0.155 (0.07, 0.15) | 0.183 (0.07, 0.15) | 0.169 (0.08, 0.16) | 0.171 (0.08, 0.16) | 0.185 (0.08, 0.15) | 0.190 (0.07, 0.16) | 0.185 (0.06, 0.15) | 0.176 (0.06, 0.15) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) | 0.000 (0.02, 0.07) |
| exploit_broker | 0.053 (0.06, 0.15) | 0.065 (0.06, 0.16) | 0.053 (0.08, 0.17) | 0.065 (0.06, 0.17) | 0.065 (0.06, 0.16) | 0.060 (0.07, 0.17) | 0.044 (0.07, 0.17) | 0.049 (0.06, 0.16) | 0.060 (0.08, 0.17) | 0.058 (0.06, 0.16) | 0.183 (0.01, 0.07) | 0.183 (0.02, 0.06) | 0.183 (0.01, 0.07) | 0.178 (0.01, 0.07) | 0.181 (0.02, 0.06) |
| command_and_control_grid | 0.211 (0.36, 0.52) | 0.220 (0.38, 0.53) | 0.206 (0.41, 0.55) | 0.218 (0.38, 0.54) | 0.215 (0.40, 0.56) | 0.211 (0.37, 0.52) | 0.204 (0.45, 0.57) | 0.218 (0.38, 0.54) | 0.225 (0.39, 0.55) | 0.208 (0.38, 0.55) | 0.132 (0.25, 0.52) | 0.150 (0.28, 0.50) | 0.130 (0.25, 0.52) | 0.132 (0.25, 0.52) | 0.146 (0.28, 0.51) |
| capacity_market | 0.132 (0.05, 0.11) | 0.162 (0.05, 0.12) | 0.162 (0.07, 0.12) | 0.157 (0.06, 0.13) | 0.139 (0.06, 0.13) | 0.157 (0.05, 0.12) | 0.109 (0.09, 0.14) | 0.150 (0.06, 0.12) | 0.141 (0.07, 0.12) | 0.155 (0.06, 0.12) | 0.002 (0.00, 0.06) | 0.000 (0.01, 0.07) | 0.002 (0.00, 0.06) | 0.002 (0.00, 0.06) | 0.000 (0.01, 0.07) |
| dispatch_control_room | 0.009 (0.03, 0.18) | 0.012 (0.02, 0.17) | 0.014 (0.03, 0.16) | 0.012 (0.03, 0.17) | 0.016 (0.03, 0.16) | 0.014 (0.03, 0.17) | 0.009 (0.03, 0.16) | 0.009 (0.03, 0.17) | 0.009 (0.03, 0.16) | 0.009 (0.03, 0.17) | 0.000 (0.00, 0.10) | 0.002 (0.00, 0.09) | 0.000 (0.00, 0.10) | 0.000 (0.00, 0.10) | 0.002 (0.00, 0.09) |
| smart_grid_backbone | 0.042 (0.10, 0.58) | 0.019 (0.08, 0.59) | 0.032 (0.09, 0.59) | 0.028 (0.08, 0.58) | 0.023 (0.08, 0.59) | 0.019 (0.08, 0.60) | 0.028 (0.08, 0.61) | 0.025 (0.08, 0.57) | 0.037 (0.10, 0.61) | 0.023 (0.08, 0.59) | 0.000 (0.01, 0.57) | 0.000 (0.00, 0.55) | 0.000 (0.01, 0.57) | 0.000 (0.01, 0.57) | 0.000 (0.00, 0.56) |
| reconciliation_office | 0.206 (0.25, 0.16) | 0.225 (0.25, 0.16) | 0.215 (0.24, 0.14) | 0.231 (0.24, 0.16) | 0.231 (0.23, 0.15) | 0.201 (0.25, 0.16) | 0.155 (0.26, 0.15) | 0.225 (0.25, 0.16) | 0.192 (0.25, 0.16) | 0.225 (0.25, 0.16) | 0.396 (0.18, 0.08) | 0.389 (0.19, 0.10) | 0.396 (0.18, 0.08) | 0.396 (0.18, 0.08) | 0.387 (0.19, 0.09) |
| protocol_office | 0.361 (0.31, 0.15) | 0.394 (0.30, 0.16) | 0.356 (0.30, 0.14) | 0.380 (0.30, 0.16) | 0.380 (0.28, 0.13) | 0.359 (0.29, 0.15) | 0.354 (0.29, 0.12) | 0.368 (0.30, 0.16) | 0.368 (0.27, 0.13) | 0.373 (0.30, 0.16) | 0.463 (0.23, 0.07) | 0.472 (0.24, 0.07) | 0.463 (0.23, 0.07) | 0.451 (0.22, 0.07) | 0.465 (0.24, 0.07) |
| joint_task_force | 0.030 (0.10, 0.19) | 0.025 (0.08, 0.18) | 0.019 (0.08, 0.17) | 0.021 (0.08, 0.18) | 0.023 (0.09, 0.17) | 0.016 (0.06, 0.19) | 0.009 (0.08, 0.16) | 0.030 (0.09, 0.18) | 0.014 (0.08, 0.18) | 0.021 (0.09, 0.18) | 0.118 (0.02, 0.05) | 0.148 (0.02, 0.06) | 0.118 (0.02, 0.05) | 0.120 (0.02, 0.05) | 0.148 (0.02, 0.06) |
| defense_appropriations | 0.109 (0.08, 0.15) | 0.116 (0.07, 0.16) | 0.102 (0.07, 0.16) | 0.111 (0.08, 0.17) | 0.130 (0.08, 0.16) | 0.132 (0.07, 0.14) | 0.123 (0.08, 0.13) | 0.113 (0.07, 0.16) | 0.100 (0.07, 0.16) | 0.113 (0.08, 0.17) | 0.007 (0.03, 0.09) | 0.005 (0.04, 0.10) | 0.007 (0.03, 0.09) | 0.007 (0.03, 0.09) | 0.005 (0.04, 0.10) |
| homeland_security_act | 0.069 (0.22, 0.54) | 0.081 (0.20, 0.51) | 0.088 (0.20, 0.51) | 0.076 (0.19, 0.50) | 0.076 (0.19, 0.51) | 0.072 (0.22, 0.51) | 0.044 (0.18, 0.52) | 0.083 (0.22, 0.51) | 0.086 (0.23, 0.52) | 0.074 (0.22, 0.52) | 0.000 (0.04, 0.49) | 0.000 (0.06, 0.49) | 0.000 (0.04, 0.49) | 0.000 (0.04, 0.49) | 0.000 (0.06, 0.50) |
| preprint_server | 0.074 (0.10, 0.14) | 0.093 (0.12, 0.15) | 0.083 (0.09, 0.13) | 0.100 (0.11, 0.14) | 0.102 (0.11, 0.13) | 0.090 (0.08, 0.12) | 0.058 (0.09, 0.13) | 0.083 (0.10, 0.12) | 0.079 (0.12, 0.15) | 0.097 (0.10, 0.13) | 0.227 (0.03, 0.07) | 0.241 (0.03, 0.07) | 0.227 (0.03, 0.07) | 0.225 (0.03, 0.08) | 0.243 (0.03, 0.07) |
| supplier_network | 0.150 (0.13, 0.13) | 0.171 (0.13, 0.13) | 0.167 (0.13, 0.12) | 0.160 (0.13, 0.12) | 0.162 (0.14, 0.14) | 0.141 (0.14, 0.16) | 0.144 (0.11, 0.11) | 0.174 (0.14, 0.13) | 0.162 (0.14, 0.14) | 0.167 (0.13, 0.12) | 0.310 (0.10, 0.08) | 0.317 (0.10, 0.08) | 0.310 (0.10, 0.08) | 0.312 (0.10, 0.08) | 0.324 (0.10, 0.08) |
| fusion_research_campus | 0.081 (0.19, 0.56) | 0.083 (0.20, 0.56) | 0.095 (0.29, 0.55) | 0.088 (0.19, 0.54) | 0.067 (0.21, 0.57) | 0.072 (0.20, 0.55) | 0.088 (0.34, 0.58) | 0.090 (0.20, 0.55) | 0.093 (0.22, 0.58) | 0.086 (0.20, 0.55) | 0.005 (0.01, 0.55) | 0.016 (0.06, 0.56) | 0.005 (0.01, 0.55) | 0.005 (0.01, 0.55) | 0.016 (0.06, 0.56) |
| hyperscale_botnet | 0.125 (0.32, 0.48) | 0.007 (0.35, 0.49) | 0.002 (0.39, 0.49) | 0.009 (0.36, 0.49) | 0.009 (0.36, 0.50) | 0.005 (0.33, 0.47) | 0.139 (0.39, 0.52) | 0.009 (0.34, 0.47) | 0.007 (0.36, 0.49) | 0.007 (0.35, 0.49) | 0.116 (0.25, 0.48) | 0.111 (0.28, 0.46) | 0.116 (0.25, 0.48) | 0.118 (0.26, 0.48) | 0.111 (0.28, 0.46) |
| moonshot_grant | 0.023 (0.04, 0.16) | 0.014 (0.03, 0.16) | 0.014 (0.07, 0.15) | 0.019 (0.03, 0.16) | 0.016 (0.03, 0.16) | 0.023 (0.03, 0.16) | 0.019 (0.08, 0.15) | 0.012 (0.03, 0.16) | 0.023 (0.03, 0.16) | 0.019 (0.03, 0.15) | 0.007 (0.00, 0.07) | 0.005 (0.02, 0.08) | 0.007 (0.00, 0.07) | 0.007 (0.00, 0.08) | 0.005 (0.02, 0.08) |

Played 0 times: strategist:w.insight_income=1.5: data_licensing_deal; strategist:w.threshold_tokens=1: data_licensing_deal; strategist:w.insight_cap=10,keep_spare=1: data_licensing_deal; strategist: data_licensing_deal; strategist:w.strat_contents=1: data_licensing_deal; strategist:w.team_share=1,plan_team=1: data_licensing_deal; strategist:w.insight_income=1.5,w.team_share=1,plan_team=1,w.strat_contents=1,w.threshold_tokens=1,w.insight_cap=10,keep_spare=1,w.limit_room=1,plan_limits=1,w.upkeep_loss=1: data_licensing_deal; strategist:w.limit_room=1,plan_limits=1: data_licensing_deal; strategist:w.patience=0.5: data_licensing_deal; strategist:w.upkeep_loss=1: data_licensing_deal; greedy: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; greedy:w.insight_cap=10,keep_spare=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, capacity_market, smart_grid_backbone, homeland_security_act; greedy:plan_limits=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; greedy:w.upkeep_loss=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, dispatch_control_room, smart_grid_backbone, homeland_security_act; greedy:w.insight_cap=10,keep_spare=1,plan_limits=1,w.upkeep_loss=1: data_licensing_deal, tidal_barrage, positron_storage_ring, continental_supergrid, ransomware_syndicate, capacity_market, smart_grid_backbone, homeland_security_act
