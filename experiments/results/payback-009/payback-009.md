# 009 engine-payback sweep: which levers make a building engine win?

> **Historical:** ran on rules r3 (today's is r6). Configs, parameters, the rules
> sections and game records named here were removed or rewritten later; several of its
> specs are refused today and the records do not replay. The refused specs moved to
> [`archive/`](archive/) in 016, and `scripts/payback/resources.py` was deleted
> (retired in 015: it keyed on a phase removed in 014); paths below are as they were.

2026-10-28. Rules r3 (007's tuned v0), 2 countries. **Measure only:** the baseline
config, content and `RULES_REVISION` are unchanged. Every lever is a config override in a
spec, or a **content patch** built into a separate binary (marked ◆). Committed content is
never touched (README, *Payback sweep (009)*).

Agents: `engine` (008's building agent, its defaults) against `greedy`, which never plays
an economy building. Both agents play the same seeds, 300000–300095 (96 per cell), at
difficulties 0 and 4. The two headline levers also ran on 300096–300239 (240 in all). The base
cells reproduce 008's games on those seeds exactly. The first 96 seeds are kinder than
008's 240: base greedy wins 77% at d4 there, against 71% over all 240.

Results: [`results/payback-009-singles`](results/payback-009-singles),
[`results/payback-009-combos`](results/payback-009-combos),
[`results/payback-009-content`](results/payback-009-content) (one directory per
patch). The game records were deleted in 038 (they never replayed on later rules);
[their README](results/payback-009-records/README.md) stays. Every table row comes from `scripts/payback/summarize.py` over those directories.

## Answer

**The lever that makes a building engine win is economy buildings that give insight**,
not only credits (◆ `insight_inc`: Fab Line, Trade Office, Solar Field, Microgrid, Fusion
Plant and Megastructure also give +1 insight income). It works best with those buildings
at half cost (◆ `econ_half`). Neither is reachable by config.

| 240 paired seeds | d0 greedy / engine | d4 greedy / engine | d4 engine − greedy | Economy buildings standing (engine, d4) | Engine d4 wins with 2+ |
|---|---|---|---|---|---|
| base (008's run, same seeds) | 94.6 / 92.9 | 70.8 / 68.3 | −2.5 ±3.1 (p 0.50) | 0.93 | 21% |
| ◆ `insight_inc` | 94.6 / 96.7 | 70.8 / 77.1 | +6.2 ±3.1 (p 0.06) | 2.05 | 74% |
| ◆ `econ_half` + `insight_inc` | 96.7 / 95.8 | 70.4 / **82.5** | **+12.1 ±2.9 (p < 0.001**; 41 seeds only the engine won, 12 only greedy) | 3.42 | 96% |

**Audit (032):** the headline holds on 144 fresh seeds: `econ_half` + `insight_inc`, d4
+11.8, p 0.002. The explanations below do not carry that weight: the tier-2 "agent
artifact", insight being what binds the engine, the cause given for the keep cost's cliff
and `pool_reduction` 6 landing on "about 70%" are **unverified hypotheses**, not findings.
Corrections, checked against the data in 032; the text below is left as written:

- **Cheaper tier 2:** "mostly Counter-intelligence" is false: 98 of 4,420 d0 plays (2%),
  the top plays are the usual ones, and Purge Protocol plays are down (109 against about
  235). The cause of the collapse is unknown, and greedy lost 9 points at d4 too; the
  lever may be measured and bad.
- **`overclock_cheap`** was a void test: the engine played it 1 and 0 times of 94 offers.
  It shows agents don't value a lone insight building at any price, not that insight
  income doesn't help. `insight_inc` works by riding on buildings bought for credits, so
  its effect is real for these agents but its mechanism is inferred.
- **"Why" section:** stated from 32 traces, with no test. `econ_noinsight`, the one test
  of insight spending, did nothing.
- **Unlock:** only 4 of 10 tier-2 cards need antimatter, and "misses the tier-1 opposition
  cards" was never measured.
- **Keep cost:** the Builder holds 12-16 credits after round 1, and agents then kept
  nearly everything they drafted, so `keep_cost` 3's collapse probably measures agent
  behavior, and "next to a cliff" is overstated.
- **Longer deck:** "builds for rounds it doesn't survive" is a guess.
- **`pool_reduction` 6:** the 69.8 is on the first 96 seeds, where greedy wins 77; there
  5 (79.2) is closer to today's rate. Greedy is not a non-building team under half cost:
  it builds 0.8.
- **Length:** the target is 8-10 rounds (`docs/principles.md`), not the 7-8 reported as a
  pass; `pool_reduction` 6 takes the engine to 6.9.
- **Statistics:** the "dose-response" is four points, each ±5, on seeds where base is
  −6.2 against −2.5 over 240. The ranked table isn't strictly ranked (`fc4`, +4.2, sits
  below two +2.1 rows) and has no multiple-comparison note for about 28 levers × 2
  difficulties. On the 144 fresh seeds alone: `econ_half` + `insight_inc` d4 80.6 against
  greedy 68.8, d0 97.2 / 95.1; `insight_inc` d4 74.3 / 66.7 (p 0.09); base 66.7 / 66.7.

- Greedy builds no economy building under `insight_inc`, and 0.8 a game under the
  half-cost combination (cheap Solar Field). Its win rates stay at the base's, so the
  difficulty targets hold for a team that doesn't build.
- **A building team gains 12 points at d4.** At 82.5% the top difficulty is no longer
  "narrow", so d4 needs its bite back (007 proposal 3).
- d0 stays at 96–97% for both.
- Games keep their length (7.0–8.0 rounds). Adversary operations and decisions per game
  barely move.

Cheaper buildings, stronger credit or energy income, an earlier tier-2 unlock, cheaper
tier 2, more antimatter, the keep cost and a longer game do not do it on their own.
Under the cost and income levers the engine builds 2.4–3.4 economy buildings a game
and wins no more, or less.

**Why:** credits are not scarce for the seat that has the building slots. From 32 traced
games (`results/payback-009-resources.txt`, `scripts/payback/resources.py`):

| At the last adversary phase | Builder credits | Builder insight | Protector credits | Protector insight |
|---|---|---|---|---|
| greedy, d0 / d4 | 56 / 56 (34–89) | 0.5 / 0.8 | 6 / 4 | 2.0 / 1.8 |
| engine, d0 / d4 | 53 / 48 (1–99) | 0.2 / 0.0 | 12 / 12 | 1.5 / 1.9 |

The Builder (6 building slots, credit track +1…+6, factories at 3 each) ends the game
with 50-odd unspent credits, and its insight is gone every round. Insight caps how many
cards a seat can play, and the game is won by playing opposition cards. A building that
adds credits adds nothing the Builder can spend. The Protector is short of credits but
has 3 slots. Credits can't be given (only insight can), so the Builder's surplus can't
reach it. An income building pays back only in the resource that limits plays.

## Ranked levers

Engine minus greedy, paired on 96 seeds (bold: sign test p < 0.05). "Econ" means the
economy buildings the engine has standing at the end, per game. Rounds and tier 2 are
the engine's. Luck is the seed's share of win/loss variance; with only two similar
agents it is mostly seed, so compare it between levers, not with 008's five-agent
figures. Base on these seeds: d4 greedy 77.1, engine 70.8 (−6.2).

| # | Lever (strength) | d0 greedy / engine | d4 greedy / engine | d4 engine − greedy | Econ d4 | Tier 2 d4 | Rounds d0 / d4 | Luck d0 / d4 | Cost |
|---|---|---|---|---|---|---|---|---|---|
| 1 | ◆ `econ_half` + `insight_inc` | 99.0 / 93.8 | 72.9 / 85.4 | **+12.5 ±5.0** (p 0.02; 240 seeds: **+12.1 ±2.9**, p < 0.001) | 3.32 | 2.78 | 7.7 / 7.1 | 48 / 62% | d4 too easy for a building team (82.5 over 240). d0: −5.2 on these 96 seeds, −0.8 over 240. Greedy builds 0.8 economy buildings here too (cheap Solar Field) |
| 2 | ◆ `insight_inc` + `keep_cost` 0 | 95.8 / 96.9 | 82.3 / 88.5 | +6.2 ±4.6 | 2.48 | 2.42 | 7.8 / 7.0 | 48 / 58% | d4 too easy (88.5): needs d4 retuned |
| 3 | ◆ `insight_inc`: income buildings +1 insight | 95.8 / 95.8 | 77.1 / 81.2 | +4.2 ±4.7 (240 seeds: **+6.2 ±3.1**, p 0.06) | 2.07 | 2.52 | 8.0 / 6.9 | 61 / 68% | One more income line on six cards; insight is already set each round from the track plus buildings |
| 4 | ◆ `insight_credit`: only Fab Line, Trade Office, Megastructure +1 insight | 95.8 / 99.0 | 77.1 / 79.2 | +2.1 ±4.7 (d0 +3.1) | 1.29 | 2.42 | 8.2 / 7.1 | 49 / 70% | The weaker strength: half the effect |
| 5 | ◆ `insight_inc` + `factory_credits` 4 | 96.9 / 99.0 | 78.1 / 80.2 | +2.1 ±5.5 (d0 +2.1) | 2.23 | 2.45 | 7.9 / 7.0 | 49 / 56% | Factories lift greedy as much as the engine |
| 6 | `economy.factory_credits` 4 | 96.9 / 93.8 | 78.1 / 82.3 | +4.2 ±5.1 | 1.11 | 2.41 | 8.0 / 7.4 | 48 / 61% | None; greedy also rises at 5 (d4 82). Factories, not buildings |
| 7 | `economy.factory_credits` 5 | 96.9 / 97.9 | 82.3 / 84.4 | +2.1 ±5.1 | 1.10 | 2.34 | 7.8 / 7.3 | 49 / 55% | d4 gets easier for both |
| 8 | `cards.keep_cost` 0 | 95.8 / 96.9 | 82.3 / 83.3 | +1.0 ±5.0 | 1.18 | 2.21 | 7.8 / 7.0 | 48 / 58% | Removes the keep payment (less table cost); d4 +5 for greedy |
| 9 | `keep_cost` 0 + `factory_credits` 4 | 94.8 / 96.9 | 82.3 / 83.3 | +1.0 ±4.8 | 1.26 | 2.27 | 7.7 / 7.0 | 48 / 62% | As keep 0 |
| 10 | `ai.start_credits` 18 | 95.8 / 91.7 | 78.1 / 79.2 | +1.0 ±5.2 | 1.15 | 2.04 | 8.1 / 7.0 | 56 / 61% | None |
| 11 | ◆ `antimatter`: Refinery 2 energy, track's 5th value +1 antimatter | 94.8 / 97.9 | 71.9 / 72.9 | +1.0 ±4.6 | 0.95 | 2.47 | 8.2 / 7.1 | 63 / 75% | Tier 2 +0.3 a game; not an engine |
| 12 | `keep_cost` 2 | 79.2 / 58.3 | 44.8 / 44.8 | 0 | 0.97 | 2.32 | 8.5 / 7.2 | 59 / 73% | **Breaks d0** (greedy 79, engine 58) |
| 13 | ◆ `insight_inc` + `stage_cards` 4,4,4 | 97.9 / 90.6 | 78.1 / 77.1 | −1.0 ±5.2 (d0 **−7.3**) | 2.50 | 2.97 | 8.5 / 7.3 | 57 / 63% | A longer deck undoes the insight lever: the engine over-invests |
| 14 | ◆ `econ_noinsight`: economy buildings cost 0 insight | 95.8 / 96.9 | 76.0 / 74.0 | −2.1 ±4.7 | 1.40 | 2.33 | 8.2 / 7.2 | 48 / 72% | Spending less insight on a credit building doesn't help either |
| 15 | `economy.processor_compute` 2 | 99.0 / 99.0 | 85.4 / 83.3 | −2.1 ±3.9 | 0.99 | 2.48 | 8.1 / 6.9 | 49 / 72% | d4 easier for both (more draws) |
| 16 | `momentum_drones` 0, `escalation_outposts` 0 (slower spread) | 99.0 / 95.8 | 75.0 / 71.9 | −3.1 ±5.4 | 0.95 | 2.16 | 8.2 / 7.0 | 69 / 64% | Games no longer |
| 17 | ◆ `unlock_m1`: tier-2 mark one track value earlier | 96.9 / 86.5 | 68.8 / 65.6 | −3.1 ±5.8 (d0 **−10.4**) | 0.93 | 2.15 | 8.3 / 7.3 | 52 / 63% | Tier-2 plays don't rise; d0 engine falls |
| 18 | ◆ `econ_half`: economy buildings half cost | 97.9 / 93.8 | 74.0 / 69.8 | −4.2 ±5.5 | 2.53 | 2.20 | 7.9 / 7.2 | 48 / 64% | Engine builds 2.5; no gain |
| 19 | ◆ `bld_all_m3`: every building −3 | 92.7 / 91.7 | 72.9 / 68.8 | −4.2 ±5.1 | 2.43 | 2.17 | 8.2 / 7.2 | 60 / 70% | As above |
| 20 | ◆ `income_p1` / `income_x2`: economy income +1 / ×2 | 95.8 / 92.7 | 77.1 / 71.9 | −5.2 ±5.2 (both) | 2.40 / 2.69 | 2.43 / 2.45 | 8.3 / 7.4 | 57 / 66% | Engine builds most; no gain |
| 21 | ◆ `overclock_cheap`: Overclock Rig 6, no upkeep | 96.9 / 93.8 | 77.1 / 71.9 | −5.2 ±5.0 | 0.94 | 2.17 | 8.3 / 7.2 | 59 / 68% | Still rarely built |
| – | **base** | 95.8 / 94.8 | 77.1 / 70.8 | −6.2 ±5.1 | 0.96 | 2.16 | 8.1 / 7.1 | 59 / 68% | |
| 22 | `cards.draw_cost` 2 | 97.9 / 96.9 | 87.5 / 80.2 | −7.3 ±4.3 | 0.92 | 2.46 | 7.8 / 6.7 | 69 / 67% | d4 easier, mostly for greedy |
| 23 | `keep_cost` 0 + `stage_cards` 4,4,4 (± `factory_credits` 4) | 97.9 / 97.9 | 86.5 / 80.2 | −6.2 ±4.6 (−7.3 with fc4) | 2.02 | 2.75 | 8.3 / 7.5 | 49 / 62% | Longer, still no engine win |
| 24 | `adversary.stage_cards` 4,4,3 / 4,4,4 / 4,4,6 (longer game) | 97.9 / 91.7–85.4 | 78.1 / 68.8–65.6 | −9.4 / **−12.5** / −9.4 (d0 −6 / −7 / **−12.5**) | 1.3 / 1.5 / 2.0 | 2.5–3.0 | 8.5–8.9 / 7.4–7.6 | 47–58 / 61–67% | Engine builds more and loses more; greedy still wins in 7.8 rounds |
| 25 | `keep_cost` 3 (v0's) | 64.6 / 9.4 | 31.2 / 12.5 | **−18.8** (d0 **−55.2**) | 0.18 | 1.91 | 8.0 / 6.9 | 39 / 60% | Breaks the game; credits are scarce early |
| 26 | ◆ `t2_half` + `t2_noam`: tier 2 half cost, no antimatter | 97.9 / 61.5 | 67.7 / 35.4 | **−32.3** (d0 **−36.5**) | 1.10 | 2.17 | 9.2 / 7.7 | 40 / 47% | **Agent artifact** (unverified hypothesis), below |

Strategy health in 008's terms, for the headline lever against the base (d4):
- Buildings played: engine 3.64 a game (base 2.51), greedy 1.82 (Turnkey Factory and
  Server Farm only).
- Tier 2 played: 2.52 (base 2.16).
- Engine wins with 2+ buildings played: 96% (base 87%). 008's measure counts Turnkey
  Factory and Server Farm, so it is 85–100% for every lever and every agent; the
  economy-building count above is the one that separates them.
- Adversary operations per round: 10.3 (base 11.1).
- Decisions per game: 237 (base 234).

## Findings by lever

- **Cheaper buildings** (◆ `bld_all_m3`, ◆ `econ_half`) and **stronger income buildings**
  (◆ `income_p1`, ◆ `income_x2`): the engine plays far more of them (2.4–2.7 economy
  buildings standing at d4, against 0.96) and wins 4–5 points *less* than greedy.
  Credits go unspent (above), so cost and credit yield are not what hold the engine back.
- **Earlier tier-2 unlock** (◆ `unlock_m1`; the Protector then has tier 2 from the
  start): tier-2 plays don't rise (2.15 at d4), because tier-2 cards still need
  antimatter. At d0 the engine falls 10 points: it drafts from tier 2 and misses the
  tier-1 opposition cards.
- **Cheaper tier 2** (◆ `t2_half`+`t2_noam`): greedy is barely touched at d0 (97.9) but
  drops 9 points at d4. The engine collapses: 46 cards a game (35 at base), mostly
  Counter-intelligence (98 plays over 96 games at d0) and Purge Protocol, and 26 of 96
  d0 games run out the deck. Unverified hypothesis: this is the agents' known blind spot
  (no term for the adversary deck running out, 007 proposal 10), not a verdict on the lever. **Unmeasured
  for a building team.**
- **More antimatter** (◆ `antimatter`): tier 2 +0.3 a game, +1 point; not an engine
  lever while tier 2 is mostly Purge Protocol.
- **A longer game** (`stage_cards` up to the printed 4,4,6; slower spread):
  - Longer decks make the engine invest more (2.0 economy buildings standing at 4,4,6)
    and lose more: −9 to −12.5 against greedy.
  - The engine's horizon grows with the deck, so it builds for rounds it doesn't
    survive. Its d0 losses at 4,4,6 are corruption in rounds 9–13 while greedy wins in
    7.8.
  - With the spread slowed down (`calm`), games are no longer than at the base.
  - There is no pipeline-speed parameter. The pipeline's three slots are code.
- **Keep cost:** 0 helps both agents (+5 at d4) and the engine slightly more (+1 vs
  −6 at base). 2 and 3 wreck d0 for both, the engine most; unverified hypothesis: because
  credits are short in rounds 1–2 (the Builder starts with 12). 007's keep cost of 1 sits next to a cliff.
- **Config proxies** (`factory_credits` 4–5, `processor_compute` 2, `draw_cost` 2,
  `start_credits` 18) make d4 easier for both agents by up to 10 points and leave the
  engine-vs-greedy gap within noise. `factory_credits` 4 is the best config-only lever
  for the engine (+4.2 at d4, p 0.54).

**Not reachable by config:** building costs, building income, tier-2 costs, the
tier-2 unlock and antimatter on the AI tracks, and the Refinery rate all live in
`content/`. They were run as content patches, as 007's `cards-t2cut` was (the ask's
default; see the report). A slower pipeline isn't a parameter. Moving credits between
players, the one sink the traces point at, would be a rule.

## Combinations worth a design change

1. **Half-cost economy buildings that also give +1 insight** (◆ `econ_half` +
   `insight_inc`). The clearest engine:
   - +12.1 at d4 over 240 seeds (p < 0.001), with 3.4 economy buildings standing and
     96% of the engine's d4 wins on 2+ of them.
   - Even at d0.
   - Cost alone does nothing (`econ_half`: −4); together with insight income it doubles
     the insight lever's effect.
   - Costs: six cards get a new cost and a "+1 insight" line, read at round start like
     the track's insight. Games stay 7–8 rounds.
   - A building team then wins 82.5% at d4, so d4 needs about 10 points of bite back.
     **Config can give it back:** `adversary.pool_reduction` (007's d4 knob, 4 today)
     at 5 or 6 (96 seeds, `results/payback-009-content/econ_half+insight_inc+d4tune`):

     | `pool_reduction` | d4 greedy | d4 engine | engine − greedy | Engine rounds | Econ standing |
     |---|---|---|---|---|---|
     | 4 (today) | 72.9 | 85.4 | **+12.5** | 7.14 | 3.32 |
     | 5 | 64.6 | 79.2 | **+14.6** (p 0.01) | 7.04 | 3.32 |
     | 6 | 60.4 | 69.8 | +9.4 (p 0.11) | 6.88 | 3.33 |

     At 6 (unverified hypothesis, 96 seeds, p 0.11), a building team lands on today's d4 rate
     (about 70%) and a non-building team falls to 60%: building becomes the way to beat the top difficulty. The cost is
     about 0.3 rounds shorter at d4.
2. **Income buildings give +1 insight, at today's costs** (◆ `insight_inc`): the smaller
   change.
   - +6.2 at d4 (p 0.06, 240 seeds), 2.05 economy buildings standing. d4 lands at 77% for
     a building team against 71% for one that doesn't: still "narrow".
   - The dose-response is clean: base −6.2 → credit buildings only (◆ `insight_credit`)
     +2.1 → all six income buildings +4.2 → plus half cost +12.5 (96 seeds each).
3. **`insight_inc` + `keep_cost` 0**: the engine leads by 6 (88.5 vs 82.3 at d4).
   Removing the keep payment cuts a step at the table, but it lifts both agents 5–11
   points at d4, so this also needs d4 retuned.

Combinations that don't work:
- A longer deck with `insight_inc` (−1 at d4, **−7.3** at d0): the engine over-invests.
- `factory_credits` 4 lifts greedy as much as the engine.

The design reading: an engine pays back when what it produces is what limits the team.
That is plays (insight), or a way to move the Builder's surplus credits to the
Protector. Cheaper, bigger credit income does not pay back.

## Records worth a look

The files named here were deleted in 038 (see *Results* above); what they showed stands.

1. `PATCHED-econ_half+insight_inc-d4-engine-seed300038.json`, **patched-content build**
   (won't replay on main): the best combination.
   - The engine builds Assembly Cluster, Microgrid, Solar Field and Trade Office on the
     Builder, and Fab Line on the Protector (the credit-starved seat). It wins on
     opposition in round 8.
   - At the baseline both agents lose this seed on corruption (the engine in round 6),
     and greedy loses it under the patch too.
2. `PATCHED-insight_inc-d4-engine-seed300041.json`, **patched-content build** (won't
   replay on main): the insight lever at today's costs.
   - The engine builds Fab Line, Grid Battery and Microgrid on the Builder, and Server
     Farm and Solar Field on the Protector, and wins on opposition in round 8.
   - Greedy loses this seed (deck out, round 9).
3. `base-d4-engine-seed300041.json`, main: the same seed at the baseline. The engine
   builds Grid Battery and Microgrid and runs out the deck in round 9. Same table, same
   engine; only the insight differs.
4. `base-d4-engine-seed300094.json` with `base-d4-greedy-seed300094.json`, main: at the
   unchanged baseline the engine's Fab Line and Solar Field win on opposition in round
   8, and greedy loses on corruption in round 7. The rare game where a credit engine
   already decides it.
5. `base-d0-engine-seed300008.json`, main: the price of building. The engine puts Fab
   Line, Microgrid and Solar Field on the Builder and loses on corruption in round 7,
   while greedy wins at d0.
6. `deck446-d0-engine-seed300080.json`, main (config `stage_cards` 4,4,6): a longer game
   makes the engine over-invest. Four economy buildings, and it loses on corruption in
   round 13 of a 14-card deck. Greedy wins the seed.
