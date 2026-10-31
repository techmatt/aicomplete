# Growth as a choice, the marker worth clearing, Science on the board (041, r20)

Judged by the `planner`, d2, 32 seeds a cell (±8-12 points on a win rate), with 040's early-stopping
gate: `aic-sim experiment experiments/archive/gate-041.json`, then `python scripts/cards/screen.py --gate DIR`
and `--preset 041`. Full tables: `gate.md`, `gate-detail.md`. The gate ran 15 of 034's 27 agents for
time: the four new policies, all six ignore-tag checks and four exploits (no mono-tag, keep-all,
tier-1-only, no-presence, turtle, Unity farm, free cities, events-only, no-powers, keep-cheapest-2).

## Verdict against "done when"

| Asked | Result | |
|---|---|---|
| Ignore-Science loses clearly | **+3 ±9 at c2, -6 ±12 at c3: level** (r19: -16 / -9) | **not met** |
| Tokens left a seat 1-3 | 1.2 / 1.9 | met |
| Markers get removed in some games | 3.7 / 5.0 a game, in 57 of 64 games 2 or more | met |
| Failure mix: every loss occurs, none over 60% | 22 losses: Discord 13 (59%), presence 9 (41%), **cap 0** | **cap not met** |

## r20 at d2 (32 seeds, planner)

| | c2 | c3 |
|---|---|---|
| win % | 88 | 44 |
| win rounds | 7:2 8:10 9:11 10:5 | 7:2 8:7 9:4 10:1 |
| loss rounds | 7:1 8:1 9:2 | 7:9 8:8 9:1 |
| wins: purge / last Unity card | 1 / 27 | 0 / 14 |
| losses: Discord / presence / cap | 1 / 3 / 0 | 12 / 6 / 0 |
| tokens left a seat at the end | 1.2 | 1.9 |
| tokens taken a seat by r6 / r10 | 11.4 / 13.0 | 10.9 / 13.3 |
| growth: Expand / Develop / Consolidate % | 53 / 20 / 28 | 53 / 28 / 20 |
| markers placed / cleared a game | 7.5 / 3.7 | 10.9 / 5.0 |
| cleared from cities | 0.9 | 1.0 |
| threats delayed a game | 1.3 | 1.8 |
| rich actions a game | 0.4 | 0.8 |
| presence lost: wins / losses | 12.7 / 20.0 | 19.6 / 22.6 |

**Harm shares, presence lost in the base cells (17.5 a game):** Discord cards 53%, stage-III and
Border War twists 25%, corruption 11%, war 5%, tier-2 threats 5%, tier-1 threats 0%. No other source
is close to Discord, and tier-1 threats do nothing to presence: they are countered, and stage I is short.

**Unity-deck wins are 41 of 42:** allowed for now. The win rate fell at the first cut (44 / 6) and rose
past r19's 66 / 38 at c2 once steps were cheap enough; c2 at 88 has no pressure left to measure, so
most c2 gaps below are upper bounds. The curve below shows the same.

## The gate (gap = planner minus check, ± one standard error; base 88 / 44)

| check | c2 | c3 |
|---|---|---|
| **always-Expand** | +15 ±12 | +7 ±13 |
| **always-Consolidate** | **-3 ±8 ✗** | **-17 ±14 ✗** |
| **never-Remove** | +15 ±12 | +25 ±14 |
| **Science board rush** | +20 ±12 | +11 ±13 |
| Science rush | +20 ±12 | +14 ±13 |
| rich-spend | +33 ±18 | +42 ±16 |
| tier-2-rush | +7 ±9 | **-20 ±15 ✗** |
| no-counters | +75 ±17 | +50 ±18 |
| ignore-Science | **+3 ±9** | **-6 ±12 ✗** |
| ignore-Industry | -8 ±8 ✗ | +25 ±17 |
| ignore-Cyber | +3 ±9 | +11 ±13 |
| ignore-Power | +38 ±15 | +50 ±18 |
| ignore-Diplomacy | +50 ±21 | +50 ±18 |
| ignore-Security | +25 ±15 | +38 ±15 |

