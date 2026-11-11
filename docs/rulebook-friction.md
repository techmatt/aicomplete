# Rulebook friction (rules revision 81)

*What is still hard to say in [rulebook.md](rulebook.md) (third draft), ranked by how much each costs a reader, with
the rule change that would fix it. Matt's test: a rule that can't be said well is a bad sign about the rule. Only what
**wording can't fix** stays here: the third draft rewords everything wording could (what it fixed is listed in
[experiments/results/180_rulebook_check/](../experiments/results/180_rulebook_check/README.md)). The table states the
engine; the engine facts with file:line are in 173's and 180's working papers.*

**How it's ranked.** Cost is how often a player meets the rule (every round, most games, rarely) times how much it asks
of them: an exception, a forward reference, more than about three sentences to state, or a result a player wouldn't
predict from the board or card. **Evidence** is from 180's review of the r77 book: two fresh cold readers (a general
hobby gamer **G** and a Spirit Island veteran **V**) read it paragraph by paragraph and answered 28 rules questions
each, checked against the engine; a critical designer (**D**) rated every paragraph and proposed rows; two more fresh
readers (**G2**, **V2**) read the third draft. "Flagged" means a reader marked the paragraph at severity 2-3 (of 3);
"D 2/5" is the designer's rating (5 = ready to print).

**Length.** At r81 Part 1 (sections 1-14, without Sources lines) runs **7,873 words**, the whole page **12,001**,
against r80's 7,969 and 12,128 and r79's 8,060 and 12,244 (187's `wc_part1.py` in its results folder; 172's regex had r79 at 7,992); the r77
second draft was 7,790 and 11,712. Spirit Island's base-game
core rules are about 6,000-8,000 words (172's estimate, not a count), so Part 1 sits at the top of that range, still
with no diagrams. Its density: r73 had 40 rows only a rule or component change would fix, 17 met every round; r74-r77
and 179's icons cut that to 3, all every round. 180's fresh read of the r77 book found 14 more (9 every round), so 17
and 12, mostly in the adversary's turn: with the bigger faults gone, readers reach the next layer.

## The worst left

The rows before #41 keep their r73 numbers; what r74-r77 and 179 fixed is in their write-ups (175-179) and in git
history of this page. 180's rows are #41 on; r80 (187, Matt's rulings, 2026-11-11) built #41-#43, #46, #48-#50,
#52 and #53, and r81 (188, Matt's rulings, 2026-11-11) #16, #47, #51 and #54
([rules-history.md](rules-history.md)).

1. **Insight and keeps** (#2): one resource with a reset, a carve-out, a debt and three keep prices.
2. **What removes an ambassador** (#44): five fates, and the troop that removes one isn't used up.
3. **Growth every round** (#45): every city builds, a blocked one too.

## The table

| # | Rule | What makes it hard (evidence) | Proposed change |
|---|---|---|---|
| 2 | [Insight and keeps](rulebook.md#resources) (every round) | Reset each round, except Patience keeps your own but not gifted insight; the board prints 5 but you have 6 from round 1; less last round's losses, a debt across the round; credits stand in for keeps but not plays; opening tier-2 keeps 1 insight, later 2, upgraded 1; the opening's 4 free tier-1 keeps, then 5 credits (173: G, V flagged; 180: D 2/5 on the insight paragraph, ranks it first). | Print 6 on the board. Insight-loss tokens on your board, removed when insight is set. Patience keeps all unspent insight. **One keep price all game** (1 insight a card), credits never. |
| 44 | [What removes an ambassador](rulebook.md#ambassadors-meet-agents) (every round) | An agent cancels one 1 for 1; a troop removes one and **stays** (it can remove two in a turn); corruption removes all; a drone stop and a protection each spend one (D, rank 3; G flagged the march line, "ambassadors don't fight" while troops remove them). | One contact rule for every adversary piece: a troop meeting an ambassador removes it and is removed, 1 for 1, as an agent does. |
| 45 | [Growth](rulebook.md#9-production) (every round) | Every city builds a factory every round it has a free zone next to it, a blocked city too, though blocking stops everything else a site does; a corrupted city's goes by agents nearby (G, V flagged: "the map fills with factories", "'blocked or not' contradicts 'blocked sites make nothing'"; G2 flagged the corrupted city's zone choice). | A blocked city doesn't build (blocking means the site does nothing). Optionally, a city builds only while it has no factory next to it. |
| 31 | [Event lines](rulebook.md#the-turn) (every round) | r80 closed the list (seven other-line kinds, `content::EVENT_LINE_KINDS`, each with an icon named), but event faces print no icon row yet, so a reader still reads each line's words. | Draw the two missing glyphs (Blackout, Forced March) and give event faces an icon row. |
| 56 | [Corruption](rulebook.md#corruption) (every round) | r80: one mark for all, but a node hex still needs its own clause: only the node goes (ambassadors stay), and land corruption goes on only if the node is lost (a protection or Sanctuary leaves none) (187's doc pass). | A corrupted node hex is open land that corrupts: everything on it goes, as on a site. |
| 57 | [Projects](rulebook.md#17-projects) (every round) | r80's slide-left: slot 1's Security tag limits only a card dealt into it, never one that slides in (187's doc pass). | Drop the slot tag, or refill at the tagged slot. |
| 55 | [Stations](rulebook.md#9-production) (every round with a corrupted station) | r79: a corrupted station has two outputs by its megaproject's state: an adversary cube onto the card until it completes, compute in the adversary's compute step (another phase) after; Vigil's once-a-round cap needs a marker per Protector (185, not yet read by a cold reader). | A corrupted station always makes an adversary cube: onto the megaproject while it is open, else onto the adversary's compute track as 1 compute in the same step. |

## Engine against docs/rules

A mismatch between the engine and `docs/rules/` goes here as an M-row (doc says, engine does, where), ranked by how
badly the doc misleads.

| # | Doc says | Engine does | Where |
|---|---|---|---|
| M1 | A threat that flips a player's station (Station Coup, **Station Seizure**) takes it only with 2 agents and no player ambassador on it. | Only the unforced flip (Station Coup) checks that; Station Seizure (`force`) corrupts the station outright, even a player's (146, D60). | `rules/board.md:84-88`; `effects/new_ops.rs:20-21, 312-330` |

**Not rules for a player** (left out of the rulebook): the engine's 64-tip safety cap in one check, the round-1000
stop, `pull_forced`, the embargo state that would forbid a Claim (no content sets it; the threat named Embargo lands
agents and charges energy or cards), and mechanisms no deck card prints (lowered income, slot-adding buildings, tags
from hand, place-token triggers, copy income, the chip-paid Harden action, building upkeep, Sabotage).
