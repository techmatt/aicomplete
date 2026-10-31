# One standard placement, a Science niche, a better Develop (046, r22)

Judged by the `planner`, d2. The prompt called this revision r21; 044 took that number. **The gate is
partial:** `experiments/gate-046.json` (`experiments/gate.json` since 052) on fresh seeds from 3000 crashed at 495 games on an engine bug
(fixed here, ruling 3 below). It was resumed on the fixed
binary and cut at 2:44am for time: 623 of 1,984 games, 12-14 seeds a check, not 32. Read each gap as direction, not as a verdict.

## Verdict against "done when"

| Asked | Result | |
|---|---|---|
| Gate passes at d2 | c3: no check ahead. c2: tier-2-rush and ignore-Industry -8 ±7, inside the noise, with the planner at 93 | **not shown** |
| Always-Consolidate loses by more than the noise | c2 +7 ±12, c3 +31 ±18 fresh; 16 seeds before the niche pack +13 / +44 | **c3 about 1.7 SE; c2 no** |
| Ignore-Science loses by more than the noise | c2 0 ±10, c3 +23 ±18 fresh; 16 tuning seeds +12 / +18 | **c3 about 1.3 SE; c2 no** |
| Develop picked by both AIs | Builder 37-44%, Protector 17-19%, overall 28-35% | **met** (the Protector just under the 20% floor) |
| Tokens: mixed 11-13 by r10, always-Consolidate 8-9 | mixed 11.4 by r6, 12.3 by r10; always-Consolidate 7.5 by r6, 9.8 by r10 | met |
| Failure mix: Discord and presence each under about 60% | 22 planner losses over the three seed sets at r22: Discord 16 (73%), presence 6, cap 0 | **Discord over** |

A stopped gate with a dearer tier-2 keep (4, 3 with the upgrade, 12 seeds from 2000) is the only view of
c2 under pressure: planner 83%, always-Consolidate +33 ±18, ignore-Science +50 ±22, every other check
behind or level except ignore-Industry (0 ±15). Its c3 fell to 8%, so that keep was dropped, but the rules
it shares with r22 do separate at c2 once c2 can lose. `gate-keep-4-3.md`.

## r22 at d2

| | c2 | c3 |
|---|---|---|
| win %: tuning seeds 1000-1015 | 100 | 56 |
| win %: seeds 2000-2011 (2000-2015 for c3) | 100 | 33 (38) |
| win %: fresh gate seeds from 3000 (13-14) | 93 | 71 |
| tokens taken a seat by r6 / r10 | 11.4 / 12 | 10.4 / 12.3 |
| growth: Expand / Develop / Consolidate % | 43 / 28 / 28 | 42 / 35 / 23 |
| growth, Builder | 33 / 37 / 30 | 35 / 44 / 21 |
| growth, Protector | 54 / 19 / 27 | 57 / 17 / 26 |
| markers placed / cleared a game | 4.1 / 2.9 | 8.3 / 5.0 |
| niche cards kept / played a game | 3.3 / 2.2 | 3.8 / 2.2 |
| presence lost a game | 8.5 | 17.1 |

The same rules read 33 to 71 at c3 across three seed sets (52% of 44 games), so ±15 on any one set. c2
at 40 of 42 games has no pressure left: r20 was 88, 044's fix 81, and the Science pack
took presence lost from 11.7 to 8.1 a game there.

## The gate (partial, 12-14 seeds; gap = planner minus check, ± one standard error; base 93 / 71)

| check | c2 | c3 |
|---|---|---|
| **always-Consolidate** | +7 ±12 | +31 ±18 |
| always-Expand | +14 ±13 | +33 ±19 |
| never-Remove | +7 ±12 | +31 ±18 |
| Science board rush | +50 ±18 | +33 ±19 |
| Science rush | +33 ±14 | +8 ±17 |
| rich-spend | +33 ±14 | +50 ±21 |
| tier-2-rush | +0 ±10 | +8 ±17 |
| tier-1-only | +62 ±17 | +88 ±12 |
| **ignore-Science** | +0 ±10 | +23 ±18 |
| ignore-Industry | -7 ±7 | +15 ±18 |
| ignore-Cyber | +42 ±14 | +23 ±18 |
| ignore-Power | +29 ±15 | +50 ±21 |
| ignore-Diplomacy | +88 ±12 | +88 ±12 |
| ignore-Security | +29 ±15 | +75 ±17 |
| no-powers | +0 ±10 | +8 ±17 |
| no-counters | +100 ±0 | +88 ±12 |
| ignore-threats | +100 ±0 | +88 ±12 |
| Unity farm | +62 ±17 | +62 ±19 |
| mono-tag ×6, keep-all, keep-2, no-presence, events-only, turtle, free-only | see `gate.md` | see `gate.md` |

