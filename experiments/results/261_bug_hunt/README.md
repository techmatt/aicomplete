# 261 bug hunt (r121 → r122): the rules since 235

A hunt on the rules rewritten since hunt 235 (items 252-259: the node routes, free-then-claim and the Liberated
marker, cities never owned and hosting, `claim_site` and its rider, the converters' Actions and `w.sink`, the
adversary research track), before a balance pass reads on them; and the beam's stall 259 found. Start 1:38pm,
hunt-end target 4:38pm (cap 5:38pm). [fixed.md](fixed.md) lists F322-F378, [deferred.md](deferred.md) D106-D119 (D110 and D111 fixed as F368, F367) and
W34-W35; [docs/qa-findings.md](../../../docs/qa-findings.md) keeps the open list.

**Play changed (r122, one bump):** Peer Review's rider (F324), open-and-claim cards' targets (F323), a node route
needs a free hex for its node (F358), Factory Occupation's guard (F364), and Survey Drone Fleet's energy before its
nodes (F345, Matt's answer, message 11995). The golden game reaches none: its fingerprint held; the content hash
moved. Agents: the beam's stall (F322) and the card-target heuristic (F346). No read was farmed; no fresh seeds spent.

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 1:38-1:52 | recent changes 252-259, code read (8 finders, each list verified) | 30 real of 30: 3 bugs (claim cards' dead target, Peer Review's rider, the raw `retake_agents` label), stale rules text in five places for the cube converter, hosting, embargo, the research track; all fixed |
| 1:40-1:52 | the beam stall (seed 3) | reproduced (14 credits paid one at a time, then withdrawn, every plan); fixed with the phase's history, seed 3 back in the slow lane |
| 1:43-1:53 | the sweep, all 25 cases at one seed | stopped after its first case: 10 min a case (4 h for all); targeted subset instead |
| 1:54-2:02 | agents' op arms, log lines, faces by op, measures, this hunt's diff (5 finders, verified) | 12 real of 14: the target heuristic ignoring the claim rider, the Sweep valued on an empty track, readers counting Hostile Buyout's frees as cities, dead measures |
| 1:52-2:09 | as the user: the player view at real speed, 2 and 3 countries, 5 games to rounds 5-6 | 12: an empty "uncovers ," prompt, seats drawn "done" while another decides, track names, research never narrated, standard-action labels, a short Muster (Walls) |
| 2:09-2:23 | agent fixes measured (16 then 48 paired dev seeds, strategist d0-d2, split by fix) | the research valuation cost the judge ~4 points a level: reverted (D106); the target fix neutral, kept |
| 2:17-2:20 | e2e lanes and odd input on the fixed build | e2e, sources, odd green; e2e:researcher a SyntaxError since 252 (fixed, green); a parse test for every script |
| 2:17-3:00 | the sweep, 6 cases on the new rules (c3-d2, c2-d4, c3-d0/c4-d2 engine, c2-d2/c3-d4 strategist), 1,616 captures | green: only W16's `truncated`, one 270 ms step, heap 21-67 MB with no growth; every new moment captured |
| 2:20-2:28 | order inside a round, dead options, a printed game's bookkeeping, viewer paths (4 finders, verified) | 6 real of 10: a route paid with no hex for its node, a dead track question, the draft's keep price, rulebook limits, the research track missing from the pieces, a regression in this hunt's prompt fix |
| 2:28-2:36 | this hunt's own diff, independent review (3 reviewers, verified) | 13 real of 14: a split history row, Open Market's free keep, note wording, a test that skips the snapshot, page budgets; all fixed |
| 2:31 | Matt's message 11995 | Survey Drone Fleet reordered in r122 (fit, loop smoke, play read) |
| 2:42-2:50 | content sweep: adversary text, player aid and boards, the 252-259 cards (3 finders, verified) | 11 real of 14: Factory Occupation's missing guard (a bump), its unprinted fallback, ten word fixes, two card items deferred |
| 2:43 | the checker itself | `counter_names.py` checks the sweep's moments (finds main's three dead ones) |
| 3:00-3:10 | contact sheets of the 6-case sweep (new-rule moments first) | no bug at a new-rule moment; the step bar cuts a route's label (D112), "unaffordable" with no price (fixed) |
| 3:02-3:15 | long runs: the player view swept (c2-d0, c3-d2, c4-d3 engine; 655 captures) | green: no spike over 150 ms, heap 34-42 MB, only `truncated` |
| 3:02-3:10 | as the user again: the session's findings rechecked at real speed on r122 | 7 of 7 pass; the turn panel's raw events (D115) |
| 3:11-3:20 | paths not yet driven: `cards:check`, the legal probe, viewer state and old records (2 finders, verified) | `cards:check` only scene art (D83, grown to 12 cards); 3 player-view bugs (a record stuck at setup, Space re-pressing a header button, a pinned seat hiding the answers): fixed, `odd.mjs` scenario fails on the old viewer |
| 3:13-3:15 | the deferred list: D110, D111 | fixed (the report's hosted cities, stopped Muster landings named) |
| 3:20-3:30 | the late fixes, independent review (verified) | 11 real: the player-view record fix broke the sweep's watch (fixed, re-swept), Space on a summary, Factory Occupation's empty zone; all fixed |
| 3:21-3:23 | native = wasm (78 records, 12 variants), the wasm smoke | green |
| 3:23-3:35 | recent changes, analysts' tooling since 235 (3 finders, verified) | 20 real of 25: 8 fixed (read_lib on agent-lever screens, am_read, paired --pool, resource_use), 12 deferred as D117 |
| 3:38-3:50 | words against code in the pages not yet read (war, alignment, threats, card pages, architecture; 3 finders, verified) | 30 real of 30, all stale prose from before 206-259; fixed |
| 3:50-3:56 | words against code, the last pages (adversary, economy, board hand text; web, card-art and speed pages) | 5 real of 5, small; fixed: the lens is drying (30, then 5) |
| 3:56-4:00 | a new verb against old guards: the r113-r122 verbs (2 finders, verified) | players' verbs: nothing; one scope: the Liberated marker binds Corrupt lines only (words fixed; D118, Matt's call) |
| 3:58-4:05 | the hunt's own last diff (rounds 6-9), independent review | 3 real, small; fixed |
| 4:00-4:11 | the final build: full lane, clippy, odd, e2e, a 2-case debug sweep (c2-d0, c2-d2 strategist) | green; only `truncated`, no step over 150 ms, heap 26-40 MB |

