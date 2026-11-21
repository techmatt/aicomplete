# Rulebook friction (as of rules revision 150)

*What the game costs a human at the table, ranked, with the options that would cut it, for Matt to rule on. Matt's
test: a rule that can't be said well is a bad sign about the rule. Wording the hand-play could fix is already fixed in
[rulebook.md](rulebook.md) and [player-aid.md](player-aid.md); what stays here needs a rules, component or content
change. 309 rebuilt this page after the overhaul, from a hand-play pass: a d2 game resolved in full from the rulebook
and aid alone, and a d4 spot check of rounds 1, 3 and 5. The logs, errors and per-round counts are in
`experiments/results/309_rulebook_friction_refresh/`.*

**How it's ranked.** Cost is how often a player meets the rule (every round, most games, rarely) times how much it
asks each time: a lookup, arithmetic on a printed number, something to remember with nothing on the table to show
it, a check run for every site or piece, components moved, or a result a player wouldn't predict. **Load** is how
many such items a player carries **at once** at the peak. **Fix** names the kind of change: **P** a rules change,
**W** wording or layout, **A** the player aid, **C** a component or card face. **Ruling** names the
[rulings.md](rulings.md) line a row's options touch: a pick that crosses one is Matt's call before it is built.

**The table cost at r150**, per round, from the hand logs (two practised players: 3 s a component, 10 s a check, 20 s
a lookup, 15 s a decision, 5 s a modifier):

