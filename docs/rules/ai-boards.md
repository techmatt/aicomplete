# AI boards (module `ai`; [`content/ais.json`](../../content/ais.json))

*The three boards, their special rules, node tracks and capstones. The powers, the boards' insight conversions, and hardening are in [ai-powers.md](ai-powers.md); terms and conventions in [README.md](README.md).*

Each AI board has a **special rule**, a printed **income**, **printed tags**, two **node tracks** (economy and
card-play), building slots (`ai.slots`) and starting credits (`ai.start_credits`), config lists by board, so a board
has the same in every seating, and its **powers**. The first token starts on the economy
track, so its first space is uncovered from setup; placing a node takes the next token off a track and uncovers what
is printed under it. Uncovered values add to the income every round (insight at the start of the round, the rest in
upkeep). The board module owns nodes and counts the tokens taken (`board.track.<seat>`); the AI board reads it.

**The Builder, for new players.** Your special rule is Breadth: five building slots, so your engine is wider. You start with 40 credits and earn 15 a round, and each credit space of your
economy track adds 2 or 3 more, so placing a node is how your income grows; you're short of energy. Round 1 deals
you 2 tier-1 and 2 tier-2 cards into your hand, free (the tier-2s point a build direction), then drafts and keeps as
every round does (r82). Each round you may turn credits into a chip with Mint, 2 credits into 1 insight with Focus,
and 1 chip into 2 cards with Procure. Late in the game, sacrifice cards you have finished with:
an Industry card pays out chips, a Power card energy, and an Industry card with an energy plays a building from your
hand for no credits. Each round you may Dispatch 1 ambassador for free (2 from your fourth economy space). Your printed
Industry tag counts like one on your cards.

**The Protector, for new players.** Your special rule is Wall: adversary troops can't enter a hex with your node or
strike into it, so where you stand is a barrier and no troop strikes a facility under you (war, threats and corruption still can). But each adversary turn Musters 1 troop in every country you're in
(a normal Muster: the lowest-numbered factory, anyone's, else city; on your node's hex it stays in the supply), and it can strike in that round's march. The first two troops each round that end their march next to your node pay you 1 insight each (Vigil; r79 once a round, r84 twice). You spend energy to harden
facilities with Bulwark, and your board prints 2 energy a round. You earn fewer credits (13 a round), so you start with
more: 46. Each credit space of your economy track adds 2 or 3 credits, the first one 1 energy too. Round 1 deals as
for the Builder. Late in the game, sacrifice
cards you have finished with: a Security card pays out credits or clears a corruption marker, a Diplomacy card draws 2
cards (Debrief), and a Security card with an energy shelters a whole country for the round. Each round you may Dispatch 1
ambassador for free (2 from your fourth economy space). Your printed Security tag counts like one on your
cards.

**The Researcher, for new players.** Your special rule is Patience: insight you don't spend stays with you
into the next round, so you can save up for a big round (r82: all of it, including insight another player gave you). You start with 40 credits and earn 11 a round, 3 more for each credit
space of your economy track; round 1 deals as for the Builder. You play off
the board, on your turn budget and on research cubes. Each round you may turn 1 energy into 3 cards, a card from your hand into 2 insight (Synthesis), and 3
credits into a research cube on a megaproject that isn't complete yet. Your second node power costs 3 research cubes and, like any node, the uncovered space's energy,
so build stations early: once their country's card completes they put cubes on your board. Sacrifice an Industry card
to build a station for free, a Cyber card to strip 3 compute off an adversary project, a Power card and an energy
for an antimatter. Your printed Science tag counts like one on your cards (1 Science card makes 2 for the discount). Each round you may Dispatch 2
ambassadors for free (up to 4), to stand guard on your stations. The hard part to explain: Open
Science's insight is handed out in upkeep and lands at the start of the next round.

