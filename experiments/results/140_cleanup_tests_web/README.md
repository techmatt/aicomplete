# 140: cleanup, a faster fast lane, the web checks (r58, no rules change)

`RULES_REVISION` stays 58: nothing here changes play. The content hash moved (`978ed08fd7235651`, was
`506adf78bab370df`) because `content/map.json` lost four empty `"after": []` lines; the golden fingerprint did not.

## Where things stood

139 ended `done` at r58 (main 01fdfe4). It had already removed the clock (`board.clock`, `clock_step`, `Op::War`,
`war_twist`, the war-round view globals) and `board.blocker_stays` with their tests, so 140 found only stale comments
of those (`board/mod.rs`, `production.rs`, a factories test still named for "production steps only"). One live leftover:
`scripts/wasm-parity.mjs` still played `board.clock=3`, so native = wasm failed at its first variant from 139 on.

## Same play

`scripts/bench/same_play.py` on main 01fdfe4's release binary (`out/140/base-sim.exe`) and the final one: 63 summaries, 63 traces, 549
records, 0 differences (every base record replays on the final binary). A run took 20-23 minutes on 6 threads. The matrix now also plays every variant of `configs/variants.json` (13, greedy);
the compare reads the base run's 63 summaries, so the 9 new variant rows play on the new binary only.

## Retired code and audit notes

| Note (item) | Done |
|---|---|
| Clock, `Op::War`, `war_twist`, `blocker_stays` (137-138) | 139 removed them; 140 fixed the stale comments and wasm-parity's `board.clock=3` |
| `corrupt_city`'s `min_ambassadors` / `else_ambassadors` (138) | Removed: no content prints them (op.rs, `new_ops.rs`, `effects/mod.rs`, `docs/ops.md`) |
| `corrupt_city`'s no-location form, `most_infiltrated_city` (138) | **Kept**: Infiltration Drive (`content/projects.json`) runs a bare `corrupt_city` before its printed `corrupt site=city cut=1`; see below |
| Megaproject `after` field and its doc column (137) | Removed: `content.rs` (field, check, overlay example), `docs/blocks.rs` (corps column), `docs/harms.rs`, `docs/run.rs`, `map.json`, `docs/rules/board.md`, `docs/experiments.md` |
| Shadow Factories' fallback placement (138) | **Left**, note below |
| Purge level as an enum (136) | `adversary::PurgeLevel` (`All`, `NoAmbassadors`, `NoTroops`, `CitiesOnly`) with `counts_*`, `win_text`, `steps`; `alignment::bar_level` returns it; `board/mod.rs`, `eval.rs`, `experiment.rs` read it |
| `difficulty_block` hardcoding levels (138-139) | 139 placed the extra threats and corrupt cut by their dials; 140 adds `board_difficulty_at` (the board's line now shows at level 3 in `docs/rules/adversary.md`). `stage1_removed` (2), `build_bonus` / `stage2_removed` (3) and difficulty 4's rows are fixed levels in code, not dials, so the table states them as is |
| `regen.py` rewriting the rules README revision (138) | Done: it sets "at rules revision N" from `aic-sim show` |
| `death_audit.py --root` (137-138) | Done: `--root DIR` runs the binary from DIR (a folder holding its revision's `configs/`), so the exe needn't sit beside a copied `configs/` |
| Variant lists in one place (136) | `configs/variants.json`, read by the core's `variant_matrix` (`include_str!`), `wasm-parity.mjs` and `same_play.py` |
| One spread-piece helper (137) | Nothing to do: 138 removed the spread; `arrive` no longer forms botnets and no "spread piece" filter remains |
| Per-target price hook, per-op registry | Notes with sketches below (not attempted) |

**Infiltration Drive** prints "Corrupt cities (1 fewer)." but its effects are `corrupt_city` (the most-infiltrated
clean city in each country is corrupted) and then `corrupt site=city cut=1`. The first line is not on the card; it
looks like 138's conversion kept the old op beside the new one. Removing it changes play, so 140 leaves it (asked
Matt, design question 1, default taken). A rules item that drops it can then delete the `at: None` arm in
`effects/mod.rs` and `adversary::most_infiltrated_city`'s use there (`new_ops.rs` still uses it when a threat has no
city token).

**Shadow Factories** (`adversary_factory` with `fallback_ambassadors` 2) places its 2 ambassadors only when no
factory zone is free, on a factory the players choose. A creation line is unconditional and lands by the lowest-number
rule, so it can't express "only if the build failed", and the players' choice would become a fixed hex: both change
play. Making it a creation line needs a condition on creation lines (`if: no_free_zone`) and a ruling on where it lands.

