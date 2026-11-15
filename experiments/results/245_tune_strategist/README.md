# 245: tune the strategist, oracle-guided, train seeds against test seeds

Base: main 797a015, r109. No rules change (`RULES_REVISION` stays 109; greedy plays as before). **Shipped:** the
judge's hand terms, `w.hand` 1.5 and `w.hand_cost` 0.1 (were greedy's 3 and 0.4) in `STRATEGIST`
(`agents/mod.rs`): a card in hand counts 1.5 + 0.1 × its cost, so playing it wins over holding it more often. Two
numbers, no new code in the eval; the new binary's `strategist` replays the tuned arm's test games action for action
(36 of 36 checked, `same_games.py`). Tables: [screens.txt](screens.txt), [test-read.txt](test-read.txt) (and
[bands.txt](bands.txt)), [agree-train.txt](agree-train.txt), [agree-test.txt](agree-test.txt) (raw:
[agree-test.jsonl](agree-test.jsonl)), [cards.txt](cards.txt).

## Oracle agreement: `aic-sim agree`

New command (docs/simulator.md): replays each game the oracle won and asks candidate specs to choose on every recorded
table with more than one legal action, tallied by the oracle's action kind, as the same pick and as a pick the oracle
made later the same round. The agents follow the recorded game, so every spec sees the same tables (a fast-lane test:
an agent on its own game agrees at every decision). About 0.3 s an agent and game on 6 threads: 100+ specs on 64
oracle games in under 40 minutes, against 30 minutes for six arms' win rates.

**Train set:** 243's kept read (`243-rd-*`, seeds 47488-47679, 2,304 oracle games, all won). Fit set: 64 d2-d3 oracle
games (`mkset.py --every 18`). Readers: `agree_table.py` (score: same + half of same-or-later, over the kinds the search
decides; keep, draft and tier come from the keep plan and barely move), `mkagents.py`.

**What agreement found** (fit set, one weight at a time ×0.5 and ×2, 57 flat weights and a probe; agree-train.txt):
lower `w.hand_cost` (+2.0 score at 0.2), lower `w.ambassador` (+1.9), lower `w.hand` (+1.1), higher `w.threat` (+0.9),
lower `w.purge` (+0.8). The hand terms raise agreement on plays (+3), powers (+4) and standard actions (+5); the rest
mostly on standard actions (fewer ambassadors bought).

**Agreement is a guide, not a target.** The hand terms carried to wins; the ambassador, threat and purge moves did
not: `w.hand_cost=0.2` with `w.ambassador=2.5, w.threat=18, w.purge=1.8` read −5.1 ± 1.7 against +3.5 ± 1.6 for
`w.hand_cost=0.2` alone. The oracle sees the deck and buys fewer ambassadors because it knows which cities fall; a
player without that sight needs them (162 added the purge term for it).

## Screens (train seeds 47488-47679, d2-d3, three pairs, 1,152 games an arm, paired against today's)

| Arm | gap ± SE | d2 | d3 |
|---|---|---|---|
| `w.hand_cost=0.1, w.hand=1.5` (**frozen**) | +5.6 ± 1.5 | +7.1 | +4.0 |
| `w.hand_cost=0.1, w.hand=1.5, w.unity_tip=36` | +5.8 ± 1.6 | +6.6 | +5.0 |
| `w.hand_cost=0.1, w.hand=1.5, w.threat=12` | +4.0 ± 1.6 | +7.5 | +0.5 |
| `w.hand_cost=0.2, w.hand=1.5` | +3.9 ± 1.6 | +6.2 | +1.6 |
| `w.hand_cost=0.1/0.15, w.hand=2.25`; `0.15, 1.5` | +3.5 to +3.7 | +4.5 to +6.2 | +1.2 to +3.0 |
| `w.hand_cost=0.2` | +3.5 ± 1.6 | +3.3 | +3.6 |
| `w.hand_cost=0.1, w.hand=1.5` with `built_cost=0.6` / `node_danger=120` / `hand=0.75` / `sell_rule=2` | +1.7 to +3.0 | | |
| `w.built_income=1` (new probe, below) | −1.8 ± 1.4 | −3.6 | 0.0 |
| `w.hand_cost=0` (`w.hand=1.5`) | **−24.3 ± 1.8** | | |

The base arm reproduces 243's strategist on these seeds exactly (77.1 / 56.4). A plateau: `w.hand_cost` 0.1-0.2 by
`w.hand` 0.75-2.25 reads +2.4 to +5.8; nothing stacked on it beyond noise. **`w.hand_cost` 0 is a cliff:** every card
in a packet then counts the same, and the draft goes blind. Ranges screened: `w.hand` 0.75-6, `w.hand_cost` 0-0.8, the
57 flat weights ×0.5 to ×2 (agreement), `sell_rule` 1-2.

**Round 2, from the shipped judge** (after the test read; no fresh seeds spent): agreement again pointed at
`w.ambassador`, `w.threat`, `w.purge`, then `w.energy_cap` 4 (spare energy cheap: plays +1.2), `w.freeing`, `w.credits`,
`w.track_capstone` (agree-train.txt). Screened alone on the train seeds against the shipped judge's own games there:
`w.freeing=1` −0.9, `w.energy_cap=4` −1.4, `w.ambassador=3.5` −2.2, `w.track_capstone=32` −2.6, `w.credits=0.45` −3.4
(± 1.3-1.4; the base was picked on these seeds, so its neighbors read a little low). Nothing beat it: tuning stopped.
Past the hand terms, what the oracle does more of (fewer ambassadors, paying threats, cheaper credits) is what full
information buys, not what a judge without it should copy.

