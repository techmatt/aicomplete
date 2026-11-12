# 199: insight as a spending plan, chips worth claiming (r87)

`RULES_REVISION` 86 → 87, bumped from the constant (the prompt said 87 too). Built on main 9c2dd63 (198). Every
number is a candidate. The old side is r86 (`out/r86`: 9c2dd63 with 199's stats-only counters patched in, so both
sides read the keep plan).

## What changed (r87)

| change | r86 | r87 | read | why |
|---|---|---|---|---|
| Card-play track insight totals (`content/ais.json`) | 3, then 7, 9, 11, 12, 13 on every board (Researcher 7, 9, 10, 12, 13) | 3, then Builder 5, 7, 8, 9, 10; Protector 5, 6, 7, 8, 9; Researcher 4, 5, 6, 7, 8 | screen 1 `t1`: +4.2 ± 2.6 vs r86's track | Matt: insight income is the lever, each board at its own rate |
| Builder's **Focus** | cut (196) | back: 2 credits → 1 insight, once a round | 0.04 uses a Builder game (fresh) | Matt: Focus comes back |
| `board.claim_factory` | 3 | **2** (Liberate stays 3) | with the eval weight below: 0.49 claims a game | Matt: a claim costs 2 ambassadors |
| Chip converters (`content/map.json` `chip_converter`, new field) | none | Tidal Grid 2 chips → 3 energy; Polar Compute Array 2 chips → 3 cards; Delta Seawall 2 chips → 2 Sway to Unity, 1 card; once a round each, as the cube converter (a city in reach, after completion); Onyx none | 1.33 uses a game (Polar Compute Array; Tidal Grid's never used) | Matt: chips hoarded early pay late |
| D88: `take_site` | a guarded neutral site dropped out, so the player's own site turned neutral | a guarded neutral site counts as neutral: nothing happens in that country | — | Matt's ruling |
| `compute_income` d3 | 18 | **22** | d3 53.5 → 44.7 (confirmed fresh) | calibration, last |
| Eval's `owned_factory` (agents) | 4 | **10** | claims 0.02 → 0.46 a game, +3.9 ± 2.0 (r86 track), +3.6 on t1 (t1 alone +4.2); 16: 2.3 claims, −8 vs 10 | the eval never paid 2 ambassadors (10) for a factory worth 4 |
| Stats only | — | `cards.keep_at.<s>.<id>`, `cards.keep_ok.<s>.<id>`, `cards.keep_plan.*`, `cards.late_play.<id>`, `cards.kept_t{1,2}.rNN`, `cards.played_t{1,2}.rNN`, `cards.seat_insight_kept/played.<s>.rNN`, `economy.seat_insight_unspent.<s>.rNN`, `limit.pass.{unaffordable,chosen}`, `board.chip_converter.<c>`, `board.chips_converted`, `board.chip_converter_idle`, `adversary.sites_guarded` | — | the keep-plan measures |

**Chip prices:** nothing to halve. Every printed chip price is 1 (stations, Procure, Parts Exchange, Orbital Mass
Driver: r83 halved them); the 2-4 chip numbers are threat penalties (the threat card standard, locked) and
`city_build_chips` (a city's growth, no player's price).

## Design check (give / get at `values.json`; the anchors in brackets)

| item | give | get | gain | where it matters in an 8-10 round game |
|---|---|---|---|---|
| Focus | 2 credits | 1 insight 2.5 (4) | +0.5 (+2) | a keep or play 1 short; credits bind first (unused: 0.04 a game) |
| Claim a factory | 2 ambassadors 8 (8) | 1 chip a round 4 (6) | pays back in 2 rounds (1.3) | claimed by round 3: 6-8 chips; claims come at d0 (1.39 a game), seldom at d2-d3 (0.10-0.12: agents crowd factories) |
| Tidal Grid chips | 2 chips 8 (12) | 3 energy 13.5 (13.5) | +5.5 (+1.5) | from round 3 (megaprojects complete by round 3 in 97% of games) |
| Polar Compute Array chips | 2 chips 8 (12) | 3 cards 12 (15) | +4 (+3) | the one used: 1.33 a game |
| Delta Seawall chips | 2 chips 8 (12) | 2 Sway + 1 card 12 (13) | +4 (+1) | 4-country games only; first priced 3 Sway (12 for 12 at the anchors: even), fixed before any read |
| Mint (unchanged) | 3 credits | 1 chip 4 (6) | +1 (+3) | feeds Procure and the converters |

## Ladder (fresh, paired against r86)

| d | strategist r87 | r86 | gap ± SE | band | verdict | greedy r87 (r86) |
|---|---|---|---|---|---|---|
| d0 | **91.7** | 88.7 | +3.0 ± 2.0 | 90-100 | met | 87.5 (90.3) |
| d1 | **74.3** | 71.5 | +2.8 ± 2.7 | 70-80 | met | 68.8 (61.8) |
| d2 | **62.3** | 63.2 | −0.9 ± 2.4 | 55-65 | met | 66.7 (65.3) |
| d3 | **44.7** | 50.0 | −5.3 ± 2.8 | 40-50 | met | 50.0 (47.2) |

d0-d2: 44479-44622 (432 games a level; the final build replays them exactly, 0 of 432 differ). d3: the first read put
it at 53.5 at compute 18; screened 20/22/24 and 3 agents on those seeds (49.5 / 45.1 / 44.4 / 40.7), 22 confirmed on
44623-44766. Greedy: 48 seeds a set (bands are the strategist's). `stage_extra` untouched. Tables:
[`ladder-strategist.md`](ladder-strategist.md), [`ladder-greedy.md`](ladder-greedy.md).

## Insight by board (strategist, fresh, a seat-round)

| board | to keeps | to plays | all spent | unspent | bound | r86: keeps / plays / spent / unspent / bound |
|---|---|---|---|---|---|---|
| Builder | 2.03 | 1.58 | 4.30 | 3.3 | 18% | 2.25 / 1.67 / 4.62 / 4.9 / 12% |
| Protector | 2.33 | 1.55 | 4.81 | 3.8 | 20% | 2.53 / 1.63 / 5.09 / 6.0 / 10% |
| Researcher | 1.95 | 1.55 | 4.18 | 5.4 (banked) | 19% | 3.09 / 1.53 / 5.83 / 14.6 / 10% |

Against Matt's 6 to keeps and 2-4 to plays: **plays are in band at the low end (1.6); keeps are 2.0-2.3, a third of 6.**
Unspent fell by a third (Builder, Protector) and two thirds (Researcher). No allowed lever moved keeps toward 6: every
income cut lowers keep spending (t1 1.99, t2 1.88 a Builder seat-round, r86's track 2.14), and the fallback, a
tier-2 keep at 3, reached 2.6-3.7 and lost 4.2 ± 2.8 (−7.8 with t1). The keep is demand-limited: a seat drafts 4 a
round and keeps ~1.7. **Patience:** the Researcher still carries 39 insight a game (106 at r86): its income (up to 8)
outruns its spending (4.2), and Synthesis is never used (0.00), so binding shows only on the other boards' terms.

## The keep plan (strategist, fresh, per game, both seats; [`fresh-measures.md`](fresh-measures.md))

| measure [target] | r87 | r86 |
|---|---|---|
| kept (drafted keeps) | 28.6 | 34.8 |
| dead keeps, never had the resource [under ~15%] | **3.01 (11% of keeps)** | 5.40 (17%) |
| dead keeps, held back (playable, never played) | 10.75 | 12.56 |
| dead keeps kept in the game's last round | 2.35 | 3.04 |
| late keeps (played 3+ rounds after the keep) [present, worth it] | 0.64 (5% of kept plays): reasoned 28.0 against a printed 18.4 | 0.86 (6%): 28.7 / 18.4 |
| passed: short of insight / keepable but not playable by next round / chosen, a seat-game [both present] | 1.72 / 8.03 / 7.34 | 1.05 / 7.70 / 5.34 |
| tier-2 keeps a game, rounds 2 / 3 / 4 / 5 [earlier than r86] | 1.10 / 1.22 / 1.68 / 1.96 | 1.39 / 1.93 / 2.90 / 2.97 |

Tier 2 is kept **later** at r87: the first card-play space fell 7 → 5 (4 for the Researcher), so round 2-4 seats can
afford one tier-2 keep, not two. Screen 5's `t4` (start 7, then flat: Builder 7, 8, 9, 10, Protector 7, 8, 9,
Researcher 6, 7, 8) puts tier 2 back near r86 (round 4: 2.47) at +2.1 ± 2.5 vs t1, with binding back to ~10% and
the Researcher's bank at 72: Matt kept t1 (design question 1, message 8979), t4 the alternative.

## Claims and chips (strategist, fresh, per game, both seats)

| measure | r87 | r86 |
|---|---|---|
| neutral factory claims | 0.49 (d0 1.39, d1 0.37, d2 0.12, d3 0.10) | 0.00 |
| chips held as the action phase ends, a seat-round / share at 0 | 0.50 / 88% | 0.60 / 85% |
| chips spent | 8.31 | 5.80 |
| what they buy | Procure 2.91, stations 2.71, Polar Compute Array's converter 2.66 (1.33 uses), Orbital Mass Driver 0.02 | Procure 2.86, stations 2.89 |

## Screens (dev seeds 36800-36831, 384 games a cell; [`screens.md`](screens.md))

| cell | gap ± SE | read |
|---|---|---|
| r87 rulings, r86 track (the prompt's numbers) vs r86 | +0.0 ± 1.5 | Focus 0.01, claims 0.02 a game: the rulings alone change little |
| t1 (shipped) / t2 (deeper, fewer steps) / t1 with no keep upgrade | +4.2 ± 2.6 / −0.8 ± 2.6 / +4.2 ± 2.7 | t2 binds 23-31% and Focus still 0.03 |
| tier-2 keep 3 / with t1 | −4.2 ± 2.8 / −7.8 ± 2.8 | the fallback |
| eval `owned_factory` 10 / 16 | +3.9 ± 2.0 / −4.2 vs t1 | 16's cells ran with t1 already in content (edited mid-farm), so `f16` = `f16t1` |
| keep plan `plan_rounds` 2 / 3 (vs 4) | −9.6 ± 2.7 / −6.0 ± 2.5 | dead keeps unchanged: not adopted |
| t3 / t4 (early-heavy) vs t1 | −4.4 ± 2.5 / +2.1 ± 2.5 | t4: Matt kept t1 (message 8979), t4 the alternative |
| draft hand 5 (`cards.deal`) / with t4, vs r87 | +1.8 ± 2.7 / +3.1 ± 2.7 | keeps a Builder seat-round 1.96 → 2.13 / 2.28 (Researcher 3.01); with t4, tier-2 keeps at round 4-5 2.82 / 3.26 (r86 2.90 / 2.97): the next lever toward 6 to keeps, not adopted (past the prompt's levers) |

An eval-side spending plan (`plan_over`) changed no game (the strategist keeps by `keep_plan.rs`) and was removed.

## Files

`screen_spec.py`, `read_spec.py` (specs in `experiments/farm/s199-*.json`, `f199-*.json`), `plan_read.py` (the
measures), `dead_read.py`; tables `screens.md`, `fresh-measures.md`, `ladder-*.md`. Seeds: dev 36800-36831; fresh
44479-44766 spent (ledger next free 44767).

## For new players (r87's changes)

Your card-play track still starts at 3 insight, but each board now grows it at its own pace: the Builder up to 10,
the Protector to 9, the Researcher (who keeps unspent insight) to 8. The Builder can again pay 2 credits for 1 insight
(Focus), once a round. Claiming a neutral factory costs 2 ambassadors and pays you a chip every round; once a
megaproject is complete, three of the corps also let you trade 2 chips, once a round, for 3 energy (Horizon), 3 cards
(Cintamani) or 2 Unity and a card (Biomia), so chips saved early pay off later. A firewalled neutral factory or station
now protects your own: a seizure that finds its only neutral target guarded takes nothing. At level 3 the adversary
gains 22 compute a round. **What resisted the summary:** nothing.
