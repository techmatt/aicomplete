# 263: balance pass 2, game length and the difficulty curve (r124)

Matt's rulings (locked): at higher difficulty the adversary must spread, reach over troops; games run 8-10 rounds;
bands are soft; the research rule stays at 5, 5, 5, 4, 4. Amendment 12068: new cities are the headline (retakes
apart); d3-d4 "most games" is at least 70% with a new city, d0 "rarely" at most 20% (candidates); a d0 cause on the
adversary's side takes adversary levers, no income dials; reach over the band. Amendment 12112 (Matt): D107's
printed form and the antimatter supply trim ride this item, shipped first. Message 12119: D107 reads "a spare host
slot". Bumped from the constant, 123 to 124 (main had 262's r123).

## What changed (r124)

- **Botnets don't spread at d0** (`adversary.botnet_spread_at` 1, a difficulty step): below it an event's or
  project's "Botnets spread" line does nothing (Botnet Swarm's too).
- **A city corrupts with 5 agents at d0** (`corrupt_at` 5, `corrupt_cut` 2, 1, 1, 0): 3 from d1 as before.
- **Antimatter trim:** Sanctuary and Open Archive give 2 antimatter once, as the capstone is uncovered (was 1 a
  round); Penning Trap Array and Antiproton Foundry gain none when built. Track labels print a one-time gain
  (`ai::once_gains`).
- **D107's printed form:** a city a player hosts or could host (one with a host slot to spare has a node on or next to
  it) can't be bought. The engine's check (the largest matching) is unchanged: no play change.
- Screened, not shipped: `city_cut_at` (from that level a city corrupts with one agent fewer again; 5, never).
- Stats: by-round `map.cities_corrupted` and `map.cities_retaken`; `scripts/cards/spread_read.py`.

## Why: what made d0 long

On 262's fresh read (r123), d0 ran 9.9-10.0 rounds and lost 10-16% of games to the deck. Those losses were botnets:
`alignment.purge_blocked_by_botnets` 3.2 a d0 game (0.15 at d3), 3.4 botnets on the board at the cap. Botnets spread
6.4 a game at every level and the players took down 2.5 at d0: with few corrupted cities at d0, the players purged
the cities and then waited on botnets the low-Unity purge still asked for. After the botnet cut, the d0 losses left
were games where the adversary took new cities late (1.7 a lost game against 0.27 a won one), which the d0 city number
answers. Neither touches the purge or Liberate.

## Give / get

- Penning Trap Array (10 cr, 1 ins): pay 26, get the Action 12 x 3.5 = 42 (1.62; was 60). Antiproton Foundry (12 cr,
  1 ins): pay 52, get 108 (2.08; was 126). Both still a visible gain.
- The capstone lump: a seat reaching Sanctuary or Open Archive streamed 3.2-3.3 antimatter (262's fresh read; 57% and
  56% of those seats reach it). Lump 2 (36 at the anchors) against the step's 5 energy (22.5), beside Sanctuary's
  shield and Open Archive's cube. A lump of 3 would trim almost nothing.

## Reads

Dev seeds 36800-36831 (192 games a cell, judge and greedy pooled), [screens.md](screens.md):

| Arm (d0) | win % | rounds | deck-empty | games with a new city |
|---|---|---|---|---|
| r123 | 87.0 | 10.13 | 13% | 39% |
| botnets don't spread (r123 base) | 95.3 | 8.89 | 5% | 22% |
| city 5 (r123 base) | 87.0 | 9.90 | 13% | 19% |
| both (r123 base) | 91.7 | 8.96 | 8% | 15% |
| r124 without city 5 (with the trim) | 89.1 | 9.11 | 11% | 25% |
| that, botnets spreading again | 88.5 | 10.08 | 11% | 35% |
| r124 (with the trim) | 93.2 | 8.80 | 7% | 10% |

The botnet lever was picked on the r123 base and re-read on the trim's base (amendment 12112): there it still cut a
round but not the deck-empty losses, which the city number then did. The antimatter trim on its own, at d0, -6.2
(95.3 to 89.1) on the dev seeds; d1-d4 within 1.6. `city_cut_at` 3 (d3-d4): d3 56.8 to 50.0, d4 26.0 to 15.6,
games with a new city 85% to 96% and 94% to 100%: reach the target didn't need, at 7-10 points.

**Fresh paired read** (`f263-read`, 49548-49595, three pairs, d0-d4, judge and greedy, 1,440 games a side; r123,
main d963c6d, natively on the same seeds), [read.md](read.md):

