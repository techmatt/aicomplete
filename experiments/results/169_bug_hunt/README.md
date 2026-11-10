# 169 bug hunt (r72 → r73)

A hunt on what changed since 162, in the planner's order: r70's Gather, Push and standing liberations; r71's Unity
prohibitions and fear-card decks; the read tools since 162; the sim players' move heuristics. Cut short at Matt's
message 7804 (10:38am): fix what was found, no new rounds. [fixed.md](fixed.md) lists F205-F224,
[deferred.md](deferred.md) D72-D76; [docs/qa-findings.md](../../../docs/qa-findings.md) keeps the open list.

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 9:35-9:50 | code read, 5 finders + verifiers: r70 moves, r71 fear cards, words, read tools, agent moves | 20 real (Wall, Incursion, house words, ladder, endgame probe, 2 agent misreads) |
| 9:40-10:25 | the sweep, 5 cases at 2 seeds (2 new d1 strategist cases), seeds 41432-41433 | firewall drawn with no glyph (91 hard fails), board-text overlaps in agents' games (D75) |
| 9:45-10:25 | engine smoke `use.json` 16 seeds (41432-41447): loops, play_read, board_read --powers, runaway_read | no loop; the strategist keeps 38 cards it never plays (D74); Patrol used 1.25-2.5 a game |
| 9:55-10:15 | code read 2: r69 cards, viewer for r70/r71, 159-165, content checks, faces by op | Honeypot Relay's own trigger, prohibitions unlabeled, hex-pick count, Network Cleanup III (D73), wording |
| 10:25 | odd input (`npm run odd`, 12 scenarios) | none |
| 10:35-10:40 | the farm's remote worker (message 7792), below | none |
| 10:40-10:50 | independent review of the hunt's own diff, 5 areas + verifiers | 17 faults in the diff, all fixed (a second free-play path, stale prose siblings, the sweep's word list) |
| 10:45 | verify sweep on the fixed build (`c3-d2`, `c2-d3-strategist`, seed 41432) | no hard fail; D75 only |

## The agent fix smoke (amendment 7657)

`smoke169.json`: strategist and greedy, the three pairs, d1-d3, 32 seeds (41500-41531), the agent fix off (rules
fixes in) and on. Win %, on minus off, paired SE: strategist d1 74.0 vs 76.0 (-2.1, 1.5), d2 71.9 (+0.0), d3 44.8
(+0.0); greedy d1 69.8 vs 71.9 (-2.1, 1.5), d2 62.5 vs 65.6 (**-3.1**, 1.8), d3 57.3 vs 56.2 (+1.0, 2.3). Over the
±2 bar, so the fix is deferred (D72) with its diff. The smoke ran on a loaded box (the farm's 1.6 free threads at
10:02am): no timing is read from it.

## The remote worker (message 7792)

The aic-farm wasm bundle built from this branch (`scripts/farm/build.mjs`) gives the same fingerprint as native
(`r73, content a856e33ac05ebb10, lines ebaa59b56eb70993`). `cell-d2`, seeds 41600-41603: native and wasm-node shards
differ in 0 of 4 games (`compare.py`); seed 41604 in Edge (`run.mjs --browser`) against native: 0 of 1.
`screens.assemble` of the wasm and browser shards reads in `ladder.py` against the native assembly: gap +0.0, 5 seeds
on mixed engines. The box was loaded, so no rate is reported.

## Not done

- `speed.py check` (rules and state changed: three small guards, no agent change): skipped at message 7804; the box
  was loaded all morning, so a timing read then proved nothing.
- The full 25-case sweep on the fixed build was started and stopped at message 7804 (41 captures, no hard fail).
- Lenses not run: long runs beyond the sweep's heap read (17-73 MB over 10 game pairs, no growth), a content sweep of
  art and audio, and real-speed play beyond the screenshots.
