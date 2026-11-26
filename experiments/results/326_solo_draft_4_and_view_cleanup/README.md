# 326: solo's draft hand is 4, and the player view cleaned up (r154)

Matt's rulings (Nov 25), now in [docs/rulings.md](../../../docs/rulings.md): **solo's draft hand is 4 cards** (the
locked 4: build 4, each card's tier chosen, no passing, then keep as usual; 323's build 7 / take 4 withdrawn), and
**the player view is the default** (*Play game* and any browser load open it; the debug view one button away; a saved
debug setting no longer wins).

## The solo draft

- `configs/solo.json` (derived by `scripts/configs/solo.py`): `cards.deal` 4 (the baseline's) and `cards.no_pass` 1,
  the param 278's table-time prototype already had: the hand you build is your draft, nothing picked, passed or
  discarded; the keep follows. Solo has no `cards.draft` step at all now.
- **`cards.take` is gone** (nothing else used it): its param, its arms in `draft_pick`, the draft phase's `begin`, the
  hand's label, and the agents' `take_best` (the eval's "only the best `take` count").
- **Rules revision 153 → 154** (bumped from the constant): solo games change. Every other config plays as before: the
  golden fingerprint went back to its pre-323 value (`3e1f9ab59d3fdb3d`), the param's removal its only change there.
- The tutorial (`web/src/tutorial.ts`) passes on its line at seed 24 with no re-pick: its *Take your cards* step
  folded into *Keep* (19 steps, was 20); *A round* lists the five steps the bar now shows.

## The judge read: solo d1 (fresh seeds 52534-52725, 192 a board, 576 games)

Spec `experiments/archive/farm/solo.json` (strategist, d1, a variant per board) on r154; kept at
`out/keep/326/solo-fresh/`. Reported, not dialed.

| Board | Games | Win % ± SE | Rounds mean | Min-max | 6-8 | 10+ | Rounds (games) | Losses |
|---|---|---|---|---|---|---|---|---|
| Builder | 192 | 90.1 ± 2.2 | 6.16 | 1-11 | 47% | 8% | 1:2 2:11 3:15 4:22 5:24 6:29 7:32 8:29 9:13 10:7 11:8 | Discord deck 10, event deck 7, research 2 |
| Protector | 192 | 97.9 ± 1.0 | 4.79 | 1-11 | 32% | 3% | 1:7 2:21 3:30 4:31 5:32 6:34 7:19 8:9 9:4 10:3 11:2 | event deck 2, Discord deck 2 |
| Researcher | 192 | 100.0 ± 0.0 | 4.94 | 1-10 | 30% | 1% | 1:3 2:12 3:11 4:48 5:57 6:33 7:15 8:9 9:3 10:1 | — |

Against 323's build-7 read (other fresh seeds, 52342-52533, so a comparison across seed sets): Builder 92.2 → 90.1,
Protector 99.5 → 97.9, Researcher 100 → 100; mean rounds 5.94 → 6.16, 4.76 → 4.79, 4.72 → 4.94. Seeing 4 cards a
round instead of 7 costs the Builder and Protector a point or two (within about 1 SE each) and lengthens the Builder's
games a little (47% in 6-8 rounds, was 38%). Solo still plays easier and shorter than the 2-player d1 game: 112 of
576 wins come in rounds 1-3 (323: 105). The Builder sits in the d1 band (80-95), the others above it.

`design_check.py` on the read: `warnings: design: usage 10 (Builder Fortify II 0.01 outside 0.05-2.50, Researcher
Hack I 0.11 outside 0.25-2.50, Make antimatter 0.02 outside 0.05-0.75, ...); design: rounds d1 36% in 6-8, 4% at 10+`
(323's: usage 11; rounds 30% in 6-8, 3% at 10+; the usage bands are the 2-player game's). On the tree: `warnings: none`.

## The player view

1. **The phase bar** shows the round's five steps, Adversary · Cards · Actions · Income · Threats, the current one
   lit (each phase by its rulebook step: A-B the adversary's, C cards, D actions, E income, F-G threats; `playerStep`
   in format.ts). The engine's sub-steps stay the debug view's; the adversary's turn panel names its steps the same way
   ("Adversary: Card beat", "Threats: cleanup") where it read "B lines", "G Cleanup".
2. **Shading:** the hand and buildings are shaded only on your own move in the action phase, a card you can't play
   (a building whose Action you can't use) with its reason on hover; never in the build or keep, nor while a question
   is open. A playable card is ringed then; otherwise neither.
3. **Captions** under the draft's cards say only what the printed card doesn't: an unmet need ("needs 2 diplomacy
   (you have 0)"), a keep price where no Keep button names it ("keep 3 insight (you have 1)"), and in a passing draft
   the keep price beside *Draft*. The "N credits to play" repeats are gone; *Keep (1 insight)* says the price.
4. **"Your move"** heads the right panel (else "Done for this phase", "Waiting", "Game over"); **no seat numbers
   anywhere in the player view**: a seat is its board's name ("Builder", its country's added when two share a board),
   solo's "you" in grammar ("You decide:", "strikes your research station", "you have no nodes left", a piece
   "(yours)"), in the prompt, its answer buttons, the seat rows, the turn panel, hovers, the threat and megaproject
   rows and the outcome line (`seatNames`, `seatText`, `setPlayerSeats` in format.ts). The draft's headings are words
   ("Keep which?"), not the engine's pile labels ("drafted (keep?)"). The engine's "C draft: Seat 0 (Builder)
   to act" line over the map is gone (the panel says it). **In the build and keep the draft takes the right panel's
   width**, *Done keeping* in its head (that panel only said "Pick from the cards over the map"); *Show the map*
   folds the draft to its head. In the action phase the panel is full and stays.
5. **The adversary's turn replay** sits right over the adversary cards it replays (next, this and last round), not
   atop the column. Under them, its ◀ ▶ moved as each step's cards changed height (`turn-nav-moved`); over them it
   holds, once the research row's marker border stopped adding 2 px.

The view: every load opens the player view (`aic.view` is no longer read or written); scripts start on the debug view
with `aic.startView` (cdp.mjs), as `aic.firstConfig` picks their config.

## Shots (1920×1080, solo seed 24, the Builder; `web/scripts/phase-shots.mjs`)

The same game and moves on main (r153, before) and this build (after): each tier a tier-1 card, every keep taken.
Before shows four moments, the draft's take among them; after has no take (the hand you build is the draft).

| Moment | Before (main, 944ad71b) | After |
|---|---|---|
| Build the hand | [build](shots/before-1920x1080-1-build.jpg) | [build](shots/after-1920x1080-1-build.jpg) |
| Take 4 of 7 | [draft](shots/before-1920x1080-2-draft.jpg) | (gone) |
| Keep | [keep](shots/before-1920x1080-3-keep.jpg) | [keep](shots/after-1920x1080-2-keep.jpg) |
| Action phase | [actions](shots/before-1920x1080-4-actions.jpg) | [actions](shots/after-1920x1080-3-actions.jpg) |

Before: 16 phase chips with "C draft" lit; "C draft: Seat 0 (Builder) to act" over the map; "What Seat 0 (Builder) can
do now" with "Pick from the cards over the map." and nothing else in a third of the width; seven cards each captioned
"N credits to play · keep 1 insight"; the hand shaded grey through the draft; the replay atop the column. After: five
steps with Cards lit; *Your draft* and *Done keeping* over the map and the panel's width; four cards at nearly twice
the size, a caption only for an unmet need, *Keep (1 insight)* on the button; the hand printed plain; the replay
over the adversary's cards; "Builder" and "Your move" where "Seat 0" was.

## Checks

- `tutorial-play.mjs` at 1366×768 and 1920×1080: 19 steps to the hand-off, every control. `playthrough.mjs` (1366×768,
  seed 1): solo lost in round 9 (the research track; the small policy, as in 323), three_seat lost in round 10 (a
  seat's last node): 0 failures, no rule unreached. It ticks five new rules: `fiveSteps`, `noSeatNumbers`,
  `draftUnshaded`, `yourMove`, `replayWithCards`. The screen checks' below-fold rule counts the open turn panel only
  (the closed Replay button now sits in the column's scroll).
- The panel's width also goes to the map for a question outside the action phase (a threat's "pay 3 energy or
  discard 3 cards?", its answers over the map) and at the game's end; in the action phase a question keeps the panel
  (its Undo, and the map doesn't move under the hex to pick).