| Level | Judge r124 | r123 | gap | Greedy r124 | r123 | gap | Band (judge) |
|---|---|---|---|---|---|---|---|
| d0 | 95.1 | 88.2 | **+6.9 ± 3.1** | 92.4 | 91.0 | +1.4 ± 3.4 | ≥ 90 met |
| d1 | 95.1 | 95.1 | +0.0 ± 2.2 | 81.9 | 86.8 | -4.9 ± 3.6 | 80-95: 0.1 over, as r123 |
| d2 | 86.1 | 84.7 | +1.4 ± 2.6 | 84.0 | 88.2 | -4.2 ± 3.1 | 75-90 met |
| d3 | 63.2 | 60.4 | +2.8 ± 3.7 | 59.0 | 62.5 | -3.5 ± 3.6 | 55-70 met |
| d4 | 22.9 | 23.6 | -0.7 ± 2.3 | 28.5 | 27.8 | +0.7 ± 2.9 | - |

d1-d4 play changed only through the antimatter trim (the d0 levers are d0's alone); no gap there passes 1.4 SE.
Full oracle (`f263-oracle`, 49548-49559, d2-d3): 72/72, 10.8 and 9.2 rounds.

**Game length and spread** ([spread.md](spread.md), pre-r124 counters on every side so they compare; a floor on new
cities; by round: [spread-rounds.md](spread-rounds.md)), judge and greedy pooled:

| Level | rounds r121 / r123 / r124 | deck-empty r121 / r123 / r124 | new cities a game r121 / r123 / r124 | games with one r121 / r123 / r124 |
|---|---|---|---|---|
| d0 | 10.11 / 9.80 / **8.52** | 11% / 10% / **6%** | 0.55 / 0.41 / 0.11 | 36% / 30% / **9%** |
| d1 | 9.55 / 9.43 / 9.44 | 8% / 7% / 10% | 1.11 / 0.93 / 0.88 | 65% / 57% / 53% |
| d2 | 9.52 / 9.07 / 9.04 | 10% / 6% / 8% | 1.55 / 1.32 / 1.29 | 71% / 72% / 70% |
| d3 | 8.69 / 8.69 / 8.68 | 15% / 17% / 18% | 1.84 / 1.77 / 1.75 | 83% / 82% / **82%** |
| d4 | 7.72 / 7.69 / 7.79 | 6% / 7% / 9% | 2.62 / 2.53 / 2.41 | 92% / 91% / **89%** |

With r124's exact tally, games with a new city: 11 / 57 / 77 / 86 / 92% at d0-d4. By the end of round 4: 1 / 15 /
31 / 43 / 70%; by round 6: 3 / 24 / 50 / 66 / 89%. At d3-d4 the adversary takes a new city in most games before
the purge stops it (the purge wins in round 8.1 at d3); at d0, rarely and late (rounds 10-13).

**Economy (amendment 12112, fresh read):** antimatter spent 49% → 56% of 2.93 → 2.49 gained a seat-game (dev seeds,
the trim on and off alone: 50% → 59%). Buy energy 4.68 → 4.61 a game, hosted energy 15.75 → 15.22 (shorter
games); on the dev seeds the trim on and off, 4.37 / 4.42 and 16.2 / 16.3: unchanged.

**Speed** ([speed.md](speed.md), `per_decision.py`, 16 games, d2): the judge 1.9 → 2.0 ms a decision (1.05×),
greedy 2.1 → 2.2 (1.02×).

Kept: `out/keep/263/` (fresh read both sides, oracle, the dev screens on the r124 base). Seeds 49548-49595 taken.

## Candidates and open questions

- **Targets (candidates):** d0 at most 20% of games with a new city (11%), d3-d4 at least 70% (86%, 92%).
- **d4 runs 7.8 rounds**, under 8-10 on r121, r123 and r124 alike: 59% of its games end in a node loss (troops),
  round 7.6 when won. A d4 screen after the fresh read ([screen-d4.md](screen-d4.md), dev seeds, 192 games an arm):
  no stage swap at d4 7.70 → 7.91 rounds (25.5 → 27.6%); that and a city at 2 agents 7.59 (21.9%). Neither moves
  its length: the node clock (Muster, the research rule's Muster 3, troops) sets it. Not shipped; Matt's.
- **Reach against troops at d3-d4:** the curve's top already comes from both; `city_cut_at` would put more of it on
  reach at a cost of 7-10 points. Matt's to pick.

## For new players

At the easiest level (0) botnets never spread: the botnet each country starts with is all you face, and a city
needs 5 agents to fall (3 from level 1 up). Sanctuary and Open Archive now give 2 antimatter once, when you uncover
them, not 1 every round; Penning Trap Array and Antiproton Foundry make their antimatter only from their Action. You
can't buy energy from a city a player hosts or could host. No rule resisted a summary.
