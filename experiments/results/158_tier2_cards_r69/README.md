# 158: stronger tier-2 player cards for the end game (r69)

Matt's direction (2026-11-08, 2026-11-09): cards that **convert** adversary ambassadors, **gather** adversary and
allied pieces onto one place with a strike, and a **firewall**; no cascades, nothing global, nothing that frees a
city. Amendments 6947 / 6949: a converted ambassador goes onto **the node that cast the card** (not the city, not the
supply); a firewall stops every adversary ambassador arrival on its hex and lasts one round (the building places a
fresh one each round); a "with N X tags, also" threshold op is fine; d3 may take its own `ambassador_income` step.
`RULES_REVISION` 68 → 69, bumped from the constant (the prompt also said r69).

Sized from 157's gap readout (its final read, `157_strategist_purge_r68/gap.md`): d3's step-2 losses end with 2.5
cities left, the last ones fresh corruptions (0.3 adversary ambassadors on them, so the Liberate bill is the flat 5);
at the third-last round the team held 4.0 ambassadors against a 20.1 bill. One more late liberation in 16 of the 44
one-city losses would put d3 at the band's middle. So the cards aim at: ambassadors for the Liberate (Rally the
Allies, the thresholds), stopping the late corruptions (the firewall, gather + strike, converting the stacks around a
clean city before a Corrupt line pulls them in), and thinning a city's stack (convert).

Scripts: [specs158.py](specs158.py) (screens, card-off configs, reads), [rates158.py](rates158.py) (each new card's
offered / drafted / kept / played / Action uses a game, and plays in won games), [notes158.py](notes158.py) (the
pricing notes), [cards158.overlay](cards158.overlay) (the ten cards; `apply_overlay.py`).

## The rules (r69)

- **New ops** (`op.rs`; resolved in `effects/endgame.rs`): `convert { max }`, `gather { troops, strike }`,
  `firewall`, `if_tags { tag, count, then }`. House terms Convert, Gather at a city, Firewall (`HOUSE_TERMS`).
- **Gather takes a corrupted city only** (screened: a gather at any hex left its unstruck pieces next to clean
  cities, and city corruptions rose; s1, below).
- **The firewall** is a hex token (`firewall`): `adversary::place` skips its hex, `plan::rally_sites` and
  `plan::create_plan` skip it (the pool spreads over the other sites; a creation line takes the next site), and
  `plan::corrupt_plan` pulls nothing into it. Cleared in the adversary's reveal phase, after its income.