**Probe retired: `w.built_income`** (flat mode counting a building's income at the track's rate, for 243's energy
cards). Lower agreement on plays and powers, −1.8 alone and −1.4 beside `w.hand_cost=0.2`; it moved Dyson Swarm Tranche
from 2.6 to 3.9% of games. Retired (`eval::RETIRED` names it with these numbers).

## Done when: the fresh test read (47704-47895, 192 seeds × 3 pairs a level, paired)

| | d0 | d1 | d2 | d3 | pooled d2-d3 | all |
|---|---|---|---|---|---|---|
| today's strategist | 88.9 | 78.6 | 75.3 | 55.2 | | 74.5 |
| tuned | 94.8 | 87.8 | 85.1 | 61.1 | | 82.2 |
| gap ± SE (n 576 a level) | +5.9 ± 1.5 | +9.2 ± 2.0 | +9.7 ± 2.0 | +5.9 ± 2.4 | **+7.8 ± 1.5** (n 1,152) | +7.7 ± 1.0 |

The gate (pooled d2-d3 over 2 SE, no level down by 2 SE) passes at 5 SE with every level up: **it ships as the
strategist.** By pair: bp +7.3, br +8.6, pr +7.2 (each ± 1.7). The fresh gain is above the frozen screen's (+7.8
against +5.6 at d2-d3): no sign the screens' pick was a seed fluke.

**Oracle agreement, before and after** (575 oracle games won on test seeds 47704-47751, d0-d3; the oracle lost one
d3 game of 576): same pick 52.3% → 53.8% of 345,861 decisions; plays 34.7 → 40.0%, powers 40.6 → 46.8%, Actions
(`use`) 32.6 → 36.3%, standard actions 38.7 → 47.1%, choices 81.1 → 81.1%. Greedy reads 53.2% (old judge's hand
terms, no purge or last-node terms). Every level gains 3-4 score points (agree-test.txt).

**Energy and Power cards, before and after** (share of games played in; test read, d0-d3; the oracle on its 48 seeds):

| card | today's | tuned | oracle |
|---|---|---|---|
| Dyson Swarm Tranche | 5.3% | 5.2% | 30.0% |
| Continental Supergrid | 4.1% | 3.9% | 18.8% |
| Power to the People | 12.8% | 13.1% | 50.5% |
| Tidal Barrage | 7.2% | 4.4% | 13.5% |
| Smart Grid Backbone | 0.6% | 0.8% | 1.0% |

Unmoved: they are kept (0.22-0.25 a game) and left unplayed, a gap the hand terms don't reach, and the one lever aimed
at it (`built_income`) lost games. What the hand terms do change (test read, a game, today's → tuned): cards drawn 96.8
→ 77.5, cards played 67.8 → 58.8, sales 1.24 → 0.63, credits short of a play 256 → 190 seat-card-rounds, card triggers
64.9 → 51.7; adversary ambassadors placed at d0 49.6 → 43.1, corruptions 5.95 → 4.97; rounds 9.46 → 9.06. A hand card
worth less makes drawing worth less: the judge spends on the board rather than on cards it can't afford. Dyson Swarm Tranche stays a
`design_check` kept-not-played warning (0.05 played a game); Deep State Ledger's warning clears.

**Time a game** (the test read, both arms side by side on 6 threads, 2,304 games each): agent 1,682 → 1,243 ms a game,
2.85 → 2.23 ms a decision; games are shorter (9.46 → 9.06 rounds, 590 → 557 decisions). One thread, quiet box, 32 d2 games on the
test seeds ([per-decision.txt](per-decision.txt)): 1,333 → 1,004 ms a game (0.75×), 2.2 → 1.8 ms a decision (0.80×),
fewer options a decision (26.7 → 22.4: smaller hands). (6 games from seed 1000 read 1.16× a decision on longer games;
too few to weigh.) `speed.py check`: greedy 1.01×, engine 1.00×,
planner 0.95× (unchanged agents; the check has no strategist row).

**Bands re-read on the shipped judge** (ladder.py, the test read): d0 94.8 (85-100, met), d1 87.8 (65-80), d2 85.1
(50-65), d3 61.1 (35-50): three levels over. Today's judge already read d2 and d3 over (75.3, 55.2). The bands were set
on a weaker judge; per Matt's ruling they are simply re-read here, and moving them or the difficulty is his call.

`design_check` on the tuned arm (old: today's on the same games): kept-not-played Dyson Swarm Tranche, Tidal Grid chip
converter unused, antimatter 37% spent; today's had those plus Deep State Ledger and d0 at 10.1 rounds.

## Seeds and outputs

Train: 47488-47679 (spent by 243). Test: 47704-47895, taken (ledger next free 47896); the oracle's agreement games are
its first 48 seeds. Kept (`out/keep/`): `245-test-read` (both arms, games.jsonl), `245-test-oracle` (the oracle's 576
games, the fit set and every `agree` output with its spec list), `245-screens` (s1-s4 specs and results.json; the
games, 2 GB, are not kept: rerun from the specs).
