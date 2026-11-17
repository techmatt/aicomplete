# 259: every adversary gets a research rule (r120, r121)

Matt's rulings (2026-11-16, locked): every adversary gets a research rule; its research cubes stockpile and fire an
adversary-specific effect at a threshold; difficulty lowers the threshold, and d3-d4 get a stronger effect; sources are
the adversary's stations after a project, some projects, and cards; players get cards that remove adversary cubes.
Staging defaults (amendment 11878, all standing): a per-board schema with only the Militarist's effect built; corrupted
stations after their country's megaproject feed the track instead of compute; adversary cards only as card sources;
the Militarist's effect is troops; firing spends the threshold and keeps the overflow, checked once a round; Backdoor
Research keeps its line and adds a research line.

## The rule

- **One track** on the adversary board (`adversary.research`, public). Once a round, at the end of the Income beat's
  compute step, a track holding `research_at` cubes a country (times the countries) spends that, keeps the rest, and
  fires the board's research rule once.
- **The Militarist's Weapons Program:** Muster 2 (each country, 2 troops on its lowest-numbered factory, else city);
  from `research_strong_at` (3) Muster 3. It is a Muster, so the board's difficulty line (from d2) adds 1 a country
  and a no-Muster token or Wall stops a troop.
- **Sources:** a corrupted research station whose country's megaproject is complete puts `station_cubes` (1) a round
  on the track (it made compute before); three projects (Backdoor Research, Lab Seizure, Station Coup: 2 a country);
  three threats' uncountered lines (Wiper Malware 2, Data Breach 2, Zero Day 4).
- **Removal:** Counterintelligence Sweep and Arms Inspectors (below). The off arm is `research_at` all 0: stations
  make compute again and research lines do nothing.

## Candidates (each with its reason)

| Number | Value | Reason |
|---|---|---|
| `research_at` | 5, 5, 5, 4, 4 a country | fires 0.1-0.3 / 0.6-0.8 / 0.9-1.3 / 1.5-1.9 / 1.8-2.0 a game (d0-d4, judge / greedy, dev seeds); the lower 4,4,4,3,3 and 5,4,4,3,3 cost the judge 5-6 points (d2-d3) for about 0.5 more fires |
| `research_strong_at` | 3 | Matt: d3-d4 get the stronger effect |
| Weapons Program | Muster 2; strong Muster 3 | 4 troops a fire at 2 countries (20 at the anchors), the size of a stage-III card's Muster doubled; strong 8 with the d2 line |
| station feed | `station_cubes` (1) a round | the station's old compute output, one for one |
| project lines | 2 a country (4 at 2 countries) | 12 at `adversary_research` 3: each project's cost raised to hold its compute ratio (9 → 14, 8 → 13, 19 → 24) |
| threat lines | 2, 2, 4 | each in place of a harm of about the same size: Wiper Malware's third discard, Data Breach's 5 compute, Zero Day's 10 compute (4, not 3: 3 left it break-even with its counter, the review's catch) |

## Reads

Specs in `experiments/farm/`: `s259` (on, off, `research_at` 4,4,4,3,3), `s259b` (5,4,4,3,3), `s259w` (the eval term
on/off), all on the dev seeds 36800-36831, bp, judge and greedy; `f259-read` (fresh, 49452-49499, three pairs,
d0-d4, judge and greedy, 1,440 games) with r119 (main 28c3c8a, built in `out/r119`) natively on the same seeds;
`f259-oracle` (full oracle, d2-d3, 49452-49463, 72 games); `f259-off` (the rule off on the fresh seeds, d2-d3: a
diagnostic on spent seeds). Kept: `out/keep/259/`. Seeds 49452-49499 taken (next free 49500).

The reads ran on r120. r121 (the full lane's catch after the freeze: a removal Action is not offered on an empty
track) replayed the fresh read's seeds: `farm/compare.py` finds 310 of 1,440 games differ only in counters and options
offered, with no outcome changed (paired gap +0.0 ± 0.0). The numbers below stand for r121.

**Paired, r120 against r119 (fresh seeds, pooled over the three pairs, 144 games a cell):**

| Level | Judge r120 | r119 | Δ | Greedy r120 | r119 | Δ | Fires a game (judge / greedy) |
|---|---|---|---|---|---|---|---|
| d0 | 88.9 | 95.1 | -6.2 ± 3.2 | 88.2 | 86.8 | +1.4 ± 3.9 | 0.20 / 0.17 |
| d1 | 91.0 | 91.7 | -0.7 ± 3.2 | 88.9 | 85.4 | +3.5 ± 3.9 | 0.65 / 0.62 |
| d2 | 77.8 | 90.3 | **-12.5 ± 4.4** | 84.7 | 84.0 | +0.7 ± 4.1 | 1.20 / 1.01 |
| d3 | 51.4 | 57.6 | -6.2 ± 4.9 | 50.7 | 64.6 | **-13.9 ± 5.5** | 1.92 / 1.73 |
| d4 | 21.5 | 20.8 | +0.7 ± 4.5 | 19.4 | 22.9 | -3.5 ± 4.7 | 2.11 / 1.92 |
| all | 66.1 | 71.1 | -5.0 ± 1.8 | 66.4 | 68.8 | -2.4 ± 2.0 | 1.22 / 1.09 |

