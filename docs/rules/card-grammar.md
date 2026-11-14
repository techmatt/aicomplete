# The card grammar

*How a card's printed words resolve. Terms and conventions are in [README.md](README.md).*

(**locked**; `crates/aic-core/src/face.rs` generates every face from the card's ops, so a card's words can't differ
from how it plays.)

**Where it sits is what it is.** The top strip holds the credit octagon, the insight hexagon, requirement plates
(yellow: at least that many of a tag, never at most; stacked cards: built over) and tags, and nothing else; the
**target box**, a tab under the title, holds a map card's target (r106). Above the art, on buildings
only, is what lasts while the building is active: income, *Effect:*, *Action:*. Cover the building and it stops.
Below the art is what happens once, when you play the card: extra costs first, then gains, placements and moves.

**Boxes.** No box: gain it once, now. A brown box is income: above the art it pays each round ("Each round, ...");
with a track arrow (below the art, or after an Effect's `:`) it moves your income track once ("Raise your X income by N" up, "Lower ..." down). A minus
before an icon is pay or lose, only that: every paid cost prints it, an Action's too. A dark frame acts on the adversary's pieces, projects or threats. A dashed outline is
another player. A thin solid outline is your cards: a token in it goes to, or comes from, one of your cards (nothing
counts tokens across cards). No outline is you, or this card.

**Connectors.** `→` is an Action: once a round, pay the left to get the right; a bare `→` is free ("Action: get
..."), and needs no "once a round": every Action is. `:` is an Effect: when the left happens, get the right. `/` is
per. OR is written out; a line opening with OR is the other half of an Action, one use a round between them. No card
has an exception: every symbol means what it always means ([card-guidelines.md](../card-guidelines.md)).

**Shapes.** Square tiles are resources, discs are tokens on cards (alone, on this card), bare silhouettes are pieces
on the board and moving them (each piece one symbol: ambassador, agent, node, botnet, troop, drone), ringed circles are
tags. A number goes before its icon, no number means 1. The less mark (a number over a double chevron) is a discount
or, before the jam mark outside the dark frame, a jam. A ringed number before discs is At N. The clock is a delay only;
the wall is a firewall (agent, hex) or cordon (agent, country). `≤`N is up to N, your choice: a give. A pin after a
move is onto one piece. The near mark is on or next to the target. The shield icon is one stop on one piece (a
hardening token); the roof covers many (Shelter's reach; Aegis Doctrine's pieces). The text says what the icons
say, in order, opening with the box's word.

Card details beyond the card text:

The card set: the draft decks are `configs/baseline.json`'s card list (content holds more, marked "not in deck" in
cards.md); their counts are the generated *draft set in numbers* line in [player-cards.md](player-cards.md). Income
raises are on programs, the engines' triggers, Actions and Effects on buildings; each tag is primary on 24 to 29. Every card is in [cards.md](../cards.md)
(generated), priced in [card-pricing.md](../card-pricing.md). How each primitive resolves:

- **Target** (r106, **locked**, Matt 2026-11-14): a card that touches the map names **one target**, a hex in your
  reach (within 1 of any of your nodes; no card prints a range). The **target box** in the header prints its
  condition with its glyph ("Target: a corrupted city"); you pick the hex when the card (or its Action, or its
  Effect each time it fires) resolves. Its effects act **on it**, or **on or next to it** where printed, or **in its
  country** where printed; nothing reaches past 1 from it, no card picks a country freely, and no card has two
  targets: one that wants two places says "it and N next to it" (Back-Channel Talks: "free it and 1 corrupted
  factory next to it"). With no hex meeting the condition the card's map effects do nothing; its other parts still
  play. `target` is an op opening the list it scopes; `Content::check` refuses a map op with no target before it,
  two targets, or a free country. A card names the pieces it removes: "3 Agents or 1 Troop" is one kind, your choice.
- **Gain, Place:** Gain puts pieces in your supply; Place puts new ones onto the map, on the target (Amnesty Decree:
  "Place 2 Ambassadors on it"), each arriving as any ambassador does.
- **Hardening:** a different facility for each token, each one a token can protect ([ai-powers.md](ai-powers.md)):
  the target, then facilities next to it.
- **Factories:** "build a factory on it": the target is a free industrial zone; the factory is yours (your cube).
- **Moving ambassadors** (`move_ambassadors`, `consolidate`): onto the target (or onto pieces on or next to it),
  each arriving as in [ambassadors.md](ambassadors.md); stop at any time. "Move up to N" (Summit Caucus, Liaison
  Corps) is a "you may move": it never vetoes a card that does something else.
- **Convert, firewall** (`convert`, `firewall`; house terms): the target is a city with agents on or next to it; a
  land hex. How each resolves is in [ambassadors.md](ambassadors.md), *Convert and firewall*.
- **Gather, push** (`gather`, `push`; house terms, r70): onto the target, each from a hex next to it (gather), or
  from it (push). A strike (`gather` with `strike`) moves nothing: it removes up to N of the pieces it names on or
  next to the target, your choice. How they resolve: [ambassadors.md](ambassadors.md), *Moving pieces*.
- **"With N X tags, also …"** (`if_tags`): the rest resolves only if you hold N tags of X on your cards when
  the card resolves (an event's own tags never count, as for a gain per tag); it never blocks the card.
- **Peek:** the top cards of the adversary deck, behind the face-up card, are shown; one fewer after each reveal.
- **Jam:** the chosen threat needs less of each resource (not below 0) while in play; if that leaves it fully paid, it
  is countered at once. **Counter a threat:** partial payments on it are refunded. **Delay N threats** (Predictive
  Modeling, Early Warning Drill): only threats you may counter, as countering and jamming; with none, the card still
  plays for its draw or hardening tokens.
- **Cleanse:** take the adversary cube off the target, a corrupted research station or factory, then off the rest
  the card counts next to it (or in its country), one hex at a time. A station becomes neutral. `only` limits it to stations (Reclamation Crew,
  priced as a station flip) or to factories (Back-Channel Talks, cheap); its cube icon then carries that facility's glyph. With `cities` (Great Liberation) a corrupted
  city counts too (its cube carries the city glyph), and it is liberated ([ambassadors.md](ambassadors.md)).
- **Liberate:** liberate the target, a corrupted city, as the Liberate action does, unpaid
  ([ambassadors.md](ambassadors.md)): its agents and the adversary's cube come off, Unity sways, your cube goes on
  (the word Liberate carries your cube). Each
  engine's twist: `ambassadors` (gain that many), `troops` (up to that many adversary troops in the city's country come
  off), `botnet` (a botnet on or next to the city comes off; its icon carries the city glyph). Only the liberate capstones print it ([player-cards.md](player-cards.md)).
- **Card text:** extra costs and lowered income included; a long card sets smaller. **Shelter** takes no target
  (r106): "Shelter your Nodes and facilities in reach this round": none of your nodes or facilities on a hex in
  your reach when it resolves can be destroyed this round.
- **Projects:** a drain removes compute from one visible adversary project; **drain all** from every visible one; a
  discard takes one with its compute; the rest slide left and the right slot refills.
- **Botnets:** removing a botnet (tier 2, the target) is, with Decommission (a Unity card, [alignment.md](alignment.md)), the only way to remove
  one ([adversary.md](adversary.md)); cutting one off (`cut_botnet`; no content prints it) makes it produce no compute in the next compute step.
- **Draw:** every draw takes the top cards of the tier-1 deck into your hand, no keep paid. An empty tier-1 deck
  reshuffles its discard; with both empty it does nothing, and a card or power that only draws (Procure) is
  not offered. A played event goes to its played pile, never the discard, so no reshuffle brings it back.
- **Count as power (`gain_per_tokens`; here only):** gain 1 of a resource per N tokens here, on the Action's own
  card; the tokens stay.
- **Cash out (`cash_out`; one card):** remove every token of a type from one card (an Action's own, "here"; a
  program's, one of your cards you pick); gain the card's rate of a resource for each.
- **Per tag (`gain_per_tag`):** once, 1 of a resource per N tags of a kind on your cards in play; a program counts
  itself (it enters play before it resolves) and prints "(including this)" after the tag, an event doesn't.
- **Troops or agents (`remove_troops`, `remove_ambassadors_country`):** up to N on the target, or on or next to it,
  your pick of hexes when there are more than N; never "per hex" (Matt's ruling 6). Board powers and the standard
  actions, which have no target, still remove in reach (or one country, where printed).
- **Help cards** (keyword `help`, ~10%): the player chooses another player; both gain, the helper paying. **In a
  1-player game, "another player" means you** (Matt's ruling of 2026-11-04): the gift is yours.
- **Cordon (`cordon`):** "cordon its country until the end of the next adversary turn" (the target's): until then no Rally or
  creation line places agents there, a project's included (118); its pool waits in the supply and lands at its next
  Rally; the rest of a build goes on. Like a firewall it comes off at that turn's compute step. Hardpoint Lockdown prints it: a Security card that does an
  ambassador's job.
- **Play a building, paying no credits (`free_building`):** the Builder's Rush Build (1 Industry and 1 energy). Its tags are sacrificed first,
  so a building that needed those tags or the sacrificed stack can't then be played; its other costs and requirements
  stand, paid from what is left after the power's energy. The power isn't offered when no building in hand would still
  be playable after some sacrifice and that energy.

**The engine wording (candidate).** Every trigger is an Effect on a building (`:`, `when`): it fires once for each
action that sets it off (a card play, a building's Action, an AI power, a standard action), its unit printed, with
no limit a round, and resolves after what caused it. A building counts itself: playing a Science
building with a Science trigger fires it. The `play` mark is a card you play.

- **Play triggers (`when`, `play`):** "When you play a Science card, gain 7 credits" (Citation Engine); with no tag, any
  card. **Per tag** (`play_tag`): "when you play a card, gain 1 insight per tag on it" (Interdisciplinary Institute), the tier-2
  form, once for each tag of any kind on the played card (the empty ring), or per tag of one kind, the
  ringed tag after `/` on the right of the `:`.
- **Teammate triggers (`team_play`):** "When another player plays a Security card, remove 2 Agents on it"
  (Joint Task Force; the owner picks the target, in the owner's reach): what it removes is the owner's, so the owner's "when you remove agents" triggers
  fire for it, not the player's whose card set it off. With `both`, "you both gain" (Consulate Tower, Substation Hub), the dashed box holding the other's gain. Your own plays don't fire them, except in a 1-player game
  (*Help cards*, above): they fire on your plays and a shared gain lands once.
- **Rebate (`play_cost`):** "when you play a card printed at 10 credits or more, gain 5 credits" (Lights-Out Assembly):
  the printed cost, before discounts. `≥`N is at least N.
- **Fight triggers:** "when you remove a Troop" (Security Contractor HQ), "when you remove adversary compute"
  (Security Ops Floor) and "when you remove Agents" (Honeypot Relay) fire once for the card, Action, power, converter
  or standard action that removed any, however many, canceling an arrival included.
- **Discounts (`card_discount`):** "your Cyber cards cost 5 credits less to play" (Darknet Exchange), or insight (no
  deck card prints it). A card with the tag twice is discounted once. Like every discount, never
  below `discount_floor`, and a cost under it stays.
- **Threshold counter (`threshold`):** a trigger adds a token here; **At N** (a house term): whenever the tokens here reach
  N, whatever put them there (a feeder card's too), N come off and the reward happens (Telemetry Archive: a
  Science card played adds 2 Datasets; at 4, 3 insight).
- **When built:** an engine building's one-time gain prints below the art and resolves when it is built,
  before its own play fires its trigger (Telemetry Archive's 2 Datasets then the 2 its Science play adds: At 4 pays at
  once).
- **Per-tag income:** a program's "raise your credit income by 1 per 2 Science tags on your cards (including
  this)" (`production_per_tag`, Research Royalties) counts once, when it resolves, the program itself included; later tags
  don't raise it. A building's "each round, 3 credits per Science tag on your cards (including this)" (`income_per_tag` with an
  `amount`, Foundation Model Campus) counts again every round while it is active.
- **For new players:** a building with a colon pays you every time the thing on its left happens: you play a card of
  that tag, a teammate does, you remove a troop. The less mark after the colon makes those cards cheaper. A ringed
  number and discs before the colon count up on the card and pay when they reach it.
- **Reuse (`reuse`, an Action):** use again another of your buildings' Actions you used this round, paying its cost
  again, either half of an "A, OR B" Action, not only the half you used (Matt, r48: a combo with a strong Action is
  fine; a card it breaks is redesigned); never one whose own Action is a reuse (two would loop) (Simulation Cluster).
