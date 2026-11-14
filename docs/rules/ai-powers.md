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

<!-- generated: powers -->
| Board | Power | Cost | Effect |
|---|---|---|---|
| Builder | Mint | 3 credits | gain 1 chip |
| Builder | Buy In | 6 credits | place 1 Node; if the new node brings a factory into your reach, also gain 1 chip |
| Builder | Scrap | 1 Industry | gain 1 chip |
| Builder | Overcharge | 1 Power | gain 2 energy |
| Builder | Rush Build | 1 Industry + 1 energy | play a building from your hand, paying no credits |
| Builder | Procure | 1 chip | draw 2 cards |
| Builder | Fortify | 2 chips | remove 1 Troop in reach and harden 1 facility in reach |
| Builder | Teardown | 1 chip | destroy one of your factories anywhere; Fallout goes on its hex; gain 1 antimatter |
| Protector | Bulwark | 1 energy | harden 1 facility in reach |
| Protector | Accord | 1 Diplomacy | place 1 Node; if the new node brings a city into your reach, also put a new ambassador on one such city |
| Protector | Debrief | 1 Security | draw 1 card and remove 1 Agent in a hex in reach |
| Protector | Patrol | 1 energy | push up to 3 Agents or Troops from a hex in reach, each to a hex next to it |
| Protector | Interdict | 1 Security | remove up to 2 Troops in one hex in reach |
| Protector | Reclaim | 1 Security + 6 credits + 2 energy | turn 1 corrupted station or factory in reach neutral; gain 1 antimatter |
| Researcher | Peer Review | 1 research cube | place 1 Node; if the new node brings a research station into your reach, also gain 1 insight |
| Researcher | Grant | 3 credits | add 1 research cube in your color to an open megaproject |
| Researcher | Field Station | 1 Industry | build a research station in reach, paying no chips |
| Researcher | Audit | 1 Cyber | remove 3 compute from one adversary project |
| Researcher | Disrupt | 1 Cyber + 1 research cube | remove up to 2 Troops in one country |
| Researcher | Meltdown | 2 research cubes | destroy one of your research stations anywhere; gain 1 antimatter |
| Researcher | Synthesis | 1 insight | draw 1 card |
<!-- /generated -->

**Sacrifice is late-game liquidation.** Once the cards needing a tag are played, its tags turn into engine rewards,
each payoff priced to be worth the card: 1 chip (r83: worth 2 before), 2 energy, up to 2 troops removed in a hex (Interdict, r98), a card and an agent removed (Debrief, r91); a tag and an
energy buy a building (r35).

**Each power's tag is its story (r93, Matt, amendment 9513).** A tag-paid power is paid in the tag its effect fits
(the tag table is below): the Protector's Accord in Diplomacy (trust), its Debrief and
Interdict in Security (force, protection). The one exception is the Researcher's Disrupt, paid in Cyber: it
hacks troops anywhere in a country, no reach.

**Node powers' riders (r93, candidate; Matt, message 9565).** A node power places next to your nodes as any placement
does, and its bonus comes only when the new node is **on or next to a fresh spot of its board's kind**, one none of your
other nodes is on or next to: Buy In's chip by a factory, Accord's ambassador by a city (a new ambassador onto that
city, standing toward its claim), Peer Review's insight by a research station (Peer Review costs 1 research cube, 3
before). With no such spot it still places, without the bonus. Why fresh: a city, a factory and a station are on or
next to almost every hex, so "on or next to" alone was met on 99-100% of placements (212's screen); a fresh spot, about
72%, pays for reaching a new one. Nothing places a node anywhere; "within range 2 of one of your nodes"
is kept for a stronger version (a track upgrade or a card), never a board's own power (Matt), and no content prints it yet.

| Tag | Story | Its usual effects (the tag story table of scripts/design_check.json) |
|---|---|---|
| Security | force and protection | troops and agents removed, pushes, shelters, cordons, hardening, shields on node hexes |
| Diplomacy | cooperation: trust, Unity, green cleanup | ambassadors, Unity Sway, cleanses, land lifts, gifts |
| Science | discovery | insight, research cubes, stations, megaproject cubes, antimatter physics, delays |
| Industry | building | chips, factories, capital, free buildings |
| Power | energy and antimatter | energy, antimatter, the weapons it feeds |
| Cyber | the network | botnets, compute drains, firewalls, node placement, information (insight) |

**Tags are story, not rules (Matt, message 9548):** no tag owns an effect, as in Magic's colors. The table is what a
player expects; a card whose effects fit none of its tags is a soft design warning (`design_check.py`'s tag-story),
never refused. Draws, credits, triggers, threat counters, liberation and a tag's own payoffs fit any tag.

**Card throughput:** the Builder's Procure turns a spare chip into 2 cards; the Researcher's Synthesis turns 1 insight
into a card (r100; 2 before) (r91, Matt: the Researcher's draw outlet; Fieldwork, 1 energy for 3 cards, is cut, since it starved it).

**Troop powers (r91, amendment 9247: early troops decide games).** Each board removes or stops troops its own way, no
two alike: the Protector's **Interdict** (a Security card: up to 2 troops in one hex in reach; r98, Matt's ruling: Stand Down
merged into it and Shelter cut, 9 powers to 7) beside its Patrol (pushes them away); the Builder's **Fortify** (2 chips: 1 troop in reach and a facility
hardened); the Researcher's **Disrupt** (a Cyber card and a research cube: up to 2 troops in one country, no reach
needed). The standard action Remove a troop (`troop_credits` 10 credits, `troop_per_round` once a round) is the hedge.

