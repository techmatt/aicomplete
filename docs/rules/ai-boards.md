# AI boards (module `ai`; [`content/ais.json`](../../content/ais.json))

*The three boards, node tracks and capstones, node and sacrifice powers, hardening and shields. Terms and conventions are in [README.md](README.md).*

Each AI board has a **special rule**, a printed **income**, **printed tags**, two **node tracks** (economy and
card-play), building slots (`ai.slots`) and starting credits (`ai.start_credits`), config lists by board, so a board
has the same in every seating, and its **powers**. The first token starts on the economy
track, so its first space is uncovered from setup; placing a node takes the next token off a track and uncovers what
is printed under it. Uncovered values add to the income every round (insight at the start of the round, the rest in
upkeep). The board module owns nodes and counts the tokens taken (`board.track.<seat>`); the AI board reads it.

**The Builder, for new players.** Your special rule is Breadth: five building slots (the Protector has four, the
Researcher three), so your engine is wider. You start with 40 credits and earn 25 a round, and each credit space of your
economy track adds 2 or 3 more, so placing a node is how your income grows; you're short of energy. Round 1 deals you
10 cards: 4 are free and each further keep costs 5, so choose between more cards now and more money for the first
plays. Each round you may turn credits into a chip with Mint. Late in the game, sacrifice cards you have finished with:
an Industry card pays out chips, a Power card energy, and an Industry card with an energy plays a building from your
hand for no credits.

**The Protector, for new players.** Your special rule is Wall: adversary troops can't enter a hex with your node or
strike into it, so where you stand is a barrier and a facility under you is safe. You spend energy to harden
facilities with Bulwark, and your board prints 1 energy a round. You earn fewer credits (21 a round), so you start with
more: 46. Each credit space of your economy track adds 2 or 3 credits, the first one 1 energy too. Round 1 deals you
10 cards: 4 are free and each further keep costs 5. Late in the game, sacrifice cards you have finished with: a
Security card pays out credits or clears a corruption marker, and a Security card with an energy shelters a whole country
for the round.

**The Researcher, for new players.** Your special rule is Patience: insight you don't spend stays with you into the
next round, so you can save up for a big round. You start with 40 credits and earn 18 a round, 3 more for each credit
space of your economy track; round 1 deals you 10 cards, 4 of them free to keep and 5 credits each after. You play off
the board, on your turn budget and on research cubes. Each round you may turn 1 energy into 3 cards and 3 credits into
a research cube on a megaproject that isn't complete yet. Your second node power costs 3 research cubes and no energy,
so build stations early: once their country's card completes they put cubes on your board. Sacrifice an Industry card
to build a station for free, a Cyber card to strip 3 compute off an adversary project, a Power card and an energy
for an antimatter. Your Science tag counts for cards' requirements, never for a sacrifice. The hard part to explain: Open
Science's insight is handed out in upkeep and lands at the start of the next round.

**Printed and revealed tags (candidate).** Tags printed on a board and tags a track space reveals count for what a
card requires, exactly as tags on your cards do, and for nothing else: no tag discount, no income per tag, no
sacrifice.

**Capstones (candidate).** Each track ends in a capstone, a standing rule once its space is uncovered, as a card's
Effect is while it is in play. Late spaces cost more energy, so a capstone is what a seat pushes on for; at 1-2
placements a round it is reachable about round 8.

<!-- generated: boards -->
| Board | Printed income | Starting credits | Slots | Printed tags | Economy track | Card-play track | Tokens in all |
|---|---|---|---|---|---|---|---|
| Builder | 25 credits, 4 insight | 40 | 5 | Industry | 7 | 7 | 14 |
| Protector | 21 credits, 4 insight, 1 energy | 46 | 4 | Security | 7 | 7 | 14 |
| Researcher | 18 credits, 4 insight | 40 | 3 | Science | 7 | 7 | 14 |
<!-- /generated -->

**The starting-economy scaffold (candidate; Matt).** Until engines carry the economy, the boards start and earn much
more, and the economy track is how a seat's income grows; it comes down as engines take over. **The reasoning:**
round 1 deals 10, so a board at 40 that keeps 2-4 extra cards (10-20 credits) has 20-30 left for its first plays at a
tier-1 median of 6 credits and 4 insight: keeping more and playing less is a real choice, as in Terraforming Mars'
first generation. The Protector earns 4 less than the Builder and starts 6 ahead. A credit space pays 2-3 a round for
a placement of 1 energy plus its step (0-3), energy at about 2 credits: the early spaces pay back in a round or two,
the cost-3 space in about three, so a space placed by round 6 of 8-10 pays for itself and the last ones are a
late-game choice. **The pressure that keeps it honest is the adversary's:** a player ambassador costs 20 credits
(`board.ambassador_credits`) and a corrupted city makes 2 adversary ambassadors a build.

