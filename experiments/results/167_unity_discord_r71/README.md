# 167: Unity and Discord in fear-card shape, r71

Built on r70 (166, merged whole at 86beca1). `RULES_REVISION` 70 → 71 (from the constant, as the prompt said).
Seeds spent: dev 36800-36847 (screens), 40808-40855 (the full read), 40856-40903 (the confirm read),
40904-40908 (the on/off screen, 3 of its 5 seeds kept), 40912-40959 (a second fresh read, Matt's message 7611).

## The shape

Every Unity and Discord card is one verb, scoped by a **board condition** (never reach, never "one player"), its tiers
widening it. Three ops do it (`crates/aic-core/src/op.rs`, the conditions in `Where`, read by `plan::holds`):

- `per_site`: "for each X (at most N)", team ops once per site where X holds, counted on the whole board.
- `remove_where`: up to N agents, troops, botnets, markers or station and factory cubes (never a city's) off each hex
  where X holds, or in each country piece from such hexes, or by each player from one such hex.
- `prohibit` (158's firewall generalized): in the next adversary turn Rally, Deploy or Corrupt skips a site where X
  holds *as the step runs*. A Rally share or Deploy troop it blocks stays in the supply (no redirect); a covered site
  doesn't corrupt (Corrupt lines and natural corruption) and nothing is pulled into it. State: two vars a step
  (`prohibit.<step>.round`, `.where`), off at the end of the turn it acted in.

Conditions: holding an ambassador; on or next to one; on or next to an ambassador or a node; within 2 of one (tier
IIIs); a city, station or factory holding an ambassador; a corrupted city, on or next to one (tiers I-II only: r68's
tier-III clearing there was rejected); any corrupted site; a site a player owns.

The deck is written by `deck167.py` (alignment.json is hand-laid, so `apply_overlay.py` refuses it); its `V(...)`
entries keep every screened variant (first, A-H) beside the final one, H.

## Cards (final, H)

Unity, 23 (three new):

| Card | Verb | Condition, tiers I / II / III | Purpose |
|---|---|---|---|
| Grassroots Funding | credit income +1 | each owned site, at most 2 / 3 / 4 | kept |
| Open Market | free keep | each owned site, at most 2 / 3 / 4 | kept |
| Think Tank | insight income +1 | each owned site, at most 1 / 1 / 2 | kept (its III clearing dropped) |
| New Deal | gain 2 credits (+1 / 2 / 3 ambassadors) | each owned site, at most 2 / 3 / 4 | kept, ambassadors added |
| Mutual Aid | gain 1 energy (+1 / 2 / 3 ambassadors) | each owned site, at most 1 / 2 / 3 | kept, ambassadors added |
| Rebuilding Fund | gain 2 credits (+1 / 2 / 3 ambassadors) | each corrupted city, at most 2 / 3 / 4 | kept |
| Popular Mandate | gain 3 ambassadors | each corrupted city, at most 1 / 2 / 3 | kept |
| Safe Corridors | gain 2 ambassadors | each owned site, at most 2 / 3 / 4 | **changed**: its move is gone (moving is Gather and Push now, 166) |
| Ceasefire | counter a threat | each owned site, at most 1 / 2 / 3 | kept |
| Global Coalition | counter a threat | each corrupted city, at most 1 / 2 / 3 | kept (its credits dropped) |
| Volunteer Corps | sway 1 Unity | each owned site, at most 2 / 3 / 4 | kept (III's 8 ambassadors to one player dropped) |
| Citizens' Assembly | sway 1 Unity | each corrupted site, at most 2 / 3 / 4 | kept |
| Civil Defense | remove agents, each country piece | 2 / 3 next to a corrupted city; 6 within 2 of a player piece | **changed**: its station opening is gone (one verb) |
| Town Hall Accord | each player removes agents from one hex | 3 / 4 next to a corrupted city; 6 within 2 of a player piece | kept |
| Reconciliation Panel | remove agents from each hex | 1 / 2 next to a corrupted city; 3 within 2 of a player piece | kept (its Sway dropped) |
| Peacekeepers | remove troops from each hex | 1 / 2 by an ambassador; 2 by a player piece | kept |
| Disarmament Talks | each player removes troops from one hex | 2 / 3 / 4 by an ambassador | **changed**: troops only (agents are Town Hall's) |
| Decommission | remove each botnet | on / by an ambassador / by a player piece | kept, no longer "anywhere" |
| Network Cleanup | clear markers, for their reward | 1 / 1 / 2 on each hex by an ambassador / a player piece | **changed**: its botnet is gone (Decommission's) |
| Reclamation | adversary cube off stations and factories | on / by an ambassador / by a player piece | kept, never a city |
| **Neighborhood Watch** | no Rally lands | on a site holding an ambassador / by one / by a player piece | new prohibition |
| **Rule of Law** | no site corrupts | by an ambassador / by a player piece / within 2 of one | new prohibition |
| **Demilitarized Zone** | no troop deployed | by a player piece / within 2 / within 2 | new prohibition |

Discord: Incursion (an extra threat for each corrupted city, at most 2 / 4; III a tier-2 threat in each country piece
for each, at most 3), Two Fronts (the same by corrupted sites, at most 2; tier-2 2 / 3), Propaganda (every player loses
2 credits for each corrupted city, at most 2 / 3 / 4; **changed**: no Unity pull, no ambassador loss). Blight,
Offensive and Firestorm unchanged: they already name a board condition.

## Sim players

- The Rally forecast (`plan::next_rally`) gives a prohibited site's share to no one, and the eval puts no corruption
  pressure on a hex a pending Corrupt prohibition covers (`eval.rs`, `banned`): meeting the condition before the
  adversary turn scores. Deploy has no forecast term (troops are not read by hex).
- Values: `per_site` (its ops' worth × `rates.sites_expected` 2, at most its max), `remove_where` (the piece's own row ×
  `target_share` × hexes or countries), `prohibit` 10 credits (`configs/agents/values.json`). Resources at the ledger's
  values (`resources`).

## Reads (strategist, B+P / B+R / P+R, d0-d3, 48 seeds a cell, 576 games a side, paired against r70)

| Read | Deck | Seeds | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|---|
| full read | E | 40808-40855 | 90.3 vs 99.3: **−9.0 ± 2.6** | 71.5 vs 93.1: **−21.5 ± 4.2** | 68.8 vs 80.6: **−11.8 ± 4.6** | 50.7 vs 73.6: **−22.9 ± 4.6** |
| confirm (the prompt's one) | G | 40856-40903 | 95.8 vs 99.3: −3.5 ± 1.8 | 86.8 vs 97.9: **−11.1 ± 3.0** | 75.0 vs 83.3: **−8.3 ± 4.2** | 70.1 vs 75.7: −5.6 ± 4.2 |
| second fresh (Matt, 7611) | H | 40912-40959 | 94.4 vs 100.0: −5.6 ± 1.9 | 82.6 vs 95.1: **−12.5 ± 3.2** | 78.5 vs 82.6: −4.2 ± 4.2 | 70.1 vs 75.7: −5.6 ± 4.5 |

r71 farmed (`farm-seeds`: native, a few shards wasm on the laptop), r70 native from 86beca1's binary (`out/r70`).
Each fresh read was spent on its own deck: E → G (more ambassadors, Demilitarized Zone's tier I widened) → H (three
seldom-true conditions moved to owned sites). G and H each changed after a read, so neither is confirmed by a read
of its own deck other than the one listed.

Dev screens (36800-36847, 16 seeds a cell, against r70 on the same seeds; `*_screen.txt`): the first rewrite read −15
to −26; **r70's Unity tiers restored on the r71 build** read +2 / −2 / 0 / −12 against r70, r70's Discord tiers −25 to
−15 (no help): the gap is the Unity rewrite. Stronger variants A-D closed part of it (B, D: −6 to −19 on 16 seeds).

## Why the delta misses

The lost games are round-cap losses at **6+ Unity tips with 1-2 corrupted cities left** (`ends167.py`: d1 on H, 25 of
25 losses at 6+ tips, 1.8 cities left and nothing else; r70 on the same seeds lost 7). At 6+ tips only cities count,
so the tier III that resolves then decides it. r70's tier IIIs were seven "remove up to 6 agents on or next to one
corrupted city in reach" (r68's clearing, rejected by Matt) and "one player gains 6-8 ambassadors": they fuelled the
last Liberates where the players stood. Ambassador gains (G, H: up to 9-12 a player at tier III) recover about half the
gap. Two d1 dev screens on H (36800-36847, 144 games a variant, against r70 on the same seeds; `clear_screen.txt`,
`tiers_screen.txt`, specs `clear167.py`, `tiers167.py`):

| d1 | win % | against H | against r70 |
|---|---|---|---|
| H | 80.6 | | −12.5 ± 4.0 |
| H, tier-III removal by a corrupted city (6 / 6 / 3) | 81.9 | +1.4 ± 1.7 | −11.1 ± 4.0 |
| H, r70's tiers I and II on every Unity card | 81.2 | +0.7 ± 4.4 | −11.8 ± 3.6 |
| H, r70's tier III on every Unity card | 95.8 | **+15.3 ± 3.2** | +2.8 ± 2.6 |

So the gap is r70's tier III, and not its clearing as such: the same removal scoped by a board condition is worth
+1.4. What r70's tier III had is **aim**: one player, in their reach, took the clearing, the 6-8 ambassadors or the
move where the next Liberate was. The fear-card shape spreads the same help over the board by design. This is the rule
that resisted. The 5-seed r70-card restore screen (`restore_r70_cards.txt`, 9 games a cell) is too small to name one
card.

No collapse, though: every level is within ±13, d0, d2 and d3 within ±8 on H. Since 162's judge every band reads
13-16 high, so 168's recalibration has room to take the gap up with the adversary dials rather than reintroduce reach
or one player on Unity cards.

## Per-effect statistics

`scripts/cards/effect_stats.py`. What each effect did a resolution and each prohibition's fire and block counts:
`effects_G_confirm.md` (the confirm read), `effects_E_read.md` (the full read). The on/off screen (`effects_E_onoff.md`,
deck E, seeds 40904-40908, 3 of 5 kept: 9 paired games a cell) is too small to resolve ±5: one game is 11 points.

Flagged (beyond ±5 points over the games the card resolved in, at some level) on E's screen: Grassroots Funding,
Think Tank, Civil Defense, Volunteer Corps, Global Coalition, New Deal, Town Hall Accord, Popular Mandate, Mutual Aid,
Network Cleanup, Citizens' Assembly, Rebuilding Fund, Safe Corridors, Neighborhood Watch, Rule of Law, Demilitarized
Zone, Incursion, Offensive, Two Fronts, Firestorm. Each flag rests on 1-6 games (SE 11-50 points), none beyond 2 SE:
left for that reason. What changed after the screen was for the delta, not a flag (G, H). The effects that are
large by what they do (`effects_H_second.md`): New Deal (+10 ambassadors and +9 credits a resolution, summed over
seats), Popular Mandate and Safe Corridors (+8 ambassadors), Rule of Law (12.5 pulls kept off a firing game): candidates
to trim once 168's dials are set.

**Prohibitions on H (the final deck; second read):** Neighborhood Watch fired in 55% of the games it resolved in
(by tier: I 59%, II 80%, III 40%), 5.4 agents kept off a firing game; Rule of Law 44% (I 21%, II 63%, III 43%), 12.5
pulls a firing game; Demilitarized Zone 62% (I 81%, II and III 56%, one condition so one tally), 3.0 troops. On G
(confirm read): 56% / 50% / 64%, 5.9 / 11.4 / 3.1.

## Checks

`speed.py check` gives no verdict (its baseline is r69). `per_decision.py out/r70 out/r71h` (other games on the same
seeds, so ms a decision): greedy 1.74× on 6 games from seed 1000, 0.57× on 12 from seed 2000; the strategist 0.89× on
4. No consistent cost: game-to-game spread. `loop_smoke.py --run`: no timeouts, no card at the play flag (most 4).
Fast lane, `test-all`, clippy and `docs --check` clean. `cards:fit` not run: Alignment cards have no frame and no
player card changed (the effect ledger moved only its revision label).

## Files

`deck167.py` (the deck), `specs167.py` (farm specs: split, decks, onoff), `pair167.py`, `screen167.py`, `ends167.py`,
`counters167.py`, `fx167.py` (readers); `*_screen.txt`, `read_E.txt`, `confirm_G.txt`, `second_H.txt` (outputs).
