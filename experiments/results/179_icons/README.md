# 179: the rulebook-friction icon rulings

Rules revision 77, unchanged: a face-only item. Card faces, the icon set, the map's agent piece and the docs that
teach them; no op, content or tunable moves, so no game plays differently (content hash `cfdd2bbc39ded04b`, as
before).

## What each ruling became

| # | Ruling (Matt) | Mark | Where it prints |
|---|---|---|---|
| 16 | node tracks print each space's cost as a cost | "pay N energy" first, in the cost style | the viewer's player board (`panels.ts`, `.m-cost`), docs/rules/ai-boards.md's space tables (`blocks.rs space()`), the rulebook's *Node tracks* table ("free", "pay N") |
| 33 | agents get their own figure, matching the map piece | `agent`: a figure in a brimmed hat, a collar cut into the coat | every agent on a card (27 cards), and the map's `ambassador:adversary` (board.ts), drawn the same |
| 34 | the minus is pay or lose; a distinct "less" mark | `less:N`: the number over a double chevron down | discounts (`card_discount`, `building_discount`), jams; an Action's costs left of `→` now print their minus |
| 35 | a wall for firewall and cordon; the clock a delay only | `wall`: the map's firewall bricks | Firewall Array, Air Gap, Hardpoint Lockdown |
| 36 | a track arrow in the below-the-art brown box | `raise` / `lower`: an arrow up or down over a track | every one-time income move (38 cards), no sign before the number |
| 37 | the shield is one stop, the roof a whole country | shield: one stop on one piece (hardening, a node shield); roof: cover over many (Shelter; with a node, Aegis Doctrine by amendment 8227, and Sanctuary) | `civic_shelter`, `corruption_shield` faces |
| 38 | At N gets a ring around the number | `at:N`: the number in a ring | the six threshold cards |
| 39 | onto one piece gets a destination mark | `onto`: a map pin after the move | Summit Caucus (Liaison Corps keeps the bare move: onto pieces) |
| 40 | the jam outside the dark frame | `less:N jam`, no frame | the 14 jam cards and the auto-counter |
| 8045 | each piece one symbol usable in a sentence | ambassador, agent, node, botnet, troop, drone (`drone`: the map's chevrons) | icon set and icon lines; `card.js` draws a `{piece}` marker in rules text as that icon inline (default (b): the generated text keeps its words) |

`icons.png` is the sheet (`node tools/card-art/render.mjs`), `agent-map.png` the map's agent at 5x.

## Fit

`npm run cards:fit` on main's faces and on these: every card fits (0 overflow), and every card's box sizes are the same
as main's. The first draft moved one card (Retained Earnings' top box, 6.3 pt to 5.4 pt: the ring and the track arrow
on one line); narrowing the track arrow (3.4 mm, 3 mm above the art) and the ring above the art put it back.

## Decisions

- **Aegis and Sanctuary print the roof.** The shield is one stop on one piece (a token); the roof covers many pieces.
  Aegis keeps any one of your nodes or facilities once a round, Sanctuary every node from corruption: both cover all
  your pieces, though Aegis still stops one a round. Matt's amendment 8227 ruled Aegis; Sanctuary (`corruption_shield`,
  the same shape) follows it. No player card prints either (they are capstones), so only the face code and the legend
  change.
- **The minus in the dark frame stays** ("remove up to N"): the adversary loses them, which is "lose" under #34.
- **Inline piece symbols:** default (b) of question 1 (unanswered): the renderer supports them, no generated face uses
  them yet. Converting card text to inline symbols is a follow-up, and costs width on every card.
- **No rules-history row:** no rules revision; the rows are one per revision.
