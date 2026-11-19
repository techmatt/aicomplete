# 278: hard-cut proposals: where a d2 round's minutes go, and what to cut (r132)

Proposals and prototypes only. **No rules change, no `RULES_REVISION` bump** (r132: main at 0873d56, after 277's
Growth pass).

**Flags.** Two prototype flags stayed in code at baseline 0: `cards.no_pass` (cut 1) and `board.turn_cap` (cut 3). Two more are reverted and kept as patches: [pull_any_tag.patch](pull_any_tag.patch) (cut 6) and
`host_fixed.patch` (cut 8). A stats-only counter, `map.arrival_checks`, measures cut 4.

**Later (Matt, 2026-11-18; removed in 281):** no turn budget ("too many non-card turns means things are too cheap"),
so `board.turn_cap` left the code and `host_fixed.patch` was dropped (in git history before 281). `cards.no_pass`
stays as an ablation switch only: passing is locked.

With every flag off, the 28 traced games replay identically. Only the config hash moves, and with it
`GOLDEN_FINGERPRINT`, because the state hashes its config.

**Rates (268, Matt's yardstick):** 15 s a decision (every engine decision with more than one option, as 274-276
did: amendment 1), 3 s a component, 10 s a check, 20 s a lookup (a new card face read included), 5 s a modifier.

**Traces:** 268's seeds 268000-268013, the judge, Builder and Protector, d2 and d4 ([traces.sh](traces.sh)), 28 games
a side.

## 1. The breakdown

