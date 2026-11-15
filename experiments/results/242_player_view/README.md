# 242 player view

A player view in the browser viewer (`web/src/player/`), beside the debug view, switched by *Player view* /
*Debug view* in the header (V). How it works and its checks: [web/README.md](../../../web/README.md), *Player view*.
No rules change (r109): the view reads the same engine view and legal actions as the debug view.

## Done-when

| Bar | Result |
|---|---|
| A full 3-seat game at d0 played in the player view, no debug view | [record.json](record.json): `configs/three_seat.json` (new: the baseline at 3 countries, difficulty 0, derived by `scripts/configs/three_seat.py`), seed 1, 1680 actions to round 13, lost (seat 0's last node). Played at 2560×1440 by `web/scripts/player-play.mjs`, clicking only the player view's own controls (buttons, map hexes, the turn stepper); `aic-sim replay` checks it ok. [summary.json](summary.json) counts the clicks: 156 tier, 156 draft, 121 keep, 33 play, 86 use, 142 sell, 128 hex picks, 219 adversary-turn steps. |
| Every legal action has a control on screen (`no-control` 0) | 0 in that game (counted every step, each seat to act shown in turn). The player-mode sweep (`AIC_SWEEP_VIEW=player`, cases c3-d0, c2-d0, c4-d0, c1-d4, one seed each, watch and play, 763 captures at three sizes): no `no-control`, no hard failure; 7583 `truncated` for
a look, all with a hover (hand tiles' names and text at 1366×768, the turn panel's lines). Its first run failed 2205
`clipped` (the adversary row's lines in the narrow column) and 91 `raw-id` (track keys in the turn panel): both fixed. |

The game is random play by the script (passing least), not a human's: it shows every decision of a whole game
reaches a control in the player view, not that the play is good.

## Shots (2560×1440)

- [r2-draft.png](r2-draft.png): the draft, the offered cards printed with their keep price.
- [r2-power.png](r2-power.png): the action phase, the hand as tiles with *Play* / *Sell*, the powers with *Use*.
- [r2-adversary-turn.png](r2-adversary-turn.png): the adversary's turn, a phase a line, the step's hexes outlined.
- [end.png](end.png): the end.

## Decisions

- **One seat's controls at a time.** The view shows the seat deciding, else the first to act; the other seats to act
  get a "Seat N to act ▸" button. So the sweep's `no-control` check, in the player view, shows each seat to act in
  turn and counts that seat's actions (every action has a control on its seat's screen). Drawing three seats'
  controls at once would be the information overload the item is against.
- **The adversary's turn** comes from one `apply`, so the engine gives no board between its phases: the panel steps
  through the phases' events (the rules' notes and the pieces added or taken, one line each) and outlines each step's
  hexes on the board as it stands after the turn. A per-phase state would need a wasm export that stops at each phase.
- **Printed cards** render in a shadow root (the card renderer's `.card` style clashes with the viewer's chips);
  `fitCards` measures inside a shadow root now. No scene art (3.6 MB each).
- **A fresh browser opens the debug view**, so the e2e, sweep and odd-input scripts keep working unchanged; the
  toggle is remembered.
- **Generic fallbacks:** an action no component claims becomes a button under *Other* in the bar; the seat's values,
  card groups, piles and raw tracks reuse the debug view's generic chips.
