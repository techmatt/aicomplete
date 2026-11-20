# 294: overhaul 3 of 7, troops and winning (r140 → r141)

Matt's rulings of 2026-11-19 (the prompt's nine). The rules as built: [docs/rules/troops.md](../../../docs/rules/troops.md),
[adversary.md](../../../docs/rules/adversary.md), [alignment.md](../../../docs/rules/alignment.md). Every card line
that changed, old → new: [content_changes.md](content_changes.md), written by
[`scripts/content/troops_294.py`](../../../scripts/content/troops_294.py). Everything here is a candidate for his review.

## The win, as a new player is told it

You win the moment no city on the board is corrupted. Nothing else counts: not botnets, agents or troops, and not
how many Unity cards you have drawn. Each country starts with one corrupted city, so the game opens two liberations
from a win (at two players) and the adversary's job is to corrupt more cities, and guard them, before you get there.

## The adversary's whole turn

1. **Card.** Flip the event card and deal its threats. Each site of *last round's* type still holding an agent takes
   1 more (escalate); the Militarist adds 1 to each factory holding an agent; every site of *this* card's type takes
   1 (Infiltrate). Then, if the card shows the icon: **Strike**, then **Redeploy**. Then its other lines.
2. **Income.** Next round's card goes face up. Each corrupted site makes one thing (a city 1 Discord, a research
   station 1 research cube, a factory 1 agent that walks down its arrow, a botnet 1 compute). Compute fills the
   leftmost project. Every 2 cubes a country on the research track move its marker one slot, and that slot's icon
   happens once; slot 7 loses the game.

There is no March beat. Troops do nothing in a round that shows no icon: they sit, and they guard.

## Troops

- **Garrison.** A new troop goes, in its country, on the lowest-numbered corrupted factory; with none, the
  lowest-numbered corrupted city; with none, the lowest-numbered city. Nobody chooses. It stays there.
