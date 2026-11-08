# 147: the agents at r61, tempo, and the ladder re-read

Rules revision 61 throughout: agents only, no rules change (no `RULES_REVISION` bump). One measures-only tally was
added (`threats.countered.s<stage>.t<tier>` and `threats.uncountered.s<stage>.t<tier>`, so a read sees the counter
rate by stage). It moved the golden fingerprint (state JSON includes stats) but no game. Scripts:
[specs.py](specs.py) (screens and reads), [read147.py](read147.py) (win, nodes lost by cause, counter rate by tier and
`--stages`, `--vs` paired), [pairs147.py](pairs147.py) (each cell paired against one base cell),
[same147.py](same147.py) (do two cells play the same games), [worth147.py](worth147.py) (amendment 6307's node
and troop worth).

**The r61-equivalent spec** (`strategist:w.shelter_armor=0,w.corrupt_exposure=0,w.blight=0,w.tempo=0,t.node=4,
t.node_late=3,t.destroy_node=-4,t.each_destroy_node=-4,t.troops=-5`) on 147's binary plays exactly r61's agents'
games: 96 of 96 identical against the frozen r61 binary (37240-37247, d0-d3, three pairs). Every "r61" column below
is that spec, on the same seeds as the final agents.

## What changed in the agents

- **D67, corruption risk** (`w.corrupt_exposure` 0.25, `w.blight` 1; `agents/eval.rs`). A node on a site filling
  toward its Corrupt number counts as a share lost to its owner. The sites are a clean city, a station or factory not
  the adversary's, and open land with no building. The share is ((what the site would gather + 1) / its Corrupt
  number)², times its `node` and `shelter_node`; the seat's danger then reads its nodes less the exposed ones. Land
  with no Corrupt land line on the card ahead counts a quarter. A node armored by its owner's ambassador or a
  hardening token counts a quarter, and none counts under Sanctuary. Each land-corruption token, and
  open land filling toward one, costs its Discord drip over up to 3 rounds at its share of a Discord tip. This
  replaces 136's city-only `node_exposure`, which was off; the old name still sets the new weight.
- **The tempo trade** (`w.tempo` 0.5, `w.tempo_ref` 4; `lasting` in eval.rs). Each threat's harm at the value table
  splits into what it takes once (a payment, a discard, a building switched off) and what it leaves on the table
  (troops, created ambassadors, botnets, compute, sites and nodes taken, Sway). The unpaid weight is
  (once + lasting × r) / (1 + `tempo` × (r − 1)), with r the rounds left (the deck's clock cut by the projected win)
  over `tempo_ref`. At 4 rounds left this is 146's printed harm. Earlier, a one-off harm weighs less (paying slows
  the engine) and a lasting one more; late it is the reverse.
- **Amendment 6307, the node and troop rows** (`configs/agents/values.json`), measured on r61's agents at d2 on the
  dev seeds ([worth147.py](worth147.py), 192 games). A node is the revealed price: 7.8 credits paid a node placed on
  the node powers (Deploy, Accord, Buy In, Peer Review, track steps included). A troop is what a threat's troop
  strikes in play: 4.6 stations a game from 5.2 troops, 0.88 stations a troop at `build_station` 8, so 7.0. The
  changes: `rates.node` 4 → 8, `node_late` 3 → 6, `destroy_node` and `each_destroy_node` −4 → −8, `troops` −5 → −7.
  "A troop at 9" was `troops` 5 + `destroy_node` 4 on a landing that destroys; that is now 15, and a plain troop 7.
  Nothing was lowered. The measure was the default of question 1 (unanswered).
- **D71, shelter armor** (`w.shelter_armor` 1; `agents/guide.rs` `Armor`). The forecast reads armor as
  `board::destroy_piece` does: a hardening token, or the node owner's or site owner's ambassador. Each destroy spends
  one unit while any lasts, then takes a node. No more any player's ambassador, and no armored node that never falls.
- **Engine, same play:** `threats::seats_near` reads a threat's reach once for every seat, and
  `counter::standing` walks each seat's Auto Counters before asking who is near. Greedy played the same 16 games
  with all of 147's switches off; the speed effect was nil (1.01x, loaded box).

