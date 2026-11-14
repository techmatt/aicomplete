# 222: sim players fit to judge (r97)

No rules or content change; `RULES_REVISION` stays 97. Base: main efce080 (221 on 219's r97; 221 left play
unchanged). Specs: [spec.py](spec.py). Readers: [read.py](read.py) (agents paired on pair, difficulty and seed;
`--other` pairs a kept read), [am_use.py](am_use.py) (burns by round, plants by Action use, per agent). The
no-op smoke is `scripts/tune/noop_weights.py`.

## What separates the strategist from greedy

`strategist` is `engine` with `STRATEGIST`'s overrides, which set every `Weights::engine` field back to greedy's flat
default. What is left: `strategy=1`, `w.purge` 3.6, `w.last_node` 120 and the agent's RNG stream (keyed by kind).

## Screens

Screen 1 (`s222-fac`, dev seeds 36800-36831, 384 games a cell, d0-d3; gap to the strategist, paired):

| agent | win | gap |
|---|---|---|
| strategist (r97) | 55.2 | |
| greedy | 66.4 | +11.2 ± 2.8 |
| `strategy=0` | 63.5 | +8.3 ± 2.5 |
| `w.purge=0` | 63.0 | +7.8 ± 2.7 |
| `w.last_node=0` | 56.2 | +1.0 ± 1.7 |
| all three off | 63.8 | +8.6 ± 2.6 |
| greedy `strategy=1` | 63.8 | +8.6 ± 2.9 |

Screen 2 (`s222-lev`, same seeds): `purge_one` (the purge term toward the largest single Liberate bill, a trial
switch since removed) +1.3 ± 1.1; `w.purge=1.8` +3.1 ± 2.5; `w.strat_scale=0.5` +5.7 ± 2.3; both +4.7 ± 2.3;
**`t.keep_tier2=7.5`: +0.0 ± 0.0, every game the same** (below).

The first fresh read (`f222`, 46720-46751, 32 seeds, 384 games a cell) tested the purge term off: −1.0 ± 2.6 against
the r97 judge, and greedy led the r97 judge by only 1.3 ± 2.6 there. The dev seeds had overstated the gap.

Screens 3 and 4 (`s222-am`, `s222-bl`, 219's spent fresh seeds 46624-46719, 96 seeds, 1,152 games a cell, d0-d3;
greedy is 219's kept `f219g`, the same r97 play; the r97 judge reproduced 219's 81.9 / 62.5 / 49.3 / 39.6 game for game):

