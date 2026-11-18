# 274: table time, the free and small maintenance cuts (r129)

Matt's ruling (2026-11-18): about 2 hours for a 2-player d2 game; start with free and small maintenance cuts, hard
(strategy-changing) cuts discussed with him first. `RULES_REVISION` 128 → 129 (bumped from the constant). Base: r128
(273). Tables: [read.md](read.md).

## The four cuts

| # | Cut | Made? | Why |
|---|---|---|---|
| 1 | Troop stacks march as one | **Made** | The troops that start the march on one hex step together on one tied-step choice; each strike stays one troop's. At 1 step it plays the same as before except that a stack can no longer split across tied hexes. With Forced March's extra steps the stack takes each step together (before, each troop walked all its steps in turn). Troops already on a hex the stack enters march later on their own (amendment). |
| 2 | Projects count the hex only | **Made** | Asset Seizure, Station Coup and Sabotage Teams pick "the X with the most agents on its hex", as Night Raid does. Clearing agents next to a site no longer protects it. That is a small targeting change, not a new decision. |
| 3a | Skip a country with no place for the tier's deck | **Made, does nothing** | The literal test ("no uncorrupted site of any type the tier's deck prints") never fires at 2-4 countries. Both tiers print military bases, every country has one per front, and a base is never the adversary's. The tag-limited skip never fires either (smoke: 0 of 5,288 pulls). |
| 3b | Jams floor at 1 | **Reverted (switch off)** | In code as `adversary.jam_floor` (0 / 1), baseline 0. On the 12 jam cards' give / get it leaves Force Protection Order under break-even. FPO was already 0.2 under before the floor (Matt's question 1, defaulted to the strict reading). Hard-cut candidate. |
| 3c | Keep rejecting on tag mismatch | **Kept** | Turnover barely drops: 2.5 cards turned and rejected a round at d2 (0.9 per threat dealt) and 3.5 at d4, unchanged from r128. Hard-cut candidate below. |
| 4 | Tracks and used markers into the aid or the C list | **Made** | Rulebook §2's pieces table gains income tracks, tag dials, used markers (and a bought chip on a city), tip tracks, project compute tracks and a waits token; §7 and §9 step 5 point to them. The player aid gets *Tracks and markers*, and *Once a round* now says "put a used marker on its used space". No engine change; nothing a good player does changes. |

## Minutes a round (268's rates)

Seed 268003 at d2, the judge, traced on r128 and r129 (`minutes.py`). The two play the same game: the same troop
moves every round. The counts are what each cut removes. March and compute come from the trace; projects come from
the smoke's candidates read a round (0.16 at d2) × 6 hexes; income from 268's 7-10 printed lines a seat re-added (8.5
a seat as modifiers).

| Round | March: checks / choices cut | Compute: components cut | Projects: checks cut | Income: modifiers cut | Minutes cut | Threat cards still turned and rejected |
|---|---|---|---|---|---|---|
| 1 | 0 / 0 | 11 | 1.0 | 17 | 2.1 | 2 |
| 2 | 0 / 0 | 24 | 1.0 | 17 | 2.8 | 0 |
| 3 | 0 / 0 | 19 | 1.0 | 17 | 2.5 | 8 |
| 4 | 0 / 0 | 23 | 1.0 | 17 | 2.7 | 5 |
| 5 | 0 / 0 | 15 | 1.0 | 17 | 2.3 | 3 |
| 6 | 0 / 0 | 23 | 1.0 | 17 | 2.7 | 0 |
| 7 | 0 / 0 | 21 | 1.0 | 17 | 2.6 | 1 |
| 8 | 0 / 0 | 22 | 1.0 | 17 | 2.7 | 0 |
| 9 | 0 / 0 | 25 | 1.0 | 17 | 2.8 | 1 |
| 10 | 7 / 0 | 21 | 1.0 | 17 | 3.8 | 1 |
| **mean** | 0.7 / 0.0 | 20.4 | 1.0 | 17 | **2.7** | 2.1 |

**By cut, per round (14 traced games a level):**
- **d2:** the march saves about 0.1 minutes (1.1 troop moves a round, 0.6 stack steps), projects about 0.2, the
  skip 0, the jam floor 0 (off; on, it removes nothing and adds payments), the compute track about 1.1 (20-25
  cubes become one marker move per project and beat) and income tracks about 1.4. Total **about 2.7 minutes a round**,
  about 29 minutes over a 10.6-round game.
- **d4:** the march saves about 0.4, projects 0.5, compute about 2.3 and income 1.4. Total **about 4.6 minutes a
  round**.
- Tip tracks save a 20 s lookup per tip (about 0.1 minutes a round). Used markers and tag dials turn memory into
  pieces; their minutes are 0 by these rates.

**Against the target:** 268's hand count put a d2 game at about 490 minutes at r125. These cuts take about 6% off
that. Most of the saving is components (the compute and income tracks), not the rule cuts. Matt's 2 hours needs the
hard cuts: the Builder's late action phase, the per-site checks (natural corruption and city growth, held for the
adversary redesign) and threat turnover. The engine's own table proxies (`design_check` `table_cost`) don't move
(components 45.2, checks 49.1 a round at d2 on the read, ±0% on r128): they count a troop's move and a check per
troop step, not choices or the ring count.

## Jam give / get

