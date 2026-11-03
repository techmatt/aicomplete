# 008 engine agent: does a building engine beat no-building play?

> **Historical:** ran on rules r3 (today's is r6); the rules and content it measured
> were rewritten later (the board in 014, opposition replaced by Alignment in 013).

2026-10-28. Rules r3 (007's tuned v0), 2 countries; no rules changed. The question for
the engine-payback design call: 007's strongest agent (greedy) never plays an economy
building, and beam and rollout scored at or below it. Is that the rules or the agents?

**Answer: the rules.** An agent built to value income by the rounds left plays economy
buildings and gains nothing for them. An agent that plays a lot of them loses 10 points
at difficulty 4. The buildings that do pay are the two that act on the board (Turnkey
Factory builds a factory, Server Farm places a processor): taking those away costs 9–11
points.

Report: [`results/engine-agent-008/report.md`](results/engine-agent-008/report.md)
(240 paired seeds per cell, 300000+). Sweep log:
[`results/engine-sweep-008/sweeps.jsonl`](results/engine-sweep-008/sweeps.jsonl).

## The engine agent (`aic-sim`, `engine`)

The heuristic's **horizon mode** (`agents/eval.rs`; README, *Agents*):

- **Income is worth what it pays over the rounds left.** That covers tracks, income
  buildings and rewards, less building upkeep.
- **The rounds left** come from the adversary deck's clock (one reveal a round), cut
  short by when the team is on course to win (opposition still needed at its rate so
  far). They are discounted by danger: unpaid threats, the pool and population near the
  edge.
- **A building with no income** earns a per-round value for the rounds left.
- **A card in hand** counts by the chance it can be paid for (credits and antimatter)
  before the end.
- **Resources** fade when under two rounds are left.

The search is one action at a time, like greedy. An investment's payback is in the
valuation itself, so the agent doesn't need to look rounds ahead to see it. A beam over
the round (`width` > 0) and rollouts across rounds (`candidates` > 0) are options, but
both did worse.

## The sweep (difficulty 4, seeds 0+)

48 games per point unless marked; ± about 7 points at 48 games and 4.7 at 96. `H` is
the horizon weights on greedy search, before the engine kind's defaults existed.

| Agent | Games | Win % | Buildings / game | Tier 2 / game |
|---|---|---|---|---|
| greedy | 48 / 96 | 64.6 / 68.8 | 1.8 | 1.7 |
| greedy:H (`horizon` 0.5 / 1 / 1.5 / 2) | 48 | 56 / 60 / 58 / 58 | 1.0 / 2.3 / 3.4 / 4.0 | 1.9–2.4 |
| greedy:H, `risk` 0 / 0.25, `ability` 0 / 1, `spend_horizon` 0, `opp_rate` 1 | 48 | 58–63 | 1.7–3.0 | 1.9–2.5 |
| greedy:H, `hand` 3 / `hand_cost` 0.4 (greedy's hand values) | 48 | 62.5 | 0.3 | 1.5 |
| `engine:width=4` (beam), `w.horizon=2` | 48 | 45.8, 41.7 | 3.2, 4.6 | 3.1 |
| `engine:width=4,candidates=2` (beam + rollouts) | 48 | 43.8 | 3.1 | 3.0 |
| `engine:width=0`, with `samples=3` | 48 | 64.6, 60.4 | 2.4 | 2.2 |
| after the review fixes: `engine`, `production` 1 / **0** | 96 | 65.6 / **72.9** | 2.7 / 2.5 | 2.5 / 2.2 |

- The more buildings a setting played, the more often it lost (4 or more a game: 42–58%).
- Beam search loses about 20 points whatever it is scoring. That is 007's finding too:
  more search exploits the heuristic's blind spots.
- An insight-queue term (each card in hand waits for the insight of the cards before
  it) cut wins to 4%. Spending insight on one card lowered every other card's value, so
  playing anything looked costly. It is off (`insight_wait` 0).
- The defaults are the best greedy-search row with `production` 0.

## The answer (difficulties 0 and 4, 240 paired seeds)

| Agent | d0 win % | d4 win % | Buildings played / game (d0, d4) | Economy buildings standing / game (d0, d4) | Tier 2 / game (d0, d4) |
|---|---|---|---|---|---|
| greedy | 94.6 | 70.8 | 1.89, 1.82 | 0.00, 0.00 | 2.14, 1.75 |
| engine | 92.9 | 68.3 | 3.58, 2.43 | 1.90, 0.93 | 2.75, 2.10 |
| 007's building greedy (`built_cost` 1.2, `hand` 1, `hand_cost` 0.2) | 92.9 | 60.4 | 11.0, 10.5 | 4.08, 3.86 | 3.26, 2.34 |
| engine forbidden to build (`w.built=-100`) | 89.6 | 59.6 | 0, 0 | 0, 0 | 2.14, 1.52 |

Economy buildings are Fab Line, Trade Office, Solar Field, Microgrid, Overclock Rig,
Assembly Cluster, Grid Battery and the tier-2 income buildings. Greedy's 1.8 buildings
a game are all Turnkey Factory (the Builder's unique card) and Server Farm.

Head to head on the same seeds (the agent minus greedy; exact sign test):

- **engine vs greedy:** −1.7 ± 2.0 (p 0.54) at d0; −2.5 ± 3.1 (p 0.50) at d4. **Not
  stronger.**
- **building greedy vs greedy:** −1.7 ± 2.0 (p 0.54) at d0; **−10.4 ± 3.3 (p 0.003)**
  at d4.
- **engine vs the same engine forbidden to build:** +3.3 ± 1.8 (p 0.10) at d0; **+8.8 ±
  2.7 (p 0.003)** at d4. Greedy beats the no-build engine by 11.2 at d4 (p 0.002) while
  playing no economy buildings. So that gain comes from Turnkey Factory and Server Farm,
  the two buildings both agents play, not from an income engine.
- **Within the engine's d4 games,** those with 2+ economy buildings standing won 61% (57
  games), the others 70.5% (183). This is confounded (they build when it is going well
  or badly), but it is not the sign of an engine that pays.

**Verdict: a building engine does not beat no-building play at these numbers.** Income
buildings cost a card, a credit to keep, 5–12 credits and an insight. They return 1–2 a
round in a game that lasts about 7 rounds. They pay back in 4–6 rounds at best and
compete with the opposition cards (Precision Strike, Rally, Sweep, Purge Protocol) that
win the game. Only tier-2 Purge Protocol (and, for the engine, some Counter-intelligence)
is played from tier 2. The tier-2 buildings stay dead (under 13% played even by the
building greedy). This is 007's proposal 5 (engine depth) measured with an agent that
values payback: the payback design call is needed. The agents were not missing it.

## Strategy-health gate (for tuning prompts)

The strongest agent (greedy) plays no economy building and 1.8–2.1 tier-2 cards a game
(nearly all Purge Protocol). By the gate in state.md's pending fix, calibration should
not continue until a payback lever makes the engine agent beat greedy by building.
`engine-agent-008.json` is the measure to rerun for each lever (009). Watch for the
engine agent going ahead of greedy with economy buildings standing above about 2 a game.
