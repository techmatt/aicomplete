# 251: profiling the sim (judges, oracles, the engine)

Rules revision r112 (no rules change; no bump). Base: main at 3d78441 (after 249). Every timing here was taken on a
loaded box (Matt's messages 11448 and 11468: 0-0.2 hardware threads free): read ratios only. Ratios are paired:
base and new binaries on the same games, in turn, by process CPU time (`cpu_pair.py`), which holds better than wall
time under load.

## The arms

- **greedy** and **the judge** (`strategist`), no oracle.
- **The real oracle** (249, `oracle:worlds=4,alts=3,budget=32,answers=1,truth=1` on the judge's weights as
  `r249-arms` ships it; amendment 11454: this is "the real oracle"), d0-d4 split, d2 timed and checked.
- **The full oracle** (the ladder's `oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3`; amendment
  11454), d2.
- **p248j** (the strategist on 57 standard-action variants), d2 checked; one seed of d0-d3 profiled.

## Where the time goes (before, then after)

The profile build's split (`--features profile`; its `clone` is a throwaway copy beside each apply, so clones and
drops read high), share of agent time, and allocations a game. Call counts are identical before and after in every
arm: the same games.

| arm | games | eval | rules apply | clone | legal | policy | oracle's own (determinize, sync, top) | allocations a game | MB a game |
|---|---|---|---|---|---|---|---|---|---|
| greedy + judge, before | 48 (2 seeds, 3 pairs, d0-d3) | 50.4% (27.9 µs) | 33.2% (15.4 µs) | 4.1% | 0.1% | 4.6% | 0.4% | 5.88 M | 2,627 |
| greedy + judge, after | same | 50.2% | 34.3% | 4.0% | 0.1% | 4.5% | 0.3% | 5.67 M | 2,512 |
| p248j, before | 228 (1 seed, d0-d3) | 51.2% (43.7 µs) | 34.8% | 4.0% | 0.1% | 2.5% | 0.4% | 4.59 M | 1,865 |
| p248j, after | same | 49.7% | 35.3% | 4.3% | 0.1% | 2.8% | 0.5% | 4.44 M | 1,790 |
| real oracle d0-d4, before | 5 (1 seed a level) | 44.4% (30.8 µs) | 33.6% (+0.9% in place) | 4.0% | 0.1% | 7.8% | under 0.2% | 417 M | 175,834 |
| real oracle d0-d4, after | same | 43.6% | 33.4% (+1.0%) | 4.2% | 0.1% | 8.1% | under 0.2% | 402 M | 170,520 |
| full oracle d2, before | 4 (bp) | 47.1% (33.0 µs) | 22.9% (+0.8%) | 2.8% | 0.1% | 13.4% | 1.6% | 4.93 M | 1,946 |
| full oracle d2, after | same | 47.4% | 23.5% (+0.8%) | 2.9% | 0.1% | 12.7% | 1.4% | 4.91 M | 1,908 |

The oracles' `line` part holds their rollouts whole (99.5% of the real oracle's time, 91% of the full oracle's at d2),
and the eval, apply and policy shares inside it again. The judge's own split (greedy and the judge ran in one process)
is greedy's and the judge's together; each plays the same search.

Outside the agents (the `time:` line): legal-move generation 1.6% of a game, the game's own applies 0.7%, the replay
check 0.5%, setup 0.3-0.5%. **Output** (`stream:`, new): turning games into lines and writing them took 0.02 s for 48
judge games and 0.07 s for 228 p248j games, under 0.02% of the run; it costs bytes, not time.

**The real oracle is the judge's games over again.** Its lines (each a judge game to the end) are 99.5% of its time,
820 ms a line; its sampling, voting and candidate pick are under 0.2%. At 1 seed a level (one thread, a loaded box)
a game took 279 / 141 / 173 / 112 / 199 s at d0-d4. So the judge's costs are the oracle's, in proportion, and the
oracle's own dials (`worlds`, `budget`) scale it linearly (249: twice both, about twice the time).

