# 150: board tags and the Exascale keep (r64)

Matt's 2026-11-08 rulings on 149's r63. `RULES_REVISION` 63 → 64, bumped from the constant.

Scripts here: [specs150.py](specs150.py) (every screen and read spec; `experiments/configs/msb_deck.json`, the
baseline deck plus Market Stabilization Board), [keep150.py](keep150.py) (per card: games kept, played, kept and
never played, plays a game), [ladder150.py](ladder150.py) (the paired ladder read, `--msb` for item 3),
[gain150.py](gain150.py) (screen 1's trace). Screens pooled with 149's `pool149.py`. Reads: [f1.md](f1.md) ..
[f5.md](f5.md), [final_f3f5.md](final_f3f5.md), [final_rules_pooled.md](final_rules_pooled.md).

## Item 1: printed board tags count fully (one tag rule)

`cards::tag_counts` now counts the board's printed tags beside its revealed ones and its cards, so a printed tag
counts for requirements, the Industry and Science discounts, per-tag income and production, gains per tag and a
capstone's count. `cards::required_tags` (printed tags for requirements only) is gone; the eval's copy of it too.
A sacrifice still discards cards (`cards::sources`), so a board tag is never sacrificed.

**Give / get** (written before the first screen; `values.json`: a credit 1, a chip 2, insight 2.5, a tag 1 first,
0.5 more). A printed tag is free, so every line is a gain; none is a conversion.

| Board, tag | Give | Get |
|---|---|---|
| Builder, Industry | 0 (printed) | the tag (1); the Industry discount (1 credit a building per 2 Industry) one card sooner: about 0.5 a building played; Vertical Monopoly +3 credits a round while it stands; Full Capacity +0.5 chip a round (1); Economies of Scale +0.5 credit income; Antitrust Action +0.5 chip |
| Protector, Security | 0 (printed) | the tag (1); Homeland Security Act +1 credit income; Defense Appropriations +0.5 credit income (its requirements were already met by the printed tag at r63) |
| Researcher, Science | 0 (printed) | the tag (1); the Science discount (1 insight off tier-2 keeps and card plays per 2 Science, floor 1) one card sooner: about 1.25 a play (2.5 half the time); Foundation Model Campus +3 credits a round; Research Royalties +0.5 credit income; Cryo Compute Burn +1 insight |

The Researcher gains most: its discount reaches every card it plays.

## Item 2: Exascale Training Cluster's keep

**Before** (r63's binary, dev seeds 36800-36863, d0 and d2, the three pairs, the strategist, 384 games): kept in
68.8% of games, played in 2.1%, kept and never played in 66.7% (Builder + Protector 59.4 / 3.1, Builder + Researcher
60.9 / 3.1, Protector + Researcher 85.9 / 0.0). The prompt's 59-72% / 1-3% holds.

**Why** (a traced build, `AIC_DBG_CARD`, not kept): the keep plan values the card at `m × (18 + 7.5 × rounds left)`
table credits (6 cubes at 3; +3 insight a round at 2.5 over the adversary deck's 7-10 cards left; `m` the strategy
multiplier, 1.2-2.2) against a price of 10-23 (its energy at the spare rate, credits over the cap at the idle rate):
120-160 against 10-23 in rounds 1-4. The play eval, at the same tables, reads the play at −5 to −44 points against
holding the card (its hand worth is 2-14 of that), with the play legal and funded (40 credits, 16 energy). The two
price the card on different scales: the eval's research cube is 0.3 points (the table's 3 credits is 1.8), its
resources are concave (credits over `credit_cap`, energy over `energy_cap`, insight over `insight_cap` are cheap, so
a 15-credit, 5-energy, 2-insight play crosses them), its horizon is `rounds_left` with a survival discount, and it
has no strategy multiplier. Early (round 4) the eval reads the play near even (+3.6 before the hand term) and plays
other cards first; later it reads it a loss.

**Three levers on the dev seeds** (the same games, 384 a cell; the strategist; screens 1-3 in [specs150.py](specs150.py)):

| Lever | Exascale kept / played | d0 wins (bp, br, pr) | d2 wins | Kept? |
|---|---|---|---|---|
| none (r64's tags only) | 68.5 / 3.1 | 96.9, 85.9, 98.4 | 43.8, 39.1, 73.4 | base |
| `plan_eval`: keep a pick only if the eval's one-ply play gain is positive | 19.0 / 1.8 | 42.2, 34.4, 53.1 | 9.4, 9.4, 20.3 | no: the gain at the keep is negative for most cards played later ([gain_dev.md](gain_dev.md), [gain150.py](gain150.py)), so it drops good keeps; [plan_eval.patch](plan_eval.patch) |
| `plan_clock`: the plan's rounds left by the eval's clock (the deck cut short by the projected win) | 68.5 / 3.1 | identical | identical | no: the clock never cut the deck at a keep |
| `plan_spare`: under Patience with spare insight, an insight income at the idle rate, as the eval's horizon term prices it | 67.2 / 2.9 | 96.9, 85.9, 95.3 | 43.8, 39.1, 73.4 | **yes**: an exact mirror of the eval's rule (reviewed: units, condition, the subtracted term); about neutral |

**After:** still kept and not played. `plan_spare` closes one gap the review confirmed (and is in), but it fires
only once a Researcher's insight is already spare, which is after most Exascale keeps. Fresh-seed rates are below
(the final read). Its give / get stands as 148 priced it: at the anchors, give 15 + 4 × 4 insight + 5 energy 22.5 =
53.5, get +3 insight income 36 + 6 cubes 36 + tags 1.5 = 73.5 (1.37); at the table, give 42.5, get 38.25, a loss.
The play eval agrees with the table, not the anchors. A power change is Matt's call (not this item's). What would
make keep and play agree in general is one pricing for both (the eval's resource units in the plan, or the plan
calling the eval), an agents item of its own: the one-ply eval read at the keep (`plan_eval`) is not it.

**Fresh seeds, the baseline deck, d0-d3:** F1 (37912-38007, 1152 games): r63 kept 59.5% / played 3.4% / kept and
never played 56.2%; r64 58.9 / 2.3 / 56.6 (Protector + Researcher 79.7 / 1.0). F3-F5 (the final build, 3456 games):
r63 62.6 / 3.0 / 59.6, r64 62.2 / 2.7 / 59.5. Still unplayed.

## Item 3: Market Stabilization Board, in the deck against out

A second config on each read's seeds (`experiments/configs/msb_deck.json`: the baseline deck plus the card, at the
read's dials), paired on the same pair and seed against the baseline deck (the deal changes with the deck, so a pair
shares only its seed). It stays out of the deck. Win gap ±SE, and games where it was played:

