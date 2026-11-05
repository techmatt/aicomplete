# Threats (module `threats`; [`content/threats.json`](../../content/threats.json))

*Threat sources, dealing, countering and what threats do. Terms and conventions are in [README.md](README.md).*

- Variant **`countries`** (default): each threat sits on one country and acts there; who may pay toward it is set by
  its **sources** (below), and "each player" in its text means each player with a node there.
- Variant **`shared_row`**: threats sit in one row, anyone may pay, and each acts on the whole board: an effect per
  country (Blackout, Sabotage, Embargo, Overrun, Mass Production) hits every country, "each player" is every player, and a
  placement (Infiltration's, Agitators', Sleeper Cells' and Insurrection's ambassadors, Orbital Strike's, Siege's and
  Total War's troops and ambassadors, Station Seizure's station) is that many on the whole board, players choosing
  where. The row is still dealt `per_country` threats per country, so more countries deal more threats.
- Variant **`off`**: no threats ([README.md](README.md#off-variants)).

**Threat sources (candidate; design doc, Matt).** Threats come from the adversary's pieces on the map, and reaching
those pieces decides who counters them. Every threat prints a **source type**; its sources are all the adversary
pieces of that type in its country:

| Source type | Source pieces |
|---|---|
| influence | corrupted cities |
| cyber | corrupted research stations and botnets |
| economic | adversary factories |
| military | troops |

Military bases and adversary ambassadors are never sources.

- **Who may counter:** a player with any of the threat's sources in reach, or with their ambassador on one. Payments
  are pooled, and a player who lost their access can still withdraw. A player's card that jams or counters a threat
  ([card-grammar.md](card-grammar.md)) reaches only threats they may counter; a reward from the Unity deck reaches any threat.
- **Countering doesn't consume the ambassador.** It holds the source and keeps access to later threats there until an
  adversary ambassador cancels it. The rules that take ambassadors anyway still do: a facility's defense ([ambassadors.md](ambassadors.md)), and a
  fallback liberation ([ambassadors.md](ambassadors.md)).
- **No source when dealt: a target (Matt).** A threat with no source in its country names one: the lowest-numbered
  clean city, station or factory of its type, neutral before player-owned; for a factory or station with none to take,
  the lowest site one would be built on; for troops the country's lowest base a Wall leaves open. While the threat is
  face up the target is its source: anyone with it in reach, or with an ambassador on it, may counter.
  **Countered, nothing is created.** Uncountered, as part of its execution and before its own effect, the target is
  corrupted, built then corrupted, or the troop lands (a Wall on the base stops it). A player's station named as a
  cyber target flips only as an effect would ([board.md](board.md): `station_flip_at`); short of that one adversary
  ambassador lands on it, the station holds, and no source appears. At most one piece a country a round, and a
  station that held uses up that round's piece. Nothing is set aside for lack of a source. A country with nothing to name (no base at 1 country) leaves the
  threat with no source: nobody can pay it, and it makes nothing.
- **A source that leaves:** the rule reads the map now, so a threat whose sources are gone can't be paid
  until one is back, and it still executes. Marches come before the deal, so this takes a troop spent or a piece gone
  inside the round.
- **Shield:** while a threat is face up, the adversary's pieces of its source type in its country can't be destroyed
  or liberated until it is countered: removal and liberate cards skip them, and player ambassadors at a corrupted
  city's fallback wait, and liberate it the moment the threat is countered. An adversary ambassador is never shielded.
- **Standing counters:** while a player has a tier-2 capstone with an automatic counter, each threat of its type in
  their home country needs that much less, whenever it was dealt; one it pays in full is countered at once, as dealt
  or as the counter comes into play (118). Under `shared_row` no threat has a country, so it counters none (D54).

**For new players.** Every threat grows out of something on the map. Find what its badge names in its country: a
corrupted city (influence), a corrupted station or botnet (cyber), an adversary factory (economic), a troop
(military). You can pay only with one of those next to your nodes, or with your own ambassador on one. If the country
has none, the threat names a target on the map, and that counts as its source: counter it and nothing happens, leave
it and the adversary gets a new piece there (a player's station usually holds, taking an adversary ambassador). Until a threat is countered, you can't remove its sources, so counter
first and clear after.

**Dealing.** In `threats.deal` each country gets `per_country` threat (plus difficulty 1's extra, [adversary.md](adversary.md)), face up: tier 1
until this round's adversary card is stage II or later, then tier 2 (difficulty 4: a round sooner). Each tier's deck
holds one copy of each of its threats, shuffled at setup; its discards reshuffle when it runs out, for any deal. Each
deck holds enough threats that a deal doesn't run short at 4 countries and difficulty 4; a deal the deck and its
discards can't supply deals what it can and counts the rest as `threats.short`.

**Energy counters (candidate).** A counter is one cost, each resource it prints paid in full. Most print credits alone;
the grid's own harms, Blackout, Sabotage and Commandeer, take 1 credit and 1 energy, so energy has a use every round
a threat lands. Any resource a seat gets without a specific card may be a counter's; the full threat redesign waits
for the adversary pass. **Insight counters (r41):** Smear Campaign, Infiltration (tier 1), Deepfake Wave and Purge
Protocols (tier 2) take 5 insight and nothing else, about one round's printed insight: a sink for the insight a seat
didn't spend on cards. Insight is pooled like any payment; the players bring it through their converters (the cube
converter, Cintamani's, Focus, Vigil, Synthesis), and no rule lets it stand for another resource.

**Countering.** A threat is countered only when fully paid. Payments are pooled across players (`Pay` 1, or
as much as is still useful, at a time; `Withdraw` takes back everything you put on it in one resource, even after you
lose your nodes there). The moment it is fully paid it is discarded and the payments are spent (an overpayment,
possible after a jam, goes back, later seats first). In `threats.execute`, anything still on an uncountered threat is
refunded, and the threats execute in order (the row, then country 0, 1, ...) and are discarded. Only threats that were
on the board during this round's counter phase execute: one dealt later in the round (a Discord tip in production or
upkeep, False Flag) waits beside its country (or the row) and executes next round, after a counter phase in which it
can be paid (Matt).

Choice threats are preferred; a destroyed node leaves the game, so a tier-1 threat's loss can be permanent too.
**Counters cost credits (candidate),** 4-8 by tier and harm, or 5 insight on four of them (the table prints them):

<!-- generated: threats -->
| Threat | Tier | Source | If not countered | Counter |
|---|---|---|---|---|
| Requisition | 1 | economic | Each player here discards 2 cards or pays 6 credits. | 4 credits |
| Commandeer | 1 | economic | Each player here discards 2 cards or pays 4 energy. | 1 credit + 1 energy |
| Supply Raid | 1 | economic | Each player here loses 6 credits. | 5 credits |
| Blackout | 1 | cyber | Cities here produce no energy next round; the adversary gains 3 compute. | 1 credit + 1 energy |
| Sabotage | 1 | cyber | Players here make no energy in upkeep next round and each loses 3 tokens from their cards; the adversary gains 3 compute. | 1 credit + 1 energy |
| Infiltration | 1 | cyber | 3 adversary ambassadors land on player-owned pieces here. The adversary gains 3 compute. | 5 insight |
| Agitators | 1 | influence | 3 adversary ambassadors land on clean cities here. Each player here destroys 1 of their nodes here. | 5 credits |
| Orbital Strike | 1 | military | 1 troop lands on a factory here, or a city if no factory. The adversary gains 3 compute. Each player here destroys 1 of their nodes here. | 6 credits |
| Defection | 1 | influence | Each player here loses an ambassador. Sway 2 toward Discord. Each player here destroys 1 of their nodes here. | 4 credits |
| Eviction | 1 | military | Each player here pays 6 credits or loses a node here (never their last). | 5 credits |
| Embargo | 1 | economic | No site here can be claimed next round; the adversary gains 3 compute. | 4 credits |
| Smear Campaign | 1 | influence | Each player here gets 2 less insight next round. Move 2 Sway from the Unity bucket to the Undecided. | 5 insight |
| Station Seizure | 2 | cyber | An uncorrupted research station here becomes corrupted (a player's holds unless 2 adversary ambassadors and no player ambassador stand on it: an adversary ambassador lands there instead). The adversary gains 4 compute. Each player here destroys 1 of their nodes here. | 7 credits |
| Overrun | 2 | influence | The uncorrupted city here with the most adversary ambassadors becomes corrupted. Each player here destroys 1 of their nodes here. | 8 credits |
| Mass Production | 2 | economic | The adversary builds now, here; the adversary gains 4 compute. | 7 credits |
| Purge Protocols | 2 | cyber | Each player here discards their most expensive card in hand and loses 3 tokens from their cards. | 5 insight |
| Total War | 2 | military | 2 troops land on factories here, or cities if no factories; each destroys all nodes on its hex. 2 adversary ambassadors land on military bases here. The adversary gains 4 compute. | 8 credits |
| Conscription | 2 | economic | Each player here discards 3 cards or pays 6 energy; the adversary gains 3 compute. | 7 credits |
| Sleeper Cells | 2 | cyber | 4 adversary ambassadors land on player-owned pieces here. The adversary gains 4 compute. Each player here destroys 1 of their nodes here. | 7 credits |
| Insurrection | 2 | influence | 4 adversary ambassadors land on clean cities here. Sway 2 toward Discord. Each player here destroys 1 of their nodes here. | 7 credits |
| Siege | 2 | military | 2 troops land on factories here, or cities if no factories. Each player here destroys 2 of their nodes here and loses an ambassador. The adversary gains 3 compute. | 8 credits |
| Manhunt | 2 | military | Each player here destroys 2 of their nodes here. | 8 credits |
| Asset Freeze | 2 | economic | Each player here loses 8 credits. The adversary gains 3 compute. | 7 credits |
| Deepfake Wave | 2 | influence | Sway 3 toward Discord. Each player here gets 2 less insight next round. | 5 insight |
<!-- /generated -->

**Compute.** About half the threats also give the adversary compute, onto its projects ([adversary.md](adversary.md)), when they execute
uncountered: candidate amounts, printed on the threat.

An uncountered threat also sways `threat_discord` Discord. "Pay or discard" is the player's choice; with neither
possible, the player pays what they have and discards their hand. Purge Protocols compares printed
credit costs; on a tie the player chooses. A lost ambassador comes from the player's supply, else from the board
(their pick of hex).
