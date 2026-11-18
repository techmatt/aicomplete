# 271: the draw economy (r127)

Draw is a precious resource (Matt's rulings, 2026-11-18): plays are bounded by cards, never by a play limit. One
overlay, [draw.overlay](draw.overlay), written by [make_overlay.py](make_overlay.py) from r126's `content/cards.json`
(rerun it, not edit the overlay), carries every content change. `RULES_REVISION` 126 → 127.

## Draw before / after

[draw_inventory.py](draw_inventory.py) counts every draw op in content (all 208 cards; [before](draw_before.md),
[after](draw_after.md)). A trigger or Action counts its draw once.

| | r126 | r127 |
|---|---|---|
| Cards that draw | 69 | 27 |
| Cards drawn (sum of counts) | 177 | 38 |
| By count: 1 / 2 / 3 / 4+ | 7 / 29 / 23 / 10 | 18 / 7 / 2 / 0 |
| By tag (a card under each of its tags): Science / Cyber / Diplomacy | 57 / 56 / 25 | 17 / 16 / 10 |
| Industry / Power / Security | 34 / 43 / 32 | 2 / 7 / 2 |
| Source, cards: on play / Action / trigger | 60 / 9 / 5 | 24 / 1 / 2 |
| Boards | Procure 1 chip → 2; Debrief, Field Survey, Synthesis 1 each | Procure 2 chips → 1 and 5 credits; the rest as was |
| Converters | Polar Compute Array 2 chips → 3; Delta Seawall 2 chips → 2 Sway and 1 | Polar 1 chip → 1 and 2 credits; Delta as was |
| Megaproject payouts | Polar and Delta 1 card a player cube (8 cubes each) | none (Polar 9 credits a cube, Delta 1 Sway and 4 credits) |

On the scale: **3** only Recursive Self-Improvement and Zero-Day Auction (was 6); **2** Breakthrough Leak (Science),
Foundation Model Campus, Data Licensing Deal, Envoy Briefings, Orbital Embassy, Shadow Broker Feed and the Cyber
engine's Action; **1** the rest; Industry (Materials Hedge Desk, Supplier Network) and Security (Recruit Operatives,
Headquarters Raid) keep two one-time draw 1s each, Power none of its own (its draws are on multi-tag cards:
Exascale Training Cluster, Fusion Commons, Orbital Embassy, Zero-Day Auction). No card
draws tier 2: the `draw` op has no tier (a tier-2 draw is a new op, left for the balance pass).

**The three engines** (the ledger can't price a repeatable draw: they are priced by hand against AI Central, 18-20
credits and a real requirement): Preprint Server (needs 4 Science: when you play a Science card, draw 1; 18 cr),
Command-and-Control Grid (needs 3 Cyber: Action, 1 energy: draw 2; 20 cr), Consulate Tower (needs 3 Diplomacy: when
another player plays a Diplomacy card, draw 1; 14 cr).

**Repeatable draws elsewhere became gains:** Orbital Observatory Array (1 energy → 3 cards, needs 1 Cyber: now 2
insight), Demand Response Grid (→ 6 credits), Patent Library (3 credits → 2 insight), Safehouse Network (2 tokens →
8 credits), Wafer Fab Line (2 energy → 2 chips, 2 credits), Positron Storage Ring and Bug Bounty Board (draw dropped,
drain 4 → 5), Synthetic Data Forge (draw dropped); Fusion Research Campus (Power play: draw 2 → 1 energy), Supplier
Network (Industry play: 2 credits), Reconciliation Office (agent removal: 3 credits), Agency Directorate (threshold:
2 insight, was 3 cards).

**Any-card payouts:** none left (`play` with no tag, `play_tag` with no tag, `play_cost`). Capital Projects Office
(+10 on any card at 10+) and Lights-Out Assembly (+5) pay 3 and 2 on an Industry play; Interdisciplinary Institute
counts Science tags; Agency Directorate, Conglomerate Headquarters and Command-and-Control Grid lost theirs (the
first two count Security / Industry plays). The engine forms stay (tests pin them on the r126 prints).

**The Builder's loop:** Mint (3 credits → chip) stays; Procure 2 chips → 1 card and 5 credits; Parts Exchange
1 chip → 5 credits (was → 3 cards). A credit → card now costs 6 credits of chips and pays 5 back once a round.

## Pricing: each card's value check

[ledger.md](ledger.md): every changed card's cost and the effect ledger's reasoned value (values.json), before and
after. Rule: a card's cost moves by its change in reasoned value, floor 3, so `cost - reasoned` holds where the floor
doesn't bind; the three engines are priced by hand. 33 changed cards land at 3 credits, all cut from more, most where
the floor binds on an old price far under its reasoned value (Forensic Audit, Fusion Commons, Field Sensor Grid). **Many tier-2 cards land
at 3 credits** (Capital Projects Office, Conglomerate Headquarters, Agency Directorate, Zero-Day Auction): the
ledger valued their any-card or big-draw text highly, so dropping it dropped their price. That is the balance pass's
job; the prices here keep every card's ledger gap, not its play rate. Watch first: Orbital Mass Driver 15 → 3 (+13
credits, +2 chips income), Vertical Monopoly 20 → 4, Capital Projects Office, Agency Directorate and Conglomerate
Headquarters 16-18 → 3. A card under 3 (Fusion Commons 0, Counterintelligence Sweep 1) keeps its price.

