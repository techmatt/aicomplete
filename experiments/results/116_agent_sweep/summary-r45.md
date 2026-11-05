# Agent sweep, r45

strategist at d2, pairs bp, br, pr; sweep seeds 33932+100, confirm seeds 34032+96; binary 459d098c8ed8aba6. Written by scripts/tune/agent_sweep.py at 10:33pm.

**Best setting:** `w.credit_cap=16`

Spec: `strategist:w.insight=0.5,w.energy=0.8,w.credits=0.6,w.chips=2,w.table=0.6,w.horizon=1,w.credit_cap=16,w.packet_keep=0.5,plan_credit=3,w.strat_scale=1`

## Held-out confirm (288 games a setting)

|  | hand-set | best | greedy |
|---|---|---|---|
| won % | 74.3 | 76.4 | 57.3 |
| credits left a seat | 47.6 | 48.4 | 14.7 |
| rounds | 6.93 | 6.88 | 5.25 |

- best less hand-set: +2.1 ±2.6 points (paired); by pair: bp +0.0 ±4.4, br -3.1 ±4.0, pr +9.4 ±4.9
- strategist less greedy: before +17.0 ±3.6, after +19.1 ±3.6
- credits left a seat, best less hand-set: +0.8 ±3.1

## On the sweep seeds (300 games a setting)

|  | hand-set | best | greedy |
|---|---|---|---|
| won % | 76.7 | 81.0 | 61.0 |
| credits left a seat | 46.9 | 49.1 | 15.7 |
| rounds | 6.92 | 6.98 | 5.40 |

- best less hand-set: +4.3 ±2.6 points (paired); by pair: bp +4.0 ±4.9, br +5.0 ±5.2, pr +4.0 ±3.5
- strategist less greedy: before +15.7 ±3.5, after +20.0 ±3.3
- credits left a seat, best less hand-set: +2.2 ±3.0

## Profile around the best (81.0% won on the sweep seeds)

Each parameter within 1 grid step(s) of the best, the others at the best setting: won-% gain over the best, paired, ± SE (credits left a seat after). *Matters*: some grid point plays worse than the best by over 2 SE.

| parameter | hand-set | best | matters | grid (* best) |
|---|---|---|---|---|
| `w.insight` | 0.5 | 0.5 | yes | 0.35: -3.7 ±1.9 (50) · 0.5*: +0.0 ±0.0 (49) · 0.75: -5.0 ±2.1 (47) |
| `w.energy` | 0.8 | 0.8 | yes | 0.6: -3.3 ±2.5 (49) · 0.8*: +0.0 ±0.0 (49) · 1.1: -6.0 ±2.8 (48) |
| `w.credits` | 0.6 | 0.6 | yes | 0.45: -1.0 ±2.6 (47) · 0.6*: +0.0 ±0.0 (49) · 0.75: -10.7 ±2.8 (49) |
| `w.chips` | 2 | 2 | no | 1.5: -4.3 ±2.4 (47) · 2*: +0.0 ±0.0 (49) · 2.75: -2.0 ±2.4 (45) |
| `w.table` | 0.6 | 0.6 | yes | 0.45: -6.7 ±2.9 (47) · 0.6*: +0.0 ±0.0 (49) · 0.75: -7.0 ±2.8 (49) |
| `w.horizon` | 1 | 1 | yes | 0.75: +0.0 ±2.9 (52) · 1*: +0.0 ±0.0 (49) · 1.25: -14.3 ±2.9 (42) |
| `w.credit_cap` | 12 | 16 | yes | 12: -4.3 ±2.6 (47) · 16*: +0.0 ±0.0 (49) · 20: -7.3 ±2.5 (49) |
| `w.packet_keep` | 0.5 | 0.5 | yes | 0.25: -4.7 ±2.1 (45) · 0.5*: +0.0 ±0.0 (49) · 0.75: -4.3 ±2.3 (47) |
| `plan_credit` | 3 | 3 | no | 2: -2.3 ±2.1 (46) · 3*: +0.0 ±0.0 (49) · 4: -1.7 ±1.5 (50) |
| `w.strat_scale` | 1 | 1 | no | 0.5: -1.7 ±2.6 (50) · 1*: +0.0 ±0.0 (49) · 1.5: -4.0 ±2.5 (52) |

## Search trail

