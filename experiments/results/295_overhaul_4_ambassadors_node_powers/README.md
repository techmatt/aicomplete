# 295: overhaul 4 of 7, ambassadors and node powers (r142)

Matt's rulings of 2026-11-19 for this item: an ambassador has one action a round (move to a hex next to it, or to
any of your nodes; or act); it arrives ready (to try); Dispatches go, and no board gets more; liberation is move in,
then act, its price set by calibration; Growth becomes ordinary node powers (one cheap, one or two costly with a
benefit, usable in the same round); a round-2 or round-3 win is rare, the levers being the liberation price and the
starting corrupted cities; 7-10 rounds the rough aim. Rules revision 141 to 142, bumped from the constant.

Tables: [read.md](read.md) (the before-and-after read), [levers.md](levers.md) (the lever screens);
`early_table.py` makes both from farmed games.

## What was built

**An ambassador's round.** Each of your ambassadors on the map does one thing a round, then is tipped over until
the round ends: it **moves** (onto a land hex next to it, or onto any of your nodes from anywhere), or it **acts**
where it stands (Liberate the site; Take down a botnet). An ambassador in your supply comes onto one of your nodes
with the Move action and **arrives ready**, so it can still move or act that round. Blocking, armor and the
one-for-one meeting with agents work as before, tipped or not. A card that moves an ambassador is not that piece's
own action: it stays as it was, a tipped one tipped; one a card places from a supply or converts arrives ready.
When one of several leaves a hex, a ready one leaves first. The table sees all of it: the piece is tipped.

**Liberation.** Liberate is the act of one of your ready ambassadors standing on the corrupted site, so the plain
route is: move an ambassador onto the city one round, liberate the next. The bill is as it was (5 ambassadors for a
city, 4 a station, 3 a factory, used up: yours on the site first, the acting one among them, then your supply). It
needs no reach: the piece is there. No agent may be on the site and no troop on or next to it, as before.
**294's gap** (a guard next to the city, two hexes from the liberator's node, out of Remove a troop's reach) is
closed by one line: a troop on or next to one of your ambassadors is in your reach for anything that targets troops.

**Dispatches** are gone: no board prints one, no track space adds one. Economy space 4 keeps its 2 credits on every
board; the Researcher's card-play space 2, which printed only Dispatch +1, prints +1 credit.

**Node powers, old to new.** No Growth and no pick-one: three ordinary powers a board, each once a round, all usable
in the same round, each placing one node. Give / get at the unit anchors (energy 3, chip 4, insight 2.67, ambassador
5.33, a node 5.33, a station 6.67), before each track step's energy:

| Board | Power | r141 (Growth, pick one) | r142 | Net at the anchors, old to new |
|---|---|---|---|---|
| Builder | Buy In | free: 1 node, 1 credit | unchanged | +6.33 |
| Builder | Sprawl | 1 energy: 2 nodes | 2 energy: 1 node, 2 credits | +7.67 to +1.33 |
| Builder | Annex | 1 chip: 1 node within 2, 6 credits | 1 chip: 1 node within 2, 3 credits | +7.33 to +4.33 |
| Protector | Levy | free: 1 node, 1 credit | unchanged | +6.33 |
| Protector | Outpost | 1 energy: 2 nodes | 2 energy: 1 node, 2 credits | +7.67 to +1.33 |
| Protector | Accord | 1 energy: 1 node, 1 ambassador, 2 credits | 2 energy: 1 node, 1 ambassador | +9.67 to +4.67 |
| Researcher | Field Survey | free: 1 node, 1 card | unchanged | +8.67 |
| Researcher | Peer Review | 1 insight: 2 nodes, 2 credits | 2 insight: 1 node, 1 research cube | +10.0 to +4.0 |
| Researcher | Field Station | 1 insight: a station (no chips), 1 node | 2 insight: the same | +9.33 to +6.67 |

