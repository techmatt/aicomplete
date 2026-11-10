# AI boards (module `ai`; [`content/ais.json`](../../content/ais.json))

*The three boards, their special rules, node tracks and capstones. The powers, the boards' insight conversions, hardening and shields are in [ai-powers.md](ai-powers.md); terms and conventions in [README.md](README.md).*

Each AI board has a **special rule**, a printed **income**, **printed tags**, two **node tracks** (economy and
card-play), building slots (`ai.slots`) and starting credits (`ai.start_credits`), config lists by board, so a board
has the same in every seating, and its **powers**. The first token starts on the economy
track, so its first space is uncovered from setup; placing a node takes the next token off a track and uncovers what
is printed under it. Uncovered values add to the income every round (insight at the start of the round, the rest in
upkeep). The board module owns nodes and counts the tokens taken (`board.track.<seat>`); the AI board reads it.

**The Builder, for new players.** Your special rule is Breadth: five building slots, so your engine is wider. You start with 40 credits and earn 15 a round, and each credit space of your
economy track adds 2 or 3 more, so placing a node is how your income grows; you're short of energy. Round 1 deals you
8 tier-1 cards (4 free, each further keep 5 credits) and 2 tier-2 cards (1 insight each), so choose between more cards
now and more money for the first plays. Each round you may turn credits into a chip with Mint, 5 credits into an
insight with Focus, and 2 chips into 2 cards with Procure. Late in the game, sacrifice cards you have finished with:
an Industry card pays out chips, a Power card energy, and an Industry card with an energy plays a building from your
hand for no credits. Each round you may move 1 ambassador for free (Movement 1; 2 from your fourth economy space). Your printed
Industry tag counts like one on your cards.

**The Protector, for new players.** Your special rule is Wall: adversary troops can't enter a hex with your node or
strike into it, so where you stand is a barrier and a facility under you is safe. But each round's build lands 1 troop on a factory
(else a city) in every country you're in (anyone's factory, never your node's hex), and it can strike in that round's march. Each troop your Wall stops pays you 1 insight (Vigil). You spend energy to harden
facilities with Bulwark, and your board prints 2 energy a round. You earn fewer credits (13 a round), so you start with
more: 46. Each credit space of your economy track adds 2 or 3 credits, the first one 1 energy too. Round 1 deals as
for the Builder. Late in the game, sacrifice
cards you have finished with: a Security card pays out credits or clears a corruption marker, a Diplomacy card draws 2
cards (Debrief), and a Security card with an energy shelters a whole country for the round. Each round you may move 1
ambassador for free (Movement 1; 2 from your fourth economy space). Your printed Security tag counts like one on your
cards.

