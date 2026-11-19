# 280: bookkeeping cuts (r133): production from the tracks, arrivals checked as placed, one move a botnet

Matt's rulings (2026-11-18, after 278): make 278's cut 2 (production read from the income tracks, with a "blocked"
chip), cut 4 (an arrival's check folded into its line's site read) and cut 9 (compute and botnet seeding as one move).
They are bookkeeping cuts: what a good player decides should not change.

**Result.** All three are made, and **play is unchanged**: the 28 traced games replay r132's event for event (less
the waiting-pile moves cut 9 removes), and the fresh paired read's 1,440 games a side match r132's game for game
(below). `RULES_REVISION` is bumped from the constant, 132 to 133, because the printed procedure changed (the
rulebook carries the revision). The golden fingerprint did not move; the content hash did (Botnet Recruiters' text).

**Minutes (278's rates and `breakdown.py`, 278's seeds 268000-268013, the judge):**

| Cut | d2 min a round | d4 | 278's estimate (d2 / d4) | What it removes from a round (d2) |
|---|---|---|---|---|
| 2, production from the tracks | **−1.47** | −1.13 | −2.8 / −2.8 | 17.8 site reads become 6.2 reads (tracks, chips, adversary factories; d4 7.5): −1.94; adds about 9 components (track moves when a site changes hands, chips placed and lifted): +0.47 |
| 4, arrivals checked as placed | **−0.60** | −0.55 | ≤ −1.3 / −1.2 | 3.6 of the 7.6 arrival checks (the ones on a site an Infiltrate, Surge or Grow line names); 4.0 stay (Spills, seeds, threats' and cards' agents) |
| 9, one move a botnet | **−0.01** | −0.01 | −0.4 / −0.6 | the waiting pile's move in rounds where nothing waits; the second read of each botnet (3.3 a round at d2, 3.8 at d4) is outside 278's model (est. −0.5 / −0.6 at 10 s) |
| **All three** | **−2.08** (42.4 → **40.3**) | **−1.69** (42.9 → **41.2**) | −4.5 | games 339 → 323 minutes at d2 (8.0 rounds), 272 → 262 at d4 (6.4) |

The paired SE across the 14 seeds is ±0.1 or less: the games are the same, so the spread is only how much each
game's board uses the procedure. Full tables after: [breakdown-d2-after.md](breakdown-d2-after.md),
[breakdown-d4-after.md](breakdown-d4-after.md) (before: 278's [breakdown-d2.md](../278_hard_cut_proposals/breakdown-d2.md)).

**Why each falls short of 278's estimate:**
- **Cut 2.** 278 assumed 2-3 chips placed and lifted a round. The replay counts about 9 components a round:
  - agents land on players' and neutral stations most rounds: 2.9 chips and moves from the compute step's seeds,
    1.9 from the lines;
  - the seats' ambassadors on adversary sites come and go in the action phase: 3.4.
  
  Production still reads 6.2 things a round, not 2-3: each producing track (both seats', the adversary's, the
  neutral stations'), each chip, and each adversary factory, which makes its troop where it stands.
- **Cut 4.** Only the arrivals on a site the line itself names fold. A Spill's arrival, a botnet's seed, and the agents
  a threat or card places land on hexes no line named, and keep their check (278 flagged this as a ceiling).
- **Cut 9.** 278's breakdown counts a track marker once a phase, however far it moves, so "half of compute's 17
  components" was never there to cut: the step already moved one marker a project. What the cut really removes is
  the second read of each botnet (once to seed, once for compute). 278's model never counted that read, so the
  breakdown can't see it go.

## What changed

1. **Production from the tracks** (cut 2; a table procedure, no engine change).
   - What each player's factories and stations make sits on their income tracks; the adversary's corrupted cities'
     Discord and stations' cubes on its board; each megaproject counts its neutral stations.
   - A marker moves only when a site changes hands.
   - A **"blocked" chip** goes on a producing site when an opposing piece lands there and comes off when the last one
     leaves. In production each track pays what it shows, less what its chipped sites make.
   - Neutral cities are still read for their hosts.
   - The engine computes the same output it always did. The chips are the table's record of what `production.rs`
     reads.
2. **Arrivals checked as placed** (cut 4). An Infiltrate, Surge or Grow line checks each site it names as its agents
   go on (`place_named`); the check is the same, only the count moves (`map.arrival_folded`, out of
   `map.adversary_checks`).
3. **One move a botnet** (cut 9).
   - The compute step opens with the botnets, in hex order. Each one with no player ambassador seeds and adds its
     compute in the same read (`botnets_seed` returns the compute; a hack still stops the compute, a cordon only the
     seed).
   - **One clause keeps play identical:** a botnet passed as blocked, whose ambassador a later seed cancels, adds its
     compute then. Without it, 2 of the 28 traced games diverged. In both, Cintamani's two starting botnets sit side by
     side: the Protector stands on the lower-numbered one, and the other's seed cancels it. r132 read compute after
     every seed, so it counted that botnet.
   - The step's own compute goes straight onto the projects. The waiting pile holds only compute made outside the step.
     There is one exception: with compute waiting to go onto every project (`ComputeEvery`, which no content prints
     today), the fresh compute waits as it did in r132, so a project resolving first shows the agents the same pile.

**Not made:** the no-pass draft, the turn budget, hosts fixed, threats by any tag, Growth printed (Matt's rulings).
`cards.no_pass` and `board.turn_cap` stay at 0.

## The paired read: play unchanged

f280-read against r132 (277's `f277-read` spec, farmed at 6b02d53), fresh seeds **50028-50075** (taken; next free
50076). The judge and greedy, three pairs, d0-d4, 48 seeds a cell: 1,440 games a side.
- [same_play.py](same_play.py): **0 games differ** outside the counters.
- The counters that differ are only the three the cuts re-count: `map.arrival_checks`, `map.adversary_checks` and
  `map.arrival_folded`. `adversary.compute_botnets` and `adversary.compute_blocked` match game for game.
- Win % 43.0 ± 1.3 on both sides, paired gap +0.0 ± 0.0 (`scripts/farm/compare.py`).
- No level moves, so there is no cause to name.
- After the diff review's fix to `pour` (a path no content reaches), the same seeds were farmed again on the final
  build (14ccc26): again 0 games differ from r132.
- **Speed** (`speed.py pair`, r132 against r133, quiet box): random, greedy, engine and rollout replay the same games
  at 0.97-1.04 x. The planner rows were stopped unread: they take tens of minutes and add nothing for identical games.

## Files

- [breakdown.py](breakdown.py) is 278's script, plus `--tracks` (cut 2's replay: the trace's tokens give each
  city's, station's and factory's producer and blocker) and `--tracks-only`.
  - Without the flag, on a trace without r133's counters, it reproduces 278's 42.4 / 42.9.
  - **Not seen:** blackouts and Factory Occupation (round flags and vars, not tokens), so the chip count is a floor.
- `python experiments/results/280_bookkeeping_cuts/breakdown.py BASE NEW --compare --level 2 [--tracks-only]`:
  - BASE is 278's base traces (conductor's kept `278/trace-r132-base-and-flags.tgz`, `base3/`);
  - NEW is `sh experiments/results/278_hard_cut_proposals/traces.sh OUT` on this build.
  - Without `--tracks-only` it gives cuts 4 and 9; with it, all three.
- [same_play.py](same_play.py): the paired read's same-games check. [firstdiff.py](firstdiff.py): the first differing
  event between two traces (waiting-pile moves set aside); it found cut 9's edge.
- `experiments/farm/f280-read.json` is the new side's spec. The farmed games are kept at conductor's kept folder for 280.

## For new players

- **Production comes off your tracks.** Your income tracks already show what your factories and stations make. When
  an agent lands on one of them, put a blocked chip on it. Take the chip off when the agent goes. In production,
  collect what the track shows, less what the chipped sites make. You don't walk the map.
- **Check a site as the agents go on.** When an Infiltrate, Surge or Grow line puts agents on a site, count that
  site's agents right then. If it has reached its number, it corrupts. There is no second pass.
- **A botnet is one move.** In the compute step, take each botnet in turn. One with no ambassador on it puts its agent
  next to it and adds its compute in the same move. Then everything goes onto the projects at once.
- No rule resisted the cut. Cut 9 needed one clause to keep play the same: a seed that cancels the ambassador on a
  botnet you already passed lets that botnet add its compute.
