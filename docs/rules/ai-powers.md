# AI-board powers (module `ai`; [`content/ais.json`](../../content/ais.json))

*Each board's Growth (its node placements) and powers, the boards' insight conversions, shelters and hardening.
The boards, their special rules and tracks are in [ai-boards.md](ai-boards.md); terms and conventions in
[README.md](README.md).*

**The shape (Matt, 2026-11-18).** A board prints a **Growth** (pick one option a round, Spirit Island's "pick one"),
two or three **tiered powers** (pick one tier a round) and at most one **threshold** power: about 3-4 decisions a
round. Each Growth option and each tier is one use of its group: one used marker a group. In `content/ais.json` a
power names its `group`; powers sharing one are a single use a round between them.

**Powers (locked: tag costs are sacrifices).** A power's cost names N of one tag or N of `any` tag, never a mix,
plus optional other resources; in the table a tag in the cost is a sacrifice. A higher **tier** sacrifices more of
the same tag for a stronger effect (tier II: 2 tags). `Content::check` refuses a mix, and refuses shields and delays
on a board power (they are the cards'). It is paid by
discarding played cards that carry those tags: a program from your play area, or a building **with its whole stack**
beneath it; events can't be sacrificed (they are gone once played). A card pays with all its tags that match; extra
tags are wasted. What can be sacrificed is fixed when the power is used; the player chooses the cards one at a time
among those that still help.

**Threshold powers (Matt, 2026-11-18; a few, where sacrifice would work against the board).** "With N Science on
your cards" counts tags as a requirement does (the board's printed and revealed tags count) and sacrifices nothing:
the Researcher's Lab, so the Science AI can hoard Science. `Content::check` refuses a power that both counts and
sacrifices tags.

**Resource-paid powers stand** (Matt, 2026-11-15, "energy powers things"; message 12618): the Builder's Fortify is
paid in chips (its chip sink) and the Protector's Patrol in energy, which also keeps Security cards for Interdict.

**One target (locked).** A power that touches the map names one target, a hex in your reach, as a
card does ([card-grammar.md](card-grammar.md)): its text opens "Target: ...", and it acts on the target, or on or
next to it where printed. `Content::check` holds powers to the cards' check and the opening words. Node placements
(next to your nodes) and Lab (off the map) take none; the one exception is Hack's country.

<!-- generated: powers -->
| Board | Power | Use (once a round) | Cost | Effect |
|---|---|---|---|---|
| Builder | Sprawl | Growth (pick one) | 1 energy | place 2 Nodes |
| Builder | Buy In | Growth (pick one) | nothing | place 1 Node; gain 1 chip |
| Builder | Annex | Growth (pick one) | nothing | place 1 Node within 2 of yours; gain 3 credits |
| Builder | Scrap I | Scrap, tier I | 1 Industry | gain 1 chip |
| Builder | Scrap II | Scrap, tier II | 2 Industry | gain 1 chip; play a building from your hand, paying no credits |
| Builder | Overcharge I | Overcharge, tier I | 1 Power | gain 2 energy |
| Builder | Overcharge II | Overcharge, tier II | 2 Power | gain 2 energy and 1 antimatter |
| Builder | Fortify I | Fortify, tier I | 1 chip | Target: Troops. Remove 1 Troop on it |
| Builder | Fortify II | Fortify, tier II | 2 chips | Target: Troops. Remove up to 2 Troops on it; harden a facility on or next to it |
| Protector | Outpost | Growth (pick one) | 2 energy | place 2 Nodes |
| Protector | Accord | Growth (pick one) | 1 energy | place 1 Node; gain 1 ambassador |
| Protector | Levy | Growth (pick one) | nothing | place 1 Node; gain 3 credits |
| Protector | Patrol I | Patrol, tier I | 1 energy | Target: Agents or Troops. Push up to 3 from it, each to a hex next to it |
| Protector | Patrol II | Patrol, tier II | 2 energy | Target: Agents or Troops. Push up to 3 from it, each to a hex next to it; harden a facility on or next to it |
| Protector | Interdict I | Interdict, tier I | 1 Security | Target: Troops. Remove up to 2 on it |
| Protector | Interdict II | Interdict, tier II | 2 Security | Target: Agents or Troops. Remove up to 2 Troops and up to 2 Agents on it; draw 1 card |
| Researcher | Peer Review | Growth (pick one) | 2 insight | place 2 Nodes |
| Researcher | Field Survey | Growth (pick one) | nothing | place 1 Node; draw 1 card |
| Researcher | Field Station | Growth (pick one) | 1 insight | Target: an open hex. Build a research station on it, paying no chips; place 1 Node |
| Researcher | Lab I | Lab, tier I, with 2 Science on your cards | 1 insight | draw 1 card |
| Researcher | Lab II | Lab, tier II, with 4 Science on your cards | 1 insight + 2 credits | draw 1 card; add 1 research cube in your color to an open megaproject |
| Researcher | Hack I | Hack, tier I | 1 Cyber | remove up to 2 Troops in one country |
| Researcher | Hack II | Hack, tier II | 2 Cyber | remove up to 2 Troops in one country; remove 3 compute from one adversary project |
<!-- /generated -->

**Growth is the board's node placement** (Matt, 2026-11-18). There is no other node route: each round a board may
pick one Growth option, and every placement, by Growth or card, pays the energy printed on the track space it
uncovers (locked). Each board's three options keep the old routes' intents: a cheap option (Buy In, Accord, Field
Survey: 1 node and a small rider), the energy-rich boards' two-node option paid in energy (Sprawl, Outpost), the
Researcher's paid in insight (Peer Review), the Builder's within-2 reach (Annex), the Researcher's station (Field
Station). With no token left, no step it can pay or no free hex, an option isn't offered; a two-node option needs
both nodes' steps (276). Cards that place nodes add
to Growth (cards aren't capped).

**Growth's give / get** (276, at the unit anchors; a node 5.33, besides each placement's track step): Sprawl 1
energy (3) for 2 nodes (10.67); Buy In nothing for a node and a chip (8); Annex nothing for a node within 2 and 3
credits (8.33 and the reach); Outpost 2 energy (6) for 2 nodes; Accord 1 energy (3) for a node and an ambassador (8);
Levy nothing for a node and 3 credits (8.33); Peer Review 2 insight (3.33) for 2 nodes; Field Survey nothing for a
node and a card (8); Field Station 1 insight (1.67) for a station and a node (12). One option a round and the
steps' rising energy cap them all.

**Sacrifice is late-game liquidation.** Once the cards needing a tag are played, its tags turn into engine rewards,
each payoff priced to be worth the card: 1 chip (Scrap I), 2 energy (Overcharge I), up to 2 troops removed on a
target (Interdict I and Hack I); tier II pays a second tag for more: a free building (Scrap II), an antimatter
(Overcharge II), agents and a card (Interdict II), compute (Hack II).

**Each power's tag is its story.** A tag-paid power is paid in the tag its effect fits
(the tag table is below): the Protector's Interdict in Security (force, protection). The one exception is the
Researcher's Hack, paid in Cyber: it hacks troops anywhere in a country, no reach.

**Card throughput:** draw is a precious resource (*Card draw*, [player-cards.md](player-cards.md)). The Researcher's
Lab (with 2 Science, 1 insight → a card) is its draw outlet; Field Survey and Interdict II draw beside what they do.
The Builder has no draw power: its cards are its draw.

**Troop powers (early troops decide games).** Each board removes or stops troops its own way, no two alike: the
Protector's **Interdict** (Security: up to 2 troops on its target; tier II agents too) beside its Patrol (pushes
them away); the Builder's **Fortify** (chips: 1 troop on its target, tier II up to 2 and a facility hardened); the
Researcher's **Hack** (Cyber: up to 2 troops in one country, no reach needed). The standard action Remove a troop
(`troop_credits` 7 credits, `troop_per_round` once a round) is the hedge.

