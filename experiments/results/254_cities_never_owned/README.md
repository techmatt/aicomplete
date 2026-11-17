# 254: cities are never player-owned

Base: main 777f192 (253, r114). **Rules change: r115** (bumped from the constant, 114 to 115). Matt's candidate
(2026-11-16): the adversary is the AI that seizes cities, the player AIs free them. **Shipped on.**

Tables: [screen.md](screen.md) (the dev-seed screen of the rule and its replacements), [reach-screen.md](reach-screen.md)
(hosting on or next to the city), [read1-on-city.md](read1-on-city.md) (first fresh read, spent),
[read2.md](read2.md) (second fresh read, which found the Unity-card driver), [confirm.md](confirm.md) (confirm 1,
spent), [confirm2.md](confirm2.md) (confirm 2, the ship call), [design-check.txt](design-check.txt) (confirm 2's on arm against its off arm). Readers:
`read254.py` (253's, with city energy by route and the per-owned-site Unity payouts), `pooled254.py` (the paired gap
pooled over levels), `hosted254.py` (hosted cities paying by round). Specs: `experiments/farm/s254-screen.json`
(farmed at 3b8988f; its `liberate_energy` lever left the code after it), `s254-reach.json`, `f254-read.json`,
`f254-read2.json`, `f254-confirm.json`, `f254-confirm2.json`.

## What made or kept a city player-owned at r114, and what each does now

| Route | r114 | r115 (`board.cities_unowned` 1) |
|---|---|---|
| Setup's bound city (`board::bound_city`) | the clean city nearest each start, the seat's cube on it | gone; hosting replaces its energy |
| Claim a site, city case (`claim_city` 5) | a neutral city became yours | refused (`claim_cost`); Claim takes stations and factories; its design_check row and `city_claimed` anchor gone |
| Liberate (action or card) with `liberate_neutral` 0 (r112's rule, a screen arm) | the city became the liberator's | a city is left neutral whatever the switch |
| Hostile Buyout (`own`) | the liberated city became yours | targets any corrupted site; a station or factory it frees becomes yours, a city stays neutral |
| The Unity cards paying per site a player owns (9 cards, `per_site` `owned`) | counted the bound cities, most of their sites | count a hosted city (below) |
| City growth (`grow_cities`) | an aligned city's owner picked the hex of the facility it builds | the lowest-numbered hex, as a neutral city's (the owner's pick is dead code with the switch on) |
| Cards and powers | none claim a city (Power Purchase Agreement, Microgrid Lab Compact claim stations) | unchanged |

Converters (a node in a city of the country) read no ownership. Off (`cities_unowned` 0) is r114 except Hostile
Buyout's wider target, in every read's off arm: the ship call compares on with that off, not with r114 itself;
Hostile Buyout is played 0.15 a game on both arms, so the gap it could carry is small.

## Where owned-city income went (candidates, value-checked)

| Candidate | Value check | Fate |
|---|---|---|
| **Hosting** (`host_city` 1, `host_reach` 1): in the income step a neutral city with your node on or next to it makes its energy (1) for you; one city a player, one host a city (the players choose; the engine takes a largest matching), none if its energy was bought that round | **Where it matters:** every start spot is next to a city, so it pays from round 1 (0.49-0.58 a seat-round in round 1, 0.81-0.86 in rounds 2-5, confirm 2) as the bound city did. **Starting value:** the bound city's exactly (1 energy a round, 4.5 at values.json) where the start's neighbor city starts clean. **Give / get:** no price of its own; the node it rides on is already placed (or an Expand, the track step's energy) for 1 energy a round: over 5 income rounds at `energy_income_share` 0.8, 18 credits. **Caps:** one city a player, one host a city, the city's 1 energy. **Loops:** none. **Sim players:** 12.1-13.3 energy a game hosted (the bound city: 11.3-11.8) | **shipped** |
| Hosting on the city only (`host_reach` 0) | pays later (0.31 a seat-round in round 1: a node must be placed on the city first) and puts nodes where agents land | not adopted: read2 reach vs on-city +0.4 ± 1.7 (judge), +1.1 ± 2.0 (greedy) |
| **Hosted cities count as owned** for the per-owned-site Unity cards (Grassroots Funding and eight more) | the bound cities were 2 of each game's ~2.5 owned sites: without them those cards paid nearly nothing. Per-site payouts a game (confirm 2): off 4.32-4.60, on 4.74-5.09, of which hosted cities 3.67-3.81 | **shipped** |
| Liberating a city pays its liberator 2 energy (`liberate_energy`) | 2 energy (9) on a Liberate (40 at the anchors); 1.7 city liberations a seat-game, 3.4 energy | not adopted: dev screen -1.3 ± 2.3 alone; it adds income rather than moving it |

## The reads (paired; judge and greedy; three pairs; 48 seeds a cell)

| read | seeds | arm | judge d0-d3 | judge d0-d4 | greedy d0-d3 | greedy d0-d4 |
|---|---|---|---|---|---|---|
| dev screen (judge, d0-d3) | 36800-36831 | rule alone / host on the city | -4.7 ± 2.2 / -0.3 ± 2.2 | | | |
| fresh 1 | 48828-48875 | host on the city | -3.6 ± 1.7 | -2.6 ± 1.5 | -1.2 ± 2.2 | -1.8 ± 1.9 |
| fresh 2 | 48876-48923 | on the city / on or next to | -5.0 ± 1.8 / -5.0 ± 1.9 | -4.0 / -3.6 | -7.6 / -6.8 | -5.8 / -4.7 |
| confirm 1 | 48924-48971 | shipped, before the eval fix | +2.1 ± 1.7 | +1.8 ± 1.5 | +2.3 ± 2.0 | +2.1 ± 1.8 |
| **confirm 2** | **48972-49019** | **shipped** | **+0.7 ± 1.8** | **+0.1 ± 1.7** | **+1.4 ± 2.2** | **+1.8 ± 1.9** |

Confirm 2 by level, judge: d0 -3.5 ± 2.1, d1 +0.7, d2 -1.4, d3 +6.9 ± 5.1, d4 -2.1 ± 4.0; greedy: d0 -0.7, d1 +2.8,
d2 -6.2 ± 4.8, d3 +9.7 ± 5.1, d4 +3.5. No level is worse beyond 2 SE. Wins (judge, off → on): d0 97.9 → 94.4, d1 93.1
→ 93.8, d2 84.7 → 83.3, d3 47.9 → 54.9, d4 22.2 → 20.1.

What moved between reads: fresh 2 made a hosted city stay for sale (bought, it hosts no one); the Unity-card count is
the change between fresh 2 and confirm 1; the final diff review's eval fix (the agents' hosted term skips a city bought
from this round, as the engine does) between confirm 1 and 2. Each confirm ran on seeds nothing was picked on; confirm
1 is spent (the fix came after it).

## Ownership, Claim, Liberate, retakes (confirm 2, a game unless marked; off → on)

| | judge | greedy |
|---|---|---|
| stations and factories made the players' (claims, card builds, the Station action) | 2.36 → 2.33 | 2.52 → 2.48 |
| of them claims (station + factory) | 0.55 → 0.53 | 0.65 → 0.63 |
| Claim / Liberate a seat-game | 0.04 → 0.03 / 2.25 → 2.32 | 0.03 → 0.04 / 2.28 → 2.24 |
| retakes | 0.92 → 0.92 | 0.88 → 0.83 |
| cities liberated | 3.47 → 3.47 | 3.34 → 3.28 |

253's r114 (48780-48827): Claim 0.04, Liberate 2.24 a seat-game, owned sites 0.5 a game from claims, retakes 0.84.

## Income (confirm 2; spent / gained a seat-game, off → on)

Judge: credits 104% → 104% and energy 90% → 90% bind; insight 82% → 83% and ambassadors 81% → 80% bind; chips 67% →
66% bind loosely; research cubes 55% → 58% and antimatter 44% → 45% don't (antimatter's design_check band 50%).
**City energy (a game, judge / greedy):** aligned 11.3 / 11.8 → 0; hosted 0 → 12.1 / 13.3; bought 6.3 / 5.3 → 8.3 /
6.8 (Buy energy 3.15 → 4.15 a seat-game, judge). Hosted cities pay 0.49-0.58 a seat-round in round 1 and 0.81-0.86
in rounds 2-5; a seat-game with none: 1 of 720 (judge). **Unity cards paying per owned site:** 4.32 → 4.74 a game
(judge), 3.67 of them hosted cities; greedy 4.60 → 5.09.

## design_check (confirm 2: on against off)

Claim a city's job is gone; **job-no-card for Item C: Build a research station** alone. Standard-action flags:
Claim a site 0.04 (unused) on both arms; **Make antimatter 0.04 a seat-game (band from 0.05) is new on the on arm**,
at the band's edge and not flagged in confirm 1. Also new on the on arm: kept-not-played Exascale Training Cluster
(kept 0.44 a game, played 0.09; an energy-hungry card). Gone on the on arm: victory-skips antimatter (73%, min 75%).
Hostile Buyout plays 0.17 a game on both arms; `play_read.py` (confirm 1's on arm): no card kept and never played;
`loop_smoke.py --run`: no timeout, no card near the flag (most 5 plays); card fit holds.

## Speed

`per_decision.py`, r114 against r115, 8 games (c2, d2, seed 1000, one thread, best of 2; other rules, so ms a
decision; the box loaded, ratios only): greedy 0.80× a decision at 0.88× the options, the judge 0.74× at 0.76× (no
Claim a city to weigh). `speed.py check` gives no verdict across a revision.

## Seeds

48828-49019 taken in the ledger (fresh 1, fresh 2, confirm 1, confirm 2); next free 49020. Outputs kept:
`out/keep/254/` (fresh, fresh2, confirm, confirm2, screen, reach).
