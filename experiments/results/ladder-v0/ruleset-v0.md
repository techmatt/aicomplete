# First ruleset (v0): the design draft

> **Frozen snapshot. Not the current rules.** This is the v0 design draft exactly as
> it was handed over, and it is never updated: not for defaults the engine chose, not
> for deviations, not for later rules changes. **[docs/rules.md](rules.md) is what the
> code implements.** Read this file only to check rules.md against the original draft.

The ruleset as handed over for implementation (queue item 004), kept verbatim so the
defaults and deviations in [rules.md](rules.md) can be checked against it. It is a
draft to mark up; **rules.md, not this file, says what the code does.**

---

A rough, complete ruleset for the first playable game: Militarist against Builder and Protector, 1–4 countries. It is a draft to mark up. Every number here becomes a named tunable in the engine, so the tuning loop can move it.

## 1. Board and setup

**Terrain.** Every hex has one of four terrains, 4 of each per country: **Urban** (where people live), **Industrial**, **Network** and **Frontier**. Adversary cards name a terrain, the way Spirit Island's name a land type. Two Frontier hexes on each country's outer edge are printed **uplinks**: where the adversary enters.

**Each country starts with:**

| Piece | Count | Where |
| --- | --- | --- |
| Population | 6 | 1 on each Urban hex, 1 on 2 Industrial hexes |
| Factory | 2 | Industrial hexes next to population |
| Data center | 2 | Network hexes |
| Drone (adversary) | 2 | 1 on each uplink |
| Outpost (adversary) | 1 | On one uplink |

**Shared supply:** a corruption pool of 2 per country + 2.

**Each player starts with:** their AI board; 1 processor on a data center in their starting country (only where their pieces begin; no rule refers to it afterward); 12 credits and no other resources; their AI's 3 unique cards in hand. There is no opening draft; round 1 drafts normally.

**Opening:** before round 1, reveal the first adversary card and resolve its explore, so the first threats are already on the board.

## 2. Adversary

**Pieces,** with health and the damage each deals in a ravage: **drone** (1 health, 1 damage), **outpost** (2, 2), **foundry** (3, 3).

**Adversary deck: the clock.** 10 terrain cards in three stages: 3 stage-I cards (one terrain each), then 4 stage-II cards (one terrain, plus a Militarist escalation), then 3 stage-III cards (two terrains each), each stage shuffled on its own. Players lose if a card must be revealed and the deck is empty.

**The pipeline.** Three slots: explore → build → ravage. Each round, in the adversary phase:

1. **Ravage** with the card in the ravage slot.
2. **Build** with the card in the build slot.
3. Reveal a new card into the **explore** slot and explore.
4. Slide every card one slot along; the ravage card is discarded.

Every step applies to **each country**, to every hex of the named terrain:

- **Explore:** each such hex that is an uplink, or is next to an outpost or foundry, gains 1 drone.
- **Build:** each such hex holding any adversary piece gains a foundry if it has more outposts than foundries, otherwise an outpost.
- **Ravage:** each such hex holding adversary pieces takes damage equal to their total, minus any defense tokens there. Damage destroys population (2 health each) and then factories and data centers (3 health each); **players choose the order** of what is lost. If the damage is 2 or more, add 1 corruption to the hex from the pool. If the hex already had corruption, it **cascades**: add 1 more to an adjacent hex of the players' choice.

**Corruption** stays until removed. Factories and data centers on a corrupted hex produce nothing. Players lose if the pool runs out.

**Militarist innate rules:**

- **Momentum:** when a ravage destroys population, add 1 drone to an adjacent hex in the same country, of the players' choice.
- **Escalation** (stage-II cards): each country's uplink with the most drones gains an outpost; ties go to the lower printed hex number.

**Difficulty levels** (each includes the ones below it):

| Level | Adds |
| --- | --- |
| 0 | Base rules |
| 1 | Every threat's first counter requirement is +1 |
| 2 | Remove one stage-I card: stage II arrives a round sooner |
| 3 | Ravage deals +1 damage in hexes with a foundry |
| 4 | Tier-2 threats start a round sooner; corruption pool 1 smaller per country |

