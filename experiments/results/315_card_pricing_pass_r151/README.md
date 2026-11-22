# 315: card and pricing pass before Matt's playthrough (r151 → r152)

`RULES_REVISION` 151 → 152 (bumped from the constant). The content edits are one overlay,
[cards315.overlay](cards315.overlay). Anchors: [card-guidelines.md](../../../docs/card-guidelines.md); pay = credits +
2.67 × (keep + play insight) + other costs at the anchors (keep 2.67 tier 1, 8 tier 2).

## The read: r152 against r151

The judge, `experiments/archive/farm/r315.json` (read.json at d0-d4: the three pairs, one seat each), fresh seeds
52066-52161 (96 a cell, two standing sets, 288 games a level), paired. r151 is `farm-seeds --at 6264f584` (main
plus the spec), r152 is 78aba285. Reader: `bench/ladder.py`.

| Level | r151 | r152 | Paired gap | Band (soft) |
|---|---|---|---|---|
| d0 | 96.2 | 97.2 | +1.0 ± 1.4 | 90-100, met |
| d1 | 94.8 | 94.8 | +0.0 ± 1.6 | 80-95, met |
| d2 | 92.4 | 94.1 | +1.7 ± 1.7 | 75-90, unmet (r151 already above it) |
| d3 | 83.7 | 85.8 | +2.1 ± 2.3 | 55-70, unmet (r151 already above it) |
| d4 | 55.2 | 52.8 | −2.4 ± 3.0 | none |

No level moved by more than about 1 SE. d3 measures: agents placed 54.2 → 51.6 a game, corruptions 10.7 → 10.1 (Fortify
I and Interdict I now take agents). Rounds and insight slack unchanged. Not dialed (bands soft).

## Screens (seeds 52162-52209, 48 a cell, 720 games an arm, paired on one base)

| Arm (`experiments/archive/farm/s315*.json`) | Pooled gap | What it showed |
|---|---|---|
| Troop powers widened ("on or next to", Fortify I + a hardening) | +0.3 ± 0.2 | use unchanged (Fortify I 0.11 → 0.12): troops in reach are the limit, not the price |
| Back-Channel Talks without its Unity rider, 7 cr | −1.1 ± 0.9 (with the other cleanse edits) | played 0.28 → 0.13 a game: the judge plays it for the Unity |
| Back-Channel Talks with the rider, 10 cr | −0.7 ± 0.9 | played 0.26 (kept) |
| Income cards (Cryptojacking +8, Multiyear +6) and Field Grant | −0.8 ± 0.9 | Cryptojacking played 0.065 → 0.097, Multiyear 0.051 → 0.076 |
| Fortify I on agents | +0.6 ± 0.9 | Fortify I 0.11 → 1.34 a seat-game |
| The candidate (Multiyear +7, Interdict I on agents too) | +1.1 ± 1.3 | Interdict I 0.21 → 1.46, Interdict II 0.76 → 0.26 |

## Each card and power: price against its anchor, use before and after (the fresh read)

