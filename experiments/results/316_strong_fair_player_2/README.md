# 316: strong fair player, next rows: does a fair player reach 90% at d4?

Base: main d5719cb, **r152** (315's card pass). No rules, content or agent change: every row is an existing spec.
Fresh set C, 52210-52245 (12 seeds a pair: bp 52210, br 52222, pr 52234): 36 games a cell at d3-d4, 24 at d0-d2 for
the costly rows (each pair's first 8; Matt, amendment 14593), 36 everywhere for greedy, the judge and the full oracle. Every row plays the same seeds; gaps are paired on (pair, level, seed).
Specs `gen_specs.py` (archived as `experiments/archive/farm/f316-*`), launcher `farm_all.py`, reader `table.py`.
The item ran under a 4-hour cap and the farm was cut at 2:31am: every cell is complete but E256 at d4, 32 of 36
(br 52226-52227 and pr 52244-52245 unplayed); `table.py` reads the cut jobs' finished shards. Costs ran on a loaded
box (another item beside it; about half of each costly row's shards ran on the browser worker, whose times are not
counted): read them as ratios. Row 3 (more worlds, or 512 lines) did not run: no time left under the cap.

## The rows

E is the real oracle's search on the engine's eval (`oracle:strategy=1,worlds=4,alts=3,answers=1,truth=1,budget=N`,
314's r2 search with more lines); J128 is `real:budget=128` (the judge's eval: the yardstick since 315).

Win % ± SE (games). Gaps paired on (pair, level, seed); native seconds a game from the main box's shards.

| row | d0 | d1 | d2 | d3 | d4 | d4 gap to J128 | d3-d4 gap to J128 | native s a game: d0-d2, d3-d4 |
|---|---|---|---|---|---|---|---|---|
| greedy | 92 ± 5 (36) | 94 ± 4 (36) | 89 ± 5 (36) | 81 ± 7 (36) | 44 ± 8 (36) | −22.2 ± 10.6 (36) | −13.9 ± 6.3 (72) | 1, 1 |
| judge | 94 ± 4 (36) | 100 ± 0 (36) | 89 ± 5 (36) | 78 ± 7 (36) | 58 ± 8 (36) | −8.3 ± 10.1 (36) | −8.3 ± 6.2 (72) | 1, 1 |
| J128: judge's eval, 128 lines | 100 ± 0 (24) | 100 ± 0 (24) | 96 ± 4 (24) | 86 ± 6 (36) | 67 ± 8 (36) | - | - | 180, 320 |
| **row 1, E128: engine's eval, 128 lines** | 100 ± 0 (24) | 100 ± 0 (24) | 100 ± 0 (24) | 100 ± 0 (36) | **89 ± 5 (36)** | **+22.2 ± 9.0 (36)** | +18.1 ± 5.4 (72) | 85, 340 |
| row 2, E256: engine's eval, 256 lines | 100 ± 0 (24) | 100 ± 0 (24) | 96 ± 4 (24) | 100 ± 0 (36) | 81 ± 7 (32) | +15.6 ± 9.1 (32) | +14.7 ± 5.2 (68) | 90, 530 |
| full oracle | 100 ± 0 (36) | 100 ± 0 (36) | 100 ± 0 (36) | 100 ± 0 (36) | 100 ± 0 (36) | +33.3 ± 8.0 (36) | +23.6 ± 5.0 (72) | 2, 2 |

Steps, paired: E128 − J128 +22.2 ± 9.0 at d4, +18.1 ± 5.4 at d3-d4, +9.7 ± 2.8 at d0-d4 (144); E256 − E128 −6.2 ±
7.7 at d4 (32), −2.9 ± 3.6 at d3-d4 (68), −2.1 ± 1.9 at d0-d4 (140).

**Row 2's eval was chosen before row 1 read**: under the cap the 256-line row had to farm beside the 128-line rows,
so it took the engine's eval on 314's set-A evidence (+13.9 ± 9.0 at 32 lines). Set C agrees: E128 is above J128 at
d2-d4 and tied at d0-d1, so a judge's-eval 256-line row was not needed.

## Verdicts

- **d4 by row:** J128 67 ± 8, E128 89 ± 5, E256 81 ± 7 (the full oracle 100 on the same seeds). Native cost at
  d3-d4: J128 ~320 s a game, E128 ~340, E256 ~530 (1.6x E128); at d0-d2 the engine's eval runs half the judge's
  (its searches end sooner: a candidate winning every world stops the search).
- **The eval is the lever; at 128 lines the curve has flattened.** Swapping the judge's eval for the engine's
  inside the same 128-line search adds +22.2 ± 9.0 at d4 (2.5 SE), which 314's set A foreshadowed at 32 lines.
  Doubling to 256 lines adds nothing: −2.9 ± 3.6 at d3-d4 (68 games, the firmer read), −6.2 ± 7.7 at d4 (32), so
  the doubling's gain at d3-d4 is at most about +4 points (2 SE up). One doubling, on one seed set. 314's climb (56 → 67 → 89% at d4 for 32 → 64 → 128 lines, judge's eval,
  set A) stops at 128 on the engine's eval (set C): past about 8 decisions searched a round, more lines bought nothing here.
- **Distance from 90% at d4:** the best row, E128, reads 89 ± 5: **1 point short** of the bar (point estimate at
  least 90%, Matt, amendment 14593), well within its SE (32 of 36 won). The full oracle wins all 36: 11 points
  remain, which more lines do not close; what is left is the sampled worlds' guesses (worlds, the determinization) or
  the eval, row 3's question.
- **The yardstick:** no row reaches 90% at d4, but E128 beats `real:budget=128` on fresh seeds (+22.2 ± 9.0 at d4,
  +18.1 ± 5.4 at d3-d4), so by amendment 14593 it is named the fair yardstick: `skilled_rows.py`'s `real` row and
  rulings *The oracles*. The `real` alias (`agents/mod.rs`) is unchanged: it stays the search on the judge's eval.
- **Set C beside 314's sets** (r151, before 315's card pass, so seeds and revision both differ): the judge reads
  58% at d4 (set B 50%, set A 78%), J128 67% (set B 62%, set A 89%).

**Next, not run:** row 3 on E128's 128 lines: `worlds=8` (each decision on twice the worlds; 314's r4 tried it only
on the judge's eval at 64 lines) and `alts=5` (more candidates a decision); E128 at d4 on 36 more fresh seeds to
settle 89 against 90.

## Seeds

Fresh, all spent: 52210-52245 (set C). Ledger next free: 52246.