**The gate does not pass:** always-Consolidate, tier-2-rush at c3, ignore-Science and ignore-Industry at
c2. I did not tune around them. Structural causes, as far as I can read them:

- **Growth is not the only door to a token.** Always-Consolidate still takes 11.4 / 11.9 tokens by round
  10, because its 2-3 energy buys the standard action (`presence_energy` 1 + the step) and the 9-13
  cheap city cubes a game fill in the rest. Consolidate's bundle pays for the same tokens plus a
  resource surplus, so "presence or resources" is a real choice only for the turn, not for the game.
  Cheapest next test: `presence_energy` 2, or a limit of one standard placement a round.
- **Develop is dead for the Protector** (0.0 picks a game against the Builder's 3-6): 1 insight this
  round loses to Expand's second token and energy. A Builder draw is worth more than an insight.
- **Science.** Cheaper tier-2 board cards moved the Sweep from 5 plays to 21 and c3 from 38 to 50 (16
  seeds), yet ignoring the tag is still level. Two reasons. Agents draft tier 2 nearly every pick, so
  tier-1 Science (four of the six reworks) is seen only in the opening deal: 3-6 offers in 32 games
  against about 40 for each tier-2 card. And `never_tag` also bars the multi-tag cards, whose other
  tags have plenty of substitutes, so a seat loses no job: delay and shields replace counters and
  ambassadors the other tags already buy. `science_every` 1 did nothing (81 / 44 against 81 / 38, 16
  seeds). A tag becomes necessary when a job needs it; a `needs` Science on a strong card of another tag
  would force the result, at the cost of 034's "no tag is mandatory".
- **tier-2-rush at c3** is the same draft fact: the base nearly is the rush.

## The cap (ruling 1)

11 rounds through d2 by swapping a stage-I card for a stage-III; 10 from d3. Stage III hits 2 presence
and pulls 2 Sway back (the ruling's "press harder"). **No game ended at the cap in any of the gate's 628
games.** Harder late stages can't make cap losses on
their own: they end games earlier by presence or Discord, or they slow a win that then lands by 10. A
longer Unity deck (4, 3 in `experiments/archive/screen-041-tune2.json`; 4, 4 the same probe with
`alignment.unity_cards=4,4`, its spec not kept; 16 seeds) moved wins a round later and made no
cap loss either: at 4, 4 one win reached round 11. Stage III did move the mix: Discord's share of
presence lost 54% to 53%, twists 16% to 25%, and c3 losses from 8 / 2 to about 2:1. Cap losses need a
team that survives to 11 without winning, so what would make them is a win that gets harder late
without the survival getting harder, for example a tip's Sway rising in stage III.

## What the agents can and can't do here

- **Growth: yes, with a fix.** At 2 levels of choices Expand was scored as its first token and the
  Builder picked it 0.3-0.5 times a game; scored 4 levels deep it does (2.8-6.2 of 8-15 picks). Mixed within a seat at
  the Builder, nearly fixed at the Protector (Expand 6, Consolidate 2).
- **Remove: yes, for the wrong reason.** Agents clear for the Sway: 2.8-4 of 3.7-5 cleared markers are
  not on cities, and only 0.9-1.0 a game is on a city. The "2-4 when a city is near falling, fewer
  otherwise" shape is not what they show: they clear whenever a marker is in reach. Judge the rule by
  the never-Remove gap (+15 / +25), not by the count. At 1 credit back they cleared 0.1 a game.
- **Shield and delay: partly.** Both are worth nothing to a one-step score without `w.shield` and
  `w.threat_waits`, both hand-set. Rollouts see two rounds, so a delay's cost next round is seen and a
  shield's save usually is not. Judge them by the Science gap only with that caveat.

## The loops and their caps

- **Growth presence → uncovered income → more resources → more Expand.** Replaced in part: step costs
  hold (always-Expand stops at 11.7 by round 6, 12.8 by round 10) and one growth a round holds, but the
  standard action is a second door, above.
- **Marker reward → …** Confirmed, no farm: a marker comes only from a destroyed presence, and losing a
  token (its hex, its reach, the presence loss itself) costs far more than 1 Sway. The reward is 13-15%
  of a game's Unity Sway, though, so it is a real income from being hit, a small comeback.
- **Science board actions → fewer threats → more insight.** Confirmed: each effect is fixed, the floor
  stays at 1, and a delay stacks a threat onto next round rather than removing it.

## Tuning record (16 seeds, same seeds; c2 / c3)

| step | win % | tokens left | markers cleared | notes |
|---|---|---|---|---|
| first cut: steps 0, 0, 0, 1, 1, 2, 3, 4 | 44 / 6 | 2.2 / 3.6 | 4.8 / 5.3 | Builder Develop, Protector Expand, fixed |
| cheap steps | 81 / 25 | 1.2 / 1.7 | 3.8 / 4.8 | |
| first cut, 1 credit back, no Sway | 31 / 12 | 2.7 / 3.9 | 0.1 / 0.1 | agents stop clearing |
| middle steps, growth scored deep, eval terms | 88 / 38 | 1.5 / 1.5 | 3.8 / 5.9 | |
| plus review fixes, stage III 2 hits and a pull | 81 / 38 | 1.6 / 2.1 | 3.9 / 5.4 | **kept** |
| Unity deck 4, 3 | 81 / 31 | 1.3 / 1.6 | 3.9 / 6.6 | no cap loss |
| Unity deck 4, 4 | 94 / 44 | 1.4 / 1.5 | 3.9 / 6.8 | no cap loss |
| ignore-Science | 94 / 44 | 1.5 / 1.6 | 3.4 / 5.2 | |
| `science_every` 1 | 81 / 44 | 1.4 / 1.8 | 3.5 / 6.2 | |
| tier-2 Science at 4 and 5 credits, 1 insight | 88 / 50 | 1.3 / 1.8 | 3.8 / 5.4 | **kept** |

These seeds picked the kept rows; the 32-seed table above includes them, so it is not a fresh
confirmation of any one lever.

## New-player summary of r20

Each round you pick one way to grow: **Expand** puts out 2 tokens, **Develop** puts out 1 and gives you
a card or an insight, **Consolidate** puts out none and gives you a solid pile of resources. Every token
costs the energy printed on the space it comes from, cheap at first, up to 4 at the end; a few tier-2
cards say "free". You can also place a token as an action, for 1 energy more. A token lost leaves a
corruption marker: nothing can stand there, and it counts toward corrupting a city. Spend 1 energy to
clear a marker in your reach and sway 1 toward Unity. Science cards now reach the board: they clear
markers, shield your tokens and make a threat wait a round. The game lasts at most 11 rounds, 10 at the
hardest levels, and the last rounds are the worst: each player loses 2 tokens and Unity slips back.

**Rules that resisted the summary:** the shield, which is a hardening token that absorbs a destruction
but not a falling city, and goes with the token if it is lost another way; and the rich action, which
sits beside Place presence at a worse price and which the summary leaves out. A delayed threat
"waits", and a threat dealt after the counter phase waits too, by the same rule: that one reads well.

## The curve (r20, planner, 12 seeds a cell, win %, ±14; ruling 5 waits for this)

| countries | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| 2 | 100 | 100 | 83 | 42 | 25 |
| 3 | 83 | 75 | 50 | 8 | 17 |

Against r19: 100, 100, 67, 17, 17 and 100, 58, 42, 0, 0. d0-d1 no longer differ from each other at c2,
d2 is easier at both sizes, and d3-d4 lift off zero. The step from d2 to d3 is still the steep one, and
losses at d3-d4 are mostly presence (c2 13 of 16, c3 17 of 21), with stage III as hard as it is. c3 at d0
now loses, to Discord only. No cell had a cap loss, at 120 games: a d3-d4 cap of 10 ended none either.
`experiments/archive/screen-041-curve.json`, `--quick --games 12`, 7.5 min on 8 threads.