**The Researcher, for new players.** Your special rule is Patience: your own insight you don't spend stays with you
into the next round, so you can save up for a big round (insight another player gave you this round doesn't carry). You start with 40 credits and earn 11 a round, 3 more for each credit
space of your economy track; round 1 deals as for the Builder. You play off
the board, on your turn budget and on research cubes. Each round you may turn 1 energy into 3 cards, a card from your hand into 2 insight (Synthesis), and 3
credits into a research cube on a megaproject that isn't complete yet. Your second node power costs 3 research cubes and no energy,
so build stations early: once their country's card completes they put cubes on your board. Sacrifice an Industry card
to build a station for free, a Cyber card to strip 3 compute off an adversary project, a Power card and an energy
for an antimatter. Your printed Science tag counts like one on your cards (1 Science card makes 2 for the discount). Each round you may move 2
ambassadors for free (Movement 2, up to 4), to stand guard on your stations. The hard part to explain: Open
Science's insight is handed out in upkeep and lands at the start of the next round.

**Board tags: one tag rule (candidate; Matt, 2026-11-08).** A board's printed tag (Builder Industry, Protector
Security, Researcher Science) and a tag a track space reveals count like any tag on your cards (requirements, tag
discounts, income per tag, a capstone's count), except that they can't be sacrificed: a sacrifice discards a card.
Revealed tags are rare, late and costly: two a board, on space 6 (cost 3), placed about round 6.

**Movement (candidate; Matt, design doc rev 24).** There is no paid Move. Each round you have your board's
**Movement**: its printed number plus 1 for each uncovered Movement space, each one a free Move (one of your
ambassadors from your supply or the board onto a piece in reach). Movement unused this round is lost; a space
counts from when it is uncovered. Cards that move ambassadors (Liaison Corps, Summit Caucus) add to it. Economy space 4
is reached about round 2-3. **The Researcher is the movement-heavy board,** as its stations spread over the map: its own
ambassador on one armors it and keeps agents off, which would block its cubes.

**Give / get** (`values.json`): a Movement is 1.4 a round; space 6
(cost 3, about 18) is even on the way to the capstone. A printed tag is free: the tag (1), its discount one card
sooner, the per-tag cards' pay; the Researcher's Science discount reaches every card it plays.

**Capstones (candidate).** Each track ends in a capstone, a standing rule once its space is uncovered, as a card's
Effect is while it is in play. A capstone is what a seat pushes on for; at 1-2
placements a round it is reachable about round 8.

<!-- generated: boards -->
| Board | Printed income | Starting credits | Slots | Printed tags | Movement | Economy track | Card-play track | Tokens in all |
|---|---|---|---|---|---|---|---|---|
| Builder | 15 credits, 5 insight, 1 energy | 40 | 5 | Industry | 1 (up to 2) | 7 | 7 | 14 |
| Protector | 13 credits, 5 insight, 2 energy | 46 | 4 | Security | 1 (up to 2) | 7 | 7 | 14 |
| Researcher | 11 credits, 5 insight, 1 energy | 40 | 3 | Science | 2 (up to 4) | 7 | 7 | 14 |
<!-- /generated -->

**The starting-economy scaffold (candidate; Matt).** Until engines carry the economy, the boards start and earn much
more, and the economy track is how a seat's income grows; it comes down as engines take over (3 energy to start,
`ai.start_energy`; earlier steps are in [rules-history.md](../rules-history.md)). **The reasoning:**
a board at 40 that keeps 2-4 extra cards in round 1 (10-20 credits) has 20-30 left for its first plays at a tier-1
median of 10 credits: keeping more and playing less is a real choice. A credit space pays 2-3 a round for 1 energy
plus its step (0-3): the early spaces pay back in a round or two, the cost-3 space in about three, so the last ones
are a late-game choice.

**One payment path (locked).** Every space prints a **cost** in energy, the last ones more. Every placement pays it,
on top of what a node power costs; a card pays only the step, and a card that prints "free" pays nothing (all four
deck cards that place a node). Only tracks whose next step you can pay are offered, and a placement you can't pay doesn't happen.

**Hard to explain to a new player:** a placement's price is two numbers, the power's and the space's; Harden has two
prices, Bulwark's energy and the standard project's credits (`harden_credits`), and Faraday Yard hardens when played; "bound" is a new word for a city that is otherwise only aligned.