| Read | d0 | d1 | d2 | d3 | played, % of games (d0 / d1 / d2 / d3) |
|---|---|---|---|---|---|
| F1 (r63's dials) | +1.7 ±2.1 | −6.9 ±3.4 | +1.4 ±3.5 | −1.0 ±3.6 | 47 / 36 / 32 / 32 |
| F2 (final d0-d2) | +0.3 ±2.2 | +3.1 ±2.9 | +0.3 ±3.6 | +5.6 ±3.5 | 52 / 40 / 40 / 28 |
| F3 (final) | +1.0 ±2.1 | +1.0 ±2.8 | −6.6 ±3.5 | −7.3 ±3.5 | 56 / 48 / 38 / 32 |
| F4 (final) | +0.7 ±1.9 | +2.1 ±3.1 | +0.3 ±3.4 | +4.2 ±3.5 | 50 / 41 / 41 / 29 |
| F5 (final) | −0.7 ±2.1 | +0.7 ±2.9 | +3.5 ±3.3 | −1.4 ±3.4 | 47 / 43 / 33 / 22 |
| **Final rules pooled** | **+0.6 ±0.9** (F1-F5) | **+1.7 ±1.5** (F1-F5) | **−0.6 ±1.7** (F2-F5) | **−1.5 ±2.0** (F3-F5) | 50 / 43 / 38 / 27 |

Kept in 48-70% of games, 0.27-0.50 plays a game. No gap clears 2 SE on any read or pooled: the card is neutral to
the win rate at every level. [final_rules_pooled.md](final_rules_pooled.md) has the pooled tables.

## The ladder (strategist, paired against r63's binary and configs on the same fresh seeds)

Each read is two 48-seed sets × three pairs, 288 games a level. Tables: [f1.md](f1.md) .. [f5.md](f5.md),
[final_f3f5.md](final_f3f5.md) (the final read, pooled), [final_rules_pooled.md](final_rules_pooled.md).

| Read | Dials | Seeds | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|---|
| F1 | r63's | 37912-38007 | 91.7 (r63 88.9) | 77.8 (73.3) | **53.8** (53.5) | 49.7 (49.3) |
| F2 | `counter_more_credits` 2, `build_bonus` 4 | 38008-38103 | **89.9** (88.2) | 75.3 (72.9) | 61.8 (54.9) | **51.0** (49.3) |
| F3 | `counter_more_credits` 2, `stage2_removed` 2 (final) | 38104-38199 | 91.7 (92.4) | 80.2 (81.9) | **65.6** (59.4) | 44.8 (54.2) |
| F4 | final | 38200-38295 | 93.8 (92.4) | 75.7 (76.4) | 61.5 (53.1) | **35.4** (48.3) |
| F5 | final | 38296-38391 | 92.7 (92.0) | 77.8 (79.5) | 55.2 (52.4) | **38.9** (48.3) |
| **F3-F5 pooled (the final read)** | final | 864 games a level | **92.7** ±0.9 | **77.9** ±1.4 | **60.8** ±1.7 | **39.7** ±1.7 |

Bold in a read row is outside its band. **Why the pools are sound:** `counter_more_credits` acts from
`counter_more_at` (2), and `stage2_removed` and `build_bonus` act only from d3 (each level includes the ones below,
none above), so d0 and d1 played the same rules on F1-F5 (1440 games: d0 **91.9** ±0.7, d1 77.4 ±1.1), d2 on F2-F5
(1152: 61.0 ±1.4), d3 only on F3-F5 (864: 39.7 ±1.7). All five reads ran one binary (the final code differs only in
comments, doc text and tests; F5's set a replays exactly on the final build, `same_results.py`).

**d3 is unmet** on the final read by 0.3 (0.2 SE), and on F4 and F5 alone. What bound it: the d3 dial.
`stage2_removed` 2 read −8.3 on the dev seeds but −9.4 to −12.8 against r63 on the fresh ones; no screened d3 dial
lands between it and F2's `build_bonus` 4 (51.0), and `build_bonus` is not monotone. Per Matt (message 6594), no
further dial. d2 sits mid-band pooled (F3 alone 65.6, 0.6 over).

**Dial screens** (dev seeds 36800-36863, d2 or d3, 192 games a cell, paired against the dialed base):
`counter_more_at` 2 → 3 (counters' second step from d3) +23.4 ±4.0 at d2, too far; `counter_more_credits` 3 → 2
+6.2 ±4.0 at d2, +4.2 ±3.8 at d3; 3 → 1 +8.9 / +1.6. With `counter_more_credits` 2, `build_bonus` 4 read d3 +0.5
±4.0 and `stage2_removed` 2 read −8.3 ±4.1; F2's d3 (51.0) took the second. `build_bonus` 5 on the F2 build read
+9.4 ±3.5 at d3 (larger builds made d3 easier: the dial isn't monotone).

## Engine smoke and speed

- `loop_smoke.py --run` (16 seeds, engine and strategist): no timeouts, top 2 plays of one card in a game.
- `play_read.py` (same run, over 6%): the strategist keeps nothing over 6.2% unplayed (Tidal Barrage, Joint Task
  Force 6.2%); the engine keeps Agency Directorate unplayed in 29% (engine only, as at r63).
- `npm run cards:fit`: no card changed; it passes.
- `per_decision.py` r63 → r64 (one thread; the box ran a sim beside it): the strategist 2.4 → 1.9 ms a decision
  (0.81×, 16 games). Greedy is heavy-tailed: 2.02× on seeds 1000-1015, 0.86× on 2000-2005, 0.55× on 3000-3015,
  1.11× pooled over the 38 games; `plan_spare` on / off reads 0.95× and the `same_position` reorder 1.01× on the same
  games, so neither is a cost. No slowdown kept; the `speed.py` baseline is not updated (no verdict across a
  revision).