[breakdown.py](breakdown.py) counts every event in a trace, assigns it to its phase and step, and gives it a kind:
- **Decisions:** actions with more than one option.
- **Components:** tokens, resources, cards, track markers (one move per marker per step) and used markers.
- **Checks:**
  - the adversary's per-site checks (the counter);
  - one per troop in the march;
  - one per threat card turned;
  - the board uses a seat weighs (276's `ai.offered`), plus each target asked;
  - 5 per Buy energy (268's re-run of the host matching);
  - each factory, station and city read in production, plus each host;
  - each threat read when threats execute;
  - each front at war.
- **Lookups and modifiers:** the trace can't see these, so they are **modeled**. They come from 268's hand-play log (r125, d2), shared out by step, minus 274's income modifiers and 275's city-growth step.

Full tables: [breakdown-d2.md](breakdown-d2.md) and [breakdown-d4.md](breakdown-d4.md). Every step has a row (the
small ones are grouped as "other steps"), the steps add up to each section and the sections add up to the round.
Below the tables are the counts this page quotes.

| Section (a round) | d2 minutes | d4 minutes | What drives it (d2) |
|---|---|---|---|
| Action phase, both seats | **15.9** | 14.0 | 32 decisions (card plays 8.2, Growth 5.6, Dispatch 4.7, building Actions 3.1, Liberate 2.1, passing 1.8), 65 components; 2.1 modeled minutes of Builder arithmetic (r125's discounts and rebates) |
| Draft | **9.4** | 8.6 | 21 decisions (tier 7.6, pick 6.6, keep 5.1, pass 1.4), 26 cards moved, 7.6 new faces read (2.5 min) |
| Production and hosting | **4.2** | 4.4 | 17.8 sites read for producer and blockers (3.0 min) |
| Adversary: lines and arrivals | 3.5 | 3.8 | 15.5 per-site checks, 7.6 of them arrival checks |
| Adversary: compute and projects | 2.2 | 3.0 | 17 components (marker moves, seeded agents); modeled modifiers |
| Adversary: threats dealt | 2.0 | 2.1 | 5.8 cards turned (2.2 dealt here, 3.5 rejected), 2 new faces |
| Threats executed | 1.9 | 2.2 | 10 components, 1.8 decisions (pay or discard) |
| Adversary: march | 1.4 | 2.8 | 1.6 troops (d4 5.1), 1.2 choices |
| Cleanup, war, sway, the two adversary card steps, income and upkeep, bases, round start | 2.0 | 2.1 | used markers cleared 0.5; war 0.4 |
| **Round** | **42.4** | **42.9** | |
| by kind: decisions / components / checks / lookups and faces / modifiers | 14.1 / 8.2 / 9.2 / 8.7 / 2.3 | 12.5 / 8.7 / 9.1 / 10.7 / 1.8 | |
| Rounds a game (traced), minutes a game | 8.00, **339** | 6.36, 272 | |

**Seats against everything else (d2).** The seats' own rows (their draft and action phase, with their modeled
arithmetic) come to Builder 11.2 and Protector 9.9 minutes. 276's `trace.md` had 9.7 and 9.1 without lookups or
markers. Everything outside the seats is **21.3 minutes, not ~34**: the adversary's turn 9.5, production 4.2, the
shared draft (dealing, faces, modeled) about 4, threats executed 1.9, the rest about 2.

**Reconciling with 52.6** (268's r125 hand count, 60.0 a round: lookups 4.8, modifiers 3.3, checks 23.7, components
11.9, decisions 8.6 with its 58 folded to 0.6 × 15 s, faces about 7.7; less 274-276's deltas, from three different
engine proxies). Against this model's 42.4: decisions at the full 15 s add 5.6 (folded as 268 did, the round is
**36.8**); the hand count's 142 checks a round include reads the trace can't see (tags, targets in reach, the hosting
plan) where the model counts 55; new faces fall from about 24 to 12 a round (a 4-card draft against 268's 8).

Read 42.4 as a **floor**, of the same kind as 274-276's numbers.

**The known candidates the breakdown rules out:**
- **The Builder's late action phase** is gone as a spike. The Builder now plays 2-3 cards a round in every round (breakdown-d2.md's counts: r1 2.7 … r7 2.8), against 268's 14-16 late. What is left of it is cut 5.
- **Income and upkeep** is 0.2 minutes a round: nothing to cut.
- **Components on the adversary's turn** are 1.7 minutes at d2: compute and projects 0.9, lines 0.3, march 0.3, the two card steps 0.15. That is cut 9.

**What 2 hours needs.** At 8 rounds, 120 minutes is 15 minutes a round, a third of today's 42.4. The decisions alone
are 14.1 minutes a round (56 at 15 s). Even with every cut below (about 30 a round), 120 minutes is about 4 rounds.
**The 2-hour d2 game needs a shorter game (about 4 rounds) as well as the cuts.** Halving the decisions alone would
save 7 minutes a round, which is not enough.

## 2. The proposals, ranked by d2 minutes saved a round

How each line was measured:
- **Measured:** the flag on against off, on the 28 traced games. The figure is the section the rule touches. The whole round's change, paired by seed (mean ± SE over the 14 seeds), is in [prototypes.md](prototypes.md): once a rule changes a decision the games diverge, so the round figure carries that drift.
- **Est.:** estimated from the breakdown's counts.
- **Screen:** the judge on the dev seeds 36800-36831, d2 win-rate gap. It is indicative: 32 seeds give about ±10 points, and d4's 3% base is at the floor ([screen.md](screen.md), [screen2.md](screen2.md)).

| # | Cut | d2 min a round | d4 | Basis | Screen |
|---|---|---|---|---|---|
| 1 | Draft without passing, plus the tier split named once | **−3.0** | −2.7 | measured: `cards.no_pass` draft −1.60 (round −2.0 ± 0.7; d4 −1.52, round −1.6 ± 1.1). Est.: tier once, 7.6 → 2 decisions, −1.4 (d4 6.8 → 2, −1.2) | +6 ± 11 |
| 2 | Production from tracks, a "blocked" chip on a site | **−2.8** | −2.8 | est.: 17.8 site reads a round (d4 17.3), 3.0 minutes, become 2-3 chips placed and lifted (about 0.1) | — |
| 3 | A turn budget: 3 non-card turns a seat a round, cards never capped | **−2.6** | −1.7 | measured: `board.turn_cap=3` action phase −2.57 (round −1.9 ± 0.7; d4 −1.70, round −1.6 ± 0.9); cap 4: −1.65 / −0.73 | −19 ± 12 (cap 4 −19 ± 10) |
| 4 | An arrival's check folded into its line's site read | **−1.3** | −1.2 | est. ceiling: `map.arrival_checks` 7.6 a round (d4 7.2) × 10 s; a Spill's arrivals on unnamed hexes keep theirs | — |
| 5 | One price change a card play (a discount or a rebate, never both) | −0.6 | −0.5 | est., low confidence: half of the Builder's modeled 15 modifiers (r125's count) | — |
| 6 | Threats dealt by site, ignoring tags | −0.6 | −0.35 | measured: `pull_any_tag` (patch), dealing −0.63 (round −0.6 ± 0.7; d4 +1.0 ± 1.1) | −12 ± 11 |
| 7 | Growth printed: no choice of option | −0.5 | −0.5 | est.: one decision a seat; against 277's direction | — |
| 8 | Hosts fixed at the action phase's start | −0.4 | −0.3 | est., a recount on the patched traces: Buy energy checks 3.4 → 0.9 a round. The measured action phase is −0.13 (d4 −0.31) | −16 ± 9 |
| 9 | Compute and seeding as one move a project and a botnet | −0.4 | −0.6 | est.: half of compute's 17 components (d4 22) | — |
| 10 | Short standard menu (Dispatch, Liberate, Buy energy, station, ambassador) | 0 | 0 | measured: `std_off`, round +0.7 ± 0.6 (the judge spends the turns elsewhere) | +16 ± 8 |
| 11 | Jam floor (a jam never counters alone) | 0 | 0 | measured: `jam_floor=1`, round +0.5 ± 0.6; it removes no step | −12 ± 7 |

