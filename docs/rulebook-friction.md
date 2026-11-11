# Rulebook friction (rules revision 75)

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
round), leaving 25 and 9. The word counts are r73's.

## The ten worst

r74 (175, Matt's rulings of 2026-11-10) fixed the adversary side: corruption (#1), Rally and blocked landings (#9),
the last node (#10), projects and compute (#11), what counts as a Deploy (#12) and the node drain (#23) are gone from
the table. r75 (176, the same rulings) fixed the player side: ambassadors meeting agents (#3: every contact 1 for 1,
Remove spends its ambassador), Claim and Liberate (#4: one payment rule, Liberate makes the site yours, no botnet
block), factory growth (#5: next to the city, a neutral city's factory neutral), the protection order (#6: the owner
chooses, only your own ambassador, war's node guarded), production timing (#7: one income step; Buy energy in the
action phase), corp troops (#13: one ambassador stops one troop), holding Discord off (#14: no filter), tipping (#15:
both tracks keep their overflow) and the converters (#25: each once a round, a city of the country in reach); Aegis
Doctrine's half of #24 went with #6. The rows keep their r73 numbers.

1. **Insight and keeps** (#2): one resource with a reset, a carve-out, a debt and three keep prices.
2. **Near and nearest** (#8): two distance tests on one threat, plus per-type token conditions.
3. **Node tracks** (#16): two prices from two places, printed numbers that read as income.
4. **Words with two meanings** (#17): Deploy, tier and board each name two things.
5. **Wall, Vigil and the Protector's troops** (#18): a landing printed on the wrong board, a counterfactual payout.
6. **Move** (#22): "movement" that takes an ambassador from anywhere on the map.
7. **Liberate cards** (#29): since r75 the opposite of the Liberate action: agents stay, no cube unless printed.
8. **Draft direction** (#26): it flips with the round's parity, and nothing on the map tracks it.
9. **Tags on your cards** (#21): covered, event, board and a card's own tags each count differently.
10. **Event lines** (#31): an open list of line kinds on every event card.

## The table

| # | Rule | What makes it hard (evidence) | Proposed change |
|---|---|---|---|
| 2 | [Insight and keeps](rulebook.md#resources) (every round) | Reset each round, except Patience keeps your own but not gifted insight (G, V flagged: "track where each insight came from"); the board prints 5 but you have 6 from round 1 (V2 asked); less last round's losses, a debt across the round; credits stand in for keeps but not plays (V flagged); opening tier-2 keeps 1 insight, later 2, upgraded 1, Science-discounted only from round 2; the opening's 4 free tier-1 keeps, then 5 credits. | Print 6 on the board. Insight-loss tokens on your board, removed when insight is set. Patience keeps all unspent insight. **One keep price all game** (1 insight a card), credits never. |
| 8 | [Near and nearest](rulebook.md#10-threats) (every round) | Two distance tests on one threat: **near** (a node or ambassador within 1, to pay) and **nearest** (a node only, any distance, to be hit). G flagged; V asked (V15). Threats dealt late wait a round; money on an unpaid threat is refunded at round's end even if it waits. Location tokens: cities and stations uncorrupted, factories not the adversary's (else a free zone), bases any, the most adversary pieces on or next to each candidate (counting up to seven hexes a candidate), the scarcer type placed first on a retry. | Name them apart on the card ("payers", "target"). One token condition for every type ("not the adversary's"), or one location type per threat. A "waits" marker on a late threat. |
| 16 | [Node tracks](rulebook.md#node-tracks) (every round) | Two prices from two places: the power's (or card's) cost plus the energy printed on the space uncovered. V's false familiarity (severity 3): Spirit Island track numbers are what you **gain**, so 0, 0, 0, 1, 2, 3, 5 reads as energy income; the "card-play track" reads as a card-play limit, which doesn't exist. | Print each space's cost as a cost (a minus, or "pay"), and the track's name for what it gives. Or the powers cost only their tag or currency and every placement pays the space, stated once on the track. |
| 17 | [Words with two meanings](rulebook.md#24-glossary) (every round) | **Deploy** is every board's node power and the adversary's troop line. **Tier** is a deck (1, 2) and a Unity or Discord card's strength (I-III), while event cards have **stages** I-III, and a stage-I card deals tier-1 threats. **Board** is an AI board and the adversary board, and on adversary cards a country ("each board"; G flagged). **Overrun** is a threat's name, so natural corruption can't borrow it. (The second draft already avoids "strike" for cards and uses the cards' "on or next to"; 172's draft had defined "next to" to include "on", against the printed cards.) | Rename: the node power "Expand", the adversary line "Muster"; Unity and Discord "strength I-III"; print "each country" on adversary cards. |
| 18 | [Wall, Vigil and the Protector's troops](rulebook.md#5-your-ai-board) (every round, 2+ players) | Wall stops only adversary troops; a factory on the Wall hex still makes its troop; a Deploy onto a Wall hex stays in the supply (r74). The adversary deploys 1 troop a round where the Protector is, on the lowest-numbered factory, which can be a player's (G: "my own factory gets a troop that strikes it"): printed on the adversary's board, not the Protector's. **Vigil** pays when a troop is about to step next to a node on a facility, a counterfactual (D 3 of 3; G, V flagged "fiddly"). | Print the landing on the Protector's board too, and land it on an adversary or neutral factory. Vigil: "when a troop ends its march next to your node, gain 1 insight". |
| 19 | [When an Effect fires](rulebook.md#connectors) (card text) | "When you remove a troop" fires once a troop; "when you remove agents" or compute once per card, Action, power or standard action that removed any. G and V both flagged it as a patch. A teammate's trigger that removes counts as its owner's. | All once per action, or all per piece, with the unit printed. |
| 20 | [Difficulty](rulebook.md#21-difficulty) (setup) | The incomes don't climb: compute 3, 4, 18, 24, then 13 at level 4; agents 1, 4, 4, 4, 3 (G, V flagged "non-monotonic, arbitrary"). Level 4 is a list of setup changes. | Smooth the curve, or explain level 4's trade (fewer pieces, harder deck) on the difficulty card. |
| 21 | [Tags on your cards](rulebook.md#tags) (every round) | Covered buildings' tags count; events' don't; board tags count but can't be sacrificed; a card's own tags don't count for its own requirement but do count for its own per-tag gain. | "A card's own tags never count for anything on that card"; board tags the one stated exception. |
| 22 | [Move](rulebook.md#standard-actions) (every round) | "Movement" reads as distance, but each point puts an ambassador from your supply **or anywhere on the map** onto a site, base, node or (r75) botnet in reach. G, V: "can I move onto an empty hex?" (G11, V14 right). | Rename it ("Dispatch"), or limit the source to your reach, so a move is a move. |
| 24 | [Shelter](rulebook.md#20-powers-and-capstones) (late game) | Shelter covers any country, with no reach, unlike every other power, and lasts from the action phase to cleanup, so it never meets the adversary's turn (G25, V8 asked); it is also the one protection spent automatically (r75). Its roof mark looks like Aegis Doctrine's shield. (Aegis's half of this row, spent automatically on anyone's piece, went at r75: the Protector chooses when, on their own pieces.) | Print "any country" on Shelter. |
| 26 | [Draft direction](rulebook.md#rounds-2-on-build-draft-keep) (every round) | The direction flips with the round's parity (next player in odd rounds, previous in even); nothing on the map tracks it (G: "which direction is next?"). Player order is set by board, so players must sit by it. | Fixed direction, or an arrow on the round track. |
| 27 | [Removing from around a hex](rulebook.md#16-moving-and-removing-pieces) (late game) | Removes only from the hexes **next to** the chosen hex, never the hex itself; troops first, in hex order, not the player's choice. V's false familiarity: Spirit Island targets the land itself. | "Remove up to N on or next to a hex in reach, your choice." |
| 28 | [Land corruption](rulebook.md#corruption) (stage III) | "Corrupt land" targets any open hex with no base or facility, including one holding your node, which it destroys. G and V: "how is land corruption removed?" Nothing removes it (no op, card or action): a permanent Discord source and a permanent no-node hex, unlike every other corruption. Yet it doesn't stop building: a city can grow a factory there and a player can build a station there. V's false familiarity: Spirit Island has many ways to remove blight. | Let Clear (or Liberate) remove it; land corruption forbids buildings too; or exclude hexes with a node and print "permanent" on the token. |
| 29 | [Liberate cards](rulebook.md#claim-and-liberate) (late game) | They liberate a city but leave the agents on it, so it starts blocked, and put a cube on only if the card prints align; since r75 the Liberate action removes the agents and puts your cube on, so the card and the action now differ in two ways (Matt, 2026-11-10: the cards stay as they are for now; M1). | Any liberation removes the agents on the site and makes it yours. Item 178 takes it. |
| 30 | [Firewall and cordon](rulebook.md#16-moving-and-removing-pieces) (late game) | A firewall comes off at the next adversary reveal, so it covers this round's or next round's turn depending on when it was placed; natural corruption ignores it. The cordon (Hardpoint Lockdown) lasts to the end of next round (a different span from a firewall's), stops Rally and card arrivals but not Gather or Push, and the cordoned country's pool is lost, not saved (G, V flagged the first draft's cordon, named only in its legend). | "Lasts until the end of the next adversary turn" printed on both tokens; a firewalled site doesn't corrupt; a cordoned pool waits. |
| 31 | [Event lines](rulebook.md#the-turn) (every round) | An event card's other lines are an open list (income, a new factory, Discord, Unity taken back, a blackout, Border War's node loss, Forced March). | A fixed short list of line kinds, each with an icon. |
| 32 | [Convert](rulebook.md#16-moving-and-removing-pieces) (late game) | Converted agents arrive on the node that gave you reach (your choice of node), not where they stood: a new ambassador appears away from the agent it replaced (G, V asked which node). | "Each converted agent becomes your ambassador where it stood." |
| 33 | [Icon: agents](rulebook.md#marks) | Agents print as the **ambassador figure** inside the dark frame: your piece and the enemy's share a figure (G flagged). Convert prints both side by side. | An agent figure of its own, matching the map piece. |
| 34 | [Icon: the minus](rulebook.md#marks) | Four meanings by position: pay or lose now, remove (dark frame), cheaper (after `:`), lower an income. Action costs left of `→` print no minus. | One rule for costs, and a distinct "less" mark for discounts. |
| 35 | [Icon: the clock](rulebook.md#special-marks) | Delay, firewall (agent + hex) and cordon (agent + country) share it. | A wall mark for firewall and cordon; the clock for delay only. |
| 36 | [Icon: the brown box](rulebook.md#marks) | Above the art, income each round; below the art, a one-time income raise, told apart by position and a plus. | A track arrow in the below-art box. |
| 37 | [Icon: the shield](rulebook.md#special-marks) | Three meanings by company (hardening, shield, Aegis), while the roof with a country is Shelter. Since r75 Aegis prints shield + node (your own pieces), the node shield's own mark, told apart only by sitting on a capstone. | Shield for one stop, roof for a whole country. |
| 38 | [Icon: the disc](rulebook.md#connectors) | Four meanings by the connector after it: add here, At N, pay tokens, one of your cards. | Teach At N with its own ring around the number. |
| 39 | [Icon: move lines](rulebook.md#16-moving-and-removing-pieces) | Liaison Corps (move up to N ambassadors onto pieces in reach) and Summit Caucus (move up to N onto one piece) print the same icon line. | A destination mark for the one-piece form. |
| 40 | [Icon: the dark frame's numbers](rulebook.md#marks) | A minus number in the frame is "remove up to N", but delay prints a bare count there and jam a minus meaning "less to pay". | The jam outside the frame. |

## Engine against docs/rules

Mismatches found while writing and reviewing, each confirmed in the engine (file:line in 172's and 173's working
papers). The rulebook states the engine; a row goes when an item brings `docs/rules/` to the engine (r75 settled M7,
M11, M21, M22, M29, M34 and M35). Ranked by how badly the doc misleads.

| # | Doc says | Engine does | Where |
|---|---|---|---|
| M1 | A liberate card works like the Liberate action (the docs say so no longer, r75: ambassadors.md and README *The ladder* now state the difference) | It takes the cube off and sways Unity but **leaves the agents** on the city, which is blocked, and puts a cube on only with align; the Liberate action cancels them and puts your cube on. Matt (2026-11-10): the cards stay for now; **item 178 takes it** | `effects/removal.rs` (`liberate_city`) against `board/ambassadors.rs` (`liberate_site`) |
| M48 | Peer Review "costs 3 research cubes and no energy" (ai-boards.md:36) | It pays the uncovered track space's energy like every node placement (the content prints "place 1 node") | `board/node.rs`; `content/ais.json` |
| M2 | Difficulty "each level includes the ones below", incomes under level 3 (adversary.md, generated) | Incomes are a per-level list read at every level (3/4/18/24/13 compute, 0/3/3/3/2 agents + Garrisons 1); level 4's are lower than level 3's | `adversary.rs:115-119`; `docs/blocks.rs:376` |
| M3 | Counter surcharges: credits +3 from level 1, +2 more from level 2 (threats.md, twice) | +4 from level 1, +0 more at level 2 | `configs/baseline.json` `counter_bonus_credits` 4, `counter_more_credits` 0 |
| M4 | Level 3 gives 4 compute per unpaid threat (threats.md, new-player summary) | From level 2 (`board_difficulty_at` 2), with a Deploy +1 the summary omits | `adversary.rs:305-345` |
| M5 | Requirement plates: yellow at least, **red at most** (card-grammar.md, card-guidelines.md Legend) | No at-most requirement exists; only the renderer knows the field | `cards/play.rs:20-26`; `content.rs` |
| M6 | The Protector: "a facility under you is safe" (ai-boards.md) | Only from adversary troops: war, threats and corruption still destroy it | `board/destroy.rs` (no Wall check) |
| M8 | Purge: "botnets always count" (alignment.md prose) | At 6 Unity tips botnets stop counting (`purge_at` 3, 6, 6); the generated table is right | `adversary.rs:1505-1511` |
| M9 | A tip's steps (alignment.md) | Also makes corp troops at every neutral factory, before the card is drawn | `alignment.rs:407-416` |
| M10 | Round phase 1: "AI boards sway" (economy.md) | The adversary board's drip and land corruption only | `alignment.rs:523-530` |
| M13 | Stations make `station_cubes` (1) "in its owner's color" (board.md) | A player's makes `owned_station_cubes` (2) | `board/production.rs:413-414` |
| M14 | "Two stations a country fill an 8-cube card in round 4" (board.md) | At setup a country has one neutral station (1 a round); the other is the adversary's and makes compute | `board/production.rs` (`station_cubes`), `adversary.rs` (`compute_step`) |
| M15 | Setup gives each player their board's credit production (board.md) | Only start credits and energy; production is income in upkeep | `ai.rs:380-385` |
| M16 | An adversary cube on a megaproject pays its adversary payout (board.md; printed on the cards) | No rule or op puts one there | `board/production.rs:471-501` |
| M17 | "Most tier-2 buildings" must replace their own tag (player-cards.md) | 3 of 23 do (Orbital Embassy, Agency Directorate, Power to the People) | `content/cards.json` |
| M18 | Every token card arrives loaded (player-cards.md) | Popular Uprising enters with 0 Followers | `content/cards.json` |
| M19 | Cyber has the only standing counter (player-cards.md) | Market Stabilization Board has one too (Economic, 6) | `content/cards.json` |
| M20 | Discounts apply to buildings (player-cards.md "buildings minus your discounts") | Tag discounts ("your X cards cost 5 less") reach every card kind | `cards/price.rs:164-169` |
| M23 | A strike removes pieces "next to" the hex (ambassadors.md) | Adjacent hexes only; Honeypot Relay and Lure Network take agents only | `effects/moves.rs:119-134` |
| M24 | Player ambassadors move only through cards and Move (ambassadors.md); reach is 1 for every move (README *Reach*) | Another player's Gather moves yours; Move takes one from anywhere; Peacekeeper Rotation (content, not in the deck) moves 2 with no reach | `effects/moves.rs:32-37`; `effects/ambassadors.rs:45-64` |
| M27 | A cordoned board's pool "lands nowhere" (adversary.md) | The pool empties: those agents are lost, as with no site of the type | `adversary.rs:971-984` |
| M31 | Casus Belli: "the fronts next to the threat's base" fight twice (war.md) | The one front that base belongs to | `effects/new_ops.rs:91-98` |
| M32 | No threat makes corp troops (war.md lists their sources) | War Footing (a project) fills every base | `effects/new_ops.rs:661-676` |
| M33 | "Every troop was there through an action phase except..." (war.md) | Deploy, the Protector landing and Declaration of War also land troops before the march | `adversary.rs:334-345` |
| M36 | Troops march "in hex-number order" (war.md) | Country by country, then hex number | `board/troops.rs:124-126` |
| M37 | Threats that deal a threat: Provocation, Last Ultimatum (threats.md) | Full Spectrum Assault too | `content/threats.json` |
| M38 | New-player summary: turn up threats until one has a listed tag (threats.md) | It also needs a site of its location type in the country | `threats/mod.rs:215` |
| M39 | Vigil happens in the march, phase 7, and a shelter never meets a march (ai-powers.md) | A threat's immediate march (phase 17) also fires Vigil (wasted insight) and meets a shelter | `effects/new_ops.rs:125-133` |
| M40 | A Blackout threat stops a country's cities (economy.md) | The Blackout threat takes energy or cards; only an event line and Rolling Blackouts set the flag | `content/threats.json`; `board/production.rs:322` |
| M41 | Smear Campaign and Deepfake Wave take insight (economy.md) | Deepfake Wave doesn't; Data Heist (a project) does | `content/threats.json`, `projects.json` |
| M42 | Antitrust Action is chip income (economy.md) | A one-time gain per tag | `content/cards.json` |
| M43 | Events "are used and discarded" (README new-player summary) | They go to a played pile and never return | `cards/play.rs:273-296` |
| M44 | The game ends when a round starts with no card (adversary.md, alignment.md) | The round's Sway step and its tip check run first | `configs/baseline.json` `round` |
| M45 | A prohibition applies to this round's turn "if its build is ahead" | A Unity tip in the build step's own check counts as ahead, after Rally and Deploy ran | `adversary.rs:1076-1085` |
| M46 | Start corrupted cities: `start_corrupted` + `extra_corrupted` (board.md) | Capped at the 2 cities each piece prints in its corruption order | `board/mod.rs:285` |

**Not rules for a player** (left out of the rulebook): the engine's 64-tip safety cap in one check, the round-1000
stop, `pull_forced`, and mechanisms no deck card prints (lowered income, slot-adding buildings, tags from hand,
place-token triggers, copy income, the chip-paid Harden action, building upkeep, Sabotage).
