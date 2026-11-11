# AI Complete: Rulebook (second draft)

*Rules revision 76. This book teaches the game as the simulator plays it today. Every number is a candidate. The AI
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
19. [Corp troops in full](#19-corp-troops-in-full)
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

**You always see what's coming.** Next round's adversary card is face up all round. The agents waiting to land are in
open view.

**Team choices.** Where the rules say "the players choose", decide together. If you can't agree, the first player in
player order (§3) decides.

*Sources: rules/README.md (Terms: players choose), rules/alignment.md (The purge, Losing), rules/adversary.md.*

---

## 2. The map

### Countries and hexes

There is one **country** per player. A country is a square tile of 16 hexes, numbered 1-16 row by row. Each country
also belongs to a **megacorp** (Horizon, Cintamani, Onyx, Biomia), which has its own **megaproject** card and its own
troops. The megacorps are on nobody's side.

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
| **A player's** | that player's | 1 energy to its owner | 2 cubes in its owner's color | 2 chips to its owner |
| **Corrupted** (the adversary's) | adversary | 1 Discord | 1 compute for the adversary | 1 adversary troop on its hex |

That is what each site makes each round in production's income step (§9); a neutral city's energy is bought in the
action phase (§8), and a corrupted station's compute comes in the adversary's compute step (§6).

**Blocked.** A site with an **opposing piece** on it makes nothing. An agent blocks a neutral or player site. A
player's ambassador blocks a corrupted site, so a corrupted city with your ambassador on it makes no Discord. Nodes
never block, and a node doesn't own the hex it is on.

### Fronts and military bases

A **front** is a long edge shared by two countries, marked on the tiles. With 2 countries there is one front; with 3
or 4, each country has two. Each country has a **military base** for each of its fronts, printed on an open hex by the
border. A base is not a site: it is where its megacorp keeps its **corp troops** for that front (§11). Ambassadors may
stand on a base. A one-player game has no fronts and no bases.

### Pieces

| Piece | Whose | In brief |
|---|---|---|
| **Node** | a player | where you act from (one node per hex, of any player) |
| **Ambassador** | a player | claims and frees sites, blocks, guards; an **ambassador** means any player's |
| **Agent** | the adversary | lands on sites, blocks them, corrupts them |
| **Botnet** | the adversary | lasting token: makes compute, counts against the purge |
| **Adversary troop** | the adversary | marches and raids player facilities (§6) |
| **Corp troop** | a megacorp | waits at a base and fights across its front (§11) |
| **Owner cube** | a player or the adversary | who owns a site |
| **Corruption marker** | — | left where a node was destroyed; no node can be placed there |
| **Land corruption** | the adversary | a corrupted open hex (§6) |
| **Firewall, hardening token, location token** | — | §16, §13, §10 |

Some cards hold their own tokens: **Datasets**, **Followers**, **Capital** and **Operatives** (§7). These stay on the
card; they are not pieces on the map.

### Reach

A hex is **in your reach** if it is on or next to one of your nodes. Unless a card says otherwise ("in one country"),
everything you target on the map must be in your reach. Your ambassadors don't give you reach.

*Sources: rules/board.md (Country pieces), rules/ambassadors.md (One site model, Blocking), rules/war.md (Bases),
rules/README.md (Terms); content/map.json; crates/aic-core/src/map.rs, modules/common.rs.*

---

## 3. Setup

**Choose** the number of players (1-4) and a difficulty level (0-4, §21; level 0 to learn).

1. **Lay out the map** for the number of players: one country each, with its military bases.
2. **In each country** (each tile prints these places):
   - its marked city gets an adversary cube: it starts **corrupted**;
   - its marked research station gets an adversary cube and a **botnet**;
   - its other research station, next to the printed **starting hex**, and its two printed factories start
     **neutral**;
   - its megaproject card goes beside the map, empty.
   No agents or troops start on the map.
3. **The adversary.** Its agent supply is 24 per country. Build the **event deck**: deal the number of stage I, II and
   III cards your level asks for (§21), shuffle each stage on its own, and stack them with stage I on top. Turn the
   top card face up. Put the agents for one round's **agent income** into each country's **pool** beside the map (1 a
   country at level 0). Deal projects into the adversary board's slots (§17).
4. **Shuffle** the Unity deck, the Discord deck, the two threat decks and the two player decks. The Unity and Discord
   tracks start empty.
5. **Players** take AI boards in this order, which is the **player order**: Builder, Protector, Researcher, then a
   second Builder. Sit in player order: cards pass around the table by it (§7). Player 1 plays in country 1, and so
   on. With 2 players there is no Researcher; solo, only the Builder plays. Each player:
   - places the first node from their **economy track** on their country's starting hex. This uncovers the track's
     first space (+1 insight each round, §5);
   - takes 1 ambassador, 3 energy and their board's starting credits;
   - puts their cube on their **home city**: their country's neutral city nearest that first node (tie: the lower hex
     number). It is now theirs like any other city, and can be lost the same way.

The game starts with the adversary's turn, before anyone acts.

*Sources: rules/board.md (Setup), rules/adversary.md (Income, The event deck), rules/ai-boards.md;
configs/baseline.json (`start_corrupted` 1, `start_ambassadors` 1, `supply_per_country` 24, `start_energy` 3);
crates/aic-core/src/modules/board/mod.rs, modules/adversary.rs, modules/ai.rs.*

---

## 4. The round

Each round runs in this order. Only step **D** is yours to play; the rest you resolve.

| | Step | What happens |
|---|---|---|
| | **Round start** | Set your insight (§5). Refresh your powers, Actions, antimatter action and Movement. |
| **A** | **Sway** | Each land-corruption token adds 1 Discord (§12). |
| **B** | **The adversary's turn** (§6) | The face-up event card acts: it deals threats, lands agents, deploys troops, runs projects and corrupts. Adversary troops march. The next card turns face up and the adversary takes its income: agents into the pools, and all its compute onto the projects at once. |
| **C** | **Cards** (§7) | Round 1: the opening deal. Later rounds: build a hand, draft, keep. |
| **D** | **Action phase** (§8) | All players at once: play cards, use powers and Actions, place nodes, move ambassadors, pay threats. |
| **E** | **Production** (§9) | The income step: sites and megaprojects produce and everyone collects income, in one printed order. Then growth: extra corp troops, and each city with a free industrial zone next to it builds a factory. |
| **F** | **Threats** (§10) | Every threat not paid off takes effect. |
| **G** | **War** (§11) | Every front fights. |
| **H** | **Cleanup** | Shelters and other this-round effects end. |

So a threat is dealt in B, paid off in D, and takes effect in F. There is no window to act before the adversary: what
you set up in D (an ambassador, a hardening token) is what meets the next round's adversary turn.

**After every action and every step**, check in this order:

1. Place any Unity just gained on its track, or put it against Discord (§12).
2. Tip any full track, Discord first (§12).
3. Check the ways to lose. Then check the purge. (If you would win and lose at once, you lose.)

*Sources: rules/economy.md (The round); configs/baseline.json `round`; crates/aic-core/src/rules.rs,
modules/alignment.rs.*

---

## 5. Your AI board

Each player is one AI board: the **Builder**, the **Protector** or the **Researcher**. Your board shows a **special
rule**, your **income**, one **tag** (§7), your **building slots**, your **Movement**, two **node tracks** and eight
**powers**.

| Board | Special rule | Income (upkeep) | Insight a round | Slots | Tag | Movement | Starting credits |
|---|---|---|---|---|---|---|---|
| Builder | *Breadth*: 5 slots | 15 credits, 1 energy | 5 (+1 from setup) | 5 | Industry | 1 | 40 |
| Protector | *Wall* and *Vigil* | 13 credits, 2 energy | 5 (+1) | 4 | Security | 1 | 46 |
| Researcher | *Patience* | 11 credits, 1 energy | 5 (+1) | 3 | Science | 2 | 40 |

- **Patience** (Researcher): your own unspent insight carries over to the next round. Insight another player gave you
  does not.
- **Wall** (Protector): adversary troops can't enter, be deployed onto (the troop stays in the supply) or strike a hex
  with your node. Nothing else is
  stopped: corruption, war and threats still destroy what is there. An adversary factory on that hex still makes its
  troop there; it can't strike that hex, and marches off as usual.
- **Vigil** (Protector): in the march, once per adversary troop, when a troop starts a step next to one of your nodes
  whose hex holds a station or factory owned by any player, gain 1 insight (you can spend it this round).

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
| **Movement** | **no** | your board, each round | the Move action, 1 a move |

**Insight is this round's attention.** At the start of each round, set your insight to 5, plus what your uncovered track spaces and
cards add (your setup node's space adds 1, so you start with 6), less any insight a threat or project made you
lose last round (§10, §17). Insight you don't spend is lost (except under Patience). Everything else on your
board and cards pays in production's **income step**, at its end (**upkeep**, §9).

### Node tracks

Your nodes start on your board, on two tracks of 7 spaces: the **economy track** and the **card-play track**. To place a
node, take the next one off either track. That uncovers the space under it, and what the space shows is yours for the
rest of the game: income, Movement, a tag, a free research station, the **keep upgrade** (tier-2 keeps cost 1 insight
instead of 2, §7), and on the last space a **capstone**, a lasting rule (§20). The card-play track does not limit how
many cards you play.

**Placing a node costs two things:** the cost of the power or card that places it, **plus the energy printed on the
space** you uncover. The printed numbers are costs, not income:

| Space | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| Economy track (energy to uncover) | 0 | 0 | 0 | 1 | 2 | 3 | 5 |
| Card-play track | 0 | 0 | 1 | 2 | 2 | 3 | 5 |

A card that places a node **free** pays neither.

> **Example.** The Builder has placed 3 economy nodes. Their Deploy power costs 1 energy, and the economy track's
> fourth space costs 1 more: 2 energy for the node. Placing it on the card-play track instead (its first space, cost 0)
> would cost 1.

**Where:** on a land hex in your reach with no node of any player, no corruption marker and no land corruption. A hex
with agents or a corrupted site is allowed (a node doesn't remove agents).

**A destroyed node leaves the game.** It does not go back to its track; the space stays uncovered.

### Powers

Each power can be used once a round, in the action phase. Every board has two **node powers**: **Deploy** (1 energy:
place a node) and a dearer second one (§20). So powers place up to 2 nodes a round; cards can place more. The other
six powers are conversions, draws and **sacrifice powers**; §20 lists them all.

**Sacrifice.** A power whose cost names a tag ("1 Industry") is paid by discarding one of your cards in play that
carries that tag: a program, or a building together with every building stacked under it (a tag on a covered building
counts). Events can't be sacrificed, and neither can your board or its track spaces. Tags on the card beyond the cost
are wasted.

*Sources: rules/ai-boards.md, rules/ai-powers.md, rules/economy.md (resource table); content/ais.json;
crates/aic-core/src/modules/ai.rs, modules/economy.rs, modules/board/node.rs, modules/cards/sacrifice.rs.*

---

## 6. The adversary's turn

The adversary has a **board**, an **event deck**, a **project deck**, a supply of agents, troops and botnets, and a
**pool** of agents beside the map for each country. Each event card shows its **stage** (I, II or III): the stage sets
which threats it deals (§10). Adversary cards say "each board" where they mean "each country".

The only adversary so far is **the Militarist**. Its board has three project slots and three rules:

- **Garrisons:** +1 agent income in each country. (The agent income in §21's table already includes it.)
- **Troops where the Protector stands:** step 6 below.
- **Warmongering:** in production's growth (§9), each military base with an adversary troop on or next to it gains 1 extra corp
  troop, even past its limit (§19).

### The turn

1. **Event.** The face-up card becomes this round's card. If there is none, you lose.
2. **Threats.** The card's first line deals threats (§10).
3. **Rally X.** Every event card has one. Each country's whole pool lands **evenly on every site of type X** in that
   country (cities, coastal cities, research stations, factories or hexes with a botnet), whoever owns it. The
   remainder goes one each to the lowest-numbered of those sites that aren't the adversary's or firewalled (if too
   few, then the adversary's). The pool is now empty.
4. **Deploy N.** N adversary troops land in each country, one at a time, on its lowest-numbered factory (any owner's);
   with none, on its lowest-numbered city; with none, they don't land.
5. **Other lines**, at most two, in printed order: an income raise, a new neutral factory in each country, Discord,
   Unity taken back, a blackout (cities make no energy this round), and so on. Each card says what it does.
6. **Troops where the Protector stands.** In each country where the Protector has a node: Deploy 1.
7. **Projects.** Ongoing projects act (§17).
8. **Corrupt X**, once for each Corrupt line on the card (below).
9. **Natural corruption.** Every city, station and factory not already the adversary's that holds its corrupt number of
   agents **on its own hex** corrupts. Then every hex with a player's node and **2 agents** loses the node.
10. **March.** Adversary troops march (below).
11. **Reveal and income.** Turn the next event card face up. Put one round of agent income into each country's pool.
12. **Compute.** All the adversary's compute arrives now: its income, each botnet and corrupted station without a
    player's ambassador on it, and what was made since the last turn. Projects fill and resolve (§17). Firewalls come
    off.

**Every Deploy is the same:** an event card's, a project's ("Deploy 3") and step 6's. A Deploy prohibition stops each,
and the level surcharge (§21) adds to each.

**A blocked landing stays in the supply.** A firewalled site's Rally share (§16), a site or troop a prohibition
covers (§12), a Deploy onto a hex where the Protector's node stands (Wall, §5): none of them lands, and nothing moves
on to the next site.

Whenever an agent lands where players' ambassadors stand, it cancels one of them, one for one (§8).

### Corruption

A site's **corrupt number** is how many agents it takes: city **4**, station **3**, factory **3**, open land **3**
(fewer at higher levels, §21), and a **node 2** (a hex with your node, whatever else is there). A corruption marker
on a city counts as an agent there. A site can corrupt two ways:

- **Natural corruption** (step 9, every round): it already holds its corrupt number on its own hex.
- **A Corrupt line** (step 8, when the card prints one): "Corrupt X" names a type (cities, coastal cities, stations,
  factories, open land or nodes). **Every agent next to a site of type X moves onto it** (the players choose if it is
  next to several); then **every site of type X holding its corrupt number corrupts**, and its agents go back to the
  supply. Only sites the adversary doesn't own yet count.

> **Example: Corrupt cities.** A neutral city on hex 6 holds 2 agents. Hex 2, next to it, holds 1; hex 7, also next
> to it, holds 2. All 3 move onto hex 6, which now holds 5, at least 4: it corrupts, and the 5 agents go back to the
> supply. Had hex 7 also been next to another city, the players could have sent its 2 agents there instead, leaving
> hex 6 with 3: safe this round.

Some Discord cards and projects also print a Corrupt line, which works the same way, and some threats and projects
corrupt the site they name. Corruption never spreads by itself.

**When a site corrupts**, the adversary's cube replaces any owner's cube. Any node on that hex is destroyed (unless
something protects it, §13), then every ambassador on it is removed. A corrupted open hex gets a **land corruption**
token: no node can be placed there (a factory or station still can), it adds 1 Discord every round, and nothing
removes it. A site's corruption never leaves a corruption marker. **When a node hex corrupts** (2 agents), only the
node is lost (§13 applies, and it leaves a marker); its agents go back to the supply, and nothing else on the hex
changes. §15 has every detail.

### The march

An adversary troop's **targets** are player-owned research stations and factories, never anything else. In step 10,
each adversary troop takes **2 steps**, one troop at a time (country by country, in hex-number order). Each step:

- if an **ambassador** (any player's) stands on the troop's hex, the troop removes one (the players choose whose) and
  that is its step: ambassadors don't fight, so a troop can remove two in a turn;
- else, if a target is **on or next to** the troop's hex, the troop **strikes** it (the players choose which, if several):
  the target is destroyed unless something protects it (§13). Either way the troop is used up and leaves the map;
- otherwise the troop **moves** one hex toward the nearest target, by land and around Wall hexes, crossing borders if
  it must (the players choose among equally good hexes). With no way to any target, it stays.

So a troop two hexes from your facility will hit it this turn (or another target in range, if the players pick
that one).

> **Example: a march.** An adversary troop stands two hexes from the Builder's factory, and no other player facility
> is nearer. Step 1: no target on or next to it, so it moves one hex toward the factory. Step 2: the factory is next to
> it, so it strikes. A corp troop waits at a military base next to the factory, and the Builder chooses to spend it:
> that corp troop is removed instead, and the factory stands. The adversary troop is used up either way. The Builder
> could instead have spent a hardening token or their own ambassador on the factory, or nothing (§13). A troop is a threat to the nearest player-owned station
> or factory, and you can always see which.

### Projects and botnets

The adversary spends **compute** on **projects**, all of it once a round in step 12: each point goes onto its
cheapest project, and a project that fills **resolves** with its printed effect (§17). Compute comes from its income,
botnets, corrupted stations and some threats; what threats and cards make between turns waits beside the projects.
**Drain** (§8), some cards and the Researcher's Audit take it off a project.

A **botnet** stays until a card or the **Take down a botnet** action (§8) removes it. It makes 1 compute a round
unless a player's ambassador stands on it, and counts against the purge until Unity has tipped 6 times (§14).
Removing one adds 1 Unity.

*Sources: rules/adversary.md (Each round, Income, Rally X, Corrupt X, Botnets), rules/war.md (the march);
content/adversary.json; crates/aic-core/src/modules/adversary.rs, modules/plan.rs, modules/board/troops.rs.*

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

Each player is dealt **8 tier-1 cards** and **2 tier-2 cards**. Keep up to 4 tier-1 cards **free**; each further tier-1
card costs **5 credits**. Each tier-2 card kept costs **1 insight**. Discard the rest. There is no draft in round 1.

### Rounds 2 on: build, draft, keep

1. **Build a hand.** Each player takes 4 cards one at a time, choosing for each whether it comes from the tier-1 or the
   tier-2 deck. Each arrives face up, so you see it before choosing the next. (This is not a draw.)
2. **Draft.** Take one card from the hand in front of you and pass the rest on, until the hands are gone. In odd rounds
   pass to the next player in player order; in even rounds, to the previous one. Solo, you keep all 4.
3. **Keep.** Pay **1 insight** for each tier-1 card and **2 insight** for each tier-2 card you keep (1 with the keep
   upgrade, and less with Science discounts, below). When your insight runs out, each further insight a keep needs
   costs **5 credits**. Discard the cards you don't keep.

Keeps and plays spend the same insight, so every card you keep is insight you can't spend playing one this round.

### Playing a card

In the action phase, play a card from your hand by paying:

- its **credit** cost (the octagon), less your discounts;
- its **insight** cost (the hexagon): 0, 1 or 2. Credits can't pay this;
- any extra cost printed below the art: energy, chips, antimatter or research cubes.

A **requirement plate** must be met. A **yellow plate** means "at least N of this tag on your cards". A
**stacked-cards plate** means the card must be built over a building whose top card has that tag. A card's own tags
never count toward its own requirement.

**Discounts** (a discount never takes a cost below 1; a cost printed 0 stays 0):

- every 2 Science tags on your cards take 1 insight off each card you play, and off each tier-2 card you keep from
  round 2 on;
- every 2 Industry tags take 1 credit off each building;
- some cards make all your cards of one tag cheaper.

**Play a card if any part of it can act.** Parts with nothing to act on (no target in reach, say) are skipped. A
program or event with nothing at all to do can't be played; a building always can.

### Tags

The tags **on your cards** are those on your programs, your buildings (covered ones too), your AI board and your
uncovered track spaces. Tags unlock requirements, set discounts and per-tag payouts, and pay for sacrifice powers and
some threats. Only cards in play can be sacrificed (§5).

### Buildings and slots

Put a building into a free slot, or **build over** one of your buildings; with no free slot you must build over one. A
covered building **stops working**: its income, Effect and Action stop, and any tokens on it sit idle. **Its tags still
count.** A card can't be uncovered.

- An **Action** (`→`: pay the left, get the right) is used once a round. "A, OR B" is one use: either half.
- An **Effect** (`:`: when the left happens, get the right) fires every time, with no limit. A building can trigger
  itself: a Science building with a Science trigger fires when you play it.
- An **income** pays every round in upkeep.
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
| **Move** | 1 Movement | put one of your ambassadors, from your supply or from anywhere on the map, onto a site, base, node or botnet in your reach (not the hex it is on) |
| **Claim** | ambassadors (below) | take a neutral site in reach: it becomes yours |
| **Liberate** | ambassadors (below) | take a corrupted site in reach: it becomes yours |
| **Remove agents** | 1 ambassador from your supply, spent | remove up to 3 agents in your reach, from one hex or several; nothing is placed |
| **Take down a botnet** | 5 credits | remove a botnet from a hex where your ambassador stands (the ambassador stays); add 1 Unity |
| **Buy energy** | 4 credits | gain the energy (1) of a neutral city in your reach that isn't blocked or blacked out; each city once a round |
| **Build a research station** | 2 chips | on an open hex in reach that isn't an industrial zone; it is yours |
| **Clear** | 2 credits | remove a corruption marker in reach; add 2 Unity |
| **Envoy** | 1 ambassador from your supply | remove 1 Discord from the Discord track |
| **Gain an ambassador** | 20 credits | into your supply |
| **Harden** | 3 credits | a hardening token on a player's station or factory in reach (§13) |
| **Drain** | 5 credits | remove up to 3 compute from one adversary project |
| **Research cube** | 1 research cube | gain 1 insight |
| **Sell** | a card from your hand | gain 1 credit |

### Ambassadors meet agents

**Every contact is one for one.** When an agent lands on a hex where players' ambassadors stand, it and one of them
are removed (the agent back to the adversary's supply); if several players' ambassadors are there, the players choose
whose. When your ambassador arrives on a hex with agents, it removes one of them and is removed itself.

**Remove agents** spends 1 ambassador from your supply to remove up to 3 agents anywhere in your reach, one at a
time, from one hex or several: your choice, and you may stop early. It places nothing on the map.

### Claim and Liberate

**Claim** takes a **neutral** site; **Liberate** takes a **corrupted** one. Otherwise they work the same way:

- **Where:** the site must be in your reach.
- **Price:** city **5**, station or factory **3**, plus 1 for each agent on it. Those agents are removed.
- **Who pays:** your own ambassadors standing on the site pay first, then your supply. Another player's never pay.
- **Result:** your cube goes on: the site is **yours**. Liberate first takes the adversary's cube off, and adds
  Unity: city **+5**, station or factory **+2**. A botnet on the site stays.

> **Example: block, then liberate.** A corrupted city in your reach makes 1 Discord a round. You Move an ambassador
> onto it (1 Movement): now it is blocked and makes nothing. A round later you Liberate it: the price is 5 (no agents
> on it), your standing ambassador pays 1 and your supply pays 4. The adversary's cube comes off, yours goes on, and
> you add 5 Unity. The city is yours: from this round's production it makes you 1 energy a round.

**Liberate cards** (some tier-2 cards) liberate a corrupted city in reach for their own price instead. They take the
adversary's cube off and add 5 Unity, but **leave the agents on the city**, so it starts blocked, and put your cube
on only if the card says so.

### Megaprojects and converters

Each megacorp's **megaproject** needs 8 cubes, which research stations in its country put on it (§9). When it
completes, each cube on it pays its owner the payout below, and its **converter** opens. Any player with a **city of
that country in reach** (any city, corrupted ones too) may use the converter, **once a round** each.

| Megacorp (megaproject) | Payout, each of your cubes | Converter |
|---|---|---|
| Horizon (Tidal Grid) | 3 energy, 3 credits | 1 research cube + 2 energy → 1 antimatter |
| Cintamani (Polar Compute Array) | draw 1 card, 5 credits | 1 research cube → 2 insight |
| Onyx (Mountain Fusion Works) | 4 chips, 3 credits | 1 research cube → 2 chips |
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

1. **Factories.** A player's factory makes 2 chips for its owner. A corrupted factory makes 1 adversary troop on its
   own hex. A neutral factory, or a blocked one, makes nothing.
2. **Cities.** Each unblocked city makes 1 energy: a player's city for its owner; a corrupted city 1 Discord instead.
   A neutral city's energy is bought in the action phase (Buy energy, §8), not here.
3. **Stations.** A player's station makes 2 cubes in its owner's color; a neutral one makes 1 neutral cube. They go
   onto their country's megaproject. Once it is complete, a player's cubes go onto their board as **research cubes**,
   and a neutral station's are lost. (A corrupted station's compute comes in the adversary's compute step, §6.)
4. **Megaprojects.** A megaproject holding 8 cubes or more **completes**: every cube on it pays, overflow included
   (§8). Neutral cubes pay nothing.
5. **Upkeep.** Each player collects their income: board, track spaces, programs and buildings.

Insight is not part of it: it is set at the start of the round (§5).

**Growth:**

6. **Extra corp troops.** Warmongering and some projects add corp troops to bases (§19).
7. **Cities build factories.** Every city, blocked or not, builds one factory on a **free** industrial zone (one with
   no factory) **next to it**, country by country in hex-number order; a city with none builds nothing. The factory
   takes the city's owner:
   - a player's city builds its owner's factory, on the zone its owner chooses;
   - a neutral city builds a **neutral** factory (anyone may claim it, §8), on the zone the players choose;
   - a corrupted city builds the adversary's: on a zone with an agent on or next to it if it can (lowest number
     first); a corruption marker there comes off.

*Sources: rules/war.md (Every round, Factories), rules/economy.md (Cities' energy, The round), rules/board.md
(Research cubes and the corps); configs/baseline.json `round` (`board.produce`, `economy.upkeep`, `board.bases`);
crates/aic-core/src/modules/board/production.rs (produce, produce_units, grow_factories), modules/economy.rs.*

---

## 10. Threats

Each round the adversary deals **threats**: problems placed on the map that you pay off in the action phase or suffer
at the end of the round.

**Dealing.** The event card's first line deals one or two threats in each country, each naming one or two **tags**
(Military, Influence, Cyber, Economic, Grid). Turn up threat cards until one fits (§18 has the details).

**Location tokens.** A threat prints one or two location types (city, coastal city, research station, factory,
military base). It puts a **location token** in its country on a place of each type: the one with the most adversary
pieces on or next to it. §18 has the tie-breaks.

**Near and nearest.** These are two different tests:

- **Near (who can pay):** you are near a threat if one of your nodes **or ambassadors** is on or next to one of its
  tokens. Only players near a threat may pay toward it or use a card on it.
- **Nearest (who gets hit):** if the threat takes effect, its harm to a player hits the player whose **node** is
  closest to one of its tokens, at any distance, near or not (tie: the tied players choose which of them takes it).

**Paying it off.** A threat prints one or two ways to pay it off. **Pay one way in full** and it is discarded.

- **Resources** (credits, energy, insight, research cubes, chips, or ambassadors from your supply) may be pooled by all
  the players near it. You may take back what you put on it until you pass. Anything left on a threat that isn't paid
  off goes back to its payers when threats take effect.
- **Tags:** one player alone sacrifices cards in play carrying that many tags of the matching kind (2 for a tier-1
  threat, 3 for tier 2):

| Threat tag | Military | Influence | Cyber | Economic | Grid |
|---|---|---|---|---|---|
| Your card tag | Security | Diplomacy | Cyber | Industry | Power |

Cards can also **jam** a threat (it needs less of each resource), **delay** it (it waits a round, and can be paid next
round), or pay it off outright. A few buildings pay off part of every threat of a type, wherever it is.

**When threats take effect** (step F), each threat not paid off does what it prints, country by country, then is
discarded with its tokens. Pieces it places go on its token. A resource harm reads "pay N or discard M cards": the
player chooses; with neither possible, they pay what they have and discard their hand. A threat dealt after the
action phase began (by a Discord card or another threat) waits until next round.

> **Example.** A Military threat's token sits on a research station. The Protector's ambassador stands next to it, so
> the Protector is near and may pay; the Builder's node is 2 hexes away, so the Builder is not near. Nobody pays.
> In step F the harm hits the player whose node is closest: the Builder, 2 hexes away (the Protector's nearest node is
> 3 away). The Protector's ambassador made them near, not nearest.

*Sources: rules/threats.md; content/threats.json; crates/aic-core/src/modules/threats/ (mod.rs, near.rs, counter.rs).*

---

## 11. War

The megacorps fight each other across the fronts with **corp troops**. These are not the adversary's troops: they
never march, and they never strike your stations.

**Where corp troops come from.** Each base holds up to **2** corp troops. Whenever a corp makes troops, it puts 1 on
each of its bases that has room. A corp makes troops:

- **each time the Discord track tips** (§12): every neutral factory puts 1 corp troop on each base of its own country,
  unless an agent stands on it. Each of your ambassadors on that factory, or on a base, stops one troop and is spent
  (§19);
- from Warmongering and some projects, in production's growth (§9, §19).

**War**, at the end of every round (step G), on each front:

1. The two bases' piles cancel 1 for 1, and both bases are emptied.
2. Each troop left over crosses and strikes the rival corp's factory nearest its base, counting hexes (tie: the lower
   number): a neutral or player factory, never the adversary's.
3. The strike is on the **factory**, and the factory is protected like any facility (§13). If it is destroyed, it
   adds **1 Discord**, and **any node on its hex** would be destroyed too: that node is protected like any node, so
   its owner may spend a protection for it (§13).
4. A troop whose strike is stopped is still used up, and the next troop aims at the same factory.

A corp troop at a base also protects facilities on or next to its hex from **adversary** troops: the facility's owner
may spend it instead (§13).

*Sources: rules/war.md; crates/aic-core/src/modules/board/troops.rs, modules/board/production.rs.*

---

## 12. Unity and Discord

The world leans toward **Unity** (you) or **Discord** (the adversary). Each side has its own **track**, its own **tip
count** and its own deck of one-shot cards.

### Gaining Unity and Discord

Effects **sway** Unity or Discord ("add 2 Unity"): put that many on that side's track.

- **Unity** comes from liberating sites (city 5, station or factory 2), clearing markers (2), removing botnets (1),
  Biomia, and cards.
- **Discord** comes from corrupted cities (1 each a round), land corruption (1 each a round), factories destroyed in war
  (1 each), the antimatter action (1), and threats, projects and cards that print it.

**Unity against Discord.** Whenever you gain Unity, you may put any of it against Discord instead: each Unity put
there removes 1 Discord from the Discord track. You decide as the action or step that gained it resolves, before
any track tips. The rest goes onto the Unity track. (**Envoy** also removes 1 Discord, for 1 ambassador.)

> **Example.** 3 players, so a track tips at 9. Discord is at 7. One step adds 3 Discord (10) and 2 Unity. Putting
> both Unity against Discord leaves 8: Discord doesn't tip. Had the step added 4 Discord (11), the same 2 would leave
> 9: Discord tips, but keeps nothing toward its next tip instead of 2.

### Tipping

A track **tips** when it holds **3 × the number of players**. Then:

1. Take one tip's worth off. The rest stays on the track toward its next tip, on both tracks.
2. Add 1 to that side's tip count.
3. **Discord only:** every neutral factory makes corp troops (§11).
4. Draw that side's top card and resolve it at the tier for its tip count, this tip included: **tier I** on tips 1-2,
   **tier II** on tips 3-5, **tier III** from tip 6.

If both tracks are full, Discord tips first. A card that fills a track again tips it again.

### The cards

Every Unity and Discord card prints three tiers. Each acts across the map by a condition ("for each site a player
owns"), not in one player's reach. Some Unity cards **forbid** the adversary's next Rally, Deploy or Corrupt where a
condition holds: that step skips each site where the condition holds when the step runs, and what it would have placed
stays in the supply. A Corrupt prohibition also stops natural corruption there, and no agent moves onto such a site; a
Deploy prohibition stops every Deploy that turn: the event card's, a project's and the Protector landing.

- **The Unity deck** (23 cards) reshuffles when it runs out.
- **The Discord deck** (6 cards) **never reshuffles.** A Discord tip with no card left loses the game: you survive 6
  Discord tips and lose on the 7th, at every level (§21).

*Sources: rules/alignment.md; configs/baseline.json (`tip_per_player` 3, `carry` 2, `hold_off` 1, `tier_at` 3, 6); content/alignment.json;
crates/aic-core/src/modules/alignment.rs, modules/board/production.rs (the tip's corp troops).*

---

## 13. Protecting your pieces

### What can be destroyed

Your **nodes** and your **facilities** (stations and factories) can be destroyed:

| By | What it destroys |
|---|---|
| **Corruption** (a Corrupt line, natural corruption, a threat that corrupts a site) | the node on the corrupted hex, then every ambassador there |
| **A node hex's corruption** (2 agents on a hex with your node, §6) | that node; the agents go to the supply |
| **An adversary troop's strike** | the player-owned station or factory it strikes (never a node) |
| **War** | the factory a corp troop strikes, and any node on its hex |
| **Threats and event lines** | what they print ("destroy a node", Border War's node near a base) |

A destroyed facility takes its owner's cube with it. A destroyed node leaves the game and leaves a **corruption
marker** on its hex, unless the hex already holds a marker, a botnet or an adversary cube. A site's corruption leaves
none (the adversary's cube goes on); a node hex's corruption (2 agents) does, as any lost node. No node can be placed on a
marker. **Clear** (§8) removes one.

### The protection order

When a node or facility would be destroyed:

1. **Shelter** on that country this round (the Protector's power, some cards) stops it, automatically, spending
   nothing. It lasts from the action phase to cleanup, so it covers production, threats and war, never the
   adversary's turn.
2. Otherwise **its owner chooses** one protection to spend, or none (for a neutral piece, the players choose). Each is
   used up:
   - a **corp troop** at a base on or next to the hex (against an adversary troop's strike only);
   - a **hardening token** on the hex (on a hex with your node it is called a **shield**);
   - **your own ambassador** on the hex (never another player's);
   - **Aegis Doctrine** (a Protector capstone): once a round, on one of the Protector's own nodes or facilities.
3. With nothing spent, the piece is destroyed.

The owner of a node is its player; of a facility, the player whose cube is on it. A node on a factory that war
destroys is protected the same way (§11). Corruption never destroys the node of a player with **Sanctuary** (a
Protector capstone), and spends nothing of theirs.

**Hardening** goes on a player-owned station or factory (Harden, §8; the Protector's Bulwark).

### Your last node

Nothing protects a last node but the protection order above: anything that destroys a node can destroy your last
one, at any stage. Any player with no nodes loses the game for everyone, so keep a second node on the board: the
first spaces of both tracks cost no energy, so an early second node costs only its power. Watch the hexes next to a
lone node: two agents on it take it, and a Corrupt nodes line pulls in every agent next to it.

*Sources: rules/war.md (Node loss), rules/ai-powers.md (Shelters, hardening and shields), rules/ambassadors.md
(Armor); crates/aic-core/src/modules/board/destroy.rs, modules/board/node.rs, modules/adversary.rs (corrupt_hex),
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

Agents waiting in the pools are not on the map. Corrupted stations, factories and land never count, nor do corp
troops.

### Losing

Everyone loses at once if:

- a Discord tip finds the Discord deck empty;
- any player has no nodes on the map;
- the event step finds no card to turn up (the **round cap**: 11 rounds at levels 0-2, 9 at levels 3-4). The round
  after the last card still runs its Sway step first, so a Discord tip there can lose the game before the event step.

The adversary running out of agents is not a loss: a placement it can't make doesn't happen.

*Sources: rules/alignment.md (The purge, Losing), rules/adversary.md (The event deck); configs/baseline.json
(`purge_at`); crates/aic-core/src/modules/alignment.rs, modules/adversary.rs.*

---

# Part 2: Reference

## 15. Corruption in full

**Corrupt X** names a type: cities, coastal cities, research stations, factories, open land or nodes. A **target** is
a place of that type the adversary doesn't own ("open land": an open hex with no base, station, factory or land
corruption, where a node may stand; "nodes": a hex with any player's node).

1. **Move.** Every agent next to a target moves onto it, across borders too. An agent next to several targets goes
   onto one the players choose; agents already on a target stay. An agent moving onto players' ambassadors cancels
   one of them (§8).
2. **Corrupt.** Every target holding its corrupt number corrupts (§6), and its agents go back to the supply. A
   corruption marker on a city counts as an agent there. A node target only loses its node (§13 applies); nothing
   else on its hex changes.

A card may print "(1 fewer)": the corrupt number is 1 lower for that line. No agent moves onto a firewalled target,
but it still corrupts on what it holds.

**Natural corruption** (step 9) checks each city, station and factory that isn't the adversary's: with its corrupt
number of agents on its own hex (a marker on a city counting as one), it corrupts. Then each hex with a player's node
and **2 agents** loses that node (§13 still applies); the agents go back to the supply and the site there, if any, is
untouched. Open land never corrupts this way. A firewall doesn't stop it.

> **Example: Corrupt cities.** A neutral city on hex 6 holds 2 agents. Hex 2, next to it and to no other city, holds
> 1; hex 7 holds 2 and is next to hex 6 and to the city on hex 11, which holds 1. Hex 2's agent moves onto hex 6. The
> players send both of hex 7's agents to hex 11 (they choose): hex 6 holds 3 and hex 11 holds 3, short of 4, so
> neither corrupts this time. Had they sent one each way, hex 6 would hold 4 and corrupt.

**What corruption destroys:** the node on the hex (§13), then every ambassador on it. Nothing else.

*Sources: rules/adversary.md (Corrupt X); crates/aic-core/src/modules/plan.rs (converge, corrupt_plan,
natural_targets), modules/adversary.rs (corrupt_sites, corrupt_node).*

---

## 16. Moving and removing pieces

Some cards and the Protector's Patrol move pieces on the map: ambassadors (any player's), agents and adversary troops.
Botnets and markers never move.

- **Gather up to N:** pick a land hex in your reach; move pieces into it, each from a hex next to it.
- **Push up to N:** pick a land hex in your reach; move pieces off it, each to a land hex next to it, your choice.

You may stop after any piece. Moved pieces arrive as usual: an agent moved onto players' ambassadors cancels one of
them (§8).
No agent enters a firewall; no adversary troop enters the Protector's node hex.

**Remove from around a hex.** Some cards (Kettle Operation, Lure Network, Honeypot Relay) remove up to N agents, or
agents and troops, from the hexes **next to** a hex you pick in reach, never from that hex itself: troops first, then
agents, in hex-number order.

**Convert.** Some late cards turn agents into your ambassadors: up to N agents on or next to a city in your reach come
off and become your ambassadors, arriving on the hex of the node that gave you reach (your choice if several).

**Firewall.** Some cards put a firewall on a land hex in your reach. While it stands, no agent lands, is placed or
moves in there: its Rally share stays in the supply, like any blocked landing. Troops pass. It comes off at the end
of the next adversary turn (step 12).

**Cordon.** Hardpoint Lockdown cordons a country: until the end of next round, no agents land there from a Rally or a
card line (troops still land, and Gather and Push still move agents in). That country's pool still empties at its
Rally: those agents go back to the supply.

*Sources: rules/ambassadors.md (Moving pieces), rules/adversary.md (Firewalls); crates/aic-core/src/modules/effects/moves.rs,
modules/effects/endgame.rs, modules/effects/removal.rs, modules/adversary.rs.*

---

## 17. Projects

The adversary board has **slots** (the Militarist: one Military slot that takes only a Military project, and two open
slots that take any). **To deal**, turn up projects one at a time until every slot is full: each goes into the first
empty slot it fits (a Universal project fits any), else into the project discard. When the deck runs out, shuffle the
discard back in.

A project prints its cost **for each number of players**. All compute arrives in one step of the adversary's turn
(step 12, the **compute step**): its printed compute income, 1 for each botnet and each corrupted research station
without a player's ambassador on it, and every compute made since the last compute step (threats taking effect, cards
and projects that print it), which waits beside the projects until then. Put it on one point at a time: each goes
onto the **cheapest** project showing, by cost (tie: the first slot). The moment a project holds its cost, it
**resolves** and leaves its slot:

- a **one-time** project does what it prints and leaves the game;
- an **ongoing** project does its "Now:" line at once, then goes beside the board and acts every round from the next:
  in step 7 of the adversary's turn, or in production or war if that is what it prints.

**Each slot refills as it empties:** deal the next project into it at once, and the rest of the compute goes on.
Compute with no project to go on is lost. No project resolves outside the compute step.

*Sources: rules/adversary.md (The project deck); content/projects.json; crates/aic-core/src/modules/adversary.rs.*

---

## 18. Threat details

**Dealing.** Stage I event cards deal one tier-1 threat in each country, stage II one tier-2 (Supply Lines: one tier-1
and one tier-2), and stage III two tier-2. Each deal names one or two threat tags, or "any". For each deal, for each
country in turn, turn up the top card of that tier's threat deck. It stays if it carries a named tag (a **Universal**
card has every tag) **and** the country has a place of one of its location types. Otherwise discard it and turn up the
next. When a threat deck runs out, shuffle its discards into a new deck.

**Location tokens.** For each printed type, the token goes on the place of that type with the most adversary pieces
(agents, troops, botnets) on or next to it; tie: the lower hex number. Cities and stations must not be corrupted;
factories must not be the adversary's (with none, a free industrial zone takes the token); any base will do. A type
the country has none of gets no token. A threat's two tokens never share a hex.

**Difficulty.** From level 1, resource items on threats cost more (§21); tag items never change.

*Sources: rules/threats.md; crates/aic-core/src/modules/threats/mod.rs, threats/counter.rs.*

---

## 19. Corp troops in full

- **At a Discord tip,** every neutral factory with no agent on it puts 1 troop on each base of its own country that has
  room (2 bases at 3-4 countries, 1 at 2 countries).
- **One ambassador always stops one troop**, and is spent. A player's ambassador on a base stops the troop that base
  would get. Each one on a neutral factory stops one of that factory's troops at that tip (if it can't stop them
  all, the players choose which base goes without). A full base takes no troop and spends no ambassador (with
  several players' ambassadors there, the earliest in player order is spent).
- **Warmongering** and the Arms Race project add 1 troop to a base even past its limit (at most 1 extra a base a
  round). The War Footing project fills every base.

*Sources: rules/war.md; crates/aic-core/src/modules/board/production.rs.*

---

## 20. Powers and capstones

Each power can be used once a round, in the action phase. A tag in a cost means sacrifice a card with that tag (§5).

| Board | Node power | Conversions | Draw | Sacrifice and other powers |
|---|---|---|---|---|
| Builder | **Buy In** (1 energy, 6 credits: a node) | **Mint** (3 credits: 1 chip), **Focus** (5 credits: 1 insight) | **Procure** (2 chips: draw 2) | **Scrap** (1 Industry: 2 chips), **Overcharge** (1 Power: 2 energy), **Rush Build** (1 Industry, 1 energy: play a building from your hand without paying its credits; its other costs and requirements stand) |
| Protector | **Accord** (1 Diplomacy, 1 energy: a node) | **Stand Down** (1 Security: 3 credits) | **Debrief** (1 Diplomacy: draw 2) | **Recovery** (1 Security: clear a marker in reach, +2 Unity), **Shelter** (1 Security, 1 energy: in any one country, nothing can be destroyed this round), **Bulwark** (1 energy: harden a facility in reach), **Patrol** (1 energy: push up to 3 agents or adversary troops from a hex in reach) |
| Researcher | **Peer Review** (3 research cubes: a node) | **Grant** (3 credits: 1 cube in your color on a megaproject that isn't complete), **Synthesis** (discard a card from your hand: 2 insight) | **Fieldwork** (1 energy: draw 3) | **Field Station** (1 Industry: a research station without the chips, where Build a research station could go), **Audit** (1 Cyber: remove 3 compute from a project), **Particle Run** (1 Power, 1 energy: 1 antimatter) |

Every board also has **Deploy** (1 energy: place a node). Node powers also pay the track space's energy (§5).

| Board | Economy track capstone | Card-play track capstone |
|---|---|---|
| Builder | **Full Capacity**: 1 chip each round per 2 Industry tags on your cards | **Master Plan**: your buildings cost 2 credits less |
| Protector | **Sanctuary**: corruption never destroys your nodes | **Aegis Doctrine**: once a round, when one of your nodes or facilities would be destroyed, you may keep it (you choose when) |
| Researcher | **Open Archive**: 1 research cube each round | **Open Science**: each round in upkeep, give 2 insight from the supply in all to other players, split as you like; it arrives with their insight next round |

*Sources: rules/ai-powers.md, rules/ai-boards.md; content/ais.json; crates/aic-core/src/modules/ai.rs.*

---

## 21. Difficulty

Your claim and liberate costs are the same at every level. Each row is the whole level:

| Level | Corrupt numbers (city / station / factory / land / node) | Threat costs | Deploy | Unpaid threats | Event cards (I / II / III) | Rounds | Compute income | Agent income a country (with Garrisons) | Other |
|---|---|---|---|---|---|---|---|---|---|
| 0 | 4 / 3 / 3 / 3 / 2 | as printed | as printed | — | 4 / 4 / 3 | 11 | 1 | 1 | — |
| 1 | 3 / 2 / 2 / 3 / 2 | +1 each non-credit resource, +4 credits | as printed | — | 4 / 4 / 3 | 11 | 4 | 4 | — |
| 2 | 3 / 2 / 2 / 3 / 2 | +2 each non-credit resource, +4 credits | +1 troop on every Deploy | +4 compute each | 4 / 4 / 3 | 11 | 22 | 4 | — |
| 3 | 3 / 2 / 2 / 3 / 2 | as level 2 | +1 | +4 compute each | 4 / 2 / 3 | 9 | 24 | 4 | — |
| 4 | 3 / 2 / 2 / 3 / 2 | as level 2 | +1 | +4 compute each | 3 / 2 / 4 | 9 | 24 | 4 | the last stage-I card deals tier-2 threats |

Both incomes climb with the level (never down). Level 4 is level 3 with a stage-I card swapped for a stage-III card
and tier-2 threats a card sooner. Threat surcharges apply to resource items only; a tag
item ("2 Security") never changes.

*Sources: rules/adversary.md (Difficulty, The event deck), rules/threats.md (Countering); configs/baseline.json
`adversary` (`compute_income`, `ambassador_income`, `counter_bonus_*`, `counter_more_*`); content/adversary.json
(Garrisons); crates/aic-core/src/modules/adversary.rs, modules/threats/counter.rs.*

---

## 22. Player count and solo play

- **Countries:** one per player, 1-4. 2 countries share one front; 3 countries have three fronts; 4 form a ring of
  four.
- **Boards:** Builder, Protector, Researcher, then a second Builder. 2 players: no Researcher. Solo: the Builder only.
- **Tipping:** both tracks tip at 3 × the number of players.
- **Projects** print a cost for each number of players (§17).
- **Corp troops:** at 3-4 countries every corp has two bases, so a neutral factory makes 2 troops at a Discord tip.
- **Solo:** one country, so no fronts, no bases and no war. You keep all 4 cards you take in the draft. "Another
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
| Pieces (bare silhouettes) | ambassador (a figure), node (a house shape), troop, botnet (a starburst), cube, corruption marker |
| Places | city (a skyline), research station (a dome with a mast), factory (a sawtooth roof), a hex outline, a country (a cluster of hexes), a megaproject (a card with a cube), an adversary project (a plain card) |
| Tags (ringed circles) | Science, Industry, Cyber, Power, Diplomacy, Security, each its own symbol; an empty ring is any tag |

### Marks

| Mark | Means |
|---|---|
| Number before an icon | how many (no number: 1) |
| Minus before an icon | pay it or lose it now |
| `≤`N | up to N, your choice |
| `≥`N | at least N (a rebate's printed credit cost) |
| `/` | per |
| OR, ALL, ACT | written out; ACT is an Action granted |
| Brown box above the art | income, each round |
| Brown box below the art, with a plus | raise your income once (with a minus: lower it) |
| **Dark frame** | acts on the adversary: its pieces, projects or threats. Inside it a minus number means "remove up to N" |
| Ambassador figure **in the dark frame** | an **agent** (outside the frame it is an ambassador) |
| A place after a piece | where: after a cube, only that kind of site; after a botnet or ambassadors, on or next to that place; after a country, in one country |
| Dashed outline | another player |
| Thin solid outline | one of your cards: a disc in it goes to, or comes from, one of your cards |
| A disc alone | a token on this card ("here") |

### Connectors

| Mark | Means |
|---|---|
| `→` | an Action: pay the left, get the right, once a round. A bare `→` is free. |
| OR at a line's start | the other half of the Action above: one use a round, either half |
| `:` | an Effect: when the left happens, get the right. A minus after it: those cards cost that much less |
| Discs left of `:` | At N: when the tokens here reach N, they come off and the right happens |

### Special marks

| Mark | Means |
|---|---|
| Play (a card with an arrow onto it) | a card you play, with its tag if any; in a dashed outline, another player's |
| Node | place a node, paying its track space |
| Node with a struck-through coin | place a node free |
| Shield | stops one destruction: alone, a hardening token on a facility; with a node, a shield, or on a capstone the Aegis Doctrine (your own nodes and facilities) |
| Shelter (a roof) | a whole country, this round |
| Threat (`!`) | a threat; struck through: pay it off |
| Jam (a threat with a down arrow) | a threat needs that much less of each resource (its minus means "less to pay") |
| Clock | after a threat: delay (it waits a round). After an agent and a hex: a firewall. After an agent and a country: a cordon |
| Requirement plates | yellow: at least; stacked cards: built over |

**When an Effect fires.** An Effect on removing troops fires once for each troop. One on removing agents or compute
fires once for each card, Action, power or standard action that removed any. A **rebate** Effect fires when you play a
card whose printed credit cost, before discounts, is at least N.

*Sources: docs/card-guidelines.md (Legend), rules/card-grammar.md (Boxes, Connectors, Shapes);
crates/aic-core/src/face.rs, modules/cards/triggers.rs.*

---

## 24. Glossary

- **Agent:** the adversary's piece. **Ambassador:** a player's piece, any player's.
- **At N:** §7, *Tokens on cards*.
- **Base (military base):** where a megacorp keeps its corp troops for one front, §2 and §11.
- **Blocked:** a site with an opposing piece on it; it makes nothing, §2.
- **Building:** a card. **Facility:** a research station or factory on the map.
- **Compute:** what the adversary spends on projects, §6 and §17.
- **Corrupt number:** how many agents corrupt a site, §6.
- **Corrupted:** owned by the adversary. **Neutral:** owned by nobody.
- **A country you're in:** a country where you have a node. **Your home country:** the one you start in.
- **Draw:** take the top tier-1 card, free.
- **Firewall, cordon:** §16. **Front:** §2.
- **Here:** on this card.
- **In reach:** on or next to one of your nodes.
- **Income step:** production's first part, where every site, megaproject and income pays in one printed order;
  **growth** follows it, §9.
- **Land:** a city or open hex.
- **Megaproject:** a megacorp's 8-cube card, §8.
- **Movement:** your Moves each round, §5.
- **Near (a threat):** your node or ambassador on or next to one of its tokens. **Nearest:** the player whose node is
  closest, §10.
- **Next to:** sharing an edge. **On or next to:** that hex, or one sharing an edge with it.
- **On your cards:** on your programs, your buildings (covered ones too), your board and your track spaces.
- **Natural corruption:** a site corrupting on the agents on its own hex, and a node hex losing its node to 2 agents, §6.
- **Player order:** Builder, Protector, Researcher, second Builder, §3.
- **Pool:** the agents waiting to land in a country, §6.
- **Purge:** the win, §14.
- **Remove up to N:** as many as there are, up to N.
- **Shield:** a hardening token on a hex with your node.
- **Site:** a city, research station or factory.
- **Stage:** I, II or III, printed on each event card, §6.
- **Sway:** add Unity or Discord to its track. **Tip:** §12.
- **Tier:** player cards and threats come in tier 1 and tier 2 (decks); Unity and Discord cards resolve at tier I, II
  or III (by tip count).
- **Troop:** on a card, an adversary troop, unless it says corp troop.

*Sources: rules/README.md (Terms), docs/player-aid.md.*
