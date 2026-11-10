# cards (player cards: opening, draft, keeps, play, slots, tags, sacrifice, selling, tokens, Effects/Actions, events, liberate capstones, icon grammar): engine-verified rules (172 verifier)

**Numbers** are baseline config (`configs/baseline.json` cards module, lines 176-191; `ai.slots` 168-172; `board.sell_credits` 39) and `content/cards.json` (168-card deck: 113 tier 1, 55 tier 2).

**Decks and piles**
- Two decks, tier 1 and tier 2, one copy of each card, shuffled at setup (modules/cards/mod.rs:279-297).
- Each tier has its own discard pile. Cards go there when sold, sacrificed, discarded from hand, or drafted and not kept (deck.rs:6-11).
- A played event goes to its owner's played-events pile, never to a discard, so it never comes back (play.rs:273-296).
- When a deck is empty and a card must come from it, shuffle that tier's discard into a new deck. If the discard is also empty, nothing is dealt (deck.rs:34-46).
- There is no hand limit.

**Round 1: the opening** (mod.rs:349-362; price.rs:17-25, 36-38)
1. Each player is dealt 8 tier-1 cards (`opening_deal`) and 2 tier-2 cards (`opening_deal_tier2`). There is no draft this round.
2. Keep cards one at a time:
   - Your first 4 tier-1 keeps are free (`opening_free`). Each tier-1 keep after that costs 5 credits (`opening_keep_credits`).
   - Each tier-2 keep costs 1 insight (`opening_keep_tier2`). Tier-2 keeps don't use up any of the 4 free keeps, and Science tags don't discount them.
3. When the action phase starts, cards you didn't keep go to the discard (mod.rs:333-342).

**Rounds 2 and later: build, draft, keep**
1. **Build a hand** (`cards.tier`). Each player takes 4 cards (`deal`) one at a time. For each card you choose the tier-1 or tier-2 deck, and it comes face up, so you see it before your next choice. You can't pass. A tier is offered only while its deck or discard has a card. If both tiers run out, every hand stays as it is (play.rs:211-228; mod.rs:365-374, 419-421).
2. **Draft** (`cards.draft`). Everyone takes 1 card from the hand in front of them. Then all the hands pass at once: to the next seat (s+1) in odd rounds, to the previous seat (s−1) in even rounds. The first draft is round 2, so it passes to s−1. Repeat until the hands are empty. You can't pass (play.rs:351-374).
3. **Keep** (`cards.keep`). Keep any of your drafted cards:
   - A tier-1 card costs 1 insight (`keep_tier1`).
   - A tier-2 card costs 2 insight (`keep_tier2`), or 1 (`keep_tier2_upgraded`) once your AI board's keep-upgrade space is uncovered (ai.rs:108).
   - Science discount: tier-2 keeps only.
   - If you are short of insight, each missing insight costs 5 credits (`insight_credits`). You must spend all the insight you have first, and this credit option is for keeps only (play.rs:10-16).
   - If the team has an Open Market free keep, it is used automatically on the first keep (by any player) that would cost something (play.rs:235-237; alignment.rs:125).
   - Unkept cards go to the discard when the action phase begins.

**Playing a card** (action phase, as often as you can pay; play.rs:29-65, 180-207)
- **Requirements** (`needs`, the yellow plate): you must have at least that many tags of each listed kind. The card's own tags don't count, because it is still in your hand. Requirements are checked only when you play (play.rs:20-26).
- **Credit cost:** the printed credits, less your discounts.
  - Buildings only: 1 credit off per 2 Industry tags (`industry_every`), plus any building discount (Master Plan, a board capstone).
  - Any card: "your X cards cost 5 credits less" buildings (4 in the deck, all credits). A card is discounted once per tag kind, even if it shows that tag twice.
  - (price.rs:119-169)
- **Insight cost:** 0, 1 or 2, as printed (25, 112 and 31 cards), less 1 per 2 Science tags (`science_every`). It comes from the same insight as keeps, and you can't buy it with credits (price.rs:82-87).
- **Discount floor:** no discount takes a cost below 1 (`discount_floor`). A cost printed below 1 (0) stays 0 (price.rs:76-79). So in practice, with 2 or more Science tags every 2-insight cost becomes 1.
- **Other costs:** you also pay any printed energy (14 cards), chips (1), antimatter (4) or research cubes (1).
- **Lowered income:** a card can lower an income ("lower your credit income by N"). Your total income of that resource must be at least N to play it. No card in the deck uses this (play.rs:138-146, 311-313).
- **Playable only if something happens:** a program or event is offered only if at least one of its parts has something to act on. A part with nothing to act on just does nothing; the rest still resolves (feasible.rs:8-128; play.rs:42-48).
  - When checking a program's "gain per tag", count its own tags. Never count an event's own tags (feasible.rs:147-161).
  - A building can always be played; its when-built effect is not checked (play.rs:39-41).