- **Unity on the one-shot events** (Matt's answer to question 1, message 7063: "add the Unity sway to the six
  events, and to similar cards in this set (ones that hit the adversary's pieces at a city) where it fits"): Mass
  Defection, Kettle Operation and Rally the Allies sway 2, Double Agents, Surgical Strike, Open Source Uprising and
  Air Gap 1. The two Actions (Embassy of Exiles, Lure Network) sway none: once a round it would be a Unity engine.
- **Player cards carry `draft`** (as 156's Unity cards); the card viewer's draft badge reads it
  (`web/scripts/cards-export.mjs`).

## Give / get (values.json; the keep, 2 insight = 5, on top of every pay)

New rows: `convert` 5 an ambassador (x `target_share` 0.7), `gather` 2 a move plus its strike at `remove_troop` or
`remove_ambassador` (x 0.7 a piece), `firewall` 8 a hex-round, `if_tags` 0.6 of its rider. Final cards (the
overlays [cards158.overlay](cards158.overlay), then [cards158_strong.overlay](cards158_strong.overlay) and
[cards158_sway.overlay](cards158_sway.overlay)); first-draft numbers in brackets.

| Card | Kind, tag | Price | Effect | Pay | Get | Where it matters |
|---|---|---|---|---|---|---|
| Mass Defection | event, Diplomacy | 8 cr, 1 ins | convert 5 around a city; sway 2; with 3 Diplomacy, gain 3 ambassadors | 10.5 | 32.7 (15.3) | rounds 7-10, a guarded city in reach |
| Embassy of Exiles | building, Diplomacy | 10 cr, 1 ins | gain 3 ambassadors; Action: convert 2 | 12.5 | 25.3 (15.1) | from round 5 |
| Double Agents | event, Security | 8 cr, 1 ins | convert 3; remove 2 troops in reach; sway 1 | 10.5 | 24.9 (14.0) | rounds 6-10 |
| Kettle Operation | event, Security (needs 2) | 18 cr, 2 ins | gather ambassadors and troops at a corrupted city, remove 6 there, troops first; sway 2 | 23.0 | 56.2 (32.8) | rounds 7-10, troops around a held city |
| Lure Network | building, Cyber | 12 cr, 1 ins | draw 2; Action: gather ambassadors at a corrupted city, remove 3 | 14.5 | 20.6 (17.9) | from round 5 |
| Firewall Array | building, Cyber+Power (needs 2 Cyber) | 15 cr, 1 ins | draw 2; Action: firewall a hex | 17.5 | 23.5 | from round 6 |
| Air Gap | event, Cyber | 6 cr, 1 ins | firewall a hex; remove 3 adversary ambassadors from a hex; sway 1 | 8.5 | 18.6 (12.4) | the round before a Corrupt cities card |
| Rally the Allies | event, Diplomacy | 14 cr, 1 ins | gain 5 ambassadors; move up to 3 onto one piece; sway 2; with 3 Diplomacy, gain 3 | 16.5 | 39.7 (25.3) | rounds 7-10, toward a Liberate |
| Surgical Strike | event, Security | 14 cr, 1 ins | remove 4 adversary ambassadors or 3 troops from a hex; sway 1; with 3 Security, a botnet in reach | 16.5 | 41.8 (26.8) | rounds 6-10 |
| Open Source Uprising | program, Science+Diplomacy | 8 cr, 1 ins | convert 4; draw 2; sway 1 | 10.5 | 27.5 (16.0) | rounds 6-10 |

Every card gains for the seat that uses it, keep included (get - pay - 5: +1.0 Firewall Array to +28.2 Kettle
Operation). **The final effects read cheap at the table** (get / pay 1.34-3.1; the ledger lists Kettle Operation,
Surgical Strike, Rally the Allies and Mass Defection beyond 2 SD of the deck's mean gap, -7.4 +- 8.2): the effects grew
after screening and prices stayed (a price is never lowered; raising one now would redo the read). Matt's note that
tier 2's insight keep is the price of power stands behind it; raising credits is the lever if he wants them dearer.
Plain routes (guideline 13): the Remove action evicts one adversary ambassador for one of yours from the supply; a
convert takes one and gives you one on the board. An ambassador standing on a site cancels one arrival; the firewall
stops every arrival on its hex for the round. Rally the Allies' 5-8 ambassadors and 2 Sway for 16.5 + 5.

**Printable:** `npm run cards:fit` fits every face with no overflow; eight new faces print below 7.5 pt (5.4-6.3:
Lure Network's top box; Mass Defection, Double Agents, Kettle Operation, Air Gap and Surgical Strike at 5.8; Rally
the Allies and Open Source Uprising at 6.3), recorded exceptions (guideline 9) beside the deck's ten earlier ones.

**Scene subjects** are drafted in `tools/card-art/scenes.json` (no generation): defectors at a checkpoint, an exiles'
embassy, a rally in a square, two double agents, a kettle of riot shields, a decoy server, a wall of glowing panels,
a cable pulled, a rooftop strike team, an open-air hackathon.

## Screens (strategist unless named; dev seeds 36800-, the three pairs; win %, paired against the same seeds)

| Screen | Cards | d2 on / off | d3 on / off | Note |
|---|---|---|---|---|
| s1, 32 seeds | first draft | 52.1 / 54.2 | 34.4 / 49.0 (-14.6, SE 6.1) | per card below |
| s2, seeds 36832-36879 | first draft; `strong` | 53.5 / 52.1; strong 61.8 | 43.1 / 44.4; strong 40.3 | |
| s3, greedy, 24 seeds | first draft; `strong` | | 61.1 / 70.8; strong 63.9 | |
| s4, 80 seeds | strong, gather at a corrupted city only | 60.0 / 52.9 (+7.1, SE 4.0) | 39.2 / 46.2 (-7.1, SE 3.9) | greedy d3 66.7 / 70.8 |
| s5, 80 seeds | + Unity sway on six events | 57.1 / 52.9 | 44.6 / 46.2 | greedy d3 66.7 / 70.8 |
| s6, 80 seeds, d0-d3 | final (+ Air Gap's sway) | 62.1 | 45.4; d3 `ambassador_income` 2: 27.5 (-17.9, SE 3.6) | d0 94.2, d1 79.6 |

**Why the first cards cost d3** (s4, d3, on against off): the same cities freed (4.05 / 4.31), but the liberate
capstones are drawn less (card liberations 1.54 / 2.05 a game: ten more tier-2 cards dilute the deck), and none of the
new cards swayed Unity, so the team reached 6.23 / 7.03 Unity tips (the purge needs 6). The 9-round d3 game has the
fewest rounds to make a late card pay. The Unity sway closed most of it (s5: 44.6 / 46.2). Greedy reads the cards
4 points under the deck without them at d3 on 24 seeds a pair (66.7 / 70.8, SE ~5.5: inside noise).

**Per card, on and off** (s1, the first draft, 96 games a cell: every paired gap is inside 2 SE; the all-off cell is
the only one outside it):

| Card off | d2 | d3 |
|---|---|---|
| (none) | 52.1 | 34.4 |
| Mass Defection | 51.0 | 31.2 |
| Embassy of Exiles | 49.0 | 45.8 |
| Double Agents | 50.0 | 39.6 |
| Kettle Operation | 53.1 | 43.8 |
| Lure Network | 52.1 | 40.6 |
| Firewall Array | 46.9 | 40.6 |
| Air Gap | 52.1 | 36.5 |
| Rally the Allies | 57.3 | 37.5 |
| Surgical Strike | 59.4 | 39.6 |
| Open Source Uprising | 53.1 | 39.6 |
| all ten | 54.2 | 49.0 |

## The final read (fresh seeds 39592-39735: three 48-seed sets, the three pairs, d0-d3, paired against r68's binary)

[final.md](final.md) (with the standing measures), [ends.md](ends.md), [endgame.md](endgame.md), [rates_final.md](rates_final.md),
[wins_cards.md](wins_cards.md), [oracle.md](oracle.md).

| Level | r69 | SE | r68, same seeds | Paired gap (SE) | Sets | Band | Verdict |
|---|---|---|---|---|---|---|---|
| d0 | 88.7 | 1.5 | 92.8 | -4.2 (1.9) | 86.1 / 89.6 / 90.3 | 90+ | unmet by 1.3 |
| d1 | 76.6 | 2.0 | 77.5 | -0.9 (2.6) | 73.6 / 79.2 / 77.1 | 70-80 | met |
| d2 | 61.1 | 2.3 | 61.6 | -0.5 (2.9) | 59.0 / 65.3 / 59.0 | 55-65 | met |
| d3 | 46.8 | 2.4 | 46.5 | +0.2 (2.9) | 42.4 / 53.5 / 44.4 | 40-50 | met |

The new cards don't move d1-d3 (every gap inside 1 SE) and cost d0 4.2 points (2.2 SE). At d3 the team frees
more cities (2.81 / 2.27 a game) and reaches more Unity tips (7.5 / 7.1), and loses less to the Discord deck (8 / 20
games); it loses more to the round cap (222 / 211).

**Oracle row** (155's `oracle`, the read's first 24 seeds a level, d3 and d4, the final build): d3 **100** (24 of 24;
the strategist on the same seeds 29.2), d4 **66.7** (16 of 24; 5 losses the Discord deck, 3 the round cap; the
strategist 0). 157's r68 oracle read d4 37.5 on its own seeds: the cards lift the perfect player's d4 by 29 points on
24 games (SE ~10), where they don't lift the strategist at all. The judge doesn't play them as well as they can be played.

## The end-game probe (every loss; the snapshot is the start of the third-last round)

Round-cap losses did not shrink: d0 49 / 31 (r69 / r68), d1 93 / 83, d2 117 / 100, d3 222 / 211. At d3 the step-2
round-cap losses (6+ Unity tips, cities only) are 145 / 133 games, with 2.8 / 2.4 corrupted cities left, 0.4 / 0.3
adversary ambassadors on them, and 4.6 / 4.0 team ambassadors at the snapshot against a Liberate bill of 19.0 / 20.1.
The cards add ambassadors (+0.6 at the snapshot) and liberations, but the adversary corrupts the cities back: the
cities standing at the end are again fresh corruptions with a flat bill.

**The new cards in wins** ([wins_cards.md](wins_cards.md)): every game, won or lost, played at least one; won games
played 5.7-7.9 of them (plays and Action uses) and lost games 6.7-10.0 (lost games run longer). Per card, the games
that played it won less than those that didn't at d0-d1 (a long game plays more cards); at d2 Kettle Operation (64.2
/ 57.3), Lure Network (75.0 / 56.5) and Firewall Array (83.3 / 56.7) sit in the winning games, the rest no higher.

## Dials

- **Kept from 157:** `compute_income` 3,4,10,8,10, `counter_bonus_credits` 4, `counter_more_credits` 0. On 80 dev seeds
  a level r69 read 94.2 / 79.6 / 62.1 / 45.4, all in band, so the final read ran at 157's dials.
- **d3's own ambassador step** (`ambassador_income` d3 1 → 2, amendment 6947's question 4): 27.5 against 45.4 on the
  same 240 dev games (-17.9, SE 3.6). Rejected: d3 stays with d2's step.
- **d0, after the final read** (the time gate: over 2 hours left; message 7059 kept the full read as the last full
  read): d0 read 88.7, under 90 by 1.3 (0.9 SE). Dev screen, 240 games each: `compute_income` d0 2 → 88.8, 1 → 89.2,
  5 → 92.5, base 94.2; `ambassador_income` d0 0 → 98.3. Taken: **`ambassador_income` 0,1,1,1,2**, re-read d0 alone on
  fresh seeds 39736-39879: **95.4** (SE 1.0; sets 94.4 / 96.5 / 95.1). At d0 a board's Rally pool starts empty and fills
  only from income its events and board raise during the game ("Income: +1 ambassador a board a round"); creation
  lines and threats still place ambassadors. d1-d3 don't read
  the d0 entry, so the final read stands for them.

## The final cards, on and off, and how the agents use them

**On and off** (s7: the final build, d3, dev seeds 36800-36831, 96 games a cell; [offs_final.md](offs_final.md)):
all ten on 47.9, all ten off 49.0. One card off at a time: Embassy of Exiles 37.5 (-10.4), Mass Defection 38.5
(-9.4), Air Gap 40.6 (-7.3), Rally the Allies 41.7 (-6.2), Firewall Array and Lure Network 43.8 (-4.1), Surgical
Strike 44.8, Double Agents 45.8, Open Source Uprising 46.9, Kettle Operation 49.0 (+1.0). Paired SE 4-6 a cell: only
Embassy of Exiles and Mass Defection reach 2 SE; the rest are inside noise.

**Rates a game** (final read, strategist, [rates_final.md](rates_final.md); greedy on the read's first 24 seeds a
level, [rates_greedy.md](rates_greedy.md)). At d3, strategist: each card offered 0.58-0.70 a game, kept 0.44-0.72,
played 0.09-0.68; greedy: kept 0.32-0.91, played 0.22-0.88.

| Card | Strategist d3 kept / played (Action uses) | Greedy kept / played (Action uses) |
|---|---|---|
| Mass Defection | 0.67 / 0.62 | 0.89 / 0.75 |
| Embassy of Exiles | 0.67 / 0.59 (0.81) | 0.77 / 0.57 (0.46) |
| Double Agents | 0.64 / 0.60 | 0.91 / 0.88 |
| Kettle Operation | 0.49 / 0.37 | 0.48 / 0.35 |
| Lure Network | 0.70 / 0.23 (0.30) | 0.89 / 0.72 (0.89) |
| Firewall Array | 0.44 / 0.09 (0.12) | 0.65 / 0.22 (0.42) |
| Air Gap | 0.68 / 0.61 | 0.73 / 0.51 |
| Rally the Allies | 0.72 / 0.65 | 0.84 / 0.83 |
| Surgical Strike | 0.71 / 0.68 | 0.79 / 0.73 |
| Open Source Uprising | 0.62 / 0.53 | 0.32 / 0.27 |

**Cards the agents keep and rarely play:**
- **Firewall Array** (strategist plays 0.09 of 0.44 kept): it needs 2 Cyber tags and a building slot in the last
  rounds, and the agents price a firewall as unseen worth (8 a hex-round: `Worth::unseen`), so a slot's other
  building usually wins.
- **Lure Network** under the strategist (0.23 of 0.70; greedy plays 0.72): the strategist's late schedule weights a
  building lower than an event, and its Action needs a corrupted city in reach with adversary pieces next to it.
- **Kettle Operation** is kept least (0.48): it needs 2 Security tags.

No new card reads as kept and never played in `play_read.py` (16-seed smoke) and `loop_smoke.py --run` was clean (no
timeouts, no card played more than 3 times a game).