## On / off (the dev seeds, paired)

Base: everything on with `w.corrupt_exposure` 1 and the new rows (147's build before the last tweak and the review
fixes below, which F3 reads). Strategist,
64 seeds (36800-36863) × 3 pairs = 192 games a cell, the gap of each cell from the base:

| Cell | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| D67 off (`corrupt_exposure=0,blight=0`) | +3.1 ±2.5 | | −3.6 ±3.2 | −2.6 ±3.6 |
| `blight=0` alone | | | +1.0 ±1.0 | |
| `corrupt_exposure=0.25` (**kept**) | +1.6 ±2.3 | +4.2 ±2.5 | −1.0 ±3.0 | +0.5 ±2.8 |
| `corrupt_exposure=0.5` | | | +0.0 ±2.0 | |
| tempo off (`tempo=0`) | +2.6 ±2.3 | | | −7.3 ±3.3 |
| D67 and tempo off | | | −5.7 ±3.9 | |
| `tempo` 0.25 / 1, `tempo_ref` 6 | | | −1.6 / −1.6 / −0.5 | |
| D71 off (`shelter_armor=0`) | | | 0.0 (6 of 96 games differ, s1) | +0.5 ±0.5 |
| old node / troop rows | +2.6 ±2.6 | | −2.6 ±3.6 | −4.7 ±3.6 |
| r61's agents (the r61-equivalent spec) | +1.0 ±2.6 | +4.2 ±3.8 | +3.6 ±3.9 | −0.5 ±3.9 |