- **Order of play** (play.rs:265-298):
  1. Pay.
  2. The card enters play: a building goes to a slot, a program to your play area, an event is set aside.
  3. Its printed effects resolve in order.
  4. Every player's play triggers fire, seat by seat.
  5. An event goes to your played-events pile.

**Card types**
- **Programs** stay in your play area. Their tags count, and they can be sacrificed.
- **Events** resolve and go to your played-events pile. Their tags never count.
- **Buildings** go into slots. Only buildings may have income, Effects, Actions or tokens; content is refused otherwise (content.rs:1058-1069).

**Slots and covering** (tableau.rs:145-172; play.rs:320-349)
- Slots come from your AI board: Builder 5, Protector 4, Researcher 3 (`ai.slots`). A building that adds slots exists in the rules, but no card prints one.
- With a slot free, you choose: an empty slot, or build over any of your buildings. With no slot free, you must build over one.
- A building with `replaces` must go over a building whose top card has that tag, even if a slot is free. Only 3 deck cards have it: Orbital Embassy, Agency Directorate, Power to the People.
- When you build over a building, the whole old stack slides under the new card:
  - The covered cards' income, Effects, Actions and token counting stop.
  - Their tags still count.
  - Tokens stay on them but do nothing.
  - The stack is sacrificed as one unit.
- A building turned face down by the adversary (Zero Day) does nothing until it turns up, but its tags still count (mod.rs:137-192; tableau.rs:14-24).

**Tags** (tableau.rs:58-85)
- The six tags are Science, Industry, Cyber, Power, Diplomacy and Security. A card has 1-3.
- Your tags are those on:
  - your active buildings and the cards under them,
  - your programs,
  - your AI board, printed or revealed.
- Board tags count for everything but can't be sacrificed.

