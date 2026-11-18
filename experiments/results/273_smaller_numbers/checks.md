# 273's checks (r128 as shipped: prices to the nearest)

- **Give / get:** no card, Action or conversion was repriced against another; every one moved by the same ÷ 1.5 as
  values.json and the anchors, so each keeps r127's ratio up to its rounding (half an r127 credit either way).
- **`npm run cards:fit`** (web/): exit 0; 208 player cards, none under 7.5 pt; titles under 15 pt 11 (smallest 13.5,
  Counterinsurgency Campaign).
- **`play_read.py`** on the fresh read, judge: no card kept in over 1% of games and never played; none never kept
  (r127 the same).
- **`loop_smoke.py --run`** (16 seeds, 96 games): no timeouts, no card past the play flag.
- **`speed.py check`**: no verdict (its baseline is r112). **`per_decision.py`** r127 against r128 (the first build;
  the agents are unchanged since; c2 d2, 6 games, one thread, best of 2, a quiet box): strategist 1.2 → 1.3 ms a
  decision (1.05×), greedy 1.2 → 1.0 (0.84×). No slowdown.
- **`same_results.py` / `farm/compare.py`:** not needed: no rules file changed after the shipped build was read.
- **design_check** on the fresh read with `--old` r127 (same seeds): README, *Checks*.
- **Fast lane, full lane (`cargo test-all`), clippy, `aic-sim docs --check`:** clean.
