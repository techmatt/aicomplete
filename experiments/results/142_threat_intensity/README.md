# 142: threat intensity and the credit economy (r59)

Rules revision 59, bumped from the constant (58 to 59; the prompt also said r59). Rules: Matt's design doc *Adversary
system* (*Claiming sites*, *Difficulty and threat intensity*, *Credits*, 2026-11-07), his message 5982 (the ladder:
d2's stage swap to d4) and 5992 (no node counters). Tables: [confirm_r59.md](confirm_r59.md) (the final build on the confirming
seeds, with the paired gap to r58), [confirm_r58.md](confirm_r58.md), [confirm_pre_credit_lever.md](confirm_pre_credit_lever.md)
(the build before W3 + G6 on the same seeds), [read_r59.md](read_r59.md) and [read_r58.md](read_r58.md) (the first
fresh seeds), [death_audit.md](death_audit.md). Scripts: [specs.py](specs.py) (screens and reads), [gen_levers.py](gen_levers.py)
and [levers.json](levers.json) (the levers as run: 23 generated from r59's pre-freeze content, the other 16 added by hand; a rerun writes
`levers.gen.json`),
[read142.py](read142.py), [freeze_142.py](freeze_142.py) with [overlay_142.txt](overlay_142.txt) (the frozen income
levers), [add_claim_cards.py](add_claim_cards.py), [death_runs.py](death_runs.py).

## Where things stood

The code was r58 (main 3888848). On this item's fresh seeds r58 reads **84.0 / 66.7 / 52.1 / 42.4** (d0-d3; 139's own
read 77.8 / 61.8 / 51.4 on its seeds), median 9 / 9 / 9 / 8; credit peaks a seat 81 / 76 / 71 / 65 (d2 by board:
Builder 71, Protector 62, Researcher 78, as 139 said); 22 of 25 tier-1 threats offered credits, as the prompt said;
credits paid 59% of all counters at d2 (standing counters and cards included), nodes 2%. 30 of 64 threats shared a
currency between a counter and their harm (21 by nodes: every uncountered threat destroys `threat_nodes`).

## The rules

- **Counters** (all 64 threats, `drafts/adversary/threats_t*.json`): a menu leads with a scarce currency for its tag,
  then a second one or a tag sacrifice, **credits last on every menu**. Printed is paid: 138's `counter_pct` 150 is
  folded into the numbers (`counter_pct` 100). Tier 1: 3 energy / 3 ambassadors / 8 insight / 2 research cubes / 4
  chips / 2 tags / **15 credits**; tier 2: 4 / 4 / 10 / 3 / 6 / 3 / **23**; tier 3: 5 / 5 / 12 / 4 / 8 / 4, two-resource
  options, **33**. First options across the decks: energy, ambassadors, insight and cubes about a quarter each.
- **No shared currency.** No harm takes credits (credit losses became chips, energy, buildings face down, discards);
  a harm taking energy, insight or ambassadors drops that counter; no menu offers nodes (Matt, 5992). `check.py`
  (`harm_currencies`) and the load check (`content::harm_currencies` in `check_threat`; the adversary module refuses a
  nodes counter at a tier with `threat_nodes` > 0) refuse one; an overlay test covers both.
- **Harms.** Tier 1: an adversary ambassador (or a standing troop for Military) on its location and 3-4 compute, plus
  its tag's harm (no permanent harm at tier 1). Tier 2-3: corrupt, seize or destroy sites, take ambassadors, pull Unity
  back 2-3, more pieces, 4-8 compute. Projects about 1.5x as severe. Every card's `value` line prices the harm at 2-5x
  the cheapest counter for one player near it.
