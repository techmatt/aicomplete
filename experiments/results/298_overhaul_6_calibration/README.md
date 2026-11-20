# 298: overhaul 6 of 7, calibration (r143 → r144)

Matt's rulings of 2026-11-20 (Redeploy toward the players, Offensive's lowest-numbered pick, too early or too hot
priced higher, the gated Strike rider) built and read; the ladder re-laid; the judge retaught; the bands read on
fresh seeds; three seats, hedges, the rulebook, `docs/teach.md` and the table cost. Rules revision 143 to 144,
bumped from the constant. Everything here is a candidate for his review.

- Tables: [fresh.md](fresh.md) (the fresh read, both sides, judge and greedy), [screens.md](screens.md) (the dev
  screens: levers, the Strike rider on the final build, three seats), [table_cost.md](table_cost.md) (step 8).
- Content edits: [`scripts/content/sweep_298.py`](../../../scripts/content/sweep_298.py), rerunnable from r143's
  sources. Readers: `screen.py`, `seats.py`, `early_troops.py`, `make_tables.sh` (rewrites the two tables).

## The rulings, as built

| Ruling | Built | Read (fresh, the judge, r143 → r144) |
|---|---|---|
| Redeploy toward the corrupted city nearest a player piece | each troop moves onto **its own country's** corrupted city nearest a node or ambassador (hex distance; ties: lower hex; Wall's city excluded); one already there stays | troops moved a game: d0-d2 0.00-0.03 (was 0.00-0.02), d3 0.04 → 0.22 |
| Offensive: lowest-numbered node hex holding an agent or a troop | country, then hex number; none: it deals nothing (a tally) | idle on about a third of its resolves at d2-d3 (screen 1) |
| Martial Law priced higher | 24 → **36** compute a board | project troops by round 3 with no botnet or factory troop in the game: d2 7 → 0, d3 16 → 8 of 288 (`early_troops.py`) |
| Every way to gain an ambassador priced higher | Gain a player ambassador 16 → **20** credits; Accord 2 → **3** energy | Gain 0.99 → 0.58 a seat-game (hedge band 0.05-0.75); Accord 0.61 → 0.49 a Protector seat-round (its row 0.05-0.90) |
| Strike, gated | rider built behind `board.strike_home`, **off** | see below |

