# 324: the player view reworked for solo (r153, no rules change)

The player view, laid out so a new player can learn the game alone in solo (323), on Matt's direction of Nov 25. No
play changes: the engine changes are view words only (a power's "needs tags" now names its tags and counts; a gone
adversary project reads "gone"). The multi-seat view runs on the same layout. Shots: [shots/](shots/), before (main at
fac66ec) and after, at 1366×768 and 1920×1080, from `web/scripts/playthrough.mjs` seed 1.

## What changed

| Direction | Now |
|---|---|
| The map is the center | Map in the middle column; at 1366×768 it is 739×387 px from round 1 to the end (it was 919×239-283 and shrank as the bar grew: D144). The foot is a fixed height (`--strip-h`), so nothing late in a game takes the map's room. |
| The board as a strip of numbers | A row per seat in the foot: resources with income, nodes, hand, each track's uncovered count. The board's name opens its whole face (a dialog: values, tracks, converters, powers' text). |
| Real card images | The hand and buildings print (`web/card/card.js`, shadow roots cached per card and scale, so a face is fitted once); the draft prints over the map as large as one row fits. The adversary's cards, threats and projects are drawn as the card viewer's plain faces in CSS (no kind but program, building and event has a printed frame yet). No scene art. |
| One "what you can do now" panel | Right column: the shown seat's legal actions only, by kind (cards, node powers, other powers, building Actions, standard actions with Liberate's line, converters, threat payments, *Other*), *Done (pass)* at its foot. A card, power or Action it can't use now is shaded, its reason atop the hover (`data-why`). A line atop names the node powers and building Actions not used this round, usable ones first. |
| An adversary panel always visible | Left column, first: this round's card, the next one face up with its Strike, Redeploy and Escalate notices, last round's on a line, the projects (the leftmost whole with its compute bar), the research row; then the threats, then the Alignment and loss clocks. |
| Replay of the whole adversary turn (D142) | *Replay the adversary's turn (round N)* atop the left column all round; it steps the turn that ran, the map and rows at each step's end, the last step the board as the turn left it. |
| A discarded project reads "gone" (D147) | The pile and its count read "gone" (engine status and label, panel word). |
| Undo inside the action phase | *Undo* and *Undo to the phase's start* (Ctrl+Z the first) in the panel, back to the action phase's start and never past an action that drew a card from a deck or reshuffled one (it would show the next card); every seat's actions in a multi-seat game. The header's unlimited Undo is the debug view's. |
| Baked numbers | Nothing in the solo view names a level (the playthrough checks the panel's text every step). |
| Debug and sim detail behind V | The piles and raw tracks left the player view; the debug view is unchanged. |
| D143 | *Pass every seat* is hidden in the keep phase. |

## Checks

- `playthrough.mjs` passes on solo and three_seat at both sizes: no hard screen issue, no console error, every rule
  tick seen. It now also ticks the printed cards, the shaded reasons, the reminder, Undo, the Replay button, "gone"
  and (solo) no level in the view, and once a round undoes a move and replays the adversary's turn (a solo game: 5
  undos, 1 undo to the phase's start, 7 replays; three-seat 8, 1 and 8). Soft looks (truncated turn-step lines) are
  as on main.
- Both games are main's to the action (solo: 154 actions, lost in round 9 to the research track; three-seat: 460,
  lost in round 10, seat 2's last node), heap the same (9 and 22 MB): the policy plays the same game through the new
  controls, undos included.
- The player-view sweep (`AIC_SWEEP_VIEW=player`, cases c1-d4 and c3-d0, 12 games): no failure, no legal action
  without a control; 653 truncated turn-step lines for a look (they hover whole), as on main. The other browser
  lanes (e2e, e2e:corps, e2e:sources, e2e:researcher, troops-play, ambassadors-play, player-play, odd) pass.

## Diff review

An adversarial review (four finders, six verifiers) found 12 items, 10 distinct; 10 fixed, 1 rejected, 1 partly:

- Fixed: a cached printed card detached before its first frame was reused unfitted (`fitHost`); the replay showed
  only the tail of a turn a mid-turn question split (it now runs the pass's frame and each answer's); a power's row
  had no rules text and its hover nothing (text under the name; `board:` cards hover); programs and events in play
  drew as chips (printed now); a seat that had passed read "not now: it is used in the action phase" (it says it
  passed); hover.ts still named the old panels; the README grew past its warning line (trimmed to 14.3 KB); the
  playthrough's `pastRows` could pass on a replay's last step alone (replays excluded), its Undo check could not fail
  (now: the move is legal again after Undo; once a game *Undo to the phase's start* lands on the phase's first step),
  and `shadedWhy` accepted any reason (it refuses "not now" in the action phase).
- Done anyway though the verifier rejected it as a scope choice: the unused-this-round line names every power, not
  only the node powers.
- Left: the threats, adversary cards and AI board are plain faces, not printed images (no frame exists for them).
- Seen on the board's face after the review: a power short of its sacrifice read a bare "needs tags"; it now names
  the tags to sacrifice.

## Decisions

- **Scene art stays out** (staging default): the scenes are 3.6 MB originals in the shared `art-originals/`.
- **Plain faces for the frameless kinds** (staging default): threats, adversary cards, projects and AI boards have no
  printed frame; the panel draws them as the card viewer's plain faces do (a dark strip, the name, the whole text).
- **Undo stops at a draw** (staging default) and, with several seats, takes back every seat's actions back to the
  phase's start (one person plays every seat).
- **D145 left** (staging default): the multi-seat draft keeps its clicks; solo's draw-and-keep is 323's.
- A fresh browser still opens the debug view on baseline.json: a new player presses V (*Player view*), picks
  `solo.json` and *New game*. Left for the tutorial item.
