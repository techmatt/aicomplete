# Threats (module `threats`; [`content/threats.json`](../../content/threats.json))

*The threat decks, dealing, location tokens, who is near, countering and what threats do. Terms and conventions are in
[README.md](README.md). Every threat, with its text and counter menu, is in [adversary-cards.md](../adversary-cards.md).*

- Variant **`countries`** (default): the model below. Variant **`shared_row`** plays the same (135: the row went with
  the old sources; it stays so its config loads). Variant **`off`**: no threats ([README.md](README.md#off-variants)).

**The decks (candidate; design doc, Matt).** One threat deck per tier, each card printed once and shuffled at setup:
tier 1 (this round's losses: credits, energy, insight, cards, a blackout, adversary ambassadors), tier 2 (lasting
ones: nodes, facilities, corrupted sites, botnets, corp troops) and tier 3 (the last rounds' squeeze). Each card carries
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
(`threats.pull_forced`: a content bug, never seen in `drafts/adversary/check.py`'s 165,600 simulated pulls). Difficulty
1 adds `extra_threats` more pulls of the card's first pull, countries in turn. A threat that deals a threat (Provocation,
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
player is near, whenever it was dealt; one it pays in full is countered at once (118). Difficulty 1 adds
`counter_bonus` to the first option's credits and `counter_bonus_all` to every resource item.

**Calibration (candidate, 134).** A counter is about a third to half a round's income (tier 1: 6-8 credits, 1-2
energy, 4-5 insight, 1-2 ambassadors, a research cube, 2 tags; tier 2: 9-12 credits, 2-3 energy, 3 tags; tier 3:
13-18 credits, or two resources); failing costs about 1.5-2x the counter per player near it. **No counter prints 1
credit** (Matt, r48): a credits option is 5 or more.

**Executing.** Threats execute in `threats.execute`, after production and war, at the round's end: anything still on
an uncountered threat is refunded, then each threat (country 0, 1, ...) does what it says near its tokens, sways
`threat_discord` toward Discord (unprinted: the floor that keeps a threat nobody is near from being free to ignore),
and is discarded with its tokens. Only threats on the board during this round's counter phase execute: one dealt later
waits and executes next round, after a counter phase in which it can be paid (Matt). "Pay or discard" is the player's
choice; with neither possible, the player pays what they have and discards their hand. Theft hits the player near it
with the most credits (ties: the lowest seat). No threat takes a player's last node.

**For new players (the adversary model).** Each round the adversary's event card comes first. Its top line deals
threats: in each country, turn up cards from the named tier's threat deck until one has a listed tag (a Universal card
has every tag), discard the misses, and put its location tokens on the board. If you have a node or an ambassador on
or next to a token, you are *near* that threat: you may help pay it off, and its harm hits you. Each threat lists a
few ways to pay, and the table pays one in full: credits and other resources can be pooled between players; giving up
cards for their tags, or removing your nodes next to it, is done by one player alone. Next the card's standing lines
land compute, ambassadors and troops, then its twists, arrivals and build, and the spread, corruption and march. You
draft and act; production then runs from one snapshot of the board, and war comes after it. At the end of the round
each threat nobody paid does what it says near its tokens and also sways 1 toward Discord. Tier-1 threats cost you
this round; tier 2 can take nodes, stations and factories for good, or make the corps' troops pile up; tier 3 arrives
in the last rounds beside a second, lighter threat. Compute fills the cheapest project on the adversary's board; a
full project fires, and when every slot has fired they refill. **What resisted a summary:** where a token goes (the
most-threatened uncorrupted site of its type, by count of adversary pieces), that a threat skips a country lacking
its location, and that only resource options pool.

**Compute.** Most threats also give the adversary compute, onto its projects ([adversary.md](adversary.md)), when they
execute uncountered: printed on the threat.
