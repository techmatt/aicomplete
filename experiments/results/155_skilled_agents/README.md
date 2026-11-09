# 155: skilled-player agents: a full-information search, and what it costs (r66)

No rules change: `RULES_REVISION` 66, `GOLDEN_FINGERPRINT` unmoved, existing agents' play unchanged (the strategist
replays its stage-A games on the final binary seed for seed; the planner's code is untouched).

## What was built

- **`oracle`** (`crates/aic-sim/src/agents/oracle.rs`, a ladder kind, opt in): the planner's search on full
  information, the Alignment deal included (D20 does not bind it: it measures, it judges no balance). Every line is
  played to the game's end on the true state. A line that wins is followed to the end without more search: the game is
  deterministic from seed and choices, so it proves the seed winnable, and the rate is a floor on perfect play. Lines
  that do not win compare on the heuristic `horizon` rounds on (the planner's judgment). Dials: `alts` (lines a
  decision), `budget` (a round), `reseeds` (lines of the rollout policy on other guesses at a round's first searched
  decision, while no win is in hand), `answers` (how deep a rollout answers a prompt), `threads` (lines side by side).
- **`threads`**: a decision's lines are played on scoped threads in batches and handed back in order, with the
  sequential loop's stops, so the choice is the same at any count: 2 games at 1 and at 4 threads played the same
  actions, and the same as the stage-A binary. 1.9 × a game at 4 (`reseeds=2` batches only 2).
- **The end-game probe on every loss** (`"all_losses": true`; `reason` added to each read). Off: as 154.
- **The skilled row** (`scripts/bench/skilled_rows.py READ.json`): the read's companion spec, the oracle and the
  strategist on its first 24 seeds a level (8 a pair), d3 and d4, the probe on every loss.
- Scripts here: `specs155.py` (the specs: three pairs, 8 seeds each, one spec a pair), `run155.sh`, `curve155.py`
  (the tables), `losses155.py`, `proven155.py` (seeds any oracle row won), `same155.py` (same games across runs).

## The cost curve (24 seeds a level, 39112-39135; full table in [curve.md](curve.md))

Time is one thread's agent seconds a game (mean, slowest). The strategist: d3 45.8, d4 12.5.

| Oracle row | d3 win % | s (max) | d4 win % (SE) | s (max) |
|---|---|---|---|---|
| defaults (`alts=2,budget=8`) | 91.7 | 15 (108) | 50.0 (10.2) | 21 (131) |
| `alts=4,budget=16,reseeds=2` | 100 | 20 (213) | 58.3 (10.1) | 38 (154) |
| `alts=8,budget=48,reseeds=4` | 100 | 39 (488) | 70.8 (9.3) | 64 (384) |
| `alts=8,budget=48,reseeds=12` | 100 | 43 (559) | 79.2 (8.3) | 65 (227) |
| `alts=8,budget=48,reseeds=32` | | | 79.2 (8.3) | 90 (510) |
| `alts=8,budget=48,reseeds=12,answers=1` | | | 70.8 (9.3) | 28 (89) |
| `alts=16,budget=96,reseeds=32,answers=1` | 100 | 9 (120) | 83.3 (7.6) | 37 (144) |
| **`alts=32,budget=192,reseeds=64,answers=1`** | **100** | 10 (145) | **95.8 (4.1)** | 61 (302) |
| the same, **fresh seeds 39136-39159** ([fresh.md](fresh.md)) | **100** | 2 (15) | **100** | 19 (102) |

All rows are `oracle:strategy=1,...`. Levers off the curve that lost: `margin=400` (d4 50.0) and comparing lines at
the game's end (`horizon=99`, d4 50.0, d3 91.7), each against `alts=4,budget=16,reseeds=2`'s 58.3. At full prompt
depth the curve stopped at 79.2 (`reseeds` 12 → 32 gained nothing for 1.4 × the time); the cheaper rollout policy
(`answers=1`: a prompt answered one prompt deep, 0.43 × a line) bought the budget that took it on. Seeds proved
winnable by some oracle row: d3 24/24, d4 23/24 (br 39126 never). The budget was picked on the curve seeds; the
fresh set is the confirmation (24/24 at both levels, against the strategist's 50.0 and 12.5 on it).

Wins are not monotone in budget on a seed: a line chosen on the heuristic when no win is found leads elsewhere
(pr 39130 was won at the defaults, lost at `reseeds=32`).

**Before amendment 6814** the planner (`planner:strategy=1`) was measured on the same seeds at four budgets: d4 16.7
/ 45.8 / 41.7 / 50.0 (`budget` 8, 16, 32, and 16 with `horizon=3`); rows in curve.md, nothing built for it. Its lines
read the decks and dice (principles.md, *Information*), so it was never a fair-information player.

## Where the time goes (`--features profile`, the same two lost d4 games, br 39120-39121, 2 threads, quiet box)

| `alts=8,budget=48,reseeds=12` | lines | ms a line | rollout actions/s (a thread) | apply | eval | clone | legal | answering prompts |
|---|---|---|---|---|---|---|---|---|
| `answers` 99 (before) | 842 | 360 | 488 | 59% | 33% | 1.7% | 0.1% | 76% of scoring, 1.23 ms each |
| `answers=1` (after) | 842 | 160 | 1163 | 42% | 45% | 2.2% | 0.1% | 48%, 0.30 ms each |

Lines are 99.7-99.9% of the oracle's time; inside them it is the strategist's own scoring (`apply_to` on a copy, then
`Eval::value`). State copies (1.1 µs) and move generation are not where the time is, so undo or cheaper copies would
buy under 3%. Transpositions: a line's policy is seeded at the line's start, so two lines reaching one state need not
continue alike; making the policy a function of the state (seeded at each round start) would allow a table, but two
alternatives of one decision rarely meet the same round-start state (every counter must match): not built. Cuts
taken: `answers=1` in the oracle's rollouts (2.4 × the rollout actions a second; it changes the oracle's lines, not
any other agent) and `threads` (1.9 × a game's wall time at 4, play unchanged). Move ordering is the strategist's
already (the shortlist is its best-scored actions, one per idea).

## Verdict

**A standard read row: yes.** `oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1` on 24 seeds at d3 and d4
costs about 28 CPU-minutes (10 s and 61 s a game), 6-7 minutes on 6 threads, its slowest game (~5 min) the floor;
`skilled_rows.py` writes it. 48 seeds a level is about 12-14 minutes: over the 10. An overnight read is not needed
for this agent: 96 seeds at d4 is about 1.6 CPU-hours, ~20 minutes on 6 threads, for an SE near 2 points at 96%.

## The losses at the top budget

[losses.md](losses.md). One loss in 48 games at the top row (br 39126 at d4, the curve set): the Discord deck ran out
in round 7 with 3 of 9 Unity cards drawn, 5 adversary cities, 43 adversary ambassadors and 65 troops on the board,
the purge 73 short (78 three rounds earlier: it was never close). The losses at the lower budgets look alike: all
d4, nearly all `discord_deck` in rounds 6-9 with 3-5 Unity cards drawn and the purge 60-120 short, and one `node`
loss in round 5. They are lost to d4's shortened Discord deck, early, not near the round cap.

## Seeds

39112-39135 (the curve), 39136-39159 (the confirmation). Next free: 39160.
