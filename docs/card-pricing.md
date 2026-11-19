# Card pricing

How a card's price is set, as the set stands. Every price is a **candidate**. The cards themselves are
[cards.md](cards.md), generated from `content/cards.json`; the rules for writing one are
[card-guidelines.md](card-guidelines.md). What each pass changed and why is in git and in the pass's write-up under
`experiments/results/`. A card's note for the card viewer is one
line in `docs/card-pricing-notes.json` (keys sorted; CLAUDE.md, *Working rules*); a note dated before r128 prints
r127's credits (÷ 1.5 for today's, 273).

## The rates

**The rates' one home is `configs/agents/values.json`**: a value in credits for each resource, tag, token and op,
each op's row saying what one unit is. The agents value cards by the same table (`docs/agents.md`, *The value table*), so
a rate moved there moves the agents' read of every card that prints it. Change a rate there, not here.

Two other places hold rates, and neither is a price:

- **The search's weights** (`Weights` in `crates/aic-sim/src/agents/eval.rs`) are points tuned against win rate, not
  credits. Where one shadows a table row (held energy `energy` 0.8, `antimatter` 2, `liberated` and
  `liberate_progress`, `station_flip`), the gap is deliberate until a screen moves it: held energy sits below the
  table's price on purpose. A table edit does not move them; say in the write-up when a price and its weight part
  further.
- **The effect ledger** (`scripts/cards/effect_ledger.py`) mirrors `agents/value.rs` by hand. The fast-lane test
  `effect_ledger_mirrors_the_table` checks its committed `cards.csv` against value.rs card by card (bar troop
  removal: below): after a change to value.rs, `values.json` or a card, rerun the ledger, and fix its `op_worth` if
  the test still fails.

**Where a credit number lives** (273's rescale touched each): content ops with `resource: credits`, a card's `cost`,
`play_cost`'s `at`, `building_discount`, `card_discount`, `cash_out`'s `rate`, threat counters and payments (in
`drafts/adversary/`), the AI boards' `credits`, track incomes and power costs, the map's megaproject payouts and
converters, baseline's `*_credits`, `ai.start_credits` and `cards.industry_every`; values.json; in the agents,
`credit_equivalent`'s rates (`agents/mod.rs`) and eval.rs's credit weights; design_check's anchors and job rows.
`experiments/results/273_smaller_numbers/scale.py` walks the content ones.

## A fair price

`python scripts/cards/effect_ledger.py --stdout [--cards PROPOSED.json]` prints each card's `fit` (the printed value the
joint fit gives it: Σ multiplier × reasoned value per fitted effect, + an effect on one or two cards and the tags at
their reasoned value, + its tier's constant, + the rate per needed tag × its
`needs`) and `fair_credits`: the fit less what the card costs in other resources (insight, energy, chips, antimatter,
cubes, `lowers`) at the table's rates. That is the credit price that would put the card on the fitted curve. It writes
nothing, so a staging session can price proposed cards: copy `content/cards.json` to `out/`, edit, pass it with
`--cards`. The fit's constants move with the set; read them from the printout, never from an old write-up. So `fair_credits`
moves when any other card changes: to reprice an edited card, move its price by its change in the reasoned value (`--reasoned-delta BASE`).

## The method (Matt's rulings)

