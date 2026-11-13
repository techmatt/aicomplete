# 210: settle r91 (Lift land corruption, re-reads, two new measures) (r92)

`RULES_REVISION` 91 → 92, bumped from the constant (the prompt named no revision). Built on main 1bd6e14 (207, r91).
Old side: r91 = bd08a51 (main plus this item's stats-only measures, which play the same games: 60 of 60 identical
on the dev seeds, `same_farm.py`) with the per-round land-token export, built in `out/r91src`. Every number is a
candidate. Tables: [screen-picks.md](screen-picks.md), [screen-lift.md](screen-lift.md),
[fresh-ladder.md](fresh-ladder.md), [fresh-measures.md](fresh-measures.md), [spend-fresh.md](spend-fresh.md),
[usage-fresh.md](usage-fresh.md); design checks [design-screen.txt](design-screen.txt),
[design-fresh.txt](design-fresh.txt). Readers: `read210.py`, `spend210.py`; specs: `spec.py`.

## What changed

| change | r91 | r92 | why |
|---|---|---|---|
| Lift land corruption (new) | — (Clear cut: no standard action removed corruption) | **`land_cubes` 2 research cubes**: 1 land-corruption token in reach, face up or down | Matt (candidate): the one standard action that removes corruption, "costly but not absurd" |
| Clear, Project: drain, Project: harden | cut (price params 0) | **deleted** with `clear_cost`, `drain_credits`, `drain_amount`, `harden_credits` | parked P2 |
| Make antimatter in economy.md / ai-boards.md | said 3 cubes | say 4 (`antimatter_cubes`) | parked P1 |