**Hot spots** (sampled, `profile_win.py`, the judge, 5 games from 36802; self time, before):

1. The binary search by name, 15.7%: `GameState::var` 2.0%, `tag_counts` 1.5%, `pile` 1.4%, the node tracks
   (`track_taken`, `track2_taken`) 2.6%, `near_mask`, `has_disabled`, `disabled`, `alignment::after`.
2. `Eval::value` 6.6% self, 49.8% total: the board read (`TokenKeys::read`, the HexTokens fold, 1.8%), per-call vectors
   and folds (`from_iter` 6.3% total), `tag_counts` 3.4% (249's `card_terms`, every seat with a building, every
   position), `node_count` 1.4% (the same), `need_and_share_with` 3%, `next_rally` 2.4%, `next_corrupt_n` 2.1%.
3. memcpy (vcruntime) 5.9%, drops 4.8%, `GameState::clone` 4.3%, mimalloc and `__rust_alloc` 6.2%.
4. In the rules: `effects::one`, `enter_play`, `cards::triggers::fighting` (4.6% total), `std_targets` 3.2%,
   `place_node` 2.7%, `Cx::tally` 2.6%, `sacrifice_keeping` 2.5%; `format_inner` 2.9% total (labels).

**Output size.** A game line is 82 KB (768 judge games; 228 p248j games alike): `counters` 42%, `actions` 34%,
`cards` 15%, `uses` 5%, `sources` 3% (`line_bytes.py`). p248j's 33-38 MB 2-seed shards are 456 such lines.

## What changed (same play)

- **The board read is shared by the hexes' `Arc`** (`TokenKeys::read_shared`, `eval.rs`): a one-entry thread store
  keyed by the table of hexes, which holds that `Arc`, so a held table is never written in place and its address
  can't come back. A decision's options that move no token share one read.
- **249's `card_terms`** (the judge's building earnings): a seat's node count comes off that board read, not a scan
  by name; whether a city is corrupted is read once a position, not once a seat; tags are read only when a term
  needs them. Routing its tags through the seat-terms store was tried and gained nothing: a miss (every option that
  plays a card) prices the hand too.
- `flat_value` takes track income through `ai::each_track_income` (no vector), and `plan::next_corrupt_n` takes the
  minimum without collecting (five sites a position).
- **Opt-in lean lines:** a spec's `"drop": ["actions", "cards"]` writes those keys empty in `games.jsonl` and farm
  lines (about half the bytes); the run's own report, `results.json` and `cards.csv` still read its games whole, but
  such a run can't `--resume` (refused: a resume replays saved actions) and its lines can't be replayed. Default
  output is byte for byte as before (the identity check below compares whole lines). An unknown key is refused.
- `experiment` prints `stream:` (time and bytes writing lines) after `time:`.

## Identity: base against new, whole lines

