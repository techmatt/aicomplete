# Teaching AI Complete in ten minutes

*A spoken teach for new players, of the game as built (the revision the [rulebook](rulebook.md) names). Read it aloud with the map, a board and
the adversary's board on the table. Detail: the [rulebook](rulebook.md) (its § numbers are given below), the
[player aid](player-aid.md) and [rules/](rules/README.md).*

## What you are

You are good AIs, working together against one evil AI: the adversary. It sends **agents** onto the world's cities,
research stations and factories. When enough agents crowd onto one, it is **corrupted**: the adversary's. You spread
**nodes** over the map, send **ambassadors** onto corrupted places to take them back, and build a tableau of cards
that pays for it all. Everyone wins or loses together.

## How you win and lose

**You win the moment no city on the map is corrupted.** Each country starts with two corrupted cities. Take every
corrupted city back and you win, in any round. Nothing else counts: not botnets, not agents, not troops.

**You lose** if any one of four clocks runs out (§14):

- the Discord deck: seven cards, never reshuffled; the eighth time Discord tips, you lose;
- your nodes: if any player has no node left on the map, everyone loses;
- the adversary's event deck: one card a round, 13 rounds at most on the easy levels, 11 on the hard ones;
- the adversary's research row: when its marker reaches the seventh slot, you lose.

Games are meant to run about seven to ten rounds.

## The map

One country per player, each a tile of 16 numbered hexes. A hex is a city, open land or ocean. Cities, research
stations and factories are **sites**. A site is neutral, a player's, or corrupted, shown by the cube on it. **No player
ever owns a city**: a city is its country's or the adversary's. Each factory sits on a zone with an arrow to its city.
When the rules break a tie, the lower-numbered hex goes first.

Two words you'll hear all game: **next to** means sharing an edge, and **in your reach** means on or next to one of
your nodes. Anything you target on the map must be in your reach.

## A round, in order

1. **Sway.** Each Fallout token on the map adds a Discord.
2. **The adversary's turn**, two beats: its **Card** beat, then its **Income** beat. More in a minute.
3. **Cards.** Build a hand, draft, keep.
4. **Actions.** Everyone at once, in any order, until everyone passes.
5. **Production.** Your factories, cities, stations and income pay.
6. **Threats.** Every threat nobody paid off takes effect.
7. **Cleanup.** This round's effects end; stand your ambassadors back up.

Notice the adversary acts before you do, and its next card is already face up: you always see its next move coming.

## Your board

You're the **Builder**, the **Protector** or the **Researcher**. Each board has a special rule. The Builder has five
building slots instead of four. The Protector's **Wall**: no troop is ever placed on, moves onto or strikes a hex with
its node. The Researcher's **Patience**: unspent insight carries over.

Your nodes sit on two tracks on your board. To place a node, take the next one off either track: the space you
uncover is yours for good (income, more insight, a tag, and at the end a capstone). **Every placement pays the energy
printed on that space.** Nodes come only from your three **node powers** and from cards: one cheap power that's free,
and two costly ones with a bonus. Each works once a round, and you may use all three in one round.

Your **insight** is the number on the rightmost card-play space you've uncovered, 3 to start. It pays for keeping and
playing cards, and what you don't spend is gone.

Energy comes from cities: **every free city with your node on or next to it pays you 1 energy a round**, and pays every
other player beside it too. One agent on a city shuts it off for everyone, so clear agents early.

## Ambassadors and Liberate

**Each of your ambassadors on the map does one thing a round**, then tips over until cleanup: it **moves** to a hex
next to it or to any of your nodes, or it **acts** where it stands. In the action phase, ambassadors from your supply can
come onto your nodes, free, and arrive standing, ready to go.

Taking a site back is **move in, then act**. Move an ambassador onto a corrupted site. While it stands there, the site
makes nothing for the adversary. Next round it stands up and acts: **Liberate**. Pay in ambassadors: 5 for a city, 4
for a station, 3 for a factory, the ones on the site first, then your supply. The adversary's cube comes off, the site
is neutral again, and you gain Unity. You need no reach for this: your ambassador carries it.

