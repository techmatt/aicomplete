# The card grammar

*How a card's printed words resolve. Terms and conventions are in [README.md](README.md).*

(**locked**; `crates/aic-core/src/face.rs` generates every face from the card's ops, so a card's words can't differ
from how it plays.)

**Where it sits is what it is.** The top strip holds the credit octagon, the insight hexagon, requirement plates
(yellow: at least that many of a tag, never at most; stacked cards: built over) and tags, and nothing else. Above the art, on buildings
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
move is onto one piece. The shield is one stop on one piece (a hardening token; with a node, a shield); the roof
covers many (Shelter's country; with a node, all your pieces). A node with a struck-through coin is placed free. The text says what the icons
say, in order, opening with the box's word.

Card details beyond the card text:

The card set: the draft decks are `configs/baseline.json`'s card list (content holds more, marked "not in deck" in
cards.md); their counts are the generated *draft set in numbers* line in [player-cards.md](player-cards.md). Income
raises are on programs, the engines' triggers, Actions and Effects on buildings; each tag is primary on 24 to 29. Every card is in [cards.md](../cards.md)
(generated), priced in [card-pricing.md](../card-pricing.md). How each primitive resolves:

- **Reach:** a card's map target is in your reach (within 1 of your node) where its text says so; a removal in one
  country (a tier-2 sweep) is not. A removal (`damage`) takes a hex in reach holding adversary
  pieces; a country removal any country holding them. A card names the pieces it removes: where it says "2 agents or 1 troop" you choose one kind.
- **Hardening:** a different facility in reach for each token, each one a token can protect ([ai-powers.md](ai-powers.md)).
- **Factories:** "build a factory" is always on a free industrial zone in reach; the factory is yours (your cube). A
  building that builds one can be played with no such zone and then builds nothing.
- **Moving ambassadors** (`move_ambassadors`, `move_one`, `consolidate`, `spread_ambassadors`): onto pieces (city,
  base, facility or node), each arriving as in [ambassadors.md](ambassadors.md); stop at any time. "Move up to N" (Summit
  Caucus, Liaison Corps) is a "you may move": it never vetoes a card that does something else.
- **Convert, firewall** (`convert`, `firewall`; house terms): a city in reach with agents on or next to it; a land
  hex in reach. How each resolves is in [ambassadors.md](ambassadors.md), *Convert and firewall*.
- **Gather, push** (`gather`, `push`; house terms, r70): a land hex in reach, with a piece the card names next to it
  (gather) or on it (push). A strike (`gather` with `strike`) moves nothing: it removes up to N of the pieces it
  names on or next to a land hex in reach, your choice. How they resolve: [ambassadors.md](ambassadors.md), *Moving pieces*.
- **"With N X tags, also …"** (`if_tags`): the rest resolves only if you hold N tags of X on your cards when
  the card resolves (an event's own tags never count, as for a gain per tag); it never blocks the card.
- **Peek:** the top cards of the adversary deck, behind the face-up card, are shown; one fewer after each reveal.
- **Jam:** the chosen threat needs less of each resource (not below 0) while in play; if that leaves it fully paid, it
  is countered at once. **Counter a threat:** partial payments on it are refunded. **Delay N threats** (Predictive
  Modeling, Early Warning Drill): only threats you may counter, as countering and jamming; with none, the card still
  plays for its draw or shields.
- **Cleanse:** take the adversary cube off a corrupted research station or factory in reach, one hex at a time (some
  cards: all in the first hex's country). A station becomes neutral. `only` limits it to stations (Reclamation Crew,
  priced as a station flip) or to factories (Back-Channel Talks, cheap); its cube icon then carries that facility's glyph. With `cities` (Great Liberation) a corrupted
  city counts too (its cube carries the city glyph), and it is liberated ([ambassadors.md](ambassadors.md)).
- **Liberate:** liberate one corrupted city in your reach as the Liberate action does, unpaid
  ([ambassadors.md](ambassadors.md)): its agents and the adversary's cube come off, Unity sways, your cube goes on
  (its icon line always ends → cube). Each
  engine's twist: `ambassadors` (gain that many), `troops` (up to that many adversary troops in the city's country come
  off), `botnet` (a botnet on or next to the city comes off; its icon carries the city glyph). Only the liberate capstones print it ([player-cards.md](player-cards.md)).
