# AI-board powers (module `ai`; [`content/ais.json`](../../content/ais.json))

*Each board's powers (node, sacrifice and insight), the boards' insight conversions, shelters and hardening.
The boards, their special rules and tracks are in [ai-boards.md](ai-boards.md); terms and conventions in
[README.md](README.md).*

**Powers (locked: tag costs are sacrifices).** A power's cost names N of one tag or N of `any` tag, never a mix,
plus optional other resources; in the table a tag in the cost is a sacrifice, and thresholds above 1 are for powerful
cards only. `Content::check` refuses a mix, and refuses shields and delays on a board power (they are the cards'). It is paid by
discarding played cards that carry those tags: a program from your play area, or a building **with its whole stack**
beneath it; events can't be sacrificed (they are gone once played). A card pays with all its tags that match; extra
tags are wasted. Each power is usable once a round. What can be sacrificed is fixed when the power is used; the player chooses the cards one at a time among those that
still help.

**One target (locked).** A power that touches the map names one target, a hex in your reach, as a
card does ([card-grammar.md](card-grammar.md)): its text opens "Target: ...", and it acts on the target, or on or
next to it where printed. `Content::check` holds powers to the cards' check and the opening words. Node placements
(next to your nodes), the burns (your own piece anywhere) and Grant and Audit (off the map) take none; the one
exception is Disrupt's country.

<!-- generated: powers -->
| Board | Power | Cost | Effect |
|---|---|---|---|
| Builder | Mint | 3 credits | gain 1 chip |
| Builder | Expand | nothing | place 1 Node |
| Builder | Buy In | 2 energy | place 1 Node; if the new node brings a factory into your reach, also gain 1 chip |
| Builder | Annex | 1 chip | place 1 Node within 2 of one of yours |
| Builder | Scrap | 1 Industry | gain 1 chip |
| Builder | Overcharge | 1 Power | gain 2 energy |
| Builder | Rush Build | 1 Industry + 1 energy | play a building from your hand, paying no credits |
| Builder | Procure | 1 chip | draw 2 cards |
| Builder | Fortify | 1 chip | Target: Troops. Remove 1 Troop on it; harden a facility on or next to it |
| Builder | Teardown | 1 chip | destroy one of your factories anywhere; Fallout goes on its hex; gain 1 antimatter |
| Protector | Bulwark | 1 energy | Target: a facility. Harden it |
| Protector | Expand | nothing | place 1 Node |
| Protector | Accord | 2 energy | place 1 Node; if the new node brings a city into your reach, also put a new ambassador on one such city |
| Protector | Outpost | 3 energy | place 1 Node; gain 1 ambassador |
| Protector | Debrief | 1 Security | Target: Agents. Draw 1 card and remove 1 Agent on it |
| Protector | Patrol | 1 energy | Target: Agents or Troops. Push up to 3 from it, each to a hex next to it |
| Protector | Interdict | 1 Security | Target: Troops. Remove up to 2 on it |
| Protector | Reclaim | 1 Security + 6 credits + 2 energy | Target: a corrupted research station or factory. Turn it neutral; gain 1 antimatter |
| Researcher | Expand | nothing | place 1 Node |
| Researcher | Peer Review | 2 insight | place 1 Node; if the new node brings a research station into your reach, also gain 1 insight |
| Researcher | Field Survey | 3 insight | place 1 Node; draw 1 card |
| Researcher | Grant | 3 credits | add 1 research cube in your color to an open megaproject |
| Researcher | Field Station | 1 Industry | Target: an open hex. Build a research station on it, paying no chips |
| Researcher | Audit | 1 Cyber | remove 3 compute from one adversary project |
| Researcher | Disrupt | 1 Cyber + 1 research cube | remove up to 2 Troops in one country |
| Researcher | Meltdown | 2 research cubes | destroy one of your research stations anywhere; gain 1 antimatter |
| Researcher | Synthesis | 1 insight | draw 1 card |
<!-- /generated -->

**Sacrifice is late-game liquidation.** Once the cards needing a tag are played, its tags turn into engine rewards,
each payoff priced to be worth the card: 1 chip, 2 energy, up to 2 troops removed on a target (Interdict), a card and
an agent removed (Debrief); a tag and an energy buy a building.

**Each power's tag is its story.** A tag-paid power is paid in the tag its effect fits
(the tag table is below): the Protector's Debrief and
Interdict in Security (force, protection). The one exception is the Researcher's Disrupt, paid in Cyber: it
hacks troops anywhere in a country, no reach.

**Energy powers things (Matt, 2026-11-15).** The two boards that earn the most energy place nodes with it: Buy In
and Accord cost 2 energy each (no credits, no sacrifice), beside the track step every placement pays; Bulwark and Patrol cost 1. At the anchors a node (8) and its rider (a chip 6, an ambassador 8, at the
72% a fresh spot comes up) are worth 12-14 for 9. The Researcher, short of energy, places with insight (Peer Review
2, its rider 1 back: 8 for 8 + 2.9), never research cubes (Matt, 2026-11-16).

**Node routes' give / get (252, at the unit anchors; a node 8, besides the track step every placement pays):**
Expand, nothing for a node (the base route, once a round, Matt's ruling); Annex 1 chip (6) for a node within 2 (8,
a reach no other route has): 1.33; Outpost 3 energy (13.5) for a node and an ambassador (16): 1.19; Field Survey 3
insight (12) for a node and a card (13): 1.08. Each is once a round; the step's rising energy caps all three.

**Node powers' riders (candidate).** A node power places next to your nodes as any placement does, and its bonus comes
only when the new node is **on or next to a fresh spot of its board's kind**, one none of your other nodes is on or
next to: Buy In's chip by a factory, Accord's ambassador by a city (a new ambassador onto that city, where it
stands as any ambassador does), Peer Review's insight by a research station that stood there before the placement (not one its uncovered track
space builds, r122). With no such spot it still places, without the bonus. Why
fresh: a city, a factory and a station are on or next to almost every hex, so "on or next to" alone was met on 99-100%
of placements; a fresh spot, about 72%, pays for reaching a new one. Nothing places a node anywhere: next to your
nodes, occasionally within 2 (Annex, the Builder's top route; Matt, 2026-11-16), and cards may target.

| Tag | Story | Its usual effects (the tag story table of scripts/design_check.json) |
|---|---|---|
| Security | force and protection | troops and agents removed, pushes, shelters, cordons, hardening, shields on node hexes |
| Diplomacy | cooperation: trust, Unity, green cleanup | ambassadors, Unity Sway, cleanses, land lifts, gifts |
| Science | discovery | insight, research cubes, stations, megaproject cubes, antimatter physics, delays |
| Industry | building | chips, factories, capital, free buildings |
| Power | energy and antimatter | energy, antimatter, the weapons it feeds |
| Cyber | the network | botnets, compute drains, firewalls, node placement, information (insight) |

**Tags are story, not rules:** no tag owns an effect, as in Magic's colors. The table is what a
player expects; a card whose effects fit none of its tags is a soft design warning (`design_check.py`'s tag-story),
never refused. Draws, credits, triggers, threat counters, liberation and a tag's own payoffs fit any tag.

**Card throughput:** the Builder's Procure turns a spare chip into 2 cards; the Researcher's Synthesis, its draw
outlet, turns 1 insight into a card.

**Troop powers (early troops decide games).** Each board removes or stops troops its own way, no two alike: the
Protector's **Interdict** (a Security card: up to 2 troops on its target) beside its Patrol (pushes them away); the
Builder's **Fortify** (1 chip: 1 troop on its target and a facility on or next to it hardened); the Researcher's
**Disrupt** (a Cyber card and a research cube: up to 2 troops in one country, no reach needed). The standard action
Remove a troop (`troop_credits` 10 credits, `troop_per_round` once a round) is the hedge.

**Antimatter: each board a rare burn of its own engine.** The steady source is the tier-2 converter plants
([economy.md](economy.md)); each board's own power is a different decision, once a round, for 1 antimatter: the
Builder's **Teardown** (1 chip: destroy one of your own factories, and a Fallout token goes on its hex), the
Researcher's **Meltdown** (2 research cubes: destroy one of your own research stations), the Protector's **Reclaim**
(a Security card, 6 credits and 2 energy: turn its target, a corrupted station or factory, neutral, its adversary cube
off, well above Remove a troop's 10). Teardown and Meltdown reach anywhere. A burned piece is gone, no protection asked (you chose it), unless rebuilt the
normal way; with no piece or target the power can't be used. No power spends antimatter: the spends are the majors,
tier-2 cards and a few dear tier-1 ones.

## Insight conversions (candidate)

Insight is this round's attention, readable off the board: it pays keeps and plays ([player-cards.md](player-cards.md)),
and credits never buy a keep. The card-play track prints it as totals ([ai-boards.md](ai-boards.md), *The card-play
track*); the economy track prints none. Every player has the cube converter, a standard action: 1 research cube for 1 insight, once a
round (`insight_cubes`, `cube_insight`, `insight_per_round`; [economy.md](economy.md)). **Insight must bind** (the target: about 6 a round to drafting and 2-4 to plays), so the
boards' own insight ways are few:

- **Protector, Vigil** (an Effect of Wall, not a power; some cap is locked, its size a candidate): the first two
  troops each round that end their march next to your node pay 1 insight each (`board.vigil_cap` troops a round);
  later ones that round pay nothing, so a troop parked by your node doesn't pay every march. Any march counts (a
  threat's immediate march too); a troop used up striking has no end to its march. An Effect because it happens in a
  march, outside the action phase where powers are used: the march's (phase 7) pays this round's keeps and plays, an
  immediate march's (phase 18) comes after them.
- **Researcher, Synthesis reversed** (a power, once a round): 1 insight → draw a card, the Researcher's draw outlet
  and a use for Patience's bank. No other insight → card action exists. Give / get 4 for 5 at
  the anchors (a card kept at 1 insight is 4 more): a draw, not a gain. Peer Review, its node power, pays 1 insight by a fresh station.

**For new players.** Insight is what you spend to keep and play cards: each round, the number on the rightmost space you've uncovered on
your card-play track that shows one (3 at the start). When you
are short, turn 1 research cube into 1 insight once a round (Turn a cube into insight; Cintamani's converter gives 2 insight for 1 cube), and use your board's own
way: the Protector gains 1 each of the first two times each round a troop ends its march next to its node (Vigil), the
Researcher gains 1 with each node Peer Review places by a station new to its nodes and may spend 1 for a card (Synthesis). Insight you don't spend still goes at the round's end (the Researcher
keeps all of it), but a few threat counters offer insight (3 at tier 1, 4 at tier 2: [threats.md](threats.md) *Counters*),
so spare insight can stop one.

**Insight loops.** Nothing turns insight back into what makes it: insight buys cards, and a card that pays insight
(a trigger, an income) is capped by plays and slots. The cards that make research cubes are capped by a once-a-round Action (Particle Accelerator), an
income of 1 a round (Cryo Sample Vault) and one-time plays (Field Grant, Collider Consortium, Survey Drone Fleet), and the cube converter is once a
round, so cubes → insight → cards → cubes can't run away. Every megaproject converter is once a round, which caps Onyx's loop: 1 cube → 1 chip, 2 chips → a station → 1 cube a round once the
station's card is complete, which would otherwise grow its cubes each round, capped only by open hexes in reach.

## Shelters and hardening

A **shelter** (a card's "Shelter your Nodes and facilities in reach", no target: yours in your reach when a piece would
be destroyed, so nodes placed after it count) lasts until cleanup (phase 20) of the round it is used in. Used in the actions, it covers the rest of the actions and what comes after them: the income step,
growth, the threats that execute and war (phases 15-19). The march (phase 7) has already happened, but a threat's
immediate march (Total War's, phase 18) meets it. It stops a destruction automatically, spending nothing, and so does the Protector's Aegis Doctrine capstone, a shelter
on its own pieces (the first time each round one of its nodes or facilities would be destroyed); the other protections
(a hardening token below, your own ambassador) are the piece owner's choice; drones never protect ([war.md](war.md),
*Destruction*).

Hardening (the standard actions, cards) goes only on facilities a token can protect, player-owned stations and
factories that are not corrupted, and "harden N" is N different facilities, fewer if fewer qualify. A card's (a
shield, usually Security's) hardening token on a hex of your nodes absorbs a destruction there and goes with the node if it is
lost another way. On your own facility it is the facility's hardening token: a table has one token, so it stays
when the node goes, and protects the facility.
