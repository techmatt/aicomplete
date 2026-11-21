# 302: the antimatter capstones to the sink ruling (r146 → r147)

Matt's rulings of 2026-11-20 for this item: the four antimatter liberate capstones (The Nuclear Option, Hostile
Buyout, Root Access, Peacekeeping Mission) follow rulings.md's antimatter-sink line (2.0-2.5× get / pay); 301
priced them fair (1.0-1.3) on a planner answer that missed it. The d0 setup lever stays as built. Rules revision 146
to 147, bumped from the constant. [capstones.overlay](capstones.overlay) is the card pass.

## The four cards, old → new

Pay = credits + 18 a antimatter + the tier-2 keep 8 (no play insight on any of the four); get at the unit anchors
(card-guidelines.md), a liberation by card 20 (r146's bill). Each keeps its 1 antimatter.

| Card | r146 | r147 | Pay | Get | Get / pay |
|---|---|---|---|---|---|
| The Nuclear Option | 0 cr: free the stations / factories next to it, Rescue, sway 2 Discord | 0 cr: free them, Rescue, **+3 energy income** (the sway goes) | 26 → 26 | 25.7 → 61.8 | 0.99 → **2.38** (2.65 at two sites freed) |
| Hostile Buyout | 8 cr: liberate, made yours, gain 3 ambassadors | **0 cr**: the same, **gain 5 ambassadors** | 34 → 26 | 44.7 → 55.4 | 1.31 → **2.13** |
| Root Access | 2 cr: Rescue, remove a botnet next to it | **0 cr**: the same, **+3 insight income** | 28 → 26 | 28.7 → 52.7 | 1.02 → **2.03** |
| Peacekeeping Mission | 3 cr: remove 3 troops next to it, Rescue | 3 cr: the same, **place 3 ambassadors on it, sway 4 Unity** | 29 → 29 | 30.5 → 61.2 | 1.05 → **2.11** |

The gets: The Nuclear Option a liberation 20 + about one site next to a corrupted city 7 + the claim rider 6 (0.5 ×
a site owned 12, as design_check reads it; r146's note summed these less its Discord to 25.7, 2 short) + 3 × 9.6,
over the 2.5 ceiling only when two sites stand next to its city (2.65); Hostile Buyout r146's 44.7 + 2 × 5.33; Root Access r146's 28.7 + 3 × 8;
Peacekeeping Mission r146's 30.5 + 3 × (5.33 + a move 1.33) + 4 × 2.67. The agents' table (values.json, no change but
The Nuclear Option override's note) reads them 2.2-2.3 (`effect_ledger.py`: reasoned 56.2, 55.3, 56.7, 65.2 against
printed + keep 24.9-27.9), so each nets well above 0 for the seat that plays it.

**Why each.** Each adds what its engine's story gives after the liberation, not more liberation:

- **The Nuclear Option** (Power): the city's reactor is yours, +3 energy income. Its face held no fifth line
  (`cards:fit` 6.3 pt with one), so the 2 Discord sway goes for it: asked (questions/aicomplete__302_antimatter_capstones__1),
  Matt's message 13930: "The card's drama is now the cleanse around its city; the ruling wins over keeping the
  collateral line."
- **Hostile Buyout** (Diplomacy): the buyout brings its people, 5 ambassadors (was 3), and its 8 credits go. A
  fourth face line (+2 chip income, tried first) failed `cards:fit` (6.9 pt) and tripped design_check's tag story
  (chips are Industry's); ambassadors are Diplomacy's, priced at the anchor.
- **Root Access** (Cyber): root on the city's network, and its data is yours: +3 insight income (Cyber's
  `production:insight`), its 2 credits gone. A fifth line (remove 5 agents in its country) failed `cards:fit`.
- **Peacekeeping Mission** (Security): the peacekeepers stay (3 ambassadors placed on the freed city) and the
  peace holds (4 Sway toward Unity). It fits five lines.

None cuts antimatter; three cut credits (8, 2 and 0 to 0), where a face had no room for a stronger line.

**Played with no target.** An income line is not a map part, so it acts with no corrupted city in reach (feasible.rs:
a part with nothing to act on never blocks the rest, Matt's r48 rule): The Nuclear Option and Root Access can now be
played as an income card alone (+3 energy for 26, 1.1; +3 insight for 26, 0.9), Peacekeeping Mission as a 4 Sway
(0.4). A test (`a_kept_card_with_no_target_is_unofferable`) moves from Root Access to Forensic Audit for it.

## The fresh smoke read (two seats, the judge, seeds 51168-51263, 288 games a level, paired)

r146 is main at 05d7043f (`farm-seeds --at`), r147 this build. Tables: [fresh.md](fresh.md) (`make_tables.sh`).

| | d0 (at least 90) | d1 (80-95) | d2 (75-90) | d3 (55-70) |
|---|---|---|---|---|
| Wins r146 → r147 | 90 → **92%** | 88 → **86%** | 80 → **82%** | 69 → **67%** |
| In 6-8 / at 10+ (r147) | 54% / 14% | 50% / 18% | 47% / 18% | 50% / 18% |
| Won by round 3 | 3 → 3 | 2 → 2 | 1 → 2 | 2 → 3 |

Every level stays in band; each moves 2-3 points (SE about 2.5 a level), in both directions, so the change reads
as noise in win rate. The length bars miss as on main (r146's own 10+ shares here are 14-19%).

**Antimatter is not lifted.** Spent ÷ gained stays 35% (1.28 of 3.66 a seat a game at r147, 1.27 of 3.63 at r146,
the levels averaged; every level within 0.02 spent). The four are offered about as often (39-47 games in 100), kept about half of those
(17-25 in 100, the same at both revisions) and played almost every time kept (15-22 in 100, each within 1.3 of r146).
Doubling their value moved neither the keep nor the play: what binds the capstones is how often one is dealt to a seat
holding antimatter, not their payoff. A sink that pays 2.3× is a better play when it happens, not a more frequent
one. To lift the 38% (35% on these seeds at r146), the levers would be more antimatter sinks in the deck or the
agents' antimatter saving (`am_save`; card-pricing.md, *Rates known to be off*), each its own item.

Cities freed by card stay 134 a hundred games; The Nuclear Option and Root Access played no more often though they
now act with no target (the judge's keep and play didn't change for the income alone). `loop_smoke.py --run`: no
loop; `play_read.py`: nothing kept and never played (Annihilation Lance never kept, as on main).
