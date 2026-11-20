# AI boards (module `ai`; [`content/ais.json`](../../content/ais.json))

*The three boards, their special rules, node tracks and capstones. The powers, the boards' insight conversions, and hardening are in [ai-powers.md](ai-powers.md); terms and conventions in [README.md](README.md).*

Each AI board has a **special rule**, a printed **income**, **printed tags**, two **node tracks** (economy and
card-play), building slots (`ai.slots`) and starting credits (`ai.start_credits`), config lists by board so a board
has the same in every seating, and its **powers**. The first token starts on the economy track, its first space
uncovered from setup; placing a node takes the next token off a track, uncovering what is printed under it. Uncovered values add to the income every round (insight at the start of the round, the rest in
upkeep). The board module owns nodes and counts the tokens taken (`board.track.<seat>`); the AI board reads it.

**The boards, for new players.** Each starts with 2 tier-1 and 2 tier-2 cards, as every seat does (the tier-2s point a
build direction), Dispatches 1 ambassador free each round (the Researcher 2; more from the fourth economy space), and
counts its printed tag like one on its cards; its powers are in [ai-powers.md](ai-powers.md). The **Builder**'s special
rule is Breadth: five building slots, a wider engine. It starts with 27 credits and earns 10 a round, each credit space
of its economy track 1 or 2 more, so placing a node is how its income grows; the energy of the cities beside its nodes (1 from each free,
unblocked city with its node on or next to it), its Growth's steps and Sprawl spend. Its powers turn
finished cards into chips or a building played free (Scrap: Industry), energy or antimatter (Overcharge: Power), and
chips into troops removed (Fortify).

The **Protector**'s special rule is Wall: no adversary troop is ever placed on a hex with its node, Redeploys onto it or
strikes it, so no troop strikes its node or a facility under it (threats and corruption still can). When troops Strike, each troop on or next to its nodes pays it 1 insight, at most 2 a round
(Vigil). Its Growth places 2 nodes for 1 energy (Outpost), 1 with an ambassador and 2 credits for 1 energy (Accord), or 1 and a credit free (Levy);
it pushes troops and agents and hardens with energy (Patrol), and its board prints 2 energy a round. It earns 9 credits
a round, so starts with 31. Each credit space of its economy track adds 1 or 2 credits, the
first one 1 energy too. Late in the game, a sacrificed Security card removes troops, two of them agents too (Interdict).

The **Researcher**'s special rule is Patience: insight it doesn't spend stays into the next round (all of it, even
insight another player gave it), to save up for a big round. It starts with 27 credits and earns 7 a round, 2
more for each credit space of its economy track, and plays off the board, on insight and research cubes: with Science
on its cards, Lab turns insight into a card (its draw) and credits into megaproject cubes, and Cyber cards become troops
removed or compute stripped (Hack). Its Growth runs on insight, never research cubes (Matt, 2026-11-16): Peer Review
places 2 nodes and 2 credits for 1, Field Station a station and a node for 1, Field Survey a node and a card for nothing, so it
builds stations early: once their country's card completes they put cubes on its board. Its printed Science tag makes 1 Science card count 2 for a discount. The hard
part to explain: Open Science's insight is handed out in upkeep and lands at the start of the next round.

