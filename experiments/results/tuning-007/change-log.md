# 007 tuning run: change log

> **Historical:** ran on rules r2 and produced r3 (today's is r6). Configs, the rules
> sections and game records named here were removed or rewritten later; the records do
> not replay.

Overnight calibration of ruleset v0 against the difficulty targets in
[`docs/principles.md`](../docs/principles.md), 2026-10-27. Numbers only: no rule, effect
or phase changed. The pre-run numbers are kept in
[`configs/v0_untuned.json`](../configs/v0_untuned.json). The measures are in the
harness reports under [`results/`](results/): `tune-before`, `tune-after`,
`tune-agents-007`, `variants-spread`, `variants-modules`, `cards-t2cut`. Every sweep row is in
[`results/tuning-007/sweeps.jsonl`](results/tuning-007/sweeps.jsonl) (written by
`scripts/tune/sweep.py`).

Sweeps ran at 2 countries, difficulty 0 unless stated, on seeds 0+ (24–96 games per
point). Finals ran on fresh seeds 100000+ (200 games per cell for random and greedy).

## Diagnosis (v0 as handed over)

- **Nothing won:** 0 of 1,000 games, for every agent at every difficulty. Every game
  ended on the corruption pool, in rounds 2–7.
- **The adversary snowballs:** about 8 → 13 → 27 → 44 pieces over rounds 1–3. Every
  drone hex becomes an outpost at the next build, and every outpost seeds explore on its
  neighbors. Players destroyed 0–3 pieces a game.
- **Credits were the players' bottleneck.** Keeping drafted cards at 3 each ate the
  starting 12 credits in round 1, leaving nothing to play them with. (Greedy keeps nearly
  everything it drafts: its hand value ignores affordability. Humans would keep less, so
  `keep_cost` sensitivity is partly the agent's.)
- **Difficulty 3 does nothing:** levels 2 and 3 play identically, even with
  `foundry_bonus` 5. Foundries grow where pieces pile up (uplinks, outpost hexes), which
  hold no population or buildings, so the extra damage there destroys nothing.

## Agent fixes (aic-sim only; no rules)

Two harness problems would have mis-ranked the ladder. Both are fixed and committed
separately.

1. **Beam never grew its engine.** It scored an action with only one level of its own
   choices resolved. Placing a processor asks for a track and then a hex, so it was
   scored with no processor placed. Beam took "gain 8 credits" every round and placed
   about 1 processor a game, against greedy's 10. Beam and rollout now take `choices`
   (default 2, as greedy). On an interim candidate: beam 3.1% → 93.8% won (32 games).
2. **Pool weight fitted to v0's 6-corruption pool.** On the tuned 22-corruption pool,
   `w.pool` 12 made beam buy back corruption (Rebuild) instead of purging, and it ran out
   the deck. Defaults are now `w.pool` 6 and `w.opposition` 6: beam 58% → 96% (24
   games), greedy 94% → 95% (64).

Even after both fixes, **greedy is the strongest agent on the tuned game.** At
difficulty 4 (48 games): greedy 56%, `greedy:samples=3` 54%, `greedy:depth=2` 46%, beam
50%, rollout 42% (24 games). More search exploits the heuristic's blind spots: it has no
term for the adversary deck running out. Difficulty 4 is calibrated to greedy.

## Changes

Each change was checked by leaving it out of a candidate set (64–96 games). The
"without" column is measured on the candidate it was tested in, not on the final set: a
leave-one-out on the final set was skipped at wrap-up. Small changes that each looked
dispensable were not additive: dropping four of them together cost 40 points.

| # | Tunable | v0 → tuned | Why | Greedy d0 win %: without → with (64–96 games) |
|---|---|---|---|---|
| 1 | `cards.keep_cost` | 3 → 1 | Credits were the bottleneck: keeping cards left nothing to play them with | 64 → 95 |
| 2 | `board.start_outposts` | 1 → 0 | The printed outpost seeds explore from round 1; without it the spread starts a round later | 76 → 97 (interim set) |
| 3 | `adversary.corruption_at` | 2 → 3 | A lone outpost hex (2 damage) no longer corrupts; corruption needs a stack | 66 → 95 |
| 4 | `adversary.stage_cards` | 3,4,3 → 4,4,2 | Fewer double-terrain stage-III cards; the deck is still 10 cards (the clock) | 77 → 95 |
| 5 | `adversary.pool_per_country` | 2 → 10 | The pool (6 at 2 countries) emptied by round 4; now 22. Per country, to scale "for each country" | 92 (8/country) → 95 |
| 6 | `economy.factory_credits` | 1 → 3 | Pays for the cards that purge; with it, outpost health could stay at v0's 2 and games last 8+ rounds | 57 (2) → 92 (3) |
| 7 | `adversary.counter_bonus` (d1+) | 1 → 2 | Level 1 barely moved the win rate (95 → 95); 2 gives 92. 3 is no stronger | – |
| 8 | `adversary.pool_reduction` (d4+) | 1 → 4 | Level 4 barely bit (86 → 83); pool 22 → 14 at difficulty 4 gives about 67 | – |

Considered and not kept:

- **Outpost and foundry health 1/2.** Strongest single lever (+21), but games ended in
  round 6. Factory credits 3 (#6) wins as often with v0's health and 8+ rounds.
- **Start credits 16, opposition thresholds** (`per_player` 2–10, `outpost` 2, `foundry` 3).
  Either no effect, or a shorter game for the same win rate. Opposition stays at v0.
- **`stage1_removed` 2** (d2: 86 → 75) and **`tier2_early` 3** (no effect).
- **No Momentum or Escalation:** within noise.

## Measures before and after

2 countries. Before: `results/tune-before` (v0 untuned, seeds 0–49, 50 games per cell,
agents as fixed above). After: `results/tune-after` (seeds 100000–100199, 200 games per
cell, no win-probability probes). Win % ± standard error. Rows marked † come from runs
on the same configs and seeds whose files are not in the repo: the search-agent run
(24 games per cell) and an earlier probed run of `tune-after`. Both were deleted before a
rerun, and the rerun was stopped at wrap-up.

| Measure | Before (v0) | After (tuned) | Target |
|---|---|---|---|
| Random, d0–d4 | 0% everywhere | 0% everywhere (0/1,000) | almost never |
| Greedy, d0 / d1 / d2 / d3 / d4 | 0 / 0 / 0 / 0 / 0 | 95.5 / 95.0 / 86.0 / 86.0 / 73.0 (±1.5–3.1) | d0 almost always; falls smoothly |
| Beam, rollout: d0 / d4 † | 0 / 0 | 87.5 / 70.8 each (24 games); greedy on the same seeds 100 / 75 | strongest wins narrowly at d4 |
| Rounds, greedy d0 / d4 | 3.8 / 3.1 | 7.75 / 7.04 | 8–10 |
| Win margin at d4: deck cards left / pool left | – | 1.8 / 8.6 of 14 | narrow |
| Ended by (all 2,000 after games) | corruption 100% | opposition 871, corruption 1,067 (nearly all random), deck 31, population 31 | – |
| Adversary operations per round, greedy | 13.9 | 9.5–10.8 (max 47) | low |
| Decisions per game / forced / close, greedy d0 | 108 / 6% / 75% | 214 / 4% / 67% | few forced |
| Uncertain win-probability checkpoints, greedy d0 / d4 † | 0% / 0% (ladder-v0) | 15% / 48% | uncertain late |
| Decided by round 2, greedy d0 / d4 † | 100% / 100% (ladder-v0) | 60% / 0% | rare |
| Luck share (seed's share of the variance) of survival, d0 / d4 | 55% / 88% | 7% / 29% | mostly decisions |
| Luck share of win/loss, d0 / d4 | – (nothing won) | 4% / 21% | mostly decisions |
| Threats countered, greedy d0 / d4 | 38% / 5% | 37% / 6% | – |

**Difficulty curve:** three steps, not four. Levels 0 and 1 play almost alike
(`counter_bonus` moves greedy by 0–3 points: it counters few threats either way). Levels
2 and 3 are identical (`foundry_bonus` is dead, above). The live steps are level 2
(−9.5 points) and level 4 (−13). Numbers can't separate levels 1 and 3; see the
proposals.

**Skill gap:** with random at 0%, the gap is greedy's own win rate, so it narrows
(+96 → +73) where the target wants it to widen. No stronger rung exists today: every
search agent scores at or below greedy.

## Variant comparison (tuned numbers; indicative)

Greedy, 96 games per cell, seeds 200000+ (`results/variants-modules`,
`results/variants-spread`). Random won 0–1% everywhere.

| Variant | d0 win % | d4 win % | Note |
|---|---|---|---|
| **baseline** (pushback, countries, Militarist) | 93.8 | 65.6 | |
| opposition `income` | 92.7 | 63.5 | |
| opposition `free_purchases` | 91.7 | 63.5 | |
| opposition `counter_threat` | 92.7 | 68.8 | |
| opposition `off` | 91.7 | 63.5 | **The reward doesn't matter:** all four rewards are within noise of none |
| threats `shared_row` | 75.0 | 21.9 | Every threat acts in every country: far harder; would need its own numbers |
| threats `off` | 99.0 | 97.9 | **Threats carry the difficulty:** without them d4 is as easy as d0 |
| cards `off` | 0.0 | 0.0 | Cards are the whole of the players' offense |
| spread `clusters` | 1.0 | 0.0 | Lost on population by round 4 (95 of 96); untuned |
| spread `assets` | 4.2 | 0.0 | Lost on the pool by round 2. Its pool (3 per country) is its asset cap; `pool_reduction` 4 emptied it at d4, so assets keeps 1 |

The spread variants share the Militarist's deck and the tuned board, economy and card
numbers, and were first guesses before tonight. Both are far too strong, so comparing
them to the baseline needs a tuning pass of their own. The agents now see clusters and
corrupted assets (they were blind to them, and crashed on `assets`), but a heuristic
written for the Militarist plays them weakly.

## Card health

Judged with `greedy:w.built_cost=1.2,w.hand=1,w.hand_cost=0.2`, which builds and plays
economy programs: plain greedy plays no buildings and holds cheap programs (its hand
value beats their effect). 320 games (d0 and d4, `results/tune-agents-007`).

- **Must-picks:** Precision Strike (3.9 plays a game), Contract Bid (4.4), Rally (2.4),
  Sweep (2.3), and tier-2 Purge Protocol (1.3, played 88% of the times it is drawn). The
  game is won on opposition, and these are the opposition cards.
- **Dead (played under 15% of the times gained):** Counter-intelligence (0%),
  Emergency Fab (0%), Liquidate Reserves (0%), Power Share (0%), Recon Sweep (0%),
  Scavenge (1%), Evacuate (13%), and four tier-2 buildings: Megastructure (6%), Quantum
  Core (10%), Orbital Lance (13%), Antimatter Refinery (15%). Shelter Order (Protector
  unique) was never played.
- **Tier 2 doesn't pay off,** except Purge Protocol. A 30% cost cut on the other nine
  (`results/cards-t2cut`, same seeds, a separate binary) moved their play rates by under
  0.1 a game, and Counter-intelligence stayed at 0 at cost 3. Win rates were within
  noise (95.0 → 93.1 at d0, 69.4 → 64.4 at d4). **No cost change applied:** cost isn't
  what holds these cards back.
- **Buildings don't pay back in an 8-round game:** the building agents won 1–10 points
  less than plain greedy, which never builds (same seeds: d0 96 / 91 / 95, d4 74 / 64 /
  69).

## Proposals (structural; not built)

1. **Difficulty 3 needs an effect that bites.** Foundries stand on uplinks, which hold
   no player pieces. Options: "foundries ravage adjacent hexes too", "+1 corruption on a
   ravage with a foundry", or move `pool_reduction` to level 3 and give level 4 a new
   effect.
2. **Difficulty 1 barely moves a strong team.** A stronger level 1: "+1 threat per
   country per round", or "threats' every requirement +1", not only the first.
3. **Difficulty 4's `tier2_early` does nothing measurable;** `pool_reduction` is its
   whole effect.
4. **The opposition reward choice doesn't matter** (all four ≈ off). Make rewards
   bigger or tie them to what decides games (threats: `counter_threat` was the best,
   +3 points at d4), or drop the module.
5. **Engine depth is weak:** buildings don't pay back, and tier 2 is antimatter-gated.
   Ideas: give tier-1 economy buildings a one-off effect on entry; an antimatter source
   on tier-1 (or on the AI tracks' middle values); unlock tier 2 a track step earlier.
6. **The draft keep step at 1 credit is nearly free.** Consider "keep one card free
   each round, others at 2".
7. **Win routes and an early clear-board win.** Greedy's wins (96 games,
   `variants-modules`, pushback) end at opposition level 3 (d0 67%, d4 48%), level 2
   with no foundries (27% / 37%), or levels 0–1 (7% / 16%). With no starting outpost,
   the opening board is 4 drones, and Sweep or Watchtower Net can clear it: "no adversary
   pieces" at level 0 then wins in round 2. A random team did this
   (`variants-modules` record `surprising-win-…-seed200032`); greedy did it in 3–4% of
   games. Proposal: the level-0 clear-board win only counts once stage II has begun,
   or start the adversary with an outpost again and offset it elsewhere.
8. **Explore from outposts makes spread exponential** (every drone hex becomes an
   outpost at the next build, and each outpost seeds explore around it). The numbers
   now hold it to about 10 operations a round, but explore from uplinks only, with
   outposts feeding build, would give a linear clock that is easier to read.
9. **Spread variants** need their own tuning pass (both are lost by round 2–4 at
   these numbers).
10. **Agents (005 follow-ups):** the heuristic has no term for the adversary deck
    running out, values cards in hand by cost and not affordability, and values
    buildings at less than their cost. More search exploits these, so beam and rollout
    score below greedy. Until it is fixed, "the strongest agent" means greedy.

## Records worth a look

All replay under rules revision 3 (`aic-sim replay`), and open in the viewer.

1. `results/tune-after/records/close-loss-c9-baseline-base-d4-greedy-seed100019.json`:
   the narrowest d4 loss. The pool ran out in round 8 with one piece over the level-2
   bar. This is what "narrow" looks like at the top.
2. `results/tune-after/records/close-loss-c9-baseline-base-d4-greedy-seed100187.json`:
   a d4 loss on the clock (the deck ran out in round 9) with 9 of 14 pool left. Greedy
   never sees the deck running out.
3. `results/variants-modules/records/surprising-win-c0-baseline-pushback-d0-random-seed200032.json`:
   a random team wins in round 2 by sweeping the 4 opening drones (proposal 7).
4. `results/variants-modules/records/close-loss-c23-baseline-shared_row-d4-greedy-seed200036.json`:
   the shared threat row at d4. Every threat hits both countries, and greedy wins 22%.
5. `results/variants-spread/records/close-loss-c7-clusters-base-d4-greedy-seed200014.json`:
   clusters at the tuned numbers, lost on population in round 6. What the spread
   variant does before its own tuning pass.
