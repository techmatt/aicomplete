# 014 — the new board model

> **Frozen design input (016).** Matt's 014 board prompt, verbatim as queued
> (conductor item `014_board_model`, its front matter dropped). It records the design
> as handed over: *Locked* and *Candidate* are as of 014, and later items have changed
> some of it. What the code implements is [docs/rules/](../../../docs/rules/README.md); this file is not
> updated.

## Goal

Replace the board with Matt's redesigned model: cities, research stations, factories, military bases and data centers, with ambassadors as the adversary's main pressure. When done, Matt can play a full game in the viewer on the new board, to a win or a loss.

**Locked** marks Matt's decisions. **Candidate** marks starting points: make each one a tunable or a variant, never hard code. Every number below is a starting calibration reasoned for an 8–10 round game; simulation adjusts it later.

This is prompt A of three. Cards (B) and AI boards (C) come after, so carry today's cards over as placeholders and keep the card economy as it is.

## Remove

Population, processors, drones, outposts, foundries, uplinks, typed land (Urban, Industrial, Network, Frontier), defense tokens, and anything opposition left behind. Bump `RULES_REVISION`, and drop old configs and records, as 013 did. Keep the Alignment module (013) as it is.

## Board

- **Locked:** population is implicit, and people live in cities. Each country is still a 16-hex piece.
- **Candidate hex types:** city, military base, open and ocean. Buildings (factory, research station, data center) go on open hexes, one per hex.
- **Candidate adjacency rules:** a factory must touch a city; a data center must touch one of your own. Nothing builds on ocean, and an ocean edge between two countries is not a front.
- **Owner cubes, locked:** cities and research stations carry one. A city or station with no cube is clean; an adversary cube means corrupted; a player cube means aligned or owned.
- **Setup per country (candidate):**
  - 3 cities, 1 of them corrupted;
  - 1 research station, corrupted;
  - 2 factories, each touching a city;
  - 1 military base on each land border with another country;
  - 0–2 ocean hexes on the outer edge, to vary the pieces.

  Design the layouts so every country has at least one front at 2–3 players.
- **Each player starts with** 2 data centers, 1 player ambassador, 25 credits, and their AI board's credit production. Credit production starts at about 8 (candidate).

## Production and economy

- **Cities, locked:** each clean city makes 1 energy a round; a corrupted one makes it for the adversary.
  - Candidate: a clean city's energy goes to the player with the most data centers next to it, players choosing on a tie.
  - Candidate: an aligned city gives its owner +1 more.
  - Candidate: aligning a clean city costs 2 energy and 1 player ambassador.
- **Data centers, locked:** player presence. Each makes 1 compute. You act within 1 hex of your data centers.
  - Placing one takes it from your AI board's track, uncovering values the way processors did. Map the old processor tracks onto data centers.
- **Research stations, locked:** each makes one megaproject cube a round in its owner's color, onto its country's megaproject card. Neutral stations make nothing.
  - A player ambassador turns a neutral station into an owned one.
  - When a project completes, each cube on it pays its owner. Candidates: 10 credits a cube to players; adversary cubes become Discord, at 1 Sway per 2 cubes.
  - Afterward, owned stations keep producing the benefit printed on that project.
  - Candidate: one placeholder project per corp, 7 cubes. Reasoning: about 2 stations a country make 1–2 cubes a round, so a project completes around round 4–5.
- **Anything diverted to the adversary** (energy, compute) sways Discord, 1 per unit, unless it has a direct use like megaproject cubes. Candidate.
- **Credits, locked shape:**
  - credit production on each AI board, raised by cards;
  - resources act as discounts at AI-specific rates: compute for Cyber cards, energy for buildings;
  - standard conversions, including selling a card for 1 credit;
  - each AI's money source is part of its flavor.

  For A, wire the mechanisms in and keep today's numbers where they exist.
- **Standard actions, locked:** they exist but are deliberately poorly priced. Cards are always the better way, and standard actions are a luck hedge. Candidate prices, at roughly double a card's rate:

  | Action | Cost |
  |---|---|
  | Place a data center | 2 energy + 4 credits |
  | Build a factory or research station | 2 energy + 8 credits |
  | Remove an adversary ambassador within reach | 2 energy |
  | Move a player ambassador | 1 energy |
  | Gain a player ambassador | 6 credits |
  | Harden a building (Protector only) | 3 credits a token |
  | Sell a card | gain 1 credit |

## Ambassadors

**Locked rule:** an ambassador sits on a piece. The next time that piece does its normal thing, the ambassador redirects it to its own side and is removed.

**Adversary ambassadors** (they spawn and never move by themselves):
- On a factory, when it builds a unit: the unit is adversary-owned. Locked.
- On a military base, when the base produces: 3 Discord, and the ambassador is consumed. It does not start a war. Locked; the 3 is a candidate.
- On a city: that round's energy goes to the adversary. Candidate.
- On a station: that round's cube is adversary-colored. Candidate.
- On a data center: its compute goes to the adversary. Candidate.

