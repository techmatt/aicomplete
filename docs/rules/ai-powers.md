# AI-board powers (module `ai`; [`content/ais.json`](../../content/ais.json))

*Each board's powers (node, sacrifice and insight), the boards' insight conversions, shelters, hardening and shields.
The boards, their special rules and tracks are in [ai-boards.md](ai-boards.md); terms and conventions in
[README.md](README.md).*

**Powers (locked: tag costs are sacrifices).** A power's cost names tags (`any` for any tag) and optional other
resources; in the table a tag in the cost is a sacrifice, and thresholds are for powerful cards only. It is paid by
discarding played cards that carry those tags: a program from your play area, or a building **with its whole stack**
beneath it; events can't be sacrificed (they are gone once played). A card pays with all its tags, each tag first
toward its own requirement, then toward `any`; extra tags are wasted. Each power is usable once a round. What can be sacrificed is fixed when the power is used; the player chooses the cards one at a time among those that
still help.

<!-- generated: powers -->
| Board | Power | Cost | Effect |
|---|---|---|---|
| Builder | Mint | 3 credits | gain 1 chip |
| Builder | Deploy | 1 energy | place 1 node |
| Builder | Buy In | 1 energy + 6 credits | place 1 node |
| Builder | Scrap | 1 Industry | gain 2 chips |
| Builder | Overcharge | 1 Power | gain 2 energy |
| Builder | Rush Build | 1 Industry + 1 energy | play a building from your hand, paying no credits |
| Builder | Focus | 5 credits | gain 1 insight |
| Builder | Procure | 2 chips | draw 2 cards |
| Protector | Bulwark | 1 energy | harden 1 facility in reach |
| Protector | Deploy | 1 energy | place 1 node |
| Protector | Accord | 1 Diplomacy + 1 energy | place 1 node |
| Protector | Stand Down | 1 Security | gain 3 credits |
| Protector | Recovery | 1 Security | clear a corruption marker in reach |
| Protector | Shelter | 1 Security + 1 energy | in one country this round, no nodes or facility can be destroyed |
| Protector | Debrief | 1 Diplomacy | draw 2 cards |
| Researcher | Fieldwork | 1 energy | draw 3 cards |
| Researcher | Deploy | 1 energy | place 1 node |
| Researcher | Peer Review | 3 research cubes | place 1 node |
| Researcher | Grant | 3 credits | add 1 research cube in your color to an incomplete megaproject |
| Researcher | Field Station | 1 Industry | build a research station in reach, paying no chips |
| Researcher | Audit | 1 Cyber | remove 3 compute from one adversary project |
| Researcher | Particle Run | 1 Power + 1 energy | gain 1 antimatter |
| Researcher | Synthesis | nothing | discard a card from your hand: gain 2 insight |
<!-- /generated -->

**Sacrifice is late-game liquidation.** Once the cards needing a tag are played, its tags turn into engine rewards,
each payoff priced to be worth the card: 2 chips, 2 energy, 3 credits, a marker with its Sway, 2 cards (Debrief, 108); a
tag and an energy buy a building, a sheltered country or an antimatter (r35). **Card throughput (108):** the Builder's
Procure turns 2 spare chips into 2 cards, as the Researcher's Fieldwork turns an energy into 3.

## Insight conversions (candidate, r41)

Insight is the turn's attention: it pays keeps and plays ([player-cards.md](player-cards.md)), and keeps stay in insight
(credits stand in only for keeps, at `insight_credits` an insight). Each board prints 5 a round. Every player has the
cube converter, a standard action with no limit a round ([economy.md](economy.md)); each board adds one conversion of its
own currency:

- **Builder, Focus** (a power, once a round): 5 credits → 1 insight, the keep's credit rate opened to plays. At the
  agents' values (a credit 1, an insight 2.5) it reads as a loss: it pays when one insight short stops a play or a keep.
- **Protector, Vigil** (an Effect of Wall, not a power): when your Wall stops a troop, gain 1 insight. A troop is
  stopped when a hex next to it holds your node and something it would strike there but for the Wall (a facility any
  player owns, or the node once troops strike nodes); once a troop a march. An Effect because it happens in the march (phase 5),
  outside the action phase where powers are used, and the insight pays this round's keeps and plays. The adversary's
  troops cap it.
- **Researcher, Synthesis** (a power, once a round, no price but the card): discard a card from your hand, gain 2
  insight; Patience banks it. With Fieldwork (1 energy → 3 cards) it turns 1 energy into 2 cards and 2 insight: about 4.5
  credits into 13 at the agents' values. Each is once a round, which caps the loop. A tier-1 card kept for 1 insight and never
  played returns 2; a tier-2 keep (3, or 2 upgraded) is not refunded in full.

**For new players.** Insight is what you spend to keep and play cards, and every board now gets 5 a round. When you
are short, turn a research cube into 1 insight at any time (Cintamani's converter gives 2), and use your board's own
way: the Builder pays 5 credits for 1 (Focus), the Protector gains 1 each time its Wall stops a troop (Vigil), the
Researcher discards a card for 2 (Synthesis). Insight you don't spend still goes at the round's end (the Researcher
keeps it), but four threats now take 5 insight to counter, so spare insight can stop one.

**Insight loops (r41).** Nothing turns insight back into what makes it: insight buys cards, and a card that pays insight
(a trigger, an income) is capped by plays and slots. No card makes research cubes since r44 (Sample Repository
was cut), so cubes → insight → cards → cubes has no card in it. Onyx's converter keeps its once a round: 1 cube → 2 chips → a 2-chip station → 1 cube a round
once the station's card is complete, a loop that would double its cubes each round, capped only by open hexes in reach.
The other three converters drop the limit (Horizon's needs 2 energy a use; Biomia's 2 cubes a Sway; Cintamani's only
makes insight).

## Shelters, hardening and shields

A **shelter** (the Protector's Shelter, or a card's "shelter one country") lasts until cleanup (phase 18) of the round
it is used in. Used in the actions, it covers the rest of the actions and what comes after them: troops from bases,
war, production and the threats that execute (phases 13-17). The march (phase 5) has already happened, so a shelter never meets it.

Hardening (the standard actions, cards) goes only on facilities a token can protect, player-owned stations and
factories that are not corrupted, and "harden N" is N different facilities, fewer if fewer qualify. A Science card's
**shield** is the same token on a hex of your nodes: it absorbs a destruction there and goes with the token if it is
lost another way. On your own facility a shield is the facility's hardening token: a table has one token, so it stays
when the node goes, and protects the facility.
