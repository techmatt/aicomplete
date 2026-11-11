# 184 bug hunt (r77 → r78)

A hunt on what changed since 169 (r73): r74's one Corrupt shape and nodes at 2, r75's contact and payments, r77's
rulebook-friction rulings and renames, the rulebook, 179's icons. Start 4:46am, hunt-end 7:46am (cap 8:46am).
[fixed.md](fixed.md) lists F225-F271, [deferred.md](deferred.md) D77-D87 and W28; [docs/qa-findings.md](../../../docs/qa-findings.md)
keeps the open list. One rules revision (r78: F225, F262); the agents' three misreads fixed (F257, play moves).

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 4:46-5:25 | recent changes: code read of r74-r77 and the rulebook (5 finders + verifiers); the sweep, 6 cases at 2 seeds (42997-42998); odd input; loop smoke and the read tools (43013-43028) | 11 (r77 words and icons, Dispatch prompt, rulebook blackout, Station Coup, solo cost); odd 12/12 ok; no loop |
| 5:15-5:25 | r77's rule items one by one (strikes, convert, liberate cards, threats' locations, Vigil and the draft, Clear and firewalls) | 9 (Nuclear Option offer, convert icons, waits token, cordon hover, read scripts) |
| 5:25-5:40 | content: card fit and the card viewer check, the sweep's 33 contact sheets read as a new player (4 readers + verifiers) | 16 viewer faults; 5 cards under 7.5 pt; 10 scenes never drawn |
| 5:32-5:45 | code read 3: the viewer's code, agents against r74-r77, read scripts, the card viewer, the rules files | 16 (waits badge never shown, threat effects hidden, card viewer words and blank columns, 3 agent misreads, dead win split) |
| 5:55-6:00 | the e2e lanes on the fixed build and on main (a scratch copy) | `e2e:corps` and `e2e:researcher` red on main since r75: stale checks, fixed |
| 6:05-6:30 | independent review of the hunt's own diff (4 reviewers + verifiers) | 26 faults in the diff, all fixed; the rulebook's revision line now checked |
| 5:57-6:36 | sweep 2: the paths not yet driven (11 cases at 1 seed, 43029: off-variants, shared row, 1 and 4 countries, overlay, cubes) | 1,673 captures; one clipped label (this hunt's own, fixed); a content-hash banner from rebuilding mid-sweep |
| 6:31-6:39 | code read 4: faces by op, the player aid, sim reports, content checks, records across revisions | 8 (solo token fallback: a rules fix; saved sessions; probe classes; words) |
| 6:37-6:45 | sweep 2's 55 contact sheets read as a new player (5 readers + verifiers) | 10 (tier chip, last-node hover, red chips after a win, prompt grammar; six deferred, D87) |
| 7:00-7:21 | the final build: all four e2e lanes, odd input, the engine's per-decision re-read, a verify sweep (5 cases, 43038) and the checks this hunt taught (F267) | all green: 721 captures, no hard fail, no spike; engine 0.97× a decision |
| 7:28-7:40 | long runs and state across games and sessions (3 finders + verifiers: the viewer over many games, aic-sim over long and resumed runs, the card viewer over a session) | 3 (a stale tooltip, a resumed run's heuristics rate, a silent "Mark all seen"), fixed with D87's flag chip |

## Checks run

- **Sweep 1** (r77 build, c2-d1 and c2-d3 strategist, c3-d2, c4-d3 engine, c1-d4, c2-d2 engine; 24 games, 1,559
  captures): no hard fail, no step over 150 ms, heap 7-70 MB a game; overlaps are D75's troop counts.
- **Engine smoke** (`use.json`, 16 seeds): no timeout, no card past the play flag; the strategist keeps 43 cards it
  never plays (D74); Focus and Recovery never used (D84).
- **Agent fixes, on/off** (same 16 seeds): 13 of 96 games play differently, every cell's win % equal. Cost a decision
  (`per_decision.py`, 8 games at c3): strategist 1.00×, greedy 0.91× on a loaded box; engine 1.20× there, 0.97× on a
  quieter re-read (best of 2 passes).
- `speed.py check` gives no verdict across a revision (r77 baseline, r78 build); the rules change is one guard in
  `feasible`, so the agents' per-decision read above stands for it.

Seeds: sweeps 42997-42998, 43029, 43038; smokes 43013-43028; per-decision 43030-43037 (ledger: next free 43039).

## Not done

- A real-speed session by hand: 88 contact sheets were read instead (nine readers, each finding verified).
- Long runs beyond the sweeps' heap read (8-72 MB a game across 52 games, no growth from game to game).
- Audio: the project ships none.

## For a new player (r78)

- A card is playable when some part of it does something; a sway toward Discord is a price, not a part. The Nuclear
  Option can't be played without a corrupted city in reach to liberate.
- A threat that lands troops "on the base here" or "on the factory here" lands nothing when it has no such token
  (at 1 country there are no bases).
