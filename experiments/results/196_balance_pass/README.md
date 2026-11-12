# 196: balance pass after the project overhaul (r86)

`RULES_REVISION` 85 → 86, bumped from the constant (the prompt said 86 too). Built on main 7986540 (195). Every
number is a candidate. The base is 195's build (`out/base`: r85's play, with 196's stats-only counters patched in so
both sides read insight binding).

## What changed (r86)

| change | before (r85) | after (r86) | read | why |
|---|---|---|---|---|
| Card-play track's printed start (`content/ais.json` `insight`, all three boards) | 6 | **3** (later spaces' totals 7, 9, ... unchanged) | screen 3: track 4 binds 6-8% of seat-rounds, 3 binds 9-11%; fresh: **10.4%** (r85 3.4%); d1-d3 −13 to −18 at r85's dials on the dev seeds (screen 3, P3) | Matt: insight should bind; the bar is 10%, which 4 doesn't reach |
| Builder's **Focus** (5 credits → 1 insight) | a power | **cut** | 0.00 uses at r85; at 2 credits (give 2, get 2.5) still 0.03 a Builder game, with the track at 6 or 4 | dead: insight is short in a seat-round, never by one point a Builder would buy |
| `adversary.stage_extra` (new list, an entry a level: event cards added, stage I first, then stage II) | none | **2, 0, 0, 0, 0** (d0: 5 / 5 / 3 cards, a 13-round cap) | screen 1: 1 card +8.3 at d0 (base track); screen 4 (track 3): 2 cards 91.7, 3 cards 95.8 | every d0 loss is the round cap; the prompt's pick, as a dial |
| `compute_income` d0-d4 | 1, 4, 19, 24, 24 | **1, 4, 17, 18, 24** | d3: screen 4 (18 with 2 agents) 44.8; d2: the fresh read put d2 at 53.5 at 19, so 17, confirmed **63.9** on the next fresh seeds | calibration, last |
| `ambassador_income` d0-d4 | 0, 2, 2, 3, 3 | **0, 2, 2, 2, 3** | with d3's compute above | calibration |
| Eval's held chip (`w.chips`, sim players) | 2 | **4** | +4.2 ± 2.3 (screen 1w, d0-d3); chips held at 0: 92% → 84% of seat-rounds (screen 1 against 1w) | chips were never held long enough to buy a station |
| Eval's `w.adversary_cube` (sim players) | a weight, 0.5 | **deleted** (0.5 inline) | 0, 0.5 and 1 play the same 384 games at r85 | prompt item 4: nothing reaches it |
| Stats only: `cards.insight_bound.<s>.rNN`, `cards.insight_short.<s>.rNN` | none | per seat-round counters | — | the done-when bar needs seat-rounds |

**Kept as printed, after screens** (dev seeds 36800-36831, 96 games a level a cell, `screens.md`):