**Player ambassadors** (they move only through cards and the standard action):
- Turn a neutral station into an owned one. Locked.
- On a military base: 2 Unity when the base produces. Locked; the 2 is a candidate.
- On a factory: no unit is built, and 1 Unity. Candidate.
- 3 on a corrupted city free it, with its cube returned to the supply. Locked; the 3 is a candidate.
- Opposing ambassadors on one piece cancel each other, with no Unity gained. Locked.
- Defense: a player ambassador on a building is consumed instead of the building when it would be destroyed. Candidate.

## Adversary

**Each round** (candidates):
1. **Build:** each corrupted city gains 2 adversary ambassadors, and each factory next to a corrupted city gains 1.
2. **Spread:** every hex with several adversary ambassadors keeps one and moves the rest to empty non-ocean hexes, players choosing. Priority: player-owned hexes, then clean cities, then empty hexes. Uncorrupted cities never spread; they hold their ambassadors.
3. **Corrupt:** a clean city holding 5 adversary ambassadors becomes corrupted, and they return to supply.
4. **Overflow** is a variant, off by default: an ambassador placed where one already sits corrupts the hex instead.

Calibration: one corrupted city plus its factories makes about 2–3 ambassadors a round per country. If about half reach a clean city and players remove some, the first new corruption comes around rounds 5–6, and the supply then doubles.

**The adversary deck, locked:** it is the clock and the stager, and it flavors the adversary.
- About 10 cards in stages I/II/III (3/4/3). The next round's card is face up a round ahead, as Spirit Island's pipeline telegraphed.
- Each card adds a twist to that round's build or spread, for example "build also targets factories next to clean cities". Stage II carries the Militarist's escalation.
- Draft the cards as candidates, and list them in the report.

**War** is on the round counter, not a card. Candidate rounds: 2, 4, 6, 7, 8 and every round after.

**Corp troops, locked:**
- There are no player troops. Troops belong to the megacorp of their territory, and they fight on fronts.
- Each base produces 1 unit a round.
- **On a war round,** the units on either side of a front cancel 1-for-1. The surplus crosses, acts at once and is used up: each destroys the rival factory nearest that base, or a data center if there is no factory.
- **At peace,** units wait at the base, capped at 3. This is open with Matt; it's a placeholder.

**Adversary troops,** candidate: they come from ambassadors on factories. They stay on the board, and in production each destroys one adjacent player building (players choose; hardening absorbs first).

Every destruction, from either kind of troop, sways 1 Discord.

**Threats:** keep the threat row and counters. Remap today's threats to the new pieces using this menu of failure costs:
- ambassadors onto player buildings or clean cities;
- an adversary troop;
- a station flipped to the adversary;
- credits lost;
- a blackout;
- discarding cards;
- losing an ambassador;
- Discord.

Prefer choice threats ("lose 4 credits or discard 2 cards"). Permanent losses only in tier 2.

**Difficulty 0–4:** remap the ladder to this model, for example larger builds, a second corrupted city, a smaller supply, tier 2 sooner. Say what each level adds.

## Round order

Locked for now:
1. **Adversary:** build, spread, corrupt, reveal threats and the next deck card.
2. **Draft,** unchanged in A.
3. **Actions (simultaneous):** cards, standard actions, countering threats.
4. **Production,** in this order:
   1. corps produce at bases (ambassadors divert units);
   2. front battles on war rounds;
   3. adversary troops strike;
   4. cities, stations and data centers produce (ambassadors divert);
   5. credit income;
   6. uncountered threats execute.
5. **Upkeep:** tips and the checks below.

## Win and loss

Placeholders.

**Win:** no corrupted cities, no adversary ambassadors and no adversary troops on the board. Unity tips ease this bar:
- after 2 Unity tips, ambassadors no longer count;
- after 5, troops no longer count either.

8 Unity tips win outright. That should be rare.

**Loss:**
- every city is corrupted;
- the adversary deck runs out;
- the adversary's ambassador supply runs out, at 10 per country.

## Also

- **Placeholders:** carry today's cards, AI unique cards and AI boards over. Remap anything that names a removed piece, or cut it, and list each change in the report. Builder and Protector keep their identities: data centers replace processors on their tracks, and growth options are remapped (a processor becomes a data center; defense becomes hardening or an ambassador).
- **Agents:** greedy and engine learn the new board on the `--quick` path. Deep agents only need to keep working.
- **Viewer:** shows hex types, owner cubes, buildings, both kinds of ambassador, corp and adversary troops, fronts and war rounds, megaproject cards with their cubes, the face-up deck card, and the supply.
- **Quick screen:** run `--quick` at 2 and 3 countries, d0 and d4. Report:
  - the round of the first new corruption;
  - adversary ambassadors on the board each round;
  - cities corrupted at the end;
  - destructions per war round;
  - Unity and Discord tips;
  - win and loss rates, and causes;
  - how often standard actions are used (it should be modest).

  Content changes are this prompt's own work, so no patches are needed. Report what looks off; don't tune.
- **Parked edit:** add to `docs/principles.md`: "Calibrate by reasoning first: before simulating, think through when each decision, card and number has a place to matter in an 8–10 round game, and set its starting value so it does; simulation then adjusts and evaluates."

**Done when:** a game in the viewer on the new board plays through to a win or a loss, and the quick screen reports the numbers above.

Commit when done.