The cheap power with one costly one is about the old two-node option (Buy In + Sprawl: 2 nodes and 3 credits for 2
energy, +7.67, the old Sprawl's net). A board that pays for all three gets more than its old best: Builder +12.0
(was +7.67), Protector +12.33 (+9.67), Researcher +19.33 (+10.0); the track steps' energy is what holds it. Used a
seat-round on the first screen: the free one 0.47-0.69, Accord and Field Station the most used costly ones, the
Protector's Outpost 0.05 (the band's floor).

**Adjacent changes.** `board.start_corrupted`'s range ends at 2, the cities a country prints corrupted (3 placed 2,
silently; d4's `adversary.extra_corrupted` adds nothing while the start is 2). The view's seat value `movement`
is `ambassadors_ready`. `w.movement` in the agents' weights is inert. Nothing in `content/cards.json` named Dispatch,
Growth or the old Liberate as a rule (the card Dispatch Control Room keeps its name), so no card text changed.

## The levers

d0, the judge, the three pairs pooled, 96 games a cell on the dev seeds (36800-36831), the shipped build
([levers.md](levers.md) has the first screen, the pairs and d1-d3):

| Cell | Wins | Rounds | Won by round 3 | Won by round 5 | Fastest win |
|---|---|---|---|---|---|
| asked: Liberate 5, 1 corrupted city a country | 100% | 5.9 | 23% | 49% | round 1 |
| **2 corrupted cities a country (the pick)** | 90% | 8.1 | 0% | 14% | round 4 |
| Liberate 6 | 94% | 6.4 | 19% | 40% | round 2 |
| Liberate 7 | 95% | 7.0 | 6% | 26% | round 2 |
| 2 cities and Liberate 6 | 88% | 9.1 | 0% | 7% | round 4 |
| arrives spent | 97% | 6.1 | 20% | 44% | round 2 |
| arrives spent, 2 cities | 82% | 8.9 | 0% | 11% | round 5 |
| arrives spent, 2 cities, Liberate 6 | 84% | 9.4 | 0% | 5% | round 4 |

- **The starting cities are the lever that meets the bar**; the price alone does not at 6 and nearly does at 7 (6%
  by round 3, with wins in round 2). Together they add about a round and cost wins above d0.
- **Move in, then act is no brake by itself**: at the asked numbers 23% of d0 games are still won by round 3 (r141:
  20%), the fastest in round 1, because an ambassador arrives ready on a node that stands on or next to the city.
- **Picked: `start_corrupted` 2, `liberate_at` 5, arrives ready**, then confirmed at the size a retuned value asks
  for (tuning-items.md): 504 games a cell on fresh seeds 50472-50639, d0, the same seeds across cells. Asked numbers:
  97% wins, 5.9 rounds, 22% won by round 3 (112 games), fastest round 1. Two cities: 92% wins, 8.0 rounds, 0.6% by
  round 3 (3 games), 14% by round 5, fastest round 3. Liberate 7 alone: 93%, 7.3 rounds, 9% by round 3, fastest
  round 1: the price alone misses the bar.
- **Arrives ready: kept.** Arriving spent took wins by round 3 from 23% to 20% alone, and with two cities cost 8
  points of d0 wins (90 to 82) for 0.8 of a round. It is not the brake; the cities are. What it would change at the
  table: one more round before every liberation that starts from the supply, and no card-placed same-round act.

## The before-and-after read (fresh seeds 50424-50471, 144 games a level; [read.md](read.md))

