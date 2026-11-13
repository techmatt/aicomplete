# 204: antimatter earned, and Unity and Discord by narrative (r90)

`RULES_REVISION` 89 → 90, bumped from the constant (the prompt said r90 too). Built on main accd80a (203, r89). Every
number is a candidate. Old side: r89 = accd80a's binary and configs (`out/r89`), same seeds. Tables: [tables.md](tables.md);
the sway pass: [sway-table.md](sway-table.md); design checks: [design-fresh.txt](design-fresh.txt).

## What changed

| change | r89 | r90 | why |
|---|---|---|---|
| Sway-for-antimatter action | off, code and `antimatter_discord`, `antimatter_uses` kept | **deleted** (and `alignment.act`) | no cheap source; amendment 9256 |
| Make antimatter (standard) | — | `antimatter_cubes` **4** research cubes for 1 | the desperate source; 4 not 3: 3 put 23% of supply on it (205's cheap-source cap is 20%), 4 puts 10% (screen 1) |
| Pair Forge (Builder power) | — | 1 chip + 1 energy → 1 antimatter (2 chips in every read before the confirm) | each board its own route; Researcher keeps Particle Run, Protector none but Sanctuary |
| Antimatter anchor | none (`values.json` 7) | **18** (card-guidelines, `values.json`) | amendment 9260: three cubes, the desperate price, at par |
| Annihilation Lance (tier 2, new) | — | 2 antimatter, 2 credits: every agent and troop in one hex, 2 troops in reach; **2 Discord** | the one big stack-clear, a card (9260) |
| Fusion Commons (tier 2, new) | — | 2 antimatter, 4 credits, Science 2: 6 energy, 3 cards, **3 Unity** | a constructive breakthrough |
| Containment Reactor (tier 1, new) | — | 7 credits; Action: 1 antimatter → 4 energy, 1 Unity | the tier-1 enabler, a repeatable sink |
| Breakthrough (standard) | — | 1 antimatter → `breakthrough_unity` **2** Unity, once a round | the spend the agents use (screen 4); Matt (9319): ship as a temporary candidate |
| Eval: held antimatter (`w.antimatter`) | 5 (200, saving for Annihilate) | **2** | at 5 the seats hoarded (9 made, 0.4 spent a seat-game at d0) |
| Sway pass | 44 untagged sources | every content sway tagged `narrative`; 6 moved (sway-table.md) | Matt's ruling; amendment 9196 |
| Unity strength III | Civil Defense, Town Hall Accord, Reconciliation Panel reach within 2 of any piece | stay by corrupted cities (4, 5, 3 agents) | amendment 9154: III no panacea |
| `compute_income` d1 | 4 | **8** | dial, last: d1 read 84.4 over 70-80 |

**Give / get at the anchors (antimatter 18):** Make antimatter 4 cubes (24) → 18: 0.75 (a standard action, a loss by
design). Pair Forge 1 chip + 1 energy (10.5) → 18: 1.7 (2 chips: 16.5 → 18, 1.09, and never used). A plant's Action
10.5 → 36: 3.4. Tidal Grid's converter the same. Containment Reactor: 11 once, then 18 → 22 a use (1.22). Fusion Commons
48 → 54 (1.13). Annihilation Lance 46 → a hex's stack + 10 − 8: 1.0 at 8 pieces in the hex, under it on a small stack,
flagged (a late, huge-stack card). Kinetic Interdiction with its new Discord 1: 10 → 8 credits, 34 → 36 (1.06). The
Nuclear Option at Discord 2: 41 → 37 (0.90, the price of an extreme act; flagged). Breakthrough 18 → 8 (0.44).

## Antimatter (fresh read, strategist, a seat-game)

| d | r89 gained / spent | r90 gained / spent | r90 won games spending it (r89) | desperate share |
|---|---|---|---|---|
| d0 | 6.00 / 0.18 | 5.29 / 2.69 | 91% (24%) | 10% |
| d1 | 5.96 / 0.25 | 5.25 / 2.88 | 94% (29%) | 10% |
| d2 | 3.46 / 0.12 | 3.26 / 1.96 | 89% (20%) | 10% |
| d3 | 3.01 / 0.10 | 2.47 / 1.55 | 84% (18%) | 7% |
| d4 | 1.61 / 0.06 | 1.77 / 1.09 | 78% (13%) | 10% |

Sources: Tidal Grid's converter 62%, upkeep (Positron Ring, Sanctuary, Open Archive) 13%, the desperate action 10%,
Particle Run 4%, the plants 8%: engines supply about 90%. Uses: Breakthrough 87%, Fusion Commons 4%, Containment
Reactor 3%, Annihilation Lance 2%, the ultimates 5%. Pair Forge 0 uses a game at 2 chips.

## Ablations (d3-d4, dev seeds, paired, 192 games a cell)

No chips −13.0 ± 3.2, no research cubes −7.8 ± 2.9, no antimatter −8.9 ± 2.6 (200's `no_spend`: sources stay).

## Ladder and rounds

**Confirm read (final build, 45535-45630, 96 a pair):** strategist 91.0 / 78.5 / 62.5 / 42.4 / 20.5 against r89
86.8 / 76.4 / 60.4 / 44.4 / 22.6 (+0.8 ± 1.5): every band met (d0 ≥ 90, d1 70-80, d2 55-65, d3 40-50). Rounds 7.50 /
7.77 / 6.82 / 6.30 / 5.76 (r89 7.80 / 7.81 / 6.91 / 6.32 / 5.63). Won games spending antimatter 92 / 95 / 88 / 89 /
80% (r89 19-31%). Greedy was read only on the first fresh read. Fresh read (45439-45534, before the d1 dial and Pair Forge's 1 chip): strategist
92.0 / 84.4 / 63.5 / 44.8 / 27.1 against r89 85.4 / 78.5 / 61.8 / 41.3 / 23.6 (+4.2 ± 1.5); greedy 89.6 / 84.7 /
74.3 / 57.6 / 33.3 against 84.0 / 74.3 / 66.0 / 47.2 / 25.0 (+8.6 ± 2.2). Rounds r90 7.54 / 7.55 / 6.68 / 6.27 / 5.80
against r89 7.92 / 8.10 / 6.64 / 6.55 / 5.92. Unity tips 6.8 / 8.0 / 7.3 / 6.2 / 5.1 (r89 6.4 / 8.1 / 7.0 / 6.6 /
5.1), Discord tips 1.1 / 1.5 / 2.5 / 2.8 / 2.9 (r89 1.1 / 1.5 / 2.3 / 2.8 / 3.1).

## Files

`apply_sway.py` (the tags and side moves, rerunnable), `spec.py` (every spec: `experiments/farm/s204-*`, `f204-*`,
`d204-d1`, `c204-strategist`), `am_read.py` (the antimatter and tips table), `tables.md`, `sway-table.md`,
`design-fresh.txt`. Seeds: dev 36800-36831; fresh 45439-45630 spent (ledger next free 45631).
