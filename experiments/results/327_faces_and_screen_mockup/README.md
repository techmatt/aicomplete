# 327: printed faces and the solo screen mockup (r154, no rules change)

Matt's direction (Nov 25): show the printed component, with rules text on hover, as Terraforming Mars and Spirit
Island digital do, and no derived or internal numbers on screen. This item designs the faces the player view still
draws as text, and builds a static mockup of the solo screen from them for Matt to react to. The player view itself
is unchanged.

**See it:** `cd web && npm run cards`, then <http://localhost:5180/mockup.html> (1920 × 1080),
<http://localhost:5180/mockup.html?size=1366> (1366 × 768), and <http://localhost:5180/faces.html> (every face).
The card viewer's grid now prints boards, event cards, projects and threats too, and its header links to both pages.

## Shots

- [mockup-1920x1080.png](shots/mockup-1920x1080.png), [mockup-1366x768.png](shots/mockup-1366x768.png)
- [faces-board.png](shots/faces-board.png): the three boards and the research strip (bare, then with the marker on 3)
- [faces-event.jpg](shots/faces-event.jpg) (16), [faces-project.jpg](shots/faces-project.jpg) (30),
  [faces-threat.jpg](shots/faces-threat.jpg) (57)

## The faces

- **One home for icons.** `crates/aic-core/src/print_face.rs` turns content into icon lines. A player op (board
  powers, track spaces) goes through `face.rs` as a card's does. An adversary op gets one line in the dark frame
  (`adversary_line`: Infiltrate is agent, pin and site; Strike is troop and burst; Requisition is −N resource OR −N
  cards; and so on). `aic-sim docs` writes `web/card/print-faces.json`, and `docs --check` keeps it current. The
  words stay each component's hand-written `text`, carried for the hover.
- **The board** (`renderBoard`, a 180 × 110 mm steel panel, the staging default):
  - the name plate with the printed tags and income, and the special rule as one icon line (Breadth: 5 slots; Wall:
    troop, wall, node, beside Vigil's trigger; Patience: insight, carry);
  - the economy and card-play tracks as seven spaces each: the energy to cover a space at top left, and what it
    prints (an income step in the brown box, the insight total, a tag, the keep discount, the capstone's name). A
    node token sits on each space still covered;
  - the three node powers as boxes, each with its own used-marker socket. Each tiered power is one box with its
    tiers I and II as rows, sharing one socket, as the rules say: "a tiered power's tiers share one".
- **Adversary event card, project, threat:** a player frame recolored by kind (threats red, projects violet, event
  cards slate), drawn through `renderCard`, so `fitCards` sizes them.
  - *Event card:* the Card beat (threats, with their tier in roman) and Infiltrate above the window; Strike,
    Redeploy, flags and Escalate (printed on every card, per the spine ruling) below. The stage is in the octagon.
  - *Project:* the compute cost in the octagon, a compute-fill track in the window (filled in the mockup from the
    view's 9/14), and its effect icons below.
  - *Threat:* tier in the octagon, tag medallions; what it does unpaid in the dark top box; its sites in the window;
    the pay options joined by OR, then → counter.
  - A scene prints where `scenes.json` names one (War Drums, Blackout); elsewhere the window shows an emblem. Over a
    scene, a threat's sites or a project's compute track ride on a band at the window's foot.
  - Hover words are the content's own, with solo's resolved values: a "(level 1+)" gate is dropped, and the research
    row's loss slot says "slot 7", not K.
- **Research strip** (`renderResearch`): the six slot icons plus the loss slot, the marker on the slot reached, and
  in the mockup the cubes on the track (1 of 3).
- **New marks, one meaning each** (`icons.js`): coastal, escalate, strike, redeploy, spread, blackout (a city
  struck through), sabotage (the energy bolt struck through), embargo, free, corrupt, research_track, facedown,
  token, discord, recall, carry, again, deck, lose, and `tier:N`. The adversary's own income (Funding, Signal
  Jamming) is the raise arrow in its dark frame; the brown box stays the player's income.
  `free` (paying no credits) had been printed by `face.rs` for Scrap II and Field Station with no glyph behind it.
  No card drew it, so the fast lane never caught it.
- **Fit:** `npm run cards:fit` now fits the 103 adversary cards with the 213 player cards. All 316 sit at 7.5 pt,
  none overflows, and the three boards spill nothing (a new check). Titles under 15 pt: 11, all of them player
  cards as before.

## The mockup

- **A real state.** I saved a strategist game of `configs/solo.json` (Builder, seed 24, `aic-sim run ... --save`),
  replayed it in the dev server's engine to round 3, four actions into the action phase, and opened it there as the
  live game. `web/scripts/mockup-state.mjs` saves the view, the legal actions, the prompt and phase bar as the
  player view words them, and the map's SVG with its computed styles (`web/mockup/state.json`). The page reads that
  file and draws the rest from printed faces.
- **Layout.**
  - Top: the five-step phase bar (Actions lit).
  - Adversary strip: Last / This / Next (War Drums / Signal Jamming / Factory Drive), the three projects, the
    research strip and its cubes.
  - The map in the center, under one prompt line ("You decide: build Darknet Exchange where?" with its two answers).
  - Left column: the four resources (icon, amount, +income), then insight and ambassadors, the tags as an icon row,
    the win counter (three corrupted cities → 0), the Unity/Discord bar, and the four loss clocks as ring gauges.
    Each gauge shows a count on the table: cards left in the event deck (the arc out of the deck's 10), the
    research marker's slot, cards left in the Discord deck, and your nodes on the map (plain rings, red near the
    loss).
  - Right column: the threat in play.
  - Bottom: the Builder board (5 economy and 2 card-play spaces uncovered, Buy In's marker on), the hand of 7 fanned,
    and the building slots (1 built, 4 empty).
- **The 1366 version** keeps the layout at smaller card scales, with two resources a row.
- **Hover:** every card, board space, power box, slot, gauge and resource holds its words in `title`.

## Decisions to look at (staging defaults)

- **Faces print icons only.** The words live on hover, per Matt's direction. A printed edition would need the text
  in the rulebook or on the faces: this item doesn't decide that.
- **"The three node powers… only the tier owned":** the tree answered half of this. Every board has exactly three
  ordinary node powers, so those are the three boxes. No tier is owned: a tier is chosen each use, by its sacrifice.
  So each tiered power shows both tiers in one box with one shared marker. Showing only the owned tier needs a
  rule that doesn't exist.
- **Board size** 180 × 110 mm; **numbers** are solo's resolved values (Infiltrate prints no count: solo's is 1).
- **Map:** the client's current map is captured as is. It is not redrawn as a printed board.
- **The Protector's Wall** reuses the wall mark (no agent may arrive) with a troop: no troop may arrive or strike
  there. It is the nearest existing meaning, worth Matt's eye.

## Checks

`cargo test-all` and clippy are clean. `npm run cards:fit` passes. `npm run cards:check` passes but for *every deck
card names a scene*, which lists 17 player cards and fails the same way on main (this item touched no scene or
deck).
