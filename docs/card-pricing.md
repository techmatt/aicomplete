# Card pricing (r34)

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
- **The cost curve:** tier 1 median 6 (most 4-9, a few strong cards at 10-12), tier 2 median 14 (ultimates 16-21 and
  1 antimatter). Six credits is about a quarter of a round's printed credits on r33's boards (25 / 21 / 18), tier 2 about
  half.
- **One shape for one job across the tags:** income per tag 1 a tag of its own, needs 3, 9 credits; a standing counter
  (`auto_counter`) 8 credits, needs 2; a typed jam 5 and a rider of its tag, a free typed-jam Action 3 a round.
- **Repeatable and progressive cards** are priced on their uses: an Action (value less its cost) × uses × 0.7 (tier 1
  about 5 uses, tier 2 about 2-3); a progressive token card (uses × payoff) × 0.7, counting adds as uses (ruling 12).
- **A gift to another seat** is 0.75 × face; **a tag left in play** 1, then 0.5 each (events leave none).
- **The liberate capstones**: about 30-40 credits' worth, at least half in their engine's resource, out of reach
  before mid-game; a city liberated is 25 (5 Sway and a city), against the fallback's 5 ambassadors at about 100.

## Rates known to be off at r34

Measured or found against play; each is still the table's number until a pass moves it.

- **Antimatter 9** (a round of antimatter income costs 8-11 credits): the heuristic holds antimatter at 2 points, about 3
  credits; at the table's 9 the strategist kept its antimatter and lost 59 points (seeds 31000). What antimatter buys
  is the ultimates' 1-a-card gate, not 9 credits of anything.
- **Troop removal 5** (3.6 each in one country): the heuristic counts a troop at 12 points, about 20 credits. Interceptor
  Battery's Action (2 energy for a troop anywhere) nets −0.4 at the table, and agents keep it and never play it.
- **City liberated 25:** the heuristic's corrupted city is 30 points, about 50 credits.
- **The recipe nets every card about its keep + 1** over its costs at the table, so the table alone can't rank cards for
  a draft: tier 2 reads as worth no more than its keep unless the draft leaves the play cost out, as the agents' does.
- **Energy 2:** measured under 2 at r31; at game end seats hold 6-11 energy unspent at r34, not read per round.
- **Insight 2.5:** a fit (it centers tier 1 on its printed prices), not a measure; `economy.insight_unspent` would settle it.
- **A player cube at completion, about 9:** measured cheaper in play, as energy and chips buy less than their rates.
