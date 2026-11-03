# 011: 007's accepted proposals, agent and harness fixes

> **Historical:** ran on rules r4 (today's is r6). The `opposition` module, parameters
> and rules.md sections named here were removed or rewritten later;
> `proposals-011.json` and two of its screens are refused today, and live in
> [`archive/`](archive/) since 016 (paths below are as they were).

2026-10-28. Rules **r4**, 2 countries, the tuned baseline. Each rule change is a variant
or a tunable (a broader overhaul is coming). Three are new defaults: the clear-board
win from stage II, and new effects for difficulty levels 1 and 3. Everything else is
off by default. Rules text: [`docs/rules.md`](../docs/rules.md) §2, §2a, §3, §7.

Results: `results/screen-011-*` (the screens that chose the defaults). No other number
was retuned.

## The measurement was stopped (Matt's message #444)

`experiments/proposals-011.json` was the one experiment the item asked for:
- 10 variants: the new defaults, r3's rules by override, opposition `off`, `income` 4,
  `free_purchases` 4, `insight` 1 and `counter_all`, `free_keep` at keep cost 2,
  `uplinks` and `uplinks` with foundries;
- d0–d4 × greedy and engine × 64 seeds, no probes.

It was stopped at 2,109 of 6,400 games (28 minutes, about 1.25 games a second): the
game is headed for a rules overhaul, so a big measurement of this ruleset teaches
little. The harness writes its results only when the grid finishes, so the stopped
run left no results. The spec is kept, ready to run.

**What is not measured:**
- the five-level curve at the new defaults;
- the opposition rewards against `off`;
- `free_keep`;
- the `uplinks` spread;
- the engine agent with its new clock default at the baseline (it was measured only on
  the cheap-tier-2 patch, below).

**What is measured** (greedy, screens below, 64–96 seeds a point):
- level 1's extra threat costs 11.5 points from d0;
- level 3's reach costs 6–9 points from d2;
- level 2's step varied from 0 to −5 between the screens' variants and seed sets
  (007: −9.5 over 200 seeds). That leaves level 2 the step most likely to sit flat at
  the new defaults: the new level 1 already takes more from d0 than level 2 used to;
- level 4 was not rerun (007: −13).

## Choosing levels 1 and 3 (screens, greedy, no probes)

Each candidate is read as the change from the level below, paired on seeds (64 a
point unless noted; `experiments/screen-011-difficulty*.json`). Target: about 5–10
points off the top agent at its level.

| Level | Candidate | Change (pts) | Verdict |
|---|---|---|---|
| 1 | r3: first requirement +2 | −7.8 (007: 0 to −3 over 200) | Noisy; it only bites a team that counters |
| 1 | every requirement +1 / +2 / +3 | −3.1 / (−4.7, −3.1, 0.0 at 96) / −4.7 | Dead: greedy stops countering and lets threats execute |
| 1 | first +2 and every +1 | −1.0 (96) | Dead |
| 1 | +1 threat per country a round | −32.8 | Far too strong |
| 1 | **+1 threat a round in all, countries in turn** | **−11.5 ±3.9 (96, p 0.007)** | **Chosen**; slightly above the band |
| 3 | r3: `foundry_bonus` 1 | 0 | Dead (007) |
| 3 | foundry reach 1 / 2 | 0 / +1.5 | Kill too little (population needs 2, buildings 3) |
| 3 | **foundry reach 3** | **−6.2 ±3.0, −9.4** (two seed sets) | **Chosen** |
| 3 | foundry reach 4 | −9.4 | No stronger than 3 |

Both new effects are printable and modelable. The extra threat is one more card a
round, its country fixed by the round number. The reach is "a ravaged foundry also
hits each adjacent hex for 3 per foundry, less defense; no corruption".

## Deck clock (agent)

`w.clock` (agents/eval.rs) charges `clock` per round the team, at its opposition rate
so far, would still be short of the **top** opposition level (an outright win) when the
adversary deck runs out. With `k` cards left, `k` action phases come after this one,
so in the last round every point short costs `clock` / rate. The engine's default is
30; greedy's is 0, so its games, which the difficulty curve is calibrated on, are
unchanged.

On 009's cheap-tier-2 patch (`t2_half+t2_noam`, separate binary; 48 seeds from 300000;
`results/screen-011-clock*`):

| Engine | d0 win % | d4 win % | d0 / d4 deck-outs (of 48) |
|---|---|---|---|
| greedy (reference) | 100.0 | 66.7 | 0 / 1 |
| engine, no clock (009: 61.5 / 35.4 over 96) | 60.4 | 31.2 | 15 / 15 |
| first form: short of the level-2 bar, clock 5 / 15 | 58.3 / 56.2 | 39.6 / 41.7 | – |
| top level, k + 1 phases, clock 10 / 30 | 56.2 / 62.5 | 45.8 / 45.8 | 14 / 7, 12 / 7 |
| **final: top level, k phases, clock 10 / 30** | 56.2 / **62.5** | 45.8 / **47.9** | 14 / 7, **12 / 6** |

- **d4: the collapse is mostly fixed.** Engine 31 → 48% and deck-outs 15 → 6. It is
  still 19 points behind greedy there.
- **d0: not fixed.** 60 → 62%, within noise, 12 deck-outs in 48. The first form's
  failure was that a team at level 2 with foundries it couldn't clear counted as on
  course (deck-out games ended at 19–23 of the 24 points for the top level). Measuring
  against the top level and only the phases still to come fixes that reading but not
  the play.
- **Why d0 isn't fixed (trace of seed 300002):** the engine builds through rounds 1–4
  with no opposition to score. Drones give none, and outposts only appear in round 4,
  so the penalty has nothing to choose between. It then needs a late sprint (0
  opposition at round 4, 25 at round 10). It plays 48 cards a game on this patch, about
  70% of threats countered. The fix belongs in how the horizon values early investment
  against the clock, not in a bigger clock weight (30 and 10 play almost alike).
- Greedy with the clock (10): 97.9 / 75.0 (d4 +8.3, p 0.29). Not made a default (it
  would move the calibrated curve), but worth a look.

## Harness

- **Probes are opt-in.** Win-probability probes were already off unless a spec set
  them. Closeness was on (every choice) and is now off by default.
  `--winprob N`, `--closeness N` and `--variants A,B` on `aic-sim experiment` give the
  two-stage pattern: screen every variant without probes, then probe the finalists
  from the same spec (README, *Experiments*). This item used only the first stage.
- **Record picks** rank agents by measured win rate at the top difficulty (as the
  report's top agent), not by ladder tier; tied agents share a rank.
- `docs/principles.md` gained the line on weighing a measure's cost against the
  decision it informs.
