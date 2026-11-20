# AI Complete: Rulebook (fourth draft)

*Rules revision 146. This book teaches the game as the simulator plays it today. Every number is a candidate. The AI
boards, the threats, the adversary's cards and the Unity and Discord cards are placeholders, so card text is left out
except where an example needs it. Part 1 teaches the game in the order you play it; Part 2 is the reference. Each
section ends with a small **Sources** line for reviewers: the rules files ([rules/](rules/README.md)) and code it
states. A ten-minute spoken version is [teach.md](teach.md); the table's reference sheet is
[player-aid.md](player-aid.md). What is still hard to say, and the rule change that would fix it, is ranked in
[rulebook-friction.md](rulebook-friction.md).*

**Part 1: Learning the game**

1. [The game in brief](#1-the-game-in-brief)
2. [The map](#2-the-map)
3. [Setup](#3-setup)
4. [The round](#4-the-round)
5. [Your AI board](#5-your-ai-board)
6. [The adversary's turn](#6-the-adversarys-turn)
7. [Cards](#7-cards)
8. [The action phase](#8-the-action-phase)
9. [Production](#9-production)
10. [Threats](#10-threats)
11. [Troops](#11-troops)
12. [Unity and Discord](#12-unity-and-discord)
13. [Protecting your pieces](#13-protecting-your-pieces)
14. [Winning and losing](#14-winning-and-losing)

**Part 2: Reference**

15. [Corruption in full](#15-corruption-in-full)
16. [Moving and removing pieces](#16-moving-and-removing-pieces)
17. [Projects](#17-projects)
18. [Threat details](#18-threat-details)
19. [Troops in full](#19-troops-in-full)
20. [Powers and capstones](#20-powers-and-capstones)
21. [Difficulty](#21-difficulty)
22. [Player count and solo play](#22-player-count-and-solo-play)
23. [Reading a card: the icon legend](#23-reading-a-card-the-icon-legend)
24. [Glossary](#24-glossary)

---

# Part 1: Learning the game

## 1. The game in brief

You and up to three other players are good AIs working together against one evil AI, **the adversary**. The adversary
sends its **agents** into the world's cities and facilities. When enough agents crowd onto one, they **corrupt** it
and it becomes the adversary's. Late in the game it raises **troops**, which guard what it holds and strike your
pieces, and it pushes the world toward **Discord**. You spread your **nodes** over the map, send your **ambassadors**
onto corrupted places to take them back, and build a tableau of cards that pays for it all.

Everyone wins or loses together.

- **You win** the moment **no city on the map is corrupted**. Each country starts with 2 corrupted cities: liberate
  every corrupted city on the map (§8). Botnets, agents, troops and corrupted stations or factories never count.
- **You lose** if any of these happens:
  - a Discord tip finds the Discord deck empty. **The Discord deck is a clock**: 7 cards, never reshuffled (§12);
  - any player has no nodes left on the map;
  - the adversary's event deck runs out: 10 rounds at most at levels 0-2, 9 at levels 3-4 (the **round cap**);
  - the adversary's research marker reaches the seventh slot of its research row (§6).

Most games run about 6-8 rounds; a long one reaches the round cap.

**You see the adversary's next move.** Next round's event card is face up all round: the sites its new agents will
land on, and any Strike or Redeploy icon its troops will follow. Last round's card lies face up too: it says which
sites escalate. The compute waiting beside the projects and the research track are public. (The threat, Unity and
Discord decks stay hidden.)

**Team choices.** Where the rules say "the players choose", decide together. If you can't agree, the first player in
player order (§3) decides. A choice inside your own card, power or action is yours.

*Sources: rules/README.md (Terms: players choose), rules/alignment.md (The only win, Losing), rules/adversary.md
(Each round, Research track).*

---

## 2. The map

### Countries and hexes

There is one **country** per player. A country is a square piece of 16 hexes, numbered 1-16 row by row. Each country
also belongs to a **megacorp** (Horizon, Cintamani, Onyx, Biomia), which has its own **megaproject** card (§8). The
megacorps are on nobody's side.

A hex is a **city**, **open** land or **ocean**. Cities and open hexes are **land**. Some open hexes are printed as
**industrial zones**: factories go only there, and each zone prints an **arrow** to its city, the nearest city of its
country, one hex away (§6). A city next to ocean, or printed as coastal, is a **coastal city**. Each country also
prints its cities' **corruption order** (the first two start corrupted) and, in some countries, an **empty station
site** where a card may open a research station. Nothing is ever built on ocean, and no piece enters it.

Two hexes are **next to** each other if they share an edge, across country borders too. "**On or next to** a hex"
means that hex or one next to it. A border is only an edge: nothing sits on it. When the rules break a tie between
hexes, the lower country number goes first, then the lower hex number.

### Sites

**Cities**, **research stations** and **factories** are **sites**. (Research stations and factories are
**facilities**. A **building** is always a card, never a piece on the map.) Each site is in one of three states, shown
by the cube on it:

| State | Cube | A city | A research station | A factory |
|---|---|---|---|---|
| **Neutral** | none | 1 energy to **every** player with a node on or next to it | 1 neutral cube onto its megaproject | nothing |
| **A player's** | that player's | never: **no player owns a city** | 1 cube in its owner's color | 1 chip to its owner |
| **Corrupted** (the adversary's) | adversary | 1 Discord | 1 cube onto the adversary's research track | its agents move along its arrow onto its city; then, holding none, it makes 1 agent |

Neutral and player sites pay in production (§9); corrupted sites make their **one output** in the adversary's Income
beat (§6). A city is the adversary's or its country's, never a player's.

**Blocked.** A site with one **opposing piece** on it makes nothing while the piece stands. An agent blocks a neutral
or player site, so one agent on a city shuts off its energy for everyone. A player's ambassador blocks a corrupted
site, so a corrupted city with your ambassador on it makes no Discord. Nodes never block, and a node doesn't own the
hex it is on.

### Pieces

| Piece | Whose | In brief |
|---|---|---|
| **Node** | a player | where you act from (one node per hex, of any player) |
| **Ambassador** | a player | moves or acts once a round: liberates, blocks, takes down botnets, armors your pieces; an **ambassador** means any player's |
| **Agent** | the adversary | lands on sites, blocks them, corrupts them, destroys nodes in pairs |
| **Botnet** | the adversary | lasting token: makes compute; never counts against the win |
| **Troop** | the adversary | garrisons, guards corrupted sites, strikes your pieces (§11) |
| **Owner cube** | a player or the adversary | who owns a site |
| **Fallout** | the adversary | the one scar, on open land; nothing is built on it; 1 Discord a round (§6) |
| **Hardening token** | — | saves one facility or node once (§13) |
| **Location token** | — | marks where a threat sits (§10) |
| **Firewall** | — | no agent lands on its hex, nothing there corrupts (§16) |
| **No-Infiltrate, no-Strike, no-Corrupt** | — | a Unity card's prohibitions, for the next adversary turn (§12) |
| **Round-flag marker** | — | an effect that outlasts its card (a blackout; Sabotage Teams) |
| **Waits token** | — | on a threat that takes effect a round late (§10) |

**Tracks and markers.** The adversary's board holds its compute income track, a compute track beside each project
slot (§17), its research row (§6) and its last-round space. Your board holds your income tracks (§9), your tag dials
(§7) and a used marker for each power, building slot and converter (§5). Each side has a tip track (§12). The player
aid's *Tracks and markers* says when each moves.

### Reach

A hex is **in your reach** if it is on or next to one of your nodes. Everything you target on the map must be in your
reach. One exception: **a troop on or next to one of your ambassadors is in your reach** for anything that targets
troops, so an ambassador standing on a guarded city brings its guards into range (§11).

**One target.** A card that touches the map names **one target**: a hex in your reach that meets the condition in
its **target box**, printed in the card's header ("Target: a corrupted city"). You pick the hex when the card
resolves (an Action when you use it, an Effect each time it fires). Board powers and standard actions that touch the
map do the same: their words open with the target ("Target: Troops. Remove up to 2 on it"). The card then acts **on
it**, or **on or next to it**, or **in its country**, exactly as printed, and never further. **Gain** puts pieces in
your supply; **Place** puts new ones on the map, on the target. With no hex meeting the condition, the card's map
effects do nothing.

*Sources: rules/board.md (Country pieces), rules/ambassadors.md (One site model), rules/adversary.md (One output a
corrupted site), rules/README.md (Terms); content/map.json; crates/aic-core/src/map.rs, modules/common.rs.*

---

## 3. Setup

**Choose** the number of players (1-4) and a difficulty level (0-4, §21; level 0 to learn).

1. **Lay out the map** for the number of players: 2 countries side by side; 3 with each touching both others; 4 in
   a ring.
2. **In each country** (each piece prints these places):
   - its first **2 cities** in its printed corruption order get an adversary cube: they start **corrupted**;
   - its marked research station gets an adversary cube and a **botnet**;
   - its other printed research stations (one next to the printed **starting hex**) and its printed setup factories
     start **neutral**. Its empty station site and its other industrial zones start empty;
   - its megaproject card goes beside the map, empty.
   No troops start on the map. **At level 0 only**, each city left free takes 4 agents from the supply (one arrival
   short of its 5; blocked, it pays no energy until they are cleared); at other levels no agents start on the map.
3. **The adversary** (the Militarist's board):
   - its agent supply is 24 per country;
   - build the **threat decks** as its board's **War Rooms** says: each tier's deck from every Security and
     Universal threat of that tier and as many of its other threats, taken at random; shuffle each. The rest stay in
     the box;
   - build the **event deck**: deal the number of stage I, II and III cards your level asks for (§21; levels 0-2:
     5, 5 and 3), shuffle each stage on its own, and stack them with stage I on top. Turn the top card face up: it is
     round 1's card;
   - shuffle the project deck and fill the board's three project slots (§17);
   - set the compute income marker for your level (§21) and the research marker before the first slot, with no cubes.
4. **Shuffle** the Unity deck, the Discord deck and the two player decks. The Unity and Discord tracks start empty.
5. **Players** take AI boards in this order, which is the **player order**: Builder, Protector, Researcher, then a
   second Builder. Sit in player order: cards pass around the table by it (§7). Player 1 plays in country 1, and so
   on: that is your **home country**. With 2 players the Builder and the Protector play by default (any two may);
   solo, the Builder plays. Each player:
   - places the first node from their **economy track** on their country's starting hex. This uncovers the track's
     first space, which shows nothing (§5);
   - takes 1 ambassador into their supply, 3 energy and their board's starting credits;
   - owns no city: every starting hex is next to a city, and a free city next to your node pays you its energy (§9).

Then play round 1 (§4). Its round start and Sway step come first, as in every round, so the adversary's turn comes
before anyone acts.

*Sources: rules/board.md (Setup), rules/adversary.md (Each round, The event deck), rules/threats.md (Built once, at
setup), rules/ai-boards.md; configs/baseline.json (`start_corrupted` 2, `start_ambassadors` 1, `supply_per_country`
24, `start_energy` 3, `research_start` 0); crates/aic-core/src/modules/board/mod.rs, modules/adversary/,
modules/ai.rs.*

---

## 4. The round

Each round runs in this order. Steps **C** and **D** are your choices; the rest you resolve.

| | Step | What happens |
|---|---|---|
| | **Round start** | Set your insight (§5). Clear your used markers (the player aid's *Once a round*). |
| **A** | **Sway** | Each Fallout token adds 1 Discord (§12). |
| **B** | **The adversary's turn** (§6) | Two beats: **Card** (threats, Escalate, Commandeered Plants, Infiltrate, Strike, Redeploy, one other line), **Income** (next card up; each corrupted site's one output; compute onto the projects; the research check). |
| **C** | **Cards** (§7) | Build a hand, draft, keep. Round 1 first deals the opening hand. |
| **D** | **Action phase** (§8) | All players at once: play cards, use powers (your nodes come from them) and Actions, move and act with ambassadors, take standard actions, pay threats. |
| **E** | **Production** (§9) | Everything pays: factories, cities, stations, megaprojects, then income. |
| **F** | **Threats** (§10) | Every threat not paid off takes effect, one that waited from last round included, and lands an agent. |
| **G** | **Cleanup** | Shelters and other this-round effects end. Stand every tipped ambassador up. |

So a threat is dealt in B, paid off in D, and takes effect in F. There is no window to act before the adversary: what
you set up in D (an ambassador on a site, a hardening token, a troop removed) is what meets the next round's
adversary turn, and the face-up card tells you what that turn will do.

**After every action and every step**, check the tracks: put any Unity just gained on its track, then tip any full
track, Discord first (§12). Then check the ways to lose, then the win (§14). If you would win and lose at once, you
lose.

*Sources: rules/economy.md (The round); configs/baseline.json `round`; crates/aic-core/src/rules.rs,
modules/alignment.rs.*

---

## 5. Your AI board

Each player is one AI board: the **Builder**, the **Protector** or the **Researcher**. Your board shows a **special
rule**, your **income**, one **tag** (§7), your **building slots**, two **node tracks** and its **powers** (§20):
three **node powers** and two or three **tiered powers**.

| Board | Special rule | Income each round | Insight at the start | Slots | Tag | Starting credits |
|---|---|---|---|---|---|---|
| Builder | *Breadth*: 5 slots | 10 credits, 1 energy | 3 | 5 | Industry | 27 |
| Protector | *Wall* and *Vigil* | 9 credits, 2 energy | 3 | 4 | Security | 31 |
| Researcher | *Patience* | 7 credits, 1 energy | 3 | 4 | Science | 27 |

- **Patience** (Researcher): all your unspent insight carries over to the next round, including insight another
  player gave you.
- **Wall** (Protector): **no troop is ever placed on a hex with your node** (it stays in the supply), none Redeploys
  onto it, and **none strikes it**, so that node and a facility on its hex are safe from troops. A piece next to it is
  not. Only troops are stopped: corruption, agents and threats still destroy what is there. A troop already on a hex
  when you place a node there stays.
- **Vigil** (Protector): whenever troops Strike, gain 1 insight for each troop on or next to your nodes, at most 2 a
  round. Strikes happen in the adversary's turn, before the draft, so this insight pays this round's keeps and plays.

### Resources

Nothing has a holding limit, and nothing caps your income.

| Resource | Kept between rounds? | Mainly from | Mainly spent on |
|---|---|---|---|
| **Credits** | yes | income, track spaces, cards, megaproject payouts, node powers, selling a card (1) | card costs, standard actions, Lab II, threats |
| **Energy** | yes | income, every free city next to your nodes (1 each), Horizon's converters, cards, the Builder's Overcharge | it powers things: every track space you uncover, the costly node powers, Patrol, building Actions, cards, threats |
| **Insight** | **no** | set each round (below); Vigil; converters; cards | keeping and playing cards, the Researcher's Peer Review, Field Station and Lab, threats |
| **Chips** | yes | your factories, cards, the Builder's Scrap and Full Capacity, Onyx's converter | research stations (2), Fortify, Annex, chip converters, cards, threats; chips can't be sold |
| **Antimatter** | yes | antimatter plants (a building's Action, once a round), the Builder's Overcharge II, two capstones; at worst Make antimatter (§8) | the majors: tier-2 cards (liberations, troop strikes and more) and a few tier-1 cards |
| **Research cubes** | yes | your stations, once their megaproject is complete (§9); Cryo Sample Vault; Open Archive; Peer Review; cards | 1 for 1 insight once a round, converters, Lift Fallout, Make antimatter, building Actions, threats |
| **Ambassadors** | yes (your supply) | 1 at the start, cards, the Protector's Accord, 20 credits | liberating, Remove agents, armor, threats; on the map they move and act (§8) |

**Insight is this round's attention.** Your card-play track prints it, as in Spirit Island: 3 at its start, and on
later spaces the new total (the Builder 7, 8, 9, 10; the Protector and the Researcher 7, 8, 9): a big jump on the
first space, then a step at a time. At the start of each round, set your insight to the number on the rightmost space
you've uncovered that shows one, plus what your cards add. Insight you don't spend is lost (except under Patience),
and no threat or project takes it. Everything else on your board and cards pays at the end of production (§9).

### Node tracks

Your nodes start on your board, on two tracks of 7 spaces: the **economy track** and the **card-play track**. To place a
node, take the next one off either track. That uncovers the space under it, and what the space shows is yours for the
rest of the game: income, a new insight total (card-play track only), a free research station (the Researcher), a tag
(space 6), the **keep upgrade** (§7), and on the last space a **capstone**, a lasting rule (§20). The card-play track
does not limit how many cards you play.

**Every placement pays the energy printed on the space** you uncover, whatever places the node; a power or card that
places one costs only its own price on top:

| Space | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| Economy track: energy to pay | (setup) | free | free | pay 1 | pay 2 | pay 3 | pay 5 |
| Card-play track: energy to pay | free | free | pay 1 | pay 2 | pay 2 | pay 3 | pay 5 |

**Where:** on a land hex next to one of your nodes (two node powers reach 2 away), with no node of any player and no
Fallout. A hex with agents or a corrupted site is allowed (a node doesn't remove agents).

**A destroyed node leaves the game.** It does not go back to its track; the space stays uncovered.

### Powers

Powers are used in the action phase. **Nodes come only from powers and cards:** there is no standard action for
nodes. Every board prints **three node powers**, each used at most once a round, all three usable in the same round:
**one cheap** (free: a node and a small extra) and **two costly** ones, each a node and a bigger benefit for the
board's own currency. So you place 1 node a round free, and up to 3 when you pay; cards can place more. A node power
needs a node to place (a token, a space whose energy you can pay, a free hex in its range): with none, you can't use
it, not even for its extra. The cheap ones are the Builder's **Buy In**, the Protector's **Levy** (a node and 1
credit each) and the Researcher's **Field Survey** (a node and a card); the costly ones are in §20.

Each board's other powers are **tiered**: pick one tier of each a round, a higher tier costing more for more. Most are
paid by **sacrificing** cards with a tag (§7, *Tags*); a **threshold** power only needs the tags on your cards. §20
lists them all.

> **Example.** The Builder has 2 economy spaces uncovered (the setup space and one more) and none on the card-play
> track. Buy In onto the card-play track's first space costs nothing. Sprawl onto the economy track's third space
> costs 2 energy (Sprawl's) and nothing for the space; Annex onto the economy track's fourth space then costs 1 chip
> and 1 energy (the space's). Three nodes in one round, for 3 energy and a chip.

*Sources: rules/ai-boards.md, rules/ai-powers.md, rules/economy.md (resource table), rules/troops.md (Wall);
content/ais.json; crates/aic-core/src/modules/ai.rs, modules/economy.rs, modules/board/node.rs,
modules/board/troops.rs (Wall, Vigil), modules/cards/sacrifice.rs.*

---

## 6. The adversary's turn

The adversary has a **board**, an **event deck**, a **project deck**, a **research row**, and a supply of agents,
troops and botnets. Each event card shows its **stage** (I, II or III): the stage sets which threats it deals (§10).
There is no agent income: the cards and the corrupted factories say where agents land.

The adversary in this book is **the Militarist**. Its board prints:

- **War Rooms:** how its threat decks are built at setup (§3): every Security and Universal threat, and as many others.
- **Commandeered Plants:** every round, after Escalate, each factory not yet the adversary's that holds an agent takes 1 more.
- **War Research:** its research row (below).
- Three open project slots, and its difficulty levels (§21).

### The turn: two beats

**1. Card.** This round's card (last round's) moves to the **last-round space**, and the one already there goes to
the discard. The face-up card becomes this round's card (if there is none, you lose: the round cap). Then, in order:

- **Threats.** Its first line deals threats (§10).
- **Escalate.** Each site of the type **last round's card** infiltrated (a stage-III card names two: both), not yet the
  adversary's, that holds an agent takes **1 more**, in every country, in hex order. Round 1 has no last-round card:
  nothing escalates.
- **Commandeered Plants** (the board's rule, above).
- **Infiltrate X.** Every event card has one; stage-III cards have two. **1 agent** (2 at level 4, on cities) lands
  **on each site of type X** (cities, coastal cities, research stations, factories or hexes holding a botnet) in every
  country that isn't yet the adversary's, whoever owns it. A line naming research stations, factories or botnet hexes
  lands at most 1 on each, even at level 4. A firewalled site (§16) or one with a no-Infiltrate token (§12) takes
  none: its agent stays in the supply.
- **Strike**, where the card shows the icon: every troop destroys one of your pieces on or next to it (§11).
- **Redeploy**, where the card shows the icon: each troop moves onto its country's corrupted city nearest a
  player node or ambassador (§11).
- **At most one other line**, with its own icon (8 of the 16 cards print none): **Funding** (+1 compute a country a
  round, for good), a **site opening** (a neutral factory on each country's lowest-numbered free industrial zone, or a
  neutral research station on its lowest-numbered empty station site), **Botnets spread** (level 1+), a **blackout**
  (until the end of next round, cities make no energy and no Discord) or **Fallout** (1 in each country, next to its
  lowest-numbered corrupted city).

**Botnets spread (level 1+)** puts a botnet in each country onto its lowest-numbered site without one that is next to
one of its botnets (a country with none: its lowest-numbered research station). At level 0, skip it.

Whenever an agent lands where players' ambassadors stand, it cancels one of them, one for one (§8).

**2. Income.** Turn the next event card face up: its Infiltrate lines and icons tell you what next round's Card beat
will do. Then, in order:

1. **Outputs.** Each corrupted site makes its **one output** (below), and each botnet its compute.
2. **The compute step.** Add the adversary's compute income, the botnets' compute and what waits beside the projects
   (made since the last step), and move all of it onto the projects at once (§17).
3. **The research check** (below).
4. Firewalls, cordons (§16) and no-Infiltrate, no-Strike and no-Corrupt tokens (§12) come off.

### One output a corrupted site

In the Income beat each corrupted site makes one thing. **A player's ambassador on it stops it** (blocked, §2).

| Site | Its output |
|---|---|
| **City** | 1 Discord (none under a blackout) |
| **Research station** | 1 research cube onto the adversary's research track |
| **Factory** | its agents move along its zone's arrow onto its city, an arrival there, unless that city is already the adversary's, firewalled, under a no-Infiltrate token or in a cordoned country; then, if it holds no agent, it makes 1 agent on itself |
| **Botnet** | 1 compute (none with your ambassador on its hex, or when a card hacks it) |

So a corrupted factory feeds its city 1 agent a round, and one whose city has fallen keeps a single agent. Liberate a
factory, or stand an ambassador on it, and its agents stop coming. A factory makes no troop.

### Corruption

A site's **corrupt number** is how many agents it takes: city **5** (**3** from level 1: Sleeper Cells, §21), station
**3** and factory **3**, printed on the map's sites. Only agents count.

**Corruption happens on arrival.** Whenever the adversary puts agents on a hex (an Infiltrate, an Escalate,
Commandeered Plants, a factory's agent or push, a threat's or card's agents), check **that hex alone**:

- a city, station or factory not yet the adversary's holding its corrupt number **corrupts**, and its agents go back
  to the supply;
- otherwise, **2 agents on your node's hex destroy your node**, whatever else is there. The agents go back to the
  supply, and the node leaves no mark: losing it is punishment enough. Protections apply (§13).

**Check as you place.** A line puts all the agents it sends to a hex on it, then checks that hex once. There is no
second pass. Nothing else on the board is checked: a site only reaches its number when agents land on it. Agents a
player moves (Gather, Push, §16) are no arrival and check nothing. Nothing on a firewalled hex (§16) or one with a
no-Corrupt token (§12) corrupts or loses a node.

**Whatever corrupts takes the adversary's cube**, replacing any owner's cube, and the agents on it go back to the
supply. A node on that hex is destroyed (unless something protects it, §13), and every ambassador on it is removed.

**Fallout** is the one scar, on free open land. Nothing can be built on it (no node, factory or research station),
and each token adds 1 Discord every round in the Sway step until a player lifts it (Lift Fallout, §8; some cards).
A Fallout line drops it next to the lowest-numbered corrupted city; a threat on or next to its first token; others next
to the site they name. With no free hex, it is lost.

> **Example: escalate.** Last round's card infiltrated factories. This round, the neutral factory on hex 9 holds 1
> agent and the one on hex 15 holds none. Escalate: hex 9 takes 1 more (2); hex 15 takes none. Then Commandeered
> Plants: hex 9 holds agents, so it takes 1 more (3), its corrupt number: it corrupts, and its 3 agents go back to the
> supply. Had you removed hex 9's one agent in last round's action phase, neither line would have touched it: both
> read "holds an agent".

§15 has every detail.

### Projects and botnets

The adversary spends **compute** on **projects** in the compute step, always on the **leftmost**; a project that fills
**resolves** at once, in every country, and the rest slide left (§17). About one project a round lands. Some cards and
the Researcher's Hack II take compute off a project.

Each country starts with a **botnet** on its corrupted research station; more come from **Botnets spread** lines and
the project Botnet Swarm. A botnet stays until a card, the Unity card Decommission or the **Take down a botnet** action
(§8) removes it. It makes 1 compute a round and nothing else, and it never counts against the win. Removing one sways
neither side.

### Research: War Research

Every adversary board has a **research row**: six icon slots and a marker, printed on the board. Cubes go onto it from
the corrupted research stations (1 each a round, above) and from lines that print "cubes onto the research track":
eight threats, three projects (Backdoor Research, Lab Seizure, Station Coup) and the Discord card Captured Labs.

In the **research check**, while the track holds **K cubes a country** (3 at levels 0-1, 2 from level 2; with two countries, twice that),
spend them and move the marker one slot; the slot's icon happens **once**, as the marker arrives, in each country. One
check can advance several slots. **Slot 7 is a loss.** The Militarist's row:

1. **New factory:** each country, a neutral factory opens on its lowest-numbered free industrial zone.
2. **Cells: factories:** Infiltrate: factories.
3. **Funding:** +1 compute a country a round.
4. **Garrison:** each country, 1 troop garrisons (§11).
5. **Strike:** every troop strikes (§11).
6. **Garrison:** each country, 1 troop garrisons.
7. **The players lose.**

Take cubes off with **Counterintelligence Sweep** and **Arms Inspectors**; nothing moves the marker back. Liberating
a corrupted station, or standing an ambassador on it, stops its cube. The row is public: watch it as you watch the
projects.

*Sources: rules/adversary.md (Each round, One output, Research track, Infiltrate X, Arrival corruption, Botnets);
docs/adversary-cards.md (the Militarist, the event deck); content/adversary.json;
crates/aic-core/src/modules/adversary/ (rally.rs, corrupt.rs, research.rs), modules/plan.rs.*

---

## 7. Cards

### Decks and kinds

There are two player decks: **tier 1**, and the stronger **tier 2** (the majors). A card is one of three kinds:

- **Programs** (green) resolve when played and stay in front of you. Most lasting **income** comes from programs.
- **Buildings** (blue) go into a slot on your AI board and keep working: an **Action** you may use once a round, an
  **Effect** that pays each time its trigger happens, or an income each round.
- **Events** resolve and go to your played pile. They never come back, and their tags don't count.

Every card has 1-3 **tags**: Science, Industry, Cyber, Power, Diplomacy, Security.

**Draw** means take the top tier-1 card into your hand, free. When a deck runs out, shuffle its discards into a new
deck. There is no hand limit, and no limit on how many cards you play: **draw is precious**, so cards are what run
short. Only three buildings draw again and again, each at the end of a tag chain: Preprint Server (Science),
Command-and-Control Grid (Cyber) and Consulate Tower (Diplomacy).

### Round 1: the opening

Before round 1's draft, each player is dealt **2 tier-1 cards** and **2 tier-2 cards** straight into their hand, kept
**free**. The tier-2 cards point you toward a build to plan for; you need not play both. Then round 1 builds a hand,
drafts and keeps like every round.

### Every round: build, draft, keep

1. **Build a hand.** Each player takes the top card of the tier-1 or the tier-2 deck, your choice each time, one at a
   time until they have 4. Each arrives face up, so you see it before choosing the next. (This is not a draw.)
2. **Draft.** Take one card from the hand in front of you and pass the rest on, until the hands are gone. Always
   pass to the next player in player order. Solo, you take all 4, then keep those you want.
3. **Keep.** Pay **1 insight** for each tier-1 card and **3 insight** for each tier-2 card you keep (2 with the keep
   upgrade, and less with Science discounts, below). Credits can't pay for a keep: when your insight runs out, you
   keep no more. Discard the cards you don't keep.

Keeps and plays spend the same insight, so every card you keep is insight you can't spend playing one this round.

### Playing a card

In the action phase, play a card from your hand by paying:

- its **credit** cost (the octagon), less your discounts;
- its **insight** cost (the hexagon): 0, 1 or 2. Credits can't pay this;
- any extra cost printed below the art: energy, chips, antimatter or research cubes.

A **requirement plate** must be met. A **yellow plate** means "at least N of this tag on your cards". A
**stacked-cards plate** means the card must be built over a building whose top card has that tag. A card's own tags
never count toward its own requirement. They do count for its own per-tag gains once played, except on an event.

**Discounts** (a discount can bring a cost down to 0, never below):

- every 2 Science tags on your cards take 1 insight off each card you play, and off each tier-2 card you keep;
- every 3 Industry tags take 1 credit off each building;
- some cards make all your cards of one tag cheaper.

**Play a card if any part of it can act.** Parts with nothing to act on (no hex in reach meets its target box, say)
are skipped. A program or event with nothing at all to do can't be played; a building always can. A sway toward
Discord is a price, not a part that acts.

**Sell a card** from your hand for 1 credit at any time in the action phase.

### Tags

The tags **on your cards** are those on your programs, your buildings (covered ones too), your AI board and your
uncovered track spaces. Tags unlock requirements, set discounts and per-tag payouts, and pay for sacrifice powers and
some threats. Keep the count on your **tag dials** (§2): turn a dial when a card with that tag comes into play or is
sacrificed.

**Sacrifice.** A cost that names a tag ("1 Industry", "2 Industry") is paid by discarding cards of yours in play that
carry that many of the tag in all: each a program, or a building together with every building stacked under it (a tag
on a covered building counts). Events can't be sacrificed, and neither can your board or its track spaces. Tags on the
card beyond the cost are wasted. An income the card raised stays raised; an income it pays while in play stops.

### Buildings and slots

Put a building into a free slot, or **build over** one of your buildings; with no free slot you must build over one. A
covered building **stops working**: its income, Effect and Action stop, and any tokens on it sit idle. **Its tags still
count.** A card can't be uncovered.

- An **Action** (`→`: pay the left, get the right) is used once a round. "A, OR B" is one use: either half.
- An **Effect** (`:`: when the left happens, get the right) fires once for each action that sets it off (a card play,
  a building's Action, a power, a standard action), however many pieces that action removed or tokens it placed, with
  no limit a round (§23). A building can trigger itself.
- An **income** pays every round, at the end of production.
- A one-time gain printed below the art happens when you build it.

### Tokens on cards

Some buildings hold their own tokens: **Datasets** (Science), **Followers** (Diplomacy), **Capital** (Industry) or
**Operatives** (Security). Most arrive with a few, and gain more in one of two ways:

- an "A, OR B" Action: add tokens here, OR spend or count the tokens here; or
- an Effect that adds tokens when something happens, with an **At N**: when the tokens here reach N, N come off and
  the reward happens.

**A card only ever counts its own tokens.** Sacrificing or discarding a card loses its tokens.

### Help cards

A **help card** names another player: you choose one, and you both gain, you paying. Some Effects fire when **another
player** plays a card of a tag.

*Sources: rules/player-cards.md, rules/card-grammar.md (The engine wording, Help cards); configs/baseline.json
`cards`; crates/aic-core/src/modules/cards/ (deck.rs, play.rs, price.rs, tableau.rs, triggers.rs, sacrifice.rs).*

---

## 8. The action phase

All players act at once, as often as they like and in any order. When you **pass**, you are done until next round's
action phase: you can't play, pay or take anything back. The phase ends when everyone has passed. You may:

- **play cards** from your hand (§7);
- **use** your buildings' Actions and your board's powers, each once a round (§5, §20);
- **move or act** with each of your ambassadors, once each (below);
- **pay toward threats** you are near (§10);
- take **standard actions**, as often as you can pay, but Remove a troop and Turn a cube into insight once a round
  each;
- use a **megacorp converter** (below).

### Ambassadors: one action a round

Each of your ambassadors on the map is **ready** (standing) or **tipped** (on its side). Each does **one thing a
round**, then is tipped over until cleanup stands every one up:

- **Move:** onto a land hex next to it, or onto any of your nodes from anywhere on the map. The hex needs no piece and
  no reach.
- **Act** where it stands: **Liberate** the site, or **Take down a botnet** on its hex.

**From your supply**, any number of your ambassadors may come onto your nodes, free: each **arrives ready**, so it may
still move or act this round. An ambassador a card or power places on the map arrives ready too. **A card's move is
not the ambassador's own action:** one a card moves (Gather, Push, "move up to N") stays as it was, a tipped one
tipped. A tipped ambassador still blocks, armors and cancels like a ready one.

Ambassadors are both **pieces and currency**: the ones in your supply pay for Liberate, Remove agents, threats and
some cards, and the ones on the map block, armor and act.

### Standard actions

Standard actions are always there, a hedge against bad luck: cards and your board's powers do the same things better.
Liberate, the way to win, is the exception.

| Action | Cost | Effect |
|---|---|---|
| **Move an ambassador** | that ambassador's action this round | onto a land hex next to it, or onto one of your nodes (from your supply: arrives ready) |
| **Liberate a site** | the action of your ready ambassador on it, and 3 ambassadors, any site (below) | a corrupted site with no agent on it and no troop on or next to it: it becomes neutral |
| **Take down a botnet** | the action of your ready ambassador on its hex, and 13 credits | the botnet comes off (your ambassador stays) |
| **Remove agents** | 3 ambassadors from your supply, used up | target agents: remove up to 3 on or next to it, your choice; nothing is placed |
| **Remove a troop** | 7 credits | once a round: target troops, remove 1 on it |
| **Lift Fallout** | 2 research cubes | target Fallout: remove it; the one standard action that removes Fallout |
| **Build a research station** | 2 chips | target an open hex that isn't an industrial zone or Fallout: the station is yours |
| **Gain a player ambassador** | 20 credits | into your supply |
| **Turn a cube into insight** | 1 research cube | gain 1 insight; once a round |
| **Make antimatter** | 5 research cubes | gain 1 antimatter and 2 energy: the desperate way; your engines make it far cheaper |
| **Sell a card** | a card from your hand | gain 1 credit |

### Ambassadors meet agents

**Every contact is one for one.** When an agent lands on a hex where players' ambassadors stand, it and one of them
are removed (the agent back to the adversary's supply, the ambassador out of play: not back to your supply); if
several players' ambassadors are there, the players choose whose. When your ambassador arrives on a hex with agents,
it removes one of them and is removed itself. So a position is held only until the next arrival.

**Remove agents** removes its agents one at a time, your choice, and you may stop early.

### Liberate and claiming

**Liberate is move in, then act.** Move one of your ambassadors onto a corrupted site (it tips over). Next round,
ready again, it acts: **Liberate**. One that arrives ready on the site (from your supply onto your node on that hex,
or placed by a card) may act the same round.

- **Reach:** none needed: your node need not be near. The round your ambassador stands there tipped is the risk.
- **Price, in ambassadors:** city **5**, station **4**, factory **3**, the same at every level.
- **Who pays:** your own ambassadors standing on the site pay first (the one acting among them), then your supply.
  Another player's never pay.
- **No agents, no guards:** a site with an agent on it can't be liberated: clear it first (Remove agents, a card, a
  power). **A corrupted site with a troop on or next to it can't be liberated** either: remove the troop or push it
  away first (§11).
- **Result:** the adversary's cube comes off and the site is left **neutral**; it adds Unity: city **+5**, station or
  factory **+2**. A card may claim a station or factory afterward; a city stays its country's. A botnet on the site
  stays. The site holds no agents, so the adversary needs its full corrupt number of arrivals to take it back.

> **Example: move in, then act.** A corrupted city 2 hexes from your node makes 1 Discord a round, and you hold 3
> ambassadors in your supply. In round 4 one comes from your supply onto your node (it arrives ready) and moves on to
> the hex between (tipped). In round 5 it moves onto the city (tipped): the city is blocked and makes no Discord. In
> round 6 it stands ready and acts: Liberate. The price is 3; it pays 1, your supply pays 2. The cube comes off, you
> add 5 Unity, and the city pays 1 energy every round to every player with a node on or next to it. Had a troop stood
> next to the city, you would have removed it first.

**Liberate cards** (Popular Uprising and some tier-2 cards) liberate their target, a corrupted city in reach, as
Liberate does, for the card's own price instead of ambassadors and with no ambassador on it. Of the liberate cards,
only Hostile Buyout may target any corrupted site, and puts your cube on a station or factory it frees (a city it
frees stays neutral). The others **rescue** it (the cards' word): liberate it, then **claim a facility next to the
city**: a neutral station or factory there with no agent on it becomes yours, even out of your reach; with none,
nothing more happens. A guarded city is no target for any of them, except Peacekeeping Mission, which first removes up
to 3 troops on or next to it and rescues only if none is left.

**Only cards claim.** A card that lets you **claim** a neutral research station or factory in your reach puts your
cube on it: it is **yours**. One with an agent on it can't be claimed. Open Research Campus and Industrial Commons
open a station or factory on their target and claim it.

### Megaprojects and converters

Each megacorp's **megaproject** needs 8 cubes, which neutral and player research stations in its country put on it
(§9); the adversary's never do. When it completes, each cube on it pays its owner the payout below, and its
**converter** opens. Any player with **a city of that country in reach** (any city, corrupted ones too) may use the
converter, **once a round** each, and the **chip converter** where the card prints one, also once a round: chips you
saved early pay here late.

| Megacorp (megaproject) | Payout, each of your cubes | Converter | Chip converter |
|---|---|---|---|
| Horizon (Tidal Grid) | 3 energy, 2 credits | 2 research cubes → 4 energy | 1 chip → 2 energy |
| Cintamani (Polar Compute Array) | 6 credits | 1 research cube → 2 insight | 1 chip → draw 1, 2 credits |
| Onyx (Mountain Fusion Works) | 2 chips, 2 credits | 1 research cube → 1 chip | none |
| Biomia (Delta Seawall) | 1 Unity, 3 credits | 1 research cube → 1 Unity | 2 chips → 2 Unity, draw 1 |

*Sources: rules/economy.md (Standard actions), rules/ambassadors.md (One action a round, Liberating, Every contact,
Remove agents, Claiming), rules/board.md (Converters); crates/aic-core/src/modules/board/standard.rs,
modules/board/ambassadors.rs, modules/alignment.rs; content/map.json.*

---

## 9. Production

Production is one **income step**, in this order. Do each part for the whole map before the next.

**Read it from the tracks, not the map.** What your factories and the cities beside your nodes make sits on your
income tracks. Stations follow their country's megaproject: until it completes, every neutral or player station there
puts its cube onto that megaproject, read off the map; once it completes, your stations' cubes move to your income
tracks. Move a marker only when a site changes hands (claimed, liberated, corrupted, built or destroyed), a
megaproject completes, or a node next to a city is placed or lost. When an opposing piece lands on a producing site
(an agent on yours or a neutral one), put a **blocked** chip on it, and lift the chip when the last such piece leaves;
a blackout chips every city. Each track pays what it shows less what its chipped sites make.

1. **Factories.** A player's factory makes 1 chip for its owner. A neutral factory, or a blocked one, makes nothing.
2. **Cities.** Each city that is neither corrupted nor blocked pays **1 energy to every player** with a node on or
   next to it, however many players that is and however many cities a player stands beside. Sharing a city costs no
   one anything.
3. **Stations.** A player's station makes 1 cube in its owner's color; a neutral one makes 1 neutral cube. They go
   onto their country's megaproject. Once it is complete, a player's cubes go onto their board as **research cubes**,
   and a neutral station's are lost.
4. **Megaprojects.** A megaproject holding 8 cubes or more **completes**, once: every cube on it pays, overflow
   included (§8), then its cubes come off. Neutral cubes pay nothing. From then on its stations' cubes go to their
   owners' boards (step 3). Countries complete in order.
5. **Income.** Each player collects the rest of their income (insight aside, §5): board, track spaces, programs and
   buildings. Factory chips, city energy and research cubes were paid in steps 1-3, once: don't collect them again.

The adversary's corrupted sites don't produce here: they made their one output in its Income beat (§6). New neutral
facilities come from site-opening lines (§6), not from production. A destroyed factory leaves the map and frees its
zone.

*Sources: rules/troops.md (Every round: production), rules/economy.md (Cities' energy, The round), rules/board.md
(Research cubes and the corps); configs/baseline.json `round` (`board.produce`, `economy.upkeep`);
crates/aic-core/src/modules/board/production.rs, modules/economy.rs.*

---

## 10. Threats

Each round the adversary deals **threats**: problems placed on the map that you pay off in the action phase or suffer
at the end of the round.

**Dealing.** The event card's first line deals threats: stage I one tier-1 threat in each country, stage II one tier-2
(Supply Lines: a tier 1 and a tier 2), stage III two tier-2s. For each, in each country, deal **the top card** of that
tier's threat deck there. No card is turned and discarded, and no country is skipped. Each threat carries 1 or 2
**tags** (Security, Diplomacy, Cyber, Industry, Power: the tags your cards carry; **Universal** counts as every tag)
and 1 or 2 **location types** (city, coastal city, research station, factory).

**Location tokens.** A threat puts a **location token** in its country on a site of each of its types that isn't the
adversary's: the one with the most agents on its own hex (tie: the lowest number). With no such site of any type, its
one token goes on the country's lowest-numbered city not the adversary's (§18).

**Near.** You are **near** a threat if one of your nodes **or ambassadors** is on or next to one of its tokens. Only
players near a threat may pay toward it or use a card on it.

**Who gets hit.** If it takes effect, a threat's harm to a player hits **this country's player**: the player whose
home country it was dealt in, wherever their nodes are now and whoever was near it. A threat in a country that is no
player's home only does what it does to the board. This is the one rule that reads your home country.

**Paying it off.** A threat prints one or two ways to pay it off. **Pay one way in full** and it is discarded.

- **Resources** (credits, energy, insight, research cubes, chips, or ambassadors from your supply) may be pooled by all
  the players near it. You may take back what you put on it until you pass. Anything left on a threat that isn't paid
  off goes back to its payers when threats take effect.
- **Tags:** one player alone sacrifices cards in play carrying 2 of the threat's tag in all (tier 1) or 3 (tier 2).

A threat never asks for what it takes: one that takes energy never offers energy as its price. Cards can also **jam**
a threat (it needs less of each resource), **delay** it (it waits a round, and can be paid next round), or pay it off
outright. A few buildings pay off part of every threat of a type, wherever it is.

**When threats take effect** (step F), each threat not paid off does what it prints, country by country, then lands
**1 agent** on its first token (an arrival, §6), then is discarded with its tokens. Pieces it places go on its token.
A resource harm reads "pays N or discards M cards": the player chooses; with neither possible, they pay what they
have and discard their hand. Many threats also give the adversary compute, which waits for the next compute step.

A threat dealt after the action phase (by a Discord card tipped later in the round, or by another threat taking
effect) gets a **waits** token: it takes effect next round, after an action phase in which it can be paid.

> **Example.** A Security threat dealt in the Protector's home country sits on a research station. The Builder's
> ambassador stands next to it, so the Builder is near and may pay; the Protector's nodes are far away, so the
> Protector can't. Nobody pays. In step F the harm hits the Protector, whose country it is, and 1 agent lands on the
> station.

*Sources: rules/threats.md; content/threats.json; crates/aic-core/src/modules/threats/ (mod.rs, near.rs,
counter.rs).*

---

## 11. Troops

Troops are the adversary's heavy pieces: **late and significant**. You see them coming and must plan to meet them.
They **garrison, guard and strike, and never move on their own**.

**Where they come from.** Only the research row's two **Garrison** slots (§6), three projects and the Discord card
Standing Army make troops (§19 lists where each lands); no threat, event card or factory does. A Garrison puts 1
troop (2 from level 3, Arms Race, §21) in each country onto its **lowest-numbered corrupted factory**; with none, its lowest-numbered corrupted city;
with none, its lowest-numbered city. Nobody chooses. No troop is ever placed on the Protector's node hex (Wall, §5):
it stays in the supply.

**Guard.** A corrupted site (city, station or factory) with a troop **on or next to it can't be liberated**, by the
Liberate action or by a card. **Deal with the troop first, then liberate:** remove it (Remove a troop, 7 credits; the
Builder's Fortify, the Protector's Interdict, the Researcher's Hack; cards; Unity's Peacekeepers and Disarmament
Talks) or move it away (the Protector's Patrol; Push and Gather cards). A troop on or next to one of your ambassadors
is in your reach for anything that targets troops: the ambassador you moved onto a guarded city brings its guards into
range.

**Strike.** An icon, on an event card (stage II's Mobilization, Fifth Column and Border War, and all four stage-III
cards) and on the research row's slot 5. Because the next card is face up, **you see an event card's Strike coming a
full round ahead**. On a Strike, **every troop destroys one of your pieces on or next to its hex**: a node, or a
player's research station or factory; never a city, nor a neutral or adversary facility, nor anything on the
Protector's node hex. Where a troop has several targets, the players choose. Each piece is protected as usual (§13).
**The troop is not used up:** it stays and strikes again on the next icon. A troop with no target does nothing. A
strike that takes a player's last node ends the game at once.

**Redeploy.** An icon, rare: stage II's Redeployment and stage III's Occupation (after its Strike). Each troop is picked
up and put on **its own country's corrupted city nearest a player piece** (your nodes and ambassadors, counted in
hexes; ties: the lower hex number): the garrisons come to where you are working. One already there stays; with no
corrupted city in its country, it stays. Nothing is struck on the way, and none lands on the Protector's node hex.

**Troops ignore ambassadors.** A troop garrisons, guards and strikes whatever ambassadors stand by it, and removes
none. Your own ambassador on a struck piece's hex is still armor against the strike (§13).

> **Example: a guarded city.** The research row reaches slot 4: a troop garrisons on a corrupted factory next to a
> corrupted city. The city can't be liberated now, and the troop is out of the Builder's reach. In the action phase
> the Builder's ambassador moves onto the city (tipped); the troop is next to it, so now in reach, and the Builder
> uses Fortify I (1 chip) to remove it. Next round the ambassador, ready again, liberates the city. Had the next
> card shown a Strike with the troop still there, it would have destroyed a player's node, station or factory on or
> next to it, if one stood there (never the ambassador).

*Sources: rules/troops.md (Adversary troops, Wall), rules/ambassadors.md (Troops and reach); content/adversary.json,
content/projects.json; crates/aic-core/src/modules/board/troops.rs.*

---

## 12. Unity and Discord

The world leans toward **Unity** (you) or **Discord** (the adversary). Each side has its own **track**, its own **tip
count** and its own deck of one-shot cards. The two tracks are independent: nothing on one side empties the other.

### Gaining Unity and Discord

Effects **sway** Unity or Discord ("add 2 Unity"): put that many on that side's track.

- **Unity** is trust and cooperation: liberating sites (city 5, station or factory 2), Biomia's converter and payout,
  and cards that build or bring people together.
- **Discord** is fear, force and collateral harm: corrupted cities (1 each a round, in the Income beat), Fallout (1
  each a round, in the Sway step), threats, projects, Discord cards, and your own strikes (cards that print it).
  Removing a botnet or a troop sways nothing.

**Unity never cancels Discord.** Unity you gain always goes onto the Unity track. Nothing takes Discord off its track
but a Unity card that says so: four do (Volunteer Corps, Safe Corridors, Reconciliation Panel, Town Hall Accord), each
counting something on the board.

### Tipping

The Unity track **tips** when it holds **3 × the number of players**, the Discord track when it holds **4 × the
number of players**. Then:

1. Take one tip's worth off. The rest stays on the track toward its next tip.
2. Add 1 to that side's tip count.
3. Draw that side's top card and resolve it at the strength for its tip count, this tip included: **strength I** on
   tips 1-2, **strength II** on tips 3-5, **strength III** from tip 6.

If both tracks are full, Discord tips first. A track still full after a tip (what stayed, or a card that fills it)
tips again at once. The player aid's *Tip strips* print the strengths on one row a tip, with the Discord cards left.

### The cards

Every Unity and Discord card prints three strengths. Like Spirit Island's fear cards, each acts across the map by a
condition ("for each station or factory a player owns"), never in one player's reach.

- **The Unity deck** (23 cards) reshuffles when it runs out. Some Unity cards **forbid** the adversary's Infiltrate,
  Strike or Corrupt: put a no-Infiltrate, no-Strike or no-Corrupt token on every hex where the card's condition holds
  as it resolves. In the next adversary turn that step skips a hex with its token: an Infiltrate's agent for it stays
  in the supply; no troop strikes a piece there; agents may land on a no-Corrupt hex, but nothing there corrupts and
  no node there is lost to agents. The tokens come off at the end of that turn, like a firewall. No Unity card frees a
  city: you still reach it and Liberate.
- **The Discord deck** (7 cards: Incursion, Captured Labs, Propaganda, Scorched Earth, Offensive, Standing Army,
  Firestorm) **never reshuffles.** A Discord tip with no card left loses the game: you survive 7 Discord tips and
  lose on the 8th, at every level. Discord cards destroy no nodes themselves: they deal threats, create agents, take
  credits, drop Fallout, feed the research track and bring troops, more the more sites are corrupted. **Offensive**
  deals its tier-2 threats (1, 2 or 3 by strength) onto the lowest-numbered node hex holding an agent or a troop (the
  lower country, then the lower hex number), its first token on that hex; with none, it deals nothing.
  **Standing Army** makes troops (§19).

*Sources: rules/alignment.md; configs/baseline.json (`tip_per_player` 3, `discord_tip_per_player` 4, `carry` 2,
`tier_at` 3, 6); content/alignment.json; crates/aic-core/src/modules/alignment.rs.*

---

## 13. Protecting your pieces

### What can be destroyed

Your **nodes** and your **facilities** (stations and factories) can be destroyed, only where the board is overrun:

| By | What it destroys |
|---|---|
| **Corruption** (agents arriving, a threat or project that corrupts a site) | the node on the corrupted hex, then every ambassador there |
| **2 agents on your node's hex** (§6) | that node; the agents go to the supply |
| **A troop's Strike** (§11) | one node, or player-owned station or factory, on or next to the troop |
| **A threat taking effect** | what it prints: "destroy a node" takes this country's player's node closest to its tokens (their pick on a tie); Siege takes its factory and the node there |

Night Raid, a project, lands 1 agent on each of two node hexes already holding one in each country: a hex it brings to
2 agents loses its node at once. Nothing else destroys a node. A destroyed facility takes its owner's cube with it. A
destroyed node leaves the game and leaves no mark on its hex.

### The protection order

When a node or facility would be destroyed:

1. **Shelter** this round (a card shelters its player's nodes and facilities in their reach, as it stands when the
   piece would be destroyed: a node placed after the card is covered too) stops it, automatically, spending
   nothing. It lasts from when you use it to cleanup, so it covers production and threats, never a Strike (which
   comes in the adversary's turn, before the action phase).
2. **Aegis Doctrine** (a Protector capstone) is Shelter on the Protector's own pieces once a round: the first time
   each round one of its nodes or facilities would be destroyed, it is saved, automatically.
3. Otherwise **its owner chooses** one protection to spend, or none (for a neutral piece, the players choose). Each is
   used up:
   - a **hardening token** on the hex;
   - **your own ambassador** on the hex (never another player's).
4. With nothing spent, the piece is destroyed.

A facility's owner is the player whose cube is on it. One hardening token on a hex with both your facility and your
node can save either, once. Neither corruption nor agents destroy the nodes of a player with **Sanctuary** (a
Protector capstone).

**Hardening** goes on a player-owned station or factory that isn't corrupted (the Builder's Fortify II, the
Protector's Patrol II, which you can't use with no such facility on or next to a target; some cards), or, where a card
says "harden N of your Nodes" (Stress Test Suite, Early Warning Drill), on the hexes of N of your nodes: the same token.

### Your last node

Nothing protects a last node but the protection order above: anything that destroys a node can destroy your last
one, at any stage. Any player with no nodes loses the game for everyone, so keep a second node on the board: the free
node power and the first spaces of both tracks cost nothing. Watch the hexes next to a lone node: two agents on it
destroy it, and a troop next to it strikes it on the next Strike icon.

*Sources: rules/troops.md (Node loss, Destruction), rules/ai-powers.md (Shelters and hardening),
rules/ambassadors.md (Armor); crates/aic-core/src/modules/board/destroy.rs, modules/board/node.rs,
modules/adversary/corrupt.rs.*

---

## 14. Winning and losing

### Winning

You win **the moment no city on the map is corrupted**, at any round and any tip count. Each country starts with 2,
and only liberating cities (§8) takes them away. Corrupted stations and factories, botnets, agents, troops and Fallout
never count, but a troop guards the sites next to it, so troops near the last corrupted cities must go first.

### Losing

Everyone loses at once if:

- a Discord tip finds the Discord deck empty;
- any player has no nodes on the map;
- the Card beat finds no card to turn up (the **round cap**: 10 rounds at levels 0-2, 9 at levels 3-4). The round
  after the cap still opens with its Sway step, so a Discord tip can lose the game there first; then, with no card
  left, you lose;
- the adversary's research marker reaches slot 7 of its row (in its research check, §6).

The adversary running out of agents is not a loss: a placement it can't make doesn't happen. Every city corrupted is
not a loss either: a corrupted board drives Discord.

Checks run after every action and step: Unity just gained goes on its track, full tracks tip (Discord first), then
the losses, then the win.

*Sources: rules/alignment.md (The only win, Losing), rules/adversary.md (The event deck, Research track);
configs/baseline.json (`clear_win_stage` 1); crates/aic-core/src/modules/alignment.rs, modules/adversary/.*

---

# Part 2: Reference

## 15. Corruption in full

An **arrival** is any time the adversary puts agents on a hex from its supply: an Infiltrate, an Escalate,
Commandeered Plants, a factory's own agent, a threat's or Discord card's or project's creation, the agent an
uncountered threat lands on its first token (§18); and a corrupted factory's push along its arrow. Each agent arriving
on players' ambassadors cancels one of them first (§8). Each placement puts all its agents for a hex on, then checks
that hex, and only that hex:

1. **A site** (city, station or factory) not yet the adversary's, holding its corrupt number of agents, corrupts (§6),
   and its agents go back to the supply.
2. **Otherwise, a player's node** on a hex holding **2 agents** is destroyed (§13 applies) and the agents go back to
   the supply; nothing else on the hex changes: no mark.

| Level | City | Station | Factory | Your node |
|---|---|---|---|---|
| 0 | 5 | 3 | 3 | 2 |
| 1-4 | 3 | 3 | 3 | 2 |

Nothing on a firewalled hex or one with a no-Corrupt token corrupts, a node included (agents already on it stay). A
player's Gather or Push is no arrival. A site never corrupts by itself between arrivals, so one sitting below its
number stays safe until the adversary lands more there. A liberated site starts clean.

**What corruption does:** the site takes the adversary's cube (replacing a player's), the node on the hex is destroyed
(§13), and every ambassador on it is removed. Nothing else.

**Station flips.** An effect that flips a player's research station (the threat Station Seizure) takes it only when 2
agents and no player ambassador stand on it, those agents used up; short of that, one agent lands on it instead (a
player ambassador there cancels it). The project Station Coup instead turns a player's station neutral with no agent
test. A neutral station flips as printed.

**Picking a site.** Event, project and Discord lines pick in each country the **lowest-numbered** site that fits
("its lowest-numbered neutral factory"). Only threat tokens (§18) and the project Crackdown (the hex holding the most
player ambassadors) count pieces to pick.

*Sources: rules/adversary.md (Arrival corruption), rules/board.md (A station is a defended investment);
crates/aic-core/src/modules/adversary/ (rally.rs, corrupt.rs).*

---

## 16. Moving and removing pieces

Some cards and the Protector's Patrol move pieces on the map: ambassadors (any player's), agents and troops. Botnets
never move.

- **Gather up to N:** pick a land hex in your reach; move pieces into it, each from a hex next to it.
- **Push up to N:** pick a land hex in your reach; move pieces off it, each to a land hex next to it, your choice.

You may stop after any piece. Moved pieces arrive as usual: an agent moved onto players' ambassadors cancels one of
them, and an ambassador moved onto agents cancels one (§8). No agent enters a firewall; no troop enters the
Protector's node hex. A moved troop garrisons its new hex, guarding and striking from there, so pushing one away from a
corrupted site opens it to a Liberate.

**Remove around a hex.** Some cards (Kettle Operation, Lure Network, Honeypot Relay) remove up to N agents, or
agents and troops, **on or next to** the card's target: the target itself counts, and you choose which pieces, a
batch at a time, and may stop early. Nothing moves.

**Convert.** Some late cards turn agents into your ambassadors: up to N agents on or next to a city in your reach
each become your ambassador where it stood. The converted agents come off first, then your ambassadors go on, so two
converted on one hex both stay; an agent the card left on that hex meets one of them, one for one (§8). Standing on
the city, a converted ambassador pays toward a Liberate there.

**Firewall.** Some cards put a firewall on a land hex in your reach. While it stands, no agent lands, is placed or
moves in there: an agent an Infiltrate or Escalate would land there stays in the supply, and a factory pushes none
onto it. Nothing on it corrupts: no arrival, station flip or threat that corrupts its site takes it (agents already
there stay). Troops are not stopped. It lasts until the end of the next adversary turn (its Income beat).

**Cordon.** Hardpoint Lockdown cordons a country until the end of the next adversary turn: no agent lands there,
whatever sends it (Escalate, Commandeered Plants, Infiltrate, a creation line, a project's included), and its corrupted
factories neither push nor make an agent. Troops still land, Gather and Push still move agents in, and its botnets'
compute still counts.

*Sources: rules/ambassadors.md (Moving pieces, Convert and firewall), rules/card-grammar.md (Cordon);
crates/aic-core/src/modules/effects/, modules/adversary/.*

---

## 17. Projects

The adversary board has **slots** (the Militarist: three open slots that take any project). **To deal**, turn up
the top project and put it into the first empty slot; repeat until every slot is full. A project that resolves or is
discarded goes to the project discard; when the deck runs out, shuffle the discard into a new deck.

A project prints **one cost a board**: in play it costs that times the number of countries. All compute arrives in one
step, the **compute step** of the Income beat. (Keep each project's compute as a marker on a track beside it, not a
pile of cubes.) The compute step adds: the compute income (printed a country, so times the countries), 1 for each
botnet without a player's ambassador on it, and every compute made since the last compute step (threats taking
effect, War Footing, cards and projects that print it), which waits beside the projects until then: that public pile
is your warning. All of it goes onto the **leftmost** project. The moment a project holds its cost, it **resolves** at
once, does what it prints in **every country**, and leaves. No project is ongoing; one that lasts does it through
something already on the table (an income raised, pieces on the board, Fallout, a round flag).

**Slide and refill:** when a project leaves (resolved or discarded), the projects to its right slide one slot left,
and the empty rightmost slot is dealt a new project at once; the rest of the compute goes onto the new leftmost.
Compute with no project to go on is lost.

**What projects do.** A cheap project does one thing, a costly one two or three: sway Discord or pull Unity back, land
agents on a site in each country, corrupt or take a factory or station, drop Fallout, spread botnets, add cubes to the
research track, make troops, land agents on your nodes (Night Raid), send ambassadors home (Crackdown, Purge Lists),
take research cubes off megaprojects, strip hardening, make players pay or discard, or run the clock (Accelerated
Timetable discards the top event card: the round cap comes a round sooner). **Its target is read off the board when
it resolves**, in each country: always the lowest-numbered hex that fits (§15). Nothing is marked in advance: look at
the leftmost project and the board to see what it will hit, and move agents off, claim the site or firewall it
first. Three projects make troops: Botnet Conscripts, War Plants and Martial Law (36 a board;
where their troops land: §19). Full texts and costs: [adversary-cards.md](adversary-cards.md), *Projects*.

*Sources: rules/adversary.md (The project deck); content/projects.json; crates/aic-core/src/modules/adversary/projects.rs.*

---

## 18. Threat details

**Dealing.** Each threat the event card prints, for each country in turn, takes the top card of that tier's deck. The
decks were built at setup (§3) and are never filtered. When a threat deck runs out, shuffle its discards into a new
deck. A threat that deals a threat, and a Discord card's extra threat, deal the next card the same way.

**Location tokens.** In printed order, each token goes on the site of its type that isn't the adversary's with the
most agents on its own hex (troops, botnets and neighbors don't count); tie: the lowest hex number. A threat's two
tokens never share a hex; a type with no site left gets no token. **Fallback:** a threat with no such site of any of
its types puts its one token on the country's lowest-numbered city not the adversary's; there none of its located
lines ("the factory here") acts, but the token still marks who is near and where its agent lands. Its first token is
its **anchor**: a threat that takes effect unpaid also lands **1 agent** there after its harm, an arrival (§15).

**Prices.** One option, two at most: a scarce currency or a tag sacrifice, or credits.

| Tier | Energy | Ambassadors | Insight | Research cubes | Chips | Tags | Credits |
|---|---|---|---|---|---|---|---|
| 1 | 3 | 3 | 3 | 3 | 3 | 2 | 8 |
| 2 | 4 | 4 | 4 | 5 | 4 | 3 | 11 |

A tier-2 threat is about two tier-1s. A tier-1 threat never takes its own site. A threat costs what it prints at
every level (§21). Harms: [harms.md](harms.md).

*Sources: rules/threats.md; crates/aic-core/src/modules/threats/mod.rs, threats/counter.rs.*

---

## 19. Troops in full

**Where each troop lands:**

| Source | Troops | Where |
|---|---|---|
| Research row, slots 4 and 6 (**Garrison**) | 1 a country (2 from level 3, Arms Race) | garrisons: the lowest-numbered corrupted factory; with none, the lowest-numbered corrupted city; with none, the lowest-numbered city |
| **Botnet Conscripts** (project) | 1 in each country holding 2+ botnets | its lowest-numbered botnet comes off; the troop garrisons |
| **War Plants** (project) | 1 a corrupted factory | on the factory's hex (none under a player ambassador) |
| **Martial Law** (project) | 2 a country | its lowest-numbered corrupted city with a node or ambassador on or next to it; with none, they garrison |
| **Standing Army** (Discord), I | 1 a country | its lowest-numbered corrupted factory (none where no factory has fallen) |
| **Standing Army**, II / III | 1 / 2 a country | as Martial Law |

A troop never lands on the Protector's node hex: it stays in the supply. It stays where it lands until a Redeploy, a
player's Push or Gather, or a removal.

**Strike order.** Hex by hex, country by country and by hex number within each, every troop destroys one target on or
next to its hex; the players choose among several. A no-Strike token makes the pieces on its hex no target. **Vigil**
pays the Protector 1 insight for each troop on or next to its nodes, at most 2 a round, whenever troops Strike.

**Redeploy** ("each Troop moves onto its country's corrupted city nearest a player Node or Ambassador"): distance
is counted in hexes from the nearest node or ambassador of any player; ties go to the lower hex number. A troop
already there stays, and one whose country has no corrupted city stays. A corrupted city under the Protector's node
is no destination.

**Removing troops:** Remove a troop (7 credits, once a round); Fortify I / II (Builder, 1 / 2 chips: 1 / up to 2 on
its target); Interdict I / II (Protector, 1 / 2 Security: up to 2 on its target); Hack I / II (Researcher, 1 / 2 Cyber:
up to 2 in one country, no reach); Patrol (Protector, energy: pushes up to 3); cards; the Unity cards Peacekeepers and
Disarmament Talks. Removing a troop sways nothing. A Discord tip doesn't move troops.

*Sources: rules/troops.md (Adversary troops), rules/adversary.md (The project deck), rules/alignment.md (Details);
crates/aic-core/src/modules/board/troops.rs.*

---

## 20. Powers and capstones

Each board has three **node powers** (each once a round, all usable the same round) and two or three **tiered
powers** (one tier of each a round), used in the action phase. A tag in a cost means sacrifice cards carrying that
many of it (§7, *Tags*); a threshold power ("with 2 Science on your cards") counts the tags on your cards and
sacrifices none. Every placement also pays the track space's energy (§5).

| Board | Node powers | Tiered powers (one tier each) |
|---|---|---|
| Builder | **Buy In** (a node and 1 credit); **Sprawl** (2 energy: a node and 2 credits); **Annex** (1 chip: a node within 2 of yours and 3 credits) | **Scrap** I (1 Industry: 1 chip) / II (2 Industry: 1 chip and play a building from your hand without paying its credits; its other costs and requirements stand); **Overcharge** I (1 Power: 2 energy) / II (2 Power: 2 energy and 1 antimatter); **Fortify** I (1 chip: target troops; remove 1 on it) / II (2 chips: remove up to 2 on it and harden a facility on or next to it) |
| Protector | **Levy** (a node and 1 credit); **Outpost** (2 energy: a node within 2 of yours and 2 credits); **Accord** (3 energy: a node and 1 ambassador) | **Patrol** I (1 energy: target agents or troops; push up to 3 from it) / II (2 energy: the same, and harden a facility on or next to it); **Interdict** I (1 Security: target troops; remove up to 2 on it) / II (2 Security: target agents or troops; remove up to 2 troops and up to 2 agents on it; draw 1) |
| Researcher | **Field Survey** (a node and draw 1); **Peer Review** (2 insight: a node and 1 research cube); **Field Station** (2 insight: target an open hex; build a research station on it without the chips, and place a node) | **Lab** I (with 2 Science on your cards, 1 insight: draw 1) / II (with 4 Science, 1 insight and 2 credits: draw 1 and add 1 cube in your color to a megaproject that isn't complete); **Hack** I (1 Cyber: remove up to 2 troops in one country) / II (2 Cyber: the same, and remove 3 compute from one adversary project) |

| Board | Economy track capstone | Card-play track capstone |
|---|---|---|
| Builder | **Full Capacity**: 1 chip each round per 4 Industry tags on your cards | **Master Plan**: your buildings cost 1 credit less |
| Protector | **Sanctuary**: neither corruption nor agents destroy your nodes; 2 antimatter once, as you uncover it | **Aegis Doctrine**: once a round, the first time one of your nodes or facilities would be destroyed, it is sheltered |
| Researcher | **Open Archive**: 1 research cube each round; 2 antimatter once, as you uncover it | **Open Science**: each round, 2 insight from the supply, split among the other players; it arrives with their insight next round |

Each capstone costs 5 energy to uncover, on space 7 of its track. Space 6 of each track (3 energy) reveals a tag, which
counts like a tag on your cards but can't be sacrificed.

*Sources: rules/ai-powers.md, rules/ai-boards.md; content/ais.json; crates/aic-core/src/modules/ai.rs.*

---

## 21. Difficulty

The adversary's reference card prints this ladder. Levels are cumulative: each keeps every level below it, adds
its setup (set once, before round 1) and at most one named rule. Each row shows what its level changes. Your own
costs (liberate, cards, actions) and the threats' costs never change. The table is generated from the rules
(`aic-sim docs`); the words below are not.

<!-- generated: rulebook_difficulty -->
| Level | Name | Setup (set once; incomes and threshold a country) | Mid-game rule it adds | Rules active |
|---|---|---|---|---|
| 0 | **The Militarist** | Compute income 0 a country; Infiltrate 1 a site; Research track: a slot every 3 cubes a country; Each free city starts with 4 agents; Deck 4 / 4 / 2 (10 rounds); Sites print their corrupt numbers: city 5, station 3, factory 3 | — | 0 |
| 1 | **Sleeper Cells**: *Cells wake in every city.* | Compute income 3 a country; Free cities start with no agents | When an Agent lands on a city, it falls at 3 agents, not 5. | 1 |
| 2 | **War Footing**: *Every unanswered strike feeds the war machine.* | Compute income 5 a country; Research track: a slot every 2 cubes a country | When a threat executes uncountered, the adversary also gains 4 compute. | 2 |
| 3 | **Arms Race**: *The labs build weapons, not cures.* | Compute income 10 a country; Deck 4 / 3 / 2 (9 rounds) | When the research track reaches a Garrison icon, each country takes 2 Troops, not 1. | 3 |
| 4 | **War Economy**: *The whole economy marches.* | Compute income 15 a country; Infiltrate 2 a city (1 a site from a station, factory or botnet line); Deck 2 / 4 / 3 (9 rounds) | — | 3 |
<!-- /generated -->

The research track's K is a setup step: 3 cubes a country at levels 0-1, 2 from War Footing. **Arms Race**'s rule
doubles the row's Garrison icons: each one lands 2 troops a country, not 1.

- **Setup:** put the compute income marker on the adversary board's track at the number shown, a country (times the
  countries in play), and build the event deck with that many stage I / II / III cards (§3); at level 0, put 4
  agents on each free city. Nothing else to remember.
- **Mid-game rules** fire only on something you already resolve: agents landing on a city (Sleeper Cells), a threat
  taking effect unpaid (War Footing: its 4 compute comes at the next compute step, §17; a threat that only waits
  gives none). *Rules active* counts the ones in play.
- **Infiltrate** lands 1 agent a site at levels 0-3; at level 4, 2 on each city a line names and still 1 on each
  research station, factory or botnet hex a line names (§6).
- **Before level 1,** a "Botnets spread (level 1+)" line does nothing, as it prints: the botnets set up are the only
  ones, unless a project or card adds one.

*Sources: rules/adversary.md (Difficulty, The event deck), rules/threats.md (Countering); configs/baseline.json
`adversary` (`compute_income`, `infiltrate_agents`, `research_at`, the deck's dials); content/adversary.json
(the board's `levels`); crates/aic-sim/src/docs/ladder.rs (the table, from both).*

---

## 22. Player count and solo play

- **Countries:** one per player, 1-4.
- **Boards:** Builder, Protector, Researcher, then a second Builder. 2 players: the Builder and the Protector by
  default, or any two. Solo: the Builder.
- **Tipping:** the Unity track tips at 3 × the number of players, the Discord track at 4 ×.
- **Projects** print one cost a board, times the number of countries in play (§17); the compute income, every
  "+N compute" line and the research track's cubes a slot are a country's too.
- **Solo:** you take all 4 cards in the draft and keep those you want, paying as usual (§7). "Another player" on a
  card means you: an Effect that fires when another player plays a card fires on your own plays, and a shared gain
  is paid once.

*Sources: rules/board.md (The layouts), rules/ai-boards.md (`ai.boards`), rules/alignment.md (Tipping),
rules/adversary.md (projects, Research track), rules/card-grammar.md (Help cards, Teammate triggers).*

---

## 23. Reading a card: the icon legend

A card reads like a sentence. A few **header slots** sit at the top, and small icons for pieces, resources and tags sit
inside ordinary text, as on a Spirit Island power card. Words do the rest, and the text always says what the icons
say, in the same order.

### Where things sit

- **Top strip:** the credit cost (an octagon), the insight cost (a hexagon), requirement plates and the card's tags.
- **Target box:** a tab under the title, holding a map card's target.
- **Above the art** (buildings only): what lasts while the building is uncovered: income, Effects, Actions.
- **Below the art:** what happens once, when you play the card: extra costs first, then gains, placements and moves.

### The icons

The shape says the kind of thing; the picture inside says which. (The art is not final.)

| Kind | Icons |
|---|---|
| Resources (square tiles) | credits (a coin), energy (a lightning bolt), insight (an eye), chips (a chip), antimatter (an atom), research cubes (stacked cubes), cards (two cards); the adversary's compute (a grid) |
| Tokens on cards (discs) | Datasets (a database), Followers (two people), Capital (a bank), Operatives (a target) |
| Pieces (bare silhouettes) | each piece has one symbol of its own, usable in a sentence: **ambassador** (a figure), **agent** (a figure in a brimmed hat, as on the map), **node** (a house shape), **botnet** (a starburst), **troop** (a winged arrowhead); also the cube and the Fallout token |
| Places | city (a skyline), research station (a dome with a mast), factory (a sawtooth roof), any site (a hex holding a block: city, station or factory), **Rescue** (a skyline under a flag: liberate it, then claim a station or factory next to it), a hex outline, a country (a cluster of hexes), a megaproject (a card with a cube), an adversary project (a plain card), one of your buildings (a block with windows), a building slot (a dashed outline) |
| Tags (ringed circles) | Science, Industry, Cyber, Power, Diplomacy, Security, each its own symbol; an empty ring is any tag |

### Marks

| Mark | Means |
|---|---|
| Number before an icon | how many (no number: 1) |
| Minus before an icon | pay it or lose it, and only that: every cost you pay prints it, an Action's too |
| Less (a number over a double chevron down) | that much less: a discount, or a jam |
| `≤`N | up to N, your choice |
| `≥`N | at least N |
| `/` | per |
| OR, ALL, ACT | written out; ACT is an Action granted |
| Brown box above the art | income, each round |
| Brown box with a track arrow | raise your income of that resource for the rest of the game; below the art, or after an Effect's `:` |
| **Dark frame** | acts on the adversary: its pieces, projects or threats. Inside it a minus number means "remove up to N" (the adversary loses them) |
| Target box | the header's tab: the target's condition and glyph; the card acts on that hex |
| A place after a piece | where: after a cube, only that kind of site (the any-site glyph: any of the three); after a botnet, agents or ambassadors, on or next to that place; the near mark, on or next to the target; after a country, in the target's country |
| Dashed outline | another player |
| Thin solid outline | one of your cards: a disc in it goes to, or comes from, one of your cards |
| A disc alone | a token on this card ("here") |

### Connectors

| Mark | Means |
|---|---|
| `→` | an Action: pay the left, get the right, once a round. A bare `→` is free. |
| OR at a line's start | the other half of the Action above: one use a round, either half |
| `:` | an Effect: when the left happens, get the right. The less mark after it: those cards cost that much less |
| A number in a ring, then discs, left of `:` | At N: when the tokens here reach N, they come off and the right happens |

### Special marks

| Mark | Means |
|---|---|
| Play (a card with an arrow onto it) | a card you play, with its tag if any; in a dashed outline, another player's |
| Node | place a node, paying its track space |
| Move (a figure with an arrow) | move ambassadors onto pieces; with a pin after it, all onto one piece |
| Shield | one stop on one piece: a hardening token, on a facility or with a node |
| Roof | cover over many pieces, always with a node: a Shelter (§13), or a capstone's standing cover (Aegis Doctrine, Sanctuary) |
| Sway (a marker on a track) | add that much Unity, or Discord on a card that says so |
| Peek (a card with an eye) | look at the next event cards under the face-up one |
| Threat (`!`) | a threat; struck through: pay it off |
| Jam (a threat with a down arrow) | after the less mark, outside the dark frame: a threat needs that much less of each resource |
| Clock | a delay only: the threat waits a round |
| Bricks | no agent may arrive: after an agent and a hex, a firewall; after an agent and a country, a cordon (not the Protector's Wall, which stops troops) |
| Requirement plates | yellow: at least; stacked cards: built over |

**When an Effect fires:** once per action that sets it off (§7); a per-tag play trigger ("+1 per Diplomacy tag on
it") fires once and pays per tag.

*Sources: docs/card-guidelines.md (Legend), rules/card-grammar.md (Boxes, Connectors, Shapes);
crates/aic-core/src/face.rs, modules/cards/triggers.rs.*

---

## 24. Glossary

- **Agent:** the adversary's piece. **Ambassador:** a player's piece, any player's.
- **Arrival:** the adversary putting agents on a hex; only that hex is checked for corruption, §6 and §15.
- **At N:** §7, *Tokens on cards*.
- **Blocked:** a site with an opposing piece on it; it makes nothing, §2.
- **Building:** a card. **Facility:** a research station or factory on the map.
- **Compute:** what the adversary spends on projects, §6 and §17.
- **Corrupt number:** how many agents corrupt a site, §6.
- **Corrupted:** owned by the adversary. **Neutral:** owned by nobody.
- **A country you're in:** a country where you have a node. **Your home country:** the one you start in.
- **Discord:** the adversary's track; 7 Discord cards, and an 8th tip loses, §12. **Unity:** your track, §12.
- **Draw:** take the top tier-1 card, free.
- **Escalate:** each site of last round's Infiltrate type, not yet the adversary's, holding an agent takes 1 more, §6.
- **Firewall, cordon:** §16.
- **Garrison:** where a new troop goes: the lowest-numbered corrupted factory, else corrupted city, else city, §11.
- **Guard:** a troop on or next to a corrupted site; the site can't be liberated, §11.
- **Hardening token:** one stop on one facility or node, §13.
- **Here:** on this card.
- **In reach:** on or next to one of your nodes (for troops, also on or next to your ambassadors).
- **Infiltrate X:** an event line: 1 agent onto each site of type X not yet the adversary's, §6.
- **Land:** a city or open hex (any hex but ocean).
- **Liberate:** your ready ambassador's act on a corrupted site: the cube comes off, the site is left neutral, §8.
- **Megaproject:** a megacorp's 8-cube card, §8.
- **Near (a threat):** your node or ambassador on or next to one of its tokens, §10.
- **Next to:** sharing an edge. **On or next to:** that hex, or one sharing an edge with it.
- **On your cards:** on your programs, your buildings (covered ones too), your board and your track spaces.
- **One output:** what each corrupted site makes in the Income beat, §6.
- **Player order:** Builder, Protector, Researcher, second Builder, §3.
- **Ready, tipped:** an ambassador that may still move or act this round, or has, §8.
- **Redeploy:** an event icon: each troop moves onto its country's corrupted city nearest a player piece, §11.
- **Remove up to N:** as many as there are, up to N.
- **Research row:** the adversary's six icon slots; slot 7 loses, §6.
- **Shelter:** none of your nodes or facilities in your reach can be destroyed until cleanup, §13 (a card's Shelter
  takes no target).
- **Site:** a city, research station or factory.
- **Stage:** I, II or III, printed on each event card, §6.
- **Strength:** Unity and Discord cards resolve at strength I, II or III (by tip count), §12.
- **Strike:** an icon: every troop destroys one of your pieces on or next to it, §11.
- **Sway:** add Unity or Discord to its track. **Tip:** §12.
- **Target:** the one hex in your reach a card that touches the map acts on, meeting its target box, §2.
- **This country's player:** the player whose home country a threat was dealt in; its harm hits them, §10.
- **Tier:** player cards and threats come in tier 1 and tier 2 (decks).
- **Troop:** always the adversary's, §11.

*Sources: rules/README.md (Terms), docs/player-aid.md.*
