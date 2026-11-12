# AI Complete: Rulebook (third draft)

*Rules revision 84. This book teaches the game as the simulator plays it today. Every number is a candidate. The AI
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
**nodes** over the map, send your **ambassadors** to claim places and free them, and build a tableau of cards that pays
for it all.

Everyone wins or loses together.

- **You win** the moment the map is **purged**: the adversary holds none of what the purge asks for. At first that is
  every corrupted city, botnet, adversary troop and agent. As the world swings toward **Unity**, it asks for less; in
  the end, only the corrupted cities (§14).
- **You lose** if any of these happens:
  - the Discord deck is empty when Discord needs a card. **The Discord deck is the game's clock**: 6 cards, never
    reshuffled (§12);
  - any player has no nodes left on the map;
  - the adversary's event deck runs out: 11 rounds at most (9 at the hardest levels).

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
| **Neutral** | none | its energy may be bought (Buy energy, §8) | 1 neutral cube onto its megaproject | nothing |
| **A player's** | that player's | 1 energy to its owner | 2 cubes in its owner's color | 1 chip to its owner |
| **Corrupted** (the adversary's) | adversary | 1 Discord | 1 compute for the adversary | 1 adversary troop on its hex |

Sites make this in production (§9), except a neutral city's energy (bought, §8) and a corrupted station's compute
(the adversary's turn, §6).

**Blocked.** A site with an **opposing piece** on it makes nothing. An agent blocks a neutral or player site. A
player's ambassador blocks a corrupted site, so a corrupted city with your ambassador on it makes no Discord. Nodes
never block, and a node doesn't own the hex it is on.

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
| **Corruption marker** | — | left where a node was destroyed other than by corruption; no node can be placed there |
| **Land corruption** | the adversary | a corrupted open hex or node hex; nothing is built or placed on it (§6) |
| **Hardening token** | — | saves one facility or node once (§13) |
| **Location token** | — | marks where a threat sits (§10) |
| **Firewall** | — | no agent lands on its hex (§16) |
| **No-Rally, no-Muster, no-Corrupt** | — | that adversary step skips its hex (§12) |

### Reach

A hex is **in your reach** if it is on or next to one of your nodes. Unless a card says otherwise ("in one country"),
everything you target on the map must be in your reach. Your ambassadors don't give you reach.

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
     start **neutral**: every city starts with a neutral factory and a neutral station next to it;
   - its megaproject card goes beside the map, empty.
   No agents or troops start on the map.
3. **The adversary:**
   - its agent supply is 24 per country;
   - build the **event deck**: deal the number of stage I, II and III cards your level asks for (§21; level 0:
     4, 4 and 3), shuffle each stage on its own, and stack them with stage I on top. Turn the top card face up;
   - put one round's **agent income** into each country's **pool** beside the map (1 a country at level 0);
   - shuffle the project deck and fill the adversary board's project slots (§17).
4. **Shuffle** the Unity deck, the Discord deck, the two threat decks and the two player decks. The Unity and Discord
   tracks start empty.
5. **Players** take AI boards in this order, which is the **player order**: Builder, Protector, Researcher, then a
   second Builder. Sit in player order: cards pass around the table by it (§7). Player 1 plays in country 1, and so
   on. With 2 players there is no Researcher; solo, only the Builder plays. Each player:
   - places the first node from their **economy track** on their country's starting hex. This uncovers the track's
     first space, which shows nothing (§5);
   - takes 1 ambassador, 3 energy and their board's starting credits;
   - puts their cube on their **home city**: their country's neutral city nearest that first node (tie: the lower hex
     number). It is now theirs like any other city, and can be lost the same way.

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
| | **Round start** | Set your insight (§5). Refresh your powers, Actions, antimatter action and Dispatches. |
| **A** | **Sway** | Each land-corruption token adds 1 Discord (§12). |
| **B** | **The adversary's turn** (§6) | Four beats: **Card** (threats, agents, troops), **Corrupt**, **March**, **Income** (the next card, agents, compute). |
| **C** | **Cards** (§7) | Build a hand, draft, keep. Round 1 first deals the opening hand. |
| **D** | **Action phase** (§8) | All players at once: play cards, use powers and Actions, place nodes, dispatch ambassadors, pay threats. |
| **E** | **Production** (§9) | Everything pays (the income step), then growth: extra drones, and cities gather chips and build. |
| **F** | **Threats** (§10) | Every threat not paid off takes effect. |
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
rule**, your **income**, one **tag** (§7), your **building slots**, your **Dispatches**, two **node tracks** and eight
**powers**.

| Board | Special rule | Income each round | Insight a round | Slots | Tag | Dispatches | Starting credits |
|---|---|---|---|---|---|---|---|
| Builder | *Breadth*: 5 slots | 15 credits, 1 energy | 6 | 5 | Industry | 1 | 40 |
| Protector | *Wall* and *Vigil* | 13 credits, 2 energy | 6 | 4 | Security | 1 | 46 |
| Researcher | *Patience* | 11 credits, 1 energy | 6 | 3 | Science | 2 | 40 |

- **Patience** (Researcher): all your unspent insight carries over to the next round, including insight another
  player gave you.
- **Wall** (Protector): adversary troops can't move onto, land on or strike a hex with your node (a troop that would
  land there stays in the supply), so a facility on that hex is safe from them; one next to it is not. Only troops are stopped: corruption, war and threats still destroy what is there.
  (Production isn't a landing: an adversary factory on that hex still makes its troop there, which marches off.)
  The Militarist answers it: each turn it Musters 1 in each country where you have a node (§6, its board).
- **Vigil** (Protector): the first two times each round an adversary troop ends its march next to one of your nodes,
  gain 1 insight; later troops that round pay nothing. A troop that strikes doesn't count. It pays in any march: the
  adversary's turn's (spend it this round) or the one a threat starts at once (that comes after the action phase, so
  its insight is lost).

### Resources

Nothing has a holding limit, and nothing caps your income.

| Resource | Kept between rounds? | Mainly from | Mainly spent on |
|---|---|---|---|
| **Credits** | yes | income, cards, track spaces, megaprojects | card costs, standard actions, threats, a neutral city's energy |
| **Energy** | yes | income, your cities, neutral cities (bought), cards | placing nodes, powers, cards, building Actions, threats |
| **Insight** | **no** | set each round (below) | keeping and playing cards, threats |
| **Chips** | yes | your factories, cards, Builder powers | buildings, research stations, Procure, threats |
| **Antimatter** | yes | the antimatter action, cards, Horizon's converter | tier-2 cards |
| **Research cubes** | yes | your stations, once their megaproject is complete (§9) | 1 insight each, converters, Peer Review, threats |
| **Ambassadors** | yes (your supply) | 1 at the start, cards, 20 credits | claiming, liberating, Remove agents, Envoy, threats |
| **Dispatches** | **no** | your board and track spaces, each round | the Dispatch action, 1 each |

**Insight is this round's attention.** Your card-play track prints it, as in Spirit Island: 6 at its start, and on
later spaces the new total (7, 9, ...). At the start of each round, set your insight to the number on the rightmost
space you've uncovered (6 before any), plus what your cards add. Insight you don't spend is lost (except under Patience),
and no threat or project takes it: it is spent the turn it comes. Everything else on your
board and cards pays at the end of production's **income step** (§9).

### Node tracks

Your nodes start on your board, on two tracks of 7 spaces: the **economy track** and the **card-play track**. To place a
node, take the next one off either track. That uncovers the space under it, and what the space shows is yours for the
rest of the game: income, a new insight total (card-play track only), Dispatches, a tag, a free research station, the
**keep upgrade** (§7), and on the last space a **capstone**, a lasting rule (§20). The card-play track does not limit how many cards you play.

**Every placement pays the energy printed on the space** you uncover, whatever places the node; a power or card
that places one costs only its own tag or currency on top (Expand: 1 energy):

| Space | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| Economy track: energy to pay | free | free | free | pay 1 | pay 2 | pay 3 | pay 5 |
| Card-play track: energy to pay | free | free | pay 1 | pay 2 | pay 2 | pay 3 | pay 5 |

> **Example.** The Builder has placed 3 economy nodes (the setup node included) and none from the card-play track.
> Expanding onto the economy track's fourth space costs 2 energy (Expand's 1 and the space's 1); onto the card-play
> track's first space, just Expand's 1.

**Where:** on a land hex in your reach with no node of any player, no corruption marker and no land corruption. A hex
with agents or a corrupted site is allowed (a node doesn't remove agents).

**A destroyed node leaves the game.** It does not go back to its track; the space stays uncovered.

### Powers

Each power can be used once a round, in the action phase. Every board has two **node powers**: **Expand** (place a
node for 1 energy, plus the space's) and a dearer second one (§20). So powers place up to 2 nodes a round; cards can place more. The other
six, printed on your board, are conversions, draws and **sacrifice powers**, paid by discarding a card with a tag
(§7, *Tags*). §20 lists every board's.

*Sources: rules/ai-boards.md, rules/ai-powers.md, rules/economy.md (resource table); content/ais.json;
crates/aic-core/src/modules/ai.rs, modules/economy.rs, modules/board/node.rs, modules/board/troops.rs (Vigil),
modules/adversary/ (`wall_pressure`), modules/cards/sacrifice.rs.*

---

## 6. The adversary's turn

The adversary has a **board**, an **event deck**, a **project deck**, a supply of agents, troops and botnets, and a
**pool** of agents beside the map for each country. Each event card shows its **stage** (I, II or III): the stage sets
which threats it deals (§10).

The adversary in this book is **the Militarist**. Its board has three project slots and four rules:

- **Garrisons:** +1 agent income in each country. (The agent income in §21's table already includes it.)
- **Troops where the Protector stands:** each round, Muster 1 in each country where the Protector has a node (the
  Card beat).
- **Warmongering:** in production's growth (§9), each military base with an adversary troop on or next to it gains 1
  extra drone, even past its limit (§19).
- **War Takes Ground:** when a drone destroys a factory in war (§19), it also destroys a node on that hex (§13
  applies to each).

### The turn: four beats

The adversary's board prints its turn as a strip of four beats, played in order.

**1. Card.** The face-up event card becomes this round's card (if there is none, you lose). Then, in order:

- **Threats.** Its first line deals threats (§10).
- **Rally X.** Every event card has one. In each country, deal the whole pool **one agent at a time onto each site of
  type X** (cities, coastal cities, research stations, factories or hexes with a botnet), whoever owns it, in
  hex-number order, going round again until the pool is empty. A firewalled site (§16) or one with a no-Rally token
  (§12) still takes its turn in the deal, but the agent dealt to it goes back to the supply instead.
- **Muster N.** N adversary troops land in each country on its lowest-numbered factory (any owner's); with none, on
  its lowest-numbered city.
- **Other lines**, at most two, each one of seven kinds with its own icon: an income raise, a new neutral factory in
  each country, Discord, Unity taken back, a blackout, node loss by a front, a forced march.
- **Troops where the Protector stands** (the board): Muster 1 in each country where the Protector has a node.

**2. Corrupt.** Each Corrupt line on the card, in order; then natural corruption (both below).

**3. March.** Adversary troops march (below).

**4. Income.** Turn the next event card face up. Put one round of agent income into each country's pool. Then all the
adversary's compute arrives: its income, each botnet and corrupted station without a player's ambassador on it, and
what was made since the last turn, onto the projects (§17). Then ongoing projects act. Firewalls, cordons (§16) and
no-Rally, no-Muster and no-Corrupt tokens (§12) come off.

**Every Muster** (an event card's, a project's, the Protector's) is raised by the level surcharge (§21). A troop that
would land on a hex with the Protector's node (Wall, §5) or a no-Muster token stays in the supply.

Whenever an agent lands where players' ambassadors stand, it cancels one of them, one for one (§8). A troop landing
does nothing to them until it marches (below).

### Corruption

A place's **corrupt number** is how many agents it takes: city **4**, station **3**, factory **3** (each 1 fewer from
level 1, §21), open land **3**, and a hex with a node **2** (whatever else is there: a city with your node loses the
node at 2 agents, and corrupts only at its own number; a Corrupt open land line counts open land's 3 even
where a node stands). Only agents count. A place can corrupt two ways, both in the Corrupt beat:

- **A Corrupt line** (first, when the card prints one): "Corrupt X" names a type (cities, coastal cities, stations,
  factories, open land or nodes). Its **targets** are the places of that type the adversary doesn't own. **Every agent
  next to a target moves onto it** (the players choose if it is next to several; agents already on a target stay);
  then **every target holding its corrupt number corrupts**, and its agents go back to the supply.
- **Natural corruption** (then, every round): every city, station and factory not already the adversary's that holds
  its corrupt number **on its own hex** corrupts; then every hex with a player's node and **2 agents**.

Nothing on a firewalled hex (§16) or one with a no-Corrupt token (§12) corrupts, either way.

> **Example: Corrupt cities.** A neutral city on hex 6 holds 2 agents. Hex 2, next to it, holds 1; hex 7, also next
> to it, holds 2. All 3 move onto hex 6, which now holds 5, at least 4: it corrupts, and the 5 agents go back to the
> supply. Had hex 7 also been next to another city, the players could have sent its 2 agents there instead, leaving
> hex 6 with 3: safe this round.

Some Discord cards and projects also print a Corrupt line, which works the same way, and some threats and projects
corrupt the site they name. Corruption never spreads by itself.

**Whatever corrupts takes one adversary mark:**

- **A site** (city, station, factory) takes the adversary's cube, replacing any owner's cube (a corruption marker
  there comes off). Any node on that hex is destroyed (unless something protects it, §13), then every ambassador on
  it is removed.
- **Open land and a node hex** take a **land corruption** token. On open land any node is destroyed and every
  ambassador removed, as for a site; on a node hex (2 agents) only the node is lost (§13 applies), and the token goes
  on **face down** whether or not the node is saved. Nothing can be built or placed on land corruption (no node, factory or research station). A face-up
  token (open land's) adds 1 Discord every round until a player Clears it (§8); a face-down one adds none.

§15 has every detail.

### The march

An adversary troop's **targets** are player-owned research stations and factories, never anything else. In the March
beat, each adversary troop takes **2 steps**, one troop at a time (country by country, in hex-number order). Each step:

- troops **ignore ambassadors**: they neither stop for them nor remove them (agents deal with ambassadors; troops hit
  facilities). Your own ambassador on your facility still protects it from a strike (§13);
- if a target is **on or next to** the troop's hex, the troop **strikes** it (the players choose which, if several):
  the target is destroyed unless something protects it (§13). Either way the troop is used up and leaves the map;
- otherwise the troop **moves** one hex toward the nearest target, by land and around Wall hexes, crossing borders if
  it must (the players choose among equally good hexes). With no way to any target, it stays.

So a troop two hexes from your facility will hit it this turn (or another target in range, if the players pick
that one), and a troop mustered onto your factory strikes it (or another target in range) in the same turn's march,
unless ambassadors standing there use up its steps first, one each.

> **Example: a march.** An adversary troop stands two hexes from the Builder's factory, and no other player facility
> is nearer. Step 1: no target on or next to it, so it moves one hex toward the factory. Step 2: the factory is next to
> it, so it strikes. The Builder chooses to spend the hardening token on the factory: the token is removed instead,
> and the factory stands. The adversary troop is used up either way. The Builder could instead have spent their own
> ambassador on the factory, or nothing (§13); a drone at a nearby base never helps.

### Projects and botnets

The adversary spends **compute** on **projects** in the Income beat, always on the **leftmost**; a project that fills
**resolves** with its printed effect, and the rest slide left (§17). **Drain** (§8), some cards and the Researcher's Audit take compute off a project.

A **botnet** stays until a card or the **Take down a botnet** action (§8) removes it. It makes 1 compute a round
unless a player's ambassador stands on it, and counts against the purge until Unity has tipped 6 times (§14).
Removing one adds 1 Unity.

*Sources: rules/adversary.md (Each round, Income, Rally X, Corrupt X, Botnets), rules/war.md (the march);
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
deck. There is no hand limit.

### Round 1: the opening

Before round 1's draft, each player is dealt **2 tier-1 cards** and **2 tier-2 cards** straight into their hand, kept
**free**. The tier-2 cards point you toward a build to plan for; you need not play both. Then round 1 builds a hand,
drafts and keeps like every round.

### Every round: build, draft, keep

1. **Build a hand.** Each player takes the top card of the tier-1 or the tier-2 deck, your choice each time, one at a
   time until they have 4. Each arrives face up, so you see it before choosing the next. (This is not a draw.)
2. **Draft.** Take one card from the hand in front of you and pass the rest on, until the hands are gone. Always
   pass to the next player in player order. Solo, you take all 4, then keep those you want, paying as below.
3. **Keep.** Pay **1 insight** for each tier-1 card and **2 insight** for each tier-2 card you keep (1 with the keep
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
except on an event.

**Discounts** (a discount never takes a cost below 1; a cost printed 0 stays 0):

- every 2 Science tags on your cards take 1 insight off each card you play, and off each tier-2 card you keep;
- every 2 Industry tags take 1 credit off each building;
- some cards make all your cards of one tag cheaper.

**Play a card if any part of it can act.** Parts with nothing to act on (no target in reach, say) are skipped. A
program or event with nothing at all to do can't be played; a building always can. A sway toward Discord is a price,
not a part that acts.

### Tags

The tags **on your cards** are those on your programs, your buildings (covered ones too), your AI board and your
uncovered track spaces. Tags unlock requirements, set discounts and per-tag payouts, and pay for sacrifice powers and
some threats.

**Sacrifice.** A cost that names a tag ("1 Industry") is paid by discarding one of your cards in play that carries
that tag: a program, or a building together with every building stacked under it (a tag on a covered building
counts). Events can't be sacrificed, and neither can your board or its track spaces. Tags on the card beyond the cost
are wasted.

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
- take **standard actions**, as often as you can pay (below);
- use a **megacorp converter** (below);
- take the **antimatter action** once: gain 1 antimatter and add 1 Discord.

### Standard actions

Standard actions are always there; cards do the same things better.

| Action | Cost | Effect |
|---|---|---|
| **Dispatch** | 1 Dispatch | put one of your ambassadors, from your supply or from anywhere on the map, onto a site, base, node or botnet in your reach (not the hex it is on) |
| **Claim** | ambassadors: city 5, station or factory 3, +1 an agent (below) | take a neutral site in reach: it becomes yours |
| **Liberate** | ambassadors, as Claim | take a corrupted site in reach: it becomes yours |
| **Remove agents** | 1 ambassador from your supply, spent | remove up to 3 agents in your reach, from one hex or several; nothing is placed |
| **Take down a botnet** | 5 credits | remove a botnet in your reach where your ambassador stands (the ambassador stays); add 1 Unity |
| **Buy energy** | 4 credits | gain the energy (1) of a neutral city in your reach that isn't blocked or blacked out; each city once a round, for all players together |
| **Build a research station** | 1 chip | on an open hex in reach that isn't an industrial zone or corrupted land; it is yours |
| **Clear** | 2 credits | remove a corruption marker in reach and add 2 Unity; or remove a land corruption token in reach (no reward) |
| **Envoy** | 1 ambassador from your supply | remove 1 Discord from the Discord track |
| **Gain an ambassador** | 20 credits | into your supply |
| **Harden** | 3 credits | a hardening token on a player's station or factory in reach (§13) |
| **Drain** | 5 credits | remove up to 3 compute from one adversary project |
| **Research cube** | 1 research cube | gain 1 insight |
| **Sell** | a card from your hand | gain 1 credit |

### Ambassadors meet agents

**Every contact is one for one.** When an agent lands on a hex where players' ambassadors stand, it and one of them
are removed (the agent back to the adversary's supply); if several players' ambassadors are there, the players choose
whose. When your ambassador arrives on a hex with agents, it removes one of them and is removed itself. So an agent
and an ambassador never share a hex.

**Remove agents** removes its agents one at a time, your choice, and you may stop early.

### Claim and Liberate

**Claim** takes a **neutral** site; **Liberate** takes a **corrupted** one. Otherwise they work the same way:

- **Where:** the site must be in your reach.
- **Price, in ambassadors:** city **5**, station or factory **3**, plus 1 for each agent on it. Those agents are
  removed. (Agents and your ambassadors never share a hex, so a site with agents is paid from your supply.)
- **Who pays:** your own ambassadors standing on the site pay first, then your supply. Another player's never pay.
- **Result:** your cube goes on: the site is **yours**. Liberate first takes the adversary's cube off, and adds
  Unity: city **+5**, station or factory **+2**. A botnet on the site stays.

> **Example: block, then liberate.** A corrupted city in your reach makes 1 Discord a round. You Dispatch an
> ambassador onto it (1 Dispatch): now it is blocked and makes nothing. A round later you Liberate it: the price is 5 (no agents
> on it), your standing ambassador pays 1 and your supply pays 4. The adversary's cube comes off, yours goes on, and
> you add 5 Unity. The city is yours: from this round's production it makes you 1 energy a round.

**Liberate cards** (Popular Uprising and some tier-2 cards) liberate a corrupted city in reach as Liberate does, for the card's own price
instead of ambassadors.

A card that lets you **claim** a neutral research station pays its extra (one ambassador per agent on it) as Claim
does: your ambassadors on the station first, then your supply.

### Megaprojects and converters

Each megacorp's **megaproject** needs 8 cubes, which research stations in its country put on it (§9). When it
completes, each cube on it pays its owner the payout below, and its **converter** opens. Any player with a **city of
that country in reach** (any city, corrupted ones too) may use the converter, **once a round** each.

| Megacorp (megaproject) | Payout, each of your cubes | Converter |
|---|---|---|
| Horizon (Tidal Grid) | 3 energy, 3 credits | 1 research cube + 2 energy → 1 antimatter |
| Cintamani (Polar Compute Array) | draw 1 card, 5 credits | 1 research cube → 2 insight |
| Onyx (Mountain Fusion Works) | 2 chips, 3 credits | 1 research cube → 1 chip |
| Biomia (Delta Seawall) | 1 Unity, draw 1 card | 2 research cubes → 1 Unity |

*Sources: rules/economy.md (standard actions, Cities' energy), rules/ambassadors.md (Claiming, Liberating, Every
contact, Remove agents), rules/board.md (Converters), rules/alignment.md (The antimatter action);
crates/aic-core/src/modules/board/standard.rs, modules/board/ambassadors.rs, modules/board/mod.rs
(converter_blocked), modules/alignment.rs; content/map.json (`once`).*

---

## 9. Production

Production has two parts: the **income step**, where everything pays, then **growth**. Do each step for the whole
map before the next.

**The income step**, in this order:

1. **Factories.** A player's factory makes 1 chip for its owner. A corrupted factory makes 1 adversary troop on its
   own hex. A neutral factory, or a blocked one, makes nothing.
2. **Cities.** Each unblocked city makes 1 energy: a player's city for its owner; a corrupted city 1 Discord instead.
   A neutral city's energy is bought in the action phase (Buy energy, §8), not here.
3. **Stations.** A player's station makes 2 cubes in its owner's color; a neutral one makes 1 neutral cube. They go
   onto their country's megaproject. Once it is complete, a player's cubes go onto their board as **research cubes**,
   and a neutral station's are lost. A corrupted station makes 1 **adversary cube** onto its country's megaproject
   until it is complete; after that it makes compute instead, in the adversary's compute step (§6). A player's
   ambassador on a corrupted station blocks either.
4. **Megaprojects.** A megaproject holding 8 cubes or more **completes**, once: every cube on it pays, overflow
   included (§8), then its cubes come off. An adversary cube pays the card's adversary payout (1 Sway to Discord;
   Polar Compute Array, 1 adversary compute). Neutral cubes pay nothing. From then on its stations' cubes go to their
   owners' boards (step 3).
5. **Income.** Each player collects their income (insight aside, §5): board, track spaces, programs and buildings.

**Growth:**

6. **Extra drones.** Warmongering and some projects add drones to bases (§19).
7. **Cities grow.** Every city, a player's, neutral or corrupted, puts **1 chip on itself for each unclaimed neutral
   factory next to it** (a city makes none of its own; a player's or the adversary's factory feeds nothing; an agent on
   the city stops nothing). A city holds at most 3. A city with no unclaimed factory beside it does nothing.
8. **Cities build.** Then, country by country in hex-number order, each city holding **3 chips** spends them and builds
   a **neutral** facility (anyone may claim it, §8): whichever of factory and research station it has **fewer of next
   to it**, counting anyone's (a tie: a factory). A factory goes on a **free** industrial zone (no factory, station or
   land corruption; a node doesn't count), a station on free open land that is no industrial zone and no base, **next
   to the city**; with none, on one within 2 hexes **in the city's own country**; with none there, nothing is built
   (it never builds the other kind instead) and its chips wait. A player's city: its owner picks the hex. A neutral or
   corrupted city: the lowest-numbered (a corrupted city's on or next to an agent first). A destroyed factory leaves
   the map and frees its zone. So claiming a neutral factory stops it growing its cities.

*Sources: rules/war.md (Every round, Factories), rules/economy.md (Cities' energy, The round), rules/board.md
(Research cubes and the corps); configs/baseline.json `round` (`board.produce`, `economy.upkeep`, `board.bases`);
crates/aic-core/src/modules/board/production.rs (produce, produce_units, grow_cities), modules/economy.rs.*

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
  closest to one of its tokens, at any distance, near or not (tie: the tied players choose which of them takes it).

**Paying it off.** A threat prints one or two ways to pay it off. **Pay one way in full** and it is discarded.

- **Resources** (credits, energy, insight, research cubes, chips, or ambassadors from your supply) may be pooled by all
  the players near it. You may take back what you put on it until you pass. Anything left on a threat that isn't paid
  off goes back to its payers when threats take effect.
- **Tags:** one player alone sacrifices cards in play carrying 2 of the threat's tag in all (tier 1) or 3 (tier 2).

Cards can also **jam** a threat (it needs less of each resource), **delay** it (it waits a round, and can be paid next
round), or pay it off outright. A few buildings pay off part of every threat of a type, wherever it is.

**When threats take effect** (step F), each threat not paid off does what it prints, country by country, then is
discarded with its tokens. Pieces it places go on its token. A resource harm reads "pay N or discard M cards": the
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
and some projects (§19). Drones come:

- **each time the Discord track tips** (§12): each country with a neutral factory that has no agent and no ambassador
  on it puts 1 drone on each of its bases, however many such factories it has;
- from Warmongering and some projects, in production's growth (§9, §19).

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

- **Unity** comes from liberating sites (city 5, station or factory 2), clearing markers (2), removing botnets (1),
  Biomia, and cards.
- **Discord** comes from corrupted cities (1 each a round), face-up land corruption (1 each a round), factories destroyed in war
  (1 each), the antimatter action (1), and threats, projects and cards that print it.

**Unity against Discord.** Whenever you gain Unity, you may put any of it against Discord instead: each Unity put
there removes 1 Discord from the Discord track. You decide as the action or step that gained it resolves, before
any track tips. The rest goes onto the Unity track. (**Envoy** also removes 1 Discord, for 1 ambassador.)

> **Example.** 3 players, so a track tips at 9. Discord is at 7. One step adds 3 Discord (10) and 2 Unity. Putting
> both Unity against Discord leaves 8: Discord doesn't tip. Had the step added 4 Discord (11), the same 2 would leave
> 9: Discord still tips, but to 0 instead of 2.

### Tipping

A track **tips** when it holds **3 × the number of players**. Then:

1. Take one tip's worth off. The rest stays on the track toward its next tip, on both tracks.
2. Add 1 to that side's tip count.
3. **Discord only:** neutral factories arm the bases (§11).
4. Draw that side's top card and resolve it at the strength for its tip count, this tip included: **strength I** on
   tips 1-2, **strength II** on tips 3-5, **strength III** from tip 6.

If both tracks are full, Discord tips first. A track still full after a tip (what stayed, or a card that fills
it) tips again at once.

### The cards

Every Unity and Discord card prints three strengths. Each acts across the map by a condition ("for each site a player
owns"), not in one player's reach. Some Unity cards **forbid** the adversary's Rally, Muster or Corrupt: put a no-Rally,
no-Muster or no-Corrupt token on every hex where the card's condition holds as it resolves. That step skips a hex with
its token, and what it would have placed there stays in the supply; a no-Corrupt token stops every corruption there
and every agent moving onto it. The tokens come off at the end of the next adversary turn, like a firewall.

- **The Unity deck** (23 cards) reshuffles when it runs out.
- **The Discord deck** (6 cards) **never reshuffles.** A Discord tip with no card left loses the game: you survive 6
  Discord tips and lose on the 7th, at every level (§21).

*Sources: rules/alignment.md; configs/baseline.json (`tip_per_player` 3, `carry` 2, `hold_off` 1, `tier_at` 3, 6); content/alignment.json;
crates/aic-core/src/modules/alignment.rs, modules/board/production.rs (the tip's drones).*

---

## 13. Protecting your pieces

### What can be destroyed

Your **nodes** and your **facilities** (stations and factories) can be destroyed:

| By | What it destroys |
|---|---|
| **Corruption** (a Corrupt line, natural corruption, a threat that corrupts a site) | the node on the corrupted hex, then every ambassador there |
| **A node hex's corruption** (2 agents on a hex with your node, §6) | that node; the agents go to the supply |
| **An adversary troop's strike** | the player-owned station or factory it strikes (never a node) |
| **War** | the factory a drone strikes, then a node on its hex (War Takes Ground) |
| **Threats and event lines** | what they print ("destroy a node", Border War's node near a base) |

A destroyed facility takes its owner's cube with it. A destroyed node leaves the game. Corruption leaves its own mark
(§6: the adversary's cube, or land corruption); a node lost any other way leaves a **corruption marker** on its hex,
unless the hex already holds a marker, a botnet or an adversary cube. A marker only stops a node being placed there;
it never counts as an agent. **Clear** (§8) removes one.

### The protection order

When a node or facility would be destroyed:

1. **Shelter** on that country this round (the Protector's power, some cards) stops it, automatically, spending
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
node can save either, once: you may keep it for the second (it stays on the hex while your node does). Corruption never destroys
the node of a player with **Sanctuary** (a Protector capstone), and spends nothing of theirs.

**Hardening** goes on a player-owned station or factory (Harden, §8; the Protector's Bulwark).

### Your last node

Nothing protects a last node but the protection order above: anything that destroys a node can destroy your last
one, at any stage. Any player with no nodes loses the game for everyone, so keep a second node on the board: the
first spaces of both tracks cost no energy, so an early second node by Expand costs just its 1 energy. Watch the hexes next to a
lone node: two agents on it take it, and a Corrupt nodes line pulls in every agent next to it.

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
| 3-5 | corrupted cities, botnets, adversary troops |
| 6 or more | corrupted cities |

Agents waiting in the pools are not on the map. Corrupted stations, factories and land never count, nor do
drones.

### Losing

Everyone loses at once if:

- a Discord tip finds the Discord deck empty;
- any player has no nodes on the map;
- the Card beat finds no card to turn up (the **round cap**: 11 rounds at levels 0-2, 9 at levels 3-4). Round 12
  (10 at levels 3-4) still opens with its Sway step, so a Discord tip can lose the game there first; then, with no
  card left, you lose.

The adversary running out of agents is not a loss: a placement it can't make doesn't happen.

*Sources: rules/alignment.md (The purge, Losing), rules/adversary.md (The event deck); configs/baseline.json
(`purge_at`); crates/aic-core/src/modules/alignment.rs, modules/adversary/.*

---

# Part 2: Reference

## 15. Corruption in full

**Corrupt X** names a type: cities, coastal cities, research stations, factories, open land or nodes. A **target** is
a place of that type the adversary doesn't own ("open land": an open hex with no base, station, factory or land
corruption, a node on it or not; "nodes": a hex with any player's node).

1. **Move.** Every agent next to a target moves onto it, across borders too, even off one of the adversary's own
   sites. An agent next to several targets goes onto one the players choose; agents already on a target stay. An agent moving onto players' ambassadors cancels
   one of them (§8).
2. **Corrupt.** Every target holding its corrupt number of agents corrupts (§6), and its agents go back to the supply.
   A node target loses its node (§13 applies) and, if it goes, takes a land corruption token; nothing else on its
   hex changes.

A card may print "(1 fewer)": the corrupt number is 1 lower for that line. No agent moves onto a firewalled target or
one with a no-Corrupt token, and it doesn't corrupt (agents already on it stay).

**Natural corruption** (after the lines) checks each city, station and factory that isn't the adversary's: with its
corrupt number of agents on its own hex, it corrupts. Then each hex with a player's node and **2 agents** loses that
node as a node target does; the site there, if any, is untouched. Open land never corrupts this way. Nothing on a
firewalled hex or one with a no-Corrupt token corrupts, a node included.

> **Example: the players choose.** A neutral city on hex 6 holds 2 agents. Hex 2, next to it and to no other city, holds
> 1; hex 7 holds 2 and is next to hex 6 and to the city on hex 11, which holds 1. Hex 2's agent moves onto hex 6. The
> players send both of hex 7's agents to hex 11 (they choose): hex 6 holds 3 and hex 11 holds 3, short of 4, so
> neither corrupts this time. Had they sent one each way, hex 6 would hold 4 and corrupt.

**What corruption destroys:** the node on the hex (§13), then every ambassador on it. Nothing else.

*Sources: rules/adversary.md (Corrupt X); crates/aic-core/src/modules/plan.rs (converge, corrupt_plan,
natural_targets), modules/adversary/ (corrupt_sites, corrupt_node).*

---

## 16. Moving and removing pieces

Some cards and the Protector's Patrol move pieces on the map: ambassadors (any player's), agents and adversary troops.
Botnets and markers never move.

- **Gather up to N:** pick a land hex in your reach; move pieces into it, each from a hex next to it.
- **Push up to N:** pick a land hex in your reach; move pieces off it, each to a land hex next to it, your choice.

You may stop after any piece. Moved pieces arrive as usual: an agent moved onto players' ambassadors cancels one of
them (§8).
No agent enters a firewall; no adversary troop enters the Protector's node hex.

**Remove around a hex.** Some cards (Kettle Operation, Lure Network, Honeypot Relay) remove up to N agents, or
agents and troops, **on or next to** a hex you pick in reach: the hex itself counts, and you choose which pieces, a
batch at a time, and may stop early. Nothing moves.

**Convert.** Some late cards turn agents into your ambassadors: up to N agents on or next to a city in your reach
each become your ambassador where it stood. The converted agents come off first, then your ambassadors go on, so two
converted on one hex both stay; an agent the card left on that hex meets one of them, one for one (§8).

**Firewall.** Some cards put a firewall on a land hex in your reach. While it stands, no agent lands, is placed or
moves in there: an agent Rally deals to it goes back to the supply. Nothing on it corrupts: no Corrupt line, natural corruption, station flip or threat
that corrupts its site takes it (agents already there stay). Troops pass. It lasts until the end of the next adversary
turn (its Income beat).

**Cordon.** Hardpoint Lockdown cordons a country until the end of the next adversary turn: no agents land
there from a Rally or a card line (troops still land, and Gather and Push still move agents in). That country's pool
waits: it lands at the first Rally after the cordon ends, with the income added meanwhile.

*Sources: rules/ambassadors.md (Moving pieces), rules/adversary.md (Firewalls); crates/aic-core/src/modules/effects/moves.rs,
modules/effects/endgame.rs, modules/effects/removal.rs, modules/adversary/.*

---

## 17. Projects

The adversary board has **slots** (the Militarist: three open slots that take any project). **To deal**, turn up
projects one at a time until every slot is full: each goes into the first empty slot. When the deck runs out, shuffle the
discard back in.

A project prints its cost **for each number of players**. All compute arrives in one step, the **compute step** of
the Income beat: its printed compute income, 1 for each botnet and each corrupted research station without a player's
ambassador on it, and every compute made since the last compute step (threats taking effect, cards and projects that
print it), which waits beside the projects until then. All of it goes onto the **leftmost** project. The moment a
project holds its cost, it **resolves** and leaves its slot:

- a **one-time** project does what it prints and leaves the game;
- an **ongoing** project does its "Now:" line at once, then goes beside the board and acts every round from the next:
  in the Income beat after the compute step, or in production or war if that is what it prints.

**Slide and refill:** when a project leaves (resolved or discarded), the projects to its right slide one slot left,
and the empty rightmost slot is dealt a new project at once; the rest of the compute goes onto the new leftmost.
Compute with no project to go on is lost. No project resolves outside the compute step.

*Sources: rules/adversary.md (The project deck); content/projects.json; crates/aic-core/src/modules/adversary/.*

---

## 18. Threat details

**Dealing.** Stage I event cards deal one tier-1 threat in each country, stage II one tier-2 (Supply Lines: one tier-1
and one tier-2), and stage III two tier-2. Each deal names one or two threat tags, or "any". For each deal, for each
country in turn, turn up the top card of that tier's threat deck. It stays if it carries a named tag (a **Universal**
card has every tag) **and** the country has a place of one of its location types that isn't the adversary's.
Otherwise discard it and turn up the next. When a threat deck runs out, shuffle its discards into a new deck; if a
reshuffled deck has no card that fits, take the next card anyway.

**Location tokens.** In printed order, each token goes on the place of its type that isn't the adversary's with the
most agents on its own hex; tie: the lowest hex number. The same test holds for every type (a base is never the
adversary's). A threat's two tokens never share a hex; a token with no place left gets none.

**Difficulty.** From level 1, resource items on threats cost more (§21); tag items never change.

*Sources: rules/threats.md; crates/aic-core/src/modules/threats/mod.rs, threats/counter.rs.*

---

## 19. Drones in full

- **At a Discord tip,** each country with at least one neutral factory that has no agent and no player's ambassador on
  it puts 1 drone on each of its bases that has room (2 bases at 3-4 countries, 1 at 2 countries): once a country,
  however many such factories.
- **One ambassador always stops one drone**, and is spent. A player's ambassador on a base stops the drone that base
  would get, whatever makes it. A full base takes no drone and spends no ambassador. With several players'
  ambassadors on one base, the earliest in player order is spent first.
- **Warmongering** and the Arms Race project add 1 drone to a base even past its limit (at most 1 extra a base a
  round). The War Footing project fills every base.

*Sources: rules/war.md; crates/aic-core/src/modules/board/production.rs.*

---

## 20. Powers and capstones

Each power can be used once a round, in the action phase. A tag in a cost means sacrifice a card with that tag (§5).

| Board | Node power | Conversions | Draw | Sacrifice and other powers |
|---|---|---|---|---|
| Builder | **Buy In** (6 credits: a node) | **Mint** (3 credits: 1 chip), **Focus** (5 credits: 1 insight) | **Procure** (1 chip: draw 2) | **Scrap** (1 Industry: 1 chip), **Overcharge** (1 Power: 2 energy), **Rush Build** (1 Industry, 1 energy: play a building from your hand without paying its credits; its other costs and requirements stand) |
| Protector | **Accord** (1 Diplomacy: a node) | **Stand Down** (1 Security: 3 credits) | **Debrief** (1 Diplomacy: draw 2) | **Recovery** (1 Security: clear a marker in reach, +2 Unity), **Shelter** (1 Security, 1 energy: in any one country, no node or facility can be destroyed from now to cleanup, §13), **Bulwark** (1 energy: harden a facility in reach), **Patrol** (1 energy: push up to 3 agents or adversary troops from a hex in reach) |
| Researcher | **Peer Review** (3 research cubes: a node) | **Grant** (3 credits: 1 cube in your color on a megaproject that isn't complete), **Synthesis** (discard a card from your hand: 2 insight) | **Fieldwork** (1 energy: draw 3) | **Field Station** (1 Industry: a research station without the chips, where Build a research station could go), **Audit** (1 Cyber: remove 3 compute from a project), **Particle Run** (1 Power, 1 energy: 1 antimatter) |

Every board also has **Expand** (1 energy: a node). Every placement also pays the track space's energy (§5).

| Board | Economy track capstone | Card-play track capstone |
|---|---|---|
| Builder | **Full Capacity**: 1 chip each round per 4 Industry tags on your cards | **Master Plan**: your buildings cost 2 credits less |
| Protector | **Sanctuary**: corruption never destroys your nodes | **Aegis Doctrine**: Shelter on your own pieces once a round: the first time each round one of your nodes or facilities would be destroyed, it isn't (automatic, spending nothing) |
| Researcher | **Open Archive**: 1 research cube each round | **Open Science**: each round in the income step, give 2 insight from the supply in all to other players, split as you like; it arrives with their insight next round |

*Sources: rules/ai-powers.md, rules/ai-boards.md; content/ais.json; crates/aic-core/src/modules/ai.rs.*

---

## 21. Difficulty

Each row is the whole level. Your own costs (claim, liberate, cards, actions) never change.

| Level | Corrupt numbers (city / station / factory / land / node) | Threat items | Muster | Each unpaid threat | Event cards (I / II / III) | Rounds | Compute income | Agent income a country (with Garrisons) | Other |
|---|---|---|---|---|---|---|---|---|---|
| 0 | 4 / 3 / 3 / 3 / 2 | as printed | as printed | — | 4 / 4 / 3 | 11 | 1 | 1 | — |
| 1 | 3 / 2 / 2 / 3 / 2 | credits +4, other resources +1 | as printed | — | 4 / 4 / 3 | 11 | 4 | 3 | — |
| 2 | 3 / 2 / 2 / 3 / 2 | credits +4, other resources +2 | +1 troop | +4 compute | 4 / 4 / 3 | 11 | 19 | 3 | — |
| 3 | 3 / 2 / 2 / 3 / 2 | as level 2 | +1 troop | +4 compute | 4 / 2 / 3 | 9 | 24 | 4 | — |
| 4 | 3 / 2 / 2 / 3 / 2 | as level 2 | +1 troop | +4 compute | 3 / 2 / 4 | 9 | 24 | 4 | the last stage-I card deals tier-2 threats |

- **Threat items:** each resource item on a threat costs that much more; a tag item ("2 Security") never changes.
- **Muster:** every Muster (an event card's, a project's, the Protector's) lands 1 more troop in each country
  it lands in.
- **Each unpaid threat:** a threat that takes effect unpaid also gives the adversary 4 compute, at its next compute
  step (§17). A threat that only waits gives none.

Both incomes climb with the level (never down). Level 4 is level 3 with a stage-I card swapped for a stage-III card
and tier-2 threats a card sooner.

*Sources: rules/adversary.md (Difficulty, The event deck), rules/threats.md (Countering); configs/baseline.json
`adversary` (`compute_income`, `ambassador_income`, `counter_bonus_*`, `counter_more_*`); content/adversary.json
(Garrisons); crates/aic-core/src/modules/adversary/, modules/threats/counter.rs.*

---

## 22. Player count and solo play

- **Countries:** one per player, 1-4. 2 countries share one front; 3 countries have three fronts; 4 form a ring of
  four.
- **Boards:** Builder, Protector, Researcher, then a second Builder. 2 players: no Researcher. Solo: the Builder only.
- **Tipping:** both tracks tip at 3 × the number of players.
- **Projects** print a cost for each number of players (§17).
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
| Pieces (bare silhouettes) | each piece has one symbol of its own, usable in a sentence: **ambassador** (a figure), **agent** (a figure in a brimmed hat, as on the map), **node** (a house shape), **botnet** (a starburst), **troop** (a winged arrowhead), **drone** (a megacorp's troop: two chevrons, as on the map); also the cube and the corruption marker |
| Places | city (a skyline), research station (a dome with a mast), factory (a sawtooth roof), a hex outline, a country (a cluster of hexes), a megaproject (a card with a cube), an adversary project (a plain card), one of your buildings (a block with windows), a building slot (a dashed outline) |
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
| A place after a piece | where: after a cube, only that kind of site; after a botnet, agents or ambassadors, on or next to that place; after a country, in one country |
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
| Roof | cover over many pieces: with a country, Shelter (the whole country, this round); with a node, all your pieces (the Aegis Doctrine shelters the first of your nodes or facilities that would be destroyed each round; Sanctuary, every node from corruption) |
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
- **At N:** §7, *Tokens on cards*.
- **Base (military base):** where a megacorp keeps its drones for one front, §2 and §11.
- **Blocked:** a site with an opposing piece on it; it makes nothing, §2.
- **Building:** a card. **Facility:** a research station or factory on the map.
- **Compute:** what the adversary spends on projects, §6 and §17.
- **Corrupt number:** how many agents corrupt a site, §6.
- **Corrupt X:** an adversary line: agents next to each place of type X move onto it, then each holding its corrupt
  number corrupts, §6 and §15.
- **Corrupted:** owned by the adversary. **Neutral:** owned by nobody.
- **A country you're in:** a country where you have a node. **Your home country:** the one you start in.
- **Discord:** the adversary's track; 6 Discord cards, and a 7th tip loses, §12. **Unity:** your track, §12.
- **Dispatch:** the standard action that puts one of your ambassadors onto a piece or botnet in reach; **Dispatches:** how many
  you have each round, §5 and §8.
- **Draw:** take the top tier-1 card, free.
- **Drone:** a megacorp's piece, waiting at a base and fighting in war, §11.
- **Expand:** every board's first node power, 1 energy plus the space's, §5.
- **Firewall, cordon:** §16. **Front:** §2.
- **Hardening token:** one stop on one facility or node, §13.
- **Here:** on this card.
- **In reach:** on or next to one of your nodes.
- **Income step:** production's first part, where every site, megaproject and income pays in one printed order;
  **growth** follows it, §9.
- **Land:** a city or open hex.
- **Megaproject:** a megacorp's 8-cube card, §8.
- **Muster N:** N adversary troops land in each country, §6.
- **Natural corruption:** a site corrupting on the agents on its own hex, and a node hex losing its node to 2 agents, §6.
- **Near (a threat):** your node or ambassador on or next to one of its tokens. **Nearest:** the player whose node is
  closest, §10.
- **Next to:** sharing an edge. **On or next to:** that hex, or one sharing an edge with it.
- **On your cards:** on your programs, your buildings (covered ones too), your board and your track spaces.
- **Player order:** Builder, Protector, Researcher, second Builder, §3.
- **Pool:** the agents waiting to land in a country, §6.
- **Purge:** the win, §14.
- **Rally X:** an event line: each country's pool is dealt one agent at a time onto its sites of type X, §6.
- **Remove up to N:** as many as there are, up to N.
- **Shelter:** in one country, no node or facility can be destroyed until cleanup, §13.
- **Site:** a city, research station or factory.
- **Stage:** I, II or III, printed on each event card, §6.
- **Strength:** Unity and Discord cards resolve at strength I, II or III (by tip count), §12.
- **Sway:** add Unity or Discord to its track. **Tip:** §12.
- **Target:** a place a Corrupt line can corrupt, §15; an adversary troop's targets are player facilities, §6.
- **Tier:** player cards and threats come in tier 1 and tier 2 (decks).
- **Troop:** always the adversary's; a megacorp's piece is a drone.

*Sources: rules/README.md (Terms), docs/player-aid.md.*
