# 239: docs trim

Every doc over 12,288 bytes (15 KB budget, 3 KB free) at the start, before and after. card-guidelines.md stays over by Matt's answer (MESSAGE #10705).

| Doc | Before | After | Free under 15 KB |
|---|---|---|---|
| docs/architecture.md | 14,589 | 12,245 | 3,115 |
| docs/rules/ai-boards.md | 14,434 | 12,207 | 3,153 |
| README.md | 14,396 | 12,277 | 3,083 |
| docs/rules/player-cards.md | 14,389 | 12,179 | 3,181 |
| docs/card-guidelines.md | 14,384 | 13,507 | 1,853 |
| docs/agents.md | 14,368 | 12,287 | 3,073 |
| web/README.md | 14,339 | 12,283 | 3,077 |
| docs/scripts.md | 14,334 | 12,252 | 3,108 |
| docs/experiments.md | 14,334 | 12,265 | 3,095 |
| docs/rules/adversary.md | 14,333 | 12,281 | 3,079 |
| docs/rules/card-grammar.md | 14,320 | 12,158 | 3,202 |
| docs/principles.md | 14,318 | 12,254 | 3,106 |
| docs/rules/alignment.md | 14,313 | 12,242 | 3,118 |
| docs/speed.md | 14,298 | 12,287 | 3,073 |
| docs/rules/war.md | 13,819 | 11,147 | 4,213 |
| docs/card-pricing.md | 13,501 | 12,148 | 3,212 |
| docs/rules/README.md | 13,476 | 12,241 | 3,119 |
| docs/simulator.md | 13,406 | 12,266 | 3,094 |
| docs/rules/ai-powers.md | 13,221 | 11,881 | 3,479 |
| docs/rules/economy.md | 12,959 | 12,091 | 3,269 |
| docs/qa-findings.md | 12,839 | 12,199 | 3,161 |
| docs/rules/tunables.md | 12,485 | 12,162 | 3,198 |

The helpers' notes follow, as written during the trim (one helper a file set). Where a later lead edit differs, the report's *Moved* section is current.

## Helper rules-ai

## rules-ai notes (239_docs_trim)

`aic-sim docs --check` passes (only budget warnings for web/README.md and docs/card-guidelines.md, not mine).

| File | Before | After |
|---|---|---|
| docs/rules/ai-boards.md | 14,434 | 12,207 |
| docs/rules/ai-powers.md | 13,221 | 11,881 |
| docs/rules/alignment.md | 14,313 | 12,242 |

### docs/rules/ai-boards.md

- Item history, about 20 refs: r82 x3, r89, r79/r84, r87, r91, r93, r95 x2, r97 x2, r100, r75, "199's `t4`",
  "Matt, 2026-11-08", "Matt, design doc rev 24", "(candidate; Matt)", "earlier steps are in rules-history.md",
  the screen-068 write-up link on the card-play track (a reasoning link, not evidence a reader needs).
- Dropped the Protector clause "r89: the adversary's Muster where you stand is gone": history. Wall's "no adversary
  troop is ever placed on a hex with your node" already covers it.
- Duplicates replaced with pointers or cut:
  - Round 1's free 2+2 cards in each board's new-player text: already in the README's generated new-player text.
  - Node powers' rider list: now points to ai-powers *Node powers' riders*.
  - "Nodes don't grow on their own" and Place a node's `node_credits` 6: already in the README new-player text and
    economy.md's standard-action table. Kept "twice a round".
  - The opening of Loops and caps restated economy.md *No limits*: now a pointer.
  - Buy energy's "none while an agent stands on it" and `city_energy_credits`: economy.md's Buy energy row.
- Restatements of generated tables:
  - Node-power costs in *One payment path* ("Buy In 6 credits, Accord 1 Diplomacy, Peer Review 1 research cube").
  - The insight totals (Builder to 10, the others to 9, 7 on the first space): node_track2.
  - Printed tags per board: the boards table.
  - In the new-player text, the full per-power lists (Mint/Procure/Fortify, Interdict/Debrief, Disrupt/Grant/Field
    Station/Audit/Meltdown) are cut to one summary line per board plus a pointer to ai-powers.md.
- Reworded:
  - The three "for new players" paragraphs are now one "**The boards, for new players.**" block in the third person,
    with the shared facts (starting cards, Dispatches, printed tag) stated once.
  - "Dispatch 1 ambassador for free (2 from your fourth economy space)" became "Dispatches 1 ambassador free each round
    (the Researcher 2; more from the fourth economy space)". The Researcher's "(up to 4)" is in the boards table.
  - The Researcher's "to stand guard on your stations" was cut: the Dispatches paragraph keeps "its own ambassador on
    one of its stations armors it".
  - Rewrote "**Insight prints here only ...**" to add "The economy track prints none". That line moved here from
    ai-powers.

### docs/rules/ai-powers.md

- Item history, about 25 refs:
  - r107 date, 206 x1, r83 "worth 2 before", r98 x3 (Interdict, the Stand Down merge with "9 powers to 7", the Overload
    rename), r91 x4 (including the Focus cut and the Fieldwork cut), r35, r93 x2, the amendment and message numbers
    (9513, 9565, 9548, 9247), 212's screen, "3 before", r100 x3, 227's Seize rename, r97.
  - r41 x2, r44 with Sample Repository, r75 with Matt's ruling 9, r79/r84/192 on the Vigil cap, r77-r78,
    r106/238/r98 in the shelter parenthetical, r82, r81 x2 with friction #51.
- "One target (r107; Matt, 2026-11-14)" became "One target (locked)". rules/README *Terms* (Reach) marks it locked.
- *Insight conversions*: the long restatement of the card-play track totals is now a pointer to ai-boards *The
  card-play track*. Heading "(candidate, r41)" became "(candidate)". Nothing links to that anchor (checked with
  git grep).
- Reworded: "so a troop parked by your node no longer pays every march" became "... doesn't pay every march".
  "Insight must bind (Matt, 2026-11-12: about 6 a round ... is the target)" became "(the target: about 6 a round to
  drafting and 2-4 to plays)".

### docs/rules/alignment.md

