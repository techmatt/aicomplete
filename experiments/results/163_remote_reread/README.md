# 163: cross-engine pairing, and 161's remote read redone cleanly

Rules r69, build 99d3086 + 163's script changes (no engine change). The `read` spec (`experiments/farm/read.json`:
the strategist, three pairs, d0-d3, 12 cells a seed). Amendment #7385 and the answer to question 1 cut part 2 to
**32 seeds, 40072-40103**, `shard_seeds` 4 and `lease_seconds` 600 left as they are: 8 shards a run. The per-run
detail (every shard, its leases, re-runs, the reclaim) is in [runs.md](runs.md), written by [runs.py](runs.py) from
each job's `job.json`, `main.log` and a board record taken during the run ([kill_at.py](kill_at.py)).

## Part 1: the read tools

- **Cross-engine pairing.** `screens.pair_seeds(a, b, same_engine=False)` pairs every shared seed and counts those
  on mixed engines. `paired.py` and `ladder.py` default to it and take `--same-engine` for 160's rule. The mixed
  count prints as information (`... paired: the engines play alike`).
- **The win % mismatch** (161: d0 95.1 against 94.7 on identical lines). `ladder.paired` returned the old side's
  rate over the *paired* keys only, while the new column used all of the new side's games. With 276 of 576 seeds
  left out as mixed, the two columns covered different seeds. The old column is now its side's whole rate; only the
  gap and its SE are paired.
- `scripts/bench/test_read_tools.py` builds two identical reads, one all native and one with half its seeds on wasm,
  where the odd and even seeds win at different rates. It checks that both win % columns agree (by default and under
  `--same-engine`), that the gap is +0.0 with no seed left out, and that `paired.py` counts the mixed seeds. All three
  tests fail on the old scripts and pass on the new.

## Part 2: correctness

