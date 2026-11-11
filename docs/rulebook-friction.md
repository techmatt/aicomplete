# Rulebook friction (rules revision 77)

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

**Length.** The third draft's Part 1 (sections 1-14, without Sources lines) runs **7,992 words**, the whole page
**12,163** (172's word regex over the Markdown); the r77 second draft was 7,790 and 11,712. Spirit Island's base-game
core rules are about 6,000-8,000 words (172's estimate, not a count), so Part 1 sits at the top of that range, still
with no diagrams. Its density: r73 had 40 rows only a rule or component change would fix, 17 met every round; r74-r77
and 179's icons cut that to 3, all every round. 180's fresh read of the r77 book found 14 more (9 every round), so 17
and 12, mostly in the adversary's turn: with the bigger faults gone, readers reach the next layer.

## The worst left

The rows before #41 keep their r73 numbers; what r74-r77 and 179 fixed is in their write-ups (175-179) and in git
history of this page. 180's rows are #41 on.

1. **Insight and keeps** (#2): one resource with a reset, a carve-out, a debt and three keep prices.
2. **The adversary's turn** (#41): twelve steps by hand every round.
3. **Three kinds of corruption** (#42): site, open land and node hex, each with its own aftermath and marker rule.
4. **Rally's split** (#43): divide, then a remainder order with two skips.
5. **What removes an ambassador** (#44): five fates, and the troop that removes one isn't used up.
6. **Growth every round** (#45): every city builds, a blocked one too.

## The table

| # | Rule | What makes it hard (evidence) | Proposed change |
|---|---|---|---|
| 2 | [Insight and keeps](rulebook.md#resources) (every round) | Reset each round, except Patience keeps your own but not gifted insight; the board prints 5 but you have 6 from round 1; less last round's losses, a debt across the round; credits stand in for keeps but not plays; opening tier-2 keeps 1 insight, later 2, upgraded 1; the opening's 4 free tier-1 keeps, then 5 credits (173: G, V flagged; 180: D 2/5 on the insight paragraph, ranks it first). | Print 6 on the board. Insight-loss tokens on your board, removed when insight is set. Patience keeps all unspent insight. **One keep price all game** (1 insight a card), credits never. |
| 41 | [The adversary's turn](rulebook.md#the-turn) (every round) | Twelve steps, several empty most rounds; ongoing projects act between Muster and Corrupt; step 12 brings compute and ends firewalls, cordons and prohibitions; step 9 is two rules (V flagged: "twelve steps to run by hand every round"; D 2/5). | Four beats printed as a strip on the adversary board: **Card** (threats, Rally, Muster, other lines), **Corrupt** (lines, then natural), **March**, **Income** (reveal, pools, compute; ongoing projects act here; temporary effects end). |
| 42 | [Corruption's aftermath](rulebook.md#corruption) (every round) | A site, open land and a node hex all "corrupt", at different numbers, with different results: a site leaves no marker, a node hex does, open land gets its own token; a marker counts as an agent on a city, for corruption only (G, V flagged the same paragraph; G's overall: "corruption was the hardest thing to learn"; V: "two near-identical events with opposite marker rules"; on the third draft G2, V2 and G3 still flagged it, G3 on open land with a node). | One aftermath for all: whatever corrupts gets one adversary mark (the cube on a site, a land-corruption token on open land and on a node hex), and markers stop counting as agents. Or: a node hex's loss leaves no marker, like a site's. |
| 43 | [Rally X](rulebook.md#the-turn) (every round) | Divide the pool evenly, then the remainder to the lowest-numbered sites that aren't the adversary's or firewalled, else the adversary's; a firewalled share stays in the supply; no site of the type, nothing lands (G, V flagged; both asked what an empty type does; G2 and V2 flagged it again). | Deal the pool round-robin: one agent onto each site of type X in hex order, skipping firewalled ones, until the pool is empty. |
| 44 | [What removes an ambassador](rulebook.md#ambassadors-meet-agents) (every round) | An agent cancels one 1 for 1; a troop removes one and **stays** (it can remove two in a turn); corruption removes all; a drone stop and a protection each spend one (D, rank 3; G flagged the march line, "ambassadors don't fight" while troops remove them). | One contact rule for every adversary piece: a troop meeting an ambassador removes it and is removed, 1 for 1, as an agent does. |
| 45 | [Growth](rulebook.md#9-production) (every round) | Every city builds a factory every round it has a free zone next to it, a blocked city too, though blocking stops everything else a site does; a corrupted city's goes by agents nearby (G, V flagged: "the map fills with factories", "'blocked or not' contradicts 'blocked sites make nothing'"; G2 flagged the corrupted city's zone choice). | A blocked city doesn't build (blocking means the site does nothing). Optionally, a city builds only while it has no factory next to it. |
| 46 | [Location tokens](rulebook.md#18-threat-details) (every round) | Count adversary pieces on and next to every candidate, then a tie-break, then printed order unless that leaves a token off, when the type with fewer places goes first (D, rank 5). | A token goes on the place of its type, not the adversary's, with the most agents on its own hex (tie: lowest number). |
| 47 | [Two tag vocabularies](rulebook.md#10-threats) (every round) | Threat tags (Military, Influence, Economic, Grid) translate to card tags (Security, Diplomacy, Industry, Power) through a table; only Cyber is shared (D, rank 6). | Print threats with the player-card tag names and icons. |
| 16 | [Node tracks](rulebook.md#node-tracks) (every round) | Two prices from two places: the power's (or card's) cost plus the space's. (179 printed each space's cost as a cost; 180's V: "the book defuses the biggest trap"; D rank 7.) | The powers cost only their tag or currency and every placement pays the space, stated once on the track. |
| 48 | [The Protector's landing](rulebook.md#6-the-adversarys-turn) (every round with the Protector) | A Muster unlike every other (skips players' factories, never a city), printed on the adversary's board but met as Wall's price (G, V flagged the Wall paragraph: "a downside buried inside a power called Wall"; D rank 8; G2 and G3 flagged the Wall paragraph again). | A normal Muster 1 in each country with the Protector's node, or cut it and let Vigil pay on any troop. |
| 31 | [Event lines](rulebook.md#the-turn) (every round) | An event card's other lines are an open list (income, a new factory, Discord, Unity taken back, a blackout, Border War's node loss, Forced March). | A fixed short list of line kinds, each with an icon. |
| 49 | [Compute onto projects](rulebook.md#17-projects) (every round) | Each point onto the cheapest project, a slot refilling mid-step and the rest going on; costs by player count; by printed cost, however full (D rank 10; V2 asked; G3: "a reader will expect it to finish the nearly full project"). | All compute onto the leftmost slot; when it resolves, slide left and refill from the right. |
| 50 | [Discord-tip drones](rulebook.md#19-drones-in-full) (most games) | Per neutral factory, per base, stopped by an agent on the factory, by each ambassador on the factory or the base, players choosing which base goes without, full bases spending nothing, player order among ambassadors (G, V flagged §19; V, then G2, flagged §11). | At a Discord tip, each base of a country with an unblocked neutral factory gains 1 drone; an ambassador on the base stops it. |
| 51 | [The protection menu](rulebook.md#the-protection-order) (most games) | Shelter automatic but only for what goes through the order; then a choice of four, one (a megacorp's drone) good only against troops; one token with two names (G flagged; D rank 11). | Drones never protect; protections are Shelter, a hardening token or your own ambassador; one name for the token. |
| 52 | [War takes the node](rulebook.md#11-war) (most games) | A drone's strike on a factory also destroys the node on its hex (D rank 13). | A drone destroys only the factory. |
| 53 | [Prohibitions](rulebook.md#the-cards) (a few times a game) | "The next" Rally, Muster or Corrupt; the condition read as each step runs; a Corrupt prohibition stops natural corruption and moves; a Muster one stops every Muster (V, then G2, flagged; D 2/5). | A prohibition places bricks tokens on the hexes that qualify when it resolves, lasting like a firewall. |
| 54 | [Take down a botnet](rulebook.md#standard-actions) (most games) | The one standard action that ignores reach: only your ambassador on the botnet counts (the engine never checks reach), against §2's "everything you target must be in your reach". | Require reach like every other action, or print "anywhere your ambassador stands" on the action. |

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