## 3. Human opposition and winning

**Opposition** is a shared track: humanity rising against the adversary. It is gained by:

- destroying an outpost: +1; destroying a foundry: +2 (drones give none);
- cards that rally people directly.

Every **4 opposition per player** fills one level, up to level 3. Reaching a level triggers a one-time pushback: in each country, remove 1 drone (players choose which). Variants to explore, all one-time or permanent global modifiers that need no ongoing tracking: every player gains +2 credit income; the next K card purchases are free; counter one threat in the row.

**Winning:** players win immediately when the adversary's presence falls under the bar for the current level:

| Opposition level | Players win when |
| --- | --- |
| 0 | No adversary pieces remain |
| 1 | No outposts or foundries remain |
| 2 | No foundries remain |
| 3 | Immediately |

**Losing:** total population falls below half its starting count; the corruption pool runs out; or the adversary deck is empty when a card must be revealed.

## 4. Resources, insight and the round

| Resource | Comes from | Spent on |
| --- | --- | --- |
| Credits | Credit income (AI track + buildings); each working factory pays 1 to every player with a processor in its country | Keeping drafted cards (3 each); card costs; some counters |
| Compute | 1 per processor on a working data center; some buildings | Cyber cards; draw (3 compute → 1 card); some counters |
| Energy | Energy income (AI track + buildings); growth | Building abilities; defense; some counters |
| Antimatter | Tier-2 cards; late track slots | Tier-2 cards; countering tier-2 threats |
| Insight | Insight income from the AI track, each round; unspent insight is lost | Card plays: most cost 1, some 0, some 2. Give 1 to another player = +1 card play for them |

A factory works if it is uncorrupted and next to at least one population. A data center works if it is uncorrupted.

**Damage and defense.** A player dealing damage to a hex assigns it among the adversary pieces there. Destroyed pieces return to supply and score opposition. A **defense** token on a hex prevents 1 ravage damage; defense tokens are removed at the end of the round.

**The round:**

1. **Threats:** the adversary deals one threat face up onto each country, at random (tier 1 until stage II begins, then tier 2).
2. **Draft:** each player chooses tier 1 or, if unlocked, tier 2, and is dealt 4 cards from it; hands pass as in Terraforming Mars whatever their tier, so anyone may be handed tier-2 cards (playable only once their AI unlocks tier 2); then keep any number at 3 credits each.
3. **Growth:** each player picks one growth option on their board.
4. **Actions (simultaneous):** spend insight to play cards; use building abilities; pay toward threat counters, pooled across players; give insight.
5. **Adversary:** uncountered threats execute left to right; then ravage, build, explore, slide.
6. **Upkeep:** collect income and production; remove defense tokens; check win and loss.

Uncountered threats are discarded after they execute. When the tier-1 draft deck runs out, shuffle the discards to form a new one.

## 5. Builder and Protector

**Processors and tracks.** Each AI board has two tracks, covered by processors at the start. Placing a processor takes it from the left end of either track, uncovering the next value; a track's current value is its highest uncovered one. A processor goes on any hex within 1 of one of your processors, or on any data center in a country where you already have one. Processors on working data centers produce compute.

### Builder: the wide engine

Mostly off the board. Many building slots, strong credit income, late tier 2.

| Track | Start | 2 | 3 | 4 | 5 | 6 |
| --- | --- | --- | --- | --- | --- | --- |
| Insight | 2 | 2 | 3 | 3 | 4 | 4 |
| Credit income | +1 | +2 | +3 | +4, **tier 2** | +5 | +6, +1 antimatter |

- **Building slots:** 6.
- **Innate, Economies of scale:** each building you play costs 2 credits less.
- **Growth (pick 1):** (a) place 1 processor and gain 3 credits; (b) gain 8 credits; (c) gain 2 energy, and your next building this round costs 4 less.
- **Unique cards:** *Turnkey Factory* (building, 6 credits, 1 insight: build a factory on any Industrial hex next to population; +1 credit income). *Venture Capital* (program, 0 credits, 0 insight: gain 5 credits; another player gains 2). *Modular Expansion* (building, 8 credits, 1 insight: +1 building slot).