**Node powers (candidate).** There is no growth phase. Nodes come from the board's two **node powers**, used in the
action phase, each once a round, so the powers place 0-2 nodes a round (cards that place nodes add to that: cards
aren't capped). Every board has the same first one, 1 energy for 1 node; the second costs a steep price in the board's
own currency ([ai-powers.md](ai-powers.md)). Three things keep the loops short: 2 placements a round by power and rising step costs cap energy → node →
energy income; each second power is once a round; and every tag sacrifice permanently loses cards.

<!-- generated: node_track -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 (on the board at setup) | +1 insight | +1 insight | +1 insight |
| 2 | +3 credits | +1 energy, +2 credits | +3 credits |
| 3 | +2 credits | +3 credits | builds a free research station in reach |
| 4 | +3 credits, **Movement +1** (cost 1) | +3 credits, **Movement +1** (cost 1) | +3 credits, **Movement +1** (cost 1) |
| 5 | +3 credits (cost 2) | +3 credits (cost 2) | +3 credits (cost 2) |
| 6 | +3 credits, Power tag (cost 3) | +2 credits, Cyber tag (cost 3) | +3 credits, Power tag (cost 3) |
| 7 | **Full Capacity** (capstone): 1 chip each round per 2 Industry tags on your cards. (cost 5) | **Sanctuary** (capstone): Corruption never destroys your nodes. (cost 5) | **Open Archive** (capstone): Each round, 1 research cube onto your board. (cost 5) |
<!-- /generated -->

The **keep upgrade**: tier-2 keeps cost `keep_tier2_upgraded` instead of `keep_tier2` ([player-cards.md](player-cards.md)).

**The card-play track (draft candidates, reasoning in `experiments/results/screen-068-boards/boards.md`).** None of it
is uncovered at setup. A placement takes the next token from the track the player chooses (asked when both
have tokens).

<!-- generated: node_track2 -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 | +1 insight | +1 insight | +1 insight |
| 2 | **keep upgrade** | **keep upgrade** | +2 insight, **Movement +1** |
| 3 | +2 insight (cost 1) | +2 insight (cost 1) | **keep upgrade** (cost 1) |
| 4 | +2 credits (cost 2) | +2 credits (cost 2) | +1 insight (cost 2) |
| 5 | +2 insight (cost 2) | +2 insight (cost 2) | +2 insight (cost 2) |
| 6 | +1 insight, Cyber tag (cost 3) | +1 insight, Diplomacy tag (cost 3) | +2 credits, Industry tag (cost 3) |
| 7 | +1 insight, **Master Plan** (capstone): Your buildings cost 2 credits less. (cost 5) | +1 insight, **Aegis Doctrine** (capstone): Each round, stop the first destruction in a country you're in. (cost 5) | +1 insight, **Open Science** (capstone): Each round, 2 insight from the supply, split among other players. (cost 5) |
<!-- /generated -->

**Loops and caps.** No resource has a holding limit and no cap limits production ([economy.md](economy.md), *No
limits*): a runaway is fixed at the cards that feed it. Program credit income stacked by draw chains is bounded by
insight (every card that draws costs at least 1 to play, and credits can't buy it) and by the per-tag programs' rate
of 1 a 2 tags. Credits → chips → buildings → income → credits runs once a round and in 5 slots at most; chips
can't be sold back. Rush Build
takes 1 Industry tag and 1 energy out and puts a building with 1-3 tags in, once a round: the Industry count can hold
level, but each use spends an energy and a card for good, and the building still pays its insight, so energy and the
hand cap it. **The energy gift** (Shared Grid Accord and Interconnector Treaty, programs) is one-shot. Energy bought
from a neutral city costs `city_energy_credits` (5); credits → energy → a card's income is capped by the cards and
the cities' energy (a city's `corp_energy` cubes a round, none while an agent stands on it). **Energy
counters** spend energy for good. **The Researcher's:** Patience with the cube converters (Cintamani's, a cube for 2
insight; everyone's standard action, a cube for 1) is the intended engine, with no limit a round: capped by how many
stations make cubes (and Open Archive's 1 a round) and, for Cintamani's, by the city node in its country. Bought cubes
go only onto incomplete cards and are spent at completion, so credits never reach a converter. Patience banks only
the Researcher's own insight, and no standard action gives insight. Open Science's insight goes only to
other players; alone it is the Researcher's own, and Patience keeps it. Fieldwork's three
cards are paid for again to keep in play (credits and insight); sold, they are 3 credits for 1 energy once a round.
Followers → ambassadors → alignment → city energy → node is capped by building count and the Followers each
ambassador costs. Evicting ambassadors stay on the map, capped by the supply, and the next arrival there cancels one.
**Movement** has no loop: a number a round, used up by moves, gaining nothing held. Wall has no loop, but it takes
troops away as a harm where the Protector stands; `docs/harms.md`'s checks count sources, so they hold.

`ai.boards` (`[0, 1, 2]`: Builder, Protector, Researcher) gives each seat its board; the list repeats for more seats,
so 4 players repeat the Builder. A board has no unique cards or starting building, and content that prints
`cards` or `start` on a board is refused.

<!-- generated: board_details -->
| Board | Special rule |
|---|---|
| Builder | *Breadth*: You have 5 building slots; the Protector has 4 and the Researcher 3. |
| Protector | *Wall*: Adversary troops can't enter or strike a hex with your node. Vigil: when your Wall stops a troop, gain 1 insight. |
| Researcher | *Patience*: Your own unspent insight carries over to the next round; insight other players gave you this round does not. |
<!-- /generated -->
