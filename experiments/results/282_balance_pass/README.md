# 282: global balance pass after the overhaul (r134)

Matt (2026-11-18): bring the game back into shape after the table-time / card-economy overhaul (r127-r133), by content
and rules, not dials alone; bands soft. `RULES_REVISION` 133 → 134 (bumped from the constant). Base: main at 6bf6647
(r133 with 281's cleanup, play unchanged). Every number below is a **candidate**.

## Why d3-d4 collapsed (r133, 280's kept read and the dev screen)

The judge lost d3-d4 to the last node (node endings 87 / 121 of 144 on the fresh seeds' r133 side). Nodes fell to the
**march** (9 a game at d3), and the troops came mostly from the **research track** (12 research troops a game at d3):
since 275 every Infiltrate puts an agent on each named station and factory, a station or factory fell at 2, a corrupted
station makes research cubes for the adversary, and Weapons Program musters troops. The d2 → d3 step was also a cliff:
compute income 5 → 15 a country.

## What changed

| Area | r133 | r134 |
|---|---|---|
| Station and factory corrupt numbers (`corrupt_station_at`, `corrupt_factory_at`) | 2 | **3** (cities 5, 3 from d1; a node 2) |
| Stage I Infiltrates | Long Patrols: stations; Factory Drive: factories | **Long Patrols: cities; Factory Drive: coastal cities** |
| `research_at` (a country, d0-d4) | 5, 4, 4, 4, 4 | **5, 5, 5, 4, 4** (Matt's ruling of 2026-11-17, which config had drifted from since 267) |
| d4's Infiltrate 2 | on every site | **on a city; at most 1 on a station, factory or botnet hex** (new `infiltrate_facility_cap` 1) |
| Compute income d3 / d4 (a ladder dial) | 15 / 18 | **10 / 15** |
| `cards.discount_floor` | 1 | **0** (Matt: a card may be free) |
| Buy a city's energy / Gain an ambassador | 3 / 13 credits | **4 / 16** |
| Command-and-Control Grid | 13 cr; Action 1 energy: draw 2 | **4 cr; Action: draw 1** (Matt: 2 with Reuse) |
| Tidal Grid / Polar converters | 3 cubes → 5 energy, 2 chips → 3 energy; Polar 1 chip → card + 1 cr | **2 cubes → 4, 1 chip → 2; Polar card + 2 cr** |
| New Deal, Rebuilding Fund / Propaganda | 1 credit a site / lose 1 a city, at most 2-4 | **2 a site / lose 2 a city, at most 1-3** |
| Surge projects (compute a board) | 9-22 | **13-32** (priced as the captures they make) |
| Cards | 22 tier-2 cards at 2-3 credits | 13 repriced up, 8 made stronger ([givget.md](givget.md)) |

Card edits are two overlays, [prices.overlay](prices.overlay) and [effects.overlay](effects.overlay) (`apply_overlay.py`,
rerunnable from r133); projects and Unity / Discord cards are [apply_drafts.py](apply_drafts.py); every give / get line
at the unit anchors is in [givget.md](givget.md). Pricing notes: [pricing_notes.py](pricing_notes.py).

## Screens (dev seeds 36800-36831, the judge, three pairs, 96 games a cell)

Screen 1, one lever at a time on r133 (gap to base, paired): stations at 3 +12.5 ± 5.4 d2, +10.4 ± 4.3 d3; Arms Race off
+5.2 d3; `threat_agents` 0 +9.4 d2; research 5/5/5/4/4 +8.3 d2; stage I on cities +6.2 d3; none moved d4. Screen 2,
combined: stations and factories at 3 + stage I on cities + research thresholds (E): 91 / 82 / 69 / 30 / 12; E with d4's
Infiltrate at 1 everywhere 25 at d4; E with compute 12 / 15 37 at d3. Screen 3, the full package with prices (P): 96 /
83 / 73 / 41 / 18; P + the facility cap 19 at d4; P + compute 10 / 15 55 at d3; both (shipped) 96 / 83 / 73 / 55 / 34.
The cap and the dial only work together at d4 (+16.7 ± 4.4 together, +1 and +2 alone). Pre-freeze read (judge and greedy,
paired against r133 on the dev seeds): judge 99 / 83 / 82 / 53 / 32, greedy 97 / 83 / 74 / 46 / 17.

**values.json overrides** for the 17 cards the table reads under 0 (givget.md's FLAG column): on / off on the dev seeds
cost the judge about 2 points pooled (d0 −8.3 ± 2.8), greedy about 2 (d2 −7.3 ± 4.7). Reverted and flagged, not shipped.

## Fresh read (seeds 50076-50123, 1,440 games a side, the judge and greedy, three pairs, d0-d4)

r133 farmed at 6bf6647 from 280's spec; r134 at the freeze (d49c2e2). Full table: [read.md](read.md) ([read.py](read.py)).

| Agent | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| Judge win % r133 → r134 (gap ± SE) | 87.5 → **95.8** (+8.3 ± 3.3) | 61.8 → **84.0** (+22.2 ± 4.7) | 46.5 → **79.2** (+32.6 ± 4.6) | 20.1 → **52.1** (+31.9 ± 4.6) | 3.5 → **26.4** (+22.9 ± 3.9) |
| Band | 90+ met | 80-95 met | 75-90 met | 55-70 **unmet** (−2.9, 0.7 SE) | none |
| Greedy win % | 93.1 → 97.2 | 59.0 → 82.6 | 45.1 → 77.8 | 9.7 → 44.4 | 3.5 → 23.6 |
| Judge rounds | 8.57 → 7.81 | 9.70 → 9.50 | 9.23 → 9.42 | 7.91 → 9.19 | 6.26 → 8.53 |
| Cities corrupted by round 4 | 0.31 → 0.68 | 1.89 → 2.28 | 2.26 → 2.69 | 2.94 → 3.00 | 3.72 → 3.89 |
| Cities corrupted at the end | 0.12 → 0.03 | 1.05 → 0.30 | 1.79 → 0.55 | 3.24 → 1.45 | 4.43 → 3.12 |
| Node endings (of 144) | 3 → 0 | 38 → 4 | 50 → 9 | 87 → 19 | 121 → 61 |

**The adversary spreads more at every level**: cities corrupted by round 4 rise with level (0.68 → 3.89) and rise from
r133 at every level; the players purge more of them by the end. Rounds sit at 8.5-9.5 for d1-d4 (d4 6.3 → 8.5, the cause
the node endings); d0 runs 7.8 (warning: easy games end early).

## Turns, usage and resources (judge, fresh read)

- **Non-card turns a seat-round** (standard actions, board powers past Growth, building Actions) against card plays:
  d2 3.91 → 3.95 against 4.44 → 4.90 plays; d4 2.75 → 3.41 against 3.57 → 4.27. Plays rose more than turns at every
  level. Buy energy (a game, d0-d2: 8.3 / 6.1 / 5.1 → 5.4 / 5.2 / 4.3) and Gain an ambassador (2.0-2.2 → 1.2-1.6)
  fell after their price rises.
- **Warnings gone:** Buy energy's job-parity, Patrol I's band (3.56 → in), Gain an ambassador (0.97 → in), Tidal
  Grid's converter (0.24 → in), the d3 / d4 rounds, Peacekeeping Mission's kept-not-played.
- **Warnings left:** Remove a troop 0.89 (band 0.75; 1.23 at r133); Pay a threat's counter 5.36 (band 5.0); antimatter
  49% spent of gained, the victory-skip 70% (67% at r133; min 75); Make antimatter unused (left, amendment 4);
  kept-not-played Open Research Campus, Orbital Mass Driver, Forensic Audit, Moonshot Grant; table-memory +20% / +30%
  at d3 / d4 (longer games with more engines built); dials-hide-design (the three moved dials sit beside the content).

| Resource | Gained a seat-round | Spent | Spent of gained | Top uses | Binds? |
|---|---|---|---|---|---|
| Credits | 19.6 | 21.0 | 107% | cards 72%, standard actions 20% | yes (the start covers the rest) |
| Insight | 5.8 | 4.3 | 75% | keeps 69%, plays 14% | half: the rest lapses at round start |
| Energy | 4.8 | 3.9 | 82% | powers 59%, building Actions 18% | mostly |
| Chips | 0.7 | 0.5 | 71% | converters 72%, powers 13% | loosely |
| Antimatter | 0.3 | 0.2 | 48% | cards 67%, building Actions 32% | no: under its 50% band |
| Research cubes | 0.7 | 0.6 | 74% | converters 44%, building Actions 27% | loosely |
| Ambassadors | 2.2 | 1.9 | 85% | standard actions 97% (Liberate, Remove agents, moves) | yes |

## Engines and agent values

- Command-and-Control Grid draws 1 (2 with Simulation Cluster's Reuse), so Reuse stays one rule; at 4 credits it reads
  0.58-0.77 alone and 1.15-1.54 with Reuse (no price makes a lone draw-1 engine fair at the anchors).
- `drawn_card`: the prompt's 4 is r127's scale; `values.json` reads 2.667 since r128 (under the draw anchor 3.33). Left.
- The jam (274): Force Protection Order sits about at break-even (3.0-3.5 against 3.7); not changed.

## Checks

- **design_check** on the fresh read with `--old` its r133 side: the report's warnings line. `--quiet`: dials-hide-design,
  friction-stale (as on main).
- `npm run cards:fit` 0 under 7.5 pt; loop smoke (16 seeds) no timeouts, no card at the flag; `play_read.py`: no card kept
  and never played on the pre-freeze read; fast lane and clippy clean at the freeze.
- Speed (`per_decision.py`, d2, 6 games, best of 2, r133 built from 6bf6647; a shared box, ratios only): the judge
  0.92× a decision (1.8 → 1.6 ms), greedy 1.09× (1.4 → 1.6 ms); games run longer (1.04-1.10× rounds). No agent code
  changed. `speed.py check` gives no verdict across the revision; the baseline is not re-based.
- Diff review: 3 reviewers by dimension (engine, content, docs), then skeptics: 13 distinct findings, all fixed (stale
  doc comments, an untested cap, pricing notes, the e2e Tidal numbers, stale docs and value notes, two tests whose
  floor had moved).

## New-player summary (r134)

- **Research stations and factories fall at 3 agents**, not 2; cities still 5 (3 from level 1).
- **The first rounds' Infiltrates go to cities**: Long Patrols and Factory Drive now name cities and coastal cities.
- **At level 4** an Infiltrate puts 2 agents on each city it names but only 1 on a station, factory or botnet hex.
- **The research track fires at 5 a country through level 2** (4 from level 3); the adversary's compute income is 10
  at level 3 and 15 at level 4.
- **Prices:** Buy energy 4 credits, Gain an ambassador 16; discounts can make a card free; the cheapest tier-2 cards
  cost what they give; Command-and-Control Grid draws 1 a round (2 with Simulation Cluster).
- **Rules that resisted:** the d3 band (52.1 against 55, 0.7 SE under); Remove a troop over its band; antimatter
  under-spent; sell stays 1 credit by ruling.

## What resisted (for Matt)

1. **d3 judge 52.1** (band 55-70): 0.7 SE under; reached only with the compute dial (d3 15 → 10). Not chased further.
2. **Sell a card stays 1 credit** (standing ruling); the other 1-credit prints folded.
3. **Remove a troop** 0.89 a seat-game (band 0.75): it is the d3-d4 troop answer, so a price rise works against the d3 band.
4. **Antimatter** still skipped in 30% of d3-d4 wins and 49% spent: no antimatter engine was changed here.
5. **values.json overrides** cost the agents about 2 points: the table's rates (agent removed 1.33 against 3.33, card
   2.67 against 3.33, insight 1.67 against 2.67) want an agents item.
6. **Kept-not-played:** Orbital Mass Driver (now 15; its needs, not its price), Open Research Campus, Forensic Audit
   (its 4 cubes), Moonshot Grant.

## Files

- [keys.py](keys.py) counter means by level; [turns.py](turns.py) non-card turns; [screen.py](screen.py) lever screens;
  [make_spec.py](make_spec.py) lever × pair specs; [read.py](read.py) the fresh read; [add_overrides.py](add_overrides.py)
  the reverted overrides.
- Specs: `experiments/archive/farm/s282-diff.json`, `s282-diff2.json`, `s282-p.json`, `s282-pre.json`, `f282-read.json`.
  Outputs: `out/keep/282/` (kept by conductor).
