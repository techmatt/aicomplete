# 227: corruption terms, Fallout and its sources (r99)

Matt's rulings of 2026-11-13. `RULES_REVISION` bumped from the constant, 98 to 99. Base: r98 on main (fa2bd44).
Readers: [read.py](read.py) (223's: win %, pairs, powers), [measures.py](measures.py) (Fallout a game by agent and
difficulty slice), [uses.py](uses.py) (per-card uses), [density_specs.py](density_specs.py) (the density screen's
overlays), [shards.sh](shards.sh) (a base side natively, 6 one-thread `farm` shards). Outputs: `out/keep/227_fallout/`.

## What was built

| # | Ruling | Built |
|---|---|---|
| 1 | Names (locked) | Land corruption is **Fallout** everywhere a player reads it (content, faces, rulebook, rules, viewer; [rename.py](rename.py)) and in code where cheap ([rename_ids.py](rename_ids.py): `FALLOUT` (token key `fallout`), `lift_fallout` op, `Removable::Fallout`, `place_fallout`). Seize is **Reclaim**; the threat op `seize_factory` is `corrupt_factory` ("corrupts the factory here"); the ledger's "facilities seized" is "facilities corrupted". Night Raid prints "Agents next to a node move onto it: 2 Agents destroy your node"; the rulebook's "Corrupt nodes line" is Night Raid's node line. Blight keeps its name. No card or power id changed (Reclaim's power has no id; powers key on board and order). |
| 2 | Recovery cut (locked) | The Protector has 6 powers. `lift_fallout`'s `unity` field went with it (its one user). |
| 3-4 | Fallout on all four sources | A new op, `fallout { count, at }`: each token on a free open hex (open land, no building, node or Fallout) on or next to its anchor, the lowest-numbered first; none free, it is lost. `at: location` (a threat's token) or `corrupted_city` / `corrupted_factory` / `corrupted_station` (a project's or Discord card's: in each country, next to its lowest-numbered `count` such sites). Valued at `values.json` `fallout` −12 (burn_blight's row). Tallies `fallout.placed[.source]`, `fallout.lifted`. |
| ledger | Done-when | `HARMS` row **Fallout placed** (`op_harms`: `fallout` and Corrupt open land; `RULE_HARMS`: Teardown). Printed counts: tier-1 threats 1, tier-2 2, Discord cards 1, projects 1 (events 2, your burns yes). |
| 4 | Project bundles | `content::project_rule_break` and `check.py`: a third effect only at cost 14 and up (`THIRD_EFFECT_COST`); unused at the shipped density. |

