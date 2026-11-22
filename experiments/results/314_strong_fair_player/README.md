# 314: a strong fair-information player at r151

Base: main 897b72f, r151. No rules or content change; `RULES_REVISION` stays 151 and greedy's and the judge's play is
unchanged (the two retired weights were 0). Readers: `table.py` (the tables, `--set B` for the strongest run),
`screen.py` (the option screen); specs: `gen_specs.py` (in `experiments/archive/farm/`, `f314-*`, `s314-*`, `g314*`);
launcher `farm_all.py`. Every timing ran on a loaded box (another item and conductor beside it): read costs as ratios.

**What changed in code.** `real` is now an alias (`agents/mod.rs`): the real oracle (`worlds=4,alts=3,budget=32,
answers=1,truth=1`) on `STRATEGIST`, so `skilled_rows.py`'s `real` row tracks the judge. `tries` is taken on the real
oracle as a measuring arm (a restart reads the true loss: never a fair player). Two new real-oracle options, off by
default: `phases=1` (search the draft too) and `lone=1` (a lone candidate unsearched). `w.track_insight` and
`w.sink_idle` are deleted (`eval::RETIRED`).

## 1. The decomposition (fresh set A: 51936-51953, 6 seeds a pair, 18 a cell)

Each row is the real oracle (`real`: the judge's eval, 4 worlds, 32 lines a round, which searches about the round's
first 2 action-phase decisions) with one thing changed. Win % ± SE (games); the gap is paired on (pair, seed), d3-d4.

| row | d0 | d1 | d2 | d3 | d4 | d3-d4 gap to r0, paired | native s a game |
|---|---|---|---|---|---|---|---|
| greedy | 89 ± 7 (18) | 94 ± 5 (18) | 89 ± 7 (18) | 89 ± 7 (18) | 61 ± 11 (18) | +2.8 ± 7.4 (36) | 0.8 |
| judge | 94 ± 5 (18) | 100 ± 0 (18) | 94 ± 5 (18) | 83 ± 9 (18) | 78 ± 10 (18) | +8.3 ± 10.1 (36) | 0.8 |
| real oracle (r0: `real`) | 94 ± 5 (18) | 94 ± 5 (18) | 100 ± 0 (18) | 89 ± 7 (18) | 56 ± 12 (18) | - | 126 |
| r1, step 1: `real:tries=3` (not fair) | 100 ± 0 (18) | 100 ± 0 (18) | 100 ± 0 (18) | 100 ± 0 (18) | 67 ± 11 (18) | +11.1 ± 5.3 (36) | 101 |
| r2, step 2: the full oracle's eval | 100 ± 0 (18) | 100 ± 0 (18) | 100 ± 0 (18) | 89 ± 7 (18) | 83 ± 9 (18) | +13.9 ± 9.0 (36) | 97 |
| r4, step 3: 8 worlds (64 lines) | 100 ± 0 (18) | 89 ± 7 (18) | 100 ± 0 (18) | 83 ± 9 (18) | 72 ± 11 (18) | +5.6 ± 7.9 (36) | 178 |
| r5, step 3: 64 lines a round | - | - | - | 83 ± 9 (18) | 67 ± 11 (18) | +2.8 ± 9.3 (36) | 229 |
| r3, step 3: 128 lines a round | 100 ± 0 (18) | 100 ± 0 (18) | 100 ± 0 (18) | 94 ± 5 (18) | 89 ± 7 (18) | +19.4 ± 7.8 (36) | 325 |
| full oracle | 100 ± 0 (18) | 100 ± 0 (18) | 100 ± 0 (18) | 100 ± 0 (18) | 100 ± 0 (18) | +27.8 ± 7.6 (36) | 1.7 |

r2 is `oracle:strategy=1,worlds=4,alts=3,budget=32,answers=1,truth=1`; r5 (the curve's middle point, added after
the diff review) ran d3-d4 alone. Seconds a game are native shards' means on a loaded box (ratios only).

- **Lines a round are the lever.** 32 → 64 → 128 lines a round (the decisions searched each round: about 2 → 4 →
  8) reads 56 → 67 → 89% at d4 (d3 89 → 83 → 94). The same 64 lines spent on 8 worlds instead (r4: more worlds, the same 2 decisions)
  reads 72%. The real oracle's weakness was not its sampling but how few decisions it searched.
- **The eval is worth as much** (step 2): the same 32-line search on the full oracle's eval (engine weights,
  `strategy=1`) reads 83% at d4, +13.9 ± 9.0 at d3-d4. 249's discarded engine-weights version (3 worlds) read 55% to
  the judge's 70% on r112; at r151 the judge's eval is the weaker one inside the search.