**Board tags: one tag rule (candidate).** A board's printed tag and a tag a track space reveals count like any tag on
your cards (requirements, tag discounts, income per tag, a capstone's count), except that they can't be sacrificed: a
sacrifice discards a card. Revealed tags are rare, late and costly: two a board, on space 6 (cost 3), placed about round 6.

**Dispatches (candidate).** There is no paid Dispatch. Each round you have your board's **Dispatches**: its printed
number plus 1 for each uncovered Dispatch space, each a free Dispatch (one of your ambassadors from your supply or
anywhere on the map onto a piece or botnet's hex in reach). Unused ones are lost; a space counts once
uncovered. Cards that move ambassadors (Liaison Corps, Summit Caucus) add to it. Economy space 4 is reached about
round 2-3. **The Researcher is the Dispatch-heavy board:** its own ambassador on one of its spread-out stations armors
it and keeps agents off, which would block its cubes.

**Give / get** (`values.json`): a Dispatch is about 0.93 a round (`movement`); space 6 (3 energy, 9) pays back in
about ten rounds, so it is taken on the way to the capstone. A printed tag is free.

**Capstones (candidate).** Each track ends in a capstone, a standing rule once uncovered, as a card's Effect is in
play: what a seat pushes for, reached about round 8 at 1-2 placements a round.

<!-- generated: boards -->
| Board | Printed income | Starting credits | Slots | Printed tags | Dispatches | Economy track | Card-play track | Tokens in all |
|---|---|---|---|---|---|---|---|---|
| Builder | 10 credits, 1 energy | 27 | 5 | Industry | 1 (up to 2) | 7 | 7 | 14 |
| Protector | 9 credits, 2 energy | 31 | 4 | Security | 1 (up to 2) | 7 | 7 | 14 |
| Researcher | 7 credits, 1 energy | 27 | 4 | Science | 2 (up to 4) | 7 | 7 | 14 |
<!-- /generated -->

**The starting-economy scaffold (candidate).** Until engines carry the economy, the boards start and earn much more,
and the economy track grows a seat's income; it comes down as engines take over (3 energy to start,
`ai.start_energy`). **Why:** a board at 27 with 4 cards in hand and round 1's keeps has 3-4 plays at a tier-1 median
of 5 credits; credits never buy a keep, so insight limits round 1's keeps and credits its plays. A credit space pays
1-2 a round for its step (0-3): the early ones pay back in a round or two, the cost-3 one in about three, so the last
are a late-game choice.

**One payment path (locked).** Every space prints a **cost** in energy, the last ones more. Every placement pays it,
whatever places the node: a Growth option costs only its printed price (Buy In nothing) and a card only its
credits, so the energy is stated once, on the track; no card places a node free. Only tracks whose next step you can
pay are offered; a placement you can't pay doesn't happen. A Growth option is used for its node: with no token left,
no step you can pay or no free hex in its range, it isn't offered (r122).

**Hard to explain to a new player:** Faraday Yard hardens when played, beside Fortify II's and Patrol II's hardening; a city beside your node pays you but isn't yours.

**Growth (Matt, 2026-11-18; options candidate).** There is no standard action or other route for nodes. Each round a
board picks **one** of its three printed Growth options ([ai-powers.md](ai-powers.md)), Spirit Island's "pick one":
**one easy option and two costly ones** (Matt, 2026-11-18). The easy one is free, a node and a small rider (Buy In and
Levy a credit, Field Survey a card): the floor, never the best pick when you can pay. The costly two pull different
ways: two nodes for the board's own currency (Sprawl and Outpost 1 energy, Peer Review 2 nodes and 2 credits for 1 insight) against a rider or
reach (Annex, 1 chip: a node within 2 and 6 credits; Accord, 1 energy: a node, an ambassador and 2 credits; Field
Station, 1 insight: a station and a node). So 0-2 nodes a round come from Growth (cards that place nodes add to that: cards aren't
capped). Two things keep the loops short: one option a round and rising step costs cap energy → node → energy
income, and the two-node options spend what the board's engine makes.

<!-- generated: node_track -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| 1 (on the board at setup) | toward the capstone | toward the capstone | toward the capstone |
| 2 | +2 credits | +1 energy, +1 credit | +2 credits |
| 3 | +1 credit | +2 credits | builds a free research station in reach |
| 4 | pay 1 energy: +2 credits, **Dispatch +1** | pay 1 energy: +2 credits, **Dispatch +1** | pay 1 energy: +2 credits, **Dispatch +1** |
| 5 | pay 2 energy: +2 credits | pay 2 energy: +2 credits | pay 2 energy: +2 credits |
| 6 | pay 3 energy: +2 credits, Power tag | pay 3 energy: +1 credit, Cyber tag | pay 3 energy: +2 credits, Power tag |
| 7 | pay 5 energy: **Full Capacity** (capstone): 1 chip each round per 4 Industry tags on your cards. | pay 5 energy: 2 antimatter once, as it is uncovered, **Sanctuary** (capstone): Neither corruption nor Agents destroy your Nodes. | pay 5 energy: 2 antimatter once, as it is uncovered, **Open Archive** (capstone): Each round, 1 research cube onto your board. |
<!-- /generated -->

The **keep upgrade**: tier-2 keeps cost `keep_tier2_upgraded` instead of `keep_tier2` ([player-cards.md](player-cards.md)).

**The card-play track (draft candidates).** None of it is uncovered at setup. A placement takes the next token from
the track the player chooses (asked when both have tokens). **Insight prints here only, as totals** (Spirit Island
style): the printed start shows 3 and a space that raises it shows the new total; your insight is the number on the
rightmost space you've uncovered that shows one. **Each board grows it at its own rate** (income is the lever, as
Spirit Island's energy), **early-heavy, late-flat** (table below): a jump on the first space, then 1 a step, so a
tier-2 keep (3 insight) fits from round 2.

<!-- generated: node_track2 -->
| Space | Builder | Protector | Researcher |
|---|---|---|---|
| printed start | insight 3 | insight 3 | insight 3 |
| 1 | **insight 7** | **insight 7** | **insight 7** |
| 2 | **keep upgrade** | **keep upgrade** | **Dispatch +1** |
| 3 | pay 1 energy: **insight 8** | pay 1 energy: **insight 8** | pay 1 energy: **keep upgrade** |
| 4 | pay 2 energy: +1 credit | pay 2 energy: +1 credit | pay 2 energy: **insight 8** |
| 5 | pay 2 energy: toward the capstone | pay 2 energy: toward the capstone | pay 2 energy: toward the capstone |
| 6 | pay 3 energy: **insight 9**, Cyber tag | pay 3 energy: Diplomacy tag | pay 3 energy: +1 credit, Industry tag |
| 7 | pay 5 energy: **insight 10**, **Master Plan** (capstone): Your buildings cost 1 credit less. | pay 5 energy: **insight 9**, **Aegis Doctrine** (capstone): Once a round, the first time one of your Nodes or facilities would be destroyed, it is sheltered. | pay 5 energy: **insight 9**, **Open Science** (capstone): Each round, 2 insight from the supply, split among other players. |
<!-- /generated -->

**Loops and caps.** No limits ([economy.md](economy.md), *No limits*): a runaway is fixed at the cards that feed it.
Program credit income stacked by draw chains is bounded by insight (as there) and the per-tag programs' 1 a 3 tags.
No power turns credits into chips (Mint is cut, Matt 2026-11-18): chips come from owned factories (`factory_chips` a round), Scrap I and II, Full Capacity and cards, each capped a round, and can't be sold back; stations, Fortify, Annex and the chip converters spend them. Scrap II (once a round) spends two Industry cards for good, so the cards cap
it. **The energy gift** (Shared Grid Accord, Interconnector Treaty) is one-shot. No action turns credits into energy; cities' energy
([economy.md](economy.md)) is capped by the cities beside your nodes, `corp_energy` each a round. **Energy counters** spend energy for good. **The Researcher's:** Patience with
the cube converters (Cintamani's 2 insight, once a round; the standard action's 1, once a round) is the intended engine,
capped by how many stations make cubes (and Open Archive's and Cryo Sample Vault's 1 a round) and Cintamani's once a
round.
**Antimatter:** a plant's Action (a resource for 1) is once a round, beside the Builder's Overcharge II, two economy capstones' 2 (once) and the desperate
standard action (5 cubes for 1 and 2 energy, 262; Horizon's converters pay energy since 262); it pays for the majors that
print it ([economy.md](economy.md)). Patience banks all the Researcher's unspent
insight, gifts too; Open Science's goes only to other players. Followers → ambassadors → alignment → city
energy → node is capped by building count and the Followers each ambassador costs. Remove agents places nothing,
capped by the supply. **Dispatches** have no loop: a number a round, used up, gaining nothing held. Wall has no loop,
but it takes troops' strikes away as a harm where the Protector stands; `docs/harms.md`'s checks count sources, so they hold.

`ai.boards` (`[0, 1, 2]`: Builder, Protector, Researcher) gives each seat its board, repeating for more seats (4
players repeat the Builder). A board has no unique cards or starting building: content printing `cards` or `start` on
one is refused.

<!-- generated: board_details -->
| Board | Special rule |
|---|---|
| Builder | *Breadth*: You have 5 building slots; the Protector and the Researcher have 4. |
| Protector | *Wall*: No adversary Troop is placed on, Redeploys onto or strikes a hex with your Node. Vigil: when Troops Strike, gain 1 insight for each Troop on or next to your Nodes (at most 2 a round). |
| Researcher | *Patience*: All your unspent insight carries over to the next round, including insight other players gave you. |
<!-- /generated -->
