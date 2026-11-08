# 148: card and economy rulings (r62), the agent fixes, the ladder

Rules revision 62 (bumped from the constant, 61 to 62, as the prompt said). Matt's card rulings of 2026-11-07, the
agent and small fixes queued from 145 and 147, then the strategist ladder paired against r61 on fresh seeds. Card edits
are one overlay, [cards148.overlay](cards148.overlay) (`scripts/cards/apply_overlay.py`). Scripts: [specs.py](specs.py)
(screens and reads), [cards148.py](cards148.py) (the named cards kept / played, claims, the Auto Counter's cut, the
round cap's share of losses), [holdings148.py](holdings148.py) (item 6).

## Give / get, before the first screen

At `configs/agents/values.json` (give: credits + play insight at 2.5; get: the table's worth) and at Matt's anchors
(pay: credits + 4 × (keep + play insight) + other costs at the anchors; get: the effects at the anchors).

| Card | Change | Table give / get | Anchors pay / get |
|---|---|---|---|
| Exascale Training Cluster (t2, 15 cr, 2 ins) | energy 8 → 5; the +25 override cut | 42.5 / 38.25 (0.90): **a loss** | 53.5 / 73.5 (1.37) |
| Microgrid Lab Compact (t2, 1 ins) | 8 → 14 cr; claims as Claim does | 16.5 / 20.75 (1.26) | 26 (+9 a use) / 33 (1.27) |
| Power Purchase Agreement (t1, 2 cr, 2 energy) | claims as Claim does | 11 / 20 (1.8); each adversary ambassador on the station +4 give (an ambassador) | unchanged price |
| Autonomous Defense Grid (t2, 9 cr, 1 ins) | Cyber threats anywhere need 6 less (was 5, near the holder) | 11.5 / 16 (1.39) | 21 / 29 (1.38) |
| Market Stabilization Board (t2, 9 cr, 1 ins) | its Economic twin, same change; out of the deck | 11.5 / 16 (1.39) | 21 / 29 (1.38) |

- **Exascale:** the table prices energy at its full anchor (4.5) and insight and a research cube near 0.6 of theirs
  (2.5, 3), which is why 130 gave it +25. Matt cut the override, so the table reads a loss; per the 2026-11-05 rule the
  fix would be an override row, which the ruling removed. Flagged, not changed (needs a decision).
- **Microgrid Lab Compact, the anchor price:** a claimed station makes 2 cubes a round (`owned_station_cubes`), at the
  anchor's 6 a cube for about 4 rounds (142's horizon), less the risk the table already takes (20 of 24, × 0.83): 40 a
  claim, 31 net of its 2 energy. A holder claims about once a game (0.5-0.75 uses a game where played, r61 and r62), so
  get 31 + 2 tags = 33; at 1.27, 14 credits (13 reads 1.32, 15 reads 1.22). 142 priced a claim at the 3-ambassador
  route (15), not at what the station makes, which is why it read 1.0 and was kept in 89% of games.
- **The Auto Counter, the table row:** `auto_counter` 0.67 → 1.0 a point a round: 0.5 cuts a holder-round (measured on
  the first r62 screen: 6.6-12.9 points cut a game where played, over about 4.3 rounds held, at 6 a cut) × 12 credits
  a cut (a scarce option paid in full, about 3.5 units at the table) / 6. r61's measure on the same seeds was 0.41 cuts
  a holder-round at cut 5.

## What the cards did

The final read's seeds (37528-37623, 288 games a level), r61 → r62, strategist, share of games:

| Card | d0 kept / played | d1 | d2 | d3 |
|---|---|---|---|---|
| Exascale Training Cluster | 56 / 7 → 72 / 3 | 48 / 3 → 65 / 1 | 45 / 3 → 65 / 1 | 39 / 1 → 59 / 1 |
| Microgrid Lab Compact | 94 / 81 → 91 / 70 | 94 / 79 → 91 / 58 | 96 / 72 → 92 / 58 | 91 / 65 → 89 / 51 |
| Power Purchase Agreement | 31 / 33 → 31 / 32 | 32 / 28 → 33 / 31 | 32 / 23 → 30 / 25 | 31 / 25 → 34 / 23 |
| Autonomous Defense Grid | 29 / 13 → 66 / 42 | 24 / 7 → 58 / 39 | 23 / 3 → 55 / 31 | 22 / 3 → 50 / 28 |

- **Exascale is still kept and not played.** Energy is not what holds it back: on the dev seeds (d1-d2, 96 games
  each) it is played in 1-4% at 5 energy, 4% at 4 and 10-11% at 3, kept in 67-86% throughout. The keep plan wants it and
  the play eval does not (with the override cut, the table reads it a loss). 5 stays, Matt's starting value; the
  decision is listed below. Kept more than at r61 (energy 5 is easier to plan for).
- **Microgrid Lab Compact at 14** is played less (−11 to −21 points) and kept as much; claims by card a game hold
  (0.93 / 0.58 / 0.60 / 0.46, r61 0.90 / 0.74 / 0.59 / 0.55). Uses a game where played 0.45-0.87.
- **The Auto Counter.** The first try (3 less, table-wide) was the wrong lever: on the dev seeds it fired no more often
  (fires are bounded by threats some player is near, and on a two-player table that is usually the holder), it just
  cut less (threats countered by it 0.27 → 0.03 a game at d0). At 6 less, table-wide, Autonomous Defense Grid is
  played 3-5× as often, cuts 6.6-9.1 points a game (r61 0.2-1.2) and counters 0.8-1.4 threats a game on its own (r61
  0.02-0.25). **Market Stabilization Board** in a deck with it (a second config, dev seeds, 96 games a level): played
  in 40-50% of games; win against the base deck −2.0 / +2.1 / −8.3 / +13.6 at d0-d3 (unpaired games: the deck
  changes every deal), unresolved, so it stays out.

## Item 6: leftover resources

What each seat held at the end of the turn before the winning one, before income (`unspent.<seat>.<resource>` at the
winning round less 1), won games of the final read, per board ([holdings_f1.md](holdings_f1.md), every resource).
"Large": over a sink's price and over two rounds of the seat's own income of it.

| d | board | wins | credits held (income a round) | energy held / large (income) | research cubes held / large (income) | chips / antimatter / ambassadors large |
|---|---|---|---|---|---|---|
| d0 | Builder | 160 | 12.1 (32.5) | 4.6 / 25% (3.2) | 0.8 / 4% (0.8) | 15% / 21% / 7% |
| d0 | Protector | 175 | 10.8 (30.5) | 3.4 / 4% (4.2) | 4.1 / 24% (1.3) | 13% / 16% / 14% |
| d0 | Researcher | 171 | 14.3 (36.2) | 4.2 / 27% (2.9) | 2.9 / 15% (1.1) | 12% / 11% / 17% |
| d1 | Builder | 133 | 10.6 (32.7) | 4.9 / 34% (3.0) | 0.5 / 1% (0.7) | 12% / 12% / 6% |
| d1 | Protector | 153 | 8.6 (29.8) | 2.5 / 5% (3.8) | 3.1 / 19% (1.0) | 10% / 10% / 18% |
| d1 | Researcher | 146 | 13.7 (32.8) | 4.0 / 34% (2.4) | 2.8 / 17% (0.9) | 14% / 12% / 16% |
| d2 | Builder | 105 | 8.8 (28.9) | 4.0 / 30% (2.5) | 0.4 / 0% (0.5) | 8% / 7% / 5% |
| d2 | Protector | 133 | 8.6 (30.3) | 2.2 / 3% (3.8) | 2.9 / 20% (1.1) | 12% / 7% / 14% |
| d2 | Researcher | 128 | 12.1 (34.6) | 3.8 / 30% (2.5) | 2.1 / 8% (0.9) | 8% / 9% / 14% |
| d3 | Builder | 75 | 8.5 (29.9) | 2.8 / 21% (2.8) | 0.3 / 0% (0.5) | 7% / 5% / 4% |
| d3 | Protector | 100 | 7.1 (26.7) | 1.6 / 1% (3.8) | 1.7 / 9% (0.7) | 6% / 1% / 4% |
| d3 | Researcher | 95 | 11.7 (32.8) | 3.2 / 24% (2.7) | 2.1 / 9% (0.8) | 12% / 3% / 7% |

**No resource piles up with no reasonable sink, so no sink was added.** Credits: a third of a round's income at most,
never large. Energy: the Builder and Researcher hold over two rounds' energy in 21-34% of wins, the Protector (Shelter
spends it) in 1-5%; in those seat-games the seat left at least one node power unused that round 71% of the time (each
takes energy: 1, or the board's second plus the track space), a threat went uncountered that round in 11%, and
Exascale Training Cluster (5 energy) sat kept in most hands: sinks were open and declined, and late nodes are worth
little to a team about to win (`node_late`). Research cubes: the Protector (9-24%) and Researcher (8-17%) hold 2-4
rounds of cube income; the cube converter (1 insight a cube, any number a round) and a completed megaproject's
converter are always open, so this is the "insight has nothing to buy that turn" case, not a missing sink. Antimatter
(1-21% hold 2+) has only the tier-2 ultimates as a sink, by design: it is held when a planned ultimate never came, one
of Matt's good reasons. Chips (6-15% over 7) and ambassadors (4-18% over 4) have their sinks every round (Procure,
Parts Exchange, the sale; Claim, Liberate, counters). The game was won dominating (3+ adversary cards left) in 11-22%
of the large stocks.

