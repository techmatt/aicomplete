# 321 bug hunt: fixed (F446 on)

r153, no rules change: every fix is the viewer's player view, a view status, a prompt's words or the checkers. The
removal prompt's words live in a pending question's state: a record or saved session stopped at that question (Homeland
Security Act's "remove agent from which hex?") replays with a fingerprint warning; a finished game is unchanged.

## Turn flow for one person playing every seat

- **F446** 320's turn panel: ◀ ▶ Done moved between steps (the step's label, its length varying, wrapped the head
  row: ▶ sat at 1229,81 on step 1 and 1258,99 on step 4 at 1366×768), so a second click on ▶ missed. The buttons hold
  one spot beside the fixed title, the label on its own line. playthrough.mjs fails on ▶ moving.
- **F447** After a seat's move the view went back to the first seat to act (seat 0): each of seat 1's moves took a
  click on its row first, and seat 0's buttons then stood where seat 1's had been. The view stays on the seat that
  acted while it has a move in the same phase (a new phase, or a draft pick round, opens on the first seat; the
  diff review moved the draft out). playthrough.mjs fails on the view leaving such a seat.
- **F448** ← → did nothing in the player view: they step the open turn panel, as ◀ ▶ do.
- **F449** A threat's pay buttons (in the country rows, far from the seat panel) belong to the shown seat but said
  only "pay 1 credit": the paying seat is named before them in its color.

## Hovers

- **F450** Every player-view control (Play, Use, pay, a choice's answer, the hex pick) hovered the action's raw JSON
  (`{"type":"pay","seat":0,...}`), over the printed card for a hand card's Play. The player view's hovers drop it (the
  debug view keeps it, W9). playthrough.mjs fails on a raw-JSON hover.
- **F451** The player view's hovers carried the debug view's internals: a hex's "hex 3,2 · index 18 · piece
  position 0,2 · 4 neighbors", a token's key, a value's key, scope and source, a card's id and pile, a track's and a
  board's keys. A `plain` flag drops them in the player view (W21 stands for the debug view).
- **F452** An answer naming a card (Annihilation Lance's "discard which adversary project?", a threat's "discard
  which card?") had no hover, and the projects sat below the side column's fold: it hovers the card's name, progress
  and text.

## Needs and statuses

- **F453** A hand card short of its tags said "needs tags": it says which and what the seat holds ("needs tags: 1
  diplomacy (you have 0)", `cards/view.rs` `needs_words`). Test `a_hand_card_short_of_its_tags_says_what_it_needs`.
- **F454** The draft and keep notes gave the play and keep price only: a card's unmet needs follow in the warning
  color ("needs 3 cyber (you have 0)"), so a 3-insight keep isn't spent blind.
- **F455** A hand card "ready · map part: no target" (it plays, its map lines do nothing: Arcology Foundry with no
  corrupted city in reach pays 6 credits, 2 energy and 1 insight for 3 chips) had its status in the ready style: it is
  a warning now, the tile still playable.
- **F456** A removal choosing hex by hex (Homeland Security Act's 4 agents) asked "remove agent from which hex?" with
  no count: it says "(N left to remove)".
- **F457** The bar's "Liberate refused" note stood beside the end banner: it goes once the game is over.

## The checkers

- **F458** playthrough.mjs gains the three checks above (raw-JSON hover, ▶ moving, the view leaving a seat with a
  move); `web/scripts/cards-play.mjs` (new) plays named cards in the player view in a rigged three-seat game and
  reports what each play and Action did.

## The hunt's own diff (independent review)

The review found three faults in round 1's fixes, all fixed: the debug view lost its JSON hover on its own hex pick
(the class test is now the `plain` flag); `plain` missed the track-step and board-name hovers; the stay-on-seat rule
made the draft's pick order 0,1,2,2,0,1 (a draft pick now hands back to the first seat). Random play clears the seat
the view stays on.
