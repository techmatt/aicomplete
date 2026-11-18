# 273: smaller numbers, credits ÷ 1.5 (r128)

Matt's ruling (2026-11-18): "Smaller numbers: credit / compute ÷ ~1.5 where possible", on 271's pick: credits ÷ 1.5,
a card's cost floored at 2, compute left as is. 271 picked prices rounded up; **Matt's message 12506** (after the
first read, below) moved prices to the nearest whole number, as gains already were. `RULES_REVISION` 127 → 128. No
rule, effect or lever changed; nothing was repriced for balance.

## How it was done

[scale.py](scale.py) reads r127's files (`git show aea5535805:...`, so it reruns from the same base) and writes every
change: [scale.overlay](scale.overlay) (cards, AI boards, the map's megaprojects; applied with
`scripts/cards/apply_overlay.py`), the threat and project drafts (then `scripts/content/load_adversary.py`),
`configs/baseline.json` and `configs/agents/values.json` (text edits keep their hand layout), and
[scale_table.md](scale_table.md), **every number it changed, by source, r127 → r128** (480 rows). The nine
`content/alignment.json` lines (hand-laid; the overlay refuses the file) and the hand prose were edited by hand;
[scripts_scale.py](scripts_scale.py) scales the design checks' anchors and job rows.

**Rounding:** every number is the nearest whole number to x ÷ 1.5, never under 1; a card's cost is floored at 2 and
a card printed at 0-2 keeps its price. "1 credit per N tags" becomes 1 per round(1.5 N): per 2 → per 3 is exact.

## Every source (rows: scale_table.md)

