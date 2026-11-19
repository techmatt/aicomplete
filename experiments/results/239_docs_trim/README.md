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


## What the trim cut

Seven helpers took a file set each (rules-ai, rules-cards, rules-adv, readmes, design, tools, reads); `aic-sim docs
--check` passed on the final tree, and every heading another file links to was kept. Across the set the cuts were
of four kinds:

- **Item history in prose:** "(r82, Matt)", "(227 cut ...)", "(184)" and the like, about 20 a rules file. Every
  **candidate** / **locked** marker and every cited ruling number stayed.
- **Duplicates replaced by pointers,** each target checked: tags as story (ai-powers.md), energy-cost and capstone
  pricing (card-pricing.md), house terms (rules/README.md, *Terms*), own tags (card-grammar.md, *Per tag*), the
  "for new players" paragraphs of adversary.md and war.md (rulebook.md), the agent table's restated defaults.
- **Content restatement:** card names listed where a count does (end-game cards, typed counters, At-N and feeder
  cards), and tables the generators already print.
- **Rewording** for length, with each meaning checked against the code (a building's per-tag income freezes at
  play, `tableau.rs`; Actions refresh each round, economy's round table).

A stale line found on the way was fixed: player-cards' "the cards config lists which cards are in the game (all
of them)" (cards.md shows 18 "not in deck"). The helpers' full notes, file by file, are this README at commit
74bd2f0 (`git show 74bd2f0:experiments/results/239_docs_trim/README.md`); 284 cut them to keep the write-up
under 15 KB.

## Left for Matt, and where each stands (284)

| # | Contradiction or question | Status |
|---|---|---|
| 1 | player-cards: "the three antimatter ones" while the capstone table lists four (Nuclear Option, Hostile Buyout, Root Access, Peacekeeping) | Fixed before 284: player-cards now says "the four antimatter ones 4-15 and 1 antimatter" (cards.json agrees) |
| 2 | Land lifts: player-cards "Diplomacy or Power", ai-powers' tag table Diplomacy only | 284, doc only: player-cards says "land lifts Diplomacy (three Power cards print one too)", matching `design_check.json`'s tag story (`lift_fallout` in Diplomacy's ops only). Adding it to Power's ops is Matt's call |
| 3 | adversary.md: "all four adversity sources" with three named | 284: named, threats, Discord cards, adversary events, projects (the `fallout` op is printed in all four files); op.rs's comment, which listed three, fixed too |
| 4 | `plan_am`: docs "built Actions only", the `forced_table!` row "powers and built Actions" | The docs line was already cut to a pointer; the code counts both (`am_makes`, keep_plan.rs: board powers whose tags the seat holds, one option a group, never a burn, plus each built card's Action). 284 fixed policy.rs's comment, which read as plants only |
| 5 | `sweep` and `planner:strategy=1`: experiments.md made `sweep` the verdict spot check and `planner:strategy=1` the ceiling | 284: experiments.md now matches agents.md and the code. The strategy players spot-check; the oracle sits above them; `sweep` (`planner:deals=1`) is a cheap profile for price screens, never a verdict |
| 6 | economy.md: "Lift Fallout alone removes corruption; corrupted sites have none" | 284: "the one standard action that removes Fallout; a corrupted site's is Liberate" |
| 7 | Open Market: alignment.md "used before credits", while credits never buy a keep | 239 took player-cards' wording ("used first"); open only if "before credits" meant a free play |
| 8 | Capstone ratio 1.0-1.35 (player-cards) against 1.0-1.5 (card-pricing) | player-cards' copy was cut; card-pricing's 1.0-1.5 is the one statement |
| 9 | speed.md's strategist label, `engine:strategy=1` (r73's tiers table) | Cut to "strategist (the judge)"; the table is r73 data and wants a re-time |
| 10 | `same-noplanner.json` cost, 1 min against 2 min | Timed in 239's last commit: 44 s on 6 threads at r108, both pages |
| 11 | e2e.mjs "needs the dev server" | Fixed in 239's last commit |
| 12 | card-guidelines.md over the 12 KB target | Left by Matt's answer (MESSAGE #10705); at 14.9 KB after 284 took CLAUDE.md's card checklists |
| 13 | The "for new players" paragraphs (rules files) | Kept, tightened; whether they stay is Matt's call |
| 14 | `card-pricing.md`'s heading "Rates known to be off at r36" | Left (a test reads a sibling heading form) |
| 15 | qa-findings D66 (restate card-pricing's cost range and principles' credits a game) | Still open |
| 16 | Two numberings of "Matt's ruling N" in code comments (r75's rulings, card rulings 1-12) | Left; only content.rs's "ruling 12" is the card one |
