# The card grammar

*How a card's printed words resolve. Terms and conventions are in [README.md](README.md).*

(**locked**; `crates/aic-core/src/face.rs` generates every face from the card's ops, so a card's words can't differ
from how it plays. No card text is hand-written.)

**Where it sits is what it is.** The top strip holds the credit octagon, the insight hexagon, requirement plates
(yellow: at least; red: at most; stacked cards: built over) and tags, and nothing else. Above the art, on buildings
only, is what lasts while the building is active: income, *Effect:*, *Action:*. Cover the building and it stops.
Below the art is what happens once, when you play the card: extra costs first, then gains, placements and moves.

**Boxes.** No box: gain it once, now. A brown box is income: above the art it pays each round ("Each round, ...");
below the art it moves your income track once ("Raise your X income by N", "Lower ..." for a minus). A minus before a
plain icon: pay or lose it now. A dark frame acts on the adversary's pieces, projects or threats. A dashed outline is
another player. A thin solid outline is your cards: a token in it goes to, or comes from, one of your cards (nothing
counts tokens across cards). No outline is you, or this card.

**Connectors.** `→` is an Action: once a round, pay the left to get the right; a bare `→` is free ("Action: get
..."), and needs no "once a round": every Action is. `:` is an Effect: when the left happens, get the right. `/` is
per. OR is written out; a line opening with OR is the other half of an Action, one use a round between them. No card
has an exception: every symbol means what it always means ([card-guidelines.md](../card-guidelines.md)).

**Shapes.** Square tiles are resources, discs are tokens on cards (alone, on this card), bare silhouettes are pieces
on the board and moving them, ringed circles are tags. A number goes before its icon, no number means 1, and a
negative always has its minus. In the dark frame a clock is a delay (threats wait a round), a minus on the jam mark a
jam (off one threat's counter). `≤`N is up to N, your choice: a give. The shield mark stops one destruction (alone, a
hardening token on a facility; with a node, a shield; with a country, Aegis Dome Array's); the shelter mark is a whole
country for the round. A node with a struck-through coin is placed free. The rules text says what the icons say, in
the same order, and opens with the box's word.

Card details beyond the card text:

The card set: 159 cards, all in the draft decks: 110 tier 1 and 49 tier 2; 42% programs, 41% buildings and 17% events
(the engines are on buildings); each tag primary on 24 to 30. Every card is listed with its cost, insight, tags, kind
and text in [cards.md](../cards.md) (generated) and priced in [card-pricing.md](../card-pricing.md). Card text says what a
card does; how each primitive resolves:

- **Reach:** a card's map target is in your reach (within 1 of your node) where its text says so; a removal in one
  country (troops or ambassadors by number) is not. A removal (`damage`) takes a hex in reach holding adversary
  pieces; a country removal any country holding them. A card names the pieces it removes: where it says "2 adversary
  ambassadors or 1 troop" you choose one kind.
- **Hardening:** a different facility in reach for each token, each one a token can protect ([ai-boards.md](ai-boards.md)).
- **Factories:** "build a factory" is always on a free industrial zone in reach; the factory is yours (your cube). A
  building that builds one can be played with no such zone and then builds nothing.
- **Moving ambassadors** (`move_ambassadors`, `move_one`, `consolidate`, `spread_ambassadors`): onto pieces (city,
  base, facility or node), each arriving as in [ambassadors.md](ambassadors.md); stop at any time. `move_one`'s `range` 2 on Embassy Compound and
  Peacekeeper Rotation is an exception to reach 1 kept on purpose: moving is those cards' point.
- **Peek:** the top cards of the adversary deck, behind the face-up card, are shown; one fewer after each reveal.
- **Jam:** the chosen threat needs less of each resource (not below 0) while in play; if that leaves it fully paid, it
  is countered at once. **Counter a threat:** partial payments on it are refunded.
- **Cleanse:** take the adversary cube off a corrupted research station or factory in reach, one hex at a time (some
  cards: all in the first hex's country). A station becomes neutral. `only` limits it to stations (Reclamation Crew,
  priced as a station flip) or to factories (Back-Channel Talks, cheap). With `cities` (Great Liberation) a corrupted
  city counts too, and it is liberated ([ambassadors.md](ambassadors.md)).
- **Liberate:** liberate one corrupted city in your reach ([ambassadors.md](ambassadors.md)): it is clean; with `align` your cube goes on it. Each
  engine's twist: `ambassadors` (gain that many), `troops` (up to that many adversary troops in the city's country come
  off), `botnet` (a botnet on or next to the city comes off). Only the liberate capstones print it ([player-cards.md](player-cards.md)).
- **Clear a city (`clear_city`):** remove up to N adversary ambassadors (Liberation Accords: 6) around one corrupted
  city in reach, on it or next to it, your choice which (with no more there than the card takes, all go). Those next to the city don't raise the fallback, so clearing them only
  thins the city's build before it spreads; each on the city costs the fallback one more ambassador ([ambassadors.md](ambassadors.md)).
- **Card text:** generated from the card's ops (*The card grammar*, above), extra costs and lowered income included; a
  long card sets smaller. **Harden N facilities** is N different ones (Hardening, above); **shelter one country**:
  nothing in it, facility or node, can be destroyed this round. The house words are in [README.md](README.md#terms)'s *Terms*.
- **Projects:** a drain removes compute from one visible adversary project; **drain all** from every visible one; a
  discard takes one with its compute (if the row is then empty, the next row is dealt).
- **Botnets:** removing a botnet (tier 2, in reach) is, with Decommission (a Unity card, [alignment.md](alignment.md)), the only way to remove
  one ([adversary.md](adversary.md)); cutting one off (a hack) makes it produce no compute in the next build.
- **Draw:** every draw takes the top cards of the tier-1 deck into your hand, no keep paid (the deck reshuffles its
  discards when empty). Science's card draw.
- **Count as power (`gain_per_tokens`; here only):** gain 1 of a resource per N tokens here, on the Action's own
  card; the tokens stay.
- **Cash out (`cash_out`; one card):** remove every token of a type from one card (an Action's own, "here"; a
  program's, one of your cards you pick); gain the card's rate of a resource for each.
- **Per tag (`gain_per_tag`):** once, 1 of a resource per N tags of a kind on your cards in play; a program counts
  itself (it enters play before it resolves), an event doesn't.
- **Troops in one country (`remove_troops`):** remove up to N troops in one country, any country, your pick of hexes
  when there are more than N. **Ambassadors in one country** (`remove_ambassadors_country` with a number): up to N
  there, the same way. Neither reads "per hex": Matt's ruling 6.
- **Help cards** (keyword `help`, ~10%): the player chooses another player; both gain, the helper paying. Without
  another player, a card that gives from your own stock (`transfer`; no card prints it) is not offered, nor is one
  whose only effect is another player's gain (r39: the energy gift Actions, Substation Hub and Treaty Grid Exchange,
  took their energy at one country and gave nothing); other help cards are (their own gain still applies). Likewise a
  draw with both tier-1 piles empty (Fieldwork) is not offered on its own.
- **Moves of "up to 2 hexes"** count hex distance (`Map::distance`, over land and sea); the destination must be a
  piece.
- **Cordon (`cordon`):** in one country, no arrivals land in the next round's build (an out-of-turn build before it
  brings none either); the rest of a build goes on. Hardpoint Lockdown prints it: a Security card that does an
  ambassador's job.
- **Play a building, paying no credits (`free_building`):** the Builder's Rush Build (1 Industry and 1 energy). Its tags are sacrificed first,
  so a building that needed those tags or the sacrificed stack can't then be played; its other costs and requirements
  stand, paid from what is left after the power's energy. The power isn't offered when no building in hand would still
  be playable after some sacrifice and that energy (r39: it took both and played nothing).
- **Aegis Dome Array's shelter:** automatic, the first destruction of a facility or node each round in a country where
  its owner has a node.
