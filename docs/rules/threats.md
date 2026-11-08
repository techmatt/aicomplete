# Threats (module `threats`; [`content/threats.json`](../../content/threats.json))

*The threat decks, dealing, location tokens, who is near, countering and what threats do. Terms and conventions are in
[README.md](README.md). Every threat, with its text and counter menu, is in [adversary-cards.md](../adversary-cards.md).*

- Variant **`countries`** (default): the model below. Variant **`shared_row`** plays the same (135: the row went with
  the old sources; it stays so its config loads). Variant **`off`**: no threats ([README.md](README.md#off-variants)).

**The decks (candidate; Matt's threat card standard, 2026-11-07, locked).** One threat deck per tier, each card
printed once and shuffled at setup: tier 1 and tier 2; **there is no tier 3**. A tier-2 card is about two tier-1s
(bigger consequences: a node, a site corrupted or seized, a botnet, troops, Unity pulled back), so no pull asks for
two tier-1s. Each card carries 1 or 2 **tags** (Military, Influence, Cyber, Economic, Grid; **Universal** counts as every tag) and 1 or 2 **location
types** (city, coastal city, research station, factory, military base).

<!-- generated: threats -->
| Tier | Cards | Military | Influence | Cyber | Economic | Grid | Universal |
|---|---|---|---|---|---|---|---|
| 1 | 25 | 6 | 7 | 6 | 6 | 5 | 0 |
| 2 | 32 | 7 | 7 | 7 | 8 | 6 | 5 |
| 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
<!-- /generated -->

**Dealing: the event card's pulls.** The adversary's event card ([adversary.md](adversary.md)) prints its pulls on
its top line ("each country: a Military 2 threat", "Military 1 or Cyber 1", "any tier-2 threat"). **The ladder:**
stage I pulls one tier-1 threat a country, stage II one tier-2 (or a tier 1 and a tier 2: Supply Lines), stage III
two tier-2s (`check.py` refuses another shape). In `threats.deal`, for each pull and each country in order, **the draw rule**: turn up the top card of
that tier's deck; it stays if it carries a listed tag (Universal fits any; "any" takes every card) and the country has
a site of one of its location types; otherwise it goes to that tier's discard and the next is turned up. An empty deck
reshuffles its discard. A pull still unfilled after one reshuffle takes the next card anyway and is counted
(`threats.pull_forced`: a content bug; `drafts/adversary/check.py`'s simulated pulls never need one, but threats a
threat deals and the ones waiting a round are out of the deck there too). From
difficulty `extra_threats_at` (4) the card adds `extra_threats` more pulls of its first pull, countries in turn: below
d4 the ladder raises intensity, never the count (142, Matt). A threat that deals a threat (Provocation,
Last Ultimatum) deals it now, into its own country, at its printed tier, any tag; it executes next round.

**Location tokens.** A dealt threat puts a token on a site of each of its location types in its country (two tokens
never share a hex): a city or coastal-city token on an uncorrupted city (coastal for coastal), a station token on an
uncorrupted research station, a factory token on a factory that isn't the adversary's, else a free industrial zone, a
base token on a military base. Among those sites, the one with the most adversary pieces on or next to it (adversary
ambassadors, troops, botnets); ties, the lower hex number. Tokens go in printed order, but if that leaves one off the
board (a city token on the only clean coastal city), the type with fewer sites goes first. A printed game does this by eye. A location type the country
lacks gets no token; the pull never deals a threat with none (so a base-only threat never lands at 1 country).

**Near.** A player is **near** a threat with one of their nodes or ambassadors on or next to any of its tokens. Only a
player near it may pay toward it. A piece it places lands on its token unless the text says otherwise (with two tokens,
"here" is the token of its first printed location). A threat nobody is near still executes.

**The nearest player (146; Matt's standard).** An uncountered threat's harm hits **one player, the nearest**: the
one whose node is closest to one of its tokens (hex distance, wherever that node is, near or not); on a tie the tied
players pick one (the engine asks the lowest tied seat, a loss; agents pick the seat that loses least). A player with
no node on the board is never hit; with no node anywhere, only its board consequences land. So every uncountered
threat lands on exactly one player, a predictable amount. "Destroy a node" takes the hit player's node closest to
it, never their last.

**Countering: a menu.** A threat prints a few ways to pay; the table pays **one option in full**:

- **Resources** (credits, energy, insight, research cubes, chips, ambassadors from your supply) are pooled across the
  players near it (`Pay` 1, or as much as is still useful; `Withdraw` takes back everything you put on it in one
  resource, even after you stop being near). The moment some option's resources are all covered it is countered and
  discarded; the rest goes back, later seats first.
- **Tags**: one player sacrifices cards in play carrying that many tags of the kind (as an AI-board power's cost,
  [ai-powers.md](ai-powers.md)): Security for Military, Diplomacy for Influence, Cyber, Industry for Economic, Power
  for Grid. Tier 1 asks 2 tags, tier 2 asks 3 (Matt, 2026-11-06).
- A tag option is paid whole by one player (`CounterWith`). (The engine can also read a nodes item, a player's own
  nodes near a token; no menu prints one.)

A player's card that jams or counters a threat reaches only threats they are near (a typed one, threats carrying its
tag or Universal); a jam takes its amount off every resource item of every option. A Unity reward reaches any threat.
**Standing counters:** a tier-2 capstone with an automatic counter takes its amount off each threat of its type on
the table, wherever it is and whenever it was dealt (148: no longer only those its holder is near); one it pays in
full is countered at once (118). From difficulty
`counter_bonus_at` (1) every resource item costs `counter_bonus_all` (1) more, a credits item `counter_bonus_credits` (3) more
instead, and the first option's items `counter_bonus` (0) more still; from `counter_more_at` (2) every item
`counter_more_all` (1) more again, credits `counter_more_credits` (2).

**Counters (146; Matt's standard).** **One counter option, two at most**: a scarce currency or a tag sacrifice, or
credits ("pay 3 energy or 12 credits"). Printed amounts are paid (`counter_pct` 100 by tier and `counter_credit_pct`
100 are the screens' dials). Tier 1: 3 energy, 3 ambassadors, 6 insight, 3 research cubes, 6 chips, 2 tags or 12
credits; tier 2: 4, 4, 7, 5, 7, 3 tags or 17 credits (146: the screens' 85% folded in). Nothing is priced as a share of income. **A counter never shares
a currency with its harm** ("pay 7 credits or lose 8" is a design error): no harm takes credits, and a harm taking
energy, chips, cubes, insight, ambassadors or nodes drops that counter. The load check and `drafts/adversary/check.py`
refuse a threat with more than two counter options, more than two consequences, a shared currency, a harm on each
player (a flag on every hex near it, a line in each country, "each player destroys"), or a special piece (a troop
that stands or occupies, corp troops past their cap). No counter prints 1 credit (Matt, r48).

**Executing.** Threats execute in `threats.execute`, after production and upkeep, before war, at the round's end: anything still on
an uncountered threat is refunded, then each threat (country 0, 1, ...) does what it prints, its player harm on the
nearest player, and is discarded with its tokens. **No harm is unprinted** (146: the node loss for every player near
an uncountered threat and the floor sway toward Discord are gone). The adversary board may print compute for each
uncountered threat (the Militarist's difficulty line: 4, from `board_difficulty_at`; [adversary.md](adversary.md)).
Only threats on the board during this round's counter phase execute: one dealt later waits and executes next round,
after a counter phase in which it can be paid (Matt). **A resource harm reads "pays N or discards M cards"** (the
player's choice; with neither possible, the player pays what they have and discards their hand), so holding none
dodges nothing; theft is the same payment, and the adversary gains its compute either way. A corruption or seizure a
threat prints is corruption, with its cleanup (the node on the site or its armor, the ambassadors there), and takes
only the site it names. No threat takes a player's last node.

**For new players (the adversary model).** Each round the adversary's event card comes first. Its top line deals
threats: in each country, turn up cards from the named tier's threat deck until one has a listed tag (a Universal card
has every tag), discard the misses, and put its location tokens on the board. If you have a node or an ambassador on
or next to a token, you are *near* that threat and may help pay it off. Each threat prints one way to pay, or two: a
scarce thing (energy, ambassadors, insight, research cubes, chips, or cards given up for their tags) or credits;
resources can be pooled between players, giving up cards is done by one player alone. What a threat takes if unpaid is
never what it asks for. Next the card's standing lines land compute, troops and its new ambassadors (always on the
lowest-numbered sites of a kind), then its twists, its Corrupt lines (each site that can gather enough adversary
ambassadors from itself and next door corrupts) and the march. You draft and act; production then runs from one
snapshot of the board. At the end of the round each threat nobody paid does what it prints, and nothing else: one
consequence, sometimes two, from a short list (troops or adversary ambassadors on its spot, compute, Sway toward
Discord, Unity pulled back, a site corrupted or seized, a botnet, or the nearest player paying energy, chips, cubes or
ambassadors or discarding cards, losing insight, a building, or a node). A player harm always hits the one player whose
node is nearest the threat. Tier 1 is small; tier 2 is about two tier-1s. Compute fills the cheapest project on the
adversary's board; a full project fires, and when every slot has fired they refill. **Harder levels** make threats
harsher, not more: difficulty 1 lets sites corrupt with one fewer ambassador and makes every counter cost 1 more of
each thing (3 more credits), 2 adds 1 more again (3 credits), 3 gives the adversary 4 compute for each unpaid threat
(and a shorter, busier game), and only 4 deals an extra threat a round.
**What resisted a summary:** where a token goes (the most-threatened uncorrupted site of its type, by count of
adversary pieces), that a threat skips a country lacking its location, and that only resource options pool.

**Compute.** Many threats also give the adversary compute, onto its projects ([adversary.md](adversary.md)), when they
execute uncountered: printed on the threat. Not every threat gives it.