| Card or power | r151 → r152 | Pay | Get at the anchors | Get / pay | Kept / played a game, r151 → r152 |
|---|---|---|---|---|---|
| Back-Channel Talks | 7 → 10 cr (rider kept) | 9.67 → 12.67 | a factory 9.33 + half a second 4.67 + 2 Unity 5.33 = 19.33 | 2.00 → 1.53 | 0.38 / 0.38 → 0.36 / 0.34 |
| Reclamation Crew | 8 → 9 cr | 10.67 → 11.67 | a station 15.33 (+3.33 an agent cleared) | 1.44 → 1.31 (1.60 with an agent) | 0.38 / 0.31 → 0.36 / 0.30 |
| The Nuclear Option | 0 → 2 cr, 1 am | 26 → 28 | 20 + a site next to it 12.3 (was 7) + claim 6 + 3 × 9.6 = 67.1 | 2.58 → 2.40 (sinks 2.0-2.5) | 0.13 / 0.13 → 0.12 / 0.13 |
| Great Liberation | 3 more sites (was 2); 21 cr, 1 am, 2 ins kept | 52.3 | 2 cities + 2 facilities 64.7; 4 cities 80 | 1.00-1.15 → 1.24-1.53 (sinks 2.0-2.5) | 0.10 / 0.05 → 0.11 / 0.06 |
| Cryptojacking Campaign | +5 → +8 credit income | 23.67 | 20 → 32 | 0.84 → 1.35 | 0.35 / 0.06 → 0.35 / 0.08 |
| Multiyear Defense Budget | +5 → +7 credit income, +1 energy | 25.67 | 29.6 → 37.6 | 1.15 → 1.46 | 0.23 / 0.04 → 0.23 / 0.05 |
| Field Grant | + draw 1, 1 insight to play | 8.67 → 11.33 | 12 → 15.33 | 1.38 → 1.35 | 0.34 / 0.24 → 0.32 / 0.23 |
| Fortify I (1 chip) | 1 troop → up to 1 troop and 2 agents | 4 | 3.33 → about 5 (1.5 agents a use, unmeasured) | 0.83 → about 1.25 | 0.13 → 1.33 a seat-game |
| Interdict I (1 Security) | 2 troops → 2 troops and 1 agent | a card | +3.33 | (sacrifice unpriced) | 0.25 → 1.28 a seat-game |
| Interdict II | unchanged | | | | 0.71 → 0.26 (in its 0.05-2.0) |
| Hack I | unchanged: in band (0.40) on this read | | | | 0.40 → 0.40 |
| Liberate a station | unchanged: design_check counted its 2 Unity twice | 15.99 | 20.67 → 15.33 | 1.29 → 0.96 | 5.98 → 5.87 Liberates a game |

Kept counts include cards drawn. `values.json` reads every changed card net positive for the seat that would play it
(effect_ledger: Cryptojacking reasoned 32.7 against 21 printed, Multiyear 38.6 / 23, Field Grant 18.7 / 8.7, Back-Channel
22.8 / 10, Reclamation 16.3 / 9, Nuclear 82.9 / 20, Great Liberation 80 / 44.3): no table fix. Faces: `cards:fit` 0 below
7.5 pt.

## Findings

- **Liberate a station's 1.29 was a table bug.** 312 raised `station_freed` and `factory_freed` by the 2 Unity a freed
  facility sways, and the Liberate rows also add that sway: counted twice. Both rows now mark it `priced_in`, as the
  city's row does; station 0.96, factory 0.58, as at r150. No price moved.
- **The troop answers are troop-limited.** Troops stand only at d2+ (0-1 a country), so a troop-only tier I goes unused
  at d0-d1 whatever its area or rider. Taking agents too puts both in band at every level.
- **Kept-not-played eased, not solved by much.** Both income cards clear `design_check`'s warning (played over a quarter of
  kept), but are still played in under a third of the games that keep them: the judge's income horizon is short.
- **The prompt's numbers had moved:** under the current judge at d0-d4, r151 reads Fortify I 0.13, Interdict I 0.25
  (in band), Hack I 0.40 (in band) and Move 1.37 (the prompt: 0.08-0.11, 0.15-0.24, 0.25, 1.27, d0-d3 reads).
- **Unprinted ops:** `adversary_cubes` and `botnet_cubes` went in 293; `counter_all`, `pushback`, `remove_botnets` and
  `one_player` are deleted: each acts with no target (every country, the whole board, any player), which the target
  rulings (*Card targets*, card-guidelines 21) refuse a card. `values.json`'s `pushback` row stays as `remove_where`'s
  agent rate.

## design_check (r152 against r151, 1,440 games a side)

`warnings: design: usage Make antimatter 0.03 outside 0.05-0.75; design: usage Liberate a site 98% outside 50%-95%;
design: rounds d4 50% in 6-8, 16% at 10+` (r151: the same three, plus the kept-not-played pair, Fortify I and Move).

## Resources

Spent of gained, r151 → r152: credits 107 → 107%, energy 79 → 79%, chips 53 → 57% (Fortify I's chip), research cubes
56 → 56%, ambassadors 73 → 72%, antimatter (d3-d4) 51%. Insight slack 10.7 → 10.6%, bound 18.6 → 18.7% of seat-rounds.

Outputs: `out/keep/315/` (kept at `C:/Code/aicomplete-control/.conductor/kept/aicomplete/315_card_pricing_pass_r151`): both read
sides (`r151.jsonl`, `r152.jsonl`, read as `FILE=experiments/farm/r315.json`), the three screens, design_check's output.
Seeds spent: 52066-52209 (the ledger's next free is 52210).