- **Restarts are worth about 10 points** (step 1, not fair): tries=3 reads 67% at d4, +11.1 ± 5.3 at d3-d4.
- **The full oracle's edge over the real oracle** is +27.8 ± 7.6 at d3-d4 here (44 points at d4): most of it is
  search effort and eval, not sight of the deck; the 4x-lines player leaves 11 points at d4 (89 against 100).
- Set A is an easy set: the judge reads 78% at d4 (37-56% on 313's sets), above the 32-line real oracle (56%).

**The option screen** (d4, 313's spent seeds 51552-51559, 24 games an arm, paired on base): `lone=1` +4.2 ± 4.2,
`phases=1` (with 64 lines, so it changes two things) +0.0 ± 10.4, both +4.2 ± 11.2; base 41.7%. Neither is shown to
help; the strongest player uses neither. (The diff review found `phases=1` could search a pending prompt whose world
keeps the table's reshuffle streams; 314 now searches outside the action phase only with no prompt pending. The
screen's phases arms ran before that guard: no reported result rests on them.)

## 2. The strongest fair player (fresh set B: 51954-51969, 16 seeds a cell)

`real:budget=128`: the real oracle on the judge's eval, 4 worlds, 128 lines a round. Chosen on set A (its best row);
set B is its fresh read, with greedy, the judge and the full oracle on the same seeds.

| row | d0 | d1 | d2 | d3 | d4 | native s a game |
|---|---|---|---|---|---|---|
| greedy | 100 ± 0 (16) | 100 ± 0 (16) | 88 ± 8 (16) | 69 ± 12 (16) | 44 ± 12 (16) | 1.3 |
| judge | 94 ± 6 (16) | 94 ± 6 (16) | 88 ± 8 (16) | 81 ± 10 (16) | 50 ± 12 (16) | 1.3 |
| **strongest: `real:budget=128`** | 100 ± 0 (16) | 88 ± 8 (16) | 94 ± 6 (16) | 100 ± 0 (16) | **62 ± 12 (16)** | **470** |
| full oracle | 100 ± 0 (16) | 100 ± 0 (16) | 100 ± 0 (16) | 100 ± 0 (16) | 100 ± 0 (16) | 2.8 |

Paired on (pair, seed): the strongest against the judge +15.6 ± 7.9 at d3-d4 (+12.5 ± 12.5 at d4), +7.5 ± 4.6 all
levels; against the full oracle −37.5 ± 12.5 at d4.

**Cost:** about 330-470 native seconds a game (set A 325, set B 470, each on a loaded box), about 3-4x the 32-line
real oracle's (101-126 s) and some 400x the judge's; a 16-seed d0-d4 row (80 games) is ~10 native CPU-hours, ~45 min
on the 6 main slots and the 8-core browser worker.

**Against 90% at d4:** 62% on set B, **28 points short** (the full oracle 100% on the same seeds); 89% on set A, the
set it was picked on. Pooled, 26 of 34 (76%). **The curve is still climbing:** at d4 on set A, 56 → 67 → 89% for 32 →
64 → 128 lines a round, each doubling worth 11-22 points (SE ~11 a cell); and the eval step alone (r2) adds about as
much as the first doubling. More search, and the full oracle's eval inside it, would close more of the gap; nothing
here says where it flattens. The full oracle's edge is not mainly sight of the deck: search effort and eval account
for most of it on set A.

**Next, not run here:** the full oracle's eval with 128 lines (r2 + r3, a ~4x row), then 256 lines; restarts can't
join a fair player.

## 3. Greedy against the judge (fresh 51888-51935 and 51970-52065: 144 seeds, three pairs, d0-d4)

Paired on (pair, seed), 2,160 games a side:

| level | greedy | judge | greedy − judge | SE |
|---|---|---|---|---|
| d0 | 96.1 | 96.8 | −0.7 | 1.2 |
| d1 | 93.1 | 95.4 | −2.3 | 1.5 |
| d2 | 91.7 | 94.0 | −2.3 | 1.7 |
| d3 | 73.8 | 78.7 | −4.9 | 2.5 |
| d4 | 48.1 | 50.2 | −2.1 | 2.9 |
| all | 80.6 | 83.0 | **−2.5** | **0.9** |
| d1-d4 | 76.7 | 79.6 | −2.9 | 1.1 |

**The order is settled: the judge above greedy**, by 2.5 ± 0.9 points overall (2.8 SE), every level the same way.
The two halves agree (g314, 48 seeds: −1.2 ± 1.7; g314b, 96 seeds: −3.1 ± 1.1). 313's 48-seed ladder had greedy +4.6
over r151's judge (w.purge 3.6, reach_rule off); 313's shipped judge reverses it.

## Seeds

Fresh, all spent: 51888-51935 (g314), 51936-51953 (set A), 51954-51969 (set B), 51970-52065 (g314b). Screens on
313's spent 51552-51559. Ledger next free: 52066.
