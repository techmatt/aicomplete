# 298: the dev-seed screens

Written by `make_tables.sh` (`screen.py`) from farmed games; dev seeds 36800-36831, 32 a pair, the judge. Arms
are `set` lists (the specs: `experiments/farm/s298a.json`, `s298r.json`, `s298t.json`). Per-seat columns divide by
the seats; Accord is per Protector seat-round.

## Screen 1 (f3facbcc: r144's Redeploy and Offensive in, the r143 judge; the three pairs pooled, 96 games a cell)

base: Arms Race off (`arms_race_at=5`), r143 prices; arms: Arms Race on; price: + Martial Law 36, Gain a player
ambassador 20, Accord 3 energy, Remove agents 3 ambassadors; home: price + the Strike rider (`board.strike_home=1`);
k3: price + `research_at=3,3,2,2,2`.

| Arm | Level | Games | Wins | Rounds | Research losses | Won by r3 | Strike hits | Sent home | Troops redeployed | Troop by r3 | Gain ambassador / seat-game | Remove agents / seat-game | Accord / Protector seat-round | Offensive deals / idle |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | d0 | 96 | 89% | 8.4 | 3 | 1 | 0.07 | 0.00 | 0.02 | 1 | 1.08 | 1.01 | 0.63 | 0.40 / 0.15 |
| base | d1 | 96 | 86% | 8.8 | 4 | 0 | 0.14 | 0.00 | 0.02 | 0 | 0.95 | 1.06 | 0.61 | 0.57 / 0.15 |
| base | d2 | 96 | 80% | 8.6 | 11 | 0 | 0.23 | 0.00 | 0.03 | 4 | 0.98 | 0.95 | 0.60 | 0.54 / 0.29 |
| base | d3 | 96 | 66% | 8.5 | 24 | 0 | 0.36 | 0.00 | 0.05 | 8 | 0.94 | 0.76 | 0.59 | 0.70 / 0.31 |
| arms | d0 | 96 | 89% | 8.4 | 3 | 1 | 0.07 | 0.00 | 0.02 | 1 | 1.08 | 1.01 | 0.63 | 0.40 / 0.15 |
| arms | d1 | 96 | 86% | 8.8 | 4 | 0 | 0.14 | 0.00 | 0.02 | 0 | 0.95 | 1.06 | 0.61 | 0.57 / 0.15 |
| arms | d2 | 96 | 80% | 8.6 | 11 | 0 | 0.23 | 0.00 | 0.03 | 4 | 0.98 | 0.95 | 0.60 | 0.54 / 0.29 |
| arms | d3 | 96 | 66% | 8.5 | 24 | 0 | 0.68 | 0.00 | 0.12 | 8 | 0.94 | 0.80 | 0.58 | 0.67 / 0.34 |
| price | d0 | 96 | 93% | 8.1 | 0 | 0 | 0.03 | 0.00 | 0.00 | 1 | 0.64 | 0.32 | 0.52 | 0.38 / 0.23 |
| price | d1 | 96 | 85% | 9.2 | 3 | 0 | 0.08 | 0.00 | 0.03 | 0 | 0.54 | 0.31 | 0.49 | 0.70 / 0.10 |
| price | d2 | 96 | 79% | 8.7 | 13 | 0 | 0.23 | 0.00 | 0.02 | 2 | 0.60 | 0.29 | 0.49 | 0.59 / 0.27 |
| price | d3 | 96 | 64% | 8.6 | 16 | 0 | 0.59 | 0.00 | 0.07 | 7 | 0.48 | 0.27 | 0.46 | 0.71 / 0.30 |
| home | d0 | 96 | 93% | 8.1 | 0 | 0 | 0.03 | 0.00 | 0.00 | 1 | 0.64 | 0.32 | 0.52 | 0.38 / 0.23 |
| home | d1 | 96 | 85% | 9.2 | 3 | 0 | 0.08 | 0.05 | 0.02 | 0 | 0.54 | 0.31 | 0.49 | 0.70 / 0.10 |
| home | d2 | 96 | 79% | 8.6 | 13 | 0 | 0.23 | 0.08 | 0.02 | 2 | 0.60 | 0.28 | 0.49 | 0.59 / 0.27 |
| home | d3 | 96 | 61% | 8.7 | 16 | 0 | 0.62 | 0.17 | 0.08 | 7 | 0.48 | 0.27 | 0.46 | 0.71 / 0.30 |
| k3 | d0 | 96 | 94% | 8.6 | 0 | 0 | 0.01 | 0.00 | 0.00 | 1 | 0.73 | 0.34 | 0.53 | 0.44 / 0.19 |
| k3 | d1 | 96 | 90% | 9.2 | 0 | 0 | 0.00 | 0.00 | 0.00 | 0 | 0.56 | 0.39 | 0.49 | 0.65 / 0.19 |
| k3 | d2 | 96 | 79% | 8.7 | 13 | 0 | 0.23 | 0.00 | 0.02 | 2 | 0.60 | 0.29 | 0.49 | 0.59 / 0.27 |
| k3 | d3 | 96 | 64% | 8.6 | 16 | 0 | 0.59 | 0.00 | 0.07 | 7 | 0.48 | 0.27 | 0.46 | 0.71 / 0.30 |

