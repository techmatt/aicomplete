# 267: a difficulty ladder players can track (r125)

Matt's rulings (2026-11-17): Spirit Island's adversary card. One reference per adversary, cumulative named levels,
about one new mid-game rule a level and three active at the top (soft), every other change setup, numbers where
they're used, the arithmetic modifiers cut or merged; the ladder only adds. Bumped from the constant, 124 to 125.
Base: main f4ba70b (r124; 264-266 changed no play, so 263's numbers stand).

## The ladder

| Level | Name | Setup (set once; incomes and threshold a country) | Mid-game rule it adds | Active |
|---|---|---|---|---|
| 0 | The Militarist | Compute income 0; agent income +0; research threshold 5; deck 5 / 5 / 3 (13 rounds); sites print city 5, station 2, factory 2 | — | 0 |
| 1 | Sleeper Cells | Compute income 3; research threshold 4 | When a Corrupt step checks a city, it falls at 3 agents, not 5; a card's Botnets spread line now takes effect | 1 |
| 2 | War Footing | Compute income 5 | When a threat executes uncountered, the adversary also gains 4 compute | 2 |
| 3 | Arms Race | Compute income 15; deck 5 / 3 / 3 (11 rounds) | When Weapons Program fires, it musters 3, not 2 | 3 |
| 4 | War Economy | Compute income 18; agent income +1; deck 3 / 4 / 4 (11 rounds) | — | 3 |

`crates/aic-sim/src/docs/ladder.rs` builds this one table for `docs/rules/adversary.md`, rulebook §21 and
`web/card/adversary.json`. `docs --check` refuses a mid-game dial (`botnet_spread_at`, `corrupt_cut_at`, `city_cut_at`,
`board_difficulty_at`, `research_strong_at`) at a level whose content names no rule. It also refuses a rule whose
printed numbers ("3 agents, not 5", "4 compute", "musters 3, not 2") or "Botnets spread" aren't the engine's, and a
`corrupt_cut` on a station, factory or land. **Counted honestly** (second diff review), Sleeper Cells merges two
mid-game changes (a city at 3, botnets spread: Matt's own L1 example as one named rule). So named rules active run
0 / 1 / 2 / 3 / 3, but changes run **0 / 2 / 3 / 4 / 4**. `design_check.py`'s `ladder` check counts changes (one a
mid-game dial) and flags level 1 (2 new) and the top (4 active) against the soft budget of about 1 and 3.

## What changed

- **Cut, no mid-game arithmetic left:**
  - the two counter surcharges (`counter_bonus*`, `counter_more*`: d1 credits +4 and other items +1, d2 +1 more);
  - the d2 Muster +1 (the board's `standing_troops_bonus`; the op stays, printed by no content);
  - d4's tier-2 look-ahead (`tier2_early`).
- **Tier-2 sooner is the deck's build:** at d4 a stage-II card takes a stage-I card's place (3 / 4 / 4:
  `stage1_removed` 2, new `stage2_added` 1). Tier 2 still arrives from round 4, as the look-ahead dealt it.
- **Sites print** a station's and a factory's corrupt number at 2 at every level (r124: 3 at d0, 2 from d1). The
  city's 5 → 3 is Sleeper Cells (`corrupt_cut` 2, 0, 0, 0), Matt's own example of a level-1 rule.
- **Two rules replace a number as they fire:** Sleeper Cells' city at 3 and Arms Race's Muster 3. Each prints its
  number whole and fires on a step players already resolve. They are the only number changes left mid-game.
- **Difficulty recovered with setup markers:** the research marker at 4 from level 1 (`research_at` 5, 4, 4, 4, 4;
  was 4 from d3) and compute income 15 at d3 (`compute_income` 0, 3, 5, 15, 18; was 13).
- **Named levels in content** (`drafts/adversary/militarist.json` `levels`: name, flavor, rule). The board's
  `difficulty` line is level 2's rule, War Footing.
- **The reference card:** a new face kind (`adversary_board`, `web/cards/reference.js`), 100 × 140 mm, from
  `web/card/adversary.json`. It prints the rules, the research rule, the escalation (the deck's stages), the setup
  and the ladder: [militarist-reference.png](militarist-reference.png).
- **Harms:** `RULE_HARMS` names Sleeper Cells (city lost), War Footing and Arms Race on their rows (`docs/harms.md`).

## Screens

Dev seeds 36800-36831, 192 games a cell ([screens.md](screens.md)). Cutting the surcharges, Muster +1 and the
look-ahead cost 3-6 points at d1-d4. Station and factory at 2 everywhere held d0 (93.2 → 95.3). Compute income
alone recovered the levels: d1-d2 at 7, 8, then d3 15. d4's 3 / 4 / 4 deck screened at 20, 18 and 16; 18 shipped
(28.1%, 7.93 rounds, against r124's 25.5% and 7.70).

**First fresh read, spent** (`f267-read`, 49596-49643, compute 0, 7, 8, 15, 18). Every band was met and every judge
gap was within about 2 SE. But adversary ops rose 11% a round (d1 +44%, d2 +19%), over design_check's 10%, against
the standing fewer-tokens ruling: every compute point is an op. Screen 5 put r124's d1-d2 compute back with the
research marker at 4 from d1: d1 ops 20.5 (r124 20.4), d1 win 90.1 (base 89.6), d2 83.9 (base 79.2). That shipped,
confirmed on the next free seeds.

## The confirmation read (shipped)

`f267-read2`, 49644-49691, three pairs, d0-d4, judge and greedy, 1,440 games a side. r124 (main f4ba70b) ran
natively on the same seeds with its own configs ([read.md](read.md), [spread.md](spread.md)):

| Level | Judge r125 | r124 | gap | Greedy r125 | r124 | gap | Band (judge) |
|---|---|---|---|---|---|---|---|
| d0 | 97.2 | 95.1 | +2.1 ± 2.1 | 96.5 | 96.5 | +0.0 ± 2.0 | ≥ 90 met |
| d1 | 91.0 | 92.4 | -1.4 ± 2.8 | 82.6 | 85.4 | -2.8 ± 4.1 | 80-95 met |
| d2 | 87.5 | 87.5 | +0.0 ± 3.1 | 84.0 | 82.6 | +1.4 ± 3.9 | 75-90 met |
| d3 | 65.3 | 62.5 | +2.8 ± 4.2 | 66.0 | 61.8 | +4.2 ± 4.0 | 55-70 met |
| d4 | 33.3 | 36.1 | -2.8 ± 4.3 | 38.9 | 29.2 | +9.7 ± 4.9 | - |

Against 263's read (49548-49595, judge 95.1 / 95.1 / 86.1 / 63.2 / 22.9): every level is within its band. d4's r124
side itself ran 36.1 on these seeds (263's r124: 22.9), so d4's level is seed noise, and the gap is the measure. Greedy at d4
gains 9.7 (2.0 SE). The 3 / 4 / 4 deck is the likely mover: stage-II cards add agent income instead of a stage-I
card's tier-1 threat.

Pooled, judge and greedy (r124 → r125):

| Level | rounds | deck-empty | games with a new city | new cities a game | adversary ops a round |
|---|---|---|---|---|---|
| d0 | 8.22 → 8.16 | 4% → 3% | 6% → 6% | 0.07 → 0.08 | 9.7 → 9.9 |
| d1 | 9.40 → 9.54 | 10% → 11% | 59% → 60% | 1.08 → 1.15 | 20.2 → 20.3 |
| d2 | 9.05 → 9.05 | 10% → 10% | 70% → 73% | 1.59 → 1.53 | 32.2 → 31.2 |
| d3 | 8.32 → 8.39 | 15% → 16% | 85% → 87% | 1.98 → 2.06 | 51.5 → 54.4 |
| d4 | 7.74 → **8.01** | 9% → 10% | 96% → 97% | 2.87 → 3.00 | 65.8 → 65.3 |

Spread targets (candidates) met: d0 6% of games with a new city (≤ 20%), d3 87% and d4 97% (≥ 70%). d4 now runs 8.0
rounds, inside 8-10 for the first time since r121. Adversary ops overall 35.1 → 35.6 a round (+1.4%).

**Full oracle** (`f267-oracle2`, 49644-49655, d2-d3): 72/72, 10.3 and 9.5 rounds (263: 72/72, 10.8 and 9.2). On the
first read's build (`f267-oracle`, 49596-49607): 72/72, 9.8 and 9.25.

**Speed** (`bench/per_decision.py`, 8 games, d2, one thread, box loaded: ratios only): the judge 1.9 → 1.8 ms a
decision (0.97×); greedy 2.5 → 1.8 (0.72×, fewer options a decision on these 8 games) ([speed.md](speed.md), the
judge's table first). `speed.py check` gives no verdict across a rules revision; nothing slowed.

Kept: `out/keep/267/` (both fresh reads and both r124 sides, both oracles, the six dev screens). Seeds 49596-49691
taken (ledger next free 49692).

## Warnings

`design_check.py` on the confirmation read against r124:
- **ladder-new** level 1 (2) and **ladder-active** level 4 (4): Sleeper Cells' two changes (above). This item's own
  check; asked of Matt (default: keep).
- **dials-hide-design** (`corrupt_cut`, `stage1_removed`): this item's design is those dials.
- **usage**, Protector Bulwark 0.23 and Make antimatter 0.01: both are on the r124 side too.
- The first read's tokens +11% is gone (+1.4%). r124's `rounds d4 7.8` is gone too (8.0).

## Open

- **Two instead-rules (Sleeper Cells, Arms Race)** stay named rules. The alternative is a site printing 5 / 3 with a
  level token. Matt's call.
- **`Op::StandingTroopsBonus`** is printed by no content now; it stays for a later adversary's level rule.
- **Greedy d4 +9.7** on the confirmation seeds (2.0 SE), the judge -2.8: a d4 read after the next d4 change will show
  whether the deck build moved greedy alone.

## For new players

Each difficulty level is named, and each keeps every level below it. Before the game you set the adversary's markers
(compute income, agent income, research threshold) and build its event deck as the level's row says; nothing else
changes how threats are priced. At most one new rule joins at each level, and it fires only on something you already
resolve:
- **Sleeper Cells** (1): a city falls at 3 agents, and botnets spread.
- **War Footing** (2): an unpaid threat feeds the adversary 4 compute.
- **Arms Race** (3): Weapons Program musters 3.
- **War Economy** (4): no new rule: a harder deck (tier-2 threats from round 4) and higher incomes.

Stations and factories fall at 2 agents at every level. No rule resisted a summary.
