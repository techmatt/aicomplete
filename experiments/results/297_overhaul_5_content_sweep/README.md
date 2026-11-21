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
| Event cards | 16 | 12 carried up to two lines beyond Infiltrate and escalate | 7 | 9 had one line or none |
| Threats | 57 (25 + 32) | 9 cube lines under troop names (294) | 8 | Arsenal reads true |
| Projects | 30 | 4 "most agents" picks; 2 troop projects | 7 (Martial Law joins) | Crackdown, Lab Purge count no agents |
| Discord cards | 7 | Firestorm III, Two Fronts, Blight, Offensive | 4 | Offensive keeps its pick (below) |
| Unity cards | 23 | Demilitarized Zone; the ruling's 4-5 Discord removers | 5 | 292's city line is the rule's words |
| Board powers | 21 | Outpost, Accord, Levy; Wall and Vigil (294) | 1 | below |
| Player cards | 191 | none names a removed mechanic; Twin City Compact, Peacekeeping Mission, 5 kept and not played | 2 | below |
| The research row | 1 | its clock (K), its sources | sources only | K 2 and the six slots stay (the screen below) |

## Event cards: an event is Infiltrate X, an icon, and one line at most

The standard is now **at most one other line** (it was two; `content::event_rule_break` and
`drafts/adversary/check.py` refuse a second), so a card reads: threats, where its agents land, a Strike or Redeploy
icon, and at most one of the board's icons. The seven changed (War Drums, Signal Jamming, Fifth Column, Border
War, Total Mobilization, Blitzkrieg, Naval Blockade), old → new: [content_changes.md](content_changes.md), *Event
cards*.

Border War was the one node loss an event printed with no troop behind it; as a Strike it is the same harm where a
troop stands, which the table sees coming. Nothing prints `each_destroy_node` now (the op stays until the card set
freezes).

## Threats: a name says what the card does

294 turned nine troop lines into research cubes and left the names. Four are **renamed** (the id stays, so play
and notes keep): Orbital Strike → **Weapons Test** (2 cubes), Total War → **Weapons Program** (4 cubes), Garrison
State → **Closed City** (3 agents on the city, 3 cubes), Iron Curtain → **Brain Drain** (3 cubes, remove 2 Unity).
Four are **redesigned** to fit their names: Supply Raid, Call-Up, Manhunt, Conscription (each, old → new:
content_changes.md). Arsenal keeps its line. Uncountered worth over its cheapest counter, at `values.json`: 0.50
(Weapons Test) to 1.75 (Conscription, which takes the factory with the anchor agent).

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
| Botnet Conscripts (project, 16; was 11) | a country holding 2+ botnets: its lowest-numbered becomes 1 troop **on its hex** | a country starts with one botnet; Take down a botnet answers it |
| War Plants (project, 22; was 19) | every corrupted factory makes 1 troop on its hex; an ambassador on it stops it | factories fall from stage II or slot 2 |
| Martial Law (project, 24) | 2 troops a country on a corrupted city the players stand by (else they garrison) | the dearest project; it sits filling |
| Standing Army (Discord; was Two Fronts) | I: 1 troop on a corrupted factory; II: 1 a country by the players, as Martial Law; III: 2 | II starts at the third Discord tip |

Firestorm is one verb again (create 2 / 3 / 4 agents), and so is **Offensive**: it deals 1 / 2 / 3 tier-2 threats
onto the node hex with the most agents and troops, and no longer also creates agents (293's line in place of Surge)
or, at strength III, lands the round's Infiltrate again. On the first screen every game with a troop by round 3 at
d2-d3 was Botnet Conscripts turning the starting botnet, hence its condition.

## The Unity cards that remove Discord

A new op, `pull_discord` ("Remove N Discord from the Discord track": up to N, never below 0), printed only inside
a board condition. Four cards, each by a different condition (what each was: [content_changes.md](content_changes.md)):

| Card | Now (strength I / II / III) | The condition |
|---|---|---|
| Volunteer Corps | Remove 1 Discord for each research station or factory a player owns, or free city with a player's node on or next to it (at most 2 / 3 / 4) | what the players hold |
| Safe Corridors | Remove 1 Discord for each hex holding an ambassador (at most 2 / 3 / 4) | ambassadors out on the map |
| Reconciliation Panel | Remove 1 Discord for each free city with no agent on it (at most 2 / 3 / 4) | the cities the players keep clear (a first cut counted corrupted cities: a game still running always has one, so it was no condition) |
| Town Hall Accord | Remove 2 Discord for each city holding an ambassador (at most 1 / 2 / 3) | ambassadors in cities |

No other Unity card touches the Discord track, and Unity swayed still never does. At a sway's worth (2.67) the
four are worth 2.7-16 a resolve; three of them replace richer cards (two of three agent-removals by corrupted
cities, one of five ambassador hand-outs), so the Unity deck is a little poorer in pieces and now answers the
Discord clock. Demilitarized Zone's strengths widen in turn: no Strike on or next to an ambassador; an ambassador
or a node; within 2 of either.

## The research row: its clock and its sources

**The row is unchanged** (New factory · Cells: factories · Funding · Garrison · Strike · Garrison · loss at 7) **and
K stays 2 a country.** Its sources now: corrupted stations (1 cube a round each, about three quarters of all
cubes), eight threats, three projects (Backdoor Research, Lab Seizure, Station Coup), and the Discord card
**Captured Labs** (was Blight): 2 cubes for each corrupted research station, at most 2 / 3 / 4. Five threat cube
lines went with their troop names (10 cubes), and Captured Labs feeds by what the players left corrupted instead.

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

**Outpost** (Protector): 2 energy: place 1 node **within 2 of yours**; gain 2 credits (was a plain node, used 0.05
a seat-round beside Accord at the same price); the reach is the benefit (a shield is kept to cards). **Accord and
Levy are left:** Accord's use is the Liberate bill pulling ambassadors (below), not a pricing fault.

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
- **Typed jam and counter cards** (295's kept games, 576 a sim player): none is dead. Threats dealt are security
  39%, industry 24%, diplomacy 22%, cyber 16%; Garrison Command is used on 0.42-0.50 of the rounds it stands, the
  other Action jams on 0.21-0.33 (no threat of their tag in reach on 61-76% of rounds). Not changed: against the
  Militarist the non-Security jams are thin by the deck's lean.

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

Greedy, fresh: wins 91 → 94, 86 → 92, 78 → 81, 47 → 62% (its other rows: smoke.md). Every game finished; no card
kept in over 1% of games and never played; design_check's kept-not-played list is 4, from 5 at r142 (Breach and
Clear left it).

- **The research row threatens without ending most games:** it reaches slot 4 or later in a fifth to two thirds
  of games by level and ends 1-24% of them, d3 the most; fewer than at r142, so d2 and d3 win more.
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
- **Hedge prices.** Gain a player ambassador and Remove agents stay over their band: item 6's tuning.
- **Lines that stay long.** The garrison line and Botnets spread print their whole rule: a print-pass call.

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
