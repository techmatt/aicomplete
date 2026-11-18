# Rulebook friction (as of rules revision 125)

*What the game costs a human at the table, ranked, with the change that would cut it. Matt's test: a rule that can't
be said well is a bad sign about the rule. Only what **wording can't fix** stays open here. 268 rebuilt this page
from a hand-play pass (a d2 game resolved in full from [rulebook.md](rulebook.md) and [player-aid.md](player-aid.md),
and a d4 spot check: the log and per-round counts are in `experiments/results/268_table_cost_audit/`).*

**How it's ranked.** Cost is how often a player meets the rule (every round, most games, rarely) times how much it
asks each time: a lookup, arithmetic on a printed number, something to remember with nothing on the table to show
it, a check run for every site or piece, components moved, or a result a player wouldn't predict. **Load** is how
many such items a player carries **at once** at the peak: easy rules that are each fine alone still add up.
**Fix** names the kind of change: **P** a rules change, **W** wording or layout, **A** the player aid, **C** a
component.

**The table cost at r125**, per round (two practised players, 3 s a component, 10 s a check, 20 s a lookup, 15 s a
decision):

| Level | Lookups | Modifiers | Memory (peak, both seats) | Per-site checks | Components | Decisions | Minutes |
|---|---|---|---|---|---|---|---|
| d2 (8 rounds) | 13-17 | 12-61 | 12-22 | 81-227 | 127-367 | 40-80 | 40-85 (a game: about 490) |
| d4 (rounds 1, 5, 10) | 14-24 | 17-44 | 10-12 | 101-250 | 205-560 | 49-87 | 45-103 (a game: over 12 hours) |
| Setup | 5-6 | 7-8 | 0 | 11-14 | 85-100 | 2-3 | 9-12 |

A game runs **6-8 hours at d2 and over 12 at d4**: the largest finding. Most of it is bookkeeping volume (checks that
find nothing, compute, agents, troops, failed city builds) and the Builder's late action phase, not rules held in mind.
At d4 the load held in mind is no higher than at d2; the volume is (compute 2-4×, troops up to 25 a round).

## The table

