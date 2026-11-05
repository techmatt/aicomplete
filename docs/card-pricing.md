# Card pricing (r44)

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

- **Matt's unit anchors set the printed price** (111, 2026-11-04; the table and the anchor check are in
  [card-guidelines.md](card-guidelines.md), rulings 19-25): a card's anchor value over what it costs (credits, 4 an
  insight of keep and play, other costs at the anchors) sits near 1.2-1.5 at tier 1 and 1.8-2.5 at tier 2, never more,
  and every removal number fits the board (3-6 ambassadors, 1-2 troops, 3-6 compute). Where the table disagrees, the
  anchor wins for the price and the table stays the agents': `values.json` has chips 1.5 (anchor 3), insight 2.5 (4),
  a drawn card 4 (5), an ambassador removed about 2.2 and a troop 11 (5 each), a project discard 8 (about 24 at 2 a
  compute). The recipe and fit below are the agents' view and the effect ledger's.
- **Tier is power** (ruling 4): costs and effects go up a lot with tier, few cards cost 0, and tier is not a proxy for
  cost.
- **The recipe:** value the effect at the rates; then credits = value − the keep (tier 1 2.5, tier 2 5 at r43's 2 insight) − 2.5 × the
  insight to play − about 1, so the card beats the plain route it replaces (ruling 7).
- **A `needs` chain is the price** (ruling 10): needs 2-3 of a tag on an engine (108), needs 3-4 sits 2-4 under the
  recipe, needs 5-6 about half and much bigger (Zero-Day Auction: 1 antimatter per Cyber tag).
- **The cost curve** (r44, Terraforming Mars scale): tier 1 median 9 (r43: 10; most 6-12; the big income and
  late cards 16-40, Gigafactory Charter's "40 credits, +12 credit income, draw 2" the top), tier 2 median 14 (r43: 16;
  most 9-20; the liberate capstones 14-36). 20 cards cost 5 or less (r43: 6): the anchors read most flat-income and
  niche cards under 1.1 at r43's prices. The deck's median is 10. Nine credits is about a third of a round's printed
  credits on the boards (25 / 21 / 18).
- **Engines are priced at their seat's fire rate** (106, Matt: err toward strong engines that look broken when played
  well): a trigger at the rate a seat built around its tag sees (about 1 card of the tag a round, 3 tags of any kind),
  not the table's generic `fires_play` × `tag_share`, so the effect ledger reads most engine cards under their
  printed price. That gap is the point; a card that nets below 0 for its own seat is still a bug. **Payback within 2
  rounds** (108): give (credits, insight to play and keep, other costs) less the when-built gain and the card's own
  first fire, over a round's yield at that rate, is 2 or less for every engine building
  (`experiments/results/108_engines_worth_playing/payback.py`). **Engines pay up front** (108): the when-built gain is
  about two fires, in cards (draw 3, 4 on Foundation Model Campus and Vertical Monopoly) where the fire pays credits, since a credit gain on a
  credit cost would be a circular payment (guideline 5). In a game the
  agents read a one-tag trigger at the seat's own share of the tag's cards (107, `rates.own_mix`), and a threshold
  at its own trigger's fill (`own_fill`).
- **Antimatter only on true ultimates** (108): Orbital Mass Driver, Recursive Self-Improvement and Dyson Swarm
  Tranche pay its 7 in credits; the liberate and removal ultimates keep it.
- **Card throughput** (108): credits pile up (the strategist ends r42's games on 13-21), so the fix is dearer, bigger
  cards and more of them: engines draw when built, Action buildings turn spare credits (Patent Library, Bug Bounty
  Board), chips (Parts Exchange), energy (Grid Analytics Desk) or Agents (Safehouse Network) into cards, three engines
  draw on their tag's plays (Preprint Server, Supplier Network, Fusion Research Campus), and two board powers do the
  same (the Builder's Procure, the Protector's Debrief). Late cards worth 28-40 credits (Moonshot Grant, Headquarters
  Raid, Wartime Retooling, Gigafactory Charter) take spare credits (111). A flat credit sink stays the last resort.
- **Insight off every card** counts `rates.insight_discount_share` (0.25) of the plays (107): the floor below.
- **Insight discounts mostly sit at the floor** (106): `discount_floor` 1 holds insight too, and `science_every` already
  takes a seat with 2 Science tags to it, so a 1-insight discount bites only on 2-insight cards; no deck card prints
  one since r44 (Hyperscale Botnet took 4 insight a round instead). Science's discount prints credits (Grad Student Pool).
- **One shape for one job across the tags:** a per-tag income program ("raise your credit income by 1 per X tag"),
  needs 2 of X (r43), 6 credits, one per tag (r44); a building's per-tag income pays 3 a tag (Foundation Model Campus, r43); a standing counter
  (`auto_counter`, Cyber's alone since r44) 7 credits, needs 2; a typed jam 5 and a rider of its tag; a free typed-jam
  Action building 5 credits and no play insight (r44), about 3 a round.
- **Income has a horizon** (r35): an income, of credits, energy or anything, counts for the rounds left at a typical
  purchase less a tempo discount: 4 rounds on a tier-1 card, 2.5 on a tier-2 one (`rates.horizon_t1`, `horizon_t2`).
- **Repeatable and progressive cards** are priced on their uses: an Action (value less its cost) × the horizon × 0.7;
  a progressive token card (uses × payoff) × 0.7, counting adds as uses (ruling 12).
- **Energy is rare** (r35): about 4.5 credits an energy, 5 to buy from a neutral city (r36). +1 energy of income or
  production is the horizon × 4.5 × 0.8 (`rates.energy_income_share`: it arrives a drop at a time, not when needed):
  about 14 on a tier-1 card, 9 on a tier-2 one (r36, Matt's ruling; it replaced r35's 8-10 anchor), and no card above
  +2 a card (one tag payoff, Tidal Barrage, reaches +3 at 6 Power tags, r44). Three threats' counters, a few cards
  and Grid Analytics Desk's draw take energy.
- **A station is a flip** (r35): freeing one is worth a cube a round both ways, about 15 (`cleanse_station`); freeing a
  factory stays about 6.
- **A gift to another seat** is 0.75 × face; **a tag left in play** 1, then 0.5 each (events leave none).
- **The liberate capstones**: priced at their own value, at least half in their engine's resource, out of reach
  before mid-game: a city liberated is 45 (5 Sway, the city and the ambassadors and Discord it stops; play reads about
  50), against the fallback's 5 ambassadors at about 100; each engine's twist at its own row (3 ambassadors; up to 2
  troops in the city's country; a botnet next to it; a draw 2). At r44 they read 1.2-1.5 at the anchors, under the
  tier-2 band on purpose: the deck's late credit sinks, pending Matt's liberation anchor.

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