`jam_give_get.py 0.4 0.6` (tiers weighted by the smoke's threats dealt; [jam_give_get.md](jam_give_get.md)). Under the
floor a jam of 3+ keeps about 93% of its points (the small energy and insight items are the ones it can't reach).
Each card's get drops 0.4-0.6 credits. 11 of 12 still show a gain (+1.0 to +3.9). **Force Protection Order** (2
credits + 1 insight, jam 6 Security) goes from 3.5 against 3.7 to 3.0 against 3.7. It was under break-even before the
floor (a ledger flag for the balance pass). By the prompt's rule the floor is reverted: its switch stays, off.

## Hard-cut candidates (for Matt)

1. **The jam floor** (`adversary.jam_floor=1`): a jam never counters a threat alone. Needs FPO repriced or reworded
   first. Screened on f274-read's seeds (`s274-jam`, 1,440 games, paired with r129's floor-off side): the judge
   −1.8 ± 1.5 pooled (d1 −6.9 ± 4.6, d2 −4.2 ± 3.8, d3 +4.2 ± 3.1, d4 −2.8 ± 1.7), greedy −3.2 ± 1.5; rounds
   unchanged. A small cost to the team, as a cut that only removes a free counter should be.
2. **Deal the top threat that has a site, ignoring tags:** removes about 2.5 turned and rejected cards a round at d2
   (13 s each, about 0.5 minutes) and 3.5 at d4. It changes which threats a round brings, so it is a strategy change.
3. **The march at more than one step:** if the stack-steps-together order matters at d4, a stack could instead
   strike on the hex where it ends. Not proposed; recorded because the cut changes Forced March slightly.

## Read (fresh, paired)

f274-read: seeds 49836-49883 (taken), 1,440 games a side, the judge and greedy, three pairs, d0-d4, r128 farmed at its own commit.

| Level | Judge win % r128 → r129 (gap ± SE) | Greedy | Judge rounds |
|---|---|---|---|
| d0 | 94 → 94 (-0.7 ± 0.7) | 91 → 91 | 8.44 → 8.47 |
| d1 | 76 → 76 (+0.0 ± 1.4) | 74 → 74 | 10.77 → 10.83 |
| d2 | 62 → 67 (+4.9 ± 1.8) | 56 → 56 | 10.70 → 10.58 |
| d3 | 24 → 18 (-6.2 ± 2.3) | 19 → 19 | 9.21 → 9.42 |
| d4 | 5 → 5 (+0.0 ± 1.4) | 4 → 5 | 8.01 → 8.06 |
| pooled | 52 → 52 (-0.4 ± 0.7) | 49 → 49 (+0.0 ± 0.8) | 9.43 → 9.47 |

Greedy doesn't move at any level. The judge's d2 and d3 moves are in opposite directions on games that diverge once
a project or march differs; pooled, the gap is nil. **Bands** (Matt's soft bands): d0 is met (94 ≥ 90); d1 (76
against 80-95), d2 (67 against 75-90) and d3 (18 against 55-70) miss, as on r128 (271's draw cut and 273's rescale).
Per the prompt, the misses are reported, not chased. Rounds d1-d2 stay over 10.

## Resource use (the judge, r129)

- **Credits bind:** 20.1 spent against 19.1 gained a seat-round (the starting stock covers the rest). Spent 69% on
  cards and 22% on standard actions; 7.5 left at the end.
- **Insight half binds:** 82% of gains spent (keeps 57%, plays 21%). What's left is lost at round start.
- **Energy does not bind:** 81% spent, mostly on powers (61%); 10.9 left at the end.
- **Chips** (79%) and **research cubes** (78%) go mostly to converters.
- **Antimatter is spent at 47%:** 1.8 left at the end.
- **Ambassadors bind:** 90% spent, 96% on standard actions (Liberate, Remove agents).

## New-player summary (r129)

- **The march:** troops on the same hex march together as one stack, one step. If a target is on or next to it, one
  troop strikes each target (each striking troop is used up). The rest move one hex together toward the nearest
  target; when two hexes are equally good, you choose once for the whole stack.
- **Projects:** a project that takes a factory or station, or strips hardening, picks the one with the most agents
  on its own hex. Agents next to it don't count.
- **Threats:** unchanged at 2-4 countries. A jam still takes its amount off each resource, down to 0.
- **Components:** keep your income on income tracks, your tags on tag dials, the once-a-round uses under used markers,
  the Unity and Discord tips on tip tracks, and each project's compute on its track.
- **Rules that resisted a cut:** the jam floor, which breaks a card's price, and the tag test in threat dealing,
  which would change which threats come.

## Checks

- **Tests and lint:** the fast lane and `cargo test-all` pass; clippy is clean.
- **Speed:** `per_decision.py`, r128 against r129. The judge and greedy each cost 1.01-1.03× a game and 1.02× a
  decision: same rules, same games.
- **design_check:** on the read with `--old` r128, the same warnings as r128 on the same seeds, except agency_directorate's kept-not-played
  (gone). That leaves job-parity (Buy energy), kept-not-played (Open Research Campus, Forensic Audit), usage 6,
  rounds d1-d2 and table-memory d1 4.5.
- **Cards:** no card or face changed (project texts only), so `cards:fit`, `play_read` and `loop_smoke` don't apply.
  The 16-seed smoke had no timeouts.

## Files

- [trace_cuts.py](trace_cuts.py): per-round march, threat and compute counts from a trace.
- [minutes.py](minutes.py): what each cut removes per round, and minutes at 268's rates.
- [counters.py](counters.py): a farmed file's 274 counters.
- [jam_give_get.py](jam_give_get.py): the jam floor's give / get.
- Specs: `experiments/archive/farm/f274-read.json`, `s274-smoke.json` and `s274-jam.json`.
- The read outputs: `out/keep/274/` (kept by conductor).
