# Threats (module `threats`; [`content/threats.json`](../../content/threats.json))

*The threat decks, dealing, location tokens, who is near, countering and what threats do. Terms and conventions are in
[README.md](README.md). Every threat, with its text and counter menu, is in [adversary-cards.md](../adversary-cards.md). Source: `drafts/adversary/`, loaded into `content/` by
`scripts/content/load_adversary.py`: edit the drafts.*

- Variant **`countries`** (default): the model below. Variant **`off`**: no threats
  ([README.md](README.md#off-variants)).

**The decks (candidate; Matt's threat card standard, 2026-11-07, locked).** One threat deck per tier, each card
printed once: tier 1 and tier 2; **there is no tier 3**. A tier-2 card is about two tier-1s
(bigger consequences: a node, a site corrupted, a botnet, more Fallout, more research cubes, Unity pulled back; no threat makes a troop), so no event card
deals two tier-1s. Each card carries 1 or 2 **tags**, the player cards' own (Security, Diplomacy, Cyber, Industry, Power, with their icons; r81, friction #47; **Universal** counts as every tag) and 1 or 2 **location
types** (city, coastal city, research station, factory).

**Built once, at setup (r139, Matt 2026-11-19).** The adversary's board prints how its decks are built, and that mix
is its lean. The Militarist's **War Rooms**: "Setup: build each threat deck from every Security and Universal threat
of its tier and as many of its other threats, taken at random; shuffle it. The rest stay in the box." Nothing is
filtered during a round: the table below counts the cards printed, not a game's decks.

<!-- generated: threats -->
| Tier | Cards | Security | Diplomacy | Cyber | Industry | Power | Universal |
|---|---|---|---|---|---|---|---|
| 1 | 25 | 6 | 7 | 6 | 6 | 5 | 0 |
| 2 | 32 | 7 | 7 | 7 | 8 | 6 | 5 |
| 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
<!-- /generated -->

**Dealing: the next card, always (r139).** The adversary's event card ([adversary.md](adversary.md)) prints its
threats on its top line, with no tag: "Threat, each country: a tier-1 threat". **The ladder:**
stage I deals one tier-1 threat a country, stage II one tier-2 (or a tier 1 and a tier 2: Supply Lines), stage III
two tier-2s (`check.py` refuses another shape). In `threats.deal`, for each threat printed and each country in order,
the **top card** of that tier's deck is dealt there. No card is turned and discarded, and no country is skipped. An
empty deck reshuffles its discard. The ladder raises intensity,
never the count of threats dealt (142, Matt). A threat that deals a threat
(Provocation, Full Spectrum Assault, Last Ultimatum) and a Discord card's extra threat deal the next card the same
way, at the printed tier, a threat's into its own country; one dealt after the counter phase executes next round.

**Location tokens.** A dealt threat puts a token on a site of each of its location types in its country (two tokens
never share a hex), one condition for every type: a site of the type that is **not the adversary's**. Tokens go in
printed order (r80, Matt's ruling #46),
each onto the site of its type with **the most agents on its own hex** (troops, botnets and neighbors don't count);
ties, the lowest hex number. A location type with no such site left in the country gets no token. **The fallback
(r139):** a dealt threat with no such site of any of its types puts its one token on the country's lowest-numbered
city, one not the adversary's first; there none of its located lines acts, whatever its types. The token still marks who is near and where its agent lands. A located line
("the factory here") acts only on its own kind of hex (r137, D132): with no token of that kind, as on the fallback
city or a first token dealt onto a node hex of another kind (Offensive), it does nothing (184).

**Near.** Every threat card prints, above its text, "Payers: any player with a Node or Ambassador on or next to a
token. Target: this country's player." A player is **near** a threat with one of their nodes or ambassadors on or next to any of its tokens. Only a
player near it may pay toward it. A piece it places lands on its token unless the text says otherwise (with two tokens,
"here" is the token of its first printed location). A threat nobody is near still executes.

**This country's player (r139, Matt 2026-11-19; locked).** An uncountered threat's harm hits **one player: the
player of the country it was dealt in**, the one whose home country it is (the country they start in), wherever
their nodes are now and whoever was near it. Card texts say "This country's player pays ...". There is no distance
to measure and no tie. Where seats wrap onto countries, it is the lowest such seat; in a country that is no player's
home, only the threat's board consequences land. This is the one rule that reads a player's home country. "Destroy
a node" takes that player's node closest to the threat's tokens (ties: their pick), their last too (175).

**Countering: a menu.** A threat prints a few ways to pay; the table pays **one option in full**:

- **Resources** (credits, energy, insight, research cubes, chips, ambassadors from your supply) are pooled across the
  players near it (`Pay` 1, or as much as is still useful; `Withdraw` takes back everything you put on it in one
  resource, even after you stop being near). The moment some option's resources are all covered it is countered and
  discarded; the rest goes back, later seats first.
- **Tags**: one player sacrifices cards in play carrying that many of the threat's own tag (as an AI-board power's
  cost, [ai-powers.md](ai-powers.md)): a Security threat takes Security tags, with no table between. Tier 1 asks 2
  tags, tier 2 asks 3 (Matt, 2026-11-06).
- A tag option is paid whole by one player (`CounterWith`). (The engine can also read a nodes item, a player's own
  nodes near a token; no menu prints one.)

A player's card that jams or counters a threat reaches only threats they are near (a typed one, threats carrying its
tag or Universal); a jam takes its amount off every resource item of every option (floor 0). 274's jam floor
(`adversary.jam_floor` 1: a jam never takes an item below 1, so never counters alone) is off: on the 12 jam cards'
give / get it left Force Protection Order under break-even (a hard-cut candidate, with Matt). A Unity reward reaches any threat.
**Standing counters:** a tier-2 capstone with an automatic counter takes its amount off each threat of its type on
the table, wherever it is and whenever it was dealt (148: no longer only those its holder is near); one it pays in
full is countered at once (118). A threat costs what it prints at every level (267: the ladder adds no
surcharge).

**Counters (146; Matt's standard).** **One counter option, two at most**: a scarce currency or a tag sacrifice, or
credits ("pay 3 energy or 8 credits"). Printed amounts are paid (`counter_pct` 100 by tier and `counter_credit_pct`
100 are the screens' dials). Tier 1: 3 energy, 3 ambassadors, 3 insight, 3 research cubes, 3 chips, 2 tags or 8
credits; tier 2: 4, 4, 4 insight, 5, 4 chips, 3 tags or 11 credits (r128: credits ÷ 1.5, to the nearest; r83: chips halved with the chip's doubled worth;
r89: insight 6 and 7 → 3 and 4, about half a round's insight at r87's income) (146: the screens' 85% folded in). Nothing is priced as a share of income. **A counter never shares
a currency with its harm** ("pay 5 credits or lose 5" is a design error): no harm takes credits, and a harm taking
energy, chips, cubes, insight, ambassadors or nodes drops that counter. The load check and `drafts/adversary/check.py`
refuse a threat with more than two counter options, more than two consequences, a shared currency, a harm on each
player (a flag on every hex near it, a line in each country, "each player destroys"), or a special piece (a troop
that stands or occupies). No counter prints 1 credit (Matt, r48).

**Executing.** Threats execute in `threats.execute`, after production and upkeep, at the round's end: anything still on
an uncountered threat is refunded, then each threat (country 0, 1, ...) does what it prints, its player harm on its
country's player, and is discarded with its tokens. **No harm is unprinted** (146: the node loss for every player near
an uncountered threat and the floor sway toward Discord are gone). **Every uncountered threat also lands
`adversary.threat_agents` (1) agent on its anchor** (its first location token's hex) after its harm, an arrival that may corrupt
the site or take a node there ([adversary.md](adversary.md)). **A tier-1 threat never takes its own site** (r137,
D130): its creation and that agent stay under the site's number on an empty site at every level (Agitators, Embargo
and Insider Access create 1); a tier-2 threat whose creation reaches it takes the site uncountered, as its effect
([harms.md](../harms.md), *Threats that take their own site*). The adversary board may print compute for each
uncountered threat (the Militarist's level-2 rule, War Footing: 4, from `board_difficulty_at` (2); [adversary.md](adversary.md)).
Only threats on the board during this round's counter phase execute: one dealt later gets a printed **waits** token
and executes next round,
after a counter phase in which it can be paid (Matt). **A resource harm reads "pays N or discards M cards"** (the
player's choice; with neither possible, the player pays what they have and discards their hand), so holding none
dodges nothing; theft is the same payment, and the adversary gains its compute either way. A corruption or seizure a
threat prints is corruption, with its cleanup (the node on the site or its armor, the ambassadors there), and takes
only the site it names. A threat takes a player's last node like any other (175: no last-node safety, [troops.md](troops.md)).
Its compute waits for the adversary's compute step ([adversary.md](adversary.md)).

**For new players (the adversary model).** Each round the adversary's event card comes first. Its top line deals
threats: in each country, deal the top card of the named tier's threat deck (the decks were built at setup, by the
line on the adversary's board) and put each location token on the site of its type holding the most agents; if the
country has no such site left, its token goes on the country's lowest-numbered city instead. If you have a node or an ambassador on
or next to a token, you are *near* that threat and may help pay it off. Each threat prints one way to pay, or two: a
scarce thing (energy, ambassadors, insight, research cubes, chips, or cards given up for their tags) or credits;
resources can be pooled between players, giving up cards is done by one player alone. What a threat takes if unpaid is
never what it asks for. Next last round's sites escalate (each site of last round's type still holding an agent
takes 1 more), then the card infiltrates (1 agent on every site of the type it names not yet the
adversary's); where it shows a Strike icon, every troop then destroys a player piece on or next to it (the card was
face up a round ahead); then its other lines.
A site corrupts the moment an arrival brings it to its number. You draft and act; production then runs from one
snapshot of the board. At the end of the round each threat nobody paid does what it prints, then lands 1 agent on
its spot, and nothing else: one consequence, sometimes two, from a short list (agents on its spot, compute, Sway toward
Discord, Unity pulled back, a site corrupted, a botnet, Fallout next to its token (227: mild and lifted, so tier 1
carries 1 and tier 2 up to 2), or the country's player paying energy, chips, cubes or
ambassadors or discarding cards, losing a building, or a node, or cubes onto the adversary's research track; never
insight, spent the turn it comes, r82; the full list: [harms.md](../harms.md)). A player harm always hits the one player who
started in the threat's country. Tier 1 is small; tier 2 is about two tier-1s. Compute waits for the adversary's compute
step, then fills the leftmost project on its board; a full project fires there, the rest slide left and the right
slot refills. **Harder levels** make the adversary harsher, not threats more or dearer: each level is named,
adds setup (the income marker, the event deck) and at most one rule that fires on an event you already
resolve: level 1 lets a city corrupt at 3 (the cards' "Botnets spread (level 1+)" start too), 2 gives 4 compute for each unpaid threat, 3 a shorter,
busier game, and 4 a harder deck; no level deals an extra threat
([adversary.md](adversary.md) *Difficulty*). The agents come from the cards' Infiltrate lines (stage III names two
site types; 2 agents a city at level 4, 1 on a facility), from Escalate and from corrupted factories, each feeding its city 1 a round.
**What resisted a summary:** that only resource options pool.

**Compute.** Many threats also give the adversary compute, onto its projects ([adversary.md](adversary.md)), when they
execute uncountered: printed on the threat. Not every threat gives it.
