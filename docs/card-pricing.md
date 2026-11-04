# Card pricing (r35)

How a card's price is set, as the set stands. Every price is a **candidate**. The cards themselves are
[cards.md](cards.md), generated from `content/cards.json`; the rules for writing one are
[card-guidelines.md](card-guidelines.md). What each pass changed and why is in git and in the pass's write-up under
`experiments/results/screen-NNN/` (076's card-by-card verdicts: `screen-076-cards/cards.md`). A card whose price needs
a word in the card viewer gets one current line in `docs/card-pricing-notes.json`, keyed by card id, written by the item
that sets the price.

## The rates

**The rates' one home is `configs/agents/values.json`**: a value in credits for each resource, tag, token and op,
each op's row saying what one unit is. The agents value cards by the same table (`docs/agents.md`, *The value table*), so
a rate moved there moves the agents' read of every card that prints it. Change a rate there, not here.

## The method (Matt's rulings)

- **Tier is power** (ruling 4): costs and effects go up a lot with tier, few cards cost 0, and tier is not a proxy for
  cost.
- **The recipe:** value the effect at the rates; then credits = value − the keep (tier 1 2.5, tier 2 6.5) − 2.5 × the
  insight to play − about 1, so the card beats the plain route it replaces (ruling 7).
- **A `needs` chain is the price** (ruling 10): needs 3-4 of a tag sits 2-4 under the recipe, needs 6 about half.
- **The cost curve:** tier 1 median 6 (most 4-9, a few strong cards at 10-12), tier 2 median 12 (ultimates 16-21 and
  1 antimatter; the liberate capstones 25-47). Six credits is about a quarter of a round's printed credits on r33's boards (25 / 21 / 18), tier 2 about
  half.
- **One shape for one job across the tags:** income per tag 1 a tag of its own, needs 3, 9 credits; a standing counter
  (`auto_counter`) 13 credits, needs 2; a typed jam 5 and a rider of its tag, a free typed-jam Action 3 a round.
- **Income has a horizon** (r35): an income, of credits, energy or anything, counts for the rounds left at a typical
  purchase less a tempo discount: 4 rounds on a tier-1 card, 2.5 on a tier-2 one (`rates.horizon_t1`, `horizon_t2`).
- **Repeatable and progressive cards** are priced on their uses: an Action (value less its cost) × the horizon × 0.7;
  a progressive token card (uses × payoff) × 0.7, counting adds as uses (ruling 12).
- **Energy is rare** (r35): about 4.5 credits an energy; +1 energy of income about 8-10 credits on a tier-1 card, +2 a
  tier-2 one, and no card above +2 (a big income fits the target at no price). Two strong cards and three threats'
  counters take energy, and two buildings gift it on (1 energy: another player's 2 chips or 1 insight).
- **A station is a flip** (r35): freeing one is worth a cube a round both ways, about 15 (`cleanse_station`); freeing a
  factory stays about 6.
- **A gift to another seat** is 0.75 × face; **a tag left in play** 1, then 0.5 each (events leave none).
- **The liberate capstones**: priced at their own value, at least half in their engine's resource, out of reach
  before mid-game: a city liberated is 45 (5 Sway, the city and the ambassadors and Discord it stops; play reads about
  50), against the fallback's 5 ambassadors at about 100; each engine's twist at its own row (an ambassador; up to 3
  troops in the city's country; a botnet next to it).

## Rates known to be off at r35

Measured or found against play; each is still the table's number until a pass moves it.

- **Antimatter 7** (lowered from 9 at r35): the heuristic holds antimatter at 2 points, about 3 credits; what antimatter
  buys is the ultimates' 1-a-card gate, not 7 credits of anything.
- **Troop removal 15** (10 each in one country, from 5 and 3.6 at r35): the heuristic counts a troop at 12 points, about
  20 credits. A count prints "up to": Kinetic Interdiction's 6 in one country reads 60, though a country rarely holds 6.
- **Energy income against its anchor:** at 4.5 an energy over a 4-round horizon, +1 energy of income reads 18 on a tier-1
  card, against the 8-10 Matt's anchor sets (the anchor priced Baseload Contract and the Power-grid incomes); one of
  the two moves when play says which.
- **The recipe nets every card about its keep + 1** over its costs at the table, so the table alone can't rank cards for
  a draft: tier 2 reads as worth no more than its keep unless the draft leaves the play cost out, as the agents' does.
- **Energy 4.5** (from 2 at r35): seats now hold about 1-2 at rounds 3 and 6 and 1.8-2.6 at the end (090,
  `energy_read.py`), against 6-12 at the end at r34.
- **Insight 2.5:** a fit (it centers tier 1 on its printed prices), not a measure; `economy.insight_unspent` would settle it.
- **A player cube at completion, about 9:** measured cheaper in play, as energy and chips buy less than their rates.