| lever | read | why not |
|---|---|---|
| 191's tier-1 play costs back (64 cards to 1 insight, credits to de6b0aa's) | binding +1 point over the track alone (7.9% vs 7.1% at track 4); alone (screen 2) d0 −8.3, d1 −6.2, d2 +7.3, d3 −1.0, all −2.1 ± 2.8 (screen 1 read the signs the other way at d0-d1: noise) | 64 face edits, give / get and card-fit for a point of binding; it is also cheaper credits, which the locked rulings rule out as the way to raise player power |
| Cheaper claim (`board.claim_factory` 3 → 2) | 0.02 neutral factory claims a game; no game changes | the claim isn't a sim-player bug (below) |
| Eval's `owned_factory` 4 → 12 (so the agents claim) | claims 0.32 a game (1.8 with claim 2), wins −3.9 ± 2.1 | claiming a neutral factory loses: 3 ambassadors and the city growth it fed, for 1 chip a round |
| Face-down land corruption drips again (194's d0 lead) | Clears 0.10 → 0.92 a game, d0 +1.0 ± 3.1, d3 −4.2 | the Clears came back; the d0 wins didn't, so the drip isn't why d0 lagged |
| `adversary.wall_landing` | players own no factory (0.00 claims) | still inert; kept as the prompt said |
| Patience (uncapped, #5117) | banks 99-106 insight a Researcher game at r86 (84-85 at r85) | Matt's ruling; asked (question 1), default kept |

## Ladder (strategist, paired against r85 on the same seeds)

| d | r86 | r85 | gap ± SE | band | verdict | seeds |
|---|---|---|---|---|---|---|
| d0 | **90.5** | 83.6 | +6.9 ± 2.3 | 90-100 | met (by 0.5) | 44191-44334 |
| d1 | **70.8** | 76.4 | −5.6 ± 2.7 | 70-80 | met (by 0.8) | 44191-44334 |
| d2 | **63.9** | 66.2 | −2.3 ± 2.7 | 55-65 | met | 44335-44478 (compute 17) |
| d3 | **40.7** | 48.1 | −7.4 ± 2.8 | 40-50 | met (by 0.7) | 44191-44334 |

432 games a level (three pairs × 144 seeds). The first read (44191-44334) had d2 at compute 19: **53.5**, −10.2 ±
2.8, under its band by 1.5. d2's compute moved to 17 and was confirmed on the next free seeds; the final config
replays d0, d1 and d3 of the first read exactly (16 seeds × 3 pairs a level, 0 of 144 differ: `same_games.py`), so
those rows stand. d0, d1 and d3 sit within an SE of their band edges. Greedy (48 seeds, 144 games a level): 88.2 /
74.3 / 68.1 (d2 confirm) / 53.5 against r85's 85.4 / 80.6 / 75.7 / 43.8: greedy d0, d2 and d3 are outside the bands
(the bands are the strategist's). Tables: [`ladder-strategist.md`](ladder-strategist.md) (with the standing
measures), [`ladder-greedy.md`](ladder-greedy.md).

## Insight binds

[`insight-binding.md`](insight-binding.md). A seat-round is **bound** when a drafted card went unkept while the seat
held less insight than its keep (wanted or not: the counter can't tell), or a card stayed in hand short of insight
and of nothing else; **short** counts any insight shortfall, `bound` included.

| measure (strategist, fresh) | r86 | r85 |
|---|---|---|
| bound, all seat-rounds | **10.4%** (d0 9.4, d1 10.2, d2 11.1 at compute 19 / 10.5 at 17, d3 11.2) | 3.4% |
| bound by pair: Builder+Protector / Builder+Researcher / Protector+Researcher | 8.9 / 11.0 / 11.2% | 3.3 / 3.5 / 3.6% |
| short (any insight shortfall) | 16.9% | 12.3% |
| rounds every seat ended at 0 insight | 7% | 1% |
| drafted cards unkept for want of insight, a seat-game | 1.05 | 0.10 |
| tier-2 chosen in hand-building, a game | 41.1 | 40.3 |
| insight carried by Patience, a game (both seats) | 66.4 | 55.1 |

**Tier-2 against two tier-1s** is not yet a real trade: players choose tier 2 as often as at r85 (41.1 against 40.3
a game) and skip the keep instead (1.05 unkept a seat-game against 0.10).

## Resource use

[`resource-use.md`](resource-use.md) (189's reader, r86 against r85, per seat a game):

| resource | spent | ran out (seat-rounds) | read |
|---|---|---|---|
| credits | 245 (249) | 30% (31%) | the main limit: 101 cards a seat-game left in hand short of credits |
| insight | 36 (36) | 7% (1%) | binds now: keeps 40, plays 24 a game; Patience piles it up for the Researcher |
| energy | 28 (27) | 35% (39%) | powers 20 of it; binds |
| chips | 2.8 (2.0) | 86% (91%) | more held at weight 4, still rare: no player owns a factory; they buy stations and Procure |
| antimatter | 0.1 (0.1) | 75% (76%) | barely used: Particle Run makes 0.3-0.9 a game, cards spend 0.14 |
| research cubes | 3.8 (3.1) | 78% (83%) | Grant and the converters; small |
| ambassadors | 16.6 (16.9) | 50% (48%) | binds |

**Powers never used** (`board_read.py --powers`, fresh r86): the Researcher's **Synthesis** (0.00 at every level,
both revisions: Patience means the Researcher never runs out), the Protector's **Recovery** (0.01-0.14, D84) and
**Shelter** (0.1-0.3).

## Ideas not built, ranked for Matt

1. **Cap Patience** (carry at most 3): the Researcher banks 100 insight a game and never binds; Synthesis would come
   alive with it. Held by #5117 (question 1).
2. **Make tier 2 a trade**: tier-2 picks didn't fall when insight tightened. A tier-2 keep at 3, or tier-1 keeps at
   0, would make two tier-1s the cheap alternative.
3. **Give chips a place**: claimed factories never pay (one chip a round for 3 ambassadors and a growth feed). A
   claimed factory making 2 chips, or a chip-priced standard action every board has (Procure is the Builder's), would
   make chips a currency.
4. **Antimatter**: spent 0.1 a seat-game. Either more cards price in it or it folds into energy.
5. **Cut Synthesis and Recovery** if Patience stays uncapped (both read near 0).
6. **Threat counters' insight items** (6 at tier 1, 7 at tier 2, `docs/rules/threats.md`) now sit above a round's
   starting insight (3) until the card-play track is uncovered: the insight option of a counter is out of early
   reach. Not touched (the threat card standard stands); a reprice to the new start is Matt's.
7. **d0, d1 and d3 sit at their band edges**: a next calibration has little room; d0 could take a third card
   (95.8 on the dev seeds), d3 compute 20.

## Speed

`per_decision.py out/base out/new86` (8 games, seed 1000, c2 d2, one thread, best of 2, quiet box): greedy **0.64x**
a decision on 0.66x the options (less insight, fewer playable cards a decision), 0.51x a game; strategist 0.88x a
decision on 0.93x the options. `speed.py check --update` set the baseline at r86.

## Checks

Fast lane and `cargo test-all` green; clippy clean; `aic-sim docs --check` current; `regen.py` run. Not run: `npm run
cards:fit` (no card face changed; the boards' printed start did), `play_read.py` and `loop_smoke.py` (no card or op
changed), `bench/same_results.py` (the farmed reads have no games.csv: `same_games.py` instead, above).

## Files

`screen_spec.py` (screens 1-4, the adversary-cube check), `read_spec.py` (the fresh specs, in `experiments/farm/`),
`cells.py` (the per-lever reader), `agent_gap.py` (weight cells), `same_cells.py`, `same_games.py`; tables
`screens.md`, `ladder-*.md`, `insight-binding.md`, `resource-use.md`. Seeds: dev 36800-36831; fresh 44191-44478
spent (ledger next free 44479).

## For new players (r86's changes)

Your card-play track now starts at 3 insight, not 6: early on you can keep a tier-2 card and a tier-1 card, or three
tier-1s, and play only free cards, so choose what to keep; the track's later spaces (7, 9, ...) are as before. The
Builder no longer has Focus. At level 0 the adversary deck holds two more event cards (13 rounds at most). At level 2
the adversary gains 17 compute a round (was 19); at level 3, 18 compute and 2 agents a country (was 24 and 3).
**What resisted the summary:** nothing.
