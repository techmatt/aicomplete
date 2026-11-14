# 235 bug hunt (r106): playtest readiness

A hunt for Matt's browser playtest after r98 → r106 in a day (227 Fallout and Reclaim, 228 the Researcher, 230
antimatter, 231/233 the one-shot project deck on per-board compute, 234 one target per map-touching card; 234 landed
after staging and joined the scope). Start 11:34am, hunt-end target 2:33pm (cap 3:33pm). No play changed: the
golden fingerprint held; the content hash moved for the adversary faces. [fixed.md](fixed.md) lists F289-F317,
[deferred.md](deferred.md) D92-D102 and W31-W32; [docs/qa-findings.md](../../../docs/qa-findings.md) keeps the
open list (D73 closed there: Network Cleanup III now widens to within 2).

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 11:34-11:42 | setup; the sweep's quick cases at r106 (8 games) | clean but for W16's `truncated` |
| 11:38-12:00 | recent changes r99-r106, code read (5 finders, each list verified) | 32 real of 34: stale rules and rulebook text, 4 log lines, the invisible reach Shelter, Lift Fallout's price, the supply chip; fixed but 3 deferred |
| 12:02-12:16 | as the user: one full game by hand at real speed (baseline seed 235, 10 rounds, every move on screen) | 11: pointless harm choice, opening-hand label, jam wording, draft tier, part payments, Sheltered count, banner code ... |
| 12:02-12:42 | the sweep, 5 cases × 2 seeds (20 games, 3,100 captures); contact sheets reviewed | the threat ring over hex numbers, inverted Dispatches chip, axial coordinates in labels, the loss strip |
| 12:03-12:10 | content sweep: faces by op, adversary faces, prompts, the player aid (4 finders, verified) | 14 real: Botnets spread, Sabotage Teams, Factory Drive, raw-key and plural prompts, Liberate's factory price, insight rule |
| 12:18-12:20 | odd input (`npm run odd`, 12 scenarios) | clean |
| 12:24-12:42 | the hunt's own diff, independent review (4 areas, verified) | 17 real of 19: sibling prompts, the draft tile's lost needs hint, the ring fix too small, `warn-at-start` noise, a face overclaim |
| 12:43-1:03 | the fixed build re-swept (5 cases) and `cards:fit` | clean; no card overflows |
| 12:43-12:48 | code read: dead target picks, viewer state, view paths | 4: dead picks on 4 cards (deferred, a bump), Space after a click, threat sacrifice options lost from the bars |
| 12:58-1:05 | as the user: 3 countries, d2, the Researcher | 8: sacrifice prompt words, unexplained Discord, the opening's project (Matt's call), unnamed sacrifice |
| 1:03-1:47 | long runs: 3 cases × 3 seeds (27 games, c4-d3 included) | clean: heap 22-49 MB a game with no growth across them, steps ≤ 88 ms |
| 1:05-1:10 | card viewer and export, agents against the changed words, the rulebook's round walk | 5: card backs' keep 2 (3 since r95), boards' insight missing, OR ops, phase names, economy.md order; agents: nothing |
| 1:08-1:18 | e2e lanes and `cards:check`; main built side by side; the new checks validated on main | e2e, sources, researcher green; corps red on main too (D102); `cards:check` fails only on scene art (D83); `mark-over-number` and `warn-at-start` fire on main, not here |
| 1:19-1:35 | as the user: targets by hand (10 targeted cards, 12 plays), 4 countries | 7: silent lone targets, map halves that skip in silence, project cost × countries, ids in the log, a two-part building's text |
| 1:19-1:30 | rules a printed game can't track (r99-r106) | 4 of 4 claims, 3 real: round flags and incomes with no marker (Matt's), Lab Purge's order (face fixed), Fallout's cross-border tie-break (a bump) |
| 1:47-2:05 | a second independent review of the later fixes (2 areas, verified) | 7 real of 7: Space-after-click still open in Chromium (`:focus-visible`), draft hints silenced by the phase names, the "or" lost between threat options, Discord notes with Alignment off, a war factory's Discord unnamed; all fixed |
| 2:06-2:20 | the final build: e2e, sources, researcher, odd input, two sweeps (6 games), `cargo test-all`, clippy | green; the duplicate Discord deck chip (D98) fixed and re-checked |

The hunt ended at 2:20pm, 13 minutes before the target: every lens had had a round, the last fix was in and green,
and the remaining deferred items each need a rules bump or Matt's ruling. The leftover budget went to D101 (a power's
lone sacrifice named on its button, F321), tested and green by 2:35pm.

## Checker

`web/scripts/sweep.mjs` (web/README.md *The sweep*) gains r99-r106's moments and two look-only checks,
`mark-over-number` and `warn-at-start`, both shown to fire on main's viewer and stay quiet on this build.
`invariants.rs` checks a node-destroyed log line against the board in every random game.
