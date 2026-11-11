# Rulebook friction (rules revision 77)

*What is still hard to say in [rulebook.md](rulebook.md) (second draft), ranked by how much each costs a reader, with
the rule change that would fix it. Matt's test: a rule that can't be said well is a bad sign about the rule. Only what
**wording can't fix** stays here: the second draft rewords everything wording could (what it fixed is listed in
[experiments/results/173_rulebook_review/](../experiments/results/173_rulebook_review/README.md)). The table states
the engine; the per-area engine facts with file:line are in 172's and 173's working papers.*

**How it's ranked.** Cost is how often a player meets the rule (every round, most games, rarely) times how much it asks
of them: an exception, a forward reference, more than about three sentences to state, or a result a player wouldn't
predict from the board or card. **Evidence** is from 173's review: two cold readers (a general hobby gamer **G** and a
Spirit Island veteran **V**) read the first draft paragraph by paragraph and answered 25 rules questions each, checked
against the engine; a critical designer (**D**) rated every paragraph, and again on the second draft. "Flagged" means a
reader marked the paragraph confusing or patched at severity 2-3 (of 3); a question id (G19) means an answer that was
wrong against the engine. Fresh readers then answered the same questions from the second draft
([answer key](../experiments/results/173_rulebook_review/answer-key.md)).

**Length.** The second draft's Part 1 (sections 1-14, the learn-to-play rules, with six worked examples, without Sources
lines) runs about **7,350 words**, the whole page about **10,800** (172's word regex over the Markdown). Spirit
Island's base-game core rules are about 6,000-8,000 words (172's estimate from its page count, not a count). Part 1 is
inside that range with its examples, against the first draft's 7,975 core words with none; it still has no diagrams.
Its density is the sharper signal: at r73, 40 entries that only a rule or component change would fix, 17 of them in
rules met every round; r74 fixed six (five of those every round), leaving 34 and 12; r75 fixed nine more (three every
round), leaving 25 and 9; r76 fixed one (setup), leaving 24 and 9; r77 fixed thirteen (six every round), leaving 11
and 3; 179's icons fixed eight (none every round), leaving 3 and 3. The word counts are r73's.

## The worst left

r74 (175, Matt's rulings of 2026-11-10) fixed the adversary side: corruption (#1), Rally and blocked landings (#9),
the last node (#10), projects and compute (#11), what counts as a Muster (#12) and the node drain (#23) are gone from
the table. r75 (176, the same rulings) fixed the player side: ambassadors meeting agents (#3: every contact 1 for 1,
Remove spends its ambassador), Claim and Liberate (#4: one payment rule, Liberate makes the site yours, no botnet
block), factory growth (#5: next to the city, a neutral city's factory neutral), the protection order (#6: the owner
chooses, only your own ambassador, war's node guarded), production timing (#7: one income step; Buy energy in the
action phase), drones (#13: one ambassador stops one drone), holding Discord off (#14: no filter), tipping (#15:
both tracks keep their overflow) and the converters (#25: each once a round, a city of the country in reach); Aegis
Doctrine's half of #24 went with #6. r76 (177) fixed difficulty (#20): both incomes climb with the level, and level 4
is level 3 plus a short list (no removed Discord card, no extra threat). r77 (178, Matt's rulings) took the rest he
ruled: threats print payers and target with one token condition (#8); Expand, Muster, strength and "each country"
(#17); the Protector's landing and Vigil (#18); every trigger once per action (#19); "(including this)" on own-tag
gains (#21); Dispatch (#22); Shelter's "any country" (#24); one draft direction (#26); "on or next to, your choice"
(#27); Clear takes land corruption, which forbids building (#28); liberate cards liberate as the action does (#29);
firewall and cordon to the next adversary turn's end (#30); converts where they stood (#32). 179 (Matt's icon rulings,
no rules change) fixed the icons: an agent figure of its own (#33), the minus for pay or lose only with a less mark
for discounts (#34), a wall for firewall and cordon (#35), the track arrow below the art (#36), the shield one stop and
the roof more (#37), At N's ring (#38), the destination pin (#39) and the jam outside the frame (#40); node-track
spaces print their cost as a cost (#16's half). The rows keep their r73 numbers.

1. **Insight and keeps** (#2): one resource with a reset, a carve-out, a debt and three keep prices.
2. **Node tracks** (#16): two prices from two places.
3. **Event lines** (#31): an open list of line kinds on every event card.

## The table

| # | Rule | What makes it hard (evidence) | Proposed change |
|---|---|---|---|
| 2 | [Insight and keeps](rulebook.md#resources) (every round) | Reset each round, except Patience keeps your own but not gifted insight (G, V flagged: "track where each insight came from"); the board prints 5 but you have 6 from round 1 (V2 asked); less last round's losses, a debt across the round; credits stand in for keeps but not plays (V flagged); opening tier-2 keeps 1 insight, later 2, upgraded 1, Science-discounted only from round 2; the opening's 4 free tier-1 keeps, then 5 credits. | Print 6 on the board. Insight-loss tokens on your board, removed when insight is set. Patience keeps all unspent insight. **One keep price all game** (1 insight a card), credits never. |
| 16 | [Node tracks](rulebook.md#node-tracks) (every round) | Two prices from two places: the power's (or card's) cost plus the energy printed on the space uncovered. (179: each space now prints its cost as a cost, "pay N", so V's false familiarity, the numbers read as Spirit Island income, is gone; Matt kept the track names.) | The powers cost only their tag or currency and every placement pays the space, stated once on the track. |
| 31 | [Event lines](rulebook.md#the-turn) (every round) | An event card's other lines are an open list (income, a new factory, Discord, Unity taken back, a blackout, Border War's node loss, Forced March). | A fixed short list of line kinds, each with an icon. |

## Engine against docs/rules

r77 (178) brought `docs/rules/` to the engine and cleared every M-row: M1, M23, M27 and M39 by engine changes (a
rules change), the rest by fixing the docs. A new mismatch between the engine and `docs/rules/` goes here as an M-row
(doc says, engine does, where), ranked by how badly the doc misleads.

**Not rules for a player** (left out of the rulebook): the engine's 64-tip safety cap in one check, the round-1000
stop, `pull_forced`, and mechanisms no deck card prints (lowered income, slot-adding buildings, tags from hand,
place-token triggers, copy income, the chip-paid Harden action, building upkeep, Sabotage).