| run | what | wall | output against A |
|---|---|---|---|
| A | `aic-sim experiment read --seed 40072 --games 32 --threads 6` | 342.4 s | - |
| B | `farm-seeds`: main box (6 slots) + Matt's laptop tab (7 cores) | 565.3 s (box loaded) | 0 of 384 games differ |
| C | `farm-seeds`: main box + a headless Edge tab (2 cores), killed at +118 s | 424.3 s | 0 of 384 differ |
| B2 | B again on a quiet box, laptop sat out (gate stuck) | 410.3 s | 0 of 384 differ |
| B3 | B again from a committed tree, laptop sat out (gate stuck) | 422.0 s | 0 of 384 differ |
| B4 | B again, quiet box, the laptop tab gated again (10:39pm) | 283.0 s | 0 of 384 differ |
| B5 | 56 seeds (40072-40127), so the tab runs on all 7 cores | 977.0 s | 0 of 384 differ (A's seeds); = B4 |

- Games are compared after `engine` and every timing are set aside (`scripts/farm/compare.py`). B's file equals C's
  byte for byte once `"engine":"wasm"` reads `native`.
- **`ladder.py` B against A:** the gap is +0.0 ± 0.0 at d0-d3, on all 96 games a difficulty, with no seed left out.
  24 seeds a difficulty were played on mixed engines and paired. Both win % columns are equal: d0 100.0, d1 88.5,
  d2 81.2, d3 57.3. Under `--same-engine` the columns stay equal and only the gap's seeds drop. C against A gives the
  same table, with no mixed seeds.

## Part 2: speed

Seeds a minute a core, from each shard's ms (4 seeds a shard; a shard runs on one core):

| worker | run | rate | note |
|---|---|---|---|
| main box, experiment threads | A | 0.97 | per-game ms, 6 threads; 5.61 seeds a minute on the box by wall |
| main box, farm shards | B4, B2, B3, C | 1.13, 1.17, 1.12, 1.12 | 6 at once; B2's first round alone is also 1.17 |
| main box, farm shards | B | 0.56 | loaded: conductor reported 0 free threads (Defender 15%, an outside python 17%) |
| laptop tab, wasm | B4 | **1.22** | 2 shards on 2 of its 7 cores, after the tab gated again (reloaded?) |
| laptop tab, wasm | B | 0.54 | the same 2 of 7 cores, at 9:53pm; why it ran at half B4's rate is unknown (the main box's load can't reach it) |
| headless Edge tab | C | - | killed before its first shard came in |

- **Steady state:** every run had 8 shards across 8 or more slots, so all first-round shards ran at once. The
  first-round rate is the steady state, and the second round (B2's two re-leased shards, 1.29 and 1.09) did not
  run faster. The laptop's gate and bundle were warm, so neither is in its shards' ms.
- **The farm's native shards run ~1.15 a core against the experiment's 0.97** on the same box and seeds. Six
  processes beat six threads in one process by ~18%. I did not dig into why.
- **B / A at equal main-box slots, from B4's rates:** A is 6 × 0.97 = 5.8 seeds a minute. B is 6 main slots plus the
  tab's 7 cores: 6 × 0.97 + 7 × 1.22 = 14.4, so **B / A = 0.40** in time. At the farm's own native rate it is
  6 × 1.15 + 8.5 = 15.4, or 0.38. Both assume the tab keeps 1.22 a core with all 7 cores busy. Only 2 were busy
  here, and a laptop's per-core rate usually drops when all its cores run. At B's 0.54 a core, B / A is 0.61.
- **B5 tested that assumption, and it fails.** B5 was 56 seeds (40072-40127, 14 shards), so the tab got 7 shards
  on all 7 cores, leased at +45 s. It returned **one**: 40108 at +344 s, 0.81 a core. The main box re-ran the other
  six once its own were done and won them all. The tab had not finished any of them when the main box's copy landed,
  between 469 s and 976 s in. That puts the tab's per-core rate on 7 busy cores **below 0.26-0.57**: 4 seeds over
  each shard's (landing − 45 s). On 2 cores it was 1.22, so the tab's 7 cores delivered less than its 2 did. B5 took
  976 s, about twice what the main box alone would take for 56 seeds (~480 s at B2's rate), because the shards came
  back to the main box serially at the tail. Its output still equals A's on A's 32 seeds and B4's on all 56. I can't
  see why the laptop slows: memory (seven wasm sims at once) or power or thermal throttling would each do it.
  With the tab at 7 cores, **B / A at equal slots is worse than 1** (B5). With the tab capped at 2 cores (B4's
  rate), it is 5.8 / (5.8 + 2.4) = **0.71**.
- **Measured: B4 / A = 283 / 342 = 0.83.** At 32 seeds the job is bound by its slowest shard (a 4-seed native shard
  is ~3.5 min), and 5 of the tab's 7 cores sat idle. B (565 s) ran on a loaded box. B2 and B3 (main only) took
  410-422 s, because 8 shards on 6 slots leave a second round of 2 with 4 slots idle.
- **Shards by worker:** B and B4: main 6 (native), laptop 2 (wasm). C: main 8, which includes the dead tab's 2. B2
  and B3: main 8.
- **Re-runs:** only at the tail, once the main box's own shards were done. B: shards 6 and 7 at 9:59:25pm and
  10:00:10pm (+357 s, +403 s); the laptop still won both, at +402 s and +521 s. B4: the same two at +179 s and
  +183 s; the laptop won at +184 s and +222 s. C: the dead tab's 2 shards, at 10:09:41pm and 10:10:01pm. B2 and B3:
  none.
- **C, the reclaim:** the tab was killed at 10:08:37pm (+118 s, about a third of A's wall), holding shards 40096 and
  40100. The main box leased them **64.1 s and 84.5 s after the kill**. The tab was stale after `fresh_seconds`
  (10 s); the rest was waiting for a main slot to free, since no idle worker was connected. C completed identical
  to A.

## The laptop tab's gate (message #7419)

From 10:05pm to 10:39pm the laptop tab did not pass a gate. It stayed fresh, but its gates for
`01692ec48502-8c979b7a` (handed out at 10:05:33pm) and `7e494f4cabf6` (10:25pm) stayed `pending`. It sat out C, B2
and B3. At 10:39pm it passed the next commit's gate (`b7fe5ff89bbd`) within seconds, now named "WOrk laptop" (97's
Name box), so it had most likely been reloaded onto 97's page. It then ran B4.

- **Not this repo's build.** `bundles/7e494f4cabf6/wasm.fingerprint` reads `aic-farm rules r69 content
  8bd963c4138f6cb0 games 2 lines fddac94fa5f04bdb`, the same as native. The board's key shows `bundle: ready`,
  `error: ''`.
- **It began at conductor 97's broker handover** (`broker.log`: `22:02:44 conductor's code changed: handing over to
  a new broker`). 97 changed the page and worker protocol: a core's `worker.js` now posts `need` and waits for the
  page to compile the module. A tab whose `index.html` predates 97 but creates a core with the new `worker.js` never
  answers `need`, so that core's gate hangs. The old page's reload ("finish running shards, then reload") then waits
  on that core. A headless Edge on the 97 page gated the same key in 4 s (10:05:37pm). This is my reading of the
  code; I could not see the laptop's own log.
- A stuck `pending` gate keeps the worker out for `_patience`, which is `lease_seconds` when that is set. Since 161's
  600 s, that is 10 minutes, where it used to be 1.

## Recommended `[remote]` values (not edited)

- `shard_seeds` 4 → **2**. A 4-seed shard is ~3.5 min, native or in the laptop tab (B4), and a job ends on its
  slowest shard. 2 halves that tail and gives a 32-seed job 16 shards over the 13 slots. The cost is ~30 ms a shard.
  1 would also do.
- `seed_seconds` 28 → **50**. Native now measures 51 s a seed on a core since 162's judge.
- `lease_seconds` **600, keep.** With per-shard renewal it is only a backstop, and a 2-seed wasm shard is ~4 min. Note
  that it also sets how long a stuck gate sits a worker out (above).
- `main_slots` **6, keep.**
- **Not a `[remote]` value, but the one that matters most:** set the laptop page's core slider to **2-3**, not 7.
  On 7 cores the tab returned 1 shard in 16 min (B5). On 2 it returned 2 in under 4 min (B4).
