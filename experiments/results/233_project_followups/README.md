# 233: project follow-ups, 231's picks (r103)

`RULES_REVISION` 102 → 103, bumped from the constant. Matt's rulings from 231 stand (one-shot, every country, compute
per board, 3-4 big and 5-6 small a d2 game, no reshuffle in d3; bands soft). Every number is a candidate.

## What changed

- **Big-project potency at the same costs** (231's screen-5 overlay, as asked): Mob Rule Sway 5; Data Heist 4 tokens,
  10 credits or 3 cards; Hostile Takeover and Grand Offensive corrupt with 1 fewer (Grand Offensive both corrupts,
  amendment 10375); Recruitment Drive Rally 3; Propaganda Network Sway 8. No card overshot, so none was cut back.
- **Deck 28 → 30** (18 small, 12 big): Rumor Mill (small, diplomacy, Sway 4) and Martial Law (big, security, Rally 3
  stations, Corrupt stations, 15), from the prompt's description: 231's drafts weren't in the repo.
- **Small projects +2 a board** (the prompt asked +1). +1 still resolved 6.5 small a d2 game on the fresh read f233,
  so a second +1 was screened (s233c) and confirmed (f233b) under Matt's message 10433. Rumor Mill: drafted at 6,
  printed at 8 (amendment 10375's "+1 applies", then the second +1).
- **Small fixes:** `Op::ExtraWar` lost its dead `front` / `ongoing_use` fields and `FrontPick`; `ProjectCard.ongoing` is
  gone (content with it is refused by the parser), as is the dead `resolve` key in the loader and checker. A production
  per tag is valued at the tags in play now (`income_frozen_tag`, as `IncomePerTag`, 229); the live per-tag term is gone.
- **Team-op pricing test** (231's IncreaseIncome miss): `Op::team_reach` declares each project op's reach (each
  country or once). `project_ops_reach_as_declared` (aic-core) measures it on the engine on a 2-country board, and
  `team_ops_price_per_country` (aic-sim) prices each op at 1 and 2 countries against it. It found two value misses,
  now fixed: a project's `deal_threats each_country` (False Flag, Accelerated Timetable) and War Plants'
  `adversary_factory` were priced once. Five arms (corrupt, requisition, lose_tokens, lose_ambassador,
  adversary_build) price a threat's one country by design; the project give / get multiplies them. The Python mirror
  missed Corrupt's countries, now fixed in `projects_233.py`.

## Give / get (values.json; give = cost a board × 2 countries × compute 1.1)

`python experiments/results/233_project_followups/projects_233.py` prints all 30. The touched cards:

| Project | cost a board | give | get | get / give |
|---|---|---|---|---|
| Mob Rule | 14 | 30.8 | 39.8 | 1.29 |
| Data Heist | 12 | 26.4 | 28.0 | 1.06 |
| Hostile Takeover | 15 | 33.0 | 49.8 | 1.51 |
| Recruitment Drive | 20 | 44.0 | 46.2 | 1.05 |
| Propaganda Network | 18 | 39.6 | 40.8 | 1.03 |
| Grand Offensive | 22 | 48.4 | 73.2 | 1.51 |
| Rumor Mill (new) | 8 | 17.6 | 16.0 | 0.91 |
| Martial Law (new) | 15 | 33.0 | 39.8 | 1.21 |

The small projects' get / give falls by about an eighth (+2 on costs of 5-12). A project is the adversary's: "gain" is
harm per compute, and every card harms (none is a no-op at the table).

## Screens (spent seeds, paired against r102)

| screen | seeds | cell | strategist d2 | d2 small / big | d3 | note |
|---|---|---|---|---|---|---|
| s233a | 47088-47135 | stated (asked numbers) | 66.7 (−5.6 ±4.7) | 5.73 / 3.74 | 41.0 | no reshuffle; most draws 28 |
| s233a | same | big potency off | 77.1 | 5.88 / 3.89 | 48.6 | potency is worth ~10 points at d2 |
| s233a | same | Rumor Mill 6 | 68.1 | 5.71 / 3.75 | 44.4 | |
| s233b | same | P2 (Mob Rule Sway 6, Propaganda 10, Recruitment Rally 4, Data Heist 5 / 12) | 63.9 | 6.01 / 3.88 | 44.4 | |
| s233b | same | P3 (P2, Rally 4 on Mob Rule, Hostile Takeover, Martial Law) | 66.0 | 5.86 / 3.85 | 42.4 | more potency is flat |
| s233c | 47088-47183 | stated | 64.9 (−4.5 ±3.3) | 6.10 / 3.76 | 39.2 | |
| s233c | same | small +2 | 61.8 (−7.6 ±3.4) | 5.71 / 3.59 | 39.6 | adopted |

Past the stated numbers, potency doesn't move d2 (P2, P3), so it shipped as asked.

## Fresh reads (2 players, B+P / B+R / P+R, 48 seeds a cell, d0-d3, against r102 on the same seeds)

**f233** (47136-47183, the +1 build; spent once the build changed): strategist 82.6 / 77.1 / 63.2 / 37.5
(d3 −18.1 ±5.5); d2 6.47 small / 3.77 big (small over target). Greedy d2 6.06 / 3.55.

**f233b, the confirm** (47184-47231, the shipped build), 144 games a row:

| agent | d | r103 win % | r102 | gap ±SE | band | d small / big | r102 small / big |
|---|---|---|---|---|---|---|---|
| strategist | 0 | 84.7 | 84.0 | +0.7 ±4.1 | 85+ unmet by 0.3 | 1.80 / 0.71 | 2.01 / 0.91 |
| strategist | 1 | 81.9 | 75.7 | +6.2 ±4.5 | 65-80 unmet | 3.52 / 1.77 | 3.81 / 2.15 |
| strategist | 2 | 79.2 | 73.6 | +5.6 ±4.7 | 50-65 **unmet** | 5.68 / 3.40 | 6.88 / 3.88 |
| strategist | 3 | 48.6 | 48.6 | +0.0 ±5.2 | 35-50 met | 8.34 / 5.15 | 9.67 / 5.72 |
| greedy | 0 | 82.6 | 82.6 | +0.0 ±4.1 | | 1.72 / 0.72 | |
| greedy | 1 | 78.5 | 79.9 | −1.4 ±4.4 | | 3.29 / 1.71 | |
| greedy | 2 | 69.4 | 72.2 | −2.8 ±4.8 | | 5.72 / 3.32 | |
| greedy | 3 | 54.2 | 51.4 | +2.8 ±5.2 | | 8.78 / 5.38 | |

- **Throughput met:** d2 resolves 5.7 small and 3.4 big (both agents); 231's r102 resolved 7.0 / 3.5.
- **No reshuffle in any game:** the most draws in a d3 game were 27 of 30 (24 at d2).
- **d2 band unmet.** The seed sets disagree: small +2 against r102 is −7.6 ±3.4 on the 96 screen seeds and +5.6 ±4.7
  here. Pooled, d2 sits a few points under r102, around 66-70 against a 65 ceiling. The prompt's rule held throughput
  first, and Matt's 10433 allowed one confirm, so the miss is reported. The levers left in scope (potency past the
  asked numbers, s233b) don't move d2.
- **d3:** f233 read −18 on its seeds and f233b reads 0; s233c's 96 seeds read −9.4 ±3.7. d3 sits in its band on
  every read.
- **Per project** (both agents pooled, per difficulty): [projects.md](projects.md). Every project completes at d2-d3;
  Rumor Mill 0.30 and Martial Law 0.21 a d2 game (f233b).
- **Resource use:** [resource-use.md](resource-use.md); within a few percent of r102 at every level.
- **Design check:** [design-check.md](design-check.md). r102 trips the same warnings on these seeds, except Builder
  Fortify 0.21 (band 0.25+).

## Checks

- Loop smoke (`loop_smoke.py --run`): no timeouts; most plays of one card 4. `npm run cards:fit` passes.
- ProductionPerTag at play tags, on / off, 16 dev seeds (36800), `use.json`: the strategist plays the same games. The
  engine agent reads 75/31/50 against 69/38/50 by pair, within noise. Per-tag cards are played a little less (Economies of
  Scale 41 → 27 plays in 96 games). The eval term mirrors `effects::ProductionPerTag`, which counts tags in play
  once, at resolve; the card's own tags count through `own_tags`.
- Speed (`per_decision.py`, r102 against r103, d2, 2 countries; other games, so per decision): greedy 1.06× ms a
  decision on 30 games (options 1.02×; 12 games read 1.23×, the game mix), strategist 0.56× on 4. `speed.py check`
  gives no verdict across the revision; the baseline was updated (`--update`) on the shipped build.

## Recipes

- **Base side for free:** 231's kept `new-f231b.jsonl` is r102 on 47088-47135. The r102 binary replays it exactly
  (one seed checked, all 24 cells, timing fields aside), so s233a and s233b reused it as base.
- **Content screens on one build:** overlays in each variant's `set` (`projects.mob_rule.effects.1.amount=6`). Cards
  can't be added by overlay, so the new deck was the build and 231's numbers were the overlay.
- **Readers:** [`gaps.py`](gaps.py) (an arm against r102, paired, with small and big resolutions; several base
  files), [`projects_read.py`](projects_read.py) (195's, without the ongoing columns),
  [`projects_233.py`](projects_233.py) (the deck as an overlay on the draft, with give / get). 231's `arms.py` misreads
  a second base file's gaps: use `gaps.py`.
- **Cost:** a 1,152-game read (one build, 3 shards) took 9-11 min beside its twin. 3,456 games on 6 shards took 14 min.
