# Card pricing (r42)

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

Two other places hold rates, and neither is a price:

- **The search's weights** (`Weights` in `crates/aic-sim/src/agents/eval.rs`) are points tuned against win rate, not
  credits. Where one shadows a table row (held energy `energy` 0.8, `antimatter` 2, `liberated` and
  `liberate_progress`, `station_flip`), the gap is deliberate until a screen moves it: held energy below the table's
  price is on purpose (100), and `w.table_energy` and `w.table_antimatter` stay off. A table edit does not
  move them; say in the write-up when a price and its weight part further.
- **The effect ledger** (`scripts/cards/effect_ledger.py`) mirrors `agents/value.rs` by hand. The fast-lane test
  `effect_ledger_mirrors_the_table` checks its committed `cards.csv` against value.rs card by card (bar troop removal, which the ledger counts at what one
  removes in play): after a change to
  value.rs, `values.json` or a card, rerun the ledger, and fix its `op_worth` if the test still fails.

## A fair price

`python scripts/cards/effect_ledger.py --stdout [--cards PROPOSED.json]` prints each card's `fit` (the printed value the
joint fit gives it: Σ multiplier × reasoned value per fitted effect, + an effect on one or two cards and the tags at
their reasoned value, + its tier's constant, + the rate per needed tag × its
`needs`) and `fair_credits`: the fit less what the card costs in other resources (insight, energy, chips, antimatter,
cubes, `lowers`) at the table's rates. That is the credit price that would put the card on the fitted curve. It writes
nothing, so a staging session can price proposed cards: copy `content/cards.json` to `out/`, edit, pass it with
`--cards`. The fit's constants move with the set; read them from the printout, never from an old write-up.

## The method (Matt's rulings)

- **Tier is power** (ruling 4): costs and effects go up a lot with tier, few cards cost 0, and tier is not a proxy for
  cost.
- **The recipe:** value the effect at the rates; then credits = value − the keep (tier 1 2.5, tier 2 6.5) − 2.5 × the
  insight to play − about 1, so the card beats the plain route it replaces (ruling 7).
- **A `needs` chain is the price** (ruling 10): needs 3-4 of a tag sits 2-4 under the recipe, needs 6 about half.
- **The cost curve** (r42, 106's overhaul, Terraforming Mars scale): tier 1 median 9 (most 6-10; the big income
  programs 18-40, Gigafactory Charter's "40 credits, +12 credit income" the top), tier 2 median 14 (most 10-19; the
  liberate capstones 18-42). The deck's median is 10 (r41: 7). Nine credits is about a third of a round's printed
  credits on the boards (25 / 21 / 18).
- **Engines are priced at their seat's fire rate** (106, Matt: err toward strong engines that look broken when played
  well): a trigger at the rate a seat built around its tag sees (about 1 card of the tag a round, 3 tags of any kind),
  not the table's generic `fires_play` × `tag_share`, so the effect ledger reads most engine cards under their
  printed price. That gap is the point; a card that nets below 0 for its own seat is still a bug. In a game the
  agents read a one-tag trigger at the seat's own share of the tag's cards (107, `rates.own_mix`), and a threshold
  at its own trigger's fill (`own_fill`).
- **Insight off every card** counts `rates.insight_discount_share` (0.25) of the plays (107): the floor below.
- **Insight discounts mostly sit at the floor** (106): `discount_floor` 1 holds insight too, and `science_every` already
  takes a seat with 2 Science tags to it, so a 1-insight discount bites only on 2-insight cards (Hyperscale Botnet).
  Science's discount card prints credits instead (Grad Student Pool).
- **One shape for one job across the tags:** a per-tag income program ("raise your credit income by 1 per X tag"),
  needs 3 of X, 10 credits, one per tag (r42); a building's per-tag income pays 2 a tag (Foundation Model Campus); a standing counter
  (`auto_counter`) 13 credits, needs 2; a typed jam 5 and a rider of its tag, a free typed-jam Action 3 a round.
- **Income has a horizon** (r35): an income, of credits, energy or anything, counts for the rounds left at a typical
  purchase less a tempo discount: 4 rounds on a tier-1 card, 2.5 on a tier-2 one (`rates.horizon_t1`, `horizon_t2`).
- **Repeatable and progressive cards** are priced on their uses: an Action (value less its cost) × the horizon × 0.7;
  a progressive token card (uses × payoff) × 0.7, counting adds as uses (ruling 12).
- **Energy is rare** (r35): about 4.5 credits an energy, 5 to buy from a neutral city (r36). +1 energy of income or
  production is the horizon × 4.5 × 0.8 (`rates.energy_income_share`: it arrives a drop at a time, not when needed):
  about 14 on a tier-1 card, 9 on a tier-2 one (r36, Matt's ruling; it replaced r35's 8-10 anchor), and no card above
  +2. Two strong cards and three threats' counters take energy, and two buildings turn it into a gift with a boost
  (1 energy: another player's 5 chips or 8 credits, about 1.7× an energy).
- **A station is a flip** (r35): freeing one is worth a cube a round both ways, about 15 (`cleanse_station`); freeing a
  factory stays about 6.
- **A gift to another seat** is 0.75 × face; **a tag left in play** 1, then 0.5 each (events leave none).
- **The liberate capstones**: priced at their own value, at least half in their engine's resource, out of reach
  before mid-game: a city liberated is 45 (5 Sway, the city and the ambassadors and Discord it stops; play reads about
  50), against the fallback's 5 ambassadors at about 100; each engine's twist at its own row (an ambassador; up to 3
  troops in the city's country; a botnet next to it).

## Rates known to be off at r36

Measured or found against play; each is still the table's number until a pass moves it.

- **Antimatter 7** (lowered from 9 at r35): the heuristic holds antimatter at 2 points, about 3 credits; what antimatter
  buys is the ultimates' 1-a-card gate, not 7 credits of anything.
- **Troop removal 11** (7 each in one country; 15 and 10 at r35-r37, 4 less at r38 when `alignment.troop` went to 0):
  the heuristic counts a troop at 12 points, about 20 credits. A count prints "up to": the ledger and the agents count what one removes in play (`values.json` `troops_removed`, 102; Kinetic Interdiction's 6 about 3.6).
- **The recipe nets every card about its keep + 1** over its costs at the table, so the table alone can't rank cards for
  a draft: tier 2 reads as worth no more than its keep unless the draft leaves the play cost out, as the agents' does.
- **Energy 4.5** (from 2 at r35): seats now hold about 1-2 at rounds 3 and 6 and 1.8-2.6 at the end (090,
  `energy_read.py`), against 6-12 at the end at r34.
- **Insight 2.5:** a fit (it centers tier 1 on its printed prices), not a measure; `economy.insight_unspent` would settle it.
- **A player cube at completion, about 9:** measured cheaper in play, as energy and chips buy less than their rates.