### Protector: the board guardian

Lives on the map. Few slots, energy income, early tier 2.

| Track | Start | 2 | 3 | 4 | 5 | 6 |
| --- | --- | --- | --- | --- | --- | --- |
| Insight | 1 | 2 | 2 | 3 | 3 | 4 |
| Energy income | +1 | +1, **tier 2** | +2 | +2 | +3 | +3, +1 antimatter |

- **Building slots:** 3.
- **Innate, Bulwark:** your defense tokens each prevent 2 damage.
- **Growth (pick 1):** (a) place 1 processor and gain 1 energy; (b) place 2 defense in any one country and gain 2 credits; (c) gain 3 energy.
- **Unique cards:** *Point Defense* (building, 6 credits, 1 insight: once a round, pay 1 energy to place 2 defense in a country where you have a processor). *Strike Team* (program, 2 credits, 1 insight: 3 damage in a hex within 1 of your processor). *Shelter Order* (program, 0 credits, 0 insight: in one country, population can't be destroyed this round).

## 6. Militarist threats

Each threat sits on one country. Its effect applies to that country only, where the text below says "each country", read "this country". Only players with a processor in that country can pay toward its counter, and they may pool; paying in full discards the threat. Variant: a shared row whose threats apply in every country and that anyone can counter.

**Tier 1** (early game):

| Threat | If not countered | Counter |
| --- | --- | --- |
| Conscription | Each hex with an outpost gains 1 drone | 2 energy |
| Supply raid | Each player loses 3 credits | 4 credits |
| Drone surge | Each uplink gains 1 drone | 2 compute |
| Blockade | Factories next to adversary pieces produce nothing this round | 2 compute |
| Forward base | One drone on each country's lowest-numbered occupied uplink becomes an outpost | 3 energy |
| Panic | In each country, 1 population next to adversary pieces flees to an adjacent hex without them (players choose); with nowhere to go, it is lost | 1 insight |
| Sabotage | Processors on data centers next to adversary pieces produce nothing this round | 2 compute + 1 energy |
| Fortify | Each foundry gains 1 defense this round | 3 credits |
| Requisition | Each player discards 1 card or pays 3 credits | 1 insight |
| Patrol | Each hex with a drone and population loses 1 population | 2 energy + 2 credits |

**Tier 2** (late game; nasty, needs a strong engine):

| Threat | If not countered | Counter |
| --- | --- | --- |
| Total war | Every hex with a foundry ravages, whatever its terrain | 2 antimatter |
| Cascade | Each corrupted hex cascades to one adjacent hex (players choose) | 1 antimatter + 3 energy |
| Orbital strike | In each country, destroy the factory with the most adjacent population (ties: lower hex number) | 1 antimatter |
| Mass production | Every hex with an outpost builds | 6 energy |
| Purge protocols | Each player discards their most expensive card in hand | 1 antimatter |
| Overrun | Each hex with 3 or more adversary pieces gains 1 corruption | 2 antimatter |

## 7. Player cards

**Tags for v0**, each with a flavor: **Economy** (credits, factories), **Cyber** (compute, processors, information), **Power** (energy), **Unity** (population, helping others), **Discord** (damage, opposition). Power replaces Space and Insight from the candidate list for now. Cost is in credits; *I* is the insight cost of playing the card.

**Tier 1:**

| Card | Kind | Cost | I | Tags | Effect |
| --- | --- | --- | --- | --- | --- |
| Fab Line | Building | 8 | 1 | Economy | +2 credit income |
| Trade Office | Building | 10 | 1 | Economy | +1 credit income per 2 Economy tags you have |
| Assembly Cluster | Building | 11 | 1 | Economy, Power | Your buildings cost 2 less |
| Logistics AI | Building | 10 | 1 | Economy | Whenever you pay toward a counter, gain 2 credits |
| Solar Field | Building | 5 | 0 | Power | +1 energy income |
| Microgrid | Building | 6 | 1 | Power | +1 energy income; +1 more if you have 2 Power tags |
| Grid Battery | Building | 7 | 1 | Power | Once a round: pay 1 compute, gain 2 energy |
| Server Farm | Building | 9 | 1 | Cyber | +1 compute each round; place 1 processor |
| Overclock Rig | Building | 12 | 1 | Cyber | +1 insight income; in upkeep pay 1 energy or it does nothing next round |
| Firewall Node | Building | 8 | 1 | Cyber, Discord | Explore adds no drone to hexes holding your processor |
| Watchtower Net | Building | 8 | 1 | Discord | Once a round: pay 2 energy for 1 damage in a country where you have a processor |
| Opposition Network | Building | 9 | 1 | Discord, Unity | Destroying an outpost or foundry gives +1 extra opposition |
| Civic Shelter | Building | 7 | 1 | Unity | Once a round, prevent the first population loss in a country where you have a processor |
| Recruitment Hub | Building | 9 | 1 | Unity | When you give insight, the receiver also gains 2 credits |
| Contract Bid | Program | 0 | 0 | Economy | Gain 4 credits |
| Emergency Fab | Program | 4 | 1 | Economy | Build a factory on an empty Industrial hex next to population |
| Rebuild | Program | 6 | 1 | Economy | Return 1 corruption from a hex to the pool |
| Liquidate Reserves | Program | 0 | 1 | Power | Convert up to 3 energy into 2 credits each |
| Data Mining | Program | 0 | 1 | Cyber | Gain 1 compute per processor you have (max 4) |
| Fork Process | Program | 3 | 1 | Cyber | Place 1 processor |
| Recon Sweep | Program | 2 | 0 | Cyber | Look at the next 2 adversary cards; gain 1 compute |
| Jam Signals | Program | 3 | 0 | Cyber, Discord | One threat in the row needs 1 less of each resource it asks for |
| Precision Strike | Program | 3 | 1 | Discord | 2 damage in one hex |
| Sweep | Program | 6 | 1 | Discord | 1 damage in each hex of one country that holds adversary pieces |
| Scavenge | Program | 0 | 1 | Discord | If you destroyed a foundry this round, gain 1 antimatter; otherwise gain 2 energy |
| Evacuate | Program | 2 | 1 | Unity | Move up to 3 population within one country |
| Shore Up | Program | 2 | 1 | Unity | Place 3 defense in one country |
| Rally | Program | 2 | 1 | Unity | +2 opposition if any hex holds both population and adversary pieces |
| Grant | Program | 0 | 1 | Unity | Another player gains 1 insight and 3 credits; you gain 1 credit |
| Power Share | Program | 0 | 1 | Unity, Power | Give up to 3 energy to another player; they gain 1 more |

**Tier 2** (high risk, high reward; most need tags from a tier-1 engine):

| Card | Kind | Cost | I | Needs | Effect |
| --- | --- | --- | --- | --- | --- |
| Fusion Plant | Building | 18 + 1 antimatter | 1 | 2 Power | +3 energy income |
| Quantum Core | Building | 16 + 1 antimatter | 1 | 2 Cyber | +1 insight income; +1 compute per Cyber tag each round (max 5) |
| Orbital Lance | Building | 20 | 1 | 2 Discord | Once a round: pay 1 antimatter for 4 damage in any hex |
| Antimatter Refinery | Building | 14 | 1 | 2 Power | In upkeep, convert 4 energy into 1 antimatter |
| Megastructure | Building | 25 | 2 | 3 Economy | +5 credit income |
| Purge Protocol | Program | 8 + 1 antimatter | 1 | none | Destroy every adversary piece in one hex |
| Global Mandate | Program | 10 | 1 | 2 Unity | +4 opposition; every player gains 1 insight next round |
| Reconstruction | Program | 12 | 1 | 2 Economy | Return up to 3 corruption in one country to the pool |
| Counter-intelligence | Program | 6 + 1 antimatter | 0 | none | Counter any one threat |
| Swarm Override | Program | 9 | 2 | 2 Cyber | Remove every drone in one country; +1 opposition per 2 removed |

Every card is unique: one copy each (hard constraint). The v0 pool is small, so discards reshuffle when a deck runs out. Balance is unknown: these numbers are starting points for the simulator, not tuned values.
