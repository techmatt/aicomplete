# 276: the board rework (r131)

Matt's rulings (2026-11-18, table time): node placement is each board's **Growth** (pick one printed option a round,
Spirit Island's "pick one"); powers are **tiered** and stay sacrifice-paid where a tag pays; a few may be
**threshold** powers; **Mint goes**. Message 12618: Fortify (chips) and Patrol (energy) stay resource-paid.
`RULES_REVISION` 130 → 131 (bumped from the constant). Base: r130 (275). Every shape below the rulings is a
**candidate**. The boards as built, each old job's successor and every give / get: [design.md](design.md).

## The boards (done-when 1)

| Board | Before (r130): powers, decisions a round | After (r131): Growth + powers | Decisions a round | Power uses a round, judge (r130 → r131) |
|---|---|---|---|---|
| Builder | 10 (Mint, Expand, Buy In, Annex, Scrap, Overcharge, Rush Build, Procure, Fortify, Teardown) | Growth (Sprawl / Buy In / Annex); Scrap I-II, Overcharge I-II, Fortify I-II | **4** | 2.48 → 1.31 |
| Protector | 8 (Bulwark, Expand, Accord, Outpost, Debrief, Patrol, Interdict, Reclaim) | Growth (Outpost / Accord / Levy); Patrol I-II, Interdict I-II | **3** | 1.91 → 1.53 |
| Researcher | 9 (Expand, Peer Review, Field Survey, Grant, Field Station, Audit, Disrupt, Meltdown, Synthesis) | Growth (Peer Review / Field Survey / Field Station); Lab I-II (threshold), Hack I-II | **3** | 2.47 → 1.56 |

Targets a round: the Builder's Fortify, the Protector's Patrol and Interdict, the Researcher's Field Station (a hex
each); the node placements' hexes as before. Uses: the judge on the fresh read, levels pooled (by level:
[read.md](read.md)). Each job's successor, or why it goes, is in [design.md](design.md); in short: Expand, Buy In,
Accord, Outpost, Peer Review and Field Survey fold into Growth; Rush Build into Scrap II; Teardown's antimatter into
Overcharge II; Bulwark into Patrol II; Debrief into Interdict II; Synthesis into Lab I, Grant into Lab II; Disrupt
into Hack I, Audit into Hack II; Field Station into Growth. **Gone with a reason:** Mint (the ruling), Procure (0.04
a game, the Builder's draw is its cards), Reclaim (0.02 a game; Liberate turns a site neutral), Meltdown (0.2-0.5 a
game; Open Archive's antimatter and the plants stay), the fresh-spot riders (a check on every placement).

## Table time (done-when 2)

[trace.md](trace.md): seeds 268000-268013, the judge, Builder and Protector, r130 against r131, 268's rates.

| d2, a seat-round | Decisions taken | Board uses weighed (offered) | Powers used | Components | Checks | Minutes |
|---|---|---|---|---|---|---|
| Builder | 30.4 → 30.6 | 4.8 → 2.1 | 2.4 → 1.4 | 33.2 → 34.4 | 0.4 → 0.1 | 10.1 → 9.7 (−0.4) |
| Protector | 27.0 → 28.7 | 3.5 → 2.0 | 1.8 → 1.6 | 28.3 → 30.9 | 0.7 → 0.6 | 8.8 → 9.1 (+0.3) |

**The menu halves; the minutes barely move: −0.1 minutes a round at d2 (275's 52.7 → 52.6), −1.4 at d4.** "Weighed"
is a stats-only engine count (`ai.offered.<board>`, patched into r130's tree too): the board uses a seat could make
as its action phase began, a group counted once, each a 10 s check at 268's rate. Printed, 10 / 8 / 9 powers become
4 / 3 / 3 groups. But the judge took only about 2 board powers a round before (1.4 after), so 268's decisions taken
don't fall, and the two-node options and Accord's ambassador add components (+1.2, +2.6). The board's paid yes/no
list was never where the d2 round's time went: by these rates it was about 1.4 minutes of 52.7.

## Paired read (done-when 3)

f276-read, fresh seeds 49932-49979 (taken), 1,440 games a side, the judge and greedy, three pairs, d0-d4. r131
farmed at f1538d1 (native and wasm), r130 farmed at 8d5893b (275's own f275-read spec, the same grid). Gaps paired
by seed, ± SE.

| Level | Judge win % r130 → r131 (gap) | Greedy | Judge rounds | Judge power uses a seat-round (Builder / Protector / Researcher) |
|---|---|---|---|---|
| d0 | 93.8 → 84.0 (**−9.7 ± 3.2**) | 86.1 → 88.9 (+2.8 ± 3.5) | 8.22 → 8.83 | 2.39 / 2.09 / 2.54 → 1.22 / 1.47 / 1.55 |
| d1 | 62.5 → 68.8 (+6.2 ± 4.4) | 59.0 → 55.6 (−3.5 ± 4.3) | 9.79 → 9.17 | 2.43 / 2.00 / 2.45 → 1.37 / 1.60 / 1.60 |
| d2 | 45.8 → 50.7 (+4.9 ± 4.7) | 40.3 → 37.5 (−2.8 ± 4.4) | 9.40 → 9.17 | 2.49 / 1.91 / 2.46 → 1.32 / 1.57 / 1.64 |
| d3 | 13.2 → 13.9 (+0.7 ± 3.3) | 10.4 → 7.6 (−2.8 ± 2.9) | 7.67 → 7.45 | 2.55 / 1.85 / 2.47 → 1.32 / 1.52 / 1.56 |
| d4 | 6.9 → 4.9 (−2.1 ± 2.3) | 3.5 → 2.8 (−0.7 ± 1.8) | 6.05 → 6.11 | 2.51 / 1.71 / 2.42 → 1.31 / 1.48 / 1.46 |

The win rate holds: only the judge's d0 moves past 2 SE (−9.7 ± 3.2), where the r130 judge won 94%; d1-d2 lean up
(+5-6, about 1.2 SE), greedy is within ±3.5 everywhere. The judge's d0 drop is one cell of ten and the dev screen
(pre-fix build) read d0 +3.1 ± 4.6: flagged, not chased (bands are reported, not chased). By pair (`ladder.py
--pairs`) it sits in the Builder's pairs (judge d0 bp 97.9 → 83.3, br 87.5 → 75.0; pr 95.8 → 93.8), while at d1-d2
the judge's Builder-Protector pair rises (49.0 → 65.6; d1-d2 pooled +5.6 ± 3.2). br stays the weakest pair.

Power use by level, each board and power: [read.md](read.md). Placements a seat-game are flat (10.59 → 10.30), as are
corruptions, nodes lost and rounds.

**Usage bands** ([usage.md](usage.md), design_check's bands, levels and agents pooled): every new power and Growth
option is inside its band. Growth is taken in 73-76% of seat-rounds (Builder 0.26 Buy In + 0.40 Sprawl + 0.07 Annex;
Protector 0.44 Accord + 0.27 Outpost + 0.05 Levy; Researcher 0.34 Field Survey + 0.31 Peer Review + 0.08 Field
Station); Lab I 0.57 a seat-round (band 0.40-0.90); the tier IIs 0.10-0.84 a seat-game; Patrol I 3.37 (band to 3.50).
275's two band misses are gone with their powers (Procure 0.17, Bulwark 0.17); Scrap I sits at 1.70 a seat-game
(Scrap was 2.02, over).

## Resource use (the judge, levels pooled, a seat-round, r130 → r131)

| Resource | Gained | Spent | Spent of gained | Top uses (r131) |
|---|---|---|---|---|
| Credits | 18.01 → 17.89 | 19.30 → 19.13 | 107% → 107% (binds; the start covers the rest) | cards 71%, standard actions 23% |
| Insight | 6.05 → 6.11 | 4.92 → 4.90 | 81% → 80% | keeps 62%, plays 21%, powers 10% |
| Energy | 4.66 → 4.40 | 3.97 → 3.69 | 85% → 84% | powers 60%, building Actions 17%, track steps 11% |
| Chips | 0.85 → 0.67 | 0.65 → 0.48 | 76% → 72% | converters 66%, powers 19% |
| Antimatter | 0.32 → 0.30 | 0.17 → 0.16 | 52% → 52% | cards 63%, building Actions 35% |
| Research cubes | 0.62 → 0.63 | 0.47 → 0.48 | 76% → 75% | converters 40%, building Actions 28%, standard actions 19% |
| Ambassadors | 1.89 → 1.96 | 1.68 → 1.72 | 89% → 88% (binds) | standard actions 95% |

**Chips** lose Mint: 0.85 → 0.67 gained, still mostly spent on converters (66%), the rest on Fortify. **Energy**
spends 60% on powers (Sprawl, Outpost, Accord, Patrol). **Insight** pays keeps (62%) and plays (21%); powers take 10%
(the Researcher's Lab and Growth). Credits and ambassadors bind (107%, 88%); the rest as at r130.

## Candidates I set (adjacent numbers and small rules)

- **Growth prices:** Sprawl 1 energy, Outpost 2 energy, Peer Review 2 insight (two nodes each); Accord 1 energy;
  Field Station 1 insight; Annex and Levy 3 credits. On the dev screen a free Accord was always taken over the third
  option (Stockpile, a node and 2 energy: 0.00 a game; Levy, a node and 3 credits: 0.00): Accord at 1 energy lets
  Levy in (0.05 a seat-round). Stockpile was named so because "Muster" is the adversary's troop line.
- **A two-node option needs both steps** (diff review): it was offered, and paid, when only one node could go down.
- **Interdict II removes both kinds** (two `damage` ops; one asked "troops or agents").
- **A hardening after a target narrows the target** (Patrol II): `effects::aim` and `feasible` count a facility it
  can protect, as they count a push; before, Patrol II was offered on hexes where nothing could act.
- **Lab II needs an open megaproject** (Grant's job: its 2 credits buy the cube), though its draw could act.
- **Agents:** strategy scoring and the keep plan count a group once (its best option) and a threshold only when met;
  a rollout gives a two-node option its second track and hex; `never_second_node` is inert (one placement route).

## Checks

- **Diff review** (Ultracode: three reviewers by dimension, each finding sent to a skeptic): 8 found, 7 fixed, 1
  rejected (tier II offered when only tier I's part can act: the cards' r48 rule, Matt's call; see the report).
- **design_check** on the fresh read with `--old` r130: the report's warnings line. Against r130's read on the same
  seeds: none new from the boards (kept-not-played forensic_audit 0.02 replaces interconnector_treaty 0.02); gone:
  Procure's and Bulwark's usage misses and table-memory d0.
- **Same results:** the fresh read's r131 side replayed on the final build (after the stats-only `ai.offered` count,
  96b8173): all 1,440 games identical but that counter.
- **Smokes:** loop smoke (16 seeds) no timeouts, no card at the flag; the slow lane's Scrap II net widened to seeds
  2-160 (2-48 offered it in no game).
- **Speed** (`per_decision.py`, d2, 6 games, best of 2, a shared box: ratios only): the judge 1.22× a decision (1.5 →
  1.8 ms), greedy 1.14×; `speed.py check` gives no verdict across the revision (engine 1.35×, planner 1.34× a game).
  Kept: the judge's rollouts read a two-node Growth option's second track and hex (diff review: short of them it
  read 2 nodes as 1), and the strategy and keep plan read groups; `speed.py check --update` re-based.

## New-player summary (r131)

- **Growth: pick one a round.** Your board prints three Growth options; each round you may take one. Each places
  nodes (every placement still pays its track space's energy) with a small extra: a chip, an ambassador, a card,
  credits, a research station, or a second node for your board's currency.
- **Powers have tiers.** Each power is used once a round at one tier: tier I sacrifices one card with the tag, tier
  II two, for a stronger effect (Scrap: a chip, or a chip and a building played free).
- **The Researcher's Lab counts Science, it doesn't spend it:** with 2 Science on your cards, 1 insight draws a card;
  with 4, 2 credits more add a cube to a megaproject.
- **Fewer, bigger choices:** 3-4 board decisions a round instead of 8-10. Gone: Mint (credits into chips), the
  burns, Procure, Reclaim, Synthesis as a free-standing power.
- **Rules that resisted:** table time (what a player weighs halves, the minutes don't: −0.1 a d2 round);
  the bands at d1-d3 (not chased); the Protector's third Growth option needed Accord priced before it was ever taken.

## Files

- [design.md](design.md): the boards before and after, every job's successor, give / get.
- [trace_seats.py](trace_seats.py), [trace.md](trace.md): the traced games' player side.
- [read.py](read.py), [read.md](read.md): power use by level and resource use; [usage.md](usage.md): usage bands.
- Specs: `experiments/farm/f276-read.json` (new side; the old side ran 275's `f275-read.json` at 8d5893b).
  Read outputs: `out/keep/276/` (kept by conductor).