- **Matt's unit anchors set the printed price** (2026-11-04): the anchors, the anchor check and its ceilings (a fair
  card 1.0-1.5, never cut toward them) are [card-guidelines.md](card-guidelines.md), rulings 19-25. A repriced card
  aims at the middle, about 1.2-1.3, at its played tag count (Matt, *Economy* rev 23).
  **The anchors set the price; the table is fixed to see a card's real worth, never the price cut to make it show a
  gain. A card the strategist leaves unplayed gets more power, not a lower price.** Every removal number fits the board (3-6 ambassadors, 1-2 troops, 3-6 compute). Where the table
  disagrees, the anchor wins for the price and the table stays the agents': since 283 `values.json` reads the
  resources, a drawn card and a tier-2 horizon at the anchors (the keep moved with insight: 2.67 and 6.93, its
  expected 2.6 insight at tier 2, not the anchor's 8), but an ambassador removed about 1.47 and a
  troop 7.33 (3.33 each) and a project discard 5.33 (about 16 at 1.33 a compute): at the anchors those lost play. Every credit number is r127's ÷ 1.5 since r128 (273). The recipe and fit below are the agents' view and the effect ledger's.
- **Tier is power** (ruling 4): costs and effects go up a lot with tier, few cards cost 0, and tier is not a proxy for
  cost.
- **The recipe:** value the effect at the rates; then credits = value − the keep (`rates.keep_tier1`, `keep_tier2`) −
  `resources.insight` × the insight to play − about 0.67, so the card beats the plain route it replaces (ruling 7). **Insight to play is
  a price** (Matt): spread over 0, 1 and 2, most tier-1 cards at 0; moving a card 1 insight moves its credits the
  anchor's 2.67, less what would leave it under a 1-credit gain at the table. A card's **table net** (the
  agents' reading, CLAUDE.md's give / get) is `effect_ledger.py --stdout`'s reasoned − printed − `rates.keep_tierN`
  (283's `experiments/results/283_agents_antimatter/table_nets.py` prints every card's).
- **A `needs` chain is the price** (ruling 10): needs 2-3 of a tag on an engine, needs 3-4 sits 2-4 under the recipe,
  needs 5-6 about half and much bigger (Zero-Day Auction, needs 5 Cyber: 2 antimatter and draw 3).
- **The cost curve** (Terraforming Mars scale; Matt: fewer cheap cards, more dear and high-impact ones): most cards in
  the middle, the big income and late cards dearest, a few cheap ones (each card's cost: `docs/cards.md`). Six
  credits is about two thirds of a round's printed credits on the boards (10 / 9 / 7).
- **Big early and late cards** (Matt): early jump-starts, no `needs`, playable in rounds 1-3 from the starting 27-31
  (Fabrication Megaplex, Frontier Lab Charter, National Grid Buildout, beside Gigafactory Charter, Wartime Retooling
  and Multiyear Defense Budget). The anchors price income at mid-game; bought in round 1-3 it runs about 5 rounds
  (about 1.25× the anchor), so a jump-start reads 1.0-1.3 at the anchors and stays under 1.5 early. Late objectives
  reach a goal in one play: Moonshot Grant's 8 cubes fill a megaproject (cubes at 6 when they complete it), the
  liberate capstones. **The majors:** tier-2 cards paying 1 antimatter (18 since 283) for a big payoff, priced at the anchors
  at 1.0-2.5 (each card's line in `docs/card-pricing-notes.json`).
- **Sway past a tip carries over** ([rules/alignment.md](rules/alignment.md)), so a big Sway can tip twice (8 at two players, where the whole pool is 8; Peace Conference). Sway is priced at the
  anchor per point, up to what the pool can hold.
- **Engines are priced at their seat's fire rate** (Matt: err toward strong engines that look broken when played
  well): a trigger at the rate a seat built around its tag sees (about 1 card of the tag a round, 3 tags of any kind),
  not the table's generic `fires_play` × `tag_share`, so the effect ledger reads most engine cards under their
  printed price. That gap is the point; a card that nets below 0 for its own seat is still a bug. **Payback, as
  measured** (the aim is 2 rounds, a bar no item enforces until a repricing pass; Matt, D126): give (credits,
  insight to play and keep, other costs) less the when-built gain and the card's own first fire, over a round's
  yield at that rate (`scripts/cards/payback.py`). At r137, 15 of 38 engine buildings pay back within 2 rounds and 28
  within 3 (median 2.3); eight take 5 or more (Power to the People 17.5, Consulate Tower 10.2, Honeypot Relay 6.5,
  Reconciliation Office 6.3, Supplier Network 6, Security Ops Floor 5.2, Substation Hub 5.1, Preprint Server 5), and
  two (Agency Field Station, Joint Task Force) yield nothing the script reads.
  **Engines pay up front:** the when-built gain is about two fires, in cards (since 271's scarce draw, at most
  2: Foundation Model Campus) where the fire pays credits, since a credit gain on a credit cost would be a circular
  payment (guideline 5). How the agents read a trigger in play: agents.md, *The value table*.
- **Card throughput** (271's scarce draw): credits pile up, so the fix is dearer, bigger cards and more of them, but
  repeatable draw is thin: two engines draw 1 on a tag's plays (Preprint Server on each Science card you play,
  Consulate Tower on each Diplomacy card another player plays), one Action draws 1 a round (Command-and-Control
  Grid), board powers draw 1 a use (the Researcher's Field Survey and Lab, the Protector's Interdict II), and two
  chip converters draw 1 (Polar Compute Array's, Delta Seawall's). The rest is once: 1-2 when a building is built,
  1-3 when a program or event is played. Late, dear cards (Moonshot Grant, Headquarters Raid) take spare credits. A flat credit sink stays the
  last resort.
- **Insight off every card** counts `rates.insight_discount_share` (0.25) of the plays: the floor below.
- **Insight discounts reach 0:** `discount_floor` 0 holds insight too, so `science_every` (a seat with 2 Science
  tags) makes a 1-insight card free to play; no deck card prints an insight discount. Science's discount prints
  credits (Grad Student Pool).
- **One shape for one job across the tags:** a per-tag income program ("raise your credit income by 1 per 3 X tags"),
  needs 2 of X, 4 credits and no play insight, one per tag (priced at 6 tags at play, 1.30 at the anchors); a
  building's per-tag income pays 2 a tag (Foundation Model Campus); a standing counter (`auto_counter`, Cyber's
  and Industry's) 9 credits and no play insight, needs 2; a typed jam 5 and a rider of its tag; a free typed-jam Action building 5
  credits and no play insight, about 2 a round.
- **Income has a horizon:** any income counts for the rounds left at a typical
  purchase less a tempo discount: 4 rounds on a tier-1 card, 3.5 on a tier-2 one (`rates.horizon_t1`, `horizon_t2`;
  283: the anchor check's 3-4 rounds).
- **Repeatable and progressive cards** are priced on their uses: an Action (value less its cost) × the horizon × 0.7;
  a progressive token card (uses × payoff) × 0.7, counting adds as uses (ruling 12). A conversion Action passes both
  the anchor ceiling and a gain at `values.json` only when its output is priced alike in both (Sway, a liberation, a
  megaproject cube: `project_cube` and the `research_cubes` resource both 4). At tier 1, energy into insight reads
  over 1.5 at the anchors or a loss at the table (a gain needs over 1.8 insight an energy, the ceiling under 1.69);
  into cards only 1.13-1.35 cards an energy passes both (3 energy for 4 cards). Tier 2's 2.5 ceiling passes either
  at 1:2.
- **Energy is rare:** about 3 credits an energy, 4 to buy from a neutral city. +1 energy of income or production is
  the horizon × 3 × 0.8 (`rates.energy_income_share`: it arrives a drop at a time, not when needed): about 9.6 on a
  tier-1 card, 8.4 on a tier-2 one (Matt's ruling), and no card above +2 a card but the major Dyson Swarm Tranche (+3) and one tag payoff (Tidal
  Barrage, +3 at 6 Power tags). Threat counters that offer energy (by tag), fourteen cards and ten Actions take
  energy ([cards.md](cards.md)).
- **Energy as a second cost** (Matt): a card whose title reads power-hungry prints energy and costs 3 credits less
  an energy, so pay holds at the anchors; uncommon (14 of 191 deck cards). The late sinks price energy at its 3
  anchor too: Citywide Microgrid's 4 energy is 12 of its 30, Exascale Training Cluster's 2 is 6 of its 23 (258 moved
  the rest into credits: energy binds), and
  each 2-energy Action is a use's 6. The table prices energy at the full anchor while its insight, draw and chip
  rates sit at about 0.6 of theirs, so an energy-for-X Action reads as a loss at `values.json` for a seat short of
  energy and a gain only through the spare-energy rate; two of the three sinks whose output the table prices lowest
  (an ambassador removed; a draw and chips) carry an override.
- **A station is a flip:** freeing one is worth a cube a round both ways, about 10 (`cleanse_station`); freeing a
  factory stays about 4.
- **A gift to another seat** is 0.75 × face; **a tag left in play** 0.67, then 0.33 each (events leave none).
- **The liberate capstones**: priced at their own value, at least half in their engine's resource, out of reach
  before mid-game: a city liberated is 30 (5 Sway, the city and the ambassadors and Discord it stops; play reads about
  33), against the Liberate action's 5 ambassadors at about 27; each engine's twist at its own row (3 ambassadors;
  up to 3 troops in the city's country; a botnet next to it). They read 1.0-1.5 at the anchors, inside the
  margin like any fair card: the deck's late credit sinks, pending Matt's liberation anchor.

## Rates known to be off at r36

Measured or found against play; each is still the table's number until a pass moves it.

- **Antimatter 18** (the anchor, Matt 283; was 12): the heuristic holds antimatter at 2 points, about 2 credits (283:
  at 3 or 4 Antiproton Foundry ran 3x but Orbital Relay Launcher's burns fell 4x, so less was spent), so a seat keeps it for
  a card it holds that needs it (`am_save`) and makes it, from a power or a built Action first, when that plays such
  a card this round (`am_source`).
- **Troop removal 7.33** (4.67 each in one country): the heuristic counts a troop at 12 points, about 13 credits. A count
  prints "up to": the ledger and the agents count what one removes in play (`values.json` `troops_removed`; Decapitation
  Strike's 3 about 1.75).
- **The recipe nets every card about its keep + 0.67** over its costs at the table, so the table alone can't rank cards for
  a draft: tier 2 reads as worth no more than its keep unless the draft leaves the play cost out, as the agents' does.
- **Energy 3:** what seats hold: `energy_read.py` (the last read: `screen-090/balance.md`).
- **Insight 2.67:** the anchor since 283 (was 1.67, a fit that centered tier 1 on its printed prices).
- **A player cube at completion, about 6:** measured cheaper in play, as energy and chips buy less than their rates.