**Antimatter: each board a rare burn of its own engine (r97, Matt's ruling, 2026-11-13).** The steady source is the
tier-2 converter plants ([economy.md](economy.md)); each board's own power is a different decision, once a round, for
1 antimatter: the Builder's **Teardown** (1 chip: destroy one of your own factories, and a Fallout token goes on
its hex), the Researcher's **Meltdown** (2 research cubes: destroy one of your own research stations; r98: renamed from
Overload, a tier-2 threat's name), the Protector's **Reclaim** (227: renamed from Seize; a Security card, 6 credits and 2 energy: turn a
corrupted station or factory in reach neutral, its adversary cube off, well above Remove a troop's 10). Teardown and
Meltdown reach anywhere. A burned piece is gone, no protection asked (you chose it), unless rebuilt the
normal way; with no piece or target the power can't be used. No power spends antimatter: the spends are the majors,
tier-2 cards and a few dear tier-1 ones.

## Insight conversions (candidate, r41)

Insight is this round's attention, readable off the board: it pays keeps and plays ([player-cards.md](player-cards.md)),
and credits never buy a keep (r82). The card-play track prints it as totals, Spirit Island style (r82): 3 at its
start, then the new total on each space that raises it, each board at its own rate (r87: the Builder to 10, the
Protector and the Researcher to 9 (r100; the Researcher 8 before); 7 from the first space (r95; the Researcher 6 until r100), then a step at a time); a seat's insight is the number on the rightmost space it has
uncovered. The economy track prints none. Every player has the
cube converter, a standard action with no limit a round ([economy.md](economy.md)). **Insight must bind** (Matt,
2026-11-12: about 6 a round to drafting and 2-4 to plays is the target); r91 cut the Builder's Focus (2 credits → 1
insight, used 0.01-0.04 a Builder game: an uninteresting conversion), so the boards' own insight ways are:

- **Protector, Vigil** (an Effect of Wall, not a power; capped, r79, Matt's ruling: some cap is locked, its size a
  candidate): the first two troops each round that end their march next to your node pay 1 insight each (`board.vigil_cap`
  troops a round, 2 since r84, 192; 1 at r79-r83); later ones that round pay nothing, so a troop parked by your node no longer pays every march.
  Any march counts (a threat's immediate march too); a troop used up striking has no end to its march. An Effect
  because it happens in a march, outside the action phase where powers are used: the march's (phase 7) pays this
  round's keeps and plays, an immediate march's (phase 18) comes after them. (r77-r78 paid every troop every march.)
- **Researcher, Synthesis reversed** (r91, Matt, 2026-11-12; a power, once a round): 1 insight → draw a card (r100: was 2), the
  Researcher's draw outlet and a use for Patience's bank. No other insight → card action exists. Give / get 4 for 5 at
  the anchors (a card kept at 1 insight is 4 more): a draw, not a gain. Peer Review, its node power, pays 1 insight by a fresh station.

**For new players.** Insight is what you spend to keep and play cards: each round, the number on the rightmost space you've uncovered on
your card-play track (3 at the start). When you
are short, turn a research cube into 1 insight at any time (Cintamani's converter gives 2), and use your board's own
way: the Protector gains 1 each of the first two times each round a troop ends its march next to its node (Vigil), the
Researcher gains 1 with each node Peer Review places by a station new to its nodes and may spend 1 for a card (Synthesis). Insight you don't spend still goes at the round's end (the Researcher
keeps all of it), but a few threat counters offer insight (3 at tier 1, 4 at tier 2: [threats.md](threats.md) *Counters*),
so spare insight can stop one.

**Insight loops (r41).** Nothing turns insight back into what makes it: insight buys cards, and a card that pays insight
(a trigger, an income) is capped by plays and slots. No card makes research cubes since r44 (Sample Repository
was cut), so cubes → insight → cards → cubes has no card in it. Every megaproject converter is once a round (r75, Matt's
ruling 9; only Onyx's was), which caps Onyx's loop: 1 cube → 1 chip → a 1-chip station → 1 cube a round once the
station's card is complete, which would otherwise double its cubes each round, capped only by open hexes in reach.

## Shelters and hardening

A **shelter** (a card's "shelter one country", no reach needed; r98 cut the Protector's Shelter power) lasts until cleanup (phase 20) of the round
it is used in. Used in the actions, it covers the rest of the actions and what comes after them: the income step,
growth, the threats that execute and war (phases 15-19). The march (phase 7) has already happened, but a threat's
immediate march (Total War's, phase 18) meets it. It stops a destruction automatically, spending nothing, and so does the Protector's Aegis Doctrine capstone, a shelter
on its own pieces once a round (r82: the first time each round one of its nodes or facilities would be destroyed); the
other protections (a hardening token below, your own ambassador) are the piece owner's choice; drones never protect
(r81, friction #51) ([war.md](war.md), *Destruction*).

Hardening (the standard actions, cards) goes only on facilities a token can protect, player-owned stations and
factories that are not corrupted, and "harden N" is N different facilities, fewer if fewer qualify. A card's (a shield, usually Security's)
hardening token on a hex of your nodes (one name for the token, r81) absorbs a destruction there and goes with the node if it is
lost another way. On your own facility it is the facility's hardening token: a table has one token, so it stays
when the node goes, and protects the facility.
