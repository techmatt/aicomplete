# Threats (module `threats`; [`content/threats.json`](../../content/threats.json))

*The threat decks, dealing, location tokens, who is near, countering and what threats do. Terms and conventions are in
[README.md](README.md). Every threat, with its text and counter menu, is in [adversary-cards.md](../adversary-cards.md).*

- Variant **`countries`** (default): the model below. Variant **`shared_row`** plays the same (135: the row went with
  the old sources; it stays so its config loads). Variant **`off`**: no threats ([README.md](README.md#off-variants)).

**The decks (candidate; design doc, Matt).** One threat deck per tier, each card printed once and shuffled at setup:
tier 1 (adversary pieces at its location and compute, plus its tag's harm: energy, insight, chips, cards, a blackout,
Unity, sway), tier 2 (lasting ones: nodes, facilities, corrupted sites, botnets, troops, ambassadors lost, Unity
pulled back) and tier 3 (the last rounds' squeeze). Each card carries
1 or 2 **tags** (Military, Influence, Cyber, Economic, Grid; **Universal** counts as every tag) and 1 or 2 **location
types** (city, coastal city, research station, factory, military base).

<!-- generated: threats -->
| Tier | Cards | Military | Influence | Cyber | Economic | Grid | Universal |
|---|---|---|---|---|---|---|---|
| 1 | 25 | 6 | 7 | 6 | 6 | 5 | 0 |
| 2 | 28 | 6 | 7 | 6 | 7 | 5 | 3 |
| 3 | 11 | 4 | 1 | 2 | 2 | 1 | 3 |
<!-- /generated -->

**Dealing: the event card's pulls.** The adversary's event card ([adversary.md](adversary.md)) prints its pulls on
its top line ("each country: a Military 2 threat", "Military 1 or Cyber 1", "any tier-2 threat"); a stage-III card
pulls twice. In `threats.deal`, for each pull and each country in order, **the draw rule**: turn up the top card of
that tier's deck; it stays if it carries a listed tag (Universal fits any; "any" takes every card) and the country has
a site of one of its location types; otherwise it goes to that tier's discard and the next is turned up. An empty deck
reshuffles its discard. A pull still unfilled after one reshuffle takes the next card anyway and is counted
(`threats.pull_forced`: a content bug, never seen in `drafts/adversary/check.py`'s 165,600 simulated pulls). From
difficulty `extra_threats_at` (4) the card adds `extra_threats` more pulls of its first pull, countries in turn: below
d4 the ladder raises intensity, never the count (142, Matt). A threat that deals a threat (Provocation,
Last Ultimatum) deals it now, into its own country, at its printed tier, any tag; it executes next round.

**Location tokens.** A dealt threat puts a token on a site of each of its location types in its country (two tokens
never share a hex): a city or coastal-city token on an uncorrupted city (coastal for coastal), a station token on an
uncorrupted research station, a factory token on a factory that isn't the adversary's, else a free industrial zone, a
base token on a military base. Among those sites, the one with the most adversary pieces on or next to it (adversary
ambassadors, troops, botnets); ties, the lower hex number. A printed game does this by eye. A location type the country
lacks gets no token; the pull never deals a threat with none (so a base-only threat never lands at 1 country).

**Near.** A player is **near** a threat with one of their nodes or ambassadors on or next to any of its tokens. Only a
player near it may pay toward it; "each player near it" is who its harm hits; a piece it places lands on its token
unless the text says otherwise, and "near it" in its text means on or next to a token. A threat nobody is near still
executes.

**Countering: a menu.** A threat prints a few ways to pay; the table pays **one option in full**:

- **Resources** (credits, energy, insight, research cubes, chips, ambassadors from your supply) are pooled across the
  players near it (`Pay` 1, or as much as is still useful; `Withdraw` takes back everything you put on it in one
  resource, even after you stop being near). The moment some option's resources are all covered it is countered and
  discarded; the rest goes back, later seats first.
- **Tags**: one player sacrifices cards in play carrying that many tags of the kind (as an AI-board power's cost,
  [ai-powers.md](ai-powers.md)): Security for Military, Diplomacy for Influence, Cyber, Industry for Economic, Power
  for Grid. Tier 1 asks 2 tags, tier 2 asks 3, tier 3 3 or more (Matt, 2026-11-06).
- **Nodes**: one player removes that many of their nodes on or next to a token; they leave the game as destroyed
  ones do, and never a player's last node.
- A tag or node option, or an option mixing them with resources, is paid whole by one player (`CounterWith`).

A player's card that jams or counters a threat reaches only threats they are near (a typed one, threats carrying its
tag or Universal); a jam takes its amount off every resource item of every option. A Unity reward reaches any threat.
**Standing counters:** a tier-2 capstone with an automatic counter takes its amount off each threat of its type the
player is near, whenever it was dealt; one it pays in full is countered at once (118). From difficulty
`counter_bonus_at` (2) every resource item costs `counter_bonus_all` (2) more, a credits item `counter_bonus_credits` (6) more
instead, and the first option's items `counter_bonus` (0) more still; from `counter_more_at` (3) every item
`counter_more_all` (1) more again, credits `counter_more_credits` (3).

**Counters (candidate, 142; design doc *Difficulty and threat intensity*, Matt).** A menu leads with a scarce
currency fitting the card's tag (Military: energy or ambassadors; Influence: ambassadors or insight; Cyber: research
cubes or insight; Economic: chips or energy; Grid: energy or research cubes), then a second one or a tag sacrifice,
and lists **credits last**, on every menu. Printed amounts are paid (`counter_pct` 100 by tier and
`counter_credit_pct` 100 are the screens' dials). Tier 1: 3 energy, 3 ambassadors, 8 insight, 2 cubes, 4 chips, 2
tags or 15 credits; tier 2: 4, 4, 10, 3, 6, 3 tags or 23 credits; tier 3: 5, 5, 12, 4, 8, 4 tags, two-resource
options, or 33 credits. Nothing is priced as a share of income. **A counter never shares a currency with its harm**
("pay 7 credits or lose 8" is a design error): no threat's harm takes credits, a harm taking energy or insight drops
that counter, and since every uncountered threat destroys nodes (`threat_nodes`) **no menu offers nodes**. The load
check and `drafts/adversary/check.py` refuse a shared currency. No counter prints 1 credit (Matt, r48).

**Executing.** Threats execute in `threats.execute`, after production and war, at the round's end: anything still on
an uncountered threat is refunded, then each threat (country 0, 1, ...) does what it says near its tokens, sways
`threat_discord` toward Discord (unprinted: the floor that keeps a threat nobody is near from being free to ignore),
has each player near it destroy `threat_nodes` of their nodes near it (138, candidate: 1 / 2 / 3 by tier, `harm_bonus`
2 more from difficulty `harm_bonus_at` 3; never a last node in stage I), and is discarded with its tokens. Only threats on the board during this round's counter phase execute: one dealt later
waits and executes next round, after a counter phase in which it can be paid (Matt). "Pay or discard" is the player's
choice; with neither possible, the player pays what they have and discards their hand. Theft hits the player near it
with the most credits (ties: the lowest seat), whatever it takes. No threat takes a player's last node.

**For new players (the adversary model).** Each round the adversary's event card comes first. Its top line deals
threats: in each country, turn up cards from the named tier's threat deck until one has a listed tag (a Universal card
has every tag), discard the misses, and put its location tokens on the board. If you have a node or an ambassador on
or next to a token, you are *near* that threat: you may help pay it off, and its harm hits you. Each threat lists a
few ways to pay, scarce things first (energy, ambassadors, insight, research cubes, chips, or cards given up for their
tags) and credits last; the table pays one in full: resources can be pooled between players, giving up cards is done
by one player alone. What a threat takes if unpaid is never what it asks for. Next the card's standing lines land
compute, troops and its new ambassadors (always on the lowest-numbered sites of a kind), then its twists, its Corrupt
lines (each site that can gather enough adversary ambassadors from itself and next door corrupts) and the march. You
draft and act; production then runs from one snapshot of the board. At the end of the round each threat nobody paid
does what it says near its tokens, destroys 1, 2 or 3 nodes of each player near it (by tier) and sways 1 toward
Discord. A tier-1 threat leaves an adversary ambassador or a troop on its spot and feeds the adversary's projects; tier
2 corrupts and seizes sites, takes ambassadors and pulls Unity back; tier 3 arrives in the last rounds. Compute fills
the cheapest project on the adversary's board; a full project fires, and when every slot has fired they refill.
**Harder levels** make threats harsher, not more: difficulty 1 lets sites corrupt with one fewer ambassador, 2 makes
every counter cost 2 more of each thing (6 more credits), 3 adds 1 more again (3 credits) and makes each unpaid threat take 2 more nodes, and only 4 deals an extra
threat a round. **What resisted a summary:** where a token goes (the most-threatened uncorrupted site of its type, by
count of adversary pieces), that a threat skips a country lacking its location, and that only resource options pool.

**Compute.** Most threats also give the adversary compute, onto its projects ([adversary.md](adversary.md)), when they
execute uncountered: printed on the threat.
