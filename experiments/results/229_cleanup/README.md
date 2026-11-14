# 229 cleanup (r100, no play change)

The audit notes of 222-228 and Matt's extras of 2026-11-13, carried out on 228's main (r100). No rules revision:
nothing removed was reachable at the baseline, and the per-tag income change moves only agents that project hand
tags.

## Matt's extras

- **Harden for chips is gone:** `Op::HardenAction`, `StandardAction::HardenChips`, its view `seat_chips`, its
  `values.json` row, design-check row, `NOT_TUNABLES` entry and test. No card granted it since r42.
- **Dead params:** threats variant `shared_row` (and `configs/shared_row.json`, `scripts/configs/shared_row.py`, its
  variant-matrix rows), `adversary.extra_threats` with its step `extra_threats_at` (5: never a level; the pair only
  existed together), `adversary.wall_troops`, `wall_landing` and `adversary::wall_pressure`.
- **Per-tag income:** `cards.tag_income_once` is gone; a building's per-tag income always freezes at play. In
  `agents/value.rs` an `IncomePerTag` is now `Worth::income_frozen_tag`, read at `Ctx::play_tags` (the seat's tags in
  play), not at `Ctx::tags`, which `w.tag_hand` projects with the hand's tags. Engine params live on `VariantDef`
  only, so no module-level param was needed (none is left); architecture.md says the next such rule adds one.
- **Retired probes:** `plan_hand`, `old_counters`, `always_liberate`, `strat_power_uses` (always 1), `plan_chain`,
  `engine_rush`, in `policy::RETIRED`; a spec naming one is refused with the reason.
- **Overlay `+`:** a last path step `+` appends to a list, in the engine's `--set` (two appends both stand) and in
  `apply_overlay.py` (which skips an equal value already there, so a rerun doesn't append twice).

## The per-tag income read (f229, fresh seeds 46976-47007)

`experiments/farm/f229.json`: greedy and the strategist, three board pairs, d0-d3, 32 seeds a cell, 768 games a side,
both native, base 1aa6b5e. `bench/same_results.py read_base read_new --ignore-config-hash`: **same: 768 games.** The
shift is exactly zero: flat agents (`w.table` 0, `w.tag_hand` 0) read `tags` = `play_tags`. The agent it moves is
`engine` (`w.tag_hand` 0.5) and the planner above it: on per_decision's 16 games at d2, engine won 5 (6 before),
9.44 rounds (9.69).

The new side ran the rebuilt binary (Matt, #10233): `new.exe` (md5 ad7019…) was copied at 12:08am from the build
holding the value.rs change; the base `base.exe` (a8e17a…) is 1aa6b5e's, built before any edit; the base side ran
12:06-12:10am, the new side from 12:10am. `new.exe` refuses a retired probe (`greedy:plan_chain=1`), base doesn't.

**CopyIncome** (`cards::copy_income`, #10233) read the seat's live tag count for a copied per-tag income; it now
reads the count frozen at that building's play, as the building pays. No card prints `copy_income`, so no game moves.

## Speed (quiet box, 12:18am)

`per_decision.py OLD NEW --games 16` (c2 d2, seeds 1000+, best of 2), ms a decision new / old: greedy 0.98, strategist
0.98 (the same games), engine 0.99 (its games moved). The `speed.py` baseline (r96, 215) was re-based at r100 here.