**One payment path (locked).** Every space prints a **cost** in energy, the last ones more. Every placement pays it,
on top of what a node power costs; a card pays only the step, and a card that prints "free" pays nothing (three
cards). Only tracks whose next step you can pay are offered, and a placement you can't pay doesn't happen.

**Hard to explain to a new player:** a placement's price is two numbers, the power's and the space's; Harden has two
prices, Bulwark's energy and a card's chips; "bound" is a new word for a city that is otherwise only aligned.

**Node powers (candidate).** There is no growth phase. Nodes come from the board's two **node powers**, used in the
action phase, each once a round, so the powers place 0-2 nodes a round (cards that place nodes add to that: cards
aren't capped). Every board has the same first one, 1 energy for 1 node; the second costs a steep price in the board's
own currency, plus 1 energy on the Builder and Protector (the powers table below). The player chooses between the two
tracks. Three things keep the loops short: 2 placements a round by power and rising step costs cap energy → node →
energy income; each second power is once a round; and every tag sacrifice permanently loses cards.

<!-- generated: node_track -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 (on the board at setup) | +1 insight | +1 insight | +1 insight |
| 2 | +3 credits | +1 energy, +2 credits | +3 credits |
| 3 | +2 credits | +3 credits | builds a free research station in reach |
| 4 | +3 credits, Power tag (cost 1) | +3 credits, Diplomacy tag (cost 1) | +3 credits, Power tag (cost 1) |
| 5 | +3 credits (cost 2) | +3 credits (cost 2) | +3 credits (cost 2) |
| 6 | +3 credits (cost 3) | +2 credits (cost 3) | +3 credits (cost 3) |
| 7 | **Full Capacity** (capstone): 1 chip each round per 2 Industry tags you have. (cost 5) | **Sanctuary** (capstone): When the adversary spreads, it skips your nodes. (cost 5) | **Open Archive** (capstone): Each round, 1 research cube onto your board. (cost 5) |
<!-- /generated -->

The **keep upgrade**: tier-2 keeps cost `keep_tier2_upgraded` instead of `keep_tier2` ([player-cards.md](player-cards.md)).

**The card-play track (draft candidates, reasoning in `experiments/results/screen-068-boards/boards.md`).** The second
track, none on the board at setup. A placement takes the next token from the track the player chooses (asked when both
have tokens).

<!-- generated: node_track2 -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 | +1 insight | +1 insight | +1 insight |
| 2 | **keep upgrade**, Industry tag | **keep upgrade**, Security tag | +2 insight |
| 3 | +2 insight (cost 1) | +2 insight (cost 1) | **keep upgrade**, Science tag (cost 1) |
| 4 | +1 credit, Science tag (cost 2) | +1 credit, Cyber tag (cost 2) | +1 insight, Cyber tag (cost 2) |
| 5 | +2 insight (cost 2) | +2 insight (cost 2) | +2 insight (cost 2) |
| 6 | Cyber tag (cost 3) | Diplomacy tag (cost 3) | +1 credit, Industry tag (cost 3) |
| 7 | +1 insight, **Master Plan** (capstone): Your buildings cost 2 credits less. (cost 5) | +1 insight, **Aegis Doctrine** (capstone): Each round, stop the first destruction in a country you're in. (cost 5) | +1 insight, **Open Science** (capstone): Each round, 2 insight from the supply, split among other players. (cost 5) |
<!-- /generated -->

**Loops and caps.** Credits → chips → buildings → income → credits runs once a round and in 5 slots at most. Rush Build
takes 1 Industry tag and 1 energy out and puts a building with 1-3 tags in, once a round: the Industry count can hold
level, but each use spends an energy and a card for good, and the building still pays its insight, so energy and the
hand cap it. **The energy gift** (1 energy → another player's 5 chips at Substation Hub or 8 credits at Treaty Grid
Exchange, once a round a building) is a conversion with a boost: an energy is worth about 4.5, the gift 7.5-8. Energy
bought from a neutral city costs `city_energy_credits` (5), so credits → energy → Treaty Grid Exchange → credits gains 3 a
round, for the team, not the buyer; Substation Hub's 5 chips sell back for 5 credits (`chips_credits` 1), so its loop
gains only where the chips pay a chip cost. It is capped by once a round, the two buildings, and the cities' energy
(a city's `corp_energy` cubes a round, none while an adversary ambassador stands on it). Nothing turns the gift back
into energy. **Energy counters**
spend energy for good. **The Researcher's:** Patience with Cintamani's converter (a cube for an insight) is
the intended engine, capped by how many stations make cubes, by the city node in Cintamani's country that the converter
needs, and by once a round per player. Bought cubes go only onto incomplete cards and are spent at completion, so credits never reach a converter.
Open Science's insight comes from the supply, not the Researcher's bank, and goes only to other players, so Patience
can't bank it. Fieldwork's three cards are paid for again to keep in play (credits and insight); sold, they are 3
credits for 1 energy once a round. Followers → ambassadors → alignment → city energy → node is capped by building
count and the Followers each ambassador costs. Evicting ambassadors stay on the map, capped by the supply, and the next
arrival there cancels one. Wall has no loop, but it takes troops away as a harm where the Protector stands;
`docs/harms.md`'s checks count sources, so they hold.

**Sacrifice is late-game liquidation.** Once the cards needing a tag are played, its tags turn into engine rewards,
each payoff priced to be worth the card: 2 chips, 2 energy, 3 credits, a marker with its Sway; a tag and an energy buy a
building, a sheltered country or an antimatter (r35).

**Powers (locked: tag costs are sacrifices).** A power's cost names tags (`any` for any tag) and optional other
resources; in the table a tag in the cost is a sacrifice, and thresholds are for powerful cards only. It is paid by
discarding played cards that carry those tags: a program from your play area, or a building **with its whole stack**
beneath it; events can't be sacrificed (they are gone once played). A card pays with all its tags, each tag first
toward its own requirement, then toward `any`; extra tags are wasted. Each power is usable once a round. What can be sacrificed is fixed when the power is used; the player chooses the cards one at a time among those that
still help.

<!-- generated: powers -->
| Board | Power | Cost | Effect |
|---|---|---|---|
| Builder | Mint | 3 credits | gain 1 chip |
| Builder | Deploy | 1 energy | place 1 node |
| Builder | Buy In | 1 energy + 6 credits | place 1 node |
| Builder | Scrap | 1 Industry | gain 2 chips |
| Builder | Overcharge | 1 Power | gain 2 energy |
| Builder | Rush Build | 1 Industry + 1 energy | play a building from your hand, paying no credits |
| Protector | Bulwark | 1 energy | harden 1 facility in reach |
| Protector | Deploy | 1 energy | place 1 node |
| Protector | Accord | 1 Diplomacy + 1 energy | place 1 node |
| Protector | Stand Down | 1 Security | gain 3 credits |
| Protector | Recovery | 1 Security | clear a corruption marker in reach |
| Protector | Shelter | 1 Security + 1 energy | in one country this round, no nodes or facility can be destroyed |
| Researcher | Fieldwork | 1 energy | draw 3 cards |
| Researcher | Deploy | 1 energy | place 1 node |
| Researcher | Peer Review | 3 research cubes | place 1 node |
| Researcher | Grant | 3 credits | add 1 research cube in your color to an incomplete megaproject |
| Researcher | Field Station | 1 Industry | build a research station in reach, paying no chips |
| Researcher | Audit | 1 Cyber | remove 3 compute from one adversary project |
| Researcher | Particle Run | 1 Power + 1 energy | gain 1 antimatter |
<!-- /generated -->

`ai.boards` (`[0, 1, 2]`: Builder, Protector, Researcher) gives each seat its board; the list repeats for more seats,
so 4 players repeat the Builder. The engine reads a board's `cards` and `start` (one copy each, so a repeated AI's
second seat would have neither); no content prints them.

A **shelter** (the Protector's Shelter, or a card's "shelter one country") lasts until cleanup (phase 18) of the round
it is used in. Used in the actions, it covers the rest of the actions and what comes after them: troops from bases,
war, production and the threats that execute (phases 13-17). The march (phase 5) has already happened, so a shelter never meets it.

<!-- generated: board_details -->
| Board | Special rule |
|---|---|
| Builder | *Breadth*: You have 5 building slots; the Protector has 4 and the Researcher 3. |
| Protector | *Wall*: Adversary troops can't enter or strike a hex with your node. |
| Researcher | *Patience*: Your unspent insight carries over to the next round. |
<!-- /generated -->

Hardening (the standard actions, cards) goes only on facilities a token can protect, player-owned stations and
factories that are not corrupted, and "harden N" is N different facilities, fewer if fewer qualify. A Science card's
**shield** is the same token on a hex of your nodes: it absorbs a destruction there and goes with the token if it is
lost another way. On your own facility a shield is the facility's hardening token: a table has one token, so it stays
when the node goes, and protects the facility.