`id_check.py`: a pinned fingerprint (sha-256 over every game's seed, cell, won, reason, rounds, decisions) on each
side, and whole game lines compared with the engine field and timings set aside. Dev seeds 36800-36831 (32 a cell);
base farmed or played natively from 3d78441's code, new farmed from bc71095.

| arm (spec) | games base / new | seeds | fingerprint base | fingerprint new | lines that differ |
|---|---|---|---|---|---|
| greedy and the judge, three pairs, d0-d3 (`i251-judges`) | 768 / 768 | 32 | 89012788751c514f | 89012788751c514f | 0 |
| the real oracle, bp, d2 (`i251-real`) | 32 / 32 | 32 | d22b7643a1ee9614 | d22b7643a1ee9614 | 0 |
| the full oracle, bp, d2 (`i251-full`) | 32 / 32 | 32 | 855525a71091fcea | 855525a71091fcea | 0 |
| p248j, all 57 variants, d2 (`i251-p248j-d2`) | 1,824 / 1,824 | 32 | 4604c089dc2fc964 | 4604c089dc2fc964 | 0 |
| p248j, six variants, d0-d3 (`i251-p248j`) | 768 / 768 | 32 | 2ea5bb321d31e65f | 2ea5bb321d31e65f | 0 |

Base sides: the judges and the real oracle farmed from 591ed64 (3d78441's code; native and wasm shards mixed), the
rest played natively by the base binary (`shards.py`); new sides farmed from bc71095 (the real oracle, judges and the
six-variant p248j mixed native and wasm). The final commit's change after bc71095 (refusing `--resume` with `drop`)
touches no play.

## Time: same games, base against new (CPU time, in turn)

`cpu_pairs2.sh` (`cpu_pair.py`), after the farms (the box quieter; still shared): process CPU seconds, base and new
in turn on the same games, one thread.

| arm | games a pass | passes | base s (median) | new s (median) | new / base (median) | passes' spread |
|---|---|---|---|---|---|---|
| the judge, bp, d2 | 6 (36800-36805) | 6 | 7.63 | 7.34 | **0.962** | 0.83-1.00 |
| greedy, bp, d2 | 6 | 6 | 8.24 | 8.16 | 0.990 | 0.98-1.02 |
| p248j, all 57 variants, d2 | 57 (seed 36800) | 4 | 50.30 | 49.42 | **0.982** | 0.981-0.987 |
| the real oracle, bp, d2 | 1 (seed 36801) | 2 | 178.4 | 176.2 | **0.988** | 0.983-0.993 |
| the full oracle, bp, d2 | 4 | 2 | 9.29 | 8.81 | 0.949 | 0.91-0.99 (first read, loaded) |

**Per-game time is lower, by little:** p248j 1.8%, the real oracle at d2 1.2%, the judge about 4%, greedy 1%. The
first read, on a box with no thread free (messages 11448, 11468), swung 0.83-1.26 a pass; only the second is quoted.
What moved is in the profile too: allocations 3-4% fewer a game in every arm (same call counts), the eval's time
against the rules' 4% lower on the judge and p248j. The fixes reach the heuristic's board reads only; the costs that
would move these numbers by tens of percent are the list below.

## Report-only levers

None measured: every lever that changes play here is a dial of the arm's spec (the real oracle's `worlds` and
`budget`, linear in time: 249's 2x read; the full oracle's `budget`, `reseeds`, `tries`), Matt's to set, or one
speed.md already lists as measured and refused (`answers=1`, `discards` 1).

## What is left (too big for this item)

1. **Typed slots for the hottest vars** (node tracks, stage, buckets, paid vars), beside `vars` and out of serde and
   `PartialEq`: the binary search by name is 16% self; slots would take most of it. About 8-10%.
2. **Per-position reads off the shared board read**: `reach_masks` and `Sites::read` could share the same `Arc` key;
   a tags-only store by tableau for `tag_counts` (3.7%); `next_rally` and `next_corrupt_n` read the same card five
   and more times a position. About 5-10%.
3. **Resuming a prompt's phase instead of replaying it** for each option: `Rules::apply_to` is a third of the time.
   A redesign of how prompts resolve; large, unestimated.
4. **Allocations** (6 M a judge game, 417 M a real-oracle game): per-call vectors in the heuristic and the rules'
   target lists (`std_targets`, `feasible_aimed`, `aim`, `node_sites`) into reused buffers. About 5%.
5. **Lean output by default** for farmed screens whose readers need no actions: halves a big spec's bytes (a
   ruling, staged question 1: kept opt-in).

## Files

`prof_runs.sh`, `prof_more.sh` (profile-feature runs), `shards.py` (a farm spec natively on a fixed binary),
`cpu_pair.py`, `cpu_pairs.sh` (paired CPU time), `pairs.sh` (per_decision.py pairs), `id_check.py`, `line_bytes.py`,
`mkspec.py` (the identity specs `experiments/farm/i251-*.json`).