**Give / get at the anchors.** Lift: 2 cubes (12 at the anchors, 6 at values.json's 3) → a land token lifted, priced
12 (`extra_anchors.land_token`: a face-up token drips 1 Discord a round, about 3 rounds at a sway 4): 1.0, at par as
a standard action must be (locked: standard actions are hedges); the agents see 2.0. A face-down token (a corrupted
node hex's) drips 0 Discord, so lifting one buys only the hex back: under par by design.

Nothing places a node anywhere (locked): `node_sites` already requires a hex next to your node (with none left, your
home country); no card or power places elsewhere.

## Re-reads on r91 (dev seeds 36800-36831, 2,880 games, each lever paired against base)

| pick (206 screen 1) | the reverted lever | gap, levels pooled | by level d0-d4 | verdict |
|---|---|---|---|---|
| Station at 1 chip | 2 chips | **−8.3 ± 2.0** | −10.4, +4.2, −10.4, −8.3, −16.7 | holds |
| Claim / Liberate table 5 / 3 / 2 | factory 3 | **−2.9 ± 1.6** | −3.1, +4.2, −2.1, −5.2, −8.3 | holds |
| Harden project cut | back at 3 credits | **−0.6 ± 1.8** | −8.3, +1.0, +2.1, +5.2, −3.1 | holds (no gain from it; code deleted, P2) |
| Build a factory off | 3 chips | **+0.0 ± 0.0** | identical games | holds (still never taken: no free industrial zone in reach) |
| Stage-II Musters removed (9257) | r88's Muster 1 a country on Mobilization, Station Raids, Border War, Supply Lines, Forced March | **−8.1 ± 1.7** | −6.2, −2.1, −13.5, −8.3, −10.4 | stay removed |

The per-round limits (Place a node twice, Remove a troop once) are in this base. Annihilate is no longer in content
(r89), so the Muster re-read runs without it.

## Lift land corruption

Screen (dev seeds, r92): off, 1, 2 and 3 cubes read within ±0.2 of each other (paired, levels pooled). Uses a
seat-game: 1 cube 0.017, 2 cubes 0.011, 3 cubes 0.005. **Its band (hedge, 0.05-0.5) is out of reach by price**: on
the fresh read a seat-game sees 0.09 open-land corruptions (face-up tokens) and 0.37 node-hex ones (face down, no
Discord); the agents lift the face-up ones they can reach with cubes to spare (16% of them) and never a face-down one,
which costs a player nothing but the hex. Even at 1 cube the use is a third of the band's floor.

## Fresh read (45919-45999, 81 seeds × 3 pairs × d0-d4, r92 against r91 on the same seeds)

| d | r92 | r91 | gap ± SE (paired) | band | | rounds | land tokens a round r92 (r91) | Lift a seat-game |
|---|---|---|---|---|---|---|---|---|
| d0 | 88.9 ± 2.0 | 88.9 | +0.0 ± 0.0 | ≥ 90 | **unmet** | 7.81 | 0.297 (0.299) | 0.004 |
| d1 | 75.7 ± 2.8 | 75.7 | +0.0 ± 0.0 | 70-80 | met | 7.50 | 0.128 (0.136) | 0.019 |
| d2 | 57.2 ± 3.2 | 57.6 | −0.4 ± 0.4 | 55-65 | met | 7.07 | 0.125 (0.128) | 0.010 |
| d3 | 40.7 ± 3.2 | 40.7 | +0.0 ± 0.0 | 40-50 | met | 6.94 | 0.129 (0.132) | 0.008 |
| d4 | 23.0 ± 2.7 | 23.5 | −0.4 ± 0.4 | - | | 5.94 | 0.273 (0.294) | 0.031 |

Land tokens on the board a round, levels pooled: **0.189 against 0.196 (−4%)**. **d0 misses by 1.1 (0.55 SE)** and
r91 reads the same 88.9 on these seeds: it is r91's level, not this change. No dial is left: d0's `compute_income` and
`ambassador_income` are both 0 (206's d0 read 90.3 on 45631-45726). No dial moved.

## The two standing measures (fresh, levels pooled; `ladder.py --measures` prints them from now on)

| board | slack seat-rounds | bound seat-rounds | placements a seat-game | on/next to a factory / city / station | own factory / city / station | nodes at end | countries | groups | farthest (hexes) |
|---|---|---|---|---|---|---|---|---|---|
| Builder | 2.0% | 21.2% | 6.94 | 98% / 92% / 85% | 3% / 47% / 25% | 4.01 | 1.07 | 1.16 | 2.45 |
| Protector | 1.8% | 29.4% | 8.60 | 92% / 89% / 89% | 7% / 38% / 14% | 7.26 | 1.62 | 1.66 | 4.36 |
| Researcher | 4.8% | 21.4% | 8.89 | 85% / 84% / 92% | 3% / 35% / 20% | 4.07 | 1.30 | 1.44 | 2.76 |
| all | 2.9% | 24.1% | 8.14 | 91% / 88% / 89% | 4% / 40% / 19% | 5.11 | 1.33 | 1.42 | 3.19 |

- **Slack** (kept every card it drafted, held no card it could have played, insight left over) is rare: 2-5% of
  seat-rounds, beside insight binding in 21-29%. The two never meet in a seat-round (a short keep is never "kept all").
- **Placement:** almost every placement lands on or next to a factory, a city and a station (the map is dense: none
  of the three is 0%), so the "own" columns say more: 40% next to a city the seat holds, 19% its own station, 4% its
  own factory. Nodes stay bunched: 1.3 countries and 1.4 groups a seat at the end, at most 3.2 hexes apart (the
  Protector, with the most nodes, spreads most: 1.6 countries, 4.4 hexes).

## How each resource is spent (fresh, a seat-game; [spend-fresh.md](spend-fresh.md))

| resource | gained | spent | by kind |
|---|---|---|---|
| credits | 222 | 240 | cards 70%, standard actions 23% (Place a node 11%), powers 5% |
| insight | 34 | 29 | keeps 50%, plays 32%, powers 10% (Synthesis 9%), counters 5% |
| energy | 32 | 24 | powers 24%, cards 23%, standard 21% (Place a node's step), counters 16%, converters 8% |
| chips | 6.7 | 4.7 | Procure 37%, converters 31%, stations 24%, Fortify 6% |
| antimatter | 5.3 | 3.0 | Breakthrough 87%, cards 10% |
| research cubes | 12.6 | 8.7 | standard 36% (Make antimatter 20%, insight 16%), converters 33%, Peer Review 14%, counters 9%; Lift under 1% |
| ambassadors | 17.5 | 13.9 | Liberate 63%, Dispatch 22%, Remove agents 8% |

## Seeds and runs

Dev 36800-36831 (picks screen 2,880 games; Lift screen 1,440 + 480 + r91 base 480). Fresh 45919-45999 (1,215 a side,
native, 6 threads, about 2 min a side); ledger next free 46144. The farm's broker dropped on the first screen (its
shards ran one at a time), so every read ran natively. Kept: `out/keep/f210-s`, `out/keep/f210-r91-s`.
