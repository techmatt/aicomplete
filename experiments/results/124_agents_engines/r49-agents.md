# 124: agents play the engines (r49, agents only)

No rules, card or price change; `RULES_REVISION` stays 49. Tables: [dev-r49-base.md](dev-r49-base.md) (the re-read
before any change), [disproof.md](disproof.md), [dev-levers.md](dev-levers.md) (screen 1), [dev-factorial.md](dev-factorial.md)
(screen 2), [dev-candidate.md](dev-candidate.md) (screens 3-4), [fresh.md](fresh.md) (the final read),
[energy-fresh.md](energy-fresh.md), [sources-fresh.md](sources-fresh.md). Scripts: `read124.py` (win, gap, limits,
peaks, keeps by insight flag, engine-card plays), `variant_cards.py`, `energy_read.py`. Specs:
`experiments/screen-124-{dev,disproof,candidate,candidate15,r49}.json`; screens 1-2 (which name the dropped
`w.limit_room`) are in `experiments/archive/`.

## Premises re-read at r49 (dev seeds 34756-34899, 144 a cell, the three pairs at d2, 864 games)

| Claim (prompt) | At r49 |
|---|---|
| Hyperscale Botnet legal 138 steps, played 0 (r47) | strategist 0.009 a game (kept 0.36, offered 0.49); greedy 0.116 |
| Moonshot Grant kept and never played | strategist played 0.019, kept 0.03, offered 0.16: played nearly every time it is kept; it is rarely kept |
| Flagged Researchers keep fewer picks: strategist 54% vs 60%, greedy 30% vs 38% | 54% vs 67%, greedy 28% vs 36% (122's flag rule) |
| Data Licensing Deal (D42) | not in `configs/baseline.json`'s deck: offered 0 |

Strategist 70.6% in 6.02 rounds, greedy 49.1% in 4.91 (dev). Energy lost at the limit 4.4 a game (56% of games).

## Instrumentation

`games.jsonl` per seat: `start` (setup's holdings), `keeps` (picks offered, kept after the opening, tier choices,
tier 2, insight held at each keep), `sources` (gains by `own:`/`other:` action and card, or `phase:` automatic phase;
they sum to `gained`). Play unchanged (`same_results.py`: same 96 games). `runaway_read.py` flags a peak's rise over
`start`, not the peak: on 96 games the credit flags went 1 → 0 and the insight flags 41 → 11 (seats start with 6).
`source_read.py` reads both. Upkeep (not `board.produce`) is where most credit income lands; most insight income
arrives at `board.cleanup`; energy is ~70% `board.produce` (cities).

## Disproof rows (strategist, dev, 432 games each)

| Row | Botnet played (kept) | Moonshot played (kept) | Reading |
|---|---|---|---|
| base | 0.009 (0.36) | 0.019 (0.03) | |
| Botnet cost 1, Moonshot cost 6 | 0.065 (0.34) | 0.144 (0.08) | Moonshot: price. Botnet nearly free is played in 1 game of 5 it is kept: not price alone |
| Both `needs` [] | 0.025 (0.56) | 0.019 (0.10) | not legality |
| `w.insight_income=1.5` | 0.125 (0.32) | 0.023 (0.04) | Botnet: the play check's insight income (D53) |
| `w.patience=0.5` (Patience insight at plain; screen 1) | | | flagged Researcher keeps 54% → 53%: not the held value, the plan (it keeps only what it can fit) |

D53's cause: the play check counts an insight a round at the held weight, 0.5 points (0.83 credits), while the
table that keeps it says 2.5 credits; held in hand Botnet was worth more than built. At 1.5 points (2.5 × the engine's
table 0.6) it is played at greedy's rate. Moonshot at its price is played when kept; it is kept rarely because 46
credits for 8 cubes and 2 draws barely beats its price at the table (57 for 48.5): a good player rarely buys it either.

## Levers (each a switch; dev, strategist, 432 games a cell, paired against base)

| Lever | Alone | Notes |
|---|---|---|
| `w.strat_contents` (D47: strategies read triggers' and thresholds' ops) | **+2.5 ±1.2** | |
| `w.insight_income` 1 / 1.5 (D53) | +1.2 ±1.8 / −2.8 ±2.2 | Botnet 0.05 / 0.13 a game |
| `w.insight_cap=10,keep_spare` (finding 2) | +0.0 ±1.8 | flagged Researcher keeps 54% → 75%, tier 2 58% → 61% |
| income 1 / 1.5 with the cap (factorial) | +2.1 ±2.2 / +2.3 ±2.3 | the cap prices a Patience pile's income idle, so 1.5 no longer overplays |
| `w.limit_room,plan_limits` (finding 3, first form) | −0.9 ±0.7 | the hand's value clipped by today's room: a gain card held at the limit read cheap, so it was played into the limit |
| `plan_limits` (final form: the plan's budget and room only) | −0.2 ±0.2 | the play check already sees the engine's clamp |
| `w.upkeep_loss` (finding 4) | +0.5 ±0.7 | energy lost 4.37 → 3.89; with `plan_limits` 0.0 ±0.7, 4.00 |
| `w.team_share,plan_team` (D47) | −0.9 ±1.9 | on the candidate −2.1: left off |
| `w.threshold_tokens` (D47) | +0.0 ±0.9 | on the candidate −1.4: left off |
| candidate: contents, income 1, cap, `plan_limits`, upkeep | +2.8 ±2.2 | |
| **frozen: the same with income 1.5** | **+3.7 ±2.5** | Botnet 0.14, Moonshot 0.02 |

The review of the first diff (a three-lens read, each finding checked by a skeptic) found four real defects in the
levers, all off by default then: per-tag credit production not capped (fixed), insight income priced idle outside
Patience (fixed: Patience only), the plan's room read before its payments (fixed: a card's own payment frees room, then
the budget's room), `upkeep_loss` documented for four resources (two: chips held aren't valued in horizon mode,
ambassadors have no income). Not fixed: `upkeep_loss` reads `economy::income`, so Sabotage's lost energy isn't seen;
`GainPerTag`, `GainPerTokens` and `CashOut` gains and Copy's income aren't clipped by the plan's room.

"Solo gifts" (D47's 2-seat table): every read is at 2 players, where the table's `countries` 2 is right; nothing
built.

**102's Phase A switches folded in**: `plan_net`, `plan_rates` (the plan switch), `plan_cover_spent`, `plan_idle`,
`plan_gross`, `w.power_dear`, `w.token_cash`, `w.converter_any`, `w.flip_open`, `w.push_adv` were all on by default;
their on paths stay and the switches are gone. Smoke: the same 96 games before and after (16 dev seeds, both agents).
The ten specs naming them moved to `experiments/archive/`.

**Greedy** (the new player) gets the spare-insight keep and the two limit rules (Matt, message 5216: what a new
player would do): `insight_cap`, `upkeep_loss` in `Weights::default`, `keep_spare` and `plan_limits` (which follow
the heuristic's `insight_cap` and `upkeep_loss`). The strategy and insight-income levers are the engine's: greedy
has no strategy and no horizon. For a while the item ran greedy unchanged (the default of question 1); the answer
came before the commit and the reads below are the final build's.

## The final read (fresh seeds 34948-34995, 48 a cell, paired against main 27da088)

| Agent | Before | After | Gap ± SE | Rounds |
|---|---|---|---|---|
| strategist | 74.3% (81 / 67 / 75) | 76.4% (77 / 67 / 85) | +2.1 ±3.5 | 6.23 → 6.24 |
| greedy | 45.8% (40 / 40 / 58) | 52.1% (42 / 46 / 69) | +6.2 ±3.3 | 4.83 → 4.87 |
| lead | 28.5 ±5.5 | 24.3 ±5.6 | −4.2 | |

The same final build on the dev seeds (34756-34899, 144 a cell, 432 games an agent), paired against the build before:

| Agent | Before | After | Gap ± SE | Rounds |
|---|---|---|---|---|
| strategist | 70.6% | 74.3% | +3.7 ±2.5 | 6.02 → 6.04 |
| greedy | 49.1% | 48.1% | −0.9 ±1.9 | 4.91 → 4.94 |
| lead | 21.5 ±2.9 | 26.2 ±3.0 | +4.7 | |

The lead grows on the larger read and shrinks on the fresh one, which is greedy's +6.2 there (most of it pr, +11):
the two reads disagree on greedy by about 2 SE of the fresh one, and Matt weighs the 432-game read (message 5216).
Greedy's flagged Researcher keeps 25% → 47% fresh (28% → 47% dev), which is the change; nothing else of greedy's moved.

Lost at the limits a game, fresh (dev): strategist energy 5.38 → 4.29, 65% → 62% of games (4.37 → 3.35, 56% →
51%); credits 3.28 → 2.47 (1.41 → 2.19), chips 1.92 → 1.73, ambassadors 0.58 → 0.42, credit production 0.67 → 0.71.
Greedy energy 0.37 → 0.28 (0.57 → 0.53). What is still lost lands at `board.produce` and upkeep with the tracks
full: the seat has nothing left to spend it on.

Peaks per seat (strategist, mean / highest), before → after: Builder credits 68 / 150 → 65 / 150, energy 9.0 → 8.7;
Researcher insight 28.2 / 72 → 22.6 / 89, credits 62 / 146 → 62 / 150; the rest within a point (fresh.md).

The Researcher, 122's flag rule, fresh (dev): strategist flagged 51% → **73%** (54% → 74%) against unflagged 64% →
70% (67% → 73%), flagged tier-2 share 56% → 60% (58% → 59%); flagged seats 58 → 38 of 96 (the fixed rule: 9 of 96).
Greedy flagged 25% → **47%** (28% → 47%) against unflagged 37% → 43% (36% → 45%), tier 2 77% → 80% (77% → 78%).

Plays a game of every r41 engine card (the 45 `aic-sim cards` marks engine, plus the two named): fresh.md. Strategist:
Hyperscale Botnet 0.021 → 0.153, Moonshot Grant 0.007 → 0.049; none at 0 but Data Licensing Deal (not dealt).
Greedy plays 9 at 0 (Intrusion Lab, Tidal Barrage, Positron Storage Ring, Continental Supergrid, Economies of Scale,
Ransomware Syndicate, Capacity Market, Dispatch Control Room, Smart Grid Backbone; before also Neural Mirror, Defense
Appropriations, Homeland Security Act, Moonshot Grant): its flat terms count no card income, so the new player doesn't
buy income programs and buildings; the strategist plays every one.

## Speed

`per_decision.py` (c2, d2, 24 games from seed 1000, one thread, best of two; other games, so per decision):

| | ms/decision | decisions a game | options/decision |
|---|---|---|---|
| strategist, final / before | **1.33-1.39×** (two reads; 0.9-1.0 → 1.3-1.4) | 1.19× | 1.09× |
| strategist, the levers off / before (same games) | 1.05× | 1.00× | 1.00× |
| greedy, final / before (its games barely moved) | **0.97×** | 1.00× | 1.01× |

Greedy's first final build read 1.21× a decision: the flat eval read the board's tokens and the seat's income
afresh for `upkeep_loss` on every position; it now takes the tokens the position already read and the income from
the seat-terms store (the same 288 fresh games). For the strategist the code costs 5%; the rest is play. What the frozen agent costs a decision against it with one lever off (16
games, under load): `strat_contents` 1.36×, `keep_spare` 1.20×, `insight_income` 1.15×, the limits 1.01×. Each makes
the strategist keep and build more engine cards (keep rate 56% → 72%, more buildings with triggers), so a position
has a larger hand and more standing effects to score. Kept: the better play is the point of the item, and a
strategist game still costs about a third of a second (`speed.py check`: engine 332 ms a game, 1.55× its baseline,
no verdict across changed games; planner 1.12×). The baseline is updated to this build for that reason.
