# 308 bug hunt: fixed (F424 on)

r149 → r150. Three fixes change play (F424-F426: one bump, bumped from the constant; the golden game reaches none, so
its fingerprint held and the content hash moved); the rest are view logic, notes, labels, faces, rules words and the
checker. Labels and notes moved, so a session saved in the viewer before this hunt may replay with a warning (the
record's revision).

## Play (r150)

- **F424** A Push (the Protector's Patrol I / II, Checkpoint Network: "Target: Agents or Troops. Push ...") took only
  hexes in node reach, though r142 puts a troop on or next to your ambassador in your reach for anything that targets
  troops (troops.md, *Guarding*; `target_hexes` already offered it, then `push_hexes` dropped it): the Patrol could
  never move the guard beside a city its ambassador stood on. `moves::pushable` adds such a hex and pushes troops only
  from it. Test `a_push_moves_a_troop_next_to_your_ambassador`.
- **F425** The same for a strike-gather (Kettle Operation): `gather_hexes` with `strike` takes the troop's hex, and
  around it the strike removes troops, and agents only where the seat's nodes reach (a diff review moved the limit
  from the whole strike to each hex). Test `a_strike_takes_a_troop_next_to_your_ambassador`. The rulebook (§16),
  ambassadors.md and the Push and strike prose say both.
- **F426** Cascade Failure ([station, city]: "A Botnet lands on the station here") dealt where its country has no
  station left put its botnet on the city token's hex: `place_botnet` read "here" through `token`'s fallback to the
  first token. It now reads the station's token only; with none it takes the nearest other piece, as on a fallback
  token (D132). Test `a_botnet_for_the_station_here_never_lands_on_the_city_token`.

## The view and notes

- **F427** "Strike ahead, in the next adversary Card beat" showed under "Next round: not revealed yet" while this
  round's own Strike was resolving (a target or protection question pauses the Card beat; the icons are still this
  round's card's). Inside the build phase the notices read "Strike in this adversary turn" / "Redeploy in this Card
  beat". (The first fix made `line_ahead` read next round's card from the build's start; the diff review found that
  hid a Redeploy still to come on the same card, so it was reverted for the words.)
- **F428** A troop's strike on a sheltered piece or one Aegis Doctrine stopped logged "strikes ..." and then nothing:
  `destroy_piece_named` now notes "... holds: it is sheltered" / "Aegis Doctrine stops it", as a spent protection does
  (D133's notes); every "holds" note is capitalized and names a facility's owner.
- **F429** A deck-out read "LOST in round 11" after a round line of "of 10 at most": the flip that finds no card is
  round 11's. The outcome now reads "LOST after round 10, the round cap" (the record's round is unchanged).
- **F430** In the cap's last round the Next round box said "not revealed yet": it says "none: the deck is out, lost at
  the next Card beat".
- **F431** The Redeploy notice, its hover, the build phase's text, the op's doc (docs/ops.md) and its value note gave
  the r141 rule ("each troop not on or next to a corrupted city"): each now says r144's (each troop onto its country's
  corrupted city nearest a player node or ambassador).
- **F432** "Strike ahead, in the next adversary Card beat" also showed for the research row's Strike slot, which
  fires in the Income beat: the notice says "the next adversary turn", the hover names both beats.
- **F433** The turn panel's Cleanup step read "-2 tipped ambassadors (seat 0)", as if two were lost: a tipped marker
  reads "2 ambassadors (seat 0) ready again" (or "used"), counted only where the hex's ambassadors stayed (one that
  moved, paid or was cancelled is in the ambassadors' own count).
- **F434** A Strike's target question named a node's owner but not a station's or factory's (seat 0 answers for the
  table): "seat 1's factory at ...", and the note too.
- **F435** The threat pay buttons read "+1 credit" beside a hand card's "Sell (+1 credit)": "pay 1 credit".
- **F436** Labels: "node on the board" and "node left on the economy track" (plural counts), "no Strike (round) 11"
  ("no Strike in round 11"), "Liberate refused 5" (debug chip: "guarded sites Liberate can't take"), a power's "no
  node left" beside the node-loss row ("no node token left on your tracks"); the Liberate line's "3 a city, 3 a
  station, 3 a factory ambassadors" ("3 ambassadors at any site"); a note's period before the "; " join; a Strike
  notice with no troop on the board says so.

## Faces and rules words

- **F437** The War Research row printed "Every 2 cubes a country (4 cubes with two countries, 6 with three)", r141's
  K; r144 made it 3 at levels 0-1 (the card's own level lines said so): it prints K by the level line.
- **F438** Closed City printed "3 Agents are created on the city here" and, with no city token, made them on its
  factory: "created here" (the rule: a piece lands on its token).
- **F439** The rulebook's Liberate line printed 5 / 4 / 3 ambassadors (pre-r145); 3 at any site.
- **F440** Siegebreaker Reactor (r149) also targets a guarded city and rescues only if its removal leaves no guard;
  troops.md, the rulebook, Peacekeeping's pricing note ("the only Rescue that takes a guarded city") and its
  player-cards row named Peacekeeping alone.
- **F441** threats.md said no located line acts on a fallback token; lines measured from it still measure (286).
- **F442** War Footing's flavor "Every unanswered strike" (Strike is a house word since r141): "threat".
- **F443** Neighborhood Watch's faces said only an Infiltrate is kept off; its token keeps off an Escalate's and a
  factory's push too. Rule of Law's token guards from the moment it lands: "Until the end of the next adversary turn".
  Offensive's "(country by country)" read as one threat a country: "(one hex: the lowest country first)", and the
  rulebook's *Picking a site* names its exception.
- **F444** The new-player summary said uncountered threats fill the Discord track (7 of 57 print a sway); economy.md
  priced Accord at 2 energy (3 since r144) and listed 15 antimatter tier-2 cards without the finale buildings (16).

## The checker

- **F445** `sweep.mjs`'s moments stopped at r141: `COUNTER_EVENTS_R142` (troop and Wall strikes, a troop destroyed by
  site, a Redeploy that stayed, a botnet taken down, Vigil, a card's liberation by site, the antimatter standard action,
  an ambassador shield, troops besieging, walled or sent by Discord or a project, a project discarded, research's
  Infiltrate, an event's Escalate, a Discord pull, each finale building's Action), checked against 1,152 r149 games;
  cases `c2-d0-strategist` and `c4-d4-strategist` (the judge at d0's setup agents and at d4's deck-out).