| pass | parameter | from | to | won % before | best step | each point |
|---|---|---|---|---|---|---|
| 1 | `w.insight` | 0.5 | 0.5 | 76.7 | -0.7 ±2.3 | 0.35:-1.3 0.75:-0.7 |
| 1 | `w.energy` | 0.8 | 0.8 | 76.7 | -1.3 ±2.6 | 0.6:-2.3 1.1:-1.3 |
| 1 | `w.credits` | 0.6 | 0.6 | 76.7 | +2.3 ±2.9 | 0.45:+2.3 0.75:-2.7 |
| 1 | `w.chips` | 2 | 2 | 76.7 | +2.3 ±2.2 | 1.5:-3.0 2.75:+2.3 |
| 1 | `w.table` | 0.6 | 0.6 | 76.7 | +1.0 ±3.0 | 0.45:-3.0 0.75:+1.0 |
| 1 | `w.horizon` | 1 | 1 | 76.7 | +1.7 ±2.9 | 0.75:+1.7 1.25:-2.0 |
| 1 | `w.credit_cap` | 12 | 16 | 76.7 | +4.3 ±2.6 | 9:+2.0 16:+4.3 |
| 1 | `w.packet_keep` | 0.5 | 0.5 | 81.0 | -4.3 ±2.3 | 0.25:-4.7 0.75:-4.3 |
| 1 | `plan_credit` | 3 | 3 | 81.0 | -1.7 ±1.5 | 2:-2.3 4:-1.7 |
| 1 | `w.strat_scale` | 1 | 1 | 81.0 | -1.7 ±2.6 | 0.5:-1.7 1.5:-4.0 |

## What the best setting does differently (confirm games, per game, paired)

| counter | hand-set | best | gap |
|---|---|---|---|
| economy.credits_unspent.r01 | 6.74 | 7.66 | +0.91 ±0.17 |
| card.pick_sum.deep_state_ledger | 2.10 | 1.68 | -0.42 ±0.10 |
| card.drawn.grad_student_pool | 0.57 | 0.47 | -0.09 ±0.03 |
| slots.over.0.r06 | 0.48 | 0.72 | +0.24 ±0.08 |
| card.offered.deep_state_ledger | 0.75 | 0.67 | -0.08 ±0.03 |
| card.drafted.deep_state_ledger | 0.75 | 0.67 | -0.08 ±0.03 |
| cards.played.r01 | 6.35 | 6.28 | -0.07 ±0.02 |
| ai.power.researcher.3 | 1.47 | 1.56 | +0.09 ±0.03 |
| economy.credits_unspent.r02 | 14.72 | 16.06 | +1.34 ±0.47 |
| cards.insight_played.r01 | 5.03 | 4.95 | -0.08 ±0.03 |
| ai.power.protector.6 | 1.61 | 1.45 | -0.16 ±0.06 |
| card.pick_sum.grad_student_pool | 0.31 | 0.49 | +0.18 ±0.07 |
| card.drawn.citation_engine | 0.64 | 0.54 | -0.10 ±0.04 |
| card.pick_sum.joint_task_force | 0.62 | 0.41 | -0.21 ±0.08 |
| cards.used.simulation_cluster | 0.48 | 0.36 | -0.11 ±0.04 |
| cards.reused | 0.48 | 0.36 | -0.11 ±0.04 |
| card.offered.foundation_model_campus | 0.74 | 0.69 | -0.06 ±0.02 |
| card.drafted.foundation_model_campus | 0.74 | 0.69 | -0.06 ±0.02 |
| cards.kept.neural_mirror | 0.11 | 0.16 | +0.05 ±0.02 |
| card.kept.neural_mirror | 0.11 | 0.16 | +0.05 ±0.02 |
| economy.insight_carried.r03 | 8.00 | 7.70 | -0.30 ±0.12 |
| cards.insight_after_keep.r03 | 14.49 | 14.09 | -0.40 ±0.17 |
| card.pick_sum.research_royalties | 0.54 | 0.36 | -0.18 ±0.07 |
| card.pick_sum.citation_engine | 0.37 | 0.53 | +0.16 ±0.07 |
| alignment.tip_discord.r10 | 0.03 | 0.01 | -0.02 ±0.01 |
| cards.trigger.protector | 5.51 | 6.49 | +0.98 ±0.42 |
| board.wall_stops | 5.51 | 6.49 | +0.98 ±0.42 |
| ai.power.researcher.3.r01 | 0.42 | 0.44 | +0.03 ±0.01 |
| card.offered.aegis_grid_lockdown | 0.13 | 0.20 | +0.07 ±0.03 |
| card.drafted.aegis_grid_lockdown | 0.13 | 0.20 | +0.07 ±0.03 |
