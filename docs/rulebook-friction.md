# Rulebook friction (as of rules revision 98)

*What is still hard to say in [rulebook.md](rulebook.md), ranked by how much each costs a reader, with the change that
would fix it. Matt's test: a rule that can't be said well is a bad sign about the rule. Only what **wording can't fix**
stays here. The full r97 list, with evidence and the options weighed, is 224's working paper (kept, not in the repo);
r98 (item 223, Matt's rulings 2026-11-13) built its entries 1, 2, 6, 10 and 11, so they are gone from this page.*

**How it's ranked.** Cost is how often a player meets the rule (every round, most games, rarely) times how much it asks
of them: an exception, a forward reference, more than about three sentences to state, or a result a player wouldn't
predict from the board or card. **Fix** names the kind of change: **P** a rules change, **W** wording or layout,
**C** a component.

**Closed since r82:** #57 (project slot tag: r83, three open slots), #55 (a station's two outputs: kept by Matt's
ruling, board.md), M1 (Station Seizure is the unforced flip, which checks 2 agents), #56 (the node hex: r98, "2 agents
on your node's hex destroy it", no mark), and r98's one scar (Fallout only), Corrupt X's tie (an agent
next to two targets stays), Wall (no troop is ever placed on the Protector's node hex) and the Protector's powers
(9 to 7). #44 (ambassador fates) was mostly built by r83; what is left is row 5.

## The table

| # | Rule | What makes it hard | Proposed change |
|---|---|---|---|
| 1 | [City growth](rulebook.md#9-production) (every round) | Every city (up to 12) counts the unclaimed neutral factories beside it, takes that many chips (cap 3), and at 3 builds whichever of factory and station it has fewer of next to it, on a free hex next to it, else within 2 in its country, else waits; a blocked or corrupted city still grows. Bookkeeping for 12 cities a round, for an effect a player rarely steers (old #45; 224 #3). | Open: Matt is open to another design (a later item). Candidates: a blocked or corrupted city doesn't grow; a city takes 1 chip if any unclaimed neutral factory is beside it. **P** |
| 2 | [Unity tips](rulebook.md#12-unity-and-discord) (most games, late) | One tip count read against two tables: strength I/II/III at tips 1-2, 3-5, 6+, the purge easing at 3 and 7; a player who learned "6 is the big one" waits a tip too few (224 #7). | A tip-count track per side with every mark printed on it (strength steps, purge steps), each track's length printed per player count. **C** |
| 3 | [Projects](rulebook.md#17-projects) (every round) | "The X with the most agents on or next to it" counts seven hexes; "the node hex holding the most agents" counts one; both read at resolution (224 #12). | Two agent-count glyphs on project faces: hex and ring, hex alone. **W/C** |
| 4 | [Event lines](rulebook.md#the-turn-four-beats) (every round) | The other-line kinds are closed (`content::EVENT_LINE_KINDS`, each with an icon named), but event faces print no icon row yet, so each line is read in words (old #31; 224 #13). | Draw the two missing glyphs (Blackout, Forced March) and give event faces an icon row. **C** |
| 5 | [What spends an ambassador](rulebook.md#ambassadors-meet-agents) (every round) | Many short rules, each fine alone: an agent contact cancels 1 for 1 either way, corruption removes all on the hex, a base spends one per drone, armor spends one, Claim, Liberate, Remove agents and Envoy spend from the supply, blocking and Dispatch leave one standing (old #44; 224 #15). | One "what spends an ambassador" box on the player aid. **W** |

224's other open entries (the income tally, once-a-round bookkeeping, near against nearest, the insight that arrives
outside the action phase) were judged wording or player-aid work in its paper; the small wording fixes it listed went
to the planner's parked edits.

## Engine against docs/rules

A mismatch between the engine and `docs/rules/` goes here as an M-row (doc says, engine does, where), ranked by how
badly the doc misleads. None is open at r98.

**Not rules for a player** (left out of the rulebook): the engine's 64-tip safety cap in one check, the round-1000
stop, `pull_forced`, the embargo state that would forbid a Claim (no content sets it; the threat named Embargo lands
agents and charges energy or cards), and mechanisms no deck card prints (lowered income, slot-adding buildings, tags
from hand, place-token triggers, copy income, building upkeep, Sabotage).