**Reuse doubles the Cyber engine:** Simulation Cluster (Reuse, now 3 credits) repeats Command-and-Control Grid's
Action for a second energy, 4 cards a round with both built, past "2 a round is very strong". Left for Matt (For
Matt, in the report): Reuse could skip a draw Action, or the engine draw 1.

**Give / get (conversions and Actions changed, at values.json: chips 4, energy 4.5, insight 2.5, card 4, antimatter
18, drain 1.1 a compute):** Procure 8 → 9 (card and 5 credits); Polar converter 4 → 6; Parts Exchange 4 → 5;
Command-and-Control Grid 4.5 → 8; Orbital Observatory Array 4.5 → 5; Demand Response Grid 4.5 → 6; Patent Library 3 →
5; Wafer Fab Line 9 → 10; Positron Storage Ring 9 → 18; Bug Bounty Board 4 → 5.5. Every one gains; each capped once a
round. Megaproject payouts keep their value (a card 4 → 4 credits).

## Plays and draws a seat a round (d2, 2 players, strategist)

Seed 268003 (268's game), traced: [before](trace_before.md) (r126, 6 rounds), [after](trace_after.md) (r127, 10
rounds). [trace_plays.py](trace_plays.py) counts plays, cards drawn off the decks outside the draft (round 1 includes
the opening's 4), building Actions and powers.

| | r126 plays / draws | r127 plays / draws |
|---|---|---|
| 268003, Builder | 5.5 / 7.0 (10 plays in round 5, 9 in 6) | 3.1 / 1.1 (2-3 a round; 9 in the last) |
| 268003, Protector | 1.7 / 2.7 | 2.0 / 1.4 |
| Dev seeds 36800-36811 (12 games), Builder | 4.8 / 5.8 | 2.9 / 1.0 |
| Same, Protector | 2.0 / 2.4 | 1.8 / 1.0 |

([12-game before](trace12_before.md), [after](trace12_after.md).) The Builder's late burst is gone; plays now sit
under TM's ~4, the Protector's unchanged.

## Fresh read

`experiments/farm/f271-read.json`: judge (strategist) and greedy, three pairs, d0-d4, 48 seeds a cell, seeds
49740-49787 (taken in the ledger), 1,440 games farmed (main box 16 shards native, a browser 8 in wasm). r126 (main
a4196fe) natively on the same seeds from `git archive` (its own configs), so the comparison is paired but changes
everything in this item at once. [read.md](read.md) is the full table (`read_table.py`; plays and drawn are the
games' counters per game-round over all seats).

| Level | Judge win % r126 → r127 | Greedy | Rounds (judge) | Plays / drawn a game-round (judge) |
|---|---|---|---|---|
| d0 | 97 → 92 | 96 → 95 | 8.3 → 8.3 | 14.6 / 9.0 → 10.4 / 2.1 |
| d1 | 90 → 79 | 87 → 76 | 9.1 → 10.3 | 14.3 / 9.0 → 10.1 / 2.1 |
| d2 | 84 → 65 | 87 → 58 | 9.0 → 10.4 | 13.8 / 8.5 → 9.7 / 1.9 |
| d3 | 44 → 18 | 47 → 23 | 8.7 → 9.5 | 12.3 / 7.7 → 8.9 / 1.8 |
| d4 | 30 → 10 | 27 → 3 | 8.1 → 8.3 | 12.5 / 7.4 → 8.4 / 1.7 |

(SE about 2-4 points a cell.) Draw falls about 4×, plays about 30%; the team is weaker everywhere above d0 (by 11-26
points), d1-d2 games run past 10 rounds, and at d3 the judge falls under greedy. Bands miss, as the prompt allows: the
balance pass follows. Its likely levers: the cheap prices at 3 (above), insight on cards that no longer draw, and
values.json's `drawn_card` at 4, which the agents still use while draw is now scarce.

**design_check (on r126's same seeds):** kept-not-played Open Research Campus and Forensic Audit (Forensic Audit's
draw cut); usage 7 (Procure 0.19 a seat-round, under its 0.40 band: its new price; Bulwark, Scrap, the hedges);
rounds d1 10.6, d2 10.5; **table-memory**: once-a-round uses a seat a round up 16-18% at d0-d2 (4.5-4.6 against the
4.5 budget): with fewer cards, seats use more board powers and building Actions. Fewer plays did not lower what a
seat tracks a round.

## Not done

- **Smaller numbers (~1.5×)**: deferred (Matt's question 1, default taken): a rescale of every credit cost, gain,
  income and compute number, with most number-pinning tests, is its own item. Proposed factors: credits ÷1.5 (card
  costs 3-46 → 2-31: the cheap cards' 3 / 4 / 5 / 6 become 2 / 3 / 3 / 4, so 2 and 3 collide at the bottom; ÷1.5
  holds only with a floor of 2 and rounding up), compute ÷1 (per-country project costs 7-24 are already small, and
  ÷1.5 breaks whole per-country scaling).
- **Liaison Corps' line split**: its face already prints two icon lines; the long text is the move op's generated
  wording ("from your supply or on or next to it onto pieces on or next to it"), shared with every move card: a
  `face.rs` wording change, not content. Left.