- **Clear a city (`clear_city`):** remove up to N agents (Amnesty Decree: 4) around one corrupted
  city in reach, on it or next to it, your choice which (with no more there than the card takes, all go). Each on the city adds one to the Liberate action's bill ([ambassadors.md](ambassadors.md)); those
  next to it don't, so clearing them only thins what a Corrupt line could move onto it.
- **Card text:** extra costs and lowered income included; a long card sets smaller. **Shelter any country** (no
  reach needed): nothing in it, facility or node, can be destroyed this round.
- **Projects:** a drain removes compute from one visible adversary project; **drain all** from every visible one; a
  discard takes one with its compute, and its slot refills at once from the deck.
- **Botnets:** removing a botnet (tier 2, in reach) is, with Decommission (a Unity card, [alignment.md](alignment.md)), the only way to remove
  one ([adversary.md](adversary.md)); cutting one off (`cut_botnet`; no content prints it) makes it produce no compute in the next compute step.
- **Draw:** every draw takes the top cards of the tier-1 deck into your hand, no keep paid. An empty tier-1 deck
  reshuffles its discard; with both empty it does nothing, and a card that only draws (Fieldwork) is
  not offered. A played event goes to its played pile, never the discard, so no reshuffle brings it back.
- **Count as power (`gain_per_tokens`; here only):** gain 1 of a resource per N tokens here, on the Action's own
  card; the tokens stay.
- **Cash out (`cash_out`; one card):** remove every token of a type from one card (an Action's own, "here"; a
  program's, one of your cards you pick); gain the card's rate of a resource for each.
- **Per tag (`gain_per_tag`):** once, 1 of a resource per N tags of a kind on your cards in play; a program counts
  itself (it enters play before it resolves) and prints "(including this)" after the tag, an event doesn't.
- **Troops or ambassadors in reach (`remove_troops`, `remove_ambassadors_country` with `reach`; Matt's ruling
  of 2026-11-04):** remove up to N from any hexes in your reach, across countries, your pick of hexes when there are
  more than N. **In one country** (the same ops without `reach`): up to N in one country, any country, the same way;
  only a few big tier-2 sweeps print it, as a deliberate power step. Neither reads "per hex": Matt's ruling 6.
- **Help cards** (keyword `help`, ~10%): the player chooses another player; both gain, the helper paying. **In a
  1-player game, "another player" means you** (Matt's ruling of 2026-11-04): the gift is yours.
- **Moves of "up to N hexes"** (`move_one`'s `range`; no deck card prints one) count hex distance (`Map::distance`,
  over land and sea); the destination must be a piece.
- **Cordon (`cordon`):** "cordon a country until the end of the next adversary turn": until then no Rally or
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
- **Teammate triggers (`team_play`):** "When another player plays a Security card, remove 2 agents in reach"
  (Joint Task Force): what it removes is the owner's, so the owner's "when you remove agents" triggers
  fire for it, not the player's whose card set it off. With `both`, "you both gain" (Consulate Tower, Substation Hub), the dashed box holding the other's gain. Your own plays don't fire them, except in a 1-player game
  (*Help cards*, above): they fire on your plays and a shared gain lands once.
- **Rebate (`play_cost`):** "when you play a card printed at 10 credits or more, gain 5 credits" (Lights-Out Assembly):
  the printed cost, before discounts. `≥`N is at least N.
- **Fight triggers:** "when you remove a Troop" (Security Contractor HQ), "when you remove adversary compute"
  (Security Ops Floor) and "when you remove Agents" (Honeypot Relay) fire once for the card, Action, power, converter
  or standard action that removed any, however many, canceling an arrival included.
- **Placing tokens (`place_tokens`):** "when you place a Dataset on one of your cards" fires once a placement, however
  many tokens (no deck card prints it); a trigger's own tokens count too.
- **Discounts (`card_discount`):** "your Cyber cards cost 5 credits less to play" (Darknet Exchange), or insight (no
  deck card prints it). A card with the tag twice is discounted once. Like every discount, never
  below `discount_floor`, and a cost under it stays.
- **Threshold counter (`threshold`):** a trigger adds a token here; **At N** (a house term): whenever the tokens here reach
  N, whatever put them there (a feeder card's too), N come off and the reward happens (Telemetry Archive: a
  Science card played adds 2 Datasets; at 4, 3 insight).
- **Copy (`copy_income`, a program):** raise your income by one of your buildings' Each round output, as it pays when
  you play the copy (a per-tag income at its count then); the building may be covered or sacrificed later. No card
  prints it.
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