- **The ladder** (intensity, not count): d1 Corrupt X one fewer (as before); **d2 counters cost 2 more of each item,
  6 more credits** (`counter_bonus_all` 2, the new `counter_bonus_credits` 6, from `counter_bonus_at` 2;
  `counter_bonus`, 0, now reaches every item of the first option, not its credits); **d3 each player near an uncountered threat destroys 2 more nodes**
  (`harm_bonus` 2 from `harm_bonus_at` 3) and counters 1 more an item, 3 more credits, again (`counter_more_all` 1,
  `counter_more_credits` 3 from `counter_more_at` 3, new), with d3's `build_bonus` raised 1 to 2 and its stage-II card
  removed; d4 the
  extra threat a round (`extra_threats_at` 4, 139's 0 reverted) and the stage-I-for-III swap (`stage_swap_at` 4,
  Matt's 5982: a stage-III card pulls twice). **Extra threats appear only at d4.**
- **Credits.** Boards print 15 / 13 / 11 (from 21 / 19 / 17) and 1 energy a round more (Builder and Researcher 1,
  Protector 2): 3 of the 6 credits moved to energy (Matt's message 6017 asked for a credit lever); the 16 flat credit
  programs 1 less (20% from 6 up) and the two flat income buildings 20% less (`overlay_142.txt`); credit counters
  +25%. Every board starts with 3 energy (from 1): the player-board levers that held d0 near 90 (below).
- **Claiming.** Claim stays 3 ambassadors; a player's station makes 2 cubes (`owned_station_cubes`, new) and factory 2
  chips (`factory_chips` 1 to 2): at `values.json` 3 ambassadors (12) pay back in 3 rounds (factory, 4 a round) or 2
  (station, 6). New op `claim_station` and two cards: **Power Purchase Agreement** (tier-1 event, 2 credits + 2
  energy: claim a neutral station in reach with no adversary ambassador on it) and **Microgrid Lab Compact** (tier-2
  building, 8 credits: Action, 2 energy: the same). Give / get: PPA anchors pay 15 (2 + keep 4 + 2 energy 9), get the
  ambassador route's 15 + 2 tags (1.07); table claim_station 20 against 13.5. MLC pay 20 (+9 a use), get 15 a claim
  (1.0 at 2 uses); table 20 a claim against 9. The ledger (`effect_ledger.py`) reads PPA fair at 6 credits against 2
  printed (cheaper than the ambassador route, as asked) and MLC fair 8.6 against 8.
- **Parked P1:** Infiltration Drive's bare `corrupt_city` is gone; `corrupt_city` needs `at` (the load check refuses
  it bare) and `adversary::most_infiltrated_city` is removed. **P2:** e2e's `board fills its pane` floor at 1920x1080
  is 0.40 (2560 keeps 0.43); not run here.

## Calibration (dev seeds 36800-36831, 96 games a level)

Order as asked: income and credit counters, then currencies and harms, then the ladder. 16-seed screens (s2-s4) only
scoped; every row below is 32 seeds. Cells name `gen_levers.py`'s levers.

| Cell | d0 | d1 | d2 | d3 | d2 credit peak |
|---|---|---|---|---|---|
| r59 as built (the prompt's rules, no trims) | 90.6 | 83.3 | 84.4 | 85.4 | 88 |
| I3+C25 (boards -3, credit counters +25%) | 81.2 | 70.8 | 59.4 | 52.1 | 76 |
| I3+P1+C25 (+ programs -20%) | 72.9 | 64.6 | 55.2 | 46.9 | 68 |
| I3+T1+P1+C25 (+ track credit spaces -1) | 66.7 | 39.6 | 35.4 | 31.2 | 58 |
| I3+P1+C25+A1 (tier-1 creates 1, not 2) | 75.0 | 60.4 | 59.4 | 56.2 | 68 |
| I3+P1+C25+A1+E2 (start energy 3) | 88.5 | 72.9 | 67.7 | 63.5 | 75 |
| **+D2+H2 (frozen; d2 +2 an item, d3 +2 nodes)** | **88.5** | **72.9** | **60.4** | **52.1** | 71 |
| frozen + H3 / + B2 (d3 build bonus 2) / + D3 (d2 +3 an item) | | | - / 60.4 / 47.9 | 54.2 / 42.7 / 45.8 | |
| frozen + B2 + F1 (d2's first option +1 more; read fresh on the first seeds) | (88.5) | (72.9) | 55.2 | 47.9 | 68 |
| that + W4 (boards -4 credits, +1 energy a round) | 81.2 | 72.9 | 49.0 | 43.8 | 64 |
| that + W4 + C20 (credit counters +20% more) | 83.3 | 63.5 | 49.0 | 38.5 | 64 |
| that + W3 (-3 credits, +1 energy) / + W3 + E5 (start energy 5) | 90.6 / 94.8 | 78.1 / 90.6 | | | |
| B2 + G6 (d2 credits +6 an item) + F0 (F1 off) | (87.5) | (75.0) | 55.2 | 38.5 | 71 |
| W4 + G6 + F0 | (81.2) | (72.9) | 52.1 | 41.7 | 62 |
| **final: W3 + G6 + F0** (its d2 not screened on the dev seeds) | 90.6 | 78.1 | | 44.8 | |
| final + B3 / + B4 (d3 build bonus 3 / 4) / + B3 + R2 (d3 removes 2 stage-II cards) | | | | 44.8 / 49.0 / 42.7 | |
| final + M13 (d3 counters +1 an item, +3 credits: adopted after the two independent reads) / + M26 | | | | 35.4 / 17.7 | |
| final + L5 (start credits -5) / + W3b (3 more credits into 1 more energy), after the reads; not adopted | 84.4 / 88.5 | 72.9 / 85.4 | 41.7 / 54.2 | | 62 / 63 (final 67) |
| frozen with C50 for C25 | 70.8 | 65.6 | 46.9 | 35.4 | 67 |
| frozen + L10 (start credits -10) | 78.1 | 52.1 | 54.2 | 47.9 | 65 |

The as-built rules alone read far too easy at every level (84-91): without extra threats below d4 the table sees 24
threats a game, not 32, and banks 88-113 credits. Win rate is very sensitive to credits: the full trim (T1) costs d1
30 points. **Credit peaks and win rate move together**: every cell whose peaks fall clearly (58-68 at d2) reads d0
67-79; the energy start that restores d0 puts the peak back. The frozen cell folded into content replays its 48
d0-d3 games on 4 seeds exactly. After a first fresh read (90.3 / 77.1 / 66.7 / 58.3) d2 and d3 sat 7 and 13 over:
B2 (fresh d3 -6.2 ±4.1 on the same games) and F1 were screened on the dev seeds and adopted, and False Flag's added
tier-1 threat was taken out (a project adding threat count); the whole fresh read below is the final build's.

## The reads

**The first fresh seeds** (36680-36699 and 36748-36775) read the build of 9:51am (frozen + B2 + F1): 92.4 / 78.5 /
63.9 / 50.7 against r58's 84.0 / 66.7 / 52.1 / 42.4 ([read_r59.md](read_r59.md)). B2 and F1 had been picked after an
earlier read on these seeds, so that read is not a confirm (Matt, message 6016); Matt's message 6017 then asked for a
credit lever, and W3 + G6 + F0 (screened on the dev seeds) went in. **The confirming read below is on seeds no screen or
read of this item used** (the ledger's next free), r58 beside it on the same seeds; it is the only independent read.

| independent seeds, 48 a pair, 144 games a level | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| r59 final, 36776-36799 + 36832-36855 (before M13) | **88.9 ±2.6** | **81.9 ±3.2** | **63.2 ±4.0** | 64.6 ±4.0 |
| r58, same seeds | 82.6 ±3.2 | 74.3 ±3.6 | 59.0 ±4.1 | 51.4 ±4.2 |
| paired r59 - r58 | +6.2 ±4.3 | +7.6 ±4.6 | +4.2 ±5.3 | +13.2 ±5.5 |
| r59 final, 36856-36903 (before M13) | **88.9 ±2.6** | **78.5 ±3.4** | **54.9 ±4.1** | 52.8 ±4.2 |
| r58, same seeds | 86.1 ±2.9 | 75.0 ±3.6 | 57.6 ±4.1 | 45.8 ±4.2 |
| **both sets pooled (288 games a level)** | **88.9** | **80.2** | **59.0** | 58.7 |
| **r59 with M13 (final), 36904-36951, d3 only** | | | | **46.5 ±4.2** |
| r58, same seeds (paired +2.8 ±5.8) | | | | 43.8 ±4.1 |
| the build before W3 + G6, first independent set | 88.2 | 86.1 | 68.1 | 66.0 |
| paired final - that build | +0.7 ±2.9 | -4.2 ±3.8 | -4.9 ±5.1 | -1.4 ±5.0 |
| rounds: mean, median (r58 median), first set | 9.15, 9 (9) | 8.92, 9 (9) | 8.66, 9 (9) | 8.30, 8 (8) |
| ends, first set: Unity deck / node / Discord deck / purge / cap | 107 / 1 / 9 / 21 / 6 | 112 / 4 / 21 / 6 / 1 | 87 / 12 / 41 / 4 / 0 | 92 / 23 / 26 / 1 / 2 |

M13 (d3's second counter step) only acts at d3, so d0-d2 above are the final build's. Pooled, d3 read no harder than
d2 (58.7 against 59.0); M13 was picked on the dev seeds (44.8 to 35.4) to take about 10 off, and read on a third new
set. The rest of this section is the first independent set.

r58 reads 8.7 points easier at d3 on these seeds than on the first set: seed sets move d3 as much as a lever does.

**Nodes lost a game** (r59 / r58): uncountered threats 2.07 / 2.85 (d0), 2.83 / 2.78 (d1), 5.47 / 3.49 (d2), 6.61 /
3.63 (d3), corruption 2.24 / 2.10, 2.31 / 1.96, 1.93 / 1.96, 2.25 / 2.49: threats lead from d1 (d0: 2.07 against
2.24). Spent as counters 0 (r58 0.6-0.9).

**Threats a game** (dealt / countered / uncountered): r59 d0 23.0 / 18.1 / 2.42, d1 23.3 / 17.6 / 2.92, d2 24.3 / 15.6 /
6.15, d3 23.8 / 14.8 / 5.89; r58 31.8 / - / 4.08, 31.7 / - / 4.12, 30.8 / - / 4.83, 31.8 / - / 5.56 (r58 has no
countered-by-tier tally). By tier at d2: t1 11.4 / 7.9 / 3.05, t2 9.0 / 6.1 / 2.10, t3 4.0 / 1.6 / 1.00.

**Counters by what paid** (a game, d2, r59 / r58): credits 7.0 / 13.5, energy 1.2 / 1.0, insight 1.2 / 0.9, tags 0.62
/ 0.62, research cubes 0.12 / 0.07, ambassadors 0.01 / 0.03, nodes 0 / 0.48, cards 2.6 / 2.9, standing (capstone auto
counters) 2.8 / 3.9. **Credits are 45% of all counters at d2** (r58 58%; d0 42%, d1 44%, d3 42%), 75% of those paid
in a currency. Per tier: confirm_r59.md.

**Credits a seat** (r59 / r58): peak 93 / 88 (d0), 83 / 82 (d1), 75 / 73 (d2), 66 / 66 (d3); at d2 Builder 69 / 79,
Protector 61 / 60, Researcher 93 / 79. Spent 440 / 427, 400 / 403, 380 / 364, 344 / 333.

**Claims a game** (d0 / d2, route): stations by card 0.90 / 0.58 (new), by ambassadors 0.10 / 0.05 (r58 0.14 / 0.02);
cities 0.27 / 0.09 (0.19 / 0.10); factories 0.17 / 0.03 (0.07 / 0.02). Stations by card are most claims.

## The death audit (136's `death_audit.py` on the first independent set's last-node losses, final build)

4 losses at d1, 12 at d2, 48 at d3 (r58 on these seeds: 21 / 37 / 48 node ends). **The fatal blow:** d1 threats 3,
corruption 1; d2 threats 11, Border War 1 (Cascade Failure 4, War Economy 2); d3 threats 37, corruption 6, Border War
4, other 1 (General Mobilization 5, Data Breach 5). The fatal phase is the threat execute step in 3 / 11 / 37. A
node-safe line (extra placements affordable against nodes needed) existed in 0 / 4 / 10; corruption deaths with a
node-safe placement 1 of 1, none, 3 of 5. The fatal hit took every node held at the end of the round before in 2 / 4 /
24. **Verdict:** the deaths are the board's local threat hits, rarer than r58's at d1-d2 and as many at d3, where an
unpaid threat takes 2 more nodes and counters cost 3 more an item.

## The new cards and op in play (confirming read, 576 games)

Microgrid Lab Compact kept in 89% of games, played in 71% (0.72 plays a game); Power Purchase Agreement kept in 28%,
played in 30% (0.30). Stations claimed by card 0.54-0.90 a game against 0.01-0.10 by ambassadors: the energy route is
now how stations are claimed. MLC's keep share reads strong for a tier-2 building at 8 credits (flagged, not
repriced: the ledger reads it fair at 8.6). `loop_smoke.py --run`: no timeouts, no card at the play flag.

## Speed

`per_decision.py` (r58 against r59, strategist, d2, 6 games from 1000, one thread, nothing else running): 2.3 ms a
decision against 3.4 (1.48x), 1087 ms a game against 1861; options a decision 16.6 to 24.0 (1.44x): the menus' three
currencies each add pay and withdraw options. Not fixed here; `speed.py` gives no verdict across a revision.

## Done when, against the independent reads

- No threat's counter shares a currency with its uncountered harm, and the check refuses one: **met** (0 of 64;
  `check.py`, the load check and the adversary module's nodes rule; overlay test).
- Counters paid in a mix, no currency over about half at d2: **met** (credits 45% and 48% of counters at d2 on the
  two independent sets, r58 58% and 60%: G6 put the d2 step on credits, +6 against +2 an item).
- Credit peaks fall clearly from r58's 60-80: **not met.** d2 peak a seat 75 / 73 and 69 / 76 (r59 / r58, the two
  sets): 2.5 lower pooled. Every cell that lowered peaks clearly (58-64 at d2) cost d0-d1 6-30 points (the
  calibration table): agents bank whatever credits arrive and pay scarce currencies last. W3 (3 credits a round into
  1 energy) took the d2 peak 6 lower on the same seeds at no cost at d0.
- d1 / d2 / d3 within about ±5 of 75 / 60 / 45, d0 90 or more, median 8-10: **partly: d2, d3 and the medians met;
  d0 and d1 not** (the closest cell adopted). Pooled independent 88.9 / 80.2 / 59.0 (d0-d2, 288 games a level), d3
  46.5 on the third set; medians 9 / 9 / 9 / 8. **d0 88.9** (1.1 short): its losses are round-5 deaths and the cap
  (node 1-4, cap 6 of 144) more than threats, and every lever that raised it (E5, W3+E5, W3b) raised d1 more (85-91 on
  the dev seeds). **d1 80.2** (+5.2): the d0-d1 gap is d1's one step (Corrupt X one fewer), 8.7 pooled where the band
  wants 15; a harsher d1 step was not screened (the prompt fixes d1 as "Corrupt X one fewer, as now").
- Extra threats appear only at d4: **met** (`extra_threats_at` 4; the swap at d4 too; False Flag deals one tier-2
  threat a country, as at r58).
- The rules docs' new-player summary covers counters, harms, the ladder and claiming: **met** (threats.md's *For new
  players* and ambassadors.md's).
