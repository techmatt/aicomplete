# Calibrated at d2, pressure per country (053, r23)

Judged by the `planner`, d2. Fresh seeds from 5000 (never used to pick anything), 48 a cell, are the
verdict; tuning used seeds 1000-1023 and 2000-2031. Specs: `experiments/band.json`,
`experiments/gate.json` (gate-r23), `experiments/archive/gate-close-r23.json`. Full gate: `gate.md`.

## Verdict

| Asked | Result | |
|---|---|---|
| Planner at c2 and c3 d2 inside 60-75% | **c2 71 ±7, c3 62 ±7**, 48 fresh seeds | **met** |
| No dominance check beats the planner by more than the noise | at 48 seeds the closest is tier-2-rush, -10 ±9 at c2 and -6 ±9 at c3 | **met** (level, not behind) |
| Discord under 80% of losses, presence and cap live | 32 planner losses: Discord 12 (38%), presence 19 (59%), cap 1 (3%) | **met** |
| Same rules, tuning seeds (not the verdict) | c2 67, 78; c3 67, 69 | agrees |

The strategy-health tool reads its 8-seed gate as "not passed" because any check level or ahead on the
sign fails it: 9 of 64 cells at ±22-25. Those 9 were rerun at 48 seeds, paired on the band's seeds.

## Where c2's slack came from

Same seeds, 24 games, planner, r22 rules with D20 in. A number is per country and round.

| | c2 | c3 | c3 / c2 |
|---|---|---|---|
| Discord Sway, all | 1.46 | 1.92 | 1.31 |
| from bases | 0.23 | 0.69 | **3.00** |
| from the rest | 1.23 | 1.23 | **1.00** |
| Unity Sway | 1.57 | 1.41 | 0.90 |
| arrivals | 2.04 | 2.41 | 1.18 |
| standard actions | 2.56 | 2.24 | 0.88 |
| tokens placed | 1.32 | 1.32 | 1.00 |
| threats dealt / executed | 1.60 / 0.08 | 1.43 / 0.02 | 0.89 / 0.30 |
| presence lost | 0.66 | 0.92 | 1.39 |
| corruptions | 0.015 | 0.054 | 3.66 |
| nodes made | 0.15 | 0.36 | 2.36 |
| cities freed | 0.066 | 0.035 | 0.53 |

So the slack is **harm per country at one source, not tokens or actions**: tokens match exactly, actions
are a little better at c2, threats are no harder at c3 (d1's one extra threat is shared, so c2 sees more
a country), and every Discord source but bases matches to the second decimal. A country has one front
at c2 and two at c3, so a printed "each military base" is 2 bases at c2 and 6 at c3. The gap was never
that c2 had too little pressure; c3 had double on one line.

**A second cause, an agent's:** D20 alone took the planner from 96 / 54 to 67 / 38 on these seeds. The
guess alone (`redeal_alignment` in `Greedy`: each decision scored on a redealt Alignment deal, the lines
still on the true one) gave 75 / 42, so hiding the deal from the planner's lines (`hide_alignment_deal`)
cost the rest, inside the noise (±10). So c2's 93-100
at r22 was mostly agents knowing which Unity cards were dealt, and the "structural" c2 of 046 was not
the game's. (052's `niche_jobs` clamp went in before this baseline, and its share isn't split out.)

## Calibration on paper, then measured

Bites: base arrivals on War Drums (stage I), Border War (II), Blitzkrieg (III); Escalation on all five
stage-II cards, rounds 4-8 at d2. Each sways on the next production step: rounds 1, 3, 5, then every round from 6 (`board.clock`).

| Lever | When it bites | Equal on paper | Measured, seeds 1000-1023 unless marked; base 67 / 38 |
|---|---|---|---|
| **First base per country** | rounds 4-8 every round, 2-3 other cards | 1 a country: c3 bases 0.5 tip a step, as c2, not 1 | **c2 unchanged by construction; c3 67, 69 on 2000-2031 (base 31)** |
| A country sways once for its bases | production steps | same | c3 42: spread still floods bases |
| Discord bucket at c3 | every tip | 6 × 1.31 ≈ 8 | 7: 54; 8: 75; treats the symptom |
| Bucket 3 and 5 | every tip | about a third more Discord | with softened steps: 58 / 46 |
| `extra_threats` 2 | every round from 1 | adds 1 a country at c2, 0.67 at c3 | with softened steps: 50 / 46 |
| `escalation_ambassadors` 2 | rounds 4-8 | one rate per country once bases count | with softened steps: 83 / 54: c3 feels ambassadors more |
| `project_discord_per` 1 | once a megaproject completes, about round 5 | even per country (0.97) | 70 / 62 on 56 seeds: in band |
| Discord steps 1 presence, not 2 | tips 5-7 | moves presence losses to the deck | 82 / 68 on 56 seeds: c2 out of band |