| Level | Lookups | Modifiers | Memory (peak, both seats) | Per-site checks | Components | Decisions | Minutes |
|---|---|---|---|---|---|---|---|
| d2 (7 rounds) | 11-16 | 30-42 | 4-6 | 51-75 | 92-165 | 31-44 | 29-39 (a game: about 250) |
| d4 (rounds 1, 3, 5) | 6-16 | 15-27 | 2-5 | 57-113 | 123-204 | 35-40 | 25-44 |
| Setup | 0-4 | 2-12 | 0 | 6-80 | 52-122 | 1-2 | 4-22 (d4's counted sorting the threat decks card by card) |

The traced yardstick (`scripts/bench/breakdown.py` on 268's seeds, judge games, the hand log's lookups and modifiers
modeled in): **24.3 minutes a round at d2 and 23.4 at d4; a game 175 and 166 minutes** (7.2 and 7.1 rounds). At r125
the hand play read 40-85 minutes a round and about 8 hours a game; 298's yardstick read 209 / 224 minutes at r143. The
overhaul cut the per-site checks by about 70% (no march, war, growth or site sweeps) and memory from 12-22 items to
4-6 (used markers, the tipped ambassador). What is left is **volume, not rules held in mind**: the adversary's Card
beat (about 26 checks a round, most finding nothing), the action phase (9.8 minutes a round, 40% of the table), and
at d4 the compute and threats that come twice as fast. The hand count runs a third above the yardstick: it counts
every production recount and threat-token test the trace leaves out.

## The table

| # | Rule | Met | What it costs | Load | Options (pick **bold**) | Ruling |
|---|---|---|---|---|---|---|
| 1 | Escalate and Commandeered Plants ([§6](rulebook.md#the-turn-two-beats)) | every round from round 2 | two "holds an agent" scans of the same sites, back to back: d2 about 85 checks a game for 4 hits; d4 29 checks in 3 rounds for none | 0 | **P: fold the board's rule into Escalate** ("factories always escalate, whatever last round's type"): one scan; P: Commandeered Plants fires only when last round's card didn't name factories; A: an aid line "scan for agents, not for sites" (no rules change) **Built (312, r151): factories escalate every round; the Militarist prints no rule.** | *Adversary and harms*, Spine ("Boards differ by ... one rule") and Card-driven ("checks on arrival, never a board sweep"): both lines are sweeps |
| 2 | Threat location tokens ([§10](rulebook.md#10-threats), [§18](rulebook.md#18-threat-details)) | every round | "the site of its type with the most agents, tie the lowest" for each type of each threat: up to 13 checks a round, 17 threats a d2 game, 23 in 6 d4 rounds | 0 | **P: the lowest-numbered site of each type not yet the adversary's**, as event, project and Discord lines pick; A: a placement order on the aid **Built (312, r151).** | *Adversary and harms*, Spine: "no 'most agents' picks" (293 took them off the event cards; threat tokens kept theirs) |
| 3 | The action phase's discounts ([§7](rulebook.md#playing-a-card)) | every round from round 4 | each tier-2 keep 2 modifiers (keep upgrade, then 1 off per 2 Science); the Industry discount a second reading of a dial nothing prompts (a d4 error); about 20 keep modifiers a game | 2-3 | **C: discount pips on the tag dials** (Science at 2, 4, 6; Industry at 3, 6, 9) so the dial shows the discount; A: a keep-cost row on the aid (3, −1 upgrade, −1 per 2 Science, floor 0); P: Science discounts plays only, not keeps **Built (312, r151): the aid's *Tracks and markers* and rulebook §2/§7 print the pips (display only).** | *Table time and the card economy*: discounts about TM's count (C and A keep them); the P option changes *Resources and engines*, Insight's keep prices |
| 4 | The compute step at d3-d4 ([§17](rulebook.md#17-projects)) | most rounds at d4 | 32-66 compute a round; two projects resolve in one step in rounds 3, 5 and 6, each a resolve, slide and refill with a remainder held in mind | 1 | **A: "move the remainder onto the next project's track before you slide"** (one line on the aid's round); P: at most one project resolves a step, the rest waits (a slower adversary at d4: a rules item to read); P: compute in units of 5 at d4 **Built (312, r151): the aid's round, step B.** | none (*Difficulty*'s bands if P) |
| 5 | Unity tip cards ([§12](rulebook.md#the-cards)) | 8 tips a d2 game | each tip a map-wide count; 5 of the 8 drawn did nothing (Neighborhood Watch, Peacekeepers, Decommission, Global Coalition, Civil Defense): read, counted, zero | 1 | **P (content): each Unity card prints a small floor when its condition counts 0** (say 2 Unity or 2 credits each player); C: one strength a row, the current tip's strength marked; W: shorter faces (three 150-200-character strengths) **Built (312, r151): every Unity tier prints "every player gains 2 credits" when it counts none (`floor`).** | *Unity, Discord and winning*: fear-card shape, board conditions (a floor keeps both) |
| 6 | Threats taking effect ([§10](rulebook.md#10-threats)) | every round; 5-6 a round at d4 | d4 rounds 3 and 5: 12-13 checks and 38-44 components; three on one hex lost an anchor agent (an error); 5 part-payments put on and handed back for nothing | 1-2 | **A: a two-step row a threat on the aid ("its harm, then 1 agent on its anchor")**; P: put resources on a threat only to complete a way (pooled in one go); C: an anchor mark on the first token **Built (312, r151): the aid's round, step F.** | *Adversary and harms*, Threats (locked): who may pay is unchanged by either |
| 7 | Production ([§9](rulebook.md#9-production)) | every round | "read it from the tracks" assumes few changes; with nodes placed every round the hand player recounted 5 cities × 2 seats and every station (14-19 checks), and missed a city across the border (a d4 error) | 0 | **A: the node powers' aid line says "move your energy marker for each free city next to the new node"**, so the track stays right as nodes go down; C: a city-energy pip by each city, taken when a node goes next to it **Built (312, r151): the aid's round, step E.** | *Resources and engines*, City energy ("on the energy income track"): the pick keeps the track; the C option adds a piece beside it |
| 8 | Corrupted factories and zone arrows ([§6](rulebook.md#one-output-a-corrupted-site)) | most rounds; 7 factories in a d4 round | two steps a factory (push along the arrow, then make 1 if empty); two zones (Horizon 11, Cintamani 14) touch two cities and the hand players had no arrow to read | 0 | **C: print each zone's arrow and "push, then make 1" on the zone** (the map's print; the arrow's tie is now in §2); P: a factory only pushes (no new agent while one is on it) **Built (312, r151): the aid's *Zone arrows*, rulebook §2; the web map already draws the arrows.** | *Adversary and harms*, Spine: "factory 1 agent ... pushed next round along its zone's printed arrow" |
| 9 | Ambassadors on the map ([§8](rulebook.md#ambassadors-one-action-a-round)) | every round | about 30 moves a game; which arrive ready (from the supply, placed, converted) took a lookup; 9 Infiltrate agents landed on parked ambassadors, so each landing site is a two-piece check | 1 (tipped pieces show the rest) | **A: one aid line "arrives ready: from your supply, placed by a card or power, converted"**; C: a colored base for a piece that arrived this round **Built (312, r151): the aid's *What spends an ambassador*.** | *Boards and pieces*, Ambassador actions: "It arrives ready (to try: say how it played...)". How it played: no hand-play error came from it, but which pieces arrive ready (supply, card, convert) took a lookup; arriving spent would end same-round acts from the supply (not measured here) |
| 10 | Once-a-round uses ([§5](rulebook.md#5-your-ai-board), the aid's *Once a round*) | every round | memory 4-6 at the peak (12-22 at r125): the used markers carry the rest | 4-6 | **Keep the used markers and amend the ruling**; remove them (back to memory: 268 counted 12-22 items) **Built (312): rulings.md keeps the markers.** | *Rulebook and audits*: "Rejected: ... used-markers (powers are implicitly once a round)", but 274's markers are in the rules, rulebook and aid |
| 11 | Card faces | every round | the hand-play's card errors: an "A, OR B" Action whose halves have no names (Safehouse Network), a building that triggers itself (Agency Directorate), Incursion II's extra threat with no tier or country, Open Market's "next 3 kept free" with no token | 0 | **W (content): name the halves, print "(this card counts)" on self-triggers, give Incursion II its tier and country**; C: tokens for lasting card effects **Built (312, r151): halves print "(A) ...; OR (B) ...", self-triggers in the aid's *Words on cards* and rulebook §7 (a face mark, even "(this too)", took token buildings below 7.5 pt), Incursion I-II their tier and country.** | none (a content pass with `npm run cards:fit`) |
| 12 | Level rules ([§21](rulebook.md#21-difficulty)) | every round at d1+ | 4 lookups in 3 d4 rounds (2 a city, War Footing, Arms Race) | 1 | **A: done (309): the aid's *Your level*** | — |
| 13 | Rulebook length | every round | 15,600 words; 100 lookups in a 7-round game, flat at 11-16 a round (each round brings a new kind of thing) | 0 | **A: a one-page adversary-turn checklist** (Card: threats, Escalate, the board's rule, Infiltrate, icons; Income: outputs, compute, research), expected to take about 5 lookups a round; W: cut Part 2's restatements of Part 1 (§15 of §6, §19 of §11) **Built (312): the rulebook's adversary-turn checklist.** | none |

**Errors the hand-players made**, each a rule hard enough to get wrong. Fixed in words by 309: a jammed-to-0 threat
is paid off (§10: a d4 error, a d2 lookup); "neither possible" pays from the supply and discards the hand (§10: the
d4 error was a miscounted hand, but the words read both ways); a contact counts
as removing agents for an Effect (§8); an arrow's tie (§2); War Rooms reads any tag (§3). Answered by 312's picks (r151), each a row
above, not yet re-played by hand: a self-triggering building and an unnamed "A, OR B" half (row 11); a city across the
border (row 7); an anchor agent lost among three threats (row 6); the Industry discount (row 3). One was the tooling's: the table script named
threats by id (orbital_strike for Weapons Test), so a threat looked unprinted (`trace_rounds.py` now prints both).

**Not met in the hand play:** troops never came at d2 (research stopped at slot 3); at d4 one Strike and one
Redeploy (3 troops × 3 cities × every player piece) read cleanly. Re-check them at the next refresh.

**Closed since r125:** the late action phase's tag counts (274's dials), once-a-round memory (274's used markers, row
10), city growth (275), natural corruption's node check (r140's arrival checks), threat dealing (r139's decks), the
march (r141), city energy (r139), Unity's two tables (r141's win), the Income beat's spread across sections, the
projects' seven-hex counts (297), compute volume at d2 (a compute track), income re-adding (income tracks).

## What a per-rule review misses

The r98 review scored rules one at a time and missed §21's ten-column difficulty table for 26 revisions. Each cause
is a check to run on every refresh:

1. **Combined load.** Scoring each rule alone never sees several easy ones carried at once. *Check:* the concurrent
   load column; a hand-play's peak memory and modifiers a round, not only each rule's cost.
2. **Generated tables read as data.** A generated table looks solved because it is generated. *Check:* read every
   generated block in the rulebook and player aid as rules a player must apply, and count its mid-game entries.
3. **No refresh.** *Check:* the trigger below; `design_check`'s `friction-stale`.
4. **One more dial.** Each balance item reached for one more dial, judging only its own. *Check:* the ladder's
   mid-game count (`design_check` `ladder`) and the principle "a dial is a rule" ([principles.md](principles.md)).
5. **Simulated playtesters.** Agents apply every modifier perfectly and for free; win rate and tokens moved are blind
   to lookups, arithmetic and memory. *Check:* a hand-play pass, played from the rulebook and aid only, every refresh;
   `design_check`'s `table-*` budgets between them.

## Refreshing

**Rerun this review every 5 rules revisions** (`table_cost.refresh_every` in `scripts/design_check.json`). The title's
"as of rules revision N" is the record: `design_check.py` warns `friction-stale` once `RULES_REVISION` is 5 or more
past it. A refresh:

1. **Audit the rulebook, teach and aid against `docs/rules/` and the engine first** (309's two audits found 28
   mismatches the revision line hid: `regen.py` stamps the revision whatever the text says).
2. Trace 268's seeds: `python experiments/results/309_rulebook_friction_refresh/traces.py OUT` (d2 and d4, 14 judge
   games each, 6 at a time, about 15 s, plus the same games as an experiment in `OUT/exp`), then split one game a
   level with `python experiments/results/268_table_cost_audit/trace_rounds.py OUT/d2-S.out DIR`: `script.md` (what
   the table reveals and decides, cards by id and name), `counts.json`, and `check.md` (each round's pieces, each
   seat's resources and the tracks).
3. A helper resolves the game by hand from the rulebook and aid only, reading each round's check only after logging
   it ([the brief](../experiments/results/309_rulebook_friction_refresh/handplay-brief.md): **update its step labels
   to the round first**; `breakdown.py` parses them). Its errors are findings.
4. `python scripts/bench/breakdown.py OUT --level 2` (and `--level 4`) prints every step's minutes at 268's rates,
   its lookups and modifiers modeled from the hand log (`--hand DIR`, default 309's; `--hand 268` for r125's);
   `breakdown.py BASE FLAG --compare` reads a flag's minutes against base, paired by seed.
5. `python experiments/results/309_rulebook_friction_refresh/table_cost_read.py GAMES.jsonl` prints the three
   `table_cost` proxies by level beside their budgets: reset a budget only where a rules change moved it on purpose.
6. Rank as above, rewrite this page and its title's revision.

Between refreshes, `design_check`'s `table-components`, `table-checks` and `table-memory` warn when a level's read
passes its budget (309's reset: r149's read + about 10%) or rises 10% on `--old`.

## Engine against docs/rules

A mismatch between the engine and `docs/rules/` goes here as an M-row (doc says, engine does, where), ranked by how
badly the doc misleads. **None is open at r150.** The d4 pass suspected one (Turncoats, "neither way possible", kept the
ambassadors): the trace shows the Builder held 3 cards, so the discard was possible and taken; with neither possible
the engine pays all of the resource from the supply and discards the hand (`threats/ops.rs`, `requisition`), as the
rules say.

**Not rules for a player** (left out of the rulebook): the engine's 64-tip safety cap in one check, the round-1000
stop, the embargo state that would forbid a Claim (no content sets it), and mechanisms no deck card prints (lowered
income, slot-adding buildings, tags from hand, place-token triggers, copy income, building upkeep, Sabotage).
