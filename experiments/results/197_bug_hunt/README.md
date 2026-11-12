# 197 bug hunt (r86)

A hunt on 195's seven project ops (r85) and 196's balance pass (r86: the card-play track at 3, the Focus cut,
`stage_extra`). Start 5:43am, hunt-end 6:13am (cap 7:13am). [fixed.md](fixed.md) lists F272-F288,
[deferred.md](deferred.md) D88-D91 and W29-W30; [docs/qa-findings.md](../../../docs/qa-findings.md) keeps the
open list. No play changed: the golden game's fingerprint held, the content hash moved for the faces.

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 5:43-6:00 | recent changes: code read of r85-r86 (5 finders, each verified by a skeptic: 13 claims real of 15, 2 refuted); readers that tally powers by index | 12 fixed, 2 deferred |
| 5:47-6:15 | the sweep: `c2-d4`, `c3-d3-engine`, the new `c3-d4-strategist`, 2 seeds a mode (seeds 100-101, 1000-1001); 1,046 captures, 0 steps over 150 ms, heap 15-35 MB | F284 (7,316 `token-unknown`), D91; the r85 moments all fired (factory troops, the clock, sites taken, raids, botnets) |
| 6:10-6:22 | the hunt's own fixes (independent review), rulebook against r86, agents on the new ops (3 finders, 3 verifiers) | F285-F288, D90; agents: nothing |
| 6:15-6:20 | re-sweep `c3-d4-strategist` on the fixed build (seeds 100, 1000) | clean (exit 0) |
