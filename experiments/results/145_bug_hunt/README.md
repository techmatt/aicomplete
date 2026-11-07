# 145 bug hunt (r59 → r60)

A 3-hour hunt (Matt, amendment 6121) focused on what changed since the 118 hunt: 122's caps (gone at r51), 130's
energy sinks, 131's agents table, 134-135's adversary drafts and model, 136's alignment and Envoy, 137's sites and
war, 138's corruption and arrivals, 139's Discord cards, 141's speed work, 142's threat intensity, and 144's cleanup
(landed while this item waited). [fixed.md](fixed.md) lists F165-F204 (F194, F195 and F200 were taken back: D70, D71);
[deferred.md](deferred.md) D57-D71; [docs/qa-findings.md](../../../docs/qa-findings.md) keeps the open list.

## 142's skipped checks, on r59 (amendment 6121)

- `npm run cards:fit`: the same nine top boxes below 7.5 pt as at 140 (none from 142). A face 145 tried for the
  claim-station cards set Microgrid Lab Compact at 5.8 pt; taken back (D70).
- `play_read.py` (16-seed engine smoke, `experiments/use.json`): kept and never played: Exascale Training Cluster
  (engine 47.9%, strategist 43.8% of games), Cryo-Compute Burn (engine 14.6%), Great Liberation (strategist 20.8%),
  Telemetry Archive, Security Ops Floor, Synthetic Data Forge, Homeland Security Act, Checkpoint Network; never kept:
  Baseload Contract, Capacitor Discharge, Dispatch Control Room, Tidal Barrage (engine), University Spinout
  (strategist). D57.
- `speed.py check`: no verdict (baseline r58). `per_decision.py` r58 → r59 (d2, 16 games, one thread): greedy 1.63×
  a decision (options 1.13×, 2.17× a game), engine 0.58× (D58).
- `bench/same_results.py`: 142's build (4d03477) against main (4f3febc, 144's cleanup) on the smoke: the same 96
  games; the script's one "difference" was 144's reworded spec note (F174).

## Reads of the hunt's own changes (16-seed smoke, d2, 96 games a build)

| step | against | games | pooled win % |
|---|---|---|---|
| F165-F167 (rules, round 1) | main | all 96 the same | 62.5 |
| F168-F169 (agents, round 1) | round-1 rules | differ | 62.5 → 66.7 |
| F195-F196, F198 (agents, round 4) | commit f858701 | differ | 65.6 → 65.6 |
| F199-F201 (round 5) | commit d0a80e4 | differ | 65.6 → 67.7 |

Smokes, not reads: each step's cells are 16 games (SE about 12 points). The round-2 rules fixes (F180-F183) were
built beside F189's agent change, so no row isolates them; each touches a rare state (a full base at a tip, a tip in
the Sway phase, a project filled in the build, a coastal token squeezed out).

The round-4 and round-5 rows include the shelter-armor rewrite (F195, F200) that was then taken back (D71), so they
read a build that didn't land.

**Speed** (`per_decision.py`, engine, d2, 16 games, quiet box): r60 against r59 1.24× a decision after rounds 1-4,
1.13× after F198 (one menu read). The first `speed.py --update` read the greedy at 11.7 s a game (r59: 1.2 s):
`per_decision` put it at 10.5× a decision. A bisect over the kept builds (greedy, d2, 3 games from 1000) read main
2.4 s, round 1 3.7 s, round 3 3.4 s, round 4 49 s, round 5 74 s; round 4's guide on round 3's took it back to 2.9 s,
and the old `absorbed` alone did not. The armor rewrite was taken back (D71); the final build reads 3.05 s, and the
baseline was moved to r60 on it.

## Checker

The sweep (`web/scripts/sweep.mjs`) at r59 on fresh seeds 14500 on, all 23 cases at one seed: 2,941 captures in 68
minutes, no step over 150 ms, heap 7-34 MB across 22 game pairs (no growth). Hard fails: land corruption had no
glyph (F171). Looks: `raw-id` withdraw buttons (F193), `board-text-overlap` on a 95-troop hex (W26), `truncated`
(W16). On r60 (seeds 14600 on, the three d3 cases): 424 captures, exit 0. `npm run odd` 12/12; `npm run e2e`,
`e2e:sources`, `e2e:corps` pass at r60, `e2e:researcher` after F202; `wasm-parity.mjs` 84 records over 13 variants,
native = wasm.
