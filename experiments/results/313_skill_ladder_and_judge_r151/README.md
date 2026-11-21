# 313: the skill ladder at r151, and the judge against the oracles

Base: main 20cae79, r151. No rules or content change: `RULES_REVISION` stays 151. The judge (`strategist`) changes:
`w.purge` 3.6 → 1.8 and `reach_rule` on (D141). Readers here: `lad.py` (the ladder table), `scr.py` (an agent-arm
screen, paired), `split.py` + `rank.py` (agree split by the judge's result), `choose_kinds.py` (the `choose` misses
by option). Specs: `experiments/archive/farm/{l313,l313o,s313a-g,f313}.json` (judge arms pinned to r151's judge where
they were farmed with it). Outputs kept: `out/keep/313/`. Every timing ran on a loaded box.

## 1. The ladder at r151 (fresh seeds 51552-51599, d0-d4, three pairs)

Greedy and the judge on 48 seeds a pair (144 games a cell); both oracles on the first 16 (48 games a cell: a real
oracle game is 45-110 s, and 8 seeds of both oracles took 55 min on 4 main-box shards, the browser worker taking
none). The oracle rows (`skilled_rows.py`'s `oracle` and `real`) ran r151 play (farm tree checked against 4ac5eeaf,
reach_rule off; Matt's message 14336).

On the 16 seeds all four played (48 games a cell; gap to the judge paired on seed):

| level | greedy | judge | real oracle | full oracle | full − real | judge's place, greedy → real |
|---|---|---|---|---|---|---|
| d0 | 89.6 | 95.8 | 100.0 | 100.0 | 0 | above both |
| d1 | 95.8 | 85.4 | 93.8 | 100.0 | +6.2 | below both |
| d2 | 87.5 | 83.3 | 77.1 | 100.0 | +22.9 | between (real below greedy) |
| d3 | 75.0 | 56.2 | 75.0 | 100.0 | +25.0 | below both (−18.8 ± 7.7) |
| d4 | 41.7 | 27.1 | 33.3 | 100.0 | +66.7 | below both |

Rounds (all games; won): judge 6.8-7.6 (6.1-6.8), greedy 6.6-7.2, real oracle 6.7-7.4, full oracle 7.2-8.2 (it
plays every game to a win, so its rounds are its wins'). Full table with SEs: [ladder16.md](ladder16.md); greedy and
the judge on all 48 seeds: [ladder48.md](ladder48.md) (judge 96.5 / 91.7 / 88.9 / 68.1 / 36.8 at d0-d4; greedy
+4.6 ± 1.6 over it pooled, +11.1 ± 4.3 at d3).

- **The full oracle won all 240 games, d4 included** (243 at r109: 95.8% at d4). Every seed read is winnable: Matt's
  "very strong agents should reach 90% or more even at d4" holds for the oracle that sees the deck, not for any
  fair-information player: the real oracle reads 33% at d4, the judge 27-37%.
- **The full oracle's edge over the real oracle is 0-67 points, growing with level** (+23 d2, +25 d3, +67 d4). It is
  not all sight of the deck: the two differ in two things, so this comparison changed two at once. The real oracle
  also has no restarts (`tries=1` against 3) and searches on 249's judge weights where the full one uses the engine's
  value table. Sight of the deck plus restarts is the gap. The real oracle sat at greedy's level: −6 to +8 around it.
- **The judge sits below greedy on these seeds** (d1, d3, d4), and below the real oracle at d1-d4 except d2. The 96
  spent r151 seeds of the screens read the other way (judge 79.2% against greedy 75.0% at d1-d4, s313a), so greedy
  against the judge swings ±5 points with the seed set at this scale. It is not a settled lead either way.

## 2. Where the judge loses

**The real oracle is built on the judge's own evaluation.** Its row (`skilled_rows.py` `real`) is the search on
249's judge weights (`strategy=0`, `w.hand=1.5`, `w.purge=3.6`, `w.last_node=120`, ...; it predates
`w.card_action`, `w.sink`, `w.am_*`, `w.adv_read` and `w.lib_clock`). Its rollouts are greedy on that eval
(`planner::rollout_policy`) and its candidates are that eval's best. So under `docs/rulings.md` it is the yardstick,
not a tuning target. The full oracle (engine weights, value table, `strategy=1`) is not built on the judge, so its
play is the target.

`aic-sim agree` on the full oracle's 240 won games, split by the judge's result on the same pair, level and seed (73
games the judge lost, 167 it won), the judge as r151 plays it:

| kind | decisions a lost game | judge's miss % (lost) | (won) | excess misses a lost game |
|---|---|---|---|---|
| choose | 117.1 | 10 | 9 | +0.7 |
| threat | 16.0 | 30 | 28 | +0.3 |
| power | 24.2 | 27 | 26 | +0.2 |
| keep | 43.5 | 5 | 5 | +0.0 |
| convert | 3.2 | 77 | 78 | −0.0 |
| sell | 7.2 | 67 | 68 | −0.1 |
| pass | 23.6 | 46 | 47 | −0.1 |
| tier | 61.3 | 24 | 24 | −0.2 |
| use | 11.6 | 30 | 33 | −0.4 |
| draft | 53.6 | 33 | 34 | −0.6 |
| standard | 28.7 | 46 | 49 | −0.9 |
| play | 37.4 | 22 | 26 | −1.4 |

A miss is a pick the oracle didn't make even later that round; `excess` is how many more of them a lost game has than
the won-game rate predicts. **The judge disagrees with the full oracle as often in the games it wins as in those it
loses** (56.5% same pick in lost games, 56.2% in won; greedy 57.4 / 55.9). No decision kind carries the losses: the
largest excess, `choose`, is under one decision a game. By raw misses the kinds rank draft (17.5 a lost game), tier
(14.6), standard (13.2), choose (11.7), pass (10.9), play (8.2). Inside `choose` (8 seeds, [choose_kinds.py](choose_kinds.py)):
hex picks differ 8.2 a lost game against 4.7 a won one; on track uncovers the oracle takes the card-play track where
the judge takes the economy track, 2.4 a game in lost and 2.1 in won games.

## 3. Judge fixes, each screened on and off

Screens on r151's spent fresh set (312's read, 51456-51551), d1-d4, three pairs; gap to base paired; all/d2-d3.