**Why c2 can't pass or fail:** near 100% a check can only tie or edge ahead by a game. That is the
structural cause at c2: two seats at d2 have no pressure (the planner won 93-100 on three seed sets). The gate needs c2 near 80% to read, and every lever that got it there (keep 4 and 3, keep 3 and
3, the Protector's Expand at 1 credit) took c3 to 8-38%: c2 and c3 share one difficulty and they are about
35 points apart, so one number can't be right for both. A size-specific lever is the next test.

## Re-checks (ruling 5)

- **Tier-2-rush at c3:** level. Planner against rush, c3: 56 / 62 (16 seeds), 33 / 42 (12), 71 / 63 (14): about
  23 of 42 each. Not a beat, not a clear loss.
- **Ignore-Industry at c2:** -7 ±7 on the fresh seeds and 0 ±15 at keep 4 and 3: Industry is still the one tag
  a c2 team doesn't miss. At c3, +15 ±18.

## Why agents draft tier 2 (ruling 2's question)

Both. A blind spot, now fixed: each pick was priced alone, so a seat drafted more tier-2 cards than its
insight could keep. `w.packet_budget` counts picks only as far as the round's insight keeps them, best
first (c3 69 against 38 with it off, 16 seeds, ±15; the tier share itself barely moved, 25% tier 1 before
and after). The keep plan priced by printed cost, so a 2-credit Science card lost every keep to a tier-2
card; `plan_niche` adds a value for the job on the table. Niche cards kept went from 1.7-2.5 to 3.3-3.8 a game.

The rest is the rules: **tier 2 is simply better.** Planner against rush at three keeps (12 seeds, c2 / c3):
3 and 2: 100 and 100 / 33 and 42; 3 and 3: 67 and 75 / 8 and 17; 4 and 3: 83 and 75 / 8 and 25. A dearer keep
makes the game harder, not the mix better. Tier-1-only loses by 60-90 points. Tier-1 Science now reaches
hands through Develop's draws and the eight niche cards, not through picks.

## Rulings

1. **One placement a round:** `presence_action_cap` 1, per seat. Rich action uses went from 0.2 to 1.4 a
   game at c2 under always-Consolidate and 5 at c3: a small second door, still dear. Kept apart from the cap.
2. **Niche:** all five cards were already Science, so nothing moved; the check now refuses it elsewhere. Six
   cards reworked (`docs/card-pricing.md`). `clear_energy` 2 was tried and dropped: it hurt both sides.
3. **Develop draws:** Protector 0% to 18-19%. Lowering the Protector's Expand gain to 1 credit took it to
   20-27% but cost 12-18 points at both sizes: dropped. A draw through `ai.growth` could reshuffle the
   tier-1 deck, and the reshuffle panicked on an undeclared RNG purpose; a real crash for a player too,
   present since r20 for the Builder. Fixed with a test (`develop_can_reshuffle_the_tier_1_deck`).
4. **Growth depth:** see below.

## Ruling 4: growth at full depth only at the top level

The planner follows its line outside the action phase, so a top-level score alone would have changed
nothing: it now re-roots its line at the top-level growth pick. On r21's rules, 16 seeds, c2 d2, one thread
(`scripts/bench/first_divergence.py`):

- **Engine:** 16 of 16 games identical. Top level is unchanged, as it should be.
- **Planner:** every game differs, and **not only at growth**: the first differing action is a threat payment
  in 12 games, a card play or choice in 3, a tier pick in 1. Rollouts now grow differently, so line values
  move and the planner's choices move with them. Wins 15 and 15, Develop's scoring unchanged at top level.
  "Play changes only where growth is chosen" holds for the engine, not for the planner.
- **Cost:** planner 25.6 to 22.4 ms a decision (-13%), 12.0 to 9.6 s a game; engine 2.9 to 2.6.

`per_decision.py`, r20 to r22, 16 seeds, c2 d2, one thread, under load: 28.5 to 35.2 ms a decision (×1.23),
decisions 455 to 503, options a decision 15.6 to 16.7. Other rules, so other games: ruling 4 saved 13%, and
r22's bigger hands (draws) and longer games cost more than that.

## Loops and caps

- **Develop's draw, more cards, more placement:** confirmed. One growth a round and step costs hold:
  tokens stay at 11-13 by r10.
- **Science niche, fewer losses, more insight, more Science:** confirmed. Effects fixed, floor 1, eight
  cards in 150; niche cards played 2.2 a game of 3.3-3.8 kept.
- **New:** the niche and c2. Presence lost fell by a third at c2, which is part of why c2 has no pressure.

## Known issues

- `niche_jobs` (`agents/policy.rs`) counts a card's printed shields, not what the seat's hexes can take, and
  jobs per card, so two clear cards can claim one marker. Agent-only; found in review with the gate
  running, left as played so the gate stays valid. Cheap fix, next agent change. (052 clamped the
  shields; two cards claiming one job is still open.)
- Scoring per-seat: the Protector's Develop at 18-19% is under the 20% floor by a hair.

## New-player summary of r22

Each round you pick one way to grow: **Expand** puts out 2 tokens, **Develop** puts out 1 and draws you a
card, **Consolidate** puts out none and gives you a pile of resources. You can also place a token as an
action for 1 energy more, but only once a round. Every token costs the energy printed on its space. A lost
token leaves a corruption marker: spend 1 energy to clear one in your reach and sway 1 toward Unity. Only
Science cards do better than that: they clear several markers, shield your tokens against the next hit,
and make a threat wait a round.

**Rules that resisted the summary:** the rich action still sits beside Place presence at a worse price and
isn't limited by "once a round", and the summary leaves it out. The shield still needs its two-part rule
(absorbs a destruction, but goes if the token is lost another way). The one-a-round limit itself reads well.

## The curve (r22, planner, 8 seeds a cell, win %)

| countries | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| 2 | 100 | 100 | 88 | 38 | 25 |
| 3 | 100 | 62 | 50 | 25 | 12 |

Against r20 (12 seeds): 100, 100, 83, 42, 25 and 83, 75, 50, 8, 17: the same curve within ±18 on 8 seeds.
The step from d2 to d3 is still the steep one (50 points at c2), and d3-d4 losses are mostly presence (18
of 25), so the difficulty's stage-III swap is what bites, not Discord. No cap losses in any cell.

`experiments/screen-046-curve.json`, `--quick --games 8`, seed 0.