Full oracle on r120, d2-d3: 72/72, 10.1 rounds, 1.8 fires a game (r119's 258 read: 72/72, 10.2).

**Attribution of the two >10-point drops** (the Militarist's Weapons Program, the only research effect), on the same
seeds with `research_at` 0 (stations make compute again, research lines idle; the r120 threat and project text and
the two removal cards stay):

| Cell | r120 - off (the rule firing) | off - r119 (the content around it) |
|---|---|---|
| judge d2 | -7.6 ± 3.0 | -4.9 ± 3.9 |
| greedy d3 | -6.2 ± 3.2 | -7.6 ± 5.6 |

The rule's firing explains about 60% of the judge's d2 drop and about 45% of greedy's d3 drop. The rest is the content
around it, within noise in each cell: two removal cards that are dead with the rule off, and threats and projects
that lost their compute or discard when their research line went in.

**Where the cubes come from** (a game, pooled, d0-d4): stations 5.5 / 10.6 / 13.2 / 14.0 / 14.8; threats 1.2 / 1.2 /
1.5 / 1.9 / 1.9; projects 0.5 / 1.1 / 3.0 / 4.7 / 5.1. Players removed 2.4 / 2.7 / 2.7 / 2.3 / 1.9. Weapons Program
troops a game: 0.5 / 1.8 / 5.2 / 12.3 / 14.6 (pooled). Adversary compute made fell 6% (judge, all levels: 241 against
256; r119's stations made 12 a game), mostly at d3-d4 (the stations', Data Breach's and Zero Day's compute).

**Removal cards** (fresh read, a game, pooled): Counterintelligence Sweep kept 0.20-0.25, played 0.48-0.76;
Arms Inspectors kept 0.22-0.25, played 0.34-0.61, **its Action used 0.36-0.60 a game**, and (at r120, before r121's feasibility fix) offered on
an empty track 0.31-0.52 a game; r121 offers it only on a non-empty track, with the same outcomes. play_read: neither kept and never played, for either agent. Arms Inspectors is played and
used, so not dead, but it uses its Action about once a copy: at measured play it pays 12 for about 4-8. Proposal (not
built): a when-played removal (3 cubes) plus the Action, or an Action that draws when the track is empty.

**The agents.** The judge's and greedy's eval reads the track as a share of its threshold (`w.research` 30, a
project's weight) and counts each fire in full (the review's fix). Dev seeds, on against off: the judge removes 2.0
cubes a game against 1.1, with no win cost (mean of the five levels 69.4 against 66.3, 160 games, noisy); greedy 64.4 against 63.7.

## Value check (what a fire costs the players)

At the anchors (remove 1 troop 5) and at `values.json` (troops 7), 2 countries:

| Level | Troops a fire | At the anchors | At the table | Threshold | A cube (anchors / table) |
|---|---|---|---|---|---|
| d0-d1 | 4 | 20 | 28 | 10 | 2.0 / 2.8 |
| d2 | 6 (difficulty line +1) | 30 | 42 | 10 | 3.0 / 4.2 |
| d3-d4 | 8 (strong 3, +1) | 40 | 56 | 8 | 5.0 / 7.0 |

`values.json`: `remove_research` 3 and `adversary_research` -3 a cube (one number for every level: the d0-d2 middle).

## Give / get (before the first screen)

Anchors: a research cube removed 2 (a d0-d1 fire's 4 troops at 5, over 10 cubes; 3 at d2, 5 at d3-d4); the table's
`remove_research` 3. Pay = credits + 4 × (keep + play insight).

- **Counterintelligence Sweep** (tier-1 event, Cyber, 5 cr, 0 insight: remove 4 adversary research cubes, draw 1):
  pay 5 + 4 = 9; get 4 × 2 + 5 = 13 at the anchors (1.44×), 4 × 3 + 5 = 17 at the table (1.9×); at d3-d4 cubes 25.
- **Arms Inspectors** (tier-1 building, Security, 8 cr, 0 insight; Action: remove 2 adversary research cubes):
  pay 8 + 4 = 12; get 4 uses × 4 = 16 at the anchors (1.33×), 4 × 6 = 24 at the table (2.0×). Capped by once a round
  and by the track: it does nothing on an empty track.
- Threats (adversary side, uncountered at `values.json`): Wiper Malware 12 → 14 (1.56× its cheapest counter), Data
  Breach 17.5 → 18 (1.29×), Zero Day 17 → 18 (1.2×; 3 cubes left it at 1.0×).
- Projects (adversary side), compute at 1.1: Backdoor Research 30.8 → 36 (1.17), Lab Seizure 28.6 → 32 (1.12),
  Station Coup 52.8 → 58.8 (1.11).

**What it costs the players a game** (fires a game on the fresh read × a fire at the anchors; ×1.4 at the table):
judge d0-d4 4 / 13 / 36 / 77 / 84 credits; greedy 3 / 12 / 30 / 69 / 77.

## Open for Matt

- **"Difficulty lowers the threshold"** ships as 5 at d0-d2 and 4 at d3-d4 (a country). A drop at every level cost the
  judge about 5 points at d2-d3 on the screen (5,4,4,3,3), and a higher d0 would leave d0 almost never firing.
- **The off arm** restores station compute only; a clean "no rule" arm is r119 itself (the paired read).
- **Arms Inspectors** is used about once a copy (above): a stronger Action or a when-played removal, if wanted.

## For new players

The adversary keeps a public **research track**. Its corrupted research stations put a cube on it each round once
their country's megaproject is complete, and so do three projects and three threats you let through. When the track
reaches 5 cubes a country (4 at levels 3-4), the adversary spends them in its Income beat. The Militarist's
**Weapons Program** then musters 2 troops in each country, or 3 at levels 3-4; like every Muster, 1 more from
level 2 (so 2 / 2 / 3 / 4 / 4 a country at levels 0-4). Counterintelligence Sweep (remove 4,
draw a card) and Arms Inspectors (an Action: remove 2) take cubes off the track.