## The Strike rider on the final build (4ad672dc: r144 and the retaught judge; Matt's message 13713)

| Arm | Level | Games | Wins | Rounds | Research losses | Won by r3 | Strike hits | Sent home | Troops redeployed | Troop by r3 | Gain ambassador / seat-game | Remove agents / seat-game | Accord / Protector seat-round | Offensive deals / idle |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | d0 | 96 | 89% | 9.0 | 0 | 0 | 0.02 | 0.00 | 0.00 | 1 | 0.69 | 0.47 | 0.49 | 0.46 / 0.18 |
| base | d1 | 96 | 94% | 9.2 | 0 | 0 | 0.03 | 0.00 | 0.03 | 0 | 0.54 | 0.45 | 0.47 | 0.61 / 0.20 |
| base | d2 | 96 | 82% | 8.9 | 13 | 0 | 0.29 | 0.00 | 0.01 | 2 | 0.59 | 0.44 | 0.47 | 0.52 / 0.33 |
| base | d3 | 96 | 64% | 8.4 | 20 | 0 | 0.57 | 0.00 | 0.07 | 7 | 0.53 | 0.42 | 0.46 | 0.73 / 0.25 |
| home | d0 | 96 | 89% | 9.0 | 0 | 0 | 0.02 | 0.01 | 0.00 | 1 | 0.69 | 0.47 | 0.49 | 0.46 / 0.18 |
| home | d1 | 96 | 94% | 9.2 | 0 | 0 | 0.03 | 0.01 | 0.03 | 0 | 0.54 | 0.45 | 0.47 | 0.61 / 0.20 |
| home | d2 | 96 | 81% | 9.0 | 13 | 0 | 0.30 | 0.09 | 0.01 | 2 | 0.59 | 0.45 | 0.47 | 0.52 / 0.33 |
| home | d3 | 96 | 62% | 8.4 | 21 | 0 | 0.57 | 0.06 | 0.07 | 7 | 0.53 | 0.43 | 0.46 | 0.73 / 0.25 |

## Three seats (4ad672dc; configs/three_seat.json, Builder + Protector + Researcher, 32 games a cell)

base: the config; c1: one corrupted city a country at setup; lib4: Liberate at 4 ambassadors.

