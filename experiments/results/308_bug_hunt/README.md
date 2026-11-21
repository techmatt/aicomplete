# 308 bug hunt (r149 → r150): the overhaul since 285

A hunt on what changed since hunt 285 (r136, items 286-307), the planner's focus first: the new adversary turn and
income, Redeploy and Strike, liberation, the event deck's deck-out, 305's player view and 306's finale buildings.
Start 11:16pm, cap 3:15am. [fixed.md](fixed.md) lists F424-F445, [deferred.md](deferred.md) D136-D141;
[docs/qa-findings.md](../../../docs/qa-findings.md) keeps the open list.

**Play changed (r150, one bump from the constant 149):** a Push and a strike-gather reach a troop on or next to your
ambassador, as r142's rule and `target_hexes` already said (F424, F425); Cascade Failure's botnet reads its station
token only (F426). The golden game reaches none: its fingerprint held; the content hash moved (faces' words). No read
was farmed and no fresh seed spent (the staging default stood): sweeps on 3080 and 3090, the user sessions on
3101-3104.

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 11:16-11:24 | setup: release and test builds, snapshot on port 65493, fast lane; e2e, odd, sources, researcher, corps, troops-play, ambassadors-play, playthrough, `cards:fit` | all green on r149 |
| 11:17-11:27 | round 1, recent changes: code read of the five focus areas (5 finders, each list verified) | 16 real: 4 user-facing bugs (Push reach, Closed City on a factory, a sheltered strike unnarrated, the cleanse's agents: D136), ~12 words (Redeploy's r141 wording in five places, the deck-out round, rulebook Liberate prices, War Research's K) |
| 11:24-11:41 | the sweep, debug view, r149: c1-d4, c3-d2 and the new c2-d0 / c4-d4 strategist cases, 702 captures | green: only W16's `truncated` |
| 11:28-11:42 | round 2, as the user: four player-view games at 1366×768 through its controls (a helper), every Liberate line checked against the engine (119 states) | 1 bug (Strike ahead shown during its own Card beat), 6 words or polish (pay buttons read "+1", labels, the Liberate rule line) |
| 11:33-11:43 | round 2, words against code: adversary faces by op, Unity / Discord faces, rules pages (3 finders, verified) | 11 real: 2 bugs (Cascade Failure's botnet on the city, Kettle Operation's reach: F425, F426), 9 words |
| 11:42-11:48 | round 3, contact sheets and ~25 full-size shots at the new-rule moments (a reviewer) | thin: 0 bugs; words (a struck facility's owner, chip labels), D138 |
| 11:52-12:09 | round 4: the sweep in the player view on r150 (c2-d4, c3-d1 / c4-d1 strategist, c2-d2 engine, 756 captures); every two-location threat's located ops read (round 2's pointer) | thin: green, only `truncated`; the located ops clean |
| 11:52-12:01 | this hunt's diff, independent review (3 reviewers, verified) | 12 real in the fixes: the strike's limit per hex, `line_ahead` hiding a Redeploy still to come (reverted for words), the tipped summary on a moved ambassador, the playthrough check on the new Liberate line, faces and prose; all fixed but D141 (agents) |

Rounds 3 and 4 were thin in a row: the hunt stopped on yield at about 12:10am.

## Checker

`web/scripts/sweep.mjs` gains `COUNTER_EVENTS_R142` (r142-r149's moments, checked against 1,152 r149 games of
306's kept read: troop and Wall strikes, a troop destroyed by site, a Redeploy that stayed, a botnet taken down, Vigil,
a card's liberation by site, the antimatter standard action, troops besieging or walled, a Discord pull, each finale
building's Action) and two judge cases, `c2-d0-strategist` and `c4-d4-strategist`. c4-d4-strategist at one seed
takes about 6 minutes (its records are the strategist's at 4 countries). Each play fix has a fast-lane test; the view
and note fixes are words the sweep's `stale-word` and the e2e lanes read.

**Seeds and reads.** Sweeps on 3080 and 3090 (one game a case and mode), user sessions on 3101-3104; 306's kept r149
games file (`r149-screen-judge.jsonl`) fed the counter-key check.
