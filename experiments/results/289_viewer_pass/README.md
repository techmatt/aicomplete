# 289: player-view layout pass

r138, no bump: no play changes. The new notes go through `cx.emit` (skipped in replay), status strings are view
only, and no option label changed (`golden_game` holds). Closes 285's D123, D127, D129 and D133 as deferred.md picks
them. The prompt's labels were shifted one place against deferred.md (its "D123" is D127's fold, its "D127" is
D129's outlines). Every pick of the four was done.

## What changed

| Finding | Pick | Done |
|---|---|---|
| D123 | blank the hand's play marks while a choice is pending | The engine has said "waiting on the question" since 285. The player view's hand tiles now show no status while a question is open; the printed cost stays. |
| D127 | smaller printed cards at 1366; the packet takes the buildings column's width | `printDraft` (`web/src/player/view.ts`) places the draft's printed cards at 0.8 and shrinks them (to 0.4 at least) until the lowest Draft / Keep button sits inside the seat panel's visible height: 0.61 (218 px) at 1366x768, 0.8 at 1920 and 2560. The draft and keep span the hand and buildings columns (`.p-body.drafting`), with the buildings under the powers. A resize redraws the player view. |
| D129 | one viewer item | **Outlines:** during a hex answer only the hexes to pick are marked, now purple (the accent blue matched Onyx's border). **Figures:** an unaffordable power reads "unaffordable: needs 4 energy (1 + 3 for the next track space); you have 2" (`ai::short_words`, `board::placements_energy`); another resource short reads "needs 2 credits (you have 0)". **War:** "War on the Horizon-Cintamani front: 3 Drones against 1 Drone: Horizon's 2 Drones cross to strike Cintamani's nearest factories; all are used up", "Cintamani has no factory left to strike", "War: no Drones at any front". **Drones:** "2 Drones arrive at the base at X". **After a loss** the turn panel opens at the step that ended the game. **The end screen's powers** read "game over", not "ready". **The packet heading's** "take one; the rest pass on" shows only beside Draft buttons. |
| D133 | with D129's viewer item | **Strikes:** "The adversary troop at X strikes Seat 1's node at Y and is used up", written before the protection choice. **Turn panel:** a protection answered in its own frame is a one-step turn (only a frame whose notes strike or destroy). The step's notes come first, then the step list (it scrolls, 5.5em at heights up to 820 px). The panel sits atop the side column while open. **Notes** name a node "Seat N's node" (`common::piece_words`); option labels keep "seat N's node", since records match answers by label. **Supply:** "agents go to the supply" was already 286's (D125). **Prompt:** "The adversary troops at X move to which hex?" went to D85's row. |

## Checks

- **The sweep's `below-fold` check** is a hard fail: a Draft or Keep button, or the open turn panel, outside its
  scroll pane's visible part or the window. It runs at all three sizes.
- **`AIC_SWEEP_SHOT_SIZE`** shoots, and checks in full, at 1920x1080 or 2560x1440.
- **Parked P1:** `design_check.json`'s `usage` rows take `levels`. Antimatter spent reads d3-d4 seats only.
  On 288's kept r138 read it gives 52% (d3 54%, d4 50%), inside 50-110%; pooled across all levels it gave about 49%.

- **Clipped progress chips:** with the full checks at 1920 for the first time, the sweep found the adversary and
  country progress chips (`.prog`) clipped 3-4 px: 3,329 captures, all `span.prog`, from before this item. They now
  take their height as their line height, with a block label.

- **Speed:** `speed.py check` gives no verdict (its baseline is r137). `speed.py pair` against main's build gives
  games a minute, same revision and same games, on a quiet box:
  - one thread: greedy c2 1.04×, c3 0.99×; engine c2 1.01×, c3 1.14×;
  - eight threads: 1.02× to 1.08×.

  No slowdown.

## Sweeps (player view, final build, one seed a case, AIC_SWEEP_SEED 100)

| Run | Cases | Captures | Hard fails | Look |
|---|---|---|---|---|
| 1366x768 | c2-d0, c4-d0, c1-d4, c3-d0, c4-d3-engine, c2-d2-strategist | 1,238 | 0 | truncated 4,269 (hover cards), board-text-overlap 10 |
| 1920x1080 (shot size) | c2-d0, c4-d3-engine | 418 | 0 (after the chip fix; 3,329 `clipped` before) | truncated 985 |
| 1366x768, after the chip fix | c2-d0 | 181 | 0 | truncated 805 |

Each run also checks the other two sizes for layout, `below-fold` included. The contact sheets' first pages are
`sheet-1366.png` and `sheet-1920.png`: the draft (#8), the keep (#16) and the turn panel (#56, #106) with every Draft
and Keep button and the whole panel visible. At 1366 the draft's printed cards come out at 218 px tall (scale 0.61);
at 1920 and 2560 they stay at 0.8.
