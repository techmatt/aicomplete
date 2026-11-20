# 297: overhaul 5 of 7, the content sweep (r142 → r143)

Matt's rulings of 2026-11-19 for this item: the sweep covers every card, threat, project, Discord card and Unity
card that names something removed or that items 1-4 patched; about 4-5 of the 23 Unity cards remove Discord, each
by a board condition; troop content for projects and Discord cards, late and significant; the research row is fed
by content; 7-10 rounds the rough aim. Rules revision 142 to 143, bumped from the constant (the content changes
play; so do four project picks and three small op changes). Everything here is a candidate for his review.

- Every changed card, old → new, by deck, to read beside the card viewer: [content_changes.md](content_changes.md)
  (written by [`scripts/content/sweep_297.py`](../../../scripts/content/sweep_297.py) and
  [`cards.overlay`](cards.overlay), which rerun from the r142 sources).
- The before-and-after table by pair and level: [smoke.md](smoke.md) (`smoke_table.py` prints it from farmed games).

## What the sweep covers (listed before anything changed)

| Deck | Cards | In scope: named something removed, or patched by items 1-4 | Changed | Left, and why |
|---|---|---|---|---|
| Event cards | 16 | 12 carried up to two lines beyond Infiltrate and escalate | 7 | 9: four were already Infiltrate X (+ Strike or Redeploy) alone; five keep one line |
| Threats | 57 (25 + 32) | 9 cube lines under troop names (294) | 8 | Arsenal: an arms plant feeding weapons research reads true |
| Projects | 30 | 4 "most agents" picks; 2 troop projects (294's numbers) | 7 (Martial Law joins as a troop project) | Crackdown and Lab Purge count ambassadors and cubes, not agents |
| Discord cards | 7 | Firestorm III (294's troop rider); Two Fronts (fronts went with war); Blight (Fallout's old name, Firestorm with a rider); Offensive (293's creation line in place of Surge) | 4 | Offensive keeps its pick (below) |
| Unity cards | 23 | Demilitarized Zone (strengths II and III the same line, 294); the ruling's 4-5 Discord removers | 5 | 8 cards carry 292's "free city with a player's Node on or next to it": the rule's own words now |
| Board powers | 21 | Outpost (barely used), Accord, Levy; Wall and Vigil (redone by 294) | 1 | below |
| Player cards | 191 in the deck | none names a removed mechanic (searched: war, base, front, march, muster, host, Dispatch, Growth, purge); Twin City Compact (292), Peacekeeping Mission (294's note), 5 kept and not played | 2 | below |
| The research row | 1 | its clock (K), its sources | sources only | K 2 and the six slots stay (the screen below) |

## Event cards: an event is Infiltrate X, an icon, and one line at most

The standard is now **at most one other line** (it was two; `content::event_rule_break` and
`drafts/adversary/check.py` refuse a second), so a card reads: threats, where its agents land, a Strike or Redeploy
icon, and at most one of the board's icons.

| Stage | Card | Reads now |
|---|---|---|
| I | Probing Strikes | Infiltrate: coastal cities |
| I | Factory Drive | Infiltrate: coastal cities · New factory |
| I | Long Patrols | Infiltrate: cities · Botnets spread |
| I | War Drums | Infiltrate: cities (was + Sway 1 toward Discord) |
| I | Signal Jamming | Infiltrate: botnet hexes · Funding (was + Botnets spread) |
| II | Mobilization | Infiltrate: coastal cities · **Strike** |
| II | Station Raids | Infiltrate: research stations · Botnets spread |
| II | Shell Labs | Infiltrate: research stations · New lab |
| II | Fifth Column | Infiltrate: cities · **Strike** (was + Botnets spread) |
| II | Border War | Infiltrate: factories · **Strike** (was: each player with a node by a factory destroys one) |
| II | Supply Lines | Infiltrate: factories · Funding |
| II | Redeployment | Infiltrate: cities · **Redeploy** |
| III | Total Mobilization | Infiltrate: factories, cities · **Strike** (was + Remove 2 Unity) |
| III | Blitzkrieg | Infiltrate: cities, research stations · **Strike** · Fallout (was + Sway 1) |
| III | Occupation | Infiltrate: research stations, cities · **Strike** · **Redeploy** |
| III | Naval Blockade | Infiltrate: coastal cities, factories · **Strike** · Blackout (was + Fallout) |

Border War was the one node loss an event printed with no troop behind it; as a Strike it is the same harm where a
troop stands, which the table sees coming. Nothing prints `each_destroy_node` now (the op stays until the card set
freezes).

## Threats: a name says what the card does

294 turned nine troop lines into research cubes and left the names. Four are **renamed** (the id stays, so play
and notes keep): Orbital Strike → **Weapons Test** (2 cubes), Total War → **Weapons Program** (4 cubes), Garrison
State → **Closed City** (3 agents on the city, 3 cubes), Iron Curtain → **Brain Drain** (3 cubes, remove 2 Unity).
Four are **redesigned** to fit their names: Supply Raid (pays 2 chips or discards 1 card; the adversary gains 4
compute), Call-Up (1 agent created here; 1 Fallout), Manhunt (pays 3 ambassadors or discards 3 cards; 2 agents
created here), Conscription (2 agents created here; pays 2 chips or discards 2 cards). Arsenal keeps its line.
Uncountered worth over its cheapest counter, at `values.json` (the threat standard's give / get): Weapons Test
0.50, Supply Raid 0.78, Call-Up 1.17, Weapons Program 0.73, Manhunt 1.15, Conscription 1.75 (it takes the factory
with the anchor agent), Closed City 0.94, Brain Drain 1.03.

## Projects

- **No "most agents" pick is left.** Asset Seizure and Station Coup take the country's lowest-numbered neutral
  factory or station (with none neutral, the lowest-numbered player-owned one turns neutral); Sabotage Teams strips
  the lowest-numbered hardened facility; Night Raid adds 1 agent to each of the country's two lowest-numbered hexes
  **holding a player node and an agent**, so the nodes at risk are the ones an agent already stands on, and it no
  longer seeds a clean node.
- **Troop projects, each by something the table sees** (below): Botnet Conscripts, War Plants, Martial Law.

## Troops: where they come from now

| Source | Line | Why it is late and seen coming |
|---|---|---|
| Research row, slots 4 and 6 | 1 troop a country garrisons | the marker and its cubes are public |
| Botnet Conscripts (project, 16 a board; was 11) | each country holding 2 or more botnets: its lowest-numbered botnet becomes 1 troop **on its hex** (was 2 troops that garrison) | a country starts with one botnet; the second comes from Botnets spread or a threat, and Take down a botnet answers it |
| War Plants (project, 22; was 19) | every corrupted factory makes 1 troop on its hex; an ambassador on it stops it (it no longer builds a factory first) | factories fall from stage II or the row's slot 2; liberate or stand on them |
| Martial Law (project, 24) | each country: 2 troops land on its lowest-numbered corrupted city with a node or an ambassador on or next to it (with none, they garrison) (was 3 agents on a research station) | the dearest project; it sits in its slot filling |
| Standing Army (Discord; was Two Fronts) | I: 1 troop on each country's lowest-numbered corrupted factory, none where no factory has fallen; II: 1 a country lands by the players as Martial Law's; III: 2 | strength II starts at the third Discord tip |

Firestorm is one verb again (create 2 / 3 / 4 agents), and so is **Offensive**: it deals 1 / 2 / 3 tier-2 threats
onto the node hex with the most agents and troops, and no longer also creates agents (293's line in place of Surge)
or, at strength III, lands the round's Infiltrate again. On the first screen every game with a troop by round 3 at
d2-d3 was Botnet Conscripts turning the starting botnet, hence its condition.

## The Unity cards that remove Discord

A new op, `pull_discord` ("Remove N Discord from the Discord track": up to N, never below 0), printed only inside
a board condition. Four cards, each by a different condition:

| Card | Was | Now (strength I / II / III) | The condition |
|---|---|---|---|
| Volunteer Corps | Sway 1 toward Unity for each site a player holds (at most 2 / 3 / 4) | Remove 1 Discord for each research station or factory a player owns, or free city with a player's node on or next to it (at most 2 / 3 / 4) | what the players hold |
| Safe Corridors | every player gains 2 ambassadors for each such site (at most 2 / 3 / 4) | Remove 1 Discord for each hex holding an ambassador (at most 2 / 3 / 4) | ambassadors out on the map |
| Reconciliation Panel | remove 1 / 2 / 3 agents from each hex by a corrupted city | Remove 1 Discord for each free city with no agent on it (at most 2 / 3 / 4) | the cities the players keep clear (a first cut counted corrupted cities: a game still running always has one, so it was no condition) |
| Town Hall Accord | each player removes 3 / 4 / 5 agents from one hex by a corrupted city | Remove 2 Discord for each city holding an ambassador (at most 1 / 2 / 3) | ambassadors in cities |

No other Unity card touches the Discord track, and Unity swayed still never does. At a sway's worth (2.67) the
four are worth 2.7-16 a resolve; three of them replace richer cards (two of three agent-removals by corrupted
cities, one of five ambassador hand-outs), so the Unity deck is a little poorer in pieces and now answers the
Discord clock. Demilitarized Zone's strengths widen in turn: no Strike on or next to an ambassador; an ambassador
or a node; within 2 of either.

## The research row: its clock and its sources

**The row is unchanged** (New factory · Cells: factories · Funding · Garrison · Strike · Garrison · loss at 7) **and
K stays 2 a country.** Its sources now: corrupted stations (1 cube a round each, about three quarters of all
cubes), eight threats (Weapons Test 2, Wiper Malware 1; Weapons Program 4, Zero Day 4, Closed City 3, Brain Drain
3, Arsenal 3, Data Breach 2), three projects (Backdoor Research, Lab Seizure, Station Coup: 2 a country; Station
Coup also opens the adversary a station a country), and the Discord card **Captured Labs** (was Blight): 2 cubes
for each corrupted research station, at most 2 / 3 / 4. Five threat cube lines went with their troop names (10
cubes across the two decks), and Captured Labs feeds by what the players left corrupted instead.

The K screen (the judge, dev seeds 36800-36831, 96 games a cell, the first cut of the content):

| Level | K 2 (asked): wins, slot-7 losses, games at slot 4+ | K 3: wins, slot-7 losses, games at slot 4+ |
|---|---|---|
| d0 | 89%, 3, 15 | 90%, 0, 2 |
| d1 | 90%, 1, 27 | 85%, 0, 5 |
| d2 | 80%, 12, 31 | 76%, 3, 16 |
| d3 | 56%, 23, 47 | 69%, 5, 31 |

At K 3 the row stops being a threat below d3 and its Garrison slots nearly never arrive (troops from the row: 0.03
a game at d0-d1); at K 2 it ends a quarter of d3 games and few below. K 2 meets the smoke bar ("threatens without
ending most games at any level"), is the ruling's number, and keeps the row a troop source, so no longer row was
built.

## Board powers

**Outpost** (Protector): 2 energy: place 1 node **within 2 of yours**; gain 2 credits (was Sprawl's line, a plain
node and 2 credits, used 0.05 a seat-round because Accord gives an ambassador at the same price). Give / get: 6
for a node 5.33 and 2 credits, +1.33 as before; the reach is the benefit. A shield was the first idea; the content
check keeps shields to cards. **Accord and Levy are left:** Accord's use above the free Levy is the Liberate bill
pulling ambassadors (below), not a pricing fault of either.

## Player cards

- **Breach and Clear** (tier 1 event, 2 cr, 1 insight): "Target: Agents or Troops. Remove 3 Agents or 2 Troops on
  it." Pay 2 + 2.67 × 2 = 7.33; get 10 (agents) or 6.67 (troops): 1.36× / 0.91×. It was 1 troop, 0.45×, and no card
  at all without a troop in reach.
- **Peacekeeping Mission** (tier 2 event, 13 cr, 1 antimatter): "Remove 3 Troops on or next to it. Rescue it." The
  troop half now opens its own target: it is the one Rescue that may target a guarded city, and rescues it when its
  removal leaves no guard. Pay 13 + 18 + 8 = 39; get a city 30, 1.5 troops 5, the claim rider 6 = 41, 1.05×.
- **Left:** Twin City Compact (292 gave it a give / get, 1.84×, and it reads as one thing); Multiyear Defense
  Budget (1.15×) and Moonshot Grant (1.07×) are fair and dear, kept by a judge that then can't afford them;
  Industrial Commons and Open Research Campus need a free zone or an empty station site in reach and are kept
  without one: the keep plan's, for item 6.
- **Typed jam and counter cards** (`out/.../jam/jam_read.md`'s read of 295's kept games, 576 a sim player): no
  typed card is dead. Security is 39% of threats dealt, cyber 16%, industry 24%, diplomacy 22%; Garrison Command
  (security) is used on 0.42-0.50 of the rounds it stands, the three other Action jams (Sentinel Mesh, Market
  Watchdog, Civic Watch) on 0.21-0.33, with no threat of their tag in reach on 61-76% of rounds. The typed programs
  are played 0.16-0.44 a game, about half the time for their second effect. Not changed here: against the
  Militarist the non-Security jams are thin by the deck's lean, which another adversary would reverse.

## The smoke read (every table: [smoke.md](smoke.md))

Two players, three pairs pooled, 144 games a cell; r142 at 2282b5387d against the final build 02ee710e. The
same-seed read (50424-50471, r142's side 295's kept games) shaped the content; the fresh read (50640-50687, both
sides farmed here) saw no change after it.

| Fresh, the judge | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| Wins r142 → r143 | 87 → 92% | 81 → 86% | 65 → 77% | 49 → 65% |
| Rounds | 8.4 → 8.1 | 8.8 → 8.7 | 8.4 → 8.6 | 8.0 → 8.4 |
| Won by round 3 | 2 → 1 | 0 → 0 | 0 → 0 | 0 → 0 |
| Ended by slot 7 | 5 → 2 | 11 → 4 | 39 → 20 | 56 → 34 |
| Games at slot 4+ | 55 → 28 | 68 → 48 | 96 → 82 | 116 → 93 |
| First troop, round; games with one by round 3 | 7.0; 0 → 6.2; 4 | 6.5; 4 → 5.7; 7 | 5.8; 6 → 5.3; 17 | 4.8; 23 → 4.8; 23 |
| Strike hits a game | 0.11 → 0.10 | 0.12 → 0.08 | 0.22 → 0.37 | 0.44 → 0.55 |
| Redeploy: troops moved a game | 0.02 → 0.00 | 0.00 → 0.00 | 0.00 → 0.01 | 0.04 → 0.01 |
| Discord taken off by Unity cards a game | 0 → 2.5 | 0 → 2.7 | 0 → 2.8 | 0 → 2.6 |

Greedy, fresh: wins 91 → 94, 86 → 92, 78 → 81, 47 → 62%, rounds 8.0-8.8 → 8.2-8.8, slot 7 ending 2 → 0, 13 → 1,
28 → 16, 58 → 30. Every game finished; agents on the board peak at 15-18 a game (r142 16-20), Discord swayed
22-39 (23-41). Cards kept and not played: none kept in over 1% of games and never played; design_check's
kept-not-played list is 4 (Cryptojacking Campaign 0.08, Open Research Campus 0.06, Multiyear Defense Budget 0.05,
one more), from 5 at r142; Breach and Clear left it.

- **The research row threatens without ending most games:** it reaches slot 4 or later in a fifth to two thirds
  of games by level and ends 1-24% of them, d3 the most. Fewer games end on it than at r142 (five threat cube lines
  went with their troop names), so d2 and d3 win more; the row still decides a quarter of the d3 games it ends.
- **Troops:** the same-seed read took games with a troop by round 3 at d2 / d3 from 16 / 24 to 7 / 8. **The fresh
  read does not confirm it:** 6 → 17 at d2 and 23 → 23 at d3, mostly Martial Law resolving at round 3 when d2-d3's
  compute income fills it (11 and 15 games), and at d0-d1 Standing Army I where a factory fell early (4 and 7). The
  same-seed gain was a seed effect; "late" needs a rule (below, 3), not a price.
- **Strike** hits more at d2-d3 (0.37 and 0.55 a game, from 0.22 and 0.44) as troops land by the players; at d0-d1
  it stays rare. **Redeploy** stays inert. Neither got real work from content alone (below, 1 and 2).
- **The comparison changes everything in this item at once** (event lines, threats, projects, Unity, Discord,
  Outpost, two player cards): no row above is credited to one card.

## What was not redesigned, and why

- **Offensive's pick.** The card is one verb now (below), but still aims at "the Node hex with the most Agents and
  Troops": the last "most" pick in content. It is what the card is (it finds where the players are weakest), and
  the ruling named the four projects; the lowest-numbered node hex holding an agent or a troop is the swap if Matt
  wants none left.
- **The Militarist's level 3, Arms Race,** still has a name and no rule: ladder work is item 6's.
- **Hedge prices.** Gain a player ambassador and Remove agents stay over their band (below); a price move there is
  a tuning read on a judge item 6 reteaches.
- **Lines that stay long.** The garrison line and Botnets spread print their whole rule; an icon with the rule on
  the aid would shorten six cards, a print-pass call.

## What needs a rule change, not content (none built)

1. **Strike rarely hits.** Content now lands troops beside the players (Botnet Conscripts, Martial Law, Standing
   Army II and III) and adds a third stage-II Strike, and hits stay rare: the answer is cheap and always there
   (Remove a troop, 7 credits), and the Protector's Wall makes its nodes and their hexes immune in two of the three
   pairs. What would change it: a Strike that also sends home one ambassador on or next to the troop (the piece a
   liberation stages), or Remove a troop priced by calibration as Liberate was.
2. **Redeploy stays inert.** A garrison is placed on a corrupted site by rule, and the new placements put troops
   on corrupted cities, so a troop is almost never away from one when the icon shows. What would give it work:
   Redeploy moving each troop onto the corrupted city nearest a player piece; or cutting the icon.
3. **Late troops from projects.** At d3 the compute income resolves a 16-24 project by round 3 whatever it costs;
   only the conditions hold troops back. A stage on the project deck (troop projects under the stage-II cards), or
   "from stage II" on a card, would make "late" a rule.
4. **Supply ambassadors beside ambassadors as pieces.** Liberate's bill (5 from the supply, used up, while one
   piece stands on the city), Remove agents (2 from the supply) and card costs still spend supply ambassadors. It
   reads as two things under one name: the piece that moves and acts, and a currency. It is also why Gain a player
   ambassador and Accord run hot. Options: pay the bill only in pieces standing on the site; or name the supply
   currency apart.