**Sacrifice** (to pay an AI-board power's tag cost; sacrifice.rs:18-189)
1. Give up sources one at a time, choosing only sources that still help pay the cost:
   - a program,
   - a whole building stack (all its tags),
   - a card in hand, only with a "tags from hand" effect, which no deck card has.
2. Each tag pays its own named requirement first. Any extra tags pay "any tag" requirements. Anything left over is wasted.
3. Sacrificed cards lose their tokens and go to the discard.

**Selling** (board standard action; board/mod.rs:319-353): discard any card from your hand for 1 credit (`sell_credits`).

**Drawing** (deck.rs:18-31): "draw N" always takes from the tier-1 deck into your hand, with no keep cost. A card whose only effect is a draw is not offered if there are no tier-1 cards left in the deck or the discard.

**Effects (triggers, shown with ":")** (triggers.rs:87-135, 186-223)
- An Effect fires every time its event happens, with no limit per round. It resolves after the thing that set it off.
- A building counts itself: playing it can fire its own trigger.
- **Play triggers:**
  - "When you play [tag] card": once if the played card has the tag.
  - "Per tag on it": once per matching tag on the played card.
  - Rebate: fires if the played card's printed cost (before discounts) is at least N.
- **Another player's play:** fires when another player plays a matching card. "You both gain" also pays that player. In a 1-player game these fire on your own plays, and a shared gain is paid once. What such a trigger removes counts as the trigger owner's removal.
- **Fight triggers:**
  - "When you remove a troop": once per troop.
  - "When you remove adversary compute" and "when you remove agents": once per action (card, Action, power, standard action) that removed any.
- **At N** (threshold.rs; triggers.rs:63-79): when a card's own tokens reach N, from any source, N come off and the reward resolves. Repeat while N or more remain. When-built tokens count, so a building's own play can pay at once.

**Actions (shown with "→")** (play.rs:149-177, 376-421; mod.rs:299-307)
- Use each building's Action once a round, paying the left side, which may include tokens from that card. Actions refresh at the start of the round.
- "A, OR B": one use a round. You pick a half you can pay that has something to act on.
- A half that discards its card exists in the rules (`once`), but no card prints one.
- Reuse (Simulation Cluster): use another building's Action again this round, either half, paying its cost again. It can't reuse another reuse card (triggers.rs:291-323).

**Tokens on cards** (tableau.rs:87-117; effects/mod.rs:204-224)
- Token types are Datasets, Followers, Capital and Operatives. Only the 9 buildings that print a type hold them, and a card counts only its own tokens.
- "Add N here": puts tokens on this card.
- Feeders ("add N to one of your X buildings"): add to one of your active buildings of that type that you pick.
- Token Actions (Synthetic Data Forge, Safehouse Network, Popular Uprising): add a token, OR spend or count the ones here.
- At N cards: gain tokens from a trigger and pay out at N.
- Threats can make you remove tokens; you choose which card each one comes from (threats/ops.rs:16-34).
- Sacrificing a card or discarding it from play removes its tokens.

**Liberate capstones**
- Eight deck cards liberate one corrupted city in your reach, each with an extra effect: 5 events, 2 tier-2 buildings (when built), and Popular Uprising, a tier-1 building whose Action is "add 1 Follower here, OR pay 4 Followers: liberate", usable again in later rounds.
- Liberating needs a corrupted city in reach. With no such city, a liberate event is still offered only if one of its other parts has something to act on.

**"With N X tags, also …"** (3 events): the extra part resolves only if you have N tags of X when the card resolves. The event's own tags don't count. It never stops the rest of the card (effects/mod.rs:235-240).

**Help cards:** "another player gains" goes to a player you choose. In a 1-player game, that player is you (effects/mod.rs:69-83).

**How the printed icons work** (face.rs)
- **Above the art:** what lasts while the building is active.
  - Brown box: income each round.
  - ":" Effects.
  - "→" Actions; their costs print with no minus.
- **Below the art:** what happens once, when played.
  - Extra costs print with a minus.
  - A brown box here moves an income track once ("+N").
- **Inside the dark frame:** marks act on adversary pieces, projects or threats.
  - Agents print as the ambassador silhouette inside the dark frame.
  - Removals have a minus.
  - A clock means a delay, a firewall or a cordon, depending on what it sits beside.
- **Dashed outline:** another player.
- **Thin solid outline:** one of your cards.
- **Disc marks:**
  - "+N disc": add tokens here.
  - "N disc :": At N.
  - "N disc →": pay tokens from this card.

## Player count and difficulty

**Player count**
- **Deck size doesn't scale.** The deck is the same 168 cards at every count. Round 1 deals each player 10 cards (8 tier-1 and 2 tier-2), and each later round draws 4 per player. With more players the decks empty sooner and reshuffle their discards sooner (deck.rs:34-46). Played events never return.
- **Draft passing** (play.rs:362-371): direction alternates by round, s+1 in odd rounds and s−1 in even rounds. Each player still ends up with 4 cards (`deal`).
  - 1 player: the hand passes back to you, so you take all 4 cards you chose. In effect you pick 4 cards and their tiers.
  - 2 players: you take every other card from your own hand and your partner's.
  - 3-4 players: each card comes from a different player's hand.
- **AI boards:** a seat's board is `boards[seat % 3]`, so a 4th player repeats the Builder, with 5 slots (ai.rs:113-133).
- **Solo:**
  - "Another player" means you (effects/mod.rs:69-83).
  - "When another player plays" triggers fire on your own plays, and a "you both gain" is paid once (triggers.rs:91, 103, 123).
  - A give to another player (`transfer`) is not counted as acting, so it can't make a card playable. No deck card prints it (feasible.rs:123).
- **Open Market free keeps** are one pool for the whole team, used by whichever player keeps first (alignment.rs:120-131). How often they come up therefore depends on player count.

**Difficulty**
- The cards module has no difficulty parameters. Every cards number (deal 4, opening 8+2, 4 free, keeps 1 and 2 insight, upgraded 1, 5 credits per insight, Science and Industry every 2, floor 1, sell 1 credit) is the same at difficulty 0-4.
- Difficulty affects cards only indirectly, through which adversary cards and threats appear and how many:
  - threats can turn a building face down (Zero Day: it does nothing, but its tags still count; mod.rs:137-192),
  - make you lose tokens (threats/ops.rs:16-34),
  - force discards (requisition).
- Board slots (5/4/3) and starting resources are set per AI board, not by difficulty.

## Mismatches (engine vs docs/rules)

- **Most tier-2 buildings have `replaces` ("A building with `replaces` (most tier-2 buildings, over their own tag ...)")** (docs/rules/player-cards.md:110-112): engine: Only 3 of the 23 tier-2 buildings in the deck have `replaces` (orbital_embassy, agency_directorate, power_to_the_people). The other 20 go into a free slot or over any building you choose. (content/cards.json (replaces field); crates/aic-core/src/modules/cards/tableau.rs:163-172)
- **Every token card "arrives loaded" ("Add 2 Operatives here" when built)** (docs/rules/player-cards.md:117): engine: Popular Uprising has no when-built effects (effects: []), so it enters play with 0 Followers. The other 8 token buildings do add tokens when built. (content/cards.json:3392 (popular_uprising); crates/aic-core/src/modules/cards/play.rs:283-284)
- **Cyber has the only standing automatic counter (Autonomous Defense Grid); "no other tag has one"** (docs/rules/player-cards.md:68-69): engine: Market Stabilization Board (tier 2, Industry, in the baseline deck) also has auto_counter: economic threats need 6 less to counter. (content/cards.json (market_stabilization_board passives auto_counter source economic amount 6); crates/aic-core/src/modules/cards/play.rs:291-293)
- **Requirement plates: yellow means at least, red means at most** (docs/rules/card-grammar.md:9; docs/card-guidelines.md Legend (Requirement plates row)): engine: The engine has no at-most requirement. CardDef has only `needs` (at least), it uses deny_unknown_fields, and requirements_met checks only have >= n. The `at_most` field exists only in the web renderer. (crates/aic-core/src/modules/cards/play.rs:20-26; crates/aic-core/src/content.rs:338-382; web/card/card.js:55)
- **In the dark frame, a clock is a delay (threats wait a round); Legend: "Clock | delay: a threat waits a round"** (docs/rules/card-grammar.md:26; docs/card-guidelines.md Legend (Clock row)): engine: The face generator also prints the clock in the dark frame for Firewall (agent+hex+clock) and Cordon (agent+country+clock). Neither one delays a threat. (crates/aic-core/src/face.rs:451-454, 567-570, 309-312)
- **Dark frame: "its numbers are Remove N"** (docs/card-guidelines.md Legend (Dark frame row)): engine: Delay prints an unsigned count in the dark frame ("N clock": delay N threats), and Jam / AutoCounter print −N jam meaning less to counter. Neither is a removal. (crates/aic-core/src/face.rs:309-312, 463-470, 855-858)
- **Playing a card costs its credits (buildings minus your discounts: Industry tags, Master Plan): implies only buildings are discounted** (docs/rules/player-cards.md:24): engine: Card discounts by tag (Grad Student Pool, Darknet Exchange, Standardized Tooling, Protocol Office: 5 credits) apply to every card with the tag, programs and events included. Only the Industry-tag and building discounts are limited to buildings. (crates/aic-core/src/modules/cards/price.rs:164-169, 190-195)
- **New-player summary: "events are used and discarded"** (docs/rules/README.md:45): engine: Played events go to the player's played-events pile, never the discard, so no reshuffle returns them. (crates/aic-core/src/modules/cards/play.rs:273-296; mod.rs:120-125)

## Hard to state

- [high] **Keep pricing: opening (4 free tier-1, then 5 credits; tier-2 at 1 insight, outside the free count, no Science discount) vs later rounds (1/2 insight by tier, upgrade to 1, Science discount on tier-2 only), shortfall at 5 credits per insight only after all your insight is spent, Open Market free keep consumed automatically and team-wide**: Three regimes with exceptions (tier-2 doesn't use free keeps; Science only on tier-2 keeps; credits only for keeps; forced insight-first; shared auto-consumed freebie). A player can't predict it from the card. Proposed: One rule for every round: tier-1 keep costs 1 insight, tier-2 costs 2, any missing insight costs 5 credits each (your choice of which to pay). Make the opening 'keep 4 tier-1 cards free' a one-line exception. Let the Open Market keep be claimed by a player, not auto-spent.
- [medium] **Science discount (1 insight off per 2 Science tags) with discount_floor 1 and a cost under the floor staying as printed**: Insight costs are only 0-2, so the scaling 'per 2 tags' only ever matters once. The floor clause is an exception ('a cost printed under it stays'). Proposed: State it as: 'With 2+ Science tags, any card or tier-2 keep costing 2 insight costs 1.' Generally: 'Discounts never take a cost below 1; a 0 cost stays 0.'
- [medium] **Own tags: never toward a card's own requirement; do count for a program's own 'gain per tag' and its 'with N tags'; never for an event**: Three cases depending on card type and on which part of the card reads tags. A forward reference to when the program enters play. Proposed: 'A card's own tags never count for anything on that card.' That changes ~1 program's (Consular Pouch-style) gain-per-tag. Alternatively, print the own-tag-inclusive number.
- [medium] **Playable only if some part acts: programs/events need one part with a target; buildings are always playable; gain-per-tag counts the program's own tags in the check; transfer needs another seat**: An exception for buildings, plus a per-op list of what 'has something to act on' means (draw needs a deck, feeder needs a matching building, etc.). Proposed: Drop the check for players: 'You may play any card you can pay for; parts with nothing to act on do nothing.' A blank play only wastes the player's own resources.
- [high] **Fight triggers: per troop for 'remove a troop', but once per action for 'remove compute' / 'remove agents'; removals by a teammate trigger inside your action count for the trigger owner, not you**: Different counting units for sibling triggers. The ownership transfer is invisible on the board, and 'action' must be defined (card, Action, power, converter, standard action). Proposed: Make all three 'once per action that removed any' (or all per piece), and print the unit on the card ('each troop' vs 'once').
- [low] **'When another player plays' triggers fire on your own plays only in a 1-player game, and a 'you both gain' pays once there**: A solo exception tucked into a trigger definition. Proposed: Put the solo rule in one place in the solo section: 'Alone, every "another player" is you; a shared gain is paid once.'
- [medium] **Slots and covering: free slot or build over any; a `replaces` building must cover a matching-tag top even with a free slot; covered cards keep tags and inert tokens; stack is sacrificed whole**: Several clauses and one exception (replaces) that applies to only 3 cards. Covered tokens persisting but inert is not visible. Proposed: Print the replace plate clearly and say 'Cards with the stacked-cards plate must be built over a building of that tag.' Discard tokens from a covered card when it is covered.
- [high] **Agents print as the ambassador silhouette, distinguished only by the dark frame**: The same figure means your piece or the enemy's depending on a frame. Convert prints both side by side (framed −N figure, then unframed N figure + node). Proposed: Give agents their own silhouette (or a color) on cards, matching the board piece.
- [medium] **The clock icon means delay (alone), firewall (agent+hex), or cordon (agent+country)**: One mark, three unrelated rules; the Legend lists only delay. Proposed: Distinct icons for firewall and cordon (e.g., a wall/barrier), keeping the clock for delay only.
- [medium] **Liaison Corps (move up to N ambassadors onto pieces in reach) and Summit Caucus (move up to N in-reach ambassadors onto one piece) print the identical icon line 'N move'**: The same icons give two different rules; a player must read the text. Proposed: Add a destination mark to consolidate (e.g., 'N move → 1 piece') or merge the two ops into one rule.
- [medium] **The minus sign has four meanings: pay/lose now (below the art), remove (in the dark frame), cheaper (after ':' in a discount), lower income (in a brown box below the art). Action costs left of → have no minus at all.**: The meaning depends on position and frame, and costs print with a minus in one place and without it in another. Proposed: Print Action costs with the same minus as play costs, or drop the minus on play costs and rely on position. Use a distinct 'less' mark for discounts.
- [low] **Shield mark: a hardening token on a facility (alone), a node shield (with node), the Aegis Doctrine (with country); versus the shelter roof (with country)**: 'Shield+country' and 'roof+country' are near-identical country protections with different rules (first destruction a round in a country you're in, vs. nothing destroyed this round). Proposed: Reserve the shield for one-destruction stops and the roof for whole-country effects. Print Aegis as roof plus '1'.
- [low] **Brown box below the art (one-time income-track move, '+N') vs above the art (paid each round, 'N')**: The same box, told apart only by its position and a plus sign. Proposed: Add an arrow/track glyph to the below-art box ('raise income').
- [low] **Discs: '+N disc' add here; 'N disc :' At N; 'N disc →' pay tokens; disc in a thin outline goes to one of your cards**: Four meanings for one icon, told apart by the connector after it. Proposed: Keep the connector rule but teach 'At N' as its own named box or ring around the number.
- [low] **Draft: you choose the tiers for a hand you will mostly pass away; direction alternates by round, the first draft (round 2) going to the previous seat**: The direction alternates by round parity, which is not tracked on the board. Proposed: Fixed direction, or print the arrow on the round track.
- [low] **Rebate triggers read the printed cost, not what you paid**: With discounts in play, a player will expect the paid price. Proposed: Keep it but print 'printed cost' in the rebate wording (the face does), and teach it once.
- [low] **Lowered-income cost (`lowers`), slot-adding buildings, tags-from-hand, place-token triggers, copy income and one-shot Action halves are in the engine but printed by no deck card**: Rules text for mechanisms the player never meets. Proposed: Leave them out of the rulebook until a card prints them.