Two things stop a Liberate: **an agent on the site**, so clear it first, and **a troop on or next to it**, so remove or
push the troop first.

When an agent lands on your ambassador, or your ambassador walks onto an agent, both are removed: one for one.

## The adversary's turn

**Card beat.** Last round's card slides to the side; the face-up card becomes this round's. Then:

- it **deals threats**, one or two in each country;
- **Escalate**: every site of the kind last round's card named that still holds an agent takes one more;
- the Militarist's own rule: every factory holding an agent takes one more;
- **Infiltrate**: one agent onto every site of the kind it names that isn't the adversary's yet, in every country;
- a **Strike** or **Redeploy** icon, if it shows one, for the troops;
- at most one other line: compute, a new neutral factory or station, botnets spreading, a blackout or Fallout.

Agents corrupt a site **the moment they land** and reach its number: a city at 5 (3 from level 1), a station or factory
at 3. Two agents landing on your node's hex destroy the node instead. Only the hex they land on is checked.

**Income beat.** The next card turns face up. Then **each corrupted site makes one thing**: a city makes a Discord; a
station puts a cube on the adversary's research row; a factory pushes its agents along its arrow onto its city, then,
if empty, makes a new one; a botnet makes compute. Your ambassador standing on any of them stops it.

All the compute goes onto the leftmost **project** on the adversary's board. When a project fills, it fires once, in
every country: about one a round. The pile of waiting compute is public: that's your warning.

The **research row** advances a slot for every 3 cubes a country (2 from level 2), and each slot's icon happens once: a new factory,
more infiltration, more compute, then **troops**, then a Strike, then more troops. Slot seven loses the game.
Stop its cubes by standing on, or liberating, corrupted stations.

## Troops

Troops come late, from the research row, a few projects and one Discord card. They never wander. They do three things:

- **garrison**: a new troop goes on its country's lowest-numbered corrupted factory, else its corrupted city;
- **guard**: a corrupted site with a troop on or next to it can't be liberated;
- **Strike**: on the icon, every troop destroys one node, station or factory of yours on or next to it. The troop
  stays to strike again.

A rare **Redeploy** icon picks up each troop and puts it on its country's corrupted city nearest one of your
pieces: the garrisons come to where you are working.

An event card's Strike icon is face up a full round before it happens. Use that round. A troop next to one of your
ambassadors is in your reach, so walking an ambassador up to a guarded city lets you remove its guard.

## Threats

Each threat sits on one or two sites in a country. If your node or ambassador is on or next to one of its tokens,
you're **near** it and may help pay it off: one way in full, resources pooled between you, or one player sacrificing
cards with its tag. Unpaid at the end of the round, it does what it prints to the player whose home country it's in,
and lands one more agent on its spot.

## Cards and the draft

At the start you get two tier-1 and two tier-2 cards, free. Every round, each player builds a hand of four, one card at
a time from either deck. Then draft: take one, pass the rest to the next player, until they're gone. Keep what you
drafted for insight: 1 for a tier-1 card, 3 for a tier-2.

**Programs** stay in front of you and are where lasting income comes from. **Buildings** go in your slots: an Action
once a round, or an Effect that pays when its trigger happens. **Events** are used once. Every card shows tags: they
unlock other cards and pay for your board's sacrifice powers. Draw is precious: there's no limit on plays, only on
cards.

## Unity and Discord

Two separate tracks. Liberating sways **Unity**: 5 for a city, 2 for a station or factory. Corrupted cities, Fallout,
threats and projects sway **Discord**. When a track fills, it **tips**: draw that side's card. Unity cards help, a
little more on later tips. Discord cards hurt, and there are only seven: the eighth Discord tip loses. **Unity never
cancels Discord**; only a few Unity cards take Discord off its track.

## Three tips

1. **Read the face-up card every round.** It tells you which sites take agents and whether troops strike. Clear the
   agent that would escalate; move pieces out of a Strike's reach.
2. **Keep a second node on the map**, and never let one sit alone next to agents. Losing your last node loses the
   game for everyone.
3. **Liberate early and together.** Pool ambassadors toward the cities, stand on corrupted factories to stop their
   agents, and deal with a troop before it garrisons next to the last cities you need.