## Item 7: the round cap's share of d0 losses

On the final read, **17 of 35 d0 losses (49%) are the round cap** (`deck_empty`: the event deck ran out); the rest are
the Discord deck (18). r61's agents on the same seeds: 17 of 35 too. At d1-d3: 12 of 72, 6 of 105, 24 of 153.
A round more at d0-d2 (`stage_cards` 4,5,3, d3's cap held at 10 by `stage2_removed` 2; 64 dev seeds) moved d0 by
+0.5 ±2.0 and left its round-cap losses at 6 of 10: those games never reach a Unity win, so a longer clock doesn't save
them ([screen_cap.md](screen_cap.md)).

## The fixes

| Fix | What | Test / on-off |
|---|---|---|
| Shelter forecast | `agents/guide.rs`: one armor pool a hex for the whole forecast, drawn down across the country's threats in resolution order (hardening, then the node owner's ambassador, then the site owner's, as `destroy_piece`); "destroy a node" never takes a seat's last node (`destroy_nearest_node`), troop destroys spare it below the last-safe stage | tests `armor_is_one_pool_across_a_countrys_threats`, `the_forecast_spares_the_last_node`. Greedy line (d2, 96 games, against r61's binary at r61 rules): 2 of 96 games differ (one flips to a loss), win 71.9 → 70.8; strategist 0 of 96 differ; greedy 11.3 → 10.5 ms a decision (0.93×, same games) |
| D68 | the gates' Unity-farm check and its `unity_farm` switch removed (farmed a reward 137 removed) | `gate.json`, `gate-exploit.json`, principles.md; dominance list 21 → 20 |
| D65 | Rolling Blackouts "until the end of next round", Market Crash "at the next upkeep" (the faces follow the ops) | text only |
| D59 | a troop's arrow counts the next march's Forced March: this round's card until its march, then next round's (face up in `NEXT` from the reveal) | `a_troop_marches_two_steps_toward_node_then_strikes` extended (a review found the first version dropped the twist after the march) |
| D61 | `RoundFlag::DoubleBuild`, `double_build_at`, creation's `twice` removed | same play (golden moved only with the rules) |
| Fingerprint | `GameState::fingerprint` hashes the state with `stats` cleared; wasm exports `fingerprint`, `wasm-smoke.mjs` uses it | a measures-only tally moves no golden; 148's own measures (every `unspent.*` exported, research cubes recorded) moved none |
| `near_mask` | `threats::near`: a threat's reach as a u128 bit mask (no allocation) for `is_near`, `seats_near`, `nodes_near`, `near_hexes`; the Vec `reach` stays as the fallback past 128 hexes | `near_mask_matches_the_reach` (every step of random games, 2-4 countries) |
| D64 | Recruitment Drive and Infiltration Drive say "not corrupted" (aligned cities count, as the engine always did) | text only |
| Search memo (speed, same play) | `agents/search.rs` `position_hash` hashes the piles, which `same_position` compares: positions differing only in piles shared one bucket, and one Shadow Factories scoring made 6.2M deep compares (r62's strategist 1.30× a decision; r61 has the weakness, r62's play found it) | F1 set a replayed: 576 of 576 games the same (`same_results.py`); strategist 1.30× → 1.16× a decision, greedy 1.11× → 1.08× |
| View | a country flag's label is its doc's name ("Stations dark"), not its key | the invariant test refused `station_dark` once Rolling Blackouts showed up in a fast-lane game |

## The ladder

Strategist, two 48-seed sets pooled (37528-37575, 37576-37623), the three pairs, 288 games a level, paired against
r61's binary on the same seeds ([final_read.md](final_read.md), [final_read_detail.txt](final_read_detail.txt)):

| d | r62 | r61 | paired gap | r62 set a / b | band |
|---|---|---|---|---|---|
| d0 | **87.8 ±1.9** | 87.8 ±1.9 | +0.0 ±2.2 | 86.8 / 88.9 | ≥ 90: **unmet** |
| d1 | **75.0 ±2.6** | 74.3 ±2.6 | +0.7 ±2.6 | 74.3 / 75.7 | 70-85: met |
| d2 | **63.5 ±2.8** | 60.8 ±2.9 | +2.8 ±2.6 | 59.0 / 68.1 | 55-65: met |
| d3 | **46.9 ±2.9** | 45.5 ±2.9 | +1.4 ±2.5 | 34.0 / 59.7 | 40-50: met |

Pooled d2 is under 65, so the screened d2 step (`counter_more_credits` 3 → 6) was not taken; no dial changed.
**d0 is unmet** by 2.2 (1.2 SE). What bound it: the seed set (r61's agents read 87.8 on the same games; r62 reads 94.3
on the 64 dev seeds) and no dial that reaches d0 (the counter steps start at d1; a round more moves d0 +0.5 ±2.0, as
item 7 shows). The two d3 sets differ by 25.7 points on the same build: a level's 144 games are still a ballpark.

## Speed

`per_decision.py`, c2 d2, one thread, against r61's binary: strategist (32 games) 1.5 → 1.7 ms a decision, **1.16×**,
with rounds, decisions and options a decision level (0.98-1.00×); greedy (16 games) 9.1 → 9.8, 1.08×. Before the memo
fix: 1.30× and 1.11×. On identical games the shelter forecast costs 1.00× and `near_mask` 0.99×. A profile (Visual
Studio's collector, symbol builds of both) found the 1.30× in one game (seed 1013, 10.7 ms a decision against 1.4) and
the memo's bucket scan behind it. What is left, 1.14-1.16×, scatters per seed (0.47× to 2.84×, median 1.07×): play,
not code (more free-build checks with Microgrid Lab Compact and Exascale cheaper, more threat re-pricing with the Auto
Counter). Kept; the speed baseline moved to r62 (`speed.py check --update`).
