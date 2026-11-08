# 151: the adversary's income and rally (r65)

Matt's income-and-rally direction (2026-11-08, locked; design doc *Adversary system*). `RULES_REVISION` 64 → 65,
bumped from the constant (the prompt also said r65).

Scripts here: [adversary_cards.py](adversary_cards.py) (the one generator: the Militarist, its 15 events and the 20
projects, written to `drafts/adversary/`, with each card's give / get and pressure line in [cards.md](cards.md)),
[specs151.py](specs151.py) (screen and read specs), [ladder151.py](ladder151.py) (the paired ladder and the standing
measures), [run_old.sh](run_old.sh) (r64's binary on the read specs). Reads: [f1.md](f1.md) and on.

## The rules

- **Income, every adversary.** Compute income (`compute_income`, a list by difficulty, fixed, not per board) goes onto
  the projects, and ambassador income (`ambassador_income`, a board) into each board's pool, at the end of the
  adversary's turn (the reveal step); setup fills the pools once for round 1. Boards, events and projects raise an
  income with the new op `increase_income` ("Income: +1 ambassador a board a round"); the Militarist's old creation
  line (2 on its lowest-numbered corrupted city) is now its **Garrisons** rule, +1 ambassador a board. No troop
  income: troops are printed (Deploy). Project costs stay per player. No player card lowers an income yet (player
  cards stay as printed); the op would be a negative `increase_income` on a player card, which the slots refuse today.
- **Rally X** (new op `rally`): every event's first step after its threat lands each board's whole pool on that
  board's sites of type X: the neutral ones, else the players', else the adversary's, spread evenly; the players
  choose which sites take the remainder, one each (the candidate tie rule). The supply caps it; a cordoned board's
  pool lands nowhere. A project may print a counted rally ("Rally 3, each board: botnet hexes").
- **The event shape:** threat, rally, deploy, other lines (at most two), corrupt, always in that order. Per-event
  compute and every creation line are gone. Event, project and board troops land deterministically, on each board's
  lowest-numbered factory (else city); a project's station flip takes the lowest-numbered station. Threats keep
  their own targeting (player cards, AI boards and threats stay as printed).
- **"Each board", never "each country"**, in every event, project and board line.
- **The standard's checks.** `content::event_rule_break` / `project_rule_break` (the load check) and `check.py`'s
  `event_breaks` / `project_breaks` refuse: an event without exactly one Rally X, a second (counted) Rally, a step out
  of order (a Corrupt line before the corrupt step included), more than two other lines, a per-event compute or
  creation line, "each country" in any event, project or board text, and a project with more than two consequences.
- **`AdversaryBuild`** ("the adversary builds now": Grand Offensive, the Offensive Discord card) now rallies each
  board's ambassador income to this round's card's Rally site.
- **The ladder.** `build_bonus` is retired with the board's creation line; d3's step is its income entries and
  `stage2_removed` 2. New `stage1_added` (0): the prompt's smaller d3 step, screened and not taken.
- **Market Stabilization Board** joins the baseline deck (Matt), after Microgrid Lab Compact as 148's config had it.

## Give / get

Every card's give / get, written before the first screen, is in [cards.md](cards.md) (name, then each line as
printed, then its give / get and, for events, the pressure it makes). Prices at `configs/agents/values.json`: a
rallied ambassador 2 a board (the new `rally` row), a troop 9 with its strike, a Corrupt line 10, an income rise per
round left at the `increase_income` row (1.1 a compute; an ambassador a board counts 2 compute a board).

- **Events:** harm 34-86 at two boards against the players' cheapest answer, a removal a piece landed (5 each):
  0.85-1.91x, the pulls priced on their own threat cards. Factory Drive reads 0.85x (flagged, kept): its new neutral
  factory is a gain to the players too, a target both sides contest.
- **Projects:** 0.61-3.28 harm a compute absorbed (two players); the one-shot player harms (Raiding Parties, Purge,
  Market Crash, Data Heist) sit high because a compute point is 1.1 and a card 4.
- **The board:** Garrisons 4 a round at two boards (0.8x a removal, every round); Wall troops 1.8x; Warmongering as before.

## Rally targets and pressure

Five site types across the deck: coastal cities (Probing Strikes, Mobilization, Naval Blockade), cities (War Drums,
Fifth Column, Forced March, Blitzkrieg), research stations (Long Patrols, Station Raids, Occupation), factories
(Factory Drive, Border War, Supply Lines, Total Mobilization), botnet hexes (Signal Jamming; the Botnet Swarm
project). Most cards corrupt the type they rally to: the pool lands where the Corrupt line gathers. Each event's
pressure line is in [cards.md](cards.md).

## The ladder (strategist, paired against r64's binary and configs on the same fresh seeds)

Two 48-seed sets × three pairs a read, 288 games a level; r64's binary carries one measures patch (the project
tally, amendment 6618), no play change. Screens are in [screens.md](screens.md); read tables in [f1.md](f1.md),
[f2f3.md](f2f3.md), [final_f4f5.md](final_f4f5.md).

