# 268: table-cost audit, what the game costs a human at the table (r125)

No rules change, no `RULES_REVISION` bump: measures, a ranking, and fixes to words, layout and the player aid.

## Method

- **Hand-play** (the staging default, standing): the team's decisions come from a recorded game by the judge
  (`strategist`), and a helper resolves everything else by hand from `docs/rulebook.md` and `docs/player-aid.md`
  only, as the table would: every adversary beat, production, threats, war, tipping and each action's effect. Cards
  are read from `docs/cards.md` and `docs/adversary-cards.md` as the physical components. After logging a round it
  reads the engine's end state for that round (`check.md`), logs each mismatch as an error and corrects to it.
  Brief: [handplay-brief.md](handplay-brief.md).
- **d2:** seed 268000-268003 traced; 268003 played in full (Builder in Horizon, Protector in Cintamani, 8 rounds, a
  purge win in round 8). **d4:** seed 268010 (11 rounds, lost to the round cap), setup and rounds 1, 5 and 10.
- **Tools:** [trace_rounds.py](trace_rounds.py) splits an `aic-sim run --trace` into the table script, per-round
  mechanical counts and the end-of-round states; [measure_read.py](measure_read.py) prints `design_check`'s table-cost
  proxies by level for a read. Run both from the repo root.
- **Rates** for minutes: 3 s a component, 10 s a per-site check, 20 s a lookup, 5 s a modifier, 15 s a real decision,
  20 s a new card face read.

## Per-round counts

d2, seed 268003 ([log](d2/log.md), [friction rows](d2/friction.md)):

| Round | Lookups | Modifiers | Memory peak | Memory total | Checks | Components | Decisions | Errors | Minutes |
|---|---|---|---|---|---|---|---|---|---|
| 0 (setup) | 5 | 7 | 0 | 0 | 11 | 85 | 2 | 0 | 9 |
| 1 | 17 | 12 | 12 | 14 | 81 | 127 | 40 | 1 | 40 |
| 2 | 13 | 28 | 12 | 15 | 91 | 170 | 44 | 1 | 42 |
| 3 | 13 | 35 | 13 | 16 | 133 | 206 | 52 | 2 | 52 |
| 4 | 14 | 38 | 16 | 18 | 150 | 246 | 51 | 0 | 59 |
| 5 | 14 | 59 | 18 | 20 | 227 | 367 | 68 | 3 | 85 |
| 6 | 15 | 50 | 22 | 24 | 165 | 265 | 68 | 1 | 69 |
| 7 | 16 | 61 | 21 | 23 | 170 | 320 | 80 | 1 | 77 |
| 8 | 13 | 33 | 13 | 15 | 120 | 208 | 58 | 1 | 56 |
| **game** | **120** | **323** | **22** | | **1,148** | **1,994** | **463** | **10** | **~490** |

d4 spot check, seed 268010 ([log](d4/log.md), [friction rows](d4/friction.md)):

| Round | Lookups | Modifiers | Memory peak | Memory total | Checks | Components | Decisions | Errors | Minutes |
|---|---|---|---|---|---|---|---|---|---|
| 0 (setup) | 6 | 8 | 0 | 0 | 14 | ~100 | 3 | 0 | 12 |
| 1 | 14 | 17 | 10 | 11 | 101 | ~205 (48 compute) | 49 | 0 | 45 |
| 5 | 22 | 34 | 12 | 14 | ~200 | ~420 (~180 compute moves) | 64 | 1 | 80 |
| 10 | 24 | 44 | 11 | 13 | ~250 | ~560 (~125 compute moves) | 87 | 3 | 103 |

The engine's own count of the same games ([d2](d2/trace_counts.json), [d4](d4/trace_counts.json), from the trace):
tokens moved a round 20-57 at d2 and 24-135 at d4; compute onto projects 12-29 at d2 and 38-81 at d4; real decisions
55-96 at d2, 51-88 at d4 (the engine splits a play into more steps than a human counts); round-scoped memory keys
9-23 at d2, 9-16 at d4.

**Concurrent load.** d2 peaks at 22 in round 6: Builder 12 (Fortify, Overcharge, Procure, Annex used; three building
Actions used; a converter; both Dispatches; a city bought; Master Plan's new discount), Protector 10 (Outpost, Expand,
Patrol, Bulwark; an Action; a chip converter; a city bought; 2 Vigil; a part payment to come back; a card with no
target). d4 peaks at 12 (round 5). d4 doesn't raise what is held in mind; it raises volume.

**Turn and setup time.** d2: 40-85 minutes a round, about 490 for the game (about 6 hours if a check that finds
nothing takes 3 s). d4: 45-103 a round, over 12 hours a game. Setup 9-12 minutes, 85-100 components. The adversary's
turn and production are 60-70% of a d4 round; the Builder's late action phase about a third of a d2 one.

