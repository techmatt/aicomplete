# 180: rulebook cold-reader check after the friction rulings (r77)

No rules change: no engine, content or `docs/rules/` edit, no `RULES_REVISION` bump. The third draft of
[docs/rulebook.md](../../../docs/rulebook.md) and the rebuilt [friction table](../../../docs/rulebook-friction.md) come
from the review below, 173's method on the book as 175-179 left it (r77, 179's icons).

## The review

| Reader | Saw | Did |
|---|---|---|
| G, a general hobby gamer (cold) | the r77 second draft only | 166 paragraph notes; 28 questions |
| V, a Spirit Island veteran (cold) | the r77 second draft only | 158 paragraph notes with false familiarity; 28 questions |
| D, a critical designer | the draft, the friction table, `docs/rules/` | 171 paragraph ratings; structure; 14 friction rows proposed and ranked; 9 suspect facts |
| 4 verifiers | the engine | every G and V answer checked in the code (file:line) |
| a gap verifier | the engine | 25 questions the readers or D said the book left open ([answer key](answer-key.md), last table) |
| G2, V2 (fresh, cold) | the third draft before its last fixes | 19 questions each on what the first read flagged, graded against the engine |
| an edit auditor | the engine and `git diff` | every changed sentence that states a rule, told to refute it: 75 claims |
| G3 (fresh, cold) | the committed draft, less G3's own three fixes | 12 questions on the last fixes, graded against the engine |

Every answer, with the engine's answer and evidence: [answer-key.md](answer-key.md). Raw outputs: [raw/](raw/) (the
reader files keep every paragraph note; `edit_audit.json` the audit; `questions_*.json` the question texts).

## Results

| Draft | Readers | Right | Partly | Wrong |
|---|---|---|---|---|
| Second (r77, as 179 left it) | G, V: 56 | 54 | 2 | 0 |
| Third, before its last fixes | G2, V2: 38 | 38 | 0 | 0 |
| Committed, on the last fixes | G3: 12 | 12 | 0 | 0 |

- **The book answers what it's asked.** At r77 a cold reader gets 54 of 56 right against the engine (173's first
  draft: 43 of 50). Both partly answers were book faults: Vigil said "you can spend it this round" for every march,
  but a threat's march comes after the action phase (V12); §11 put the drone stopper only under the Discord tip, so
  an ambassador on a base seemed not to stop Warmongering's drone (V26).
- **The gaps were the finding.** The gap verifier found six statements wrong or misleading against the engine and
  twelve facts the book lacked. Wrong: Take down a botnet ignores reach, against §2's rule; solo keeps are paid like
  any keep ("you keep all 4" read as free); §19's "earliest in player order is spent" sat on the full-base clause,
  where nothing is spent; setup never shuffled the project deck; bases are placed by the layout, not printed on
  tiles; Shelter's "nothing can be destroyed" (it stops only what goes through the protection order). Lacking: an
  empty Rally type sends the pool to the supply; Buy energy is once a city for all players; a megaproject pays once;
  a full track tips again at once; agents and ambassadors never share a hex; a marker counts as an agent for
  corruption only; a troop mustered onto your factory strikes it that turn; and five more (answer key, gaps).
- **The edit audit** found 8 of 75 changed claims overstated or wrong (the Sway icon also prints Discord on one
  card; open land takes no cube; Warmongering's drone goes past a full base; a node hex's marker has §13's
  exceptions; ...). All were fixed before G3 read.
- **Severity.** Paragraphs a reader rated 2-3 (of 3): G 16, V 15 on the second draft; G2 15, V2 4, G3 3 on the third.
  None rated 3. What G2 and G3 still flag is mostly what only a rule change fixes (the friction rows).
- **One question used a retired word:** V5 asked about a "Deploy" line, 178's old name for Muster. V read it as
  Muster, right; the grade stands.

## What the third draft changes (wording, structure)

- **Corruption** in §6: three short cases (a site, open land, a node hex), each with its own result; "place" and
  "target" where open land and node hexes aren't sites; the node number's interplay with a city and with Corrupt
  open land stated; agents on the adversary's own sites move; a marker counts for corruption only.
- **Ambassadors:** Claim and Liberate's price says its unit (ambassadors) in the table and the bullets; agents and
  ambassadors never share a hex; a troop landing does nothing to them until it marches; the march's ambassador step
  no longer says "ambassadors don't fight".
- **Wall and the Protector landing:** Wall says what it stops in one sentence; the landing is the Militarist's board
  rule (its content text), stated there, with Wall pointing to it.
- **Production:** "Upkeep" (paying, in most euros) is now the **Income** step everywhere; growth runs every round, a
  blocked city builds, a destroyed factory frees its zone; a megaproject completes once.
- **Difficulty:** one row a level with notes for each column: threat items (credits +4, others +1/+2, tags never),
  +1 troop on every Muster, 4 compute for each unpaid threat at the next compute step.
- **Smaller:** the round table's B and E rows; steps C and D are the players'; round 1's round start and Sway come
  first; sacrifice moved to §7 with the tags it needs; Effects "once per action, no limit a round"; the requirement
  plate sentence split; the tipping example; a full track tips again; drones' sources and stoppers; the hardening
  token on a shared hex; Shelter's scope; the round cap; location tokens' printed-order rule; the legend's income
  arrow, Sway, Peek and Bricks (not the Protector's Wall); the glossary alphabetized with Corrupt X, Discord, Unity,
  Hardening token, Rally X, Shelter and Target.
- **Length:** Part 1 7,790 → 7,992 words, the page 11,712 → 12,163 (172's regex). The new facts cost about 450 words;
  cuts of repeats D found (compute sources, the Muster paragraph, the site-table patch, the prohibition detail) paid
  part back.

**D's proposals not taken:** moving §13 before §10, megaprojects into §9 and the fronts into §11 (sound, but a
restructure for a separate pass); diagrams, a components list and a layout per player count (need art); cutting the
Sources lines and the header for print (reviewer apparatus Matt uses; drop them for print). D's rank-1 "insight and
keeps" and several rows are rule changes, now in the friction table.

## The friction table

Rebuilt from 3 rows to 17: 179 had left #2, #16 and #31; this read adds #41-#54 (14 rows, 9 met every round), ranked
by cost with D's ranking and the readers' flags as evidence. The top ten: insight and keeps (#2), the adversary's
twelve-step turn (#41), corruption's three aftermaths (#42), Rally's split (#43), what removes an ambassador (#44),
growth every round (#45), location tokens (#46), two tag vocabularies (#47), node tracks (#16), the Protector's
landing (#48). One engine-against-docs row: M1, `rules/board.md` says Station Seizure needs 2 agents and no ambassador
to flip a player's station; the engine takes it outright (only Station Coup checks).

## New-player summary

No rules changed. The book now says outright what a new player trips on: the Claim price is paid in ambassadors,
agents and ambassadors never share a hex, Take down a botnet is the one action that needs no reach, and solo you pay
for the cards you keep. Rules that still resist a short statement, and need a rule change rather than words: insight
and keeps, the adversary's twelve-step turn, corruption's three outcomes, and Rally's split.