- **Guarding.** A corrupted site with a troop on or next to it can't be liberated, by the Liberate action or by any
  card that liberates or frees a site (Rescue cards, Hostile Buyout, Great Liberation, Reclamation Crew, Back-Channel
  Talks, Unity's Reclamation). The Liberate action's target box prints it.
- **Strike.** Where the icon is shown: on the event card, which lies face up for a whole round before it resolves
  (stage II: Mobilization, Fifth Column; stage III: all four cards, where Muster 1 stood), and on slot 5 of the
  Militarist's research row (the row and its cube count are public, so the table sees the marker one check away).
  What it does: each troop destroys one player node, or player-owned research station or factory, on or next to its
  hex; the players choose which, and a protection absorbs the hit as for any destruction. **The troop is not used
  up.** A Strike that takes a seat's last node ends the game there. No other timing was built. Considered and set aside: a Strike every round (the march's rhythm, without the
  walk: it makes every garrison a standing node loss, which the ruling's "shown a round ahead" reads against), and
  a Strike on the troop's arrival (not seen a round ahead).
- **Redeploy.** When: only on its icon, on two event cards (stage II's Redeployment, which was Forced March, and
  stage III's Occupation after its Strike). What the table does: pick up each troop that is not on or next to a
  corrupted city and put it on the nearest corrupted city (a walk over land; ties: the lower country, then the lower
  hex number). No path is walked. The first reading built, "each troop guarding no corrupted site", moved a troop in
  1 of 96 hands-off games, because a garrison always starts on a corrupted site; the shipped reading also pulls a
  troop off a corrupted factory whose city is free.

## Where troops, stations and cubes come from now

| Source | What | Was |
|---|---|---|
| Research row, slots 4 and 6 (Garrison) | 1 troop a country | Fallout; Dread |
| Research row, slot 5 (Strike) | every troop strikes | Cells: cities |
| Botnet Conscripts (project, 11 a board) | each country's lowest-numbered botnet becomes 2 troops, which garrison | 3 troops on the botnet's hex |
| War Plants (project, 19 a board) | every adversary factory makes 1 troop on its hex | 2 troops |
| Firestorm III (Discord card) | 2 agents and 1 troop a country | 4 agents |
| Orbital Strike, Supply Raid, Call-Up (tier-1 threats) | 2 research cubes each | 1 troop |
| Manhunt, Conscription, Garrison State, Iron Curtain, Arsenal (tier-2 threats) | 3 research cubes each | 1 troop |
| Total War (tier-2 threat) | 4 research cubes | 2 troops that march at once |
| Data Breach, Zero Day (threats) | 2 and 4 research cubes | 1 and 2 (293 had halved them for K 1) |
| Backdoor Research, Lab Seizure, Station Coup (projects) | 2 research cubes a country | 1 a country |

No event card musters, no threat lands a troop (`content::threat_rule_break` refuses one), and a factory's output is
still one agent. No new line opens an adversary research station: the cube lines alone carried the smoke bar
(below), and a station line would have been a new verb on a threat or Discord card, which is item 5's.

**The Militarist's row, War Research, at K 2 a country:** 1 New factory · 2 Cells: factories · 3 Funding ·
4 Garrison · 5 Strike · 6 Garrison · 7 loss. (At r140, K 1: New factory, Cells: factories, Funding, Fallout,
Cells: cities, Dread.) Its one rule, Commandeered Plants, is unchanged.

## The Protector's Wall and Vigil

| | Old | New |
|---|---|---|
| Wall | Adversary Troops can't enter or strike a hex with your Node. | No adversary Troop is placed on, Redeploys onto or strikes a hex with your Node. |
| Vigil | The first two times each round a Troop ends its march next to your Node, gain 1 insight. | When Troops Strike, gain 1 insight for each Troop on or next to your Nodes (at most 2 a round). |

Wall keeps its job (the Protector's nodes and what stands on their hexes are never struck) and loses the part the
march gave it (blocking a path). Vigil pays when a Strike resolves instead of when a walk ends.

## What answers a guarded city

A troop is removed by: the standard action Remove a troop (7 credits, once a round, a troop in reach); thirteen
player cards (Breach and Clear, Signal Hijack, Checkpoint Network, Bounty Board, Ceasefire Monitors, Peacekeeper
Rotation; tier 2: Surgical Strike, Kettle Operation, Decapitation Strike, Annihilation Lance, Double Agents, Total
Containment, Peacekeeping Mission); the boards' troop powers; Unity's Peacekeepers and Disarmament Talks. It is
moved off by Push and Gather (Checkpoint Network, the Protector's Patrol). So every board has an answer at every
table, the standard action, and nothing was added. One gap: a guard that stands next to the city but outside the
liberator's reach (two hexes from their nearest node) can't be targeted by the standard action; the player needs a
node nearer, or a card that acts "on or next to" its target.

## The smoke bar

The judge, two players, three pairs, 48 seeds a cell: r140 (293's kept read) against this build on the same seeds
(50328-50375), then this build alone on fresh seeds (50376-50423). Every table, by pair and level, with the dev
screen: [smoke.md](smoke.md). The three pairs pooled, 144 games a level:

| Level | Wins r140 → r141 (same seeds) | Fresh | Rounds r140 → r141 | Fresh | Won by round 5 (r140 → r141; fresh) | Losses, fresh |
|---|---|---|---|---|---|---|
| d0 | 90% → 98% | 99% | 7.0 → 5.3 | 5.7 | 29 → 85; 73 | round cap 2 |
| d1 | 74% → 88% | 95% | 7.4 → 6.8 | 6.4 | 13 → 46; 58 | cap 3, Discord deck 1, slot 7 3 |
| d2 | 64% → 88% | 91% | 7.0 → 6.8 | 6.6 | 13 → 38; 40 | cap 3, slot 7 10 |
| d3 | 50% → 80% | 73% | 6.6 → 6.7 | 6.4 | 8 → 28; 41 | cap 2, Discord deck 6, slot 7 31 |

**Early wins** (Matt's answer to question 1: leave the win as ruled and report these; item 4 carries "a round-2 or
round-3 win is rare" as its bar, with the liberation price and the starting corrupted cities as its levers). Share
of all 144 games a level won by the round named:

| Level | r140: by round 3 / by round 5 / fastest | r141, same seeds | r141, fresh seeds |
|---|---|---|---|
| d0 | 0% / 20% / round 4 | 30% / 59% / round 1 | 20% / 51% / round 1 |
| d1 | 0% / 9% / round 4 | 12% / 32% / round 1 | 11% / 40% / round 1 |
| d2 | 0% / 9% / round 4 | 8% / 26% / round 1 | 10% / 28% / round 1 |
| d3 | 0% / 6% / round 4 | 6% / 19% / round 1 | 11% / 28% / round 1 |

A round-1 or round-2 win is 8% of fresh d0 games (12 of 144) and 6-10% at d1-d3.

No game timed out. The rise in wins is the new win and K 2 together (at r140 most losses were the K 1 track's slot
7: 12, 34, 49 and 68 of 144): this read does not separate them.

**Troops and the row, fresh read:**

| Level | Games with a troop | First troop, round (min-max) | By round 3 | Slot reached, median | Strike icons a game | Strike hits a game | Redeploy icons / troops moved a game | Games where a troop alone blocked a Liberate | Remove a troop a game |
|---|---|---|---|---|---|---|---|---|---|
| d0 | 20 of 144 | 7.1 (6-9) | 0 | 2 | 0.44 | 0.01 | 0.24 / 0.01 | 1 | 0.06 |
| d1 | 45 | 5.8 (3-9) | 3 | 3 | 0.69 | 0.01 | 0.27 / 0.00 | 5 | 0.24 |
| d2 | 83 | 5.0 (2-8) | 12 | 3 | 0.99 | 0.17 | 0.31 / 0.00 | 18 | 0.68 |
| d3 | 99 | 4.4 (2-8) | 27 | 4 | 1.14 | 0.22 | 0.35 / 0.00 | 31 | 0.88 |

- **Games finish and rounds are sane**, though short: 5.3-6.8 rounds against the 7-9 aim.
- **No pile runs away** (fresh, d0-d3): agents on the board peak at 14-15 on average (62 at most), Discord swayed
  15-31 a game (73), research cubes 11-20 (41), troops on the board 7 at most.
- **The row at K 2**, where players leave its stations alone (the dev screen's hands-off arm, d0, 96 games: Dispatch
  and Liberate switched off): slot 4 or later in 77% of games, slot 7 ends 17% (16 games), the first troop lands in
  round 7 (5 at the earliest). With players free to act the row stops at slot 2 or 3 and no d0 game is lost to it.
  One starting cube a country (`adversary.research_start` 1, screened beside it) takes the hands-off slot-7 losses to
  33% and two to 39%, with no change to normal games; the asked numbers already meet the bar, so they ship.
- **Troops appear late at the plain setting** (d0: never before round 5 on any seed, round 7 on average). On the
  same seeds r140 had a troop in 122 of 144 d0 games, the first in round 2.9 and 91 of them by round 3 (threats);
  r141 has one in 14, the first in round 6.7, none by round 3. At d2 and d3 the compute income resolves a project
  early, and Botnet Conscripts or War Plants lands a troop by round 3 in 12 and 27 of 144 fresh games (r140: 116
  and 112).
- **Games are still won, and most of them early**: at d0 half are won by round 5 and the quickest in round 1 (two
  Liberates). The ruling calls an early d0 win "fine, though not easy"; for the judge it is the usual game. Matt's
  answer (question 1): leave it, ship as ruled; item 4 sets how rare an early win is.
  **The brake screen** (dev seeds, d0, 96 games an arm; [brake.md](brake.md); neither brake ships): two corrupted
  cities a country at setup gives 93% wins in 7.0 rounds, 25 games won by round 5 and none before round 4; Liberate
  at 6 ambassadors 98% in 6.0 rounds, 43 by round 5; both 90% in 8.1 rounds, 14 by round 5. The shipped build on the
  same seeds: 99%, 5.3 rounds, 56 by round 5, the earliest in round 2. The first brake alone lands d0 in Matt's
  aims (at least 90%, 7-9 rounds); it is unread at d1-d3.
- **Strikes are rare and Redeploy is nearly inert.** A garrison stands on the adversary's own sites, so a Strike
  finds a player piece beside it about once in five games at d3 (r140's march took 1.0-1.6 nodes a game); and
  because a garrison starts on or next to a corrupted site, a Redeploy icon moved a troop in 2 of 576 fresh games.
  Troops matter as guards: at d3 a troop alone stood between a seat and a Liberate in 31 of 144 games, and the
  judge bought Remove a troop 0.9 times a game.

**After the diff review** the eval's win clock and five engine paths changed (below), so the first read of this
build (commit c45eabbd, same seeds) was replayed on the final build: every game differs, wins 90.8% → 88.2% over
the four levels (paired gap -2.6 ± 1.4). The tables above are the final build's.

## Checks

- **Diff review** (four finders, each set checked by a skeptic): 31 confirmed, 4 rejected. Fixed: Unity's
  Reclamation took the cube off a guarded site (the one freeing path without the guard); a blocked Strike tallied
  under a name no reader used; a Strike that took a seat's last node let the card's later lines and the research
  loop run on; Botnet Conscripts logged troops Wall had kept off; the research row's Strike was not shown ahead;
  the judge's win clock had lost its target (every horizon one round long); stale words in docs, generators and
  two scripts. Left for Matt: the liberate cards' target boxes don't print the guard (the house term does), and
  Demilitarized Zone's strengths II and III are the same line.
- **The player view** ([`web/scripts/troops-play.mjs`](../../../web/scripts/troops-play.mjs), seed 17, every move
  clicked): a troop garrisons Horizon #7 in round 1 with its guard link and shield; seat 1's bar reads "Liberate
  refused: 1 corrupted site in reach guarded by a troop"; the Strike notice shows under the face-up card a round
  ahead (round 5, Fifth Column); the game ends "WON in round 5: no city is corrupted"; no march, Muster or purge
  wording on any step. It runs on a demo override (`research_start` 8, `corrupt_at` 20, `station_cubes` 0) so that
  troops arrive in round 1: at the baseline they arrive after most games are won.

## What resisted a clean rule

- **"Toward" in Redeploy.** Moving one hex toward a city needs a path and a tie-break at every step; the rule moves
  the troop the whole way in one lift.
- **"A corrupted factory, else a city."** The ruling doesn't say which city. Built: a corrupted one first (the troop
  guards at once), then any, always the lowest-numbered.
- **Liberate cards that also remove troops** (Peacekeeping Mission: liberate, then up to 3 troops off in its
  country). The guard is checked before the card acts, so its troop removal can't open its own target. Left as is;
  reordering it is a card change for item 5.
- **Threat names.** Orbital Strike, Call-Up, Conscription, Garrison State and the others now put cubes on the
  research track under a troop's name. The lines are legal and close in worth (a troop at 4.67, a cube at 2); the
  names want item 5.
- **The engine's names.** The win's end reason code is now `liberated` (readers take `purge` too, for older games
  files), but the helpers are still `purge_count` and `purge_won`, and `Op::Troops` keeps the threat-era fields no
  content prints. Both are listed for cleanup.