**The lines shipped (density lo3):** Call-Up (tier 1: 1 Fallout next to its token, for its Sway 2), Casus Belli
(tier 2: 2, for its Sway 3), Siege (tier 2: 1 on the destroyed factory's hex, for its 4 compute), Station Coup
(project, 9: plus 1 a country next to its lowest-numbered corrupted station), **Scorched Earth** (a new Discord card:
I 1 a country by its lowest-numbered corrupted city; II by each of its 2; III also by a corrupted factory).

## Give / get (threats priced uncountered at `values.json`; prices unchanged, the pricing rulings stand)

- Call-Up: counters 3 ambassadors (12) or 12 credits; uncountered troop 9 + Fallout 12 = 21 (was 17), 1.75× the cheapest counter.
- Casus Belli: counters 16 / 17; uncountered war step 8 + 2 Fallout 24 = 32 (was 20), 2.0×.
- Siege: counters 16 / 17; uncountered factory 15 + Fallout 12 = 27 (was 19.4), 1.69×.
- Station Coup: +12 × countries × target_share on its get (30); price kept at 9 a player.
- Each threat's uncountered harm rose above its counter, as the standard asks; none became cheaper to ignore.

## Density screen (46784-46815, spent; d1-d2, a game, all lines on and taken off by overlay)

| level | lines | placed | lifted | drip Discord | strategist gap | greedy gap |
|---|---|---|---|---|---|---|
| hi | + Agitators, Provocation (t1), Asset Seizure, Infiltration Drive (third effect) | 2.87 | 1.80 | 6.6 | +6.2 ± 2.8 | −1.8 ± 2.6 |
| mid | hi less Agitators, Asset Seizure | 2.53 | 1.64 | 5.8 | +4.2 ± 2.8 | −2.6 ± 2.7 |
| lo | Call-Up, Casus Belli 2, Siege, Infiltration Drive (third effect), Scorched Earth | 1.71 | 1.21 | 3.4 | +6.0 ± 2.7 | +0.3 ± 2.7 |
| **lo3 (shipped)** | Call-Up, Casus Belli 2, Siege, Station Coup, Scorched Earth | 1.55 | 0.99 | 3.8 | +5.2 ± 2.6 | −2.9 ± 2.6 |
| lo2 | lo3 with Casus Belli 1 | 1.53 | 1.01 | 3.6 | | |

lo3 kept tier 2 placing more than tier 1 (ruling 4) with lifts at the band's top; lo2 gained nothing. Base r98 on
these seeds: placed 0.26, lifted 0.13. Full table: `out/keep/227_fallout/screen-measures.txt`.

## Fresh read (46880-46911, 32 seeds × 3 pairs × d0-d3, 384 games an agent; r98 natively on the same seeds)

| agent | d0 | d1 | d2 | d3 | all, gap ± SE | rounds base → new |
|---|---|---|---|---|---|---|
| greedy r98 | 82.3 | 66.7 | 53.1 | 41.7 | | 9.10 |
| greedy r99 | 81.2 | 65.6 | 45.8 | 40.6 | −2.6 ± 2.8 | 9.29 |
| strategist r98 | 90.6 | 67.7 | 47.9 | 52.1 | | 8.95 |
| strategist r99 | 81.2 | 68.8 | 49.0 | 39.6 | −4.9 ± 2.7 | 9.08 |

By pair (gap ± SE): greedy bp +3.1 ± 5.2, br +3.1 ± 4.6, **pr −14.1 ± 4.8**; strategist bp −9.4 ± 4.9, br −3.1 ±
4.6, pr −2.3 ± 4.4. Largest cells: strategist d3 −12.5 ± 5.0, d0 −9.4 ± 4.7. The screen on 46784-46815 read the
strategist +5.2 ± 2.6 on the same build: the two reads disagree by ~2.7 SE, so the change's effect on win rate is
small and not settled at this size (pooled, about 0). The fresh seeds alone are the read. Mixed native and wasm
shards (farm-seeds); native = wasm is a core invariant.

**Fallout (fresh, d1-d2, a game, greedy / strategist):** placed 1.39 / 1.46 (base 0.15 / 0.21); lifted 0.91 / 0.91
(base 0.10 / 0.11): **in the 0.5-1 band** on lifts (the default reading; Matt's question 1 went unanswered). Sources
(strategist): tier-1 threats 0.41, tier-2 0.20, projects 0.47, Discord 0.28, events 0.07, burns 0.03. Drip Discord
2.9 / 3.4 a game (0.1 at base); Discord tips 2.83 / 2.81 (2.66 / 2.86): the drip replaced the Sway Call-Up and Casus
Belli printed, about one for one.

**What came back into use (all games, a game, base → new):** Lift Fallout 0.03 → 0.52; cards' lifts 0.06 → 0.31
(Network Cleanup's 0.02 → 0.08); the green lift cards' plays unchanged (Field Sensor Grid 0.91 → 0.89, Shared Grid
Accord 0.97 → 0.98, Open-Weights Release 0.25 → 0.25, Fusion Startup Stake 0.08 → 0.07, Pumped Hydro Storage 0.06 →
0.05, Shadow Broker Scrub 0 → 0): they are played for their other halves, and now find a token to lift. Reclaim 0.10
/ 0.12 a Protector game (Seize 0.16 / 0.09), Recovery 0.04 → cut.

**Replay on the final build:** after the freeze only text changed (Sanctuary's wording, its face, comments): seeds
46880-46885 (144 games) replayed on the final build match the farmed games exactly (`farm/compare.py`: 0 differ).

## Speed

`per_decision.py out/227_fallout/base out/227_fallout/new --games 16` (c2, d2, seed 1000, best of 2, a quiet box):
greedy **0.95×** a decision (1.7 ms both; 0.95× the decisions, 0.98× the rounds), the strategist **1.03×** (1.8 →
1.9 ms; 1.08× the rounds). No `--update`: speed.py gives no verdict across a revision.

## Not done / candidates

- The project anchor is the country's lowest-numbered corrupted station, not necessarily the one the project just
  took (the op does not carry the taken hex).
- `design_check` flags Lift Fallout dominant (0.26 a seat-game against its 0.01-0.20 hedge band, 210): the band was
  set when Fallout was rare; Matt's to widen.
- Night Raid's node line is still `corrupt` with `site: node` in the schema; only its print changed.