**Method notes.** Two "engine disagreements" the d2 helper logged were its own errors: the trace's
`adversary.compute` is a running total of compute made (waiting compute included), not a project's, so the income
read 7 a country where it is 5 (§21 is right); and round 3's overflow was 12, under Purge Lists' 14. Both are
findings of row 5 (compute is hard to audit). Five cards it "couldn't find" in `docs/cards.md` were display renames
(the script printed ids: Demand Response Grid is Grid Analytics Desk); a printed card carries its name, so this is not
a table cost. Each helper once saw a check section early through a failed edit or a grep; both say so in their logs.

## The ranking

[docs/rulebook-friction.md](../../../docs/rulebook-friction.md) holds the ranked table (16 rows, with the concurrent
load column), the "What a per-rule review misses" section and the refresh trigger.

## Fixes made (words, layout, the aid)

- **Player aid** (`crates/aic-sim/src/docs/aid.rs`, numbers from the rules): the round on one table; *Once a round*
  (every once-a-round use, each board's powers by name, and the table convention of a spare cube on each one used);
  *Tip strips* (Unity's two tables on one row a tip, with the Discord cards left, and the tip thresholds by player
  count); *Corrupt numbers* at each level; *The march*; *What spends an ambassador*; *City growth* as a checklist; then
  the house terms as before.
- **Rulebook:** §3 the pool at level 4; §4 round start clears the used markers, step F includes a threat that waited;
  §5 Vigil "moved or not"; §6 Botnets spread parsed one way, natural corruption "only hexes holding 2 or more agents",
  whatever corrupts returns the agents on it, a threat's Fallout hex order; §7 a sacrificed card's raised income
  stays; §8 the Liberate action against a liberate card on a site with agents, where a canceled ambassador goes; §9 a
  city whose chips wait is skipped until a facility next to it is built or destroyed or a hex within 2 frees; §12 the
  pointer to the tip strips; §17 compute kept as a marker, not cubes; §20 Fortify hardens a player's facility. Each
  checked against the engine (a diff review caught three first drafts that weren't: the aid's growth order, Fortify
  narrowed to your own facilities, and the waiting-chips trigger).

## Needs a rules change (smallest fix each)

| Cost | Smallest rules change |
|---|---|
| Game length (6-8 h at d2, 12+ at d4) | A table-time budget per level as a design target, and the cuts below against it |
| Builder's late action phase | A card's rebate doesn't stack with a discount on the same play; or a play cap a round |
| City growth | A city at 3 chips that can't build loses them; or a blocked or corrupted city doesn't grow |
| Hosting | Hosts chosen at the start of the action phase with a host token, standing all round |
| Threat dealing | A country with no place for a tier's deck is skipped; a jam never takes a threat below 1 |
| The march | Troops on one hex move as one stack (one choice) |
| Projects' seven-hex count | Count the hex only, as Night Raid does |
| d4 compute volume | Compute and project costs in units of 5 (one cube = 5) |
| Card text (content, not rules) | Shorter wording for the four faces under 7.5 pt; Naval Blockade "no energy and no Discord"; event cards' Botnets spread line as the rulebook now reads; Liaison Corps' line split |

**Components (C)**, each the smallest physical change: a tip-count track per side with every mark printed (row 11);
a used-marker space on each power and building slot, and a "bought" chip for a city (row 2); a compute track beside
each project slot (row 5); income tracks, one marker a resource (row 8); a tag tally board, a dial a tag (row 1); an
icon row on event faces with the Blackout and Forced March glyphs (row 15); project faces' two agent-count glyphs
(row 13); a waits / delayed token for a threat that takes effect a round late (row 6); host tokens if hosting moves to
the action phase (row 10). Row numbers are the friction page's.

## design_check: the soft checks and their budgets

- **Mid-game modifiers a level:** 267's `ladder` (about one new a level, three active at the top); at r125 it warns
  level 1 (2 changes) and level 4 (4 active), as on main.
- **New (`table_cost` in `scripts/design_check.json`):** components moved a round, per-site checks a round and
  once-a-round uses a seat a round, by level, from a read's counters; budget each, and a 10% rise on `--old` warns.
  Budgets = 267's final read (`f267-read2`, 1,440 games, judge and greedy, three pairs) + about 10%:

| Level | Components (read) | Budget | Checks (read) | Budget | Memory a seat (read) | Budget |
|---|---|---|---|---|---|---|
| d0 | 22.2 | 24 | 44.2 | 49 | 4.0 | 4.5 |
| d1 | 32.6 | 36 | 46.2 | 51 | 4.1 | 4.5 |
| d2 | 44.1 | 49 | 46.6 | 51 | 4.0 | 4.5 |
| d3 | 68.5 | 75 | 48.5 | 53 | 3.8 | 4.5 |
| d4 | 80.3 | 88 | 53.8 | 59 | 3.8 | 4.5 |

  The proxies are lower bounds on the hand counts (they leave out cards, resources and checks the counters don't
  see: the hand-play's d2 components run 127-367 a round against the proxy's 44), so they watch movement, not level.
  A troop counts once a step: a move is an adversary op (`ops_already` drops the tokens check's troop counters), a
  strike is `board.troop_strikes`.
- **Refresh:** `friction-stale` warns when `RULES_REVISION` is 5 or more past the friction page's title revision.
