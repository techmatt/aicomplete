# 285 bug hunt (r135 → r136): the rules since 261

A hunt on what changed since hunt 261 (r124, items 262-284): the card-driven adversary (Infiltrate and Grow
look-ahead, arrival corruption, Spill, Surge, botnet seeding), scarce draw and the chain-end draw engines, the Growth
and tiered-power boards, production off the income tracks, 282's facility and city changes, 283's agent values and
antimatter, and the player view where they surface. Start 1:51am, hunt-end target 4:51am (cap 5:51am).
[fixed.md](fixed.md) lists F379-F423 (and D102, D123), [deferred.md](deferred.md) D120-D134; [docs/qa-findings.md](../../../docs/qa-findings.md)
keeps the open list.

**Play changed (r136, one bump from the constant):** a cordon keeps out every agent and Grow, across a border too
(F379), cross-border ties break by country first (F380), Field Station isn't offered with no station site in reach
(F381), Overrun dealt onto a node hex corrupts only a city (F416). The golden game reaches none: its fingerprint held; the content hash moved (faces' words). Agents: one fix
screened and reverted (D120). No read was farmed; no fresh seeds spent.

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 1:51-1:57 | setup: release and test builds, viewer snapshot, the fast lane, e2e lanes, odd input, `cards:fit` | e2e, sources, researcher, odd green; e2e:corps red as on main (D102); `cards:check` crashed at its export (F400) |
| 1:53-2:10 | recent changes 262-284, code read (7 finders by area, each list verified) | 34 real of 34 (30 distinct): the cordon leak (a bump), cross-border ties, notes ahead of their acts, the adversary's turn unnarrated, ~15 stale rules lines, viewer labels, two dead tools |
| 1:57-2:26 | the sweep, 6 cases on r135 (c1-d4, c2-d4, c3-d0 engine, c2-d2 / c3-d3 / c4-d1 judge), 1,312 captures | green: only W16's `truncated` and a memory flag that was the 4-country case's size (F404) |
| 1:58-2:00 | as the user: a traced 3-country judge game's notes read in order | Infiltrate and Spill never named; a node loss logged twice (D125) |
| 2:13-2:23 | agents, faces by op, order inside a round, dead options, viewer paths (5 finders, verified) | 13 real: Field Station paid for nothing (a bump), faces missing "not yet the adversary's", the reference card's per-country numbers, Rule of Law's node clause, three viewer faults, the keep plan's board tags (D120) |
| 2:27-2:31 | contact sheets and 42 full-size shots at the new-rule moments (a reviewer) | counts consistent everywhere; one: Scrap II's free build still marked unaffordable (D123) |
| 2:29-2:31 | the keep-plan fix screened, 16 paired dev seeds x 4 cells | 42 → 40 wins of 64: reverted (D120) |
| 2:30-2:36 | this hunt's diff, independent review (3 reviewers, verified) | 9 real: the cordon's outbound half dropped from the words, F381 missing from the rules text, a write-up line, the sweep's new counter, doc headroom; all fixed |
| 2:34-2:56 | the sweep in the player view on r136 (c3-d2, c2-d4, c4-d3 engine, c1-d2 engine, c3-d1 judge), 1,084 captures; native = wasm (52 records, 12 variants) | green: only `truncated`; the new memory check 3.6 → 4.0 MB |
| 2:35-2:48 | as the user: two games in the player view through its controls (a helper) | 14: the turn panel still silent on the march, threats dealt, creations, no-op lines (fixed, F408); "purge:" read as a fact; draft layout, outlines, numberless "unaffordable" (D127, D129) |
| 2:39-2:47 | analysts' tooling, card faces by op, state across rounds, rules pages (4 finders, verified) | 10 real: three dead measures and a pooled arm in read scripts, payback's per-tag pricing, rules pages' troops and burns; state: nothing |
| 2:59-3:09 | this hunt's diff, second review; e2e:corps (D102) re-derived; per_decision old vs new | 4 small (Night Raid's note order, a silent Surge, an allocation, a Spill cancel tally); D102 fixed (no engine bug); same games, 1.07× greedy, 1.03× strategist a decision (a sweep beside it) |
| 3:01-3:33 | paths not yet driven: the sweep on variants (threats, alignment, cards off; no hold-off; overlay; cubes 3; c4-d0), 7 cases; the CLI commands; odd input on this hunt's fixes; the farm bundle | green throughout; `npm run odd` 18 scenarios (3 new); farm native = wasm on 24 games |
| 3:07-3:20 | the rulebook read whole against the code (3 finders by section, verified) | 9 real of 10, all words (F415) |
| 3:20-3:33 | Unity / Discord and threat faces, the generators' own text, a review of round 4 (4 finders, verified) | 12 real: Overrun on a node hex (a play fix, F416), tier-1 threats taking their site and a name clash (Matt's calls, D130, D131), ten word fixes (F417) |
| 3:39-3:58 | a second player-view session (two games to the end, every note checked against the map); the third diff review | agents' notes all match; 11 + 6 real: troops, tokens off, no-op lines, plurals (fixed, F418); notes below the fold, strikes, a plural prompt (D133); Offensive's other threats on a node hex (D132) |
| 3:39-4:00 | the sweep on judge and engine cases (c2-d1, c4-d2 engine; c2-d3, c3-d4 judge), 951 captures | green but c3-d4's watch: its records were made by a binary I rebuilt mid-sweep (the error bar's content hash); rerun alone on the final build; a two-digit count over a hex number (D75) |
| 3:56-3:59 | final lanes on the last build; `probe`, `panes`, `compare` | e2e, odd (18), sources, researcher and corps green (corps for the first time since 235); the paths work |
| 3:58-4:03 | the card viewer and its server, docs/scripts.md's flags (2 finders, verified); the fourth diff review | 3 real (the reference card's doubled "a country" from F395, a note's text on sectioned cards, a second name clash: D131); scripts' flags: nothing; 8 small in the last fixes, all fixed |
| 4:07-4:12 | code reading: 280's production from the tracks, 278-281's flags and leftovers, the agents' forecasts (3 finders, verified) | production: nothing; flags: nothing; forecasts: no Spill forecast (D134, an agents heuristic) |
| 4:12-4:17 | the standing `smoke` experiment spec; the counter-name check on r136 games | green; only legacy fallbacks unwritten |
| 4:17-4:24 | words against code: architecture, README, simulator, experiments, reads; card-grammar, player-cards, board (3 finders, verified) | engine and sim docs: nothing; 4 stale lines in the rules pages (F420) |
| 4:21-4:26 | words: war, economy, ambassadors; principles, card-guidelines, card-pricing; agents, speed (3 finders, verified) | rules pages: nothing; 5 stale numbers in the design and speed pages (F421; one left: it can't be reproduced) |
| 4:26-4:29 | words: web and card-art READMEs; rulebook friction and the aid's source (2 finders, verified) | READMEs and aid: nothing; 2 small in the friction page (F422) |
| 4:27-4:35 | code reading: the viewer's board, panels and timeline modules (2 finders, verified) | 3 real, small, fixed (F423) |
| 4:35-4:40 | the final green: fast lane, e2e, odd on the last build | green |

The hunt ran to about 4:40am and closed at the 4:51 target with the last green. Every lens had at least one real
round; the last passes were thin (code reading: 1 agents heuristic, then 3 small viewer faults; words: 4, 5, 2 small
lines; paths, odd input, long runs and the checker green), so the lenses were drying, not dry.

**Seeds and reads.** No read was farmed, no fresh seed spent: the agent smoke and per_decision ran on dev seeds 0-15
and 1000-1015, the sweeps on 2850-2892. 283's kept r135 games file fed the counter-name check and the script tests.

## Checker

`web/scripts/sweep.mjs` (web/README.md, *The sweep*) gains `COUNTER_EVENTS_R131` (guards holding, a blocked or
build Infiltrate, Wall stopping a landing, research troops, a skipped spread, capstones, a card drawn, a cordon) and
two judge cases (`c3-d3-strategist`, `c4-d1-strategist`); its memory check now replays the first fresh game at the end
(F404). The engine now writes a note for every Infiltrate, Surge, Spill and seed, so the sweep's moments and the
player view's turn panel show them.