| Level | Wins r141 to r142 | Rounds | Won by round 3 | Won by round 5 | Fastest win | Liberate actions a game | How r142's games ended |
|---|---|---|---|---|---|---|---|
| d0 | 99% to 90% | 5.8 to 8.3 | 20% to 1% (1 game) | 47% to 11% | round 1 to 3 | 2.8 to 4.7 | liberated 130, round cap 10, research 3, Discord deck 1 |
| d1 | 93% to 83% | 7.0 to 9.0 | 10% to 0% | 28% to 9% | round 1 to 5 | 4.8 to 6.0 | liberated 120, round cap 14, research 6, Discord deck 4 |
| d2 | 89% to 67% | 6.8 to 8.7 | 8% to 0% | 24% to 6% | round 1 to 4 | 4.8 to 5.8 | liberated 96, research 24, round cap 14, Discord deck 10 |
| d3 | 82% to 51% | 7.0 to 8.3 | 6% to 0% | 19% to 3% | round 1 to 4 | 5.4 to 5.8 | liberated 73, research 42, round cap 16, Discord deck 13 |

- **The bar is met at d0**: 1 of 144 games won by round 3, 90% won, 8.3 rounds. Every game finished (no timeout).
- **No pile runs away**: agents on the board peak at 18-20 a game (r141 14-16; the most 59), Discord swayed 27-43
  (15-32), research cubes 15-23 (11-20), troops under 1 at the end.
- **d2 and d3 fall below their soft bands** (75-90 and 55-70): the research row reaches slot 7 in 24 and 42 of 144
  games (r141: 12 and 18). Games are 1.3-2.5 rounds longer and the row's clock was set on 6-round games. Part of the
  fall is the rules alone: on the dev seeds the asked numbers under r142 win 100 / 85 / 82 / 65% at d0-d3, two
  cities 90 / 89 / 73 / 50%. Not chased here (no band work in this item).
- **The comparison changes several things at once** (the ambassador rules, the node powers, the starting cities):
  only [levers.md](levers.md) separates the cities.
- **Greedy** on the same seeds wins 94 / 85 / 77 / 44% in 7.9-9.0 rounds, above the judge at d0-d2: the judge's
  liberation terms were written for a Liberate paid from the supply in reach.

## Three seats (a finish check, not a read)

`configs/three_seat.json` is derived from the baseline, so it has two corrupted cities a country too. The judge at
d0 on 16 dev seeds (spec `three295`): every game finishes; 8 of 16 won in 12.4 rounds (6-14), 8 lost to the round
cap; with one city a country 10 of 16 in 10.7 rounds (7-14). No win before round 6 either way: at three seats the
game is long under r142 whatever the lever, and its d0 is well under the two-seat 90%.

## The sim players

A judge that could not liberate would meet the bar for the wrong reason. Checked: every win ends `liberated`; the
judge takes 4.7-6.0 Liberate actions a game (97% of seat-games use it) and makes 18-25 ambassador moves a game
(r141: 9-12 Dispatches); cities liberated a game rise from 3.3-5.9 to 5.2-6.8. Two changes to its eval, no new
heuristic: a seat's saving toward a Liberate bill counts whole once one of its ambassadors stands on the site and
half while it is only in reach (`STAND_OFF`), and the Dispatch income term is gone. The existing block term (an
ambassador on a corrupted city) is what draws the first move in. Its gather rule no longer asks for reach.

## Cost

No timing read: another item ran beside this one throughout. Load-free counts on the paired fresh read: decisions a
round 55.2 to 58.3 (+6%), options a decision 13.0 to 14.8 (+14%), decisions a game 366 to 500 (longer games).
`speed.py` gives no verdict across a rules revision; its baseline was not re-based.

## Diff review

17 findings from five reviewers, each verified by a skeptic, 11 distinct, all fixed: an op after "Target: Troops"
filtered by node reach again and removed nothing from a troop the new reach line offered; "Agents or Troops"
targets lacked that line; Gather and Push landed ambassadors past the ready / tipped bookkeeping, and no removal
trimmed the marker (now one home, `board::take_ambassadors`); a card's move stood a tipped ambassador up again;
the view counted `tipped` markers as pieces; the guard measure and three texts still said reach; the node-and-2-credits
power stood on all three boards (Peer Review now gains a research cube); `never_second_node`'s test; `start_corrupted`
above the printed cities; the screen's spec inherited the moved baseline; the golden constants.
