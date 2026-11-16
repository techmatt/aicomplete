# 249: the real oracle, a better judge, design checks for standard-action prices

Base: main 5ab1f91, r112. **No rules change** (no `RULES_REVISION` bump). Item A of A → B → C (Matt, 2026-11-16):
design against a **real oracle**, very strong play that can't see the future; the judge (the strategist) must value
cards before Item B moves the game onto them.

Tables: [oracles.md](oracles.md) (the three agents d0-d4), [screens.md](screens.md) (the judge's screens and its
fresh read), [agree.txt](agree.txt) (agreement with the real oracle; raw `agree_real*.jsonl`), [arms.md](arms.md)
(the rank check), [design-check.txt](design-check.txt) (r112's flags). Scripts: `mkspec.py` (every spec),
`oracles.py`, `pooled.py` (the ship rule's read), `arms.py`, `port_jobs.py` (job parity's data).

## 1. The real oracle (`oracle:worlds=K`, `agents/oracle.rs` `choose_real`)

The oracle's search, run on worlds sampled from what players can see (`determinize`: deck orders, the Alignment deal,
the adversary deck and projects, the dice), never the true table. At a decision of the action phase, while the round's
`budget` lasts (counted in lines, a world each), it samples `worlds` worlds, plays each candidate to the game's end on
every world (the followed lines' move first, then the shortlist by the heuristic, `alts` more), and takes the one
that wins on most worlds, then the best mean heuristic `horizon` rounds on; a candidate winning on every world ends
the search. Between searches it plays the move most followed lines play next; a line is dropped once the table shows
something its world guessed otherwise (`determinize::agrees`: the two states equal but for the hidden piles, dice and
a pending snapshot). No restarts (`tries`): a restart reads the true game's loss. A fast-lane test checks that hidden
information alone moves no decision; another that it replays and repeats.

**Spec (skilled_rows.py `real`):** `oracle:strategy=0,worlds=4,alts=3,budget=32,answers=1,truth=1` + the judge's
weights (before this item's `card_action`). Its lines score their own world's table (`truth=1`: the world is the
guess). **First version** (the engine's weights, `strategy=1`, worlds 3, alts 2, budget 12) read **55% d0-d4 against
the judge's 70%** on 48568-48571 (spent): its engine-policy lines play slow and lose on an empty deck at d1-d2. On the
judge's weights (the search on top of the judge) it read 21/24 against the judge's 19/24 at d3 on the dev seeds.

**Fresh read (48572-48575, 12 games a level and agent, three pairs; [oracles.md](oracles.md)):**

| | d0 | d1 | d2 | d3 | d4 | all | rounds (all / won) | s a game |
|---|---|---|---|---|---|---|---|---|
| judge | 12/12 | 12/12 | 10/12 | 8/12 | 5/12 | 78.3 ± 5.3 | 8.70 / 8.62 | 1.6 |
| real oracle | 11/12 | 12/12 | 10/12 | 9/12 | 6/12 | 80.0 ± 5.2 | 9.05 / 8.83 | 217 |
| full oracle | 12/12 | 12/12 | 12/12 | 12/12 | 12/12 | 100 | 10.18 / 10.18 | 26 |

**It lands at or below the full oracle, but not clearly above the judge** (+1 game in 60; +1 at d3 and d4, -1 at d0).
Why: (1) with 4 worlds a candidate's win share moves in quarters, and most decisions see every candidate win on all or
none, so the choice falls to a 4-world mean heuristic two rounds on: about as noisy as the judge's own one-step score;
(2) 32 lines a round buy one or two full searches, so most moves are the followed lines' vote, the judge's own policy
on a guessed world; (3) the full oracle's edge is information, not search: it finds a winning line on the true deck
and follows it (60/60 at 26 s a game), which no sample count recovers. **Twice the worlds and lines** (8 worlds,
64 a round, ~420 s a game) on fresh 48768-48779 (one pair, farmed; [real2.md](real2.md)): d3 10/12, d4 6/12 against
the old judge's 10/12, 5/12 and the shipped judge's 11/12, 6/12. More lines don't make it clearly better at this
scale; a clearly stronger fair player needs a better line policy than the judge, or far more lines (cost is linear,
already 135-270× the judge). Seconds a game are agent time on one thread, on a loaded box (ratios only).

## 2. The judge

**Agreement is not an honest guide for this real oracle.** Built on the judge, it agrees with the judge on 82% of
plays; every change to the judge lowers raw agreement (`agree.txt`). `agree` now tallies a `miss` group, the
decisions where the first agent (the base judge) picked otherwise: there `w.card_action` 2-4 recovered +7 to +10
points of plays and greedy +25 (the judge 0 by construction). The hand terms toward greedy's (`w.hand` 3 /
`w.hand_cost` 0.4, or halfway) recovered +12 overall there, but **lost wins** (screen 3: -5.0 ± 2.0 and -1.6 ± 1.9;
beside `card_action` 3, -0.2 against `card_action` alone +3.6): the real oracle defers plays the judge makes, and
copying that without its search costs games. Win rate decided.

**New terms (flat mode, sentence-sized, 0 = off):** `w.card_income`, a card's income counted as the AI track's;
`w.card_action`, a building counts what it earns a round beyond income (its standing rules and its Action's share, at
the value table's rates) for that many rounds, at the credit weight.

**Screens** (d2-d3 on 248's spent seeds 48376-48471, 576 games a cell, paired): `card_action` 2 / 3 / 4 / 6:
+3.3 / +3.6 / +3.3 / +0.7 (± 1.8-1.9); `card_income` 0.5 / 1: +1.2 / +0.2; both (0.5 + 4): +4.2 ± 1.8; card_action 2 +
income 1: +2.4. `card_income` read no gain (as 246's `prod_income`): left off.

**Fresh read, the ship rule** (48576-48767, farmed, 2,304 games an agent; `pooled.py`): `card_action` 3 against the
judge d0 -1.0 ± 1.1, d1 +1.0 ± 1.3, d2 +0.7 ± 1.5, d3 +5.0 ± 2.1; **pooled d2-d3 +2.86 ± 1.28 (2.2 SE), no level
down by 2 SE: shipped** (`STRATEGIST` gains `w.card_action=3`). Greedy is unchanged. Before the ship the judge's
agreement with the real oracle was 82.4% of plays (140.2 score), after 66% (130.3) by construction; on its overrides
0% → 8.4% of plays.

**Bands on the shipped judge** (the fresh read): d0 95.3 ± 0.9 (90+ met), d1 95.0 ± 0.9 (80-95 met, at the top),
d2 91.5 ± 1.2 (75-90 unmet, 1.5 over), d3 73.3 ± 1.8 (55-70 unmet, 3.3 over). Rounds 8.79 / 8.49 / 8.38 / 8.28.
The old judge on the same seeds: 96.4 / 93.9 / 90.8 / 68.2.

**Cost:** the term reads the value table per building: per decision 1.31× the old judge (one thread, loaded box,
`per_decision.py`, same seeds), then 0.92× once each building's earnings are stored per decision
(`built_round_cached`; the same games, checked on 24 d3 seeds). The judge stays the fast screen.

## 3. Rank check: 248's arms on the shipped judge ([arms.md](arms.md))

Arms (248): a the baseline; b every standard action off but Liberate and Dispatch; c Liberate off too. The judges on
248's probe seeds 48376-48471 (1,152 games an arm; the old judge is 248's kept read, which the reader reproduces to
the game); the real oracle on 248's oracle spot-check seeds 48376-48379 (48 an arm), beside 248's full oracle there.

| arm (gap to a) | old judge | shipped judge | real oracle (48) | full oracle (48, 248) |
|---|---|---|---|---|
| a | 87.9 | 89.3 | 95.8 | 100 |
| b | 60.6 (-27.3 ± 1.5) | 60.9 (-28.4 ± 1.6) | 77.1 (-18.8 ± 7.1) | 97.9 (-2.1 ± 2.1) |
| c | 20.6 (-67.4 ± 1.4) | 24.1 (-65.2 ± 1.5) | 33.3 (-62.5 ± 7.1) | 85.4 (-14.6 ± 5.1) |

**The gaps barely move on the shipped judge** (b -1.1, c +2.2), **but the real oracle reads nearly the same gaps**:
c's -62.5 ± 7.1 is the judge's -65.2 within noise, b's -18.8 ± 7.1 sits 9.6 points (1.3 SE) inside the judge's. 248's
reading that the game is winnable without standard actions and "the judge is the bottleneck" was the full oracle's
sight of the deck: a player who can't see the future loses c about as often as the judge does. On the same 48 seeds
the shipped judge reads b -27.1 ± 6.5, c -72.9 ± 6.5. Item B's cards, not the judge, have to close c.

## 4. Design checks (`scripts/design_check.py`, `design_check.json`, `design_check_jobs.json`)

- **Job parity** (`check_jobs`): each standard row's price per unit of its job at the anchors must be at least 1.25 ×
  the median card's for that job (its rate at most 0.8 of theirs); its outcome no stronger (`outcome` owned against
  the job's `card_outcome` neutral); a job no card does is flagged. The cards are 248's hand-read table, ported with
  their other effects as expressions over the anchors (`port_jobs.py`); costs read from content.
- **Circular anchors:** an extra anchor whose `anchor_sources.set_from` names the action it measures, or one equal
  to the action's give with no `independent` derivation.
- **Per-case exemptions:** `exempt` is `{case, why}` on a row with that `case`; a bare or whole-action one is
  flagged and exempts nothing. Liberate is three rows (city, station, factory); only the city is exempt.
- **Claim rows:** city (the Liberate price, 5 ambassadors, for +1 energy income, 14), station (3, owned station 20),
  factory (2, factory 19).

**r112's flags (12; Item B fixes the rules):** circular anchor: station (Build a research station), botnet (Take down
a botnet); over the anchors: Liberate a station 1.17, Liberate a factory 1.69, Claim a factory 1.19; job parity: Take
down a botnet 1.07 × the median card (16 against 15), Claim a station 1.23 × (24 against 19.5); stronger outcome:
Liberate a station, Liberate a factory (yours, where Reclamation Crew and Back-Channel Talks only free the site); no
card for the job: Build a research station, Claim a city, Sell a card. Information only: Liberate a city 1.12 at the
anchors and 1.04 × the median liberate card (exempt, the city's case).

## Seeds and outputs

Fresh, taken: 48568-48571 (the first real-oracle read; spent by Matt's message 11293 once the real oracle changed
because of it), 48572-48575 (the real-oracle read), 48576-48767 (the judge's fresh read), 48768-48779 (the 2× real
oracle); ledger next free 48780. Spent seeds re-used: 248's 48376-48471 (screens, rank check), 48376-48379 (the real
oracle on the arms, as 248's oracle spot-check), dev 36800-36823 (calibration). Kept (`out/keep/`): `249-oracles`
(first version), `249-oracles-final`, `249-fresh`, `249-arms-real`, `249-armsjudge`, `249-real2`.

**Diff review** (a reviewer on the whole diff, after the fresh read): no hidden-information leak, no thread-count or
hash-order dependence, the store's key complete. Fixed: design_check's job check re-derived the exemption with looser
rules (now `exemption()`), and could raise on an empty job or an unknown outcome. Noted, not changed: on a pending
choice the real oracle samples worlds by `determinize`'s pending rule, which keeps the true streams after a step that
drew a pile out and reshuffled (rare); the agents' standing exception (151) scores the true table there anyway.