**Board tags: one tag rule (candidate; Matt, 2026-11-08).** A board's printed tag (Builder Industry, Protector
Security, Researcher Science) and a tag a track space reveals count like any tag on your cards (requirements, tag
discounts, income per tag, a capstone's count), except that they can't be sacrificed: a sacrifice discards a card.
Revealed tags are rare, late and costly: two a board, on space 6 (cost 3), placed about round 6.

**Dispatches (candidate; Matt, design doc rev 24).** There is no paid Dispatch. Each round you have your board's
**Dispatches**: its printed number plus 1 for each uncovered Dispatch space, each one a free Dispatch (one of your
ambassadors from your supply or anywhere on the map onto a piece, base, node or botnet's hex in reach). Dispatches unused this round are lost; a space
counts from when it is uncovered. Cards that move ambassadors (Liaison Corps, Summit Caucus) add to it. Economy space 4
is reached about round 2-3. **The Researcher is the Dispatch-heavy board,** as its stations spread over the map: its own
ambassador on one armors it and keeps agents off, which would block its cubes.

**Give / get** (`values.json`): a Dispatch is 1.4 a round; space 6 (cost 3, about 18) is even on the way to the
capstone. A printed tag is free.

**Capstones (candidate).** Each track ends in a capstone, a standing rule once its space is uncovered, as a card's
Effect is while it is in play. A capstone is what a seat pushes on for; at 1-2
placements a round it is reachable about round 8.

<!-- generated: boards -->
| Board | Printed income | Starting credits | Slots | Printed tags | Dispatches | Economy track | Card-play track | Tokens in all |
|---|---|---|---|---|---|---|---|---|
| Builder | 15 credits, 1 energy | 40 | 5 | Industry | 1 (up to 2) | 7 | 7 | 14 |
| Protector | 13 credits, 2 energy | 46 | 4 | Security | 1 (up to 2) | 7 | 7 | 14 |
| Researcher | 11 credits, 1 energy | 40 | 3 | Science | 2 (up to 4) | 7 | 7 | 14 |
<!-- /generated -->

**The starting-economy scaffold (candidate; Matt).** Until engines carry the economy, the boards start and earn much
more, and the economy track is how a seat's income grows; it comes down as engines take over (3 energy to start,
`ai.start_energy`; earlier steps are in [rules-history.md](../rules-history.md)). **The reasoning:**
a board at 40 with 4 cards in hand and round 1's keeps has 3-4 plays at a tier-1 median of 10 credits; credits never
buy a keep (r82), so insight is round 1's limit on keeps and credits on plays. A credit space pays 2-3 a round for its step (0-3): the early spaces pay back in a round or two, the cost-3 space in about three, so the last ones
are a late-game choice.

**One payment path (locked).** Every space prints a **cost** in energy, the last ones more. Every placement pays it,
whatever places the node (r81, friction #16): a node power costs only its tag or currency (Expand 1 energy (r82), Buy In 6
credits, Accord 1 Diplomacy, Peer Review 3 research cubes) and a card only its credits, so the energy is stated once,
on the track; no card places a node free. Only tracks whose next step you can pay are offered, and a placement you
can't pay doesn't happen.

**Hard to explain to a new player:** Harden has two
prices, Bulwark's energy and the standard project's credits (`harden_credits`), and Faraday Yard hardens when played; "bound" is a new word for a city that is otherwise only aligned.

**Node powers (candidate).** Nodes don't grow on their own. Nodes come from the board's two **node powers**, used in the
action phase, each once a round, so the powers place 0-2 nodes a round (cards that place nodes add to that: cards
aren't capped). Every board has the same first one, Expand: 1 energy plus the space's (r82); the second costs a steep price in the board's
own currency or a tag ([ai-powers.md](ai-powers.md)). Three things keep the loops short: 2 placements a round by power and rising step costs cap energy → node →
energy income; each second power is once a round; and every tag sacrifice permanently loses cards.

<!-- generated: node_track -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 (on the board at setup) | nothing | nothing | nothing |
| 2 | +3 credits | +1 energy, +2 credits | +3 credits |
| 3 | +2 credits | +3 credits | builds a free research station in reach |
| 4 | pay 1 energy: +3 credits, **Dispatch +1** | pay 1 energy: +3 credits, **Dispatch +1** | pay 1 energy: +3 credits, **Dispatch +1** |
| 5 | pay 2 energy: +3 credits | pay 2 energy: +3 credits | pay 2 energy: +3 credits |
| 6 | pay 3 energy: +3 credits, Power tag | pay 3 energy: +2 credits, Cyber tag | pay 3 energy: +3 credits, Power tag |
| 7 | pay 5 energy: **Full Capacity** (capstone): 1 chip each round per 4 Industry tags on your cards. | pay 5 energy: **Sanctuary** (capstone): Corruption never destroys your Nodes. | pay 5 energy: **Open Archive** (capstone): Each round, 1 research cube onto your board. |
<!-- /generated -->

The **keep upgrade**: tier-2 keeps cost `keep_tier2_upgraded` instead of `keep_tier2` ([player-cards.md](player-cards.md)).

**The card-play track (draft candidates, reasoning in `experiments/results/screen-068-boards/boards.md`).** None of it
is uncovered at setup. A placement takes the next token from the track the player chooses (asked when both
have tokens). **Insight prints here only, as totals** (r82, Matt's amendment, Spirit Island style): the printed start
shows 3 (r86; 6 before) and a space that raises it shows the new total; your insight is the number on the rightmost space you've
uncovered that shows one. **Each board grows it at its own rate** (r87, Matt: income is the lever, as Spirit Island's
energy): the Builder to 10, the Protector 9, the Researcher, whose Patience banks it, 8 (r86: 13 on all three).

<!-- generated: node_track2 -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| printed start | insight 3 | insight 3 | insight 3 |
| 1 | **insight 5** | **insight 5** | **insight 4** |
| 2 | **keep upgrade** | **keep upgrade** | **insight 5**, **Dispatch +1** |
| 3 | pay 1 energy: **insight 7** | pay 1 energy: **insight 6** | pay 1 energy: **keep upgrade** |
| 4 | pay 2 energy: +2 credits | pay 2 energy: +2 credits | pay 2 energy: **insight 6** |
| 5 | pay 2 energy: **insight 8** | pay 2 energy: **insight 7** | pay 2 energy: **insight 7** |
| 6 | pay 3 energy: **insight 9**, Cyber tag | pay 3 energy: **insight 8**, Diplomacy tag | pay 3 energy: +2 credits, Industry tag |
| 7 | pay 5 energy: **insight 10**, **Master Plan** (capstone): Your buildings cost 2 credits less. | pay 5 energy: **insight 9**, **Aegis Doctrine** (capstone): Once a round, the first time one of your Nodes or facilities would be destroyed, it is sheltered. | pay 5 energy: **insight 8**, **Open Science** (capstone): Each round, 2 insight from the supply, split among other players. |
<!-- /generated -->

**Loops and caps.** No resource has a holding limit and no cap limits production ([economy.md](economy.md), *No
limits*): a runaway is fixed at the cards that feed it. Program credit income stacked by draw chains is bounded by
insight (every card that draws costs at least 1 to play, and credits can't buy it) and by the per-tag programs' rate
of 1 a 2 tags. Credits → chips → buildings → income → credits runs once a round and in 5 slots at most; chips
can't be sold back. Rush Build
takes 1 Industry tag and 1 energy out and puts a building with 1-3 tags in, once a round: the Industry count can hold
level, but each use spends an energy and a card for good, and the building still pays its insight, so energy and the
hand cap it. **The energy gift** (Shared Grid Accord and Interconnector Treaty, programs) is one-shot. Energy bought
from a neutral city (Buy energy) costs `city_energy_credits` (4); credits → energy → a card's income is capped by the
cards and the cities' energy (once per city a round, a city's `corp_energy` a buy, none while an agent stands on it). **Energy
counters** spend energy for good. **The Researcher's:** Patience with the cube converters (Cintamani's, a cube for 2
insight, once a round since r75; everyone's standard action, a cube for 1, with no limit) is the intended engine:
capped by how many stations make cubes (and Open Archive's 1 a round) and, for Cintamani's, by its once a round and
a Cintamani city in reach. Bought cubes
go only onto incomplete cards and are spent at completion, so credits never reach a converter. Patience banks all the
Researcher's unspent insight (r82: a gift too); only cards that print it pass insight, at their cost, and no standard
action gives it. Open Science's insight goes only to other players; alone it is the Researcher's own, and Patience
keeps it. Fieldwork's three
cards are paid for again to keep in play (credits and insight); sold, they are 3 credits for 1 energy once a round.
Followers → ambassadors → alignment → city energy → node is capped by building count and the Followers each
ambassador costs. Remove agents spends its ambassador and places nothing (r75), capped by the supply.
**Dispatches** have no loop: a number a round, used up, gaining nothing held. Wall has no loop, but it takes
troops away as a harm where the Protector stands; `docs/harms.md`'s checks count sources, so they hold.

`ai.boards` (`[0, 1, 2]`: Builder, Protector, Researcher) gives each seat its board; the list repeats for more seats,
so 4 players repeat the Builder. A board has no unique cards or starting building, and content that prints
`cards` or `start` on a board is refused.

<!-- generated: board_details -->
| Board | Special rule |
|---|---|
| Builder | *Breadth*: You have 5 building slots; the Protector has 4 and the Researcher 3. |
| Protector | *Wall*: Adversary Troops can't enter or strike a hex with your Node. Each adversary turn, the adversary Musters 1 in each country where you have a Node. Vigil: the first two times each round a Troop ends its march next to your Node, gain 1 insight. |
| Researcher | *Patience*: All your unspent insight carries over to the next round, including insight other players gave you. |
<!-- /generated -->
