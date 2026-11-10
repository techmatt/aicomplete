# AI Complete: Rulebook (first draft)

*Rules revision 73. A first draft: it states the game as the simulator plays it today, which is not always what
`docs/rules/` says (the differences are listed in the friction table). Every number is a candidate. The AI
boards are placeholders, and so are the threats, the adversary's cards, and the Unity and Discord cards. Specific card
text is therefore left out. Each section ends with a **Sources** line naming the rules files ([rules/](rules/README.md))
and code it states, so a reviewer can check it. What was hard to write is ranked in
[rulebook-friction.md](rulebook-friction.md).*

## Contents

1. [The game in brief](#1-the-game-in-brief)
2. [The board](#2-the-board)
3. [Setup](#3-setup)
4. [The round](#4-the-round)
5. [Resources](#5-resources)
6. [Your AI board](#6-your-ai-board)
7. [Cards](#7-cards)
8. [The action phase](#8-the-action-phase)
9. [Ambassadors and sites](#9-ambassadors-and-sites)
10. [The adversary's turn](#10-the-adversarys-turn)
11. [Threats](#11-threats)
12. [Production and upkeep](#12-production-and-upkeep)
13. [Troops, war and destruction](#13-troops-war-and-destruction)
14. [Unity and Discord](#14-unity-and-discord)
15. [Winning and losing](#15-winning-and-losing)
16. [Difficulty](#16-difficulty)
17. [Player count and solo play](#17-player-count-and-solo-play)
18. [Reading a card: the icon legend](#18-reading-a-card-the-icon-legend)
19. [Glossary](#19-glossary)

---

## 1. The game in brief

You and up to three other players are good AIs working together against one evil AI, **the adversary**. The adversary
sends its **agents** into the world's cities and facilities, where they crowd in and corrupt them. It raises troops
and pushes the world toward **Discord**. You spread your **nodes** over the board and send your **ambassadors** to
claim and free sites. You also build up a tableau of cards that pays for it all.

Everyone wins or loses together.

- **You win** by the **purge**: removing the adversary's hold on the board. How much you must remove depends on how far
  the world has swung toward **Unity**. With little Unity you must clear almost everything; once Unity has tipped 6
  times, only the corrupted cities.
- **You lose** if a Discord tip finds the Discord deck empty, if any player has no nodes left on the board, or if the
  adversary's event deck runs out (11 rounds at most).

Each round the adversary acts first, and next round's card is always face up. Then you draft cards and act together,
all at once, in any order. Then the board produces, unpaid threats take effect, and the countries fight the war across
their borders.

*Sources: rules/README.md, rules/alignment.md (The purge, Losing), rules/adversary.md (Each round).*

---

## 2. The board

### Countries and hexes

There is one **country** per player. A country is a square piece of 16 hexes, numbered 1-16 row by row. The layout of
the pieces depends on the number of players. Each country is also a **megacorp** (Horizon, Cintamani, Onyx, Biomia)
with its own **megaproject** card.

A hex is **city**, **open** land or **ocean**; city and open hexes are **land**. Some open hexes are printed as **industrial zones**: factories go only
there. A city next to ocean, or printed as coastal, is a **coastal city**. Nothing is ever built on ocean, and no piece
enters it.

Two hexes are **adjacent** if they share an edge, across borders too. **Next to** a hex means on it or adjacent to it.
When the rules break a tie between hexes, the **lower hex number** wins. Between countries, the lower country number
wins.

### Sites

**Cities**, **research stations** and **factories** are **sites**. A **facility** is a research station or factory.
(A **building** is always a card, never a piece on the board.) A site is owned by a player, by the adversary, or by
nobody. An owner cube on it shows which:

| Owner | City | Research station | Factory |
|---|---|---|---|
| Nobody: **neutral** (a city: **clean**) | sells its 1 energy a round to a player with a node next to it | 1 neutral cube onto its megaproject (§12) | nothing (but corp troops at a Discord tip, §13) |
| A player: **owned** (a city: **aligned**) | its 1 energy a round to its owner | 2 cubes in its owner's color | 2 chips to its owner |
| The adversary: **corrupted** | its energy becomes 1 Discord (§14) | 1 adversary compute (§10) | 1 adversary troop on its hex |

Every city also builds a factory each round (§12).

### Fronts and military bases

A **front** is a long edge shared by two country pieces, marked on the tiles. With 2 countries there is one front;
with 3 or 4, every country has exactly two. Each country has a **military base** for each of its fronts, on an open
hex by the border (the setup diagram for the player count shows where). A base is not a site: it is its megacorp's
pile of **corp troops** for that front (§13). A one-player game has no fronts and no bases.

### Pieces

| Piece | Whose | What it does |
|---|---|---|
| **Node** | a player | where you act from. One node per hex, of any player. |
| **Ambassador** | a player ("an ambassador" means any player's) | claims and frees sites, blocks, guards, cancels agents |
| **Agent** | the adversary | lands on sites, shuts them off, gathers to corrupt them |
| **Botnet** | the adversary | lasting token: makes compute, blocks claims nearby |
| **Adversary troop** | the adversary | raids your stations and factories |
| **Corp troop** | a megacorp (nobody's) | waits at a base and fights across its front |
| **Owner cube** | a player or the adversary | who owns a site |
| **Corruption marker** | none | left where a node was destroyed |
| **Land corruption** | the adversary | a corrupted empty hex |
| **Firewall, hardening token, location token** | none | §9, §13, §11 |

Some cards hold their own tokens: **Datasets**, **Followers**, **Capital** and **Operatives** (§7). An Operative is a
token on a Security card, not a piece on the board.

### Reach

A hex is **in reach** if it is on or next to one of your nodes. Every board target of a standard action, a power and
most cards must be in your reach. A few big cards say "in one country" instead, and then any country will do.

*Sources: rules/board.md (Country pieces), rules/ambassadors.md (One site model), rules/war.md (Bases and war),
rules/README.md (Terms); content/map.json; crates/aic-core/src/map.rs, modules/common.rs.*

---

## 3. Setup

1. **Lay out the map** for the number of players, one country each, with the military bases on their hexes.
2. **In each country** (each piece prints these places):
   - its marked city gets an adversary cube (it starts **corrupted**);
   - its marked research station gets an adversary cube and a **botnet**;
   - its other research station, next to the printed **starting hex**, starts **neutral**;
   - its two printed factories start **neutral**;
   - its megaproject card goes beside the board, empty.
3. **The adversary:**
   - its agent supply is 24 per country;
   - build the **event deck**: deal the number of stage I, II and III cards your difficulty asks for (§16), shuffle
     each stage on its own, and stack them stage I on top (each card shows its **stage** on its back); turn the top
     card face up;
   - put one round of **agent income** in each country's **pool** (1 agent a country at the base difficulty);
   - shuffle the project deck and deal projects into the adversary board's slots (§10).
4. **Shuffle** the Unity deck, the Discord deck, the two threat decks and the two player decks. The Unity track and
   the Discord track start empty (§14).
5. **Each player** takes an AI board, in this order: Builder, Protector, Researcher, then a second Builder. This is the
   **player order**: player 1 plays in country 1, and so on. Each player:
   - places their first node from the economy track on their country's starting hex (uncovering the track's first
     space, +1 insight);
   - takes 1 ambassador, 3 energy and the board's starting credits;
   - puts their cube on their **home city**: the clean city of their country nearest their first node (tie: the lower
     hex number). It is an aligned city in every way, and can be lost like any other.

The game starts with the adversary's turn, before anyone acts (§4).

*Sources: rules/board.md (Setup), rules/adversary.md (Income, The event deck), rules/ai-boards.md;
configs/baseline.json (`start_corrupted` 1, `start_ambassadors` 1, `supply_per_country` 24, `start_energy` 3);
crates/aic-core/src/modules/board/mod.rs, modules/adversary.rs, modules/ai.rs.*

---

## 4. The round

A round runs in this order:

| | Step | What happens |
|---|---|---|
| | **Round start** | Your insight is set (§5). Your powers, Actions, antimatter action and Movement refresh. |
| **A** | **Sway** | Each land-corruption token sways 1 Discord (§14). |
| **B** | **The adversary's turn** (§10) | The face-up event card becomes this round's card: threats, rally, deploy, projects, corruption. Adversary troops march. The next card is turned face up, and the adversary collects its income. |
| **C** | **Cards** (§7) | Round 1: the opening deal. Later: build a hand, draft, keep. |
| **D** | **Action phase** (§8) | All players at once: play cards, use Actions and powers, place nodes, take standard actions, pay threats. |
| **E** | **Production and upkeep** (§12) | Factories, cities, stations and megaprojects produce; everyone collects income. |
| **F** | **Threats** (§11) | Unpaid threats take effect. |
| **G** | **War** (§13) | Every front fights. |
| **H** | **Cleanup** | Shelters and other one-round effects end. |

**After every action and every step,** check in this order: place any Unity just swayed, or hold Discord off with it
(§14); tip any full track, Discord first (§14); check the ways to lose; check the purge (you win).

*Sources: rules/economy.md (The round); configs/baseline.json `round`; crates/aic-core/src/rules.rs,
modules/alignment.rs.*

---

## 5. Resources

Nothing has a holding limit, and nothing caps how much you produce.

| Resource | Kept? | Comes from | Spent on |
|---|---|---|---|
| **Credits** | yes | board income, track spaces, cards, megaprojects, selling a card (1) | card costs, extra opening keeps, standard actions, threat counters, a neutral city's energy |
| **Energy** | yes | board income, your cities, neutral cities (bought), cards | node placements, powers, power-hungry cards, Actions, threat counters |
| **Insight** | **no** | set each round from your board, tracks and cards; research cubes; some powers and cards | keeping and playing cards, threat counters |
| **Chips** | yes | your factories, cards, Builder powers | some buildings, research stations, Procure, threat counters. Chips can't be sold. |
| **Antimatter** | yes | the antimatter action (§8), cards, Horizon's converter | tier-2 cards |
| **Research cubes** | yes | your stations, once their megaproject is complete (§12) | 1 insight each, converters, Peer Review, threat counters |
| **Ambassadors** | yes (your supply) | 1 at the start, cards, a 20-credit standard action | claiming, liberating, Envoy, guarding, blocking, threat counters |
| **Movement** | **no** | your board, each round | moving ambassadors, one point a move |

**Insight is this round's attention.** At the start of each round your insight is set to your board's 5, plus the
insight your uncovered track spaces and cards add, less any insight you were made to lose last round. Insight you
don't spend is lost. **Exception:** the Researcher keeps their own unspent insight (*Patience*), but not insight
another player gave them.

### When income arrives

Insight comes at the start of the round. Everything else on your board and cards comes in
upkeep, at the end of production (§12). A city's energy and a factory's chips come earlier in production.

*Sources: rules/economy.md (resource table, No limits), rules/ai-boards.md, rules/player-cards.md (Playing);
crates/aic-core/src/modules/economy.rs, modules/board/production.rs.*

---

## 6. Your AI board

Each player is one AI board: the **Builder**, the **Protector** or the **Researcher**. A board prints a **special
rule**, an **income**, one **tag** (§7), its **building slots**, its **Movement**, two **node tracks** and eight
**powers**.

| Board | Special rule | Income each round | Slots | Tag | Movement | Starting credits |
|---|---|---|---|---|---|---|
| Builder | *Breadth*: 5 building slots | 15 credits, 5 insight, 1 energy | 5 | Industry | 1 | 40 |
| Protector | *Wall* and *Vigil* (below) | 13 credits, 5 insight, 2 energy | 4 | Security | 1 | 46 |
| Researcher | *Patience*: your own unspent insight carries over to the next round | 11 credits, 5 insight, 1 energy | 3 | Science | 2 | 40 |

**Wall:** adversary troops can't move onto, be deployed onto or strike a hex with the Protector's node. (An
adversary factory on that hex still makes its troop there, but the troop can't strike the hex.) Wall stops only
adversary troops: war, threats and corruption still destroy what is there. **Vigil:** once per adversary troop each
march, if the troop stands next to your node's hex before one of its steps, and that hex holds a player-owned station
or factory, gain 1 insight.

### Node tracks

Your nodes start on your board, on two tracks of 7 spaces: the **economy track** and the **card-play track**. To place a
node, take the next token off either track. That uncovers the space under it, and what the space shows is yours for
the rest of the game: income, Movement, a tag, a free research station, the **keep upgrade** (tier-2 keeps cost 1
insight instead of 2), and on the last space a **capstone**, a lasting rule.

**Every placement pays two prices:** the price of the power or card that places it, **plus the energy printed on the
space** you uncover. On the economy track the spaces cost 0, 0, 0, 1, 2, 3, 5; on the card-play track 0, 0, 1, 2, 2,
3, 5. A card that places a node **free** pays neither.

**Where:** a node goes on a land hex in your reach with no node on it, no corruption marker and no land corruption. A
hex with agents or a corrupted site is allowed.

**A lost node leaves the game.** It does not return to its track, and its space stays uncovered.

### Powers

Each power can be used once a round, in the action phase.

- **Node powers.** Every board has **Deploy** (1 energy: place a node), and a dearer second one: the Builder's **Buy
  In** (1 energy and 6 credits), the Protector's **Accord** (sacrifice 1 Diplomacy, 1 energy), the Researcher's **Peer
  Review** (3 research cubes). So the powers place 0-2 nodes a round; cards can place more.
- **Conversions:** the Builder's **Mint** (3 credits: 1 chip) and **Focus** (5 credits: 1 insight), the Researcher's
  **Grant** (3 credits: 1 cube in your color on a megaproject that isn't complete) and **Synthesis** (discard a card
  from your hand: 2 insight).
- **Draws:** the Builder's **Procure** (2 chips: draw 2), the Protector's **Debrief** (sacrifice 1 Diplomacy: draw 2),
  the Researcher's **Fieldwork** (1 energy: draw 3).
- **Sacrifice powers** pay with your cards (below): the Builder's **Scrap** (1 Industry: 2 chips), **Overcharge** (1
  Power: 2 energy) and **Rush Build** (1 Industry and 1 energy: play a building from your hand without paying its
  credits); the Protector's **Stand Down** (1 Security: 3 credits), **Recovery** (1 Security: clear a corruption marker
  in reach, swaying 2 Unity) and **Shelter** (1 Security and 1 energy: in any one country, nothing can be destroyed
  this round); the Researcher's **Field Station** (1 Industry: a free research station in reach), **Audit** (1 Cyber:
  remove 3 compute from a project) and **Particle Run** (1 Power and 1 energy: 1 antimatter).
- The Protector's **Bulwark** (1 energy: harden a facility in reach, §13) and **Patrol** (1 energy: push up to 3 agents
  or troops from a hex in reach, §9).

**Sacrifice.** A power that names a tag in its cost ("1 Industry") is paid by **discarding a card in play** carrying
that tag: a program, or a building together with every building stacked under it. Events can't be sacrificed, and
neither can your board's tag. Any tags on the card beyond the cost are wasted.

### Capstones

| Board | Economy track | Card-play track |
|---|---|---|
| Builder | **Full Capacity**: 1 chip each round per 2 Industry tags on your cards | **Master Plan**: your buildings cost 2 credits less |
| Protector | **Sanctuary**: corruption never destroys your nodes | **Aegis Doctrine**: once a round, stop the first destruction (of anyone's piece) in a country where you have a node |
| Researcher | **Open Archive**: 1 research cube each round | **Open Science**: each round, 2 insight to other players, arriving next round |

*Sources: rules/ai-boards.md, rules/ai-powers.md; content/ais.json; crates/aic-core/src/modules/ai.rs,
modules/board/node.rs, modules/board/destroy.rs, modules/cards/sacrifice.rs.*

---

## 7. Cards

### Decks and kinds

There are two player decks: **tier 1**, and **tier 2**, which is stronger. A card is one of three kinds:

- **Programs** (green) resolve when played and stay in front of you. Most lasting **income** comes from programs.
- **Buildings** (blue) go into a slot on your AI board and keep working: an **Action** you may use once a round, an
  **Effect** that pays each time its trigger happens, or an income each round.
- **Events** resolve and go to your played pile. They never come back, and their tags don't count.

Every card has 1-3 **tags**: Science, Industry, Cyber, Power, Diplomacy, Security.

**Draw** always means from the tier-1 deck, with no keep to pay. When a deck runs out, shuffle its discard pile to
make a new one. There is no hand limit.

### Round 1: the opening

Each player is dealt **8 tier-1 cards** and **2 tier-2 cards**. Keep up to 4 tier-1 cards **free**; each further
tier-1 card costs **5 credits**. Each tier-2 card kept costs **1 insight**. There is no draft in round 1.

### Rounds 2 on: build, draft, keep

1. **Build a hand.** Each player takes 4 cards one at a time, choosing for each whether it comes from the tier-1 or the
   tier-2 deck. Each arrives face up, so you see it before choosing the next.
2. **Draft.** Take one card from the hand in front of you and pass the rest on, until the hands are gone. Hands pass to
   the previous player in even rounds and to the next player in odd rounds. (Solo, your hand comes back to you: you
   take all 4 cards you chose.)
3. **Keep.** Pay **1 insight** for each tier-1 card and **2 insight** for each tier-2 card you keep (1 once you have
   the keep upgrade). If you run out of insight, each further insight a keep needs costs **5 credits**. Cards you don't
   keep are discarded.

### Playing a card

In the action phase, play a card from your hand by paying:

- its **credit** cost (the octagon), less your discounts;
- its **insight** cost (the hexagon): 0, 1 or 2, from the same insight as your keeps. Credits can't stand in here;
- any extra cost printed below the art: energy, chips, antimatter or research cubes.

Its **requirement** must be met: a **yellow plate** means "at least N of this tag on your cards". A **stacked-cards
plate** means it must be built over a building of that tag. A card's own tags never count toward its own requirement.

**Discounts:**

- every 2 Science tags take 1 insight off each card you play and each tier-2 card you keep (rounds 2 on);
- every 2 Industry tags take 1 credit off your buildings;
- some cards make all your cards of one tag cheaper;
- a discount never takes a cost below 1. A cost printed at 0 stays 0.

**Nothing to act on never blocks a card.** A part of a card with nothing to act on (no target in reach, say) does
nothing, and the rest resolves. A program or event that would do nothing at all can't be played.

**Sell** a card from your hand for 1 credit at any time in the action phase.

### Tags

The tags **on your cards** are those on your programs, your buildings (covered ones too) and your AI board, printed
or revealed by a track space. Tags unlock requirements, set discounts and per-tag payouts, and pay for sacrifice powers
(§6) and some threat counters (§11). Your board's tags can't be sacrificed.

### Buildings and slots

A building goes into a slot. If a slot is free, put it there or **build over** one of your buildings; if none is free,
you must build over one. A covered building **stops working**: its income, Effect and Action stop, and any tokens on
it sit idle. **Its tags still count.** A building with a stacked-cards plate must be built over a building whose top
card has that tag, even if a slot is free.

- An **Action** (`→`: pay the left, get the right) is used once a round. An "A, OR B" Action is one use: either half.
- An **Effect** (`:`: when the left happens, get the right) fires every time, with no limit. A building counts
  itself: playing a Science building with a Science trigger fires it. An Effect on removing a troop fires for each
  troop; one on removing agents or compute fires once for each card, Action, power or standard action that removed
  any. A **rebate** Effect fires when you play a card whose printed credit cost, before discounts, is at least N.
- An **income** pays every round in upkeep.
- A building's one-time gain, printed below the art, happens when you build it.

### Tokens on cards

Some buildings hold their own tokens: **Datasets** (Science), **Followers** (Diplomacy), **Capital** (Industry) or
**Operatives** (Security). Most arrive with a few. Then they gain more in one of two ways:

- an "A, OR B" Action: add tokens here, OR spend or count the tokens here; or
- an Effect that adds tokens when something happens, with an **At N**: when the tokens here reach N, N come off and
  the reward happens.

A few cards add tokens to one of your token cards. **A card only ever counts its own tokens.** Sacrificing or
discarding a card loses its tokens.

### Help cards and teammates

A **help card** names another player: you choose one, and both of you gain, you paying. Some Effects fire when
**another player** plays a card of a tag.

*Sources: rules/player-cards.md, rules/card-grammar.md (The engine wording, Draw, Help cards); configs/baseline.json
`cards`; crates/aic-core/src/modules/cards/ (deck.rs, play.rs, price.rs, tableau.rs, triggers.rs, sacrifice.rs).*

---

## 8. The action phase

All players act at once, as often as they like and in any order. When you pass, you are done for this action phase;
it ends when everyone has passed. You may:

- **play cards** from your hand (§7);
- **use** your buildings' Actions and your board's powers, each once a round (§6);
- **pay toward threats** you are near (§11);
- take **standard actions**, as often as you can pay (below);
- use a **megacorp converter** (below);
- take the **antimatter action** once: gain 1 antimatter and sway 1 Discord.

### Standard actions

Standard actions are poor value on purpose: cards are the better route.

| Action | Cost | Effect |
|---|---|---|
| **Move** | 1 Movement | one of your ambassadors, from your supply or anywhere on the board, onto a site, base or node in reach other than the hex it leaves (§9) |
| **Claim** | 4 ambassadors for a city, 3 for a station or factory, +1 per agent on it | take a neutral site in reach (§9) |
| **Liberate** | 5 ambassadors for a city, 3 for a station or factory, +1 per agent on it | free an adversary site in reach (§9) |
| **Remove agents** | 1 ambassador from your supply | it goes onto a hex in reach with at least one agent, and removes up to 3 agents there (§9) |
| **Build a research station** | 2 chips | on an open hex in reach that isn't an industrial zone; it is yours |
| **Clear** | 2 credits | remove a corruption marker in reach; sway 2 Unity |
| **Envoy** | 1 ambassador from your supply | move 1 Discord off the Discord track |
| **Gain an ambassador** | 20 credits | into your supply |
| **Harden** | 3 credits | a hardening token on a player-owned station or factory in reach (§13) |
| **Drain** | 5 credits | remove up to 3 compute from one adversary project |
| **Research cube** | 1 research cube | gain 1 insight |
| **Sell** | a card from your hand | gain 1 credit |

### Megacorp converters

Each megaproject needs 8 cubes. When it completes (§12), each cube on it pays the payout below, and its **converter**
opens. Any player with a **node on a city in that country** may use a converter, as often as they can pay; Onyx's only
once a round each.

| Megacorp (megaproject) | Payout, each of your cubes | Converter |
|---|---|---|
| Horizon (Tidal Grid) | 3 energy, 3 credits | 1 research cube + 2 energy → 1 antimatter |
| Cintamani (Polar Compute Array) | draw 1 card, 5 credits | 1 research cube → 2 insight |
| Onyx (Mountain Fusion Works) | 4 chips, 3 credits | 1 research cube → 2 chips (once a round) |
| Biomia (Delta Seawall) | sway 1 Unity, draw 1 card | 2 research cubes → sway 1 Unity |

*Sources: rules/economy.md (standard actions), rules/board.md (Converters), rules/alignment.md (The antimatter
action); crates/aic-core/src/modules/board/standard.rs, modules/alignment.rs.*

---

## 9. Ambassadors and sites

### Arrivals: ambassadors and agents cancel

When an **agent arrives** on a hex holding a player's ambassador, the two **cancel**: both are removed (the agent back
to the adversary's supply). When an **ambassador arrives** on a hex holding agents, it cancels one of them the same
way. If several players' ambassadors are there, the earliest in player order loses theirs.

**Remove agents** is the one exception: its ambassador removes up to 3 agents and **stays**. Any agents beyond 3 stay
beside it.

### Claiming a neutral site

**Claim** a neutral site in reach by spending ambassadors from your supply: **4 for a city, 3 for a station or
factory**, plus **1 for each agent on it** (they cancel). Put your cube on it.

- **A botnet on or next to the site blocks claims.**
- Ambassadors already standing on the site don't help pay.

### Liberating an adversary site

**Liberate** an adversary site in reach: **5 ambassadors for a city, 3 for a station or factory**, plus **1 for each
agent on it**.

- **Ambassadors standing on the site pay first**: yours, then other players'. Your supply pays the rest.
- The agents on the site cancel, the adversary cube comes off, and the site turns neutral.
- Sway **5 Unity** for a city, **2** for a station or factory.
- A botnet doesn't stop a liberation, and a botnet on the site stays.

**Liberate cards** (a family of tier-2 cards) liberate a corrupted city in reach for their own price instead. They take
the adversary cube off and sway 5 Unity, but **leave the agents on the city**, so it starts dark (below).

### Blocking: one opposing piece shuts a site off

While an **agent** stands on a neutral or player site, it produces nothing; a city with an agent is **dark**. While a
**player's ambassador** stands on an adversary site, it produces nothing (a corrupted city then diverts no Discord). The
blocking piece stays. A blocked city still builds its factory each round (§12).

### Guarding

When your node, or a site you own, would be destroyed, **one of your ambassadors on that hex** is removed instead
(§13 gives the full order). An ambassador of a site's owner also guards another player's node on that hex, except when
that site is being corrupted: its owner's cube is already gone, so only the node owner's own ambassador guards.

### Spent or standing

- **Spent:** claiming, liberating, Envoy, guarding, canceling an agent, and stopping a corp troop (§13).
- **Standing:** blocking, Remove agents, and being moved.

### Moving pieces

**Move** (the standard action) takes one of your ambassadors from your supply or from anywhere on the board onto a
site, base or node in your reach.

Some cards and the Protector's Patrol move pieces on the board: ambassadors (any player's), agents and troops.
Botnets and markers never move.

- **Gather up to N:** pick a land hex in your reach; move pieces into it, each from a hex adjacent to it.
- **Push up to N:** pick a land hex in your reach; move pieces off it, each to an adjacent land hex you choose.

You may stop after any piece. Moved pieces arrive as above. No agent enters a firewall; no troop enters the Protector's
node hex.

**Gather and remove.** Some cards (the rules call this a strike) remove up to N agents, or agents and troops, from
the hexes **adjacent** to a hex you pick in reach, never from that hex itself, troops first.

**Convert.** Some late cards convert agents: up to N agents on or next to a city in your reach come off and become your
ambassadors, arriving on the hex of the node you used for reach.

**Firewall.** Some cards put a firewall on a land hex in your reach. No agent lands, is placed or is pulled there while
it stands, and the agents a Rally would land there spread over the other sites instead. Troops pass. It comes off at
the end of the next adversary turn.

*Sources: rules/ambassadors.md (all sections), rules/README.md (Site claim); crates/aic-core/src/modules/board/ambassadors.rs,
modules/board/standard.rs, modules/effects/moves.rs, modules/effects/endgame.rs, modules/effects/removal.rs.*

---

## 10. The adversary's turn

The adversary has a **board**, an **event deck**, a **project deck**, and its agents, troops and botnets. "Each board"
on an adversary card means each country. Each event card shows its **stage** (I, II or III): the stage sets which
threats it deals (§11) and, in stage I, protects your last node (§13).

The only adversary so far is **the Militarist**. Its board has three project slots and prints three rules:

- **Garrisons:** +1 agent income in each country.
- **Troops where the Protector stands:** step 6 of the turn, below.
- **Warmongering:** in each production, each military base with an adversary troop on or next to it gains 1 corp
  troop, even past its limit (§13).

**You always see what's coming.** Next round's event card is face up all round, and the pools of agents waiting to
land are public.

### The turn

1. **Event.** The face-up card becomes this round's card. If there is none, you lose.
2. **Threats.** The card's first line deals threats (§11).
3. **Rally X.** Each country's whole **pool** of agents lands on that country's sites of type X (cities, coastal cities,
   research stations, factories or botnet hexes): on the **neutral** ones if there are any, else on the players', else
   on the adversary's. A firewalled site doesn't count, so the pool spreads over the rest. Spread them evenly; the
   players choose which sites take the remainder, one each. Agents with no site to land on go back to the supply.
4. **Deploy N.** N adversary troops land in each country, on its lowest-numbered factory (any owner), else its
   lowest-numbered city, skipping hexes with the Protector's node.
5. **Other lines**, at most two, in printed order: an income raise, a new neutral factory in each country (on its
   lowest-numbered free industrial zone), a Discord sway, Unity pulled back, a blackout (cities make no energy this
   round), or "each player destroys a node on or next to a military base" (Border War: this one can take a last node
   from stage II on). Forced March gives every troop 1 more step in this round's march.
6. **Troops where the Protector stands.** In each country where the Protector has a node, 1 adversary troop lands the
   same way as Deploy.
7. **Projects.** Ongoing projects act.
8. **Botnets.** Each botnet puts 1 compute onto the projects, unless a player's ambassador stands on it.
9. **Corrupt X**, for each Corrupt line on the card, in order (below).
10. **Natural corruption.** Every city, station and factory not the adversary's that holds its corrupt number of
    agents on its own hex corrupts (a corruption marker on a city counts as an agent).
11. **March.** Adversary troops march (§13).
12. **Reveal and income.** Turn the next event card face up. Each country's pool gains its **agent income**, and the
    adversary's **compute income** goes onto its projects. Firewalls come off.

Every arriving agent cancels a player's ambassador where it lands (§9), and none lands on a firewall.

### Corruption

"**Corrupt cities**" (or stations, factories, land) works on each country in turn, its sites in hex-number order:

1. A site of that type the adversary doesn't own counts the agents **on it and next to it**: on its hex and on the
   adjacent hexes in its own country. A corruption marker on a city counts as an agent.
2. If the count reaches the **corrupt number** (city 4, station 3, factory 3, land 3), it takes its own agents and
   **just enough** from the adjacent hexes, lowest-numbered first. Those agents go back to the supply.
3. The site is **corrupted**: the adversary's cube replaces any owner cube. "Corrupt land" targets open hexes with no
   base, station or factory (a node or other pieces may stand there); such a hex gets a land-corruption token.
4. Go on to the next site. An agent used once can't count again, and every site that qualifies corrupts, so a crowded
   country can cascade.

A card may print "(1 fewer)". A firewalled site pulls nothing in, but still corrupts on what it holds.

**What corruption destroys:** the node on the corrupted hex (§13 gives what can stop that), then every player's
ambassador left on the hex. Nothing else.

**Land corruption** blocks node placement on its hex and sways 1 Discord every round.

### Projects

The adversary spends **compute** on **projects**. To deal, turn up the top project: it goes into the first empty slot
it fits (the Militarist's Military slot takes only a Military project; its two open slots take any; a Universal project
fits every slot), else into the project discard. When the deck runs out, shuffle the discard back in.

A project costs its printed number **times the number of players**. Each point of compute goes onto the **cheapest**
visible project (tie: the first slot). When a project holds its cost, it **resolves**: once, at once, or **ongoing**,
acting every round from the next. A resolved project leaves the game. When all the slots have resolved, deal new
projects. Compute with no project to fill is lost.

Compute comes from the adversary's income, botnets, adversary stations, agents on your nodes (§12), unpaid threats and
some threats' printed compute. You fight it with **Drain**, some cards, and the Researcher's Audit.

### Botnets

A botnet stays until removed. It makes compute each round, **blocks claims** on or next to its hex, and counts against
the purge (§15) until Unity eases it. Only a few tier-2 cards and one Unity card remove botnets. Removing one sways 1
Unity.

*Sources: rules/adversary.md (Each round, Income, Rally X, Corrupt X, Botnets, The project deck); content/adversary.json,
content/projects.json; crates/aic-core/src/modules/adversary.rs, modules/plan.rs.*

---

## 11. Threats

### Dealing

The event card's first line prints its **pulls**. Stage I cards pull one tier-1 threat in each country, stage II one
tier-2 (Supply Lines: one tier-1 and one tier-2), and stage III two tier-2. A pull names one or two **tags** (Military,
Influence, Cyber, Economic, Grid) or "any".

For each pull, for each country in turn, turn up the top card of that tier's threat deck. It stays if it carries a tag
the pull names (a **Universal** card has every tag) **and** the country has a place of one of its location types
(below). Otherwise discard it and turn up the next. When a threat deck runs out, shuffle its discard into a new deck.

### Location tokens

A threat prints one or two location types (city, coastal city, research station, factory, military base). It puts a
**location token** in its country on a place of each type: the one with the **most adversary pieces on or next to it**
(agents, troops, botnets; tie: the lower hex number). Cities and stations must be uncorrupted; factories must not be
the adversary's (with none, a free industrial zone); any base will do. A type the country has none of gets no token. A
threat's two tokens never share a hex.

### Near

You are **near** a threat if one of your nodes or ambassadors is on or next to one of its tokens. Only players near a
threat may pay toward it or use a card on it.

### Countering

A threat prints one or two ways to pay off. **Pay one option in full** and the threat is countered and discarded.

- **Resources** (credits, energy, insight, research cubes, chips, or ambassadors from your supply) can be pooled among
  the players near it. You may take back what you put on it during the action phase. Anything still on an unpaid threat
  comes back to its payers when threats take effect, even on a threat that waits.
- **Tags:** one player alone sacrifices cards in play carrying that many tags of the kind (2 for tier 1, 3 for tier
  2): Security for Military, Diplomacy for Influence, Cyber for Cyber, Industry for Economic, Power for Grid.

Cards can also **jam** a threat (it needs less of each resource), **delay** it (it waits a round, and can be paid
next round), or counter it outright. A few buildings counter part of every threat of a type, wherever it is.

### When threats take effect

After production, each unpaid threat does what it prints, country by country, then is discarded with its tokens. A
threat dealt after the action phase began (by a Discord card or another threat, say) waits and takes effect next
round.

**Its harm to a player always hits the nearest player:** the one whose node is closest to one of its tokens, however
far (tie: the tied players choose). Pieces it places go on its token.

- A resource harm reads "pay N or discard M cards". The player chooses; with neither possible, they pay what they have
  and discard their hand.
- A threat that destroys a node takes the nearest player's node closest to it, **never their last**.

*Sources: rules/threats.md; content/threats.json; crates/aic-core/src/modules/threats/ (mod.rs, near.rs, counter.rs).*

---

## 12. Production and upkeep

Production reads the board as each step starts.

1. **Corp troops.** Warmongering (§10) and some projects add extra corp troops to bases (§13).
2. **Factories.**
   - A player's factory makes 2 chips for its owner.
   - An adversary factory makes 1 adversary troop on its own hex (Wall doesn't stop it).
   - A neutral factory makes nothing.
3. **Cities build factories.** Each city builds one factory, country by country in hex-number order, on a **free**
   industrial zone (one with no factory). A blocked city still builds. A city with no free zone it can use builds
   nothing.
   - A clean or aligned city builds on a free zone adjacent to it (an aligned city's owner chooses which; otherwise
     the players do). An aligned city's factory is its owner's. A clean city's goes to the player with the most nodes
     on or next to the zone (tie: the tied players choose); with none, it is neutral.
   - A corrupted city builds anywhere in its own country: the lowest-numbered free zone with an agent on or next to
     it, else the lowest-numbered free zone. The factory is the adversary's, and any corruption marker on that zone
     comes off.
4. **Cities' energy.** Each city makes 1 energy.
   - Aligned: to its owner.
   - Clean: one player with a node on or next to it may buy it for 5 credits (the players choose who); otherwise it is
     lost.
   - Corrupted: it becomes 1 Discord.
5. **Stations.**
   - A player's makes 2 cubes in its owner's color; a neutral one makes 1 neutral cube. They go onto their country's
     megaproject until it is complete. After that, a player's cubes go onto their board as **research cubes**, and a
     neutral station's are lost.
   - An adversary station makes 1 compute.
6. **Your nodes.** Each hex where your node stands with no site or base, and at least one agent, costs you 1 insight
   next round and gives the adversary 1 compute.
7. **Megaprojects.** A megaproject holding its 8 cubes or more **completes**, and **every cube on it pays**, overflow
   included: each of your cubes pays you the card's payout (§8); neutral cubes pay nothing. Its converter opens.
8. **Upkeep.** Each player collects their income: board, track spaces, programs and buildings.

**A blocking piece stops a site's output** (§9): an agent on a neutral or player site, a player's ambassador on an
adversary site.

*Sources: rules/war.md (Factories), rules/economy.md (Cities' energy, The round), rules/board.md (Research cubes and
the corps), rules/ambassadors.md (One site model); crates/aic-core/src/modules/board/production.rs,
modules/economy.rs.*

---

## 13. Troops, war and destruction

There are two kinds of troop. **Adversary troops** march across the board and raid your facilities. **Corp troops**
wait at the military bases and fight each other across the fronts.

### Adversary troops: the march

An adversary troop's **targets** are player-owned research stations and factories. Never cities, nodes, or neutral or
adversary facilities.

In the adversary's turn, **each adversary troop takes 2 steps**, one troop at a time, country by country in hex-number
order. Each step is one of:

- **strike** a target on or next to its hex (the players choose which, if several). The target is destroyed, unless
  something stops it (below). Either way the troop is used up;
- if there is none, **move** one hex toward the nearest target, walking over land around Wall hexes. The players choose
  among equally good hexes. With no way to any target, it stays.

So a troop two hexes from your facility hits it this round. You can always work out a troop's next target: the
nearest player-owned station or factory.

### Corp troops and war

Each base holds up to **2** corp troops.

- **Whenever a corp makes troops, it puts 1 on each of its bases** that has room.
- **A neutral factory makes corp troops each time the Discord track tips** (§14), unless an agent stands on it.
- **A player's ambassador on a base** is used up instead of the 1 troop that base would get. **One on that neutral
  factory** is used up instead of all of the factory's troops at that tip (2 at 3-4 countries). A full base makes no
  troop and uses up no ambassador. Several players' ambassadors: the earliest in player order goes first.
- Warmongering (§10) and the Arms Race project add 1 troop to a base past its limit (at most 1 a base a round). The
  War Footing project fills every base.

**War**, at the end of every round, on each front:

1. The two bases' piles cancel 1 for 1, and both bases are emptied.
2. Each troop left over crosses and strikes the rival corp's factory nearest its base, counting hexes (tie: the lower
   number). It can hit a neutral or player factory, never an adversary one.
3. The factory can be saved like any destruction (below). If it is destroyed, it sways **1 Discord**, and **any node
   on its hex goes with it**: no Shelter, hardening, ambassador or Aegis Doctrine saves that node, only the stage-I
   last-node rule.
4. A troop whose strike is stopped is still used up, and the next troop aims at the same factory.

A corp troop at a base also protects facilities on or next to its hex from adversary troops' strikes: it is removed
instead.

### Destruction

When a node or facility would be destroyed, the first of these that applies stops it (and is used up, if a piece):

1. **Last node.** While this round's event card is stage I, no player's last node can be destroyed.
2. **Shelter** on the country this round.
3. A **corp troop** at a base on or next to the hex (against an adversary troop's strike only).
4. A **hardening token** on the hex.
5. **Its owner's ambassador** on the hex (for a node: the node owner's, or else the ambassador of the player who owns
   the site there).
6. **Aegis Doctrine**, once a round (§6).

Corruption never destroys the node of a player with **Sanctuary** (§6), and spends nothing of theirs. A destroyed
station or factory takes its owner cube with it.

**Hardening** goes on a player-owned, uncorrupted station or factory. A hardening token on a hex with your node is
called a **shield**.

### Losing nodes

Nodes are destroyed only where the board is overrun: by corruption of their own hex, by war, by threats and by event
lines that print it (Border War). Nothing takes a player's last node while the event card is stage I; a threat never
does, at any stage; war and Border War can from stage II on.

A destroyed node **leaves the game** and leaves a **corruption marker** on its hex, unless the hex already holds a
marker, a botnet or an adversary cube. No node can be placed on a marked hex. **Clear** removes one (§8).

*Sources: rules/war.md, rules/ai-powers.md (Shelters, hardening and shields); crates/aic-core/src/modules/board/troops.rs,
modules/board/destroy.rs, modules/board/node.rs, modules/board/production.rs.*

---

## 14. Unity and Discord

The world leans toward **Unity** (you) or **Discord** (the adversary). Each side has its own **track**, its own **tip
count** and its own deck of one-shot cards. Nothing on one side empties the other.

### Sway

Effects **sway** Unity or Discord: each point goes onto that side's track.

- **Unity** comes from liberating sites (5 for a city, 2 for a station or factory), clearing markers (2), removing
  botnets (1), Biomia's converter, and cards.
- **Discord** comes from corrupted cities (1 each a round), factories destroyed in war (1 each), the antimatter action
  (1), land corruption (1 a round each), and threats, projects and cards that print it.

**Holding Discord off.** Unity swayed during an action or step waits beside its track until that action or step is
done. Then the players may use some or all of it to remove Discord from the Discord track instead, 1 for 1, but only
if that leaves the Discord track below its tipping point. The rest goes onto the Unity track. **Envoy** removes 1
Discord for 1 ambassador.

### Tipping

A track **tips** when it holds **3 × the number of players**. Then:

1. Take one tip's worth off. Unity keeps any extra toward the next tip; **Discord empties**.
2. Add 1 to that side's tip count.
3. **Discord only:** each neutral factory makes corp troops (§13).
4. Draw that side's top card and resolve it at the tier its tip count shows: **tier I** on tips 1-2, **tier II** on
   tips 3-5, **tier III** from tip 6.

If both tracks are full, Discord tips first. A card that fills a track again tips it again.

### The cards

Every Unity and Discord card prints three tiers. Each acts on the board by a condition ("for each site a player owns",
"on each hex next to an ambassador"), never by one player's reach. Some Unity cards **forbid** the adversary's next
Rally, Deploy or Corrupt where a condition holds: that step skips each site where the condition holds when the step
runs, and what it would have placed stays in the supply. Set your pieces before its turn. Discord cards never destroy a
node directly, but a city their Corrupt line corrupts destroys any node on it (§13).

- **The Unity deck** (23 cards) reshuffles when it runs out.
- **The Discord deck** (6 cards) **never reshuffles: it is the clock.** A Discord tip with no card left loses the game.

*Sources: rules/alignment.md; configs/baseline.json (`tip_per_player` 3, `carry` 1, `tier_at` 3, 6); content/alignment.json;
crates/aic-core/src/modules/alignment.rs, modules/board/production.rs (the tip's corp troops).*

---

## 15. Winning and losing

### Winning: the purge

You win the moment the board holds none of what the purge asks for. It asks for less as Unity tips:

| Unity tips | The adversary must have none of these on the board |
|---|---|
| 0-2 | corrupted cities, botnets, adversary troops, agents |
| 3-5 | corrupted cities, botnets, adversary troops |
| 6 or more | corrupted cities |

Corrupted stations, factories and land never count, nor do corp troops.

### Losing

Everyone loses at once if:

- a Discord tip finds the Discord deck empty;
- any player has no nodes on the board;
- a round's event step finds no card to turn up (the **round cap**: 11 rounds at difficulty 0-2, 9 at 3-4).

The adversary running out of agents is not a loss: a placement it can't make doesn't happen.

*Sources: rules/alignment.md (The purge, Losing), rules/adversary.md (The event deck); configs/baseline.json (`purge_at`);
crates/aic-core/src/modules/alignment.rs, modules/adversary.rs.*

---

## 16. Difficulty

Choose a level from 0 to 4. Your claim and liberate costs are the same at every level.

| Level | Changes |
|---|---|
| 0 | the base game |
| 1 | Cities, stations and factories corrupt with 1 fewer agent (3, 2, 2; land stays 3). Threat counters cost more: +1 on each non-credit item, +4 on a credits item. |
| 2 | As level 1, and counters cost +1 more on each non-credit item. Each Deploy line lands 1 more troop in each country. The adversary gains 4 compute for each unpaid threat. |
| 3 | As level 2, with 2 fewer stage-II event cards (9 rounds). |
| 4 | As level 3, and: a stage-I event card swapped for a stage-III card; one Discord card removed (it counts as a tip already made, so the tiers come a tip sooner); 1 extra threat each round, countries in turn; the last stage-I card's tier-1 pulls deal tier 2. |

The adversary's incomes are set per level:

| Level | Compute income | Agent income (each country) | Event cards (I / II / III) | Rounds |
|---|---|---|---|---|
| 0 | 3 | 1 | 4 / 4 / 3 | 11 |
| 1 | 4 | 4 | 4 / 4 / 3 | 11 |
| 2 | 18 | 4 | 4 / 4 / 3 | 11 |
| 3 | 24 | 4 | 4 / 2 / 3 | 9 |
| 4 | 13 | 3 | 3 / 2 / 4 | 9 |

*Sources: rules/adversary.md (Difficulty, The event deck), rules/threats.md (Countering); configs/baseline.json
`adversary` (`compute_income`, `ambassador_income`, `counter_bonus_*`, `counter_more_*`); content/adversary.json
(Garrisons, the difficulty line); crates/aic-core/src/modules/adversary.rs, modules/threats/counter.rs.*

---

## 17. Player count and solo play

- **Countries:** one per player, 1-4. The layout and its fronts depend on the count: 2 countries share one front; 3
  countries, three; 4 countries form a ring of four.
- **Boards:** Builder, Protector, Researcher in player order; a fourth player takes a second Builder. With 2 players there
  is no Researcher; solo, there is only the Builder.
- **Tipping:** both tracks tip at 3 × the number of players.
- **Projects** cost their printed number times the number of players.
- **Corp troops:** a corp has a base on each of its fronts, so at 3 or 4 countries each neutral factory makes 2 troops
  at a Discord tip.
- **Solo (1 player):** one country, so no fronts, no bases and no war. You draft alone: your hand comes back to you,
  so you take all 4 cards you chose. "Another player" on a card means you. An Effect that fires when another player
  plays a card fires on your own plays, and a shared gain is paid once.
- **Team choices** (where a rally's remainder lands, how much Unity holds Discord off, a choice on a Unity or Discord
  card) are made by the players together.

*Sources: rules/board.md (fronts), rules/ai-boards.md (`ai.boards`), rules/war.md, rules/alignment.md (Tipping),
rules/adversary.md (projects), rules/card-grammar.md (Help cards, Teammate triggers).*

---

## 18. Reading a card: the icon legend

A card is read like a sentence: a few **header slots** at the top, and small icons inside ordinary text for pieces,
resources and tags. Plain words do the rest, and the text always says what the icons say, in the same order.

### Where things sit

- **Top strip:** the credit cost (an octagon), the insight cost (a hexagon), requirement plates and the card's tags.
- **Above the art** (buildings only): what lasts while the building is uncovered: income, Effects, Actions.
- **Below the art:** what happens once, when you play the card: extra costs first, then gains, placements and moves.

### The icons

The shape says the kind of thing; the picture inside says which. (The art is not final.)

| Kind | Icons |
|---|---|
| Resources (square tiles) | credits (a coin, gold), energy (a lightning bolt, orange), insight (an eye, purple), chips (a chip, green), antimatter (an atom, magenta), research cubes (stacked cubes, teal), cards (two cards, gray); the adversary's compute (a grid, blue) |
| Tokens on cards (discs) | Datasets (a database), Followers (two people), Capital (a bank), Operatives (a target) |
| Pieces (bare silhouettes) | ambassador (a figure), node (a house shape), troop, botnet (a starburst), cube, corruption marker |
| Places | city (a skyline), research station (a dome with a mast), factory (a sawtooth roof), a hex outline, a country (a cluster of hexes), a megaproject (a card with a cube), an adversary project (a plain card) |
| Tags (ringed circles) | Science, Industry, Cyber, Power, Diplomacy, Security, each its own symbol in a ring |

### Shapes

| Mark | Means |
|---|---|
| Square tile | a resource (cards included) |
| Disc | a token on a card; alone, on this card ("here") |
| Bare silhouette | a piece on the board, or moving one |
| Ringed circle | a tag; an empty ring is a tag of any kind |
| Number before an icon | how many (no number: 1) |
| Minus before an icon | pay or lose it now |
| `≤`N | up to N, your choice |
| `≥`N | at least N (a rebate's printed credit cost) |
| `/` | per |
| OR, ALL, ACT | written out; ACT is an Action granted |

### Boxes and outlines

| Mark | Means |
|---|---|
| No box | once, now; you, or this card |
| Brown box above the art | income, each round |
| Brown box below the art | raise your income once (with a minus: lower it) |
| Dark frame | acts on the adversary: its pieces, projects or threats. A number with a minus in it is "remove up to N" (on a jam: "less to pay"). |
| Ambassador silhouette in the dark frame | an **agent** (outside the frame it is an ambassador) |
| A place glyph after a piece | where: after a cube, only that kind of site; after a botnet or ambassadors, on or next to that place; a country glyph, in one country |
| Dashed outline | another player |
| Thin solid outline | one of your cards: a disc in it goes to (or comes from) one of your cards |

### Connectors

| Mark | Means |
|---|---|
| `→` | an Action: pay the left, get the right, once a round. A bare `→` is free. |
| OR at a line's start | the other half of the Action above: one use a round, either half |
| `:` | an Effect: when the left happens, get the right. A minus after it: those cards cost that much less. |
| Discs left of `:` | At N: when the tokens here reach N, they come off and the right happens |

### Special marks

| Mark | Means |
|---|---|
| Play (a card with an arrow onto it) | a card you play, with its tag if any; in a dashed outline, another player's |
| Node | place a node, paying its track space |
| Node with a struck-through coin | place a node free |
| Shield | stops one destruction: alone, a hardening token on a facility; with a node, a shield; with a country, Aegis Doctrine |
| Shelter (a roof) | a whole country, this round |
| Threat (`!`) | a threat; struck through: counter it |
| Jam (a threat with a down arrow) | a threat needs that much less of each resource |
| Clock | after a threat: delay (the threat waits a round). After an agent and a hex: a firewall; after an agent and a country: a cordon (no agents land there through next round) |
| Requirement plates | yellow: at least; stacked cards: built over. (A red "at most" plate is drawn for, but no card has one.) |

*Sources: docs/card-guidelines.md (Legend), rules/card-grammar.md (Boxes, Connectors, Shapes);
crates/aic-core/src/face.rs.*

---

## 19. Glossary

- **Adjacent:** sharing an edge. **Next to X:** on X, or adjacent to it.
- **Agent:** the adversary's piece. **Ambassador:** a player's piece, any player's.
- **Aligned / clean / corrupted / neutral / owned:** §2, *Sites*.
- **At N:** §7, *Tokens on cards*.
- **Base (military base):** a megacorp's pile of corp troops on a front, §2 and §13.
- **Building:** a card. **Facility:** a research station or factory on the board.
- **Compute:** what the adversary spends on projects, §10.
- **Corrupt number:** how many agents corrupt a site, §10.
- **A country you're in:** a country where you have a node. **Your home country:** the one you start in.
- **Dark:** a city with an agent on it, §9.
- **Draw:** from the tier-1 deck, with no keep to pay.
- **Firewall:** §9. **Front:** §2.
- **Here:** on this card.
- **In reach:** on or next to one of your nodes.
- **Land:** a city or open hex.
- **Megaproject:** a megacorp's 8-cube card, §8 and §12. **Open megaproject:** one that isn't complete.
- **Movement:** your free moves each round, §6.
- **Near (a threat):** your node or ambassador on or next to one of its tokens.
- **On your cards:** on your programs, your buildings (covered ones too) and your board.
- **Player order:** Builder, Protector, Researcher, second Builder, §3.
- **Pool:** the agents waiting to rally in a country, §10.
- **Purge:** the win, §15.
- **Remove up to N:** as many as there are, up to N.
- **Shield:** a hardening token on a hex with your node.
- **Site:** a city, research station or factory.
- **Stage:** I, II or III, printed on each event card, §10.
- **Sway, tip:** §14.
- **Tier:** player cards and threats come in tier 1 and tier 2 (decks); Unity and Discord cards resolve at tier I, II
  or III (by tip count).
- **Troop:** on a card, an adversary troop, unless it says corp troop, §13.

*Sources: rules/README.md (Terms), docs/player-aid.md.*