None of the switches is resolved from the noise at one level. Across levels tempo helps (d3 −7.3 off, 2.2 SE) and so
does D67 (d2, d3), while at d0 every part read slightly worse on. D67 does what it was asked to do: corruption node
losses fall from 3.9-4.1 a game to 2.6-2.9 at 1, and to 2.3-3.1 at 0.25. Greedy lines (96 games, d2, the dev seeds,
paired against r61's greedy): the final agents −4.2 ±5.9 (corruption losses 2.98 → 2.43), the `corrupt_exposure`
0.25 build before the review fixes −3.1 ±6.0, exposure 1 −12.5 ±6.0 with the new rows and −2.1 ±5.9 with the old,
tempo off −8.3, D67 off +1.0. Greedy
reads the rows' raise as worse, by 10 ±6, which is not resolved. The blight term alone moves little (+1.0 ±1.0
off), though land corruption is common: on F2, 0.7 / 1.1 / 1.3 / 1.9 land hexes a game at d0-d3 (r61's agents 0.8 /
1.2 / 1.5 / 2.1), dripping 0.5-1.8 Discord a game (r61's 0.5-2.0).

## The final read

Strategist, the three pairs pooled, d0-d3, 288 games a level, the final agents and the r61-equivalent spec on the
same seeds. F1 (37240-37335) ran the build with `corrupt_exposure` 1. The dev screens then took it to 0.25, which
spent F1, and F2 (37336-37431, the next free) read that build. A read-only review after F2 found three faults, so
**F3** (37432-37527) is the final read. The faults: D67 took the site owner's ambassador as armor against corruption
(the engine takes the site owner's cube first, so only the node owner's ambassador or a hardening token is armor, and
Sanctuary keeps every node); D67 charged a node's danger step twice (once in its term and again by cutting the count);
and the war forecast took the node owner's ambassador as a factory's armor. With D67's exposure off, blight now also
counts land filling toward corruption.

| Read | Agents | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|
| **F3** | **147, final** | **87.8 ±1.9** | **76.0 ±2.5** | **68.1 ±2.7** | **49.7 ±2.9** |
| F3 | r61's | 88.9 ±1.9 | 79.5 ±2.4 | 68.4 ±2.7 | 52.8 ±2.9 |
| F3 | paired gap | −1.0 ±2.3 | −3.5 ±2.7 | −0.3 ±3.1 | −3.1 ±3.1 |
| F3 | set a / set b (final) | 86.8 / 88.9 | 78.5 / 73.6 | 70.8 / 65.3 | 50.0 / 49.3 |
| | band | ≥ 90 | 70-85 | 55-65 | 40-50 |

**On F3, d0 is unmet** (87.8, 2.2 under, 1.2 SE) and **d2 is unmet** (68.1, 3.1 over, 1.1 SE). d1 and d3 are met.
What bound them is the seed set: r61's agents on the same games read 88.9 and 68.4, missing both too, and the same
build's levels on F2's seeds were 90.6 / 62.2 (F2 played the pre-fix agents, which are level with F3's against
r61's on both reads). No difficulty dial reaches d0 (the counter steps start at d1, the Militarist's compute at d3).
d2's dial is its counter step (`counter_more_*`). Retuning it is a rules change (r62) that needed a screen, a confirm
read on 37528+, the bump and the docs, more than the 80 minutes left; the screen of that step below is the next step,
for the planner. No dial was changed and the rules stay r61. **The d2 step screened** (final agents, dev seeds,
192 games, d2, paired against the r61 ladder at 62.0): d2's credits item +6 instead of +3
(`counter_more_credits=6`) −3.1 ±3.9; items +2 and credits +6 −14.1 ±3.9. The first would take F3's 68.1 to about
65, the band's edge; the second overshoots. Every level's rates: [final_read.txt](final_read.txt)
(F3); F2's in [final_read_f2.txt](final_read_f2.txt).

| Read | Agents | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|
| F1 | 147, exposure 1 | 89.9 ±1.8 | 71.9 ±2.6 | 63.5 ±2.8 | 41.0 ±2.9 |
| F1 | r61's | 93.8 ±1.4 | 76.0 ±2.5 | 62.5 ±2.9 | 46.9 ±2.9 |
| F1 | paired gap | −3.8 ±2.1 | −4.2 ±2.7 | +1.0 ±2.9 | −5.9 ±3.0 |
| **F2** | **147, final** | **90.6 ±1.7** | **74.7 ±2.6** | **62.2 ±2.9** | **48.3 ±2.9** |
| F2 | r61's | 91.3 ±1.7 | 75.0 ±2.6 | 64.2 ±2.8 | 46.9 ±2.9 |
| F2 | paired gap | −0.7 ±2.1 | −0.3 ±2.6 | −2.1 ±3.3 | +1.4 ±3.0 |
| F2 | set a / set b (final) | 88.9 / 92.4 | 72.2 / 77.1 | 55.6 / 68.8 | 44.4 / 52.1 |
| | band | ≥ 90 | 70-85 | 55-65 | 40-50 |

F2 (the pre-fix build) met every band, with d0 0.6 over its floor and its two sets 13 points apart at d2. F1 missed d0
by 0.1. Across the three reads (864 games a level each side) the final-or-near-final agents are level with r61's
agents: −3.8 to +1.4 a read, with no level past 2 SE on F2 or F3. The seed set moves a level as much as any change
here does.

**Nodes lost a game, r61's agents → the final agents (F3):** to corruption 2.56 → 1.98 (d0), 3.48 → 2.68 (d1),
3.71 → 2.93 (d2), 4.17 → 2.95 (d3); to threats 0.18 → 0.10, 0.45 → 0.37, 0.64 → 0.52, 0.80 → 0.77. Land corrupted
0.66 → 0.55, 1.10 → 0.97, 1.39 → 1.25, 2.28 → 1.89 hexes a game, and its Discord drip 0.57 → 0.59, 0.88 → 0.82,
1.17 → 0.93, 2.35 → 1.83.

## Greedy's speed (D58)

`per_decision.py`, c2 d2, 16 games from seed 1000, one thread, best of two, quiet box:

| Old → new | ms / decision | ratio | ms / game | decisions | options / decision |
|---|---|---|---|---|---|
| greedy r60 → r61 (6:00pm) | 2.7 → 8.1 | **3.04×** | 1147 → 4235 | 431 → 524 | 25.7 → 27.1 |
| greedy r61's → 147's agents | 7.8 → 10.1 | **1.30×** | 4075 → 5071 | 524 → 504 | 27.1 → 27.3 |
| strategist r61's → 147's | 1.6 → 1.4 | 0.89× | 922 → 840 | 574 → 590 | 18.1 → 18.0 |

`speed.py check` (147's binary against the r61 baseline): greedy 2.71×, engine 0.83×, planner 1.25× a game, with no
verdict (other games). `speed.py pair` r60 → r61 (message 6335): greedy c2 58.1 → 14.7 games a minute on one
thread (0.25×, so 3.95× a game), c3 0.15×; engine c2 0.62×, c3 0.43×. Every row is NOT LIKE FOR LIKE (other games),
consistent with per_decision's 3.04× a decision times 1.21× the decisions.

**Why r61 cost greedy 3×** (`bench/profile_win.py`, greedy, seed 1001, 3 games each binary). The search's prompt
settling (`search::settled`) went from 62% to 81% of samples. The engine run inside it went up most:
`threats::run` 3.8% → 14.3% and `Rules::enter` 4.8% → 14.3%, under it `counter::standing` / `near::is_near` /
`near::reach` (11-14%). At r61 an uncountered threat asks the hit player loss choices: D62's "pays N or discards M",
a nearest-seat tie, which node goes. A one-ply agent with `choices` 2 settles every action that closes the phase by
trying each answer. Options a decision rose only 1.05×, so the cost is in those settles, not wider menus. 146's
explanation holds. The strategist (one evaluation an option, no settle of the threat phase) got cheaper. 147's
same-play speedups (`seats_near` reading the reach once, `standing` asking for Auto Counters first, the eval's
open-threat check by `near_seats`) gave 1.01× and 0.99×: not where the time goes. A real cut means settling an
uncountered threat's loss prompts more cheaply, which moves greedy's play (D58, left open).

**Why 147's agents cost greedy 1.30× (message 6368): play, not code.** On identical games (147's binary with every
switch off, against the r61 binary, 16 games) greedy runs **0.92×** a decision, so 147's code costs nothing; the
same-play speedups net 8% there. On the same 16 seeds, turning one switch off moves greedy's ms a decision from
0.60× to 1.43× against the final agents: D67 off 1.43×, tempo off 0.60×, the old rows 0.66×, D71 off 1.05×.
Rounds and options barely move (1.01-1.11×). The cost follows how many threat-phase loss prompts a game's positions
make greedy settle, which each change moves by changing the games. No term's own arithmetic shows. The strategist's
cost fell (0.89×).

## Counter rate by tier and stage (d2)

Countered over countered plus uncountered; in brackets, threats a game at that stage and tier (F3). F2 (pre-fix
build, other seeds) beside it.

| Stage, tier | F3 r61's | F3 final | F2 r61's | F2 pre-fix |
|---|---|---|---|---|
| all, tier 1 | 68% | 67% | 68% | 67% |
| all, tier 2 | 74% | 75% | 72% | 71% |
| I, tier 1 (8.6) | 71% | 71% | 71% | 70% |
| I, tier 2 (0.3) | 60% | 62% | 52% | 45% |
| II, tier 1 (2.1) | 59% | 54% | 55% | 51% |
| II, tier 2 (10.4) | 75% | 76% | 74% | 76% |
| III, tier 1 (0.1) | 39% | 64% | 60% | 57% |
| III, tier 2 (5.4) | 73% | 74% | 70% | 64% |

The tempo shift is small and not stable across reads. On both reads the new agents let more stage-II tier-1 threats
through (−4 to −5 points) and pay stage-II tier 2 slightly more (+1 to +2). Stage I tier 1, a third of all threats,
is unchanged (70-71%). Stage III tier 2 read −6 on F2 and +1 on F3. The stage-III tier-1 and stage-I tier-2 rows
rest on under half a threat a game and swing either way. The tempo
term moves wins more than counter rates (d3 −7.3 off). Whether it moves which threats are paid was not read: the
counter rate per threat id needs a countered-by-id tally (`threats.uncountered.id.*` holds only the uncountered
side). Every level's rates: [final_read.txt](final_read.txt).