Chosen: first base per country, the only lever that moves c3 without touching c2, and the one that
removes the cause. **Ruling 2's general push added no lever:** D20 was that push, 29 and 16 points; on
top of it every push tried either left the band or moved nothing beyond the noise. The cleanest extra
push, if wanted, is `project_discord_per` 1.

## r23 at d2 (fresh seeds from 5000, 48 a cell, ± one SE)

| | c2 planner | c3 planner | c2 engine | c3 engine |
|---|---|---|---|---|
| win % | **71 ±7** | **62 ±7** | 77 ±6 | 71 ±7 |
| win rounds | 7:3 8:7 9:13 10:8 11:3 | 8:2 9:18 10:9 11:1 | 7:5 8:10 9:11 10:9 11:2 | 7:1 8:8 9:15 10:8 11:2 |
| loss rounds | 7:2 8:3 9:5 10:1 11:2 12:1 | 7:1 8:5 9:8 10:3 11:1 | 7:2 8:3 9:2 10:2 12:2 | 7:1 8:2 9:7 10:2 11:2 |
| wins: purge / last Unity card | 5 / 29 | 0 / 30 | 10 / 27 | 3 / 31 |
| losses: Discord / presence / cap | 5 / 8 / 1 | 7 / 11 / 0 | 1 / 8 / 2 | 7 / 7 / 0 |
| Unity / Discord tips | 5.2 / 5.2 | 5.3 / 5.7 | 5.1 / 4.9 | 5.1 / 5.4 |
| presence lost a game | 14.9 | 23.2 | 12.2 | 18.8 |

Presence lost by source, all four cells: Discord cards 43%, stage-III and Border War twists 38%, corruption
10%, tier-2 threats 5%, war 5%. Presence losses are mostly Discord's doing, through its cards: a tip's
presence step ends the game before the deck does.

**The cap** caused 1 of 32 planner losses here, 0 of the gate planner row's 5, and 7 of the gate's 330
losses in all (2%). It stays near 0 because wins come at rounds 8-10 and the other two clocks take most
losses by round 9-10, so few games reach round 12. No lever was added for it: it is a quiet backstop
(046, locked), and it did end games.

**The engine is level with or ahead of the planner** (+6 and +9, ±9-10): D20 cost the planner more than
the one-step engine. "A stronger team always does better" doesn't hold between these two at d2 now; this
is a finding about the planner, whose rollouts are a judge of the rules, and the band is set on it as
asked.

## The gate

gate-r23, fresh seeds from 5000, 8 seeds a check, 444 of 512 games, 18 min on 8 below-normal threads, 0
panics: `gate.md`. Every check not listed below lost ground at 8 seeds. The 9 cells level or ahead, plus
tier-2-rush and the rest of their checks at both sizes, at 48 seeds (`gate-close.md`; gap = planner minus
check, paired, ± one SE; c2 71, c3 62):

| check | c2 | c3 |
|---|---|---|
| tier-2-rush | -10 ±9 | -6 ±9 |
| no-powers | -2 ±8 | +0 ±9 |
| always-Consolidate | +0 ±8 | +15 ±10 |
| ignore-Industry | +4 ±9 | +8 ±10 |
| ignore-Cyber | +8 ±9 | +23 ±7 |
| never-Remove | +10 ±7 | +17 ±7 |
| abandon (new) | +50 ±25 at 6 seeds | +25 ±21 at 8 seeds |

**Tier-2-rush and no-powers level at both sizes are the standing structural findings:** tier 2 is simply
better (046's reading, unchanged), and AI-board powers add nothing to play as priced. Neither is a beat
beyond the noise. Always-Consolidate level at c2 is the same line: at c2 it does as well as growing.

## Rules with new-player summaries

**Bases, for new players.** An adversary card may name military bases, and stage II cards send an extra
ambassador to them. Either way it lands on one base in each country, the lowest-numbered. A base with an
ambassador on it swings the world toward Discord when it next produces, so if a country has two bases,
guard the first.

**Rule that resisted it:** "first base" is a new term for a printed number, and it only means anything at
3 and 4 countries, where a country has a second base. An ambassador can still spread onto that second
base, so "guard the first" is a good habit, not a guarantee.