- Item history, about 20 refs: r55, 238, r75 x2 (Matt's ruling 7, and "a Discord tip emptied its track"), r90 date,
  "the table of every source is in 204's write-up", r67 x2, r68 x3 (including "r68's clearing, rejected"), r71 x3,
  158, r80/#53, r82, 138, 139, 156, 146, "Matt, 2026-11-08".
- Duplicates replaced with pointers or cut:
  - "Variant `off` moves no Sway": already in README *Off-variants*.
  - The Discord-tip drone arming is now "a Discord tip arms the bases by neutral factories ([war.md])". war.md states
    the full rule.
  - Prohibition mechanics (a token where the condition holds when the card resolves; Rally's share stays in the
    supply; the tokens come off with the firewalls): now points to the README house term *In the next adversary turn,
    no X*. Kept here: the three kinds (Rally, Muster, Corrupt), any Muster troop staying in the supply, a site not
    corrupting by any route, no agent moving in, and the end-of-turn removal for one made during an adversary turn.
  - Extra threats dealt after the counter phase wait a round: now points to threats.md *Executing*. Offensive's
    duplicate "it waits a round" is folded into that bullet.
  - "No escalation in play: Discord tips don't raise the difficulty": adversary.md says it (line 175).
  - "Cards marked (draft) are for Matt to keep or prune": cards.md, generated by cards_doc.py, says it.
  - "Holding Discord off ... so the team chooses between racing toward Unity and putting off the next Discord card":
    the generated game_end block says it.
- Restatements of generated blocks:
  - The tier_at tips (I on the 1st and 2nd, II on the 3rd to 5th, III from the 6th): game_end, pointed to.
  - "with 7 cards the 6th and 7th tips resolve strength III and the 8th loses": follows from tier_at and the 7-card list.
  - The purge levels ("at 0 almost everything; from 7 only the cities") and "The steps are a list: agents, then troops,
    then botnets (troops and botnets both at 7)": purge_ease.
  - The list of player strikes with their Discord amounts in *Sway by narrative*: card text.
  - The Discord cards' pressure list (more or tier-2 threats, creation, Corrupt lines, Fallout): game_end, pointed to.
  - "the board's drip and `blight_discord` (1) for each Fallout token": became "the table's two start-of-round rows".
  - "Thresholds: *Tunables*": dropped, since the params already point to tunables by the README convention.
- Reworded:
  - The Losing bullet list is now one paragraph, "(each **locked**)". Every loss and its timing is kept.
  - "Open Market's free cards are keeps ..., used before credits" became "used first ([player-cards.md])". See below.
  - "Offensive ... then a tier-2 threat (two from strength II)" became "then deals its tier-2 threats". The card faces
    give the counts per strength.

### For Matt

- **Open Market wording (stale or contradictory).** alignment.md said the free keeps are "used before credits", but
  credits never buy a keep (player-cards.md, ai-boards.md). player-cards.md says "An Open Market free keep is used
  first." I took player-cards' wording. If "before credits" meant something else (a free *play*?), the rule needs
  restating.
- **The new-player board paragraphs** were condensed: they no longer name every power. If Matt wants each board's
  power list spelled out for new players, it is a design call; the generated powers table in ai-powers.md has it.

## Helper rules-cards

## rules-cards helper notes (239_docs_trim)

`aic-sim docs --check` passes with these edits; its only output is budget warnings for web/README.md and
docs/card-guidelines.md, which are in other helpers' sets. All three files are LF. No heading was renamed or removed.

| File | Before | After |
|---|---|---|
| docs/rules/player-cards.md | 14,389 | 12,179 |
| docs/rules/card-grammar.md | 14,320 | 12,158 |
| docs/rules/economy.md | 12,959 | 12,091 |

### docs/rules/player-cards.md

- **Item history, about 20 refs:** (Matt; as in Terraforming Mars), (r82, Matt), (r82), (r94, Matt, candidate),
  (drafts; Matt, 2026-11-09), (r93, Matt), (227 cut the Protector's Recovery), (r98, Matt, 224 #6), (Matt) x3,
  (r90: ...), (r94) x4 in the capstone table and price line, (184), (Matt, as in TM), r70. Kept every
  **candidate**/**locked** marker and "Matt's ruling 12", which card-guidelines and card-pricing also cite.
- **Duplicates replaced by pointers (each target checked):**
  - Tags are story: the soft-warning and Magic sentence now points to ai-powers.md, *Tags are story, not rules*,
    and its tag table.
  - Energy-cost pricing ("about 4.5 an energy") now points to card-pricing.md, *Energy as a second cost*.
  - Capstone pricing ("1.0-1.35 at Matt's unit anchors", "~100 credits", "out of reach before mid-game") now points
    to card-pricing.md, *The liberate capstones*.
  - The Tags list of what counts now points to the README house term *On your cards*.
  - Own tags: "(including this)" now points to card-grammar.md, *Per tag*.
  - Liberate: "agents come off, your cube goes on" was cut; the README house term *Liberate* and ambassadors.md
    state it.
  - The "Effects needing choices" paragraph was cut: README *Players choose* covers it, and the card-grammar
    legend covers `≤`N.
  - The italic topic line under the title duplicated the README index; it now reads "Terms and conventions...".
  - The delay definition moved to card-grammar.md, *Jam* (Delay N threats), where the op is defined.
- **Content restatement, with counts kept:**
  - End-game card names became counts: four convert, two strike (one also hits troops), two firewall, one masses,
    one removes. That makes ten.
  - Typed-counter breakdown: "three Industry, two each Security, Diplomacy and Cyber".
  - The six At-N and six feeder card names were cut, and the three OR-Action cards named.
  - Cut "(not Positron Storage Ring)"; "three Power ones" already gives the count. Also cut the Autonomous
    Defense Grid and Market Stabilization Board names.
- **Moved in:** card-grammar's "card set" sentence (decks are baseline.json's list; content holds more, marked
  "not in deck"; each tag primary on 24-29). It replaces "The cards config lists which cards are in the game
  (baseline: all of them)", which was **stale**: cards.md shows 18 "not in deck" cards.
- **Reworded, check for meaning:**
  - "a building's income is per tag, 3 a tag, counted once when it is played as a program's is" became "3 a tag at
    its tag count when played (as a program's)". This matches `tableau.rs`: the count freezes at play, and the
    building pays every round.
  - "Intrusion Lab's 1 Operative instead fills its 'at 2' on the first removal of compute, or a feeder's Operative"
    became "(or, for Intrusion Lab, its first removal of compute or a feeder) fills the first threshold".
  - Opening: dropped "played later once their `needs` and costs are met"; *Playing* already says it.
  - Actions: dropped "refresh at the start of a round". "Once a round" implies it, and economy's round table says
    abilities refresh.
  - Energy costs: "the Builder's heavy machinery" became "heavy machinery".

### docs/rules/card-grammar.md

- **Item history:**
  - Item and revision refs cut: (r106) x3, (r107) x2, (238: ...) x2, (r70), (118), (Matt's ruling of 2026-11-04),
    "Matt 2026-11-14", "Matt, r48".
  - Kept "(Matt's ruling 6)", which card-guidelines cites.
  - Kept the reuse rationale ("a combo with a strong Action is fine; a card it breaks is redesigned").
- **Duplicates to pointers:**
  - **Shapes paragraph:** now points to card-guidelines.md, *Legend*. Every mark in it is in that table (checked
    row by row).
    - Kept here: the near mark (not in the legend) and "the text says what the icons say".
    - **Coordinate:** whoever trims card-guidelines must keep its *Legend*.
  - Count as power and Cash out now point to ops.md (generated). One line keeps the one-card rule.
  - "Like every discount, never below `discount_floor`" now points to player-cards *Tag discounts*.
  - Draw: "A played event ... never the discard" is cut; player-cards *Decks and zones* says it.
  - Projects: "the rest slide left and the right slot refills" now points to adversary.md.
  - Target: "(within 1 of any of your nodes)" is cut; it is the house term *In reach*.
  - "The `play` mark is a card you play" is cut; it is in the legend.
  - "Powers and standard actions target too; Disrupt removes in one country" is cut. README *Reach* and the
    ai-powers table say it.
- **Content examples cut:** Back-Channel Talks, Amnesty Decree, Summit Caucus/Liaison Corps, Reclamation Crew
  ("priced as a station flip"), Back-Channel Talks ("cheap"), Hardpoint Lockdown, Consulate Tower/Substation Hub,
  Simulation Cluster, the three fight-trigger card names, Predictive Modeling/Early Warning Drill, and the first
  Telemetry Archive example. The *When built* example stays.
- **Stale text fixed (no rule change; the doc now matches the code):**
  - Per-tag income: "counts again every round while it is active" became "pays every round while it is active, at
    the tag count frozen when it was played". This matches `cards/tableau.rs`, which always freezes the count (r98).
    The `tag_income_once` param is gone.
  - Botnets: the doc said a tier-2 card and Decommission were "the only way to remove one". adversary.md and the
    standard-actions table also list Take down a botnet. The bullet now says a tier-2 card removes the target
    botnet and points to adversary.md for the other routes.
- **Reworded:**
  - Teammate triggers: "what it removes is the owner's, so the owner's ... triggers fire for it, not the player's
    whose card set it off" became "the removal is the owner's, firing the owner's ... triggers (not the player's)".
  - "Your own plays don't fire them, except in a 1-player game" became "fire them only in a 1-player game".
  - Rush Build: "isn't offered when no building in hand would still be playable" became "isn't offered unless some
    building in hand would still be playable".

### docs/rules/economy.md

- **Item history:**
  - Cut "(231: once, none ongoing)"; it now reads "(once; none is ongoing)".
  - Cut "Matt," from No limits.
- **Duplicates to pointers:**
  - Insight maxima ("up to 10 (the Builder) or 9 ...") now point to ai-boards.md.
  - Board printed energy "(1, the Protector's 2)" is cut; the generated boards table in ai-boards shows it.
  - Credits spent: the standard-action list now reads "(table below)".
  - Round row 10-13: the draft steps now point to player-cards.
  - Round row 17: the city-growth details now point to war.md (checked: *City growth*).
  - The italic topic line now reads "Terms and conventions...".
- **Code restatement:** cut "The view publishes each action with its cost (`View.standard`)".
- **Content names to counts:**
  - Chip cards: "chip income on four, a one-time gain on two".
  - The plants, as rates: 1 for a cube, a chip, 2 energy, 2 insight or 8 credits; tier 1: 2 energy.
  - The majors: 14 tier-2 at 1, one at 2, two tier-1.
  - Consular Pouch and Peace Conference are cut.

### Left for Matt (not cut or changed)

1. **Capstone pricing contradiction:** player-cards said the capstones read "1.0-1.35 at the anchors" and
   card-pricing.md says "1.0-1.5". I cut player-cards' copy, so card-pricing is now the only statement. Check that
   1.0-1.5 is current.
2. **"The three antimatter ones 18-22 and 1 antimatter"** (player-cards): the capstone table lists four antimatter
   engines: Nuclear Option, Hostile Buyout, Root Access, Peacekeeping. Either "three" is stale or one isn't paid in
   antimatter. Left as written.
3. **Land lifts:** player-cards says land lifts lean "Diplomacy or Power". The ai-powers tag table lists land lifts
   under Diplomacy only. Options: drop "or Power" in player-cards, or add lifts to Power's row in ai-powers.
4. **economy.md, Lift Fallout:** "Lift Fallout ... alone removes corruption; corrupted sites have none (cards,
   Reclaim and Liberate answer them)". Liberate a site is itself a standard action, so "corrupted sites have none"
   reads wrongly. Left as is, because the wording is a design statement.
5. The new-player paragraphs ("Token cards, for new players"; "Cities' energy, for new players"; card-grammar's
   "For new players") partly repeat the rule text above them. I kept them, only tightened, and treated the genre as
   Matt's call.

## Helper rules-adv

## rules-adv notes (239_docs_trim)

`aic-sim docs --check` passes (only warnings are web/README.md and card-guidelines.md, not mine). All four files LF.

| File | Before | After |
|---|---|---|
| docs/rules/adversary.md | 14,333 | 12,281 |
| docs/rules/war.md | 13,819 | 11,131 |
| docs/rules/README.md | 13,476 | 12,241 |
| docs/rules/tunables.md | 12,485 | 12,259 |

### docs/rules/adversary.md

- Item history: 151, 175 x4, 231 x3, 238 x2, 227 x2, 142, r80, r83, r89, r92, r98 x3, "Matt", "Matt's ruling",
  "(Matt, 2026-11-08)". Every **locked** marker kept.
- Duplicates replaced by a pointer (each home read first):
  - The "For new players" turn paragraph -> `rulebook.md` §6 (it covers every beat, corruption and the march). Kept
    the one line not there: about one project a round lands; the pile, the card ahead and the pools are face up.
  - The phase code names on each beat (`adversary.event`, `threats.deal` ...) -> economy.md's round-phase table.
  - "Players lose when the Card beat finds no card ..." -> folded into beat 1 as "none left: a loss, alignment.md"
    (alignment.md, *losing*, states the round cap, the plain loss, locked, after Sway and tips).
  - Firewall and prohibition durations -> README *Terms* (generated house terms say both come off at the end of
    the next adversary turn). Beat 4 still says they come off.
  - Lift Fallout's cost and the Take down a botnet terms -> economy.md's standard action table.
  - Starting botnet's hex -> board.md (setup: the station with a botnet on it).
  - Factory troops' blockers (ambassador, Wall) -> war.md (*Output*, *Wall*).
  - "Claim and liberate costs are the same at every difficulty" kept, shortened ("Claim and liberate cost the same at
    every level"); ambassadors.md also says it.
- Restatements of generated tables or content: "base agent income 0 below level 4" (difficulty table);
  "only the stage-III cards muster, 1 each" (adversary table's Muster column); the "Agents grow by stage" list of
  cards (now "the event cards' +1 agent"; the cards print it); Botnets spread's wording (printed on the cards, kept
  "with no such site, none", which the cards don't print); the Night Raid bullet (its card text; war.md still says
  it lands agents for the 2-agent rule); `content/projects.json` link -> plain path; "(numbers candidates)" x2 (README:
  every number is a candidate).
- Dropped: "or more than two other lines" from the check.py note (beat 1 states "at most two"); "the viewer marks
  the projects it would fill" (client display, not a rule; it is not stated elsewhere in docs, so the lead may want it
  in web/README.md).
- Reworded: "the rulebook's §2 tie-break (lower country, then number)" -> "(ties: lower country, then number)".

### docs/rules/war.md

- Item history: r75 x3, r80, r81, r82, r83 x5, r89 x6, r98 x3, 135, 137 x2, 138, 139, 146 x3, 175 x2, 177, 231,
  238, "Matt" dates and ruling numbers, "friction #51". "(r98: the Unity a marker paid is gone, a candidate)" dropped:
  "for no reward" already says it. "(Orbital Strike, Total War: r89, one troop fewer each)" -> no history clause.
- Duplicates -> pointer: both "for new players" paragraphs (troops; sites, bases and war) -> `rulebook.md` §2, §6,
  §9, §11, which cover each fact (read). Kept the one-line moral (Discord feeds war and war feeds Discord). The
  dropped "What resisted this summary: extra drones ignore the cap" is the *Extra drones* rule above it.
- Nothing else changed in meaning; bold run-in names the rulebook cites (Every round, Factories, Bases, the march,
  Node destruction, Destruction) kept.

### docs/rules/README.md

- "rules revision 108" line kept verbatim (regen.py's regex and the docs test read it); the placeholder/candidate
  statement kept, its history trimmed: "(r71, Matt's direction locked)" -> "(the shape locked)"; "rewritten by 146 to
  Matt's threat card standard, locked" -> "to Matt's threat card standard, locked".
- Item history: r106, r107 x2, 206, 137, r75 (Reach "locked" kept).
- Duplicates: "Keep these files current with every rules change, each under 15 KB" (CLAUDE.md); "(`docs/principles.md`)"
  after the 8-10 round game; *Site claim* -> pointer to ambassadors.md (it states reach, embargo, botnet, own-first
  payment); the ladder's Liberate clause -> the generated *Liberate* term; "move up to N ... / Dispatch from anywhere"
  -> ambassadors.md (*Consumed or standing*) and card-grammar.md.
- Index list tightened (the files' scope lines, no file dropped). Adjacent and Next to X on one line.
- Off-variants: the "Two don't isolate their module" sentence folded into the alignment and cards entries, same facts.

### docs/rules/tunables.md

- All but the title is the generated table. Cut the italic header and the paragraph restating it; title is now
  "# Tunables (generated)". 12,259, under target but with no margin to give.
- For the lead (not my file set): the table's descriptions carry item numbers ("142", "206:", "175:", "190:",
  "r82", "(142; 1 before)", "(142; 2 before)" ...), about 35 of them. Trimming them where the parameter
  descriptions live in code (read by `tunables_block`, crates/aic-sim/src/docs/blocks.rs:184) would free ~250 bytes.

### For Matt

- Rules files' "for new players" paragraphs (adversary.md, war.md) now point at rulebook.md, which says each of their
  facts at more length. If those paragraphs are meant to stay as a separate implementer-facing summary, restore them
  (about 2.4 KB across the two files: war.md has room, adversary.md would need another cut).
- Not a contradiction but a vagueness kept as found: adversary.md's Fallout line says "All four adversity sources print
  it" and lists three (threats, projects, Scorched Earth); the fourth is presumably Discord cards in general.

## Helper readmes

## readmes helper notes (239_docs_trim)

Files: README.md, web/README.md, docs/speed.md. `aic-sim docs --check`: exit 0, no output, after all edits.

| file | before | after |
|---|---|---|
| README.md | 14,396 | 12,277 |
| web/README.md | 14,339 | 12,283 |
| docs/speed.md | 14,298 | 12,287 |

All three are under the 12,288 line but above the ~12,100 aim: little item history was left to cut, so the rest
came from wordiness and code restatements.

### README.md

- Item-history refs x3: "(a third draft, 180)", "232" (opt-level), "(177)" (no-fail-fast).
- Duplicates replaced by pointers (each home checked by reading it):
  - `crates/aic-core` API list (`Rules::new`, `legal_actions_into`, ...): pointer to architecture.md, which has it
    under *API*.
  - `drafts/adversary/` row: the `load_adversary.py` / `--check` detail went to a pointer to docs/scripts.md
    (`content/load_adversary.py [--check]`, and `regen.py` running it).
  - Standing-spec roles (gate-exploit, gate-c3, use, same...): names kept, roles cut (in principles.md and
    experiments.md); added a link to experiments.md on the row.
  - `no_liberate.json` / `no_tier2.json`: now "`regen.py` derives `experiments/configs/` from it" (named in
    CLAUDE.md and scripts.md). That folder holds exactly those two files.
  - *Viewer* and *Card viewer* sections cut. The dev command moved into the Layout `web/` row, and the card viewer
    is named as conductor's button with a pointer to web/README.md, which keeps the exact command, port, path and
    the "keep them" warning. Neither heading is linked from anywhere (checked with git grep).
- Code restatements cut: the `crates/aic-sim` file list (`cli.rs`, `lib.rs`/`main.rs`); the `modules/` dir
  names; the `content/` row lists only the files that carry a note; the deck list in *Card text for review*
  (`--list` prints it); the `scripts/` and `docs/` rows merged.
- Reworded (worth checking):
  - "Not generated but for §21's difficulty table" -> "hand-written from the engine but for §21's difficulty table".
  - "archive/: every spec not tied to the current revision, and those the binary refuses today" -> "specs not tied
    to the current revision or refused by the binary today".
  - "wmic and tasklist hang for minutes under load" -> "hang under load".
  - "Check processes with PowerShell Get-Process" -> "use PowerShell Get-Process".
  - The `timeout N cargo test` bullet merged into the Bash `timeout` bullet.
- Headings *Build and test*, *Rules changes* (bold label), *This machine* and *Card text for review* are kept,
  since other docs point at them. *Toolchain* is now a paragraph under the same heading.

### docs/speed.md

- Item-history refs about x25: 174 (x8), 103, 104, 105, 099, 141, 148, 131, 136 (x2), 147 (x2), 149, 162, 182.
  Also gone:
  - "before 174" numbers: the 1,290 s / 531 s read times; the tiers table's *before* column (8.94, 12.1, 2.43,
    54.3, 55.4, 171); "one 12-round game took 47 s of 436 s".
  - "a sixth of a long game" (HashMap), "a tenth" (memcmp, Fact copies), the 103/104/141 profile list.
  - The `aicprof62.exe` example, and per-level deltas and seeds of the `discards` 0 read (41701-41844; −1.4, −0.9,
    −2.1, −0.5, +0.2). These are in 174's write-up, which stays linked.
- Kept with a revision tag instead of an item: "at r73" on every profile share and on the tiers; "r54 made long by
  a config costs as r55 does".
- Duplicates replaced by pointers or cut:
  - `samply` UAC: the README's *This machine* is the home; speed.md keeps "no admin, unlike samply".
  - `sweep` = `planner:deals=1,lead=0`: agents.md is the home.
  - The table's agent descriptions cut, since agents.md defines them. See Matt items.
- Reworded (worth checking):
  - "a test suite elsewhere on the box halved a whole table" -> "load can halve a table".
  - "Rejected:" grouping in *Approximations*: same three findings, same numbers.
  - "None adopted ... (rows in screen-069, on an older revision)" -> "(an older revision: screen-069)". The
    "older planner" caveat is kept on `prune=3`/`alts=1`.

### web/README.md

- Code restatements cut where the script header says it (checked each header):
  - e2e env vars (`AIC_URL` default, `AIC_BROWSER`, `AIC_SHOTS` file names) -> "atop it".
  - `e2e:corps` / `e2e:researcher` settings (station cubes 5/4, troop steps, round 20) -> "settings atop their
    scripts" (e2e-corps.mjs and e2e-researcher.mjs set them).
  - Sweep `AIC_SIM` default and `AIC_SWEEP_OUT` -> "atop the script".
  - The r99 `COUNTER_EVENTS_R99` event list -> "`COUNTER_EVENTS` and its per-revision sets", which the script
    comments by revision.
  - `odd.mjs` scenario list and fail conditions -> "listed atop it".
  - `card-fit.json` field list -> "fields atop `scripts/cards-fit.mjs`" (the 7.5-to-6 pt target box and the
    no-height note are kept).
  - Card-notes JSON field list -> "fields: `saveNote` in `cards-server.mjs`" (flag values, `text` and `hash`
    meaning kept).
  - `src/` file list cut to `main.ts`, `engine.ts`, `session.ts`.
- UI restatements cut (visible on the page):
  - Board colors (pennants, red fronts, purple).
  - The per-button action list (now "the seat's legal actions, the view's `standard` ones included").
  - Tab names, and "cards by name" in Choices.
- Duplicate e2e lines cut from the *Build and run* code block (the *Browser check* section covers them).
  `npm run dev -- --port N` folded into the dev line.
- Sections merged: *Kept, never played*, *Odd input* and *Source* -> "Other checks and the source". No other doc
  links those headings (git grep). *The sweep* and *Card viewer* are kept: qa-findings.md, sweep.mjs and
  cards-server.mjs point at them.
- Reworded (worth checking):
  - e2e "dev server must be running" -> "against a served viewer (the dev server, or *Against a built site*)".
    The script header still says "Needs the dev server running", but the README already used built sites for it.
  - Sweep "records from an aic-sim built after the served wasm load with a content-hash warning" -> "Don't rebuild
    aic-sim mid-sweep: its records load with a content-hash warning".

### Left for Matt

1. **The strategist's label disagrees between speed.md and agents.md.** speed.md's tiers table described the
   strategist as "`engine:strategy=1` plus its weights". agents.md says the strategist is greedy's flat weights
   plus `w.last_node` and the purge term, with no search and no strategy. I cut the label to "strategist (the
   judge)" so the table no longer states either. The numbers are r73's, so the old label may have been right for
   r73. Options:
   - (a) Leave it as is.
   - (b) Restate what the r73 strategist was.
   - (c) Re-measure the tiers on the current revision.
2. **The `same-noplanner.json` cost disagrees.** speed.md says "about 1 min"; experiments.md says "~2 min a side",
   which may mean both binaries, or may be stale. Not my file; flag for whoever owns experiments.md.
3. **The tiers table is r73 data, far behind the current revision** (the cost a decision has surely moved). It is
   still labeled "at r73", so it isn't wrong, but the next speed item should refresh it. That is a measurement,
   not a trim.
4. **The e2e.mjs header says it "Needs the dev server running"**, while web/README.md says a built site works too.
   A one-line script-comment fix for a later item.

## Helper design

## Helper "design": notes

`aic-sim docs --check` exits 0 on the final tree. `scripts/design_check.py`'s `anchors()` still parses the
card-guidelines anchor table (same values). All three files are LF. Every heading another file links to is kept
(*The op schema*, *Generated docs*, *The generic view* anchor, *Design warnings*, *Dominance checks*, *Event lines*,
*Legend*, *Unit anchors*, *Scarcity*).

| File | Before | After |
|---|---|---|
| docs/architecture.md | 14,589 | 12,284 |
| docs/principles.md | 14,318 | 12,254 |
| docs/card-guidelines.md | 14,384 | 13,507 (over target: see below) |

### docs/architecture.md (14,589 -> 12,284)

- **Item history removed (x5):** "(Matt, 229)" -> "(Matt)"; "(223 panicked on it)"; "every op has an arm, 191";
  "191:" before the harm-row note; "(Matt's ruling 12)" -> pointer to card-guidelines.md item 15 (same rule).
- **Duplicates replaced by pointers:**
  - `## Crates` section dropped (README *Layout* lists every crate; web/README says the viewer draws the generic view
    and offers only `legal_actions`). The intro now says `aic-core` is the rules' only home and points at *Layout*.
  - Config's *Overrides* bullet (`<module>=off` fails if required; the `off` variant keeps phases and params) dropped:
    simulator.md's `--set` row says both.
  - Generated docs: what `cards_doc.py` / `load_adversary.py` write -> pointer to scripts.md (which lists both).
  - The effect-ledger mirror test -> pointer to card-pricing.md (which names `effect_ledger_mirrors_the_table`).
  - AI boards: "so a board's sheet spans both files" dropped (rules/ai-boards.md says the same).
  - "Every op and where it prints: ops.md" dropped (ops.md is listed under Generated docs).
- **Restating code cut:** the RNG stream keying (now "(`rng.rs`)", whose module doc says it); why seeds are decimal
  strings; the `ViewOut` kinds list (derived values, card groups, piles, `board`, standard actions, arrows); "the
  `Module` hooks are documented on the trait"; "An undeclared key stays a global" (follows from the sentence before).
- **Reworded (check for meaning):**
  - Choices: three bullets folded into one paragraph; "given or forced" kept.
  - The round: "as at a table where the team talks it through" -> "as at a table".
  - Fixed-width draws merged into the RNG bullet.
  - `Records` paragraph merged into Known gaps' "records are the durable format".
  - "`docs --check` fails on ... a heading like `## 080's changes`" -> "a history heading (`current_docs.rs`)";
    "an id in `docs/card-pricing-notes.json`" -> "a pricing-note id"; "a doc page (CLAUDE.md's list) over 15 KB" ->
    "a page over 15 KB".
  - The new-op checklist: "The compiler finds the Rust sites ... but not `Op::contexts` or the `values.json` row"
    (dropped the `(op.rs)` after `Op::contexts`).

### docs/principles.md (14,318 -> 12,254)

No principle, band, threshold or number dropped or softened.

- **Item and revision history removed:** "(read at r33)", "(r59)" x2, "(231)", "(measured from r34)", "(157; ...)",
  "Current: 215's write-up (r96). Earlier ladders are in the recalibration write-ups (168, 177, 178, 185, 192);
  d0 resisted every income dial (177, 192); 196 met it with two more event cards at d0, which 200 removed (Matt:
  engines over dials)" -> "d0 resisted every income dial: its band comes from the players' side (engines over
  dials)"; "(gate-075)"; "the strategist since the ruling's `sweep`" -> "the strategist".
- **Duplicates -> pointers:** where unspent resources are read (report.md, results.json, `screen.py --gate`) ->
  experiments.md; the city-corruption counter note -> experiments.md *Measures* owns it (only the
  `liberation_read.py` pointer stays); the gate's seeds and early stopping (48 paired, within 2 SE to 96,
  `never_tag` dropped) -> `experiments/gate.json`'s `note` and agent list, which say it; the Design warnings
  "never stops work" sentence -> CLAUDE.md *Warning checks*, which says it word for word.
- **Reworded (check for meaning):**
  - Ballpark: "rotating the board pairs (Builder + Protector, Builder + Researcher, Protector + Researcher)" ->
    "rotating the three board pairs"; "Simulation is a ballpark check:" dropped (the heading says it).
  - Calibration point: "the band where a gate can tell..." -> "where a gate can tell..."; "A 48-seed band read is
    ±7 and a paired gap ±8-10, so read a band over 96 seeds" -> "Read a band over 96 seeds: at 48 it is ±7, a
    paired gap ±8-10."
  - Per-level bands: "where a level can't meet both, its 8-10 rounds win over its band" -> "a level that can't
    meet both keeps its 8-10 rounds over its band".
  - Adversary compute: "the big ones at 3-4 countries go past 50 (a few a game, as Matt asked)" -> "so a few a game
    pass 50 (as Matt asked)".
  - Information: "Where determinization or hidden-information handling gets in the way, it may be dropped" ->
    "hidden-information handling that gets in the way may be dropped".
  - Design lenses: the ten-line list is one line, same ten lenses.
  - The gate: "(d4 is not winnable, so a d4 half could not pass)" -> "(d4 is not winnable)".

### docs/card-guidelines.md (14,384 -> 13,507)

No guideline, ruling, example card, anchor or number dropped.

- **Duplicates -> pointer:** Legend rows already stated in rules/card-grammar.md (locked) -> one pointer line to
  card-grammar *Where it sits*, *Boxes*, *Connectors*. Rows removed: No box, Brown box, Dashed outline, Thin solid
  outline, `→`, OR at a line's start, `/`, Requirement plates. Kept rows with detail card-grammar lacks (Minus's
  "1 included", Dark frame's "Remove N, the jam outside", `:`'s less mark).
  **Lead: please check this against the rules helper's card-grammar.md edit.** It now sends *Shapes* to this
  Legend, and this Legend now sends boxes and connectors to card-grammar. Every row is in one file or the other on
  the current tree, but neither helper should cut its half further.
- Event lines: the list of line kinds (Rally ... Corrupt) dropped; `content::EVENT_LINE_KINDS` is the list.
- Item 9: box-size paths (`art/frames/r3/fit.json`, `web/card/card.js`) -> pointer to web/README *Text sizes*
  (tools/card-art/README covers `fit.json`).
- House wording: no longer names rules/README *Terms*. HOUSE_TERMS and player-aid.md are still named; player-aid
  says where it is generated from.
- "(Matt; rules/economy.md...)" in item 7 -> "(rules/economy.md...)"; "(rule 15)" -> "(15)".
- The rest is tightened wording.

### Left for Matt (asked: aicomplete__239_docs_trim__1, design; carrying on with the default)

card-guidelines.md stops at 13.5 KB (1.8 KB free), over the 12,288 target. The rest is Matt's rulings 1-25 with
their example cards, the anchor table and derived prices (`design_check.py` parses the table), and the Legend
rows no other doc states. Getting about 1.2 KB more takes one of these:
1. leave it at 13.5 KB (the default I took);
2. move the Legend into rules/card-grammar.md (another helper's file, and that rules page would grow);
3. shorten the rulings' example cards (*Surplus Auction*, *Blackout Script* ...), which softens them.

Noticed, not changed:
- `scripts/design_check.json` line 1 note and `scripts/design_check.py` keep item numbers ("205:", "206:"); outside
  my set.
- crates/aic-core/src/content.rs and several board modules cite "Matt's ruling N", but they use two different
  numberings (the r75 rulings and card rulings 1-12). Only content.rs's "ruling 12" is the card one.

## Helper tools

## tools helper notes (239_docs_trim)

`aic-sim docs --check`: clean for my files (only warnings: web/README.md, docs/card-guidelines.md, not mine).
No headings renamed. Generated `strategies` block in agents.md untouched. LF verified (`git ls-files --eol`).

### docs/agents.md: 14,368 -> 12,224

- Item history / dates: "(Matt, 2026-11-09)", "(Matt, 2026-11-13; ...)", "(Matt 2026-11-05)", "(Matt, 2026-11-03)",
  "(Matt, 2026-11-08)", "237", "229", "r98's tie stays". D24 / D53 / D71 kept (code comments cite them: grep aid).
- Restatements of code -> pointer:
  - Agent behaviors paragraph (move/strike/energy/kept/sell/am_save/make_short rules, ~1 KB) -> the behavior names
    plus "`behavior_table!` in `agents/policy.rs`, each row stating its rule". The table's own descriptions are the
    current text (the doc's list also lacked `target_rule`, `burn_late`, `am_last`, now named).
  - Keep plan: `plan_short`, `plan_cover`, `plan_am` descriptions -> "its switches ... are `forced_table!` rows in
    `agents/policy.rs`, each with its default and rule". 15% and `plan_rounds` kept.
  - Trigger/threshold bullet -> pointer to values.json `when` / `threshold` rows' `per` (they state own_mix,
    mix_prior, own_fill, raised_rounds in full).
  - `w.botnet_reach` sentence dropped (eval.rs field doc says it; no other weight is listed there).
  - Strategy-guide insight/energy/per-tag/trigger bullets shortened to the saying + fields (eval.rs field docs say
    the rule word for word).
  - "(the strategist's 120, 0 elsewhere)" -> "(the strategist only)": 120 is in `STRATEGIST`.
  - Planner row: "decisions inside it are scored on a guess" dropped (planner.rs module doc).
- Tiers table: dropped the "Tier" column (restated the specs) and merged the two "none" rows.
- Duplicate: "No-op weights: `scripts/tune/noop_weights.py`" -> "no-op weights: scripts.md" (its flags live there).
- Reworded for checking: "`pick_cost` 0: the keep plan judges what a seat can pay" -> "less `pick_cost` (0: the keep
  plan judges what a seat can pay) of its play cost".

### docs/scripts.md: 14,334 -> 12,252

- Item history: "(222)", "(226)", "(222)" in noop_weights.
- Every flag kept. Output descriptions shortened where the script's docstring states them in full (board_read,
  liberation_read, economy_read, corp_read, use_read, runaway_read, source_read, design_check, apply_overlay,
  ladder `--measures`); intro now says "A reader's docstring says in full what it prints".
- The seven flag-light per-cell readers (threat, slots, board, liberation, economy, energy, corp) merged into one
  bullet; gate_stop_rate and gate_close into one "Gates are suspended" bullet; no_liberate / no_tier2 into one.
- Eight "Seconds." -> one intro line "the `cards/*_read.py` readers take seconds" (energy/corp/threat/slots_read
  were unmarked before; they are the same kind of reader, but unmeasured by me).
- apply_overlay: the drop-a-card recipe -> "how to drop a card: its docstring" (docstring has it verbatim).
- per_decision: "(across revisions `--games 16`+, the agents a change touched)" -> "(`--games 16`+, speed.md;
  `--agent`, `--old-agent` the agents a change touched)": speed.md states the 16-game need.
- run_sim.ps1 "(its header says more)" and farm/compare's "(simulator.md, *The farm*)" dropped.

### docs/simulator.md: 13,406 -> 12,215

- Item history: 160 x2, 174, 178 x2, 163 x2, B5, 167, 212, 162's cost162.py, 177, 238, 213.
- Farm costs kept as current facts (1 s a strategist game, 12 s a read seed, wasm ~1.5x, laptop 2-3 cores, ~60 min
  for a 3,000-game screen); the 178 example timings (2,160 games in ~9 min; 303 s farmed / 9 min old side) dropped.
- Merged the two "Access is denied" passages (job outlives `farm-seeds`; `farm-seeds`' own build) into one bullet.
- "An arm with cards out of the deck" bullet folded into *A farmed spec*.
- Duplicate: *A ladder against the old revision* -> its first clause plus "experiments.md, *A before/after read*"
  (experiments.md has the `git archive` / own `CARGO_TARGET_DIR` / own configs recipe; verified).
- `screens.py assemble` full synopsis -> "(scripts.md)" (its flags' home).
- Output rows list folded into one paragraph; `--save*` rows tightened ("so A and B never collide", "likely a typo",
  "so a cap never truncates silently" rationales dropped; every behavior kept).
- `strategies` example: dropped the explicit `--agent strategist:strategy=1` (it is the default, said in the comment).

### For Matt / the lead

- **Stale in experiments.md (not my file):** line ~119 calls the strategist "the alias for `engine:strategy=1` plus
  its weights"; `STRATEGIST` in agents/mod.rs sets `strategy=0` (agents.md and scripts/tune/README say strategy=0).
  Fix: "`engine` plus `STRATEGIST`'s weights (`strategy=0`)".
- **plan_am wording drift:** agents.md said "built Actions ... (no burn or Reclaim)"; the `forced_table!` row says
  "the seat's powers and built Actions". I cut the doc copy to a pointer, so the code row now stands alone; if
  powers should not count, the row (or code) is what needs checking.
- Kept, though cuttable as code restatement, pending a view: the agent table's budget defaults (in `AgentSpec`)
  and the `PEAKS` order list in simulator.md.

## Helper reads

## Helper "reads": notes (239_docs_trim)

| File | Before | After |
|---|---|---|
| docs/experiments.md | 14,334 | 12,253 |
| docs/card-pricing.md | 13,501 | 12,148 |
| docs/qa-findings.md | 12,839 | 12,199 |

`aic-sim docs --check`: no failure names these files. `scripts/bench/seeds.py next 1` still parses the ledger
(`Next free: 47424` kept verbatim). No heading renamed.

### docs/experiments.md

- **Duplicates -> pointers**
  - Farming: what `farm-seeds` builds, commit-first, old side local, reading out files / `screens.py assemble`
    -> simulator.md *The farm* (verified it states each).
  - `shards.sh` arguments -> scripts.md (cost line "768 games in about 3.5 min on 6 threads" kept).
  - "Copy the binary, then rebuild freely" -> README *This machine* (running exe can't be replaced).
  - Greedy first after a change to how the game is won -> agents.md *The tiers* (the calibration-stop clause kept).
  - `--quick`, `--report-every`, `--resume`, `--winprob N`, `--closeness N`, `--variants`: their behavior -> the
    `aic-sim` help text (cli.rs). Kept what help lacks: the deep-search list for `--quick`, `--resume` refusing a
    change, `.bak`, waiting on `^time:`, probe costs.
- **Restatements of code / output**
  - Skill gap, survival, top agent, early lock-in, variants against base: report.md prints each definition
    (markdown.rs 291, 559; findings.rs). Kept *Ordered*, *Beats greedy* and "a module switched off reads as its worth".
  - Paired reads and Fresh-seed confirmation pairing rules: printed under their report headings.
  - A game that panics: the *Failed games* report text says failed.jsonl, no measure counts it, `--resume` replays.
  - Standing measures list -> `ladder.py`'s docstring (kept the `adversary.projects_revealed` semantics, which it lacks).
  - `games.csv` column list -> "its header" (kept: `adversary.corruptions` is any site, `.city` cities alone).
  - `skilled_rows.py` row contents, `band.json` / `sanity-line.json` / `same-noplanner.json` purposes -> their
    docstring / `note`s. Kept the costs (20 min on 3 threads, ~2 min a side) and the `ends156.py` win-mix rule.
  - `seeds.py` `--take` mechanics -> its docstring.
  - Saved records: "fewest pieces over the bar" (records.rs) dropped; categories kept.
  - `tune/sweep.py` / `log_table.py` description -> their docstrings.
  - Duplicate "Gates are suspended." (the spec table row says it).
- **Item history**: "236:" before the 1,152-game farm cost (number kept: it is a run cost, this page's topic, and
  not in 236's write-up). `227's density_specs.py` kept (only route to the example).
- **Fixed a stale statement (please check):** the strategist was described as "`engine:strategy=1` plus its weights"
  and cards.csv as naming it `engine:strategy=1,w.built=4,…`; `STRATEGIST` in agents/mod.rs is now `strategy=0,...`
  (agents.md agrees: no strategy). Reworded to "(`STRATEGIST` in `agents/mod.rs`)" and "the strategist as its
  `engine:` spec", asserting no number.
- Moved within the page: "A record's `cell` is an index, variant-major" to *Reading noise*; the density-screen line
  into *Content levers*; the old side's native commands into *A before/after read is paired*.

### docs/card-pricing.md

- **Item history** x~30: (204), (100), (102), (111, ...), (112), (128), (191), (108) x4, (r46) x2, (r94, 213), (214),
  (r48), (106), (107) x2, (r43) x2, (r45) x2, (125), (r35) x3, (r36) x2, (r50), (r62) x2, (130), (r90, 204),
  211's/213's on `am_save`/`am_source`, (090). History clauses cut: "r83 doubled both and halved every chip number",
  "5 at r88-r89 for Annihilate", "the probes pricing them at the table are retired", "Exascale Training Cluster's
  (insight and research cubes) was cut", the 076 card-verdicts link. "Matt" attributions on rulings kept.
- **Past-revision number already in its write-up -> cut:** "seats hold about 1-2 energy at rounds 3 and 6 and 1.8-2.6
  at the end" is in `experiments/results/screen-090/balance.md`; the line now points there.
- **Duplicates -> pointers:** anchor-check ceilings 1.5 / 2.5, "maximums, not a target band", "under 1.0 too dear",
  "one over its maximum is repriced or reported", "never cut toward the maximum" -> card-guidelines.md rulings 22 and
  *The anchor check* (verified). The Sway tip carry-over rule -> rules/alignment.md (verified line 16; the old bare
  "alignment.md" is now a working link). How agents read triggers (`rates.own_mix`, `own_fill`) -> agents.md
  *The value table* (verified).
- **Restatements of content:** the six energy Actions' card names and part of the card-throughput card names (the
  mechanism sentences and two examples per kind kept) -> cards.md.
- Reworded: "an income, of credits, energy or anything, counts" -> "any income counts".

### docs/qa-findings.md

- **Duplicates -> pointer:** the sweep's checks (150 ms spikes, `raw-id`, `clipped`, `truncated-no-hover`,
  `console`, `token-unknown`, `mark-over-number`, `warn-at-start`, `no-control`, `stale-word`, `COUNTER_EVENTS_*`)
  -> web/README.md *The sweep* (verified it lists each). "Rules prose quoting a card: docs --check now catches it"
  dropped: the Checkers list already says so.
- **Item history** x~25 in *Checkers*, *Where hunts found faults* and the tables: (235) x8, (099) x4, (118) x4,
  (145) x3, (169) x2, (184), (102) x2, (r40), 157, 204's, "Recovery was cut at r99 (227)" on D84.
  Reworded: "(235: 25 statements five revisions behind)" -> "(whole sections revisions behind)"; "r74-r77 changed
  the code and the rules" -> "one changed the code and the rules".
- Kept: the intro's hunt index (the route to each hunt's fixed.md / deferred.md), "169's agent_moves_fix.diff",
  "184's deferred.md", "199: 0.01-0.04 uses", r62 / r68 qualifiers on measured numbers, "Matt's ruling (238)".

### For Matt / the lead

- **experiments.md vs speed.md:** `same-noplanner.json` is "~2 min a side" here, "about 1 min" in speed.md, and its
  own `note` says 57-80 s a game at r69. Left both; one should be re-measured.
- **experiments.md vs agents.md:** experiments says `sweep` spot-checks a verdict and `planner:strategy=1` is the
  ceiling; agents.md *The tiers* lists `planner:strategy=1` as the spot check. Left as is.
- **experiments.md, *Costs (r73)* and *Skilled rows* (r73):** costs from a past revision, kept (this page's topic,
  not found in a write-up). A re-time would refresh them.
- **card-pricing.md heading "Rates known to be off at r36":** carries a revision; renaming it is outside a trim (a
  test, current_docs.rs, reads a sibling heading form). Left.
- **qa-findings D66** asks to restate card-pricing's cost range and principles' credits a game, "unmeasured at r60":
  still open; untouched.

