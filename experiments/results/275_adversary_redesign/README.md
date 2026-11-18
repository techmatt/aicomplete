# 275: the card-driven adversary (r130)

Matt's ruling (2026-11-18): table time. The adversary acts only where a card, project, threat or botnet names a
site, or where its agents arrive; nothing sweeps the board. `RULES_REVISION` 129 → 130 (bumped from the constant).
Base: r129 (274). The rules as built, every old job and what took it over, and every step with what it acts on are in
[design.md](design.md). Every shape below the rulings is a **candidate**.

## The rules in one table

| Step | What it acts on | Before (r129) |
|---|---|---|
| Infiltrate X (event card) | 1 agent on each site of the card's type (2 at d4); stage III cards name two types | Rally: each country's pool dealt round-robin |
| Arrival corruption | the hex an agent just landed on | Corrupt X lines (move every agent next to an X onto it) and natural corruption (every site) |
| Spill (level 2+) X into Y (10 cards) | each X holding 2+ agents, 1 onto the Y next to it holding the most | (none) |
| Grow X (12 cards) | the X holding the most agents in each country (with room, fewer than 2 facilities next to it) | city growth: every city gathered chips every round |
| Surge N: X (projects, Discord cards) | the X holding the most agents in each country | "Rally 3: X" and Corrupt lines |
| Botnet Recruiters (board) | each botnet: 1 agent onto the site next to it holding the most | Garrisons: +1 agent income |
| Uncountered threat | 1 agent on its anchor | (none) |

Gone: Rally, the pools, agent income (`ambassador_income`), Corrupt X lines and their adjacency move, natural
corruption, Corrupt open land (`corrupt_land_at`; Blitzkrieg and Naval Blockade print a Fallout line), the Liberated
marker and its +1 (`retake_agents`), city chips (`city_chip_cap`, `city_build_chips`), the Corrupt beat (the turn is
Card / March / Income). Kept: Sleeper Cells (d1: "When an agent lands on a city, it falls at 3 agents, not 5"),
2 agents on a node (on arrival), Muster, the March, projects, compute, threats, winning.

**Candidates I set (adjacent numbers and small rules):**
- **`grow_cap` 2.** Uncapped, one Grow a country a round built 9.1 facilities a game on the dev seeds against city
  growth's 4.5; with the cap 4.0 ([screen](#grow-screen)). The ruling's shape (a site holding agents grows its
  facility with an agent on it, `grow_infest_at` 1) is kept: raising it to 2 changed nothing measurable.
- **Arrival = the adversary's placement or move** (staging default): a player's Push or Gather checks nothing; a
  canceled arrival checks nothing (diff review).