| Source | What moved |
|---|---|
| Card costs (198 cards) | 3-38 → 2-25: 3 → 2, 4/5 → 3, 6 → 4, 7/8 → 5, 9 → 6, 10/11 → 7, 12 → 8 ... 34 → 23, 38 → 25. The 35 cards at 3 (271's 33 among them) go to 2 |
| Card credit gains, incomes, productions (54 numbers) | e.g. Orbital Mass Driver income 13 → 9, Gigafactory Charter 10 → 7, Citation Engine 7 → 5, Parts Exchange 5 → 3 |
| Per-tag credit rates (13 cards) | per 2 tags → per 3 (exact); "per tag" → per 2 (Continental Supergrid, Power to the People, Bounty Board, Homeland Security Act); 3 a tag → 2 (Foundation Model Campus, Vertical Monopoly) |
| Discounts and cash-outs | card discounts 5 → 3 (four cards), Liquidity Event's 5 a token → 3, Master Plan 2 → 1, `cards.industry_every` 2 → 3 (1 credit per 3 Industry tags) |
| Action credit costs | Patent Library 3 → 2, Bug Bounty Board 4 → 3, Treaty Isotope Bank 8 → 5 |
| AI boards | printed credits 15 / 13 / 11 → 10 / 9 / 7; track credit spaces 3 → 2, 2 → 1; Mint 3 → 2, Grant 3 → 2, Reclaim 6 → 4, Procure's gain 5 → 3; `ai.start_credits` 40 / 46 / 40 → 27 / 31 / 27 |
| Standard actions | Remove a troop 10 → 7, Gain an ambassador and Take down a botnet 20 → 13, Buy energy 5 → 3; Sell stays 1 |
| Threats (57) | counters 12 → 8 (tier 1), 17 → 11 (tier 2); Data Heist's payment 10 → 7 |
| Megaprojects (board-printed) | Tidal Grid 3 → 2, Polar Compute Array 9 → 6 and its converter 2 → 1, Mountain Fusion Works 3 → 2, Delta Seawall 4 → 3 |
| Unity / Discord cards | New Deal and Rebuilding Fund 2 → 1 a site; Propaganda's loss 2 → 1 a city |
| values.json (133 numbers) | every credit-valued number ÷ 1.5 (unrounded): resources (chips 2.67, energy 3, antimatter 12, insight 1.67, cubes 2, ambassadors 2.67), tags, tokens, rates (node, sway, drawn card, keeps, movement, rebate span, dead card), credit-valued op rows and overrides. Rows that are a share of a resource's rate stay |
| Agents (code) | points a credit × 1.5 (`credits` 0.9, `hand_cost`, `built_cost`, `table`, `tier_read`, credit income 1.5 a point); credit amounts ÷ 1.5 (`credit_cap` 8, `credit_idle`, `threat_harm_ref` 10, `plan_niche` 2, `credit_equivalent`'s rates, now f64, the keep plan's 1-credit floors, AutoCounter / ProjectDrain savings, Remove-botnet score); `keep_price` compares at r127's credits |
| Anchors and checks | card-guidelines.md's unit anchors ÷ 1.5 (agent 3.33, troop 3.33, energy 3, insight 2.67, card 3.33, ambassador 5.33, chip 4, insight income 8, compute 1.33, antimatter 12); design_check's anchors, card pay and job rows; payback.py reads `remove_troop`; `UNSPENT_HIGH` credits 12 → 8; `check.py`'s counter floor 5 → 3 |

The prompt's list: the **chip's worth** is values.json 4 → 2.67 and the anchor 6 → 4; the **tier-1 play cost per
point** of insight is the anchor 4 → 2.67 (the agents' `INSIGHT_COST` 2 → 1.33).

## Exceptions (what didn't divide cleanly, or where scaling would change a decision)

- **Kept at 1 (+50% against exact):** a sold card (`sell_credits`; sales +55% a round in the read), Grassroots
  Funding's +1 credit income a site, Hardened Bunker Complex's and Supply Chain Audit's +1 production, the discount
  floor (`discount_floor` 1: a discount stops at 1 credit, so discounted credits fell to 0.53× r127, not 0.67×).
- **2 → 1 (−25%):** track credit spaces of 2 (the Protector's first and last, every board's card-play space),
  Lights-Out Assembly, Supplier Network, Wafer Fab Line, Frontier Lab Charter and Counterinsurgency Campaign's +2,
  Polar's converter, New Deal, Rebuilding Fund, Propaganda. "1 per tag" → 1 per 2 tags (−25%) on four cards.
  Net, credit income is 0.65× r127 (exact 0.667×).
- **New 1-credit riders** (card-guidelines.md: no card prints 1 credit): New Deal's and Rebuilding Fund's 1 a site,
  Polar's converter "draw 1, 1 credit", Propaganda's 1 a city, and the 1-credit gains above. Counterintelligence Sweep
  already cost 1 at r127. Left for the balance pass (fold or raise), as the rule asks.
- **Jams are not scaled:** a jam takes points off every resource of a threat; the non-credit side is 3-5 and the jams
  5-6 clear it either way, so scaling them would change which counter a player pays.
- **Compute untouched** (the pick). `troops_removed` (counts) and values.json's share rows are not credits.
- **The bottom collides; no hand spacing.** Cards at 0-2 keep their price and the 35 at 3 join them at 2 (41 cards);
  4 and 5 → 3, 7 and 8 → 5. Two same-job pairs collide: Aegis Grid Lockdown and Hardpoint Lockdown (shelter events,
  4 / 5 → 3, different riders: nodes against a cordon) and Field Sensor Grid and Shadow Broker Feed (lift-Fallout
  programs, 3 / 2 → 2; Field Sensor Grid is one of 271's floor cards the balance pass reprices). Spacing either would
  move a price off the nearest, which message 12506 asks for; listed for the balance pass.
- Historical examples quoted in Matt's rulings (Peace Dividend, Threat Hunt, Eviction Worm), the drafts' per-threat
  `value` notes and `docs/card-pricing-notes.json` (each note stamped with its revision) keep their numbers;
  card-pricing.md says a note before r128 prints r127's credits.

## Read

Fresh, paired: `experiments/farm/f273-read.json`, judge and greedy, three pairs, d0-d4, 48 seeds a cell, seeds
**49788-49835** (taken in the ledger), both sides farmed from their commits (r127 cb16987; r128 afdb439, the nearest
build), 1,440 games a side ([paired.py](paired.py); every table: [read.md](read.md)). Per Matt's message the
nearest build is read on the **same seeds** as the rounded-up one: they are not fresh to it.

| Level | Judge win % r127 → r128 (gap ± SE) | Rounds | Greedy win % (gap ± SE) | Rounds |
|---|---|---|---|---|
| d0 | 92 → 90 (−2.1 ± 3.0) | 8.74 → 8.59 | 94 → 92 (−2.1 ± 2.7) | 8.51 → 8.21 |
| d1 | 65 → 64 (−1.4 ± 4.5) | 10.80 → 10.86 | 73 → 65 (−8.3 ± 4.7) | 10.73 → 11.03 |
| d2 | 60 → 49 (−11.8 ± 4.5) | 10.51 → 10.91 | 59 → 52 (−6.9 ± 5.0) | 10.76 → 10.97 |
| d3 | 25 → 31 (+6.2 ± 4.2) | 9.33 → 9.12 | 16 → 15 (−1.4 ± 3.8) | 9.72 → 9.82 |
| d4 | 8 → 10 (+2.1 ± 2.7) | 8.22 → 8.24 | 7 → 5 (−2.1 ± 2.3) | 8.47 → 8.11 |
| d0-d4 | 50 → 49 (**−1.4 ± 1.7**) | 9.52 → 9.55 | 50 → 46 (**−4.2 ± 1.7**) | 9.64 → 9.63 |

**Levels over 5 points off.** Judge d2 −11.8 and d3 +6.2 (opposite signs, pooled −1.4), greedy d1 −8.3 and d2 −6.9.
On a second seed set (271's spent 49740-49787, r128 as built against r127's kept read, a screen) the same levels
read judge d2 −2.8, d3 −2.1, greedy d1 −5.6, d2 +1.4, pooled judge −2.1 ± 1.6, greedy −1.5 ± 1.7: the level gaps
don't replicate (a level's SE is 4-5); **the level swings are noise.** What persists, about −2 points pooled, has two
measured drivers, both rounding: credit income nets 0.65× r127 against exact 0.667× (the 2 → 1 and per-tag rounding
above), and the 1-credit discount floor (discounted credits 0.53×); with `discount_floor` 0 greedy reads +0.3 ± 1.6
on 271's seeds (−1.5 with it), judge unchanged (−2.1).

**Prices rounded up (271's pick, the first build, farmed from 546f909)** read judge −4.2 ± 1.7 and
greedy −6.9 ± 1.8 on these seeds (d1-d2 −5.6 and −12 to −14): every price at x ≡ 2 (mod 3) rose a whole r127 credit
(54 cards; Buy energy +20%), plays a round fell 5-11%. On 271's seeds the same build with prices at nearest read
−0.6 ± 1.5 judge against −5.1 rounded up: the evidence behind message 12506.

## Resource use (judge, the r128 read, a seat-round, levels pooled; [resource_use.py](resource_use.py))

| Resource | Gained (r127) | Spent | Spent ÷ gained | Peak | Left at end | Spent on | Binds? |
|---|---|---|---|---|---|---|---|
| Credits | 18.9 (29.1) | 19.8 | 1.05 | 33 | 7.2 | plays 69%, standard actions 22%, counters 4%, powers 3% | yes: the starting stock is spent, under a round's income left |
| Insight | 6.1 (6.2) | 5.0 | 0.81 | - | lost each round | keeps 59%, plays 20%, powers 14% | yes: the keep's price |
| Energy | 5.5 (5.6) | 4.4 | 0.79 | 17 | 12.4 | powers 59%, Actions 20%, counters 11%, plays 8% | for some seats (energy-blocked Actions); the mean ends high |
| Chips | 1.0 (1.1) | 0.8 | 0.78 | 4 | 1.9 | converters 66%, powers 21%, stations 9% | lightly |
| Antimatter | 0.38 (0.38) | 0.19 | 0.50 | 2.4 | 1.8 | majors 58%, Actions 41% | no: half is never spent |
| Research cubes | 0.88 (0.91) | 0.69 | 0.78 | 3 | - | converters 46%, Actions 25%, powers 11% | lightly |
| Ambassadors | 1.9 (1.9) | 1.7 | 0.91 | 7 | 2.7 | standard actions 96% (Liberate, Remove agents) | yes |

Credits are 0.65 of r127's gained and spent and of the peak held; every other resource is within 5% of r127.

## Checks

design_check on the r128 read, `--old` r127 on the same seeds: new with this item: `dials-hide-design`
(industry_every 2 → 3 and city_energy_credits 5 → 3 beside the content rescale), `job-parity` Buy a city's energy
1.24 (min 1.25: at 3 credits it sits just under the line; exact 3.33 would pass), `kept-not-played` Forensic Audit
0.04 (unofferable, no target, 0.07 a game), `victory-skips` antimatter 74% (min 75%), usage Antimatter spent 50%;
as r127's: usage (Procure, Bulwark, Scrap, Make antimatter, Remove a troop, Gain an ambassador), rounds d1 10.9 and d2
10.9 (r127 10.8, 10.6), table-memory d0-d2 4.6-4.7. Loop smoke, cards:fit, play_read, speed: [checks.md](checks.md).

## For new players

Every credit number is two thirds of what it was: cards cost 0 to 25 credits (most 2-9), your board pays 7-10
credits a round and you start with 27 or 31, threats cost 8 or 11 credits to counter, and the standard actions cost
7 (remove a troop), 13 (an ambassador, a botnet) and 3 (a city's energy). A per-tag income program pays 1 credit per
3 tags. Nothing else changed: compute, energy, insight, chips and every card's effect read as before. What resisted
÷ 1.5: a sold card still pays 1, the 1-credit discount floor stays, and a few 2-credit gains became 1.
