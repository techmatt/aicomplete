# 321 bug hunt: deferred to Matt (D142 on)

Most important first. docs/qa-findings.md keeps the open list.

- **D142** (viewer, Pick) The adversary's turn can't be reopened once the round's first move is made: *Replay what
  ran since your last move* covers the last move only, so with one person playing every seat, a Tier click before
  stepping through the turn loses it (the debug view's log keeps it). Options: (a) keep the last adversary turn on
  its own *Replay the adversary's turn* button all round; (b) leave. **Pick (a)**: the turn is the round's news, and
  every seat's player wants it before their own moves.
- **D143** (viewer, Pick) *Pass every seat still to act* sits beside *Done keeping (pass)* in the keep phase: one
  click ends every seat's keeps, those of seats that never looked at their drafted cards too; Undo takes a press a
  seat. Options: (a) hide it in the keep phase; (b) a confirm; (c) leave. **Pick (a)**: a keep is a choice per seat,
  and a pass per seat costs one click.
- **D144** (viewer, Matt's call) At 1366×768 the map shrinks as the bar under the seat panel grows (standard actions,
  Liberate's line, converters): late in a game about 200 px tall, hexes about 22 px across
  (`cards-play` shot `arcology_foundry-1.png`). Options: (a) the seat panel at 32vh, not 38vh; (b) the bar on one
  scrolling row; (c) leave (1366×768 is the floor). **Pick:** look at it on the playthrough's screen first; (a) if it
  is the floor's size.
- **D145** (viewer, Matt's call) The draft takes about 7 clicks a seat a round (four tier picks, then the picks): 216
  of 460 clicks in a 10-round three-seat game (playthrough, seed 1). Options: (a) a *4 from tier N* shortcut that
  makes the four tier picks (no rule change); (b) leave. **Pick (a)** if the playthrough finds the draft slow.
- **D146** (cards, Matt's call) Agency Directorate pays its 2 Ambassadors when it is built: it is a Security card,
  and card-grammar's "a building counts itself" (its face doesn't say, as for the other trigger buildings). 317's
  give counts 3-4 later Security plays only (get 32-43, 1.37-1.83×); with its own play about 43-53 (1.83-2.28×).
  Options: (a) count it in the pricing note; (b) leave. **Pick (a)**: the note should price what it pays.
- **D147** (view, Pick) A discarded adversary project (Annihilation Lance, Zero-Day Wipe) reads "done" in the
  *resolved or discarded* pile and the projects panel's "done N", as if it completed. Options: (a) one word for both
  ("gone"); (b) leave. **Pick (a)**.