| Arm | Level | Games | Wins | Rounds | Research losses | Won by r3 | Strike hits | Sent home | Troops redeployed | Troop by r3 | Gain ambassador / seat-game | Remove agents / seat-game | Accord / Protector seat-round | Offensive deals / idle |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | d0 | 32 | 75% | 10.8 | 0 | 0 | 0.00 | 0.00 | 0.00 | 0 | 0.69 | 1.06 | 0.44 | 0.31 / 0.22 |
| base | d1 | 32 | 78% | 10.8 | 0 | 0 | 0.00 | 0.00 | 0.00 | 0 | 0.71 | 1.04 | 0.43 | 0.56 / 0.22 |
| base | d2 | 32 | 59% | 11.1 | 3 | 0 | 0.09 | 0.00 | 0.03 | 0 | 0.76 | 1.21 | 0.46 | 0.56 / 0.34 |
| base | d3 | 32 | 47% | 9.8 | 4 | 0 | 0.78 | 0.00 | 0.09 | 3 | 0.59 | 0.92 | 0.38 | 0.69 / 0.34 |
| c1 | d0 | 32 | 75% | 9.7 | 0 | 0 | 0.00 | 0.00 | 0.00 | 2 | 0.49 | 0.94 | 0.48 | 0.16 / 0.06 |
| c1 | d1 | 32 | 72% | 10.8 | 0 | 0 | 0.06 | 0.00 | 0.00 | 1 | 0.60 | 1.14 | 0.43 | 0.41 / 0.16 |
| c1 | d2 | 32 | 62% | 11.0 | 0 | 0 | 0.06 | 0.00 | 0.00 | 2 | 0.77 | 1.36 | 0.42 | 0.50 / 0.09 |
| c1 | d3 | 32 | 38% | 10.4 | 3 | 0 | 0.28 | 0.00 | 0.09 | 5 | 0.48 | 1.28 | 0.41 | 0.47 / 0.09 |
| lib4 | d0 | 32 | 78% | 10.0 | 0 | 0 | 0.06 | 0.00 | 0.00 | 0 | 0.71 | 0.89 | 0.46 | 0.25 / 0.16 |
| lib4 | d1 | 32 | 72% | 11.1 | 0 | 0 | 0.09 | 0.00 | 0.00 | 0 | 0.72 | 0.94 | 0.42 | 0.44 / 0.28 |
| lib4 | d2 | 32 | 72% | 10.6 | 1 | 0 | 0.12 | 0.00 | 0.00 | 1 | 0.80 | 1.20 | 0.42 | 0.59 / 0.25 |
| lib4 | d3 | 32 | 44% | 9.8 | 2 | 0 | 0.06 | 0.00 | 0.12 | 3 | 0.71 | 0.80 | 0.39 | 0.75 / 0.34 |

Greedy at three seats (`s298tg.json`):

| Arm | Level | Games | Wins | Rounds | Research losses | Won by r3 | Strike hits | Sent home | Troops redeployed | Troop by r3 | Gain ambassador / seat-game | Remove agents / seat-game | Accord / Protector seat-round | Offensive deals / idle |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | d0 | 32 | 91% | 10.2 | 0 | 0 | 0.00 | 0.00 | 0.00 | 0 | 0.74 | 0.59 | 0.42 | 0.44 / 0.03 |
| base | d1 | 32 | 66% | 11.4 | 0 | 0 | 0.03 | 0.00 | 0.00 | 0 | 0.76 | 0.73 | 0.44 | 0.44 / 0.22 |
| base | d2 | 32 | 69% | 11.2 | 1 | 0 | 0.09 | 0.00 | 0.00 | 0 | 0.55 | 0.76 | 0.35 | 0.47 / 0.22 |
| base | d3 | 32 | 44% | 9.9 | 3 | 0 | 0.78 | 0.00 | 0.12 | 3 | 0.58 | 0.88 | 0.38 | 0.56 / 0.38 |

## The judge's own terms against greedy (46bc29cd, the final build; `s298j.json`, 384 games an agent; wins (rounds))

| Agent | d0 | d1 | d2 | d3 | pooled | paired gap to the judge |
|---|---|---|---|---|---|---|
| `strategist` | 89% (9.0) | 94% (9.2) | 82% (8.9) | 64% (8.4) | 82.0% | +0.0 ± 0.0 |
| `strategist:w.purge=0` | 95% (7.9) | 89% (8.7) | 76% (9.1) | 66% (8.5) | 81.2% | -0.8 ± 2.2 |
| `strategist:w.hand=0,w.hand_cost=0` | 56% (11.1) | 32% (11.9) | 25% (10.9) | 14% (9.5) | 31.8% | -50.3 ± 2.8 |
| `strategist:w.card_action=0` | 97% (7.9) | 91% (8.9) | 84% (8.8) | 57% (8.6) | 82.3% | +0.3 ± 2.1 |
| `greedy` | 92% (8.4) | 90% (9.1) | 82% (9.1) | 69% (8.6) | 83.1% | +1.0 ± 2.4 |
