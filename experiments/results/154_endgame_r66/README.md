# 154: round-cap losses, classified, and the late liberation (r66)

Matt's ruling (2026-11-08): the round cap stays a plain loss; each round-cap loss's end state is examined for an agent
mistake (the players could have finished) or a design mistake (no tools to finish). `RULES_REVISION` 65 → 66, bumped
from the constant (the prompt also said r66).

Scripts here: [specs154.py](specs154.py) (screens and reads), [ends154.py](ends154.py) (end reasons, the cap's share
of losses), [endgame154.py](endgame154.py) (the classification), [screen154.py](screen154.py) (a screen or read by
lever, with Liberate uses). The probe is `crates/aic-sim/src/endgame.rs`, opt in with `"endgame"` in a spec
(docs/experiments.md). Tables: [classify.md](classify.md) (every game's verdict), [screens.md](screens.md),
[final.md](final.md).

## 1. The classification (r65, seeds 38872-38967, strategist, the three pairs)

**The probe.** For each game lost to the round cap: the end state (each kind of piece the purge still counts at the
game's own Unity level), the team's holdings at the start of the third-last round (the snapshot: ambassadors held
and on the board, credits, liberate / botnet / troop / ambassador-removal cards in hand and in play, Unity removals
still in the deck, Unity on the track), and a replay from the snapshot by a stronger player: the real game continued
(same state, same adversary deck) by `planner:strategy=1,w.blocker=8` (the ceiling tier, valuing the purge four
times as much). Cross-check on set a: `planner:strategy=1` at its own weights.

**The bound** (generous to the players, no search; `endgame154.py`'s docstring): the purge route's ambassadors
(each corrupted city at its stage's price after Remove clears its adversary ambassadors at 3 a use, liberate cards
covering cities outright) against every ambassador held, on the board, bought with every credit held and earned
(20 each) and gained at the game's own rate; botnets and troops (where they count) against the cards that remove
them, used every round; and the Unity-deck route's Sway against 5 a liberation, 2 a corruption marker in reach,
Volunteer Corps and twice the game's own Unity rate. Can't win: both routes fail.

| Level | r65 losses | Round cap (share) | Can't win (bound) | Replay won: agent mistake | Replay lost: tight (design) | Unity drawn at the end | Corrupted cities left | Near misses |
|---|---|---|---|---|---|---|---|---|
| d0 | 12 | 8 (67%) | 0 | 3 | 5 | 7.1 | 3.6 | 6 of 8 |
| d1 | 61 | 10 (16%) | 0 | 3 | 7 | 6.2 | 4.5 | 3 of 10 |
| d2 | 111 | 9 (8%) | 1 | 2 | 6 | 6.1 | 4.9 | 1 of 9 |
| d3 | 152 | 74 (49%) | 21 | 3 | 50 | 5.4 | 5.1 | 13 of 74 |

**90 design mistakes against 11 agent mistakes.** The cross-check (default planner, set a): 48 against 7. Every
replay win came through the Unity deck, none by the purge. A near miss is 8 Unity cards drawn or at most 2 purge
pieces left: d0's cap losses are near misses, d3's are far off.

**The shortfall is ambassadors.** At d3's snapshot (start of round 8 of 9) the team held 1.9 ambassadors against a
liberation bill of 37 (5.5 corrupted cities, about 4 adversary ambassadors stacked on each by the city rallies);
the bound finds ambassadors short in 46 of 74. **Then the Unity steps:** 36 of d3's 74 never drew a sixth Unity
card, and below it the purge still counts troops, 43 a game on average (troops with nothing to strike pile up; only
four cards remove them, 1-4 each), so no team purges there. Not moves (Liberate and Remove take from the supply),
not botnets (about 1 a game), and cities don't come back (the free_leaves ambassador doesn't count from step 3).

## 2. The fix: the late liberation (design mistakes dominate)

**The rule:** from stage III (`liberate_late_stage` 3: the last three rounds at every level, rounds 9-11 at d0-d2
and 7-9 at d3) the Liberate action frees a city for `liberate_late_at` 3 ambassadors instead of 5, still plus one an
adversary ambassador on it. Stations and factories keep 3. Printable: the stage is on the face-up card. Built from
the existing Liberate action; capped by the supply, reach, the corrupted cities standing and the stage.

**Give / get at `values.json`:** 3 ambassadors (4 each: 12) for a liberation (45: 5 Unity, the city, its diversion
stopped), +33 to the seat that frees it, against +25 at 5 (20 for 45). Its place: the last three rounds of an 8-10
round game, when a seat saving toward 5 can free a city a round sooner, and each liberation's 5 Unity is most of a
tip at two players (6). The agents use it with no new code: they price the action by `liberate_cost`, which reads
the stage (the eval's saving-up term, `eval.rs`, reads the same cost).

**On / off** (screen 1, dev seeds, r65 dials, paired on the same seeds; [screens.md](screens.md)):

| Late price | d0 | d1 | d2 | d3 | Cities liberated a game (d0-d3) |
|---|---|---|---|---|---|
| 5 (off, r65) | 94.8 | 79.2 | 61.5 | 36.5 | 1.12 / 1.32 / 1.14 / 0.89 |
| 4 | +0.0 | +1.0 ±1.0 | +4.2 ±2.9 | +6.2 ±3.6 | 1.21 / 1.50 / 1.41 / 1.39 |
| **3 (taken)** | +3.1 ±1.8 | +2.1 ±2.6 | +11.5 ±3.3 | +13.5 ±4.1 | 1.44 / 1.77 / 1.79 / 2.21 |
| 2 | +2.1 ±1.5 | +9.4 ±3.0 | +12.5 ±3.4 | +26.0 ±5.0 | 1.65 / 2.11 / 2.21 / 3.38 |

**Use per game** (screen 2's base, the late tally): 0.93 / 1.05 / 1.81 cities freed at the late price at d1-d3; on
the r66 same-seed read 0.50 / 0.89 / 0.85 / 1.77 (d0-d3). No agent heuristic was added: agent mistakes were 11 of
101.

**The dials** (screen 2): `counter_bonus_credits` 6 → 8 (d1 −7.3, d2 −14.6, d3 −1.0 on the dev seeds) and
`compute_income` at d3-d4 10 → 16 (d3 about −15). The same-seed check (38872-38967, before the last step) read d2
67.4, 2.4 over its band (SE 2.8): d2's own compute entry went 8 → 10 (151 measured −6.8 for it). Final
`compute_income` 5, 5, 10, 16, 16; `ambassador_income` and `counter_more_credits` unchanged.

## 3. The final read and the cap's share

Fresh seeds 38968-39111, three 48-seed sets (d1 sits within 2 points of an edge, so the three are pooled), r65 on
its own binary and configs; 432 games a level a side ([final.md](final.md)).

| Level | r66 win % (SE) | r65 | Paired gap (SE) | r66 sets | Band | Verdict | Cap share of losses r65 → r66 (games) |
|---|---|---|---|---|---|---|---|
| d0 | 95.8 (1.0) | 95.4 | +0.5 (0.5) | 95.8 / 94.4 / 97.2 | ≥ 90 | met | 50% → 44% (10 → 8) |
| d1 | 79.9 (1.9) | 77.8 | +2.1 (2.1) | 82.6 / 77.8 / 79.2 | 70-80 | met, 0.1 inside | 18% → 8% (17 → 7) |
| d2 | 62.5 (2.3) | 59.3 | +3.2 (2.6) | 70.1 / 59.0 / 58.3 | 55-65 | met | 12% → 6% (22 → 9) |
| d3 | 44.9 (2.4) | 45.1 | −0.2 (2.9) | 48.6 / 41.0 / 45.1 | 40-50 | met | 51% → 28% (121 → 67) |

The r65 → r66 gap changes the rule and three dials at once: the rule's own effect is screen 1's on / off. The dials
turned round-cap losses into Discord-deck losses (d3: 116 → 170): the late liberation finishes games, the dearer
counters and compute press harder on the way. On the classification seeds (38872-38967) r66 before the d2 step read
d0 96.9 / d1 78.8 / d2 67.4 / d3 45.8, the cap's share 67 → 78% / 16 → 13% / 8 → 4% / 49 → 39%, and its 80 cap losses
classified as 60 design and 20 agent mistakes ([classify.md](classify.md)): design mistakes still lead, the agents'
share rose (more games now end close enough for a better player to finish).

Standing measures (amendment 6618), r65 → r66: nodes lost to corruption a game 1.69 → 1.63 (d0) .. 2.91 → 2.92 (d3);
adversary ambassadors placed 66.6 → 65.7 .. 96.6 → 99.6; projects revealed 30% → 30% (d0) .. 59% → 76% (d3) of the
deck, resolved 16% → 16% .. 44% → 60%; reshuffles 0 → 0.10 a game at d3 (compute 16 turns the project deck over).

## Speed and checks

- **Speed** (`per_decision.py` r65 against r66, c2 d2, seed 1000, 16 games, one thread, quiet box): the strategist
  2.2 → 1.7 ms a decision (0.79×), greedy 17.1 → 17.0 (0.99×). `speed.py check` had no verdict across the revision;
  the baseline is updated to r66.
- `loop_smoke.py --run`: no timeouts, at most 3 plays of one card a game. `play_read.py` on the final read (1728
  strategist games): nothing kept over 1% and never played. No card changed: `npm run cards:fit` not needed.
- The probe never touches the game it measures: the probe reads (`out/p/probe-*`) and r65's own binary gave the
  same win and loss counts on the same seeds.
- No `bench/same_results.py` replay: nothing in the rules changed after the final read's build (later edits are
  tests, docs and the golden constants).
- **A test guard fixed:** `pending_choices_replay_on_the_guess_three_countries` (slow lane) compared hidden decks by
  length; at d4's compute 16 a project deck and a threat deck reshuffled mid-deal and kept their length while dealing
  a different card. The guard now also skips when a threat or project comes out that wasn't out before.
- The lab config (core tests) prices the late liberation at 1 like `liberate_at`, so no lab test sees a stage step.

## For a new player: finishing a game

You win the moment the ninth Unity card is drawn, or when the board is purged. Each Unity card you draw makes the
purge easier: with 3 drawn you no longer need to clear adversary ambassadors, and with 6 drawn only the corrupted
cities matter. Unity comes from freeing corrupted cities (5 each), clearing corruption markers and removing botnets.
The adversary's event deck is also the clock: when it runs out you lose, so watch the face-up card. When the
stage-III cards come up (the last three rounds), the Liberate action frees a corrupted city for 3 ambassadors
instead of 5, plus one for each adversary ambassador on it. Save ambassadors for those rounds, and clear the stacks
on a city with Remove first (one of yours takes 3 of theirs off). **What resisted a short summary:** troops count
against the purge until the sixth Unity card is drawn, and troops pile up wherever they have nothing to strike, so
below six cards the purge is out of reach in practice. The way to win there is the Unity deck.