| agent | d0 | d1 | d2 | d3 | all, gap to the r97 judge |
|---|---|---|---|---|---|
| r97 judge | 81.9 | 62.5 | 49.3 | 39.6 | |
| greedy | 82.6 | 69.1 | 53.5 | 44.1 | +4.0 ± 1.6 |
| `w.purge=0` | 86.5 | 58.7 | 49.0 | 43.1 | +1.0 ± 1.5 |
| `strategy=0` | 86.1 | 65.6 | 49.3 | 48.6 | **+4.1 ± 1.6** |
| `w.strat_scale=0.5` | 84.4 | 62.5 | 51.4 | 43.8 | +2.2 ± 1.4 |
| `burn_last` (power after cubes) | | | | | −0.2 ± 0.2 (with purge 0: +0.0) |
| `burn_late` (no burn before the deck's last 2 cards) | 81.6 | 63.2 | 46.9 | 37.2 | −1.1 ± 0.5 |

**The deficit is the strategy commitment**: `strategy=0` gains all of greedy's lead. The purge term, and the
dev-seed interaction it seemed to have with strategy, did not hold on 96 seeds. First taken: `w.strat_scale` 0.5
(the strategy kept at half stretch), as the default of design question 1. **Matt's answer (message 10001): the
judge drops strategy commitment, `strategy=0`**: greedy plus the purge term and the last-node guard.
`strategist:strategy=1` stays as an information cell (and `aic-sim strategies`' default agent).

## Fresh read (`f222b`, 46752-46783, 32 seeds × 3 pairs, d0-d3)

384 games a cell and agent, on one build: the r97 judge is `strategist:w.strat_scale=1`, which plays as r97's
`strategist`; "strat 0.5" is the build's then-default judge; **`strategy=0` is the shipped judge** (with no strategy
chosen, `w.strat_scale` reaches nothing, so it plays as the final `STRATEGIST`). Gaps paired on pair, difficulty and seed.

| d | strat 0.5 | r97 judge | 0.5 − r97 | greedy | 0.5 − greedy | **`strategy=0`** | **`strategy=0` − greedy** |
|---|---|---|---|---|---|---|---|
| d0 | 83.3 | 84.4 | −1.0 ± 4.8 | 82.3 | +1.0 ± 4.1 | 86.5 | +4.2 ± 4.7 |
| d1 | 67.7 | 70.8 | −3.1 ± 5.6 | 74.0 | −6.2 ± 6.1 | 75.0 | +1.0 ± 6.2 |
| d2 | 57.3 | 45.8 | +11.5 ± 4.2 | 52.1 | +5.2 ± 5.6 | 55.2 | +3.1 ± 6.2 |
| d3 | 36.5 | 33.3 | +3.1 ± 5.0 | 37.5 | −1.0 ± 6.4 | 44.8 | +7.3 ± 6.3 |
| all | 61.2 | 58.6 | +2.6 ± 2.5 | 61.5 | −0.3 ± 2.8 | 65.4 | +3.9 ± 2.9 |

**The shipped judge (`strategy=0`) reads at or above greedy at every level, d0-d3** (+4.2 / +1.0 / +3.1 / +7.3,
+3.9 ± 2.9 pooled), and +6.8 ± 2.6 over the r97 judge (paired, the r97 judge column). `strat 0.5` tied greedy pooled
(−0.3 ± 2.8) and missed d1 by 0.1 SE. 32 seeds give an SE of 4-6 a level; the pooled gaps are the firmer read. On
219's 96 seeds (screen 4, chosen there, so not a confirmation) `strategy=0` read +4.1 ± 1.6 over the r97 judge,
level with greedy. Rounds move little (8.88 against 8.80 pooled).

## keep_tier2

`values.json` `rates.keep_tier2` (6.5) is read only in table mode (the engine's and planner's packet keep,
`eval.rs`), by `FreePurchases` (tier 1) and by the pricing scripts (`payback.py`, `cards_doc.py`). Greedy and the
strategist run flat (`w.table=0`): their packet keep is 2 credits an insight of the config's keep (`keep_insight`,
3 for tier 2; the upgraded keep of 2 is not read). At 7.5 (a 3-insight keep at 2.5) every strategist game played the same
(1,152 games). Left at 6.5: no judge reads it, and a resize moves the generated pricing docs (a pricing item's).

## Antimatter burns and plants (screen 3, strategist, a seat-game of the board's seats; [am-s222-am.txt](am-s222-am.txt))

| | uses | mean round | in rounds 1-4 | in the last two |
|---|---|---|---|---|
| Teardown | 0.31-0.34 | 4.0 | 63-66% | 16-19% |
| Overload | 0.34 | 7.4 | 10% | 29% |
| Seize | 0.12 | 3.5 | 68% | 19% |

Overload is rare and late, as asked. **Teardown and Seize fire early.** Two sentence-sized fixes were screened and
dropped: `burn_last` (a board power after Make antimatter's cubes in `make_short`) moved almost no game; `burn_late`
(no board burn before the deck's last two cards) cut Overload to 0.10 but left Teardown at 63% early (it fires
through `make_short`, outside the search's filter) and cost 1.1 ± 0.5. Plants: Penning Trap Array, Positron Storage
Ring, Dark Fiber Collider and Cyclotron Lease make 1.2-3.7 antimatter a seat-game once built; **Antiproton Foundry
(1 chip) and Treaty Isotope Bank (8 credits) are built in 19-30% of games and their Actions almost never used
(0.05-0.10)**: they are played for their on-play gain, as 219 found. Not fixed here.

## No-op weights

`scripts/tune/noop_weights.py` moves each of the 108 `Weights` fields (to 0, or to 1 where the judge has 0) and lists
those whose games all match the judge's. Run on the shipped judge, Builder + Protector, d2, dev seeds 36800-36803
(4 games a weight, cut to fit the cap; the script's default is d1-d2 at 8 seeds): [noop.txt](noop.txt).
**27 change no game**, among them `w.win_margin`, `w.discord_danger`, `w.project_cube`, `w.research_cube_power`,
`w.shelter_counter`, `w.shelter_armor`, `w.station_round`, `w.factory`, `w.liberate_progress`, `w.patience`,
`w.unity_rate_start`, `w.risk`, `w.ability`, `w.hand_share`, `w.needs_floor`, `w.power_progress`, `w.energy_idle`,
`w.cube_target`; the rest (`w.production`, `w.movement`, `w.pick_cost`, `w.strat_*`, `w.insight_income`,
`w.tag_hand`, `w.spend_horizon`, `w.table_ambassador`) are table- or horizon-mode terms the flat judge never reads.
Four games is a screen, not a verdict: a listed weight is a candidate to read on more seeds before it is retired.
`hand_worth`'s antimatter branch (218) is code, not a weight, so the smoke can't reach it; in flat mode it zeroes a
hand card short of antimatter with no antimatter income, which the plants' Actions never count as.

## Speed

`per_decision.py` (base efce080's build against the final one, c2 d2, 4 games from seed 1000, one thread, best of
1, with the test build running beside it: ratios only): **strategist 0.76× a decision** (1.2 against 1.6 ms;
0.75× options a decision, no strategy multipliers in its keep plan); greedy plays the same games (1.00× decisions),
1.16× ms, the load (no greedy code changed). `speed.py check` not run (the box was not quiet); no `--update`.

## Seeds

Dev seeds 36800-36831 (screens 1-2, the no-op smoke). 219's spent 46624-46719 (screens 3-4). Fresh 46720-46751
(`f222`) and 46752-46783 (`f222b`), taken in the ledger. Kept: `out/keep/f222b.jsonl`, `out/keep/s222-bl.jsonl`.
