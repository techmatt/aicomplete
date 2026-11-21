# 305: overhaul 7 of 7, the viewer pass (r148)

r148, no bump: no play changes. The prompt named r147; 303 bumped to r148 ahead of this item, so every read here is
r148's. Engine edits are prompt words only (asks' `prompt` strings, never an option label): `golden_game` holds.

**Goal:** a person who has read `docs/teach.md` plays a whole game in the player view (V off), every overhaul rule
on screen and operable there.

## The bars

| Bar | Result |
|---|---|
| A full scripted d0 game at two seats through the player view's controls, no debug-view step | `baseline.json` seed 2: **finished**, lost in round 11 (the adversary deck ran out), 306 actions, every one clicked; 0 screen problems; every overhaul rule seen |
| The same at three seats | `three_seat.json` seed 1: **finished**, lost in round 10 (seat 2's last node), 461 actions; 0 problems; every rule seen |
| No clipped text at 1366×768 or 1920×1080 | the sweep, cases c2-d0 and c3-d0 (play seed 1000), debug and player views, shot at 1366×768 and at 1920×1080 (each run also checks the other sizes' layout): **0 hard fails in all 8 runs** (table below); the two games above at 1366×768 and seed 2 at 1920×1080: 0 clipped |
| The card viewer shows r148's faces after Rebuild | see *Card viewer* below |
| Screenshots of each turn phase at 1366×768 | [shots/](shots/) (33) and the contact sheet [sheet.png](sheet.png) |

"Scripted" is `web/scripts/playthrough.mjs`: play-policy.mjs's small policy (295's, which reads the engine's view to
choose) on the configs as shipped, each move made by clicking the control the player view shows for it, the turn
panel stepped through with its ◀ ▶ and Done. A policy game loses at d0 (the policy ignores the research row); the
bar asks the game to finish, and both do. The script fails on page scroll, off-screen elements, clipped text (no
ellipsis), a Draft / Keep button or the turn panel below its fold, placeholder words, cut pieces, console errors, a
rule drawn wrongly, or no control for the policy's move.

### Each overhaul rule, as the two games saw it on screen (round first seen, 2 seats / 3 seats)

| Rule | Where the player view shows it | 2 | 3 |
|---|---|---|---|
| Ambassadors tip after one move or act | tipped glyph on the map; *ready ambassadors n/m* on the board | 1 | 1 |
| Liberate at 3 | *Liberate* button when legal; the bar's Liberate line: each corrupted site holding your ambassador, "3 ambassadors, n here + m from supply", **ready** or **blocked:** why | 1-2 | 1-2 |
| What blocks Liberate | the Liberate line (an agent on it; a troop guarding it; your ambassador there already acted; short n), *Liberate refused* (294) and the guard's shield on the map | 1 | 1 |
| The three node powers, used this round | *Node powers* heading first in the seat panel, "n of 3 used this round", each power tagged used | 1 | 1 |
| The adversary's two beats | the turn panel names each step's beat ("the adversary's Card beat" / "Income beat") | 2 | 2 |
| The adversary's turn phase by phase | the map shows the pieces as at the end of the shown step (new, below) | 2 | 1 |
| A Strike shown a round ahead | the red notice under the face-up next card; solid strike links | 4 | 6 |
| Redeploy | the notice under the next card; the troop's move as its own step on the map | 8 | 7 |
| Troops and where they garrison | troop glyphs, guard links with shields; the legend keys both | 5 | 7 |
| The research row and K | slot boxes, cubes n / K to the next slot, "K 6 = 3 a country" | 1 | 1 |
| The round cap and the deck left | "Round 4 of 10 at most · 6 adversary cards left" in the round bar | 1 | 1 |
| Discord and Unity | the Alignment box: both tracks, tips, resolved cards, Unity deck, Discord deck in *Lose if* | 1 | 1 |

## What changed

- **The player view at 1366×768 and 1920×1080** (`player.css`): below 2200 px wide the side column runs the
  window's height and the seat panel and bar sit under the map. Before, the side column was 300 px tall at 1366×768
  with the adversary row three scrolls down, and the board's node powers sat under the seat panel's fold
  (`.p-powers` at y 655 of a 676 px viewport). The seat panel puts the node powers in its first column; the draft
  and keep take its width.
- **The turn panel's map by step** (`turnView` in `view.ts`, staging question 1's default: the client, no engine
  call): while an earlier step shows, each piece a later step changed goes back to its count before that step's first
  change (the events' `total - delta`); a troop's strike and guard links go with a troop no longer there. Tracks,
  piles and rows stay as after the turn, and the panel says so. Hover over a hex or token reads the same view
  (`aic.mapView`), as does the sweep's token check.
- **Round cap**, **K**, **beats**, **node powers' heading**, **Liberate line**: as in the table above.
- **D135 (295's clip):** the debug view at 1366×768 late in a long game scrolled the page: the Strike-ahead notice
  took four lines of the next-round box and the board's 41vh floor pushed the countries strip under the pane. One
  line with a hover, and a 33vh floor at heights up to 820 px. Sweep c2-d0 play seed 1000 (D135's own game): 0 page
  scroll.
- **D87:** Alignment off: already drew nothing (`alignmentHtml` returns empty without the values, the Discord-deck
  loss only with its pile): no change. The chips beside *Lose if* now sort by key. The legend keys the troops' strike
  reach, guard and a factory's push. Country labels on the map at 0.42 with a white halo (were 0.25). The debug
  view's hand shows no play marks while a question is open (the player view's since 289).
- **D85:** "which other player?" names the card (`Ctx::why` set for a played card; a building's by its card); the
  protection prompt names what destroys the piece ("would be destroyed by Dark Cities", or by a troop's Strike,
  agents landing on it, a Discord card...); "1 seat 1's ambassador" (a record key) shows as "1 ambassador of Seat 1
  (Builder)", and "seat 1's node" in a question as "Seat 1 (Protector)'s node". "Corrupt factories (at 2)" and the one-troop move prompt are gone from the engine since 293-298. No
  agent parses any of these (`git grep`).
- **D50:** its picks: the Reuse prompt names each building's cost paid again; a used building is dimmed, not struck
  through. The rest were done by earlier items. Its three Matt's calls stay in qa-findings.md.
- **D52:** measured, not needed (staging question 5's default): the heap after a whole game in the player view is
  14 MB (2 seats, 306 actions), 22 MB (3 seats, 461) and 29 MB (the baseline at 4 countries, seed 3, 624
  actions, by the policy through the engine); the sweep's c3 runs hold 47 MB. Moved to won't fix (W35).
- **Checks fixed on the way (both red on main):** `e2e:researcher` asserted Peer Review leaves the research cubes
  unchanged, but it gains one since 295 (now "none spent", at least as many); `troops-play.mjs`'s demo seed 17 no
  longer shows a guard at r148 (engine-only `--search 13-60`: 14 shows all three in round 3-4; the new default).
  `npm run check` also parses every `scripts/*.mjs` (`check-scripts.mjs`).

## Sweeps (one seed a case, AIC_SWEEP_SEED 1000, final build)

| View, shot size | c2-d0 captures, hard fails | c3-d0 captures, hard fails | Look |
|---|---|---|---|
| debug, 1366×768 | 153, 0 | 245, 0 | truncated (with ellipsis and hover) only |
| player, 1366×768 | 153, 0 | 245, 0 | truncated only |
| debug, 1920×1080 | 153, 0 | 245, 0 | truncated only |
| player, 1920×1080 | 153, 0 | 245, 0 | truncated only |

The first player-view pass reported 99 / 109 `token-missing`: the map drawing an earlier step while the sweep read
the view after the turn. The sweep now reads `aic.mapView`; the rerun has none.

## Shots (1366×768, player view, [shots/](shots/), [sheet.png](sheet.png))

`two-` is `baseline.json` seed 2, `three-` is `three_seat.json` seed 1: each phase the first time it showed from round 2,
each step of the turn panel, each kind of prompt, and the troops' moments.

| Phase | Shot |
|---|---|
| A Sway (Fallout's Discord) | `three-phase-36-r10-turn-A-Sway` |
| B, the adversary's Card beat: its card, threats, lines | `two-phase-03`, `-04`, `-05` |
| B, the Income beat: the next card up, the sites' output, compute | `two-phase-06`, `-07` |
| C cards: tier, draft, keep | `two-phase-08`, `-09`, `-10` |
| D actions, and its prompts: a node's place, a move, which ambassador, a target | `two-phase-11`, `-12`, `-14`; `three-phase-22`, `-25` |
| A liberation | `two-moment-liberated-r2` |
| E income and the boards | `two-phase-16`, `-17` |
| F threats and their prompts; a protection | `two-phase-01`, `-18`, `-21`, `-22`, `-23` |
| G cleanup | `two-phase-02` |
| Troops: garrison, Redeploy's move, a Strike | `two-moment-garrison-r5`, `three-moment-redeploy-r8`, `three-moment-strike-r8`, `three-phase-32` |
| The end | `two-end`, `three-end` |

## Card viewer

The viewer serving this checkout, after **Rebuild**: title "AI Complete: cards, r148", status "r148 · 3b506fa8 ·
rebuilt in 154 ms"; all 351 exported cards drawn; each of the 210 player cards' printed text (`web/card/faces.json`,
which `aic-sim docs --check` holds current at r148) on its face. Shots: [cards/](cards/) (Redeployment, the
Researcher's board). `npm run cards:check`: 52 ok, 2 FAIL, both D83's missing scenes (14 deck cards name no scene,
6 scenes not served; the art waits, as the prompt says). The stats selector still offers r94's screen (s213b), with
its "1 warning" note: no newer `results.json` holds per-card counts.

## Diff review

One reviewer pass over the whole diff: no blocker; fixed: the round cap read a round high during Sway (the deck
count holds the face-up next card, still this round's then), the Liberate blocker said "acted" where the ambassador
had only moved in, K's hover printed the ladder's numbers (a tunable), the Reuse prompt named only the first branch's
cost, an unnamed destroying effect read "the adversary", and the playthrough's Liberate check passed on any "3".
Also fixed from it: `docs/teach.md` priced a station's Liberate at 4 (3 since 301).

## Not done, and why

- Card art stays as it is (the prompt).
- The Strike-ahead notice and the turn panel step list use ellipsis with a hover at 1366×768 in the debug view: the
  sweep's `truncated` look, not clipping.