| # | Rule | Met | What it costs | Load | Fix | Smallest fix still open |
|---|---|---|---|---|---|---|
| 1 | Builder's late action phase ([§7](rulebook.md#7-cards), [§20](rulebook.md#20-powers-and-capstones)) | every round from round 4 | 9-16 plays a round, each through a rebate, a building discount, an insight discount and a tag count that moves as cards are played and sacrificed: about 20 modifiers a round, a third of table time | 6 tag counts + 3-4 discounts | **P**, **C** | No rebate stacking with a discount on one play (cards); a tag tally board (a dial a tag) |
| 2 | Once-a-round uses ([§5](rulebook.md#5-your-ai-board), [§8](rulebook.md#8-the-action-phase)) | every round | powers, building Actions, converters, cities bought, Dispatches, Vigil: nothing on the table shows which are spent | 12-22 (d2 round 6: Builder 12, Protector 10) | **A** done, **C** | 268: the aid's *Once a round* list and a spare cube on each one used. Open: a printed used-marker space on each power and slot |
| 3 | City growth and building ([§9](rulebook.md#9-production)) | every round | 6 cities × 8 rounds of grow tests; builds of 4 nested tests; 5 cities sat at 3 chips from round 5, the same failed search every round; 3 facilities built in 8 rounds | 1 | **W** done, **P** | 268: a city whose chips wait is skipped (chips turned over) until a facility next to it is built or destroyed or a hex within 2 frees. Open: a city that can't build loses its chips, or a blocked or corrupted city doesn't grow |
| 4 | Natural corruption and the node check ([§6](rulebook.md#6-the-adversarys-turn)) | every round | about 35 checks a round (13 sites, 13-21 node hexes); fired 0 times in the d2 game | 0 | **W** done | 268: "look only at hexes holding 2 or more agents" |
| 5 | Compute ([§17](rulebook.md#17-projects)) | every round | 12-29 compute onto projects a round at d2, 38-81 at d4; three sources at three times; both hand-players misread the running total | 0 | **W** done, **C** | 268: keep compute as a marker on a track, not cubes. Open: a compute track printed beside each project slot; at d4, compute in units of 5 (**P**) |
| 6 | Threat dealing and nearest ([§10](rulebook.md#10-threats), [§18](rulebook.md#18-threat-details)) | every round | 45 threat cards turned in a d2 game (10 in one round for 2 fits), 4-13 a round at d4; nearest node at any range for each threat; 7 threats paid at 0 by jams | 1-2 waiting or delayed threats | **P**, **A** | Skip a country with no place for the tier's deck; a jam never takes a threat below 1; hex distances on the map aid |
| 7 | The march ([§6](rulebook.md#the-march), Wall) | every round from mid-game | a shortest path around Wall hexes per troop; d4 round 10: 25 troops, 22 steps, 15 tie choices (13 identical) | 0 | **A** done, **P** | 268: the aid's *The march*. Open: troops on one hex move as one stack, one choice |
| 8 | Income each round ([§9](rulebook.md#9-production) step 5) | every round | 7-10 printed lines a player re-added every round | 0 | **C** | Income tracks, one marker a resource, moved when a space or card raises it |
| 9 | Corrupt numbers ([§6](rulebook.md#corruption), [§21](rulebook.md#21-difficulty)) | every round | up to three modifiers on one printed number (city 5 → 3 from level 1 → 2 on a "(1 fewer)" line → 3 with a Liberated marker) | 3 | **A** done | 268: the aid's *Corrupt numbers* at each level |
| 10 | Hosting ([§9](rulebook.md#9-production) step 2, Buy energy [§8](rulebook.md#standard-actions)) | every round | who hosts which city is a matching problem decided in production, but Buy energy in the action phase depends on it ("hosts or could host") | 2-3 host slots | **P** | Hosts chosen at the start of the action phase, a host token on each, standing all round |
| 11 | Unity's two tables ([§12](rulebook.md#tipping), [§14](rulebook.md#14-winning-and-losing)) | most games, late | strength steps at 3 and 6, purge steps at 3 and 7: two tables, two band edges, read at each tip | 2 | **A** done, **C** | 268: the aid's *Tip strips*, one row a tip. Open: a tip-count track per side with every mark printed |
| 12 | The Income beat at d4 ([§6](rulebook.md#the-turn-four-beats), [§17](rulebook.md#17-projects)) | most rounds at d3-d4 | round 5: three projects resolved, a Discord tip, four extra threats and a muster before anyone acts; the order spans §6, §12, §17 | 3-4 pending effects | **A** | An Income-beat flow card: six numbered steps, tips checked after each project |
| 13 | Projects ([§17](rulebook.md#17-projects)) | most rounds | "the X with the most agents on or next to it" counts seven hexes a candidate (Asset Seizure: 21 reads) | 0 | **W/C**, **P** | Two agent-count glyphs on project faces (hex and ring, hex alone); or count the hex only |
| 14 | Card faces | every round | four faces below the 7.5 pt floor (Popular Uprising 5.4, Positron Storage Ring 5.8, Retained Earnings 6.3, Particle Accelerator 6.3); long tier-2 texts (Liaison Corps, Power to the People, Open Source Uprising) | 0 | **W** | Shorter op wording for those four (a content pass with `npm run cards:fit`); split Liaison Corps' line |
| 15 | Event lines ([§6](rulebook.md#the-turn-four-beats)) | every round | event faces print no icon row; each line read in words | 0 | **C** | Draw the two missing glyphs (Blackout, Forced March) and give event faces an icon row |
| 16 | Rulebook length | every round | 13,700 words; 13-24 lookups a round, most in §6, §9, §10, §15, §17 | 0 | **A** done | 268: the aid now carries the round, once-a-round list, tip strips, corrupt numbers, the march, ambassador spends and growth |

**Errors the hand-players made**, each a rule hard enough to get wrong (fixed in words by 268 unless marked): a
liberate card on a site with agents (§8 against the aid's Liberate); Fortify hardening a neutral facility (§20); a
waiting threat forgotten in step F (§4); a sacrificed program's income raise (§7); a corrupted station's cube after
its megaproject completes (§6, open); a striking troop left on the map (§6, clear already); agents left on a site a
project corrupted (§6); a threat's Fallout hex (§6); Vigil for a troop that doesn't move (§5); where a canceled
ambassador goes (§8); the Botnets spread line read two ways (§6; the event cards' own text is open, **W**); Naval
Blockade's "cities make no energy" read as leaving a corrupted city's Discord on (its card text, open, **W**).

**Closed since r98:** #5 (what spends an ambassador: the aid's box), #2 by the aid (its track component stays open as
row 11). #1, #3 and #4 are rows 3, 13 and 15.

## What a per-rule review misses

The r98 review scored rules one at a time and missed §21's ten-column difficulty table for 26 revisions. Each cause
is a check to run on every refresh:

1. **Combined load.** Scoring each rule alone never sees several easy ones carried at once. *Check:* the concurrent
   load column; a hand-play's peak memory and modifiers a round, not only each rule's cost.
2. **Generated tables read as data.** A generated table looks solved because it is generated. *Check:* read every
   generated block in the rulebook and player aid as rules a player must apply, and count its mid-game entries.
3. **No refresh.** The review was last run at r98. *Check:* the trigger below; `design_check`'s `friction-stale`.
4. **One more dial.** Each balance item reached for one more dial, judging only its own. *Check:* the ladder's
   mid-game count (`design_check` `ladder`) and the principle "a dial is a rule" ([principles.md](principles.md)).
5. **Simulated playtesters.** Agents apply every modifier perfectly and for free; win rate and tokens moved are blind
   to lookups, arithmetic and memory. *Check:* a hand-play pass, played from the rulebook and aid only, every refresh;
   `design_check`'s `table-*` budgets between them.

## Refreshing

**Rerun this review every 5 rules revisions** (`table_cost.refresh_every` in `scripts/design_check.json`). The title's
"as of rules revision N" is the record: `design_check.py` warns `friction-stale` once `RULES_REVISION` is 5 or more
past it, so any rules item's warning check shows it. A refresh:

1. Trace a 2-player game at d2 and one at d4 (`aic-sim run configs/baseline.json --games 1 --seed S --agent
   strategist --set adversary.difficulty=2 --trace`) and split it with
   `experiments/results/268_table_cost_audit/trace_rounds.py TRACE OUTDIR`: `script.md` (what the table reveals and
   decides), `counts.json` (components, decisions, round-scoped memory) and `check.md` (each round's end state).
2. A helper resolves the game by hand from the rulebook and aid only, reading each round's check only after logging it
   ([the brief](../experiments/results/268_table_cost_audit/handplay-brief.md)). Its errors are findings.
3. Rank as above, rewrite this page and its title's revision, and reset the `table_cost` budgets if a rules change
   moved them on purpose.

Between refreshes, `design_check`'s `table-components`, `table-checks` and `table-memory` warn when a level's read
passes its budget or rises 10% on `--old`.

## Engine against docs/rules

A mismatch between the engine and `docs/rules/` goes here as an M-row (doc says, engine does, where), ranked by how
badly the doc misleads. None is open at r125: the two the d2 pass suspected (compute income, a project overflow)
were hand errors, the trace's running compute total read as a project's.

**Not rules for a player** (left out of the rulebook): the engine's 64-tip safety cap in one check, the round-1000
stop, `pull_forced`, the embargo state that would forbid a Claim (no content sets it; the threat named Embargo lands
agents and charges energy or cards), and mechanisms no deck card prints (lowered income, slot-adding buildings, tags
from hand, place-token triggers, copy income, building upkeep, Sabotage).
