# 238: playtest fixes from 235's hunt (r108)

The play problems 235's bug hunt deferred, fixed so Matt's playtest doesn't meet them (his picks in
[235's deferred.md](../235_bug_hunt/deferred.md)). `RULES_REVISION` bumped from the constant, 107 to 108. Base: main
398ae1c (237's agents cleanup, r107).

Files: [replay.txt](replay.txt) (the same-seed replay: `farm/compare.py`, [moved.py](moved.py), [plays.py](plays.py)),
[fresh.txt](fresh.txt) (`ladder.py --measures`), [fresh-moved.txt](fresh-moved.txt), [design_check.txt](design_check.txt),
[play_read.txt](play_read.txt), [speed.txt](speed.txt). Spec: `experiments/farm/f238.json` (fresh read).

## What shipped

- **D93, dead target picks.** Three target conditions share their op's own test (`TargetSpec`, `effects/targets.rs`):
  `ambassadors_next` "a land hex next to an Ambassador" (`moves::gathers_into`, a Gather of Ambassadors: Exodus
  Corridors, Rally the Allies), `no_firewall` "a land hex with no Firewall" (`endgame::firewall_spot`: Firewall Array's
  Action), `your_ambassadors_next` "a piece next to your Ambassador" (`ambassadors::consolidate_onto`: Summit Caucus).
  The faces print them. Other targeted moves checked: Liaison Corps gains its ambassadors first and Transmission
  Corridor (out of the deck) takes the supply as a source, so neither offers a dead piece; Air Gap's firewall rides
  beside a removal on Agents; Peacekeeper Rotation is W31 (out of the deck). 236's narrowing in `aim` stays for Push
  (Patrol, Interceptor Battery), whose test no box prints; for gather and firewall it is now the box's own test.
- **D92.** A tie for nearest is asked only when an op of the harm reads the hit seat (`Op::reads_hit_seat`, op.rs;
  `Ctx::seats` debug-asserts the list, and the random-game tests ran it). Otherwise the lowest tied seat stands in,
  read by nothing (`threats.hit_ties_unasked`). `one_player` doesn't count: it resolves under its own seat.
- **D94.** The march stops once a seat has no node (with `node_loss` on): no troop moves or asks after the last
  node falls (`board.march_cut_by_loss`).
- **D103.** Fallout's landing next to its anchor sorts the free neighbors by (country, hex number), the rulebook's §2
  tie-break (was number alone).
- **D95.** The seat whose action swayed the Unity places it (`Choices::acting`, set per trigger in `rules.rs`; seat 0
  in an automatic phase): "Seat 1: put 1 Unity on the track, or take 1 Discord off?", buttons "put 1 Unity on the
  track" / "take 1 Discord off". A tip card resolving in an action's checks asks the acting seat too.
- **D96.** A reach Shelter is one flag a seat (`board::reach_shelter(seat)`), read against the reach when a piece
  would be destroyed (`board::reach_sheltered`), so Aegis Grid Lockdown's own new nodes are covered. The agents'
  shelter read (237, D97) follows through the same function.
- **D100.** The adversary's visible projects the next compute step fills, as the board stands (income, the botnets and
  stations making compute, what waits; `projects::completes_next_step`, sharing `compute_step`'s source test), show
  ", completes next compute step" on their compute bar. The rulebook's §17 tells players to add it up.
- **D105, D98 (viewer).** A ready card or Action whose map half has no target in reach reads "ready · map part: no
  target" (warn color, still ready). A card's joined text puts "When played:" before the one-time part when a lasting
  part sits above the art. At short heights (≤ 820 px) each threat is one line (name, source, payments; hover for the
  rest).
- **D104** is in the rulebook's pieces table: a round-flag marker and the adversary's compute and agent income tracks.
  **D99** (Teardown's Fallout stays under a node) is Matt's ruling, recorded as W33 in `docs/qa-findings.md`.

**Prices.** No card was repriced: a target condition only removes picks where the op moved nothing. Plays per game of
the four cards on the replay: Summit Caucus 0.655 → 0.649, Exodus Corridors 0.184 → 0.184, Rally the Allies 0.484
→ 0.484, Firewall Array 0.167 → 0.168 (Action uses 0.357 both). No give / get line changes.

## Reads

- **Same-seed replay** (f236's seeds 47328-47375, 1,152 games a side, base r107 native, new farmed): 1,146 games differ
  in some decision, 3 in outcome; win 74.1 → 74.2 (paired +0.1 ±0.2). What moved, a game: tie questions 1.03 → 0.61
  (0.42 now unasked), marches cut at the loss 0.06, targets taken 25.90 → 25.55 and missed 7.14 → 7.51 (a card that
  plays for its other lines no longer pays onto a dead hex), Unity held off 5.92 → 5.94.
- **Fresh read f238** (47376-47423, taken; 48 seeds a cell, 2 players, B+P / B+R / P+R, d0-d3, 144 games a row),
  paired against base r107 natively:

| agent | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| greedy r108 (gap) | 88.2 (+0.7) | 77.1 (0.0) | 71.5 (0.0) | 44.4 (−0.7) |
| strategist r108 (gap) | 85.4 (+0.7) | 76.4 (+0.7) | 67.4 (0.0) | 50.7 (0.0) |

  Paired SE 0.0-0.7; win SE 2.7-4.2. Bands (soft): greedy d2 and strategist d2, d3 above theirs, as on base. Standing
  measures within 1% of base on every row (shares within 0.1 point) ([fresh.txt](fresh.txt)). What moved, a game ([fresh-moved.txt](fresh-moved.txt)):
  tie questions 1.10 → 0.68, marches cut at the loss 0.10, targets missed 6.37 → 6.65; 4 of 1,152 outcomes flip.
- **design_check** (fresh, against base): the same warnings as base's own read (Dyson Swarm Tranche and Deep State
  Ledger kept-not-played, five usage bands, d0 rounds 10.1); none from this item.
- **play_read** (fresh): no card kept and never played, none never kept. **loop_smoke** ([loop_smoke.txt](loop_smoke.txt)).
- **Speed** ([speed.txt](speed.txt)): `speed.py pair` r107 → r108 on a quiet box, engine 0.99-1.00×, greedy
  0.98-1.04× (games differ: speed times length, and lengths moved < 1%). `check` gives no verdict across the bump;
  the baseline is re-based at r108 (`--update`, nothing beside).