- **Escalation printed** (staging default): stage III cards name two Infiltrate types; no agent-income tracker.
- **The Liberated marker is cut:** liberation clears the site's agents, so it needs its full number of arrivals again.
- **Cordons** stop every line in the country (Infiltrate, Surge, Spill, Grow, seeds, a threat's anchor agent):
  nothing waits now that pools are gone. `values.json`'s cordon 2 → 4 (a turn of arrivals stopped outright).
- **Surge counts** on the nine projects that rallied or corrupted (Surge 2 on a station or factory, 3-4 on a city);
  Discord cards that ended "then Corrupt cities" create 1 agent more instead. Project costs unchanged: at the
  per-agent row the surged projects price at 0.3-0.5 a compute ([projects.json](../../../drafts/adversary/projects.json)'s
  value lines), but a Surge 2 onto a station takes it outright, which that row underprices. Flagged for the balance pass.
- **The judge reads the botnets' coming seeds** (`w.seed_read` 1) beside the Infiltrate forecast: on/off on the dev
  seeds, the judge +6 points pooled (d2 +10 ± 6), greedy −2 (noise).

## Table time (done-when 2)

268's rates (10 s a check, 3 s a component) on 274's traced seeds (268000-268013, the judge, Builder and Protector),
r129 against r130 ([table_time.py](table_time.py), [table_time.md](table_time.md)). Checks before are 268's proxy
(`design_check.json` table_cost at r129: every facility twice, for the Corrupt lines and natural corruption, every
city twice for growth, each troop step, each agent Rally dealt, each threat dealt); after, each site a line names,
each arrival, each botnet seeding, each troop step and each threat dealt.

| Level | Checks a round | Components a round | Minutes a round moved |
|---|---|---|---|
| d2 | 47.7 → 20.4 | 37.8 → 36.9 | **−4.6** (274's 57.3 → 52.7) |
| d4 | 54.8 → 27.0 | 72.2 → 72.0 | **−4.6** |

The adversary's per-site checks fall by more than half; components (agents, troops and facilities moved) are flat:
the arrivals that replace the pools move as many pieces. Seed 268003 (268's hand-played game) runs 8 rounds on r130
against 10 (the game diverges at round 1). On the fresh read the proxy reads +15-19% components at d0-d2 (more
arrivals: 7-11 a round against Rally's 4-10). 274's ~2 h target still needs the hard cuts it listed.

## Fresh paired read (done-when 3)

f275-read, seeds 49884-49931 (taken), 1,440 games a side, the judge and greedy, three pairs, d0-d4; r129 natively
from its tree, r130 farmed. Gaps paired by seed (a seed's mean over pairs), ± SE.

| Level | Judge win % r129 → r130 (gap) | Greedy | Judge rounds | Cities corrupted by round 4 (judge) | By the game's end | Node losses (games of 144) |
|---|---|---|---|---|---|---|
| d0 | 91 → 85 (−5.6 ± 3.8) | 93 → 87 (−6.2 ± 3.5) | 8.90 → 9.27 | 0.00 → 0.20 | 0.20 → 0.49 | 0 → 6 |
| d1 | 68 → 54 (−13.9 ± 5.7) | 69 → 51 (−18.1 ± 5.3) | 10.98 → 9.74 | 0.24 → 1.66 | 1.92 → 3.72 | 8 → 49 |
| d2 | 65 → 30 (−35.4 ± 4.6) | 59 → 32 (−27.1 ± 5.0) | 10.83 → 9.79 | 0.51 → 2.16 | 2.23 → 4.92 | 11 → 83 |
| d3 | 24 → 10 (−13.9 ± 4.2) | 19 → 7 (−11.8 ± 4.0) | 9.99 → 7.85 | 0.91 → 2.98 | 2.66 → 5.16 | 46 → 106 |
| d4 | 14 → 0 (−13.9 ± 3.7) | 3 → 1 (−2.8 ± 1.9) | 8.72 → 5.96 | 1.46 → 3.14 | 3.31 → 4.20 | 92 → 139 |

**The adversary spreads more at every higher level** by round 4 (judge 0.20, 1.66, 2.16, 2.98, 3.14; greedy alike).
By the game's end d4 (4.20) sits under d3 (5.16): the cause is game length, not spread. d4 games end at round 6.0
(node losses in 139 of 144), so fewer rounds are left to corrupt cities.

**Why the drop.** The loss is to the last node (`node` endings 6 → 139 at d4). The agents land on more sites a round
(7-11 arrivals, against Rally's 4-10 at r129), every Infiltrate puts one on each named site whoever owns it, and
a station or factory falls at 2: corruption is more frequent (corruptions 14.2 → 21.5 a game at d2), and an
adversary factory makes troops, which march on nodes. Per the prompt the bands are reported, not chased.

**Grow screen** (dev seeds 36800-36815, 1,920 games, the judge and greedy pooled): uncapped, win % d0-d4 78 / 44 /
36 / 9 / 2 with 9.1 facilities grown a game; `grow_cap` 2: 87 / 54 / 36 / 13 / 4, 4.0 grown (r129: 4.5);
`grow_infest_at` 2 alone: no change. Kept `grow_cap` 2, the infest rule at 1 (Matt's candidate).

## Resource use (the judge, r130, f275-read)

| Resource | Gained a seat-round | Spent | Spent of gained | Top uses |
|---|---|---|---|---|
| Credits | 18.0 | 19.1 | 106% (binds; the start covers the rest) | cards 67%, standard actions 24% |
| Insight | 6.0 | 4.8 | 81% (half binds: the rest is lost at round start) | keeps 58%, plays 20% |
| Energy | 4.5 | 3.9 | 86% (r129 81%) | powers 65% |
| Chips | 0.8 | 0.6 | 76% | converters 59%, powers 30% |
| Antimatter | 0.3 | 0.2 | 50% | cards |
| Research cubes | 0.6 | 0.5 | 77% | converters, building Actions |
| Ambassadors | 1.7 | 1.6 | 93% (binds) | standard actions 97% (Remove agents, Liberate, Dispatch moves) |

Ambassadors and credits bind harder than at r129 (90%, 105%): more arrivals to cancel and remove.

## Agents in and out (docs/harms.md)

A game-round, the judge: arrivals 7.1 / 8.6 / 9.0 / 10.1 / 10.9 at d0-d4, of them Infiltrate 4.4 / 3.4 / 2.9 / 2.6 /
4.2 (the rest: threats' anchors and creation lines, Grow's agents, seeds, Surges); canceled on arrival 0.4-0.6;
removed by players 2.2 falling to 1.0; returned by corruption 2.1 rising to 7.4. The table is generated into
[docs/harms.md](../../../docs/harms.md) from `AGENT_FLOW` (refresh with [flow.py](flow.py)).

## Checks

- **Diff review** (Ultracode: four reviewers by dimension, then skeptics): 22 found, 16 fixed, 6 rejected (see the
  report). Fixed: an arrival check where every agent was canceled; a held station flip that corrupted on its arrival;
  cordons in the seed forecast and on a threat's anchor; the eval canceling one ambassador against two arrivals;
  stale words in op docs, the cordon flag, the aid's tracks, the rulebook's cordon and Grow; two browser scripts
  setting the removed `corrupt_land_at`; Surge's harms; `spill_at` in the ladder table; the board's and projects'
  value lines.
- **design_check** on the read with `--old` r129: see the report's warnings line. The table-checks "rise" compares the
  new proxy (site passes 0) on both reads; under r129's own formula the old read is ~48 a round, not 2.5-8.3.
- **Speed** (`per_decision.py`, d2, quiet box, best of 3): the judge 1.22× a decision (1.1 → 1.4 ms), greedy 1.06×.
  Kept: the judge now reads the botnets' coming seeds (`w.seed_read`, about 1.12× of it, worth +6 points on the dev
  seeds) and an exact Infiltrate forecast every evaluation; `speed.py check --update` re-based.
- **Smokes:** the 16-seed loop smoke has no timeouts and no card at the play flag; `cards:fit` clean.

## New-player summary (r130)

- **The event card ahead shows where agents land** (Infiltrate: one on each site of its type, two types late in the
  game) and **which site grows** (Grow: in each country the site of its type holding the most agents builds a
  neutral factory or station next to it, with an agent on it if that site held agents).
- **A site corrupts when an agent lands on it** and it then holds its number (city 5, 3 from level 1; station and
  factory 2). Two agents landing on your node's hex destroy the node. Nothing is checked where nothing landed.
- **From level 2, Spill lines** move one agent from each crowded site (2 or more) onto a neighboring site.
- **Each botnet** puts an agent next to itself every round; **each threat you don't pay** puts one on its spot.
- **Projects Surge** agents onto the most-crowded site; a Surge 2 takes a station or factory unless you stand there.
- **Your answer:** an ambassador on a site cancels one arriving agent; Remove agents before a site is one short;
  firewalls, no-Infiltrate and no-Corrupt tokens, and cordons (a whole country's turn) still block.
- **Rules that resisted:** the win rate at d2-d4 (bands not chased); 274's 2-hour target (needs the hard cuts).

## Files

- [design.md](design.md): the rules as built, each old job's replacement, each step's target.
- [table_time.py](table_time.py), [table_time.md](table_time.md): checks and components a round, before and after.
- [screen.py](screen.py): win %, rounds, cities corrupted and growth by arm and level. [flow.py](flow.py): harms.md's
  agent flow. [resources.py](resources.py): resource use.
- Specs: `experiments/archive/farm/f275-read.json`, `s275-smoke.json`, `s275-grow.json`, `s275-trace.json`,
  `s275-seedread.json`. Read outputs: `out/keep/275/` (kept by conductor).
