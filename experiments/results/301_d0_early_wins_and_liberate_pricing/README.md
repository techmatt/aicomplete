# 301: d0's early wins and Liberate pricing (r145 → r146)

Matt's rulings of 2026-11-20 for this item: Liberate 3 stays, with `corrupt_at` 5 at every level; no seat-count lever
(three seats wait for his playtest); length stays his ruling (most games 6-8, 10-11 exceptional; running out the deck
a fair loss). Rules revision 145 to 146, bumped from the constant.

- Tables: [fresh.md](fresh.md) (r145 against r146 on fresh seeds, d0-d3, and d0's lever-off cell),
  [screens.md](screens.md) (four screens). `make_tables.sh` rewrites both (with 299's `hist.py`);
  [capstones.overlay](capstones.overlay) is the card pass.

## What changed

| Change | r145 | r146 | Why |
|---|---|---|---|
| `adversary.setup_city_agents` (new, a ladder list) | none | **4** at level 0, 0 from level 1 | d0's round-3 wins: the adversary takes a new city before the starting ones can be liberated |
| `board.claim_station` (Liberate a station) | 4 | **3** | the price table: a city (3) no cheaper than a station; one price, 3 ambassadors, at any site |
| `values.json` `liberate` (a city liberated by card) | 30 | **20** | the action's bill a card replaces: 3 ambassadors (16) + their move in (3 × 1.33) |
| The liberate cards' prices | | see below | each keeps the get / pay it was priced at against the new bill |
| The Nuclear Option | liberate | **also frees each corrupted station or factory next to its city** | Matt's message 13884 |
| Botnet Conscripts (parked P1) | troop on the botnet's hex | **the troop garrisons** | Matt, 297/1 |

## The fresh read (two seats, the judge, seeds 51072-51167, 288 games a level; r145 is main at a099a19d)

| d0 (bar) | r145 | r146, lever off (the pricing alone) | **r146** |
|---|---|---|---|
| Won by round 3 (about 1%: r144 3 of 288) | 15 of 288 (5.2%) | 18 (6.3%) | **4 (1.4%)** |
| Wins (band: at least 90) | 93% | 95% | **95%** |
| In rounds 6-8 (no lower than r145's 51%) | 47% | 51% | **54%** |
| Mean rounds / at 10+ | 6.5 / 11% | 6.4 / 8% | 6.8 / 10% |

| d1-d3, r145 → r146 | d1 | d2 | d3 |
|---|---|---|---|
| Wins (band) | 91 → 91% (80-95) | 82 → 84% (75-90) | 66 → 69% (55-70) |
| In 6-8 / at 10+ | 58 → 53% / 15 → 12% | 57 → 54% / 15 → 14% | 52 → 59% / 14 → 14% |
| Won by round 3 | 6 → 7 | 4 → 9 | 8 → 7 |

**d0:** the lever cuts round-3 wins from 18 to 4 of 288 on the priced build (15 at r145), keeps d0 at 95% and
raises its 6-8 share to 54%. 4 of 288 is about 1% (1.4%; r144's read had 3; SE about 2 games). The pricing alone
raises d0's early wins a little (15 → 18, within noise; screen 3 read 16 → 23). **d1-d3** take only the pricing
and P1 (the lever is 0 there): every win rate stays in band and moves 0-3 points; the 10+ bar (under ~10%) stays
missed at every level, as at r145 (299: cap losses set its floor). d2's round-3 wins 4 → 9 (r145's own read on
299's seeds had 6): unexplained beyond noise, watched. The liberate cards are played as often as before (each within
3 plays a hundred games; cities freed by card 122 → 132 a hundred games): the agents' lower value for a liberation
and the lower prices cancel. Tables: [fresh.md](fresh.md); capstone plays: `caps_read.py` on the kept reads.

## The d0 lever

Levers screened at d0 only (288 games a cell, 299's spent seeds 50784-50879, [screens.md](screens.md)): compute income
2, 3 or 5 at level 0 (no effect on round-3 wins: 16-18 of 288, a project needs rounds to buy), agents on the starting
corrupted cities (1-2 on r145: 16 → 9 and 6; 2-3 on the priced build: 23 → 11-13; an ambassador clears each), on the free cities (1-4) and a troop
a country at setup (15: it guards one city of two). Early wins are games where the adversary corrupts no new city by
round 3 (1 of r145's 16 corrupted one, against 1.2 a game in its later wins): at level 0 a city falls at 5 agents and the free one
(a country holds three cities, two corrupted at setup) needs three or four rounds of Infiltrate and Escalate. 4
agents on it at setup make it fall on its first arrival; 3 read the same within noise (5 of 288 against 4 on the
priced build). Not chosen: anything that changes level 1-3 (a level-0 setup line is the prompt's lever), the
city's corrupt number at level 0 (the ruling keeps `corrupt_at` 5).

A side effect: a city with an agent on it is blocked, so at level 0 the free city pays no energy until it is
cleared or falls.

## Liberate pricing

**The price table.** A station liberated at 3 ambassadors (was 4): with the city at 3 since r145, the station was
dearer than the city it serves, though a city is worth more (the win, 5 Sway, its Discord stopped) and a station a
flip (about 10). Every site now costs 3. The factory stays 3. Screened alone at d0 (screen 1, `st3`): 92% wins, 20
round-3 wins (r145 94%, 16).

**The capstones against the bill.** A liberation by card is now anchored on the Liberate action it replaces (3
ambassadors at 5.33, 16, and their move in, 3 at 1.33, 4: **20**; was 30 against 5 ambassadors). Each card's credits
come off so it keeps the get / pay it was priced at (pay = credits + 2.67 × (keep + play insight) + other costs at
the anchors; the tier-2 keep 8, tier 1 2.67):

| Card | Price r145 → r146 | Pay | Get | Get / pay r145 → r146 |
|---|---|---|---|---|
| Citywide Microgrid | 17 cr, 4 energy, 1 ins → **9 cr** | 31.7 | 37.3 | 1.19 → 1.18 |
| Power to the People | 15 cr, 4 energy → **7 cr** | 27 | 35.5 | 1.30 → 1.31 |
| Hostile Buyout | 15 cr, 1 am → **8 cr** | 34 | 44.7 | 1.33 → 1.31 |
| Root Access | 12 cr, 1 am → **2 cr** | 28 | 28.7 | 1.02 → 1.02 |
| Peacekeeping Mission | 13 cr, 1 am → **3 cr** | 29 | 30.5 | 1.04 → 1.05 |
| Forensic Audit | 2 cr, 4 cubes, 1 ins → **2 cubes** | 20.7 | 24 | 1.19 → 1.16 |
| Popular Uprising (tier 1) | 17 cr → **11 cr** | 13.7 | 14.1 | 1.00 → 1.03 |
| The Nuclear Option | 4 cr, 1 am → **0 cr**, frees the sites next to it | 26 | 25.7 (one site), 32.7 (two) | 0.96 → 0.99 (1.26) |
| Great Liberation (three sites) | 21 cr, 1 am (kept) | 52.3 | 60 | 1.72 → 1.15 |

The Nuclear Option could not reach 1.0 by price (its antimatter 18 and the keep 8 are over the new bill alone);
Matt's answer (message 13884): stronger, not dear. It frees each corrupted station or factory next to its city
before its Rescue, so the Rescue's claim may take one it freed; the table reads that line as six sites, so
`values.json` carries an override (-43) for the one site a corrupted city has next to it in play. Antimatter sinks
are ruled to start at 2.0-2.5× (rulings, *Antimatter*); Matt's answer asked for fair against the new bill, so the
four antimatter capstones read 1.0-1.3, as before (a question for Matt below).

## Content effects and notes

- Botnet Conscripts: its botnet comes off and its troop garrisons (a corrupted factory, else a corrupted city, else a
  city); rulings.md's troops line now reads as Matt's 297/1. No read moved at d0 (botnets don't spread there).
- `design_check.json`'s `city_liberated` anchor is 20, as values.json; The Nuclear Option's job row counts the sites
  it frees; its tag-story mismatch (Power printing a cleanse) is an exception with Matt's reason.
