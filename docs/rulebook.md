# AI Complete: Rulebook (third draft)

*Rules revision 132. This book teaches the game as the simulator plays it today. Every number is a candidate. The AI
boards, the threats, the adversary's cards and the Unity and Discord cards are placeholders, so card text is left out
except where an example needs it. Part 1 teaches the game in the order you play it; Part 2 is the reference. Each
section ends with a small **Sources** line for reviewers: the rules files ([rules/](rules/README.md)) and code it
states. What is still hard to say, and the rule change that would fix it, is ranked in
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
11. [War](#11-war)
12. [Unity and Discord](#12-unity-and-discord)
13. [Protecting your pieces](#13-protecting-your-pieces)
14. [Winning and losing](#14-winning-and-losing)

**Part 2: Reference**

15. [Corruption in full](#15-corruption-in-full)
16. [Moving and removing pieces](#16-moving-and-removing-pieces)
17. [Projects](#17-projects)
18. [Threat details](#18-threat-details)
19. [Drones in full](#19-drones-in-full)
20. [Powers and capstones](#20-powers-and-capstones)
21. [Difficulty](#21-difficulty)
22. [Player count and solo play](#22-player-count-and-solo-play)
23. [Reading a card: the icon legend](#23-reading-a-card-the-icon-legend)
24. [Glossary](#24-glossary)

---

# Part 1: Learning the game

## 1. The game in brief

You and up to three other players are good AIs working together against one evil AI, **the adversary**. The adversary
sends its **agents** into the world's cities and facilities. When enough agents crowd in, they **corrupt** the place
and it becomes the adversary's. It also raises troops and pushes the world toward **Discord**. You spread your
**nodes** over the map, send your **ambassadors** to free places and claim facilities, and build a tableau of cards that pays
for it all.

Everyone wins or loses together.

- **You win** the moment the map is **purged**: the adversary holds none of what the purge asks for. At first that is
  every corrupted city, botnet, adversary troop and agent. As the world swings toward **Unity**, it asks for less; in
  the end, only the corrupted cities (§14).
- **You lose** if any of these happens:
  - the Discord deck is empty when Discord needs a card. **The Discord deck is the game's clock**: 7 cards, never
    reshuffled (§12);
  - any player has no nodes left on the map;
  - the adversary's event deck runs out: 13 rounds at most at levels 0-2, 11 at the hardest levels.

**You see the adversary's next move.** Next round's adversary card is face up all round, and the agents waiting to
land are in open view. (The threat, Unity and Discord decks stay hidden.)

**Team choices.** Where the rules say "the players choose", decide together. If you can't agree, the first player in
player order (§3) decides.

*Sources: rules/README.md (Terms: players choose), rules/alignment.md (The purge, Losing), rules/adversary.md.*

---

## 2. The map

### Countries and hexes

There is one **country** per player. A country is a square tile of 16 hexes, numbered 1-16 row by row. Each country
also belongs to a **megacorp** (Horizon, Cintamani, Onyx, Biomia), which has its own **megaproject** card and its own
**drones**. The megacorps are on nobody's side.

A hex is a **city**, **open** land or **ocean**. Cities and open hexes are **land**. Some open hexes are printed as
**industrial zones**: factories are built only there. A city next to ocean, or printed as coastal, is a **coastal
city**. Nothing is ever built on ocean, and no piece enters it.

Two hexes are **next to** each other if they share an edge, across country borders too. "**On or next to** a hex"
means that hex or one next to it. When the rules break a tie between hexes, the lower country number goes first, then
the lower hex number.

### Sites

**Cities**, **research stations** and **factories** are **sites**. (Research stations and factories are
**facilities**. A **building** is always a card, never a piece on the map.) Each site is in one of three states, shown
by the cube on it:

| State | Cube | A city | A research station | A factory |
|---|---|---|---|---|
| **Neutral** | none | 1 energy to a player whose node is on or next to it (its **host**: one city each, two with Twin City Compact; one host a city); only a city no one hosts may be bought (Buy energy, §8) | 1 neutral cube onto its megaproject | nothing |
| **A player's** | that player's | never: **no player owns a city** | 1 cube in its owner's color | 1 chip to its owner |
| **Corrupted** (the adversary's) | adversary | 1 Discord | 1 adversary cube onto its megaproject; once that is complete, 1 cube onto the adversary's research track | 1 adversary troop on its hex |

Sites make this in production (§9), except a neutral city's energy bought in the action phase (§8) and a corrupted station's research cube
once its megaproject is complete (the adversary's Income beat, §6).

**Blocked.** A site with an **opposing piece** on it makes nothing. An agent blocks a neutral or player site. A
player's ambassador blocks a corrupted site, so a corrupted city with your ambassador on it makes no Discord. Nodes
never block, and a node doesn't own the hex it is on (a node on or next to a neutral city hosts it: the city stays its country's).

### Fronts and military bases

A **front** is a long edge shared by two countries, marked on the tiles. With 2 countries there is one front; with 3
or 4, each country has two. Each country has a **military base** for each of its fronts, on an open hex by that
border (the layout for your player count shows where). A base is not a site: it is where its megacorp keeps its **drones** for that front (§11). Ambassadors may
stand on a base. A one-player game has no fronts and no bases.

### Pieces

| Piece | Whose | In brief |
|---|---|---|
| **Node** | a player | where you act from (one node per hex, of any player) |
| **Ambassador** | a player | claims and frees sites, blocks, guards; an **ambassador** means any player's |
| **Agent** | the adversary | lands on sites, blocks them, corrupts them |
| **Botnet** | the adversary | lasting token: makes compute, counts against the purge |
| **Adversary troop** | the adversary | marches and raids player facilities (§6) |
| **Drone** | a megacorp | waits at a base and fights across its front (§11) |
| **Owner cube** | a player or the adversary | who owns a site |
| **Fallout** | the adversary | a corrupted open hex, the one scar; nothing is built or placed on it; 1 Discord a round (§6) |
| **Hardening token** | — | saves one facility or node once (§13) |
| **Location token** | — | marks where a threat sits (§10) |
| **Firewall** | — | no agent lands on its hex (§16) |
| **No-Infiltrate, no-Muster, no-Corrupt** | — | that adversary step skips its hex (§12) |
| **Round-flag marker** | — | an effect that outlasts its card (Rolling Blackouts' and Sabotage Teams' flags run through next round) |
| **Compute income track** | the adversary | on the adversary board: the income event cards and projects raise |
| **Project compute tracks** | the adversary | beside each project slot: its compute so far, a marker (§17) |
| **Income tracks** | a player | on your board, one marker a resource: your income, moved when something raises it (§9) |
| **Tag dials** | a player | on your board, one a tag: the tags you have in play (§7) |
| **Used markers** | a player | on a power's, building slot's or converter's used space, or a bought chip on a city: used this round (§5) |
| **Tip tracks** | — | Unity's and Discord's tip counts, each space printing its card strength (and Unity's the purge's) (§12) |
| **Waits token** | — | on a threat that takes effect a round late (§10) |
| **Research track** | the adversary | on the adversary board: a marker counts its research cubes up to the threshold, when its research rule fires (§6) |

### Reach

A hex is **in your reach** if it is on or next to one of your nodes. Everything you target on the map must be in your
reach. Your ambassadors don't give you reach.

**One target.** A card that touches the map names **one target**: a hex in your reach that meets the condition in
its **target box**, printed in the card's header ("Target: a corrupted city"). You pick the hex when the card
resolves (an Action when you use it, an Effect each time it fires). Board powers and standard actions that touch the
map do the same: their words open with the target ("Target: Troops. Remove up to 2 on it"). The card then acts **on it**, or **on or next to
it**, or **in its country**, exactly as printed, and never further: Back-Channel Talks targets a corrupted factory
and frees "it and 1 corrupted factory next to it". **Gain** puts pieces in your supply; **Place** puts new ones on the
map, on the target (Amnesty Decree: "Place 2 Ambassadors on it"). With no hex meeting the condition, the card's map
effects do nothing.

*Sources: rules/board.md (Country pieces), rules/ambassadors.md (One site model, Blocking), rules/war.md (Bases),
rules/README.md (Terms); content/map.json; crates/aic-core/src/map.rs, modules/common.rs.*

---

## 3. Setup

**Choose** the number of players (1-4) and a difficulty level (0-4, §21; level 0 to learn).

1. **Lay out the map** for the number of players, as its layout shows: one country each, and a military base on
   each base hex.
2. **In each country** (each tile prints these places):
   - its marked city gets an adversary cube: it starts **corrupted**;
   - its marked research station gets an adversary cube and a **botnet**;
   - its other printed research stations (one next to the printed **starting hex**) and its printed factories
     start **neutral**: every city starts with a neutral factory and a neutral station next to it. Its printed
     **empty station sites** stay empty until a card opens a station there (Open Research Campus, yours; Shell Labs, neutral);
   - its megaproject card goes beside the map, empty.
   No agents or troops start on the map.
3. **The adversary:**
   - its agent supply is 24 per country;
   - build the **event deck**: deal the number of stage I, II and III cards your level asks for (§21; levels 0-2:
     5, 5 and 3), shuffle each stage on its own, and stack them with stage I on top. Turn the top card face up;
   - shuffle the project deck and fill the adversary board's project slots (§17).
4. **Shuffle** the Unity deck, the Discord deck, the two threat decks and the two player decks. The Unity and Discord
   tracks start empty.
5. **Players** take AI boards in this order, which is the **player order**: Builder, Protector, Researcher, then a
   second Builder. Sit in player order: cards pass around the table by it (§7). Player 1 plays in country 1, and so
   on. With 2 players, pick any two of the three boards; solo, only the Builder plays. Each player:
   - places the first node from their **economy track** on their country's starting hex. This uncovers the track's
     first space, which shows nothing (§5);
   - takes 1 ambassador, 3 energy and their board's starting credits;
   - owns no city: every starting hex is next to a city, and a neutral city next to your node makes its energy for
     you (§9).

Then play round 1 (§4). Its round start and Sway step come first, as in every round, so the adversary's turn comes
before anyone acts.

*Sources: rules/board.md (Setup), rules/adversary.md (Income, The event deck), rules/ai-boards.md;
configs/baseline.json (`start_corrupted` 1, `start_ambassadors` 1, `supply_per_country` 24, `start_energy` 3);
crates/aic-core/src/modules/board/mod.rs, modules/adversary/, modules/ai.rs.*

---

## 4. The round

Each round runs in this order. Steps **C** and **D** are your choices; the rest you resolve.

| | Step | What happens |
|---|---|---|
| | **Round start** | Set your insight (§5). Refresh your powers, Actions and Dispatches: clear your used markers (the player aid's *Once a round*). |
| **A** | **Sway** | Each Fallout token adds 1 Discord (§12). |
| **B** | **The adversary's turn** (§6) | Three beats: **Card** (threats, agents, troops, new facilities), **March**, **Income** (the next card, botnets' agents, compute). |
| **C** | **Cards** (§7) | Build a hand, draft, keep. Round 1 first deals the opening hand. |
| **D** | **Action phase** (§8) | All players at once: play cards, use powers and Actions, place nodes, dispatch ambassadors, pay threats. |
| **E** | **Production** (§9) | Everything pays (the income step), then extra drones. |
| **F** | **Threats** (§10) | Every threat not paid off takes effect, one that waited from last round included, and lands an agent. |
| **G** | **War** (§11) | Every front fights. |
| **H** | **Cleanup** | Shelters and other this-round effects end. |

So a threat is dealt in B, paid off in D, and takes effect in F. There is no window to act before the adversary: what
you set up in D (an ambassador, a hardening token) is what meets the next round's adversary turn.

**After every action and every step**, check the tracks: put any Unity just gained on its track or against Discord,
then tip any full track, Discord first (§12). Then check the ways to lose, then the purge (§14). If you would win and
lose at once, you lose.

*Sources: rules/economy.md (The round); configs/baseline.json `round`; crates/aic-core/src/rules.rs,
modules/alignment.rs.*

---

## 5. Your AI board

Each player is one AI board: the **Builder**, the **Protector** or the **Researcher**. Your board shows a **special
rule**, your **income**, one **tag** (§7), your **building slots**, your **Dispatches**, two **node tracks** and eight to
ten **powers** (Protector 8, Researcher 9, Builder 10).

| Board | Special rule | Income each round | Insight a round | Slots | Tag | Dispatches | Starting credits |
|---|---|---|---|---|---|---|---|
| Builder | *Breadth*: 5 slots | 10 credits, 1 energy | 3 | 5 | Industry | 1 | 27 |
| Protector | *Wall* and *Vigil* | 9 credits, 2 energy | 3 | 4 | Security | 1 | 31 |
| Researcher | *Patience* | 7 credits, 1 energy | 3 | 4 | Science | 2 | 27 |

- **Patience** (Researcher): all your unspent insight carries over to the next round, including insight another
  player gave you.
- **Wall** (Protector): **no troop is ever placed on a hex with your node** (a Muster's, a factory's, a botnet's, a
  threat's: it stays in the supply), and none moves onto it or strikes it, so that node and a facility on its hex are
  safe from them; a piece next to it is not. Only troops are stopped: corruption, war and threats still destroy what
  is there.
- **Vigil** (Protector): the first two times each round an adversary troop ends its march (moved or not) next to one
  of your nodes, gain 1 insight; later troops that round pay nothing. A troop that strikes doesn't count. It pays in any march: the
  adversary's turn's (spend it this round) or the one a threat starts at once (that comes after the action phase, so
  its insight is lost).

### Resources

Nothing has a holding limit, and nothing caps your income.

| Resource | Kept between rounds? | Mainly from | Mainly spent on |
|---|---|---|---|
| **Credits** | yes | income, cards, track spaces, megaprojects, Growth (Buy In, Annex, Accord, Levy, Peer Review) | card costs, standard actions, Lab II, threats, a neutral city's energy |
| **Energy** | yes | income, the city (or two) your nodes host, neutral cities (bought), Horizon's two converters, cards | it powers things: placing nodes, powers (Sprawl, Outpost, Accord, Patrol), building Actions, cards, threats |
| **Insight** | **no** | set each round (below) | keeping and playing cards, the Researcher's Peer Review, Field Station and Lab, threats |
| **Chips** | yes | your factories, cards, the Builder's Scrap and Full Capacity | buildings, research stations, Fortify, Annex, chip converters, threats |
| **Antimatter** | yes | the antimatter plants, the steady source (Penning Trap Array, Antiproton Foundry, Positron Storage Ring, Dark Fiber Collider, Treaty Isotope Bank, Cyclotron Lease: a resource each use for 1, once a round, some with a second gain beside it), the Builder's Overcharge II, two capstones, cards; at worst 5 research cubes (with 2 energy) | the majors: tier-2 cards (liberations, Annihilation Lance, Fusion Commons, Antimatter Accord, Collider Consortium and more), Orbital Relay Launcher's Action, Containment Reactor, Accelerator Diplomacy |
| **Research cubes** | yes | your stations, once their megaproject is complete (§9); Cryo Sample Vault; Open Archive; cards (Field Grant, Particle Accelerator, Collider Consortium, Survey Drone Fleet) | 1 for 1 insight once a round, converters, Penning Trap Array, Field Assay Team, Research Reactor, Survey Drone Fleet, threats |
| **Ambassadors** | yes (your supply) | 1 at the start, cards, Accord, 13 credits | liberating, Remove agents, Envoy, threats |
| **Dispatches** | **no** | your board and track spaces, each round | the Dispatch action, 1 each |

**Insight is this round's attention.** Your card-play track prints it, as in Spirit Island: 3 at its start, and on
later spaces the new total, each board at its own rate (the Builder 7, 8, 9, 10; the Protector
and the Researcher 7, 8, 9): a big jump on the first space, then a step at a time. At the start of each round, set your insight to the number on the rightmost
space you've uncovered that shows one (3 before any), plus what your cards add. Insight you don't spend is lost (except under Patience),
and no threat or project takes it: it is spent the turn it comes. Everything else on your
board and cards pays at the end of production's **income step** (§9).

### Node tracks

Your nodes start on your board, on two tracks of 7 spaces: the **economy track** and the **card-play track**. To place a
node, take the next one off either track. That uncovers the space under it, and what the space shows is yours for the
rest of the game: income, a new insight total (card-play track only), Dispatches, a tag, a free research station, the
**keep upgrade** (§7), and on the last space a **capstone**, a lasting rule (§20). The card-play track does not limit how many cards you play.

**Every placement pays the energy printed on the space** you uncover, whatever places the node; a power or card
that places one costs only its own price on top (Buy In, say: nothing):

| Space | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| Economy track: energy to pay | free | free | free | pay 1 | pay 2 | pay 3 | pay 5 |
| Card-play track: energy to pay | free | free | pay 1 | pay 2 | pay 2 | pay 3 | pay 5 |

> **Example.** The Builder has placed 3 economy nodes (the setup node included) and none from the card-play track.
> Placing a node onto the economy track's fourth space with Buy In costs 1 energy (the space's); onto the
> card-play track's first space, nothing. Sprawl places two (each pays its space) and adds its 1 energy.

**Where:** on a land hex in your reach with no node of any player and no Fallout. A hex
with agents or a corrupted site is allowed (a node doesn't remove agents).

**A destroyed node leaves the game.** It does not go back to its track; the space stays uncovered.

### Powers

Powers are used in the action phase. Every board's **Growth** places your nodes: pick **one** of its three options a
round, one placing 2 nodes for a price in the board's currency (the Builder's energy, the Protector's energy, the
Researcher's insight), the others 1 node with a little more (§20). There is no standard action for nodes. A Growth
option needs a node to place: a token, a space whose energy you can pay and a free hex in its range; with none you
can't use it, not even for its extra; a two-node option needs both nodes' spaces paid. So up to 2 nodes a round come from Growth; cards can place more. Each board's
other powers are **tiered**: pick one tier of each a round, a higher tier costing more for more. Most are paid by
**sacrificing** cards with a tag (§7, *Tags*); a **threshold** power only needs the tags on your cards. §20 lists
every board's.

*Sources: rules/ai-boards.md, rules/ai-powers.md, rules/economy.md (resource table); content/ais.json;
crates/aic-core/src/modules/ai.rs, modules/economy.rs, modules/board/node.rs, modules/board/troops.rs (Vigil),
modules/cards/sacrifice.rs.*

---

## 6. The adversary's turn

The adversary has a **board**, an **event deck**, a **project deck**, and a supply of agents, troops and botnets.
Each event card shows its **stage** (I, II or III): the stage sets which threats it deals (§10). There is no agent
income: the cards say where agents land.

The adversary in this book is **the Militarist**. Its board has three project slots and three rules:

- **Botnet Recruiters:** in the Income beat, each botnet with no player's ambassador on it **seeds** 1 agent (below).
- **Warmongering:** at the end of production (§9), each military base with an adversary troop on or next to it gains 1
  extra drone, even past its limit (§19).
- **War Takes Ground:** when a drone destroys a factory in war (§11), it also destroys a node on that hex (§13
  applies to each).

### The turn: three beats

The adversary's board prints its turn as a strip of three beats, played in order.

**1. Card.** The face-up event card becomes this round's card (if there is none, you lose). Then, in order:

- **Threats.** Its first line deals threats (§10).
- **Infiltrate X.** Every event card has one; stage-III cards have two. **1 agent** (2 at level 4) lands **on each
  site of type X** (cities, coastal cities, research stations, factories or hexes with a botnet) in every country,
  whoever owns it, except the adversary's own. A firewalled site (§16) or one with a no-Infiltrate token (§12) takes
  none: its agent stays in the supply.
- **Muster N.** N adversary troops land in each country on its lowest-numbered factory (any owner's); with none, on
  its lowest-numbered city. Only the stage-III cards print one: troops are few, and they come late.
- **Other lines**, at most two, each with its own icon: a compute income raise, a new neutral factory or research
  station in each country, Discord, Unity taken back, a blackout, node loss by a front, a forced march, botnets
  spreading, Fallout. **Botnets spread** puts a botnet in each country onto its
  lowest-numbered site (city, station or factory) that is next to one of its botnets and has none of its own, or
  nowhere if it has no such site; a country with no botnet gets one on its lowest-numbered research station. The
  line prints "(level 1+)", as Botnet Swarm's does: at level 0 skip it (§21). **Fallout** drops 1 Fallout in each
  country next to its lowest-numbered corrupted city.
- **Spill (level 2+): X into Y** (most stage-II and stage-III cards). In each country, each X not yet the
  adversary's holding **2 or more** agents moves 1 of them onto the Y next to it, not yet the adversary's, holding
  the most agents (a tie: the lowest-numbered; none: it stays). Below level 2, skip it.
- **Grow: X** (most cards). In each country, the X holding the most agents (a tie: the lowest-numbered), of those
  with a free hex next to it and fewer than 2 facilities (anyone's) next to it, builds one **neutral** facility next to itself (a card may claim it, §8): whichever of
  factory and research station it has **fewer of next to it**, counting anyone's (a tie: a factory); with no room for
  that kind, the other. A factory goes on the lowest-numbered free industrial zone, a station on the lowest-numbered
  free open hex that is no industrial zone and no base ("free": no factory, station or Fallout; a node doesn't
  count). If the X that grew holds agents or is the adversary's, the facility comes with **1 agent** on it.

The escalation is on the cards: stage III names two Infiltrate types, and from level 2 Spill gathers agents onto
fewer sites.

**2. March.** Adversary troops march (below).

**3. Income.** Turn the next event card face up: its Infiltrate and Grow lines tell you where next round's agents
and facilities go. Then each botnet with no player's ambassador on it **seeds** 1 agent onto the city, station or
factory next to it, not yet the adversary's, holding the most agents (a tie: the lowest-numbered; with none, onto its
own site if that isn't the adversary's). Then all the adversary's compute arrives: its income, each botnet without a
player's ambassador on it, and what was made since the last turn, onto the projects (§17). Then the **research
track** is checked (below). Firewalls, cordons (§16) and no-Infiltrate, no-Muster and no-Corrupt tokens (§12) come
off.

**A Muster** lands what it prints at every level (§21). A troop that
would land on a hex with the Protector's node (Wall, §5) or a no-Muster token stays in the supply.

Whenever an agent lands where players' ambassadors stand, it cancels one of them, one for one (§8). A troop landing
does nothing to them until it marches (below).

### Corruption

A site's **corrupt number** is how many agents it takes: city **5** (**3** from level 1: Sleeper Cells, §21), station
**2** and factory **2**, printed on the map's sites. Only agents count.

**Corruption happens on arrival.** Whenever the adversary puts agents on a hex (an Infiltrate, a Spill's move, a
Grow's agent, a botnet's seed, a Surge, a threat's or card's agents), check **that hex alone**:

- a city, station or factory not yet the adversary's holding its corrupt number **corrupts**, and its agents go back
  to the supply;
- otherwise, **2 agents on your node's hex destroy your node**, whatever else is there (a city with your node loses
  the node at 2, and corrupts only at its own number). The agents go back to the supply, and the node leaves no mark:
  losing it is punishment enough. Protections apply (§13).

Nothing else on the board is checked: a site only reaches its number when agents land on it. Agents a player moves
(Gather, Push, §16) are no arrival and check nothing. Nothing on a firewalled hex (§16) or one with a no-Corrupt token
(§12) corrupts or loses a node.

> **Example: a Spill.** A neutral city on hex 6 holds 2 agents; the factory on hex 7, next to it, holds 1. At level
> 2 a card prints "Spill: cities into factories": the city holds 2 or more, so 1 agent moves onto hex 7, the factory
> next to it holding the most. Hex 7 now holds 2, its corrupt number: it corrupts, and its 2 agents go back to the
> supply. The city keeps 1. Had your ambassador stood on hex 7, the moving agent would have canceled it instead.

Projects print **Surge N: X**: in each country, N agents onto the X not yet the adversary's holding the most agents
(a tie: the lowest-numbered), one arrival. Some threats and projects corrupt the site they name outright.

**Whatever corrupts takes the adversary's mark** (an arrival, a threat or a project alike), and the agents on it go
back to the supply: the site takes the adversary's cube, replacing any owner's cube. Any node on that hex is
destroyed (unless something protects it, §13), then every ambassador on it is removed.

**Fallout** is the one scar on open land. Nothing can be built or placed on it (no node, factory or research
station), and each token adds 1 Discord every round until a player lifts it (Lift Fallout, §8; some cards). Threats
drop it on their location's hex if it is free, else on the first free hex next to it (§2's tie order); event cards'
Fallout lines, projects and the Discord card Scorched Earth next to the corrupted sites they name: never on a node, a
facility or another Fallout.

§15 has every detail.

### The march

Troops are few, slow and deadly. An adversary troop's **targets** are players' **nodes** and player-owned research
stations and factories, never anything else. In the March beat, the troops on each hex march together as one **stack**, one
stack at a time (country by country, in hex-number order), **1 step** each; troops already on a hex a stack enters
march later, on their own. A stack's step:

- troops **ignore ambassadors**: they neither stop for them nor remove them (agents deal with ambassadors; troops hit
  your pieces). Your own ambassador on the hex still protects your node or facility from a strike (§13);
- while a target is **on or next to** the stack's hex, one of its troops **strikes** one (the players choose which, if
  several): the target is destroyed unless something protects it (§13). Either way that troop is used up and leaves
  the map;
- the troops left **move** together one hex toward the nearest target, by land and around Wall hexes, crossing borders
  if they must (the players choose among equally good hexes, once for the stack). With no way to any target, they stay.

So a troop takes time to arrive: one two hexes from your node or facility steps next to it this turn and strikes it
(or another target in range, if the players pick that one) next turn, which leaves you a round to answer it. A troop
mustered onto your factory, or next to your node, strikes in the same turn's march. (Forced March gives every troop
an extra step that round.) The march stops the moment a player's last node falls: the game is lost (§14).

> **Example: a march.** An adversary troop stands two hexes from the Builder's factory, and no other player piece
> is nearer. This turn's march: no target on or next to it, so it moves one hex toward the factory. In the action
> phase the Builder hardens the factory. Next turn's march: the factory is next to it, so it strikes. The Builder
> spends the hardening token: the token is removed instead, and the factory stands. The adversary troop is used up
> either way. The Builder could instead have removed the troop with a card, spent their own ambassador on the
> factory, or nothing (§13); a drone at a nearby base never helps.

### Projects and botnets

The adversary spends **compute** on **projects** in the Income beat, always on the **leftmost**; a project that fills
**resolves** with its printed effect, and the rest slide left (§17). Some cards and the Researcher's Hack II take compute off a project.

Botnets spread quickly (four event cards print **Botnets spread (level 1+)**, above) but are cheap to clear. A **botnet** stays
until a card or the **Take down a botnet** action (§8, 13 credits) removes it. It makes 1 compute a round
unless a player's ambassador stands on it, and counts against the purge until Unity has tipped 7 times (§14).
Removing one sways neither side.

### Research: the Weapons Program

Every adversary board has a **research track** and a research rule of its own. Cubes go onto the track from:

- each corrupted research station whose country's megaproject is complete: 1 a round, in the Income beat (none while
  a player's ambassador stands on it);
- the projects Backdoor Research, Lab Seizure and Station Coup: 2 a country;
- the threats Wiper Malware, Data Breach and Zero Day, when they execute uncountered.

In the Income beat, after the compute, if the track holds the **threshold** (5 cubes a country at level 0, 4 from
level 1: its marker, §21), spend that many (the rest stay) and the rule fires, once a round at most. The Militarist's is the
**Weapons Program: Muster 2** (each country, 2 troops on its lowest-numbered factory, else city). At level 3 and up
it is **Muster 3** (level 3's rule, Arms Race).

Take cubes off with **Counterintelligence Sweep** (remove 4) and **Arms Inspectors** (remove 2 when built,
and an Action: remove 2). The track is public: watch it as you watch the projects.

*Sources: rules/adversary.md (Each round, Income, Infiltrate X, corruption, Botnets), rules/war.md (the march);
content/adversary.json; crates/aic-core/src/modules/adversary/, modules/plan.rs, modules/board/troops.rs.*

---

## 7. Cards

### Decks and kinds

There are two player decks: **tier 1**, and the stronger **tier 2**. A card is one of three kinds:

- **Programs** (green) resolve when played and stay in front of you. Most lasting **income** comes from programs.
- **Buildings** (blue) go into a slot on your AI board and keep working: an **Action** you may use once a round, an
  **Effect** that pays each time its trigger happens, or an income each round.
- **Events** resolve and go to your played pile. They never come back, and their tags don't count.

Every card has 1-3 **tags**: Science, Industry, Cyber, Power, Diplomacy, Security.

**Draw** means take the top tier-1 card into your hand, free. When a deck runs out, shuffle its discards into a new
deck. There is no hand limit, and no limit on how many cards you play: **draw is precious**, so cards are what run
short. On a card, drawing 1 is a good benefit, 2 very strong, 3 exceptional. Only three buildings draw again and
again, each at the end of a tag chain: Preprint Server (Science), Command-and-Control Grid (Cyber) and Consulate Tower
(Diplomacy). No card pays you for playing *any* card: a payout for a play names a tag.

### Round 1: the opening

Before round 1's draft, each player is dealt **2 tier-1 cards** and **2 tier-2 cards** straight into their hand, kept
**free**. The tier-2 cards point you toward a build to plan for; you need not play both. Then round 1 builds a hand,
drafts and keeps like every round.

### Every round: build, draft, keep

1. **Build a hand.** Each player takes the top card of the tier-1 or the tier-2 deck, your choice each time, one at a
   time until they have 4. Each arrives face up, so you see it before choosing the next. (This is not a draw.)
2. **Draft.** Take one card from the hand in front of you and pass the rest on, until the hands are gone. Always
   pass to the next player in player order. Solo, you take all 4, then keep those you want, paying as below.
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
never count toward its own requirement. They do count for its own per-tag gains, which print "(including this)",
except on an event. A per-tag income, on a program or a building, counts the tags once, when the card is played.

**Discounts** (a discount never takes a cost below 1; a cost printed 0 stays 0):

- every 2 Science tags on your cards take 1 insight off each card you play, and off each tier-2 card you keep; with
  the floor of 1, that reaches only cards costing 2 insight or more and tier-2 keeps;
- every 3 Industry tags take 1 credit off each building;
- some cards make all your cards of one tag cheaper.

**Play a card if any part of it can act.** Parts with nothing to act on (no hex in reach meets its target box, say) are skipped. A
program or event with nothing at all to do can't be played; a building always can. A sway toward Discord is a price,
not a part that acts.

### Tags

The tags **on your cards** are those on your programs, your buildings (covered ones too), your AI board and your
uncovered track spaces. Tags unlock requirements, set discounts and per-tag payouts, and pay for sacrifice powers and
some threats. Keep the count on your **tag dials** (§2): turn a dial when a card with that tag comes into play or is
sacrificed.

**Sacrifice.** A cost that names a tag ("1 Industry", "2 Industry") is paid by discarding cards of yours in play that
carry that many of the tag in all: each a program, or a building together with every building stacked under it (a tag
on a covered building counts). Events can't be sacrificed, and neither can your board or its track spaces. Tags on the card beyond the cost
are wasted. An income the card raised (its brown track-arrow box, §23) stays raised; an income it pays while in play
stops.

### Buildings and slots

Put a building into a free slot, or **build over** one of your buildings; with no free slot you must build over one. A
covered building **stops working**: its income, Effect and Action stop, and any tokens on it sit idle. **Its tags still
count.** A card can't be uncovered.

- An **Action** (`→`: pay the left, get the right) is used once a round. "A, OR B" is one use: either half.
- An **Effect** (`:`: when the left happens, get the right) fires once for each action that sets it off (a card play,
  a building's Action, a power, a standard action), however many pieces that action removed or tokens it placed, with
  no limit a round (§23). A building can trigger itself: a Science building with a Science trigger fires when you play it.
- An **income** pays every round, in production's income step.
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

*Sources: rules/player-cards.md, rules/card-grammar.md (The engine wording, Draw, Help cards); configs/baseline.json
`cards`; crates/aic-core/src/modules/cards/ (deck.rs, play.rs, price.rs, tableau.rs, triggers.rs, sacrifice.rs).*

---

## 8. The action phase

All players act at once, as often as they like and in any order. When you **pass**, you are done until next round's
action phase: you can't play, pay or take anything back. The phase ends when everyone has passed. You may:

- **play cards** from your hand (§7);
- **use** your buildings' Actions and your board's powers, each once a round (§5, §20);
- **pay toward threats** you are near (§10);
- take **standard actions**, as often as you can pay, but Remove a troop and Turn a cube into insight once a round
  each, and Buy energy once a round a city (below);
- use a **megacorp converter** (below).

### Standard actions

Standard actions are always there, a hedge against bad luck: cards and your board's powers do the same things better.

| Action | Cost | Effect |
|---|---|---|
| **Dispatch** | 1 Dispatch | target a site, base, node or botnet: put one of your ambassadors on it, from your supply or from anywhere on the map (not the hex it is on) |
| **Liberate** | ambassadors: city 5, station 4, factory 3 (below) | target a corrupted site: it becomes neutral |
| **Remove agents** | 2 ambassadors from your supply, spent | target agents: remove up to 3 on or next to it; nothing is placed |
| **Remove a troop** | 7 credits | once a round: target troops, remove 1 on it |
| **Take down a botnet** | 13 credits | target a botnet where your ambassador stands: remove it (the ambassador stays) |
| **Buy energy** | 3 credits | target a neutral city that isn't blocked or blacked out and that no player hosts or could host (none with a host slot to spare has a node on or next to it): gain its energy (1); each city once a round, for all players together |
| **Lift Fallout** | 2 research cubes | target Fallout: remove it; the one standard action that removes Fallout |
| **Build a research station** | 2 chips | target an open hex that isn't an industrial zone or Fallout: the station is yours |
| **Envoy** | 1 ambassador from your supply | remove 1 Discord from the Discord track |
| **Gain an ambassador** | 13 credits | into your supply |
| **Turn a cube into insight** | 1 research cube | gain 1 insight; once a round |
| **Make antimatter** | 5 research cubes | gain 1 antimatter and 2 energy: the desperate way; your engines make it far cheaper |
| **Sell** | a card from your hand | gain 1 credit |

### Ambassadors meet agents

**Every contact is one for one.** When an agent lands on a hex where players' ambassadors stand, it and one of them
are removed (the agent back to the adversary's supply, the ambassador out of play: not back to your supply); if several players' ambassadors are there, the players choose
whose. When your ambassador arrives on a hex with agents, it removes one of them and is removed itself. So an agent
and an ambassador never share a hex.

**Remove agents** removes its agents one at a time, your choice, and you may stop early.

### Liberate and claiming

**Liberate** takes a **corrupted** site, a city too:

- **Where:** the site must be in your reach.
- **Price, in ambassadors:** city **5**, station **4**, factory **3**.
- **No agents:** the Liberate action can't take a site with an agent on it, and no card claims one: remove them first
  (Remove agents, a card, a power). A liberate card (below) takes the agents off itself.
- **Who pays:** your own ambassadors standing on the site pay first, then your supply. Another player's never pay.
- **Result:** Liberate **frees** it: the adversary's cube comes off and
  the site is left **neutral**, and it adds Unity: city **+5**, station or factory **+2**. A card may claim a station
  or factory afterward; a city stays its country's (no player ever owns one). A botnet on the site stays. The site
  holds no agents, so the adversary needs its full corrupt number of arrivals to take it back (§6).

> **Example: block, then liberate.** A corrupted city in your reach makes 1 Discord a round. You Dispatch an
> ambassador onto it (1 Dispatch): now it is blocked and makes nothing. A round later you Liberate it: the price is 5 (no agents
> on it), your standing ambassador pays 1 and your supply pays 4. The adversary's cube comes off and you add 5 Unity.
> The city is its country's again, and your node next to it can host it (a hosted city can't be bought).

**Liberate cards** (Popular Uprising and some tier-2 cards) liberate their target, a corrupted city in reach, as Liberate does, for the card's own price
instead of ambassadors. Hostile Buyout alone may target any corrupted site, and puts your cube on a station or factory
it frees (a city it frees stays neutral). The others **rescue** it (the cards' word): liberate it, then **claim a
facility next to the city**: a neutral station or factory there with no agent on it becomes yours, even out of your
reach; with none, nothing more happens.

**Only cards claim.** A card that lets you **claim** a neutral research station or factory in your reach puts your cube
on it: it is **yours**. One with an agent on it can't be claimed (clear it first). Open Research Campus and Industrial Commons open a station or factory
on their target and claim it: the cards' way to an owned research station.

### Megaprojects and converters

Each megacorp's **megaproject** needs 8 cubes, which research stations in its country put on it (§9). When it
completes, each cube on it pays its owner the payout below, and its **converter** opens. Any player with a **city of
that country in reach** (any city, corrupted ones too) may use the converter, **once a round** each, and the
**chip converter** where the card prints one, also once a round: chips you saved early pay here late.

| Megacorp (megaproject) | Payout, each of your cubes | Converter | Chip converter |
|---|---|---|---|
| Horizon (Tidal Grid) | 3 energy, 2 credits | 3 research cubes → 5 energy | 2 chips → 3 energy |
| Cintamani (Polar Compute Array) | 6 credits | 1 research cube → 2 insight | 1 chip → draw 1, 1 credit |
| Onyx (Mountain Fusion Works) | 2 chips, 2 credits | 1 research cube → 1 chip | none |
| Biomia (Delta Seawall) | 1 Unity, 3 credits | 1 research cube → 1 Unity | 2 chips → 2 Unity, draw 1 |

*Sources: rules/economy.md (standard actions, Cities' energy), rules/ambassadors.md (Claiming, Liberating, Every
contact, Remove agents), rules/board.md (Converters);
crates/aic-core/src/modules/board/standard.rs, modules/board/ambassadors.rs, modules/board/mod.rs
(converter_blocked), modules/alignment.rs; content/map.json (`once`).*

---

## 9. Production

Production has two parts: the **income step**, where everything pays, then **extra drones**. Do each step for the
whole map before the next.

**The income step**, in this order:

1. **Factories.** A player's factory makes 1 chip for its owner. A corrupted factory makes 1 adversary troop on its
   own hex. A neutral factory, or a blocked one, makes nothing.
2. **Cities.** Each unblocked city makes 1 energy: a neutral city with your node on or next to it for you (its
   **host**: each player hosts one city, two after playing Twin City Compact, each city has one host, the players
   choosing; a city a player hosts or could host can't be bought, §8, but a city bought earlier this round may
   still be hosted); a
   corrupted city 1 Discord instead.
3. **Stations.** A player's station makes 1 cube in its owner's color; a neutral one makes 1 neutral cube. They go
   onto their country's megaproject. Once it is complete, a player's cubes go onto their board as **research cubes**,
   and a neutral station's are lost. A corrupted station makes 1 **adversary cube** onto its country's megaproject
   until it is complete; after that it puts 1 cube on the adversary's research track instead, in its Income beat
   (§6). A player's
   ambassador on a corrupted station blocks either.
4. **Megaprojects.** A megaproject holding 8 cubes or more **completes**, once: every cube on it pays, overflow
   included (§8), then its cubes come off. An adversary cube pays the card's adversary payout (1 Sway to Discord;
   Polar Compute Array, 1 adversary compute). Neutral cubes pay nothing. From then on its stations' cubes go to their
   owners' boards (step 3).
5. **Income.** Each player collects their income (insight aside, §5): board, track spaces, programs and buildings,
   as their income tracks show.

6. **Extra drones.** Then Warmongering adds drones to bases (§19).

New neutral facilities come from the event cards' Grow lines (§6), not from production. A destroyed factory leaves
the map and frees its zone.

*Sources: rules/war.md (Every round, Factories), rules/economy.md (Cities' energy, The round), rules/board.md
(Research cubes and the corps); configs/baseline.json `round` (`board.produce`, `economy.upkeep`, `board.bases`);
crates/aic-core/src/modules/board/production.rs (produce, produce_units), modules/economy.rs.*

---

## 10. Threats

Each round the adversary deals **threats**: problems placed on the map that you pay off in the action phase or suffer
at the end of the round.

**Dealing.** The event card's first line deals one or two threats in each country, each naming one or two **tags**
(Security, Diplomacy, Cyber, Industry, Power: the tags your cards carry, with the same icons). Turn up threat cards
until one fits (§18 has the details).

**Location tokens.** A threat prints one or two location types (city, coastal city, research station, factory,
military base). It puts a **location token** in its country on a place of each type that isn't the adversary's: the one
with the most agents on its own hex (tie: the lowest number). §18 has the rest.

**Near and nearest.** These are two different tests, printed above every threat's text ("Payers: any player with a
Node or Ambassador on or next to a token. Target: the player with a Node nearest a token."):

- **Near (who can pay):** you are near a threat if one of your nodes **or ambassadors** is on or next to one of its
  tokens. Only players near a threat may pay toward it or use a card on it.
- **Nearest (who gets hit):** if the threat takes effect, its harm to a player hits the player whose **node** is
  closest to one of its tokens, at any distance, near or not (tie: the tied players choose which of them takes it;
  a harm that hits no player, such as troops or Fallout landing, needs no choice).

**Paying it off.** A threat prints one or two ways to pay it off. **Pay one way in full** and it is discarded.

- **Resources** (credits, energy, insight, research cubes, chips, or ambassadors from your supply) may be pooled by all
  the players near it. You may take back what you put on it until you pass. Anything left on a threat that isn't paid
  off goes back to its payers when threats take effect.
- **Tags:** one player alone sacrifices cards in play carrying 2 of the threat's tag in all (tier 1) or 3 (tier 2).

Cards can also **jam** a threat (it needs less of each resource), **delay** it (it waits a round, and can be paid next
round), or pay it off outright. A few buildings pay off part of every threat of a type, wherever it is.

**When threats take effect** (step F), each threat not paid off does what it prints, country by country, lands 1
agent on its first token (§18), then is discarded with its tokens. Pieces it places go on its token. A resource harm reads "pay N or discard M cards": the
player chooses; with neither possible, they pay what they have and discard their hand.

A threat dealt after the action phase ended (by a Discord card tipped later in the round, or by another threat taking
effect) gets a **waits** token: it takes
effect next round.

> **Example.** A Security threat's token sits on a research station. The Protector's ambassador stands next to it, so
> the Protector is near and may pay; the Builder's node is 2 hexes away, so the Builder is not near. Nobody pays.
> In step F the harm hits the player whose node is closest: the Builder, 2 hexes away (the Protector's nearest node is
> 3 away). The Protector's ambassador made them near, not nearest.

*Sources: rules/threats.md; content/threats.json; crates/aic-core/src/modules/threats/ (mod.rs, near.rs, counter.rs).*

---

## 11. War

The megacorps fight each other across the fronts with **drones**. These are not the adversary's troops: they
never march, and they never strike your stations.

**Where drones come from.** Each base holds up to **2** drones; a full base takes no more, except from Warmongering
(§19). Drones come:

- **each time the Discord track tips** (§12): each country with a neutral factory that has no agent and no ambassador
  on it puts 1 drone on each of its bases, however many such factories it has;
- from Warmongering, at the end of production (§9, §19).

**An ambassador on a base** stops each drone that would land there, from any source, one ambassador a drone, and is
spent. A drone already at a base stays until war uses it up.

**War**, at the end of every round (step G), on each front:

1. The two bases' piles cancel 1 for 1, and both bases are emptied.
2. Each drone left over crosses and strikes the rival corp's factory nearest its base, counting hexes (tie: the lower
   number): a neutral or player factory, never the adversary's.
3. The strike is on the **factory**, protected like any facility (§13). If it is destroyed, it adds **1 Discord**,
   and (the Militarist's War Takes Ground, §6) a node on its hex is destroyed too, protected as any node is.
4. A drone whose strike is stopped is still used up, and the next drone aims at the same factory.

Drones never protect anything: they only fight.

*Sources: rules/war.md; crates/aic-core/src/modules/board/troops.rs, modules/board/production.rs.*

---

## 12. Unity and Discord

The world leans toward **Unity** (you) or **Discord** (the adversary). Each side has its own **track**, its own **tip
count** and its own deck of one-shot cards.

### Gaining Unity and Discord

Effects **sway** Unity or Discord ("add 2 Unity"): put that many on that side's track.

- **Unity** is trust and cooperation: liberating sites (city 5, station or factory 2), Biomia,
  and cards that build or bring people together.
- **Discord** is fear, force and collateral harm: corrupted cities (1 each a round), Fallout (1 each a
  round), factories destroyed in war (1 each), threats and projects, and your own strikes (Surgical Strike, Kettle
  Operation, Martial Cordon, Kinetic Interdiction, Annihilation Lance, The Nuclear Option). Taking a botnet down sways
  nothing.

**Unity against Discord.** Whenever you gain Unity, you may put any of it against Discord instead: each Unity put
there removes 1 Discord from the Discord track. The player whose action gained it decides (the first player, for a
step of the round), as it resolves, before any track tips. The rest goes onto the Unity track. (**Envoy** also removes 1 Discord, for 1 ambassador.)

> **Example.** 3 players, so the Discord track tips at 12. Discord is at 10. One step adds 3 Discord (13) and 2 Unity.
> Putting both Unity against Discord leaves 11: Discord doesn't tip. Had the step added 4 Discord (14), the same 2
> would leave 12: Discord still tips, but to 0 instead of 2.

### Tipping

The Unity track **tips** when it holds **3 × the number of players**, the Discord track when it holds **4 × the
number of players**. Then:

1. Take one tip's worth off. The rest stays on the track toward its next tip, on both tracks.
2. Add 1 to that side's tip count.
3. **Discord only:** neutral factories arm the bases (§11).
4. Draw that side's top card and resolve it at the strength for its tip count, this tip included: **strength I** on
   tips 1-2, **strength II** on tips 3-5, **strength III** from tip 6.

If both tracks are full, Discord tips first. A track still full after a tip (what stayed, or a card that fills
it) tips again at once.

The strengths here and the purge's steps (§14) both read the tip count, at different steps: the player aid's *Tip
strips* prints them on one row a tip, with the Discord cards left.

### The cards

Every Unity and Discord card prints three strengths. Each acts across the map by a condition ("for each station or
factory a player owns, or city a player hosts"), not in one player's reach. Some Unity cards **forbid** the adversary's Infiltrate, Muster or Corrupt: put a no-Infiltrate,
no-Muster or no-Corrupt token on every hex where the card's condition holds as it resolves. That step skips a hex with
its token, and what it would have placed there stays in the supply; a no-Corrupt token lets agents land but stops every
corruption and node loss there. The tokens come off at the end of the next adversary turn, like a firewall.

- **The Unity deck** (23 cards) reshuffles when it runs out.
- **The Discord deck** (7 cards) **never reshuffles.** A Discord tip with no card left loses the game: you survive 7
  Discord tips and lose on the 8th, at every level (§21).

*Sources: rules/alignment.md; configs/baseline.json (`tip_per_player` 3, `discord_tip_per_player` 4, `carry` 2, `hold_off` 1, `tier_at` 3, 6); content/alignment.json;
crates/aic-core/src/modules/alignment.rs, modules/board/production.rs (the tip's drones).*

---

## 13. Protecting your pieces

### What can be destroyed

Your **nodes** and your **facilities** (stations and factories) can be destroyed:

| By | What it destroys |
|---|---|
| **Corruption** (agents arriving, a threat or project that corrupts a site) | the node on the corrupted hex, then every ambassador there |
| **2 agents on your node's hex** (§6) | that node; the agents go to the supply |
| **An adversary troop's strike** | the node, or player-owned station or factory, it strikes (§6) |
| **War** | the factory a drone strikes, then a node on its hex (War Takes Ground) |
| **Night Raid** (a project) | lands 1 agent on each of two node hexes in each country; a hex it brings to 2 agents loses its node at once |
| **Threats and event lines** | what they print ("destroy a node", Border War's node near a base) |

A destroyed facility takes its owner's cube with it. A destroyed node leaves the game and leaves no mark on its hex.
Corruption leaves its own (§6: the adversary's cube, or Fallout).

### The protection order

When a node or facility would be destroyed:

1. **Shelter** this round (a card shelters its player's nodes and facilities in their reach, as it stands when the
   piece would be destroyed: a node placed after the card is covered too) stops it, automatically, spending
   nothing. It lasts from the action phase to cleanup, so it covers production, threats and war, never the
   adversary's turn. It stops only what goes through this order: cubes still change hands, ambassadors are still
   removed, and a node you give up to pay something is still lost.
2. **Aegis Doctrine** (a Protector capstone) is Shelter on the Protector's own pieces once a round: the first time
   each round one of its nodes or facilities would be destroyed, it is saved, automatically, spending nothing.
3. Otherwise **its owner chooses** one protection to spend, or none (for a neutral piece, the players choose). Each is
   used up:
   - a **hardening token** on the hex;
   - **your own ambassador** on the hex (never another player's).
4. With nothing spent, the piece is destroyed.

A facility's owner is the player whose cube is on it. One hardening token on a hex with both your facility and your
node can save either, once: you may keep it for the second (it stays on the hex while your node does). Neither corruption nor agents
destroy the node of a player with **Sanctuary** (a Protector capstone), and spends nothing of theirs.

**Hardening** goes on a player-owned station or factory (the Builder's Fortify II, the Protector's Patrol II; some cards), or, where a card says
"harden N of your Nodes" (Stress Test Suite, Early Warning Drill), on the hexes of N of your nodes: the same token.

### Your last node

Nothing protects a last node but the protection order above: anything that destroys a node can destroy your last
one, at any stage. Any player with no nodes loses the game for everyone, so keep a second node on the board: the
first spaces of both tracks cost no energy, so an early second node costs nothing with a free Growth option. Watch the hexes next to a
lone node: two agents on it destroy it, and Night Raid lands an agent on two node hexes in each country.

*Sources: rules/war.md (Node loss), rules/ai-powers.md (Shelters and hardening), rules/ambassadors.md
(Armor); crates/aic-core/src/modules/board/destroy.rs, modules/board/node.rs, modules/adversary/ (corrupt_hex),
modules/effects/team.rs, modules/effects/new_ops.rs.*

---

## 14. Winning and losing

### Winning: the purge

You win the moment the map holds none of what the purge asks for. It asks for less as Unity tips:

| Unity tips | The map must hold none of these |
|---|---|
| 0-2 | corrupted cities, botnets, adversary troops, agents |
| 3-6 | corrupted cities, botnets, adversary troops |
| 7 or more | corrupted cities |

Corrupted stations and factories never count, nor do Fallout or drones.

### Losing

Everyone loses at once if:

- a Discord tip finds the Discord deck empty;
- any player has no nodes on the map;
- the Card beat finds no card to turn up (the **round cap**: 13 rounds at levels 0-2, 11 at levels
  3-4). The round after the cap still opens with its Sway step, so a Discord tip can lose the game there first; then, with no
  card left, you lose.

The adversary running out of agents is not a loss: a placement it can't make doesn't happen.

*Sources: rules/alignment.md (The purge, Losing), rules/adversary.md (The event deck); configs/baseline.json
(`purge_at`); crates/aic-core/src/modules/alignment.rs, modules/adversary/.*

---

# Part 2: Reference

## 15. Corruption in full

An **arrival** is any time the adversary puts agents on a hex: an Infiltrate, a Spill's move, a Grow's agent, a
botnet's seed, a Surge, a Discord card's or project's agents, and the agent an uncountered threat lands on its anchor
(§18). Each agent arriving on players' ambassadors cancels one of them first (§8). Then check the hex it landed on,
and only that hex:

1. **A site** (city, station or factory) not yet the adversary's, holding its corrupt number of agents, corrupts (§6),
   and its agents go back to the supply.
2. **Otherwise, a player's node** on a hex holding **2 agents** is destroyed (§13 applies) and the agents go back to
   the supply; nothing else on the hex changes: no mark.

Nothing on a firewalled hex or one with a no-Corrupt token corrupts, a node included (agents already on it stay). A
player's Gather or Push is no arrival. A site never corrupts by itself between arrivals, so one sitting below its
number stays safe until the adversary lands more there.

**Picking the hex.** Spill, Surge, Grow and a botnet's seed each pick "the one holding the most agents"; a tie goes to
the lowest-numbered hex. Spill, Surge and a seed never pick a firewalled hex; Grow may pick a firewalled site (the
firewall still keeps an agent off its own hex).

> **Example: a Grow.** "Grow: cities" in a country with two neutral cities: hex 6 holds 2 agents, hex 11 holds none.
> Hex 6 grows, if it has a free hex next to it. Next to it stand one factory and no station (fewer than 2 facilities), so it builds a neutral
> research station on its lowest-numbered free open hex that is no industrial zone and no base. Hex 6 holds agents, so
> the station comes with 1 agent: an arrival. A station's number is 2, so it doesn't corrupt yet, but the next agent
> to land there corrupts it.

**What corruption destroys:** the node on the hex (§13), then every ambassador on it. Nothing else. A node lost to 2
agents leaves no mark.

*Sources: rules/adversary.md (Infiltrate X, Spill, Grow, corruption); crates/aic-core/src/modules/plan.rs (grow_plan,
most_agents), modules/adversary/ (rally.rs, corrupt_hex, arrive_check).*

---

## 16. Moving and removing pieces

Some cards and the Protector's Patrol move pieces on the map: ambassadors (any player's), agents and adversary troops.
Botnets never move.

- **Gather up to N:** pick a land hex in your reach; move pieces into it, each from a hex next to it.
- **Push up to N:** pick a land hex in your reach; move pieces off it, each to a land hex next to it, your choice.

You may stop after any piece. Moved pieces arrive as usual: an agent moved onto players' ambassadors cancels one of
them (§8).
No agent enters a firewall; no adversary troop enters the Protector's node hex.

**Remove around a hex.** Some cards (Kettle Operation, Lure Network, Honeypot Relay) remove up to N agents, or
agents and troops, **on or next to** the card's target: the target itself counts, and you choose which pieces, a
batch at a time, and may stop early. Nothing moves.

**Convert.** Some late cards turn agents into your ambassadors: up to N agents on or next to a city in your reach
each become your ambassador where it stood. The converted agents come off first, then your ambassadors go on, so two
converted on one hex both stay; an agent the card left on that hex meets one of them, one for one (§8).

**Firewall.** Some cards put a firewall on a land hex in your reach. While it stands, no agent lands, is placed or
moves in there: an agent an Infiltrate would land there stays in the supply. Nothing on it corrupts: no arrival,
station flip or threat that corrupts its site takes it (agents already there stay). Troops pass. It lasts until the end of the next adversary
turn (its Income beat).

**Cordon.** Hardpoint Lockdown cordons a country until the end of the next adversary turn: no agents land
there from an Infiltrate, a Surge, a Spill, a botnet's seed, an unpaid threat or a card line, and no Grow builds
there (troops still land, and Gather and Push still move agents in). What it would have landed stays in the supply.

*Sources: rules/ambassadors.md (Moving pieces), rules/adversary.md (Firewalls); crates/aic-core/src/modules/effects/moves.rs,
modules/effects/endgame.rs, modules/effects/removal.rs, modules/adversary/.*

---

## 17. Projects

The adversary board has **slots** (the Militarist: three open slots that take any project). **To deal**, turn up
projects one at a time until every slot is full: each goes into the first empty slot. A project that resolves or is discarded leaves the
game; once the deck runs out, no more are dealt.

A project prints **one cost a board**: in play it costs that times the number of countries. All compute arrives in one
step, the **compute step** of the Income beat. (Keep each project's compute as a number, a marker on a track
beside it, not a pile of cubes: at the hardest level 40-80 compute arrive a round.) The compute step adds: the compute income (printed a country, so times the countries), 1 for
each botnet without a player's ambassador on it (a corrupted research station feeds the research track instead, §6),
and every compute made since the last compute step (threats taking effect, cards and projects that
print it), which waits beside the projects until then: that public pile is your warning. All of it goes onto the
**leftmost** project. Add it up before you act: a project whose compute plus that sum reaches its cost **completes
next compute step** unless you change a source (an ambassador onto a botnet, a card that takes a botnet down, Hack II). The moment a project holds its cost, it **resolves** at once, does what it prints in **every
country**, and leaves the game. No project is ongoing; one that lasts does it through something already on the table
(an income raised, pieces on the board, Fallout).

**Slide and refill:** when a project leaves (resolved or discarded), the projects to its right slide one slot left,
and the empty rightmost slot is dealt a new project at once; the rest of the compute goes onto the new leftmost.
Compute with no project to go on is lost. No project resolves outside the compute step.

**What projects do.** A project is one of the adversary's big moves: a cheap one does one thing, a costly one two or
three. It builds or corrupts a factory in each country, surges agents onto cities, stations or factories,
drops Fallout, spreads or converts a botnet, makes troops on the adversary's factories, lands agents on your nodes
(Night Raid: a hex it brings to 2 agents loses its node at once), sends the
ambassadors on a country's busiest hex home, takes research cubes off megaprojects, strips hardening, or runs the
clock. **Its target is read off the board when it resolves**, in each country: "the factory with the most agents on
its hex" and "the node hexes holding the most agents" count only the hex itself; ties go to the lowest hex number. Nothing is marked in advance: look at the leftmost project and the board to see what it
will hit, and move agents off, claim the site or firewall it first. A firewalled neutral site still counts as neutral,
so a seizure finds nothing to take in that country and your own site is safe. "Turns neutral" takes the owner's cube off a site;
"discard the top card of the event deck" brings the round cap a round nearer. Only Data Heist and Containment Breach
make you pay or discard. A botnet's troops are made on its own hex, as a factory's are; on the Protector's node hex none is (Wall).

*Sources: rules/adversary.md (The project deck); content/projects.json; crates/aic-core/src/modules/adversary/.*

---

## 18. Threat details

**Dealing.** Stage I event cards deal one tier-1 threat in each country, stage II one tier-2 (Supply Lines: one tier-1
and one tier-2), and stage III two tier-2. Each deal names one or two threat tags, or "any". For each deal, for each
country in turn, turn up the top card of that tier's threat deck (a country where every city, research station,
factory and military base is the adversary's gets none; a base never is, so this only matters alone). It stays if it carries a named tag (a **Universal**
card has every tag) **and** the country has a place of one of its location types that isn't the adversary's.
Otherwise discard it and turn up the next. When a threat deck runs out, shuffle its discards into a new deck; if a
reshuffled deck has no card that fits, take the next card anyway.

**Location tokens.** In printed order, each token goes on the place of its type that isn't the adversary's with the
most agents on its own hex; tie: the lowest hex number. The same test holds for every type (a base is never the
adversary's). A threat's two tokens never share a hex; a token with no place left gets none. Its first token is its
**anchor**: a threat that takes effect unpaid also lands **1 agent** there after its harm, an arrival (§15).

**Difficulty.** A threat costs what it prints at every level (§21).

*Sources: rules/threats.md; crates/aic-core/src/modules/threats/mod.rs, threats/counter.rs.*

---

## 19. Drones in full

- **At a Discord tip,** each country with at least one neutral factory that has no agent and no player's ambassador on
  it puts 1 drone on each of its bases that has room (2 bases at 3-4 countries, 1 at 2 countries): once a country,
  however many such factories.
- **One ambassador always stops one drone**, and is spent. A player's ambassador on a base stops the drone that base
  would get, whatever makes it. A full base takes no drone and spends no ambassador. With several players'
  ambassadors on one base, the earliest in player order is spent first.
- **Warmongering** adds 1 drone to a base even past its limit (at most 1 extra a base a round).

*Sources: rules/war.md; crates/aic-core/src/modules/board/production.rs.*

---

## 20. Powers and capstones

Each board has a **Growth** (pick one option a round: one easy option, free, and two costly ones that pull
different ways) and two tiered powers (pick one tier of each a round), used in
the action phase. A tag in a cost means sacrifice cards carrying that many of it (§7, *Tags*); a threshold power ("with 2
Science on your cards") counts the tags on your cards and sacrifices none. Every placement also pays the track
space's energy (§5).

| Board | Growth (one option) | Tiered powers (one tier each) |
|---|---|---|
| Builder | Easy: **Buy In** (a node and 1 credit). Costly: **Sprawl** (1 energy: 2 nodes), **Annex** (1 chip: a node within 2 of yours and 6 credits) | **Scrap** I (1 Industry: 1 chip) / II (2 Industry: 1 chip and play a building from your hand without paying its credits; its other costs and requirements stand); **Overcharge** I (1 Power: 2 energy) / II (2 Power: 2 energy and 1 antimatter); **Fortify** I (1 chip: target troops; remove 1 on it) / II (2 chips: remove up to 2 on it and harden a facility on or next to it) |
| Protector | Easy: **Levy** (a node and 1 credit). Costly: **Outpost** (1 energy: 2 nodes), **Accord** (1 energy: a node, 1 ambassador and 2 credits) | **Patrol** I (1 energy: target agents or troops; push up to 3 from it) / II (2 energy: the same, and harden a facility on or next to it); **Interdict** I (1 Security: target troops; remove up to 2 on it) / II (2 Security: target agents or troops; remove up to 2 troops and up to 2 agents on it; draw 1) |
| Researcher | Easy: **Field Survey** (a node and draw 1). Costly: **Peer Review** (1 insight: 2 nodes and 2 credits), **Field Station** (1 insight: target an open hex; build a research station on it without the chips, and place a node) | **Lab** I (with 2 Science on your cards, 1 insight: draw 1) / II (with 4 Science, 1 insight and 2 credits: draw 1 and add 1 cube in your color to a megaproject that isn't complete); **Hack** I (1 Cyber: remove up to 2 adversary troops in one country) / II (2 Cyber: the same, and remove 3 compute from one adversary project) |

| Board | Economy track capstone | Card-play track capstone |
|---|---|---|
| Builder | **Full Capacity**: 1 chip each round per 4 Industry tags on your cards | **Master Plan**: your buildings cost 1 credit less |
| Protector | **Sanctuary**: neither corruption nor agents destroy your nodes; 2 antimatter once, as you uncover it | **Aegis Doctrine**: Shelter on your own pieces once a round: the first time each round one of your nodes or facilities would be destroyed, it isn't (automatic, spending nothing) |
| Researcher | **Open Archive**: 1 research cube each round; 2 antimatter once, as you uncover it | **Open Science**: each income step, split 2 insight from the supply among the other players as you like; it arrives with their insight next round |

*Sources: rules/ai-powers.md, rules/ai-boards.md; content/ais.json; crates/aic-core/src/modules/ai.rs.*

---

## 21. Difficulty

The adversary's reference card prints this ladder. Levels are cumulative: each keeps every level below it, adds
its setup (set once, before round 1) and at most one named rule. Each row shows what its level changes. Your own
costs (claim, liberate, cards, actions) and the threats' costs never change. The table is generated from the rules
(`aic-sim docs`); the words below are not.

<!-- generated: rulebook_difficulty -->
| Level | Name | Setup (set once; incomes and threshold a country) | Mid-game rule it adds | Rules active |
|---|---|---|---|---|
| 0 | **The Militarist** | Compute income 0; Infiltrate 1 a site; Research threshold 5; Deck 5 / 5 / 3 (13 rounds); Sites print their corrupt numbers: city 5, station 2, factory 2 | — | 0 |
| 1 | **Sleeper Cells**: *Cells wake in every city.* | Compute income 3; Research threshold 4 | When an Agent lands on a city, it falls at 3 agents, not 5. | 1 |
| 2 | **War Footing**: *Every unanswered strike feeds the war machine.* | Compute income 5 | When a threat executes uncountered, the adversary also gains 4 compute. | 2 |
| 3 | **Arms Race**: *The labs build weapons, not cures.* | Compute income 15; Deck 5 / 3 / 3 (11 rounds) | When Weapons Program fires, it musters 3, not 2. | 3 |
| 4 | **War Economy**: *The whole economy marches.* | Compute income 18; Infiltrate 2 a site; Deck 3 / 4 / 4 (11 rounds) | — | 3 |
<!-- /generated -->

- **Setup:** put the compute income marker and the research marker on the adversary board's tracks at
  the numbers shown, each a country (times the countries in play), and build the event deck with that many stage I / II / III cards (§6). Nothing else to remember.
- **Mid-game rules** fire only on something you already resolve: agents landing on a city (Sleeper Cells), a threat taking
  effect unpaid (War Footing: its 4 compute comes at the next compute step, §17; a threat that only waits gives none),
  Weapons Program firing (Arms Race). *Rules active* counts the ones in play.
- **Infiltrate** lands 1 agent a site at levels 0-3 and 2 at level 4 (§6).
- **Before level 1,** a "Botnets spread (level 1+)" line does nothing, as it prints: the botnets set up are the only
  ones. **Before level 2,** a "Spill (level 2+)" line does nothing.

*Sources: rules/adversary.md (Difficulty, The event deck), rules/threats.md (Countering); configs/baseline.json
`adversary` (`compute_income`, `infiltrate_agents`, `spill_at`, `research_at`, the deck's dials); content/adversary.json
(the board's `levels`); crates/aic-sim/src/docs/ladder.rs (the table, from both).*

---

## 22. Player count and solo play

- **Countries:** one per player, 1-4. 2 countries share one front; 3 countries have three fronts; 4 form a ring of
  four.
- **Boards:** Builder, Protector, Researcher, then a second Builder. 2 players: any two of the three. Solo: the Builder
  only.
- **Tipping:** the Unity track tips at 3 × the number of players, the Discord track at 4 ×.
- **Projects** print one cost a board, times the number of countries in play (§17); the compute income and every
  "+N compute" line are a country's too.
- **Drones:** at 3-4 countries every corp has two bases, so a neutral factory makes 2 drones at a Discord tip.
- **Solo:** one country, so no fronts, no bases and no war. You take all 4 cards in the draft and keep those you want, paying as usual (§7). "Another
  player" on a card means you: an Effect that fires when another player plays a card fires on your own plays, and a
  shared gain is paid once.

*Sources: rules/board.md (fronts), rules/ai-boards.md (`ai.boards`), rules/war.md, rules/alignment.md (Tipping),
rules/adversary.md (projects), rules/card-grammar.md (Help cards, Teammate triggers).*

---

## 23. Reading a card: the icon legend

A card reads like a sentence. A few **header slots** sit at the top, and small icons for pieces, resources and tags sit
inside ordinary text, as on a Spirit Island power card. Words do the rest, and the text always says what the icons
say, in the same order.

### Where things sit

- **Top strip:** the credit cost (an octagon), the insight cost (a hexagon), requirement plates and the card's tags.
- **Above the art** (buildings only): what lasts while the building is uncovered: income, Effects, Actions.
- **Below the art:** what happens once, when you play the card: extra costs first, then gains, placements and moves.

### The icons

The shape says the kind of thing; the picture inside says which. (The art is not final.)

| Kind | Icons |
|---|---|
| Resources (square tiles) | credits (a coin), energy (a lightning bolt), insight (an eye), chips (a chip), antimatter (an atom), research cubes (stacked cubes), cards (two cards); the adversary's compute (a grid) |
| Tokens on cards (discs) | Datasets (a database), Followers (two people), Capital (a bank), Operatives (a target) |
| Pieces (bare silhouettes) | each piece has one symbol of its own, usable in a sentence: **ambassador** (a figure), **agent** (a figure in a brimmed hat, as on the map), **node** (a house shape), **botnet** (a starburst), **troop** (a winged arrowhead), **drone** (a megacorp's troop: two chevrons, as on the map); also the cube and the Fallout token |
| Places | city (a skyline), research station (a dome with a mast), factory (a sawtooth roof), any site (a hex holding a block: city, station or factory), **Rescue** (a skyline under a flag: liberate it, then claim a station or factory next to it), a hex outline, a country (a cluster of hexes), a megaproject (a card with a cube), an adversary project (a plain card), one of your buildings (a block with windows), a building slot (a dashed outline) |
| Tags (ringed circles) | Science, Industry, Cyber, Power, Diplomacy, Security, each its own symbol; an empty ring is any tag |

### Marks

| Mark | Means |
|---|---|
| Number before an icon | how many (no number: 1) |
| Minus before an icon | pay it or lose it, and only that: every cost you pay prints it, an Action's too |
| Less (a number over a double chevron down) | that much less: a discount, or a jam |
| `≤`N | up to N, your choice |
| `≥`N | at least N (a rebate's printed credit cost) |
| `/` | per |
| OR, ALL, ACT | written out; ACT is an Action granted |
| Brown box above the art | income, each round |
| Brown box with a track arrow | raise your income of that resource for the rest of the game (arrow down would lower it; no card prints one yet); below the art, or after an Effect's `:` |
| **Dark frame** | acts on the adversary: its pieces, projects or threats. Inside it a minus number means "remove up to N" (the adversary loses them) |
| Target box | the header's plate: the target's condition and glyph; the card acts on that hex |
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
| Roof | cover over many pieces, always with a node. On a card's effect line, its Shelter: your nodes and facilities in your reach, this round. On a capstone, standing cover for your pieces: the Aegis Doctrine shelters the first of your nodes or facilities that would be destroyed each round; Sanctuary (after the corrupt mark and a colon) keeps every node from corruption and agents |
| Sway (a marker on a track) | add that much Unity (you may put it against Discord, §12), or Discord on the one card that says so |
| Peek (a card with an eye) | look at the next event cards under the face-up one |
| Threat (`!`) | a threat; struck through: pay it off |
| Jam (a threat with a down arrow) | after the less mark, outside the dark frame: a threat needs that much less of each resource |
| Clock | a delay only: the threat waits a round |
| Bricks | no agent may arrive: after an agent and a hex, a firewall; after an agent and a country, a cordon (not the Protector's Wall, which stops troops) |
| Requirement plates | yellow: at least; stacked cards: built over |

**When an Effect fires.** Every Effect fires once per action, and prints its unit: one on removing troops, agents or
compute fires once for each card, Action, power or standard action that removed any, and one on placing tokens on
your cards once for each placement. A per-tag play trigger ("+1 per Diplomacy tag on it") fires once and pays per
tag. A **rebate** Effect fires when you play a
card whose printed credit cost, before discounts, is at least N.

*Sources: docs/card-guidelines.md (Legend), rules/card-grammar.md (Boxes, Connectors, Shapes);
crates/aic-core/src/face.rs, modules/cards/triggers.rs.*

---

## 24. Glossary

- **Agent:** the adversary's piece. **Ambassador:** a player's piece, any player's.
- **Arrival:** the adversary putting agents on a hex; only that hex is checked for corruption, §6 and §15.
- **At N:** §7, *Tokens on cards*.
- **Base (military base):** where a megacorp keeps its drones for one front, §2 and §11.
- **Blocked:** a site with an opposing piece on it; it makes nothing, §2.
- **Building:** a card. **Facility:** a research station or factory on the map.
- **Compute:** what the adversary spends on projects, §6 and §17.
- **Corrupt number:** how many agents corrupt a site, §6.
- **Corrupted:** owned by the adversary. **Neutral:** owned by nobody.
- **A country you're in:** a country where you have a node. **Your home country:** the one you start in.
- **Discord:** the adversary's track; 7 Discord cards, and an 8th tip loses, §12. **Unity:** your track, §12.
- **Dispatch:** the standard action that puts one of your ambassadors onto a piece or botnet in reach; **Dispatches:** how many
  you have each round, §5 and §8.
- **Draw:** take the top tier-1 card, free.
- **Drone:** a megacorp's piece, waiting at a base and fighting in war, §11.
- **Firewall, cordon:** §16. **Front:** §2.
- **Grow X:** an event line: in each country, the X holding the most agents builds a neutral facility next to it, §6.
- **Hardening token:** one stop on one facility or node, §13.
- **Here:** on this card.
- **In reach:** on or next to one of your nodes.
- **Income step:** production's first part, where every site, megaproject and income pays in one printed order;
  extra drones follow it, §9.
- **Infiltrate X:** an event line: 1 agent onto each site of type X not yet the adversary's, §6.
- **Land:** a city or open hex.
- **Megaproject:** a megacorp's 8-cube card, §8.
- **Muster N:** N adversary troops land in each country, §6.
- **Near (a threat):** your node or ambassador on or next to one of its tokens. **Nearest:** the player whose node is
  closest, §10.
- **Next to:** sharing an edge. **On or next to:** that hex, or one sharing an edge with it.
- **On your cards:** on your programs, your buildings (covered ones too), your board and your track spaces.
- **Player order:** Builder, Protector, Researcher, second Builder, §3.
- **Purge:** the win, §14.
- **Remove up to N:** as many as there are, up to N.
- **Seed:** a botnet's 1 agent each Income beat onto the site next to it holding the most agents, §6.
- **Shelter:** none of your nodes or facilities in your reach can be destroyed until cleanup, §13 (a card's Shelter
  takes no target).
- **Site:** a city, research station or factory.
- **Spill X into Y:** an event line (level 2+): each X holding 2 or more agents moves 1 onto the Y next to it
  holding the most, §6.
- **Stage:** I, II or III, printed on each event card, §6.
- **Strength:** Unity and Discord cards resolve at strength I, II or III (by tip count), §12.
- **Surge N: X:** a project line: N agents onto each country's X holding the most agents, §6.
- **Sway:** add Unity or Discord to its track. **Tip:** §12.
- **Target:** the one hex in your reach a card that touches the map acts on, meeting its target box, §2. An
  adversary troop's targets are players' nodes and facilities, §6.
- **Tier:** player cards and threats come in tier 1 and tier 2 (decks).
- **Troop:** always the adversary's; a megacorp's piece is a drone.

*Sources: rules/README.md (Terms), docs/player-aid.md.*