**The Strike gate.** With Redeploy in (screen 1 ran on f3facbcc, which had it), Strike hits at d0-d1 were 0.07 /
0.14 a game; the rider sent home 0.00 / 0.05 ambassadors there. Re-read on the final build (4ad672dc: r144 and the
retaught judge; Matt's message 13713): hits 0.02 / 0.03, the rider sends home **0.01 / 0.01** at d0-d1 and costs d2-d3
1-2 points. It gives Strike no work at d0-d1, so it is not kept. Fresh, Strike hits a game: d0 0.07 → 0.03, d1 0.12
→ 0.05, d2 0.25 → 0.22, d3 0.69 → 1.02 (Arms Race's troops).

## The ladder

`research_at` is a real per-level list: **K 3 a country at levels 0-1, 2 from level 2** (War Footing gains a setup
step). Level 3, **Arms Race**, gets a rule: "When the research track reaches a Garrison icon, each country takes 2
Troops, not 1" (`arms_race_at` 3, `arms_race_troops` 1; the generator refuses a rule text that disagrees). On
screen 1 K 3 took research losses at d0-d1 from 3 to 0 of 192 (wins +1 / +5, noise-sized); Arms Race doubled d3's
Strike hits (0.36 → 0.68) with d3 wins unchanged (66%). Fresh, research losses: d0 5 → 1, d1 14 → 0, d2 22 → 26,
d3 48 → 52 of 288. Levels now differ by a step each: 0 the board; 1 compute and Sleeper Cells; 2 compute, K 2 and
War Footing; 3 compute, a shorter deck and Arms Race; 4 compute, Infiltrate 2 and its deck.

## The fresh read (seeds 50688-50783, two sets of 48 pooled, three pairs, 288 games a level)

| | d0 (≥90) | d1 (80-95) | d2 (75-90) | d3 (55-70) |
|---|---|---|---|---|
| Judge r143 → r144 | 92 → **93%** | 89 → **89%** | 83 → **81%** | 62 → **64%** |
| Judge r144, set 1 / set 2 | 89 / 97 | 90 / 88 | 81 / 81 | 69 / 58 |
| Rounds r144 | 8.4 | 8.9 | 9.0 | 8.5 |
| Greedy r143 → r144 | 95 → 96% | 92 → 90% | 85 → 84% | 66 → 65% |

A per-round project counter landed after the freeze; the judge's side replayed on the final build (46bc29cd)
differs in 0 of 1,152 games (`farm/compare.py`). **Every level sits in its band, in 8.4-9.0 rounds** (4-14 at the extremes; no round-3 win at d0). The comparison
changes the rules and the judge at once: r143 → r144 credits neither alone. The difficulty step from d0 to d3 is
29 points, and each level is below the one before.

## The judge

Retaught (`crates/aic-sim/src/agents/eval.rs`; a helper's work, screened on the dev seeds before any fresh seed):
its forecast now sees the next Escalate, the board's rule (Commandeered Plants) and each corrupted factory's push
(`w.adv_read`, mirrored on `rally.rs`' escalate and `factories_output`); its win clock paces off the corrupted cities
left against its liberation rate, not Unity tips (`w.lib_clock`). Both are judge-only (greedy's weights stay 0).
Troop wall's text is rewritten for garrisons and Strike. **D120** (`w.sac_tags`, `plan_sac`) off against on: −0.3 ±
0.6, no effect; it stays on. Dev screen (r143 rules, 384 games an arm): judge 78.6 → 81.0% pooled (+2.3 ± 2.0),
greedy 81.0%; d0-d2 judge 88.7% against greedy 88.3%. Cost 1.11× a decision (`per_decision.py`, a loaded box).
**Fresh, greedy still leads the judge at d0-d2 by 3 / 1 / 3 points** (same seeds; about one and a half SE each).
On the final build's dev seeds greedy leads by 1.0 ± 2.4 pooled, and no one of the judge's own terms explains it:
without the liberation bill (`w.purge`) −0.8 ± 2.2, without the building Action term +0.3 ± 2.1 (both win d0 more,
95-97% in 7.9 rounds, and d2 or d3 less), without the hand terms −50 ([screens.md](screens.md)). So the judge is
at greedy's level within noise, not above it; a better judge needs something greedy lacks, not one of its terms off.

## Three seats (`configs/three_seat.json`, dev seeds, 32 games a level)

| | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| Judge: wins, rounds (r142, 295: d0 50% in 12.4) | 75%, 10.8 | 78%, 10.8 | 59%, 11.1 | 47%, 9.8 |
| Greedy | 91%, 10.2 | 66%, 11.4 | 69%, 11.2 | 44%, 9.9 |

Shorter and better than r142 but still long, for both sim players; a quarter of d0 games reach the round cap. One
corrupted city a country (c1) and Liberate at 4 (lib4) don't shorten it (9.7-11.1 rounds; [screens.md](screens.md)).
By round (`seats.py`), three-seat seats meet the same threat load and slightly fewer agents a seat but hold 30-60% more
unspent ambassadors from round 5 on: the pace is lost in getting ambassadors onto cities, not in the adversary's pressure.
Not changed: a three-seat setup rule is Matt's (below).

## Prices and hedges

Every standard action's use on the fresh read is inside its band except Make antimatter (0.05, at the band's
floor: the desperate source). Remove agents went **2 → 3 ambassadors** (0.99 → 0.43 a seat-game; "any other hedge over
its band"). Turn a cube into insight 0.61 and Remove a troop 0.27 are in band and keep their prices. Give / get at the unit
anchors (card-guidelines.md: energy 3, ambassador 5.33, agent 3.33): Accord 9 for a node and an ambassador 10.67,
1.19× (was 1.78×); Gain a player ambassador 20 credits for 5.33, 0.27×; Remove agents 16 for 10, 0.63×: the hedges
stay poor routes, as ruled; Martial Law 36 compute a board for 2 troops a country (9.33), 0.26 a compute. Fortify I
(0.19) and Interdict I (0.24) dip under the troop-response floor at d0-d1, where K 3 brings fewer Garrisons.
**Typed jams stay:** none is dead (played 0.21-0.36 a game; actions used 0.13-0.44 a game, Garrison Command the most).
Resources spent of gained: credits 106%, ambassadors 75%, energy 69%, chips 64%, research cubes 60%, antimatter 38%
(d3-d4, under its band: as at r143).

## Docs

`docs/teach.md` is a ten-minute teach of the game as built (about 1,700 words, so 10-11 minutes aloud). The rulebook
is rewritten whole for r144 (war, fronts, the march, purge steps, Dispatch, Claim and city growth gone; §11 and §19
are troops) and checked against `RULES_REVISION` again (`RULEBOOK_REVISION` deleted, `regen.py`'s revision line back).
At 88 KB it is as long as the frozen one: the icon legend, glossary and the doubled troop and corruption sections
are the next cuts.

## For Matt

- **Three seats run 10-11 rounds** for both sim players. Options: a three-seat setup line (fewer starting corrupted
  cities only at three seats did not shorten it here), a larger starting ambassador supply at three seats, or accept
  it until a playtest. Pick: leave it for the playtest; neither lever read moved the length.
- **Martial Law at 36** still resolves by round 3 in about 3% of d3 games (8 of 288); the rest of d3's early troops
  are Botnet Conscripts and War Plants (16), which the ruling did not name.
- **The judge still trails greedy** at d0-d2 on fresh seeds; the bands are read on the judge, so a stronger judge
  would lift every level by a few points.