**Antimatter.** The steady source is the tier-2 converter plants ([economy.md](economy.md)); the Builder's
Overcharge II (2 Power tags) adds one, and the Protector's and Researcher's capstones pay 2 once. No power spends
antimatter: the spends are the majors, tier-2 cards and a few dear tier-1 ones.

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
  march, outside the action phase where powers are used: the march's (phase 5) pays this round's keeps and plays, an
  immediate march's (phase 16) comes after them.
- **Researcher, Lab** (a threshold power, once a round): with 2 Science on your cards, 1 insight → draw a card; with
  4, also 2 credits → a research cube on an open megaproject. The Researcher's draw outlet and a use for Patience's
  bank. Give / get 1.67 for 2.67 (tier II 3.67 for 6.67): a draw, not a gain. Peer Review and Field Station, its
  Growth, spend insight too.

**For new players.** Insight is what you spend to keep and play cards: each round, the number on the rightmost space you've uncovered on
your card-play track that shows one (3 at the start). When you
are short, turn 1 research cube into 1 insight once a round (Turn a cube into insight; Cintamani's converter gives 2 insight for 1 cube), and use your board's own
way: the Protector gains 1 each of the first two times each round a troop ends its march next to its node (Vigil); the
Researcher keeps what it doesn't spend (Patience) and may spend 1 for a card (Lab). Insight you don't spend still goes at the round's end (the Researcher
keeps all of it), but a few threat counters offer insight (3 at tier 1, 4 at tier 2: [threats.md](threats.md) *Counters*),
so spare insight can stop one.

**Insight loops.** Nothing turns insight back into what makes it: insight buys cards, and a card that pays insight
(a trigger, an income) is capped by plays and slots. The cards that make research cubes are capped by a once-a-round Action (Particle Accelerator), an
income of 1 a round (Cryo Sample Vault) and one-time plays (Field Grant, Collider Consortium, Survey Drone Fleet), and the cube converter is once a
round, so cubes → insight → cards → cubes can't run away. Every megaproject converter is once a round, which caps Onyx's loop: 1 cube → 1 chip, 2 chips → a station → 1 cube a round once the
station's card is complete, which would otherwise grow its cubes each round, capped only by open hexes in reach.

## Shelters and hardening

A **shelter** (a card's "Shelter your Nodes and facilities in reach", no target: yours in your reach when a piece would
be destroyed, so nodes placed after it count) lasts until cleanup (phase 18) of the round it is used in. Used in the actions, it covers the rest of the actions and what comes after them: the income step,
the extra drones, the threats that execute and war (phases 13-17). The march (phase 5) has already happened, but a threat's
immediate march (Total War's, phase 16) meets it. It stops a destruction automatically, spending nothing, and so does the Protector's Aegis Doctrine capstone, a shelter
on its own pieces (the first time each round one of its nodes or facilities would be destroyed); the other protections
(a hardening token below, your own ambassador) are the piece owner's choice; drones never protect ([war.md](war.md),
*Destruction*).

Hardening (Fortify II, Patrol II, cards) goes only on facilities a token can protect, player-owned stations and
factories that are not corrupted, and "harden N" is N different facilities, fewer if fewer qualify. "Harden N of
your Nodes" (Stress Test Suite, Early Warning Drill) puts the token on a hex of your nodes instead: it absorbs a destruction there and goes with the node if it is
lost another way. On your own facility it is the facility's hardening token: a table has one token, so it stays
when the node goes, and protects the facility.