| 4:12-4:20 | the rulebook's sections not yet read whole (§1-4, §7-10, §12-18; 3 finders, verified) | §1-4 and §12-18 nothing; one §9 line (a neutral city builds next to an agent too); fixed |

| 4:16-4:30 | content again: every face grouped by op, the whole deck and the adversary (2 finders, verified) | 10 real, all words: 3 fixed (Peacekeeping's "up to", "of your ambassadors", the events' botnet spread), 7 as D119 |

The hunt ran to about 4:35pm, the 4:38 target: every lens had had a round, and the last passes came up dry or nearly
so (words against code 30, 5, then 1; new verbs against old guards, 1 doc scope; the last diff review, 3 small; faces
by op, 10 wording items, no play fault). The remaining minutes went to the final green and the report.

**Seeds and reads.** No read was farmed and no fresh seed taken: the agent smokes ran on seeds 0-15 and 100-147
(16- and 48-game paired screens on this build, not confirmations), the sweeps on 2610-2650. No kept read was reused.

## Checker

`web/scripts/sweep.mjs` (web/README.md, *The sweep*) gains `COUNTER_EVENTS_R107` (each board power's first use,
claims by card and rider, a Liberated site's claim, hosting, sites opened, the research track, retakes) and drops
four moments no engine writes. `scripts/cards/counter_names.py` now checks every moment against the counters games
write. The fast lane's `node_scripts_parse` runs `node --check` on every Node script and module.