| lever | screen | all | d2-d3 | verdict |
|---|---|---|---|---|
| each judge term back to greedy's default, one at a time (purge, last_node, card_action, sink, am_cap, adv_read, lib_clock, hand terms) | s313a, 32 seeds | card_action −4.4 ± 2.1, hand −5.7 ± 2.2, sink −2.3 ± 1.2; the rest ±1.6 | | the terms pay or tie: no term hurts |
| **`w.purge` 3.6 → 1.8** | s313f, 96 seeds | **+1.7 ± 1.2** | +1.9 ± 1.5 | **shipped** (0: +1.6 / +2.3, a tie, so the higher value: Matt, message 14355) |
| **`reach_rule` (D141)**, after the strike fix | s313g, 96 seeds, 2×2 with purge | +0.0 (no game changed) | +0.0 | **shipped**: mirrors the engine, never fired here |
| `w.on_factory` 6 → 0 (the factory danger term, 304) | s313a 32 + s313e 64 seeds | +1.0 ± 1.7, −1.4 ± 1.3 | −1.6, −1.8 | off |
| `w.last_node` 0 | s313a + s313e | +1.3, −1.7 | | off |
| `w.sink` 0.5 → 1 (s313b) / 1.5 | s313b 32, s313c 64 seeds | +2.1 ± 1.0, then −1.2 ± 0.9; 1.5 −2.5 ± 1.0 | | off: didn't replicate |
| `w.sink_idle` (304's `sink_terms`: idle antimatter as fuel at idle worth) | s313b, s313c | +0.8 ± 0.4, −0.7 ± 0.5 | | off (code kept, default 0) |
| `w.card_action` 3 → 4.5 | s313b | −1.3 ± 2.0 | | off |
| `w.track_insight` 3 / 4.5 / 6 (follow the oracle onto the card-play track) | s313d, 64 seeds | −1.7 / −4.3 / −6.4 (±0.9-1.4) | −2.1 / −4.2 / −7.0 | off (code kept, default 0) |
| `plan_target=1` (the keep plan reads a card's target condition) | s313a | −0.3 ± 0.9 | +0.5 | off |

**What the purge term measures at r151** (Matt asked): the purge mechanic it was named for is gone (r141), but the term
still pays each seat's supply ambassadors up to the Liberate bills of the corrupted sites it reaches or stands on,
and keeps that worth for each city freed (`liberate_at` × freed), so the judge saves ambassadors toward Liberate and
spending them on one costs nothing. Halving it lets the judge spend ambassadors elsewhere.

**Skipped, and why:** `am_tie` into the engine and the `lib_clock` default change only the engine, planner and
oracle (the judge already has both); `credit_equivalent`'s tied rate reaches only the keep plan's `plan_table=0`
path and policy sort orders the judge doesn't take; D72's ambassador-bill half (`free_own_next_to`) is rare and 169's
diff no longer applies; 222's do-nothing weights change no game (a cleanup, not a fix); D57/D74 (cards kept and
never played) is a keep-versus-play pricing pick, Matt's; the oracle's energy and Power play (243) lost three probes
in 245-249 and the track lever above is its nearest cousin here, also a loss.

## 4. The fresh read: the new judge against r151's judge

Fresh seeds 51744-51887 (144 a pair, 432 games a cell), d0-d4, the three pairs; r151's judge is
`strategist:w.purge=3.6,reach_rule=0` on the same build: 313's code differs from r151's only in branches that are
off there, and the pinned arm played the same 384 games on two builds (s313a, s313b).

| level | new judge | r151 judge | gap, paired | SE | band (soft) | new in band |
|---|---|---|---|---|---|---|
| d0 | 94.9 | 93.8 | +1.2 | 1.4 | 90-100 | yes |
| d1 | 93.3 | 91.4 | +1.9 | 1.6 | 80-95 | yes |
| d2 | 90.0 | 87.7 | +2.3 | 1.7 | 75-90 | yes (top edge) |
| d3 | 80.1 | 76.2 | +3.9 | 2.1 | 55-70, no ceiling | yes (above 70, was already) |
| d4 | 56.0 | 54.2 | +1.9 | 2.3 | - | - |
| d2-d3 | 85.1 | 81.9 | **+3.1** | 1.3 | | |
| all | 82.9 | 80.6 | +2.2 | 0.8 | | |

**The bar is met on this read:** d2-d3 +3.1 ± 1.3, and no level worse. Games run a little shorter (6.55-7.17
rounds against 6.66-7.24). Cost per decision 0.93× r151's judge (`per_decision.py`, 6 games, loaded box).

**Not a clean read alone:** the first fresh read (51600-51743, 144 seeds) ran on the build before the diff review's
strike fix and read d2-d3 +0.2 ± 1.4 (all +1.2 ± 0.8, d4 +6.2 ± 2.4). The fix changed no game in 1,152 screen games, so
both reads are close to the same judge; pooled, d2-d3 is about +1.7 ± 1.0. Read the gain as about two points, real,
smaller than this read's three.

**Bands** (soft, not dialed): the new judge moves every level up 1-4 points; d0-d2 stay in band, d3 sits above 70
(no ceiling since Nov 21: P1 drops `ladder.py`'s 70 cap), d4 has no band. **Skill ceiling against Matt's ruling**
("very strong agents should reach 90% or more even at d4"): the full oracle reaches 100% at d4 (48 games), so the
seeds are winnable; no fair-information player is near it (real oracle 33%, the new judge 56% on these seeds).

## Seeds and outputs

Fresh: 51552-51599 (ladder), 51600-51743 (the first fresh read, on a build whose strike count the diff review then
fixed: spent, not reported as the read), 51744-51887 (the read above). Screens on r151's spent 51456-51551. Ledger
next free: 51888.