**Per-target price hook (sketch).** `printed_price(w, act) -> Price` is per action, so Claim and Liberate print a zero
price, filter targets by affordability in `std_targets` and pay inside `std_resolve` (`board/standard.rs`). Replace it
with `price(w, g, seat, act, hex) -> Price`: Claim and Liberate return `claim_<site>` (or `liberate_at`) plus the
hex's adversary ambassadors; every other action ignores `hex`. `std_targets` then keeps a hex if `price(..).affordable`,
`std_resolve` pays `price(..)` before the effect, and the view prints the price per target from the same function.

**Per-op registry (sketch).** Retiring one op touches op.rs, its slot table, two dispatches and the "read, not
resolved" list, `op_name`, `face.rs`'s not-printed list, `agents/value.rs`, `values.json` and `docs/harms.rs`. One
`const OPS: &[OpInfo]` in op.rs (`name`, `slots`, `printed: bool`, `value_row`, `harms`), keyed by a
`fn kind(&self) -> OpKind` fieldless twin of `Op` that `strum`-style derives or a macro writes, would hold every
per-op fact but the resolve arm; a fast-lane test checks that each `OpKind` has one row and each `value_row` exists.

## Tests

Fast lane by test binary, each binary's wall time, same load (a same-play run beside it, 6 test threads), best of
two runs (`out/140/lane_time.py`):

| Binary | Before (s) | After (s) | What changed |
|---|---|---|---|
| aic-core unit | 2.25 | 0.98 | `the_view_follows_every_variant...` runs its 13 variants on scoped threads (1.75 s to 0.5 s) |
| aic-sim unit | 3.57 | 1.77 | the dominance checks (21 agent specs, 3.3 s) split in five tests; the agent openings (1.9 s) one test an agent |
| experiment | 3.85 | 1.58 | `experiment_reports_comparisons` removes the cards module instead of threats (a debug greedy game with threats off runs ~2 s: 3.1 s to 1.5 s with the other tests beside it); the panicking-gate test plays 2 games a cell, not 4 (3.6 s to 1.3 s) |
| others (5) | 0.45 | 0.45 | |
| **Sum** | **10.12** | **4.79** | -53% |

Coverage kept: every agent spec, variant and assertion still runs; the gate test still gates (one check at 2 games),
panics, resumes and lists the game once. Merged: `the_unity_deck_deals_every_printed_card` (alignment.rs) into
`a_game_deals_every_printed_unity_card` (endgame.rs), which already counted each tier; the merged test checks
`cards_left` too and drops the pinned 9. Renamed: `player_factories_make_chips_every_round` (it read "on production
steps only" after 139 cut the clock). 136's stale-override failure: `baseline_with`, `lab_config`, `lab`,
`lab_with`, `lab_actions(_with)`, `threat_lab(_with)` and `action_only` are `#[track_caller]`; an override that
`apply_override` refuses panics with `override "x.y=1" (passed from tests/foo.rs:LINE)`, and one `Rules::new` refuses
panics with its "x.y: not a parameter" message, the helper whose config it was (`lab_config`), the test's own
overrides and the calling test's file and line.

## Web

`npm ci`, `npm run wasm`, `npm run check` (clean) and `npm run cards:fit` (runs; 9 cards' top box below 7.5 pt, as
139 left them) pass. `node scripts/wasm-parity.mjs 2`: 56 records over 13 variants, native = wasm. e2e on the dev
server (port 50817):

- `e2e:corps` (10 fails) and `e2e:researcher` (5): both moved an ambassador onto a neutral station to claim it,
  which 137 replaced with the Claim action (3 ambassadors from the supply at r58). They now Claim with
  `claim_station` 1, and set `supply_per_country` 0: a creation line otherwise lands an adversary ambassador on the
  claimed station, which blocks its cubes. `e2e:corps` also expected 3 adversary cubes on Horizon's card, gone since
  137 (adversary stations make compute): `station_cubes` 5 (its maximum) now overflows the 8-cube card in round 2
  from the claimed station alone, and the overflow check reads any count over 8. Its "once a round" check passed only
  because the energy ran out; Horizon's converter is "as often as you can pay" (105), so it now checks the menu offers
  it again exactly while a cube and 2 energy remain.
- `e2e:sources` (1): the forged payment is refused by 138's "only a player near the threat can pay" before the
  source check; either refusal passes.
- `e2e`: every check passes but `board fills its pane at 1920x1080` (453 px, floor 464), which 139 measured as the
  r58 rules moving the fixed game (its countries strip grew), not the layout; the floor is Matt's, left as is.
