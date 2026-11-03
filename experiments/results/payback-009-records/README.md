# 009 records: games where the engine decides the outcome

**The records themselves were deleted in 038:** none replayed on current rules. This table
is what they were. Git history has them (last at `ce0ed59`).

All at the tuned baseline (rules r3), 2 countries. Why each is worth a look is in
[`../payback-009/payback-009.md`](../payback-009/payback-009.md#records-worth-a-look).

None of these replay today: rules r4 and later changed the game (r6 is the 014 board).
The last column is what held on r3.

| File | Build | Replayed on r3's main? |
|---|---|---|
| `base-d4-engine-seed300094.json` | main | yes |
| `base-d4-greedy-seed300094.json` | main | yes |
| `base-d4-engine-seed300041.json` | main | yes |
| `base-d0-engine-seed300008.json` | main | yes |
| `deck446-d0-engine-seed300080.json` | main (config `adversary.stage_cards=4,4,6`, in the record) | yes |
| `PATCHED-insight_inc-d4-engine-seed300041.json` | **patched content** `insight_inc` | **no**: on main's binary `aic-sim replay` fails partway, because an action isn't legal under the unpatched content (e.g. "can't play fab_line: unaffordable"). Rebuild with `bash scripts/payback/build_patches.sh insight_inc`, then `out/patch-bin/insight_inc.exe replay <file>`. The viewer needs a wasm built from the patched content. |
| `PATCHED-econ_half+insight_inc-d4-engine-seed300038.json` | **patched content** `econ_half+insight_inc` | **no**: as above, with `build_patches.sh econ_half+insight_inc`. |