The eleven cuts in detail:

1. **Draft without passing; tier split once.**
   - **Sketch:** each player names the tier split for their hand at once ("two and two"), takes those 4 cards, and keeps what they pay insight for. Nothing is picked or passed.
   - **Strategy:** a player stops reading the partner's hand. They lose the co-op hand-off (passing the other seat a card it wants) and seeing the first card before choosing the next tier.
   - **Screen:** no detectable cost (+6 ± 11).
   - **Confound:** the judge's keep plan ran unchanged.
2. **Production from tracks.**
   - **Sketch:** each player's and the adversary's production sits on the income tracks (274's). It moves only when a site changes hands. An ambassador or agent arriving on a producing site places a "blocked" chip, lifted when it leaves.
   - **Strategy:** unchanged. Blocking still blocks, but the table pays for it when pieces move, not by re-reading every site at production. This is the kind of cut 274's income tracks were, scaled to production.
3. **A turn budget.**
   - **Sketch:** each seat takes at most 3 turns a round from Growth, powers, standard actions, building Actions and converters. Card plays are never capped (Matt's ruling), and neither are threat payments, Sell or the free counters.
   - **Strategy:** the seats now average 4.6 (Builder) and 4.8 (Protector) such turns. A good player ranks their board uses and spends 3 on the best; marginal Dispatches, converters and second powers are lost. This is the biggest strategy change here.
   - **Screen:** −19 ± 12 at cap 3, and also −19 at cap 4. The judge doesn't plan against a budget (a confound, not retuned), so part of that is the agent.
4. **An arrival's check folded into its line's site read.**
   - **Sketch:** when an Infiltrate, Surge or Grow line places agents on a site it names, the corruption check is made as they are placed, as one read of that site. Today the site is read to place, then checked again.
   - **Strategy:** none; this is a table procedure and a printed sentence. The −1.3 is a ceiling: a Spill's arrivals land on hexes no line named and keep their check.
5. **One price change a play.**
   - **Sketch:** a card play takes at most one discount or one rebate.
   - **Strategy:** the discount-stacking engines lose their combined peak.
   - **Confidence:** low. It rests on r125's hand count, and the Builder now plays about 2.5 cards a round.
6. **Threats dealt by site, any tag.**
   - **Sketch:** the top threat with a site in the country is dealt, whatever its tag.
   - **Strategy:** the event card's tags no longer steer which threats come, so holding counters for an expected tag is lost.
   - **Minutes:** 3.5 rejected cards a round go. The screen leans negative.
7. **Growth printed.**
   - **Sketch:** each board's Growth is one printed option.
   - **Strategy:** this undoes 277's three-way Growth, which Matt just asked for. It is listed only because Growth is 2.0 minutes a round.
8. **Hosts fixed at the action phase's start.**
   - **Sketch:** the round's hosts are matched once, as the action phase begins, and stand all round. A Buy energy reads the host tokens.
   - **Strategy:** a node placed mid-round no longer takes a city that round.
   - **Screen:** −16 ± 9, and the traced games ran longer (9.8 rounds against 8.0).
   - **Confound:** the agents' eval re-runs `match_hosts` on its own view.
9. **Compute and seeding as one move each.**
   - **Sketch:** the compute step moves one marker a project. A botnet's seeded agent goes on with the line's.
   - **Strategy:** none.
10. **Short standard menu.**
    - **Sketch:** Remove agents, Lift Fallout, cube to insight, Envoy, botnet, antimatter, Remove a troop and Build a factory go.
    - **Strategy:** the judge did better (+16 ± 8) with fewer options a turn, which the rates don't price.
    - **Minutes:** none, so this is a learning-load cut, not a time cut.
11. **Jam floor.**
    - **Sketch:** a jam never takes a threat's item below 1 (274's switch).
    - **Minutes:** it removes no step (the jam is still read), adds payments and costs the team. It is not a time cut.

**The combination.**
- **Measured:** the two kept flags together (`cards.no_pass` + `board.turn_cap=3`) cut **4.2 ± 0.7** minutes a d2 round (pooled −4.0) and 2.6 ± 0.9 at d4. Rounds a game go 8.0 → 8.4 and 6.4 → 6.1. The screen reads −12 ± 10.
- **With the top four's estimates added** (tier once −1.4, production −2.8, arrivals −1.3): about −9.7, so 42.4 → **32.7** a d2 round. At the traced 8 rounds that is **262 minutes** against 339 today, 2.2 × the 120-minute target.
- **The four prototypes** of the first pass together (no-pass, any tag, hosts fixed, short menu): −2.3 ± 0.5 at d2. At d4 they read −4.0 ± 0.9, but that is far more than the parts add to (about +0.1): game length moved (6.4 → 5.7 rounds), not the rules.

## 3. What I'd do first

1. **The draft (cut 1).** It is the largest cut with no detectable play cost. The draft is 9.4 minutes of a 42-minute round, mostly decisions a printed game can lose without losing the deck-building.
2. **Production from tracks with blocked chips (cut 2).** It saves about 2.8 minutes of re-reading every site with no strategy change. It only needs prototyping, as 274's income tracks did.
3. **Arrivals folded into their line (cut 4).** This is a sentence in the rulebook, worth up to 1.3 minutes, and changes nothing a player decides.
4. **Then decide the game's length, with the turn budget as the lever.** After 1-3 a d2 game is still about 4.7 hours (35.3 × 8 = 282 minutes) by these rates. The turn budget (cut 3) is the one measured cut that takes real decision time, but it costs the team at cap 3 and cap 4, and the judge isn't tuned for it. Getting to 2 hours needs about 4 rounds. That is a design call: fewer rounds, or a tighter turn budget with content rebalanced around it.

## Files

- [breakdown.py](breakdown.py): the breakdown, plus `--compare`.
- [traces.sh](traces.sh): the 28 traces, and the same games as an experiment for its per-round counters.
- [s278-trace.json](s278-trace.json): the base traces' spec.
- [s278-screen.json](s278-screen.json), [s278-screen2.json](s278-screen2.json) and [screen.py](screen.py): the screens.
  The first screen's `anytag`, `hostfix` and `top3` variants need the patches, each with its param (`"host_fixed": 0`
  in the board params, `"pull_any_tag": 0` in the adversary's) in `configs/baseline.json` and `three_seat.json`, then `regen.py`.
- [prototypes.md](prototypes.md): every `--compare`.
- The traces are kept at conductor's kept folder for 278 (`278/trace-r132-base-and-flags.tgz`, plus the screen's games).