| Read | Dials (compute income by level; others) | Seeds | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|---|
| F1 | 5,5,8,10; `counter_bonus_credits` 3 | 38392-38487 | 95.1 (r64 91.7) | 84.4 (76.7) | **66.3** (64.6) | 47.6 (41.7) |
| F2+F3 | 5,5,10,10; 3 | 38488-38679 | 97.7 (90.3) | **85.6** (78.1) | 60.9 (57.6) | 50.0 (38.7) |
| **F4+F5 (final)** | **5,5,8,10; 6**; the search fix | 38680-38871 | **95.0** ±0.9 (92.2) | **79.7** ±1.7 (79.2) | **62.5** ±2.0 (62.2) | **47.0** ±2.1 (42.5) |

Ambassador income 2,2,2,3,3 a board on every read (plus Garrisons' 1). Bold in a read row is outside its band. All
four levels are in band on the final read, none within a point of an edge, so no more reads were pooled. F1-F3 ran
the build before the agents' search fix (below), so they are not pooled with the final read. The F4+F5 sets run 5
points apart at d1 (84.7 / 75.0 in F5): sets differ as 150 found.

**What the dials did.** Ambassador income is the strong dial at d2 (1 → 2 a board: −19.8 on screen 1) and weak at
d0-d1; compute income moves d2 and d3 (+2 at d2 −5 to −7, +5 at d3 −6) and barely d1 (+1.6 for +1). d1 needed one
of its own dials: `counter_bonus_credits` 3 → 6 (−6.8 at d1, −5.2 at d2, −2.6 at d3 on the dev seeds), with d2's
compute eased back from 10 to 8 to hold d2. The prompt's smaller d3 step (one stage-II card out, one stage-I back)
made d3 16.7 points easier than `stage2_removed` 2; d3's step is its income entries (ambassadors 3, compute 10).
**The prompt's stated numbers** (compute 5, ambassadors 1 a board, every level) ran in every screen: easier than the
final dials by 6-9 at d1, 10-27 at d2 and 7-12 at d3.

## The measures, r64 against r65 (F4+F5, 576 games a level)

| Level | Nodes lost to corruption a game (r64 → r65) | Adversary ambassadors placed (of them rallied) | Projects revealed, share of the deck | Resolved, share | Reshuffles a game |
|---|---|---|---|---|---|
| d0 | 2.13 → 1.56 | 74.6 → 65.2 (64.4) | 21% → 30% | 8% → 16% | 0 → 0 |
| d1 | 2.69 → 2.38 | 86.2 → 76.9 (74.0) | 25% → 36% | 12% → 22% | 0 → 0 |
| d2 | 2.75 → 2.45 | 90.7 → 83.5 (79.5) | 36% → 55% | 22% → 40% | 0 → 0 |
| d3 | 2.98 → 2.74 | 117.9 → 94.3 (90.2) | 37% → 60% | 23% → 44% | 0 → 0 |

r65 places fewer ambassadors and takes fewer nodes by corruption, but corrupts slightly more sites (9.9-16.7 a game
against 9.9-15.4): the pool lands where its card's Corrupt line gathers. Compute income roughly doubles the project
turnover; at most 60% of the deck is seen a game and no game reshuffled it (amendment 6618: more projects would
show more of the deck only if turnover rises again). Distinct projects revealed equal the revealed count: none is
seen twice.

## The agents

- **The rally read** (`w.rally_read`, 1): `plan::next_rally` forecasts the coming Rally (the card ahead's site
  type and each board's pool, spread evenly, a cordoned board out until its cordon will lapse), and the eval counts
  those ambassadors toward each site's corruption a round ahead (pressure, at-risk and node exposure), less any a
  player ambassador on the hex would cancel. On / off and cost: [screens.md](screens.md). The remainder choice is
  left to the search (it picks by the eval, which favors the party).
- **Values:** new rows `rally` (−2 an ambassador a board) and `increase_income` (−1.1 a compute a round left; an
  ambassador a board 2 compute a board).
- **The search fix:** two strategist games (screen 3, F2) panicked on replay: a project's "each player pays N or
  discards M cards" asks every seat, card by card, mid-resolution, and the search's guess of the hidden piles and
  hands changed what the replay read. The search now skips the guess while a choice is pending
  (`search::guess_unless_pending`): the leak is the true table for one pending choice. No failure since (0 in 2304
  final-read games).
- **Review before the first fresh read:** a 14-agent review of the diff found nine real faults (the cordon's
  forecast, the difficulty line's wording against project Deploys, the pool's supply wording, stale docs, two check
  gaps, the cancel in the forecast, a Rally's value); all fixed before F1.

## Smoke reads and checks

- `loop_smoke.py --run`: no timeouts, at most 2 plays of one card a game.
- `play_read.py` on the final read (2304 strategist games): nothing kept over 6% and never played. Market
  Stabilization Board, now in the deck: kept 48-74%, played 24-51% of games (d3-d0).
- `npm run cards:fit`: passes (no player card changed).
- **Speed** (`per_decision.py`, r64 against r65, seed 1000, one thread): the strategist 1.9 → 2.1 ms a decision
  (1.13×, 16 games), greedy 29.1 → 34.0 (1.17×, 6 games); the rally read is greedy's 1.19× and the strategist's
  0.97× of it on r65 (on / off). Kept: the forecast runs once a position. `speed.py check` gave no verdict across
  the revision (its baseline was r62); the baseline is updated to r65.
- No `bench/same_results.py` replay was needed: no rules code changed after the final build (F4+F5); later edits are
  docs, tests and the golden constants.
